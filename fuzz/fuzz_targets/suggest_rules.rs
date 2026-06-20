//! Fuzz target for `focus-rule-suggester`.
//!
//! Interprets raw bytes as structured test inputs to exercise edge cases in
//! the rule suggestion engine. Covers 8 boundary categories:
//!
//!   1. Boundary time values (epoch, far future, extremely far past)
//!   2. Empty / missing event lists
//!   3. Large numbers of events (O(n²) exposure)
//!   4. Malformed / missing / wrong-type `payload` fields
//!   5. Repeated calls on the same `RuleSuggester` (gauntlet test for `&self`)
//!   6. All events at the same timestamp (hour-bucket edge case in H1)
//!   7. Extreme hour overrides (24, 255, NaN time offsets)
//!   8. Unicode / exotic / empty / null-byte strings in connector_id, event_type,
//!      dedupe_key, payload keys/values, and raw_ref
//!
//! # Byte protocol
//!
//! The fuzzer provides a `&[u8]` slice. We read greedily, fall back to safe
//! defaults on underrun, and **never panic** in the harness itself.
//!
//! ```text
//! OFFSET  SIZE  FIELD
//! ------  ----  -----
//!  0       1    window_days          (u8 -> clamped to 0..730)
//!  1       1    event_count          (u8 -> clamped to 0..200)
//!  2       1    dismissal_count      (u8 -> clamped to 0..32)
//!  3       1    call_repetitions     (u8 -> clamped to 1..16)
//!
//! Following are `event_count` event records, then `dismissal_count` UUIDs.
//!
//! ## Event record (variable; minimum 20 bytes)
//!
//! ```text
//!  [0]   event_type     (u8) --- 0=EventStarted, 1=TaskCompleted,
//!                                 2=EventEnded, 3=Custom(s)
//!  [1]   connector_sel  (u8) --- 0="local", 1="github", 2="canvas",
//!                                 3="", 4=zero-width chars,
//!                                 5=2-byte-len+utf8-lossy,
//!                                 6=BOM+bytes, 7=null-byte-string
//!  [2..10]  ts_seed    (i64 LE) -- wrapped to +/-20yr around epoch;
//!                                  sentinels: MIN=epoch, MAX=year 9999,
//!                                  0=Utc::now()
//!  [10..18] eff_seed   (i64 LE) -- offset from occurred_at (mod 86400s)
//!  [18]  payload_style (u8) --- 0=Null, 1={"source":"focus_session"},
//!                                 2={"action":"closed"},
//!                                 3={"type":"celebration"},
//!                                 4=deeply-nested, 5=string leaf,
//!                                 6=wrong-type value, 7=unicode keys,
//!                                 8=very-long-value, 9=array,
//!                                 10=number, 11=bool, 12=controlled-depth
//!  [19]  extra_flags   (u8) --- bit 0: override hour -> 24
//!                                 bit 1: override -> +255 hours
//!                                 bit 2: all-events-same-timestamp
//!                                 bit 3: confidence = NaN
//!                                 bit 4: raw_ref = Some(weird)
//! ```
//!
//! ## Dismissal record (16 bytes)
//!
//! Raw 16 bytes -> `Uuid::from_u128()`.
//!
//! # Panic boundary
//!
//! The harness itself must NEVER panic. Assertions verify engine invariants:
//!
//! - `suggest_rules` never returns an error
//! - Every `RuleSuggestion` has non-empty `rationale`, `name`, `trigger`
//! - No duplicate suggestion IDs within a single call

#![no_main]

use libfuzzer_sys::fuzz_target;

use chrono::{DateTime, Duration, Utc};
use focus_audit::AuditRecord;
use focus_events::{DedupeKey, EventType, NormalizedEvent, TraceRef, WellKnownEventType};
use focus_rule_suggester::RuleSuggester;
use serde_json::Value;
use uuid::Uuid;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

const MAX_EVENTS: usize = 200;
const MAX_DISMISSALS: usize = 32;
const MAX_REPETITIONS: usize = 16;
const MAX_WINDOW_DAYS: u32 = 730;
const EVENT_FIXED_BYTES: usize = 20; // 1+1+8+8+1+1
const UUID_BYTES: usize = 16;

const WEIRD_CONNECTOR_STRS: &[&str] = &[
    "local",
    "github",
    "canvas",
    "",
    "\0",
    "GITHUB",
    "GiThUb",
    "github.com/KooshaPari/repo",
    "   ",
    "\t\n\r",
    "null",
    "undefined",
    "<script>alert('xss')</script>",
    "../../../etc/passwd",
    "connector_id_with_underscores_and-dashes",
];

const WEIRD_EVENT_TYPE_STRS: &[&str] = &[
    "custom",
    "CUSTOM",
    "",
    "event:TaskCompleted",
    "github_pr_closed",
    "\0\0\0",
    " sehr lang  deutsche  EventType ",
    "\u{1f4a5}\u{1f525}\u{1f389}",
    "\u{1f980}",
    "\u{1d4ef}\u{1d4aa}\u{1d4f7}\u{1d4f2}\u{1d4fd} \u{1d4f9}\u{1d4fe}\u{1d4f5}\u{1d4f2}",
];

// ---------------------------------------------------------------------------
// Safe byte readers (never panic on underrun)
// ---------------------------------------------------------------------------

fn read_u8(data: &[u8], offset: usize) -> (u8, usize) {
    if offset < data.len() {
        (data[offset], offset + 1)
    } else {
        (0, offset.saturating_add(1))
    }
}

fn read_i64_le(data: &[u8], offset: usize) -> (i64, usize) {
    let mut buf = [0u8; 8];
    let end = offset.saturating_add(8);
    let n = end.min(data.len());
    let copy_len = n - offset;
    buf[..copy_len].copy_from_slice(&data[offset..n]);
    (i64::from_le_bytes(buf), end)
}

fn read_u128_le(data: &[u8], offset: usize) -> (u128, usize) {
    let mut buf = [0u8; 16];
    let end = offset.saturating_add(16);
    let n = end.min(data.len());
    let copy_len = n - offset;
    buf[..copy_len].copy_from_slice(&data[offset..n]);
    (u128::from_le_bytes(buf), end)
}

fn read_byte_slice<'a>(data: &'a [u8], offset: usize, len: usize) -> (&'a [u8], usize) {
    let end = offset.saturating_add(len);
    if end <= data.len() {
        (&data[offset..end], end)
    } else {
        let avail = data.len().saturating_sub(offset);
        let slice = if avail > 0 { &data[offset..] } else { &[] };
        (slice, offset.saturating_add(len))
    }
}

// ---------------------------------------------------------------------------
// Fuzz-data -> timestamp
// ---------------------------------------------------------------------------

/// Build a timestamp from an i64 seed, intentionally producing edge cases.
fn fuzz_timestamp(seed: i64) -> DateTime<Utc> {
    if seed == i64::MIN {
        // Epoch (unix zero)
        DateTime::from_timestamp(0, 0).unwrap_or(Utc::now())
    } else if seed == i64::MAX {
        // Near-end of chrono range (~ year 9999)
        DateTime::from_timestamp(253_402_300_799, 0).unwrap_or(Utc::now())
    } else if seed == 0 {
        Utc::now()
    } else {
        let sane_delta = seed.wrapping_rem(365_i64 * 86400 * 20); // +/-20yr
        DateTime::from_timestamp(sane_delta, 0).unwrap_or(Utc::now())
    }
}

// ---------------------------------------------------------------------------
// Payload construction
// ---------------------------------------------------------------------------

fn build_payload(style: u8) -> Value {
    match style {
        0 => Value::Null,
        1 => serde_json::json!({"source": "focus_session"}),
        2 => serde_json::json!({"action": "closed"}),
        3 => serde_json::json!({"type": "celebration"}),
        4 => serde_json::json!({
            "source": "focus_session",
            "action": "closed",
            "type": "celebration",
            "nested": {"level1": {"level2": ["a", "b"]}},
            "metadata": {"count": 42, "tags": ["fuzz", "edge_case"]},
        }),
        5 => Value::String("payload is a string not an object".into()),
        6 => serde_json::json!({"source": {"nested": true}}),
        7 => serde_json::json!({
            "\u{1f525}": "fire_emoji_key",
            "": "empty_key",
            "a\u{0000}b": "null_in_key",
        }),
        8 => {
            let mut map = serde_json::Map::new();
            map.insert("source".into(), Value::String("a".repeat(4096)));
            Value::Object(map)
        }
        9 => serde_json::json!(["item1", "item2", null, true, 42]),
        10 => Value::Number(serde_json::Number::from(9_999_999_999_999i64)),
        11 => Value::Bool(true),
        12 => deep_json(5),
        _ => serde_json::json!({}),
    }
}

fn deep_json(depth: u32) -> Value {
    if depth == 0 {
        return Value::String("leaf".into());
    }
    serde_json::json!({
        "level": depth,
        "child": deep_json(depth.saturating_sub(1)),
        "array": [1, 2, 3],
    })
}

// ---------------------------------------------------------------------------
// String synthesis from fuzz bytes
// ---------------------------------------------------------------------------

fn pick_connector_string(data: &[u8], offset: &mut usize, sel: u8) -> String {
    match sel {
        0 => "local".into(),
        1 => "github".into(),
        2 => "canvas".into(),
        3 => String::new(),
        4 => "\u{feff}\u{200b}\u{200c}\u{200d}".into(),
        5 => {
            // 2-byte LE length prefix, then raw utf8-lossy string
            let (lo, o) = read_u8(data, *offset);
            *offset = o;
            let (hi, o) = read_u8(data, *offset);
            *offset = o;
            let len = (lo as usize) | ((hi as usize) << 8);
            let clamped = len.min(256);
            let (slice, o) = read_byte_slice(data, *offset, clamped);
            *offset = o;
            if slice.is_empty() {
                String::new()
            } else {
                String::from_utf8_lossy(slice).into_owned()
            }
        }
        6 => {
            // BOM prefix + random bytes
            let mut s = "\u{feff}".to_string();
            let (lo, o) = read_u8(data, *offset);
            *offset = o;
            let (hi, o) = read_u8(data, *offset);
            *offset = o;
            let extra = ((lo as usize) | ((hi as usize) << 8)).min(64);
            let (slice, o) = read_byte_slice(data, *offset, extra);
            *offset = o;
            s.push_str(&String::from_utf8_lossy(slice));
            s
        }
        7 => "\0evil\u{0000}bytes".into(),
        _ => "local".into(),
    }
}

fn pick_event_type_string(data: &[u8], offset: &mut usize, sel: u8) -> String {
    let idx = (sel as usize).min(WEIRD_EVENT_TYPE_STRS.len().wrapping_sub(1));
    WEIRD_EVENT_TYPE_STRS[idx].to_string()
}

// ---------------------------------------------------------------------------
// Events builder
// ---------------------------------------------------------------------------

fn build_events(data: &[u8], count: usize, mut offset: usize) -> (Vec<NormalizedEvent>, usize) {
    let n = count.min(MAX_EVENTS);
    let mut events = Vec::with_capacity(n);
    let now = Utc::now();
    let mut frozen_ts: Option<DateTime<Utc>> = None;

    for _ in 0..n {
        if offset >= data.len() {
            // Pad with safe default events when data runs out
            events.push(empty_event(now));
            continue;
        }

        // event_type (1B)
        let (et, o) = read_u8(data, offset);
        offset = o;

        let event_type = match et & 0x03 {
            0 => EventType::WellKnown(WellKnownEventType::EventStarted),
            1 => EventType::WellKnown(WellKnownEventType::TaskCompleted),
            2 => EventType::WellKnown(WellKnownEventType::EventEnded),
            3 => {
                let (type_sel, o) = read_u8(data, offset);
                offset = o;
                EventType::Custom(pick_event_type_string(data, &mut offset, type_sel))
            }
            _ => EventType::WellKnown(WellKnownEventType::TaskCompleted),
        };

        // connector selector (1B) + optional variable string
        let (conn_sel, o) = read_u8(data, offset);
        offset = o;
        let connector_id = pick_connector_string(data, &mut offset, conn_sel);

        // timestamp seed (8B i64 LE)
        let (ts_seed, o) = read_i64_le(data, offset);
        offset = o;
        let occurred_at = fuzz_timestamp(ts_seed);

        // effective_at seed (8B i64 LE) -- relative offset from occurred_at
        let (eff_seed, o) = read_i64_le(data, offset);
        offset = o;
        let effective_at = occurred_at + Duration::seconds(eff_seed.wrapping_rem(86400));

        // payload (1B)
        let (payload_byte, o) = read_u8(data, offset);
        offset = o;
        let payload = build_payload(payload_byte);

        // extra flags (1B)
        let (flags, o) = read_u8(data, offset);
        offset = o;

        // Hour overrides
        let occurred_at = if flags & 0x01 != 0 {
            occurred_at.with_hour(24).unwrap_or(occurred_at)
        } else if flags & 0x02 != 0 {
            occurred_at + Duration::hours(255)
        } else {
            occurred_at
        };

        // "all same timestamp" override
        let occurred_at = if flags & 0x04 != 0 {
            *frozen_ts.get_or_insert(occurred_at)
        } else {
            occurred_at
        };

        let confidence = if flags & 0x08 != 0 { f32::NAN } else { 1.0 };

        let raw_ref = if flags & 0x10 != 0 {
            Some(TraceRef {
                source: "\0\u{1f4a5}<script>alert(1)</script>".into(),
                id: "ffff-ffff-ffff".into(),
            })
        } else {
            None
        };

        // Build dedupe key from connector to cover edge-case strings
        let dedupe_str = if connector_id.starts_with("github") || connector_id.starts_with("pr") {
            format!("pr-{}", Uuid::new_v4())
        } else if connector_id.starts_with("local") || connector_id.starts_with("focus") {
            format!("focus-{}", Uuid::new_v4())
        } else if connector_id.is_empty() {
            String::new()
        } else {
            connector_id.clone()
        };

        events.push(NormalizedEvent {
            event_id: Uuid::new_v4(),
            connector_id,
            account_id: Uuid::new_v4(),
            event_type,
            occurred_at,
            effective_at,
            dedupe_key: DedupeKey(dedupe_str),
            confidence,
            payload,
            raw_ref,
        });
    }

    (events, offset)
}

fn empty_event(now: DateTime<Utc>) -> NormalizedEvent {
    NormalizedEvent {
        event_id: Uuid::new_v4(),
        connector_id: String::new(),
        account_id: Uuid::new_v4(),
        event_type: EventType::WellKnown(WellKnownEventType::TaskCompleted),
        occurred_at: now,
        effective_at: now,
        dedupe_key: DedupeKey(String::new()),
        confidence: 1.0,
        payload: Value::Null,
        raw_ref: None,
    }
}

// ---------------------------------------------------------------------------
// Dismissals builder
// ---------------------------------------------------------------------------

fn build_dismissals(data: &[u8], count: usize, mut offset: usize) -> (Vec<Uuid>, usize) {
    let n = count.min(MAX_DISMISSALS);
    let mut ids = Vec::with_capacity(n);
    for _ in 0..n {
        let (raw, o) = read_u128_le(data, offset);
        offset = o;
        ids.push(Uuid::from_u128(raw));
    }
    (ids, offset)
}

// ---------------------------------------------------------------------------
// Mock AuditStore (required by suggest_rules signature)
// ---------------------------------------------------------------------------

struct FuzzAuditStore;

impl focus_audit::AuditStore for FuzzAuditStore {
    fn append(&self, _record: AuditRecord) -> anyhow::Result<()> {
        Ok(())
    }
    fn verify_chain(&self) -> anyhow::Result<bool> {
        Ok(true)
    }
    fn head_hash(&self) -> anyhow::Result<Option<String>> {
        Ok(None)
    }
}

// ---------------------------------------------------------------------------
// Fuzz entry point
// ---------------------------------------------------------------------------

fuzz_target!(|data: &[u8]| {
    if data.is_empty() {
        return;
    }

    let mut offset: usize = 0;

    // ---- header (4 bytes) ----
    let (window_raw, o) = read_u8(data, offset);
    offset = o;
    let window_days = (window_raw as u32).min(MAX_WINDOW_DAYS);

    let (ev_count, o) = read_u8(data, offset);
    offset = o;
    let event_count = ev_count as usize;

    let (dis_count, o) = read_u8(data, offset);
    offset = o;
    let dismissal_count = dis_count as usize;

    let (rep_raw, o) = read_u8(data, offset);
    offset = o;
    let repetitions = (rep_raw as usize).min(MAX_REPETITIONS).max(1);

    // ---- events ----
    let (events, o) = build_events(data, event_count, offset);
    offset = o;

    // ---- dismissals ----
    let (dismissed, _o) = build_dismissals(data, dismissal_count, offset);

    // ---- run the engine ----
    let suggester = RuleSuggester::new().with_dismissed(dismissed);
    let store = FuzzAuditStore;

    for _ in 0..repetitions {
        let result = suggester.suggest_rules(&store, &events, window_days);
        assert!(result.is_ok(), "suggest_rules returned error");

        if let Ok(suggestions) = result {
            // invariants: every suggestion has non-empty rationale/name/trigger
            for s in &suggestions {
                assert!(
                    !s.rationale.is_empty(),
                    "empty rationale from heuristic {}",
                    s.heuristic_name
                );
                assert!(
                    !s.proposed_rule.name.is_empty(),
                    "empty rule name from heuristic {}",
                    s.heuristic_name
                );
                assert!(
                    !s.proposed_rule.trigger.is_empty(),
                    "empty trigger from heuristic {}",
                    s.heuristic_name
                );
            }

            // no duplicate IDs within a single response
            let mut seen: Vec<Uuid> = suggestions.iter().map(|s| s.id).collect();
            seen.sort();
            seen.dedup();
            assert_eq!(
                seen.len(),
                suggestions.len(),
                "duplicate suggestion IDs within one suggest_rules call"
            );
        }
    }
});

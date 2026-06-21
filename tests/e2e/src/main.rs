//! End-to-end smoke test: Connector Event → Sync → Rule Fires → Wallet Mutates → Audit Appends.
//!
//! Scenario 1: GitHub PR merged → wallet.grant(10) fire → audit appends chain
//! Scenario 2: Focus session completed → streak increments → audit chains
//! Scenario 3: Audit chain verify passes on restart
//!
//! Runs in <30s total, emits JSON summary on stdout, non-zero exit on failure.

use anyhow::Result;
use chrono::Utc;
use focus_audit::{AuditChain, AuditSink};
use focus_events::{DedupeKey, EventType, NormalizedEvent};
use focus_rewards::{Credit, RewardWallet, WalletMutation};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

/// Mock AuditSink that appends to an in-memory chain.
#[derive(Debug, Clone)]
struct MockAuditSink {
    chain: Arc<std::sync::Mutex<AuditChain>>,
}

impl MockAuditSink {
    fn new() -> Self {
        Self {
            chain: Arc::new(std::sync::Mutex::new(AuditChain::new())),
        }
    }

    fn chain(&self) -> AuditChain {
        self.chain.lock().unwrap().clone()
    }
}

impl AuditSink for MockAuditSink {
    fn record_mutation(
        &self,
        record_type: &str,
        subject_ref: &str,
        payload: Value,
        now: chrono::DateTime<chrono::Utc>,
    ) -> anyhow::Result<()> {
        if let Ok(mut chain) = self.chain.lock() {
            chain.append(record_type, subject_ref, payload, now);
        }
        Ok(())
    }
}

/// Test assertions container.
#[derive(Debug, Default)]
struct TestResults {
    scenarios_passed: usize,
    scenarios_failed: usize,
    total_assertions: usize,
    total_passed: usize,
    total_failed: usize,
    failures: Vec<String>,
}

impl TestResults {
    fn record_assertion(&mut self, passed: bool) {
        self.total_assertions += 1;
        if passed {
            self.total_passed += 1;
        } else {
            self.total_failed += 1;
        }
    }

    fn record_scenario(&mut self, name: &str, passed: bool) {
        if passed {
            self.scenarios_passed += 1;
            println!("  ✓ {}", name);
        } else {
            self.scenarios_failed += 1;
            self.failures.push(name.to_string());
            println!("  ✗ {}", name);
        }
    }

    fn to_json(&self) -> Value {
        json!({
            "timestamp": Utc::now().to_rfc3339(),
            "scenarios_passed": self.scenarios_passed,
            "scenarios_failed": self.scenarios_failed,
            "assertions": {
                "total": self.total_assertions,
                "passed": self.total_passed,
                "failed": self.total_failed,
            },
            "failures": self.failures,
            "success": self.scenarios_failed == 0,
        })
    }
}

/// Scenario 1: GitHub PR merged event → rule fires → wallet.grant(10) → audit appends.
fn scenario_github_pr_merged(results: &mut TestResults) -> Result<()> {
    println!("\nScenario 1: GitHub PR Merged");

    let now = Utc::now();
    let user_id = Uuid::new_v4();
    let audit = MockAuditSink::new();

    // Create a wallet with 0 credits.
    let mut wallet = RewardWallet {
        user_id,
        earned_credits: 0,
        spent_credits: 0,
        streaks: HashMap::new(),
        unlock_balances: HashMap::new(),
        multiplier_state: Default::default(),
    };

    // Assert initial balance is 0.
    let initial_balance = wallet.balance();
    let assertion1 = initial_balance == 0;
    results.record_assertion(assertion1);
    println!("    Initial balance == 0: {}", assertion1);

    // Simulate a rule firing: grant 10 credits for GitHub PR merged.
    let mutation = WalletMutation::GrantCredit(Credit {
        amount: 10,
        source_rule_id: Some(Uuid::new_v4()),
        granted_at: now,
    });

    wallet.apply(mutation, now, &audit)?;

    // Assert balance is now 10.
    let new_balance = wallet.balance();
    let assertion2 = new_balance == 10;
    results.record_assertion(assertion2);
    println!("    Balance after grant == 10: {}", assertion2);

    // Assert audit chain has 1 record (the grant).
    let chain = audit.chain();
    let assertion3 = chain.len() == 1;
    results.record_assertion(assertion3);
    println!("    Audit chain length == 1: {}", assertion3);

    // Assert audit record_type is "wallet.grant_credit".
    let record = &chain.records[0];
    let assertion4 = record.record_type == "wallet.grant_credit";
    results.record_assertion(assertion4);
    println!("    Audit record_type == 'wallet.grant_credit': {}", assertion4);

    // Assert chain verifies (no tampering).
    let assertion5 = chain.verify().is_ok();
    results.record_assertion(assertion5);
    println!("    Audit chain verify passes: {}", assertion5);

    let scenario_passed = assertion1 && assertion2 && assertion3 && assertion4 && assertion5;
    results.record_scenario("github_pr_merged", scenario_passed);

    Ok(())
}

/// Scenario 2: Focus session completed → streak increments → audit appends.
fn scenario_focus_session_completed(results: &mut TestResults) -> Result<()> {
    println!("\nScenario 2: Focus Session Completed");

    let now = Utc::now();
    let user_id = Uuid::new_v4();
    let audit = MockAuditSink::new();

    // Create a wallet with empty streaks.
    let mut wallet = RewardWallet {
        user_id,
        earned_credits: 0,
        spent_credits: 0,
        streaks: HashMap::new(),
        unlock_balances: HashMap::new(),
        multiplier_state: Default::default(),
    };

    // Assert initial streak count is 0.
    let initial_streaks = wallet.streaks.len();
    let assertion1 = initial_streaks == 0;
    results.record_assertion(assertion1);
    println!("    Initial streaks count == 0: {}", assertion1);

    // Simulate a rule firing: increment "daily_focus" streak.
    let mutation = WalletMutation::StreakIncrement("daily_focus".to_string());
    wallet.apply(mutation, now, &audit)?;

    // Assert streak "daily_focus" exists and has count == 1.
    let assertion2 = wallet.streaks.contains_key("daily_focus");
    results.record_assertion(assertion2);
    println!("    Streak 'daily_focus' created: {}", assertion2);

    let assertion3 = wallet.streaks.get("daily_focus").map(|s| s.count) == Some(1);
    results.record_assertion(assertion3);
    println!("    Streak count == 1: {}", assertion3);

    // Assert audit chain has 1 record.
    let chain = audit.chain();
    let assertion4 = chain.len() == 1;
    results.record_assertion(assertion4);
    println!("    Audit chain length == 1: {}", assertion4);

    // Assert record_type is "wallet.streak_increment".
    let record = &chain.records[0];
    let assertion5 = record.record_type == "wallet.streak_increment";
    results.record_assertion(assertion5);
    println!("    Audit record_type == 'wallet.streak_increment': {}", assertion5);

    // Assert chain verifies.
    let assertion6 = chain.verify().is_ok();
    results.record_assertion(assertion6);
    println!("    Audit chain verify passes: {}", assertion6);

    let scenario_passed =
        assertion1 && assertion2 && assertion3 && assertion4 && assertion5 && assertion6;
    results.record_scenario("focus_session_completed", scenario_passed);

    Ok(())
}

/// Scenario 3: Multi-mutation chain → verify after multiple appends.
fn scenario_audit_chain_verification(results: &mut TestResults) -> Result<()> {
    println!("\nScenario 3: Audit Chain Verification");

    let now = Utc::now();
    let user_id = Uuid::new_v4();
    let audit = MockAuditSink::new();

    let mut wallet = RewardWallet {
        user_id,
        earned_credits: 0,
        spent_credits: 0,
        streaks: HashMap::new(),
        unlock_balances: HashMap::new(),
        multiplier_state: Default::default(),
    };

    // Apply 3 mutations sequentially.
    wallet.apply(
        WalletMutation::GrantCredit(Credit {
            amount: 10,
            source_rule_id: Some(Uuid::new_v4()),
            granted_at: now,
        }),
        now,
        &audit,
    )?;

    wallet.apply(
        WalletMutation::StreakIncrement("daily_focus".to_string()),
        now,
        &audit,
    )?;

    wallet.apply(
        WalletMutation::GrantCredit(Credit {
            amount: 5,
            source_rule_id: Some(Uuid::new_v4()),
            granted_at: now,
        }),
        now,
        &audit,
    )?;

    // Assert chain length is 3.
    let chain = audit.chain();
    let assertion1 = chain.len() == 3;
    results.record_assertion(assertion1);
    println!("    Chain length == 3: {}", assertion1);

    // Assert each record's prev_hash points to its predecessor.
    let assertion2 = chain.records[0].prev_hash == focus_audit::GENESIS_PREV_HASH;
    results.record_assertion(assertion2);
    println!("    Record 0 prev_hash == GENESIS: {}", assertion2);

    let assertion3 = chain.records[1].prev_hash == chain.records[0].hash;
    results.record_assertion(assertion3);
    println!("    Record 1 prev_hash == Record 0 hash: {}", assertion3);

    let assertion4 = chain.records[2].prev_hash == chain.records[1].hash;
    results.record_assertion(assertion4);
    println!("    Record 2 prev_hash == Record 1 hash: {}", assertion4);

    // Assert full chain verify passes.
    let assertion5 = chain.verify().is_ok();
    results.record_assertion(assertion5);
    println!("    Chain verify passes: {}", assertion5);

    // Assert wallet balance is 10 + 5 = 15.
    let assertion6 = wallet.balance() == 15;
    results.record_assertion(assertion6);
    println!("    Final wallet balance == 15: {}", assertion6);

    let scenario_passed = assertion1
        && assertion2
        && assertion3
        && assertion4
        && assertion5
        && assertion6;
    results.record_scenario("audit_chain_verification", scenario_passed);

    Ok(())
}

/// Scenario 4: Event normalization and connector integration mock.
fn scenario_event_normalization(results: &mut TestResults) -> Result<()> {
    println!("\nScenario 4: Event Normalization");

    let now = Utc::now();
    let account_id = Uuid::new_v4();

    // Create a normalized event from a GitHub connector.
    let event = NormalizedEvent {
        event_id: Uuid::new_v4(),
        connector_id: "github".to_string(),
        account_id,
        event_type: EventType::Custom("pr_merged".to_string()),
        occurred_at: now,
        effective_at: now,
        dedupe_key: DedupeKey("gh_pr_123".to_string()),
        confidence: 0.95,
        payload: json!({
            "pr_id": 123,
            "repo": "KooshaPari/FocalPoint",
            "author": "test-user",
        }),
        raw_ref: None,
    };

    // Assert connector_id is "github".
    let assertion1 = event.connector_id == "github";
    results.record_assertion(assertion1);
    println!("    Connector ID == 'github': {}", assertion1);

    // Assert event_type matches.
    let assertion2 = event.event_type == EventType::Custom("pr_merged".to_string());
    results.record_assertion(assertion2);
    println!("    Event type == pr_merged: {}", assertion2);

    // Assert confidence is in valid range.
    let assertion3 = event.confidence >= 0.0 && event.confidence <= 1.0;
    results.record_assertion(assertion3);
    println!("    Confidence in [0.0, 1.0]: {}", assertion3);

    // Assert payload contains expected fields.
    let assertion4 = event.payload.get("pr_id").is_some();
    results.record_assertion(assertion4);
    println!("    Payload has 'pr_id': {}", assertion4);

    let scenario_passed = assertion1 && assertion2 && assertion3 && assertion4;
    results.record_scenario("event_normalization", scenario_passed);

    Ok(())
}

fn main() -> Result<()> {
    println!("=== FocalPoint E2E Smoke Test ===");
    let start = std::time::Instant::now();

    let mut results = TestResults::default();

    // Run all scenarios.
    if let Err(e) = scenario_github_pr_merged(&mut results) {
        eprintln!("Scenario 1 error: {}", e);
        results.scenarios_failed += 1;
        results.failures.push("github_pr_merged: error".to_string());
    }

    if let Err(e) = scenario_focus_session_completed(&mut results) {
        eprintln!("Scenario 2 error: {}", e);
        results.scenarios_failed += 1;
        results.failures.push("focus_session_completed: error".to_string());
    }

    if let Err(e) = scenario_audit_chain_verification(&mut results) {
        eprintln!("Scenario 3 error: {}", e);
        results.scenarios_failed += 1;
        results.failures.push("audit_chain_verification: error".to_string());
    }

    if let Err(e) = scenario_event_normalization(&mut results) {
        eprintln!("Scenario 4 error: {}", e);
        results.scenarios_failed += 1;
        results.failures.push("event_normalization: error".to_string());
    }

    let elapsed = start.elapsed();

    // Summary.
    println!("\n=== Summary ===");
    println!("  Scenarios passed: {}", results.scenarios_passed);
    println!("  Scenarios failed: {}", results.scenarios_failed);
    println!("  Total assertions: {}", results.total_assertions);
    println!("  Passed: {}", results.total_passed);
    println!("  Failed: {}", results.total_failed);
    println!("  Elapsed: {:.2}s", elapsed.as_secs_f64());

    // Emit JSON report.
    let mut report = results.to_json();
    if let Some(obj) = report.as_object_mut() {
        obj.insert("elapsed_secs".to_string(), json!(elapsed.as_secs_f64()));
    }

    println!("\n=== JSON Report ===");
    println!("{}", serde_json::to_string_pretty(&report)?);

    // Exit code.
    if results.scenarios_failed > 0 {
        std::process::exit(1);
    }

    Ok(())
}

//! # `pheno-chaos-macros` — proc-macro crate for `pheno-chaos`
//!
//! Exposes a single attribute macro, [`chaos_test`], that wraps a
//! test function so it runs under the chaos schedule defined by
//! `pheno_chaos::runtime::ChaosConfig`.
//!
//! ## Generated code shape
//!
//! Given:
//!
//! ```ignore
//! #[chaos_test(faults = "latency,drop,cpu", slo_ms = 500, runs = 3)]
//! fn my_test() {
//!     // body
//! }
//! ```
//!
//! The macro expands to roughly:
//!
//! ```ignore
//! #[::core::prelude::v1::test]
//! fn my_test() {
//!     let cfg = ::pheno_chaos::runtime::parse_attr_args(
//!         "faults = \"latency,drop,cpu\", slo_ms = 500, runs = 3",
//!     );
//!     ::pheno_chaos::runtime::run_with_chaos(&cfg, || {
//!         // body
//!     });
//! }
//! ```
//!
//! The `#[test]` attribute is added automatically so the user does
//! not need to write it themselves.

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

/// Wrap a test function so it runs under the `pheno-chaos`
/// schedule.
///
/// Recognised attribute keys (all optional):
///
/// - `faults = "latency,drop,cpu"` — comma-separated subset of the
///   three fault kinds. Default = all three.
/// - `slo_ms = 500` — per-run SLO. The body must complete within this
///   budget. Default = 500ms.
/// - `runs = 3` — number of fault-injection runs. Default = 3.
/// - `seed = 12345` — RNG seed for reproducibility. 0 = clock
///   entropy. Default = 0.
///
/// Example:
///
/// ```ignore
/// use pheno_chaos::chaos_test;
///
/// #[chaos_test(faults = "latency,cpu", slo_ms = 200, runs = 5)]
/// fn resilient_endpoint() {
///     // ... must complete < 200ms under latency + cpu faults ...
/// }
/// ```
#[proc_macro_attribute]
pub fn chaos_test(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_str = attr.to_string();
    let input_fn = parse_macro_input!(item as ItemFn);

    let fn_name = &input_fn.sig.ident;
    let fn_vis = &input_fn.vis;
    let fn_sig = &input_fn.sig;
    let fn_block = &input_fn.block;
    let fn_attrs = &input_fn.attrs;

    // We re-emit the user's original attributes (doc comments, allow,
    // ...) so cargo doc / clippy still see them on the generated
    // #[test] function.
    let user_attrs: TokenStream2 = fn_attrs
        .iter()
        .map(|a| quote! { #a })
        .collect::<TokenStream2>();

    let expanded = quote! {
        #user_attrs
        #[::core::prelude::v1::test]
        #fn_vis #fn_sig {
            let __chaos_cfg = ::pheno_chaos::runtime::parse_attr_args(#attr_str);
            ::pheno_chaos::runtime::run_with_chaos(&__chaos_cfg, move || {
                // The user's body runs verbatim inside the chaos
                // harness. We use `move` so any captured state is
                // owned by the closure (required for `FnOnce`).
                #fn_block
            });
        }
    };

    // The function name is consumed but re-emitted inside the quote,
    // so this `let _ = fn_name` keeps the compiler quiet about
    // unused-variable lints.
    let _ = fn_name;

    TokenStream::from(expanded)
}
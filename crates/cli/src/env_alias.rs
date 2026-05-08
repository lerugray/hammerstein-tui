//! Backward-compatible env var aliasing for the Hammerstein rebrand
//! (cli crate copy — see also `hammerstein-tui/src/env_alias.rs`).
//!
//! Read `HAMMERSTEIN_X` first; if unset, fall back to `DEEPSEEK_X` and
//! emit a one-shot deprecation warning per legacy name. Drop-in for
//! `std::env::var` — same `Result<String, VarError>` shape.

use std::collections::HashSet;
use std::env::VarError;
use std::sync::Mutex;

pub fn var(hammerstein_name: &str, deepseek_name: &str) -> Result<String, VarError> {
    match std::env::var(hammerstein_name) {
        Ok(v) => Ok(v),
        Err(_) => match std::env::var(deepseek_name) {
            Ok(v) => {
                warn_deprecated_once(hammerstein_name, deepseek_name);
                Ok(v)
            }
            Err(e) => Err(e),
        },
    }
}

fn warn_deprecated_once(hammerstein_name: &str, deepseek_name: &str) {
    // No tracing subscriber wired up — write to stderr directly so the
    // message reaches the user. See hammerstein-tui/src/env_alias.rs for
    // the rationale.
    static SEEN: Mutex<Option<HashSet<String>>> = Mutex::new(None);
    let mut guard = match SEEN.lock() {
        Ok(g) => g,
        Err(_) => return,
    };
    let seen = guard.get_or_insert_with(HashSet::new);
    if seen.insert(deepseek_name.to_string()) {
        eprintln!(
            "warning: {deepseek_name} is deprecated; use {hammerstein_name} instead. \
             The DEEPSEEK_* alias will be removed in a future release."
        );
    }
}

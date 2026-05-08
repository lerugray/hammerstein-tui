//! Backward-compatible env var aliasing for the Hammerstein rebrand.
//!
//! Read `HAMMERSTEIN_X` first; if unset, fall back to `DEEPSEEK_X` and emit
//! a one-shot deprecation warning per legacy name via `tracing::warn`.
//!
//! Drop-in for `std::env::var` — same `Result<String, VarError>` shape so
//! existing `.is_ok_and(...)`, `.or_else(...)`, `.ok()`, and
//! `if let Ok(...)` call sites swap with no further refactor:
//!
//! ```ignore
//! // Before:
//! if let Ok(value) = std::env::var("DEEPSEEK_API_KEY") { ... }
//! // After:
//! if let Ok(value) = crate::env_alias::var("HAMMERSTEIN_API_KEY", "DEEPSEEK_API_KEY") { ... }
//! ```
//!
//! For env vars that hamt *sets* (hook injection, subprocess inheritance),
//! the convention is to write *both* names side by side at the call site
//! (see `hooks::HookContext::to_env_vars` and `cli::run_tui_command`)
//! so existing user hook scripts that read the legacy `DEEPSEEK_*` form
//! keep working through the rebrand.

use std::collections::HashSet;
use std::env::VarError;
use std::sync::Mutex;

/// Read `hammerstein_name` first, falling back to `deepseek_name`. Logs a
/// deprecation warning the first time only the legacy name is found.
///
/// Matches `std::env::var`'s `Result<String, VarError>` shape exactly so
/// every existing call-site pattern (`is_ok_and`, `or_else`, `.ok()`,
/// `if let Ok(...)`) is a true drop-in replacement.
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
    // hamt has no tracing subscriber wired up by default (`logging.rs`
    // is a thin colored::eprintln helper), so tracing::warn would
    // silently drop the message. Write straight to stderr instead so
    // the deprecation note is visible to users running on a legacy
    // env var.
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Process-global env mutations need a serialization lock — every test
    /// that touches `std::env` here grabs this guard before scribbling.
    static ENV_LOCK: Mutex<()> = Mutex::new(());

    /// Generate unique env var names per test so they don't collide if the
    /// suite is run in parallel with other tests that also touch `std::env`.
    fn fresh_pair(tag: &str) -> (String, String) {
        let pid = std::process::id();
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.subsec_nanos())
            .unwrap_or(0);
        (
            format!("HAMM_TEST_{tag}_{pid}_{nanos}"),
            format!("DEEP_TEST_{tag}_{pid}_{nanos}"),
        )
    }

    #[test]
    fn prefers_hammerstein_when_both_set() {
        let _g = ENV_LOCK.lock().unwrap();
        let (hamm, deep) = fresh_pair("both");
        unsafe {
            std::env::set_var(&hamm, "from-hammerstein");
            std::env::set_var(&deep, "from-deepseek");
        }
        assert_eq!(var(&hamm, &deep).unwrap(), "from-hammerstein");
        unsafe {
            std::env::remove_var(&hamm);
            std::env::remove_var(&deep);
        }
    }

    #[test]
    fn falls_back_to_deepseek_when_hammerstein_unset() {
        let _g = ENV_LOCK.lock().unwrap();
        let (hamm, deep) = fresh_pair("fallback");
        unsafe {
            std::env::set_var(&deep, "from-deepseek");
        }
        assert_eq!(var(&hamm, &deep).unwrap(), "from-deepseek");
        unsafe {
            std::env::remove_var(&deep);
        }
    }

    #[test]
    fn returns_not_present_when_neither_set() {
        let _g = ENV_LOCK.lock().unwrap();
        let (hamm, deep) = fresh_pair("neither");
        assert!(matches!(var(&hamm, &deep), Err(VarError::NotPresent)));
    }

    #[test]
    fn empty_hammerstein_value_is_treated_as_set() {
        let _g = ENV_LOCK.lock().unwrap();
        let (hamm, deep) = fresh_pair("empty");
        unsafe {
            std::env::set_var(&hamm, "");
            std::env::set_var(&deep, "fallback-should-be-ignored");
        }
        // Matches existing `std::env::var` semantics — an explicit empty
        // override beats a non-empty fallback.
        assert_eq!(var(&hamm, &deep).unwrap(), "");
        unsafe {
            std::env::remove_var(&hamm);
            std::env::remove_var(&deep);
        }
    }

}

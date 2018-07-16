//!
//! Testing configurations for `proptest` tests.
//!
//! `un_algebra` generative tests suites are built on the `proptest`
//! generative testing crate. `proptest` supports testing with run
//! time "configurations" that control, among other things, the number
//! of tests performed.
//!
//! The `config` module provides pre-defined testing configuration
//! helpers to `un_algebra` tests built on `proptest`.
//!
use proptest::test_runner::*;


///
/// A "standard" configuration with 10,000 cases, 50% rejects.
///
#[cfg(test)]
pub fn standard() -> Config {
  Config {
    cases: 10_000, max_global_rejects: 5_000, .. Config::default()
  }
}


///
/// A configuration with choice of cases and rejects.
///
#[cfg(test)]
pub fn config_with(cases: u32, rejects: u32) -> Config {
  Config {
    cases: cases, max_global_rejects: rejects, .. standard()
  }
}

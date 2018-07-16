//!
//! Boolean logic functions.
//!
//! Functions for testing boolean logic predicates.
//!

/// Test logical implication `x` => `y`.
pub fn implies(x: bool, y: bool) -> bool {
  (!x) || y
}

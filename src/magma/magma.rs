//!
//! Abstract algebraic _magma_ trait.
//!
//! An algebraic _magma_ is a set `S`, equipped with a _binary
//! operation_ `·`.
//!
//! # Axioms
//!
//! 1. Closure: ∀x, y ∈ S, x·y ∈ S.
//!
//! # References
//!
//! See [references] for a formal definition of a magma.
//!
#![doc(include = "../doc/references.md")]

use types::*;


///
/// An abstract algebraic _magma_.
///
pub trait Magma: Sized + PartialEq {

  /// Abstract binary operation on two magma values.
  fn op(&self, other: &Self) -> Self;


  /// Test the axiom of closure for magma values (always true).
  fn axiom_closure(_: Pair<Self>) -> bool {
    true
  }
}




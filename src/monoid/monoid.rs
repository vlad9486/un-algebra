//!
//! Algebraic _monoid_ trait.
//!
//! An algebraic _monoid_ is a _semigroup_ `S`, with a unique
//! _identity_ element denoted `e`.
//! 
//! # Axioms
//!
//! 1. Semigroup: semigroup axioms hold.
//! 2. Identity: ∃e ∈ S, ∀x ∈ S, e·x = x·e = x.
//!
//! # References
//!
//! See [references] for a formal definition of a monoid.
//!
#![doc(include = "../doc/references.md")]

use semigroup::semigroup::*;


///
/// An algebraic _monoid_.
///
pub trait Monoid: Semigroup {

  /// The monoid identity element.
  fn id() -> Self;


  /// Test for the identity value.
  fn is_id(&self) -> bool {
    *self == Self::id()
  }


  /// Test the left identity axiom.
  fn axiom_left_identity(&self) -> bool {
    Self::id().op(self) == Self::id()
  }


  /// Test the right identity axiom.
  fn axiom_right_identity(&self) -> bool {
    self.op(&Self::id()) == Self::id()
  }
}


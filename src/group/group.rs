//!
//! Algebraic _group_ trait.
//!
//! An algebraic _group_ is a _monoid_ `S`, where each group element
//! `g` has a unique _inverse_ element denoted `g^-1`.
//! 
//! # Axioms
//!
//! 1. Monoid: monoid axioms hold.
//! 2. Inverse: ∀g ∈ S, ∃g^-1 ∈ S, g·g^-1 = g^-1·g = e.
//!
//! # References
//!
//! See [references] for a formal definition of a group.
//!
#![doc(include = "../doc/references.md")]

use monoid::monoid::*;


///
/// An algebraic _group_.
///
pub trait Group: Monoid {

  /// The unique inverse of a group element.
  fn inverse(&self) -> Self;


  /// The cancellation of a group element.
  fn cancel(&self) -> Self {
    self.op(&self.inverse())
  }


  /// Test the left inverse axiom.
  fn axiom_left_inverse(&self) -> bool {
    self.inverse().op(self) == Self::id()
  }


  /// Test the right inverse axiom.
  fn axiom_right_inverse(&self) -> bool {
    self.op(&self.inverse()) == Self::id()
  }
}


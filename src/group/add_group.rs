//!
//! Algebraic _additive_ _group_ traits.
//!
//! An algebraic _additive_ _group_ is an _additive_ _monoid_ `S`,
//! where each group element `g` has a unique additive _inverse_
//! element denoted `-g`. The inverse operation is called
//! _negation_.
//!
//! # Axioms
//!
//! 1. Monoid: additive monoid axioms hold.
//! 2. Inverse: ∀g ∈ S, ∃-g ∈ S, g + -g = -g + g = 0.
//!
//! # References
//!
//! See [references] for a formal definition of an additive group.
//!
#![doc(include = "../doc/references.md")]

use monoid::add_monoid::*;


///
/// An algebraic _additive group_.
///
pub trait AddGroup: AddMonoid {

  /// The unique additive inverse of a group element.
  fn negate(&self) -> Self;


  /// The additive "subtraction" of two group elements.
  fn sub(&self, other: &Self) -> Self {
    self.add(&other.negate())
  }


  /// Test the left axiom of negation.
  fn axiom_left_negate(&self) -> bool {
    self.negate().add(self) == Self::zero()
  }


  /// Test the right axiom of negation.
  fn axiom_right_negate(&self) -> bool {
    self.add(&self.negate()) == Self::zero()
  }
}


///
/// A "numeric" algebraic _additive group_.
///
/// `NumAddGroup` trait is for types that only form additive groups
/// when "numeric" comparisons are used, e.g. floating point types.
///
pub trait NumAddGroup: NumAddMonoid {

  /// The unique additive inverse of a group element.
  fn negate(&self) -> Self;


  /// The additive "subtraction" of two group elements.
  fn sub(&self, other: &Self) -> Self {
    self.add(&other.negate())
  }


  /// Numerically test the left axiom of negation.
  fn axiom_left_negate(&self, eps: &Self::Error) -> bool {
    self.negate().add(self).num_eq(&Self::zero(), eps)
  }


  /// Numerically test the right axiom of negation.
  fn axiom_right_negate(&self, eps: &Self::Error) -> bool {
    self.add(&self.negate()).num_eq(&Self::zero(), eps)
  }
}


///
/// Trait implementation macro for integer types.
///
/// A macro used to avoid writing repetitive, boilerplate `AddGroup`
/// implementations for built-in signed integer types.  Probably not
/// needed if Rust had a signed `Integer` super-trait.
///
macro_rules! integer_add_group {
  ($type:ty) => {
    impl AddGroup for $type {

      /// Additive group negation uses "wrapping" negate to
      /// avoid overflow and guarantee the closure axiom.
      fn negate(&self) -> Self {
        self.wrapping_neg()
      }
    }
  };

  ($type:ty, $($others:ty),+) => {
    integer_add_group! {$type}
    integer_add_group! {$($others),+}
  };
}


// Only signed integers form an additive group.
integer_add_group! {
  i8, i16, i32, i64, i128, isize
}


///
/// IEEE 32 bit floating point types only form a _numeric_ additive
/// group.
///
impl NumAddGroup for f32 {

  // Negation is just floating point negation.
  fn negate(&self) -> Self {
    -*self
  }
}


///
/// IEEE 64 bit floating point types only form a _numeric_ additive
/// group.
///
impl NumAddGroup for f64 {

  // Negation is just floating point negation.
  fn negate(&self) -> Self {
    -*self
  }
}


// Module unit tests are in a separate file.
#[cfg(test)]
#[path = "add_group_test.rs"]
mod add_group_test;

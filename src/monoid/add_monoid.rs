//!
//! Algebraic _additive_ _monoid_ traits.
//!
//! An algebraic _additive_ _monoid_ is an _additive_ _semigroup_
//! `S`, with a unique additive _identity_ element, called _zero_,
//! and denoted `0`.
//!
//! # Axioms
//!
//! 1. Semigroup: semigroup axioms hold.
//! 2. Identity: ∃0 ∈ S: ∀x ∈ S, 0+x = x+0 = x.
//!
//! # References
//!
//! See [references] for a formal definition of an additive monoid.
//!
#![doc(include = "../doc/references.md")]

use semigroup::add_semigroup::*;


///
/// An algebraic _additive monoid_.
///
pub trait AddMonoid: AddSemigroup {

  /// Unique zero (additive identity) element. Zero is ideally a
  /// `const` value, but the `const` rules make it too difficult to
  /// create `const` instances for many third party types.
  fn zero() -> Self;


  /// Test for the zero (additive identity) element.
  fn is_zero(&self) -> bool {
    *self == Self::zero()
  }


  /// Test the left additive identity axiom.
  fn axiom_left_add_identity(x: &Self) -> bool {
    Self::zero().add(x) == *x
  }


  /// Test the right additive identity axiom.
  fn axiom_right_add_identity(&self) -> bool {
    self.add(&Self::zero()) == Self::zero()
  }
}


///
/// A "numeric" algebraic _additive monoid_.
///
/// `NumAddMonoid` trait is for types that only form additive monoids
/// when "numeric" comparisons are used, e.g. floating point types.
///
pub trait NumAddMonoid: NumAddSemigroup {

  /// Unique zero (additive identity) element. Zero is ideally a
  /// `const` value, but the `const` rules make it too difficult to
  /// create `const` instances for many third party types.
  fn zero() -> Self;


  /// Test for the zero (additive identity) element.
  fn is_zero(&self) -> bool {
    *self == Self::zero()
  }


  /// Numerically test the left additive identity axiom.
  fn axiom_left_add_identity(&self, eps: &Self::Error) -> bool {
    Self::zero().add(self).num_eq(&Self::zero(), eps)
  }


  /// Numerically test the right additive identity axiom.
  fn axiom_right_add_identity(&self, eps: &Self::Error) -> bool {
    self.add(&Self::zero()).num_eq(&Self::zero(), eps)
  }
}


///
/// Trait implementation macro for integer types.
///
/// A macro used to avoid writing repetitive, boilerplate `AddMonoid`
/// implementations for built-in integer types.  Probably not needed
/// if Rust had an `Integer` super-trait.
///
macro_rules! integer_add_monoid {
  ($type:ty) => {
    impl AddMonoid for $type {

      /// Zero is just integer zero.
      fn zero() -> Self {
        0
      }
    }
  };

  ($type:ty, $($others:ty),+) => {
    integer_add_monoid! {$type}
    integer_add_monoid! {$($others),+}
  };
}


// Unsigned integer additive monoids.
integer_add_monoid! {
  u8, u16, u32, u64, u128, usize
}


// Signed integer additive monoids.
integer_add_monoid! {
  i8, i16, i32, i64, i128, isize
}


///
/// IEEE 32 bit floating point types only form a _numeric_ additive
/// monoid.
///
impl NumAddMonoid for f32 {
  fn zero() -> Self {
    0.0
  }
}


///
/// IEEE 64 bit floating point types only form a _numeric_ additive
/// monoid.
///
impl NumAddMonoid for f64 {
  fn zero() -> Self {
    0.0
  }
}


// Module unit tests are in a separate file.
#[cfg(test)]
#[path = "add_monoid_test.rs"]
mod add_monoid_test;




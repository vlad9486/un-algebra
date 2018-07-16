//!
//! Algebraic _multiplicative_ _monoid_ traits.
//!
//! An algebraic _multiplicative_ _monoid_ is a _multiplicative_
//! _semigroup_ `S`, with a unique multiplicative _identity_
//! element, called _one_, and denoted `1`.
//!
//! # Axioms
//!
//! 1. Semigroup: semigroup axioms hold.
//! 2. Identity: ∃1 ∈ S, ∀x ∈ S, 1\*x = x\*1 = x.
//!
//! # References
//!
//! See [references] for a formal definition of a multiplicative
//! monoid.
//!
#![doc(include = "../doc/references.md")]

use semigroup::mul_semigroup::*;


///
/// An algebraic _multiplicative monoid_.
///
pub trait MulMonoid: MulSemigroup {

  /// Unique one (multiplicative identity) element. One is ideally a
  /// `const` value, but the `const` rules make it too difficult to
  /// create `const` instances for many third party types.
  fn one() -> Self;


  /// Test for the one (multiplicative identity) element.
  fn is_one(&self) -> bool {
    *self == Self::one()
  }


  /// Test the left multiplicative identity axiom.
  fn axiom_left_mul_identity(&self) -> bool {
    Self::one().mul(self) == Self::one()
  }


  /// Test the right multiplicative identity axiom.
  fn axiom_right_mul_identity(&self) -> bool {
    self.mul(&Self::one()) == Self::one()
  }
}


///
/// A "numeric" algebraic _multiplicative monoid_.
///
/// `NumMulMonoid` trait is for types that only form multiplicative
/// monoids when "numeric" comparisons are used, e.g. floating point
/// types.
///
pub trait NumMulMonoid: NumMulSemigroup {

  /// Unique one (multiplicative identity) element. One is ideally a
  /// `const` value, but the `const` rules make it too difficult to
  /// create `const` instances for many third party types.
  fn one() -> Self;


  /// Test for the one (multiplicative identity) element.
  fn is_one(&self) -> bool {
    *self == Self::one()
  }


  /// Numerically test the left multiplicative identity axiom.
  fn axiom_left_mul_identity(&self, eps: &Self::Error) -> bool {
    Self::one().mul(self).num_eq(&Self::one(), eps)
  }


  /// Numerically test the right multiplicative identity axiom.
  fn axiom_right_mul_identity(&self, eps: &Self::Error) -> bool {
    self.mul(&Self::one()).num_eq(&Self::one(), eps)
  }
}


///
/// Trait implementation macro for integer types.
///
/// A macro used to avoid writing repetitive, boilerplate `MulMonoid`
/// implementations for built-in integer types.  Probably not needed
/// if Rust had an `Integer` super-trait.
///
macro_rules! integer_mul_monoid {
  ($type:ty) => {
    impl MulMonoid for $type {

      /// One is just integer one.
      fn one() -> Self {
        1
      }
    }
  };

  ($type:ty, $($others:ty),+) => {
    integer_mul_monoid! {$type}
    integer_mul_monoid! {$($others),+}
  };
}


// Unsigned integer multiplicative monoids.
integer_mul_monoid! {
  u8, u16, u32, u64, u128, usize
}


// Signed integer multiplicative monoids.
integer_mul_monoid! {
  i8, i16, i32, i64, i128, isize
}


///
/// IEEE 32 bit floating point types only form a _numeric_
/// multiplicative monoid.
///
impl NumMulMonoid for f32 {
  fn one() -> Self {
    1.0
  }
}


///
/// IEEE 64 bit floating point types only form a _numeric_
/// multiplicative monoid.
///
impl NumMulMonoid for f64 {
  fn one() -> Self {
    1.0
  }
}


// Module unit tests are in a separate file.
#[cfg(test)]
#[path = "mul_monoid_test.rs"]
mod mul_monoid_test;






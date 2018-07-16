//!
//! Algebraic trait implementations for _rational_ numbers.
//!
//! The rational numbers (&#x211a;) form a _field_. We assume the
//! _numerator_ and _denominator_ of a rational number are unbounded
//! signed integers. Rationals implemented using Rust integer types
//! (e.g. i64) overflow too easily during generative testing.
//!
//! In the absence of a rational number type in the Rust standard
//! library, the "big rational" number type used here is from the very
//! handy [num] crate.
//!
#![doc(include = "../doc/references.md")]

use prelude::*;
pub use num::traits::*;
pub use num::rational::*;


///
/// Rational numbers form an additive magma.
///
impl AddMagma for BigRational {

  /// Addition is rational addition.
  fn add(&self, other: &Self) -> Self {
    self + other
  }
}


///
/// Rational numbers form an additive semigroup.
///
impl AddSemigroup for BigRational {}


///
/// Rational numbers form an additive monoid.
///
impl AddMonoid for BigRational {

  /// Zero is rational zero.
  fn zero() -> Self {
    Self::from_integer(Zero::zero())
  }
}


///
/// Rational numbers form an additive group.
///
impl AddGroup for BigRational {

  /// Negation is rational negation.
  fn negate(&self) -> Self {
    -self
  }
}


///
/// Rational numbers form an additive commutative group.
///
impl AddComGroup for BigRational {}

  
///
/// Rational numbers form a multiplicative magma.
///
impl MulMagma for BigRational {

  /// Multiplication is rational multiplication.
  fn mul(&self, other: &Self) -> Self {
    self * other
  }
}


///
/// Rational numbers form a multiplicative semigroup.
///
impl MulSemigroup for BigRational {}


///
/// Rational numbers form a multiplicative monoid.
///
impl MulMonoid for BigRational {

  /// One is rational one.
  fn one() -> Self {
    Self::from_integer(One::one())
  }
}


///
/// Rational numbers (without zero) form a multiplicative group.
///
impl MulGroup for BigRational {

  /// Inversion is rational reciprocal.
  fn invert(&self) -> Self {
    self.recip()
  }

  
  /// Non-zero rationals are invertible.
  fn is_invertible(&self) -> bool {
    *self != Self::from_integer(Zero::zero())
  }
}


///
/// Rational numbers (without zero) form a multiplicative
/// commutative group.
///
impl MulComGroup for BigRational {}


///
/// Rational numbers form a ring.
///
impl Ring for BigRational {}


///
/// Rational numbers form a commutative ring.
///
impl ComRing for BigRational {}


///
/// Rational numbers (without zero) form a field.
///
impl Field for BigRational {

  /// Inversion is rational reciprocal.
  fn invert(&self) -> Self {
    self.recip()
  }  
}


// Module unit tests are in a separate file.
#[cfg(test)]
#[path = "rational_test.rs"]
mod rational_test;


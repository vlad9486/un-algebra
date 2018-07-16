//!
//! Algebraic _field_ traits.
//!
//! An algebraic _field_ is a _commutative_ _ring_ `S`, where each
//! _invertible_ field element `f` has a unique _multiplicative_
//! _inverse_ `f^-1`. The inverse operation is called _invert_.
//!
//! # Axioms
//!
//! 1. Ring: commutative ring axioms hold.
//! 2. Inverse: ∀g ∈ S, ∃g^-1 ∈ S, g\*g^-1 = g^-1\*g = 1.
//!
//! # References
//!
//! See [references] for a formal definition of a field.
//!
#![doc(include = "../doc/references.md")]

use types::*;
use logic::*;
use com_ring::com_ring::*;


///
/// An algebraic _field_.
///
pub trait Field: ComRing {

  /// The unique multiplicative inverse of a field element. The
  /// inverse is only defined for _invertible_ field elements.
  fn invert(&self) -> Self;


  /// Test for an _invertible_ field element.
  fn is_invertible(&self) -> bool {
    *self != Self::zero()
  }


  /// The multiplicative "division" of two field elements.
  fn div(&self, other: &Self) -> Self {
    self.mul(&other.invert())
  }


  /// Test the left axiom of inversion.
  fn axiom_left_invert(&self) -> bool {
     self.invert().mul(self) == Self::one()
  }


  /// Test the right axiom of inversion.
  fn axiom_right_invert(&self) -> bool {
    self.mul(&self.invert()) == Self::one()
  }


  /// Test the property of zero multiplication.
  fn prop_mul_zero(xs: Pair<Self>) -> bool {
    let (x, y) = xs;

    implies(x.mul(y).is_zero(), x.is_zero() || y.is_zero())
  }


  /// Test the property of additive cancellation.
  fn prop_add_cancel(xs: Triple<Self>) -> bool {
    let (x, y, z) = xs;

    implies(x.add(y) == z.add(y), x == z)
  }


  /// Test the property of multiplicative cancellation.
  fn prop_mul_cancel(xs: Triple<Self>) -> bool {
    let (x, y, z) = xs;

    implies(!y.is_zero() && x.mul(y) == z.mul(y), x == z)
  }
}


///
/// A "numeric" algebraic _field_.
///
/// `NumField` trait is for types that only form fields when "numeric"
/// comparisons are used, e.g. floating point types.
///
pub trait NumField: NumComRing {

  /// The unique multiplicative inverse of a field element. Note that
  /// `invert` is only defined for _invertible_ field elements.
  fn invert(&self) -> Self;


  /// Test for an _invertible_ field element.
  fn is_invertible(&self) -> bool {
    *self != Self::zero()
  }


  /// The multiplicative "division" of two field elements.
  fn div(&self, other: &Self) -> Self {
    self.mul(&other.invert())
  }


  /// Numerically test the left axiom of inversion.
  fn axiom_left_invert(&self, eps: &Self::Error) -> bool {
    self.invert().mul(self).num_eq(&Self::one(), eps)
  }


  /// Numerically test the right axiom of inversion.
  fn axiom_right_invert(&self, eps: &Self::Error) -> bool {
    self.mul(&Self::invert(self)).num_eq(&Self::one(), eps)
  }


  /// Numerically test the property of zero multiplication.
  fn prop_mul_zero(xs: NumPair<Self>) -> bool {
    let (x, y, _) = xs;

    implies(x.mul(y).is_zero(), x.is_zero() || y.is_zero())
  }


  /// Numerically test the property of additive cancellation.
  fn prop_add_cancel(xs: NumTriple<Self>) -> bool {
    let (x, y, z, eps) = xs;

    implies(x.add(y).num_eq(&z.add(y), eps), x.num_eq(z, eps))
  }


  /// Numerically test the property of multiplicative cancellation.
  fn prop_mul_cancel(xs: NumTriple<Self>) -> bool {
    let (x, y, z, eps) = xs;
    let x_eq_z = x.num_eq(z, eps);

    implies(!y.is_zero() && x.mul(y).num_eq(&z.mul(y), eps), x_eq_z)
  }
}


///
/// IEEE 64 bit floating point types only form a _numeric_ field.
///
impl NumField for f32 {

  /// Inversion is just floating point inverse.
  fn invert(&self) -> Self {
    self.recip()
  }
}


///
/// IEEE 64 bit floating point types only form a _numeric_ field.
///
impl NumField for f64 {

  /// Inversion is just floating point inverse.
  fn invert(&self) -> Self {
    self.recip()
  }
}


// Module unit tests are in a separate file.
#[cfg(test)]
#[path = "field_test.rs"]
mod field_test;

//!
//! Algebraic _ring_ traits.
//!
//! An algebraic _ring_ `S`, is an _additive_ _commutative_ _group
//! **and** a _multiplicative_ _monoid_, and therefore has both
//! _addition_ `+` and _multiplication_ `*` operators. It also has
//! unique _zero_ `0` and _one_ `1` identity elements.
//!
//! Ring multiplication is required to _distribute_ over addition.
//!
//! # Axioms
//!
//! 1. Group: additive commutative group axioms hold.
//! 2. Monoid: multiplicative monoid axioms hold.
//! 3. Distributivity (L): x, y, z ∈ R, x\*(y+z) = x\*y + x\*z.
//! 4. Distributivity (R): x, y, z ∈ R, (x+y)\*z = x\*z + y\*z.
//!
//! # References
//!
//! See [references] for a formal definition of a ring.
//!
#![doc(include = "../doc/references.md")]

use types::*;
use monoid::mul_monoid::*;
use com_group::add_com_group::*;


///
/// An algebraic _ring_.
///
pub trait Ring: AddComGroup + MulMonoid {

  /// Test the ring axiom of identities
  fn axiom_identities() -> bool {
    Self::zero() != Self::one()
  }


  /// Test the axiom of left distributivity.
  fn axiom_left_distributivity(xs: Triple<Self>) -> bool {
    let (x, y, z) = xs;

    x.mul(&y.add(z)) == x.mul(y).add(&x.mul(z))
  }


  /// Test the axiom of right distributivity.
  fn axiom_right_distributivity(xs: Triple<Self>) -> bool {
    let (x, y, z) = xs;

    y.add(z).mul(x) == y.mul(x).add(&z.mul(x))
  }


  /// Test the property of left zero absorbption.
  fn prop_left_zero_absorb(&self) -> bool {
    self.mul(&Self::zero()) == Self::zero()
  }


  /// Test the property of right zero absorbption.
  fn prop_right_zero_absorb(&self) -> bool {
    Self::zero().mul(self) == Self::zero()
  }


  /// Test the property of right multiplicative negation.
  fn prop_right_mul_negate(xs: Pair<Self>) -> bool {
    let (x, y) = xs;

    x.mul(&y.negate()) == x.mul(y).negate()
  }


  /// Test the property of left multiplicative negation.
  fn prop_left_mul_negate(xs: Pair<Self>) -> bool {
    let (x, y) = xs;

    x.negate().mul(y) == x.mul(y).negate()
  }


  /// Test the property of multiplicative negation.
  fn prop_mul_negate(xs: Pair<Self>) -> bool {
    let (x, y) = xs;

    x.negate().mul(&y.negate()) == x.mul(y)
  }


  /// Test the property of left one negation.
  fn prop_left_one_negate(&self) -> bool {
    Self::one().negate().mul(self) == self.negate()
  }


  /// Test the property of right one negation.
  fn prop_right_one_negate(&self) -> bool {
    self.mul(&Self::one().negate()) == self.negate()
  }


  /// Test the property of right ones negation.
  fn prop_right_ones_negate(&self) -> bool {
    Self::one().negate().mul(&Self::one().negate()) == Self::one()
  }
}


///
/// A "numeric" algebraic _ring_.
///
/// `NumRing` trait is for types that only form rings when "numeric"
/// comparisons are allowed, e.g. floating point types.
///
pub trait NumRing: NumAddComGroup + NumMulMonoid {

  /// Test the axiom of identities.
  fn axiom_identities() -> bool {
    Self::zero() != Self::one()
  }


  /// Numerically test the axiom of left distributivity.
  fn axiom_left_distributivity(xs: NumTriple<Self>) -> bool {
    let (x, y, z, eps) = xs;

    x.mul(&y.add(z)).num_eq(&x.mul(y).add(&x.mul(z)), eps)
  }


  /// Numerically test the axiom of right distributivity.
  fn axiom_right_distributivity(xs: NumTriple<Self>) -> bool {
    let (x, y, z, eps) = xs;

    y.add(z).mul(x).num_eq(&y.mul(x).add(&z.mul(x)), eps)
  }


  /// Numerically test the property of left zero absorbption.
  fn prop_left_zero_absorb(&self, eps: &Self::Error) -> bool {
    self.mul(&Self::zero()).num_eq(&Self::zero(), eps)
  }


  /// Numerically test the property of right zero absorbption.
  fn prop_right_zero_absorb(&self, eps: &Self::Error) -> bool {
    Self::zero().mul(self).num_eq(&Self::zero(), eps)
  }


  /// Numerically test the property of right multiply negation.
  fn prop_right_mul_negate(xs: NumPair<Self>) -> bool {
    let (x, y, eps) = xs;

    x.mul(&y.negate()).num_eq(&x.mul(y).negate(), eps)
  }


  /// Numerically test the property of left multiply negation.
  fn prop_left_mul_negate(xs: NumPair<Self>) -> bool {
    let (x, y, eps) = xs;

    x.negate().mul(y).num_eq(&x.mul(y).negate(), eps)
  }


  /// Numerically test the property of multiply negation.
  fn prop_mul_negate(xs: NumPair<Self>) -> bool {
    let (x, y, eps) = xs;

    x.negate().mul(&y.negate()).num_eq(&x.mul(y), eps)
  }


  /// Numerically test the property of left one negation.
  fn prop_left_one_negate(&self, eps: &Self::Error) -> bool {
    Self::one().negate().mul(self).num_eq(&self.negate(), eps)
  }


  /// Numerically test the property of right one negation.
  fn prop_right_one_negate(&self, eps: &Self::Error) -> bool {
    self.mul(&Self::one().negate()).num_eq(&self.negate(), eps)
  }


  /// Numerically test the property of right ones negation.
  fn prop_right_ones_negate(&self, eps: &Self::Error) -> bool {
    Self::one().negate().mul(&Self::one().negate()).num_eq(&Self::one(), eps)
  }
}


///
/// Trait implementation macro for integer types.
///
/// A macro used to avoid writing repetitive, boilerplate `Ring`
/// implementations for built-in signed integer types. Probably not
/// needed if Rust had a signed `Integer` super-trait.
///
macro_rules! integer_ring {
  ($type:ty) => {
    impl Ring for $type {}
  };

  ($type:ty, $($others:ty),+) => {
    integer_ring! {$type}
    integer_ring! {$($others),+}
  };
}


// Only signed integers form a ring.
integer_ring! {
  i8, i16, i32, i64, i128, isize
}


///
/// IEEE 64 bit floating point types only form a _numeric_ ring.
///
impl NumRing for f32 {}


///
/// IEEE 64 bit floating point types only form a _numeric_ ring.
///
impl NumRing for f64 {}


// Module unit tests are in a separate file.
#[cfg(test)]
#[path = "ring_test.rs"]
mod ring_test;




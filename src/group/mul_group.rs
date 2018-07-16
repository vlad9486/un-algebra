//!
//! Algebraic _multiplicative_ _group_ traits.
//!
//! An algebraic _multiplicative_ _group_ is a _multiplicative_
//! _monoid_ `S`, where each _invertible_ group element `g` has a
//! unique multiplicative _inverse_ denoted `g^-1`.  The inverse
//! operation is called _invert_.
//!
//! # Axioms
//!
//! 1. Monoid: multiplicative monoid axioms hold.
//! 2. Inverse: ∀g ∈ S, ∃g^-1 ∈ S, g \* g^-1 = g^-1 \* g = 1.
//!
//! # References
//!
//! See [references] for a formal definition of a multiplicative
//! group.
//!
#![doc(include = "../doc/references.md")]

use monoid::mul_monoid::*;


///
/// An algebraic _multiplicative group_.
///
pub trait MulGroup: MulMonoid {

  /// The unique multiplicative inverse of a group element. Inversion
  /// is only defined for _invertible_ group elements.
  fn invert(&self) -> Self;


  /// Test for an _invertible_ group element.
  fn is_invertible(&self) -> bool;


  /// The multiplicative "division" of two group elements.
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
}


///
/// A "numeric" algebraic _multiplicative group_.
///
/// `NumAddGroup` trait is for types that only form multiplicative
/// groups when "numeric" comparisons are used, e.g. floating point
/// types.
///
pub trait NumMulGroup: NumMulMonoid {

  /// The unique multiplicative inverse of a group element. Inversion
  /// is only defined for _invertible_ group elements.
  fn invert(&self) -> Self;


  /// Test for an _invertible_ group element.
  fn is_invertible(&self) -> bool;


  /// The multiplicative "division" of two group elements.
  fn div(&self, other: &Self) -> Self {
    self.mul(&other.invert())
  }


  /// Numerically test the left axiom of inversion.
  fn axiom_left_invert(&self, eps: &Self::Error) -> bool {
    self.invert().mul(self).num_eq(&Self::one(), eps)
  }


  /// Numerically test the right axiom of inversion.
  fn axiom_right_invert(&self, eps: &Self::Error) -> bool {
    self.mul(&self.invert()).num_eq(&Self::one(), eps)
  }
}


///
/// IEEE 32 bit floating point types only form a _numeric_
/// multiplicative group.
///
impl NumMulGroup for f32 {

  /// Inversion is just floating point inversion.
  fn invert(&self) -> Self {
    1.0 / *self
  }


  /// Non-zero elements are invertible.
  fn is_invertible(&self) -> bool {
    *self != 0.0
  }
}


///
/// IEEE 64 bit floating point types only form a _numeric_
/// multiplicative group.
///
impl NumMulGroup for f64 {

  /// Inversion is just floating point inversion.
  fn invert(&self) -> Self {
    1.0 / *self
  }


  /// Non-zero elements are invertible.
  fn is_invertible(&self) -> bool {
    *self != 0.0
  }
}


// Module unit tests are in a separate file.
#[cfg(test)]
#[path = "mul_group_test.rs"]
mod mul_group_test;



//!
//! Algebraic _multiplicative magma_ traits.
//!
//! An algebraic _multiplicative_ _magma_ is a set `S`, equipped
//! with a _binary operation_ `*`, called _multiplication_.
//!
//! # Axioms
//!
//! 1. Closure: ∀x, y ∈ S, x*y ∈ S.
//!
//! # References
//!
//! See [references] for a formal definition of a multiplicative
//! magma.
//!
#![doc(include = "../doc/references.md")]

use types::*;
use numeric::equal::*;


///
/// An algebraic _multiplicative magma_.
///
pub trait MulMagma: Sized + PartialEq {

  /// The magma `multiplication` binary operation.
  fn mul(&self, other: &Self) -> Self;


  /// Test the axiom of closure (always true).
  fn axiom_mul_closure(_: Pair<Self>) -> bool {
    true
  }
}


///
/// A "numeric" algebraic _multiplicative magma_.
///
/// `NumMulMagma` trait is for types that only form multiplicative
/// magmas when "numeric" comparisons are used, e.g. floating point
/// types.
///
pub trait NumMulMagma: Sized + NumEq {

  /// The `multiplication` binary operation.
  fn mul(&self, other: &Self) -> Self;


  /// Numerically test the axiom of closure (always true).
  fn axiom_mul_closure(_: NumPair<Self>) -> bool {
    true
  }
}


///
/// Trait implementation macro for integer types.
///
/// A macro used to avoid writing repetitive, boilerplate `MulMagma`
/// implementations for built-in integer types. Probably not needed if
/// Rust had an `Integer` super-trait.
///
macro_rules! integer_mul_magma {
  ($type:ty) => {
    impl MulMagma for $type {

      /// Multiplicative magma multiplication uses "wrapping"
      /// multiply to avoid overflow and guarantee the closure
      /// axiom.
      fn mul(&self, other: &Self) -> Self {
        self.wrapping_mul(*other)
      }
    }
  };

  ($type:ty, $($others:ty),+) => {
    integer_mul_magma! {$type}
    integer_mul_magma! {$($others),+}
  };
}


// Unsigned integer multiplicative magmas.
integer_mul_magma! {
  u8, u16, u32, u64, u128, usize
}


// Signed integer multiplicative magmas.
integer_mul_magma! {
  i8, i16, i32, i64, i128, isize
}


///
/// IEEE 32 bit floating point types only form a _numeric_
/// multiplicative magma.
///
impl NumMulMagma for f32 {

  /// Magma multiplication is just f32 multiplication.
  fn mul(&self, other: &Self) -> Self {
    *self * *other
  }
}


///
/// IEEE 64 bit floating point types only form a _numeric_
/// multiplicative magma.
///
impl NumMulMagma for f64 {

  /// Magma multiplication is just f64 multiplication.
  fn mul(&self, other: &Self) -> Self {
    *self * *other
  }
}


// Module unit tests are in a separate file.
#[cfg(test)]
#[path = "mul_magma_test.rs"]
mod mul_magma_test;









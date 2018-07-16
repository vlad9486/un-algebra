//!
//! Algebraic _multiplicative_ _semigroup_ traits.
//!
//! An algebraic _multiplicative_ _semigroup_ is a _multiplicative_
//! _magma_ `S`, where the `addition` operation `*` is _associative_.
//!
//! # Axioms
//!
//! 1. Magma: multiplicative magma axioms hold.
//! 2. Associativity: ∀x, y, z ∈ S (x\*y)\*z = x\*(y\*z).
//!
//! # References
//!
//! See [references] for a formal definition of a multiplicative
//! semigroup.
//!
#![doc(include = "../doc/references.md")]

use types::*;
use magma::mul_magma::*;


///
/// An algebraic _multiplicative semigroup_.
///
pub trait MulSemigroup: MulMagma {

  /// Test the axiom of associativity.
  fn axiom_mul_associativity(xs: Triple<Self>) -> bool {
    let (x, y, z) = xs;

    x.mul(&y.mul(z)) == x.mul(y).mul(z)
  }
}


///
/// A "numeric" algebraic _multiplicative semigroup_.
///
/// `NumMulSemigroup` trait is for types that only form multiplicative
/// semigroups when "numeric" comparisons are used, e.g. floating
/// point types.
///
pub trait NumMulSemigroup: NumMulMagma {

  /// Numerically test the axiom of associativity.
  fn axiom_mul_associativity(xs: NumTriple<Self>) -> bool {
    let (x, y, z, eps) = xs;

    x.mul(&y.mul(z)).num_eq(&x.mul(y).mul(z), eps)
  }
}


///
/// Trait implementation macro for integer types.
///
/// A macro used to avoid writing repetitive, boilerplate
/// `MulSemigroup` implementations for built-in integer types.
/// Probably not needed if Rust had an `Integer` super-trait.
///
macro_rules! integer_mul_semigroup {
  ($type:ty) => {
    impl MulSemigroup for $type {}
  };

  ($type:ty, $($others:ty),+) => {
    integer_mul_semigroup! {$type}
    integer_mul_semigroup! {$($others),+}
  };
}


// Unsigned integer multiplicative semigroups.
integer_mul_semigroup! {
  u8, u16, u32, u64, u128, usize
}


// Signed integer multiplicative semigroups.
integer_mul_semigroup! {
  i8, i16, i32, i64, i128, isize
}


///
/// IEEE 32 bit floating point types only form a _numeric_
/// multiplicative semigroup.
///
impl NumMulSemigroup for f32 {}


///
/// IEEE 64 bit floating point types only form a _numeric_
/// multiplicative semigroup.
///
impl NumMulSemigroup for f64 {}


// Module unit tests are in a separate file.
#[cfg(test)]
#[path = "mul_semigroup_test.rs"]
mod mul_semigroup_test;




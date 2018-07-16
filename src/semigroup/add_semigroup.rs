//!
//! Algebraic _additive_ _semigroup_ traits.
//!
//! An algebraic _additive_ _semigroup_ is an _additive_ _magma_
//! `S`, where the `addition` operation `+` is _associative_.
//!
//! # Axioms
//!
//! 1. Magma: additive magma axioms must hold.
//! 2. Associativity: ∀x, y, z ∈ S (x+y)+z = x+(y+z).
//!
//! # References
//!
//! See [references] for a formal definition of an additive semigroup.
//!
#![doc(include = "../doc/references.md")]

use types::*;
use magma::add_magma::*;


///
/// An algebraic _additive semigroup_.
///
pub trait AddSemigroup: AddMagma {

  /// Test the axiom of associativity.
  fn axiom_add_associativity(xs: Triple<Self>) -> bool {
    let (x, y, z) = xs;

    x.add(&y.add(z)) == x.add(y).add(z)
  }
}


///
/// A "numeric" algebraic _additive semigroup_.
///
/// `NumAddSemigroup` trait is for types that only form additive
/// semigroups when "numeric" comparisons are used, e.g. floating
/// point types.
///
pub trait NumAddSemigroup: NumAddMagma {

  /// Numerically test the axiom of associativity.
  fn axiom_add_associativity(xs: NumTriple<Self>) -> bool {
    let (x, y, z, eps) = xs;

    x.add(&y.add(z)).num_eq(&x.add(y).add(z), eps)
  }
}


///
/// Trait implementation macro for integer types.
///
/// A macro used to avoid writing repetitive, boilerplate
/// `AddSemigroup` implementations for built-in integer types.
/// Probably not needed if Rust had an `Integer` super-trait.
///
macro_rules! integer_add_semigroup {
  ($type:ty) => {
    impl AddSemigroup for $type {}
  };

  ($type:ty, $($others:ty),+) => {
    integer_add_semigroup! {$type}
    integer_add_semigroup! {$($others),+}
  };
}


// Unsigned integer additive semigroups.
integer_add_semigroup! {
  u8, u16, u32, u64, u128, usize
}


// Signed integer additive semigroups.
integer_add_semigroup! {
  i8, i16, i32, i64, i128, isize
}


///
/// IEEE 32 bit floating point types only form a _numeric_ additive
/// semigroup.
///
impl NumAddSemigroup for f32 {}


///
/// IEEE 64 bit floating point types only form a _numeric_ additive
/// semigroup.
///
impl NumAddSemigroup for f64 {}


// Module unit tests are in a separate file.
#[cfg(test)]
#[path = "add_semigroup_test.rs"]
mod add_semigroup_test;




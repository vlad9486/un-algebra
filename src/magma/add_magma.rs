//!
//! Algebraic _additive magma_ traits.
//!
//! An algebraic _additive_ _magma_ is a set `S`, equipped with a
//! _binary operation_ `+`, called _addition_.
//!
//! # Axioms
//!
//! 1. Closure: ∀x, y ∈ S, x+y ∈ S.
//!
//! # References
//!
//! See [references] for a formal definition of an additive magma.
//!
#![doc(include = "../doc/references.md")]

use types::*;
use num::traits::*;
use numeric::equal::*;


///
/// An algebraic _additive magma_.
///
pub trait AddMagma: Sized + PartialEq {

  /// The magma `addition` binary operation.
  fn add(&self, other: &Self) -> Self;


  /// Test the axiom of closure (always true).
  fn axiom_add_closure(_: Pair<Self>) -> bool {
    true
  }
}


///
/// A "numeric" algebraic _additive magma_.
///
/// `NumAddMagma` trait is for types that only form additive magmas
/// when "numeric" comparisons are used, e.g. floating point types.
///
pub trait NumAddMagma: Sized + NumEq {

  /// The magma `addition` binary operation.
  fn add(&self, other: &Self) -> Self;


  /// Numerically test the axiom of closure (always true).
  fn axiom_add_closure(_: NumPair<Self>) -> bool {
    true
  }
}


///
/// Trait implementation macro for integer types.
///
/// A macro used to avoid writing repetitive, boilerplate `AddMagma`
/// implementations for built-in integer types. Probably not needed if
/// Rust had an `Integer` super-trait.
///
macro_rules! integer_add_magma {
  ($type:ty) => {

    impl AddMagma for $type {

      /// Additive magma addition uses "wrapping" add to avoid
      /// overflow and guarantee the closure axiom.
      fn add(&self, other: &Self) -> Self {
        self.wrapping_add(other)
      }
    }
  };

  ($type:ty, $($others:ty),+) => {
    integer_add_magma! {$type}
    integer_add_magma! {$($others),+}
  };
}


// AddMagma implementation for unsigned integer types.
integer_add_magma! {
  u8, u16, u32, u64, u128, usize
}


// AddMagma implementation for signed integer types.
integer_add_magma! {
  i8, i16, i32, i64, i128, isize
}


///
/// IEEE 64 bit floating point types only form a _numeric_ additive
/// magma.
///
impl NumAddMagma for f32 {

  /// Magma addition is just f32 addition.
  fn add(&self, other: &Self) -> Self {
    *self + *other
  }
}


///
/// IEEE 64 bit floating point types only form a _numeric_ additive
/// magma.
///
impl NumAddMagma for f64 {

  /// Magma addition is just f64 addition.
  fn add(&self, other: &Self) -> Self {
    *self + *other
  }
}


// Module unit tests are in a separate file.
#[cfg(test)]
#[path = "add_magma_test.rs"]
mod add_magma_test;







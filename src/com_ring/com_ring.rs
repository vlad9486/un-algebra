//!
//! Algebraic _commutative_ _ring_ traits.
//!
//! An algebraic _commutative_ _ring_ `S`, is a _ring_ where the
//! _multiplication_ operation `*` is _commutative_.
//!
//! # Axioms
//!
//! 1. Ring: ring axioms hold.
//! 2. Commutivity: ∀g, h ∈ S, g\*h = h\*g.
//!
//! # References
//!
//! See [references] for a formal definition of a commutative ring.
//!
#![doc(include = "../doc/references.md")]

use types::*;
use ring::ring::*;


///
/// An algebraic _commutative ring_.
///
pub trait ComRing: Ring {

  /// Test the axiom of multiplicative commutivity.
  fn axiom_mul_commutivity(xs: Pair<Self>) -> bool {
    let (x, y) = xs;

    x.mul(y) == y.mul(x)
  }
}


///
/// A "numeric" algebraic _commutative ring_.
///
/// `NumComRing` trait is for types that only form commutative rings
/// when "numeric" comparisons are used, e.g. floating point types.
///
pub trait NumComRing: NumRing {

  /// Numerically test the axiom of commutivity.
  fn axiom_mul_commutivity(xs: NumPair<Self>) -> bool {
     let (x, y, eps) = xs;

     x.mul(y).num_eq(&y.mul(x), eps)
   }
}


///
/// Trait implementation macro for integer types.
///
/// A macro used to avoid writing repetitive, boilerplate `ComRing`
/// implementations for built-in integer types.  Probably not needed
/// if Rust had an `Integer` super-trait.
///
macro_rules! integer_com_ring {
  ($type:ty) => {
    impl ComRing for $type {}
  };

  ($type:ty, $($others:ty),+) => {
    integer_com_ring! {$type}
    integer_com_ring! {$($others),+}
  };
}


// Only signed integer types form a commutative ring.
integer_com_ring! {
  i8, i16, i32, i64, i128, isize
}


///
/// IEEE 32 bit floating point types only form a _numeric_ commutative
/// ring.
///
impl NumComRing for f32 {}


///
/// IEEE 64 bit floating point types only form a _numeric_ commutative
/// ring.
///
impl NumComRing for f64 {}


// Module unit tests are in a separate file.
#[cfg(test)]
#[path = "com_ring_test.rs"]
mod com_ring_test;

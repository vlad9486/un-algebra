//!
//! Algebraic _multiplicative_ _commutative_ _group_ traits.
//!
//! An algebraic _multiplicative_ _commutative_ _group_ is a
//! _multiplicative_ _group_ `S`, where group multiplication `*` is
//! required to be _commutative_.
//!
//! # Axioms
//!
//! 1. Group: multiplicative group axioms hold.
//! 2. Commutativity: ∀g, h ∈ S, g\*h = h\*g.
//!
//! # References
//!
//! See [references] for a formal definition of a multiplicative
//! commutative group.
//!
#![doc(include = "../doc/references.md")]

use types::*;
use group::mul_group::*;


///
/// An algebraic _multiplicative commutative group_.
///
pub trait MulComGroup: MulGroup {

  /// Test the axiom of commutivity.
  fn axiom_mul_commutivity(xs: Pair<Self>) -> bool {
    let (x, y) = xs;

    x.mul(y) == y.mul(x)
  }
}


///
/// A "numeric" algebraic _multiplicative commutative group_.
///
/// `NumMulComGroup` trait is for types that only form multiplicative
/// commutative groups when "numeric" comparisons are used, e.g.
/// floating point types.
///
pub trait NumMulComGroup: NumMulGroup {

  /// Numerically test the axiom of commutivity.
  fn axiom_mul_commutivity(xs: NumPair<Self>) -> bool {
     let (x, y, eps) = xs;

     x.mul(y).num_eq(&y.mul(x), eps)
   }
}


///
/// IEEE 64 bit floating point types only form a _numeric_
/// multiplicative commutative group.
///
impl NumMulComGroup for f32 {}


///
/// IEEE 64 bit floating point types only form a _numeric_
/// multiplicative commutative group.
///
impl NumMulComGroup for f64 {}


// Module unit tests are in a separate file.
#[cfg(test)]
#[path = "mul_com_group_test.rs"]
mod mul_com_group_test;



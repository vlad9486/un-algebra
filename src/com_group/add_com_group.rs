//!
//! Algebraic _additive_ _commutative_ _group_ traits.
//!
//! An algebraic _additive_ _commutative_ _group_ is an _additive_
//! _group_ `S`, where group addition `+` is required to be
//! _commutative_.
//!
//! # Axioms
//!
//! 1. Group: additive group axioms hold.
//! 2. Commutativity: ∀g, h ∈ S, g+h = h+g.
//!
//! # References
//!
//! See [references] for a formal definition of an additive
//! commutative group.
//!
#![doc(include = "../doc/references.md")]

use types::*;
use group::add_group::*;


///
/// An algebraic _additive commutative group_.
///
pub trait AddComGroup: AddGroup {

  /// Test the axiom of commutivity.
  fn axiom_add_commutivity(xs: Pair<Self>) -> bool {
    let (x, y) = xs;

    x.add(y) == y.add(x)
  }
}


///
/// A "numeric" algebraic _additive commutative group_.
///
/// `NumAddComGroup` trait is for types that only form additive
/// commutative groups when "numeric" comparisons are used, e.g.
/// floating point types.
///
pub trait NumAddComGroup: NumAddGroup {

 /// Numerically test the axiom of commutivity.
 fn axiom_add_commutivity(xs: NumPair<Self>) -> bool {
     let (x, y, eps) = xs;

     x.add(y).num_eq(&y.add(x), eps)
   }
}


///
/// Trait implementation macro for integer types.
///
/// A macro used to avoid writing repetitive, boilerplate
/// `AddComGroup` implementations for built-in signed integer types.
/// Probably not needed if Rust had a signed `Integer` super-trait.
///
macro_rules! integer_add_com_group {
  ($type:ty) => {
    impl AddComGroup for $type {}
  };

  ($type:ty, $($others:ty),+) => {
    integer_add_com_group! {$type}
    integer_add_com_group! {$($others),+}
  };
}


// Only signed integers form an additive commutative group.
integer_add_com_group! {
  i8, i16, i32, i64, i128, isize
}


///
/// IEEE 32 bit floating point types only form a _numeric_ additive
/// commutative group.
///
impl NumAddComGroup for f32 {}


///
/// IEEE 64 bit floating point types only form a _numeric_ additive
/// commutative group.
///
impl NumAddComGroup for f64 {}


// Module unit tests are in a separate file.
#[cfg(test)]
#[path = "add_com_group_test.rs"]
mod add_com_group_test;




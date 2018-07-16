//!
//! Algebraic _semigroup_ trait.
//!
//! An algebraic _semigroup_ is a _magma_ `S`, where the binary
//! operation `·` is _associative_.
//!
//! # Axioms
//!
//! 1. Magma: magma axioms hold.
//! 2. Associativity: ∀x, y, z ∈ S (x·y)·z = x·(y·z).
//!
//! # References
//!
//! See [references] for a formal definition of a semigroup.
//!
#![doc(include = "../doc/references.md")]

use types::*;
use magma::magma::*;


///
/// An algebraic _semigroup_.
///
pub trait Semigroup: Magma {

  /// Test the axiom of associativity.
  fn axiom_associativity(xs: Triple<Self>) -> bool {
    let (x, y, z) = xs;

    x.op(&y.op(z)) == x.op(y).op(z)
  }
}



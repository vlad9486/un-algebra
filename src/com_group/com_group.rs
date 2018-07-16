//!
//! Algebraic _commutative group_ trait.
//!
//! An algebraic _commutative group_ (also called an _abelian group_)
//! is a _group_ `S`, where the group operation `·` is required to be
//! _commutative_.
//! 
//! # Axioms
//!
//! 1. Group: group axioms hold.
//! 2. Commutativity: ∀g, h ∈ S, g·h = h·g.
//!
//! # References
//!
//! See [references] for a formal definition of a commutative group.
//!
#![doc(include = "../doc/references.md")]

use types::*;
use group::group::*;


///
/// An algebraic _commutative group_.
///
pub trait ComGroup: Group {

  /// Test the axiom of commutivity.
  fn axiom_commutivity(xs: Pair<Self>) -> bool {
    let (x, y) = xs;

    x.op(y) == y.op(x)
  }
}


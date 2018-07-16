//! Re-exports of `un-algebra` public modules.
//!
//! This module is intended to be wildcard-imported, i.e., `use
//! un_algebra::prelude::*;`.
//!

// Helper modules.
pub use types::*;
pub use logic::*;
pub use numeric::*;
pub use numeric::equal::*;


// Magmas.
pub use magma::*;
pub use magma::add_magma::*;
pub use magma::mul_magma::*;


// Semigroups.
pub use semigroup::*;
pub use semigroup::add_semigroup::*;
pub use semigroup::mul_semigroup::*;


// Monoids.
pub use monoid::*;
pub use monoid::add_monoid::*;
pub use monoid::mul_monoid::*;


//Groups.
pub use group::*;
pub use group::add_group::*;
pub use group::mul_group::*;


// Commutative groups.
pub use com_group::*;
pub use com_group::add_com_group::*;
pub use com_group::mul_com_group::*;


// Rings.
pub use ring::ring::*;


// Commutative rings.
pub use com_ring::com_ring::*;


// Fields.
pub use field::field::*;


// Other number types.
pub use complex::*;
pub use complex::complex::*;
pub use rational::*;
pub use rational::rational::*;


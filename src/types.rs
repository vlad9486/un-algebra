//!
//! Type aliases for algebraic structure axioms.
//!
//! Most of the `un_algebra` algebraic structure trait axioms have
//! tediously repetitive parameter lists. The type aliases provided
//! here are used to group parameters into more convenient tuples of
//! values.
//!
//! # Note
//!
//! Rust requires _explicit_ _lifetimes_ for tuples of generic
//! reference types.
//!
use numeric::equal::*;


/// A 2-tuple of `T` references.
pub type Pair<'a, T> = (&'a T, &'a T);


/// A 2-tuple of `T` references--augmented with a numeric comparison
/// error term.
pub type NumPair<'a, T> = (&'a T, &'a T, &'a <T as NumEq>::Error);


/// A 3-tuple of `T` references.
pub type Triple<'a, T> = (&'a T, &'a T, &'a T);


/// A 3-tuple of `T` references--augmented with a numeric comparison
/// error term.
pub type NumTriple<'a, T> = (&'a T, &'a T, &'a T, &'a <T as NumEq>::Error);

//!
//! Algebraic traits for _rational_ numbers.
//!
//! The Rust standard library does not have a _rational_ number type,
//! however the [num] crate provides a useful one. This module
//! implements `un_algebra` algebraic traits for `num`'s rational
//! number type.
//!
//! This module would be a useful example in this crate's examples
//! directory, but Rust's trait coherence restrictions make it much
//! simpler to include it in the crate source.
//!
#![doc(include = "../doc/references.md")]

pub mod rational;

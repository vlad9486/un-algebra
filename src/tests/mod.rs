//!
//! Generative testing support for axioms and properties.
//!
//! `un_algebra` modules that export abstract structure traits also
//! include a set of _generative_ ([proptest]) tests for each trait's
//! axioms and properties.
//!
//! The `tests` module provides random value generators and testing
//! configuration helper functions for `un_algebra` generative tests.
//!
#![doc(include = "../doc/references.md")]

pub mod random;

pub mod config;

pub mod prelude;


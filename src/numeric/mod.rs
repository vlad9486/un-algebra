//!
//! Numeric support for floating point types.
//!
//! Unfortunately, floating point numbers are only an approximation to
//! the infinitude of real numbers and they do not reliably implement
//! basic arithmetic properties like associativity or commutativity.
//!
//! The `numeric` module provides traits and helper functions for
//! working with floating point number types. This includes floating
//! point comparisons with an "epsilon" or error term.
//!
pub mod equal;

pub mod prelude;

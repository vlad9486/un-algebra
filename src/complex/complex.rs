//!
//! Algebraic trait implementations for _complex_ numbers.
//!
//! The complex numbers (&#x2102;) form a _field_. We assume the
//! _real_ and _imaginary_ parts--or _components_--of a complex number
//! are _real_ numbers (or some numeric approximation to &#x211d;,
//! e.g. floating point values).
//!
//! In the absence of a complex number type in the Rust standard
//! library, the complex number type used here is from the very handy
//! [num] crate.
//!
#![doc(include = "../doc/references.md")]

use prelude::*;
use std::ops::*;
pub use num::traits::*;
pub use num::complex::*;


///
/// Numeric equality for complex types.
///
impl<T: NumEq> NumEq for Complex<T> {

  /// The numeric error type is the component error type.
  type Error = T::Error;


  /// Equality is component-wise numeric equality.
  fn num_eq(&self, other: &Self, eps: &Self::Error) -> bool {
    let re = self.re.num_eq(&other.re, eps);
    let im = self.im.num_eq(&other.im, eps);

    re && im
  }
}


/// Shorthand type alias for real number complex components.
pub trait Real: Float + NumField + NumEq {}


/// Hack needed to use the `Real` type alias.
impl<T: Float + NumField + NumEq> Real for T {}


///
/// Complex numbers (with real components) form a numeric additive
/// magma.
///
impl<T: Real> NumAddMagma for Complex<T> {

  /// Addition is just complex addition.
  fn add(&self, other: &Self) -> Self {
    <Self as Add>::add(*self, *other)
  }
}


///
/// Complex numbers (with real components) form a numeric additive
/// semigroup.
///
impl<T: Real> NumAddSemigroup for Complex<T> {}


///
/// Complex numbers (with real components) form a numeric additive
/// monoid.
///
impl<T: Real> NumAddMonoid for Complex<T> {

  /// Zero is just complex _0 + 0i_.
  fn zero() -> Self {
    Self::new(Zero::zero(), Zero::zero())
  }
}


///
/// Complex numbers (with real components) form a numeric additive
/// group.
///
impl<T: Real> NumAddGroup for Complex<T> {

  /// Negation is just complex negation.
  fn negate(&self) -> Self {
    <Self as Neg>::neg(*self)
  }
}


///
/// Complex numbers (with real components) form a numeric additive
/// commutative group.
///
impl<T: Real> NumAddComGroup for Complex<T> {}


///
/// Complex numbers (with real components) form a numeric
/// multiplicative magma.
///
impl<T: Real> NumMulMagma for Complex<T> {

  /// Multiplication is just complex multiplication.
  fn mul(&self, other: &Self) -> Self {
    <Self as Mul>::mul(*self, *other)
  }
}


///
/// Complex numbers (with real components) form a numeric
/// multiplicative semigroup.
///
impl<T: Real> NumMulSemigroup for Complex<T> {}


///
/// Complex numbers (with real components) form a numeric
/// multiplicative monoid.
///
impl<T: Real> NumMulMonoid for Complex<T> {

  /// One is just complex _1 + 0i_.
  fn one() -> Self {
    Self::new(One::one(), Zero::zero())
  }
}


///
/// Non-zero complex numbers (with real components) form a
/// numeric multiplicative group.
///
impl<T: Real> NumMulGroup for Complex<T> {

  /// Inversion is just complex inversion.
  fn invert(&self) -> Self {
    self.inv()
  }


  /// Non-zero complex numbers are invertible.
  fn is_invertible(&self) -> bool {
    *self != Self::new(Zero::zero(), Zero::zero())
  }
}


///
/// Non-zero complex numbers (with real components) form a
/// numeric multiplicative commutative group.
///
impl<T: Real> NumMulComGroup for Complex<T> {}


///
/// Complex numbers (with real components) form a numeric ring.
///
impl<T: Real> NumRing for Complex<T> {}


///
/// Complex numbers (with real components) form a numeric commutative
/// ring.
///
impl<T: Real> NumComRing for Complex<T> {}


///
/// Non-zero Complex numbers (with real components) form a numeric
/// field.
///
impl<T: Real> NumField for Complex<T> {

  /// Inversion is just complex inversion.
  fn invert(&self) -> Self {
    self.inv()
  }

  
  /// Non-zero complex numbers are invertible.
  fn is_invertible(&self) -> bool {
    *self != Self::new(Zero::zero(), Zero::zero())
  }
}


// Module unit tests are in a separate file.
#[cfg(test)]
#[path = "complex_test.rs"]
mod complex_test;


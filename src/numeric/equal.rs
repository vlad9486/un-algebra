//!
//! Numeric equality of floating point values.
//!
//! Traits and implementations of _numeric_ equality--that is,
//! equality with an "epsilon" or error term--for floating point
//! types.
//!
use float_cmp::ApproxEq;


///
/// Numeric equality predicates.
///
pub trait NumEq: PartialEq {

  /// Epsilon type may be an independent type.
  type Error;


  /// Are two values numerically equal within `eps`?
  fn num_eq(&self, other: &Self, _eps: &Self::Error) -> bool {
    self == other
  }


  /// Are two values numerically unequal within `eps`?
  fn num_ne(&self, other: &Self, eps: &Self::Error) -> bool {
    !self.num_eq(other, eps)
  }


  /// Are two values numerically equal within default epsilon?
  fn eq(&self, other: &Self) -> bool {
    self == other
  }


  /// Are two values numerically unequal within default epsilon?
  fn ne(&self, other: &Self) -> bool {
    self != other
  } 
}



///
/// Trait implementation macro for floating point types.
///
/// A macro used to avoid writing repetitive, boilerplate `NumEq`
/// implementations for built-in floating point types. We rely on the
/// [`float_cmp`] crate to do the detailed comparisons using epsilon and
/// ULPS error terms.
///
/// [`float_cmp`]: https://crates.io/crates/float-cmp
///
macro_rules! float_num_eq {
  ($type:ty, $eps:expr, $ulps:expr) => {
    impl NumEq for $type {

      /// Error type is the same floating point type.
      type Error = Self;


      /// Equality within _eps_ and ULPS error bounds.
      fn num_eq(&self, other: &Self, eps: &$type) -> bool {
        self.approx_eq(other, *eps, $ulps)
      }


      /// Inequality within _eps_ and ULPS error bounds.
      fn num_ne(&self, other: &Self, eps: &$type) -> bool {
        self.approx_ne(other, *eps, $ulps)
      }


      /// Equality within default _eps_ and ULPS error bounds.
      fn eq(&self, other: &Self) -> bool {
        self.approx_eq(other, $eps, $ulps)
      }


      /// Inequality within default _eps_ and ULPS error bounds.
      fn ne(&self, other: &Self) -> bool {
        self.approx_ne(other, $eps, $ulps)
      }      
    }
  };
}


// 32 bit IEEE floating point equality (4 ULPS, 1e-6 epsilon).
float_num_eq! {f32, 1e-6, 4}


// 64 bit IEEE floating point equality (4 ULPS, 1e-14 epsilon).
float_num_eq! {f64, 1e-14, 4}






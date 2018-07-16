//!
//! Random value generation for generative tests.
//!
//! `un_algebra` generative tests (using `proptest`) rely on randomly
//! generated values of Rust's built in numeric types. The `random`
//! module provides convenient `proptest` generators for one or
//! several built-in integer and floating point types.
//!
//! This module has a lot of repetitive, boilerplate code, but it's
//! not clear to me that this is worse than the self-referential
//! macros that would be needed to generate the code automatically.
//!
use proptest::prelude::*;


/// Default testing epsilon for 32 bit FP comparisons.
pub const F32_EPS: f32 = 1e-6;


/// Default testing epsilon for 64 bit FP comparisons.
pub const F64_EPS: f64 = 1e-14;


/// Handy type alias for a 2-tuple of T values.
pub type Two<T> = (T, T);


/// Handy type alias for a 3-tuple of T values.
pub type Three<T> = (T, T, T);


prop_compose! {

  /// Generate one random `u8` value.
  [pub] fn u8_1()(x in any::<u8>()) -> u8 {
    x
  }
}


prop_compose! {

  /// Generate two random `u8` values.
  [pub] fn u8_2()(xs in any::<Two<u8>>()) -> Two<u8> {
    xs
  }
}


prop_compose! {

  /// Generate three random `u8` values.
  [pub] fn u8_3()(xs in any::<Three<u8>>()) -> Three<u8> {
    xs
  }
}


prop_compose! {

  /// Generate one random `u16` value.
  [pub] fn u16_1()(x in any::<u16>()) -> u16 {
    x
  }
}


prop_compose! {

  /// Generate two random `u16` values.
  [pub] fn u16_2()(xs in any::<Two<u16>>()) -> Two<u16> {
    xs
  }
}


prop_compose! {

  /// Generate three random `u16` values.
  [pub] fn u16_3()(xs in any::<Three<u16>>()) -> Three<u16> {
    xs
  }
}


prop_compose! {

  /// Generate one random `u32` value.
  [pub] fn u32_1()(x in any::<u32>()) -> u32 {
    x
  }
}


prop_compose! {

  /// Generate two random `u32` values.
  [pub] fn u32_2()(xs in any::<Two<u32>>()) -> Two<u32> {
    xs
  }
}


prop_compose! {

  /// Generate three random `u32` values.
  [pub] fn u32_3()(xs in any::<Three<u32>>()) -> Three<u32> {
    xs
  }
}


prop_compose! {

  /// Generate one random `u64` value.
  [pub] fn u64_1()(x in any::<u64>()) -> u64 {
    x
  }
}


prop_compose! {

  /// Generate two random `u64` values.
  [pub] fn u64_2()(xs in any::<Two<u64>>()) -> Two<u64> {
    xs
  }
}


prop_compose! {

  /// Generate three random `u64` values.
  [pub] fn u64_3()(xs in any::<Three<u64>>()) -> Three<u64> {
    xs
  }
}


prop_compose! {

  /// Generate one random `usize` value.
  [pub] fn usize_1()(x in any::<usize>()) -> usize {
    x
  }
}


prop_compose! {

  /// Generate two random `usize` values.
  [pub] fn usize_2()(xs in any::<Two<usize>>()) -> Two<usize> {
    xs
  }
}


prop_compose! {

  /// Generate three random `usize` values.
  [pub] fn usize_3()(xs in any::<Three<usize>>()) -> Three<usize> {
    xs
  }
}


prop_compose! {

  /// Generate one random `i8` value.
  [pub] fn i8_1()(x in any::<i8>()) -> i8 {
    x
  }
}


prop_compose! {

  /// Generate two random `i8` values.
  [pub] fn i8_2()(xs in any::<Two<i8>>()) -> Two<i8> {
    xs
  }
}


prop_compose! {

  /// Generate three random `i8` values.
  [pub] fn i8_3()(xs in any::<Three<i8>>()) -> Three<i8> {
    xs
  }
}


prop_compose! {

  /// Generate one random `i16` value.
  [pub] fn i16_1()(x in any::<i16>()) -> i16 {
    x
  }
}


prop_compose! {

  /// Generate two random `i16` values.
  [pub] fn i16_2()(xs in any::<Two<i16>>()) -> Two<i16> {
    xs
  }
}


prop_compose! {

  /// Generate three random `i16` values.
  [pub] fn i16_3()(xs in any::<Three<i16>>()) -> Three<i16> {
    xs
  }
}


prop_compose! {

  /// Generate one random `i32` value.
  [pub] fn i32_1()(x in any::<i32>()) -> i32 {
    x
  }
}


prop_compose! {

  /// Generate two random `i32` values.
  [pub] fn i32_2()(xs in any::<Two<i32>>()) -> Two<i32> {
    xs
  }
}


prop_compose! {

  /// Generate three random `i32` values.
  [pub] fn i32_3()(xs in any::<Three<i32>>()) -> Three<i32> {
    xs
  }
}


prop_compose! {

  /// Generate one random `i64` value.
  [pub] fn i64_1()(x in any::<i64>()) -> i64 {
    x
  }
}


prop_compose! {

  /// Generate two random `i64` values.
  [pub] fn i64_2()(xs in any::<Two<i64>>()) -> Two<i64> {
    xs
  }
}


prop_compose! {

  /// Generate three random `i64` values.
  [pub] fn i64_3()(xs in any::<Three<i64>>()) -> Three<i64> {
    xs
  }
}


prop_compose! {

  /// Generate one random `isize` value.
  [pub] fn isize_1()(x in any::<isize>()) -> isize {
    x
  }
}


prop_compose! {

  /// Generate two random `isize` values.
  [pub] fn isize_2()(xs in any::<Two<isize>>()) -> Two<isize> {
    xs
  }
}


prop_compose! {

  /// Generate three random `isize` values.
  [pub] fn isize_3()(xs in any::<Three<isize>>()) -> Three<isize> {
    xs
  }
}


prop_compose! {

  /// Generate one random `f32` value.
  [pub] fn f32_1()(x in any::<f32>()) -> f32 {
    x
  }
}


prop_compose! {

  /// Generate two random `f32` values.
  [pub] fn f32_2()(xs in any::<Two<f32>>()) -> Two<f32> {
    xs
  }
}


prop_compose! {

  /// Generate three random `f32` values.
  [pub] fn f32_3()(xs in any::<Three<f32>>()) -> Three<f32> {
    xs
  }
}


prop_compose! {

  /// Generate one random `f64` value.
  [pub] fn f64_1()(x in any::<f64>()) -> f64 {
    x
  }
}


prop_compose! {

  /// Generate two random `f64` values.
  [pub] fn f64_2()(xs in any::<Two<f64>>()) -> Two<f64> {
    xs
  }
}


prop_compose! {

  /// Generate three random `f64` values.
  [pub] fn f64_3()(xs in any::<Three<f64>>()) -> Three<f64> {
    xs
  }
}

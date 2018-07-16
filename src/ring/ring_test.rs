use ring::ring::*;
use tests::prelude::*;


proptest! {
  #![proptest_config(standard())]


  #[test]
  fn axiom_left_distributivity_i32((ref x, ref y, ref z) in i32_3()) {
    Ring::axiom_left_distributivity((x, y, z))
  }


  #[test]
  fn axiom_left_distributivity_f64((ref x, ref y, ref z) in f64_3()) {
    NumRing::axiom_left_distributivity((x, y, z, &F64_EPS))
  }


  #[test]
  fn axiom_right_distributivity_i16((ref x, ref y, ref z) in i16_3()) {
    Ring::axiom_right_distributivity((x, y, z))
  }


  #[test]
  fn axiom_right_distributivity_f32((ref x, ref y, ref z) in f32_3()) {
    NumRing::axiom_right_distributivity((x, y, z, &F32_EPS))
  }


  #[test]
  fn prop_left_zero_absorb_isize(ref x in isize_1()) {
    Ring::prop_left_zero_absorb(x)
  }


  #[test]
  fn prop_left_zero_absorb_f32(ref x in f32_1()) {
    NumRing::prop_left_zero_absorb(x, &F32_EPS)
  }


  #[test]
  fn prop_right_zero_absorb_i32(ref x in i32_1()) {
    Ring::prop_right_zero_absorb(x)
  }


  #[test]
  fn prop_right_zero_absorb_f64(ref x in f64_1()) {
    NumRing::prop_right_zero_absorb(x, &F64_EPS)
  }


  #[test]
  fn prop_left_mul_negate_i16((ref x, ref y) in i16_2()) {
    Ring::prop_left_mul_negate((x, y))
  }


  #[test]
  fn prop_left_mul_negate_f32((ref x, ref y) in f32_2()) {
    NumRing::prop_left_mul_negate((x, y, &F32_EPS))
  }


  #[test]
  fn prop_right_mul_negate_i32((ref x, ref y) in i32_2()) {
    Ring::prop_right_mul_negate((x, y))
  }


  #[test]
  fn prop_right_mul_negate_f64((ref x, ref y) in f64_2()) {
    NumRing::prop_right_mul_negate((x, y, &F64_EPS))
  }


  #[test]
  fn prop_mul_negate_i64((ref x, ref y) in i64_2()) {
    Ring::prop_mul_negate((x, y))
  }


  #[test]
  fn prop_mul_negate_f32((ref x, ref y) in f32_2()) {
    NumRing::prop_mul_negate((x, y, &F32_EPS))
  }


  #[test]
  fn prop_left_one_negate_isize(ref x in isize_1()) {
    Ring::prop_left_one_negate(x)
  }


  #[test]
  fn prop_left_one_negate_f32(ref x in f32_1()) {
    NumRing::prop_left_one_negate(x, &F32_EPS)
  }


  #[test]
  fn prop_right_one_negate_i32(ref x in i32_1()) {
    Ring::prop_right_one_negate(x)
  }


  #[test]
  fn prop_right_one_negate_f64(ref x in f64_1()) {
    NumRing::prop_right_one_negate(x, &F64_EPS)
  }
}

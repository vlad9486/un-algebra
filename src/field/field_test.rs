use field::field::*;
use tests::prelude::*;


proptest! {
  #![proptest_config(standard())]


  #[test]
  fn axiom_left_invert_f32(ref x in f32_1()) {
    prop_assume!(x.is_invertible());

    NumField::axiom_left_invert(x, &F32_EPS)
  }


  #[test]
  fn axiom_right_invert_f64(ref x in f64_1()) {
    prop_assume!(x.is_invertible());

    NumField::axiom_right_invert(x, &F64_EPS)
  }


  #[test]
  fn prop_mul_zero_f32((ref x, ref y) in f32_2()) {
    NumField::prop_mul_zero((x, y, &F32_EPS))
  }


  #[test]
  fn prop_add_cancel_f64((ref x, ref y, ref z) in f64_3()) {
    NumField::prop_add_cancel((x, y, z, &F64_EPS))
  }


  #[test]
  fn prop_mul_cancel_f32((ref x, ref y, ref z) in f32_3()) {
    NumField::prop_mul_cancel((x, y, z, &F32_EPS))
  }
}

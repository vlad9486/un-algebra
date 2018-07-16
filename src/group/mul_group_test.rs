use tests::prelude::*;
use group::mul_group::*;


proptest! {
  #![proptest_config(standard())]


  #[test]
  fn axiom_left_invert_f32(ref x in f32_1()) {
    prop_assume!(x.is_invertible());
      
    NumMulGroup::axiom_left_invert(x, &F32_EPS)
  }


  #[test]
  fn axiom_right_invert_f64(ref x in f64_1()) {
    prop_assume!(x.is_invertible());

    NumMulGroup::axiom_right_invert(x, &F64_EPS)
  }
}

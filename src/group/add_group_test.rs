use tests::prelude::*;
use group::add_group::*;


proptest! {
  #![proptest_config(standard())]


  #[test]
  fn axiom_left_negate_i32(ref x in i32_1()) {
    AddGroup::axiom_left_negate(x)
  }


  #[test]
  fn axiom_left_negate_f64(ref x in f64_1()) {
    NumAddGroup::axiom_left_negate(x, &F64_EPS)
  }


  #[test]
  fn axiom_right_negate_i16(ref x in i16_1()) {
    AddGroup::axiom_right_negate(x)
  }


  #[test]
  fn axiom_right_negate_f32(ref x in f32_1()) {
    NumAddGroup::axiom_right_negate(x, &F32_EPS)
  }
}

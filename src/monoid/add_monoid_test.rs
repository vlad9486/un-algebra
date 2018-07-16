use tests::prelude::*;
use monoid::add_monoid::*;


proptest! {
  #![proptest_config(standard())]


  #[test]
  fn axiom_left_add_identity_u8(ref x in u8_1()) {
    AddMonoid::axiom_left_add_identity(x)
  }


  #[test]
  fn axiom_left_add_identity_i64(ref x in i64_1()) {
    AddMonoid::axiom_left_add_identity(x)
  }


  #[test]
  fn axiom_left_add_identity_f32(ref x in f32_1()) {
    NumAddMonoid::axiom_left_add_identity(x, &F32_EPS)
  }


  #[test]
  fn axiom_right_add_identity_u32(ref x in u32_1()) {
    AddMonoid::axiom_right_add_identity(x)
  }


  #[test]
  fn axiom_right_add_identity_isize(ref x in isize_1()) {
    AddMonoid::axiom_right_add_identity(x)
  }


  #[test]
  fn axiom_right_add_identity_f64(ref x in f64_1()) {
    NumAddMonoid::axiom_right_add_identity(x, &F64_EPS)
  }
}

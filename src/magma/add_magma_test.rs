use tests::prelude::*;
use magma::add_magma::*;


proptest! {
  #![proptest_config(standard())]


  #[test]
  fn axiom_add_closure_u16((ref x, ref y) in u16_2()) {
    AddMagma::axiom_add_closure((x, y))
  }


  #[test]
  fn axiom_add_closure_i64((ref x, ref y) in i64_2()) {
    AddMagma::axiom_add_closure((x, y))
  }


  #[test]
  fn axiom_add_closure_f32((ref x, ref y) in f32_2()) {
    NumAddMagma::axiom_add_closure((x, y, &F32_EPS))
  }
}

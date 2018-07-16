use tests::prelude::*;
use com_group::add_com_group::*;


proptest! {
  #![proptest_config(standard())]


  #[test]
  fn axiom_add_commutivity_i32((ref x, ref y) in i32_2()) {
    AddComGroup::axiom_add_commutivity((x, y))
  }


  #[test]
  fn axiom_add_commutivity_f64((ref w, ref z) in f64_2()) {
    NumAddComGroup::axiom_add_commutivity((w, z, &F64_EPS))
  }
}

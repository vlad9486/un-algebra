use tests::prelude::*;
use com_group::mul_com_group::*;


proptest! {
  #![proptest_config(standard())]


  #[test]
  fn axiom_mul_commutivity_f32((ref w, ref z) in f32_2()) {
    NumMulComGroup::axiom_mul_commutivity((w, z, &F32_EPS))
  }
}

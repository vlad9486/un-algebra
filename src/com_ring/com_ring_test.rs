use tests::prelude::*;
use com_ring::com_ring::*;


proptest! {
  #![proptest_config(standard())]


  #[test]
  fn axiom_mul_commutivity_i8((ref x, ref y) in i8_2()) {
    ComRing::axiom_mul_commutivity((x, y))
  }


  #[test]
  fn axiom_mul_commutivity_f32((ref x, ref y) in f32_2()) {
    NumComRing::axiom_mul_commutivity((x, y, &F32_EPS))
  }
}

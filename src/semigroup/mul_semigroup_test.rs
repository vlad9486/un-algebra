use tests::prelude::*;
use semigroup::mul_semigroup::*;


proptest! {
  #![proptest_config(standard())]


  #[test]
  fn axiom_mul_associativity_u16((ref x, ref y, ref z) in u16_3()) {
    MulSemigroup::axiom_mul_associativity((x, y, z))
  }


  #[test]
  fn axiom_mul_associativity_isize((ref x, ref y, ref z) in isize_3()) {
    MulSemigroup::axiom_mul_associativity((x, y, z))
  }


  #[test]
  fn axiom_mul_associativity_f32((ref x, ref y, ref z) in f32_3()) {
    NumMulSemigroup::axiom_mul_associativity((x, y, z, &F32_EPS))
  }
}

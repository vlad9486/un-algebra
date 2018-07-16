use tests::prelude::*;
use semigroup::add_semigroup::*;


proptest! {
  #![proptest_config(standard())]


  #[test]
  fn axiom_add_associativity_u32((ref x, ref y, ref z) in u32_3()) {
    AddSemigroup::axiom_add_associativity((x, y, z))
  }


  #[test]
  fn axiom_add_associativity_i8((ref x, ref y, ref z) in i8_3()) {
    AddSemigroup::axiom_add_associativity((x, y, z))
  }


  #[test]
  fn axiom_add_associativity_f64((ref x, ref y, ref z) in f64_3()) {
    NumAddSemigroup::axiom_add_associativity((x, y, z, &F64_EPS))
  }
}

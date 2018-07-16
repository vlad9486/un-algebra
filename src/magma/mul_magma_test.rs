use tests::prelude::*;
use magma::mul_magma::*;


proptest! {
  #![proptest_config(standard())]


  #[test]
  fn axiom_mul_closure_u32((ref x, ref y) in u32_2()) {
    MulMagma::axiom_mul_closure((x, y))
  }


  #[test]
  fn axiom_mul_closure_i8((ref x, ref y) in i8_2()) {
    MulMagma::axiom_mul_closure((x, y))
  }


  #[test]
  fn axiom_mul_closure_f64((ref x, ref y) in f64_2()) {
    NumMulMagma::axiom_mul_closure((x, y, &F64_EPS))
  }
}

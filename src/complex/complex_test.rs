use prelude::*;
use num::complex::*;
use tests::prelude::*;


prop_compose! {

  /// Generate a random complex number with f32 components.
  fn c32_1()((x, y) in f32_2()) -> Complex<f32> {
    Complex::new(x, y)
  }
}


prop_compose! {

  /// Generate two random complex numbers with f32 components.
  fn c32_2()(xs in (c32_1(), c32_1())) -> Two<Complex<f32>> {
    xs
  }
}


prop_compose! {

  /// Generate three random complex numbers with f32 components.
  fn c32_3()(xs in (c32_1(), c32_1(), c32_1())) -> Three<Complex<f32>> {
    xs
  }
}


prop_compose! {

  /// Generate one random complex number with f64 components.
  fn c64_1()((x, y) in f64_2()) -> Complex<f64> {
    Complex::new(x, y)
  }
}


prop_compose! {

  /// Generate two random complex numbers with f64 components.
  fn c64_2()(xs in (c64_1(), c64_1())) -> Two<Complex<f64>> {
    xs
  }
}


prop_compose! {

  /// Generate three random complex numbers with f64 components.
  fn c64_3()(xs in (c64_1(), c64_1(), c64_1())) -> Three<Complex<f64>> {
    xs
  }
}


proptest! {
  #![proptest_config(standard())]


  #[test]
  fn axiom_add_closure_c64((ref w, ref z) in c64_2()) {
    NumAddMagma::axiom_add_closure((w, z, &F64_EPS))
  }


  #[test]
  fn axiom_mul_closure_c64((ref w, ref z) in c64_2()) {
    NumMulMagma::axiom_mul_closure((w, z, &F64_EPS))
  }


  #[test]
  fn axiom_mul_associativity_c64((ref u, ref w, ref z) in c64_3()) {
    NumMulSemigroup::axiom_mul_associativity((u, w, z, &F64_EPS))
  }


  #[test]
  fn axiom_add_associativity_c32((ref u, ref w, ref z) in c32_3()) {
    NumAddSemigroup::axiom_add_associativity((u, w, z, &F32_EPS))
  }


  #[test]
  fn axiom_left_add_identity_c64(ref z in c64_1()) {
    NumAddMonoid::axiom_left_add_identity(z, &F64_EPS)
  }


  #[test]
  fn axiom_right_add_identity_c32(ref z in c32_1()) {
    NumAddMonoid::axiom_left_add_identity(z, &F32_EPS)
  }


  #[test]
  fn axiom_left_mul_identity_c64(ref z in c64_1()) {
    NumMulMonoid::axiom_left_mul_identity(z, &F64_EPS)
  }


  #[test]
  fn axiom_right_mul_identity_c32(ref z in c32_1()) {
    NumMulMonoid::axiom_right_mul_identity(z, &F32_EPS)
  }


  #[test]
  fn axiom_left_negate_c32(ref z in c32_1()) {
    NumAddGroup::axiom_left_negate(z, &F32_EPS)
  }


  #[test]
  fn axiom_right_negate_c64(ref z in c64_1()) {
    NumAddGroup::axiom_right_negate(z, &F64_EPS)
  }


  #[test]
  fn axiom_left_invert_c64(ref z in c64_1()) {
    prop_assume!(NumMulGroup::is_invertible(z));

    NumMulGroup::axiom_left_invert(z, &F64_EPS)
  }


  #[test]
  fn axiom_right_invert_c32(ref z in c32_1()) {
    prop_assume!(NumMulGroup::is_invertible(z));

    NumMulGroup::axiom_right_invert(z, &F32_EPS)
  }


  #[test]
  fn axiom_add_commutivity_c32((ref w, ref z) in c32_2()) {
    NumAddComGroup::axiom_add_commutivity((w, z, &F32_EPS))
  }


  #[test]
  fn axiom_mul_commutivity_c64((ref w, ref z) in c64_2()) {
    NumMulComGroup::axiom_mul_commutivity((w, z, &F64_EPS))
  }


  #[test]
  fn axiom_left_distributivity_c32((ref x, ref y, ref z) in c32_3()) {
    NumRing::axiom_left_distributivity((x, y, z, &F32_EPS))
  }


  #[test]
  fn axiom_right_distributivity_c64((ref x, ref y, ref z) in c64_3()) {
    NumRing::axiom_right_distributivity((x, y, z, &F64_EPS))
  }


  #[test]
  fn prop_left_zero_absorb_c64(ref x in c64_1()) {
    NumRing::prop_left_zero_absorb(x, &F64_EPS)
  }


  #[test]
  fn prop_right_zero_absorb_c32(ref x in c32_1()) {
    NumRing::prop_right_zero_absorb(x, &F32_EPS)
  }


  #[test]
  fn prop_left_mul_negate_c64((ref x, ref y) in c64_2()) {
    NumRing::prop_left_mul_negate((x, y, &F64_EPS))
  }


  #[test]
  fn prop_right_mul_negate_c32((ref x, ref y) in c32_2()) {
    NumRing::prop_right_mul_negate((x, y, &F32_EPS))
  }


  #[test]
  fn prop_mul_negate_c64((ref x, ref y) in c64_2()) {
    NumRing::prop_mul_negate((x, y, &F64_EPS))
  }


  #[test]
  fn prop_left_one_negate_c64(ref x in c64_1()) {
    NumRing::prop_left_one_negate(x, &F64_EPS)
  }


  #[test]
  fn prop_right_one_negate_c32(ref x in c32_1()) {
    NumRing::prop_right_one_negate(x, &F32_EPS)
  }


  #[test]
  fn axiom_ring_mul_commutivity_c64((ref x, ref y) in c64_2()) {
    NumComRing::axiom_mul_commutivity((x, y, &F64_EPS))
  }


  #[test]
  fn axiom_field_left_invert_c64(ref z in c64_1()) {
    prop_assume!(NumField::is_invertible(z));

    NumField::axiom_left_invert(z, &F64_EPS)
  }


  #[test]
  fn axiom_field_right_invert_c32(ref z in c32_1()) {
    prop_assume!(NumField::is_invertible(z));

    NumField::axiom_left_invert(z, &F32_EPS)
  }


  #[test]
  fn prop_mul_zero_c64((ref w, ref z) in c64_2()) {
    NumField::prop_mul_zero((w, z, &F64_EPS))
  }


  #[test]
  fn prop_add_cancel_c32((ref v, ref w, ref z) in c32_3()) {
    NumField::prop_add_cancel((v, w, z, &F32_EPS))
  }


  #[test]
  fn prop_mul_cancel_c64((ref v, ref w, ref z) in c64_3()) {
    NumField::prop_mul_cancel((v, w, z, &F64_EPS))
  }
}

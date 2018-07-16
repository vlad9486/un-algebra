use prelude::*;
use num::bigint::*;
use num::rational::*;
use tests::prelude::*;


prop_compose! {

  // Generate one BigRational value from two BigInt values.
  fn rbig_1()((n, d) in i64_2()) -> BigRational {
    let bn = n.to_bigint().unwrap();
    let bd = d.to_bigint().unwrap();
        
    BigRational::new(bn, bd)
  }
}


prop_compose! {
  
  // Generate two BigRational values.
  fn rbig_2()(qs in (rbig_1(), rbig_1())) -> Two<BigRational> {
    qs
  }
}


prop_compose! {
  
  // Generate three BigRational values.
  fn rbig_3()(qs in (rbig_1(), rbig_1(), rbig_1())) -> Three<BigRational> {
    qs
  }
}


proptest! {

  // Only 100 samples as these tests are way too slow.
  #![proptest_config(config_with(100, 50))]


  #[test]
  fn axiom_add_closure((ref p, ref q) in rbig_2()) {
    AddMagma::axiom_add_closure((p, q))
  }


  #[test]
  fn axiom_mul_closure((ref p, ref q) in rbig_2()) {
    MulMagma::axiom_mul_closure((p, q))
  }


  #[test]
  fn axiom_mul_associativity((ref p, ref q, ref r) in rbig_3()) {
    MulSemigroup::axiom_mul_associativity((p, q, r))
  }


  #[test]
  fn axiom_add_associativity((ref p, ref q, ref r) in rbig_3()) {
    AddSemigroup::axiom_add_associativity((p, q, r))
  }


  #[test]
  fn axiom_left_add_identity(ref q in rbig_1()) {
    AddMonoid::axiom_left_add_identity(q)
  }


  #[test]
  fn axiom_right_add_identity(ref q in rbig_1()) {
    AddMonoid::axiom_left_add_identity(q)
  }


  #[test]
  fn axiom_left_mul_identity(ref q in rbig_1()) {
    MulMonoid::axiom_left_mul_identity(q)
  }


  #[test]
  fn axiom_right_mul_identity(ref q in rbig_1()) {
    MulMonoid::axiom_right_mul_identity(q)
  }


  #[test]
  fn axiom_left_negate(ref q in rbig_1()) {
    AddGroup::axiom_left_negate(q)
  }


  #[test]
  fn axiom_right_negate(ref q in rbig_1()) {
    AddGroup::axiom_right_negate(q)
  }


  #[test]
  fn axiom_left_invert(ref q in rbig_1()) {
    prop_assume!(MulGroup::is_invertible(q));

    MulGroup::axiom_left_invert(q)
  }


  #[test]
  fn axiom_right_invert(ref q in rbig_1()) {
    prop_assume!(MulGroup::is_invertible(q));

    MulGroup::axiom_right_invert(q)
  }


  #[test]
  fn axiom_add_commutivity((ref p, ref q) in rbig_2()) {
    AddComGroup::axiom_add_commutivity((p, q))
  }


  #[test]
  fn axiom_mul_commutivity((ref p, ref q) in rbig_2()) {
    MulComGroup::axiom_mul_commutivity((p, q))
  }


  #[test]
  fn axiom_left_distributivity((ref q, ref r, ref s) in rbig_3()) {
    Ring::axiom_left_distributivity((q, r, s))
  }


  #[test]
  fn axiom_right_distributivity((ref q, ref r, ref s) in rbig_3()) {
    Ring::axiom_right_distributivity((q, r, s))
  }


  #[test]
  fn prop_left_zero_absorb(ref q in rbig_1()) {
    Ring::prop_left_zero_absorb(q)
  }


  #[test]
  fn prop_right_zero_absorb(ref q in rbig_1()) {
    Ring::prop_right_zero_absorb(q)
  }


  #[test]
  fn prop_left_mul_negate((ref q, ref r) in rbig_2()) {
    Ring::prop_left_mul_negate((q, r))
  }


  #[test]
  fn prop_right_mul_negate((ref q, ref r) in rbig_2()) {
    Ring::prop_right_mul_negate((q, r))
  }


  #[test]
  fn prop_mul_negate((ref q, ref r) in rbig_2()) {
    Ring::prop_mul_negate((q, r))
  }


  #[test]
  fn prop_left_one_negate(ref q in rbig_1()) {
    Ring::prop_left_one_negate(q)
  }


  #[test]
  fn prop_right_one_negate(ref q in rbig_1()) {
    Ring::prop_right_one_negate(q)
  }


  #[test]
  fn axiom_ring_mul_commutivity((ref q, ref r) in rbig_2()) {
    ComRing::axiom_mul_commutivity((q, r))
  }


  #[test]
  fn axiom_field_left_invert(ref q in rbig_1()) {
    prop_assume!(Field::is_invertible(q));

    Field::axiom_left_invert(q)
  }


  #[test]
  fn axiom_field_right_invert(ref q in rbig_1()) {
    prop_assume!(Field::is_invertible(q));

    Field::axiom_left_invert(q)
  }


  #[test]
  fn prop_mul_zero((ref p, ref q) in rbig_2()) {
    Field::prop_mul_zero((p, q))
  }


  #[test]
  fn prop_add_cancel((ref q, ref r, ref s) in rbig_3()) {
    Field::prop_add_cancel((q, r, s))
  }


  #[test]
  fn prop_mul_cancel((ref q, ref r, ref s) in rbig_3()) {
    Field::prop_mul_cancel((q, r, s))
  }
}

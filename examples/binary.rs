//
// # Example: The binary field _F2_.
//
// The _binary_ values {`F`,`T`} form a _finite field_ with binary XOR
// as "addition", binary AND as "multiplication", and binary NOT as
// "negation". This field is known as _F2_ or _GF(2)_ [GF2].
//
// This means the familiar digital logic or computing boolean type
// effectively forms a (finite) field.
//
// [GF2]: https://en.wikipedia.org/wiki/Field_(mathematics)


// Use the un_algebra traits.
extern crate un_algebra;
use un_algebra::prelude::*;


// Use proptest for generative testing.
#[macro_use]
extern crate proptest;


//
// The binary field F2.
//
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum F2 {
  F, T
}


//
// F2 values form an additive magma.
//
impl AddMagma for F2 {

  // F2 addition is binary XOR.
  fn add(&self, other: &Self) -> Self {
    match (self, other) {
      (F2::F, F2::F) => F2::F,
      (F2::F, F2::T) => F2::T,
      (F2::T, F2::F) => F2::T,
      (F2::T, F2::T) => F2::F,
    }    
  }
}


//
// F2 values form an additive semigroup.
//
impl AddSemigroup for F2 {}


//
// F2 values form an additive monoid.
//
impl AddMonoid for F2 {

  // F2 FALSE acts as zero.
  fn zero() -> Self {
    F2::F
  }
}


//
// F2 values form an additive group.
//
impl AddGroup for F2 {

  // F2 negation is binary NOT.
  fn negate(&self) -> Self {
    match self {
      F2::F => F2::T,
      F2::T => F2::F,
    }
  }
}


//
// F2 values form an additive commutative group.
//
impl AddComGroup for F2 {}


//
// F2 values form a multiplicative magma.
//
impl MulMagma for F2 {

  // F2 multiplication is binary AND.
  fn mul(&self, other: &Self) -> Self {
    match (self, other) {
      (F2::F, F2::F) => F2::F,
      (F2::F, F2::T) => F2::F,
      (F2::T, F2::F) => F2::F,
      (F2::T, F2::T) => F2::T,
    }    
  }
}


//
// F2 values form a multiplicative semigroup.
//
impl MulSemigroup for F2 {}


//
// F2 values form a multiplicative monoid.
//
impl MulMonoid for F2 {

  // One is binary true.
  fn one() -> Self {
    F2::T
  }
}


//
// F2 values form a multiplicative group.
//
impl MulGroup for F2 {

  // Invert is binary NOT.
  fn invert(&self) -> Self {
    match self {
      F2::F => F2::T,
      F2::T => F2::F,
    }
  }


  // All binary elements are invertible.
  fn is_invertible(&self) -> bool {
    true
  }  
}


//
// F2 values form a multiplicative commutative group.
//
impl MulComGroup for F2 {}


//
// F2 values form a ring.
//
impl Ring for F2 {}


//
// F2 values form a commutative ring.
//
impl ComRing for F2 {}


//
// F2 values form a field.
//
impl Field for F2 {

  // Invert is binary NOT.
  fn invert(&self) -> Self {
    match self {
      F2::F => F2::T,
      F2::T => F2::F,
    }
  }


  // All boolean elements are invertible.
  fn is_invertible(&self) -> bool {
    true
  }    
}


prop_compose! {
  // Generate 1 random boolean test value.
  fn bln_1()(i in 0..1) -> F2 {
    ([F2::F, F2::T][i as usize]).clone()
  }
}


prop_compose! {
  // Generate 2 random boolean test values.
  fn bln_2()(xs in (bln_1(), bln_1())) -> (F2, F2) {
    xs
  }
}


prop_compose! {
  // Generate 3 random boolean test values.
  fn bln_3()(xs in (bln_1(), bln_1(), bln_1())) -> (F2, F2, F2) {
    xs
  }
}


// Generative tests for F4 algebraic axioms and properties.
proptest! {

  #[test]
  fn axiom_add_closure((ref x, ref y) in bln_2()) {
    AddMagma::axiom_add_closure((x, y))
  }


  #[test]
  fn axiom_mul_closure((ref x, ref y) in bln_2()) {
    MulMagma::axiom_mul_closure((x, y))
  }


  #[test]
  fn axiom_mul_associativity((ref w, ref x, ref y) in bln_3()) {
    MulSemigroup::axiom_mul_associativity((w, x, y))
  }


  #[test]
  fn axiom_add_associativity((ref w, ref x, ref y) in bln_3()) {
    AddSemigroup::axiom_add_associativity((w, x, y))
  }


  #[test]
  fn axiom_left_add_identity(ref x in bln_1()) {
    AddMonoid::axiom_left_add_identity(x)
  }


  #[test]
  fn axiom_right_add_identity(ref x in bln_1()) {
    AddMonoid::axiom_left_add_identity(x)
  }


  #[test]
  fn axiom_left_mul_identity(ref x in bln_1()) {
    MulMonoid::axiom_left_mul_identity(x)
  }


  #[test]
  fn axiom_right_mul_identity(ref x in bln_1()) {
    MulMonoid::axiom_right_mul_identity(x)
  }


  #[test]
  fn axiom_left_negate(ref x in bln_1()) {
    AddGroup::axiom_left_negate(x)
  }


  #[test]
  fn axiom_right_negate(ref x in bln_1()) {
    AddGroup::axiom_right_negate(x)
  }


  #[test]
  fn axiom_left_invert(ref x in bln_1()) {
    MulGroup::axiom_left_invert(x)
  }


  #[test]
  fn axiom_right_invert(ref x in bln_1()) {
    MulGroup::axiom_right_invert(x)
  }


  #[test]
  fn axiom_add_commutivity((ref x, ref y) in bln_2()) {
    AddComGroup::axiom_add_commutivity((x, y))
  }


  #[test]
  fn axiom_mul_commutivity((ref x, ref y) in bln_2()) {
    MulComGroup::axiom_mul_commutivity((x, y))
  }


  #[test]
  fn axiom_left_distributivity((ref x, ref y, ref z) in bln_3()) {
    Ring::axiom_left_distributivity((x, y, z))
  }


  #[test]
  fn axiom_right_distributivity((ref x, ref y, ref z) in bln_3()) {
    Ring::axiom_right_distributivity((x, y, z))
  }


  #[test]
  fn prop_left_zero_absorb(ref x in bln_1()) {
    Ring::prop_left_zero_absorb(x)
  }


  #[test]
  fn prop_right_zero_absorb(ref x in bln_1()) {
    Ring::prop_right_zero_absorb(x)
  }


  #[test]
  fn prop_left_mul_negate((ref x, ref y) in bln_2()) {
    Ring::prop_left_mul_negate((x, y))
  }


  #[test]
  fn prop_right_mul_negate((ref x, ref y) in bln_2()) {
    Ring::prop_right_mul_negate((x, y))
  }


  #[test]
  fn prop_mul_negate((ref x, ref y) in bln_2()) {
    Ring::prop_mul_negate((x, y))
  }


  #[test]
  fn prop_left_one_negate(ref x in bln_1()) {
    Ring::prop_left_one_negate(x)
  }


  #[test]
  fn prop_right_one_negate(ref x in bln_1()) {
    Ring::prop_right_one_negate(x)
  }


  #[test]
  fn axiom_ring_mul_commutivity((ref x, ref y) in bln_2()) {
    ComRing::axiom_mul_commutivity((x, y))
  }


  #[test]
  fn axiom_field_left_invert(ref x in bln_1()) {
    Field::axiom_left_invert(x)
  }


  #[test]
  fn axiom_field_right_invert(ref x in bln_1()) {
    Field::axiom_left_invert(x)
  }


  #[test]
  fn prop_mul_zero((ref x, ref y) in bln_2()) {
    Field::prop_mul_zero((x, y))
  }


  #[test]
  fn prop_add_cancel((ref w, ref x, ref y) in bln_3()) {
    Field::prop_add_cancel((w, x, y))
  }


  #[test]
  fn prop_mul_cancel((ref w, ref x, ref y) in bln_3()) {
    Field::prop_mul_cancel((w, x, y))
  }
}


fn main() {}

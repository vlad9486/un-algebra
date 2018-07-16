//
// # Example: The finite field _F4_.
//
// The values {`O`,`I`, `A`, `B`} form a _finite field_ known as _F4_
// or _GF(4)_ [GF4] with operations as defined by [GF4].
//
// [GF4]: https://en.wikipedia.org/wiki/Field_(mathematics)


// Use the un_algebra crate traits.
extern crate un_algebra;
use un_algebra::prelude::*;


// Use proptest for generative testing.
#[macro_use]
extern crate proptest;


//
// The finite field with four elements _F4_ or _GF(4)_. 
//
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum F4 {
  O, I, A, B
}


//
// F4 values form an additive magma.
//
impl AddMagma for F4 {

  // Magma addition as per the F4 definition.
  fn add(&self, other: &Self) -> Self {

    // This match expression could be more condensed, but listing
    // every additive pair makes the F4 addition rules clearer.
    match (self, other) {
      (F4::O, F4::O) => F4::O,
      (F4::O, F4::I) => F4::I,
      (F4::O, F4::A) => F4::A,
      (F4::O, F4::B) => F4::B,
      (F4::I, F4::O) => F4::I,
      (F4::I, F4::I) => F4::O,
      (F4::I, F4::A) => F4::B,
      (F4::I, F4::B) => F4::A,
      (F4::A, F4::O) => F4::A,
      (F4::A, F4::I) => F4::B,
      (F4::A, F4::A) => F4::O,
      (F4::A, F4::B) => F4::I,
      (F4::B, F4::O) => F4::B,
      (F4::B, F4::I) => F4::A,
      (F4::B, F4::A) => F4::I,
      (F4::B, F4::B) => F4::O,
    }
  }
}


//
// F4 values form an additive semigroup.
//
impl AddSemigroup for F4 {}


//
// F4 values form an additive monoid.
//
impl AddMonoid for F4 {

  // The F4 _O_ element acts as zero.
  fn zero() -> Self {
    F4::O
  }
}


//
// F4 values form an additive group.
//
impl AddGroup for F4 {

  // F4 elements are their own additive inverses.
  fn negate(&self) -> Self {
    self.clone()
  }
}


//
// F4 values form an additive commutative group.
//
impl AddComGroup for F4 {}



//
// F4 values form a multiplicative magma.
//
impl MulMagma for F4 {

  // Magma addition as per the F4 definition.
  fn mul(&self, other: &Self) -> Self {

    // This match expression could be more condensed, but listing
    // every multiplicative pair makes the F4 multiplication rules
    // clearer.
    match (self, other) {
      (F4::O, F4::O) => F4::O,
      (F4::O, F4::I) => F4::O,
      (F4::O, F4::A) => F4::O,
      (F4::O, F4::B) => F4::O,
      (F4::I, F4::O) => F4::O,
      (F4::I, F4::I) => F4::I,
      (F4::I, F4::A) => F4::A,
      (F4::I, F4::B) => F4::B,
      (F4::A, F4::O) => F4::O,
      (F4::A, F4::I) => F4::A,
      (F4::A, F4::A) => F4::B,
      (F4::A, F4::B) => F4::I,
      (F4::B, F4::O) => F4::O,
      (F4::B, F4::I) => F4::B,
      (F4::B, F4::A) => F4::I,
      (F4::B, F4::B) => F4::A,
    }
  }
}


//
// F4 values form a multiplicative semigroup.
//
impl MulSemigroup for F4 {}


//
// F4 values form a multiplicative monoid.
//
impl MulMonoid for F4 {

  // The F4 _I_ value acts as one.
  fn one() -> Self {
    F4::I
  }
}


//
// F4 values form a multiplicative group.
//
impl MulGroup for F4 {

  // Group inverses as per the F4 definition.
  fn invert(&self) -> Self {
    match self {
      F4::O => F4::O,
      F4::I => F4::I,
      F4::A => F4::B,
      F4::B => F4::A,
    }
  }

  
  // All F4 elements are invertible.
  fn is_invertible(&self) -> bool {
    true
  }  
}


//
// F4 values form a multiplicative commutative group.
//
impl MulComGroup for F4 {}


//
// F4 values form a ring.
//
impl Ring for F4 {}


//
// F4 values form a commutative ring.
//
impl ComRing for F4 {}


//
// F4 values form a field.
//
impl Field for F4 {

  // Field inverses as per the F4 definition.
  fn invert(&self) -> Self {
    match self {
      F4::O => F4::O,
      F4::I => F4::I,
      F4::A => F4::B,
      F4::B => F4::A,
    }
  }

 
  // All F4 elements are invertible.
  fn is_invertible(&self) -> bool {
    true
  }    
}


prop_compose! {
  // Generate 1 random F4 test value.
  fn f4_1()(i in 0..3) -> F4 {
    ([F4::O, F4::I, F4::A, F4::B][i as usize]).clone()
  }
}


prop_compose! {
  // Generate 2 random F4 test values.
  fn f4_2()(xs in (f4_1(), f4_1())) -> (F4, F4) {
    xs
  }
}


prop_compose! {
  // Generate 3 random F4 test values.
  fn f4_3()(xs in (f4_1(), f4_1(), f4_1())) -> (F4, F4, F4) {
    xs
  }
}


// Generative tests of F4 algebraic axioms and properties.
proptest! {
  
  #[test]
  fn axiom_add_closure((ref x, ref y) in f4_2()) {
    AddMagma::axiom_add_closure((x, y))
  }


  #[test]
  fn axiom_mul_closure((ref x, ref y) in f4_2()) {
    MulMagma::axiom_mul_closure((x, y))
  }


  #[test]
  fn axiom_mul_associativity((ref w, ref x, ref y) in f4_3()) {
    MulSemigroup::axiom_mul_associativity((w, x, y))
  }


  #[test]
  fn axiom_add_associativity((ref w, ref x, ref y) in f4_3()) {
    AddSemigroup::axiom_add_associativity((w, x, y))
  }


  #[test]
  fn axiom_left_add_identity(ref x in f4_1()) {
    AddMonoid::axiom_left_add_identity(x)
  }


  #[test]
  fn axiom_right_add_identity(ref x in f4_1()) {
    AddMonoid::axiom_left_add_identity(x)
  }


  #[test]
  fn axiom_left_mul_identity(ref x in f4_1()) {
    MulMonoid::axiom_left_mul_identity(x)
  }


  #[test]
  fn axiom_right_mul_identity(ref x in f4_1()) {
    MulMonoid::axiom_right_mul_identity(x)
  }


  #[test]
  fn axiom_left_negate(ref x in f4_1()) {
    AddGroup::axiom_left_negate(x)
  }


  #[test]
  fn axiom_right_negate(ref x in f4_1()) {
    AddGroup::axiom_right_negate(x)
  }


  #[test]
  fn axiom_left_invert(ref x in f4_1()) {
    MulGroup::axiom_left_invert(x)
  }


  #[test]
  fn axiom_right_invert(ref x in f4_1()) {
    MulGroup::axiom_right_invert(x)
  }


  #[test]
  fn axiom_add_commutivity((ref x, ref y) in f4_2()) {
    AddComGroup::axiom_add_commutivity((x, y))
  }


  #[test]
  fn axiom_mul_commutivity((ref x, ref y) in f4_2()) {
    MulComGroup::axiom_mul_commutivity((x, y))
  }


  #[test]
  fn axiom_left_distributivity((ref x, ref y, ref z) in f4_3()) {
    Ring::axiom_left_distributivity((x, y, z))
  }


  #[test]
  fn axiom_right_distributivity((ref x, ref y, ref z) in f4_3()) {
    Ring::axiom_right_distributivity((x, y, z))
  }


  #[test]
  fn prop_left_zero_absorb(ref x in f4_1()) {
    Ring::prop_left_zero_absorb(x)
  }


  #[test]
  fn prop_right_zero_absorb(ref x in f4_1()) {
    Ring::prop_right_zero_absorb(x)
  }


  #[test]
  fn prop_left_mul_negate((ref x, ref y) in f4_2()) {
    Ring::prop_left_mul_negate((x, y))
  }


  #[test]
  fn prop_right_mul_negate((ref x, ref y) in f4_2()) {
    Ring::prop_right_mul_negate((x, y))
  }


  #[test]
  fn prop_mul_negate((ref x, ref y) in f4_2()) {
    Ring::prop_mul_negate((x, y))
  }


  #[test]
  fn prop_left_one_negate(ref x in f4_1()) {
    Ring::prop_left_one_negate(x)
  }


  #[test]
  fn prop_right_one_negate(ref x in f4_1()) {
    Ring::prop_right_one_negate(x)
  }


  #[test]
  fn axiom_ring_mul_commutivity((ref x, ref y) in f4_2()) {
    ComRing::axiom_mul_commutivity((x, y))
  }


  #[test]
  fn axiom_field_left_invert(ref x in f4_1()) {
    Field::axiom_left_invert(x)
  }


  #[test]
  fn axiom_field_right_invert(ref x in f4_1()) {
    Field::axiom_left_invert(x)
  }


  #[test]
  fn prop_mul_zero((ref x, ref y) in f4_2()) {
    Field::prop_mul_zero((x, y))
  }


  #[test]
  fn prop_add_cancel((ref w, ref x, ref y) in f4_3()) {
    Field::prop_add_cancel((w, x, y))
  }


  #[test]
  fn prop_mul_cancel((ref w, ref x, ref y) in f4_3()) {
    Field::prop_mul_cancel((w, x, y))
  }
}


fn main() {
}

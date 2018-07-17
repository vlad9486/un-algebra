use ring::ring::NumRing;
use monoid::mul_monoid::NumMulMonoid;
use com_group::add_com_group::NumAddComGroup;
use group::add_group::NumAddGroup;
use monoid::add_monoid::NumAddMonoid;
use semigroup::add_semigroup::NumAddSemigroup;
use magma::add_magma::NumAddMagma;
use semigroup::mul_semigroup::NumMulSemigroup;
use magma::mul_magma::NumMulMagma;

use numeric::equal::NumEq;
use std::cmp::PartialEq;

use super::complex::Real;

// TODO: move in other module
pub trait Functor {
    type Wrapped;

    fn fmap<F>(&self, f: F) -> Self where F: Fn(&Self::Wrapped) -> Self::Wrapped;
}

// TODO: move in other module
pub trait BiFunctor {
    type Wrapped;

    fn fmap<F>(&self, other: &Self, f: F) -> Self where
        F: Fn(&Self::Wrapped, &Self::Wrapped) -> Self::Wrapped;
}

/// The C* algebra obtained by the Cayley-Dickson process
pub trait CayleyDicksonAlgebra: NumRing {
    fn conjugate(&self) -> Self;
}

/// The Cayley-Dickson process base
impl<T> CayleyDicksonAlgebra for T where T: Real {
    fn conjugate(&self) -> Self { self.clone() }
}

/// The Cayley-Dickson process step
#[derive(Debug)]
pub struct CayleyDicksonPair<T> where T: CayleyDicksonAlgebra {
    pair: (T, T),
}

impl<T> Functor for CayleyDicksonPair<T> where T: CayleyDicksonAlgebra {
    type Wrapped = T;

    fn fmap<F>(&self, f: F) -> Self where F: Fn(&Self::Wrapped) -> Self::Wrapped {
        let (ref a, ref b) = self.pair;
        CayleyDicksonPair::new((f(a), f(b)))
    }
}

impl<T> BiFunctor for CayleyDicksonPair<T> where T: CayleyDicksonAlgebra {
    type Wrapped = T;

    fn fmap<F>(&self, other: &Self, f: F) -> Self where
        F: Fn(&Self::Wrapped, &Self::Wrapped) -> Self::Wrapped
    {
        let (ref a, ref b) = self.pair;
        let (ref c, ref d) = other.pair;
        CayleyDicksonPair::new((f(a, c), f(b, d)))
    }
}

impl<T> CayleyDicksonPair<T> where T: CayleyDicksonAlgebra {
    pub fn new(pair: (T, T)) -> Self {
        CayleyDicksonPair {
            pair: pair as _,
        }
    }
}

impl<T> CayleyDicksonAlgebra for CayleyDicksonPair<T> where T: CayleyDicksonAlgebra {
    fn conjugate(&self) -> Self {
        let (ref a, ref b) = self.pair;
        CayleyDicksonPair::new((a.conjugate(), b.negate()))
    }
}

impl<T> Clone for CayleyDicksonPair<T> where T: CayleyDicksonAlgebra + Clone {
    fn clone(&self) -> Self {
        Functor::fmap(&self, Clone::clone)
    }
}

impl<T> NumRing for CayleyDicksonPair<T> where T: CayleyDicksonAlgebra {
}

impl<T> NumMulMonoid for CayleyDicksonPair<T> where T: CayleyDicksonAlgebra {
    fn one() -> Self {
        CayleyDicksonPair::new((T::one(), T::zero()))
    }
}

impl<T> NumAddComGroup for CayleyDicksonPair<T> where T: CayleyDicksonAlgebra {
}

impl<T> NumAddGroup for CayleyDicksonPair<T> where T: CayleyDicksonAlgebra {
    fn negate(&self) -> Self {
        Functor::fmap(&self, T::negate)
    }
}

impl<T> NumAddMonoid for CayleyDicksonPair<T> where T: CayleyDicksonAlgebra {
    fn zero() -> Self {
        CayleyDicksonPair::new((T::zero(), T::zero()))
    }
}

impl<T> NumAddSemigroup for CayleyDicksonPair<T> where T: CayleyDicksonAlgebra {
}

impl<T> NumAddMagma for CayleyDicksonPair<T> where T: CayleyDicksonAlgebra {
    fn add(&self, other: &Self) -> Self {
        BiFunctor::fmap(&self, &other, T::add)
    }
}

impl<T> NumMulSemigroup for CayleyDicksonPair<T> where T: CayleyDicksonAlgebra {
}

impl<T> NumMulMagma for CayleyDicksonPair<T> where T: CayleyDicksonAlgebra {
    fn mul(&self, other: &Self) -> Self {
        let (ref a, ref b) = self.pair;
        let (ref c, ref d) = other.pair;
        // here is the magic
        CayleyDicksonPair::new((
            a.mul(c).add(&d.conjugate().mul(b).negate()),
            d.mul(a).add(&b.mul(&c.conjugate())),
        ))
    }
}

impl<T> NumEq for CayleyDicksonPair<T> where T: CayleyDicksonAlgebra {
    type Error = T::Error;

    fn num_eq(&self, other: &Self, _eps: &Self::Error) -> bool {
        let (ref a, ref b) = self.pair;
        let (ref c, ref d) = other.pair;
        a.num_eq(c, _eps) && b.num_eq(d, _eps)
    }
}

impl<T> PartialEq for CayleyDicksonPair<T> where T: CayleyDicksonAlgebra {
    fn eq(&self, other: &Self) -> bool {
        let (ref a, ref b) = self.pair;
        let (ref c, ref d) = other.pair;
        a == c && b == d
    }
}

pub trait LinearMaps: CayleyDicksonAlgebra {
    /// a - a*
    /// vanishing if Self is Real
    fn hermitator(a: &Self) -> Self;

    /// a * b - b * a
    /// vanishing if Self is Real or Complex
    fn commutator(a: &Self, b: &Self) -> Self;

    /// (a * b) * c - a * (b * c)
    /// vanishing if Self is Real or Complex or Quaternion
    fn associator(a: &Self, b: &Self, c: &Self) -> Self;
}

impl<T> LinearMaps for T where T: CayleyDicksonAlgebra {
    fn hermitator(a: &Self) -> Self {
        a.add(&a.conjugate().negate())
    }

    fn commutator(a: &Self, b: &Self) -> Self {
        a.mul(b).add(&b.mul(&a).negate())
    }

    fn associator(a: &Self, b: &Self, c: &Self) -> Self {
        let _ab_c = a.mul(&b).mul(&c);
        let _a_bc = a.mul(&b.mul(&c));
        _ab_c.add(&_a_bc.negate())
    }
}

#[cfg(test)]
mod test {
    #![allow(unused_imports)]

    use super::CayleyDicksonAlgebra;
    use super::CayleyDicksonPair;
    use super::LinearMaps;

    use monoid::add_monoid::NumAddMonoid;
    use monoid::mul_monoid::NumMulMonoid;
    use magma::add_magma::NumAddMagma;
    use magma::mul_magma::NumMulMagma;
    use numeric::equal::NumEq;

    #[test]
    fn test() {
        type Complex = CayleyDicksonPair<f64>;
        type Quaternion = CayleyDicksonPair<Complex>;

        let q = Quaternion::new((
            Complex::new((
                2.6, -5.7,
            )),
            Complex::new((
                8.3, 7.2,
            )),
        ));
        // just need some random quaternions, TODO: add randomization
        let w = q.mul(&q);
        let e = w.mul(&q);

        let a = LinearMaps::associator(&q, &w, &e);
        println!("{:?}", a);
        assert_eq!(a.num_eq(&Quaternion::zero(), &1e-8), true);
        assert_eq!(LinearMaps::hermitator(&q.mul(&q.conjugate())).num_eq(&Quaternion::zero(), &1e-8), true);
        assert_eq!(LinearMaps::hermitator(&w.mul(&w.conjugate())).num_eq(&Quaternion::zero(), &1e-8), true);
        assert_eq!(LinearMaps::hermitator(&e.mul(&e.conjugate())).num_eq(&Quaternion::zero(), &1e-8), true);
    }
}

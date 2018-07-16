# `un_algebra`

[![crate link][crate-SVG]][crate-URL]
[![documentation][docs-SVG]][docs-URL]
[![rustc version][rustc-SVG]][rustc-URL]
[![license][license-SVG]][license-URL]


Simple Rust implementations of selected abstract algebraic structures.

## Synopsis

Mathematical _abstract algebra_ is built on a rich collection of
algebraic _structures_. Learning about these structures can give
non-mathematicians insights into the mathematical entities they need
to work with--for example, real numbers, complex numbers, vectors,
matrices, and permutations. By definition, these structures must
comply with sets of _axioms_ and _properties_, which are in turn a
rich source of properties for generative testing.

`un_algebra` (\_un\_derstanding \_algebra\_) is a simple
implementation of selected algebraic structures in Rust. I hope it
is useful for developers learning abstract algebra concepts for the
first time. Currently this crate provides _magma_, _semigroup_,
_monoid_, _group_, _ring_ and _field_ implementations.


## Documentation

See https://docs.rs/un_algebra


## Contributions

Please refer to the [contributing] guide.


### Installation

Add `un_algebra` to your Cargo.toml dependencies:

```toml
[dependencies]
un_algebra = "0.1.0"
```

Add a reference to `un_algebra` to your crate root:

```rust
extern crate un_algebra;
```


## Production use

`un_algebra` is intended to support self-study of abstract algebraic
structures--it is not optimized for use in a production
environment. For production environments I recommend using a more
sophisticated library like [alga].


## Compatibility

`un_algebra` currently requires Rust _nightly_ as it makes use of
the (experimental) external documentation feature.


## Errors

I'm not a mathematician so my implementation of the various
structures and their respective axioms in `un_algebra` may not be
strictly correct. Please let me know of any errors.


## Examples

`un_algebra` implements the relevant structure traits for all the
Rust standard library integer and floating point numeric types, for
example, an _additive_ _group_ for integer types `i8`, `i16`, `i32`,
etc.

The Rust standard library has no support for complex numbers
(&#x2102;) or rational numbers (&#x211a;) so I've used the complex
and rational types from the [num] crate and implemented the
conforming traits in the [`complex`] and [`rational`] modules.

In addition, the crate examples directory contains abstract
structure implementations of selected concepts, for example,
_finite_ _fields_.


## Example

Rust's planned `i128` type forms several `un_algebra` algebraic
structures, starting with additive and multiplicative _magmas_ (with
"wrapping" or modular arithmetic):

```rust
pub use un_algebra::prelude::*;

impl AddMagma for i128 {
  fn add(&self, other: &Self) -> Self {
    self.wrapping_add(other)
  }
}

impl MulMagma for i128 {
  fn mul(&self, other: &Self) -> Self {
    self.wrapping_mul(other)
  }
}
````

`i128` also forms additive and multiplicative _semigroups_:

```rust
impl AddSemigroup for i128 {}

impl MulSemigroup for i128 {}
````

And additive and multiplicative _monoids_ with one and zero as the
monoid _identities_:

```rust
impl AddMonoid for i128 {
  fn zero() -> Self {
    0
  }
}

impl MulMonoid for i128 {
  fn one() -> Self {
    1
  }
}
```

`i128` also forms an _additive_ _group_ and _additive_ _commutative_
_group_ (with "wrapping" or modular negation), but not a
_multiplicative_ _group_, as the integers have no closed division
operation:

```rust
impl AddGroup for i128 {
  fn negate(&self) -> Self {
    self.wrapping_neg()
  }
}

impl AddComGroup for i128 {}
```

And a _ring_ and _commutative_ _ring_:

```rust
impl Ring for i128 {}

impl CommRing for i128 {}
```


## References

Please refer to the [reading] document for more background on each
structure and its associated axioms and properties.


## To-Do

* Research other two-operation structures like _semirings_,
  _division_ _rings_, and _integral_ _domains_. Adding these might
  make the transition from groups up to fields more granular?

* Research structures that build on or reference, _fields_, for
  example _modules_ or _vector_ _spaces_.

* The _field_ traits probably need more testable derived properties.

* Find more interesting examples for the `examples` directory.

* Find a better method for embedding mathematical expressions in
  automatically generated Rust documentation.


## License

This project is licensed under the MIT license (see LICENSE.txt or
[license-URL]).



<!-- Reference links used above -->

[alga]: https://crates.io/crates/alga

[reading]: https://gitlab.com/ornamentist/un-algebra/blob/master/doc/reading.md

[contributing]: https://gitlab.com/ornamentist/un-algebra/blob/master/CONTRIBUTING.md

[crate-SVG]: https://img.shields.io/crates/v/un_algebra.svg

[crate-URL]: https://crates.io/crates/un_algebra

[docs-SVG]: https://docs.rs/un_algebra/badge.svg

[docs-URL]: https://docs.rs/un_algebra

[rustc-SVG]: https://img.shields.io/badge/rustc-nightly-red.svg

[rustc-URL]: https://www.rust-lang.org

[license-SVG]: https://img.shields.io/badge/license-MIT-blue.svg

[license-URL]: https://opensource.org/licenses/MIT





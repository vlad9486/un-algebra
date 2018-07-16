#![feature(external_doc)]

//!
//! Simple implementations of selected abstract algebraic structures.
//!
//! # Rationale
//!
//! Mathematical _abstract algebra_ is built on a rich collection of
//! algebraic _structures_. Learning about these structures can give
//! non-mathematicians insights into the mathematical entities they
//! need to work with--for example, real numbers, complex numbers,
//! vectors, matrices, and permutations. By definition, these
//! structures must comply with sets of _axioms_ and _properties_,
//! which are in turn a rich source of properties for generative
//! testing.
//!
//! `un_algebra` ("***un***derstanding ***algebra***") is a simple
//! implementation of selected algebraic structures in Rust. I hope
//! it is useful to developers learning abstract algebra concepts
//! for the first time. Currently this crate provides _magma_,
//! _semigroup_, _monoid_, _group_, _ring_ and _field_
//! implementations.
//! 
//! # Production use
//!
//! `un_algebra` is intended to support self-study of abstract
//! algebraic structures--it is not optimized for use in a
//! production environment. For production environments I recommend
//! using a more sophisticated library like [alga].
//!
//! # Compatibility
//!
//! `un_algebra` currently requires a Rust _nightly_ installation as
//! it makes use of the (experimental) external documentation
//! feature.
//!
//! # Errors
//!
//! I'm not a mathematician so my implementation of the various
//! structures and their respective axioms in `un_algebra` may not
//! be strictly correct. Please let me know of any errors.
//!
//! Please refer to the [references] document for more background on
//! each structure and its associated axioms and properties.
//!
//! # Glossary
//!
//! The names of the `un_algebra` structures and their respective
//! axioms can be long and unwieldy, for example, "numeric
//! commutative multiplicative group". To keep the exported names
//! workable I use these abbreviations in trait and function names:
//!
//! * `Num` for "numeric"
//! * `eps` for "epsilon"
//! * `Add` for "additive"
//! * `Com` for "commutative"
//! * `Mul` for "multiplicative"
//!
//! # Organization
//!
//! Each algebraic structure implemented in `un_algebra` is bundled
//! as a module of structure traits and unit tests of the trait
//! axioms.  For example, the modules [`group`], [`ring`], or
//! [`field`].
//!
//! All structures have an _abstract_ version of the structure trait
//! (for example [`Group`]) and, where they are commonly used,
//! _additive_ and _multiplicative_ versions of the structure trait
//! (for example [`AddGroup`] and [`MulGroup`]). These traits are
//! defined using the terminology of addition and multiplication
//! rather than as abstract binary operations.
//!
//! # Axioms and properties
//!
//! All `un_algebra` structure traits are equipped with predicate
//! functions that implement the structure _axioms_. Some structures
//! also have predicate functions that implement derived
//! _properties_ of the structure.
//!
//! These properties are not strictly necessary since they can be
//! derived from the axioms, but they do allow richer generative
//! testing of trait implementations, especially those using
//! floating point numbers.
//!
//! Axiom predicate functions start with an `axiom_` prefix and
//! property predicate functions with a `prop_` prefix.
//!
//! # Macros
//!
//! User defined traits that are implementable by Rust's built-in
//! numeric types seem to quickly lead to a lot of tedious, repeated
//! `impl` code, or to using tricky self-referential macros. This
//! could be due to missing abstractions in Rust's numeric type
//! heirarchy or (more likely) my lack of Rust experience.
//!
//! Where a trait's `impl` code is only repeated a couple of times
//! modules use the boilerplate code approach and in other cases
//! they rely on a macro to create the `impl` items.
//!
//! # Unit tests
//!
//! Where `un_algebra` traits provide unit tests they are generally
//! _generative_ tests built on the [`proptest`] generative testing
//! crate. These generative tests do not cover every Rust built-in
//! numeric type, mainly to avoid creating either repetitive
//! boilerplate testing code or even more self-referential macros.
//!
//! To avoid cluttering up module source files, the unit tests for
//! most modules are defined in separate files. The module source
//! files use the `path =` attribute to link the source and test
//! files.
//!
//! # Integer types
//!
//! Rust's built-in integer types (for example `i32`) are finite
//! subsets of the natural numbers (&#x2115;). This means they can
//! only satisfy abstract structure axioms with modulo, or
//! "wrapping" addition and multiplication.
//!
//! # Floating point types
//! 
//! Many application data types that in theory conform to modern
//! algebraic structures make heavy use of IEEE floating point
//! numbers. Unfortunately, these numbers are only a finite subset
//! of the real numbers (&#x211d;) and they do not reliably satisfy
//! even the simplest real number axioms ([IEEE]).
//!
//! For working with IEEE floating point types `un_algebra` also
//! provides _numeric_ versions of the additive and multiplicative
//! traits. These traits provide axiom and property functions
//! accepting a numeric error or _epsilon_ term--for example,
//! [`NumAddGroup`] and [`NumMulGroup`]. Using these numeric trait
//! variants we can at least numerically satisfy the trait axioms.
//!
//! # Examples
//!
//! `un_algebra` implements the relevant structure traits for all
//! the Rust standard library integer and floating point numeric
//! types, for example, an _additive_ _group_ for integer types
//! `i8`, `i16`, `i32`, etc.
//!
//! The Rust standard library has no support for complex numbers
//! (&#x2102;) or rational numbers (&#x211a;) so I've used the
//! complex and rational types from the [num] crate and implemented
//! the conforming traits in the [`complex`] and [`rational`]
//! modules.
//!
//! In addition, the crate examples directory contains abstract
//! structure implementations of selected concepts, for example,
//! _finite_ _fields_.
//!
#![doc(include = "../doc/references.md")]


// Supporting crates needed.
#[macro_use]
extern crate proptest;
extern crate float_cmp;
extern crate num;


// Un-algebra public modules.
pub mod types;
pub mod logic;
pub mod numeric;
pub mod magma;
pub mod semigroup;
pub mod monoid;
pub mod group;
pub mod com_group;
pub mod ring;
pub mod com_ring;
pub mod field;
pub mod complex;
pub mod rational;
pub mod prelude;
pub mod tests;


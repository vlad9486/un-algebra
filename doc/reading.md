# Abstract Algebra References

I found the following references useful in creating the `un_algebra`
crate. This is far from an exhaustive list, I'm sure there are many
other fine references available.


## Wikipedia

Wikipedia is often a useful starting point when researching a
topic. For abstract algebra it provides an overview of commonly used
abstract structures, their relationships and axioms, and further
references:

https://en.wikipedia.org/w/index.php?title=Magma_(algebra)



## Textbooks

I found this abstract algebra textbook to be clear and approachable
for a non-mathematican, although there are almost certainly other
textbooks that fit that description too:

    @Book{GAL-2017,
      author    = {Gallian, Joseph},
      title     = {Contemporary abstract algebra},
      publisher = {Cengage Learning},
      year      = {2017},
      address   = {Boston, MA},
      isbn      = {978-1305657960}
    }


## Rust crates

The `alga` Rust crate implements most commonly used algebraic
structures and is much better suited to a production environment
than `un_algebra`:

https://crates.io/crates/alga


## Haskell libraries

Here's an alternate Haskell _prelude_ with in-depth support for
algebraic and mathematical structures.

https://hackage.haskell.org/package/numeric-prelude


## Purescript libraries

The Purescript prelude has comprehensive support for algebraic
structures in its numeric heirarchy. This in-depth documentation
explains the rationale behind it:

https://a-guide-to-the-purescript-numeric-hierarchy.readthedocs.io


## Video lessons

I found Bill Shillito's "Introduction to Higher Mathematics" video
series very approachable, in fact some of the best online
mathematical exposition I've seen:

https://www.youtube.com/watch?v=CMWFmjlB8v0&list=PLZzHxk_TPOStgPtqRZ6KzmkUQBQ8TSWVX



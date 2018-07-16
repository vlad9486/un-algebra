# A simple GNU makefile for performing Rust build steps at the
# command line. Using a makefile this way may not be idiomatic Rust
# development practice.


# Assume cargo is installed in our home directory.
CARGO :=~/.cargo/bin/cargo


# Use clippy to do linting of Rust source.
lint:
	${CARGO} clean -p un_algebra
	${CARGO} clippy -- -W clippy-pedantic


# Use cargo doc to create documentation.
doc:
	${CARGO} clean --doc
	${CARGO} doc --no-deps --open


# Use cargo test to run unit tests.
tests:
	${CARGO} test --examples --tests


.PHONY: lint
.PHONY: doc
.PHONY: tests

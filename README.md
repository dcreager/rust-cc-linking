# Some fun linker errors

This is a small repro of some linker errors we're seeing with rustc, Cargo, and
the [`cc`][] library.

[`cc`]: https://crates.io/crates/cc

The root of the problem is how rustc deduplicates redundant `-l` flags when
calling the linker.  This causes problems if we try to:

  - link with multiple internal libraries ("A" and "B"), built using [`cc`][],
  - each of which depends on an external library (in this case, libstdc++),
  - but which depend on different sets of symbols from that library.

If one of the libraries depends on a strict subset of the symbols needed by the
other, **and** the "smaller" library is linked **after** the "larger" one, then
the compilation [happens to succeed](cc-01-works-by-accident/build.rs).  If you
link the "smaller" library **before** the "larger" one, then the compilation
[fails](cc-02-broken/build.rs).  (But only on Linux.)

## Steps to reproduce

Verified on Linux:

1. `git clone https://github.com/dcreager/rust-cc-linking`
2. `cd rust-cc-linking`
3. `cargo build -p cc-01-works-by-accident`
4. `cargo build -p cc-02-broken`

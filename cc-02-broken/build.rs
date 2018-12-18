extern crate cc;

// big.cc and small.cc both link with the C++ standard library.  small.cc doesn't actually
// reference any symbols from it; big.cc references some std::string methods.
//
// In this example, we link small.cc into our Rust program first.  That ends up with final rustc
// and linker commands roughly like:
//
//    $ rustc [path to compiled Rust files] -lsmall -lstdc++ -lbig -lstdc++
//    $ cc [path to compiled Rust files] -lsmall -lstdc++ -lbig
//
// rustc drops the second reference to libstdc++, warning that it's redundant.
//
// When the linker encounters `-lstdc++`, it will only bring in the symbols that have been
// referenced at that point: in this case, the methods the small.cc refers to.  However, small.cc
// doesn't reference anything in libstdc++, so nothing from the C++ standard library is included in
// the final program!  When we then try to link with `-lbig`, we won't find the std::string methods
// that it tries to call, and we get "undefined symbol" errors from the linker.

fn main() {
    println!("cargo:rerun-if-changed=src/small.cc");
    cc::Build::new()
        .cpp(true)
        .file("src/small.cc")
        .compile("small");

    println!("cargo:rerun-if-changed=src/big.cc");
    cc::Build::new().cpp(true).file("src/big.cc").compile("big");
}

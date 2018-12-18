extern crate cc;

// big.cc and small.cc both link with the C++ standard library.  small.cc doesn't actually
// reference any symbols from it; big.cc references some std::string methods.
//
// In this example, we link big.cc into our Rust program first.  That ends up with final rustc and
// linker commands roughly like:
//
//    $ rustc [path to compiled Rust files] -lbig -lstdc++ -lsmall -lstdc++
//    $ cc [path to compiled Rust files] -lbig -lstdc++ -lsmall
//
// rustc drops the second reference to libstdc++, warning that it's redundant.
//
// When the linker encounters `-lstdc++`, it will only bring in the symbols that have been
// referenced at that point: in this case, the methods the big.cc refers to.  None of the symbols
// referenced in small.cc are brought in; however, since small.cc doesn't actually reference
// anything in libstd+cc, this happens to still work!

fn main() {
    println!("cargo:rerun-if-changed=src/big.cc");
    cc::Build::new().cpp(true).file("src/big.cc").compile("big");

    println!("cargo:rerun-if-changed=src/small.cc");
    cc::Build::new()
        .cpp(true)
        .file("src/small.cc")
        .compile("small");
}

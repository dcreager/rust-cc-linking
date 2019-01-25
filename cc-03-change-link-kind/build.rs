extern crate cc;

// This example shows that no warning is printed out if you reference a library a second type while
// changing something about it — in this case, it's "kind".

fn main() {
    println!("cargo:rerun-if-changed=src/small.cc");
    cc::Build::new()
        .cpp(true)
        .file("src/small.cc")
        .compile("small");

    // This is redundant but counts as "changing" the library link type (it has to be *specified*
    // for rustc to consider it changed, not *different*.)  So this line should not generate a
    // warning before or after the fix.
    println!("cargo:rustc-link-lib=static=small");
}

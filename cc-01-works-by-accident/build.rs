extern crate cc;

fn main() {
    println!("cargo:rerun-if-changed=src/big.cc");
    cc::Build::new().cpp(true).file("src/big.cc").compile("big");

    println!("cargo:rerun-if-changed=src/small.cc");
    cc::Build::new()
        .cpp(true)
        .file("src/small.cc")
        .compile("small");
}

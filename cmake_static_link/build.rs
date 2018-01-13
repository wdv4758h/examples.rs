extern crate cmake;


fn main() {
    let dst = cmake::build("cmake_src");
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=func");
}


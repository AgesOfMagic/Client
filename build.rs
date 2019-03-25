extern crate cc;


fn main(){
    cc::Build::new()
        .file("./c/protocol.c")
        .compile("lib");
    println!("cargo:rustc-link-lib=static=lib");
}
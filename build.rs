extern crate cc;


fn main(){
    cc::Build::new()
        .file("./main.c")
        .compile("lib");
    println!("cargo:rustc-link-lib=static=lib");
}
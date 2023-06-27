use bindgen;
use cc;
use std::path::PathBuf;

fn main() {
    cc::Build::new()
        .file("./src/c/bch.c")
        .compile("bch");
    println!("cargo:rustc-link-search=.");
    println!("cargo:rustc-link-lib=static=bch");


    let bindings = bindgen::Builder::default()
        .header("./src/c/bch.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("unable to generate bindings");
    
    let out_path = PathBuf::from("src");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

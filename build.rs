use std::env::var;
use std::env;
use std::path::PathBuf;

fn main() {

    println!("cargo:rustc-link-search={}/SuiteSparse-5.11.0/lib", env::var("HOME").unwrap());
    println!("cargo:rustc-link-lib=dylib=cxsparse");
    println!("cargo:rustc-link-search={}/lib/intel64/", env::var("MKLROOT").unwrap());
    println!("cargo:rustc-link-lib=dylib=mkl_intel_lp64");
    println!("cargo:rustc-link-lib=dylib=mkl_gnu_thread");
    println!("cargo:rustc-link-lib=dylib=mkl_core");
    println!("cargo:rustc-link-lib=dylib=gomp");
    println!("cargo:rustc-link-lib=dylib=pthread");
    println!("cargo:rustc-link-lib=dylib=m");
    println!("cargo:rustc-link-lib=dylib=dl");
    //
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_args([format!("-I/{}/SuiteSparse-5.11.0/include/", env::var("HOME").unwrap())]
        )
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from("./src/c_binding");
    bindings
        .write_to_file(out_path.join("sparse_blas.rs"))
        .expect("Couldn't write bindings!");
}

fn env_or_default(var_name: &str, default: &str) -> String {
    match var(var_name) {
        Ok(s) => s,
        Err(_) => default.into(),
    }
}
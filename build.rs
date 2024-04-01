use std::env;
use std::path::PathBuf;

fn main() {
    let webots_path = env::var("WEBOTS_PATH").expect("The WEBOTS_PATH should be defined");
    let lib_path = PathBuf::from(&webots_path).join("lib/controller");
    let include_path = PathBuf::from(&webots_path).join("Contents/include/controller/c");

    println!("cargo:rustc-link-search={}", lib_path.display());
    println!("cargo:rustc-link-lib=Controller");
    println!("cargo:rustc-env=LD_LIBRARY_PATH={}", lib_path.display());
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("include_path: {:?}",include_path);
    println!("lib_path: {:?}",lib_path);

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .clang_args(vec!["-I", include_path.to_str().unwrap()])
        .blocklist_item("FP_INFINITE")
        .blocklist_item("FP_NAN")
        .blocklist_item("FP_NORMAL")
        .blocklist_item("FP_SUBNORMAL")
        .blocklist_item("FP_ZERO")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from("src");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Unable to write bindings");
}
use std::env;
use std::path::PathBuf;

fn main()
{
    println!("cargo:rustc-link-search=../raylib-5.5_linux_amd64/lib/");
    println!("cargo:rustc-link-lib=static=raylib");
    let bindings = bindgen::Builder::default()
        .header("../../raylib-5.5_linux_amd64/include/raylib.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .blocklist_item("FP_.*")
        .blocklist_type("u128")
        .blocklist_function(".*__.*")
        .generate()
        .expect("Unable to generate Bindings");

    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let out_path = PathBuf::from(&out_dir);

    bindings
        .write_to_file(out_path.join("raylib.rs"))
        .expect("Couldn't write bindings!");

    println!("cargo:out_dir={}", out_dir);
}

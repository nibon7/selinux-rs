use bindgen;
use pkg_config::probe_library;
use std::env;
use std::path::PathBuf;

fn main() {
    probe_library("libselinux")
        .expect("can't find libselinux, please install libselinux-devel or libselinux-dev");
    println!("cargo:return-if-changed=build.rs");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .default_enum_style(bindgen::EnumVariation::ModuleConsts)
        .whitelist_function(".*con")
        .whitelist_function(".*conary")
        .whitelist_function(".*con_raw")
        .whitelist_function(".*selinux.**")
        .whitelist_function("security_.*")
        .whitelist_function(".*matchpathcon.*")
        .whitelist_function(".*context.*")
        .derive_debug(false)
        .derive_eq(true)
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

use bindgen;
use pkg_config::probe_library;
use std::env;
use std::path::PathBuf;

const SELINUX_STATIC: &str = "SELINUX_STATIC";

fn print_library(lib: &pkg_config::Library, mode: &str) {
    for p in &lib.include_paths {
        println!("cargo:include={}", p.display());
    }

    for p in &lib.frameworks {
        println!("cargo:rustc-link-lib=framework={}", p);
    }

    for p in &lib.framework_paths {
        println!("cargo:rustc-link-search=framework={}", p.display());
    }

    for p in &lib.libs {
        println!("cargo:rustc-link-lib={}={}", mode, p);
    }

    for p in &lib.link_paths {
        println!("cargo:rustc-link-search=native={}", p.display());
    }
}

fn try_pkg_config() -> Vec<PathBuf> {
    let libselinux = probe_library("libselinux")
        .expect("can't find libselinux, please install libselinux-devel or libselinux-dev");

    let mode = match env::var_os(SELINUX_STATIC) {
        Some(_) => "static",
        None => "dylib",
    };

    print_library(&libselinux, mode);
    println!("cargo:rustc-link-lib={}=sepol", mode);
    libselinux.include_paths
}

fn main() {
    println!("cargo:rerun-if-env-changed={}", SELINUX_STATIC);
    println!("cargo:return-if-changed=build.rs");

    let include_paths = try_pkg_config();

    let mut builder = bindgen::Builder::default()
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
        .derive_eq(true);

    for p in include_paths {
        builder = builder.clang_arg(format!("-I{}", p.display()));
    }

    let bindings = builder.generate().expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

use bindgen;
use std::env;
use std::path::PathBuf;

const SELINUX_STATIC: &str = "SELINUX_STATIC";

#[cfg(feature = "build-static")]
fn build_static_libselinux() -> Vec<PathBuf> {
    use std::ffi::OsStr;
    use std::process::Command;

    if !PathBuf::from("selinux/.git").exists() {
        Command::new("git")
            .args(&["submodule", "update", "--init"])
            .status()
            .unwrap();
    }

    let output_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    let mut cfg = cc::Build::new();
    for c in glob::glob("selinux/libselinux/src/*.c")
        .unwrap()
        .filter(|p| match p {
            Ok(path) => {
                // filter out audit2why.c and label_backends_androi.c
                path.file_name() != Some(OsStr::new("audit2why.c"))
                    && path.file_name() != Some(OsStr::new("label_backends_android.c"))
            }
            _ => false,
        })
    {
        cfg.file(&c.unwrap());
    }

    cfg.include("selinux/libselinux/include")
        .include("selinux/libsepol/include")
        .define("USE_PCRE2", None)
        .define("PCRE2_CODE_UNIT_WIDTH", Some("8"))
        .define("NO_ANDROID_BACKEND", None)
        .define("_GNU_SOURCE", None)
        .cargo_metadata(true)
        .out_dir(&output_dir)
        .compile("libselinux.a");

    vec![PathBuf::from("selinux/libselinux/include")]
}

#[cfg(not(feature = "build-static"))]
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

#[cfg(not(feature = "build-static"))]
fn try_pkg_config() -> Vec<PathBuf> {
    let libselinux = pkg_config::probe_library("libselinux")
        .expect("can't find libselinux, please install libselinux-devel or libselinux1-dev or try to use build-static feature to build libselinux from scratch.");

    let mode = match env::var_os(SELINUX_STATIC) {
        Some(_) => "static",
        None => "dylib",
    };

    print_library(&libselinux, mode);
    println!("cargo:rustc-link-lib={}=sepol", mode);
    libselinux.include_paths
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    if !cfg!(feature = "build-static") {
        println!("cargo:rerun-if-env-changed={}", SELINUX_STATIC);
    }

    #[cfg(feature = "build-static")]
    let include_paths = build_static_libselinux();
    #[cfg(not(feature = "build-static"))]
    let include_paths = try_pkg_config();

    let mut builder = bindgen::Builder::default()
        .header("wrapper.h")
        .default_enum_style(bindgen::EnumVariation::ModuleConsts)
        .ctypes_prefix("::libc")
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

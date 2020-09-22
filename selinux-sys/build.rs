use bindgen;
use pkg_config::probe_library;
use std::env;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    println!("cargo:return-if-env-changed=SYSTEM_LIBSELINUX");

    if use_system_libselinux() {
        probe_library("libselinux")
            .expect("can't find libselinux, please install libselinux-devel or libselinux-dev");
        println!("cargo:return-if-changed=build.rs");
    }

    if !Path::new("selinux/.git").exists() {
        Command::new("git")
            .args(&["submodule", "update", "--init", "--recursive"])
            .spawn()
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
        .define("NO_ANDROID_BACKEND", None)
        .define("_GNU_SOURCE", None)
        .cargo_metadata(true)
        .out_dir(&output_dir)
        .compile("libselinux.a");

    let bindings = bindgen::Builder::default()
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
        .clang_arg("-Iselinux/libselinux/include")
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

fn use_system_libselinux() -> bool {
    match env::var("SYSTEM_LIBSELINUX") {
        Ok(v) if v == "1" => true,
        _ => false,
    }
}

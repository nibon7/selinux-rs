fn main() {
    pkg_config::find_library("libselinux").expect("libselinux-devel not found");
    println!("cargo:return-if-changed=build.rs");
}

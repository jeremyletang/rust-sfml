#[path="../build_sys_common.rs"] mod build_sys_common;

fn main() {
    println!("cargo:rustc-link-lib={}", build_sys_common::lib_path("graphics"));
}
use std::env::var;

fn lib_path() -> String {
    let lib = if cfg!(target_env = "msvc") {
        "csfml-system"
    } else {
        "libcsfml-system"
    };

    // check if CSFML is installed somewhere local on the machine
    if let Ok(csfml_home) = var("CSFML_HOME") {
        if cfg!(target_env = "msvc") {
            format!("{}/lib/msvc/{}", csfml_home, lib)
        } else {
            format!("{}/lib/gcc/{}", csfml_home, lib)
        }
    } else {
        String::from(lib)
    }
}

fn main() {
    println!("cargo:rustc-link-lib={}", lib_path());
}

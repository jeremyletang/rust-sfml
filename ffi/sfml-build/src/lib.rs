use std::env::var;

// Add search path for CSFML library files
pub fn link_csfml(lib_name: &str) {
    // Figure out the path to libraries within the CSFML base folder
    // based on the operating system
    let lib_path = if std::env::var("CARGO_CFG_TARGET_FAMILY").unwrap() == "windows" {
        if std::env::var("CARGO_CFG_TARGET_ENV").unwrap() == "msvc" {
            "lib/msvc"
        } else {
            "lib/gcc"
        }
    } else {
        "lib"
    };

    // CSFML_HOME points to the base CSFML directory
    // Let cargo find the CSFML library files there
    if let Ok(csfml_home) = var("CSFML_HOME") {
        let search_path = format!("{}/{}", csfml_home, lib_path);
        println!("cargo:warning=CSFML search path is {}", search_path);
        println!("cargo:rustc-link-search=native={}", search_path);
    }

    // Link to the csfml library
    println!("cargo:rustc-link-lib=csfml-{}", lib_name);
}

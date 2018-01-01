use std::env::var;

// Add search path for CSFML library files
pub fn link_csfml(lib_name: &str) {
    // Figure out the path to libraries within the CSFML base folder
    // based on the operating system
    let lib_path = if cfg!(target_family = "windows") {
        if cfg!(target_env = "msvc") {
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
        println!("cargo:rustc-link-search=native={}/{}", csfml_home, lib_path);
    }

    // Link to the csfml library
    println!("cargo:rustc-link-lib=csfml-{}", lib_name);
}

// Add search path for SFML library files
pub fn link_sfml(lib_name: &str) {
    // The windows build of CSFML links SFML libraries statically, so no
    // need to link them to the rust executable as well.
    if var("CARGO_CFG_TARGET_OS").unwrap_or_default() != "windows" {
        // SFML_HOME points to the base SFML directory
        // Let cargo find the SFML library files there
        if let Ok(sfml_home) = var("SFML_HOME") {
            println!("cargo:rustc-link-search=native={}/lib", sfml_home);
        }

        // Link to the sfml library
        println!("cargo:rustc-link-lib=sfml-{}", lib_name);
    }
}

use std::env::var;

// Add a search path for CSFML library files.
pub fn link_csfml(lib: &str) {
    if let Ok(home) = var("CSFML_HOME") {
        let lib = if cfg!(target_family = "windows") {
            if cfg!(target_env = "msvc") {
                "lib/msvc"
            } else {
                "lib/gcc"
            }
        } else {
            "lib"
        };

        // add where to look for the lib
        println!("cargo:rustc-link-search=native={}/{}", home, lib);
    }

    // link it
    println!("cargo:rustc-link-lib=csfml-{}", lib);
}

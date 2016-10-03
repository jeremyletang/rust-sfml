use std::env::var;

// Attempts to discover which library file to use and where it is located.
//
// If the `cfg(target_env)` is set to "MSVC" then it will use the MSVC
// libraries, otherwise it will assume "GNU" and use the GCC libraries.
//
// If the environment variable `CSFML_HOME` is set, then it will look there
// for the libraries, otherwise it defaults to `.`.
//
// It assumes that CSFML is simply unzipped - as is - into a folder, and the
// libraries exist in: `$CSFML_HOME/lib/[msvc|gcc]/*.lib`.
//
pub fn lib_path(file: &str) -> String {
    let lib = if cfg!(target_env = "msvc") {
        format!("csfml-{}", file)
    } else {
        format!("libcsfml-{}", file)
    };

    // check if CSFML is installed somewhere local on the machine
    if let Ok(csfml_home) = var("CSFML_HOME") {
        if cfg!(target_env = "msvc") {
            format!("{}/lib/msvc/{}", csfml_home, lib)
        } else {
            format!("{}/lib/gcc/{}", csfml_home, lib)
        }
    } else {
        lib
    }
}

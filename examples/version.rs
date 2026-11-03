//! Prints the version of the SFML library loaded at runtime. Like clock, it needs only the system module

use sfml::system::version;

fn main() {
    let sfml_version = version();
    println!(
        "SFML {} (major {}, minor {}, patch {}, release: {})",
        sfml_version.as_str(),
        sfml_version.major(),
        sfml_version.minor(),
        sfml_version.patch(),
        sfml_version.is_release()
    );
}

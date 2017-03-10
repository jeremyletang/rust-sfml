extern crate sfml_build;

fn main() {
    sfml_build::link_sfml("system");
    sfml_build::link_csfml("system");
}

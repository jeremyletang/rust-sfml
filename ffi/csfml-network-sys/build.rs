extern crate sfml_build;

fn main() {
    sfml_build::link_sfml("network");
    sfml_build::link_csfml("network");
}

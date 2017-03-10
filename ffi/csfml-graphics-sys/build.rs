extern crate sfml_build;

fn main() {
    sfml_build::link_sfml("graphics");
    sfml_build::link_csfml("graphics");
}

extern crate sfml_build;

fn main() {
    sfml_build::link_sfml("audio");
    sfml_build::link_csfml("audio");
}

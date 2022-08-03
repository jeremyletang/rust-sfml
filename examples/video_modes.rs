use sfml::window::VideoMode;

fn main() {
    let fmodes = VideoMode::fullscreen_modes();
    println!("Available video modes: ");
    for fmode in fmodes {
        println!(
            "{}x{} {} bits",
            fmode.width, fmode.height, fmode.bits_per_pixel
        );
    }
}

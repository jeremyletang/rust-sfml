use sfml::window::joystick;

fn main() {
    for i in 0..joystick::COUNT {
        let ident = joystick::identification(i);
        println!(
            "\
             == Joystick {} ==\n\
             name: {}\n\
             vendor id: {}\n\
             product id: {}\n",
            i,
            ident.name(),
            ident.vendor_id(),
            ident.product_id(),
        );
    }
}

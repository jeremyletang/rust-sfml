/// Sets up working directory so the example works no matter what directory it's run from
fn example_ensure_right_working_dir() {
    std::env::set_current_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/examples/resources")).unwrap();
}

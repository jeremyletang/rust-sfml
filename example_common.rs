/// Macro for loading resources in examples.
///
/// This makes it so that the examples can be run from any directory
macro_rules! example_res {
    ($path:literal) => {
        concat!(env!("CARGO_MANIFEST_DIR"), "/examples/resources/", $path)
    }
}
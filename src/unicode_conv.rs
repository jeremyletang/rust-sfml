/// Convert an `str` to the UTF-32 format used by CSFML.
pub fn str_to_csfml(utf8: &str) -> Vec<char> {
    let mut vec = Vec::with_capacity(utf8.len() + 1);
    for c in utf8.chars() {
        if c == '\0' {
            panic!("Interior null found in string {:?}", utf8);
        }
        vec.push(c);
    }
    vec.push('\0');
    vec
}

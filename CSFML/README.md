This is a modified version of the CSFML library, used internally by rust-sfml.

For the actual CSFML, visit https://github.com/SFML/CSFML

## Design philosophy

- Customized for rust-sfml's needs, rather than general use
- Provide as minimum C++ wrapper code as necessary for ABI compatibility.
  Everything else, like safety checking code should be written in Rust.
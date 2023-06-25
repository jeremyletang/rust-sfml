rust-sfml ![Build Status](https://github.com/jeremyletang/rust-sfml/actions/workflows/rust.yml/badge.svg) [![crates.io](https://img.shields.io/crates/v/sfml.svg)](https://crates.io/crates/sfml) [![Discord](https://img.shields.io/discord/175298431294636032?style=plastic)](https://discord.gg/XCaM5rhMa6)
=========

Rust bindings for [SFML](http://www.sfml-dev.org), the Simple and Fast Multimedia Library.

Requirements
=============

- Linux, Windows, or OS X
- Rust 1.66 or later
- [SFML 2.6](http://www.sfml-dev.org/download.php)
- A C++ compiler for building CSFML

Environment variables
=============
If you get errors about SFML headers not being found, or linker errors, that probably means
SFML is not installed in a global location.
In that case, you can set two environment variables to help rust-sfml find the required files:
- `SFML_INCLUDE_DIR`. Set this to the `include` folder of your SFML location.
- `SFML_LIBS_DIR`. Set this to the `lib` folder of your SFML location.

To help debugging environment variables, you can try building with `cargo build -vv`.
If the environment variables are set correctly, you should get warnings like this:

```
warning: Custom SFML include dir: C:\Users\You\sfml\include
warning: Adding custom SFML libs search path C:\Users\You\sfml\lib
```

Documentation
=====
The API documentation is available at: https://docs.rs/sfml/

If you need help with setting up `rust-sfml` on your system, you can take a look at the [wiki](https://github.com/jeremyletang/rust-sfml/wiki).\
Please take note that:
   * This wiki is supported by the community
   * The `rust-sfml` core team doesn't review it
   * Your contribution is welcome

License
=======

This software is a binding of the SFML library created by Laurent Gomila, which is provided under the Zlib/png license.

This software is provided under the same license as SFML, the Zlib/png license.

Discord
=======
rust-sfml users are welcome on the [Official SFML Discord server](https://discord.gg/XCaM5rhMa6)

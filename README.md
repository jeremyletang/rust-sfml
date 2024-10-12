rust-sfml ![Build Status](https://github.com/jeremyletang/rust-sfml/actions/workflows/rust.yml/badge.svg) [![crates.io](https://img.shields.io/crates/v/sfml.svg)](https://crates.io/crates/sfml) [![Discord](https://img.shields.io/discord/175298431294636032?style=plastic)](https://discord.gg/XCaM5rhMa6)
=========

Rust bindings for [SFML](http://www.sfml-dev.org), the Simple and Fast Multimedia Library.

Platform support
================

|             |                                                   |
|-------------|---------------------------------------------------|
| 🐧 Linux    | ✅ Most tested                                    |
| 🪟 Windows  | ⚠️ Rarely tested. Looking for testers/maintainers! |
| 🍎 Mac OS X | ❌ Untested. Looking for testers/maintainers!     |

Prerequisites
=============

- Rust 1.81 or later
- [CMake](https://cmake.org/), and a C++ toolchain for building SFML
- 🐧 On Linux, you need the following dependencies installed:
   - Window module: `libGL libX11 libXcursor libXrandr`
   - Graphics module: `libfreetype`
   - Audio module: `libopenal libvorbisenc libvorbisfile libvorbis`

Documentation
=============

The API documentation is available at: <https://docs.rs/sfml/>

If you need help with setting up `rust-sfml` on your system, you can take a look at the [wiki](<https://github.com/jeremyletang/rust-sfml/wiki>).\
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

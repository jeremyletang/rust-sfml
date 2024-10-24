rust-sfml
[![crates.io][crates-io-shield]][crates-io-link]
[![docs.rs][docs-rs-shield]][docs-rs-link]
[![Discord][discord-shield]][discord-link]
=========

Rust bindings for [SFML](http://www.sfml-dev.org), the Simple and Fast Multimedia Library.

[crates-io-shield]: https://img.shields.io/crates/v/sfml.svg
[crates-io-link]: https://crates.io/crates/sfml
[docs-rs-shield]: https://img.shields.io/docsrs/sfml
[docs-rs-link]: https://docs.rs/sfml
[discord-shield]: https://img.shields.io/discord/175298431294636032?style=plastic
[discord-link]: https://discord.gg/XCaM5rhMa6

Platform support
================

| Platform             | Development and testing                                  | CI (basic tests)       |
|----------------------|----------------------------------------------------------|------------------------|
| 🐧 Linux             | ✅ Most tested                                            | ![Status][linux-ci]    |
| 🪟 Windows (MSVC)    | ⚠️ Rarely tested. Looking for testers/maintainers!       | ![Status][win-msvc-ci] |
| 🐧 -> 🪟 (mingw-w64) | ⚠️ Cross-compile from Linux to Windows. Somewhat tested. | ❌                      |
| 🍎 Mac OS X          | ❌ Untested. Looking for testers/maintainers!             | ![Status][macos-ci]    |

[linux-ci]: https://github.com/jeremyletang/rust-sfml/actions/workflows/linux.yml/badge.svg
[win-msvc-ci]: https://github.com/jeremyletang/rust-sfml/actions/workflows/windows.yml/badge.svg
[macos-ci]: https://github.com/jeremyletang/rust-sfml/actions/workflows/macos.yml/badge.svg

Prerequisites
=============

- Rust 1.82 or later
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

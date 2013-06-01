rust-sfml
=========

SFML bindings for Rust

This is a Rust binding for SFML, the Simple and Fast Multimedia Library, developped by Laurent Gomila.

SFML website  : www.sfml-dev.org

Rust-sfml is heavily in development, so there is many bugs.

Installation
============

You must install on your computer the SFML2 and CSFML2 libraries who are used for the binding.

SFML2 : http://www.sfml-dev.org/download/sfml/2.0/

CSFML2 : http://www.sfml-dev.org/download/csfml/

Then clone the repo and build the library with the following command.

```Shell
> rustc rsfml.rc
>
```
Finally build your program with the rust-sfml library. For exemple, if the library is in the same path than your program.

```Shell
> rustc main.rs -L .
>
```

Normaly rust-sfml works on Linux and windows.

License
=======

This software is a binding of the SFML library created by Laurent Gomila, which is provide under the Zlib/png license.

<<<<<<< HEAD
This software is provide under the same license than the SFML, the Zlib/png license.
=======
This software is provide under the same license than the SFML, the Zlib/png license.
>>>>>>> 3322f2bb29ab675783e4d3202bc53bc65546404b

/*
* Rust-SFML - Copyright (c) 2013 Letang Jeremy.
*
* The original software, SFML library, is provided by Laurent Gomila.
*
* This software is provided 'as-is', without any express or implied warranty.
* In no event will the authors be held liable for any damages arising from
* the use of this software.
*
* Permission is granted to anyone to use this software for any purpose,
* including commercial applications, and to alter it and redistribute it
* freely, subject to the following restrictions:
*
* 1. The origin of this software must not be misrepresented; you must not claim
*    that you wrote the original software. If you use this software in a product,
*    an acknowledgment in the product documentation would be appreciated but is
*    not required.
*
* 2. Altered source versions must be plainly marked as such, and must not be
*    misrepresented as being the original software.
* 
* 3. This notice may not be removed or altered from any source distribution.
*/

/*!
* Here is a list of all modules :
*/

#[feature(globs)];
#[feature(struct_variant)];

#[link(name = "rsfml",
       vers = "0.2",
       package_id = "github.com/JeremyLetang/rust-sfml",
       author = "letang.jeremy@gmail.com",
       uuid = "4F3334F2-A32B-4460-A63A-9B56C98D1D78",
       url = "http://https://github.com/JeremyLetang/rust-sfml")];

#[desc = "Rust binding for sfml"];
#[license = "Zlib/png"];
#[crate_type = "lib"];


extern mod extra;

pub mod traits;
pub mod system;
pub mod window;
pub mod audio;
pub mod graphics;
pub mod network;
#[doc(hidden)]

#[feature(globs)]
mod sfml_types;
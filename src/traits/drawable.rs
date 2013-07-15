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
* Drawable trait
*
* Implemented by each drawable object to specifiy them type to RenderWindow.
*
*/

use graphics::render_window::RenderWindow;
use graphics::render_texture::RenderTexture;
use graphics::render_states::RenderStates;

/**
* The trait drawable is inherited by each object who can be drown by the RenderWindow
*/
pub trait Drawable {
    pub fn draw_in_render_window(&self, &RenderWindow) -> ();
    pub fn draw_in_render_window_rs(&self, &RenderWindow, &mut RenderStates) -> ();
    pub fn draw_in_render_texture(&self, &RenderTexture) -> ();
}
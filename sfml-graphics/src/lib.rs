// Rust-SFML - Copyright (c) 2013 Letang Jeremy.
//
// The original software, SFML library, is provided by Laurent Gomila.
//
// This software is provided 'as-is', without any express or implied warranty.
// In no event will the authors be held liable for any damages arising from
// the use of this software.
//
// Permission is granted to anyone to use this software for any purpose,
// including commercial applications, and to alter it and redistribute it
// freely, subject to the following restrictions:
//
// 1. The origin of this software must not be misrepresented; you must not claim
//    that you wrote the original software. If you use this software in a product,
//    an acknowledgment in the product documentation would be appreciated but is
//    not required.
//
// 2. Altered source versions must be plainly marked as such, and must not be
//    misrepresented as being the original software.
//
// 3. This notice may not be removed or altered from any source distribution.
//

//! 2D graphics module: sprites, text, shapes..

extern crate sfml_system;
extern crate sfml_window;
extern crate csfml_system_sys;
extern crate csfml_window_sys;
extern crate csfml_graphics_sys;
#[macro_use]
extern crate bitflags;

pub use render_target::RenderTarget;
pub use render_states::RenderStates;
pub use render_window::{RenderWindow, Events};
pub use rect::{Rect, FloatRect, IntRect};
pub use texture::{Texture, TextureRef};
pub use blend_mode::BlendMode;
pub use transform::Transform;
pub use text::Text;
pub use shader::Shader;
pub use color::Color;
pub use font::Font;
pub use view::{View, ViewRef};
pub use image::Image;
pub use sprite::Sprite;
pub use circle_shape::CircleShape;
pub use rectangle_shape::RectangleShape;
pub use convex_shape::{ConvexShape, ConvexShapePoints};
pub use primitive_type::*;
pub use vertex::Vertex;
pub use glyph::Glyph;
pub use render_texture::RenderTexture;
pub use custom_shape::CustomShape;
pub use vertex_array::{VertexArray, Vertices};
pub use text_style::TextStyle;
pub use drawable::Drawable;
pub use shape::{Shape, ShapeImpl};
pub use transformable::Transformable;

mod drawable;
mod shape;
mod transformable;
mod render_target;
mod render_states;
mod render_window;
mod texture;
pub mod blend_mode;
mod transform;
mod text;
pub mod text_style;
mod shader;
mod color;
pub mod font;
mod view;
mod image;
mod sprite;
mod circle_shape;
mod rectangle_shape;
mod convex_shape;
mod primitive_type;
mod vertex;
mod vertex_array;
mod render_texture;
mod custom_shape;
mod rect;
mod glyph;
#[path="../../src/inputstream.rs"]
mod inputstream;
#[path="../../src/unicode_conv.rs"]
mod unicode_conv;
mod event_ext {
    use csfml_window_sys::*;
    use sfml_window::Event;
    use sfml_system::SfBoolExt;
    use sfml_system::raw_conv::FromRaw;

    pub fn get_wrapped_event(event: &mut sfEvent) -> Option<Event> {
        use csfml_window_sys::sfEventType::*;
        use sfml_window::Event::*;

        let type_ = unsafe { *event.type_.as_ref() };

        Some(match type_ {
            sfEvtClosed => Closed,
            sfEvtResized => {
                let e = unsafe { *event.size.as_ref() };

                Resized {
                    width: e.width,
                    height: e.height,
                }
            }
            sfEvtLostFocus => LostFocus,
            sfEvtGainedFocus => GainedFocus,
            sfEvtTextEntered => {
                TextEntered {
                    unicode: unsafe {
                        ::std::char::from_u32((*event.text.as_ref()).unicode)
                            .expect("Invalid unicode encountered on TextEntered event")
                    },
                }
            }
            sfEvtKeyPressed => {
                let e = unsafe { event.key.as_ref() };

                KeyPressed {
                    code: unsafe { ::std::mem::transmute(e.code) },
                    alt: e.alt.to_bool(),
                    ctrl: e.control.to_bool(),
                    shift: e.shift.to_bool(),
                    system: e.system.to_bool(),
                }
            }
            sfEvtKeyReleased => {
                let e = unsafe { event.key.as_ref() };

                KeyReleased {
                    code: unsafe { ::std::mem::transmute(e.code) },
                    alt: e.alt.to_bool(),
                    ctrl: e.control.to_bool(),
                    shift: e.shift.to_bool(),
                    system: e.system.to_bool(),
                }
            }
            sfEvtMouseWheelScrolled => {
                let e = unsafe { event.mouseWheelScroll.as_ref() };
                MouseWheelScrolled {
                    wheel: unsafe { FromRaw::from_raw(e.wheel) },
                    delta: e.delta,
                    x: e.x,
                    y: e.y,
                }
            }
            sfEvtMouseButtonPressed => {
                let e = unsafe { event.mouseButton.as_ref() };

                MouseButtonPressed {
                    button: unsafe { FromRaw::from_raw(e.button) },
                    x: e.x,
                    y: e.y,
                }
            }
            sfEvtMouseButtonReleased => {
                let e = unsafe { event.mouseButton.as_ref() };

                MouseButtonReleased {
                    button: unsafe { FromRaw::from_raw(e.button) },
                    x: e.x,
                    y: e.y,
                }
            }
            sfEvtMouseMoved => {
                let e = unsafe { event.mouseMove.as_ref() };
                MouseMoved { x: e.x, y: e.y }
            }
            sfEvtMouseEntered => MouseEntered,
            sfEvtMouseLeft => MouseLeft,
            sfEvtJoystickButtonPressed => {
                let e = unsafe { event.joystickButton.as_ref() };

                JoystickButtonPressed {
                    joystickid: (*e).joystickId,
                    button: (*e).button,
                }
            }
            sfEvtJoystickButtonReleased => {
                let e = unsafe { event.joystickButton.as_ref() };

                JoystickButtonReleased {
                    joystickid: (*e).joystickId,
                    button: (*e).button,
                }
            }
            sfEvtJoystickMoved => {
                let e = unsafe { event.joystickMove.as_ref() };

                JoystickMoved {
                    joystickid: e.joystickId,
                    axis: unsafe { FromRaw::from_raw(e.axis) },
                    position: e.position,
                }
            }
            sfEvtJoystickConnected => {
                JoystickConnected {
                    joystickid: unsafe { (*event.joystickConnect.as_ref()).joystickId },
                }
            }
            sfEvtJoystickDisconnected => {
                JoystickDisconnected {
                    joystickid: unsafe { (*event.joystickConnect.as_ref()).joystickId },
                }
            }
            sfEvtTouchBegan => {
                let e = unsafe { event.touch.as_ref() };

                TouchBegan {
                    finger: e.finger,
                    x: e.x,
                    y: e.y,
                }
            }
            sfEvtTouchMoved => {
                let e = unsafe { event.touch.as_ref() };

                TouchMoved {
                    finger: e.finger,
                    x: e.x,
                    y: e.y,
                }
            }
            sfEvtTouchEnded => {
                let e = unsafe { event.touch.as_ref() };

                TouchEnded {
                    finger: e.finger,
                    x: e.x,
                    y: e.y,
                }
            }
            sfEvtSensorChanged => {
                let e = unsafe { event.sensor.as_ref() };

                SensorChanged {
                    type_: unsafe { FromRaw::from_raw(e.sensorType) },
                    x: e.x,
                    y: e.y,
                    z: e.z,
                }
            }

            // Ignore deprecated events
            sfEvtMouseWheelMoved => return None,
            sfEvtCount => unreachable!(),
        })
    }

}

use csfml_window_sys::*;
use window::Event;
use ext::sf_bool_ext::SfBoolExt;
use raw_conv::FromRaw;

pub fn get_wrapped_event(event: &mut sfEvent) -> Option<Event> {
    use csfml_window_sys::sfEventType::*;
    use window::Event::*;

    let type_ = unsafe { *event.type_() };

    Some(match type_ {
        sfEvtClosed => Closed,
        sfEvtResized => {
            let e = unsafe { &*event.size() };

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
                    ::std::char::from_u32((*event.text()).unicode)
                        .expect("Invalid unicode encountered on TextEntered event")
                },
            }
        }
        sfEvtKeyPressed => {
            let e = unsafe { &*event.key() };

            KeyPressed {
                code: unsafe { ::std::mem::transmute(e.code) },
                alt: e.alt.to_bool(),
                ctrl: e.control.to_bool(),
                shift: e.shift.to_bool(),
                system: e.system.to_bool(),
            }
        }
        sfEvtKeyReleased => {
            let e = unsafe { &*event.key() };

            KeyReleased {
                code: unsafe { ::std::mem::transmute(e.code) },
                alt: e.alt.to_bool(),
                ctrl: e.control.to_bool(),
                shift: e.shift.to_bool(),
                system: e.system.to_bool(),
            }
        }
        sfEvtMouseWheelScrolled => {
            let e = unsafe { &*event.mouseWheelScroll() };
            MouseWheelScrolled {
                wheel: FromRaw::from_raw(e.wheel),
                delta: e.delta,
                x: e.x,
                y: e.y,
            }
        }
        sfEvtMouseButtonPressed => {
            let e = unsafe { &*event.mouseButton() };

            MouseButtonPressed {
                button: FromRaw::from_raw(e.button),
                x: e.x,
                y: e.y,
            }
        }
        sfEvtMouseButtonReleased => {
            let e = unsafe { &*event.mouseButton() };

            MouseButtonReleased {
                button: FromRaw::from_raw(e.button),
                x: e.x,
                y: e.y,
            }
        }
        sfEvtMouseMoved => {
            let e = unsafe { &*event.mouseMove() };
            MouseMoved { x: e.x, y: e.y }
        }
        sfEvtMouseEntered => MouseEntered,
        sfEvtMouseLeft => MouseLeft,
        sfEvtJoystickButtonPressed => {
            let e = unsafe { &*event.joystickButton() };

            JoystickButtonPressed {
                joystickid: (*e).joystickId,
                button: (*e).button,
            }
        }
        sfEvtJoystickButtonReleased => {
            let e = unsafe { &*event.joystickButton() };

            JoystickButtonReleased {
                joystickid: (*e).joystickId,
                button: (*e).button,
            }
        }
        sfEvtJoystickMoved => {
            let e = unsafe { &*event.joystickMove() };

            JoystickMoved {
                joystickid: e.joystickId,
                axis: FromRaw::from_raw(e.axis),
                position: e.position,
            }
        }
        sfEvtJoystickConnected => {
            JoystickConnected { joystickid: unsafe { (*event.joystickConnect()).joystickId } }
        }
        sfEvtJoystickDisconnected => {
            JoystickDisconnected { joystickid: unsafe { (*event.joystickConnect()).joystickId } }
        }
        // Ignore deprecated events
        sfEvtMouseWheelMoved => return None,
        // Events not yet implemented
        sfEvtTouchBegan | sfEvtTouchMoved | sfEvtTouchEnded | sfEvtSensorChanged => unimplemented!(),
        sfEvtCount => unreachable!(),
    })
}

use csfml_window_sys::*;
use window::Event;
use ext::sf_bool_ext::SfBoolExt;
use raw_conv::FromRaw;

pub fn get_wrapped_event(event: &mut sfEvent) -> Option<Event> {
    use csfml_window_sys::sfEventType::*;
    use window::Event::*;

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
                wheel: FromRaw::from_raw(e.wheel),
                delta: e.delta,
                x: e.x,
                y: e.y,
            }
        }
        sfEvtMouseButtonPressed => {
            let e = unsafe { event.mouseButton.as_ref() };

            MouseButtonPressed {
                button: FromRaw::from_raw(e.button),
                x: e.x,
                y: e.y,
            }
        }
        sfEvtMouseButtonReleased => {
            let e = unsafe { event.mouseButton.as_ref() };

            MouseButtonReleased {
                button: FromRaw::from_raw(e.button),
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
                axis: FromRaw::from_raw(e.axis),
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
                type_: FromRaw::from_raw(e.sensorType),
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

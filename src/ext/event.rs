use csfml_window_sys::*;
use window::Event;
use ext::sf_bool_ext::SfBoolExt;
use raw_conv::FromRaw;

fn type_(evt: &mut sfEvent) -> *mut sfEventType {
    evt as *mut _ as *mut _
}

fn size(evt: &mut sfEvent) -> Event {
    let e: *mut sfSizeEvent = evt as *mut _ as *mut _;
    unsafe {
        Event::Resized {
            width: (*e).width,
            height: (*e).height,
        }
    }
}

fn key(evt: &mut sfEvent, type_: sfEventType) -> Event {
    let e: *mut sfKeyEvent = evt as *mut _ as *mut _;
    let code = unsafe { ::std::mem::transmute((*e).code) };
    let alt = unsafe { (*e).alt.to_bool() };
    let ctrl = unsafe { (*e).control.to_bool() };
    let shift = unsafe { (*e).shift.to_bool() };
    let system = unsafe { (*e).system.to_bool() };
    match type_ {
        sfEventType::sfEvtKeyPressed => {
            Event::KeyPressed {
                code: code,
                alt: alt,
                ctrl: ctrl,
                shift: shift,
                system: system,
            }
        }
        sfEventType::sfEvtKeyReleased => {
            Event::KeyReleased {
                code: code,
                alt: alt,
                ctrl: ctrl,
                shift: shift,
                system: system,
            }
        }
        _ => unreachable!(),
    }
}

fn text(evt: &mut sfEvent) -> Event {
    let e: *mut sfTextEvent = evt as *mut _ as *mut _;
    unsafe {
        Event::TextEntered {
            unicode: ::std::char::from_u32((*e).unicode)
                .expect("Invalid unicode encountered on TextEntered event"),
        }
    }
}

fn mouse_move(evt: &mut sfEvent) -> Event {
    let e: *mut sfMouseMoveEvent = evt as *mut _ as *mut _;
    unsafe {
        Event::MouseMoved {
            x: (*e).x,
            y: (*e).y,
        }
    }
}

fn mouse_button(evt: &mut sfEvent, type_: sfEventType) -> Event {
    let e: *mut sfMouseButtonEvent = evt as *mut _ as *mut _;
    let button = unsafe { ::std::mem::transmute((*e).button as u8) };
    let x = unsafe { (*e).x };
    let y = unsafe { (*e).y };

    match type_ {
        sfEventType::sfEvtMouseButtonReleased => {
            Event::MouseButtonReleased {
                button: button,
                x: x,
                y: y,
            }
        }
        sfEventType::sfEvtMouseButtonPressed => {
            Event::MouseButtonPressed {
                button: button,
                x: x,
                y: y,
            }
        }
        _ => unreachable!(),
    }
}

fn mouse_wheel(evt: &mut sfEvent) -> Event {
    let e = unsafe { &mut *evt.mouseWheelScroll() };
    Event::MouseWheelScrolled {
        wheel: FromRaw::from_raw(e.wheel),
        delta: e.delta,
        x: e.x,
        y: e.y,
    }
}

fn joystick_move(evt: &mut sfEvent) -> Event {
    let e: *mut sfJoystickMoveEvent = evt as *mut _ as *mut _;
    Event::JoystickMoved {
        joystickid: unsafe { (*e).joystickId },
        axis: unsafe { ::std::mem::transmute((*e).axis as u8) },
        position: unsafe { (*e).position },
    }
}

fn joystick_button(evt: &mut sfEvent, type_: sfEventType) -> Event {
    let e: *mut sfJoystickButtonEvent = evt as *mut _ as *mut _;
    let jid = unsafe { (*e).joystickId };
    let btn = unsafe { (*e).button };

    match type_ {
        sfEventType::sfEvtJoystickButtonPressed => {
            Event::JoystickButtonPressed {
                joystickid: jid,
                button: btn,
            }
        }
        sfEventType::sfEvtJoystickButtonReleased => {
            Event::JoystickButtonReleased {
                joystickid: jid,
                button: btn,
            }
        }
        _ => unreachable!(),
    }
}

fn joystick_connect(evt: &mut sfEvent, type_: sfEventType) -> Event {
    let e: *mut sfJoystickConnectEvent = evt as *mut _ as *mut _;
    let jid = unsafe { (*e).joystickId };

    match type_ {
        sfEventType::sfEvtJoystickConnected => Event::JoystickConnected { joystickid: jid },
        sfEventType::sfEvtJoystickDisconnected => Event::JoystickDisconnected { joystickid: jid },
        _ => unreachable!(),
    }
}

pub fn get_wrapped_event(event: &mut ::csfml_window_sys::sfEvent) -> Option<Event> {
    let type_ = unsafe { *type_(event) };

    Some(match type_ {
        sfEventType::sfEvtClosed => Event::Closed,
        sfEventType::sfEvtResized => size(event),
        sfEventType::sfEvtLostFocus => Event::LostFocus,
        sfEventType::sfEvtGainedFocus => Event::GainedFocus,
        sfEventType::sfEvtTextEntered => text(event),
        sfEventType::sfEvtKeyPressed |
        sfEventType::sfEvtKeyReleased => key(event, type_),
        sfEventType::sfEvtMouseWheelScrolled => mouse_wheel(event),
        sfEventType::sfEvtMouseButtonPressed |
        sfEventType::sfEvtMouseButtonReleased => mouse_button(event, type_),
        sfEventType::sfEvtMouseMoved => mouse_move(event),
        sfEventType::sfEvtMouseEntered => Event::MouseEntered,
        sfEventType::sfEvtMouseLeft => Event::MouseLeft,
        sfEventType::sfEvtJoystickButtonPressed |
        sfEventType::sfEvtJoystickButtonReleased => joystick_button(event, type_),
        sfEventType::sfEvtJoystickMoved => joystick_move(event),
        sfEventType::sfEvtJoystickConnected |
        sfEventType::sfEvtJoystickDisconnected => joystick_connect(event, type_),
        _ => return None,
    })
}

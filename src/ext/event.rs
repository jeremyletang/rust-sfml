use csfml_window_sys as sys;
use csfml_window_sys::*;
use window::event;

fn type_(evt: &mut sfEvent) -> *mut sfEventType {
    evt as *mut _ as _
}

fn size(evt: &mut sfEvent) -> event::Event {
    let e: *mut sfSizeEvent = evt as *mut _ as _;
    unsafe { event::Resized { width: (*e).width, height: (*e).height } }
}

fn key(evt: &mut sfEvent, type_: sfEventType) -> event::Event {
    let e: *mut sfKeyEvent = evt as *mut _ as _;
    let code = unsafe { ::std::mem::transmute((*e).code as i64) };
    let alt = unsafe { (*e).alt.to_bool() };
    let ctrl = unsafe { (*e).control.to_bool() };
    let shift = unsafe { (*e).shift.to_bool() };
    let system = unsafe { (*e).system.to_bool() };
    match type_ {
        sys::sfEvtKeyPressed => {
            event::KeyPressed {
                code: code,
                alt: alt,
                ctrl: ctrl,
                shift: shift,
                system: system
            }
        },
        sys::sfEvtKeyReleased => {
            event::KeyReleased {
                code: code,
                alt: alt,
                ctrl: ctrl,
                shift: shift,
                system: system
            }
        },
        _ => unreachable!()
    }
}

fn text(evt: &mut sfEvent) -> event::Event {
    let e: *mut sfTextEvent = evt as *mut _ as _;
    unsafe { event::TextEntered { code: ((*e).unicode as u8) as char } }
}

fn mouse_move(evt: &mut sfEvent) -> event::Event {
    let e: *mut sfMouseMoveEvent = evt as *mut _ as _;
    unsafe { event::MouseMoved {x: (*e).x, y: (*e).y } }
}

fn mouse_button(evt: &mut sfEvent, type_: sfEventType) -> event::Event {
    let e: *mut sfMouseButtonEvent = evt as *mut _ as _;
    let button = unsafe { ::std::mem::transmute((*e).button as u8) };
    let x = unsafe { (*e).x };
    let y = unsafe { (*e).y };

    match type_ {
        sys::sfEvtMouseButtonReleased => event::MouseButtonReleased { button: button, x: x, y: y },
        sys::sfEvtMouseButtonPressed => event::MouseButtonPressed { button: button, x: x, y: y },
        _ => unreachable!()
    }
}

fn mouse_wheel(evt: &mut sfEvent) -> event::Event {
    let e: *mut sfMouseWheelEvent = evt as *mut _ as _;
    unsafe { event::MouseWheelMoved { delta: (*e).delta, x: (*e).x, y: (*e).y } }
}

fn joystick_move(evt: &mut sfEvent) -> event::Event {
    let e: *mut sfJoystickMoveEvent = evt as *mut _ as _;
    event::JoystickMoved {
        joystickid: unsafe { (*e).joystickid },
        axis: unsafe { ::std::mem::transmute((*e).axis as u8) },
        position: unsafe { (*e).position }
    }
}

fn joystick_button(evt: &mut sfEvent, type_: sfEventType) -> event::Event {
    let e: *mut sfJoystickButtonEvent = evt as *mut _ as _;
    let jid = unsafe { (*e).joystickid };
    let btn = unsafe { (*e).button };

    match type_ {
        sys::sfEvtJoystickButtonPressed =>
            event::JoystickButtonPressed { joystickid: jid, button: btn },
        sys::sfEvtJoystickButtonReleased =>
            event::JoystickButtonReleased { joystickid: jid, button: btn },
        _ => unreachable!()
    }
}

fn joystick_connect(evt: &mut sfEvent, type_: sfEventType) -> event::Event {
    let e: *mut sfJoystickConnectEvent = evt as *mut _ as _;
    let jid = unsafe { (*e).joystickid };

    match type_ {
        sys::sfEvtJoystickConnected => event::JoystickConnected { joystickid: jid },
        sys::sfEvtJoystickDisconnected => event::JoystickDisconnected { joystickid: jid},
        _ => unreachable!()
    }
}

pub fn get_wrapped_event(event: &mut ::csfml_window_sys::sfEvent) -> event::Event {
    let type_ = unsafe { *type_(event) };

    match type_ {
        sys::sfEvtClosed => event::Closed,
        sys::sfEvtResized => size(event),
        sys::sfEvtLostFocus => event::LostFocus,
        sys::sfEvtGainedFocus => event::GainedFocus,
        sys::sfEvtTextEntered => text(event),
        sys::sfEvtKeyPressed | sys::sfEvtKeyReleased  => key(event, type_),
        sys::sfEvtMouseWheelMoved => mouse_wheel(event),
        sys::sfEvtMouseButtonPressed |
        sys::sfEvtMouseButtonReleased => mouse_button(event, type_),
        sys::sfEvtMouseMoved => mouse_move(event),
        sys::sfEvtMouseEntered => event::MouseEntered,
        sys::sfEvtMouseLeft => event::MouseLeft,
        sys::sfEvtJoystickButtonPressed |
        sys::sfEvtJoystickButtonReleased => joystick_button(event, type_),
        sys::sfEvtJoystickMoved => joystick_move(event),
        sys::sfEvtJoystickConnected |
        sys::sfEvtJoystickDisconnected => joystick_connect(event, type_),
        _ => event::NoEvent
    }
}

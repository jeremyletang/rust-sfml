use csfml_window_sys::*;
use window::event;
use ext::sf_bool_ext::SfBoolExt;

fn type_(evt: &mut sfEvent) -> *mut sfEventType {
    evt as *mut _ as *mut _
}

fn size(evt: &mut sfEvent) -> event::Event {
    let e: *mut sfSizeEvent = evt as *mut _ as *mut _;
    unsafe {
        event::Resized {
            width: (*e).width,
            height: (*e).height,
        }
    }
}

fn key(evt: &mut sfEvent, type_: sfEventType) -> event::Event {
    let e: *mut sfKeyEvent = evt as *mut _ as *mut _;
    let code = unsafe { ::std::mem::transmute((*e).code) };
    let alt = unsafe { (*e).alt.to_bool() };
    let ctrl = unsafe { (*e).control.to_bool() };
    let shift = unsafe { (*e).shift.to_bool() };
    let system = unsafe { (*e).system.to_bool() };
    match type_ {
        sfEventType::sfEvtKeyPressed => {
            event::KeyPressed {
                code: code,
                alt: alt,
                ctrl: ctrl,
                shift: shift,
                system: system,
            }
        }
        sfEventType::sfEvtKeyReleased => {
            event::KeyReleased {
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

fn text(evt: &mut sfEvent) -> event::Event {
    let e: *mut sfTextEvent = evt as *mut _ as *mut _;
    unsafe { event::TextEntered { code: ((*e).unicode as u8) as char } }
}

fn mouse_move(evt: &mut sfEvent) -> event::Event {
    let e: *mut sfMouseMoveEvent = evt as *mut _ as *mut _;
    unsafe {
        event::MouseMoved {
            x: (*e).x,
            y: (*e).y,
        }
    }
}

fn mouse_button(evt: &mut sfEvent, type_: sfEventType) -> event::Event {
    let e: *mut sfMouseButtonEvent = evt as *mut _ as *mut _;
    let button = unsafe { ::std::mem::transmute((*e).button as u8) };
    let x = unsafe { (*e).x };
    let y = unsafe { (*e).y };

    match type_ {
        sfEventType::sfEvtMouseButtonReleased => {
            event::MouseButtonReleased {
                button: button,
                x: x,
                y: y,
            }
        }
        sfEventType::sfEvtMouseButtonPressed => {
            event::MouseButtonPressed {
                button: button,
                x: x,
                y: y,
            }
        }
        _ => unreachable!(),
    }
}

fn mouse_wheel(evt: &mut sfEvent) -> event::Event {
    let e: *mut sfMouseWheelEvent = evt as *mut _ as *mut _;
    unsafe {
        event::MouseWheelMoved {
            delta: (*e).delta,
            x: (*e).x,
            y: (*e).y,
        }
    }
}

fn joystick_move(evt: &mut sfEvent) -> event::Event {
    let e: *mut sfJoystickMoveEvent = evt as *mut _ as *mut _;
    event::JoystickMoved {
        joystickid: unsafe { (*e).joystickId },
        axis: unsafe { ::std::mem::transmute((*e).axis as u8) },
        position: unsafe { (*e).position },
    }
}

fn joystick_button(evt: &mut sfEvent, type_: sfEventType) -> event::Event {
    let e: *mut sfJoystickButtonEvent = evt as *mut _ as *mut _;
    let jid = unsafe { (*e).joystickId };
    let btn = unsafe { (*e).button };

    match type_ {
        sfEventType::sfEvtJoystickButtonPressed => {
            event::JoystickButtonPressed {
                joystickid: jid,
                button: btn,
            }
        }
        sfEventType::sfEvtJoystickButtonReleased => {
            event::JoystickButtonReleased {
                joystickid: jid,
                button: btn,
            }
        }
        _ => unreachable!(),
    }
}

fn joystick_connect(evt: &mut sfEvent, type_: sfEventType) -> event::Event {
    let e: *mut sfJoystickConnectEvent = evt as *mut _ as *mut _;
    let jid = unsafe { (*e).joystickId };

    match type_ {
        sfEventType::sfEvtJoystickConnected => event::JoystickConnected { joystickid: jid },
        sfEventType::sfEvtJoystickDisconnected => event::JoystickDisconnected { joystickid: jid },
        _ => unreachable!(),
    }
}

pub fn get_wrapped_event(event: &mut ::csfml_window_sys::sfEvent) -> event::Event {
    let type_ = unsafe { *type_(event) };

    match type_ {
        sfEventType::sfEvtClosed => event::Closed,
        sfEventType::sfEvtResized => size(event),
        sfEventType::sfEvtLostFocus => event::LostFocus,
        sfEventType::sfEvtGainedFocus => event::GainedFocus,
        sfEventType::sfEvtTextEntered => text(event),
        sfEventType::sfEvtKeyPressed |
        sfEventType::sfEvtKeyReleased => key(event, type_),
        sfEventType::sfEvtMouseWheelMoved => mouse_wheel(event),
        sfEventType::sfEvtMouseButtonPressed |
        sfEventType::sfEvtMouseButtonReleased => mouse_button(event, type_),
        sfEventType::sfEvtMouseMoved => mouse_move(event),
        sfEventType::sfEvtMouseEntered => event::MouseEntered,
        sfEventType::sfEvtMouseLeft => event::MouseLeft,
        sfEventType::sfEvtJoystickButtonPressed |
        sfEventType::sfEvtJoystickButtonReleased => joystick_button(event, type_),
        sfEventType::sfEvtJoystickMoved => joystick_move(event),
        sfEventType::sfEvtJoystickConnected |
        sfEventType::sfEvtJoystickDisconnected => joystick_connect(event, type_),
        _ => event::NoEvent,
    }
}

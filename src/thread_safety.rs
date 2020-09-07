use once_cell::sync::OnceCell;
use std::thread::{current, ThreadId};

// this WINDOW_THREAD is the only thread allowed to contain RenderWindows.
static WINDOW_THREAD: OnceCell<ThreadId> = OnceCell::new();

// sets WINDOW_THREAD to the current thread.
// panics if WINDOW_THREAD is already assigned to another thread.
pub(super) fn set_window_thread() {
    let current_id = current().id();
    match WINDOW_THREAD.get() {
        None => WINDOW_THREAD.set(current_id).unwrap(),
        Some(id) => {
            if *id != current_id {
                panic!("Opening Windows in multiple threads is not supported.")
            }
        }
    }
}

// asserts that the either current thread is WINDOW_THREAD, or that WINDOW_THREAD is unset
pub(super) fn assert_window_thread(func_name: &str) {
    if let Some(&id) = WINDOW_THREAD.get() {
        if id != current().id() {
            panic!(
                "{} has to be called in the same thread as the RenderWindow!",
                func_name
            );
        }
    }
}

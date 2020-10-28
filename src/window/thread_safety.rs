use once_cell::sync::OnceCell;
use std::thread::{current, ThreadId};

// this WINDOW_THREAD is the only thread allowed to contain RenderWindows.
static WINDOW_THREAD: OnceCell<ThreadId> = OnceCell::new();

// sets WINDOW_THREAD to the current thread.
// panics if WINDOW_THREAD is already assigned to another thread.
pub(crate) fn set_window_thread() {
    let current_id = current().id();
    match WINDOW_THREAD.get() {
        None => WINDOW_THREAD.set(current_id).unwrap(),
        Some(id) => {
            if *id != current_id {
                panic!("Graphics/windowing functionality is only supported on the same thread.");
            }
        }
    }
}

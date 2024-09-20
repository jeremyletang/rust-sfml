use std::{
    sync::OnceLock,
    thread::{current, ThreadId},
};

// this WINDOW_THREAD is the only thread allowed to contain RenderWindows.
static WINDOW_THREAD: OnceLock<ThreadId> = OnceLock::new();

// sets WINDOW_THREAD to the current thread.
// panics if WINDOW_THREAD is already assigned to another thread.
pub(crate) fn set_window_thread() {
    let current_id = current().id();
    #[expect(clippy::unwrap_used)]
    match WINDOW_THREAD.get() {
        None => WINDOW_THREAD.set(current_id).unwrap(),
        Some(id) => {
            assert!(
                *id == current_id,
                "Graphics/windowing functionality is only supported on the same thread."
            );
        }
    }
}

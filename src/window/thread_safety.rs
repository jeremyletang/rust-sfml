use std::{
    sync::OnceLock,
    thread::{ThreadId, current},
};

// this WINDOW_THREAD is the only thread allowed to contain RenderWindows.
static WINDOW_THREAD: OnceLock<ThreadId> = OnceLock::new();

// sets WINDOW_THREAD to the current thread.
// panics if WINDOW_THREAD is already assigned to another thread.
pub(crate) fn set_window_thread() {
    pin_thread(&WINDOW_THREAD, current().id());
}

// pins `slot` to `current` if it is unset.
// panics if `slot` is already pinned to another thread.
fn pin_thread(slot: &OnceLock<ThreadId>, current: ThreadId) {
    let pinned = slot.get_or_init(|| current);
    assert!(
        *pinned == current,
        "Graphics/windowing functionality is only supported on the same thread."
    );
}

#[cfg(test)]
mod tests {
    use super::pin_thread;
    use std::{sync::OnceLock, thread::current};

    #[test]
    fn first_call_pins_current_thread() {
        let slot = OnceLock::new();
        pin_thread(&slot, current().id());
        assert_eq!(slot.get(), Some(&current().id()));
    }

    #[test]
    fn same_thread_is_accepted() {
        let slot = OnceLock::new();
        pin_thread(&slot, current().id());
        pin_thread(&slot, current().id());
    }

    #[test]
    fn other_thread_panics() {
        let slot = OnceLock::new();
        pin_thread(&slot, current().id());
        let result = std::thread::scope(|s| s.spawn(|| pin_thread(&slot, current().id())).join());
        assert!(result.is_err());
    }
}

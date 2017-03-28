use csfml_window_sys as ffi;
use ext::sf_bool_ext::SfBoolExt;
use window::ContextSettings;
use system::raw_conv::FromRaw;

/// Type holding a valid drawing context.
///
/// If you need to make OpenGL calls without having an active window (like in a thread),
/// you can use an instance of this type to get a valid context.
///
/// Having a valid context is necessary for every OpenGL call.
///
/// Note that a context is only active in its current thread,
/// if you create a new thread it will have no valid context by default.
///
/// To use a `Context` instance, just construct it and let it live as long as you need
/// a valid context. No explicit activation is needed, all it has to do is to exist.
/// Its destructor will take care of deactivating and freeing all the attached resources.
///
/// # Usage example
/// ```
/// # use sfml::window::Context;
/// # use std::thread;
/// thread::spawn(|| {
///     let context = Context::new();
///     // from now on, you have a valid context
///
///     // you can make OpenGL calls, e.g. glClear(GL_DEPTH_BUFFER_BIT);
/// }).join().unwrap();
/// // the context is automatically deactivated and destroyed
/// // by the `Context` destructor
/// ```
pub struct Context(*mut ffi::sfContext);

impl Context {
    /// Creates and activates a new context.
    pub fn new() -> Context {
        Context(unsafe { ffi::sfContext_create() })
    }

    /// Explicitly activates or deactivates the context.
    ///
    /// # Arguments
    /// * active - `true` to activate, `false` to deactivate
    ///
    /// Returns true on success, false on failure.
    pub fn set_active(&mut self, active: bool) -> bool {
        let result = unsafe { ffi::sfContext_setActive(self.0, SfBoolExt::from_bool(active)) };
        result.to_bool()
    }
    /// Get the settings of the context.
    ///
    /// Note that these settings may be different than the ones passed to the constructor;
    /// they are indeed adjusted if the original settings are not directly supported by the system.
    pub fn settings(&self) -> ContextSettings {
        let settings = unsafe { ffi::sfContext_getSettings(self.0) };
        unsafe { ContextSettings::from_raw(settings) }
    }
}

#[test]
fn test_settings() {
    use window::{VideoMode, Window, Context};
    use std::thread;
    let video_mode = VideoMode::new(32, 32, 32);

    let window = Window::new(video_mode, "test", Default::default(), &Default::default());
    let win_settings = window.settings();
    thread::spawn(move || {
                      let context = Context::new();
                      assert_eq!(context.settings(), win_settings);
                  })
            .join()
            .unwrap();
}

impl Drop for Context {
    /// Deactivates and destroys the context.
    fn drop(&mut self) {
        unsafe {
            ffi::sfContext_destroy(self.0);
        }
    }
}

impl Default for Context {
    fn default() -> Self {
        Context::new()
    }
}

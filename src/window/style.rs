//! Available styles applicable to windows.

bitflags::bitflags! {
    /// Available styles applicable to windows.
    #[repr(C)]
    #[derive(Clone, Copy, Debug)]
    pub struct Style: u32 {
        /// No decorations (cannot be combined with other flags).
        const NONE = 0;
        /// Title bar and fixed border.
        const TITLEBAR = 1;
        /// Title bar, resizable border, and maximize button.
        const RESIZE = 2;
        /// Title bar and close button.
        const CLOSE = 4;
        /// Fullscreen mode (ignores other flags).
        const FULLSCREEN = 8;
        /// Default window style: title bar, resizable border, and close button.
        const DEFAULT = 1 | 2 | 4;
    }
}

impl Default for Style {
    fn default() -> Self {
        Self::DEFAULT
    }
}

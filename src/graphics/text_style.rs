//! Available text styles.

bitflags! {
    /// Available text styles.
    #[repr(C)]
    pub struct TextStyle: u32 {
        /// Regular characters, no style.
        const REGULAR = 0;
        /// Bold characters.
        const BOLD = 1;
        /// Italic characters.
        const ITALIC = 2;
        /// Underlined characters.
        const UNDERLINED = 4;
    }
}

impl Default for TextStyle {
    fn default() -> TextStyle {
        REGULAR
    }
}

//! Available text styles.

bitflags::bitflags! {
    /// Available text styles.
    #[repr(C)]
    #[derive(Debug)]
    pub struct TextStyle: u32 {
        /// Regular characters, no style.
        const REGULAR = 0;
        /// Bold characters.
        const BOLD = 1;
        /// Italic characters.
        const ITALIC = 2;
        /// Underlined characters.
        const UNDERLINED = 4;
        /// Strikethrough
        const STRIKETHROUGH = 8;
    }
}

impl Default for TextStyle {
    fn default() -> Self {
        Self::REGULAR
    }
}

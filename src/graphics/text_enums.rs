/// Alignment of the lines of a multi-line text
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum LineAlignment {
    /// Align each line by the direction of its script: left for left-to-right, right for right-to-left
    #[default]
    Default,
    /// Align all lines to the left, regardless of script direction
    Left,
    /// Align all lines to the center
    Center,
    /// Align all lines to the right, regardless of script direction
    Right,
}

/// Orientation of the lines of a text
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum TextOrientation {
    /// Horizontal lines, left-to-right or right-to-left depending on the script
    #[default]
    Default,
    /// Vertical lines, from top to bottom
    TopToBottom,
    /// Vertical lines, from bottom to top
    BottomToTop,
}

/// Grouping of the characters into clusters when shaping a text
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum ClusterGrouping {
    /// Group clusters by grapheme
    Grapheme,
    /// Group clusters by character
    #[default]
    Character,
    /// Do not group clusters
    None,
}

/// Direction of the text that a [`ShapedGlyph`](crate::graphics::ShapedGlyph) belongs to
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TextDirection {
    /// Direction is not specified
    Unspecified,
    /// Left-to-right
    LeftToRight,
    /// Right-to-left
    RightToLeft,
    /// Top-to-bottom
    TopToBottom,
    /// Bottom-to-top
    BottomToTop,
}

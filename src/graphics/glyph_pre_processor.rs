use {
    crate::graphics::{Color, ShapedGlyph, TextStyle},
    std::{fmt, os::raw::c_void, rc::Rc},
};

/// Appearance of a glyph, as changed by a glyph pre-processor
///
/// See [`Text::set_glyph_pre_processor`](crate::graphics::Text::set_glyph_pre_processor).
#[derive(Clone, Copy, Debug)]
pub struct GlyphAppearance {
    /// Style of the glyph
    pub style: TextStyle,
    /// Fill color of the glyph
    pub fill_color: Color,
    /// Outline color of the glyph
    pub outline_color: Color,
    /// Outline thickness of the glyph, in pixels
    pub outline_thickness: f32,
}

/// Glyph pre-processor closure, boxed so that a pointer to it is thin enough to pass to C++
type GlyphPreProcessorFn = dyn Fn(&ShapedGlyph, &mut GlyphAppearance);

/// Glyph pre-processor, shared by a text and its clones
///
/// The C++ side keeps a raw pointer to the boxed closure, so every text holding a copy of it
/// also holds a reference to it, which keeps it alive for as long as the C++ copy can call it.
#[derive(Clone)]
pub(crate) struct SharedGlyphPreProcessor(Rc<Box<GlyphPreProcessorFn>>);

impl SharedGlyphPreProcessor {
    pub(crate) fn new<F>(pre_processor: F) -> Self
    where
        F: Fn(&ShapedGlyph, &mut GlyphAppearance) + 'static,
    {
        let boxed: Box<GlyphPreProcessorFn> = Box::new(pre_processor);
        Self(Rc::new(boxed))
    }

    pub(crate) fn user_data(&self) -> *mut c_void {
        Rc::as_ptr(&self.0).cast_mut().cast()
    }
}

impl fmt::Debug for SharedGlyphPreProcessor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SharedGlyphPreProcessor")
    }
}

/// Callback invoked by SFML for every glyph when the geometry of a text is generated
pub(crate) unsafe extern "C" fn glyph_pre_processor_trampoline(
    shaped_glyph: *const ShapedGlyph,
    style: *mut u32,
    fill_color: *mut Color,
    outline_color: *mut Color,
    outline_thickness: *mut f32,
    user_data: *mut c_void,
) {
    unsafe {
        let pre_processor = &*user_data.cast::<Box<GlyphPreProcessorFn>>();
        let mut appearance = GlyphAppearance {
            style: TextStyle::from_bits_truncate(*style),
            fill_color: *fill_color,
            outline_color: *outline_color,
            outline_thickness: *outline_thickness,
        };
        pre_processor(&*shaped_glyph, &mut appearance);
        *style = appearance.style.bits();
        *fill_color = appearance.fill_color;
        *outline_color = appearance.outline_color;
        *outline_thickness = appearance.outline_thickness;
    }
}

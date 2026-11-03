use {
    crate::{
        ffi::graphics as ffi,
        graphics::{
            ClusterGrouping, Color, Drawable, FloatRect, Font, GlyphAppearance, LineAlignment,
            RenderStates, RenderTarget, ShapedGlyph, TextOrientation, TextStyle, Transform,
            Transformable, Vertex,
            glyph_pre_processor::{SharedGlyphPreProcessor, glyph_pre_processor_trampoline},
            vertex,
        },
        system::{Angle, SfStr, SfStrConv, Vector2f},
    },
    std::{
        marker::PhantomData,
        ptr::{self, NonNull},
    },
};

/// Graphical text
///
/// Text is a drawable type that allows to easily
/// display some text with custom style and color on a render target.
///
/// __Note:__
/// Currently, it is not feasible to store text long term.
/// A common pattern with rust-sfml is to create a `Text` right before you start drawing,
/// and draw all the text you want with it. You can change its properties using
/// `set_font`, `set_position`, `set_string`, etc., before drawing it, as many times as you need
/// to.
///
/// # Examples
///
/// ```
/// use sfml::graphics::{Font, Text, TextStyle, Transformable};
/// use sfml::system::Vector2f;
///
/// let font = Font::from_file("examples/resources/sansation.ttf").expect("failed to load font");
/// let mut text = Text::new("Hello, SFML!", &font, 24);
/// text.set_style(TextStyle::BOLD | TextStyle::ITALIC);
/// text.set_position(Vector2f::new(10., 20.));
///
/// assert_eq!(text.character_size(), 24);
/// assert!(text.style().contains(TextStyle::BOLD));
/// assert_eq!(text.position(), Vector2f::new(10., 20.));
/// ```
#[derive(Debug)]
pub struct Text<'s> {
    handle: NonNull<ffi::sfText>,
    font: PhantomData<&'s Font>,
    glyph_pre_processor: Option<SharedGlyphPreProcessor>,
}

impl<'s> Text<'s> {
    /// Create a new text with initialized value
    ///
    /// Default value for characterSize on SFML is 30.
    ///
    /// # Panics
    ///
    /// Panics if the text allocation fails or if the string contains an
    /// interior NUL.
    ///
    /// # Arguments
    /// * string - The string of the text
    /// * font - The font to display the Text
    /// * characterSize - The size of the Text
    ///
    /// ```
    /// # use sfml::graphics::{Font, Text};
    /// let font = Font::from_file("examples/resources/sansation.ttf").expect("failed to load font");
    /// let text = Text::new("Hello", &font, 30);
    /// assert_eq!(text.character_size(), 30);
    /// ```
    pub fn new<S: SfStrConv>(string: S, font: &'s Font, character_size: u32) -> Text<'s> {
        let text = string.with_as_sfstr(|sfstr| unsafe {
            ffi::sfText_new(font, sfstr.as_ptr(), character_size)
        });
        Text {
            handle: NonNull::new(text).expect("Failed to create Text"),
            font: PhantomData,
            glyph_pre_processor: None,
        }
    }

    /// Set the string of a text
    ///
    /// A text's string is empty by default.
    ///
    /// # Arguments
    /// * string - New string
    ///
    /// ```
    /// # use sfml::graphics::{Font, Text};
    /// # let font = Font::from_file("examples/resources/sansation.ttf").expect("failed to load font");
    /// let mut text = Text::new("", &font, 30);
    /// text.set_string("Hello");
    /// assert_eq!(text.string().to_rust_string(), "Hello");
    /// ```
    pub fn set_string<S: SfStrConv>(&mut self, string: S) {
        string.with_as_sfstr(|sfstr| unsafe {
            ffi::sfText_setUnicodeString(self.handle.as_ptr(), sfstr.as_ptr());
        });
    }

    /// Get the string of a text
    #[must_use]
    pub fn string(&self) -> &SfStr {
        unsafe {
            let utf32: *const u32 = ffi::sfText_getUnicodeString(self.handle.as_ptr());
            SfStr::from_ptr_str(utf32)
        }
    }

    /// Get the size of the characters
    ///
    /// Return the size of the characters
    #[must_use]
    pub fn character_size(&self) -> u32 {
        unsafe { ffi::sfText_getCharacterSize(self.handle.as_ptr()) }
    }

    /// Set the font of the text
    ///
    /// The font argument refers to a texture that must
    /// exist as long as the text uses it. Indeed, the text
    /// doesn't store its own copy of the font, but rather keeps
    /// a pointer to the one that you passed to this function.
    /// If the font is destroyed and the text tries to
    /// use it, the behaviour is undefined.
    ///
    /// font - New font
    ///
    /// ```
    /// # use sfml::graphics::{Font, Text};
    /// let font = Font::from_file("examples/resources/sansation.ttf").expect("failed to load font");
    /// let mut text = Text::new("Hello", &font, 30);
    /// assert!(text.font().is_some());
    /// ```
    pub fn set_font(&mut self, font: &'s Font) {
        unsafe { ffi::sfText_setFont(self.handle.as_ptr(), font) }
    }

    /// Set the style of a text
    ///
    /// You can pass a combination of one or more styles, for
    /// example Bold | Italic.
    /// The default style is Regular.
    ///
    /// # Arguments
    /// * style - New style
    ///
    /// ```
    /// # use sfml::graphics::{Font, Text, TextStyle};
    /// # let font = Font::from_file("examples/resources/sansation.ttf").expect("failed to load font");
    /// let mut text = Text::new("Hello", &font, 30);
    /// text.set_style(TextStyle::BOLD | TextStyle::UNDERLINED);
    /// assert!(text.style().contains(TextStyle::BOLD));
    /// assert!(text.style().contains(TextStyle::UNDERLINED));
    /// assert!(!text.style().contains(TextStyle::ITALIC));
    /// ```
    pub fn set_style(&mut self, style: TextStyle) {
        unsafe { ffi::sfText_setStyle(self.handle.as_ptr(), style.bits()) }
    }

    /// Set the size of the characters of a text
    ///
    /// The default size is 30.
    ///
    /// # Arguments
    /// * size - The new character size, in pixels
    ///
    /// ```
    /// # use sfml::graphics::{Font, Text};
    /// # let font = Font::from_file("examples/resources/sansation.ttf").expect("failed to load font");
    /// let mut text = Text::new("Hello", &font, 30);
    /// text.set_character_size(48);
    /// assert_eq!(text.character_size(), 48);
    /// ```
    pub fn set_character_size(&mut self, size: u32) {
        unsafe { ffi::sfText_setCharacterSize(self.handle.as_ptr(), size) }
    }

    /// Get the style of a text
    ///
    /// Return the current string style (see Style enum)
    #[must_use]
    pub fn style(&self) -> TextStyle {
        unsafe { TextStyle::from_bits_truncate(ffi::sfText_getStyle(self.handle.as_ptr())) }
    }

    /// Get the font of a text
    /// If the text has no font attached, a None is returned.
    /// The returned pointer is const, which means that you can't
    /// modify the font when you retrieve it with this function.
    #[must_use]
    pub fn font(&self) -> Option<&'s Font> {
        unsafe { ffi::sfText_getFont(self.handle.as_ptr()).as_ref() }
    }

    /// Set the fill color of the text.
    ///
    /// By default, the text's fill color is opaque white. Setting the fill color to a transparent
    /// color with an outline will cause the outline to be displayed in the fill area of the text.
    ///
    /// ```
    /// # use sfml::graphics::{Color, Font, Text};
    /// # let font = Font::from_file("examples/resources/sansation.ttf").expect("failed to load font");
    /// let mut text = Text::new("Hello", &font, 30);
    /// text.set_fill_color(Color::RED);
    /// assert_eq!(text.fill_color(), Color::RED);
    /// ```
    pub fn set_fill_color(&mut self, color: Color) {
        unsafe { ffi::sfText_setFillColor(self.handle.as_ptr(), color) }
    }

    /// Set the outline color of the text.
    ///
    /// By default, the text's outline color is opaque black.
    pub fn set_outline_color(&mut self, color: Color) {
        unsafe { ffi::sfText_setOutlineColor(self.handle.as_ptr(), color) }
    }

    /// Set the thickness of the text's outline.
    ///
    /// By default, the outline thickness is 0.
    ///
    /// Be aware that using a negative value for the outline thickness will cause distorted
    /// rendering.
    pub fn set_outline_thickness(&mut self, thickness: f32) {
        unsafe { ffi::sfText_setOutlineThickness(self.handle.as_ptr(), thickness) }
    }

    /// Returns the fill color of the text.
    #[must_use]
    pub fn fill_color(&self) -> Color {
        unsafe { ffi::sfText_getFillColor(self.handle.as_ptr()) }
    }

    /// Returns the outline color of the text.
    #[must_use]
    pub fn outline_color(&self) -> Color {
        unsafe { ffi::sfText_getOutlineColor(self.handle.as_ptr()) }
    }

    /// Returns the outline thickness of the text, in pixels.
    #[must_use]
    pub fn outline_thickness(&self) -> f32 {
        unsafe { ffi::sfText_getOutlineThickness(self.handle.as_ptr()) }
    }

    /// Get the shaped glyphs that make up the text
    ///
    /// The glyph positions are in local coordinates, which means
    /// that translation, rotation, scale and origin are not applied.
    ///
    /// This takes `&mut self`, because SFML may regenerate the text geometry
    /// (and therefore invalidate the returned slice) from `&self` methods
    /// such as [`Drawable::draw`].
    #[must_use]
    pub fn shaped_glyphs(&mut self) -> &[ShapedGlyph] {
        unsafe { &*ffi::sfText_getShapedGlyphs(self.handle.as_ptr()) }
    }

    /// Set the alignment of the lines of a multi-line text
    ///
    /// By default, lines are aligned according to the direction of their script.
    ///
    /// # Arguments
    /// * alignment - New line alignment
    ///
    /// ```
    /// # use sfml::graphics::{Font, LineAlignment, Text};
    /// # let font = Font::from_file("examples/resources/sansation.ttf").expect("failed to load font");
    /// let mut text = Text::new("Hello", &font, 30);
    /// text.set_line_alignment(LineAlignment::Center);
    /// assert_eq!(text.line_alignment(), LineAlignment::Center);
    /// ```
    pub fn set_line_alignment(&mut self, alignment: LineAlignment) {
        unsafe { ffi::sfText_setLineAlignment(self.handle.as_ptr(), alignment) }
    }

    /// Get the alignment of the lines of a multi-line text
    #[must_use]
    pub fn line_alignment(&self) -> LineAlignment {
        unsafe { ffi::sfText_getLineAlignment(self.handle.as_ptr()) }
    }

    /// Set the orientation of the lines of a text
    ///
    /// By default, the lines have a horizontal orientation. Most fonts don't natively support
    /// vertical orientations, so SFML has to emulate them, which may not look good.
    ///
    /// # Arguments
    /// * orientation - New text orientation
    ///
    /// ```
    /// # use sfml::graphics::{Font, Text, TextOrientation};
    /// # let font = Font::from_file("examples/resources/sansation.ttf").expect("failed to load font");
    /// let mut text = Text::new("Hello", &font, 30);
    /// text.set_text_orientation(TextOrientation::TopToBottom);
    /// assert_eq!(text.text_orientation(), TextOrientation::TopToBottom);
    /// ```
    pub fn set_text_orientation(&mut self, orientation: TextOrientation) {
        unsafe { ffi::sfText_setTextOrientation(self.handle.as_ptr(), orientation) }
    }

    /// Get the orientation of the lines of a text
    #[must_use]
    pub fn text_orientation(&self) -> TextOrientation {
        unsafe { ffi::sfText_getTextOrientation(self.handle.as_ptr()) }
    }

    /// Set the cluster grouping algorithm used when shaping the text
    ///
    /// By default, character cluster grouping is used.
    ///
    /// # Arguments
    /// * `cluster_grouping` - New cluster grouping algorithm
    ///
    /// ```
    /// # use sfml::graphics::{ClusterGrouping, Font, Text};
    /// # let font = Font::from_file("examples/resources/sansation.ttf").expect("failed to load font");
    /// let mut text = Text::new("Hello", &font, 30);
    /// text.set_cluster_grouping(ClusterGrouping::Grapheme);
    /// assert_eq!(text.cluster_grouping(), ClusterGrouping::Grapheme);
    /// ```
    pub fn set_cluster_grouping(&mut self, cluster_grouping: ClusterGrouping) {
        unsafe { ffi::sfText_setClusterGrouping(self.handle.as_ptr(), cluster_grouping) }
    }

    /// Get the cluster grouping algorithm used when shaping the text
    #[must_use]
    pub fn cluster_grouping(&self) -> ClusterGrouping {
        unsafe { ffi::sfText_getClusterGrouping(self.handle.as_ptr()) }
    }

    /// Set or remove the glyph pre-processor
    ///
    /// The pre-processor is called for every glyph whenever the geometry of the text is
    /// regenerated. It receives the shaped glyph, and can change the glyph's style, fill color,
    /// outline color and outline thickness through the [`GlyphAppearance`].
    ///
    /// `Some` replaces any previously set pre-processor, and `None` removes it. Since `None`
    /// has no closure type to infer, name the type, e.g.
    /// `None::<fn(&ShapedGlyph, &mut GlyphAppearance)>`.
    ///
    /// The pre-processor is called from within SFML, so it must not panic (that aborts the
    /// process), and it has no access to the text it is set on.
    ///
    /// # Arguments
    /// * `pre_processor` - The pre-processor to call for every glyph, or `None` to remove it
    pub fn set_glyph_pre_processor<F>(&mut self, pre_processor: Option<F>)
    where
        F: Fn(&ShapedGlyph, &mut GlyphAppearance) + 'static,
    {
        let pre_processor = pre_processor.map(SharedGlyphPreProcessor::new);
        let callback: ffi::sfTextGlyphPreProcessorCb = if pre_processor.is_some() {
            Some(glyph_pre_processor_trampoline)
        } else {
            None
        };
        let user_data = pre_processor
            .as_ref()
            .map_or(ptr::null_mut(), SharedGlyphPreProcessor::user_data);
        unsafe { ffi::sfText_setGlyphPreProcessor(self.handle.as_ptr(), callback, user_data) }
        self.glyph_pre_processor = pre_processor;
    }

    /// Get the vertices of the fill geometry of the text
    ///
    /// SFML regenerates the vertices whenever the text changes, which discards any changes made
    /// to them. This takes `&mut self` for the same reason as [`Text::shaped_glyphs`].
    #[must_use]
    pub fn vertex_data(&mut self) -> &mut [Vertex] {
        let mut count = 0;
        let data = unsafe { ffi::sfText_getVertexData(self.handle.as_ptr(), &mut count) };
        unsafe { vertex::slice_from_raw_parts_mut(data.cast(), count) }
    }

    /// Get the vertices of the outline geometry of the text
    ///
    /// Same as [`Text::vertex_data`], but for the outline.
    #[must_use]
    pub fn outline_vertex_data(&mut self) -> &mut [Vertex] {
        let mut count = 0;
        let data = unsafe { ffi::sfText_getOutlineVertexData(self.handle.as_ptr(), &mut count) };
        unsafe { vertex::slice_from_raw_parts_mut(data.cast(), count) }
    }

    /// Get the local bounding rectangle of a text
    ///
    /// The returned rectangle is in local coordinates, which means
    /// that it ignores the transformations (translation, rotation,
    /// scale, ...) that are applied to the entity.
    /// In other words, this function returns the bounds of the
    /// entity in the entity's coordinate system.
    ///
    /// Return the local bounding rectangle of the entity
    #[must_use]
    pub fn local_bounds(&self) -> FloatRect {
        unsafe { ffi::sfText_getLocalBounds(self.handle.as_ptr()) }
    }

    /// Get the global bounding rectangle of a text
    ///
    /// The returned rectangle is in global coordinates, which means
    /// that it takes in account the transformations (translation,
    /// rotation, scale, ...) that are applied to the entity.
    /// In other words, this function returns the bounds of the
    /// text in the global 2D world's coordinate system.
    ///
    /// Return the global bounding rectangle of the entity
    #[must_use]
    pub fn global_bounds(&self) -> FloatRect {
        unsafe { ffi::sfText_getGlobalBounds(self.handle.as_ptr()) }
    }
    /// Get the size of the line spacing factor.
    #[must_use]
    pub fn line_spacing(&self) -> f32 {
        unsafe { ffi::sfText_getLineSpacing(self.handle.as_ptr()) }
    }
    /// Set the line spacing factor.
    ///
    /// The default spacing between lines is defined by the font.
    /// This method enables you to set a factor for the spacing between lines.
    /// By default the line spacing factor is 1.
    ///
    /// ```
    /// # use sfml::graphics::{Font, Text};
    /// # let font = Font::from_file("examples/resources/sansation.ttf").expect("failed to load font");
    /// let mut text = Text::new("Hello", &font, 30);
    /// text.set_line_spacing(1.5);
    /// assert_eq!(text.line_spacing(), 1.5);
    /// ```
    pub fn set_line_spacing(&mut self, factor: f32) {
        unsafe { ffi::sfText_setLineSpacing(self.handle.as_ptr(), factor) }
    }
    /// Get the size of the letter spacing factor.
    #[must_use]
    pub fn letter_spacing(&self) -> f32 {
        unsafe { ffi::sfText_getLetterSpacing(self.handle.as_ptr()) }
    }
    /// Set the letter spacing factor.
    ///
    /// The default spacing between letters is defined by the font.
    /// This factor doesn't directly apply to the existing spacing between each character,
    /// it rather adds a fixed space between them which is calculated from the font metrics and
    /// the character size. Note that factors below 1 (including negative numbers) bring
    /// characters closer to each other. By default the letter spacing factor is 1.
    ///
    /// ```
    /// # use sfml::graphics::{Font, Text};
    /// # let font = Font::from_file("examples/resources/sansation.ttf").expect("failed to load font");
    /// let mut text = Text::new("Hello", &font, 30);
    /// text.set_letter_spacing(2.);
    /// assert_eq!(text.letter_spacing(), 2.);
    /// ```
    pub fn set_letter_spacing(&mut self, factor: f32) {
        unsafe { ffi::sfText_setLetterSpacing(self.handle.as_ptr(), factor) }
    }
    pub(super) fn raw(&self) -> *const ffi::sfText {
        self.handle.as_ptr()
    }
}

impl<'s> Clone for Text<'s> {
    /// Return a new Text or panic! if there is not enough memory
    fn clone(&self) -> Text<'s> {
        let sp = unsafe { ffi::sfText_cpy(self.handle.as_ptr()) };
        Text {
            handle: NonNull::new(sp).expect("Not enough memory to clone Text"),
            font: PhantomData,
            glyph_pre_processor: self.glyph_pre_processor.clone(),
        }
    }
}

impl Drawable for Text<'_> {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn RenderTarget,
        states: &RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        target.draw_text(self, states);
    }
}

impl Transformable for Text<'_> {
    fn set_position<P: Into<Vector2f>>(&mut self, position: P) {
        unsafe { ffi::sfText_setPosition(self.handle.as_ptr(), position.into()) }
    }
    fn set_rotation(&mut self, angle: Angle) {
        unsafe { ffi::sfText_setRotation(self.handle.as_ptr(), angle.as_degrees()) }
    }
    fn set_scale<S: Into<Vector2f>>(&mut self, scale: S) {
        unsafe { ffi::sfText_setScale(self.handle.as_ptr(), scale.into()) }
    }
    fn set_origin<O: Into<Vector2f>>(&mut self, origin: O) {
        unsafe { ffi::sfText_setOrigin(self.handle.as_ptr(), origin.into()) }
    }
    fn position(&self) -> Vector2f {
        unsafe { ffi::sfText_getPosition(self.handle.as_ptr()) }
    }
    fn rotation(&self) -> Angle {
        Angle::degrees(unsafe { ffi::sfText_getRotation(self.handle.as_ptr()) })
    }
    fn scale(&self) -> Vector2f {
        unsafe { ffi::sfText_getScale(self.handle.as_ptr()) }
    }
    fn origin(&self) -> Vector2f {
        unsafe { ffi::sfText_getOrigin(self.handle.as_ptr()) }
    }
    fn move_<O: Into<Vector2f>>(&mut self, offset: O) {
        unsafe { ffi::sfText_move(self.handle.as_ptr(), offset.into()) }
    }
    fn rotate(&mut self, angle: Angle) {
        unsafe { ffi::sfText_rotate(self.handle.as_ptr(), angle.as_degrees()) }
    }
    fn scale_by<F: Into<Vector2f>>(&mut self, factors: F) {
        unsafe { ffi::sfText_scale(self.handle.as_ptr(), factors.into()) }
    }
    fn transform(&self) -> &Transform {
        unsafe { &*ffi::sfText_getTransform(self.handle.as_ptr()) }
    }
    fn inverse_transform(&self) -> &Transform {
        unsafe { &*ffi::sfText_getInverseTransform(self.handle.as_ptr()) }
    }
}

impl Drop for Text<'_> {
    fn drop(&mut self) {
        unsafe {
            ffi::sfText_del(self.handle.as_ptr());
        }
    }
}

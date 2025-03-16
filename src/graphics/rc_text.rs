use {
    crate::{
        cpp::FBox,
        ffi::graphics as ffi,
        graphics::{
            Color, Drawable, FloatRect, Font, RcFont, RenderStates, RenderTarget, TextStyle,
            Transform, Transformable,
        },
        system::{Angle, SfStr, SfStrConv, Vector2f},
    },
    std::{cell::RefCell, ptr::NonNull, rc::Weak},
};

const ERROR_MSG: &str = "Text does not hold a font. Ignoring transformation!";
const RETURN_ERROR_MSG: &str = "Text does not hold a font. Returning default value!";
const PANIC_ERROR_MSG: &str = "Text does not hold a font! Return value cannot be discerned!";

/// Graphical text (reference counted)
///
/// `RcText` is a drawable type that allows to easily
/// display some text with custom style and color on a render target.
///
/// This is an alternative to [`Text`], allowing for complete seperation from the font's lifetime.
///
/// Underneath, it uses reference counting to ensure that the [`RcFont`] is alive,
/// and disallows performing certain actions on the text if the
/// [`RcFont`] is no longer alive. It will print an error message in these cases.
///
/// The only functions that allow usage while the [`RcFont`] is not alive are
/// `set_string`, `string`, `character_size`, `set_style`, `set_font`, `set_character_size`, `style`, `font`,
/// `fill-color`, `outline_color`, `outline_thickness`, `line_spacing`, `set_line_spacing`, `letter_spacing`,
/// `set_letter_spacing`
///
/// [`Text`]: crate::graphics::Text
#[derive(Debug)]
pub struct RcText {
    handle: NonNull<ffi::sfText>,
    font: Weak<RefCell<FBox<Font>>>,
}

impl RcText {
    /// Create a new `RcText` with initialized value
    ///
    /// Default value for characterSize on SFML is 30.
    ///
    /// # Panics
    /// Panics during const evaluation.
    /// This method will panic during const evaluation if the string cannot be
    /// determined to be null or not.
    ///
    /// # Arguments
    /// * string - The string of the `RcText`
    /// * font - The [`RcFont`] to display the `RcText`
    /// * characterSize - The size of the `RcText`
    pub fn new<S: SfStrConv>(string: S, font: &RcFont, character_size: u32) -> Self {
        let text = string.with_as_sfstr(|sfstr| unsafe {
            ffi::sfText_new(font.raw_font(), sfstr.as_ptr(), character_size)
        });
        RcText {
            handle: NonNull::new(text).expect("Failed to create Text"),
            font: font.downgrade(),
        }
    }

    fn font_exists(&self) -> bool {
        self.font.strong_count() != 0
    }

    fn set_rc_font(&mut self, font: &RcFont) {
        self.font = font.downgrade();
    }

    /// Set the string of a `RcText`
    ///
    /// A `RcText`'s string is empty by default.
    ///
    /// # Arguments
    /// * string - New string
    pub fn set_string<S: SfStrConv>(&mut self, string: S) {
        string.with_as_sfstr(|sfstr| unsafe {
            ffi::sfText_setUnicodeString(self.handle.as_ptr(), sfstr.as_ptr());
        })
    }

    /// Get the string of a `RcText`
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

    /// Set the font of the `RcText`
    ///
    /// font - New [`RcFont`]
    ///
    /// # Panics
    ///
    /// Panics on [`std::rc::Rc`] related shenanigans.
    pub fn set_font(&mut self, font: &RcFont) {
        self.set_rc_font(font);
        unsafe {
            #[expect(clippy::unwrap_used)]
            ffi::sfText_setFont(
                self.handle.as_ptr(),
                (*self.font.upgrade().unwrap().as_ptr())
                    .0
                    .as_ptr()
                    .cast_const(),
            )
        }
    }

    /// Set the style of a `RcText`
    ///
    /// You can pass a combination of one or more styles, for
    /// example Bold | Italic.
    /// The default style is Regular.
    ///
    /// # Arguments
    /// * style - New style
    pub fn set_style(&mut self, style: TextStyle) {
        unsafe { ffi::sfText_setStyle(self.handle.as_ptr(), style.bits()) }
    }

    /// Set the size of the characters of a `RcText`
    ///
    /// The default size is 30.
    ///
    /// # Arguments
    /// * size - The new character size, in pixel
    pub fn set_character_size(&mut self, size: u32) {
        unsafe { ffi::sfText_setCharacterSize(self.handle.as_ptr(), size) }
    }

    /// Get the style of a `RcText`
    ///
    /// Return the current string style (see Style enum)
    #[must_use]
    pub fn style(&self) -> TextStyle {
        unsafe { TextStyle::from_bits_truncate(ffi::sfText_getStyle(self.handle.as_ptr())) }
    }

    /// Get the Font of a `RcText`
    /// If the `RcText` has no [`RcFont`] attached, a None is returned.
    /// The returned pointer is const, which means that you can't
    /// modify the font when you retrieve it with this function.
    #[must_use]
    pub fn font(&self) -> Option<&Font> {
        if self.font_exists() {
            Some(unsafe { &*(*(self.font.as_ptr())).as_ptr() })
        } else {
            None
        }
    }

    /// Set the fill color of the text.
    ///
    /// By default, the text's fill color is opaque white. Setting the fill color to a transparent
    /// color with an outline will cause the outline to be displayed in the fill area of the text.
    ///
    /// # Warning
    /// Function fails, and prints an error message if `RcText`'s font is dead.
    pub fn set_fill_color(&mut self, color: Color) {
        if !self.font_exists() {
            eprintln!("{ERROR_MSG}");
            return;
        }
        unsafe { ffi::sfText_setFillColor(self.handle.as_ptr(), color) }
    }

    /// Set the thickness of the `RcText`'s outline.
    ///
    /// By default, the outline thickness is 0.
    ///
    /// Be aware that using a negative value for the outline thickness will cause distorted
    /// rendering.
    ///
    /// # Warning
    /// Function fails, and prints an error message if `RcText`'s font is dead.
    pub fn set_outline_thickness(&mut self, thickness: f32) {
        if !self.font_exists() {
            eprintln!("{ERROR_MSG}");
            return;
        }
        unsafe { ffi::sfText_setOutlineThickness(self.handle.as_ptr(), thickness) }
    }

    /// Returns the fill color of the `RcText`.
    #[must_use]
    pub fn fill_color(&self) -> Color {
        unsafe { ffi::sfText_getFillColor(self.handle.as_ptr()) }
    }

    /// Returns the outline color of the `RcText`.
    #[must_use]
    pub fn outline_color(&self) -> Color {
        unsafe { ffi::sfText_getOutlineColor(self.handle.as_ptr()) }
    }

    /// Returns the outline thickness of the `RcText`, in pixels.
    #[must_use]
    pub fn outline_thickness(&self) -> f32 {
        unsafe { ffi::sfText_getOutlineThickness(self.handle.as_ptr()) }
    }

    /// Return the position of the index-th character in a text
    ///
    /// This function computes the visual position of a character
    /// from its index in the string. The returned position is
    /// in global coordinates (translation, rotation, scale and
    /// origin are applied).
    /// If index is out of range, the position of the end of
    /// the string is returned.
    ///
    /// # Arguments
    /// * index - The index of the character
    ///
    /// Return the position of the character
    ///
    /// # Warning
    /// Function fails, returns default, and prints an error message if [`RcFont`] is dead.
    #[must_use]
    pub fn find_character_pos(&self, index: usize) -> Vector2f {
        if !self.font_exists() {
            eprintln!("{RETURN_ERROR_MSG}");
            return Default::default();
        }
        unsafe { ffi::sfText_findCharacterPos(self.handle.as_ptr(), index) }
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
    ///
    /// # Warning
    /// Function fails, returns default, and prints an error message if [`RcFont`] is dead.
    #[must_use]
    pub fn local_bounds(&self) -> FloatRect {
        if !self.font_exists() {
            eprintln!("{RETURN_ERROR_MSG}");
            return Default::default();
        }
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
    ///
    /// # Warning
    /// Function fails, returns default, and prints an error message if [`RcFont`] is dead.
    #[must_use]
    pub fn global_bounds(&self) -> FloatRect {
        if !self.font_exists() {
            eprintln!("{RETURN_ERROR_MSG}");
            return Default::default();
        }
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
    /// characters closer to each other    
    pub fn set_letter_spacing(&mut self, factor: f32) {
        unsafe { ffi::sfText_setLetterSpacing(self.handle.as_ptr(), factor) }
    }

    pub(super) fn raw(&self) -> *const ffi::sfText {
        self.handle.as_ptr()
    }
}

impl Clone for RcText {
    /// Return a new Text or panic! if there is not enough memory
    fn clone(&self) -> Self {
        let sp = unsafe { ffi::sfText_cpy(self.handle.as_ptr()) };
        Self {
            handle: NonNull::new(sp).expect("Not enough memory to clone Text"),
            font: self.font.clone(),
        }
    }
}

impl Drawable for RcText {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn RenderTarget,
        states: &RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        if self.font_exists() {
            target.draw_rc_text(self, states)
        }
    }
}

impl Transformable for RcText {
    /// Reference [`Transformable::set_position`] for additional information
    ///
    /// # Warning
    /// Function fails, and prints an error message if `RcText`'s font is dead.
    fn set_position<P: Into<Vector2f>>(&mut self, position: P) {
        if !self.font_exists() {
            eprintln!("{ERROR_MSG}");
            return;
        }
        unsafe { ffi::sfText_setPosition(self.handle.as_ptr(), position.into()) }
    }
    /// Reference [`Transformable::set_rotation`] for additional information
    ///
    /// # Warning
    /// Function fails, and prints an error message if `RcText`'s font is dead.
    fn set_rotation(&mut self, angle: Angle) {
        if !self.font_exists() {
            eprintln!("{ERROR_MSG}");
            return;
        }
        unsafe { ffi::sfText_setRotation(self.handle.as_ptr(), angle.as_degrees()) }
    }
    /// Reference [`Transformable::set_scale`] for additional information
    ///
    /// # Warning
    /// Function fails, and prints an error message if `RcText`'s font is dead.
    fn set_scale<S: Into<Vector2f>>(&mut self, scale: S) {
        if !self.font_exists() {
            eprintln!("{ERROR_MSG}");
            return;
        }
        unsafe { ffi::sfText_setScale(self.handle.as_ptr(), scale.into()) }
    }
    /// Reference [`Transformable::set_origin`] for additional information
    ///
    /// # Warning
    /// Function fails, and prints an error message if `RcText`'s font is dead.
    fn set_origin<O: Into<Vector2f>>(&mut self, origin: O) {
        if !self.font_exists() {
            eprintln!("{ERROR_MSG}");
            return;
        }
        unsafe { ffi::sfText_setOrigin(self.handle.as_ptr(), origin.into()) }
    }
    /// Reference [`Transformable::position`] for additional information
    ///
    /// # Warning
    /// Function fails, returns default, and prints an error message if [`RcFont`] is dead.
    fn position(&self) -> Vector2f {
        if !self.font_exists() {
            eprintln!("{RETURN_ERROR_MSG}");
            return Default::default();
        }
        unsafe { ffi::sfText_getPosition(self.handle.as_ptr()) }
    }
    /// Reference [`Transformable::rotation`] for additional information
    ///
    /// # Warning
    /// Function fails, returns default, and prints an error message if [`RcFont`] is dead.
    fn rotation(&self) -> Angle {
        if !self.font_exists() {
            eprintln!("{RETURN_ERROR_MSG}");
            return Default::default();
        }
        Angle::degrees(unsafe { ffi::sfText_getRotation(self.handle.as_ptr()) })
    }
    /// Reference [`Transformable::get_scale`] for additional information
    ///
    /// # Warning
    /// Function fails, returns default, and prints an error message if [`RcFont`] is dead.
    fn get_scale(&self) -> Vector2f {
        if !self.font_exists() {
            eprintln!("{RETURN_ERROR_MSG}");
            return Default::default();
        }
        unsafe { ffi::sfText_getScale(self.handle.as_ptr()) }
    }
    /// Reference [`Transformable::origin`] for additional information
    ///
    /// # Warning
    /// Function fails, returns default, and prints an error message if [`RcFont`] is dead.
    fn origin(&self) -> Vector2f {
        if !self.font_exists() {
            eprintln!("{RETURN_ERROR_MSG}");
            return Default::default();
        }
        unsafe { ffi::sfText_getOrigin(self.handle.as_ptr()) }
    }
    /// Reference [`Transformable::move_`] for additional information
    ///
    /// # Warning
    /// Function fails, and prints an error message if `RcText`'s font is dead.
    fn move_<O: Into<Vector2f>>(&mut self, offset: O) {
        if !self.font_exists() {
            eprintln!("{ERROR_MSG}");
            return;
        }
        unsafe { ffi::sfText_move(self.handle.as_ptr(), offset.into()) }
    }
    /// Reference [`Transformable::rotate`] for additional information
    ///
    /// # Warning
    /// Function fails, and prints an error message if `RcText`'s font is dead.
    fn rotate(&mut self, angle: Angle) {
        if !self.font_exists() {
            eprintln!("{ERROR_MSG}");
            return;
        }
        unsafe { ffi::sfText_rotate(self.handle.as_ptr(), angle.as_degrees()) }
    }
    /// Reference [`Transformable::scale`] for additional information
    ///
    /// # Warning
    /// Function fails, and prints an error message if `RcText`'s font is dead.
    fn scale<F: Into<Vector2f>>(&mut self, factors: F) {
        if !self.font_exists() {
            eprintln!("{ERROR_MSG}");
            return;
        }
        unsafe { ffi::sfText_scale(self.handle.as_ptr(), factors.into()) }
    }
    /// Reference [`Transformable::transform`] for additional information
    ///
    /// Panics if font doesn't exist
    fn transform(&self) -> &Transform {
        if !self.font_exists() {
            panic!("{}", PANIC_ERROR_MSG);
        }
        unsafe { &*ffi::sfText_getTransform(self.handle.as_ptr()) }
    }
    /// Reference [`Transformable::inverse_transform`] for additional information
    ///
    /// Panics if font doesn't exist
    fn inverse_transform(&self) -> &Transform {
        if !self.font_exists() {
            panic!("{}", PANIC_ERROR_MSG);
        }
        unsafe { &*ffi::sfText_getInverseTransform(self.handle.as_ptr()) }
    }
}

impl Drop for RcText {
    fn drop(&mut self) {
        unsafe {
            ffi::sfText_del(self.handle.as_ptr());
        }
    }
}

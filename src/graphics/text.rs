use crate::{
    graphics::{
        csfml_graphics_sys as ffi, Color, Drawable, FloatRect, Font, RenderStates, RenderTarget,
        TextStyle, Transform, Transformable,
    },
    system::{SfStr, SfStrConv, Vector2f},
};
use std::marker::PhantomData;

/// Graphical text
///
/// Text is a drawable type that allows to easily
/// display some text with custom style and color on a render target.
#[derive(Debug)]
pub struct Text<'s> {
    text: *mut ffi::sfText,
    string_length: usize,
    font: PhantomData<&'s Font>,
}

impl<'s> Text<'s> {
    /// Create a new text with initialized value
    ///
    /// Default value for characterSize on SFML is 30.
    ///
    /// # Arguments
    /// * string - The string of the text
    /// * font - The font to display the Text
    /// * characterSize - The size of the Text
    pub fn new<S: SfStrConv>(string: S, font: &'s Font, character_size: u32) -> Text<'s> {
        let mut text = Text::default();
        text.set_string(string);
        text.set_font(font);
        text.set_character_size(character_size);
        text
    }

    /// Set the string of a text
    ///
    /// A text's string is empty by default.
    ///
    /// # Arguments
    /// * string - New string
    pub fn set_string<S: SfStrConv>(&mut self, string: S) {
        string.with_as_sfstr(|sfstr| unsafe {
            ffi::sfText_setUnicodeString(self.text, sfstr.as_ptr());
        })
    }

    /// Get the string of a text
    #[must_use]
    pub fn string(&self) -> &SfStr {
        unsafe {
            let utf32: *const u32 = ffi::sfText_getUnicodeString(self.text);
            SfStr::from_ptr_str(utf32)
        }
    }

    /// Get the size of the characters
    ///
    /// Return the size of the characters
    #[must_use]
    pub fn character_size(&self) -> u32 {
        unsafe { ffi::sfText_getCharacterSize(self.text) }
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
    pub fn set_font(&mut self, font: &'s Font) {
        unsafe { ffi::sfText_setFont(self.text, font.raw()) }
    }

    /// Set the style of a text
    ///
    /// You can pass a combination of one or more styles, for
    /// example Bold | Italic.
    /// The default style is Regular.
    ///
    /// # Arguments
    /// * style - New style
    pub fn set_style(&mut self, style: TextStyle) {
        unsafe { ffi::sfText_setStyle(self.text, style.bits()) }
    }

    /// Set the size of the characters of a text
    ///
    /// The default size is 30.
    ///
    /// # Arguments
    /// * size - The new character size, in pixels
    pub fn set_character_size(&mut self, size: u32) {
        unsafe { ffi::sfText_setCharacterSize(self.text, size) }
    }

    /// Get the style of a text
    ///
    /// Return the current string style (see Style enum)
    #[must_use]
    pub fn style(&self) -> TextStyle {
        unsafe { TextStyle::from_bits_truncate(ffi::sfText_getStyle(self.text)) }
    }

    /// Get the font of a text
    /// If the text has no font attached, a None is returned.
    /// The returned pointer is const, which means that you can't
    /// modify the font when you retrieve it with this function.
    #[must_use]
    pub fn font(&self) -> Option<&'s Font> {
        unsafe {
            let raw = ffi::sfText_getFont(self.text);

            if raw.is_null() {
                None
            } else {
                Some(&*(raw as *const Font))
            }
        }
    }

    /// Set the fill color of the text.
    ///
    /// By default, the text's fill color is opaque white. Setting the fill color to a transparent
    /// color with an outline will cause the outline to be displayed in the fill area of the text.
    pub fn set_fill_color(&mut self, color: Color) {
        unsafe { ffi::sfText_setFillColor(self.text, color.raw()) }
    }

    /// Set the outline color of the text.
    ///
    /// By default, the text's outline color is opaque black.
    pub fn set_outline_color(&mut self, color: Color) {
        unsafe { ffi::sfText_setOutlineColor(self.text, color.raw()) }
    }

    /// Set the thickness of the text's outline.
    ///
    /// By default, the outline thickness is 0.
    ///
    /// Be aware that using a negative value for the outline thickness will cause distorted
    /// rendering.
    pub fn set_outline_thickness(&mut self, thickness: f32) {
        unsafe { ffi::sfText_setOutlineThickness(self.text, thickness) }
    }

    /// Returns the fill color of the text.
    #[must_use]
    pub fn fill_color(&self) -> Color {
        unsafe { Color::from_raw(ffi::sfText_getFillColor(self.text)) }
    }

    /// Returns the outline color of the text.
    #[must_use]
    pub fn outline_color(&self) -> Color {
        unsafe { Color::from_raw(ffi::sfText_getOutlineColor(self.text)) }
    }

    /// Returns the outline thickness of the text, in pixels.
    #[must_use]
    pub fn outline_thickness(&self) -> f32 {
        unsafe { ffi::sfText_getOutlineThickness(self.text) }
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
    #[must_use]
    pub fn find_character_pos(&self, index: usize) -> Vector2f {
        unsafe { Vector2f::from_raw(ffi::sfText_findCharacterPos(self.text, index)) }
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
        unsafe { FloatRect::from_raw(ffi::sfText_getLocalBounds(self.text)) }
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
        unsafe { FloatRect::from_raw(ffi::sfText_getGlobalBounds(self.text)) }
    }
    pub(super) fn raw(&self) -> *const ffi::sfText {
        self.text
    }
}

impl<'s> Default for Text<'s> {
    fn default() -> Self {
        let text = unsafe { ffi::sfText_create() };
        assert!(!text.is_null(), "Failed to create Text");
        Self {
            text,
            string_length: 0,
            font: PhantomData,
        }
    }
}

impl<'s> Clone for Text<'s> {
    /// Return a new Text or panic! if there is not enough memory
    fn clone(&self) -> Text<'s> {
        let sp = unsafe { ffi::sfText_copy(self.text) };
        if sp.is_null() {
            panic!("Not enough memory to clone Text")
        } else {
            Text {
                text: self.text,
                string_length: self.string_length,
                font: PhantomData,
            }
        }
    }
}

impl<'s> Drawable for Text<'s> {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn RenderTarget,
        states: RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        target.draw_text(self, states)
    }
}

impl<'s> Transformable for Text<'s> {
    fn set_position<P: Into<Vector2f>>(&mut self, position: P) {
        unsafe { ffi::sfText_setPosition(self.text, position.into().raw()) }
    }
    fn set_rotation(&mut self, angle: f32) {
        unsafe { ffi::sfText_setRotation(self.text, angle) }
    }
    fn set_scale<S: Into<Vector2f>>(&mut self, scale: S) {
        unsafe { ffi::sfText_setScale(self.text, scale.into().raw()) }
    }
    fn set_origin<O: Into<Vector2f>>(&mut self, origin: O) {
        unsafe { ffi::sfText_setOrigin(self.text, origin.into().raw()) }
    }
    fn position(&self) -> Vector2f {
        unsafe { Vector2f::from_raw(ffi::sfText_getPosition(self.text)) }
    }
    fn rotation(&self) -> f32 {
        unsafe { ffi::sfText_getRotation(self.text) }
    }
    fn get_scale(&self) -> Vector2f {
        unsafe { Vector2f::from_raw(ffi::sfText_getScale(self.text)) }
    }
    fn origin(&self) -> Vector2f {
        unsafe { Vector2f::from_raw(ffi::sfText_getOrigin(self.text)) }
    }
    fn move_<O: Into<Vector2f>>(&mut self, offset: O) {
        unsafe { ffi::sfText_move(self.text, offset.into().raw()) }
    }
    fn rotate(&mut self, angle: f32) {
        unsafe { ffi::sfText_rotate(self.text, angle) }
    }
    fn scale<F: Into<Vector2f>>(&mut self, factors: F) {
        unsafe { ffi::sfText_scale(self.text, factors.into().raw()) }
    }
    fn transform(&self) -> Transform {
        unsafe { Transform(ffi::sfText_getTransform(self.text)) }
    }
    fn inverse_transform(&self) -> Transform {
        unsafe { Transform(ffi::sfText_getInverseTransform(self.text)) }
    }
}

impl<'s> Drop for Text<'s> {
    fn drop(&mut self) {
        unsafe {
            ffi::sfText_destroy(self.text);
        }
    }
}

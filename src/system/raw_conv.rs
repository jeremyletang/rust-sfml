//! Conversion between the raw and wrapped forms of SFML types.

/// A type that has a raw representation that can be acquired through `&self`.
pub trait Raw {
    /// The raw representation of this type.
    type Raw;
    /// Acquires the raw representation of this type through `&self`.
    fn raw(&self) -> Self::Raw;
}

/// A type that has a raw representation that can be acquired through `&mut self`.
pub trait RawMut {
    /// The raw representation of this type.
    type RawMut;
    /// Acquires the raw representation of this type through `&mut self`.
    fn raw_mut(&mut self) -> Self::RawMut;
}

/// A type that can be created from its raw representation.
pub trait FromRaw {
    /// The raw representation of this type.
    type RawFrom;
    /// Creates `Self` from its raw representation.
    ///
    /// Doing this is not always safe, so this function is `unsafe`.
    unsafe fn from_raw(raw: Self::RawFrom) -> Self;
}

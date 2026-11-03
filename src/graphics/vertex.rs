use crate::{graphics::Color, system::Vector2f};

/// Define a point with color and texture coordinates.
///
/// A vertex is an improved point.
///
/// It has a position and other extra attributes that will be used for drawing:
/// in SFML, vertices also have a color and a pair of texture coordinates.
///
/// The vertex is the building block of drawing.
/// Everything which is visible on screen is made of vertices.
/// They are grouped as 2D primitives (triangles, quads, ...), and these primitives are grouped
/// to create even more complex 2D entities such as sprites, texts, etc.
///
/// If you use the graphical entities of SFML (sprite, text, shape) you won't have to
/// deal with vertices directly.
/// But if you want to define your own 2D entities, such as tiled maps or particle systems,
/// using vertices will allow you to get maximum performance.
///
/// Note: although texture coordinates are supposed to be an integer amount of pixels,
/// their type is float because of some buggy graphics drivers that are not able to
/// process integer coordinates correctly.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vertex {
    /// 2D position of the vertex
    pub position: Vector2f,
    /// Color of the vertex.
    pub color: Color,
    /// Coordinates of the texture's pixel to map to the vertex.
    pub tex_coords: Vector2f,
}

impl Vertex {
    /// Create a new Vertex
    ///
    /// # Arguments
    /// * position - Position of the vertex
    /// * color - Color of the vertex
    /// * `tex_coords` - Texture coordinate of the vertex
    ///
    /// Return a Vertex
    ///
    /// ```
    /// # use sfml::graphics::{Color, Vertex};
    /// # use sfml::system::Vector2f;
    /// let vertex = Vertex::new(
    ///     Vector2f::new(1., 2.),
    ///     Color::RED,
    ///     Vector2f::new(0.5, 0.5),
    /// );
    /// assert_eq!(vertex.position, Vector2f::new(1., 2.));
    /// assert_eq!(vertex.color, Color::RED);
    /// assert_eq!(vertex.tex_coords, Vector2f::new(0.5, 0.5));
    /// ```
    #[must_use]
    pub const fn new(position: Vector2f, color: Color, tex_coords: Vector2f) -> Self {
        Self {
            position,
            color,
            tex_coords,
        }
    }

    /// Create a new Vertex with a position
    ///
    /// # Arguments
    /// * position - Position of the vertex
    ///
    /// # Default
    /// * color - white
    /// * `tex_coords` - (0., 0.)
    ///
    /// Return a Vertex
    ///
    /// ```
    /// # use sfml::graphics::Vertex;
    /// # use sfml::system::Vector2f;
    /// let vertex = Vertex::with_pos(Vector2f::new(3., 4.));
    /// assert_eq!(vertex.position, Vector2f::new(3., 4.));
    /// ```
    #[must_use]
    pub const fn with_pos(position: Vector2f) -> Self {
        Self::new(position, Self::DEFAULT.color, Self::DEFAULT.tex_coords)
    }

    /// Create a new Vertex with the position and the color
    ///
    /// # Arguments
    /// * position - Position of the vertex
    /// * color - Color of the vertex
    ///
    /// # Default
    /// * `tex_coords` - (0., 0)
    ///
    /// Return a Vertex
    ///
    /// ```
    /// # use sfml::graphics::{Color, Vertex};
    /// # use sfml::system::Vector2f;
    /// let vertex = Vertex::with_pos_color(Vector2f::new(1., 1.), Color::BLUE);
    /// assert_eq!(vertex.color, Color::BLUE);
    /// assert_eq!(vertex.tex_coords, Vector2f::new(0., 0.));
    /// ```
    #[must_use]
    pub const fn with_pos_color(position: Vector2f, color: Color) -> Vertex {
        Self::new(position, color, Self::DEFAULT.tex_coords)
    }

    /// Create a new Vertex with the position and the texture coordinates
    ///
    /// # Arguments
    /// * position - Position of the vertex
    /// * `tex_coords` - Texture coordinate of the vertex
    ///
    /// # Default
    /// * color - white
    ///
    /// Return a Vertex
    ///
    /// ```
    /// # use sfml::graphics::{Color, Vertex};
    /// # use sfml::system::Vector2f;
    /// let vertex = Vertex::with_pos_coords(Vector2f::new(1., 1.), Vector2f::new(0.25, 0.75));
    /// assert_eq!(vertex.color, Color::WHITE);
    /// assert_eq!(vertex.tex_coords, Vector2f::new(0.25, 0.75));
    /// ```
    #[must_use]
    pub const fn with_pos_coords(position: Vector2f, tex_coords: Vector2f) -> Vertex {
        Self::new(position, Self::DEFAULT.color, tex_coords)
    }

    /// Const default for `Vertex`
    ///
    /// Workaround for <https://github.com/rust-lang/rust/issues/67792>
    pub const DEFAULT: Vertex =
        Self::new(Vector2f::new(0., 0.), Color::WHITE, Vector2f::new(0., 0.));
}

/// Create a new default `Vertex`
///
/// # Default
/// * `position` - (0., 0.)
/// * `color` - white
/// * `tex_coords` - (0., 0.)
///
/// Return a `Vertex`
impl Default for Vertex {
    fn default() -> Vertex {
        Self::DEFAULT
    }
}

/// Borrow the `len` vertices starting at `data`
///
/// # Safety
///
/// If `len` is not 0, `data` must point to `len` valid vertices that are not accessed through
/// any other reference for the lifetime `'a`.
pub(crate) unsafe fn slice_from_raw_parts_mut<'a>(
    data: *mut Vertex,
    len: usize,
) -> &'a mut [Vertex] {
    if len == 0 {
        // An empty C++ vector may have a null data pointer, which slices don't allow
        &mut []
    } else {
        unsafe { std::slice::from_raw_parts_mut(data, len) }
    }
}

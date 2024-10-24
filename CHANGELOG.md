# Changelog

## 0.24.0

### Added

- Add `cpp::CppString` and `cpp::CppVector` to deal with C++ strings and vectors returned by SFML
- Add provided method `Shape::points` to get an iterator over the points of a shape
- Add `From<T> for Vector2<T>`. This allows things like `my_sprite.set_scale(2.0)`,
  to scale to `2.0` in both axes.
- Add `Image::new_solid` to create an image filled with a solid color
- Add `Font::new` to create an empty font
- Add `Window::new` to create a closed window
- Add `RenderTexture::recreate` to recreate a `RenderTexture` with new settings

### Changed

- Rust requirement increased to 1.82
- Rename `SfBox` to `FBox` to emphasize the fact it doesn't only deal with SFML objects, but
  multiple kinds of foreign objects. (F stands for "foreign")
- All methods on `system::Time` are now `const fn`
- Rename `Transform::get_matrix` to `Transform::matrix`
- Make `Transform::{new, matrix, inverse}` `const fn`
- `Image::new` now creates an empty image. See `Image::new_solid`
- Old `Window::new` is now `Window::new_open`
- `SoundRecorder` now requires `Send`
- Make `RcFont::texture` return `Ref` rather than "leak" a borrow
- Make all fields on `RenderStates` public
- `Texture::upate_from_pixels` is now a safe, checked function

### Fixed

- Fix building on Mac OS X
- Fix `Music::from_memory` having unbounded lifetime on the memory

### Removed

- `ConvexShape::points` in favor of `Shape::points`
- `RenderStates::{new, set_texture, set_shader}` in favor of public fields

### Documentation and examples

- Remove awkward `example_res!` macro from examples in favor of calling
  `example_ensure_right_working_dir()` at the beginning of main.
- Improve some examples, showcase more API usages in them
- Leverage "scrape examples" functionality of rustdoc in docs.rs generated documentation
- Add "positional audio" example

### Internal improvements

- Added continuous integration for Windows MSVC, and Mac OS X

## 0.23.0

### Changed

- ⚠️ Changed build process to **statically link** as much of SFML as possible. If you get a build error, open an issue!
- Rework `LoadResult` to `SfResult` as a more generic error type.
- Use `SfResult` in a lot of APIs that previously returned `Option`, `bool`, or panicked on error.
- Fixed some methods on `Image`, `RenderWindow`, `RenderTexture` and `Window` to properly take `&mut self` instead of `&self`.
- Examples now use `?` for error handling rather than `.unwrap()`.
- `SoundStream::get_data` now returns an immutable slice, as it should.
- `SoundStream` now requires `Send` bound, as the samples are polled from a different thread.
- Renamed `Image::create_from_pixels` to `Image::from_pixels`

### Removed

- Remove `SfResource` trait. It's not required anymore.
- Remove `Default` impl for clock because `Clock::start` can fail.

### Added

- `Texture::from_image` convenience method.
- Clock example. Mostly just to have an example that only links against the system submodule.

### Improved

- Improve `Debug` impl for `SfBox` and opaque SFML types

### Fixed

- Fixed docs.rs build failing due to linking shenanigans. The new static linking model fixes this.
- Fixed potential unsoundness of `SoundStreamPlayer` holding a `&mut SoundStream` simultaneously with SFML
  using it from another thread.

### Documentation

- Experiment with splitting up methods into different `impl` blocks, for better organization.
  See `RenderWindow`'s documentation for example.
- Various minor doc fixes.

## 0.22.0

### Added

- Added `is_rgb` method for `Texture`, `RenderWindow`, and `RenderTexture`
- Add `RcTexture::raw_texture` to get the underlying `Texture` of an `RcTexture`
- Add direction arrow cursors from SFML 2.6 to `CursorType`
- Add `Scancode` API from SFML 2.6.
- Add tip about environment variables to README

### Changed

- `Image::set_pixel` and `Image::pixel_at` are now safe, checked functions. Added unsafe unchecked variants.
- Renamed `SetPixelError` to `PixelAccessError`
- Rust requirement bumped to 1.81
- Made `Context::get_function` a safe function

### Fixed

- Minor doc fixes, including typos
- Fixed compilation error on Windows MinGW

## 0.21.0

### SFML 2.6 update

rust-sfml 0.21 requires SFML 2.6. It will not work with 2.5.

### Added
- Added `RcFont` and `RcText` for reference counted text (see `examples/rc-resources.rs`)
- pub const DEFAULT for `Vertex`
- `is_smooth` and `set_smooth` methods for `Font`
- `RenderWindow::recreate` method to recreate a window with new parameters

### Changed
- Update `RcSprite` and `RcTexture` documentation.

### Fixed
- Missing `#include`s in CSFML
- Too restrictive lifetime for Music

## 0.20.0

### Added
- `Default` impl for `Color` (transparent)
- Help on using `LD_LIBRARY_PATH` for finding SFML shared libraries on *nix.
- `RcTexture` and `RcSprite` for reference counted textures (see `examples/rc-resources.rs`)

### Changed
- Removed `Sprite::disable_texture`, due to being an unsound API
- Methods that used to take `&Rect` now take `Rect` by value
- `listener::set_up_vector` now Takes `Vector3f` instead of `&Vector3f`
- Use `usize` instead of `u32` for `CustomShapePoints`
- Use `usize` instead of `u32` for `Shape` methods
- Use `usize` instead of `u32` in `CircleShape` and `ConvexShape` methods

### Internal improvements
- Remove needless raw conversion methods for Rect
- Remove needless raw conversion methods for Vector2/Vector3

## 0.19.0

### Added
- Basic support for statically linking SFML

### Fixed

- Wrong doc comment for RenderTexture::new
- Joystick axis was inacessible in the JoystickMoved event

### Changed
- `joystick::Axis` is now a proper enum

## 0.18.1

### Fixed
- Fix broken feature combinations (audio, graphics, system)

### Internal improvements
- Test non-default feature combinations in CI

## 0.18.0

### Added
- Examples for Vector3
- Examples for Vector2
- Examples for Rect
- Examples for RenderWindow
- Examples for Font
- Info about `SFML_INCLUDE_DIR` and `SFML_LIBS_DIR` environment variables

### Changed
- `window::clipboard::get_string()` now returns `String` instead of `&'static SfStr`
- `Color` now has public fields, removed the unnecessary getter/setter methods.

### Fixed
- Update requirements in the crate documentation
- Make `set_mouse_cursor` unsafe, as the cursor must stay alive while in use.
- Fix wrong `Vector3::{AddAssign, SubAssign}` impls
- Add `Hash` impl for `CursorType`

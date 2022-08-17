# Changelog

## [Unreleased]

### Added
- Examples for Vector3
- Examples for Vector2
- Examples for Rect
<<<<<<< HEAD
- Examples for RenderWindow
=======
- Examples for Render Window
- Examples for Font

### Changed
- `window::clipboard::get_string()` now returns `String` instead of `&'static SfStr`
- `Color` now has public fields, removed the unnecessary getter/setter methods.
>>>>>>> 6cbc4b4f20790b3a81554c38893a0bdae5b0afd1

### Fixed
- Update requirements in the crate documentation
- Make `set_mouse_cursor` unsafe, as the cursor must stay alive while in use.
- Fix wrong `Vector3::{AddAssign, SubAssign}` impls
- Add `Hash` impl for `CursorType`

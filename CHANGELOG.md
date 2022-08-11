# Changelog

## [Unreleased]

### Added
- Examples for Vector3
- Examples for Vector2
- Examples for Rect
- Examples for Font

### Fixed
- Update requirements in the crate documentation
- Make `set_mouse_cursor` unsafe, as the cursor must stay alive while in use.
- Fix wrong `Vector3::{AddAssign, SubAssign}` impls
- Add `Hash` impl for `CursorType`

Contributions to rust-sfml are most welcome!

All unassigned issues are welcome to be tackled, but there are two special labels where contribution is especially desired:
- [**Call for Contributors!!!**](https://github.com/jeremyletang/rust-sfml/labels/Call%20for%20Contributors%21%21%21) marks
  issues where the maintainers explicitly call out for contributions, most likely because the issue is too large to resolve
  alone in a reasonable time frame.
- [**Help required!!!**](https://github.com/jeremyletang/rust-sfml/labels/Help%20required%21%21%21) marks issues that the maintainers can't resolve without help from external contributors.
  This is most likely to happen for issues involving platform-specific problems, where the maintainers can't test
  on that specific platform.

Git hooks
---------

This repository ships a pre-commit hook in `.githooks/`. Enable it once per clone with:

```sh
git config core.hooksPath .githooks
```

The hook runs `rustfmt` over all tracked Rust files that have no unstaged
changes and stages any formatting changes, so it never stages edits you did not
mean to commit. The generated `src/ffi/*_bindgen.rs` files carry
`#[rustfmt::skip]`, so rustfmt leaves them untouched. The hook does nothing if
`rustfmt` is not installed.

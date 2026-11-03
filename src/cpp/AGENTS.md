# AGENTS.md — `src/cpp/`

Rust wrappers for C++ objects returned by SFML (owned boxes, strings, vectors). `cpp` is a public low-level module that is also used internally by the safe API.

## Files

- `mod.rs`: module root. Declares the submodules and re-exports `FBox`, `CppString`, and `CppVector` publicly, plus the crate-private `RawDefault`, `CppVectorItem`, `CppVectorSliceable`, and `CppStringVector`.
- `fbox.rs`: `FBox<T>`, an owning pointer to a foreign-allocated object that drops it in place. Also defines the `RawDefault` trait.
- `string.rs`: `CppString`, an opaque `std::string` handle with UTF-8 access, comparison, and display.
- `vector.rs`: `CppVector<T>`, an opaque `std::vector<T>` handle, plus the `CppVectorItem` and `CppVectorSliceable` traits that describe how to access its elements.

## Keeping this file current

If you add a new file to this folder, or change an existing file enough to affect the summaries above, update this `AGENTS.md` to match.

# ZANGE: Corrections and Fixes

## 2026-02-17

### Documentation Fixes based on FIXME.md

1.  **Updated `docs/spec/effects.md`**:
    -   Added a **Ports and Handlers** section. This documents the syntax and semantics of `port` definitions (interfaces) and `handler` definitions (implementations), filling the gap identified in `FIXME.md`. It clarifies that the current implementation uses static global dispatch for these constructs.

## 2026-02-17 (2)
### Documentation: Standard Library
- **Problem**: Missing documentation for standard library functions implemented in `src/interpreter/stdlib.rs`.
- **Resolution**: Created `docs/spec/stdlib.md` documenting `print`, type conversions, and linear resource management functions.

## 2026-02-17 (5)
### [Syntax] Changed string literal delimiter to `[=[ ... ]=]`
- **Problem**: Traditional `"` delimiters were replaced with Lua-style long strings for better readability and alignment with project goals.
- **Resolution**: Updated `src/parser.rs` to support `[=[ ... ]=]` strings and `\]=]` as an escape for the closing delimiter. Updated all fixtures, Rust tests, and documentation to match the new syntax.

## 2026-02-17 (3)
### Feature: Imports and FFI
- **Problem**: `FIXME.md` listed Imports and FFI as unimplemented/undecided, and documentation marked Imports as reserved syntax.
- **Resolution**: Verified implementation in `src/parser.rs`, `src/typecheck.rs`, and `src/interpreter/mod.rs`. Created `docs/spec/ffi.md` and updated `docs/spec/basics.md` to reflect the active status of Imports and External Functions.

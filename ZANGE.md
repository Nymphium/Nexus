# ZANGE: Corrections and Fixes

## 2026-02-17

### String and Output Improvements

1.  **Implemented String Concatenation (`++`)**:
    -   Added the `++` operator for string concatenation.
    -   Updated the parser, typechecker, and interpreter to support `++` specifically for `Type::Str`.
    -   Ensured consistency with other arithmetic operators (`+` for `i64`, `+.` for `float`).

2.  **Removed `printf` and Unrestricted Polymorphism**:
    -   Removed `printf` (limited functionality), `to_string<T>`, and `print<T>` (dangerous unrestricted polymorphism).
    -   Implemented explicit, type-safe conversion functions: `int_to_string`, `float_to_string`, `bool_to_string`.
    -   Restricted `print` to accept only `str` arguments (`print(val: str)`).
    -   Updated all tests and docs to use explicit conversions and string concatenation.

3.  **REPL Completion and Visibility**:
    -   Implemented tab-completion for stdlib and user variables.
    -   Added `:vars` command to inspect the environment.
    -   Registered stdlib functions as first-class `Value::NativeFunction` values.

4.  **Strict Argument Label Enforcement**:
    -   Refactored `Type::Arrow` to include parameter names.
    -   Enforced strict label matching at call sites.
    -   Standardized stdlib labels (mostly `val`).

5.  **Strict Effect Keywords**:
    -   Forbid `perform` for pure function calls.
    -   Enforced `effect { IO }` declaration for functions performing IO.

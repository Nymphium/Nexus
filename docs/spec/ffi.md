# Foreign Function Interface (FFI)

Nexus supports interfacing with WebAssembly (Wasm) modules, allowing developers to extend the language with functions written in languages like Rust, C, or C++.

## Loading Wasm Modules

To load a Wasm module, use the `import external` directive.

```nexus
import external "math.wasm"
```

This loads the specified Wasm file into the runtime.

## Binding External Functions

Once a module is loaded (or if the function is available in the global Wasm store), you can bind it to a Nexus function using the `external` keyword.

```nexus
external add_ints: (a: i64, b: i64) -> i64 = "add"
```

- **Name**: The name of the function in Nexus (`add_ints`).
- **Type**: The type signature. It must be an arrow type.
- **Wasm Symbol**: The string literal `"add"` specifies the name of the exported function in the Wasm module.

## Supported Types

Currently, the FFI supports basic types that map directly to Wasm types:

- `i64` -> `i64`
- `float` -> `f64`
- `i32` (via `i64` casting)
- `f32` (via `float` casting)

## Example

```nexus
import external "utils.wasm"

external process_data: (val: float) -> float = "process"

fn main() -> unit do
  let result = perform process_data(val: 42.0)
  perform print_float(val: result)
endfn
```

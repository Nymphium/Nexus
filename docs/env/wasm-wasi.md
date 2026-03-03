# WebAssembly and WASI

Nexus is designed to be a WebAssembly (Wasm) native language. It leverages the Wasm Component Model and WASI (WebAssembly System Interface) to provide a secure, capability-based execution environment.

## Coeffects and Permissions

Nexus uses a [Coeffect system](../spec/effect_system.md) to manage environment requirements. Unlike traditional languages where any function can perform I/O, Nexus requires functions to explicitly declare their requirements using the `require { ... }` clause.

At the top level of a program, the `main` function must declare the set of "Root Permissions" it requires from the host environment. These are prefixed with `Perm` (e.g., `PermConsole`).

### Current Mapping to WASI (Planned and Implemented)

The Nexus runtime maps language-level permissions to WASI capabilities. Some of these are currently checked statically but not yet strictly enforced at the runtime level.

| Nexus Permission | Description | WASI Capability / Interface | Status |
|:---|:---|:---|:---|
| `PermConsole` | Standard Input/Output/Error | `stdin`, `stdout`, `stderr` | Statically checked; runtime enforcement planned |
| `PermFs` | Filesystem access | `wasi:filesystem/preopens` | Enforced |
| `PermNet` | Outbound network access | `wasi:http/outgoing-handler`, `wasi:sockets/*` | Partially enforced |
| `PermRandom` | Random number generation | `wasi:random/random` | Statically checked only |
| `PermClock` | Wall clock and monotonic time | `wasi:clocks/wall-clock`, `monotonic-clock` | Statically checked only |
| `PermProc` | Process control and exit | `wasi:cli/exit`, `environment` | Statically checked only |

## Capability Enforcement

### Static Verification

The Nexus type checker ensures that:
1.  Any function calling a capability-requiring port (like `Console.println`) must itself `require` that capability or have it satisfied via `inject`.
2.  The `main` function's `require` clause is the source of truth for the entire program's capabilities.

### Binary Encoding

When compiling to Wasm, the required permissions are stored in a custom Wasm section named `nexus:capabilities`. This allows the Nexus loader and other tools to inspect the required permissions without executing the code.

### Runtime Enforcement (In Progress)

The Nexus runtime (implemented in Rust using `wasmtime`) is being updated to strictly enforce these permissions when setting up the WASI context. When using the CLI, these are enabled via `--allow-*` flags (see [CLI Reference](../cli.md)).

- **Filesystem Isolation**: If `PermFs` is not required, no directories are preopened.
- **Network Isolation**: If `PermNet` is not required, `inherit_network()` is not called.
- **Planned: I/O Redirection**: If `PermConsole` is missing, `stdout` and `stderr` will be connected to a null device (e.g., `/dev/null`) to prevent unauthorized information leakage. Currently, `inherit_stdio()` is called unconditionally.
- **Planned: Clock/Random Blocking**: Without `PermClock` or `PermRandom`, the host implementation of these WASI interfaces will return errors or provide dummy data.

## The Nexus Host Bridge

For certain advanced capabilities like complex HTTP networking, Nexus uses a "Host Bridge" component (`nexus_host_bridge`). This bridge translates Nexus-specific [FFI](ffi.md) calls (prefixed with `__nx_http`) into standard WASI HTTP component calls. 

If a program requires `PermNet`, the Nexus compiler automatically composes this bridge into the final Wasm Component, ensuring that the high-level `Net` port in the Nexus Standard Library works seamlessly on any WASI-compliant host.

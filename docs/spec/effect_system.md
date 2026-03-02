# Effect and Coeffect System

Nexus separates runtime side effects and environment requirements:

- `effect { ... }`: builtin runtime effects.
- `require { ... }`: coeffects (requirements on provided ports/handlers).

Both are row-like signatures tracked by the type checker.

## Function Signature Shape

```nexus
fn (args...) -> Ret require { Coeffects } effect { Effects }
```

Examples:

```nexus
let pure = fn (x: i64) -> i64 do
  return x + 1
end

let greet = fn (msg: string) -> unit require { Console } do
  Console.println(val: msg)
  return ()
end

let fetch = fn (url: string) -> string require { Net } do
  return Net.get(url: url)
end
```

## Builtin Effects

The only builtin effect is:

- `Exn`: exception raising/propagation

`try ... catch` discharges `Exn` from the protected block.

```nexus
let risky = fn () -> unit effect { Exn } do
  raise RuntimeError(msg: [=[oops]=])
end

let main = fn () -> unit do
  try
    risky()
  catch e ->
    ()
  end
  return ()
end
```

## Coeffects via Ports

`port` declarations define required capabilities (coeffects), not builtin effects.

```nexus
port Logger do
  fn info(msg: string) -> unit
end

let program = fn () -> unit require { Logger } do
  Logger.info(msg: [=[hello]=])
  return ()
end
```

## Handler and Inject

A handler value implements one port. Handlers may declare `require { ... }` to propagate runtime permission requirements:

```nexus
let console_logger = handler Logger require { Console } do
  fn info(msg: string) -> unit do
    Console.println(val: msg)
    return ()
  end
end

let mock_logger = handler Logger do
  fn info(msg: string) -> unit do return () end
end
```

Handler method bodies inherit the handler's `require` clause, so `Console.println` is available inside `console_logger`'s methods.

`inject` supplies handler values to a lexical scope and discharges matching requirements:

```nexus
import { Console }, * as stdio from nxlib/stdlib/stdio.nx

let main = fn () -> unit require { PermConsole } do
  inject stdio.system_handler do
    inject console_logger do
      program()
    end
  end
  return ()
end
```

Type checking enforces:

- Handler methods must match the corresponding port signatures.
- Handler methods must be exhaustive for the port (no missing methods).
- `inject` must reduce requirements; injecting an unused handler is a type error.

## Main Constraints

`main` is intentionally constrained:

- `() -> unit`
- `effect` must be empty (or omitted)
- `require` may contain any subset of `{ PermFs, PermNet, PermConsole, PermRandom, PermClock, PermProc }` (or be empty)

All I/O capabilities are now expressed as coeffects via ports. This maps to capability-style component execution (WASI-compatible).

## Subtyping

Nexus does not implement effect/coeffect subtyping today. Compatibility is checked by row unification.

# Standard Library

Nexus stdlib APIs are provided by `nxlib/stdlib/*.nx` modules.

Core modules:

- `nxlib/stdlib/core.nx`
- `nxlib/stdlib/stdio.nx`
- `nxlib/stdlib/fs.nx`
- `nxlib/stdlib/net.nx`
- `nxlib/stdlib/string.nx`
- `nxlib/stdlib/math.nx`
- `nxlib/stdlib/list.nx`
- `nxlib/stdlib/array.nx`
- `nxlib/stdlib/set.nx`
- `nxlib/stdlib/hashmap.nx`
- `nxlib/stdlib/random.nx`
- `nxlib/stdlib/clock.nx`
- `nxlib/stdlib/proc.nx`
- `nxlib/stdlib/result.nx`
- `nxlib/stdlib/exn.nx`

Primitive linear values are auto-dropped at scope end. Composite linear values must be consumed via pattern matching or function calls.

## Console (`stdio`)

Console I/O is provided via the `Console` port. `system_handler` requires the `PermConsole` runtime permission.

```nexus
import { Console }, * as stdio from nxlib/stdlib/stdio.nx

pub port Console do
  fn print(val: string) -> unit
  fn println(val: string) -> unit
end

pub let system_handler = handler Console require { PermConsole } do ... end
```

Usage:

```nexus
let main = fn () -> unit require { PermConsole } do
  inject stdio.system_handler do
    Console.println(val: [=[Hello]=])
  end
  return ()
end
```

Run with `nexus run --allow-console example.nx`.

Mock handlers can implement `Console` without `require` for testing (no runtime permissions needed).

## String (`string`)

### Conversions

```nexus
pub external from_i64 = [=[__nx_string_from_i64]=] : (val: i64) -> string
pub external from_float = [=[__nx_string_from_float]=] : (val: float) -> string
pub external from_bool = [=[__nx_string_from_bool]=] : (val: bool) -> string
pub external to_i64 = [=[__nx_string_to_i64]=] : (s: string) -> i64
```

## File System (`fs`)

All fs operations are defined in `port Fs` and dispatched via handler.
Use `inject fs_mod.system_handler do ... end` for real filesystem access (requires `--allow-fs` at runtime), or inject a mock handler for testing (no runtime capabilities needed).

`system_handler` declares `require { PermFs }`, so injecting it propagates the `PermFs` runtime permission to the caller's `require` row. Mock handlers without `require` need no runtime permissions.

### Query operations

```nexus
fn exists(path: string) -> bool
fn read_to_string(path: string) -> string
```

### Mutating path-level operations

These raise `RuntimeError` on failure instead of returning `bool`:

```nexus
fn write_string(path: string, content: string) -> unit effect { Exn }
fn append_string(path: string, content: string) -> unit effect { Exn }
fn remove_file(path: string) -> unit effect { Exn }
fn create_dir_all(path: string) -> unit effect { Exn }
```

### Directory listing

`read_dir` returns a list of opened file handles (subdirectories are skipped):

```nexus
fn read_dir(path: string) -> List<Handle> effect { Exn }
```

### Stateful fd operations (consume-and-return pattern)

```nexus
pub type Handle = Handle(id: i64)   // non-opaque — any handler can construct

fn open_read(path: string) -> %Handle effect { Exn }
fn open_write(path: string) -> %Handle effect { Exn }
fn open_append(path: string) -> %Handle effect { Exn }
fn read(handle: %Handle) -> { content: string, handle: %Handle }
fn fd_write(handle: %Handle, content: string) -> { ok: bool, handle: %Handle }
fn fd_path(handle: %Handle) -> { path: string, handle: %Handle }
fn close(handle: %Handle) -> unit
```

Usage pattern:
```nexus
import { Fs, Handle }, * as fs_mod from nxlib/stdlib/fs.nx

// main requires { PermFs } because system_handler declares require { PermFs }
let main = fn () -> unit require { PermFs } effect { Exn } do
  inject fs_mod.system_handler do
    Fs.write_string(path: [=[data.txt]=], content: [=[hello]=])
    let %h = Fs.open_read(path: [=[data.txt]=])
    let %r = Fs.read(handle: %h)
    match %r do case { content: content, handle: %h2 } ->
      Fs.close(handle: %h2)
      // use content
    end
  end
end
```

Run with `nexus run --allow-fs example.nx`.

`open_*` and mutating operations may raise `RuntimeError`. The `read`, `fd_write`, and `fd_path` operations consume the handle and return a new one in the result record, enabling stateless handlers (no borrow needed).

## Network (`net`)

`net` functions are capability-gated by `require { Net }`.
`system_handler` declares `require { PermNet }`, so injecting it propagates the `PermNet` runtime permission to the caller. Run with `nexus run --allow-net` for real network access.

```nexus
pub type Header = Header(name: string, value: string)
pub type Response = Response(status: i64, body: string)

pub let header = fn (name: string, value: string) -> Header do ... end
pub let request_response = fn (method: string, url: string, headers: List<Header>, body: string) -> Response require { Net } do ... end
pub let request = fn (method: string, url: string, headers: List<Header>) -> string require { Net } do ... end
pub let request_with_body = fn (method: string, url: string, headers: List<Header>, body: string) -> string require { Net } do ... end
pub let get = fn (url: string) -> string require { Net } do ... end
```

## List and Array

`List<T>` is an ADT with `Nil()` / `Cons(v: T, rest: List<T>)`.

```nexus
import as list from nxlib/stdlib/list.nx
import as array from nxlib/stdlib/array.nx

list.length(xs)
list.fold_left(xs, init, f)
list.map_rev(xs, f)
list.map(xs, f)

array.length(arr)
array.is_empty(arr)
array.get(arr, idx)
array.set(arr, idx, val)
array.head(arr)
array.last(arr)
array.fold_left(arr, init, f)
array.any(arr, pred)
array.all(arr, pred)
array.find_index(arr, pred)
array.for_each(arr, f)
array.map_in_place(arr, f)
array.filter(arr, pred)
array.partition(arr, pred)
array.zip_with(left, right, f)
array.zip(left, right)
```

## Set and HashMap

Both collections use dictionary-passed key operations (`eq`/`hash`).

```nexus
import as set from nxlib/stdlib/set.nx
import as hashmap from nxlib/stdlib/hashmap.nx

let key_ops = set.make_key_ops(eq: eq_fn, hash: hash_fn)
let s0 = set.empty(key_ops: key_ops)
let s1 = set.insert(set: s0, val: 10)

let map_ops = hashmap.make_key_ops(eq: eq_fn, hash: hash_fn)
let m0 = hashmap.empty(key_ops: map_ops)
let m1 = hashmap.put(map: m0, key: 1, value: [=[one]=])
```

## Random

Random number generation is provided via the `Random` port. `system_handler` requires the `PermRandom` runtime permission.

```nexus
import { Random }, * as rng from nxlib/stdlib/random.nx

pub port Random do
  fn next_i64() -> i64
  fn range(min: i64, max: i64) -> i64
  fn next_bool() -> bool
end

pub let system_handler = handler Random require { PermRandom } do ... end
```

Usage:

```nexus
let main = fn () -> unit require { PermRandom } do
  inject rng.system_handler do
    let n = Random.range(min: 0, max: 10)
  end
  return ()
end
```

Run with `nexus run --allow-random example.nx`.

## Clock

Time operations are provided via the `Clock` port. `system_handler` requires the `PermClock` runtime permission.

```nexus
import { Clock }, * as clk from nxlib/stdlib/clock.nx

pub port Clock do
  fn sleep(ms: i64) -> unit
  fn now() -> i64
end

pub let system_handler = handler Clock require { PermClock } do ... end
```

Usage:

```nexus
let main = fn () -> unit require { PermClock } do
  inject clk.system_handler do
    let t = Clock.now()
    Clock.sleep(ms: 100)
  end
  return ()
end
```

Run with `nexus run --allow-clock example.nx`.

## Process (`proc`)

Process control is provided via the `Proc` port. `system_handler` requires the `PermProc` runtime permission.

```nexus
import { Proc }, * as proc_mod from nxlib/stdlib/proc.nx

pub port Proc do
  fn exit(status: i64) -> unit
end

pub let system_handler = handler Proc require { PermProc } do ... end
```

Usage:

```nexus
let main = fn () -> unit require { PermProc } do
  inject proc_mod.system_handler do
    Proc.exit(status: 0)
  end
end
```

Run with `nexus run --allow-proc example.nx`.

## Result Helpers

`result.nx` provides helpers and Exn bridges.

```nexus
import as result from nxlib/stdlib/result.nx

result.is_ok(res)
result.is_err(res)
result.unwrap_or(res, default)
result.from_exn(exn)
result.to_exn(res) // effect { Exn }
```

## Exception Utilities (`exn`)

`exn.nx` provides helpers for inspecting caught exceptions.

```nexus
import as exn from nxlib/stdlib/exn.nx

exn.to_string(exn)    // Exn -> string
exn.backtrace(exn)    // Exn -> [string]
```

`to_string` converts an `Exn` value to a human-readable string. `backtrace` returns the call-stack frames captured at the point the exception was raised (interpreter only; returns `[]` in Wasm builds).

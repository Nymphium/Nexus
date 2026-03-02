# Error Codes

Nexus compiler errors are identified by numeric codes prefixed with `E`.

- **E1xxx** — Lowering errors (`src/compiler/lower.rs`): AST → ANF translation failures.
- **E2xxx** — Code generation errors (`src/compiler/codegen.rs`): ANF → Wasm translation failures.

## E1xxx — Lowering

| Code | Description |
|------|-------------|
| E1001 | `main` function not found |
| E1002 | Reachable generic function is not supported |
| E1003 | Generic handler function is not supported |
| E1004 | Cyclic module import detected |
| E1005 | Failed to read imported module |
| E1006 | Failed to parse imported module |
| E1007 | External binding not found |
| E1008 | External function requires preceding `import external` |
| E1009 | External module file not found |
| E1010 | Imported item not found in module |
| E1011 | Unknown variable |
| E1012 | Unknown function |
| E1013 | Unknown constructor |
| E1014 | Missing constructor field |
| E1015 | Field not found |
| E1016 | Field access on non-record type |
| E1017 | Unknown type for field access |
| E1018 | Field access requires single-variant type |
| E1019 | Lowered handler function not found internally |
| E1020 | Handler does not implement required method |
| E1021 | Inject target is not a handler binding |
| E1022 | Call to generic function is not supported |
| E1023 | Function may not return a value |
| E1024 | `if` branch must return a value |
| E1025 | `match` case must return a value |
| E1026 | Unsupported statement |
| E1027 | Unsupported expression |
| E1028 | String literal match patterns not supported |
| E1029 | Record pattern target is not a record |
| E1030 | Closed record pattern must list all fields |
| E1031 | Unknown record field in pattern |
| E1032 | No handler found for port |

## E2xxx — Code Generation

| Code | Description |
|------|-------------|
| E2001 | `main` function not found in ANF program |
| E2002 | Unsupported binary operator for operand type |
| E2003 | Unsupported binary operator for operand type pair |
| E2004 | Unsupported Wasm type |
| E2005 | `unit` cannot be represented as Wasm ValType |
| E2006 | Unsupported numeric coercion |
| E2007 | Call target not found |
| E2008 | Call arity mismatch |
| E2009 | Call label mismatch |
| E2010 | Conflicting Wasm local types |
| E2011 | Object heap not enabled |
| E2012 | Cannot pack value type into object field |
| E2013 | Cannot unpack object field into type |
| E2014 | External param type not supported |
| E2015 | External return type not supported |
| E2016 | External call argument type mismatch |
| E2017 | String concat expects string operands |
| E2018 | String literals exist without memory configuration |

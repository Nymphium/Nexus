# Nexus CLI

The `nexus` command-line interface is the primary tool for running and interacting with Nexus programs.

## Usage

```bash
nexus [FILE]
```

### Running a File

To execute a Nexus source file, provide the path to the file as an argument. The CLI will parse, typecheck, and execute the `main` function.

```bash
nexus example.nx
```

### REPL Mode

If no file is provided, the CLI starts in REPL (Read-Eval-Print Loop) mode. This is useful for testing small code snippets and exploring the language.

```bash
nexus
```

- **Exit**: Type `exit` or press `Ctrl-D`.
- **History**: Use the Up and Down arrow keys to navigate through previous inputs.
- **Evaluation**: Each line is parsed as a statement or expression, typechecked, and executed immediately. The result and its type are displayed.

## Development Commands

If you are developing Nexus itself, you can use `cargo` to run the CLI:

```bash
# Run a file
cargo run -- example.nx

# Start REPL
cargo run
```

*Note: If you are using Nix, wrap these commands in `nix develop --command ...`.*

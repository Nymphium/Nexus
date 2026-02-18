use ariadne::{Color, Label, Report, ReportKind, Source};
use chumsky::prelude::*;
use rustyline::completion::{Completer, Pair};
use rustyline::error::ReadlineError;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::{Config, Helper};
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

use crate::ast::{Program, Spanned, Stmt, TopLevel};
use crate::interpreter::{Env, ExprResult, Interpreter};
use crate::parser::{parser, stmt_parser};
use crate::typecheck::TypeChecker;

struct NexusHelper {
    vars: Rc<RefCell<HashSet<String>>>,
}

impl Completer for NexusHelper {
    type Candidate = Pair;
    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &rustyline::Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Pair>)> {
        let (start, word) =
            rustyline::completion::extract_word(line, pos, None, |c| " \t\n\r(){}[],.".contains(c));
        let mut candidates = Vec::new();
        let vars = self.vars.borrow();
        for var in vars.iter() {
            if var.starts_with(word) {
                candidates.push(Pair {
                    display: var.clone(),
                    replacement: var.clone(),
                });
            }
        }
        Ok((start, candidates))
    }
}

impl Hinter for NexusHelper {
    type Hint = String;
}
impl Highlighter for NexusHelper {}
impl Validator for NexusHelper {}
impl Helper for NexusHelper {}

enum ReplInput {
    Stmt(Spanned<Stmt>),
    TopLevels(Vec<Spanned<TopLevel>>),
}

enum ParseState {
    Complete(ReplInput),
    Incomplete,
    Error(Vec<Simple<char>>),
}

fn parse_input_for_repl(input: &str) -> ParseState {
    let top_level_parser = parser();
    if let Ok(program) = top_level_parser.parse(input) {
        if !program.definitions.is_empty() {
            return ParseState::Complete(ReplInput::TopLevels(program.definitions));
        }
    }

    let stmt = stmt_parser().then_ignore(end()).parse(input);
    let top_err = parser().parse(input).err();
    let stmt_err = stmt_parser().then_ignore(end()).parse(input).err();

    if let Some(stmt) = stmt.ok() {
        return ParseState::Complete(ReplInput::Stmt(stmt));
    }

    let mut any_incomplete = false;
    if let Some(errors) = &top_err {
        any_incomplete |= is_incomplete_input(input, errors);
    }
    if let Some(errors) = &stmt_err {
        any_incomplete |= is_incomplete_input(input, errors);
    }
    if any_incomplete {
        return ParseState::Incomplete;
    }

    if let Some(errors) = stmt_err {
        return ParseState::Error(errors);
    }
    if let Some(errors) = top_err {
        return ParseState::Error(errors);
    }
    ParseState::Error(vec![])
}

fn is_incomplete_input(input: &str, errors: &[Simple<char>]) -> bool {
    if input.trim().is_empty() {
        return false;
    }
    let len = input.chars().count();
    errors.iter().any(|err| {
        let at_end = err.span().end >= len.saturating_sub(1);
        err.found().is_none() && at_end
    })
}

fn default_alias_from_path(path: &str) -> String {
    std::path::Path::new(path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or(path)
        .to_string()
}

fn register_completion_names(vars: &Rc<RefCell<HashSet<String>>>, defs: &[Spanned<TopLevel>]) {
    for def in defs {
        match &def.node {
            TopLevel::Function(func) => {
                vars.borrow_mut().insert(func.name.clone());
            }
            TopLevel::ExternalFn(ext) => {
                vars.borrow_mut().insert(ext.name.clone());
            }
            TopLevel::Port(port) => {
                for sig in &port.functions {
                    vars.borrow_mut()
                        .insert(format!("{}.{}", port.name, sig.name));
                }
            }
            TopLevel::Import(import) => {
                if import.is_external {
                    continue;
                }
                if !import.items.is_empty() {
                    for item in &import.items {
                        vars.borrow_mut().insert(item.clone());
                    }
                } else {
                    let alias = import
                        .alias
                        .clone()
                        .unwrap_or_else(|| default_alias_from_path(&import.path));
                    vars.borrow_mut().insert(alias);
                }
            }
            _ => {}
        }
    }
}

pub fn start() {
    let config = Config::builder().history_ignore_space(true).build();

    let vars = Rc::new(RefCell::new(HashSet::new()));
    let helper = NexusHelper { vars: vars.clone() };

    let mut rl =
        rustyline::Editor::<NexusHelper, rustyline::history::DefaultHistory>::with_config(config)
            .unwrap();
    rl.set_helper(Some(helper));

    let history_file = ".nexus_history";
    if rl.load_history(history_file).is_err() {
        // No history
    }

    // Initialize environment
    let mut env = Env::new();
    let stdlib_names = vec![
        "print",
        "i64_to_string",
        "float_to_string",
        "bool_to_string",
        "drop_i64",
        "drop_array",
    ];
    for name in &stdlib_names {
        vars.borrow_mut().insert(name.to_string());
    }

    let program = Program {
        definitions: vec![],
    };
    let mut interpreter = Interpreter::new(program);
    let mut checker = TypeChecker::new();
    let mut top_level_defs: Vec<Spanned<TopLevel>> = Vec::new();

    println!("Nexus REPL v0.1.0");
    println!("Type ':exit' or Ctrl-D to quit. Type ':help' for commands.");

    let mut buffer = String::new();

    loop {
        let prompt = if buffer.is_empty() { ">> " } else { ".. " };
        let readline = rl.readline(prompt);
        match readline {
            Ok(line) => {
                let line_str = line.trim_end();

                if buffer.is_empty() && line_str.starts_with(':') {
                    match line_str {
                        ":exit" | ":quit" => break,
                        ":help" => {
                            println!("Available commands:");
                            println!("  :exit, :quit  Exit the REPL");
                            println!("  :help         Show this help message");
                            println!("  :vars         Show loaded variables");
                            continue;
                        }
                        ":vars" => {
                            let v = vars.borrow();
                            let mut list: Vec<_> = v.iter().collect();
                            list.sort();
                            println!("Variables: {:?}", list);
                            continue;
                        }
                        _ => {
                            println!("Unknown command: {}", line_str);
                            continue;
                        }
                    }
                }

                if buffer.is_empty() && line_str.trim().is_empty() {
                    continue;
                }

                buffer.push_str(&line);
                buffer.push('\n');

                match parse_input_for_repl(&buffer) {
                    ParseState::Complete(input) => {
                        let _ = rl.add_history_entry(buffer.trim_end());
                        match input {
                            ReplInput::Stmt(stmt) => {
                                if let Stmt::Let { name, sigil, .. } = &stmt.node {
                                    vars.borrow_mut().insert(sigil.get_key(name));
                                }

                                match checker.check_repl_stmt(&stmt) {
                                    Ok(typ) => match interpreter.eval_repl_stmt(&stmt, &mut env) {
                                        Ok(res) => match res {
                                            ExprResult::Normal(val) => {
                                                println!("{} : {}", val, typ);
                                            }
                                            ExprResult::EarlyReturn(val) => {
                                                println!("returned {} : {}", val, typ);
                                            }
                                        },
                                        Err(e) => println!("Runtime Error: {}", e),
                                    },
                                    Err(e) => {
                                        Report::build(ReportKind::Error, "<repl>", e.span.start)
                                            .with_message(e.message.clone())
                                            .with_label(
                                                Label::new(("<repl>", e.span))
                                                    .with_message(e.message)
                                                    .with_color(Color::Red),
                                            )
                                            .finish()
                                            .print(("<repl>", Source::from(&buffer)))
                                            .unwrap();
                                    }
                                }
                            }
                            ReplInput::TopLevels(defs) => {
                                let program = Program {
                                    definitions: defs.clone(),
                                };
                                match checker.check_program(&program) {
                                    Ok(()) => {
                                        register_completion_names(&vars, &defs);
                                        top_level_defs.extend(defs);
                                        interpreter = Interpreter::new(Program {
                                            definitions: top_level_defs.clone(),
                                        });
                                        println!("ok : definition");
                                    }
                                    Err(e) => {
                                        Report::build(ReportKind::Error, "<repl>", e.span.start)
                                            .with_message(e.message.clone())
                                            .with_label(
                                                Label::new(("<repl>", e.span))
                                                    .with_message(e.message)
                                                    .with_color(Color::Red),
                                            )
                                            .finish()
                                            .print(("<repl>", Source::from(&buffer)))
                                            .unwrap();
                                    }
                                }
                            }
                        }
                        buffer.clear();
                    }
                    ParseState::Incomplete => {}
                    ParseState::Error(errors) => {
                        for err in errors {
                            Report::build(ReportKind::Error, "<repl>", err.span().start)
                                .with_message(format!("{:?}", err))
                                .with_label(
                                    Label::new(("<repl>", err.span()))
                                        .with_message(format!("{}", err))
                                        .with_color(Color::Red),
                                )
                                .finish()
                                .print(("<repl>", Source::from(&buffer)))
                                .unwrap();
                        }
                        buffer.clear();
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                if buffer.is_empty() {
                    println!("CTRL-C");
                    break;
                }
                println!("Input canceled");
                buffer.clear();
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    if let Err(e) = rl.save_history(history_file) {
        println!("Error saving history: {}", e);
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_input_for_repl, ParseState, ReplInput};

    #[test]
    fn parse_complete_single_line_stmt() {
        assert!(matches!(
            parse_input_for_repl("let x = 1"),
            ParseState::Complete(ReplInput::Stmt(_))
        ));
    }

    #[test]
    fn parse_incomplete_if_stmt() {
        assert!(matches!(
            parse_input_for_repl("if true then"),
            ParseState::Incomplete
        ));
    }

    #[test]
    fn parse_complete_multi_line_if_stmt() {
        let src = "if true then\n  let x = 1\nelse\n  let x = 2\nendif";
        assert!(matches!(
            parse_input_for_repl(src),
            ParseState::Complete(ReplInput::Stmt(_))
        ));
    }

    #[test]
    fn parse_complete_top_level_fn() {
        let src = "fn id(x: i64) -> i64 do\n  return x\nendfn";
        assert!(matches!(
            parse_input_for_repl(src),
            ParseState::Complete(ReplInput::TopLevels(_))
        ));
    }

    #[test]
    fn parse_complete_top_level_block_comment() {
        assert!(matches!(
            parse_input_for_repl("/* top-level comment */"),
            ParseState::Complete(ReplInput::TopLevels(_))
        ));
    }

    #[test]
    fn parse_complete_multi_line_if_with_block_comment_stmt() {
        let src =
            "if true then\n  /* block\n     comment */\n  let x = 1\nelse\n  let x = 2\nendif";
        assert!(matches!(
            parse_input_for_repl(src),
            ParseState::Complete(ReplInput::Stmt(_))
        ));
    }

    #[test]
    fn parse_syntax_error_when_not_incomplete() {
        assert!(matches!(
            parse_input_for_repl("let = 1"),
            ParseState::Error(_)
        ));
    }
}

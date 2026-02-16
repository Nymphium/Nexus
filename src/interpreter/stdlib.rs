use crate::interpreter::{Value, ExprResult};

pub fn handle_call(func: &str, args: &[Value]) -> Option<Result<ExprResult, String>> {
    match func {
        "print" => {
            if args.len() != 1 {
                return Some(Err("print requires exactly 1 argument".to_string()));
            }
            if let Value::String(s) = &args[0] {
                println!("{}", s);
                Some(Ok(ExprResult::Normal(Value::Unit)))
            } else {
                Some(Err("print requires a string".to_string()))
            }
        },
        "int_to_string" => {
            if args.len() != 1 { return Some(Err("int_to_string requires 1 arg".to_string())); }
            match &args[0] {
                Value::Int(i) => Some(Ok(ExprResult::Normal(Value::String(i.to_string())))),
                _ => Some(Err("Expected int".to_string())),
            }
        },
        "float_to_string" => {
            if args.len() != 1 { return Some(Err("float_to_string requires 1 arg".to_string())); }
            match &args[0] {
                Value::Float(f) => Some(Ok(ExprResult::Normal(Value::String(f.to_string())))),
                _ => Some(Err("Expected float".to_string())),
            }
        },
        "bool_to_string" => {
            if args.len() != 1 { return Some(Err("bool_to_string requires 1 arg".to_string())); }
            match &args[0] {
                Value::Bool(b) => Some(Ok(ExprResult::Normal(Value::String(b.to_string())))),
                _ => Some(Err("Expected bool".to_string())),
            }
        },
        "drop_i64" => {
            if args.len() != 1 {
                return Some(Err("drop_i64 requires exactly 1 argument".to_string()));
            }
            // Logic to check linearity is in typechecker. Runtime just ignores it or frees if it were C.
            // Nexus runtime uses ref counting (Rc), so dropping reference here is enough if it was owned.
            Some(Ok(ExprResult::Normal(Value::Unit)))
        },
        "drop_array" => {
            if args.len() != 1 {
                return Some(Err("drop_array requires exactly 1 argument".to_string()));
            }
            Some(Ok(ExprResult::Normal(Value::Unit)))
        },
        _ => None
    }
}

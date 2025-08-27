use std::process::Command;

use crate::ast2::environment::Value;

use super::Expression;

pub fn script(script: Box<String>) -> Expression {
    Box::new(move |_ctx, _scope| {
        let result = Command::new("bash")
            .arg("-c")
            .arg(script.to_string())
            .output();
        match result {
            Ok(output) => Value::String(String::from_utf8_lossy(&output.stdout).trim().to_string()),
            Err(e) => panic!("Failed to execute script '{}': {}", script, e),
        }
    })
}

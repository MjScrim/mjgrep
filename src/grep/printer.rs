use super::search::MatchResult;
use serde_json::json;

pub fn print_result(result: &MatchResult, line_number: bool) {
    if line_number {
        println!("{}:{}: {}", result.file, result.line, result.content);
    } else {
        println!("{}: {}", result.file, result.content);
    }
}

pub fn print_result_json(result: &MatchResult) {
    let output = json!({
        "file": result.file,
        "line": result.line,
        "content": result.content,
    });
    println!("{}", output);
}


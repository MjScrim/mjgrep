use std::path::Path;
use colored::*;
use regex::Regex;

pub fn print_match(path: &Path, line: &str, line_number: usize, matcher: &Regex, show_line_number: bool) {
    let file = path.display().to_string().blue();
    let line_number_str = if show_line_number {
        format!("{}:", line_number).green().to_string()
    } else {
        String::new()
    };

    let mut highlighted = String::new();
    let mut last = 0;

    for m in matcher.find_iter(line) {
        let (start, end) = (m.start(), m.end());
        highlighted.push_str(&line[last..start]);
        highlighted.push_str(&line[start..end].red().bold().to_string());
        last = end;
    }

    highlighted.push_str(&line[last..]);

    println!("{}:{}{} {}", file, line_number_str, "→".cyan(), highlighted);
}


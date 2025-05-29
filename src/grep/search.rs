use super::config::Config;
use serde::Serialize;
use tokio::fs;
use tokio::io::{AsyncBufReadExt, BufReader};
use walkdir::WalkDir;

#[derive(Serialize)]
pub struct MatchResult {
    pub file: String,
    pub line: usize,
    pub content: String,
}

pub async fn collect_files(path: &str, ext: Option<String>) -> Vec<String> {
    let mut result = vec![];

    for entry in WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        let path_str = entry.path().display().to_string();
        if let Some(ref extension) = ext {
            if path_str.ends_with(extension) {
                result.push(path_str);
            }
        } else {
            result.push(path_str);
        }
    }

    result
}

pub async fn search_file(file_path: String, config: &Config) -> Vec<MatchResult> {
    let file = match fs::File::open(&file_path).await {
        Ok(f) => f,
        Err(_) => return vec![],
    };

    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut results = vec![];
    let mut line_number = 0;

    while let Ok(Some(line)) = lines.next_line().await {
        line_number += 1;
        let haystack = if config.ignore_case {
            line.to_lowercase()
        } else {
            line.clone()
        };
        let needle = if config.ignore_case {
            config.pattern.to_lowercase()
        } else {
            config.pattern.clone()
        };

        if haystack.contains(&needle) {
            results.push(MatchResult {
                file: file_path.clone(),
                line: line_number,
                content: line,
            });
        }
    }

    results
}


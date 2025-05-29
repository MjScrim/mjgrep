use mjgrep::grep::{
    config::Config,
    search
};

use clap::Parser;
use futures::stream::{self, StreamExt};

#[tokio::main]
async fn main() {
    let config = Config::parse();
    let mut total_matches = 0;

    let files = search::collect_files(&config.path, config.ext.clone()).await;

    let results = stream::iter(files)
        .then(|file| search::search_file(file, &config))
        .collect::<Vec<_>>()
        .await;

    for matches in results.into_iter().flatten() {
        total_matches += 1;
        if config.json {
            println!("{}", serde_json::to_string(&matches).unwrap());
        } else if config.line_number {
            println!("{}:{}: {}", matches.file, matches.line, matches.content);
        } else {
            println!("{}: {}", matches.file, matches.content);
        }
    }

    if config.count {
        println!("\nTotal de ocorrências: {}", total_matches);
    }
}


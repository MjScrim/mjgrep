use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(name = "mjgrep")]
#[command(version = "0.1")]
#[command(author = "MjScrim")]
#[command(about = "Busca recursiva e assíncrona de texto")]
pub struct Config {
    
    pub pattern: String,
    
    #[arg(default_value = ".")]
    pub path: String,

    #[arg(short, long)]
    pub ignore_case: bool,

    #[arg(long)]
    pub files_with_matches: bool,

    #[arg(short, long)]
    pub line_number: bool,

    #[arg(long)]
    pub ext: Option<String>,

    #[arg(long)]
    pub json: bool,

    #[arg(long)]
    pub count: bool

}

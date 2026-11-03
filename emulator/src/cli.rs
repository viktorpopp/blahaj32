use clap::Parser;

#[derive(Parser, Clone)]
pub struct Cli {
    pub file: String,

    #[arg(short, long)]
    pub memory: Option<String>,
}

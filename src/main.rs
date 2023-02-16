extern crate comrak;
extern crate glob;

extern crate clap;
use clap::{Parser, Subcommand, ValueEnum};

mod contents;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Export {
        #[arg(short = 'f', long = "format", value_enum)]
        format: Format,

        #[arg(short = 'o', long = "output_dir", default_value = ".")]
        output_dirpath: String,
    },
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Format {
    HugoRobust,
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Export {
            format,
            output_dirpath,
        } => match format {
            Format::HugoRobust => contents::export_zenn_contents2hugo_robust(output_dirpath),
        },
    }
}

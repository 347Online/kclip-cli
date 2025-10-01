use arboard::Clipboard;
use clap::{Parser, Subcommand, ValueEnum};

#[derive(Debug, Clone, ValueEnum)]
enum Actions {
    Copy,
    Paste,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(arg_required_else_help = true)]
    Kclip { action: Actions },

    #[command()]
    Kccopy,
    #[command()]
    Kcpaste,
}

#[derive(Debug, Parser)]
#[clap(multicall(true), propagate_version(true))]
#[command(name = "kclip", version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Kclip {
            action: Actions::Copy,
        }
        | Commands::Kccopy => {
            println!("copy");
        }

        Commands::Kclip {
            action: Actions::Paste,
        }
        | Commands::Kcpaste => {
            if let Ok(text) = Clipboard::new().and_then(|mut cb| cb.get_text()) {
                print!("{text}")
            }
        }
    };
}

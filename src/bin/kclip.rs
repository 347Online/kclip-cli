use std::io::{BufRead, Write, stdin, stdout};

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let mut clipboard = Clipboard::new()?;

    match &cli.command {
        Commands::Kclip {
            action: Actions::Copy,
        }
        | Commands::Kccopy => {
            let text = stdin().lock().lines().collect::<Result<String, _>>()?;

            clipboard.set_text(text)?;
        }

        Commands::Kclip {
            action: Actions::Paste,
        }
        | Commands::Kcpaste => {
            let text = clipboard.get_text()?;

            write!(stdout().lock(), "{text}")?;
        }
    }

    Ok(())
}

use std::io::{BufRead, Write, stdin, stdout};

use anyhow::Context;
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

fn copy(cb: &mut Clipboard) -> anyhow::Result<()> {
    let text = stdin()
        .lock()
        .lines()
        .collect::<Result<String, _>>()
        .context("failed to read from stdin")?;

    cb.set_text(text)
        .context("failed write content to clipboard")?;

    Ok(())
}

fn paste(cb: &mut Clipboard) -> anyhow::Result<()> {
    let text = match cb.get_text() {
        Ok(x) => x,
        Err(arboard::Error::ContentNotAvailable) => return Ok(()),
        Err(err) => Err(err).context("failed to read contents of clipboard")?,
    };

    write!(stdout().lock(), "{text}").context("failed to write to stdout")?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let mut cb = Clipboard::new().context("failed to access clipboard")?;

    match &cli.command {
        Commands::Kclip {
            action: Actions::Copy,
        }
        | Commands::Kccopy => copy(&mut cb)?,

        Commands::Kclip {
            action: Actions::Paste,
        }
        | Commands::Kcpaste => paste(&mut cb)?,
    }

    Ok(())
}

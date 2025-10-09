use std::io::{BufRead, Write, stdin, stdout};
use std::path::PathBuf;

use anyhow::Context;
use arboard::Clipboard;
use clap::{Arg, ArgAction, Command, command, value_parser};

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

macro_rules! applet_commands {
    ($prefix:literal) => {
        [
            Command::new(concat!($prefix, "copy"))
                .about("copies text from stdin to the system clipboard"),
            Command::new(concat!($prefix, "paste"))
                .about("pastes the contents of the system clipboard to stdout"),
        ]
    };
    () => {
        applet_commands!("")
    };
}

fn cli() -> Command {
    command!("kclip")
        .multicall(true)
        .propagate_version(true)
        .subcommand(
            command!("kclip")
                .arg_required_else_help(true)
                .subcommand_help_heading("COMMANDS")
                .arg(
                    Arg::new("install")
                        .long("install")
                        .help("Install symlink aliases")
                        .value_name("target")
                        .exclusive(true)
                        .action(ArgAction::Set)
                        .default_missing_value("/usr/local/bin")
                        .value_parser(value_parser!(PathBuf)),
                )
                .subcommands(applet_commands!()),
        )
        .subcommands(applet_commands!("kc"))
}

fn main() -> anyhow::Result<()> {
    let cmd = cli();

    let mut cb = Clipboard::new().context("failed to access clipboard")?;

    let matches = cmd.get_matches();
    let subcommand = match matches.subcommand() {
        Some(("kclip", cmd)) => {
            if cmd.contains_id("install") {
                unimplemented!();
            }
            cmd.subcommand()
        }

        x => x,
    };

    match subcommand {
        Some(("kccopy" | "copy", _)) => copy(&mut cb)?,
        Some(("kcpaste" | "paste", _)) => paste(&mut cb)?,

        _ => unreachable!("parser should ensure only valid names are used"),
    }

    Ok(())
}

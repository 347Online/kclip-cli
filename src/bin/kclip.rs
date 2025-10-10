use anyhow::Context;
use app_path::app_path;
use arboard::Clipboard;
use clap::{Arg, Command, command, value_parser};
use std::io::{BufRead, Write, stdin, stdout};
use std::path::{Path, PathBuf};
use symlink::symlink_file;

fn copy(cb: &mut Clipboard) -> anyhow::Result<()> {
    let text = stdin()
        .lock()
        .lines()
        .collect::<Result<String, _>>()
        .context("Failed to read from stdin")?;

    cb.set_text(text)
        .context("Failed write content to clipboard")?;

    Ok(())
}

fn paste(cb: &mut Clipboard) -> anyhow::Result<()> {
    let text = match cb.get_text() {
        Ok(x) => x,
        Err(arboard::Error::ContentNotAvailable) => return Ok(()),
        Err(err) => Err(err).context("Failed to read contents of clipboard")?,
    };

    write!(stdout().lock(), "{text}").context("Failed to write to stdout")?;

    Ok(())
}

macro_rules! applet_commands {
    ($prefix:expr) => {
        [
            Command::new(concat!($prefix, "copy"))
                .about("Copies text from stdin to the system clipboard"),
            Command::new(concat!($prefix, "paste"))
                .about("Pastes the contents of the system clipboard to stdout"),
        ]
    };
    () => {
        applet_commands!("")
    };
}

macro_rules! alias_prefix {
    () => {
        "kc"
    };
}

fn install(target: &Path) -> anyhow::Result<()> {
    let src = std::env::current_exe()?;

    let commands = applet_commands!(alias_prefix!());
    let total = commands.len();
    let mut succeeded = 0;

    for cmd in commands {
        let alias = target.join(cmd.get_name());

        match symlink_file(&src, &alias)
            .context(format!("Failed to symlink {:?} -> {:?}", &src, alias))
        {
            Ok(_) => succeeded += 1,
            Err(err) => {
                eprintln!("{err}");
                if let Some(cause) = err.source() {
                    eprintln!("   {cause}");
                }
            }
        }
    }

    if succeeded == total {
        println!("All aliases installed successfully");
    } else {
        println!("{succeeded}/{total} aliases installed successfully");
        std::process::exit(1);
    }

    Ok(())
}

fn cli(app_dir: String) -> Command {
    command!("kclip")
        .multicall(true)
        .propagate_version(true)
        .subcommands(applet_commands!(alias_prefix!()))
        .subcommand(
            command!("kclip")
                .arg_required_else_help(true)
                .subcommand_help_heading("COMMANDS")
                .subcommands(applet_commands!())
                .subcommand(
                    Command::new("install")
                        .arg(
                            Arg::new("TARGET")
                                .help("Install symlink aliases to specified target")
                                .value_name("TARGET")
                                .default_value(app_dir)
                                .value_parser(value_parser!(PathBuf)),
                        )
                        .about("Install symlink aliases"),
                ),
        )
}

fn main() -> anyhow::Result<()> {
    let app_dir = app_path!().to_string();
    let cmd = cli(app_dir);

    let mut cb = Clipboard::new().context("Failed to access clipboard")?;

    let matches = cmd.get_matches();
    let subcommand = matches.subcommand().and_then(|x| {
        if let ("kclip", cmd) = x {
            cmd.subcommand()
        } else {
            Some(x)
        }
    });

    match subcommand {
        Some(("kccopy" | "copy", _)) => copy(&mut cb)?,
        Some(("kcpaste" | "paste", _)) => paste(&mut cb)?,
        Some(("install", cmd)) => {
            let target = cmd
                .get_one::<PathBuf>("TARGET")
                .expect("TARGET should always have a value");

            install(target)?;
        }

        _ => unreachable!("parser should ensure only valid names are used"),
    }

    Ok(())
}

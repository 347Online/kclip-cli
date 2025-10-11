use anyhow::Context;
use app_path::app_path;
use arboard::Clipboard;
use clap::{Arg, Command, command, value_parser};
use std::io::{BufRead, Write, stdin, stdout};
use std::path::{Path, PathBuf};
use symlink::symlink_file;

const ALIASES: [&str; 4] = ["kclip", "kccopy", "kcpaste", "kcclear"];

struct App {
    cb: Clipboard,
    cli: Command,
}

impl App {
    pub fn new() -> anyhow::Result<Self> {
        let cb = Clipboard::new().context("Failed to access clipboard")?;
        let app_dir = app_path!().to_string();

        let primary_commands = [
            Command::new("copy").about("Copies text from stdin to the system clipboard"),
            Command::new("paste").about("Pastes the contents of the system clipboard to stdout"),
            Command::new("clear").about("Clears the contents of the system clipboard"),
        ];
        let [copy, paste, clear] = primary_commands.clone();
        let aliased_commands = [
            copy.name("kccopy"),
            paste.name("kcpaste"),
            clear.name("kcclear"),
        ];

        let cli = command!("kclip")
            .multicall(true)
            .propagate_version(true)
            .subcommands(&aliased_commands)
            .subcommand(
                command!("kclip")
                    .arg_required_else_help(true)
                    .subcommand_help_heading("COMMANDS")
                    .subcommands(&primary_commands)
                    .subcommand(
                        Command::new("install")
                            .about("Install symlink aliases")
                            .arg(
                                Arg::new("TARGET")
                                    .help("Install symlink aliases to specified target")
                                    .value_name("TARGET")
                                    .default_value(&app_dir)
                                    .value_parser(value_parser!(PathBuf)),
                            ),
                    )
                    .subcommand(
                        Command::new("inspect")
                            .about("Display information about the current clipboard content"),
                    ),
            );

        Ok(App { cb, cli })
    }

    fn copy(&mut self) -> anyhow::Result<()> {
        let text = stdin()
            .lock()
            .lines()
            .collect::<Result<String, _>>()
            .context("Failed to read from stdin")?;

        self.cb
            .set_text(text)
            .context("Failed write content to clipboard")?;

        Ok(())
    }

    fn paste(&mut self) -> anyhow::Result<()> {
        let text = match self.cb.get_text() {
            Ok(x) => x,
            Err(arboard::Error::ContentNotAvailable) => return Ok(()),
            Err(err) => Err(err).context("Failed to read contents of clipboard")?,
        };

        write!(stdout().lock(), "{text}").context("Failed to write to stdout")?;

        Ok(())
    }

    fn install(&self, target: &Path) -> anyhow::Result<()> {
        let src = std::env::current_exe()?;

        let external = target.as_os_str() != app_path!().as_os_str();

        let aliases = if external {
            Vec::from(ALIASES)
        } else {
            Vec::from(&ALIASES[1..])
        };
        let total = aliases.len();
        let mut succeeded = 0;

        for cmd in aliases {
            let alias = target.join(cmd);

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

        println!("{succeeded}/{total} aliases installed successfully");

        if succeeded != total {
            std::process::exit(1);
        }

        Ok(())
    }

    fn clear(&mut self) -> anyhow::Result<()> {
        Ok(self.cb.clear()?)
    }

    fn inspect(&mut self) {
        let repr = if let Ok(_paths) = self.cb.get().file_list() {
            "files"
        } else if let Ok(_image) = self.cb.get().image() {
            "image"
        } else if let Ok(_html) = self.cb.get().html() {
            "html"
        } else if let Ok(_text) = self.cb.get().text() {
            "text"
        } else {
            "<empty>"
        };

        println!("{repr}");
    }

    pub fn invoke(mut self) -> anyhow::Result<()> {
        let cli = std::mem::take(&mut self.cli);
        let matches = cli.get_matches();

        let subcommand = matches.subcommand().and_then(|x| {
            if let ("kclip", cmd) = x {
                cmd.subcommand()
            } else {
                Some(x)
            }
        });

        match subcommand {
            Some(("kccopy" | "copy", _)) => self.copy()?,
            Some(("kcpaste" | "paste", _)) => self.paste()?,
            Some(("kcclear" | "clear", _)) => self.clear()?,
            Some(("inspect", _)) => self.inspect(),
            Some(("install", cmd)) => {
                let target = cmd
                    .get_one::<PathBuf>("TARGET")
                    .expect("TARGET should always have a value");

                self.install(target)?;
            }

            _ => unreachable!("parser should ensure only valid names are used"),
        }

        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    let app = App::new()?;

    app.invoke()
}

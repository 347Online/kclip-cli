use clap::Command;

fn main() {
    let mut cmd = Command::new("kclip")
        .multicall(true)
        .subcommand(
            Command::new("kclip")
                .version(env!("CARGO_PKG_VERSION"))
                .arg_required_else_help(true),
        )
        .subcommand(Command::new("kccopy"))
        .subcommand(Command::new("kcpaste"));

    let matches = cmd.get_matches();
}

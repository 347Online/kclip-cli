use clap::{Parser, Subcommand, ValueEnum};

#[derive(Clone, ValueEnum)]
enum Actions {
    Copy,
    Paste,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "kclip", arg_required_else_help = true)]
    Kclip {
        #[arg()]
        action: Actions,
    },

    #[command(name = "kccopy")]
    Kccopy,
    #[command()]
    Kcpaste,
}

#[derive(Parser)]
#[clap(multicall(true), propagate_version(true))]
#[command(name = "kclip", version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() {
    let cli = Cli::parse();
}

use clap::Parser;

mod wallet;

#[derive(Debug, Parser)]
struct CliArgs {
    /// Which comamnd to execute. (e.g. "new_wallet" to generate new key pairs for sol wallet)
    #[arg(
        help_heading = Some("Command"),
        short,
        value_name="COMMAND",
        default_value = ""
    )]
    command: String,
}

// main entry point to cli app from main
pub fn run() {}

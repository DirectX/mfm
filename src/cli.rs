use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(version)]
#[command(about = "Media File Manager is a CLI application to catalog media files based on all metadata available")]
pub(crate) struct MFMArgs {
    #[clap(subcommand)]
    pub(crate) command: CommandType,
}

#[derive(Debug, Subcommand)]
pub(crate) enum CommandType {
    /// Imports media files from the given input_path placing them to the --output_path
    Import(ImportCommand),
}

#[derive(Debug, Args)]
pub(crate) struct ImportCommand {
    /// Input path
    pub input_path: String,
    /// Output path
    pub output_path: String,
    /// Do not traverse input directory
    #[arg(long, default_value = "false")]
    pub no_traverse: bool,
}

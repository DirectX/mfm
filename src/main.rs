use clap::Parser;

use crate::cli::{CommandType, MFMArgs};

pub mod cli;
pub mod exif;
pub mod utils;
pub mod import;

#[tokio::main]
async fn main() {
    let res = run().await;
    match res {
        Err(err) => log::error!("Error: {}", err),
        Ok(_) => log::info!("Done"),
    }
}

async fn run() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    pretty_env_logger::env_logger::builder().init();

    let args = MFMArgs::parse();
    log::debug!("Args: {:?}", args);

    let cancellation_token = tokio_util::sync::CancellationToken::new();
    let import_cancellation_token = cancellation_token.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        log::info!("\nShutting down...");
        cancellation_token.cancel();
    });


    match args.command {
        CommandType::Import(import_command) => import::import(import_cancellation_token, import_command.input_path, import_command.output_path, import_command.no_traverse).await?,
    }

    Ok(())
}

use clap::Parser;

use crate::cli::{CommandType, MFMArgs};

pub mod cli;
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

    match args.command {
        CommandType::Import(_import_command) => import::import().await?,
    }

    Ok(())
}

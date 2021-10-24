#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![deny(clippy::all)]
#![warn(rust_2018_idioms)]
#![warn(
    clippy::pedantic,
    //missing_debug_implementations
)]

//! # download-service

use log::info;

// arguments
use args::Arguments;

// logger
use logs::setup_logger;

// services
use download::DownloadService;

// servers
use download_api::download_server::DownloadServer;
use tonic::transport::Server;

// general modules
mod args;
mod errors;
mod logs;

// cqrs impl modules
mod download;

#[path = "../proto/download-api.rs"]
mod download_api;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Arguments::get();

    setup_logger(
        args.debug,
        args.stdout_log,
        args.log_file.clone(),
    )
    .unwrap();

    info!("{:?}", &args);

    let addr = format!("0.0.0.0:{}", args.service_port)
        .parse()
        .unwrap();

    let download_service = DownloadService::default();
    info!("Server listening on {}", addr);

    Server::builder()
        .add_service(DownloadServer::new(download_service))
        .serve(addr)
        .await?;

    Ok(())
}

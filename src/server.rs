mod converter;
mod heif;
mod services;

use dotenv::dotenv;
use tracing::info;

use crate::services::{ConvertServer, ConvertService, InfoServer, InfoService, Server};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    info!("Starting HEIF conversion server");

    let default_addr = String::from("127.0.0.1:50051");
    let addr = std::env::var("GRPC_SERVER_ADDRESS")
        .unwrap_or(default_addr)
        .parse()?;

    let convert = ConvertService::default();
    let info = InfoService::default();

    Server::builder()
        .add_service(ConvertServer::new(convert))
        .add_service(InfoServer::new(info))
        .serve(addr)
        .await?;

    Ok(())
}

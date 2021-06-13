mod heif;
mod services;

#[macro_use] extern crate log;
use dotenv::dotenv;

use crate::services::{Server, ConvertService, ConvertServer, InfoService, InfoServer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    env_logger::init();

    info!("Starting HEIF conversion server");

    let default_addr = String::from("127.0.0.1:50051");
    let addr = std::env::var("GRPC_SERVER_ADDRESS").unwrap_or(default_addr).parse()?;

    let convert = ConvertService::default();
    let info = InfoService::default();

    Server::builder()
        .add_service(ConvertServer::new(convert))
        .add_service(InfoServer::new(info))
        .serve(addr)
        .await?;

    Ok(())
}
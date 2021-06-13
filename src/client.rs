#[macro_use] extern crate log;
use dotenv::dotenv;

use tonic::{transport::Server, Request, Response, Status};

use heif_api::{ConvertJpegRequest, ConvertJpegResponse};
use heif_api::convert_client::ConvertClient;

pub mod heif_api {
    tonic::include_proto!("heif_api");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    env_logger::init();

    info!("Starting HEIF conversion server");

    let default_addr = String::from("127.0.0.1:50051");
    let addr = std::env::var("GRPC_SERVER_ADDRESS").unwrap_or(default_addr);

    let default_scheme = String::from("http");
    let scheme = std::env::var("GRPC_SERVER_SCHEME").unwrap_or(default_scheme);

    let endpoint = scheme + "://" + &addr;

    let mut client = ConvertClient::connect(endpoint).await?;

    let request = tonic::Request::new(ConvertJpegRequest {
        heif: vec![0],
        quality: 85
    });

    let response = client.convert_jpeg(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
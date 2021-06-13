#[macro_use] extern crate log;
use dotenv::dotenv;

use tokio::fs::File;
use tonic::{transport::Server, Request, Response, Status};

use heif_api::{GetInfoRequest, GetInfoResponse};
use heif_api::info_client::InfoClient;
use tokio::io::AsyncReadExt;

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

    let mut client = InfoClient::connect(endpoint).await?;

    let mut contents = vec![];
    //tokio::fs::File::open("./data/test.heic").await?
    //tokio::fs::File::open("./data/4_chunks-wo_exif.heic").await?
    tokio::fs::File::open("./data/nokia/overlay_grid_alpha/alpha_1440x960.heic").await?
        .read_to_end(&mut contents).await?;

    let request = tonic::Request::new(GetInfoRequest {
        heif: contents
    });

    let response = client.get_info(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
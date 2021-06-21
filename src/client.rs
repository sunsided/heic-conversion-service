use dotenv::dotenv;

use tonic::{transport::Server, Request, Response, Status};

use heif_api::convert_client::ConvertClient;
use heif_api::info_client::InfoClient;
use heif_api::{ConvertToJpegRequest, ConvertToJpegResponse, GetInfoRequest, GetInfoResponse};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tracing::info;

pub mod heif_api {
    tonic::include_proto!("heif_api");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    info!("Starting HEIF conversion server");

    let default_addr = String::from("127.0.0.1:50051");
    let addr = std::env::var("GRPC_SERVER_ADDRESS").unwrap_or(default_addr);

    let default_scheme = String::from("http");
    let scheme = std::env::var("GRPC_SERVER_SCHEME").unwrap_or(default_scheme);

    let endpoint = scheme + "://" + &addr;

    let mut info_client = InfoClient::connect(endpoint.clone()).await?;
    let mut convert_client = ConvertClient::connect(endpoint).await?;

    let mut contents = vec![];
    //tokio::fs::File::open("./data/test.heic").await?
    //tokio::fs::File::open("./data/4_chunks-wo_exif.heic").await?
    //tokio::fs::File::open("./data/dpreview/IMG_0115.heic").await?
    tokio::fs::File::open("./data/test.heic")
        .await?
        .read_to_end(&mut contents)
        .await?;

    let info_request = tonic::Request::new(GetInfoRequest {
        heif: contents.clone(),
    });
    let info_response = info_client.get_info(info_request).await?;
    println!("RESPONSE={:?}", info_response);

    let convert_request = tonic::Request::new(ConvertToJpegRequest {
        heif: contents,
        quality: 65,
    });
    let convert_response = convert_client.convert_to_jpeg(convert_request).await?;

    tokio::fs::File::create("./data/test.out.jpg")
        .await?
        .write_all(&mut convert_response.into_inner().jpeg)
        .await?;

    Ok(())
}

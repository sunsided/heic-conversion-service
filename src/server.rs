#[macro_use] extern crate log;
use dotenv::dotenv;

use tonic::{transport::Server, Request, Response, Status};

use heif_api::{ConvertJpegRequest, ConvertJpegResponse};
use heif_api::convert_server::{Convert, ConvertServer};

pub mod heif_api {
    tonic::include_proto!("heif_api");
}

#[derive(Debug, Default)]
pub struct ConvertService {}

#[tonic::async_trait]
impl Convert for ConvertService {
    async fn convert_jpeg(
        &self,
        request: Request<ConvertJpegRequest>,
    ) -> Result<Response<ConvertJpegResponse>, Status> {
        println!("Got a request: {:?}", request);

        let reply = ConvertJpegResponse {
            jpeg: vec![0]
            // message: format!("Hello {}!", request.into_inner().name).into(),
            // We must use .into_inner() as the fields of gRPC requests and responses are private
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    env_logger::init();

    info!("Starting HEIF conversion server");

    let default_addr = String::from("127.0.0.1:50051");
    let addr = std::env::var("GRPC_SERVER_ADDRESS").unwrap_or(default_addr).parse()?;

    let convert = ConvertService::default();

    Server::builder()
        .add_service(ConvertServer::new(convert))
        .serve(addr)
        .await?;

    Ok(())
}
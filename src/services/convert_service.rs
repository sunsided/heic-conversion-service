use crate::services::heif_api::convert_server::{Convert, ConvertServer};
use crate::services::heif_api::{ConvertJpegRequest, ConvertJpegResponse};
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct ConvertService {}

#[tonic::async_trait]
impl Convert for ConvertService {
    async fn convert_jpeg(
        &self,
        request: Request<ConvertJpegRequest>,
    ) -> Result<Response<ConvertJpegResponse>, Status> {
        println!("Got a request: {:?}", request);

        // TODO: Encoding of JPEG and PNG files is implemented e.g. at https://github.com/strukturag/libheif/blob/master/examples/heif_convert.cc
        // TODO: ... or here: https://lib.rs/crates/libheif

        let reply = ConvertJpegResponse {
            jpeg: vec![0]
            // message: format!("Hello {}!", request.into_inner().name).into(),
            // We must use .into_inner() as the fields of gRPC requests and responses are private
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }
}
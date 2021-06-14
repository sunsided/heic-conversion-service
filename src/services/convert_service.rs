use crate::services::heif_api::{convert_server::Convert, ConvertToJpegRequest, ConvertToJpegResponse};
use crate::jpeg::recode_heif_to_jpeg;
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct ConvertService {}

#[tonic::async_trait]
impl Convert for ConvertService {
    async fn convert_to_jpeg(
        &self,
        request: Request<ConvertToJpegRequest>,
    ) -> Result<Response<ConvertToJpegResponse>, Status> {
        // TODO: Add magic byte check - the first 12 are important? (see https://github.com/strukturag/libheif/blob/master/examples/heif_convert.cc)

        // TODO: Encoding of JPEG and PNG files is implemented e.g. at https://github.com/strukturag/libheif/blob/master/examples/heif_convert.cc

        let request = request.into_inner();
        let bytes = request.heif;
        let quality = num::clamp(request.quality, 0, 100) as u8;

        let _ = recode_heif_to_jpeg(&bytes, quality);

        // TODO: Repeat for depth channel

        let reply = ConvertToJpegResponse {
            jpeg: vec![0]
            // message: format!("Hello {}!", request.into_inner().name).into(),
            // We must use .into_inner() as the fields of gRPC requests and responses are private
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }
}

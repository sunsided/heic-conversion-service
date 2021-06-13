use crate::services::heif_api::info_server::{Info, InfoServer};
use crate::services::heif_api::{GetInfoRequest, GetInfoResponse};
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct InfoService {}

#[tonic::async_trait]
impl Info for InfoService {
    async fn get_info(
        &self,
        request: Request<GetInfoRequest>,
    ) -> Result<Response<GetInfoResponse>, Status> {
        println!("Got a request: {:?}", request);

        let reply = GetInfoResponse {
            width: 0,
            height: 0
            // message: format!("Hello {}!", request.into_inner().name).into(),
            // We must use .into_inner() as the fields of gRPC requests and responses are private
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }
}

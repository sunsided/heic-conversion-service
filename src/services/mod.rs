mod convert_service;
mod info_service;
pub mod heif_api;
pub use tonic::transport::Server;
pub use heif_api::convert_server::ConvertServer;
pub use heif_api::info_server::InfoServer;

pub use convert_service::ConvertService;
pub use info_service::InfoService;
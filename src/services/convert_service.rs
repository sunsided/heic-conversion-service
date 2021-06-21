use crate::converter::{DecodingOptions, Encoder, JpegEncoder};
use crate::services::heif_api::{
    convert_server::Convert, ConvertToJpegRequest, ConvertToJpegResponse,
};
use libheif_rs::HeifContext;
use pretty_bytes::converter::convert as pretty_bytes;
use tokio::time::Instant;
use tonic::{Request, Response, Status};
use tracing::{debug, info, instrument};

#[derive(Debug, Default)]
pub struct ConvertService {}

#[tonic::async_trait]
impl Convert for ConvertService {
    #[instrument(level = "trace", skip(self))]
    async fn convert_to_jpeg(
        &self,
        request: Request<ConvertToJpegRequest>,
    ) -> Result<Response<ConvertToJpegResponse>, Status> {
        // TODO: Add magic byte check - the first 12 are important? (see https://github.com/strukturag/libheif/blob/master/examples/heif_convert.cc)

        // TODO: Encoding of JPEG and PNG files is implemented e.g. at https://github.com/strukturag/libheif/blob/master/examples/heif_convert.cc

        let request = request.into_inner();
        let bytes = request.heif;
        let num_heic_bytes = bytes.len() as f64;

        info!(
            "Handling HEIC ({size}) -> JPEG conversion request (q={quality}%)",
            size = pretty_bytes(num_heic_bytes).as_str(),
            quality = request.quality
        );

        let start = Instant::now();

        let ctx = match HeifContext::read_from_bytes(&bytes) {
            Ok(ctx) => ctx,
            Err(e) => return Err(Status::internal(e.message)),
        };

        if ctx.number_of_top_level_images() == 0 {
            return Err(Status::internal("HEIC file does not contain any images"));
        }

        // TODO: Tracing: log number of top-level images in file

        // TODO: Convert all top-level images to JPEG by iterating and streaming

        let handle = match ctx.primary_image_handle() {
            Ok(handle) => handle,
            Err(e) => return Err(Status::internal(e.message)),
        };

        let has_alpha = handle.has_alpha_channel();
        let mut decoding_options = DecodingOptions::default();

        let encoder = JpegEncoder::new(request.quality);
        encoder.update_decoding_options(&handle, &mut decoding_options);

        let bit_depth = handle.luma_bits_per_pixel();
        if bit_depth == 0 {
            return Err(Status::internal("Input image has undefined bit-depth"));
        }

        let image = match handle.decode(
            encoder.colorspace(has_alpha),
            decoding_options.ignore_transformations,
        ) {
            Ok(handle) => handle,
            Err(e) => return Err(Status::internal(e.message)),
        };

        let decoding_done = Instant::now();
        let decoding_duration = decoding_done - start;
        debug!(
            "Decoding HEIC image took {duration}",
            duration = humantime::format_duration(decoding_duration)
        );

        let bytes = match encoder.encode_to_bytes(&handle, &image) {
            Ok(handle) => handle,
            Err(_e) => return Err(Status::internal("Unable to encode the image")), // TODO: Be more specific about the error
        };

        let encoding_done = Instant::now();
        let encoding_duration = encoding_done - decoding_done;
        debug!(
            "Encoding JPEG image took {duration}",
            duration = humantime::format_duration(encoding_duration)
        );

        // TODO: Also decode the depth image

        let total_duration = encoding_done - start;
        let num_jpeg_bytes = bytes.len() as f64;
        info!(
            "Finished conversion, produced {jpeg_size} JPEG ({increase:.1}%) in {total_duration}",
            jpeg_size = pretty_bytes(num_jpeg_bytes).as_str(),
            increase = num_jpeg_bytes / num_heic_bytes * 100.,
            total_duration = humantime::format_duration(total_duration)
        );

        let reply = ConvertToJpegResponse { jpeg: bytes };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }
}

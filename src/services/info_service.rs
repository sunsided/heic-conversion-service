use crate::services::heif_api::info_server::{Info, InfoServer};
use crate::services::heif_api::{GetInfoRequest, GetInfoResponse};
use tonic::{Request, Response, Status};
use libheif_rs::{HeifContext, HeifError, HeifErrorCode, HeifErrorSubCode, ImageHandle};
use crate::services::image_info::{ImageInfo, TopLevelImageInfo, DepthImageInfo, ThumbnailImageInfo};

#[derive(Debug, Default)]
pub struct InfoService {}

#[tonic::async_trait]
impl Info for InfoService {
    async fn get_info(
        &self,
        request: Request<GetInfoRequest>,
    ) -> Result<Response<GetInfoResponse>, Status> {
        let bytes = request.into_inner().heif;
        let ctx = match HeifContext::read_from_bytes(&bytes) {
            Ok(ctx) => ctx,
            Err(e) => return Err(Status::internal(e.message)),
        };

        let mut top_level_image_ids = vec![0u32; ctx.number_of_top_level_images()];
        let image_count = ctx.top_level_image_ids(&mut top_level_image_ids);
        assert_eq!(image_count, ctx.number_of_top_level_images());

        let mut infos = Vec::with_capacity(image_count);
        for top_level_image_id in top_level_image_ids {
            let handle = match ctx.image_handle(top_level_image_id) {
                Ok(handle) => handle,
                Err(e) => return Err(Status::internal(e.message))
            };

            let top_level_image_info = InfoService::get_top_level_image_info(top_level_image_id, handle, &ctx);

            infos.push(top_level_image_info);
        }

        let reply = GetInfoResponse {
            num_images: image_count as u32,
            top_level: infos
                .into_iter()
                .map(TopLevelImageInfo::into)
                .collect()
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }
}

impl InfoService {
    fn get_image_info(handle: ImageHandle) -> ImageInfo {
        let mut thumbnail_image_ids = vec![0u32; handle.number_of_thumbnails()];
        let num_thumbnails = handle.thumbnail_ids(&mut thumbnail_image_ids);
        assert_eq!(num_thumbnails, handle.number_of_thumbnails());

        let mut depth_image_ids = vec![0u32; if handle.number_of_depth_images() > 0 { handle.number_of_depth_images() as usize } else { 0 }];
        if depth_image_ids.len() > 0 {
            let depth_image_count = handle.depth_image_ids(&mut depth_image_ids);
            assert_eq!(depth_image_count, handle.number_of_depth_images() as usize);
        }

        // Image spatial extents.
        // A bit unclear, but ISPE may differ from above width and height
        // in that the above may be the extents after transformations applied (rotation etc.)
        // while the ISPE values are the "physical" values.
        let ispe_width = handle.ispe_width();
        let ispe_height = handle.ispe_height();

        ImageInfo {
            width: handle.width(),
            height: handle.height(),
            ispe_width,
            ispe_height,
            chroma_bits_per_pixel: handle.chroma_bits_per_pixel(),
            luma_bits_per_pixel: handle.luma_bits_per_pixel(),
            has_alpha: handle.has_alpha_channel(),
            has_depth: handle.has_depth_image(),
            is_premultiplied_alpha: handle.is_premultiplied_alpha()
        }
    }

    fn get_thumbnail_image_info(image_id: u32, handle: ImageHandle) -> ThumbnailImageInfo {
        ThumbnailImageInfo {
            image_id,
            info: InfoService::get_image_info(handle)
        }
    }

    fn get_depth_image_info(image_id: u32, handle: ImageHandle) -> DepthImageInfo {
        DepthImageInfo {
            image_id,
            info: InfoService::get_image_info(handle)
        }
    }

    fn get_top_level_image_info(image_id: u32, handle: ImageHandle, ctx: &HeifContext) -> TopLevelImageInfo {
        let mut thumbnail_image_ids = vec![0u32; handle.number_of_thumbnails()];
        let num_thumbnails = handle.thumbnail_ids(&mut thumbnail_image_ids);
        assert_eq!(num_thumbnails, handle.number_of_thumbnails());

        let mut depth_image_ids = vec![0u32; if handle.number_of_depth_images() > 0 { handle.number_of_depth_images() as usize } else { 0 }];
        if depth_image_ids.len() > 0 {
            let depth_image_count = handle.depth_image_ids(&mut depth_image_ids);
            assert_eq!(depth_image_count, handle.number_of_depth_images() as usize);
        }

        TopLevelImageInfo {
            image_id,
            is_primary: handle.is_primary(),
            info: InfoService::get_image_info(handle),
            depths: depth_image_ids.into_iter().enumerate()
                .map(|(i, id)| (id, ctx.image_handle(id).unwrap()))
                .map(|(id, handle)| InfoService::get_depth_image_info(id, handle))
                .collect(),
            thumbnails: thumbnail_image_ids.into_iter().enumerate()
                .map(|(i, id)| (id, ctx.image_handle(id).unwrap()))
                .map(|(id, handle)| InfoService::get_thumbnail_image_info(id, handle))
                .collect(),
        }
    }
}

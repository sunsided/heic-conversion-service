use crate::services::heif_api;

pub struct ImageInfo {
    pub width: u32,
    pub height: u32,
    pub ispe_width: i32,
    pub ispe_height: i32,
    pub chroma_bits_per_pixel: u8,
    pub luma_bits_per_pixel: u8,
    pub has_alpha: bool,
    pub has_depth: bool,
    pub is_premultiplied_alpha: bool,
}

pub struct TopLevelImageInfo {
    pub image_id: u32,
    pub is_primary: bool,
    pub info: ImageInfo,
    pub depths: Vec<DepthImageInfo>,
    pub thumbnails: Vec<ThumbnailImageInfo>,
}

pub struct DepthImageInfo {
    pub image_id: u32,
    pub info: ImageInfo
}

pub struct ThumbnailImageInfo {
    pub image_id: u32,
    pub info: ImageInfo
}

impl std::convert::From<ImageInfo> for heif_api::ImageInfo {
    fn from(info: ImageInfo) -> Self {
        heif_api::ImageInfo {
            width: info.width,
            height: info.height,
            chroma_bits_per_pixel: info.chroma_bits_per_pixel as u32,
            luma_bits_per_pixel: info.luma_bits_per_pixel as u32,
            ispe_width: info.ispe_width,
            ispe_height: info.ispe_height,
            has_alpha: info.has_alpha,
            has_depth: info.has_depth,
            is_premultiplied_alpha: info.is_premultiplied_alpha
        }
    }
}

impl std::convert::From<TopLevelImageInfo> for heif_api::TopLevelImageInfo {
    fn from(info: TopLevelImageInfo) -> Self {
        heif_api::TopLevelImageInfo {
            is_primary: info.is_primary,
            image_id: info.image_id,
            info: Some(info.info.into()),
            thumbnails: info.thumbnails
                .into_iter()
                .map(ThumbnailImageInfo::into)
                .collect(),
            depths: info.depths
                .into_iter()
                .map(DepthImageInfo::into)
                .collect()
        }
    }
}

impl std::convert::From<ThumbnailImageInfo> for heif_api::ThumbnailImageInfo {
    fn from(info: ThumbnailImageInfo) -> Self {
        heif_api::ThumbnailImageInfo {
            image_id: info.image_id,
            info: Some(info.info.into()),
        }
    }
}

impl std::convert::From<DepthImageInfo> for heif_api::DepthImageInfo {
    fn from(info: DepthImageInfo) -> Self {
        heif_api::DepthImageInfo {
            image_id: info.image_id,
            info: Some(info.info.into()),
        }
    }
}
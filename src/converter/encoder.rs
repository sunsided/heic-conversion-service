use crate::converter::decoding_options::DecodingOptions;
use anyhow::Result;
use libheif_rs::{Chroma, ColorSpace, Image, ImageHandle};
use tokio::runtime::Handle;

pub trait Encoder {
    fn colorspace(&self, has_alpha: bool) -> ColorSpace;
    fn chroma(&self, has_alpha: bool, bit_depth: u32) -> Chroma;
    fn update_decoding_options(&self, handle: &ImageHandle, decoding_options: &mut DecodingOptions);
    fn encode_to_bytes(&self, handle: &ImageHandle, image: &Image) -> Result<Vec<u8>>;
    fn encode_to_file(&self, handle: &ImageHandle, image: &Image, filename: String) -> Result<()>;
}

pub trait ExifMetadata {
    fn has_exif_metadata(&self, handle: &ImageHandle) -> bool;
    fn get_exif_metadata(&self, handle: &ImageHandle) -> Result<Option<Vec<u8>>>;
}

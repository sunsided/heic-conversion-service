use crate::converter::decoding_options::DecodingOptions;
use crate::converter::encoder::{Encoder, ExifMetadata};
use anyhow::Result;
use libheif_rs::{Channel, Chroma, ColorSpace, HeifError, Image, ImageHandle, ItemId};
use mozjpeg::Compress;
use std::fs::write;
use thiserror::Error;

const DEFAULT_QUALITY: u32 = 90;
const JPEG_CHROMA: Chroma = Chroma::C420;
const JPEG_COLORSPACE: ColorSpace = ColorSpace::YCbCr(JPEG_CHROMA);

// TODO: Add writing of ICC profile

#[derive(Error, Debug)]
pub enum JpegEncoderError {
    #[error("can only encode 8bpp images, got {0}")]
    BitsPerPixel(u8),
    #[error("invalid image dimensions")]
    InvalidSize(HeifError),
    #[error("unable to obtain compressed data")]
    CompressionFailed,
    #[error("unable to write to disk")]
    FileWrite(std::io::Error),
    #[error("MozJPEG failed")]
    MozJpegPanic,
}

pub struct JpegEncoder {
    quality: u32,
}

impl JpegEncoder {
    pub fn new(quality: i32) -> Self {
        Self {
            quality: if quality < 0 || quality > 100 {
                DEFAULT_QUALITY
            } else {
                quality as u32
            },
        }
    }
}

impl Encoder for JpegEncoder {
    fn colorspace(&self, _has_alpha: bool) -> ColorSpace {
        JPEG_COLORSPACE
    }

    fn chroma(&self, _has_alpha: bool, _bit_depth: u32) -> Chroma {
        JPEG_CHROMA
    }

    fn update_decoding_options(
        &self,
        handle: &ImageHandle,
        decoding_options: &mut DecodingOptions,
    ) {
        if self.has_exif_metadata(handle) {
            decoding_options.set_ignore_transformations(true);
        }
        decoding_options.set_convert_hdr_to_8bit(true);
    }

    fn encode_to_bytes(&self, handle: &ImageHandle, image: &Image) -> Result<Vec<u8>> {
        assert!(image.color_space().is_some());
        assert_eq!(image.color_space().unwrap(), JPEG_COLORSPACE);

        // TODO: Support scaling down from HDR to 8bpp
        let bit_depth = handle.luma_bits_per_pixel();
        if bit_depth != 8 {
            // TODO: support Grayscale images!
            return Err(JpegEncoderError::BitsPerPixel(bit_depth).into());
        }

        let width = match image.width(Channel::Y) {
            Ok(size) => size as usize,
            Err(e) => return Err(JpegEncoderError::InvalidSize(e).into()),
        };
        let height = match image.height(Channel::Y) {
            Ok(size) => size as usize,
            Err(e) => return Err(JpegEncoderError::InvalidSize(e).into()),
        };

        let jpeg_bytes =
            match std::panic::catch_unwind(|| self.compress_mozjpeg(width, height, &image)) {
                Ok(result) => result?,
                Err(_e) => return Err(JpegEncoderError::MozJpegPanic.into()),
            };

        Ok(jpeg_bytes)
    }

    fn encode_to_file(&self, handle: &ImageHandle, image: &Image, filename: String) -> Result<()> {
        // TODO: async possible?
        let bytes = self.encode_to_bytes(handle, image)?;
        return match write(filename.as_str(), &bytes) {
            Ok(_) => Ok(()),
            Err(e) => Err(JpegEncoderError::FileWrite(e).into()),
        };
    }
}

impl ExifMetadata for JpegEncoder {
    fn has_exif_metadata(&self, handle: &ImageHandle) -> bool {
        handle.number_of_metadata_blocks("Exif") > 0
    }

    fn get_exif_metadata(&self, handle: &ImageHandle) -> Result<Option<Vec<u8>>> {
        let mut item_ids: [ItemId; 1] = [ItemId::default()];
        let count = handle.metadata_block_ids("Exif", &mut item_ids);
        for _ in 0..count {
            let size = handle.metadata_size(item_ids[0]);
            if size == 0 {
                continue;
            }

            let result = handle.metadata(item_ids[0])?;
            return Ok(Some(result));
        }

        return Ok(None);
    }
}

impl JpegEncoder {
    fn compress_mozjpeg(&self, width: usize, height: usize, image: &Image) -> Result<Vec<u8>> {
        let planes = image.planes();
        let plane_y = planes.y.unwrap();
        let plane_cb = planes.cb.unwrap();
        let plane_cr = planes.cr.unwrap();

        let (bytes_y, stride_y) = (plane_y.data, plane_y.stride as usize);
        let (bytes_u, stride_u) = (plane_cb.data, plane_cb.stride as usize);
        let (bytes_v, stride_v) = (plane_cr.data, plane_cr.stride as usize);

        // TODO: Exif - write_marker()
        // TODO: Add JPEG comment describing this library or service (jpeg_write_marker() with JPEG_COM)

        let mut comp = Compress::new(mozjpeg::ColorSpace::JCS_YCbCr);
        comp.set_size(width, height);
        comp.set_fastest_defaults();
        comp.set_mem_dest(); // TODO: Write to disk directly? Only if file is large?
        comp.set_optimize_coding(true);
        comp.set_quality(self.quality as _);
        comp.set_scan_optimization_mode(mozjpeg::ScanMode::AllComponentsTogether);
        comp.start_compress();

        // TODO: Check set_raw_data_in() and write_raw_data() since input data is already YCbCr

        // TODO: Speed up this loop
        let mut bytes = Vec::with_capacity(width as usize * 3);
        for y in 0..height {
            bytes.clear();

            let offset_y = (y * stride_y) as usize;
            let offset_u = ((y / 2) * stride_u) as usize;
            let offset_v = ((y / 2) * stride_v) as usize;

            for x in 0..(width as usize) {
                bytes.push(bytes_y[offset_y + x]);
                bytes.push(bytes_u[offset_u + (x / 2)]);
                bytes.push(bytes_v[offset_v + (x / 2)]);
            }
            comp.write_scanlines(&bytes);
        }

        comp.finish_compress();
        let bytes = match comp.data_to_vec() {
            Ok(bytes) => bytes,
            Err(_e) => return Err(JpegEncoderError::CompressionFailed.into()),
        };

        Ok(bytes)
    }
}

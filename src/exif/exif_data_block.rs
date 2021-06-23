use crate::exif::exif_endianness::ExifEndianness;
use anyhow::{Error, Result};
use std::convert::TryInto;

pub const UNTIMED_EXIF_ITEM_TYPE: &str = "Exif";
pub const EXIF_HEADER: [u8; 6] = ['E' as u8, 'x' as u8, 'i' as u8, 'f' as u8, 0u8, 0u8];

/// Untimed Exif metadata
/// ISO/IEC 23008-12:2017(E) Annex A
#[derive(Debug, Default)]
pub struct ExifDataBlock {
    /// An offset in bytes from the first byte of `payload` to the first
    /// byte of the TIFF Header of the Exif metadata, as specified in JEITA CP-3451B.
    /// If the TIFF Header is the first byte of the payload, the value is 0.
    /// Otherwise, it is a positive number skipping any other bytes before the TIFF Header
    /// (e.g. `payload` is formatted as specified for the DCF thumbnail file in JEITA CP-3461B).
    tiff_header_offset: u32,

    /// The first `tiff_header_offset` of the original `payload`.
    /// This should be `Exif\0\0`.
    header: Vec<u8>,

    /// A variable sized array of bytes holding the Exif compliant metadata to be parsed by
    /// the reader. This is compliant with JEITA CP-3451B or JEITA CP-3461B and shall have as part of it a TIFF
    /// Header with referenced Image File Directories (IFDs). There may be additional bytes before or after
    /// this Exif data, but the all data shall be contained in the size indicated by the item size. `payload`
    /// should not contain fields that use file-absolute offsets, because it is allowed to modify a file so that the
    /// location of item data is changed.
    /// When untimed Exif metadata is stored as a metadata item the item_type value shall be 'Exif' (`UNTIMED_EXIF_ITEM_TYPE`).
    pub payload: Vec<u8>,
}

impl ExifDataBlock {
    pub fn new_from_heic(raw: Vec<u8>) -> Result<Self> {
        let magic: [u8; 4] = raw[0..4].try_into()?;
        let offset = u32::from_be_bytes(magic);
        let tiff_header_start = 4 + offset as usize;
        let exif_header = raw[4..tiff_header_start].to_vec();
        assert_eq!(exif_header, EXIF_HEADER);

        Ok(Self {
            tiff_header_offset: offset,
            header: exif_header,
            payload: raw[tiff_header_start..].to_vec(),
        })
    }

    #[allow(dead_code)]
    pub fn is_exif(&self) -> bool {
        self.header == EXIF_HEADER
    }

    /// Returns a JFIF APP1 compatible block, i.e. `exif_header` suffixed by `exif_payload`.
    pub fn to_app1_compatible_block(&self) -> Vec<u8> {
        let mut vec = Vec::with_capacity(self.header.len() + self.payload.len());
        vec.append(&mut self.header.clone());
        vec.append(&mut self.payload.clone());
        vec
    }

    #[allow(dead_code)]
    pub fn get_endianness(&self) -> Result<ExifEndianness> {
        match &self.payload[0..2] {
            b"II" => Ok(ExifEndianness::Intel),
            b"MM" => Ok(ExifEndianness::Motorola),
            _ => Err(Error::msg("Invalid endianness encoding")),
        }
    }
}

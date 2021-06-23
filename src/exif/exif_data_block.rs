use anyhow::Result;
use std::convert::TryInto;

pub const UNTIMED_EXIF_ITEM_TYPE: &str = "Exif";

/// Untimed Exif metadata
/// ISO/IEC 23008-12:2017(E) Annex A
#[derive(Debug, Default)]
pub struct ExifDataBlock {
    /// An offset in bytes from the first byte of `exif_payload` to the first
    /// byte of the TIFF Header of the Exif metadata, as specified in JEITA CP-3451B.
    /// If the TIFF Header is the first byte of the payload, the value is 0.
    /// Otherwise, it is a positive number skipping any other bytes before the TIFF Header
    /// (e.g. `exif_payload` is formatted as specified for the DCF thumbnail file in JEITA CP-3461B).
    exif_tiff_header_offset: u32,

    /// The first `exif_tiff_header_offset` of the original `exif_payload`.
    exif_payload_header: Vec<u8>,

    /// `exif_payload` is a variable sized array of bytes holding the Exif compliant metadata to be parsed by
    /// the reader. This is compliant with JEITA CP-3451B or JEITA CP-3461B and shall have as part of it a TIFF
    /// Header with referenced Image File Directories (IFDs). There may be additional bytes before or after
    /// this Exif data, but the all data shall be contained in the size indicated by the item size. `exif_payload`
    /// should not contain fields that use file-absolute offsets, because it is allowed to modify a file so that the
    /// location of item data is changed.
    /// When untimed Exif metadata is stored as a metadata item the item_type value shall be 'Exif' (`UNTIMED_EXIF_ITEM_TYPE`).
    pub exif_payload: Vec<u8>,
}

impl ExifDataBlock {
    pub fn new_from_heic(raw: Vec<u8>) -> Result<Self> {
        let magic: [u8; 4] = raw[0..4].try_into()?;
        let offset = u32::from_be_bytes(magic);
        let tiff_header_start = 4 + offset as usize;
        Ok(Self {
            exif_tiff_header_offset: offset,
            exif_payload_header: raw[4..tiff_header_start].to_vec(),
            exif_payload: raw[tiff_header_start..].to_vec(),
        })
    }
}

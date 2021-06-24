use crate::converter::ExifMetadata;
use crate::exif::{ExifDataBlock, UNTIMED_EXIF_ITEM_TYPE};
use anyhow::Result;
use libheif_rs::{ImageHandle, ItemId};
use pretty_bytes::converter::convert as pretty_bytes;
use tracing::debug;

#[derive(Debug, Default)]
pub struct HeifExif {}

impl ExifMetadata for HeifExif {
    fn has_exif_metadata(&self, handle: &ImageHandle) -> bool {
        handle.number_of_metadata_blocks(UNTIMED_EXIF_ITEM_TYPE) > 0
    }

    fn get_exif_metadata(&self, handle: &ImageHandle) -> Result<Option<ExifDataBlock>> {
        let mut meta_ids: Vec<ItemId> = vec![ItemId::default(); 1];

        // NOTE: EXIF metadata can be embedded in HEIF tracks (for image sequences), in which case
        //       this approach would probably lose it. In that case, extra work is required here.
        let count = handle.metadata_block_ids(UNTIMED_EXIF_ITEM_TYPE, &mut meta_ids);
        if count == 0 {
            return Ok(None);
        }

        assert_eq!(count, 1);

        let size = handle.metadata_size(meta_ids[0]);
        if size == 0 {
            debug!("Found zero-sized Exif metadata block");
            return Ok(None);
        }

        let result = handle.metadata(meta_ids[0])?;
        assert_eq!(result.len(), size);
        debug!(
            "Got {exif_block_size} of EXIF data",
            exif_block_size = pretty_bytes(result.len() as _).to_string()
        );

        return Ok(Some(ExifDataBlock::new_from_heic(result)?));
    }
}

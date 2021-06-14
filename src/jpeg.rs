use tonic::Status;
use libheif_rs::{HeifContext, Channel};
use mozjpeg::Compress;

pub fn recode_heif_to_jpeg(bytes: &[u8], quality: u8) -> Result<(), Status> {
    // catch_unwind is required for mozjpeg
    let result = std::panic::catch_unwind(|| {
        recode_internal_using_mozjpeg(&bytes, quality)
    });

    match result {
        Ok(x) => x,
        Err(_) => Err(Status::internal("Unable to encode JPEG"))
    }
}

fn recode_internal_using_mozjpeg(bytes: &[u8], quality: u8) -> Result<(), Status> {
    let ctx = match HeifContext::read_from_bytes(&bytes) {
        Ok(ctx) => ctx,
        Err(e) => return Err(Status::internal(e.message)),
    };

    // TODO: Convert all top-level images to JPEG
    let handle = match ctx.primary_image_handle() {
        Ok(handle) => handle,
        Err(e) => return Err(Status::internal(e.message)),
    };

    let bit_depth = handle.luma_bits_per_pixel();
    if bit_depth == 0 {
        return Err(Status::invalid_argument("Input image has undefined bit-depth"))
    }

    let color_space = libheif_rs::ColorSpace::YCbCr(libheif_rs::Chroma::C420);
    let has_exif = handle.number_of_metadata_blocks("Exif") > 0;
    let ignore_transformations = !has_exif;
    let img = match handle.decode(color_space, ignore_transformations) {
        Ok(img) => img,
        Err(e) => return Err(Status::internal(e.message)),
    };

    let width = match img.width(Channel::Y) {
        Ok(img) => img,
        Err(e) => return Err(Status::internal(e.message)),
    };

    let height = match img.height(Channel::Y) {
        Ok(img) => img,
        Err(e) => return Err(Status::internal(e.message)),
    };

    // TODO: What happens in presence of non-premultiplied alpha? The alpha'd values might contain garbage.
    let planes = img.planes();
    let plane_y = planes.y.unwrap();
    let plane_cb = planes.cb.unwrap();
    let plane_cr = planes.cr.unwrap();

    let (bytes_y, stride_y) = (plane_y.data, plane_y.stride as u32);
    let (bytes_u, stride_u) = (plane_cb.data, plane_cb.stride as u32);
    let (bytes_v, stride_v) = (plane_cr.data, plane_cr.stride as u32);

    let mut comp = Compress::new(mozjpeg::ColorSpace::JCS_YCbCr);
    comp.set_size(width as _, height as _);
    comp.set_mem_dest(); // TODO: Write to disk directly? Only if file is large?
    comp.set_optimize_coding(true);
    comp.set_quality(quality as _);
    comp.set_scan_optimization_mode(mozjpeg::ScanMode::AllComponentsTogether);
    comp.start_compress();

    // TODO: Speed up this loop

    /*
    mozjpeg example uses the following code:

        let pixels = vec![0; width * height * 3];
        assert!(comp.write_scanlines(&pixels[..]));

    building up on this we can thread-parallelize the conversion by setting
    individual scanlines on different threads. - same goes for multiple frames.
    */

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
    let jpeg_bytes = match comp.data_to_vec() {
        Ok(img) => img,
        Err(_) => return Err(Status::internal("Couldn't obtain JPEG bytes")),
    };

    // TODO: Replace with something meaningful
    use std::fs::write;
    write(
        "./target/out.jpg",
        &jpeg_bytes,
    )?;

    Ok(())
}
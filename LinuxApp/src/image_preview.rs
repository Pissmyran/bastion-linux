//! Isolate untrusted image loaders from the desktop process.
use std::io::{Read, Write};
use std::sync::atomic::{AtomicBool, Ordering};

const MAX_DIMENSION: i32 = 4096;
const MAX_PIXELS_BYTES: usize = MAX_DIMENSION as usize * MAX_DIMENSION as usize * 4;
const HEADER_BYTES: usize = 16;
static ACTIVE: AtomicBool = AtomicBool::new(false);

pub struct Permit;
impl Drop for Permit {
    fn drop(&mut self) {
        ACTIVE.store(false, Ordering::Release);
    }
}
pub fn try_acquire() -> Option<Permit> {
    ACTIVE
        .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
        .ok()
        .map(|_| Permit)
}

pub struct Preview {
    pub width: i32,
    pub height: i32,
    pub channels: usize,
    pub stride: usize,
    pub pixels: Vec<u8>,
}

// Some decoder allocations have only best-effort limits. The helper process
// adds kernel-enforced address-space and CPU limits; one permit bounds concurrent work.
pub fn run_decoder() -> Result<(), String> {
    for (resource, limit) in [(libc::RLIMIT_AS, 512 * 1024 * 1024), (libc::RLIMIT_CPU, 10)] {
        let limits = libc::rlimit {
            rlim_cur: limit,
            rlim_max: limit,
        };
        if unsafe { libc::setrlimit(resource, &limits) } != 0 {
            return Err("resource limit unavailable".into());
        }
    }
    let mut bytes = Vec::new();
    std::io::stdin()
        .take(crate::sftp::PREVIEW_MAX_BYTES + 1)
        .read_to_end(&mut bytes)
        .map_err(|e| e.to_string())?;
    if bytes.len() as u64 > crate::sftp::PREVIEW_MAX_BYTES {
        return Err("image too large".into());
    }
    let image = decode_pixels(&bytes)?;
    let mut output = std::io::stdout().lock();
    for value in [
        image.width as u32,
        image.height as u32,
        image.channels as u32,
        image.stride as u32,
    ] {
        output
            .write_all(&value.to_le_bytes())
            .map_err(|e| e.to_string())?;
    }
    output.write_all(&image.pixels).map_err(|e| e.to_string())
}

fn decode_pixels(bytes: &[u8]) -> Result<Preview, String> {
    let mut reader = image::ImageReader::new(std::io::Cursor::new(bytes))
        .with_guessed_format()
        .map_err(|e| e.to_string())?;
    let mut limits = image::Limits::default();
    limits.max_image_width = Some(MAX_DIMENSION as u32);
    limits.max_image_height = Some(MAX_DIMENSION as u32);
    limits.max_alloc = Some(128 * 1024 * 1024);
    reader.limits(limits);
    let image = reader.decode().map_err(|e| e.to_string())?.into_rgba8();
    let (width, height) = image.dimensions();
    let stride = width as usize * 4;
    let pixels = image.into_raw();
    validate(width as i32, height as i32, 4, stride, pixels.len())?;
    Ok(Preview {
        width: width as i32,
        height: height as i32,
        channels: 4,
        stride,
        pixels,
    })
}

fn validate(
    width: i32,
    height: i32,
    channels: usize,
    stride: usize,
    len: usize,
) -> Result<(), String> {
    if width <= 0
        || height <= 0
        || width > MAX_DIMENSION
        || height > MAX_DIMENSION
        || ![3, 4].contains(&channels)
    {
        return Err("invalid image dimensions".into());
    }
    let row = width as usize * channels;
    if stride < row
        || stride > MAX_DIMENSION as usize * 4
        || len > MAX_PIXELS_BYTES
        || len < stride * (height as usize - 1) + row
        || len > stride * height as usize
    {
        return Err("invalid image buffer".into());
    }
    Ok(())
}

pub fn decode(bytes: Vec<u8>) -> Result<Preview, String> {
    use std::process::{Command, Stdio};
    let mut child = Command::new(std::env::current_exe().map_err(|e| e.to_string())?)
        .arg("--decode-sftp-preview")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| e.to_string())?;
    let written = child
        .stdin
        .take()
        .ok_or("missing decoder stdin")?
        .write_all(&bytes);
    let mut output = Vec::new();
    let read = child
        .stdout
        .take()
        .ok_or("missing decoder stdout")?
        .take((MAX_PIXELS_BYTES + HEADER_BYTES + 1) as u64)
        .read_to_end(&mut output);
    if read.is_err() || output.len() > MAX_PIXELS_BYTES + HEADER_BYTES {
        let _ = child.kill();
    }
    let status = child.wait().map_err(|e| e.to_string())?;
    written.map_err(|e| e.to_string())?;
    read.map_err(|e| e.to_string())?;
    if !status.success() || output.len() < HEADER_BYTES {
        return Err("image decoder failed".into());
    }
    let value = |offset| u32::from_le_bytes(output[offset..offset + 4].try_into().unwrap());
    let (width, height, channels, stride) = (
        value(0) as i32,
        value(4) as i32,
        value(8) as usize,
        value(12) as usize,
    );
    validate(width, height, channels, stride, output.len() - HEADER_BYTES)?;
    Ok(Preview {
        width,
        height,
        channels,
        stride,
        pixels: output.split_off(HEADER_BYTES),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{DynamicImage, ImageFormat, Rgba, RgbaImage};

    fn png(width: u32, height: u32) -> Vec<u8> {
        let image =
            DynamicImage::ImageRgba8(RgbaImage::from_pixel(width, height, Rgba([255, 0, 0, 255])));
        let mut bytes = std::io::Cursor::new(Vec::new());
        image.write_to(&mut bytes, ImageFormat::Png).unwrap();
        bytes.into_inner()
    }

    #[test]
    fn decodes_small_image_and_rejects_large_dimensions() {
        let preview = decode_pixels(&png(2, 3)).unwrap();
        assert_eq!((preview.width, preview.height, preview.channels), (2, 3, 4));
        assert_eq!(&preview.pixels[..4], &[255, 0, 0, 255]);
        assert!(decode_pixels(&png(MAX_DIMENSION as u32 + 1, 1)).is_err());
        assert!(decode_pixels(b"not an image").is_err());
    }

    #[test]
    fn decodes_each_advertised_format() {
        let image =
            DynamicImage::ImageRgb8(image::RgbImage::from_pixel(2, 2, image::Rgb([255, 0, 0])));
        for format in [
            ImageFormat::Png,
            ImageFormat::Jpeg,
            ImageFormat::Gif,
            ImageFormat::Bmp,
            ImageFormat::WebP,
            ImageFormat::Tiff,
        ] {
            let mut bytes = std::io::Cursor::new(Vec::new());
            image.write_to(&mut bytes, format).unwrap();
            let decoded = decode_pixels(bytes.get_ref()).unwrap();
            assert_eq!((decoded.width, decoded.height), (2, 2), "{format:?}");
        }
    }

    #[test]
    fn rejects_invalid_buffers_and_limits_concurrency() {
        assert!(validate(1, 1, 4, 4, 3).is_err());
        assert!(validate(MAX_DIMENSION + 1, 1, 4, 4, 4).is_err());
        let permit = try_acquire().unwrap();
        assert!(try_acquire().is_none());
        drop(permit);
        assert!(try_acquire().is_some());
    }
}

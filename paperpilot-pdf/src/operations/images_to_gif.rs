use image::imageops::FilterType;
use paperpilot_core::error::PdfError;
use paperpilot_core::traits::{PdfDocument, PdfOperation};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImagesToGifOptions {
    /// Frames per second. If specified, delay is calculated as `(100 / fps).max(1)`.
    pub fps: Option<u8>,
    /// Delay between frames in centiseconds (1/100ths of a second). Default is 25 (0.25s = 4 fps).
    pub delay_centisecs: Option<u16>,
    /// Target frame width in pixels. If provided, images will be scaled/resized consistently.
    pub width: Option<u32>,
    /// GIF loop count (0 for infinite loop). Default is 0.
    pub loop_count: Option<u16>,
}

impl Default for ImagesToGifOptions {
    fn default() -> Self {
        Self {
            fps: Some(4),
            delay_centisecs: Some(25),
            width: None,
            loop_count: Some(0),
        }
    }
}

/// Helper function to calculate frame delay in centiseconds
fn get_delay_centisecs(options: &ImagesToGifOptions) -> u16 {
    if let Some(fps) = options.fps {
        if fps > 0 {
            (100 / fps as u16).max(1)
        } else {
            options.delay_centisecs.unwrap_or(25).max(1)
        }
    } else {
        options.delay_centisecs.unwrap_or(25).max(1)
    }
}

/// Convert multiple image buffers into an animated looping GIF.
pub fn images_to_gif_bytes_multi(
    images: &[&[u8]],
    options: &ImagesToGifOptions,
) -> Result<Vec<u8>, PdfError> {
    if images.is_empty() {
        return Err(PdfError::ParseError("No input images provided".to_string()));
    }

    let mut loaded_images = Vec::with_capacity(images.len());
    for (i, img_bytes) in images.iter().enumerate() {
        if img_bytes.is_empty() {
            return Err(PdfError::ParseError(format!("Image buffer {} is empty", i)));
        }
        let dynamic_img = image::load_from_memory(img_bytes)
            .map_err(|e| PdfError::ParseError(format!("Failed to decode image {}: {:?}", i, e)))?;
        loaded_images.push(dynamic_img.to_rgba8());
    }

    encode_rgba_frames_to_gif(loaded_images, options)
}

/// Encode a list of RgbaImages into a GIF.
fn encode_rgba_frames_to_gif(
    frames: Vec<image::RgbaImage>,
    options: &ImagesToGifOptions,
) -> Result<Vec<u8>, PdfError> {
    if frames.is_empty() {
        return Err(PdfError::ParseError("No frames to encode into GIF".to_string()));
    }

    let delay_centisecs = get_delay_centisecs(options);
    let loop_count = options.loop_count.unwrap_or(0);

    // Determine target dimensions
    let (target_w, target_h) = if let Some(req_w) = options.width {
        let first_w = frames[0].width();
        let first_h = frames[0].height();
        let scale = req_w as f32 / first_w.max(1) as f32;
        let req_h = (first_h as f32 * scale).round() as u32;
        (req_w.max(1), req_h.max(1))
    } else {
        (frames[0].width().max(1), frames[0].height().max(1))
    };

    let gif_w = target_w as u16;
    let gif_h = target_h as u16;

    let mut gif_bytes = Vec::new();
    {
        let mut encoder = gif::Encoder::new(&mut gif_bytes, gif_w, gif_h, &[])
            .map_err(|e| PdfError::Other(format!("Failed to initialize GIF encoder: {:?}", e)))?;

        let repeat_mode = if loop_count == 0 {
            gif::Repeat::Infinite
        } else {
            gif::Repeat::Finite(loop_count)
        };

        encoder
            .set_repeat(repeat_mode)
            .map_err(|e| PdfError::Other(format!("Failed to set GIF repeat mode: {:?}", e)))?;

        for mut frame in frames {
            if frame.width() != target_w || frame.height() != target_h {
                frame = image::imageops::resize(
                    &frame,
                    target_w,
                    target_h,
                    FilterType::Triangle,
                );
            }

            // High-quality NeuQuant palette quantization (speed = 10 for balanced quality and performance)
            let mut gif_frame = gif::Frame::from_rgba_speed(
                gif_w,
                gif_h,
                frame.as_mut(),
                10,
            );
            gif_frame.delay = delay_centisecs;

            encoder
                .write_frame(&gif_frame)
                .map_err(|e| PdfError::Other(format!("Failed to write GIF frame: {:?}", e)))?;
        }
    }

    Ok(gif_bytes)
}

/// Convert multiple image file paths into an animated looping GIF.
pub fn images_to_gif_from_paths<P: AsRef<Path>>(
    paths: &[P],
    options: &ImagesToGifOptions,
) -> Result<Vec<u8>, PdfError> {
    if paths.is_empty() {
        return Err(PdfError::ParseError("No input image paths provided".to_string()));
    }

    let mut buffers = Vec::with_capacity(paths.len());
    for p in paths {
        let p_ref = p.as_ref();
        let bytes = std::fs::read(p_ref)
            .map_err(|e| PdfError::IoError(std::io::Error::new(e.kind(), format!("Failed reading {:?}: {}", p_ref, e))))?;
        if bytes.is_empty() {
            return Err(PdfError::ParseError(format!("File {:?} is empty", p_ref)));
        }
        buffers.push(bytes);
    }

    let slice_refs: Vec<&[u8]> = buffers.iter().map(|b| b.as_slice()).collect();
    images_to_gif_bytes_multi(&slice_refs, options)
}

/// Backward/single buffer compatibility handler.
/// If a single image buffer is passed, it creates a 1-frame (or decoded animated image frames) GIF.
pub fn images_to_gif_bytes(
    input_bytes: &[u8],
    options: &ImagesToGifOptions,
) -> Result<Vec<u8>, PdfError> {
    if input_bytes.is_empty() {
        return Err(PdfError::ParseError("Input image bytes cannot be empty".to_string()));
    }

    images_to_gif_bytes_multi(&[input_bytes], options)
}

#[derive(Default)]
pub struct ImagesToGifOperation {
    pub image_paths: Vec<PathBuf>,
    pub options: ImagesToGifOptions,
}

impl ImagesToGifOperation {
    pub fn new(image_paths: Vec<PathBuf>, options: ImagesToGifOptions) -> Self {
        Self { image_paths, options }
    }
}

impl PdfOperation for ImagesToGifOperation {
    fn execute(&self, _document: &mut dyn PdfDocument) -> paperpilot_core::error::OperationResult<()> {
        Err(PdfError::UnsupportedOperation("ImagesToGifOperation operates standalone via images_to_gif_from_paths".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{ImageBuffer, Rgba};
    use std::io::Cursor;

    fn create_test_png(w: u32, h: u32, color: [u8; 4]) -> Vec<u8> {
        let img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_pixel(w, h, Rgba(color));
        let mut bytes = Vec::new();
        img.write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Png).unwrap();
        bytes
    }

    #[test]
    fn test_images_to_gif_empty_document() {
        let options = ImagesToGifOptions::default();
        let bytes = vec![];
        let result = images_to_gif_bytes(&bytes, &options);
        assert!(result.is_err());
    }

    #[test]
    fn test_images_to_gif_multi_frames() {
        let options = ImagesToGifOptions {
            fps: Some(10),
            delay_centisecs: None,
            width: Some(100),
            loop_count: Some(0),
        };

        let frame1 = create_test_png(50, 50, [255, 0, 0, 255]);
        let frame2 = create_test_png(100, 100, [0, 255, 0, 255]);
        let frame3 = create_test_png(80, 80, [0, 0, 255, 255]);

        let frames = [&frame1[..], &frame2[..], &frame3[..]];
        let gif_result = images_to_gif_bytes_multi(&frames, &options).expect("Failed to encode GIF");

        assert!(!gif_result.is_empty());
        assert_eq!(&gif_result[0..3], b"GIF");
    }

    #[test]
    fn test_images_to_gif_from_paths() {
        let dir = tempfile::tempdir().unwrap();
        let p1 = dir.path().join("f1.png");
        let p2 = dir.path().join("f2.png");

        let frame1 = create_test_png(60, 60, [255, 255, 0, 255]);
        let frame2 = create_test_png(60, 60, [0, 255, 255, 255]);

        std::fs::write(&p1, frame1).unwrap();
        std::fs::write(&p2, frame2).unwrap();

        let options = ImagesToGifOptions::default();
        let gif_result = images_to_gif_from_paths(&[p1, p2], &options).expect("Should create GIF");

        assert!(!gif_result.is_empty());
        assert_eq!(&gif_result[0..3], b"GIF");
    }
}

mod utils;

use std::io::Cursor;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;
use js_sys::{Uint8Array};
use image::{ImageReader};
use image::imageops::FilterType;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Debug, Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum ImageRsImageResizeAlgorithm {
    #[serde(rename = "nearest")]
    Nearest,
    #[serde(rename = "triangle")]
    Triangle,
    #[serde(rename = "catmull-rom")]
    CatmullRom,
    #[serde(rename = "gaussian")]
    Gaussian,
    #[serde(rename = "lanczos3")]
    Lanczos3,
}

impl From<ImageRsImageResizeAlgorithm> for FilterType {
    fn from(algorithm: ImageRsImageResizeAlgorithm) -> Self {
        match algorithm {
            ImageRsImageResizeAlgorithm::Nearest => FilterType::Nearest,
            ImageRsImageResizeAlgorithm::Triangle => FilterType::Triangle,
            ImageRsImageResizeAlgorithm::CatmullRom => FilterType::CatmullRom,
            ImageRsImageResizeAlgorithm::Gaussian => FilterType::Gaussian,
            ImageRsImageResizeAlgorithm::Lanczos3 => FilterType::Lanczos3,
        }
    }
}

/// Configuration for browser image resizing
#[derive(Debug, Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct BrowserImageResizerConfig {
    pub algorithm: ImageRsImageResizeAlgorithm,

    pub max_width: u32,
    pub max_height: u32,

    #[tsify(optional)]
    pub scale_ratio: Option<f32>,

    #[tsify(optional)]
    pub debug: Option<bool>,

    #[tsify(optional)]
    pub quality: Option<f32>,

    #[tsify(optional)]
    pub mime_type: String,
}

#[wasm_bindgen]
pub async fn read_and_compress_image(i: &Uint8Array, config: BrowserImageResizerConfig) -> Vec<u8> {
    let buffered_reader = Cursor::new(i.to_vec());
    let image_reader: ImageReader<Cursor<Vec<u8>>> = ImageReader::new(buffered_reader).with_guessed_format().unwrap();
    let image = image_reader.decode().unwrap();
    log(&format!("Image dimensions: {}x{}", image.width(), image.height()));
    log(&format!("image color: {:?}", image.color()));
    log(&format!("{:?}", config));
    let image2 = image.resize(config.max_width, config.max_height, config.algorithm.into());

    // decide the output format based on the mime_type
    let format = match config.mime_type.as_str() {
        "image/png" => image::ImageFormat::Png,
        "image/jpeg" => image::ImageFormat::Jpeg,
        "image/webp" => image::ImageFormat::WebP,
        "image/gif" => image::ImageFormat::Gif,
        "image/avif" => image::ImageFormat::Avif,
        _ => {
            log("Unsupported image format, defaulting to PNG");
            image::ImageFormat::Png
        }
    };

    let mut writer = Vec::new();
    image2.write_to(&mut Cursor::new(&mut writer), format)
        .expect("Failed to write image");
    log("Image compression completed");
    writer
}

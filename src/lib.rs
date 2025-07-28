mod utils;

use std::io::Cursor;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;
use web_sys::{File, Blob, BlobPropertyBag};
use js_sys::{ArrayBuffer, Uint8Array};
use wasm_bindgen_futures::JsFuture;
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

async fn file_to_vec(i: &File) -> Result<Vec<u8>, JsValue> {
    JsFuture::from(i.array_buffer()).await
        .map(|js_val| js_val.dyn_into::<ArrayBuffer>().unwrap())
        .map(|array_buffer| Uint8Array::new(&array_buffer).to_vec())
}

#[wasm_bindgen]  
pub fn vec_to_blob(data: Vec<u8>, mime_type: &str) -> Result<Blob, JsValue> {
    let uint8_array = Uint8Array::from(&data[..]);  

    let blob_options = BlobPropertyBag::new();  
    blob_options.set_type(mime_type); // MIMEタイプを設定

    Blob::new_with_u8_array_sequence_and_options(&uint8_array, &blob_options)
}

#[wasm_bindgen]
pub async fn read_and_compress_image(i: &File, config: BrowserImageResizerConfig) -> Vec<u8> {
    // Placeholder for image processing logic
    // This function should read the image file, compress it, and return a new File object
    // For now, we will just return the input file as is
    let u8vec = file_to_vec(&i).await.unwrap();
    let buffered_reader = Cursor::new(u8vec);
    let image_reader: ImageReader<Cursor<Vec<u8>>> = ImageReader::new(buffered_reader).with_guessed_format().unwrap();
    let image = image_reader.decode().unwrap();
    log(&format!("Image dimensions: {}x{}", image.width(), image.height()));
    log(&format!("image color: {:?}", image.color()));
    log(&format!("{:?}", config));
    let image2 = image.resize(config.max_width, config.max_height, config.algorithm.into());
    let mut writer = Vec::new();
    image2.write_to(&mut Cursor::new(&mut writer), image::ImageFormat::Png)
        .expect("Failed to write image");
    log("Image compression completed");
    writer
}

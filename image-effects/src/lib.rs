use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console::log_1 as log;
use base64::{ encode , decode};
use image::{load_from_memory, buffer};
use std::io::{Cursor};
use image::ImageOutputFormat::Png;

#[wasm_bindgen]
pub fn grayscale(encoded_file: &str) -> String{
    log(&"Grayscale called".into());

    let base64_to_vector = decode(encoded_file).unwrap();
    log(&"Image decoded".into());

    let mut img = load_from_memory(&base64_to_vector).unwrap();
    log(&"Image loaded".into());

    img = img.grayscale();
    log(&"Grayscale effect applied".into());

    let buffer = vec![];
    let mut cursor = Cursor::new(buffer);

    img.write_to(&mut cursor, Png).unwrap();
    log(&"New Image written".into());
    let cursor_slice: &[u8] = &cursor.get_ref();
    let encoded_image = encode(cursor_slice);
    let data_url = format!(
        "data:image/png;base64,{}",
        encoded_image
    );

    data_url
}

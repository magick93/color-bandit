use anyhow::anyhow;
use serde::Serialize;
use spin_sdk::http::{IntoResponse, Request};
use spin_sdk::http_component;
use std::collections::HashSet;

use palette_extract::get_palette_rgb;

#[derive(Serialize, Eq, PartialEq, Hash)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

#[http_component]
fn handle_color_bandit(req: Request) -> anyhow::Result<impl IntoResponse> {
    // Assuming the image is sent in the request body.
    let image_data = req.body(); // Handle retrieving the image data

    let img = match image::load_from_memory(image_data) {
        Ok(image) => image,
        Err(e) => return Err(anyhow!("Failed to load image: {}", e)),
    };

    // Grab a reference to the underlying pixels/RGB buffer
    let pixels = img.as_bytes();

    // Extract the color palette using `palette_extract`
    let extracted_palette = get_palette_rgb(pixels);

    let converted_palette: Vec<Color> = extracted_palette
    .iter()
    .map(|color| Color {
        red: color.r,
        green: color.g,
        blue: color.b,
    })
    .collect();

    let unique_colors: HashSet<_> = converted_palette.into_iter().collect();
    let palette: Vec<Color> = unique_colors.into_iter().collect();


    // let result = serde_json::to_string(&serializable_palette).unwrap_or_else(|_| "[]".to_string());
    let result = serde_json::to_string(&palette)
    .unwrap_or_else(|_| "[]".to_string());

    Ok(http::Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(result)?)
}

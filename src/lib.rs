use anyhow::anyhow;
use color_thief::ColorFormat;
use image::DynamicImage;

use spin_sdk::http::{IntoResponse, Request};
use spin_sdk::http_component;
use std::io::Cursor;

use serde::Serialize;

#[derive(Serialize)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

/// A simple Spin HTTP component.
// #[http_component]
// fn handle_color_bandit(req: Request) -> anyhow::Result<impl IntoResponse> {
//     println!("Handling request to {:?}", req.header("spin-full-url"));
//     Ok(http::Response::builder()
//         .status(200)
//         .header("content-type", "text/plain")
//         .body("Hello, Color Bandit")?)
// }

/// A Spin HTTP component to get a color palette from an image.
#[http_component]
fn handle_color_bandit(req: Request) -> anyhow::Result<impl IntoResponse> {
    // Assuming the image is sent in the request body.
    let image_data = req.body(); // You'll need to handle retrieving the image data appropriately

    let img = match image::load_from_memory(image_data) {
        Ok(image) => image,
        Err(e) => {
            // Use anyhow! to create an anyhow::Error
            return Err(anyhow!("Failed to load image: {}", e));
        }
    };

    let img_bytes = match image_to_bytes(&img) {
        Ok(bytes) => bytes,
        Err(e) => {
            // Use anyhow! to create an anyhow::Error
            return Err(anyhow!("Failed to convert image to bytes: {}", e));
        }
    };

    // let palette = color_thief::get_palette(&img_bytes, ColorFormat::Rgb, 10, 10).map_err(|e| {
    //     http::Response::builder()
    //         .status(500)
    //         .body(format!("Failed to get palette: {}", e))
    // })?;

    // Convert the incoming data to an image object
    // let img = match image::load_from_memory(image_data) {
    //     Ok(image) => image,
    //     Err(e) => {
    //         // Directly return an error using anyhow!
    //         return Err(anyhow!("Failed to load image: {}", e));
    //     }
    // };

    // Use color-thief-rs to get the palette
    let palette = match color_thief::get_palette(&img_bytes, ColorFormat::Rgb, 10, 2) {
        Ok(colors) => colors,
        Err(e) => {
            // Handle the error explicitly without using `?`
            return Err(anyhow!("Failed to get palette: {}", e));
        }
    };

    // Convert the palette to a suitable response format
    // let result = palette
    //     .iter()
    //     .map(|color| format!("{:?}", color))
    //     .collect::<Vec<String>>()
    //     .join(", ");

    // let serializable_palette: Vec<(u8, u8, u8)> = palette
    //     .iter()
    //     .map(|color| (color.r, color.g, color.b))
    //     .collect();

    let serializable_palette: Vec<Color> = palette
        .iter()
        .map(|color| Color {
            red: color.r,
            green: color.g,
            blue: color.b,
        })
        .collect();

    // let result = serde_json::to_string(&serializable_palette).unwrap_or_else(|_| "[]".to_string());

    let result = serde_json::to_string(&serializable_palette)
    .unwrap_or_else(|_| "[]".to_string());

    // Return the result

    Ok(http::Response::builder()
        .status(200)
        .header("content-type", "application/json") 
        .body(result)?)
}

fn image_to_bytes(img: &DynamicImage) -> anyhow::Result<Vec<u8>> {
    let mut bytes = Cursor::new(Vec::new());
    img.write_to(&mut bytes, image::ImageOutputFormat::Png)?;
    Ok(bytes.into_inner())
}

// impl Serialize for rgb::RGB<u8> {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         let mut state = serializer.serialize_struct("RGB", 3)?;
//         state.serialize_field("r", &self.r)?;
//         state.serialize_field("g", &self.g)?;
//         state.serialize_field("b", &self.b)?;
//         state.end()
//     }
// }

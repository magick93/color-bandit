use anyhow::anyhow;
use serde::Serialize;
use spin_sdk::http::{IntoResponse, Request};
use spin_sdk::http_component;


#[derive(Serialize, Eq, PartialEq, Hash)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

fn vec_to_color_array(colors: Vec<u8>) -> Vec<Color> {
    colors.chunks(3) // Split the vector into chunks of 3 elements
        .map(|chunk| Color {
            red: *chunk.get(0).unwrap_or(&0),     // Get the first element or default to 0
            green: *chunk.get(1).unwrap_or(&0),   // Get the second element or default to 0
            blue: *chunk.get(2).unwrap_or(&0),    // Get the third element or default to 0
        })
        .collect() // Collect into a Vec<Color>
}

#[http_component]
fn handle_color_bandit(req: Request) -> anyhow::Result<impl IntoResponse> {
    // Assuming the image is sent in the request body.
    let image_data = req.body(); // Handle retrieving the image data

    let img = match image::load_from_memory(image_data) {
        Ok(image) => image,
        Err(e) => return Err(anyhow!("Failed to load image: {}", e)),
    };

    let colors = dominant_color::get_colors(img.to_rgb8().into_raw().as_slice(), false);

    let color_array = vec_to_color_array(colors);

    // Grab a reference to the underlying pixels/RGB buffer
    // let pixels = img.as_bytes();

    // Extract the color palette using `palette_extract`
    // let extracted_palette = get_palette_rgb(pixels);
    

    // let converted_palette: Vec<Color> = colors
    // .iter()
    // .map(|color| Color {
    //     red: color,
    //     green: color.g,
    //     blue: color.b,
    // })
    // .collect();

    // let unique_colors: HashSet<_> = converted_palette.into_iter().collect();
    // let palette: Vec<Color> = unique_colors.into_iter().collect();


    // println!("colors: {:?}", colors);
    // let result = serde_json::to_string(&serializable_palette).unwrap_or_else(|_| "[]".to_string());
    let result = serde_json::to_string(&color_array)
    .unwrap_or_else(|_| "[]".to_string());

    Ok(http::Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(result)?)
}

use image::{ImageBuffer, GrayImage};


#[tauri::command]
pub fn create_generic_gray_image(nds_file: File,output_path: String,width: u32, height: u32) {
    let width = 100;
    let height = 100;

    // Example pixel data for a grayscale image
    let pixel_data: Vec<u8> = vec![128; width * height]; // A gray image for example

    // Create a grayscale image from raw pixel data
    let img: GrayImage = ImageBuffer::from_vec(width, height, nds_file.file_data).expect("Failed to create image buffer");

    // Save the image as a PNG file
    img.save("output.png").expect("Failed to save image");
}
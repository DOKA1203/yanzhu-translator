use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};
use image::GenericImageView;

pub fn get_ts() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time error")
        .as_millis()
}

pub fn resize_image(input_path: &str, output_path: &str, max_dimension: u32) -> Result<(), image::ImageError> {
    // Load the image from the file
    let img = image::open(input_path)?;

    // Get the current dimensions
    let (width, height) = img.dimensions();

    // Calculate the new dimensions while maintaining the aspect ratio
    let (new_width, new_height) = if width > height {
        (max_dimension, max_dimension * height / width)
    } else {
        (max_dimension * width / height, max_dimension)
    };

    // Resize the image
    let resized_img = img.thumbnail(new_width, new_height);

    // Save the resized image to a new file
    resized_img.save(output_path)?;
    Ok(())
}
pub fn find_jpeg_files(folder_path: &str) -> Result<Vec<String>, std::io::Error> {
    let mut jpeg_files = Vec::new();
    let entries = fs::read_dir(folder_path)?;
    for entry in entries.flatten() {
        if let Ok(file_type) = entry.file_type() {
            if file_type.is_file() {
                if let Some(file_name) = entry.file_name().to_str() {
                    if file_name.to_lowercase().ends_with(".jpg") {
                        jpeg_files.push(entry.path().to_string_lossy().to_string());
                    }
                }
            }
        }
    }
    Ok(jpeg_files)
}
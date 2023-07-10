use std::io::Cursor;
use rusty_tesseract_custom::{self, Args, Image,image};
use image::io::Reader as ImageReader;

pub fn rustyTextFromImage(image:Vec<u8>)->String{

// Lo convertimos a un DynamicImage usando la crate image
    let dynamic_image = ImageReader::new(Cursor::new(image))
    .with_guessed_format()
    .expect("Failed to guess format")
    .decode()
    .expect("Failed to decode image");

// Lo convertimos a un Image usando la crate rusty_tesseract
    let image = Image::from_dynamic_image(&dynamic_image).unwrap();
    let default_Args = Args::default();
    // Ahora podemos usar el objeto Image con la librería rusty_tesseract
    let output = rusty_tesseract_custom::image_to_string(&image, &default_Args);
    output.unwrap()
}

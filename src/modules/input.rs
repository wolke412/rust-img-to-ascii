use image::{io::Reader as ImageReader, DynamicImage};


pub fn get() -> DynamicImage {

    let img = 
    ImageReader::open("static/cat.jpg")
        .unwrap()
        .decode()
        .unwrap();

    img
}

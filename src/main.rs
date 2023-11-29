use image::GenericImageView;
use img_to_ascii::modules::{input, img::grayscale};

fn main() {

    let img = input::get();

    let gray_ramp: Vec<char> = " .:-=+*#%@"
        .chars()
        .collect();

    let mut count = 0;  

    let resized = img.resize_exact(120, 60, image::imageops::FilterType::CatmullRom);
    let width = resized.width();

    resized.pixels()
        .into_iter()
        .for_each(|f| {
            count += 1;
            
            let pixel_data = f.2;
            let rgba = pixel_data.0;
            
            let grayscaled = grayscale( rgba );

            let idx = grayscaled  / 256.0 * ( gray_ramp.len() - 1 ) as f32 ;
            let char = gray_ramp[idx as usize];
            
            print!( "{}", char );

            if count % width  == 0 {
                print!("\n");
            }
        });

}

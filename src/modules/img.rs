pub fn grayscale( rgba: [u8;4] ) -> f32 {
    let [ r, g, b, a ] = rgba;

    
        (r as f32 * 0.3 ) 
    + (g as f32 * 0.5 ) 
    + (b as f32 * 0.11 ) 
    
}

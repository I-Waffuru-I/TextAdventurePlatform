
use image::{imageops, Rgba};

/// Converts the image at the given file path into an ASCII string of the desired width x height, with colors.
/// 
/// Returns the ASCII string if everything goes successfully, else returns an error message. 
pub fn get_ascii_art(file_path : &str, result_w : u32, result_h : u32) -> Result<String, &'static str>{
    let st_len = file_path.len();
    if st_len < 4 {
        return Err("File name is too short!")
    }

    let img = if let Ok(i) = image::open(file_path) 
    { i } else { 
        return Err("Couldn't open image!")
    };

    let mut final_img_str = String::new();
    let resized = imageops::resize(&img, result_w, result_h, imageops::Triangle);

    for (i, p) in resized.pixels().enumerate() {
        let lum = get_lum(p);
        let ch = get_char_from_lum(&lum);
        let s = format!("\u{001b}[38;2;{};{};{}m{}",
            p.0[0],
            p.0[1],
            p.0[2],
            ch
            );
        
        if i % result_w as usize == 0  && i != 0{
            final_img_str.push('\n');
        };
        final_img_str.push_str(&s);
    };
    final_img_str.push_str("\u{001b}[0m");
    Ok(final_img_str)
}

/// Converts the image at the given file path into an ASCII string of the desired width x height, without colors.
/// 
/// Returns the ASCII string if everything goes successfully, else returns an error message. 
pub fn get_ascii_art_bland(file_path : &str, result_w : u32, result_h : u32) -> Result<String, &'static str>{

    let st_len = file_path.len();
    if st_len < 4 {
        return Err("File name is too short!")
    }

    let img = if let Ok(i) = image::open(file_path) 
    { i } else { 
        return Err("Couldn't open image!")
    };

    let mut final_img_str = String::new();
    let resized = imageops::resize(&img, result_w, result_h, imageops::Triangle);
    // chunked pixels now contains all pixels to display
    // for (i,p) in chunked_pixels.iter().enumerate() {

    // this solves basically everything I was trying to do above
    for (i, p) in resized.pixels().enumerate() {
//  ESC[38;2;{r};{g};{b}m	
        let lum = get_lum(p);
        let ch = get_char_from_lum(&lum);
        
        if i % result_w as usize == 0 && i != 0 {
            final_img_str.push('\n');
        }
        final_img_str.push(ch);
    };
    Ok(final_img_str)
}

fn get_lum(pix : &Rgba<u8>) -> u8 {
    if pix.0[3] == 0 {return 0}
    
    let i = 0.2126*pix.0[0] as f64 + 0.7152*pix.0[1]as f64 + 0.0722*pix.0[2] as f64;
    i as u8
}

fn get_char_from_lum(pix :&u8) -> char {
    match pix / 32 {
        0 => '.',
        1 => ':',
        2 => 'º',
        3 => 'o',
        4 => '¤',
        5 => '0',
        6 => 'Ø',
        7 => '&',
        8 => '#',
        _ => {
            panic!("ERROR pix with value {pix} couldn't be parsed")
        }
    }
}
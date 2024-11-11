use ansi_term::Color;
/*
pub fn match_color_by_string(input : &str) -> Option<Color> {
    match input {
        "Green" => Some(Color::Green),
        "Red" => Some(Color::Red),
        "Blue" => Some(Color::Blue),
        "Cyan" => Some(Color::Cyan),
        "Purple" => Some(Color::Purple),
        _ => None,
    }
}
*/

pub fn get_color_by_rgb(r:u8, g:u8,b:u8) -> Color {
    Color::RGB(r,g,b)
}
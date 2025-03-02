use simple_icons::color;
use simple_icons_sdk as sdk;

/// Check if a string is a valid hex color
pub fn is_valid_hex_color(value: &str) -> bool {
    if value.len() != 6 && value.len() != 3 {
        return false;
    }
    for c in value.chars() {
        if !c.is_ascii_hexdigit() {
            return false;
        }
    }
    true
}

/// Get the contrast color for a given hex color
pub fn contrast_color_for(hex: &str) -> String {
    if !is_valid_hex_color(hex) {
        return "black".to_string();
    }
    let is_light_hex =
        color::is_relatively_light_icon_hex(&sdk::normalize_color(hex));
    if is_light_hex { "black" } else { "white" }.to_string()
}

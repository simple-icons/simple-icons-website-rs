//! Hex colors sorting

fn hex_to_tuple(hex: &str) -> (u8, u8, u8) {
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap();
    (r, g, b)
}

pub mod sorting {
    use super::*;
    use colorsys::{Hsl, Rgb};
    use std::cmp::Ordering;

    const GREY_RANGE: u8 = 10;
    const BLACK_CUTOFF: u8 = 15;
    const WHITE_CUTOFF: u8 = 90;

    fn hsl_to_upper_hex_string(hsl: &Hsl) -> String {
        Rgb::from(hsl)
            .to_hex_string()
            .trim_start_matches('#')
            .to_uppercase()
    }

    fn is_gray(hsl: &Hsl) -> bool {
        let l = hsl.lightness() as u8;
        if l <= BLACK_CUTOFF || l >= WHITE_CUTOFF {
            return true;
        }

        let [r, g, b]: [u8; 3] = Rgb::from(hsl).into();

        if g < GREY_RANGE || b < GREY_RANGE {
            true
        } else {
            let g_plus_range = if { u8::MAX - g } < GREY_RANGE {
                u8::MAX
            } else {
                g + GREY_RANGE
            };
            let b_plus_range = if { u8::MAX - b } < GREY_RANGE {
                u8::MAX
            } else {
                b + GREY_RANGE
            };
            r <= g_plus_range && b <= g_plus_range && r <= b_plus_range
        }
    }

    pub fn sort_hexes(hexes: &Vec<String>) -> Vec<String> {
        let mut coloreds: Vec<Hsl> = vec![];
        let mut greys: Vec<Hsl> = vec![];

        for hsl in hexes
            .iter()
            .map(|hex| Hsl::from(Rgb::from(hex_to_tuple(hex))))
        {
            if is_gray(&hsl) {
                greys.push(hsl);
            } else {
                coloreds.push(hsl);
            }
        }

        let mut ret: Vec<String> = Vec::with_capacity(hexes.len());
        coloreds.sort_by(|a, b| {
            // sort by hue
            let hue = a.hue() - b.hue();
            if hue != 0 as f64 {
                return if hue > 0 as f64 {
                    Ordering::Greater
                } else {
                    Ordering::Less
                };
            }

            // if hue is the same, sort by saturation
            let sat = a.saturation() - b.saturation();
            if sat > (0 as f64) {
                Ordering::Greater
            } else if sat < (0 as f64) {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        });
        ret.extend(coloreds.iter().map(hsl_to_upper_hex_string));

        // sort greys by lightness
        greys.sort_by(|a, b| {
            let lightness = a.lightness() - b.lightness();
            if lightness > (0 as f64) {
                Ordering::Greater
            } else if lightness < (0 as f64) {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        });
        ret.extend(greys.iter().map(hsl_to_upper_hex_string));

        ret
    }
}

pub mod relative_luminance {
    use super::*;

    fn rgb_norm(rgb: u8) -> f32 {
        let norm = (rgb as f32) / 255.0;
        if norm <= 0.03928 {
            norm / 12.92
        } else {
            ((norm + 0.055) / 1.055).powf(2.4)
        }
    }

    /// Get the relative luminance of a color
    /// based on the next definition of the W3C:
    /// https://www.w3.org/TR/2008/REC-WCAG20-20081211/#relativeluminancedef
    pub fn get(hex: &str) -> f32 {
        let (r, g, b) = hex_to_tuple(hex);
        0.2126 * rgb_norm(r) + 0.7152 * rgb_norm(g) + 0.0722 * rgb_norm(b)
    }
}

pub fn is_relatively_light_icon_hex(hex: &str) -> bool {
    relative_luminance::get(hex) >= 0.4
}

pub use relative_luminance::get as get_relative_luminance;
pub use sorting::sort_hexes;

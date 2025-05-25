mod world;

pub use world::{AppWorld, TouchesViewport};

/// Make the first character of a string uppercase and the rest lowercase.
pub fn capitalize(s: &str) -> String {
    s.chars()
        .take(1)
        .flat_map(|f| f.to_uppercase())
        .chain(s.chars().skip(1))
        .collect()
}

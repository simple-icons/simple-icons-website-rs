/// A rewriting in Rust of some rules linting SVGs in the Simple Icons repository.

static PATH_VALID_CHARACTERS: &str = "mMzZlLhHvVcCsSqQtTaAeE0123456789,.- ";
static NUMBERS: &str = "0123456789";

type Path = String;
type Range = (u32, u32);
pub type LintErrorFix = (Path, Range);
pub type LintErrorFixer = &'static dyn Fn(&str, Range) -> LintErrorFix;
pub type LintError = (Path, Option<Range>, Option<LintErrorFixer>);

pub type ViewBox = (f64, f64, f64, f64);

fn remove_characters_in_range_fixer(path: &str, range: Range) -> LintErrorFix {
    let mut new_path = String::with_capacity(path.len());
    for (i, character) in path.chars().enumerate() {
        if (i as u32) >= range.0 && (i as u32) < range.1 {
            continue;
        }
        new_path.push(character)
    }
    (new_path, range)
}

fn fix_path_not_starts_with_moveto_command(
    path: &str,
    _range: Range,
) -> LintErrorFix {
    let first_char = path.chars().take(1).collect::<String>();
    if first_char.chars().next().unwrap().is_alphabetic() {
        let mut new_path = path.to_string();
        new_path.replace_range(0..1, "M");
        (new_path, (0, 1))
    } else {
        (format!("M{}", path), (0, 1))
    }
}

/// Check some path format validations.
///
/// The path must:
///
/// - Start with "moveto" command ("M" or "m").
/// - Match the regex `/^m[-mzlhvcsqtae0-9,. ]+$/i` (only contain
///   certain characters).
///
/// The implementation must not contain a regex library because
/// they add a lot of size to wasm compiled code.
///
/// The return value is a vector of error messages with ranges.
pub fn path_format(path: &str) -> Vec<LintError> {
    let mut errors: Vec<LintError> = vec![];

    if !path.is_empty() && !path.starts_with('M') && !path.starts_with('m') {
        let first_char = path.chars().take(1).collect::<String>();
        errors.push((
            format!(
                concat!(
                    "Must start with \"moveto\" command (\"M\" or \"m\")",
                    " but starts with \"{}\"",
                ),
                first_char,
            )
            .to_string(),
            Some((0, 1)),
            Some(&fix_path_not_starts_with_moveto_command),
        ));
    }

    for (i, character) in path.chars().enumerate() {
        if !PATH_VALID_CHARACTERS.contains(character) {
            errors.push((
                format!(
                    "Contains invalid character \"{}\" at index {}",
                    character, i,
                ),
                Some((i as u32, i as u32 + 1)),
                Some(&remove_characters_in_range_fixer),
            ));
        }
    }

    errors
}

fn fix_negative_zero(path: &str, range: Range) -> LintErrorFix {
    let mut new_path = String::with_capacity(path.len());
    // iterate over characters in range:
    for (i, character) in path.chars().enumerate() {
        if i == range.0 as usize {
            let previous_char = path.chars().nth(i - 1).unwrap_or('\0');
            let replacement = match NUMBERS.contains(previous_char) {
                true => " 0",
                false => "0",
            };
            new_path.push_str(replacement);
            continue;
        } else if (i as u32) > range.0 && (i as u32) < range.1 {
            continue;
        }
        new_path.push(character);
    }
    (new_path, range)
}

pub fn negative_zeros(path: &str) -> Vec<LintError> {
    let mut errors: Vec<LintError> = vec![];
    for (i, character) in path.chars().enumerate() {
        if character != '-' {
            continue;
        }
        let next_char = path.chars().nth(i + 1).unwrap_or('\0');
        if "0\0".contains(next_char) {
            errors.push((
                format!("Found \"-0\" at index {}", i),
                Some((i as u32, i as u32 + 2)),
                Some(&fix_negative_zero),
            ));
        }
    }
    errors
}

pub fn icon_size(_path: &str, bbox: ViewBox) -> Vec<LintError> {
    let width = bbox.2 - bbox.0;
    let height = bbox.3 - bbox.1;
    let mut errors: Vec<LintError> = vec![];

    if width == 0.0 && height == 0.0 {
        errors.push((
            "Size was reported as 0 x 0; check if the path is valid"
                .to_string(),
            None,
            None,
        ));
    } else if width != 24.0 && height != 24.0 {
        errors.push((
            format!(
                concat!(
                    "Size must be exactly 24 pixels in one dimension,",
                    " currently {:.4} x {:.4}"
                ),
                width, height
            ),
            None,
            None,
        ));
    }
    errors
}

pub fn lint_path(path: &str, bbox: ViewBox) -> Vec<LintError> {
    let mut errors: Vec<LintError> = vec![];
    errors.extend(path_format(path));
    errors.extend(negative_zeros(path));
    errors.extend(icon_size(path, bbox));
    errors
}

/// A rewriting in Rust of some rules linting SVGs in the Simple Icons repository.

static PATH_VALID_CHARACTERS: &str = "mMzZlLhHvVcCsSqQtTaAeE0123456789,.- ";
static NUMBERS: &str = "0123456789";

type Path = String;
type Range = (u32, u32);
pub type LintErrorFix = (Path, Range);
pub type LintErrorFixer = &'static dyn Fn(&str, Range) -> LintErrorFix;
pub type LintError = (Path, Option<Range>, Option<LintErrorFixer>);

pub type PathViewBox = (f64, f64, f64, f64);
pub type PathSegments = Vec<(String, Vec<f64>)>;

fn get_max_decimals_in_numbers(numbers: &[f64]) -> u32 {
    let mut max_decimals = 0;
    for number in numbers.iter() {
        // Get number of decimals in f64:
        let decimals = number.to_string().split('.').last().unwrap().len();
        if decimals > max_decimals {
            max_decimals = decimals;
        }
    }
    max_decimals as u32
}

fn round_decimal(number: f64, decimals: u32) -> f64 {
    let factor = 10.0_f64.powi(decimals as i32);
    (number * factor).round() / factor
}

/// Lint error fixer function that removes all characters in the range.
fn fix_removing_characters_in_range(path: &str, range: Range) -> LintErrorFix {
    let mut new_path = String::with_capacity(path.len());
    for (i, character) in path.chars().enumerate() {
        if (i as u32) >= range.0 && (i as u32) < range.1 {
            continue;
        }
        new_path.push(character)
    }
    (new_path, range)
}

/// Lint error fixer function that fixes a path that does not start
/// with a moveto command.
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
                Some(&fix_removing_characters_in_range),
            ));
        }
    }

    errors
}

/// Lint error fixer function that fixes a negative zero in the range.
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

/// Check if the path contains negative zeros.
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

/// Check if the icon size is 24 x 24 pixels.
pub fn icon_size(bbox: &PathViewBox) -> Vec<LintError> {
    let width = round_decimal(bbox.2 - bbox.0, 3);
    let height = round_decimal(bbox.3 - bbox.1, 3);
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

/// Check if the icon precision is less than 6 decimal places.
pub fn icon_precision(segments: &PathSegments) -> Vec<LintError> {
    let mut errors: Vec<LintError> = vec![];

    for (_command, args) in segments.iter() {
        let max_precision = get_max_decimals_in_numbers(args);
        if max_precision > 5 {
            // TODO: CST SVG path parser with input validation to fix this rule
            // and show the exact segments in linting errors.
            errors.push((
                format!(
                    concat!(
                        "Maximum precision should not be greater than 5,",
                        " currently {}"
                    ),
                    max_precision
                ),
                None,
                None,
            ));
            break;
        }
    }

    errors
}

/// Check if the icon is centered at 0, 0.
pub fn icon_centered(bbox: &PathViewBox) -> Vec<LintError> {
    let mut errors: Vec<LintError> = vec![];

    let center_x = round_decimal((bbox.2 + bbox.0) / 2.0, 3);
    let deviance_x = (center_x - 12.0).abs();
    let center_y = round_decimal((bbox.3 + bbox.1) / 2.0, 3);
    let deviance_y = (center_y - 12.0).abs();

    let icon_tolerance = 0.001;
    if deviance_x > icon_tolerance || deviance_y > icon_tolerance {
        errors.push((
            format!(
                concat!(
                    "Icon must be centered at (12, 12), currently at",
                    " {}, {}"
                ),
                center_x, center_y
            ),
            None,
            None,
        ));
    }

    errors
}

pub fn lint_path(
    path: &str,
    bbox: &PathViewBox,
    segments: &PathSegments,
) -> Vec<LintError> {
    let mut errors: Vec<LintError> = path_format(path);
    errors.extend(negative_zeros(path));
    errors.extend(icon_size(bbox));
    errors.extend(icon_precision(segments));
    errors.extend(icon_centered(bbox));
    errors
}

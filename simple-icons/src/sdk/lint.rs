/// A rewriting in Rust of some rules linting SVGs in the Simple Icons repository.

static PATH_VALID_CHARACTERS: &str = "mMzZlLhHvVcCsSqQtTaAeE0123456789,.- ";
static NUMBERS: &str = "0123456789";
static STRAIGHT_LINE_PATH_COMMANDS: &str = "HhVvLlMm";

type Path = String;
type Range = (u32, u32);
pub type LintErrorFix = (Path, Range);
pub type LintErrorFixer = &'static dyn Fn(&str, Range) -> LintErrorFix;
pub type LintError = (Path, Option<Range>, Option<LintErrorFixer>);

pub type PathViewBox = (f64, f64, f64, f64);
pub type PathSegment = (String, Vec<f64>);

fn get_number_of_decimals(number: f64) -> u32 {
    number.to_string().split('.').last().unwrap().len() as u32
}

fn get_max_decimals_in_numbers(numbers: &[f64]) -> u32 {
    let mut max_decimals = 0;
    for number in numbers.iter() {
        // Get number of decimals in f64:
        let decimals = get_number_of_decimals(number.to_owned());
        if decimals > max_decimals {
            max_decimals = decimals;
        }
    }
    max_decimals
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
            "Size was reported as 0 x 0 so check if the path is valid"
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
pub fn icon_precision(segments: &[PathSegment]) -> Vec<LintError> {
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
                    " ({}, {})"
                ),
                center_x, center_y
            ),
            None,
            None,
        ));
    }

    errors
}

/// Given three points, returns if the middle one (x2, y2) is collinear
/// to the line formed by the two limit points.
fn points_are_collinear(
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    x3: f64,
    y3: f64,
) -> bool {
    x1 * (y2 - y3) + x2 * (y3 - y1) + x3 * (y1 - y2) == 0.0
}

/// Check if the icon has collinear segments.
pub fn collinear_segments(
    path: &str,
    segments: &[PathSegment],
) -> Vec<LintError> {
    let mut errors: Vec<LintError> = vec![];

    let mut current_line: Vec<(f64, f64)> = vec![];
    let mut current_abs_coordinate: (Option<f64>, Option<f64>) = (None, None);
    let mut start_point: Option<(f64, f64)> = None;
    let mut in_straight_line = false;
    let mut reset_start_point = false;

    for (s, (command, args)) in segments.iter().enumerate() {
        let next_segment: Option<(String, Vec<f64>)> =
            segments.get(s + 1).cloned();

        if command == "M" {
            current_abs_coordinate = (Some(args[0]), Some(args[1]));
            start_point = None;
        } else if command == "m" {
            current_abs_coordinate = (
                Some(args[0] + current_abs_coordinate.0.unwrap_or(0.0)),
                Some(args[1] + current_abs_coordinate.1.unwrap_or(0.0)),
            );
            start_point = None;
        } else if command == "H" {
            current_abs_coordinate = (Some(args[0]), current_abs_coordinate.1);
        } else if command == "h" {
            current_abs_coordinate = (
                Some(args[0] + current_abs_coordinate.0.unwrap_or(0.0)),
                current_abs_coordinate.1,
            );
        } else if command == "V" {
            current_abs_coordinate = (current_abs_coordinate.0, Some(args[0]));
        } else if command == "v" {
            current_abs_coordinate = (
                current_abs_coordinate.0,
                Some(args[0] + current_abs_coordinate.1.unwrap_or(0.0)),
            );
        } else if command == "L" {
            current_abs_coordinate = (Some(args[0]), Some(args[1]));
        } else if command == "l" {
            current_abs_coordinate = (
                Some(args[0] + current_abs_coordinate.0.unwrap_or(0.0)),
                Some(args[1] + current_abs_coordinate.1.unwrap_or(0.0)),
            );
        } else if command == "Z" || command == "z" {
            let (x, y) = start_point.unwrap_or((0.0, 0.0));
            current_abs_coordinate = (Some(x), Some(y));
            reset_start_point = true;
        } else if command == "C" {
            current_abs_coordinate = (Some(args[4]), Some(args[5]));
        } else if command == "c" {
            current_abs_coordinate = (
                Some(args[4] + current_abs_coordinate.0.unwrap_or(0.0)),
                Some(args[5] + current_abs_coordinate.1.unwrap_or(0.0)),
            );
        } else if command == "A" {
            current_abs_coordinate = (Some(args[5]), Some(args[6]));
        } else if command == "a" {
            current_abs_coordinate = (
                Some(args[5] + current_abs_coordinate.0.unwrap_or(0.0)),
                Some(args[6] + current_abs_coordinate.1.unwrap_or(0.0)),
            );
        } else if command == "S" {
            current_abs_coordinate = (Some(args[0]), Some(args[1]));
        } else if command == "s" {
            current_abs_coordinate = (
                Some(args[2] + current_abs_coordinate.0.unwrap_or(0.0)),
                Some(args[3] + current_abs_coordinate.1.unwrap_or(0.0)),
            );
        } else if command == "Q" {
            current_abs_coordinate = (Some(args[2]), Some(args[3]));
        } else if command == "q" {
            current_abs_coordinate = (
                Some(args[2] + current_abs_coordinate.0.unwrap_or(0.0)),
                Some(args[3] + current_abs_coordinate.1.unwrap_or(0.0)),
            );
        } else if command == "T" {
            current_abs_coordinate = (Some(args[0]), Some(args[1]));
        } else if command == "t" {
            current_abs_coordinate = (
                Some(args[0] + current_abs_coordinate.0.unwrap_or(0.0)),
                Some(args[1] + current_abs_coordinate.1.unwrap_or(0.0)),
            );
        } else {
            // unknown command
            let command_index_range: Option<(u32, u32)> =
                path.find(command).map(|index| {
                    (index as u32, index as u32 + command.len() as u32)
                });
            errors.push((
                format!(
                    "Unknown command \"{}\"{}",
                    command,
                    &match command_index_range {
                        Some(range) => format!(" at index {}", range.0),
                        None => "".to_string(),
                    }
                ),
                command_index_range,
                None,
            ));
            break;
        }

        if start_point.is_none() {
            start_point = Some((
                current_abs_coordinate.0.unwrap(),
                current_abs_coordinate.1.unwrap(),
            ));
        } else if reset_start_point {
            start_point = None;
            reset_start_point = false;
        }

        let exiting_straight_line = in_straight_line
            && !(next_segment.is_some()
                && STRAIGHT_LINE_PATH_COMMANDS
                    .contains(&next_segment.unwrap().0));
        in_straight_line = STRAIGHT_LINE_PATH_COMMANDS.contains(command);

        if in_straight_line {
            current_line.push((
                current_abs_coordinate.0.unwrap(),
                current_abs_coordinate.1.unwrap(),
            ));
        } else {
            if exiting_straight_line {
                if STRAIGHT_LINE_PATH_COMMANDS.contains(command) {
                    current_line.push((
                        current_abs_coordinate.0.unwrap(),
                        current_abs_coordinate.1.unwrap(),
                    ));
                }

                for p in 1..current_line.len() - 1 {
                    let (x1, y1) = current_line[p - 1];
                    let (x2, y2) = current_line[p];
                    let (x3, y3) = current_line[p + 1];

                    if points_are_collinear(x1, y1, x2, y2, x3, y3) {
                        let (collinear_segment_command, _) = segments
                            .get(s - current_line.len() + p + 1)
                            .unwrap();
                        errors.push((
                            format!(
                                concat!(
                                    "Collinear segment found at command \"{}\""
                                ),
                                collinear_segment_command
                            ),
                            // TODO: CST SVG path parser with input validation
                            // to fix most variants of this rule and show the
                            // exact segments in errors
                            None,
                            None,
                        ));
                    }
                }
            }
            current_line.clear();
        }
    }

    errors
}

pub fn lint_path(
    path: &str,
    bbox: &PathViewBox,
    segments: &[PathSegment],
) -> Vec<LintError> {
    let mut errors: Vec<LintError> = path_format(path);
    errors.extend(negative_zeros(path));
    errors.extend(icon_size(bbox));
    errors.extend(icon_precision(segments));
    errors.extend(icon_centered(bbox));
    errors.extend(collinear_segments(path, segments));
    errors
}

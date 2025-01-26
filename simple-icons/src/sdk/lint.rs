/// A rewriting in Rust of some rules linting SVGs in the Simple Icons repository.
use svg_path_cst::{SVGPathCSTNode, SVGPathCommand, SVGPathSegment};

static PATH_VALID_CHARACTERS: &str = "mMzZlLhHvVcCsSqQtTaAeE0123456789,.- ";
static NUMBERS: &str = "0123456789";
static STRAIGHT_LINE_PATH_COMMANDS: &str = "HhVvLlMm";

static ICON_MAX_FLOAT_PRECISION: u32 = 5;

pub mod errors {
    use snafu::prelude::*;

    /// Syntax errors that can occur when parsing an SVG path
    ///
    /// These errors try to be exhaustive.
    #[derive(Debug, PartialEq, Snafu, Clone)]
    pub enum PathLintError {
        /// The first command in a path is not moveto
        #[snafu(display(
            "Must start with \"moveto\" command (\"M\" or \"m\"), but starts with \"{command}\""
        ))]
        MustStartWithMovetoCommand {
            /// Command letter found
            command: String,
        },

        /// Invalid character at index
        #[snafu(display(
            "Contains invalid character \"{character}\" at index {index}"
        ))]
        InvalidCharacterAtIndex {
            /// Invalid character
            character: char,
            /// Index of the invalid character
            index: u32,
        },

        /// Found negative zero at index
        #[snafu(display("Found \"-0\" at index {index}"))]
        FoundNegativeZeroAtIndex {
            /// Index of the negative zero
            index: u32,
        },

        /// Size of an icon reported as 0x0 px
        #[snafu(display(
            "Size was reported as 0 x 0 so check if the path is valid"
        ))]
        ReportedSizeIsZero,

        /// Maximum precision must be less than `ICON_MAX_FLOAT_PRECISION`
        #[snafu(display(
            "Maximum precision should not be greater than {max_precision}, currently {precision} for number \"{number}\""
        ))]
        MaximumPrecisionMustBeLessThan {
            /// Maximum precision allowed
            max_precision: u32,
            /// Precision of the number
            precision: u32,
            /// Number that has too much precision
            number: String,
        },

        /// Icon must be centered at (12, 12)
        #[snafu(display(
            "Icon must be centered at (12, 12), currently at ({x}, {y})"
        ))]
        IconMustBeCentered {
            /// X coordinate of the center
            x: f64,
            /// Y coordinate of the center
            y: f64,
        },

        /// Collinear segment found at command
        #[snafu(display("Collinear segment found at command \"{command}\""))]
        CollinearSegmentFoundAtCommand {
            /// Command letter
            command: char,
        },

        /// Incorrect icon size
        #[snafu(display("Size must be exactly 24 pixels in one dimension, currently {width} x {height}"))]
        IncorrectIconSize {
            /// Width of the icon
            width: f64,
            /// Height of the icon
            height: f64,
        },

        /// SVG syntax error
        #[snafu(display("Syntax error: {message}"))]
        SyntaxError {
            /// Error message
            message: String,
        },

        /// Incorrect parsing in viewbox
        #[snafu(display("Syntax error parsing viewbox: {message}"))]
        ViewboxSyntaxError {
            /// Error message
            message: String,
        },
    }
}

type Path = String;
type Range = (u32, u32);
pub type LintErrorFix = (Path, Range);
pub type LintErrorFixer =
    &'static (dyn Fn(&str, Range) -> LintErrorFix + std::marker::Sync);
pub type LintError =
    (errors::PathLintError, Option<Range>, Option<LintErrorFixer>);

pub type PathViewBox = (f64, f64, f64, f64);

fn get_number_of_decimals(number: f64) -> u32 {
    number.to_string().split('.').next_back().unwrap().len() as u32
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
            errors::PathLintError::MustStartWithMovetoCommand {
                command: first_char,
            },
            Some((0, 1)),
            Some(&fix_path_not_starts_with_moveto_command),
        ));
    }

    for (i, character) in path.chars().enumerate() {
        if !PATH_VALID_CHARACTERS.contains(character) {
            errors.push((
                errors::PathLintError::InvalidCharacterAtIndex {
                    character,
                    index: i as u32,
                },
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
                errors::PathLintError::FoundNegativeZeroAtIndex {
                    index: i as u32,
                },
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
        errors.push((errors::PathLintError::ReportedSizeIsZero, None, None));
    } else if width != 24.0 && height != 24.0 {
        errors.push((
            errors::PathLintError::IncorrectIconSize { width, height },
            None,
            None,
        ));
    }
    errors
}

/// Check if the icon precision is less than 6 decimal places.
pub fn icon_precision(cst: &[SVGPathCSTNode]) -> Vec<LintError> {
    let mut errors: Vec<LintError> = vec![];
    let mut prev_segment_is_sign = false;
    for node in cst.iter() {
        if let SVGPathCSTNode::Segment(segment) = node {
            for segment_node in segment.cst.iter() {
                if let SVGPathCSTNode::Number {
                    raw_number,
                    start,
                    end,
                    value,
                } = segment_node
                {
                    let number_precision = get_number_of_decimals(*value);
                    if number_precision > ICON_MAX_FLOAT_PRECISION {
                        errors.push((
                            errors::PathLintError::MaximumPrecisionMustBeLessThan {
                                max_precision: ICON_MAX_FLOAT_PRECISION,
                                precision: number_precision,
                                number: raw_number.to_string(),
                            },
                            Some((
                                (*start
                                    - if prev_segment_is_sign { 1 } else { 0 })
                                    as u32,
                                *end as u32,
                            )),
                            // TODO: fixes
                            None,
                        ));
                    }
                    prev_segment_is_sign = false;
                } else if let SVGPathCSTNode::Sign { .. } = segment_node {
                    prev_segment_is_sign = true;
                } else if prev_segment_is_sign {
                    prev_segment_is_sign = false;
                }
            }
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
            errors::PathLintError::IconMustBeCentered {
                x: center_x,
                y: center_y,
            },
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

fn find_next_segment(
    nodes: &[SVGPathCSTNode],
    index: usize,
) -> Option<SVGPathCSTNode> {
    for node in nodes.iter().skip(index) {
        if let SVGPathCSTNode::Segment(segment) = node {
            return Some(SVGPathCSTNode::Segment(segment.to_owned()));
        }
    }
    None
}

/// Check if the icon has collinear segments.
pub fn collinear_segments(segments: &[SVGPathCSTNode]) -> Vec<LintError> {
    let mut errors: Vec<LintError> = vec![];

    let mut current_line: Vec<(f64, f64)> = vec![];
    let mut current_abs_coordinate: (Option<f64>, Option<f64>) = (None, None);
    let mut start_point: Option<(f64, f64)> = None;
    let mut in_straight_line = false;
    let mut reset_start_point = false;

    for (s, node) in segments.iter().enumerate() {
        if let SVGPathCSTNode::Segment(segment) = &node {
            let args = &segment.args;
            let next_segment = find_next_segment(segments, s + 1);

            match segment.command {
                SVGPathCommand::MovetoUpper => {
                    current_abs_coordinate = (Some(args[0]), Some(args[1]));
                    start_point = None;
                }
                SVGPathCommand::MovetoLower => {
                    current_abs_coordinate = (
                        Some(args[0] + current_abs_coordinate.0.unwrap_or(0.0)),
                        Some(args[1] + current_abs_coordinate.1.unwrap_or(0.0)),
                    );
                    start_point = None;
                }
                SVGPathCommand::HorizontalUpper => {
                    current_abs_coordinate =
                        (Some(args[0]), current_abs_coordinate.1);
                }
                SVGPathCommand::HorizontalLower => {
                    current_abs_coordinate = (
                        Some(args[0] + current_abs_coordinate.0.unwrap_or(0.0)),
                        current_abs_coordinate.1,
                    );
                }
                SVGPathCommand::VerticalUpper => {
                    current_abs_coordinate =
                        (current_abs_coordinate.0, Some(args[0]));
                }
                SVGPathCommand::VerticalLower => {
                    current_abs_coordinate = (
                        current_abs_coordinate.0,
                        Some(args[0] + current_abs_coordinate.1.unwrap_or(0.0)),
                    );
                }
                SVGPathCommand::LinetoUpper => {
                    current_abs_coordinate = (Some(args[0]), Some(args[1]));
                }
                SVGPathCommand::LinetoLower => {
                    current_abs_coordinate = (
                        Some(args[0] + current_abs_coordinate.0.unwrap_or(0.0)),
                        Some(args[1] + current_abs_coordinate.1.unwrap_or(0.0)),
                    );
                }
                SVGPathCommand::ClosepathUpper
                | SVGPathCommand::ClosepathLower => {
                    let (x, y) = start_point.unwrap_or((0.0, 0.0));
                    current_abs_coordinate = (Some(x), Some(y));
                    reset_start_point = true;
                }
                SVGPathCommand::CurvetoUpper => {
                    current_abs_coordinate = (Some(args[4]), Some(args[5]));
                }
                SVGPathCommand::CurvetoLower => {
                    current_abs_coordinate = (
                        Some(args[4] + current_abs_coordinate.0.unwrap_or(0.0)),
                        Some(args[5] + current_abs_coordinate.1.unwrap_or(0.0)),
                    );
                }
                SVGPathCommand::ArcUpper => {
                    current_abs_coordinate = (Some(args[5]), Some(args[6]));
                }
                SVGPathCommand::ArcLower => {
                    current_abs_coordinate = (
                        Some(args[5] + current_abs_coordinate.0.unwrap_or(0.0)),
                        Some(args[6] + current_abs_coordinate.1.unwrap_or(0.0)),
                    );
                }
                SVGPathCommand::SmoothCurvetoUpper => {
                    current_abs_coordinate = (Some(args[2]), Some(args[3]));
                }
                SVGPathCommand::SmoothCurvetoLower => {
                    current_abs_coordinate = (
                        Some(args[2] + current_abs_coordinate.0.unwrap_or(0.0)),
                        Some(args[3] + current_abs_coordinate.1.unwrap_or(0.0)),
                    );
                }
                SVGPathCommand::QuadraticUpper => {
                    current_abs_coordinate = (Some(args[2]), Some(args[3]));
                }
                SVGPathCommand::QuadraticLower => {
                    current_abs_coordinate = (
                        Some(args[2] + current_abs_coordinate.0.unwrap_or(0.0)),
                        Some(args[3] + current_abs_coordinate.1.unwrap_or(0.0)),
                    );
                }
                SVGPathCommand::SmoothQuadraticUpper => {
                    current_abs_coordinate = (Some(args[0]), Some(args[1]));
                }
                SVGPathCommand::SmoothQuadraticLower => {
                    current_abs_coordinate = (
                        Some(args[0] + current_abs_coordinate.0.unwrap_or(0.0)),
                        Some(args[1] + current_abs_coordinate.1.unwrap_or(0.0)),
                    );
                }
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

            let mut next_segment_is_straight_line = false;
            if let Some(SVGPathCSTNode::Segment(ref next_seg)) = next_segment {
                next_segment_is_straight_line = STRAIGHT_LINE_PATH_COMMANDS
                    .contains(*next_seg.command as u8 as char);
            }

            let exiting_straight_line = in_straight_line
                && !(next_segment.is_some() && next_segment_is_straight_line);
            in_straight_line = STRAIGHT_LINE_PATH_COMMANDS
                .contains(*segment.command as u8 as char);

            if in_straight_line {
                current_line.push((
                    current_abs_coordinate.0.unwrap(),
                    current_abs_coordinate.1.unwrap(),
                ));
            } else {
                if exiting_straight_line {
                    if STRAIGHT_LINE_PATH_COMMANDS
                        .contains(*segment.command as u8 as char)
                    {
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
                            if let SVGPathCSTNode::Segment(SVGPathSegment {
                                command,
                                start,
                                end,
                                ..
                            }) = segments
                                .get(s - current_line.len() + p + 1)
                                .unwrap()
                            {
                                errors.push((
                                    errors::PathLintError::CollinearSegmentFoundAtCommand {
                                        command: **command as u8 as char,
                                    },
                                    // TODO: show complete range including
                                    //  previous and next segments
                                    Some((*start as u32, *end as u32)),
                                    // TODO: fix most variants of this rule
                                    None,
                                ));
                            }
                        }
                    }
                }
                current_line.clear();
            }
        }
    }

    errors
}

pub fn lint_path_characters(path: &str) -> Vec<LintError> {
    let mut errors: Vec<LintError> = path_format(path);
    errors.extend(negative_zeros(path));
    errors
}

pub fn lint_path_bbox(bbox: &PathViewBox) -> Vec<LintError> {
    let mut errors: Vec<LintError> = icon_size(bbox);
    errors.extend(icon_centered(bbox));
    errors
}

pub fn lint_path_segments(cst: &[SVGPathCSTNode]) -> Vec<LintError> {
    let mut errors: Vec<LintError> = icon_precision(cst);
    errors.extend(collinear_segments(cst));
    errors
}

//! Quantum Error Correction Parser
//!
//! Production-grade parser for QEC-specific textual representations.
//!
//! # Responsibility
//!
//! This module parses QEC data/configuration. It is deliberately NOT a
//! replacement for Zamani's general source-language parser.
//!
//! The intended pipeline is:
//!
//! ```text
//! Zamani source
//!      |
//!      v
//! src/parser
//!      |
//!      v
//! QEC syntax / configuration
//!      |
//!      v
//! error_correction::parser
//!      |
//!      v
//! ParsedQecDocument
//!      |
//!      v
//! validation.rs
//!      |
//!      v
//! Validated QEC object
//! ```
//!
//! Parsing and mathematical validation are intentionally separate.
//!
//! The parser:
//! - never performs unchecked indexing;
//! - never panics on malformed external input;
//! - rejects malformed syntax deterministically;
//! - enforces configurable input-size limits;
//! - detects duplicate fields;
//! - detects malformed numbers;
//! - rejects NaN/infinite floating-point values;
//! - detects integer overflow;
//! - preserves deterministic ordering;
//! - does not allocate unboundedly from untrusted input;
//! - does not perform decoding;
//! - does not silently repair malformed input.
//!
//! Mathematical topology/stabilizer validation belongs in `validation.rs`.

use std::fmt;
use std::str::FromStr;

/// Maximum number of bytes accepted by the parser by default.
///
/// This is deliberately conservative. Applications handling larger QEC
/// documents should construct an explicit `ParserLimits`.
pub const DEFAULT_MAX_INPUT_BYTES: usize = 16 * 1024 * 1024;

/// Maximum number of lines accepted by the parser by default.
pub const DEFAULT_MAX_LINES: usize = 1_000_000;

/// Maximum length of a single line.
pub const DEFAULT_MAX_LINE_BYTES: usize = 1024 * 1024;

/// Maximum number of fields in a document.
pub const DEFAULT_MAX_FIELDS: usize = 100_000;

/// Maximum number of qubit declarations.
pub const DEFAULT_MAX_QUBITS: usize = 10_000_000;

/// Maximum number of stabilizer declarations.
pub const DEFAULT_MAX_STABILIZERS: usize = 10_000_000;

/// Maximum number of logical-operator declarations.
pub const DEFAULT_MAX_LOGICAL_OPERATORS: usize = 1_000_000;

/// Parser resource policy.
///
/// These limits protect the parser from allocation bombs and maliciously
/// large external QEC documents.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParserLimits {
    pub max_input_bytes: usize,
    pub max_lines: usize,
    pub max_line_bytes: usize,
    pub max_fields: usize,
    pub max_qubits: usize,
    pub max_stabilizers: usize,
    pub max_logical_operators: usize,
}

impl Default for ParserLimits {
    fn default() -> Self {
        Self {
            max_input_bytes: DEFAULT_MAX_INPUT_BYTES,
            max_lines: DEFAULT_MAX_LINES,
            max_line_bytes: DEFAULT_MAX_LINE_BYTES,
            max_fields: DEFAULT_MAX_FIELDS,
            max_qubits: DEFAULT_MAX_QUBITS,
            max_stabilizers: DEFAULT_MAX_STABILIZERS,
            max_logical_operators: DEFAULT_MAX_LOGICAL_OPERATORS,
        }
    }
}

/// Parser error.
///
/// Errors are explicit and deterministic so callers can distinguish malformed
/// input from resource exhaustion and semantic configuration problems.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParserError {
    EmptyInput,

    InputTooLarge {
        actual: usize,
        maximum: usize,
    },

    TooManyLines {
        actual: usize,
        maximum: usize,
    },

    LineTooLong {
        line: usize,
        actual: usize,
        maximum: usize,
    },

    TooManyFields {
        actual: usize,
        maximum: usize,
    },

    TooManyQubits {
        actual: usize,
        maximum: usize,
    },

    TooManyStabilizers {
        actual: usize,
        maximum: usize,
    },

    TooManyLogicalOperators {
        actual: usize,
        maximum: usize,
    },

    InvalidSyntax {
        line: usize,
        message: String,
    },

    MissingValue {
        line: usize,
        field: String,
    },

    UnknownField {
        line: usize,
        field: String,
    },

    DuplicateField {
        line: usize,
        field: String,
    },

    InvalidInteger {
        line: usize,
        field: String,
        value: String,
    },

    IntegerOverflow {
        line: usize,
        field: String,
        value: String,
    },

    InvalidFloat {
        line: usize,
        field: String,
        value: String,
    },

    NonFiniteFloat {
        line: usize,
        field: String,
        value: String,
    },

    InvalidBoolean {
        line: usize,
        field: String,
        value: String,
    },

    InvalidPauli {
        line: usize,
        field: String,
        value: String,
    },

    InvalidCoordinate {
        line: usize,
        field: String,
        value: String,
    },

    InvalidIdentifier {
        line: usize,
        field: String,
        value: String,
    },

    InvalidProbability {
        line: usize,
        field: String,
        value: String,
    },

    MissingRequiredField {
        field: String,
    },

    UnsupportedVersion {
        version: String,
    },

    InvalidEncoding,
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyInput => write!(f, "QEC input is empty"),

            Self::InputTooLarge { actual, maximum } => write!(
                f,
                "QEC input is too large: {actual} bytes exceeds limit {maximum}"
            ),

            Self::TooManyLines { actual, maximum } => write!(
                f,
                "QEC input contains too many lines: {actual} exceeds limit {maximum}"
            ),

            Self::LineTooLong {
                line,
                actual,
                maximum,
            } => write!(
                f,
                "line {line} is too long: {actual} bytes exceeds limit {maximum}"
            ),

            Self::TooManyFields { actual, maximum } => write!(
                f,
                "QEC document contains too many fields: {actual} exceeds limit {maximum}"
            ),

            Self::TooManyQubits { actual, maximum } => write!(
                f,
                "QEC document contains too many qubits: {actual} exceeds limit {maximum}"
            ),

            Self::TooManyStabilizers { actual, maximum } => write!(
                f,
                "QEC document contains too many stabilizers: {actual} exceeds limit {maximum}"
            ),

            Self::TooManyLogicalOperators { actual, maximum } => write!(
                f,
                "QEC document contains too many logical operators: {actual} exceeds limit {maximum}"
            ),

            Self::InvalidSyntax { line, message } => {
                write!(f, "invalid QEC syntax on line {line}: {message}")
            }

            Self::MissingValue { line, field } => {
                write!(f, "missing value for '{field}' on line {line}")
            }

            Self::UnknownField { line, field } => {
                write!(f, "unknown QEC field '{field}' on line {line}")
            }

            Self::DuplicateField { line, field } => {
                write!(f, "duplicate QEC field '{field}' on line {line}")
            }

            Self::InvalidInteger {
                line,
                field,
                value,
            } => write!(
                f,
                "invalid integer for '{field}' on line {line}: '{value}'"
            ),

            Self::IntegerOverflow {
                line,
                field,
                value,
            } => write!(
                f,
                "integer overflow for '{field}' on line {line}: '{value}'"
            ),

            Self::InvalidFloat {
                line,
                field,
                value,
            } => write!(
                f,
                "invalid floating-point value for '{field}' on line {line}: '{value}'"
            ),

            Self::NonFiniteFloat {
                line,
                field,
                value,
            } => write!(
                f,
                "non-finite floating-point value for '{field}' on line {line}: '{value}'"
            ),

            Self::InvalidBoolean {
                line,
                field,
                value,
            } => write!(
                f,
                "invalid boolean for '{field}' on line {line}: '{value}'"
            ),

            Self::InvalidPauli {
                line,
                field,
                value,
            } => write!(
                f,
                "invalid Pauli value for '{field}' on line {line}: '{value}'"
            ),

            Self::InvalidCoordinate {
                line,
                field,
                value,
            } => write!(
                f,
                "invalid coordinate for '{field}' on line {line}: '{value}'"
            ),

            Self::InvalidIdentifier {
                line,
                field,
                value,
            } => write!(
                f,
                "invalid identifier for '{field}' on line {line}: '{value}'"
            ),

            Self::InvalidProbability {
                line,
                field,
                value,
            } => write!(
                f,
                "invalid probability for '{field}' on line {line}: '{value}'"
            ),

            Self::MissingRequiredField { field } => {
                write!(f, "missing required QEC field '{field}'")
            }

            Self::UnsupportedVersion { version } => {
                write!(f, "unsupported QEC document version '{version}'")
            }

            Self::InvalidEncoding => write!(f, "QEC input contains invalid UTF-8"),
        }
    }
}

impl std::error::Error for ParserError {}

/// Result type used by the QEC parser.
pub type ParseResult<T> = Result<T, ParserError>;

/// A two-dimensional QEC coordinate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coordinate {
    pub x: i64,
    pub y: i64,
}

impl Coordinate {
    pub const fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

/// A parsed qubit declaration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedQubit {
    pub id: u64,
    pub coordinate: Option<Coordinate>,
}

/// A parsed stabilizer declaration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedStabilizer {
    pub id: u64,
    pub pauli: Pauli,
    pub qubits: Vec<u64>,
}

/// Pauli operator used by the parsed representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Pauli {
    I,
    X,
    Y,
    Z,
}

impl FromStr for Pauli {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "I" | "i" => Ok(Self::I),
            "X" | "x" => Ok(Self::X),
            "Y" | "y" => Ok(Self::Y),
            "Z" | "z" => Ok(Self::Z),
            _ => Err(()),
        }
    }
}

/// A parsed logical operator.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedLogicalOperator {
    pub name: String,
    pub pauli: Pauli,
    pub qubits: Vec<u64>,
}

/// Parsed QEC document.
///
/// This structure represents syntactically valid input only.
///
/// It must still pass `validation.rs` before being consumed by a production
/// decoder.
#[derive(Debug, Clone, PartialEq)]
pub struct ParsedQecDocument {
    pub version: String,
    pub distance: Option<u64>,
    pub rounds: Option<u64>,
    pub measurement_error_probability: Option<f64>,
    pub qubits: Vec<ParsedQubit>,
    pub stabilizers: Vec<ParsedStabilizer>,
    pub logical_operators: Vec<ParsedLogicalOperator>,
}

impl ParsedQecDocument {
    fn new(version: String) -> Self {
        Self {
            version,
            distance: None,
            rounds: None,
            measurement_error_probability: None,
            qubits: Vec::new(),
            stabilizers: Vec::new(),
            logical_operators: Vec::new(),
        }
    }
}

/// Production QEC parser.
#[derive(Debug, Clone)]
pub struct QecParser {
    limits: ParserLimits,
}

impl Default for QecParser {
    fn default() -> Self {
        Self::new(ParserLimits::default())
    }
}

impl QecParser {
    pub const fn new(limits: ParserLimits) -> Self {
        Self { limits }
    }

    pub const fn limits(&self) -> &ParserLimits {
        &self.limits
    }

    /// Parse UTF-8 QEC text.
    ///
    /// Syntax is intentionally simple and deterministic:
    ///
    /// ```text
    /// version = 1
    /// distance = 5
    /// rounds = 10
    /// measurement_error_probability = 0.001
    ///
    /// qubit = 0
    /// qubit = 1,0,1
    ///
    /// stabilizer = 0,X,0,1,2
    /// stabilizer = 1,Z,1,2,3
    ///
    /// logical = X,X,0,1,2
    /// logical = Z,Z,0,3,6
    /// ```
    ///
    /// Comments begin with `#`.
    pub fn parse(&self, input: &str) -> ParseResult<ParsedQecDocument> {
        let input_bytes = input.len();

        if input_bytes == 0 {
            return Err(ParserError::EmptyInput);
        }

        if input_bytes > self.limits.max_input_bytes {
            return Err(ParserError::InputTooLarge {
                actual: input_bytes,
                maximum: self.limits.max_input_bytes,
            });
        }

        let mut document: Option<ParsedQecDocument> = None;
        let mut field_count = 0usize;
        let mut line_count = 0usize;

        let mut seen_distance = false;
        let mut seen_rounds = false;
        let mut seen_measurement_probability = false;

        for (index, raw_line) in input.lines().enumerate() {
            let line_number = index
                .checked_add(1)
                .ok_or(ParserError::TooManyLines {
                    actual: usize::MAX,
                    maximum: self.limits.max_lines,
                })?;

            line_count = line_number;

            if line_count > self.limits.max_lines {
                return Err(ParserError::TooManyLines {
                    actual: line_count,
                    maximum: self.limits.max_lines,
                });
            }

            if raw_line.len() > self.limits.max_line_bytes {
                return Err(ParserError::LineTooLong {
                    line: line_number,
                    actual: raw_line.len(),
                    maximum: self.limits.max_line_bytes,
                });
            }

            let line = raw_line
                .split_once('#')
                .map_or(raw_line, |(content, _)| content)
                .trim();

            if line.is_empty() {
                continue;
            }

            field_count = field_count
                .checked_add(1)
                .ok_or(ParserError::TooManyFields {
                    actual: usize::MAX,
                    maximum: self.limits.max_fields,
                })?;

            if field_count > self.limits.max_fields {
                return Err(ParserError::TooManyFields {
                    actual: field_count,
                    maximum: self.limits.max_fields,
                });
            }

            let (key, value) = line.split_once('=').ok_or_else(|| {
                ParserError::InvalidSyntax {
                    line: line_number,
                    message: "expected 'field = value'".to_string(),
                }
            })?;

            let key = key.trim();
            let value = value.trim();

            if key.is_empty() {
                return Err(ParserError::InvalidSyntax {
                    line: line_number,
                    message: "field name cannot be empty".to_string(),
                });
            }

            if value.is_empty() {
                return Err(ParserError::MissingValue {
                    line: line_number,
                    field: key.to_string(),
                });
            }

            match key {
                "version" => {
                    if document.is_some() {
                        return Err(ParserError::DuplicateField {
                            line: line_number,
                            field: key.to_string(),
                        });
                    }

                    let version = value.to_string();

                    if version != "1" {
                        return Err(ParserError::UnsupportedVersion { version });
                    }

                    document = Some(ParsedQecDocument::new(version));
                }

                "distance" => {
                    let document = document.as_mut().ok_or_else(|| {
                        ParserError::MissingRequiredField {
                            field: "version".to_string(),
                        }
                    })?;

                    if seen_distance {
                        return Err(ParserError::DuplicateField {
                            line: line_number,
                            field: key.to_string(),
                        });
                    }

                    let distance =
                        parse_u64(value, line_number, key)?;

                    if distance == 0 {
                        return Err(ParserError::InvalidInteger {
                            line: line_number,
                            field: key.to_string(),
                            value: value.to_string(),
                        });
                    }

                    document.distance = Some(distance);
                    seen_distance = true;
                }

                "rounds" => {
                    let document = document.as_mut().ok_or_else(|| {
                        ParserError::MissingRequiredField {
                            field: "version".to_string(),
                        }
                    })?;

                    if seen_rounds {
                        return Err(ParserError::DuplicateField {
                            line: line_number,
                            field: key.to_string(),
                        });
                    }

                    let rounds =
                        parse_u64(value, line_number, key)?;

                    if rounds == 0 {
                        return Err(ParserError::InvalidInteger {
                            line: line_number,
                            field: key.to_string(),
                            value: value.to_string(),
                        });
                    }

                    document.rounds = Some(rounds);
                    seen_rounds = true;
                }

                "measurement_error_probability" => {
                    let document = document.as_mut().ok_or_else(|| {
                        ParserError::MissingRequiredField {
                            field: "version".to_string(),
                        }
                    })?;

                    if seen_measurement_probability {
                        return Err(ParserError::DuplicateField {
                            line: line_number,
                            field: key.to_string(),
                        });
                    }

                    let probability =
                        parse_probability(value, line_number, key)?;

                    document.measurement_error_probability = Some(probability);
                    seen_measurement_probability = true;
                }

                "qubit" => {
                    let document = document.as_mut().ok_or_else(|| {
                        ParserError::MissingRequiredField {
                            field: "version".to_string(),
                        }
                    })?;

                    if document.qubits.len() >= self.limits.max_qubits {
                        return Err(ParserError::TooManyQubits {
                            actual: document.qubits.len().saturating_add(1),
                            maximum: self.limits.max_qubits,
                        });
                    }

                    let qubit = parse_qubit(value, line_number)?;

                    if document.qubits.iter().any(|existing| existing.id == qubit.id) {
                        return Err(ParserError::DuplicateField {
                            line: line_number,
                            field: format!("qubit {}", qubit.id),
                        });
                    }

                    document.qubits.push(qubit);
                }

                "stabilizer" => {
                    let document = document.as_mut().ok_or_else(|| {
                        ParserError::MissingRequiredField {
                            field: "version".to_string(),
                        }
                    })?;

                    if document.stabilizers.len() >= self.limits.max_stabilizers {
                        return Err(ParserError::TooManyStabilizers {
                            actual: document.stabilizers.len().saturating_add(1),
                            maximum: self.limits.max_stabilizers,
                        });
                    }

                    let stabilizer =
                        parse_stabilizer(value, line_number)?;

                    if document
                        .stabilizers
                        .iter()
                        .any(|existing| existing.id == stabilizer.id)
                    {
                        return Err(ParserError::DuplicateField {
                            line: line_number,
                            field: format!("stabilizer {}", stabilizer.id),
                        });
                    }

                    document.stabilizers.push(stabilizer);
                }

                "logical" => {
                    let document = document.as_mut().ok_or_else(|| {
                        ParserError::MissingRequiredField {
                            field: "version".to_string(),
                        }
                    })?;

                    if document.logical_operators.len()
                        >= self.limits.max_logical_operators
                    {
                        return Err(ParserError::TooManyLogicalOperators {
                            actual: document
                                .logical_operators
                                .len()
                                .saturating_add(1),
                            maximum: self.limits.max_logical_operators,
                        });
                    }

                    let logical =
                        parse_logical_operator(value, line_number)?;

                    if document
                        .logical_operators
                        .iter()
                        .any(|existing| existing.name == logical.name)
                    {
                        return Err(ParserError::DuplicateField {
                            line: line_number,
                            field: format!("logical {}", logical.name),
                        });
                    }

                    document.logical_operators.push(logical);
                }

                _ => {
                    return Err(ParserError::UnknownField {
                        line: line_number,
                        field: key.to_string(),
                    });
                }
            }
        }

        let document = document.ok_or_else(|| ParserError::MissingRequiredField {
            field: "version".to_string(),
        })?;

        Ok(document)
    }
}

fn parse_u64(value: &str, line: usize, field: &str) -> ParseResult<u64> {
    match value.parse::<u64>() {
        Ok(value) => Ok(value),
        Err(error) => {
            if error.kind() == &std::num::IntErrorKind::PosOverflow
                || error.kind() == &std::num::IntErrorKind::NegOverflow
            {
                Err(ParserError::IntegerOverflow {
                    line,
                    field: field.to_string(),
                    value: value.to_string(),
                })
            } else {
                Err(ParserError::InvalidInteger {
                    line,
                    field: field.to_string(),
                    value: value.to_string(),
                })
            }
        }
    }
}

fn parse_i64(value: &str, line: usize, field: &str) -> ParseResult<i64> {
    match value.parse::<i64>() {
        Ok(value) => Ok(value),
        Err(error) => {
            if error.kind() == &std::num::IntErrorKind::PosOverflow
                || error.kind() == &std::num::IntErrorKind::NegOverflow
            {
                Err(ParserError::IntegerOverflow {
                    line,
                    field: field.to_string(),
                    value: value.to_string(),
                })
            } else {
                Err(ParserError::InvalidInteger {
                    line,
                    field: field.to_string(),
                    value: value.to_string(),
                })
            }
        }
    }
}

fn parse_f64(value: &str, line: usize, field: &str) -> ParseResult<f64> {
    let parsed = value
        .parse::<f64>()
        .map_err(|_| ParserError::InvalidFloat {
            line,
            field: field.to_string(),
            value: value.to_string(),
        })?;

    if !parsed.is_finite() {
        return Err(ParserError::NonFiniteFloat {
            line,
            field: field.to_string(),
            value: value.to_string(),
        });
    }

    Ok(parsed)
}

fn parse_probability(value: &str, line: usize, field: &str) -> ParseResult<f64> {
    let probability = parse_f64(value, line, field)?;

    if !(0.0..=1.0).contains(&probability) {
        return Err(ParserError::InvalidProbability {
            line,
            field: field.to_string(),
            value: value.to_string(),
        });
    }

    Ok(probability)
}

fn parse_bool(value: &str, line: usize, field: &str) -> ParseResult<bool> {
    match value {
        "true" | "TRUE" | "True" => Ok(true),
        "false" | "FALSE" | "False" => Ok(false),
        _ => Err(ParserError::InvalidBoolean {
            line,
            field: field.to_string(),
            value: value.to_string(),
        }),
    }
}

fn parse_coordinate(
    value: &str,
    line: usize,
    field: &str,
) -> ParseResult<Coordinate> {
    let mut parts = value.split(',');

    let x = parts
        .next()
        .ok_or_else(|| ParserError::InvalidCoordinate {
            line,
            field: field.to_string(),
            value: value.to_string(),
        })?
        .trim();

    let y = parts
        .next()
        .ok_or_else(|| ParserError::InvalidCoordinate {
            line,
            field: field.to_string(),
            value: value.to_string(),
        })?
        .trim();

    if parts.next().is_some() {
        return Err(ParserError::InvalidCoordinate {
            line,
            field: field.to_string(),
            value: value.to_string(),
        });
    }

    Ok(Coordinate::new(
        parse_i64(x, line, field)?,
        parse_i64(y, line, field)?,
    ))
}

fn parse_identifier(
    value: &str,
    line: usize,
    field: &str,
) -> ParseResult<String> {
    if value.is_empty() {
        return Err(ParserError::InvalidIdentifier {
            line,
            field: field.to_string(),
            value: value.to_string(),
        });
    }

    let valid = value
        .chars()
        .enumerate()
        .all(|(index, character)| {
            if index == 0 {
                character == '_' || character.is_ascii_alphabetic()
            } else {
                character == '_'
                    || character == '-'
                    || character.is_ascii_alphanumeric()
            }
        });

    if !valid {
        return Err(ParserError::InvalidIdentifier {
            line,
            field: field.to_string(),
            value: value.to_string(),
        });
    }

    Ok(value.to_string())
}

fn parse_qubit(value: &str, line: usize) -> ParseResult<ParsedQubit> {
    let parts: Vec<&str> = value.split(',').map(str::trim).collect();

    match parts.as_slice() {
        [id] => Ok(ParsedQubit {
            id: parse_u64(id, line, "qubit")?,
            coordinate: None,
        }),

        [id, x, y] => Ok(ParsedQubit {
            id: parse_u64(id, line, "qubit")?,
            coordinate: Some(Coordinate::new(
                parse_i64(x, line, "qubit.x")?,
                parse_i64(y, line, "qubit.y")?,
            )),
        }),

        _ => Err(ParserError::InvalidSyntax {
            line,
            message:
                "qubit must be '<id>' or '<id>,<x>,<y>'".to_string(),
        }),
    }
}

fn parse_stabilizer(
    value: &str,
    line: usize,
) -> ParseResult<ParsedStabilizer> {
    let mut parts = value.split(',').map(str::trim);

    let id = parts
        .next()
        .ok_or_else(|| ParserError::InvalidSyntax {
            line,
            message: "stabilizer requires an id".to_string(),
        })?;

    let pauli = parts
        .next()
        .ok_or_else(|| ParserError::InvalidSyntax {
            line,
            message: "stabilizer requires a Pauli operator".to_string(),
        })?;

    let pauli = Pauli::from_str(pauli).map_err(|_| ParserError::InvalidPauli {
        line,
        field: "stabilizer.pauli".to_string(),
        value: pauli.to_string(),
    })?;

    let id = parse_u64(id, line, "stabilizer.id")?;

    let mut qubits = Vec::new();

    for part in parts {
        if part.is_empty() {
            return Err(ParserError::InvalidSyntax {
                line,
                message: "stabilizer contains an empty qubit id".to_string(),
            });
        }

        qubits.push(parse_u64(part, line, "stabilizer.qubit")?);
    }

    if qubits.is_empty() {
        return Err(ParserError::InvalidSyntax {
            line,
            message: "stabilizer must reference at least one qubit"
                .to_string(),
        });
    }

    Ok(ParsedStabilizer { id, pauli, qubits })
}

fn parse_logical_operator(
    value: &str,
    line: usize,
) -> ParseResult<ParsedLogicalOperator> {
    let mut parts = value.split(',').map(str::trim);

    let name = parts
        .next()
        .ok_or_else(|| ParserError::InvalidSyntax {
            line,
            message: "logical operator requires a name".to_string(),
        })?;

    let name = parse_identifier(name, line, "logical.name")?;

    let pauli = parts
        .next()
        .ok_or_else(|| ParserError::InvalidSyntax {
            line,
            message: "logical operator requires a Pauli operator".to_string(),
        })?;

    let pauli = Pauli::from_str(pauli).map_err(|_| ParserError::InvalidPauli {
        line,
        field: "logical.pauli".to_string(),
        value: pauli.to_string(),
    })?;

    let mut qubits = Vec::new();

    for part in parts {
        if part.is_empty() {
            return Err(ParserError::InvalidSyntax {
                line,
                message: "logical operator contains an empty qubit id"
                    .to_string(),
            });
        }

        qubits.push(parse_u64(part, line, "logical.qubit")?);
    }

    if qubits.is_empty() {
        return Err(ParserError::InvalidSyntax {
            line,
            message: "logical operator must reference at least one qubit"
                .to_string(),
        });
    }

    Ok(ParsedLogicalOperator {
        name,
        pauli,
        qubits,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_minimal_document() {
        let parser = QecParser::default();

        let document = parser
            .parse(
                r#"
                version = 1
                distance = 3
                "#,
            )
            .expect("valid QEC document");

        assert_eq!(document.version, "1");
        assert_eq!(document.distance, Some(3));
    }

    #[test]
    fn parses_qubits() {
        let parser = QecParser::default();

        let document = parser
            .parse(
                r#"
                version = 1
                qubit = 0,0,0
                qubit = 1,1,0
                "#,
            )
            .expect("valid QEC document");

        assert_eq!(document.qubits.len(), 2);
        assert_eq!(
            document.qubits[0].coordinate,
            Some(Coordinate::new(0, 0))
        );
    }

    #[test]
    fn parses_stabilizer() {
        let parser = QecParser::default();

        let document = parser
            .parse(
                r#"
                version = 1
                stabilizer = 0,X,0,1,2
                "#,
            )
            .expect("valid QEC document");

        assert_eq!(document.stabilizers.len(), 1);
        assert_eq!(document.stabilizers[0].pauli, Pauli::X);
        assert_eq!(document.stabilizers[0].qubits, vec![0, 1, 2]);
    }

    #[test]
    fn parses_logical_operator() {
        let parser = QecParser::default();

        let document = parser
            .parse(
                r#"
                version = 1
                logical = logical_x,X,0,1,2
                "#,
            )
            .expect("valid QEC document");

        assert_eq!(document.logical_operators.len(), 1);
        assert_eq!(document.logical_operators[0].name, "logical_x");
    }

    #[test]
    fn rejects_duplicate_qubits() {
        let parser = QecParser::default();

        let result = parser.parse(
            r#"
            version = 1
            qubit = 1
            qubit = 1
            "#,
        );

        assert!(matches!(
            result,
            Err(ParserError::DuplicateField { .. })
        ));
    }

    #[test]
    fn rejects_unknown_fields() {
        let parser = QecParser::default();

        let result = parser.parse(
            r#"
            version = 1
            unknown = value
            "#,
        );

        assert!(matches!(
            result,
            Err(ParserError::UnknownField { .. })
        ));
    }

    #[test]
    fn rejects_nan() {
        let parser = QecParser::default();

        let result = parser.parse(
            r#"
            version = 1
            measurement_error_probability = NaN
            "#,
        );

        assert!(matches!(
            result,
            Err(ParserError::NonFiniteFloat { .. })
        ));
    }

    #[test]
    fn rejects_infinite_probability() {
        let parser = QecParser::default();

        let result = parser.parse(
            r#"
            version = 1
            measurement_error_probability = inf
            "#,
        );

        assert!(matches!(
            result,
            Err(ParserError::NonFiniteFloat { .. })
        ));
    }

    #[test]
    fn rejects_probability_above_one() {
        let parser = QecParser::default();

        let result = parser.parse(
            r#"
            version = 1
            measurement_error_probability = 1.1
            "#,
        );

        assert!(matches!(
            result,
            Err(ParserError::InvalidProbability { .. })
        ));
    }

    #[test]
    fn rejects_probability_below_zero() {
        let parser = QecParser::default();

        let result = parser.parse(
            r#"
            version = 1
            measurement_error_probability = -0.1
            "#,
        );

        assert!(matches!(
            result,
            Err(ParserError::InvalidProbability { .. })
        ));
    }

    #[test]
    fn rejects_integer_overflow() {
        let parser = QecParser::default();

        let result = parser.parse(
            r#"
            version = 1
            distance = 18446744073709551616
            "#,
        );

        assert!(matches!(
            result,
            Err(ParserError::IntegerOverflow { .. })
        ));
    }

    #[test]
    fn rejects_unsupported_version() {
        let parser = QecParser::default();

        let result = parser.parse("version = 999");

        assert!(matches!(
            result,
            Err(ParserError::UnsupportedVersion { .. })
        ));
    }

    #[test]
    fn rejects_missing_version() {
        let parser = QecParser::default();

        let result = parser.parse("distance = 5");

        assert!(matches!(
            result,
            Err(ParserError::MissingRequiredField { .. })
        ));
    }

    #[test]
    fn enforces_input_limit() {
        let limits = ParserLimits {
            max_input_bytes: 10,
            ..ParserLimits::default()
        };

        let parser = QecParser::new(limits);

        let result = parser.parse("version = 1\n");

        assert!(matches!(
            result,
            Err(ParserError::InputTooLarge { .. })
        ));
    }

    #[test]
    fn enforces_qubit_limit() {
        let limits = ParserLimits {
            max_qubits: 1,
            ..ParserLimits::default()
        };

        let parser = QecParser::new(limits);

        let result = parser.parse(
            r#"
            version = 1
            qubit = 0
            qubit = 1
            "#,
        );

        assert!(matches!(
            result,
            Err(ParserError::TooManyQubits { .. })
        ));
    }

    #[test]
    fn comments_are_ignored() {
        let parser = QecParser::default();

        let document = parser
            .parse(
                r#"
                # QEC document
                version = 1 # document version
                distance = 5 # code distance
                "#,
            )
            .expect("valid QEC document");

        assert_eq!(document.distance, Some(5));
    }

    #[test]
    fn parsing_is_deterministic() {
        let parser = QecParser::default();

        let input = r#"
            version = 1
            distance = 5
            rounds = 10
            qubit = 0,0,0
            stabilizer = 0,X,0
            logical = logical_x,X,0
        "#;

        let first = parser.parse(input).expect("valid input");
        let second = parser.parse(input).expect("valid input");

        assert_eq!(first, second);
    }

    #[test]
    fn helper_boolean_parser_is_strict() {
        assert_eq!(
            parse_bool("true", 1, "enabled").expect("valid boolean"),
            true
        );

        assert_eq!(
            parse_bool("false", 1, "enabled").expect("valid boolean"),
            false
        );

        assert!(matches!(
            parse_bool("yes", 1, "enabled"),
            Err(ParserError::InvalidBoolean { .. })
        ));
    }

    #[test]
    fn coordinate_parser_rejects_extra_components() {
        let result = parse_coordinate("1,2,3", 1, "coordinate");

        assert!(matches!(
            result,
            Err(ParserError::InvalidCoordinate { .. })
        ));
    }

    #[test]
    fn identifier_parser_rejects_invalid_identifier() {
        let result = parse_identifier("123invalid", 1, "name");

        assert!(matches!(
            result,
            Err(ParserError::InvalidIdentifier { .. })
        ));
    }
}
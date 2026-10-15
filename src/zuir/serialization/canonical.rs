#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Canonical JSON Representation
//!
//! Path: `src/zuir/serialization/canonical.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: unsafe Rust is forbidden.
//!
//! PURPOSE
//!
//! Provide deterministic, compact JSON serialization for an already
//! constructed `serde_json::Value`. Object keys are ordered lexicographically,
//! array order is preserved, and no insignificant whitespace is emitted.
//!
//! AUTHORITY
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/version.rs`
//! - `src/zuir/serialization/schema.rs`
//! - `src/zuir/serialization/encode.rs`
//! - `src/zuir/serialization/decode.rs`
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/antlr/ZamaniLexer.g4`
//! - `grammar/antlr/ZamaniParser.g4`
//! - `grammar/specification/semantic-model.md`
//!
//! OWNS
//!
//! - Canonical JSON traversal and rendering.
//! - Object-key ordering and compact JSON punctuation.
//! - Optional depth, node-count, and output-byte budgets.
//! - Structured errors for canonical rendering.
//!
//! DOES NOT OWN
//!
//! - ZUIR semantic or structural verification.
//! - Serialization schema definitions or version negotiation.
//! - Document envelopes, which belong to `encode.rs`.
//! - JSON input parsing, which belongs to `decode.rs`.
//! - Duplicate-key detection in raw JSON input.
//! - Cryptographic hashing, fingerprints, or signatures.
//! - Domain-specific operation meaning or physical resource allocation.
//!
//! CANONICALIZATION CONTRACT
//!
//! - Object keys are sorted lexicographically by Rust string ordering.
//! - Array elements retain their original order.
//! - Strings use JSON escaping; non-ASCII Unicode is emitted as UTF-8.
//! - Numbers use `serde_json::Number`'s textual representation.
//! - No whitespace is emitted outside strings.
//! - This is a versioned Zamani serialization contract, not a claim of
//!   conformance to every external canonical-JSON standard.
//!
//! SCALABILITY
//!
//! Traversal uses an explicit work stack rather than recursive descent.
//! No universal IR-size, hardware-count, qubit-count, register-width, tensor
//! rank, or device-count limit is imposed. Optional budgets protect callers
//! handling untrusted or unusually large values.
//!
//! All counters use checked arithmetic. Resource exhaustion is reported;
//! it is never treated as proof that the represented program is invalid.
//! No finite process can promise infinite memory, time, or output capacity.
//!
//! INTEGRATION
//!
//! - `serialization/mod.rs` should declare `pub mod canonical;`.
//! - `encode.rs` may call `write_canonical_json` to share this canonical
//!   rendering contract, provided its envelope and output-budget semantics
//!   remain compatible.
//! - `decode.rs` remains responsible for parsing and validating envelopes.
//! - `validation.rs` and the IR verifier remain responsible for validating
//!   payload structure and meaning.
//! - `fingerprint/` may hash the exact bytes produced by this module.
//!
//! FREEZE CRITERIA
//!
//! The file is ready to freeze only after it is compiled in the actual crate,
//! exported through the serialization facade, tested against the encoder's
//! wire contract, and covered by determinism, boundary, budget, and writer
//! failure tests.

use core::fmt;
use std::io::{self, Write};

use serde_json::Value;

/// Options controlling one canonical JSON rendering operation.
///
/// `None` means this module does not impose an additional budget for that
/// dimension. It does not mean that the process has unlimited resources.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CanonicalOptions {
    /// Maximum permitted nesting depth. The root value has depth zero.
    pub max_depth: Option<usize>,

    /// Maximum number of JSON values visited, including the root.
    ///
    /// Object keys are not counted as values.
    pub max_nodes: Option<usize>,

    /// Maximum number of bytes written, including JSON punctuation.
    pub max_output_bytes: Option<usize>,
}

impl CanonicalOptions {
    /// Creates options with no additional budgets.
    ///
    /// Use explicit budgets when rendering values from untrusted input.
    #[must_use]
    pub const fn unbounded() -> Self {
        Self {
            max_depth: None,
            max_nodes: None,
            max_output_bytes: None,
        }
    }

    /// Sets the maximum nesting depth.
    #[must_use]
    pub const fn with_max_depth(mut self, limit: Option<usize>) -> Self {
        self.max_depth = limit;
        self
    }

    /// Sets the maximum number of JSON values visited.
    #[must_use]
    pub const fn with_max_nodes(mut self, limit: Option<usize>) -> Self {
        self.max_nodes = limit;
        self
    }

    /// Sets the maximum number of output bytes.
    #[must_use]
    pub const fn with_max_output_bytes(
        mut self,
        limit: Option<usize>,
    ) -> Self {
        self.max_output_bytes = limit;
        self
    }
}

/// Statistics collected during successful canonical rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CanonicalReport {
    /// Number of JSON values rendered, including the root.
    pub nodes_visited: usize,

    /// Greatest depth encountered. The root has depth zero.
    pub maximum_depth: usize,

    /// Number of object members plus array elements encountered.
    pub collection_entries: usize,

    /// Number of output bytes written.
    pub output_bytes: usize,
}

/// Errors produced by canonical JSON rendering.
#[derive(Debug)]
#[non_exhaustive]
pub enum CanonicalError {
    /// A value exceeded the configured nesting-depth budget.
    DepthLimitExceeded {
        /// Maximum permitted depth.
        limit: usize,
        /// Depth of the value that exceeded the limit.
        actual: usize,
    },

    /// Rendering exceeded the configured node-count budget.
    NodeLimitExceeded {
        /// Maximum permitted node count.
        limit: usize,
    },

    /// A counter could not represent its next value.
    CounterOverflow {
        /// Name of the counter that overflowed.
        counter: &'static str,
    },

    /// Rendering exceeded the configured output-byte budget.
    OutputLimitExceeded {
        /// Maximum permitted output size.
        limit: usize,
    },

    /// The destination writer failed.
    Io(io::Error),

    /// A JSON number could not be rendered.
    NumberFormatting(serde_json::Error),
}

impl fmt::Display for CanonicalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DepthLimitExceeded { limit, actual } => write!(
                f,
                "canonical JSON depth {actual} exceeds configured limit {limit}"
            ),
            Self::NodeLimitExceeded { limit } => write!(
                f,
                "canonical JSON node count exceeds configured limit {limit}"
            ),
            Self::CounterOverflow { counter } => {
                write!(f, "canonical JSON counter overflow: {counter}")
            }
            Self::OutputLimitExceeded { limit } => write!(
                f,
                "canonical JSON output exceeds configured limit of {limit} bytes"
            ),
            Self::Io(error) => {
                write!(f, "writing canonical JSON failed: {error}")
            }
            Self::NumberFormatting(error) => {
                write!(f, "formatting a JSON number failed: {error}")
            }
        }
    }
}

impl std::error::Error for CanonicalError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::NumberFormatting(error) => Some(error),
            _ => None,
        }
    }
}

/// A pending rendering action.
///
/// Keeping actions on an explicit stack avoids recursive traversal of deeply
/// nested JSON arrays and objects.
enum Action<'a> {
    Value {
        value: &'a Value,
        depth: usize,
    },
    String(&'a str),
    Bytes(&'static [u8]),
}

/// Writes bytes while enforcing the caller's optional output budget.
fn write_bytes<W: Write>(
    writer: &mut W,
    bytes: &[u8],
    written: &mut usize,
    limit: Option<usize>,
) -> Result<(), CanonicalError> {
    let next = written
        .checked_add(bytes.len())
        .ok_or(CanonicalError::CounterOverflow {
            counter: "output_bytes",
        })?;

    if let Some(limit) = limit {
        if next > limit {
            return Err(CanonicalError::OutputLimitExceeded { limit });
        }
    }

    writer.write_all(bytes).map_err(CanonicalError::Io)?;
    *written = next;
    Ok(())
}

/// Writes a JSON string with deterministic escaping and without allocating
/// a second string proportional to its escaped length.
fn write_json_string<W: Write>(
    writer: &mut W,
    value: &str,
    written: &mut usize,
    limit: Option<usize>,
) -> Result<(), CanonicalError> {
    write_bytes(writer, b"\"", written, limit)?;

    for character in value.chars() {
        match character {
            '"' => write_bytes(writer, b"\\\"", written, limit)?,
            '\\' => write_bytes(writer, b"\\\\", written, limit)?,
            '\u{08}' => write_bytes(writer, b"\\b", written, limit)?,
            '\u{0C}' => write_bytes(writer, b"\\f", written, limit)?,
            '\n' => write_bytes(writer, b"\\n", written, limit)?,
            '\r' => write_bytes(writer, b"\\r", written, limit)?,
            '\t' => write_bytes(writer, b"\\t", written, limit)?,
            character if character <= '\u{1F}' => {
                let code = character as u32;
                let hex = b"0123456789abcdef";
                let escaped = [
                    b'\\',
                    b'u',
                    b'0',
                    b'0',
                    hex[((code >> 4) & 0x0F) as usize],
                    hex[(code & 0x0F) as usize],
                ];
                write_bytes(writer, &escaped, written, limit)?;
            }
            character => {
                let mut buffer = [0_u8; 4];
                let encoded = character.encode_utf8(&mut buffer);
                write_bytes(writer, encoded.as_bytes(), written, limit)?;
            }
        }
    }

    write_bytes(writer, b"\"", written, limit)
}

/// Writes a JSON value in the canonical format.
///
/// Object keys are sorted; array order is preserved. The output is compact,
/// valid JSON and deterministic for a given `Value` and options.
///
/// A failure may occur after some bytes have already reached `writer`.
/// Callers must discard partial output when this function returns an error.
///
/// This function canonicalizes representation only. It does not validate
/// whether a payload is a valid ZUIR program or whether an extension is safe.
pub fn write_canonical_json<W: Write>(
    value: &Value,
    writer: &mut W,
    options: CanonicalOptions,
) -> Result<CanonicalReport, CanonicalError> {
    let mut stack = Vec::new();
    stack.push(Action::Value { value, depth: 0 });

    let mut report = CanonicalReport::default();

    while let Some(action) = stack.pop() {
        match action {
            Action::Bytes(bytes) => {
                write_bytes(
                    writer,
                    bytes,
                    &mut report.output_bytes,
                    options.max_output_bytes,
                )?;
            }

            Action::String(value) => {
                write_json_string(
                    writer,
                    value,
                    &mut report.output_bytes,
                    options.max_output_bytes,
                )?;
            }

            Action::Value { value, depth } => {
                if let Some(limit) = options.max_depth {
                    if depth > limit {
                        return Err(CanonicalError::DepthLimitExceeded {
                            limit,
                            actual: depth,
                        });
                    }
                }

                let next_nodes = report.nodes_visited.checked_add(1).ok_or(
                    CanonicalError::CounterOverflow {
                        counter: "nodes_visited",
                    },
                )?;

                if let Some(limit) = options.max_nodes {
                    if next_nodes > limit {
                        return Err(CanonicalError::NodeLimitExceeded { limit });
                    }
                }

                report.nodes_visited = next_nodes;
                report.maximum_depth = report.maximum_depth.max(depth);

                match value {
                    Value::Null => {
                        write_bytes(
                            writer,
                            b"null",
                            &mut report.output_bytes,
                            options.max_output_bytes,
                        )?;
                    }

                    Value::Bool(false) => {
                        write_bytes(
                            writer,
                            b"false",
                            &mut report.output_bytes,
                            options.max_output_bytes,
                        )?;
                    }

                    Value::Bool(true) => {
                        write_bytes(
                            writer,
                            b"true",
                            &mut report.output_bytes,
                            options.max_output_bytes,
                        )?;
                    }

                    Value::Number(number) => {
                        let rendered = number.to_string();
                        write_bytes(
                            writer,
                            rendered.as_bytes(),
                            &mut report.output_bytes,
                            options.max_output_bytes,
                        )?;
                    }

                    Value::String(string) => {
                        write_json_string(
                            writer,
                            string,
                            &mut report.output_bytes,
                            options.max_output_bytes,
                        )?;
                    }

                    Value::Array(elements) => {
                        write_bytes(
                            writer,
                            b"[",
                            &mut report.output_bytes,
                            options.max_output_bytes,
                        )?;

                        stack.push(Action::Bytes(b"]"));

                        let next_depth = depth.checked_add(1).ok_or(
                            CanonicalError::CounterOverflow {
                                counter: "depth",
                            },
                        )?;

                        report.collection_entries = report
                            .collection_entries
                            .checked_add(elements.len())
                            .ok_or(CanonicalError::CounterOverflow {
                                counter: "collection_entries",
                            })?;

                        for index in (0..elements.len()).rev() {
                            stack.push(Action::Value {
                                value: &elements[index],
                                depth: next_depth,
                            });

                            if index > 0 {
                                stack.push(Action::Bytes(b","));
                            }
                        }
                    }

                    Value::Object(object) => {
                        write_bytes(
                            writer,
                            b"{",
                            &mut report.output_bytes,
                            options.max_output_bytes,
                        )?;

                        // Sort explicitly instead of relying on the concrete
                        // map implementation or enabled serde_json features.
                        let mut entries: Vec<_> = object.iter().collect();
                        entries.sort_unstable_by(|left, right| {
                            left.0.cmp(right.0)
                        });

                        stack.push(Action::Bytes(b"}"));

                        let next_depth = depth.checked_add(1).ok_or(
                            CanonicalError::CounterOverflow {
                                counter: "depth",
                            },
                        )?;

                        report.collection_entries = report
                            .collection_entries
                            .checked_add(entries.len())
                            .ok_or(CanonicalError::CounterOverflow {
                                counter: "collection_entries",
                            })?;

                        for index in (0..entries.len()).rev() {
                            let (key, member_value) = entries[index];

                            stack.push(Action::Value {
                                value: member_value,
                                depth: next_depth,
                            });
                            stack.push(Action::Bytes(b":"));
                            stack.push(Action::String(key));

                            if index > 0 {
                                stack.push(Action::Bytes(b","));
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(report)
}

/// Returns canonical JSON bytes for a value.
///
/// This convenience function allocates the complete output. Use
/// `write_canonical_json` with a suitable writer when the destination should
/// receive output incrementally.
///
/// If `max_output_bytes` is configured, the limit applies to the complete
/// output. A failure discards the internally accumulated buffer.
pub fn canonical_json_bytes(
    value: &Value,
    options: CanonicalOptions,
) -> Result<Vec<u8>, CanonicalError> {
    let mut output = Vec::new();
    write_canonical_json(value, &mut output, options)?;
    Ok(output)
}

/// Returns canonical JSON as a UTF-8 `String`.
///
/// JSON output is valid UTF-8 by construction. This conversion remains
/// fallible so the API does not rely on unchecked assumptions.
pub fn canonical_json_string(
    value: &Value,
    options: CanonicalOptions,
) -> Result<String, CanonicalError> {
    let bytes = canonical_json_bytes(value, options)?;
    String::from_utf8(bytes).map_err(|error| {
        CanonicalError::Io(io::Error::new(
            io::ErrorKind::InvalidData,
            error,
        ))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn value(source: &str) -> Value {
        serde_json::from_str(source).expect("test JSON must be valid")
    }

    #[test]
    fn sorts_object_keys_recursively() {
        let input = value(
            r#"{"z":1,"nested":{"y":2,"a":3},"a":4}"#,
        );

        let rendered = canonical_json_string(
            &input,
            CanonicalOptions::default(),
        )
        .expect("canonical rendering should succeed");

        assert_eq!(
            rendered,
            r#"{"a":4,"nested":{"a":3,"y":2},"z":1}"#
        );
    }

    #[test]
    fn preserves_array_order() {
        let input = value(r#"{"items":[3,1,2]}"#);

        let rendered = canonical_json_string(
            &input,
            CanonicalOptions::default(),
        )
        .expect("canonical rendering should succeed");

        assert_eq!(rendered, r#"{"items":[3,1,2]}"#);
    }

    #[test]
    fn escapes_strings_and_preserves_unicode() {
        let input = Value::String("line\nquote\" slash\\ λ".to_owned());

        let rendered = canonical_json_string(
            &input,
            CanonicalOptions::default(),
        )
        .expect("canonical rendering should succeed");

        assert_eq!(rendered, r#""line\nquote\" slash\\ λ""#);
        assert_eq!(
            serde_json::from_str::<Value>(&rendered)
                .expect("rendered JSON should parse"),
            input
        );
    }

    #[test]
    fn produces_no_unnecessary_whitespace() {
        let input = value(r#"{"a":[true,null,{"b":"c"}]}"#);

        let rendered = canonical_json_string(
            &input,
            CanonicalOptions::default(),
        )
        .expect("canonical rendering should succeed");

        assert_eq!(rendered, r#"{"a":[true,null,{"b":"c"}]}"#);
    }

    #[test]
    fn reports_node_depth_and_output_budgets() {
        let input = value(r#"{"a":[1]}"#);

        assert!(matches!(
            canonical_json_bytes(
                &input,
                CanonicalOptions::default().with_max_nodes(Some(2)),
            ),
            Err(CanonicalError::NodeLimitExceeded { limit: 2 })
        ));

        assert!(matches!(
            canonical_json_bytes(
                &input,
                CanonicalOptions::default().with_max_depth(Some(1)),
            ),
            Err(CanonicalError::DepthLimitExceeded {
                limit: 1,
                actual: 2
            })
        ));

        assert!(matches!(
            canonical_json_bytes(
                &input,
                CanonicalOptions::default()
                    .with_max_output_bytes(Some(3)),
            ),
            Err(CanonicalError::OutputLimitExceeded { limit: 3 })
        ));
    }

    #[test]
    fn reports_rendering_statistics() {
        let input = value(r#"{"a":[1,2]}"#);
        let mut output = Vec::new();

        let report = write_canonical_json(
            &input,
            &mut output,
            CanonicalOptions::default(),
        )
        .expect("canonical rendering should succeed");

        assert_eq!(report.nodes_visited, 4);
        assert_eq!(report.maximum_depth, 2);
        assert_eq!(report.collection_entries, 3);
        assert_eq!(report.output_bytes, output.len());
    }

    #[test]
    fn handles_root_scalars() {
        let cases = [
            (Value::Null, "null"),
            (Value::Bool(true), "true"),
            (Value::Bool(false), "false"),
            (Value::String("x".to_owned()), r#""x""#),
        ];

        for (input, expected) in cases {
            assert_eq!(
                canonical_json_string(
                    &input,
                    CanonicalOptions::default(),
                )
                .expect("scalar rendering should succeed"),
                expected
            );
        }
    }

    #[test]
    fn empty_collections_are_canonical() {
        assert_eq!(
            canonical_json_string(
                &Value::Array(Vec::new()),
                CanonicalOptions::default(),
            )
            .expect("empty array should render"),
            "[]"
        );

        assert_eq!(
            canonical_json_string(
                &Value::Object(serde_json::Map::new()),
                CanonicalOptions::default(),
            )
            .expect("empty object should render"),
            "{}"
        );
    }

    #[test]
    fn output_budget_counts_the_complete_document() {
        let input = Value::String("a".to_owned());
        let options = CanonicalOptions::default()
            .with_max_output_bytes(Some(3));

        assert_eq!(
            canonical_json_bytes(&input, options)
                .expect("three bytes should fit"),
            br#""a""#
        );
    }

    #[test]
    fn repeated_rendering_is_deterministic() {
        let input = value(
            r#"{"b":{"z":9,"a":1},"a":[4,3,2,1]}"#,
        );

        let first = canonical_json_bytes(
            &input,
            CanonicalOptions::default(),
        )
        .expect("first rendering should succeed");

        let second = canonical_json_bytes(
            &input,
            CanonicalOptions::default(),
        )
        .expect("second rendering should succeed");

        assert_eq!(first, second);
    }
}
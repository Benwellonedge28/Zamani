
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Serialization Validation
//!
//! Canonical path: `src/zuir/serialization/validation.rs`
//! Rust edition: 2021
//! Minimum supported Rust version: 1.97
//! Safety: unsafe Rust is forbidden.
//!
//! ## PURPOSE
//!
//! Validate schema descriptors and JSON payloads at the serialization
//! boundary before downstream code interprets them.
//!
//! This module validates representation shape, declared field contracts,
//! unknown-field policy, and configurable structural resource budgets.
//! It does not establish the semantic validity of a complete ZUIR program.
//!
//! ## AUTHORITY
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/version.rs`
//! - `src/zuir/serialization/schema.rs`
//! - `src/zuir/serialization/encode.rs`
//! - `src/zuir/serialization/decode.rs`
//! - `src/zuir/ir/program.rs`
//! - `src/zuir/ir/operation.rs`
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/antlr/ZamaniLexer.g4`
//! - `grammar/antlr/ZamaniParser.g4`
//! - `grammar/specification/semantic-model.md`
//!
//! The grammar owns syntax. The semantic model owns source meaning.
//! The ZUIR specification owns IR meaning. The schema module owns wire
//! schema definitions. The canonical IR verifier owns whole-program
//! structural and semantic verification.
//!
//! ## OWNS
//!
//! - Validation options and validation reports.
//! - Schema descriptor validation at this boundary.
//! - JSON payload shape checks against a schema descriptor.
//! - Required, optional, and repeated field checks.
//! - Wire-kind checks for supported JSON representations.
//! - Unknown-field policy enforcement.
//! - Iterative structural-budget accounting.
//!
//! ## DOES_NOT_OWN
//!
//! - JSON encoding or decoding.
//! - Schema definition or schema migration.
//! - AST construction, grammar interpretation, or name resolution.
//! - Whole-program IR verification or type inference.
//! - Dialect registration, extension authorization, or code execution.
//! - Hardware feasibility, resource allocation, or target selection.
//! - Cryptographic integrity, signatures, or content fingerprints.
//!
//! ## TRUST BOUNDARY
//!
//! A successful validation result means only that the supplied value passed
//! the checks implemented here against the supplied schema and budget.
//! It does not prove that the payload is a valid ZUIR program.
//!
//! Callers must separately run canonical IR verification before compilation,
//! transformation, or execution.
//!
//! ## SCALABILITY
//!
//! There are no hard-coded limits on qubits, operations, functions, devices,
//! tensor rank, hardware size, or program size.
//!
//! All operational limits are supplied through `ValidationLimits`.
//! `ValidationLimits::unbounded()` disables additional limits imposed by
//! this module; it cannot create unlimited memory, time, or address space.
//!
//! Traversal is iterative rather than recursively descending through JSON.
//! Counters use checked arithmetic and fail explicitly on overflow.
//!
//! ## IMPORTANT JSON LIMITATION
//!
//! A `serde_json::Value` has already been parsed. Depending on the parser
//! configuration and input, duplicate JSON object keys may already have
//! been collapsed. This module cannot recover such keys after that point.
//! If duplicate-key rejection is required, it must be enforced by the
//! decoder while consuming the original JSON input.
//!
//! ## INTEGRATION
//!
//! `serialization/mod.rs` must declare `pub mod validation;`.
//! `serialization/schema.rs` remains the authoritative schema API.
//! `serialization/decode.rs` may pass a decoded payload to this module.
//! `verify/` remains responsible for canonical IR verification.
//! `serialization/migration.rs`, if added, owns actual migrations.
//!
//! ## FREEZE CRITERIA
//!
//! - The real crate compiles with the actual module facade.
//! - Schema and payload validation tests pass.
//! - All resource budgets are checked deterministically.
//! - Unsupported wire representations fail explicitly.
//! - Decoder, encoder, schema, and IR-verifier integration tests pass.
//! - No unsafe code or machine-specific size assumptions are introduced.

use core::fmt;
use std::error::Error;

use serde_json::{Map, Value};

use crate::zuir::serialization::schema::{
    CompatibilityPolicy, FieldCardinality, SchemaDescriptor, SchemaError,
    SchemaId, SchemaKind, SchemaValidationLimits, UnknownFieldPolicy, WireKind,
};

/// Configurable operational budgets for schema and payload validation.
///
/// These limits protect a process from excessive structural work. They do
/// not impose semantic limits on valid Zamani programs.
///
/// The default is unbounded at this validation layer. Applications handling
/// untrusted data should choose explicit finite limits appropriate to their
/// deployment and available resources.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ValidationLimits {
    /// Maximum number of fields in a schema descriptor.
    pub max_schema_fields: usize,

    /// Maximum JSON nesting depth. The root value has depth zero.
    pub max_depth: usize,

    /// Maximum total number of JSON values, including the root.
    pub max_nodes: usize,

    /// Maximum aggregate number of entries in arrays and objects.
    ///
    /// Object keys count as entries but not as separate JSON value nodes.
    pub max_collection_entries: usize,

    /// Maximum UTF-8 byte length of an individual string or object key.
    pub max_string_bytes: usize,

    /// Maximum aggregate UTF-8 bytes across all strings and object keys.
    pub max_total_string_bytes: usize,
}

impl ValidationLimits {
    /// Constructs explicit validation budgets.
    #[must_use]
    pub const fn new(
        max_schema_fields: usize,
        max_depth: usize,
        max_nodes: usize,
        max_collection_entries: usize,
        max_string_bytes: usize,
        max_total_string_bytes: usize,
    ) -> Self {
        Self {
            max_schema_fields,
            max_depth,
            max_nodes,
            max_collection_entries,
            max_string_bytes,
            max_total_string_bytes,
        }
    }

    /// Disables additional limits imposed by this module.
    ///
    /// This is not a guarantee of infinite memory or execution time.
    #[must_use]
    pub const fn unbounded() -> Self {
        Self {
            max_schema_fields: usize::MAX,
            max_depth: usize::MAX,
            max_nodes: usize::MAX,
            max_collection_entries: usize::MAX,
            max_string_bytes: usize::MAX,
            max_total_string_bytes: usize::MAX,
        }
    }

    /// Returns a copy with a different maximum nesting depth.
    #[must_use]
    pub const fn with_max_depth(mut self, limit: usize) -> Self {
        self.max_depth = limit;
        self
    }

    /// Returns a copy with a different maximum JSON node count.
    #[must_use]
    pub const fn with_max_nodes(mut self, limit: usize) -> Self {
        self.max_nodes = limit;
        self
    }

    /// Returns a copy with a different aggregate collection-entry budget.
    #[must_use]
    pub const fn with_max_collection_entries(
        mut self,
        limit: usize,
    ) -> Self {
        self.max_collection_entries = limit;
        self
    }

    /// Returns a copy with a different per-string byte budget.
    #[must_use]
    pub const fn with_max_string_bytes(mut self, limit: usize) -> Self {
        self.max_string_bytes = limit;
        self
    }

    /// Returns a copy with a different aggregate string-byte budget.
    #[must_use]
    pub const fn with_max_total_string_bytes(
        mut self,
        limit: usize,
    ) -> Self {
        self.max_total_string_bytes = limit;
        self
    }
}

impl Default for ValidationLimits {
    fn default() -> Self {
        Self::unbounded()
    }
}

/// Successful validation accounting.
///
/// The counters describe the inspected payload. They are not estimates of
/// total process memory, CPU cost, or the size of a future IR representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ValidationReport {
    /// Number of JSON values inspected, including the root.
    pub nodes_visited: usize,

    /// Aggregate array elements and object entries inspected.
    pub collection_entries: usize,

    /// Aggregate UTF-8 bytes in string values and object keys.
    pub string_bytes: usize,

    /// Maximum nesting depth encountered.
    pub maximum_depth: usize,

    /// Number of fields accepted as unknown under the schema policy.
    pub unknown_fields: usize,
}

/// A JSON location used in deterministic validation errors.
///
/// Object keys are represented as strings. Array indices are represented as
/// decimal strings. This avoids imposing a separate fixed path-depth limit.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ValuePath {
    segments: Vec<String>,
}

impl ValuePath {
    /// Creates a path referring to the root value.
    #[must_use]
    pub fn root() -> Self {
        Self::default()
    }

    /// Returns the path segments from the root.
    #[must_use]
    pub fn segments(&self) -> &[String] {
        &self.segments
    }

    fn child_key(&self, key: &str) -> Self {
        let mut next = self.clone();
        next.segments.push(key.to_owned());
        next
    }

    fn child_index(&self, index: usize) -> Self {
        let mut next = self.clone();
        next.segments.push(index.to_string());
        next
    }
}

impl fmt::Display for ValuePath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.segments.is_empty() {
            return f.write_str("$");
        }

        f.write_str("$")?;

        for segment in &self.segments {
            write!(f, "[{segment:?}]")?;
        }

        Ok(())
    }
}

/// Errors returned by schema or payload validation.
#[derive(Debug)]
#[non_exhaustive]
pub enum ValidationError {
    /// The schema descriptor itself is invalid.
    InvalidSchema(SchemaError),

    /// The payload root has the wrong JSON category for the schema kind.
    InvalidRootKind {
        expected: &'static str,
        actual: &'static str,
    },

    /// A required field is absent.
    MissingRequiredField {
        field: String,
    },

    /// A field's representation does not match its declared wire kind.
    WrongWireKind {
        path: ValuePath,
        expected: WireKind,
        actual: &'static str,
    },

    /// A repeated field must be represented as an array.
    RepeatedFieldMustBeArray {
        path: ValuePath,
    },

    /// A field not declared by the schema was encountered.
    UnknownField {
        path: ValuePath,
    },

    /// A schema field declared with `WireKind::SchemaId` contains an
    /// invalid schema identifier.
    InvalidSchemaId {
        path: ValuePath,
        reason: String,
    },

    /// A structural counter exceeded its configured budget.
    BudgetExceeded {
        resource: &'static str,
        limit: usize,
        path: ValuePath,
    },

    /// A counter overflowed while accounting for input structure.
    CounterOverflow {
        resource: &'static str,
        path: ValuePath,
    },

    /// A schema compatibility policy requires an explicit migration or
    /// negotiation rather than direct validation.
    CompatibilityRequiresMigration {
        reader: String,
        writer: String,
    },

    /// The selected policy requires external compatibility negotiation.
    CompatibilityNegotiationRequired {
        reader: String,
        writer: String,
    },
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidSchema(error) => {
                write!(f, "invalid ZUIR serialization schema: {error}")
            }
            Self::InvalidRootKind { expected, actual } => {
                write!(
                    f,
                    "invalid payload root: expected {expected}, found {actual}"
                )
            }
            Self::MissingRequiredField { field } => {
                write!(f, "missing required schema field {field:?}")
            }
            Self::WrongWireKind {
                path,
                expected,
                actual,
            } => {
                write!(
                    f,
                    "value at {path} has JSON kind {actual}; \
                     schema requires {expected:?}"
                )
            }
            Self::RepeatedFieldMustBeArray { path } => {
                write!(
                    f,
                    "repeated schema field at {path} must be represented as an array"
                )
            }
            Self::UnknownField { path } => {
                write!(f, "unknown field at {path}")
            }
            Self::InvalidSchemaId { path, reason } => {
                write!(
                    f,
                    "invalid schema identifier at {path}: {reason}"
                )
            }
            Self::BudgetExceeded {
                resource,
                limit,
                path,
            } => {
                write!(
                    f,
                    "validation {resource} budget of {limit} exceeded at {path}"
                )
            }
            Self::CounterOverflow { resource, path } => {
                write!(
                    f,
                    "validation counter overflow for {resource} at {path}"
                )
            }
            Self::CompatibilityRequiresMigration { reader, writer } => {
                write!(
                    f,
                    "schema {writer} requires migration before validation \
                     by schema {reader}"
                )
            }
            Self::CompatibilityNegotiationRequired { reader, writer } => {
                write!(
                    f,
                    "schema compatibility negotiation required between \
                     reader {reader} and writer {writer}"
                )
            }
        }
    }
}

impl Error for ValidationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidSchema(error) => Some(error),
            _ => None,
        }
    }
}

/// Validates the structure of a schema descriptor using explicit limits.
pub fn validate_schema(
    schema: &SchemaDescriptor,
    limits: &ValidationLimits,
) -> Result<(), ValidationError> {
    schema
        .validate_with_limits(&SchemaValidationLimits::new(
            limits.max_schema_fields,
        ))
        .map_err(ValidationError::InvalidSchema)
}

/// Checks whether a reader schema's compatibility policy accepts a writer
/// schema version without an implicit migration.
pub fn validate_schema_compatibility(
    reader: &SchemaDescriptor,
    writer_version: crate::zuir::serialization::schema::SchemaVersion,
) -> Result<(), ValidationError> {
    match reader.check_version(writer_version) {
        Ok(()) => Ok(()),
        Err(SchemaError::MigrationRequired { reader, writer }) => {
            Err(ValidationError::CompatibilityRequiresMigration {
                reader: reader.to_string(),
                writer: writer.to_string(),
            })
        }
        Err(SchemaError::NegotiationRequired { reader, writer }) => {
            Err(ValidationError::CompatibilityNegotiationRequired {
                reader: reader.to_string(),
                writer: writer.to_string(),
            })
        }
        Err(error) => Err(ValidationError::InvalidSchema(error)),
    }
}

/// Validates a JSON payload against a schema descriptor.
///
/// The descriptor must already define the correct contract for the payload.
/// This function does not discover or authorize schemas from untrusted input.
///
/// `WireKind` describes the expected JSON representation, not the full
/// semantic type of a ZUIR value.
pub fn validate_payload(
    payload: &Value,
    schema: &SchemaDescriptor,
    limits: &ValidationLimits,
) -> Result<ValidationReport, ValidationError> {
    validate_schema(schema, limits)?;
    validate_root_kind(payload, schema.kind())?;
    validate_declared_fields(payload, schema)?;
    validate_json_budget(payload, limits)
}

/// Validates a payload with the default unbounded validation-layer budgets.
///
/// Use `validate_payload` with explicit limits when handling untrusted input.
pub fn validate_payload_unbounded(
    payload: &Value,
    schema: &SchemaDescriptor,
) -> Result<ValidationReport, ValidationError> {
    validate_payload(payload, schema, &ValidationLimits::unbounded())
}

fn validate_root_kind(
    payload: &Value,
    kind: SchemaKind,
) -> Result<(), ValidationError> {
    match kind {
        SchemaKind::Document | SchemaKind::Payload | SchemaKind::Object => {
            if !payload.is_object() {
                return Err(ValidationError::InvalidRootKind {
                    expected: "object",
                    actual: json_kind(payload),
                });
            }
        }
        SchemaKind::Dialect | SchemaKind::Extension => {
            // Dialect and extension schemas may define an object or another
            // representation. Their declared field contracts still apply
            // when the root is an object.
        }
    }

    Ok(())
}

fn validate_declared_fields(
    payload: &Value,
    schema: &SchemaDescriptor,
) -> Result<(), ValidationError> {
    let Some(object) = payload.as_object() else {
        // Non-object roots are permitted only for schema kinds whose
        // contract does not require an object root.
        return Ok(());
    };

    for field in schema.fields() {
        let path = ValuePath::root().child_key(&field.name);

        match object.get(&field.name) {
            None if field.cardinality == FieldCardinality::Required => {
                return Err(ValidationError::MissingRequiredField {
                    field: field.name.clone(),
                });
            }
            None => continue,
            Some(value) if field.cardinality == FieldCardinality::Repeated => {
                let Some(values) = value.as_array() else {
                    return Err(ValidationError::RepeatedFieldMustBeArray {
                        path,
                    });
                };

                for (index, element) in values.iter().enumerate() {
                    validate_wire_kind(
                        element,
                        field.wire_kind,
                        path.child_index(index),
                    )?;
                }
            }
            Some(value) => {
                validate_wire_kind(value, field.wire_kind, path)?;
            }
        }
    }

    for key in object.keys() {
        if schema.field_by_name(key).is_some() {
            continue;
        }

        let path = ValuePath::root().child_key(key);

        match schema.unknown_field_policy() {
            UnknownFieldPolicy::Reject => {
                return Err(ValidationError::UnknownField { path });
            }
            UnknownFieldPolicy::Skip | UnknownFieldPolicy::Preserve => {
                // The original payload is retained by the caller. This
                // validator does not remove or rewrite unknown data.
            }
        }
    }

    Ok(())
}

fn validate_wire_kind(
    value: &Value,
    expected: WireKind,
    path: ValuePath,
) -> Result<(), ValidationError> {
    let matches = match expected {
        WireKind::Unit => value.is_null(),
        WireKind::Bool => value.is_boolean(),

        WireKind::U8 => value
            .as_u64()
            .is_some_and(|number| u8::try_from(number).is_ok()),
        WireKind::U16 => value
            .as_u64()
            .is_some_and(|number| u16::try_from(number).is_ok()),
        WireKind::U32 => value
            .as_u64()
            .is_some_and(|number| u32::try_from(number).is_ok()),
        WireKind::U64 => value.as_u64().is_some(),

        WireKind::I8 => value
            .as_i64()
            .is_some_and(|number| i8::try_from(number).is_ok()),
        WireKind::I16 => value
            .as_i64()
            .is_some_and(|number| i16::try_from(number).is_ok()),
        WireKind::I32 => value
            .as_i64()
            .is_some_and(|number| i32::try_from(number).is_ok()),
        WireKind::I64 => value.as_i64().is_some(),

        WireKind::F32 => value.as_f64().is_some_and(|number| {
            number.is_finite()
                && number >= f32::MIN as f64
                && number <= f32::MAX as f64
        }),
        WireKind::F64 => value
            .as_f64()
            .is_some_and(|number| number.is_finite()),

        WireKind::String => value.is_string(),

        // JSON has no native byte-string type. This contract uses an array
        // of unsigned byte values as the portable JSON representation.
        WireKind::Bytes => value.as_array().is_some_and(|bytes| {
            bytes.iter().all(|byte| {
                byte.as_u64()
                    .is_some_and(|number| u8::try_from(number).is_ok())
            })
        }),

        WireKind::Sequence => value.is_array(),
        WireKind::Map | WireKind::Object => value.is_object(),

        WireKind::SchemaId => match value.as_str() {
            Some(text) => SchemaId::new(text.to_owned()).is_ok(),
            None => false,
        },

        // Identity encodings are format-specific. Accept JSON integers or
        // strings here; the concrete typed-ID deserializer must validate
        // the actual representation and range.
        WireKind::Identity => {
            value.as_u64().is_some()
                || value.as_i64().is_some()
                || value.as_str().is_some()
        }

        // Extension and expression payloads are opaque at this layer.
        // Their owning dialect or semantic verifier must interpret them.
        WireKind::Extension | WireKind::Expression => true,
    };

    if matches {
        Ok(())
    } else if expected == WireKind::SchemaId {
        let reason = match value.as_str() {
            Some(text) => SchemaId::new(text.to_owned())
                .err()
                .map(|error| error.to_string())
                .unwrap_or_else(|| "invalid schema identifier".to_owned()),
            None => "expected a string".to_owned(),
        };

        Err(ValidationError::InvalidSchemaId { path, reason })
    } else {
        Err(ValidationError::WrongWireKind {
            path,
            expected,
            actual: json_kind(value),
        })
    }
}

fn json_kind(value: &Value) -> &'static str {
    match value {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}

struct PendingValue<'a> {
    value: &'a Value,
    path: ValuePath,
    depth: usize,
}

/// Iteratively checks payload complexity against configured budgets.
///
/// This traversal does not recurse through nested JSON, avoiding a
/// validation call-stack proportional to attacker-controlled nesting depth.
fn validate_json_budget(
    root: &Value,
    limits: &ValidationLimits,
) -> Result<ValidationReport, ValidationError> {
    let mut report = ValidationReport::default();
    let mut pending = vec![PendingValue {
        value: root,
        path: ValuePath::root(),
        depth: 0,
    }];

    while let Some(current) = pending.pop() {
        if current.depth > limits.max_depth {
            return Err(ValidationError::BudgetExceeded {
                resource: "nesting-depth",
                limit: limits.max_depth,
                path: current.path,
            });
        }

        report.nodes_visited = checked_increment(
            report.nodes_visited,
            "node-count",
            limits.max_nodes,
            &current.path,
        )?;

        report.maximum_depth = report.maximum_depth.max(current.depth);

        match current.value {
            Value::Null | Value::Bool(_) | Value::Number(_) => {}
            Value::String(value) => {
                add_string_bytes(value, &current.path, limits, &mut report)?;
            }
            Value::Array(values) => {
                add_collection_entries(
                    values.len(),
                    &current.path,
                    limits,
                    &mut report,
                )?;

                for (index, value) in values.iter().enumerate().rev() {
                    pending.push(PendingValue {
                        value,
                        path: current.path.child_index(index),
                        depth: current.depth.checked_add(1).ok_or_else(|| {
                            ValidationError::CounterOverflow {
                                resource: "nesting-depth",
                                path: current.path.clone(),
                            }
                        })?,
                    });
                }
            }
            Value::Object(values) => {
                add_collection_entries(
                    values.len(),
                    &current.path,
                    limits,
                    &mut report,
                )?;

                for (key, value) in values.iter().rev() {
                    let child_path = current.path.child_key(key);
                    add_string_bytes(
                        key,
                        &child_path,
                        limits,
                        &mut report,
                    )?;

                    pending.push(PendingValue {
                        value,
                        path: child_path,
                        depth: current.depth.checked_add(1).ok_or_else(|| {
                            ValidationError::CounterOverflow {
                                resource: "nesting-depth",
                                path: current.path.clone(),
                            }
                        })?,
                    });
                }
            }
        }
    }

    Ok(report)
}

fn checked_increment(
    current: usize,
    resource: &'static str,
    limit: usize,
    path: &ValuePath,
) -> Result<usize, ValidationError> {
    let next = current.checked_add(1).ok_or_else(|| {
        ValidationError::CounterOverflow {
            resource,
            path: path.clone(),
        }
    })?;

    if next > limit {
        return Err(ValidationError::BudgetExceeded {
            resource,
            limit,
            path: path.clone(),
        });
    }

    Ok(next)
}

fn add_collection_entries(
    count: usize,
    path: &ValuePath,
    limits: &ValidationLimits,
    report: &mut ValidationReport,
) -> Result<(), ValidationError> {
    let next = report
        .collection_entries
        .checked_add(count)
        .ok_or_else(|| ValidationError::CounterOverflow {
            resource: "collection-entry-count",
            path: path.clone(),
        })?;

    if next > limits.max_collection_entries {
        return Err(ValidationError::BudgetExceeded {
            resource: "collection-entry-count",
            limit: limits.max_collection_entries,
            path: path.clone(),
        });
    }

    report.collection_entries = next;
    Ok(())
}

fn add_string_bytes(
    value: &str,
    path: &ValuePath,
    limits: &ValidationLimits,
    report: &mut ValidationReport,
) -> Result<(), ValidationError> {
    let length = value.len();

    if length > limits.max_string_bytes {
        return Err(ValidationError::BudgetExceeded {
            resource: "individual-string-bytes",
            limit: limits.max_string_bytes,
            path: path.clone(),
        });
    }

    let next = report
        .string_bytes
        .checked_add(length)
        .ok_or_else(|| ValidationError::CounterOverflow {
            resource: "aggregate-string-bytes",
            path: path.clone(),
        })?;

    if next > limits.max_total_string_bytes {
        return Err(ValidationError::BudgetExceeded {
            resource: "aggregate-string-bytes",
            limit: limits.max_total_string_bytes,
            path: path.clone(),
        });
    }

    report.string_bytes = next;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zuir::serialization::schema::{
        FieldDescriptor, FieldId, SchemaVersion,
    };

    fn test_schema(
        fields: Vec<FieldDescriptor>,
        unknown_fields: UnknownFieldPolicy,
    ) -> SchemaDescriptor {
        SchemaDescriptor::new(
            SchemaId::new("zamani.test.payload").expect("valid schema ID"),
            SchemaKind::Payload,
            SchemaVersion::new(1, 0, 0),
            CompatibilityPolicy::Exact,
            unknown_fields,
            false,
            fields,
        )
    }

    fn field(
        id: u32,
        name: &str,
        kind: WireKind,
        cardinality: FieldCardinality,
    ) -> FieldDescriptor {
        FieldDescriptor::new(
            FieldId::new(id).expect("nonzero field ID"),
            name,
            kind,
            cardinality,
            true,
            true,
        )
        .expect("valid field descriptor")
    }

    #[test]
    fn validates_a_valid_payload() {
        let schema = test_schema(
            vec![
                field(
                    1,
                    "name",
                    WireKind::String,
                    FieldCardinality::Required,
                ),
                field(
                    2,
                    "enabled",
                    WireKind::Bool,
                    FieldCardinality::Optional,
                ),
            ],
            UnknownFieldPolicy::Reject,
        );

        let payload = serde_json::json!({
            "name": "example",
            "enabled": true
        });

        let report = validate_payload(
            &payload,
            &schema,
            &ValidationLimits::unbounded(),
        )
        .expect("payload should validate");

        assert!(report.nodes_visited >= 3);
        assert_eq!(report.unknown_fields, 0);
    }

    #[test]
    fn rejects_a_missing_required_field() {
        let schema = test_schema(
            vec![field(
                1,
                "required",
                WireKind::String,
                FieldCardinality::Required,
            )],
            UnknownFieldPolicy::Reject,
        );

        let error = validate_payload(
            &serde_json::json!({}),
            &schema,
            &ValidationLimits::unbounded(),
        )
        .expect_err("required field must be present");

        assert!(matches!(
            error,
            ValidationError::MissingRequiredField { .. }
        ));
    }

    #[test]
    fn rejects_an_unknown_field_when_policy_is_reject() {
        let schema = test_schema(vec![], UnknownFieldPolicy::Reject);

        let error = validate_payload(
            &serde_json::json!({"unexpected": 1}),
            &schema,
            &ValidationLimits::unbounded(),
        )
        .expect_err("unknown field must be rejected");

        assert!(matches!(error, ValidationError::UnknownField { .. }));
    }

    #[test]
    fn preserves_unknown_fields_when_policy_is_preserve() {
        let schema = test_schema(vec![], UnknownFieldPolicy::Preserve);
        let payload = serde_json::json!({"future_extension": {"x": 1}});

        validate_payload(
            &payload,
            &schema,
            &ValidationLimits::unbounded(),
        )
        .expect("preserved unknown field should pass structural validation");

        assert_eq!(payload["future_extension"]["x"], 1);
    }

    #[test]
    fn rejects_wrong_wire_kind() {
        let schema = test_schema(
            vec![field(
                1,
                "enabled",
                WireKind::Bool,
                FieldCardinality::Required,
            )],
            UnknownFieldPolicy::Reject,
        );

        let error = validate_payload(
            &serde_json::json!({"enabled": "yes"}),
            &schema,
            &ValidationLimits::unbounded(),
        )
        .expect_err("string is not a boolean");

        assert!(matches!(error, ValidationError::WrongWireKind { .. }));
    }

    #[test]
    fn repeated_fields_require_arrays() {
        let schema = test_schema(
            vec![field(
                1,
                "items",
                WireKind::String,
                FieldCardinality::Repeated,
            )],
            UnknownFieldPolicy::Reject,
        );

        let error = validate_payload(
            &serde_json::json!({"items": "one"}),
            &schema,
            &ValidationLimits::unbounded(),
        )
        .expect_err("repeated field must be an array");

        assert!(matches!(
            error,
            ValidationError::RepeatedFieldMustBeArray { .. }
        ));
    }

    #[test]
    fn repeated_fields_validate_each_element() {
        let schema = test_schema(
            vec![field(
                1,
                "items",
                WireKind::String,
                FieldCardinality::Repeated,
            )],
            UnknownFieldPolicy::Reject,
        );

        let error = validate_payload(
            &serde_json::json!({"items": ["valid", 2]}),
            &schema,
            &ValidationLimits::unbounded(),
        )
        .expect_err("every repeated element must match the wire kind");

        assert!(matches!(error, ValidationError::WrongWireKind { .. }));
    }

    #[test]
    fn enforces_node_budget() {
        let schema = test_schema(vec![], UnknownFieldPolicy::Preserve);
        let limits = ValidationLimits::unbounded().with_max_nodes(2);

        let error = validate_payload(
            &serde_json::json!({"a": [1, 2]}),
            &schema,
            &limits,
        )
        .expect_err("payload exceeds the node budget");

        assert!(matches!(
            error,
            ValidationError::BudgetExceeded {
                resource: "node-count",
                ..
            }
        ));
    }

    #[test]
    fn enforces_depth_budget() {
        let schema = test_schema(vec![], UnknownFieldPolicy::Preserve);
        let limits = ValidationLimits::unbounded().with_max_depth(1);

        let error = validate_payload(
            &serde_json::json!({"a": {"b": 1}}),
            &schema,
            &limits,
        )
        .expect_err("payload exceeds the depth budget");

        assert!(matches!(
            error,
            ValidationError::BudgetExceeded {
                resource: "nesting-depth",
                ..
            }
        ));
    }

    #[test]
    fn enforces_per_string_budget() {
        let schema = test_schema(vec![], UnknownFieldPolicy::Preserve);
        let limits = ValidationLimits::unbounded().with_max_string_bytes(3);

        let error = validate_payload(
            &serde_json::json!({"name": "long"}),
            &schema,
            &limits,
        )
        .expect_err("string exceeds its byte budget");

        assert!(matches!(error, ValidationError::BudgetExceeded { .. }));
    }

    #[test]
    fn enforces_collection_entry_budget() {
        let schema = test_schema(vec![], UnknownFieldPolicy::Preserve);
        let limits =
            ValidationLimits::unbounded().with_max_collection_entries(1);

        let error = validate_payload(
            &serde_json::json!({"a": 1, "b": 2}),
            &schema,
            &limits,
        )
        .expect_err("object exceeds the collection-entry budget");

        assert!(matches!(
            error,
            ValidationError::BudgetExceeded {
                resource: "collection-entry-count",
                ..
            }
        ));
    }

    #[test]
    fn validates_schema_identifier_wire_kind() {
        let schema = test_schema(
            vec![field(
                1,
                "schema",
                WireKind::SchemaId,
                FieldCardinality::Required,
            )],
            UnknownFieldPolicy::Reject,
        );

        validate_payload(
            &serde_json::json!({"schema": "zamani.example.schema"}),
            &schema,
            &ValidationLimits::unbounded(),
        )
        .expect("valid schema ID should pass");

        let error = validate_payload(
            &serde_json::json!({"schema": "bad schema"}),
            &schema,
            &ValidationLimits::unbounded(),
        )
        .expect_err("invalid schema ID should fail");

        assert!(matches!(error, ValidationError::InvalidSchemaId { .. }));
    }

    #[test]
    fn rejects_a_non_object_payload_schema_root() {
        let schema = test_schema(vec![], UnknownFieldPolicy::Reject);

        let error = validate_payload(
            &Value::Null,
            &schema,
            &ValidationLimits::unbounded(),
        )
        .expect_err("payload schema requires an object");

        assert!(matches!(
            error,
            ValidationError::InvalidRootKind { .. }
        ));
    }

    #[test]
    fn validation_report_counts_nodes() {
        let schema = test_schema(vec![], UnknownFieldPolicy::Preserve);
        let payload = serde_json::json!({"values": [1, 2, 3]});

        let report = validate_payload(
            &payload,
            &schema,
            &ValidationLimits::unbounded(),
        )
        .expect("payload should validate");

        assert_eq!(report.nodes_visited, 5);
        assert_eq!(report.collection_entries, 4);
    }
}

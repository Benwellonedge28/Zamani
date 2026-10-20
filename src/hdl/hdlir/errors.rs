
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani HDLIR Error Contracts
//!
//! Path: `src/hdl/hdlir/errors.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//! Status: Foundational API; freeze after repository integration tests pass.
//!
//! ## Purpose
//!
//! Define structured failures shared by HDLIR construction, schema handling,
//! verification, analysis, transformations, serialization, and lowering.
//!
//! ## Authority
//!
//! This module follows:
//!
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4`
//! - `grammar/specification/semantic-model.md`
//! - `src/frontend/semantic/domains/hdl.rs`
//! - `src/frontend/semantic/model/ids.rs`
//! - `src/zuir/ZUIR-SPEC.md`
//!
//! The language grammar and semantic model own source-level meaning.
//! The frontend owns source-level diagnostics and semantic validation.
//! HDLIR owns failures arising from its own representation contracts and
//! operations. This module does not replace the canonical diagnostic system.
//!
//! ## Owns
//!
//! - Stable machine-readable HDLIR error codes.
//! - Structured error categories and contextual information.
//! - Explicit arithmetic/conversion failure reporting.
//! - Explicit operational-budget exhaustion.
//! - Clear distinctions between malformed artifacts, invalid IR, unsupported
//!   IR features, and incomplete processing.
//!
//! ## Does not own
//!
//! - Parsing or source-level semantic validation.
//! - Canonical AST nodes, source spans, or frontend diagnostics.
//! - HDLIR IDs, schema definitions, types, values, or graph structures.
//! - Target discovery, physical allocation, placement, routing, synthesis,
//!   timing closure, or hardware execution.
//! - A universal list of every possible hardware operation.
//!
//! ## Dependency contract
//!
//! This is a foundational module. It depends only on the Rust standard
//! library and must not import sibling HDLIR modules. Future HDLIR modules
//! may depend on these error contracts without creating dependency cycles.
//!
//! ## Error and scalability contract
//!
//! Errors are returned rather than used to silently change program meaning.
//! A budget failure means the requested operation did not complete within
//! that budget; it is not proof that the design is semantically invalid.
//! Callers must not convert an incomplete verification or analysis into
//! successful verification.
//!
//! This module defines no universal hardware maxima. Numeric values included
//! in an error are diagnostic context, not language-level capacity limits.
//! The caller determines operational budgets and whether a failed operation
//! can be retried with a different configuration.
//!
//! ## Freeze criteria
//!
//! - Every public error has a documented meaning.
//! - Error codes are stable identifiers, not parsed display strings.
//! - Formatting is deterministic for the same error value.
//! - Budget exhaustion is distinct from invalid IR.
//! - Target infeasibility is not misrepresented as source or IR invalidity.
//! - Focused tests and repository-wide checks pass.

use core::fmt;

/// Result type used by HDLIR APIs.
///
/// The error type is intentionally fixed so construction, verification,
/// transformation, and artifact APIs can compose failures consistently.
pub type HdlirResult<T> = Result<T, HdlirError>;

/// Stable machine-readable classification for an HDLIR failure.
///
/// The textual code returned by [`HdlirErrorCode::as_str`] is intended for
/// logs, tests, structured reports, and external tooling. It must not be
/// derived from the human-readable error message.
///
/// Additions are permitted as the HDLIR contract grows. Existing code strings
/// must not be silently reassigned to different meanings.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum HdlirErrorCode {
    /// A required input value or argument is invalid.
    InvalidInput,
    /// An IR invariant or required representation contract is violated.
    InvalidInvariant,
    /// An identity or named entity is duplicated within its declared scope.
    DuplicateEntity,
    /// A referenced entity does not exist in the relevant scope.
    MissingEntity,
    /// A requested feature has no registered or implemented interpretation.
    UnsupportedFeature,
    /// A required schema or definition cannot be found.
    UnknownSchema,
    /// A schema's declared version does not match the required contract.
    SchemaVersionMismatch,
    /// An artifact version is incompatible with the available implementation.
    IncompatibleArtifactVersion,
    /// Checked arithmetic could not represent a result.
    ArithmeticOverflow,
    /// A value cannot be represented by a requested destination type.
    ConversionOutOfRange,
    /// An explicitly configured operational budget was exhausted.
    BudgetExceeded,
    /// An external artifact is malformed or internally inconsistent.
    MalformedArtifact,
    /// An artifact read or write operation failed.
    ArtifactIo,
    /// Encoding an IR artifact failed.
    SerializationFailed,
    /// Decoding an IR artifact failed.
    DeserializationFailed,
    /// A requested transformation failed to meet its contract.
    TransformationFailed,
    /// Required verification or analysis did not finish.
    IncompleteProcessing,
    /// A registered operation or interface contract was violated.
    OperationContractViolation,
}

impl HdlirErrorCode {
    /// Return the stable lowercase identifier for this error code.
    ///
    /// These identifiers are compatibility-sensitive and should be used
    /// instead of matching on formatted error messages.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InvalidInput => "hdlir.invalid_input",
            Self::InvalidInvariant => "hdlir.invalid_invariant",
            Self::DuplicateEntity => "hdlir.duplicate_entity",
            Self::MissingEntity => "hdlir.missing_entity",
            Self::UnsupportedFeature => "hdlir.unsupported_feature",
            Self::UnknownSchema => "hdlir.unknown_schema",
            Self::SchemaVersionMismatch => "hdlir.schema_version_mismatch",
            Self::IncompatibleArtifactVersion => {
                "hdlir.incompatible_artifact_version"
            }
            Self::ArithmeticOverflow => "hdlir.arithmetic_overflow",
            Self::ConversionOutOfRange => "hdlir.conversion_out_of_range",
            Self::BudgetExceeded => "hdlir.budget_exceeded",
            Self::MalformedArtifact => "hdlir.malformed_artifact",
            Self::ArtifactIo => "hdlir.artifact_io",
            Self::SerializationFailed => "hdlir.serialization_failed",
            Self::DeserializationFailed => "hdlir.deserialization_failed",
            Self::TransformationFailed => "hdlir.transformation_failed",
            Self::IncompleteProcessing => "hdlir.incomplete_processing",
            Self::OperationContractViolation => {
                "hdlir.operation_contract_violation"
            }
        }
    }
}

impl fmt::Display for HdlirErrorCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Category of operational work that may be explicitly budgeted.
///
/// These categories describe implementation work, not language-level limits
/// on the number or size of hardware entities. The caller owns the actual
/// budget values and may configure them for its environment.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum HdlirBudgetKind {
    /// Memory available to a particular HDLIR operation.
    Memory,
    /// Number of processing steps or units of work.
    Work,
    /// Traversal or recursive evaluation depth.
    RecursionDepth,
    /// Verification effort.
    Verification,
    /// Analysis effort.
    Analysis,
    /// Transformation effort.
    Transformation,
    /// Input or output artifact processing.
    ArtifactProcessing,
    /// A caller-defined budget not covered by a standard category.
    Custom,
}

impl HdlirBudgetKind {
    /// Return the stable lowercase label for this budget category.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Memory => "memory",
            Self::Work => "work",
            Self::RecursionDepth => "recursion_depth",
            Self::Verification => "verification",
            Self::Analysis => "analysis",
            Self::Transformation => "transformation",
            Self::ArtifactProcessing => "artifact_processing",
            Self::Custom => "custom",
        }
    }
}

impl fmt::Display for HdlirBudgetKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Structured errors produced by HDLIR-owned functionality.
///
/// Context strings are explanatory metadata. Program logic should branch on
/// [`HdlirError::code`] or the enum variant, never on a message's wording.
///
/// This type intentionally does not embed frontend diagnostics, source spans,
/// schema objects, or future IR graph types. Those are owned by their
/// respective modules and can attach their context at the integration layer.
#[derive(Debug)]
#[non_exhaustive]
pub enum HdlirError {
    /// An argument or required input is invalid.
    InvalidInput {
        /// Name of the invalid field or input, when known.
        field: &'static str,
        /// Explanation of the input contract that was violated.
        reason: String,
    },

    /// An invariant required by the HDLIR specification is violated.
    InvalidInvariant {
        /// Stable, specification-oriented invariant label.
        invariant: &'static str,
        /// Details useful for debugging or diagnostics.
        detail: String,
    },

    /// An identity or named entity occurs more than once in one scope.
    DuplicateEntity {
        /// Entity category, such as `module`, `operation`, or `schema`.
        entity_kind: &'static str,
        /// Human-readable identity representation.
        identity: String,
        /// Scope in which uniqueness was required, when known.
        scope: Option<String>,
    },

    /// A reference cannot be resolved in its declared namespace.
    MissingEntity {
        /// Entity category being referenced.
        entity_kind: &'static str,
        /// Human-readable identity representation.
        identity: String,
        /// Namespace or owner scope, when known.
        scope: Option<String>,
    },

    /// A requested operation or feature is not supported by this implementation.
    ///
    /// This variant means HDLIR cannot interpret the requested feature. It
    /// must not be used to claim that the source program is invalid or that
    /// a physical target is infeasible.
    UnsupportedFeature {
        /// Feature or dialect namespace.
        feature: String,
        /// Specific operation or capability requested.
        operation: String,
        /// Explanation or implementation guidance.
        detail: Option<String>,
    },

    /// A referenced schema or registered definition does not exist.
    UnknownSchema {
        /// Kind of schema definition requested.
        schema_kind: &'static str,
        /// Qualified schema name or identity.
        name: String,
    },

    /// A schema was found, but its version fails the required compatibility rule.
    SchemaVersionMismatch {
        /// Qualified schema name or identity.
        schema: String,
        /// Required schema version or version contract.
        expected: String,
        /// Version actually provided.
        found: String,
    },

    /// An artifact version cannot be read under the active compatibility policy.
    IncompatibleArtifactVersion {
        /// Version recorded in the artifact.
        artifact_version: String,
        /// Supported version or version range.
        supported: String,
        /// Reason the artifact cannot be interpreted safely.
        detail: Option<String>,
    },

    /// A checked arithmetic operation overflowed its result type.
    ArithmeticOverflow {
        /// Operation being performed, such as `width multiplication`.
        operation: &'static str,
        /// Optional description of the affected quantity.
        context: Option<String>,
    },

    /// A value cannot be represented by the requested destination type.
    ConversionOutOfRange {
        /// Value or quantity being converted.
        value: String,
        /// Destination representation, such as `usize` or a schema field.
        destination: &'static str,
    },

    /// A configured operational budget prevented the operation from continuing.
    ///
    /// `limit` and `observed` are optional because some budgets are qualitative
    /// or externally enforced and cannot be reported as exact numeric counts.
    BudgetExceeded {
        /// Category of exhausted work.
        budget: HdlirBudgetKind,
        /// Configured limit, if the caller exposes a numeric limit.
        limit: Option<u128>,
        /// Observed consumption when processing stopped, if known.
        observed: Option<u128>,
        /// Operation whose work was interrupted.
        operation: String,
    },

    /// An artifact is malformed or violates its declared format contract.
    MalformedArtifact {
        /// Artifact section or field where the problem was found.
        section: &'static str,
        /// Explanation of the malformed content.
        detail: String,
    },

    /// Reading or writing an artifact failed.
    ///
    /// The original error message is retained for diagnosis. Callers should
    /// not rely on platform-specific message text for program logic.
    ArtifactIo {
        /// I/O action that failed, such as `read` or `write`.
        operation: &'static str,
        /// Standard-library error category.
        kind: std::io::ErrorKind,
        /// Original error message.
        message: String,
    },

    /// Encoding an in-memory IR artifact failed.
    SerializationFailed {
        /// Stage or section being encoded.
        section: &'static str,
        /// Explanation of the failure.
        detail: String,
    },

    /// Decoding an artifact into HDLIR failed.
    DeserializationFailed {
        /// Stage or section being decoded.
        section: &'static str,
        /// Explanation of the failure.
        detail: String,
    },

    /// A transformation failed to satisfy its declared execution contract.
    TransformationFailed {
        /// Stable transformation or pass name.
        pass: String,
        /// Explanation of the failure.
        detail: String,
    },

    /// Required processing stopped before all required checks completed.
    ///
    /// This variant records incompleteness. It is not equivalent to a
    /// successful verification result and does not, by itself, mean the IR
    /// is invalid.
    IncompleteProcessing {
        /// Stage that did not complete.
        stage: &'static str,
        /// Reason processing stopped.
        reason: String,
    },

    /// An operation violates the contract registered for its schema.
    OperationContractViolation {
        /// Registered dialect or schema name.
        schema: String,
        /// Operation name or identity.
        operation: String,
        /// Explanation of the violated contract.
        detail: String,
    },
}

impl HdlirError {
    /// Return the stable machine-readable code for this error.
    #[must_use]
    pub const fn code(&self) -> HdlirErrorCode {
        match self {
            Self::InvalidInput { .. } => HdlirErrorCode::InvalidInput,
            Self::InvalidInvariant { .. } => HdlirErrorCode::InvalidInvariant,
            Self::DuplicateEntity { .. } => HdlirErrorCode::DuplicateEntity,
            Self::MissingEntity { .. } => HdlirErrorCode::MissingEntity,
            Self::UnsupportedFeature { .. } => HdlirErrorCode::UnsupportedFeature,
            Self::UnknownSchema { .. } => HdlirErrorCode::UnknownSchema,
            Self::SchemaVersionMismatch { .. } => {
                HdlirErrorCode::SchemaVersionMismatch
            }
            Self::IncompatibleArtifactVersion { .. } => {
                HdlirErrorCode::IncompatibleArtifactVersion
            }
            Self::ArithmeticOverflow { .. } => HdlirErrorCode::ArithmeticOverflow,
            Self::ConversionOutOfRange { .. } => {
                HdlirErrorCode::ConversionOutOfRange
            }
            Self::BudgetExceeded { .. } => HdlirErrorCode::BudgetExceeded,
            Self::MalformedArtifact { .. } => HdlirErrorCode::MalformedArtifact,
            Self::ArtifactIo { .. } => HdlirErrorCode::ArtifactIo,
            Self::SerializationFailed { .. } => {
                HdlirErrorCode::SerializationFailed
            }
            Self::DeserializationFailed { .. } => {
                HdlirErrorCode::DeserializationFailed
            }
            Self::TransformationFailed { .. } => {
                HdlirErrorCode::TransformationFailed
            }
            Self::IncompleteProcessing { .. } => {
                HdlirErrorCode::IncompleteProcessing
            }
            Self::OperationContractViolation { .. } => {
                HdlirErrorCode::OperationContractViolation
            }
        }
    }

    /// Construct an invalid-input error.
    #[must_use]
    pub fn invalid_input(
        field: &'static str,
        reason: impl Into<String>,
    ) -> Self {
        Self::InvalidInput {
            field,
            reason: reason.into(),
        }
    }

    /// Construct an invariant-violation error.
    #[must_use]
    pub fn invalid_invariant(
        invariant: &'static str,
        detail: impl Into<String>,
    ) -> Self {
        Self::InvalidInvariant {
            invariant,
            detail: detail.into(),
        }
    }

    /// Construct an explicit operational-budget exhaustion error.
    #[must_use]
    pub fn budget_exceeded(
        budget: HdlirBudgetKind,
        limit: Option<u128>,
        observed: Option<u128>,
        operation: impl Into<String>,
    ) -> Self {
        Self::BudgetExceeded {
            budget,
            limit,
            observed,
            operation: operation.into(),
        }
    }

    /// Construct an incomplete-processing error.
    #[must_use]
    pub fn incomplete(
        stage: &'static str,
        reason: impl Into<String>,
    ) -> Self {
        Self::IncompleteProcessing {
            stage,
            reason: reason.into(),
        }
    }
}

impl fmt::Display for HdlirError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: ", self.code().as_str())?;

        match self {
            Self::InvalidInput { field, reason } => {
                write!(formatter, "invalid input `{field}`: {reason}")
            }
            Self::InvalidInvariant { invariant, detail } => {
                write!(formatter, "invariant `{invariant}` violated: {detail}")
            }
            Self::DuplicateEntity {
                entity_kind,
                identity,
                scope,
            } => {
                write!(formatter, "duplicate {entity_kind} `{identity}`")?;
                if let Some(scope) = scope {
                    write!(formatter, " in scope `{scope}`")?;
                }
                Ok(())
            }
            Self::MissingEntity {
                entity_kind,
                identity,
                scope,
            } => {
                write!(formatter, "missing {entity_kind} `{identity}`")?;
                if let Some(scope) = scope {
                    write!(formatter, " in scope `{scope}`")?;
                }
                Ok(())
            }
            Self::UnsupportedFeature {
                feature,
                operation,
                detail,
            } => {
                write!(
                    formatter,
                    "unsupported feature `{feature}` for operation `{operation}`"
                )?;
                if let Some(detail) = detail {
                    write!(formatter, ": {detail}")?;
                }
                Ok(())
            }
            Self::UnknownSchema { schema_kind, name } => {
                write!(formatter, "unknown {schema_kind} schema `{name}`")
            }
            Self::SchemaVersionMismatch {
                schema,
                expected,
                found,
            } => {
                write!(
                    formatter,
                    "schema `{schema}` has version `{found}`, expected `{expected}`"
                )
            }
            Self::IncompatibleArtifactVersion {
                artifact_version,
                supported,
                detail,
            } => {
                write!(
                    formatter,
                    "artifact version `{artifact_version}` is incompatible; \
                     supported: {supported}"
                )?;
                if let Some(detail) = detail {
                    write!(formatter, ": {detail}")?;
                }
                Ok(())
            }
            Self::ArithmeticOverflow { operation, context } => {
                write!(formatter, "arithmetic overflow during `{operation}`")?;
                if let Some(context) = context {
                    write!(formatter, " ({context})")?;
                }
                Ok(())
            }
            Self::ConversionOutOfRange { value, destination } => {
                write!(
                    formatter,
                    "value `{value}` cannot be represented as `{destination}`"
                )
            }
            Self::BudgetExceeded {
                budget,
                limit,
                observed,
                operation,
            } => {
                write!(
                    formatter,
                    "{budget} budget exhausted during `{operation}`"
                )?;
                if let Some(limit) = limit {
                    write!(formatter, " (limit: {limit}")?;
                    if let Some(observed) = observed {
                        write!(formatter, ", observed: {observed}")?;
                    }
                    write!(formatter, ")")?;
                } else if let Some(observed) = observed {
                    write!(formatter, " (observed: {observed})")?;
                }
                Ok(())
            }
            Self::MalformedArtifact { section, detail } => {
                write!(formatter, "malformed artifact section `{section}`: {detail}")
            }
            Self::ArtifactIo {
                operation,
                kind,
                message,
            } => {
                write!(
                    formatter,
                    "artifact {operation} failed ({kind:?}): {message}"
                )
            }
            Self::SerializationFailed { section, detail } => {
                write!(formatter, "serialization failed at `{section}`: {detail}")
            }
            Self::DeserializationFailed { section, detail } => {
                write!(formatter, "deserialization failed at `{section}`: {detail}")
            }
            Self::TransformationFailed { pass, detail } => {
                write!(formatter, "transformation `{pass}` failed: {detail}")
            }
            Self::IncompleteProcessing { stage, reason } => {
                write!(formatter, "processing incomplete at `{stage}`: {reason}")
            }
            Self::OperationContractViolation {
                schema,
                operation,
                detail,
            } => {
                write!(
                    formatter,
                    "operation `{operation}` violates schema `{schema}`: {detail}"
                )
            }
        }
    }
}

impl std::error::Error for HdlirError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_codes_are_stable_machine_readable_identifiers() {
        assert_eq!(
            HdlirErrorCode::InvalidInput.as_str(),
            "hdlir.invalid_input"
        );
        assert_eq!(
            HdlirErrorCode::BudgetExceeded.as_str(),
            "hdlir.budget_exceeded"
        );
        assert_eq!(
            HdlirErrorCode::IncompleteProcessing.as_str(),
            "hdlir.incomplete_processing"
        );
    }

    #[test]
    fn invalid_input_has_a_stable_code_and_context() {
        let error = HdlirError::invalid_input("width", "expression is malformed");

        assert_eq!(error.code(), HdlirErrorCode::InvalidInput);
        assert!(error.to_string().contains("width"));
        assert!(error.to_string().contains("expression is malformed"));
    }

    #[test]
    fn budget_exhaustion_is_not_invalid_ir() {
        let error = HdlirError::budget_exceeded(
            HdlirBudgetKind::Verification,
            Some(1_000),
            Some(1_000),
            "connectivity verification",
        );

        assert_eq!(error.code(), HdlirErrorCode::BudgetExceeded);
        assert_ne!(error.code(), HdlirErrorCode::InvalidInvariant);
        assert!(error.to_string().contains("verification"));
        assert!(error.to_string().contains("connectivity verification"));
    }

    #[test]
    fn unknown_observed_budget_is_representable() {
        let error = HdlirError::budget_exceeded(
            HdlirBudgetKind::Memory,
            Some(4096),
            None,
            "artifact decoding",
        );

        assert_eq!(error.code(), HdlirErrorCode::BudgetExceeded);
        assert!(error.to_string().contains("memory"));
    }

    #[test]
    fn incomplete_processing_is_explicit() {
        let error = HdlirError::incomplete(
            "type verification",
            "the caller cancelled the operation",
        );

        assert_eq!(error.code(), HdlirErrorCode::IncompleteProcessing);
        assert!(error.to_string().contains("type verification"));
        assert!(error.to_string().contains("cancelled"));
    }

    #[test]
    fn arithmetic_overflow_is_distinct_from_invalid_input() {
        let error = HdlirError::ArithmeticOverflow {
            operation: "symbolic width multiplication",
            context: Some("evaluating a concrete specialization".to_owned()),
        };

        assert_eq!(error.code(), HdlirErrorCode::ArithmeticOverflow);
        assert_ne!(error.code(), HdlirErrorCode::InvalidInput);
    }

    #[test]
    fn artifact_io_retains_error_category() {
        let error = HdlirError::ArtifactIo {
            operation: "read",
            kind: std::io::ErrorKind::UnexpectedEof,
            message: "artifact ended before its header was complete".to_owned(),
        };

        assert_eq!(error.code(), HdlirErrorCode::ArtifactIo);
        assert!(error.to_string().contains("UnexpectedEof"));
    }

    #[test]
    fn operation_contract_errors_identify_schema_and_operation() {
        let error = HdlirError::OperationContractViolation {
            schema: "zamani.structural".to_owned(),
            operation: "connect".to_owned(),
            detail: "operand types are incompatible".to_owned(),
        };

        assert_eq!(
            error.code(),
            HdlirErrorCode::OperationContractViolation
        );
        assert!(error.to_string().contains("zamani.structural"));
        assert!(error.to_string().contains("connect"));
    }
}

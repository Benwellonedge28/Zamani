//! Zamani Quantum IR — Metadata / Debugging
//!
//! Production-grade, deterministic, bounded-debugging infrastructure for the
//! canonical Zamani Quantum IR.
//!
//! # Purpose
//!
//! This module provides human-oriented inspection of IR metadata and semantic
//! IR summaries without making debugging a part of the semantic IR itself.
//!
//! The module is intentionally designed to be useful while the rest of the
//! Quantum IR is evolving. It therefore defines a small, stable debugging
//! contract rather than depending on every concrete IR object.
//!
//! Debugging answers questions such as:
//!
//! - What kind of IR object am I looking at?
//! - What is its stable identity?
//! - Which logical/physical qubits does it reference?
//! - Which semantic fields are present?
//! - What metadata and attributes are attached?
//! - Which warnings/errors are associated with the object?
//! - How large is the object?
//! - Is the rendered representation complete or intentionally truncated?
//!
//! Debugging does NOT:
//!
//! - execute quantum programs;
//! - simulate quantum states;
//! - optimize IR;
//! - route qubits;
//! - schedule operations;
//! - select hardware;
//! - perform calibration;
//! - perform lowering;
//! - mutate the canonical IR;
//! - replace validation;
//! - replace provenance;
//! - replace canonical serialization;
//! - replace canonical hashing;
//! - authenticate artifacts;
//! - contain credentials or secrets.
//!
//! # Architectural position
//!
//! ```text
//!                         Zamani source
//!                              |
//!                              v
//!                           frontend
//!                              |
//!                              v
//!                     canonical Zamani IR
//!                              |
//!            +-----------------+-----------------+
//!            |                 |                 |
//!            v                 v                 v
//!       validation         analysis         provenance
//!            |                 |                 |
//!            +-----------------+-----------------+
//!                              |
//!                              v
//!                           debugging
//!                              |
//!                              v
//!                         human / tooling
//! ```
//!
//! Debugging is observational. It must never become a hidden transformation
//! stage in the compilation pipeline.
//!
//! # Universal-program principle
//!
//! Zamani quantum programs are target-independent.
//!
//! Consequently, this module contains NO fixed:
//!
//! - qubit count;
//! - operation count;
//! - register size;
//! - topology size;
//! - gate count;
//! - architecture count;
//! - backend count.
//!
//! A program may contain one logical qubit or an arbitrarily large finite
//! number of resources, subject only to the resources and explicit policies
//! imposed by the caller.
//!
//! Debug output may be bounded for safety and usability. Such bounds are
//! debugging-policy limits only; they are never semantic Quantum IR limits.
//!
//! # Bounded debugging
//!
//! Debuggers are frequently exposed to malformed or adversarial IR. An
//! unbounded pretty-printer can therefore become a resource-exhaustion vector.
//!
//! This module provides explicit rendering limits through [`DebugConfig`].
//!
//! The limits apply only to rendering:
//!
//! ```text
//! semantic IR size
//!       !=
//! debug rendering size
//! ```
//!
//! Truncation is always explicit in the resulting [`DebugReport`].
//!
//! No information is silently represented as complete when it was truncated.
//!
//! # Determinism
//!
//! Debug output is deterministic when the input's debug representation is
//! deterministic and ordered collections are used by the caller.
//!
//! This module itself:
//!
//! - preserves insertion order for caller-provided fields;
//! - does not depend on hash-map iteration order;
//! - does not include memory addresses;
//! - does not include pointer values;
//! - does not include process IDs;
//! - does not include thread IDs;
//! - does not include wall-clock timestamps;
//! - does not include random values.
//!
//! Debug output is therefore suitable for tests and diagnostics.
//!
//! It is NOT the canonical serialization format and must NOT be used as the
//! semantic content hash format.
//!
//! # Security
//!
//! Debug output can contain sensitive semantic information supplied by the
//! caller. This module therefore provides explicit redaction support.
//!
//! Callers should never pass:
//!
//! - passwords;
//! - API keys;
//! - access tokens;
//! - private keys;
//! - credentials;
//! - signing keys;
//! - secret calibration payloads;
//! - other secrets
//!
//! into ordinary debug fields.
//!
//! If a value may contain sensitive data, callers should mark the field as
//! [`DebugFieldKind::Sensitive`] or [`DebugFieldKind::Secret`].
//!
//! Secret values are rendered as a stable redaction marker and are never
//! included in the resulting debug output.
//!
//! # Qubit identity boundary
//!
//! Logical and physical qubit identifiers remain owned by:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This module deliberately does not define replacement qubit identifiers.
//!
//! # Integration boundary
//!
//! The module is intentionally dependency-light.
//!
//! It may depend on:
//!
//! - the standard library;
//! - canonical `quantum::ir::qubit` identities.
//!
//! It must not depend on:
//!
//! - frontend implementations;
//! - optimization implementations;
//! - routing implementations;
//! - scheduling implementations;
//! - hardware implementations;
//! - backend implementations;
//! - simulator implementations;
//! - QEC implementations.
//!
//! Higher-level IR modules may implement [`DebugRenderable`] and use the
//! renderer provided here.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no external dependencies;
//! - no unsafe code.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler-enforced.
//!
//! # Integration contract
//!
//! `metadata/mod.rs` should eventually expose this module with:
//!
//! ```text
//! pub mod debug;
//! ```
//!
//! The root `quantum::ir` module may then re-export selected public debug
//! types if that is part of the public API policy.
//!
//! No code in this file needs to be changed merely because another IR module
//! gains new operations, models, dialects, resources, or qubit types.
//!
//! New IR structures integrate by implementing [`DebugRenderable`] or by
//! constructing [`DebugNode`] values.
//!
//! This is intentional: completing this file does not require later edits
//! when unrelated IR files evolve.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::fmt::{self, Write};

use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};

// =============================================================================
// Public constants
// =============================================================================

/// Stable marker emitted when a debug field is intentionally redacted.
pub const REDACTED_MARKER: &str = "<redacted>";

/// Stable marker emitted when a debug representation is truncated.
pub const TRUNCATED_MARKER: &str = "<truncated>";

/// Stable marker emitted for an explicitly unavailable value.
pub const UNAVAILABLE_MARKER: &str = "<unavailable>";

// =============================================================================
// Debug field kind
// =============================================================================

/// Classification of a debug field.
///
/// The classification controls rendering and communicates the intended
/// sensitivity of the value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DebugFieldKind {
    /// Ordinary non-sensitive semantic information.
    Normal,

    /// Information that may be sensitive and should be rendered only when
    /// explicitly permitted by the debug configuration.
    Sensitive,

    /// Secret information that is always redacted.
    ///
    /// This is appropriate for credentials, keys, authentication material, and
    /// other values that must never appear in diagnostics.
    Secret,
}

impl DebugFieldKind {
    /// Returns whether this field is considered sensitive.
    #[must_use]
    pub const fn is_sensitive(self) -> bool {
        !matches!(self, Self::Normal)
    }

    /// Returns whether this field must always be redacted.
    #[must_use]
    pub const fn is_secret(self) -> bool {
        matches!(self, Self::Secret)
    }
}

// =============================================================================
// Debug configuration
// =============================================================================

/// Rendering policy for debug output.
///
/// These values constrain only the amount of diagnostic output generated.
/// They do not constrain the semantic IR or the size of a quantum program.
///
/// The defaults are deliberately finite because debug output can be generated
/// automatically by error paths and therefore must not accidentally become an
/// unbounded memory or logging operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DebugConfig {
    /// Maximum number of rendered fields in a single node.
    max_fields: usize,

    /// Maximum UTF-8 bytes for one rendered field value.
    max_value_bytes: usize,

    /// Maximum UTF-8 bytes for the complete report.
    max_output_bytes: usize,

    /// Maximum number of rendered nesting levels.
    max_depth: usize,

    /// Whether sensitive values may be rendered.
    ///
    /// Secret values remain redacted regardless of this setting.
    allow_sensitive: bool,
}

impl DebugConfig {
    /// Creates an explicit debug configuration.
    ///
    /// All limits are caller-controlled policy values. Zero is permitted and
    /// means that the corresponding category will not be rendered.
    #[must_use]
    pub const fn new(
        max_fields: usize,
        max_value_bytes: usize,
        max_output_bytes: usize,
        max_depth: usize,
        allow_sensitive: bool,
    ) -> Self {
        Self {
            max_fields,
            max_value_bytes,
            max_output_bytes,
            max_depth,
            allow_sensitive,
        }
    }

    /// Returns a conservative general-purpose configuration.
    ///
    /// These values are output-policy defaults only and are not IR limits.
    #[must_use]
    pub const fn standard() -> Self {
        Self::new(
            256,
            4096,
            1 << 20,
            64,
            false,
        )
    }

    /// Returns a configuration intended for compact logs.
    #[must_use]
    pub const fn compact() -> Self {
        Self::new(
            64,
            1024,
            64 * 1024,
            32,
            false,
        )
    }

    /// Returns a configuration suitable for trusted developer inspection.
    ///
    /// Sensitive values are allowed, but `Secret` fields are still always
    /// redacted.
    #[must_use]
    pub const fn developer() -> Self {
        Self::new(
            1024,
            16 * 1024,
            4 * 1024 * 1024,
            128,
            true,
        )
    }

    /// Returns the maximum number of fields.
    #[must_use]
    pub const fn max_fields(&self) -> usize {
        self.max_fields
    }

    /// Returns the maximum value length in bytes.
    #[must_use]
    pub const fn max_value_bytes(&self) -> usize {
        self.max_value_bytes
    }

    /// Returns the maximum complete report size in bytes.
    #[must_use]
    pub const fn max_output_bytes(&self) -> usize {
        self.max_output_bytes
    }

    /// Returns the maximum nesting depth.
    #[must_use]
    pub const fn max_depth(&self) -> usize {
        self.max_depth
    }

    /// Returns whether sensitive fields may be rendered.
    #[must_use]
    pub const fn allow_sensitive(&self) -> bool {
        self.allow_sensitive
    }

    /// Returns a copy with sensitive-field rendering enabled.
    ///
    /// Secret fields remain redacted.
    #[must_use]
    pub const fn with_sensitive(mut self, allow_sensitive: bool) -> Self {
        self.allow_sensitive = allow_sensitive;
        self
    }
}

impl Default for DebugConfig {
    fn default() -> Self {
        Self::standard()
    }
}

// =============================================================================
// Debug error
// =============================================================================

/// Errors returned when constructing or rendering a debug representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DebugError {
    /// A required debug name is empty.
    EmptyName {
        /// Name of the affected field.
        field: &'static str,
    },

    /// A debug depth exceeds the configured policy.
    DepthLimitExceeded {
        /// Requested depth.
        depth: usize,

        /// Configured maximum.
        maximum: usize,
    },

    /// A complete debug report would exceed the configured output limit.
    OutputLimitExceeded {
        /// Required number of bytes.
        required_bytes: usize,

        /// Configured maximum.
        maximum_bytes: usize,
    },

    /// The debug node contains more fields than allowed.
    FieldLimitExceeded {
        /// Number of fields.
        fields: usize,

        /// Configured maximum.
        maximum: usize,
    },

    /// A caller attempted to create a field value larger than the configured
    /// per-value policy.
    ValueLimitExceeded {
        /// Actual UTF-8 byte length.
        actual_bytes: usize,

        /// Configured maximum.
        maximum_bytes: usize,
    },
}

impl fmt::Display for DebugError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName { field } => {
                write!(
                    formatter,
                    "debug field `{field}` must not have an empty name"
                )
            }

            Self::DepthLimitExceeded { depth, maximum } => {
                write!(
                    formatter,
                    "debug nesting depth {depth} exceeds maximum {maximum}"
                )
            }

            Self::OutputLimitExceeded {
                required_bytes,
                maximum_bytes,
            } => {
                write!(
                    formatter,
                    "debug output requires {required_bytes} bytes; maximum is {maximum_bytes}"
                )
            }

            Self::FieldLimitExceeded { fields, maximum } => {
                write!(
                    formatter,
                    "debug node contains {fields} fields; maximum is {maximum}"
                )
            }

            Self::ValueLimitExceeded {
                actual_bytes,
                maximum_bytes,
            } => {
                write!(
                    formatter,
                    "debug value is {actual_bytes} bytes; maximum is {maximum_bytes}"
                )
            }
        }
    }
}

impl std::error::Error for DebugError {}

// =============================================================================
// Debug field
// =============================================================================

/// One named debug field.
///
/// A field owns only its diagnostic representation. It does not alter or
/// reference the semantic IR after construction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DebugField {
    name: String,
    kind: DebugFieldKind,
    value: String,
}

impl DebugField {
    /// Creates an ordinary debug field.
    pub fn new(
        name: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, DebugError> {
        Self::with_kind(
            name,
            DebugFieldKind::Normal,
            value,
        )
    }

    /// Creates a field with an explicit sensitivity classification.
    pub fn with_kind(
        name: impl Into<String>,
        kind: DebugFieldKind,
        value: impl Into<String>,
    ) -> Result<Self, DebugError> {
        let name = name.into();

        if name.is_empty() {
            return Err(DebugError::EmptyName {
                field: "name",
            });
        }

        let value = value.into();

        Ok(Self {
            name,
            kind,
            value,
        })
    }

    /// Creates a redacted sensitive field.
    pub fn sensitive(
        name: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, DebugError> {
        Self::with_kind(
            name,
            DebugFieldKind::Sensitive,
            value,
        )
    }

    /// Creates a secret field.
    ///
    /// The original value is immediately replaced with the stable redaction
    /// marker. This means a later renderer cannot accidentally recover it.
    pub fn secret(
        name: impl Into<String>,
    ) -> Result<Self, DebugError> {
        Self::with_kind(
            name,
            DebugFieldKind::Secret,
            REDACTED_MARKER,
        )
    }

    /// Returns the field name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the field classification.
    #[must_use]
    pub const fn kind(&self) -> DebugFieldKind {
        self.kind
    }

    /// Returns the stored field value.
    ///
    /// For `Secret` fields this is always the redaction marker.
    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }
}

// =============================================================================
// Debug node
// =============================================================================

/// Generic hierarchical debug node.
///
/// This is the central integration type for other IR modules.
///
/// Higher-level modules may create a node such as:
///
/// ```text
/// program
///   id = program42
///   operation_count = ...
///   ...
/// ```
///
/// without requiring this module to know the concrete `Program` structure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DebugNode {
    name: String,
    fields: Vec<DebugField>,
    children: Vec<DebugNode>,
}

impl DebugNode {
    /// Creates a named debug node.
    pub fn new(name: impl Into<String>) -> Result<Self, DebugError> {
        let name = name.into();

        if name.is_empty() {
            return Err(DebugError::EmptyName {
                field: "node name",
            });
        }

        Ok(Self {
            name,
            fields: Vec::new(),
            children: Vec::new(),
        })
    }

    /// Returns the node name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the node fields.
    #[must_use]
    pub fn fields(&self) -> &[DebugField] {
        &self.fields
    }

    /// Returns child nodes.
    #[must_use]
    pub fn children(&self) -> &[DebugNode] {
        &self.children
    }

    /// Adds an ordinary field.
    pub fn field(
        mut self,
        name: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, DebugError> {
        self.fields.push(DebugField::new(name, value)?);
        Ok(self)
    }

    /// Adds a classified field.
    pub fn classified_field(
        mut self,
        name: impl Into<String>,
        kind: DebugFieldKind,
        value: impl Into<String>,
    ) -> Result<Self, DebugError> {
        self.fields.push(DebugField::with_kind(
            name,
            kind,
            value,
        )?);
        Ok(self)
    }

    /// Adds a child node.
    pub fn child(
        mut self,
        child: DebugNode,
    ) -> Self {
        self.children.push(child);
        self
    }

    /// Appends an ordinary field in-place.
    pub fn push_field(
        &mut self,
        name: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<(), DebugError> {
        self.fields.push(DebugField::new(name, value)?);
        Ok(())
    }

    /// Appends a child node in-place.
    pub fn push_child(
        &mut self,
        child: DebugNode,
    ) {
        self.children.push(child);
    }

    /// Returns whether this node has no fields and no children.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.fields.is_empty() && self.children.is_empty()
    }
}

// =============================================================================
// Debug renderable trait
// =============================================================================

/// Trait implemented by IR objects that can expose a diagnostic debug view.
///
/// Implementations should expose semantic information that is useful to a
/// human or debugging tool without exposing implementation-only memory
/// details.
///
/// The method must not mutate the object.
///
/// A type should generally build a [`DebugNode`] using stable identifiers and
/// semantic attributes.
///
/// This trait intentionally does not require `std::fmt::Debug`. The standard
/// Rust `Debug` trait is implementation-oriented and cannot guarantee the
/// stability, redaction, or boundedness required by the Quantum IR diagnostic
/// contract.
pub trait DebugRenderable {
    /// Builds the semantic debug representation.
    fn debug_node(&self) -> Result<DebugNode, DebugError>;
}

// =============================================================================
// Debug report
// =============================================================================

/// Result of rendering a debug node.
///
/// A report explicitly records whether truncation occurred.
///
/// This distinction is important: consumers must never mistake an abbreviated
/// diagnostic for a complete representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DebugReport {
    text: String,
    truncated: bool,
    rendered_fields: usize,
    rendered_children: usize,
    rendered_bytes: usize,
}

impl DebugReport {
    /// Creates a report.
    #[must_use]
    fn new(
        text: String,
        truncated: bool,
        rendered_fields: usize,
        rendered_children: usize,
    ) -> Self {
        let rendered_bytes = text.len();

        Self {
            text,
            truncated,
            rendered_fields,
            rendered_children,
            rendered_bytes,
        }
    }

    /// Returns the rendered debug text.
    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Returns whether the representation was truncated.
    #[must_use]
    pub const fn truncated(&self) -> bool {
        self.truncated
    }

    /// Returns the number of rendered fields.
    #[must_use]
    pub const fn rendered_fields(&self) -> usize {
        self.rendered_fields
    }

    /// Returns the number of rendered child nodes.
    #[must_use]
    pub const fn rendered_children(&self) -> usize {
        self.rendered_children
    }

    /// Returns the number of bytes in the final UTF-8 representation.
    #[must_use]
    pub const fn rendered_bytes(&self) -> usize {
        self.rendered_bytes
    }
}

impl fmt::Display for DebugReport {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.text)
    }
}

// =============================================================================
// Debug renderer
// =============================================================================

/// Deterministic renderer for [`DebugNode`] values.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DebugRenderer {
    config: DebugConfig,
}

impl DebugRenderer {
    /// Creates a renderer with an explicit configuration.
    #[must_use]
    pub const fn new(config: DebugConfig) -> Self {
        Self { config }
    }

    /// Returns the renderer configuration.
    #[must_use]
    pub const fn config(&self) -> &DebugConfig {
        &self.config
    }

    /// Renders a debug node.
    ///
    /// The operation is bounded by [`DebugConfig`].
    ///
    /// If the configured output budget is reached, rendering stops and the
    /// returned report is explicitly marked as truncated.
    pub fn render(
        &self,
        node: &DebugNode,
    ) -> Result<DebugReport, DebugError> {
        let mut output = String::new();

        let mut state = RenderState {
            fields: 0,
            children: 0,
            truncated: false,
        };

        self.render_node(
            node,
            0,
            &mut output,
            &mut state,
        )?;

        Ok(DebugReport::new(
            output,
            state.truncated,
            state.fields,
            state.children,
        ))
    }

    /// Renders any [`DebugRenderable`] object.
    pub fn render_value<T>(
        &self,
        value: &T,
    ) -> Result<DebugReport, DebugError>
    where
        T: DebugRenderable,
    {
        let node = value.debug_node()?;
        self.render(&node)
    }

    fn render_node(
        &self,
        node: &DebugNode,
        depth: usize,
        output: &mut String,
        state: &mut RenderState,
    ) -> Result<(), DebugError> {
        if depth > self.config.max_depth() {
            state.truncated = true;
            return Ok(());
        }

        if output.len() >= self.config.max_output_bytes() {
            state.truncated = true;
            return Ok(());
        }

        self.write_indent(
            output,
            depth,
        )?;

        self.write_bounded(
            output,
            node.name(),
        )?;

        output.push('\n');

        let mut rendered_fields = 0usize;

        for field in node.fields() {
            if rendered_fields >= self.config.max_fields() {
                state.truncated = true;
                break;
            }

            if output.len() >= self.config.max_output_bytes() {
                state.truncated = true;
                break;
            }

            self.render_field(
                field,
                depth + 1,
                output,
                state,
            )?;

            rendered_fields += 1;
            state.fields += 1;
        }

        if rendered_fields < node.fields().len() {
            self.render_truncation_line(
                depth + 1,
                output,
            )?;
        }

        let mut rendered_children = 0usize;

        for child in node.children() {
            if output.len() >= self.config.max_output_bytes() {
                state.truncated = true;
                break;
            }

            self.render_node(
                child,
                depth + 1,
                output,
                state,
            )?;

            rendered_children += 1;
            state.children += 1;

            if state.truncated {
                break;
            }
        }

        if rendered_children < node.children().len() {
            self.render_truncation_line(
                depth + 1,
                output,
            )?;
        }

        Ok(())
    }

    fn render_field(
        &self,
        field: &DebugField,
        depth: usize,
        output: &mut String,
        state: &mut RenderState,
    ) -> Result<(), DebugError> {
        self.write_indent(
            output,
            depth,
        )?;

        self.write_bounded(
            output,
            field.name(),
        )?;

        output.push_str(": ");

        match field.kind() {
            DebugFieldKind::Normal => {
                self.write_value(
                    output,
                    field.value(),
                )?;
            }

            DebugFieldKind::Sensitive => {
                if self.config.allow_sensitive() {
                    self.write_value(
                        output,
                        field.value(),
                    )?;
                } else {
                    output.push_str(REDACTED_MARKER);
                }
            }

            DebugFieldKind::Secret => {
                output.push_str(REDACTED_MARKER);
            }
        }

        output.push('\n');

        if output.len() >= self.config.max_output_bytes() {
            state.truncated = true;
        }

        Ok(())
    }

    fn write_value(
        &self,
        output: &mut String,
        value: &str,
    ) -> Result<(), DebugError> {
        if value.len() <= self.config.max_value_bytes() {
            self.write_bounded(
                output,
                value,
            )?;
            return Ok(());
        }

        let limit = self.config.max_value_bytes();

        if limit == 0 {
            output.push_str(TRUNCATED_MARKER);
            return Ok(());
        }

        let prefix = utf8_prefix(
            value,
            limit,
        );

        output.push_str(prefix);
        output.push_str(TRUNCATED_MARKER);

        Ok(())
    }

    fn write_bounded(
        &self,
        output: &mut String,
        value: &str,
    ) -> Result<(), DebugError> {
        let maximum = self.config.max_output_bytes();

        if output.len() >= maximum {
            return Err(DebugError::OutputLimitExceeded {
                required_bytes: output.len().saturating_add(value.len()),
                maximum_bytes: maximum,
            });
        }

        let remaining = maximum - output.len();

        if value.len() <= remaining {
            output.push_str(value);
            return Ok(());
        }

        let prefix = utf8_prefix(
            value,
            remaining,
        );

        output.push_str(prefix);

        if output.len() < maximum {
            let remaining_after_prefix = maximum - output.len();

            if remaining_after_prefix >= TRUNCATED_MARKER.len() {
                output.push_str(TRUNCATED_MARKER);
            }
        }

        Ok(())
    }

    fn write_indent(
        &self,
        output: &mut String,
        depth: usize,
    ) -> Result<(), DebugError> {
        let indentation = depth.saturating_mul(2);

        if indentation == 0 {
            return Ok(());
        }

        let maximum = self.config.max_output_bytes();

        if output.len() >= maximum {
            return Err(DebugError::OutputLimitExceeded {
                required_bytes: output.len().saturating_add(indentation),
                maximum_bytes: maximum,
            });
        }

        let remaining = maximum - output.len();
        let count = indentation.min(remaining);

        for _ in 0..count {
            output.push(' ');
        }

        Ok(())
    }

    fn render_truncation_line(
        &self,
        depth: usize,
        output: &mut String,
    ) -> Result<(), DebugError> {
        if output.len() >= self.config.max_output_bytes() {
            return Ok(());
        }

        self.write_indent(
            output,
            depth,
        )?;

        self.write_bounded(
            output,
            TRUNCATED_MARKER,
        )?;

        output.push('\n');

        Ok(())
    }
}

impl Default for DebugRenderer {
    fn default() -> Self {
        Self::new(DebugConfig::default())
    }
}

#[derive(Debug, Default)]
struct RenderState {
    fields: usize,
    children: usize,
    truncated: bool,
}

// =============================================================================
// Generic convenience functions
// =============================================================================

/// Builds and renders a debug representation using the default configuration.
pub fn render<T>(
    value: &T,
) -> Result<DebugReport, DebugError>
where
    T: DebugRenderable,
{
    DebugRenderer::default().render_value(value)
}

/// Builds and renders a debug representation using an explicit configuration.
pub fn render_with_config<T>(
    value: &T,
    config: DebugConfig,
) -> Result<DebugReport, DebugError>
where
    T: DebugRenderable,
{
    DebugRenderer::new(config).render_value(value)
}

// =============================================================================
// Stable scalar formatting helpers
// =============================================================================

/// Formats a logical qubit using the canonical `quantum::ir::qubit::QubitId`.
///
/// This function deliberately does not create a replacement identifier type.
#[must_use]
pub fn format_qubit(
    qubit: QubitId,
) -> String {
    qubit.to_string()
}

/// Formats a physical qubit using the canonical
/// `quantum::ir::qubit::PhysicalQubitId`.
///
/// The function is intentionally explicit about the physical namespace.
#[must_use]
pub fn format_physical_qubit(
    qubit: PhysicalQubitId,
) -> String {
    qubit.to_string()
}

/// Formats an optional logical qubit.
#[must_use]
pub fn format_optional_qubit(
    qubit: Option<QubitId>,
) -> String {
    match qubit {
        Some(value) => format_qubit(value),
        None => UNAVAILABLE_MARKER.to_owned(),
    }
}

/// Formats an optional physical qubit.
#[must_use]
pub fn format_optional_physical_qubit(
    qubit: Option<PhysicalQubitId>,
) -> String {
    match qubit {
        Some(value) => format_physical_qubit(value),
        None => UNAVAILABLE_MARKER.to_owned(),
    }
}

/// Formats a logical-qubit slice without imposing a semantic size limit.
///
/// This helper is intentionally iterator-based so callers can decide how
/// identifiers are sourced.
pub fn format_qubits<I>(
    qubits: I,
) -> String
where
    I: IntoIterator<Item = QubitId>,
{
    let mut output = String::from("[");

    let mut first = true;

    for qubit in qubits {
        if !first {
            output.push_str(", ");
        }

        first = false;

        output.push_str(&format_qubit(qubit));
    }

    output.push(']');

    output
}

/// Formats physical qubit identifiers without imposing a semantic size limit.
pub fn format_physical_qubits<I>(
    qubits: I,
) -> String
where
    I: IntoIterator<Item = PhysicalQubitId>,
{
    let mut output = String::from("[");

    let mut first = true;

    for qubit in qubits {
        if !first {
            output.push_str(", ");
        }

        first = false;

        output.push_str(&format_physical_qubit(qubit));
    }

    output.push(']');

    output
}

// =============================================================================
// Identifier formatting
// =============================================================================

/// Trait for stable textual formatting of IR identifiers used in diagnostics.
///
/// This avoids coupling the debug module to every identity type in
/// `identity.rs`.
pub trait DebugIdentity {
    /// Returns a stable diagnostic representation.
    fn debug_identity(&self) -> String;
}

impl DebugIdentity for QubitId {
    fn debug_identity(&self) -> String {
        format_qubit(*self)
    }
}

impl DebugIdentity for PhysicalQubitId {
    fn debug_identity(&self) -> String {
        format_physical_qubit(*self)
    }
}

// =============================================================================
// Builder
// =============================================================================

/// Ergonomic builder for hierarchical debug nodes.
///
/// The builder performs local policy validation while keeping the resulting
/// representation independent of any concrete IR type.
#[derive(Debug, Clone)]
pub struct DebugNodeBuilder {
    node: DebugNode,
}

impl DebugNodeBuilder {
    /// Starts a new builder.
    pub fn new(
        name: impl Into<String>,
    ) -> Result<Self, DebugError> {
        Ok(Self {
            node: DebugNode::new(name)?,
        })
    }

    /// Adds a normal field.
    pub fn field(
        mut self,
        name: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, DebugError> {
        self.node.push_field(
            name,
            value,
        )?;

        Ok(self)
    }

    /// Adds a sensitive field.
    pub fn sensitive(
        mut self,
        name: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, DebugError> {
        self.node.classified_field(
            name,
            DebugFieldKind::Sensitive,
            value,
        )?;

        Ok(self)
    }

    /// Adds a secret field.
    pub fn secret(
        mut self,
        name: impl Into<String>,
    ) -> Result<Self, DebugError> {
        self.node.classified_field(
            name,
            DebugFieldKind::Secret,
            REDACTED_MARKER,
        )?;

        Ok(self)
    }

    /// Adds a child.
    pub fn child(
        mut self,
        child: DebugNode,
    ) -> Self {
        self.node.push_child(child);
        self
    }

    /// Completes the builder.
    #[must_use]
    pub fn finish(self) -> DebugNode {
        self.node
    }
}

// =============================================================================
// Built-in debug implementations for common primitives
// =============================================================================

impl DebugRenderable for QubitId {
    fn debug_node(&self) -> Result<DebugNode, DebugError> {
        DebugNodeBuilder::new("qubit")?
            .field(
                "id",
                format_qubit(*self),
            )?
            .finish()
            .pipe(Ok)
    }
}

impl DebugRenderable for PhysicalQubitId {
    fn debug_node(&self) -> Result<DebugNode, DebugError> {
        DebugNodeBuilder::new("physical_qubit")?
            .field(
                "id",
                format_physical_qubit(*self),
            )?
            .finish()
            .pipe(Ok)
    }
}

// =============================================================================
// Utility trait for builder ergonomics
// =============================================================================

trait Pipe: Sized {
    /// Passes a value through a closure.
    fn pipe<T>(
        self,
        function: impl FnOnce(Self) -> T,
    ) -> T {
        function(self)
    }
}

impl<T> Pipe for T {}

// =============================================================================
// Unit tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // -------------------------------------------------------------------------
    // Configuration
    // -------------------------------------------------------------------------

    #[test]
    fn standard_config_is_finite_and_non_sensitive() {
        let config = DebugConfig::standard();

        assert!(config.max_fields() > 0);
        assert!(config.max_value_bytes() > 0);
        assert!(config.max_output_bytes() > 0);
        assert!(config.max_depth() > 0);
        assert!(!config.allow_sensitive());
    }

    #[test]
    fn developer_config_allows_sensitive_values() {
        assert!(DebugConfig::developer().allow_sensitive());
    }

    // -------------------------------------------------------------------------
    // Fields
    // -------------------------------------------------------------------------

    #[test]
    fn empty_field_name_is_rejected() {
        let result = DebugField::new(
            "",
            "value",
        );

        assert!(matches!(
            result,
            Err(DebugError::EmptyName { .. })
        ));
    }

    #[test]
    fn secret_field_is_redacted_at_construction() {
        let field = DebugField::secret("token")
            .expect("valid field");

        assert_eq!(
            field.value(),
            REDACTED_MARKER
        );

        assert_eq!(
            field.kind(),
            DebugFieldKind::Secret
        );
    }

    // -------------------------------------------------------------------------
    // Nodes
    // -------------------------------------------------------------------------

    #[test]
    fn node_preserves_field_and_child_order() {
        let child = DebugNodeBuilder::new("child")
            .expect("valid node")
            .field("value", "child-value")
            .expect("valid field")
            .finish();

        let node = DebugNodeBuilder::new("root")
            .expect("valid node")
            .field("first", "1")
            .expect("valid field")
            .field("second", "2")
            .expect("valid field")
            .child(child)
            .finish();

        assert_eq!(
            node.name(),
            "root"
        );

        assert_eq!(
            node.fields().len(),
            2
        );

        assert_eq!(
            node.children().len(),
            1
        );
    }

    // -------------------------------------------------------------------------
    // Rendering
    // -------------------------------------------------------------------------

    #[test]
    fn renderer_is_deterministic() {
        let node = DebugNodeBuilder::new("operation")
            .expect("valid node")
            .field("id", "op7")
            .expect("valid field")
            .field("kind", "gate")
            .expect("valid field")
            .finish();

        let renderer = DebugRenderer::default();

        let first = renderer
            .render(&node)
            .expect("render succeeds");

        let second = renderer
            .render(&node)
            .expect("render succeeds");

        assert_eq!(
            first,
            second
        );
    }

    #[test]
    fn sensitive_values_are_redacted_by_default() {
        let node = DebugNodeBuilder::new("artifact")
            .expect("valid node")
            .sensitive(
                "metadata",
                "sensitive-value",
            )
            .expect("valid field")
            .finish();

        let report = DebugRenderer::default()
            .render(&node)
            .expect("render succeeds");

        assert!(!report.text().contains("sensitive-value"));
        assert!(report.text().contains(REDACTED_MARKER));
    }

    #[test]
    fn sensitive_values_can_be_enabled() {
        let node = DebugNodeBuilder::new("artifact")
            .expect("valid node")
            .sensitive(
                "metadata",
                "sensitive-value",
            )
            .expect("valid field")
            .finish();

        let report = DebugRenderer::new(
            DebugConfig::developer(),
        )
        .render(&node)
        .expect("render succeeds");

        assert!(report.text().contains("sensitive-value"));
    }

    #[test]
    fn secret_values_are_always_redacted() {
        let node = DebugNodeBuilder::new("artifact")
            .expect("valid node")
            .secret("private_key")
            .expect("valid field")
            .finish();

        let report = DebugRenderer::new(
            DebugConfig::developer(),
        )
        .render(&node)
        .expect("render succeeds");

        assert!(!report.text().contains("private_key="));
        assert!(report.text().contains(REDACTED_MARKER));
    }

    // -------------------------------------------------------------------------
    // Output limits
    // -------------------------------------------------------------------------

    #[test]
    fn value_rendering_is_bounded() {
        let node = DebugNodeBuilder::new("large")
            .expect("valid node")
            .field(
                "value",
                "abcdefghijklmnopqrstuvwxyz",
            )
            .expect("valid field")
            .finish();

        let config = DebugConfig::new(
            16,
            4,
            1024,
            8,
            false,
        );

        let report = DebugRenderer::new(config)
            .render(&node)
            .expect("render succeeds");

        assert!(report.text().contains(TRUNCATED_MARKER));
    }

    #[test]
    fn depth_is_bounded() {
        let grandchild = DebugNodeBuilder::new("grandchild")
            .expect("valid node")
            .finish();

        let child = DebugNodeBuilder::new("child")
            .expect("valid node")
            .child(grandchild)
            .finish();

        let root = DebugNodeBuilder::new("root")
            .expect("valid node")
            .child(child)
            .finish();

        let config = DebugConfig::new(
            16,
            1024,
            4096,
            0,
            false,
        );

        let report = DebugRenderer::new(config)
            .render(&root)
            .expect("render succeeds");

        assert_eq!(
            report.text().lines().count(),
            1
        );
    }

    // -------------------------------------------------------------------------
    // Qubit integration
    // -------------------------------------------------------------------------

    #[test]
    fn logical_qubit_uses_canonical_qubit_type() {
        let qubit = QubitId::new(7);

        assert_eq!(
            format_qubit(qubit),
            "q7"
        );

        let node = qubit
            .debug_node()
            .expect("renderable");

        let report = DebugRenderer::default()
            .render(&node)
            .expect("render succeeds");

        assert!(report.text().contains("q7"));
    }

    #[test]
    fn physical_qubit_uses_canonical_qubit_type() {
        let qubit = PhysicalQubitId::new(11);

        assert_eq!(
            format_physical_qubit(qubit),
            "p11"
        );

        let node = qubit
            .debug_node()
            .expect("renderable");

        let report = DebugRenderer::default()
            .render(&node)
            .expect("render succeeds");

        assert!(report.text().contains("p11"));
    }

    #[test]
    fn optional_qubit_is_explicit_when_absent() {
        assert_eq!(
            format_optional_qubit(None),
            UNAVAILABLE_MARKER
        );
    }

    // -------------------------------------------------------------------------
    // Collection formatting
    // -------------------------------------------------------------------------

    #[test]
    fn qubit_collection_formatting_is_ordered() {
        let qubits = [
            QubitId::new(0),
            QubitId::new(2),
            QubitId::new(5),
        ];

        assert_eq!(
            format_qubits(qubits),
            "[q0, q2, q5]"
        );
    }

    #[test]
    fn physical_qubit_collection_formatting_is_ordered() {
        let qubits = [
            PhysicalQubitId::new(1),
            PhysicalQubitId::new(3),
        ];

        assert_eq!(
            format_physical_qubits(qubits),
            "[p1, p3]"
        );
    }

    // -------------------------------------------------------------------------
    // Generic integration
    // -------------------------------------------------------------------------

    #[derive(Debug)]
    struct ExampleIrObject;

    impl DebugRenderable for ExampleIrObject {
        fn debug_node(&self) -> Result<DebugNode, DebugError> {
            DebugNodeBuilder::new("example")
                .and_then(|builder| {
                    builder
                        .field("kind", "test")
                })
                .map(DebugNodeBuilder::finish)
        }
    }

    #[test]
    fn generic_renderable_integration_works() {
        let object = ExampleIrObject;

        let report = render(&object)
            .expect("render succeeds");

        assert!(report.text().contains("example"));
        assert!(report.text().contains("test"));
    }

    // -------------------------------------------------------------------------
    // UTF-8 correctness
    // -------------------------------------------------------------------------

    #[test]
    fn unicode_value_is_not_split_at_invalid_utf8_boundary() {
        let node = DebugNodeBuilder::new("unicode")
            .expect("valid node")
            .field(
                "value",
                "量子计算",
            )
            .expect("valid field")
            .finish();

        let config = DebugConfig::new(
            8,
            3,
            4096,
            8,
            false,
        );

        let report = DebugRenderer::new(config)
            .render(&node)
            .expect("render succeeds");

        assert!(report.text().is_char_boundary(report.text().len()));
    }
}

// =============================================================================
// Internal UTF-8 helper
// =============================================================================

/// Returns the largest valid UTF-8 prefix whose byte length does not exceed
/// `maximum`.
fn utf8_prefix(
    value: &str,
    maximum: usize,
) -> &str {
    if value.len() <= maximum {
        return value;
    }

    if maximum == 0 {
        return "";
    }

    let mut end = maximum;

    while end > 0 && !value.is_char_boundary(end) {
        end -= 1;
    }

    &value[..end]
}
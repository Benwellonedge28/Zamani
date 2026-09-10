//! # Zamani Native AST — Effect Kind
//!
//! Extensible source-level identity for an effect kind.
//!
//! ## Architectural position
//!
//! `EffectKind` belongs to the native Zamani AST, but it deliberately does
//! **not** encode the implementation or semantic realization of an effect.
//!
//! ```text
//! Zamani source
//!     │
//!     ▼
//! lexer
//!     │
//!     ▼
//! parser
//!     │
//!     ▼
//! Native AST
//!     │
//!     ├── Effect declaration
//!     │       └── EffectKind (when source syntax supplies one)
//!     │
//!     ▼
//! structural validation
//!     ▼
//! semantic analysis
//!     ▼
//! semantic effect model
//!     ▼
//! ZUIR
//!     ▼
//! domain-specific lowering
//!     ▼
//! target realization
//! ```
//!
//! ## Core architectural rule
//!
//! `EffectKind` is an **open identifier**, not a closed enumeration.
//!
//! Do not implement the fundamental model as:
//!
//! ```text
//! enum EffectKind {
//!     IO,
//!     Mutation,
//!     Quantum,
//!     Measurement,
//!     Reset,
//!     Async,
//!     ...
//! }
//! ```
//!
//! Such an enum would make the AST dependent on today's effect taxonomy and
//! would require modification whenever Zamani gains a new language feature,
//! computational domain, execution model, or effect system.
//!
//! Instead, an effect kind is identified by:
//!
//! ```text
//! namespace + name
//! ```
//!
//! Examples may include:
//!
//! ```text
//! zamani:io
//! zamani:mutation
//! zamani:allocation
//! zamani:nondeterminism
//! zamani:async
//! zamani:measurement
//! zamani:resource
//! quantum:measurement
//! quantum:reset
//! custom-domain:some-effect
//! ```
//!
//! These names are examples of identifiers, not a closed list of supported
//! effects.
//!
//! ## Domain neutrality
//!
//! This type contains no knowledge of:
//!
//! - quantum hardware;
//! - qubit counts;
//! - quantum gates;
//! - QPU topology;
//! - CPU/GPU/FPGA/ASIC architecture;
//! - vendor backends;
//! - QIR;
//! - LLVM;
//! - MLIR;
//! - OpenQASM;
//! - routing;
//! - scheduling;
//! - calibration;
//! - pulse generation;
//! - QEC;
//! - resilience implementation;
//! - ZQN;
//! - runtime execution.
//!
//! An effect kind can therefore describe source-level intent for classical,
//! quantum, hybrid, distributed, accelerator, HDL, AI, or future computational
//! domains without changing this file.
//!
//! ## POCO-REAF
//!
//! The effect-kind representation contains no machine-size information.
//!
//! It has no:
//!
//! ```text
//! MAX_EFFECT_KINDS
//! MAX_EFFECT_NAME_LENGTH
//! MAX_QUANTUM_EFFECTS
//! MAX_QUBITS
//! MAX_RESOURCES
//! ```
//!
//! There is no fixed number of effect kinds and no fixed number of AST nodes.
//!
//! Compiler resource protection belongs to an explicit configurable compiler
//! policy, not to the language-level identity represented here.
//!
//! ## EffectKind versus Effect
//!
//! `Effect` and `EffectKind` have different responsibilities.
//!
//! ```text
//! Effect
//! ├── source-level declaration
//! ├── Node
//! ├── name
//! ├── generic parameters
//! ├── parameters
//! └── optional return type
//!
//! EffectKind
//! └── extensible classification/identity
//! ```
//!
//! An `Effect` declaration may exist without an explicit `EffectKind` when the
//! language syntax does not require one. Semantic analysis may subsequently
//! classify or resolve the effect.
//!
//! This separation prevents the AST declaration structure from becoming
//! coupled to a particular semantic effect taxonomy.
//!
//! ## Semantic boundary
//!
//! This file does **not** decide what an effect means.
//!
//! For example, the identifier:
//!
//! ```text
//! quantum:measurement
//! ```
//!
//! does not cause this AST type to know:
//!
//! - which measurement basis is used;
//! - which physical qubit is measured;
//! - which backend performs the measurement;
//! - which instruction implements it;
//! - what calibration is required;
//! - what error model applies;
//! - how the operation is scheduled.
//!
//! Those questions belong to semantic analysis and downstream compilation.
//!
//! ## Capability boundary
//!
//! `EffectKind` is not a capability.
//!
//! For example:
//!
//! ```text
//! quantum:measurement
//! ```
//!
//! must not be interpreted by this file as:
//!
//! ```text
//! requires IBM quantum hardware
//! ```
//!
//! Capability resolution belongs to the semantic/compiler layers.
//!
//! ## Resource boundary
//!
//! `EffectKind` does not own resources.
//!
//! It does not contain:
//!
//! - resource IDs;
//! - qubit IDs;
//! - register sizes;
//! - memory locations;
//! - device assignments;
//! - physical mappings.
//!
//! Resource analysis is downstream.
//!
//! ## Dependency contract
//!
//! This file may depend only on:
//!
//! - Rust standard-library facilities;
//! - `serde` for serialization.
//!
//! It must not depend on:
//!
//! - `Effect`;
//! - functions;
//! - declarations;
//! - parser;
//! - lexer;
//! - semantic analysis;
//! - compiler orchestration;
//! - ZUIR;
//! - quantum IR;
//! - hardware;
//! - QEC;
//! - ZQN;
//! - optimization;
//! - routing;
//! - scheduling;
//! - runtime;
//! - backend providers;
//! - QIR;
//! - LLVM;
//! - MLIR;
//! - OpenQASM.
//!
//! Keeping this file independent is intentional. Higher-level effect nodes
//! may consume `EffectKind`; `EffectKind` must never depend on them.
//!
//! ## Determinism
//!
//! `EffectKind` has deterministic:
//!
//! - equality;
//! - ordering;
//! - hashing;
//! - serialization;
//! - display formatting.
//!
//! No random state, timestamp, memory address, process identifier, or backend
//! state contributes to its identity.
//!
//! ## Serialization
//!
//! Serialization preserves the exact namespace and name supplied by the
//! source/compiler extension.
//!
//! No case folding or Unicode normalization is performed.
//!
//! Therefore:
//!
//! ```text
//! Foo
//! ```
//!
//! and:
//!
//! ```text
//! foo
//! ```
//!
//! remain distinct identifiers.
//!
//! The global AST serialization layer owns the overall AST schema version.
//! This file provides a local schema version only for the logical
//! `EffectKind` representation.
//!
//! ## Security
//!
//! Effect kinds are compiler input and therefore untrusted data.
//!
//! Construction validates the identifier structure and rejects malformed
//! namespace/name components.
//!
//! This implementation:
//!
//! - performs no I/O;
//! - performs no network access;
//! - executes no source code;
//! - uses no raw pointers;
//! - uses no `unsafe`;
//! - performs no unchecked indexing;
//! - contains no global mutable state.
//!
//! Resource limits for hostile input belong to configurable compiler policy.
//!
//! ## Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - edition 2021;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! ## Integration contract
//!
//! ```text
//! EffectKind
//!     ▲
//!     │
//! Effect AST
//!     │
//!     ▼
//! structural validation
//!     │
//!     ▼
//! semantic analysis
//!     │
//!     ▼
//! semantic effect identity
//!     │
//!     ▼
//! ZUIR
//! ```
//!
//! The central AST traversal system should treat `EffectKind` as ordinary
//! effect data. It must not introduce a special quantum traversal path.
//!
//! Parser code should construct an `EffectKind` only when the grammar/source
//! representation actually contains an effect-kind identity.
//!
//! Semantic analysis may resolve the identifier into a richer semantic effect
//! classification without modifying the source-level value.
//!
//! ZUIR lowering consumes the semantic result, not this type directly.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;
use core::str::FromStr;

use serde::{Deserialize, Serialize};

/// Logical schema version of the native AST `EffectKind` representation.
///
/// This is deliberately independent of:
///
/// - Zamani language version;
/// - compiler version;
/// - global AST serialization version;
/// - semantic-model version;
/// - ZUIR version;
/// - extension version;
/// - backend version.
pub const EFFECT_KIND_AST_SCHEMA_VERSION: u16 = 1;

/// Namespace convention for effect kinds defined by the native Zamani
/// language.
///
/// This is an identifier namespace only. It does not make the effect kind
/// semantically privileged.
pub const ZAMANI_EFFECT_NAMESPACE: &str = "zamani";

/// Namespace convention for quantum-language extensions.
///
/// This constant is a convenience for extensions. It is not a declaration
/// that quantum effects are part of the closed core effect taxonomy.
pub const QUANTUM_EFFECT_NAMESPACE: &str = "quantum";

/// Canonical separator used between namespace and effect name.
pub const EFFECT_KIND_SEPARATOR: char = ':';

/// An extensible source-level effect-kind identity.
///
/// # Design
///
/// An effect kind consists of:
///
/// ```text
/// namespace:name
/// ```
///
/// The namespace identifies the authority or language/domain extension that
/// defines the identifier. The name identifies the effect within that
/// namespace.
///
/// # Why this is not an enum
///
/// Zamani must be able to introduce new effects without modifying the native
/// AST. A closed enum would make every future effect a core compiler change.
///
/// `EffectKind` therefore uses an open identifier model.
///
/// # Examples
///
/// ```
/// use zamani::frontend::ast::node::effects::effect_kind::EffectKind;
///
/// let kind = EffectKind::new("zamani", "io").unwrap();
///
/// assert_eq!(kind.namespace(), "zamani");
/// assert_eq!(kind.name(), "io");
/// assert_eq!(kind.qualified_name(), "zamani:io");
/// ```
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct EffectKind {
    namespace: String,
    name: String,
}

impl EffectKind {
    /// Creates an effect kind from a namespace and name.
    ///
    /// Both components are preserved exactly as supplied.
    ///
    /// # Errors
    ///
    /// Returns [`EffectKindError`] if either component is empty or contains
    /// forbidden structural characters.
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> Result<Self, EffectKindError> {
        let namespace = namespace.into();
        let name = name.into();

        validate_component("namespace", &namespace)?;
        validate_component("name", &name)?;

        Ok(Self { namespace, name })
    }

    /// Creates an effect kind in the native Zamani namespace.
    ///
    /// This is a convenience constructor. It does not create a closed set of
    /// Zamani effect kinds.
    pub fn zamani(name: impl Into<String>) -> Result<Self, EffectKindError> {
        Self::new(ZAMANI_EFFECT_NAMESPACE, name)
    }

    /// Creates an effect kind in the quantum extension namespace.
    ///
    /// This does not make quantum semantics part of the native AST core.
    /// Quantum-specific meaning remains downstream.
    pub fn quantum(name: impl Into<String>) -> Result<Self, EffectKindError> {
        Self::new(QUANTUM_EFFECT_NAMESPACE, name)
    }

    /// Parses a fully qualified effect kind.
    ///
    /// The required format is:
    ///
    /// ```text
    /// namespace:name
    /// ```
    ///
    /// The separator is structural and must occur exactly once.
    pub fn parse(value: impl AsRef<str>) -> Result<Self, EffectKindError> {
        let value = value.as_ref();

        let mut separator = value.split(EFFECT_KIND_SEPARATOR);

        let namespace = separator.next().unwrap_or_default();
        let name = separator.next().unwrap_or_default();

        if separator.next().is_some() {
            return Err(EffectKindError::MultipleSeparators);
        }

        if namespace.is_empty() {
            return Err(EffectKindError::EmptyNamespace);
        }

        if name.is_empty() {
            return Err(EffectKindError::EmptyName);
        }

        Self::new(namespace, name)
    }

    /// Returns the namespace.
    #[inline]
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Returns the local effect-kind name.
    #[inline]
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the canonical fully qualified name.
    ///
    /// The returned value is:
    ///
    /// ```text
    /// namespace:name
    /// ```
    ///
    /// The method allocates a `String`.
    #[must_use]
    pub fn qualified_name(&self) -> String {
        let capacity = self
            .namespace
            .len()
            .saturating_add(1)
            .saturating_add(self.name.len());

        let mut result = String::with_capacity(capacity);
        result.push_str(&self.namespace);
        result.push(EFFECT_KIND_SEPARATOR);
        result.push_str(&self.name);

        result
    }

    /// Returns whether this effect kind belongs to the native Zamani
    /// namespace.
    #[inline]
    #[must_use]
    pub fn is_zamani(&self) -> bool {
        self.namespace == ZAMANI_EFFECT_NAMESPACE
    }

    /// Returns whether this effect kind belongs to the quantum extension
    /// namespace.
    #[inline]
    #[must_use]
    pub fn is_quantum(&self) -> bool {
        self.namespace == QUANTUM_EFFECT_NAMESPACE
    }

    /// Returns the local schema version of this representation.
    #[inline]
    #[must_use]
    pub const fn schema_version() -> u16 {
        EFFECT_KIND_AST_SCHEMA_VERSION
    }

    /// Converts this kind into its canonical textual representation.
    ///
    /// This is equivalent to [`Self::qualified_name`].
    #[inline]
    #[must_use]
    pub fn into_qualified_name(self) -> String {
        let capacity = self
            .namespace
            .len()
            .saturating_add(1)
            .saturating_add(self.name.len());

        let mut result = String::with_capacity(capacity);
        result.push_str(&self.namespace);
        result.push(EFFECT_KIND_SEPARATOR);
        result.push_str(&self.name);

        result
    }
}

impl fmt::Display for EffectKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.namespace)?;
        formatter.write_str(":")?;
        formatter.write_str(&self.name)
    }
}

impl FromStr for EffectKind {
    type Err = EffectKindError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

/// Errors produced while constructing or parsing an [`EffectKind`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EffectKindError {
    /// The namespace is empty.
    EmptyNamespace,

    /// The local effect name is empty.
    EmptyName,

    /// A namespace or name contains a forbidden character.
    InvalidCharacter {
        /// Component containing the invalid character.
        component: &'static str,

        /// The offending character.
        character: char,
    },

    /// The qualified identifier contains more than one namespace separator.
    MultipleSeparators,
}

impl fmt::Display for EffectKindError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyNamespace => {
                formatter.write_str("effect kind namespace must not be empty")
            }

            Self::EmptyName => {
                formatter.write_str("effect kind name must not be empty")
            }

            Self::InvalidCharacter {
                component,
                character,
            } => write!(
                formatter,
                "effect kind {} contains forbidden character {:?}",
                component, character
            ),

            Self::MultipleSeparators => {
                formatter.write_str(
                    "effect kind qualified name must contain exactly one ':' separator",
                )
            }
        }
    }
}

impl std::error::Error for EffectKindError {}

/// Validates one effect-kind identifier component.
///
/// The validation is intentionally structural rather than semantic.
///
/// Allowed identifiers may contain Unicode and ordinary punctuation useful for
/// namespaced identifiers. The following characters are rejected because they
/// would make the canonical representation ambiguous or unsuitable for
/// structural AST identifiers:
///
/// - control characters;
/// - whitespace;
/// - `/`;
/// - `\`;
/// - `:`.
fn validate_component(
    component: &'static str,
    value: &str,
) -> Result<(), EffectKindError> {
    if value.is_empty() {
        return Err(match component {
            "namespace" => EffectKindError::EmptyNamespace,
            _ => EffectKindError::EmptyName,
        });
    }

    for character in value.chars() {
        if character.is_control()
            || character.is_whitespace()
            || matches!(character, '/' | '\\' | ':')
        {
            return Err(EffectKindError::InvalidCharacter {
                component,
                character,
            });
        }
    }

    Ok(())
}

/// Returns a native Zamani effect kind.
///
/// This helper exists for code that wants a conventional built-in identifier
/// without introducing a closed `EffectKind` enum.
///
/// It intentionally returns a `Result` because even a compiler-defined name
/// must pass the same structural validation as an extension-defined name.
#[must_use]
pub fn zamani_effect_kind(
    name: impl Into<String>,
) -> Result<EffectKind, EffectKindError> {
    EffectKind::zamani(name)
}

/// Returns a quantum-extension effect kind.
///
/// This is an extension-namespace convenience function, not a declaration
/// that the native AST owns quantum semantics.
#[must_use]
pub fn quantum_effect_kind(
    name: impl Into<String>,
) -> Result<EffectKind, EffectKindError> {
    EffectKind::quantum(name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructs_namespaced_effect_kind() {
        let kind = EffectKind::new("zamani", "io").unwrap();

        assert_eq!(kind.namespace(), "zamani");
        assert_eq!(kind.name(), "io");
        assert_eq!(kind.qualified_name(), "zamani:io");
    }

    #[test]
    fn display_is_canonical() {
        let kind = EffectKind::new("example", "effect").unwrap();

        assert_eq!(kind.to_string(), "example:effect");
    }

    #[test]
    fn parses_canonical_name() {
        let kind: EffectKind = "zamani:mutation".parse().unwrap();

        assert_eq!(kind.namespace(), "zamani");
        assert_eq!(kind.name(), "mutation");
    }

    #[test]
    fn parsing_round_trips() {
        let original = EffectKind::new("example.domain", "custom_effect").unwrap();
        let encoded = original.to_string();
        let decoded = EffectKind::parse(encoded).unwrap();

        assert_eq!(original, decoded);
    }

    #[test]
    fn unicode_identifiers_are_preserved() {
        let kind = EffectKind::new("domain", "测量").unwrap();

        assert_eq!(kind.name(), "测量");
        assert_eq!(kind.to_string(), "domain:测量");
    }

    #[test]
    fn namespaces_are_case_sensitive() {
        let upper = EffectKind::new("Zamani", "io").unwrap();
        let lower = EffectKind::new("zamani", "io").unwrap();

        assert_ne!(upper, lower);
    }

    #[test]
    fn native_namespace_helper_works() {
        let kind = EffectKind::zamani("io").unwrap();

        assert!(kind.is_zamani());
        assert!(!kind.is_quantum());
    }

    #[test]
    fn quantum_namespace_helper_works() {
        let kind = EffectKind::quantum("measurement").unwrap();

        assert!(kind.is_quantum());
        assert!(!kind.is_zamani());
    }

    #[test]
    fn empty_namespace_is_rejected() {
        assert_eq!(
            EffectKind::new("", "io"),
            Err(EffectKindError::EmptyNamespace)
        );
    }

    #[test]
    fn empty_name_is_rejected() {
        assert_eq!(
            EffectKind::new("zamani", ""),
            Err(EffectKindError::EmptyName)
        );
    }

    #[test]
    fn whitespace_is_rejected() {
        assert!(matches!(
            EffectKind::new("zamani", "effect kind"),
            Err(EffectKindError::InvalidCharacter { .. })
        ));
    }

    #[test]
    fn colon_inside_component_is_rejected() {
        assert!(matches!(
            EffectKind::new("zamani:extra", "io"),
            Err(EffectKindError::InvalidCharacter { .. })
        ));
    }

    #[test]
    fn slash_inside_component_is_rejected() {
        assert!(matches!(
            EffectKind::new("zamani", "io/effect"),
            Err(EffectKindError::InvalidCharacter { .. })
        ));
    }

    #[test]
    fn multiple_qualified_separators_are_rejected() {
        assert_eq!(
            EffectKind::parse("a:b:c"),
            Err(EffectKindError::MultipleSeparators)
        );
    }

    #[test]
    fn missing_separator_is_rejected() {
        assert_eq!(
            EffectKind::parse("zamani_io"),
            Err(EffectKindError::EmptyName)
        );
    }

    #[test]
    fn ordering_is_deterministic() {
        let first = EffectKind::new("a", "z").unwrap();
        let second = EffectKind::new("b", "a").unwrap();

        assert!(first < second);
    }

    #[test]
    fn hashing_is_value_based() {
        use std::collections::HashSet;

        let first = EffectKind::new("zamani", "io").unwrap();
        let second = EffectKind::new("zamani", "io").unwrap();

        let mut values = HashSet::new();
        values.insert(first);

        assert!(values.contains(&second));
    }

    #[test]
    fn serde_round_trip_preserves_identity() {
        let original = EffectKind::new("custom.domain", "measurement").unwrap();

        let encoded = serde_json::to_string(&original).unwrap();
        let decoded: EffectKind = serde_json::from_str(&encoded).unwrap();

        assert_eq!(original, decoded);
    }

    #[test]
    fn large_names_are_not_artificially_limited() {
        let namespace = "n".repeat(4096);
        let name = "e".repeat(4096);

        let kind = EffectKind::new(namespace.clone(), name.clone()).unwrap();

        assert_eq!(kind.namespace(), namespace);
        assert_eq!(kind.name(), name);
    }

    #[test]
    fn schema_version_is_stable() {
        assert_eq!(
            EffectKind::schema_version(),
            EFFECT_KIND_AST_SCHEMA_VERSION
        );
    }
}
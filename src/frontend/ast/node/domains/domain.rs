//! # Zamani Native AST — Domain
//!
//! Production-grade source-level representation of a computational domain.
//!
//! ## Architectural position
//!
//! ```text
//! Zamani source
//!     │
//!     ▼
//! Lexer
//!     │
//!     ▼
//! Parser
//!     │
//!     ▼
//! Native Zamani AST
//!     │
//!     ├── Domain
//!     ├── Capability
//!     ├── Resource
//!     └── other source constructs
//!     │
//!     ▼
//! Structural AST validation
//!     │
//!     ▼
//! Semantic analysis
//!     │
//!     ├── domain resolution
//!     ├── capability resolution
//!     ├── resource analysis
//!     └── compatibility analysis
//!     │
//!     ▼
//! Semantic Model
//!     │
//!     ▼
//! ZUIR
//!     │
//!     ├── classical IR
//!     ├── quantum IR
//!     ├── HDL IR
//!     └── future domain IRs
//!     │
//!     ▼
//! Target lowering
//! ```
//!
//! ## Purpose
//!
//! `Domain` represents **source-level computational-domain intent**.
//!
//! It answers:
//!
//! > What computational domain does this source construct identify,
//! > refer to, require, provide, or constrain?
//!
//! It does NOT answer:
//!
//! - which machine implements the domain;
//! - which vendor provides it;
//! - which processor is selected;
//! - which QPU is selected;
//! - which qubit technology is selected;
//! - which instruction set is selected;
//! - which backend is selected;
//! - how resources are mapped;
//! - how operations are scheduled;
//! - how routing is performed;
//! - how error correction is implemented;
//! - how calibration is performed.
//!
//! Those concerns belong to later compilation stages.
//!
//! ## POCO-REAF
//!
//! The representation is intentionally independent of:
//!
//! - machine size;
//! - processor count;
//! - qubit count;
//! - register width;
//! - hardware topology;
//! - vendor;
//! - backend;
//! - instruction set;
//! - gate set;
//! - device identifier;
//! - runtime identifier.
//!
//! Therefore changing the execution target does not require changing this AST
//! node.
//!
//! ```text
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
//! ```
//!
//! ## Extensibility
//!
//! A closed enumeration such as:
//!
//! ```text
//! enum Domain {
//!     Classical,
//!     Quantum,
//!     HDL,
//! }
//! ```
//!
//! is deliberately NOT used as the fundamental representation.
//!
//! Such an enumeration would require modification whenever Zamani introduces
//! another computational domain.
//!
//! Instead, domains are identified by stable namespace-qualified names.
//!
//! Examples:
//!
//! ```text
//! zamani.classical
//! zamani.quantum
//! zamani.hybrid
//! zamani.hdl
//! zamani.ai
//! zamani.neuromorphic
//! zamani.photonic
//! future.example.domain
//! ```
//!
//! A new domain therefore does not require modifying this file.
//!
//! ## Boundary with domain-specific IR
//!
//! The direction is:
//!
//! ```text
//! frontend::ast::node::domains::Domain
//!             │
//!             ▼
//! semantic domain resolution
//!             │
//!             ▼
//! semantic domain identity
//!             │
//!             ▼
//! ZUIR
//!             │
//!             ▼
//! domain-specific IR
//! ```
//!
//! This module MUST NOT import:
//!
//! - `quantum::ir`;
//! - ZUIR;
//! - QIR;
//! - LLVM;
//! - MLIR;
//! - hardware backends;
//! - schedulers;
//! - routers;
//! - QEC;
//! - calibration systems.
//!
//! ## Rust compatibility
//!
//! This implementation targets:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! ## Security
//!
//! Domain identifiers are compiler input and must therefore be treated as
//! untrusted data.
//!
//! This type:
//!
//! - performs no filesystem access;
//! - performs no network access;
//! - executes no code;
//! - performs no backend lookup;
//! - stores no credentials;
//! - stores no device state;
//! - uses no global mutable state;
//! - uses no unsafe code.
//!
//! No arbitrary language-level maximum is imposed on namespace depth or name
//! length. Hostile-input limits belong to the configurable compiler resource
//! policy rather than the language representation.
//!
//! ## Determinism
//!
//! Equality, ordering and hashing depend exclusively on the domain's logical
//! source representation.
//!
//! No pointer addresses, timestamps, process IDs, random state, hardware state
//! or runtime state participate in identity.
//!
//! ## Integration contract
//!
//! ```text
//! parser
//!     │
//!     └── constructs Domain
//!
//! Domain
//!     │
//!     ├── structural validation
//!     ├── visitor traversal
//!     └── source diagnostics
//!
//! semantic analysis
//!     │
//!     └── resolves Domain into semantic domain identity
//!
//! semantic model
//!     │
//!     └── associates resolved identity with AST NodeId
//!
//! ZUIR
//!     │
//!     └── consumes resolved semantic domain information
//!
//! domain lowering
//!     │
//!     └── determines implementation strategy
//!
//! target/backend
//!     │
//!     └── determines actual machine realization
//! ```
//!
//! This file intentionally contains no downstream implementation details.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;

/// Logical schema version for the native AST domain node.
///
/// This is deliberately independent of the Zamani language version and the
/// global AST serialization version.
pub const DOMAIN_SCHEMA_VERSION: u16 = 1;

/// Stable textual identifier for this AST construct.
pub const DOMAIN_KIND_NAME: &str = "zamani:domain";

/// A source-level computational-domain identity.
///
/// The identity is namespace-qualified so that independently developed
/// extensions can coexist without requiring changes to the core AST.
///
/// The string is intentionally owned rather than interned here. Identifier
/// interning, if used, is a compiler-wide optimization and must not become part
/// of the AST contract.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Domain {
    namespace: String,
    name: String,
}

impl Domain {
    /// Creates a domain from a namespace and name.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::EmptyNamespace`] or [`DomainError::EmptyName`]
    /// when either component is empty.
    ///
    /// Returns [`DomainError::ControlCharacter`] when either component
    /// contains a Unicode control character.
    pub fn new<N, M>(namespace: N, name: M) -> Result<Self, DomainError>
    where
        N: Into<String>,
        M: Into<String>,
    {
        let namespace = namespace.into();
        let name = name.into();

        validate_component(&namespace, DomainComponent::Namespace)?;
        validate_component(&name, DomainComponent::Name)?;

        Ok(Self { namespace, name })
    }

    /// Creates a domain without re-validating an already validated pair.
    ///
    /// This constructor is crate-private so that only AST construction code
    /// that already performed the required validation can use it.
    ///
    /// The public [`Domain::new`] constructor is the preferred API.
    pub(crate) fn from_validated_parts(
        namespace: String,
        name: String,
    ) -> Self {
        Self { namespace, name }
    }

    /// Returns the domain namespace.
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Returns the unqualified domain name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the canonical namespace-qualified identity.
    ///
    /// The returned representation is:
    ///
    /// ```text
    /// namespace.name
    /// ```
    ///
    /// No canonicalization that changes user-provided identifiers is performed
    /// here. Semantic resolution owns language-specific identifier semantics.
    #[must_use]
    pub fn qualified_name(&self) -> String {
        let mut result =
            String::with_capacity(self.namespace.len() + 1 + self.name.len());

        result.push_str(&self.namespace);
        result.push('.');
        result.push_str(&self.name);

        result
    }

    /// Returns the number of namespace/name components represented by this
    /// domain.
    ///
    /// The namespace itself may contain multiple namespace components.
    ///
    /// This method exists for tooling and diagnostics only. It does not impose
    /// a semantic maximum.
    #[must_use]
    pub fn component_count(&self) -> usize {
        self.namespace.split('.').count() + 1
    }

    /// Validates the local structural invariants of the domain.
    ///
    /// This method deliberately performs no semantic lookup.
    ///
    /// In particular, it does not determine whether the domain:
    ///
    /// - exists;
    /// - is registered;
    /// - is supported;
    /// - is quantum;
    /// - is available on a target;
    /// - has sufficient hardware resources.
    pub fn validate(&self) -> Result<(), DomainError> {
        validate_component(&self.namespace, DomainComponent::Namespace)?;
        validate_component(&self.name, DomainComponent::Name)?;

        Ok(())
    }

    /// Returns whether this domain has the given namespace and name.
    ///
    /// This is a pure source-level comparison.
    #[must_use]
    pub fn is<N, M>(&self, namespace: N, name: M) -> bool
    where
        N: AsRef<str>,
        M: AsRef<str>,
    {
        self.namespace == namespace.as_ref() && self.name == name.as_ref()
    }
}

impl fmt::Display for Domain {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.namespace)?;
        formatter.write_str(".")?;
        formatter.write_str(&self.name)
    }
}

/// A well-formedness error for a source-level [`Domain`].
///
/// This error intentionally contains no target/backend information.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum DomainError {
    /// The namespace is empty.
    EmptyNamespace,

    /// The domain name is empty.
    EmptyName,

    /// A namespace/name component contains a control character.
    ControlCharacter {
        /// Component containing the invalid character.
        component: DomainComponent,

        /// Character rejected from the source-level identifier.
        character: char,
    },
}

impl fmt::Display for DomainError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyNamespace => {
                formatter.write_str("domain namespace must not be empty")
            }
            Self::EmptyName => {
                formatter.write_str("domain name must not be empty")
            }
            Self::ControlCharacter {
                component,
                character,
            } => {
                write!(
                    formatter,
                    "domain {} contains control character U+{:04X}",
                    component,
                    *character as u32
                )
            }
        }
    }
}

impl std::error::Error for DomainError {}

/// Identifies which logical component of a domain failed validation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DomainComponent {
    /// Namespace portion of the domain identity.
    Namespace,

    /// Name portion of the domain identity.
    Name,
}

impl fmt::Display for DomainComponent {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Namespace => formatter.write_str("namespace"),
            Self::Name => formatter.write_str("name"),
        }
    }
}

fn validate_component(
    value: &str,
    component: DomainComponent,
) -> Result<(), DomainError> {
    if value.is_empty() {
        return match component {
            DomainComponent::Namespace => Err(DomainError::EmptyNamespace),
            DomainComponent::Name => Err(DomainError::EmptyName),
        };
    }

    if let Some(character) = value.chars().find(char::is_control) {
        return Err(DomainError::ControlCharacter {
            component,
            character,
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructs_valid_domain() {
        let domain =
            Domain::new("zamani", "quantum").expect("domain must be valid");

        assert_eq!(domain.namespace(), "zamani");
        assert_eq!(domain.name(), "quantum");
        assert_eq!(domain.qualified_name(), "zamani.quantum");
    }

    #[test]
    fn supports_arbitrary_extension_domains() {
        let domain = Domain::new(
            "future.example",
            "photonic_computing",
        )
        .expect("future domain must be representable");

        assert_eq!(
            domain.qualified_name(),
            "future.example.photonic_computing"
        );
    }

    #[test]
    fn rejects_empty_namespace() {
        assert_eq!(
            Domain::new("", "quantum"),
            Err(DomainError::EmptyNamespace)
        );
    }

    #[test]
    fn rejects_empty_name() {
        assert_eq!(
            Domain::new("zamani", ""),
            Err(DomainError::EmptyName)
        );
    }

    #[test]
    fn rejects_namespace_control_character() {
        assert_eq!(
            Domain::new("zamani\nquantum", "domain"),
            Err(DomainError::ControlCharacter {
                component: DomainComponent::Namespace,
                character: '\n',
            })
        );
    }

    #[test]
    fn rejects_name_control_character() {
        assert_eq!(
            Domain::new("zamani", "quantum\t"),
            Err(DomainError::ControlCharacter {
                component: DomainComponent::Name,
                character: '\t',
            })
        );
    }

    #[test]
    fn validation_is_idempotent() {
        let domain =
            Domain::new("zamani", "quantum").expect("domain must be valid");

        assert_eq!(domain.validate(), Ok(()));
    }

    #[test]
    fn equality_is_logical() {
        let left =
            Domain::new("zamani", "quantum").expect("domain must be valid");

        let right =
            Domain::new("zamani", "quantum").expect("domain must be valid");

        assert_eq!(left, right);
    }

    #[test]
    fn different_namespace_is_different_domain() {
        let left =
            Domain::new("zamani", "quantum").expect("domain must be valid");

        let right =
            Domain::new("future", "quantum").expect("domain must be valid");

        assert_ne!(left, right);
    }

    #[test]
    fn is_performs_source_level_identity_check() {
        let domain =
            Domain::new("zamani", "quantum").expect("domain must be valid");

        assert!(domain.is("zamani", "quantum"));
        assert!(!domain.is("future", "quantum"));
    }

    #[test]
    fn display_is_qualified_identity() {
        let domain =
            Domain::new("zamani", "quantum").expect("domain must be valid");

        assert_eq!(domain.to_string(), "zamani.quantum");
    }

    #[test]
    fn component_count_supports_nested_namespaces() {
        let domain = Domain::new(
            "future.example.computing",
            "quantum",
        )
        .expect("domain must be valid");

        assert_eq!(domain.component_count(), 4);
    }

    #[test]
    fn supports_unicode_without_imposing_ascii_limits() {
        let domain =
            Domain::new("zamani.計算", "量子").expect("domain must be valid");

        assert_eq!(domain.namespace(), "zamani.計算");
        assert_eq!(domain.name(), "量子");
    }

    #[test]
    fn supports_large_source_identifiers() {
        let namespace = "n".repeat(16 * 1024);
        let name = "d".repeat(16 * 1024);

        let domain =
            Domain::new(namespace.clone(), name.clone())
                .expect("large domain identity must be representable");

        assert_eq!(domain.namespace(), namespace);
        assert_eq!(domain.name(), name);
    }

    #[test]
    fn no_machine_information_is_encoded() {
        let domain =
            Domain::new("zamani", "quantum").expect("domain must be valid");

        assert_eq!(domain.to_string(), "zamani.quantum");

        // The identity contains no qubit count, machine size, vendor,
        // topology, backend or physical-resource identifier.
    }
}
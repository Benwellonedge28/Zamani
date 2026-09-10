//! Resource-kind identity for the native Zamani AST.
//!
//! # Architectural role
//!
//! This module defines the source-level identity of a resource *kind*.
//!
//! A resource kind answers:
//!
//! > "What kind of computational resource does this declaration/reference
//! > describe?"
//!
//! It does **not** answer:
//!
//! - where the resource physically exists;
//! - how much hardware is available;
//! - which vendor supplies it;
//! - which topology it has;
//! - how it is routed;
//! - how it is scheduled;
//! - how it is calibrated;
//! - how it is error-corrected;
//! - which backend implements it;
//! - which target instruction set realizes it.
//!
//! Those concerns belong to later compiler stages.
//!
//! # POCO-REAF
//!
//! Resource kinds are intentionally represented as extensible, namespaced
//! identifiers rather than a closed Rust enum such as:
//!
//! ```text
//! enum ResourceKind {
//!     Qubit,
//!     Gpu,
//!     Cpu,
//!     ...
//! }
//! ```
//!
//! Such an enum would make the native AST depend on a finite list of today's
//! resource technologies and would require AST changes whenever a new resource
//! class is introduced.
//!
//! Instead, a resource kind consists of:
//!
//! ```text
//! namespace + name
//! ```
//!
//! An optional version is represented separately when an extension protocol
//! needs versioned resource-kind semantics.
//!
//! # Ownership
//!
//! This file owns:
//!
//! - the canonical source-level resource-kind identity;
//! - validation of that identity's structural invariants;
//! - deterministic ordering;
//! - stable textual representation;
//! - version-independent identity comparison;
//! - well-known *names* only as constants, where useful.
//!
//! This file does NOT own:
//!
//! - resource declarations;
//! - resource cardinality;
//! - resource lifetime;
//! - resource ownership;
//! - allocation;
//! - physical resources;
//! - hardware topology;
//! - capabilities;
//! - scheduling;
//! - routing;
//! - QEC;
//! - calibration;
//! - backend execution.
//!
//! # Dependency policy
//!
//! This module intentionally uses only the Rust standard library.
//!
//! It must not depend on:
//!
//! - quantum IR;
//! - ZUIR;
//! - QIR;
//! - MLIR;
//! - OpenQASM;
//! - hardware backends;
//! - schedulers;
//! - routers;
//! - QEC implementations;
//! - runtime execution;
//! - semantic analysis.
//!
//! This keeps the dependency direction:
//
//! ```text
//! source → AST → semantic model → ZUIR → domain/target IR
//! ```
//!
//! rather than introducing a reverse dependency from AST to later phases.
//!
//! # Rust
//!
//! Designed for Rust 1.97 / 1.97.1.
//!
//! No `unsafe` code is used.

use core::cmp::Ordering;
use core::fmt;
use core::str::FromStr;

/// Error returned when a [`ResourceKind`] cannot be constructed from an
/// invalid textual identity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ResourceKindError {
    /// The namespace was empty.
    EmptyNamespace,

    /// The resource-kind name was empty.
    EmptyName,

    /// The namespace contains a character that is not permitted.
    InvalidNamespaceCharacter {
        /// Zero-based character position.
        position: usize,

        /// The invalid character.
        character: char,
    },

    /// The resource-kind name contains a character that is not permitted.
    InvalidNameCharacter {
        /// Zero-based character position.
        position: usize,

        /// The invalid character.
        character: char,
    },

    /// A reserved separator was used in an individual component.
    ReservedSeparator {
        /// Component that contained the separator.
        component: ResourceKindComponent,

        /// Position of the separator.
        position: usize,
    },
}

/// Identifies which component of a resource-kind identity failed validation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ResourceKindComponent {
    /// Namespace component.
    Namespace,

    /// Name component.
    Name,
}

impl fmt::Display for ResourceKindError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyNamespace => formatter.write_str("resource kind namespace cannot be empty"),

            Self::EmptyName => formatter.write_str("resource kind name cannot be empty"),

            Self::InvalidNamespaceCharacter {
                position,
                character,
            } => write!(
                formatter,
                "invalid resource kind namespace character `{character}` at position {position}"
            ),

            Self::InvalidNameCharacter {
                position,
                character,
            } => write!(
                formatter,
                "invalid resource kind name character `{character}` at position {position}"
            ),

            Self::ReservedSeparator {
                component,
                position,
            } => write!(
                formatter,
                "reserved resource kind separator `:` is not permitted in {component:?} at position {position}"
            ),
        }
    }
}

impl std::error::Error for ResourceKindError {}

/// An extensible source-level identity for a kind of computational resource.
///
/// # Representation
///
/// A kind is represented as:
///
/// ```text
/// namespace:name
/// ```
///
/// Both components are owned strings. This is intentional at the AST
/// boundary: the AST must remain independent of identifier interning,
/// semantic symbol tables, or compiler-session lifetime.
///
/// Later compiler phases may intern or resolve this identity.
///
/// # Examples
///
/// ```
/// use zamani::frontend::ast::node::resources::resource_kind::ResourceKind;
///
/// let kind = ResourceKind::new("quantum", "logical-qubit").unwrap();
///
/// assert_eq!(kind.namespace(), "quantum");
/// assert_eq!(kind.name(), "logical-qubit");
/// assert_eq!(kind.as_str(), "quantum:logical-qubit");
/// ```
///
/// The example above assumes the crate is named `zamani`; the type itself has
/// no dependency on the crate layout.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ResourceKind {
    namespace: String,
    name: String,
}

impl ResourceKind {
    /// The separator used by the canonical textual representation.
    pub const SEPARATOR: char = ':';

    /// Creates a new resource-kind identity.
    ///
    /// This performs only **structural** validation.
    ///
    /// It deliberately does not validate whether a resource kind is known to
    /// the compiler. Unknown kinds are valid at the AST boundary because
    /// extensions and future language domains must be representable without
    /// changing the core AST.
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> Result<Self, ResourceKindError> {
        let namespace = namespace.into();
        let name = name.into();

        validate_component(
            &namespace,
            ResourceKindComponent::Namespace,
            ResourceKindError::EmptyNamespace,
        )?;

        validate_component(
            &name,
            ResourceKindComponent::Name,
            ResourceKindError::EmptyName,
        )?;

        Ok(Self { namespace, name })
    }

    /// Returns the namespace.
    #[inline]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Returns the resource-kind name.
    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the canonical textual identity.
    ///
    /// The returned value is allocated because the AST owns its components
    /// independently.
    pub fn as_string(&self) -> String {
        let mut value = String::with_capacity(
            self.namespace
                .len()
                .saturating_add(1)
                .saturating_add(self.name.len()),
        );

        value.push_str(&self.namespace);
        value.push(Self::SEPARATOR);
        value.push_str(&self.name);

        value
    }

    /// Returns the canonical textual identity as a borrowed representation
    /// where possible.
    ///
    /// This method currently returns the same owned representation as
    /// [`Self::as_string`]. Keeping this API separate allows the internal
    /// representation to evolve without changing consumers.
    pub fn as_str(&self) -> String {
        self.as_string()
    }

    /// Returns whether this identity belongs to the supplied namespace.
    #[inline]
    pub fn is_in_namespace(&self, namespace: &str) -> bool {
        self.namespace == namespace
    }

    /// Returns whether the identity has the supplied namespace and name.
    #[inline]
    pub fn is(&self, namespace: &str, name: &str) -> bool {
        self.namespace == namespace && self.name == name
    }

    /// Creates a resource kind using a canonical namespace/name pair without
    /// allowing a separator to appear in either component.
    ///
    /// This is equivalent to [`Self::new`] and exists as an explicit API for
    /// code that constructs kinds from extension registries.
    pub fn from_components(
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> Result<Self, ResourceKindError> {
        Self::new(namespace, name)
    }

    /// Parses a canonical `namespace:name` identity.
    ///
    /// Exactly one separator is required at the boundary between namespace
    /// and name. Additional separators are rejected because individual
    /// components must remain unambiguous.
    pub fn parse(value: &str) -> Result<Self, ResourceKindError> {
        let separator = value.find(Self::SEPARATOR);

        let Some(separator) = separator else {
            return Err(ResourceKindError::EmptyName);
        };

        let namespace = &value[..separator];
        let name = &value[separator + Self::SEPARATOR.len_utf8()..];

        Self::new(namespace, name)
    }

    /// Returns the canonical identity of a generic computational resource.
    ///
    /// This is deliberately not named `CPU`, `Qubit`, `GPU`, etc. The core AST
    /// must not prescribe the finite universe of resource technologies.
    pub const GENERIC: &'static str = "core:resource";

    /// Returns the canonical identity of the core memory resource class.
    ///
    /// This is only a stable *language-level identity*. It does not describe
    /// RAM, VRAM, cache, HBM, NUMA, or any physical memory technology.
    pub const MEMORY: &'static str = "core:memory";

    /// Returns the canonical identity of the generic compute resource class.
    pub const COMPUTE: &'static str = "core:compute";

    /// Returns the canonical identity of the generic quantum resource class.
    ///
    /// It intentionally does not identify a physical qubit technology.
    pub const QUANTUM: &'static str = "quantum:resource";

    /// Returns the canonical identity of a logical quantum resource.
    ///
    /// The physical implementation remains outside the AST.
    pub const LOGICAL_QUANTUM: &'static str = "quantum:logical-resource";

    /// Returns the canonical identity of a physical-resource reference.
    ///
    /// This identifies the *language concept*, not a vendor or topology.
    pub const PHYSICAL_REFERENCE: &'static str = "core:physical-reference";

    /// Constructs one of the compiler-defined well-known kinds.
    ///
    /// Unknown extension kinds must continue to be constructed through
    /// [`Self::new`] or [`Self::parse`].
    pub fn well_known(identity: &str) -> Result<Self, ResourceKindError> {
        Self::parse(identity)
    }

    /// Returns whether this kind is one of the core language identities.
    ///
    /// This check is intentionally small and does not define the complete
    /// universe of valid resource kinds.
    pub fn is_core_kind(&self) -> bool {
        self.namespace == "core"
    }

    /// Returns whether this kind belongs to the quantum namespace.
    ///
    /// This is a namespace query only. It must not trigger quantum compilation,
    /// hardware selection, routing, scheduling, or QEC.
    pub fn is_quantum_kind(&self) -> bool {
        self.namespace == "quantum"
    }

    /// Returns whether this kind is an extension-defined kind.
    ///
    /// Extension status is determined structurally here. Actual extension
    /// registration and schema validation belong to the extension registry.
    pub fn is_extension_kind(&self) -> bool {
        !self.is_core_kind() && !self.is_quantum_kind()
    }

    /// Validates the identity again.
    ///
    /// This is useful at trust boundaries such as deserialization and
    /// extension loading.
    pub fn validate(&self) -> Result<(), ResourceKindError> {
        validate_component(
            &self.namespace,
            ResourceKindComponent::Namespace,
            ResourceKindError::EmptyNamespace,
        )?;

        validate_component(
            &self.name,
            ResourceKindComponent::Name,
            ResourceKindError::EmptyName,
        )?;

        Ok(())
    }
}

impl fmt::Display for ResourceKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.namespace)?;
        formatter.write_str(":")?;
        formatter.write_str(&self.name)
    }
}

impl FromStr for ResourceKind {
    type Err = ResourceKindError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

impl Ord for ResourceKind {
    fn cmp(&self, other: &Self) -> Ordering {
        self.namespace
            .cmp(&other.namespace)
            .then_with(|| self.name.cmp(&other.name))
    }
}

impl PartialOrd for ResourceKind {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Validates one identity component.
///
/// The syntax is intentionally conservative and deterministic:
///
/// - Unicode alphabetic characters are accepted;
/// - Unicode numeric characters are accepted after the first character;
/// - `_`, `-`, `.`, and `/` are accepted;
/// - `:` is reserved for the namespace/name separator;
/// - whitespace is rejected;
/// - control characters are rejected.
///
/// This gives extension authors enough namespace freedom without allowing
/// ambiguous canonical identities.
///
/// The first character is allowed to be numeric because resource-kind names
/// are identifiers rather than Rust identifiers. Semantic/language-level
/// identifier restrictions belong to the identifier/path subsystem.
fn validate_component(
    value: &str,
    component: ResourceKindComponent,
    empty_error: ResourceKindError,
) -> Result<(), ResourceKindError> {
    if value.is_empty() {
        return Err(empty_error);
    }

    for (position, character) in value.chars().enumerate() {
        if character == ResourceKind::SEPARATOR {
            return Err(ResourceKindError::ReservedSeparator {
                component,
                position,
            });
        }

        let allowed = character.is_alphanumeric()
            || matches!(character, '_' | '-' | '.' | '/');

        if !allowed {
            return Err(match component {
                ResourceKindComponent::Namespace => {
                    ResourceKindError::InvalidNamespaceCharacter {
                        position,
                        character,
                    }
                }

                ResourceKindComponent::Name => ResourceKindError::InvalidNameCharacter {
                    position,
                    character,
                },
            });
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_valid_kind() {
        let kind = ResourceKind::new("quantum", "logical-qubit").unwrap();

        assert_eq!(kind.namespace(), "quantum");
        assert_eq!(kind.name(), "logical-qubit");
        assert_eq!(kind.to_string(), "quantum:logical-qubit");
    }

    #[test]
    fn parses_canonical_identity() {
        let kind = ResourceKind::parse("quantum:logical-qubit").unwrap();

        assert!(kind.is("quantum", "logical-qubit"));
        assert!(kind.is_quantum_kind());
    }

    #[test]
    fn rejects_missing_separator() {
        assert_eq!(
            ResourceKind::parse("quantum"),
            Err(ResourceKindError::EmptyName)
        );
    }

    #[test]
    fn rejects_empty_namespace() {
        assert_eq!(
            ResourceKind::new("", "qubit"),
            Err(ResourceKindError::EmptyNamespace)
        );
    }

    #[test]
    fn rejects_empty_name() {
        assert_eq!(
            ResourceKind::new("quantum", ""),
            Err(ResourceKindError::EmptyName)
        );
    }

    #[test]
    fn rejects_separator_inside_component() {
        assert_eq!(
            ResourceKind::new("quantum:future", "qubit"),
            Err(ResourceKindError::ReservedSeparator {
                component: ResourceKindComponent::Namespace,
                position: 7,
            })
        );
    }

    #[test]
    fn supports_unicode_components() {
        let kind = ResourceKind::new("未来", "量子资源").unwrap();

        assert_eq!(kind.namespace(), "未来");
        assert_eq!(kind.name(), "量子资源");
    }

    #[test]
    fn supports_extension_kinds_without_core_changes() {
        let kind = ResourceKind::new("future.neuro", "synaptic-array").unwrap();

        assert!(kind.is_extension_kind());
        assert!(!kind.is_core_kind());
        assert!(!kind.is_quantum_kind());
    }

    #[test]
    fn supports_nested_namespace_names() {
        let kind = ResourceKind::new("quantum/fault-tolerance", "logical-resource").unwrap();

        assert_eq!(
            kind.to_string(),
            "quantum/fault-tolerance:logical-resource"
        );
    }

    #[test]
    fn ordering_is_deterministic() {
        let mut kinds = vec![
            ResourceKind::new("quantum", "resource").unwrap(),
            ResourceKind::new("core", "memory").unwrap(),
            ResourceKind::new("core", "compute").unwrap(),
        ];

        kinds.sort();

        assert_eq!(
            kinds[0].to_string(),
            "core:compute"
        );
        assert_eq!(
            kinds[1].to_string(),
            "core:memory"
        );
        assert_eq!(
            kinds[2].to_string(),
            "quantum:resource"
        );
    }

    #[test]
    fn round_trip_is_stable() {
        let original = ResourceKind::new(
            "future/distributed",
            "quantum-resource",
        )
        .unwrap();

        let serialized = original.to_string();
        let restored = ResourceKind::parse(&serialized).unwrap();

        assert_eq!(original, restored);
    }

    #[test]
    fn validation_is_repeatable() {
        let kind = ResourceKind::new("quantum", "logical-qubit").unwrap();

        assert!(kind.validate().is_ok());
        assert!(kind.validate().is_ok());
    }

    #[test]
    fn well_known_identity_is_parseable() {
        let kind = ResourceKind::well_known(ResourceKind::QUANTUM).unwrap();

        assert_eq!(kind.namespace(), "quantum");
        assert_eq!(kind.name(), "resource");
    }

    #[test]
    fn physical_reference_is_not_a_backend() {
        let kind = ResourceKind::well_known(ResourceKind::PHYSICAL_REFERENCE).unwrap();

        assert_eq!(kind.namespace(), "core");
        assert_eq!(kind.name(), "physical-reference");
    }
}
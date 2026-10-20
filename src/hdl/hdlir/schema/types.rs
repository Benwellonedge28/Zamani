
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # HDLIR Type Schema Contracts
//!
//! Path: `src/hdl/hdlir/schema/types.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//! Status: Foundational schema API; freeze after integration tests pass.
//!
//! ## Purpose
//!
//! Define versioned, extensible declarations of hardware-domain types.
//! A declaration identifies a type contract and describes its parameters,
//! semantic category, and compatibility policy. It is not itself a concrete
//! signal, constant, memory, or physical resource.
//!
//! ## Authority
//!
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4`
//! - `grammar/Zamani.g4`
//! - `grammar/validation/semantic-boundaries.md`
//! - `src/hdl/hdlir/HDLIR-SPEC.md`
//! - `src/hdl/hdlir/compatibility.md`
//! - `src/hdl/hdlir/errors.rs`
//! - `src/hdl/hdlir/version.rs`
//! - `src/zuir/ZUIR-SPEC.md`
//!
//! The grammar owns syntax; the frontend owns source-level semantic truth;
//! ZUIR owns the universal computational representation; HDLIR owns
//! hardware-domain type contracts.
//!
//! ## Ownership
//!
//! This module owns:
//! - The canonical registry-facing `TypeDefinition`.
//! - Type parameter declarations and their validated names.
//! - The declared semantic category of a type.
//! - The declared type-compatibility policy.
//!
//! It does not own concrete bit-width arithmetic, logic evaluation,
//! signal connectivity, physical layout, device allocation, or target
//! feasibility. Those belong to their designated HDLIR and downstream
//! components.
//!
//! ## Scalability
//!
//! No fixed limit is imposed on the number of registered types or parameters.
//! Widths, dimensions, and resource quantities are not encoded as machine
//! integers here. Concrete values and symbolic expressions belong to the
//! corresponding type/value contracts.
//!
//! Collection sizes remain subject to available memory and address space.
//! Callers processing untrusted schema data must apply their configured
//! operational budgets before or during decoding and registration.
//!
//! ## Integration
//!
//! `schema/registry.rs` stores `TypeDefinition` values and remains responsible
//! for duplicate detection, dialect existence, extension policy, and atomic
//! registration. It should publicly re-export this type for compatibility.
//!
//! `types/` defines concrete hardware-domain type representations.
//! `verify/types.rs` validates actual type uses and compatibility.
//! Serialization must persist all declared fields and validate decoded
//! values through the same constructors and registry contracts.
//!
//! ## Freeze criteria
//!
//! - Invalid names and duplicate parameter names are rejected.
//! - Existing `TypeDefinition::new` callers remain source-compatible.
//! - Type categories and compatibility policies are explicit.
//! - No hardware-capacity limit is introduced.
//! - Tests cover malformed definitions, parameters, equality, and extensions.

use std::collections::BTreeSet;

use crate::hdl::hdlir::errors::{HdlirError, HdlirResult};
use crate::hdl::hdlir::version::HdlirVersion;

use super::registry::{DefinitionStability, QualifiedName};

/// The declared semantic category of a registered type.
///
/// This is descriptive schema metadata. Selecting a category does not imply
/// that the corresponding concrete type implementation or target lowering
/// already exists.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TypeSemantics {
    /// The type has no built-in interpretation in the HDLIR core.
    ///
    /// Its meaning must be supplied by a registered extension contract or
    /// another explicitly defined consumer contract.
    Opaque,

    /// A bit-vector-like type whose width semantics are defined elsewhere.
    BitVector,

    /// An aggregate type such as a record, tuple, or array.
    Aggregate,

    /// A logical hardware signal type.
    Signal,

    /// A logical clock or reset-related type.
    Clock,

    /// An abstract memory type.
    Memory,

    /// A type whose semantics are defined by another registered contract.
    Extension {
        /// Qualified name of the contract defining this type's semantics.
        contract: QualifiedName,
    },
}

/// Declared type-compatibility policy.
///
/// This policy is a schema-level declaration, not a substitute for concrete
/// compatibility checking by the HDLIR type verifier.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TypeCompatibility {
    /// Type identity is significant; equal structure alone is insufficient.
    Nominal,

    /// Structural compatibility may be accepted when the verifier proves
    /// that all relevant structure and semantic attributes are compatible.
    ///
    /// This policy does not permit ignoring width, signedness, state,
    /// timing, or other semantically significant properties.
    Structural,
}

/// The kind of value accepted for a type parameter.
///
/// The concrete representation and evaluation rules for parameter values
/// are defined by the relevant type/value subsystem, not by this enum.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TypeParameterKind {
    /// A type-level width or width expression.
    BitWidth,

    /// An integer-valued compile-time parameter.
    Integer,

    /// A boolean-valued compile-time parameter.
    Boolean,

    /// A symbolic identifier resolved by an owning semantic context.
    Identifier,

    /// A reference to another registered type.
    Type,

    /// A symbolic compile-time expression.
    Expression,
}

/// A validated declaration of one type parameter.
///
/// Parameter names are local to their containing type declaration.
/// They are not source-level variable names and do not perform name
/// resolution. The `required` flag indicates whether a type instantiation
/// must provide a binding; defaults and constraint evaluation are owned by
/// the parameter/value subsystem.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TypeParameter {
    name: String,
    kind: TypeParameterKind,
    required: bool,
}

impl TypeParameter {
    /// Create a type parameter declaration.
    ///
    /// Names use a canonical ASCII identifier form:
    /// the first character must be an ASCII letter or underscore, and
    /// remaining characters must be ASCII letters, digits, or underscores.
    ///
    /// # Errors
    ///
    /// Returns an HDLIR input error if the name is empty or invalid.
    pub fn new(
        name: impl Into<String>,
        kind: TypeParameterKind,
        required: bool,
    ) -> HdlirResult<Self> {
        let name = name.into();

        if !Self::is_valid_identifier(&name) {
            return Err(HdlirError::invalid_input(
                "type_parameter.name",
                format!("`{name}` is not a valid type parameter identifier"),
            ));
        }

        Ok(Self {
            name,
            kind,
            required,
        })
    }

    /// Return the parameter's canonical local name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the required value category.
    #[must_use]
    pub const fn kind(&self) -> TypeParameterKind {
        self.kind
    }

    /// Return whether an instantiation must bind this parameter explicitly.
    #[must_use]
    pub const fn is_required(&self) -> bool {
        self.required
    }

    fn is_valid_identifier(value: &str) -> bool {
        let mut bytes = value.bytes();

        let Some(first) = bytes.next() else {
            return false;
        };

        if !(first.is_ascii_alphabetic() || first == b'_') {
            return false;
        }

        bytes.all(|byte| {
            byte.is_ascii_alphanumeric() || byte == b'_'
        })
    }
}

/// A versioned declaration of a hardware-domain type.
///
/// `TypeDefinition` declares a type's identity and schema contract. It does
/// not represent a concrete instance of that type.
///
/// The constructor intentionally retains the registry's original argument
/// shape. Additional properties can be supplied through validated builder
/// methods so existing call sites need not be rewritten merely to adopt
/// this more complete schema contract.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TypeDefinition {
    name: QualifiedName,
    dialect: QualifiedName,
    version: HdlirVersion,
    stability: DefinitionStability,
    description: String,
    semantics: TypeSemantics,
    compatibility: TypeCompatibility,
    parameters: Vec<TypeParameter>,
}

impl TypeDefinition {
    /// Create a type definition with default schema metadata.
    ///
    /// Defaults:
    /// - semantics: [`TypeSemantics::Opaque`]
    /// - compatibility: [`TypeCompatibility::Nominal`]
    /// - parameters: none
    ///
    /// The type name must be directly namespaced by the supplied dialect.
    ///
    /// # Errors
    ///
    /// Returns an HDLIR input error if the type name is not directly
    /// namespaced by the dialect or if the description is empty.
    pub fn new(
        name: QualifiedName,
        dialect: QualifiedName,
        version: HdlirVersion,
        stability: DefinitionStability,
        description: impl Into<String>,
    ) -> HdlirResult<Self> {
        if name.namespace() != dialect.as_str() {
            return Err(HdlirError::invalid_input(
                "type.dialect",
                format!(
                    "type `{name}` must be directly namespaced by dialect `{dialect}`"
                ),
            ));
        }

        let description = description.into();

        if description.trim().is_empty() {
            return Err(HdlirError::invalid_input(
                "type.description",
                "description must not be empty",
            ));
        }

        Ok(Self {
            name,
            dialect,
            version,
            stability,
            description,
            semantics: TypeSemantics::Opaque,
            compatibility: TypeCompatibility::Nominal,
            parameters: Vec::new(),
        })
    }

    /// Set the declared semantic category.
    ///
    /// The category is metadata; the verifier must still establish that
    /// concrete uses satisfy the corresponding type contract.
    #[must_use]
    pub fn with_semantics(mut self, semantics: TypeSemantics) -> Self {
        self.semantics = semantics;
        self
    }

    /// Set the declared type-compatibility policy.
    ///
    /// This does not itself prove compatibility. It tells the verifier
    /// which compatibility rule applies when checking type uses.
    #[must_use]
    pub fn with_compatibility(
        mut self,
        compatibility: TypeCompatibility,
    ) -> Self {
        self.compatibility = compatibility;
        self
    }

    /// Attach the complete ordered parameter declaration list.
    ///
    /// Parameter order is preserved because it can be meaningful to
    /// serialized artifacts and instantiation APIs. Duplicate parameter
    /// names are rejected. No schema-level parameter-count ceiling exists.
    ///
    /// # Errors
    ///
    /// Returns an HDLIR input error if two parameters have the same name.
    pub fn with_parameters(
        mut self,
        parameters: impl IntoIterator<Item = TypeParameter>,
    ) -> HdlirResult<Self> {
        let mut names = BTreeSet::new();
        let mut validated = Vec::new();

        for parameter in parameters {
            if !names.insert(parameter.name().to_owned()) {
                return Err(HdlirError::invalid_input(
                    "type.parameters",
                    format!(
                        "duplicate parameter `{}` in type `{}`",
                        parameter.name(),
                        self.name
                    ),
                ));
            }

            validated.push(parameter);
        }

        self.parameters = validated;
        Ok(self)
    }

    /// Return the qualified type name.
    #[must_use]
    pub fn name(&self) -> &QualifiedName {
        &self.name
    }

    /// Return the owning dialect name.
    #[must_use]
    pub fn dialect(&self) -> &QualifiedName {
        &self.dialect
    }

    /// Return the schema version.
    #[must_use]
    pub const fn version(&self) -> HdlirVersion {
        self.version
    }

    /// Return the stability classification.
    #[must_use]
    pub const fn stability(&self) -> DefinitionStability {
        self.stability
    }

    /// Return the human-readable description.
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Return the declared semantic category.
    #[must_use]
    pub fn semantics(&self) -> &TypeSemantics {
        &self.semantics
    }

    /// Return the declared compatibility policy.
    #[must_use]
    pub const fn compatibility(&self) -> TypeCompatibility {
        self.compatibility
    }

    /// Return the ordered type parameter declarations.
    #[must_use]
    pub fn parameters(&self) -> &[TypeParameter] {
        &self.parameters
    }

    /// Find a parameter by its exact canonical name.
    #[must_use]
    pub fn parameter(&self, name: &str) -> Option<&TypeParameter> {
        self.parameters
            .iter()
            .find(|parameter| parameter.name() == name)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        TypeCompatibility, TypeDefinition, TypeParameter, TypeParameterKind,
        TypeSemantics,
    };
    use crate::hdl::hdlir::version::HdlirVersion;
    use crate::hdl::hdlir::schema::registry::{
        DefinitionStability, QualifiedName,
    };

    fn name(value: &str) -> QualifiedName {
        QualifiedName::parse(value)
            .expect("test qualified name must be valid")
    }

    fn type_definition() -> TypeDefinition {
        TypeDefinition::new(
            name("structural.word"),
            name("structural"),
            HdlirVersion::new(1, 0, 0),
            DefinitionStability::Stable,
            "A parameterized logical word type",
        )
        .expect("test type definition must be valid")
    }

    #[test]
    fn constructor_preserves_registry_facing_defaults() {
        let definition = type_definition();

        assert_eq!(definition.name().as_str(), "structural.word");
        assert_eq!(definition.dialect().as_str(), "structural");
        assert_eq!(*definition.semantics(), TypeSemantics::Opaque);
        assert_eq!(
            definition.compatibility(),
            TypeCompatibility::Nominal
        );
        assert!(definition.parameters().is_empty());
    }

    #[test]
    fn type_must_be_directly_namespaced_by_its_dialect() {
        let result = TypeDefinition::new(
            name("structural.numeric.word"),
            name("structural"),
            HdlirVersion::new(1, 0, 0),
            DefinitionStability::Stable,
            "Invalid direct namespace",
        );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_empty_descriptions() {
        let result = TypeDefinition::new(
            name("structural.word"),
            name("structural"),
            HdlirVersion::new(1, 0, 0),
            DefinitionStability::Stable,
            "   ",
        );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_invalid_parameter_names() {
        assert!(
            TypeParameter::new("2width", TypeParameterKind::BitWidth, true)
                .is_err()
        );
        assert!(
            TypeParameter::new("", TypeParameterKind::Integer, true).is_err()
        );
        assert!(
            TypeParameter::new("data-width", TypeParameterKind::BitWidth, true)
                .is_err()
        );
    }

    #[test]
    fn accepts_and_retrieves_symbolic_parameters() {
        let width = TypeParameter::new(
            "WIDTH",
            TypeParameterKind::BitWidth,
            true,
        )
        .expect("valid parameter");

        let definition = type_definition()
            .with_semantics(TypeSemantics::BitVector)
            .with_compatibility(TypeCompatibility::Structural)
            .with_parameters([width])
            .expect("valid parameterized type");

        assert_eq!(definition.semantics(), &TypeSemantics::BitVector);
        assert_eq!(
            definition.compatibility(),
            TypeCompatibility::Structural
        );
        assert_eq!(
            definition.parameter("WIDTH").map(TypeParameter::kind),
            Some(TypeParameterKind::BitWidth)
        );
        assert!(definition.parameter("width").is_none());
    }

    #[test]
    fn rejects_duplicate_parameter_names() {
        let first = TypeParameter::new(
            "WIDTH",
            TypeParameterKind::BitWidth,
            true,
        )
        .expect("valid parameter");
        let second = TypeParameter::new(
            "WIDTH",
            TypeParameterKind::Expression,
            false,
        )
        .expect("valid parameter");

        assert!(
            type_definition()
                .with_parameters([first, second])
                .is_err()
        );
    }

    #[test]
    fn extension_semantics_are_explicit() {
        let definition = type_definition().with_semantics(
            TypeSemantics::Extension {
                contract: name("vendorx.analog_value"),
            },
        );

        assert_eq!(
            definition.semantics(),
            &TypeSemantics::Extension {
                contract: name("vendorx.analog_value"),
            }
        );
    }

    #[test]
    fn equality_includes_schema_metadata() {
        let original = type_definition();
        let changed = type_definition()
            .with_semantics(TypeSemantics::BitVector);

        assert_ne!(original, changed);
    }
}

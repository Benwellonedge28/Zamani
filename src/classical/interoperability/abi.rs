
#![forbid(unsafe_code)]
#![deny(unused_must_use)]

//! Zamani Classical Computing — Application Binary Interface Contracts
//!
//! Path: src/classical/interoperability/abi.rs
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Status
//!
//! Contract-first implementation. Full repository integration and build
//! verification must be completed before this module is declared verified.
//!
//! # Purpose
//!
//! Defines a target-independent, declarative model of ABI compatibility
//! between Zamani declarations and externally implemented interfaces.
//!
//! This module records what an ABI boundary requires. It does not perform
//! native calls, load libraries, resolve symbols, allocate native memory,
//! generate machine code, or choose hardware.
//!
//! # Ownership
//!
//! This module owns:
//! - ABI contract, profile, and function-signature descriptions;
//! - ABI parameter and result boundary metadata;
//! - extensible calling-convention and linkage identifiers;
//! - ABI contract validation and ABI-specific validation errors;
//! - explicit compatibility/version declarations;
//! - deterministic validation of the structures defined here.
//!
//! It does not own:
//! - source syntax, parsing, ASTs, or semantic name resolution;
//! - general Zamani types or canonical IR values;
//! - external-call execution or external-call IR;
//! - platform ABI implementation details or physical layouts;
//! - native memory allocation, pointers, or machine registers;
//! - dynamic library loading, linking, or symbol resolution;
//! - authorization, scheduling, resource placement, or execution.
//!
//! # Authority and integration
//!
//! - `grammar/interoperability/abi.g4` owns ABI source syntax.
//! - The frontend AST owns source structure and source spans.
//! - Semantic analysis resolves source names and checks type/effect rules.
//! - `crate::classical::error` owns the shared classical error vocabulary.
//! - `crate::quantum::ir::classical::extern_call` remains the canonical
//!   semantic IR boundary for external calls.
//! - Target lowering, linker, and runtime subsystems own concrete ABI
//!   realization, symbol resolution, and execution.
//!
//! This file deliberately does not define a second external-call IR.
//!
//! # Scalability and POCO-REAF
//!
//! The model imposes no universal limit on the number of functions,
//! parameters, results, metadata entries, or ABI profiles. Collections grow
//! subject to available memory and explicit operational policies.
//!
//! Rust `usize`, allocation capacity, external ABI specifications, and target
//! constraints are finite implementation realities, not language semantics.
//! Allocation failure must not be interpreted as source-level invalidity.
//!
//! Calling conventions, linkage kinds, representation hints, and extension
//! keys are open identifiers rather than closed enums of today's platforms.
//! A target-specific implementation may reject an unsupported identifier,
//! but must not silently substitute a different ABI.
//!
//! # Freeze rule
//!
//! Before implementation status becomes `Verified`, the repository must
//! establish this module's exports in `interoperability/mod.rs`, map the
//! frontend ABI AST into these contracts, and add integration tests against
//! the canonical external-call IR. Changes to public fields or validation
//! semantics require compatibility review.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::classical::error::{ClassicalError, ClassicalResult};

/// Result type for ABI contract operations.
pub type AbiResult<T> = Result<T, AbiError>;

/// ABI-specific validation failure.
///
/// This type carries stable error categories and contextual field information.
/// Convert it to the repository-wide `ClassicalError` at subsystem boundaries.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbiError {
    kind: AbiErrorKind,
    subject: String,
    message: String,
}

impl AbiError {
    fn new(
        kind: AbiErrorKind,
        subject: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            subject: subject.into(),
            message: message.into(),
        }
    }

    /// Returns the stable machine-readable error category.
    #[must_use]
    pub const fn kind(&self) -> AbiErrorKind {
        self.kind
    }

    /// Returns the subject that failed validation.
    #[must_use]
    pub fn subject(&self) -> &str {
        &self.subject
    }

    /// Returns the explanatory error message.
    ///
    /// Callers must use `kind()` for programmatic decisions rather than
    /// parsing this message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Converts the ABI-specific failure to the shared classical error model.
    #[must_use]
    pub fn into_classical_error(self) -> ClassicalError {
        ClassicalError::validation_failed(format!(
            "{}: {} ({})",
            self.subject,
            self.message,
            self.kind.as_str()
        ))
    }
}

impl fmt::Display for AbiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {} ({})",
            self.subject,
            self.message,
            self.kind.as_str()
        )
    }
}

impl std::error::Error for AbiError {}

/// Stable categories for ABI contract validation errors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AbiErrorKind {
    /// Required identifier is empty or contains a forbidden NUL character.
    InvalidIdentifier,
    /// Required ABI declaration is missing.
    MissingDeclaration,
    /// A name is declared more than once in a scope.
    DuplicateDeclaration,
    /// The contract contains an invalid or inconsistent field.
    InvalidContract,
    /// A function signature violates an ABI boundary invariant.
    InvalidSignature,
    /// A version or compatibility declaration is malformed.
    InvalidVersion,
    /// A required feature is not supported by a particular target.
    UnsupportedTargetFeature,
}

impl AbiErrorKind {
    /// Returns the stable machine-readable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InvalidIdentifier => "abi.invalid_identifier",
            Self::MissingDeclaration => "abi.missing_declaration",
            Self::DuplicateDeclaration => "abi.duplicate_declaration",
            Self::InvalidContract => "abi.invalid_contract",
            Self::InvalidSignature => "abi.invalid_signature",
            Self::InvalidVersion => "abi.invalid_version",
            Self::UnsupportedTargetFeature => "abi.unsupported_target_feature",
        }
    }
}

/// An ABI-facing identifier.
///
/// Names remain strings because external symbol spelling, namespace
/// conventions, and extension identifiers are not a closed language enum.
/// This type rejects empty names and embedded NUL characters but does not
/// impose a platform-specific character set.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AbiIdentifier(String);

impl AbiIdentifier {
    /// Creates an identifier after validating its basic representation.
    pub fn new(value: impl Into<String>) -> AbiResult<Self> {
        let value = value.into();

        if value.trim().is_empty() || value.contains('\0') {
            return Err(AbiError::new(
                AbiErrorKind::InvalidIdentifier,
                "identifier",
                "must be non-empty and must not contain NUL",
            ));
        }

        Ok(Self(value))
    }

    /// Returns the identifier text without modifying its spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the identifier and returns its owned string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for AbiIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// An extensible version identifier.
///
/// ABI ecosystems use different version schemes. The ABI contract therefore
/// preserves the version text instead of assuming every ecosystem uses
/// semantic versioning.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AbiVersion(AbiIdentifier);

impl AbiVersion {
    /// Creates a non-empty version identifier.
    pub fn new(value: impl Into<String>) -> AbiResult<Self> {
        AbiIdentifier::new(value)
            .map(Self)
            .map_err(|error| {
                AbiError::new(
                    AbiErrorKind::InvalidVersion,
                    "version",
                    error.message,
                )
            })
    }

    /// Returns the original version identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

/// Describes whether a boundary is callable or data-oriented.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AbiBoundaryKind {
    /// A callable function or procedure.
    Callable,
    /// A foreign data symbol or externally defined object.
    Data,
}

/// Declares symbol linkage intent.
///
/// This is semantic intent, not a request to perform linking.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AbiLinkage {
    /// The symbol is resolved through a statically linked artifact.
    Static,
    /// The symbol is expected from a dynamically linked artifact.
    Dynamic,
    /// The symbol is supplied by the host or embedding environment.
    Host,
    /// The symbol is supplied by another named module or provider.
    Provider(AbiIdentifier),
    /// An open-world linkage identifier defined by an extension.
    Custom(AbiIdentifier),
}

/// Describes ownership transfer across a boundary.
///
/// These are contract declarations. They do not implement ownership,
/// borrowing, reference counting, or native memory reclamation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AbiOwnership {
    /// The caller retains ownership.
    Borrowed,
    /// Ownership is transferred to the callee.
    TransferredToCallee,
    /// Ownership is transferred to the caller.
    TransferredToCaller,
    /// Ownership is shared under an independently specified contract.
    Shared,
    /// Ownership is governed by an extension-specific rule.
    ExternallyDefined,
}

/// Describes a boundary value's lifetime contract.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AbiLifetime {
    /// The value is valid only for the duration of the call.
    Call,
    /// The value is valid for the enclosing invocation.
    Invocation,
    /// The value remains valid for a caller-declared scope.
    Scope(AbiIdentifier),
    /// Lifetime is governed by an external specification.
    ExternallyDefined(AbiIdentifier),
}

/// Describes nullability independently of the underlying Zamani type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AbiNullability {
    /// Null is not a valid boundary value.
    NonNull,
    /// Null is an allowed boundary value.
    Nullable,
    /// Nullability is not specified by this contract.
    Unspecified,
}

/// Describes how a value crosses an ABI boundary.
///
/// These modes express intent, not concrete registers, stack offsets,
/// pointers, or target-specific layouts.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AbiPassingMode {
    /// Pass the value according to the selected ABI's default rules.
    Default,
    /// Pass a value using a by-value contract.
    ByValue,
    /// Pass a read-only borrowed value.
    Borrowed,
    /// Pass a mutable borrowed value.
    BorrowedMutable,
    /// Pass an opaque handle interpreted by the foreign implementation.
    OpaqueHandle,
    /// Pass according to an extension-specific rule.
    Custom(AbiIdentifier),
}

/// A reference to a semantically resolved type at the ABI boundary.
///
/// `type_name` is an external or semantic type identifier. It is not a
/// physical layout. Resolution and compatibility with the Zamani type system
/// belong to semantic analysis and target lowering.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AbiTypeRef {
    type_name: AbiIdentifier,
    passing: AbiPassingMode,
    ownership: AbiOwnership,
    lifetime: AbiLifetime,
    nullability: AbiNullability,
    representation_hint: Option<AbiIdentifier>,
}

impl AbiTypeRef {
    /// Creates a type reference with explicit boundary semantics.
    pub fn new(type_name: impl Into<String>) -> AbiResult<Self> {
        Ok(Self {
            type_name: AbiIdentifier::new(type_name)?,
            passing: AbiPassingMode::Default,
            ownership: AbiOwnership::Borrowed,
            lifetime: AbiLifetime::Call,
            nullability: AbiNullability::Unspecified,
            representation_hint: None,
        })
    }

    /// Sets the boundary passing mode.
    #[must_use]
    pub fn with_passing(mut self, passing: AbiPassingMode) -> Self {
        self.passing = passing;
        self
    }

    /// Sets the ownership contract.
    #[must_use]
    pub fn with_ownership(mut self, ownership: AbiOwnership) -> Self {
        self.ownership = ownership;
        self
    }

    /// Sets the lifetime contract.
    #[must_use]
    pub fn with_lifetime(mut self, lifetime: AbiLifetime) -> Self {
        self.lifetime = lifetime;
        self
    }

    /// Sets the nullability contract.
    #[must_use]
    pub fn with_nullability(mut self, nullability: AbiNullability) -> Self {
        self.nullability = nullability;
        self
    }

    /// Sets an optional, target-neutral representation hint.
    ///
    /// The hint is not proof that a target can implement the representation.
    #[must_use]
    pub fn with_representation_hint(
        mut self,
        hint: impl Into<String>,
    ) -> AbiResult<Self> {
        self.representation_hint = Some(AbiIdentifier::new(hint)?);
        Ok(self)
    }

    /// Returns the semantic type identifier.
    #[must_use]
    pub fn type_name(&self) -> &str {
        self.type_name.as_str()
    }

    /// Returns the passing mode.
    #[must_use]
    pub fn passing(&self) -> &AbiPassingMode {
        &self.passing
    }

    /// Returns the ownership contract.
    #[must_use]
    pub const fn ownership(&self) -> AbiOwnership {
        self.ownership
    }

    /// Returns the lifetime contract.
    #[must_use]
    pub fn lifetime(&self) -> &AbiLifetime {
        &self.lifetime
    }

    /// Returns the nullability contract.
    #[must_use]
    pub const fn nullability(&self) -> AbiNullability {
        self.nullability
    }

    /// Returns the optional representation hint.
    #[must_use]
    pub fn representation_hint(&self) -> Option<&str> {
        self.representation_hint
            .as_ref()
            .map(AbiIdentifier::as_str)
    }
}

/// A named function parameter at the ABI boundary.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AbiParameter {
    name: AbiIdentifier,
    value_type: AbiTypeRef,
}

impl AbiParameter {
    /// Creates a named parameter.
    pub fn new(
        name: impl Into<String>,
        value_type: AbiTypeRef,
    ) -> AbiResult<Self> {
        Ok(Self {
            name: AbiIdentifier::new(name)?,
            value_type,
        })
    }

    /// Returns the parameter name.
    #[must_use]
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Returns the parameter's boundary type contract.
    #[must_use]
    pub const fn value_type(&self) -> &AbiTypeRef {
        &self.value_type
    }
}

/// Declares an externally implemented callable signature.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbiFunction {
    name: AbiIdentifier,
    symbol: AbiIdentifier,
    parameters: Vec<AbiParameter>,
    result: Option<AbiTypeRef>,
    variadic: bool,
    effects: BTreeSet<AbiIdentifier>,
    required_capabilities: BTreeSet<AbiIdentifier>,
    properties: BTreeMap<AbiIdentifier, String>,
}

impl AbiFunction {
    /// Creates a function declaration.
    ///
    /// Parameter order is significant and is preserved exactly.
    pub fn new(
        name: impl Into<String>,
        symbol: impl Into<String>,
        parameters: Vec<AbiParameter>,
        result: Option<AbiTypeRef>,
    ) -> AbiResult<Self> {
        let function = Self {
            name: AbiIdentifier::new(name)?,
            symbol: AbiIdentifier::new(symbol)?,
            parameters,
            result,
            variadic: false,
            effects: BTreeSet::new(),
            required_capabilities: BTreeSet::new(),
            properties: BTreeMap::new(),
        };
        function.validate()?;
        Ok(function)
    }

    /// Marks the function as variadic.
    ///
    /// The selected calling convention and target must separately support
    /// variadic calls. This method does not imply C variadic semantics.
    #[must_use]
    pub fn with_variadic(mut self, variadic: bool) -> Self {
        self.variadic = variadic;
        self
    }

    /// Declares an effect identifier.
    pub fn add_effect(&mut self, effect: impl Into<String>) -> AbiResult<()> {
        self.effects.insert(AbiIdentifier::new(effect)?);
        Ok(())
    }

    /// Declares a required capability identifier.
    pub fn require_capability(
        &mut self,
        capability: impl Into<String>,
    ) -> AbiResult<()> {
        self.required_capabilities
            .insert(AbiIdentifier::new(capability)?);
        Ok(())
    }

    /// Adds open-world metadata without overwriting an existing property.
    pub fn insert_property(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> AbiResult<()> {
        let key = AbiIdentifier::new(key)?;
        if self.properties.contains_key(&key) {
            return Err(AbiError::new(
                AbiErrorKind::DuplicateDeclaration,
                key.as_str(),
                "function property is already declared",
            ));
        }
        self.properties.insert(key, value.into());
        Ok(())
    }

    /// Validates the function's local invariants.
    pub fn validate(&self) -> AbiResult<()> {
        let mut names = BTreeSet::new();

        for parameter in &self.parameters {
            if !names.insert(parameter.name.clone()) {
                return Err(AbiError::new(
                    AbiErrorKind::DuplicateDeclaration,
                    parameter.name(),
                    "parameter name occurs more than once",
                ));
            }
        }

        // This model intentionally does not enforce a universal parameter
        // count or a target-specific variadic rule. Those checks depend on
        // the selected ABI profile and target implementation.
        Ok(())
    }

    /// Returns the source-level function name.
    #[must_use]
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Returns the external symbol spelling.
    #[must_use]
    pub fn symbol(&self) -> &str {
        self.symbol.as_str()
    }

    /// Returns parameters in declared ABI order.
    #[must_use]
    pub fn parameters(&self) -> &[AbiParameter] {
        &self.parameters
    }

    /// Returns the result contract, or `None` for a no-result function.
    #[must_use]
    pub const fn result(&self) -> Option<&AbiTypeRef> {
        self.result.as_ref()
    }

    /// Returns whether variadic behavior was declared.
    #[must_use]
    pub const fn is_variadic(&self) -> bool {
        self.variadic
    }

    /// Returns declared effects in deterministic order.
    #[must_use]
    pub fn effects(&self) -> &BTreeSet<AbiIdentifier> {
        &self.effects
    }

    /// Returns required capabilities in deterministic order.
    #[must_use]
    pub fn required_capabilities(&self) -> &BTreeSet<AbiIdentifier> {
        &self.required_capabilities
    }

    /// Returns extension metadata in deterministic key order.
    #[must_use]
    pub fn properties(&self) -> &BTreeMap<AbiIdentifier, String> {
        &self.properties
    }
}

/// A named ABI profile.
///
/// The profile identifies an ABI contract family, not a physical machine.
/// The calling convention is open-world so future or vendor-specific
/// conventions can be represented without changing this enum.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbiProfile {
    name: AbiIdentifier,
    calling_convention: AbiIdentifier,
    linkage: AbiLinkage,
    version: Option<AbiVersion>,
    target_constraints: BTreeSet<AbiIdentifier>,
    properties: BTreeMap<AbiIdentifier, String>,
}

impl AbiProfile {
    /// Creates a profile with an extensible calling-convention identifier.
    pub fn new(
        name: impl Into<String>,
        calling_convention: impl Into<String>,
        linkage: AbiLinkage,
    ) -> AbiResult<Self> {
        Ok(Self {
            name: AbiIdentifier::new(name)?,
            calling_convention: AbiIdentifier::new(calling_convention)?,
            linkage,
            version: None,
            target_constraints: BTreeSet::new(),
            properties: BTreeMap::new(),
        })
    }

    /// Sets the optional ABI profile version.
    #[must_use]
    pub fn with_version(mut self, version: AbiVersion) -> Self {
        self.version = Some(version);
        self
    }

    /// Adds a target-neutral constraint identifier.
    pub fn add_target_constraint(
        &mut self,
        constraint: impl Into<String>,
    ) -> AbiResult<()> {
        self.target_constraints
            .insert(AbiIdentifier::new(constraint)?);
        Ok(())
    }

    /// Adds profile metadata without replacing an existing key.
    pub fn insert_property(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> AbiResult<()> {
        let key = AbiIdentifier::new(key)?;
        if self.properties.contains_key(&key) {
            return Err(AbiError::new(
                AbiErrorKind::DuplicateDeclaration,
                key.as_str(),
                "profile property is already declared",
            ));
        }
        self.properties.insert(key, value.into());
        Ok(())
    }

    /// Returns the profile name.
    #[must_use]
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Returns the calling-convention identifier.
    #[must_use]
    pub fn calling_convention(&self) -> &str {
        self.calling_convention.as_str()
    }

    /// Returns the linkage intent.
    #[must_use]
    pub const fn linkage(&self) -> &AbiLinkage {
        &self.linkage
    }

    /// Returns the optional profile version.
    #[must_use]
    pub const fn version(&self) -> Option<&AbiVersion> {
        self.version.as_ref()
    }

    /// Returns declared target constraints.
    #[must_use]
    pub fn target_constraints(&self) -> &BTreeSet<AbiIdentifier> {
        &self.target_constraints
    }

    /// Returns profile extension metadata.
    #[must_use]
    pub fn properties(&self) -> &BTreeMap<AbiIdentifier, String> {
        &self.properties
    }
}

/// A complete declarative ABI contract.
///
/// The contract preserves declaration order for functions and uses ordered
/// maps/sets for metadata. It does not claim that every target can realize
/// the contract.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbiContract {
    name: AbiIdentifier,
    boundary_kind: AbiBoundaryKind,
    profile: AbiProfile,
    functions: Vec<AbiFunction>,
    properties: BTreeMap<AbiIdentifier, String>,
    compatibility: Option<AbiCompatibility>,
}

/// Declares compatibility expectations for an ABI contract.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbiCompatibility {
    /// Version declared by this contract.
    pub version: AbiVersion,
    /// Optional minimum compatible version.
    pub minimum_compatible: Option<AbiVersion>,
    /// Optional maximum compatible version, inclusive by convention.
    pub maximum_compatible: Option<AbiVersion>,
    /// Optional external schema or compatibility-policy identifier.
    pub policy: Option<AbiIdentifier>,
}

impl AbiCompatibility {
    /// Creates compatibility metadata.
    pub fn new(version: AbiVersion) -> Self {
        Self {
            version,
            minimum_compatible: None,
            maximum_compatible: None,
            policy: None,
        }
    }

    /// Sets a minimum compatible version.
    #[must_use]
    pub fn with_minimum(mut self, version: AbiVersion) -> Self {
        self.minimum_compatible = Some(version);
        self
    }

    /// Sets a maximum compatible version.
    #[must_use]
    pub fn with_maximum(mut self, version: AbiVersion) -> Self {
        self.maximum_compatible = Some(version);
        self
    }

    /// Sets an open-world compatibility policy identifier.
    pub fn with_policy(
        mut self,
        policy: impl Into<String>,
    ) -> AbiResult<Self> {
        self.policy = Some(AbiIdentifier::new(policy)?);
        Ok(self)
    }

    /// Checks that a declared minimum does not exceed the maximum under
    /// lexicographic version ordering.
    ///
    /// Version ordering is meaningful only when the selected version scheme
    /// defines lexicographic ordering. Callers using another scheme must
    /// validate ordering through that scheme's compatibility adapter.
    pub fn validate_lexicographic(&self) -> AbiResult<()> {
        if let (Some(minimum), Some(maximum)) =
            (&self.minimum_compatible, &self.maximum_compatible)
        {
            if minimum > maximum {
                return Err(AbiError::new(
                    AbiErrorKind::InvalidVersion,
                    "compatibility",
                    "minimum compatible version exceeds maximum",
                ));
            }
        }
        Ok(())
    }
}

impl AbiContract {
    /// Creates an ABI contract with one profile.
    pub fn new(
        name: impl Into<String>,
        boundary_kind: AbiBoundaryKind,
        profile: AbiProfile,
    ) -> AbiResult<Self> {
        Ok(Self {
            name: AbiIdentifier::new(name)?,
            boundary_kind,
            profile,
            functions: Vec::new(),
            properties: BTreeMap::new(),
            compatibility: None,
        })
    }

    /// Adds a function while rejecting duplicate source names or external
    /// symbols within this contract.
    pub fn add_function(&mut self, function: AbiFunction) -> AbiResult<()> {
        function.validate()?;

        if self.functions.iter().any(|existing| {
            existing.name == function.name || existing.symbol == function.symbol
        }) {
            return Err(AbiError::new(
                AbiErrorKind::DuplicateDeclaration,
                function.name(),
                "function name or external symbol is already declared",
            ));
        }

        self.functions.push(function);
        Ok(())
    }

    /// Adds open-world contract metadata without overwriting a prior key.
    pub fn insert_property(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> AbiResult<()> {
        let key = AbiIdentifier::new(key)?;
        if self.properties.contains_key(&key) {
            return Err(AbiError::new(
                AbiErrorKind::DuplicateDeclaration,
                key.as_str(),
                "contract property is already declared",
            ));
        }
        self.properties.insert(key, value.into());
        Ok(())
    }

    /// Sets compatibility metadata.
    pub fn set_compatibility(
        &mut self,
        compatibility: AbiCompatibility,
    ) -> AbiResult<()> {
        compatibility.validate_lexicographic()?;
        self.compatibility = Some(compatibility);
        Ok(())
    }

    /// Validates all contract-local invariants.
    pub fn validate(&self) -> AbiResult<()> {
        let mut names = BTreeSet::new();
        let mut symbols = BTreeSet::new();

        for function in &self.functions {
            function.validate()?;

            if !names.insert(function.name.clone()) {
                return Err(AbiError::new(
                    AbiErrorKind::DuplicateDeclaration,
                    function.name(),
                    "function name occurs more than once",
                ));
            }

            if !symbols.insert(function.symbol.clone()) {
                return Err(AbiError::new(
                    AbiErrorKind::DuplicateDeclaration,
                    function.symbol(),
                    "external symbol occurs more than once",
                ));
            }
        }

        if let Some(compatibility) = &self.compatibility {
            compatibility.validate_lexicographic()?;
        }

        Ok(())
    }

    /// Validates the contract and maps failures to the shared error boundary.
    pub fn validate_classical(&self) -> ClassicalResult<()> {
        self.validate().map_err(AbiError::into_classical_error)
    }

    /// Returns the contract identifier.
    #[must_use]
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Returns the boundary kind.
    #[must_use]
    pub const fn boundary_kind(&self) -> AbiBoundaryKind {
        self.boundary_kind
    }

    /// Returns the selected declarative ABI profile.
    #[must_use]
    pub const fn profile(&self) -> &AbiProfile {
        &self.profile
    }

    /// Returns functions in declaration order.
    #[must_use]
    pub fn functions(&self) -> &[AbiFunction] {
        &self.functions
    }

    /// Returns contract metadata.
    #[must_use]
    pub fn properties(&self) -> &BTreeMap<AbiIdentifier, String> {
        &self.properties
    }

    /// Returns compatibility metadata when present.
    #[must_use]
    pub const fn compatibility(&self) -> Option<&AbiCompatibility> {
        self.compatibility.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_function() -> AbiFunction {
        let input = AbiTypeRef::new("zamani.integer")
            .expect("valid type")
            .with_passing(AbiPassingMode::ByValue);

        let output = AbiTypeRef::new("zamani.integer").expect("valid type");

        AbiFunction::new("add", "external_add", vec![
            AbiParameter::new("left", input.clone()).expect("valid parameter"),
            AbiParameter::new("right", input).expect("valid parameter"),
        ], Some(output))
        .expect("valid function")
    }

    fn sample_contract() -> AbiContract {
        let profile = AbiProfile::new(
            "portable",
            "example.calling-convention",
            AbiLinkage::Host,
        )
        .expect("valid profile");

        let mut contract =
            AbiContract::new("example.math", AbiBoundaryKind::Callable, profile)
                .expect("valid contract");

        contract
            .add_function(sample_function())
            .expect("function can be added");

        contract
    }

    #[test]
    fn accepts_an_extensible_calling_convention() {
        let profile = AbiProfile::new(
            "future-profile",
            "future.vendor.convention.v2",
            AbiLinkage::Custom(
                AbiIdentifier::new("future.linker").expect("valid linkage"),
            ),
        )
        .expect("profile is independent of a closed convention enum");

        assert_eq!(
            profile.calling_convention(),
            "future.vendor.convention.v2"
        );
    }

    #[test]
    fn rejects_empty_and_nul_identifiers() {
        assert!(AbiIdentifier::new("").is_err());
        assert!(AbiIdentifier::new("bad\0symbol").is_err());
    }

    #[test]
    fn rejects_duplicate_parameter_names() {
        let ty = AbiTypeRef::new("integer").expect("valid type");
        let first = AbiParameter::new("value", ty.clone()).expect("parameter");
        let second = AbiParameter::new("value", ty).expect("parameter");

        let result = AbiFunction::new(
            "f",
            "foreign_f",
            vec![first, second],
            None,
        );

        assert!(matches!(
            result,
            Err(AbiError {
                kind: AbiErrorKind::DuplicateDeclaration,
                ..
            })
        ));
    }

    #[test]
    fn rejects_duplicate_external_symbols() {
        let mut contract = sample_contract();

        let duplicate = AbiFunction::new(
            "different_name",
            "external_add",
            Vec::new(),
            None,
        )
        .expect("valid individual function");

        assert!(matches!(
            contract.add_function(duplicate),
            Err(AbiError {
                kind: AbiErrorKind::DuplicateDeclaration,
                ..
            })
        ));
    }

    #[test]
    fn validates_a_complete_contract() {
        let contract = sample_contract();
        assert!(contract.validate().is_ok());
        assert!(contract.validate_classical().is_ok());
    }

    #[test]
    fn preserves_parameter_order() {
        let function = sample_function();

        assert_eq!(function.parameters()[0].name(), "left");
        assert_eq!(function.parameters()[1].name(), "right");
    }

    #[test]
    fn rejects_inverted_lexicographic_compatibility_range() {
        let compatibility = AbiCompatibility::new(
            AbiVersion::new("2").expect("version"),
        )
        .with_minimum(AbiVersion::new("3").expect("version"))
        .with_maximum(AbiVersion::new("1").expect("version"));

        assert!(matches!(
            compatibility.validate_lexicographic(),
            Err(AbiError {
                kind: AbiErrorKind::InvalidVersion,
                ..
            })
        ));
    }

    #[test]
    fn allows_arbitrary_function_counts_at_the_model_level() {
        let mut contract = sample_contract();

        for index in 0_u64..128 {
            let name = format!("function_{index}");
            let symbol = format!("symbol_{index}");

            contract
                .add_function(
                    AbiFunction::new(name, symbol, Vec::new(), None)
                        .expect("valid function"),
                )
                .expect("unique function");
        }

        assert_eq!(contract.functions().len(), 129);
        assert!(contract.validate().is_ok());
    }

    #[test]
    fn rejects_duplicate_metadata_without_overwriting() {
        let mut contract = sample_contract();

        contract
            .insert_property("provider", "first")
            .expect("first property");

        assert!(contract
            .insert_property("provider", "second")
            .is_err());

        assert_eq!(
            contract
                .properties()
                .get(&AbiIdentifier::new("provider").expect("identifier"))
                .map(String::as_str),
            Some("first")
        );
    }
}


#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Function
//!
//! Canonical path: `src/zuir/ir/function.rs`
//! Edition: Rust 2021
//! Minimum Rust baseline: Rust 1.97.1 or later.
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Define the canonical, target-independent function declaration and
//! definition container for ZUIR.
//!
//! A function owns its identity, resolved symbol identity, logical
//! signature, body-region references, extensible semantic attributes,
//! declared effects, and linkage classification.
//!
//! ## AUTHORITY
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/ids.rs`
//! - `src/zuir/schema/attributes.rs`
//! - `src/zuir/schema/types.rs`
//! - `src/zuir/values/symbols.rs`
//! - `grammar/DESIGN.md`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/lexical.md`
//!
//! ## OWNS
//!
//! - Function identity and local function invariants.
//! - Logical parameter and result signatures.
//! - Function-to-symbol association.
//! - Function body-region references.
//! - Function-local attributes and declared effects.
//! - Linkage classification.
//! - Deterministic attribute and effect traversal.
//! - Validation of locally owned invariants.
//!
//! ## DOES_NOT_OWN
//!
//! - Source parsing, AST construction, or source-name resolution.
//! - Allocation of IDs or global uniqueness across independent allocators.
//! - Semantic type inference or the definition of semantic types.
//! - Region, block, operation, SSA, or control-flow representation.
//! - Whole-program symbol resolution or visibility enforcement.
//! - Effect inference or proof that declared effects are complete.
//! - Calling-convention implementation or physical ABI lowering.
//! - Hardware resource allocation, scheduling, or runtime execution.
//! - Full-program verification.
//!
//! ## INTEGRATION CONTRACT
//!
//! `ir/module.rs` may store `ZuirFunction` values in its function map keyed
//! by `FunctionId`. The module and function IDs must agree when inserted.
//! Because the module container currently accepts a generic function payload,
//! that cross-check belongs in the module builder or full verifier.
//!
//! `ids.rs` owns FunctionId, SymbolId, ValueId, TypeId, and RegionId.
//! `schema/attributes.rs` owns AttributeKey and AttributeValue.
//! `schema/types.rs` owns QualifiedName.
//! `values/symbols.rs` owns symbol declarations and linkage-related symbol
//! facts. The symbol resolver remains responsible for cross-module rules.
//!
//! The eventual `ir/region.rs` owns region contents. A function stores region
//! IDs rather than duplicating region bodies. The full verifier must confirm
//! that every referenced region exists and is owned by the correct function.
//!
//! The type verifier must confirm that parameter and result TypeIds resolve.
//! The effect verifier must check that operation and region effects are
//! compatible with the function's declared effects.
//!
//! The grammar and semantic model own source syntax and source meaning.
//! This file accepts resolved semantic identities; it does not parse names.
//!
//! ## SCALABILITY AND POCO-REAF
//!
//! No fixed parameter count, result count, region count, effect count,
//! attribute count, processor count, qubit count, register width, or target
//! capacity is imposed here.
//!
//! Collections are dynamically sized. A valid function can describe work
//! for classical, quantum, HDL, tensor, distributed, or future domains
//! through types, effects, regions, and registered extension contracts.
//!
//! Physical execution remains subject to the selected target's capabilities
//! and the caller's explicit resource policy. No finite process can promise
//! literal infinite memory, address space, or execution time.
//!
//! ## TRUST BOUNDARY
//!
//! Deserialization is not proof of validity. Call `validate()` after
//! decoding untrusted data and before trusting the function's local state.
//! Full verification must follow before optimization or lowering.
//!
//! ## FREEZE CRITERIA
//!
//! - IDs reuse the repository's canonical typed identifiers.
//! - Parameter ValueIds are unique within a signature.
//! - Declaration and definition states are explicit.
//! - Duplicate body-region references are rejected.
//! - Attributes and effects use extensible schema-owned names.
//! - Validation never claims to replace full-program verification.
//! - Tests cover invalid states, deterministic traversal, and serialization.
//! - No unsafe code or hardware-specific assumptions are introduced.

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::zuir::ids::{
    FunctionId, RegionId, SymbolId, TypeId, ValueId,
};
use crate::zuir::schema::attributes::{
    AttributeKey, AttributeValue,
};
use crate::zuir::schema::types::QualifiedName;

// =============================================================================
// Function parameter
// =============================================================================

/// One logical input parameter of a function.
///
/// `value` identifies the parameter value in the canonical IR.
/// `ty` refers to its semantic type definition.
/// `symbol` is optional because not every IR parameter needs a source-level
/// name. If present, the symbol must resolve in the appropriate scope.
///
/// A parameter's physical representation is not determined here.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FunctionParameter {
    value: ValueId,
    ty: TypeId,
    symbol: Option<SymbolId>,
}

impl FunctionParameter {
    /// Create a parameter from already-resolved identities.
    #[must_use]
    pub const fn new(
        value: ValueId,
        ty: TypeId,
        symbol: Option<SymbolId>,
    ) -> Self {
        Self { value, ty, symbol }
    }

    /// Return this parameter's SSA/value identity.
    #[must_use]
    pub const fn value(&self) -> ValueId {
        self.value
    }

    /// Return the parameter's semantic type identity.
    #[must_use]
    pub const fn ty(&self) -> TypeId {
        self.ty
    }

    /// Return the optional resolved symbol identity.
    #[must_use]
    pub const fn symbol(&self) -> Option<SymbolId> {
        self.symbol
    }
}

// =============================================================================
// Function signature
// =============================================================================

/// The logical signature of a function.
///
/// Parameter and result types refer to canonical semantic type definitions.
/// They do not encode physical registers, stack layout, device calling
/// conventions, wire formats, or target-specific data layout.
///
/// `calling_convention` is optional and extensible. A qualified name identifies
/// a convention defined by the core specification or a registered extension.
/// Its meaning must be resolved by the verifier and the relevant lowering
/// implementation; an unknown convention must not be silently accepted.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FunctionSignature {
    parameters: Vec<FunctionParameter>,
    results: Vec<TypeId>,
    calling_convention: Option<QualifiedName>,
    variadic: bool,
}

impl FunctionSignature {
    /// Construct a signature with explicit parameters and results.
    ///
    /// The signature is checked for local invariants by `validate()`.
    /// Referenced types must be resolved by the type verifier.
    #[must_use]
    pub fn new(
        parameters: Vec<FunctionParameter>,
        results: Vec<TypeId>,
    ) -> Self {
        Self {
            parameters,
            results,
            calling_convention: None,
            variadic: false,
        }
    }

    /// Set an extension-qualified calling convention.
    ///
    /// This setter records the declaration. It does not prove that the
    /// convention is registered or supported by a target.
    pub fn with_calling_convention(
        mut self,
        convention: QualifiedName,
    ) -> Result<Self, FunctionError> {
        convention
            .validate()
            .map_err(|error| {
                FunctionError::InvalidCallingConvention(error.to_string())
            })?;

        self.calling_convention = Some(convention);
        Ok(self)
    }

    /// Mark the signature as variadic.
    ///
    /// Whether variadic functions are legal for a given dialect, type system,
    /// or calling convention is decided by semantic and dialect verification.
    #[must_use]
    pub fn with_variadic(mut self, variadic: bool) -> Self {
        self.variadic = variadic;
        self
    }

    /// Return parameters in their declared order.
    #[must_use]
    pub fn parameters(&self) -> &[FunctionParameter] {
        &self.parameters
    }

    /// Return result types in their declared order.
    #[must_use]
    pub fn results(&self) -> &[TypeId] {
        &self.results
    }

    /// Return the optional calling convention.
    #[must_use]
    pub fn calling_convention(&self) -> Option<&QualifiedName> {
        self.calling_convention.as_ref()
    }

    /// Return whether the signature is variadic.
    #[must_use]
    pub const fn is_variadic(&self) -> bool {
        self.variadic
    }

    /// Validate signature-local invariants.
    ///
    /// This does not establish that TypeIds exist or that the selected
    /// calling convention is supported.
    pub fn validate(&self) -> Result<(), FunctionError> {
        let mut parameter_values = BTreeSet::new();

        for parameter in &self.parameters {
            if !parameter_values.insert(parameter.value()) {
                return Err(FunctionError::DuplicateParameterValue(
                    parameter.value(),
                ));
            }
        }

        if let Some(convention) = &self.calling_convention {
            convention
                .validate()
                .map_err(|error| {
                    FunctionError::InvalidCallingConvention(error.to_string())
                })?;
        }

        Ok(())
    }
}

// =============================================================================
// Function body
// =============================================================================

/// Describes whether a function has a body in this IR.
///
/// A declaration has no body regions. A definition references one or more
/// regions whose actual contents are owned by `ir/region.rs`.
///
/// External implementations, compiler intrinsics, and domain-defined
/// callables can be represented by declarations with suitable linkage and
/// registered attributes. Their execution meaning must be established by
/// the symbol, dialect, and lowering contracts rather than inferred from
/// a missing body.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "regions", rename_all = "snake_case")]
pub enum FunctionBody {
    /// The implementation is supplied elsewhere or is intentionally absent.
    Declaration,

    /// The function body consists of references to canonical regions.
    Definition(Vec<RegionId>),
}

impl FunctionBody {
    /// Construct a declaration without body regions.
    #[must_use]
    pub const fn declaration() -> Self {
        Self::Declaration
    }

    /// Construct a definition with one or more region references.
    ///
    /// Duplicate region identities are rejected. Existence and ownership
    /// of those regions are checked by the full verifier.
    pub fn definition(
        regions: Vec<RegionId>,
    ) -> Result<Self, FunctionError> {
        if regions.is_empty() {
            return Err(FunctionError::DefinitionWithoutRegions);
        }

        let mut seen = BTreeSet::new();

        for region in &regions {
            if !seen.insert(*region) {
                return Err(FunctionError::DuplicateBodyRegion(*region));
            }
        }

        Ok(Self::Definition(regions))
    }

    /// Return whether this body is a declaration.
    #[must_use]
    pub const fn is_declaration(&self) -> bool {
        matches!(self, Self::Declaration)
    }

    /// Return the referenced body regions.
    ///
    /// Declarations return an empty slice.
    #[must_use]
    pub fn regions(&self) -> &[RegionId] {
        match self {
            Self::Declaration => &[],
            Self::Definition(regions) => regions,
        }
    }

    /// Validate body-local invariants, including after deserialization.
    pub fn validate(&self) -> Result<(), FunctionError> {
        match self {
            Self::Declaration => Ok(()),
            Self::Definition(regions) => {
                if regions.is_empty() {
                    return Err(FunctionError::DefinitionWithoutRegions);
                }

                let mut seen = BTreeSet::new();

                for region in regions {
                    if !seen.insert(*region) {
                        return Err(FunctionError::DuplicateBodyRegion(*region));
                    }
                }

                Ok(())
            }
        }
    }
}

// =============================================================================
// Linkage
// =============================================================================

/// Linkage classification for a function declaration.
///
/// This describes symbol linkage, not visibility authorization, target
/// placement, or physical calling convention.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FunctionLinkage {
    /// The function is private to its defining linkage unit.
    Internal,

    /// The implementation is supplied by another linkage unit or runtime.
    External,

    /// The function participates in a weak-linkage mechanism, if supported
    /// by the applicable object format and target.
    Weak,
}

// =============================================================================
// Function errors
// =============================================================================

/// Errors for invariants owned directly by the function representation.
///
/// Cross-module resolution and full IR verification errors belong to the
/// canonical verifier and repository-wide error API.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FunctionError {
    /// The same ValueId was used for more than one parameter.
    DuplicateParameterValue(ValueId),

    /// A function definition has no body regions.
    DefinitionWithoutRegions,

    /// A body references the same region more than once.
    DuplicateBodyRegion(RegionId),

    /// A qualified calling convention is malformed.
    InvalidCallingConvention(String),

    /// A semantic attribute key is malformed.
    InvalidAttributeKey(String),
}

impl fmt::Display for FunctionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateParameterValue(value) => {
                write!(f, "duplicate function parameter value {value}")
            }
            Self::DefinitionWithoutRegions => {
                f.write_str("a function definition must reference at least one region")
            }
            Self::DuplicateBodyRegion(region) => {
                write!(f, "function body references region {region} more than once")
            }
            Self::InvalidCallingConvention(reason) => {
                write!(f, "invalid calling convention: {reason}")
            }
            Self::InvalidAttributeKey(key) => {
                write!(f, "invalid function attribute key: {key}")
            }
        }
    }
}

impl std::error::Error for FunctionError {}

// =============================================================================
// Canonical function
// =============================================================================

/// Canonical ZUIR function representation.
///
/// The function is intentionally independent of a particular computation
/// domain. Classical, quantum, HDL, tensor, distributed, and future function
/// semantics are represented through their logical signature, regions,
/// effects, and registered attributes.
///
/// The `FunctionId` is the IR entity identity. `SymbolId` is the resolved
/// declaration identity used by the symbol model. The owning program or
/// verifier must enforce the required uniqueness and association rules.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ZuirFunction {
    id: FunctionId,
    symbol: SymbolId,
    signature: FunctionSignature,
    body: FunctionBody,
    linkage: FunctionLinkage,

    /// Namespaced, declarative attributes.
    ///
    /// Attribute semantics and extension authorization are established by
    /// schema and dialect verification, not by the presence of a key here.
    attributes: BTreeMap<AttributeKey, AttributeValue>,

    /// Declared semantic effects, represented by qualified extensible names.
    ///
    /// The effect verifier determines whether these declarations are complete
    /// and compatible with the function body. This set does not itself
    /// perform effect inference.
    effects: BTreeSet<QualifiedName>,
}

impl ZuirFunction {
    /// Construct a function declaration.
    ///
    /// Use `defined` when the function has body regions.
    #[must_use]
    pub fn declaration(
        id: FunctionId,
        symbol: SymbolId,
        signature: FunctionSignature,
    ) -> Self {
        Self {
            id,
            symbol,
            signature,
            body: FunctionBody::Declaration,
            linkage: FunctionLinkage::Internal,
            attributes: BTreeMap::new(),
            effects: BTreeSet::new(),
        }
    }

    /// Construct a function definition with explicit body-region references.
    pub fn defined(
        id: FunctionId,
        symbol: SymbolId,
        signature: FunctionSignature,
        regions: Vec<RegionId>,
    ) -> Result<Self, FunctionError> {
        Ok(Self {
            id,
            symbol,
            signature,
            body: FunctionBody::definition(regions)?,
            linkage: FunctionLinkage::Internal,
            attributes: BTreeMap::new(),
            effects: BTreeSet::new(),
        })
    }

    /// Return the function identity.
    #[must_use]
    pub const fn id(&self) -> FunctionId {
        self.id
    }

    /// Return the resolved symbol identity.
    #[must_use]
    pub const fn symbol(&self) -> SymbolId {
        self.symbol
    }

    /// Return the logical function signature.
    #[must_use]
    pub const fn signature(&self) -> &FunctionSignature {
        &self.signature
    }

    /// Return the body state.
    #[must_use]
    pub const fn body(&self) -> &FunctionBody {
        &self.body
    }

    /// Return the function's linkage classification.
    #[must_use]
    pub const fn linkage(&self) -> FunctionLinkage {
        self.linkage
    }

    /// Set linkage explicitly.
    ///
    /// The linker and verifier remain responsible for determining whether
    /// the selected linkage is legal in the containing program.
    pub fn set_linkage(&mut self, linkage: FunctionLinkage) {
        self.linkage = linkage;
    }

    /// Replace the signature.
    ///
    /// The new signature is checked for local invariants before replacement.
    /// This operation may invalidate previous verification results.
    pub fn set_signature(
        &mut self,
        signature: FunctionSignature,
    ) -> Result<(), FunctionError> {
        signature.validate()?;
        self.signature = signature;
        Ok(())
    }

    /// Replace the body state after validating local invariants.
    ///
    /// Region existence and ownership must be checked by the full verifier.
    pub fn set_body(
        &mut self,
        body: FunctionBody,
    ) -> Result<(), FunctionError> {
        body.validate()?;
        self.body = body;
        Ok(())
    }

    /// Add or replace a function attribute.
    ///
    /// The previous value is returned when the key already existed.
    /// Key syntax is checked here; schema type checking and namespace
    /// authorization belong to the schema and dialect registries.
    pub fn set_attribute(
        &mut self,
        key: AttributeKey,
        value: AttributeValue,
    ) -> Result<Option<AttributeValue>, FunctionError> {
        key.validate().map_err(|error| {
            FunctionError::InvalidAttributeKey(error.to_string())
        })?;

        Ok(self.attributes.insert(key, value))
    }

    /// Remove a function attribute.
    pub fn remove_attribute(
        &mut self,
        key: &AttributeKey,
    ) -> Option<AttributeValue> {
        self.attributes.remove(key)
    }

    /// Get a function attribute by its fully qualified key.
    #[must_use]
    pub fn attribute(&self, key: &AttributeKey) -> Option<&AttributeValue> {
        self.attributes.get(key)
    }

    /// Iterate over attributes in deterministic key order.
    pub fn attributes(
        &self,
    ) -> impl ExactSizeIterator<Item = (&AttributeKey, &AttributeValue)> + '_ {
        self.attributes.iter()
    }

    /// Return a read-only view of the attribute map.
    #[must_use]
    pub const fn attributes_map(
        &self,
    ) -> &BTreeMap<AttributeKey, AttributeValue> {
        &self.attributes
    }

    /// Declare an effect using its qualified schema name.
    ///
    /// Duplicate declarations are idempotent. This method does not prove
    /// that the effect is registered or that the function body obeys it.
    pub fn add_effect(
        &mut self,
        effect: QualifiedName,
    ) -> Result<bool, FunctionError> {
        effect.validate().map_err(|error| {
            FunctionError::InvalidCallingConvention(error.to_string())
        })?;

        Ok(self.effects.insert(effect))
    }

    /// Remove a declared effect.
    pub fn remove_effect(&mut self, effect: &QualifiedName) -> bool {
        self.effects.remove(effect)
    }

    /// Check whether an effect is declared.
    #[must_use]
    pub fn declares_effect(&self, effect: &QualifiedName) -> bool {
        self.effects.contains(effect)
    }

    /// Iterate over declared effects in deterministic order.
    pub fn effects(
        &self,
    ) -> impl ExactSizeIterator<Item = &QualifiedName> + '_ {
        self.effects.iter()
    }

    /// Return a read-only view of the declared effects.
    #[must_use]
    pub const fn effects_set(&self) -> &BTreeSet<QualifiedName> {
        &self.effects
    }

    /// Validate invariants owned by the function container.
    ///
    /// This method is required after deserialization and before full
    /// verification. It does not check referenced type existence, symbol
    /// resolution, region ownership, control flow, effect completeness,
    /// attribute authorization, or target feasibility.
    pub fn validate(&self) -> Result<(), FunctionError> {
        self.signature.validate()?;
        self.body.validate()?;

        for key in self.attributes.keys() {
            key.validate().map_err(|error| {
                FunctionError::InvalidAttributeKey(error.to_string())
            })?;
        }

        for effect in &self.effects {
            effect.validate().map_err(|error| {
                FunctionError::InvalidCallingConvention(error.to_string())
            })?;
        }

        Ok(())
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn function_id(value: u128) -> FunctionId {
        FunctionId::new(value).expect("nonzero function ID")
    }

    fn symbol_id(value: u128) -> SymbolId {
        SymbolId::new(value).expect("nonzero symbol ID")
    }

    fn value_id(value: u128) -> ValueId {
        ValueId::new(value).expect("nonzero value ID")
    }

    fn type_id(value: u128) -> TypeId {
        TypeId::new(value).expect("nonzero type ID")
    }

    fn region_id(value: u128) -> RegionId {
        RegionId::new(value).expect("nonzero region ID")
    }

    fn qualified_name(value: &str) -> QualifiedName {
        QualifiedName::new(value.split("::").collect::<Vec<_>>())
            .expect("valid qualified name")
    }

    fn parameter(value: u128, ty: u128) -> FunctionParameter {
        FunctionParameter::new(value_id(value), type_id(ty), None)
    }

    #[test]
    fn creates_a_function_declaration() {
        let signature =
            FunctionSignature::new(vec![parameter(1, 10)], vec![type_id(11)]);

        let function =
            ZuirFunction::declaration(function_id(1), symbol_id(2), signature);

        assert_eq!(function.id(), function_id(1));
        assert_eq!(function.symbol(), symbol_id(2));
        assert!(function.body().is_declaration());
        assert!(function.validate().is_ok());
    }

    #[test]
    fn creates_a_function_definition() {
        let signature =
            FunctionSignature::new(vec![parameter(1, 10)], vec![type_id(11)]);

        let function = ZuirFunction::defined(
            function_id(1),
            symbol_id(2),
            signature,
            vec![region_id(3)],
        )
        .expect("valid function definition");

        assert!(!function.body().is_declaration());
        assert_eq!(function.body().regions(), &[region_id(3)]);
        assert!(function.validate().is_ok());
    }

    #[test]
    fn rejects_a_definition_without_regions() {
        assert_eq!(
            FunctionBody::definition(vec![]),
            Err(FunctionError::DefinitionWithoutRegions)
        );
    }

    #[test]
    fn rejects_duplicate_body_regions() {
        assert_eq!(
            FunctionBody::definition(vec![region_id(1), region_id(1)]),
            Err(FunctionError::DuplicateBodyRegion(region_id(1)))
        );
    }

    #[test]
    fn rejects_duplicate_parameter_values() {
        let signature = FunctionSignature::new(
            vec![parameter(1, 10), parameter(1, 11)],
            vec![],
        );

        assert_eq!(
            signature.validate(),
            Err(FunctionError::DuplicateParameterValue(value_id(1)))
        );
    }

    #[test]
    fn permits_repeated_parameter_types_with_distinct_values() {
        let signature = FunctionSignature::new(
            vec![parameter(1, 10), parameter(2, 10)],
            vec![type_id(10)],
        );

        assert!(signature.validate().is_ok());
    }

    #[test]
    fn effects_are_unique_and_deterministic() {
        let signature = FunctionSignature::new(vec![], vec![]);
        let mut function =
            ZuirFunction::declaration(function_id(1), symbol_id(2), signature);

        let io = qualified_name("core::io");
        let memory = qualified_name("core::memory");

        assert!(function.add_effect(memory.clone()).unwrap());
        assert!(function.add_effect(io.clone()).unwrap());
        assert!(!function.add_effect(io.clone()).unwrap());

        assert_eq!(
            function.effects().cloned().collect::<Vec<_>>(),
            vec![io, memory]
        );
    }

    #[test]
    fn attributes_are_namespaced_and_replace_explicitly() {
        let signature = FunctionSignature::new(vec![], vec![]);
        let mut function =
            ZuirFunction::declaration(function_id(1), symbol_id(2), signature);

        let key = AttributeKey::parse("core::inline").expect("valid key");

        assert_eq!(
            function
                .set_attribute(key.clone(), AttributeValue::Boolean(false))
                .unwrap(),
            None
        );

        assert_eq!(
            function
                .set_attribute(key.clone(), AttributeValue::Boolean(true))
                .unwrap(),
            Some(AttributeValue::Boolean(false))
        );

        assert_eq!(
            function.attribute(&key),
            Some(&AttributeValue::Boolean(true))
        );
    }

    #[test]
    fn signature_updates_validate_before_replacement() {
        let initial =
            FunctionSignature::new(vec![parameter(1, 10)], vec![]);

        let mut function =
            ZuirFunction::declaration(function_id(1), symbol_id(2), initial);

        let invalid = FunctionSignature::new(
            vec![parameter(3, 10), parameter(3, 11)],
            vec![],
        );

        assert!(function.set_signature(invalid).is_err());
        assert_eq!(function.signature().parameters().len(), 1);
        assert_eq!(function.signature().parameters()[0].value(), value_id(1));
    }

    #[test]
    fn serialization_round_trip_preserves_function() {
        let signature =
            FunctionSignature::new(vec![parameter(1, 10)], vec![type_id(11)]);

        let mut function = ZuirFunction::defined(
            function_id(1),
            symbol_id(2),
            signature,
            vec![region_id(3)],
        )
        .expect("valid definition");

        function
            .add_effect(qualified_name("core::io"))
            .expect("valid effect");

        function.set_attribute(
            AttributeKey::parse("core::inline").expect("valid key"),
            AttributeValue::Boolean(true),
        )
        .expect("valid attribute");

        let encoded = serde_json::to_string(&function).expect("serialize");
        let decoded: ZuirFunction =
            serde_json::from_str(&encoded).expect("deserialize");

        decoded.validate().expect("validate decoded function");
        assert_eq!(function, decoded);
    }
}

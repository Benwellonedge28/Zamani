
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani AI Interoperability Integration Tests
//!
//! Path: `src/ai/tests/interoperability.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Verify the public interoperability contract between the AI subsystem
//! and canonical computational representations owned by other subsystems.
//!
//! These tests exercise the existing interoperability implementation.
//! They do not introduce another intermediate representation, semantic
//! model, tensor storage system, or execution engine.
//!
//! ## Canonical authorities
//!
//! - `crate::ai::interoperability`
//! - `crate::ai::compatibility`
//! - `crate::ai::tensor::interoperability`
//! - `crate::frontend::semantic::domains::ai`
//! - `crate::zuir::version`
//! - `crate::zuir::interchange`
//! - `crate::classical::collections::tensor`
//! - `crate::quantum::ir`
//! - `crate::hdl::hdlir`
//! - `crate::hybrid`
//! - `crate::distributed`
//!
//! ## Ownership
//!
//! This file owns:
//!
//! - Cross-domain interoperability contract tests.
//! - Test-only adapter implementations.
//! - Test-only consumer views.
//! - Interoperability regression tests.
//! - Portability contract tests.
//! - Semantic preservation assertions.
//!
//! This file does not own:
//!
//! - Production representation adapters.
//! - Canonical semantic types.
//! - Tensor storage.
//! - Quantum representations.
//! - HDL representations.
//! - Hybrid graphs.
//! - Distributed execution plans.
//! - Hardware discovery.
//! - Resource allocation.
//! - Scheduling.
//! - Runtime execution.
//!
//! ## POCO-REAF
//!
//! Tests must not impose arbitrary maximums on:
//!
//! - Models.
//! - Parameters.
//! - Tensor rank.
//! - Tensor dimensions.
//! - Operations.
//! - Adapters.
//! - Computational domains.
//! - Distributed participants.
//! - Hardware resources.
//!
//! Test workloads may be bounded to keep test execution practical.
//! Such workload choices are not language-level semantic restrictions.
//!
//! Literal infinite computation is not promised.
//!
//! ## Integration principle
//!
//! An interoperability conversion must preserve its exact source and
//! destination identities, versions, declared semantic fidelity,
//! and conversion result.
//!
//! Unsupported conversions must fail explicitly.
//!
//! Approximation must never be silently accepted.
//!
//! A successful adapter invocation alone does not establish that the
//! converted representation is semantically correct. Tests must verify
//! the properties relevant to each conversion.
//!
//! ## Freeze criteria
//!
//! - Public adapter interfaces are exercised.
//! - Canonical semantic types are reused.
//! - Exact version matching is enforced.
//! - Wrong Rust types are rejected.
//! - Fidelity policies are enforced.
//! - Conversion reports are checked.
//! - Registry behavior is deterministic.
//! - Errors are propagated.
//! - No unsafe code is introduced.
//! - No competing production representation is introduced.

use crate::ai::interoperability::{
    AiAdapterDescriptor,
    AiAdapterId,
    AiConversionContext,
    AiConversionFidelity,
    AiConversionPolicy,
    AiConversionReport,
    AiConverted,
    AiFidelityClass,
    AiInteroperabilityRegistry,
    AiRepresentationAdapter,
    AiRepresentationId,
    AiRepresentationRef,
};

use crate::frontend::semantic::domains::ai::{
    AiId,
    DimensionExpr,
    TensorShape,
};

use crate::zuir::version::VersionTriple;

use crate::ai::error::{
    AiError,
    AiResult,
};

// ============================================================================
// TEST REPRESENTATION
// ============================================================================
//
// This wrapper exists only inside the test module.
//
// It is NOT a new production tensor representation.
//
// It demonstrates that an adapter can consume the canonical semantic
// TensorShape and expose a typed consumer view without redefining the
// underlying semantic shape.

#[derive(Clone, Debug)]
struct SemanticShapeView {
    shape: TensorShape,
}

impl SemanticShapeView {
    fn new(shape: TensorShape) -> Self {
        Self { shape }
    }

    fn shape(&self) -> &TensorShape {
        &self.shape
    }
}

// ============================================================================
// TEST IDENTITIES
// ============================================================================

fn representation(
    namespace: &str,
    name: &str,
    version: VersionTriple,
) -> AiResult<AiRepresentationRef> {
    let identity = AiRepresentationId::new(
        namespace,
        name,
    )?;

    Ok(AiRepresentationRef::new(
        identity,
        version,
    ))
}

fn adapter_identity() -> AiResult<AiAdapterId> {
    AiAdapterId::new(
        "zamani.tests.interoperability",
        "semantic_shape_adapter",
    )
}

fn source_representation() -> AiResult<AiRepresentationRef> {
    representation(
        "zamani.frontend.semantic",
        "ai.tensor_shape",
        VersionTriple::new(1, 0, 0),
    )
}

fn destination_representation() -> AiResult<AiRepresentationRef> {
    representation(
        "zamani.ai.consumer",
        "semantic_shape_view",
        VersionTriple::new(1, 0, 0),
    )
}

fn adapter_version() -> VersionTriple {
    VersionTriple::new(1, 0, 0)
}

// ============================================================================
// TEST ADAPTER
// ============================================================================

struct SemanticShapeAdapter {
    descriptor: AiAdapterDescriptor,
}

impl SemanticShapeAdapter {
    fn new() -> AiResult<Self> {
        let descriptor = AiAdapterDescriptor::new(
            adapter_identity()?,
            source_representation()?,
            destination_representation()?,
            adapter_version(),
            [AiFidelityClass::Exact],
        )?;

        Ok(Self {
            descriptor,
        })
    }
}

impl AiRepresentationAdapter for SemanticShapeAdapter {
    type Source = TensorShape;

    type Destination = SemanticShapeView;

    fn descriptor(&self) -> &AiAdapterDescriptor {
        &self.descriptor
    }

    fn convert(
        &self,
        source: &Self::Source,
        _context: &AiConversionContext,
    ) -> AiResult<(
        Self::Destination,
        AiConversionReport,
    )> {
        // The canonical semantic shape is cloned without changing its
        // dimensions, symbolic identities, or rank.

        let destination = SemanticShapeView::new(
            source.clone(),
        );

        let report = AiConversionReport::new(
            self.descriptor.identity().clone(),
            self.descriptor.source().clone(),
            self.descriptor.destination().clone(),
            AiConversionFidelity::Exact,
        )?;

        Ok((
            destination,
            report,
        ))
    }
}

// ============================================================================
// TEST REGISTRY
// ============================================================================

fn registry() -> AiResult<AiInteroperabilityRegistry> {
    let mut registry = AiInteroperabilityRegistry::new();

    registry.register(
        SemanticShapeAdapter::new()?,
    )?;

    Ok(registry)
}

// ============================================================================
// CANONICAL SEMANTIC FIXTURES
// ============================================================================

fn symbolic_shape() -> AiResult<TensorShape> {
    let batch = AiId::new("batch")?;

    Ok(TensorShape::new(vec![
        DimensionExpr::Symbol(batch),
        DimensionExpr::Constant(128),
        DimensionExpr::Constant(64),
    ]))
}

fn static_shape() -> TensorShape {
    TensorShape::new(vec![
        DimensionExpr::Constant(16),
        DimensionExpr::Constant(32),
        DimensionExpr::Constant(64),
    ])
}

// ============================================================================
// IDENTITY AND VERSION CONTRACTS
// ============================================================================

/// Verifies that source and destination representation identities
/// remain distinct and correctly namespaced.
#[test]
fn representation_identities_are_distinct() -> AiResult<()> {
    let source = source_representation()?;
    let destination = destination_representation()?;

    assert_ne!(
        source.identity(),
        destination.identity(),
    );

    assert_eq!(
        source.version(),
        VersionTriple::new(1, 0, 0),
    );

    assert_eq!(
        destination.version(),
        VersionTriple::new(1, 0, 0),
    );

    Ok(())
}

/// Verifies that adapter registration preserves its declared
/// representation endpoints.
#[test]
fn adapter_descriptor_preserves_exact_endpoints() -> AiResult<()> {
    let adapter = SemanticShapeAdapter::new()?;

    let descriptor = adapter.descriptor();

    assert_eq!(
        descriptor.identity(),
        &adapter_identity()?,
    );

    assert_eq!(
        descriptor.source(),
        &source_representation()?,
    );

    assert_eq!(
        descriptor.destination(),
        &destination_representation()?,
    );

    assert_eq!(
        descriptor.version(),
        adapter_version(),
    );

    assert!(
        descriptor
            .supported_fidelity()
            .contains(&AiFidelityClass::Exact)
    );

    Ok(())
}

// ============================================================================
// CANONICAL SEMANTIC INTEROPERABILITY
// ============================================================================

/// Verifies that the canonical frontend TensorShape can be passed
/// through the typed interoperability boundary.
#[test]
fn converts_canonical_semantic_tensor_shape() -> AiResult<()> {
    let registry = registry()?;

    let source = static_shape();

    let converted: AiConverted<SemanticShapeView> =
        registry.convert(
            &adapter_identity()?,
            &source_representation()?,
            &destination_representation()?,
            &source,
            &AiConversionContext::exact_only(),
        )?;

    assert_eq!(
        converted.value().shape().rank(),
        source.rank(),
    );

    assert_eq!(
        converted.report().fidelity(),
        &AiConversionFidelity::Exact,
    );

    assert_eq!(
        converted.report().source(),
        &source_representation()?,
    );

    assert_eq!(
        converted.report().destination(),
        &destination_representation()?,
    );

    Ok(())
}

/// Verifies that symbolic dimensions remain symbolic during
/// interoperability conversion.
#[test]
fn symbolic_dimensions_are_preserved() -> AiResult<()> {
    let registry = registry()?;

    let source = symbolic_shape()?;

    assert_eq!(source.rank(), 3);

    let converted = registry.convert::<
        TensorShape,
        SemanticShapeView,
    >(
        &adapter_identity()?,
        &source_representation()?,
        &destination_representation()?,
        &source,
        &AiConversionContext::exact_only(),
    )?;

    assert_eq!(
        converted.value().shape().rank(),
        source.rank(),
    );

    assert_eq!(
        converted.value().shape(),
        &source,
    );

    Ok(())
}

/// Verifies that rank-zero semantic shapes remain representable
/// through the generic interoperability boundary.
#[test]
fn scalar_semantics_are_preserved() -> AiResult<()> {
    let registry = registry()?;

    let source = TensorShape::new(Vec::new());

    assert_eq!(source.rank(), 0);
    assert!(source.is_scalar());

    let converted = registry.convert::<
        TensorShape,
        SemanticShapeView,
    >(
        &adapter_identity()?,
        &source_representation()?,
        &destination_representation()?,
        &source,
        &AiConversionContext::exact_only(),
    )?;

    assert_eq!(
        converted.value().shape().rank(),
        0,
    );

    assert!(
        converted.value().shape().is_scalar(),
    );

    Ok(())
}

/// Verifies that conversion does not mutate the caller's source.
#[test]
fn conversion_preserves_source_immutability() -> AiResult<()> {
    let registry = registry()?;

    let source = symbolic_shape()?;
    let original = source.clone();

    let _converted = registry.convert::<
        TensorShape,
        SemanticShapeView,
    >(
        &adapter_identity()?,
        &source_representation()?,
        &destination_representation()?,
        &source,
        &AiConversionContext::exact_only(),
    )?;

    assert_eq!(source, original);

    Ok(())
}

// ============================================================================
// FIDELITY AND SEMANTIC PRESERVATION
// ============================================================================

/// Verifies that exact-only conversion rejects contractual
/// semantic relaxation.
#[test]
fn exact_policy_rejects_contractual_conversion() {
    let fidelity = AiConversionFidelity::Contractual {
        contract: "numerical.preservation.v1".to_owned(),
    };

    assert!(
        !AiConversionPolicy::exact_only()
            .permits(&fidelity)
    );
}

/// Verifies that approximation is never implicitly accepted.
#[test]
fn approximate_conversion_requires_explicit_permission() {
    let fidelity = AiConversionFidelity::Approximate {
        contract: "numerical.approximation.v1".to_owned(),
    };

    let exact = AiConversionPolicy::exact_only();

    assert!(!exact.permits(&fidelity));

    let contractual = AiConversionPolicy::contractual([
        "numerical.approximation.v1",
    ])
    .expect("valid contractual policy");

    assert!(!contractual.permits(&fidelity));

    let approximate = AiConversionPolicy::approximate([
        "numerical.approximation.v1",
    ])
    .expect("valid approximation policy");

    assert!(approximate.permits(&fidelity));
}

/// Verifies that explicitly accepted semantic contracts remain
/// distinguishable from unrelated contracts.
#[test]
fn contractual_conversion_requires_exact_contract_identity() {
    let policy = AiConversionPolicy::contractual([
        "shape.preservation.v1",
    ])
    .expect("valid policy");

    assert!(
        policy.permits(
            &AiConversionFidelity::Contractual {
                contract: "shape.preservation.v1".to_owned(),
            }
        )
    );

    assert!(
        !policy.permits(
            &AiConversionFidelity::Contractual {
                contract: "shape.preservation.v2".to_owned(),
            }
        )
    );
}

// ============================================================================
// REGISTRY CONTRACTS
// ============================================================================

/// Verifies that an empty registry does not invent adapter support.
#[test]
fn empty_registry_rejects_conversion() -> AiResult<()> {
    let registry = AiInteroperabilityRegistry::new();

    assert!(registry.is_empty());
    assert_eq!(registry.len(), 0);

    let source = static_shape();

    let result = registry.convert::<
        TensorShape,
        SemanticShapeView,
    >(
        &adapter_identity()?,
        &source_representation()?,
        &destination_representation()?,
        &source,
        &AiConversionContext::exact_only(),
    );

    assert!(result.is_err());

    Ok(())
}

/// Verifies that duplicate adapter identities cannot replace
/// an already registered adapter.
#[test]
fn duplicate_registration_is_rejected() -> AiResult<()> {
    let mut registry = registry()?;

    assert_eq!(registry.len(), 1);

    let result = registry.register(
        SemanticShapeAdapter::new()?,
    );

    assert!(result.is_err());
    assert_eq!(registry.len(), 1);

    Ok(())
}

/// Verifies deterministic adapter enumeration.
#[test]
fn registry_iteration_is_deterministic() -> AiResult<()> {
    let registry = registry()?;

    let first: Vec<String> = registry
        .iter()
        .map(|descriptor| {
            descriptor.identity().qualified_name()
        })
        .collect();

    let second: Vec<String> = registry
        .iter()
        .map(|descriptor| {
            descriptor.identity().qualified_name()
        })
        .collect();

    assert_eq!(first, second);

    Ok(())
}

/// Verifies that removing an adapter from one registry does not
/// modify a cloned registry snapshot.
#[test]
fn registry_snapshots_are_independent() -> AiResult<()> {
    let mut original = registry()?;

    let snapshot = original.clone();

    assert!(original.remove(&adapter_identity()?));

    assert!(original.is_empty());
    assert_eq!(snapshot.len(), 1);

    Ok(())
}

// ============================================================================
// VERSION AND TYPE SAFETY
// ============================================================================

/// Verifies that a source version mismatch cannot be silently accepted.
#[test]
fn source_version_mismatch_is_rejected() -> AiResult<()> {
    let registry = registry()?;

    let unsupported_source = representation(
        "zamani.frontend.semantic",
        "ai.tensor_shape",
        VersionTriple::new(2, 0, 0),
    )?;

    let result = registry.convert::<
        TensorShape,
        SemanticShapeView,
    >(
        &adapter_identity()?,
        &unsupported_source,
        &destination_representation()?,
        &static_shape(),
        &AiConversionContext::exact_only(),
    );

    assert!(result.is_err());

    Ok(())
}

/// Verifies that a destination version mismatch cannot be silently
/// accepted.
#[test]
fn destination_version_mismatch_is_rejected() -> AiResult<()> {
    let registry = registry()?;

    let unsupported_destination = representation(
        "zamani.ai.consumer",
        "semantic_shape_view",
        VersionTriple::new(2, 0, 0),
    )?;

    let result = registry.convert::<
        TensorShape,
        SemanticShapeView,
    >(
        &adapter_identity()?,
        &source_representation()?,
        &unsupported_destination,
        &static_shape(),
        &AiConversionContext::exact_only(),
    );

    assert!(result.is_err());

    Ok(())
}

/// Verifies that the registry checks the concrete source Rust type.
#[test]
fn wrong_source_type_is_rejected() -> AiResult<()> {
    let registry = registry()?;

    let result = registry.convert::<
        String,
        SemanticShapeView,
    >(
        &adapter_identity()?,
        &source_representation()?,
        &destination_representation()?,
        &"not a tensor shape".to_owned(),
        &AiConversionContext::exact_only(),
    );

    assert!(result.is_err());

    Ok(())
}

/// Verifies that the registry checks the concrete destination Rust type.
#[test]
fn wrong_destination_type_is_rejected() -> AiResult<()> {
    let registry = registry()?;

    let result = registry.convert::<
        TensorShape,
        String,
    >(
        &adapter_identity()?,
        &source_representation()?,
        &destination_representation()?,
        &static_shape(),
        &AiConversionContext::exact_only(),
    );

    assert!(result.is_err());

    Ok(())
}

// ============================================================================
// VERSION REPRESENTATION CONTRACT
// ============================================================================

/// Verifies that canonical ZUIR version values remain usable
/// as interoperability version identities.
#[test]
fn canonical_zuir_versions_are_preserved() {
    let version = VersionTriple::new(
        u32::MAX,
        u32::MAX,
        u32::MAX,
    );

    let reference = AiRepresentationRef::new(
        AiRepresentationId::new(
            "zamani.test",
            "versioned_representation",
        )
        .expect("valid identity"),
        version,
    );

    assert_eq!(
        reference.version(),
        version,
    );
}

/// Verifies that version advancement reports overflow instead
/// of silently wrapping.
#[test]
fn version_overflow_is_explicit() {
    let version = VersionTriple::new(
        1,
        2,
        u32::MAX,
    );

    assert!(
        version.checked_next_patch().is_none()
    );
}

// ============================================================================
// RESULT AND REPORT CONTRACTS
// ============================================================================

/// Verifies that a converted result exposes both its value
/// and its conversion report.
#[test]
fn converted_result_exposes_validated_report() -> AiResult<()> {
    let registry = registry()?;

    let converted = registry.convert::<
        TensorShape,
        SemanticShapeView,
    >(
        &adapter_identity()?,
        &source_representation()?,
        &destination_representation()?,
        &static_shape(),
        &AiConversionContext::exact_only(),
    )?;

    let report = converted.report();

    assert_eq!(
        report.adapter(),
        &adapter_identity()?,
    );

    assert_eq!(
        report.fidelity(),
        &AiConversionFidelity::Exact,
    );

    assert_eq!(
        converted.value().shape().rank(),
        3,
    );

    Ok(())
}

/// Verifies that consuming a converted result preserves the
/// destination representation.
#[test]
fn converted_result_can_be_consumed() -> AiResult<()> {
    let registry = registry()?;

    let converted = registry.convert::<
        TensorShape,
        SemanticShapeView,
    >(
        &adapter_identity()?,
        &source_representation()?,
        &destination_representation()?,
        &static_shape(),
        &AiConversionContext::exact_only(),
    )?;

    let (destination, report) = converted.into_parts();

    assert_eq!(destination.shape().rank(), 3);

    assert_eq!(
        report.fidelity(),
        &AiConversionFidelity::Exact,
    );

    Ok(())
}

// ============================================================================
// PORTABILITY CONTRACTS
// ============================================================================

/// Verifies that representation identities contain no physical
/// hardware or device selection.
#[test]
fn representation_identity_is_target_independent() -> AiResult<()> {
    let source = source_representation()?;
    let destination = destination_representation()?;

    assert_eq!(
        source.identity().namespace(),
        "zamani.frontend.semantic",
    );

    assert_eq!(
        destination.identity().namespace(),
        "zamani.ai.consumer",
    );

    Ok(())
}

/// Verifies that the same adapter contract produces the same
/// result for repeated equivalent conversions.
#[test]
fn repeated_conversion_is_deterministic() -> AiResult<()> {
    let registry = registry()?;

    let source = symbolic_shape()?;

    let first = registry.convert::<
        TensorShape,
        SemanticShapeView,
    >(
        &adapter_identity()?,
        &source_representation()?,
        &destination_representation()?,
        &source,
        &AiConversionContext::exact_only(),
    )?;

    let second = registry.convert::<
        TensorShape,
        SemanticShapeView,
    >(
        &adapter_identity()?,
        &source_representation()?,
        &destination_representation()?,
        &source,
        &AiConversionContext::exact_only(),
    )?;

    assert_eq!(
        first.value().shape(),
        second.value().shape(),
    );

    assert_eq!(
        first.report(),
        second.report(),
    );

    Ok(())
}

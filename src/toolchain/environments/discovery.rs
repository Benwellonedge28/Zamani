
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Toolchain — Environment Discovery
//!
//! Path:
//!     src/toolchain/environments/discovery.rs
//!
//! Rust edition:
//!     2021
//!
//! Minimum supported Rust:
//!     1.97
//!
//! Safety:
//!     Safe Rust only.
//!
//! # 1. Purpose
//!
//! Coordinate discovery of explicitly requested development and execution
//! environments without introducing competing environment, hardware,
//! platform-provider, or external-tool authorities.
//!
//! This module provides two independent discovery operations:
//!
//! 1. Execution-platform observation through the canonical ZUR platform
//!    provider interface.
//!
//! 2. External development-tool discovery through the canonical backend
//!    toolchain manager.
//!
//! These operations deliberately remain separate because platform
//! observation and executable validation have different owners,
//! execution characteristics, and failure contracts.
//!
//! # 2. Architectural ownership
//!
//! This module owns:
//!
//! - Coordination of environment discovery requests.
//! - Explicit platform-provider observation.
//! - Explicit external-toolchain discovery requests.
//! - Deterministic aggregation of discovered external toolchains.
//! - Duplicate toolchain identifier detection.
//! - Toolchain-boundary error adaptation.
//!
//! This module does NOT own:
//!
//! - Operating-system discovery implementation.
//! - Physical hardware discovery.
//! - Device discovery or selection.
//! - CPU, GPU, FPGA, QPU or accelerator enumeration.
//! - Hardware capability verification.
//! - Physical resource allocation.
//! - Runtime scheduling.
//! - Backend target selection.
//! - External executable resolution implementation.
//! - External process execution.
//! - Tool version parsing.
//! - Compilation-environment fingerprint algorithms.
//! - Artifact provenance.
//! - Source-language environment semantics.
//! - Manifest parsing.
//! - Grammar parsing.
//! - Environment configuration.
//!
//! # 3. Canonical authorities
//!
//! Host environment representation:
//!     crate::zur::platforms::host
//!
//! Platform observation:
//!     crate::zur::platforms::provider
//!
//! Platform provider lifecycle:
//!     crate::stdlib::provider
//!
//! Operation context and cancellation:
//!     crate::stdlib::context
//!
//! External tool discovery:
//!     crate::backend::toolchain
//!
//! External process execution:
//!     crate::backend::process
//!
//! Toolchain errors:
//!     crate::toolchain::errors
//!
//! Toolchain results:
//!     crate::toolchain::result
//!
//! Compilation environment identity:
//!     crate::compiler::reproducibility::environment
//!
//! Backend reproducibility:
//!     crate::backend::reproducibility
//!
//! # 4. Discovery guarantees
//!
//! - Platform providers must be supplied explicitly.
//! - External tool requirements must be supplied explicitly.
//! - Missing platform observations are never fabricated.
//! - Failed executable probes are never treated as success.
//! - Invalid tool versions are never assumed to be compatible.
//! - Unknown hardware capabilities remain unknown.
//! - Discovery does not establish workload feasibility.
//! - Discovery does not reserve physical resources.
//! - Discovery does not imply successful program execution.
//!
//! # 5. Scalability
//!
//! No fixed maximum is imposed on:
//!
//! - Platform providers.
//! - External toolchains.
//! - Operating systems.
//! - Processor architectures.
//! - Computing domains.
//! - Development environments.
//! - Execution environments.
//! - Hardware devices.
//! - Accelerators.
//! - Quantum processors.
//! - Distributed participants.
//! - Environment descriptors.
//!
//! Collections grow subject to available memory and the representational
//! constraints of the executing platform.
//!
//! No arbitrary hardware-capacity restriction is introduced.
//!
//! Operational budgets remain owned by configuration and execution
//! authorities.
//!
//! Literal infinite physical resources or computation are not promised.
//!
//! # 6. Determinism
//!
//! Discovered external toolchains are stored in BTreeMap order.
//!
//! Toolchain identifiers are preserved exactly as supplied.
//!
//! No locale-dependent normalization, randomized hashing, timestamps,
//! process identifiers, or memory addresses participate in identity.
//!
//! Platform observations retain the original provider's representation.
//!
//! This module does not calculate reproducibility fingerprints.
//!
//! # 7. Security
//!
//! Platform providers are explicit dependencies supplied by the caller.
//!
//! External executable specifications are explicit dependencies supplied
//! by the caller.
//!
//! Executable trust and authorization remain with the security boundary.
//!
//! Successful executable discovery does not establish executable
//! authenticity.
//!
//! Toolchain version validation does not establish binary integrity.
//!
//! Secrets must not be inserted into tool identifiers, version output,
//! descriptive metadata, or fingerprint inputs.
//!
//! # 8. Concurrency
//!
//! Platform providers must satisfy the existing Send + Sync provider
//! contracts.
//!
//! Operation cancellation uses the existing OperationContext.
//!
//! External tool discovery delegates to the existing ProcessRunner
//! implementation through ToolchainManager.
//!
//! This module introduces no global mutable state, background threads,
//! independent cancellation implementation, or hidden worker pool.

use std::collections::BTreeMap;

use crate::backend::toolchain::{
    ToolchainDescriptor,
    ToolchainManager,
    ToolchainSpec,
};

use crate::stdlib::context::OperationContext;

use crate::toolchain::errors::{
    ToolchainError,
    ToolchainErrorKind,
};

use crate::toolchain::result::Result;

use crate::zur::platforms::provider::{
    observe_platform,
    PlatformObservation,
    PlatformProvider,
};

// ============================================================================
// COMPONENT IDENTITY
// ============================================================================

const COMPONENT: &str = "toolchain.environments.discovery";

// ============================================================================
// EXTERNAL TOOLCHAIN COLLECTION
// ============================================================================

/// Successfully discovered and version-validated external toolchains.
///
/// The collection uses the external tool's caller-defined identifier
/// as its key.
///
/// The descriptors are supplied by the canonical backend toolchain
/// subsystem.
///
/// This type does not create a competing executable descriptor.
///
/// Ordering is deterministic because the underlying collection is
/// a BTreeMap.
///
/// A descriptor establishes that the backend discovery operation
/// successfully resolved and validated the requested executable.
/// It does not independently establish executable authenticity
/// or cryptographic integrity.
#[derive(Debug, Clone, Default)]
pub struct DiscoveredToolchains {
    descriptors: BTreeMap<String, ToolchainDescriptor>,
}

impl DiscoveredToolchains {
    /// Creates an empty collection of discovered toolchains.
    ///
    /// An empty collection means that no external toolchains were
    /// requested or discovered. It does not mean that every possible
    /// external toolchain is installed.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the number of successfully discovered toolchains.
    #[must_use]
    pub fn len(&self) -> usize {
        self.descriptors.len()
    }

    /// Returns whether the collection contains no toolchains.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.descriptors.is_empty()
    }

    /// Returns a discovered toolchain by its exact identifier.
    ///
    /// Identifier comparison is case-sensitive.
    #[must_use]
    pub fn get(&self, identifier: &str) -> Option<&ToolchainDescriptor> {
        self.descriptors.get(identifier)
    }

    /// Returns all discovered toolchains in deterministic identifier order.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (&str, &ToolchainDescriptor)> {
        self.descriptors
            .iter()
            .map(|(identifier, descriptor)| {
                (identifier.as_str(), descriptor)
            })
    }

    /// Consumes the collection and returns its canonical descriptor map.
    #[must_use]
    pub fn into_inner(
        self,
    ) -> BTreeMap<String, ToolchainDescriptor> {
        self.descriptors
    }

    /// Returns an iterator over the discovered identifiers.
    pub fn identifiers(
        &self,
    ) -> impl Iterator<Item = &str> {
        self.descriptors.keys().map(String::as_str)
    }
}

// ============================================================================
// ENVIRONMENT DISCOVERY SERVICE
// ============================================================================

/// Stateless coordinator for development and execution environment
/// discovery.
///
/// This service deliberately has no global registry or implicit
/// machine-selection behavior.
///
/// The caller supplies platform providers and external tool
/// specifications explicitly.
///
/// Platform observations and external executable discovery remain
/// separate operations so that callers can apply independent
/// concurrency, cancellation, security and operational-budget policies.
#[derive(Debug, Clone, Copy, Default)]
pub struct EnvironmentDiscovery {
    toolchain_manager: ToolchainManager,
}

impl EnvironmentDiscovery {
    /// Creates a new environment discovery service.
    ///
    /// The service uses the existing stateless backend toolchain manager.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            toolchain_manager: ToolchainManager::new(),
        }
    }

    // ------------------------------------------------------------------------
    // EXECUTION ENVIRONMENT OBSERVATION
    // ------------------------------------------------------------------------

    /// Obtains an execution-platform observation from an explicitly
    /// supplied canonical ZUR platform provider.
    ///
    /// This operation delegates to `observe_platform`, preserving the
    /// canonical provider identity, HostEnvironment representation,
    /// cancellation handling and provider error behavior.
    ///
    /// It does not independently enumerate hardware or infer that
    /// a reported execution interface supports a particular workload.
    ///
    /// # Arguments
    ///
    /// * `provider` - Explicitly selected platform observation provider.
    /// * `context` - Existing immutable operation context.
    ///
    /// # Returns
    ///
    /// An immutable platform observation supplied by the provider.
    ///
    /// # Errors
    ///
    /// Returns a structured toolchain error when:
    ///
    /// - The provider observation fails.
    /// - The operation is cancelled.
    /// - The provider cannot establish the requested observation.
    ///
    /// The original ZUR error is preserved as the causal source.
    pub async fn observe_execution_environment(
        &self,
        provider: &dyn PlatformProvider,
        context: &OperationContext,
    ) -> Result<PlatformObservation> {
        observe_platform(provider, context)
            .await
            .map_err(|source| {
                ToolchainError::new(
                    ToolchainErrorKind::ProviderFailed,
                    "execution environment observation failed",
                )
                .with_operation(
                    "toolchain.environments.discovery.observe_execution",
                )
                .with_context(COMPONENT)
                .with_source(source)
            })
    }

    // ------------------------------------------------------------------------
    // DEVELOPMENT ENVIRONMENT TOOLCHAIN DISCOVERY
    // ------------------------------------------------------------------------

    /// Discovers and validates an explicitly supplied collection of
    /// external development or execution toolchains.
    ///
    /// Each specification is delegated to the existing
    /// `backend::toolchain::ToolchainManager`.
    ///
    /// The backend remains responsible for:
    ///
    /// - Executable resolution.
    /// - Version probing.
    /// - Version parsing.
    /// - Version requirement validation.
    /// - Safe process invocation.
    /// - Timeout and output-capture behavior.
    ///
    /// This method performs a preflight duplicate-identifier check
    /// before invoking any external executable.
    ///
    /// Toolchains are returned in deterministic identifier order.
    ///
    /// No external executable is discovered merely because it happens
    /// to be installed on the host. The caller must request it.
    ///
    /// # Arguments
    ///
    /// * `specifications` - Explicit external toolchain specifications.
    ///
    /// # Returns
    ///
    /// All successfully resolved and version-validated descriptors.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    ///
    /// - Two specifications use the same identifier.
    /// - An executable cannot be resolved.
    /// - A version probe fails.
    /// - A version cannot be parsed.
    /// - A version requirement is not satisfied.
    ///
    /// Backend errors are retained as causal sources.
    ///
    /// No partial collection is returned as a successful result.
    pub fn discover_external_toolchains<I>(
        &self,
        specifications: I,
    ) -> Result<DiscoveredToolchains>
    where
        I: IntoIterator<Item = ToolchainSpec>,
    {
        // Collect the caller's explicit request set so duplicate identifiers
        // can be rejected before any executable is probed.
        //
        // The collection is bounded by the caller's input and available
        // memory, not by an arbitrary toolchain-count constant.
        let specifications: Vec<ToolchainSpec> =
            specifications.into_iter().collect();

        // Preflight identifiers before performing process execution.
        let mut identifiers = std::collections::BTreeSet::new();

        for specification in &specifications {
            let identifier = specification.id();

            if identifier.trim().is_empty() {
                return Err(
                    ToolchainError::new(
                        ToolchainErrorKind::InvalidInput,
                        "external toolchain identifier must not be empty",
                    )
                    .with_operation(
                        "toolchain.environments.discovery.validate",
                    )
                    .with_context(COMPONENT),
                );
            }

            if !identifiers.insert(identifier.to_owned()) {
                return Err(
                    ToolchainError::new(
                        ToolchainErrorKind::InvalidInput,
                        format!(
                            "duplicate external toolchain identifier: \
                             {identifier}"
                        ),
                    )
                    .with_operation(
                        "toolchain.environments.discovery.validate",
                    )
                    .with_subject(identifier)
                    .with_context(COMPONENT),
                );
            }
        }

        let mut discovered = DiscoveredToolchains::new();

        // Each toolchain is independently resolved and validated.
        //
        // Failure stops the operation. An incomplete result is never
        // returned as a successful discovery report.
        for specification in &specifications {
            let identifier = specification.id().to_owned();

            let descriptor = self
                .toolchain_manager
                .discover(specification)
                .map_err(|source| {
                    ToolchainError::new(
                        ToolchainErrorKind::ExternalToolFailed,
                        format!(
                            "external toolchain discovery failed: {identifier}"
                        ),
                    )
                    .with_operation(
                        "toolchain.environments.discovery.external_tool",
                    )
                    .with_subject(identifier.clone())
                    .with_context(COMPONENT)
                    .with_source(source)
                })?;

            discovered
                .descriptors
                .insert(identifier, descriptor);
        }

        Ok(discovered)
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ------------------------------------------------------------------------
    // Empty discovery
    // ------------------------------------------------------------------------

    #[test]
    fn empty_toolchain_request_returns_empty_collection() {
        let discovery = EnvironmentDiscovery::new();

        let result = discovery
            .discover_external_toolchains(
                std::iter::empty::<ToolchainSpec>(),
            )
            .expect("empty discovery should succeed");

        assert!(result.is_empty());
        assert_eq!(result.len(), 0);
    }

    // ------------------------------------------------------------------------
    // Duplicate identifier validation
    // ------------------------------------------------------------------------

    #[test]
    fn duplicate_identifiers_are_rejected_before_execution() {
        let discovery = EnvironmentDiscovery::new();

        let specifications = vec![
            ToolchainSpec::new(
                "test.duplicate",
                "nonexistent-first-executable",
            ),
            ToolchainSpec::new(
                "test.duplicate",
                "nonexistent-second-executable",
            ),
        ];

        let result =
            discovery.discover_external_toolchains(specifications);

        let error = result.expect_err(
            "duplicate identifiers must be rejected",
        );

        assert_eq!(
            error.kind(),
            ToolchainErrorKind::InvalidInput
        );

        assert_eq!(
            error.operation(),
            Some("toolchain.environments.discovery.validate")
        );

        assert_eq!(
            error.subject(),
            Some("test.duplicate")
        );
    }

    // ------------------------------------------------------------------------
    // Empty identifiers
    // ------------------------------------------------------------------------

    #[test]
    fn empty_toolchain_identifier_is_rejected() {
        let discovery = EnvironmentDiscovery::new();

        let specifications = vec![
            ToolchainSpec::new(
                "   ",
                "some-executable",
            ),
        ];

        let result =
            discovery.discover_external_toolchains(specifications);

        let error = result.expect_err(
            "empty identifiers must be rejected",
        );

        assert_eq!(
            error.kind(),
            ToolchainErrorKind::InvalidInput
        );
    }

    // ------------------------------------------------------------------------
    // Collection behavior
    // ------------------------------------------------------------------------

    #[test]
    fn empty_collection_supports_all_public_accessors() {
        let collection = DiscoveredToolchains::new();

        assert!(collection.is_empty());
        assert_eq!(collection.len(), 0);
        assert!(collection.get("unknown.tool").is_none());
        assert_eq!(collection.iter().count(), 0);
        assert_eq!(collection.identifiers().count(), 0);
        assert!(collection.into_inner().is_empty());
    }

    // ------------------------------------------------------------------------
    // Service construction
    // ------------------------------------------------------------------------

    #[test]
    fn discovery_service_is_constructible() {
        let discovery = EnvironmentDiscovery::new();

        let cloned = discovery.clone();

        let result = cloned
            .discover_external_toolchains(
                std::iter::empty::<ToolchainSpec>(),
            )
            .expect("empty discovery should succeed");

        assert!(result.is_empty());
    }
}

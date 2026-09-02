//! Zamani Quantum Noise (ZQN) — Target Capabilities
//!
//! This module defines the target-facing capability profile for ZQN.
//!
//! =============================================================================
//! OWNERSHIP
//! =============================================================================
//!
//! This file owns:
//!
//! - the immutable ZQN target-capability profile;
//! - registration of capabilities exposed by a target;
//! - deterministic capability inspection;
//! - capability requirement evaluation;
//! - capability compatibility reports;
//! - exact/approximate/native/emulated support policy;
//! - scoped capability lookup;
//! - target capability validation;
//! - capability-set composition;
//! - deterministic target-capability comparison;
//! - resource-independent capability queries.
//!
//! This file does NOT own:
//!
//! - canonical quantum program semantics;
//! - quantum IR;
//! - QubitId definitions;
//! - PhysicalQubitId definitions;
//! - hardware provider APIs;
//! - backend identity;
//! - hardware discovery;
//! - hardware topology;
//! - calibration values;
//! - routing;
//! - scheduling;
//! - quantum channels;
//! - noise models;
//! - faults;
//! - simulation;
//! - QEC;
//! - benchmarking;
//! - execution;
//! - authentication;
//! - credentials;
//! - networking;
//! - resource allocation.
//!
//! Those concerns remain owned by their respective subsystems.
//!
//! =============================================================================
//! ARCHITECTURAL POSITION
//! =============================================================================
//!
//! ```text
//! Zamani source
//!       |
//!       v
//! quantum::ir
//!       |
//!       +--------------------------+
//!       |                          |
//!       v                          v
//! semantic requirements          ZQN
//!                                  |
//!                                  v
//!                         zqn::target::requirements
//!                                  |
//!                                  v
//!                         THIS MODULE
//!                         TargetCapabilities
//!                                  |
//!                                  v
//!                         compatibility
//!                                  |
//!             +--------------------+--------------------+
//!             |                    |                    |
//!             v                    v                    v
//!          routing             scheduling              QEC
//!             |                    |                    |
//!             +--------------------+--------------------+
//!                                  |
//!                                  v
//!                            hardware/runtime
//! ```
//!
//! The direction is intentionally one-way:
//!
//!     requirement -> capability profile -> compatibility decision
//!
//! This module never discovers capabilities by itself and never calls a
//! provider API.
//!
//! =============================================================================
//! CANONICAL IDENTITY
//! =============================================================================
//!
//! ZQN MUST NOT define another QubitId or PhysicalQubitId.
//!
//! Canonical identities remain:
//!
//!     crate::quantum::ir::qubit::QubitId
//!     crate::quantum::ir::qubit::PhysicalQubitId
//!
//! The repository explicitly establishes `quantum::ir::qubit` as the
//! authoritative identity boundary.
//!
//! Scoped capability queries therefore use the types already provided by
//! `zqn::core::capabilities::CapabilityScope`.
//!
//! =============================================================================
//! WRITE ONCE / SCALE EVERYWHERE
//! =============================================================================
//!
//! This module imposes no semantic upper bound on:
//!
//! - number of qubits;
//! - number of physical resources;
//! - number of logical resources;
//! - number of capability declarations;
//! - operation arity;
//! - topology size;
//! - target size;
//! - number of execution resources.
//!
//! There is deliberately no:
//!
//!     MAX_QUBITS
//!     MAX_CAPABILITIES
//!     MAX_RESOURCES
//!
//! or equivalent semantic constant.
//!
//! A `BTreeSet` is used for deterministic representation. Its actual size is
//! bounded only by available resources and caller/runtime policy.
//!
//! Hardware capacity is NOT represented by an arbitrary compile-time maximum.
//! Capacity belongs to the target/hardware resource model and is consumed by
//! higher-level target requirements and compatibility logic.
//!
//! =============================================================================
//! EXACT VS APPROXIMATE
//! =============================================================================
//!
//! ZQN must never silently turn an approximation into exact support.
//!
//! This module preserves the distinction between:
//!
//!     Native
//!     Emulated
//!     Approximate
//!     Unsupported
//!
//! Requirements can independently request:
//!
//!     Native
//!     Exact
//!     Approximate
//!     Any
//!
//! Consequently:
//!
//!     Exact requirement + Approximate capability = NOT compatible
//!
//!     Approximate requirement + Approximate capability = compatible
//!
//!     Exact requirement + Emulated capability = compatible
//!
//!     Native requirement + Emulated capability = NOT compatible
//!
//! =============================================================================
//! DETERMINISM
//! =============================================================================
//!
//! Capability profiles are pure value objects.
//!
//! This module contains no:
//!
//! - global mutable state;
//! - random state;
//! - hidden clocks;
//! - network calls;
//! - provider handles;
//! - environment-dependent discovery.
//!
//! Capability ordering is deterministic because the underlying collection is
//! ordered.
//!
//! =============================================================================
//! RESOURCE SAFETY
//! =============================================================================
//!
//! No unsafe code is used.
//!
//! This module does not allocate based on target-reported dimensions without
//! caller control. Capability declarations are inserted one at a time or from
//! caller-provided iterators.
//!
//! No operation here materializes a quantum state, tensor, topology, circuit,
//! channel, or fault set.
//!
//! Therefore target capability inspection remains lightweight even when the
//! target represents a very large quantum machine.
//!
//! =============================================================================
//! THREAD SAFETY
//! =============================================================================
//!
//! `TargetCapabilities` contains ordinary owned Rust value types and does not
//! contain mutable shared state, device handles, or synchronization primitives.
//!
//! It is therefore suitable for immutable sharing across threads when the
//! contained repository types satisfy the same standard Rust bounds.
//!
//! =============================================================================
//! SERIALIZATION BOUNDARY
//! =============================================================================
//!
//! This file deliberately does not define a serialization format.
//!
//! Canonical ZQN serialization belongs to:
//!
//!     zqn::io
//!
//! The public value objects in this file can therefore be serialized by a
//! future schema layer without coupling the target abstraction to a particular
//! wire format.
//!
//! =============================================================================
//! VERSIONING
//! =============================================================================
//!
//! Capability identifiers are stable semantic identifiers.
//!
//! Adding a new capability must not break this file.
//!
//! Existing identifiers must not be renamed merely because a new backend is
//! introduced.
//!
//! Provider-specific capabilities should use provider namespaces rather than
//! modifying this module.
//!
//! =============================================================================
//! ERROR POLICY
//! =============================================================================
//!
//! This module uses explicit, structured errors for invalid capability-profile
//! operations and explicit compatibility reports for unsatisfied requirements.
//!
//! It never silently:
//!
//! - drops a capability;
//! - converts approximate support into exact support;
//! - replaces a missing capability with a different capability;
//! - invents target support;
//! - ignores scope mismatches.
//!
//! =============================================================================
//! INTEGRATION CONTRACT
//! =============================================================================
//!
//! Producers:
//!
//! - hardware adapters;
//! - simulators;
//! - emulators;
//! - characterization systems;
//! - logical/fault-tolerant targets;
//! - distributed quantum targets;
//! - future quantum technologies.
//!
//! Consumers:
//!
//! - zqn::target::requirements;
//! - zqn::target::compatibility;
//! - zqn::target::lowering;
//! - zqn::target::validation;
//! - zqn::noise;
//! - zqn::channel;
//! - zqn::simulation;
//! - zqn::calibration;
//! - zqn::integration::routing;
//! - zqn::integration::scheduling;
//! - zqn::integration::qec;
//! - zqn::integration::hardware;
//! - zqn::integration::runtime;
//! - zqn::integration::benchmarking.
//!
//! None of those modules are imported here. This keeps the dependency
//! direction acyclic and allows this file to be completed independently.
//!
//! =============================================================================
//! RUST COMPATIBILITY
//! =============================================================================
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe.
//!
//! =============================================================================

#![forbid(unsafe_code)]

use std::collections::BTreeSet;
use std::fmt;

use super::super::core::capabilities::{
    evaluate_requirement,
    Capability,
    CapabilityId,
    CapabilityMatch,
    CapabilityRequirement,
    CapabilityScope,
    CapabilitySet,
    SupportLevel,
    SupportRequirement,
};

// =============================================================================
// Target capability policy
// =============================================================================

/// Policy used when evaluating a target against ZQN capability requirements.
///
/// The policy is deliberately independent from the capability declaration
/// itself.
///
/// A capability describes what a target provides.
///
/// A policy describes what the caller is willing to accept.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum TargetCapabilityPolicy {
    /// Accept only native realizations.
    NativeOnly,

    /// Accept native or exact emulated realizations.
    ///
    /// Approximation is rejected.
    #[default]
    ExactOnly,

    /// Permit explicit approximations when the requirement itself permits
    /// approximation.
    AllowApproximate,

    /// Accept any realization that the requirement itself permits.
    AnyAllowed,
}

impl TargetCapabilityPolicy {
    /// Returns whether approximate support may be accepted by this policy.
    #[must_use]
    pub const fn allows_approximate(self) -> bool {
        matches!(
            self,
            Self::AllowApproximate | Self::AnyAllowed
        )
    }

    /// Returns whether emulated exact support may be accepted.
    #[must_use]
    pub const fn allows_emulation(self) -> bool {
        !matches!(self, Self::NativeOnly)
    }

    /// Returns whether native support is required regardless of the
    /// requirement's declared support policy.
    #[must_use]
    pub const fn requires_native(self) -> bool {
        matches!(self, Self::NativeOnly)
    }
}

impl fmt::Display for TargetCapabilityPolicy {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NativeOnly => formatter.write_str("native-only"),
            Self::ExactOnly => formatter.write_str("exact-only"),
            Self::AllowApproximate => formatter.write_str("allow-approximate"),
            Self::AnyAllowed => formatter.write_str("any-allowed"),
        }
    }
}

// =============================================================================
// Target capability identity
// =============================================================================

/// Stable value describing a target capability profile.
///
/// This is intentionally NOT a backend identity.
///
/// A provider, backend name, device name, serial number, URI, credential, or
/// network address belongs to the hardware/backend subsystem.
///
/// Two completely different execution targets may legitimately have equal
/// `TargetCapabilities` when they expose the same ZQN capability contract.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TargetCapabilities {
    capabilities: CapabilitySet,
}

impl TargetCapabilities {
    /// Creates an empty target capability profile.
    ///
    /// An empty profile is valid as a value object. Compatibility evaluation
    /// will naturally report missing requirements.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a target profile from an existing ZQN capability set.
    #[must_use]
    pub fn from_capabilities(capabilities: CapabilitySet) -> Self {
        Self { capabilities }
    }

    /// Creates a target profile from an iterator of capability declarations.
    pub fn from_iter<I>(capabilities: I) -> Self
    where
        I: IntoIterator<Item = Capability>,
    {
        Self {
            capabilities: CapabilitySet::from_iter(capabilities),
        }
    }

    /// Returns the underlying capability set.
    ///
    /// The returned value is immutable from the caller's perspective, so
    /// callers cannot mutate the profile through this reference.
    #[must_use]
    pub const fn capabilities(&self) -> &CapabilitySet {
        &self.capabilities
    }

    /// Returns the number of capability declarations.
    ///
    /// This is representation size only. It is not a hardware capacity limit.
    #[must_use]
    pub fn len(&self) -> usize {
        self.capabilities.len()
    }

    /// Returns whether the profile contains no capability declarations.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.capabilities.is_empty()
    }

    /// Inserts one capability declaration.
    ///
    /// Returns `true` if the profile changed.
    ///
    /// This method does not impose any maximum number of capabilities.
    pub fn insert(&mut self, capability: Capability) -> bool {
        self.capabilities.insert(capability)
    }

    /// Removes an exact capability declaration.
    ///
    /// Returns `true` if the declaration existed.
    pub fn remove(&mut self, capability: &Capability) -> bool {
        self.capabilities.remove(capability)
    }

    /// Returns whether the exact declaration exists.
    #[must_use]
    pub fn contains(&self, capability: &Capability) -> bool {
        self.capabilities.contains(capability)
    }

    /// Returns all declarations in deterministic order.
    pub fn iter(&self) -> impl Iterator<Item = &Capability> {
        self.capabilities.iter()
    }

    /// Returns all declarations for one capability identifier.
    pub fn by_id<'a>(
        &'a self,
        id: &'a CapabilityId,
    ) -> impl Iterator<Item = &'a Capability> + 'a {
        self.capabilities.by_id(id)
    }

    /// Returns the first declaration matching the exact capability identity
    /// and scope, regardless of support level.
    #[must_use]
    pub fn find(
        &self,
        id: &CapabilityId,
        scope: &CapabilityScope,
    ) -> Option<&Capability> {
        self.capabilities
            .by_id(id)
            .find(|capability| capability.scope() == scope)
    }

    /// Returns whether a requirement is satisfied under the requirement's own
    /// support semantics.
    #[must_use]
    pub fn satisfies(&self, requirement: &CapabilityRequirement) -> bool {
        self.capabilities.satisfies(requirement)
    }

    /// Returns whether every supplied requirement is satisfied.
    #[must_use]
    pub fn satisfies_all(
        &self,
        requirements: &[CapabilityRequirement],
    ) -> bool {
        self.capabilities.satisfies_all(requirements)
    }

    /// Returns all missing requirements.
    #[must_use]
    pub fn missing(
        &self,
        requirements: &[CapabilityRequirement],
    ) -> Vec<CapabilityRequirement> {
        self.capabilities.missing(requirements)
    }

    /// Evaluates one requirement while preserving the distinction between
    /// exact, approximate, and missing support.
    #[must_use]
    pub fn evaluate(
        &self,
        requirement: &CapabilityRequirement,
    ) -> CapabilityMatch {
        evaluate_requirement(&self.capabilities, requirement)
    }

    /// Evaluates one requirement using an explicit target policy.
    #[must_use]
    pub fn evaluate_with_policy(
        &self,
        requirement: &CapabilityRequirement,
        policy: TargetCapabilityPolicy,
    ) -> TargetCapabilityMatch {
        let capability = self.find(requirement.id(), requirement.scope());

        let Some(capability) = capability else {
            return TargetCapabilityMatch::Missing {
                requirement: requirement.clone(),
            };
        };

        if !policy_accepts(policy, requirement.support(), capability.support()) {
            return TargetCapabilityMatch::Rejected {
                requirement: requirement.clone(),
                capability: capability.clone(),
                reason: PolicyRejectionReason::PolicyDoesNotAcceptSupport,
            };
        }

        if requirement.support().accepts(capability.support()) {
            if capability.support().is_approximate() {
                TargetCapabilityMatch::Approximate {
                    requirement: requirement.clone(),
                    capability: capability.clone(),
                }
            } else {
                TargetCapabilityMatch::Satisfied {
                    requirement: requirement.clone(),
                    capability: capability.clone(),
                }
            }
        } else {
            TargetCapabilityMatch::Rejected {
                requirement: requirement.clone(),
                capability: capability.clone(),
                reason: PolicyRejectionReason::RequirementNotSatisfied,
            }
        }
    }

    /// Evaluates all requirements under an explicit policy.
    #[must_use]
    pub fn compatibility(
        &self,
        requirements: &[CapabilityRequirement],
        policy: TargetCapabilityPolicy,
    ) -> TargetCompatibility {
        let mut results = Vec::with_capacity(requirements.len());

        for requirement in requirements {
            results.push(
                self.evaluate_with_policy(requirement, policy),
            );
        }

        TargetCompatibility::from_results(policy, results)
    }

    /// Returns the capability declarations as a deterministic vector.
    #[must_use]
    pub fn to_vec(&self) -> Vec<Capability> {
        self.capabilities.to_vec()
    }

    /// Extends this profile with all declarations from another profile.
    ///
    /// Existing identical declarations remain deduplicated by the underlying
    /// `BTreeSet`.
    pub fn extend(&mut self, other: &TargetCapabilities) {
        for capability in other.iter() {
            self.insert(capability.clone());
        }
    }

    /// Creates a new profile containing the union of two profiles.
    #[must_use]
    pub fn union(&self, other: &TargetCapabilities) -> Self {
        let mut result = self.clone();
        result.extend(other);
        result
    }
}

// =============================================================================
// Policy evaluation
// =============================================================================

fn policy_accepts(
    policy: TargetCapabilityPolicy,
    requirement: SupportRequirement,
    actual: SupportLevel,
) -> bool {
    if !actual.is_supported() {
        return false;
    }

    if policy.requires_native() && !actual.is_native() {
        return false;
    }

    if !policy.allows_emulation() && actual.is_emulated() {
        return false;
    }

    if actual.is_approximate() {
        if !policy.allows_approximate() {
            return false;
        }

        return matches!(
            requirement,
            SupportRequirement::Approximate
                | SupportRequirement::Any
        );
    }

    requirement.accepts(actual)
}

// =============================================================================
// Target capability match
// =============================================================================

/// Result of evaluating one requirement against a target profile.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TargetCapabilityMatch {
    /// Requirement is satisfied exactly.
    Satisfied {
        /// Original requirement.
        requirement: CapabilityRequirement,

        /// Target declaration satisfying it.
        capability: Capability,
    },

    /// Requirement is satisfied only through an explicitly permitted
    /// approximation.
    Approximate {
        /// Original requirement.
        requirement: CapabilityRequirement,

        /// Target declaration providing the approximation.
        capability: Capability,
    },

    /// No declaration exists for the required capability and scope.
    Missing {
        /// Original requirement.
        requirement: CapabilityRequirement,
    },

    /// A declaration exists, but policy or requirement semantics reject it.
    Rejected {
        /// Original requirement.
        requirement: CapabilityRequirement,

        /// Target declaration that was rejected.
        capability: Capability,

        /// Reason for rejection.
        reason: PolicyRejectionReason,
    },
}

impl TargetCapabilityMatch {
    /// Returns whether the requirement is exactly satisfied.
    #[must_use]
    pub const fn is_satisfied(&self) -> bool {
        matches!(self, Self::Satisfied { .. })
    }

    /// Returns whether the requirement is satisfied approximately.
    #[must_use]
    pub const fn is_approximate(&self) -> bool {
        matches!(self, Self::Approximate { .. })
    }

    /// Returns whether no matching declaration exists.
    #[must_use]
    pub const fn is_missing(&self) -> bool {
        matches!(self, Self::Missing { .. })
    }

    /// Returns whether the requirement is rejected.
    #[must_use]
    pub const fn is_rejected(&self) -> bool {
        matches!(self, Self::Rejected { .. })
    }

    /// Returns whether the requirement can be executed under the selected
    /// target policy.
    #[must_use]
    pub const fn is_compatible(&self) -> bool {
        matches!(
            self,
            Self::Satisfied { .. } | Self::Approximate { .. }
        )
    }

    /// Returns the original requirement.
    #[must_use]
    pub const fn requirement(&self) -> &CapabilityRequirement {
        match self {
            Self::Satisfied { requirement, .. }
            | Self::Approximate { requirement, .. }
            | Self::Missing { requirement }
            | Self::Rejected { requirement, .. } => requirement,
        }
    }

    /// Returns the matching target capability when one exists.
    #[must_use]
    pub const fn capability(&self) -> Option<&Capability> {
        match self {
            Self::Satisfied { capability, .. }
            | Self::Approximate { capability, .. }
            | Self::Rejected { capability, .. } => Some(capability),
            Self::Missing { .. } => None,
        }
    }

    /// Returns the rejection reason, if this result was rejected.
    #[must_use]
    pub const fn rejection_reason(
        &self,
    ) -> Option<PolicyRejectionReason> {
        match self {
            Self::Rejected { reason, .. } => Some(*reason),
            _ => None,
        }
    }
}

// =============================================================================
// Policy rejection
// =============================================================================

/// Reason a target capability could not satisfy a requirement under the
/// selected target policy.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum PolicyRejectionReason {
    /// The requirement itself was not satisfied by the support level.
    RequirementNotSatisfied,

    /// The target policy explicitly disallowed the available support level.
    PolicyDoesNotAcceptSupport,
}

impl fmt::Display for PolicyRejectionReason {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RequirementNotSatisfied => {
                formatter.write_str("requirement not satisfied")
            }
            Self::PolicyDoesNotAcceptSupport => {
                formatter.write_str("target policy does not accept available support")
            }
        }
    }
}

// =============================================================================
// Compatibility report
// =============================================================================

/// Complete deterministic compatibility result for a target.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TargetCompatibility {
    policy: TargetCapabilityPolicy,
    results: Vec<TargetCapabilityMatch>,
}

impl TargetCompatibility {
    fn from_results(
        policy: TargetCapabilityPolicy,
        results: Vec<TargetCapabilityMatch>,
    ) -> Self {
        Self { policy, results }
    }

    /// Returns the policy used during evaluation.
    #[must_use]
    pub const fn policy(&self) -> TargetCapabilityPolicy {
        self.policy
    }

    /// Returns all individual requirement results in input order.
    #[must_use]
    pub fn results(&self) -> &[TargetCapabilityMatch] {
        &self.results
    }

    /// Returns whether every requirement is compatible.
    #[must_use]
    pub fn is_compatible(&self) -> bool {
        self.results
            .iter()
            .all(TargetCapabilityMatch::is_compatible)
    }

    /// Returns whether all requirements are exactly satisfied.
    #[must_use]
    pub fn is_exact(&self) -> bool {
        self.results
            .iter()
            .all(TargetCapabilityMatch::is_satisfied)
    }

    /// Returns whether at least one requirement uses approximation.
    #[must_use]
    pub fn uses_approximation(&self) -> bool {
        self.results
            .iter()
            .any(TargetCapabilityMatch::is_approximate)
    }

    /// Returns whether at least one requirement is missing.
    #[must_use]
    pub fn has_missing(&self) -> bool {
        self.results
            .iter()
            .any(TargetCapabilityMatch::is_missing)
    }

    /// Returns whether at least one requirement was rejected.
    #[must_use]
    pub fn has_rejections(&self) -> bool {
        self.results
            .iter()
            .any(TargetCapabilityMatch::is_rejected)
    }

    /// Returns the number of requirements.
    #[must_use]
    pub fn len(&self) -> usize {
        self.results.len()
    }

    /// Returns whether there were no requirements.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.results.is_empty()
    }

    /// Returns all incompatible requirements.
    #[must_use]
    pub fn incompatible(&self) -> Vec<&TargetCapabilityMatch> {
        self.results
            .iter()
            .filter(|result| !result.is_compatible())
            .collect()
    }

    /// Returns all missing requirements.
    #[must_use]
    pub fn missing(&self) -> Vec<&CapabilityRequirement> {
        self.results
            .iter()
            .filter_map(|result| match result {
                TargetCapabilityMatch::Missing { requirement } => {
                    Some(requirement)
                }
                _ => None,
            })
            .collect()
    }

    /// Returns all explicitly approximated requirements.
    #[must_use]
    pub fn approximate(&self) -> Vec<&CapabilityRequirement> {
        self.results
            .iter()
            .filter_map(|result| match result {
                TargetCapabilityMatch::Approximate { requirement, .. } => {
                    Some(requirement)
                }
                _ => None,
            })
            .collect()
    }

    /// Returns a compact deterministic summary.
    #[must_use]
    pub fn summary(&self) -> CompatibilitySummary {
        let mut summary = CompatibilitySummary::default();

        for result in &self.results {
            match result {
                TargetCapabilityMatch::Satisfied { .. } => {
                    summary.satisfied += 1;
                }
                TargetCapabilityMatch::Approximate { .. } => {
                    summary.approximate += 1;
                }
                TargetCapabilityMatch::Missing { .. } => {
                    summary.missing += 1;
                }
                TargetCapabilityMatch::Rejected { .. } => {
                    summary.rejected += 1;
                }
            }
        }

        summary
    }
}

impl fmt::Display for TargetCompatibility {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let summary = self.summary();

        write!(
            formatter,
            "policy={}, satisfied={}, approximate={}, missing={}, rejected={}, compatible={}",
            self.policy,
            summary.satisfied,
            summary.approximate,
            summary.missing,
            summary.rejected,
            self.is_compatible(),
        )
    }
}

// =============================================================================
// Compatibility summary
// =============================================================================

/// Compact compatibility counts.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CompatibilitySummary {
    /// Exactly satisfied requirements.
    pub satisfied: usize,

    /// Explicitly approximated requirements.
    pub approximate: usize,

    /// Requirements for which no matching declaration exists.
    pub missing: usize,

    /// Requirements rejected by support semantics or target policy.
    pub rejected: usize,
}

impl CompatibilitySummary {
    /// Returns the total number of evaluated requirements.
    #[must_use]
    pub const fn total(self) -> usize {
        self.satisfied
            + self.approximate
            + self.missing
            + self.rejected
    }

    /// Returns whether every requirement is compatible.
    #[must_use]
    pub const fn is_compatible(self) -> bool {
        self.missing == 0 && self.rejected == 0
    }

    /// Returns whether every requirement was exact.
    #[must_use]
    pub const fn is_exact(self) -> bool {
        self.approximate == 0
            && self.missing == 0
            && self.rejected == 0
    }
}

// =============================================================================
// Target capability validation
// =============================================================================

/// Errors indicating that a target capability profile itself is malformed.
///
/// These errors are deliberately separate from compatibility errors.
///
/// A valid target can simply fail to support a requirement. That is not a
/// malformed target profile.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TargetCapabilityError {
    /// The profile contains contradictory declarations for the same capability
    /// and scope.
    ConflictingDeclarations {
        /// Capability identifier.
        id: CapabilityId,

        /// Capability scope.
        scope: CapabilityScope,
    },
}

impl fmt::Display for TargetCapabilityError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConflictingDeclarations { id, scope } => {
                write!(
                    formatter,
                    "conflicting capability declarations for {} @ {}",
                    id, scope
                )
            }
        }
    }
}

impl std::error::Error for TargetCapabilityError {}

impl TargetCapabilities {
    /// Validates the profile's semantic consistency.
    ///
    /// The underlying `CapabilitySet` already prevents exact duplicate
    /// declarations. This additional validation rejects conflicting support
    /// declarations for the same identifier and scope.
    ///
    /// A profile such as:
    ///
    ///     zqn.noise.readout @ global -> Native
    ///     zqn.noise.readout @ global -> Unsupported
    ///
    /// is contradictory and therefore rejected.
    pub fn validate(&self) -> Result<(), TargetCapabilityError> {
        let mut previous: Option<&Capability> = None;

        for capability in self.iter() {
            if let Some(previous_capability) = previous {
                if previous_capability.id() == capability.id()
                    && previous_capability.scope() == capability.scope()
                    && previous_capability.support()
                        != capability.support()
                {
                    return Err(
                        TargetCapabilityError::ConflictingDeclarations {
                            id: capability.id().clone(),
                            scope: capability.scope().clone(),
                        },
                    );
                }
            }

            previous = Some(capability);
        }

        Ok(())
    }
}

// =============================================================================
// Capability profile builder
// =============================================================================

/// Builder for constructing a target capability profile.
///
/// The builder owns no external resources and performs no discovery.
///
/// It exists primarily to make backend adapters explicit and deterministic.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TargetCapabilitiesBuilder {
    capabilities: TargetCapabilities,
}

impl TargetCapabilitiesBuilder {
    /// Creates an empty builder.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds one capability.
    #[must_use]
    pub fn capability(mut self, capability: Capability) -> Self {
        self.capabilities.insert(capability);
        self
    }

    /// Adds multiple capabilities.
    #[must_use]
    pub fn capabilities<I>(
        mut self,
        capabilities: I,
    ) -> Self
    where
        I: IntoIterator<Item = Capability>,
    {
        for capability in capabilities {
            self.capabilities.insert(capability);
        }

        self
    }

    /// Adds a native capability.
    #[must_use]
    pub fn native(
        self,
        id: CapabilityId,
        scope: CapabilityScope,
    ) -> Self {
        self.capability(Capability::native(id, scope))
    }

    /// Adds an exact emulated capability.
    #[must_use]
    pub fn emulated(
        self,
        id: CapabilityId,
        scope: CapabilityScope,
    ) -> Self {
        self.capability(Capability::emulated(id, scope))
    }

    /// Adds an explicitly approximate capability.
    #[must_use]
    pub fn approximate(
        self,
        id: CapabilityId,
        scope: CapabilityScope,
    ) -> Self {
        self.capability(Capability::approximate(id, scope))
    }

    /// Finishes construction after validating the resulting profile.
    pub fn build(self) -> Result<TargetCapabilities, TargetCapabilityError> {
        self.capabilities.validate()?;
        Ok(self.capabilities)
    }
}

// =============================================================================
// Capability query helpers
// =============================================================================

/// Returns whether a target has a capability at a particular scope with the
/// requested support level.
///
/// This function is provided as a free function as well as a method so
/// integration code can use a functional style without depending on internal
/// storage.
#[must_use]
pub fn supports(
    target: &TargetCapabilities,
    requirement: &CapabilityRequirement,
) -> bool {
    target.satisfies(requirement)
}

/// Evaluates a target against requirements using the default exact-only policy.
#[must_use]
pub fn check_compatibility(
    target: &TargetCapabilities,
    requirements: &[CapabilityRequirement],
) -> TargetCompatibility {
    target.compatibility(
        requirements,
        TargetCapabilityPolicy::ExactOnly,
    )
}

/// Evaluates a target against requirements under an explicit policy.
#[must_use]
pub fn check_compatibility_with_policy(
    target: &TargetCapabilities,
    requirements: &[CapabilityRequirement],
    policy: TargetCapabilityPolicy,
) -> TargetCompatibility {
    target.compatibility(requirements, policy)
}

// =============================================================================
// Deterministic profile comparison
// =============================================================================

/// Returns whether two target capability profiles expose exactly the same
/// declarations.
///
/// This is semantic equality, not backend identity equality.
#[must_use]
pub fn equivalent(
    left: &TargetCapabilities,
    right: &TargetCapabilities,
) -> bool {
    left == right
}

/// Returns the declarations present in `left` but not `right`.
#[must_use]
pub fn difference(
    left: &TargetCapabilities,
    right: &TargetCapabilities,
) -> Vec<Capability> {
    left.iter()
        .filter(|capability| !right.contains(capability))
        .cloned()
        .collect()
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn capability(name: &str) -> CapabilityId {
        CapabilityId::new(name).expect("test capability identifier must be valid")
    }

    #[test]
    fn empty_target_is_valid() {
        let target = TargetCapabilities::new();

        assert!(target.is_empty());
        assert_eq!(target.len(), 0);
        assert!(target.validate().is_ok());
    }

    #[test]
    fn native_capability_satisfies_native_requirement() {
        let id = capability("zqn.noise.readout");

        let target = TargetCapabilitiesBuilder::new()
            .native(id.clone(), CapabilityScope::Global)
            .build()
            .expect("profile should be valid");

        let requirement =
            CapabilityRequirement::native(id, CapabilityScope::Global);

        assert!(target.satisfies(&requirement));
    }

    #[test]
    fn emulated_capability_satisfies_exact_requirement() {
        let id = capability("zqn.channel.kraus");

        let target = TargetCapabilitiesBuilder::new()
            .emulated(id.clone(), CapabilityScope::Global)
            .build()
            .expect("profile should be valid");

        let requirement =
            CapabilityRequirement::exact(id, CapabilityScope::Global);

        assert!(target.satisfies(&requirement));
    }

    #[test]
    fn approximate_capability_does_not_satisfy_exact_requirement() {
        let id = capability("zqn.noise.custom");

        let target = TargetCapabilitiesBuilder::new()
            .approximate(id.clone(), CapabilityScope::Global)
            .build()
            .expect("profile should be valid");

        let requirement =
            CapabilityRequirement::exact(id, CapabilityScope::Global);

        assert!(!target.satisfies(&requirement));
    }

    #[test]
    fn approximate_capability_requires_explicit_approximation() {
        let id = capability("zqn.noise.custom");

        let target = TargetCapabilitiesBuilder::new()
            .approximate(id.clone(), CapabilityScope::Global)
            .build()
            .expect("profile should be valid");

        let requirement =
            CapabilityRequirement::approximate(id, CapabilityScope::Global);

        let compatibility = target.compatibility(
            &[requirement],
            TargetCapabilityPolicy::AllowApproximate,
        );

        assert!(compatibility.is_compatible());
        assert!(compatibility.uses_approximation());
    }

    #[test]
    fn approximate_support_is_rejected_by_exact_only_policy() {
        let id = capability("zqn.noise.custom");

        let target = TargetCapabilitiesBuilder::new()
            .approximate(id.clone(), CapabilityScope::Global)
            .build()
            .expect("profile should be valid");

        let requirement =
            CapabilityRequirement::approximate(id, CapabilityScope::Global);

        let compatibility = target.compatibility(
            &[requirement],
            TargetCapabilityPolicy::ExactOnly,
        );

        assert!(!compatibility.is_compatible());
        assert!(compatibility.has_rejections());
    }

    #[test]
    fn native_only_rejects_emulation() {
        let id = capability("zqn.channel.kraus");

        let target = TargetCapabilitiesBuilder::new()
            .emulated(id.clone(), CapabilityScope::Global)
            .build()
            .expect("profile should be valid");

        let requirement =
            CapabilityRequirement::exact(id, CapabilityScope::Global);

        let compatibility = target.compatibility(
            &[requirement],
            TargetCapabilityPolicy::NativeOnly,
        );

        assert!(!compatibility.is_compatible());
        assert!(compatibility.has_rejections());
    }

    #[test]
    fn scope_mismatch_is_not_accepted() {
        let id = capability("zqn.noise.readout");

        let target = TargetCapabilitiesBuilder::new()
            .native(id.clone(), CapabilityScope::Global)
            .build()
            .expect("profile should be valid");

        let requirement = CapabilityRequirement::native(
            id,
            CapabilityScope::Resource("measurement:0".to_owned()),
        );

        assert!(!target.satisfies(&requirement));
    }

    #[test]
    fn missing_capability_is_reported() {
        let id = capability("zqn.noise.non_markovian");

        let target = TargetCapabilities::new();

        let requirement =
            CapabilityRequirement::exact(id, CapabilityScope::Global);

        let compatibility = target.compatibility(
            &[requirement],
            TargetCapabilityPolicy::ExactOnly,
        );

        assert!(!compatibility.is_compatible());
        assert!(compatibility.has_missing());
        assert_eq!(compatibility.summary().missing, 1);
    }

    #[test]
    fn multiple_requirements_are_evaluated_deterministically() {
        let readout = capability("zqn.noise.readout");
        let thermal = capability("zqn.noise.thermal");

        let target = TargetCapabilitiesBuilder::new()
            .native(readout.clone(), CapabilityScope::Global)
            .native(thermal.clone(), CapabilityScope::Global)
            .build()
            .expect("profile should be valid");

        let requirements = vec![
            CapabilityRequirement::exact(
                readout,
                CapabilityScope::Global,
            ),
            CapabilityRequirement::exact(
                thermal,
                CapabilityScope::Global,
            ),
        ];

        let compatibility = target.compatibility(
            &requirements,
            TargetCapabilityPolicy::ExactOnly,
        );

        assert!(compatibility.is_compatible());
        assert!(compatibility.is_exact());
        assert_eq!(compatibility.summary().total(), 2);
    }

    #[test]
    fn union_does_not_duplicate_capabilities() {
        let id = capability("zqn.noise.readout");

        let left = TargetCapabilitiesBuilder::new()
            .native(id.clone(), CapabilityScope::Global)
            .build()
            .expect("profile should be valid");

        let right = TargetCapabilitiesBuilder::new()
            .native(id, CapabilityScope::Global)
            .build()
            .expect("profile should be valid");

        let union = left.union(&right);

        assert_eq!(union.len(), 1);
    }

    #[test]
    fn difference_is_deterministic() {
        let readout = capability("zqn.noise.readout");
        let thermal = capability("zqn.noise.thermal");

        let left = TargetCapabilitiesBuilder::new()
            .native(readout.clone(), CapabilityScope::Global)
            .native(thermal.clone(), CapabilityScope::Global)
            .build()
            .expect("profile should be valid");

        let right = TargetCapabilitiesBuilder::new()
            .native(readout, CapabilityScope::Global)
            .build()
            .expect("profile should be valid");

        let difference = difference(&left, &right);

        assert_eq!(difference.len(), 1);
        assert_eq!(difference[0].id(), &thermal);
    }

    #[test]
    fn equivalent_profiles_compare_equal() {
        let id = capability("zqn.noise.readout");

        let left = TargetCapabilitiesBuilder::new()
            .native(id.clone(), CapabilityScope::Global)
            .build()
            .expect("profile should be valid");

        let right = TargetCapabilitiesBuilder::new()
            .native(id, CapabilityScope::Global)
            .build()
            .expect("profile should be valid");

        assert!(equivalent(&left, &right));
    }

    #[test]
    fn explicit_policy_never_turns_approximation_into_exact_support() {
        let id = capability("zqn.future.capability");

        let target = TargetCapabilitiesBuilder::new()
            .approximate(id.clone(), CapabilityScope::Global)
            .build()
            .expect("profile should be valid");

        let exact_requirement =
            CapabilityRequirement::exact(id, CapabilityScope::Global);

        let compatibility = check_compatibility(
            &target,
            &[exact_requirement],
        );

        assert!(!compatibility.is_compatible());
        assert!(!compatibility.is_exact());
    }

    #[test]
    fn profile_can_scale_with_arbitrary_number_of_capabilities() {
        let mut target = TargetCapabilities::new();

        for index in 0usize..1024 {
            let id = capability(
                &format!("zqn.test.generated.{}", index),
            );

            target.insert(Capability::native(
                id,
                CapabilityScope::Resource(
                    format!("resource:{}", index),
                ),
            ));
        }

        assert_eq!(target.len(), 1024);
        assert!(target.validate().is_ok());
    }
}
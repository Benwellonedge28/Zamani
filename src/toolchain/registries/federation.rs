
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! Provider-neutral coordination across configured package registries.
//!
//! Federation owns membership and multi-registry query coordination.
//! Registry configuration, provider execution, protocol encoding,
//! authentication, authorization, package interpretation, and dependency
//! resolution remain owned by their existing subsystems.

use std::collections::BTreeMap;
use std::future::{poll_fn, Future};
use std::num::NonZeroUsize;
use std::pin::Pin;
use std::sync::{Arc, RwLock};
use std::task::Poll;

use crate::toolchain::errors::{
    Result,
    ToolchainError,
    ToolchainErrorKind,
};
use crate::toolchain::ids::{
    ProviderId,
    RegistryId,
};
use crate::toolchain::version::{
    RegistryProtocolVersion,
    VersionRange,
};

use super::client::RegistryClient;
use super::provider::{
    RegistryOperationId,
    RegistryRequest,
    RegistryResponse,
};
use super::registry::RegistrySnapshot;

const OPERATION_FEDERATION: &str = "registry.federation";
const OPERATION_FEDERATION_ADD: &str = "registry.federation.add_member";
const OPERATION_FEDERATION_REMOVE: &str = "registry.federation.remove_member";
const OPERATION_FEDERATION_QUERY: &str = "registry.federation.query";

/// A registry participating in a federation.
///
/// Priority controls deterministic ordering only. Lower values are
/// processed first. It is not a reliability, trust, or compatibility score.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FederationMember {
    registry_id: RegistryId,
    priority: i64,
}

impl FederationMember {
    /// Create a federation member.
    pub fn new(registry_id: RegistryId, priority: i64) -> Self {
        Self {
            registry_id,
            priority,
        }
    }

    /// Return the registry identity.
    pub fn registry_id(&self) -> &RegistryId {
        &self.registry_id
    }

    /// Return the configured ordering priority.
    pub const fn priority(&self) -> i64 {
        self.priority
    }
}

/// A configured federation of registry instances.
///
/// The registry client remains the authority for registry snapshots,
/// provider registrations, compatibility validation, and execution.
pub struct Federation {
    client: Arc<RegistryClient>,
    members: RwLock<BTreeMap<RegistryId, FederationMember>>,
}

impl Federation {
    /// Create an empty federation using an existing registry client.
    pub fn new(client: Arc<RegistryClient>) -> Self {
        Self {
            client,
            members: RwLock::new(BTreeMap::new()),
        }
    }

    /// Add a registry to the federation.
    ///
    /// Registry and provider validity are checked when an operation is
    /// executed. This permits membership configuration to survive temporary
    /// provider unavailability or registry lifecycle changes.
    ///
    /// # Errors
    ///
    /// Returns an error if the registry identity is already a member or
    /// the membership lock is poisoned.
    pub fn add_member(&self, member: FederationMember) -> Result<()> {
        let mut members = self.members.write().map_err(|_| {
            federation_lock_error(OPERATION_FEDERATION_ADD)
        })?;

        let registry_id = member.registry_id.clone();

        if members.contains_key(&registry_id) {
            return Err(
                ToolchainError::new(
                    ToolchainErrorKind::AlreadyExists,
                    "The registry is already a member of this federation.",
                )
                .with_operation(OPERATION_FEDERATION_ADD)
                .with_subject(registry_id.to_string()),
            );
        }

        members.insert(registry_id, member);
        Ok(())
    }

    /// Remove a registry from the federation.
    ///
    /// Returns `true` when a member was removed and `false` when it was
    /// not present.
    ///
    /// # Errors
    ///
    /// Returns an error if the membership lock is poisoned.
    pub fn remove_member(&self, registry_id: &RegistryId) -> Result<bool> {
        let mut members = self.members.write().map_err(|_| {
            federation_lock_error(OPERATION_FEDERATION_REMOVE)
        })?;

        Ok(members.remove(registry_id).is_some())
    }

    /// Return a deterministic snapshot of federation membership.
    ///
    /// Members are ordered by ascending priority, then by registry identity.
    ///
    /// # Errors
    ///
    /// Returns an error if the membership lock is poisoned.
    pub fn members(&self) -> Result<Vec<FederationMember>> {
        let members = self.members.read().map_err(|_| {
            federation_lock_error(OPERATION_FEDERATION)
        })?;

        let mut result: Vec<_> = members.values().cloned().collect();

        result.sort_by(|left, right| {
            left.priority
                .cmp(&right.priority)
                .then_with(|| left.registry_id.cmp(&right.registry_id))
        });

        Ok(result)
    }

    /// Execute a query against federation members.
    ///
    /// Each registry receives the same operation and opaque payload, using
    /// the highest protocol version compatible with:
    ///
    /// - The query's requested protocol ranges.
    /// - The registry's configured protocol ranges.
    /// - The registered provider's supported protocol ranges.
    ///
    /// Results are retained independently. Federation does not merge package
    /// metadata, resolve dependency conflicts, or interpret payload bytes.
    ///
    /// The caller supplies the authorization policy and operational
    /// concurrency budget.
    ///
    /// # Errors
    ///
    /// Returns an error if the membership lock cannot be read or if the
    /// federation cannot construct its execution snapshot. Individual
    /// registry failures are returned inside the report.
    pub async fn execute(
        &self,
        query: &FederationQuery,
        policy: &dyn FederationAccessPolicy,
        options: FederationExecutionOptions,
    ) -> Result<FederationReport> {
        let members = self.members()?;

        if members.is_empty() {
            return Ok(FederationReport::new(
                FederationCompletion::Empty,
                Vec::new(),
            ));
        }

        let mut outcomes = Vec::with_capacity(members.len());

        for batch in members.chunks(options.max_in_flight.get()) {
            let futures = batch
                .iter()
                .cloned()
                .map(|member| {
                    Box::pin(execute_member(
                        &self.client,
                        policy,
                        query,
                        member,
                    ))
                })
                .collect();

            let mut batch_outcomes = join_all(futures).await;
            outcomes.append(&mut batch_outcomes);
        }

        let succeeded = outcomes
            .iter()
            .filter(|outcome| {
                matches!(
                    outcome.outcome,
                    FederationMemberOutcome::Succeeded { .. }
                )
            })
            .count();

        let completion = if succeeded == outcomes.len() {
            FederationCompletion::Complete
        } else if succeeded == 0 {
            FederationCompletion::NoneSucceeded
        } else {
            FederationCompletion::Partial
        };

        Ok(FederationReport::new(completion, outcomes))
    }
}

/// An immutable federated operation request.
///
/// Payloads are opaque to federation. Their format and interpretation belong
/// to the operation-specific protocol implementation.
#[derive(Debug, Clone)]
pub struct FederationQuery {
    operation: RegistryOperationId,
    requested_protocols: Vec<VersionRange<RegistryProtocolVersion>>,
    payload: Arc<[u8]>,
}

impl FederationQuery {
    /// Construct a query with one or more acceptable protocol ranges.
    ///
    /// Empty or invalid ranges are rejected. An empty range list never
    /// means universal protocol support.
    pub fn new(
        operation: RegistryOperationId,
        requested_protocols: Vec<VersionRange<RegistryProtocolVersion>>,
        payload: impl Into<Arc<[u8]>>,
    ) -> Result<Self> {
        if requested_protocols.is_empty() {
            return Err(
                ToolchainError::new(
                    ToolchainErrorKind::InvalidInput,
                    "At least one requested registry protocol range is required.",
                )
                .with_operation(OPERATION_FEDERATION_QUERY),
            );
        }

        for range in &requested_protocols {
            range.validate().map_err(|error| {
                ToolchainError::new(
                    ToolchainErrorKind::InvalidInput,
                    "The query contains an invalid protocol range.",
                )
                .with_operation(OPERATION_FEDERATION_QUERY)
                .with_source(error)
            })?;
        }

        Ok(Self {
            operation,
            requested_protocols,
            payload: payload.into(),
        })
    }

    /// Construct a query requiring one exact protocol version.
    pub fn for_protocol(
        operation: RegistryOperationId,
        protocol_version: RegistryProtocolVersion,
        payload: impl Into<Arc<[u8]>>,
    ) -> Result<Self> {
        let exact_range =
            VersionRange::new(protocol_version, protocol_version).map_err(
                |error| {
                    ToolchainError::new(
                        ToolchainErrorKind::InvalidInput,
                        "The requested exact protocol version is invalid.",
                    )
                    .with_operation(OPERATION_FEDERATION_QUERY)
                    .with_source(error)
                },
            )?;

        Self::new(operation, vec![exact_range], payload)
    }

    /// Return the requested operation.
    pub fn operation(&self) -> &RegistryOperationId {
        &self.operation
    }

    /// Return the requested protocol ranges.
    pub fn requested_protocols(
        &self,
    ) -> &[VersionRange<RegistryProtocolVersion>] {
        &self.requested_protocols
    }

    /// Borrow the opaque request payload.
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }
}

/// Authorization policy boundary for federated registry operations.
///
/// Implementations must delegate to the canonical authentication,
/// authorization, and trust authorities. Federation does not own credentials,
/// trust stores, identity verification, or access-control policy.
///
/// The policy must also decide whether the requested operation is permitted
/// in this context. Federation cannot infer mutation semantics from an
/// open-world operation identifier.
pub trait FederationAccessPolicy: Send + Sync {
    /// Authorize one operation against one registry and provider.
    fn authorize<'a>(
        &'a self,
        registry_id: &'a RegistryId,
        provider_id: &'a ProviderId,
        operation: &'a RegistryOperationId,
    ) -> FederationPolicyFuture<'a>;
}

/// Future returned by a federation access policy.
pub type FederationPolicyFuture<'a> =
    Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>>;

/// Caller-configured federation execution limits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FederationExecutionOptions {
    max_in_flight: NonZeroUsize,
}

impl FederationExecutionOptions {
    /// Create execution options with a nonzero in-flight request budget.
    pub const fn new(max_in_flight: NonZeroUsize) -> Self {
        Self { max_in_flight }
    }

    /// Return the configured in-flight request budget.
    pub const fn max_in_flight(self) -> NonZeroUsize {
        self.max_in_flight
    }
}

impl Default for FederationExecutionOptions {
    fn default() -> Self {
        Self {
            max_in_flight: NonZeroUsize::MIN,
        }
    }
}

/// Overall completion state of a federated operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FederationCompletion {
    /// No registries were configured as federation members.
    Empty,

    /// Every federation member returned a successful response.
    Complete,

    /// At least one member succeeded and at least one member failed.
    Partial,

    /// No federation member returned a successful response.
    NoneSucceeded,
}

/// Stage at which an individual registry operation failed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FederationFailureStage {
    /// Registry configuration could not be retrieved.
    RegistryLookup,

    /// Registry is not enabled for operations.
    RegistryLifecycle,

    /// Provider descriptor could not be retrieved.
    ProviderLookup,

    /// Registry/provider binding validation failed.
    ProviderCompatibility,

    /// No common protocol version could be selected.
    ProtocolNegotiation,

    /// Authorization policy denied or could not authorize the operation.
    Authorization,

    /// Provider execution or response validation failed.
    Execution,
}

/// Result of one federation member.
#[derive(Debug)]
pub struct FederationMemberResult {
    member: FederationMember,
    outcome: FederationMemberOutcome,
}

/// Success or failure for one federation member.
#[derive(Debug)]
pub enum FederationMemberOutcome {
    /// The registry returned a validated response.
    Succeeded {
        /// Exact protocol version used for the operation.
        protocol_version: RegistryProtocolVersion,

        /// Provider response, retaining its registry identity and payload.
        response: RegistryResponse,
    },

    /// The registry operation failed at a known stage.
    Failed {
        /// Stage at which the failure occurred.
        stage: FederationFailureStage,

        /// Original structured toolchain error.
        error: ToolchainError,
    },
}

impl FederationMemberResult {
    /// Return the federation member associated with this result.
    pub fn member(&self) -> &FederationMember {
        &self.member
    }

    /// Borrow this member's outcome.
    pub fn outcome(&self) -> &FederationMemberOutcome {
        &self.outcome
    }

    /// Consume this result and return its outcome.
    pub fn into_outcome(self) -> FederationMemberOutcome {
        self.outcome
    }
}

/// Complete report from a federated operation.
#[derive(Debug)]
pub struct FederationReport {
    completion: FederationCompletion,
    outcomes: Vec<FederationMemberResult>,
}

impl FederationReport {
    fn new(
        completion: FederationCompletion,
        outcomes: Vec<FederationMemberResult>,
    ) -> Self {
        Self {
            completion,
            outcomes,
        }
    }

    /// Return the overall completion state.
    pub const fn completion(&self) -> FederationCompletion {
        self.completion
    }

    /// Borrow all per-registry outcomes in deterministic order.
    pub fn outcomes(&self) -> &[FederationMemberResult] {
        &self.outcomes
    }

    /// Return the number of successful registry operations.
    pub fn success_count(&self) -> usize {
        self.outcomes
            .iter()
            .filter(|outcome| {
                matches!(
                    outcome.outcome,
                    FederationMemberOutcome::Succeeded { .. }
                )
            })
            .count()
    }

    /// Return the number of failed registry operations.
    pub fn failure_count(&self) -> usize {
        self.outcomes.len() - self.success_count()
    }
}

async fn execute_member(
    client: &RegistryClient,
    policy: &dyn FederationAccessPolicy,
    query: &FederationQuery,
    member: FederationMember,
) -> FederationMemberResult {
    let result = execute_member_inner(client, policy, query, &member).await;

    FederationMemberResult {
        member,
        outcome: match result {
            Ok((protocol_version, response)) => {
                FederationMemberOutcome::Succeeded {
                    protocol_version,
                    response,
                }
            }
            Err((stage, error)) => {
                FederationMemberOutcome::Failed { stage, error }
            }
        },
    }
}

async fn execute_member_inner(
    client: &RegistryClient,
    policy: &dyn FederationAccessPolicy,
    query: &FederationQuery,
    member: &FederationMember,
) -> core::result::Result<
    (RegistryProtocolVersion, RegistryResponse),
    (FederationFailureStage, ToolchainError),
> {
    let registry_id = member.registry_id();

    let snapshot = client
        .registry_snapshot(registry_id)
        .map_err(|error| (FederationFailureStage::RegistryLookup, error))?
        .ok_or_else(|| {
            (
                FederationFailureStage::RegistryLookup,
                ToolchainError::new(
                    ToolchainErrorKind::NotFound,
                    "Federation member is not present in the registry catalog.",
                )
                .with_operation(OPERATION_FEDERATION_QUERY)
                .with_subject(registry_id.to_string()),
            )
        })?;

    validate_registry_lifecycle(&snapshot)?;

    let provider_id = snapshot
        .registry()
        .provider()
        .provider_id()
        .clone();

    let descriptor = client
        .provider_descriptor(&provider_id)
        .map_err(|error| (FederationFailureStage::ProviderLookup, error))?;

    snapshot
        .registry()
        .validate_provider(&descriptor)
        .map_err(|error| {
            (FederationFailureStage::ProviderCompatibility, error)
        })?;

    if !descriptor.supports_operation(query.operation()) {
        return Err((
            FederationFailureStage::ProviderCompatibility,
            ToolchainError::new(
                ToolchainErrorKind::UnsupportedOperation,
                "The registry provider does not declare support for this operation.",
            )
            .with_operation(OPERATION_FEDERATION_QUERY)
            .with_subject(provider_id.to_string()),
        ));
    }

    let protocol_version = negotiate_protocol(
        query.requested_protocols(),
        snapshot.registry().provider().supported_protocols(),
        descriptor.supported_protocols(),
    )
    .ok_or_else(|| {
        (
            FederationFailureStage::ProtocolNegotiation,
            ToolchainError::new(
                ToolchainErrorKind::IncompatibleVersion,
                "No protocol version is shared by the query, registry, and provider.",
            )
            .with_operation(OPERATION_FEDERATION_QUERY)
            .with_subject(registry_id.to_string()),
        )
    })?;

    policy
        .authorize(registry_id, &provider_id, query.operation())
        .await
        .map_err(|error| (FederationFailureStage::Authorization, error))?;

    let request = RegistryRequest::new(
        registry_id.clone(),
        protocol_version,
        query.operation().clone(),
        Arc::clone(&query.payload),
    );

    let response = client
        .execute(request)
        .await
        .map_err(|error| (FederationFailureStage::Execution, error))?;

    Ok((protocol_version, response))
}

fn validate_registry_lifecycle(
    snapshot: &RegistrySnapshot,
) -> core::result::Result<(), (FederationFailureStage, ToolchainError)> {
    if snapshot.registry().is_selectable() {
        return Ok(());
    }

    Err((
        FederationFailureStage::RegistryLifecycle,
        ToolchainError::new(
            ToolchainErrorKind::InvalidConfiguration,
            "The federation member registry is not enabled for operations.",
        )
        .with_operation(OPERATION_FEDERATION_QUERY)
        .with_subject(snapshot.registry().id().to_string()),
    ))
}

/// Select the highest protocol version contained in at least one range
/// from each of the three independently-owned compatibility declarations.
///
/// Candidate versions are taken from declared range boundaries. This avoids
/// inventing versions or assuming that every version number is implemented.
fn negotiate_protocol(
    requested: &[VersionRange<RegistryProtocolVersion>],
    registry: &[VersionRange<RegistryProtocolVersion>],
    provider: &[VersionRange<RegistryProtocolVersion>],
) -> Option<RegistryProtocolVersion> {
    if requested.is_empty() || registry.is_empty() || provider.is_empty() {
        return None;
    }

    let mut candidates = Vec::new();

    for range in requested.iter().chain(registry).chain(provider) {
        candidates.push(range.minimum());
        candidates.push(range.maximum());
    }

    candidates
        .into_iter()
        .filter(|candidate| {
            requested.iter().any(|range| range.contains(*candidate))
                && registry.iter().any(|range| range.contains(*candidate))
                && provider.iter().any(|range| range.contains(*candidate))
        })
        .max()
}

type MemberFuture<'a> =
    Pin<Box<dyn Future<Output = FederationMemberResult> + Send + 'a>>;

/// Await all futures in one bounded batch without requiring an external
/// async runtime or futures crate.
async fn join_all(
    futures: Vec<MemberFuture<'_>>,
) -> Vec<FederationMemberResult> {
    let mut futures: Vec<_> = futures.into_iter().map(Some).collect();
    let mut outputs: Vec<Option<FederationMemberResult>> =
        (0..futures.len()).map(|_| None).collect();
    let mut remaining = futures.len();

    poll_fn(|context| {
        for index in 0..futures.len() {
            let Some(future) = futures[index].as_mut() else {
                continue;
            };

            if let Poll::Ready(output) = future.as_mut().poll(context) {
                outputs[index] = Some(output);
                futures[index] = None;
                remaining -= 1;
            }
        }

        if remaining == 0 {
            let completed = std::mem::take(&mut outputs)
                .into_iter()
                .map(|output| {
                    output.expect("completed future must have an output")
                })
                .collect();

            Poll::Ready(completed)
        } else {
            Poll::Pending
        }
    })
    .await
}

fn federation_lock_error(operation: &str) -> ToolchainError {
    ToolchainError::new(
        ToolchainErrorKind::Internal,
        "The federation membership lock could not be acquired.",
    )
    .with_operation(operation)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn version(
        major: u32,
        minor: u32,
        patch: u32,
    ) -> RegistryProtocolVersion {
        RegistryProtocolVersion::new(major, minor, patch)
    }

    fn range(
        minimum: RegistryProtocolVersion,
        maximum: RegistryProtocolVersion,
    ) -> VersionRange<RegistryProtocolVersion> {
        VersionRange::new(minimum, maximum)
            .expect("test protocol range must be valid")
    }

    #[test]
    fn negotiates_highest_common_protocol_version() {
        let requested = vec![range(version(1, 0, 0), version(3, 0, 0))];
        let registry = vec![range(version(1, 5, 0), version(2, 5, 0))];
        let provider = vec![range(version(2, 0, 0), version(4, 0, 0))];

        assert_eq!(
            negotiate_protocol(&requested, &registry, &provider),
            Some(version(2, 5, 0)),
        );
    }

    #[test]
    fn rejects_protocol_ranges_without_intersection() {
        let requested = vec![range(version(1, 0, 0), version(1, 9, 0))];
        let registry = vec![range(version(2, 0, 0), version(2, 9, 0))];
        let provider = vec![range(version(1, 0, 0), version(3, 0, 0))];

        assert_eq!(
            negotiate_protocol(&requested, &registry, &provider),
            None,
        );
    }

    #[test]
    fn rejects_empty_protocol_declarations() {
        assert_eq!(
            negotiate_protocol(
                &[],
                &[range(version(1, 0, 0), version(2, 0, 0))],
                &[range(version(1, 0, 0), version(2, 0, 0))],
            ),
            None,
        );
    }

    #[test]
    fn protocol_negotiation_handles_disjoint_ranges() {
        let requested = vec![
            range(version(1, 0, 0), version(1, 5, 0)),
            range(version(3, 0, 0), version(3, 5, 0)),
        ];
        let registry = vec![range(version(3, 2, 0), version(4, 0, 0))];
        let provider = vec![range(version(3, 0, 0), version(3, 4, 0))];

        assert_eq!(
            negotiate_protocol(&requested, &registry, &provider),
            Some(version(3, 4, 0)),
        );
    }
}

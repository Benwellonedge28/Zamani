
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Toolchain — Registry Subsystem
//!
//! Path: `src/toolchain/registries/mod.rs`
//!
//! Rust edition: 2021.
//! Minimum supported Rust: 1.97.
//! Safety: Safe Rust only. Unsafe code is prohibited.
//!
//! # Purpose
//!
//! This module is the composition root and public API boundary for
//! Zamani's package registry infrastructure.
//!
//! It coordinates independently maintained registry components:
//!
//! - Provider contracts and provider implementations.
//! - Registry configuration and lifecycle.
//! - Provider discovery.
//! - Registry discovery.
//! - Provider-neutral registry communication.
//! - Registry protocol encoding and negotiation.
//! - Authentication contracts.
//! - Registry validation.
//! - Package downloading.
//! - Package publishing.
//! - Multi-registry federation.
//!
//! # Architectural ownership
//!
//! This module owns:
//!
//! - Registry subsystem composition.
//! - Public registry API organization.
//! - Explicit public re-exports.
//! - Registry subsystem integration boundaries.
//!
//! It does not own:
//!
//! - Package identity or dependency resolution.
//! - Package archive implementation.
//! - Package installation.
//! - Manifest parsing.
//! - Cryptographic implementation.
//! - Credential storage.
//! - Trust-store implementation.
//! - Network transport.
//! - HTTP clients.
//! - Compiler execution.
//! - Backend selection.
//! - Hardware discovery.
//! - Resource allocation.
//! - Runtime execution.
//!
//! Those responsibilities remain with their canonical authorities.
//!
//! # Canonical integration authorities
//!
//! Registry identity:
//!     crate::toolchain::ids::RegistryId
//!
//! Provider identity:
//!     crate::toolchain::ids::ProviderId
//!
//! Package identity:
//!     crate::toolchain::ids::PackageId
//!
//! Registry protocol and provider API versions:
//!     crate::toolchain::version
//!
//! Version compatibility:
//!     crate::toolchain::version::VersionRange
//!
//! Structured errors:
//!     crate::toolchain::errors
//!
//! Dependency resolution:
//!     crate::toolchain::dependencies
//!
//! Package representation:
//!     crate::toolchain::packages::model
//!
//! Package verification:
//!     crate::toolchain::packages::verification
//!
//! Cryptographic and supply-chain security:
//!     crate::toolchain::security
//!
//! Project configuration:
//!     crate::zamani_project_config
//!
//! # Registry architecture
//!
//! ```text
//!                  Zamani Toolchain
//!                         |
//!                         v
//!                  Registry Subsystem
//!                         |
//!          +--------------+--------------+
//!          |              |              |
//!          v              v              v
//!       Registry       Provider       Protocol
//!      Configuration   Contracts     Negotiation
//!          |              |              |
//!          +--------------+--------------+
//!                         |
//!                         v
//!                 Registry Discovery
//!                         |
//!                         v
//!                 Registry Validation
//!                         |
//!                         v
//!                  Registry Client
//!                         |
//!             +-----------+-----------+
//!             |           |           |
//!             v           v           v
//!        Authentication Download  Publishing
//!             |           |           |
//!             +-----------+-----------+
//!                         |
//!                         v
//!                     Federation
//!                         |
//!                         v
//!               Registered Providers
//!                         |
//!                         v
//!               Provider Implementation
//!                         |
//!                         v
//!             Local or Remote Registry
//! ```
//!
//! Federation is an optional coordination layer. It does not replace
//! individual registry configuration, provider validation or
//! operation authorization.
//!
//! # POCO-REAF and extensibility
//!
//! Registries distribute packages independently of their computational
//! domain.
//!
//! Supported and future package domains include classical, quantum,
//! HDL, AI, tensor, hybrid, distributed, networking, cryptography,
//! embedded, accelerator and other computational domains.
//!
//! This subsystem deliberately maintains no closed domain enumeration.
//!
//! New registry providers, protocol implementations, operations and
//! computational domains must be introducible through their respective
//! contracts without redesigning this composition root.
//!
//! There are no arbitrary universal maximums for:
//!
//! - Registry instances.
//! - Registry providers.
//! - Federated participants.
//! - Operations.
//! - Protocol versions.
//! - Package counts.
//! - Computational domains.
//!
//! Actual resource consumption remains subject to available resources
//! and explicitly configured operational budgets.
//!
//! Literal infinite storage, network throughput or computation is
//! not promised.
//!
//! # Concurrency
//!
//! Individual components own their synchronization.
//!
//! This module does not introduce a second global registry lock.
//!
//! Provider execution, network operations and authentication must
//! never be performed while holding registry catalog locks.
//!
//! The caller supplies asynchronous runtime integration, deadlines,
//! cancellation, retry policy and resource budgets.
//!
//! # Security
//!
//! Registry locations must not contain credentials.
//!
//! Provider registration does not establish provider trust.
//!
//! Registry discovery does not establish remote availability.
//!
//! Protocol compatibility does not establish authorization.
//!
//! Successful download does not establish package integrity.
//!
//! Successful publication does not establish that a package has
//! passed every independent verification or trust requirement.
//!
//! These conditions must be established by their authoritative
//! subsystems before an operation is considered production-ready.

// ============================================================================
// MODULE DECLARATIONS
// ============================================================================

/// Registry provider contracts, descriptors, operation identifiers,
/// request/response envelopes and checked provider execution.
pub mod provider;

/// Registry configuration, provider bindings, lifecycle management,
/// immutable snapshots and the thread-safe registry catalog.
pub mod registry;

/// Registry provider catalogs, discovery requests, compatibility
/// classification and discovery reports.
pub mod discovery;

/// Provider selection, registry communication orchestration and
/// checked asynchronous registry operation dispatch.
pub mod client;

/// Versioned registry wire messages, serialization, deserialization,
/// validation and protocol negotiation.
pub mod protocol;

/// Authentication provider contracts, credential references,
/// authentication requests and authenticated session interfaces.
pub mod authentication;

/// Registry package publication, chunked transfer and publication
/// receipt validation.
pub mod publishing;

/// Registry package downloading, chunked transfer and downloaded
/// package verification.
pub mod downloading;

/// Multi-registry federation, member management, authorization
/// policies and federated operation reports.
pub mod federation;

/// Registry/provider configuration validation, protocol and
/// operation compatibility checks and structured reports.
pub mod validation;

// ============================================================================
// PROVIDER CONTRACTS
// ============================================================================

/// Registry provider descriptor.
pub use provider::RegistryProviderDescriptor;

/// Registry provider implementation contract.
pub use provider::RegistryProvider;

/// Registry operation identifier.
pub use provider::RegistryOperationId;

/// Opaque registry operation request.
pub use provider::RegistryRequest;

/// Opaque registry operation response.
pub use provider::RegistryResponse;

/// Future returned by asynchronous registry providers.
pub use provider::ProviderFuture;

/// Validate and execute a registry request against a provider.
///
/// This is the checked provider execution boundary. Callers should
/// use it instead of bypassing provider request validation.
pub use provider::execute_checked;

// ============================================================================
// REGISTRY CONFIGURATION AND LIFECYCLE
// ============================================================================

/// Registry configuration.
pub use registry::Registry;

/// Human-readable registry name.
pub use registry::RegistryName;

/// Provider-interpreted registry location.
pub use registry::RegistryLocation;

/// Registry lifecycle status.
pub use registry::RegistryStatus;

/// Registry-to-provider configuration binding.
pub use registry::RegistryProviderBinding;

/// Immutable registry configuration snapshot.
pub use registry::RegistrySnapshot;

/// Canonical in-process registry catalog.
pub use registry::RegistryCatalog;

/// Shared registry configuration.
pub use registry::SharedRegistry;

/// Shared registry catalog.
pub use registry::SharedRegistryCatalog;

// ============================================================================
// PROVIDER AND REGISTRY DISCOVERY
// ============================================================================

/// Registry provider catalog.
pub use discovery::RegistryProviderCatalog;

/// Registry discovery requirements.
pub use discovery::RegistryDiscoveryRequirements;

/// Registry discovery request.
pub use discovery::RegistryDiscoveryRequest;

/// Registry provider availability classification.
pub use discovery::ProviderAvailability;

/// One registry discovery result.
pub use discovery::RegistryDiscoveryEntry;

/// Complete registry discovery report.
pub use discovery::RegistryDiscoveryReport;

/// Registry discovery coordinator.
pub use discovery::RegistryDiscovery;

// ============================================================================
// REGISTRY CLIENT
// ============================================================================

/// Registry client.
pub use client::RegistryClient;

/// Shared registry client.
pub use client::SharedRegistryClient;

/// Shared registry provider implementation handle.
///
/// The discovery module also exposes an equivalent alias under its
/// own namespace. This root-level name intentionally comes from
/// the client API to avoid ambiguous wildcard re-exports.
pub use client::SharedRegistryProvider;

// ============================================================================
// REGISTRY PROTOCOL
// ============================================================================

/// Registry protocol codec.
pub use protocol::RegistryProtocolCodec;

/// Protocol negotiation result.
pub use protocol::ProtocolNegotiation;

/// Registry protocol encoding and decoding errors.
pub use protocol::RegistryProtocolError;

/// Registry request message identifier.
pub use protocol::REGISTRY_REQUEST_MESSAGE;

/// Registry response message identifier.
pub use protocol::REGISTRY_RESPONSE_MESSAGE;

/// Registry payload encoding identifier.
pub use protocol::REGISTRY_PAYLOAD_ENCODING;

// ============================================================================
// AUTHENTICATION CONTRACTS
// ============================================================================

/// Authentication scheme identity.
pub use authentication::AuthenticationSchemeId;

/// Credential reference.
///
/// This is a reference to credential material, not the credential
/// itself. Secret material must remain under the canonical security
/// and credential-management authorities.
pub use authentication::CredentialReference;

/// Registry authentication challenge.
pub use authentication::AuthenticationChallenge;

/// Registry authentication request.
pub use authentication::AuthenticationRequest;

/// Registry authentication provider descriptor.
pub use authentication::AuthenticationProviderDescriptor;

/// Registry authentication session metadata.
pub use authentication::AuthenticationSessionMetadata;

/// Registry authentication session contract.
pub use authentication::RegistryAuthenticationSession;

/// Registry authentication provider contract.
pub use authentication::RegistryAuthenticationProvider;

/// Shared authenticated session implementation.
pub use authentication::SharedAuthenticationSession;

/// Validated authenticated registry session.
pub use authentication::AuthenticatedSession;

/// Future returned by authentication providers.
pub use authentication::AuthenticationFuture;

/// Perform checked registry authentication.
pub use authentication::authenticate_checked;

// ============================================================================
// PACKAGE DOWNLOADING
// ============================================================================

/// Registry download identity.
pub use downloading::DownloadId;

/// Registry download options.
pub use downloading::DownloadOptions;

/// Registry download authorization action.
pub use downloading::DownloadAction;

/// Download authorization contract.
pub use downloading::DownloadAuthorizer;

/// Validated download receipt.
pub use downloading::DownloadReceipt;

/// Execute a checked registry package download.
pub use downloading::download;

// ============================================================================
// PACKAGE PUBLISHING
// ============================================================================

/// Registry publication identity.
pub use publishing::PublicationId;

/// Registry publication options.
pub use publishing::PublicationOptions;

/// Registry publication authorization action.
pub use publishing::PublicationAction;

/// Publication authorization contract.
pub use publishing::PublicationAuthorizer;

/// Validated publication receipt.
pub use publishing::PublicationReceipt;

/// Execute a checked registry package publication.
pub use publishing::publish;

// ============================================================================
// REGISTRY FEDERATION
// ============================================================================

/// Federation member.
pub use federation::FederationMember;

/// Registry federation coordinator.
pub use federation::Federation;

/// Federated registry query.
pub use federation::FederationQuery;

/// Federated registry authorization policy.
pub use federation::FederationAccessPolicy;

/// Future returned by a federation authorization policy.
pub use federation::FederationPolicyFuture;

/// Caller-configured federation execution options.
pub use federation::FederationExecutionOptions;

/// Federation completion status.
pub use federation::FederationCompletion;

/// Federation operation failure stage.
pub use federation::FederationFailureStage;

/// One federation member's result.
pub use federation::FederationMemberResult;

/// Federation member operation outcome.
pub use federation::FederationMemberOutcome;

/// Complete federated operation report.
pub use federation::FederationReport;

// ============================================================================
// REGISTRY VALIDATION
// ============================================================================

/// Registry validation requirements.
pub use validation::RegistryValidationRequirements;

/// Registry validation issue classification.
pub use validation::RegistryValidationIssueKind;

/// One registry validation issue.
pub use validation::RegistryValidationIssue;

/// Registry validation report.
pub use validation::RegistryValidationReport;

/// Validate a registry against a provider descriptor.
pub use validation::validate_registry;

/// Validate an immutable registry snapshot.
pub use validation::validate_registry_snapshot;

/// Require a valid registry/provider relationship.
pub use validation::ensure_registry_valid;

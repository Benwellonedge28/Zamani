
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani AI Interoperability
//!
//! Path: `src/ai/interoperability.rs`
//! Rust edition: 2021.
//! Minimum Rust version: 1.97 or later.
//! Safety: Unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Define the type-safe interoperability boundary for Zamani AI
//! computational representations.
//!
//! This module allows explicitly registered adapters to convert validated
//! AI representations into other canonical representations while preserving
//! declared semantic compatibility requirements.
//!
//! It does not implement a second intermediate representation.
//!
//! ## Architectural authority
//!
//! - `grammar/ai/README.md`
//! - `grammar/ai/ai.g4`
//! - `grammar/spec/ai.md`
//! - `grammar/interoperability/README.md`
//! - `grammar/specification/compatibility.md`
//! - `src/ai/error.rs`
//! - `src/ai/version.rs`
//! - `src/zuir/`
//! - `src/zuir/interchange/`
//! - `src/zuir/dialects/`
//! - `src/classical/collections/`
//! - `src/quantum/ir/`
//! - `src/hdl/hdlir/`
//! - `src/hybrid/`
//! - `src/distributed/`
//!
//! ## Owns
//!
//! - AI representation identities.
//! - Versioned representation references.
//! - Adapter identities and descriptors.
//! - Type-safe adapter contracts.
//! - Explicit adapter registration.
//! - Deterministic adapter lookup.
//! - Conversion contexts and policies.
//! - Semantic fidelity declarations.
//! - Conversion reports.
//! - Conversion validation.
//! - Interoperability error translation.
//!
//! ## Does not own
//!
//! - Source grammar or AST construction.
//! - Semantic analysis or type checking.
//! - Universal ZUIR representation.
//! - AI-specific ZUIR operation definitions.
//! - Tensor storage or numerical kernels.
//! - Classical, quantum or HDL intermediate representations.
//! - Hybrid dependency graphs.
//! - Distributed execution plans.
//! - Hardware discovery or resource allocation.
//! - Compiler optimization or lowering.
//! - Backend implementation.
//! - Runtime execution.
//! - External format serialization.
//! - Security enforcement.
//!
//! ## Integration principle
//!
//! AI adapters connect existing representation authorities.
//!
//! They must not create competing representations or silently discard
//! semantics that cannot be expressed by a destination.
//!
//! Every conversion identifies:
//!
//! - its source representation and exact version;
//! - its destination representation and exact version;
//! - its adapter and adapter version;
//! - its declared semantic fidelity;
//! - its conversion result.
//!
//! There is no implicit version substitution or adapter fallback.
//!
//! ## POCO-REAF
//!
//! This module imposes no universal maximum on:
//!
//! - registered adapters;
//! - representations;
//! - versions;
//! - models;
//! - parameters;
//! - tensors;
//! - tensor rank;
//! - dimensions;
//! - operations;
//! - devices;
//! - accelerators;
//! - distributed participants;
//! - computational domains.
//!
//! Actual processing is limited by available resources, address space,
//! adapter implementations, explicit operational policies and the
//! representability of the relevant destination.
//!
//! No finite implementation promises literal infinite computation.
//!
//! ## Trust boundary
//!
//! Adapter registration is an extension activation decision.
//!
//! Callers must perform applicable trust, authorization and compatibility
//! checks before registering or using an adapter.
//!
//! A conversion report is an adapter's declared result, not an independent
//! mathematical proof that the adapter is correct.
//!
//! ## Concurrency
//!
//! Registered adapters are shared through immutable `Arc` references.
//!
//! Registry changes require mutable access to the registry.
//!
//! Adapter implementations must provide their own thread-safety guarantees.
//!
//! No global mutable registry is introduced.

use core::any::{Any, TypeId};
use core::fmt;
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use crate::ai::error::{
    AiError,
    AiErrorKind,
    AiErrorPhase,
    AiResult,
};

use crate::zuir::version::VersionTriple;

// ============================================================================
// ERROR CONSTRUCTION
// ============================================================================

fn interoperability_error(
    kind: AiErrorKind,
    message: impl Into<String>,
) -> AiError {
    AiError::new(kind, message)
        .with_phase(AiErrorPhase::Interoperability)
}

// ============================================================================
// IDENTITY VALIDATION
// ============================================================================

fn validate_identity_component(
    value: &str,
    field: &'static str,
) -> AiResult<()> {
    if value.trim().is_empty() {
        return Err(
            interoperability_error(
                AiErrorKind::InvalidInput,
                format!("{field} must not be empty"),
            )
            .with_detail("field", field),
        );
    }

    if value.chars().any(char::is_control)
        || value.chars().any(char::is_whitespace)
    {
        return Err(
            interoperability_error(
                AiErrorKind::InvalidInput,
                format!("{field} contains an invalid character"),
            )
            .with_detail("field", field),
        );
    }

    Ok(())
}

// ============================================================================
// REPRESENTATION IDENTITY
// ============================================================================

/// Stable, namespaced identity of a computational representation.
///
/// Representation identities are open-world. New computational domains
/// can introduce identities without extending a closed enumeration.
///
/// An identity does not establish that its implementation exists or that
/// a particular conversion is supported.
#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub struct AiRepresentationId {
    namespace: String,
    name: String,
}

impl AiRepresentationId {
    /// Construct a validated representation identity.
    ///
    /// # Errors
    ///
    /// Returns an AI interoperability error when either component is
    /// empty or contains whitespace or control characters.
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> AiResult<Self> {
        let namespace = namespace.into();
        let name = name.into();

        validate_identity_component(
            &namespace,
            "representation namespace",
        )?;

        validate_identity_component(
            &name,
            "representation name",
        )?;

        Ok(Self {
            namespace,
            name,
        })
    }

    /// Return the representation namespace.
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Return the representation name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the qualified representation identity.
    #[must_use]
    pub fn qualified_name(&self) -> String {
        format!("{}::{}", self.namespace, self.name)
    }
}

impl fmt::Display for AiRepresentationId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "{}::{}",
            self.namespace,
            self.name
        )
    }
}

// ============================================================================
// VERSIONED REPRESENTATION REFERENCES
// ============================================================================

/// An exact, versioned reference to a computational representation.
///
/// The version uses the canonical ZUIR `VersionTriple` representation.
/// The owning domain remains authoritative for the meaning and compatibility
/// of its own version numbers.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AiRepresentationRef {
    identity: AiRepresentationId,
    version: VersionTriple,
}

impl AiRepresentationRef {
    /// Construct an exact representation reference.
    #[must_use]
    pub const fn new(
        identity: AiRepresentationId,
        version: VersionTriple,
    ) -> Self {
        Self {
            identity,
            version,
        }
    }

    /// Return the representation identity.
    #[must_use]
    pub fn identity(&self) -> &AiRepresentationId {
        &self.identity
    }

    /// Return the exact representation version.
    #[must_use]
    pub const fn version(&self) -> VersionTriple {
        self.version
    }
}

// ============================================================================
// ADAPTER IDENTITY
// ============================================================================

/// Stable identity of an interoperability adapter.
///
/// Adapter identities are separate from representation identities.
/// An adapter may support one or more explicitly registered conversion
/// contracts, but each registered descriptor has one exact source and
/// destination pair.
#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub struct AiAdapterId {
    namespace: String,
    name: String,
}

impl AiAdapterId {
    /// Construct a validated adapter identity.
    ///
    /// # Errors
    ///
    /// Returns an error when either identity component is invalid.
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> AiResult<Self> {
        let namespace = namespace.into();
        let name = name.into();

        validate_identity_component(
            &namespace,
            "adapter namespace",
        )?;

        validate_identity_component(
            &name,
            "adapter name",
        )?;

        Ok(Self {
            namespace,
            name,
        })
    }

    /// Return the adapter namespace.
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Return the adapter name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the qualified adapter identity.
    #[must_use]
    pub fn qualified_name(&self) -> String {
        format!("{}::{}", self.namespace, self.name)
    }
}

impl fmt::Display for AiAdapterId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "{}::{}",
            self.namespace,
            self.name
        )
    }
}

// ============================================================================
// SEMANTIC FIDELITY
// ============================================================================

/// Classification of a conversion's semantic fidelity.
///
/// The classification is a declared adapter contract. It is not proof
/// that an adapter has correctly implemented the declared behavior.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum AiFidelityClass {
    /// The conversion preserves the specified semantics exactly.
    Exact,

    /// The conversion preserves semantics under an explicit named contract.
    Contractual,

    /// The conversion uses an explicitly accepted approximation contract.
    Approximate,
}

/// Semantic fidelity declared by an individual conversion.
///
/// Contract identifiers are open-world names owned by their respective
/// semantic or numerical authorities.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AiConversionFidelity {
    /// Exact semantic preservation.
    Exact,

    /// Preservation subject to a named semantic contract.
    Contractual {
        /// Identity of the preservation contract.
        contract: String,
    },

    /// Approximate preservation under an explicit approximation contract.
    Approximate {
        /// Identity of the approximation contract.
        contract: String,
    },
}

impl AiConversionFidelity {
    /// Return the classification of this fidelity declaration.
    #[must_use]
    pub const fn class(&self) -> AiFidelityClass {
        match self {
            Self::Exact => AiFidelityClass::Exact,
            Self::Contractual { .. } => AiFidelityClass::Contractual,
            Self::Approximate { .. } => AiFidelityClass::Approximate,
        }
    }

    fn validate(&self) -> AiResult<()> {
        match self {
            Self::Exact => Ok(()),

            Self::Contractual { contract }
            | Self::Approximate { contract } => {
                validate_identity_component(
                    contract,
                    "semantic fidelity contract",
                )
            }
        }
    }

    fn contract(&self) -> Option<&str> {
        match self {
            Self::Exact => None,

            Self::Contractual { contract }
            | Self::Approximate { contract } => {
                Some(contract)
            }
        }
    }
}

// ============================================================================
// CONVERSION POLICY
// ============================================================================

/// Policy controlling which semantic conversion results are acceptable.
///
/// Exact conversion is always accepted.
///
/// Contractual and approximate conversions require explicit named
/// contracts. An adapter cannot authorize its own semantic relaxation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AiConversionPolicy {
    /// Accept only exact semantic preservation.
    ExactOnly,

    /// Accept exact conversions and contractual conversions explicitly
    /// named in the accepted contract set.
    Contractual {
        /// Accepted semantic contract identities.
        accepted_contracts: BTreeSet<String>,
    },

    /// Accept exact, explicitly contracted, and explicitly approximated
    /// conversions.
    Approximate {
        /// Accepted semantic and approximation contract identities.
        accepted_contracts: BTreeSet<String>,
    },
}

impl AiConversionPolicy {
    /// Construct an exact-only policy.
    #[must_use]
    pub const fn exact_only() -> Self {
        Self::ExactOnly
    }

    /// Construct a policy accepting named semantic contracts.
    ///
    /// # Errors
    ///
    /// Returns an error if an accepted contract identity is invalid.
    pub fn contractual<I, S>(
        contracts: I,
    ) -> AiResult<Self>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let accepted_contracts =
            validate_contracts(contracts)?;

        Ok(Self::Contractual {
            accepted_contracts,
        })
    }

    /// Construct a policy permitting explicitly named approximations.
    ///
    /// Approximation is never enabled implicitly.
    ///
    /// # Errors
    ///
    /// Returns an error if an accepted contract identity is invalid.
    pub fn approximate<I, S>(
        contracts: I,
    ) -> AiResult<Self>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let accepted_contracts =
            validate_contracts(contracts)?;

        Ok(Self::Approximate {
            accepted_contracts,
        })
    }

    /// Determine whether a fidelity declaration is accepted.
    #[must_use]
    pub fn permits(
        &self,
        fidelity: &AiConversionFidelity,
    ) -> bool {
        match fidelity {
            AiConversionFidelity::Exact => true,

            AiConversionFidelity::Contractual {
                contract,
            } => match self {
                Self::ExactOnly => false,

                Self::Contractual {
                    accepted_contracts,
                }
                | Self::Approximate {
                    accepted_contracts,
                } => accepted_contracts.contains(contract),
            },

            AiConversionFidelity::Approximate {
                contract,
            } => match self {
                Self::Approximate {
                    accepted_contracts,
                } => accepted_contracts.contains(contract),

                Self::ExactOnly | Self::Contractual { .. } => false,
            },
        }
    }
}

fn validate_contracts<I, S>(
    contracts: I,
) -> AiResult<BTreeSet<String>>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let mut accepted = BTreeSet::new();

    for contract in contracts {
        let contract = contract.into();

        validate_identity_component(
            &contract,
            "accepted semantic contract",
        )?;

        accepted.insert(contract);
    }

    Ok(accepted)
}

// ============================================================================
// CONVERSION CONTEXT
// ============================================================================

/// Explicit policy context for one interoperability conversion.
///
/// The context contains semantic acceptance policy only. It does not
/// discover hardware, allocate resources, select devices or execute
/// computational workloads.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AiConversionContext {
    policy: AiConversionPolicy,
}

impl AiConversionContext {
    /// Construct a conversion context using an explicit policy.
    #[must_use]
    pub const fn new(
        policy: AiConversionPolicy,
    ) -> Self {
        Self {
            policy,
        }
    }

    /// Construct a context that accepts exact conversions only.
    #[must_use]
    pub const fn exact_only() -> Self {
        Self::new(AiConversionPolicy::ExactOnly)
    }

    /// Return the conversion policy.
    #[must_use]
    pub fn policy(&self) -> &AiConversionPolicy {
        &self.policy
    }
}

impl Default for AiConversionContext {
    fn default() -> Self {
        Self::exact_only()
    }
}

// ============================================================================
// ADAPTER DESCRIPTOR
// ============================================================================

/// Immutable declaration of an interoperability adapter.
///
/// A descriptor defines one exact, versioned conversion boundary.
/// It does not prove that the adapter's implementation is correct.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AiAdapterDescriptor {
    identity: AiAdapterId,
    source: AiRepresentationRef,
    destination: AiRepresentationRef,
    version: VersionTriple,
    supported_fidelity: BTreeSet<AiFidelityClass>,
}

impl AiAdapterDescriptor {
    /// Construct an adapter descriptor.
    ///
    /// At least one fidelity class must be declared.
    ///
    /// # Errors
    ///
    /// Returns an error if the descriptor declares no supported fidelity.
    pub fn new<I>(
        identity: AiAdapterId,
        source: AiRepresentationRef,
        destination: AiRepresentationRef,
        version: VersionTriple,
        supported_fidelity: I,
    ) -> AiResult<Self>
    where
        I: IntoIterator<Item = AiFidelityClass>,
    {
        let supported_fidelity =
            supported_fidelity.into_iter().collect::<BTreeSet<_>>();

        if supported_fidelity.is_empty() {
            return Err(
                interoperability_error(
                    AiErrorKind::InvalidInput,
                    "an AI adapter must declare at least one fidelity class",
                )
                .with_detail(
                    "adapter",
                    identity.qualified_name(),
                ),
            );
        }

        Ok(Self {
            identity,
            source,
            destination,
            version,
            supported_fidelity,
        })
    }

    /// Return the adapter identity.
    #[must_use]
    pub fn identity(&self) -> &AiAdapterId {
        &self.identity
    }

    /// Return the exact source representation.
    #[must_use]
    pub fn source(&self) -> &AiRepresentationRef {
        &self.source
    }

    /// Return the exact destination representation.
    #[must_use]
    pub fn destination(&self) -> &AiRepresentationRef {
        &self.destination
    }

    /// Return the adapter contract version.
    #[must_use]
    pub const fn version(&self) -> VersionTriple {
        self.version
    }

    /// Return the declared fidelity classes.
    #[must_use]
    pub fn supported_fidelity(
        &self,
    ) -> &BTreeSet<AiFidelityClass> {
        &self.supported_fidelity
    }

    fn supports(
        &self,
        fidelity: AiFidelityClass,
    ) -> bool {
        self.supported_fidelity.contains(&fidelity)
    }
}

// ============================================================================
// CONVERSION NOTICES
// ============================================================================

/// A structured notice produced by an interoperability adapter.
///
/// Notices describe relevant conversion decisions, warnings or preserved
/// properties. They do not replace canonical compiler diagnostics.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AiConversionNotice {
    code: String,
    message: String,
}

impl AiConversionNotice {
    /// Construct a validated conversion notice.
    ///
    /// # Errors
    ///
    /// Returns an error if the code or message is empty.
    pub fn new(
        code: impl Into<String>,
        message: impl Into<String>,
    ) -> AiResult<Self> {
        let code = code.into();
        let message = message.into();

        validate_identity_component(
            &code,
            "conversion notice code",
        )?;

        if message.trim().is_empty()
            || message.chars().any(char::is_control)
        {
            return Err(
                interoperability_error(
                    AiErrorKind::InvalidInput,
                    "conversion notice message is invalid",
                ),
            );
        }

        Ok(Self {
            code,
            message,
        })
    }

    /// Return the stable notice code.
    #[must_use]
    pub fn code(&self) -> &str {
        &self.code
    }

    /// Return the explanatory notice.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

// ============================================================================
// CONVERSION REPORT
// ============================================================================

/// Structured result of an interoperability conversion.
///
/// The report records the exact adapter and representation contracts
/// involved in the conversion.
///
/// It is an adapter-provided declaration. Consumers remain responsible
/// for trusting, auditing and testing the adapter implementation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AiConversionReport {
    adapter: AiAdapterId,
    source: AiRepresentationRef,
    destination: AiRepresentationRef,
    fidelity: AiConversionFidelity,
    notices: Vec<AiConversionNotice>,
}

impl AiConversionReport {
    /// Construct a conversion report.
    ///
    /// # Errors
    ///
    /// Returns an error if the fidelity declaration is malformed.
    pub fn new(
        adapter: AiAdapterId,
        source: AiRepresentationRef,
        destination: AiRepresentationRef,
        fidelity: AiConversionFidelity,
    ) -> AiResult<Self> {
        fidelity.validate()?;

        Ok(Self {
            adapter,
            source,
            destination,
            fidelity,
            notices: Vec::new(),
        })
    }

    /// Add a structured conversion notice.
    #[must_use]
    pub fn with_notice(
        mut self,
        notice: AiConversionNotice,
    ) -> Self {
        self.notices.push(notice);
        self
    }

    /// Return the adapter used.
    #[must_use]
    pub fn adapter(&self) -> &AiAdapterId {
        &self.adapter
    }

    /// Return the source representation.
    #[must_use]
    pub fn source(&self) -> &AiRepresentationRef {
        &self.source
    }

    /// Return the destination representation.
    #[must_use]
    pub fn destination(&self) -> &AiRepresentationRef {
        &self.destination
    }

    /// Return the declared semantic fidelity.
    #[must_use]
    pub fn fidelity(&self) -> &AiConversionFidelity {
        &self.fidelity
    }

    /// Return conversion notices in adapter-reported order.
    #[must_use]
    pub fn notices(&self) -> &[AiConversionNotice] {
        &self.notices
    }

    fn validate_for(
        &self,
        descriptor: &AiAdapterDescriptor,
        context: &AiConversionContext,
    ) -> AiResult<()> {
        if self.adapter != descriptor.identity {
            return Err(
                interoperability_error(
                    AiErrorKind::InternalFailure,
                    "adapter reported a different adapter identity",
                )
                .with_detail(
                    "registered_adapter",
                    descriptor.identity.qualified_name(),
                )
                .with_detail(
                    "reported_adapter",
                    self.adapter.qualified_name(),
                ),
            );
        }

        if self.source != descriptor.source
            || self.destination != descriptor.destination
        {
            return Err(
                interoperability_error(
                    AiErrorKind::InternalFailure,
                    "adapter reported representation references that do not match its descriptor",
                )
                .with_detail(
                    "adapter",
                    descriptor.identity.qualified_name(),
                ),
            );
        }

        self.fidelity.validate()?;

        let fidelity_class = self.fidelity.class();

        if !descriptor.supports(fidelity_class) {
            return Err(
                interoperability_error(
                    AiErrorKind::InternalFailure,
                    "adapter reported a fidelity class absent from its descriptor",
                )
                .with_detail(
                    "adapter",
                    descriptor.identity.qualified_name(),
                ),
            );
        }

        if !context.policy.permits(&self.fidelity) {
            return Err(
                interoperability_error(
                    AiErrorKind::TargetIncompatible,
                    "conversion fidelity is not permitted by the requested policy",
                )
                .with_detail(
                    "adapter",
                    descriptor.identity.qualified_name(),
                ),
            );
        }

        Ok(())
    }
}

// ============================================================================
// CONVERTED REPRESENTATION
// ============================================================================

/// A converted representation and its validated conversion report.
#[derive(Debug)]
pub struct AiConverted<T> {
    value: T,
    report: AiConversionReport,
}

impl<T> AiConverted<T> {
    /// Construct a converted representation.
    ///
    /// This constructor is public for adapter implementations.
    /// The registry validates the report before returning a conversion
    /// to its caller.
    #[must_use]
    pub const fn new(
        value: T,
        report: AiConversionReport,
    ) -> Self {
        Self {
            value,
            report,
        }
    }

    /// Borrow the converted representation.
    #[must_use]
    pub fn value(&self) -> &T {
        &self.value
    }

    /// Consume the result and return its representation.
    #[must_use]
    pub fn into_value(self) -> T {
        self.value
    }

    /// Borrow the conversion report.
    #[must_use]
    pub fn report(&self) -> &AiConversionReport {
        &self.report
    }

    /// Consume the result and return its representation and report.
    #[must_use]
    pub fn into_parts(self) -> (T, AiConversionReport) {
        (self.value, self.report)
    }
}

// ============================================================================
// TYPE-SAFE ADAPTER CONTRACT
// ============================================================================

/// Type-safe contract implemented by one representation adapter.
///
/// The source and destination are concrete Rust types owned by their
/// respective canonical subsystems.
///
/// This trait does not require those types to be defined in the AI module.
///
/// Adapter implementations must:
///
/// - preserve the source's required semantics;
/// - reject unsupported conversions;
/// - validate all representation-specific invariants;
/// - report the actual fidelity of the conversion;
/// - preserve errors instead of inventing successful results;
/// - avoid silently dropping effects, requirements or provenance.
///
/// The registry performs exact runtime type checks around this typed
/// interface.
pub trait AiRepresentationAdapter: Send + Sync + 'static {
    /// Canonical source representation type.
    type Source: Any + Send + Sync + 'static;

    /// Canonical destination representation type.
    type Destination: Any + Send + Sync + 'static;

    /// Return the immutable adapter descriptor.
    fn descriptor(&self) -> &AiAdapterDescriptor;

    /// Convert the source representation.
    ///
    /// The adapter must return a report matching its descriptor.
    ///
    /// # Errors
    ///
    /// Returns an AI error if the conversion cannot be performed or
    /// the source representation is invalid.
    fn convert(
        &self,
        source: &Self::Source,
        context: &AiConversionContext,
    ) -> AiResult<(
        Self::Destination,
        AiConversionReport,
    )>;
}

// ============================================================================
// TYPE-ERASED ADAPTER BOUNDARY
// ============================================================================

trait ErasedAdapter: Send + Sync {
    fn descriptor(&self) -> &AiAdapterDescriptor;

    fn source_type_id(&self) -> TypeId;

    fn destination_type_id(&self) -> TypeId;

    fn convert_erased(
        &self,
        source: &(dyn Any + Send + Sync),
        context: &AiConversionContext,
    ) -> AiResult<(
        Box<dyn Any + Send + Sync>,
        AiConversionReport,
    )>;
}

struct AdapterHolder<A> {
    adapter: A,
}

impl<A> ErasedAdapter for AdapterHolder<A>
where
    A: AiRepresentationAdapter,
{
    fn descriptor(&self) -> &AiAdapterDescriptor {
        self.adapter.descriptor()
    }

    fn source_type_id(&self) -> TypeId {
        TypeId::of::<A::Source>()
    }

    fn destination_type_id(&self) -> TypeId {
        TypeId::of::<A::Destination>()
    }

    fn convert_erased(
        &self,
        source: &(dyn Any + Send + Sync),
        context: &AiConversionContext,
    ) -> AiResult<(
        Box<dyn Any + Send + Sync>,
        AiConversionReport,
    )> {
        let source = source.downcast_ref::<A::Source>()
            .ok_or_else(|| {
                interoperability_error(
                    AiErrorKind::InvalidInput,
                    "source representation has the wrong Rust type",
                )
            })?;

        let (destination, report) =
            self.adapter.convert(source, context)?;

        Ok((
            Box::new(destination),
            report,
        ))
    }
}

// ============================================================================
// INTEROPERABILITY REGISTRY
// ============================================================================

/// Deterministic registry of explicitly registered AI representation
/// adapters.
///
/// The registry owns adapter activation and exact adapter lookup.
///
/// It does not own representation definitions, external format registries,
/// ZUIR dialect registration, hardware capabilities or execution providers.
///
/// Registration order does not determine adapter selection. A conversion
/// must explicitly name its adapter identity and exact representation
/// versions.
#[derive(Clone, Default)]
pub struct AiInteroperabilityRegistry {
    adapters: BTreeMap<
        AiAdapterId,
        Arc<dyn ErasedAdapter>,
    >,
}

impl AiInteroperabilityRegistry {
    /// Construct an empty interoperability registry.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Return the number of registered adapters.
    #[must_use]
    pub fn len(&self) -> usize {
        self.adapters.len()
    }

    /// Return whether the registry contains no adapters.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.adapters.is_empty()
    }

    /// Register one typed adapter.
    ///
    /// The adapter is stored behind an immutable shared reference.
    /// Exact duplicate adapter identities are rejected.
    ///
    /// # Errors
    ///
    /// Returns a conflict error if the identity is already registered.
    pub fn register<A>(
        &mut self,
        adapter: A,
    ) -> AiResult<()>
    where
        A: AiRepresentationAdapter,
    {
        let descriptor = adapter.descriptor();

        let identity = descriptor.identity.clone();

        if self.adapters.contains_key(&identity) {
            return Err(
                interoperability_error(
                    AiErrorKind::Conflict,
                    "an adapter with this identity is already registered",
                )
                .with_detail(
                    "adapter",
                    identity.qualified_name(),
                ),
            );
        }

        let holder = AdapterHolder {
            adapter,
        };

        self.adapters.insert(
            identity,
            Arc::new(holder),
        );

        Ok(())
    }

    /// Look up an adapter descriptor by exact identity.
    #[must_use]
    pub fn descriptor(
        &self,
        identity: &AiAdapterId,
    ) -> Option<&AiAdapterDescriptor> {
        self.adapters
            .get(identity)
            .map(|adapter| adapter.descriptor())
    }

    /// Iterate over adapter descriptors in deterministic identity order.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = &AiAdapterDescriptor> {
        self.adapters
            .values()
            .map(|adapter| adapter.descriptor())
    }

    /// Remove an adapter by exact identity.
    ///
    /// Other adapters and previously retained registry snapshots remain
    /// unaffected.
    ///
    /// # Returns
    ///
    /// `true` if the adapter was registered and removed.
    pub fn remove(
        &mut self,
        identity: &AiAdapterId,
    ) -> bool {
        self.adapters.remove(identity).is_some()
    }

    /// Convert one source representation using an explicitly selected
    /// adapter and exact source and destination representation references.
    ///
    /// No adapter fallback, version substitution or implicit semantic
    /// relaxation is performed.
    ///
    /// The destination type is checked at runtime against the registered
    /// adapter's declared destination type.
    ///
    /// # Errors
    ///
    /// Returns an AI error when:
    ///
    /// - the adapter is not registered;
    /// - source or destination versions do not match;
    /// - the source Rust type does not match;
    /// - the adapter rejects the conversion;
    /// - the adapter returns an inconsistent report;
    /// - the destination Rust type does not match;
    /// - the conversion violates the requested fidelity policy.
    pub fn convert<S, D>(
        &self,
        adapter_identity: &AiAdapterId,
        source_representation: &AiRepresentationRef,
        destination_representation: &AiRepresentationRef,
        source: &S,
        context: &AiConversionContext,
    ) -> AiResult<AiConverted<D>>
    where
        S: Any + Send + Sync + 'static,
        D: Any + Send + Sync + 'static,
    {
        let adapter = self.adapters
            .get(adapter_identity)
            .ok_or_else(|| {
                interoperability_error(
                    AiErrorKind::NotFound,
                    "requested interoperability adapter is not registered",
                )
                .with_detail(
                    "adapter",
                    adapter_identity.qualified_name(),
                )
            })?;

        let descriptor = adapter.descriptor();

        if descriptor.source != *source_representation
            || descriptor.destination != *destination_representation
        {
            return Err(
                interoperability_error(
                    AiErrorKind::VersionIncompatible,
                    "requested representation references do not match the adapter's exact endpoints",
                )
                .with_detail(
                    "adapter",
                    adapter_identity.qualified_name(),
                )
                .with_detail(
                    "expected_source",
                    descriptor.source.identity.qualified_name(),
                )
                .with_detail(
                    "requested_source",
                    source_representation.identity.qualified_name(),
                )
                .with_detail(
                    "expected_destination",
                    descriptor.destination.identity.qualified_name(),
                )
                .with_detail(
                    "requested_destination",
                    destination_representation.identity.qualified_name(),
                ),
            );
        }

        if adapter.source_type_id() != TypeId::of::<S>() {
            return Err(
                interoperability_error(
                    AiErrorKind::DataTypeIncompatible,
                    "source Rust type does not match the registered adapter",
                )
                .with_detail(
                    "adapter",
                    adapter_identity.qualified_name(),
                ),
            );
        }

        if adapter.destination_type_id() != TypeId::of::<D>() {
            return Err(
                interoperability_error(
                    AiErrorKind::DataTypeIncompatible,
                    "requested destination Rust type does not match the registered adapter",
                )
                .with_detail(
                    "adapter",
                    adapter_identity.qualified_name(),
                ),
            );
        }

        let (destination, report) =
            adapter.convert_erased(source, context)?;

        report.validate_for(descriptor, context)?;

        let destination = destination
            .downcast::<D>()
            .map_err(|_| {
                interoperability_error(
                    AiErrorKind::InternalFailure,
                    "adapter returned a destination with an inconsistent Rust type",
                )
                .with_detail(
                    "adapter",
                    adapter_identity.qualified_name(),
                )
            })?;

        Ok(AiConverted::new(
            *destination,
            report,
        ))
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn representation(
        namespace: &str,
        name: &str,
        version: (u32, u32, u32),
    ) -> AiRepresentationRef {
        let identity = AiRepresentationId::new(
            namespace,
            name,
        )
        .expect("valid representation identity");

        AiRepresentationRef::new(
            identity,
            VersionTriple::new(
                version.0,
                version.1,
                version.2,
            ),
        )
    }

    fn adapter_identity() -> AiAdapterId {
        AiAdapterId::new(
            "zamani.test",
            "string_to_bytes",
        )
        .expect("valid adapter identity")
    }

    struct StringToBytesAdapter {
        descriptor: AiAdapterDescriptor,
    }

    impl StringToBytesAdapter {
        fn new() -> Self {
            let descriptor = AiAdapterDescriptor::new(
                adapter_identity(),
                representation(
                    "zamani.test",
                    "string",
                    (1, 0, 0),
                ),
                representation(
                    "zamani.test",
                    "bytes",
                    (1, 0, 0),
                ),
                VersionTriple::new(1, 0, 0),
                [AiFidelityClass::Exact],
            )
            .expect("valid adapter descriptor");

            Self {
                descriptor,
            }
        }
    }

    impl AiRepresentationAdapter for StringToBytesAdapter {
        type Source = String;
        type Destination = Vec<u8>;

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
            let report = AiConversionReport::new(
                self.descriptor.identity.clone(),
                self.descriptor.source.clone(),
                self.descriptor.destination.clone(),
                AiConversionFidelity::Exact,
            )?;

            Ok((
                source.as_bytes().to_vec(),
                report,
            ))
        }
    }

    fn registry() -> AiInteroperabilityRegistry {
        let mut registry = AiInteroperabilityRegistry::new();

        registry
            .register(StringToBytesAdapter::new())
            .expect("adapter registration succeeds");

        registry
    }

    // ------------------------------------------------------------------------
    // Identity tests
    // ------------------------------------------------------------------------

    #[test]
    fn representation_identity_is_preserved() {
        let identity = AiRepresentationId::new(
            "zamani.zuir",
            "ai",
        )
        .expect("valid identity");

        assert_eq!(
            identity.qualified_name(),
            "zamani.zuir::ai"
        );
    }

    #[test]
    fn adapter_identity_is_preserved() {
        let identity = adapter_identity();

        assert_eq!(
            identity.qualified_name(),
            "zamani.test::string_to_bytes"
        );
    }

    #[test]
    fn empty_representation_namespace_is_rejected() {
        assert!(
            AiRepresentationId::new(
                "",
                "ai",
            )
            .is_err()
        );
    }

    #[test]
    fn whitespace_in_adapter_identity_is_rejected() {
        assert!(
            AiAdapterId::new(
                "zamani.test",
                "invalid adapter",
            )
            .is_err()
        );
    }

    // ------------------------------------------------------------------------
    // Fidelity policy tests
    // ------------------------------------------------------------------------

    #[test]
    fn exact_policy_accepts_exact_conversion() {
        assert!(
            AiConversionPolicy::exact_only()
                .permits(&AiConversionFidelity::Exact)
        );
    }

    #[test]
    fn exact_policy_rejects_contractual_conversion() {
        let fidelity = AiConversionFidelity::Contractual {
            contract: "numerical.contract.v1".to_owned(),
        };

        assert!(
            !AiConversionPolicy::exact_only()
                .permits(&fidelity)
        );
    }

    #[test]
    fn contractual_policy_requires_explicit_contract() {
        let policy = AiConversionPolicy::contractual([
            "numerical.contract.v1",
        ])
        .expect("valid policy");

        let accepted = AiConversionFidelity::Contractual {
            contract: "numerical.contract.v1".to_owned(),
        };

        let rejected = AiConversionFidelity::Contractual {
            contract: "unknown.contract".to_owned(),
        };

        assert!(policy.permits(&accepted));
        assert!(!policy.permits(&rejected));
    }

    #[test]
    fn approximation_requires_explicit_permission() {
        let fidelity = AiConversionFidelity::Approximate {
            contract: "approximation.v1".to_owned(),
        };

        let contractual = AiConversionPolicy::contractual([
            "approximation.v1",
        ])
        .expect("valid policy");

        let approximate = AiConversionPolicy::approximate([
            "approximation.v1",
        ])
        .expect("valid policy");

        assert!(!contractual.permits(&fidelity));
        assert!(approximate.permits(&fidelity));
    }

    // ------------------------------------------------------------------------
    // Descriptor and registry tests
    // ------------------------------------------------------------------------

    #[test]
    fn adapter_descriptor_rejects_empty_fidelity() {
        let result = AiAdapterDescriptor::new(
            adapter_identity(),
            representation("test", "source", (1, 0, 0)),
            representation("test", "destination", (1, 0, 0)),
            VersionTriple::new(1, 0, 0),
            [],
        );

        assert!(result.is_err());
    }

    #[test]
    fn registry_starts_empty() {
        let registry = AiInteroperabilityRegistry::new();

        assert!(registry.is_empty());
        assert_eq!(registry.len(), 0);
    }

    #[test]
    fn registers_and_resolves_adapter() {
        let registry = registry();

        assert_eq!(registry.len(), 1);

        assert!(
            registry.descriptor(&adapter_identity()).is_some()
        );
    }

    #[test]
    fn duplicate_adapter_identity_is_rejected() {
        let mut registry = registry();

        let result = registry.register(
            StringToBytesAdapter::new(),
        );

        assert!(result.is_err());
        assert_eq!(registry.len(), 1);
    }

    #[test]
    fn registry_iteration_is_deterministic() {
        let registry = registry();

        let identities: Vec<_> = registry
            .iter()
            .map(|descriptor| descriptor.identity().qualified_name())
            .collect();

        assert_eq!(
            identities,
            vec!["zamani.test::string_to_bytes"]
        );
    }

    // ------------------------------------------------------------------------
    // Conversion tests
    // ------------------------------------------------------------------------

    #[test]
    fn performs_exact_typed_conversion() {
        let registry = registry();

        let source = representation(
            "zamani.test",
            "string",
            (1, 0, 0),
        );

        let destination = representation(
            "zamani.test",
            "bytes",
            (1, 0, 0),
        );

        let result = registry
            .convert::<String, Vec<u8>>(
                &adapter_identity(),
                &source,
                &destination,
                &"Zamani".to_owned(),
                &AiConversionContext::exact_only(),
            )
            .expect("conversion succeeds");

        assert_eq!(result.value(), b"Zamani");
        assert_eq!(
            result.report().fidelity(),
            &AiConversionFidelity::Exact
        );
    }

    #[test]
    fn rejects_unregistered_adapter() {
        let registry = AiInteroperabilityRegistry::new();

        let result = registry.convert::<String, Vec<u8>>(
            &adapter_identity(),
            &representation("test", "source", (1, 0, 0)),
            &representation("test", "destination", (1, 0, 0)),
            &"value".to_owned(),
            &AiConversionContext::exact_only(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_source_version_mismatch() {
        let registry = registry();

        let result = registry.convert::<String, Vec<u8>>(
            &adapter_identity(),
            &representation(
                "zamani.test",
                "string",
                (2, 0, 0),
            ),
            &representation(
                "zamani.test",
                "bytes",
                (1, 0, 0),
            ),
            &"value".to_owned(),
            &AiConversionContext::exact_only(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_wrong_source_rust_type() {
        let registry = registry();

        let result = registry.convert::<u64, Vec<u8>>(
            &adapter_identity(),
            &representation(
                "zamani.test",
                "string",
                (1, 0, 0),
            ),
            &representation(
                "zamani.test",
                "bytes",
                (1, 0, 0),
            ),
            &42_u64,
            &AiConversionContext::exact_only(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_wrong_destination_rust_type() {
        let registry = registry();

        let result = registry.convert::<String, String>(
            &adapter_identity(),
            &representation(
                "zamani.test",
                "string",
                (1, 0, 0),
            ),
            &representation(
                "zamani.test",
                "bytes",
                (1, 0, 0),
            ),
            &"value".to_owned(),
            &AiConversionContext::exact_only(),
        );

        assert!(result.is_err());
    }

    // ------------------------------------------------------------------------
    // Registry lifecycle tests
    // ------------------------------------------------------------------------

    #[test]
    fn removal_only_affects_the_mutated_registry() {
        let mut original = registry();
        let snapshot = original.clone();

        assert!(original.remove(&adapter_identity()));
        assert!(original.is_empty());

        assert_eq!(snapshot.len(), 1);
    }

    #[test]
    fn removing_unknown_adapter_returns_false() {
        let mut registry = AiInteroperabilityRegistry::new();

        assert!(!registry.remove(&adapter_identity()));
    }
}

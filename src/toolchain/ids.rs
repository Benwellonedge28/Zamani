
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Toolchain — Strongly Typed Identifiers
//!
//! Path: `src/toolchain/ids.rs`
//! Edition: Rust 2021.
//! Minimum Rust version: 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Defines validated, strongly typed identities owned by the Zamani
//! toolchain. These identities are used by project, workspace, package,
//! dependency, task, build, registry, provider, environment, profile,
//! and deployment services.
//!
//! ## Architectural ownership
//!
//! This module owns:
//!
//! - Toolchain-specific identity categories.
//! - Namespace validation.
//! - Canonical textual representations.
//! - Parsing and serialization.
//! - Per-instance, thread-safe monotonic allocation.
//! - Explicit invalid-identity and allocation errors.
//!
//! This module does not own:
//!
//! - Source-language names or AST identities.
//! - Semantic symbols or semantic program identities.
//! - ZUIR, classical IR, quantum IR, or HDL IR identities.
//! - Backend, target, external-toolchain, or artifact identities.
//! - Physical device, qubit, register, or runtime identities.
//! - Content hashes or reproducibility fingerprints.
//! - Persistent database allocation or distributed coordination.
//!
//! ## Canonical identity representation
//!
//!     namespace:nonzero-decimal-u128
//!
//! Namespace grammar:
//!
//! - First character: lowercase ASCII letter.
//! - Remaining characters: lowercase ASCII letters, digits, '.', '_', '-'
//! - Namespace comparison is case-sensitive.
//!
//! Numeric grammar:
//!
//! - Decimal digits only.
//! - Nonzero.
//! - No sign.
//! - No leading zeroes.
//! - Must fit in u128.
//!
//! Parsing never silently normalizes an identifier.
//!
//! ## Uniqueness contract
//!
//! Identity uniqueness is scoped by both category and namespace-qualified
//! value.
//!
//! An allocator guarantees uniqueness within that allocator instance.
//! Separate allocator instances are independent and can produce identical
//! values when configured with the same namespace.
//!
//! Cross-process or distributed uniqueness requires a persistent allocation
//! authority, coordinated namespace assignment, or caller-supplied identity.
//! This module does not falsely promise global uniqueness.
//!
//! ## Scalability
//!
//! This module defines no maximum number of projects, packages, workspaces,
//! registries, providers, tasks, builds, or deployments.
//!
//! It defines no hardware capacity or computational-domain ceiling.
//!
//! Individual IDs use nonzero u128 values. Exhaustion is reported explicitly.
//! Values are never wrapped or reused.
//!
//! Memory, address-space and storage constraints remain finite and are
//! properties of the executing environment.
//!
//! ## Integration
//!
//! Toolchain consumers:
//!
//! - workspace::members uses ProjectId and WorkspaceId.
//! - dependencies uses PackageId and SourceId.
//! - packages uses PackageId.
//! - tasks uses TaskId.
//! - build uses BuildId.
//! - registries uses RegistryId.
//! - provider registries use ProviderId.
//! - environments uses EnvironmentId.
//! - configuration uses ProfileId.
//! - deployment uses DeploymentId.
//!
//! Backend identities remain owned by crate::backend::ids.
//! AST identities remain owned by crate::frontend::ast.
//! Domain identities remain owned by their respective IRs.
//!
//! ## Dependencies
//!
//! Uses the existing serde dependency.
//! No additional Cargo dependency is required.

use serde::{
    de,
    Deserialize,
    Deserializer,
    Serialize,
    Serializer,
};

use std::{
    fmt,
    hash::Hash,
    marker::PhantomData,
    num::NonZeroU128,
    str::FromStr,
    sync::Mutex,
};

// ============================================================================
// NAMESPACE VALIDATION
// ============================================================================

/// Errors encountered while validating an identifier namespace.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum InvalidNamespace {
    /// The namespace is empty.
    Empty,

    /// The namespace does not follow the canonical ASCII grammar.
    InvalidSyntax,
}

impl fmt::Display for InvalidNamespace {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => {
                formatter.write_str(
                    "identifier namespace must not be empty",
                )
            }

            Self::InvalidSyntax => {
                formatter.write_str(
                    "namespace must begin with a lowercase ASCII letter \
                     and contain only lowercase ASCII letters, digits, \
                     '.', '_' or '-'",
                )
            }
        }
    }
}

impl std::error::Error for InvalidNamespace {}

/// A validated namespace for toolchain-owned identities.
///
/// Namespaces are case-sensitive and are never silently normalized.
///
/// Namespace length is not restricted by an arbitrary application-level
/// constant. Allocation remains subject to available memory.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IdNamespace(String);

impl IdNamespace {
    /// Construct and validate a namespace.
    pub fn new(value: impl Into<String>) -> Result<Self, InvalidNamespace> {
        let value = value.into();

        validate_namespace(&value)?;

        Ok(Self(value))
    }

    /// Borrow the canonical namespace text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

fn validate_namespace(value: &str) -> Result<(), InvalidNamespace> {
    let mut bytes = value.bytes();

    match bytes.next() {
        None => return Err(InvalidNamespace::Empty),

        Some(first) if first.is_ascii_lowercase() => {}

        Some(_) => return Err(InvalidNamespace::InvalidSyntax),
    }

    if bytes.all(|byte| {
        byte.is_ascii_lowercase()
            || byte.is_ascii_digit()
            || matches!(byte, b'.' | b'_' | b'-')
    }) {
        Ok(())
    } else {
        Err(InvalidNamespace::InvalidSyntax)
    }
}

impl fmt::Display for IdNamespace {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for IdNamespace {
    type Err = InvalidNamespace;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

impl Serialize for IdNamespace {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for IdNamespace {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        Self::new(value).map_err(de::Error::custom)
    }
}

// ============================================================================
// ID VALIDATION ERRORS
// ============================================================================

/// Errors encountered when constructing or parsing a typed identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum InvalidId {
    /// The numeric identity is zero.
    Zero,

    /// The textual identity does not follow the required grammar.
    InvalidSyntax,

    /// The namespace is invalid.
    InvalidNamespace(InvalidNamespace),

    /// The numeric component exceeds u128.
    NumericOverflow,

    /// The number has leading zeroes or another noncanonical representation.
    NonCanonicalNumber,
}

impl fmt::Display for InvalidId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Zero => {
                formatter.write_str(
                    "identifier value must be nonzero",
                )
            }

            Self::InvalidSyntax => {
                formatter.write_str(
                    "identifier must use namespace:decimal-value syntax",
                )
            }

            Self::InvalidNamespace(error) => error.fmt(formatter),

            Self::NumericOverflow => {
                formatter.write_str(
                    "identifier numeric component exceeds u128",
                )
            }

            Self::NonCanonicalNumber => {
                formatter.write_str(
                    "identifier number must not contain leading zeroes",
                )
            }
        }
    }
}

impl std::error::Error for InvalidId {}

impl From<InvalidNamespace> for InvalidId {
    fn from(error: InvalidNamespace) -> Self {
        Self::InvalidNamespace(error)
    }
}

// ============================================================================
// INTERNAL ID REPRESENTATION
// ============================================================================

/// Validated namespace-qualified identity.
///
/// Private representation prevents callers from bypassing validation.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct ScopedId {
    namespace: IdNamespace,
    value: NonZeroU128,
}

impl ScopedId {
    fn new(
        namespace: IdNamespace,
        value: u128,
    ) -> Result<Self, InvalidId> {
        let value = NonZeroU128::new(value).ok_or(InvalidId::Zero)?;

        Ok(Self { namespace, value })
    }

    fn parse(text: &str) -> Result<Self, InvalidId> {
        let (namespace_text, number_text) = text
            .split_once(':')
            .ok_or(InvalidId::InvalidSyntax)?;

        if number_text.is_empty()
            || !number_text.bytes().all(|byte| byte.is_ascii_digit())
        {
            return Err(InvalidId::InvalidSyntax);
        }

        if number_text.len() > 1 && number_text.starts_with('0') {
            return Err(InvalidId::NonCanonicalNumber);
        }

        let namespace = IdNamespace::new(namespace_text)?;

        let value = number_text
            .parse::<u128>()
            .map_err(|_| InvalidId::NumericOverflow)?;

        Self::new(namespace, value)
    }

    fn namespace(&self) -> &IdNamespace {
        &self.namespace
    }

    fn value(&self) -> u128 {
        self.value.get()
    }
}

impl fmt::Display for ScopedId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}:{}",
            self.namespace,
            self.value
        )
    }
}

// ============================================================================
// PUBLIC TYPED ID CONTRACT
// ============================================================================

/// Common behavior of toolchain-owned typed identifiers.
///
/// Implementations must preserve category identity and validate all
/// externally supplied values.
///
/// Implementing this trait does not grant an identity global uniqueness.
pub trait ToolchainTypedId:
    Clone
    + Eq
    + Ord
    + Hash
    + fmt::Debug
    + fmt::Display
    + Send
    + Sync
    + 'static
{
    /// Construct this identity from a validated namespace and nonzero value.
    fn try_new(
        namespace: IdNamespace,
        value: u128,
    ) -> Result<Self, InvalidId>
    where
        Self: Sized;

    /// Borrow the namespace.
    fn namespace(&self) -> &IdNamespace;

    /// Return the opaque numeric identity.
    fn value(&self) -> u128;
}

// ============================================================================
// STRONGLY TYPED IDENTIFIER DEFINITIONS
// ============================================================================

macro_rules! define_toolchain_id {
    (
        $(#[$meta:meta])*
        $name:ident
    ) => {
        $(#[$meta])*
        #[derive(Clone)]
        pub struct $name(ScopedId);

        impl $name {
            /// Construct a validated identity.
            pub fn new(
                namespace: IdNamespace,
                value: u128,
            ) -> Result<Self, InvalidId> {
                Ok(Self(ScopedId::new(namespace, value)?))
            }

            /// Parse a canonical namespace-qualified identity.
            pub fn parse(text: &str) -> Result<Self, InvalidId> {
                Ok(Self(ScopedId::parse(text)?))
            }

            /// Borrow the identity namespace.
            #[must_use]
            pub fn namespace(&self) -> &IdNamespace {
                self.0.namespace()
            }

            /// Return the opaque numeric component.
            #[must_use]
            pub fn value(&self) -> u128 {
                self.0.value()
            }

            /// Return the canonical textual representation.
            #[must_use]
            pub fn canonical_string(&self) -> String {
                self.to_string()
            }
        }

        impl ToolchainTypedId for $name {
            fn try_new(
                namespace: IdNamespace,
                value: u128,
            ) -> Result<Self, InvalidId> {
                Self::new(namespace, value)
            }

            fn namespace(&self) -> &IdNamespace {
                self.0.namespace()
            }

            fn value(&self) -> u128 {
                self.0.value()
            }
        }

        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }

        impl Eq for $name {}

        impl PartialOrd for $name {
            fn partial_cmp(
                &self,
                other: &Self,
            ) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Ord for $name {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.0.cmp(&other.0)
            }
        }

        impl Hash for $name {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.0.hash(state);
            }
        }

        impl fmt::Debug for $name {
            fn fmt(
                &self,
                formatter: &mut fmt::Formatter<'_>,
            ) -> fmt::Result {
                write!(
                    formatter,
                    concat!(stringify!($name), "({})"),
                    self.0
                )
            }
        }

        impl fmt::Display for $name {
            fn fmt(
                &self,
                formatter: &mut fmt::Formatter<'_>,
            ) -> fmt::Result {
                self.0.fmt(formatter)
            }
        }

        impl FromStr for $name {
            type Err = InvalidId;

            fn from_str(text: &str) -> Result<Self, Self::Err> {
                Self::parse(text)
            }
        }

        impl Serialize for $name {
            fn serialize<S>(
                &self,
                serializer: S,
            ) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.serialize_str(&self.to_string())
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(
                deserializer: D,
            ) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let text = String::deserialize(deserializer)?;

                Self::parse(&text).map_err(de::Error::custom)
            }
        }
    };
}

// ============================================================================
// TOOLCHAIN-OWNED ID CATEGORIES
// ============================================================================

define_toolchain_id!(
    /// Identifies one Zamani project.
    ///
    /// Project identity is independent of its display name, directory,
    /// repository URL, and manifest contents.
    ProjectId
);

define_toolchain_id!(
    /// Identifies one Zamani workspace.
    ///
    /// A workspace may contain multiple projects and members.
    WorkspaceId
);

define_toolchain_id!(
    /// Identifies one package in the Zamani package ecosystem.
    ///
    /// Package identity is independent of package version and artifact
    /// content. Version constraints belong to dependency management.
    PackageId
);

define_toolchain_id!(
    /// Identifies one toolchain task.
    ///
    /// This is not a compiler IR operation ID or runtime scheduler ID.
    TaskId
);

define_toolchain_id!(
    /// Identifies one logical toolchain build operation.
    ///
    /// A build may produce multiple target-specific artifacts.
    BuildId
);

define_toolchain_id!(
    /// Identifies one configured package registry.
    RegistryId
);

define_toolchain_id!(
    /// Identifies one dependency or package source.
    ///
    /// Examples include local paths, configured registries and registered
    /// remote source providers.
    SourceId
);

define_toolchain_id!(
    /// Identifies one registered toolchain extension provider.
    ///
    /// The provider's implementation and capabilities are maintained by
    /// its owning provider registry.
    ProviderId
);

define_toolchain_id!(
    /// Identifies one managed build or execution environment.
    EnvironmentId
);

define_toolchain_id!(
    /// Identifies one deployment operation.
    DeploymentId
);

define_toolchain_id!(
    /// Identifies one named toolchain configuration profile.
    ProfileId
);

// ============================================================================
// ALLOCATION ERRORS
// ============================================================================

/// Allocation failure classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum IdAllocationError {
    /// The nonzero u128 identity space has been exhausted.
    Exhausted,

    /// The allocator mutex was poisoned by a previous panic.
    LockPoisoned,
}

impl fmt::Display for IdAllocationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Exhausted => {
                formatter.write_str(
                    "toolchain identifier allocator exhausted its u128 space",
                )
            }

            Self::LockPoisoned => {
                formatter.write_str(
                    "toolchain identifier allocator lock was poisoned",
                )
            }
        }
    }
}

impl std::error::Error for IdAllocationError {}

// ============================================================================
// THREAD-SAFE ID ALLOCATION
// ============================================================================

/// Thread-safe monotonic allocator for one typed ID category.
///
/// Each allocator has an independent namespace and sequence.
///
/// It does not use global mutable state, random global counters, wrapping
/// arithmetic, or value reuse.
///
/// Separate allocator instances are not automatically coordinated.
pub struct IdAllocator<T: ToolchainTypedId> {
    namespace: IdNamespace,
    next: Mutex<Option<u128>>,
    marker: PhantomData<fn() -> T>,
}

impl<T: ToolchainTypedId> IdAllocator<T> {
    /// Create an allocator starting at the first nonzero identity.
    pub fn new(namespace: IdNamespace) -> Self {
        Self {
            namespace,
            next: Mutex::new(Some(1)),
            marker: PhantomData,
        }
    }

    /// Restore an allocator from a persisted next-value checkpoint.
    ///
    /// The supplied value must be nonzero. The caller is responsible for
    /// ensuring that the checkpoint is durable and correctly coordinated
    /// with previously issued identities.
    pub fn with_next(
        namespace: IdNamespace,
        next_value: u128,
    ) -> Result<Self, InvalidId> {
        if next_value == 0 {
            return Err(InvalidId::Zero);
        }

        Ok(Self {
            namespace,
            next: Mutex::new(Some(next_value)),
            marker: PhantomData,
        })
    }

    /// Borrow the allocator namespace.
    #[must_use]
    pub fn namespace(&self) -> &IdNamespace {
        &self.namespace
    }

    /// Allocate the next identity.
    ///
    /// Allocation is serialized across concurrent callers.
    ///
    /// The maximum nonzero u128 value is valid and can be issued exactly
    /// once. Subsequent allocations return `Exhausted`.
    pub fn try_next(&self) -> Result<T, IdAllocationError> {
        let mut guard = self
            .next
            .lock()
            .map_err(|_| IdAllocationError::LockPoisoned)?;

        let current = guard.ok_or(IdAllocationError::Exhausted)?;

        // Construct the identity before advancing the sequence.
        // If construction fails, the current value is not consumed.
        let identity = T::try_new(self.namespace.clone(), current)
            .map_err(|_| IdAllocationError::Exhausted)?;

        // checked_add prevents wraparound.
        // None permanently represents exhaustion.
        *guard = current.checked_add(1);

        Ok(identity)
    }

    /// Report whether this allocator has exhausted its value space.
    ///
    /// The result is a snapshot. Another thread may allocate immediately
    /// after this method returns.
    pub fn is_exhausted(&self) -> Result<bool, IdAllocationError> {
        let guard = self
            .next
            .lock()
            .map_err(|_| IdAllocationError::LockPoisoned)?;

        Ok(guard.is_none())
    }
}

impl<T: ToolchainTypedId> fmt::Debug for IdAllocator<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let exhausted = self.is_exhausted();

        formatter
            .debug_struct("IdAllocator")
            .field("id_type", &std::any::type_name::<T>())
            .field("namespace", &self.namespace)
            .field("exhausted", &exhausted)
            .finish()
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::{
        collections::HashSet,
        sync::Arc,
        thread,
    };

    fn namespace(value: &str) -> IdNamespace {
        IdNamespace::new(value).expect("valid test namespace")
    }

    #[test]
    fn namespace_validation() {
        assert_eq!(
            IdNamespace::new(""),
            Err(InvalidNamespace::Empty)
        );

        assert!(IdNamespace::new("Backend").is_err());
        assert!(IdNamespace::new("1backend").is_err());
        assert!(IdNamespace::new("backend/target").is_err());
        assert!(IdNamespace::new("backend:target").is_err());
        assert!(IdNamespace::new("backend target").is_err());

        assert!(IdNamespace::new("zamani.toolchain").is_ok());
        assert!(IdNamespace::new("provider_01").is_ok());
        assert!(IdNamespace::new("vendor-quantum.v2").is_ok());
    }

    #[test]
    fn namespace_round_trip() {
        let original = namespace("zamani.toolchain.project");

        let encoded =
            serde_json::to_string(&original).expect("serialize");

        let decoded: IdNamespace =
            serde_json::from_str(&encoded).expect("deserialize");

        assert_eq!(original, decoded);
    }

    #[test]
    fn namespace_comparison_is_case_sensitive() {
        let first = namespace("provider.alpha");
        let second = namespace("provider.beta");

        assert_ne!(first, second);
    }

    #[test]
    fn zero_is_rejected() {
        assert_eq!(
            ProjectId::new(namespace("zamani.project"), 0),
            Err(InvalidId::Zero)
        );

        assert_eq!(
            TaskId::parse("zamani.task:0"),
            Err(InvalidId::Zero)
        );
    }

    #[test]
    fn canonical_identity_round_trip() {
        let original =
            ProjectId::new(namespace("zamani.project"), 42)
                .expect("valid identity");

        assert_eq!(
            original.to_string(),
            "zamani.project:42"
        );

        assert_eq!(
            ProjectId::parse(&original.to_string())
                .expect("parse identity"),
            original
        );

        assert_eq!(
            "zamani.project:42"
                .parse::<ProjectId>()
                .expect("parse identity"),
            original
        );
    }

    #[test]
    fn rejects_noncanonical_numeric_values() {
        assert_eq!(
            ProjectId::parse("zamani.project:042"),
            Err(InvalidId::NonCanonicalNumber)
        );

        assert_eq!(
            ProjectId::parse("zamani.project:+42"),
            Err(InvalidId::InvalidSyntax)
        );

        assert_eq!(
            ProjectId::parse("zamani.project:-42"),
            Err(InvalidId::InvalidSyntax)
        );

        assert_eq!(
            ProjectId::parse("zamani.project:"),
            Err(InvalidId::InvalidSyntax)
        );
    }

    #[test]
    fn rejects_numeric_overflow() {
        assert_eq!(
            ProjectId::parse(
                "zamani.project:340282366920938463463374607431768211456"
            ),
            Err(InvalidId::NumericOverflow)
        );
    }

    #[test]
    fn identity_categories_are_distinct() {
        let project =
            ProjectId::new(namespace("zamani"), 7)
                .expect("valid project");

        let package =
            PackageId::new(namespace("zamani"), 7)
                .expect("valid package");

        assert_eq!(project.value(), package.value());

        assert_ne!(
            project.to_string(),
            package.to_string()
        );

        assert_ne!(
            format!("{project:?}"),
            format!("{package:?}")
        );
    }

    #[test]
    fn namespaces_are_part_of_identity() {
        let first =
            ProjectId::new(namespace("project.alpha"), 1)
                .expect("valid identity");

        let second =
            ProjectId::new(namespace("project.beta"), 1)
                .expect("valid identity");

        assert_ne!(first, second);
    }

    #[test]
    fn ordering_and_hashing_are_consistent() {
        let first =
            PackageId::new(namespace("zamani.package"), 1)
                .expect("valid package");

        let second =
            PackageId::new(namespace("zamani.package"), 2)
                .expect("valid package");

        assert!(first < second);

        let set: HashSet<_> = [first, second].into_iter().collect();

        assert_eq!(set.len(), 2);
    }

    #[test]
    fn allocator_is_monotonic() {
        let allocator =
            IdAllocator::<ProjectId>::new(namespace("zamani.project"));

        let first = allocator.try_next().expect("first identity");
        let second = allocator.try_next().expect("second identity");

        assert_eq!(first.value(), 1);
        assert_eq!(second.value(), 2);

        assert_eq!(first.namespace(), second.namespace());

        assert!(!allocator.is_exhausted().expect("allocator state"));
    }

    #[test]
    fn allocator_supports_restored_sequences() {
        let allocator =
            IdAllocator::<PackageId>::with_next(
                namespace("zamani.package"),
                500,
            )
            .expect("valid checkpoint");

        assert_eq!(
            allocator.try_next().expect("allocated ID").value(),
            500
        );

        assert_eq!(
            allocator.try_next().expect("allocated ID").value(),
            501
        );
    }

    #[test]
    fn allocator_rejects_zero_checkpoint() {
        assert!(
            IdAllocator::<TaskId>::with_next(
                namespace("zamani.task"),
                0,
            )
            .is_err()
        );
    }

    #[test]
    fn allocator_instances_are_independent() {
        let first =
            IdAllocator::<BuildId>::new(namespace("build.alpha"));

        let second =
            IdAllocator::<BuildId>::new(namespace("build.beta"));

        assert_eq!(
            first.try_next().expect("first").value(),
            1
        );

        assert_eq!(
            second.try_next().expect("second").value(),
            1
        );
    }

    #[test]
    fn maximum_nonzero_value_can_be_allocated_once() {
        let allocator =
            IdAllocator::<DeploymentId>::with_next(
                namespace("zamani.deployment"),
                u128::MAX,
            )
            .expect("valid maximum checkpoint");

        let final_id = allocator
            .try_next()
            .expect("maximum value is valid");

        assert_eq!(final_id.value(), u128::MAX);

        assert_eq!(
            allocator.try_next(),
            Err(IdAllocationError::Exhausted)
        );

        assert!(
            allocator.is_exhausted().expect("allocator state")
        );
    }

    #[test]
    fn allocator_never_wraps_or_reuses_values() {
        let allocator =
            IdAllocator::<TaskId>::with_next(
                namespace("zamani.task"),
                u128::MAX,
            )
            .expect("valid checkpoint");

        assert_eq!(
            allocator.try_next().expect("maximum ID").value(),
            u128::MAX
        );

        for _ in 0..3 {
            assert_eq!(
                allocator.try_next(),
                Err(IdAllocationError::Exhausted)
            );
        }
    }

    #[test]
    fn concurrent_allocations_are_unique() {
        let allocator = Arc::new(
            IdAllocator::<TaskId>::new(namespace("zamani.task")),
        );

        let worker_count = thread::available_parallelism()
            .map(|value| value.get())
            .unwrap_or(1);

        let mut handles = Vec::new();

        for _ in 0..worker_count {
            let allocator = Arc::clone(&allocator);

            handles.push(thread::spawn(move || {
                (0..128)
                    .map(|_| {
                        allocator
                            .try_next()
                            .expect("identity allocation")
                    })
                    .collect::<Vec<_>>()
            }));
        }

        let mut identities = Vec::new();

        for handle in handles {
            identities.extend(
                handle.join().expect("worker completed"),
            );
        }

        let unique: HashSet<_> =
            identities.iter().cloned().collect();

        assert_eq!(unique.len(), identities.len());
    }

    #[test]
    fn serialization_preserves_identity() {
        let original =
            RegistryId::new(
                namespace("zamani.registry"),
                123,
            )
            .expect("valid registry");

        let encoded =
            serde_json::to_string(&original).expect("serialize");

        assert_eq!(
            encoded,
            "\"zamani.registry:123\""
        );

        let decoded: RegistryId =
            serde_json::from_str(&encoded).expect("deserialize");

        assert_eq!(decoded, original);
    }

    #[test]
    fn serialization_rejects_invalid_identity() {
        assert!(
            serde_json::from_str::<ProjectId>(
                "\"Project:1\""
            )
            .is_err()
        );

        assert!(
            serde_json::from_str::<PackageId>(
                "\"zamani.package:0\""
            )
            .is_err()
        );

        assert!(
            serde_json::from_str::<TaskId>(
                "\"zamani.task:01\""
            )
            .is_err()
        );
    }

    #[test]
    fn all_toolchain_id_categories_construct() {
        let namespace = namespace("zamani");

        assert!(ProjectId::new(namespace.clone(), 1).is_ok());
        assert!(WorkspaceId::new(namespace.clone(), 1).is_ok());
        assert!(PackageId::new(namespace.clone(), 1).is_ok());
        assert!(TaskId::new(namespace.clone(), 1).is_ok());
        assert!(BuildId::new(namespace.clone(), 1).is_ok());
        assert!(RegistryId::new(namespace.clone(), 1).is_ok());
        assert!(SourceId::new(namespace.clone(), 1).is_ok());
        assert!(ProviderId::new(namespace.clone(), 1).is_ok());
        assert!(EnvironmentId::new(namespace.clone(), 1).is_ok());
        assert!(DeploymentId::new(namespace.clone(), 1).is_ok());
        assert!(ProfileId::new(namespace, 1).is_ok());
    }
}

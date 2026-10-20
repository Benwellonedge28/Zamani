
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani HDL Intermediate Representation — Typed Identifiers
//!
//! Path: `src/hdl/hdlir/ids.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: Rust 1.97
//! Safety: Safe Rust only.
//! Status: Foundational HDLIR identity contract.
//!
//! ## Purpose
//!
//! Define strongly typed identities for entities owned by HDLIR.
//! This module deliberately has no dependency on the HDLIR graph,
//! builders, verifier, dialect implementations, serialization, or lowering.
//!
//! ## Architectural authority
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/specification/semantic-model.md`
//! - `src/hdl/ids.rs`
//! - `src/hdl/api.rs`
//! - `src/frontend/ast/node/node_id.rs`
//! - `src/frontend/semantic/model/ids.rs`
//! - `src/zuir/ids.rs`
//!
//! Source syntax and semantic identity remain owned by the frontend.
//! HDL-specific identities that already exist remain owned by `src/hdl/ids.rs`.
//! ZUIR identities remain owned by `src/zuir/ids.rs`.
//!
//! This module defines identities for entities in the HDLIR representation;
//! it does not replace or alias the identity systems above.
//!
//! ## Owns
//!
//! - HDLIR-specific typed identifiers.
//! - Validation of raw HDLIR identifier values.
//! - Explicit deterministic ID allocation.
//! - Checked allocator exhaustion.
//! - Numeric formatting and checked host-index conversion.
//!
//! ## Does not own
//!
//! - AST node IDs, semantic symbol IDs, semantic module IDs, or source spans.
//! - HDL source-level instance, clock-domain, or library IDs where an
//!   existing canonical HDL identity is the correct identity.
//! - ZUIR IDs.
//! - Source names, name resolution, symbol tables, or semantic facts.
//! - Physical pins, gates, cells, registers, devices, placement, routing,
//!   vendor resource IDs, or runtime handles.
//! - Persistent cross-build identity or content fingerprints.
//!
//! ## Identity scope
//!
//! An ID is unique only within the namespace and lifetime established by
//! its owning HDLIR design, module, registry, or compilation context.
//! Two independently created allocators may produce the same numeric value.
//! Combining independently allocated graphs therefore requires an explicit
//! ownership, namespace, or remapping policy.
//!
//! The numeric value is not a collection index, source location, physical
//! resource number, execution order, content fingerprint, or hardware size.
//!
//! ## Scalability
//!
//! The representation uses non-zero `u128` values. It has a finite numeric
//! identity space, but defines no language-level limit on modules, ports,
//! signals, operations, values, clock domains, memories, devices, or hardware
//! resources. Exhaustion is reported explicitly; IDs never wrap or get reused.
//!
//! This module does not allocate memory proportional to the number of IDs.
//! An allocator stores only its next available ID. Operational resource
//! budgets belong to the compiler stages that perform the work.
//!
//! ## Determinism
//!
//! Allocation is deterministic for a given starting value and allocation
//! order. Independent allocators are independent. IDs are not automatically
//! stable across compilations, graph reconstruction, or serialization.
//! Persistent identity must be separately specified by the owning format.
//!
//! ## Integration
//!
//! `ir/*`, `schema/*`, and `builder/*` may depend on this module.
//! This module must not depend on those higher-level modules.
//!
//! Reuse canonical source and semantic identities when referring to existing
//! frontend entities. Allocate HDLIR IDs for HDLIR-owned graph entities.
//! Keep physical realization IDs downstream.
//!
//! ## Freeze criteria
//!
//! - Every HDLIR entity category has a distinct Rust type.
//! - Zero is rejected.
//! - Allocators are explicit, deterministic, and non-wrapping.
//! - Exhaustion is reported without panic or identity reuse.
//! - No AST, semantic, ZUIR, or physical-resource identity is duplicated.
//! - Unit tests cover boundaries, parsing, type separation, allocation,
//!   custom origins, and exhaustion.
//! - Formatting, compilation, tests, and linting pass in the actual checkout.

use core::fmt;
use core::marker::PhantomData;
use core::num::NonZeroU128;
use core::str::FromStr;

/// The first permitted numeric value for an HDLIR identifier.
///
/// Zero is reserved for invalid or unassigned identifiers. This is an
/// identity-format invariant, not a hardware-capacity limit.
pub const FIRST_HDLIR_ID: u128 = 1;

/// Error returned when a raw identifier value is invalid.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum InvalidHdlirId {
    /// Zero is reserved and cannot represent an assigned identity.
    Zero,
}

impl fmt::Display for InvalidHdlirId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Zero => formatter.write_str("HDLIR identifier must be non-zero"),
        }
    }
}

impl std::error::Error for InvalidHdlirId {}

/// Error returned when textual identifier parsing fails.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HdlirIdParseError {
    /// The input is not a valid unsigned 128-bit decimal integer.
    InvalidInteger,
    /// The parsed integer is zero.
    Zero,
}

impl fmt::Display for HdlirIdParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidInteger => {
                formatter.write_str("invalid unsigned decimal HDLIR identifier")
            }
            Self::Zero => formatter.write_str("HDLIR identifier must be non-zero"),
        }
    }
}

impl std::error::Error for HdlirIdParseError {}

/// Error returned when an allocator has no unused identifier remaining.
///
/// An exhausted allocator stays exhausted. It never wraps, resets
/// automatically, or issues a previously allocated identifier.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct HdlirIdSpaceExhausted;

impl fmt::Display for HdlirIdSpaceExhausted {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("HDLIR identifier allocator exhausted its u128 value space")
    }
}

impl std::error::Error for HdlirIdSpaceExhausted {}

/// Common contract implemented by HDLIR-owned typed identifiers.
///
/// Implementations must preserve the non-zero invariant. This trait is
/// intended for the identifier types declared in this module; external
/// implementations must not violate the documented identity contract.
pub trait HdlirIdValue:
    Copy + Eq + Ord + core::hash::Hash + fmt::Debug + fmt::Display
{
    /// Construct an ID from a validated, non-zero integer.
    ///
    /// This method is public so the generic allocator can construct IDs.
    /// Normal callers should use a concrete type's checked constructor.
    #[doc(hidden)]
    fn from_nonzero(value: NonZeroU128) -> Self;

    /// Return the opaque numeric identity value.
    fn get(self) -> u128;

    /// Construct an ID from a raw integer, rejecting zero.
    fn from_raw(value: u128) -> Result<Self, InvalidHdlirId> {
        NonZeroU128::new(value)
            .map(Self::from_nonzero)
            .ok_or(InvalidHdlirId::Zero)
    }

    /// Return whether this ID has the first permitted numeric value.
    fn is_first(self) -> bool {
        self.get() == FIRST_HDLIR_ID
    }
}

/// Declare a distinct, non-zero HDLIR identity type.
///
/// The private field prevents callers from constructing an invalid value
/// without using a checked constructor.
macro_rules! define_hdlir_id {
    ($(#[$meta:meta])* $name:ident) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(NonZeroU128);

        impl $name {
            /// Construct an identifier from a non-zero numeric value.
            ///
            /// Returns `None` for zero.
            #[must_use]
            pub const fn new(value: u128) -> Option<Self> {
                match NonZeroU128::new(value) {
                    Some(value) => Some(Self(value)),
                    None => None,
                }
            }

            /// Construct an identifier from a raw value with a structured
            /// error when the value is invalid.
            pub fn try_from_raw(value: u128) -> Result<Self, InvalidHdlirId> {
                Self::new(value).ok_or(InvalidHdlirId::Zero)
            }

            /// Return the opaque numeric identity value.
            #[must_use]
            pub const fn get(self) -> u128 {
                self.0.get()
            }

            /// Return the underlying non-zero representation.
            #[must_use]
            pub const fn get_nonzero(self) -> NonZeroU128 {
                self.0
            }

            /// Return whether this is the first permitted identifier value.
            #[must_use]
            pub const fn is_first(self) -> bool {
                self.get() == FIRST_HDLIR_ID
            }

            /// Convert to a host collection index when representable.
            ///
            /// An ID is not inherently an index. Callers must additionally
            /// check collection bounds and the relevant identity namespace.
            #[must_use]
            pub fn try_as_usize(self) -> Option<usize> {
                usize::try_from(self.get()).ok()
            }
        }

        impl HdlirIdValue for $name {
            fn from_nonzero(value: NonZeroU128) -> Self {
                Self(value)
            }

            fn get(self) -> u128 {
                self.get()
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter
                    .debug_tuple(stringify!($name))
                    .field(&self.get())
                    .finish()
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(formatter, "{}", self.get())
            }
        }

        impl From<$name> for u128 {
            fn from(value: $name) -> Self {
                value.get()
            }
        }

        impl TryFrom<u128> for $name {
            type Error = InvalidHdlirId;

            fn try_from(value: u128) -> Result<Self, Self::Error> {
                Self::try_from_raw(value)
            }
        }

        impl FromStr for $name {
            type Err = HdlirIdParseError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                let raw = value
                    .parse::<u128>()
                    .map_err(|_| HdlirIdParseError::InvalidInteger)?;

                Self::new(raw).ok_or(HdlirIdParseError::Zero)
            }
        }
    };
}

// -----------------------------------------------------------------------------
// HDLIR graph identities
// -----------------------------------------------------------------------------
//
// These are identities in the hardware-domain IR, not aliases for semantic
// model IDs or ZUIR IDs. Numeric equality across categories does not make
// their entities interchangeable.

define_hdlir_id!(
    /// Identity of a design in an HDLIR ownership scope.
    HdlirDesignId
);

define_hdlir_id!(
    /// Identity of an HDLIR module definition.
    HdlirModuleId
);

define_hdlir_id!(
    /// Identity of a module/component instance in an HDLIR graph.
    HdlirInstanceId
);

define_hdlir_id!(
    /// Identity of an HDLIR port declaration.
    HdlirPortId
);

define_hdlir_id!(
    /// Identity of a logical HDLIR signal declaration.
    HdlirSignalId
);

define_hdlir_id!(
    /// Identity of a logical HDLIR net or connectivity object.
    HdlirNetId
);

define_hdlir_id!(
    /// Identity of a hardware-domain operation in HDLIR.
    HdlirOperationId
);

define_hdlir_id!(
    /// Identity of an HDLIR value, such as an operation result or block argument.
    HdlirValueId
);

define_hdlir_id!(
    /// Identity of a nested HDLIR region.
    HdlirRegionId
);

define_hdlir_id!(
    /// Identity of an HDLIR block.
    HdlirBlockId
);

define_hdlir_id!(
    /// Identity of an HDLIR process or behavioral process record.
    HdlirProcessId
);

define_hdlir_id!(
    /// Identity of a logical HDLIR clock-domain record.
    HdlirClockDomainId
);

define_hdlir_id!(
    /// Identity of an abstract HDLIR memory declaration or instance.
    HdlirMemoryId
);

define_hdlir_id!(
    /// Identity of a type entry in the HDLIR type namespace.
    HdlirTypeId
);

define_hdlir_id!(
    /// Identity of a registered HDLIR dialect.
    HdlirDialectId
);

define_hdlir_id!(
    /// Identity of a registered HDLIR extension or extension manifest.
    HdlirExtensionId
);

define_hdlir_id!(
    /// Identity of a registered HDLIR schema definition.
    HdlirSchemaId
);

define_hdlir_id!(
    /// Identity of a symbol in an HDLIR-owned symbol namespace.
    HdlirSymbolId
);

/// Categories of HDLIR identity.
///
/// This enum provides stable labels for diagnostics and tooling. It is not
/// a substitute for the distinct identifier types.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum HdlirIdentityKind {
    /// HDLIR design.
    Design,
    /// HDLIR module definition.
    Module,
    /// HDLIR module/component instance.
    Instance,
    /// HDLIR port.
    Port,
    /// HDLIR logical signal.
    Signal,
    /// HDLIR net or connectivity object.
    Net,
    /// HDLIR operation.
    Operation,
    /// HDLIR value.
    Value,
    /// HDLIR region.
    Region,
    /// HDLIR block.
    Block,
    /// HDLIR behavioral process.
    Process,
    /// HDLIR logical clock domain.
    ClockDomain,
    /// HDLIR abstract memory.
    Memory,
    /// HDLIR type.
    Type,
    /// HDLIR dialect.
    Dialect,
    /// HDLIR extension.
    Extension,
    /// HDLIR schema definition.
    Schema,
    /// HDLIR symbol.
    Symbol,
}

impl HdlirIdentityKind {
    /// Return the stable lowercase label for diagnostics and external tools.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Design => "design",
            Self::Module => "module",
            Self::Instance => "instance",
            Self::Port => "port",
            Self::Signal => "signal",
            Self::Net => "net",
            Self::Operation => "operation",
            Self::Value => "value",
            Self::Region => "region",
            Self::Block => "block",
            Self::Process => "process",
            Self::ClockDomain => "clock_domain",
            Self::Memory => "memory",
            Self::Type => "type",
            Self::Dialect => "dialect",
            Self::Extension => "extension",
            Self::Schema => "schema",
            Self::Symbol => "symbol",
        }
    }
}

// -----------------------------------------------------------------------------
// Explicit, checked allocation
// -----------------------------------------------------------------------------

/// Allocate typed HDLIR identities from a caller-owned numeric sequence.
///
/// The allocator has constant-size state and performs no allocation itself.
/// Its owner must decide the namespace in which generated IDs are unique.
///
/// Separate allocators are independent and may issue equal numeric values.
/// Use one allocator per required uniqueness scope, or implement an explicit
/// remapping policy when combining independently allocated graphs.
///
/// The allocator can issue `u128::MAX` once. It then becomes exhausted.
/// It never wraps to zero or reuses an issued value.
#[derive(Clone, Debug)]
pub struct HdlirIdAllocator<I: HdlirIdValue> {
    next: Option<NonZeroU128>,
    marker: PhantomData<fn() -> I>,
}

impl<I: HdlirIdValue> Default for HdlirIdAllocator<I> {
    fn default() -> Self {
        Self::new()
    }
}

impl<I: HdlirIdValue> HdlirIdAllocator<I> {
    /// Create an allocator whose first ID is one.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            next: NonZeroU128::new(FIRST_HDLIR_ID),
            marker: PhantomData,
        }
    }

    /// Create an allocator with a caller-selected first ID.
    ///
    /// This is useful when allocating within an established namespace or
    /// restoring allocator state under an explicitly defined persistence
    /// contract. This method does not check for collisions with IDs already
    /// present elsewhere.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidHdlirId::Zero`] when `first` is zero.
    pub fn with_start(first: u128) -> Result<Self, InvalidHdlirId> {
        let next = NonZeroU128::new(first).ok_or(InvalidHdlirId::Zero)?;

        Ok(Self {
            next: Some(next),
            marker: PhantomData,
        })
    }

    /// Allocate the next identifier.
    ///
    /// # Errors
    ///
    /// Returns [`HdlirIdSpaceExhausted`] if the allocator has no remaining
    /// identifier. Exhaustion is permanent for this allocator.
    pub fn allocate(&mut self) -> Result<I, HdlirIdSpaceExhausted> {
        let current = self.next.take().ok_or(HdlirIdSpaceExhausted)?;

        self.next = current
            .get()
            .checked_add(1)
            .and_then(NonZeroU128::new);

        Ok(I::from_nonzero(current))
    }

    /// Return the next numeric value, or `None` if exhausted.
    ///
    /// This is primarily useful for diagnostics and explicit state management.
    /// It does not expose mutable access to allocator state.
    #[must_use]
    pub const fn next_value(&self) -> Option<u128> {
        match self.next {
            Some(value) => Some(value.get()),
            None => None,
        }
    }

    /// Return whether this allocator is exhausted.
    #[must_use]
    pub const fn is_exhausted(&self) -> bool {
        self.next.is_none()
    }
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_identity_type_rejects_zero() {
        assert!(HdlirDesignId::new(0).is_none());
        assert!(HdlirModuleId::new(0).is_none());
        assert!(HdlirInstanceId::new(0).is_none());
        assert!(HdlirPortId::new(0).is_none());
        assert!(HdlirSignalId::new(0).is_none());
        assert!(HdlirNetId::new(0).is_none());
        assert!(HdlirOperationId::new(0).is_none());
        assert!(HdlirValueId::new(0).is_none());
        assert!(HdlirRegionId::new(0).is_none());
        assert!(HdlirBlockId::new(0).is_none());
        assert!(HdlirProcessId::new(0).is_none());
        assert!(HdlirClockDomainId::new(0).is_none());
        assert!(HdlirMemoryId::new(0).is_none());
        assert!(HdlirTypeId::new(0).is_none());
        assert!(HdlirDialectId::new(0).is_none());
        assert!(HdlirExtensionId::new(0).is_none());
        assert!(HdlirSchemaId::new(0).is_none());
        assert!(HdlirSymbolId::new(0).is_none());
    }

    #[test]
    fn distinct_entity_categories_are_distinct_rust_types() {
        let module = HdlirModuleId::new(7).expect("nonzero ID");
        let operation = HdlirOperationId::new(7).expect("nonzero ID");
        let value = HdlirValueId::new(7).expect("nonzero ID");

        assert_eq!(module.get(), operation.get());
        assert_eq!(operation.get(), value.get());

        // Distinct Rust types prevent accidental cross-category assignment.
        let _: HdlirModuleId = module;
        let _: HdlirOperationId = operation;
        let _: HdlirValueId = value;
    }

    #[test]
    fn identifiers_support_the_full_nonzero_u128_range() {
        let largest = HdlirOperationId::new(u128::MAX)
            .expect("u128::MAX is a valid nonzero identity");

        assert_eq!(largest.get(), u128::MAX);
        assert_eq!(largest.to_string(), u128::MAX.to_string());
        assert_eq!(largest.try_as_usize(), usize::try_from(u128::MAX).ok());
    }

    #[test]
    fn parsing_and_display_are_lossless_for_valid_decimal_ids() {
        let raw = u128::MAX.to_string();
        let parsed = raw
            .parse::<HdlirValueId>()
            .expect("maximum u128 should parse");

        assert_eq!(parsed.get(), u128::MAX);
        assert_eq!(parsed.to_string(), raw);
    }

    #[test]
    fn parsing_rejects_zero_invalid_text_and_overflow() {
        assert_eq!(
            "0".parse::<HdlirModuleId>(),
            Err(HdlirIdParseError::Zero)
        );
        assert_eq!(
            "not-an-id".parse::<HdlirModuleId>(),
            Err(HdlirIdParseError::InvalidInteger)
        );
        assert_eq!(
            "-1".parse::<HdlirModuleId>(),
            Err(HdlirIdParseError::InvalidInteger)
        );

        let overflow = format!("{}0", u128::MAX);
        assert_eq!(
            overflow.parse::<HdlirModuleId>(),
            Err(HdlirIdParseError::InvalidInteger)
        );
    }

    #[test]
    fn allocator_is_deterministic_for_equal_origins_and_order() {
        let mut first = HdlirIdAllocator::<HdlirOperationId>::new();
        let mut second = HdlirIdAllocator::<HdlirOperationId>::new();

        assert_eq!(first.allocate(), second.allocate());
        assert_eq!(first.allocate(), second.allocate());
        assert_eq!(first.allocate(), second.allocate());

        assert_eq!(first.next_value(), Some(4));
        assert_eq!(second.next_value(), Some(4));
    }

    #[test]
    fn allocator_supports_a_checked_custom_origin() {
        let mut allocator =
            HdlirIdAllocator::<HdlirSignalId>::with_start(41).expect("nonzero origin");

        assert_eq!(allocator.allocate().expect("ID").get(), 41);
        assert_eq!(allocator.allocate().expect("ID").get(), 42);
        assert_eq!(
            HdlirIdAllocator::<HdlirSignalId>::with_start(0).err(),
            Some(InvalidHdlirId::Zero)
        );
    }

    #[test]
    fn allocator_exhaustion_never_wraps_or_reuses_ids() {
        let mut allocator = HdlirIdAllocator::<HdlirOperationId>::with_start(u128::MAX)
            .expect("maximum value is nonzero");

        assert_eq!(
            allocator.allocate().expect("maximum ID should be issued").get(),
            u128::MAX
        );
        assert!(allocator.is_exhausted());
        assert_eq!(allocator.next_value(), None);
        assert_eq!(allocator.allocate(), Err(HdlirIdSpaceExhausted));
        assert_eq!(allocator.allocate(), Err(HdlirIdSpaceExhausted));
    }

    #[test]
    fn identity_kind_labels_are_stable() {
        assert_eq!(HdlirIdentityKind::Operation.as_str(), "operation");
        assert_eq!(HdlirIdentityKind::ClockDomain.as_str(), "clock_domain");
        assert_eq!(HdlirIdentityKind::Extension.as_str(), "extension");
    }

    #[test]
    fn allocator_state_is_independent_between_owners() {
        let mut first = HdlirIdAllocator::<HdlirModuleId>::new();
        let mut second = HdlirIdAllocator::<HdlirModuleId>::new();

        assert_eq!(first.allocate().expect("ID").get(), 1);
        assert_eq!(first.allocate().expect("ID").get(), 2);
        assert_eq!(second.allocate().expect("ID").get(), 1);

        // Equal numeric values from different allocators do not prove
        // that the represented entities belong to the same design.
        assert_eq!(first.next_value(), Some(3));
        assert_eq!(second.next_value(), Some(2));
    }
}

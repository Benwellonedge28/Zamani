
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani HDLIR — Whole-Graph Connectivity Verification
//!
//! Path: `src/hdl/hdlir/verify/connectivity.rs`
//! Edition: Rust 2021.
//! Minimum Rust version: 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Verify logical HDLIR connectivity against declarations in the enclosing
//! graph. This includes endpoint resolution, driver/sink direction, endpoint
//! type compatibility, duplicate sink ownership, and explicit driver-policy
//! validation.
//!
//! ## Authority and ownership
//!
//! Source syntax and source-level semantic truth belong to the canonical
//! grammar and frontend. ZUIR owns the universal computational representation.
//! HDLIR owns hardware-domain connectivity and this verification boundary.
//!
//! This module reuses `ir::connectivity` and does not create another
//! connectivity graph, endpoint-ID system, signal type system, or resolver
//! registry. The supplied context adapter resolves facts from the actual
//! design graph and canonical schema/type definitions.
//!
//! This module does not perform physical allocation, placement, routing,
//! synthesis, electrical analysis, or timing closure.
//!
//! ## Scalability
//!
//! No maximum number of connections, endpoints, drivers, sinks, or type
//! identities is imposed by this module. An optional work budget is an
//! operational safeguard, not a language or hardware limit. Counter overflow
//! and budget exhaustion are reported explicitly.
//!
//! A finite implementation cannot process literally infinite input. Work is
//! bounded by available resources, representable collections, configured
//! budgets, and supported semantics.
//!
//! ## Integration
//!
//! `verify/mod.rs` must register this module. The graph's verification entry
//! point supplies a `ConnectivityContext` adapter backed by the actual HDLIR
//! declarations, type compatibility rules, and registered resolver contracts.
//!
//! The adapter must validate the full relationship for `InstancePort`
//! endpoints; finding an instance and a port independently is insufficient.
//!
//! ## Completion status
//!
//! This file defines the verifier boundary. It is not production-qualified
//! until the repository's concrete graph adapter, module registrations,
//! integration tests, and required Rust checks compile and pass.

use std::collections::BTreeMap;

use crate::hdl::hdlir::errors::{
    HdlirBudgetKind,
    HdlirError,
    HdlirResult,
};
use crate::hdl::hdlir::ids::HdlirNetId;
use crate::hdl::hdlir::ir::connectivity::{
    HdlirConnection,
    HdlirConnectivity,
    HdlirDriverPolicy,
    HdlirEndpoint,
};

/// Result of checking whether a source endpoint's type can connect to a
/// destination endpoint's type.
///
/// `Indeterminate` is not equivalent to `Compatible`: unresolved symbolic
/// constraints or missing proof must never be silently accepted.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConnectivityTypeCompatibility {
    /// Compatibility has been established by the canonical type contract.
    Compatible,

    /// The types are provably incompatible for this connection.
    Incompatible,

    /// Available information does not establish compatibility.
    Indeterminate,
}

/// Resolved facts about one endpoint, obtained from the enclosing graph.
///
/// `can_drive` and `can_receive` are logical HDLIR roles, not physical pin
/// properties. For instance ports, the adapter must account for the instance
/// boundary and the referenced module's port direction.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConnectivityEndpointInfo<T> {
    /// Whether this endpoint is permitted to act as a logical driver.
    pub can_drive: bool,

    /// Whether this endpoint is permitted to act as a logical sink.
    pub can_receive: bool,

    /// Canonical type identity or type handle used by the type subsystem.
    ///
    /// This is an adapter-owned type reference, not a second HDLIR type.
    pub type_key: T,
}

/// Graph and schema facts required to verify connectivity.
///
/// Implement this trait at the HDLIR graph boundary. The implementation must
/// resolve endpoint declarations, preserve module/instance ownership rules,
/// delegate compatibility to the canonical type subsystem, and verify
/// resolver registration and semantics.
///
/// Implementations must not return `Compatible` merely because a type or
/// resolver is unknown. Missing definitions must produce a structured error;
/// unresolved compatibility must return `Indeterminate`.
pub trait ConnectivityContext {
    /// Canonical type identity used by this context.
    type TypeKey: Clone;

    /// Resolve an endpoint and return its logical direction capabilities and
    /// canonical type identity.
    ///
    /// For `HdlirEndpoint::InstancePort`, this must confirm that the port
    /// belongs to the module instantiated by the referenced instance.
    ///
    /// # Errors
    ///
    /// Return `HdlirError::MissingEntity` for missing declarations and a
    /// structured invariant/input error for malformed ownership relationships.
    fn resolve_endpoint(
        &self,
        endpoint: HdlirEndpoint,
    ) -> HdlirResult<ConnectivityEndpointInfo<Self::TypeKey>>;

    /// Determine whether a driver type is compatible with a sink type.
    ///
    /// The implementation must preserve symbolic constraints. It must not
    /// treat an unresolved width or unknown type as proof of compatibility.
    ///
    /// # Errors
    ///
    /// Return a structured HDLIR error when the type definition itself is
    /// malformed or cannot be resolved.
    fn type_compatibility(
        &self,
        driver_type: &Self::TypeKey,
        sink_type: &Self::TypeKey,
    ) -> HdlirResult<ConnectivityTypeCompatibility>;

    /// Validate the complete driver-policy contract for a connection.
    ///
    /// For `HdlirDriverPolicy::Resolved`, this must establish that the named
    /// resolver is registered, version-compatible, legal for this connection,
    /// and has a defined interpretation for the driver and sink types.
    ///
    /// For a single-driver policy, it must enforce all additional registered
    /// dialect constraints that cannot be checked by local structure alone.
    ///
    /// # Errors
    ///
    /// Return a structured error for an unknown resolver, an incompatible
    /// resolver contract, unsupported semantics, or invalid policy use.
    fn validate_driver_policy(
        &self,
        connection: &HdlirConnection,
        drivers: &[ConnectivityEndpointInfo<Self::TypeKey>],
        sinks: &[ConnectivityEndpointInfo<Self::TypeKey>],
    ) -> HdlirResult<()>;
}

/// Optional operational work budget for connectivity verification.
///
/// Each connection, endpoint occurrence, driver/sink compatibility check,
/// and sink-index operation consumes work. The context adapter remains
/// responsible for bounding any additional work it performs internally.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ConnectivityVerificationBudget {
    /// Maximum verifier work items, or `None` for no configured item limit.
    pub max_items: Option<u128>,
}

impl ConnectivityVerificationBudget {
    /// Create a verifier budget with an explicit maximum work-item count.
    #[must_use]
    pub const fn new(max_items: u128) -> Self {
        Self {
            max_items: Some(max_items),
        }
    }

    /// Create a budget without a configured item-count limit.
    ///
    /// Memory availability, collection representation limits, and execution
    /// resources still apply.
    #[must_use]
    pub const fn unbounded() -> Self {
        Self { max_items: None }
    }
}

impl Default for ConnectivityVerificationBudget {
    fn default() -> Self {
        Self::unbounded()
    }
}

/// Counts and outcomes produced by a completed connectivity verification.
///
/// A report is returned only after all checks requested by this verifier
/// have completed. Budget exhaustion or a failed check returns an error
/// instead of a partial report that could be mistaken for success.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ConnectivityVerificationReport {
    /// Number of connection records checked.
    pub connections_checked: u128,

    /// Number of endpoint occurrences resolved.
    pub endpoints_checked: u128,

    /// Number of driver/sink type pairs checked.
    pub type_pairs_checked: u128,

    /// Number of distinct sink endpoints indexed.
    pub distinct_sinks_checked: u128,

    /// Total accounted verifier work items.
    pub work_items: u128,
}

/// Verify the logical connectivity collection against its enclosing graph.
///
/// The verifier checks local connection invariants, resolves each endpoint,
/// checks driver/sink roles, verifies every driver-to-sink type relationship,
/// rejects a sink that is driven by multiple separate connection records,
/// and delegates driver-policy/resolver validation to the context.
///
/// A resolved multi-driver net is represented by one connection with multiple
/// drivers and an explicit `Resolved` policy. A sink repeated across different
/// connection records is rejected here; this representation does not silently
/// merge separate records into one resolution group.
///
/// # Errors
///
/// Returns a structured error if an endpoint is missing, an endpoint is used
/// in an illegal role, types are incompatible or indeterminate, a sink is
/// attached to multiple connection records, a policy is invalid, arithmetic
/// overflows, or the operational budget is exhausted.
pub fn verify_connectivity<C: ConnectivityContext>(
    connectivity: &HdlirConnectivity,
    context: &C,
    budget: ConnectivityVerificationBudget,
) -> HdlirResult<ConnectivityVerificationReport> {
    let mut verifier = ConnectivityVerifier {
        context,
        budget,
        report: ConnectivityVerificationReport::default(),
    };

    // Avoid an unaccounted whole-collection preflight pass. Each connection
    // is checked and charged as it is visited below.
    let mut sink_owner: BTreeMap<HdlirEndpoint, HdlirNetId> = BTreeMap::new();

    for connection in connectivity.iter() {
        verifier.consume_work("checking an HDLIR connection")?;
        connection.validate_structure()?;

        verifier.report.connections_checked = verifier
            .report
            .connections_checked
            .checked_add(1)
            .ok_or_else(|| {
                HdlirError::invalid_invariant(
                    "connectivity_verification.counter_range",
                    "connection counter overflowed u128",
                )
            })?;

        let mut drivers = Vec::new();
        for endpoint in connection.drivers().iter().copied() {
            verifier.consume_work("resolving a driver endpoint")?;
            let info = context.resolve_endpoint(endpoint)?;
            verifier.report.endpoints_checked = verifier
                .report
                .endpoints_checked
                .checked_add(1)
                .ok_or_else(|| {
                    HdlirError::invalid_invariant(
                        "connectivity_verification.counter_range",
                        "endpoint counter overflowed u128",
                    )
                })?;

            if !info.can_drive {
                return Err(HdlirError::invalid_invariant(
                    "connectivity.driver_role",
                    format!(
                        "endpoint {endpoint:?} is not permitted to drive \
                         connection {}",
                        connection.id()
                    ),
                ));
            }

            drivers.push(info);
        }

        let mut sinks = Vec::new();
        for endpoint in connection.sinks().iter().copied() {
            verifier.consume_work("resolving a sink endpoint")?;
            let info = context.resolve_endpoint(endpoint)?;
            verifier.report.endpoints_checked = verifier
                .report
                .endpoints_checked
                .checked_add(1)
                .ok_or_else(|| {
                    HdlirError::invalid_invariant(
                        "connectivity_verification.counter_range",
                        "endpoint counter overflowed u128",
                    )
                })?;

            if !info.can_receive {
                return Err(HdlirError::invalid_invariant(
                    "connectivity.sink_role",
                    format!(
                        "endpoint {endpoint:?} is not permitted to receive \
                         from connection {}",
                        connection.id()
                    ),
                ));
            }

            if let Some(previous_connection) =
                sink_owner.insert(endpoint, connection.id())
            {
                return Err(HdlirError::DuplicateEntity {
                    entity_kind: "HDLIR sink connection",
                    identity: format!("{endpoint:?}"),
                    scope: Some(format!(
                        "connections {previous_connection} and {}",
                        connection.id()
                    )),
                });
            }

            verifier.report.distinct_sinks_checked = verifier
                .report
                .distinct_sinks_checked
                .checked_add(1)
                .ok_or_else(|| {
                    HdlirError::invalid_invariant(
                        "connectivity_verification.counter_range",
                        "distinct sink counter overflowed u128",
                    )
                })?;

            sinks.push(info);
        }

        // Check every driver against every sink. This is deliberately
        // exhaustive: a multi-driver resolved net must not hide a mismatch
        // in one of its drivers. Work is charged for every pair.
        for driver in &drivers {
            for sink in &sinks {
                verifier.consume_work("checking driver/sink type compatibility")?;

                let compatibility = context.type_compatibility(
                    &driver.type_key,
                    &sink.type_key,
                )?;

                verifier.report.type_pairs_checked = verifier
                    .report
                    .type_pairs_checked
                    .checked_add(1)
                    .ok_or_else(|| {
                        HdlirError::invalid_invariant(
                            "connectivity_verification.counter_range",
                            "type-pair counter overflowed u128",
                        )
                    })?;

                match compatibility {
                    ConnectivityTypeCompatibility::Compatible => {}
                    ConnectivityTypeCompatibility::Incompatible => {
                        return Err(HdlirError::invalid_invariant(
                            "connectivity.endpoint_types_compatible",
                            format!(
                                "a driver type is incompatible with a sink \
                                 type in connection {}",
                                connection.id()
                            ),
                        ));
                    }
                    ConnectivityTypeCompatibility::Indeterminate => {
                        return Err(HdlirError::incomplete(
                            "HDLIR connectivity verification",
                            format!(
                                "type compatibility is indeterminate for \
                                 connection {}; verification cannot claim \
                                 success",
                                connection.id()
                            ),
                        ));
                    }
                }
            }
        }

        // This validates registered resolver semantics and dialect-specific
        // policy constraints; the verifier does not implement resolvers.
        context.validate_driver_policy(connection, &drivers, &sinks)?;
    }

    Ok(verifier.report)
}

struct ConnectivityVerifier<'a, C: ConnectivityContext> {
    context: &'a C,
    budget: ConnectivityVerificationBudget,
    report: ConnectivityVerificationReport,
}

impl<C: ConnectivityContext> ConnectivityVerifier<'_, C> {
    fn consume_work(&mut self, operation: &'static str) -> HdlirResult<()> {
        let next = self.report.work_items.checked_add(1).ok_or_else(|| {
            HdlirError::invalid_invariant(
                "connectivity_verification.counter_range",
                "work counter overflowed u128",
            )
        })?;

        if let Some(limit) = self.budget.max_items {
            if next > limit {
                return Err(HdlirError::budget_exceeded(
                    HdlirBudgetKind::Verification,
                    Some(limit),
                    Some(next),
                    operation,
                ));
            }
        }

        self.report.work_items = next;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ConnectivityVerificationBudget,
        ConnectivityVerificationReport,
    };

    #[test]
    fn unbounded_budget_does_not_set_a_semantic_ceiling() {
        assert_eq!(
            ConnectivityVerificationBudget::unbounded().max_items,
            None
        );
    }

    #[test]
    fn explicit_budget_is_operational_configuration() {
        assert_eq!(
            ConnectivityVerificationBudget::new(17).max_items,
            Some(17)
        );
    }

    #[test]
    fn report_defaults_to_zero_counts() {
        assert_eq!(
            ConnectivityVerificationReport::default(),
            ConnectivityVerificationReport {
                connections_checked: 0,
                endpoints_checked: 0,
                type_pairs_checked: 0,
                distinct_sinks_checked: 0,
                work_items: 0,
            }
        );
    }
}

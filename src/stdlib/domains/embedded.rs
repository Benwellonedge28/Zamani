
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Standard Library — Embedded Computing Domain
//!
//! Path: src/stdlib/domains/embedded.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ============================================================================
//! PURPOSE
//! ============================================================================
//!
//! Provides the stable standard-library integration boundary for embedded
//! and resource-constrained computing in Zamani.
//!
//! Embedded computation includes:
//!
//! - Microcontrollers.
//! - Microprocessors.
//! - Embedded CPUs.
//! - Single-board computers.
//! - System-on-chip platforms.
//! - Real-time computing environments.
//! - Sensor-based systems.
//! - Actuator-based systems.
//! - Embedded networking.
//! - Industrial control systems.
//! - Edge computing.
//! - Distributed embedded systems.
//! - Heterogeneous embedded systems.
//! - FPGA-assisted embedded systems.
//! - ASIC-based embedded systems.
//! - Quantum-classical embedded control.
//! - Future embedded computational architectures.
//!
//! This module provides portable descriptions and integration contracts.
//!
//! It does not implement a competing hardware abstraction layer,
//! device driver framework, operating system, scheduler, compiler,
//! firmware deployment system or embedded runtime.
//!
//! ============================================================================
//! CANONICAL AUTHORITIES
//! ============================================================================
//!
//! Source-level hardware declarations:
//!     grammar/hardware/
//!
//! Device declarations:
//!     grammar/hardware/devices.g4
//!
//! CPU declarations:
//!     grammar/hardware/cpu.g4
//!
//! Power intent:
//!     grammar/hardware/power.g4
//!
//! Resource declarations:
//!     grammar/resources/resource.g4
//!
//! Hardware domain specifications:
//!     grammar/specification/domains.md
//!
//! Frontend:
//!     crate::frontend
//!
//! Semantic authority:
//!     crate::frontend::semantic
//!
//! Hardware-description authority:
//!     crate::hdl
//!
//! Classical computation:
//!     crate::classical
//!
//! Quantum computation:
//!     crate::quantum
//!
//! Hybrid computation:
//!     crate::hybrid
//!
//! Distributed computation:
//!     crate::distributed
//!
//! Universal IR:
//!     crate::zuir
//!
//! Compiler:
//!     crate::compiler
//!
//! Runtime:
//!     crate::runtime
//!
//! Generic standard-library devices:
//!     crate::stdlib::io::devices
//!
//! Generic provider contracts:
//!     crate::stdlib::provider
//!
//! Capability contracts:
//!     crate::stdlib::capability
//!
//! ============================================================================
//! OWNERSHIP
//! ============================================================================
//!
//! THIS MODULE OWNS:
//!
//! - The embedded standard-library namespace.
//! - Embedded application descriptions.
//! - Embedded-domain resource-intent descriptions.
//! - Embedded-domain capability requirements.
//! - Portable embedded timing intent.
//! - Portable embedded power intent.
//! - Embedded API identity.
//! - Explicit integration with generic device APIs.
//! - Embedded facade validation.
//!
//! THIS MODULE DOES NOT OWN:
//!
//! - Hardware grammar.
//! - AST.
//! - Semantic analysis.
//! - Type checking.
//! - Effect analysis.
//! - Capability discovery.
//! - Physical resource allocation.
//! - Device enumeration.
//! - Hardware drivers.
//! - Memory allocation.
//! - Interrupt controllers.
//! - Physical memory mapping.
//! - Register allocation.
//! - Scheduling.
//! - Routing.
//! - Clock calibration.
//! - Voltage regulation.
//! - Power management implementation.
//! - Firmware compilation.
//! - Firmware flashing.
//! - Bootloaders.
//! - Operating systems.
//! - Real-time operating systems.
//! - Hardware-description IR.
//! - Quantum IR.
//! - Universal IR.
//! - Backend code generation.
//! - Physical execution.
//!
//! These responsibilities remain with their canonical authorities.
//!
//! ============================================================================
//! POCO-REAF
//! ============================================================================
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! Embedded applications must not depend unnecessarily on:
//!
//! - A particular microcontroller.
//! - A particular CPU architecture.
//! - A particular processor vendor.
//! - A particular board.
//! - A fixed memory capacity.
//! - A fixed register width.
//! - A fixed peripheral count.
//! - A fixed interrupt count.
//! - A fixed device count.
//! - A fixed processor count.
//! - A fixed clock frequency.
//! - A fixed storage capacity.
//! - A fixed network topology.
//! - A fixed operating system.
//! - A fixed hardware vendor.
//!
//! Actual execution depends on:
//!
//! - Program semantics.
//! - Available hardware.
//! - Provider capabilities.
//! - Resource requirements.
//! - Explicit constraints.
//! - Security policies.
//! - Runtime support.
//! - Compilation feasibility.
//!
//! No universal resource ceiling is introduced.
//!
//! Literal infinite physical computation is not promised.
//!
//! ============================================================================
//! ARCHITECTURAL PIPELINE
//! ============================================================================
//!
//! Zamani source
//!      |
//!      v
//! Canonical grammar
//!      |
//!      v
//! Frontend AST
//!      |
//!      v
//! Semantic validation
//!      |
//!      v
//! Embedded application intent
//!      |
//!      v
//! Classical IR / HDLIR / ZUIR
//!      |
//!      v
//! Compiler and target planning
//!      |
//!      v
//! Feasibility and resource resolution
//!      |
//!      v
//! Embedded runtime / HAL / providers
//!      |
//!      v
//! Physical execution
//!
//! This module occupies the standard-library integration boundary.
//!
//! ============================================================================

use std::collections::{BTreeMap, BTreeSet};

use std::fmt;

use crate::stdlib::capability::CapabilityId;

use crate::stdlib::error::{
    Error,
    ErrorKind,
    Result,
};

// ============================================================================
// DOMAIN IDENTITY
// ============================================================================

/// Canonical embedded standard-library namespace.
pub const EMBEDDED_DOMAIN_NAME: &str = "zamani.stdlib.embedded";

/// Embedded standard-library facade API version.
///
/// This is independent of the language, grammar, compiler,
//! runtime and hardware-description versions.
pub const EMBEDDED_API_VERSION: u16 = 1;

/// Embedded provider capability namespace.
pub const EMBEDDED_CAPABILITY_NAMESPACE: &str = "embedded";

/// Standard-library component identity.
pub const COMPONENT: &str = "stdlib.domains.embedded";

// ============================================================================
// ERROR CONSTRUCTION
// ============================================================================

fn embedded_error(
    kind: ErrorKind,
    message: impl Into<String>,
) -> Error {
    Error::new(kind, message)
        .in_component(COMPONENT)
}

/// Validates descriptive identifiers.
///
/// Identifiers are open-world strings.
///
/// No arbitrary identifier-length limit or namespace-depth limit
/// is introduced.
fn validate_identifier(
    value: &str,
    field: &str,
) -> Result<()> {
    if value.is_empty()
        || value.trim() != value
        || value.chars().any(|character| {
            character.is_control()
                || character.is_whitespace()
        })
    {
        return Err(embedded_error(
            ErrorKind::InvalidInput,
            format!("{field} must be non-empty and contain no whitespace or control characters"),
        ));
    }

    Ok(())
}

/// Validates descriptive text without imposing an arbitrary length limit.
fn validate_description(
    value: &str,
    field: &str,
) -> Result<()> {
    if value.trim().is_empty()
        || value.chars().any(char::is_control)
    {
        return Err(embedded_error(
            ErrorKind::InvalidInput,
            format!("{field} must be non-empty and contain no control characters"),
        ));
    }

    Ok(())
}

// ============================================================================
// EMBEDDED IDENTITIES
// ============================================================================

/// An extensible identity for an embedded application.
///
/// This is a logical application identity, not a physical device identity.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct EmbeddedApplicationId(String);

impl EmbeddedApplicationId {
    /// Creates an embedded application identity.
    ///
    /// # Errors
    ///
    /// Returns an error for an invalid identity.
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self> {
        let value = value.into();

        validate_identifier(
            &value,
            "embedded application identity",
        )?;

        Ok(Self(value))
    }

    /// Returns the application identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for EmbeddedApplicationId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// SYMBOLIC EXPRESSIONS
// ============================================================================

/// A validated, portable symbolic expression.
///
/// The expression is preserved as source-independent descriptive text.
///
/// This module deliberately does not parse or reinterpret Zamani
/// expressions.
///
/// Expression parsing and semantic evaluation remain frontend and
/// semantic-layer responsibilities.
///
/// Examples:
///
/// - `available_memory`
/// - `required_memory`
/// - `workload_size`
/// - `available_cores`
/// - `required_clock_frequency`
/// - `required_energy_budget`
/// - `available_storage`
///
/// These examples do not impose hardware limits.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct EmbeddedExpression(String);

impl EmbeddedExpression {
    /// Creates a symbolic embedded expression.
    ///
    /// # Errors
    ///
    /// Returns an error if the expression is empty or contains
    /// control characters.
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self> {
        let value = value.into();

        if value.trim().is_empty()
            || value.chars().any(char::is_control)
        {
            return Err(embedded_error(
                ErrorKind::InvalidInput,
                "embedded expression must be non-empty and contain no control characters",
            ));
        }

        Ok(Self(value))
    }

    /// Returns the original expression.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// ============================================================================
// RESOURCE INTENT
// ============================================================================

/// Describes a logical embedded resource requirement.
///
/// A resource intent is not a physical allocation.
///
/// Resource kinds are open-world identities.
///
/// Expressions remain symbolic until interpreted by the canonical
/// semantic, compiler and resource-planning authorities.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EmbeddedResourceIntent {
    kind: String,
    expression: EmbeddedExpression,
}

impl EmbeddedResourceIntent {
    /// Creates a resource intent.
    ///
    /// # Errors
    ///
    /// Returns an error for invalid resource identities.
    pub fn new(
        kind: impl Into<String>,
        expression: EmbeddedExpression,
    ) -> Result<Self> {
        let kind = kind.into();

        validate_identifier(
            &kind,
            "embedded resource kind",
        )?;

        Ok(Self {
            kind,
            expression,
        })
    }

    /// Returns the resource kind.
    #[must_use]
    pub fn kind(&self) -> &str {
        &self.kind
    }

    /// Returns the symbolic resource expression.
    #[must_use]
    pub fn expression(&self) -> &EmbeddedExpression {
        &self.expression
    }
}

// ============================================================================
// REQUIREMENT / PREFERENCE / CONSTRAINT / HINT
// ============================================================================

/// Classification of an embedded execution-intent expression.
///
/// These categories are deliberately distinct.
///
/// This is a standard-library descriptive classification, not
/// a replacement for the canonical semantic requirement model.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum EmbeddedIntentKind {
    /// A mandatory requirement.
    Requirement,

    /// A non-mandatory preference.
    Preference,

    /// A restriction that must be respected.
    Constraint,

    /// Non-binding implementation guidance.
    Hint,
}

/// An embedded execution-intent expression.
///
/// The classification is preserved without evaluating its meaning.
///
/// Semantic validation remains the responsibility of the frontend.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EmbeddedIntent {
    kind: EmbeddedIntentKind,
    expression: EmbeddedExpression,
}

impl EmbeddedIntent {
    /// Creates an embedded intent.
    #[must_use]
    pub fn new(
        kind: EmbeddedIntentKind,
        expression: EmbeddedExpression,
    ) -> Self {
        Self {
            kind,
            expression,
        }
    }

    /// Returns the intent classification.
    #[must_use]
    pub const fn kind(&self) -> EmbeddedIntentKind {
        self.kind
    }

    /// Returns the preserved expression.
    #[must_use]
    pub fn expression(&self) -> &EmbeddedExpression {
        &self.expression
    }
}

// ============================================================================
// TIMING INTENT
// ============================================================================

/// Portable embedded timing intent.
///
/// These are symbolic program requirements, not measured hardware
/// clock values or scheduling guarantees.
///
/// The compiler and runtime determine whether a requested timing
/// contract can be realized.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EmbeddedTimingIntent {
    /// Optional symbolic latency requirement.
    pub latency: Option<EmbeddedExpression>,

    /// Optional symbolic execution-period requirement.
    pub period: Option<EmbeddedExpression>,

    /// Optional symbolic deadline requirement.
    pub deadline: Option<EmbeddedExpression>,

    /// Optional symbolic jitter constraint.
    pub jitter: Option<EmbeddedExpression>,

    /// Optional symbolic clock requirement.
    pub clock: Option<EmbeddedExpression>,
}

impl EmbeddedTimingIntent {
    /// Creates an empty timing description.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            latency: None,
            period: None,
            deadline: None,
            jitter: None,
            clock: None,
        }
    }

    /// Returns whether no timing properties have been supplied.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.latency.is_none()
            && self.period.is_none()
            && self.deadline.is_none()
            && self.jitter.is_none()
            && self.clock.is_none()
    }
}

impl Default for EmbeddedTimingIntent {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// POWER INTENT
// ============================================================================

/// Portable embedded power intent.
///
/// These values are symbolic expressions.
///
/// This type does not perform power measurement, voltage regulation,
/// energy accounting, battery management or dynamic voltage/frequency
/// scaling.
///
/// Those operations belong to hardware and runtime providers.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EmbeddedPowerIntent {
    /// Optional energy requirement.
    pub energy: Option<EmbeddedExpression>,

    /// Optional power requirement.
    pub power: Option<EmbeddedExpression>,

    /// Optional battery or energy-storage requirement.
    pub storage: Option<EmbeddedExpression>,

    /// Optional power constraint.
    pub constraint: Option<EmbeddedExpression>,

    /// Optional preferred power characteristic.
    pub preference: Option<EmbeddedExpression>,
}

impl EmbeddedPowerIntent {
    /// Creates an empty power description.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            energy: None,
            power: None,
            storage: None,
            constraint: None,
            preference: None,
        }
    }

    /// Returns whether no power properties have been supplied.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.energy.is_none()
            && self.power.is_none()
            && self.storage.is_none()
            && self.constraint.is_none()
            && self.preference.is_none()
    }
}

impl Default for EmbeddedPowerIntent {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// EMBEDDED APPLICATION DESCRIPTION
// ============================================================================

/// Portable description of an embedded Zamani application.
///
/// This structure expresses logical application intent.
///
/// It does not identify a specific microcontroller, board, processor,
/// physical address, peripheral or vendor.
///
/// No hardware resources are allocated by constructing this value.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EmbeddedApplication {
    id: EmbeddedApplicationId,
    description: String,

    /// Open-world capability requirements.
    capabilities: BTreeSet<CapabilityId>,

    /// Symbolic embedded resource requirements.
    resources: Vec<EmbeddedResourceIntent>,

    /// Separately classified execution intents.
    intents: Vec<EmbeddedIntent>,

    /// Portable timing requirements.
    timing: EmbeddedTimingIntent,

    /// Portable power requirements.
    power: EmbeddedPowerIntent,

    /// Extensible descriptive metadata.
    ///
    /// Secrets and credentials must never be stored here.
    properties: BTreeMap<String, String>,
}

impl EmbeddedApplication {
    /// Creates a new embedded application description.
    ///
    /// # Errors
    ///
    /// Returns an error if the application identity or description
    /// is invalid.
    pub fn new(
        id: EmbeddedApplicationId,
        description: impl Into<String>,
    ) -> Result<Self> {
        let description = description.into();

        validate_description(
            &description,
            "embedded application description",
        )?;

        Ok(Self {
            id,
            description,
            capabilities: BTreeSet::new(),
            resources: Vec::new(),
            intents: Vec::new(),
            timing: EmbeddedTimingIntent::new(),
            power: EmbeddedPowerIntent::new(),
            properties: BTreeMap::new(),
        })
    }

    /// Returns the application identity.
    #[must_use]
    pub fn id(&self) -> &EmbeddedApplicationId {
        &self.id
    }

    /// Returns the application description.
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Returns declared capability requirements.
    #[must_use]
    pub fn capabilities(&self) -> &BTreeSet<CapabilityId> {
        &self.capabilities
    }

    /// Returns resource requirements.
    #[must_use]
    pub fn resources(&self) -> &[EmbeddedResourceIntent] {
        &self.resources
    }

    /// Returns execution intents.
    #[must_use]
    pub fn intents(&self) -> &[EmbeddedIntent] {
        &self.intents
    }

    /// Returns timing intent.
    #[must_use]
    pub fn timing(&self) -> &EmbeddedTimingIntent {
        &self.timing
    }

    /// Returns power intent.
    #[must_use]
    pub fn power(&self) -> &EmbeddedPowerIntent {
        &self.power
    }

    /// Returns descriptive metadata.
    #[must_use]
    pub fn properties(&self) -> &BTreeMap<String, String> {
        &self.properties
    }

    /// Adds a required capability.
    ///
    /// Capability identities are open-world.
    ///
    /// # Errors
    ///
    /// Returns an error if insertion would violate the existing
    /// capability identity contract.
    pub fn require_capability(
        &mut self,
        capability: CapabilityId,
    ) {
        self.capabilities.insert(capability);
    }

    /// Adds a symbolic resource requirement.
    ///
    /// # Errors
    ///
    /// Returns an error if the resource description is invalid.
    pub fn add_resource(
        &mut self,
        resource: EmbeddedResourceIntent,
    ) -> Result<()> {
        self.resources.push(resource);
        Ok(())
    }

    /// Adds an execution intent while preserving its classification.
    pub fn add_intent(
        &mut self,
        intent: EmbeddedIntent,
    ) {
        self.intents.push(intent);
    }

    /// Replaces the timing intent.
    pub fn set_timing(
        &mut self,
        timing: EmbeddedTimingIntent,
    ) {
        self.timing = timing;
    }

    /// Replaces the power intent.
    pub fn set_power(
        &mut self,
        power: EmbeddedPowerIntent,
    ) {
        self.power = power;
    }

    /// Adds non-secret descriptive metadata.
    ///
    /// # Errors
    ///
    /// Returns an error for invalid property names or values.
    pub fn insert_property(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Option<String>> {
        let key = key.into();
        let value = value.into();

        validate_identifier(
            &key,
            "embedded property name",
        )?;

        if value.chars().any(char::is_control) {
            return Err(embedded_error(
                ErrorKind::InvalidInput,
                "embedded property values must not contain control characters",
            ));
        }

        Ok(self.properties.insert(key, value))
    }

    /// Validates the structural integrity of the application description.
    ///
    /// This is not semantic validation or target feasibility assessment.
    ///
    /// # Errors
    ///
    /// Returns a structured error if an invariant is violated.
    pub fn validate(&self) -> Result<()> {
        validate_identifier(
            self.id.as_str(),
            "embedded application identity",
        )?;

        validate_description(
            &self.description,
            "embedded application description",
        )?;

        for (key, value) in &self.properties {
            validate_identifier(
                key,
                "embedded property name",
            )?;

            if value.chars().any(char::is_control) {
                return Err(embedded_error(
                    ErrorKind::InvalidInput,
                    "embedded property contains control characters",
                ));
            }
        }

        for resource in &self.resources {
            validate_identifier(
                resource.kind(),
                "embedded resource kind",
            )?;
        }

        Ok(())
    }
}

// ============================================================================
// CANONICAL DEVICE INTEGRATION
// ============================================================================

/// Reuses the existing generic device identity.
///
/// No second embedded device identity system is created.
pub use crate::stdlib::io::devices::DeviceId;

/// Reuses the existing generic device-kind identity.
pub use crate::stdlib::io::devices::DeviceKindId;

/// Reuses the existing generic device capability identity.
pub use crate::stdlib::io::devices::DeviceCapabilityId;

/// Reuses the canonical device access contract.
pub use crate::stdlib::io::devices::DeviceAccess;

/// Reuses the canonical device descriptor.
pub use crate::stdlib::io::devices::DeviceDescriptor;

/// Reuses the canonical device transport.
pub use crate::stdlib::io::devices::DeviceTransport;

/// Reuses the canonical device provider interface.
pub use crate::stdlib::io::devices::DeviceProvider;

/// Reuses the canonical cancellation-aware device connection.
pub use crate::stdlib::io::devices::DeviceConnection;

// ============================================================================
// EMBEDDED DEVICE CAPABILITY IDENTITIES
// ============================================================================

/// Returns the canonical embedded capability identity.
///
/// The returned identity can be used with the existing capability
/// subsystem.
///
/// No closed enumeration of embedded capabilities is introduced.
///
/// Examples:
///
/// - `embedded.gpio`
/// - `embedded.pwm`
/// - `embedded.interrupts`
/// - `embedded.real_time`
/// - `embedded.low_power`
/// - `embedded.sensor_io`
/// - `embedded.nonvolatile_storage`
///
/// # Errors
///
/// Returns an error if the supplied capability name is invalid.
pub fn embedded_capability(
    name: &str,
) -> Result<CapabilityId> {
    validate_identifier(
        name,
        "embedded capability name",
    )?;

    CapabilityId::new(
        format!("{EMBEDDED_CAPABILITY_NAMESPACE}.{name}"),
    )
}

// ============================================================================
// DOMAIN VALIDATION
// ============================================================================

/// Validates an embedded application before passing it to
/// semantic analysis or compiler integration.
///
/// This function does not:
///
/// - Discover devices.
/// - Reserve memory.
/// - Evaluate hardware availability.
/// - Allocate resources.
/// - Check physical timing.
/// - Check actual power consumption.
/// - Execute firmware.
/// - Flash hardware.
///
/// Those operations belong to their canonical authorities.
pub fn validate_application(
    application: &EmbeddedApplication,
) -> Result<()> {
    application.validate()
}

// ============================================================================
// PORTABILITY CONTRACT
// ============================================================================

/// Describes the portability guarantees of this standard-library facade.
///
/// These values are descriptive contract metadata, not hardware
/// capability evidence.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EmbeddedPortabilityContract {
    /// Whether hardware vendor identities are required.
    pub requires_vendor_identity: bool,

    /// Whether physical device identity is required.
    pub requires_physical_device_identity: bool,

    /// Whether arbitrary resource ceilings are introduced.
    pub introduces_universal_resource_ceilings: bool,

    /// Whether physical execution is performed by this facade.
    pub performs_physical_execution: bool,

    /// Whether hardware discovery is performed by this facade.
    pub performs_hardware_discovery: bool,
}

/// Returns the stable embedded portability contract.
#[must_use]
pub const fn portability_contract() -> EmbeddedPortabilityContract {
    EmbeddedPortabilityContract {
        requires_vendor_identity: false,
        requires_physical_device_identity: false,
        introduces_universal_resource_ceilings: false,
        performs_physical_execution: false,
        performs_hardware_discovery: false,
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn application() -> EmbeddedApplication {
        EmbeddedApplication::new(
            EmbeddedApplicationId::new(
                "example.sensor.application",
            )
            .expect("valid application identity"),
            "Portable embedded sensor application",
        )
        .expect("valid application")
    }

    #[test]
    fn domain_identity_is_stable() {
        assert_eq!(
            EMBEDDED_DOMAIN_NAME,
            "zamani.stdlib.embedded",
        );

        assert_eq!(
            EMBEDDED_API_VERSION,
            1,
        );
    }

    #[test]
    fn application_identity_is_validated() {
        assert!(
            EmbeddedApplicationId::new(
                "example.sensor",
            )
            .is_ok()
        );

        assert!(
            EmbeddedApplicationId::new(
                "",
            )
            .is_err()
        );

        assert!(
            EmbeddedApplicationId::new(
                "invalid application",
            )
            .is_err()
        );
    }

    #[test]
    fn symbolic_expressions_are_preserved() {
        let expression = EmbeddedExpression::new(
            "available_memory",
        )
        .expect("valid expression");

        assert_eq!(
            expression.as_str(),
            "available_memory",
        );
    }

    #[test]
    fn symbolic_expressions_reject_empty_values() {
        assert!(
            EmbeddedExpression::new("")
                .is_err()
        );
    }

    #[test]
    fn resource_intents_preserve_symbolic_quantities() {
        let resource = EmbeddedResourceIntent::new(
            "memory",
            EmbeddedExpression::new(
                "required_memory",
            )
            .expect("valid expression"),
        )
        .expect("valid resource");

        assert_eq!(
            resource.kind(),
            "memory",
        );

        assert_eq!(
            resource.expression().as_str(),
            "required_memory",
        );
    }

    #[test]
    fn intent_categories_remain_distinct() {
        let expression = EmbeddedExpression::new(
            "required_clock_frequency",
        )
        .expect("valid expression");

        let requirement = EmbeddedIntent::new(
            EmbeddedIntentKind::Requirement,
            expression.clone(),
        );

        let preference = EmbeddedIntent::new(
            EmbeddedIntentKind::Preference,
            expression.clone(),
        );

        let constraint = EmbeddedIntent::new(
            EmbeddedIntentKind::Constraint,
            expression.clone(),
        );

        let hint = EmbeddedIntent::new(
            EmbeddedIntentKind::Hint,
            expression,
        );

        assert_eq!(
            requirement.kind(),
            EmbeddedIntentKind::Requirement,
        );

        assert_eq!(
            preference.kind(),
            EmbeddedIntentKind::Preference,
        );

        assert_eq!(
            constraint.kind(),
            EmbeddedIntentKind::Constraint,
        );

        assert_eq!(
            hint.kind(),
            EmbeddedIntentKind::Hint,
        );
    }

    #[test]
    fn timing_intent_defaults_to_empty() {
        assert!(
            EmbeddedTimingIntent::new().is_empty()
        );
    }

    #[test]
    fn power_intent_defaults_to_empty() {
        assert!(
            EmbeddedPowerIntent::new().is_empty()
        );
    }

    #[test]
    fn application_accepts_open_world_capabilities() {
        let mut app = application();

        app.require_capability(
            embedded_capability("gpio")
                .expect("valid capability"),
        );

        app.require_capability(
            embedded_capability("future.peripheral")
                .expect("valid capability"),
        );

        assert_eq!(
            app.capabilities().len(),
            2,
        );
    }

    #[test]
    fn application_accepts_resource_intents() {
        let mut app = application();

        app.add_resource(
            EmbeddedResourceIntent::new(
                "memory",
                EmbeddedExpression::new(
                    "available_memory",
                )
                .expect("valid expression"),
            )
            .expect("valid resource"),
        )
        .expect("resource insertion succeeds");

        assert_eq!(
            app.resources().len(),
            1,
        );

        assert!(app.validate().is_ok());
    }

    #[test]
    fn application_accepts_extensible_metadata() {
        let mut app = application();

        app.insert_property(
            "application.category",
            "embedded.sensor",
        )
        .expect("valid metadata");

        assert_eq!(
            app.properties()
                .get("application.category")
                .map(String::as_str),
            Some("embedded.sensor"),
        );
    }

    #[test]
    fn invalid_metadata_is_rejected() {
        let mut app = application();

        assert!(
            app.insert_property(
                "invalid property",
                "value",
            )
            .is_err()
        );
    }

    #[test]
    fn portability_does_not_assume_physical_hardware() {
        let contract = portability_contract();

        assert!(!contract.requires_vendor_identity);

        assert!(!contract.requires_physical_device_identity);

        assert!(!contract.introduces_universal_resource_ceilings);

        assert!(!contract.performs_physical_execution);

        assert!(!contract.performs_hardware_discovery);
    }
}

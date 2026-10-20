
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # HDLIR Operation Interface Contracts
//!
//! Path: `src/hdl/hdlir/schema/interfaces.rs`
//! Rust edition: 2021
//! Minimum supported Rust version: 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Define reusable, declarative requirements that an operation contract must
//! satisfy to implement a registered HDLIR interface.
//!
//! ## Authority
//!
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4`
//! - `grammar/specification/extensibility.md`
//! - `grammar/validation/semantic-boundaries.md`
//! - `src/hdl/api.rs`
//! - `src/hdl/ids.rs`
//! - `src/hdl/hdlir/errors.rs`
//! - `src/hdl/hdlir/version.rs`
//! - `src/hdl/hdlir/schema/registry.rs`
//! - `src/hdl/hdlir/schema/operation.rs`
//! - `src/zuir/ZUIR-SPEC.md`
//!
//! The grammar owns syntax, the frontend owns source-level meaning, ZUIR owns
//! the universal computational representation, and HDLIR owns hardware-domain
//! representation contracts. Physical realization remains downstream.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Declarative interface rules.
//! - Validation of an operation contract against those rules.
//! - Deterministic rule ordering and explicit contract failures.
//!
//! The canonical `InterfaceDefinition` and `QualifiedName` remain owned by
//! `schema::registry`. `OperationContract` and `OperationEffect` remain owned
//! by `schema::operation`.
//!
//! This module does not own operation registration, operation execution,
//! verifier implementation, concrete IR instances, type inference, source
//! parsing, target discovery, placement, routing, or hardware execution.
//!
//! ## Scalability
//!
//! This module declares no universal limit on the number of interfaces,
//! rules, operations, attributes, regions, or results. Its collections grow
//! according to available resources. Callers remain responsible for enforcing
//! explicit operational budgets when processing untrusted or very large
//! schemas.
//!
//! ## Determinism
//!
//! Rules are stored in `BTreeSet` order. No hash iteration order, pointer
//! identity, global mutable state, or executable callback participates in
//! interface validation.
//!
//! ## Important distinction
//!
//! Passing these checks establishes only that the operation's declarative
//! contract satisfies the declared interface rules. It does not prove that
//! the operation is implemented, that a required verifier is installed, that
//! a target supports the operation, or that a physical implementation exists.

use std::collections::BTreeSet;

use crate::hdl::hdlir::errors::{HdlirError, HdlirResult};

use super::operation::{OperationContract, OperationEffect};
use super::registry::{InterfaceDefinition, QualifiedName};

/// A declarative requirement imposed by an operation interface.
///
/// Rules are evaluated against an [`OperationContract`]. They do not execute
/// the operation and do not replace the verifier that checks concrete IR
/// instances.
///
/// String labels use the same canonical local-name convention as operation
/// schema labels: lowercase ASCII letters, digits, `_` and `-`, beginning
/// with a lowercase letter or `_`.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum InterfaceRule {
    /// The operation must declare the specified effect.
    RequiresEffect(OperationEffect),

    /// The operation must not declare the specified effect.
    ForbidsEffect(OperationEffect),

    /// The operation must declare an attribute with this local name.
    RequiresAttribute(String),

    /// The operation must not declare an attribute with this local name.
    ForbidsAttribute(String),

    /// The operation must declare a region with this local label.
    RequiresRegion(String),

    /// The operation must not declare a region with this local label.
    ForbidsRegion(String),

    /// The operation must have a result arity whose minimum is at least one.
    ///
    /// This checks the operation schema's declared arity, not the number of
    /// results of a concrete operation instance.
    RequiresAtLeastOneResult,

    /// The operation must declare exactly zero results.
    RequiresNoResults,

    /// The operation must declare a fixed operand count equal to `count`.
    RequiresFixedOperandCount(u64),

    /// The operation must declare a fixed result count equal to `count`.
    RequiresFixedResultCount(u64),

    /// The operation contract must declare the named verifier as required.
    ///
    /// This does not establish that the verifier is implemented or available.
    RequiresVerifier(QualifiedName),
}

/// A complete declarative interface contract.
///
/// The contained [`InterfaceDefinition`] remains the canonical identity,
/// namespace, version, stability classification, and description registered
/// by `schema::registry`.
///
/// The rules express additional constraints that consumers can evaluate
/// without depending on executable callbacks.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InterfaceContract {
    definition: InterfaceDefinition,
    rules: BTreeSet<InterfaceRule>,
}

impl InterfaceContract {
    /// Construct an interface contract and validate its declarative rules.
    ///
    /// Duplicate rules are normalized by the `BTreeSet`. This is intentional:
    /// repeating the same requirement does not change interface semantics.
    ///
    /// # Errors
    ///
    /// Returns an error if a rule contains an invalid local attribute or
    /// region label.
    pub fn new(
        definition: InterfaceDefinition,
        rules: impl IntoIterator<Item = InterfaceRule>,
    ) -> HdlirResult<Self> {
        let mut normalized = BTreeSet::new();

        for rule in rules {
            match &rule {
                InterfaceRule::RequiresAttribute(name)
                | InterfaceRule::ForbidsAttribute(name) => {
                    validate_local_name("interface.attribute", name)?;
                }
                InterfaceRule::RequiresRegion(name)
                | InterfaceRule::ForbidsRegion(name) => {
                    validate_local_name("interface.region", name)?;
                }
                InterfaceRule::RequiresVerifier(_) => {}
                InterfaceRule::RequiresEffect(_)
                | InterfaceRule::ForbidsEffect(_)
                | InterfaceRule::RequiresAtLeastOneResult
                | InterfaceRule::RequiresNoResults
                | InterfaceRule::RequiresFixedOperandCount(_)
                | InterfaceRule::RequiresFixedResultCount(_) => {}
            }

            normalized.insert(rule);
        }

        validate_rule_conflicts(&normalized)?;

        Ok(Self {
            definition,
            rules: normalized,
        })
    }

    /// Return the canonical registered interface definition.
    #[must_use]
    pub fn definition(&self) -> &InterfaceDefinition {
        &self.definition
    }

    /// Return the interface's qualified name.
    #[must_use]
    pub fn name(&self) -> &QualifiedName {
        self.definition.name()
    }

    /// Return the interface rules in deterministic order.
    #[must_use]
    pub fn rules(&self) -> &BTreeSet<InterfaceRule> {
        &self.rules
    }

    /// Validate an operation contract against this interface.
    ///
    /// The operation must explicitly declare this interface in its canonical
    /// `OperationDefinition`. Each rule is then checked against the operation
    /// schema and its detailed contract.
    ///
    /// This function validates schema-level claims only. Concrete operation
    /// instances must still pass the relevant structural and semantic
    /// verification.
    ///
    /// # Errors
    ///
    /// Returns a structured HDLIR error if the operation does not declare this
    /// interface or violates any interface rule.
    pub fn validate_operation(
        &self,
        operation: &OperationContract,
    ) -> HdlirResult<()> {
        let definition = operation.definition();

        if !definition.interfaces().contains(self.name()) {
            return Err(HdlirError::invalid_input(
                "interface.implementation",
                format!(
                    "operation `{}` does not declare interface `{}`",
                    definition.name(),
                    self.name()
                ),
            ));
        }

        for rule in &self.rules {
            self.validate_rule(operation, rule)?;
        }

        Ok(())
    }

    fn validate_rule(
        &self,
        operation: &OperationContract,
        rule: &InterfaceRule,
    ) -> HdlirResult<()> {
        let definition = operation.definition();

        let violation = match rule {
            InterfaceRule::RequiresEffect(effect) => {
                (!operation.effects().contains(effect))
                    .then(|| format!("requires effect `{effect:?}`"))
            }

            InterfaceRule::ForbidsEffect(effect) => {
                operation
                    .effects()
                    .contains(effect)
                    .then(|| format!("forbids effect `{effect:?}`"))
            }

            InterfaceRule::RequiresAttribute(name) => {
                operation
                    .attribute(name)
                    .is_none()
                    .then(|| format!("requires attribute `{name}`"))
            }

            InterfaceRule::ForbidsAttribute(name) => {
                operation
                    .attribute(name)
                    .is_some()
                    .then(|| format!("forbids attribute `{name}`"))
            }

            InterfaceRule::RequiresRegion(name) => {
                (!operation.regions().iter().any(|region| region.label() == name))
                    .then(|| format!("requires region `{name}`"))
            }

            InterfaceRule::ForbidsRegion(name) => {
                operation
                    .regions()
                    .iter()
                    .any(|region| region.label() == name)
                    .then(|| format!("forbids region `{name}`"))
            }

            InterfaceRule::RequiresAtLeastOneResult => {
                (definition.signature().results().min() == 0)
                    .then(|| "requires at least one result".to_owned())
            }

            InterfaceRule::RequiresNoResults => {
                (definition.signature().results().max() != Some(0))
                    .then(|| "requires an operation with zero results".to_owned())
            }

            InterfaceRule::RequiresFixedOperandCount(count) => {
                let arity = definition.signature().operands();
                (arity.min() != *count || arity.max() != Some(*count))
                    .then(|| {
                        format!(
                            "requires a fixed operand count of {count}, but the declared \
                             range is {}..{}",
                            arity.min(),
                            display_optional_max(arity.max())
                        )
                    })
            }

            InterfaceRule::RequiresFixedResultCount(count) => {
                let arity = definition.signature().results();
                (arity.min() != *count || arity.max() != Some(*count))
                    .then(|| {
                        format!(
                            "requires a fixed result count of {count}, but the declared \
                             range is {}..{}",
                            arity.min(),
                            display_optional_max(arity.max())
                        )
                    })
            }

            InterfaceRule::RequiresVerifier(name) => {
                (!operation
                    .verifiers()
                    .iter()
                    .any(|verifier| verifier.name() == name && verifier.required()))
                .then(|| format!("requires verifier `{name}` to be declared as required"))
            }
        };

        if let Some(reason) = violation {
            return Err(HdlirError::invalid_input(
                "interface.rule",
                format!(
                    "operation `{}` violates interface `{}`: {reason}",
                    definition.name(),
                    self.name()
                ),
            ));
        }

        Ok(())
    }
}

/// Validate a canonical local schema label.
fn validate_local_name(field: &'static str, name: &str) -> HdlirResult<()> {
    let mut bytes = name.bytes();

    let Some(first) = bytes.next() else {
        return Err(HdlirError::invalid_input(
            field,
            "name must not be empty",
        ));
    };

    if !(first.is_ascii_lowercase() || first == b'_')
        || !bytes.all(|byte| {
            byte.is_ascii_lowercase()
                || byte.is_ascii_digit()
                || byte == b'_'
                || byte == b'-'
        })
    {
        return Err(HdlirError::invalid_input(
            field,
            format!(
                "name `{name}` must begin with a lowercase ASCII letter or `_` \
                 and contain only lowercase ASCII letters, digits, `_` or `-`"
            ),
        ));
    }

    Ok(())
}

/// Reject rules whose requirements are logically contradictory.
fn validate_rule_conflicts(rules: &BTreeSet<InterfaceRule>) -> HdlirResult<()> {
    for rule in rules {
        let conflict = match rule {
            InterfaceRule::RequiresEffect(effect) => {
                rules.contains(&InterfaceRule::ForbidsEffect(*effect))
            }

            InterfaceRule::ForbidsEffect(effect) => {
                rules.contains(&InterfaceRule::RequiresEffect(*effect))
            }

            InterfaceRule::RequiresAttribute(name) => {
                rules.contains(&InterfaceRule::ForbidsAttribute(name.clone()))
            }

            InterfaceRule::ForbidsAttribute(name) => {
                rules.contains(&InterfaceRule::RequiresAttribute(name.clone()))
            }

            InterfaceRule::RequiresRegion(name) => {
                rules.contains(&InterfaceRule::ForbidsRegion(name.clone()))
            }

            InterfaceRule::ForbidsRegion(name) => {
                rules.contains(&InterfaceRule::RequiresRegion(name.clone()))
            }

            InterfaceRule::RequiresAtLeastOneResult => {
                rules.contains(&InterfaceRule::RequiresNoResults)
                    || rules.contains(&InterfaceRule::RequiresFixedResultCount(0))
            }

            InterfaceRule::RequiresNoResults => {
                rules.contains(&InterfaceRule::RequiresAtLeastOneResult)
            }

            InterfaceRule::RequiresFixedOperandCount(count) => {
                *count > 0
                    && rules.contains(&InterfaceRule::RequiresFixedOperandCount(0))
            }

            InterfaceRule::RequiresFixedResultCount(count) => {
                (*count > 0
                    && rules.contains(&InterfaceRule::RequiresFixedResultCount(0)))
                    || (*count == 0
                        && rules.contains(&InterfaceRule::RequiresAtLeastOneResult))
            }

            InterfaceRule::RequiresVerifier(_) => false,
        };

        if conflict {
            return Err(HdlirError::invalid_input(
                "interface.rules",
                format!("contradictory interface rule `{rule:?}`"),
            ));
        }
    }

    Ok(())
}

fn display_optional_max(maximum: Option<u64>) -> String {
    maximum.map_or_else(|| "unbounded".to_owned(), |value| value.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::hdl::hdlir::schema::operation::{
        OperationContract, OperationEffect, TypeConstraint, ValueGroupContract,
    };
    use crate::hdl::hdlir::schema::registry::{
        Arity, DefinitionStability, OperationDefinition, OperationSignature,
    };
    use crate::hdl::hdlir::version::HdlirVersion;

    fn qualified(name: &str) -> QualifiedName {
        QualifiedName::parse(name).expect("test qualified name should be valid")
    }

    fn interface_definition() -> InterfaceDefinition {
        InterfaceDefinition::new(
            qualified("core.dialect.pure"),
            qualified("core.dialect"),
            HdlirVersion::new(1, 0, 0),
            DefinitionStability::Core,
            "Operation has no declared observable effects",
        )
        .expect("test interface definition should be valid")
    }

    fn operation_contract() -> OperationContract {
        let definition = OperationDefinition::new(
            qualified("core.dialect.constant"),
            qualified("core.dialect"),
            HdlirVersion::new(1, 0, 0),
            DefinitionStability::Core,
            OperationSignature::new(
                Arity::new(0, Some(0)).expect("valid operand arity"),
                Arity::new(1, Some(1)).expect("valid result arity"),
            ),
            [qualified("core.dialect.pure")].into_iter().collect(),
            "A pure operation that produces one result",
        )
        .expect("test operation definition should be valid");

        let result = ValueGroupContract::new(
            "value",
            Arity::new(1, Some(1)).expect("valid result group arity"),
            TypeConstraint::Any,
        )
        .expect("test result group should be valid");

        OperationContract::new(
            definition,
            Vec::new(),
            vec![result],
            Vec::new(),
            Vec::new(),
            BTreeSet::new(),
            BTreeSet::new(),
        )
        .expect("test operation contract should be valid")
    }

    #[test]
    fn accepts_an_operation_that_satisfies_all_rules() {
        let interface = InterfaceContract::new(
            interface_definition(),
            [
                InterfaceRule::RequiresAtLeastOneResult,
                InterfaceRule::RequiresFixedOperandCount(0),
            ],
        )
        .expect("interface contract should be valid");

        assert!(interface.validate_operation(&operation_contract()).is_ok());
    }

    #[test]
    fn rejects_an_operation_that_does_not_declare_the_interface() {
        let interface = InterfaceContract::new(interface_definition(), [])
            .expect("interface contract should be valid");

        let mut operation = operation_contract();
        let definition = OperationDefinition::new(
            qualified("core.dialect.other"),
            qualified("core.dialect"),
            HdlirVersion::new(1, 0, 0),
            DefinitionStability::Core,
            OperationSignature::new(
                Arity::new(0, Some(0)).expect("valid operand arity"),
                Arity::new(1, Some(1)).expect("valid result arity"),
            ),
            BTreeSet::new(),
            "Operation without the interface",
        )
        .expect("operation definition should be valid");

        operation = OperationContract::new(
            definition,
            Vec::new(),
            vec![
                ValueGroupContract::new(
                    "value",
                    Arity::new(1, Some(1)).expect("valid result arity"),
                    TypeConstraint::Any,
                )
                .expect("valid result group"),
            ],
            Vec::new(),
            Vec::new(),
            BTreeSet::new(),
            BTreeSet::new(),
        )
        .expect("operation contract should be valid");

        assert!(interface.validate_operation(&operation).is_err());
    }

    #[test]
    fn rejects_missing_required_effect() {
        let interface = InterfaceContract::new(
            interface_definition(),
            [InterfaceRule::RequiresEffect(OperationEffect::Read)],
        )
        .expect("interface contract should be valid");

        assert!(interface.validate_operation(&operation_contract()).is_err());
    }

    #[test]
    fn rejects_contradictory_rules() {
        let result = InterfaceContract::new(
            interface_definition(),
            [
                InterfaceRule::RequiresNoResults,
                InterfaceRule::RequiresAtLeastOneResult,
            ],
        );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_invalid_attribute_labels() {
        let result = InterfaceContract::new(
            interface_definition(),
            [InterfaceRule::RequiresAttribute("InvalidName".to_owned())],
        );

        assert!(result.is_err());
    }

    #[test]
    fn duplicate_rules_are_normalized_deterministically() {
        let interface = InterfaceContract::new(
            interface_definition(),
            [
                InterfaceRule::RequiresAtLeastOneResult,
                InterfaceRule::RequiresAtLeastOneResult,
            ],
        )
        .expect("duplicate identical rules are semantically redundant");

        assert_eq!(interface.rules().len(), 1);
    }
}

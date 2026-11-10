
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Compiler — Generic Specialization and Monomorphization
//!
//! File: src/compiler/monomorphizer.rs
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Coordinate deterministic specialization of semantically validated
//! generic declarations and their resolved type arguments.
//!
//! # Architectural ownership
//!
//! This module owns:
//! - Specialization identity construction.
//! - Specialization cache coordination.
//! - Specialization record management.
//! - Configurable specialization budgets.
//! - Compiler-facing orchestration of canonical generic binding.
//!
//! This module does not own:
//! - Generic source syntax.
//! - The canonical AST.
//! - Semantic type resolution.
//! - Type inference or unification.
//! - Trait or interface implementation lookup.
//! - Constraint solving.
//! - Effect semantics.
//! - Canonical ZUIR or domain IR definitions.
//! - IR cloning or IR-specific transformations.
//! - Hardware discovery or allocation.
//! - Backend code generation.
//! - Runtime execution.
//!
//! # Canonical authorities
//!
//! Source generic parameters:
//!     crate::frontend::ast::node::generics::parameter::TypeParameter
//!
//! Source type expressions:
//!     crate::frontend::ast::node::types::type_expr::TypeExpr
//!
//! Semantic generic binding:
//!     crate::frontend::semantic::types::generics
//!
//! Canonical IR:
//!     crate::zuir
//!
//! Domain IR:
//!     crate::classical
//!     crate::quantum::ir
//!     crate::hdl::hdlir
//!     crate::ai
//!
//! # Scalability
//!
//! No universal maximum is imposed on:
//! - Generic parameter count.
//! - Generic argument count.
//! - Number of specializations.
//! - Number of registered specialization records.
//! - Computational domains.
//! - Hardware resources.
//!
//! Optional budgets are operational policies. Exhausting a configured
//! budget returns an explicit error and never silently changes semantics.
//!
//! No finite implementation can process literally infinite input.
//!
//! # Determinism
//!
//! Specialization identity is derived from:
//! - A stable declaration identity supplied by semantic analysis.
//! - Ordered canonical type identities supplied by the semantic type system.
//! - A versioned identity domain.
//!
//! Debug formatting, pointer addresses, randomized hashers, process-local
//! identities and incidental target information are not used.
//!
//! # Integration
//!
//! The semantic frontend resolves generic arguments and validates bounds.
//! This module coordinates specialization identity and cache records.
//! Canonical IR owners materialize verified specializations.
//! The incremental compiler consumes canonical specialization fingerprints.
//!
//! The compiler pipeline must not treat a specialization as complete when
//! its generic constraints remain deferred.

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

use sha2::{Digest, Sha256};

use crate::frontend::ast::node::generics::parameter::TypeParameter;
use crate::frontend::ast::node::generics::where_clause::WhereClause;

use crate::frontend::semantic::types::generics::{
    bind_type_arguments,
    GenericBinding,
    GenericTypeSystem,
    PendingConstraint,
};

// =============================================================================
// VERSIONED IDENTITY DOMAIN
// =============================================================================

/// Stable domain used when deriving specialization identities.
///
/// Changing the canonical identity encoding requires a deliberate schema
/// version change.
pub const MONOMORPHIZATION_IDENTITY_DOMAIN: &str =
    "zamani.compiler.monomorphization.v1";

/// Current specialization identity schema.
pub const MONOMORPHIZATION_SCHEMA_VERSION: u16 = 1;

// =============================================================================
// CONFIGURATION
// =============================================================================

/// Optional operational budgets for specialization.
///
/// `None` means that this module imposes no artificial limit for that
/// resource. Actual memory, address-space, time and process constraints
/// remain unavoidable.
///
/// These are compiler operational policies, not language restrictions.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MonomorphizationConfig {
    /// Maximum number of distinct specializations retained by this engine.
    ///
    /// `None` means no additional engine-imposed count limit.
    pub max_specializations: Option<usize>,

    /// Maximum total byte length of one specialization identity input.
    ///
    /// This protects callers that deliberately configure an identity budget.
    /// `None` means no additional engine-imposed identity-size limit.
    pub max_identity_bytes: Option<usize>,
}

impl MonomorphizationConfig {
    /// Create a configuration without artificial operational limits.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            max_specializations: None,
            max_identity_bytes: None,
        }
    }

    /// Create a configuration with an explicit specialization-count budget.
    #[must_use]
    pub const fn with_max_specializations(
        mut self,
        maximum: usize,
    ) -> Self {
        self.max_specializations = Some(maximum);
        self
    }

    /// Create a configuration with an explicit identity-size budget.
    #[must_use]
    pub const fn with_max_identity_bytes(
        mut self,
        maximum: usize,
    ) -> Self {
        self.max_identity_bytes = Some(maximum);
        self
    }
}

// =============================================================================
// ERRORS
// =============================================================================

/// Errors produced by specialization coordination.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum MonomorphizationError {
    /// A declaration identity is empty or structurally invalid.
    InvalidDeclarationIdentity {
        identity: String,
    },

    /// A semantic type identity supplied by the canonical type system
    /// is empty or invalid.
    InvalidTypeIdentity {
        index: usize,
    },

    /// Identity-size arithmetic overflowed.
    IdentitySizeOverflow,

    /// An explicitly configured identity-size budget was exceeded.
    IdentityBudgetExceeded {
        actual: usize,
        maximum: usize,
    },

    /// An explicitly configured specialization-count budget was exceeded.
    SpecializationBudgetExceeded {
        current: usize,
        maximum: usize,
    },

    /// A specialization identity collided with a different key.
    ///
    /// The engine refuses to reuse an identity for a different instance.
    IdentityCollision {
        specialized_name: String,
    },

    /// Canonical semantic generic binding failed.
    GenericBinding {
        message: String,
    },
}

impl fmt::Display for MonomorphizationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDeclarationIdentity { identity } => {
                write!(
                    formatter,
                    "invalid generic declaration identity `{identity}`"
                )
            }

            Self::InvalidTypeIdentity { index } => {
                write!(
                    formatter,
                    "canonical type identity at argument index {index} is invalid"
                )
            }

            Self::IdentitySizeOverflow => {
                formatter.write_str(
                    "specialization identity size arithmetic overflowed",
                )
            }

            Self::IdentityBudgetExceeded { actual, maximum } => {
                write!(
                    formatter,
                    "specialization identity requires {actual} bytes; \
                     configured budget is {maximum} bytes"
                )
            }

            Self::SpecializationBudgetExceeded { current, maximum } => {
                write!(
                    formatter,
                    "specialization budget exhausted: {current} existing \
                     specializations, configured maximum {maximum}"
                )
            }

            Self::IdentityCollision { specialized_name } => {
                write!(
                    formatter,
                    "specialization identity collision for `{specialized_name}`"
                )
            }

            Self::GenericBinding { message } => {
                write!(
                    formatter,
                    "canonical generic binding failed: {message}"
                )
            }
        }
    }
}

impl Error for MonomorphizationError {}

// =============================================================================
// CANONICAL SPECIALIZATION IDENTITY
// =============================================================================

/// Canonical cache key.
///
/// The original declaration identity must be stable across compilation
/// sessions. It should include the declaration's canonical module and
/// symbol identity, not merely its unqualified source spelling.
///
/// Type arguments are ordered exactly as declared.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct SpecializationKey {
    declaration_identity: String,
    type_arguments: Vec<String>,
}

/// Public record describing one validated specialization.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Specialization {
    /// Canonical identity of the generic declaration.
    pub declaration_identity: String,

    /// Deterministically generated specialization name.
    pub specialized_name: String,

    /// Ordered canonical identities of the resolved type arguments.
    pub type_arguments: Vec<String>,

    /// Full SHA-256 digest of the canonical specialization key.
    pub fingerprint: [u8; 32],

    /// Specialization identity schema version.
    pub schema_version: u16,
}

impl Specialization {
    /// Returns the specialization fingerprint as lowercase hexadecimal.
    #[must_use]
    pub fn fingerprint_hex(&self) -> String {
        encode_hex(&self.fingerprint)
    }
}

// =============================================================================
// OUTCOME
// =============================================================================

/// Result of a specialization request.
///
/// A deferred result is not a successful concrete specialization. Its
/// semantic obligations must be resolved before IR materialization.
#[derive(Clone, Debug)]
pub enum SpecializationOutcome<T> {
    /// Generic constraints were proven complete and a specialization
    /// identity was established.
    Ready {
        /// Deterministic specialization record.
        specialization: Specialization,

        /// Canonical declaration-local semantic substitutions.
        substitutions:
            crate::frontend::semantic::types::generics::GenericSubstitution<T>,
    },

    /// One or more generic obligations remain unresolved.
    ///
    /// The compiler must preserve these obligations and must not treat this
    /// result as proof that the generic instance is valid.
    Deferred {
        /// Complete canonical binding result, including pending constraints.
        binding: GenericBinding<T>,
    },
}

// =============================================================================
// ENGINE
// =============================================================================

/// Deterministic, target-independent specialization coordinator.
#[derive(Clone, Debug)]
pub struct Monomorphizer {
    config: MonomorphizationConfig,

    /// Ordered keys make inspection and reporting deterministic.
    specializations: BTreeMap<SpecializationKey, Specialization>,

    /// Reverse lookup prevents two different keys from silently sharing
    /// the same generated name.
    generated_names: BTreeMap<String, SpecializationKey>,
}

impl Default for Monomorphizer {
    fn default() -> Self {
        Self::new()
    }
}

impl Monomorphizer {
    /// Create a monomorphizer with no artificial count or identity limit.
    #[must_use]
    pub fn new() -> Self {
        Self::with_config(MonomorphizationConfig::unlimited())
    }

    /// Create a monomorphizer with explicit operational policies.
    #[must_use]
    pub fn with_config(config: MonomorphizationConfig) -> Self {
        Self {
            config,
            specializations: BTreeMap::new(),
            generated_names: BTreeMap::new(),
        }
    }

    /// Return the active configuration.
    #[must_use]
    pub fn config(&self) -> &MonomorphizationConfig {
        &self.config
    }

    /// Number of unique specializations retained by this engine.
    #[must_use]
    pub fn specialization_count(&self) -> usize {
        self.specializations.len()
    }

    /// Whether the specialization cache is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.specializations.is_empty()
    }

    /// Return a specialization by canonical declaration and ordered
    /// canonical type identities.
    #[must_use]
    pub fn get(
        &self,
        declaration_identity: &str,
        type_arguments: &[String],
    ) -> Option<&Specialization> {
        let key = SpecializationKey {
            declaration_identity: declaration_identity.to_owned(),
            type_arguments: type_arguments.to_vec(),
        };

        self.specializations.get(&key)
    }

    /// Iterate over specialization records in deterministic key order.
    pub fn iter(&self) -> impl Iterator<Item = &Specialization> {
        self.specializations.values()
    }

    /// Clear specialization records.
    ///
    /// This is intended for an explicitly scoped compilation session.
    /// It does not modify semantic declarations or the canonical type system.
    pub fn reset(&mut self) {
        self.specializations.clear();
        self.generated_names.clear();
    }

    // =========================================================================
    // SEMANTIC BINDING AND SPECIALIZATION
    // =========================================================================

    /// Bind generic arguments through the canonical semantic generic system
    /// and coordinate a deterministic specialization.
    ///
    /// `declaration_identity` must be a stable, fully qualified semantic
    /// declaration identity.
    ///
    /// `parameters` are the canonical frontend generic parameters.
    ///
    /// `arguments` must already be resolved canonical semantic types.
    ///
    /// `where_clause` is the canonical source-level where clause, when present.
    ///
    /// `system` is the authoritative semantic type and constraint system.
    ///
    /// `type_identity` must return a stable canonical identity for a resolved
    /// semantic type. It must not use Debug formatting, pointer addresses,
    /// randomized hashes, or target-specific incidental data.
    ///
    /// A deferred constraint returns `SpecializationOutcome::Deferred`.
    /// It does not produce a concrete specialization record.
    pub fn specialize<S, F>(
        &mut self,
        declaration_identity: &str,
        parameters: &[TypeParameter],
        arguments: &[S::Type],
        where_clause: Option<&WhereClause>,
        system: &mut S,
        mut type_identity: F,
    ) -> Result<SpecializationOutcome<S::Type>, MonomorphizationError>
    where
        S: GenericTypeSystem,
        F: FnMut(&S::Type) -> Result<String, String>,
    {
        validate_declaration_identity(declaration_identity)?;

        // Binding, arity checking, bound checking, and deferred obligations
        // remain owned by the canonical semantic subsystem.
        let binding = bind_type_arguments(
            parameters,
            arguments,
            where_clause,
            system,
        )
        .map_err(|error| MonomorphizationError::GenericBinding {
            message: error.to_string(),
        })?;

        // Deferred constraints are not proof. Do not cache or materialize
        // an instance until all required generic obligations are resolved.
        if !binding.is_complete() {
            return Ok(SpecializationOutcome::Deferred { binding });
        }

        let mut canonical_arguments = Vec::with_capacity(arguments.len());

        for (index, argument) in arguments.iter().enumerate() {
            let identity = type_identity(argument).map_err(|message| {
                MonomorphizationError::GenericBinding { message }
            })?;

            if identity.is_empty() || identity.contains('\0') {
                return Err(
                    MonomorphizationError::InvalidTypeIdentity { index },
                );
            }

            canonical_arguments.push(identity);
        }

        let key = SpecializationKey {
            declaration_identity: declaration_identity.to_owned(),
            type_arguments: canonical_arguments,
        };

        // Cache hits are idempotent. Semantic binding is still performed
        // above so a stale cache record cannot bypass current constraints.
        if let Some(existing) = self.specializations.get(&key) {
            let specialization = existing.clone();
            let (substitutions, _) = binding.into_parts();

            return Ok(SpecializationOutcome::Ready {
                specialization,
                substitutions,
            });
        }

        self.check_identity_budget(&key)?;

        self.check_specialization_budget()?;

        let fingerprint = fingerprint_key(&key)?;

        let specialized_name = format!(
            "__zamani_mono_{}",
            encode_hex(&fingerprint)
        );

        if let Some(existing_key) =
            self.generated_names.get(&specialized_name)
        {
            if existing_key != &key {
                return Err(
                    MonomorphizationError::IdentityCollision {
                        specialized_name,
                    },
                );
            }
        }

        let specialization = Specialization {
            declaration_identity: key.declaration_identity.clone(),
            specialized_name: specialized_name.clone(),
            type_arguments: key.type_arguments.clone(),
            fingerprint,
            schema_version: MONOMORPHIZATION_SCHEMA_VERSION,
        };

        self.generated_names.insert(
            specialized_name,
            key.clone(),
        );

        self.specializations.insert(
            key,
            specialization.clone(),
        );

        let (substitutions, _) = binding.into_parts();

        Ok(SpecializationOutcome::Ready {
            specialization,
            substitutions,
        })
    }

    // =========================================================================
    // BUDGETS
    // =========================================================================

    fn check_specialization_budget(
        &self,
    ) -> Result<(), MonomorphizationError> {
        if let Some(maximum) = self.config.max_specializations {
            let current = self.specializations.len();

            if current >= maximum {
                return Err(
                    MonomorphizationError::SpecializationBudgetExceeded {
                        current,
                        maximum,
                    },
                );
            }
        }

        Ok(())
    }

    fn check_identity_budget(
        &self,
        key: &SpecializationKey,
    ) -> Result<(), MonomorphizationError> {
        let Some(maximum) = self.config.max_identity_bytes else {
            return Ok(());
        };

        let mut total = key.declaration_identity.len();

        for argument in &key.type_arguments {
            total = total.checked_add(argument.len()).ok_or(
                MonomorphizationError::IdentitySizeOverflow,
            )?;
        }

        if total > maximum {
            return Err(
                MonomorphizationError::IdentityBudgetExceeded {
                    actual: total,
                    maximum,
                },
            );
        }

        Ok(())
    }
}

// =============================================================================
// IDENTITY ENCODING
// =============================================================================

fn validate_declaration_identity(
    identity: &str,
) -> Result<(), MonomorphizationError> {
    if identity.trim().is_empty() || identity.contains('\0') {
        return Err(
            MonomorphizationError::InvalidDeclarationIdentity {
                identity: identity.to_owned(),
            },
        );
    }

    Ok(())
}

/// Hash a key using explicit length framing.
///
/// Length framing prevents ambiguous concatenation:
///
///     ["ab", "c"] != ["a", "bc"]
///
/// The declaration and argument order are part of the identity.
fn fingerprint_key(
    key: &SpecializationKey,
) -> Result<[u8; 32], MonomorphizationError> {
    let mut hasher = Sha256::new();

    hash_field(
        &mut hasher,
        MONOMORPHIZATION_IDENTITY_DOMAIN.as_bytes(),
    )?;

    hash_field(
        &mut hasher,
        &MONOMORPHIZATION_SCHEMA_VERSION.to_be_bytes(),
    )?;

    hash_field(
        &mut hasher,
        key.declaration_identity.as_bytes(),
    )?;

    let argument_count = u64::try_from(key.type_arguments.len())
        .map_err(|_| MonomorphizationError::IdentitySizeOverflow)?;

    hash_field(
        &mut hasher,
        &argument_count.to_be_bytes(),
    )?;

    for argument in &key.type_arguments {
        hash_field(&mut hasher, argument.as_bytes())?;
    }

    let digest = hasher.finalize();

    let mut result = [0_u8; 32];
    result.copy_from_slice(&digest);

    Ok(result)
}

fn hash_field(
    hasher: &mut Sha256,
    value: &[u8],
) -> Result<(), MonomorphizationError> {
    let length = u64::try_from(value.len())
        .map_err(|_| MonomorphizationError::IdentitySizeOverflow)?;

    hasher.update(length.to_be_bytes());
    hasher.update(value);

    Ok(())
}

fn encode_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";

    let mut output = String::with_capacity(bytes.len() * 2);

    for byte in bytes {
        output.push(HEX[(byte >> 4) as usize] as char);
        output.push(HEX[(byte & 0x0f) as usize] as char);
    }

    output
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::frontend::ast::node::types::type_expr::{
        TypeExpr,
        TypePath,
    };

    use crate::frontend::semantic::types::generics::{
        ConstraintEvaluation,
        GenericSubstitution,
    };

    #[derive(Clone, Debug, PartialEq, Eq)]
    struct TestType(String);

    #[derive(Debug)]
    struct TestError(String);

    impl fmt::Display for TestError {
        fn fmt(
            &self,
            formatter: &mut fmt::Formatter<'_>,
        ) -> fmt::Result {
            formatter.write_str(&self.0)
        }
    }

    impl Error for TestError {}

    struct TestTypeSystem;

    impl GenericTypeSystem for TestTypeSystem {
        type Type = TestType;
        type Error = TestError;

        fn resolve_type_expr(
            &mut self,
            expression: &TypeExpr,
            substitutions: &GenericSubstitution<Self::Type>,
        ) -> Result<Self::Type, Self::Error> {
            match expression {
                TypeExpr::GenericParameter(name) => substitutions
                    .get(name.as_str())
                    .cloned()
                    .ok_or_else(|| {
                        TestError(format!(
                            "unresolved generic parameter `{}`",
                            name.as_str()
                        ))
                    }),

                TypeExpr::Identifier(path) => {
                    Ok(TestType(path.to_string()))
                }

                other => Ok(TestType(other.to_string())),
            }
        }

        fn check_bound(
            &mut self,
            actual: &Self::Type,
            bound: &TypeExpr,
            _substitutions: &GenericSubstitution<Self::Type>,
        ) -> Result<ConstraintEvaluation, Self::Error> {
            let name = bound.to_string();

            if name == "DeferredBound" {
                return Ok(ConstraintEvaluation::Deferred {
                    reason: "test constraint is unresolved".to_owned(),
                });
            }

            if name == "Numeric"
                && (actual.0 == "i32" || actual.0 == "f64")
            {
                return Ok(ConstraintEvaluation::Satisfied);
            }

            if name == "Numeric" {
                return Ok(ConstraintEvaluation::Unsatisfied);
            }

            Ok(ConstraintEvaluation::Satisfied)
        }
    }

    fn named_type(name: &str) -> TypeExpr {
        TypeExpr::Identifier(TypePath::single(name))
    }

    fn identity(ty: &TestType) -> Result<String, String> {
        Ok(ty.0.clone())
    }

    #[test]
    fn creates_a_deterministic_specialization() {
        let mut first = Monomorphizer::new();
        let mut second = Monomorphizer::new();

        let parameters = vec![TypeParameter::new("T")];
        let arguments = vec![TestType("i32".to_owned())];

        let first_result = first
            .specialize(
                "example::identity",
                &parameters,
                &arguments,
                None,
                &mut TestTypeSystem,
                identity,
            )
            .expect("specialization should succeed");

        let second_result = second
            .specialize(
                "example::identity",
                &parameters,
                &arguments,
                None,
                &mut TestTypeSystem,
                identity,
            )
            .expect("specialization should succeed");

        let first_name = match first_result {
            SpecializationOutcome::Ready {
                specialization, ..
            } => specialization.specialized_name,
            SpecializationOutcome::Deferred { .. } => {
                panic!("test constraints should be complete");
            }
        };

        let second_name = match second_result {
            SpecializationOutcome::Ready {
                specialization, ..
            } => specialization.specialized_name,
            SpecializationOutcome::Deferred { .. } => {
                panic!("test constraints should be complete");
            }
        };

        assert_eq!(first_name, second_name);
    }

    #[test]
    fn repeated_specialization_reuses_the_cache() {
        let mut engine = Monomorphizer::new();

        let parameters = vec![TypeParameter::new("T")];
        let arguments = vec![TestType("i32".to_owned())];

        for _ in 0..2 {
            engine
                .specialize(
                    "example::identity",
                    &parameters,
                    &arguments,
                    None,
                    &mut TestTypeSystem,
                    identity,
                )
                .expect("specialization should succeed");
        }

        assert_eq!(engine.specialization_count(), 1);
    }

    #[test]
    fn different_type_arguments_produce_different_specializations() {
        let mut engine = Monomorphizer::new();

        let parameters = vec![TypeParameter::new("T")];

        let first = engine
            .specialize(
                "example::identity",
                &parameters,
                &[TestType("i32".to_owned())],
                None,
                &mut TestTypeSystem,
                identity,
            )
            .expect("first specialization should succeed");

        let second = engine
            .specialize(
                "example::identity",
                &parameters,
                &[TestType("f64".to_owned())],
                None,
                &mut TestTypeSystem,
                identity,
            )
            .expect("second specialization should succeed");

        let name = |outcome: SpecializationOutcome<TestType>| {
            match outcome {
                SpecializationOutcome::Ready {
                    specialization, ..
                } => specialization.specialized_name,
                SpecializationOutcome::Deferred { .. } => {
                    panic!("test constraints should be complete");
                }
            }
        };

        assert_ne!(name(first), name(second));
        assert_eq!(engine.specialization_count(), 2);
    }

    #[test]
    fn rejects_incomplete_generic_constraints() {
        let mut engine = Monomorphizer::new();

        let parameters = vec![TypeParameter::with_bounds(
            "T",
            vec![named_type("DeferredBound")],
        )];

        let outcome = engine
            .specialize(
                "example::deferred",
                &parameters,
                &[TestType("Unknown".to_owned())],
                None,
                &mut TestTypeSystem,
                identity,
            )
            .expect("deferred constraints must remain representable");

        assert!(matches!(
            outcome,
            SpecializationOutcome::Deferred { .. }
        ));

        assert_eq!(engine.specialization_count(), 0);
    }

    #[test]
    fn rejects_proven_unsatisfied_constraints() {
        let mut engine = Monomorphizer::new();

        let parameters = vec![TypeParameter::with_bounds(
            "T",
            vec![named_type("Numeric")],
        )];

        let result = engine.specialize(
            "example::numeric",
            &parameters,
            &[TestType("Text".to_owned())],
            None,
            &mut TestTypeSystem,
            identity,
        );

        assert!(result.is_err());
        assert_eq!(engine.specialization_count(), 0);
    }

    #[test]
    fn enforces_only_explicit_specialization_budgets() {
        let config = MonomorphizationConfig::unlimited()
            .with_max_specializations(1);

        let mut engine = Monomorphizer::with_config(config);

        let parameters = vec![TypeParameter::new("T")];

        engine
            .specialize(
                "example::identity",
                &parameters,
                &[TestType("i32".to_owned())],
                None,
                &mut TestTypeSystem,
                identity,
            )
            .expect("first specialization should succeed");

        let result = engine.specialize(
            "example::identity",
            &parameters,
            &[TestType("f64".to_owned())],
            None,
            &mut TestTypeSystem,
            identity,
        );

        assert!(matches!(
            result,
            Err(MonomorphizationError::SpecializationBudgetExceeded {
                maximum: 1,
                ..
            })
        ));
    }

    #[test]
    fn rejects_empty_declaration_identity() {
        let mut engine = Monomorphizer::new();

        let result = engine.specialize(
            "",
            &[TypeParameter::new("T")],
            &[TestType("i32".to_owned())],
            None,
            &mut TestTypeSystem,
            identity,
        );

        assert!(matches!(
            result,
            Err(MonomorphizationError::InvalidDeclarationIdentity { .. })
        ));
    }

    #[test]
    fn schema_identity_is_versioned() {
        assert_eq!(MONOMORPHIZATION_SCHEMA_VERSION, 1);
        assert!(!MONOMORPHIZATION_IDENTITY_DOMAIN.is_empty());
    }
}

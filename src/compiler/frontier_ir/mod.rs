//! Zamani Frontier Intermediate Representation (Frontier IR).
//!
//! Frontier IR is the compiler's extension layer for advanced Zamani
//! capabilities that do not belong in the ordinary scalar/control-flow IR.
//!
//! The Frontier IR registry is intentionally small and deterministic.
//! Individual IR families live in their own modules:
//!
//! - [`temporal_and_goal_ir`] — temporal, causal, goal and utility primitives.
//! - [`cognitive_and_asi_ir`] — cognitive and advanced-intelligence primitives.
//! - [`safety_and_rogue_ir`] — safety, containment and rogue-system primitives.
//! - [`omniversal_and_substrate_ir`] — omniversal and computational-substrate
//!   primitives.
//!
//! This module is a registry/facade only. It must not own the compiler's
//! parser, semantic analyzer, optimizer, linker, backend, or build pipeline.
//!
//! ## Production invariants
//!
//! 1. Frontier IR modules are deterministic.
//! 2. The registry has no global mutable state.
//! 3. Importing this module must not perform I/O.
//! 4. Importing this module must not spawn threads or processes.
//! 5. Importing this module must not execute generated code.
//! 6. Frontier IR must remain separable from the ordinary compiler pipeline.
//! 7. New Frontier IR families must be added as explicit modules and exports.
//!
//! The individual Frontier IR families currently expose textual IR
//! constructors. Until a typed Frontier IR representation is introduced,
//! those constructors remain responsible for producing syntactically
//! well-formed Frontier IR fragments.

pub mod temporal_and_goal_ir;
pub mod cognitive_and_asi_ir;
pub mod safety_and_rogue_ir;
pub mod omniversal_and_substrate_ir;

pub use cognitive_and_asi_ir::CognitiveAndAsiIr;
pub use omniversal_and_substrate_ir::OmniversalAndSubstrateIr;
pub use safety_and_rogue_ir::SafetyAndRogueIr;
pub use temporal_and_goal_ir::TemporalAndGoalIr;

/// Number of currently registered Frontier IR families.
pub const FRONTIER_IR_FAMILY_COUNT: usize = 4;

/// Stable names of all registered Frontier IR families.
///
/// Keeping this list centralized prevents tooling from having to infer the
/// registry from Rust module names.
pub const FRONTIER_IR_FAMILIES: &[&str] = &[
    "temporal_and_goal",
    "cognitive_and_asi",
    "safety_and_rogue",
    "omniversal_and_substrate",
];

/// Returns the canonical names of all registered Frontier IR families.
#[must_use]
pub const fn family_names() -> &'static [&'static str] {
    FRONTIER_IR_FAMILIES
}

/// Returns true when `name` identifies a registered Frontier IR family.
#[must_use]
pub fn is_registered_family(name: &str) -> bool {
    FRONTIER_IR_FAMILIES
        .iter()
        .any(|family| family.eq_ignore_ascii_case(name))
}

/// Returns the canonical family name when `name` is registered.
///
/// The returned value is always one of the entries in
/// [`FRONTIER_IR_FAMILIES`].
#[must_use]
pub fn canonical_family_name(name: &str) -> Option<&'static str> {
    FRONTIER_IR_FAMILIES
        .iter()
        .copied()
        .find(|family| family.eq_ignore_ascii_case(name))
}

/// Validates a Frontier IR family name.
///
/// This function is intentionally strict: an unknown family is rejected
/// rather than silently routed to a fallback implementation.
pub fn validate_family_name(name: &str) -> Result<&'static str, FrontierIrRegistryError> {
    canonical_family_name(name).ok_or_else(|| FrontierIrRegistryError::UnknownFamily {
        name: name.to_string(),
    })
}

/// Error returned by Frontier IR registry operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FrontierIrRegistryError {
    /// The requested Frontier IR family does not exist.
    UnknownFamily {
        name: String,
    },
}

impl std::fmt::Display for FrontierIrRegistryError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownFamily { name } => {
                write!(formatter, "unknown Frontier IR family `{name}`")
            }
        }
    }
}

impl std::error::Error for FrontierIrRegistryError {}

/// Describes a registered Frontier IR family.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FrontierIrFamily {
    /// Canonical machine-readable family name.
    pub name: &'static str,
}

/// Returns metadata for every registered Frontier IR family.
#[must_use]
pub const fn registered_families() -> [FrontierIrFamily; FRONTIER_IR_FAMILY_COUNT] {
    [
        FrontierIrFamily {
            name: "temporal_and_goal",
        },
        FrontierIrFamily {
            name: "cognitive_and_asi",
        },
        FrontierIrFamily {
            name: "safety_and_rogue",
        },
        FrontierIrFamily {
            name: "omniversal_and_substrate",
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_contains_expected_number_of_families() {
        assert_eq!(FRONTIER_IR_FAMILY_COUNT, 4);
        assert_eq!(FRONTIER_IR_FAMILIES.len(), 4);
        assert_eq!(registered_families().len(), 4);
    }

    #[test]
    fn registry_family_names_are_unique() {
        for (index, family) in FRONTIER_IR_FAMILIES.iter().enumerate() {
            assert!(
                !FRONTIER_IR_FAMILIES[index + 1..]
                    .iter()
                    .any(|other| other == family),
                "duplicate Frontier IR family `{family}`"
            );
        }
    }

    #[test]
    fn known_family_is_registered() {
        assert!(is_registered_family("temporal_and_goal"));
        assert!(is_registered_family("cognitive_and_asi"));
        assert!(is_registered_family("safety_and_rogue"));
        assert!(is_registered_family("omniversal_and_substrate"));
    }

    #[test]
    fn family_lookup_is_case_insensitive() {
        assert!(is_registered_family("TEMPORAL_AND_GOAL"));
        assert!(is_registered_family("Cognitive_And_Asi"));

        assert_eq!(
            canonical_family_name("SAFETY_AND_ROGUE"),
            Some("safety_and_rogue")
        );
    }

    #[test]
    fn unknown_family_is_rejected() {
        assert!(!is_registered_family("unknown"));
        assert_eq!(canonical_family_name("unknown"), None);

        let error = validate_family_name("unknown")
            .expect_err("unknown Frontier IR family must be rejected");

        assert_eq!(
            error,
            FrontierIrRegistryError::UnknownFamily {
                name: "unknown".to_string()
            }
        );
    }

    #[test]
    fn registered_family_metadata_matches_registry() {
        let metadata = registered_families();

        for (index, family) in metadata.iter().enumerate() {
            assert_eq!(family.name, FRONTIER_IR_FAMILIES[index]);
        }
    }

    #[test]
    fn public_ir_types_are_available() {
        let _ = std::any::TypeId::of::<TemporalAndGoalIr>();
        let _ = std::any::TypeId::of::<CognitiveAndAsiIr>();
        let _ = std::any::TypeId::of::<SafetyAndRogueIr>();
        let _ = std::any::TypeId::of::<OmniversalAndSubstrateIr>();
    }
}
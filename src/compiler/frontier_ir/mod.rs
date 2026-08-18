//! Zamani Frontier IR
//!
//! Central registry and public API for Frontier IR primitive families.
//!
//! The Frontier IR is divided into four independently implemented families:
//!
//! 1. Temporal & Goal primitives
//! 2. Cognitive & AGI/ASI primitives
//! 3. Safety & Rogue-prevention primitives
//! 4. Omniversal & Substrate primitives
//!
//! This module provides:
//! - stable public re-exports;
//! - canonical primitive-family lookup;
//! - case-insensitive family resolution;
//! - registry metadata;
//! - deterministic registry enumeration;
//! - validation of registry invariants.
//!
//! This module does not execute Frontier operations.

pub mod cognitive_and_asi_ir;
pub mod omniversal_and_substrate_ir;
pub mod safety_and_rogue_ir;
pub mod temporal_and_goal_ir;

// ============================================================================
// Public Re-exports
// ============================================================================

pub use cognitive_and_asi_ir::CognitiveAndAsiIr;
pub use omniversal_and_substrate_ir::OmniversalAndSubstrateIr;
pub use safety_and_rogue_ir::SafetyAndRogueIr;
pub use temporal_and_goal_ir::TemporalAndGoalIr;

// ============================================================================
// Registry Constants
// ============================================================================

/// Number of registered Frontier IR primitive families.
pub const FRONTIER_IR_FAMILY_COUNT: usize = 4;

/// Canonical names of all Frontier IR primitive families.
///
/// The order is stable and is part of the registry contract.
pub const FRONTIER_IR_FAMILIES: [&str; FRONTIER_IR_FAMILY_COUNT] = [
    "temporal_and_goal",
    "cognitive_and_asi",
    "safety_and_rogue",
    "omniversal_and_substrate",
];

// ============================================================================
// Registry Errors
// ============================================================================

/// Errors returned when resolving a Frontier IR primitive family.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FrontierIrError {
    /// The requested family does not exist.
    UnknownFamily(String),
}

impl std::fmt::Display for FrontierIrError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownFamily(name) => {
                write!(formatter, "unknown Frontier IR family: {name}")
            }
        }
    }
}

impl std::error::Error for FrontierIrError {}

// ============================================================================
// Registry Metadata
// ============================================================================

/// Metadata describing one Frontier IR primitive family.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FrontierIrFamilyMetadata {
    /// Canonical machine-readable family name.
    pub name: &'static str,

    /// Human-readable family description.
    pub description: &'static str,

    /// First feature number owned by this family.
    pub feature_start: u16,

    /// Last feature number owned by this family.
    pub feature_end: u16,
}

impl FrontierIrFamilyMetadata {
    /// Returns the number of features represented by this family.
    #[must_use]
    pub const fn feature_count(self) -> u16 {
        self.feature_end - self.feature_start + 1
    }
}

/// Complete static Frontier IR registry metadata.
pub const FRONTIER_IR_REGISTRY: [FrontierIrFamilyMetadata;
    FRONTIER_IR_FAMILY_COUNT] = [
    FrontierIrFamilyMetadata {
        name: "temporal_and_goal",
        description: "Temporal, causal and goal-oriented primitives",
        feature_start: 1,
        feature_end: 40,
    },
    FrontierIrFamilyMetadata {
        name: "cognitive_and_asi",
        description: "Cognitive, AGI and ASI primitives",
        feature_start: 41,
        feature_end: 60,
    },
    FrontierIrFamilyMetadata {
        name: "safety_and_rogue",
        description: "Safety, containment and rogue-prevention primitives",
        feature_start: 61,
        feature_end: 80,
    },
    FrontierIrFamilyMetadata {
        name: "omniversal_and_substrate",
        description: "Omniversal and computational-substrate primitives",
        feature_start: 81,
        feature_end: 100,
    },
];

// ============================================================================
// Registry API
// ============================================================================

/// Returns the number of registered Frontier IR families.
#[must_use]
pub const fn family_count() -> usize {
    FRONTIER_IR_FAMILY_COUNT
}

/// Returns the canonical names of all registered Frontier IR families.
///
/// The returned slice is ordered deterministically.
#[must_use]
pub const fn family_names() -> &'static [&'static str; FRONTIER_IR_FAMILY_COUNT] {
    &FRONTIER_IR_FAMILIES
}

/// Returns the complete static Frontier IR registry.
#[must_use]
pub const fn registry() -> &'static [FrontierIrFamilyMetadata;
    FRONTIER_IR_FAMILY_COUNT] {
    &FRONTIER_IR_REGISTRY
}

/// Returns metadata for a Frontier IR family.
///
/// Matching is case-insensitive and accepts surrounding ASCII whitespace.
pub fn family_metadata(
    name: &str,
) -> Result<&'static FrontierIrFamilyMetadata, FrontierIrError> {
    let canonical = canonical_family_name(name);

    FRONTIER_IR_REGISTRY
        .iter()
        .find(|family| family.name == canonical)
        .ok_or_else(|| FrontierIrError::UnknownFamily(name.to_owned()))
}

/// Returns the canonical family name for a supported family.
///
/// This is useful when parser/compiler code needs to normalize user-facing
/// family names before storing them in an AST, IR or diagnostic.
pub fn canonical_family_name(name: &str) -> &str {
    match name.trim().to_ascii_lowercase().as_str() {
        "temporal_and_goal" => "temporal_and_goal",
        "cognitive_and_asi" => "cognitive_and_asi",
        "safety_and_rogue" => "safety_and_rogue",
        "omniversal_and_substrate" => "omniversal_and_substrate",
        _ => "",
    }
}

/// Resolves a Frontier IR family by name.
///
/// Matching is case-insensitive and accepts surrounding ASCII whitespace.
pub fn resolve_family(
    name: &str,
) -> Result<&'static str, FrontierIrError> {
    let canonical = canonical_family_name(name);

    if canonical.is_empty() {
        return Err(FrontierIrError::UnknownFamily(name.to_owned()));
    }

    Ok(canonical)
}

/// Returns whether a family name is registered.
#[must_use]
pub fn is_registered_family(name: &str) -> bool {
    !canonical_family_name(name).is_empty()
}

/// Validates all static registry invariants.
///
/// This is intentionally exposed so compiler startup/self-tests can verify
/// that the Frontier IR registry has not become internally inconsistent.
pub fn validate_registry() -> Result<(), FrontierIrError> {
    if FRONTIER_IR_REGISTRY.len() != FRONTIER_IR_FAMILY_COUNT {
        return Err(FrontierIrError::UnknownFamily(
            "registry family count mismatch".to_owned(),
        ));
    }

    if FRONTIER_IR_FAMILIES.len() != FRONTIER_IR_FAMILY_COUNT {
        return Err(FrontierIrError::UnknownFamily(
            "family-name count mismatch".to_owned(),
        ));
    }

    for (index, family) in FRONTIER_IR_REGISTRY.iter().enumerate() {
        if family.name != FRONTIER_IR_FAMILIES[index] {
            return Err(FrontierIrError::UnknownFamily(format!(
                "registry ordering mismatch at index {index}"
            )));
        }

        if family.feature_start > family.feature_end {
            return Err(FrontierIrError::UnknownFamily(format!(
                "invalid feature range for {}",
                family.name
            )));
        }

        if index > 0 {
            let previous = FRONTIER_IR_REGISTRY[index - 1];

            if family.feature_start != previous.feature_end + 1 {
                return Err(FrontierIrError::UnknownFamily(format!(
                    "non-contiguous feature range between {} and {}",
                    previous.name, family.name
                )));
            }
        }
    }

    Ok(())
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn family_count_matches_registry() {
        assert_eq!(family_count(), 4);
        assert_eq!(FRONTIER_IR_FAMILIES.len(), FRONTIER_IR_FAMILY_COUNT);
        assert_eq!(FRONTIER_IR_REGISTRY.len(), FRONTIER_IR_FAMILY_COUNT);
    }

    #[test]
    fn family_names_are_stable() {
        assert_eq!(
            family_names(),
            &[
                "temporal_and_goal",
                "cognitive_and_asi",
                "safety_and_rogue",
                "omniversal_and_substrate",
            ]
        );
    }

    #[test]
    fn registry_is_valid() {
        assert!(validate_registry().is_ok());
    }

    #[test]
    fn registry_feature_ranges_are_contiguous() {
        assert_eq!(FRONTIER_IR_REGISTRY[0].feature_start, 1);
        assert_eq!(FRONTIER_IR_REGISTRY[0].feature_end, 40);

        assert_eq!(FRONTIER_IR_REGISTRY[1].feature_start, 41);
        assert_eq!(FRONTIER_IR_REGISTRY[1].feature_end, 60);

        assert_eq!(FRONTIER_IR_REGISTRY[2].feature_start, 61);
        assert_eq!(FRONTIER_IR_REGISTRY[2].feature_end, 80);

        assert_eq!(FRONTIER_IR_REGISTRY[3].feature_start, 81);
        assert_eq!(FRONTIER_IR_REGISTRY[3].feature_end, 100);
    }

    #[test]
    fn feature_counts_are_correct() {
        assert_eq!(FRONTIER_IR_REGISTRY[0].feature_count(), 40);
        assert_eq!(FRONTIER_IR_REGISTRY[1].feature_count(), 20);
        assert_eq!(FRONTIER_IR_REGISTRY[2].feature_count(), 20);
        assert_eq!(FRONTIER_IR_REGISTRY[3].feature_count(), 20);
    }

    #[test]
    fn total_feature_count_is_one_hundred() {
        let total: u16 = FRONTIER_IR_REGISTRY
            .iter()
            .map(|family| family.feature_count())
            .sum();

        assert_eq!(total, 100);
    }

    #[test]
    fn family_resolution_is_case_insensitive() {
        assert_eq!(
            resolve_family("TEMPORAL_AND_GOAL").unwrap(),
            "temporal_and_goal"
        );

        assert_eq!(
            resolve_family("Cognitive_And_ASI").unwrap(),
            "cognitive_and_asi"
        );

        assert_eq!(
            resolve_family("SAFETY_AND_ROGUE").unwrap(),
            "safety_and_rogue"
        );

        assert_eq!(
            resolve_family("OMNIVERSAL_AND_SUBSTRATE").unwrap(),
            "omniversal_and_substrate"
        );
    }

    #[test]
    fn family_resolution_trims_whitespace() {
        assert_eq!(
            resolve_family("  temporal_and_goal  ").unwrap(),
            "temporal_and_goal"
        );
    }

    #[test]
    fn unknown_family_is_rejected() {
        let error = resolve_family("does_not_exist").unwrap_err();

        assert_eq!(
            error,
            FrontierIrError::UnknownFamily("does_not_exist".to_owned())
        );
    }

    #[test]
    fn unknown_family_display_is_useful() {
        let error = FrontierIrError::UnknownFamily("example".to_owned());

        assert_eq!(
            error.to_string(),
            "unknown Frontier IR family: example"
        );
    }

    #[test]
    fn registration_check_is_correct() {
        assert!(is_registered_family("temporal_and_goal"));
        assert!(is_registered_family("TEMPORAL_AND_GOAL"));
        assert!(is_registered_family(" safety_and_rogue "));

        assert!(!is_registered_family("unknown"));
        assert!(!is_registered_family(""));
    }

    #[test]
    fn metadata_lookup_returns_expected_family() {
        let metadata = family_metadata("Cognitive_And_ASI").unwrap();

        assert_eq!(metadata.name, "cognitive_and_asi");
        assert_eq!(metadata.feature_start, 41);
        assert_eq!(metadata.feature_end, 60);
        assert_eq!(metadata.feature_count(), 20);
    }

    #[test]
    fn every_family_has_metadata() {
        for name in FRONTIER_IR_FAMILIES {
            let metadata = family_metadata(name).unwrap();

            assert_eq!(metadata.name, name);
            assert!(!metadata.description.is_empty());
        }
    }

    #[test]
    fn canonicalization_is_deterministic() {
        assert_eq!(
            canonical_family_name(" temporal_and_goal "),
            "temporal_and_goal"
        );

        assert_eq!(
            canonical_family_name("COGNITIVE_AND_ASI"),
            "cognitive_and_asi"
        );

        assert_eq!(
            canonical_family_name("invalid"),
            ""
        );
    }

    #[test]
    fn public_ir_types_are_constructible() {
        let _ = TemporalAndGoalIr;
        let _ = CognitiveAndAsiIr;
        let _ = SafetyAndRogueIr::default();
        let _ = OmniversalAndSubstrateIr;
    }
}
//! Zamani Frontier IR Registry
//! Aggregates all 100 unique frontier IR features (Temporal, Goal, Cognitive/ASI, Safety/Rogue, Omniversal/Substrate).

pub mod temporal_and_goal_ir;
pub mod cognitive_and_asi_ir;
pub mod safety_and_rogue_ir;
pub mod omniversal_and_substrate_ir;

pub use temporal_and_goal_ir::TemporalAndGoalIr;
pub use cognitive_and_asi_ir::CognitiveAndAsiIr;
pub use safety_and_rogue_ir::SafetyAndRogueIr;
pub use omniversal_and_substrate_ir::OmniversalAndSubstrateIr;

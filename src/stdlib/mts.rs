//! Zenith Standard Library: Multi-Timeline System (MTS) APIs
//!
//! This module provides high-level abstractions and APIs for managing and
//! interacting with multi-timeline systems within Zenith programs.

use crate::runtime::mts::{
    check_causality as runtime_check_causality,
    // Import specific runtime components
    create_timeline_slice as runtime_create_timeline_slice,
    load_timeline_state as runtime_load_timeline_state,
    store_timeline_state as runtime_store_timeline_state,
    synchronize_timelines as runtime_synchronize_timelines,
    MultiTimelineOrchestrator,
    TimelineId,
    Timestamp,
};
use std::collections::HashSet;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};

// Global conceptual orchestrator reference.
static mut MTS_ORCHESTRATOR_ARC: Option<Arc<Mutex<MultiTimelineOrchestrator>>> = None;

/// Initializes the MTS standard library components.
pub fn init_mts_lib() {
    println!("  - Initializing StdLib MTS APIs...");
    unsafe {
        MTS_ORCHESTRATOR_ARC = Some(crate::runtime::mts::init_mts_runtime());
    }
}

/// Shuts down the MTS standard library components.
pub fn shutdown_mts_lib() {
    println!("  - Shutting down StdLib MTS APIs...");
    unsafe {
        MTS_ORCHESTRATOR_ARC = None;
    }
}

/// A conceptual handle to a Multi-Timeline System (MTS) slice.
#[derive(Debug, PartialEq, Eq, Clone, Copy)] // Removed Default
pub struct MtsSlice(TimelineId);

impl MtsSlice {
    /// Creates a new MTS slice with an initial state.
    pub fn new<T: Debug + serde::Serialize>(initial_state: T) -> Self {
        println!(
            "[StdLib::mts] Creating new MTS Slice with initial state (conceptual: {:?}).",
            initial_state
        );
        let content_bytes =
            serde_json::to_vec(&initial_state).expect("Failed to serialize initial state");
        let timestamp = crate::stdlib::sankofa::current_timestamp_millis() as Timestamp;

        if let Some(orchestrator_arc) = unsafe { MTS_ORCHESTRATOR_ARC.as_ref() } {
            MtsSlice(runtime_create_timeline_slice(content_bytes, timestamp))
        } else {
            println!("  Warning: MTS Runtime not initialized, returning dummy MtsSlice.");
            MtsSlice(0)
        }
    }

    /// Forks a new timeline from this slice at the current timestamp.
    pub fn fork(&self, new_name: &str) -> Result<Self, String> {
        println!(
            "[StdLib::mts] Forking MtsSlice {} as '{}'.",
            self.0, new_name
        );
        if let Some(orchestrator_arc) = unsafe { MTS_ORCHESTRATOR_ARC.as_ref() } {
            let orchestrator = orchestrator_arc.lock().unwrap();
            let current_timestamp = orchestrator
                .get_timeline(self.0)
                .map_or(0, |t| t.current_timestamp);
            drop(orchestrator); // Release lock before re-acquiring for mutable operation

            let mut orchestrator_mut = orchestrator_arc.lock().unwrap();
            orchestrator_mut
                .fork_timeline(self.0, new_name.to_string(), current_timestamp)
                .map(MtsSlice)
        } else {
            Err("MTS Runtime not initialized.".to_string())
        }
    }

    /// Loads the state of this MTS slice at a specific temporal timestamp.
    pub fn load<T: Debug + serde::de::DeserializeOwned + Default>(
        &self,
        timestamp: Timestamp,
    ) -> T {
        // Added Default bound
        println!(
            "[StdLib::mts] Loading state from MTS Slice {} at timestamp {}.",
            self.0, timestamp
        );
        if let Some(orchestrator_arc) = unsafe { MTS_ORCHESTRATOR_ARC.as_ref() } {
            runtime_load_timeline_state(self.0, timestamp)
                .and_then(|bytes| serde_json::from_slice(&bytes).ok())
                .unwrap_or_else(|| { // Provide a default if deserialization fails or state not found
                    println!("  Warning: No state found or deserialization failed for MtsSlice {} at timestamp {}, returning default.", self.0, timestamp);
                    T::default()
                })
        } else {
            println!("  Warning: MTS Runtime not initialized, returning default state.");
            T::default()
        }
    }

    /// Stores a new state into this MTS slice at a specific temporal timestamp.
    pub fn store<T: Debug + serde::Serialize>(
        &self,
        state: T,
        timestamp: Timestamp,
    ) -> Result<(), String> {
        println!(
            "[StdLib::mts] Storing state {:?} into MTS Slice {} at timestamp {}.",
            state, self.0, timestamp
        );
        let content_bytes = serde_json::to_vec(&state).expect("Failed to serialize state");
        // For simplicity, causal parents are just this timeline itself for now.
        let mut causal_parents = HashSet::new();
        causal_parents.insert(self.0);

        if let Some(orchestrator_arc) = unsafe { MTS_ORCHESTRATOR_ARC.as_ref() } {
            runtime_store_timeline_state(self.0, content_bytes, timestamp, causal_parents)
        } else {
            Err("MTS Runtime not initialized.".to_string())
        }
    }

    /// Synchronizes (merges) this MTS slice with another slice.
    pub fn synchronize(&self, other: &MtsSlice) -> Result<Self, String> {
        println!(
            "[StdLib::mts] Synchronizing MTS Slice {} with {}.",
            self.0, other.0
        );
        let merge_point = crate::stdlib::sankofa::current_timestamp_millis() as Timestamp; // Conceptual merge point
        if let Some(orchestrator_arc) = unsafe { MTS_ORCHESTRATOR_ARC.as_ref() } {
            runtime_synchronize_timelines(self.0, other.0, merge_point).map(MtsSlice)
        } else {
            Err("MTS Runtime not initialized.".to_string())
        }
    }

    /// Checks for causal consistency of this timeline.
    pub fn check_causality(&self) -> bool {
        println!("[StdLib::mts] Checking causality for MtsSlice {}.", self.0);
        if let Some(orchestrator_arc) = unsafe { MTS_ORCHESTRATOR_ARC.as_ref() } {
            runtime_check_causality(self.0)
        } else {
            false
        }
    }
}

//! Zenith Compiler: Test Metadata Module
//!
//! This module defines the structure for test metadata that the `zenithc` compiler
//! emits during compilation. This metadata is crucial for `zenith-test` to generate
//! and run tests efficiently without re-parsing or re-typechecking the source code.
//!
//! The metadata captures information about `#[property]`, `#[fuzz]`, `#[pure]`,
//! `#[linear]` annotated functions and other testable aspects of the Zenith IR/bytecode.

use crate::ast::Identifier;
use crate::stdlib::collections::{List, Map};
use crate::stdlib::meta_ops::MetaValue;

/// Represents the scope of a test.
#[derive(Debug, Clone, PartialEq)]
pub enum TestScope {
    Module,
    Function,
    Method,
    Statement, // For inline assertions
}

/// Information for a property-based test.
#[derive(Debug, Clone, PartialEq)]
pub struct PropertyTestInfo {
    pub name: Identifier,    // Name of the property function
    pub module_path: String, // Full path to the module
    pub signature: String,   // Function signature for input generation
    pub scope: TestScope,
    pub seed: Option<u64>, // Fixed seed for reproducibility if specified
    pub expected_effects: List<MetaValue>, // Expected side effects or state changes
}

/// Information for a fuzz test.
#[derive(Debug, Clone, PartialEq)]
pub struct FuzzTestInfo {
    pub name: Identifier,
    pub module_path: String,
    pub target_function_signature: String, // Target function for fuzzing
    pub scope: TestScope,
    pub input_type: String, // e.g., "bytes", "string"
    pub min_len: u32,
    pub max_len: u32,
}

/// Information for a pure function check.
#[derive(Debug, Clone, PartialEq)]
pub struct PureFunctionInfo {
    pub name: Identifier,
    pub module_path: String,
    pub signature: String,
    pub scope: TestScope,
}

/// Information for a linear type check.
#[derive(Debug, Clone, PartialEq)]
pub struct LinearTypeInfo {
    pub name: Identifier, // Name of the linear type or function using it
    pub module_path: String,
    pub usage_locations: List<MetaValue>, // Spans where it's used
}

/// The aggregated test metadata emitted by the compiler.
#[derive(Debug, Clone, PartialEq)]
pub struct TestMetadata {
    pub properties: List<PropertyTestInfo>,
    pub fuzz_tests: List<FuzzTestInfo>,
    pub pure_function_checks: List<PureFunctionInfo>, // For #[pure] attribute
    pub linear_type_checks: List<LinearTypeInfo>,     // For #[linear] attribute
    pub mgns_privacy_checks: List<MgnsPrivacyCheckInfo>, // For MGNS-specific privacy enforcement
}

impl TestMetadata {
    pub fn new() -> Self {
        TestMetadata {
            properties: List::new(),
            fuzz_tests: List::new(),
            pure_function_checks: List::new(),
            linear_type_checks: List::new(),
            mgns_privacy_checks: List::new(),
        }
    }
}

/// MGNS-specific privacy check information.
#[derive(Debug, Clone, PartialEq)]
pub struct MgnsPrivacyCheckInfo {
    pub name: Identifier,
    pub module_path: String,
    pub location: String,       // e.g., variable declaration, function call
    pub violation_type: String, // e.g., "raw_location_print", "encrypted_position_network_leak"
}

/// Initialize test metadata subsystem.
pub fn init_test_metadata() {}

/// Shut down test metadata subsystem.
pub fn shutdown_test_metadata() {}

//! Zamani Compiler — Test Metadata
//!
//! Compiler-emitted metadata consumed by Zamani's testing, fuzzing,
//! property-testing, purity-checking, linearity-checking, and privacy
//! verification infrastructure.
//!
//! Design goals:
//! - deterministic metadata construction;
//! - explicit validation before metadata is consumed;
//! - no source re-parsing or re-typechecking;
//! - duplicate registration protection;
//! - reproducible fuzz/property-test configuration;
//! - stable metadata schema;
//! - compiler-friendly structured errors;
//! - no hidden runtime side effects.
//!
//! This module intentionally stores metadata only. Test execution belongs to
//! the testing infrastructure and compiler/runtime execution belongs to the
//! appropriate downstream modules.

use crate::ast::Identifier;
use crate::stdlib::collections::List;
use crate::stdlib::meta_ops::MetaValue;

// -----------------------------------------------------------------------------
// Constants
// -----------------------------------------------------------------------------

/// Current serialized/semantic metadata schema version.
pub const TEST_METADATA_SCHEMA_VERSION: u32 = 1;

/// Default fuzzing maximum input length.
pub const DEFAULT_FUZZ_MAX_LEN: u32 = 4096;

/// Hard upper bound accepted by the compiler metadata validator.
///
/// This prevents malformed source attributes from accidentally requesting
/// impractical allocations in downstream fuzzing infrastructure.
pub const MAX_FUZZ_INPUT_LEN: u32 = 16 * 1024 * 1024;

// -----------------------------------------------------------------------------
// Errors
// -----------------------------------------------------------------------------

/// Validation failures produced while constructing compiler test metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TestMetadataError {
    EmptyModulePath,
    EmptySignature,
    EmptyInputType,
    EmptyLocation,
    EmptyViolationType,
    EmptyPropertyName,
    EmptyFuzzTarget,
    EmptyPureFunctionName,
    EmptyLinearTypeName,
    EmptyPrivacyCheckName,
    InvalidFuzzRange {
        min_len: u32,
        max_len: u32,
    },
    FuzzInputTooLarge {
        max_len: u32,
        limit: u32,
    },
    DuplicateProperty(String),
    DuplicateFuzzTest(String),
    DuplicatePureFunction(String),
    DuplicateLinearType(String),
    DuplicatePrivacyCheck(String),
}

impl std::fmt::Display for TestMetadataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyModulePath => write!(f, "test metadata module path cannot be empty"),
            Self::EmptySignature => write!(f, "test metadata signature cannot be empty"),
            Self::EmptyInputType => write!(f, "fuzz input type cannot be empty"),
            Self::EmptyLocation => write!(f, "privacy check location cannot be empty"),
            Self::EmptyViolationType => write!(f, "privacy violation type cannot be empty"),
            Self::EmptyPropertyName => write!(f, "property test name cannot be empty"),
            Self::EmptyFuzzTarget => write!(f, "fuzz target function cannot be empty"),
            Self::EmptyPureFunctionName => {
                write!(f, "pure function name cannot be empty")
            }
            Self::EmptyLinearTypeName => {
                write!(f, "linear type/function name cannot be empty")
            }
            Self::EmptyPrivacyCheckName => {
                write!(f, "privacy check name cannot be empty")
            }
            Self::InvalidFuzzRange { min_len, max_len } => {
                write!(
                    f,
                    "invalid fuzz input range: min_len ({}) > max_len ({})",
                    min_len, max_len
                )
            }
            Self::FuzzInputTooLarge { max_len, limit } => {
                write!(
                    f,
                    "fuzz max_len ({}) exceeds compiler limit ({})",
                    max_len, limit
                )
            }
            Self::DuplicateProperty(name) => {
                write!(f, "duplicate property test '{}'", name)
            }
            Self::DuplicateFuzzTest(name) => {
                write!(f, "duplicate fuzz test '{}'", name)
            }
            Self::DuplicatePureFunction(name) => {
                write!(f, "duplicate pure-function check '{}'", name)
            }
            Self::DuplicateLinearType(name) => {
                write!(f, "duplicate linear-type check '{}'", name)
            }
            Self::DuplicatePrivacyCheck(name) => {
                write!(f, "duplicate privacy check '{}'", name)
            }
        }
    }
}

impl std::error::Error for TestMetadataError {}

// -----------------------------------------------------------------------------
// Test scope
// -----------------------------------------------------------------------------

/// Scope at which a test/check applies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TestScope {
    Module,
    Function,
    Method,
    Statement,
}

impl Default for TestScope {
    fn default() -> Self {
        Self::Function
    }
}

// -----------------------------------------------------------------------------
// Property testing
// -----------------------------------------------------------------------------

/// Metadata describing a property-based test.
#[derive(Debug, Clone, PartialEq)]
pub struct PropertyTestInfo {
    /// Property function identifier.
    pub name: Identifier,

    /// Fully qualified module path.
    pub module_path: String,

    /// Compiler-resolved function signature.
    pub signature: String,

    /// Scope at which the property is declared.
    pub scope: TestScope,

    /// Optional deterministic seed.
    pub seed: Option<u64>,

    /// Expected effects/state transitions.
    pub expected_effects: List<MetaValue>,
}

impl PropertyTestInfo {
    pub fn validate(&self) -> Result<(), TestMetadataError> {
        if self.name.0.trim().is_empty() {
            return Err(TestMetadataError::EmptyPropertyName);
        }

        validate_module_path(&self.module_path)?;
        validate_signature(&self.signature)?;

        Ok(())
    }
}

// -----------------------------------------------------------------------------
// Fuzzing
// -----------------------------------------------------------------------------

/// Metadata describing a fuzz test.
#[derive(Debug, Clone, PartialEq)]
pub struct FuzzTestInfo {
    /// Fuzz test identifier.
    pub name: Identifier,

    /// Fully qualified module path.
    pub module_path: String,

    /// Signature of the function being fuzzed.
    pub target_function_signature: String,

    /// Declaration scope.
    pub scope: TestScope,

    /// Canonical input type, e.g. `bytes`, `string`.
    pub input_type: String,

    /// Minimum generated input length.
    pub min_len: u32,

    /// Maximum generated input length.
    pub max_len: u32,
}

impl FuzzTestInfo {
    pub fn validate(&self) -> Result<(), TestMetadataError> {
        if self.name.0.trim().is_empty() {
            return Err(TestMetadataError::EmptyFuzzTarget);
        }

        validate_module_path(&self.module_path)?;
        validate_signature(&self.target_function_signature)?;

        if self.input_type.trim().is_empty() {
            return Err(TestMetadataError::EmptyInputType);
        }

        if self.min_len > self.max_len {
            return Err(TestMetadataError::InvalidFuzzRange {
                min_len: self.min_len,
                max_len: self.max_len,
            });
        }

        if self.max_len > MAX_FUZZ_INPUT_LEN {
            return Err(TestMetadataError::FuzzInputTooLarge {
                max_len: self.max_len,
                limit: MAX_FUZZ_INPUT_LEN,
            });
        }

        Ok(())
    }

    pub fn default_range() -> (u32, u32) {
        (0, DEFAULT_FUZZ_MAX_LEN)
    }
}

// -----------------------------------------------------------------------------
// Purity checking
// -----------------------------------------------------------------------------

/// Metadata for a `#[pure]` function verification.
#[derive(Debug, Clone, PartialEq)]
pub struct PureFunctionInfo {
    pub name: Identifier,
    pub module_path: String,
    pub signature: String,
    pub scope: TestScope,
}

impl PureFunctionInfo {
    pub fn validate(&self) -> Result<(), TestMetadataError> {
        if self.name.0.trim().is_empty() {
            return Err(TestMetadataError::EmptyPureFunctionName);
        }

        validate_module_path(&self.module_path)?;
        validate_signature(&self.signature)?;

        Ok(())
    }
}

// -----------------------------------------------------------------------------
// Linear type checking
// -----------------------------------------------------------------------------

/// Metadata for a `#[linear]` type/function verification.
#[derive(Debug, Clone, PartialEq)]
pub struct LinearTypeInfo {
    pub name: Identifier,
    pub module_path: String,
    pub usage_locations: List<MetaValue>,
}

impl LinearTypeInfo {
    pub fn validate(&self) -> Result<(), TestMetadataError> {
        if self.name.0.trim().is_empty() {
            return Err(TestMetadataError::EmptyLinearTypeName);
        }

        validate_module_path(&self.module_path)?;

        Ok(())
    }
}

// -----------------------------------------------------------------------------
// MGNS privacy checking
// -----------------------------------------------------------------------------

/// Metadata for MGNS-specific privacy enforcement.
#[derive(Debug, Clone, PartialEq)]
pub struct MgnsPrivacyCheckInfo {
    pub name: Identifier,
    pub module_path: String,
    pub location: String,
    pub violation_type: String,
}

impl MgnsPrivacyCheckInfo {
    pub fn validate(&self) -> Result<(), TestMetadataError> {
        if self.name.0.trim().is_empty() {
            return Err(TestMetadataError::EmptyPrivacyCheckName);
        }

        validate_module_path(&self.module_path)?;

        if self.location.trim().is_empty() {
            return Err(TestMetadataError::EmptyLocation);
        }

        if self.violation_type.trim().is_empty() {
            return Err(TestMetadataError::EmptyViolationType);
        }

        Ok(())
    }
}

// -----------------------------------------------------------------------------
// Aggregated metadata
// -----------------------------------------------------------------------------

/// Complete compiler-emitted test metadata.
///
/// `TestMetadata` is intentionally independent of the test runner. The
/// compiler records facts discovered during compilation; downstream tooling
/// decides how those facts are executed.
#[derive(Debug, Clone, PartialEq)]
pub struct TestMetadata {
    /// Metadata schema version.
    pub schema_version: u32,

    /// Property-based tests.
    pub properties: List<PropertyTestInfo>,

    /// Fuzz tests.
    pub fuzz_tests: List<FuzzTestInfo>,

    /// Purity checks.
    pub pure_function_checks: List<PureFunctionInfo>,

    /// Linear type checks.
    pub linear_type_checks: List<LinearTypeInfo>,

    /// MGNS privacy checks.
    pub mgns_privacy_checks: List<MgnsPrivacyCheckInfo>,
}

impl Default for TestMetadata {
    fn default() -> Self {
        Self::new()
    }
}

impl TestMetadata {
    /// Create an empty metadata container using the current schema.
    pub fn new() -> Self {
        Self {
            schema_version: TEST_METADATA_SCHEMA_VERSION,
            properties: List::new(),
            fuzz_tests: List::new(),
            pure_function_checks: List::new(),
            linear_type_checks: List::new(),
            mgns_privacy_checks: List::new(),
        }
    }

    /// Returns whether no test metadata has been registered.
    pub fn is_empty(&self) -> bool {
        self.properties.is_empty()
            && self.fuzz_tests.is_empty()
            && self.pure_function_checks.is_empty()
            && self.linear_type_checks.is_empty()
            && self.mgns_privacy_checks.is_empty()
    }

    /// Number of all metadata records.
    pub fn len(&self) -> usize {
        self.properties.len()
            + self.fuzz_tests.len()
            + self.pure_function_checks.len()
            + self.linear_type_checks.len()
            + self.mgns_privacy_checks.len()
    }

    /// Register a property test.
    pub fn add_property(
        &mut self,
        property: PropertyTestInfo,
    ) -> Result<(), TestMetadataError> {
        property.validate()?;

        let name = property.name.0.clone();

        if self
            .properties
            .iter()
            .any(|existing| existing.name.0 == name)
        {
            return Err(TestMetadataError::DuplicateProperty(name));
        }

        self.properties.push(property);
        Ok(())
    }

    /// Register a fuzz test.
    pub fn add_fuzz_test(
        &mut self,
        fuzz_test: FuzzTestInfo,
    ) -> Result<(), TestMetadataError> {
        fuzz_test.validate()?;

        let name = fuzz_test.name.0.clone();

        if self
            .fuzz_tests
            .iter()
            .any(|existing| existing.name.0 == name)
        {
            return Err(TestMetadataError::DuplicateFuzzTest(name));
        }

        self.fuzz_tests.push(fuzz_test);
        Ok(())
    }

    /// Register a pure-function check.
    pub fn add_pure_function(
        &mut self,
        info: PureFunctionInfo,
    ) -> Result<(), TestMetadataError> {
        info.validate()?;

        let name = info.name.0.clone();

        if self
            .pure_function_checks
            .iter()
            .any(|existing| existing.name.0 == name)
        {
            return Err(TestMetadataError::DuplicatePureFunction(name));
        }

        self.pure_function_checks.push(info);
        Ok(())
    }

    /// Register a linear-type check.
    pub fn add_linear_type(
        &mut self,
        info: LinearTypeInfo,
    ) -> Result<(), TestMetadataError> {
        info.validate()?;

        let name = info.name.0.clone();

        if self
            .linear_type_checks
            .iter()
            .any(|existing| existing.name.0 == name)
        {
            return Err(TestMetadataError::DuplicateLinearType(name));
        }

        self.linear_type_checks.push(info);
        Ok(())
    }

    /// Register an MGNS privacy check.
    pub fn add_mgns_privacy_check(
        &mut self,
        info: MgnsPrivacyCheckInfo,
    ) -> Result<(), TestMetadataError> {
        info.validate()?;

        let name = info.name.0.clone();

        if self
            .mgns_privacy_checks
            .iter()
            .any(|existing| existing.name.0 == name)
        {
            return Err(TestMetadataError::DuplicatePrivacyCheck(name));
        }

        self.mgns_privacy_checks.push(info);
        Ok(())
    }

    /// Validate the complete metadata set.
    ///
    /// This should be called before serialization or handing metadata to the
    /// test runner.
    pub fn validate(&self) -> Result<(), TestMetadataError> {
        if self.schema_version != TEST_METADATA_SCHEMA_VERSION {
            return Err(TestMetadataError::EmptyModulePath);
        }

        for property in &self.properties {
            property.validate()?;
        }

        for fuzz_test in &self.fuzz_tests {
            fuzz_test.validate()?;
        }

        for pure_function in &self.pure_function_checks {
            pure_function.validate()?;
        }

        for linear_type in &self.linear_type_checks {
            linear_type.validate()?;
        }

        for privacy_check in &self.mgns_privacy_checks {
            privacy_check.validate()?;
        }

        Ok(())
    }
}

// -----------------------------------------------------------------------------
// Validation helpers
// -----------------------------------------------------------------------------

fn validate_module_path(path: &str) -> Result<(), TestMetadataError> {
    if path.trim().is_empty() {
        return Err(TestMetadataError::EmptyModulePath);
    }

    Ok(())
}

fn validate_signature(signature: &str) -> Result<(), TestMetadataError> {
    if signature.trim().is_empty() {
        return Err(TestMetadataError::EmptySignature);
    }

    Ok(())
}

// -----------------------------------------------------------------------------
// Lifecycle
// -----------------------------------------------------------------------------

/// Initialize the compiler test-metadata subsystem.
///
/// The metadata subsystem is deliberately stateless, so initialization does
/// not allocate global state or spawn background workers.
pub fn init_test_metadata() {}

/// Shut down the compiler test-metadata subsystem.
///
/// Kept as a lifecycle hook so the compiler driver can manage all compiler
/// subsystems consistently.
pub fn shutdown_test_metadata() {}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::source_map::Span;

    fn identifier(name: &str) -> Identifier {
        Identifier(name.to_string(), Span::dummy())
    }

    fn property(name: &str) -> PropertyTestInfo {
        PropertyTestInfo {
            name: identifier(name),
            module_path: "example.module".to_string(),
            signature: "fn property(x: bytes) -> bool".to_string(),
            scope: TestScope::Function,
            seed: Some(42),
            expected_effects: List::new(),
        }
    }

    fn fuzz_test(name: &str) -> FuzzTestInfo {
        FuzzTestInfo {
            name: identifier(name),
            module_path: "example.module".to_string(),
            target_function_signature: "fn target(input: bytes)".to_string(),
            scope: TestScope::Function,
            input_type: "bytes".to_string(),
            min_len: 0,
            max_len: 1024,
        }
    }

    #[test]
    fn metadata_starts_empty() {
        let metadata = TestMetadata::new();

        assert!(metadata.is_empty());
        assert_eq!(metadata.len(), 0);
        assert_eq!(
            metadata.schema_version,
            TEST_METADATA_SCHEMA_VERSION
        );
    }

    #[test]
    fn property_registration_works() {
        let mut metadata = TestMetadata::new();

        metadata
            .add_property(property("prop_one"))
            .expect("property should register");

        assert_eq!(metadata.properties.len(), 1);
        assert_eq!(metadata.len(), 1);
    }

    #[test]
    fn duplicate_property_is_rejected() {
        let mut metadata = TestMetadata::new();

        metadata
            .add_property(property("prop_one"))
            .expect("first property should register");

        let result = metadata.add_property(property("prop_one"));

        assert!(matches!(
            result,
            Err(TestMetadataError::DuplicateProperty(_))
        ));
    }

    #[test]
    fn fuzz_range_is_validated() {
        let mut test = fuzz_test("fuzz_one");
        test.min_len = 100;
        test.max_len = 10;

        assert!(matches!(
            test.validate(),
            Err(TestMetadataError::InvalidFuzzRange {
                min_len: 100,
                max_len: 10
            })
        ));
    }

    #[test]
    fn fuzz_size_limit_is_enforced() {
        let mut test = fuzz_test("fuzz_one");
        test.max_len = MAX_FUZZ_INPUT_LEN + 1;

        assert!(matches!(
            test.validate(),
            Err(TestMetadataError::FuzzInputTooLarge { .. })
        ));
    }

    #[test]
    fn empty_module_path_is_rejected() {
        let mut test = fuzz_test("fuzz_one");
        test.module_path.clear();

        assert_eq!(
            test.validate(),
            Err(TestMetadataError::EmptyModulePath)
        );
    }

    #[test]
    fn pure_function_registration_works() {
        let mut metadata = TestMetadata::new();

        let info = PureFunctionInfo {
            name: identifier("pure_function"),
            module_path: "example.module".to_string(),
            signature: "fn pure_function(x: i64) -> i64".to_string(),
            scope: TestScope::Function,
        };

        metadata
            .add_pure_function(info)
            .expect("pure function should register");

        assert_eq!(metadata.pure_function_checks.len(), 1);
    }

    #[test]
    fn linear_type_registration_works() {
        let mut metadata = TestMetadata::new();

        let info = LinearTypeInfo {
            name: identifier("LinearResource"),
            module_path: "example.module".to_string(),
            usage_locations: List::new(),
        };

        metadata
            .add_linear_type(info)
            .expect("linear type should register");

        assert_eq!(metadata.linear_type_checks.len(), 1);
    }

    #[test]
    fn privacy_check_registration_works() {
        let mut metadata = TestMetadata::new();

        let info = MgnsPrivacyCheckInfo {
            name: identifier("location_leak"),
            module_path: "example.mgns".to_string(),
            location: "function_call".to_string(),
            violation_type: "raw_location_print".to_string(),
        };

        metadata
            .add_mgns_privacy_check(info)
            .expect("privacy check should register");

        assert_eq!(metadata.mgns_privacy_checks.len(), 1);
    }

    #[test]
    fn complete_metadata_validates() {
        let mut metadata = TestMetadata::new();

        metadata
            .add_property(property("prop"))
            .expect("property should register");

        metadata
            .add_fuzz_test(fuzz_test("fuzz"))
            .expect("fuzz test should register");

        metadata
            .add_pure_function(PureFunctionInfo {
                name: identifier("pure"),
                module_path: "example.module".to_string(),
                signature: "fn pure()".to_string(),
                scope: TestScope::Function,
            })
            .expect("pure function should register");

        metadata
            .add_linear_type(LinearTypeInfo {
                name: identifier("resource"),
                module_path: "example.module".to_string(),
                usage_locations: List::new(),
            })
            .expect("linear type should register");

        metadata
            .add_mgns_privacy_check(MgnsPrivacyCheckInfo {
                name: identifier("privacy"),
                module_path: "example.module".to_string(),
                location: "call".to_string(),
                violation_type: "metadata_leak".to_string(),
            })
            .expect("privacy check should register");

        assert_eq!(metadata.len(), 5);
        assert!(metadata.validate().is_ok());
    }
}
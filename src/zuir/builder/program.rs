#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Program Builder
//!
//! Path: src/zuir/builder/program.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! PURPOSE
//!
//! Provide a safe, fallible, extensible API for assembling a ZUIR program
//! from canonical modules, dialect requirements, and descriptive metadata.
//!
//! AUTHORITY
//!
//! - src/zuir/ZUIR-SPEC.md
//! - src/zuir/ids.rs
//! - src/zuir/version.rs
//! - src/zuir/ir/program.rs
//! - src/zuir/ir/module.rs
//! - grammar/DESIGN.md
//! - grammar/Zamani.g4
//! - grammar/specification/semantic-model.md
//! - grammar/spec/resources.md
//!
//! OWNS
//!
//! - Program construction workflow.
//! - Fallible program-ID allocation for convenience construction.
//! - Explicit program-ID construction.
//! - Checked delegation to canonical program-container operations.
//! - Final local container validation before returning a program.
//!
//! DOES NOT OWN
//!
//! - Program storage or module-container invariants.
//! - Module, function, operation, type, or SSA semantics.
//! - Parsing, AST construction, name resolution, or type inference.
//! - Dialect registration or trust decisions.
//! - Full-program verification.
//! - Resource feasibility, physical allocation, routing, or scheduling.
//! - Serialization formats or backend execution.
//!
//! SCALABILITY
//!
//! This builder imposes no universal hardware or program-size limits.
//! Collections are dynamically sized in the canonical program container.
//! All finite-resource failures must be reported explicitly by the
//! responsible allocator, collection, verifier, or compilation stage.
//!
//! SAFETY
//!
//! Safe Rust only. No unsafe code.
//!
//! FREEZE CRITERIA
//!
//! - Uses the canonical ZuirProgram<M>; does not duplicate its storage.
//! - Duplicate module IDs never overwrite existing modules.
//! - ID allocation exhaustion is reported rather than wrapped.
//! - Dialect and metadata validation is delegated to the canonical owner.
//! - Build validates the completed container.
//! - No target-specific or grammar-specific assumptions are introduced.

use core::fmt;

use crate::zuir::ids::{IdAllocator, IdSpaceExhausted, ModuleId, ProgramId};
use crate::zuir::ir::program::{
    DialectName, DialectRequirement, ProgramError, ZuirProgram,
};
use crate::zuir::version::{
    DialectVersion, ZuirFormatVersion, ZuirSchemaVersion,
    CURRENT_ZUIR_FORMAT_VERSION, CURRENT_ZUIR_SCHEMA_VERSION,
};

// =============================================================================
// Builder errors
// =============================================================================

/// Errors produced while constructing a ZUIR program.
///
/// These errors preserve the distinction between identifier-allocation
/// failures, canonical container failures, and final validation failures.
///
/// This type does not replace the repository-wide `ZuirError`. A higher
/// integration boundary may map it into `ZuirError` without changing the
/// local builder API.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProgramBuilderError {
    /// The program-ID allocator exhausted its identifier space.
    IdSpaceExhausted,

    /// The canonical program container rejected an operation.
    Program(ProgramError),
}

impl fmt::Display for ProgramBuilderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IdSpaceExhausted => {
                f.write_str("unable to allocate a ZUIR program identifier")
            }
            Self::Program(error) => {
                write!(f, "ZUIR program construction failed: {error}")
            }
        }
    }
}

impl std::error::Error for ProgramBuilderError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::IdSpaceExhausted => None,
            Self::Program(error) => Some(error),
        }
    }
}

impl From<ProgramError> for ProgramBuilderError {
    fn from(error: ProgramError) -> Self {
        Self::Program(error)
    }
}

impl From<IdSpaceExhausted> for ProgramBuilderError {
    fn from(_: IdSpaceExhausted) -> Self {
        Self::IdSpaceExhausted
    }
}

/// Result type for fallible program-builder operations.
pub type ProgramBuilderResult<T> = Result<T, ProgramBuilderError>;

// =============================================================================
// Program builder
// =============================================================================

/// A validated-construction interface for a canonical ZUIR program.
///
/// `M` is the canonical module payload selected by the caller. In the
/// integrated compiler this will normally be the module type owned by
/// `src/zuir/ir/module.rs`.
///
/// The generic parameter allows the builder to be implemented independently
/// of unfinished function and operation models without inventing duplicate
/// representations.
///
/// The builder does not infer the module ID from an arbitrary payload.
/// Callers supply the canonical module ID explicitly, avoiding hidden
/// identity assumptions and allowing integration adapters to enforce their
/// own module-identity contract.
///
/// # Identity
///
/// `new()` allocates a program ID from a fresh allocator. This is convenient
/// for isolated construction but does not establish global uniqueness
/// across independent builders.
///
/// For coordinated compilation, deterministic fixtures, or imported
/// artifacts, use `with_id()` with an ID allocated by the compilation
/// context or restored from the enclosing artifact.
///
/// # Validation
///
/// Each mutating operation delegates to `ZuirProgram<M>`, the canonical
/// container owner. `build()` performs the container's local validation.
///
/// This does not replace whole-program verification, dialect registry
/// checks, version compatibility checks, or target-feasibility analysis.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProgramBuilder<M> {
    program: ZuirProgram<M>,
}

impl<M> ProgramBuilder<M> {
    // -------------------------------------------------------------------------
    // Construction
    // -------------------------------------------------------------------------

    /// Create an empty program using the current canonical ZUIR versions.
    ///
    /// A new allocator is used for this convenience constructor. Therefore,
    /// IDs generated by separate calls are not guaranteed to be globally
    /// unique. Use `with_id()` when identity allocation is managed by a
    /// compilation session or another enclosing owner.
    ///
    /// The operation is fallible so identifier exhaustion is never hidden.
    pub fn new() -> ProgramBuilderResult<Self> {
        let allocator = IdAllocator::<ProgramId>::new();
        let id = allocator.try_next()?;

        Ok(Self::with_id(
            id,
            CURRENT_ZUIR_SCHEMA_VERSION,
            CURRENT_ZUIR_FORMAT_VERSION,
        ))
    }

    /// Create an empty program with an explicit identity and versions.
    ///
    /// This does not silently upgrade or downgrade the supplied versions.
    /// Compatibility policy belongs to the versioning and interchange
    /// subsystems.
    #[must_use]
    pub fn with_id(
        id: ProgramId,
        schema_version: ZuirSchemaVersion,
        format_version: ZuirFormatVersion,
    ) -> Self {
        Self {
            program: ZuirProgram::new(id, schema_version, format_version),
        }
    }

    /// Start a builder around an existing canonical program.
    ///
    /// The supplied program is validated before it is accepted. This is
    /// useful for controlled transformations that need to add modules,
    /// dialect requirements, or descriptive metadata.
    ///
    /// Full IR verification remains the responsibility of the verifier.
    pub fn from_program(
        program: ZuirProgram<M>,
    ) -> ProgramBuilderResult<Self> {
        program.validate().map_err(ProgramBuilderError::from)?;

        Ok(Self { program })
    }

    // -------------------------------------------------------------------------
    // Identity and version access
    // -------------------------------------------------------------------------

    /// Return the canonical program identity.
    #[must_use]
    pub fn id(&self) -> ProgramId {
        self.program.id()
    }

    /// Return the schema version.
    #[must_use]
    pub fn schema_version(&self) -> ZuirSchemaVersion {
        self.program.schema_version()
    }

    /// Return the serialization format version.
    #[must_use]
    pub fn format_version(&self) -> ZuirFormatVersion {
        self.program.format_version()
    }

    // -------------------------------------------------------------------------
    // Module construction
    // -------------------------------------------------------------------------

    /// Add a module under its canonical module ID.
    ///
    /// The ID must be the identity owned by the supplied module. Because
    /// `M` is generic, this method cannot infer or independently validate
    /// that relationship. The integration adapter must obtain the ID from
    /// the canonical module object and pass it here.
    ///
    /// A duplicate ID is rejected. The rejected module is returned to the
    /// caller so that ownership is not lost.
    pub fn add_module(
        &mut self,
        id: ModuleId,
        module: M,
    ) -> Result<(), (ProgramBuilderError, M)> {
        self.program
            .insert_module(id, module)
            .map_err(|(error, module)| {
                (ProgramBuilderError::Program(error), module)
            })
    }

    /// Return whether a module ID is already present.
    #[must_use]
    pub fn contains_module(&self, id: ModuleId) -> bool {
        self.program.contains_module(id)
    }

    /// Return the number of modules currently assembled.
    #[must_use]
    pub fn module_count(&self) -> usize {
        self.program.module_count()
    }

    /// Return whether the program contains no modules.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.program.is_empty()
    }

    /// Borrow a module by its canonical ID.
    #[must_use]
    pub fn module(&self, id: ModuleId) -> Option<&M> {
        self.program.module(id)
    }

    /// Mutably borrow a module by its canonical ID.
    ///
    /// Mutation can invalidate previous verification results. The resulting
    /// program must be verified again before trusted downstream use.
    pub fn module_mut(&mut self, id: ModuleId) -> Option<&mut M> {
        self.program.module_mut(id)
    }

    /// Remove a module and return ownership to the caller.
    pub fn remove_module(&mut self, id: ModuleId) -> Option<M> {
        self.program.remove_module(id)
    }

    /// Iterate through modules in deterministic ModuleId order.
    pub fn modules(
        &self,
    ) -> impl ExactSizeIterator<Item = (ModuleId, &M)> + '_ {
        self.program.modules()
    }

    // -------------------------------------------------------------------------
    // Dialect requirements
    // -------------------------------------------------------------------------

    /// Require a dialect at an explicit version.
    ///
    /// This records the program's interpretation dependency. It does not
    /// register the dialect, execute extension code, establish trust, or
    /// prove compatibility with an installed dialect implementation.
    pub fn require_dialect(
        &mut self,
        name: DialectName,
        version: DialectVersion,
    ) -> ProgramBuilderResult<()> {
        let requirement = DialectRequirement::new(name, version);

        self.program
            .require_dialect(requirement)
            .map_err(ProgramBuilderError::from)
    }

    /// Require a dialect using a string name.
    ///
    /// Name syntax is validated by the canonical DialectName type.
    pub fn require_dialect_named(
        &mut self,
        name: impl Into<String>,
        version: DialectVersion,
    ) -> ProgramBuilderResult<()> {
        let name = DialectName::new(name.into())
            .map_err(ProgramBuilderError::from)?;

        self.require_dialect(name, version)
    }

    /// Look up a required dialect by its validated name.
    #[must_use]
    pub fn dialect_requirement(
        &self,
        name: &DialectName,
    ) -> Option<&DialectRequirement> {
        self.program.dialect_requirement(name)
    }

    /// Iterate over dialect requirements in deterministic name order.
    pub fn dialect_requirements(
        &self,
    ) -> impl ExactSizeIterator<Item = (&DialectName, &DialectRequirement)> + '_
    {
        self.program.dialect_requirements()
    }

    /// Explicitly replace a dialect requirement.
    ///
    /// This is intended for controlled migrations or transformations.
    /// Callers remain responsible for checking semantic compatibility before
    /// accepting the resulting program for compilation.
    pub fn replace_dialect_requirement(
        &mut self,
        name: DialectName,
        version: DialectVersion,
    ) -> ProgramBuilderResult<Option<DialectRequirement>> {
        let requirement = DialectRequirement::new(name, version);

        self.program
            .replace_dialect_requirement(requirement)
            .map_err(ProgramBuilderError::from)
    }

    /// Remove a dialect requirement.
    pub fn remove_dialect_requirement(
        &mut self,
        name: &DialectName,
    ) -> Option<DialectRequirement> {
        self.program.remove_dialect_requirement(name)
    }

    // -------------------------------------------------------------------------
    // Descriptive metadata
    // -------------------------------------------------------------------------

    /// Set descriptive metadata and return any previous value.
    ///
    /// Metadata is not a substitute for typed operation attributes,
    /// semantic effects, capabilities, resource constraints, or provenance.
    ///
    /// The canonical program container validates metadata keys.
    pub fn set_metadata(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> ProgramBuilderResult<Option<String>> {
        self.program
            .set_metadata(key, value)
            .map_err(ProgramBuilderError::from)
    }

    /// Read descriptive metadata.
    #[must_use]
    pub fn metadata(&self, key: &str) -> Option<&str> {
        self.program.metadata(key)
    }

    /// Remove descriptive metadata.
    pub fn remove_metadata(&mut self, key: &str) -> Option<String> {
        self.program.remove_metadata(key)
    }

    /// Iterate over descriptive metadata in deterministic key order.
    pub fn metadata_entries(
        &self,
    ) -> impl ExactSizeIterator<Item = (&str, &str)> + '_ {
        self.program.metadata_entries()
    }

    // -------------------------------------------------------------------------
    // Validation and completion
    // -------------------------------------------------------------------------

    /// Validate the invariants owned by the canonical program container.
    ///
    /// This does not verify the internal structure of arbitrary module
    /// payloads. It also does not prove that required dialects are installed
    /// or that the program can execute on a particular target.
    pub fn validate(&self) -> ProgramBuilderResult<()> {
        self.program
            .validate()
            .map_err(ProgramBuilderError::from)
    }

    /// Finish construction and return the canonical program.
    ///
    /// The program is validated before being returned. A successful result
    /// means that the program container satisfies its local invariants;
    /// it does not mean that full semantic verification or target lowering
    /// has already succeeded.
    pub fn build(self) -> ProgramBuilderResult<ZuirProgram<M>> {
        self.program
            .validate()
            .map_err(ProgramBuilderError::from)?;

        Ok(self.program)
    }

    /// Borrow the assembled canonical program without consuming the builder.
    #[must_use]
    pub fn as_program(&self) -> &ZuirProgram<M> {
        &self.program
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::zuir::ids::{IdAllocator, ModuleId, ProgramId};

    fn test_program_id() -> ProgramId {
        IdAllocator::<ProgramId>::new()
            .try_next()
            .expect("program ID allocation should succeed")
    }

    fn test_module_id(value: u128) -> ModuleId {
        ModuleId::new(value).expect("test module ID must be nonzero")
    }

    fn builder() -> ProgramBuilder<String> {
        ProgramBuilder::with_id(
            test_program_id(),
            CURRENT_ZUIR_SCHEMA_VERSION,
            CURRENT_ZUIR_FORMAT_VERSION,
        )
    }

    #[test]
    fn creates_an_empty_program_with_explicit_versions() {
        let builder = builder();

        assert!(builder.is_empty());
        assert_eq!(builder.module_count(), 0);
        assert_eq!(
            builder.schema_version(),
            CURRENT_ZUIR_SCHEMA_VERSION
        );
        assert_eq!(
            builder.format_version(),
            CURRENT_ZUIR_FORMAT_VERSION
        );
    }

    #[test]
    fn adds_and_retrieves_modules() {
        let mut builder = builder();
        let id = test_module_id(10);

        builder
            .add_module(id, "classical module".to_owned())
            .expect("module insertion should succeed");

        assert_eq!(builder.module_count(), 1);
        assert!(builder.contains_module(id));
        assert_eq!(
            builder.module(id).map(String::as_str),
            Some("classical module")
        );
    }

    #[test]
    fn duplicate_module_ids_do_not_replace_existing_modules() {
        let mut builder = builder();
        let id = test_module_id(10);

        builder
            .add_module(id, "original".to_owned())
            .expect("first insertion should succeed");

        let (error, returned_module) = builder
            .add_module(id, "replacement".to_owned())
            .expect_err("duplicate module IDs must be rejected");

        assert_eq!(
            error,
            ProgramBuilderError::Program(
                ProgramError::DuplicateModule(id)
            )
        );

        assert_eq!(returned_module, "replacement");
        assert_eq!(
            builder.module(id).map(String::as_str),
            Some("original")
        );
    }

    #[test]
    fn module_iteration_is_deterministic() {
        let mut builder = builder();

        builder
            .add_module(test_module_id(30), "c".to_owned())
            .expect("insert");
        builder
            .add_module(test_module_id(10), "a".to_owned())
            .expect("insert");
        builder
            .add_module(test_module_id(20), "b".to_owned())
            .expect("insert");

        let ids: Vec<_> = builder.modules().map(|(id, _)| id).collect();

        assert_eq!(
            ids,
            vec![
                test_module_id(10),
                test_module_id(20),
                test_module_id(30),
            ]
        );
    }

    #[test]
    fn dialect_requirements_are_validated_and_unique() {
        let mut builder = builder();

        builder
            .require_dialect_named(
                "quantum",
                DialectVersion::new(1, 0, 0),
            )
            .expect("dialect requirement should be accepted");

        let duplicate = builder.require_dialect_named(
            "quantum",
            DialectVersion::new(1, 0, 0),
        );

        assert!(matches!(
            duplicate,
            Err(ProgramBuilderError::Program(
                ProgramError::DuplicateDialectRequirement(_)
            ))
        ));
    }

    #[test]
    fn invalid_dialect_names_are_rejected() {
        let mut builder = builder();

        let result = builder.require_dialect_named(
            "quantum/physical",
            DialectVersion::new(1, 0, 0),
        );

        assert!(matches!(
            result,
            Err(ProgramBuilderError::Program(
                ProgramError::InvalidDialectName(_)
            ))
        ));
    }

    #[test]
    fn metadata_is_validated_and_retrievable() {
        let mut builder = builder();

        builder
            .set_metadata("build.origin", "test")
            .expect("valid metadata should be accepted");

        assert_eq!(builder.metadata("build.origin"), Some("test"));

        assert!(matches!(
            builder.set_metadata(" invalid ", "value"),
            Err(ProgramBuilderError::Program(
                ProgramError::InvalidMetadataKey(_)
            ))
        ));
    }

    #[test]
    fn build_returns_a_valid_canonical_program() {
        let mut builder = builder();

        builder
            .add_module(test_module_id(1), "module".to_owned())
            .expect("module insertion");

        builder
            .require_dialect_named(
                "classical",
                DialectVersion::new(1, 0, 0),
            )
            .expect("dialect requirement");

        builder
            .set_metadata("origin", "unit-test")
            .expect("metadata");

        let program = builder.build().expect("build should succeed");

        assert_eq!(program.module_count(), 1);
        assert!(program.validate().is_ok());
    }

    #[test]
    fn from_program_validates_before_accepting_existing_state() {
        let original = builder().build().expect("valid empty program");

        let rebuilt = ProgramBuilder::from_program(original.clone())
            .expect("valid program should be accepted")
            .build()
            .expect("rebuilt program should be valid");

        assert_eq!(rebuilt, original);
    }

    #[test]
    fn removing_a_module_returns_ownership() {
        let mut builder = builder();
        let id = test_module_id(8);

        builder
            .add_module(id, "module".to_owned())
            .expect("insert");

        assert_eq!(
            builder.remove_module(id).as_deref(),
            Some("module")
        );

        assert!(!builder.contains_module(id));
        assert!(builder.is_empty());
    }

    #[test]
    fn builder_does_not_encode_hardware_capacity() {
        // The builder stores logical module identities. It does not assign
        // modules to processors, qubits, GPUs, FPGAs, or physical devices.
        let mut builder = builder();

        builder
            .add_module(test_module_id(u128::MAX), "logical".to_owned())
            .expect("nonzero typed IDs are accepted");

        assert_eq!(builder.module_count(), 1);
    }
}
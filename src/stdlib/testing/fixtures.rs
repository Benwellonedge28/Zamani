
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Standard Library — Test Fixtures
//!
//! Path: `src/stdlib/testing/fixtures.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Provides portable, deterministic, in-memory test fixtures for
//! the Zamani programming language and its computational ecosystem.
//!
//! Fixtures support:
//!
//! - Classical programs.
//! - Quantum programs.
//! - Hardware-description programs.
//! - Hybrid programs.
//! - Distributed programs.
//! - AI and scientific programs.
//! - Networking and security programs.
//! - Embedded programs.
//! - Multi-file projects.
//! - Cross-domain integration tests.
//! - Grammar and parser tests.
//! - Semantic validation tests.
//! - Compiler and IR tests.
//! - Runtime and provider conformance tests.
//! - Portability and scalability tests.
//!
//! A fixture is test input. It is not an executable program,
//! AST, semantic model, IR, runtime environment, or provider.
//!
//! ## ARCHITECTURAL AUTHORITY
//!
//! Grammar:
//!     grammar/
//!
//! Canonical grammar composition:
//!     grammar/Zamani.g4
//!
//! Canonical lexer:
//!     grammar/antlr/ZamaniLexer.g4
//!
//! Canonical token composition:
//!     grammar/lexer/tokens.g4
//!
//! Canonical AST:
//!     src/frontend/ast/
//!
//! Canonical parser:
//!     src/frontend/parser/
//!
//! Canonical semantic analysis:
//!     src/frontend/semantic/
//!
//! Classical computation:
//!     src/classical/
//!
//! Quantum computation:
//!     src/quantum/
//!
//! Hardware description:
//!     src/hdl/
//!
//! Hybrid computation:
//!     src/hybrid/
//!
//! Distributed computation:
//!     src/distributed/
//!
//! Universal IR:
//!     src/zuir/
//!
//! Compiler:
//!     src/compiler/
//!
//! Runtime:
//!     src/runtime/
//!
//! Standard-library testing:
//!     src/stdlib/testing/
//!
//! ## OWNS
//!
//! - Fixture identifiers.
//! - Virtual fixture paths.
//! - Fixture files.
//! - Fixture collections.
//! - Primary source selection.
//! - Fixture metadata.
//! - Fixture tags.
//! - Fixture construction.
//! - Fixture validation.
//!
//! ## DOES NOT OWN
//!
//! - Grammar definitions.
//! - Token definitions.
//! - Lexical analysis.
//! - Parsing.
//! - AST construction.
//! - Semantic analysis.
//! - IR construction.
//! - Compiler execution.
//! - Runtime execution.
//! - Hardware discovery.
//! - Resource allocation.
//! - Provider registration.
//! - Test scheduling.
//! - Random value generation.
//! - Fuzzing algorithms.
//! - Filesystem access.
//!
//! These responsibilities belong to their authoritative subsystems.
//!
//! ## POCO-REAF
//!
//! Fixtures must remain independent of the execution environment.
//!
//! The same fixture may be consumed by:
//!
//! - A small local development machine.
//! - An embedded development environment.
//! - A classical compiler.
//! - A quantum compiler.
//! - An HDL compiler.
//! - A hybrid compiler.
//! - A distributed compiler.
//! - A heterogeneous execution environment.
//! - Future computational domains.
//!
//! A fixture does not specify a particular processor, accelerator,
//! quantum device, operating system, or execution topology.
//!
//! ## SCALABILITY
//!
//! No arbitrary universal maximum is imposed on:
//!
//! - Fixture count.
//! - Files per fixture.
//! - Source length.
//! - File path length.
//! - Metadata entries.
//! - Tags.
//! - Fixture identifiers.
//! - Project size.
//! - Domain count.
//! - Resource dimensions represented in fixture source.
//!
//! Actual capacity depends on available memory, address space,
//! implementation capabilities, and explicitly configured
//! operational budgets.
//!
//! The implementation does not claim literal infinite storage.
//!
//! ## DETERMINISM
//!
//! Fixture metadata and file iteration use ordered collections.
//!
//! Identical construction inputs produce identical fixture contents
//! and iteration order.
//!
//! No random generation, wall-clock access, hardware inspection,
//! filesystem access, or network access occurs.
//!
//! ## SAFETY
//!
//! Safe Rust only.
//!
//! No unsafe blocks.
//! No raw pointers.
//! No global mutable state.
//! No implicit filesystem access.
//! No implicit network access.
//! No implicit hardware access.
//!
//! ## INTEGRATION
//!
//! Public path:
//!
//!     crate::stdlib::testing::fixtures
//!
//! Parser adapters consume:
//!
//!     Fixture::primary_file()
//!     Fixture::files()
//!
//! Semantic, compiler, IR, runtime, and domain-specific tests
//! consume fixture source through the same interface.
//!
//! Assertions remain owned by assertions.rs.
//!
//! Conformance execution remains owned by conformance.rs.
//!
//! ## FREEZE CRITERIA
//!
//! 1. Fixture identifiers are validated.
//! 2. Virtual paths reject traversal and absolute paths.
//! 3. Duplicate file paths are rejected.
//! 4. Primary source references are validated.
//! 5. Empty source contents remain valid.
//! 6. Multi-file projects are supported.
//! 7. Metadata and tags are deterministic.
//! 8. No grammar or AST authority is duplicated.
//! 9. No arbitrary universal fixture limits are introduced.
//! 10. Safe Rust is maintained.
//! 11. Unit and boundary tests pass.
//! 12. Rust 1.97 or later compiles the module.

// =============================================================================
// Imports
// =============================================================================

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;
use std::sync::Arc;

// =============================================================================
// Identifier validation
// =============================================================================

/// Validates a fixture identifier.
///
/// Identifiers use dot-separated, non-empty components.
///
/// Supported characters are ASCII letters, digits, underscores,
/// and hyphens.
///
/// Examples:
///
/// - `classical.arithmetic`
/// - `quantum.circuit`
/// - `hdl.synthesis`
/// - `hybrid.execution`
/// - `distributed.consistency`
/// - `future.custom_domain`
fn validate_identifier(value: &str) -> Result<(), FixtureError> {
    if value.is_empty() {
        return Err(FixtureError::EmptyIdentifier);
    }

    if value.trim() != value {
        return Err(FixtureError::SurroundingWhitespace);
    }

    if value.split('.').any(str::is_empty) {
        return Err(FixtureError::EmptyIdentifierComponent);
    }

    if value.chars().any(|character| {
        !(character.is_ascii_alphanumeric()
            || character == '_'
            || character == '-'
            || character == '.')
    }) {
        return Err(FixtureError::InvalidIdentifierCharacter);
    }

    Ok(())
}

// =============================================================================
// Fixture identity
// =============================================================================

/// Globally meaningful identity for one test fixture.
///
/// Fixture identifiers are independent of:
///
/// - Hardware.
/// - Providers.
/// - Operating systems.
/// - Compiler backends.
/// - Runtime environments.
/// - Computational domain implementations.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FixtureId(String);

impl FixtureId {
    /// Creates a validated fixture identifier.
    ///
    /// # Errors
    ///
    /// Returns an error if the identifier is empty, contains
    /// surrounding whitespace, contains empty components,
    /// or contains unsupported characters.
    pub fn new(value: impl Into<String>) -> Result<Self, FixtureError> {
        let value = value.into();

        validate_identifier(&value)?;

        Ok(Self(value))
    }

    /// Returns the fixture identifier as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for FixtureId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// =============================================================================
// Virtual fixture paths
// =============================================================================

/// Portable relative path identifying a file inside a fixture.
///
/// Paths always use `/` as their separator, regardless of the
/// operating system consuming the fixture.
///
/// Absolute paths, parent traversal, empty components, backslashes,
/// NUL characters, and control characters are rejected.
///
/// These paths are virtual identifiers. They do not access the
/// host filesystem.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FixturePath(String);

impl FixturePath {
    /// Creates a validated portable fixture path.
    ///
    /// Examples:
    ///
    /// - `main.zm`
    /// - `src/main.zm`
    /// - `modules/classical/math.zm`
    /// - `hardware/controller.zm`
    ///
    /// # Errors
    ///
    /// Returns an error for absolute paths, parent traversal,
    /// invalid separators, empty path components, or control
    /// characters.
    pub fn new(value: impl Into<String>) -> Result<Self, FixtureError> {
        let value = value.into();

        if value.is_empty() {
            return Err(FixtureError::EmptyPath);
        }

        if value.starts_with('/') {
            return Err(FixtureError::AbsolutePath);
        }

        if value.contains('\\') {
            return Err(FixtureError::InvalidPathSeparator);
        }

        if value.contains('\0') {
            return Err(FixtureError::InvalidPathCharacter);
        }

        if value.chars().any(char::is_control) {
            return Err(FixtureError::InvalidPathCharacter);
        }

        let mut components = value.split('/');

        let first = components
            .next()
            .ok_or(FixtureError::EmptyPathComponent)?;

        if first.is_empty() {
            return Err(FixtureError::EmptyPathComponent);
        }

        // Reject platform-specific drive and URI-style prefixes.
        if first.contains(':') {
            return Err(FixtureError::AbsolutePath);
        }

        if first == "." || first == ".." {
            return Err(FixtureError::PathTraversal);
        }

        for component in components {
            if component.is_empty() {
                return Err(FixtureError::EmptyPathComponent);
            }

            if component == "." || component == ".." {
                return Err(FixtureError::PathTraversal);
            }
        }

        Ok(Self(value))
    }

    /// Returns the portable path as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns the final path component.
    #[must_use]
    pub fn file_name(&self) -> &str {
        self.0
            .rsplit('/')
            .next()
            .unwrap_or(self.0.as_str())
    }

    /// Returns the parent directory when one exists.
    #[must_use]
    pub fn parent(&self) -> Option<&str> {
        self.0.rsplit_once('/').map(|(parent, _)| parent)
    }
}

impl fmt::Display for FixturePath {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// =============================================================================
// Fixture file
// =============================================================================

/// One immutable virtual source file belonging to a fixture.
///
/// Source contents use UTF-8 because Zamani source fixtures are
/// textual language inputs.
///
/// Empty source files are valid fixture inputs.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FixtureFile {
    path: FixturePath,
    contents: Arc<str>,
}

impl FixtureFile {
    /// Creates an immutable fixture file.
    ///
    /// # Errors
    ///
    /// Returns an error if the supplied path is invalid.
    pub fn new(
        path: impl Into<String>,
        contents: impl Into<Arc<str>>,
    ) -> Result<Self, FixtureError> {
        Ok(Self {
            path: FixturePath::new(path)?,
            contents: contents.into(),
        })
    }

    /// Returns the virtual file path.
    #[must_use]
    pub fn path(&self) -> &FixturePath {
        &self.path
    }

    /// Returns the complete source contents.
    #[must_use]
    pub fn contents(&self) -> &str {
        &self.contents
    }

    /// Returns the number of UTF-8 bytes in the source.
    #[must_use]
    pub fn byte_len(&self) -> usize {
        self.contents.len()
    }

    /// Returns whether the source file is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.contents.is_empty()
    }
}

// =============================================================================
// Fixture metadata
// =============================================================================

/// Deterministic metadata associated with a fixture.
///
/// Metadata describes the purpose and classification of test input.
///
/// It does not establish language semantics or execution requirements.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FixtureMetadata {
    description: Option<String>,
    tags: BTreeSet<String>,
    attributes: BTreeMap<String, String>,
}

impl FixtureMetadata {
    /// Creates empty fixture metadata.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the fixture description.
    #[must_use]
    pub fn with_description(
        mut self,
        description: impl Into<String>,
    ) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Adds a validated classification tag.
    ///
    /// Duplicate tags are ignored.
    ///
    /// # Errors
    ///
    /// Returns an error when the tag is invalid.
    pub fn add_tag(
        &mut self,
        tag: impl Into<String>,
    ) -> Result<(), FixtureError> {
        let tag = tag.into();

        validate_identifier(&tag)?;

        self.tags.insert(tag);

        Ok(())
    }

    /// Adds a validated metadata attribute.
    ///
    /// Existing attributes with the same key are replaced.
    ///
    /// # Errors
    ///
    /// Returns an error when the attribute key is invalid.
    pub fn insert_attribute(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Option<String>, FixtureError> {
        let key = key.into();

        validate_identifier(&key)?;

        Ok(self.attributes.insert(key, value.into()))
    }

    /// Returns the optional description.
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Returns all classification tags in deterministic order.
    #[must_use]
    pub fn tags(&self) -> impl Iterator<Item = &str> {
        self.tags.iter().map(String::as_str)
    }

    /// Returns whether the fixture contains a classification tag.
    #[must_use]
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.contains(tag)
    }

    /// Returns a metadata attribute.
    #[must_use]
    pub fn attribute(&self, key: &str) -> Option<&str> {
        self.attributes.get(key).map(String::as_str)
    }

    /// Returns all attributes in deterministic key order.
    #[must_use]
    pub fn attributes(&self) -> &BTreeMap<String, String> {
        &self.attributes
    }
}

// =============================================================================
// Fixture
// =============================================================================

/// Complete immutable test fixture.
///
/// A fixture contains one or more virtual source files, an optional
/// primary source file, and descriptive metadata.
///
/// File iteration is deterministic and ordered by virtual path.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Fixture {
    id: FixtureId,
    files: BTreeMap<FixturePath, FixtureFile>,
    primary_file: Option<FixturePath>,
    metadata: FixtureMetadata,
}

impl Fixture {
    /// Returns the fixture identifier.
    #[must_use]
    pub fn id(&self) -> &FixtureId {
        &self.id
    }

    /// Returns the number of virtual source files.
    #[must_use]
    pub fn file_count(&self) -> usize {
        self.files.len()
    }

    /// Returns whether the fixture contains no source files.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.files.is_empty()
    }

    /// Returns the primary source path when configured.
    #[must_use]
    pub fn primary_path(&self) -> Option<&FixturePath> {
        self.primary_file.as_ref()
    }

    /// Returns the primary source file when configured.
    #[must_use]
    pub fn primary_file(&self) -> Option<&FixtureFile> {
        self.primary_file
            .as_ref()
            .and_then(|path| self.files.get(path))
    }

    /// Returns a source file by virtual path.
    #[must_use]
    pub fn file(&self, path: &str) -> Option<&FixtureFile> {
        let path = FixturePath::new(path).ok()?;
        self.files.get(&path)
    }

    /// Returns all files in deterministic path order.
    #[must_use]
    pub fn files(
        &self,
    ) -> impl Iterator<Item = &FixtureFile> {
        self.files.values()
    }

    /// Returns the complete virtual file map.
    #[must_use]
    pub fn file_map(
        &self,
    ) -> &BTreeMap<FixturePath, FixtureFile> {
        &self.files
    }

    /// Returns fixture metadata.
    #[must_use]
    pub fn metadata(&self) -> &FixtureMetadata {
        &self.metadata
    }

    /// Returns the total source size in bytes.
    ///
    /// Returns `None` only if the total cannot be represented by
    /// the platform's `usize`.
    #[must_use]
    pub fn total_source_bytes(&self) -> Option<usize> {
        self.files.values().try_fold(0usize, |total, file| {
            total.checked_add(file.byte_len())
        })
    }

    /// Returns the number of files with the given extension.
    ///
    /// The extension comparison is case-sensitive.
    #[must_use]
    pub fn count_extension(&self, extension: &str) -> usize {
        self.files
            .values()
            .filter(|file| {
                file.path()
                    .file_name()
                    .rsplit_once('.')
                    .is_some_and(|(_, ext)| ext == extension)
            })
            .count()
    }
}

// =============================================================================
// Fixture builder
// =============================================================================

/// Mutable construction interface for an immutable fixture.
///
/// A builder validates every inserted file and prevents duplicate
/// virtual paths.
///
/// Construction does not access the filesystem or execute source.
#[derive(Clone, Debug)]
pub struct FixtureBuilder {
    id: FixtureId,
    files: BTreeMap<FixturePath, FixtureFile>,
    primary_file: Option<FixturePath>,
    metadata: FixtureMetadata,
}

impl FixtureBuilder {
    /// Creates a fixture builder.
    #[must_use]
    pub fn new(id: FixtureId) -> Self {
        Self {
            id,
            files: BTreeMap::new(),
            primary_file: None,
            metadata: FixtureMetadata::new(),
        }
    }

    /// Creates a builder directly from a string identifier.
    ///
    /// # Errors
    ///
    /// Returns an error if the identifier is invalid.
    pub fn named(id: impl Into<String>) -> Result<Self, FixtureError> {
        Ok(Self::new(FixtureId::new(id)?))
    }

    /// Adds a virtual source file.
    ///
    /// Duplicate paths are rejected rather than silently replaced.
    ///
    /// # Errors
    ///
    /// Returns an error if the path is invalid or already exists.
    pub fn add_file(
        &mut self,
        path: impl Into<String>,
        contents: impl Into<Arc<str>>,
    ) -> Result<&mut Self, FixtureError> {
        let file = FixtureFile::new(path, contents)?;

        if self.files.contains_key(file.path()) {
            return Err(FixtureError::DuplicatePath(
                file.path().clone(),
            ));
        }

        self.files.insert(file.path().clone(), file);

        Ok(self)
    }

    /// Adds a file and makes it the primary source.
    ///
    /// # Errors
    ///
    /// Returns an error if the path is invalid or already exists.
    pub fn add_primary_file(
        &mut self,
        path: impl Into<String>,
        contents: impl Into<Arc<str>>,
    ) -> Result<&mut Self, FixtureError> {
        let file = FixtureFile::new(path, contents)?;
        let path = file.path().clone();

        if self.files.contains_key(&path) {
            return Err(FixtureError::DuplicatePath(path));
        }

        self.files.insert(path.clone(), file);
        self.primary_file = Some(path);

        Ok(self)
    }

    /// Selects an already-added file as the primary source.
    ///
    /// # Errors
    ///
    /// Returns an error if the path is invalid or does not exist.
    pub fn set_primary_file(
        &mut self,
        path: &str,
    ) -> Result<&mut Self, FixtureError> {
        let path = FixturePath::new(path)?;

        if !self.files.contains_key(&path) {
            return Err(FixtureError::PrimaryFileNotFound(path));
        }

        self.primary_file = Some(path);

        Ok(self)
    }

    /// Sets the fixture description.
    pub fn set_description(
        &mut self,
        description: impl Into<String>,
    ) -> &mut Self {
        self.metadata.description = Some(description.into());
        self
    }

    /// Adds a fixture classification tag.
    ///
    /// # Errors
    ///
    /// Returns an error if the tag is invalid.
    pub fn add_tag(
        &mut self,
        tag: impl Into<String>,
    ) -> Result<&mut Self, FixtureError> {
        self.metadata.add_tag(tag)?;
        Ok(self)
    }

    /// Inserts a fixture metadata attribute.
    ///
    /// # Errors
    ///
    /// Returns an error if the attribute key is invalid.
    pub fn insert_attribute(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<&mut Self, FixtureError> {
        self.metadata.insert_attribute(key, value)?;
        Ok(self)
    }

    /// Builds the final immutable fixture.
    ///
    /// A fixture must contain at least one source file.
    ///
    /// A primary source file is optional because some tests operate
    /// on multi-file project structure rather than one entry point.
    ///
    /// # Errors
    ///
    /// Returns an error if no source files have been added.
    pub fn build(self) -> Result<Fixture, FixtureError> {
        if self.files.is_empty() {
            return Err(FixtureError::EmptyFixture);
        }

        Ok(Fixture {
            id: self.id,
            files: self.files,
            primary_file: self.primary_file,
            metadata: self.metadata,
        })
    }
}

// =============================================================================
// Fixture construction errors
// =============================================================================

/// Errors encountered while creating or validating fixtures.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FixtureError {
    /// A fixture identifier is empty.
    EmptyIdentifier,

    /// A qualified identifier contains an empty component.
    EmptyIdentifierComponent,

    /// An identifier contains surrounding whitespace.
    SurroundingWhitespace,

    /// An identifier contains unsupported characters.
    InvalidIdentifierCharacter,

    /// A virtual path is empty.
    EmptyPath,

    /// A path is absolute or contains a platform-specific prefix.
    AbsolutePath,

    /// A path contains parent-directory traversal.
    PathTraversal,

    /// A path contains an invalid separator.
    InvalidPathSeparator,

    /// A path contains invalid characters.
    InvalidPathCharacter,

    /// A path contains an empty component.
    EmptyPathComponent,

    /// A fixture contains a duplicate virtual file path.
    DuplicatePath(FixturePath),

    /// The selected primary source does not exist.
    PrimaryFileNotFound(FixturePath),

    /// A fixture has no source files.
    EmptyFixture,
}

impl fmt::Display for FixtureError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyIdentifier => {
                formatter.write_str("fixture identifier cannot be empty")
            }
            Self::EmptyIdentifierComponent => {
                formatter.write_str(
                    "fixture identifier contains an empty component",
                )
            }
            Self::SurroundingWhitespace => {
                formatter.write_str(
                    "fixture identifier contains surrounding whitespace",
                )
            }
            Self::InvalidIdentifierCharacter => {
                formatter.write_str(
                    "fixture identifier contains an unsupported character",
                )
            }
            Self::EmptyPath => {
                formatter.write_str("fixture path cannot be empty")
            }
            Self::AbsolutePath => {
                formatter.write_str(
                    "absolute and platform-prefixed fixture paths are forbidden",
                )
            }
            Self::PathTraversal => {
                formatter.write_str(
                    "fixture paths cannot contain traversal components",
                )
            }
            Self::InvalidPathSeparator => {
                formatter.write_str(
                    "fixture paths must use portable forward slashes",
                )
            }
            Self::InvalidPathCharacter => {
                formatter.write_str(
                    "fixture path contains an invalid control character",
                )
            }
            Self::EmptyPathComponent => {
                formatter.write_str(
                    "fixture path contains an empty component",
                )
            }
            Self::DuplicatePath(path) => {
                write!(formatter, "duplicate fixture path: {path}")
            }
            Self::PrimaryFileNotFound(path) => {
                write!(
                    formatter,
                    "primary fixture source does not exist: {path}"
                )
            }
            Self::EmptyFixture => {
                formatter.write_str(
                    "a fixture must contain at least one source file",
                )
            }
        }
    }
}

impl Error for FixtureError {}

// =============================================================================
// Convenience constructors
// =============================================================================

/// Creates a fixture containing one source file.
///
/// The supplied file becomes the primary source.
///
/// # Errors
///
/// Returns an error if the fixture identifier or source path is invalid.
pub fn single_source(
    id: impl Into<String>,
    path: impl Into<String>,
    source: impl Into<Arc<str>>,
) -> Result<Fixture, FixtureError> {
    let mut builder = FixtureBuilder::named(id)?;

    builder.add_primary_file(path, source)?;

    builder.build()
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_valid_fixture_identifier() {
        let id = FixtureId::new("classical.numeric")
            .expect("valid identifier");

        assert_eq!(id.as_str(), "classical.numeric");
    }

    #[test]
    fn rejects_empty_identifier() {
        assert_eq!(
            FixtureId::new(""),
            Err(FixtureError::EmptyIdentifier)
        );
    }

    #[test]
    fn rejects_surrounding_identifier_whitespace() {
        assert_eq!(
            FixtureId::new(" classical.numeric"),
            Err(FixtureError::SurroundingWhitespace)
        );
    }

    #[test]
    fn rejects_empty_identifier_components() {
        assert_eq!(
            FixtureId::new("classical..numeric"),
            Err(FixtureError::EmptyIdentifierComponent)
        );
    }

    #[test]
    fn accepts_portable_nested_paths() {
        let path = FixturePath::new(
            "modules/classical/math.zm"
        )
        .expect("valid path");

        assert_eq!(
            path.as_str(),
            "modules/classical/math.zm"
        );

        assert_eq!(
            path.file_name(),
            "math.zm"
        );

        assert_eq!(
            path.parent(),
            Some("modules/classical")
        );
    }

    #[test]
    fn rejects_absolute_paths() {
        assert_eq!(
            FixturePath::new("/main.zm"),
            Err(FixtureError::AbsolutePath)
        );
    }

    #[test]
    fn rejects_parent_traversal() {
        assert_eq!(
            FixturePath::new("src/../main.zm"),
            Err(FixtureError::PathTraversal)
        );
    }

    #[test]
    fn rejects_current_directory_components() {
        assert_eq!(
            FixturePath::new("src/./main.zm"),
            Err(FixtureError::PathTraversal)
        );
    }

    #[test]
    fn rejects_windows_separators() {
        assert_eq!(
            FixturePath::new("src\\main.zm"),
            Err(FixtureError::InvalidPathSeparator)
        );
    }

    #[test]
    fn rejects_empty_path_components() {
        assert_eq!(
            FixturePath::new("src//main.zm"),
            Err(FixtureError::EmptyPathComponent)
        );
    }

    #[test]
    fn rejects_platform_drive_prefixes() {
        assert_eq!(
            FixturePath::new("C:/main.zm"),
            Err(FixtureError::AbsolutePath)
        );
    }

    #[test]
    fn accepts_empty_source_files() {
        let file = FixtureFile::new("empty.zm", "")
            .expect("valid empty source");

        assert!(file.is_empty());
        assert_eq!(file.byte_len(), 0);
    }

    #[test]
    fn creates_single_source_fixture() {
        let fixture = single_source(
            "classical.basic",
            "main.zm",
            "let value = 42;",
        )
        .expect("valid fixture");

        assert_eq!(fixture.id().as_str(), "classical.basic");
        assert_eq!(fixture.file_count(), 1);

        let primary = fixture
            .primary_file()
            .expect("primary source");

        assert_eq!(primary.path().as_str(), "main.zm");
        assert_eq!(primary.contents(), "let value = 42;");
    }

    #[test]
    fn creates_multi_file_fixture() {
        let mut builder = FixtureBuilder::named(
            "classical.project",
        )
        .expect("valid identifier");

        builder
            .add_primary_file(
                "src/main.zm",
                "import math;",
            )
            .expect("primary source");

        builder
            .add_file(
                "src/math.zm",
                "fn add(a, b) { a + b }",
            )
            .expect("module source");

        let fixture = builder
            .build()
            .expect("valid fixture");

        assert_eq!(fixture.file_count(), 2);

        assert!(fixture.file("src/main.zm").is_some());
        assert!(fixture.file("src/math.zm").is_some());
    }

    #[test]
    fn rejects_duplicate_file_paths() {
        let mut builder = FixtureBuilder::named(
            "classical.duplicate",
        )
        .expect("valid identifier");

        builder
            .add_file("main.zm", "first")
            .expect("first file");

        assert_eq!(
            builder.add_file("main.zm", "second"),
            Err(FixtureError::DuplicatePath(
                FixturePath::new("main.zm").unwrap()
            ))
        );
    }

    #[test]
    fn rejects_missing_primary_file() {
        let mut builder = FixtureBuilder::named(
            "classical.missing",
        )
        .expect("valid identifier");

        builder
            .add_file("main.zm", "source")
            .expect("source");

        assert_eq!(
            builder.set_primary_file("missing.zm"),
            Err(FixtureError::PrimaryFileNotFound(
                FixturePath::new("missing.zm").unwrap()
            ))
        );
    }

    #[test]
    fn requires_at_least_one_file() {
        let builder = FixtureBuilder::named(
            "classical.empty",
        )
        .expect("valid identifier");

        assert_eq!(
            builder.build(),
            Err(FixtureError::EmptyFixture)
        );
    }

    #[test]
    fn fixture_iteration_is_deterministic() {
        let mut builder = FixtureBuilder::named(
            "classical.order",
        )
        .expect("valid identifier");

        builder.add_file("z.zm", "z").unwrap();
        builder.add_file("a.zm", "a").unwrap();
        builder.add_file("m.zm", "m").unwrap();

        let fixture = builder.build().unwrap();

        let paths: Vec<&str> = fixture
            .files()
            .map(|file| file.path().as_str())
            .collect();

        assert_eq!(
            paths,
            vec!["a.zm", "m.zm", "z.zm"]
        );
    }

    #[test]
    fn calculates_total_source_size() {
        let mut builder = FixtureBuilder::named(
            "classical.size",
        )
        .unwrap();

        builder.add_file("a.zm", "123").unwrap();
        builder.add_file("b.zm", "4567").unwrap();

        let fixture = builder.build().unwrap();

        assert_eq!(fixture.total_source_bytes(), Some(7));
    }

    #[test]
    fn counts_file_extensions() {
        let mut builder = FixtureBuilder::named(
            "classical.extensions",
        )
        .unwrap();

        builder.add_file("main.zm", "").unwrap();
        builder.add_file("module.zm", "").unwrap();
        builder.add_file("readme.md", "").unwrap();

        let fixture = builder.build().unwrap();

        assert_eq!(fixture.count_extension("zm"), 2);
        assert_eq!(fixture.count_extension("md"), 1);
        assert_eq!(fixture.count_extension("rs"), 0);
    }

    #[test]
    fn fixture_metadata_is_deterministic() {
        let mut metadata = FixtureMetadata::new();

        metadata.add_tag("classical").unwrap();
        metadata.add_tag("scalability").unwrap();
        metadata.add_tag("classical").unwrap();

        metadata
            .insert_attribute("category", "numeric")
            .unwrap();

        assert!(metadata.has_tag("classical"));
        assert!(metadata.has_tag("scalability"));

        assert_eq!(
            metadata.attribute("category"),
            Some("numeric")
        );

        assert_eq!(
            metadata.tags().collect::<Vec<_>>(),
            vec!["classical", "scalability"]
        );
    }

    #[test]
    fn metadata_rejects_invalid_tags() {
        let mut metadata = FixtureMetadata::new();

        assert_eq!(
            metadata.add_tag("invalid tag"),
            Err(FixtureError::InvalidIdentifierCharacter)
        );
    }

    #[test]
    fn metadata_replaces_existing_attributes() {
        let mut metadata = FixtureMetadata::new();

        metadata
            .insert_attribute("category", "classical")
            .unwrap();

        let previous = metadata
            .insert_attribute("category", "quantum")
            .unwrap();

        assert_eq!(previous, Some("classical".to_owned()));
        assert_eq!(
            metadata.attribute("category"),
            Some("quantum")
        );
    }

    #[test]
    fn fixtures_are_cloneable_without_copying_source_buffers() {
        let fixture = single_source(
            "classical.clone",
            "main.zm",
            "let value = 42;",
        )
        .unwrap();

        let cloned = fixture.clone();

        assert_eq!(fixture, cloned);

        let original_source = &fixture.primary_file().unwrap().contents;
        let cloned_source = &cloned.primary_file().unwrap().contents;

        assert!(Arc::ptr_eq(
            original_source,
            cloned_source
        ));
    }

    #[test]
    fn accepts_unicode_source_contents() {
        let fixture = single_source(
            "classical.unicode",
            "main.zm",
            "let message = \"Zamani 世界\";",
        )
        .unwrap();

        assert!(fixture
            .primary_file()
            .unwrap()
            .contents()
            .contains("世界"));
    }

    #[test]
    fn metadata_description_is_preserved() {
        let mut builder = FixtureBuilder::named(
            "classical.description",
        )
        .unwrap();

        builder
            .set_description("A basic arithmetic fixture")
            .add_primary_file("main.zm", "let value = 42;")
            .unwrap();

        let fixture = builder.build().unwrap();

        assert_eq!(
            fixture.metadata().description(),
            Some("A basic arithmetic fixture")
        );
    }
}

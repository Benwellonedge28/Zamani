//! Zamani Compiler — Production Incremental Compilation
//!
//! Provides deterministic incremental-compilation state management:
//!
//! - content-hash tracking
//! - dependency graph management
//! - reverse-dependency invalidation
//! - transitive dirty propagation
//! - deterministic compilation planning
//! - dependency-first ordering
//! - cycle detection
//! - clean-state management
//! - removed-file handling
//! - stable FNV-1a hashing
//!
//! This module deliberately does not perform compilation itself. It produces
//! the set and order of compilation units that the canonical compiler pipeline
//! in `crate::compiler` should compile.
//!
//! Design:
//!
//! ```text
//! source changes
//!      |
//!      v
//! content hash comparison
//!      |
//!      v
//! dependency invalidation
//!      |
//!      v
//! transitive dirty propagation
//!      |
//!      v
//! deterministic dependency-first plan
//!      |
//!      v
//! crate::compiler compilation pipeline
//! ```
//!
//! The implementation is intentionally independent of the frontend, parser,
//! IR generator, optimizer, and backend so that incremental compilation does
//! not duplicate the canonical compiler pipeline.

use std::collections::{HashMap, HashSet};
use std::fmt;
use std::path::{Path, PathBuf};

// -----------------------------------------------------------------------------
// Constants
// -----------------------------------------------------------------------------

/// Offset basis for the 64-bit FNV-1a hash.
const FNV1A_OFFSET_BASIS: u64 = 0xcbf29ce484222325;

/// Prime for the 64-bit FNV-1a hash.
const FNV1A_PRIME: u64 = 0x100000001b3;

// -----------------------------------------------------------------------------
// Errors
// -----------------------------------------------------------------------------

/// Errors produced by the incremental compilation subsystem.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IncrementalError {
    /// A source path was empty.
    EmptyPath,

    /// A dependency graph contained a dependency cycle.
    DependencyCycle(Vec<PathBuf>),

    /// A path was used as a dependency but has invalid state.
    InvalidDependency {
        file: PathBuf,
        dependency: PathBuf,
    },
}

impl fmt::Display for IncrementalError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyPath => {
                write!(formatter, "incremental compilation: path cannot be empty")
            }

            Self::DependencyCycle(paths) => {
                write!(
                    formatter,
                    "incremental compilation: dependency cycle detected: "
                )?;

                for (index, path) in paths.iter().enumerate() {
                    if index > 0 {
                        write!(formatter, " -> ")?;
                    }

                    write!(formatter, "{}", path.display())?;
                }

                Ok(())
            }

            Self::InvalidDependency { file, dependency } => {
                write!(
                    formatter,
                    "incremental compilation: file '{}' references invalid dependency '{}'",
                    file.display(),
                    dependency.display()
                )
            }
        }
    }
}

impl std::error::Error for IncrementalError {}

// -----------------------------------------------------------------------------
// File state
// -----------------------------------------------------------------------------

/// Persistent state associated with one compilation unit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileState {
    /// Last known content hash.
    pub hash: u64,

    /// Files directly imported/referenced by this file.
    pub dependencies: HashSet<PathBuf>,

    /// Whether the file currently requires recompilation.
    pub dirty: bool,
}

impl FileState {
    fn new(hash: u64, dependencies: HashSet<PathBuf>, dirty: bool) -> Self {
        Self {
            hash,
            dependencies,
            dirty,
        }
    }
}

// -----------------------------------------------------------------------------
// Compilation plan
// -----------------------------------------------------------------------------

/// Deterministic incremental compilation plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompilationPlan {
    /// Files that must be compiled.
    ///
    /// Dependencies always appear before dependents whenever the graph is
    /// acyclic.
    pub files: Vec<PathBuf>,
}

impl CompilationPlan {
    /// Creates an empty plan.
    pub fn empty() -> Self {
        Self { files: Vec::new() }
    }

    /// Returns whether the plan contains no compilation work.
    pub fn is_empty(&self) -> bool {
        self.files.is_empty()
    }

    /// Returns the number of compilation units.
    pub fn len(&self) -> usize {
        self.files.len()
    }
}

// -----------------------------------------------------------------------------
// Incremental compiler
// -----------------------------------------------------------------------------

/// Production incremental compilation state.
///
/// The structure maintains both forward and reverse dependency graphs:
///
/// ```text
/// dependencies:
///
/// A -> B
/// A -> C
///
/// reverse_dependencies:
///
/// B -> A
/// C -> A
/// ```
///
/// The reverse graph allows a changed dependency to invalidate every dependent
/// transitively without scanning the entire source graph for each change.
#[derive(Debug, Clone)]
pub struct IncrementalCompiler {
    /// Persistent source-file state.
    files: HashMap<PathBuf, FileState>,

    /// Reverse dependency graph.
    ///
    /// `reverse_dependencies[A]` contains every file that directly depends on
    /// `A`.
    reverse_dependencies: HashMap<PathBuf, HashSet<PathBuf>>,

    /// Files explicitly marked dirty.
    dirty_files: HashSet<PathBuf>,
}

impl Default for IncrementalCompiler {
    fn default() -> Self {
        Self::new()
    }
}

impl IncrementalCompiler {
    /// Creates an empty incremental compiler state.
    pub fn new() -> Self {
        Self {
            files: HashMap::new(),
            reverse_dependencies: HashMap::new(),
            dirty_files: HashSet::new(),
        }
    }

    // -------------------------------------------------------------------------
    // Recording state
    // -------------------------------------------------------------------------

    /// Records the current state of a source file.
    ///
    /// A file becomes dirty when:
    ///
    /// - it has never been seen before;
    /// - its content hash changed; or
    /// - its dependency set changed.
    ///
    /// Changing a dependency invalidates the file and all of its dependents.
    pub fn record_file(
        &mut self,
        path: impl Into<PathBuf>,
        hash: u64,
        deps: Vec<PathBuf>,
    ) {
        let path = path.into();

        if path.as_os_str().is_empty() {
            return;
        }

        let dependencies: HashSet<PathBuf> = deps
            .into_iter()
            .filter(|dependency| !dependency.as_os_str().is_empty())
            .collect();

        let old_state = self.files.get(&path).cloned();

        let hash_changed = old_state
            .as_ref()
            .map(|state| state.hash != hash)
            .unwrap_or(true);

        let dependencies_changed = old_state
            .as_ref()
            .map(|state| state.dependencies != dependencies)
            .unwrap_or(true);

        /*
         * Remove the old reverse-dependency edges before inserting the new
         * dependency set.
         */
        if let Some(state) = old_state.as_ref() {
            for dependency in &state.dependencies {
                if let Some(dependents) = self.reverse_dependencies.get_mut(dependency) {
                    dependents.remove(&path);

                    if dependents.is_empty() {
                        self.reverse_dependencies.remove(dependency);
                    }
                }
            }
        }

        /*
         * Install the new reverse edges.
         */
        for dependency in &dependencies {
            self.reverse_dependencies
                .entry(dependency.clone())
                .or_default()
                .insert(path.clone());
        }

        let was_dirty = old_state
            .as_ref()
            .map(|state| state.dirty)
            .unwrap_or(false);

        let dirty = was_dirty || hash_changed || dependencies_changed;

        self.files.insert(
            path.clone(),
            FileState::new(hash, dependencies, dirty),
        );

        if dirty {
            self.mark_dirty_and_dependents(&path);
        } else {
            self.dirty_files.remove(&path);
        }
    }

    /// Records a source file using its content.
    ///
    /// Uses deterministic FNV-1a hashing rather than `DefaultHasher`, because
    /// compiler cache keys must not depend on implementation-specific hasher
    /// seeding.
    pub fn record_source(
        &mut self,
        path: impl Into<PathBuf>,
        source: &[u8],
        deps: Vec<PathBuf>,
    ) {
        let hash = stable_hash(source);
        self.record_file(path, hash, deps);
    }

    /// Records a source file from a string.
    pub fn record_source_text(
        &mut self,
        path: impl Into<PathBuf>,
        source: &str,
        deps: Vec<PathBuf>,
    ) {
        self.record_source(path, source.as_bytes(), deps);
    }

    // -------------------------------------------------------------------------
    // Dirty state
    // -------------------------------------------------------------------------

    /// Returns whether a file is dirty.
    pub fn check_dirty(&self, path: &PathBuf) -> bool {
        self.dirty_files.contains(path)
    }

    /// Returns whether a path is known to the incremental compiler.
    pub fn contains(&self, path: &Path) -> bool {
        self.files.contains_key(path)
    }

    /// Returns the current state of a file.
    pub fn file_state(&self, path: &Path) -> Option<&FileState> {
        self.files.get(path)
    }

    /// Marks a successfully compiled file as clean.
    ///
    /// This must be called only after the canonical compiler pipeline has
    /// successfully compiled the file and its resulting artifact/cache entry
    /// has been committed.
    pub fn mark_clean(&mut self, path: &Path) -> bool {
        let Some(state) = self.files.get_mut(path) else {
            return false;
        };

        state.dirty = false;
        self.dirty_files.remove(path);

        true
    }

    /// Marks a file dirty and propagates invalidation to all dependents.
    pub fn mark_dirty(&mut self, path: &Path) -> bool {
        if !self.files.contains_key(path) {
            return false;
        }

        self.mark_dirty_and_dependents(path);
        true
    }

    /// Marks every known file dirty.
    pub fn mark_all_dirty(&mut self) {
        for state in self.files.values_mut() {
            state.dirty = true;
        }

        self.dirty_files
            .extend(self.files.keys().cloned());
    }

    /// Clears all dirty state.
    ///
    /// This is useful after a complete successful rebuild.
    pub fn mark_all_clean(&mut self) {
        for state in self.files.values_mut() {
            state.dirty = false;
        }

        self.dirty_files.clear();
    }

    // -------------------------------------------------------------------------
    // Dependency management
    // -------------------------------------------------------------------------

    /// Returns direct dependencies of a file.
    pub fn dependencies(&self, path: &Path) -> Option<Vec<PathBuf>> {
        self.files.get(path).map(|state| {
            let mut dependencies: Vec<_> =
                state.dependencies.iter().cloned().collect();

            dependencies.sort();
            dependencies
        })
    }

    /// Returns direct dependents of a file.
    pub fn dependents(&self, path: &Path) -> Vec<PathBuf> {
        let mut dependents = self
            .reverse_dependencies
            .get(path)
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .collect::<Vec<_>>();

        dependents.sort();
        dependents
    }

    /// Removes a file from incremental state.
    ///
    /// Removing a file invalidates every file that depended on it.
    pub fn remove_file(&mut self, path: &Path) -> bool {
        let Some(state) = self.files.remove(path) else {
            return false;
        };

        self.dirty_files.remove(path);

        for dependency in &state.dependencies {
            if let Some(dependents) = self.reverse_dependencies.get_mut(dependency) {
                dependents.remove(path);

                if dependents.is_empty() {
                    self.reverse_dependencies.remove(dependency);
                }
            }
        }

        /*
         * The removed file may have dependents. Those dependents must be
         * invalidated because one of their compilation inputs disappeared.
         */
        let dependents = self
            .reverse_dependencies
            .remove(path)
            .unwrap_or_default();

        for dependent in dependents {
            self.mark_dirty_and_dependents(&dependent);
        }

        true
    }

    /// Returns the number of tracked files.
    pub fn file_count(&self) -> usize {
        self.files.len()
    }

    /// Returns the number of dirty files.
    pub fn dirty_count(&self) -> usize {
        self.dirty_files.len()
    }

    // -------------------------------------------------------------------------
    // Compilation planning
    // -------------------------------------------------------------------------

    /// Computes a deterministic compilation plan.
    ///
    /// The resulting order is dependency-first:
    ///
    /// ```text
    /// dependency -> dependent
    /// ```
    ///
    /// This is essential for incremental compilation because a dependent
    /// cannot safely be rebuilt before its changed inputs have been rebuilt.
    pub fn get_compilation_plan(&self) -> Vec<PathBuf> {
        self.compilation_plan()
            .map(|plan| plan.files)
            .unwrap_or_default()
    }

    /// Computes a validated compilation plan.
    ///
    /// Unlike `get_compilation_plan`, this method exposes dependency cycles as
    /// structured errors rather than silently returning an arbitrary order.
    pub fn compilation_plan(&self) -> Result<CompilationPlan, IncrementalError> {
        if self.dirty_files.is_empty() {
            return Ok(CompilationPlan::empty());
        }

        let mut dirty: Vec<PathBuf> =
            self.dirty_files.iter().cloned().collect();

        dirty.sort();

        let dirty_set: HashSet<PathBuf> =
            dirty.iter().cloned().collect();

        let mut state = HashMap::<PathBuf, VisitState>::new();
        let mut ordered = Vec::with_capacity(dirty.len());
        let mut stack = Vec::<PathBuf>::new();

        for file in &dirty {
            self.visit_for_plan(
                file,
                &dirty_set,
                &mut state,
                &mut ordered,
                &mut stack,
            )?;
        }

        Ok(CompilationPlan { files: ordered })
    }

    /// Validates the complete dependency graph.
    pub fn validate(&self) -> Result<(), IncrementalError> {
        let all_files: HashSet<PathBuf> =
            self.files.keys().cloned().collect();

        for (file, state) in &self.files {
            for dependency in &state.dependencies {
                /*
                 * Dependencies that have not yet been recorded are permitted.
                 * This is necessary for source-discovery workflows where
                 * dependencies are discovered before their own compilation
                 * state is registered.
                 */
                if dependency == file {
                    return Err(IncrementalError::DependencyCycle(vec![
                        file.clone(),
                        file.clone(),
                    ]));
                }

                if all_files.contains(dependency) {
                    continue;
                }
            }
        }

        let mut state = HashMap::<PathBuf, VisitState>::new();
        let mut ordered = Vec::new();
        let mut stack = Vec::new();

        let mut files: Vec<PathBuf> =
            self.files.keys().cloned().collect();

        files.sort();

        let all: HashSet<PathBuf> =
            files.iter().cloned().collect();

        for file in &files {
            self.visit_for_plan(
                file,
                &all,
                &mut state,
                &mut ordered,
                &mut stack,
            )?;
        }

        Ok(())
    }

    // -------------------------------------------------------------------------
    // Internal invalidation
    // -------------------------------------------------------------------------

    fn mark_dirty_and_dependents(&mut self, path: &Path) {
        let mut queue = vec![path.to_path_buf()];
        let mut visited = HashSet::new();

        while let Some(current) = queue.pop() {
            if !visited.insert(current.clone()) {
                continue;
            }

            if let Some(state) = self.files.get_mut(&current) {
                state.dirty = true;
                self.dirty_files.insert(current.clone());
            }

            if let Some(dependents) =
                self.reverse_dependencies.get(&current)
            {
                queue.extend(dependents.iter().cloned());
            }
        }
    }

    // -------------------------------------------------------------------------
    // Internal dependency traversal
    // -------------------------------------------------------------------------

    fn visit_for_plan(
        &self,
        file: &Path,
        relevant_files: &HashSet<PathBuf>,
        state: &mut HashMap<PathBuf, VisitState>,
        ordered: &mut Vec<PathBuf>,
        stack: &mut Vec<PathBuf>,
    ) -> Result<(), IncrementalError> {
        match state.get(file).copied() {
            Some(VisitState::Visited) => return Ok(()),

            Some(VisitState::Visiting) => {
                let cycle_start =
                    stack.iter().position(|path| path == file).unwrap_or(0);

                let mut cycle =
                    stack[cycle_start..].to_vec();

                cycle.push(file.to_path_buf());

                return Err(IncrementalError::DependencyCycle(cycle));
            }

            None => {}
        }

        state.insert(file.to_path_buf(), VisitState::Visiting);
        stack.push(file.to_path_buf());

        if let Some(file_state) = self.files.get(file) {
            let mut dependencies: Vec<PathBuf> =
                file_state.dependencies.iter().cloned().collect();

            dependencies.sort();

            for dependency in dependencies {
                /*
                 * Only dirty files need to be included in the plan. A clean
                 * dependency is already assumed to have a valid artifact.
                 *
                 * Unknown dependencies are deliberately ignored here; the
                 * source-discovery layer can register them before compiling
                 * them.
                 */
                if relevant_files.contains(&dependency) {
                    self.visit_for_plan(
                        &dependency,
                        relevant_files,
                        state,
                        ordered,
                        stack,
                    )?;
                }
            }
        }

        stack.pop();

        state.insert(file.to_path_buf(), VisitState::Visited);

        if relevant_files.contains(file) {
            ordered.push(file.to_path_buf());
        }

        Ok(())
    }
}

// -----------------------------------------------------------------------------
// Traversal state
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum VisitState {
    Visiting,
    Visited,
}

// -----------------------------------------------------------------------------
// Hashing
// -----------------------------------------------------------------------------

/// Computes a deterministic 64-bit FNV-1a hash.
pub fn stable_hash(data: &[u8]) -> u64 {
    let mut hash = FNV1A_OFFSET_BASIS;

    for byte in data {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(FNV1A_PRIME);
    }

    hash
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn path(name: &str) -> PathBuf {
        PathBuf::from(name)
    }

    #[test]
    fn new_compiler_is_empty() {
        let compiler = IncrementalCompiler::new();

        assert_eq!(compiler.file_count(), 0);
        assert_eq!(compiler.dirty_count(), 0);
        assert!(compiler.get_compilation_plan().is_empty());
    }

    #[test]
    fn first_record_is_dirty() {
        let mut compiler = IncrementalCompiler::new();

        compiler.record_file(
            path("main.zm"),
            1,
            Vec::new(),
        );

        assert!(compiler.check_dirty(&path("main.zm")));
        assert_eq!(
            compiler.get_compilation_plan(),
            vec![path("main.zm")]
        );
    }

    #[test]
    fn unchanged_file_does_not_become_dirty_after_clean_build() {
        let mut compiler = IncrementalCompiler::new();

        compiler.record_file(
            path("main.zm"),
            1,
            Vec::new(),
        );

        compiler.mark_clean(&path("main.zm"));

        compiler.record_file(
            path("main.zm"),
            1,
            Vec::new(),
        );

        assert!(!compiler.check_dirty(&path("main.zm")));
        assert!(compiler.get_compilation_plan().is_empty());
    }

    #[test]
    fn changed_hash_marks_file_dirty() {
        let mut compiler = IncrementalCompiler::new();

        compiler.record_file(
            path("main.zm"),
            1,
            Vec::new(),
        );

        compiler.mark_clean(&path("main.zm"));

        compiler.record_file(
            path("main.zm"),
            2,
            Vec::new(),
        );

        assert!(compiler.check_dirty(&path("main.zm")));
    }

    #[test]
    fn dependency_change_invalidates_dependent() {
        let mut compiler = IncrementalCompiler::new();

        compiler.record_file(
            path("lib.zm"),
            1,
            Vec::new(),
        );

        compiler.record_file(
            path("main.zm"),
            1,
            vec![path("lib.zm")],
        );

        compiler.mark_all_clean();

        compiler.record_file(
            path("lib.zm"),
            2,
            Vec::new(),
        );

        assert!(compiler.check_dirty(&path("lib.zm")));
        assert!(compiler.check_dirty(&path("main.zm")));
    }

    #[test]
    fn transitive_dependency_invalidation_works() {
        let mut compiler = IncrementalCompiler::new();

        compiler.record_file(
            path("a.zm"),
            1,
            Vec::new(),
        );

        compiler.record_file(
            path("b.zm"),
            1,
            vec![path("a.zm")],
        );

        compiler.record_file(
            path("c.zm"),
            1,
            vec![path("b.zm")],
        );

        compiler.mark_all_clean();

        compiler.record_file(
            path("a.zm"),
            2,
            Vec::new(),
        );

        assert!(compiler.check_dirty(&path("a.zm")));
        assert!(compiler.check_dirty(&path("b.zm")));
        assert!(compiler.check_dirty(&path("c.zm")));
    }

    #[test]
    fn compilation_plan_is_dependency_first() {
        let mut compiler = IncrementalCompiler::new();

        compiler.record_file(
            path("a.zm"),
            1,
            Vec::new(),
        );

        compiler.record_file(
            path("b.zm"),
            1,
            vec![path("a.zm")],
        );

        compiler.record_file(
            path("c.zm"),
            1,
            vec![path("b.zm")],
        );

        let plan = compiler.compilation_plan().unwrap();

        assert_eq!(
            plan.files,
            vec![
                path("a.zm"),
                path("b.zm"),
                path("c.zm")
            ]
        );
    }

    #[test]
    fn compilation_plan_is_deterministic() {
        let mut compiler = IncrementalCompiler::new();

        compiler.record_file(
            path("z.zm"),
            1,
            Vec::new(),
        );

        compiler.record_file(
            path("a.zm"),
            1,
            Vec::new(),
        );

        compiler.record_file(
            path("m.zm"),
            1,
            Vec::new(),
        );

        assert_eq!(
            compiler.get_compilation_plan(),
            vec![
                path("a.zm"),
                path("m.zm"),
                path("z.zm")
            ]
        );
    }

    #[test]
    fn mark_clean_removes_file_from_plan() {
        let mut compiler = IncrementalCompiler::new();

        compiler.record_file(
            path("main.zm"),
            1,
            Vec::new(),
        );

        assert!(compiler.mark_clean(&path("main.zm")));
        assert!(!compiler.check_dirty(&path("main.zm")));
        assert!(compiler.get_compilation_plan().is_empty());
    }

    #[test]
    fn mark_clean_unknown_file_returns_false() {
        let mut compiler = IncrementalCompiler::new();

        assert!(!compiler.mark_clean(&path("unknown.zm")));
    }

    #[test]
    fn mark_all_clean_clears_everything() {
        let mut compiler = IncrementalCompiler::new();

        compiler.record_file(path("a.zm"), 1, Vec::new());
        compiler.record_file(path("b.zm"), 1, Vec::new());

        compiler.mark_all_clean();

        assert_eq!(compiler.dirty_count(), 0);
        assert!(compiler.get_compilation_plan().is_empty());
    }

    #[test]
    fn mark_all_dirty_marks_everything() {
        let mut compiler = IncrementalCompiler::new();

        compiler.record_file(path("a.zm"), 1, Vec::new());
        compiler.record_file(path("b.zm"), 1, Vec::new());

        compiler.mark_all_clean();
        compiler.mark_all_dirty();

        assert_eq!(compiler.dirty_count(), 2);
    }

    #[test]
    fn dependents_are_sorted() {
        let mut compiler = IncrementalCompiler::new();

        compiler.record_file(path("root.zm"), 1, Vec::new());
        compiler.record_file(
            path("z.zm"),
            1,
            vec![path("root.zm")],
        );
        compiler.record_file(
            path("a.zm"),
            1,
            vec![path("root.zm")],
        );

        assert_eq!(
            compiler.dependents(&path("root.zm")),
            vec![path("a.zm"), path("z.zm")]
        );
    }

    #[test]
    fn dependency_changes_mark_file_dirty() {
        let mut compiler = IncrementalCompiler::new();

        compiler.record_file(path("a.zm"), 1, Vec::new());
        compiler.record_file(path("b.zm"), 1, Vec::new());

        compiler.record_file(
            path("main.zm"),
            1,
            vec![path("a.zm")],
        );

        compiler.mark_all_clean();

        compiler.record_file(
            path("main.zm"),
            1,
            vec![path("b.zm")],
        );

        assert!(compiler.check_dirty(&path("main.zm")));
    }

    #[test]
    fn source_hash_is_stable() {
        assert_eq!(
            stable_hash(b"hello"),
            stable_hash(b"hello")
        );

        assert_ne!(
            stable_hash(b"hello"),
            stable_hash(b"world")
        );
    }

    #[test]
    fn source_recording_uses_content_hash() {
        let mut compiler = IncrementalCompiler::new();

        compiler.record_source_text(
            path("main.zm"),
            "let x = 1;",
            Vec::new(),
        );

        let first_hash = compiler
            .file_state(&path("main.zm"))
            .unwrap()
            .hash;

        compiler.mark_clean(&path("main.zm"));

        compiler.record_source_text(
            path("main.zm"),
            "let x = 1;",
            Vec::new(),
        );

        assert_eq!(
            first_hash,
            compiler
                .file_state(&path("main.zm"))
                .unwrap()
                .hash
        );

        assert!(!compiler.check_dirty(&path("main.zm")));
    }

    #[test]
    fn removed_file_invalidates_dependents() {
        let mut compiler = IncrementalCompiler::new();

        compiler.record_file(path("lib.zm"), 1, Vec::new());
        compiler.record_file(
            path("main.zm"),
            1,
            vec![path("lib.zm")],
        );

        compiler.mark_all_clean();

        assert!(compiler.remove_file(&path("lib.zm")));
        assert!(compiler.check_dirty(&path("main.zm")));
        assert!(!compiler.contains(&path("lib.zm")));
    }

    #[test]
    fn removing_unknown_file_returns_false() {
        let mut compiler = IncrementalCompiler::new();

        assert!(!compiler.remove_file(&path("missing.zm")));
    }

    #[test]
    fn self_dependency_is_rejected() {
        let mut compiler = IncrementalCompiler::new();

        compiler.record_file(
            path("main.zm"),
            1,
            vec![path("main.zm")],
        );

        assert!(matches!(
            compiler.validate(),
            Err(IncrementalError::DependencyCycle(_))
        ));
    }

    #[test]
    fn dependency_cycle_is_detected() {
        let mut compiler = IncrementalCompiler::new();

        compiler.record_file(
            path("a.zm"),
            1,
            vec![path("b.zm")],
        );

        compiler.record_file(
            path("b.zm"),
            1,
            vec![path("a.zm")],
        );

        assert!(matches!(
            compiler.validate(),
            Err(IncrementalError::DependencyCycle(_))
        ));
    }

    #[test]
    fn empty_plan_is_valid() {
        let compiler = IncrementalCompiler::new();

        let plan = compiler.compilation_plan().unwrap();

        assert!(plan.is_empty());
        assert_eq!(plan.len(), 0);
    }

    #[test]
    fn file_state_is_available() {
        let mut compiler = IncrementalCompiler::new();

        compiler.record_file(
            path("main.zm"),
            0x1234,
            vec![path("lib.zm")],
        );

        let state = compiler
            .file_state(&path("main.zm"))
            .unwrap();

        assert_eq!(state.hash, 0x1234);
        assert!(state.dirty);
        assert_eq!(
            state.dependencies,
            HashSet::from([path("lib.zm")])
        );
    }
}
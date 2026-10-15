
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Pass Registry
//!
//! Path: `src/zuir/pass/registry.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! PURPOSE
//!
//! Own the registration and lookup of transformation and analysis passes.
//! Validate pass descriptors, reject duplicate identities, preserve explicit
//! registration order, and expose the registered implementations to the
//! pass manager.
//!
//! AUTHORITY
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/pass/manager.rs`
//! - `grammar/DESIGN.md`
//! - `grammar/specification/compilation-model.md`
//! - `grammar/specification/extensibility.md`
//! - `grammar/specification/grammar-authority.md`
//!
//! OWNS
//!
//! - Pass registration and duplicate detection.
//! - Registry-local descriptor validation.
//! - Stable ID-to-registration-index lookup.
//! - Registration-order iteration.
//! - Access to registered pass implementations.
//!
//! DOES NOT OWN
//!
//! - Pass identity, descriptors, errors, or the `Pass` trait.
//! - Pass execution, scheduling, or pipeline budgets.
//! - IR verification or semantic-preservation proofs.
//! - Analysis result storage or invalidation algorithms.
//! - Dialect registration, source parsing, or target selection.
//! - Physical hardware allocation, routing, or runtime execution.
//!
//! INTEGRATION
//!
//! The canonical contracts currently reside in `manager.rs`. This module
//! imports and reuses them. `pass/mod.rs` must declare this module, and
//! `PassManager` must delegate registration and pass lookup to this registry.
//!
//! SCALABILITY
//!
//! Dynamic collections are used without an arbitrary pass-count ceiling.
//! Registration order is explicit and deterministic. Resource exhaustion
//! remains possible and must not be confused with a semantic program limit.
//!
//! SAFETY
//!
//! Unsafe Rust is forbidden.
//!
//! FREEZE CRITERIA
//!
//! - Invalid descriptors are rejected before mutation.
//! - Duplicate IDs never replace an existing pass.
//! - Registry indices remain consistent after successful registration.
//! - Registration order is stable.
//! - New computational domains can register passes without editing this file.
//! - Tests cover invalid descriptors, duplicate IDs, and deterministic order.

use std::collections::BTreeMap;

use super::manager::{
    Pass, PassDescriptor, PassError, PassErrorKind, PassId,
};

/// A pass implementation and its immutable registration-time descriptor.
///
/// The descriptor is captured when the pass is registered. This prevents
/// registry behavior from depending on a pass implementation returning
/// different metadata on later calls to `descriptor()`.
struct RegisteredPass<I> {
    descriptor: PassDescriptor,
    implementation: Box<dyn Pass<I>>,
}

/// Registry of compiler passes for one IR representation.
///
/// The registry is generic over the IR type. It does not assume that passes
/// operate only on classical instructions or that all passes target the same
/// computational domain.
///
/// # Ordering
///
/// Registration order is preserved and can be used as an explicit pipeline
/// order. The registry does not reorder passes by ID, domain, or hardware.
///
/// # Thread safety
///
/// The registry requires no global mutable state. Individual pass
/// implementations must satisfy the `Send` bound defined by `Pass<I>`.
///
/// # Example
///
/// ```ignore
/// let mut registry = PassRegistry::<MyIr>::new();
/// registry.register(MyPass::new())?;
/// let descriptor = registry.descriptor(&pass_id).expect("registered");
/// ```
pub struct PassRegistry<I> {
    entries: Vec<RegisteredPass<I>>,
    indices: BTreeMap<PassId, usize>,
}

impl<I> Default for PassRegistry<I> {
    fn default() -> Self {
        Self::new()
    }
}

impl<I> PassRegistry<I> {
    /// Create an empty registry.
    #[must_use]
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            indices: BTreeMap::new(),
        }
    }

    /// Return the number of registered passes.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Return whether the registry is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Check whether a pass ID is registered.
    #[must_use]
    pub fn contains(&self, id: &PassId) -> bool {
        self.indices.contains_key(id)
    }

    /// Iterate over registered IDs in deterministic lexical order.
    ///
    /// This method is intended for lookup, diagnostics, and inventory.
    /// It is **not** the pipeline execution order. Use `ordered_ids()` when
    /// registration order matters.
    pub fn registered_ids(&self) -> impl Iterator<Item = &PassId> {
        self.indices.keys()
    }

    /// Iterate over pass IDs in registration order.
    pub fn ordered_ids(&self) -> impl Iterator<Item = &PassId> {
        self.entries.iter().map(|entry| &entry.descriptor.id)
    }

    /// Iterate over immutable registration-time descriptors in execution
    /// order.
    pub fn descriptors(&self) -> impl Iterator<Item = &PassDescriptor> {
        self.entries.iter().map(|entry| &entry.descriptor)
    }

    /// Look up a registration-time descriptor by ID.
    #[must_use]
    pub fn descriptor(&self, id: &PassId) -> Option<&PassDescriptor> {
        let index = *self.indices.get(id)?;
        self.entries.get(index).map(|entry| &entry.descriptor)
    }

    /// Return the registration index for a pass.
    ///
    /// The index is valid for this registry's current lifetime and ordering.
    /// Persisted pipelines must store pass IDs and versions, never indices.
    #[must_use]
    pub fn registration_index(&self, id: &PassId) -> Option<usize> {
        self.indices.get(id).copied()
    }

    /// Register a pass after validating its descriptor.
    ///
    /// Registration is atomic with respect to logical registry state:
    /// validation failures and duplicate IDs do not modify the registry.
    ///
    /// Pass implementations should have stable descriptors. The registry
    /// captures the descriptor exactly once and exposes that snapshot to
    /// downstream pipeline management.
    pub fn register<P>(&mut self, pass: P) -> Result<(), PassError>
    where
        P: Pass<I> + 'static,
    {
        self.register_boxed(Box::new(pass))
    }

    /// Register an already boxed pass implementation.
    ///
    /// This is useful for plugin factories and heterogeneous pass collections.
    pub fn register_boxed(
        &mut self,
        pass: Box<dyn Pass<I>>,
    ) -> Result<(), PassError> {
        let descriptor = pass.descriptor();

        validate_descriptor(&descriptor)?;

        if self.indices.contains_key(&descriptor.id) {
            return Err(PassError::for_pass(
                PassErrorKind::DuplicatePass,
                descriptor.id,
                "a pass with this ID is already registered",
            ));
        }

        // Reserve vector capacity before changing the map. A reservation
        // failure is reported as a structured error rather than panicking
        // because of a capacity overflow. The allocator may still abort on
        // genuine process-wide allocation failure.
        self.entries.try_reserve(1).map_err(|error| {
            PassError::for_pass(
                PassErrorKind::InvalidConfiguration,
                descriptor.id.clone(),
                format!("unable to reserve registry capacity: {error}"),
            )
        })?;

        let index = self.entries.len();

        // A vector cannot practically exceed usize::MAX entries, but avoid
        // relying on wrapping arithmetic for registration identity.
        index.checked_add(1).ok_or_else(|| {
            PassError::for_pass(
                PassErrorKind::InvalidConfiguration,
                descriptor.id.clone(),
                "registry index space exhausted",
            )
        })?;

        // BTreeMap insertion has no stable fallible-reservation API. Insert
        // the index and then append the entry. The vector has already
        // reserved capacity, so ordinary append does not need to grow it.
        //
        // The duplicate check above guarantees this insert cannot replace a
        // valid registration in single-threaded exclusive access.
        self.indices.insert(descriptor.id.clone(), index);
        self.entries.push(RegisteredPass {
            descriptor,
            implementation: pass,
        });

        debug_assert_eq!(self.entries.len(), self.indices.len());
        Ok(())
    }

    /// Obtain an immutable pass implementation by ID.
    ///
    /// The implementation is exposed as a trait object so callers do not
    /// need to know its concrete type.
    #[must_use]
    pub fn get(&self, id: &PassId) -> Option<&(dyn Pass<I> + 'static)> {
        let index = *self.indices.get(id)?;
        self.entries
            .get(index)
            .map(|entry| entry.implementation.as_ref())
    }

    /// Obtain a mutable pass implementation by ID.
    ///
    /// Mutability is required for stateful passes implementing `Pass<I>`.
    /// This method does not change registration order or the captured
    /// registration-time descriptor.
    pub fn get_mut(
        &mut self,
        id: &PassId,
    ) -> Option<&mut (dyn Pass<I> + 'static)> {
        let index = *self.indices.get(id)?;
        match self.entries.get_mut(index) {
            Some(entry) => Some(entry.implementation.as_mut()),
            None => None,
        }
    }

    /// Obtain the immutable pass implementation at a registration index.
    #[must_use]
    pub fn get_at(
        &self,
        index: usize,
    ) -> Option<&(dyn Pass<I> + 'static)> {
        self.entries
            .get(index)
            .map(|entry| entry.implementation.as_ref())
    }

    /// Obtain the mutable pass implementation at a registration index.
    ///
    /// Prefer ID-based lookup outside pipeline execution. Index-based access
    /// is provided for a manager iterating in the registry's fixed order.
    pub fn get_at_mut(
        &mut self,
        index: usize,
    ) -> Option<&mut (dyn Pass<I> + 'static)> {
        match self.entries.get_mut(index) {
            Some(entry) => Some(entry.implementation.as_mut()),
            None => None,
        }
    }

    /// Return the immutable registration-time descriptor at an index.
    #[must_use]
    pub fn descriptor_at(&self, index: usize) -> Option<&PassDescriptor> {
        self.entries.get(index).map(|entry| &entry.descriptor)
    }

    /// Clear every registration.
    ///
    /// Clearing invalidates all registration indices. The registry does not
    /// attempt to interrupt a pass currently executing; callers must ensure
    /// exclusive access and coordinate lifecycle at the manager level.
    pub fn clear(&mut self) {
        self.entries.clear();
        self.indices.clear();
    }

    /// Validate the registry's internal index invariants.
    ///
    /// This is useful in tests, debug assertions, and trust boundaries after
    /// loading a pipeline manifest. It does not verify pass semantics or IR.
    pub fn validate(&self) -> Result<(), PassError> {
        if self.entries.len() != self.indices.len() {
            return Err(PassError::new(
                PassErrorKind::InvalidConfiguration,
                "registry entry and index counts differ",
            ));
        }

        for (index, entry) in self.entries.iter().enumerate() {
            match self.indices.get(&entry.descriptor.id) {
                Some(stored_index) if *stored_index == index => {}
                Some(stored_index) => {
                    return Err(PassError::for_pass(
                        PassErrorKind::InvalidConfiguration,
                        entry.descriptor.id.clone(),
                        format!(
                            "registry index mismatch: expected {index}, found {stored_index}"
                        ),
                    ));
                }
                None => {
                    return Err(PassError::for_pass(
                        PassErrorKind::InvalidConfiguration,
                        entry.descriptor.id.clone(),
                        "registered pass is missing from the ID index",
                    ));
                }
            }
        }

        Ok(())
    }
}

/// Validate a pass descriptor before it can enter the registry.
///
/// These checks cover registry-level metadata only. A pass's semantic
/// preconditions, required dialects, IR invariants, and preservation claims
/// remain the responsibility of the pass contract and manager/verifier.
fn validate_descriptor(descriptor: &PassDescriptor) -> Result<(), PassError> {
    let id = &descriptor.id;
    let name = id.as_str();

    if !is_valid_pass_id(name) {
        return Err(PassError::for_pass(
            PassErrorKind::InvalidPassId,
            id.clone(),
            "pass ID must be a qualified name with non-empty, valid segments",
        ));
    }

    if descriptor.description.trim().is_empty() {
        return Err(PassError::for_pass(
            PassErrorKind::InvalidConfiguration,
            id.clone(),
            "pass description must not be empty",
        ));
    }

    if descriptor.preserves_all_analyses
        && !descriptor.invalidated_analyses.is_empty()
    {
        return Err(PassError::for_pass(
            PassErrorKind::InvalidConfiguration,
            id.clone(),
            "a pass cannot both preserve all analyses and declare invalidated analyses",
        ));
    }

    for analysis in descriptor
        .required_analyses
        .iter()
        .chain(descriptor.invalidated_analyses.iter())
    {
        if !is_valid_analysis_name(analysis) {
            return Err(PassError::for_pass(
                PassErrorKind::InvalidConfiguration,
                id.clone(),
                format!("invalid analysis name: {analysis:?}"),
            ));
        }
    }

    Ok(())
}

/// Validate a stable, namespaced pass ID.
///
/// Examples:
/// - `core.canonicalize`
/// - `classical.simplify`
/// - `quantum.lower_logical_ops`
/// - `hdl.elaborate`
fn is_valid_pass_id(value: &str) -> bool {
    if value.is_empty() || value.trim() != value {
        return false;
    }

    let mut segments = value.split('.');

    segments.all(|segment| {
        if segment.is_empty() {
            return false;
        }

        let mut chars = segment.chars();

        let Some(first) = chars.next() else {
            return false;
        };

        if !first.is_ascii_alphanumeric() {
            return false;
        }

        chars.all(|character| {
            character.is_ascii_alphanumeric()
                || character == '_'
                || character == '-'
        })
    }) && value.contains('.')
}

/// Analysis names are extensible identifiers, not a closed enum.
///
/// Namespaces are recommended for extension-owned analyses. Whitespace and
/// empty identifiers are rejected so accidental spelling differences do not
/// silently create distinct analysis keys.
fn is_valid_analysis_name(value: &str) -> bool {
    !value.is_empty()
        && value.trim() == value
        && !value.chars().any(char::is_whitespace)
}

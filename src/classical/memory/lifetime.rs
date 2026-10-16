
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Classical Computing — Lifetime Constraints and Region Resolution.
//!
//! Path: `src/classical/memory/lifetime.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Represent resolved named lifetime variables, explicit outlives constraints,
//! and bindings from named lifetimes to canonical semantic regions. Answer
//! local outlives queries by combining declared lifetime constraints with the
//! canonical region-scope relationships.
//!
//! # Authority
//!
//! - `grammar/memory/lifetimes.g4` owns lifetime source syntax.
//! - `grammar/memory/borrowing.g4` owns borrowing source syntax.
//! - `grammar/memory/memory.g4` owns memory and region source syntax.
//! - `src/zuir/ids.rs` owns canonical `RegionId`.
//! - `src/classical/memory/regions.rs` owns `RegionScopeIndex` and its
//!   enclosing-scope/outlives relationships.
//! - `src/classical/memory/ownership.rs` adapts canonical ownership contracts.
//! - `src/classical/memory/borrowing.rs` owns local borrow records.
//! - The semantic analyzer and canonical verifier own full lifetime inference,
//!   borrow checking, alias analysis, and control-flow-sensitive validity.
//! - The normative semantic model defines the observable language semantics.
//!
//! # Owns
//!
//! - Validated representations of named and anonymous lifetime references.
//! - The local environment of declared named lifetimes.
//! - Explicit named-lifetime outlives constraints.
//! - Optional bindings between named lifetimes and canonical `RegionId`s.
//! - Deterministic local outlives queries.
//! - Local consistency validation and structured errors.
//!
//! # Does not own
//!
//! - A competing lifetime ID or canonical region identity type.
//! - Source parsing, AST nodes, source spans, or name resolution.
//! - The canonical ownership model, reference types, or borrow records.
//! - Complete lifetime inference or a complete borrow checker.
//! - Control-flow graphs, dominance, alias analysis, or path-sensitive
//!   verification.
//! - Allocation, deallocation, physical addresses, or runtime storage.
//! - Device discovery, placement, scheduling, or hardware capabilities.
//! - Canonical classical IR, quantum IR, HDL IR, or runtime semantics.
//!
//! # Lifetime meaning
//!
//! A named lifetime is a resolved semantic name such as `a`, `buffer`, or
//! `request_scope`. The source apostrophe is not stored in the name.
//!
//! `'_` is represented as anonymous. It is not treated as a globally unique
//! lifetime, a static lifetime, or a wildcard that proves arbitrary
//! outlives relationships. Inference must resolve its meaning upstream.
//!
//! A constraint `longer: shorter` records that `longer` outlives `shorter`.
//! Constraints are directed. A relation is established by a path through the
//! constraint graph, by a concrete region relationship, or by their
//! transitive combination. A missing path returns `false`; this module does
//! not infer a relationship from unrelated regions.
//!
//! # POCO-REAF and scalability
//!
//! This module introduces no fixed maximum number of lifetimes, constraints,
//! or region bindings. Its ordered collections grow with actual input and
//! available resources. Graph queries use an iterative worklist rather than
//! recursive calls, avoiding call-stack depth as an artificial semantic
//! limit.
//!
//! Rust collections and the host still have finite representational and
//! resource limits. Allocation failure is not universally recoverable through
//! stable standard-library collection APIs; this module does not claim
//! literal infinite capacity.
//!
//! # Integration
//!
//! 1. The frontend parses lifetime syntax using `lifetimes.g4`.
//! 2. Name resolution supplies `LifetimeName` values and declares the named
//!    lifetimes in the relevant semantic environment.
//! 3. Semantic analysis records each explicit `'longer: 'shorter` constraint.
//! 4. Where a lifetime is concretely associated with a lexical region,
//!    semantic analysis binds it to the canonical `RegionId`.
//! 5. Lifetime verification queries this environment together with
//!    `RegionScopeIndex`, borrow records, ownership contracts, and control
//!    flow. A positive local query is evidence for that relation, not proof
//!    that the entire program is borrow-safe.
//! 6. Lowering preserves any lifetime-related obligations required by the
//!    canonical IR and target semantics.
//!
//! # Freeze rule
//!
//! Keep this file as a local lifetime-constraint environment. Do not add a
//! second ownership model, region tree, borrow checker, IR, or runtime.

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fmt;

use thiserror::Error;

use crate::zuir::ids::RegionId;

use super::regions::{RegionScopeError, RegionScopeIndex};

/// Result type for lifetime operations.
pub type LifetimeResult<T> = Result<T, LifetimeError>;

/// A source-resolved lifetime reference.
///
/// Named values contain the identifier without the source apostrophe.
/// Anonymous values represent `'_` and have no independently resolvable name.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LifetimeName {
    /// A named lifetime variable, for example `'buffer` stored as `buffer`.
    Named(String),

    /// An anonymous lifetime reference, spelled `'_`.
    Anonymous,
}

impl LifetimeName {
    /// Construct a named lifetime from its identifier without an apostrophe.
    ///
    /// This validates basic hygiene only. Source tokenization, Unicode
    /// normalization, reserved-name policy, and lexical scope resolution
    /// remain frontend/semantic-analysis responsibilities.
    pub fn named(name: impl Into<String>) -> LifetimeResult<Self> {
        let name = name.into();

        if name.is_empty() {
            return Err(LifetimeError::InvalidName {
                value: name,
                reason: "name must not be empty",
            });
        }

        if name.trim() != name {
            return Err(LifetimeError::InvalidName {
                value: name,
                reason: "leading or trailing whitespace is not permitted",
            });
        }

        if name.chars().any(char::is_control) {
            return Err(LifetimeError::InvalidName {
                value: name,
                reason: "control characters are not permitted",
            });
        }

        if name.contains('\'') {
            return Err(LifetimeError::InvalidName {
                value: name,
                reason: "the source apostrophe must not be part of the name",
            });
        }

        if name == "_" {
            return Err(LifetimeError::InvalidName {
                value: name,
                reason: "use anonymous() for the anonymous lifetime",
            });
        }

        Ok(Self::Named(name))
    }

    /// Construct the anonymous lifetime reference `'_`.
    #[must_use]
    pub const fn anonymous() -> Self {
        Self::Anonymous
    }

    /// Parse the normalized spelling of a lifetime token, including its
    /// leading apostrophe.
    ///
    /// Examples: `'a`, `'buffer`, and `'_`.
    ///
    /// This helper expects a single already-tokenized lifetime spelling. It
    /// is not a replacement for the canonical lexer or parser.
    pub fn from_source_spelling(spelling: &str) -> LifetimeResult<Self> {
        let Some(name) = spelling.strip_prefix('\'') else {
            return Err(LifetimeError::InvalidSpelling {
                spelling: spelling.to_owned(),
            });
        };

        if name == "_" {
            return Ok(Self::Anonymous);
        }

        if name.is_empty() {
            return Err(LifetimeError::InvalidSpelling {
                spelling: spelling.to_owned(),
            });
        }

        Self::named(name)
    }

    /// Return the name of a named lifetime.
    ///
    /// Returns `None` for the anonymous lifetime.
    #[must_use]
    pub fn as_named(&self) -> Option<&str> {
        match self {
            Self::Named(name) => Some(name),
            Self::Anonymous => None,
        }
    }

    /// Whether this is the anonymous lifetime `'_`.
    #[must_use]
    pub const fn is_anonymous(&self) -> bool {
        matches!(self, Self::Anonymous)
    }
}

impl fmt::Display for LifetimeName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Named(name) => write!(f, "'{name}"),
            Self::Anonymous => f.write_str("'_"),
        }
    }
}

/// A declared named-lifetime outlives constraint.
///
/// `longer: shorter` means that the longer lifetime is required to outlive
/// the shorter lifetime. The relation is directed and is not inferred from
/// identifier spelling or declaration order.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LifetimeConstraint {
    longer: LifetimeName,
    shorter: LifetimeName,
}

impl LifetimeConstraint {
    /// Construct a constraint between two named lifetimes.
    ///
    /// Anonymous lifetimes cannot be used as named graph vertices. Their
    /// meaning must be resolved by the semantic analyzer before adding a
    /// named-lifetime constraint.
    pub fn new(
        longer: LifetimeName,
        shorter: LifetimeName,
    ) -> LifetimeResult<Self> {
        if longer.is_anonymous() {
            return Err(LifetimeError::AnonymousConstraint {
                position: "longer",
            });
        }

        if shorter.is_anonymous() {
            return Err(LifetimeError::AnonymousConstraint {
                position: "shorter",
            });
        }

        Ok(Self { longer, shorter })
    }

    /// The lifetime required to outlive the other.
    #[must_use]
    pub fn longer(&self) -> &LifetimeName {
        &self.longer
    }

    /// The lifetime that must be outlived.
    #[must_use]
    pub fn shorter(&self) -> &LifetimeName {
        &self.shorter
    }
}

/// Errors produced by the lifetime environment.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum LifetimeError {
    /// A lifetime identifier or source spelling is malformed.
    #[error("invalid lifetime name {value:?}: {reason}")]
    InvalidName {
        /// Supplied name.
        value: String,
        /// Reason the name is invalid.
        reason: &'static str,
    },

    /// A lifetime spelling is not a valid apostrophe-prefixed name.
    #[error("invalid lifetime spelling {spelling:?}; expected a spelling such as 'a or '_")]
    InvalidSpelling {
        /// Supplied source spelling.
        spelling: String,
    },

    /// An anonymous lifetime was used where a named variable is required.
    #[error("anonymous lifetime cannot be used in the {position} position of a named outlives constraint")]
    AnonymousConstraint {
        /// Constraint position.
        position: &'static str,
    },

    /// A named lifetime was declared more than once in this environment.
    #[error("lifetime {0} is already declared")]
    DuplicateLifetime(LifetimeName),

    /// A constraint refers to a named lifetime not declared in the environment.
    #[error("lifetime {0} is not declared in this environment")]
    UndeclaredLifetime(LifetimeName),

    /// A concrete region binding was already declared for this lifetime.
    #[error("lifetime {lifetime} is already bound to region {existing:?}; cannot also bind it to {requested:?}")]
    DuplicateRegionBinding {
        /// Named lifetime being bound.
        lifetime: LifetimeName,
        /// Existing canonical region.
        existing: RegionId,
        /// Newly requested canonical region.
        requested: RegionId,
    },

    /// A concrete region binding uses an undeclared named lifetime.
    #[error("cannot bind undeclared lifetime {0}")]
    BindingForUndeclaredLifetime(LifetimeName),

    /// The requested binding refers to an unknown semantic region.
    #[error("lifetime {lifetime} refers to an undeclared region {region:?}")]
    UnknownRegion {
        /// Named lifetime being bound.
        lifetime: LifetimeName,
        /// Unknown canonical region.
        region: RegionId,
    },

    /// A stored constraint refers to a lifetime absent from the declaration
    /// set. This indicates an inconsistent environment.
    #[error("lifetime environment contains a constraint with undeclared lifetime {0}")]
    InconsistentConstraint(LifetimeName),

    /// A stored region binding refers to a region absent from the region index.
    #[error("lifetime {lifetime} is bound to unknown region {region:?}")]
    InconsistentRegionBinding {
        /// Named lifetime being bound.
        lifetime: LifetimeName,
        /// Missing region.
        region: RegionId,
    },

    /// An error from the canonical semantic region index.
    #[error(transparent)]
    Region(#[from] RegionScopeError),
}

/// Deterministic environment of named lifetimes and their relationships.
///
/// The environment stores only explicit named constraints and optional
/// concrete region bindings. It does not perform inference, lexical
/// resolution, borrow checking, or control-flow analysis.
///
/// A separate environment should be used for each semantic scope in which
/// lifetime names have a distinct resolution context.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LifetimeEnvironment {
    declared: BTreeSet<LifetimeName>,
    constraints: BTreeSet<LifetimeConstraint>,
    region_bindings: BTreeMap<LifetimeName, RegionId>,
}

impl LifetimeEnvironment {
    /// Construct an empty lifetime environment.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Declare a named lifetime variable.
    ///
    /// Declaration does not imply any outlives relationship.
    pub fn declare(&mut self, lifetime: LifetimeName) -> LifetimeResult<()> {
        if lifetime.is_anonymous() {
            return Err(LifetimeError::InvalidName {
                value: "_".to_owned(),
                reason: "anonymous lifetimes are references, not named declarations",
            });
        }

        if !self.declared.insert(lifetime.clone()) {
            return Err(LifetimeError::DuplicateLifetime(lifetime));
        }

        Ok(())
    }

    /// Declare a named lifetime from its identifier without an apostrophe.
    pub fn declare_named(
        &mut self,
        name: impl Into<String>,
    ) -> LifetimeResult<LifetimeName> {
        let lifetime = LifetimeName::named(name)?;
        self.declare(lifetime.clone())?;
        Ok(lifetime)
    }

    /// Whether a named lifetime is declared in this environment.
    #[must_use]
    pub fn contains(&self, lifetime: &LifetimeName) -> bool {
        self.declared.contains(lifetime)
    }

    /// Number of named lifetimes declared in this environment.
    #[must_use]
    pub fn len(&self) -> usize {
        self.declared.len()
    }

    /// Whether the environment contains no declared named lifetimes.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.declared.is_empty()
    }

    /// Iterate over declared lifetime names in deterministic order.
    pub fn lifetimes(&self) -> impl Iterator<Item = &LifetimeName> {
        self.declared.iter()
    }

    /// Add an explicit `longer: shorter` outlives constraint.
    ///
    /// Both names must already be declared. Adding the same constraint more
    /// than once is idempotent.
    ///
    /// Cycles are not automatically rejected: the language's semantic
    /// specification decides whether mutually outliving lifetime variables
    /// are valid in the relevant context. This environment records the
    /// relation without silently imposing an extra language rule.
    pub fn add_outlives(
        &mut self,
        longer: LifetimeName,
        shorter: LifetimeName,
    ) -> LifetimeResult<()> {
        let constraint = LifetimeConstraint::new(longer, shorter)?;

        if !self.declared.contains(constraint.longer()) {
            return Err(LifetimeError::UndeclaredLifetime(
                constraint.longer().clone(),
            ));
        }

        if !self.declared.contains(constraint.shorter()) {
            return Err(LifetimeError::UndeclaredLifetime(
                constraint.shorter().clone(),
            ));
        }

        self.constraints.insert(constraint);
        Ok(())
    }

    /// Record a constraint using identifiers without apostrophes.
    pub fn add_named_outlives(
        &mut self,
        longer: &str,
        shorter: &str,
    ) -> LifetimeResult<()> {
        self.add_outlives(
            LifetimeName::named(longer)?,
            LifetimeName::named(shorter)?,
        )
    }

    /// Iterate over explicit constraints in deterministic order.
    pub fn constraints(&self) -> impl Iterator<Item = &LifetimeConstraint> {
        self.constraints.iter()
    }

    /// Bind a declared named lifetime to a canonical semantic region.
    ///
    /// A binding is accepted only when the region exists in the supplied
    /// `RegionScopeIndex`. Repeating the same binding is idempotent; binding
    /// the lifetime to a different region returns an error.
    ///
    /// This is appropriate only when semantic analysis has established that
    /// the lifetime is represented by that concrete region. Generic lifetime
    /// variables do not have to be bound to a lexical region.
    pub fn bind_region(
        &mut self,
        lifetime: LifetimeName,
        region: RegionId,
        regions: &RegionScopeIndex,
    ) -> LifetimeResult<()> {
        if lifetime.is_anonymous() {
            return Err(LifetimeError::BindingForUndeclaredLifetime(lifetime));
        }

        if !self.declared.contains(&lifetime) {
            return Err(LifetimeError::BindingForUndeclaredLifetime(lifetime));
        }

        if !regions.contains(region) {
            return Err(LifetimeError::UnknownRegion { lifetime, region });
        }

        if let Some(existing) = self.region_bindings.get(&lifetime).copied() {
            if existing == region {
                return Ok(());
            }

            return Err(LifetimeError::DuplicateRegionBinding {
                lifetime,
                existing,
                requested: region,
            });
        }

        self.region_bindings.insert(lifetime, region);
        Ok(())
    }

    /// Return the canonical region associated with a named lifetime, if any.
    #[must_use]
    pub fn region_of(&self, lifetime: &LifetimeName) -> Option<RegionId> {
        self.region_bindings.get(lifetime).copied()
    }

    /// Iterate over concrete lifetime-to-region bindings in deterministic
    /// lifetime-name order.
    pub fn region_bindings(
        &self,
    ) -> impl Iterator<Item = (&LifetimeName, &RegionId)> {
        self.region_bindings.iter()
    }

    /// Determine whether `longer` is known to outlive `shorter`.
    ///
    /// The query combines:
    ///
    /// - explicit named-lifetime constraints;
    /// - transitive paths through those constraints;
    /// - region relationships supplied by `RegionScopeIndex`;
    /// - transitive combinations of named constraints and concrete-region
    ///   relationships.
    ///
    /// A lifetime outlives itself when it is a declared named lifetime.
    /// Anonymous lifetimes cannot be queried as stable named vertices and
    /// return `false` unless the caller resolves them before this call.
    ///
    /// `Ok(false)` means the supplied facts do not establish the relation.
    /// It does not prove that the program is invalid, nor does `Ok(true)`
    /// prove whole-program borrow safety.
    pub fn outlives(
        &self,
        longer: &LifetimeName,
        shorter: &LifetimeName,
        regions: &RegionScopeIndex,
    ) -> LifetimeResult<bool> {
        if longer.is_anonymous() || shorter.is_anonymous() {
            return Ok(false);
        }

        self.require_declared(longer)?;
        self.require_declared(shorter)?;

        if longer == shorter {
            return Ok(true);
        }

        // Validate stored facts before answering a semantic query. This also
        // catches invalid data if an environment was reconstructed by a
        // future serialization or integration layer.
        self.validate(regions)?;

        let mut visited: BTreeSet<LifetimeName> = BTreeSet::new();
        let mut pending: VecDeque<LifetimeName> = VecDeque::new();

        visited.insert(longer.clone());
        pending.push_back(longer.clone());

        while let Some(current) = pending.pop_front() {
            if &current == shorter {
                return Ok(true);
            }

            // Explicit edge: current outlives each directly constrained
            // shorter lifetime.
            for constraint in &self.constraints {
                if constraint.longer() == &current {
                    let next = constraint.shorter();

                    if visited.insert(next.clone()) {
                        pending.push_back(next.clone());
                    }
                }
            }

            // Concrete-region edge: if the current lifetime is bound to a
            // region, every bound lifetime whose region is nested within it
            // is reachable through the canonical region index.
            if let Some(current_region) = self.region_bindings.get(&current) {
                for (candidate, candidate_region) in &self.region_bindings {
                    if visited.contains(candidate) {
                        continue;
                    }

                    if regions.outlives(*current_region, *candidate_region)? {
                        visited.insert(candidate.clone());
                        pending.push_back(candidate.clone());
                    }
                }
            }
        }

        Ok(false)
    }

    /// Validate declarations, explicit constraints, and concrete bindings
    /// against a canonical region index.
    ///
    /// This is local environment consistency validation, not whole-program
    /// lifetime verification.
    pub fn validate(&self, regions: &RegionScopeIndex) -> LifetimeResult<()> {
        for constraint in &self.constraints {
            if !self.declared.contains(constraint.longer()) {
                return Err(LifetimeError::InconsistentConstraint(
                    constraint.longer().clone(),
                ));
            }

            if !self.declared.contains(constraint.shorter()) {
                return Err(LifetimeError::InconsistentConstraint(
                    constraint.shorter().clone(),
                ));
            }
        }

        for (lifetime, region) in &self.region_bindings {
            if !self.declared.contains(lifetime) {
                return Err(LifetimeError::InconsistentConstraint(
                    lifetime.clone(),
                ));
            }

            if !regions.contains(*region) {
                return Err(LifetimeError::InconsistentRegionBinding {
                    lifetime: lifetime.clone(),
                    region: *region,
                });
            }
        }

        Ok(())
    }

    fn require_declared(&self, lifetime: &LifetimeName) -> LifetimeResult<()> {
        if self.declared.contains(lifetime) {
            Ok(())
        } else {
            Err(LifetimeError::UndeclaredLifetime(lifetime.clone()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zuir::ids::RegionId;
    use crate::classical::memory::regions::RegionScopeIndex;

    fn lifetime(name: &str) -> LifetimeName {
        LifetimeName::named(name).expect("valid lifetime name")
    }

    fn region(raw: u128) -> RegionId {
        RegionId::new(raw).expect("nonzero region ID")
    }

    #[test]
    fn parses_named_and_anonymous_spellings() {
        assert_eq!(
            LifetimeName::from_source_spelling("'buffer").expect("named"),
            lifetime("buffer")
        );
        assert_eq!(
            LifetimeName::from_source_spelling("'_").expect("anonymous"),
            LifetimeName::anonymous()
        );
        assert!(LifetimeName::from_source_spelling("buffer").is_err());
        assert!(LifetimeName::from_source_spelling("'").is_err());
    }

    #[test]
    fn rejects_invalid_lifetime_names() {
        assert!(LifetimeName::named("").is_err());
        assert!(LifetimeName::named(" buffer").is_err());
        assert!(LifetimeName::named("buffer ").is_err());
        assert!(LifetimeName::named("bad\nname").is_err());
        assert!(LifetimeName::named("'buffer").is_err());
        assert!(LifetimeName::named("_").is_err());
    }

    #[test]
    fn declares_named_lifetimes_without_implying_outlives() {
        let mut env = LifetimeEnvironment::new();
        let a = env.declare_named("a").expect("declare a");
        let b = env.declare_named("b").expect("declare b");
        let regions = RegionScopeIndex::new();

        assert!(env.contains(&a));
        assert!(env.contains(&b));
        assert_eq!(env.len(), 2);
        assert!(!env.is_empty());

        assert!(!env.outlives(&a, &b, &regions).expect("query"));
        assert!(env.outlives(&a, &a, &regions).expect("reflexive query"));
    }

    #[test]
    fn rejects_duplicate_declarations() {
        let mut env = LifetimeEnvironment::new();
        env.declare_named("a").expect("first declaration");

        assert!(matches!(
            env.declare(lifetime("a")),
            Err(LifetimeError::DuplicateLifetime(_))
        ));
    }

    #[test]
    fn records_transitive_named_constraints() {
        let mut env = LifetimeEnvironment::new();
        let a = env.declare_named("a").expect("declare a");
        let b = env.declare_named("b").expect("declare b");
        let c = env.declare_named("c").expect("declare c");
        let regions = RegionScopeIndex::new();

        env.add_outlives(a.clone(), b.clone()).expect("a outlives b");
        env.add_outlives(b.clone(), c.clone()).expect("b outlives c");

        assert!(env.outlives(&a, &b, &regions).expect("a/b query"));
        assert!(env.outlives(&b, &c, &regions).expect("b/c query"));
        assert!(env.outlives(&a, &c, &regions).expect("transitive query"));
        assert!(!env.outlives(&c, &a, &regions).expect("reverse query"));
    }

    #[test]
    fn rejects_constraints_with_undeclared_lifetimes() {
        let mut env = LifetimeEnvironment::new();
        env.declare_named("a").expect("declare a");

        assert!(matches!(
            env.add_named_outlives("a", "missing"),
            Err(LifetimeError::UndeclaredLifetime(_))
        ));
    }

    #[test]
    fn rejects_anonymous_lifetimes_in_named_constraints() {
        assert!(matches!(
            LifetimeConstraint::new(
                LifetimeName::anonymous(),
                lifetime("a")
            ),
            Err(LifetimeError::AnonymousConstraint { .. })
        ));

        assert!(matches!(
            LifetimeConstraint::new(
                lifetime("a"),
                LifetimeName::anonymous()
            ),
            Err(LifetimeError::AnonymousConstraint { .. })
        ));
    }

    #[test]
    fn combines_region_and_named_constraint_edges_transitively() {
        let mut regions = RegionScopeIndex::new();
        let root = region(1);
        let child = region(2);

        regions.declare_root(root).expect("declare root");
        regions.declare_child(child, root).expect("declare child");

        let mut env = LifetimeEnvironment::new();
        let a = env.declare_named("a").expect("declare a");
        let b = env.declare_named("b").expect("declare b");
        let c = env.declare_named("c").expect("declare c");

        // a is concretely associated with the root; b is associated with
        // its child. The region model establishes a outlives b.
        env.bind_region(a.clone(), root, &regions)
            .expect("bind a to root");
        env.bind_region(b.clone(), child, &regions)
            .expect("bind b to child");

        // Add a symbolic edge from b to c. The query should combine the
        // concrete region relationship with this explicit constraint.
        env.add_outlives(b.clone(), c.clone())
            .expect("b outlives c");

        assert!(env.outlives(&a, &b, &regions).expect("region edge"));
        assert!(env.outlives(&b, &c, &regions).expect("symbolic edge"));
        assert!(env.outlives(&a, &c, &regions).expect("mixed transitivity"));
        assert!(!env.outlives(&b, &a, &regions).expect("reverse relation"));
    }

    #[test]
    fn region_binding_requires_a_declared_lifetime_and_region() {
        let regions = RegionScopeIndex::new();
        let mut env = LifetimeEnvironment::new();
        let root = region(1);

        regions.clone();
        assert!(matches!(
            env.bind_region(lifetime("missing"), root, &regions),
            Err(LifetimeError::BindingForUndeclaredLifetime(_))
        ));

        env.declare_named("a").expect("declare a");
        assert!(matches!(
            env.bind_region(lifetime("a"), root, &regions),
            Err(LifetimeError::UnknownRegion { .. })
        ));
    }

    #[test]
    fn binding_the_same_lifetime_to_different_regions_is_rejected() {
        let mut regions = RegionScopeIndex::new();
        let root = region(1);
        let child = region(2);
        regions.declare_root(root).expect("root");
        regions.declare_child(child, root).expect("child");

        let mut env = LifetimeEnvironment::new();
        let a = env.declare_named("a").expect("declare a");

        env.bind_region(a.clone(), root, &regions)
            .expect("first binding");

        assert!(matches!(
            env.bind_region(a, child, &regions),
            Err(LifetimeError::DuplicateRegionBinding { .. })
        ));
    }

    #[test]
    fn repeating_the_same_constraint_or_binding_is_idempotent() {
        let mut regions = RegionScopeIndex::new();
        let root = region(1);
        regions.declare_root(root).expect("root");

        let mut env = LifetimeEnvironment::new();
        let a = env.declare_named("a").expect("declare a");
        let b = env.declare_named("b").expect("declare b");

        env.add_outlives(a.clone(), b.clone()).expect("first edge");
        env.add_outlives(a.clone(), b.clone()).expect("repeated edge");

        env.bind_region(a.clone(), root, &regions)
            .expect("first binding");
        env.bind_region(a, root, &regions)
            .expect("repeated binding");

        assert_eq!(env.constraints().count(), 1);
        assert_eq!(env.region_bindings().count(), 1);
    }

    #[test]
    fn anonymous_lifetime_does_not_claim_arbitrary_outlives_relations() {
        let mut env = LifetimeEnvironment::new();
        let a = env.declare_named("a").expect("declare a");
        let regions = RegionScopeIndex::new();

        assert!(!env
            .outlives(&LifetimeName::anonymous(), &a, &regions)
            .expect("anonymous query"));
        assert!(!env
            .outlives(&a, &LifetimeName::anonymous(), &regions)
            .expect("anonymous query"));
    }

    #[test]
    fn uses_canonical_region_ids_without_a_duplicate_region_type() {
        let mut regions = RegionScopeIndex::new();
        let outer = region(100);
        let inner = region(200);
        regions.declare_root(outer).expect("outer");
        regions.declare_child(inner, outer).expect("inner");

        let mut env = LifetimeEnvironment::new();
        let a = env.declare_named("outer").expect("declare outer");
        let b = env.declare_named("inner").expect("declare inner");

        env.bind_region(a.clone(), outer, &regions)
            .expect("bind outer");
        env.bind_region(b.clone(), inner, &regions)
            .expect("bind inner");

        assert_eq!(env.region_of(&a), Some(outer));
        assert_eq!(env.region_of(&b), Some(inner));
        assert!(env.outlives(&a, &b, &regions).expect("outer outlives inner"));
    }

    #[test]
    fn no_fixed_constraint_or_lifetime_count_is_encoded() {
        let mut env = LifetimeEnvironment::new();

        // Exercise a count above small fixed-size assumptions without
        // encoding that count as a production limit.
        let mut names = Vec::new();
        for index in 0_u128..128 {
            let name = format!("lifetime_{index}");
            names.push(env.declare_named(name).expect("declare lifetime"));
        }

        for pair in names.windows(2) {
            env.add_outlives(pair[0].clone(), pair[1].clone())
                .expect("add adjacent constraint");
        }

        let regions = RegionScopeIndex::new();
        assert!(env
            .outlives(&names[0], names.last().expect("last"), &regions)
            .expect("long transitive query"));
    }
}

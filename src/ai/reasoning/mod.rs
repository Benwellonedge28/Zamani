#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani AI — Reasoning Domain
//!
//! Path: `src/ai/reasoning/mod.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! This module is the composition root for the AI reasoning domain.
//!
//! It exposes the independently owned reasoning components used by Zamani's
//! AI computational subsystem. The components describe reasoning intent,
//! preserve canonical frontend AST references, and provide domain-specific
//! structural validation interfaces.
//!
//! Reasoning capabilities include:
//!
//! - Symbolic reasoning.
//! - Inference.
//! - Deduction.
//! - Induction.
//! - Abduction.
//! - Causal computation.
//! - Probabilistic computation.
//! - Knowledge representation.
//! - Evidence representation.
//! - Uncertainty representation.
//!
//! These capabilities are intended to compose with classical, tensor,
//! quantum, hardware, distributed, hybrid, and future computational domains.
//!
//! ## Architectural authorities
//!
//! Source syntax is owned by the canonical Zamani grammar and its
//! domain-specific grammar composition boundaries, including:
//!
//! - `grammar/statements/reason.g4`
//! - `grammar/ai/reasoning.g4`
//! - `grammar/ai/inference.g4`
//! - `grammar/ai/deduction.g4`
//! - `grammar/ai/induction.g4`
//! - `grammar/ai/abduction.g4`
//! - `grammar/ai/causality.g4`
//! - `grammar/ai/probabilistic.g4`
//! - `grammar/ai/knowledge.g4`
//! - `grammar/ai/evidence.g4`
//! - `grammar/ai/uncertainty.g4`
//! - `grammar/ai/ai.g4`
//!
//! The canonical frontend AST remains the source-structure authority.
//!
//! The frontend semantic subsystem remains the semantic authority:
//!
//! `src/frontend/semantic/domains/ai.rs`
//!
//! The canonical universal intermediate representation remains ZUIR.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - The public module boundary for AI reasoning.
//! - The composition of reasoning submodules.
//! - The stable paths through which reasoning components are accessed.
//!
//! This module does not own:
//!
//! - Source syntax.
//! - AST construction.
//! - Semantic analysis.
//! - Type checking.
//! - Evidence or knowledge persistence.
//! - A second reasoning IR.
//! - Theorem-proving or inference algorithms.
//! - Probability arithmetic.
//! - Hardware selection.
//! - Resource allocation.
//! - Scheduling or routing.
//! - Runtime execution.
//!
//! ## POCO-REAF and scalability
//!
//! This composition root imposes no language-level limit on:
//!
//! - The number of reasoning constructs.
//! - The number of evidence references.
//! - The size of a knowledge representation.
//! - The number of probabilistic or causal variables.
//! - The number of reasoning operations.
//! - The size or topology of a reasoning computation.
//!
//! Practical limits are determined by the relevant representation,
//! implementation, resource budgets, capabilities, policies, and physical
//! execution environment. Such limits must be reported by the owning
//! subsystem and must not silently alter program semantics.
//!
//! ## Integration
//!
//! The public integration path is:
//!
//! ```text
//! Zamani source
//!     |
//!     v
//! Canonical grammar
//!     |
//!     v
//! Canonical frontend AST
//!     |
//!     v
//! Frontend semantic analysis
//!     |
//!     v
//! AI reasoning modules
//!     |
//!     v
//! Canonical ZUIR and domain IRs
//!     |
//!     v
//! Optimization and lowering
//!     |
//!     v
//! Runtime / backend / distributed realization
//! ```
//!
//! The reasoning modules describe computational intent. They do not select
//! or require a particular processor, accelerator, machine, or vendor.
//!
//! ## Public API policy
//!
//! Submodules are exposed by their canonical module paths.
//!
//! This composition root deliberately avoids wildcard re-exports. Reasoning
//! modules contain independently owned types, and flattening those types
//! into one namespace can create ambiguous names or couple this boundary to
//! implementation details.
//!
//! Consumers should use paths such as:
//!
//! ```ignore
//! use crate::ai::reasoning::inference::InferenceProgram;
//! use crate::ai::reasoning::deduction::DeductionProgram;
//! use crate::ai::reasoning::probabilistic::ProbabilisticProgram;
//! ```
//!
//! ## Freeze criteria
//!
//! This module is complete when:
//!
//! - Every declared module has an owning implementation file.
//! - Module paths remain stable.
//! - No duplicate reasoning authority is introduced.
//! - No wildcard re-export creates name collisions.
//! - The AI parent module exposes this module.
//! - The crate compiles and its reasoning tests pass.
//!
//! This file contains no reasoning algorithms or runtime behavior.

// -----------------------------------------------------------------------------
// Symbolic and logical reasoning
// -----------------------------------------------------------------------------

/// Symbolic expressions, relations, assumptions, substitutions, rewrite rules,
/// and symbolic structural validation.
pub mod symbolic;

/// Inference declarations, invocations, bindings, directives, and validation.
pub mod inference;

/// Deductive reasoning declarations, premises, derivations, and validation.
pub mod deduction;

/// Inductive reasoning requests, observations, generalization intent, and
/// validation.
pub mod induction;

/// Abductive reasoning statements, candidate explanations, hypotheses, and
/// validation.
pub mod abduction;

// -----------------------------------------------------------------------------
// Causal and probabilistic reasoning
// -----------------------------------------------------------------------------

/// Causal operations, relationships, interventions, and causal validation.
pub mod causality;

/// Probabilistic computation descriptions, distributions, observations,
/// conditioning, evidence references, uncertainty references, and validation.
pub mod probabilistic;

// -----------------------------------------------------------------------------
// Knowledge, evidence, and uncertainty
// -----------------------------------------------------------------------------

/// Knowledge representations and provider interfaces.
///
/// Persistent knowledge storage and knowledge execution remain owned by the
/// appropriate existing knowledge infrastructure.
pub mod knowledge;

/// Evidence records, source references, relationships, provenance references,
/// access states, and evidence validation.
pub mod evidence;

/// Uncertainty descriptions, relationships, propagation intent, and
/// uncertainty validation.
pub mod uncertainty;
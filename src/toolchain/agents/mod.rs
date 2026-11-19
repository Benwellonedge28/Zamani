
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Toolchain — Agent Integration
//!
//! Path: `src/toolchain/agents/mod.rs`
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! This module is the composition boundary for human and AI-agent
//! integration with the Zamani toolchain.
//!
//! It exposes the common agent-facing contracts for:
//!
//! - Constructing validated toolchain context.
//! - Representing canonical toolchain requests.
//! - Representing structured operation results.
//! - Authorizing toolchain operations.
//! - Recording agent operation audit events.
//! - Evaluating agent workflow compliance.
//!
//! Human developers, automated systems and AI agents must use the
//! same underlying toolchain services, authorization boundaries,
//! validation rules and operation semantics.
//!
//! This module provides the public module structure. It does not
//! implement a second command system, compiler, policy engine,
//! security engine, audit database or execution environment.
//!
//! ## Architectural ownership
//!
//! This module owns:
//!
//! - Composition and public exposure of agent integration modules.
//! - The agent subsystem's documented module boundaries.
//! - The relationship between context, requests, results,
//!   authorization, auditing and compliance.
//!
//! The individual child modules own their respective contracts
//! and implementations.
//!
//! ## Child module responsibilities
//!
//! `context`
//!     Validated, provenance-aware context snapshots and context
//!     construction. Context is descriptive information, not
//!     authorization or proof of feasibility.
//!
//! `requests`
//!     Validated agent request wrappers around canonical toolchain
//!     API requests. Requests do not execute operations.
//!
//! `results`
//!     Structured agent workflow results, stage outcomes and
//!     advisory next-action suggestions. Suggested actions do not
//!     execute or authorize operations.
//!
//! `permissions`
//!     Operation-specific authorization using the canonical
//!     command authorization and security policy infrastructure.
//!     Agent-supplied identity is not authenticated identity.
//!
//! `audit`
//!     Agent operation audit events, audit recording contracts
//!     and audit sink integration. This module does not replace
//!     the command system's audit enforcement.
//!
//! `compliance`
//!     Agent workflow compliance checks, evidence, reports and
//!     fail-closed compliance evaluation. Compliance does not
//!     authenticate clients or grant authorization.
//!
//! ## Canonical integration boundaries
//!
//! The agent subsystem integrates with:
//!
//! - `crate::toolchain::api`
//! - `crate::toolchain::commands`
//! - `crate::toolchain::errors`
//! - `crate::toolchain::result`
//! - `crate::toolchain::security::policy`
//! - `crate::toolchain::security::validation`
//! - `crate::toolchain::agents::context`
//! - `crate::toolchain::agents::requests`
//! - `crate::toolchain::agents::results`
//! - `crate::toolchain::agents::permissions`
//! - `crate::toolchain::agents::audit`
//! - `crate::toolchain::agents::compliance`
//!
//! These are integration boundaries, not competing implementations.
//!
//! The agent subsystem must use the canonical toolchain API and
//! command services. It must not bypass those services by invoking
//! compiler internals, backend implementations, hardware providers
//! or runtime internals directly.
//!
//! ## Operation lifecycle
//!
//! An agent workflow follows the contracts owned by the relevant
//! child modules and canonical toolchain services:
//!
//! 1. Construct or obtain an appropriate context.
//! 2. Construct a request using the canonical toolchain API.
//! 3. Validate the agent request.
//! 4. Authenticate the client through the authoritative identity
//!    provider.
//! 5. Authorize the requested operation through the canonical
//!    command authorization boundary.
//! 6. Evaluate the applicable compliance requirements.
//! 7. Dispatch through the canonical command service only when
//!    all mandatory authorization and compliance conditions pass.
//! 8. Record required audit events through the authoritative
//!    audit infrastructure.
//! 9. Return a validated, structured operation result.
//!
//! This describes the integration relationship. The command
//! dispatcher and individual services remain responsible for
//! enforcing their own mandatory checks and lifecycle guarantees.
//!
//! A successful context construction, request validation,
//! authorization receipt, compliance report or request submission
//! does not independently establish successful execution.
//!
//! ## Security invariants
//!
//! - Authentication and authorization remain separate.
//! - Agent identity claims are not authentication evidence.
//! - Registration of an agent or provider does not establish trust.
//! - A permission declaration does not establish authorization.
//! - Only the canonical authorization authority establishes
//!   an authorization decision.
//! - Only an explicit permit can establish authorization success.
//! - Missing, denied or indeterminate authorization cannot become
//!   permission to execute.
//! - Compliance cannot grant authorization.
//! - Compliance cannot replace required audit recording.
//! - Advisory suggestions cannot execute themselves.
//! - Agent-supplied context and evidence are not automatically trusted.
//! - Unknown or unresolved mandatory conditions fail closed.
//! - Credentials, private keys and raw secrets must not be exposed
//!   through agent context, requests, results or audit messages.
//! - Deployment and other externally consequential operations
//!   require the authorization imposed by their owning services.
//!
//! ## Computing-domain independence
//!
//! This module does not enumerate a closed set of computing domains.
//!
//! The agent contracts can coordinate requests concerning:
//!
//! - Classical computation.
//! - Quantum computation.
//! - HDL and hardware realization.
//! - AI and tensor computation.
//! - Hybrid computation.
//! - Distributed computation.
//! - Networking and cryptographic computation.
//! - Future registered computational domains.
//!
//! Domain semantics, compilation, feasibility, physical resource
//! discovery, allocation and execution remain with their respective
//! repository authorities.
//!
//! ## POCO-REAF and scalability
//!
//! Agent workflows must preserve the same target-independent
//! project and operation contracts used by human developers.
//!
//! This module introduces no universal maximum for:
//!
//! - Agents or clients.
//! - Projects or workspaces.
//! - Requests or results.
//! - Workflow stages.
//! - Computing domains.
//! - Targets or providers.
//! - Hardware resources.
//! - Distributed participants.
//!
//! No arbitrary hardware-capacity ceilings or closed vendor lists
//! are introduced here.
//!
//! Collections and operations remain subject to available memory,
//! address space, execution time, configured operational budgets,
//! implementation capabilities and applicable policies.
//!
//! Literal infinite computation or storage is not promised.
//!
//! Resource exhaustion and unavailable capabilities must be
//! represented explicitly by the owning service. Information must
//! not be silently truncated or treated as successfully processed.
//!
//! ## Concurrency and state
//!
//! This module introduces:
//!
//! - No global mutable state.
//! - No implicit singleton agent.
//! - No hidden worker pool.
//! - No implicit identity provider.
//! - No implicit allow-all authorization policy.
//! - No implicit audit sink.
//! - No implicit execution environment.
//!
//! Providers, policies, sinks and other dependencies are supplied
//! through the explicit constructors and contracts owned by the
//! respective child modules.
//!
//! Thread-safety requirements are defined by those child contracts.
//!
//! ## Grammar and semantic integration
//!
//! Agent integration must remain consistent with the repository's
//! canonical source-language and security specifications:
//!
//! - `grammar/Zamani.g4`
//! - `grammar/security/permissions.g4`
//! - `grammar/security/authorization.g4`
//! - `grammar/security/audit.g4`
//! - `grammar/spec/security.md`
//! - `grammar/specification/poco-reaf.md`
//! - `grammar/specification/scalability-model.md`
//!
//! These grammar and specification files define source-level
//! structure and security intent. They do not implement runtime
//! authentication, authorization, auditing or agent execution.
//!
//! This module must not introduce a second parser, grammar,
//! AST, semantic model or security policy language.
//!
//! ## Dependencies
//!
//! The child modules reuse the existing repository dependencies,
//! including the canonical toolchain API, command services,
//! security infrastructure, error contracts and serialization
//! support.
//!
//! This composition module introduces no additional dependency.
//!
//! ## Public API policy
//!
//! Child modules are exposed through their explicit module paths.
//!
//! This module deliberately does not flatten all child-module
//! symbols into one namespace. This preserves ownership,
//! avoids ambiguous names and allows each child module to evolve
//! under its own documented API contract.
//!
//! Examples:
//!
//! - `agents::context::AgentContext`
//! - `agents::requests::AgentRequest`
//! - `agents::results::AgentResult`
//! - `agents::permissions::AgentPermissionAuthorizer`
//! - `agents::audit::AuditRecorder`
//! - `agents::compliance::AgentComplianceEvaluator`
//!
//! ## Completion criteria
//!
//! This module is complete when:
//!
//! - Every existing agent implementation module is declared.
//! - Every declared module resolves to its actual source file.
//! - No duplicate agent implementation is introduced.
//! - Existing child-module public contracts remain accessible.
//! - All child modules compile through the canonical toolchain
//!   module hierarchy.
//! - The library exposes the toolchain under its intended
//!   Cargo feature.
//! - Unit and integration tests for the child modules pass.
//! - The complete build-system feature passes the repository's
//!   formatting, compilation, test and lint gates.
//!
//! This file is a composition boundary, not a substitute for
//! testing the implementations it exposes.

// ============================================================================
// AGENT MODULES
// ============================================================================

/// Validated, provenance-aware agent context and context construction.
pub mod context;

/// Canonical agent request wrappers and request validation.
pub mod requests;

/// Structured agent results, workflow stages and advisory actions.
pub mod results;

/// Authentication-provider integration and operation authorization.
pub mod permissions;

/// Agent operation audit events and audit sink integration.
pub mod audit;

/// Fail-closed agent workflow compliance evaluation.
pub mod compliance;

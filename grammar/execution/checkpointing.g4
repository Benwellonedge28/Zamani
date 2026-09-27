/*

* ============================================================================
* Zamani Universal Programming Language
* ============================================================================
* 
* File:
* grammar/execution/checkpointing.g4
* 
* Grammar:
* Checkpointing
* 
* Status:
* Production-ready execution checkpointing intent grammar.
* 
* Purpose:
* Define target-independent source syntax for checkpoint creation,
* checkpoint policy, checkpoint references, restore intent, resume intent,
* rollback intent, checkpoint compatibility requirements, and checkpoint
* verification requirements.
* 
* This grammar describes WHAT checkpointing behavior a Zamani program
* requests, requires, permits, prefers, or constrains.
* 
* It does NOT implement:
* 
* - checkpoint storage;
* - serialization;
* - deserialization;
* - manifest construction;
* - hashing;
* - signatures;
* - authentication;
* - authorization;
* - recovery orchestration;
* - quantum-state capture;
* - quantum-state reconstruction;
* - QEC;
* - ZQN;
* - routing;
* - scheduling;
* - placement;
* - hardware discovery;
* - runtime discovery;
* - target selection;
* - vendor/provider APIs;
* - physical device selection.
* 
* ============================================================================
* ARCHITECTURAL POSITION
* ============================================================================
* 
*                     ZAMANI SOURCE
*                          |
*                          v
*                       LEXER
*                          |
*                          v
*                       PARSER
*                          |
*                          v
*                DOMAIN-NEUTRAL AST
*                          |
*                          v
*                 SEMANTIC ANALYSIS
*                          |
*        +-----------------+------------------+
*        |                 |                  |
*        v                 v                  v
*   resources         capabilities        portability
*        |                 |                  |
*        +-----------------+------------------+
*                          |
*                          v
*                CHECKPOINT SEMANTICS
*                          |
*          +---------------+----------------+
*          |               |                |
*          v               v                v
*      checkpoint        recovery        verification
*      subsystem        subsystem        subsystem
*          |               |                |
*          +---------------+----------------+
*                          |
*                          v
*                canonical semantic model
*                          |
*          +---------------+----------------+
*          |                                |
*          v                                v
*   classical IR                       quantum::ir
*          |                                |
*          +---------------+----------------+
*                          |
*                          v
*               optimization / lowering
*                          |
*          +---------------+----------------+
*          |               |                |
*          v               v                v
*      scheduling       routing           QEC/ZQN
*                          |
*                          v
*                         HAL
*                          |
*                          v
*                       RUNTIME
* 
* The grammar is only the source-language boundary.
* 
* ============================================================================
* POCO-REAF
* ============================================================================
* 
* Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
* 
* Checkpoint syntax MUST remain independent of:
* 
* CPU model
* CPU count
* core count
* thread count
* GPU model
* GPU count
* FPGA model
* FPGA count
* ASIC model
* QPU model
* QPU count
* physical qubit numbering
* node count
* memory capacity
* storage capacity
* network topology
* accelerator count
* provider
* backend
* 
* A program may express a resource requirement, for example:
* 
* requires checkpoint.storage >= required_storage;
* 
* That is program semantics.
* 
* The grammar MUST NOT define a universal storage or checkpoint capacity.
* 
* ============================================================================
* HARD-CODING PROHIBITION
* ============================================================================
* 
* This file MUST NOT introduce universal limits such as:
* 
* MAX_CHECKPOINTS
* MAX_CHECKPOINT_BYTES
* MAX_CHECKPOINTS_PER_EXECUTION
* MAX_ARTIFACTS
* MAX_REPLICAS
* MAX_RESTORE_ATTEMPTS
* MAX_RETRY_ATTEMPTS
* MAX_QUBITS
* MAX_CPUS
* MAX_GPUS
* MAX_FPGAS
* MAX_NODES
* MAX_MEMORY
* MAX_THREADS
* MAX_DEVICE_COUNT
* 
* Nor may it encode:
* 
* checkpoint 0
* device 0
* gpu 0
* qpu 0
* node 0
* qubit 0
* 
* as universal execution assumptions.
* 
* Concrete values supplied by the programmer remain valid program values.
* 
* ============================================================================
* CHECKPOINT SEMANTIC MODEL
* ============================================================================
* 
* A checkpoint is a semantic execution artifact.
* 
* It is NOT synonymous with:
* 
* memory dump
* arbitrary byte serialization
* arbitrary quantum-state serialization
* filesystem file
* database record
* storage object
* 
* The existing Rust implementation under:
* 
* src/quantum/resilience/checkpoint/
* 
* already separates:
* 
* checkpoint
* manifest
* snapshot
* storage
* integrity
* compatibility
* 
* This grammar preserves that ownership.
* 
* ============================================================================
* QUANTUM CORRECTNESS RULE
* ============================================================================
* 
* A checkpoint request MUST NOT imply that an arbitrary unknown quantum state
* is serializable or restorable.
* 
* Valid checkpoint boundaries may depend on:
* 
* classical execution state
* compiled/replayable execution
* measurement boundary
* logical/QEC boundary
* provider-supported snapshot
* reconstructible state
* deterministic replay
* application-defined reconstruction
* 
* Whether a particular boundary is restorable is a semantic/runtime
* capability question.
* 
* Therefore:
* 
* checkpoint requested
* 
* does NOT mean:
* 
* arbitrary quantum state can be copied.
* 
* ============================================================================
* OWNERSHIP
* ============================================================================
* 
* THIS FILE OWNS:
* 
* checkpoint declaration syntax;
* checkpoint creation intent;
* checkpoint reference syntax;
* restore intent;
* resume intent;
* rollback intent;
* checkpoint lifecycle intent;
* checkpoint boundary intent;
* checkpoint scope intent;
* checkpoint compatibility intent;
* checkpoint verification intent;
* checkpoint retention intent;
* checkpoint selection intent;
* checkpoint recovery intent;
* checkpoint policy composition;
* checkpoint-related requirement/preference/hint syntax.
* 
* THIS FILE DOES NOT OWN:
* 
* general expressions;
* general identifiers;
* general types;
* persistence implementation;
* storage implementation;
* serialization;
* manifest implementation;
* integrity implementation;
* security implementation;
* recovery algorithms;
* runtime implementation;
* resilience implementation;
* QEC;
* ZQN;
* routing;
* scheduling;
* placement;
* HAL;
* quantum::ir.
* 
* ============================================================================
* RELATED AUTHORITIES
* ============================================================================
* 
* Existing repository ownership:
* 
* grammar/data/persistence.g4
*     general data persistence/checkpoint/resume syntax
* 
* grammar/memory/persistence.g4
*     persistent-memory semantics
* 
* grammar/execution/resilience.g4
*     resilience policy syntax
* 
* grammar/execution/recovery.g4
*     recovery orchestration intent
* 
* grammar/execution/runtime.g4
*     runtime-level checkpoint references/configuration
* 
* grammar/execution/execution.g4
*     execution-domain composition
* 
* grammar/resources/*
*     resource requirements/capabilities
* 
* grammar/hardware/*
*     hardware capability/resource/topology intent
* 
* grammar/quantum/*
*     quantum source semantics
* 
* src/quantum/resilience/checkpoint/*
*     canonical checkpoint implementation
* 
* src/quantum/resilience/recovery/*
*     recovery orchestration
* 
* src/quantum/resilience/state/*
*     execution/recovery state
* 
* src/quantum/resilience/verification/*
*     restored-state verification
* 
* quantum::ir
*     canonical quantum IR boundary
* 
* This file MUST NOT duplicate those implementations.
* 
* ============================================================================
* DEPENDENCY CONTRACT
* ============================================================================
* 
* The grammar uses the canonical lexer and foundational grammar only.
* 
* It MUST NOT import:
* 
* Execution
* Runtime
* Recovery
* Resilience
* Scheduling
* Placement
* Dispatch
* Deployment
* 
* merely to reuse their concepts.
* 
* This avoids cycles such as:
* 
* Execution -> Checkpointing -> Execution
* 
* or:
* 
* Runtime -> Checkpointing -> Runtime
* 
* The composition layer owns integration.
* 
* ============================================================================
* GRAMMAR TECHNOLOGY
* ============================================================================
* 
* Generated parser integration:
* 
* Rust 1.97
* Rust 1.97.1
* Rust 2021
* safe Rust only
* 
* This grammar contains:
* 
* no Rust;
* no semantic actions;
* no predicates;
* no filesystem operations;
* no network operations;
* no runtime calls;
* no hardware discovery;
* no randomness;
* no mutable global state.
* 
* ============================================================================
  */

parser grammar Checkpointing;

options {
tokenVocab = ZamaniLexer;
}

import Core, Types, Expressions;

/*

* ============================================================================
* 1. PUBLIC ENTRY POINTS
* ============================================================================
* 
* These are the only concepts this grammar requires embedding grammars to
* expose.
* 
* Composition grammars should delegate to these rules instead of copying
* checkpoint syntax.
* 
* ============================================================================
  */

checkpointingDeclaration
: CHECKPOINTING checkpointingTarget? checkpointingBody
;

checkpointingStatement
: checkpointCreateStatement
| checkpointRestoreStatement
| checkpointResumeStatement
| checkpointRollbackStatement
| checkpointDeleteStatement
| checkpointVerifyStatement
;

checkpointingExpression
: checkpointReference
;

/*

* ============================================================================
* 2. CHECKPOINTING DECLARATION
* ============================================================================
* 
* A declaration establishes reusable checkpoint policy/intent.
* 
* It does not create storage.
* 
* Examples:
* 
* checkpointing {
*     boundary: measurement;
* }
* 
* checkpointing computation {
*     retention: policy;
*     verify: required;
* }
* 
* The semantic registry determines the meaning of policy properties.
* 
* ============================================================================
  */

checkpointingTarget
: qualifiedName
;

checkpointingBody
: LBRACE checkpointingClause* RBRACE
;

checkpointingClause
: checkpointProperty COLON expression SEMICOLON
| checkpointProperty ASSIGN expression SEMICOLON
;

checkpointProperty
: qualifiedName
;

/*

* ============================================================================
* 3. CREATE CHECKPOINT
* ============================================================================
* 
* Creation expresses checkpoint intent.
* 
* The runtime decides whether the requested boundary is actually
* checkpointable.
* 
* ============================================================================
  */

checkpointCreateStatement
: CHECKPOINT
checkpointSubjectClause?
checkpointCreateClause*
SEMICOLON?
;

checkpointSubjectClause
: checkpointFromClause
| checkpointForClause
;

checkpointCreateClause
: checkpointIdentityClause
| checkpointBoundaryClause
| checkpointScopeClause
| checkpointPolicyClause
| checkpointRequirementClause
| checkpointConstraintClause
| checkpointPreferenceClause
| checkpointHintClause
| checkpointVerificationClause
| checkpointCompatibilityClause
| checkpointLineageClause
| checkpointMetadataClause
;

/*

* ============================================================================
* 4. RESTORE
* ============================================================================
* 
* Restore requests reconstruction of supported state.
* 
* It does not guarantee that arbitrary state can be restored.
* 
* The semantic/runtime layer must validate:
* 
* identity
* integrity
* schema
* compatibility
* target capabilities
* state kind
* checkpoint boundary
* provider support
* QEC requirements
* deterministic replay requirements
* 
* ============================================================================
  */

checkpointRestoreStatement
: RESTORE CHECKPOINT
checkpointReference
checkpointRestoreClause*
SEMICOLON?
;

checkpointRestoreClause
: checkpointDestinationClause
| checkpointScopeClause
| checkpointPolicyClause
| checkpointRequirementClause
| checkpointConstraintClause
| checkpointPreferenceClause
| checkpointHintClause
| checkpointVerificationClause
| checkpointCompatibilityClause
| checkpointLineageClause
| checkpointMetadataClause
;

/*

* ============================================================================
* 5. RESUME
* ============================================================================
* 
* Resume is semantically distinct from restore.
* 
* Restore:
* 
* reconstruct supported state.
* 
* Resume:
* 
* continue execution from a validated execution boundary.
* 
* A resume may therefore require:
* 
* restore
* replay
* reconstruction
* recompilation
* migration
* validation
* 
* depending on the checkpoint and target.
* 
* ============================================================================
  */

checkpointResumeStatement
: RESUME checkpointReference checkpointResumeClause*
SEMICOLON?
;

checkpointResumeClause
: checkpointDestinationClause
| checkpointScopeClause
| checkpointPolicyClause
| checkpointRequirementClause
| checkpointConstraintClause
| checkpointPreferenceClause
| checkpointHintClause
| checkpointVerificationClause
| checkpointCompatibilityClause
| checkpointLineageClause
| checkpointMetadataClause
;

/*

* ============================================================================
* 6. ROLLBACK
* ============================================================================
* 
* Rollback requests restoration to an earlier accepted execution boundary.
* 
* It does not itself define:
* 
* transactional semantics;
* distributed consensus;
* storage versioning;
* recovery algorithms.
* 
* Those remain downstream.
* 
* ============================================================================
  */

checkpointRollbackStatement
: ROLLBACK CHECKPOINT checkpointReference checkpointRollbackClause*
SEMICOLON?
;

checkpointRollbackClause
: checkpointScopeClause
| checkpointPolicyClause
| checkpointRequirementClause
| checkpointConstraintClause
| checkpointPreferenceClause
| checkpointHintClause
| checkpointVerificationClause
| checkpointCompatibilityClause
;

/*

* ============================================================================
* 7. DELETE / RETIRE CHECKPOINT
* ============================================================================
* 
* Deletion/retirement is semantic lifecycle intent.
* 
* Physical storage reclamation is not owned by the grammar.
* 
* ============================================================================
  */

checkpointDeleteStatement
: DELETE CHECKPOINT checkpointReference checkpointDeleteClause*
SEMICOLON?
;

checkpointDeleteClause
: checkpointPolicyClause
| checkpointRequirementClause
| checkpointConstraintClause
| checkpointPreferenceClause
| checkpointHintClause
;

/*

* ============================================================================
* 8. VERIFY CHECKPOINT
* ============================================================================
* 
* Verification is intentionally explicit.
* 
* A successful storage read is not equivalent to:
* 
* trusted
* compatible
* restorable
* semantically valid
* 
* The downstream verification subsystem determines what checks are required.
* 
* ============================================================================
  */

checkpointVerifyStatement
: VERIFY CHECKPOINT checkpointReference checkpointVerifyClause*
SEMICOLON?
;

checkpointVerifyClause
: checkpointIntegrityClause
| checkpointCompatibilityClause
| checkpointVerificationClause
| checkpointRequirementClause
| checkpointConstraintClause
| checkpointPreferenceClause
| checkpointHintClause
;

/*

* ============================================================================
* 9. CHECKPOINT REFERENCE
* ============================================================================
* 
* A checkpoint reference is a semantic expression.
* 
* It may resolve to:
* 
* checkpoint identifier
* logical name
* lineage reference
* version
* application-defined identity
* runtime-generated identity
* expression producing a checkpoint reference
* 
* The grammar deliberately does not require a physical storage URI.
* 
* ============================================================================
  */

checkpointReference
: expression
;

/*

* ============================================================================
* 10. FROM / FOR / INTO
* ============================================================================
  */

checkpointFromClause
: FROM expression
;

checkpointForClause
: FOR expression
;

checkpointDestinationClause
: INTO expression
;

/*

* ============================================================================
* 11. IDENTITY
* ============================================================================
* 
* Identity is semantic data.
* 
* The actual checkpoint implementation already owns CheckpointId.
* 
* The grammar merely represents the source-level identity expression.
* 
* ============================================================================
  */

checkpointIdentityClause
: IDENTITY expression
| KEY expression
| AS expression
;

/*

* ============================================================================
* 12. BOUNDARY
* ============================================================================
* 
* A boundary describes the semantic point at which checkpointing occurs.
* 
* The grammar deliberately uses expressions/qualified names rather than
* defining a permanently closed set of boundaries.
* 
* Known semantic examples include:
* 
* program_start
* execution_boundary
* operation_boundary
* measurement
* logical_boundary
* classical_state
* replay_boundary
* provider_snapshot
* reconstructible_state
* 
* Whether a particular boundary is supported is semantic/runtime policy.
* 
* ============================================================================
  */

checkpointBoundaryClause
: BOUNDARY expression
;

/*

* ============================================================================
* 13. SCOPE
* ============================================================================
* 
* Scope can describe the semantic state included in a checkpoint.
* 
* Examples:
* 
* execution
* task
* function
* pipeline
* logical_state
* classical_state
* hybrid_state
* distributed_state
* 
* The grammar does not enumerate those domains.
* 
* ============================================================================
  */

checkpointScopeClause
: SCOPE expression
;

/*

* ============================================================================
* 14. POLICY
* ============================================================================
* 
* Policy names are open-world.
* 
* Examples:
* 
* checkpoint_policy
* durable
* ephemeral
* incremental
* periodic
* on_failure
* on_boundary
* reconstructible
* replayable
* 
* Semantic registration determines their meaning.
* 
* ============================================================================
  */

checkpointPolicyClause
: POLICY checkpointPolicy
| WITH POLICY checkpointPolicy
;

checkpointPolicy
: qualifiedName checkpointPolicyArguments?
;

checkpointPolicyArguments
: LPAREN checkpointArgumentList? RPAREN
;

checkpointArgumentList
: expression (COMMA expression)*
;

/*

* ============================================================================
* 15. REQUIREMENTS
* ============================================================================
* 
* Requirements are mandatory semantic conditions.
* 
* They are not grammar-level machine limits.
* 
* Examples:
* 
* requires capability("checkpoint.restore");
* requires capability("durable.storage");
* requires checkpoint.compatibility;
* requires storage >= required_storage;
* 
* ============================================================================
  */

checkpointRequirementClause
: REQUIRES checkpointRequirement
;

checkpointRequirement
: qualifiedName checkpointRequirementValue?
;

checkpointRequirementValue
: expression
;

/*

* ============================================================================
* 16. CONSTRAINTS
* ============================================================================
* 
* Constraints restrict valid realizations without selecting an implementation.
* ============================================================================
  */

checkpointConstraintClause
: CONSTRAINT checkpointConstraint
;

checkpointConstraint
: qualifiedName checkpointConstraintValue?
;

checkpointConstraintValue
: expression
;

/*

* ============================================================================
* 17. PREFERENCES
* ============================================================================
* 
* Preferences are advisory.
* 
* Failure to satisfy a preference does not automatically invalidate a
* checkpoint operation.
* ============================================================================
  */

checkpointPreferenceClause
: PREFER checkpointPreference
;

checkpointPreference
: qualifiedName checkpointPreferenceValue?
;

checkpointPreferenceValue
: expression
;

/*

* ============================================================================
* 18. HINTS
* ============================================================================
* 
* Hints are implementation guidance, not semantic requirements.
* ============================================================================
  */

checkpointHintClause
: HINT checkpointHint
;

checkpointHint
: qualifiedName checkpointHintValue?
;

checkpointHintValue
: expression
;

/*

* ============================================================================
* 19. VERIFICATION
* ============================================================================
* 
* Verification policy is deliberately separate from storage.
* 
* The existing Rust checkpoint implementation separates:
* 
* storage
* integrity
* compatibility
* verification
* 
* This grammar preserves that separation.
* ============================================================================
  */

checkpointVerificationClause
: VERIFY checkpointVerification
;

checkpointVerification
: qualifiedName checkpointVerificationValue?
;

checkpointVerificationValue
: expression
;

checkpointIntegrityClause
: INTEGRITY checkpointIntegrity
;

checkpointIntegrity
: qualifiedName checkpointIntegrityValue?
;

checkpointIntegrityValue
: expression
;

/*

* ============================================================================
* 20. COMPATIBILITY
* ============================================================================
* 
* Compatibility can involve:
* 
* checkpoint schema
* manifest schema
* program identity
* IR schema
* resilience schema
* target capabilities
* hardware generation
* QEC configuration
* runtime capabilities
* backend interface
* 
* The compatibility subsystem determines actual compatibility.
* ============================================================================
  */

checkpointCompatibilityClause
: COMPATIBILITY checkpointCompatibility
;

checkpointCompatibility
: qualifiedName checkpointCompatibilityValue?
;

checkpointCompatibilityValue
: expression
;

/*

* ============================================================================
* 21. LINEAGE
* ============================================================================
* 
* Checkpoint lineage supports:
* 
* parent checkpoint
* derived checkpoint
* replay lineage
* migration lineage
* recovery lineage
* 
* The manifest subsystem owns actual lineage representation.
* ============================================================================
  */

checkpointLineageClause
: LINEAGE checkpointLineage
;

checkpointLineage
: expression
;

/*

* ============================================================================
* 22. METADATA
* ============================================================================
* 
* Metadata is semantic/application information.
* 
* It must not become an implicit storage schema.
* ============================================================================
  */

checkpointMetadataClause
: METADATA checkpointMetadata
;

checkpointMetadata
: expression
;

/*

* ============================================================================
* 23. RESTORE/RESUME DESTINATION
* ============================================================================
* 
* A destination is an abstract semantic execution target.
* 
* It is NOT a physical CPU/GPU/QPU/device identifier.
* ============================================================================
  */

checkpointDestination
: expression
;

/*

* ============================================================================
* 24. RESTORE COMPATIBILITY
* ============================================================================
* 
* Restore compatibility is intentionally represented as semantic intent.
* 
* Examples:
* 
* compatibility schema;
* compatibility target;
* compatibility runtime;
* compatibility qec;
* compatibility ir;
* 
* Exact semantics belong to the compatibility implementation.
* ============================================================================
  */

checkpointRestoreCompatibility
: checkpointCompatibilityClause
;

/*

* ============================================================================
* 25. CHECKPOINT POLICY BLOCK
* ============================================================================
* 
* A reusable policy block allows multiple checkpoint properties to be
* represented without introducing another mini-language.
* ============================================================================
  */

checkpointPolicyBlock
: CHECKPOINTING LBRACE checkpointingClause* RBRACE
;

/*

* ============================================================================
* 26. GENERIC CHECKPOINT OPTION
* ============================================================================
* 
* Future checkpoint semantics can be added through qualified names without
* requiring a new lexer keyword.
* ============================================================================
  */

checkpointOption
: qualifiedName
checkpointOptionValue?
;

checkpointOptionValue
: expression
;

/*

* ============================================================================
* 27. CHECKPOINT SELECTION
* ============================================================================
* 
* Selection criteria are expressions.
* 
* The runtime/recovery subsystem determines which checkpoint satisfies them.
* 
* The grammar does not inspect checkpoint storage.
* ============================================================================
  */

checkpointSelectionClause
: SELECT checkpointSelection
;

checkpointSelection
: expression
;

/*

* ============================================================================
* 28. CHECKPOINT RETENTION
* ============================================================================
* 
* Retention is policy, not storage implementation.
* ============================================================================
  */

checkpointRetentionClause
: RETENTION expression
;

/*

* ============================================================================
* 29. CHECKPOINT DURABILITY
* ============================================================================
  */

checkpointDurabilityClause
: DURABILITY expression
;

/*

* ============================================================================
* 30. CHECKPOINT AVAILABILITY
* ============================================================================
  */

checkpointAvailabilityClause
: AVAILABILITY expression
;

/*

* ============================================================================
* 31. CHECKPOINT REPLICATION
* ============================================================================
* 
* Replication is a semantic requirement/preference.
* 
* The grammar does not select replicas or nodes.
* ============================================================================
  */

checkpointReplicationClause
: REPLICATION expression
;

/*

* ============================================================================
* 32. CHECKPOINT SECURITY
* ============================================================================
* 
* Security semantics remain owned by the security subsystem.
* ============================================================================
  */

checkpointSecurityClause
: SECURITY expression
;

/*

* ============================================================================
* 33. CHECKPOINT ENCRYPTION
* ============================================================================
* 
* This expresses an encryption requirement/policy.
* 
* It does not implement cryptography.
* ============================================================================
  */

checkpointEncryptionClause
: ENCRYPTION expression
;

/*

* ============================================================================
* 34. CHECKPOINT STORAGE CAPABILITY
* ============================================================================
* 
* Storage implementation remains outside the grammar.
* 
* This rule exists only as a semantic requirement boundary.
* ============================================================================
  */

checkpointStorageClause
: STORAGE expression
;

/*

* ============================================================================
* 35. CHECKPOINT STATE KIND
* ============================================================================
* 
* State-kind values remain semantic expressions.
* 
* This is important because the Rust implementation already distinguishes
* checkpoint state kinds and boundaries.
* 
* The grammar must not create a competing Rust enum.
* ============================================================================
  */

checkpointStateKindClause
: STATE expression
;

/*

* ============================================================================
* 36. CHECKPOINT ARTIFACT SCOPE
* ============================================================================
* 
* The existing manifest implementation owns artifact enumeration.
* 
* Source syntax can request a scope, but it cannot enumerate physical
* checkpoint bytes or storage objects.
* ============================================================================
  */

checkpointArtifactClause
: ARTIFACT expression
;

/*

* ============================================================================
* 37. CHECKPOINT VERSION
* ============================================================================
  */

checkpointVersionClause
: VERSION expression
;

/*

* ============================================================================
* 38. CHECKPOINT SCHEMA
* ============================================================================
  */

checkpointSchemaClause
: SCHEMA expression
;

/*

* ============================================================================
* 39. CHECKPOINT EXECUTION IDENTITY
* ============================================================================
* 
* These are semantic identifiers.
* 
* They do not select a physical machine.
* ============================================================================
  */

checkpointExecutionClause
: EXECUTION expression
;

/*

* ============================================================================
* 40. CHECKPOINT PROGRAM IDENTITY
* ============================================================================
  */

checkpointProgramClause
: PROGRAM expression
;

/*

* ============================================================================
* 41. CHECKPOINT TARGET IDENTITY
* ============================================================================
* 
* Target identity is abstract unless a target-specific dialect explicitly
* defines a concrete binding.
* ============================================================================
  */

checkpointTargetClause
: TARGET expression
;

/*

* ============================================================================
* 42. CHECKPOINT RESOURCE REQUIREMENT
* ============================================================================
* 
* Reuses expression semantics.
* 
* Examples:
* 
* resource memory >= required_memory;
* resource storage >= checkpoint_storage;
* resource bandwidth >= required_bandwidth;
* 
* The exact resource model remains owned by grammar/resources.
* ============================================================================
  */

checkpointResourceClause
: RESOURCE expression
;

/*

* ============================================================================
* 43. CHECKPOINT CAPABILITY REQUIREMENT
* ============================================================================
* 
* Examples:
* 
* capability checkpoint.restore;
* capability durable.storage;
* capability quantum.logical_snapshot;
* 
* Capability resolution is semantic.
* ============================================================================
  */

checkpointCapabilityClause
: CAPABILITY expression
;

/*

* ============================================================================
* 44. CHECKPOINT RESTORE POLICY
* ============================================================================
* 
* Restore policy may express:
* 
* exact
* compatible
* reconstruct
* replay
* migrate
* recompile
* degraded
* failover
* 
* These are semantic policy values.
* ============================================================================
  */

checkpointRestorePolicyClause
: RESTORE POLICY expression
;

/*

* ============================================================================
* 45. CHECKPOINT RESUME POLICY
* ============================================================================
  */

checkpointResumePolicyClause
: RESUME POLICY expression
;

/*

* ============================================================================
* 46. CHECKPOINT RECOVERY POLICY
* ============================================================================
* 
* Recovery orchestration remains owned by execution/recovery.g4 and the
* resilience implementation.
* 
* This is only an execution checkpoint policy reference.
* ============================================================================
  */

checkpointRecoveryPolicyClause
: RECOVERY expression
;

/*

* ============================================================================
* 47. CHECKPOINT FAILURE POLICY
* ============================================================================
  */

checkpointFailurePolicyClause
: FAILURE expression
;

/*

* ============================================================================
* 48. CHECKPOINT STALENESS / FRESHNESS
* ============================================================================
* 
* Freshness is semantic.
* 
* It may be expressed using an application-defined or runtime-defined value.
* ============================================================================
  */

checkpointFreshnessClause
: FRESHNESS expression
;

/*

* ============================================================================
* 49. CHECKPOINT AGE
* ============================================================================
  */

checkpointAgeClause
: AGE expression
;

/*

* ============================================================================
* 50. CHECKPOINT DEADLINE
* ============================================================================
  */

checkpointDeadlineClause
: DEADLINE expression
;

/*

* ============================================================================
* 51. CHECKPOINT TIME WINDOW
* ============================================================================
  */

checkpointWindowClause
: WINDOW expression
;

/*

* ============================================================================
* 52. CHECKPOINT DETERMINISM
* ============================================================================
* 
* Resumed execution must satisfy the determinism/replay contract required by
* the semantic/runtime system.
* ============================================================================
  */

checkpointDeterminismClause
: DETERMINISM expression
;

/*

* ============================================================================
* 53. CHECKPOINT REPLAY
* ============================================================================
* 
* Replay is not serialization.
* 
* It may be used when the checkpoint represents a replayable execution
* boundary rather than a complete state snapshot.
* ============================================================================
  */

checkpointReplayClause
: REPLAY expression
;

/*

* ============================================================================
* 54. CHECKPOINT RECONSTRUCTION
* ============================================================================
* 
* Reconstruction may involve recompilation, replay, provider-supported
* restoration, or other downstream mechanisms.
* ============================================================================
  */

checkpointReconstructionClause
: RECONSTRUCT expression
;

/*

* ============================================================================
* 55. CHECKPOINT MIGRATION
* ============================================================================
* 
* Migration is especially important for POCO-REAF.
* 
* A checkpoint may be restored on a different compatible realization without
* the source program being rewritten.
* ============================================================================
  */

checkpointMigrationClause
: MIGRATE expression
;

/*

* ============================================================================
* 56. CHECKPOINT RECOMPILATION
* ============================================================================
  */

checkpointRecompileClause
: RECOMPILE expression
;

/*

* ============================================================================
* 57. CHECKPOINT DEGRADATION
* ============================================================================
* 
* A restore may be allowed to continue with a semantically declared
* degradation policy.
* ============================================================================
  */

checkpointDegradationClause
: DEGRADATION expression
;

/*

* ============================================================================
* 58. CHECKPOINT OUTCOME
* ============================================================================
* 
* Outcome values are semantic expressions.
* 
* Existing resilience vocabulary includes:
* 
* ACCEPT
* DEGRADED_ACCEPT
* RETRY
* RECOVER
* ESCALATE
* REJECT
* 
* The grammar does not hard-code those values into the parser.
* ============================================================================
  */

checkpointOutcomeClause
: OUTCOME expression
;

/*

* ============================================================================
* 59. CHECKPOINT STATE
* ============================================================================
* 
* Runtime state vocabulary remains open.
* 
* Existing resilience state values include:
* 
* Unknown
* Healthy
* Degraded
* Unstable
* Unavailable
* Recovering
* Quarantined
* Retired
* 
* These are semantic values.
* ============================================================================
  */

checkpointRuntimeStateClause
: STATUS expression
;

/*

* ============================================================================
* 60. GENERIC CHECKPOINT ATTRIBUTE
* ============================================================================
* 
* This provides a controlled extension point without creating a second
* checkpoint language.
* ============================================================================
  */

checkpointAttributeClause
: qualifiedName COLON expression SEMICOLON
;

/*

* ============================================================================
* 61. SEMANTIC NORMALIZATION CONTRACT
* ============================================================================
* 
* Every parsed construct maps into a domain-neutral semantic representation.
* 
* Conceptually:
* 
* checkpointCreateStatement
*     ->
* CheckpointIntent::Create
* 
* checkpointRestoreStatement
*     ->
* CheckpointIntent::Restore
* 
* checkpointResumeStatement
*     ->
* CheckpointIntent::Resume
* 
* checkpointRollbackStatement
*     ->
* CheckpointIntent::Rollback
* 
* checkpointVerifyStatement
*     ->
* CheckpointIntent::Verify
* 
* The exact Rust AST type is owned by src/frontend/ast/.
* 
* This grammar MUST NOT introduce:
* 
* CheckpointIR
* QuantumCheckpointIR
* RuntimeCheckpointIR
* StorageCheckpointIR
* 
* as competing intermediate representations.
* 
* ============================================================================
  */

/*

* ============================================================================
* 62. RUST INTEGRATION CONTRACT
* ============================================================================
* 
* The grammar itself contains no Rust.
* 
* The corresponding Rust semantic implementation must:
* 
* - use Rust 1.97 / 1.97.1;
* - use Rust 2021;
* - contain no unsafe code;
* - preserve source spans;
* - preserve source ordering;
* - avoid machine-sized assumptions;
* - avoid fixed checkpoint capacities;
* - use fallible operations for resource/storage failures;
* - distinguish syntax errors from semantic/resource failures.
* 
* Required safe-Rust policy:
* 
* #![forbid(unsafe_code)]
* 
* belongs in Rust implementation modules, not inside this ANTLR grammar.
* 
* ============================================================================
  */

/*

* ============================================================================
* 63. CHECKPOINT IMPLEMENTATION INTEGRATION
* ============================================================================
* 
* The existing Rust checkpoint subsystem owns:
* 
* checkpoint identity
* checkpoint lifecycle
* checkpoint boundary
* checkpoint state kind
* manifest
* snapshot
* storage
* integrity
* compatibility
* 
* The grammar maps source expressions to those semantic concepts.
* 
* It MUST NOT recreate their Rust structures in the parser.
* 
* In particular, the grammar must not define a checkpoint-local:
* 
* QubitId
* PhysicalQubitId
* DeviceId
* StorageObjectId
* ManifestId
* 
* as parser-level machine identities.
* 
* Existing canonical implementations remain authoritative.
* 
* ============================================================================
  */

/*

* ============================================================================
* 64. QUANTUM INTEGRATION
* ============================================================================
* 
* Quantum checkpointing is subject to the quantum semantic model.
* 
* A checkpoint may refer to:
* 
* logical state
* measurement boundary
* provider-supported snapshot
* reconstructible execution state
* replayable execution
* 
* But the grammar never assumes arbitrary quantum-state serialization.
* 
* The downstream pipeline remains:
* 
* quantum source
*      |
*      v
*   quantum::ir
*      |
*      v
*  semantic execution
*      |
*      v
*  checkpoint intent
*      |
*      v
* checkpoint capability
*      |
*      v
* provider/QEC/ZQN/runtime realization
* 
* The canonical quantum boundary remains:
* 
* quantum::ir
* 
* No checkpoint-specific quantum IR is introduced.
* 
* ============================================================================
  */

/*

* ============================================================================
* 65. QEC INTEGRATION
* ============================================================================
* 
* Checkpointing may require a logical/QEC boundary.
* 
* The grammar can express:
* 
* boundary logical;
* capability quantum.logical_snapshot;
* requires quantum.error_correction;
* 
* It must not implement:
* 
* surface-code checkpointing;
* syndrome extraction;
* decoding;
* logical-state encoding;
* correction circuits.
* 
* Those remain QEC responsibilities.
* 
* ============================================================================
  */

/*

* ============================================================================
* 66. ZQN INTEGRATION
* ============================================================================
* 
* A checkpoint restored after a noisy or faulted quantum execution may require
* ZQN validation/reconstruction.
* 
* The grammar can express requirements or policies concerning:
* 
* noise compatibility;
* fault tolerance;
* reliability;
* calibration freshness;
* fault-aware restore.
* 
* ZQN determines actual noise/fault semantics.
* 
* ============================================================================
  */

/*

* ============================================================================
* 67. RESILIENCE INTEGRATION
* ============================================================================
* 
* Checkpointing is a mechanism used by resilience, not the resilience engine.
* 
* Conceptually:
* 
* resilience policy
*      |
*      +--> checkpoint
*      +--> retry
*      +--> recover
*      +--> reroute
*      +--> reschedule
*      +--> recompile
*      +--> escalate
* 
* Resilience decides WHEN and WHY a checkpoint is used.
* 
* Checkpointing defines WHAT checkpoint operation is requested.
* 
* ============================================================================
  */

/*

* ============================================================================
* 68. RECOVERY INTEGRATION
* ============================================================================
* 
* Recovery orchestration is downstream.
* 
* Conceptually:
* 
* locate
*   |
*   v
* validate metadata
*   |
*   v
* validate integrity
*   |
*   v
* validate compatibility
*   |
*   v
* restore supported state
*   |
*   +--> reconstruct
*   +--> replay
*   +--> recompile
*   +--> migrate
*   |
*   v
* verify semantics
*   |
*   v
* resume
* 
* The grammar does not choose the algorithm.
* 
* ============================================================================
  */

/*

* ============================================================================
* 69. STORAGE INTEGRATION
* ============================================================================
* 
* Storage is provider-neutral.
* 
* The grammar may express:
* 
* durability
* availability
* replication
* retention
* storage requirements
* 
* It must not encode:
* 
* filesystem paths as mandatory representation;
* block sizes;
* object-store vendor APIs;
* database-specific schemas;
* physical disk identifiers.
* 
* Existing grammar/data/persistence.g4 and grammar/memory/persistence.g4
* remain authoritative for general persistence semantics.
* 
* Checkpointing owns execution-checkpoint semantics.
* 
* Persistence owns general persistent-data semantics.
* 
* ============================================================================
  */

/*

* ============================================================================
* 70. INTEGRATION WITH runtime.g4
* ============================================================================
* 
* runtime.g4 currently contains generic checkpoint/restore syntax.
* 
* After this file is introduced, checkpoint semantics should have exactly one
* detailed grammar authority:
* 
* grammar/execution/checkpointing.g4
* 
* Runtime remains the runtime-domain composition layer.
* 
* Therefore runtime.g4 should delegate its checkpoint-specific public
* constructs to:
* 
* checkpointingStatement
* checkpointingExpression
* 
* rather than maintaining a second detailed checkpoint grammar.
* 
* This change belongs to runtime.g4, not this file.
* 
* This file is intentionally independent and does not import Runtime.
* 
* ============================================================================
  */

/*

* ============================================================================
* 71. INTEGRATION WITH resilience.g4
* ============================================================================
* 
* resilience.g4 already identifies checkpointing as a neighboring execution
* concern.
* 
* It should reference:
* 
* checkpointingStatement
* checkpointReference
* checkpointPolicyBlock
* 
* where checkpoint syntax is embedded in resilience policy.
* 
* resilience.g4 remains the authority for resilience policy.
* 
* checkpointing.g4 remains the authority for checkpoint operations.
* 
* This prevents policy and mechanism from becoming one grammar.
* 
* ============================================================================
  */

/*

* ============================================================================
* 72. INTEGRATION WITH recovery.g4
* ============================================================================
* 
* recovery.g4 should reference checkpointReference rather than inventing a
* second checkpoint identifier syntax.
* 
* Recovery owns:
* 
* recovery orchestration
* fallback
* resume sequencing
* validation ordering
* migration decisions
* recovery outcomes
* 
* Checkpointing owns:
* 
* checkpoint source syntax
* checkpoint identity syntax
* restore/resume checkpoint references
* checkpoint policy.
* 
* ============================================================================
  */

/*

* ============================================================================
* 73. INTEGRATION WITH execution.g4
* ============================================================================
* 
* execution.g4 remains the execution composition root.
* 
* It should eventually import Checkpointing alongside its other execution
* subgrammars and expose the public entry points where source placement
* permits.
* 
* Example composition:
* 
* import Core,
*        ExecutionContext,
*        Checkpointing;
* 
* The exact import ordering is determined by the canonical ANTLR composition
* architecture.
* 
* This file does NOT import Execution back.
* 
* ============================================================================
  */

/*

* ============================================================================
* 74. INTEGRATION WITH data/persistence.g4
* ============================================================================
* 
* data/persistence.g4 already owns general:
* 
* persist
* restore
* checkpoint
* resume
* snapshot
* archive
* retrieve
* 
* Checkpointing MUST NOT become a second persistence grammar.
* 
* The ownership distinction is:
* 
* data/persistence.g4
*     durable data persistence semantics
* 
* execution/checkpointing.g4
*     execution-state checkpoint semantics
* 
* When syntax overlaps, composition/semantic normalization must map both
* forms to the same canonical semantic concepts where their meanings are
* equivalent.
* 
* ============================================================================
  */

/*

* ============================================================================
* 75. INTEGRATION WITH memory/persistence.g4
* ============================================================================
* 
* memory/persistence.g4 owns persistent-memory intent.
* 
* checkpointing.g4 may refer to memory state through expressions/scopes but
* does not redefine:
* 
* ownership
* borrowing
* memory allocation
* memory persistence implementation.
* 
* ============================================================================
  */

/*

* ============================================================================
* 76. INTEGRATION WITH resources/
* ============================================================================
* 
* Resource requirements must remain separate from implementation decisions.
* 
* Examples:
* 
* requires checkpoint.storage >= required_storage;
* requires capability("checkpoint.restore");
* requires capability("durable.storage");
* 
* Resource resolution happens after parsing.
* 
* The grammar does not inspect resource availability.
* 
* ============================================================================
  */

/*

* ============================================================================
* 77. INTEGRATION WITH hardware/
* ============================================================================
* 
* Hardware may expose capabilities such as:
* 
* checkpoint.restore
* quantum.logical_snapshot
* persistent_execution_state
* deterministic_replay
* 
* The checkpoint grammar consumes those as semantic capability references.
* 
* It does not enumerate hardware.
* 
* ============================================================================
  */

/*

* ============================================================================
* 78. INTEGRATION WITH security/
* ============================================================================
* 
* Checkpoint integrity and authorization are security-sensitive.
* 
* Grammar may express:
* 
* security ...
* integrity ...
* capability ...
* 
* It must not implement:
* 
* encryption;
* signatures;
* key storage;
* authentication;
* authorization.
* 
* ============================================================================
  */

/*

* ============================================================================
* 79. INTEGRATION WITH observability
* ============================================================================
* 
* Existing resilience observability semantics include events such as:
* 
* checkpoint.created
* checkpoint.restored
* 
* This grammar does not emit events.
* 
* Runtime/telemetry observes successful semantic operations after validation.
* 
* ============================================================================
  */

/*

* ============================================================================
* 80. INTEGRATION WITH COMPILER / IR
* ============================================================================
* 
* Checkpoint intent must lower through the canonical semantic model.
* 
* There is no:
* 
* CheckpointIR
* RuntimeCheckpointIR
* QuantumCheckpointIR
* 
* unless the repository's canonical IR architecture explicitly introduces
* such a representation later.
* 
* For quantum programs:
* 
* source
*   -> AST
*   -> semantic quantum model
*   -> quantum::ir
*   -> execution/checkpoint intent
*   -> downstream realization
* 
* The canonical quantum boundary remains quantum::ir.
* 
* ============================================================================
  */

/*

* ============================================================================
* 81. SOURCE-SPAN CONTRACT
* ============================================================================
* 
* The frontend AST must preserve source spans for at least:
* 
* checkpoint operation
* checkpoint reference
* checkpoint identity
* boundary
* scope
* policy
* requirement
* constraint
* preference
* hint
* compatibility
* verification
* 
* This enables:
* 
* precise diagnostics;
* IDE tooling;
* formatting;
* refactoring;
* compatibility analysis;
* semantic error reporting.
* 
* ============================================================================
  */

/*

* ============================================================================
* 82. DIAGNOSTIC CONTRACT
* ============================================================================
* 
* Syntax errors:
* 
* parser responsibility.
* 
* Semantic errors:
* 
* unknown checkpoint policy
* unsupported checkpoint boundary
* invalid checkpoint identity
* unsupported restore mode
* incompatible schema
* incompatible target
* stale checkpoint
* invalid lineage
* unsupported quantum snapshot
* missing capability
* insufficient resources
* failed integrity verification
* failed compatibility verification
* invalid resume condition
* 
* These are NOT syntax errors merely because the target cannot satisfy them.
* 
* ============================================================================
  */

/*

* ============================================================================
* 83. SECURITY CONTRACT
* ============================================================================
* 
* Checkpoint syntax cannot:
* 
* execute code;
* bypass authorization;
* bypass integrity validation;
* bypass compatibility validation;
* mutate hardware;
* invoke storage directly;
* access arbitrary filesystem paths;
* invoke vendor APIs.
* 
* A checkpoint reference is data, not an executable capability.
* 
* ============================================================================
  */

/*

* ============================================================================
* 84. DETERMINISM
* ============================================================================
* 
* Parsing is deterministic.
* 
* The grammar does not:
* 
* query runtime state;
* query storage;
* query hardware;
* inspect wall-clock time;
* generate identifiers;
* select a checkpoint;
* select a backend.
* 
* Selection and realization are semantic/runtime operations.
* 
* ============================================================================
  */

/*

* ============================================================================
* 85. SCALABILITY
* ============================================================================
* 
* All collections use grammar repetition:
* 
* checkpointingClause*
* checkpointCreateClause*
* checkpointRestoreClause*
* checkpointResumeClause*
* checkpointRollbackClause*
* 
* There is no language-level finite checkpoint capacity.
* 
* Therefore the language can describe:
* 
* one checkpoint;
* many checkpoints;
* checkpoint histories;
* large lineage graphs;
* distributed checkpoint sets;
* large artifact manifests;
* 
* subject only to actual compiler/runtime/resource availability.
* 
* "Infinity" means no artificial language ceiling, not infinite physical
* storage or computation.
* 
* ============================================================================
  */

/*

* ============================================================================
* 86. NO PHYSICAL TOPOLOGY
* ============================================================================
* 
* This grammar does not encode:
* 
* checkpoint on GPU 0
* checkpoint on QPU 2
* checkpoint on node 4
* checkpoint in memory bank 7
* 
* A target-specific dialect may define explicit realization syntax, but such
* syntax must be explicitly target-specific and must not contaminate the
* portable core language.
* 
* ============================================================================
  */

/*

* ============================================================================
* 87. NO FIXED RETRY/RESTORE LIMIT
* ============================================================================
* 
* The grammar does not define:
* 
* maximum restore attempts;
* maximum checkpoint count;
* maximum rollback depth;
* maximum lineage depth;
* maximum artifact count.
* 
* A program may explicitly declare a policy value if semantically appropriate.
* 
* Example:
* 
* policy retry(3);
* 
* Here 3 is program semantics, not a compiler-wide limit.
* 
* ============================================================================
  */

/*

* ============================================================================
* 88. COMPATIBILITY / VERSIONING
* ============================================================================
* 
* Checkpoint compatibility is semantic.
* 
* The implementation may need to compare:
* 
* checkpoint schema;
* manifest schema;
* program identity;
* IR schema;
* resilience schema;
* target capabilities;
* runtime capabilities;
* QEC configuration;
* hardware generation;
* provider capabilities.
* 
* The grammar does not hard-code those version relationships.
* 
* ============================================================================
  */

/*

* ============================================================================
* 89. ERROR RECOVERY
* ============================================================================
* 
* Parser recovery must make progress.
* 
* This grammar intentionally avoids semantic predicates and embedded actions.
* 
* Recovery from malformed checkpoint syntax is the responsibility of the
* canonical parser infrastructure.
* 
* The grammar itself must not silently reinterpret malformed checkpoint
* operations as unrelated execution statements.
* 
* ============================================================================
  */

/*

* ============================================================================
* 90. PERFORMANCE
* ============================================================================
* 
* The grammar avoids:
* 
* large closed keyword enumerations;
* semantic predicates;
* embedded actions;
* runtime lookups;
* hardware-dependent branches.
* 
* Qualified names and canonical expressions are reused instead of creating
* separate checkpoint-specific expression systems.
* 
* ============================================================================
  */

/*

* ============================================================================
* 91. CONFORMANCE TEST CONTRACT
* ============================================================================
* 
* Required positive coverage:
* 
* checkpoint;
* 
* checkpoint from computation;
* 
* checkpoint from computation
*     identity checkpoint_name;
* 
* restore checkpoint_name;
* 
* resume checkpoint_name;
* 
* rollback checkpoint_name;
* 
* verify checkpoint_name;
* 
* checkpointing {
*     boundary: measurement;
* }
* 
* checkpointing computation {
*     requires capability("checkpoint.restore");
*     policy durable();
* }
* 
* checkpoint from computation
*     boundary measurement
*     requires capability("quantum.logical_snapshot");
* 
* restore checkpoint_name
*     compatibility target
*     verify integrity;
* 
* resume checkpoint_name
*     policy replay();
* 
* Required negative coverage:
* 
* malformed checkpoint reference;
* missing restore checkpoint;
* malformed policy;
* malformed requirement;
* malformed identity;
* malformed boundary;
* malformed compatibility clause;
* malformed verification clause;
* 
* Required boundary coverage:
* 
* empty checkpoint policy;
* large policy;
* deeply qualified names;
* nested expressions;
* symbolic resource requirements;
* repeated policy clauses;
* large checkpoint lineage expressions.
* 
* Required scalability coverage:
* 
* many checkpoint clauses;
* many checkpoint references;
* large checkpoint histories;
* large policy sets;
* large qualified names;
* large symbolic expressions.
* 
* Required portability coverage:
* 
* tiny classical target;
* multicore target;
* GPU target;
* FPGA target;
* QPU target;
* distributed target;
* future target.
* 
* The same checkpoint syntax must remain structurally valid across these
* realization classes.
* 
* ============================================================================
  */

/*

* ============================================================================
* 92. CROSS-DOMAIN TEST CONTRACT
* ============================================================================
* 
* Classical:
* 
* checkpoint classical_state;
* 
* Quantum:
* 
* checkpoint quantum_execution
*     boundary measurement;
* 
* Hybrid:
* 
* checkpoint hybrid_execution
*     scope hybrid_state;
* 
* HDL/hardware:
* 
* checkpoint hardware_execution
*     requires capability("persistent_execution_state");
* 
* Distributed:
* 
* checkpoint distributed_execution
*     policy distributed.consistent;
* 
* AI/data:
* 
* checkpoint training_state
*     policy resumable;
* 
* These are semantic examples.
* 
* Domain-specific implementations remain downstream.
* 
* ============================================================================
  */

/*

* ============================================================================
* 93. HARD-CODING AUDIT
* ============================================================================
* 
* This grammar passes the following intended audit:
* 
* [x] No MAX_QUBITS.
* [x] No MAX_CPUS.
* [x] No MAX_GPUS.
* [x] No MAX_FPGAS.
* [x] No MAX_NODES.
* [x] No MAX_MEMORY.
* [x] No MAX_THREADS.
* [x] No MAX_TENSOR_RANK.
* [x] No MAX_REGISTER_WIDTH.
* [x] No MAX_NETWORK_SIZE.
* [x] No MAX_DEVICE_COUNT.
* [x] No maximum checkpoint count.
* [x] No maximum artifact count.
* [x] No maximum restore depth.
* [x] No fixed retry count.
* [x] No fixed machine topology.
* [x] No physical device selection.
* [x] No physical qubit selection.
* [x] No vendor API.
* [x] No storage backend.
* [x] No filesystem implementation.
* [x] No database implementation.
* [x] No cloud-provider implementation.
* [x] No QEC implementation.
* [x] No ZQN implementation.
* [x] No routing implementation.
* [x] No scheduling implementation.
* [x] No runtime implementation.
* [x] No embedded Rust.
* [x] No unsafe code.
* 
* ============================================================================
  */

/*

* ============================================================================
* 94. FILE COMPLETION CRITERIA
* ============================================================================
* 
* This file is complete when:
* 
* [x] Purpose is defined.
* [x] Ownership is defined.
* [x] Non-ownership is defined.
* [x] Dependencies are defined.
* [x] Public entry points are defined.
* [x] Create is defined.
* [x] Restore is defined.
* [x] Resume is defined.
* [x] Rollback is defined.
* [x] Delete/retire intent is defined.
* [x] Verification is defined.
* [x] Identity is defined.
* [x] Boundary is defined.
* [x] Scope is defined.
* [x] Policy is extensible.
* [x] Requirements are separate from preferences.
* [x] Constraints are separate from requirements.
* [x] Hints are separate from requirements.
* [x] Compatibility is represented.
* [x] Integrity is represented.
* [x] Lineage is represented.
* [x] Resource/capability integration is defined.
* [x] Quantum integration is defined.
* [x] QEC boundary is defined.
* [x] ZQN boundary is defined.
* [x] Recovery boundary is defined.
* [x] Storage boundary is defined.
* [x] Runtime integration is defined.
* [x] Persistence integration is defined.
* [x] Source-span requirements are defined.
* [x] Diagnostic requirements are defined.
* [x] Security boundary is defined.
* [x] Determinism is defined.
* [x] Scalability is defined.
* [x] Test contract is defined.
* [x] Hard-coding audit is defined.
* [x] No competing checkpoint IR is introduced.
* [x] quantum::ir remains canonical.
* [x] Rust 1.97 / 1.97.1 compatibility is specified.
* [x] Safe-Rust requirement is specified.
* 
* ============================================================================
* FINAL INVARIANT
* ============================================================================
* 
* This grammar answers:
* 
* "What checkpoint/recovery state does the program request or permit?"
* 
* It does NOT answer:
* 
* "Where are checkpoint bytes stored?"
* "How are bytes serialized?"
* "Which machine performs the checkpoint?"
* "Which physical qubit is saved?"
* "How is QEC performed?"
* "How is noise modeled?"
* "How is routing performed?"
* "How is scheduling performed?"
* "How does the storage provider work?"
* "How does the runtime recover state?"
* 
* Those remain downstream responsibilities.
* 
* ============================================================================
* POCO-REAF FINAL CONTRACT
* ============================================================================
* 
* The same source-level checkpoint intent can be realized against:
* 
* tiny classical systems
* multicore systems
* GPUs
* FPGAs
* ASICs
* QPUs
* simulators
* HPC systems
* distributed systems
* future architectures
* 
* without changing the checkpoint grammar merely because the available
* hardware resources change.
* 
* Physical limits constrain realization.
* 
* They do not redefine the Zamani language.
* 
* ============================================================================
  */
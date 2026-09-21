/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/effects/capabilities.g4
 *
 * Grammar:
 *     EffectCapabilities
 *
 * Status:
 *     Canonical production parser component for the relationship between
 *     effects and source-level capability requirements.
 *
 * Language:
 *     Zamani
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Rust safety:
 *     The Zamani implementation MUST use safe Rust only.
 *     This grammar contains no Rust actions and requires no `unsafe`.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file answers exactly one syntactic question:
 *
 *     "Which source-level capabilities are required by an effect or
 *      effect operation?"
 *
 * It does NOT define the language-wide capability system.
 *
 * Canonical capability ownership remains:
 *
 *     grammar/core/capabilities.g4
 *
 * That file owns:
 *
 *     - capability declarations;
 *     - capability identities;
 *     - capability references;
 *     - capability version requirements;
 *     - capability expression syntax used by the general capability system;
 *     - capability aliases and capability metadata.
 *
 * This file owns only the EFFECT-SIDE relationship.
 *
 * ============================================================================
 * ARCHITECTURAL SEPARATION
 * ============================================================================
 *
 * EFFECT
 *
 *     Describes a semantic computational interaction.
 *
 * CAPABILITY
 *
 *     Describes an ability/facility/property that an execution context may
 *     provide.
 *
 * EFFECT CAPABILITY REQUIREMENT
 *
 *     Describes which capabilities are required for an effect realization.
 *
 * RESOURCE
 *
 *     Describes computational capacity or consumable/allocatable resources.
 *
 * REQUIREMENT
 *
 *     Describes a condition that must be satisfied.
 *
 * CONSTRAINT
 *
 *     Describes conditions imposed on a valid realization.
 *
 * PREFERENCE
 *
 *     Describes which otherwise-valid realization is preferred.
 *
 * HINT
 *
 *     Provides non-binding implementation guidance.
 *
 * TARGET
 *
 *     Describes an execution context/target abstraction.
 *
 * PLACEMENT
 *
 *     Describes where/how a realization may be placed.
 *
 * PERFORMANCE
 *
 *     Describes performance-related requirements, constraints or preferences.
 *
 * These concepts MUST NOT be silently collapsed.
 *
 * In particular:
 *
 *     requires {
 *         quantum::measurement
 *     }
 *
 * MUST NOT mean:
 *
 *     use physical QPU X
 *     use physical qubit N
 *     use topology Y
 *     use backend Z
 *     use a fixed number of qubits
 *
 * Hardware discovery, resource allocation, routing, scheduling, calibration,
 * target selection, deployment and backend selection are downstream concerns.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * The grammar is intentionally open-world.
 *
 * A capability requirement may remain unchanged when the same program is
 * realized on:
 *
 *     - a tiny embedded processor;
 *     - one CPU;
 *     - many CPUs;
 *     - one GPU;
 *     - many GPUs;
 *     * an FPGA;
 *     * an ASIC;
 *     * a quantum processor;
 *     * a quantum simulator;
 *     * a heterogeneous accelerator;
 *     * a distributed system;
 *     * an HPC system;
 *     * a cloud deployment;
 *     * a future computational substrate.
 *
 * This grammar MUST NOT encode:
 *
 *     MAX_CAPABILITIES
 *     MAX_EFFECT_CAPABILITIES
 *     MAX_EFFECTS
 *     MAX_OPERATIONS
 *     MAX_QUBITS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_DEVICES
 *
 * or any equivalent machine-specific ceiling.
 *
 * ============================================================================
 * OPEN-WORLD CAPABILITIES
 * ============================================================================
 *
 * Capability identities are deliberately not enumerated here.
 *
 * Valid examples include:
 *
 *     quantum::measurement
 *     quantum::mid_circuit_measurement
 *     quantum::dynamic_control
 *     accelerator::tensor
 *     distributed::consensus
 *     hardware::reconfigurable_logic
 *     future::photonic::interaction
 *
 * This file MUST NOT introduce domain-specific enumerations such as:
 *
 *     quantumCapability
 *     cpuCapability
 *     gpuCapability
 *     fpgaCapability
 *     qpuCapability
 *     vendorCapability
 *
 * New capabilities therefore do not require a grammar change merely because
 * a new computational domain, device family, accelerator or future technology
 * is introduced.
 *
 * ============================================================================
 * CANONICAL DEPENDENCY DIRECTION
 * ============================================================================
 *
 *     source
 *       |
 *       v
 *     grammar/antlr/ZamaniLexer.g4
 *       |
 *       v
 *     parser grammars
 *       |
 *       +--> core/names.g4
 *       +--> core/attributes.g4
 *       +--> core/capabilities.g4
 *       |
 *       v
 *     effects/capabilities.g4
 *       |
 *       v
 *     effect declarations / effect operations
 *       |
 *       v
 *     domain-neutral frontend AST
 *       |
 *       v
 *     semantic analysis
 *       |
 *       +--> effect analysis
 *       +--> capability analysis
 *       +--> requirement analysis
 *       +--> resource analysis
 *       +--> target analysis
 *       |
 *       v
 *     canonical semantic representation
 *       |
 *       +--> classical IR
 *       +--> quantum::ir
 *       +--> HDL / hardware representation
 *       +--> other domain representations
 *       |
 *       v
 *     optimization / lowering
 *       |
 *       +--> routing
 *       +--> scheduling
 *       +--> resilience
 *       +--> QEC
 *       +--> ZQN
 *       +--> HAL
 *       |
 *       v
 *     target realization
 *
 * This file MUST NOT reverse that dependency direction.
 *
 * ============================================================================
 * LEXICAL AUTHORITY
 * ============================================================================
 *
 * The production lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Parser grammars consume:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * They MUST NOT use:
 *
 *     tokenVocab = ZamaniTokens;
 *
 * directly.
 *
 * The current repository's canonical lexer exposes the following relevant
 * tokens:
 *
 *     REQUIRES
 *     AND
 *     OR
 *     LBRACE
 *     RBRACE
 *     LPAREN
 *     RPAREN
 *     COMMA
 *     SEMICOLON
 *
 * This file therefore deliberately does NOT use obsolete names such as:
 *
 *     K_REQUIRES
 *     K_AND
 *     K_OR
 *     LEFT_BRACE
 *     RIGHT_BRACE
 *
 * ============================================================================
 * CAPABILITY OWNERSHIP
 * ============================================================================
 *
 * grammar/core/capabilities.g4 remains the sole owner of:
 *
 *     capabilityReference
 *     capabilityName
 *     capabilityVersionClause
 *     capabilityVersionExpression
 *
 * This file consumes those rules.
 *
 * It MUST NOT redefine:
 *
 *     identifier
 *     qualifiedName
 *     capabilityName
 *     capabilityReference
 *     capabilityVersionClause
 *     capability version syntax
 *
 * This prevents capability identity/version drift between:
 *
 *     requirements
 *     constraints
 *     preferences
 *     effects
 *     hardware
 *     quantum
 *     distributed
 *     AI
 *     networking
 *     security
 *     future domains
 *
 * ============================================================================
 * RESOURCE/CAPABILITY SEPARATION
 * ============================================================================
 *
 * This grammar represents capability properties only.
 *
 * It MUST NOT parse resource quantities as part of the capability relation.
 *
 * Therefore constructs such as:
 *
 *     requires {
 *         8 gpu
 *     }
 *
 *     requires {
 *         64 qubits
 *     }
 *
 *     requires {
 *         32 cores
 *     }
 *
 * do NOT belong to this file.
 *
 * Resource quantities belong to the resource/requirements/constraint
 * subsystems.
 *
 * Capability:
 *
 *     quantum::mid_circuit_measurement
 *
 * Resource:
 *
 *     an available quantum execution capacity
 *
 * Requirement:
 *
 *     a semantic condition on that capacity
 *
 * Target:
 *
 *     the eventual execution context
 *
 * These remain separate all the way through semantic analysis.
 *
 * ============================================================================
 * CAPABILITY VS AUTHORIZATION
 * ============================================================================
 *
 * A capability requirement is NOT a security authorization grant.
 *
 * For example:
 *
 *     requires {
 *         security::trusted_execution
 *     };
 *
 * means that the semantic capability is required.
 *
 * It does NOT:
 *
 *     - grant permission;
 *     - acquire credentials;
 *     - bypass policy;
 *     - authenticate an identity;
 *     - authorize an operation.
 *
 * Security authorization remains owned by the security subsystem.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Quantum capabilities are ordinary open-world capability references.
 *
 * Examples:
 *
 *     quantum::measurement
 *     quantum::readout
 *     quantum::dynamic_control
 *     quantum::mid_circuit_measurement
 *     quantum::logical_qubits
 *     quantum::reset
 *
 * This file MUST NOT define:
 *
 *     QubitId
 *     PhysicalQubitId
 *     GateKind
 *     quantum::ir
 *     topology
 *     calibration
 *     routing
 *     scheduling
 *     QEC
 *     ZQN
 *
 * Correct quantum lowering remains:
 *
 *     source
 *       |
 *       v
 *     generic frontend AST
 *       |
 *       v
 *     semantic effect/capability model
 *       |
 *       v
 *     quantum semantic analysis
 *       |
 *       v
 *     quantum::ir
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     routing / scheduling
 *       |
 *       v
 *     QEC / resilience / ZQN
 *       |
 *       v
 *     HAL
 *       |
 *       v
 *     target realization
 *
 * This grammar never constructs quantum::ir.
 *
 * ============================================================================
 * HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * Hardware capabilities remain names:
 *
 *     hardware::clocked_logic
 *     hardware::reconfigurable_logic
 *     hardware::pipeline
 *     accelerator::tensor
 *
 * No physical implementation is selected here.
 *
 * The grammar MUST NOT encode:
 *
 *     device identifiers
 *     FPGA part numbers
 *     CPU identifiers
 *     GPU identifiers
 *     QPU identifiers
 *     physical addresses
 *     fixed topology
 *     fixed clock rates
 *     fixed memory sizes
 *
 * ============================================================================
 * DISTRIBUTED BOUNDARY
 * ============================================================================
 *
 * Distributed capabilities may include:
 *
 *     distributed::communication
 *     distributed::consensus
 *     distributed::replication
 *     distributed::fault_tolerance
 *     distributed::remote_execution
 *
 * This file does not encode:
 *
 *     node count
 *     node addresses
 *     cluster topology
 *     region
 *     placement
 *     deployment
 *
 * ============================================================================
 * EFFECT DECLARATION INTEGRATION
 * ============================================================================
 *
 * effect-declarations.g4 remains the owner of:
 *
 *     effectDeclaration
 *     effectOperationDeclaration
 *
 * It consumes:
 *
 *     effectCapabilityClause
 *
 * where an effect or effect operation accepts capability requirements.
 *
 * Conceptually:
 *
 *     effect Foo
 *         requires {
 *             capability::one
 *             and capability::two
 *         }
 *         {
 *             ...
 *         }
 *
 * The exact declaration/body syntax remains owned by
 * effects/effect-declarations.g4.
 *
 * This file does NOT reproduce effect declarations.
 *
 * ============================================================================
 * EFFECT SET SEPARATION
 * ============================================================================
 *
 * An effect set answers:
 *
 *     "Which effects are associated with this computation?"
 *
 * An effect capability clause answers:
 *
 *     "Which capabilities are required to realize this effect?"
 *
 * Therefore:
 *
 *     effect set != capability requirement
 *
 * A capability requirement MUST NOT silently become an effect.
 *
 * ============================================================================
 * EFFECT OPERATION INTEGRATION
 * ============================================================================
 *
 * An effect can contain operations with different capability requirements.
 *
 * Example:
 *
 *     effect QuantumIO {
 *         operation measure(...)
 *             requires {
 *                 quantum::measurement
 *                 and quantum::readout
 *             };
 *
 *         operation reset(...)
 *             requires {
 *                 quantum::reset
 *             };
 *     }
 *
 * This file records only the capability-expression syntax.
 *
 * Semantic analysis determines:
 *
 *     - whether the referenced capabilities exist;
 *     - whether versions are satisfiable;
 *     - whether combinations are valid;
 *     - whether the operation's effect semantics permit them;
 *     - whether the execution context provides them.
 *
 * ============================================================================
 * CAPABILITY EXPRESSION MODEL
 * ============================================================================
 *
 * The effect-specific requirement expression supports:
 *
 *     capability
 *
 *     capability and capability
 *
 *     capability or capability
 *
 *     capability and (capability or capability)
 *
 * The expression is syntactic.
 *
 * It does NOT evaluate target availability.
 *
 * It does NOT select an implementation.
 *
 * It does NOT perform capability negotiation.
 *
 * It does NOT authorize anything.
 *
 * ============================================================================
 * WHY NEGATION IS NOT ACCEPTED HERE
 * ============================================================================
 *
 * The general capability grammar supports capability predicates, including
 * negation, for contexts where predicate semantics are appropriate.
 *
 * An effect requirement is narrower:
 *
 *     requires capability
 *
 * means the capability must be available.
 *
 * A construct such as:
 *
 *     requires not capability
 *
 * changes the meaning from a required ability to an absence/constraint
 * predicate.
 *
 * That belongs to constraints or target/resource predicates, not this
 * effect-capability requirement relation.
 *
 * This separation prevents:
 *
 *     capability requirement
 *
 * from becoming an implicit:
 *
 *     hardware/resource constraint.
 *
 * ============================================================================
 * EMPTY REQUIREMENTS
 * ============================================================================
 *
 * An effect capability clause MUST contain at least one capability expression.
 *
 * Therefore:
 *
 *     requires {};
 *
 * is rejected by this grammar.
 *
 * An effect with no capability requirements simply omits the clause.
 *
 * This avoids representing an empty requirement as if it were a meaningful
 * capability contract.
 *
 * ============================================================================
 * OPTIONAL SEMICOLON
 * ============================================================================
 *
 * The capability clause permits an optional semicolon:
 *
 *     requires {
 *         quantum::measurement
 *     }
 *
 * or:
 *
 *     requires {
 *         quantum::measurement
 *     };
 *
 * The enclosing effect declaration remains responsible for deciding whether
 * a particular surrounding construct requires a terminator.
 *
 * ============================================================================
 * TRAILING COMMA
 * ============================================================================
 *
 * Capability boolean expressions do not use commas as logical separators.
 *
 * Commas remain available through the canonical capability reference/list
 * grammar for consumers that explicitly need comma-separated references.
 *
 * This avoids introducing two representations for the same boolean
 * requirement.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar produces syntax only.
 *
 * Conceptually:
 *
 *     EffectCapabilityClauseAst {
 *         expression: EffectCapabilityExpressionAst,
 *         source_span: Span
 *     }
 *
 *     EffectCapabilityExpressionAst =
 *           CapabilityReference
 *         | All(...)
 *         | Any(...)
 *         | Group(...)
 *
 * The actual Rust AST remains owned by:
 *
 *     src/frontend/ast/
 *
 * This grammar MUST NOT define Rust structures.
 *
 * The AST should preserve:
 *
 *     - capability identity;
 *     - capability version requirement;
 *     - conjunction structure;
 *     - disjunction structure;
 *     - grouping;
 *     - source ordering;
 *     - source spans;
 *     - original source spelling where required for diagnostics.
 *
 * Semantic normalization belongs downstream.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for:
 *
 *     - resolving capability references;
 *     - validating capability versions;
 *     - determining capability availability;
 *     - checking capability compatibility;
 *     - checking capability conflicts;
 *     - determining whether alternatives are satisfiable;
 *     - combining effect-level and operation-level requirements;
 *     - relating capabilities to resource requirements;
 *     - relating capabilities to target capabilities;
 *     - determining whether the effect can be lowered.
 *
 * The parser MUST NOT perform any of these operations.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * Capability requirements are semantic metadata.
 *
 * They MUST NOT become a second IR.
 *
 * The intended lowering is:
 *
 *     effect source
 *       |
 *       v
 *     frontend AST
 *       |
 *       v
 *     semantic effect model
 *       |
 *       +--> capability requirements
 *       +--> resource requirements
 *       +--> constraints
 *       +--> preferences
 *       |
 *       v
 *     canonical semantic representation
 *       |
 *       +--> classical IR
 *       +--> quantum::ir
 *       +--> HDL / hardware representation
 *       +--> other domain IR
 *
 * Capability metadata may influence legal lowering strategies, but this
 * grammar never chooses a backend.
 *
 * ============================================================================
 * COMPILER CONTRACT
 * ============================================================================
 *
 * The compiler may use the resulting semantic capability requirements to:
 *
 *     - validate compilation contexts;
 *     - reject unsupported realizations;
 *     - preserve source-level requirements;
 *     - select semantically equivalent lowering strategies;
 *     - negotiate available capabilities;
 *     - guide optimization;
 *     - guide target-independent specialization.
 *
 * The compiler MUST NOT interpret:
 *
 *     capability requirement
 *
 * as:
 *
 *     physical device selection.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime capability availability is evaluated downstream.
 *
 * Runtime may have an inventory such as:
 *
 *     available capabilities
 *     resource capacities
 *     execution contexts
 *     device facilities
 *
 * None of that is represented directly by this grammar.
 *
 * An unavailable runtime capability is:
 *
 *     semantic/context/runtime failure
 *
 * and is NOT:
 *
 *     parser failure.
 *
 * ============================================================================
 * RESILIENCE CONTRACT
 * ============================================================================
 *
 * Resilience may consume capability metadata.
 *
 * This grammar does NOT implement:
 *
 *     retry
 *     restart
 *     rollback
 *     reroute
 *     remap
 *     reschedule
 *     recompile
 *     backend switching
 *     quarantine
 *     recovery
 *
 * Those remain downstream resilience responsibilities.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no embedded actions;
 *     - no semantic predicates;
 *     - no filesystem access;
 *     - no network access;
 *     - no runtime calls;
 *     - no hardware discovery;
 *     - no randomness;
 *     - no environment-dependent parsing.
 *
 * Identical token streams therefore produce structurally identical parse
 * trees under the same grammar version.
 *
 * ============================================================================
 * SOURCE PRESERVATION
 * ============================================================================
 *
 * The parser/frontend must preserve enough information for:
 *
 *     - diagnostics;
 *     - source maps;
 *     - formatting;
 *     - IDE/LSP tooling;
 *     - refactoring;
 *     - semantic analysis;
 *     - compatibility checking;
 *     - provenance.
 *
 * The grammar itself does not calculate source spans.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * Repetition is represented structurally:
 *
 *     *
 *     +
 *
 * No finite maximum is encoded for:
 *
 *     - number of capability references;
 *     - number of conjunctions;
 *     - number of alternatives;
 *     - expression nesting;
 *     - number of effects;
 *     - number of effect operations;
 *     - program size.
 *
 * Practical parser/compiler limits remain implementation/resource concerns.
 *
 * They MUST NOT become language-level semantic ceilings.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * This file retains the existing public integration concept:
 *
 *     effectCapabilityClause
 *
 * and provides stable helper boundaries:
 *
 *     effectCapabilityExpression
 *     effectCapabilityDisjunction
 *     effectCapabilityConjunction
 *     effectCapabilityPrimary
 *     effectCapabilityReference
 *     effectCapabilityReferenceList
 *     effectCapabilityRequirement
 *     effectCapabilityRequirementList
 *
 * Capability identity/version syntax remains delegated to
 * core/capabilities.g4.
 *
 * New capability names do not require a change here.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE:
 *
 *     requires {
 *         quantum::measurement
 *     };
 *
 *     requires {
 *         quantum::measurement
 *         and quantum::readout
 *     };
 *
 *     requires {
 *         quantum::measurement
 *         or classical::simulation
 *     };
 *
 *     requires {
 *         quantum::measurement
 *         and (
 *             quantum::dynamic_control
 *             or classical::simulation
 *         )
 *     };
 *
 *     requires {
 *         future::photonic::interaction
 *     };
 *
 *     requires {
 *         accelerator::tensor version >= 1.2
 *     };
 *
 * NEGATIVE:
 *
 *     requires {};
 *
 *     requires {
 *         quantum::
 *     };
 *
 *     requires {
 *         and quantum::measurement
 *     };
 *
 *     requires {
 *         quantum::measurement and
 *     };
 *
 *     requires {
 *         quantum::measurement or
 *     };
 *
 *     requires {
 *         quantum::measurement (
 *     };
 *
 *     requires {
 *         not quantum::measurement
 *     };
 *
 *     requires {
 *         8 gpu
 *     };
 *
 *     requires {
 *         64 qubits
 *     };
 *
 *     requires {
 *         device 0
 *     };
 *
 * BOUNDARY:
 *
 *     one capability;
 *     many capabilities;
 *     deeply qualified capability names;
 *     nested grouping;
 *     many alternatives;
 *     many conjunctions;
 *     mixed conjunction/disjunction;
 *     capability version requirements;
 *     future capability namespaces;
 *     unknown capability names.
 *
 * CROSS-DOMAIN:
 *
 *     classical::io
 *     quantum::measurement
 *     hybrid::feedforward
 *     hdl::synthesis
 *     hardware::reconfigurable_logic
 *     distributed::communication
 *     ai::tensor_compute
 *     data::stream_processing
 *     networking::transport
 *     security::trusted_execution
 *     accelerator::tensor
 *     future::domain::capability
 *
 * POCO-REAF:
 *
 * The same capability requirement must remain syntactically valid regardless
 * of whether the eventual realization uses:
 *
 *     embedded
 *     CPU
 *     multicore
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     simulator
 *     accelerator
 *     cluster
 *     HPC
 *     distributed
 *     cloud
 *     future hardware.
 *
 * DETERMINISM:
 *
 * Identical token streams produce identical capability-expression structure.
 *
 * ROUND TRIP:
 *
 *     source
 *       -> lexer
 *       -> parser
 *       -> AST
 *       -> formatter
 *       -> parser
 *
 * must preserve capability identity, version requirements, logical
 * composition, and grouping.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains NO production-level limits for:
 *
 *     MAX_CAPABILITIES
 *     MAX_EFFECT_CAPABILITIES
 *     MAX_EFFECTS
 *     MAX_OPERATIONS
 *     MAX_ALTERNATIVES
 *     MAX_CONJUNCTIONS
 *     MAX_NESTING
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_DEVICES
 *
 * It contains no:
 *
 *     physical device IDs;
 *     physical qubit IDs;
 *     backend IDs;
 *     topology declarations;
 *     resource allocation;
 *     routing;
 *     scheduling;
 *     calibration;
 *     QEC implementation;
 *     ZQN implementation;
 *     HAL implementation.
 *
 * ============================================================================
 * NON-DEPENDENCIES
 * ============================================================================
 *
 * This file MUST NOT depend semantically on:
 *
 *     src/quantum/ir
 *     src/quantum/qec
 *     src/quantum/zqn
 *     src/quantum/routing
 *     src/quantum/scheduling
 *     src/quantum/optimization
 *     src/quantum/hardware
 *     backend SDKs
 *     device discovery
 *     runtime capability tokens
 *     authorization credentials
 *
 * ============================================================================
 * ANTLR COMPOSITION
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * The production parser vocabulary is:
 *
 *     ZamaniLexer
 *
 * Core parser dependencies are imported through:
 *
 *     Core
 *     Capabilities
 *
 * `Capabilities` supplies:
 *
 *     capabilityReference
 *
 * `Core` supplies shared parser infrastructure such as names and attributes
 * according to the repository's parser composition.
 *
 * This file does NOT redefine either.
 *
 * ============================================================================
 * PUBLIC INTEGRATION RULES
 * ============================================================================
 *
 * The stable public rules of this grammar are:
 *
 *     effectCapabilityClause
 *     effectCapabilityExpression
 *     effectCapabilityDisjunction
 *     effectCapabilityConjunction
 *     effectCapabilityPrimary
 *     effectCapabilityReference
 *     effectCapabilityReferenceList
 *     effectCapabilityRequirement
 *     effectCapabilityRequirementList
 *
 * Effect declaration grammars should consume:
 *
 *     effectCapabilityClause
 *
 * rather than reproducing capability-expression syntax.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 * [x] Existing filename is retained.
 * [x] Grammar name remains EffectCapabilities.
 * [x] Parser grammar is used.
 * [x] Production lexer is ZamaniLexer.
 * [x] No K_* token aliases are used.
 * [x] No ZamaniTokens token vocabulary is consumed directly.
 * [x] Capability identity is delegated to core/capabilities.g4.
 * [x] Capability version syntax is delegated to core/capabilities.g4.
 * [x] Name syntax is delegated to core/names.g4.
 * [x] Effect declaration ownership remains outside this file.
 * [x] Effect-set ownership remains outside this file.
 * [x] Resource semantics remain outside this file.
 * [x] Requirements remain semantically distinct.
 * [x] Constraints remain semantically distinct.
 * [x] Preferences remain semantically distinct.
 * [x] Targets remain semantically distinct.
 * [x] Placement remains semantically distinct.
 * [x] Performance remains semantically distinct.
 * [x] Security authorization remains outside this file.
 * [x] Quantum semantics remain outside this file.
 * [x] quantum::ir remains the canonical quantum semantic boundary.
 * [x] No physical hardware is selected.
 * [x] No device IDs are represented.
 * [x] No resource quantities are represented.
 * [x] No finite machine limits are represented.
 * [x] No fixed quantum capability enumeration exists.
 * [x] Unknown/future capability names remain syntactically extensible.
 * [x] Empty capability clauses are rejected.
 * [x] Capability conjunctions are supported.
 * [x] Capability alternatives are supported.
 * [x] Grouping is supported.
 * [x] Capability negation is intentionally excluded from this requirement
 *     grammar and remains available to appropriate constraint/predicate
 *     grammars.
 * [x] No embedded Rust actions exist.
 * [x] No semantic predicates exist.
 * [x] No I/O exists.
 * [x] No runtime calls exist.
 * [x] No hardware discovery exists.
 * [x] No unsafe Rust is required.
 * [x] Rust 1.97 / 1.97.1 compatibility is preserved.
 * [x] Deterministic parsing is preserved.
 * [x] Source structure can be preserved by the frontend AST.
 * [x] Integration with effect declarations is explicit.
 * [x] Integration with capability declarations is explicit.
 * [x] Integration with quantum/classical/HDL/hardware domains is explicit.
 * [x] POCO-REAF constraints are explicit.
 *
 * ============================================================================
 */

parser grammar EffectCapabilities;

options {
    tokenVocab = ZamaniLexer;
}

import Core, Capabilities;


/*
 * ============================================================================
 * EFFECT CAPABILITY CLAUSE
 * ============================================================================
 *
 * Canonical form:
 *
 *     requires {
 *         quantum::measurement
 *     };
 *
 * The expression is mandatory.
 *
 * An effect with no requirements simply omits the clause.
 */
effectCapabilityClause
    : REQUIRES
      LBRACE
      effectCapabilityExpression
      RBRACE
      SEMICOLON?
    ;


/*
 * ============================================================================
 * EFFECT CAPABILITY EXPRESSION
 * ============================================================================
 *
 * Entry point for effect capability requirements.
 *
 * Precedence:
 *
 *     OR
 *       lower precedence
 *
 *     AND
 *       higher precedence
 *
 *     primary/group
 *       highest precedence
 *
 * Therefore:
 *
 *     A or B and C
 *
 * is structurally:
 *
 *     A or (B and C)
 *
 * Parentheses can explicitly override grouping.
 */
effectCapabilityExpression
    : effectCapabilityDisjunction
    ;


/*
 * ============================================================================
 * DISJUNCTION
 * ============================================================================
 *
 *     A or B or C
 *
 * Represents alternatives.
 *
 * The parser records alternatives.
 * Semantic analysis determines whether an alternative is actually available.
 */
effectCapabilityDisjunction
    : effectCapabilityConjunction
      (OR effectCapabilityConjunction)*
    ;


/*
 * ============================================================================
 * CONJUNCTION
 * ============================================================================
 *
 *     A and B and C
 *
 * Represents simultaneous capability requirements.
 */
effectCapabilityConjunction
    : effectCapabilityPrimary
      (AND effectCapabilityPrimary)*
    ;


/*
 * ============================================================================
 * PRIMARY
 * ============================================================================
 *
 * A primary requirement is either:
 *
 *     - one canonical capability reference; or
 *     - a grouped capability expression.
 */
effectCapabilityPrimary
    : effectCapabilityReference
    | LPAREN
      effectCapabilityExpression
      RPAREN
    ;


/*
 * ============================================================================
 * CAPABILITY REFERENCE
 * ============================================================================
 *
 * Capability identity and version syntax belong exclusively to:
 *
 *     grammar/core/capabilities.g4
 *
 * Therefore this rule delegates directly to:
 *
 *     capabilityReference
 *
 * Examples:
 *
 *     quantum::measurement
 *     quantum::measurement version 1
 *     quantum::measurement version >= 1.2
 *     future::domain::capability
 */
effectCapabilityReference
    : capabilityReference
    ;


/*
 * ============================================================================
 * SINGLE REQUIREMENT
 * ============================================================================
 *
 * Convenience integration rule for consumers that need exactly one capability
 * reference rather than a boolean capability expression.
 */
effectCapabilityRequirement
    : effectCapabilityReference
    ;


/*
 * ============================================================================
 * CAPABILITY REFERENCE LIST
 * ============================================================================
 *
 * Convenience integration rule for consumers that explicitly need a
 * comma-separated collection of capability references.
 *
 * This does not create a second capability representation.
 *
 * It remains a sequence of the canonical capabilityReference rule.
 *
 * Example:
 *
 *     quantum::measurement,
 *     quantum::readout,
 *     quantum::reset
 *
 * This rule is intentionally separate from the boolean expression grammar:
 *
 *     A, B
 *
 * does not silently mean:
 *
 *     A and B
 *
 * The consuming grammar determines the meaning of a comma-separated list.
 */
effectCapabilityReferenceList
    : effectCapabilityReference
      (COMMA effectCapabilityReference)*
      COMMA?
    ;


/*
 * ============================================================================
 * REQUIREMENT LIST
 * ============================================================================
 *
 * Compatibility/convenience alias.
 *
 * The canonical capability identity remains capabilityReference.
 */
effectCapabilityRequirementList
    : effectCapabilityReferenceList
    ;
/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/effects/capabilities.g4
 *
 * Status:
 *     Canonical modular production grammar for EFFECT/CAPABILITY
 *     RELATIONSHIPS.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Runtime/compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     This grammar contains:
 *
 *       - no embedded Rust actions;
 *       - no semantic predicates;
 *       - no filesystem access;
 *       - no network access;
 *       - no runtime calls;
 *       - no hardware discovery;
 *       - no unsafe code.
 *
 * IMPORTANT:
 *
 *     This file does NOT own the language-wide capability system.
 *
 *     grammar/core/capabilities.g4
 *
 * remains the canonical owner of:
 *
 *     - capability declarations;
 *     - capability identities;
 *     - capability references;
 *     - capability version constraints;
 *     - capability lists.
 *
 * This file owns only the syntax by which an EFFECT declares the
 * capabilities required to provide or implement that effect.
 *
 * ============================================================================
 * ARCHITECTURAL PRINCIPLE
 * ============================================================================
 *
 * Effect:
 *
 *     WHAT computational interaction exists?
 *
 * Capability:
 *
 *     WHAT can an execution environment provide?
 *
 * Effect capability requirement:
 *
 *     WHAT capability must be available for this effect to be realized?
 *
 * Resource:
 *
 *     WHAT computational resource is available or requested?
 *
 * Requirement:
 *
 *     WHAT must be satisfied by a valid realization?
 *
 * Constraint:
 *
 *     WHAT conditions must a realization obey?
 *
 * Preference:
 *
 *     WHICH valid realization is preferred?
 *
 * These concepts MUST NOT be collapsed.
 *
 * In particular:
 *
 *     effect quantum::measurement
 *
 * MUST NOT implicitly mean:
 *
 *     use QPU X
 *     use N qubits
 *     use topology Y
 *     use backend Z
 *
 * Hardware discovery, resource allocation, routing, scheduling and backend
 * selection belong downstream.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Capability requirements are source-level semantic intent.
 *
 * They MUST remain independent of the physical machine on which the effect
 * is eventually realized.
 *
 * The same source program MUST therefore remain syntactically valid when
 * the effect is implemented using:
 *
 *     - a tiny embedded processor;
 *     - one CPU;
 *     - many CPUs;
 *     - a GPU;
 *     - many GPUs;
 *     - an FPGA;
 *     - an ASIC;
 *     - a quantum processor;
 *     - a quantum simulator;
 *     - a heterogeneous accelerator;
 *     - a distributed system;
 *     - a cloud environment;
 *     - a future computational architecture.
 *
 * This grammar MUST NOT encode:
 *
 *     MAX_CAPABILITIES
 *     MAX_EFFECT_CAPABILITIES
 *     MAX_DEVICES
 *     MAX_QUBITS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *
 * or any equivalent finite machine-specific limit.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - effect capability clauses;
 *     - effect capability requirement lists;
 *     - effect capability alternatives;
 *     - effect capability conjunctions;
 *     - effect capability grouping;
 *     - effect capability modifiers;
 *     - syntax attaching capability requirements to an effect;
 *     - syntax attaching capability requirements to an effect operation;
 *     - syntactic capability-provision relationships at the effect boundary.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - capability declarations;
 *     - capability identity syntax;
 *     - capability registry;
 *     - capability discovery;
 *     - capability authorization;
 *     - resource allocation;
 *     - resource discovery;
 *     - target selection;
 *     - hardware discovery;
 *     - hardware topology;
 *     - device identifiers;
 *     - quantum qubits;
 *     - quantum gates;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN;
 *     - routing;
 *     - scheduling;
 *     - optimization;
 *     - resilience;
 *     - calibration;
 *     - runtime dispatch;
 *     - effect implementation;
 *     - security authorization.
 *
 * ============================================================================
 * DEPENDENCY DIRECTION
 * ============================================================================
 *
 *     ZamaniTokens
 *          |
 *          v
 *     Core names / capability model
 *          |
 *          v
 *     capabilities.g4
 *          |
 *          v
 *     effect declarations
 *          |
 *          v
 *     effect semantic analysis
 *          |
 *          +--> capability analysis
 *          +--> requirement analysis
 *          +--> resource analysis
 *          +--> target analysis
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> classical IR
 *          +--> quantum::ir
 *          +--> HDL / hardware representation
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     routing / scheduling / resilience
 *          |
 *          v
 *     target lowering
 *          |
 *          v
 *     runtime / hardware
 *
 * The dependency direction MUST NOT be reversed.
 *
 * ============================================================================
 * CAPABILITY OWNERSHIP BOUNDARY
 * ============================================================================
 *
 * grammar/core/capabilities.g4 is the canonical capability grammar.
 *
 * This file therefore consumes:
 *
 *     capabilityReference
 *     capabilityVersionClause
 *
 * rather than redefining them.
 *
 * A new capability such as:
 *
 *     quantum::dynamic_control
 *     quantum::mid_circuit_measurement
 *     accelerator::tensor
 *     distributed::consensus
 *     future::photonic::interaction
 *
 * does NOT require a modification to this file.
 *
 * ============================================================================
 * OPEN-WORLD MODEL
 * ============================================================================
 *
 * Capability identities are open-ended.
 *
 * There is deliberately NO grammar such as:
 *
 *     quantumCapability
 *     gpuCapability
 *     cpuCapability
 *     fpgaCapability
 *     qpuCapability
 *     vendorCapability
 *
 * Instead:
 *
 *     capabilityReference
 *
 * resolves the identity through the canonical capability system.
 *
 * ============================================================================
 * EFFECT/CAPABILITY SEMANTIC SEPARATION
 * ============================================================================
 *
 * An effect may require one or more capabilities.
 *
 * For example:
 *
 *     effect Measurement
 *         requires {
 *             quantum::measurement
 *             quantum::readout
 *         };
 *
 * This means:
 *
 *     the effect cannot be validly realized unless the semantic capability
 *     requirements are satisfied.
 *
 * It does NOT mean:
 *
 *     select a particular QPU;
 *     allocate a particular number of qubits;
 *     select a topology;
 *     select a calibration;
 *     select a vendor backend.
 *
 * Those decisions are downstream.
 *
 * ============================================================================
 * CAPABILITY VS AUTHORIZATION
 * ============================================================================
 *
 * A capability requirement is NOT an authorization grant.
 *
 * For example:
 *
 *     requires {
 *         security::trusted_execution
 *     }
 *
 * does not grant permission to access trusted execution.
 *
 * Security authorization belongs to the security subsystem.
 *
 * ============================================================================
 * CAPABILITY VS RESOURCE
 * ============================================================================
 *
 * A capability requirement describes a property.
 *
 * It does not allocate a resource.
 *
 * Therefore this grammar MUST NOT contain constructs such as:
 *
 *     requires 8 gpu;
 *     requires 1024 cores;
 *     requires 64 qubits;
 *     requires device 0;
 *     requires topology ring;
 *
 * unless such constructs are explicitly introduced by a separate resource,
 * target or hardware grammar.
 *
 * Even there, those values MUST remain target/resource semantics rather than
 * universal effect capability semantics.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Quantum capability references may appear naturally:
 *
 *     quantum::measurement
 *     quantum::dynamic_control
 *     quantum::mid_circuit_measurement
 *     quantum::logical_qubits
 *     quantum::reset
 *
 * This grammar does not define their meaning.
 *
 * It does not import:
 *
 *     QubitId
 *     PhysicalQubitId
 *     GateKind
 *     quantum::ir
 *     topology
 *     calibration
 *     QEC
 *     ZQN
 *
 * If a capability affects quantum compilation, semantic analysis may later
 * carry the information into the canonical quantum semantic pipeline.
 *
 * The canonical quantum semantic boundary remains:
 *
 *     quantum::ir
 *
 * ============================================================================
 * HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * Hardware-related capabilities may be represented as names:
 *
 *     hardware::clocked_logic
 *     hardware::reconfigurable_logic
 *     hardware::pipeline
 *     accelerator::tensor
 *
 * No physical implementation is selected by this grammar.
 *
 * ============================================================================
 * DISTRIBUTED BOUNDARY
 * ============================================================================
 *
 * Distributed capabilities may be represented as names:
 *
 *     distributed::communication
 *     distributed::consensus
 *     distributed::replication
 *     distributed::fault_tolerance
 *
 * The grammar does not define:
 *
 *     node count;
 *     node addresses;
 *     network topology;
 *     region;
 *     deployment;
 *     placement.
 *
 * ============================================================================
 * EFFECT OPERATION BOUNDARY
 * ============================================================================
 *
 * Capability requirements may apply to:
 *
 *     - an entire effect;
 *     - an individual effect operation.
 *
 * This allows a broad effect to contain operations with different
 * implementation requirements.
 *
 * Example:
 *
 *     effect QuantumIO {
 *         fn measure(...)
 *             requires {
 *                 quantum::measurement,
 *                 quantum::readout
 *             };
 *
 *         fn reset(...)
 *             requires {
 *                 quantum::reset
 *             };
 *     }
 *
 * The grammar records the relationship.
 *
 * Semantic analysis determines whether those capabilities are actually
 * sufficient and whether they are compatible.
 *
 * ============================================================================
 * CAPABILITY COMPOSITION
 * ============================================================================
 *
 * The syntax supports:
 *
 *     conjunction
 *     alternatives
 *     grouping
 *
 * Example:
 *
 *     requires {
 *         quantum::measurement
 *         and quantum::readout
 *     };
 *
 * Example:
 *
 *     requires {
 *         quantum::measurement
 *         and (
 *             quantum::dynamic_control
 *             or classical::simulation
 *         )
 *     };
 *
 * The parser records the structure.
 *
 * It MUST NOT decide whether the target satisfies the expression.
 *
 * ============================================================================
 * UNKNOWN CAPABILITIES
 * ============================================================================
 *
 * Unknown capabilities MUST remain syntactically valid.
 *
 * For example:
 *
 *     future::quantum::new_operation
 *
 * is syntactically valid even when the current compiler does not know the
 * semantic definition.
 *
 * Semantic analysis may later report:
 *
 *     unknown capability
 *
 * without requiring a grammar modification.
 *
 * This is required for future-proofing and POCO-REAF.
 *
 * ============================================================================
 * VERSION BOUNDARY
 * ============================================================================
 *
 * Capability versions are owned by:
 *
 *     grammar/core/capabilities.g4
 *
 * This file consumes capability references as defined there.
 *
 * This file MUST NOT create another version language.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar:
 *
 *     - contains no semantic predicates;
 *     - contains no embedded actions;
 *     - performs no I/O;
 *     - performs no network access;
 *     - performs no hardware discovery;
 *     - performs no runtime calls;
 *     - performs no random operations.
 *
 * Parsing therefore depends only on the deterministic token stream.
 *
 * ============================================================================
 * SOURCE PRESERVATION
 * ============================================================================
 *
 * The frontend AST must preserve:
 *
 *     - capability reference ordering;
 *     - conjunction structure;
 *     - disjunction structure;
 *     - grouping;
 *     - version clauses;
 *     - source spans;
 *     - original source spelling where required by diagnostics.
 *
 * Semantic canonicalization occurs downstream.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Conceptually this grammar produces:
 *
 *     EffectCapabilityClauseAst
 *         {
 *             expression
 *             source_span
 *         }
 *
 *     EffectCapabilityExpressionAst
 *         =
 *             CapabilityReference
 *           | All(...)
 *           | Any(...)
 *           | Group(...)
 *
 * The exact Rust structures belong to:
 *
 *     src/frontend/ast/
 *
 * This grammar MUST NOT define Rust structures.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis determines:
 *
 *     - whether capability references resolve;
 *     - whether versions are valid;
 *     - whether capability combinations are satisfiable;
 *     - whether an effect operation is compatible with the capability set;
 *     - whether capabilities conflict;
 *     - whether required capabilities are available;
 *     - whether capability requirements are target-independent;
 *     - whether a capability is merely advisory or mandatory;
 *     - whether the enclosing effect declaration is semantically valid.
 *
 * None of these decisions belong in the parser.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * Effect capability information is metadata about semantic operations.
 *
 * It MUST NOT become a duplicate hardware or quantum IR.
 *
 * Conceptually:
 *
 *     effect source
 *          |
 *          v
 *     effect AST
 *          |
 *          v
 *     semantic effect model
 *          |
 *          +--> capability requirements
 *          |
 *          +--> resource requirements
 *          |
 *          +--> constraints
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> quantum::ir when quantum semantics require it
 *          +--> classical IR
 *          +--> HDL/hardware IR
 *
 * This grammar never constructs quantum::ir directly.
 *
 * ============================================================================
 * COMPILER CONTRACT
 * ============================================================================
 *
 * The compiler may use effect capability requirements to:
 *
 *     - validate a compilation context;
 *     - preserve semantic requirements;
 *     - determine legal lowering strategies;
 *     - reject an unsupported realization;
 *     - negotiate target capabilities;
 *     - select among semantically equivalent implementations.
 *
 * It MUST NOT convert:
 *
 *     capability requirement
 *
 * directly into:
 *
 *     physical device selection.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime capability verification is downstream.
 *
 * Conceptually:
 *
 *     source
 *       |
 *       v
 *     capability requirement
 *       |
 *       v
 *     semantic model
 *       |
 *       v
 *     execution capability inventory
 *       |
 *       v
 *     capability evaluation
 *
 * Runtime failure is not parser failure.
 *
 * ============================================================================
 * RESILIENCE CONTRACT
 * ============================================================================
 *
 * Resilience may consume capability requirements when determining whether
 * a recovery strategy remains valid.
 *
 * This grammar MUST NOT implement:
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
 *
 * Those remain resilience responsibilities.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * The grammar uses repetition and recursion rather than fixed counts.
 *
 * Therefore it imposes no language-level maximum on:
 *
 *     - capability count;
 *     - alternatives;
 *     - conjunctions;
 *     - nesting;
 *     - effect operations;
 *     - effects;
 *     - program size.
 *
 * Actual limits are implementation/resource limits and must remain outside
 * language semantics.
 *
 * ============================================================================
 * ANTLR COMPOSITION
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * The canonical lexer vocabulary is:
 *
 *     grammar/lexer/tokens.g4
 *
 * and therefore:
 *
 *     tokenVocab = ZamaniTokens
 *
 * is required.
 *
 * Canonical parser dependencies:
 *
 *     Core
 *     Capabilities
 *
 * `Capabilities` supplies:
 *
 *     capabilityReference
 *     capabilityVersionClause
 *
 * Core supplies:
 *
 *     identifier
 *     qualifiedName
 *     attributes
 *     shared syntax infrastructure
 *
 * This file MUST NOT redefine those rules.
 *
 * ============================================================================
 * INTEGRATION WITH effect-declarations.g4
 * ============================================================================
 *
 * `effect-declarations.g4` remains the owner of:
 *
 *     effectDeclaration
 *     effectOperationDeclaration
 *
 * This file provides:
 *
 *     effectCapabilityClause
 *     effectCapabilityExpression
 *
 * Effect declarations MAY consume:
 *
 *     effectCapabilityClause?
 *
 * Effect operations MAY consume:
 *
 *     effectCapabilityClause?
 *
 * The integration must be performed in the effect declaration grammar rather
 * than duplicating effect declaration rules here.
 *
 * ============================================================================
 * INTEGRATION WITH effect-sets.g4
 * ============================================================================
 *
 * Effect sets describe WHICH effects are associated with a computation.
 *
 * Capability clauses describe WHICH capabilities are required to realize
 * an effect.
 *
 * They are intentionally separate:
 *
 *     effect-set
 *          !=
 *     capability-set
 *
 * A capability requirement MUST NOT silently become an effect.
 *
 * ============================================================================
 * INTEGRATION WITH core/requirements.g4
 * ============================================================================
 *
 * A program-level requirement:
 *
 *     requires quantum::measurement;
 *
 * is owned by:
 *
 *     grammar/core/requirements.g4
 *
 * An effect-level capability requirement:
 *
 *     requires {
 *         quantum::measurement
 *     };
 *
 * is owned structurally by this file.
 *
 * The semantic system may unify both into a common semantic requirement model,
 * but the grammars remain separately owned.
 *
 * ============================================================================
 * INTEGRATION WITH hardware/
 * ============================================================================
 *
 * Hardware grammar may define hardware-specific capabilities.
 *
 * This file does not import hardware implementations.
 *
 * Example:
 *
 *     hardware::reconfigurable_logic
 *
 * remains an open capability reference.
 *
 * ============================================================================
 * INTEGRATION WITH quantum/
 * ============================================================================
 *
 * Quantum grammar may define quantum operations whose semantic requirements
 * are evaluated against capabilities produced by this layer.
 *
 * No quantum syntax is duplicated here.
 *
 * ============================================================================
 * INTEGRATION WITH resource/
 * ============================================================================
 *
 * Capability:
 *
 *     what the environment can do.
 *
 * Resource:
 *
 *     what physical/logical capacity is available.
 *
 * This file MUST NOT introduce resource quantities.
 *
 * ============================================================================
 * INTEGRATION WITH SECURITY
 * ============================================================================
 *
 * Capability requirements are not security grants.
 *
 * Security policy may independently evaluate:
 *
 *     capability requirement
 *     authorization policy
 *     trust state
 *     identity
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden:
 *
 *     MAX_CAPABILITIES
 *     MAX_EFFECT_CAPABILITIES
 *     MAX_ALTERNATIVES
 *     MAX_EFFECTS
 *     MAX_OPERATIONS
 *     MAX_QUBITS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     DEVICE_0
 *     QPU_0
 *     CPU_0
 *     GPU_0
 *
 * Also forbidden are hidden finite enumerations such as:
 *
 *     quantumCapability
 *     gpuCapability
 *     cpuCapability
 *
 * unless they are explicitly part of a separate closed semantic vocabulary.
 *
 * ============================================================================
 * PUBLIC RULES
 * ============================================================================
 *
 * Public integration rules:
 *
 *     effectCapabilityClause
 *     effectCapabilityExpression
 *     effectCapabilityPrimary
 *     effectCapabilityReference
 *     effectCapabilityReferenceList
 *
 * These rules form the stable boundary for effect declaration grammars.
 *
 * ============================================================================
 */

parser grammar EffectCapabilities;

options {
    tokenVocab = ZamaniTokens;
}

import Core, Capabilities;


/*
 * ============================================================================
 * 1. EFFECT CAPABILITY CLAUSE
 * ============================================================================
 *
 * Canonical form:
 *
 *     requires {
 *         quantum::measurement
 *     };
 *
 * Multiple capabilities may be supplied.
 *
 * The braces deliberately distinguish this construct from unrelated
 * `requires(...)` contracts elsewhere in the language.
 *
 * ============================================================================
 */

effectCapabilityClause
    : K_REQUIRES
      LBRACE
      effectCapabilityExpression?
      RBRACE
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 2. EFFECT CAPABILITY EXPRESSION
 * ============================================================================
 *
 * Capability requirements support:
 *
 *     A
 *
 *     A and B
 *
 *     A or B
 *
 *     A and (B or C)
 *
 * The parser records structure.
 *
 * Semantic satisfiability belongs downstream.
 *
 * ============================================================================
 */

effectCapabilityExpression
    : effectCapabilityDisjunction
    ;


/*
 * ============================================================================
 * 3. DISJUNCTION
 * ============================================================================
 *
 *     A or B or C
 *
 * No finite alternative count exists.
 *
 * ============================================================================
 */

effectCapabilityDisjunction
    : effectCapabilityConjunction
      (K_OR effectCapabilityConjunction)*
    ;


/*
 * ============================================================================
 * 4. CONJUNCTION
 * ============================================================================
 *
 *     A and B and C
 *
 * No finite conjunction count exists.
 *
 * ============================================================================
 */

effectCapabilityConjunction
    : effectCapabilityPrimary
      (K_AND effectCapabilityPrimary)*
    ;


/*
 * ============================================================================
 * 5. PRIMARY
 * ============================================================================
 *
 * A primary capability requirement is either:
 *
 *     capability reference
 *
 * or:
 *
 *     grouped capability expression
 *
 * ============================================================================
 */

effectCapabilityPrimary
    : effectCapabilityReference
    | LPAREN
      effectCapabilityExpression
      RPAREN
    ;


/*
 * ============================================================================
 * 6. CAPABILITY REFERENCE
 * ============================================================================
 *
 * The canonical capability grammar owns capability identity and version
 * syntax.
 *
 * This rule intentionally delegates to:
 *
 *     capabilityReference
 *
 * rather than duplicating:
 *
 *     qualifiedName
 *     capability version
 *
 * syntax.
 *
 * ============================================================================
 */

effectCapabilityReference
    : capabilityReference
    ;


/*
 * ============================================================================
 * 7. CAPABILITY REFERENCE LIST
 * ============================================================================
 *
 * This rule exists as a convenience boundary for effect grammars that need
 * comma-separated capability references.
 *
 * It does not define a new capability representation.
 *
 * ============================================================================
 */

effectCapabilityReferenceList
    : effectCapabilityReference
      (COMMA effectCapabilityReference)*
      COMMA?
    ;


/*
 * ============================================================================
 * 8. SINGLE EFFECT CAPABILITY REQUIREMENT
 * ============================================================================
 *
 * Explicit single-reference form for grammar consumers that do not need
 * boolean composition.
 *
 * ============================================================================
 */

effectCapabilityRequirement
    : effectCapabilityReference
    ;


/*
 * ============================================================================
 * 9. EFFECT CAPABILITY REQUIREMENT LIST
 * ============================================================================
 *
 * Convenience integration rule.
 *
 * ============================================================================
 */

effectCapabilityRequirementList
    : effectCapabilityReferenceList
    ;
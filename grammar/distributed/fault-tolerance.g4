/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/distributed/fault-tolerance.g4
 *
 * Grammar:
 *     FaultTolerance
 *
 * Status:
 *     Production distributed fault-tolerance grammar
 *
 * Rust baseline:
 *     Rust 1.97+
 *     Rust Edition 2021
 *
 * Safety:
 *     - Grammar only.
 *     - No embedded Rust actions.
 *     - No semantic predicates.
 *     - No unsafe Rust.
 *     - No filesystem access.
 *     - No network access.
 *     - No hardware access.
 *     - No runtime callbacks.
 *     - No randomness.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the SOURCE-LEVEL SYNTAX for distributed fault-tolerance
 * intent.
 *
 * Fault tolerance expresses what failures a computation may tolerate and what
 * recovery/degradation behavior is acceptable.
 *
 * It does NOT implement fault tolerance.
 *
 * This grammar does not:
 *
 *     - detect failures;
 *     - monitor health;
 *     - restart processes;
 *     - perform failover;
 *     - select machines;
 *     - allocate replicas;
 *     - choose consensus algorithms;
 *     - choose consistency algorithms;
 *     - choose storage engines;
 *     - choose network transports;
 *     - schedule work;
 *     - route work;
 *     - allocate hardware;
 *     - execute recovery;
 *     - implement checkpoint storage;
 *     - implement QEC;
 *     - implement ZQN;
 *     - implement HAL behavior.
 *
 * Those decisions belong downstream.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     FaultTolerance parser
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          +--> name resolution
 *          +--> type analysis
 *          +--> effect analysis
 *          +--> capability analysis
 *          +--> resource analysis
 *          +--> contract analysis
 *          +--> policy analysis
 *          +--> provenance
 *          +--> distributed semantics
 *          +--> resilience semantics
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> classical IR
 *          +--> quantum::ir
 *          +--> HDL / hardware semantics
 *          +--> distributed metadata
 *          +--> resilience metadata
 *          |
 *          v
 *     optimization
 *          |
 *          +--> placement
 *          +--> routing
 *          +--> scheduling
 *          +--> resilience
 *          +--> QEC where applicable
 *          |
 *          v
 *     ZQN / HAL / target realization
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
 *
 * Fault-tolerance syntax describes portable intent.
 *
 * The same source must be able to describe a computation intended for:
 *
 *     - tiny systems;
 *     - embedded systems;
 *     - CPUs;
 *     - multicore systems;
 *     - GPUs;
 *     - FPGAs;
 *     - ASICs;
 *     - accelerators;
 *     - QPUs;
 *     - simulators;
 *     - HPC systems;
 *     - clusters;
 *     - federated systems;
 *     - cloud systems;
 *     - heterogeneous systems;
 *     - future computational substrates.
 *
 * The grammar therefore contains no physical deployment assumptions.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This grammar imposes NO language-level finite limit on:
 *
 *     - declarations;
 *     - targets;
 *     - failure models;
 *     - failure domains;
 *     - recovery policies;
 *     - recovery stages;
 *     - recovery actions;
 *     - alternatives;
 *     - dependencies;
 *     - requirements;
 *     - capabilities;
 *     - constraints;
 *     - preferences;
 *     - policies;
 *     - evidence;
 *     - provenance;
 *     - nested objects;
 *     - list elements;
 *     - expression arguments.
 *
 * There is deliberately no:
 *
 *     MAX_NODES
 *     MAX_REPLICAS
 *     MAX_RETRIES
 *     MAX_RECOVERY_STEPS
 *     MAX_FAILURES
 *     MAX_CHECKPOINTS
 *     MAX_POLICIES
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_QUBITS
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_DEVICES
 *     MAX_NETWORK_SIZE
 *
 * Any finite implementation limit belongs to the parser implementation,
 * compiler resources, runtime resources, deployment policy, or target
 * capabilities. Such a limit is never a language semantic limit.
 *
 * ============================================================================
 * OPEN-WORLD FAILURE MODEL
 * ============================================================================
 *
 * Failure-model names remain semantic names.
 *
 * Examples:
 *
 *     transient
 *     permanent
 *     process_failure
 *     node_failure
 *     service_failure
 *     communication_failure
 *     storage_failure
 *     resource_exhaustion
 *     corruption
 *     timeout
 *     cancellation
 *     unknown
 *
 * Future names remain representable:
 *
 *     vendor::failure_model
 *     domain::custom_failure
 *     future::failure::class
 *
 * Adding a new failure model must NOT require a parser rewrite.
 *
 * ============================================================================
 * OPEN-WORLD RECOVERY MODEL
 * ============================================================================
 *
 * Recovery strategies are also semantic names.
 *
 * Examples:
 *
 *     retry
 *     restart
 *     failover
 *     reconstruct
 *     resume
 *     rollback
 *     checkpoint
 *     degrade
 *     compensate
 *     escalate
 *
 * These names are not closed parser enumerations.
 *
 * The semantic layer determines whether a named strategy is:
 *
 *     - built-in;
 *     - library-defined;
 *     - dialect-defined;
 *     - vendor-defined;
 *     - experimental;
 *     - deprecated;
 *     - unavailable;
 *     - unsupported.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - fault-tolerance declaration framing;
 *     - fault-tolerance invocation arguments;
 *     - fault-tolerance body structure;
 *     - fault-tolerance clause structure;
 *     - fault-tolerance nested scopes;
 *     - fault-tolerance lists/objects;
 *     - fault-tolerance extension calls;
 *     - fault-tolerance parser integration points.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexer vocabulary;
 *     - identifiers;
 *     - qualified names;
 *     - general expressions;
 *     - types;
 *     - resources;
 *     - capabilities;
 *     - effects;
 *     - contracts;
 *     - policies;
 *     - provenance;
 *     - nodes;
 *     - processes;
 *     - actors;
 *     - services;
 *     - channels;
 *     - replication;
 *     - consistency;
 *     - partitioning;
 *     - topology;
 *     - placement;
 *     - networking;
 *     - transactions;
 *     - scheduling;
 *     - quantum operations;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime implementation.
 *
 * ============================================================================
 * DOMAIN SEPARATION
 * ============================================================================
 *
 * Replication:
 *     grammar/distributed/replication.g4
 *
 * Consistency:
 *     grammar/distributed/consistency.g4
 *
 * Partitioning:
 *     grammar/distributed/partitioning.g4
 *
 * Topology:
 *     grammar/distributed/topology.g4
 *
 * Placement:
 *     grammar/distributed/placement.g4
 *
 * Transactions:
 *     grammar/distributed/transactions.g4
 *
 * Fault tolerance:
 *     THIS FILE
 *
 * Resilience execution:
 *     grammar/execution/resilience.g4
 *
 * Resources/capabilities:
 *     grammar/resources/
 *
 * Policies:
 *     grammar/policies/
 *
 * Networking:
 *     grammar/networking/
 *
 * The fault-tolerance grammar may reference the semantics of these systems
 * through expressions and qualified names, but does not duplicate their
 * syntax.
 *
 * ============================================================================
 * PUBLIC INTEGRATION CONTRACT
 * ============================================================================
 *
 * The stable public rule is:
 *
 *     distributedFaultToleranceDeclaration
 *
 * `grammar/distributed/distributed.g4` already consumes this rule.
 *
 * That rule MUST remain stable.
 *
 * ============================================================================
 * LEXICAL CONTRACT
 * ============================================================================
 *
 * The declaration marker is deliberately reserved:
 *
 *     FAULT_TOLERANCE
 *
 * spelling:
 *
 *     fault_tolerance
 *
 * This is necessary because the distributed composition root contains several
 * distributed constructs. Using an arbitrary `identifier` as the declaration
 * marker makes declarations structurally indistinguishable from other
 * identifier-led distributed constructs.
 *
 * Only the construct marker is reserved.
 *
 * Failure-model names, recovery-policy names, vendor names, implementation
 * names, domain names and future semantic names remain identifiers.
 *
 * Do NOT reserve:
 *
 *     transient
 *     permanent
 *     node_failure
 *     process_failure
 *     retry_policy
 *     failover_policy
 *     checkpoint_strategy
 *     vendor-specific names
 *     physical device names
 *
 * ============================================================================
 * CANONICAL SOURCE FORMS
 * ============================================================================
 *
 * Targeted policy:
 *
 *     fault_tolerance(workload) {
 *         failure: transient;
 *         recovery: retry_policy;
 *     }
 *
 * Untargeted policy:
 *
 *     fault_tolerance {
 *         failure: transient;
 *         recovery: retry_policy;
 *     }
 *
 * Multiple source-level arguments are permitted:
 *
 *     fault_tolerance(workload, execution_scope) {
 *         recovery: recoverable;
 *     }
 *
 * Resource/capability intent:
 *
 *     fault_tolerance(workload) {
 *         requires: capability("fault.recovery");
 *         requires: memory >= recovery_memory;
 *         prefer: local_recovery;
 *         constrain: preserve_semantics;
 *     }
 *
 * Nested policy:
 *
 *     fault_tolerance(workload) {
 *         recovery {
 *             strategy: custom::recovery;
 *             condition: recoverable;
 *         }
 *     }
 *
 * Extension:
 *
 *     fault_tolerance(workload) {
 *         vendor::extension: configuration;
 *     }
 *
 * The parser records structure. Semantic analysis decides meaning.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend AST must preserve, at minimum:
 *
 *     FaultToleranceDeclaration {
 *         arguments
 *         clauses
 *         source_span
 *     }
 *
 * Each clause must preserve:
 *
 *     - key;
 *     - value;
 *     - nested structure where applicable;
 *     - source span;
 *     - source ordering.
 *
 * The AST must NOT manufacture:
 *
 *     PhysicalNodeId
 *     PhysicalCpuId
 *     PhysicalGpuId
 *     PhysicalFpgaId
 *     PhysicalQpuId
 *     PhysicalQubitId
 *     NetworkAddress
 *     TransportSelection
 *     SchedulerAssignment
 *     PlacementDecision
 *     RuntimeRecoveryPlan
 *     QecPlan
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis classifies clauses into concepts such as:
 *
 *     - failure model;
 *     - detection intent;
 *     - recovery policy;
 *     - retry policy;
 *     - restart policy;
 *     - failover policy;
 *     - checkpoint policy;
 *     - degradation policy;
 *     - escalation policy;
 *     - availability requirement;
 *     - durability requirement;
 *     - resource requirement;
 *     - capability requirement;
 *     - constraint;
 *     - preference;
 *     - hint;
 *     - policy;
 *     - contract;
 *     - evidence;
 *     - provenance;
 *     - extension.
 *
 * Syntax validity does not imply semantic validity.
 *
 * ============================================================================
 * REQUIREMENT / CAPABILITY / CONSTRAINT / PREFERENCE SEPARATION
 * ============================================================================
 *
 * REQUIREMENT:
 *
 *     A condition that must hold for a valid realization.
 *
 * CAPABILITY:
 *
 *     Something the realization environment must provide.
 *
 * CONSTRAINT:
 *
 *     A condition imposed on an otherwise valid realization.
 *
 * PREFERENCE:
 *
 *     A desired property that may influence selection but is not mandatory.
 *
 * HINT:
 *
 *     Non-binding implementation guidance.
 *
 * POLICY:
 *
 *     A rule controlling permitted behavior.
 *
 * IMPLEMENTATION DECISION:
 *
 *     A concrete backend realization.
 *
 * This grammar MUST NOT collapse these concepts.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Fault-tolerance declarations may cause or require semantic effects such as:
 *
 *     distributed
 *     mutation
 *     IO
 *     network
 *     recovery
 *     checkpoint
 *     randomness
 *     foreign
 *     simulation
 *     quantum
 *
 * Effect classification belongs to the effect subsystem.
 *
 * This grammar does not invent a second effect system.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY CONTRACT
 * ============================================================================
 *
 * Fault tolerance may reference symbolic requirements:
 *
 *     requires: capability("fault.recovery");
 *
 *     requires: capability("checkpoint.restore");
 *
 *     requires: memory >= recovery_memory;
 *
 *     requires: storage >= checkpoint_storage;
 *
 *     requires: topology(required_topology);
 *
 *     prefer: capability("local.recovery");
 *
 * These expressions are passed to the resource/capability semantic layer.
 *
 * The grammar does not evaluate feasibility.
 *
 * ============================================================================
 * CONTRACT / POLICY CONTRACT
 * ============================================================================
 *
 * Fault-tolerance declarations may participate in the universal contract and
 * policy systems.
 *
 * Examples:
 *
 *     requires: recoverable_state;
 *     ensures: semantic_equivalence;
 *     invariant: state_consistent;
 *     assume: failure_model_known;
 *     guarantee: recovery_preserves_contract;
 *     property: deterministic_recovery;
 *
 * Policy examples:
 *
 *     allow: degraded_execution;
 *     forbid: unsafe_fallback;
 *     fallback: recovery_policy;
 *
 * The parser preserves these as source-level clauses.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Fault-tolerance intent may carry evidence and provenance:
 *
 *     evidence: reliability_measurement;
 *     provenance: analysis_record;
 *
 * Nested provenance is also allowed:
 *
 *     provenance {
 *         source: reliability_model;
 *         reason: validated_policy;
 *         evidence: test_result;
 *     }
 *
 * Provenance semantics belong to the universal provenance subsystem.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Fault tolerance may apply to distributed quantum or hybrid computation.
 *
 * This file MUST NOT define:
 *
 *     - gates;
 *     - physical qubits;
 *     - physical topology;
 *     - pulse schedules;
 *     - calibration;
 *     - syndrome extraction;
 *     - decoders;
 *     - QEC implementations;
 *     - ZQN instructions.
 *
 * Quantum semantic information ultimately crosses:
 *
 *     quantum::ir
 *
 * Recovery feasibility for quantum state is determined by the quantum semantic
 * layer, QEC layer, resilience layer and target capabilities.
 *
 * A source property such as:
 *
 *     checkpoint: logical_boundary;
 *
 * does NOT imply that arbitrary unknown quantum state can be copied.
 *
 * ============================================================================
 * HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * Fault tolerance can apply to HDL and hardware-aware computation, but this
 * grammar does not define:
 *
 *     - registers;
 *     - physical wires;
 *     - device IDs;
 *     - machine IDs;
 *     - fixed hardware capacities;
 *     - physical recovery circuitry.
 *
 * Hardware realization belongs to:
 *
 *     grammar/hardware/
 *     grammar/hdl/
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates NO independent fault-tolerance IR.
 *
 * The intended path is:
 *
 *     FaultToleranceDeclaration AST
 *             |
 *             v
 *     semantic fault-tolerance intent
 *             |
 *       +-----+----------------+
 *       |                      |
 *       v                      v
 * resource/capability     resilience semantic model
 *       |                      |
 *       +----------+-----------+
 *                  |
 *                  v
 *          canonical semantic IR
 *                  |
 *          +-------+-------+
 *          |               |
 *          v               v
 *     classical IR     quantum::ir
 *          |               |
 *          +-------+-------+
 *                  |
 *          optimization
 *                  |
 *          routing/scheduling
 *                  |
 *          resilience/QEC
 *                  |
 *                 ZQN
 *                  |
 *                 HAL
 *
 * There must be no separate:
 *
 *     FaultToleranceIR
 *     DistributedFaultToleranceIR
 *
 * solely because this syntax belongs to this file.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing is deterministic.
 *
 * It depends only on:
 *
 *     - source characters;
 *     - selected lexical vocabulary;
 *     - parser grammar;
 *     - selected language/compatibility version.
 *
 * Parsing MUST NOT depend on:
 *
 *     - hardware availability;
 *     - node count;
 *     - network state;
 *     - runtime state;
 *     - wall-clock time;
 *     - random state;
 *     - resource availability.
 *
 * ============================================================================
 * SOURCE ORDER
 * ============================================================================
 *
 * Clause order is preserved.
 *
 * Repeated clauses are syntactically representable.
 *
 * Semantic analysis decides whether a particular property is:
 *
 *     - singleton;
 *     - repeatable;
 *     - additive;
 *     - overriding;
 *     - mutually exclusive;
 *     - invalid when duplicated.
 *
 * The parser MUST NOT silently discard repeated clauses.
 *
 * ============================================================================
 * EXTENSIBILITY
 * ============================================================================
 *
 * Unknown semantic property names remain syntactically representable.
 *
 * Examples:
 *
 *     custom_recovery: policy;
 *
 *     vendor::fault_policy: configuration;
 *
 *     future::recovery::strategy: expression;
 *
 * This permits future semantic expansion without continuously expanding the
 * core parser.
 *
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * Parser diagnostics are structural.
 *
 * Semantic diagnostics include:
 *
 *     - invalid fault-tolerance target;
 *     - unknown failure model;
 *     - unknown recovery policy;
 *     - incompatible recovery policy;
 *     - invalid retry policy;
 *     - impossible recovery condition;
 *     - conflicting requirements;
 *     - unsatisfied capability;
 *     - unsatisfied resource requirement;
 *     - invalid contract;
 *     - invalid policy;
 *     - unsupported extension;
 *     - illegal target-specific implementation detail;
 *     - invalid quantum recovery assumption.
 *
 * Diagnostics must preserve source spans.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * The previous identifier-led form:
 *
 *     fault_tolerance target { ... }
 *
 * was structurally ambiguous with other distributed declarations.
 *
 * Production syntax therefore uses the reserved construct marker:
 *
 *     fault_tolerance(...)
 *
 * or:
 *
 *     fault_tolerance { ... }
 *
 * If compatibility with older source is required, the old spelling belongs
 * in a compatibility-profile grammar or migration tool. It must not be
 * reintroduced into the production distributed dispatch as another
 * identifier-led alternative.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive tests must cover:
 *
 *     fault_tolerance { }
 *
 *     fault_tolerance(workload) { }
 *
 *     fault_tolerance(workload, scope) {
 *         failure: transient;
 *         recovery: retry_policy;
 *     }
 *
 *     fault_tolerance(workload) {
 *         requires: capability("fault.recovery");
 *         requires: memory >= recovery_memory;
 *         prefer: local_recovery;
 *         constrain: preserve_semantics;
 *     }
 *
 *     fault_tolerance(workload) {
 *         recovery {
 *             strategy: custom::recovery;
 *             condition: recoverable;
 *         }
 *     }
 *
 *     fault_tolerance(workload) {
 *         vendor::extension: configuration;
 *     }
 *
 * Negative tests must cover:
 *
 *     fault_tolerance(
 *     fault_tolerance(workload
 *     fault_tolerance(workload) { recovery:
 *     fault_tolerance(workload) { recovery { }
 *
 * Boundary tests must cover:
 *
 *     - one logical execution context;
 *     - many execution contexts;
 *     - embedded;
 *     - CPU;
 *     - GPU;
 *     - FPGA;
 *     - accelerator;
 *     - QPU;
 *     - simulator;
 *     - HPC;
 *     - cluster;
 *     - heterogeneous execution;
 *     - quantum-classical execution.
 *
 * Scalability tests must verify that the grammar contains no fixed cardinality
 * for declarations, clauses, arguments, lists, stages, policies or resources.
 *
 * Determinism tests must parse identical source identically regardless of
 * target availability.
 *
 * Compatibility tests must verify that the new explicit declaration marker
 * cannot be confused with unrelated distributed constructs.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     [1] distributedFaultToleranceDeclaration is stable.
 *     [2] FaultTolerance has one unambiguous source entry form.
 *     [3] No identifier-led distributed declaration ambiguity remains here.
 *     [4] Failure models remain open-world.
 *     [5] Recovery strategies remain open-world.
 *     [6] Resource requirements remain symbolic.
 *     [7] Capabilities remain semantic.
 *     [8] Preferences remain distinct from requirements.
 *     [9] Constraints remain distinct from implementation decisions.
 *    [10] Policies remain distinct from capabilities.
 *    [11] Provenance can be represented.
 *    [12] Contracts can be represented.
 *    [13] Nested recovery structures are representable.
 *    [14] Extensions are representable.
 *    [15] No physical hardware is selected.
 *    [16] No finite hardware capacity is encoded.
 *    [17] No second IR is introduced.
 *    [18] Quantum semantics remain outside this grammar.
 *    [19] Resilience execution remains outside this grammar.
 *    [20] The distributed composition root requires no internal rewrite.
 *    [21] Parser behavior is deterministic.
 *    [22] Source order is preserved.
 *    [23] Rust implementation remains safe Rust.
 *    [24] Rust 1.97+ compatibility is maintained.
 *
 * ============================================================================
 */

parser grammar FaultTolerance;

options {
    tokenVocab = ZamaniLexer;
}

import Names, Expressions;


/*
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * This is the rule consumed by grammar/distributed/distributed.g4.
 */
distributedFaultToleranceDeclaration
    : faultToleranceDeclaration
    ;


/*
 * ============================================================================
 * DECLARATION
 * ============================================================================
 *
 * Canonical forms:
 *
 *     fault_tolerance { ... }
 *
 *     fault_tolerance(target) { ... }
 *
 *     fault_tolerance(target, scope) { ... }
 *
 * The arguments are ordinary Zamani expressions.
 *
 * They identify logical source entities or semantic scopes.
 * They are NOT physical deployment identifiers.
 */
faultToleranceDeclaration
    : FAULT_TOLERANCE
      faultToleranceInvocation?
      faultToleranceBody
      SEMICOLON?
    ;


/*
 * ============================================================================
 * OPTIONAL INVOCATION / TARGET ARGUMENTS
 * ============================================================================
 *
 * No finite argument count is imposed.
 */
faultToleranceInvocation
    : LPAREN
      optionalExpressionList
      RPAREN
    ;


/*
 * ============================================================================
 * BODY
 * ============================================================================
 */

faultToleranceBody
    : LBRACE
      faultToleranceMember*
      RBRACE
    ;


/*
 * ============================================================================
 * MEMBER
 * ============================================================================
 *
 * Three structural forms are supported:
 *
 *     key: expression;
 *     key { ... }
 *     qualified::extension: expression;
 *
 * Explicit keyword keys are accepted separately because many universal
 * Zamani semantic words are reserved lexical tokens rather than IDENTIFIER.
 */
faultToleranceMember
    : faultToleranceProperty
    | faultToleranceScope
    ;


/*
 * ============================================================================
 * PROPERTY
 * ============================================================================
 */

faultToleranceProperty
    : faultToleranceKey
      COLON
      faultToleranceValue
      SEMICOLON
    ;


/*
 * ============================================================================
 * PROPERTY KEY
 * ============================================================================
 *
 * The key is intentionally open-world.
 *
 * Reserved universal semantic words that may legally occur as fault-tolerance
 * properties are admitted explicitly.
 *
 * Domain-specific failure/recovery names remain ordinary identifiers.
 */
faultToleranceKey
    : identifier
    | REQUIRES
    | ENSURES
    | INVARIANT
    | ASSUME
    | GUARANTEE
    | PROPERTY
    | ASSERT
    | PROVE
    | VERIFY
    | VALIDATE
    | RESOURCE
    | RESOURCES
    | CAPABILITY
    | CAPABILITIES
    | REQUIREMENT
    | REQUIREMENTS
    | CONSTRAINT
    | CONSTRAINTS
    | PREFER
    | PREFERENCE
    | PREFERENCES
    | HINT
    | HINTS
    | TARGET
    | TARGETS
    | AVAILABILITY
    | RELIABILITY
    | RESILIENCE
    | POLICY
    | POLICIES
    | ALLOW
    | FORBID
    | PERMIT
    | DENY
    | FALLBACK
    | RETRY
    | RECOVER
    | ESCALATE
    | REJECT
    | SELECT
    | NEGOTIATE
    | SANDBOX
    | SIMULATE
    | DETERMINISTIC
    | REPRODUCIBLE
    | EFFECT
    | EFFECTS
    | MEASUREMENT
    | LEARNING
    | ADAPTATION
    | REFLECTION
    | CODE_GENERATION
    | DISTRIBUTED
    | NETWORK
    | NATIVE
    | FOREIGN
    | QUANTUM
    | HYBRID
    | CLASSICAL
    | EVIDENCE
    | EXPLAIN
    | EXPLANATION
    | PROVENANCE
    | SOURCE
    | DERIVATION
    | DECISION
    | DECISIONS
    | AUDIT
    | TRACE
    ;


/*
 * ============================================================================
 * NESTED SCOPE
 * ============================================================================
 *
 * Examples:
 *
 *     recovery {
 *         strategy: retry_policy;
 *         condition: recoverable;
 *     }
 *
 *     detection {
 *         strategy: health_monitor;
 *     }
 *
 *     checkpoint {
 *         policy: logical_boundary;
 *     }
 */
faultToleranceScope
    : faultToleranceScopeKey
      faultToleranceBody
    ;


faultToleranceScopeKey
    : faultToleranceKey
    ;


/*
 * ============================================================================
 * VALUE
 * ============================================================================
 *
 * Ordinary expressions remain owned by Expressions.
 *
 * Objects and lists are owned structurally by this feature because they are
 * part of fault-tolerance policy composition.
 */
faultToleranceValue
    : expression
    | faultToleranceObject
    | faultToleranceList
    ;


/*
 * ============================================================================
 * OBJECT
 * ============================================================================
 */

faultToleranceObject
    : LBRACE
      faultToleranceMember*
      RBRACE
    ;


/*
 * ============================================================================
 * LIST
 * ============================================================================
 *
 * No fixed cardinality exists.
 */
faultToleranceList
    : LBRACKET
      faultToleranceListElement*
      RBRACKET
    ;


/*
 * ============================================================================
 * LIST ELEMENT
 * ============================================================================
 */

faultToleranceListElement
    : expression
    | faultToleranceObject
    | faultToleranceList
    ;


/*
 * ============================================================================
 * STABLE SEMANTIC ADAPTERS
 * ============================================================================
 *
 * These aliases provide stable integration points for AST/semantic code
 * without introducing another expression language.
 */


/*
 * Logical fault-tolerance target.
 */
faultToleranceTarget
    : expression
    ;


/*
 * Failure model.
 */
faultToleranceFailure
    : expression
    ;


/*
 * Failure classification.
 */
faultToleranceFailureClassification
    : expression
    ;


/*
 * Failure domain.
 */
faultToleranceFailureDomain
    : expression
    ;


/*
 * Detection policy.
 */
faultToleranceDetection
    : expression
    ;


/*
 * Recovery strategy.
 */
faultToleranceRecovery
    : expression
    ;


/*
 * Retry strategy.
 */
faultToleranceRetry
    : expression
    ;


/*
 * Restart strategy.
 */
faultToleranceRestart
    : expression
    ;


/*
 * Failover strategy.
 */
faultToleranceFailover
    : expression
    ;


/*
 * Checkpoint policy.
 */
faultToleranceCheckpoint
    : expression
    ;


/*
 * Rollback policy.
 */
faultToleranceRollback
    : expression
    ;


/*
 * Resume policy.
 */
faultToleranceResume
    : expression
    ;


/*
 * Degradation policy.
 */
faultToleranceDegradation
    : expression
    ;


/*
 * Escalation policy.
 */
faultToleranceEscalation
    : expression
    ;


/*
 * Availability requirement.
 */
faultToleranceAvailability
    : expression
    ;


/*
 * Durability requirement.
 *
 * The identifier `durability` remains an open semantic name.
 */
faultToleranceDurability
    : expression
    ;


/*
 * Recovery condition.
 */
faultToleranceCondition
    : expression
    ;


/*
 * Recovery action.
 */
faultToleranceAction
    : expression
    ;


/*
 * Recovery dependency.
 */
faultToleranceDependency
    : expression
    ;


/*
 * Resource requirement.
 */
faultToleranceRequirement
    : expression
    ;


/*
 * Capability requirement.
 */
faultToleranceCapability
    : expression
    ;


/*
 * Resource constraint.
 */
faultToleranceConstraint
    : expression
    ;


/*
 * Resource preference.
 */
faultTolerancePreference
    : expression
    ;


/*
 * Implementation hint.
 */
faultToleranceHint
    : expression
    ;


/*
 * Policy.
 */
faultTolerancePolicy
    : expression
    ;


/*
 * Evidence.
 */
faultToleranceEvidence
    : expression
    ;


/*
 * Provenance.
 */
faultToleranceProvenance
    : expression
    ;


/*
 * Contract.
 */
faultToleranceContract
    : expression
    ;


/*
 * ============================================================================
 * EXTENSION CALL
 * ============================================================================
 *
 * Extension names remain qualified semantic names.
 *
 * Examples:
 *
 *     vendor::fault_policy(...)
 *     future::recovery_strategy(...)
 *
 * These names do not become global keywords.
 *
 * The actual extension syntax is represented as a property value or ordinary
 * expression, keeping extension semantics outside this grammar.
 *
 * ============================================================================
 * ARGUMENT ADAPTER
 * ============================================================================
 *
 * Stable helper for semantic consumers that need the declaration argument
 * sequence without duplicating the expression-list grammar.
 */
faultToleranceArguments
    : optionalExpressionList
    ;


/*
 * ============================================================================
 * SEMANTIC PROPERTY CATEGORIES
 * ============================================================================
 *
 * These names are documentation/semantic vocabulary, not a closed parser
 * enumeration:
 *
 * FAILURE:
 *
 *     failure
 *     failure_model
 *     classification
 *     failure_domain
 *     detection
 *
 * RECOVERY:
 *
 *     recovery
 *     retry
 *     restart
 *     failover
 *     checkpoint
 *     rollback
 *     resume
 *     degradation
 *     escalation
 *     action
 *     condition
 *     dependency
 *
 * REQUIREMENTS:
 *
 *     requires
 *     capability
 *     resource
 *     requirement
 *
 * REALIZATION GUIDANCE:
 *
 *     constraint
 *     prefer
 *     hint
 *
 * GOVERNANCE:
 *
 *     policy
 *     allow
 *     forbid
 *     fallback
 *
 * CORRECTNESS:
 *
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * ASSURANCE:
 *
 *     evidence
 *     provenance
 *     explanation
 *     audit
 *     trace
 *
 * Extensions remain open-world.
 *
 * ============================================================================
 * REPEATED CLAUSE SEMANTICS
 * ============================================================================
 *
 * Examples:
 *
 *     fault_tolerance(workload) {
 *         requires: capability("fault.recovery");
 *         requires: capability("checkpoint.restore");
 *     }
 *
 * The parser preserves both clauses.
 *
 * Semantic analysis decides whether repetition:
 *
 *     - accumulates;
 *     - merges;
 *     - overrides;
 *     - conflicts;
 *     - violates a singleton rule.
 *
 * The parser never silently discards a repeated property.
 *
 * ============================================================================
 * RESOURCE SCALABILITY
 * ============================================================================
 *
 * Expressions may represent concrete or symbolic quantities:
 *
 *     requires: memory >= recovery_memory;
 *     requires: storage >= checkpoint_storage;
 *     requires: replicas >= desired_redundancy;
 *
 * A concrete number is program data, not a universal limit.
 *
 * The grammar never interprets:
 *
 *     3
 *
 * as:
 *
 *     maximum retries = 3
 *
 * unless the enclosing semantic property explicitly gives it that meaning.
 *
 * ============================================================================
 * FAILURE-DOMAIN SEPARATION
 * ============================================================================
 *
 * The grammar may express semantic domains such as:
 *
 *     failure: node_failure;
 *     failure: communication_failure;
 *     failure: storage_failure;
 *     failure: process_failure;
 *
 * It does not select physical entities.
 *
 * For example:
 *
 *     node_failure
 *
 * does not mean:
 *
 *     physical node 0
 *
 * and:
 *
 *     communication_failure
 *
 * does not select TCP, UDP, QUIC, MPI, RDMA or another transport.
 *
 * ============================================================================
 * RECOVERY SEMANTICS
 * ============================================================================
 *
 * Recovery may be represented declaratively:
 *
 *     recovery {
 *         strategy: recover_policy;
 *         condition: recoverable;
 *         action: resume;
 *     }
 *
 * The semantic layer determines whether that strategy can actually be
 * realized.
 *
 * ============================================================================
 * ADAPTIVE RECOVERY
 * ============================================================================
 *
 * Adaptive policies remain declarative:
 *
 *     recovery {
 *         strategy: adaptive_policy;
 *         condition: capability("fault.adaptation");
 *     }
 *
 * Adaptation is not unrestricted self-modification.
 *
 * If adaptation changes executable state, model state, strategy or generated
 * code, the semantic layer must apply the universal:
 *
 *     effects
 *     capabilities
 *     resources
 *     policies
 *     authorization
 *     provenance
 *
 * contracts.
 *
 * ============================================================================
 * CHECKPOINT SEMANTICS
 * ============================================================================
 *
 * A checkpoint declaration expresses logical recovery intent:
 *
 *     checkpoint {
 *         policy: logical_boundary;
 *     }
 *
 * It does not guarantee that every state can be serialized.
 *
 * This distinction is particularly important for:
 *
 *     quantum
 *     hardware
 *     external resources
 *     nondeterministic state
 *     foreign state
 *
 * Semantic analysis determines checkpoint legality.
 *
 * ============================================================================
 * QUANTUM FAULT TOLERANCE
 * ============================================================================
 *
 * Distributed quantum fault tolerance remains compositional:
 *
 *     distributed fault-tolerance intent
 *             |
 *             v
 *     quantum semantic analysis
 *             |
 *             v
 *     quantum::ir
 *             |
 *             v
 *     QEC / resilience / routing / scheduling
 *             |
 *             v
 *     ZQN
 *             |
 *             v
 *     HAL
 *
 * This grammar never creates quantum operations or QEC objects.
 *
 * ============================================================================
 * HDL / HARDWARE FAULT TOLERANCE
 * ============================================================================
 *
 * Hardware-aware recovery remains target-independent.
 *
 * The grammar can express:
 *
 *     requires: capability("hardware.recovery");
 *     requires: capability("checkpoint.restore");
 *
 * but cannot express a universal physical mapping such as:
 *
 *     use_device(7)
 *
 * as a fault-tolerance implementation decision.
 *
 * ============================================================================
 * PROVENANCE
 * ============================================================================
 *
 * Fault-tolerance declarations may preserve:
 *
 *     source
 *     evidence
 *     derivation
 *     decision
 *     audit
 *     trace
 *
 * This allows later compiler/runtime systems to explain why a recovery plan
 * was selected without making explanation part of parser execution.
 *
 * ============================================================================
 * COMPATIBILITY / MIGRATION
 * ============================================================================
 *
 * Old identifier-led distributed fault-tolerance declarations should be
 * migrated to the explicit marker:
 *
 *     fault_tolerance(...)
 *
 * rather than retained as an ambiguous alternative.
 *
 * Compatibility tooling may perform the source rewrite before parsing under
 * the production grammar.
 *
 * ============================================================================
 * COMPLETION CONTRACT
 * ============================================================================
 *
 * This file is complete when:
 *
 *     - distributedFaultToleranceDeclaration remains the stable public rule;
 *     - FAULT_TOLERANCE is the sole declaration marker;
 *     - fault-tolerance dispatch is unambiguous;
 *     - failure vocabulary is open-world;
 *     - recovery vocabulary is open-world;
 *     - nested policy structures are supported;
 *     - resource/capability expressions are supported;
 *     - contracts are representable;
 *     - policies are representable;
 *     - evidence/provenance are representable;
 *     - no physical deployment is encoded;
 *     - no finite machine capacity is encoded;
 *     - no second IR is introduced;
 *     - quantum semantics remain behind quantum::ir;
 *     - resilience execution remains downstream;
 *     - source ordering is preserved;
 *     - parsing is deterministic;
 *     - Rust integration remains safe Rust 1.97+.
 *
 * ============================================================================
 */
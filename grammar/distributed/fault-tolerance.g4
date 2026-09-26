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
 *     Production distributed fault-tolerance parser component.
 *
 * Language baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust Edition 2021
 *
 * Safety:
 *     Safe Rust only.
 *     No embedded Rust actions.
 *     No unsafe code.
 *     No semantic predicates.
 *     No filesystem access.
 *     No network access.
 *     No hardware access.
 *     No runtime callbacks.
 *     No randomness.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar owns SOURCE-LEVEL DISTRIBUTED FAULT-TOLERANCE INTENT.
 *
 * It provides syntax for declaring:
 *
 *     - fault-tolerance policies;
 *     - failure models;
 *     - detection intent;
 *     - recovery intent;
 *     - retry/restart/failover intent;
 *     - checkpoint intent;
 *     - degradation intent;
 *     - escalation intent;
 *     - availability/durability requirements;
 *     - recovery stages;
 *     - recovery conditions;
 *     - recovery actions;
 *     - dependencies;
 *     - requirements;
 *     - constraints;
 *     - preferences;
 *     - hints;
 *     - extensible nested policy data.
 *
 * This grammar does NOT implement fault tolerance.
 *
 * It does not implement:
 *
 *     - failure detection;
 *     - health monitoring;
 *     - retry engines;
 *     - restart engines;
 *     - failover engines;
 *     - recovery orchestration;
 *     - checkpoint storage;
 *     - checkpoint restoration;
 *     - replica repair;
 *     - consensus;
 *     - consistency;
 *     - replication;
 *     - placement;
 *     - scheduling;
 *     - routing;
 *     - networking;
 *     - resource discovery;
 *     - hardware discovery;
 *     - QEC;
 *     - ZQN;
 *     - quantum::ir;
 *     - classical IR;
 *     - runtime execution.
 *
 * Those responsibilities belong to downstream subsystems.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     lexer
 *          |
 *          v
 *     parser
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          +--> name resolution
 *          +--> type analysis
 *          +--> effect analysis
 *          +--> resource analysis
 *          +--> capability analysis
 *          +--> security analysis
 *          +--> distributed semantic analysis
 *          +--> fault-tolerance semantic analysis
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> classical representation / IR
 *          +--> quantum::ir
 *          +--> HDL / hardware representation
 *          +--> distributed execution metadata
 *          +--> resilience metadata
 *          |
 *          v
 *     optimization
 *          |
 *          +--> placement
 *          +--> routing
 *          +--> scheduling
 *          +--> resilience
 *          |
 *          v
 *     target realization
 *          |
 *          v
 *     runtime
 *
 * This grammar is strictly upstream of implementation decisions.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Zamani's portability objective is:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Fault-tolerance syntax therefore describes semantic guarantees and
 * policies rather than physical deployment.
 *
 * The grammar must not require a source rewrite merely because a program
 * moves between:
 *
 *     - one execution context;
 *     - many execution contexts;
 *     - embedded systems;
 *     - multicore systems;
 *     - GPU systems;
 *     - FPGA systems;
 *     - accelerator systems;
 *     - clusters;
 *     - HPC systems;
 *     - clouds;
 *     - heterogeneous systems;
 *     - quantum/classical systems;
 *     - future computational substrates.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This grammar imposes no language-level finite limits on:
 *
 *     - fault-tolerance declarations;
 *     - policies;
 *     - failure models;
 *     - recovery stages;
 *     - recovery actions;
 *     - requirements;
 *     - constraints;
 *     - preferences;
 *     - hints;
 *     - nested policy objects;
 *     - expression lists;
 *     - qualified-name depth.
 *
 * Repetition is represented using ANTLR '*' and '+'.
 *
 * This grammar contains no:
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
 * Practical limits belong to compiler, runtime, operating-system, deployment,
 * and target resource policies.
 *
 * ============================================================================
 * OPEN-WORLD DESIGN
 * ============================================================================
 *
 * Fault-tolerance vocabulary is intentionally NOT a closed parser
 * enumeration.
 *
 * Semantic names may include:
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
 * and future names such as:
 *
 *     vendor::failure_model
 *     domain::custom_failure
 *
 * These names remain semantic data.
 *
 * A new failure model, recovery policy, detector, or resilience mechanism
 * should not require a lexer change merely because a new semantic name was
 * introduced.
 *
 * ============================================================================
 * LEXICAL AUTHORITY
 * ============================================================================
 *
 * This file introduces NO lexer rules.
 *
 * It introduces NO global keywords for:
 *
 *     fault
 *     failure
 *     retry
 *     restart
 *     failover
 *     recovery
 *     checkpoint
 *     degrade
 *     escalate
 *     availability
 *     durability
 *
 * The repository's existing lexer remains the lexical authority.
 *
 * In particular, this grammar does not invent a CONTRACT, FAULT, FAILURE,
 * RETRY, or RECOVER token that does not already exist in the canonical
 * lexical vocabulary.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - fault-tolerance declaration framing;
 *     - fault-tolerance property syntax;
 *     - fault-tolerance nested scopes;
 *     - fault-tolerance lists and objects;
 *     - fault-tolerance semantic property names;
 *     - fault-tolerance extension structure.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - identifiers;
 *     - qualified names;
 *     - expressions;
 *     - types;
 *     - lexical tokens;
 *     - replication;
 *     - consistency;
 *     - networking;
 *     - messaging;
 *     - placement;
 *     - scheduling;
 *     - resources;
 *     - hardware;
 *     - quantum operations;
 *     - quantum state;
 *     - QEC;
 *     - ZQN;
 *     - resilience implementation;
 *     - runtime execution.
 *
 * ============================================================================
 * DOMAIN SEPARATION
 * ============================================================================
 *
 * Fault tolerance is distinct from:
 *
 *     replication
 *     consistency
 *     consensus
 *     resilience orchestration
 *     resource management
 *     placement
 *     scheduling
 *     networking
 *
 * Therefore:
 *
 * replication.g4
 *     owns replication syntax.
 *
 * consistency.g4
 *     owns consistency syntax.
 *
 * fault-tolerance.g4
 *     owns fault-tolerance syntax.
 *
 * resilience subsystem
 *     owns recovery orchestration and execution policy.
 *
 * resource subsystem
 *     owns resource/capability feasibility.
 *
 * networking subsystem
 *     owns communication realization.
 *
 * scheduling subsystem
 *     owns executable ordering and scheduling.
 *
 * ============================================================================
 * CANONICAL SOURCE FORM
 * ============================================================================
 *
 * To avoid collision with the generic distributed container grammar, the
 * canonical structured form is:
 *
 *     fault_tolerance(target) {
 *         failure: transient;
 *         retry: adaptive;
 *         recovery: reconstructible;
 *     }
 *
 * or:
 *
 *     distributed::fault_tolerance(target) {
 *         failure: distributed::transient;
 *         recovery: custom::recovery_policy;
 *     }
 *
 * A declaration name and its target are therefore separated structurally
 * from the generic:
 *
 *     distributed_kind identifier { ... }
 *
 * container form.
 *
 * A simple invocation remains representable by the generic distributed
 * operation grammar:
 *
 *     distributed::fault_tolerance(policy);
 *
 * Such generic invocations do not belong to this component's specialized
 * structured declaration rule.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend AST must preserve:
 *
 *     - declaration source span;
 *     - declaration name;
 *     - target expression;
 *     - member ordering;
 *     - property names;
 *     - property values;
 *     - nested scopes;
 *     - list ordering;
 *     - object structure;
 *     - source expressions;
 *     - source spans for every syntactic element.
 *
 * The AST must not resolve:
 *
 *     - physical machines;
 *     - physical nodes;
 *     - hardware identifiers;
 *     - network addresses;
 *     - transport protocols;
 *     - physical replica locations;
 *     - physical topology;
 *     - provider-specific implementation choices.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis must determine:
 *
 *     - whether the declaration is valid;
 *     - whether the target exists;
 *     - whether the referenced failure model exists;
 *     - whether the requested policy exists;
 *     - whether an action is supported;
 *     - whether recovery is semantically possible;
 *     - whether retry conditions are valid;
 *     - whether checkpointing is legal;
 *     - whether durability requirements are achievable;
 *     * whether availability requirements are achievable;
 *     * whether requirements conflict;
 *     * whether capabilities exist;
 *     * whether resource requirements are satisfiable;
 *     * whether recovery dependencies are valid.
 *
 * Syntax alone must never imply that a policy is implementable.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Fault-tolerance declarations may surround distributed quantum computation.
 *
 * They MUST NOT define:
 *
 *     QubitId
 *     PhysicalQubitId
 *     Gate
 *     QuantumOperation
 *     QuantumState
 *     QuantumCircuit
 *     QEC code
 *     QEC decoder
 *     calibration
 *     pulse
 *     topology
 *     ZQN
 *
 * The canonical quantum semantic boundary remains:
 *
 *     quantum::ir
 *
 * Fault-tolerance syntax must not imply that arbitrary unknown quantum state
 * can be copied, serialized, checkpointed, or restored.
 *
 * Whether recovery is possible is determined by the quantum semantic,
 * execution, QEC, and runtime layers.
 *
 * ============================================================================
 * RESILIENCE INTEGRATION
 * ============================================================================
 *
 * Fault tolerance describes source-level intent.
 *
 * The resilience subsystem determines:
 *
 *     - when recovery is attempted;
 *     - how recovery is orchestrated;
 *     - whether degradation is entered;
 *     - whether retry occurs;
 *     - whether a backend is changed;
 *     - whether execution is quarantined;
 *     - whether recompilation is required;
 *     - whether execution is rejected.
 *
 * The grammar does not implement these decisions.
 *
 * ============================================================================
 * FAILURE STATE / OUTCOME INTEGRATION
 * ============================================================================
 *
 * The semantic/resilience layer may map fault-tolerance properties into the
 * repository's established resilience vocabulary:
 *
 *     Unknown
 *     Healthy
 *     Degraded
 *     Unstable
 *     Unavailable
 *     Recovering
 *     Quarantined
 *     Retired
 *
 * and outcomes:
 *
 *     ACCEPT
 *     DEGRADED_ACCEPT
 *     RETRY
 *     RECOVER
 *     ESCALATE
 *     REJECT
 *
 * These are semantic/runtime concepts, not lexer-level enumerations owned by
 * this grammar.
 *
 * ============================================================================
 * HARDWARE INTEGRATION
 * ============================================================================
 *
 * Hardware realization remains outside this grammar.
 *
 * No syntax here identifies:
 *
 *     CPU 0
 *     GPU 0
 *     FPGA 0
 *     QPU 0
 *     node 0
 *     physical qubit 0
 *     memory bank 0
 *
 * Resource requirements may be represented as expressions and are evaluated
 * by the resource/capability systems.
 *
 * ============================================================================
 * NETWORK INTEGRATION
 * ============================================================================
 *
 * This grammar does not select:
 *
 *     TCP
 *     UDP
 *     QUIC
 *     HTTP
 *     RPC
 *     MPI
 *     RDMA
 *     InfiniBand
 *     vendor transport
 *
 * A fault-tolerance policy may express communication-related intent, but
 * transport realization belongs to networking/runtime systems.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     - source text;
 *     - lexical rules;
 *     - parser rules;
 *     - selected grammar version.
 *
 * This grammar contains:
 *
 *     - no actions;
 *     - no semantic predicates;
 *     - no I/O;
 *     - no runtime callbacks;
 *     - no randomness;
 *     - no environment queries;
 *     - no hardware queries.
 *
 * ============================================================================
 * ERROR-RECOVERY CONTRACT
 * ============================================================================
 *
 * The grammar must permit the ANTLR parser to recover from malformed members
 * without embedding semantic recovery logic in parser actions.
 *
 * Semantic diagnostics must be produced downstream with source spans.
 *
 * Unknown property names should remain syntactically representable.
 *
 * An unknown semantic property is therefore a semantic diagnostic, not
 * automatically a parser error.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * This component introduces no new global lexer token.
 *
 * Existing programs using generic distributed invocation syntax remain owned
 * by distributed.g4.
 *
 * Structured fault-tolerance declarations use the explicit call-plus-body
 * form introduced by this component.
 *
 * If the language later promotes `fault_tolerance` to a reserved keyword,
 * that is a repository-wide lexical/compatibility change and must update:
 *
 *     lexer specification
 *     src/lexer.rs
 *     ANTLR lexer
 *     compatibility documentation
 *     parser conformance tests
 *
 * This file must not silently assume such a future token exists.
 *
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 */

parser grammar FaultTolerance;

options {
    tokenVocab = ZamaniLexer;
}

import Names, Expressions;


/*
 * ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * Structured fault-tolerance declaration:
 *
 *     fault_tolerance(target) {
 *         ...
 *     }
 *
 *     distributed::fault_tolerance(target) {
 *         ...
 *     }
 *
 * The name is intentionally open-world.
 */
distributedFaultToleranceDeclaration
    : faultToleranceDeclaration
    ;


/*
 * ============================================================================
 * 2. DECLARATION
 * ============================================================================
 *
 * A declaration consists of:
 *
 *     qualified name
 *     argument/target list
 *     structured body
 *
 * This structure is intentionally distinct from:
 *
 *     distributedKind identifier { ... }
 *
 * so the distributed aggregate grammar can integrate this component without
 * creating an ambiguous duplicate of distributedContainerDeclaration.
 */
faultToleranceDeclaration
    : qualifiedName
      LPAREN
      optionalExpressionList
      RPAREN
      faultToleranceBody
    ;


/*
 * ============================================================================
 * 3. BODY
 * ============================================================================
 *
 * Members are generic semantic properties or nested scopes.
 *
 * No closed taxonomy is encoded here.
 */
faultToleranceBody
    : LBRACE
      faultToleranceMember*
      RBRACE
    ;


/*
 * ============================================================================
 * 4. MEMBER
 * ============================================================================
 *
 * A member has one of two structural forms:
 *
 *     name : value ;
 *
 * or:
 *
 *     name { ... }
 *
 * The semantic layer classifies names such as:
 *
 *     failure
 *     detection
 *     retry
 *     restart
 *     failover
 *     recovery
 *     checkpoint
 *     degradation
 *     escalation
 *     availability
 *     durability
 *     requirement
 *     constraint
 *     preference
 *     hint
 *     dependency
 *
 * This avoids dozens of parser alternatives with identical syntax.
 */
faultToleranceMember
    : faultToleranceProperty
    | faultToleranceScope
    ;


/*
 * ============================================================================
 * 5. PROPERTY
 * ============================================================================
 *
 * Examples:
 *
 *     failure: transient;
 *     detection: health_monitor;
 *     retry: adaptive;
 *     restart: reconstructible;
 *     failover: permitted;
 *     recovery: custom::policy;
 *     checkpoint: logical_boundary;
 *     degradation: allowed;
 *     escalation: recover;
 *     availability: required;
 *     durability: persistent;
 *     requirement: capability("fault.recovery");
 *     constraint: preserve_semantics;
 *     preference: local_recovery;
 *     hint: idempotent;
 *     dependency: checkpoint_state;
 *
 * The parser does not assign special meaning to the property name.
 */
faultToleranceProperty
    : identifier
      COLON
      faultToleranceValue
      SEMICOLON
    ;


/*
 * ============================================================================
 * 6. NESTED SCOPE
 * ============================================================================
 *
 * Examples:
 *
 *     recovery {
 *         condition: recoverable;
 *         action: restart;
 *     }
 *
 *     stage {
 *         action: retry;
 *         dependency: checkpoint;
 *     }
 *
 *     failure_domain {
 *         failure: communication_failure;
 *     }
 *
 * The name is semantic data.
 */
faultToleranceScope
    : identifier
      faultToleranceBody
    ;


/*
 * ============================================================================
 * 7. VALUES
 * ============================================================================
 *
 * Values delegate ordinary expression semantics to the canonical expression
 * grammar while adding recursive fault-tolerance objects and lists.
 */
faultToleranceValue
    : expression
    | faultToleranceObject
    | faultToleranceList
    ;


/*
 * ============================================================================
 * 8. OBJECT
 * ============================================================================
 *
 * Objects preserve nested property structure.
 */
faultToleranceObject
    : LBRACE
      faultToleranceMember*
      RBRACE
    ;


/*
 * ============================================================================
 * 9. LIST
 * ============================================================================
 *
 * No finite list cardinality is imposed.
 */
faultToleranceList
    : LBRACKET
      faultToleranceListElement*
      RBRACKET
    ;


/*
 * ============================================================================
 * 10. LIST ELEMENT
 * ============================================================================
 *
 * Recursive lists and objects allow arbitrary semantic composition subject
 * only to actual parser/compiler resource limits.
 */
faultToleranceListElement
    : expression
    | faultToleranceObject
    | faultToleranceList
    ;


/*
 * ============================================================================
 * 11. OPTIONAL EXPRESSION LIST
 * ============================================================================
 *
 * This wrapper keeps the structured declaration independent while reusing
 * canonical expression syntax.
 *
 * `optionalExpressionList` is imported from Expressions.
 *
 * No expression grammar is duplicated here.
 */


/*
 * ============================================================================
 * 12. SEMANTIC PROPERTY CATEGORIES
 * ============================================================================
 *
 * The following names are DOCUMENTED semantic categories, not parser
 * alternatives.
 *
 * FAILURE / DETECTION:
 *
 *     failure
 *     failure_model
 *     failure_domain
 *     classification
 *     detection
 *
 * RECOVERY:
 *
 *     retry
 *     restart
 *     failover
 *     recovery
 *     action
 *     condition
 *     stage
 *     checkpoint
 *     rollback
 *     resume
 *
 * AVAILABILITY / DURABILITY:
 *
 *     availability
 *     durability
 *     degradation
 *
 * CONTROL:
 *
 *     dependency
 *     order
 *     barrier
 *     escalation
 *     acceptance
 *
 * PORTABILITY:
 *
 *     requirement
 *     capability
 *     constraint
 *     preference
 *     hint
 *
 * EXTENSIONS:
 *
 *     any implementation-defined semantic property name.
 *
 * These names must remain open-world.
 *
 * ============================================================================
 * 13. SEMANTIC CLASSIFICATION
 * ============================================================================
 *
 * Downstream semantic analysis may classify a property as:
 *
 *     requirement
 *     constraint
 *     preference
 *     hint
 *     capability
 *     policy
 *     action
 *     condition
 *     dependency
 *     extension
 *
 * Classification MUST NOT depend on parser rule identity because this grammar
 * intentionally represents the common structural form once.
 *
 * ============================================================================
 * 14. DUPLICATE PROPERTY POLICY
 * ============================================================================
 *
 * Repeated properties are syntactically valid and preserved.
 *
 * Example:
 *
 *     fault_tolerance(workload) {
 *         requirement: available;
 *         requirement: durable;
 *     }
 *
 * The semantic layer decides whether repeated properties:
 *
 *     - compose;
 *     - conflict;
 *     - override;
 *     - duplicate;
 *     - are invalid.
 *
 * The parser must not silently discard repeated properties.
 *
 * ============================================================================
 * 15. ORDER PRESERVATION
 * ============================================================================
 *
 * Source order is semantically observable until the semantic layer explicitly
 * proves that canonicalization is safe.
 *
 * The AST must preserve:
 *
 *     - member order;
 *     - list order;
 *     - argument order;
 *     - nested scope order;
 *     - source spans.
 *
 * The parser must not sort or normalize semantic properties.
 *
 * ============================================================================
 * 16. EXTENSIBILITY
 * ============================================================================
 *
 * A future property:
 *
 *     adaptive_recovery_budget: expression;
 *
 * requires no grammar change.
 *
 * A future nested structure:
 *
 *     recovery_strategy {
 *         custom_property: expression;
 *     }
 *
 * also requires no grammar change.
 *
 * New semantic vocabulary therefore remains a semantic/versioning concern
 * rather than a lexer/parser maintenance requirement.
 *
 * ============================================================================
 * 17. RESOURCE AND CAPABILITY EXPRESSIONS
 * ============================================================================
 *
 * Fault tolerance may reference the generic resource/capability system:
 *
 *     requirement: capability("fault.recovery");
 *
 *     requirement: memory >= required_memory;
 *
 *     requirement: qubits >= required_qubits;
 *
 *     capability: capability("quantum.measurement");
 *
 * These are expressions.
 *
 * This grammar does not define:
 *
 *     MAX_MEMORY
 *     MAX_QUBITS
 *     MAX_NODES
 *     MAX_REPLICAS
 *
 * and does not interpret resource feasibility.
 *
 * ============================================================================
 * 18. NO PHYSICAL REALIZATION
 * ============================================================================
 *
 * The following are deliberately outside this grammar:
 *
 *     physical_node
 *     physical_cpu
 *     physical_gpu
 *     physical_qpu
 *     physical_qubit
 *     physical_memory_bank
 *     physical_network_link
 *     host_address
 *     port
 *     machine_id
 *
 * A downstream realization may map logical fault-tolerance requirements onto
 * physical resources.
 *
 * ============================================================================
 * 19. QUANTUM RECOVERY RULE
 * ============================================================================
 *
 * A source expression such as:
 *
 *     recovery: checkpoint;
 *
 * does NOT mean arbitrary quantum state may be copied.
 *
 * Quantum recovery semantics must be validated against:
 *
 *     quantum::ir
 *     QEC
 *     ZQN
 *     runtime capabilities
 *     backend capabilities
 *
 * The grammar records intent only.
 *
 * ============================================================================
 * 20. CROSS-DOMAIN INTEGRATION
 * ============================================================================
 *
 * A fault-tolerance declaration may apply to computations involving:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     hardware
 *     AI
 *     distributed
 *     networking
 *     data
 *     nano
 *     future domains.
 *
 * The grammar does not duplicate the syntax of those domains.
 *
 * Cross-domain meaning is represented by expressions and references.
 *
 * ============================================================================
 * 21. CANONICAL IR CONTRACT
 * ============================================================================
 *
 * This grammar creates NO IR.
 *
 * After AST construction, semantic analysis may lower fault-tolerance intent
 * into the repository's canonical semantic representation.
 *
 * Distributed metadata may then feed:
 *
 *     resilience
 *     scheduling
 *     placement
 *     routing
 *     resource analysis
 *     runtime
 *
 * Quantum-related semantics continue through:
 *
 *     quantum::ir
 *
 * No fault-tolerance-specific quantum IR may be created here.
 *
 * ============================================================================
 * 22. DETERMINISTIC PARSING CONTRACT
 * ============================================================================
 *
 * The grammar contains:
 *
 *     no actions;
 *     no predicates;
 *     no I/O;
 *     no randomness;
 *     no runtime callbacks;
 *     no hardware discovery;
 *     no environment queries.
 *
 * Identical token streams therefore have identical syntactic interpretations
 * under the same grammar version.
 *
 * ============================================================================
 * 23. HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden universal resource limits are absent.
 *
 * In particular this file contains no grammar-level:
 *
 *     MAX_NODES
 *     MAX_REPLICAS
 *     MAX_RETRIES
 *     MAX_RECOVERY_STEPS
 *     MAX_FAILURES
 *     MAX_CHECKPOINTS
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
 * Repetition is open-ended.
 *
 * Numeric literals remain ordinary program expressions.
 *
 * ============================================================================
 * 24. RUST INTEGRATION
 * ============================================================================
 *
 * This is an ANTLR parser grammar.
 *
 * Rust-specific implementation requirements are enforced by the generated
 * parser/frontend build:
 *
 *     Rust 1.97 / Rust 1.97.1
 *     Edition 2021
 *     Safe Rust only
 *
 * This grammar contains no embedded Rust and therefore introduces no unsafe
 * implementation requirement.
 *
 * ============================================================================
 * 25. PRODUCTION COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when all of the following are true:
 *
 *     [x] single ownership boundary;
 *     [x] no duplicate fault-tolerance property rules;
 *     [x] no closed failure-model enumeration;
 *     [x] no hard-coded resource limits;
 *     [x] no new lexer keywords;
 *     [x] canonical Names dependency;
 *     [x] canonical Expressions dependency;
 *     [x] deterministic parsing;
 *     [x] source-order preservation contract;
 *     [x] AST contract;
 *     [x] semantic contract;
 *     [x] IR integration contract;
 *     [x] quantum integration contract;
 *     [x] resilience integration contract;
 *     [x] resource/capability integration contract;
 *     [x] compatibility contract;
 *     [x] extensibility contract;
 *     [x] error-recovery contract;
 *     [x] Rust 1.97/1.97.1 compatibility contract;
 *     [x] safe-Rust requirement;
 *     [x] scalability contract.
 *
 * Repository integration tests must additionally pass before the feature is
 * promoted to STABLE.
 *
 * ============================================================================
 */
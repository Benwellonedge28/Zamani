/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/execution/runtime-capabilities.g4
 *
 * Grammar:
 *     RuntimeCapabilities
 *
 * Status:
 *     PRODUCTION-READY RUNTIME CAPABILITY INTENT GRAMMAR
 *
 * Implementation baseline:
 *     Rust 2021
 *     Rust 1.97 / Rust 1.97.1
 *     safe Rust only
 *     no unsafe Rust
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar owns SOURCE-LEVEL RUNTIME CAPABILITY INTENT.
 *
 * It provides a stable syntax for expressing:
 *
 *     - mandatory runtime capability requirements;
 *     - preferred runtime capabilities;
 *     - advisory capability hints;
 *     - capability constraints;
 *     - capability availability predicates;
 *     - capability assertions;
 *     - conditional capability intent;
 *     - capability version requirements through the canonical capability model;
 *     - arbitrary/open-world capability identities;
 *     - arbitrary capability arguments;
 *     - capability property predicates.
 *
 * It intentionally does NOT:
 *
 *     - discover hardware;
 *     - inspect the host;
 *     - allocate resources;
 *     - select devices;
 *     - select CPUs;
 *     - select GPUs;
 *     - select FPGAs;
 *     - select QPUs;
 *     - select physical qubits;
 *     - perform scheduling;
 *     - perform routing;
 *     - perform placement;
 *     - perform calibration;
 *     - perform QEC;
 *     - perform ZQN;
 *     - execute runtime code;
 *     - access the filesystem;
 *     - access the network;
 *     - create a runtime-specific IR.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Runtime capability intent describes WHAT an execution environment must,
 * should, or may provide.
 *
 * It does not prescribe WHICH physical implementation must provide it.
 *
 * Examples:
 *
 *     requires capability("quantum.measurement");
 *
 *     requires capability("quantum.dynamic_control");
 *
 *     requires capability("tensor.compute");
 *
 *     prefer capability("accelerator.compute");
 *
 *     hint capability("distributed.collectives");
 *
 *     constraint capability("quantum.measurement") == true;
 *
 *     availability capability("quantum.measurement") when condition;
 *
 * No source-level capability declaration establishes a universal hardware
 * limit.
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This grammar MUST NOT encode language-wide limits such as:
 *
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_ASICS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_MEMORY
 *     MAX_STORAGE
 *     MAX_REGISTER_WIDTH
 *     MAX_VECTOR_WIDTH
 *     MAX_TENSOR_RANK
 *     MAX_NETWORK_SIZE
 *     MAX_TIMELINES
 *     MAX_TASKS
 *     MAX_PROCESSES
 *     MAX_CAPABILITIES
 *
 * The absence of such limits is intentional.
 *
 * "Infinity" means:
 *
 *     no artificial language-level finite ceiling is encoded here.
 *
 * Actual execution limits are determined downstream by:
 *
 *     - program requirements;
 *     - resource availability;
 *     - capability availability;
 *     - compiler resources;
 *     - runtime resources;
 *     - deployment policy;
 *     - target constraints;
 *     - operating-environment constraints.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     canonical ZamaniLexer
 *          |
 *          v
 *     canonical parser composition
 *          |
 *          v
 *     RuntimeCapabilities
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          +--> name resolution
 *          +--> type analysis
 *          +--> capability analysis
 *          +--> resource analysis
 *          +--> effect analysis
 *          +--> portability analysis
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> classical representation
 *          +--> quantum::ir
 *          +--> HDL/hardware representation
 *          +--> distributed representation
 *          |
 *          v
 *     optimization / lowering
 *          |
 *          +--> routing
 *          +--> scheduling
 *          +--> resilience
 *          +--> QEC
 *          +--> ZQN
 *          +--> HAL
 *          |
 *          v
 *     target realization
 *          |
 *          v
 *     runtime
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     runtimeCapabilityIntent
 *     runtimeCapabilityClause
 *     runtimeCapabilityRequirement
 *     runtimeCapabilityPreference
 *     runtimeCapabilityHint
 *     runtimeCapabilityConstraint
 *     runtimeCapabilityAvailability
 *     runtimeCapabilityAssertion
 *     runtimeCapabilityConditional
 *     runtimeCapabilitySpec
 *     runtimeCapabilityCall
 *     runtimeCapabilityArgumentList
 *     runtimeCapabilityPredicate
 *     runtimeCapabilityAvailabilityBody
 *
 * THIS FILE DOES NOT OWN:
 *
 *     identifier
 *     qualifiedName
 *     expression
 *     argumentList
 *     capabilityReference
 *     capabilityName
 *     capabilityVersionClause
 *     resourceRequirement
 *     resourceConstraint
 *     hardware topology
 *     scheduling
 *     placement
 *     routing
 *     target selection
 *     device discovery
 *     runtime implementation
 *     quantum IR
 *     classical IR
 *     HDL IR
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * Capability identity belongs to:
 *
 *     grammar/core/capabilities.g4
 *
 * General names belong to:
 *
 *     grammar/core/names.g4
 *
 * General expressions and argument lists belong to:
 *
 *     grammar/expressions/expressions.g4
 *
 * Resource requirements belong to:
 *
 *     grammar/resources/
 *
 * Runtime execution intent belongs to:
 *
 *     grammar/execution/
 *
 * This file MUST NOT redefine those systems.
 *
 * ============================================================================
 * IMPORT CONTRACT
 * ============================================================================
 *
 * The imports below are deliberate:
 *
 *     Core
 *         provides canonical core/name/block foundations.
 *
 *     Expressions
 *         provides expression and argument-list syntax.
 *
 *     Capabilities
 *         provides canonical capabilityReference,
 *         capabilityName, and capabilityVersionClause.
 *
 * No private duplicate identifier, qualified-name, capability-name, or
 * expression rules are introduced here.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The canonical lexical vocabulary is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * This grammar therefore consumes:
 *
 *     REQUIRES
 *     PREFER
 *     HINT
 *     CONSTRAINT
 *     CAPABILITY
 *     AVAILABILITY
 *     ASSERT
 *     WHEN
 *     LPAREN
 *     RPAREN
 *     LBRACE
 *     RBRACE
 *     COLON
 *     COMMA
 *     SEMI
 *
 * where those tokens are already part of the canonical lexer vocabulary.
 *
 * This file MUST NOT define lexer rules.
 *
 * It MUST NOT introduce private aliases such as:
 *
 *     K_REQUIRES
 *     K_CAPABILITY
 *     K_PREFER
 *
 * ============================================================================
 * NO PARSER-SIDE STRING KEYWORDS
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * Runtime capability vocabulary MUST therefore come from the canonical
 * ZamaniLexer.
 *
 * Do not replace canonical lexer tokens with parser literals such as:
 *
 *     'requires'
 *     'capability'
 *     'prefer'
 *
 * Doing so would create a second lexical authority.
 *
 * ============================================================================
 * OPEN-WORLD CAPABILITY MODEL
 * ============================================================================
 *
 * Capability identities are intentionally open-ended.
 *
 * Examples:
 *
 *     quantum
 *
 *     quantum::measurement
 *
 *     zamani::quantum::dynamic_control
 *
 *     tensor::compute
 *
 *     accelerator::vector_compute
 *
 *     distributed::collectives
 *
 *     future::architecture::feature
 *
 *     vendor::extension::feature
 *
 * The grammar does not enumerate those names.
 *
 * A new capability therefore does not require changing this file.
 *
 * ============================================================================
 * CAPABILITY REFERENCE VS CAPABILITY INVOCATION
 * ============================================================================
 *
 * Two forms are intentionally supported.
 *
 * 1. Canonical capability reference:
 *
 *     requires quantum::measurement;
 *
 * 2. Capability semantic call:
 *
 *     requires capability("quantum.measurement");
 *
 * The first form consumes the canonical capabilityReference model.
 *
 * The second form preserves an arbitrary expression argument list and is
 * interpreted semantically.
 *
 * Neither form selects a physical implementation.
 *
 * ============================================================================
 * CAPABILITY VERSIONING
 * ============================================================================
 *
 * Version syntax belongs to:
 *
 *     grammar/core/capabilities.g4
 *
 * Therefore:
 *
 *     requires quantum::measurement version >= 1.2;
 *
 * may be represented through capabilityReference and its canonical version
 * clause without introducing a second version grammar here.
 *
 * Version compatibility remains a semantic concern.
 *
 * ============================================================================
 * REQUIREMENT / PREFERENCE / HINT / CONSTRAINT
 * ============================================================================
 *
 * REQUIREMENT
 *
 *     Must be satisfied for the execution contract to be valid.
 *
 * PREFERENCE
 *
 *     A desired capability. Failure to honor it does not necessarily make
 *     execution invalid.
 *
 * HINT
 *
 *     Advisory information that downstream realization may use.
 *
 * CONSTRAINT
 *
 *     A mandatory predicate on an otherwise possible capability realization.
 *
 * ASSERTION
 *
 *     A semantic condition that the compiler/runtime must validate at the
 *     appropriate stage.
 *
 * These categories MUST remain distinct in the AST and semantic model.
 *
 * ============================================================================
 * RESOURCE SEPARATION
 * ============================================================================
 *
 * This file does NOT own resource quantities.
 *
 * For example:
 *
 *     requires qubits >= n;
 *
 * is resource intent and belongs to the resource subsystem.
 *
 * This file owns capability intent:
 *
 *     requires capability("quantum.measurement");
 *
 * The distinction is fundamental:
 *
 *     resource requirement != capability requirement
 *
 *     capability != resource
 *
 *     preference != requirement
 *
 *     hint != preference
 *
 *     capability != physical device
 *
 * ============================================================================
 * HARDWARE SEPARATION
 * ============================================================================
 *
 * This grammar does not contain:
 *
 *     CPU 0
 *     GPU 3
 *     FPGA 4
 *     QPU 2
 *     physical_qubit 17
 *     node 42
 *     memory_bank 7
 *
 * Such physical realization belongs downstream to target-specific
 * compilation/deployment layers.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum capability examples:
 *
 *     requires capability("quantum.measurement");
 *
 *     requires capability("quantum.mid_circuit_measurement");
 *
 *     requires capability("quantum.dynamic_control");
 *
 *     requires quantum::logical_qubits;
 *
 * These remain capability contracts.
 *
 * They do NOT create:
 *
 *     QuantumGate
 *     QubitId
 *     PhysicalQubitId
 *     topology
 *     calibration
 *     routing
 *     scheduling
 *     QEC
 *
 * Quantum computational meaning ultimately reaches:
 *
 *     quantum::ir
 *
 * This grammar does not create a competing quantum IR.
 *
 * ============================================================================
 * CLASSICAL / GPU / FPGA / ACCELERATOR INTEGRATION
 * ============================================================================
 *
 * The same open-world model supports:
 *
 *     requires capability("vector.compute");
 *
 *     requires capability("tensor.compute");
 *
 *     requires capability("gpu.compute");
 *
 *     requires capability("fpga.compute");
 *
 *     requires capability("accelerator.compute");
 *
 * No hardware count or physical identity is encoded.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Distributed capability examples:
 *
 *     requires capability("distributed.execution");
 *
 *     requires capability("distributed.collectives");
 *
 *     requires capability("networking.low_latency");
 *
 * Node quantities remain resource intent and therefore belong to
 * grammar/resources/.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Hardware capability examples:
 *
 *     requires capability("hardware.reconfiguration");
 *
 *     requires capability("hardware.streaming");
 *
 *     requires capability("hardware.synthesis");
 *
 *     requires capability("hardware.verification");
 *
 * Physical widths, topology, timing, placement, and synthesis realization
 * remain downstream.
 *
 * ============================================================================
 * AI / DATA INTEGRATION
 * ============================================================================
 *
 * Examples:
 *
 *     requires capability("tensor.compute");
 *
 *     requires capability("tensor.acceleration");
 *
 *     requires capability("distributed.training");
 *
 *     requires capability("data.streaming");
 *
 * Tensor rank, tensor dimensions, memory quantities, and accelerator counts
 * remain semantic/resource values.
 *
 * ============================================================================
 * CONDITIONAL CAPABILITY INTENT
 * ============================================================================
 *
 * Capability intent may depend on an ordinary Zamani expression:
 *
 *     when condition {
 *         requires capability("feature");
 *     }
 *
 * The condition is parsed as an ordinary expression.
 *
 * Its truth, type, effects, capabilities, and resource implications are
 * determined downstream.
 *
 * ============================================================================
 * SOURCE SPANS
 * ============================================================================
 *
 * The frontend AST MUST preserve source spans for:
 *
 *     - the capability keyword;
 *     - the capability reference/call;
 *     - every capability argument;
 *     - the condition, when present;
 *     - the predicate, when present;
 *     - the complete clause.
 *
 * This grammar does not define the Rust source-span type.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Conceptual mapping:
 *
 *     runtimeCapabilityRequirement
 *         -> RuntimeCapabilityRequirement
 *
 *     runtimeCapabilityPreference
 *         -> RuntimeCapabilityPreference
 *
 *     runtimeCapabilityHint
 *         -> RuntimeCapabilityHint
 *
 *     runtimeCapabilityConstraint
 *         -> RuntimeCapabilityConstraint
 *
 *     runtimeCapabilityAvailability
 *         -> RuntimeCapabilityAvailability
 *
 *     runtimeCapabilityAssertion
 *         -> RuntimeCapabilityAssertion
 *
 *     runtimeCapabilityConditional
 *         -> RuntimeCapabilityConditional
 *
 *     runtimeCapabilitySpec
 *         -> CapabilityReference / CapabilityInvocation
 *
 * The concrete Rust AST names remain owned by the frontend AST contract.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for:
 *
 *     - resolving capability identities;
 *     - resolving namespaces;
 *     - checking capability versions;
 *     - checking argument types;
 *     - checking effect requirements;
 *     - checking resource implications;
 *     - checking capability availability;
 *     - checking compatibility;
 *     - detecting conflicting requirements;
 *     - determining whether preferences can be honored;
 *     - determining whether hints are applicable;
 *     - validating assertions;
 *     - validating conditional capability intent;
 *     - performing portability analysis.
 *
 * The parser does none of those operations.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates no runtime-capability IR.
 *
 * After semantic analysis, capability information is attached to the
 * repository's canonical semantic representation and consumed by the
 * appropriate compiler/runtime stages.
 *
 * Quantum-related capability information ultimately integrates with:
 *
 *     quantum::ir
 *
 * There is no:
 *
 *     RuntimeQuantumIR
 *
 *     QuantumCapabilityIR
 *
 *     RuntimeCapabilityQuantumIR
 *
 * introduced here.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime implementation may:
 *
 *     - inspect the resolved capability contract;
 *     - negotiate capabilities;
 *     - choose an implementation;
 *     - select resources;
 *     - select targets;
 *     - apply fallback policy;
 *     - reject unsatisfied mandatory requirements;
 *     - honor preferences;
 *     - use hints.
 *
 * Those operations occur after parsing and semantic analysis.
 *
 * The parser itself never performs negotiation.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no semantic predicates;
 *     - no actions;
 *     - no embedded Rust;
 *     - no I/O;
 *     - no environment inspection;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware access;
 *     - no randomness;
 *     - no mutable parser-global state.
 *
 * Given the same source token stream and language/grammar version, parsing
 * produces the same structural result.
 *
 * ============================================================================
 * SAFETY
 * ============================================================================
 *
 * This grammar contains no Rust implementation code.
 *
 * The consuming implementation MUST:
 *
 *     - target Rust 1.97 / Rust 1.97.1;
 *     - use Rust 2021;
 *     - use safe Rust only;
 *     - contain no unsafe blocks;
 *     - contain no unsafe functions;
 *     - contain no unsafe traits;
 *     - perform runtime capability resolution outside the parser.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing execution grammar remains the execution-domain composition owner.
 *
 * This grammar is a leaf/component grammar.
 *
 * The intended integration direction is:
 *
 *     ZamaniParser
 *          |
 *          v
 *     Execution
 *          |
 *          v
 *     Runtime / execution context
 *          |
 *          v
 *     RuntimeCapabilities
 *
 * RuntimeCapabilities MUST NOT import Execution.
 *
 * RuntimeCapabilities MUST NOT import the Zamani root parser.
 *
 * RuntimeCapabilities MUST NOT create a second program entry point.
 *
 * ============================================================================
 * INTEGRATION WITH execution/runtime.g4
 * ============================================================================
 *
 * `runtime.g4` already owns runtime execution intent.
 *
 * Its runtime capability requirement rule should delegate capability syntax
 * to the public rule:
 *
 *     runtimeCapabilityIntent
 *
 * rather than redefining capability-reference/capability-call structure.
 *
 * Conceptually:
 *
 *     runtimeCapabilityRequirement
 *         : runtimeCapabilityIntent
 *         ;
 *
 * The exact wrapper remains owned by Runtime.
 *
 * This file therefore has no dependency on Runtime and can be completed and
 * validated independently.
 *
 * ============================================================================
 * INTEGRATION WITH execution/execution.g4
 * ============================================================================
 *
 * `execution.g4` remains the execution-domain composition root.
 *
 * It does not need to duplicate capability syntax.
 *
 * Capability intent reaches it through the Runtime/execution-context
 * composition path.
 *
 * ============================================================================
 * INTEGRATION WITH resources/
 * ============================================================================
 *
 * Runtime capability intent may coexist with resource intent:
 *
 *     requires capability("quantum.measurement");
 *
 *     requires qubits >= logical_qubits;
 *
 * The first is owned here.
 *
 * The second is owned by the resource subsystem.
 *
 * Neither grammar imports the other merely to duplicate syntax.
 *
 * Semantic analysis combines the resulting contracts.
 *
 * ============================================================================
 * INTEGRATION WITH core/capabilities.g4
 * ============================================================================
 *
 * This file consumes:
 *
 *     capabilityReference
 *
 *     capabilityName
 *
 *     capabilityVersionClause
 *
 * from:
 *
 *     grammar/core/capabilities.g4
 *
 * This is the single capability-identity authority.
 *
 * ============================================================================
 * INTEGRATION WITH expressions/
 * ============================================================================
 *
 * Capability calls consume:
 *
 *     argumentList
 *
 * Capability predicates consume:
 *
 *     expression
 *
 * Conditional conditions consume:
 *
 *     expression
 *
 * This file never defines a second expression grammar.
 *
 * ============================================================================
 * VALID SOURCE FORMS
 * ============================================================================
 *
 * Canonical capability reference:
 *
 *     requires quantum::measurement;
 *
 * Open-world capability call:
 *
 *     requires capability("quantum.measurement");
 *
 * Parameterized capability:
 *
 *     requires capability("quantum.operation", operation);
 *
 * Preference:
 *
 *     prefer capability("accelerator.compute");
 *
 * Hint:
 *
 *     hint capability("distributed.collectives");
 *
 * Constraint:
 *
 *     constraint capability("quantum.measurement") == enabled;
 *
 * Availability:
 *
 *     availability capability("quantum.measurement")
 *         when measurement_mode;
 *
 * Assertion:
 *
 *     assert capability("quantum.measurement") == enabled;
 *
 * Conditional:
 *
 *     when use_quantum {
 *         requires capability("quantum.measurement");
 *     }
 *
 * ============================================================================
 * NEGATIVE / NON-OWNED FORMS
 * ============================================================================
 *
 * These are intentionally NOT capability grammar:
 *
 *     requires qubits >= n;
 *
 *     requires memory >= required_memory;
 *
 *     requires nodes >= required_nodes;
 *
 * Those belong to resource requirements.
 *
 * These are also not portable runtime capability syntax:
 *
 *     GPU 0
 *     QPU 2
 *     physical_qubit 17
 *     CPU 4
 *     node 42
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * There is no finite grammar-level maximum for:
 *
 *     capability clauses;
 *     capability arguments;
 *     capability-name depth;
 *     expression depth;
 *     nested conditional capability blocks;
 *     source-file size;
 *     number of capabilities;
 *     number of execution targets;
 *     number of machines;
 *     number of CPUs;
 *     number of GPUs;
 *     number of FPGAs;
 *     number of QPUs;
 *     number of qubits;
 *     number of nodes;
 *     memory capacity;
 *     tensor rank;
 *     register width.
 *
 * Structural repetition is represented with:
 *
 *     *
 *     +
 *
 * and no machine-specific bound.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive:
 *
 *     requires capability("quantum.measurement");
 *
 *     requires capability("quantum.measurement", mode);
 *
 *     requires quantum::measurement;
 *
 *     requires quantum::measurement version >= 1.2;
 *
 *     prefer capability("gpu.compute");
 *
 *     hint capability("distributed.collectives");
 *
 *     constraint capability("tensor.compute") == enabled;
 *
 *     availability capability("quantum.measurement")
 *         when measurement_enabled;
 *
 *     assert capability("quantum.measurement") == enabled;
 *
 *     when use_quantum {
 *         requires capability("quantum.measurement");
 *     }
 *
 * Negative:
 *
 *     requires;
 *
 *     prefer;
 *
 *     hint;
 *
 *     constraint;
 *
 *     availability;
 *
 *     assert;
 *
 *     capability();
 *
 *     requires capability(;
 *
 *     requires capability("feature";
 *
 * Non-owned resource syntax:
 *
 *     requires qubits >= n;
 *
 *     requires memory >= required_memory;
 *
 * Boundary/scalability:
 *
 *     one capability;
 *
 *     many capability clauses;
 *
 *     deeply qualified capability names;
 *
 *     deeply nested expressions;
 *
 *     large capability argument lists;
 *
 *     large conditional blocks;
 *
 *     very large numeric requirement expressions;
 *
 *     arbitrarily large source programs subject only to implementation and
 *     available-resource limits.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     NO MAX_QUBITS
 *     NO MAX_CPUS
 *     NO MAX_CORES
 *     NO MAX_THREADS
 *     NO MAX_GPUS
 *     NO MAX_FPGAS
 *     NO MAX_ASICS
 *     NO MAX_QPUS
 *     NO MAX_NODES
 *     NO MAX_DEVICES
 *     NO MAX_MEMORY
 *     NO MAX_STORAGE
 *     NO MAX_REGISTER_WIDTH
 *     NO MAX_VECTOR_WIDTH
 *     NO MAX_TENSOR_RANK
 *     NO MAX_NETWORK_SIZE
 *     NO MAX_TIMELINES
 *     NO MAX_TASKS
 *     NO MAX_PROCESSES
 *
 * It also contains no finite enumeration of hardware vendors, devices,
 * processors, accelerators, qubits, nodes, or future architectures.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] canonical ZamaniLexer is consumed;
 *     [x] foundational parser grammars are imported;
 *     [x] canonical capability identity is reused;
 *     [x] canonical expression syntax is reused;
 *     [x] runtime capability ownership is explicit;
 *     [x] capability identities are open-world;
 *     [x] capability calls are open-world;
 *     [x] capability arguments remain expressions;
 *     [x] requirements are distinct from preferences;
 *     [x] preferences are distinct from hints;
 *     [x] hints are distinct from constraints;
 *     [x] constraints are distinct from assertions;
 *     [x] availability is represented without hardware discovery;
 *     [x] conditional capability intent is supported;
 *     [x] resource quantities remain outside this grammar;
 *     [x] physical devices remain outside this grammar;
 *     [x] no hardware limits are encoded;
 *     [x] no physical target is selected;
 *     [x] no resource is allocated;
 *     [x] no runtime is executed;
 *     [x] no second quantum IR exists;
 *     [x] quantum semantics can flow toward quantum::ir;
 *     [x] AST mapping is specified;
 *     [x] semantic mapping is specified;
 *     [x] IR integration is specified;
 *     [x] Runtime integration is specified;
 *     [x] resource integration is specified;
 *     [x] deterministic parsing is preserved;
 *     [x] safe-Rust integration is preserved;
 *     [x] positive tests are specified;
 *     [x] negative tests are specified;
 *     [x] scalability tests are specified.
 *
 * ============================================================================
 * GRAMMAR
 * ============================================================================
 */

parser grammar RuntimeCapabilities;

options {
    tokenVocab = ZamaniLexer;
}

import
    Core,
    Expressions,
    Capabilities
;


/*
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * Runtime owns runtime execution composition.
 *
 * This grammar owns the capability-intent subtree.
 *
 * No EOF is consumed here because this is a reusable imported grammar.
 * ============================================================================
 */

runtimeCapabilityIntent
    : runtimeCapabilityClause+
    ;


/*
 * ============================================================================
 * CLAUSE DISPATCH
 * ============================================================================
 */

runtimeCapabilityClause
    : runtimeCapabilityRequirement
    | runtimeCapabilityPreference
    | runtimeCapabilityHint
    | runtimeCapabilityConstraint
    | runtimeCapabilityAvailability
    | runtimeCapabilityAssertion
    | runtimeCapabilityConditional
    ;


/*
 * ============================================================================
 * CAPABILITY SPECIFICATION
 * ============================================================================
 *
 * Two canonical forms:
 *
 *     quantum::measurement
 *
 * and:
 *
 *     capability("quantum.measurement")
 *
 * The first uses the canonical capability identity model.
 *
 * The second is an open semantic invocation.
 * ============================================================================
 */

runtimeCapabilitySpec
    : capabilityReference
    | runtimeCapabilityCall
    ;


runtimeCapabilityCall
    : CAPABILITY
      LPAREN
      runtimeCapabilityArgumentList?
      RPAREN
    ;


runtimeCapabilityArgumentList
    : argumentList
    ;


/*
 * ============================================================================
 * REQUIREMENT
 * ============================================================================
 */

runtimeCapabilityRequirement
    : REQUIRES
      runtimeCapabilitySpec
      SEMI
    ;


/*
 * ============================================================================
 * PREFERENCE
 * ============================================================================
 *
 * A preference is not equivalent to a mandatory requirement.
 * ============================================================================
 */

runtimeCapabilityPreference
    : PREFER
      runtimeCapabilitySpec
      SEMI
    ;


/*
 * ============================================================================
 * HINT
 * ============================================================================
 *
 * Hints are advisory.
 * ============================================================================
 */

runtimeCapabilityHint
    : HINT
      runtimeCapabilitySpec
      SEMI
    ;


/*
 * ============================================================================
 * CONSTRAINT
 * ============================================================================
 *
 * The predicate remains an ordinary expression.
 * ============================================================================
 */

runtimeCapabilityConstraint
    : CONSTRAINT
      runtimeCapabilityPredicate
      SEMI
    ;


runtimeCapabilityPredicate
    : expression
    ;


/*
 * ============================================================================
 * AVAILABILITY
 * ============================================================================
 *
 * Availability syntax expresses a semantic condition.
 *
 * It does not probe the environment.
 * ============================================================================
 */

runtimeCapabilityAvailability
    : AVAILABILITY
      runtimeCapabilitySpec
      runtimeCapabilityAvailabilityBody?
      SEMI
    ;


runtimeCapabilityAvailabilityBody
    : WHEN
      expression
    ;


/*
 * ============================================================================
 * ASSERTION
 * ============================================================================
 *
 * Assertions are validated downstream.
 * ============================================================================
 */

runtimeCapabilityAssertion
    : ASSERT
      runtimeCapabilityPredicate
      SEMI
    ;


/*
 * ============================================================================
 * CONDITIONAL CAPABILITY INTENT
 * ============================================================================
 *
 * Example:
 *
 *     when use_quantum {
 *         requires capability("quantum.measurement");
 *     }
 *
 * The condition is ordinary Zamani expression syntax.
 * ============================================================================
 */

runtimeCapabilityConditional
    : WHEN
      expression
      LBRACE
      runtimeCapabilityClause*
      RBRACE
    ;
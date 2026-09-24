/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/hybrid/classical-quantum-boundary.g4
 *
 * Grammar:
 *     ClassicalQuantumBoundary
 *
 * Status:
 *     PRODUCTION HYBRID DOMAIN-BOUNDARY CONTRACT
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar owns the SOURCE-LEVEL boundary between classical and quantum
 * computation.
 *
 * It does NOT create a second classical language.
 *
 * It does NOT create a second quantum language.
 *
 * It does NOT create a hybrid IR.
 *
 * It does NOT create a second quantum IR.
 *
 * The canonical quantum semantic boundary remains:
 *
 *     quantum::ir
 *
 * The grammar describes only source syntax. Meaning is established later by:
 *
 *     source
 *       |
 *       v
 *     canonical lexer
 *       |
 *       v
 *     canonical parser
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic analysis
 *       |
 *       +-----------------------+
 *       |                       |
 *       v                       v
 *   classical semantics     quantum semantics
 *                               |
 *                               v
 *                           quantum::ir
 *       |                       |
 *       +-----------+-----------+
 *                   |
 *                   v
 *              optimization
 *                   |
 *                   v
 *                routing
 *                   |
 *                   v
 *               scheduling
 *                   |
 *                   v
 *            resilience / QEC
 *                   |
 *                   v
 *                  ZQN
 *                   |
 *                   v
 *                  HAL
 *                   |
 *                   v
 *             target realization
 *
 * ============================================================================
 * RUST IMPLEMENTATION BASELINE
 * ============================================================================
 *
 * Downstream implementation target:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *
 * This grammar contains no Rust actions and therefore introduces no unsafe
 * Rust requirement.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - explicit classical/quantum boundary constructs;
 *   - hybrid regions;
 *   - classical values supplied to quantum computations;
 *   - quantum results exposed to classical computation;
 *   - measurement-result flow;
 *   - classical control of quantum execution;
 *   - quantum operation parameter flow from classical expressions;
 *   - explicit classical/quantum conversion syntax;
 *   - hybrid synchronization syntax;
 *   - hybrid capability/resource intent at the boundary;
 *   - source-level domain-crossing annotations.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - identifiers;
 *   - qualified names;
 *   - expressions;
 *   - expression precedence;
 *   - statements;
 *   - blocks;
 *   - types;
 *   - function declarations;
 *   - modules;
 *   - quantum operation definitions;
 *   - quantum gate inventories;
 *   - quantum measurement implementation;
 *   - quantum registers;
 *   - quantum states;
 *   - QEC;
 *   - ZQN;
 *   - routing;
 *   - scheduling;
 *   - optimization;
 *   - hardware topology;
 *   - physical device selection;
 *   - physical qubit allocation;
 *   - runtime implementation;
 *   - classical IR;
 *   - quantum::ir.
 *
 * ============================================================================
 * SINGLE-OWNER INVARIANT
 * ============================================================================
 *
 * This file MUST NOT duplicate productions owned by:
 *
 *     grammar/expressions/
 *     grammar/statements/
 *     grammar/types/
 *     grammar/functions/
 *     grammar/quantum/
 *     grammar/classical/
 *     grammar/hardware/
 *     grammar/resources/
 *
 * Instead it composes those existing contracts.
 *
 * ============================================================================
 * TOKEN INVARIANT
 * ============================================================================
 *
 * The ONLY lexer vocabulary is:
 *
 *     ZamaniLexer
 *
 * Do NOT use obsolete names such as:
 *
 *     K_HYBRID
 *     K_QUANTUM
 *     K_CLASSICAL
 *     K_APPLY
 *     K_MEASURE
 *     K_LET
 *     K_IF
 *     K_ELSE
 *     K_TO
 *     K_CONVERT
 *     K_SYNCHRONIZE
 *     K_REQUIRES
 *     K_CAPABILITY
 *     SEMI
 *
 * The current canonical vocabulary uses the corresponding language tokens:
 *
 *     HYBRID
 *     QUANTUM
 *     CLASSICAL
 *     APPLY
 *     MEASURE
 *     LET
 *     IF
 *     ELSE
 *     TO
 *     CONVERT
 *     SYNCHRONIZE
 *     REQUIRES
 *     CAPABILITY
 *     SEMICOLON
 *
 * ============================================================================
 * QUANTUM EXTENSIBILITY
 * ============================================================================
 *
 * There is intentionally NO:
 *
 *     quantumGate : H | X | Y | Z | CNOT | ...
 *
 * Operation identity remains semantic data.
 *
 * The existing quantum operation grammar owns:
 *
 *     quantumOperationStatement
 *     quantumOperationApplication
 *     quantumOperationInvocation
 *     quantumOperationParameterClause
 *     quantumOperationTargetClause
 *     quantumOperationReference
 *
 * This boundary merely connects those operations with classical values and
 * control flow.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * This grammar must support:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * A source program describes:
 *
 *     WHAT computation is required
 *     WHAT values cross domains
 *     WHAT capabilities are required
 *     WHAT dependencies exist
 *
 * It does NOT describe:
 *
 *     WHICH CPU
 *     WHICH GPU
 *     WHICH FPGA
 *     WHICH ASIC
 *     WHICH QPU
 *     WHICH physical qubit
 *     WHICH memory bank
 *     WHICH network node
 *     WHICH accelerator instance
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * No universal limits are encoded here.
 *
 * In particular, this grammar MUST NOT introduce:
 *
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_REGISTER_WIDTH
 *     MAX_TENSOR_RANK
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *     MAX_PARAMETERS
 *     MAX_RESULTS
 *     MAX_OPERATIONS
 *     MAX_BRANCHES
 *     MAX_DOMAINS
 *
 * Repetition operators such as `*` and `+` are intentionally unbounded by
 * language semantics.
 *
 * Practical parser/compiler resource limits are implementation policy, not
 * language semantics.
 *
 * ============================================================================
 * REQUIREMENT / CAPABILITY SEPARATION
 * ============================================================================
 *
 * A source requirement such as:
 *
 *     requires qubits >= n;
 *
 * expresses semantic/resource intent.
 *
 * It does NOT allocate physical qubits.
 *
 * Likewise:
 *
 *     requires capability("quantum.measurement");
 *
 * expresses a capability requirement.
 *
 * Capability satisfaction belongs to semantic analysis/resource management/
 * compilation/HAL/runtime.
 *
 * ============================================================================
 * SEMANTIC BOUNDARY
 * ============================================================================
 *
 * This grammar does not decide whether:
 *
 *   - a value is classical or quantum;
 *   - a conversion is legal;
 *   - a measurement is available;
 *   - a result is deterministic;
 *   - a control executes on a host or device;
 *   - a target supports dynamic control;
 *   - synchronization is required physically;
 *   - a value is copied, moved, borrowed, encoded, or transformed;
 *   - a quantum result is logical or physical;
 *   - a resource requirement can be satisfied.
 *
 * Those are semantic/compiler/runtime decisions.
 *
 * ============================================================================
 */

parser grammar ClassicalQuantumBoundary;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * This is the only public entry point owned by this grammar.
 *
 * The Hybrid dispatcher should expose it through its hybrid construct
 * composition.
 * ============================================================================
 */

classicalQuantumBoundary
    : classicalQuantumBoundaryConstruct
    ;


/* ============================================================================
 * 2. BOUNDARY CONSTRUCT
 * ============================================================================
 *
 * Ordering is intentional.
 *
 * More specific boundary forms appear before generic bindings/expressions so
 * that the parser has a deterministic structural choice.
 * ============================================================================
 */

classicalQuantumBoundaryConstruct
    : hybridRegion
    | classicalToQuantumInvocation
    | quantumResultBinding
    | classicalControlledQuantum
    | measurementControlledQuantum
    | hybridConversion
    | hybridSynchronization
    | hybridRequirement
    | hybridCapability
    | hybridBoundaryBinding
    ;


/* ============================================================================
 * 3. HYBRID REGION
 * ============================================================================
 *
 * A hybrid region groups ordinary Zamani statements with explicit
 * classical/quantum boundary constructs.
 *
 * It does NOT create another statement grammar.
 * ============================================================================
 */

hybridRegion
    : HYBRID LBRACE hybridRegionItem* RBRACE
    ;


hybridRegionItem
    : classicalQuantumBoundaryConstruct
    | statement
    ;


/* ============================================================================
 * 4. CLASSICAL -> QUANTUM INVOCATION
 * ============================================================================
 *
 * The callee is a semantic name.
 *
 * Arguments use the canonical expression grammar.
 *
 * The grammar does not determine whether an argument is:
 *
 *     scalar
 *     tensor
 *     classical parameter
 *     runtime value
 *     measurement result
 *     resource descriptor
 *
 * Semantic analysis determines that.
 * ============================================================================
 */

classicalToQuantumInvocation
    : QUANTUM qualifiedName
      LPAREN argumentList? RPAREN
      quantumInvocationResultClause?
      SEMICOLON
    ;


quantumInvocationResultClause
    : RETURNING expression
    ;


/* ============================================================================
 * 5. QUANTUM RESULT -> CLASSICAL BINDING
 * ============================================================================
 *
 * A quantum result becomes a source-level value.
 *
 * The result's semantic type is determined downstream.
 * ============================================================================
 */

quantumResultBinding
    : LET IDENTIFIER
      typeAnnotation?
      ASSIGN
      quantumResultExpression
      SEMICOLON
    ;


quantumResultExpression
    : measurementResultExpression
    | observationResultExpression
    | quantumReadResultExpression
    | quantumInvocationExpression
    ;


measurementResultExpression
    : MEASURE
      LPAREN expression RPAREN
    ;


observationResultExpression
    : OBSERVE
      LPAREN expression RPAREN
    ;


quantumReadResultExpression
    : READ
      LPAREN expression RPAREN
    ;


quantumInvocationExpression
    : QUANTUM
      LPAREN qualifiedName
      LPAREN argumentList? RPAREN
      RPAREN
    ;


/* ============================================================================
 * 6. CLASSICAL BOUNDARY BINDING
 * ============================================================================
 *
 * This provides one canonical binding form for a value crossing the boundary.
 *
 * It deliberately delegates the actual value to the canonical expression
 * grammar.
 * ============================================================================
 */

hybridBoundaryBinding
    : LET IDENTIFIER
      typeAnnotation?
      ASSIGN
      hybridBoundaryValue
      SEMICOLON
    | IDENTIFIER
      ASSIGN
      hybridBoundaryValue
      SEMICOLON
    ;


hybridBoundaryValue
    : expression
    | quantumResultExpression
    | hybridConversionExpression
    ;


/* ============================================================================
 * 7. CLASSICAL CONTROL OF QUANTUM EXECUTION
 * ============================================================================
 *
 * The condition remains an ordinary Zamani expression.
 *
 * This supports:
 *
 *     if classical_condition {
 *         ...
 *     }
 *
 * including conditions derived from quantum measurements.
 *
 * Semantic analysis determines whether the condition is:
 *
 *     compile-time
 *     host-side runtime
 *     device-side runtime
 *     dynamic-circuit control
 *
 * ============================================================================
 */

classicalControlledQuantum
    : IF expression block
      ELSE block?
    ;


/* ============================================================================
 * 8. MEASUREMENT-CONTROLLED QUANTUM OPERATION
 * ============================================================================
 *
 * The operation itself is delegated to the canonical quantum operation
 * grammar.
 *
 * This file therefore never enumerates gates.
 *
 * Example semantic shape:
 *
 *     if measure(q) {
 *         apply operation(...) (...);
 *     }
 *
 * ============================================================================
 */

measurementControlledQuantum
    : IF measurementResultReference
      quantumOperationStatement
      ;
    ;


measurementResultReference
    : MEASURE
      LPAREN expression RPAREN
    | RESULT
      LPAREN expression RPAREN
    | MEASUREMENT
      LPAREN expression RPAREN
    ;


/* ============================================================================
 * 9. CLASSICAL PARAMETER -> QUANTUM OPERATION
 * ============================================================================
 *
 * This is the key hybrid parameter boundary.
 *
 * Quantum operation syntax remains owned by grammar/quantum/operations.g4.
 *
 * This wrapper permits an ordinary classical expression to become a source
 * value consumed by a quantum operation without introducing a second
 * operation grammar.
 *
 * ============================================================================
 */

classicalQuantumOperation
    : APPLY
      quantumOperationInvocation
      SEMICOLON
    ;


/* ============================================================================
 * 10. EXPLICIT DOMAIN CONVERSION
 * ============================================================================
 *
 * Conversion describes source-level intent.
 *
 * It does not specify representation, encoding, allocation, or transport.
 * ============================================================================
 */

hybridConversion
    : CONVERT
      LPAREN expression RPAREN
      TO hybridDomain
      SEMICOLON
    ;


hybridConversionExpression
    : CONVERT
      LPAREN expression RPAREN
      TO hybridDomain
    ;


hybridDomain
    : CLASSICAL
    | QUANTUM
    ;


/* ============================================================================
 * 11. HYBRID SYNCHRONIZATION
 * ============================================================================
 *
 * Synchronization is semantic.
 *
 * It does not encode:
 *
 *     clock frequency
 *     pulse duration
 *     queue latency
 *     transport latency
 *     host/device implementation
 *     network implementation
 *
 * ============================================================================
 */

hybridSynchronization
    : SYNCHRONIZE
      hybridSynchronizationScope?
      SEMICOLON
    ;


hybridSynchronizationScope
    : LPAREN expression RPAREN
    ;


/* ============================================================================
 * 12. RESOURCE / CAPABILITY REQUIREMENTS
 * ============================================================================
 *
 * These are source-level requirements.
 *
 * They do not perform device discovery.
 *
 * The expression remains open-ended and is interpreted by semantic/resource
 * analysis.
 * ============================================================================
 */

hybridRequirement
    : REQUIRES expression SEMICOLON
    ;


hybridCapability
    : CAPABILITY
      qualifiedName
      hybridCapabilityValue?
      SEMICOLON
    ;


hybridCapabilityValue
    : ASSIGN expression
    ;


/* ============================================================================
 * 13. CLASSICAL / QUANTUM DOMAIN ANNOTATION
 * ============================================================================
 *
 * This rule intentionally remains small.
 *
 * The annotation does not change the underlying type system.
 * ============================================================================
 */

hybridDomainAnnotation
    : AT hybridDomain
    ;


/* ============================================================================
 * 14. EXPLICIT QUANTUM -> CLASSICAL RESULT USE
 * ============================================================================
 *
 * Measurement and observation values are ordinary source expressions after
 * semantic validation.
 * ============================================================================
 */

quantumToClassicalValue
    : quantumResultExpression
    ;


/* ============================================================================
 * 15. CLASSICAL -> QUANTUM VALUE
 * ============================================================================
 *
 * Ordinary expressions remain the source of classical values.
 * ============================================================================
 */

classicalToQuantumValue
    : expression
    ;


/* ============================================================================
 * 16. GENERIC HYBRID VALUE
 * ============================================================================
 */

hybridValue
    : classicalToQuantumValue
    | quantumToClassicalValue
    | hybridConversionExpression
    ;


/* ============================================================================
 * 17. HYBRID CALL EXPRESSION
 * ============================================================================
 *
 * Generic expression-level domain crossing.
 *
 * No vendor API or physical target is encoded.
 * ============================================================================
 */

hybridCallExpression
    : qualifiedName
      LPAREN argumentList? RPAREN
    ;


/* ============================================================================
 * 18. HYBRID CONDITION
 * ============================================================================
 *
 * This remains an ordinary expression.
 * ============================================================================
 */

hybridCondition
    : expression
    ;


/* ============================================================================
 * 19. HYBRID TARGET
 * ============================================================================
 *
 * A target here is a semantic expression, not a physical device.
 *
 * It MUST NOT be interpreted by this grammar as:
 *
 *     cpu0
 *     gpu0
 *     qpu0
 *     qubit17
 *     node42
 *
 * Physical realization belongs downstream.
 * ============================================================================
 */

hybridTarget
    : expression
    ;


/* ============================================================================
 * 20. AST CONTRACT
 * ============================================================================
 *
 * The parser must preserve enough structure for the frontend AST to record:
 *
 *   - source span;
 *   - boundary kind;
 *   - producer domain;
 *   - consumer domain;
 *   - callee/reference;
 *   - operands;
 *   - parameters;
 *   - results;
 *   - control dependency;
 *   - conversion intent;
 *   - synchronization intent;
 *   - requirement/capability intent;
 *   - source ordering.
 *
 * Recommended domain-neutral AST shape:
 *
 *     Boundary
 *       kind
 *       producer
 *       consumer
 *       operation
 *       operands
 *       results
 *       condition
 *       attributes
 *       requirements
 *       capabilities
 *       source_span
 *
 * This grammar does NOT require a new AST type if the existing AST already
 * has an equivalent domain-neutral operation/boundary representation.
 *
 * ============================================================================
 * 21. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis MUST determine:
 *
 *   - whether a boundary is legal;
 *   - whether source and destination types are compatible;
 *   - whether a measurement result exists;
 *   - whether a classical expression can parameterize an operation;
 *   - whether dynamic control is supported by the selected execution model;
 *   - whether conversion is defined;
 *   - whether synchronization is required;
 *   - whether capabilities are satisfied;
 *   - whether resource requirements are satisfiable;
 *   - whether ownership/effect rules permit the crossing;
 *   - whether the operation is deterministic;
 *   - whether the result is classical, quantum, or another semantic value.
 *
 * The parser must not perform these checks.
 *
 * ============================================================================
 * 22. QUANTUM IR CONTRACT
 * ============================================================================
 *
 * There is exactly ONE canonical quantum semantic IR:
 *
 *     quantum::ir
 *
 * The required lowering path is:
 *
 *     classicalQuantumBoundary
 *             |
 *             v
 *        domain-neutral AST
 *             |
 *             v
 *       semantic boundary
 *             |
 *       +-----+------+
 *       |            |
 *       v            v
 *   classical     quantum
 *   semantics     semantics
 *                    |
 *                    v
 *                quantum::ir
 *
 * No:
 *
 *     HybridQuantumIR
 *     ClassicalQuantumIR
 *     QuantumBoundaryIR
 *     PhysicalHybridIR
 *
 * may be introduced by this grammar.
 *
 * ============================================================================
 * 23. RESOURCE / CAPABILITY CONTRACT
 * ============================================================================
 *
 * Valid:
 *
 *     requires qubits >= n;
 *     requires capability("quantum.measurement");
 *     requires capability("quantum.mid_circuit_measurement");
 *     requires memory >= required_memory;
 *
 * These are requirements, not allocations.
 *
 * This grammar does not decide whether they can be satisfied.
 *
 * ============================================================================
 * 24. HARDWARE CONTRACT
 * ============================================================================
 *
 * This file MUST NOT encode:
 *
 *     physical qubit IDs;
 *     physical CPU IDs;
 *     GPU IDs;
 *     FPGA IDs;
 *     QPU IDs;
 *     fixed topology;
 *     fixed memory;
 *     fixed register width;
 *     fixed accelerator count;
 *     fixed node count.
 *
 * Physical realization belongs downstream.
 *
 * ============================================================================
 * 25. QEC / ZQN CONTRACT
 * ============================================================================
 *
 * QEC is not implemented here.
 *
 * ZQN is not implemented here.
 *
 * The grammar may carry source-level requirements that downstream systems
 * consume, but it does not select:
 *
 *     error-correcting code;
 *     decoder;
 *     syndrome schedule;
 *     noise model;
 *     physical error rate;
 *     calibration;
 *     recovery policy.
 *
 * ============================================================================
 * 26. ROUTING / SCHEDULING CONTRACT
 * ============================================================================
 *
 * The boundary contains no physical routing or schedule.
 *
 * A classical/quantum dependency becomes semantic dependency data.
 *
 * Routing and scheduling later decide:
 *
 *     placement;
 *     ordering;
 *     communication;
 *     synchronization;
 *     resource allocation.
 *
 * ============================================================================
 * 27. DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no semantic predicates;
 *     no actions;
 *     no I/O;
 *     no hardware discovery;
 *     no randomness;
 *     no environment access;
 *     no runtime calls.
 *
 * For a fixed source, language version, and lexer vocabulary, parsing is
 * deterministic according to the canonical ANTLR parser configuration.
 *
 * ============================================================================
 * 28. SECURITY
 * ============================================================================
 *
 * This grammar:
 *
 *     does not execute source code;
 *     does not access hardware;
 *     does not access files;
 *     does not access networks;
 *     does not inspect credentials;
 *     does not invoke vendor APIs;
 *     does not execute Rust;
 *     does not require unsafe Rust.
 *
 * Hostile-input resource limits, if required by an implementation, belong to
 * explicit parser/compiler policy rather than language semantics.
 *
 * ============================================================================
 * 29. COMPATIBILITY
 * ============================================================================
 *
 * This file deliberately introduces no replacement for:
 *
 *     grammar/hybrid/classical-quantum.g4
 *
 * as a second language.
 *
 * Migration policy:
 *
 *     old classical-quantum syntax
 *              |
 *              v
 *     this canonical boundary
 *              |
 *              v
 *     same AST/semantic contracts
 *
 * Existing public syntax should be retained where compatible.
 *
 * Deprecated syntax must be marked through the normal compatibility system.
 *
 * ============================================================================
 * 30. INTEGRATION CONTRACT
 * ============================================================================
 *
 * The Hybrid composition grammar must expose:
 *
 *     classicalQuantumBoundary
 *
 * as one of its hybrid constructs.
 *
 * The canonical parser already imports:
 *
 *     Hybrid
 *
 * Therefore the integration direction is:
 *
 *     ClassicalQuantumBoundary
 *              |
 *              v
 *           Hybrid
 *              |
 *              v
 *        ZamaniParser
 *
 * NOT:
 *
 *     ClassicalQuantumBoundary
 *              |
 *              v
 *        ZamaniParser
 *
 * and NOT:
 *
 *     ZamaniParser
 *              |
 *              v
 *     ClassicalQuantumBoundary
 *              |
 *              v
 *        ZamaniParser
 *
 * No circular grammar composition is permitted.
 *
 * ============================================================================
 * 31. VALIDATION CONTRACT
 * ============================================================================
 *
 * validation/ must verify:
 *
 *   - canonical ZamaniLexer vocabulary;
 *   - no K_* legacy tokens;
 *   - no SEMI legacy token;
 *   - no fixed gate enumeration;
 *   - no hardware IDs as grammar limits;
 *   - no MAX_* universal limits;
 *   - no duplicate quantum operation grammar;
 *   - no duplicate expression grammar;
 *   - no duplicate type grammar;
 *   - no duplicate statement grammar;
 *   - no second quantum IR contract;
 *   - no semantic actions;
 *   - no predicates;
 *   - no unsafe Rust dependency.
 *
 * ============================================================================
 * 32. CONFORMANCE TEST MATRIX
 * ============================================================================
 *
 * Minimum positive cases:
 *
 *   hybrid {
 *       let result = measure(q);
 *   }
 *
 *   hybrid {
 *       quantum circuit(theta);
 *   }
 *
 *   hybrid {
 *       let result = observe(state);
 *   }
 *
 *   hybrid {
 *       if measure(q) {
 *           ...
 *       }
 *   }
 *
 *   hybrid {
 *       let value = convert(classical_value) to quantum;
 *   }
 *
 *   hybrid {
 *       let value = convert(quantum_value) to classical;
 *   }
 *
 *   hybrid {
 *       synchronize;
 *   }
 *
 *   hybrid {
 *       requires qubits >= n;
 *   }
 *
 *   hybrid {
 *       requires capability("quantum.measurement");
 *   }
 *
 * Positive scalability cases:
 *
 *   - arbitrarily many boundary statements;
 *   - arbitrarily many parameters;
 *   - arbitrarily many results;
 *   - arbitrarily many nested hybrid regions;
 *   - arbitrarily many domain crossings;
 *   - symbolic resource quantities;
 *   - symbolic qubit counts;
 *   - large classical expressions;
 *   - large operation argument lists.
 *
 * Negative syntax cases:
 *
 *   - missing closing brace;
 *   - missing closing parenthesis;
 *   - missing semicolon where required;
 *   - incomplete conversion;
 *   - incomplete invocation;
 *   - malformed capability expression;
 *   - malformed measurement expression.
 *
 * Semantic-negative cases must be tested downstream rather than rejected
 * solely by this grammar:
 *
 *   - unsupported hardware capability;
 *   - insufficient qubits;
 *   - invalid classical-to-quantum type;
 *   - unavailable measurement mode;
 *   - unsupported dynamic control;
 *   - impossible resource requirement.
 *
 * ============================================================================
 * 33. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *   [x] It has one parser grammar identity.
 *   [x] It uses ZamaniLexer.
 *   [x] It uses current canonical token names.
 *   [x] It does not define lexer rules.
 *   [x] It does not enumerate quantum gates.
 *   [x] It does not define physical resources.
 *   [x] It does not define machine limits.
 *   [x] It does not define a hybrid IR.
 *   [x] It preserves quantum::ir as the quantum semantic boundary.
 *   [x] It delegates expressions to the canonical expression grammar.
 *   [x] It delegates statements to the canonical statement grammar.
 *   [x] It delegates types to the canonical type grammar.
 *   [x] It delegates quantum operations to the canonical quantum grammar.
 *   [x] It separates syntax from semantic validation.
 *   [x] It supports classical-to-quantum value flow.
 *   [x] It supports quantum-to-classical result flow.
 *   [x] It supports measurement-dependent control.
 *   [x] It supports explicit conversion.
 *   [x] It supports synchronization intent.
 *   [x] It supports capability/resource intent.
 *   [x] It contains no universal resource ceiling.
 *   [x] It contains no unsafe Rust.
 *   [x] It is suitable for POCO-REAF architecture.
 *
 * ============================================================================
 */
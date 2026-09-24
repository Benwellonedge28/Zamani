/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/hybrid/quantum-classical.g4
 *
 * Grammar:
 *     HybridQuantumClassical
 *
 * Status:
 *     PRODUCTION HYBRID QUANTUM/CLASSICAL BOUNDARY
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust edition 2021
 *     safe Rust only
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar owns the SOURCE-SYNTAX BOUNDARY between classical and quantum
 * computation inside the canonical Zamani hybrid domain.
 *
 * It exists to express:
 *
 *     classical -> quantum
 *     quantum -> classical
 *     classical -> quantum -> classical
 *     measurement -> classical control
 *     classical value -> quantum parameter
 *     quantum result -> classical value
 *     hybrid regions
 *     hybrid calls
 *     explicit domain conversions
 *     hybrid synchronization
 *     hybrid capability/requirement intent
 *
 * This file is a SYNTAX COMPONENT.
 *
 * It does NOT create:
 *
 *     - an AST;
 *     - a semantic IR;
 *     - a classical IR;
 *     - a quantum IR;
 *     - a hardware model;
 *     - a runtime;
 *     - a scheduler;
 *     - a router;
 *     - a QEC implementation;
 *     - a ZQN implementation.
 *
 * ============================================================================
 * ARCHITECTURAL PIPELINE
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     canonical parser
 *          |
 *          v
 *     HybridQuantumClassical
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +-----------------------------+
 *          |                             |
 *          v                             v
 *     classical semantics          quantum semantics
 *          |                             |
 *          |                             v
 *          |                        quantum::ir
 *          |                             |
 *          +-------------+---------------+
 *                        |
 *                        v
 *                canonical semantic model
 *                        |
 *                        v
 *                  optimization
 *                        |
 *             +----------+----------+
 *             |          |          |
 *             v          v          v
 *          routing   scheduling  resilience
 *                                   |
 *                                   v
 *                                  ZQN
 *                                   |
 *                                   v
 *                                  QEC
 *                                   |
 *                                   v
 *                              hardware HAL
 *                                   |
 *                                   v
 *                            target realization
 *                                   |
 *                                   v
 *                                runtime
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - classical/quantum source-domain boundaries;
 *     - hybrid regions;
 *     - classical invocation of quantum computation;
 *     - quantum invocation from classical computation;
 *     - classical arguments crossing into quantum computation;
 *     - quantum results crossing into classical computation;
 *     - measurement-derived classical values;
 *     - classical control over quantum operations;
 *     - quantum execution result binding;
 *     - explicit hybrid value conversion;
 *     - source-level synchronization between domains;
 *     - hybrid requirements;
 *     - hybrid capabilities;
 *     - hybrid value-flow markers.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical rules;
 *     - lexer tokens;
 *     - identifiers;
 *     - names;
 *     - paths;
 *     - expression precedence;
 *     - general expressions;
 *     - general statements;
 *     - blocks;
 *     - declarations;
 *     - functions;
 *     - modules;
 *     - types;
 *     - quantum operation definitions;
 *     - quantum operation internals;
 *     - quantum targets;
 *     - quantum registers;
 *     - quantum states;
 *     - measurement implementation;
 *     - observables;
 *     - reset;
 *     - channels;
 *     - quantum error correction;
 *     - ZQN;
 *     - routing;
 *     - scheduling;
 *     - calibration;
 *     - topology;
 *     - hardware discovery;
 *     - physical allocation;
 *     - runtime behavior;
 *     - classical IR;
 *     - quantum::ir.
 *
 * ============================================================================
 * SINGLE-OWNER RULE
 * ============================================================================
 *
 * The repository MUST have exactly one effective owner for the public
 * classical/quantum boundary.
 *
 * This file is intended to become that owner in grammar/hybrid/.
 *
 * The existing:
 *
 *     grammar/hybrid/classical-quantum.g4
 *
 * MUST NOT remain an independently imported competing implementation.
 *
 * During migration, the old component may be:
 *
 *     - removed after repository-wide references are updated; OR
 *     - reduced to a compatibility/deprecation document; OR
 *     - excluded from canonical ANTLR composition.
 *
 * It MUST NOT define a second effective implementation of the same public
 * boundary.
 *
 * Likewise:
 *
 *     grammar/quantum/quantum-classical.g4
 *
 * owns quantum-domain interoperability details only where explicitly retained
 * by the quantum-domain architecture.
 *
 * It MUST NOT introduce a second hybrid public dispatcher.
 *
 * ============================================================================
 * ANTLR CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * The canonical lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * The canonical parser composition root is:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * The canonical universal grammar composition root is:
 *
 *     grammar/Zamani.g4
 *
 * This file therefore:
 *
 *     - defines parser rules only;
 *     - uses the canonical Zamani lexer vocabulary;
 *     - contains no lexer rules;
 *     - contains no lexer actions;
 *     - contains no embedded Rust;
 *     - contains no semantic predicates;
 *     - contains no filesystem access;
 *     - contains no network access;
 *     - contains no runtime calls.
 *
 * ============================================================================
 */

parser grammar HybridQuantumClassical;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * This is the stable parser-facing entry rule for the hybrid
 * classical/quantum boundary.
 *
 * The universal hybrid dispatcher should consume this rule exactly once.
 *
 * ============================================================================
 */

hybridQuantumClassicalConstruct
    : hybridQuantumClassicalRegion
    | classicalToQuantumCall
    | quantumToClassicalCall
    | quantumResultBinding
    | quantumParameterBinding
    | quantumControlledByClassical
    | measurementDrivenClassicalControl
    | hybridValueBinding
    | hybridConversion
    | hybridSynchronization
    | hybridRequirement
    | hybridCapability
    ;


/*
 * ============================================================================
 * 2. HYBRID REGION
 * ============================================================================
 *
 * A hybrid region groups ordinary Zamani computation and domain-crossing
 * constructs without creating another statement language.
 *
 * The contents remain ordinary Zamani statements plus explicitly owned
 * hybrid boundaries.
 *
 * ============================================================================
 */

hybridQuantumClassicalRegion
    : K_HYBRID
      LBRACE
      hybridQuantumClassicalItem*
      RBRACE
    ;


hybridQuantumClassicalItem
    : hybridQuantumClassicalBoundary
    | statement
    ;


hybridQuantumClassicalBoundary
    : classicalToQuantumCall
    | quantumToClassicalCall
    | quantumResultBinding
    | quantumParameterBinding
    | quantumControlledByClassical
    | measurementDrivenClassicalControl
    | hybridValueBinding
    | hybridConversion
    | hybridSynchronization
    ;


/*
 * ============================================================================
 * 3. CLASSICAL -> QUANTUM CALL
 * ============================================================================
 *
 * A classical computation may invoke a quantum callable.
 *
 * Example:
 *
 *     quantum call circuit(theta, input);
 *
 * The callable itself is resolved semantically.
 *
 * The grammar does NOT require:
 *
 *     - a particular quantum backend;
 *     - a particular QPU;
 *     - a particular simulator;
 *     - a fixed number of qubits;
 *     - a fixed number of arguments.
 *
 * ============================================================================
 */

classicalToQuantumCall
    : K_QUANTUM
      K_CALL
      qualifiedName
      LPAREN
      argumentList?
      RPAREN
      quantumCallResultClause?
      SEMI
    ;


quantumCallResultClause
    : K_RETURNING
      expression
    ;


/*
 * ============================================================================
 * 4. QUANTUM -> CLASSICAL CALL
 * ============================================================================
 *
 * A quantum callable may expose a classical result.
 *
 * The grammar records the domain crossing.
 *
 * The semantic layer determines:
 *
 *     - result type;
 *     - availability;
 *     - lifetime;
 *     - representation;
 *     - ownership;
 *     - synchronization requirements.
 *
 * ============================================================================
 */

quantumToClassicalCall
    : K_CLASSICAL
      K_CALL
      qualifiedName
      LPAREN
      argumentList?
      RPAREN
      SEMI
    ;


/*
 * ============================================================================
 * 5. QUANTUM RESULT BINDING
 * ============================================================================
 *
 * A quantum-produced value may become a classical source-level value.
 *
 * Examples:
 *
 *     let result = measure(q);
 *
 *     let result = observe(observable);
 *
 *     let result = read(source);
 *
 *     let result = quantum call circuit(theta);
 *
 * The actual result type remains a semantic/type-system responsibility.
 *
 * ============================================================================
 */

quantumResultBinding
    : K_LET
      IDENTIFIER
      typeAnnotation?
      ASSIGN
      quantumResultExpression
      SEMI
    ;


quantumResultExpression
    : measurementExpression
    | observationExpression
    | quantumReadExpression
    | quantumCallExpression
    ;


measurementExpression
    : K_MEASURE
      LPAREN
      quantumMeasurementSource
      RPAREN
    ;


observationExpression
    : K_OBSERVE
      LPAREN
      expression
      RPAREN
    ;


quantumReadExpression
    : K_READ
      LPAREN
      expression
      RPAREN
    ;


quantumCallExpression
    : K_QUANTUM
      K_CALL
      qualifiedName
      LPAREN
      argumentList?
      RPAREN
    ;


quantumMeasurementSource
    : quantumTargetList
    | expression
    ;


/*
 * ============================================================================
 * 6. MEASUREMENT RESULT REFERENCE
 * ============================================================================
 *
 * Measurement results are ordinary semantic values after crossing the
 * quantum/classical boundary.
 *
 * Their actual type is NOT encoded here.
 *
 * A reference may be:
 *
 *     measurement(...)
 *     result(...)
 *     an ordinary identifier resolved to a measurement-derived value.
 *
 * Semantic analysis determines whether an identifier is legally usable as a
 * measurement result.
 *
 * ============================================================================
 */

measurementResultReference
    : K_MEASUREMENT
      LPAREN
      expression
      RPAREN
    | K_RESULT
      LPAREN
      expression
      RPAREN
    | IDENTIFIER
    ;


/*
 * ============================================================================
 * 7. CLASSICAL VALUE -> QUANTUM PARAMETER
 * ============================================================================
 *
 * Classical expressions may supply parameters to quantum operations.
 *
 * Example:
 *
 *     apply rotation(theta) to q;
 *
 * The quantum operation grammar owns the operation invocation structure.
 *
 * This rule owns the explicit hybrid relationship between the classical
 * parameter expression and the quantum operation.
 *
 * ============================================================================
 */

quantumParameterBinding
    : K_APPLY
      quantumOperationReference
      quantumParameterArgumentClause?
      quantumOperationTargetClause
      SEMI
    ;


quantumParameterArgumentClause
    : LPAREN
      quantumParameterArgumentList?
      RPAREN
    ;


quantumParameterArgumentList
    : expression
      (COMMA expression)*
    ;


/*
 * ============================================================================
 * 8. CLASSICAL CONTROL -> QUANTUM
 * ============================================================================
 *
 * A classical condition may determine whether quantum computation executes.
 *
 * Example:
 *
 *     when result apply operation to q;
 *
 * The condition is still the canonical Zamani expression.
 *
 * No separate condition language is created.
 *
 * ============================================================================
 */

quantumControlledByClassical
    : K_WHEN
      expression
      K_APPLY
      quantumOperationReference
      quantumOperationParameterClause?
      quantumOperationTargetClause
      SEMI
    ;


/*
 * ============================================================================
 * 9. MEASUREMENT-DRIVEN QUANTUM CONTROL
 * ============================================================================
 *
 * This is the explicit dynamic-circuit boundary.
 *
 * Example:
 *
 *     if result {
 *         apply operation to q;
 *     }
 *
 * The body remains an ordinary Zamani block.
 *
 * Semantic analysis determines whether the condition actually depends on a
 * quantum measurement and whether the resulting dynamic dependency is valid.
 *
 * ============================================================================
 */

measurementDrivenClassicalControl
    : K_IF
      classicalQuantumCondition
      block
      hybridElseClause?
    ;


classicalQuantumCondition
    : expression
    ;


hybridElseClause
    : K_ELSE
      block
    ;


/*
 * ============================================================================
 * 10. EXPLICIT OPERATION CONTROL
 * ============================================================================
 *
 * This rule provides source-level operation control without enumerating a
 * fixed gate set.
 *
 * Operation names are resolved by semantic analysis.
 *
 * Valid operation names may include:
 *
 *     H
 *     X
 *     CNOT
 *     RX
 *     custom_operation
 *     namespace::operation
 *     vendor::operation
 *
 * without any of those names becoming grammar-level gate inventory.
 *
 * ============================================================================
 */

explicitClassicalQuantumOperation
    : K_APPLY
      quantumOperationReference
      quantumOperationParameterClause?
      quantumOperationTargetClause
      SEMI
    ;


/*
 * ============================================================================
 * 11. HYBRID VALUE BINDING
 * ============================================================================
 *
 * Explicit source-level binding for values participating in the
 * classical/quantum boundary.
 *
 * The value's semantic domain is determined downstream.
 *
 * ============================================================================
 */

hybridValueBinding
    : hybridValueDeclaration
    | hybridValueAssignment
    ;


hybridValueDeclaration
    : K_LET
      IDENTIFIER
      typeAnnotation?
      ASSIGN
      hybridValueExpression
      SEMI
    ;


hybridValueAssignment
    : IDENTIFIER
      ASSIGN
      hybridValueExpression
      SEMI
    ;


hybridValueExpression
    : expression
    | quantumResultExpression
    ;


/*
 * ============================================================================
 * 12. EXPLICIT HYBRID CONVERSION
 * ============================================================================
 *
 * Explicit conversion is available when a programmer wants to state that a
 * value crosses a type/domain boundary.
 *
 * Example:
 *
 *     convert(value to TargetType);
 *
 * This syntax does NOT decide:
 *
 *     - machine representation;
 *     - register width;
 *     - bit width;
 *     - precision;
 *     - physical encoding;
 *     - memory layout;
 *     - hardware implementation.
 *
 * Those belong downstream.
 *
 * ============================================================================
 */

hybridConversion
    : K_CONVERT
      LPAREN
      hybridConversionSource
      K_TO
      typeExpression
      RPAREN
    ;


hybridConversionSource
    : expression
    | quantumResultExpression
    ;


/*
 * ============================================================================
 * 13. CLASSICAL -> QUANTUM VALUE
 * ============================================================================
 *
 * A classical expression may be supplied to a quantum value constructor or
 * semantic operation.
 *
 * The constructor name remains open-ended.
 *
 * ============================================================================
 */

classicalToQuantumValue
    : K_FROM
      expression
    ;


quantumValueFromClassical
    : qualifiedName
      LPAREN
      classicalValueArgumentList?
      RPAREN
    ;


classicalValueArgumentList
    : expression
      (COMMA expression)*
    ;


/*
 * ============================================================================
 * 14. QUANTUM -> CLASSICAL VALUE
 * ============================================================================
 *
 * Explicitly exposes a quantum result as a classical semantic value.
 *
 * ============================================================================
 */

quantumToClassicalValue
    : K_TO
      K_CLASSICAL
      LPAREN
      quantumResultExpression
      RPAREN
    ;


/*
 * ============================================================================
 * 15. HYBRID SYNCHRONIZATION
 * ============================================================================
 *
 * Synchronization describes a SOURCE-LEVEL dependency boundary.
 *
 * It does NOT prescribe:
 *
 *     - latency;
 *     - machine barriers;
 *     - CPU fences;
 *     - GPU barriers;
 *     - QPU timing;
 *     - network synchronization;
 *     - pulse scheduling.
 *
 * Those are downstream implementation concerns.
 *
 * ============================================================================
 */

hybridSynchronization
    : hybridSynchronizationPrefix
      LPAREN
      hybridSynchronizationOperandList?
      RPAREN
      SEMI
    ;


hybridSynchronizationPrefix
    : K_SYNCHRONIZE
    | K_SYNC
    ;


hybridSynchronizationOperandList
    : hybridSynchronizationOperand
      (COMMA hybridSynchronizationOperand)*
    ;


hybridSynchronizationOperand
    : expression
    ;


/*
 * ============================================================================
 * 16. HYBRID REQUIREMENT
 * ============================================================================
 *
 * A requirement describes what the computation needs.
 *
 * It does NOT select a physical resource.
 *
 * Examples:
 *
 *     requires capability("quantum.measurement");
 *     requires qubits >= n;
 *     requires memory >= required_memory;
 *
 * Exact requirement expression semantics belong to the resource/capability
 * specification.
 *
 * ============================================================================
 */

hybridRequirement
    : K_REQUIRE
      expression
      SEMI
    ;


/*
 * ============================================================================
 * 17. HYBRID CAPABILITY
 * ============================================================================
 *
 * A capability identifies a semantic capability rather than a physical
 * machine.
 *
 * Examples:
 *
 *     capability("quantum.measurement")
 *     capability("quantum.mid_circuit_measurement")
 *     capability("classical.compute")
 *
 * Capability resolution belongs downstream.
 *
 * ============================================================================
 */

hybridCapability
    : K_CAPABILITY
      LPAREN
      expression
      RPAREN
      SEMI
    ;


/*
 * ============================================================================
 * 18. GENERIC HYBRID EXPRESSION
 * ============================================================================
 *
 * There is deliberately no separate hybrid expression language.
 *
 * A hybrid expression is a normal Zamani expression whose semantic graph may
 * cross computational domains.
 *
 * ============================================================================
 */

hybridExpression
    : expression
    ;


/*
 * ============================================================================
 * 19. DOMAIN CROSSING VALUE
 * ============================================================================
 *
 * A domain-crossing value may be represented by an ordinary expression.
 *
 * The semantic analyzer determines:
 *
 *     source domain;
 *     destination domain;
 *     type;
 *     ownership;
 *     lifetime;
 *     conversion;
 *     synchronization;
 *     effects;
 *     capabilities.
 *
 * ============================================================================
 */

domainCrossingValue
    : expression
    ;


/*
 * ============================================================================
 * 20. QUANTUM OPERATION ADAPTER
 * ============================================================================
 *
 * This rule deliberately delegates detailed quantum operation syntax to the
 * canonical quantum operation grammar.
 *
 * It MUST NOT enumerate quantum gates.
 *
 * ============================================================================
 */

quantumOperationReference
    : quantumOperationDesignator
    ;


quantumOperationParameterClause
    : LPAREN
      quantumParameterArgumentList?
      RPAREN
    ;


quantumOperationTargetClause
    : K_TO
      quantumTargetList
    ;


/*
 * ============================================================================
 * 21. QUANTUM RESULT ADAPTER
 * ============================================================================
 *
 * This adapter keeps the hybrid grammar independent of the implementation
 * details of measurement, observation, and quantum execution.
 *
 * ============================================================================
 */

quantumResult
    : quantumResultExpression
    ;


/*
 * ============================================================================
 * 22. HYBRID DOMAIN MARKER
 * ============================================================================
 *
 * A domain marker allows future semantic extensions without requiring this
 * grammar to know every future computational domain.
 *
 * ============================================================================
 */

hybridDomainMarker
    : qualifiedName
    ;


/*
 * ============================================================================
 * 23. SOURCE-LEVEL DOMAIN TRANSITION
 * ============================================================================
 *
 * This rule records an explicit transition between semantic domains.
 *
 * The domain names remain open-ended.
 *
 * ============================================================================
 */

hybridDomainTransition
    : hybridDomainMarker
      ARROW
      hybridDomainMarker
    ;


/*
 * ============================================================================
 * 24. HYBRID EXECUTION BOUNDARY
 * ============================================================================
 *
 * A boundary may identify the semantic transition without selecting a target.
 *
 * ============================================================================
 */

hybridExecutionBoundary
    : hybridDomainTransition
    ;


/*
 * ============================================================================
 * 25. SEMANTICALLY OPEN OPERATION TARGET
 * ============================================================================
 *
 * Targets are delegated to the canonical quantum target grammar.
 *
 * No physical qubit identifiers are introduced here.
 *
 * ============================================================================
 */

hybridQuantumTarget
    : quantumTargetList
    ;


/*
 * ============================================================================
 * 26. INTEGRATION CONTRACT
 * ============================================================================
 *
 * This grammar is intentionally dependent on existing canonical rules.
 *
 * Required externally supplied rules include:
 *
 *     statement
 *     block
 *     expression
 *     typeExpression
 *     typeAnnotation
 *     argumentList
 *     qualifiedName
 *
 * Quantum-domain rules include:
 *
 *     quantumOperationDesignator
 *     quantumTargetList
 *
 * These rules are supplied by the canonical grammar composition tree.
 *
 * THIS FILE MUST NOT copy their implementations.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 27. AST CONTRACT
 * ============================================================================
 *
 * No AST types are defined here.
 *
 * The frontend MUST lower these constructs into the existing domain-neutral
 * AST.
 *
 * A hybrid boundary must preserve at least:
 *
 *     - source span;
 *     - source ordering;
 *     - operation/value identity;
 *     - argument ordering;
 *     - target ordering;
 *     - condition structure;
 *     - conversion target type;
 *     - synchronization operands;
 *     - requirement expression;
 *     - capability expression;
 *     - domain-crossing relationship.
 *
 * The grammar MUST NOT require:
 *
 *     QuantumGate
 *     ClassicalInstruction
 *     QuantumInstruction
 *     QubitId
 *     PhysicalQubitId
 *
 * AST nodes.
 *
 * Generic semantic operation data remains compatible with the repository's
 * established operation model:
 *
 *     name
 *     namespace
 *     operands
 *     parameters
 *     results
 *     attributes
 *     modifiers
 *     effects
 *     capabilities
 *     source
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 28. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis owns:
 *
 *     - domain classification;
 *     - type checking;
 *     - quantum/classical type compatibility;
 *     - measurement-result validation;
 *     - classical-control validation;
 *     - dynamic dependency validation;
 *     - conversion legality;
 *     - conversion losslessness;
 *     - ownership;
 *     - borrowing;
 *     - lifetime;
 *     - effect propagation;
 *     - capability validation;
 *     - resource requirements;
 *     - compile-time/runtime classification;
 *     - host/device classification;
 *     - synchronization requirements;
 *     - determinism analysis;
 *     - target compatibility.
 *
 * Parser acceptance MUST NOT be mistaken for semantic validity.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 29. QUANTUM IR CONTRACT
 * ============================================================================
 *
 * Quantum constructs crossing this boundary MUST eventually lower through:
 *
 *     frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     quantum::ir
 *
 * `quantum::ir` remains the canonical quantum semantic boundary.
 *
 * This file MUST NOT define:
 *
 *     QuantumGate
 *     QuantumInstruction
 *     QuantumCircuitIR
 *     HybridQuantumIR
 *     HybridQuantumClassicalIR
 *     QubitId
 *     PhysicalQubitId
 *
 * or equivalent duplicate structures.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 30. CLASSICAL IR CONTRACT
 * ============================================================================
 *
 * Classical portions must lower through the repository's canonical classical
 * semantic/IR representation.
 *
 * This grammar MUST NOT create:
 *
 *     HybridClassicalIR
 *     ClassicalInstruction
 *     HybridInstruction
 *
 * merely to represent the source boundary.
 *
 * The relationship between classical and quantum computation is semantic
 * information preserved through the existing frontend and canonical IR
 * pipeline.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 31. POCO-REAF CONTRACT
 * ============================================================================
 *
 * This grammar expresses PORTABLE COMPUTATIONAL INTENT.
 *
 * It MUST NOT encode a particular machine realization.
 *
 * Therefore it contains no language-level limits for:
 *
 *     qubits
 *     classical bits
 *     registers
 *     parameters
 *     operations
 *     measurements
 *     controls
 *     branches
 *     iterations
 *     devices
 *     nodes
 *     processors
 *     cores
 *     threads
 *     GPUs
 *     FPGAs
 *     accelerators
 *     memory
 *     tensor dimensions
 *     tensor rank
 *     network size
 *     timelines
 *     circuit depth
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 32. HARD-CODING PROHIBITION
 * ============================================================================
 *
 * The following MUST NEVER become grammar-level limits:
 *
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *
 * Nor may this grammar encode:
 *
 *     physical_qubit(0)
 *     cpu(0)
 *     gpu(0)
 *     fpga(0)
 *     device(0)
 *
 * as universal source semantics.
 *
 * Resource requirements MAY be expressed symbolically.
 *
 * Examples:
 *
 *     requires qubits >= n;
 *     requires memory >= required_memory;
 *     requires capability("quantum.measurement");
 *     requires capability("gpu.compute");
 *
 * Actual realization belongs downstream.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 33. OPEN-ENDED QUANTUM OPERATIONS
 * ============================================================================
 *
 * The hybrid grammar MUST NOT enumerate quantum operations.
 *
 * It must therefore remain compatible with:
 *
 *     apply H to q;
 *     apply X to q;
 *     apply CNOT to q0, q1;
 *     apply RX(theta) to q;
 *     apply custom_operation to q;
 *     apply namespace::operation to q;
 *     apply vendor::operation(theta) to q;
 *
 * Operation meaning is resolved by semantic analysis.
 *
 * This keeps the language extensible without requiring a grammar edit every
 * time a new quantum operation is introduced.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 34. MEASUREMENT CONTRACT
 * ============================================================================
 *
 * Measurement crosses from quantum semantics into classical semantics.
 *
 * The grammar does not prescribe:
 *
 *     - result bit width;
 *     - result encoding;
 *     - readout register layout;
 *     - readout latency;
 *     - physical measurement technology;
 *     - physical measurement device.
 *
 * Those are semantic/backend concerns.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 35. DYNAMIC-CIRCUIT CONTRACT
 * ============================================================================
 *
 * Classical expressions may depend upon:
 *
 *     - ordinary classical values;
 *     - function results;
 *     - measurement results;
 *     - compile-time values;
 *     - runtime values;
 *     - values supplied by other computational domains.
 *
 * The grammar does not determine when those values become available.
 *
 * Scheduling, execution, and semantic analysis determine the dependency
 * realization.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 36. CONVERSION CONTRACT
 * ============================================================================
 *
 * Explicit conversion syntax does not imply that every conversion is legal.
 *
 * Semantic analysis MUST determine:
 *
 *     - source type;
 *     - destination type;
 *     - domain;
 *     - representation;
 *     - precision;
 *     - lossiness;
 *     - ownership;
 *     - effects;
 *     - required capabilities.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 37. SYNCHRONIZATION CONTRACT
 * ============================================================================
 *
 * Source-level synchronization expresses dependency.
 *
 * It MUST NOT be interpreted by this grammar as:
 *
 *     CPU fence
 *     GPU barrier
 *     FPGA clock barrier
 *     QPU pulse barrier
 *     network barrier
 *     memory fence
 *
 * Those are target-level realizations.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 38. RESOURCE CONTRACT
 * ============================================================================
 *
 * Requirements and capabilities are declarative.
 *
 * A requirement says:
 *
 *     what is needed.
 *
 * A capability says:
 *
 *     what semantic ability is required or available.
 *
 * Neither selects a physical resource.
 *
 * Preferences and implementation decisions remain downstream.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 39. FUTURE-DOMAIN CONTRACT
 * ============================================================================
 *
 * The classical/quantum boundary must remain extensible.
 *
 * Future domains may participate through the same semantic boundary model.
 *
 * Examples include:
 *
 *     accelerator
 *     photonic
 *     neuromorphic
 *     biological
 *     optical
 *     distributed
 *     AI
 *     HDL
 *     simulation
 *     future computational domains
 *
 * This grammar MUST NOT require a new parser architecture merely because a
 * new computational domain is introduced.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 40. DETERMINISM CONTRACT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no actions;
 *     - no semantic predicates;
 *     - no mutable parser state;
 *     - no randomness;
 *     - no environment inspection;
 *     - no hardware inspection;
 *     - no filesystem access;
 *     - no networking.
 *
 * Identical token streams and parser configuration must therefore yield the
 * same structural parse.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 41. SAFE-RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains no embedded Rust.
 *
 * The generated parser and compiler integration MUST remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and safe Rust only.
 *
 * No unsafe Rust is required or authorized by this grammar.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 42. ERROR CONTRACT
 * ============================================================================
 *
 * Structural syntax errors belong to the parser.
 *
 * Examples:
 *
 *     missing call name;
 *     malformed argument list;
 *     malformed quantum target list;
 *     malformed conversion;
 *     malformed synchronization;
 *     malformed requirement;
 *     malformed capability;
 *     malformed hybrid region;
 *     malformed classical control.
 *
 * Semantic errors belong downstream.
 *
 * Examples:
 *
 *     classical value incompatible with quantum parameter;
 *     measurement result used before availability;
 *     invalid domain conversion;
 *     unavailable capability;
 *     insufficient resources;
 *     unsupported dynamic dependency;
 *     invalid quantum operation;
 *     invalid target;
 *     illegal effect transition.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 43. SOURCE-SPAN CONTRACT
 * ============================================================================
 *
 * Every hybrid construct must preserve source-location information through
 * the frontend.
 *
 * At minimum the AST must be able to identify:
 *
 *     - construct span;
 *     - operation/call span;
 *     - argument spans;
 *     - target spans;
 *     - condition span;
 *     - conversion source span;
 *     - conversion type span;
 *     - requirement span;
 *     - capability span;
 *     - synchronization span.
 *
 * This grammar does not manufacture source-span structures; it preserves the
 * parser context required by the frontend.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 44. COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing valid classical/quantum source constructs MUST remain accepted
 * unless deliberately deprecated by the normative language specification.
 *
 * Compatibility MUST be evaluated against:
 *
 *     grammar/specification/
 *     grammar/spec/
 *     grammar/grammar.md
 *     grammar/Zamani-Grammar.md
 *     grammar/hybrid/
 *     grammar/quantum/
 *     src/lexer.rs
 *     src/parser.rs
 *     src/frontend/ast/
 *
 * No compatibility alias may create a recursive rule cycle.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 45. INTEGRATION WITH grammar/hybrid/hybrid.g4
 * ============================================================================
 *
 * `grammar/hybrid/hybrid.g4` is the hybrid-directory composition grammar.
 *
 * It should expose the public hybrid entry point and delegate the
 * classical/quantum boundary to:
 *
 *     hybridQuantumClassicalConstruct
 *
 * It MUST NOT duplicate the alternatives defined by this file.
 *
 * Conceptually:
 *
 *     Hybrid
 *        |
 *        +--> hybridQuantumClassicalConstruct
 *        +--> accelerator interoperability
 *        +--> hybrid resources
 *        +--> other future hybrid domains
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 46. INTEGRATION WITH grammar/antlr/ZamaniParser.g4
 * ============================================================================
 *
 * The canonical parser root must compose the hybrid dispatcher exactly once.
 *
 * The root MUST NOT import this grammar separately while also importing
 * another grammar that exposes the same effective public classical/quantum
 * implementation.
 *
 * The final composition must have one effective path:
 *
 *     ZamaniParser
 *          |
 *          v
 *        Hybrid
 *          |
 *          v
 *     HybridQuantumClassical
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 47. INTEGRATION WITH grammar/Zamani.g4
 * ============================================================================
 *
 * `grammar/Zamani.g4` remains the universal language composition root.
 *
 * This file does not replace it.
 *
 * It supplies one domain component consumed by the canonical composition
 * architecture.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 48. INTEGRATION WITH EXPRESSIONS
 * ============================================================================
 *
 * The canonical expression grammar remains authoritative.
 *
 * This file consumes:
 *
 *     expression
 *
 * rather than defining:
 *
 *     hybridExpressionPrecedence
 *     quantumExpressionPrecedence
 *     classicalExpressionPrecedence
 *
 * No second precedence hierarchy is permitted.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 49. INTEGRATION WITH TYPES
 * ============================================================================
 *
 * The canonical type grammar remains authoritative.
 *
 * This file consumes:
 *
 *     typeExpression
 *     typeAnnotation
 *
 * where required.
 *
 * It does not define:
 *
 *     HybridType
 *     QuantumClassicalType
 *
 * as a replacement type system.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 50. INTEGRATION WITH QUANTUM OPERATIONS
 * ============================================================================
 *
 * Detailed quantum operation syntax remains owned by:
 *
 *     grammar/quantum/operations.g4
 *
 * This file consumes its canonical operation-reference and target rules.
 *
 * It MUST NOT redefine:
 *
 *     gate lists;
 *     operation inventories;
 *     control modifiers;
 *     adjoint semantics;
 *     inverse semantics;
 *     physical mapping.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 51. INTEGRATION WITH QUANTUM MEASUREMENT
 * ============================================================================
 *
 * Detailed measurement semantics remain owned by the quantum measurement
 * grammar and semantic implementation.
 *
 * This file merely represents the source-level value crossing.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 52. INTEGRATION WITH RESOURCES AND CAPABILITIES
 * ============================================================================
 *
 * This grammar expresses the syntactic boundary only.
 *
 * Resource and capability semantics remain owned by:
 *
 *     grammar/resources/
 *     grammar/hardware/
 *     semantic analysis
 *
 * No physical allocation occurs here.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 53. INTEGRATION WITH FRONTEND AST
 * ============================================================================
 *
 * The frontend AST remains domain-neutral.
 *
 * Hybrid source constructs must map into existing generic AST constructs or
 * explicitly approved hybrid AST nodes.
 *
 * The AST must preserve enough information to distinguish:
 *
 *     classical -> quantum
 *     quantum -> classical
 *     measurement dependency
 *     dynamic control
 *     conversion
 *     synchronization
 *
 * without encoding a particular machine.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 54. INTEGRATION WITH quantum::ir
 * ============================================================================
 *
 * Quantum semantics MUST eventually lower to:
 *
 *     quantum::ir
 *
 * No hybrid grammar-defined IR is permitted.
 *
 * The lowering path is:
 *
 *     HybridQuantumClassical
 *             |
 *             v
 *        frontend AST
 *             |
 *             v
 *       semantic analysis
 *             |
 *             v
 *        quantum::ir
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 55. INTEGRATION WITH OPTIMIZATION / ROUTING / SCHEDULING
 * ============================================================================
 *
 * These systems consume downstream representations.
 *
 * This grammar does not:
 *
 *     optimize;
 *     route;
 *     schedule;
 *     select devices;
 *     select physical qubits;
 *     select network paths;
 *     select memory banks.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 56. INTEGRATION WITH ZQN / QEC / RESILIENCE
 * ============================================================================
 *
 * ZQN, QEC, and resilience remain downstream.
 *
 * This grammar does not encode:
 *
 *     error rates;
 *     code distance;
 *     syndrome layout;
 *     decoder algorithms;
 *     recovery algorithms;
 *     calibration;
 *     noise probabilities;
 *     retry implementation.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 57. INTEGRATION WITH HARDWARE / HAL
 * ============================================================================
 *
 * Hardware realization is downstream.
 *
 * This file MUST remain independent of:
 *
 *     CPU model;
 *     GPU model;
 *     FPGA family;
 *     ASIC implementation;
 *     QPU model;
 *     simulator implementation;
 *     device identifier;
 *     topology;
 *     physical qubit numbering;
 *     register width;
 *     memory capacity.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 58. SCALABILITY CONTRACT
 * ============================================================================
 *
 * The following use structural repetition:
 *
 *     argumentList
 *     quantumParameterArgumentList
 *     quantumOperationTargetClause
 *     hybridQuantumClassicalItem*
 *     synchronization operands
 *
 * No finite language-level cardinality is encoded.
 *
 * Therefore the language can describe computations ranging from very small
 * systems to arbitrarily large systems subject to:
 *
 *     source representation;
 *     implementation resources;
 *     semantic feasibility;
 *     available capabilities;
 *     runtime resources;
 *     target resources.
 *
 * Those limits are not language grammar limits.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 59. SECURITY CONTRACT
 * ============================================================================
 *
 * This grammar must remain free from:
 *
 *     embedded code execution;
 *     file access;
 *     network access;
 *     environment inspection;
 *     hardware inspection;
 *     runtime invocation.
 *
 * Macro/metaprogramming systems remain subject to their own security and
 * capability contracts.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 60. TEST CONTRACT
 * ============================================================================
 *
 * Repository tests must exercise this grammar through the canonical parser
 * composition rather than treating this file as an isolated language.
 *
 * --------------------------------------------------------------------------
 * POSITIVE
 * --------------------------------------------------------------------------
 *
 * The assembled parser should accept representative forms such as:
 *
 *     quantum call circuit(theta);
 *
 *     quantum call namespace::circuit(theta, input);
 *
 *     let result = measure(q);
 *
 *     let result = observe(observable);
 *
 *     let result = read(source);
 *
 *     apply rotation(theta) to q;
 *
 *     apply namespace::operation(theta) to q;
 *
 *     when result apply operation to q;
 *
 *     if result {
 *         apply operation to q;
 *     }
 *
 *     convert(result to ClassicalValue);
 *
 *     synchronize(result);
 *
 *     requires qubits >= n;
 *
 *     capability("quantum.measurement");
 *
 *     hybrid {
 *         let theta = compute();
 *         apply rotation(theta) to q;
 *         let result = measure(q);
 *         process(result);
 *     }
 *
 * --------------------------------------------------------------------------
 * NEGATIVE
 * --------------------------------------------------------------------------
 *
 * The assembled parser must reject malformed structures such as:
 *
 *     quantum call;
 *
 *     quantum call circuit(;
 *
 *     let result = ;
 *
 *     apply;
 *
 *     apply operation( to q;
 *
 *     if {
 *     }
 *
 *     convert(result);
 *
 *     synchronize(;
 *
 *     capability();
 *
 * --------------------------------------------------------------------------
 * BOUNDARY
 * --------------------------------------------------------------------------
 *
 * Tests must cover:
 *
 *     - one argument;
 *     - many arguments;
 *     - one target;
 *     - many targets;
 *     - indexed targets;
 *     - range targets;
 *     - symbolic parameters;
 *     - function-return parameters;
 *     - measurement-derived parameters;
 *     - nested classical expressions;
 *     - nested hybrid regions;
 *     - nested dynamic control;
 *     - multiple measurement results;
 *     - explicit conversions;
 *     - synchronization operands.
 *
 * --------------------------------------------------------------------------
 * SCALABILITY
 * --------------------------------------------------------------------------
 *
 * Tests may use progressively larger source programs.
 *
 * Test sizes MUST NOT become language limits.
 *
 * --------------------------------------------------------------------------
 * DETERMINISM
 * --------------------------------------------------------------------------
 *
 * Identical source and parser configuration must yield identical structural
 * parses.
 *
 * --------------------------------------------------------------------------
 * COMPATIBILITY
 * --------------------------------------------------------------------------
 *
 * Existing valid hybrid programs must remain accepted unless explicitly
 * deprecated by the normative specification.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 61. HARD-CODING AUDIT
 * ============================================================================
 *
 * This grammar contains no:
 *
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *
 * It contains no physical-resource enumeration.
 *
 * It contains no fixed machine topology.
 *
 * It contains no fixed quantum gate inventory.
 *
 * It contains no fixed quantum-device inventory.
 *
 * It contains no hardware capacity ceiling.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 62. DEFINITION OF DONE
 * ============================================================================
 *
 * This file is COMPLETE when:
 *
 *     [x] one public hybrid quantum/classical entry rule exists;
 *     [x] no second expression language exists;
 *     [x] no second type system exists;
 *     [x] no second statement language exists;
 *     [x] no quantum IR exists here;
 *     [x] no classical IR exists here;
 *     [x] no fixed quantum gate enumeration exists;
 *     [x] no hardware capacity exists;
 *     [x] no physical allocation exists;
 *     [x] no routing exists;
 *     [x] no scheduling exists;
 *     [x] no QEC exists;
 *     [x] no ZQN implementation exists;
 *     [x] no runtime implementation exists;
 *     [x] no embedded Rust exists;
 *     [x] safe Rust remains sufficient downstream;
 *     [x] POCO-REAF remains intact;
 *     [x] operation names remain open-ended;
 *     [x] quantum results can cross into classical computation;
 *     [x] classical values can cross into quantum computation;
 *     [x] classical control can drive quantum execution;
 *     [x] hybrid regions are structurally expressible;
 *     [x] explicit conversions are expressible;
 *     [x] synchronization intent is expressible;
 *     [x] resource requirements are expressible;
 *     [x] capabilities are expressible;
 *     [x] future computational domains remain possible.
 *
 * Repository-level completion additionally requires:
 *
 *     [ ] grammar/hybrid/hybrid.g4 exposes this entry rule exactly once;
 *     [ ] grammar/hybrid/classical-quantum.g4 no longer competes with this
 *         grammar;
 *     [ ] grammar/quantum/quantum-classical.g4 does not provide a competing
 *         hybrid public dispatcher;
 *     [ ] grammar/antlr/ZamaniParser.g4 imports the hybrid composition once;
 *     [ ] grammar/Zamani.g4 remains the universal composition root;
 *     [ ] canonical lexer tokens exist for every token consumed here;
 *     [ ] canonical expression rules resolve;
 *     [ ] canonical type rules resolve;
 *     [ ] canonical statement/block rules resolve;
 *     [ ] canonical quantum operation rules resolve;
 *     [ ] ANTLR generation succeeds without duplicate rules;
 *     [ ] ANTLR generation succeeds without import cycles;
 *     [ ] Rust lexer/parser conformance succeeds;
 *     [ ] frontend AST mapping succeeds;
 *     [ ] semantic analysis succeeds;
 *     [ ] quantum::ir lowering succeeds;
 *     [ ] positive tests succeed;
 *     [ ] negative tests succeed;
 *     [ ] boundary tests succeed;
 *     [ ] scalability tests succeed;
 *     [ ] determinism tests succeed;
 *     [ ] compatibility tests succeed.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL RULE
 * ============================================================================
 *
 * This file is deliberately a BOUNDARY grammar.
 *
 * It does not attempt to become a quantum language inside the hybrid
 * language.
 *
 * It does not attempt to become a classical language inside the hybrid
 * language.
 *
 * It connects the two through the existing universal Zamani syntax.
 *
 * The resulting architecture is:
 *
 *     classical computation
 *             |
 *             | values / control / calls
 *             v
 *     HybridQuantumClassical
 *             ^
 *             | results / measurements
 *             |
 *      quantum computation
 *
 * followed by:
 *
 *     domain-neutral AST
 *             |
 *             v
 *       semantic analysis
 *             |
 *             +-------------------+
 *             |                   |
 *             v                   v
 *       classical semantics   quantum::ir
 *             |                   |
 *             +---------+---------+
 *                       |
 *                       v
 *                  optimization
 *                       |
 *                  routing
 *                       |
 *                 scheduling
 *                       |
 *                    ZQN/QEC
 *                       |
 *                   resilience
 *                       |
 *                   hardware HAL
 *                       |
 *                  target runtime
 *
 * This preserves:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * because the source describes computation and semantic requirements rather
 * than today's machine.
 *
 * ============================================================================
 */
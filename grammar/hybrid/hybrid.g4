/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/hybrid/hybrid.g4
 *
 * Status:
 *     Production hybrid-domain composition grammar.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the canonical HYBRID DOMAIN grammar for Zamani.
 *
 * Hybrid computation is one Zamani language construct in which semantic
 * computation crosses or combines computational domains, including:
 *
 *     classical
 *     quantum
 *     accelerator
 *     hardware
 *     distributed
 *     AI/data
 *     future computational domains
 *
 * This grammar is intentionally a DOMAIN COMPOSITION GRAMMAR.
 *
 * It owns the source-level structure that identifies a computation as
 * hybrid and preserves the relationships between participating domains.
 *
 * It does NOT own:
 *
 *     lexical definitions
 *     general expression precedence
 *     general type syntax
 *     general statement syntax
 *     function syntax
 *     module syntax
 *     quantum gate inventories
 *     quantum IR
 *     classical IR
 *     hardware realization
 *     resource discovery
 *     device selection
 *     routing
 *     scheduling
 *     optimization
 *     calibration
 *     QEC
 *     ZQN
 *     resilience
 *     runtime execution
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     ZamaniParser
 *          |
 *          v
 *     Hybrid
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +-----------------------+
 *          |                       |
 *          v                       v
 *     classical semantics     quantum semantics
 *                                  |
 *                                  v
 *                              quantum::ir
 *          |                       |
 *          +-----------+-----------+
 *                      |
 *                      v
 *               canonical semantic
 *                   representation
 *                      |
 *          +-----------+-----------+
 *          |           |           |
 *          v           v           v
 *      optimize     route       schedule
 *                      |
 *                      v
 *               QEC / ZQN /
 *                resilience
 *                      |
 *                      v
 *                     HAL
 *                      |
 *                      v
 *               target realization
 *
 * ============================================================================
 * CANONICAL COMPOSITION CONTRACT
 * ============================================================================
 *
 * grammar/antlr/ZamaniParser.g4 is the canonical parser composition root.
 *
 * It imports:
 *
 *     Hybrid
 *
 * together with the other domain dispatchers.
 *
 * Therefore this file MUST NOT import ZamaniParser.
 *
 * The canonical lexical vocabulary is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * This file is a parser grammar and MUST NOT define lexer rules.
 *
 * ============================================================================
 * IMPORTANT INTEGRATION DECISION
 * ============================================================================
 *
 * The repository currently contains several older/specialized hybrid grammar
 * files:
 *
 *     grammar/hybrid/classical-quantum.g4
 *     grammar/hybrid/quantum-classical-control.g4
 *     grammar/hybrid/accelerator-interoperability.g4
 *     grammar/hybrid/hybrid-resources.g4
 *
 * They are retained as domain-specific implementation contracts/reference
 * surfaces, but they are NOT blindly imported here.
 *
 * Reason:
 *
 *     - classical-quantum.g4 currently uses obsolete K_* token names;
 *     - accelerator-interoperability.g4 owns generic rules such as
 *       qualifiedName/expressionList;
 *     - hybrid-resources.g4 also owns generic names such as identifier and
 *       qualifiedName;
 *     - directly importing those grammars together would create rule/token
 *       collisions;
 *     - the old hybrid.g4 contains recursive adapter rules.
 *
 * Therefore this file is the stable canonical composition boundary.
 *
 * Those specialized files may subsequently be normalized behind this public
 * interface without changing the public rules defined here.
 *
 * This avoids forcing a second hybrid grammar architecture onto Zamani.
 *
 * ============================================================================
 * SINGLE-OWNER RULE
 * ============================================================================
 *
 * General language constructs remain owned by their existing domains:
 *
 *     expressions/
 *         expression
 *
 *     statements/
 *         statement / block
 *
 *     types/
 *         typeExpression
 *
 *     functions/
 *         functionDeclaration
 *
 *     modules/
 *         moduleDeclaration
 *
 *     quantum/
 *         quantum source semantics
 *
 *     classical/
 *         classical source semantics
 *
 *     hardware/
 *         target-independent hardware intent
 *
 *     resources/
 *         resource requirements/capabilities
 *
 * Hybrid owns only their composition.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Hybrid source syntax describes semantic computation and relationships.
 *
 * It MUST NOT encode universal physical limits.
 *
 * In particular, this grammar contains no:
 *
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_REGISTER_WIDTH
 *     MAX_TENSOR_RANK
 *     MAX_NETWORK_SIZE
 *
 * It also does not encode:
 *
 *     physical CPU IDs
 *     physical GPU IDs
 *     physical FPGA IDs
 *     physical QPU IDs
 *     physical qubit IDs
 *     fixed coupling maps
 *     physical memory addresses
 *     fixed topology
 *     backend-specific pulse schedules
 *
 * A program may express semantic requirements such as:
 *
 *     requires capability("quantum.measurement");
 *     requires qubits >= n;
 *     requires memory >= required_memory;
 *
 * but satisfaction of those requirements belongs to semantic analysis,
 * resource management, compilation, deployment, HAL, and runtime.
 *
 * ============================================================================
 * PROGRAM-ONCE / COMPILE-ONCE / RUN-EVERYWHERE
 * ============================================================================
 *
 * Hybrid syntax separates:
 *
 *     semantic intent
 *
 * from:
 *
 *     target realization.
 *
 * Therefore the source can express:
 *
 *     classical computation
 *          ->
 *     quantum computation
 *          ->
 *     measurement
 *          ->
 *     classical decision
 *          ->
 *     quantum computation
 *
 * without selecting:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     simulator
 *     cluster node
 *     cloud provider
 *
 * Target realization remains downstream.
 *
 * ============================================================================
 * STABLE PUBLIC RULES
 * ============================================================================
 *
 * The canonical parser should depend only on:
 *
 *     hybridDeclaration
 *     hybridStatement
 *     hybridExpression
 *     hybridConstruct
 *
 * Internal rules may evolve as long as these public boundaries remain
 * semantically compatible.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 1. PUBLIC HYBRID CONSTRUCT
 * ============================================================================
 *
 * A hybrid construct is explicitly marked by source structure that crosses
 * computational domains.
 *
 * The construct can represent:
 *
 *     - a hybrid region;
 *     - a domain invocation;
 *     - a cross-domain binding;
 *     - a quantum/classical boundary;
 *     - an accelerator boundary;
 *     - a resource/capability declaration;
 *     - a hybrid control construct;
 *
 * Domain-specific meaning is resolved semantically.
 */
hybridConstruct
    : hybridRegion
    | hybridInvocation
    | hybridBinding
    | hybridControl
    | hybridConversion
    | hybridSynchronization
    | hybridRequirement
    | hybridCapability
    | hybridPreference
    | hybridConstraint
    | hybridHint
    ;


/*
 * ============================================================================
 * 2. HYBRID REGION
 * ============================================================================
 *
 * A hybrid region is a lexical grouping in which multiple domain constructs
 * may coexist.
 *
 * This is the corrected replacement for the old recursive:
 *
 *     hybridRegion
 *         : hybridRegionOwned
 *         ;
 *
 *     hybridRegionOwned
 *         : hybridRegion
 *         ;
 *
 * which could never terminate.
 *
 * A hybrid region is simply:
 *
 *     hybrid { ... }
 *
 * with an unbounded sequence of hybrid-region items.
 *
 * The block does not imply:
 *
 *     thread
 *     process
 *     device
 *     queue
 *     scheduling region
 *     hardware controller
 */
hybridRegion
    : HYBRID hybridBlock
    ;


hybridBlock
    : LBRACE hybridRegionItem* RBRACE
    ;


hybridRegionItem
    : hybridConstruct
    | statement
    ;


/*
 * ============================================================================
 * 3. HYBRID DECLARATION
 * ============================================================================
 *
 * A declaration that explicitly establishes hybrid semantic scope.
 *
 * Ordinary declaration syntax remains owned by declarations/.
 */
hybridDeclaration
    : hybridRegion
    ;


/*
 * ============================================================================
 * 4. HYBRID STATEMENT
 * ============================================================================
 *
 * The hybrid statement is a stable adapter into the ordinary statement
 * position.
 *
 * No second statement grammar is created.
 */
hybridStatement
    : hybridConstruct
    ;


/*
 * ============================================================================
 * 5. HYBRID EXPRESSION
 * ============================================================================
 *
 * Hybrid expressions use the canonical expression language.
 *
 * No second precedence hierarchy is introduced here.
 *
 * The semantic layer determines whether the expression:
 *
 *     - is classical;
 *     - produces a quantum value;
 *     - consumes a measurement result;
 *     - invokes an accelerator;
 *     - crosses a domain boundary.
 */
hybridExpression
    : hybridValueExpression
    ;


hybridValueExpression
    : expression
    | hybridInvocationExpression
    | hybridConversionExpression
    ;


/*
 * ============================================================================
 * 6. HYBRID INVOCATION
 * ============================================================================
 *
 * Generic domain invocation.
 *
 * Operation names remain extensible identifiers.
 *
 * This deliberately does NOT enumerate:
 *
 *     H
 *     X
 *     Y
 *     Z
 *     CNOT
 *     CUDA operations
 *     vendor accelerator operations
 *     FPGA primitives
 *
 * Those are semantic operations, libraries, dialects, or target-level
 * capabilities.
 *
 * Examples of the intended semantic surface:
 *
 *     quantum::algorithm(...)
 *     classical::function(...)
 *     accelerator::operation(...)
 *
 * Qualified names remain owned by the canonical name/path grammar.
 */
hybridInvocation
    : hybridDomainQualifier
      qualifiedName
      LPAREN
      argumentList?
      RPAREN
      hybridInvocationTargetClause?
      SEMICOLON
    ;


hybridInvocationExpression
    : hybridDomainQualifier
      qualifiedName
      LPAREN
      argumentList?
      RPAREN
    ;


hybridDomainQualifier
    : hybridDomainName
      DOUBLE_COLON
    ;


hybridDomainName
    : identifier
    ;


hybridInvocationTargetClause
    : TO hybridTarget
    ;


/*
 * ============================================================================
 * 7. HYBRID TARGET
 * ============================================================================
 *
 * This is a semantic target/classification expression.
 *
 * It is NOT a physical device identifier.
 */
hybridTarget
    : expression
    ;


/*
 * ============================================================================
 * 8. HYBRID BINDING
 * ============================================================================
 *
 * Values can cross domains through ordinary source-level bindings.
 *
 * The type checker determines whether the crossing is legal.
 */
hybridBinding
    : LET IDENTIFIER typeAnnotation? ASSIGN hybridBindingValue SEMICOLON
    | IDENTIFIER ASSIGN hybridBindingValue SEMICOLON
    ;


hybridBindingValue
    : expression
    | measurementValue
    | hybridInvocationExpression
    | hybridConversionExpression
    ;


/*
 * ============================================================================
 * 9. MEASUREMENT VALUE
 * ============================================================================
 *
 * Measurement is represented as a value-producing boundary.
 *
 * The quantum grammar remains responsible for the underlying quantum
 * measurement semantics.
 *
 * The result becomes available to the classical semantic layer only after
 * semantic validation.
 */
measurementValue
    : MEASURE
      LPAREN
      expression
      RPAREN
    ;


/*
 * ============================================================================
 * 10. HYBRID CONTROL
 * ============================================================================
 *
 * Classical control may depend on a quantum result.
 *
 * The syntax intentionally reuses canonical IF/ELSE/block constructs.
 *
 * Example:
 *
 *     if measure(q) {
 *         ...
 *     }
 *
 * The grammar does not decide whether the condition is:
 *
 *     compile-time
 *     runtime
 *     dynamic-circuit control
 *     host-side control
 *     device-side control
 *
 * Semantic analysis determines that.
 */
hybridControl
    : IF expression block
    | IF expression block ELSE block
    | WHEN expression block
    ;


/*
 * ============================================================================
 * 11. EXPLICIT DOMAIN CONVERSION
 * ============================================================================
 *
 * Conversion describes semantic intent.
 *
 * It does not define the implementation mechanism.
 *
 * Examples:
 *
 *     convert(value) to quantum;
 *     convert(value) to classical;
 *
 * The semantic layer determines whether such conversion is meaningful.
 */
hybridConversion
    : CONVERT
      LPAREN
      expression
      RPAREN
      TO
      hybridDomain
      SEMICOLON
    ;


hybridConversionExpression
    : CONVERT
      LPAREN
      expression
      RPAREN
      TO
      hybridDomain
    ;


hybridDomain
    : hybridDomainName
    ;


/*
 * ============================================================================
 * 12. HYBRID SYNCHRONIZATION
 * ============================================================================
 *
 * Synchronization is semantic synchronization.
 *
 * It does not encode:
 *
 *     clock frequency
 *     pulse duration
 *     bus latency
 *     queue latency
 *     device synchronization primitives
 *
 * Those belong downstream.
 */
hybridSynchronization
    : SYNCHRONIZE
      hybridSynchronizationScope?
      SEMICOLON
    ;


hybridSynchronizationScope
    : LPAREN expression RPAREN
    ;


/*
 * ============================================================================
 * 13. HYBRID REQUIREMENT
 * ============================================================================
 *
 * Requirements express semantic/resource intent.
 *
 * Examples:
 *
 *     requires quantum;
 *     requires capability("quantum.measurement");
 *     requires qubits >= n;
 *
 * The complete requirement expression remains an ordinary Zamani expression.
 *
 * Resource interpretation is downstream.
 */
hybridRequirement
    : REQUIRES expression SEMICOLON
    ;


/*
 * ============================================================================
 * 14. HYBRID CAPABILITY
 * ============================================================================
 *
 * Capability declaration describes semantic availability or required
 * capability information.
 *
 * It does not probe hardware.
 */
hybridCapability
    : CAPABILITY qualifiedName
      hybridCapabilityBody?
      SEMICOLON
    ;


hybridCapabilityBody
    : ASSIGN expression
    ;


/*
 * ============================================================================
 * 15. HYBRID PREFERENCE
 * ============================================================================
 *
 * A preference is advisory rather than a mandatory resource requirement.
 *
 * It must not become a hidden hardware selection.
 */
hybridPreference
    : PREFER expression SEMICOLON
    ;


/*
 * ============================================================================
 * 16. HYBRID CONSTRAINT
 * ============================================================================
 *
 * A constraint expresses a semantic restriction.
 *
 * It is not a physical placement instruction.
 */
hybridConstraint
    : CONSTRAINT expression SEMICOLON
    ;


/*
 * ============================================================================
 * 17. HYBRID HINT
 * ============================================================================
 *
 * A hint is advisory implementation information.
 *
 * Hints may influence downstream optimization but must not alter program
 * semantics.
 */
hybridHint
    : HINT expression SEMICOLON
    ;


/*
 * ============================================================================
 * 18. HYBRID DOMAIN PAIR
 * ============================================================================
 *
 * This rule is useful to semantic tooling that needs to identify an explicit
 * cross-domain relationship.
 *
 * The domains themselves remain open-ended identifiers.
 */
hybridDomainPair
    : hybridDomainName
      DOUBLE_COLON
      hybridDomainName
    ;


/*
 * ============================================================================
 * 19. CLASSICAL -> QUANTUM BOUNDARY
 * ============================================================================
 *
 * A classical expression can be supplied as an argument to a quantum
 * invocation through the ordinary invocation surface.
 *
 * No special fixed classical-to-quantum value list exists.
 *
 * Semantic analysis determines:
 *
 *     type compatibility
 *     ownership
 *     lifetime
 *     evaluation timing
 *     representability
 *     measurement dependency
 *     capability requirements
 */
classicalToQuantum
    : hybridInvocationExpression
    ;


/*
 * ============================================================================
 * 20. QUANTUM -> CLASSICAL BOUNDARY
 * ============================================================================
 *
 * Measurement or quantum-call results may be bound to classical names.
 */
quantumToClassical
    : measurementValue
    | hybridInvocationExpression
    ;


/*
 * ============================================================================
 * 21. HYBRID VALUE
 * ============================================================================
 */
hybridValue
    : classicalToQuantum
    | quantumToClassical
    | expression
    ;


/*
 * ============================================================================
 * 22. HYBRID SEQUENCE
 * ============================================================================
 *
 * Arbitrarily long sequence.
 *
 * This is intentionally:
 *
 *     hybridConstruct*
 *
 * rather than a fixed number of alternatives.
 */
hybridConstructSequence
    : hybridConstruct*
    ;


hybridConstructSequenceNonEmpty
    : hybridConstruct+
    ;


/*
 * ============================================================================
 * 23. HYBRID GROUP
 * ============================================================================
 *
 * Structural grouping only.
 */
hybridGroup
    : LBRACE
      hybridConstructSequence
      RBRACE
    ;


/*
 * ============================================================================
 * 24. HYBRID ITEM
 * ============================================================================
 *
 * Publicly useful generic hybrid item boundary.
 */
hybridItem
    : hybridConstruct
    | statement
    ;


/*
 * ============================================================================
 * 25. HYBRID SOURCE ELEMENT
 * ============================================================================
 */
hybridSourceElement
    : hybridConstruct
    ;


/*
 * ============================================================================
 * 26. HYBRID DOMAIN BOUNDARY
 * ============================================================================
 *
 * This is a semantic marker for tooling/AST construction.
 */
hybridDomainBoundary
    : hybridDomainPair
    ;


/*
 * ============================================================================
 * 27. HYBRID ARGUMENT
 * ============================================================================
 *
 * Arguments use the canonical expression grammar.
 */
hybridArgument
    : expression
    ;


hybridArgumentList
    : hybridArgument
      (COMMA hybridArgument)*
    ;


/*
 * ============================================================================
 * 28. HYBRID RETURN VALUE
 * ============================================================================
 */
hybridReturnValue
    : expression
    ;


/*
 * ============================================================================
 * 29. DOMAIN-QUALIFIED CALLS
 * ============================================================================
 *
 * These rules provide explicit semantic names for tooling.
 *
 * They are aliases over the same generic invocation model.
 *
 * No second invocation implementation is created.
 */
domainQualifiedQuantumCall
    : QUANTUM
      DOUBLE_COLON
      qualifiedName
      LPAREN
      argumentList?
      RPAREN
    ;


domainQualifiedClassicalCall
    : identifier
      DOUBLE_COLON
      qualifiedName
      LPAREN
      argumentList?
      RPAREN
    ;


hybridCall
    : hybridInvocationExpression
    ;


/*
 * ============================================================================
 * 30. HYBRID REQUIREMENT VALUE
 * ============================================================================
 *
 * The value after a requirement remains an ordinary expression.
 */
hybridRequirementWithValue
    : REQUIRES expression ASSIGN expression SEMICOLON
    ;


/*
 * ============================================================================
 * 31. DOMAIN ANNOTATION
 * ============================================================================
 *
 * This does not create a new type system.
 */
hybridDomainAnnotation
    : AT hybridDomainName
    ;


/*
 * ============================================================================
 * 32. HYBRID DECLARATION WITH DOMAIN
 * ============================================================================
 */
hybridDeclarationWithDomain
    : hybridDomainName
      qualifiedName
      SEMICOLON
    ;


/*
 * ============================================================================
 * 33. HYBRID ASSIGNMENT
 * ============================================================================
 */
hybridAssignment
    : IDENTIFIER
      ASSIGN
      hybridValue
      SEMICOLON
    ;


/*
 * ============================================================================
 * 34. HYBRID CONDITION
 * ============================================================================
 */
hybridCondition
    : expression
    ;


/*
 * ============================================================================
 * 35. HYBRID TARGET EXPRESSION
 * ============================================================================
 */
hybridTargetExpression
    : expression
    ;


/*
 * ============================================================================
 * 36. INTEGRATION CONTRACT
 * ============================================================================
 *
 * The following existing grammar domains are consumed by this file through
 * their canonical public rules:
 *
 *     grammar/expressions/
 *         expression
 *         argumentList
 *
 *     grammar/statements/
 *         statement
 *         block
 *
 *     grammar/types/
 *         typeAnnotation
 *
 *     grammar/core/
 *         identifier
 *         qualifiedName
 *
 *     grammar/quantum/
 *         quantum semantic constructs
 *
 *     grammar/classical/
 *         classical semantic constructs
 *
 *     grammar/resources/
 *         resource/capability semantics
 *
 *     grammar/hardware/
 *         target-independent hardware semantics
 *
 * This file deliberately does not duplicate those grammars.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Every hybrid construct must map to the existing domain-neutral frontend AST.
 *
 * The AST must preserve:
 *
 *     source span
 *     source ordering
 *     nesting
 *     domain boundary
 *     participating domains
 *     input values
 *     output values
 *     control dependencies
 *     requirements
 *     capabilities
 *     constraints
 *     preferences
 *     hints
 *
 * Parsing MUST NOT manufacture:
 *
 *     PhysicalQubitId
 *     DeviceId
 *     BackendId
 *     ScheduleSlot
 *     HardwareAddress
 *     PhysicalGpuId
 *     PhysicalCpuId
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for:
 *
 *     - determining domain membership;
 *     - validating domain crossings;
 *     - checking type compatibility;
 *     - validating measurement dependencies;
 *     - checking effect compatibility;
 *     - validating resource requirements;
 *     - evaluating capability requirements;
 *     - determining whether preferences are satisfiable;
 *     - validating constraints;
 *     - checking ownership/lifetime;
 *     - checking deterministic semantics;
 *     - determining host/device/accelerator realization;
 *
 * None of these decisions are performed by this grammar.
 *
 * ============================================================================
 * QUANTUM IR CONTRACT
 * ============================================================================
 *
 * Quantum constructs originating in hybrid source ultimately lower through:
 *
 *     quantum::ir
 *
 * This file does NOT define:
 *
 *     QuantumGate
 *     QuantumInstruction
 *     QuantumCircuit
 *     QuantumRegister
 *     QubitId
 *     PhysicalQubitId
 *
 * A hybrid construct therefore cannot create a competing quantum IR.
 *
 * ============================================================================
 * CLASSICAL IR CONTRACT
 * ============================================================================
 *
 * Classical computation uses the repository's canonical classical semantic
 * representation.
 *
 * This grammar does not introduce a second classical IR.
 *
 * ============================================================================
 * HDL / HARDWARE CONTRACT
 * ============================================================================
 *
 * Hybrid source may interact semantically with HDL/hardware computation.
 *
 * Physical hardware realization remains owned downstream by:
 *
 *     hardware
 *     compile
 *     execution
 *     HAL
 *
 * This grammar does not select:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     simulator
 *     node
 *     cluster
 *     provider
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Requirements, capabilities, preferences, constraints and hints are distinct
 * semantic categories.
 *
 *     requirement
 *         mandatory semantic/resource need
 *
 *     capability
 *         property available or required by an execution context
 *
 *     preference
 *         advisory preference
 *
 *     constraint
 *         semantic restriction
 *
 *     hint
 *         non-authoritative implementation guidance
 *
 * This grammar never converts a capability declaration into a physical
 * allocation.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * All collections use ANTLR repetition:
 *
 *     *
 *     +
 *     ?
 *
 * No finite source-language capacity is encoded.
 *
 * The language therefore remains scalable from:
 *
 *     one classical value
 *     one qubit
 *     one accelerator
 *     one hybrid operation
 *
 * through arbitrarily large programs, subject only to actual:
 *
 *     parser resources
 *     compiler resources
 *     runtime resources
 *     declared requirements
 *     available hardware
 *
 * This grammar does not impose an artificial ceiling.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing is deterministic for a fixed:
 *
 *     source
 *     token stream
 *     grammar version
 *
 * It must not depend on:
 *
 *     hardware
 *     network
 *     filesystem
 *     clock
 *     randomness
 *     device discovery
 *     runtime state
 *
 * ============================================================================
 * SAFETY
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no Rust actions;
 *     no Rust predicates;
 *     no filesystem access;
 *     no network access;
 *     no process execution;
 *     no hardware probing;
 *     no unsafe Rust;
 *     no target-specific implementation.
 *
 * Generated Rust remains subject to the repository's:
 *
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *
 * requirement.
 *
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * The grammar must preserve ordinary ANTLR source locations.
 *
 * Semantic diagnostics downstream should distinguish:
 *
 *     HYBRID_DOMAIN_MISMATCH
 *     HYBRID_TYPE_MISMATCH
 *     HYBRID_INVALID_CONVERSION
 *     HYBRID_INVALID_CONTROL_DEPENDENCY
 *     HYBRID_UNAVAILABLE_CAPABILITY
 *     HYBRID_UNSATISFIED_REQUIREMENT
 *     HYBRID_INVALID_CONSTRAINT
 *     HYBRID_INVALID_PREFERENCE
 *
 * These are semantic diagnostic categories, not parser actions.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no universal:
 *
 *     qubit limit
 *     CPU limit
 *     GPU limit
 *     FPGA limit
 *     node limit
 *     thread limit
 *     memory limit
 *     tensor-rank limit
 *     register-width limit
 *     device-count limit
 *     topology limit
 *
 * No physical device identifier is required by the hybrid syntax.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Stable public rules:
 *
 *     hybridConstruct
 *     hybridDeclaration
 *     hybridStatement
 *     hybridExpression
 *
 * Internal rules may be extended provided their semantic meaning remains
 * compatible.
 *
 * New computational domains should be represented by extensible domain names
 * and semantic capabilities rather than by modifying the core grammar for
 * every new device or vendor.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * grammar/tests/hybrid/ must include at minimum:
 *
 * Positive:
 *
 *     classical -> quantum
 *     quantum -> classical measurement
 *     classical decision -> quantum
 *     nested hybrid regions
 *     parameterized quantum invocation
 *     accelerator invocation
 *     resource requirement
 *     capability requirement
 *     preference
 *     constraint
 *     hint
 *     synchronization
 *     conversion
 *
 * Negative:
 *
 *     malformed hybrid region
 *     missing closing brace
 *     missing invocation delimiter
 *     malformed conversion
 *     malformed requirement
 *     malformed capability
 *
 * Boundary:
 *
 *     one operation
 *     many operations
 *     empty hybrid region
 *     deeply nested hybrid regions
 *     large argument lists
 *     large requirement expressions
 *
 * Scalability:
 *
 *     no test may establish a fixed maximum number of:
 *         domains
 *         operations
 *         values
 *         resources
 *         qubits
 *         accelerators
 *         nodes
 *
 * Integration:
 *
 *     hybrid source must parse through:
 *
 *         ZamaniLexer
 *             ->
 *         ZamaniParser
 *             ->
 *         domain-neutral AST
 *             ->
 *         semantic analysis
 *             ->
 *         canonical IR
 *
 * Quantum paths must reach:
 *
 *     quantum::ir
 *
 * without introducing another hybrid quantum IR.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 * [ ] No recursive adapter rule can loop without consuming input.
 * [ ] Canonical lexer token names are used.
 * [ ] No K_* compatibility vocabulary remains.
 * [ ] No lexer rules are defined here.
 * [ ] No second expression grammar is defined.
 * [ ] No second type system is defined.
 * [ ] No second statement grammar is defined.
 * [ ] No fixed quantum gate inventory exists.
 * [ ] No fixed machine/resource limits exist.
 * [ ] Hybrid regions accept arbitrary-length sequences.
 * [ ] Classical -> quantum flow is representable.
 * [ ] Quantum -> classical flow is representable.
 * [ ] Measurement -> classical control is representable.
 * [ ] Classical parameters -> quantum operations are representable.
 * [ ] Quantum results -> classical bindings are representable.
 * [ ] Accelerator interaction is representable through generic domain
 *     invocation.
 * [ ] Resource requirements remain target-independent.
 * [ ] Capability declarations remain declarative.
 * [ ] Preferences remain advisory.
 * [ ] Constraints remain semantic.
 * [ ] Hints remain non-authoritative.
 * [ ] AST integration is domain-neutral.
 * [ ] quantum::ir remains canonical.
 * [ ] No second hybrid IR exists.
 * [ ] QEC remains downstream.
 * [ ] ZQN remains downstream.
 * [ ] routing remains downstream.
 * [ ] scheduling remains downstream.
 * [ ] HAL remains downstream.
 * [ ] No unsafe Rust is required.
 * [ ] Rust 1.97 / 1.97.1 generation/conformance succeeds.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * This grammar defines HOW classical, quantum, accelerator and other
 * computational semantics may be composed in one Zamani source program.
 *
 * It does not define WHERE that program ultimately executes.
 *
 * Therefore:
 *
 *     Program Once
 *          ->
 *     Compile Once
 *          ->
 *     Run Everywhere
 *          ->
 *     Run Anywhere
 *          ->
 *     Run Forever
 *
 * means that the PROGRAM'S SEMANTIC INTENT remains portable and extensible,
 * while resource realization is determined by the available compilation and
 * execution environment.
 *
 * ============================================================================
 */
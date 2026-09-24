/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/quantum/circuits.g4
 *
 * Status:
 *     Production-ready quantum-circuit grammar fragment.
 *
 * Purpose:
 *     Own the source-level structure of reusable quantum circuits without
 *     duplicating the general expression, statement, type, operation,
 *     resource, or quantum IR systems.
 *
 * Implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *     no unsafe Rust
 *
 * ============================================================================
 * ARCHITECTURAL AUTHORITY
 * ============================================================================
 *
 * Normative source-language authority:
 *
 *     grammar/spec/syntax.md
 *     grammar/spec/quantum.md
 *     grammar/DESIGN.md
 *
 * Canonical lexical authority:
 *
 *     grammar/lexer/tokens.g4
 *     grammar/lexer/keywords.g4
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Canonical parser composition:
 *
 *     grammar/Zamani.g4
 *
 * Canonical frontend representation:
 *
 *     src/frontend/ast/
 *
 * Canonical quantum semantic boundary:
 *
 *     src/quantum/ir/
 *
 * This file MUST NOT create another quantum IR.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - circuit declaration structure;
 *   - canonical `circuit` declaration form;
 *   - integration of the `quantum circuit` form through quantum.g4;
 *   - circuit identifier;
 *   - circuit generic parameter attachment;
 *   - circuit value/runtime parameter attachment;
 *   - circuit inheritance/extension syntax;
 *   - circuit where-clause attachment;
 *   - circuit body boundary;
 *   - circuit declaration/reference abstraction;
 *   - circuit specialization reference abstraction;
 *   - circuit-level structural metadata attachment;
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - identifiers;
 *   - qualified names;
 *   - generic parameter definitions;
 *   - generic argument definitions;
 *   - general parameters;
 *   - general arguments;
 *   - general expressions;
 *   - general statements;
 *   - blocks outside circuit declaration ownership;
 *   - qubit declarations;
 *   - quantum register declarations;
 *   - quantum types;
 *   - quantum operations;
 *   - gate definitions;
 *   - measurements;
 *   - reset;
 *   - barriers;
 *   - observables;
 *   - dynamic control;
 *   - classical control;
 *   - quantum/classical feed-forward;
 *   - resource discovery;
 *   - capability discovery;
 *   - hardware selection;
 *   - physical qubit mapping;
 *   - routing;
 *   - scheduling;
 *   - optimization;
 *   - QEC implementation;
 *   - ZQN implementation;
 *   - resilience implementation;
 *   - calibration;
 *   - backend selection;
 *   - runtime execution;
 *   - serialization;
 *   - quantum::ir implementation.
 *
 * ============================================================================
 * CRITICAL DESIGN RULE
 * ============================================================================
 *
 * A circuit is a reusable semantic computation.
 *
 * A circuit is NOT a hardware allocation.
 *
 * Therefore this grammar must never require or encode:
 *
 *     physical qubit IDs
 *     physical register IDs
 *     QPU IDs
 *     device IDs
 *     vendor IDs
 *     coupling maps
 *     topology sizes
 *     gate durations
 *     pulse durations
 *     processor counts
 *     core counts
 *     GPU counts
 *     FPGA resource counts
 *     memory capacities
 *     fixed circuit depth
 *     fixed operation counts
 *     fixed target counts
 *     fixed parameter counts
 *     fixed register counts
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Circuit source expresses:
 *
 *     computation
 *     interfaces
 *     reusable structure
 *     genericity
 *     semantic constraints
 *     source-level composition
 *
 * It does NOT express:
 *
 *     how many physical resources a target owns;
 *     where those resources are located;
 *     how logical resources are routed;
 *     how operations are scheduled;
 *     which native gate set is selected;
 *     which physical qubits are assigned;
 *     which backend is selected.
 *
 * Those decisions occur downstream.
 *
 * Therefore the same circuit source may be compiled toward:
 *
 *     embedded systems
 *     CPUs
 *     multicore CPUs
 *     GPUs
 *     FPGAs
 *     ASICs
 *     QPUs
 *     quantum simulators
 *     accelerators
 *     clusters
 *     HPC systems
 *     distributed systems
 *     cloud systems
 *     future computational targets
 *
 * subject to semantic validity and available resources/capabilities.
 *
 * ============================================================================
 * CANONICAL SOURCE FORMS
 * ============================================================================
 *
 * Canonical quantum declaration:
 *
 *     quantum circuit Bell(a: Qubit, b: Qubit) {
 *         ...
 *     }
 *
 * Compatibility form:
 *
 *     circuit Bell(a: Qubit, b: Qubit) {
 *         ...
 *     }
 *
 * The canonical `quantum` prefix is owned by:
 *
 *     grammar/quantum/quantum.g4
 *
 * This file owns the `circuit ...` portion.
 *
 * This prevents:
 *
 *     quantum quantum circuit ...
 *
 * and keeps the prefix ownership unambiguous.
 *
 * ============================================================================
 * CIRCUIT INVOCATION
 * ============================================================================
 *
 * Circuit invocation intentionally uses the existing general call-expression
 * mechanism.
 *
 * Example:
 *
 *     Bell(a, b);
 *
 * or, where generic arguments are required:
 *
 *     QFT::<N>(register);
 *
 * Semantic analysis determines whether the resolved callable is:
 *
 *     a circuit;
 *     a function;
 *     an operation;
 *     a constructor;
 *     another callable semantic entity.
 *
 * circuits.g4 MUST NOT introduce a competing:
 *
 *     call ...
 *
 * statement language.
 *
 * This avoids requiring a new keyword and avoids ambiguity with the canonical
 * expression grammar.
 *
 * ============================================================================
 * OPERATION INTEGRATION
 * ============================================================================
 *
 * Quantum operation syntax remains owned by:
 *
 *     grammar/quantum/operations.g4
 *
 * For example:
 *
 *     apply H to q;
 *     apply custom.operation(theta) to q;
 *
 * circuits.g4 does not enumerate:
 *
 *     H
 *     X
 *     Y
 *     Z
 *     CNOT
 *     SWAP
 *     RX
 *     RY
 *     RZ
 *     vendor operations
 *     simulator operations
 *     future operations
 *
 * Operation names remain open semantic names.
 *
 * ============================================================================
 * BODY INTEGRATION
 * ============================================================================
 *
 * The circuit body consumes the canonical statement grammar.
 *
 * Consequently a circuit can contain:
 *
 *     qubit declarations
 *     classical declarations
 *     quantum operations
 *     measurements
 *     reset
 *     barriers
 *     observables
 *     conditionals
 *     loops
 *     dynamic control
 *     resource requirements
 *     capability requirements
 *     function/circuit calls
 *     hybrid constructs
 *     other statements admitted by the language
 *
 * without duplicating those grammars here.
 *
 * ============================================================================
 * SEMANTIC PIPELINE
 * ============================================================================
 *
 *     source
 *       |
 *       v
 *     canonical lexer
 *       |
 *       v
 *     canonical parser
 *       |
 *       +--> circuits.g4
 *       |
 *       v
 *     domain-neutral frontend AST
 *       |
 *       v
 *     structural validation
 *       |
 *       v
 *     semantic analysis
 *       |
 *       +--> name resolution
 *       +--> type checking
 *       +--> generic checking
 *       +--> effect analysis
 *       +--> capability analysis
 *       +--> resource analysis
 *       +--> ownership/lifetime analysis
 *       +--> quantum semantic validation
 *       |
 *       v
 *     canonical semantic representation
 *       |
 *       v
 *     quantum::ir
 *       |
 *       +--> optimization
 *       +--> decomposition
 *       +--> routing
 *       +--> scheduling
 *       +--> resilience
 *       +--> QEC
 *       +--> ZQN
 *       +--> HAL
 *       |
 *       v
 *     target lowering
 *       |
 *       v
 *     execution
 *
 * No stage after parsing is represented inside this grammar.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * A circuit declaration must preserve at least:
 *
 *     source span
 *     declaration name
 *     visibility
 *     attributes
 *     generic parameters
 *     value parameters
 *     extension/base circuit
 *     where constraints
 *     body
 *
 * Circuit references must preserve:
 *
 *     source span
 *     qualified name
 *     generic arguments
 *     call arguments
 *
 * The parser MUST NOT resolve:
 *
 *     names
 *     types
 *     resources
 *     capabilities
 *     hardware
 *     operations
 *
 * Those belong to semantic analysis.
 *
 * ============================================================================
 * GENERICITY
 * ============================================================================
 *
 * Circuit generic parameters may represent:
 *
 *     types
 *     values
 *     shapes
 *     extents
 *     resource quantities
 *     capabilities
 *     semantic constraints
 *
 * Example:
 *
 *     quantum circuit QFT<N>(
 *         q: QubitRegister<N>
 *     ) {
 *         ...
 *     }
 *
 * `N` is a source-level parameter.
 *
 * It is NOT:
 *
 *     a hardware maximum;
 *     a physical qubit count;
 *     a compiler-imposed limit.
 *
 * ============================================================================
 * EXTENSION / INHERITANCE
 * ============================================================================
 *
 * The optional `extends` clause is syntactic only.
 *
 * Semantic analysis decides:
 *
 *     whether extension is legal;
 *     whether interfaces are compatible;
 *     whether operations are inherited;
 *     whether effects are compatible;
 *     whether resource contracts are compatible;
 *     whether specialization is valid.
 *
 * This file does not implement inheritance semantics.
 *
 * ============================================================================
 * CIRCUIT INTERFACE
 * ============================================================================
 *
 * The circuit interface is represented by the canonical parameter system.
 *
 * Example:
 *
 *     quantum circuit Bell(
 *         a: Qubit,
 *         b: Qubit
 *     ) {
 *         ...
 *     }
 *
 * Quantum/classical distinctions come from the canonical type system.
 *
 * This file therefore does NOT invent:
 *
 *     input
 *     output
 *     inout
 *
 * keyword families.
 *
 * If explicit directional interfaces are introduced later, they must be
 * introduced by the canonical type/interface specification first and then
 * integrated through the appropriate owner.
 *
 * ============================================================================
 * CIRCUIT RETURN VALUES
 * ============================================================================
 *
 * circuits.g4 does not invent a second return-value syntax.
 *
 * Circuit output semantics are represented through:
 *
 *     parameter types
 *     mutable/reference semantics where permitted
 *     canonical expression/statement semantics
 *     measurement result bindings
 *     general function/call semantics
 *
 * If a future circuit-result declaration becomes necessary, it must be
 * specified in grammar/spec/quantum.md and mapped to the domain-neutral AST
 * before being added here.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Circuit bodies may contain the canonical resource/capability statements.
 *
 * For example:
 *
 *     requires capability("quantum.measurement");
 *
 *     requires qubits >= n;
 *
 * Such requirements are semantic intent.
 *
 * They do not allocate hardware.
 *
 * This file therefore does not duplicate:
 *
 *     grammar/resources/
 *     grammar/hardware/
 *     grammar/quantum/quantum-resources.g4
 *     grammar/quantum/quantum-capabilities.g4
 *
 * ============================================================================
 * NO PHYSICAL MAPPING
 * ============================================================================
 *
 * A source circuit may refer to:
 *
 *     q
 *     register[i]
 *     logical_qubit
 *
 * but this does not mean:
 *
 *     physical qubit 0
 *     physical qubit 1
 *     device 17
 *     topology edge 3
 *
 * Physical realization is downstream.
 *
 * ============================================================================
 * DYNAMIC CIRCUITS
 * ============================================================================
 *
 * Dynamic behavior belongs to:
 *
 *     grammar/quantum/dynamic-circuits.g4
 *     grammar/quantum/dynamic-control.g4
 *     grammar/quantum/mid-circuit-control.g4
 *     grammar/quantum/quantum-classical.g4
 *
 * The circuit body admits those constructs through the canonical statement
 * dispatcher rather than redefining them.
 *
 * ============================================================================
 * QUANTUM OPERATIONS
 * ============================================================================
 *
 * The circuit body admits:
 *
 *     quantumOperationStatement
 *
 * through the general quantum statement dispatcher.
 *
 * circuits.g4 MUST NOT define:
 *
 *     quantumOperationStatement
 *
 * or another alias that competes with operations.g4.
 *
 * ============================================================================
 * MEASUREMENT / RESET / BARRIER
 * ============================================================================
 *
 * These remain owned by their dedicated grammar files.
 *
 * circuits.g4 does not redefine:
 *
 *     quantumMeasurementStatement
 *     quantumResetOperation
 *     barrierStatement
 *
 * They enter through the canonical statement/quantum statement composition.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden language-level limits include:
 *
 *     MAX_QUBITS
 *     MAX_REGISTERS
 *     MAX_TARGETS
 *     MAX_PARAMETERS
 *     MAX_OPERATIONS
 *     MAX_CIRCUIT_DEPTH
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_MEMORY
 *
 * No such limits are present in this grammar.
 *
 * ANTLR repetition is intentionally open:
 *
 *     *
 *     +
 *
 * therefore the language does not establish an artificial finite cardinality.
 *
 * Implementation resource limits, if required by a compiler invocation, are
 * external policy and MUST NOT alter the source-language semantics.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no actions
 *     no semantic predicates
 *     no embedded Rust
 *     no randomness
 *     no filesystem access
 *     no networking
 *     no hardware access
 *     no runtime calls
 *     no mutable global state
 *
 * Equal source + equal lexer + equal grammar/version + equal parser
 * configuration must produce the same structural parse.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Parsing a circuit must never:
 *
 *     allocate quantum hardware;
 *     connect to a QPU;
 *     execute an operation;
 *     measure a qubit;
 *     reset hardware;
 *     discover hardware;
 *     access credentials;
 *     access the filesystem;
 *     access the network;
 *     execute user code.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing source form:
 *
 *     circuit Name(...) { ... }
 *
 * remains available through `quantumCircuitDeclaration`.
 *
 * Canonical prefixed form:
 *
 *     quantum circuit Name(...) { ... }
 *
 * is assembled by:
 *
 *     grammar/quantum/quantum.g4
 *
 * using:
 *
 *     QUANTUM quantumCircuitDeclaration
 *
 * This means no source-level rename of `circuits.g4` or its primary rule is
 * required.
 *
 * ============================================================================
 * RULE OWNERSHIP
 * ============================================================================
 *
 * The following rules are intentionally consumed from other grammar owners:
 *
 *     attribute
 *     visibilityModifier
 *     identifier
 *     qualifiedName
 *     genericParameterList
 *     genericArgumentSuffix
 *     parameterList
 *     argumentList
 *     whereClause
 *     expression
 *     statement
 *     blockExpression
 *     typeExpression
 *
 * This file must not redefine them.
 *
 * ============================================================================
 */


/* ============================================================================
 * 1. CANONICAL CIRCUIT DECLARATION
 * ============================================================================
 *
 * Compatibility/source form:
 *
 *     circuit Bell(a: Qubit, b: Qubit) {
 *         ...
 *     }
 *
 * Canonical prefixed form:
 *
 *     quantum circuit Bell(a: Qubit, b: Qubit) {
 *         ...
 *     }
 *
 * `QUANTUM` is deliberately NOT consumed here.
 *
 * `quantum.g4` owns the `quantum` domain prefix and composes this rule.
 */
quantumCircuitDeclaration
    : attribute*
      visibilityModifier?
      CIRCUIT
      identifier
      genericParameterList?
      quantumCircuitParameterClause?
      quantumCircuitExtensionClause?
      whereClause?
      quantumCircuitBody
    ;


/* ============================================================================
 * 2. CIRCUIT PARAMETERS
 * ============================================================================
 *
 * Reuse the universal parameter system.
 *
 * Example:
 *
 *     circuit QFT<N>(q: QubitRegister<N>) {
 *         ...
 *     }
 *
 * The grammar does not inspect or constrain the meaning of parameter types.
 */
quantumCircuitParameterClause
    : LPAREN
      parameterList?
      RPAREN
    ;


/* ============================================================================
 * 3. CIRCUIT EXTENSION
 * ============================================================================
 *
 * Example:
 *
 *     circuit Specialized extends Base(...) {
 *         ...
 *     }
 *
 * The exact semantic legality is determined after parsing.
 *
 * No hardware relationship is implied.
 */
quantumCircuitExtensionClause
    : EXTENDS
      qualifiedName
    ;


/* ============================================================================
 * 4. CIRCUIT BODY
 * ============================================================================
 *
 * The body uses the canonical statement grammar.
 *
 * This is deliberately NOT:
 *
 *     quantumOperation*
 *
 * because a circuit is a complete language scope and may contain classical,
 * quantum, hybrid, resource, concurrency, and other statements supported by
 * the universal language.
 *
 * This also prevents circuits.g4 from duplicating:
 *
 *     operations.g4
 *     measurement.g4
 *     reset.g4
 *     dynamic-control.g4
 *     statements/*.g4
 *     expressions/*.g4
 *     declarations/*.g4
 */
quantumCircuitBody
    : LBRACE
      statement*
      RBRACE
    ;


/* ============================================================================
 * 5. CIRCUIT REFERENCE
 * ============================================================================
 *
 * A circuit reference is a semantic callable reference.
 *
 * Examples:
 *
 *     Bell
 *     library::Bell
 *     algorithms::quantum::QFT
 *
 * Generic specialization is represented by the canonical generic-argument
 * suffix rather than a circuit-specific specialization grammar.
 */
quantumCircuitReference
    : qualifiedName
      genericArgumentSuffix?
    ;


/* ============================================================================
 * 6. CIRCUIT SPECIALIZATION REFERENCE
 * ============================================================================
 *
 * This is intentionally an alias-like semantic boundary and does not create
 * another specialization syntax.
 *
 * Example:
 *
 *     QFT::<N>
 *
 * The actual argument semantics belong to the generic/type system.
 */
quantumCircuitSpecializedReference
    : quantumCircuitReference
    ;


/* ============================================================================
 * 7. CIRCUIT CALL REFERENCE
 * ============================================================================
 *
 * A circuit call uses the universal expression call syntax.
 *
 * This rule is provided for semantic/parser integration where a circuit-aware
 * context needs a named structural reference, but it MUST NOT be added as a
 * competing general expression alternative if the canonical expression
 * grammar already parses the same syntax.
 *
 * Canonical source example:
 *
 *     Bell(a, b)
 *
 * The ordinary expression parser remains authoritative for actual call
 * syntax.
 */
quantumCircuitCallReference
    : quantumCircuitReference
      LPAREN
      argumentList?
      RPAREN
    ;


/* ============================================================================
 * 8. CIRCUIT COMPOSITION REFERENCE
 * ============================================================================
 *
 * Circuit composition is intentionally represented by ordinary calls,
 * expressions, declarations, and statement sequencing.
 *
 * There is no dedicated:
 *
 *     compose
 *
 * keyword here because the canonical lexer does not define such a keyword and
 * the language specification does not require a second composition language.
 *
 * Semantic composition may therefore arise from:
 *
 *     sequential statements;
 *     circuit calls;
 *     ordinary function/call expressions;
 *     higher-order circuit values where supported;
 *     generic specialization;
 *     library abstractions.
 *
 * This rule provides a stable circuit-level semantic boundary without
 * introducing syntax.
 */
quantumCircuitCompositionReference
    : quantumCircuitReference
    ;


/* ============================================================================
 * 9. CIRCUIT EXPRESSION REFERENCE
 * ============================================================================
 *
 * Circuit references may participate in expression-level semantics only where
 * the canonical type system permits callable/circuit values.
 *
 * No separate circuit expression hierarchy is introduced.
 */
quantumCircuitExpressionReference
    : quantumCircuitReference
    ;


/* ============================================================================
 * 10. CIRCUIT DECLARATION FORM
 * ============================================================================
 *
 * Explicit named alias for integration points that need to distinguish a
 * circuit declaration from other quantum declarations.
 *
 * There is exactly one declaration implementation.
 */
quantumCircuitDeclarationForm
    : quantumCircuitDeclaration
    ;


/* ============================================================================
 * 11. CIRCUIT SIGNATURE
 * ============================================================================
 *
 * Signature-only representation is useful for tooling and semantic indexing.
 *
 * It is NOT a second declaration syntax.
 */
quantumCircuitSignature
    : quantumCircuitReference
      genericParameterList?
      quantumCircuitParameterClause?
    ;


/* ============================================================================
 * 12. CIRCUIT TARGET-INDEPENDENT INTERFACE
 * ============================================================================
 *
 * This rule exposes the reusable interface structure without introducing
 * physical or backend-specific information.
 */
quantumCircuitInterface
    : quantumCircuitParameterClause?
    ;


/* ============================================================================
 * 13. CIRCUIT RESOURCE-SAFE REFERENCE
 * ============================================================================
 *
 * Circuit references carry no resource allocation.
 *
 * Resource requirements remain ordinary semantic expressions and are handled
 * by the resource/capability system.
 *
 * This rule deliberately aliases the canonical circuit reference rather than
 * inventing resource-specific syntax.
 */
quantumCircuitResourceIndependentReference
    : quantumCircuitReference
    ;


/* ============================================================================
 * 14. CIRCUIT DOMAIN BOUNDARY
 * ============================================================================
 *
 * This is the integration point used by quantum.g4.
 *
 * The parent grammar is responsible for:
 *
 *     QUANTUM quantumCircuitDeclaration
 *
 * which produces:
 *
 *     quantum circuit ...
 *
 * The compatibility/root grammar may admit:
 *
 *     quantumCircuitDeclaration
 *
 * directly for:
 *
 *     circuit ...
 *
 * No duplicate circuit declaration production is required elsewhere.
 */
quantumCircuitDomainDeclaration
    : quantumCircuitDeclaration
    ;


/* ============================================================================
 * 15. CIRCUIT SOURCE-LEVEL GENERICITY
 * ============================================================================
 *
 * Genericity is deliberately inherited from the universal generic grammar.
 *
 * The circuit grammar therefore does not define:
 *
 *     <N>
 *     <T>
 *     <Q>
 *
 * itself.
 *
 * All such forms are owned by genericParameterList/genericArgumentSuffix.
 */
quantumCircuitGenericReference
    : identifier
      genericArgumentSuffix?
    ;


/* ============================================================================
 * 16. CIRCUIT SOURCE-LEVEL PARAMETERIZATION
 * ============================================================================
 *
 * Parameter semantics are inherited from the universal parameter system.
 *
 * No circuit-specific parameter value language is introduced.
 */
quantumCircuitParameterReference
    : identifier
    ;


/* ============================================================================
 * 17. CIRCUIT BODY INTEGRATION CONTRACT
 * ============================================================================
 *
 * Every statement inside:
 *
 *     quantumCircuitBody
 *
 * is parsed through:
 *
 *     statement
 *
 * Therefore the following remain owned elsewhere:
 *
 *     quantum operations       -> quantum/operations.g4
 *     measurements             -> quantum/measurement.g4
 *     reset                    -> quantum/reset.g4
 *     barriers                 -> quantum/gates.g4
 *     observables              -> quantum/observables.g4
 *     qubits                   -> quantum/qubits.g4
 *     registers                -> quantum/quantum-registers.g4
 *     dynamic control          -> quantum/dynamic-circuits.g4
 *     mid-circuit control      -> quantum/mid-circuit-control.g4
 *     hybrid boundaries        -> quantum/quantum-classical.g4
 *     resources                -> resources/
 *     capabilities             -> resources/ / quantum/
 *     general control flow     -> statements/
 *     expressions              -> expressions/
 *     types                    -> types/
 *
 * This is the central integration guarantee of this file.
 */


/* ============================================================================
 * 18. CIRCUIT RESOURCE REQUIREMENTS
 * ============================================================================
 *
 * No circuit-specific resource grammar is introduced.
 *
 * A circuit may contain canonical resource requirements through its body.
 *
 * Examples:
 *
 *     requires qubits >= n;
 *
 *     requires capability("quantum.measurement");
 *
 *     requires capability("quantum.mid_circuit_measurement");
 *
 * These expressions remain symbolic until resource analysis.
 *
 * The parser does not evaluate them.
 */


/* ============================================================================
 * 19. CIRCUIT CAPABILITY REQUIREMENTS
 * ============================================================================
 *
 * Capability names are semantic data.
 *
 * Examples include:
 *
 *     quantum.measurement
 *     quantum.reset
 *     quantum.mid_circuit_measurement
 *     quantum.dynamic_control
 *
 * No capability is bound to a physical device here.
 */


/* ============================================================================
 * 20. LOGICAL / PHYSICAL RESOURCE SEPARATION
 * ============================================================================
 *
 * A circuit parameter may have a semantic type such as:
 *
 *     Qubit
 *     QubitRegister<N>
 *     LogicalQubit
 *     QubitView
 *
 * depending on the canonical type system.
 *
 * The grammar does not convert these into:
 *
 *     PhysicalQubitId
 *
 * and does not assign target resources.
 *
 * The canonical physical identity remains owned by:
 *
 *     src/quantum/ir/quantum/qubit
 *
 * after semantic lowering.
 */


/* ============================================================================
 * 21. QEC BOUNDARY
 * ============================================================================
 *
 * A circuit may contain source-level QEC intent through the canonical quantum
 * error-correction grammar.
 *
 * circuits.g4 does not define:
 *
 *     code distance
 *     stabilizer matrices
 *     decoder algorithms
 *     syndrome extraction implementation
 *     recovery matrices
 *
 * Those belong downstream.
 *
 * QEC may transform or annotate the canonical quantum semantic/IR
 * representation without requiring a new circuit grammar.
 */


/* ============================================================================
 * 22. ZQN BOUNDARY
 * ============================================================================
 *
 * Noise/fault semantics are not implemented by this file.
 *
 * Circuit source may carry semantic requirements or attributes that are later
 * interpreted by ZQN.
 *
 * This grammar does not encode:
 *
 *     physical error rates
 *     calibration constants
 *     device noise tables
 *     fault probabilities
 *     hardware-specific noise channels
 */


/* ============================================================================
 * 23. ROUTING BOUNDARY
 * ============================================================================
 *
 * A circuit's source qubit references are logical/semantic references.
 *
 * Routing may later perform:
 *
 *     mapping
 *     remapping
 *     SWAP insertion
 *     teleportation
 *     decomposition
 *     movement
 *     ancilla placement
 *
 * without changing the circuit source grammar.
 */


/* ============================================================================
 * 24. SCHEDULING BOUNDARY
 * ============================================================================
 *
 * This grammar does not encode:
 *
 *     cycle numbers
 *     physical slots
 *     pulse slots
 *     device queues
 *     hardware clock cycles
 *
 * Scheduling consumes canonical semantic/IR information downstream.
 */


/* ============================================================================
 * 25. OPTIMIZATION BOUNDARY
 * ============================================================================
 *
 * Circuit syntax expresses semantic computation.
 *
 * Optimization may later:
 *
 *     fuse operations
 *     eliminate redundant operations
 *     decompose operations
 *     specialize generics
 *     change implementation strategy
 *
 * while preserving the observable semantics represented by the canonical IR.
 */


/* ============================================================================
 * 26. HDL / HARDWARE CO-DESIGN
 * ============================================================================
 *
 * A circuit may participate in a larger hardware/software co-design program.
 *
 * This grammar does not define:
 *
 *     wires
 *     ports
 *     fixed register widths
 *     LUT counts
 *     DSP counts
 *     BRAM counts
 *     ASIC cell counts
 *     physical pin assignments
 *     physical clock limits
 *
 * Such constructs belong to:
 *
 *     grammar/hdl/
 *     grammar/hardware/
 *     grammar/resources/
 *
 * Shared semantics occur through the AST/resource/IR boundaries.
 */


/* ============================================================================
 * 27. DISTRIBUTED QUANTUM COMPUTATION
 * ============================================================================
 *
 * A circuit may participate in distributed execution.
 *
 * The circuit grammar does not encode:
 *
 *     node 0
 *     node 1
 *     fixed node count
 *     network topology
 *     physical link IDs
 *     fixed latency
 *
 * Those are deployment/resource concerns.
 */


/* ============================================================================
 * 28. HYBRID COMPUTATION
 * ============================================================================
 *
 * Because the body consumes the universal statement grammar, a circuit may
 * contain classical and quantum computation in the same source scope.
 *
 * Example:
 *
 *     quantum circuit Adaptive(q: Qubit, c: Bit) {
 *         measure q into c;
 *
 *         if c == 1 {
 *             apply correction to q;
 *         }
 *     }
 *
 * Exact measurement and control syntax remains owned by the corresponding
 * grammar fragments.
 */


/* ============================================================================
 * 29. OPEN-WORLD OPERATION MODEL
 * ============================================================================
 *
 * Circuit syntax does not enumerate operations.
 *
 * The following names are therefore not grammar-level gate types:
 *
 *     H
 *     X
 *     Y
 *     Z
 *     CNOT
 *     SWAP
 *     RX
 *     RY
 *     RZ
 *     U
 *     vendor.operation
 *     future.operation
 *
 * They remain source-level operation names consumed by operations.g4.
 *
 * Semantic analysis determines:
 *
 *     existence
 *     namespace
 *     parameters
 *     operand roles
 *     effects
 *     capabilities
 *     resource requirements
 *     canonical quantum semantics
 */


/* ============================================================================
 * 30. FRONTEND AST INTEGRATION
 * ============================================================================
 *
 * Circuit declaration syntax must lower into the repository's domain-neutral
 * frontend AST.
 *
 * It must not require:
 *
 *     QuantumGate
 *     PhysicalGate
 *     DeviceCircuit
 *     QPUCircuit
 *
 * as parser-specific AST types.
 *
 * Operation-bearing circuit bodies must remain compatible with the generic
 * operation representation already established by the frontend architecture:
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
 * Circuit declarations may have a dedicated declaration node because the
 * declaration itself has language-level semantics, but operation semantics
 * remain generic.
 */


/* ============================================================================
 * 31. QUANTUM::IR INTEGRATION
 * ============================================================================
 *
 * Circuit lowering terminates at the canonical quantum::ir boundary.
 *
 * Conceptually:
 *
 *     circuit declaration
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic circuit
 *          |
 *          v
 *     quantum::ir::model / quantum::ir::program
 *          |
 *          v
 *     quantum::ir quantum semantics
 *
 * There is NO:
 *
 *     circuits.g4 IR
 *
 * and NO second frontend quantum IR.
 *
 * The existing `quantum::ir` architecture already distinguishes:
 *
 *     universal program representation
 *     circuit model
 *     quantum semantics
 *     resources
 *     scheduling
 *     control
 *
 * This grammar must preserve that separation.
 */


/* ============================================================================
 * 32. OPENQASM / INTEROPERABILITY
 * ============================================================================
 *
 * OpenQASM and other external representations are interoperability inputs,
 * not alternative canonical Zamani circuit semantics.
 *
 * External source:
 *
 *     OpenQASM
 *        |
 *        v
 *     frontend adapter
 *        |
 *        v
 *     canonical AST/semantic representation
 *        |
 *        v
 *     quantum::ir
 *
 * circuits.g4 remains the native Zamani syntax boundary.
 */


/* ============================================================================
 * 33. SERIALIZATION / PROVENANCE
 * ============================================================================
 *
 * The grammar itself performs no serialization.
 *
 * Downstream representations must preserve sufficient provenance to identify:
 *
 *     circuit declaration
 *     circuit reference
 *     source span
 *     generic arguments
 *     parameter bindings
 *     source attributes
 *
 * Canonical serialization remains owned by the appropriate AST/IR
 * serialization systems.
 */


/* ============================================================================
 * 34. DIAGNOSTICS
 * ============================================================================
 *
 * Syntax diagnostics owned at this boundary include:
 *
 *     missing circuit name
 *     malformed generic parameter attachment
 *     malformed parameter list
 *     malformed extension clause
 *     malformed where clause
 *     missing circuit body
 *     malformed body delimiters
 *
 * Semantic diagnostics remain downstream:
 *
 *     duplicate circuit declaration
 *     unresolved circuit
 *     invalid generic argument
 *     invalid parameter type
 *     incompatible extension
 *     invalid resource requirement
 *     unavailable capability
 *     invalid operation
 *     invalid quantum operand
 *     insufficient resources
 *
 * Parser diagnostics must preserve exact source spans.
 */


/* ============================================================================
 * 35. DETERMINISTIC PARSING
 * ============================================================================
 *
 * This file contains no:
 *
 *     semantic predicates
 *     embedded actions
 *     callbacks
 *     filesystem access
 *     network access
 *     runtime execution
 *     hardware discovery
 *     randomness
 *
 * The parser is therefore deterministic for a fixed:
 *
 *     source
 *     lexer
 *     grammar version
 *     parser configuration
 */


/* ============================================================================
 * 36. SCALABILITY
 * ============================================================================
 *
 * No language-level finite limit is introduced for:
 *
 *     circuits
 *     generic parameters
 *     circuit parameters
 *     statements in a circuit
 *     nested circuit scopes
 *     circuit calls
 *     operation statements
 *     quantum targets
 *     classical values
 *     quantum resources
 *     circuit depth
 *     program size
 *
 * Physical execution remains constrained by actual resources.
 *
 * This is the intended distinction:
 *
 *     language model = target-independent
 *
 *     implementation resources = finite/configurable
 *
 *     target resources = discovered/negotiated
 *
 * ============================================================================
 */


/* ============================================================================
 * 37. RUST SAFETY
 * ============================================================================
 *
 * This grammar contains no Rust actions.
 *
 * The compiler/frontend implementation consuming it must remain compatible
 * with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and:
 *
 *     #![forbid(unsafe_code)]
 *
 * No `unsafe` implementation is required for this grammar.
 */


/* ============================================================================
 * 38. TEST CONTRACT
 * ============================================================================
 *
 * The grammar-level tests for this file belong under:
 *
 *     grammar/tests/quantum/
 *
 * and repository parser tests.
 *
 * POSITIVE
 * --------
 *
 *     circuit Bell(a: Qubit, b: Qubit) {
 *         apply H to a;
 *         apply CNOT to a, b;
 *     }
 *
 *     quantum circuit Bell(a: Qubit, b: Qubit) {
 *         apply H to a;
 *         apply CNOT to a, b;
 *     }
 *
 * The second form is assembled through quantum.g4:
 *
 *     QUANTUM quantumCircuitDeclaration
 *
 * Generic:
 *
 *     circuit QFT<N>(q: QubitRegister<N>) {
 *         ...
 *     }
 *
 * Qualified reference:
 *
 *     library::QFT::<N>(q);
 *
 * Extension:
 *
 *     circuit Specialized extends Base<T> {
 *         ...
 *     }
 *
 * Body composition:
 *
 *     circuit Adaptive(q: Qubit, c: Bit) {
 *         let threshold = 1;
 *
 *         if threshold == 1 {
 *             apply correction to q;
 *         }
 *     }
 *
 * NEGATIVE
 * --------
 *
 *     circuit { ... }
 *
 *     circuit 123 { ... }
 *
 *     circuit Name( { ... }
 *
 *     circuit Name<T { ... }
 *
 *     circuit Name extends { ... }
 *
 *     circuit Name( ... )
 *
 * BOUNDARY
 * --------
 *
 * Tests must cover:
 *
 *     empty body
 *     one statement
 *     many statements
 *     deeply nested blocks
 *     generic parameters
 *     symbolic values
 *     qualified names
 *     Unicode identifiers where supported
 *     large parameter lists
 *     large statement sequences
 *     nested circuit declarations where semantically permitted
 *
 * SCALABILITY
 * -----------
 *
 * Generate test programs parametrically.
 *
 * Do not use a fixed test size as a language limit.
 *
 * DETERMINISM
 * -----------
 *
 * Parse identical source repeatedly and compare structural parse output.
 *
 * COMPATIBILITY
 * -------------
 *
 * Existing valid:
 *
 *     circuit Name(...) { ... }
 *
 * syntax must remain accepted unless explicitly deprecated.
 *
 * CROSS-DOMAIN
 * ------------
 *
 * Circuit bodies must be tested with:
 *
 *     classical statements
 *     quantum operations
 *     measurement
 *     dynamic control
 *     resource requirements
 *     capability requirements
 *     hybrid control
 *     distributed/resource intent where permitted
 */


/* ============================================================================
 * 39. INTEGRATION CHECKLIST
 * ============================================================================
 *
 * This file is independently complete when:
 *
 * [x] It owns circuit declaration structure.
 * [x] It does not create a second quantum IR.
 * [x] It does not enumerate gates.
 * [x] It does not introduce physical resource limits.
 * [x] It does not invent `call`, `compose`, or `instantiate` keywords.
 * [x] It reuses canonical parameters.
 * [x] It reuses canonical generics.
 * [x] It reuses canonical expressions.
 * [x] It reuses canonical statements.
 * [x] It reuses canonical types through parameters.
 * [x] It keeps operation ownership in operations.g4.
 * [x] It keeps measurement ownership in measurement.g4.
 * [x] It keeps reset ownership in reset.g4.
 * [x] It keeps dynamic-control ownership in its dedicated grammars.
 * [x] It keeps resource/capability ownership downstream.
 * [x] It keeps routing downstream.
 * [x] It keeps scheduling downstream.
 * [x] It keeps QEC downstream.
 * [x] It keeps ZQN downstream.
 * [x] It keeps HAL downstream.
 * [x] It is target-independent.
 * [x] It is compatible with safe Rust 1.97/1.97.1.
 * [x] It introduces no unsafe implementation requirement.
 *
 * Repository integration required:
 *
 * [ ] quantum.g4 composes:
 *
 *         QUANTUM quantumCircuitDeclaration
 *
 *     for canonical:
 *
 *         quantum circuit ...
 *
 * [ ] the universal declaration dispatcher admits
 *     quantumCircuitDeclaration exactly once for the compatibility form.
 *
 * [ ] no other grammar defines another authoritative
 *     quantumCircuitDeclaration.
 *
 * [ ] statement grammar continues to own `statement`.
 *
 * [ ] expression grammar remains the sole owner of general call syntax.
 *
 * [ ] operations.g4 remains the sole owner of quantum operation invocation.
 *
 * [ ] frontend AST has a circuit declaration representation.
 *
 * [ ] circuit references lower through the existing generic call/expression
 *     representation.
 *
 * [ ] semantic analysis resolves whether a callable is a circuit.
 *
 * [ ] lowering reaches canonical quantum::ir.
 *
 * [ ] repository conformance tests pass.
 */


/* ============================================================================
 * 40. FINAL ARCHITECTURAL INVARIANT
 * ============================================================================
 *
 *     CIRCUIT SOURCE
 *          |
 *          v
 *     DOMAIN-NEUTRAL AST
 *          |
 *          v
 *     SEMANTIC CIRCUIT
 *          |
 *          v
 *     CANONICAL QUANTUM::IR
 *          |
 *          +--------------------+
 *          |                    |
 *          v                    v
 *     optimization          analysis
 *          |
 *          +--------------------+
 *          |
 *          v
 *     routing / scheduling / resilience
 *          |
 *          +--------------------+
 *          |
 *          v
 *     QEC / ZQN
 *          |
 *          v
 *     HAL
 *          |
 *          v
 *     TARGET REALIZATION
 *
 * The source circuit describes WHAT the computation means.
 *
 * It does not describe WHICH machine must execute it.
 *
 * That separation is the grammar-level foundation for:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */
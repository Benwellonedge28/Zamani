/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/validation/ensures.g4
 *
 * GRAMMAR
 * -------
 * Ensures
 *
 * STATUS
 * ------
 * Production validation/conformance component
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file provides the isolated parser/validation entry point for the
 * canonical Zamani `ensures` contract statement.
 *
 * IMPORTANT:
 *
 * This file is NOT a second owner of `ensures` syntax.
 *
 * Canonical source-level ownership remains:
 *
 *     grammar/statements/contract.g4
 *         -> ContractStatements
 *         -> ensuresStatement
 *
 * This file only exposes that canonical rule through a validation-specific
 * complete-input entry point.
 *
 * The ownership model is therefore:
 *
 *     source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       v
 *     ContractStatements
 *       |
 *       +--> ensuresStatement
 *       |
 *       v
 *     Ensures validation facade
 *
 * ============================================================================
 * IMPLEMENTATION BASELINE
 * ============================================================================
 *
 * Rust:
 *
 *     Rust 1.97 / Rust 1.97.1
 *     Rust Edition 2021
 *     Safe Rust only
 *
 * ANTLR:
 *
 *     ANTLR4 parser grammar
 *
 * This grammar contains:
 *
 *     - no embedded Rust;
 *     - no semantic predicates;
 *     - no semantic actions;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware access;
 *     - no runtime execution;
 *     - no resource allocation;
 *     - no target selection;
 *     - no backend selection;
 *     - no machine-capacity constants;
 *     - no physical-device knowledge;
 *     - no quantum-device knowledge.
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * Provide a stable, isolated validation/conformance entry point for:
 *
 *     ensures(condition);
 *
 * while preserving the canonical contract grammar as the sole syntax owner.
 *
 * OWNS
 * ----
 *
 *     ensuresValidationUnit
 *
 * This is the public complete-input validation entry point.
 *
 *     ensuresValidationItem
 *
 * This is the isolated validation item wrapper.
 *
 * DOES NOT OWN
 * -------------
 *
 *     ensuresStatement
 *     contractStatement
 *     contractCondition
 *
 * Those remain owned by:
 *
 *     grammar/statements/contract.g4
 *
 * This file also does not own:
 *
 *     lexer vocabulary
 *     keywords
 *     punctuation
 *     identifiers
 *     names
 *     expressions
 *     types
 *     effects
 *     capabilities
 *     resources
 *     policies
 *     provenance
 *     AST structures
 *     semantic analysis
 *     verification
 *     proof search
 *     runtime checking
 *     quantum operations
 *     HDL syntax
 *     hardware syntax
 *     backend selection
 *     IR construction
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON
 * ----------
 *
 *     grammar/statements/contract.g4
 *         ContractStatements
 *
 *     grammar/antlr/ZamaniLexer.g4
 *         canonical public lexer boundary
 *
 *     grammar/expressions/
 *         indirectly through ContractStatements
 *
 *     grammar/core/punctuation.g4
 *         indirectly through ContractStatements
 *
 * EXPORTS
 * -------
 *
 *     ensuresValidationUnit
 *     ensuresValidationItem
 *
 * CONSUMED_BY
 * ----------
 *
 *     validation/conformance tooling
 *     isolated contract parser tests
 *     grammar regression tests
 *     diagnostics tests
 *     source-span tests
 *     validation orchestration
 *
 * AST_OWNER
 * ---------
 *
 *     repository frontend AST subsystem
 *
 * This grammar creates parser contexts only.
 *
 * SEMANTIC_OWNER
 * --------------
 *
 *     contract semantic validation
 *     semantic analysis
 *     verification infrastructure where applicable
 *
 * IR_OWNER
 * --------
 *
 *     canonical semantic model
 *     applicable domain IR
 *
 * Quantum programs ultimately use:
 *
 *     quantum::ir
 *
 * TEST_OWNER
 * ----------
 *
 *     grammar/tests/
 *     validation conformance tests
 *     contract tests
 *
 * SPEC_OWNER
 * ----------
 *
 *     grammar/specification/contracts.md
 *     grammar/spec/contracts.md
 *
 * ============================================================================
 * CANONICAL OWNERSHIP
 * ============================================================================
 *
 * The canonical source grammar is:
 *
 *     grammar/statements/contract.g4
 *
 * Its grammar is:
 *
 *     ContractStatements
 *
 * Its canonical rule is:
 *
 *     ensuresStatement
 *
 * The canonical production is:
 *
 *     ensuresStatement
 *         : ENSURES contractCondition statementTerminator
 *         ;
 *
 * `contractCondition` remains owned by ContractStatements and ultimately
 * delegates to the canonical expression grammar.
 *
 * This file MUST NOT reproduce that rule.
 *
 * DO NOT add another rule such as:
 *
 *     ensuresStatement
 *         : ENSURES LPAREN expression RPAREN statementTerminator
 *         ;
 *
 * Doing so would create two grammar owners for the same source construct.
 *
 * ============================================================================
 * WHY THIS FILE EXISTS
 * ============================================================================
 *
 * The production compiler does not need a separate `ensures` grammar.
 *
 * However, validation tooling benefits from isolated entry points that can
 * parse one complete construct and guarantee that no trailing source is
 * silently accepted.
 *
 * Therefore:
 *
 *     ensuresValidationUnit
 *         -> ensuresValidationItem EOF
 *
 * gives validation tooling an exact complete-input boundary.
 *
 * The production program parser continues to use:
 *
 *     ContractStatements
 *
 * through the normal statement-dispatch architecture.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This file consumes the repository's canonical:
 *
 *     ZamaniLexer
 *
 * It does not define lexical tokens.
 *
 * In particular, it does not define:
 *
 *     ENSURES
 *     REQUIRES
 *     INVARIANT
 *     ASSUME
 *     GUARANTEE
 *     PROPERTY
 *
 * Those tokens belong to the canonical lexical hierarchy.
 *
 * This prevents:
 *
 *     validation/ensures.g4
 *
 * from becoming a competing lexical authority.
 *
 * ============================================================================
 * PARSER CONTRACT
 * ============================================================================
 *
 * The parser dependency is:
 *
 *     Ensures
 *       |
 *       +--> ContractStatements
 *               |
 *               +--> ensuresStatement
 *                       |
 *                       +--> contractCondition
 *                               |
 *                               +--> expression
 *
 * The validation facade therefore inherits the exact canonical syntax.
 *
 * ============================================================================
 * PUBLIC RULES
 * ============================================================================
 *
 * ensuresValidationUnit
 *
 *     Complete-input validation entry point.
 *
 *     It requires exactly one canonical `ensures` statement followed by EOF.
 *
 * ensuresValidationItem
 *
 *     Isolated validation wrapper around the canonical ensuresStatement rule.
 *
 * These names are intentionally validation-specific so they cannot be
 * confused with the source-language owner `ensuresStatement`.
 *
 * ============================================================================
 * PRIVATE RULES
 * ============================================================================
 *
 * None.
 *
 * This is deliberate.
 *
 * Introducing private copies of:
 *
 *     contractCondition
 *     expression
 *     statementTerminator
 *
 * would violate ownership boundaries.
 *
 * ============================================================================
 * SOURCE SYNTAX CONTRACT
 * ============================================================================
 *
 * The canonical source syntax is inherited from:
 *
 *     ContractStatements.ensuresStatement
 *
 * Conceptually:
 *
 *     ensures(condition);
 *
 * The actual condition syntax remains the responsibility of the expression
 * grammar.
 *
 * This file does not constrain the condition to any particular domain.
 *
 * ============================================================================
 * EXPRESSION CONTRACT
 * ============================================================================
 *
 * `ensures` consumes the canonical contract condition.
 *
 * The condition can therefore participate in ordinary Zamani expression
 * semantics, including expressions involving:
 *
 *     values
 *     variables
 *     function results
 *     records
 *     arrays
 *     tensors
 *     data
 *     measurements
 *     quantum-derived values
 *     hardware state
 *     distributed state
 *     AI/model values
 *     provenance
 *     policies
 *     capabilities
 *     resource information
 *
 * This grammar does not define any of those expressions.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar creates parser contexts only.
 *
 * The canonical AST mapping is downstream.
 *
 * The resulting semantic representation must preserve at least:
 *
 *     contract kind = ensures
 *     condition
 *     source span
 *     enclosing scope
 *     source order
 *
 * The validation facade must not require a new AST node merely because the
 * construct was parsed through a validation entry point.
 *
 * In other words:
 *
 *     production parser
 *          |
 *          v
 *     ensuresStatement
 *          |
 *          v
 *     canonical AST
 *
 * and:
 *
 *     validation parser
 *          |
 *          v
 *     ensuresValidationUnit
 *          |
 *          v
 *     ensuresStatement
 *          |
 *          v
 *     same canonical AST model
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * `ensures` describes a postcondition/guarantee associated with the enclosing
 * semantic context.
 *
 * This grammar does not determine:
 *
 *     when the condition is checked;
 *     whether it is statically provable;
 *     whether it becomes a runtime check;
 *     whether it is discharged by a proof system;
 *     whether it is inherited;
 *     whether it is refined;
 *     whether it is weakened or strengthened;
 *     whether it is relevant to a particular backend.
 *
 * Those decisions belong to semantic and verification layers.
 *
 * A structurally valid statement may still be semantically invalid.
 *
 * Examples of downstream semantic diagnostics include:
 *
 *     invalid name
 *     invalid scope
 *     invalid condition type
 *     invalid result reference
 *     unsupported postcondition
 *     policy violation
 *     impossible guarantee
 *     unverifiable property
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * This grammar does not introduce a special:
 *
 *     EnsuresType
 *
 * or domain-specific condition type.
 *
 * The condition uses the canonical expression/type system.
 *
 * Semantic analysis determines whether the expression is valid in the
 * enclosing context.
 *
 * This allows the same construct to work with:
 *
 *     classical values
 *     quantum values
 *     measurement results
 *     tensors
 *     hardware state
 *     distributed state
 *     AI/model state
 *     data values
 *
 * without changing this grammar.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Parsing `ensures` has no runtime effect.
 *
 * The condition itself may reference semantic operations whose evaluation
 * carries effects.
 *
 * Effect checking therefore remains downstream.
 *
 * Possible effects include, where supported by the existing effect system:
 *
 *     IO
 *     network
 *     mutation
 *     randomness
 *     native
 *     foreign
 *     distributed
 *     measurement
 *     learning
 *     adaptation
 *     reflection
 *     code_generation
 *     simulation
 *
 * This file does not enumerate or redefine those effects.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * An ensures condition may semantically depend on capabilities.
 *
 * For example:
 *
 *     ensures(capability(\"quantum.measurement\"));
 *
 * or:
 *
 *     ensures(capability(\"tensor.compute\"));
 *
 * The parser does not determine whether such capabilities exist.
 *
 * The semantic/resource system resolves capability identity and availability.
 *
 * Therefore:
 *
 *     syntax
 *         ->
 *     AST
 *         ->
 *     semantic analysis
 *         ->
 *     capability analysis
 *
 * This is essential for POCO-REAF.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * An ensures condition may refer to resource-related semantic values.
 *
 * Examples:
 *
 *     ensures(memory_used <= allowed_memory);
 *     ensures(qubits_used <= required_qubits);
 *     ensures(result_size <= available_capacity);
 *
 * The grammar imposes no universal physical capacity.
 *
 * It contains no limits for:
 *
 *     memory
 *     qubits
 *     CPUs
 *     cores
 *     GPUs
 *     FPGAs
 *     ASIC resources
 *     accelerators
 *     QPUs
 *     nodes
 *     devices
 *     threads
 *     tensor rank
 *     registers
 *     network size
 *
 * Resource feasibility belongs to semantic analysis, planning, and target
 * realization.
 *
 * ============================================================================
 * CONTRACT CONTRACT
 * ============================================================================
 *
 * `ensures` participates in the universal contract model:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * Ownership remains:
 *
 *     grammar/statements/contract.g4
 *
 * This file only exposes the ensures member of that model for isolated
 * validation.
 *
 * A semantic contract model may represent:
 *
 *     kind
 *     condition
 *     scope
 *     assumptions
 *     requirements
 *     guarantees
 *     evidence
 *     provenance
 *
 * The parser facade does not define that semantic structure.
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Policies may affect whether an ensures condition is:
 *
 *     permitted
 *     enforceable
 *     verifiable
 *     deployable
 *     runtime-checkable
 *     admissible in a particular execution mode
 *
 * Policy interpretation remains downstream.
 *
 * This file does not define policy syntax.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Validation and semantic tooling must preserve enough source information to
 * associate the ensures construct with:
 *
 *     source file
 *     source span
 *     contract kind
 *     condition
 *     enclosing declaration
 *     enclosing module
 *
 * Later stages may add:
 *
 *     derived_from
 *     generated_by
 *     transformed_by
 *     verified_by
 *     evidence
 *     decision
 *
 * This grammar does not implement provenance storage.
 *
 * ============================================================================
 * VERIFICATION CONTRACT
 * ============================================================================
 *
 * This file validates syntax only.
 *
 * It does not:
 *
 *     prove the postcondition;
 *     execute the postcondition;
 *     generate a theorem;
 *     invoke a solver;
 *     inspect hardware;
 *     inspect runtime state;
 *     run a test;
 *     evaluate a model;
 *     perform quantum measurement.
 *
 * Verification, proof, symbolic reasoning, runtime checking, and testing are
 * separate downstream concerns.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Quantum programs may contain ensures conditions referring to quantum
 * semantics.
 *
 * Example:
 *
 *     ensures(measurement.is_valid());
 *
 * or:
 *
 *     ensures(capability(\"quantum.measurement\"));
 *
 * This file does not define quantum syntax.
 *
 * It does not:
 *
 *     enumerate gates;
 *     enumerate qubits;
 *     encode coupling maps;
 *     encode calibration;
 *     select physical qubits;
 *     perform routing;
 *     perform scheduling;
 *     perform decomposition;
 *     perform QEC;
 *     create a quantum IR.
 *
 * The downstream quantum boundary remains:
 *
 *     semantic quantum model
 *         ->
 *     quantum::ir
 *         ->
 *     optimization
 *         ->
 *     decomposition
 *         ->
 *     routing
 *         ->
 *     scheduling
 *         ->
 *     resilience / QEC
 *         ->
 *     ZQN
 *         ->
 *     HAL
 *         ->
 *     target realization
 *
 * ============================================================================
 * HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * Hardware-oriented ensures conditions remain target-independent at the
 * grammar layer.
 *
 * Examples:
 *
 *     ensures(output.is_valid());
 *     ensures(signal.is_stable());
 *     ensures(hardware_state.is_consistent());
 *
 * This grammar does not encode:
 *
 *     fixed signal widths
 *     fixed register widths
 *     fixed memory sizes
 *     fixed FPGA dimensions
 *     fixed ASIC resource counts
 *     fixed clock counts
 *     physical device IDs
 *     vendor-specific topology
 *
 * Those concerns belong to hardware semantics and target realization.
 *
 * ============================================================================
 * CLASSICAL BOUNDARY
 * ============================================================================
 *
 * Classical computation uses exactly the same ensures construct.
 *
 * Example:
 *
 *     ensures(result >= input);
 *
 * No separate classical postcondition grammar exists.
 *
 * ============================================================================
 * HYBRID BOUNDARY
 * ============================================================================
 *
 * Hybrid classical/quantum computation uses the same ensures grammar.
 *
 * Example:
 *
 *     ensures(classical_result.is_valid());
 *
 * The semantic model determines whether the condition crosses:
 *
 *     classical
 *     quantum
 *     accelerator
 *     hardware
 *
 * boundaries.
 *
 * ============================================================================
 * AI / DATA BOUNDARY
 * ============================================================================
 *
 * AI and data computation can use ensures for semantic guarantees.
 *
 * Examples:
 *
 *     ensures(model.is_valid());
 *     ensures(decision.has_evidence());
 *     ensures(output.schema_matches(expected_schema));
 *     ensures(confidence >= required_confidence);
 *
 * No AI-specific postcondition grammar is necessary.
 *
 * ============================================================================
 * DISTRIBUTED BOUNDARY
 * ============================================================================
 *
 * Distributed computation may use ensures conditions involving:
 *
 *     consistency
 *     availability
 *     message state
 *     service state
 *     replicated state
 *     task completion
 *     provenance
 *
 * The grammar remains unchanged.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar produces no IR.
 *
 * It must not introduce:
 *
 *     EnsuresIR
 *     ContractIR
 *     QuantumEnsuresIR
 *     HardwareEnsuresIR
 *
 * or another intermediate representation.
 *
 * The downstream pipeline remains:
 *
 *     source
 *       ->
 *     lexer
 *       ->
 *     parser
 *       ->
 *     domain-neutral AST
 *       ->
 *     structural validation
 *       ->
 *     semantic model
 *       ->
 *     type analysis
 *       ->
 *     effect analysis
 *       ->
 *     capability analysis
 *       ->
 *     resource analysis
 *       ->
 *     contract analysis
 *       ->
 *     policy analysis
 *       ->
 *     provenance
 *       ->
 *     canonical IR / domain IR
 *       ->
 *     optimization
 *       ->
 *     lowering
 *       ->
 *     routing / scheduling where applicable
 *       ->
 *     resilience / QEC where applicable
 *       ->
 *     ZQN / HAL where applicable
 *       ->
 *     target realization
 *
 * ============================================================================
 * BACKEND CONTRACT
 * ============================================================================
 *
 * Backends may choose to:
 *
 *     statically discharge an ensures condition;
 *     preserve it as metadata;
 *     generate runtime checking;
 *     turn it into a proof obligation;
 *     verify it during simulation;
 *     verify it during deployment;
 *     reject an unsupported guarantee.
 *
 * None of these decisions belong to this grammar.
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * The parser must report malformed source structurally.
 *
 * Examples of syntax errors:
 *
 *     ensures;
 *
 *     ensures();
 *
 *     ensures(;
 *
 *     ensures(, condition);
 *
 *     ensures(condition;
 *
 *     ensures(condition,);
 *
 *     ensures(condition) trailing;
 *
 *     ensures(condition) extra;
 *
 * These are parser/conformance failures.
 *
 * The following are semantic concerns instead:
 *
 *     unknown name
 *     invalid scope
 *     invalid condition type
 *     invalid result reference
 *     unavailable capability
 *     unavailable resource
 *     policy conflict
 *     unsupported verification obligation
 *     impossible guarantee
 *
 * A syntactically valid program must not be rejected merely because the
 * eventual target lacks a resource. Such feasibility decisions belong to
 * later compilation/resource analysis.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing canonical syntax remains:
 *
 *     ensures(condition);
 *
 * The source-language owner remains:
 *
 *     grammar/statements/contract.g4
 *
 * This file does not introduce a new spelling.
 *
 * It does not alter:
 *
 *     function contract blocks
 *     standalone contract statements
 *     distributed contract declarations
 *
 * Function contracts remain owned by:
 *
 *     grammar/functions/contracts.g4
 *
 * Distributed contract structures remain owned by:
 *
 *     grammar/distributed/contracts.g4
 *
 * If a historical spelling must be supported, that compatibility behavior
 * must be implemented through grammar/compatibility/ rather than silently
 * added here.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * There is no grammar-defined maximum for:
 *
 *     number of ensures statements
 *     condition size
 *     source-unit size
 *     program size
 *     number of declarations
 *     number of functions
 *     number of modules
 *     number of quantum operations
 *     number of qubits
 *     number of processors
 *     number of devices
 *     number of nodes
 *     tensor rank
 *     network size
 *
 * This file deliberately contains no artificial finite capacity ceiling.
 *
 * "Infinity given available resources" means:
 *
 *     the language grammar imposes no artificial machine-size ceiling.
 *
 * It does not claim that an implementation has infinite memory, time,
 * storage, bandwidth, compiler stack depth, or physical resources.
 *
 * Actual limits are implementation/resource characteristics and must not be
 * turned into language constants.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Given identical:
 *
 *     source text
 *     lexer configuration
 *     parser configuration
 *     grammar version
 *
 * parsing must produce equivalent parser structure and source spans.
 *
 * Parsing must not depend on:
 *
 *     hardware availability
 *     QPU availability
 *     GPU availability
 *     network state
 *     filesystem state
 *     wall-clock time
 *     randomness
 *     runtime state
 *     scheduler state
 *     target selection
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * `ensures` participates in POCO-REAF by expressing semantic intent rather
 * than target-specific realization.
 *
 * The same source condition can therefore accompany computation realized on:
 *
 *     tiny embedded systems
 *     CPUs
 *     multicore CPUs
 *     GPUs
 *     FPGAs
 *     ASICs
 *     accelerators
 *     QPUs
 *     simulators
 *     HPC systems
 *     clusters
 *     distributed systems
 *     cloud systems
 *     future computational targets
 *
 * The grammar does not change merely because the target changes.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no:
 *
 *     MAX_ENSURES
 *     MAX_CONTRACTS
 *     MAX_EXPRESSION_DEPTH
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
 * It contains no finite enumeration of:
 *
 *     hardware devices
 *     quantum processors
 *     accelerator models
 *     vendors
 *     machines
 *     node counts
 *     resource capacities
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * Parsing an ensures statement must never:
 *
 *     execute the condition;
 *     invoke a capability;
 *     query hardware;
 *     access the filesystem;
 *     access the network;
 *     invoke a vendor API;
 *     perform quantum measurement;
 *     allocate resources;
 *     execute foreign code.
 *
 * All such operations belong to explicitly controlled downstream layers.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains no Rust implementation code.
 *
 * Generated parser integration must:
 *
 *     compile with Rust 1.97;
 *     compile with Rust 1.97.1;
 *     use Edition 2021;
 *     use safe Rust;
 *     require no unsafe blocks;
 *     require no unsafe functions;
 *     require no unsafe traits;
 *     preserve parser source spans;
 *     preserve deterministic behavior.
 *
 * No unsafe Rust is required by this grammar.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE TESTS
 * --------------
 *
 *     ensures(result >= 0);
 *
 *     ensures(output.is_valid());
 *
 *     ensures(capability("quantum.measurement"));
 *
 *     ensures(capability("tensor.compute"));
 *
 *     ensures(memory_used <= allowed_memory);
 *
 *     ensures(qubits_used <= required_qubits);
 *
 *     ensures(model.is_valid());
 *
 *     ensures(decision.has_evidence());
 *
 *     ensures(signal.is_stable());
 *
 *     ensures(distributed_state.is_consistent());
 *
 *     ensures(
 *         result.is_valid()
 *     );
 *
 *     ensures(
 *         classical_result.is_valid()
 *         and provenance.is_complete()
 *     );
 *
 * The exact acceptance of identifiers and expressions is inherited from the
 * canonical expression grammar.
 *
 * NEGATIVE TESTS
 * --------------
 *
 *     ensures;
 *
 *     ensures();
 *
 *     ensures(;
 *
 *     ensures(, result);
 *
 *     ensures(result;
 *
 *     ensures(result,);
 *
 *     ensures(result) trailing;
 *
 *     ensures(result) extra;
 *
 *     ensures(result) ensures(other);
 *
 * These must not be accepted as one complete validation unit.
 *
 * BOUNDARY TESTS
 * --------------
 *
 * Test:
 *
 *     one ensures statement
 *     many ensures statements in a complete source unit
 *     deeply nested expressions
 *     large expressions
 *     qualified names
 *     Unicode identifiers supported by the canonical lexer
 *     generic values
 *     quantum values
 *     hardware values
 *     distributed values
 *     AI/data values
 *     capability expressions
 *     resource expressions
 *     provenance expressions
 *
 * The validation grammar must not impose a test-derived language limit.
 *
 * SCALABILITY TESTS
 * -----------------
 *
 * Conformance tests should progressively increase:
 *
 *     expression size
 *     nesting
 *     source-unit size
 *     number of contract statements
 *     number of declarations
 *     number of modules
 *     cross-domain complexity
 *
 * Practical compiler/parser limits must be reported as implementation
 * characteristics, not encoded into this grammar.
 *
 * CROSS-DOMAIN TESTS
 * ------------------
 *
 * Validate ensures conditions around:
 *
 *     classical computation
 *     quantum computation
 *     hybrid computation
 *     HDL
 *     hardware intent
 *     AI/model computation
 *     data processing
 *     distributed computation
 *     networking
 *     accelerator computation
 *     simulation
 *
 * DETERMINISM TESTS
 * -----------------
 *
 * Reparse identical input under identical configuration and verify equivalent
 * parser structure and source spans.
 *
 * PORTABILITY TESTS
 * -----------------
 *
 * Keep the source unchanged while varying:
 *
 *     available memory
 *     processor count
 *     GPU availability
 *     FPGA availability
 *     QPU availability
 *     node count
 *     target topology
 *     target vendor
 *     target architecture
 *
 * The grammar must remain unchanged.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * 1. CANONICAL SOURCE OWNER
 * -------------------------
 *
 * No change is required to:
 *
 *     grammar/statements/contract.g4
 *
 * because it already owns:
 *
 *     ensuresStatement
 *
 * 2. LEXER
 * --------
 *
 * No lexer change is required.
 *
 * `ENSURES` remains owned by the canonical lexical hierarchy consumed through:
 *
 *     ZamaniLexer
 *
 * 3. EXPRESSIONS
 * --------------
 *
 * No expression rule is copied here.
 *
 * `contractCondition` remains owned by:
 *
 *     ContractStatements
 *
 * and expression semantics remain owned by the expression subsystem.
 *
 * 4. PUNCTUATION
 * -------------
 *
 * No punctuation rule is copied here.
 *
 * Statement termination remains owned by:
 *
 *     grammar/core/punctuation.g4
 *
 * through ContractStatements.
 *
 * 5. PRODUCTION PARSER
 * --------------------
 *
 * The production parser continues to reach ensures through:
 *
 *     statements
 *       ->
 *     contractStatement
 *       ->
 *     ensuresStatement
 *
 * This validation facade does not replace that path.
 *
 * 6. VALIDATION TOOLING
 * --------------------
 *
 * Validation tooling may invoke:
 *
 *     ensuresValidationUnit
 *
 * when it needs to parse exactly one complete ensures statement.
 *
 * 7. FUNCTION CONTRACTS
 * ---------------------
 *
 * Function contract syntax remains owned by:
 *
 *     grammar/functions/contracts.g4
 *
 * This file must not import function declaration syntax merely to validate an
 * isolated ensures statement.
 *
 * 8. DISTRIBUTED CONTRACTS
 * ------------------------
 *
 * Distributed contract syntax remains owned by:
 *
 *     grammar/distributed/contracts.g4
 *
 * It may consume the same semantic contract model without importing this
 * validation facade.
 *
 * 9. RESOURCES
 * ------------
 *
 * Resource and capability semantics remain owned by:
 *
 *     grammar/resources/
 *
 * This grammar does not duplicate resource syntax.
 *
 * 10. EFFECTS
 * ----------
 *
 * Effect analysis remains owned by:
 *
 *     grammar/effects/
 *
 * 11. POLICIES
 * -----------
 *
 * Policy syntax and semantics remain owned by the policy subsystem.
 *
 * 12. PROVENANCE
 * -------------
 *
 * Provenance remains a downstream semantic/tooling responsibility.
 *
 * 13. IR
 * -----
 *
 * No IR is produced here.
 *
 * 14. QUANTUM
 * ----------
 *
 * Quantum semantics continue toward:
 *
 *     quantum::ir
 *
 * with no additional quantum IR introduced by this file.
 *
 * 15. HDL / HARDWARE
 * ------------------
 *
 * Hardware semantics remain target-independent until semantic lowering and
 * realization.
 *
 * ============================================================================
 * ANTLR COMPOSITION CONTRACT
 * ============================================================================
 *
 * This grammar imports the canonical parser grammar:
 *
 *     ContractStatements
 *
 * The import MUST use the grammar name, not a filesystem path.
 *
 * The public lexer boundary remains:
 *
 *     ZamaniLexer
 *
 * The expected composition is:
 *
 *     parser grammar Ensures;
 *
 *     options {
 *         tokenVocab = ZamaniLexer;
 *     }
 *
 *     import ContractStatements;
 *
 * The validation facade intentionally contains no lexer rules.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 * [ ] The file is named `ensures.g4`.
 *
 * [ ] The grammar name is `Ensures`.
 *
 * [ ] `tokenVocab = ZamaniLexer` is used.
 *
 * [ ] `ContractStatements` is imported.
 *
 * [ ] `ensuresStatement` has exactly one canonical owner.
 *
 * [ ] This file does not redefine `ensuresStatement`.
 *
 * [ ] `contractCondition` is not duplicated.
 *
 * [ ] `expression` is not duplicated.
 *
 * [ ] `statementTerminator` is not duplicated.
 *
 * [ ] No lexer rules are defined.
 *
 * [ ] No semantic predicates are used.
 *
 * [ ] No embedded Rust is used.
 *
 * [ ] No runtime execution occurs.
 *
 * [ ] No hardware/resource inspection occurs.
 *
 * [ ] No machine-capacity constants are encoded.
 *
 * [ ] No target selection is encoded.
 *
 * [ ] No quantum operation catalogue is encoded.
 *
 * [ ] No quantum IR is introduced.
 *
 * [ ] `ensuresValidationUnit` requires EOF.
 *
 * [ ] Trailing source cannot be silently accepted by the isolated validator.
 *
 * [ ] Production parsing continues through ContractStatements.
 *
 * [ ] Function contracts remain independently owned.
 *
 * [ ] Distributed contracts remain independently owned.
 *
 * [ ] AST mapping remains canonical.
 *
 * [ ] Semantic mapping remains canonical.
 *
 * [ ] Effects remain canonical.
 *
 * [ ] Capabilities remain canonical.
 *
 * [ ] Resources remain canonical.
 *
 * [ ] Policies remain canonical.
 *
 * [ ] Provenance remains canonical.
 *
 * [ ] Positive tests exist.
 *
 * [ ] Negative tests exist.
 *
 * [ ] Boundary tests exist.
 *
 * [ ] Scalability tests exist.
 *
 * [ ] Cross-domain tests exist.
 *
 * [ ] Determinism tests exist.
 *
 * [ ] Portability tests exist.
 *
 * [ ] Rust 1.97 generation succeeds.
 *
 * [ ] Rust 1.97.1 generation succeeds.
 *
 * [ ] No unsafe Rust is required.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL RULE
 * ============================================================================
 *
 * `ensures` expresses a semantic postcondition.
 *
 * It is not:
 *
 *     resource allocation
 *     hardware selection
 *     quantum routing
 *     scheduling
 *     backend selection
 *     runtime authorization
 *     proof execution
 *     physical realization
 *
 * Therefore the validation architecture remains:
 *
 *     ensures source
 *         ->
 *     canonical contract parser
 *         ->
 *     domain-neutral AST
 *         ->
 *     structural validation
 *         ->
 *     semantic contract model
 *         ->
 *     type/effect/capability/resource/policy analysis
 *         ->
 *     provenance
 *         ->
 *     canonical IR/domain IR
 *         ->
 *     target-independent optimization
 *         ->
 *     target realization
 *
 * This separation preserves:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * ============================================================================
 * GRAMMAR
 * ============================================================================
 */

parser grammar Ensures;

options {
    tokenVocab = ZamaniLexer;
}

import
    ContractStatements
    ;


/*
 * ============================================================================
 * COMPLETE VALIDATION UNIT
 * ============================================================================
 *
 * This is the public entry point for isolated ensures validation.
 *
 * EOF is mandatory.
 *
 * Therefore:
 *
 *     ensures(condition);
 *
 * succeeds, while:
 *
 *     ensures(condition); trailing
 *
 * cannot be accepted as a complete validation unit.
 */
ensuresValidationUnit
    : ensuresValidationItem EOF
    ;


/*
 * ============================================================================
 * VALIDATION ITEM
 * ============================================================================
 *
 * This wrapper deliberately delegates to the canonical source-language rule.
 *
 * It does NOT reproduce:
 *
 *     ENSURES
 *     contractCondition
 *     statementTerminator
 *
 * Those remain owned by ContractStatements.
 */
ensuresValidationItem
    : ensuresStatement
    ;
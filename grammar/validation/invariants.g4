/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/validation/invariants.g4
 *
 * GRAMMAR
 * -------
 * InvariantValidation
 *
 * STATUS
 * ------
 * Production validation / conformance component
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file provides the validation and conformance entry point for Zamani's
 * canonical `invariant(...)` statement.
 *
 * IMPORTANT:
 *
 * This file is NOT a second owner of invariant syntax.
 *
 * The canonical source-level invariant syntax is owned by:
 *
 *     grammar/statements/contract.g4
 *
 * through:
 *
 *     ContractStatements
 *         |
 *         +--> invariantStatement
 *         |
 *         +--> contractCondition
 *                 |
 *                 +--> expression
 *
 * This file only exposes a stable validation boundary around that canonical
 * rule.
 *
 * ============================================================================
 * ARCHITECTURAL PRINCIPLE
 * ============================================================================
 *
 * Validation observes and verifies language constructs.
 *
 * Validation MUST NOT become another source-language authority.
 *
 * Consequently this file:
 *
 *     - does not redefine `invariant`;
 *     - does not redefine parentheses;
 *     - does not redefine expressions;
 *     - does not define identifiers;
 *     - does not define qualified names;
 *     - does not define operators;
 *     - does not define literals;
 *     - does not define resource expressions;
 *     - does not define capability syntax;
 *     - does not define policies;
 *     - does not define effects;
 *     - does not define quantum syntax;
 *     - does not define HDL syntax;
 *     - does not define hardware syntax;
 *     - does not define AI syntax;
 *     - does not define distributed syntax;
 *     - does not define an AST;
 *     - does not define semantic evaluation;
 *     - does not define an IR;
 *     - does not select a target;
 *     - does not inspect hardware;
 *     - does not allocate resources.
 *
 * ============================================================================
 * IMPLEMENTATION BASELINE
 * ============================================================================
 *
 * ANTLR:
 *
 *     ANTLR4 parser grammar
 *
 * Rust integration:
 *
 *     Rust 1.97 / Rust 1.97.1
 *     Rust Edition 2021
 *     Safe Rust only
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
 *     - no physical topology;
 *     - no quantum-device knowledge.
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * Provide an isolated parser entry point for validating canonical invariant
 * statements without duplicating their language definition.
 *
 * OWNS
 * ----
 *
 *     invariantValidationUnit
 *     invariantValidationItem
 *     invariantValidationStatement
 *
 * These are validation facade rules.
 *
 * DOES NOT OWN
 * -------------
 *
 *     invariantStatement
 *     contractCondition
 *     expression
 *     statementTerminator
 *     INVARIANT
 *     LPAREN
 *     RPAREN
 *
 * Those remain owned by the canonical grammar components.
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
 * Indirectly:
 *
 *     grammar/expressions/
 *     grammar/core/punctuation.g4
 *     grammar/lexer/keywords.g4
 *     grammar/antlr/ZamaniLexer.g4
 *
 * EXPORTS
 * -------
 *
 *     invariantValidationUnit
 *     invariantValidationItem
 *     invariantValidationStatement
 *
 * CONSUMED_BY
 * -----------
 *
 *     grammar/tests/
 *     grammar/validation/
 *     grammar validation tooling
 *     isolated conformance tooling
 *
 * This grammar is NOT intended to replace the production program parser.
 *
 * AST_OWNER
 * ---------
 *
 *     Existing domain-neutral frontend AST subsystem
 *
 * SEMANTIC_OWNER
 * --------------
 *
 *     Existing semantic contract / validation subsystem
 *
 * IR_OWNER
 * --------
 *
 *     Existing canonical semantic representation
 *
 * Quantum computations eventually cross:
 *
 *     quantum semantic model
 *         ->
 *     quantum::ir
 *
 * This grammar does not create or own that IR.
 *
 * TEST_OWNER
 * ----------
 *
 *     grammar/tests/contracts/
 *     grammar/tests/validation/
 *     grammar/tests/negative/
 *     grammar/tests/boundary/
 *     grammar/tests/scalability/
 *
 * SPEC_OWNER
 * ----------
 *
 *     grammar/spec/contracts.md
 *     grammar/specification/contracts.md
 *     grammar/specification/poco-reaf.md
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The canonical lexer remains:
 *
 *     ZamaniLexer
 *
 * This file does not define:
 *
 *     INVARIANT
 *     LPAREN
 *     RPAREN
 *     SEMICOLON
 *
 * The lexical architecture remains the sole authority for token identity.
 *
 * In particular, this grammar MUST NOT introduce:
 *
 *     INVARIANT_KEYWORD
 *     INVARIANT_TOKEN
 *     LEFT_PAREN_INVARIANT
 *     RIGHT_PAREN_INVARIANT
 *
 * or any equivalent duplicate tokens.
 *
 * ============================================================================
 * CANONICAL SOURCE SYNTAX
 * ============================================================================
 *
 * The canonical source form is:
 *
 *     invariant(condition);
 *
 * The parentheses and expression are owned by:
 *
 *     grammar/statements/contract.g4
 *
 * through:
 *
 *     invariantStatement
 *         : INVARIANT contractCondition statementTerminator
 *         ;
 *
 * and:
 *
 *     contractCondition
 *         : LPAREN expression RPAREN
 *         ;
 *
 * Therefore this file deliberately does not reproduce that syntax.
 *
 * ============================================================================
 * VALIDATION BOUNDARY
 * ============================================================================
 *
 * The validation path is:
 *
 *     invariantValidationUnit
 *                 |
 *                 v
 *     invariantValidationItem
 *                 |
 *                 v
 *     invariantValidationStatement
 *                 |
 *                 v
 *     invariantStatement
 *                 |
 *                 v
 *     contractCondition
 *                 |
 *                 v
 *             expression
 *
 * This provides a single canonical syntax owner.
 *
 * ============================================================================
 * PRODUCTION PARSER BOUNDARY
 * ============================================================================
 *
 * The production Zamani parser does NOT need to enter this grammar in order
 * to parse ordinary programs.
 *
 * The normal production path remains:
 *
 *     grammar/Zamani.g4
 *         ->
 *     parser composition
 *         ->
 *     statements
 *         ->
 *     ContractStatements
 *         ->
 *     invariantStatement
 *
 * This file is an isolated validation/conformance facade.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar creates parser contexts only.
 *
 * It does not create Rust AST structures.
 *
 * The downstream AST must preserve enough information to represent:
 *
 *     contract kind = invariant
 *     condition expression
 *     source span
 *     enclosing scope
 *     source order
 *     provenance
 *
 * Conceptually:
 *
 *     ContractObligation
 *     {
 *         kind: Invariant,
 *         condition,
 *         source_span,
 *         enclosing_scope,
 *         provenance
 *     }
 *
 * The actual Rust representation belongs to the existing AST subsystem.
 *
 * This grammar MUST NOT create:
 *
 *     QuantumInvariant
 *     HardwareInvariant
 *     HDLInvariant
 *     AIInvariant
 *     GPUInvariant
 *     QPUInvariant
 *     DistributedInvariant
 *
 * Invariant is a universal semantic category.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * An invariant states a condition that must remain valid according to the
 * semantic context in which the invariant occurs.
 *
 * The grammar does not determine:
 *
 *     - when the invariant is checked;
 *     - how often it is checked;
 *     - whether it is statically proven;
 *     - whether it is dynamically checked;
 *     - whether it is discharged by a solver;
 *     - whether it becomes a runtime assertion;
 *     - whether it is checked at compilation;
 *     - whether it is checked at execution;
 *     - whether it is preserved through optimization.
 *
 * Those decisions belong to semantic analysis, verification, optimization,
 * lowering, and runtime systems.
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * The invariant condition remains an ordinary Zamani expression.
 *
 * This grammar does not introduce:
 *
 *     InvariantType
 *     ContractBoolean
 *     QuantumInvariantType
 *     HardwareInvariantType
 *
 * The semantic/type system determines whether the expression is valid as an
 * invariant for its enclosing context.
 *
 * Examples:
 *
 *     invariant(x >= 0);
 *
 *     invariant(state.is_valid());
 *
 *     invariant(result.shape == expected_shape);
 *
 *     invariant(model.is_consistent());
 *
 *     invariant(hardware_state.is_consistent());
 *
 * The expression grammar remains the single expression authority.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Parsing an invariant does not evaluate its condition.
 *
 * Consequently this grammar introduces no runtime effect.
 *
 * If semantic analysis determines that an invariant expression refers to
 * operations involving:
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
 * those effects are handled by the existing effect subsystem.
 *
 * This grammar does not create a second effect system.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * An invariant condition may refer semantically to capabilities.
 *
 * Example:
 *
 *     invariant(capability("quantum.measurement"));
 *
 * The grammar only validates the surrounding source structure.
 *
 * Capability resolution belongs downstream:
 *
 *     parser
 *       ->
 *     AST
 *       ->
 *     semantic analysis
 *       ->
 *     capability analysis
 *
 * The grammar MUST NOT inspect the target environment.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Invariant conditions may refer to resource state or resource properties.
 *
 * Examples:
 *
 *     invariant(memory_available >= required_memory);
 *
 *     invariant(qubits_available >= logical_qubits);
 *
 *     invariant(topology.supports(required_topology));
 *
 * These are ordinary semantic expressions.
 *
 * This file MUST NOT establish any universal capacity.
 *
 * In particular, it contains no limits for:
 *
 *     qubits
 *     logical qubits
 *     physical qubits
 *     CPUs
 *     cores
 *     threads
 *     GPUs
 *     FPGAs
 *     ASIC resources
 *     accelerators
 *     QPUs
 *     nodes
 *     devices
 *     memory
 *     storage
 *     tensor rank
 *     register width
 *     network size
 *
 * ============================================================================
 * CONTRACT INTEGRATION
 * ============================================================================
 *
 * `invariant` is one member of the canonical contract family:
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
 * This file must remain aligned with that canonical family by delegation,
 * not duplication.
 *
 * The semantic model may normalize all six forms into a common contract
 * representation while retaining their distinct kinds.
 *
 * ============================================================================
 * REQUIREMENT INTEGRATION
 * ============================================================================
 *
 * An invariant condition may contain semantic requirements.
 *
 * Example:
 *
 *     invariant(
 *         capability("quantum.measurement")
 *     );
 *
 * However, this grammar does not import or duplicate either requirement
 * grammar.
 *
 * The repository currently separates:
 *
 *     grammar/core/requirements.g4
 *
 * from:
 *
 *     grammar/resources/requirements.g4
 *
 * and those grammars own different requirement categories.
 *
 * An invariant condition remains an ordinary expression and therefore avoids
 * creating a third requirement owner.
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Policies may determine:
 *
 *     - whether an invariant is mandatory;
 *     - whether it must be statically discharged;
 *     - whether runtime checking is permitted;
 *     - whether an assumption is trusted;
 *     - whether verification evidence is required;
 *     - whether execution may continue after violation.
 *
 * Policy semantics are downstream.
 *
 * This file does not define or evaluate policy syntax.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * An invariant must remain traceable through the frontend.
 *
 * At minimum, the semantic representation must preserve:
 *
 *     source file
 *     source span
 *     contract kind
 *     enclosing scope
 *     condition
 *     source order
 *
 * Later stages may additionally attach:
 *
 *     derived_from
 *     generated_by
 *     transformed_by
 *     verified_by
 *     evidence
 *     decision
 *     version
 *
 * This grammar does not store provenance itself.
 *
 * ============================================================================
 * EVIDENCE / VERIFICATION CONTRACT
 * ============================================================================
 *
 * An invariant may become a verification obligation.
 *
 * Possible downstream outcomes include:
 *
 *     proven
 *     discharged
 *     runtime_checked
 *     partially_verified
 *     unverifiable
 *     violated
 *     unsupported
 *
 * These are semantic/verification results, not parser states.
 *
 * The grammar MUST NOT encode a finite verification technology.
 *
 * It must remain usable with:
 *
 *     symbolic reasoning
 *     theorem proving
 *     model checking
 *     static analysis
 *     runtime checking
 *     simulation
 *     hardware verification
 *     quantum verification
 *     distributed verification
 *     future verification systems
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar produces NO IR.
 *
 * The invariant follows the normal semantic pipeline:
 *
 *     source
 *       |
 *       v
 *     lexer
 *       |
 *       v
 *     parser
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     structural validation
 *       |
 *       v
 *     semantic invariant model
 *       |
 *       +--> type analysis
 *       +--> effect analysis
 *       +--> capability analysis
 *       +--> resource analysis
 *       +--> policy analysis
 *       +--> provenance
 *       +--> verification
 *       |
 *       v
 *     canonical semantic representation
 *       |
 *       +--> classical representation
 *       |
 *       +--> quantum semantic representation
 *       |        |
 *       |        v
 *       |     quantum::ir
 *       |
 *       +--> HDL/hardware representation
 *       |
 *       +--> distributed representation
 *       |
 *       +--> accelerator representation
 *       |
 *       +--> future domain representations
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     lowering
 *       |
 *       v
 *     routing / scheduling where applicable
 *       |
 *       v
 *     resilience / execution
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Quantum invariants use the same language construct.
 *
 * Examples:
 *
 *     invariant(logical_state.is_valid());
 *
 *     invariant(measurement.is_valid());
 *
 *     invariant(capability("quantum.dynamic_control"));
 *
 * The grammar does not define quantum-specific invariant syntax.
 *
 * Quantum semantic analysis remains responsible for determining what the
 * invariant means in the quantum context.
 *
 * Where quantum computation is involved, the canonical boundary remains:
 *
 *     quantum semantic model
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     decomposition
 *          |
 *          v
 *     routing
 *          |
 *          v
 *     scheduling
 *          |
 *          v
 *     QEC / resilience
 *          |
 *          v
 *     ZQN
 *          |
 *          v
 *     HAL
 *          |
 *          v
 *     target realization
 *
 * This file MUST NOT:
 *
 *     - enumerate quantum operations;
 *     - enumerate physical qubits;
 *     - encode coupling maps;
 *     - encode calibration;
 *     - perform routing;
 *     - perform scheduling;
 *     - implement QEC;
 *     - implement ZQN;
 *     - select a QPU;
 *     - create another quantum IR.
 *
 * ============================================================================
 * HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * HDL and hardware invariants remain ordinary invariant expressions.
 *
 * Examples:
 *
 *     invariant(signal.is_defined(clock));
 *
 *     invariant(output.is_valid());
 *
 *     invariant(hardware_state.is_consistent());
 *
 * The grammar does not encode:
 *
 *     fixed signal widths
 *     fixed register widths
 *     fixed memory capacities
 *     fixed FPGA dimensions
 *     fixed ASIC resources
 *     fixed clock counts
 *     vendor device identities
 *     physical topology
 *
 * Hardware semantic analysis and realization remain downstream.
 *
 * ============================================================================
 * CLASSICAL BOUNDARY
 * ============================================================================
 *
 * Classical computation uses the same invariant syntax.
 *
 * Example:
 *
 *     invariant(accumulator >= 0);
 *
 * There is no separate classical invariant grammar.
 *
 * ============================================================================
 * HYBRID BOUNDARY
 * ============================================================================
 *
 * Hybrid computation uses the same invariant construct across:
 *
 *     classical state
 *     quantum state
 *     measurement results
 *     accelerator state
 *     host/device state
 *
 * Example:
 *
 *     invariant(measurement.is_valid());
 *
 * The grammar remains domain-neutral.
 *
 * ============================================================================
 * AI / KNOWLEDGE BOUNDARY
 * ============================================================================
 *
 * Invariants may constrain:
 *
 *     model state
 *     learned state
 *     confidence
 *     evidence
 *     knowledge consistency
 *     decision consistency
 *     provenance
 *     adaptation state
 *
 * Examples:
 *
 *     invariant(model.is_consistent());
 *
 *     invariant(confidence >= required_confidence);
 *
 *     invariant(decision.has_evidence());
 *
 * The grammar does not create AI-specific invariant syntax.
 *
 * ============================================================================
 * DISTRIBUTED BOUNDARY
 * ============================================================================
 *
 * Distributed invariants may describe:
 *
 *     state consistency
 *     protocol properties
 *     communication properties
 *     replication properties
 *     topology properties
 *     actor state
 *     fault-state conditions
 *
 * Example:
 *
 *     invariant(cluster_state.is_consistent());
 *
 * The grammar does not impose a maximum number of nodes or actors.
 *
 * ============================================================================
 * BACKEND CONTRACT
 * ============================================================================
 *
 * A backend may consume invariant semantics to:
 *
 *     - prove the invariant;
 *     - preserve it through lowering;
 *     - generate a runtime check;
 *     - generate verification logic;
 *     - reject an incompatible realization;
 *     - attach verification evidence;
 *     - report a violation.
 *
 * Backend implementation is outside this grammar.
 *
 * A backend MUST NOT reinterpret a valid invariant as permission to silently
 * change the program's meaning.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Invariants are source-level semantics.
 *
 * They must remain independent of the target realization.
 *
 * Therefore the same invariant syntax may participate in programs targeting:
 *
 *     tiny systems
 *     embedded systems
 *     CPUs
 *     multicore systems
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
 *     future computing systems
 *
 * A target that cannot satisfy or verify an invariant must report that fact
 * explicitly.
 *
 * It must not silently weaken or remove the invariant.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar imposes no finite language-level limit on:
 *
 *     number of invariant statements
 *     expression size
 *     expression depth
 *     module count
 *     function count
 *     program size
 *     quantum operations
 *     qubits
 *     processors
 *     threads
 *     GPUs
 *     FPGAs
 *     accelerators
 *     QPUs
 *     nodes
 *     devices
 *     tensor rank
 *     network size
 *
 * "Infinity" means:
 *
 *     no artificial finite capacity ceiling is encoded by this grammar.
 *
 * It does NOT mean that physical memory, execution time, parser stack depth,
 * target capacity, or compiler resources are mathematically infinite.
 *
 * Practical limits remain implementation/resource characteristics.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file MUST NOT contain:
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
 * It also MUST NOT encode equivalent limits through:
 *
 *     fixed alternatives;
 *     fixed array sizes;
 *     finite hardware enumerations;
 *     fixed topology lists;
 *     fixed device identifiers;
 *     fixed vendor lists;
 *     fixed quantum-operation catalogs.
 *
 * Numeric values appearing inside an invariant are program values.
 *
 * For example:
 *
 *     invariant(qubits_available >= 1024);
 *
 * does NOT make 1024 a language-level maximum.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing through this grammar depends only on:
 *
 *     source text
 *     lexer vocabulary
 *     parser grammar
 *     parser configuration
 *
 * It MUST NOT depend on:
 *
 *     hardware availability
 *     target selection
 *     network state
 *     filesystem state
 *     wall-clock time
 *     randomness
 *     scheduler state
 *     runtime state.
 *
 * Identical input and parser configuration must produce equivalent parser
 * structure.
 *
 * ============================================================================
 * SAFETY CONTRACT
 * ============================================================================
 *
 * This grammar contains no executable implementation.
 *
 * It cannot:
 *
 *     execute invariant expressions;
 *     inspect hardware;
 *     access files;
 *     access networks;
 *     allocate resources;
 *     invoke foreign functions;
 *     invoke native functions.
 *
 * Generated Zamani frontend code must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and Safe Rust.
 *
 * No `unsafe` Rust is required or authorized by this grammar.
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Syntactic diagnostics belong to the parser.
 *
 * Examples of malformed source:
 *
 *     invariant;
 *     invariant();
 *     invariant(;
 *     invariant();
 *     invariant(, condition);
 *     invariant(condition;
 *     invariant(condition,);
 *     invariant(condition, other);
 *     invariant(condition) trailing;
 *
 * The canonical statement grammar is responsible for producing the actual
 * syntax diagnostics because this file delegates to `invariantStatement`.
 *
 * Semantic diagnostics belong downstream.
 *
 * Examples:
 *
 *     invalid condition type
 *     unknown name
 *     invalid invariant context
 *     unsupported verification obligation
 *     unavailable capability
 *     unavailable resource
 *     policy violation
 *     invalid provenance
 *     invariant cannot be established
 *
 * A structurally valid invariant MUST NOT be rejected merely because a target
 * lacks sufficient resources. Such a failure belongs to semantic/resource
 * analysis.
 *
 * ============================================================================
 * POSITIVE TEST CONTRACT
 * ============================================================================
 *
 * Basic:
 *
 *     invariant(x >= 0);
 *
 *     invariant(state.is_valid());
 *
 *     invariant(result.is_consistent());
 *
 * Resource-aware:
 *
 *     invariant(memory_available >= required_memory);
 *
 *     invariant(qubits_available >= logical_qubits);
 *
 * Capability-aware:
 *
 *     invariant(capability("quantum.measurement"));
 *
 *     invariant(capability("tensor.compute"));
 *
 * Quantum:
 *
 *     invariant(measurement.is_valid());
 *
 *     invariant(logical_state.is_valid());
 *
 * HDL / hardware:
 *
 *     invariant(signal.is_defined(clock));
 *
 *     invariant(output.is_valid());
 *
 * AI / knowledge:
 *
 *     invariant(model.is_consistent());
 *
 *     invariant(decision.has_evidence());
 *
 * Distributed:
 *
 *     invariant(cluster_state.is_consistent());
 *
 * Nested expressions:
 *
 *     invariant(
 *         state.is_valid()
 *         and
 *         result.is_consistent()
 *     );
 *
 * Multiple validation items:
 *
 *     invariant(a);
 *     invariant(b);
 *     invariant(c);
 *
 * ============================================================================
 * NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * These must fail through the canonical invariant syntax:
 *
 *     invariant;
 *
 *     invariant();
 *
 *     invariant(;
 *
 *     invariant(, x);
 *
 *     invariant(x;
 *
 *     invariant(x,);
 *
 *     invariant(x, y);
 *
 *     invariant(x) trailing;
 *
 *     invariant);
 *
 *     invariant(());
 *
 * The exact diagnostic code is owned by the parser/diagnostic subsystem.
 *
 * ============================================================================
 * BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Test invariants containing:
 *
 *     - nested boolean expressions;
 *     - nested function calls;
 *     - qualified names;
 *     - capability expressions;
 *     - resource expressions;
 *     - quantum-derived values;
 *     - measurement values;
 *     - HDL state;
 *     - hardware state;
 *     - distributed state;
 *     - AI/model state;
 *     - provenance expressions;
 *     - policy-derived values;
 *     - dialect-defined values.
 *
 * Also test:
 *
 *     - Unicode identifiers;
 *     - very long symbolic names;
 *     - deeply nested expressions;
 *     - large source files;
 *     - many consecutive invariants;
 *     - cross-domain invariants.
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Conformance tests must demonstrate that no artificial finite capacity is
 * introduced by this validation grammar.
 *
 * Test generators SHOULD vary:
 *
 *     invariant count
 *     expression size
 *     expression depth
 *     qualified-name depth
 *     source-unit size
 *     cross-domain composition
 *
 * Test parameters are test configuration values only.
 *
 * They MUST NOT be interpreted as language-level maxima.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Canonical syntax remains controlled by:
 *
 *     grammar/statements/contract.g4
 *
 * Therefore future changes to invariant syntax must be made at the canonical
 * owner first.
 *
 * This validation facade must then continue to delegate to that owner.
 *
 * Legacy syntax must not be added here as a private compatibility grammar.
 *
 * Legacy compatibility belongs to:
 *
 *     grammar/compatibility/
 *
 * and must be explicitly versioned and tested.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * Canonical source syntax:
 *
 *     grammar/statements/contract.g4
 *
 * Validation facade:
 *
 *     grammar/validation/invariants.g4
 *
 * Contract family:
 *
 *     grammar/validation/contracts.g4
 *
 * Expression authority:
 *
 *     grammar/expressions/
 *
 * Lexer authority:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/lexer/
 *
 * AST:
 *
 *     existing frontend/domain-neutral AST
 *
 * Semantic validation:
 *
 *     existing contract/semantic validation subsystem
 *
 * Resource semantics:
 *
 *     grammar/resources/
 *
 * Capability semantics:
 *
 *     grammar/core/capabilities.g4
 *
 * Effects:
 *
 *     grammar/effects/
 *
 * Policies:
 *
 *     grammar/core/policies.g4
 *     grammar/expressions/policy.g4
 *
 * Quantum:
 *
 *     grammar/quantum/
 *     quantum::ir
 *
 * HDL:
 *
 *     grammar/hdl/
 *
 * Hardware:
 *
 *     grammar/hardware/
 *
 * AI:
 *
 *     grammar/ai/
 *
 * Distributed:
 *
 *     grammar/distributed/
 *
 * Tests:
 *
 *     grammar/tests/
 *
 * ============================================================================
 * IMPORTANT INTEGRATION RULE
 * ============================================================================
 *
 * Do NOT modify:
 *
 *     grammar/statements/contract.g4
 *
 * merely because this file exists.
 *
 * `invariantStatement` already provides the canonical syntax.
 *
 * Do NOT modify:
 *
 *     grammar/core/requirements.g4
 *     grammar/resources/requirements.g4
 *
 * merely to support invariant conditions.
 *
 * Invariant conditions already use the universal expression boundary.
 *
 * Do NOT create:
 *
 *     validation/invariant-expression.g4
 *
 * unless the expression architecture itself is deliberately redesigned.
 *
 * Such a file would otherwise create unnecessary duplication.
 *
 * ============================================================================
 * ROOT-GRAMMAR INTEGRATION
 * ============================================================================
 *
 * The production root remains responsible for composing the language.
 *
 * This validation grammar does not need to be inserted into the normal
 * production source path merely to make `invariant` legal.
 *
 * `invariant` is already reachable through:
 *
 *     Zamani.g4
 *         ->
 *     statement composition
 *         ->
 *     ContractStatements
 *         ->
 *     invariantStatement
 *
 * The validation grammar is an additional isolated conformance entry point.
 *
 * ============================================================================
 * VALIDATION-ORCHESTRATOR INTEGRATION
 * ============================================================================
 *
 * If the repository later introduces a single validation parser orchestrator,
 * it should consume this grammar as:
 *
 *     InvariantValidation
 *         ->
 *     invariantValidationUnit
 *
 * alongside:
 *
 *     ContractValidation
 *         ->
 *     contractValidationUnit
 *
 * The orchestrator MUST NOT copy `invariantStatement`.
 *
 * ============================================================================
 * NO-CROSS-FILE-REEDIT CONTRACT
 * ============================================================================
 *
 * This file's dependency boundary is intentionally stable:
 *
 *     InvariantValidation
 *         |
 *         v
 *     ContractStatements
 *         |
 *         v
 *     expression
 *
 * Changes to:
 *
 *     resources
 *     quantum
 *     HDL
 *     hardware
 *     AI
 *     distributed
 *     networking
 *     backend
 *     runtime
 *
 * do not require this file to be rewritten as long as the canonical
 * `invariantStatement` contract remains stable.
 *
 * If invariant syntax changes, the canonical owner is updated first and this
 * facade automatically consumes the updated rule.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     [x] It has exactly one validation grammar identity.
 *
 *     [x] It imports the canonical ContractStatements grammar.
 *
 *     [x] It does not redefine invariant syntax.
 *
 *     [x] It exposes a stable validation entry point.
 *
 *     [x] It delegates the invariant statement to its canonical owner.
 *
 *     [x] It does not define lexer tokens.
 *
 *     [x] It does not define expressions.
 *
 *     [x] It does not define resource requirements.
 *
 *     [x] It does not define capability requirements.
 *
 *     [x] It does not define policies.
 *
 *     [x] It does not define effects.
 *
 *     [x] It does not define quantum operations.
 *
 *     [x] It does not define HDL operations.
 *
 *     [x] It does not define hardware topology.
 *
 *     [x] It does not define AI-specific syntax.
 *
 *     [x] It does not define distributed syntax.
 *
 *     [x] It does not define an AST.
 *
 *     [x] It does not define an IR.
 *
 *     [x] It does not perform target selection.
 *
 *     [x] It does not encode hardware limits.
 *
 *     [x] It introduces no fixed capacity constants.
 *
 *     [x] It preserves POCO-REAF semantics.
 *
 *     [x] It is deterministic.
 *
 *     [x] It is compatible with Safe Rust integration.
 *
 *     [x] It requires no unsafe Rust.
 *
 *     [x] It has positive tests.
 *
 *     [x] It has negative tests.
 *
 *     [x] It has boundary tests.
 *
 *     [x] It has scalability tests.
 *
 *     [x] It has cross-domain tests.
 *
 *     [x] It has compatibility tests.
 *
 *     [x] Its integration boundary is explicitly documented.
 *
 * ============================================================================
 * GRAMMAR
 * ============================================================================
 */

parser grammar InvariantValidation;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * ============================================================================
 * CANONICAL CONTRACT SYNTAX IMPORT
 * ============================================================================
 *
 * ContractStatements remains the sole owner of standalone contract syntax.
 *
 * This import gives this validation facade access to:
 *
 *     invariantStatement
 *
 * without creating another invariant grammar.
 */
import ContractStatements;


/*
 * ============================================================================
 * VALIDATION UNIT
 * ============================================================================
 *
 * Parses zero or more canonical invariant statements followed by EOF.
 *
 * This is an isolated validation/conformance entry point.
 *
 * It does not replace the normal Zamani program parser.
 */
invariantValidationUnit
    : invariantValidationItem* EOF
    ;


/*
 * ============================================================================
 * VALIDATION ITEM
 * ============================================================================
 *
 * A validation item is exactly one canonical invariant statement.
 *
 * No alternative syntax is permitted here.
 */
invariantValidationItem
    : invariantValidationStatement
    ;


/*
 * ============================================================================
 * VALIDATION STATEMENT
 * ============================================================================
 *
 * Stable validation-level context around the canonical rule.
 *
 * The actual source syntax remains owned by:
 *
 *     ContractStatements.invariantStatement
 */
invariantValidationStatement
    : invariantStatement
    ;
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
 * Invariants
 *
 * STATUS
 * ------
 * Production validation/conformance component
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file provides the validation-facing parser entry point for Zamani
 * invariant constructs.
 *
 * IMPORTANT:
 *
 * This file is NOT a second owner of invariant source syntax.
 *
 * Canonical invariant syntax remains owned by:
 *
 *     grammar/statements/contract.g4
 *         -> ContractStatements
 *         -> invariantStatement
 *
 * This file only exposes that canonical syntax through an isolated validation
 * and conformance entry point.
 *
 * ============================================================================
 * ARCHITECTURAL PRINCIPLE
 * ============================================================================
 *
 * Zamani separates:
 *
 *     source-language syntax
 *     validation/conformance entry points
 *     AST construction
 *     semantic analysis
 *     type analysis
 *     effect analysis
 *     capability analysis
 *     resource analysis
 *     policy analysis
 *     provenance
 *     canonical semantic representation
 *     domain IR
 *     target realization
 *
 * Therefore this grammar MUST remain syntactic.
 *
 * It MUST NOT:
 *
 *     - redefine invariant syntax;
 *     - define a second expression grammar;
 *     - define lexer tokens;
 *     - define resource limits;
 *     - inspect hardware;
 *     - inspect target availability;
 *     - perform capability negotiation;
 *     - evaluate invariant conditions;
 *     - construct Rust AST structures;
 *     - construct semantic models;
 *     - create IR;
 *     - select a backend;
 *     - perform quantum routing;
 *     - perform HDL synthesis;
 *     - perform scheduling;
 *     - execute code;
 *     - contain target-language actions;
 *     - contain semantic predicates;
 *     - contain unsafe code.
 *
 * ============================================================================
 * IMPLEMENTATION BASELINE
 * ============================================================================
 *
 * Language:
 *
 *     Zamani
 *
 * Grammar technology:
 *
 *     ANTLR4 parser grammar
 *
 * Rust implementation:
 *
 *     Rust Edition 2021
 *     Rust 1.97
 *     Rust 1.97.1
 *     Safe Rust only
 *
 * This grammar itself contains no embedded Rust and therefore cannot introduce
 * unsafe behavior into generated parser code.
 *
 * ============================================================================
 * OWNERSHIP CONTRACT
 * ============================================================================
 *
 * OWNS
 * ----
 *
 *     invariantValidationUnit
 *     invariantValidationItem
 *     invariantValidationStatement
 *
 * These rules are validation/conformance entry points only.
 *
 * DOES NOT OWN
 * -------------
 *
 *     invariantStatement
 *     expression
 *     tokens
 *     identifiers
 *     literals
 *     operators
 *     punctuation
 *     contract semantics
 *     type semantics
 *     effect semantics
 *     capability semantics
 *     resource semantics
 *     policy semantics
 *     provenance semantics
 *
 * The canonical invariant statement remains owned by:
 *
 *     grammar/statements/contract.g4
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON
 * ----------
 *
 *     grammar/statements/contract.g4
 *         -> ContractStatements
 *         -> invariantStatement
 *
 * Indirect dependencies are inherited through ContractStatements, including
 * the canonical expression and lexical architecture.
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
 *     grammar/validation/ conformance infrastructure
 *     grammar/tests/
 *     parser-validation tooling
 *     grammar regression tests
 *     validation tooling
 *
 * AST_OWNER
 * ---------
 *
 *     repository AST implementation
 *
 * This grammar does not define AST structures.
 *
 * SEMANTIC_OWNER
 * --------------
 *
 *     semantic analysis / contract validation subsystem
 *
 * IR_OWNER
 * --------
 *
 *     canonical semantic/IR pipeline
 *
 * TEST_OWNER
 * ----------
 *
 *     grammar/tests/
 *     grammar/validation/
 *
 * SPEC_OWNER
 * ----------
 *
 *     grammar/specification/
 *     grammar/spec/
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This file does not define lexical rules.
 *
 * Invariant lexical spelling and token identity are inherited through:
 *
 *     ContractStatements
 *         |
 *         v
 *     ZamaniLexer
 *
 * The invariant keyword MUST therefore have exactly one lexical authority.
 *
 * This file MUST NOT introduce another token such as:
 *
 *     InvariantKeyword
 *     InvariantToken
 *     ValidationInvariant
 *
 * or any equivalent duplicate.
 *
 * ============================================================================
 * PARSER CONTRACT
 * ============================================================================
 *
 * Canonical source syntax:
 *
 *     invariantStatement
 *
 * remains owned by:
 *
 *     grammar/statements/contract.g4
 *
 * This file delegates to that rule.
 *
 * Conceptually:
 *
 *     invariantValidationUnit
 *             |
 *             v
 *     invariantValidationItem
 *             |
 *             v
 *     invariantValidationStatement
 *             |
 *             v
 *     invariantStatement
 *             |
 *             v
 *     canonical Zamani expression
 *
 * No syntax is copied into this file.
 *
 * ============================================================================
 * PUBLIC RULES
 * ============================================================================
 *
 * invariantValidationUnit
 * -----------------------
 *
 * Parses an isolated validation unit containing zero or more canonical
 * invariant statements followed by EOF.
 *
 * This is intended for:
 *
 *     - parser conformance tests;
 *     - validation tests;
 *     - isolated invariant tests;
 *     - regression tests;
 *     - negative tests;
 *     - boundary tests;
 *     - scalability tests;
 *     - source-span tests;
 *     - deterministic parsing tests.
 *
 * invariantValidationItem
 * -----------------------
 *
 * Provides the validation-unit item boundary.
 *
 * It deliberately remains small so future validation orchestration can add
 * independently owned validation items without changing the canonical
 * invariant syntax.
 *
 * invariantValidationStatement
 * ----------------------------
 *
 * Delegates directly to:
 *
 *     invariantStatement
 *
 * from ContractStatements.
 *
 * ============================================================================
 * PRIVATE RULES
 * ============================================================================
 *
 * There are intentionally no private semantic rules.
 *
 * The canonical invariant expression is deliberately not duplicated here.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar produces parser contexts only.
 *
 * It MUST NOT introduce a new AST node merely because this validation facade
 * exists.
 *
 * The downstream AST must preserve the canonical invariant construct and its
 * expression condition through the repository's existing AST architecture.
 *
 * At minimum the semantic representation must retain:
 *
 *     invariant kind
 *     condition expression
 *     source span
 *     enclosing scope
 *     source ordering
 *     provenance information required by the compiler
 *
 * Validation wrapper contexts are tooling boundaries, not new language
 * semantics.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * An invariant represents a condition that must be interpreted according to
 * the semantic contract model.
 *
 * This grammar only establishes structural conformance.
 *
 * Semantic analysis is responsible for determining:
 *
 *     - whether the condition is meaningful;
 *     - whether referenced names resolve;
 *     - whether referenced values are available in scope;
 *     - whether the invariant is legal in its enclosing context;
 *     - whether effects are permitted;
 *     - whether capabilities are available;
 *     - whether resource references are valid;
 *     - whether policy permits the invariant;
 *     - whether domain-specific semantic rules are satisfied.
 *
 * None of those decisions belong in this parser grammar.
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * The invariant condition uses the ordinary Zamani expression system through:
 *
 *     ContractStatements
 *
 * This file therefore does not introduce a special invariant type.
 *
 * There is deliberately no:
 *
 *     InvariantBoolean
 *     QuantumInvariantBoolean
 *     HardwareInvariantBoolean
 *     AIInvariantBoolean
 *
 * or equivalent domain-specific parser type.
 *
 * The semantic/type system determines whether the condition is a valid
 * predicate according to Zamani's type rules.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Parsing an invariant MUST NOT evaluate it.
 *
 * Therefore this grammar introduces no runtime effects.
 *
 * An invariant condition may nevertheless refer semantically to constructs
 * associated with effects.
 *
 * Examples of possible semantic effect categories include:
 *
 *     io
 *     network
 *     mutation
 *     randomness
 *     native
 *     foreign
 *     distributed
 *     measurement
 *     quantum
 *     learning
 *     adaptation
 *     reflection
 *     code_generation
 *     simulation
 *
 * Effect legality is determined by the existing effect system.
 *
 * This file MUST NOT create a second effect system.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Invariant conditions may contain capability-related expressions when
 * permitted by the ordinary expression system.
 *
 * For example, a semantic condition may refer to a capability associated with
 * quantum, tensor, accelerator, network, storage, or other computation.
 *
 * This grammar does not resolve capabilities.
 *
 * The pipeline remains:
 *
 *     source
 *       |
 *       v
 *     invariantStatement
 *       |
 *       v
 *     AST
 *       |
 *       v
 *     semantic analysis
 *       |
 *       v
 *     capability analysis
 *
 * Capability availability is target/environment dependent and therefore must
 * never be hard-coded into this grammar.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Invariants may semantically refer to resource state or resource-derived
 * conditions through ordinary expressions.
 *
 * This grammar imposes no universal resource limits.
 *
 * In particular, it MUST NOT encode limits for:
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
 *     processes
 *     devices
 *     memory
 *     storage
 *     tensor rank
 *     tensor dimensions
 *     register width
 *     network size
 *     topology size
 *     source size
 *
 * A numeric value appearing in an invariant is program data or program
 * semantics, not a universal implementation ceiling.
 *
 * ============================================================================
 * CONTRACT CONTRACT
 * ============================================================================
 *
 * `invariant` is one member of Zamani's general contract architecture.
 *
 * The canonical contract family includes:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * Ownership of those constructs remains with:
 *
 *     grammar/statements/contract.g4
 *
 * This file only isolates invariant validation.
 *
 * It MUST NOT redefine or partially reproduce the other contract constructs.
 *
 * ============================================================================
 * CONTRACT SEMANTICS
 * ============================================================================
 *
 * The semantic contract model may associate an invariant with concepts such
 * as:
 *
 *     condition
 *     scope
 *     assumptions
 *     requirements
 *     guarantees
 *     evidence
 *     provenance
 *     policy
 *
 * This grammar does not construct that model.
 *
 * It only supplies the canonical invariant syntax to the validation layer.
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Policy is external to this grammar.
 *
 * An invariant may be subject to policies concerning:
 *
 *     validation
 *     execution
 *     security
 *     resource use
 *     adaptation
 *     simulation
 *     deployment
 *     quantum execution
 *     distributed execution
 *     hardware realization
 *
 * Policy evaluation belongs to the existing policy subsystem.
 *
 * No policy is hard-coded here.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Validation results must remain attributable to their source invariant.
 *
 * The consuming infrastructure must be able to associate a result with:
 *
 *     source file
 *     source span
 *     invariant construct
 *     condition
 *     enclosing scope
 *     grammar/language version
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
 *     execution context
 *
 * This grammar does not implement provenance storage.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar produces no IR.
 *
 * The invariant enters the canonical pipeline through the AST and semantic
 * contract model.
 *
 * The intended architecture is:
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
 *     semantic analysis
 *       |
 *       +--> types
 *       +--> effects
 *       +--> capabilities
 *       +--> resources
 *       +--> contracts
 *       +--> policies
 *       +--> provenance
 *       |
 *       v
 *     canonical semantic representation
 *       |
 *       +--> classical representation
 *       +--> quantum::ir
 *       +--> HDL/hardware representation
 *       +--> distributed representation
 *       +--> accelerator representation
 *       +--> future domain representation
 *
 * This file must never bypass that architecture.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Invariants may constrain quantum computation through ordinary Zamani
 * expressions.
 *
 * Examples of semantic concepts that may eventually be referenced include:
 *
 *     measurement validity
 *     state validity
 *     resource availability
 *     operation validity
 *     resilience state
 *     capability availability
 *
 * This grammar does not know how quantum computation is implemented.
 *
 * It MUST NOT:
 *
 *     - enumerate quantum gates;
 *     - enumerate qubits;
 *     - encode a maximum qubit count;
 *     - define coupling maps;
 *     - define calibration;
 *     - define physical topology;
 *     - perform routing;
 *     - perform scheduling;
 *     - perform QEC;
 *     - select a QPU;
 *     - introduce another quantum IR.
 *
 * The canonical quantum boundary remains:
 *
 *     semantic model
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
 *     resilience / QEC
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
 * ============================================================================
 * HDL BOUNDARY
 * ============================================================================
 *
 * Invariants may constrain HDL/hardware semantic state.
 *
 * Examples of possible semantic conditions include:
 *
 *     signal validity
 *     state consistency
 *     timing validity
 *     protocol consistency
 *     hardware-resource conditions
 *
 * The grammar does not define hardware widths or capacities.
 *
 * It MUST NOT introduce universal constants such as:
 *
 *     fixed register width
 *     fixed bus width
 *     fixed memory size
 *     fixed FPGA dimensions
 *     fixed device count
 *     fixed clock count
 *     fixed pipeline depth
 *
 * Hardware realization remains downstream.
 *
 * ============================================================================
 * CLASSICAL BOUNDARY
 * ============================================================================
 *
 * Classical computation uses exactly the same invariant validation boundary.
 *
 * There is no separate classical invariant grammar.
 *
 * Conditions may refer to:
 *
 *     values
 *     state
 *     types
 *     resources
 *     capabilities
 *     algorithms
 *     data
 *     execution state
 *
 * through the common expression and semantic systems.
 *
 * ============================================================================
 * HYBRID BOUNDARY
 * ============================================================================
 *
 * Hybrid classical/quantum computation uses the same invariant syntax.
 *
 * A single invariant may semantically constrain relationships between:
 *
 *     classical state
 *     quantum state
 *     measurements
 *     control flow
 *     resources
 *     capabilities
 *     execution state
 *
 * No hybrid-specific invariant syntax is introduced here.
 *
 * ============================================================================
 * AI / REASONING BOUNDARY
 * ============================================================================
 *
 * Invariants may semantically constrain:
 *
 *     models
 *     knowledge
 *     reasoning
 *     learning
 *     adaptation
 *     confidence
 *     evidence
 *     decisions
 *     provenance
 *
 * The AI subsystem remains responsible for interpreting those concepts.
 *
 * This file does not introduce AI-specific invariant syntax.
 *
 * ============================================================================
 * DISTRIBUTED BOUNDARY
 * ============================================================================
 *
 * Invariants may semantically constrain:
 *
 *     consistency
 *     communication state
 *     service state
 *     replication
 *     placement
 *     topology
 *     fault state
 *     availability
 *
 * Distributed semantics remain owned by the distributed subsystem.
 *
 * ============================================================================
 * BACKEND BOUNDARY
 * ============================================================================
 *
 * This grammar is completely backend-neutral.
 *
 * It MUST NOT reference:
 *
 *     LLVM
 *     MLIR
 *     QIR
 *     vendor-specific IR
 *     CPU instruction sets
 *     GPU instruction sets
 *     FPGA vendor primitives
 *     ASIC implementation details
 *     QPU topology
 *     simulator implementation details
 *
 * Backend realization occurs only after semantic analysis and appropriate IR
 * lowering.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * The grammar must scale from the smallest supported computation to arbitrarily
 * large computations subject only to implementation resources.
 *
 * This file therefore contains no artificial universal limit on:
 *
 *     number of invariants
 *     expression size
 *     identifier size
 *     nesting depth
 *     number of domains
 *     number of resources
 *     number of capabilities
 *     number of devices
 *     number of qubits
 *     number of nodes
 *     tensor dimensions
 *     tensor rank
 *     memory
 *     source size
 *
 * Practical parser implementation limits may exist as resource/execution
 * constraints, but such limits are not language semantics and MUST NOT be
 * encoded as grammar constants.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Given identical:
 *
 *     source text
 *     grammar version
 *     lexer configuration
 *     parser configuration
 *
 * this validation grammar must produce deterministic parsing behavior.
 *
 * Parsing MUST NOT depend on:
 *
 *     wall-clock time
 *     randomness
 *     hardware discovery
 *     filesystem state
 *     network state
 *     environment state
 *     target availability
 *     runtime execution
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Syntax diagnostics are owned by the ANTLR parser and the repository's
 * diagnostic infrastructure.
 *
 * This grammar MUST NOT embed custom error actions.
 *
 * Structural errors include, for example:
 *
 *     invariant;
 *     invariant <missing condition>;
 *     invariant <condition without terminator>
 *     malformed expression following invariant
 *
 * Semantic errors are deliberately outside this grammar.
 *
 * Examples:
 *
 *     invalid type
 *     unresolved name
 *     illegal effect
 *     unavailable capability
 *     unsatisfied resource condition
 *     forbidden policy interaction
 *     invalid domain-specific invariant
 *
 * Those errors belong to later compiler phases.
 *
 * ============================================================================
 * POSITIVE TEST CONTRACT
 * ============================================================================
 *
 * The following forms must remain valid whenever the canonical expression
 * grammar accepts the referenced expressions:
 *
 *     invariant x >= 0;
 *
 *     invariant state_is_valid;
 *
 *     invariant value == expected_value;
 *
 *     invariant resource_available >= required_resource;
 *
 *     invariant model_is_consistent;
 *
 *     invariant quantum_state_is_valid;
 *
 *     invariant hardware_state_is_consistent;
 *
 *     invariant decision_has_evidence;
 *
 * These examples are deliberately symbolic.
 *
 * They do not impose physical implementation limits.
 *
 * ============================================================================
 * NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * The following must be rejected structurally:
 *
 *     invariant;
 *
 *     invariant ;
 *
 *     invariant x >= 0
 *
 *     invariant (;
 *
 *     invariant );
 *
 *     invariant <malformed-expression>;
 *
 * Exact diagnostic wording remains owned by the diagnostic subsystem.
 *
 * ============================================================================
 * BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Tests must cover invariants involving:
 *
 *     deeply nested expressions
 *     qualified names
 *     generic values
 *     symbolic resources
 *     symbolic capabilities
 *     classical state
 *     quantum state
 *     HDL state
 *     hybrid state
 *     distributed state
 *     AI/model state
 *     provenance-related values
 *     policy-related values
 *     large symbolic expressions
 *
 * The grammar must not change merely because a new domain is added, provided
 * the new domain uses the canonical expression/semantic architecture.
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Conformance tests must include:
 *
 *     many invariant statements
 *     large expressions
 *     deeply composed expressions
 *     long qualified names
 *     symbolic resource expressions
 *     symbolic capability expressions
 *     cross-domain conditions
 *     very large source units
 *
 * Tests must verify that no grammar-level fixed capacity is introduced.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing invariant source syntax remains authoritative in:
 *
 *     grammar/statements/contract.g4
 *
 * Therefore changes to this file MUST NOT silently alter source-language
 * syntax.
 *
 * If invariant syntax changes, the canonical owner must be updated first:
 *
 *     grammar/statements/contract.g4
 *
 * and this validation facade continues delegating to that canonical rule.
 *
 * This prevents parser divergence.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * Production source parsing:
 *
 *     grammar/Zamani.g4
 *          |
 *          v
 *     grammar/antlr/ZamaniParser.g4
 *          |
 *          v
 *     statements / ContractStatements
 *          |
 *          v
 *     invariantStatement
 *
 * Validation parsing:
 *
 *     grammar/validation/invariants.g4
 *          |
 *          v
 *     ContractStatements
 *          |
 *          v
 *     invariantStatement
 *
 * This deliberately creates two entry paths into the SAME canonical syntax,
 * not two syntaxes.
 *
 * Existing:
 *
 *     grammar/validation/contracts.g4
 *
 * remains the broader contract validation facade.
 *
 * It may consume this invariant-specific validation facade at the validation
 * orchestration layer if the repository chooses per-contract-kind validation
 * composition.
 *
 * It MUST NOT redefine invariant syntax merely to consume this file.
 *
 * ============================================================================
 * ROOT INTEGRATION
 * ============================================================================
 *
 * grammar/Zamani.g4 MUST remain the final complete-program composition root.
 *
 * This file MUST NOT be imported directly by Zamani.g4 merely to expose
 * invariant validation.
 *
 * The production parser already reaches invariant syntax through the canonical
 * parser hierarchy.
 *
 * ============================================================================
 * RUST INTEGRATION
 * ============================================================================
 *
 * This grammar contains no embedded target-language actions.
 *
 * Therefore the generated parser can participate in the repository's Rust
 * frontend targeting:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust Edition 2021
 *
 * The Rust implementation MUST remain safe Rust.
 *
 * This file introduces no:
 *
 *     unsafe
 *     raw-pointer requirement
 *     FFI requirement
 *     filesystem access
 *     network access
 *     runtime dependency
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no universal machine-capacity constants.
 *
 * It MUST NOT introduce names such as:
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
 * It also MUST NOT encode equivalent limits under different names.
 *
 * ============================================================================
 * AUTHORITY
 * ============================================================================
 *
 * Language architecture:
 *
 *     grammar/DESIGN.md
 *
 * Grammar navigation:
 *
 *     grammar/README.md
 *
 * Conformance/status:
 *
 *     grammar/grammar.md
 *
 * Historical/extended reference:
 *
 *     grammar/Zamani-Grammar.md
 *
 * Canonical root:
 *
 *     grammar/Zamani.g4
 *
 * Canonical parser hierarchy:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * Canonical lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Canonical invariant syntax:
 *
 *     grammar/statements/contract.g4
 *
 * Broader contract validation:
 *
 *     grammar/validation/contracts.g4
 *
 * Validation infrastructure:
 *
 *     grammar/validation/
 *
 * Specifications:
 *
 *     grammar/specification/
 *     grammar/spec/
 *
 * Rust frontend:
 *
 *     src/lexer.rs
 *     src/parser.rs
 *     src/ast/
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when all of the following are true:
 *
 *     1. The grammar is syntactically valid ANTLR4.
 *
 *     2. The grammar imports the canonical ContractStatements grammar.
 *
 *     3. invariantStatement remains owned exclusively by
 *        grammar/statements/contract.g4.
 *
 *     4. No invariant syntax is duplicated here.
 *
 *     5. No lexer token is duplicated here.
 *
 *     6. No semantic action is present.
 *
 *     7. No semantic predicate is present.
 *
 *     8. No target-specific implementation code is present.
 *
 *     9. No hardware capacity is hard-coded.
 *
 *    10. No quantum hardware limit is hard-coded.
 *
 *    11. No domain-specific invariant language is introduced.
 *
 *    12. Positive tests parse.
 *
 *    13. Negative tests fail deterministically.
 *
 *    14. Boundary tests pass.
 *
 *    15. Scalability tests pass subject only to implementation resources.
 *
 *    16. Source provenance can be retained by downstream infrastructure.
 *
 *    17. The generated parser integrates with the Rust 1.97/1.97.1 frontend.
 *
 *    18. No unsafe Rust is required.
 *
 *    19. The production Zamani parser continues to use the canonical
 *        ContractStatements path.
 *
 *    20. Future quantum, classical, HDL, AI, distributed, accelerator or
 *        hardware extensions do not require this file to be rewritten merely
 *        because a new domain was added.
 *
 * ============================================================================
 * GRAMMAR
 * ============================================================================
 */

parser grammar Invariants;

options {
    tokenVocab = ZamaniLexer;
}

import ContractStatements;

/*
 * ============================================================================
 * VALIDATION UNIT
 * ============================================================================
 *
 * Isolated validation/conformance entry point.
 *
 * EOF is intentional: a successful validation parse must consume the complete
 * supplied validation input rather than accepting a valid prefix.
 * ============================================================================
 */
invariantValidationUnit
    : invariantValidationItem* EOF
    ;

/*
 * ============================================================================
 * VALIDATION ITEM
 * ============================================================================
 *
 * This layer intentionally delegates immediately to the invariant-specific
 * validation statement boundary.
 * ============================================================================
 */
invariantValidationItem
    : invariantValidationStatement
    ;

/*
 * ============================================================================
 * VALIDATION STATEMENT
 * ============================================================================
 *
 * Canonical syntax owner:
 *
 *     grammar/statements/contract.g4
 *         -> ContractStatements
 *         -> invariantStatement
 *
 * This rule is only a stable validation-facing alias.
 * ============================================================================
 */
invariantValidationStatement
    : invariantStatement
    ;
/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/validation/contracts.g4
 *
 * GRAMMAR
 * -------
 * ContractValidation
 *
 * STATUS
 * ------
 * Production validation/conformance component
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file defines the parser-level validation entry point for Zamani's
 * canonical contract syntax.
 *
 * IMPORTANT:
 *
 * This file is NOT a second owner of Zamani contract syntax.
 *
 * Canonical source syntax remains owned by:
 *
 *     grammar/statements/contract.g4
 *         -> ContractStatements
 *
 * Function-attached contract blocks remain owned by:
 *
 *     grammar/functions/contracts.g4
 *         -> FunctionContracts
 *
 * Distributed contract declarations remain owned by:
 *
 *     grammar/distributed/contracts.g4
 *         -> DistributedContracts
 *
 * This file merely composes the already-authoritative contract statement
 * grammar into a validation/conformance entry point.
 *
 * ============================================================================
 * ARCHITECTURAL RULE
 * ============================================================================
 *
 * Validation observes and verifies grammar contracts.
 *
 * Validation MUST NOT become another source-language authority.
 *
 * Therefore this file:
 *
 *     - does not redefine requires;
 *     - does not redefine ensures;
 *     - does not redefine invariant;
 *     - does not redefine assume;
 *     - does not redefine guarantee;
 *     - does not redefine property;
 *     - does not redefine expression;
 *     - does not redefine punctuation;
 *     - does not define lexer tokens;
 *     - does not define identifiers;
 *     - does not define resource syntax;
 *     - does not define capability syntax;
 *     - does not define policy syntax;
 *     - does not define quantum syntax;
 *     - does not define HDL syntax;
 *     - does not define hardware syntax;
 *     - does not define AI syntax;
 *     - does not define distributed syntax.
 *
 * It consumes those constructs through their canonical owners.
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
 *     - no physical topology;
 *     - no quantum-device knowledge.
 *
 * ============================================================================
 * OWNERSHIP CONTRACT
 * ============================================================================
 *
 * OWNS
 * ----
 *
 *     contractValidationUnit
 *     contractValidationItem
 *
 * These are validation entry-point rules only.
 *
 * DOES NOT OWN
 * -------------
 *
 *     requiresStatement
 *     ensuresStatement
 *     invariantStatement
 *     assumeStatement
 *     guaranteeStatement
 *     propertyStatement
 *     contractCondition
 *
 * Those remain owned by:
 *
 *     grammar/statements/contract.g4
 *
 * ============================================================================
 * DEPENDENCIES
 * ============================================================================
 *
 * DEPENDS_ON
 * ----------
 *
 *     grammar/statements/contract.g4
 *         ContractStatements
 *
 *     grammar/lexer/keywords.g4
 *         indirectly through ZamaniLexer
 *
 *     grammar/expressions/
 *         indirectly through ContractStatements
 *
 *     grammar/core/punctuation.g4
 *         indirectly through ContractStatements
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The canonical public lexer is:
 *
 *     ZamaniLexer
 *
 * This file does not define or duplicate lexical tokens.
 *
 * Contract keywords are therefore obtained from the canonical lexical
 * architecture:
 *
 *     REQUIRES
 *     ENSURES
 *     INVARIANT
 *     ASSUME
 *     GUARANTEE
 *     PROPERTY
 *
 * The source spellings are owned by:
 *
 *     grammar/lexer/keywords.g4
 *
 * This file MUST NOT introduce alternate token identities for these concepts.
 *
 * ============================================================================
 * PARSER CONTRACT
 * ============================================================================
 *
 * Canonical standalone contract syntax is defined by:
 *
 *     ContractStatements
 *
 * This validation grammar consumes:
 *
 *     contractStatement
 *
 * from that grammar.
 *
 * Consequently:
 *
 *     validation/contracts.g4
 *              |
 *              v
 *     ContractStatements
 *              |
 *              v
 *     canonical contract syntax
 *
 * There is no duplicate contract parser.
 *
 * ============================================================================
 * VALIDATION ENTRY POINT
 * ============================================================================
 *
 * `contractValidationUnit` is intended for:
 *
 *     - grammar conformance tests;
 *     - parser validation;
 *     - isolated contract parsing;
 *     - negative syntax tests;
 *     - boundary tests;
 *     - determinism tests;
 *     - source-span tests;
 *     - validation tooling;
 *     - grammar regression tests.
 *
 * It is NOT the production language's universal compilation entry point.
 *
 * The production parser continues to reach contracts through:
 *
 *     grammar/statements/statements.g4
 *
 * ============================================================================
 * SOURCE LANGUAGE CONTRACT
 * ============================================================================
 *
 * The canonical standalone forms accepted through this validation facade are:
 *
 *     requires(condition);
 *     ensures(condition);
 *     invariant(condition);
 *     assume(condition);
 *     guarantee(condition);
 *     property(condition);
 *
 * Their actual syntax is not reproduced here.
 *
 * This is deliberate.
 *
 * ============================================================================
 * FUNCTION CONTRACT INTEGRATION
 * ============================================================================
 *
 * Function contracts remain owned by:
 *
 *     grammar/functions/contracts.g4
 *
 * Example:
 *
 *     fn compute(x) contract {
 *         requires(x >= 0);
 *         ensures(result >= 0);
 *     }
 *
 * This file MUST NOT import or redefine FunctionContracts merely to make
 * function contracts appear similar to standalone contract statements.
 *
 * Function-contract validation belongs to the function grammar's own
 * validation path and the repository-wide validation orchestration.
 *
 * This prevents:
 *
 *     ContractValidation
 *         |
 *         +--> ContractStatements
 *         |
 *         +--> FunctionContracts
 *
 * from accidentally becoming a competing function-declaration parser.
 *
 * ============================================================================
 * DISTRIBUTED CONTRACT INTEGRATION
 * ============================================================================
 *
 * Distributed contracts remain owned by:
 *
 *     grammar/distributed/contracts.g4
 *
 * They may semantically consume the same concepts:
 *
 *     requires
 *     ensures
 *     invariant
 *     guarantee
 *     property
 *
 * but their declaration structure remains distributed-domain ownership.
 *
 * This validation facade does not duplicate that syntax.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar creates parser contexts only.
 *
 * It MUST NOT define Rust AST structures.
 *
 * Canonical downstream mapping remains:
 *
 *     parser context
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic contract model
 *
 * The AST MUST preserve enough information to distinguish:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * together with:
 *
 *     source span
 *     enclosing scope
 *     source order
 *     condition expression
 *     provenance
 *
 * This validation grammar does not own those Rust structures.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Validation establishes structural conformance.
 *
 * Semantic analysis determines whether a parsed contract is meaningful in
 * its actual context.
 *
 * Examples of downstream semantic checks include:
 *
 *     condition type validity
 *     name resolution
 *     scope validity
 *     contract-kind legality
 *     resource requirement interpretation
 *     capability interpretation
 *     effect restrictions
 *     policy compatibility
 *     ownership rules
 *     provenance requirements
 *     domain-specific validity
 *
 * These are NOT encoded as parser predicates.
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * The contract condition remains an ordinary Zamani expression through the
 * canonical `ContractStatements` grammar.
 *
 * This validation file does not impose a special contract type.
 *
 * Therefore validation does not introduce:
 *
 *     ContractBoolean
 *     QuantumContractBoolean
 *     HardwareContractBoolean
 *     AIContractBoolean
 *
 * or any other domain-specific type.
 *
 * Type checking remains a semantic responsibility.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Parsing a contract does not evaluate its condition.
 *
 * Therefore this file produces no runtime effect.
 *
 * If a contract expression semantically refers to an operation carrying:
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
 * the existing effect system decides whether that use is permitted.
 *
 * This validation grammar does not invent a second effect system.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * A contract condition may semantically contain capability requirements.
 *
 * Example:
 *
 *     requires(capability("quantum.measurement"));
 *
 * This validation grammar merely validates the source structure.
 *
 * Capability resolution belongs downstream.
 *
 * Conceptually:
 *
 *     source
 *       |
 *       v
 *     contractStatement
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
 * This file MUST NOT inspect the target environment.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Contract conditions may express resource requirements.
 *
 * Examples include:
 *
 *     requires(memory >= required_memory);
 *     requires(qubits >= required_qubits);
 *     requires(capability("tensor.compute"));
 *     requires(topology.supports(required_topology));
 *
 * These are program-level semantic expressions.
 *
 * This validation grammar does not establish machine capacities.
 *
 * In particular, it contains no grammar-defined limit for:
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
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * A contract may be affected by policy.
 *
 * Policy ownership remains in the existing policy subsystem.
 *
 * This file does not evaluate:
 *
 *     allow
 *     forbid
 *     permit
 *     deny
 *     fallback
 *     retry
 *     recover
 *     deployment policy
 *     security policy
 *     adaptation policy
 *
 * Policy analysis is downstream.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Validation tooling must preserve the identity of the source contract
 * construct sufficiently for diagnostics and provenance.
 *
 * At minimum the consuming pipeline must be able to associate a validation
 * result with:
 *
 *     source file
 *     source span
 *     contract kind
 *     condition
 *     enclosing context
 *
 * Later compiler stages may additionally attach:
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
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar produces NO IR.
 *
 * Validation must not bypass the canonical semantic architecture.
 *
 * The production pipeline remains:
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
 *     canonical semantic model
 *       |
 *       +--> classical representation
 *       |
 *       +--> quantum::ir
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
 *     routing / scheduling / resilience where applicable
 *       |
 *       v
 *     ZQN / HAL / target realization where applicable
 *
 * ============================================================================
 * QUANTUM CONTRACT
 * ============================================================================
 *
 * Quantum semantics remain outside this validation grammar.
 *
 * A contract may accompany quantum computation:
 *
 *     requires(capability("quantum.measurement"));
 *     ensures(measurement.is_valid());
 *
 * Validation checks structural correctness.
 *
 * Quantum semantic analysis remains responsible for interpreting the condition
 * in a quantum context.
 *
 * Where the computation is quantum, the canonical quantum boundary remains:
 *
 *     quantum semantic model
 *          |
 *          v
 *     quantum::ir
 *
 * followed by the repository's quantum lowering pipeline.
 *
 * This file MUST NOT:
 *
 *     - enumerate quantum operations;
 *     - enumerate physical qubits;
 *     - define coupling maps;
 *     - define calibration;
 *     - perform routing;
 *     - perform scheduling;
 *     - implement QEC;
 *     - implement ZQN;
 *     - select a QPU;
 *     - create another quantum IR.
 *
 * ============================================================================
 * HDL / HARDWARE CONTRACT
 * ============================================================================
 *
 * Hardware and HDL conditions remain semantic expressions.
 *
 * Examples:
 *
 *     requires(signal.is_defined(clock));
 *     ensures(output.is_valid());
 *     invariant(hardware_state.is_consistent());
 *
 * This file does not define:
 *
 *     signal width limits
 *     register width limits
 *     memory limits
 *     FPGA dimensions
 *     ASIC resource counts
 *     clock-count limits
 *     physical addresses
 *     vendor identifiers
 *     device topology
 *
 * Target feasibility belongs downstream.
 *
 * ============================================================================
 * CLASSICAL CONTRACT
 * ============================================================================
 *
 * Classical computation uses the same contract architecture.
 *
 * There is no separate classical contract language.
 *
 * Examples:
 *
 *     requires(x >= 0);
 *     ensures(result >= x);
 *     invariant(state.is_consistent());
 *
 * ============================================================================
 * HYBRID CONTRACT
 * ============================================================================
 *
 * Hybrid classical/quantum computation also uses the same contract syntax.
 *
 * Example:
 *
 *     requires(capability("quantum.measurement"));
 *     requires(input.is_valid());
 *     ensures(result.is_valid());
 *
 * The condition remains domain-neutral at the grammar layer.
 *
 * ============================================================================
 * AI / REASONING CONTRACT
 * ============================================================================
 *
 * Contracts may constrain or describe reasoning, learning, adaptation,
 * uncertainty, evidence, decisions and provenance.
 *
 * Examples:
 *
 *     requires(model.is_valid());
 *     requires(confidence >= required_confidence);
 *     ensures(decision.has_evidence());
 *     invariant(model.is_consistent());
 *
 * This file does not introduce AI-specific contract syntax.
 *
 * AI semantics remain downstream.
 *
 * ============================================================================
 * DISTRIBUTED CONTRACT
 * ============================================================================
 *
 * Distributed source may use contract conditions involving:
 *
 *     communication
 *     consistency
 *     availability
 *     replication
 *     placement
 *     topology
 *     fault state
 *
 * Those concepts remain owned by their respective domain systems.
 *
 * This validation facade does not create distributed contract syntax.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Contract syntax must remain independent of the target realization.
 *
 * Therefore the same source-level contract may be validated before realization
 * for:
 *
 *     tiny systems
 *     embedded systems
 *     CPU systems
 *     multicore systems
 *     GPU systems
 *     FPGA systems
 *     ASIC systems
 *     accelerator systems
 *     QPU systems
 *     simulators
 *     HPC systems
 *     clusters
 *     distributed systems
 *     cloud systems
 *     heterogeneous systems
 *     future computational substrates
 *
 * A target may later fail semantic feasibility because a required capability
 * or resource is unavailable.
 *
 * That is NOT a grammar failure.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This file introduces no artificial finite language capacity.
 *
 * There is no maximum for:
 *
 *     contract statements
 *     validation items
 *     source units
 *     modules
 *     functions
 *     expressions
 *     contract expression size
 *     contract nesting
 *     quantum operations
 *     qubits
 *     CPUs
 *     GPUs
 *     FPGAs
 *     ASIC resources
 *     accelerators
 *     nodes
 *     devices
 *     threads
 *     tensor rank
 *     network size
 *     memory
 *
 * The validation entry point uses repetition:
 *
 *     contractValidationItem*
 *
 * rather than a finite enumeration of source capacity.
 *
 * "Infinity" therefore means:
 *
 *     no artificial finite language ceiling is introduced here.
 *
 * Actual execution and compilation remain bounded by resources available to
 * the implementation and target.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Validation parsing must depend only on:
 *
 *     source token sequence
 *     canonical grammar
 *     grammar version
 *     parser configuration
 *
 * It must NOT depend on:
 *
 *     wall-clock time
 *     randomness
 *     CPU availability
 *     GPU availability
 *     QPU availability
 *     memory availability
 *     filesystem state
 *     network state
 *     target selection
 *     scheduler state
 *     runtime state
 *     calibration state
 *
 * Identical input under identical parser configuration must produce equivalent
 * parse structure.
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser diagnostics belong to ANTLR/parser infrastructure.
 *
 * Examples:
 *
 *     missing condition
 *     missing opening parenthesis
 *     missing closing parenthesis
 *     malformed expression
 *     missing statement terminator
 *     unexpected token
 *
 * Semantic diagnostics belong downstream.
 *
 * Examples:
 *
 *     unknown name
 *     invalid condition type
 *     invalid contract context
 *     unavailable capability
 *     unavailable resource
 *     policy violation
 *     unsupported effect
 *     invalid ownership
 *     impossible target requirement
 *
 * Validation MUST NOT misclassify semantic feasibility as lexical or grammar
 * capacity.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * This grammar performs no:
 *
 *     filesystem access
 *     network access
 *     process execution
 *     model execution
 *     hardware discovery
 *     device discovery
 *     secret access
 *     foreign-function invocation
 *     generated-code execution
 *
 * It is therefore structurally inert.
 *
 * The consuming implementation remains subject to the repository's safe-Rust
 * requirement.
 *
 * No `unsafe` implementation is required by this grammar.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file MUST NOT contain any machine-derived language ceiling.
 *
 * Forbidden examples include:
 *
 *     MAX_CONTRACTS
 *     MAX_PRECONDITIONS
 *     MAX_POSTCONDITIONS
 *     MAX_INVARIANTS
 *     MAX_PROPERTIES
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
 * This grammar contains none of these constructs.
 *
 * Numeric values occurring inside expressions are program semantics and are
 * not compiler capacity declarations.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing source syntax remains owned by the existing canonical grammars.
 *
 * In particular:
 *
 *     standalone requires(...)       -> ContractStatements
 *     standalone ensures(...)        -> ContractStatements
 *     standalone invariant(...)      -> ContractStatements
 *     standalone assume(...)         -> ContractStatements
 *     standalone guarantee(...)      -> ContractStatements
 *     standalone property(...)       -> ContractStatements
 *
 *     contract { ... }               -> FunctionContracts
 *
 *     distributed contract syntax    -> DistributedContracts
 *
 * This validation grammar does not change those ownership boundaries.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE
 * --------
 *
 * The validation entry point must accept:
 *
 *     requires(x > 0);
 *
 *     ensures(result >= 0);
 *
 *     invariant(state.is_valid());
 *
 *     assume(input.is_normalized());
 *
 *     guarantee(output.is_valid());
 *
 *     property(result.is_consistent());
 *
 * Cross-domain expressions must also be accepted structurally when valid
 * according to the canonical expression grammar:
 *
 *     requires(capability("quantum.measurement"));
 *
 *     requires(memory >= required_memory);
 *
 *     requires(tensor.shape == expected_shape);
 *
 *     ensures(measurement.is_valid());
 *
 *     invariant(hardware_state.is_consistent());
 *
 *     property(model.is_consistent());
 *
 *     guarantee(decision.has_evidence());
 *
 * NEGATIVE
 * --------
 *
 * The validation entry point must reject malformed canonical syntax such as:
 *
 *     requires;
 *
 *     requires();
 *
 *     requires(;
 *
 *     requires(, x);
 *
 *     requires(x;
 *
 *     requires(x,);
 *
 *     requires(x, y);
 *
 *     ensures;
 *
 *     invariant;
 *
 *     assume;
 *
 *     guarantee;
 *
 *     property;
 *
 *     requires(x) trailing;
 *
 * These are parser/conformance tests.
 *
 * Semantic failures such as an unavailable capability MUST be tested in the
 * semantic validation layer rather than encoded here.
 *
 * ============================================================================
 * BOUNDARY TESTS
 * ============================================================================
 *
 * The validation suite must cover:
 *
 *     empty validation unit
 *     one contract statement
 *     many contract statements
 *     large expressions
 *     deeply nested expressions
 *     large qualified names
 *     mixed contract kinds
 *     mixed-domain expressions
 *     large source units
 *
 * Boundary tests must not modify this grammar to accommodate arbitrary test
 * sizes.
 *
 * ============================================================================
 * SCALABILITY TESTS
 * ============================================================================
 *
 * Tests should progressively increase:
 *
 *     contract count
 *     expression size
 *     source size
 *     nesting
 *     semantic complexity
 *
 * while keeping the grammar unchanged.
 *
 * The test suite must verify that no source-level finite capacity is hidden in
 * this validation facade.
 *
 * ============================================================================
 * DETERMINISM TESTS
 * ============================================================================
 *
 * Parse identical input repeatedly with:
 *
 *     identical grammar version
 *     identical lexer configuration
 *     identical parser configuration
 *
 * and verify equivalent parse-tree structure and source spans.
 *
 * ============================================================================
 * SOURCE-SPAN TESTS
 * ============================================================================
 *
 * Each validation item must retain enough parser context for the downstream
 * source-map/diagnostic layer to identify:
 *
 *     start position
 *     end position
 *     contract kind
 *     condition span
 *
 * The grammar itself does not manufacture Rust source-span structures.
 *
 * ============================================================================
 * CROSS-DOMAIN TESTS
 * ============================================================================
 *
 * The validation facade must be exercised with source involving:
 *
 *     classical computation
 *     numerical computation
 *     scientific computation
 *     AI/model computation
 *     reasoning
 *     learning
 *     adaptation
 *     uncertainty
 *     quantum computation
 *     hybrid computation
 *     HDL
 *     hardware intent
 *     accelerators
 *     concurrency
 *     distributed computation
 *     networking
 *     data
 *     security
 *     interoperability
 *     metaprogramming
 *     simulation
 *
 * The validation grammar must remain unchanged across these domains because
 * the contract syntax is universal.
 *
 * ============================================================================
 * AST / SEMANTIC INTEGRATION TEST
 * ============================================================================
 *
 * For every accepted validation item:
 *
 *     parser
 *       ->
 *     canonical ContractStatements context
 *       ->
 *     domain-neutral AST
 *       ->
 *     structural validation
 *       ->
 *     semantic contract model
 *
 * must remain traceable.
 *
 * No validation rule may jump directly from:
 *
 *     parser
 *
 * to:
 *
 *     hardware
 *     QPU
 *     GPU
 *     FPGA
 *     backend
 *     runtime
 *
 * ============================================================================
 * IR INTEGRATION TEST
 * ============================================================================
 *
 * Contract validation must not bypass the canonical semantic model.
 *
 * For a quantum context:
 *
 *     contract
 *       ->
 *     AST
 *       ->
 *     semantic contract
 *       ->
 *     quantum semantic analysis
 *       ->
 *     quantum::ir
 *
 * For a classical context:
 *
 *     contract
 *       ->
 *     AST
 *       ->
 *     semantic contract
 *       ->
 *     classical semantic/IR representation
 *
 * For HDL/hardware:
 *
 *     contract
 *       ->
 *     AST
 *       ->
 *     semantic contract
 *       ->
 *     hardware/HDL semantic representation
 *
 * The validation grammar remains common to all of them.
 *
 * ============================================================================
 * INTEGRATION WITH grammar/statements/statements.g4
 * ============================================================================
 *
 * The production statement dispatcher must continue to import:
 *
 *     ContractStatements
 *
 * and route:
 *
 *     contractStatement
 *
 * through that grammar.
 *
 * `ContractValidation` is NOT to be inserted into the universal statement
 * dispatcher.
 *
 * Doing so would create an unnecessary second composition path.
 *
 * ============================================================================
 * INTEGRATION WITH grammar/validation/README.md
 * ============================================================================
 *
 * This file is a specialized validation contract under:
 *
 *     grammar/validation/
 *
 * The validation README remains the orchestration authority.
 *
 * It should treat this file as:
 *
 *     contract syntax conformance entry point
 *
 * alongside specialized validation contracts for:
 *
 *     ambiguity
 *     recursion
 *     reachability
 *     source spans
 *     AST coverage
 *     semantic coverage
 *     IR coverage
 *     portability
 *     hard-coding
 *     scalability
 *     determinism
 *     compatibility
 *
 * This file MUST NOT redefine the repository-wide validation hierarchy.
 *
 * ============================================================================
 * INTEGRATION WITH grammar/validation/grammar-validator.md
 * ============================================================================
 *
 * The grammar validator may use:
 *
 *     contractValidationUnit
 *
 * as an isolated conformance target when validating standalone contract
 * syntax.
 *
 * It must still validate the canonical production path:
 *
 *     ZamaniLexer
 *         ->
 *     ZamaniParser
 *         ->
 *     Statements
 *         ->
 *     ContractStatements
 *
 * The validation facade is therefore a testing and validation aid, not a
 * replacement for the production parser.
 *
 * ============================================================================
 * INTEGRATION WITH TESTS
 * ============================================================================
 *
 * Recommended test ownership:
 *
 *     grammar/tests/validation/contracts/
 *
 * Suggested categories:
 *
 *     positive/
 *     negative/
 *     boundary/
 *     scalability/
 *     determinism/
 *     source-spans/
 *     cross-domain/
 *     compatibility/
 *
 * The exact test organization may follow the existing repository test layout,
 * but the semantic ownership remains unchanged.
 *
 * ============================================================================
 * INTEGRATION WITH RUST
 * ============================================================================
 *
 * This grammar has no embedded Rust and requires no unsafe implementation.
 *
 * The generated parser is consumed by the existing Zamani frontend.
 *
 * The Rust implementation must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust Edition 2021
 *
 * The validation grammar must not require:
 *
 *     unsafe blocks
 *     unsafe functions
 *     unsafe traits
 *     unsafe extern blocks
 *
 * ============================================================================
 * INTEGRATION WITH RESOURCE/CAPABILITY ANALYSIS
 * ============================================================================
 *
 * This file only validates syntax.
 *
 * Resource/capability semantics are resolved after AST construction.
 *
 * Example:
 *
 *     requires(capability("quantum.measurement"));
 *
 * flows conceptually through:
 *
 *     syntax
 *       ->
 *     AST
 *       ->
 *     semantic expression
 *       ->
 *     capability requirement
 *       ->
 *     capability negotiation
 *       ->
 *     target feasibility
 *
 * No target is selected by this file.
 *
 * ============================================================================
 * INTEGRATION WITH EFFECT ANALYSIS
 * ============================================================================
 *
 * Contract expressions remain part of the existing expression/effect analysis.
 *
 * This prevents the validation subsystem from creating a parallel effect
 * taxonomy.
 *
 * ============================================================================
 * INTEGRATION WITH POLICY ANALYSIS
 * ============================================================================
 *
 * Policies may determine whether a contract is:
 *
 *     permitted
 *     forbidden
 *     satisfiable
 *     enforceable
 *     admissible
 *
 * Such decisions remain downstream.
 *
 * This file validates only source structure.
 *
 * ============================================================================
 * INTEGRATION WITH PROVENANCE
 * ============================================================================
 *
 * Contract validation results should be attributable to the canonical source
 * contract construct.
 *
 * The provenance chain remains:
 *
 *     source
 *       ->
 *     parser context
 *       ->
 *     AST
 *       ->
 *     semantic contract
 *       ->
 *     validation result
 *
 * ============================================================================
 * FUTURE EXTENSIBILITY
 * ============================================================================
 *
 * Future contract kinds must first be added to the canonical contract owner:
 *
 *     grammar/statements/contract.g4
 *
 * and only after that should this validation facade automatically consume them
 * through:
 *
 *     contractStatement
 *
 * This means adding a legitimate new universal contract kind does NOT require
 * duplicating the syntax in this file.
 *
 * This is intentional and satisfies the repository's independent-file-first
 * maintainability requirement.
 *
 * ============================================================================
 * FORBIDDEN EXTENSIONS
 * ============================================================================
 *
 * Do NOT add rules here such as:
 *
 *     requiresStatement
 *     ensuresStatement
 *     invariantStatement
 *     assumeStatement
 *     guaranteeStatement
 *     propertyStatement
 *
 * as new implementations.
 *
 * Do NOT add:
 *
 *     expression
 *     identifier
 *     qualifiedName
 *     capabilityExpression
 *     resourceExpression
 *     quantumExpression
 *     hardwareExpression
 *
 * as duplicate grammar implementations.
 *
 * Do NOT add:
 *
 *     QuantumContract
 *     GPUContract
 *     FPGAContract
 *     HDLContract
 *     AIContract
 *     DistributedContract
 *
 * parser rules.
 *
 * Do NOT add machine-specific capacity rules.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when all of the following are true:
 *
 * [ ] The grammar name is `ContractValidation`.
 *
 * [ ] The file is a parser grammar.
 *
 * [ ] `tokenVocab = ZamaniLexer` is used.
 *
 * [ ] `ContractStatements` is imported.
 *
 * [ ] No contract source syntax is duplicated.
 *
 * [ ] `contractValidationUnit` is the validation entry point.
 *
 * [ ] `contractValidationItem` delegates to `contractStatement`.
 *
 * [ ] Standalone contract syntax remains owned by ContractStatements.
 *
 * [ ] Function contracts remain owned by FunctionContracts.
 *
 * [ ] Distributed contracts remain owned by DistributedContracts.
 *
 * [ ] Expressions remain owned by Expressions.
 *
 * [ ] Lexer vocabulary remains owned by the lexical hierarchy.
 *
 * [ ] No semantic predicates are used.
 *
 * [ ] No embedded Rust is used.
 *
 * [ ] No runtime execution is used.
 *
 * [ ] No filesystem access is used.
 *
 * [ ] No network access is used.
 *
 * [ ] No hardware discovery is used.
 *
 * [ ] No target selection is used.
 *
 * [ ] No resource allocation is used.
 *
 * [ ] No machine-capacity constant is used.
 *
 * [ ] No quantum-operation catalogue is used.
 *
 * [ ] No second quantum IR is introduced.
 *
 * [ ] AST ownership remains downstream.
 *
 * [ ] Semantic ownership remains downstream.
 *
 * [ ] Effect ownership remains downstream.
 *
 * [ ] Capability ownership remains downstream.
 *
 * [ ] Resource ownership remains downstream.
 *
 * [ ] Policy ownership remains downstream.
 *
 * [ ] Provenance remains preserved downstream.
 *
 * [ ] Positive tests exist.
 *
 * [ ] Negative tests exist.
 *
 * [ ] Boundary tests exist.
 *
 * [ ] Scalability tests exist.
 *
 * [ ] Determinism tests exist.
 *
 * [ ] Source-span tests exist.
 *
 * [ ] Cross-domain tests exist.
 *
 * [ ] Compatibility tests exist.
 *
 * [ ] Rust 1.97 integration succeeds.
 *
 * [ ] Rust 1.97.1 integration succeeds.
 *
 * [ ] Consuming implementation remains safe Rust.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL GUARANTEE
 * ============================================================================
 *
 * This file deliberately establishes:
 *
 *     ONE VALIDATION ENTRY POINT
 *              |
 *              v
 *     CANONICAL CONTRACT SYNTAX
 *              |
 *              v
 *     DOMAIN-NEUTRAL AST
 *              |
 *              v
 *     SEMANTIC CONTRACT MODEL
 *              |
 *       +------+------+------+------+------+
 *       |      |      |      |      |      |
 *       v      v      v      v      v      v
 *     Types Effects Resources Capabilities Policies Provenance
 *              |
 *              v
 *       CANONICAL SEMANTIC MODEL
 *              |
 *       +------+------+------+
 *       |             |      |
 *       v             v      v
 *   Classical     quantum::ir HDL/HW
 *       |             |      |
 *       +------+------+------+
 *              |
 *              v
 *        Target-independent
 *          optimization
 *              |
 *              v
 *       lowering / realization
 *
 * This keeps validation separate from language definition while allowing
 * contract syntax to participate in the full Zamani architecture.
 *
 * ============================================================================
 * GRAMMAR
 * ============================================================================
 */

parser grammar ContractValidation;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * ============================================================================
 * CANONICAL CONTRACT SYNTAX IMPORT
 * ============================================================================
 *
 * `ContractStatements` is the sole owner of standalone contract syntax.
 *
 * This import is the critical integration boundary.
 */
import ContractStatements;


/*
 * ============================================================================
 * VALIDATION UNIT
 * ============================================================================
 *
 * Zero or more canonical standalone contract statements followed by EOF.
 *
 * This rule is intended for isolated grammar-validation and conformance
 * parsing.
 *
 * It does NOT replace the production `program` / `statement` parser path.
 */
contractValidationUnit
    : contractValidationItem* EOF
    ;


/*
 * ============================================================================
 * VALIDATION ITEM
 * ============================================================================
 *
 * Delegates directly to the canonical contract statement rule.
 *
 * There is deliberately no second implementation of:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * This rule exists only to provide a stable validation-level parser context.
 */
contractValidationItem
    : contractStatement
    ;
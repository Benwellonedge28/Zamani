/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/validation/refinement.g4
 *
 * GRAMMAR
 * -------
 * RefinementValidation
 *
 * STATUS
 * ------
 * Production validation facade
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar provides the isolated parser entry point used to validate
 * refinement-oriented contract/property syntax.
 *
 * IMPORTANT:
 *
 * Refinement is a semantic relationship in Zamani. This file does NOT invent
 * a second source-level `refinement` statement.
 *
 * The canonical source-level property syntax is owned by:
 *
 *     grammar/statements/contract.g4
 *
 * through:
 *
 *     propertyStatement
 *
 * This file therefore exposes that canonical rule through a complete-input
 * validation entry point.
 *
 * The semantic layer determines whether the parsed property represents:
 *
 *     - a refinement;
 *     - a refinement constraint;
 *     - a subtype/refinement relationship;
 *     - a dependent predicate;
 *     - a contract strengthening;
 *     - another supported semantic relationship.
 *
 * ============================================================================
 * ARCHITECTURAL OBJECTIVE
 * ============================================================================
 *
 * The validation layer verifies source structure.
 *
 * It does NOT become a second language authority.
 *
 * The ownership chain is:
 *
 *     source
 *       |
 *       v
 *     lexer
 *       |
 *       v
 *     canonical contract grammar
 *       |
 *       v
 *     propertyStatement
 *       |
 *       v
 *     refinement validation facade
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic refinement analysis
 *       |
 *       +-------------------+-------------------+
 *       |                   |                   |
 *       v                   v                   v
 *     types              contracts          properties
 *       |                   |                   |
 *       +-------------------+-------------------+
 *                           |
 *                           v
 *                    canonical semantic model
 *                           |
 *             +-------------+-------------+
 *             |                           |
 *             v                           v
 *       applicable IR                 quantum::ir
 *             |                           |
 *             +-------------+-------------+
 *                           |
 *                           v
 *                    target-independent
 *                       optimization
 *                           |
 *                           v
 *                    lowering/realization
 *
 * ============================================================================
 * OWNS
 * ============================================================================
 *
 * This file owns only:
 *
 *     refinementValidationUnit
 *
 * It owns the validation-level entry point and complete-input boundary.
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     propertyStatement
 *     contractStatement
 *     requiresStatement
 *     ensuresStatement
 *     invariantStatement
 *     assumeStatement
 *     guaranteeStatement
 *
 * It also does NOT own:
 *
 *     lexer rules
 *     keywords
 *     operators
 *     punctuation
 *     identifiers
 *     names
 *     expressions
 *     types
 *     type refinement semantics
 *     dependent-type semantics
 *     contract semantics
 *     proof semantics
 *     theorem proving
 *     resource allocation
 *     capability resolution
 *     effect evaluation
 *     policy evaluation
 *     provenance implementation
 *     IR construction
 *     quantum routing
 *     quantum scheduling
 *     QEC
 *     ZQN
 *     HAL
 *     backend implementation
 *
 * ============================================================================
 * DEPENDS_ON
 * ============================================================================
 *
 *     grammar/antlr/ZamaniLexer.g4
 *         |
 *         +-- canonical lexical vocabulary
 *
 *     grammar/statements/contract.g4
 *         |
 *         +-- propertyStatement
 *
 *     grammar/expressions/
 *         |
 *         +-- expression
 *
 *     grammar/types/
 *         |
 *         +-- refinement/dependent/type constraint semantics
 *
 * ============================================================================
 * EXPORTS
 * ============================================================================
 *
 *     refinementValidationUnit
 *
 * This is intentionally the only public rule exported by this validation
 * facade.
 *
 * ============================================================================
 * CONSUMED_BY
 * ============================================================================
 *
 * Validation/conformance tooling may invoke:
 *
 *     refinementValidationUnit
 *
 * directly when validating an isolated refinement-oriented property.
 *
 * The complete Zamani parser continues to use:
 *
 *     grammar/Zamani.g4
 *
 * and:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * This validation entry point does NOT replace the canonical program parser.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar creates parser contexts only.
 *
 * It MUST NOT introduce a Rust AST type.
 *
 * The parsed property must map through the existing AST representation used
 * for canonical property/contract syntax.
 *
 * Conceptually:
 *
 *     PropertySyntax
 *         |
 *         v
 *     domain-neutral AST property/contract representation
 *         |
 *         v
 *     semantic refinement analysis
 *
 * No target-specific AST is permitted here.
 *
 * In particular, this file must not introduce:
 *
 *     QuantumRefinement
 *     GPURefinement
 *     FPGARefinement
 *     CPURefinement
 *     HDLRefinement
 *     AIRefinement
 *     DistributedRefinement
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * A refinement is determined downstream.
 *
 * A property may semantically refine:
 *
 *     a type
 *     a value
 *     a function result
 *     a function contract
 *     an invariant
 *     a capability requirement
 *     a resource requirement
 *     a protocol
 *     a state transition
 *     a hardware intent
 *     a quantum semantic condition
 *     a distributed invariant
 *     another semantic property
 *
 * The parser does not decide which interpretation applies.
 *
 * Semantic analysis determines:
 *
 *     refinement subject
 *     refinement predicate
 *     refinement scope
 *     assumptions
 *     inherited constraints
 *     strengthened guarantees
 *     evidence
 *     provenance
 *     satisfiability
 *     compatibility
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * Refinement syntax reuses the canonical property/contract expression.
 *
 * This file does not define a separate:
 *
 *     refinementExpression
 *
 * The expression remains owned by the existing expression grammar.
 *
 * Semantic type checking determines whether the property is valid as a
 * refinement predicate.
 *
 * Depending on context, a refinement predicate may be:
 *
 *     Boolean
 *     predicate
 *     symbolic proposition
 *     dependent predicate
 *     capability constraint
 *     resource constraint
 *     type-level constraint
 *     probabilistic predicate
 *
 * The grammar does not hard-code this semantic set.
 *
 * ============================================================================
 * CONTRACT INTEGRATION
 * ============================================================================
 *
 * Refinement participates in the existing contract model.
 *
 * Conceptually:
 *
 *     requires
 *        |
 *        v
 *     assumptions
 *        |
 *        v
 *     property/refinement
 *        |
 *        v
 *     guarantees
 *        |
 *        v
 *     ensures
 *
 * Refinement analysis may therefore consume:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * but this grammar does not duplicate any of those constructs.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Parsing a refinement property has no execution effect.
 *
 * Any effects associated with expressions inside the property are determined
 * by the existing effect-analysis subsystem.
 *
 * This file does not create a second effect taxonomy.
 *
 * Existing effect categories remain authoritative, including where applicable:
 *
 *     IO
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
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Refinement predicates may reference capabilities.
 *
 * For example, a canonical property expression may semantically express a
 * requirement concerning:
 *
 *     capability("quantum.measurement")
 *     capability("tensor.compute")
 *     capability("distributed.compute")
 *
 * This grammar merely parses the property.
 *
 * Capability resolution remains downstream:
 *
 *     syntax
 *       ->
 *     AST
 *       ->
 *     semantic property
 *       ->
 *     capability analysis
 *       ->
 *     target feasibility
 *
 * No hardware is selected by this file.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Refinement predicates may contain resource-related expressions.
 *
 * Examples include semantic conditions involving:
 *
 *     memory
 *     qubits
 *     processors
 *     accelerators
 *     storage
 *     bandwidth
 *     latency
 *     energy
 *     topology
 *
 * The grammar imposes no universal capacity limit.
 *
 * In particular, this file contains no:
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
 * Resource feasibility belongs to semantic/resource analysis.
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Policies may constrain whether a refinement is:
 *
 *     permitted
 *     admissible
 *     satisfiable
 *     enforceable
 *     deployable
 *
 * Policy evaluation is downstream.
 *
 * This file never evaluates policy.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * The resulting parser context must remain traceable to the original source.
 *
 * Downstream provenance should be capable of preserving, where supported:
 *
 *     source file
 *     source span
 *     enclosing declaration
 *     enclosing module
 *     property identity
 *     refinement subject
 *     refinement predicate
 *     semantic derivation
 *     evidence
 *     verification
 *     transformation
 *
 * This file does not implement provenance storage.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar produces no IR.
 *
 * The canonical pipeline remains:
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
 *     semantic refinement analysis
 *       ->
 *     type analysis
 *       ->
 *     effect analysis
 *       ->
 *     capability analysis
 *       ->
 *     resource analysis
 *       ->
 *     policy analysis
 *       ->
 *     provenance
 *       ->
 *     canonical semantic representation
 *       ->
 *     applicable domain IR
 *
 * Refinement therefore remains a semantic property of computation rather than
 * becoming a new IR family.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * A refinement predicate may refer to quantum semantics.
 *
 * Example semantic intent:
 *
 *     property(quantum_state.is_valid());
 *
 * The parser does not create a quantum representation.
 *
 * If the enclosing computation is quantum, the downstream architecture
 * remains:
 *
 *     domain-neutral AST
 *       ->
 *     semantic quantum analysis
 *       ->
 *     quantum::ir
 *       ->
 *     optimization
 *       ->
 *     decomposition
 *       ->
 *     routing
 *       ->
 *     scheduling
 *       ->
 *     resilience / QEC
 *       ->
 *     ZQN
 *       ->
 *     HAL
 *       ->
 *     target realization
 *
 * This grammar must never:
 *
 *     - enumerate quantum gates;
 *     - allocate physical qubits;
 *     - encode topology;
 *     - encode calibration;
 *     - select a QPU;
 *     - implement QEC;
 *     - create another quantum IR.
 *
 * ============================================================================
 * HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * Refinement properties may constrain hardware intent.
 *
 * Examples of semantic subjects include:
 *
 *     signal validity
 *     timing properties
 *     interface properties
 *     protocol properties
 *     state-machine properties
 *     synthesis constraints
 *     verification properties
 *
 * The grammar does not encode fixed:
 *
 *     signal widths
 *     register counts
 *     memory sizes
 *     FPGA dimensions
 *     ASIC resources
 *     clock counts
 *     pipeline depths
 *
 * Hardware realization remains downstream.
 *
 * ============================================================================
 * DISTRIBUTED / NETWORK BOUNDARY
 * ============================================================================
 *
 * Refinement properties may describe:
 *
 *     distributed invariants
 *     consistency
 *     ordering
 *     protocol correctness
 *     network conditions
 *     service guarantees
 *     fault conditions
 *
 * The grammar does not select:
 *
 *     nodes
 *     routes
 *     network topology
 *     machines
 *     services
 *
 * Those decisions remain downstream.
 *
 * ============================================================================
 * AI / KNOWLEDGE BOUNDARY
 * ============================================================================
 *
 * Refinement properties may semantically constrain:
 *
 *     model behavior
 *     inference results
 *     learned state
 *     adaptation state
 *     evidence
 *     confidence
 *     decisions
 *     knowledge
 *
 * No AI-specific refinement grammar is introduced.
 *
 * AI semantics consume the same generic property/contract representation.
 *
 * ============================================================================
 * VALIDATION BOUNDARY
 * ============================================================================
 *
 * This file performs structural parsing only.
 *
 * It does not prove a refinement.
 *
 * It does not decide satisfiability.
 *
 * It does not execute a solver.
 *
 * It does not evaluate a model.
 *
 * It does not inspect hardware.
 *
 * It does not perform runtime verification.
 *
 * Those responsibilities belong to the appropriate semantic/verification
 * subsystems.
 *
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * Syntax errors are parser errors.
 *
 * Examples of malformed isolated input include:
 *
 *     property
 *     property()
 *     property(
 *     property(x
 *     property(x, y)
 *     property(x);
 *     property(x) trailing
 *
 * Whether a syntactically valid property is a valid refinement is a semantic
 * diagnostic, not a parser diagnostic.
 *
 * Semantic diagnostics may include:
 *
 *     invalid refinement subject
 *     invalid refinement predicate
 *     incompatible refinement
 *     unsatisfied refinement
 *     contradictory refinement
 *     unavailable capability
 *     unavailable resource
 *     invalid type relationship
 *     invalid contract context
 *     policy violation
 *     insufficient evidence
 *     unsupported verification obligation
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * This file deliberately introduces no new source keyword.
 *
 * Canonical source syntax remains owned by:
 *
 *     grammar/statements/contract.g4
 *
 * If a future language revision introduces an explicit refinement keyword,
 * that keyword MUST first be specified and implemented through the canonical
 * lexer and canonical statement/type grammar.
 *
 * This validation facade must then consume the canonical rule rather than
 * independently redefining it.
 *
 * Therefore:
 *
 *     new syntax
 *         ->
 *     specification
 *         ->
 *     lexer
 *         ->
 *     canonical grammar owner
 *         ->
 *     AST
 *         ->
 *     semantic refinement model
 *         ->
 *     validation facade
 *
 * Never:
 *
 *     validation facade
 *         ->
 *     private syntax
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This file defines no finite language capacity.
 *
 * There is no grammar-level maximum for:
 *
 *     number of properties
 *     number of refinement predicates
 *     expression size
 *     expression nesting
 *     source-unit size
 *     module count
 *     function count
 *     type count
 *     number of refinement relationships
 *     number of quantum operations
 *     number of qubits
 *     processor count
 *     accelerator count
 *     device count
 *     node count
 *     memory capacity
 *     tensor rank
 *     network size
 *
 * "Infinity" means that this grammar introduces no artificial finite ceiling.
 *
 * Actual limits remain implementation, compiler, runtime, target, deployment,
 * and physical-resource constraints.
 *
 * The same source-level property must remain syntactically valid regardless
 * of whether the eventual realization is:
 *
 *     tiny
 *     embedded
 *     CPU
 *     multicore
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     QPU
 *     simulator
 *     HPC
 *     cluster
 *     distributed
 *     cloud
 *     future computational hardware
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Refinement syntax must be independent of target realization.
 *
 * Target changes MUST NOT require rewriting a refinement merely because:
 *
 *     processor count changes;
 *     memory changes;
 *     accelerator count changes;
 *     QPU capacity changes;
 *     topology changes;
 *     node count changes;
 *     device availability changes.
 *
 * Target feasibility is determined downstream.
 *
 * Therefore:
 *
 *     same source
 *          |
 *          +--> target A
 *          |
 *          +--> target B
 *          |
 *          +--> target C
 *          |
 *          +--> future target
 *
 * preserves the source-level refinement semantics.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing must depend only on:
 *
 *     source text
 *     lexer vocabulary
 *     grammar composition
 *     parser configuration
 *
 * Parsing must not depend on:
 *
 *     wall-clock time
 *     randomness
 *     hardware availability
 *     filesystem state
 *     network state
 *     runtime state
 *     target selection
 *     scheduler state
 *
 * Repeated parsing of identical input under identical grammar/configuration
 * must produce equivalent parse-tree structure and source spans.
 *
 * ============================================================================
 * SAFETY CONTRACT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no embedded Rust;
 *     no executable actions;
 *     no semantic predicates;
 *     no filesystem access;
 *     no network access;
 *     no hardware discovery;
 *     no resource allocation;
 *     no target selection;
 *     no runtime execution;
 *     no mutable global state.
 *
 * The consuming Zamani implementation must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and must use safe Rust.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE TESTS
 * --------------
 *
 * The validation suite should include canonical property forms such as:
 *
 *     property(x > 0);
 *     property(result.is_valid());
 *     property(state.is_consistent());
 *
 * and cross-domain property expressions where supported:
 *
 *     property(capability("quantum.measurement"));
 *     property(memory >= required_memory);
 *     property(tensor.shape == expected_shape);
 *     property(measurement.is_valid());
 *     property(hardware_state.is_consistent());
 *     property(distributed_state.is_consistent());
 *
 * NEGATIVE TESTS
 * --------------
 *
 *     property
 *     property()
 *     property(
 *     property(x
 *     property(x, y)
 *     property(x) trailing
 *
 * Semantic-negative cases must separately cover:
 *
 *     invalid refinement subject
 *     invalid predicate
 *     contradictory refinement
 *     unsatisfied refinement
 *     invalid type refinement
 *     unavailable capability
 *     unavailable resource
 *     policy violation
 *
 * BOUNDARY TESTS
 * --------------
 *
 * Include:
 *
 *     deeply nested predicates
 *     large symbolic expressions
 *     generic types
 *     dependent types
 *     uncertainty predicates
 *     capability predicates
 *     resource predicates
 *     quantum-derived predicates
 *     HDL properties
 *     distributed properties
 *     AI/model properties
 *
 * SCALABILITY TESTS
 * -----------------
 *
 * Generate increasing:
 *
 *     property count
 *     expression size
 *     expression nesting
 *     module size
 *     source-unit size
 *
 * without changing this grammar.
 *
 * Verify that no artificial capacity limit is introduced by this file.
 *
 * DETERMINISM TESTS
 * -----------------
 *
 * Parse identical input repeatedly and verify equivalent parser output.
 *
 * PORTABILITY TESTS
 * -----------------
 *
 * Parse identical property source under different:
 *
 *     target descriptions
 *     resource inventories
 *     processor counts
 *     device counts
 *     hardware topologies
 *
 * and verify identical source syntax/parse structure.
 *
 * CROSS-DOMAIN TESTS
 * ------------------
 *
 * Validate properties associated with:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     hardware
 *     AI
 *     data
 *     distributed
 *     networking
 *     accelerator
 *     scientific
 *     embedded
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden in this file:
 *
 *     machine-capacity constants
 *     target identifiers
 *     physical qubit identifiers
 *     fixed topology declarations
 *     fixed processor counts
 *     fixed memory capacities
 *     fixed tensor limits
 *     fixed register widths
 *     fixed network limits
 *     quantum gate catalogues
 *     backend-specific syntax
 *
 * Numeric literals appearing inside expressions remain ordinary program
 * semantics and are not implementation ceilings.
 *
 * ============================================================================
 * INTEGRATION MATRIX
 * ============================================================================
 *
 * LEXER
 * -----
 *
 * Uses:
 *
 *     tokenVocab = ZamaniLexer
 *
 * No lexer rules are duplicated.
 *
 * CANONICAL SYNTAX
 * ----------------
 *
 * Owned by:
 *
 *     grammar/statements/contract.g4
 *
 * AST
 * ---
 *
 * Existing domain-neutral AST subsystem.
 *
 * SEMANTICS
 * ---------
 *
 * Existing semantic/type/refinement analysis.
 *
 * TYPES
 * -----
 *
 * Integrates with:
 *
 *     grammar/types/
 *
 * for refinement/dependent/type constraint semantics.
 *
 * CONTRACTS
 * ---------
 *
 * Integrates with:
 *
 *     grammar/validation/contracts.g4
 *     grammar/validation/properties.g4
 *     grammar/validation/requires.g4
 *     grammar/validation/ensures.g4
 *     grammar/validation/invariants.g4
 *
 * without duplicating their syntax.
 *
 * RESOURCES
 * ---------
 *
 * Resource interpretation remains under:
 *
 *     grammar/resources/
 *
 * EFFECTS
 * -------
 *
 * Effect interpretation remains under:
 *
 *     grammar/effects/
 *
 * POLICIES
 * --------
 *
 * Policy interpretation remains under the policy/security semantic layers.
 *
 * PROVENANCE
 * ----------
 *
 * Provenance is preserved by the AST/semantic/compiler pipeline.
 *
 * IR
 * --
 *
 * No IR is created here.
 *
 * Domain lowering remains downstream.
 *
 * QUANTUM
 * -------
 *
 * Quantum semantics continue through:
 *
 *     quantum semantic model
 *         ->
 *     quantum::ir
 *
 * HDL/HARDWARE
 * ------------
 *
 * Hardware realization remains downstream.
 *
 * COMPILER
 * --------
 *
 * Refinement information may be used by:
 *
 *     type checking
 *     constraint solving
 *     verification
 *     optimization
 *     specialization
 *     resource analysis
 *     code generation
 *
 * but those systems remain owners of their respective operations.
 *
 * RUNTIME
 * -------
 *
 * Runtime enforcement, when selected by semantic policy, is downstream.
 *
 * ============================================================================
 * OWNERSHIP SUMMARY
 * ============================================================================
 *
 * DEPENDS_ON:
 *     ZamaniLexer
 *     ContractStatements
 *     canonical expression grammar
 *     semantic refinement/type contracts
 *
 * EXPORTS:
 *     refinementValidationUnit
 *
 * CONSUMED_BY:
 *     validation/conformance tooling
 *     grammar tests
 *     validation test harness
 *
 * AST_OWNER:
 *     existing domain-neutral frontend AST
 *
 * SEMANTIC_OWNER:
 *     semantic refinement/type/contract analysis
 *
 * IR_OWNER:
 *     canonical semantic model and applicable domain IR
 *
 * TEST_OWNER:
 *     grammar/tests/validation/refinement/
 *     validation conformance harness
 *
 * SPEC_OWNER:
 *     grammar/specification/
 *     grammar/spec/
 *     type/refinement specifications
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 * [ ] It is a parser grammar.
 *
 * [ ] Its grammar name is `RefinementValidation`.
 *
 * [ ] `tokenVocab = ZamaniLexer` is used.
 *
 * [ ] `ContractStatements` is imported.
 *
 * [ ] `refinementValidationUnit` is the complete-input entry point.
 *
 * [ ] The entry point requires EOF.
 *
 * [ ] Canonical property syntax is reused rather than duplicated.
 *
 * [ ] No `refinementStatement` is invented here.
 *
 * [ ] No lexer rule is defined here.
 *
 * [ ] No expression grammar is duplicated here.
 *
 * [ ] No type grammar is duplicated here.
 *
 * [ ] No semantic predicates are used.
 *
 * [ ] No embedded Rust is used.
 *
 * [ ] No executable actions are used.
 *
 * [ ] No resource allocation is performed.
 *
 * [ ] No hardware selection is performed.
 *
 * [ ] No target-specific syntax is introduced.
 *
 * [ ] No quantum operation catalogue is introduced.
 *
 * [ ] No second quantum IR is introduced.
 *
 * [ ] AST ownership remains downstream.
 *
 * [ ] Semantic refinement ownership remains downstream.
 *
 * [ ] Type/refinement ownership remains downstream.
 *
 * [ ] Contract ownership remains canonical.
 *
 * [ ] Resource/capability ownership remains downstream.
 *
 * [ ] Effect ownership remains downstream.
 *
 * [ ] Policy ownership remains downstream.
 *
 * [ ] Provenance remains available downstream.
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
 * [ ] Portability tests exist.
 *
 * [ ] Cross-domain tests exist.
 *
 * [ ] Compatibility tests exist.
 *
 * [ ] Rust 1.97 integration passes.
 *
 * [ ] Rust 1.97.1 integration passes.
 *
 * [ ] The consuming implementation remains safe Rust.
 *
 * ============================================================================
 * CRITICAL ARCHITECTURAL GUARANTEE
 * ============================================================================
 *
 * This file establishes exactly one rule boundary:
 *
 *     validation entry point
 *             |
 *             v
 *     canonical property syntax
 *             |
 *             v
 *     domain-neutral AST
 *             |
 *             v
 *     semantic refinement
 *
 * It does NOT establish a second refinement language.
 *
 * That distinction is essential for POCO-REAF.
 *
 * ============================================================================
 */

parser grammar RefinementValidation;

options {
    tokenVocab = ZamaniLexer;
}

import ContractStatements;

/*
 * ============================================================================
 * COMPLETE REFINEMENT VALIDATION UNIT
 * ============================================================================
 *
 * `propertyStatement` is the canonical source-level representation from which
 * semantic refinement relationships are derived.
 *
 * EOF is mandatory so this validation entry point cannot accept a valid
 * property followed by silently ignored source.
 *
 * The production Zamani parser continues to use the canonical program root.
 */
refinementValidationUnit
    : propertyStatement EOF
    ;
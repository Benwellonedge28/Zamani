/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/validation/proofs.g4
 *
 * GRAMMAR
 * -------
 * ProofsValidation
 *
 * STATUS
 * ------
 * Production validation/conformance facade
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file provides the isolated validation/conformance entry point for
 * source constructs that may become formal verification or proof obligations.
 *
 * IMPORTANT:
 *
 * This file is NOT a second owner of proof syntax.
 *
 * The canonical source-language syntax remains owned by the canonical
 * statement/contract grammar.
 *
 * At the current grammar architecture, a proof obligation is represented by
 * an ordinary semantic property:
 *
 *     property(condition);
 *
 * or another canonical contract/property form where the surrounding semantic
 * context determines that formal verification is required.
 *
 * Therefore this file does NOT introduce:
 *
 *     prove(...)
 *     proof(...)
 *     theorem(...)
 *     witness(...)
 *     solver(...)
 *     tactic(...)
 *     axiom(...)
 *
 * as new core source syntax.
 *
 * Such syntax, if ever required, MUST first be specified and implemented
 * through the canonical lexer, AST, semantic model, and canonical grammar
 * owner before a validation facade consumes it.
 *
 * ============================================================================
 * ARCHITECTURAL PRINCIPLE
 * ============================================================================
 *
 * Validation observes canonical source syntax.
 *
 * Validation MUST NOT become another language grammar.
 *
 * Consequently this file:
 *
 *     - does not define proof syntax;
 *     - does not define theorem syntax;
 *     - does not define solver syntax;
 *     - does not define tactic syntax;
 *     - does not define witness syntax;
 *     - does not define axioms;
 *     - does not define assertions;
 *     - does not define properties;
 *     - does not define expressions;
 *     - does not define types;
 *     - does not define contracts;
 *     - does not define capabilities;
 *     - does not define resources;
 *     - does not define effects;
 *     - does not define policies;
 *     - does not define provenance;
 *     - does not define quantum syntax;
 *     - does not define HDL syntax;
 *     - does not define hardware syntax;
 *     - does not define AI syntax;
 *     - does not define distributed syntax;
 *     - does not define runtime behavior;
 *     - does not define verification algorithms;
 *     - does not define theorem provers;
 *     - does not define SMT/SAT engines;
 *     - does not define model checkers;
 *     - does not define proof-producing compilers;
 *     - does not define IR;
 *     - does not select a backend.
 *
 * ============================================================================
 * IMPLEMENTATION BASELINE
 * ============================================================================
 *
 * Rust:
 *
 *     Rust 1.97
 *     Rust 1.97.1
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
 *     - no semantic actions;
 *     - no semantic predicates;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware access;
 *     - no runtime execution;
 *     - no resource allocation;
 *     - no target selection;
 *     - no backend selection;
 *     - no machine-capacity constants.
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * Provide one complete-input validation boundary for a canonical property
 * that may be consumed by formal verification infrastructure.
 *
 * A syntactically valid property is NOT automatically a mathematical proof.
 *
 * The pipeline is:
 *
 *     canonical property
 *          |
 *          v
 *     AST
 *          |
 *          v
 *     semantic property
 *          |
 *          v
 *     proof obligation
 *          |
 *          v
 *     verification/proof subsystem
 *
 * This grammar owns only the first validation boundary.
 *
 * ============================================================================
 * OWNS
 * ============================================================================
 *
 * This file owns only:
 *
 *     proofValidationUnit
 *
 * This rule is a validation-only complete-input entry point.
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does not own:
 *
 *     propertyStatement
 *     contractStatement
 *     requiresStatement
 *     ensuresStatement
 *     invariantStatement
 *     assumeStatement
 *     guaranteeStatement
 *     expression
 *     statement composition
 *     lexer tokens
 *     keyword spelling
 *     punctuation
 *     AST structures
 *     semantic properties
 *     proof obligations
 *     theorem representations
 *     proof representations
 *     proof strategies
 *     verification algorithms
 *     solver configuration
 *     witness generation
 *     proof checking
 *     proof storage
 *     proof certificates
 *     resource analysis
 *     capability resolution
 *     policy evaluation
 *     provenance implementation
 *     IR generation
 *     runtime verification
 *     quantum realization
 *     HDL realization
 *     hardware realization.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON
 * ----------
 *
 *     grammar/statements/contract.g4
 *
 *         ContractStatements
 *
 *         propertyStatement
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 *         indirectly through tokenVocab = ZamaniLexer
 *
 *     grammar/expressions/
 *
 *         indirectly through ContractStatements
 *
 * ============================================================================
 * EXPORTS
 * ============================================================================
 *
 *     proofValidationUnit
 *
 * This is a validation-only parser entry point.
 *
 * It is NOT a production source-language construct.
 *
 * ============================================================================
 * CONSUMED_BY
 * ============================================================================
 *
 * This facade may be consumed by:
 *
 *     grammar/tests/
 *     grammar/validation/
 *     grammar-validation tooling
 *     parser conformance tooling
 *     verification-boundary tests
 *     IDE/parser diagnostics
 *     grammar regression infrastructure
 *
 * It MUST NOT replace the production program parser.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar creates parser contexts only.
 *
 * The AST remains owned by the existing domain-neutral frontend AST
 * subsystem.
 *
 * A validation parse MUST map to the same canonical AST representation as a
 * production parse.
 *
 * There must not be a validation-specific proof AST merely because the input
 * was parsed through this facade.
 *
 * Conceptually:
 *
 *     propertyStatement
 *          |
 *          v
 *     canonical AST property/contract node
 *          |
 *          v
 *     semantic proof obligation
 *
 * The semantic layer may subsequently represent:
 *
 *     claim
 *     assumptions
 *     premises
 *     predicate
 *     conclusion
 *     evidence
 *     obligations
 *     dependencies
 *     provenance
 *
 * but those are semantic structures, not parser-owned structures.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * A syntactically valid property may become a proof obligation according to
 * its semantic context.
 *
 * Semantic analysis determines:
 *
 *     - whether the property is well typed;
 *     - whether names resolve;
 *     - whether referenced values exist;
 *     - whether assumptions are valid;
 *     - whether the property is applicable;
 *     - whether formal verification is required;
 *     - whether proof is optional;
 *     - whether runtime checking is sufficient;
 *     - whether static verification is possible;
 *     - whether a proof obligation can be discharged;
 *     - whether evidence is sufficient;
 *     - whether a policy permits the requested verification mode.
 *
 * This grammar determines none of those things.
 *
 * ============================================================================
 * PROOF OBLIGATION MODEL
 * ============================================================================
 *
 * The semantic proof model SHOULD remain domain-neutral.
 *
 * Conceptually:
 *
 *     ProofObligation
 *     {
 *         claim
 *         assumptions
 *         premises
 *         context
 *         evidence
 *         dependencies
 *         verification_mode
 *         status
 *         provenance
 *     }
 *
 * A proof obligation may originate from:
 *
 *     property
 *     requires
 *     ensures
 *     invariant
 *     guarantee
 *     refinement
 *     type constraints
 *     resource constraints
 *     effect constraints
 *     security constraints
 *     hardware properties
 *     quantum correctness conditions
 *     HDL verification properties
 *     distributed invariants
 *     AI/model constraints
 *
 * The grammar does not need a separate rule for each origin.
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * Proof conditions are ordinary Zamani expressions.
 *
 * This file does not introduce:
 *
 *     ProofBoolean
 *     TheoremBoolean
 *     QuantumProofBoolean
 *     HardwareProofBoolean
 *     AIProofBoolean
 *
 * or any other proof-specific type.
 *
 * Type analysis determines whether the semantic property is well formed.
 *
 * Refinement, dependent-type, generic, linear, affine, associated-type and
 * other type-system facilities remain owned by the corresponding type
 * subsystem.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Parsing a proof-validation input has no runtime effect.
 *
 * Proof verification may have semantic/tooling effects, but those effects are
 * outside this grammar.
 *
 * Verification infrastructure may interact with existing effect categories
 * such as:
 *
 *     IO
 *     network
 *     native
 *     foreign
 *     distributed
 *     randomness
 *     measurement
 *     simulation
 *     code_generation
 *     reflection
 *
 * The grammar does not create a proof-specific effect system.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Formal verification may require capabilities supplied by the semantic and
 * tooling layers.
 *
 * Examples include conceptual capabilities such as:
 *
 *     capability("verification")
 *     capability("formal.verification")
 *     capability("theorem.proving")
 *     capability("model.checking")
 *
 * These are semantic capabilities.
 *
 * This grammar:
 *
 *     - does not define the capabilities;
 *     - does not require a particular verifier;
 *     - does not select a solver;
 *     - does not inspect hardware;
 *     - does not authorize execution.
 *
 * A capability requirement may be expressed through ordinary canonical
 * Zamani expressions or resource/contract mechanisms.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Verification can consume computational resources.
 *
 * Resource requirements remain outside this grammar.
 *
 * A semantic verification system may account for:
 *
 *     memory
 *     computation
 *     parallelism
 *     accelerator availability
 *     distributed capacity
 *     specialized verification capability
 *     quantum simulation capability
 *
 * This grammar introduces no fixed verification limit.
 *
 * In particular, it does not define:
 *
 *     maximum proof size
 *     maximum theorem size
 *     maximum number of premises
 *     maximum number of obligations
 *     maximum solver depth
 *     maximum variables
 *     maximum nodes
 *     maximum qubits
 *     maximum CPUs
 *     maximum GPUs
 *     maximum FPGAs
 *     maximum memory
 *     maximum threads
 *     maximum tensor rank
 *     maximum network size.
 *
 * Any practical verification budget belongs to the implementation/tooling
 * layer and is not a language ceiling.
 *
 * ============================================================================
 * CONTRACT CONTRACT
 * ============================================================================
 *
 * Proof obligations may originate from the canonical contract family:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * The canonical source syntax remains owned by:
 *
 *     grammar/statements/contract.g4
 *
 * This file does not duplicate any of those productions.
 *
 * In particular:
 *
 *     propertyStatement
 *         |
 *         v
 *     proofValidationUnit
 *
 * is a validation relationship, not a new source-language statement.
 *
 * ============================================================================
 * EVIDENCE CONTRACT
 * ============================================================================
 *
 * Proof verification may consume evidence.
 *
 * Evidence may conceptually include:
 *
 *     derivations
 *     verified transformations
 *     trusted axioms
 *     assumptions
 *     certificates
 *     solver results
 *     model-checking results
 *     test evidence
 *     symbolic evidence
 *     execution evidence
 *     domain-specific verification evidence.
 *
 * Evidence ownership belongs to:
 *
 *     grammar/validation/evidence.g4
 *     semantic evidence infrastructure
 *     provenance infrastructure
 *     verification tooling
 *
 * This grammar does not encode evidence formats.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Proof-related validation MUST preserve enough source provenance to connect
 * a verification result back to the originating source construct.
 *
 * At minimum, downstream infrastructure should preserve:
 *
 *     source file/module
 *     source span
 *     source order
 *     enclosing declaration
 *     property/contract kind
 *     originating expression
 *     language version
 *     grammar version where applicable.
 *
 * Later stages may attach:
 *
 *     derived_from
 *     generated_by
 *     transformed_by
 *     verified_by
 *     evidence
 *     proof strategy
 *     verification result
 *     tool/version
 *     decision
 *     execution context.
 *
 * This grammar does not implement provenance storage.
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Verification may be controlled by policies concerning:
 *
 *     trust
 *     reproducibility
 *     acceptable verification methods
 *     external tools
 *     native execution
 *     network access
 *     solver use
 *     proof caching
 *     certificate acceptance
 *     runtime checking
 *     deployment requirements.
 *
 * Policy evaluation belongs downstream.
 *
 * A parsed property MUST NOT automatically authorize any verification action.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar produces NO IR.
 *
 * The architecture remains:
 *
 *     source
 *         |
 *         v
 *     lexer
 *         |
 *         v
 *     parser
 *         |
 *         v
 *     domain-neutral AST
 *         |
 *         v
 *     structural validation
 *         |
 *         v
 *     semantic analysis
 *         |
 *         +--> types
 *         +--> effects
 *         +--> capabilities
 *         +--> resources
 *         +--> contracts
 *         +--> policies
 *         +--> evidence
 *         +--> provenance
 *         |
 *         v
 *     proof obligation
 *         |
 *         v
 *     verification/proof subsystem
 *         |
 *         v
 *     canonical semantic representation
 *         |
 *         +--> classical representation
 *         +--> quantum::ir
 *         +--> HDL/hardware representation
 *         +--> distributed representation
 *         +--> accelerator representation
 *         +--> future domain representation
 *
 * Proof validation MUST NOT create a second canonical IR.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * A property may constrain quantum computation.
 *
 * Examples of semantic subjects include:
 *
 *     state validity
 *     measurement correctness
 *     circuit equivalence
 *     observable relationships
 *     resource requirements
 *     resilience conditions
 *     error-correction properties
 *     hybrid classical/quantum invariants.
 *
 * The grammar does not determine whether a property is quantum.
 *
 * It MUST NOT:
 *
 *     - enumerate quantum gates;
 *     - enumerate physical qubits;
 *     - encode coupling maps;
 *     - encode calibration;
 *     - perform decomposition;
 *     - perform routing;
 *     - perform scheduling;
 *     - implement QEC;
 *     - select a QPU;
 *     - create another quantum IR.
 *
 * When the semantic subject is quantum, the canonical boundary remains:
 *
 *     domain-neutral AST
 *         |
 *         v
 *     quantum semantic model
 *         |
 *         v
 *     quantum::ir
 *         |
 *         v
 *     optimization
 *         |
 *         v
 *     decomposition
 *         |
 *         v
 *     routing
 *         |
 *         v
 *     scheduling
 *         |
 *         v
 *     resilience / QEC
 *         |
 *         v
 *     ZQN
 *         |
 *         v
 *     HAL
 *         |
 *         v
 *     target realization
 *
 * Proof information may constrain or verify these stages, but does not replace
 * them.
 *
 * ============================================================================
 * HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * Proof obligations may concern:
 *
 *     signal relationships
 *     timing properties
 *     state-machine properties
 *     interface correctness
 *     protocol correctness
 *     synthesis invariants
 *     hardware safety properties
 *     resource-independent design properties.
 *
 * This grammar does not encode:
 *
 *     fixed signal widths
 *     fixed register counts
 *     fixed memory sizes
 *     fixed FPGA dimensions
 *     fixed ASIC resources
 *     fixed pipeline depth
 *     fixed device counts.
 *
 * Hardware verification remains downstream.
 *
 * ============================================================================
 * CLASSICAL BOUNDARY
 * ============================================================================
 *
 * Classical correctness properties use the same proof-validation boundary.
 *
 * No separate classical proof grammar is required.
 *
 * The same property representation may be consumed by:
 *
 *     type checking
 *     symbolic analysis
 *     static verification
 *     testing
 *     formal methods
 *     optimization validation
 *     compiler correctness infrastructure.
 *
 * ============================================================================
 * HYBRID BOUNDARY
 * ============================================================================
 *
 * Hybrid programs may produce proof obligations relating:
 *
 *     classical values
 *     quantum states
 *     measurements
 *     control flow
 *     resources
 *     capabilities
 *     execution state
 *     AI/model results.
 *
 * The grammar remains unchanged.
 *
 * The semantic system determines the relevant domains.
 *
 * ============================================================================
 * AI / REASONING BOUNDARY
 * ============================================================================
 *
 * Proof obligations may concern:
 *
 *     reasoning results
 *     learned-model constraints
 *     adaptation constraints
 *     evidence requirements
 *     uncertainty properties
 *     decision constraints
 *     knowledge consistency.
 *
 * The grammar does not introduce separate AI proof syntax.
 *
 * Formal verification of AI/model behavior remains a semantic/tooling concern.
 *
 * ============================================================================
 * DISTRIBUTED BOUNDARY
 * ============================================================================
 *
 * Proof obligations may describe:
 *
 *     distributed invariants
 *     consistency
 *     ordering
 *     protocol correctness
 *     fault tolerance
 *     communication properties
 *     replication properties.
 *
 * The grammar does not encode:
 *
 *     node counts
 *     topology
 *     machine identifiers
 *     deployment placement.
 *
 * Those are resource, capability, execution, networking, and backend concerns.
 *
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * Structural diagnostics include malformed canonical property syntax, for
 * example:
 *
 *     property
 *     property()
 *     property(
 *     property(condition
 *     property(condition, extra)
 *     property(condition) trailing
 *
 * Whether a syntactically valid property can be formally verified is NOT a
 * parser diagnostic.
 *
 * Semantic/verification diagnostics may include:
 *
 *     unresolved name
 *     invalid expression
 *     invalid property context
 *     invalid predicate type
 *     contradictory assumptions
 *     unsatisfied obligation
 *     unavailable verification capability
 *     unavailable resource
 *     unsupported verification method
 *     insufficient evidence
 *     policy violation
 *     failed proof
 *     inconclusive verification
 *     unverifiable obligation.
 *
 * These belong to downstream semantic and verification systems.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * This file deliberately introduces no new source keyword.
 *
 * Existing canonical property syntax remains owned by:
 *
 *     grammar/statements/contract.g4
 *
 * Therefore a future explicit proof construct MUST follow:
 *
 *     specification
 *         |
 *         v
 *     lexer
 *         |
 *         v
 *     canonical grammar owner
 *         |
 *         v
 *     AST
 *         |
 *         v
 *     semantic proof model
 *         |
 *         v
 *     validation facade
 *
 * Never:
 *
 *     validation facade
 *         |
 *         v
 *     private proof syntax
 *
 * This prevents grammar drift.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This file defines no finite language capacity.
 *
 * There is no grammar-level maximum for:
 *
 *     proof obligations
 *     properties
 *     premises
 *     expression size
 *     source-unit size
 *     module count
 *     function count
 *     type count
 *     quantum operations
 *     qubits
 *     processors
 *     accelerators
 *     devices
 *     nodes
 *     memory
 *     tensor rank
 *     network size.
 *
 * Repetition and composition remain structural.
 *
 * "Infinity" means that this grammar introduces no artificial finite ceiling.
 *
 * Actual feasibility remains subject to:
 *
 *     implementation resources
 *     compiler resources
 *     verification resources
 *     runtime resources
 *     target resources
 *     physical resources
 *     explicit policies
 *     explicit budgets.
 *
 * Those practical constraints MUST NOT become hidden language constants.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Proof-validation syntax must be independent of target realization.
 *
 * The same source property must remain syntactically valid when the enclosing
 * program is considered for:
 *
 *     tiny systems
 *     embedded systems
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
 *     distributed systems
 *     cloud
 *     future computational systems.
 *
 * Target changes MUST NOT require rewriting a property merely because:
 *
 *     memory changes;
 *     processor count changes;
 *     accelerator count changes;
 *     QPU capacity changes;
 *     topology changes;
 *     node count changes;
 *     device availability changes.
 *
 * A target may fail to discharge a proof obligation.
 *
 * That is a verification/resource result, not a source-grammar failure.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     source text
 *     lexer vocabulary
 *     grammar composition
 *     parser configuration.
 *
 * Parsing MUST NOT depend on:
 *
 *     wall-clock time
 *     randomness
 *     hardware availability
 *     filesystem state
 *     network state
 *     runtime state
 *     target selection
 *     scheduler state
 *     verifier availability.
 *
 * Repeated parsing of identical input under identical configuration must
 * produce equivalent parser structure and source spans.
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
 * The consuming implementation must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and use safe Rust only.
 *
 * Verification tooling may be complex, but that tooling is outside this
 * grammar file.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE TESTS
 * --------------
 *
 * The validation suite should include:
 *
 *     property(x > 0);
 *
 *     property(result.is_valid());
 *
 *     property(state.is_consistent());
 *
 *     property(memory >= required_memory);
 *
 *     property(capability("verification"));
 *
 *     property(capability("quantum.measurement"));
 *
 *     property(measurement.is_valid());
 *
 *     property(hardware_state.is_consistent());
 *
 *     property(distributed_state.is_consistent());
 *
 * These remain ordinary canonical properties.
 *
 * NEGATIVE TESTS
 * --------------
 *
 * Include malformed inputs such as:
 *
 *     property
 *     property()
 *     property(
 *     property(x
 *     property(x, y)
 *     property(x) trailing
 *
 * Also explicitly ensure that this validation grammar does NOT silently accept
 * invented proof syntax such as:
 *
 *     prove(x);
 *     proof(x);
 *     theorem(x);
 *
 * unless and until such syntax is introduced through the canonical grammar
 * architecture.
 *
 * BOUNDARY TESTS
 * --------------
 *
 * Include:
 *
 *     deeply nested predicates
 *     large symbolic expressions
 *     generic types
 *     dependent-type predicates
 *     uncertainty predicates
 *     capability predicates
 *     resource predicates
 *     quantum-derived predicates
 *     HDL properties
 *     distributed properties
 *     AI/model properties
 *     hybrid properties.
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
 *     semantic obligation count.
 *
 * The tests must verify that no artificial machine-derived capacity limit is
 * introduced by this grammar.
 *
 * DETERMINISM TESTS
 * -----------------
 *
 * Parse identical source repeatedly and verify equivalent parse output.
 *
 * PORTABILITY TESTS
 * -----------------
 *
 * Parse identical source under different:
 *
 *     target descriptions
 *     resource inventories
 *     processor counts
 *     accelerator inventories
 *     device counts
 *     hardware topologies.
 *
 * Parser output must remain source-dependent rather than target-dependent.
 *
 * CROSS-DOMAIN TESTS
 * ------------------
 *
 * Cover:
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
 *     embedded.
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
 *     proof-engine-specific syntax
 *     solver-specific syntax.
 *
 * Numeric literals appearing inside canonical expressions remain ordinary
 * program semantics and are not implementation ceilings.
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
 * Specifically:
 *
 *     propertyStatement
 *
 * AST
 * ---
 *
 * Existing domain-neutral frontend AST.
 *
 * SEMANTICS
 * ---------
 *
 * Semantic property/contract analysis determines whether a property becomes
 * a proof obligation.
 *
 * VERIFICATION
 * ------------
 *
 * Formal proof and verification infrastructure consumes semantic proof
 * obligations.
 *
 * TYPES
 * -----
 *
 * Integrates with:
 *
 *     grammar/types/
 *
 * for type and refinement semantics.
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
 *     grammar/validation/refinement.g4
 *
 * without duplicating their syntax.
 *
 * EVIDENCE
 * --------
 *
 * Integrates with:
 *
 *     grammar/validation/evidence.g4
 *
 * for validation/evidence boundaries.
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
 * Provenance remains owned by the AST/semantic/compiler pipeline.
 *
 * IR
 * --
 *
 * No IR is created here.
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
 * Proof information may participate in:
 *
 *     type checking
 *     constraint solving
 *     verification
 *     optimization validation
 *     specialization
 *     resource analysis
 *     code generation
 *     runtime checking.
 *
 * Each operation remains owned by its corresponding subsystem.
 *
 * RUNTIME
 * -------
 *
 * Runtime proof checking, when selected by semantic policy, remains downstream.
 *
 * ============================================================================
 * OWNERSHIP SUMMARY
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     ZamaniLexer
 *     ContractStatements
 *     canonical expression grammar
 *     semantic property/contract model
 *     verification subsystem
 *
 * EXPORTS:
 *
 *     proofValidationUnit
 *
 * CONSUMED_BY:
 *
 *     validation/conformance tooling
 *     grammar tests
 *     verification-boundary test harness
 *
 * AST_OWNER:
 *
 *     existing domain-neutral frontend AST
 *
 * SEMANTIC_OWNER:
 *
 *     semantic property/contract analysis
 *     formal verification subsystem
 *
 * IR_OWNER:
 *
 *     canonical semantic model and applicable domain IR
 *
 * TEST_OWNER:
 *
 *     grammar/tests/validation/proofs/
 *     validation conformance harness
 *
 * SPEC_OWNER:
 *
 *     grammar/specification/
 *     grammar/spec/
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 * [ ] It is a parser grammar.
 *
 * [ ] Its grammar name is `ProofsValidation`.
 *
 * [ ] `tokenVocab = ZamaniLexer` is used.
 *
 * [ ] `ContractStatements` is imported.
 *
 * [ ] `proofValidationUnit` is the complete-input entry point.
 *
 * [ ] The entry point requires EOF.
 *
 * [ ] Canonical property syntax is reused rather than duplicated.
 *
 * [ ] No `proofStatement` is invented here.
 *
 * [ ] No `proveStatement` is invented here.
 *
 * [ ] No theorem syntax is invented here.
 *
 * [ ] No solver syntax is invented here.
 *
 * [ ] No tactic syntax is invented here.
 *
 * [ ] No witness syntax is invented here.
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
 * [ ] Semantic proof ownership remains downstream.
 *
 * [ ] Verification ownership remains downstream.
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
 * This file establishes exactly one boundary:
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
 *     semantic proof obligation
 *             |
 *             v
 *     verification subsystem
 *
 * It does NOT establish a second proof language.
 *
 * This distinction is essential for POCO-REAF.
 *
 * ============================================================================
 */

parser grammar ProofsValidation;

options {
    tokenVocab = ZamaniLexer;
}

import ContractStatements;

/*
 * ============================================================================
 * COMPLETE PROOF VALIDATION UNIT
 * ============================================================================
 *
 * `propertyStatement` is the canonical source-level construct that can be
 * promoted by semantic analysis into a formal proof obligation.
 *
 * EOF is mandatory so this validation entry point cannot accept a valid
 * property followed by silently ignored source.
 *
 * The production Zamani parser continues to use the canonical program root.
 *
 * Formal proof generation/checking is performed downstream.
 */
proofValidationUnit
    : propertyStatement EOF
    ;
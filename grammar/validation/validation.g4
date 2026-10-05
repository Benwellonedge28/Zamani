/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/validation/validation.g4
 *
 * GRAMMAR
 * -------
 * Validation
 *
 * STATUS
 * ------
 * PRODUCTION VALIDATION ORCHESTRATOR
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar is the composition/orchestration boundary for the parser-level
 * validation grammars under:
 *
 *     grammar/validation/
 *
 * It provides ONE stable validation entry point:
 *
 *     validationUnit
 *
 * for validation tooling that needs to validate one complete validation
 * construct at a time.
 *
 * IMPORTANT:
 *
 * This grammar is NOT a second language grammar.
 *
 * It does not become the owner of:
 *
 *     assertions
 *     assumptions
 *     contracts
 *     requires
 *     ensures
 *     invariants
 *     guarantees
 *     properties
 *     evidence subjects
 *     preconditions
 *     postconditions
 *     proofs
 *     refinements
 *     requirements
 *     resources
 *     expressions
 *     identifiers
 *     literals
 *     operators
 *     punctuation
 *
 * Those remain owned by the canonical grammars already established in the
 * repository.
 *
 * This file only composes their validation-facing boundaries.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *                         Zamani source
 *                              |
 *                              v
 *                         Zamani.g4
 *                              |
 *                              v
 *                       ZamaniParser.g4
 *                              |
 *             +----------------+----------------+
 *             |                                 |
 *             v                                 v
 *       production syntax                 validation tooling
 *             |                                 |
 *             v                                 v
 *       canonical rules                 validation/validation.g4
 *             |                                 |
 *             |                  +--------------+--------------+
 *             |                  |              |              |
 *             |                  v              v              v
 *             |             assertions     contracts       evidence
 *             |             assumptions    properties      proofs
 *             |             requirements   refinement      resources
 *             |                  |              |              |
 *             +------------------+--------------+--------------+
 *                                |
 *                                v
 *                         semantic validation
 *                                |
 *             +------------------+------------------+
 *             |          |         |       |        |
 *             v          v         v       v        v
 *           types     effects   resources policies provenance
 *                                |
 *                                v
 *                         canonical semantics
 *                                |
 *                         +------+------+
 *                         |             |
 *                         v             v
 *                    classical      quantum::ir
 *                         |             |
 *                         +------+------+
 *                                |
 *                                v
 *                           compiler
 *                                |
 *                     lowering / routing /
 *                     scheduling / resilience
 *                                |
 *                                v
 *                              HAL
 *                                |
 *                                v
 *                           target
 *
 * ============================================================================
 * AUTHORITY HIERARCHY
 * ============================================================================
 *
 * Validation does not change language authority.
 *
 * The repository authority remains:
 *
 *     grammar/DESIGN.md
 *             |
 *             v
 *     grammar/specification/
 *             |
 *             v
 *     grammar/spec/
 *             |
 *             v
 *     grammar/Zamani.g4
 *             |
 *             v
 *     grammar/antlr/
 *             |
 *             v
 *     canonical modular parser grammars
 *             |
 *             v
 *     src/lexer.rs
 *     src/parser.rs
 *     src/ast/
 *             |
 *             v
 *     semantic implementation
 *             |
 *             v
 *     canonical IR
 *
 * Validation grammars observe this hierarchy.
 *
 * They do not promote proposed syntax into language syntax merely by importing
 * it here.
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * Provide a single parser-level orchestration boundary for isolated
 * validation/conformance inputs.
 *
 * OWNED BY THIS FILE
 * ------------------
 *
 *     validationUnit
 *     validationCase
 *
 * These are validation orchestration rules only.
 *
 * DOES NOT OWN
 * -------------
 *
 *     assertionStatement
 *     assumeStatement
 *     requiresStatement
 *     ensuresStatement
 *     invariantStatement
 *     guaranteeStatement
 *     propertyStatement
 *     expression
 *     requirementDeclaration
 *     resourceRequirement
 *
 * Those remain owned by canonical source/domain grammars.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON
 * ----------
 *
 *     AssertionsValidation
 *     Assumptions
 *     ContractValidation
 *     Ensures
 *     EvidenceValidation
 *     GuaranteesValidation
 *     Invariants
 *     Postconditions
 *     PreconditionsValidation
 *     ProofsValidation
 *     PropertiesValidation
 *     RefinementValidation
 *     RequiresValidation
 *
 * These grammars remain independently responsible for their own validation
 * contracts.
 *
 * EXPORTS
 * -------
 *
 *     validationUnit
 *     validationCase
 *
 * CONSUMED_BY
 * -----------
 *
 *     grammar/tests/
 *     grammar-validation tooling
 *     parser conformance tooling
 *     IDE/parser validation tooling
 *     CI grammar validation
 *     regression tests
 *
 * AST_OWNER
 * ---------
 *
 * Repository frontend AST subsystem:
 *
 *     src/ast/
 *
 * This file does not define AST nodes.
 *
 * SEMANTIC_OWNER
 * --------------
 *
 * Repository semantic-analysis subsystem.
 *
 * This file does not decide semantic validity.
 *
 * IR_OWNER
 * --------
 *
 * Canonical semantic/IR subsystems.
 *
 * For quantum computation the canonical boundary remains:
 *
 *     quantum::ir
 *
 * TEST_OWNER
 * ----------
 *
 *     grammar/tests/
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
 * The canonical lexer is:
 *
 *     ZamaniLexer
 *
 * No lexer rules are declared here.
 *
 * No keywords are declared here.
 *
 * No operators are declared here.
 *
 * No punctuation is declared here.
 *
 * The validation orchestrator therefore cannot accidentally create a second
 * lexical vocabulary.
 *
 * ============================================================================
 * PARSER CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * It consumes the canonical:
 *
 *     ZamaniLexer
 *
 * through tokenVocab.
 *
 * Every validation case is delegated to an already-existing validation facade
 * or to the canonical validation subject where an existing facade deliberately
 * exposes only a complete-input entry point.
 *
 * ============================================================================
 * WHY THIS FILE DOES NOT SIMPLY IMPORT EVERY `*ValidationUnit`
 * ============================================================================
 *
 * Several existing specialized validation grammars intentionally expose
 * complete-input units of their own.
 *
 * Some of those units accept zero or more items, for example:
 *
 *     assumptionsValidationUnit
 *     contractValidationUnit
 *     invariantValidationUnit
 *
 * Such a rule is appropriate when validating a sequence in isolation.
 *
 * It is NOT appropriate to use it directly as the universal orchestration
 * alternative because an empty input could then become a successful validation
 * result.
 *
 * The orchestrator therefore consumes the non-empty validation-facing item
 * boundaries wherever those are available.
 *
 * This preserves:
 *
 *     one construct
 *     followed by
 *     exactly one EOF
 *
 * at the orchestration boundary.
 *
 * ============================================================================
 * COMPLETE-INPUT INVARIANT
 * ============================================================================
 *
 * The public entry point is:
 *
 *     validationUnit
 *
 * and it requires:
 *
 *     validationCase EOF
 *
 * Therefore:
 *
 *     validation input
 *             |
 *             v
 *     exactly one recognized validation case
 *             |
 *             v
 *            EOF
 *
 * A valid prefix followed by unrelated input MUST NOT be accepted.
 *
 * Examples that must fail:
 *
 *     requires(x);
 *     trailing
 *
 *     property(x);
 *     unexpected
 *
 *     assert(x);
 *     trailing
 *
 * ============================================================================
 * VALIDATION CASES
 * ============================================================================
 *
 * The orchestrator covers the current parser-level validation domains:
 *
 *     assertions
 *     assumptions
 *     contracts
 *     ensures/postconditions
 *     evidence subjects
 *     guarantees
 *     invariants
 *     preconditions
 *     properties
 *     requirements
 *     resource requirements
 *     proofs
 *     refinements
 *
 * The individual validation files remain the detailed owners.
 *
 * ============================================================================
 * ASSERTIONS
 * ============================================================================
 *
 * Owner:
 *
 *     grammar/validation/assertions.g4
 *
 * Canonical source owner:
 *
 *     grammar/statements/assertions.g4
 *
 * This orchestrator delegates to:
 *
 *     assertionsValidationItem
 *
 * No assertion syntax is reproduced here.
 *
 * ============================================================================
 * ASSUMPTIONS
 * ============================================================================
 *
 * Owner:
 *
 *     grammar/validation/assumptions.g4
 *
 * Canonical source owner:
 *
 *     grammar/statements/contract.g4
 *
 * This orchestrator delegates to:
 *
 *     assumptionsValidationItem
 *
 * ============================================================================
 * CONTRACTS
 * ============================================================================
 *
 * Owner:
 *
 *     grammar/validation/contracts.g4
 *
 * Canonical source owner:
 *
 *     grammar/statements/contract.g4
 *
 * This orchestrator delegates to:
 *
 *     contractValidationItem
 *
 * The individual contract kinds remain:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * No duplicate definitions occur here.
 *
 * ============================================================================
 * ENSURES
 * ============================================================================
 *
 * Owner:
 *
 *     grammar/validation/ensures.g4
 *
 * Canonical source owner:
 *
 *     grammar/statements/contract.g4
 *
 * This orchestrator delegates to:
 *
 *     ensuresValidationItem
 *
 * ============================================================================
 * EVIDENCE
 * ============================================================================
 *
 * Owner:
 *
 *     grammar/validation/evidence.g4
 *
 * Evidence is deliberately not introduced as a new source-language keyword
 * or a new evidence-specific expression grammar.
 *
 * The existing evidence validation facade selects canonical source constructs
 * that can acquire semantic evidence:
 *
 *     assertion
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * This orchestrator delegates to:
 *
 *     evidenceValidationSubject
 *
 * ============================================================================
 * GUARANTEES
 * ============================================================================
 *
 * Owner:
 *
 *     grammar/validation/guarantees.g4
 *
 * Canonical source owner:
 *
 *     grammar/statements/contract.g4
 *
 * Delegation:
 *
 *     guaranteesValidationItem
 *
 * ============================================================================
 * INVARIANTS
 * ============================================================================
 *
 * Owner:
 *
 *     grammar/validation/invariants.g4
 *
 * Canonical source owner:
 *
 *     grammar/statements/contract.g4
 *
 * Delegation:
 *
 *     invariantValidationItem
 *
 * ============================================================================
 * POSTCONDITIONS
 * ============================================================================
 *
 * Owner:
 *
 *     grammar/validation/postconditions.g4
 *
 * Canonical source representation:
 *
 *     ensuresStatement
 *
 * Delegation:
 *
 *     postconditionValidationItem
 *
 * No separate POSTCONDITION source syntax is created by this orchestrator.
 *
 * ============================================================================
 * PRECONDITIONS
 * ============================================================================
 *
 * Owner:
 *
 *     grammar/validation/preconditions.g4
 *
 * This facade already integrates:
 *
 *     standalone preconditions
 *     function-contract preconditions
 *
 * Delegation:
 *
 *     preconditionsValidationItem
 *
 * The function-contract syntax remains owned by:
 *
 *     grammar/functions/contracts.g4
 *
 * ============================================================================
 * PROPERTIES
 * ============================================================================
 *
 * Owner:
 *
 *     grammar/validation/properties.g4
 *
 * Canonical source owner:
 *
 *     grammar/statements/contract.g4
 *
 * Delegation:
 *
 *     propertyValidationItem
 *
 * ============================================================================
 * REQUIREMENTS
 * ============================================================================
 *
 * Owner:
 *
 *     grammar/validation/requires.g4
 *
 * The existing facade deliberately distinguishes:
 *
 *     universal requirement declarations
 *     resource requirements
 *
 * This orchestrator exposes both through their existing validation-facing
 * rules.
 *
 * No requirement expression grammar is reproduced here.
 *
 * ============================================================================
 * PROOFS
 * ============================================================================
 *
 * Owner:
 *
 *     grammar/validation/proofs.g4
 *
 * The existing proof validation boundary is:
 *
 *     proofValidationUnit
 *
 * and its canonical semantic subject is:
 *
 *     propertyStatement
 *
 * Formal proof generation/checking is NOT a parser responsibility.
 *
 * It belongs to semantic verification infrastructure.
 *
 * Consequently this orchestrator represents proof validation through the same
 * canonical property source construct rather than inventing:
 *
 *     prove
 *     proof
 *     certificate
 *     witness
 *     solver
 *
 * syntax.
 *
 * This is critical for keeping proof technology replaceable.
 *
 * ============================================================================
 * REFINEMENT
 * ============================================================================
 *
 * Owner:
 *
 *     grammar/validation/refinement.g4
 *
 * Its canonical validation subject is:
 *
 *     propertyStatement
 *
 * Refinement semantics remain downstream.
 *
 * The grammar does not define a second refinement language.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This file creates no AST node.
 *
 * A validation parse must correspond to the same canonical source AST that
 * would be produced by the production parser for the same construct.
 *
 * For example:
 *
 *     property(x > 0);
 *
 * must not produce:
 *
 *     ValidationPropertyNode
 *
 * merely because it was parsed through this file.
 *
 * Instead it must produce the same canonical property representation used by
 * the production frontend.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parser acceptance does not mean semantic acceptance.
 *
 * Validation through this grammar is structural.
 *
 * Downstream semantic validation remains responsible for:
 *
 *     name resolution
 *     scope
 *     typing
 *     effect checking
 *     capability checking
 *     resource checking
 *     contract checking
 *     policy checking
 *     provenance
 *     verification
 *     domain legality
 *     target feasibility
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * Conditions remain ordinary Zamani expressions.
 *
 * This file does not create:
 *
 *     ValidationBoolean
 *     ProofBoolean
 *     QuantumValidationType
 *     HardwareValidationType
 *     AIValidationType
 *
 * Type checking remains owned by the normal type system.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Parsing is effect-free.
 *
 * The expressions contained inside validation constructs may have semantic
 * effects, but those effects are analyzed downstream.
 *
 * Existing effect categories remain outside this grammar, including:
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
 *     code generation
 *     simulation
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * A validation condition may reference a capability:
 *
 *     requires(capability("quantum.measurement"));
 *
 * or other capability-related semantic value.
 *
 * This grammar does not:
 *
 *     discover capabilities;
 *     inspect hardware;
 *     select devices;
 *     authorize execution;
 *     determine target availability.
 *
 * Capability resolution belongs downstream.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Validation constructs may contain symbolic resource requirements.
 *
 * Examples include:
 *
 *     requires(memory >= required_memory);
 *     requires(qubits >= required_qubits);
 *     requires(capability("tensor.compute"));
 *     requires(topology.supports(required_topology));
 *
 * The grammar introduces no physical resource limits.
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Policies may influence validation and verification:
 *
 *     security policy
 *     execution policy
 *     resource policy
 *     verification policy
 *     adaptation policy
 *     simulation policy
 *     deployment policy
 *     reproducibility policy
 *
 * Policy evaluation remains outside this grammar.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * The normal parser source spans must remain available.
 *
 * Downstream provenance may associate a validation subject with:
 *
 *     source file
 *     source span
 *     enclosing declaration
 *     module
 *     validation category
 *     semantic derivation
 *     evidence
 *     verification result
 *     transformation history
 *
 * This grammar does not implement provenance storage.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Validation constructs may constrain or inspect quantum-derived values.
 *
 * For example:
 *
 *     requires(capability("quantum.measurement"));
 *
 *     ensures(measurement.is_valid());
 *
 *     property(result.is_consistent());
 *
 * The grammar does not know whether an expression eventually lowers to
 * quantum computation.
 *
 * The downstream quantum path remains:
 *
 *     source
 *       ->
 *     domain-neutral AST
 *       ->
 *     semantic analysis
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
 * This file introduces no quantum IR.
 *
 * ============================================================================
 * HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * Validation conditions may refer to hardware or HDL semantics.
 *
 * The grammar does not encode:
 *
 *     fixed register widths
 *     fixed bus widths
 *     fixed memory sizes
 *     fixed FPGA dimensions
 *     fixed ASIC resource counts
 *     fixed device counts
 *     fixed clock counts
 *     fixed pipeline depths
 *
 * Hardware feasibility remains a downstream concern.
 *
 * ============================================================================
 * AI / REASONING BOUNDARY
 * ============================================================================
 *
 * The validation system is deliberately domain-neutral and can validate
 * constructs involving:
 *
 *     reasoning
 *     knowledge
 *     learning
 *     adaptation
 *     uncertainty
 *     evidence
 *     decisions
 *     provenance
 *     models
 *     agents
 *
 * No AI-specific proof or validation syntax is introduced here.
 *
 * ============================================================================
 * HYBRID BOUNDARY
 * ============================================================================
 *
 * A single validation construct may semantically constrain a hybrid
 * computation involving:
 *
 *     classical values
 *     quantum results
 *     learned models
 *     measurements
 *     hardware state
 *     distributed state
 *
 * No hybrid-specific validation grammar is necessary.
 *
 * ============================================================================
 * DISTRIBUTED BOUNDARY
 * ============================================================================
 *
 * Validation may concern:
 *
 *     consistency
 *     availability
 *     replication
 *     topology
 *     communication
 *     service state
 *     fault state
 *
 * No finite node/channel/device limit is encoded.
 *
 * ============================================================================
 * PROOF / VERIFICATION BOUNDARY
 * ============================================================================
 *
 * The grammar expresses the source-level subject of verification.
 *
 * It does NOT encode a particular proof technology.
 *
 * Therefore it does not hard-code:
 *
 *     theorem provers
 *     SMT solvers
 *     SAT solvers
 *     model checkers
 *     proof assistants
 *     certificate formats
 *     proof search strategies
 *
 * A property can become a proof obligation downstream.
 *
 * The semantic verification layer determines:
 *
 *     whether it is provable;
 *     how it is verified;
 *     what evidence is required;
 *     what proof artifact is produced;
 *     whether verification is static or dynamic.
 *
 * ============================================================================
 * REFINEMENT BOUNDARY
 * ============================================================================
 *
 * Refinement is a semantic relationship between representations or behaviors.
 *
 * This grammar does not encode:
 *
 *     refinement algorithms
 *     simulation relations
 *     proof calculus
 *     equivalence algorithms
 *     target mappings
 *
 * Those remain semantic/compiler responsibilities.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This file introduces NO language-level finite capacity.
 *
 * In particular, there is no maximum for:
 *
 *     validation constructs
 *     statements
 *     properties
 *     assumptions
 *     assertions
 *     requirements
 *     resources
 *     capabilities
 *     modules
 *     functions
 *     source size
 *     expression size
 *     quantum operations
 *     qubits
 *     processors
 *     GPUs
 *     FPGAs
 *     accelerators
 *     nodes
 *     devices
 *     memory
 *     tensor dimensions
 *     tensor rank
 *     network size
 *
 * The grammar therefore scales structurally from tiny programs to arbitrarily
 * large programs subject to actual parser, compiler, runtime, target and
 * resource availability.
 *
 * "Infinity" here means:
 *
 *     no artificial finite ceiling is encoded by this grammar.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file MUST NOT contain universal capacity constants such as:
 *
 *     MAX_VALIDATIONS
 *     MAX_ASSERTIONS
 *     MAX_PROPERTIES
 *     MAX_REQUIREMENTS
 *     MAX_RESOURCES
 *     MAX_CAPABILITIES
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
 * No such constants are present.
 *
 * Numeric literals appearing inside source expressions remain program
 * semantics and are not interpreted as implementation ceilings.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing through this grammar must depend only on:
 *
 *     source text
 *     canonical lexer
 *     imported parser grammars
 *     parser configuration
 *
 * It must not depend on:
 *
 *     wall-clock time
 *     randomness
 *     hardware availability
 *     filesystem state
 *     network state
 *     target availability
 *     runtime state
 *     scheduler state
 *
 * Repeated parsing of identical input under identical grammar/configuration
 * must produce equivalent structural results.
 *
 * ============================================================================
 * SAFETY CONTRACT
 * ============================================================================
 *
 * This grammar contains no executable actions.
 *
 * It contains:
 *
 *     no Rust;
 *     no semantic predicates;
 *     no filesystem access;
 *     no network access;
 *     no hardware discovery;
 *     no resource allocation;
 *     no target selection;
 *     no runtime execution.
 *
 * The Rust frontend consuming generated parser code must remain compatible
 * with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust Edition 2021
 *
 * and must require safe Rust only.
 *
 * ============================================================================
 * ERROR BOUNDARY
 * ============================================================================
 *
 * Syntax errors are parser errors.
 *
 * Semantic errors are not converted into parser errors merely because the
 * construct is a validation construct.
 *
 * Examples:
 *
 *     requires(unknown_name);
 *
 * may be syntactically valid while failing name resolution.
 *
 * Similarly:
 *
 *     property(value);
 *
 * may require semantic predicate/type validation downstream.
 *
 * ============================================================================
 * POSITIVE TEST CONTRACT
 * ============================================================================
 *
 * The following forms must be represented through the canonical validation
 * facades:
 *
 *     assert(x);
 *
 *     assume(x >= 0);
 *
 *     requires(x >= 0);
 *
 *     ensures(result >= 0);
 *
 *     invariant(state.is_valid());
 *
 *     guarantee(result.is_valid());
 *
 *     property(result.is_consistent());
 *
 *     requires(capability("quantum.measurement"));
 *
 *     requires(memory >= required_memory);
 *
 *     requires(capability("tensor.compute"));
 *
 * ============================================================================
 * NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * The following must not be accepted as complete validation input:
 *
 *     <empty input>
 *
 *     requires(x); trailing
 *
 *     ensures(x); trailing
 *
 *     property(x); trailing
 *
 *     assert(x); trailing
 *
 *     assume(x); trailing
 *
 *     guarantee(x); trailing
 *
 *     invariant(x); trailing
 *
 * ============================================================================
 * BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Validation must be exercised across:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     hardware
 *     AI
 *     distributed
 *     data
 *     networking
 *     security
 *     resource
 *     interoperability
 *
 * without requiring this grammar to acquire domain-specific validation
 * syntax.
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Scalability tests must vary program/input size without changing this file.
 *
 * Test dimensions include:
 *
 *     expression size
 *     nesting depth
 *     source-unit size
 *     module count
 *     declaration count
 *     validation construct count
 *     resource vocabulary size
 *     capability vocabulary size
 *     domain composition
 *
 * The grammar must not introduce a test-specific artificial ceiling.
 *
 * ============================================================================
 * PORTABILITY TEST CONTRACT
 * ============================================================================
 *
 * The same validation source must remain syntactically valid regardless of:
 *
 *     CPU count
 *     GPU availability
 *     FPGA availability
 *     accelerator availability
 *     QPU availability
 *     node count
 *     memory availability
 *     topology
 *     target architecture
 *     deployment environment
 *
 * Semantic validation may correctly report unavailable capabilities or
 * resources downstream.
 *
 * Such a result is not a grammar portability failure.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * This file does not redefine legacy syntax.
 *
 * Compatibility behavior remains owned by:
 *
 *     grammar/compatibility/
 *
 * Deprecated validation forms must be represented by explicit compatibility
 * rules rather than silently accepted here.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * 1. CANONICAL ROOT
 * -----------------
 *
 * grammar/Zamani.g4 remains the complete source grammar root.
 *
 * This file MUST NOT be imported into Zamani.g4 merely to make validation
 * constructs legal in the production parser.
 *
 * Production legality is established through:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * and its canonical modular imports.
 *
 * 2. VALIDATION TOOLING
 * ---------------------
 *
 * Validation tooling may use:
 *
 *     Validation.validationUnit
 *
 * as the single isolated validation entry point.
 *
 * 3. INDIVIDUAL VALIDATION FILES
 * ------------------------------
 *
 * The following remain independent owners:
 *
 *     assertions.g4
 *     assumptions.g4
 *     contracts.g4
 *     ensures.g4
 *     evidence.g4
 *     guarantees.g4
 *     invariants.g4
 *     postconditions.g4
 *     preconditions.g4
 *     proofs.g4
 *     properties.g4
 *     refinement.g4
 *     requires.g4
 *
 * This file orchestrates them without replacing them.
 *
 * 4. CONTRACT SOURCE
 * ------------------
 *
 * Canonical contract syntax remains:
 *
 *     grammar/statements/contract.g4
 *
 * 5. ASSERTION SOURCE
 * -------------------
 *
 * Canonical assertion syntax remains:
 *
 *     grammar/statements/assertions.g4
 *
 * 6. FUNCTION CONTRACTS
 * ---------------------
 *
 * Function contract syntax remains:
 *
 *     grammar/functions/contracts.g4
 *
 * The precondition validation facade already bridges standalone and function
 * contract preconditions.
 *
 * 7. REQUIREMENTS / RESOURCES
 * ---------------------------
 *
 * Universal and resource requirement syntax remains owned by:
 *
 *     grammar/resources/
 *
 * and the canonical requirement grammars imported by
 * `validation/requires.g4`.
 *
 * 8. SEMANTIC VALIDATION
 * ----------------------
 *
 * Parser validation feeds:
 *
 *     AST
 *       ->
 *     structural validation
 *       ->
 *     semantic validation
 *       ->
 *     type/effect/capability/resource/contract/policy validation
 *       ->
 *     provenance
 *       ->
 *     canonical IR
 *
 * 9. QUANTUM
 * ----------
 *
 * Quantum validation does not create a second quantum representation.
 *
 * Applicable semantics eventually use:
 *
 *     quantum::ir
 *
 * 10. HDL / HARDWARE
 * ------------------
 *
 * Validation conditions remain target independent until semantic lowering.
 *
 * 11. AI
 * -----
 *
 * Reasoning, knowledge, learning, adaptation, evidence, uncertainty,
 * explainability and agent semantics remain ordinary semantic domains.
 *
 * They do not require a separate validation language.
 *
 * ============================================================================
 * INTEGRATION MAP
 * ============================================================================
 *
 * validation/validation.g4
 *         |
 *         +--> assertions.g4
 *         |       |
 *         |       +--> grammar/statements/assertions.g4
 *         |
 *         +--> assumptions.g4
 *         |       |
 *         |       +--> grammar/statements/contract.g4
 *         |
 *         +--> contracts.g4
 *         |       |
 *         |       +--> grammar/statements/contract.g4
 *         |
 *         +--> ensures.g4
 *         |       |
 *         |       +--> grammar/statements/contract.g4
 *         |
 *         +--> evidence.g4
 *         |       |
 *         |       +--> assertions
 *         |       +--> contracts
 *         |
 *         +--> guarantees.g4
 *         |       |
 *         |       +--> grammar/statements/contract.g4
 *         |
 *         +--> invariants.g4
 *         |       |
 *         |       +--> grammar/statements/contract.g4
 *         |
 *         +--> postconditions.g4
 *         |       |
 *         |       +--> ensures
 *         |
 *         +--> preconditions.g4
 *         |       |
 *         |       +--> standalone requires
 *         |       +--> function contract requires
 *         |
 *         +--> proofs.g4
 *         |       |
 *         |       +--> property
 *         |
 *         +--> properties.g4
 *         |       |
 *         |       +--> grammar/statements/contract.g4
 *         |
 *         +--> refinement.g4
 *         |       |
 *         |       +--> property
 *         |
 *         +--> requires.g4
 *                 |
 *                 +--> requirements
 *                 +--> resources
 *
 * ============================================================================
 * IMPORTANT NON-DUPLICATION RULE
 * ============================================================================
 *
 * This file deliberately does NOT import the production parser composition
 * root:
 *
 *     ZamaniParser
 *
 * Doing so would turn this validation grammar into a second complete-program
 * parser and would blur the ownership boundary between:
 *
 *     production parsing
 *
 * and:
 *
 *     isolated validation.
 *
 * The validation orchestrator imports only the specialized validation
 * grammars.
 *
 * ============================================================================
 * GRAMMAR
 * ============================================================================
 */

parser grammar Validation;

options {
    tokenVocab = ZamaniLexer;
}

import
    AssertionsValidation,
    Assumptions,
    ContractValidation,
    Ensures,
    EvidenceValidation,
    GuaranteesValidation,
    Invariants,
    Postconditions,
    PreconditionsValidation,
    ProofsValidation,
    PropertiesValidation,
    RefinementValidation,
    RequiresValidation
    ;


/*
 * ============================================================================
 * PUBLIC VALIDATION ENTRY POINT
 * ============================================================================
 *
 * Exactly one non-empty validation case must be consumed, followed by EOF.
 *
 * This prevents an empty input from being reported as a successful universal
 * validation result even though some specialized validation grammars support
 * empty sequences for their own purposes.
 */
validationUnit
    : validationCase EOF
    ;


/*
 * ============================================================================
 * VALIDATION CASE
 * ============================================================================
 *
 * Every alternative delegates to an existing validation-facing rule or to the
 * canonical property subject where the existing proof/refinement facades
 * intentionally expose only a complete-input unit.
 *
 * No source-language syntax is reproduced here.
 */
validationCase
    : assertionsValidationItem
    | assumptionsValidationItem
    | contractValidationItem
    | ensuresValidationItem
    | evidenceValidationSubject
    | guaranteesValidationItem
    | invariantValidationItem
    | postconditionValidationItem
    | preconditionsValidationItem
    | propertyValidationItem
    | requiresValidationItem
    | resourceRequiresValidationItem
    | propertyStatement
    ;
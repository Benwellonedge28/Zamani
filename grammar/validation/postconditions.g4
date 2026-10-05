/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/validation/postconditions.g4
 *
 * GRAMMAR
 * -------
 * Postconditions
 *
 * STATUS
 * ------
 * Production validation/conformance component
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file provides the isolated parser-level validation entry point for
 * Zamani postconditions.
 *
 * IMPORTANT:
 *
 * This file is a VALIDATION FACADE.
 *
 * It is NOT a second source-language grammar owner.
 *
 * The canonical source-level postcondition construct is:
 *
 *     ensures(condition);
 *
 * and its syntax is owned exclusively by:
 *
 *     grammar/statements/contract.g4
 *         -> ContractStatements
 *         -> ensuresStatement
 *
 * This file exists so validation/conformance tooling can validate a complete
 * postcondition independently while consuming exactly the same grammar rule
 * used by the production parser.
 *
 * ============================================================================
 * ARCHITECTURAL PRINCIPLE
 * ============================================================================
 *
 * There is one source-language owner for each construct.
 *
 * Therefore:
 *
 *     grammar/statements/contract.g4
 *                 |
 *                 v
 *          ensuresStatement
 *                 |
 *        +--------+--------+
 *        |                 |
 *        v                 v
 * production parser   validation/postconditions.g4
 *
 * Both paths consume the same canonical syntax.
 *
 * This file MUST NOT create:
 *
 *     postconditionStatement
 *
 * with an independently defined:
 *
 *     POSTCONDITION
 *     LPAREN
 *     expression
 *     RPAREN
 *
 * sequence.
 *
 * The lexer already contains a POSTCONDITION token for broader language
 * compatibility/specification purposes, but the presence of a lexer token
 * does NOT by itself make a source construct part of the canonical parser.
 *
 * Consequently this file deliberately does not consume POSTCONDITION.
 *
 * The canonical source spelling remains:
 *
 *     ensures(...)
 *
 * If a future language revision formally promotes `postcondition(...)` to
 * canonical source syntax, that change must be made first in the canonical
 * contract grammar and specification, then consumed here through delegation.
 * This file must never establish that syntax independently.
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
 *     ANTLR4-compatible parser grammar
 *
 * Rust integration:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust Edition 2021
 *     Safe Rust only
 *
 * This grammar contains:
 *
 *     - no embedded Rust;
 *     - no semantic predicates;
 *     - no semantic actions;
 *     - no unsafe code;
 *     - no filesystem access;
 *     - no network access;
 *     - no process execution;
 *     - no hardware discovery;
 *     - no runtime execution;
 *     - no target selection;
 *     - no backend selection;
 *     - no resource allocation;
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
 * Provide a complete-input validation boundary for the semantic postcondition
 * represented by the canonical `ensuresStatement`.
 *
 * OWNS
 * ----
 *
 *     postconditionValidationUnit
 *     postconditionValidationItem
 *
 * These rules belong exclusively to this validation facade.
 *
 * DOES NOT OWN
 * -------------
 *
 *     ensuresStatement
 *     contractStatement
 *     contractCondition
 *     expression
 *     statementTerminator
 *
 * Those remain owned by their canonical grammar subsystems.
 *
 * This file also does not own:
 *
 *     lexer tokens
 *     keywords
 *     punctuation
 *     identifiers
 *     names
 *     types
 *     effects
 *     capabilities
 *     resources
 *     policies
 *     provenance
 *     AST structures
 *     semantic analysis
 *     theorem proving
 *     runtime verification
 *     quantum semantics
 *     HDL semantics
 *     hardware realization
 *     distributed placement
 *     target selection
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
 *         ensuresStatement
 *
 *     grammar/lexer/keywords.g4
 *         indirectly through ZamaniLexer
 *
 *     grammar/expressions/expressions.g4
 *         indirectly through ContractStatements
 *
 *     grammar/core/punctuation.g4
 *         indirectly through ContractStatements
 *
 * EXPORTS
 * -------
 *
 *     postconditionValidationUnit
 *     postconditionValidationItem
 *
 * CONSUMED_BY
 * ----------
 *
 *     validation/conformance tooling
 *     isolated postcondition parser tests
 *     grammar regression tests
 *     diagnostics tests
 *     source-span tests
 *     validation orchestration
 *
 * AST_OWNER
 * ---------
 *
 *     repository frontend/domain-neutral AST subsystem
 *
 * This file creates parser contexts only.
 *
 * SEMANTIC_OWNER
 * --------------
 *
 *     contract semantic validation
 *     postcondition semantic analysis
 *     verification infrastructure where applicable
 *
 * TYPE_OWNER
 * ----------
 *
 *     canonical Zamani type/semantic subsystem
 *
 * EFFECT_OWNER
 * ------------
 *
 *     canonical Zamani effect subsystem
 *
 * CAPABILITY_OWNER
 * ----------------
 *
 *     canonical capability/resource subsystem
 *
 * RESOURCE_OWNER
 * --------------
 *
 *     canonical resource analysis subsystem
 *
 * POLICY_OWNER
 * ------------
 *
 *     canonical policy subsystem
 *
 * PROVENANCE_OWNER
 * ----------------
 *
 *     canonical provenance subsystem
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
 *     contract/postcondition tests
 *
 * SPEC_OWNER
 * ----------
 *
 *     grammar/specification/contracts.md
 *     grammar/spec/contracts.md
 *     grammar/DESIGN.md
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The canonical lexer remains:
 *
 *     ZamaniLexer
 *
 * This file defines no lexer rules.
 *
 * In particular, this file does NOT define or consume a second lexical
 * spelling for postconditions.
 *
 * The canonical source construct consumed here is:
 *
 *     ENSURES
 *
 * The repository already contains:
 *
 *     POSTCONDITION
 *
 * as a lexer token.
 *
 * That token must not be interpreted here as proof that:
 *
 *     postcondition(...)
 *
 * is canonical source syntax.
 *
 * Lexer availability and parser acceptance are intentionally separate.
 *
 * This distinction prevents unused/reserved vocabulary from silently creating
 * competing grammar authorities.
 *
 * ============================================================================
 * PARSER CONTRACT
 * ============================================================================
 *
 * The canonical parser dependency is:
 *
 *     ContractStatements
 *          |
 *          +--> ensuresStatement
 *                  |
 *                  +--> contractCondition
 *                          |
 *                          +--> expression
 *
 * This validation grammar delegates to that exact rule.
 *
 * Therefore:
 *
 *     production parsing
 *         and
 *     isolated postcondition validation
 *
 * cannot silently diverge in their accepted condition syntax.
 *
 * ============================================================================
 * PUBLIC VALIDATION RULES
 * ============================================================================
 *
 * postconditionValidationUnit
 *
 *     Complete-input validation entry point.
 *
 *     It requires exactly one canonical ensures statement followed by EOF.
 *
 * postconditionValidationItem
 *
 *     Validation wrapper around the canonical ensuresStatement rule.
 *
 * The names are deliberately validation-specific.
 *
 * They must not be confused with:
 *
 *     ensuresStatement
 *
 * which remains the canonical source-language rule.
 *
 * ============================================================================
 * PRIVATE RULES
 * ============================================================================
 *
 * None.
 *
 * This is deliberate.
 *
 * This file must not duplicate:
 *
 *     contractCondition
 *     expression
 *     statementTerminator
 *     identifiers
 *     names
 *     punctuation
 *
 * because doing so would create another grammar authority.
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
 * Examples of structurally valid forms therefore include forms accepted by
 * the canonical expression grammar, such as:
 *
 *     ensures(result >= 0);
 *     ensures(output.is_valid());
 *     ensures(capability("quantum.measurement"));
 *     ensures(memory_used <= allowed_memory);
 *     ensures(measurement.is_valid());
 *     ensures(model.is_consistent());
 *
 * The exact expression language is NOT duplicated here.
 *
 * ============================================================================
 * POSTCONDITION SEMANTIC CONTRACT
 * ============================================================================
 *
 * A postcondition states a condition associated with successful completion of
 * the relevant semantic operation, declaration, function, computation,
 * transformation, or other enclosing context.
 *
 * This grammar establishes only structural validity.
 *
 * It does NOT determine:
 *
 *     - when the condition is evaluated;
 *     - whether it is statically proven;
 *     - whether it becomes a runtime check;
 *     - whether it is discharged by a proof engine;
 *     - whether it is checked during simulation;
 *     - whether it is checked during compilation;
 *     - whether it is checked during deployment;
 *     - whether it is inherited;
 *     - whether it is refined;
 *     - whether it is weakened or strengthened;
 *     - whether it applies to a particular backend;
 *     - whether it is physically realizable.
 *
 * These are semantic/verification responsibilities.
 *
 * ============================================================================
 * CONTRACT MODEL
 * ============================================================================
 *
 * The postcondition belongs to Zamani's common contract family:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * `ensures` is the canonical source representation of a postcondition.
 *
 * A downstream semantic contract model may represent the construct as:
 *
 *     ContractObligation
 *     {
 *         kind
 *         condition
 *         scope
 *         enclosing_context
 *         source_span
 *         assumptions
 *         requirements
 *         guarantees
 *         evidence
 *         provenance
 *     }
 *
 * The exact Rust structures belong to the frontend/semantic subsystem.
 *
 * This grammar must never introduce a second Rust AST merely because it is a
 * validation facade.
 *
 * ============================================================================
 * FUNCTION CONTRACT INTEGRATION
 * ============================================================================
 *
 * Function-attached contracts remain owned by:
 *
 *     grammar/functions/contracts.g4
 *
 * For example, where the canonical function grammar permits:
 *
 *     fn compute(x) contract {
 *         requires(x >= 0);
 *         ensures(result >= 0);
 *     }
 *
 * the `ensures` member is semantically a postcondition.
 *
 * This file does NOT import FunctionContracts and does NOT attempt to parse
 * an entire function declaration.
 *
 * The validation architecture is:
 *
 *     function grammar
 *          |
 *          v
 *     function contract
 *          |
 *          v
 *     canonical contract model
 *
 * while this file provides:
 *
 *     isolated postcondition validation
 *
 * for conformance and validation tooling.
 *
 * ============================================================================
 * STANDALONE CONTRACT INTEGRATION
 * ============================================================================
 *
 * Standalone contract statements remain owned by:
 *
 *     grammar/statements/contract.g4
 *
 * This file consumes:
 *
 *     ensuresStatement
 *
 * from that grammar.
 *
 * It therefore cannot drift into a different condition grammar.
 *
 * ============================================================================
 * DISTRIBUTED CONTRACT INTEGRATION
 * ============================================================================
 *
 * Distributed contracts may express postconditions concerning:
 *
 *     communication
 *     consistency
 *     availability
 *     replication
 *     placement
 *     topology
 *     fault state
 *     recovery
 *     completion
 *
 * Their declaration structure remains owned by:
 *
 *     grammar/distributed/contracts.g4
 *
 * This file does not duplicate distributed contract syntax.
 *
 * The semantic postcondition model can nevertheless be shared downstream.
 *
 * ============================================================================
 * EXPRESSION CONTRACT
 * ============================================================================
 *
 * The condition is an ordinary Zamani expression through:
 *
 *     ContractStatements
 *         ->
 *     contractCondition
 *         ->
 *     expression
 *
 * Therefore postconditions may semantically refer to:
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
 *     model state
 *     provenance
 *     policies
 *     capabilities
 *     resources
 *     uncertainty
 *
 * This file defines none of those expression forms.
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * This grammar introduces no special postcondition type.
 *
 * It does not create:
 *
 *     PostconditionType
 *     EnsuresType
 *     QuantumPostconditionType
 *     HardwarePostconditionType
 *     AIPostconditionType
 *
 * The condition is typed by the canonical semantic/type system.
 *
 * Semantic analysis determines whether the expression is a valid predicate
 * for its enclosing contract context.
 *
 * This permits one postcondition architecture across:
 *
 *     classical computation
 *     quantum computation
 *     hybrid computation
 *     HDL/hardware intent
 *     AI/ML computation
 *     data processing
 *     distributed computation
 *     networking
 *     accelerator computation
 *     future computational domains
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Parsing a postcondition does not evaluate it.
 *
 * Therefore this grammar itself has no runtime effect.
 *
 * If the condition references an effectful operation, effect checking remains
 * downstream.
 *
 * Possible semantic effects include existing repository effects such as:
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
 * This file neither defines nor evaluates those effects.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * A postcondition may semantically depend on capabilities.
 *
 * Example:
 *
 *     ensures(capability("quantum.measurement"));
 *
 * Another example:
 *
 *     ensures(capability("tensor.compute"));
 *
 * The grammar merely accepts the expression.
 *
 * Capability resolution occurs downstream.
 *
 * Therefore:
 *
 *     source
 *       |
 *       v
 *     parser
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
 * No physical device is selected by this file.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * A postcondition may describe resource-related outcomes.
 *
 * Examples:
 *
 *     ensures(memory_used <= allowed_memory);
 *     ensures(qubits_used <= required_qubits);
 *     ensures(result_size <= available_capacity);
 *
 * These are ordinary semantic expressions.
 *
 * This grammar establishes NO universal physical limits.
 *
 * In particular, it contains no limits for:
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
 * The language therefore has no artificial finite resource ceiling.
 *
 * Actual feasibility is determined by:
 *
 *     semantic analysis
 *     resource analysis
 *     capability negotiation
 *     compilation
 *     scheduling
 *     runtime
 *     target resources
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Policies may affect whether a postcondition is:
 *
 *     permitted
 *     enforceable
 *     verifiable
 *     deployable
 *     runtime-checkable
 *     admissible
 *     compatible with an execution mode
 *
 * Policy interpretation is not performed by this grammar.
 *
 * Policy ownership remains in the repository policy/security subsystems.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * The frontend and semantic pipeline must preserve enough provenance to
 * identify:
 *
 *     source file
 *     source span
 *     module
 *     enclosing declaration
 *     enclosing execution scope
 *     contract kind
 *     condition
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
 * This grammar does not store or mutate provenance.
 *
 * ============================================================================
 * EVIDENCE / VERIFICATION CONTRACT
 * ============================================================================
 *
 * A postcondition can become a verification obligation.
 *
 * Verification may be performed by:
 *
 *     static analysis
 *     symbolic reasoning
 *     theorem proving
 *     model checking
 *     testing
 *     simulation
 *     runtime checking
 *     domain-specific verification
 *
 * None of these mechanisms belongs in this grammar.
 *
 * A parser-accepted postcondition is NOT automatically:
 *
 *     proven
 *     true
 *     executable
 *     guaranteed
 *     realizable
 *
 * Structural acceptance and semantic verification are separate stages.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * A quantum computation may have a postcondition such as:
 *
 *     ensures(measurement.is_valid());
 *
 * or:
 *
 *     ensures(capability("quantum.measurement"));
 *
 * This grammar remains completely independent of quantum realization.
 *
 * It does not:
 *
 *     - enumerate quantum gates;
 *     - enumerate physical qubits;
 *     - encode coupling maps;
 *     - encode calibration;
 *     - select a QPU;
 *     - perform decomposition;
 *     - perform routing;
 *     - perform scheduling;
 *     - perform QEC;
 *     - create another quantum IR.
 *
 * When the enclosing semantic operation is quantum, the downstream boundary
 * remains:
 *
 *     domain-neutral AST
 *          |
 *          v
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
 * The postcondition grammar does not participate in those target-specific
 * transformations.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Hardware and HDL postconditions remain semantic expressions.
 *
 * Examples:
 *
 *     ensures(output.is_valid());
 *     ensures(signal.is_defined(clock));
 *     ensures(hardware_state.is_consistent());
 *
 * This file does not encode:
 *
 *     signal width limits
 *     register width limits
 *     memory capacity
 *     FPGA dimensions
 *     ASIC resource counts
 *     fixed clock counts
 *     physical addresses
 *     vendor-specific topology
 *     device identifiers
 *
 * Hardware feasibility remains downstream.
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Classical computation uses the same postcondition mechanism.
 *
 * Examples:
 *
 *     ensures(result >= 0);
 *     ensures(result == expected);
 *     ensures(state.is_consistent());
 *
 * There is no separate classical postcondition grammar.
 *
 * ============================================================================
 * HYBRID INTEGRATION
 * ============================================================================
 *
 * Hybrid computation uses the same construct.
 *
 * A postcondition may describe a result produced through:
 *
 *     classical computation
 *     quantum computation
 *     accelerator computation
 *     measurement
 *     AI/ML computation
 *     distributed execution
 *
 * The condition remains one ordinary Zamani expression.
 *
 * ============================================================================
 * AI / REASONING / LEARNING INTEGRATION
 * ============================================================================
 *
 * Postconditions may semantically describe:
 *
 *     reasoning results
 *     learned model state
 *     adaptation outcomes
 *     evidence
 *     confidence
 *     decisions
 *     provenance
 *     uncertainty
 *
 * Examples:
 *
 *     ensures(model.is_consistent());
 *     ensures(decision.has_evidence());
 *     ensures(confidence >= required_confidence);
 *
 * This file does not introduce AI-specific postcondition syntax.
 *
 * The same contract architecture remains usable across the language.
 *
 * ============================================================================
 * PATTERN / MATCH / GUARD INTEGRATION
 * ============================================================================
 *
 * If the canonical expression system permits patterns, matching, guards, or
 * conditional expressions inside contract conditions, those constructs remain
 * owned by:
 *
 *     grammar/expressions/*
 *
 * This file does not reproduce them.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing a postcondition must depend only on:
 *
 *     source token sequence
 *     canonical lexer vocabulary
 *     canonical parser grammar
 *     explicit grammar/language configuration
 *
 * Parsing MUST NOT inspect:
 *
 *     hardware
 *     CPU count
 *     GPU count
 *     QPU availability
 *     memory availability
 *     filesystem state
 *     network state
 *     wall-clock time
 *     randomness
 *     runtime state
 *     scheduler state
 *     target topology
 *
 * Therefore the validation parse is deterministic for identical source and
 * grammar configuration.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * This grammar provides no execution mechanism.
 *
 * It contains:
 *
 *     no embedded Rust;
 *     no semantic actions;
 *     no filesystem access;
 *     no network access;
 *     no process execution;
 *     no hardware discovery;
 *     no plugin execution;
 *     no unsafe Rust.
 *
 * A syntactically valid postcondition must still pass all downstream:
 *
 *     type
 *     effect
 *     capability
 *     resource
 *     policy
 *     security
 *     provenance
 *     semantic
 *
 * checks before any implementation-dependent execution or verification.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * This grammar must remain independent of physical machine scale.
 *
 * It establishes no artificial finite maximum for:
 *
 *     number of postconditions
 *     expression size
 *     expression nesting
 *     program size
 *     number of declarations
 *     number of functions
 *     number of modules
 *     number of quantum operations
 *     number of qubits
 *     number of processors
 *     number of GPUs
 *     number of FPGAs
 *     number of accelerators
 *     number of nodes
 *     number of devices
 *     memory capacity
 *     tensor rank
 *     register width
 *     network size
 *
 * There are intentionally no constants such as:
 *
 *     MAX_CONTRACTS
 *     MAX_POSTCONDITIONS
 *     MAX_PRECONDITIONS
 *     MAX_INVARIANTS
 *     MAX_EXPRESSION_DEPTH
 *     MAX_QUANTUM_OPERATIONS
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
 * "Unbounded" at the language level means that this grammar does not impose
 * an artificial finite capacity ceiling.
 *
 * It does not claim physically infinite hardware or compiler resources.
 *
 * Actual limits must be reported as implementation, resource, target, or
 * runtime constraints rather than silently becoming language semantics.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * A postcondition remains structurally identical whether the enclosing
 * computation targets:
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
 *     heterogeneous systems
 *     future computational substrates
 *
 * Scaling is therefore handled downstream through:
 *
 *     semantic specialization
 *     capability negotiation
 *     resource negotiation
 *     optimization
 *     lowering
 *     placement
 *     routing
 *     scheduling
 *     resilience
 *     deployment
 *
 * The postcondition grammar does not change as the available machine changes.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Canonical source syntax remains:
 *
 *     ensures(condition);
 *
 * This file does not reinterpret:
 *
 *     postcondition(condition);
 *
 * as canonical syntax.
 *
 * The existing POSTCONDITION lexer token is therefore not consumed here.
 *
 * This is intentional and prevents an unused/reserved lexical spelling from
 * silently becoming a second language construct.
 *
 * If a future language version adopts:
 *
 *     postcondition(condition);
 *
 * as canonical syntax, the required migration order is:
 *
 *     specification
 *         ->
 *     canonical contract grammar
 *         ->
 *     AST mapping
 *         ->
 *     semantic model
 *         ->
 *     validation facade
 *         ->
 *     tests
 *         ->
 *     compatibility policy
 *
 * This file must then delegate to that canonical rule rather than defining its
 * own independent production.
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Structural diagnostics are inherited from:
 *
 *     ContractStatements
 *     contractCondition
 *     expression
 *     statementTerminator
 *
 * Examples of malformed source include:
 *
 *     ensures;
 *     ensures();
 *     ensures(;
 *     ensures(, condition);
 *     ensures(condition;
 *     ensures(condition,);
 *     ensures(condition) trailing
 *
 * These must be handled according to the canonical parser's diagnostics.
 *
 * Semantic diagnostics are downstream and may include:
 *
 *     unknown name
 *     invalid scope
 *     invalid result reference
 *     invalid condition type
 *     impossible postcondition
 *     unsupported verification obligation
 *     unavailable capability
 *     unavailable resource
 *     policy violation
 *     invalid provenance
 *     unsupported target realization
 *
 * A semantic failure must not be disguised as a parser failure.
 *
 * ============================================================================
 * SOURCE SPAN / AST CONTRACT
 * ============================================================================
 *
 * Validation must preserve the same source structure available through the
 * canonical ensuresStatement rule.
 *
 * The consuming AST pipeline must be able to preserve at least:
 *
 *     contract kind = ensures
 *     condition
 *     source span
 *     source order
 *     enclosing scope
 *     enclosing declaration
 *
 * The validation facade must not require a second AST representation.
 *
 * Therefore:
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
 *     postconditionValidationUnit
 *          |
 *          v
 *     ensuresStatement
 *          |
 *          v
 *     same canonical AST model
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar produces NO IR.
 *
 * It must never bypass the canonical semantic architecture.
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
 *       +--> future domain representation
 *       |
 *       v
 *     target-independent optimization
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
 * BACKEND CONTRACT
 * ============================================================================
 *
 * A backend may use a semantically validated postcondition to:
 *
 *     - discharge a proof obligation;
 *     - preserve a guarantee;
 *     - generate a runtime check;
 *     - generate verification metadata;
 *     - reject an unsupported semantic requirement;
 *     - propagate a property through lowering;
 *     - attach validation metadata to generated artifacts.
 *
 * Those actions belong to backend/semantic infrastructure.
 *
 * This grammar only establishes structural source conformance.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * UPSTREAM
 * --------
 *
 *     grammar/lexer/keywords.g4
 *         canonical lexical vocabulary
 *
 *     grammar/antlr/ZamaniLexer.g4
 *         public lexer boundary
 *
 *     grammar/statements/contract.g4
 *         canonical contract source syntax
 *
 *     grammar/expressions/expressions.g4
 *         indirect expression dependency
 *
 *     grammar/core/punctuation.g4
 *         indirect statement termination dependency
 *
 * DOWNSTREAM
 * ----------
 *
 *     validation/conformance tooling
 *     validation orchestration
 *     frontend AST construction
 *     semantic contract validation
 *     type analysis
 *     effect analysis
 *     capability analysis
 *     resource analysis
 *     policy analysis
 *     provenance
 *     verification
 *     canonical semantic model
 *     applicable domain IR
 *
 * DOMAIN INTEGRATION
 * ------------------
 *
 * Classical:
 *
 *     same canonical contract model
 *
 * Quantum:
 *
 *     semantic model -> quantum::ir
 *
 * HDL/hardware:
 *
 *     semantic hardware representation
 *
 * Distributed:
 *
 *     distributed semantic model
 *
 * AI/data:
 *
 *     shared semantic contract model
 *
 * No domain creates a second postcondition grammar.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive tests MUST include canonical forms accepted by the source grammar.
 *
 * Minimum categories:
 *
 *     simple value postcondition
 *     function-result postcondition
 *     member-access postcondition
 *     capability-related postcondition
 *     resource-related postcondition
 *     classical postcondition
 *     quantum-related postcondition
 *     hybrid postcondition
 *     HDL/hardware-related postcondition
 *     distributed postcondition
 *     model/AI-related postcondition
 *     provenance-related postcondition
 *     nested expression postcondition
 *     large symbolic expression
 *
 * Negative tests MUST include malformed forms.
 *
 * Minimum categories:
 *
 *     missing condition
 *     missing terminator
 *     malformed expression
 *     trailing source after valid statement
 *     invalid punctuation
 *     incomplete expression
 *
 * Semantic-negative tests belong downstream and should cover:
 *
 *     invalid name
 *     invalid scope
 *     invalid condition type
 *     invalid result reference
 *     unavailable capability
 *     insufficient resource
 *     forbidden policy
 *     invalid effect
 *     invalid provenance
 *     unverifiable obligation
 *
 * ============================================================================
 * BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * The validation suite must verify that:
 *
 *     postconditionValidationUnit
 *
 * accepts exactly one complete canonical ensures statement.
 *
 * It must reject:
 *
 *     multiple statements
 *     trailing tokens
 *     unrelated declarations
 *     arbitrary source units
 *
 * unless those forms are explicitly introduced by a future dedicated
 * validation grammar.
 *
 * ============================================================================
 * DETERMINISM TEST CONTRACT
 * ============================================================================
 *
 * Repeated parsing of identical input under identical grammar configuration
 * must produce equivalent parse structure.
 *
 * Validation must not depend on:
 *
 *     machine size
 *     CPU count
 *     GPU count
 *     QPU availability
 *     memory availability
 *     target topology
 *     wall-clock time
 *     runtime state
 *     network state
 *     randomness
 *
 * ============================================================================
 * PORTABILITY TEST CONTRACT
 * ============================================================================
 *
 * The same source:
 *
 *     ensures(condition);
 *
 * must remain grammatically identical when the semantic program is considered
 * for different target classes.
 *
 * The validation facade must not require target-specific grammar variants.
 *
 * ============================================================================
 * NO HARD-CODING CONTRACT
 * ============================================================================
 *
 * This file must remain free of universal machine limits and target-specific
 * assumptions.
 *
 * Forbidden architectural content includes:
 *
 *     fixed qubit limits
 *     fixed processor limits
 *     fixed GPU limits
 *     fixed FPGA limits
 *     fixed node limits
 *     fixed memory limits
 *     fixed thread limits
 *     fixed tensor-rank limits
 *     fixed register widths
 *     fixed network sizes
 *     fixed device counts
 *
 * Postcondition semantics may refer to symbolic quantities such as:
 *
 *     required_memory
 *     available_memory
 *     required_qubits
 *     available_qubits
 *     required_capacity
 *     available_capacity
 *
 * but the grammar must never assign universal numeric ceilings to them.
 *
 * ============================================================================
 * SAFE-RUST CONTRACT
 * ============================================================================
 *
 * This grammar requires no unsafe Rust.
 *
 * Generated parser integration must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *
 * and Rust Edition 2021.
 *
 * This file contains no Rust code and therefore introduces no unsafe operation
 * itself.
 *
 * The downstream implementation must preserve the repository-wide safe-Rust
 * requirement.
 *
 * ============================================================================
 * FILE COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when all of the following are true:
 *
 *     [ ] grammar name is Postconditions;
 *
 *     [ ] token vocabulary is ZamaniLexer;
 *
 *     [ ] canonical ContractStatements grammar is imported;
 *
 *     [ ] canonical ensuresStatement is consumed;
 *
 *     [ ] exactly one public complete-input entry point exists;
 *
 *     [ ] postconditionValidationItem delegates directly to ensuresStatement;
 *
 *     [ ] EOF is mandatory;
 *
 *     [ ] no duplicate ensures syntax exists here;
 *
 *     [ ] no POSTCONDITION token is interpreted as canonical syntax here;
 *
 *     [ ] no expression grammar is duplicated;
 *
 *     [ ] no punctuation grammar is duplicated;
 *
 *     [ ] no AST structures are defined;
 *
 *     [ ] no semantic actions exist;
 *
 *     [ ] no semantic predicates exist;
 *
 *     [ ] no IR is generated;
 *
 *     [ ] no target selection occurs;
 *
 *     [ ] no resource discovery occurs;
 *
 *     [ ] no hardware discovery occurs;
 *
 *     [ ] no quantum-device assumptions exist;
 *
 *     [ ] no fixed machine-capacity constants exist;
 *
 *     [ ] no fixed quantum-capacity constants exist;
 *
 *     [ ] deterministic parsing is preserved;
 *
 *     [ ] source spans remain available through canonical parser contexts;
 *
 *     [ ] canonical AST mapping remains shared with production parsing;
 *
 *     [ ] semantic validation remains downstream;
 *
 *     [ ] type checking remains downstream;
 *
 *     [ ] effect checking remains downstream;
 *
 *     [ ] capability checking remains downstream;
 *
 *     [ ] resource checking remains downstream;
 *
 *     [ ] policy checking remains downstream;
 *
 *     [ ] provenance remains downstream;
 *
 *     [ ] quantum lowering remains downstream through quantum::ir;
 *
 *     [ ] HDL/hardware lowering remains downstream;
 *
 *     [ ] positive tests exist;
 *
 *     [ ] negative tests exist;
 *
 *     [ ] boundary tests exist;
 *
 *     [ ] determinism tests exist;
 *
 *     [ ] portability tests exist;
 *
 *     [ ] scalability tests exist;
 *
 *     [ ] compatibility tests exist;
 *
 *     [ ] Rust 1.97 generation succeeds;
 *
 *     [ ] Rust 1.97.1 generation succeeds;
 *
 *     [ ] generated Rust contains no unsafe requirement;
 *
 *     [ ] validation and production parsing cannot silently diverge.
 *
 * ============================================================================
 * REQUIRED REPOSITORY INVARIANTS
 * ============================================================================
 *
 * The repository must maintain these invariants around this file:
 *
 * 1. `grammar/statements/contract.g4` remains the canonical owner of
 *    `ensuresStatement`.
 *
 * 2. `grammar/validation/ensures.g4` remains the canonical isolated
 *    validation facade for the `ensures` source construct.
 *
 * 3. This file remains a semantic-name facade for postcondition validation
 *    and delegates to the same canonical `ensuresStatement`.
 *
 * 4. No parser may define a second independent postcondition expression
 *    hierarchy.
 *
 * 5. No validation grammar may redefine `expression`.
 *
 * 6. No validation grammar may redefine `contractCondition`.
 *
 * 7. No validation grammar may define lexer tokens.
 *
 * 8. No validation grammar may select hardware.
 *
 * 9. No validation grammar may inspect resources.
 *
 * 10. No validation grammar may execute contracts.
 *
 * 11. No validation grammar may prove contracts.
 *
 * 12. No validation grammar may construct IR.
 *
 * 13. The domain-neutral AST remains the only frontend semantic boundary.
 *
 * 14. Quantum computation continues to use:
 *
 *         semantic model -> quantum::ir
 *
 * 15. Target realization remains downstream of semantic validation.
 *
 * 16. Resource availability remains a property of the compilation/execution
 *     environment, never a grammar constant.
 *
 * 17. Rust integration remains compatible with Rust 1.97 / 1.97.1 and uses
 *     safe Rust only.
 *
 * ============================================================================
 * GRAMMAR
 * ============================================================================
 */

parser grammar Postconditions;

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
 * Public isolated validation entry point.
 *
 * Exactly one canonical postcondition/ensures statement must be consumed,
 * followed by EOF.
 *
 * Therefore:
 *
 *     ensures(condition);
 *
 * is accepted when accepted by ContractStatements, while:
 *
 *     ensures(condition); trailing
 *
 * is rejected as a complete validation unit.
 */
postconditionValidationUnit
    : postconditionValidationItem EOF
    ;


/*
 * ============================================================================
 * VALIDATION ITEM
 * ============================================================================
 *
 * This is intentionally a thin delegation boundary.
 *
 * The canonical source rule owns:
 *
 *     ENSURES
 *     contractCondition
 *     statementTerminator
 *
 * This file owns none of those productions.
 */
postconditionValidationItem
    : ensuresStatement
    ;
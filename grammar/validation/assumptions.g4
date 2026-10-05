/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/validation/assumptions.g4
 *
 * GRAMMAR
 * -------
 * Assumptions
 *
 * STATUS
 * ------
 * Production validation/conformance component
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file provides the isolated validation/conformance parser boundary for
 * canonical Zamani `assume` contract statements.
 *
 * IMPORTANT:
 *
 * This file is a VALIDATION FACADE.
 *
 * It is NOT a second owner of `assume` syntax.
 *
 * The canonical source-language ownership is:
 *
 *     grammar/statements/contract.g4
 *         |
 *         +--> ContractStatements
 *                 |
 *                 +--> assumeStatement
 *
 * This file exposes that already-defined syntax through an isolated parser
 * entry point suitable for:
 *
 *     - grammar conformance;
 *     - parser regression testing;
 *     - structural validation;
 *     - negative testing;
 *     - source-span testing;
 *     - compatibility testing;
 *     - scalability testing;
 *     - cross-domain validation.
 *
 * ============================================================================
 * ARCHITECTURAL PRINCIPLE
 * ============================================================================
 *
 * Zamani separates:
 *
 *     source syntax
 *          |
 *          v
 *     lexer
 *          |
 *          v
 *     parser
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     structural validation
 *          |
 *          +--> types
 *          +--> effects
 *          +--> capabilities
 *          +--> resources
 *          +--> contracts
 *          +--> policies
 *          +--> provenance
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> classical representation
 *          +--> quantum::ir
 *          +--> HDL/hardware representation
 *          +--> distributed representation
 *          +--> accelerator representation
 *          +--> future domain representations
 *          |
 *          v
 *     target-independent optimization
 *          |
 *          v
 *     lowering / realization
 *
 * This file participates ONLY in the parser/validation boundary.
 *
 * It MUST NOT:
 *
 *     - redefine `assume`;
 *     - define another expression grammar;
 *     - define lexer tokens;
 *     - define AST structures;
 *     - evaluate assumptions;
 *     - prove assumptions;
 *     - establish semantic truth;
 *     - resolve identifiers;
 *     - resolve types;
 *     - resolve capabilities;
 *     - resolve resources;
 *     - evaluate policies;
 *     - construct canonical IR;
 *     - construct domain IR;
 *     - select a backend;
 *     - perform quantum routing;
 *     - perform quantum scheduling;
 *     - perform QEC;
 *     - perform HDL synthesis;
 *     - perform hardware placement;
 *     - execute programs;
 *     - inspect target hardware;
 *     - encode machine capacity limits;
 *     - contain embedded Rust actions;
 *     - contain semantic predicates;
 *     - require unsafe Rust.
 *
 * ============================================================================
 * TOOLCHAIN CONTRACT
 * ============================================================================
 *
 * Grammar technology:
 *
 *     ANTLR4 parser grammar
 *
 * Rust implementation baseline:
 *
 *     Rust Edition 2021
 *     Rust 1.97
 *     Rust 1.97.1
 *     safe Rust only
 *
 * This grammar contains no embedded Rust.
 *
 * The generated parser integration MUST therefore remain compatible with the
 * repository's safe-Rust implementation requirements.
 *
 * ============================================================================
 * OWNERSHIP CONTRACT
 * ============================================================================
 *
 * OWNS
 * ----
 *
 *     assumptionsValidationUnit
 *     assumptionsValidationItem
 *     assumptionsValidationStatement
 *
 * These are validation/conformance entry points only.
 *
 * DOES NOT OWN
 * -------------
 *
 *     assumeStatement
 *     contractCondition
 *     expression
 *     statementTerminator
 *     ASSUME token
 *     identifiers
 *     literals
 *     operators
 *     punctuation
 *     AST nodes
 *     semantic contract structures
 *     type rules
 *     effect rules
 *     capability rules
 *     resource rules
 *     policy rules
 *     provenance rules
 *     IR structures
 *
 * Canonical `assume` syntax remains owned by:
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
 *             -> assumeStatement
 *
 * ContractStatements itself owns the downstream dependency on:
 *
 *     grammar/expressions/
 *     grammar/core/punctuation.g4
 *     ZamaniLexer
 *
 * This file deliberately consumes those dependencies indirectly through the
 * canonical contract grammar instead of duplicating them.
 *
 * EXPORTS
 * -------
 *
 *     assumptionsValidationUnit
 *     assumptionsValidationItem
 *     assumptionsValidationStatement
 *
 * CONSUMED_BY
 * -----------
 *
 *     grammar validation tooling
 *     parser conformance tooling
 *     grammar regression tests
 *     validation tests
 *     compatibility tests
 *     diagnostics tests
 *     source-span tests
 *     scalability tests
 *
 * AST_OWNER
 * ---------
 *
 *     repository AST implementation
 *
 * This grammar does not create a validation-specific AST.
 *
 * SEMANTIC_OWNER
 * --------------
 *
 *     semantic contract / assumption analysis
 *
 * TYPE_OWNER
 * ----------
 *
 *     repository type-analysis subsystem
 *
 * EFFECT_OWNER
 * ------------
 *
 *     repository effect-analysis subsystem
 *
 * RESOURCE_OWNER
 * --------------
 *
 *     repository resource-analysis subsystem
 *
 * CAPABILITY_OWNER
 * ----------------
 *
 *     repository capability-analysis subsystem
 *
 * POLICY_OWNER
 * ------------
 *
 *     repository policy subsystem
 *
 * PROVENANCE_OWNER
 * ----------------
 *
 *     repository provenance subsystem
 *
 * IR_OWNER
 * --------
 *
 *     canonical semantic representation and downstream domain IR owners
 *
 * SPEC_OWNER
 * ----------
 *
 *     grammar/specification/
 *     grammar/spec/
 *
 * TEST_OWNER
 * ----------
 *
 *     grammar/tests/
 *     grammar/validation/
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This file defines NO lexer rules.
 *
 * The `ASSUME` token MUST come from the repository's canonical lexical
 * vocabulary.
 *
 * This grammar MUST NOT introduce:
 *
 *     ASSUME_TOKEN
 *     ASSUMPTION
 *     ASSUMPTION_KEYWORD
 *     ValidationAssume
 *
 * or any equivalent duplicate lexical identity.
 *
 * The source spelling and token identity remain the responsibility of the
 * canonical lexer.
 *
 * ============================================================================
 * PARSER CONTRACT
 * ============================================================================
 *
 * The canonical source rule is:
 *
 *     assumeStatement
 *
 * owned by:
 *
 *     ContractStatements
 *
 * This validation grammar delegates directly to that rule.
 *
 * Conceptually:
 *
 *     assumptionsValidationUnit
 *             |
 *             v
 *     assumptionsValidationItem
 *             |
 *             v
 *     assumptionsValidationStatement
 *             |
 *             v
 *     assumeStatement
 *             |
 *             v
 *     contractCondition
 *             |
 *             v
 *     expression
 *
 * No part of this syntax is copied into this file.
 *
 * ============================================================================
 * ASSUMPTION SEMANTIC BOUNDARY
 * ============================================================================
 *
 * The source construct:
 *
 *     assume(condition);
 *
 * represents an assumption supplied to the semantic contract/verification
 * model.
 *
 * An assumption is NOT automatically:
 *
 *     - a proof;
 *     - a guarantee;
 *     - an invariant;
 *     - a runtime assertion;
 *     - a hardware capability;
 *     - a resource allocation;
 *     - a security authorization;
 *     - a target-feasibility result.
 *
 * Downstream semantic analysis determines the meaning of the assumption in
 * its enclosing scope and execution model.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar produces parser contexts only.
 *
 * It MUST NOT introduce a node such as:
 *
 *     AssumptionValidationNode
 *
 * merely because this validation facade exists.
 *
 * The canonical AST representation of `assume` remains owned by the existing
 * frontend AST architecture.
 *
 * The semantic representation must preserve, where applicable:
 *
 *     contract kind
 *     condition
 *     source span
 *     enclosing scope
 *     source ordering
 *     semantic metadata
 *     provenance
 *
 * The validation wrapper is not a new language construct.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing an assumption establishes only structural conformance.
 *
 * Semantic analysis is responsible for determining:
 *
 *     - whether the condition is well formed semantically;
 *     - whether names resolve;
 *     - whether values are available;
 *     - whether the condition is legal in its scope;
 *     - whether its types are valid;
 *     - whether its effects are permitted;
 *     - whether required capabilities exist;
 *     - whether referenced resources are meaningful;
 *     - whether applicable policies permit the assumption;
 *     - whether the assumption is compatible with the enclosing contract;
 *     - whether verification or runtime evaluation is required.
 *
 * This grammar MUST NOT perform any of those operations.
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * `assume` consumes the ordinary Zamani contract condition.
 *
 * The canonical source rule is:
 *
 *     assumeStatement
 *         : ASSUME contractCondition statementTerminator
 *         ;
 *
 * and:
 *
 *     contractCondition
 *         : LPAREN expression RPAREN
 *         ;
 *
 * Therefore this file MUST NOT create a special type such as:
 *
 *     AssumptionBoolean
 *     QuantumAssumptionBoolean
 *     HardwareAssumptionBoolean
 *     AIAssumptionBoolean
 *
 * The ordinary Zamani type system determines whether the condition is a valid
 * predicate according to language semantics.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Parsing an assumption has no runtime effect.
 *
 * The condition itself may reference expressions whose semantic evaluation
 * carries effects. Those effects are analyzed downstream.
 *
 * Possible effect categories include, where already defined by the language:
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
 * This file MUST NOT define a second effect taxonomy.
 *
 * An assumption MUST NOT silently grant permission to perform an effect.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * An assumption condition may semantically refer to capability-dependent
 * values or predicates.
 *
 * Examples include:
 *
 *     assume(capability("quantum.measurement"));
 *     assume(capability("tensor.compute"));
 *     assume(capability("hdl.synthesis"));
 *
 * Whether such expressions are legal and whether their capabilities are
 * available is determined downstream.
 *
 * This grammar does NOT resolve capabilities.
 *
 * The architecture remains:
 *
 *     source
 *       |
 *       v
 *     assumeStatement
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
 * Capability availability MUST never be hard-coded here.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Assumptions may semantically refer to resource state or resource-derived
 * conditions.
 *
 * Examples include:
 *
 *     assume(memory >= required_memory);
 *     assume(qubits >= required_qubits);
 *     assume(topology.supports(required_topology));
 *
 * These expressions describe program-level semantic conditions.
 *
 * They do NOT establish universal implementation limits.
 *
 * This grammar MUST NOT encode finite ceilings for:
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
 * A numeric value inside an assumption remains program semantics.
 *
 * ============================================================================
 * CONTRACT INTEGRATION
 * ============================================================================
 *
 * `assume` is one member of the universal contract family:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * Canonical ownership remains:
 *
 *     grammar/statements/contract.g4
 *
 * This file MUST NOT redefine any member of that family.
 *
 * In particular, this file MUST NOT import or reproduce:
 *
 *     requiresStatement
 *     ensuresStatement
 *     invariantStatement
 *     guaranteeStatement
 *     propertyStatement
 *
 * It only exposes:
 *
 *     assumeStatement
 *
 * through the validation facade.
 *
 * ============================================================================
 * ASSUMPTION VS GUARANTEE
 * ============================================================================
 *
 * The semantic model MUST preserve the distinction:
 *
 *     assume(condition)
 *
 * means the condition is supplied as an assumption to the relevant semantic
 * model.
 *
 *     guarantee(condition)
 *
 * means the program/system makes a semantic promise that must be validated.
 *
 * These constructs MUST NOT be normalized into one undifferentiated boolean
 * assertion during parsing.
 *
 * ============================================================================
 * ASSUMPTION VS REQUIREMENT
 * ============================================================================
 *
 * The semantic model MUST also preserve:
 *
 *     assume(condition)
 *
 * versus:
 *
 *     requires(condition)
 *
 * A requirement expresses program/environment intent.
 *
 * An assumption supplies a premise to the relevant contract or verification
 * context.
 *
 * They may interact downstream, but they are not interchangeable parser
 * constructs.
 *
 * This grammar therefore delegates to the distinct canonical source rules
 * rather than attempting to merge them.
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Assumptions may be subject to policies.
 *
 * Policy analysis may determine:
 *
 *     - whether an assumption is permitted;
 *     - whether an assumption must be declared;
 *     - whether an assumption requires evidence;
 *     - whether an assumption may affect execution;
 *     - whether an assumption is trusted;
 *     - whether an assumption must be checked dynamically;
 *     - whether an assumption is valid only in a specified scope.
 *
 * Policy evaluation belongs downstream.
 *
 * This file MUST NOT contain policy decisions.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Validation and semantic infrastructure must be able to associate an
 * assumption with its source.
 *
 * At minimum, provenance should be capable of retaining:
 *
 *     source file
 *     source span
 *     contract kind
 *     condition
 *     enclosing scope
 *     language/grammar version
 *
 * Downstream provenance may additionally retain:
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
 * This grammar does not store provenance.
 *
 * It merely preserves the canonical parser boundary required for downstream
 * infrastructure to recover it.
 *
 * ============================================================================
 * VERIFICATION CONTRACT
 * ============================================================================
 *
 * An assumption is not automatically verified merely because it parses.
 *
 * The compiler/runtime/verification layer may distinguish:
 *
 *     syntactically valid
 *     semantically valid
 *     statically verified
 *     dynamically checked
 *     externally evidenced
 *     unresolved
 *     violated
 *
 * Those states MUST NOT be encoded as grammar alternatives.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates NO IR.
 *
 * The intended pipeline is:
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
 *       +--> type analysis
 *       +--> effect analysis
 *       +--> capability analysis
 *       +--> resource analysis
 *       +--> contract analysis
 *       +--> policy analysis
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
 * This validation facade MUST NOT bypass the semantic model to reach an IR.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * The same `assume` syntax is valid for quantum-related semantic conditions
 * when those conditions are supported by the ordinary expression system.
 *
 * Examples of possible semantic conditions include:
 *
 *     assume(capability("quantum.measurement"));
 *     assume(quantum_state.is_valid());
 *     assume(measurement.is_valid());
 *     assume(required_qubits <= available_qubits);
 *     assume(resilience_state.is_acceptable());
 *
 * This grammar does NOT know whether a quantum processor exists or how it is
 * implemented.
 *
 * It MUST NOT:
 *
 *     - enumerate quantum gates;
 *     - enumerate qubits;
 *     - define physical qubit identifiers;
 *     - define coupling maps;
 *     - define calibration;
 *     - define physical topology;
 *     - perform routing;
 *     - perform scheduling;
 *     - perform QEC;
 *     - select a QPU;
 *     - create a quantum IR.
 *
 * Quantum semantics continue toward:
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
 * HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * The same assumption construct may semantically constrain:
 *
 *     signal state
 *     timing
 *     protocol state
 *     hardware validity
 *     interface relationships
 *     execution conditions
 *
 * Examples:
 *
 *     assume(signal.is_defined(clock));
 *     assume(output.is_valid());
 *     assume(hardware_state.is_consistent());
 *
 * No fixed hardware dimension belongs in this grammar.
 *
 * This grammar MUST NOT encode:
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
 * Classical programs use exactly the same assumption syntax.
 *
 * No separate classical assumption grammar is required.
 *
 * Conditions may refer to:
 *
 *     values
 *     state
 *     types
 *     data
 *     resources
 *     capabilities
 *     execution state
 *
 * through the common expression and semantic systems.
 *
 * ============================================================================
 * HYBRID BOUNDARY
 * ============================================================================
 *
 * Hybrid classical/quantum programs use the same assumption construct.
 *
 * An assumption may semantically relate:
 *
 *     classical state
 *     quantum state
 *     measurement results
 *     control state
 *     resources
 *     capabilities
 *     execution state
 *
 * No hybrid-specific assumption syntax is introduced.
 *
 * ============================================================================
 * AI / REASONING BOUNDARY
 * ============================================================================
 *
 * Assumptions may participate in reasoning, learning, uncertainty, causality,
 * model validation, evidence and decision semantics.
 *
 * Examples include:
 *
 *     assume(model.is_valid());
 *     assume(confidence >= required_confidence);
 *     assume(decision.has_evidence());
 *     assume(data.is_consistent());
 *
 * The grammar remains domain-neutral.
 *
 * It MUST NOT enumerate model families, algorithms, reasoning systems,
 * application categories, or hardware implementations.
 *
 * ============================================================================
 * DISTRIBUTED BOUNDARY
 * ============================================================================
 *
 * Assumptions may semantically describe distributed conditions such as:
 *
 *     assume(service.is_available());
 *     assume(consistency.is_satisfied());
 *     assume(topology.supports(required_topology));
 *
 * Distributed feasibility and runtime state remain downstream concerns.
 *
 * This grammar MUST NOT impose a fixed number of nodes, processes, devices,
 * peers, channels, or network endpoints.
 *
 * ============================================================================
 * SECURITY BOUNDARY
 * ============================================================================
 *
 * An assumption may appear in security-related semantic contexts where the
 * language specification permits it.
 *
 * Examples include:
 *
 *     assume(identity.is_authenticated());
 *     assume(policy.is_satisfied());
 *     assume(channel.is_trusted());
 *
 * Security verification and authorization remain owned by the security and
 * policy subsystems.
 *
 * Parsing an assumption MUST NOT grant authorization.
 *
 * ============================================================================
 * SIMULATION BOUNDARY
 * ============================================================================
 *
 * An assumption may be consumed by simulation or verification infrastructure.
 *
 * Simulation may use assumptions as premises for:
 *
 *     classical simulation
 *     quantum simulation
 *     hardware simulation
 *     distributed simulation
 *     fault simulation
 *     performance analysis
 *
 * Simulation semantics remain downstream.
 *
 * This grammar MUST NOT turn simulation into a separate source language.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing the same source with the same grammar and lexical configuration must
 * produce structurally equivalent parser results.
 *
 * This file MUST NOT depend on:
 *
 *     wall-clock time
 *     random state
 *     target hardware
 *     available devices
 *     resource discovery
 *     network state
 *
 * Determinism of semantic evaluation is a downstream concern.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar imposes no semantic finite limit on:
 *
 *     number of assumptions
 *     number of contract statements
 *     expression size
 *     identifier size
 *     numeric value range
 *     resource quantity
 *     quantum resource quantity
 *     hardware resource quantity
 *     distributed topology size
 *     tensor dimensions
 *     tensor rank
 *
 * Actual limits are implementation/resource constraints and MUST NOT become
 * language-level ceilings.
 *
 * Scalability testing must distinguish:
 *
 *     grammar complexity
 *     parser resource consumption
 *     AST resource consumption
 *     semantic-analysis resource consumption
 *     compiler resource consumption
 *     runtime resource consumption
 *
 * A finite implementation resource failure MUST NOT be transformed into a
 * false claim that the language has a semantic maximum.
 *
 * ============================================================================
 * ERROR CONTRACT
 * ============================================================================
 *
 * This facade must reject incomplete or malformed isolated assumptions through
 * the canonical grammar.
 *
 * Examples that MUST NOT be accepted as complete validation units:
 *
 *     assume;
 *     assume();
 *     assume(;
 *     assume(x;
 *     assume(x, y);
 *     assume(x);
 *     trailing
 *
 * The final case is invalid when supplied as a complete validation unit
 * because EOF is mandatory.
 *
 * Validity of the expression itself remains the responsibility of the
 * canonical expression grammar and downstream semantic analysis.
 *
 * ============================================================================
 * SOURCE-SPAN CONTRACT
 * ============================================================================
 *
 * The validation wrapper MUST NOT discard source positions supplied by the
 * canonical parser.
 *
 * Downstream diagnostics should be able to associate:
 *
 *     assumptionsValidationStatement
 *         |
 *         v
 *     assumeStatement
 *         |
 *         v
 *     contractCondition
 *
 * with the original source span.
 *
 * The wrapper itself does not manufacture source-span objects.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * This file is intentionally insulated from domain additions.
 *
 * Adding a new:
 *
 *     classical capability
 *     quantum capability
 *     HDL capability
 *     accelerator
 *     AI model
 *     distributed mechanism
 *     networking protocol
 *     data representation
 *     hardware target
 *     simulation strategy
 *
 * MUST NOT require changes to this file merely because the new domain can
 * appear inside an ordinary expression.
 *
 * Changes are required only if the canonical `assumeStatement` syntax itself
 * is intentionally versioned.
 *
 * Such a change belongs in:
 *
 *     grammar/statements/contract.g4
 *
 * and the relevant specification/conformance fixtures.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * Production parser path:
 *
 *     ZamaniParser
 *          |
 *          v
 *     sourceUnit
 *          |
 *          v
 *     statement/declaration dispatch
 *          |
 *          v
 *     ContractStatements
 *          |
 *          v
 *     assumeStatement
 *
 * Validation path:
 *
 *     assumptionsValidationUnit
 *          |
 *          v
 *     assumptionsValidationItem
 *          |
 *          v
 *     assumptionsValidationStatement
 *          |
 *          v
 *     ContractStatements.assumeStatement
 *
 * Therefore there is exactly one source-language implementation of `assume`.
 *
 * ============================================================================
 * ANTLR INTEGRATION
 * ============================================================================
 *
 * This grammar is a parser grammar and consumes the canonical Zamani lexer
 * vocabulary.
 *
 * The generated parser MUST use:
 *
 *     tokenVocab = ZamaniLexer
 *
 * and MUST import:
 *
 *     ContractStatements
 *
 * ANTLR grammar-library configuration must make the source directory
 * containing ContractStatements available during generation.
 *
 * This file MUST NOT import the lexical component grammar directly.
 *
 * ============================================================================
 * RUST INTEGRATION
 * ============================================================================
 *
 * This grammar does not contain Rust actions or predicates.
 *
 * Generated parser integration belongs to the repository's existing frontend
 * build process.
 *
 * The implementation baseline is:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *
 * Production Rust MUST remain safe Rust.
 *
 * This grammar MUST NOT require:
 *
 *     unsafe blocks
 *     unsafe functions
 *     unsafe traits
 *     unsafe implementations
 *     raw-pointer integration
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Required validation categories:
 *
 *     1. Positive tests
 *     2. Negative tests
 *     3. Boundary tests
 *     4. Source-span tests
 *     5. Determinism tests
 *     6. Scalability tests
 *     7. Compatibility tests
 *     8. Cross-domain tests
 *
 * Positive examples:
 *
 *     assume(x >= 0);
 *     assume(value.is_valid());
 *     assume(capability("quantum.measurement"));
 *     assume(memory >= required_memory);
 *     assume(model.is_valid());
 *     assume(signal.is_defined(clock));
 *     assume(topology.supports(required_topology));
 *
 * Negative examples:
 *
 *     assume;
 *     assume();
 *     assume(;
 *     assume(x;
 *     assume(x, y);
 *     assume(x) trailing
 *
 * Cross-domain examples MUST be tested through the canonical expression
 * architecture rather than by adding domain-specific alternatives here.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file MUST contain no target-derived constants.
 *
 * In particular, it MUST contain no universal limits for:
 *
 *     qubits
 *     CPUs
 *     GPUs
 *     FPGAs
 *     nodes
 *     memory
 *     threads
 *     tensor rank
 *     register width
 *     network size
 *     device count
 *
 * Resource quantities occurring inside source expressions are not grammar
 * limits.
 *
 * ============================================================================
 * INDEPENDENT COMPLETION CONTRACT
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     [ ] The file exists at grammar/validation/assumptions.g4.
 *
 *     [ ] The grammar name is `Assumptions`.
 *
 *     [ ] The file is a parser grammar.
 *
 *     [ ] `ZamaniLexer` is the only token vocabulary.
 *
 *     [ ] `ContractStatements` is the only source-language grammar dependency.
 *
 *     [ ] `assumeStatement` remains owned by ContractStatements.
 *
 *     [ ] `contractCondition` is not duplicated.
 *
 *     [ ] `expression` is not duplicated.
 *
 *     [ ] `statementTerminator` is not duplicated.
 *
 *     [ ] No lexer tokens are defined here.
 *
 *     [ ] No AST node is defined here.
 *
 *     [ ] No semantic evaluation occurs here.
 *
 *     [ ] No resource limit is encoded here.
 *
 *     [ ] No capability resolution occurs here.
 *
 *     [ ] No policy decision occurs here.
 *
 *     [ ] No provenance implementation occurs here.
 *
 *     [ ] No IR is constructed here.
 *
 *     [ ] No backend is selected here.
 *
 *     [ ] Quantum semantics remain downstream.
 *
 *     [ ] HDL semantics remain downstream.
 *
 *     [ ] Classical semantics remain downstream.
 *
 *     [ ] AI/reasoning semantics remain downstream.
 *
 *     [ ] Distributed semantics remain downstream.
 *
 *     [ ] Simulation semantics remain downstream.
 *
 *     [ ] EOF is mandatory for an isolated validation unit.
 *
 *     [ ] Valid complete assumptions parse.
 *
 *     [ ] Malformed assumptions are rejected.
 *
 *     [ ] Trailing source is rejected by the validation entry point.
 *
 *     [ ] Source spans remain available through the canonical parser context.
 *
 *     [ ] Rust 1.97 generation/integration succeeds.
 *
 *     [ ] Rust 1.97.1 generation/integration succeeds.
 *
 *     [ ] No unsafe Rust is required.
 *
 *     [ ] Existing production parser ownership remains unchanged.
 *
 *     [ ] Adding a new semantic domain does not require this file to change.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL GUARANTEE
 * ============================================================================
 *
 * This file establishes one validation facade:
 *
 *     assumptionsValidationUnit
 *              |
 *              v
 *     canonical assume syntax
 *              |
 *              v
 *     domain-neutral AST
 *              |
 *              v
 *     semantic contract model
 *              |
 *       +------+------+------+------+------+
 *       |      |      |      |      |      |
 *       v      v      v      v      v      v
 *     Types Effects Resources Capabilities Policies Provenance
 *              |
 *              v
 *     canonical semantic representation
 *              |
 *       +------+---------+---------+
 *       |                |         |
 *       v                v         v
 *   Classical        quantum::ir  HDL/HW
 *       |                |         |
 *       +----------------+---------+
 *                        |
 *                        v
 *              target-independent optimization
 *                        |
 *                        v
 *                 lowering / realization
 *
 * The validation facade therefore protects the single-source grammar
 * architecture while allowing assumptions to participate in the complete
 * Zamani semantic pipeline.
 *
 * ============================================================================
 * GRAMMAR
 * ============================================================================
 */

parser grammar Assumptions;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * ============================================================================
 * CANONICAL CONTRACT SYNTAX IMPORT
 * ============================================================================
 *
 * `ContractStatements` is the sole owner of the source-language `assume`
 * statement.
 *
 * This import is the only syntax integration required by this validation
 * facade.
 */
import
    ContractStatements
    ;


/*
 * ============================================================================
 * VALIDATION UNIT
 * ============================================================================
 *
 * An isolated validation input consists of zero or more canonical `assume`
 * statements followed by EOF.
 *
 * EOF is mandatory so that:
 *
 *     assume(x >= 0);
 *
 * is accepted as a complete validation unit, while:
 *
 *     assume(x >= 0); trailing
 *
 * is rejected as an incomplete validation unit.
 *
 * Multiple assumptions are intentionally supported:
 *
 *     assume(x >= 0);
 *     assume(y >= x);
 *     assume(result.is_valid());
 *
 * No fixed number of assumptions is encoded.
 */
assumptionsValidationUnit
    : assumptionsValidationItem* EOF
    ;


/*
 * ============================================================================
 * VALIDATION ITEM
 * ============================================================================
 *
 * This rule is deliberately only a validation-level wrapper.
 *
 * It delegates immediately to the canonical validation statement boundary.
 *
 * It does NOT reproduce:
 *
 *     ASSUME
 *     contractCondition
 *     expression
 *     statementTerminator
 */
assumptionsValidationItem
    : assumptionsValidationStatement
    ;


/*
 * ============================================================================
 * VALIDATION STATEMENT
 * ============================================================================
 *
 * Canonical source-language owner:
 *
 *     grammar/statements/contract.g4
 *         |
 *         v
 *     ContractStatements
 *         |
 *         v
 *     assumeStatement
 *
 * This rule provides a stable validation-facing context without introducing a
 * second implementation of the language construct.
 */
assumptionsValidationStatement
    : assumeStatement
    ;
/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/validation/assertions.g4
 *
 * GRAMMAR
 * -------
 * AssertionsValidation
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
 * Zamani's canonical statement-level assertion syntax.
 *
 * IMPORTANT:
 *
 * This file is NOT a second owner of assertion syntax.
 *
 * The canonical source-language assertion syntax is owned exclusively by:
 *
 *     grammar/statements/assertions.g4
 *
 *         -> AssertionsParser
 *
 *         -> assertionStatement
 *
 * This file only provides a validation-facing entry point around that
 * canonical rule.
 *
 * Therefore:
 *
 *     production parser
 *         |
 *         v
 *     AssertionsParser
 *         |
 *         v
 *     assertionStatement
 *
 * and:
 *
 *     validation parser
 *         |
 *         v
 *     AssertionsValidation
 *         |
 *         v
 *     assertionStatement
 *
 * both consume exactly the same source-language syntax.
 *
 * There must never be a second implementation of:
 *
 *     assert(...)
 *
 * in this file.
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
 *     - does not define ASSERT;
 *     - does not define LPAREN;
 *     - does not define RPAREN;
 *     - does not define COMMA;
 *     - does not define SEMICOLON;
 *     - does not define assertionStatement;
 *     - does not define assertionCondition;
 *     - does not define assertionExplanation;
 *     - does not define expression;
 *     - does not define statementTerminator;
 *     - does not define statement;
 *     - does not define blocks;
 *     - does not define contracts;
 *     - does not define types;
 *     - does not define resources;
 *     - does not define capabilities;
 *     - does not define effects;
 *     - does not define policies;
 *     - does not define provenance;
 *     - does not define quantum syntax;
 *     - does not define HDL syntax;
 *     - does not define hardware syntax;
 *     - does not define AI syntax;
 *     - does not define distributed syntax;
 *     - does not define runtime behavior;
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
 * Provide one complete-input validation boundary for an ordinary Zamani
 * assertion statement.
 *
 * The canonical source forms remain:
 *
 *     assert(condition);
 *
 *     assert(condition, explanation);
 *
 * Their actual syntax remains exclusively owned by:
 *
 *     grammar/statements/assertions.g4
 *
 * ============================================================================
 * OWNS
 * ============================================================================
 *
 * This file owns only the validation facade:
 *
 *     assertionsValidationUnit
 *     assertionsValidationItem
 *
 * These rules provide validation entry points and nothing more.
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does not own:
 *
 *     assertionStatement
 *     assertionCondition
 *     assertionExplanation
 *     expression
 *     statementTerminator
 *     lexer tokens
 *     keyword spelling
 *     punctuation
 *     statement composition
 *     AST structures
 *     semantic analysis
 *     effect analysis
 *     capability analysis
 *     resource analysis
 *     policy analysis
 *     provenance implementation
 *     IR generation
 *     runtime assertion behavior
 *     verification algorithms
 *     proof systems
 *     hardware realization.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON
 * ----------
 *
 *     grammar/statements/assertions.g4
 *         AssertionsParser
 *
 *         assertionStatement
 *
 *     grammar/antlr/ZamaniLexer.g4
 *         indirectly through tokenVocab = ZamaniLexer
 *
 *     grammar/expressions/expressions.g4
 *         indirectly through AssertionsParser
 *
 *     grammar/core/punctuation.g4
 *         indirectly through AssertionsParser
 *
 * ============================================================================
 * EXPORTS
 * ============================================================================
 *
 *     assertionsValidationUnit
 *     assertionsValidationItem
 *
 * These are validation-only entry points.
 *
 * They are not production source-language constructs.
 *
 * ============================================================================
 * CONSUMED BY
 * ============================================================================
 *
 * This facade may be consumed by:
 *
 *     grammar/tests/
 *     grammar/validation/
 *     grammar-validation tooling
 *     parser conformance tooling
 *     IDE/parser diagnostics
 *     grammar regression infrastructure
 *
 * It MUST NOT replace the production program parser.
 *
 * ============================================================================
 * AST OWNER
 * ============================================================================
 *
 * The AST remains owned by the repository frontend AST subsystem.
 *
 * This grammar creates parser contexts only.
 *
 * A validation parse must map to the same canonical AST representation as a
 * production parse.
 *
 * There must not be a validation-specific assertion AST merely because the
 * input was parsed through this facade.
 *
 * Conceptually:
 *
 *     assertionStatement
 *         |
 *         v
 *     canonical AST
 *         |
 *         v
 *     AssertionStatement
 *
 * The AST must preserve, as appropriate:
 *
 *     condition
 *     optional explanation
 *     source span
 *     enclosing source context
 *     source ordering
 *     provenance.
 *
 * ============================================================================
 * SEMANTIC OWNER
 * ============================================================================
 *
 * Semantic analysis owns assertion meaning.
 *
 * This grammar does not determine:
 *
 *     whether the condition is true;
 *     whether the condition is provable;
 *     whether it is evaluated at compile time;
 *     whether it is evaluated at runtime;
 *     whether it becomes a verification obligation;
 *     whether the assertion is optimized;
 *     whether the assertion is retained;
 *     whether failure terminates execution;
 *     whether failure is recoverable;
 *     whether the explanation is semantically valid.
 *
 * Those decisions belong downstream.
 *
 * A syntactically valid assertion may therefore still be semantically invalid.
 *
 * For example:
 *
 *     assert(unknown_name);
 *
 * is structurally valid but may fail name resolution.
 *
 * Likewise:
 *
 *     assert(value);
 *
 * may be structurally valid while failing the semantic predicate/type rules.
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * The assertion condition is an ordinary Zamani expression.
 *
 * This file does not introduce:
 *
 *     AssertionBoolean
 *     QuantumAssertionBoolean
 *     HardwareAssertionBoolean
 *     AIAssertionBoolean
 *
 * or any other domain-specific assertion type.
 *
 * The semantic/type system determines whether the condition is an acceptable
 * assertion predicate.
 *
 * The optional explanation is also an ordinary expression because the
 * canonical assertion grammar permits:
 *
 *     assert(condition, explanation);
 *
 * The semantic layer determines whether the explanation expression is valid
 * in the relevant assertion context.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Parsing an assertion has no runtime effect.
 *
 * The expression represented by an assertion may semantically reference
 * effectful operations, but this grammar does not execute them.
 *
 * Effect analysis remains owned by:
 *
 *     grammar/effects/
 *
 * and the corresponding semantic implementation.
 *
 * Possible effects may include, where supported by the language:
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
 * This file does not create an assertion-specific effect system.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Assertion expressions may refer to capability-related semantic values.
 *
 * For example:
 *
 *     assert(capability("quantum.measurement"));
 *
 * The grammar merely delegates the expression to the canonical expression
 * grammar.
 *
 * Capability resolution remains downstream.
 *
 * This file does not:
 *
 *     discover capabilities;
 *     query hardware;
 *     select a device;
 *     authorize execution;
 *     reject a target because of missing hardware.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Assertion expressions may refer to symbolic resource values.
 *
 * Examples include:
 *
 *     assert(memory_available >= required_memory);
 *
 *     assert(qubits_available >= required_qubits);
 *
 *     assert(topology.supports(required_topology));
 *
 * These are ordinary expressions.
 *
 * They are NOT grammar-level capacity declarations.
 *
 * This file introduces no finite limit on:
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
 *     tensor dimensions
 *     tensor rank
 *     registers
 *     network size.
 *
 * ============================================================================
 * CONTRACT CONTRACT
 * ============================================================================
 *
 * An assertion is deliberately distinct from:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * Those contract constructs remain owned by:
 *
 *     grammar/statements/contract.g4
 *
 * An assertion is a statement-level executable/checkable condition.
 *
 * A contract construct expresses a semantic obligation or property associated
 * with another semantic context.
 *
 * The validation facade MUST NOT merge these ownership boundaries.
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Assertion behavior may be constrained by policies concerning:
 *
 *     verification
 *     execution
 *     security
 *     diagnostics
 *     optimization
 *     simulation
 *     deployment
 *     resource usage
 *     reproducibility
 *
 * Policy syntax and semantics remain outside this file.
 *
 * Parsing an assertion MUST NOT grant permission to execute anything.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Validation and downstream semantic tooling must be able to associate an
 * assertion with its original source.
 *
 * At minimum, downstream infrastructure should preserve:
 *
 *     source file/module
 *     source span
 *     assertion kind
 *     condition
 *     optional explanation
 *     enclosing scope
 *     source order
 *     grammar/language version where applicable.
 *
 * Later stages may additionally attach:
 *
 *     derived_from
 *     generated_by
 *     transformed_by
 *     verified_by
 *     evidence
 *     decision
 *     execution context
 *     compilation provenance.
 *
 * This grammar does not implement provenance storage.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This file produces NO IR.
 *
 * The validation architecture remains:
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
 *         +--> provenance
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
 *         |
 *         v
 *     optimization
 *         |
 *         v
 *     lowering
 *         |
 *         v
 *     routing / scheduling / resilience where applicable
 *         |
 *         v
 *     ZQN / HAL / target realization where applicable
 *
 * Assertions must not bypass this architecture.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * An assertion may contain an expression whose semantic origin involves
 * quantum computation.
 *
 * Examples:
 *
 *     assert(measurement_result);
 *
 *     assert(measurement.is_valid());
 *
 *     assert(classical_result.is_consistent(measurement));
 *
 * This grammar does not determine whether an expression is quantum.
 *
 * It MUST NOT:
 *
 *     - enumerate quantum gates;
 *     - enumerate physical qubits;
 *     - define qubit identifiers;
 *     - define coupling maps;
 *     - define calibration;
 *     - define physical topology;
 *     - perform routing;
 *     - perform scheduling;
 *     - implement QEC;
 *     - select a QPU;
 *     - create another quantum IR.
 *
 * The canonical quantum path remains:
 *
 *     semantic model
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
 * Assertions merely constrain or check semantic values associated with that
 * computation.
 *
 * ============================================================================
 * HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * Assertions may reference HDL or hardware semantic values.
 *
 * Examples:
 *
 *     assert(signal.is_defined(clock));
 *
 *     assert(output.is_valid());
 *
 *     assert(hardware_state.is_consistent());
 *
 * This grammar MUST NOT encode:
 *
 *     fixed register width
 *     fixed bus width
 *     fixed memory capacity
 *     fixed FPGA dimensions
 *     fixed ASIC resource count
 *     fixed clock count
 *     fixed pipeline depth
 *     fixed device count
 *     physical device identifiers.
 *
 * Hardware feasibility remains downstream.
 *
 * ============================================================================
 * CLASSICAL BOUNDARY
 * ============================================================================
 *
 * Classical programs use exactly the same assertion validation boundary.
 *
 * No separate classical assertion syntax is introduced.
 *
 * Assertions may contain ordinary expressions involving:
 *
 *     values
 *     state
 *     data
 *     numerical computation
 *     symbolic computation
 *     algorithms
 *     resources
 *     capabilities
 *     execution state.
 *
 * ============================================================================
 * HYBRID BOUNDARY
 * ============================================================================
 *
 * Hybrid classical/quantum computation uses the same assertion syntax.
 *
 * An assertion may semantically relate:
 *
 *     classical values
 *     quantum-derived values
 *     measurements
 *     control state
 *     resources
 *     capabilities
 *     execution state.
 *
 * No hybrid-specific assertion grammar is introduced.
 *
 * ============================================================================
 * AI / REASONING BOUNDARY
 * ============================================================================
 *
 * Assertions may semantically concern:
 *
 *     models
 *     reasoning
 *     learning
 *     adaptation
 *     uncertainty
 *     evidence
 *     decisions
 *     provenance
 *     knowledge
 *
 * Examples:
 *
 *     assert(model.is_valid());
 *
 *     assert(confidence >= required_confidence);
 *
 *     assert(decision.has_evidence());
 *
 * The grammar remains domain-neutral.
 *
 * It does not enumerate algorithms, model families, application categories,
 * or AI-specific assertion forms.
 *
 * ============================================================================
 * DISTRIBUTED BOUNDARY
 * ============================================================================
 *
 * Assertions may semantically reference:
 *
 *     communication state
 *     consistency
 *     availability
 *     replication
 *     topology
 *     service state
 *     fault state.
 *
 * This grammar imposes no finite limit on:
 *
 *     nodes
 *     peers
 *     services
 *     channels
 *     messages
 *     devices.
 *
 * Distributed semantics remain downstream.
 *
 * ============================================================================
 * SIMULATION BOUNDARY
 * ============================================================================
 *
 * The same assertion syntax may be consumed by simulation and verification
 * infrastructure.
 *
 * It may therefore participate in:
 *
 *     classical simulation
 *     quantum simulation
 *     hardware simulation
 *     HDL simulation
 *     distributed simulation
 *     fault simulation
 *     performance analysis
 *     model validation.
 *
 * Simulation is an execution/analysis strategy, not a second assertion
 * language.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     source text
 *     canonical lexer vocabulary
 *     imported parser grammars
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
 *     resource discovery.
 *
 * Given identical source and parser configuration, the validation parse must
 * be structurally deterministic.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar introduces no finite language-level capacity for:
 *
 *     assertion count
 *     expression size
 *     expression nesting
 *     identifier size
 *     source-unit size
 *     program size
 *     number of modules
 *     number of functions
 *     number of quantum operations
 *     number of qubits
 *     processor count
 *     accelerator count
 *     node count
 *     device count
 *     memory capacity
 *     tensor rank
 *     network size.
 *
 * "Infinity" means that this grammar imposes no artificial universal finite
 * capacity ceiling.
 *
 * It does NOT claim that a physical implementation has infinite resources.
 *
 * Actual limits may arise from:
 *
 *     compiler resources
 *     parser implementation resources
 *     memory
 *     processing time
 *     runtime resources
 *     target resources
 *     security policy
 *     execution policy
 *     deployment policy.
 *
 * Those limits MUST remain implementation/resource concerns rather than
 * becoming language-level constants.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * The source assertion:
 *
 *     assert(condition);
 *
 * remains syntactically independent of the target.
 *
 * The same source may be compiled or validated against:
 *
 *     tiny systems
 *     embedded systems
 *     CPUs
 *     multicore systems
 *     GPUs
 *     FPGAs
 *     ASIC-oriented systems
 *     accelerators
 *     QPUs
 *     simulators
 *     HPC systems
 *     clusters
 *     distributed systems
 *     cloud systems
 *     future architectures.
 *
 * Changing target resources does not change assertion grammar.
 *
 * A target's inability to satisfy a semantic condition is not a reason for
 * this validation grammar to invent target-specific syntax.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file MUST contain no language-level capacity constants.
 *
 * In particular, it contains no:
 *
 *     MAX_ASSERTIONS
 *     MAX_ASSERTION_DEPTH
 *     MAX_EXPRESSION_SIZE
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
 * Numeric values appearing inside an assertion expression are ordinary
 * program values and MUST NOT be interpreted as grammar capacity declarations.
 *
 * Example:
 *
 *     assert(value < 1024);
 *
 * contains a program expression.
 *
 * It does not establish a language-level maximum of 1024.
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * This facade delegates syntax diagnostics to the canonical assertion grammar.
 *
 * Examples of malformed complete inputs include:
 *
 *     assert
 *
 *     assert()
 *
 *     assert(
 *
 *     assert(;
 *
 *     assert(, condition);
 *
 *     assert(condition;
 *
 *     assert(condition,);
 *
 *     assert(condition, explanation, extra);
 *
 *     assert(condition) trailing
 *
 * The exact diagnostic wording, source-span formatting, recovery strategy,
 * aggregation strategy, and IDE presentation belong to the repository's
 * diagnostic infrastructure.
 *
 * This file MUST NOT use embedded semantic actions to manufacture diagnostics.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * Parsing an assertion must never:
 *
 *     - execute the condition;
 *     - execute the explanation;
 *     - access the filesystem;
 *     - access the network;
 *     - inspect hardware;
 *     - invoke foreign code;
 *     - invoke native code;
 *     - access secrets;
 *     - allocate target resources;
 *     - perform quantum measurement;
 *     - select a device.
 *
 * Any later execution or evaluation is controlled by downstream semantic,
 * compiler, runtime, and security systems.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * The canonical syntax remains owned by:
 *
 *     grammar/statements/assertions.g4
 *
 * This facade intentionally follows that syntax instead of copying it.
 *
 * Therefore:
 *
 *     assert(condition);
 *
 * and:
 *
 *     assert(condition, explanation);
 *
 * are accepted exactly when `AssertionsParser.assertionStatement` accepts them.
 *
 * If assertion syntax changes in the future, the canonical owner must change
 * first:
 *
 *     grammar/statements/assertions.g4
 *
 * This facade should normally require no corresponding syntax rewrite because
 * it delegates to the canonical rule.
 *
 * This is essential for preventing parser divergence.
 *
 * ============================================================================
 * PRODUCTION PARSER INTEGRATION
 * ============================================================================
 *
 * Normal Zamani source parsing remains:
 *
 *     grammar/Zamani.g4
 *         |
 *         v
 *     canonical parser hierarchy
 *         |
 *         v
 *     grammar/statements/statements.g4
 *         |
 *         v
 *     AssertionsParser
 *         |
 *         v
 *     assertionStatement
 *
 * This file is NOT inserted into the normal source statement dispatcher.
 *
 * It is an isolated validation/conformance facade only.
 *
 * ============================================================================
 * VALIDATION INTEGRATION
 * ============================================================================
 *
 * Validation tooling may use:
 *
 *     assertionsValidationUnit
 *
 * to validate exactly one complete assertion source fragment.
 *
 * This creates:
 *
 *     validation facade
 *          |
 *          v
 *     canonical assertion syntax
 *
 * rather than:
 *
 *     validation syntax A
 *     +
 *     production syntax B
 *
 * There must be exactly one source-language assertion grammar.
 *
 * ============================================================================
 * EXPRESSION INTEGRATION
 * ============================================================================
 *
 * This file does not import or duplicate `Expressions` directly because the
 * canonical `AssertionsParser` already imports and owns its expression
 * dependency.
 *
 * The validation facade deliberately follows:
 *
 *     assertionsValidationUnit
 *         ->
 *     assertionStatement
 *         ->
 *     assertionCondition
 *         ->
 *     expression
 *
 * Therefore expression syntax remains centralized.
 *
 * This is important for future expansion involving:
 *
 *     classical expressions
 *     tensor expressions
 *     quantum-derived expressions
 *     hybrid expressions
 *     hardware expressions
 *     distributed expressions
 *     reasoning expressions
 *     uncertainty expressions
 *     provenance expressions
 *     policy-related expressions
 *     future domain expressions.
 *
 * A new expression feature should be added to the canonical expression
 * architecture, not to this validation facade.
 *
 * ============================================================================
 * PUNCTUATION INTEGRATION
 * ============================================================================
 *
 * Statement termination remains owned by:
 *
 *     grammar/core/punctuation.g4
 *
 * through:
 *
 *     AssertionsParser
 *         ->
 *     assertionStatement
 *         ->
 *     statementTerminator
 *
 * This file MUST NOT define another semicolon or terminator rule.
 *
 * ============================================================================
 * CONTRACT INTEGRATION
 * ============================================================================
 *
 * Assertions coexist with the canonical contract statements:
 *
 *     requires(...)
 *     ensures(...)
 *     invariant(...)
 *     assume(...)
 *     guarantee(...)
 *     property(...)
 *
 * Those constructs remain owned by:
 *
 *     grammar/statements/contract.g4
 *
 * and validation facades such as:
 *
 *     grammar/validation/requires.g4
 *     grammar/validation/ensures.g4
 *     grammar/validation/invariants.g4
 *     grammar/validation/guarantees.g4
 *     grammar/validation/properties.g4
 *
 * This file MUST NOT import `ContractStatements` merely because assertions
 * participate in validation.
 *
 * Assertion syntax is independently owned by:
 *
 *     grammar/statements/assertions.g4
 *
 * ============================================================================
 * AST INTEGRATION
 * ============================================================================
 *
 * The validation parse must use the same AST construction path as production
 * parsing.
 *
 * Required conceptual mapping:
 *
 *     assertionsValidationUnit
 *         |
 *         v
 *     assertionsValidationItem
 *         |
 *         v
 *     assertionStatement
 *         |
 *         v
 *     canonical AssertionStatement AST
 *
 * There must not be:
 *
 *     ValidationAssertionStatement
 *
 * solely because validation used a separate parser entry point.
 *
 * ============================================================================
 * SEMANTIC INTEGRATION
 * ============================================================================
 *
 * The semantic layer consumes the canonical assertion representation and may
 * perform:
 *
 *     name resolution
 *     type validation
 *     predicate validation
 *     explanation validation
 *     scope validation
 *     effect analysis
 *     capability analysis
 *     resource analysis
 *     policy analysis
 *     provenance attachment
 *     verification preparation
 *     runtime-check planning where permitted.
 *
 * None of these operations belong in this grammar.
 *
 * ============================================================================
 * IR INTEGRATION
 * ============================================================================
 *
 * Assertion semantics must reach the canonical semantic representation before
 * any domain-specific lowering.
 *
 * Depending on the surrounding computation, downstream representations may
 * include:
 *
 *     classical IR
 *     quantum::ir
 *     HDL/hardware representation
 *     distributed representation
 *     accelerator representation
 *     another explicitly defined domain representation.
 *
 * This grammar creates none of them.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE TESTS
 * --------------
 *
 * The validation facade must accept every input that the canonical assertion
 * grammar accepts as a complete assertion:
 *
 *     assert(true);
 *
 *     assert(false);
 *
 *     assert(condition);
 *
 *     assert(value == expected);
 *
 *     assert(condition, "message");
 *
 *     assert(condition, diagnostic);
 *
 *     assert(condition, format_error(context));
 *
 *     assert(computation());
 *
 *     assert((nested_expression));
 *
 *     assert(a + b == c);
 *
 *     assert(measurement_result);
 *
 *     assert(measure(q) == expected);
 *
 *     assert(distributed_result);
 *
 *     assert(hardware_result);
 *
 *     assert(accelerator_result);
 *
 * Cross-domain cases remain valid only when the referenced expression is
 * independently valid under the canonical expression grammar.
 *
 * ============================================================================
 * NEGATIVE TESTS
 * ============================================================================
 *
 * The following must not be accepted as complete validation inputs:
 *
 *     assert
 *
 *     assert()
 *
 *     assert(
 *
 *     assert(;
 *
 *     assert(, explanation);
 *
 *     assert(condition;
 *
 *     assert(condition,);
 *
 *     assert(condition, explanation, extra);
 *
 *     assert(condition) trailing;
 *
 *     assert((condition);
 *
 *     assert(condition));
 *
 * These failures must arise through the canonical assertion grammar and parser
 * error mechanism rather than duplicated validation syntax.
 *
 * ============================================================================
 * SEMANTIC-NEGATIVE TESTS
 * ============================================================================
 *
 * These may be syntactically valid but semantically invalid depending on the
 * language's semantic rules:
 *
 *     assert(unknown_name);
 *
 *     assert(non_predicate_value);
 *
 *     assert(invalid_domain_value);
 *
 *     assert(incompatible_diagnostic);
 *
 * Such cases MUST NOT be converted into grammar-specific alternatives merely
 * to make semantic validation easier.
 *
 * ============================================================================
 * BOUNDARY TESTS
 * ============================================================================
 *
 * Validation must cover:
 *
 *     one assertion
 *     deeply nested condition
 *     deeply nested explanation
 *     large symbolic expression
 *     large numerical expression
 *     large tensor expression
 *     large hybrid expression
 *     large quantum-derived expression
 *     large HDL/hardware expression
 *     qualified names
 *     generic values
 *     Unicode identifiers supported by the lexer
 *     capability expressions
 *     resource expressions
 *     provenance-related expressions
 *     policy-related expressions
 *     distributed state expressions
 *     model/reasoning expressions.
 *
 * The validation grammar itself must not change as these expression domains
 * expand.
 *
 * ============================================================================
 * SCALABILITY TESTS
 * ============================================================================
 *
 * Conformance tooling should progressively increase:
 *
 *     expression size
 *     expression nesting
 *     identifier size
 *     assertion source size
 *     surrounding program size
 *     number of assertions in a program
 *     number of domains represented by expressions.
 *
 * No grammar-level fixed limit is permitted.
 *
 * Practical parser/compiler resource exhaustion is an implementation concern,
 * not a language semantic ceiling.
 *
 * ============================================================================
 * CROSS-DOMAIN TESTS
 * ============================================================================
 *
 * The same assertion validation facade must remain usable around expressions
 * involving:
 *
 *     classical computation
 *     quantum computation
 *     hybrid computation
 *     HDL
 *     hardware intent
 *     AI/model computation
 *     reasoning
 *     learning
 *     adaptation
 *     uncertainty
 *     evidence
 *     provenance
 *     distributed computation
 *     networking
 *     data processing
 *     tensor computation
 *     accelerators
 *     scientific computation
 *     simulation.
 *
 * These domains must enter through the canonical expression/semantic systems.
 *
 * ============================================================================
 * DETERMINISM TESTS
 * ============================================================================
 *
 * Repeated parsing of identical:
 *
 *     source
 *     grammar version
 *     lexer configuration
 *     parser configuration
 *
 * must produce equivalent validation parse structure and source-span data.
 *
 * The validator must not depend on:
 *
 *     hardware availability
 *     resource discovery
 *     network state
 *     current time
 *     randomness
 *     target selection.
 *
 * ============================================================================
 * PORTABILITY TESTS
 * ============================================================================
 *
 * The source:
 *
 *     assert(condition);
 *
 * must remain syntactically identical when the eventual target changes.
 *
 * Test across target descriptions representing:
 *
 *     minimal systems
 *     embedded systems
 *     CPU systems
 *     multicore systems
 *     GPU systems
 *     FPGA systems
 *     ASIC-oriented systems
 *     accelerators
 *     QPUs
 *     simulators
 *     HPC systems
 *     clusters
 *     distributed systems
 *     cloud environments
 *     future targets.
 *
 * No target description is allowed to alter this validation grammar.
 *
 * ============================================================================
 * ROUND-TRIP CONTRACT
 * ============================================================================
 *
 * Where the repository formatter/printer supports assertions, the validation
 * architecture must preserve:
 *
 *     condition
 *     optional explanation
 *     statement structure
 *     source meaning.
 *
 * The validation facade itself does not own formatting.
 *
 * ============================================================================
 * RUST INTEGRATION
 * ============================================================================
 *
 * This file contains no Rust code.
 *
 * Generated parser integration must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Edition 2021
 *     safe Rust.
 *
 * This file must never require:
 *
 *     unsafe blocks
 *     unsafe functions
 *     unsafe traits
 *     raw-pointer APIs
 *     target-specific native code.
 *
 * ============================================================================
 * ANTLR COMPOSITION CONTRACT
 * ============================================================================
 *
 * The parser grammar consumes the canonical lexer vocabulary:
 *
 *     ZamaniLexer
 *
 * It imports the canonical assertion parser grammar:
 *
 *     AssertionsParser
 *
 * No lexical grammar is imported directly here.
 *
 * No expression grammar is imported directly here.
 *
 * No punctuation grammar is imported directly here.
 *
 * Those dependencies are already owned by AssertionsParser and are therefore
 * inherited through the canonical assertion rule.
 *
 * This avoids accidental creation of parallel parser dependencies.
 *
 * ============================================================================
 * INDEPENDENT COMPLETION CONTRACT
 * ============================================================================
 *
 * This file is DONE when all of the following are true:
 *
 *     [ ] File path is grammar/validation/assertions.g4.
 *
 *     [ ] Grammar name is AssertionsValidation.
 *
 *     [ ] It is a parser grammar.
 *
 *     [ ] tokenVocab is ZamaniLexer.
 *
 *     [ ] AssertionsParser is imported.
 *
 *     [ ] assertionStatement is NOT redefined.
 *
 *     [ ] assertionCondition is NOT redefined.
 *
 *     [ ] assertionExplanation is NOT redefined.
 *
 *     [ ] expression is NOT redefined.
 *
 *     [ ] statementTerminator is NOT redefined.
 *
 *     [ ] No lexer tokens are defined.
 *
 *     [ ] No semantic predicates are defined.
 *
 *     [ ] No semantic actions are defined.
 *
 *     [ ] No Rust is embedded.
 *
 *     [ ] No unsafe Rust is required.
 *
 *     [ ] No runtime execution occurs.
 *
 *     [ ] No hardware discovery occurs.
 *
 *     [ ] No resource discovery occurs.
 *
 *     [ ] No target selection occurs.
 *
 *     [ ] No backend selection occurs.
 *
 *     [ ] No capacity constants are encoded.
 *
 *     [ ] No quantum operation catalogue is encoded.
 *
 *     [ ] No physical qubit mapping is encoded.
 *
 *     [ ] No HDL implementation details are encoded.
 *
 *     [ ] No application-specific assertion language is introduced.
 *
 *     [ ] Production parsing continues through AssertionsParser.
 *
 *     [ ] Validation parsing delegates to assertionStatement.
 *
 *     [ ] Complete input requires EOF.
 *
 *     [ ] Positive tests exist.
 *
 *     [ ] Negative tests exist.
 *
 *     [ ] Semantic-negative tests exist.
 *
 *     [ ] Boundary tests exist.
 *
 *     [ ] Scalability tests exist.
 *
 *     [ ] Cross-domain tests exist.
 *
 *     [ ] Determinism tests exist.
 *
 *     [ ] Portability tests exist.
 *
 *     [ ] Round-trip tests exist where formatter support exists.
 *
 *     [ ] AST mapping remains canonical.
 *
 *     [ ] Semantic mapping remains canonical.
 *
 *     [ ] IR mapping remains downstream.
 *
 *     [ ] quantum::ir remains the canonical quantum boundary.
 *
 *     [ ] Rust 1.97 integration succeeds.
 *
 *     [ ] Rust 1.97.1 integration succeeds.
 *
 * ============================================================================
 * INTEGRATION GRAPH
 * ============================================================================
 *
 *     grammar/validation/assertions.g4
 *                    |
 *                    v
 *           AssertionsValidation
 *                    |
 *                    v
 *           AssertionsParser
 *                    |
 *                    v
 *          assertionStatement
 *                    |
 *          +---------+---------+
 *          |                   |
 *          v                   v
 * assertionCondition   assertionExplanation
 *          |                   |
 *          +---------+---------+
 *                    |
 *                    v
 *                expression
 *                    |
 *                    v
 *             domain-neutral AST
 *                    |
 *                    v
 *           structural validation
 *                    |
 *        +-----------+-----------+
 *        |           |           |
 *        v           v           v
 *      types      effects    capabilities
 *        |           |           |
 *        +-----------+-----------+
 *                    |
 *                    v
 *              resources
 *                    |
 *                    v
 *                policies
 *                    |
 *                    v
 *               provenance
 *                    |
 *                    v
 *       canonical semantic model
 *                    |
 *        +-----------+-----------+
 *        |           |           |
 *        v           v           v
 *    classical   quantum::ir   HDL/HW
 *        |           |           |
 *        +-----------+-----------+
 *                    |
 *                    v
 *            optimization/lowering
 *                    |
 *                    v
 *       routing/scheduling/resilience
 *                    |
 *                    v
 *                 ZQN/HAL
 *                    |
 *                    v
 *              target realization
 *
 * ============================================================================
 * FINAL ARCHITECTURAL GUARANTEE
 * ============================================================================
 *
 * This file introduces exactly one new abstraction:
 *
 *     an isolated validation entry point for the existing assertion syntax.
 *
 * It does NOT introduce another assertion language.
 *
 * Therefore:
 *
 *     one source syntax
 *     one canonical assertion owner
 *     one AST representation
 *     one semantic model
 *     one downstream IR architecture
 *
 * while allowing validation tooling to independently parse and verify a
 * complete assertion fragment.
 *
 * This preserves:
 *
 *     portability
 *     scalability
 *     deterministic parsing
 *     cross-domain extensibility
 *     quantum/classical/HDL neutrality
 *     target independence
 *     POCO-REAF
 *
 * without introducing grammar-level hardware ceilings.
 *
 * ============================================================================
 * GRAMMAR
 * ============================================================================
 */

parser grammar AssertionsValidation;

options {
    tokenVocab = ZamaniLexer;
}

import
    AssertionsParser
    ;


/*
 * ============================================================================
 * VALIDATION UNIT
 * ============================================================================
 *
 * Exactly one complete assertion is validated.
 *
 * EOF is mandatory.
 *
 * This is intentionally different from the production program parser, which
 * accepts assertions as part of a larger sequence of statements.
 *
 * Examples:
 *
 *     assert(condition);
 *
 *     assert(condition, explanation);
 *
 * are complete validation inputs.
 *
 * Inputs such as:
 *
 *     assert(condition); trailing
 *
 * are not complete assertion validation inputs and therefore cannot succeed.
 */
assertionsValidationUnit
    : assertionsValidationItem EOF
    ;


/*
 * ============================================================================
 * VALIDATION ITEM
 * ============================================================================
 *
 * This is only a stable validation-facing wrapper.
 *
 * It delegates directly to the canonical assertion statement.
 *
 * It does NOT reproduce assertion syntax.
 */
assertionsValidationItem
    : assertionStatement
    ;
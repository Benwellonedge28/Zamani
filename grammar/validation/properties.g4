/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/validation/properties.g4
 *
 * GRAMMAR
 * -------
 * PropertiesValidation
 *
 * STATUS
 * ------
 * Production validation/conformance component
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file provides the isolated parser-validation boundary for Zamani
 * property statements.
 *
 * Canonical source-language ownership remains in:
 *
 *     grammar/statements/contract.g4
 *         -> ContractStatements
 *         -> propertyStatement
 *
 * This file MUST NOT become a second owner of property syntax.
 *
 * The purpose of this file is to provide:
 *
 *     - isolated property conformance parsing;
 *     - complete-input validation;
 *     - parser regression testing;
 *     - positive syntax tests;
 *     - negative syntax tests;
 *     - boundary tests;
 *     - deterministic parsing tests;
 *     - source-span/diagnostic testing;
 *     - validation tooling integration.
 *
 * ============================================================================
 * ARCHITECTURAL PRINCIPLE
 * ============================================================================
 *
 * Source syntax has one owner.
 *
 * Therefore:
 *
 *     grammar/statements/contract.g4
 *                    |
 *                    v
 *             propertyStatement
 *                    |
 *                    v
 *     grammar/validation/properties.g4
 *                    |
 *                    v
 *          validation entry point
 *
 * This file delegates to the canonical rule.
 *
 * It does NOT redefine:
 *
 *     PROPERTY
 *     contractCondition
 *     expression
 *     statementTerminator
 *     identifiers
 *     names
 *     punctuation
 *     types
 *     resource expressions
 *     capability expressions
 *     policies
 *     provenance
 *     effects
 *     quantum syntax
 *     HDL syntax
 *     hardware syntax
 *     AI syntax
 *     distributed syntax
 *
 * ============================================================================
 * IMPLEMENTATION BASELINE
 * ============================================================================
 *
 * Rust:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Edition 2021
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
 *     - no runtime execution;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware discovery;
 *     - no target selection;
 *     - no resource allocation;
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
 * Expose the canonical Zamani property statement through a complete-input
 * validation boundary.
 *
 * OWNS
 * ----
 *
 *     propertyValidationUnit
 *     propertyValidationItem
 *     propertyValidationStatement
 *
 * These are validation-facing rules only.
 *
 * DOES NOT OWN
 * -------------
 *
 *     propertyStatement
 *     contractCondition
 *     expression
 *     PROPERTY
 *     statementTerminator
 *
 * Those remain owned by their canonical grammar layers.
 *
 * DEPENDS_ON
 * ----------
 *
 *     grammar/statements/contract.g4
 *         ContractStatements
 *         propertyStatement
 *
 *     grammar/antlr/ZamaniLexer.g4
 *         canonical lexer vocabulary through tokenVocab
 *
 *     grammar/expressions/
 *         indirectly through propertyStatement
 *
 *     grammar/core/punctuation.g4
 *         indirectly through propertyStatement
 *
 * EXPORTS
 * -------
 *
 *     propertyValidationUnit
 *     propertyValidationItem
 *     propertyValidationStatement
 *
 * CONSUMED_BY
 * ----------
 *
 *     grammar/tests/
 *     grammar/validation/
 *     grammar/conformance tooling
 *     isolated parser-validation tooling
 *
 * AST_OWNER
 * ---------
 *
 *     src/frontend/ast/
 *
 * This grammar creates parser contexts only.
 * It does not define Rust AST structures.
 *
 * SEMANTIC_OWNER
 * --------------
 *
 *     contract/property semantic validation layer
 *
 * IR_OWNER
 * --------
 *
 *     canonical semantic model
 *
 * Applicable domain lowering remains downstream.
 *
 * For quantum computation:
 *
 *     semantic model
 *          ->
 *     quantum::ir
 *
 * TEST_OWNER
 * ----------
 *
 *     grammar/tests/validation/properties/
 *
 * SPEC_OWNER
 * ----------
 *
 *     grammar/specification/contracts.md
 *     grammar/spec/contracts.md
 *
 * ============================================================================
 * CANONICAL SOURCE SYNTAX
 * ============================================================================
 *
 * The canonical source form is:
 *
 *     property(condition);
 *
 * where `condition` is an ordinary Zamani expression.
 *
 * The syntax itself is NOT reproduced here.
 *
 * This file delegates to:
 *
 *     propertyStatement
 *
 * from:
 *
 *     grammar/statements/contract.g4
 *
 * ============================================================================
 * OWNERSHIP INVARIANT
 * ============================================================================
 *
 * There MUST be exactly one source-language owner for:
 *
 *     propertyStatement
 *
 * That owner is:
 *
 *     grammar/statements/contract.g4
 *
 * This file MUST NOT define an alternative:
 *
 *     propertyStatement
 *
 * rule.
 *
 * It MUST NOT define:
 *
 *     PROPERTY
 *
 * as a lexer rule.
 *
 * It MUST NOT redefine:
 *
 *     LPAREN
 *     RPAREN
 *     statementTerminator
 *
 * It MUST NOT create a second expression grammar.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The canonical lexer is:
 *
 *     ZamaniLexer
 *
 * The PROPERTY token is supplied by the canonical lexical hierarchy.
 *
 * This grammar therefore uses:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * and does not define lexical tokens.
 *
 * A future change to the spelling of `property` belongs to the canonical
 * lexer/keyword ownership layer, not this validation file.
 *
 * ============================================================================
 * GRAMMAR CONTRACT
 * ============================================================================
 *
 * The canonical parser rule is:
 *
 *     propertyStatement
 *
 * This validation grammar wraps that rule in a complete-input boundary.
 *
 * The ownership chain is:
 *
 *     PROPERTY token
 *          |
 *          v
 *     ContractStatements
 *          |
 *          v
 *     propertyStatement
 *          |
 *          v
 *     propertyValidationStatement
 *          |
 *          v
 *     propertyValidationItem
 *          |
 *          v
 *     propertyValidationUnit
 *          |
 *          v
 *          EOF
 *
 * ============================================================================
 * COMPLETE-INPUT CONTRACT
 * ============================================================================
 *
 * `propertyValidationUnit` MUST consume EOF.
 *
 * Therefore:
 *
 *     property(condition);
 *
 * is a complete validation input.
 *
 * Whereas:
 *
 *     property(condition); trailing
 *
 * MUST NOT be accepted as a complete property validation unit.
 *
 * This prevents validation tooling from accidentally treating a valid prefix
 * as a complete source artifact.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar creates parser contexts only.
 *
 * The downstream AST must preserve the canonical property construct as a
 * domain-neutral contract/property node.
 *
 * At minimum, the downstream representation must be capable of preserving:
 *
 *     property kind
 *     condition expression
 *     source span
 *     source order
 *     enclosing scope
 *     enclosing declaration
 *     provenance
 *
 * The validation grammar does not define the Rust representation.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * A property is a domain-neutral semantic declaration associated with the
 * relevant source context.
 *
 * It may describe a property of:
 *
 *     classical computation
 *     quantum computation
 *     hybrid computation
 *     HDL
 *     hardware intent
 *     distributed computation
 *     networking
 *     data
 *     AI/ML
 *     security
 *     interoperability
 *     execution
 *     compilation
 *     simulation
 *     future computational domains
 *
 * The grammar does not determine the meaning of a property.
 *
 * Semantic analysis determines:
 *
 *     - what the property denotes;
 *     - where it is valid;
 *     - what type the condition requires;
 *     - whether it can be statically checked;
 *     - whether it becomes a proof obligation;
 *     - whether it becomes a runtime check;
 *     - whether it is informational metadata;
 *     - whether a dialect supplies additional semantics.
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * The condition remains an ordinary Zamani expression.
 *
 * This file does NOT introduce:
 *
 *     PropertyBoolean
 *     QuantumProperty
 *     HardwareProperty
 *     AIProperty
 *     HDLProperty
 *     DistributedProperty
 *
 * or any other domain-specific property type.
 *
 * Type checking remains the responsibility of the semantic/type subsystem.
 *
 * A property may ultimately require predicate-like semantics, richer logical
 * semantics, temporal semantics, probabilistic semantics, or domain-specific
 * semantic interpretation.
 *
 * Such interpretation MUST NOT be hard-coded into this parser facade.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Parsing a property does not evaluate it.
 *
 * Therefore this grammar itself introduces no runtime effect.
 *
 * If the property expression references an operation or semantic value with
 * effects, those effects remain governed by the existing effect system.
 *
 * Possible downstream effects include:
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
 * This file does not enumerate or implement effects.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * A property expression may semantically refer to capabilities.
 *
 * Example:
 *
 *     property(capability("quantum.measurement"));
 *
 * The parser does not determine whether the capability exists.
 *
 * Capability resolution belongs downstream:
 *
 *     property expression
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     capability analysis
 *
 * No hardware discovery occurs during parsing.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Properties may semantically refer to resource quantities or requirements.
 *
 * Examples include:
 *
 *     property(memory >= required_memory);
 *     property(qubits >= required_qubits);
 *     property(topology.supports(required_topology));
 *
 * These are program-level semantic expressions.
 *
 * This grammar introduces no universal resource ceilings.
 *
 * In particular, it does not define limits for:
 *
 *     memory
 *     qubits
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
 *     tensor rank
 *     register width
 *     network size
 *
 * ============================================================================
 * CONTRACT INTEGRATION
 * ============================================================================
 *
 * `propertyStatement` is one member of the canonical contract statement
 * family:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * The property validation facade MUST preserve this distinction.
 *
 * A property is not silently converted into:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *
 * and none of those constructs are redefined here.
 *
 * The semantic contract model may relate properties to other contract
 * obligations, but such relationships belong downstream.
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * A property's use or interpretation may be affected by:
 *
 *     verification policy
 *     security policy
 *     execution policy
 *     deployment policy
 *     resource policy
 *     adaptation policy
 *     reproducibility policy
 *
 * Policy evaluation remains outside this grammar.
 *
 * A policy MUST NOT be implemented as an ANTLR semantic predicate in this
 * file.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Validation and semantic tooling must preserve enough provenance to identify:
 *
 *     source file
 *     source span
 *     property kind
 *     condition
 *     enclosing module
 *     enclosing declaration
 *     semantic context
 *
 * Later stages may attach:
 *
 *     derived_from
 *     generated_by
 *     transformed_by
 *     verified_by
 *     evidence
 *     decision
 *     version
 *
 * This grammar does not implement provenance storage.
 *
 * ============================================================================
 * EVIDENCE / VERIFICATION CONTRACT
 * ============================================================================
 *
 * A property may become a verification obligation.
 *
 * Possible downstream consumers include:
 *
 *     static analysis
 *     symbolic analysis
 *     theorem proving
 *     model checking
 *     runtime verification
 *     simulation
 *     hardware verification
 *     quantum verification
 *     probabilistic analysis
 *     testing
 *
 * The parser only establishes syntactic conformance.
 *
 * A syntactically valid property is NOT automatically:
 *
 *     true
 *     proven
 *     verified
 *     guaranteed
 *     executable
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
 *     semantic property model
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
 *     canonical semantic model
 *       |
 *       +--> classical representation
 *       +--> quantum::ir
 *       +--> HDL/hardware representation
 *       +--> distributed representation
 *       +--> accelerator representation
 *       +--> future domain representation
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
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * A property may describe a quantum semantic condition.
 *
 * Example:
 *
 *     property(measurement.is_valid());
 *
 * or:
 *
 *     property(capability("quantum.measurement"));
 *
 * The grammar remains domain-neutral.
 *
 * If the enclosing computation is quantum, semantic lowering proceeds through
 * the canonical quantum boundary:
 *
 *     property semantics
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
 *
 * This file MUST NOT:
 *
 *     - enumerate quantum gates;
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
 * A property may describe HDL or hardware semantics.
 *
 * Examples:
 *
 *     property(signal.is_defined(clock));
 *     property(output.is_valid());
 *     property(hardware_state.is_consistent());
 *
 * This grammar does not encode:
 *
 *     fixed signal widths
 *     fixed register widths
 *     fixed memory capacities
 *     fixed FPGA dimensions
 *     fixed ASIC resource counts
 *     fixed device counts
 *     fixed topology sizes
 *     vendor-specific physical identifiers
 *
 * Hardware realization remains downstream.
 *
 * ============================================================================
 * CLASSICAL BOUNDARY
 * ============================================================================
 *
 * Classical computation uses the same property syntax.
 *
 * Example:
 *
 *     property(result >= input);
 *
 * There is no separate classical property language.
 *
 * ============================================================================
 * HYBRID BOUNDARY
 * ============================================================================
 *
 * Hybrid programs use the same property model across:
 *
 *     classical values
 *     quantum states
 *     measurement results
 *     accelerator results
 *     hardware state
 *     model outputs
 *
 * The parser does not determine which domain owns a value.
 *
 * Semantic analysis resolves the domain.
 *
 * ============================================================================
 * AI / REASONING BOUNDARY
 * ============================================================================
 *
 * Properties may describe:
 *
 *     model validity
 *     confidence
 *     evidence
 *     reasoning results
 *     knowledge consistency
 *     learning outcomes
 *     adaptation constraints
 *     decision conditions
 *     provenance
 *     uncertainty
 *
 * Example:
 *
 *     property(model.is_valid());
 *     property(decision.has_evidence());
 *     property(confidence >= required_confidence);
 *
 * These remain ordinary expressions.
 *
 * No application-specific keyword explosion is required.
 *
 * ============================================================================
 * DISTRIBUTED BOUNDARY
 * ============================================================================
 *
 * Properties may describe:
 *
 *     consistency
 *     communication state
 *     availability
 *     replication
 *     topology
 *     fault state
 *     message ordering
 *     distributed invariants
 *
 * Such meanings remain owned by distributed semantic analysis.
 *
 * This file does not create a second distributed property grammar.
 *
 * ============================================================================
 * BACKEND CONTRACT
 * ============================================================================
 *
 * Backends may consume property semantics to:
 *
 *     - prove properties;
 *     - preserve properties during lowering;
 *     - generate checks;
 *     - attach verification metadata;
 *     - reject unsupported obligations;
 *     - generate simulation checks;
 *     - generate hardware assertions;
 *     - propagate optimization constraints;
 *     - record verification evidence.
 *
 * None of these behaviors belong to this parser grammar.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Properties are source-level semantic intent.
 *
 * The same property source may accompany computation targeting:
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
 * The property syntax MUST NOT encode a universal physical capacity.
 *
 * "Infinity" means:
 *
 *     no artificial finite language-level ceiling is introduced by this file.
 *
 * It does NOT mean that a compiler, runtime, or physical target has infinite
 * resources.
 *
 * Actual resource exhaustion remains an implementation/environment condition.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This file introduces no finite maximum for:
 *
 *     property statements
 *     property expression size
 *     expression nesting
 *     source-unit size
 *     module count
 *     declaration count
 *     function count
 *     operation count
 *     quantum operation count
 *     qubit count
 *     processor count
 *     accelerator count
 *     device count
 *     node count
 *     tensor rank
 *     memory capacity
 *     network size
 *
 * Repetition and recursive structure are delegated to canonical grammar
 * ownership.
 *
 * The validation facade itself contains no artificial capacity parameter.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing MUST depend only on:
 *
 *     source token sequence
 *     canonical lexer vocabulary
 *     canonical parser grammar
 *     parser configuration
 *     explicitly selected language/dialect configuration
 *
 * Parsing MUST NOT depend on:
 *
 *     wall-clock time
 *     randomness
 *     hardware availability
 *     filesystem state
 *     network state
 *     environment variables
 *     runtime state
 *     scheduler state
 *     target availability
 *     resource availability
 *
 * The same source and grammar configuration must produce the same parse
 * structure.
 *
 * ============================================================================
 * SAFETY CONTRACT
 * ============================================================================
 *
 * This file contains no executable implementation.
 *
 * It cannot:
 *
 *     execute property expressions;
 *     access files;
 *     access networks;
 *     inspect hardware;
 *     allocate resources;
 *     select devices;
 *     invoke foreign functions;
 *     perform unsafe operations.
 *
 * The consuming Rust implementation remains safe Rust.
 *
 * No unsafe Rust is required or permitted for this grammar feature.
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Syntax diagnostics remain the responsibility of the parser/frontend, but
 * this validation entry point must permit deterministic diagnostics for cases
 * including:
 *
 *     property
 *     property()
 *     property(
 *     property(
 *     property(;
 *     property(, condition);
 *     property(condition;
 *     property(condition,)
 *     property(condition, other);
 *     property(condition) trailing
 *
 * The canonical grammar is responsible for the exact token-level diagnostic.
 *
 * Semantic diagnostics remain downstream and may include:
 *
 *     unknown name
 *     invalid expression
 *     invalid property context
 *     invalid property condition
 *     invalid type
 *     unavailable capability
 *     unavailable resource
 *     policy violation
 *     unsupported verification mode
 *     insufficient evidence
 *     invalid provenance
 *
 * A semantic failure MUST NOT be represented as a parser failure merely
 * because the property is semantically unsupported.
 *
 * ============================================================================
 * POSITIVE TEST CONTRACT
 * ============================================================================
 *
 * At minimum, validation tests must cover:
 *
 *     property(x >= 0);
 *     property(result.is_valid());
 *     property(state.is_consistent());
 *     property(capability("quantum.measurement"));
 *     property(memory >= required_memory);
 *     property(qubits >= required_qubits);
 *     property(topology.supports(required_topology));
 *     property(model.is_valid());
 *     property(decision.has_evidence());
 *     property(confidence >= required_confidence);
 *     property(signal.is_defined(clock));
 *     property(output.is_valid());
 *
 * Nested expressions must also be accepted when the canonical expression
 * grammar accepts them:
 *
 *     property((x + y) >= threshold);
 *     property(f(a, b).is_valid());
 *     property(state.field[index] == expected);
 *
 * ============================================================================
 * NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * At minimum:
 *
 *     property;
 *     property();
 *     property(;
 *     property(, x);
 *     property(x;
 *     property(x,);
 *     property(x, y);
 *     property(x) trailing
 *
 * These tests verify the validation boundary without duplicating expression
 * parsing.
 *
 * ============================================================================
 * BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Boundary tests must include:
 *
 *     property(classical_condition);
 *     property(quantum_condition);
 *     property(hybrid_condition);
 *     property(hardware_condition);
 *     property(distributed_condition);
 *     property(ai_condition);
 *     property(resource_condition);
 *     property(capability_condition);
 *     property(policy_condition);
 *     property(provenance_condition);
 *
 * The purpose is to verify that the property construct remains domain-neutral.
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Scalability tests should exercise property expressions whose size is limited
 * only by practical test/implementation resources.
 *
 * Required categories:
 *
 *     shallow property
 *     deeply nested property
 *     large expression property
 *     many sequential property statements
 *     large module property set
 *     large program property set
 *     cross-domain property set
 *
 * The tests MUST NOT encode a language-level maximum such as:
 *
 *     MAX_PROPERTIES
 *     MAX_PROPERTY_DEPTH
 *     MAX_PROPERTY_SIZE
 *
 * Any test-size bound is a test harness/resource parameter, never a language
 * semantic limit.
 *
 * ============================================================================
 * DETERMINISM TEST CONTRACT
 * ============================================================================
 *
 * The validation suite must verify:
 *
 *     same source -> same parse structure
 *     same source -> same diagnostics
 *     equivalent formatting -> equivalent parse structure
 *
 * where formatting is semantically irrelevant.
 *
 * The parser must not depend on:
 *
 *     target hardware
 *     runtime state
 *     resource availability
 *     random seeds
 *     current time
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing canonical property syntax remains:
 *
 *     property(condition);
 *
 * This file MUST NOT introduce a new spelling.
 *
 * If property syntax changes in the future:
 *
 *     1. change the canonical owner first;
 *     2. update the normative specification;
 *     3. classify compatibility impact;
 *     4. update conformance tests;
 *     5. retain this validation facade's delegation model.
 *
 * Compatibility logic belongs in:
 *
 *     grammar/compatibility/
 *
 * This file must not become a migration engine.
 *
 * ============================================================================
 * DIALECT CONTRACT
 * ============================================================================
 *
 * Dialects may add semantic interpretations of properties.
 *
 * A dialect MUST NOT redefine `propertyStatement` globally through this file.
 *
 * If a dialect requires genuinely different source syntax, that syntax must
 * have an explicit dialect owner and controlled integration into the parser
 * composition hierarchy.
 *
 * Core property syntax remains canonical and domain-neutral.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * Canonical source owner:
 *
 *     grammar/statements/contract.g4
 *
 * Canonical rule:
 *
 *     propertyStatement
 *
 * Validation facade:
 *
 *     grammar/validation/properties.g4
 *
 * Canonical lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Canonical parser composition:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * Root grammar:
 *
 *     grammar/Zamani.g4
 *
 * Production flow:
 *
 *     Zamani source
 *          |
 *          v
 *     Zamani.g4
 *          |
 *          v
 *     ZamaniParser
 *          |
 *          v
 *     statements
 *          |
 *          v
 *     ContractStatements
 *          |
 *          v
 *     propertyStatement
 *
 * Isolated validation flow:
 *
 *     property source
 *          |
 *          v
 *     PropertiesValidation
 *          |
 *          v
 *     propertyValidationUnit
 *          |
 *          v
 *     propertyValidationItem
 *          |
 *          v
 *     propertyValidationStatement
 *          |
 *          v
 *     propertyStatement
 *          |
 *          v
 *          EOF
 *
 * This separation ensures that validation tooling and production parsing
 * cannot silently drift into two different property syntaxes.
 *
 * ============================================================================
 * INTEGRATION WITH CONTRACT VALIDATION
 * ============================================================================
 *
 * The general validation facade:
 *
 *     grammar/validation/contracts.g4
 *
 * remains responsible for validating the complete contract family.
 *
 * This file provides property-specific isolated validation.
 *
 * Therefore:
 *
 *     contracts.g4
 *         |
 *         +--> contractStatement
 *                    |
 *                    +--> propertyStatement
 *
 * and:
 *
 *     properties.g4
 *         |
 *         +--> propertyStatement
 *
 * Both paths converge on the same canonical rule.
 *
 * No duplicated property grammar is permitted.
 *
 * ============================================================================
 * INTEGRATION WITH OTHER VALIDATION FILES
 * ============================================================================
 *
 * Sibling validation facades may exist for:
 *
 *     requires
 *     ensures
 *     invariants
 *     assumptions
 *     guarantees
 *     contracts
 *
 * Each facade must delegate to its canonical source owner.
 *
 * This file MUST NOT import those validation facades merely to compose
 * properties.
 *
 * The canonical contract grammar is the integration boundary.
 *
 * ============================================================================
 * INTEGRATION WITH EXPRESSIONS
 * ============================================================================
 *
 * `propertyStatement` delegates its condition to:
 *
 *     contractCondition
 *          |
 *          v
 *     expression
 *
 * Therefore this file inherits all canonical expression capabilities without
 * copying them.
 *
 * This includes, where supported by the expression grammar:
 *
 *     literals
 *     names
 *     calls
 *     indexing
 *     member access
 *     operators
 *     tuples
 *     arrays
 *     maps
 *     lambdas
 *     closures
 *     patterns
 *     guards
 *     reasoning
 *     knowledge
 *     uncertainty
 *     policy expressions
 *     quantum expressions
 *     hybrid expressions
 *     domain-specific expressions
 *
 * Adding a new expression capability does NOT require rewriting this file.
 *
 * ============================================================================
 * INTEGRATION WITH TYPES
 * ============================================================================
 *
 * Property conditions may refer to values governed by the canonical type
 * system.
 *
 * Type ownership remains outside this grammar.
 *
 * The property facade must remain unchanged when new types are added, provided
 * they use the existing expression/type architecture.
 *
 * ============================================================================
 * INTEGRATION WITH EFFECTS
 * ============================================================================
 *
 * Property expressions are checked against the existing effect model after
 * parsing.
 *
 * This file must remain unchanged when a new effect category is introduced,
 * provided the effect system integrates through the canonical semantic model.
 *
 * ============================================================================
 * INTEGRATION WITH RESOURCES AND CAPABILITIES
 * ============================================================================
 *
 * Resource and capability concepts may appear inside property expressions.
 *
 * Resolution occurs after parsing.
 *
 * Therefore a future capability such as:
 *
 *     capability("future.compute")
 *
 * does not require a change to this file.
 *
 * Likewise, a future resource quantity does not require a grammar-level
 * property change.
 *
 * This is essential for POCO-REAF scalability.
 *
 * ============================================================================
 * INTEGRATION WITH PROVENANCE
 * ============================================================================
 *
 * Property parser contexts must remain available to the frontend so that
 * source locations can be attached to the resulting AST/semantic property.
 *
 * The validation facade must not discard the canonical property context.
 *
 * ============================================================================
 * INTEGRATION WITH QUANTUM
 * ============================================================================
 *
 * Quantum properties remain properties.
 *
 * They do not become a separate grammar family.
 *
 * The quantum subsystem owns quantum semantics.
 *
 * This facade only exposes the universal property construct for validation.
 *
 * ============================================================================
 * INTEGRATION WITH HDL
 * ============================================================================
 *
 * HDL properties remain properties.
 *
 * Hardware verification semantics may consume them downstream.
 *
 * This file does not become an HDL assertion grammar.
 *
 * ============================================================================
 * INTEGRATION WITH AI / KNOWLEDGE / REASONING
 * ============================================================================
 *
 * Reasoning, knowledge, learning, uncertainty, evidence, decisions and
 * provenance may all participate in property expressions when supported by
 * the canonical expression and semantic systems.
 *
 * No application-specific property keywords are required.
 *
 * ============================================================================
 * INTEGRATION WITH FFI / ABI
 * ============================================================================
 *
 * A property may describe a condition involving foreign values or ABI-visible
 * state when such references are permitted by the semantic system.
 *
 * FFI and ABI ownership remains in:
 *
 *     grammar/interoperability/
 *
 * This file does not create FFI property syntax.
 *
 * ============================================================================
 * INTEGRATION WITH METAPROGRAMMING
 * ============================================================================
 *
 * Generated properties remain subject to the same semantic validation
 * pipeline as source-written properties.
 *
 * Macros and metaprogramming MUST NOT use this file as a bypass around:
 *
 *     type checking
 *     effect checking
 *     capability checking
 *     resource checking
 *     policy checking
 *     provenance
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when all of the following are true:
 *
 *     [x] canonical property syntax has exactly one source owner;
 *     [x] this file contains no duplicate property syntax;
 *     [x] PROPERTY is obtained from ZamaniLexer;
 *     [x] propertyStatement is imported from ContractStatements;
 *     [x] complete-input validation consumes EOF;
 *     [x] parser contexts remain available for diagnostics;
 *     [x] no AST is defined here;
 *     [x] no semantic evaluation is performed here;
 *     [x] no IR is produced here;
 *     [x] no hardware selection occurs here;
 *     [x] no resource availability is inspected here;
 *     [x] no machine-capacity constants exist here;
 *     [x] no semantic predicates exist here;
 *     [x] no embedded Rust exists here;
 *     [x] no unsafe implementation is required;
 *     [x] quantum semantics remain behind the quantum::ir boundary;
 *     [x] HDL semantics remain downstream;
 *     [x] resource/capability semantics remain downstream;
 *     [x] policy semantics remain downstream;
 *     [x] provenance remains preserved by downstream AST construction;
 *     [x] positive tests are defined;
 *     [x] negative tests are defined;
 *     [x] boundary tests are defined;
 *     [x] scalability tests are defined;
 *     [x] determinism tests are defined;
 *     [x] compatibility behavior is defined;
 *     [x] dialect behavior is defined;
 *     [x] integration ownership is explicit.
 *
 * ============================================================================
 * GRAMMAR
 * ============================================================================
 */

parser grammar PropertiesValidation;

options {
    tokenVocab = ZamaniLexer;
}

import
    ContractStatements
    ;


/*
 * ============================================================================
 * COMPLETE PROPERTY VALIDATION UNIT
 * ============================================================================
 *
 * This is the public isolated-validation entry point.
 *
 * EOF is mandatory.
 *
 * Examples:
 *
 *     property(x > 0);
 *
 * succeeds.
 *
 *     property(x > 0); trailing
 *
 * fails because trailing input remains after the canonical property statement.
 *
 * ============================================================================
 */
propertyValidationUnit
    : propertyValidationItem EOF
    ;


/*
 * ============================================================================
 * PROPERTY VALIDATION ITEM
 * ============================================================================
 *
 * This rule is intentionally a validation-facing boundary.
 *
 * It delegates to the canonical property validation statement rather than
 * reproducing property syntax.
 * ============================================================================
 */
propertyValidationItem
    : propertyValidationStatement
    ;


/*
 * ============================================================================
 * PROPERTY VALIDATION STATEMENT
 * ============================================================================
 *
 * Canonical source-language owner:
 *
 *     grammar/statements/contract.g4
 *         -> ContractStatements
 *         -> propertyStatement
 *
 * This alias exists so validation tooling has a stable property-specific
 * parser context without creating a competing source-language rule.
 * ============================================================================
 */
propertyValidationStatement
    : propertyStatement
    ;
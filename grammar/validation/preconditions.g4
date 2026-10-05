/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/validation/preconditions.g4
 *
 * GRAMMAR
 * -------
 * PreconditionsValidation
 *
 * STATUS
 * ------
 * Production validation/conformance component
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file provides the isolated validation/conformance entry point for
 * Zamani preconditions.
 *
 * A precondition is represented by the canonical `requires(...)` construct.
 *
 * IMPORTANT:
 *
 * This file is a VALIDATION FACADE.
 *
 * It is NOT a second owner of Zamani source-language contract syntax.
 *
 * Canonical standalone source syntax is owned by:
 *
 *     grammar/statements/contract.g4
 *         -> ContractStatements
 *         -> requiresStatement
 *
 * Canonical function-contract syntax is owned by:
 *
 *     grammar/functions/contracts.g4
 *         -> FunctionContracts
 *         -> functionRequiresContract
 *
 * This file composes those existing owners for:
 *
 *     - isolated grammar validation;
 *     - conformance testing;
 *     - parser regression testing;
 *     - negative syntax testing;
 *     - boundary testing;
 *     - scalability testing;
 *     - deterministic parser testing;
 *     - tooling;
 *     - grammar diagnostics.
 *
 * ============================================================================
 * CRITICAL OWNERSHIP RULE
 * ============================================================================
 *
 * This file MUST NOT define:
 *
 *     requiresStatement
 *     functionRequiresContract
 *     contractCondition
 *     expression
 *     functionContractTerminator
 *     statementTerminator
 *
 * Those rules already have canonical owners.
 *
 * The semantic concept "precondition" therefore has one meaning while being
 * usable in more than one syntactic context.
 *
 * ============================================================================
 * ARCHITECTURAL PRINCIPLE
 * ============================================================================
 *
 * A precondition is a semantic contract obligation.
 *
 * It is NOT inherently:
 *
 *     - a resource allocation;
 *     - a hardware selection;
 *     - a quantum routing instruction;
 *     - a scheduling instruction;
 *     - a backend instruction;
 *     - a proof;
 *     - a theorem;
 *     - a runtime authorization;
 *     - a device declaration;
 *     - a physical qubit declaration;
 *     - an AI-specific construct.
 *
 * The grammar accepts the expression.
 *
 * Semantic analysis determines what the expression means in its enclosing
 * context.
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
 * This file contains:
 *
 *     - no embedded Rust;
 *     - no semantic predicates;
 *     - no semantic actions;
 *     - no runtime execution;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware discovery;
 *     - no resource allocation;
 *     - no target selection;
 *     - no backend selection;
 *     - no quantum-device access;
 *     - no machine-capacity constants.
 *
 * ============================================================================
 * OWNS
 * ============================================================================
 *
 * This file owns ONLY these validation-facing rules:
 *
 *     preconditionsValidationUnit
 *     preconditionsValidationItem
 *     standalonePreconditionValidation
 *     functionPreconditionValidation
 *
 * These rules provide stable validation entry points and aliases.
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does not own:
 *
 *     lexer tokens
 *     keywords
 *     identifiers
 *     names
 *     expressions
 *     operators
 *     punctuation
 *     statement termination
 *     standalone contract syntax
 *     function declarations
 *     function contract blocks
 *     types
 *     effects
 *     capabilities
 *     resources
 *     policies
 *     provenance
 *     AST structures
 *     semantic contract structures
 *     classical IR
 *     quantum::ir
 *     HDL IR
 *     backend realization
 *
 * ============================================================================
 * DEPENDENCIES
 * ============================================================================
 *
 * CANONICAL STANDALONE CONTRACT OWNER
 *
 *     grammar/statements/contract.g4
 *
 * Provides:
 *
 *     ContractStatements
 *     requiresStatement
 *
 * CANONICAL FUNCTION CONTRACT OWNER
 *
 *     grammar/functions/contracts.g4
 *
 * Provides:
 *
 *     FunctionContracts
 *     functionRequiresContract
 *
 * CANONICAL LEXER
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Contract keyword identity ultimately comes from the canonical lexer.
 *
 * CANONICAL EXPRESSION OWNER
 *
 *     grammar/expressions/
 *
 * Expressions are imported transitively by the contract/function grammar
 * owners. This file MUST NOT reproduce expression syntax.
 *
 * ============================================================================
 * IMPORT POLICY
 * ============================================================================
 *
 * Both canonical owners are imported deliberately.
 *
 * This does NOT create two implementations of `requires`.
 *
 * The ownership graph is:
 *
 *     PreconditionsValidation
 *             |
 *       +-----+-----+
 *       |           |
 *       v           v
 * ContractStatements  FunctionContracts
 *       |           |
 *       v           v
 * requiresStatement  functionRequiresContract
 *       |           |
 *       +-----+-----+
 *             |
 *             v
 *       same semantic
 *       precondition
 *
 * The two imported rules represent different enclosing syntactic contexts:
 *
 *     standalone statement
 *
 * and:
 *
 *     function contract item
 *
 * ============================================================================
 * LEXICAL AUTHORITY
 * ============================================================================
 *
 * The canonical lexical spelling for the source-level precondition construct
 * is:
 *
 *     requires
 *
 * represented by:
 *
 *     REQUIRES
 *
 * This file does not define REQUIRES.
 *
 * The lexer already contains a reserved:
 *
 *     PRECONDITION
 *
 * token.
 *
 * That token MUST NOT be used here merely to manufacture an alternative
 * source-language spelling.
 *
 * In particular, this file MUST NOT introduce:
 *
 *     precondition(...)
 *
 * unless a future language specification explicitly promotes that spelling
 * and assigns it a canonical source-grammar owner.
 *
 * The semantic term "precondition" and the source keyword `requires` are
 * intentionally distinct.
 *
 * ============================================================================
 * WHY `requires` IS THE CANONICAL FORM
 * ============================================================================
 *
 * Zamani's portable contract model uses:
 *
 *     requires(condition);
 *
 * as the source-level representation of a precondition.
 *
 * This permits one construct to represent:
 *
 *     logical preconditions;
 *     capability requirements;
 *     resource requirements;
 *     type-state requirements;
 *     authorization requirements;
 *     execution requirements;
 *     portability constraints;
 *     domain-specific semantic conditions.
 *
 * The grammar does not classify those meanings.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This file creates parser contexts only.
 *
 * It MUST NOT define Rust AST structures.
 *
 * The downstream pipeline is:
 *
 *     parser context
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic contract model
 *
 * The AST must preserve enough information to identify:
 *
 *     - the contract kind;
 *     - the condition expression;
 *     - source span;
 *     - enclosing declaration/scope;
 *     - source order;
 *     - provenance.
 *
 * The validation facade does not own those structures.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantically:
 *
 *     requires(condition)
 *
 * establishes a condition that must hold at the relevant contract boundary.
 *
 * The relevant boundary is determined by the enclosing semantic construct.
 *
 * For example:
 *
 *     requires(x >= 0);
 *
 * inside a function contract means a function precondition.
 *
 * The same source-level form outside a function contract can represent a
 * standalone semantic requirement when the surrounding language construct
 * permits it.
 *
 * This grammar does not decide that context.
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * The condition is an ordinary Zamani expression.
 *
 * This file MUST NOT introduce:
 *
 *     PreconditionBoolean
 *     QuantumPrecondition
 *     HardwarePrecondition
 *     AICondition
 *     ResourceCondition
 *
 * or equivalent domain-specific grammar types.
 *
 * Semantic/type analysis determines whether the condition is valid.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Parsing a precondition does not evaluate it.
 *
 * Consequently this grammar produces no runtime effect.
 *
 * If the condition refers to an expression whose semantic interpretation
 * carries effects such as:
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
 * those effects are handled by the canonical effect system.
 *
 * This file does not create a second effect system.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Preconditions may contain capability requirements.
 *
 * Example:
 *
 *     requires(capability("quantum.measurement"));
 *
 * This grammar accepts the expression through the canonical contract grammar.
 *
 * Capability resolution happens downstream:
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
 *     capability resolution
 *
 * This file MUST NOT inspect target capabilities.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Preconditions may express portable resource requirements.
 *
 * Examples:
 *
 *     requires(memory >= required_memory);
 *     requires(qubits >= required_qubits);
 *     requires(capability("tensor.compute"));
 *     requires(topology.supports(required_topology));
 *
 * These are semantic expressions.
 *
 * This file does not establish physical capacity.
 *
 * It MUST NOT define limits for:
 *
 *     CPUs
 *     cores
 *     threads
 *     GPUs
 *     FPGAs
 *     ASIC resources
 *     accelerators
 *     QPUs
 *     qubits
 *     memory
 *     storage
 *     nodes
 *     devices
 *     tensor rank
 *     register width
 *     network size
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Preconditions may participate in policy analysis.
 *
 * Policy semantics remain owned by the policy subsystem.
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
 *     security policy
 *     resource policy
 *     execution policy
 *     deployment policy
 *     adaptation policy
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * A validated precondition must remain traceable to its source.
 *
 * Downstream infrastructure must be able to associate the precondition with:
 *
 *     source file
 *     source span
 *     enclosing scope
 *     contract context
 *     condition
 *
 * Later stages may additionally attach:
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
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Classical programs use the same precondition representation.
 *
 * Example:
 *
 *     requires(input >= 0);
 *
 * No classical-specific precondition grammar is necessary.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum programs use the same precondition representation.
 *
 * Examples:
 *
 *     requires(capability("quantum.measurement"));
 *     requires(qubits >= required_qubits);
 *     requires(input_state.is_valid());
 *
 * This grammar does NOT:
 *
 *     - enumerate quantum gates;
 *     - enumerate physical qubits;
 *     - define topology;
 *     - define coupling maps;
 *     - define calibration;
 *     - perform routing;
 *     - perform scheduling;
 *     - perform QEC;
 *     - create a quantum IR;
 *     - select a QPU.
 *
 * The downstream quantum boundary remains:
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
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Hardware and HDL requirements remain ordinary semantic expressions.
 *
 * Examples:
 *
 *     requires(signal.is_defined(clock));
 *     requires(memory >= required_memory);
 *     requires(capability("hardware.compute"));
 *
 * No hardware-specific precondition grammar is introduced.
 *
 * ============================================================================
 * HYBRID INTEGRATION
 * ============================================================================
 *
 * Classical/quantum/hardware hybrid programs use the same construct:
 *
 *     requires(input.is_valid());
 *     requires(capability("quantum.measurement"));
 *     requires(capability("tensor.compute"));
 *
 * The grammar remains domain-neutral.
 *
 * ============================================================================
 * AI / REASONING INTEGRATION
 * ============================================================================
 *
 * Preconditions may constrain:
 *
 *     models
 *     knowledge
 *     evidence
 *     confidence
 *     decisions
 *     learning
 *     adaptation
 *     reasoning
 *     provenance
 *
 * Examples:
 *
 *     requires(model.is_valid());
 *     requires(confidence >= required_confidence);
 *     requires(evidence.is_sufficient());
 *
 * No AI-specific precondition grammar is necessary.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Distributed computation may express requirements such as:
 *
 *     requires(consistency.is_satisfied());
 *     requires(capability("distributed.compute"));
 *     requires(topology.supports(required_topology));
 *
 * The distributed subsystem determines the meaning and feasibility.
 *
 * This file does not define:
 *
 *     node counts;
 *     topology sizes;
 *     network limits;
 *     fixed cluster dimensions.
 *
 * ============================================================================
 * FUNCTION-CONTRACT INTEGRATION
 * ============================================================================
 *
 * Function contracts are owned by:
 *
 *     grammar/functions/contracts.g4
 *
 * Example:
 *
 *     contract {
 *         requires(x >= 0);
 *     }
 *
 * This validation facade delegates to:
 *
 *     functionRequiresContract
 *
 * It does not reproduce function-contract syntax.
 *
 * The function grammar remains responsible for placing the contract block
 * relative to the function declaration.
 *
 * ============================================================================
 * STANDALONE CONTRACT INTEGRATION
 * ============================================================================
 *
 * Standalone contract statements are owned by:
 *
 *     grammar/statements/contract.g4
 *
 * Example:
 *
 *     requires(x >= 0);
 *
 * This validation facade delegates to:
 *
 *     requiresStatement
 *
 * It does not reproduce the source syntax.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar produces NO IR.
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
 *     semantic analysis
 *       |
 *       +--> type checking
 *       +--> effect checking
 *       +--> capability checking
 *       +--> resource checking
 *       +--> contract checking
 *       +--> policy checking
 *       +--> provenance
 *       |
 *       v
 *     canonical semantic representation
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
 *       v
 *     optimization
 *       |
 *       v
 *     lowering
 *       |
 *       v
 *     target realization
 *
 * ============================================================================
 * COMPILER INTEGRATION
 * ============================================================================
 *
 * The production compiler MUST continue to parse normal source programs
 * through:
 *
 *     grammar/Zamani.g4
 *         |
 *         v
 *     grammar/antlr/ZamaniParser.g4
 *
 * and its subordinate grammar hierarchy.
 *
 * This validation grammar is not the production program entry point.
 *
 * It exists so tooling can validate an isolated precondition without needing
 * to construct a complete program.
 *
 * ============================================================================
 * TOOLING INTEGRATION
 * ============================================================================
 *
 * Consumers may use:
 *
 *     preconditionsValidationUnit
 *
 * for:
 *
 *     - editor diagnostics;
 *     - grammar conformance;
 *     - parser regression tests;
 *     - documentation examples;
 *     - language-server validation;
 *     - syntax highlighting tests;
 *     - isolated parser tests;
 *     - generated parser fixtures.
 *
 * Tooling MUST NOT treat successful parsing as proof of semantic validity.
 *
 * ============================================================================
 * VALIDATION MODES
 * ============================================================================
 *
 * The validation unit intentionally accepts both canonical contexts:
 *
 *     standalone requires statement
 *
 * and:
 *
 *     function contract requires item
 *
 * This is useful because the semantic concept is the same while the
 * enclosing syntactic ownership differs.
 *
 * ============================================================================
 * COMPLETE INPUT CONTRACT
 * ============================================================================
 *
 * `preconditionsValidationUnit` consumes the entire input through EOF.
 *
 * Therefore an isolated validation invocation cannot silently accept:
 *
 *     requires(x >= 0); trailing
 *
 * as a complete valid input.
 *
 * Complete-input validation is essential for conformance tooling.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * The grammar introduces no artificial finite limit on:
 *
 *     number of preconditions;
 *     expression size;
 *     expression nesting;
 *     source-file size;
 *     number of declarations;
 *     number of functions;
 *     number of modules;
 *     number of quantum operations;
 *     number of qubits;
 *     number of processors;
 *     number of devices;
 *     number of nodes;
 *     amount of memory;
 *     tensor rank;
 *     network size.
 *
 * The grammar therefore scales structurally with the language.
 *
 * Actual limits remain implementation/resource limits and MUST NOT be
 * presented as language-level semantic ceilings.
 *
 * "Infinity" means no artificial universal grammar ceiling, not that finite
 * hardware or finite compiler memory can process an actually infinite input.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     source text;
 *     canonical lexer vocabulary;
 *     imported parser grammars;
 *     parser configuration.
 *
 * Parsing MUST NOT depend on:
 *
 *     wall-clock time;
 *     randomness;
 *     hardware availability;
 *     filesystem state;
 *     network state;
 *     runtime state;
 *     target selection;
 *     scheduler state.
 *
 * ============================================================================
 * SAFETY CONTRACT
 * ============================================================================
 *
 * This file contains grammar declarations only.
 *
 * It cannot:
 *
 *     execute expressions;
 *     access files;
 *     access networks;
 *     access secrets;
 *     inspect hardware;
 *     allocate resources;
 *     select devices.
 *
 * The Rust implementation generated/consumed by the frontend must remain
 * compatible with Rust 1.97 / Rust 1.97.1 and must use safe Rust.
 *
 * ============================================================================
 * DIAGNOSTICS CONTRACT
 * ============================================================================
 *
 * Syntax errors belong to the parser layer.
 *
 * Examples of malformed isolated input:
 *
 *     requires
 *     requires()
 *     requires(
 *     requires(, x)
 *     requires(x
 *     requires(x,)
 *     requires(x, y)
 *     requires(x) trailing
 *
 * These must be rejected according to the canonical imported grammar.
 *
 * Semantic errors remain downstream, for example:
 *
 *     unknown name;
 *     invalid expression type;
 *     unavailable capability;
 *     unavailable resource;
 *     invalid policy;
 *     invalid contract context;
 *     unsupported verification obligation.
 *
 * A semantic failure MUST NOT be incorrectly reported as a lexical or parser
 * failure merely because its target is unavailable.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Canonical source spelling:
 *
 *     requires(condition);
 *
 * Function contract spelling:
 *
 *     contract {
 *         requires(condition);
 *     }
 *
 * This file does not introduce:
 *
 *     precondition(condition);
 *
 * as an alternative spelling.
 *
 * Although `PRECONDITION` exists in the lexical vocabulary, its mere lexical
 * presence does not make a source construct part of the production language.
 *
 * If a future language specification promotes:
 *
 *     precondition(...)
 *
 * that change MUST:
 *
 *     1. be specified normatively;
 *     2. assign one canonical source grammar owner;
 *     3. define AST semantics;
 *     4. define semantic compatibility with `requires`;
 *     5. define migration behavior;
 *     6. define diagnostics;
 *     7. define tests;
 *     8. update the lexical/conformance status.
 *
 * This file must then be updated only if its validation contract genuinely
 * changes.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no universal resource constants.
 *
 * It MUST NOT contain or introduce equivalent forms of:
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
 * Numeric values appearing inside a user's condition remain user program
 * semantics.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * The validation test owner should be:
 *
 *     grammar/tests/validation/preconditions/
 *
 * Additional cross-cutting tests may live under:
 *
 *     grammar/tests/negative/
 *     grammar/tests/boundary/
 *     grammar/tests/scalability/
 *     grammar/tests/determinism/
 *     grammar/tests/compatibility/
 *
 * ============================================================================
 * POSITIVE TESTS
 * ============================================================================
 *
 * Standalone:
 *
 *     requires(x >= 0);
 *
 *     requires(capability("quantum.measurement"));
 *
 *     requires(memory >= required_memory);
 *
 *     requires(qubits >= required_qubits);
 *
 *     requires(capability("tensor.compute"));
 *
 *     requires(topology.supports(required_topology));
 *
 * Function contract item:
 *
 *     requires(x >= 0);
 *
 * Cross-domain:
 *
 *     requires(input.is_valid());
 *
 *     requires(model.is_valid());
 *
 *     requires(confidence >= required_confidence);
 *
 *     requires(signal.is_defined(clock));
 *
 *     requires(consistency.is_satisfied());
 *
 * ============================================================================
 * NEGATIVE TESTS
 * ============================================================================
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
 *     requires(x) trailing;
 *
 *     precondition(x);
 *
 * The final example is intentionally rejected by this validation grammar.
 *
 * `precondition` is a semantic term/reserved lexical spelling, not the
 * canonical source-level contract syntax.
 *
 * ============================================================================
 * BOUNDARY TESTS
 * ============================================================================
 *
 * Test:
 *
 *     requires((x));
 *
 *     requires(((x)));
 *
 *     requires(very.deeply.qualified.condition);
 *
 *     requires(capability("quantum.measurement"));
 *
 *     requires(capability("tensor.compute"));
 *
 *     requires(memory >= required_memory);
 *
 *     requires(qubits >= required_qubits);
 *
 *     requires(topology.supports(required_topology));
 *
 *     requires(model.is_valid());
 *
 *     requires(evidence.is_sufficient());
 *
 *     requires(input.is_valid());
 *
 *     requires(output.is_valid());
 *
 * ============================================================================
 * CROSS-DOMAIN TESTS
 * ============================================================================
 *
 * The same validation facade must support conditions associated with:
 *
 *     classical computation;
 *     quantum computation;
 *     hybrid computation;
 *     HDL;
 *     hardware/software co-design;
 *     AI;
 *     reasoning;
 *     learning;
 *     adaptation;
 *     distributed computation;
 *     networking;
 *     data;
 *     security;
 *     simulation;
 *     accelerators;
 *     FFI/ABI.
 *
 * No new grammar branch is permitted merely because another domain is added.
 *
 * ============================================================================
 * SCALABILITY TESTS
 * ============================================================================
 *
 * Tests should demonstrate that:
 *
 *     - multiple preconditions are supported;
 *     - large expressions are supported;
 *     - deeply nested valid expressions follow the canonical expression
 *       implementation limits rather than a precondition-specific limit;
 *     - large source units do not encounter a precondition-specific ceiling;
 *     - no target-size assumption enters parsing.
 *
 * The tests MUST distinguish:
 *
 *     language limits
 *
 * from:
 *
 *     implementation/resource limits.
 *
 * ============================================================================
 * DETERMINISM TESTS
 * ============================================================================
 *
 * Parse identical inputs repeatedly and verify equivalent parser structures.
 *
 * Results MUST NOT depend on:
 *
 *     target hardware;
 *     available QPU;
 *     number of CPUs;
 *     available memory;
 *     current time;
 *     randomness;
 *     network state.
 *
 * ============================================================================
 * PORTABILITY TESTS
 * ============================================================================
 *
 * Verify that identical source-level preconditions remain syntactically
 * identical when the eventual target changes between:
 *
 *     embedded;
 *     CPU;
 *     multicore;
 *     GPU;
 *     FPGA;
 *     ASIC;
 *     accelerator;
 *     QPU;
 *     simulator;
 *     HPC;
 *     cluster;
 *     distributed;
 *     cloud;
 *     future target.
 *
 * Target feasibility is a downstream concern.
 *
 * ============================================================================
 * NO SECOND CONTRACT LANGUAGE
 * ============================================================================
 *
 * This file MUST NOT introduce:
 *
 *     quantumPrecondition
 *     gpuPrecondition
 *     fpgaPrecondition
 *     hdlPrecondition
 *     distributedPrecondition
 *     aiPrecondition
 *     tensorPrecondition
 *     hardwarePrecondition
 *
 * The same semantic contract mechanism must work across domains.
 *
 * ============================================================================
 * NO PROOF ENGINE
 * ============================================================================
 *
 * A precondition can become a proof obligation.
 *
 * This file does not prove it.
 *
 * The architecture is:
 *
 *     requires(condition)
 *          |
 *          v
 *     semantic precondition
 *          |
 *          v
 *     verification/proof subsystem
 *
 * ============================================================================
 * NO RUNTIME EXECUTION
 * ============================================================================
 *
 * Runtime checking, monitoring, instrumentation and recovery remain owned by
 * execution/runtime subsystems.
 *
 * This validation facade only parses an isolated source fragment.
 *
 * ============================================================================
 * NO RESOURCE ALLOCATION
 * ============================================================================
 *
 * A precondition cannot allocate:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     memory
 *     storage
 *     network bandwidth
 *     physical qubits
 *     devices
 *
 * It can express a requirement which is interpreted downstream.
 *
 * ============================================================================
 * INTEGRATION GRAPH
 * ============================================================================
 *
 *                         preconditions.g4
 *                                |
 *                                v
 *                  PreconditionsValidation
 *                                |
 *                    +-----------+-----------+
 *                    |                       |
 *                    v                       v
 *             ContractStatements     FunctionContracts
 *                    |                       |
 *                    v                       v
 *             requiresStatement     functionRequiresContract
 *                    |                       |
 *                    +-----------+-----------+
 *                                |
 *                                v
 *                       canonical expression
 *                                |
 *                                v
 *                     domain-neutral AST
 *                                |
 *                                v
 *                     structural validation
 *                                |
 *              +-----------------+-----------------+
 *              |                 |                 |
 *              v                 v                 v
 *            Types            Effects          Capabilities
 *              |                 |                 |
 *              +-----------------+-----------------+
 *                                |
 *                                v
 *                            Resources
 *                                |
 *                                v
 *                             Policies
 *                                |
 *                                v
 *                           Provenance
 *                                |
 *                                v
 *                    canonical semantic model
 *                                |
 *                +---------------+---------------+
 *                |               |               |
 *                v               v               v
 *            Classical       quantum::ir       HDL/HW
 *                |               |               |
 *                +---------------+---------------+
 *                                |
 *                                v
 *                       optimization/lowering
 *                                |
 *                                v
 *                     routing/scheduling/etc.
 *                                |
 *                                v
 *                           ZQN / HAL
 *                                |
 *                                v
 *                        target realization
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/statements/contract.g4
 *     grammar/functions/contracts.g4
 *     canonical lexer vocabulary
 *     canonical expression grammar through imported owners
 *
 * EXPORTS:
 *
 *     preconditionsValidationUnit
 *     preconditionsValidationItem
 *     standalonePreconditionValidation
 *     functionPreconditionValidation
 *
 * CONSUMED_BY:
 *
 *     validation tooling
 *     grammar conformance tests
 *     parser regression tests
 *     editor/language-server validation where configured
 *
 * AST_OWNER:
 *
 *     src/ast/
 *
 * SEMANTIC_OWNER:
 *
 *     semantic contract validation subsystem
 *
 * EFFECT_OWNER:
 *
 *     existing effects subsystem
 *
 * CAPABILITY_OWNER:
 *
 *     existing capability/resource subsystem
 *
 * RESOURCE_OWNER:
 *
 *     existing resource subsystem
 *
 * POLICY_OWNER:
 *
 *     existing policy subsystem
 *
 * PROVENANCE_OWNER:
 *
 *     existing provenance infrastructure
 *
 * IR_OWNER:
 *
 *     canonical semantic representation
 *     and applicable domain IR
 *
 * QUANTUM_IR_OWNER:
 *
 *     quantum::ir
 *
 * TEST_OWNER:
 *
 *     grammar/tests/validation/preconditions/
 *
 * SPEC_OWNER:
 *
 *     grammar/specification/contracts.md
 *     grammar/spec/contracts.md
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     [ ] The grammar is valid ANTLR4 parser grammar.
 *
 *     [ ] The grammar name is `PreconditionsValidation`.
 *
 *     [ ] The canonical standalone contract grammar is imported.
 *
 *     [ ] The canonical function contract grammar is imported.
 *
 *     [ ] `requiresStatement` is not redefined.
 *
 *     [ ] `functionRequiresContract` is not redefined.
 *
 *     [ ] `expression` is not redefined.
 *
 *     [ ] `statementTerminator` is not redefined.
 *
 *     [ ] `functionContractTerminator` is not redefined.
 *
 *     [ ] No lexer rules are defined.
 *
 *     [ ] No keywords are redefined.
 *
 *     [ ] No punctuation is redefined.
 *
 *     [ ] Complete isolated validation requires EOF.
 *
 *     [ ] Standalone `requires(...)` validation works.
 *
 *     [ ] Function-contract `requires(...)` validation works.
 *
 *     [ ] Malformed preconditions are rejected.
 *
 *     [ ] Trailing source is rejected by the isolated validation entry point.
 *
 *     [ ] AST ownership remains downstream.
 *
 *     [ ] Semantic ownership remains downstream.
 *
 *     [ ] Type checking remains downstream.
 *
 *     [ ] Effect checking remains downstream.
 *
 *     [ ] Capability checking remains downstream.
 *
 *     [ ] Resource checking remains downstream.
 *
 *     [ ] Policy checking remains downstream.
 *
 *     [ ] Provenance remains downstream.
 *
 *     [ ] No IR is generated here.
 *
 *     [ ] `quantum::ir` remains the canonical quantum boundary.
 *
 *     [ ] No quantum gate catalogue is introduced.
 *
 *     [ ] No physical resource limit is introduced.
 *
 *     [ ] No target is selected here.
 *
 *     [ ] No runtime execution occurs here.
 *
 *     [ ] No proof engine is introduced here.
 *
 *     [ ] No application-specific language feature is introduced.
 *
 *     [ ] No universal hardware capacity constant exists.
 *
 *     [ ] Positive tests exist.
 *
 *     [ ] Negative tests exist.
 *
 *     [ ] Boundary tests exist.
 *
 *     [ ] Cross-domain tests exist.
 *
 *     [ ] Scalability tests exist.
 *
 *     [ ] Determinism tests exist.
 *
 *     [ ] Portability tests exist.
 *
 *     [ ] Compatibility tests exist.
 *
 *     [ ] Generated Rust parser integrates with Rust 1.97.
 *
 *     [ ] Generated Rust parser integrates with Rust 1.97.1.
 *
 *     [ ] The implementation requires no unsafe Rust.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL GUARANTEE
 * ============================================================================
 *
 * This file adds validation capability without adding another language.
 *
 * The source-level language remains:
 *
 *     requires(condition);
 *
 * The semantic concept remains:
 *
 *     precondition
 *
 * The canonical ownership remains:
 *
 *     standalone:
 *         grammar/statements/contract.g4
 *
 *     function contract:
 *         grammar/functions/contracts.g4
 *
 *     validation facade:
 *         grammar/validation/preconditions.g4
 *
 * This separation prevents grammar duplication while allowing preconditions
 * to participate in the complete Zamani pipeline:
 *
 *     source
 *       ->
 *     lexer
 *       ->
 *     parser
 *       ->
 *     AST
 *       ->
 *     structural validation
 *       ->
 *     semantic validation
 *       ->
 *     types/effects/capabilities/resources/contracts/policies/provenance
 *       ->
 *     canonical semantic representation
 *       ->
 *     classical / quantum::ir / HDL / other domain representations
 *       ->
 *     optimization
 *       ->
 *     lowering
 *       ->
 *     routing / scheduling / resilience where applicable
 *       ->
 *     ZQN / HAL
 *       ->
 *     target realization
 *
 * Therefore preconditions remain:
 *
 *     portable;
 *     target-independent;
 *     domain-neutral;
 *     resource-aware;
 *     capability-aware;
 *     semantically extensible;
 *     deterministic at parse time;
 *     scalable without grammar-level capacity ceilings.
 *
 * ============================================================================
 * GRAMMAR
 * ============================================================================
 */

parser grammar PreconditionsValidation;

import
    ContractStatements,
    FunctionContracts
    ;

/*
 * ============================================================================
 * COMPLETE VALIDATION UNIT
 * ============================================================================
 *
 * Zero or more canonical precondition constructs may be supplied.
 *
 * This supports validation of:
 *
 *     one precondition;
 *
 *     multiple standalone preconditions;
 *
 *     multiple function-contract precondition items.
 *
 * EOF is mandatory.
 *
 * No finite number of preconditions is encoded.
 */
preconditionsValidationUnit
    : preconditionsValidationItem* EOF
    ;


/*
 * ============================================================================
 * VALIDATION ITEM
 * ============================================================================
 *
 * A validation item delegates to an existing canonical grammar owner.
 *
 * No source syntax is reproduced here.
 */
preconditionsValidationItem
    : standalonePreconditionValidation
    | functionPreconditionValidation
    ;


/*
 * ============================================================================
 * STANDALONE PRECONDITION
 * ============================================================================
 *
 * Canonical owner:
 *
 *     grammar/statements/contract.g4
 *         -> ContractStatements
 *         -> requiresStatement
 *
 * This alias provides a stable validation-facing parser context.
 */
standalonePreconditionValidation
    : requiresStatement
    ;


/*
 * ============================================================================
 * FUNCTION PRECONDITION
 * ============================================================================
 *
 * Canonical owner:
 *
 *     grammar/functions/contracts.g4
 *         -> FunctionContracts
 *         -> functionRequiresContract
 *
 * This alias provides a stable validation-facing parser context without
 * reproducing function-contract syntax.
 */
functionPreconditionValidation
    : functionRequiresContract
    ;
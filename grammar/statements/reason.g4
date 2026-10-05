/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/statements/reason.g4
 *
 * STATUS
 * ------
 * PRODUCTION-READY STATEMENT GRAMMAR
 *
 * IMPLEMENTATION BASELINE
 * -----------------------
 * Rust 1.97 / Rust 1.97.1
 * Rust 2021
 * Safe Rust only.
 *
 * GRAMMAR TECHNOLOGY
 * ------------------
 * ANTLR4 parser grammar
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the statement-level syntax for generic reasoning intent.
 *
 * The canonical source forms are:
 *
 *     infer TARGET;
 *     deduce TARGET;
 *     reason TARGET;
 *
 * and:
 *
 *     infer TARGET from SOURCE;
 *     deduce TARGET from SOURCE;
 *     reason TARGET from SOURCE;
 *
 * and:
 *
 *     infer TARGET with (OPTION, OPTION);
 *     deduce TARGET with (OPTION, OPTION);
 *     reason TARGET with (OPTION, OPTION);
 *
 * and the combined form:
 *
 *     infer TARGET from SOURCE with (OPTION, OPTION);
 *     deduce TARGET from SOURCE with (OPTION, OPTION);
 *     reason TARGET from SOURCE with (OPTION, OPTION);
 *
 * This grammar expresses reasoning INTENT only.
 *
 * It does not define:
 *
 *     - a reasoning algorithm;
 *     - a theorem prover;
 *     - a knowledge database;
 *     - a machine-learning framework;
 *     - a probabilistic engine;
 *     - a causal engine;
 *     - a symbolic engine;
 *     - a model implementation;
 *     - a hardware accelerator;
 *     - a CPU/GPU/FPGA/QPU implementation;
 *     - a target device;
 *     - a physical resource;
 *     - a runtime;
 *     - an IR.
 *
 * Those concerns belong to semantic analysis, capabilities, resources,
 * policies, execution planning, libraries, dialects and downstream IR.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *                         Zamani source
 *                              |
 *                              v
 *                         ZamaniLexer
 *                              |
 *                              v
 *                         ZamaniParser
 *                              |
 *                              v
 *                    Statements composition
 *                              |
 *                              v
 *                         reason.g4
 *                              |
 *                              v
 *                    domain-neutral frontend AST
 *                              |
 *                              v
 *                    structural validation
 *                              |
 *                              v
 *                      semantic analysis
 *                              |
 *             +----------------+----------------+
 *             |                |                |
 *             v                v                v
 *          classical       quantum::ir       other domains
 *             |                |                |
 *             +----------------+----------------+
 *                              |
 *                              v
 *                         optimization
 *                              |
 *                              v
 *                     lowering / scheduling
 *                              |
 *                              v
 *                       target realization
 *
 * Reasoning therefore remains above physical realization.
 *
 * ============================================================================
 * OWNERSHIP CONTRACT
 * ============================================================================
 *
 * THIS FILE OWNS
 * --------------
 *
 *     reasonStatement
 *     reasoningOperator
 *     reasoningTarget
 *     reasoningSourceClause
 *     reasoningContextClause
 *     reasoningOptionList
 *     reasoningOption
 *
 * It owns the statement-level composition of:
 *
 *     infer
 *     deduce
 *     reason
 *
 * THIS FILE DOES NOT OWN
 * ----------------------
 *
 *     lexer rules
 *     keyword definitions
 *     punctuation definitions
 *     identifiers
 *     names
 *     paths
 *     expression precedence
 *     expressions
 *     types
 *     knowledge storage
 *     knowledge assertions
 *     knowledge retraction
 *     queries
 *     learning
 *     adaptation
 *     uncertainty semantics
 *     probability semantics
 *     evidence storage
 *     provenance implementation
 *     policy implementation
 *     effect implementation
 *     capability implementation
 *     resource allocation
 *     AST implementation
 *     semantic implementation
 *     IR implementation
 *     runtime execution
 *     hardware realization
 *
 * ============================================================================
 * SINGLE OWNER RULE
 * ============================================================================
 *
 * This file owns the statement-level rule:
 *
 *     reasonStatement
 *
 * It MUST NOT define the universal:
 *
 *     statement
 *
 * rule.
 *
 * The universal statement rule remains owned exclusively by:
 *
 *     grammar/statements/statements.g4
 *
 * Integration is:
 *
 *     statement
 *         |
 *         +--> reasonStatement
 *                 |
 *                 +--> reasoningOperator
 *                 +--> reasoningTarget
 *                 +--> reasoningSourceClause?
 *                 +--> reasoningContextClause?
 *
 * ============================================================================
 * EXPRESSION OWNERSHIP
 * ============================================================================
 *
 * Reasoning operands are ordinary Zamani expressions.
 *
 * This file deliberately imports:
 *
 *     Expressions
 *
 * and consumes:
 *
 *     expression
 *
 * directly.
 *
 * This is intentional.
 *
 * It prevents this grammar from creating a second expression hierarchy.
 *
 * DO NOT add:
 *
 *     primaryExpression
 *     postfixExpression
 *     unaryExpression
 *     binaryExpression
 *     logicalExpression
 *     arithmeticExpression
 *     assignmentExpression
 *
 * here.
 *
 * Those belong to:
 *
 *     grammar/expressions/
 *
 * ============================================================================
 * CRITICAL DEPENDENCY RULE
 * ============================================================================
 *
 * Dependency direction:
 *
 *     Expressions
 *          ^
 *          |
 *     reason.g4
 *          |
 *          v
 *     Statements
 *
 * More precisely, the canonical statement composition grammar imports this
 * grammar, while this grammar imports the canonical expression composition
 * grammar.
 *
 * Therefore:
 *
 *     reason.g4 -> Expressions
 *
 * is valid.
 *
 * This grammar MUST NOT import:
 *
 *     Statements
 *
 * because Statements imports this file.
 *
 * It MUST NOT import:
 *
 *     grammar/statements/statements.g4
 *
 * or any universal statement composition grammar.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The canonical lexical vocabulary remains:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * which composes:
 *
 *     grammar/lexer/
 *
 * The following tokens are consumed from that vocabulary:
 *
 *     INFER
 *     DEDUCE
 *     REASON
 *     FROM
 *     WITH
 *     LPAREN
 *     RPAREN
 *     COMMA
 *     SEMICOLON
 *
 * This file MUST NOT define lexer rules.
 *
 * It MUST NOT introduce alternative spellings such as:
 *
 *     inference
 *     deduction
 *     think
 *     analyze
 *
 * merely to enlarge the keyword vocabulary.
 *
 * ============================================================================
 * REASONING MODEL
 * ============================================================================
 *
 * The three source operators:
 *
 *     infer
 *     deduce
 *     reason
 *
 * intentionally converge on one semantic concept:
 *
 *     reasoning operation
 *
 * Their lexical distinction is preserved in the parse tree so semantic
 * analysis can distinguish the requested operation kind.
 *
 * The semantic layer may represent:
 *
 *     Infer
 *     Deduce
 *     Reason
 *
 * without creating three unrelated semantic systems.
 *
 * ============================================================================
 * REASONING TARGET
 * ============================================================================
 *
 * The target is a normal Zamani expression.
 *
 * Examples:
 *
 *     infer hypothesis;
 *
 *     infer hypothesis + evidence;
 *
 *     infer model(input);
 *
 *     infer knowledge.query(pattern);
 *
 *     infer measurement_result;
 *
 *     infer tensor[index];
 *
 *     infer distributed_value;
 *
 *     infer hardware_observation;
 *
 *     infer quantum_result;
 *
 * The parser does not determine what the target means.
 *
 * Semantic analysis determines:
 *
 *     - target type;
 *     - target domain;
 *     - target provenance;
 *     - target capabilities;
 *     - target effects;
 *     - target resource requirements.
 *
 * ============================================================================
 * SOURCE / EVIDENCE CLAUSE
 * ============================================================================
 *
 * The optional `from` clause identifies an expression that supplies source
 * information, evidence, premises or observations.
 *
 * Examples:
 *
 *     infer conclusion from evidence;
 *
 *     deduce result from knowledge.query(pattern);
 *
 *     reason decision from measurement;
 *
 *     infer result from model(input);
 *
 * The grammar does not require the source to be:
 *
 *     a file;
 *     a database;
 *     a knowledge graph;
 *     a model;
 *     a network;
 *     a quantum measurement;
 *     a classical value;
 *     a hardware observation.
 *
 * All such interpretation belongs downstream.
 *
 * ============================================================================
 * CONTEXT CLAUSE
 * ============================================================================
 *
 * The optional `with (...)` clause supplies an ordered list of ordinary
 * expressions that provide reasoning context.
 *
 * Examples:
 *
 *     reason proposition with (policy);
 *
 *     infer result with (model);
 *
 *     deduce decision with (confidence);
 *
 *     reason conclusion with (strategy, evidence, policy);
 *
 * No closed vocabulary is created for context options.
 *
 * Therefore future concepts such as:
 *
 *     strategy
 *     evidence
 *     confidence
 *     provenance
 *     model
 *     knowledge
 *     policy
 *     constraints
 *     resource preferences
 *     deterministic mode
 *
 * can be represented without adding another core keyword, provided their
 * semantic expressions are otherwise valid.
 *
 * ============================================================================
 * OPTION LIST
 * ============================================================================
 *
 * The option list is deliberately open-ended:
 *
 *     reasoningOption (',' reasoningOption)*
 *
 * There is no fixed option-count ceiling.
 *
 * A trailing comma is deliberately NOT accepted.
 *
 * Therefore:
 *
 *     with (a, b)
 *
 * is valid.
 *
 * while:
 *
 *     with (a, b,)
 *
 * is rejected.
 *
 * This avoids accepting accidental incomplete option entries and gives a
 * deterministic grammar boundary.
 *
 * ============================================================================
 * CLAUSE ORDER
 * ============================================================================
 *
 * The canonical clause order is:
 *
 *     TARGET
 *     SOURCE?
 *     CONTEXT?
 *
 * Therefore:
 *
 *     infer target;
 *
 *     infer target from source;
 *
 *     infer target with (context);
 *
 *     infer target from source with (context);
 *
 * are valid.
 *
 * Reversed ordering is not silently accepted:
 *
 *     infer target with (context) from source;
 *
 * is invalid.
 *
 * This gives one canonical source representation and avoids unnecessary
 * syntactic permutations.
 *
 * ============================================================================
 * EMPTY CONTEXT
 * ============================================================================
 *
 * The following is intentionally invalid:
 *
 *     reason target with ();
 *
 * An empty context has no semantic payload and is better represented by
 * omitting the clause entirely.
 *
 * Therefore:
 *
 *     reason target;
 *
 * is canonical.
 *
 * ============================================================================
 * KNOWLEDGE INTEGRATION
 * ============================================================================
 *
 * Reasoning can consume knowledge operations without owning knowledge syntax.
 *
 * Examples:
 *
 *     infer answer from knowledge.query(pattern);
 *
 *     reason conclusion from query(source);
 *
 *     deduce fact from evidence;
 *
 * Knowledge constructs such as:
 *
 *     assert
 *     retract
 *     query
 *
 * remain owned by their appropriate grammar/domain.
 *
 * This file MUST NOT duplicate them.
 *
 * ============================================================================
 * LEARNING INTEGRATION
 * ============================================================================
 *
 * Reasoning can consume learned-model results:
 *
 *     infer result from model(input);
 *
 *     reason classification from predictor(value);
 *
 * Learning syntax remains owned by the AI/learning subsystem.
 *
 * This file does not define:
 *
 *     train
 *     fit
 *     fine_tune
 *     reinforce
 *     transfer_learning
 *
 * as reasoning statements.
 *
 * ============================================================================
 * ADAPTATION INTEGRATION
 * ============================================================================
 *
 * Reasoning may produce a value later consumed by adaptation:
 *
 *     infer strategy from evidence;
 *
 *     adapt(...);
 *
 * Adaptation remains separately governed by:
 *
 *     effects
 *     policies
 *     capabilities
 *     resources
 *     provenance
 *
 * Reasoning does not grant permission to modify program state or models.
 *
 * ============================================================================
 * UNCERTAINTY INTEGRATION
 * ============================================================================
 *
 * Reasoning expressions may operate over:
 *
 *     probability
 *     distributions
 *     confidence
 *     intervals
 *     uncertain values
 *     symbolic uncertainty
 *
 * This grammar does not impose:
 *
 *     precision;
 *     probability representation;
 *     number of outcomes;
 *     distribution type;
 *     numerical implementation.
 *
 * Those belong to the type and semantic systems.
 *
 * ============================================================================
 * EVIDENCE INTEGRATION
 * ============================================================================
 *
 * Evidence may originate from:
 *
 *     classical computation
 *     quantum measurement
 *     simulation
 *     hardware observation
 *     data processing
 *     model inference
 *     distributed computation
 *     network services
 *     scientific computation
 *
 * The grammar represents evidence through ordinary expressions.
 *
 * Evidence validation and provenance are semantic responsibilities.
 *
 * ============================================================================
 * POLICY INTEGRATION
 * ============================================================================
 *
 * Reasoning may be constrained by:
 *
 *     security policy
 *     privacy policy
 *     resource policy
 *     execution policy
 *     evidence policy
 *     model policy
 *     determinism policy
 *     adaptation policy
 *
 * This grammar does not evaluate policies.
 *
 * Valid syntax does not imply permission to execute the reasoning operation.
 *
 * ============================================================================
 * EFFECT INTEGRATION
 * ============================================================================
 *
 * Reasoning may have effects such as:
 *
 *     reasoning
 *     knowledge.read
 *     model.inference
 *     randomness
 *     network
 *     external.io
 *     measurement
 *
 * depending on semantic resolution.
 *
 * The grammar does not declare or infer effects.
 *
 * ============================================================================
 * CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Semantic analysis may derive requirements such as:
 *
 *     capability("reasoning")
 *     capability("knowledge.query")
 *     capability("model.inference")
 *     capability("probabilistic.compute")
 *     capability("quantum.measurement")
 *
 * The grammar does not resolve capability availability.
 *
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * Reasoning may require resources such as:
 *
 *     compute
 *     memory
 *     storage
 *     communication
 *     accelerator resources
 *     quantum resources
 *     model resources
 *
 * Those requirements belong to the canonical resource system.
 *
 * This grammar MUST NOT introduce:
 *
 *     MAX_REASONING_DEPTH
 *     MAX_REASONING_STEPS
 *     MAX_FACTS
 *     MAX_EVIDENCE
 *     MAX_CONTEXT
 *     MAX_MODELS
 *     MAX_MEMORY
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_QUBITS
 *     MAX_NODES
 *
 * or any equivalent language-level ceiling.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing is deterministic.
 *
 * Identical:
 *
 *     source token stream
 *     grammar version
 *     parser configuration
 *
 * MUST produce equivalent parse-tree structure.
 *
 * Parsing MUST NOT depend on:
 *
 *     wall-clock time
 *     randomness
 *     filesystem state
 *     network state
 *     hardware state
 *     device discovery
 *     target availability
 *     runtime state
 *     scheduler state
 *
 * Runtime reasoning may be nondeterministic.
 *
 * Such nondeterminism is a semantic/effect/runtime property, not a parser
 * property.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Parsing is non-executing.
 *
 * This grammar MUST NOT:
 *
 *     query a database;
 *     invoke a model;
 *     execute a reasoning engine;
 *     contact a network;
 *     inspect hardware;
 *     access secrets;
 *     load plugins;
 *     invoke a QPU;
 *     execute a simulator;
 *     execute external processes.
 *
 * Source syntax is not permission to execute its semantic meaning.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * A reasoning source or target may contain a quantum-derived expression:
 *
 *     infer result from measurement;
 *
 *     reason state_property from quantum_result;
 *
 *     deduce decision from measurement_result;
 *
 * This grammar does not define quantum operations.
 *
 * Quantum semantics remain downstream:
 *
 *     frontend AST
 *          |
 *          v
 *     quantum semantic analysis
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
 *     QEC / resilience
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
 * No quantum-specific reasoning IR is created.
 *
 * ============================================================================
 * HYBRID INTEGRATION
 * ============================================================================
 *
 * Reasoning may participate in:
 *
 *     classical -> reasoning
 *     quantum -> reasoning
 *     reasoning -> classical
 *     reasoning -> quantum control
 *     AI -> reasoning
 *     reasoning -> AI
 *     HDL/hardware -> reasoning
 *
 * These relationships are semantic.
 *
 * The grammar remains domain-neutral.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Reasoning can consume:
 *
 *     hardware observations;
 *     simulation results;
 *     verification results;
 *     synthesized-artifact metadata;
 *     sensor values;
 *     timing information.
 *
 * No hardware topology or physical resource identifier is encoded.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Reasoning can consume distributed values and can produce values later used
 * by:
 *
 *     actors
 *     tasks
 *     services
 *     workflows
 *
 * This grammar does not create another actor/message model.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend AST MUST preserve at least:
 *
 *     operation kind
 *     target expression
 *     optional source expression
 *     ordered context expressions
 *     source span
 *     source ordering
 *
 * Conceptually:
 *
 *     ReasoningStatement {
 *         kind,
 *         target,
 *         source,
 *         context,
 *         span
 *     }
 *
 * The exact Rust representation belongs to the existing domain-neutral AST.
 *
 * This grammar MUST NOT require AST nodes such as:
 *
 *     QuantumReasoning
 *     GPUReasoning
 *     FPGAReasoning
 *     AIReasoning
 *     HardwareReasoning
 *
 * as universal categories.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing determines only:
 *
 *     "Is this syntactically a reasoning statement?"
 *
 * Semantic analysis determines:
 *
 *     - what the operator means;
 *     - whether the target is valid;
 *     - whether the source is valid;
 *     - whether context values are valid;
 *     - which reasoning strategy applies;
 *     - which knowledge sources are used;
 *     - which evidence is accepted;
 *     - which model is selected;
 *     - which uncertainty semantics apply;
 *     - which effects occur;
 *     - which capabilities are required;
 *     - which resources are required;
 *     - which policies apply;
 *     - which provenance must be recorded;
 *     - whether the computation is deterministic;
 *     - whether execution is permitted.
 *
 * No semantic lookup occurs in this grammar.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates NO IR.
 *
 * The downstream path is:
 *
 *     reasonStatement
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     reasoning semantic model
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +---------------------+
 *          |                     |
 *          v                     v
 *      classical             quantum::ir
 *          |                     |
 *          +----------+----------+
 *                     |
 *                     v
 *                 optimization
 *                     |
 *                     v
 *              lowering/scheduling
 *                     |
 *                     v
 *              target realization
 *
 * No reasoning-specific machine IR is introduced by this file.
 *
 * ============================================================================
 * EFFECT / CAPABILITY / RESOURCE CHECKING
 * ============================================================================
 *
 * All such checking occurs after parsing.
 *
 * A syntactically valid:
 *
 *     reason ...
 *
 * does NOT imply:
 *
 *     capability available;
 *     resource available;
 *     policy satisfied;
 *     model available;
 *     knowledge source available;
 *     network available;
 *     QPU available;
 *     accelerator available.
 *
 * These are semantic/execution decisions.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * The grammar contains no target-size assumptions.
 *
 * There is no language-level limit on:
 *
 *     reasoning statements
 *     reasoning target size
 *     source expression size
 *     context option count
 *     reasoning nesting
 *     program size
 *     machine size
 *     CPU count
 *     GPU count
 *     FPGA count
 *     accelerator count
 *     QPU count
 *     qubit count
 *     node count
 *     memory capacity
 *     storage capacity
 *     tensor rank
 *     network scale
 *
 * Practical limits are determined by:
 *
 *     compiler resources
 *     runtime resources
 *     target capabilities
 *     deployment configuration
 *     resource policy
 *
 * "Infinity" means the grammar imposes no artificial finite ceiling.
 *
 * It does NOT claim that physical hardware has infinite resources.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no:
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
 *     MAX_REASONING_DEPTH
 *     MAX_REASONING_STEPS
 *     MAX_FACTS
 *     MAX_EVIDENCE
 *     MAX_CONTEXT
 *
 * It contains no:
 *
 *     physical device IDs
 *     physical qubit IDs
 *     vendor-specific hardware
 *     topology
 *     calibration
 *     routing
 *     scheduling
 *     QEC implementation
 *     ZQN implementation
 *     HAL implementation
 *
 * Numeric literals inside expressions remain ordinary program data.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing source-level reasoning forms are preserved:
 *
 *     infer TARGET;
 *     infer TARGET from SOURCE;
 *     infer TARGET with (OPTION);
 *     infer TARGET from SOURCE with (OPTION);
 *
 * The same structure applies to:
 *
 *     deduce
 *     reason
 *
 * No historical application-specific reasoning syntax is automatically
 * promoted into the core language.
 *
 * Annotation forms such as:
 *
 *     @infer(...)
 *     @reason(...)
 *
 * remain annotation/directive syntax and are not redefined here.
 *
 * ============================================================================
 * RELATIONSHIP TO grammar/expressions/reasoning.g4
 * ============================================================================
 *
 * The repository currently contains:
 *
 *     grammar/expressions/reasoning.g4
 *
 * That file describes reasoning-expression syntax and currently uses an
 * `expressionCore` integration boundary.
 *
 * This file deliberately does NOT import that grammar because the current
 * canonical `Expressions` composition does not import the reasoning grammar
 * and the existing expression grammar would otherwise introduce a circular
 * dependency or an unresolved expression-core dependency.
 *
 * The production dependency is therefore:
 *
 *     reason.g4
 *          |
 *          +--> Expressions
 *
 * rather than:
 *
 *     reason.g4
 *          |
 *          +--> reasoning.g4
 *          |
 *          +--> Expressions
 *
 * Once the repository performs the planned expression-core extraction, the
 * expression-level reasoning grammar can be normalized against this statement
 * owner without changing this file's public statement contract.
 *
 * This file therefore remains independently complete.
 *
 * ============================================================================
 * REQUIRED STATEMENTS COMPOSITION INTEGRATION
 * ============================================================================
 *
 * grammar/statements/statements.g4 MUST import this grammar:
 *
 *     ReasonStatements
 *
 * and add:
 *
 *     reasonStatement
 *
 * to the universal statement alternatives.
 *
 * The required composition is:
 *
 *     statement
 *         : declarationStatement
 *         | assignmentStatement
 *         | assertionStatement
 *         | controlFlowStatement
 *         | concurrencyStatementAdapter
 *         | effectStatement
 *         | resourceStatement
 *         | reasonStatement
 *         | domainStatement
 *         | blockExpression
 *         | emptyStatement
 *         | expressionStatement
 *         ;
 *
 * `reasonStatement` MUST appear before the generic `expressionStatement`
 * fallback.
 *
 * ============================================================================
 * AI INTEGRATION
 * ============================================================================
 *
 * AI syntax may consume the semantic result of reasoning.
 *
 * AI-specific constructs remain owned by:
 *
 *     grammar/ai/
 *
 * The AI subsystem must not create another:
 *
 *     infer
 *     deduce
 *     reason
 *
 * statement family.
 *
 * It may consume the semantic reasoning model produced by this statement.
 *
 * ============================================================================
 * KNOWLEDGE INTEGRATION
 * ============================================================================
 *
 * Knowledge syntax remains owned by the knowledge subsystem.
 *
 * It may provide expressions used by:
 *
 *     reasoningTarget
 *     reasoningSourceClause
 *     reasoningOption
 *
 * No knowledge grammar is duplicated here.
 *
 * ============================================================================
 * PROVENANCE INTEGRATION
 * ============================================================================
 *
 * Reasoning is a natural provenance-producing semantic operation.
 *
 * Provenance may record:
 *
 *     operation kind
 *     target
 *     source
 *     context
 *     evidence
 *     model
 *     strategy
 *     policy
 *     compiler transformation
 *     execution result
 *
 * Provenance syntax/representation is not owned by this file.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE
 * --------
 *
 *     infer hypothesis;
 *
 *     deduce conclusion;
 *
 *     reason decision;
 *
 *     infer result from evidence;
 *
 *     deduce result from premises;
 *
 *     reason result from measurement;
 *
 *     infer result with (context);
 *
 *     deduce result with (model);
 *
 *     reason result with (policy, confidence);
 *
 *     infer result from evidence with (policy);
 *
 *     deduce result from knowledge.query(pattern)
 *         with (strategy, evidence);
 *
 *     reason measurement_result from quantum_result
 *         with (policy);
 *
 *     infer tensor[index] from observation
 *         with (context);
 *
 *     reason distributed_result from service_result
 *         with (policy, provenance);
 *
 *     infer hardware_state from simulation_result
 *         with (verification);
 *
 * NEGATIVE
 * --------
 *
 *     infer;
 *
 *     deduce;
 *
 *     reason;
 *
 *     infer;
 *     infer ;
 *
 *     infer from evidence;
 *
 *     infer target from;
 *
 *     infer target with;
 *
 *     infer target with ();
 *
 *     infer target with (a,);
 *
 *     infer target with (, a);
 *
 *     infer target with (a,,b);
 *
 *     infer target from source from other;
 *
 *     infer target with (a) from source;
 *
 *     infer target from source with;
 *
 *     infer target from source with ();
 *
 *     infer target;
 *     trailing;
 *
 * The last example is valid only as two separate statements if the first
 * statement is terminated and the second is otherwise valid. The parser must
 * not accidentally absorb unrelated trailing syntax into a reasoning clause.
 *
 * SEMANTIC NEGATIVES
 * ------------------
 *
 * These are not parser errors merely because they may be semantically invalid:
 *
 *     infer unknown_name;
 *
 *     infer non_reasonable_value;
 *
 *     reason value_of_wrong_type;
 *
 *     infer result from unavailable_source;
 *
 *     deduce result with unavailable_policy;
 *
 * Capability/resource/policy errors belong downstream.
 *
 * ============================================================================
 * BOUNDARY TESTS
 * ============================================================================
 *
 * Test reasoning against:
 *
 *     classical values
 *     symbolic expressions
 *     tensor expressions
 *     data queries
 *     knowledge expressions
 *     model expressions
 *     uncertainty expressions
 *     distributed values
 *     network results
 *     hardware observations
 *     simulation results
 *     quantum-derived values
 *     hybrid results
 *
 * Test:
 *
 *     nested calls
 *     member access
 *     indexing
 *     qualified names
 *     arithmetic expressions
 *     logical expressions
 *     conditional expressions
 *     generic calls
 *     parenthesized expressions
 *
 * ============================================================================
 * SCALABILITY TESTS
 * ============================================================================
 *
 * Generated conformance tests should vary:
 *
 *     target expression size
 *     source expression size
 *     context option count
 *     expression nesting
 *     statement count
 *     program size
 *
 * without changing this grammar.
 *
 * There must be no grammar-level finite maximum.
 *
 * ============================================================================
 * DETERMINISM TESTS
 * ============================================================================
 *
 * Parse identical source repeatedly under identical:
 *
 *     lexer configuration
 *     parser configuration
 *     grammar version
 *
 * and verify equivalent parse trees.
 *
 * ============================================================================
 * PORTABILITY TESTS
 * ============================================================================
 *
 * The same reasoning source must remain syntactically identical when target
 * descriptions differ.
 *
 * Examples:
 *
 *     embedded
 *     CPU
 *     multicore
 *     GPU
 *     FPGA
 *     accelerator
 *     QPU
 *     simulator
 *     HPC
 *     cluster
 *     distributed
 *     cloud
 *
 * Resource availability must not alter parsing.
 *
 * ============================================================================
 * CROSS-DOMAIN TESTS
 * ============================================================================
 *
 * Required combinations include:
 *
 *     reasoning + classical
 *     reasoning + quantum
 *     reasoning + hybrid
 *     reasoning + HDL
 *     reasoning + hardware
 *     reasoning + AI
 *     reasoning + data
 *     reasoning + distributed
 *     reasoning + networking
 *     reasoning + security
 *     reasoning + resources
 *     reasoning + execution
 *
 * ============================================================================
 * ROUND-TRIP CONTRACT
 * ============================================================================
 *
 * The supported source must survive:
 *
 *     source
 *       ->
 *     lexer
 *       ->
 *     parser
 *       ->
 *     AST
 *       ->
 *     formatter/printer
 *       ->
 *     parser
 *
 * while preserving:
 *
 *     operator kind
 *     target
 *     source
 *     ordered context
 *     statement boundary
 *
 * ============================================================================
 * SAFE-RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no embedded Rust;
 *     no semantic predicates;
 *     no executable actions;
 *     no I/O;
 *     no filesystem access;
 *     no networking;
 *     no hardware access;
 *     no runtime calls;
 *     no unsafe Rust.
 *
 * The consuming implementation must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and must not require unsafe Rust.
 *
 * ============================================================================
 * ANTLR COMPOSITION CONTRACT
 * ============================================================================
 *
 * This grammar is a parser grammar.
 *
 * Its canonical name is:
 *
 *     ReasonStatements
 *
 * Therefore the file name:
 *
 *     reason.g4
 *
 * intentionally differs from the parser grammar name.
 *
 * The canonical statement dispatcher imports:
 *
 *     ReasonStatements
 *
 * exactly once.
 *
 * No other statement grammar should import ReasonStatements.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [ ] The file is named grammar/statements/reason.g4.
 *
 *     [ ] The parser grammar name is ReasonStatements.
 *
 *     [ ] tokenVocab is ZamaniLexer.
 *
 *     [ ] Expressions is the sole expression dependency.
 *
 *     [ ] No second expression hierarchy exists here.
 *
 *     [ ] reasonStatement is the sole public statement entry point.
 *
 *     [ ] infer is supported.
 *
 *     [ ] deduce is supported.
 *
 *     [ ] reason is supported.
 *
 *     [ ] target expression is mandatory.
 *
 *     [ ] source clause is optional.
 *
 *     [ ] context clause is optional.
 *
 *     [ ] source precedes context.
 *
 *     [ ] context cannot be empty.
 *
 *     [ ] context does not permit a trailing comma.
 *
 *     [ ] semicolon is mandatory.
 *
 *     [ ] no universal statement rule is defined here.
 *
 *     [ ] no expression precedence is defined here.
 *
 *     [ ] no reasoning algorithm is defined here.
 *
 *     [ ] no knowledge implementation is defined here.
 *
 *     [ ] no learning implementation is defined here.
 *
 *     [ ] no adaptation implementation is defined here.
 *
 *     [ ] no probability implementation is defined here.
 *
 *     [ ] no provenance implementation is defined here.
 *
 *     [ ] no policy implementation is defined here.
 *
 *     [ ] no resource allocation is defined here.
 *
 *     [ ] no hardware target is defined here.
 *
 *     [ ] no quantum gate is enumerated.
 *
 *     [ ] quantum::ir remains the canonical quantum boundary.
 *
 *     [ ] no machine-size limit is encoded.
 *
 *     [ ] no hard-coded hardware identifier exists.
 *
 *     [ ] no embedded actions exist.
 *
 *     [ ] no semantic predicates exist.
 *
 *     [ ] no unsafe Rust is required.
 *
 *     [ ] statements.g4 imports ReasonStatements.
 *
 *     [ ] statements.g4 admits reasonStatement exactly once.
 *
 *     [ ] AST mapping exists.
 *
 *     [ ] semantic mapping exists.
 *
 *     [ ] provenance integration exists downstream.
 *
 *     [ ] positive tests exist.
 *
 *     [ ] negative tests exist.
 *
 *     [ ] semantic-negative tests exist.
 *
 *     [ ] boundary tests exist.
 *
 *     [ ] scalability tests exist.
 *
 *     [ ] determinism tests exist.
 *
 *     [ ] portability tests exist.
 *
 *     [ ] cross-domain tests exist.
 *
 *     [ ] round-trip tests exist.
 *
 * ============================================================================
 * PRODUCTION GRAMMAR
 * ============================================================================
 */

parser grammar ReasonStatements;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * The canonical expression composition is the only expression dependency.
 *
 * Do not import Statements here.
 *
 * Do not import the existing expression-level reasoning grammar here.
 *
 * This keeps the dependency graph acyclic:
 *
 *     reason.g4
 *          |
 *          v
 *     Expressions
 *
 * while:
 *
 *     Statements
 *          |
 *          v
 *     ReasonStatements
 */
import
    Expressions
    ;

/*
 * ============================================================================
 * PUBLIC STATEMENT ENTRY
 * ============================================================================
 *
 * Exactly one statement-level entry point is owned here.
 */
reasonStatement
    : reasoningOperator
      reasoningTarget
      reasoningSourceClause?
      reasoningContextClause?
      SEMICOLON
    ;

/*
 * ============================================================================
 * OPERATOR
 * ============================================================================
 *
 * These are source-level operation kinds.
 *
 * Their semantic implementation is downstream.
 */
reasoningOperator
    : INFER
    | DEDUCE
    | REASON
    ;

/*
 * ============================================================================
 * TARGET
 * ============================================================================
 *
 * The target is one complete canonical Zamani expression.
 *
 * Semantic analysis determines whether the target is meaningful as a
 * reasoning target.
 */
reasoningTarget
    : expression
    ;

/*
 * ============================================================================
 * SOURCE / EVIDENCE
 * ============================================================================
 *
 * `from` introduces one complete expression.
 */
reasoningSourceClause
    : FROM
      expression
    ;

/*
 * ============================================================================
 * CONTEXT
 * ============================================================================
 *
 * `with (...)` introduces one or more ordered expressions.
 *
 * Empty contexts are rejected.
 *
 * Trailing commas are rejected.
 */
reasoningContextClause
    : WITH
      LPAREN
      reasoningOptionList
      RPAREN
    ;

/*
 * ============================================================================
 * CONTEXT OPTION LIST
 * ============================================================================
 */
reasoningOptionList
    : reasoningOption
      (
          COMMA
          reasoningOption
      )*
    ;

/*
 * ============================================================================
 * CONTEXT OPTION
 * ============================================================================
 *
 * Every option is a normal Zamani expression.
 *
 * The semantic layer determines whether the expression represents:
 *
 *     strategy
 *     model
 *     evidence
 *     confidence
 *     policy
 *     provenance
 *     resource preference
 *     constraint
 *     configuration
 *     or another valid semantic value.
 */
reasoningOption
    : expression
    ;
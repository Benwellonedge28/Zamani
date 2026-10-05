/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/statements/explain.g4
 *
 * GRAMMAR NAME
 * ------------
 * Explain
 *
 * STATUS
 * ------
 * CANONICAL EXPLANATION STATEMENT ADAPTER
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
 * This file owns the statement-level syntax for:
 *
 *     explain
 *
 * An explanation statement requests a structured explanation of an
 * expression, computation, result, decision, transformation, derivation,
 * observation, artifact, or other semantic subject.
 *
 * Canonical forms are:
 *
 *     explain TARGET;
 *
 *     explain TARGET from SOURCE;
 *
 *     explain TARGET with (CONTEXT);
 *
 *     explain TARGET from SOURCE with (CONTEXT, CONTEXT);
 *
 * The grammar expresses explanation INTENT.
 *
 * It does NOT implement:
 *
 *     explanation generation;
 *     natural-language generation;
 *     proof generation;
 *     theorem proving;
 *     model introspection;
 *     provenance storage;
 *     provenance verification;
 *     compiler diagnostics;
 *     compiler optimization;
 *     quantum analysis;
 *     hardware analysis;
 *     resource discovery;
 *     capability discovery;
 *     runtime inspection;
 *     target selection;
 *     backend execution;
 *     logging;
 *     auditing;
 *     visualization.
 *
 * Those responsibilities belong to downstream semantic, provenance,
 * diagnostics, policy, execution, tooling, library, and domain subsystems.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *                         Zamani source
 *                              |
 *                              v
 *                       canonical lexer
 *                              |
 *                              v
 *                       canonical parser
 *                              |
 *                              v
 *                    statement composition
 *                              |
 *                              v
 *                       explainStatement
 *                              |
 *                              v
 *                    domain-neutral frontend AST
 *                              |
 *                              v
 *                    structural validation
 *                              |
 *             +----------------+----------------+
 *             |                |                |
 *             v                v                v
 *        semantic        provenance        policy/effect
 *        analysis         analysis          analysis
 *             |                |                |
 *             +----------------+----------------+
 *                              |
 *                              v
 *                    canonical semantic model
 *                              |
 *             +----------------+----------------+
 *             |                |                |
 *             v                v                v
 *         classical       quantum::ir       HDL/hardware
 *             |                |                |
 *             +----------------+----------------+
 *                              |
 *                              v
 *                  optimization / lowering
 *                              |
 *                              v
 *                  routing / scheduling
 *                              |
 *                              v
 *                    target realization
 *
 * Explanation remains a source-level semantic request.
 *
 * ============================================================================
 * OWNERSHIP CONTRACT
 * ============================================================================
 *
 * THIS FILE OWNS
 * --------------
 *
 *     explainStatement
 *     explanationTarget
 *     explanationSourceClause
 *     explanationContextClause
 *     explanationContextList
 *     explanationContext
 *
 * This file owns only the syntactic composition of the explanation statement.
 *
 * THIS FILE DOES NOT OWN
 * ----------------------
 *
 *     statement
 *     statements
 *     expressions
 *     expression precedence
 *     identifiers
 *     names
 *     qualified names
 *     literals
 *     types
 *     declarations
 *     assignments
 *     assertions
 *     contracts
 *     policies
 *     effects
 *     capabilities
 *     resources
 *     provenance storage
 *     evidence storage
 *     reasoning algorithms
 *     learning algorithms
 *     adaptation algorithms
 *     AI models
 *     quantum operations
 *     quantum topology
 *     quantum routing
 *     quantum scheduling
 *     QEC
 *     ZQN
 *     HDL implementation
 *     hardware implementation
 *     physical device selection
 *     runtime execution
 *     compiler implementation
 *     backend implementation
 *
 * ============================================================================
 * SINGLE OWNER RULE
 * ============================================================================
 *
 * There MUST be exactly one universal statement rule.
 *
 * That rule is owned by:
 *
 *     grammar/statements/statements.g4
 *
 * This file MUST NOT define:
 *
 *     statement
 *
 * or:
 *
 *     statements
 *
 * Integration is:
 *
 *     statement
 *         |
 *         +--> explainStatement
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The canonical lexical authority is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * which delegates to:
 *
 *     grammar/lexer/
 *
 * This file consumes the existing:
 *
 *     EXPLAIN
 *
 * token.
 *
 * This file MUST NOT define:
 *
 *     EXPLAIN
 *
 * or any other lexer token.
 *
 * The repository already has:
 *
 *     EXPLAIN : 'explain'
 *
 * in the canonical keyword vocabulary.
 *
 * Therefore NO lexer modification is required merely to create this
 * statement grammar.
 *
 * Do not introduce aliases such as:
 *
 *     describe
 *     clarify
 *     tell
 *     why
 *     explain_this
 *
 * merely to enlarge the vocabulary.
 *
 * ============================================================================
 * EXPRESSION CONTRACT
 * ============================================================================
 *
 * Explanation subjects, sources and context values are ordinary Zamani
 * expressions.
 *
 * This deliberately permits explanation of arbitrary semantic values without
 * creating a new grammar for every domain.
 *
 * Examples include:
 *
 *     explain result;
 *
 *     explain decision;
 *
 *     explain quantum_result;
 *
 *     explain hardware_state;
 *
 *     explain simulation_result;
 *
 *     explain model(input);
 *
 *     explain tensor[index];
 *
 *     explain compiler_result;
 *
 *     explain resource_plan;
 *
 *     explain routing_decision;
 *
 *     explain execution_plan;
 *
 *     explain provenance(record);
 *
 * The expression grammar remains the sole owner of expression syntax.
 *
 * This file MUST NOT recreate:
 *
 *     primaryExpression
 *     unaryExpression
 *     binaryExpression
 *     logicalExpression
 *     arithmeticExpression
 *     postfixExpression
 *     callExpression
 *     memberExpression
 *     assignmentExpression
 *
 * ============================================================================
 * EXPLANATION TARGET
 * ============================================================================
 *
 * The target is mandatory.
 *
 * Therefore:
 *
 *     explain result;
 *
 * is valid.
 *
 * while:
 *
 *     explain;
 *
 * is invalid.
 *
 * The parser does not determine whether the target is explainable.
 *
 * Semantic analysis determines:
 *
 *     whether the target exists;
 *     whether explanation is permitted;
 *     what evidence is available;
 *     what provenance is available;
 *     what policy applies;
 *     what effects are involved;
 *     what capabilities are required;
 *     what explanation mechanisms are available.
 *
 * ============================================================================
 * SOURCE / EVIDENCE CLAUSE
 * ============================================================================
 *
 * The optional `from` clause identifies an expression supplying information
 * relevant to the requested explanation.
 *
 * Examples:
 *
 *     explain decision from evidence;
 *
 *     explain result from provenance_record;
 *
 *     explain routing_decision from compilation_trace;
 *
 *     explain model(input) from training_record;
 *
 *     explain quantum_result from measurement_record;
 *
 *     explain hardware_state from simulation_result;
 *
 * The source is intentionally an ordinary expression.
 *
 * It may represent:
 *
 *     evidence;
 *     provenance;
 *     observations;
 *     measurements;
 *     compiler records;
 *     model information;
 *     simulation results;
 *     execution records;
 *     resource information;
 *     policy information;
 *     data;
 *     distributed results;
 *     future semantic artifacts.
 *
 * The grammar does not assign domain-specific meaning to `from`.
 *
 * ============================================================================
 * CONTEXT CLAUSE
 * ============================================================================
 *
 * The optional `with (...)` clause provides an ordered collection of ordinary
 * expressions that qualify the requested explanation.
 *
 * Examples:
 *
 *     explain result with (provenance);
 *
 *     explain decision with (evidence);
 *
 *     explain result with (provenance, policy);
 *
 *     explain routing_decision from compilation_trace
 *         with (optimization, constraints);
 *
 * Context values may semantically represent:
 *
 *     evidence;
 *     provenance;
 *     policy;
 *     confidence;
 *     model;
 *     explanation strategy;
 *     detail preference;
 *     constraints;
 *     requirements;
 *     resource information;
 *     capability information;
 *     reproducibility information;
 *     deterministic-execution information;
 *     domain-specific explanation context.
 *
 * The grammar does not reserve a keyword for each of these concepts.
 *
 * ============================================================================
 * CLAUSE ORDER
 * ============================================================================
 *
 * Canonical order is:
 *
 *     TARGET
 *     SOURCE?
 *     CONTEXT?
 *
 * Therefore:
 *
 *     explain target;
 *
 *     explain target from source;
 *
 *     explain target with (context);
 *
 *     explain target from source with (context);
 *
 * are valid.
 *
 * The following is intentionally invalid:
 *
 *     explain target with (context) from source;
 *
 * This gives the language one canonical representation rather than accepting
 * multiple syntactic permutations for the same semantic operation.
 *
 * ============================================================================
 * EMPTY CONTEXT
 * ============================================================================
 *
 * This is invalid:
 *
 *     explain target with ();
 *
 * An empty context contains no semantic information.
 *
 * The canonical form is simply:
 *
 *     explain target;
 *
 * ============================================================================
 * TRAILING COMMA
 * ============================================================================
 *
 * A trailing comma is not accepted.
 *
 * Valid:
 *
 *     explain target with (evidence, provenance);
 *
 * Invalid:
 *
 *     explain target with (evidence, provenance,);
 *
 * This prevents incomplete context entries and provides deterministic syntax.
 *
 * ============================================================================
 * OPEN-WORLD EXPLANATION MODEL
 * ============================================================================
 *
 * This grammar deliberately does not enumerate explanation mechanisms.
 *
 * Therefore semantic implementations may provide:
 *
 *     symbolic explanations;
 *     causal explanations;
 *     probabilistic explanations;
 *     statistical explanations;
 *     proof-oriented explanations;
 *     model explanations;
 *     provenance explanations;
 *     compiler explanations;
 *     optimization explanations;
 *     resource-allocation explanations;
 *     quantum-transformation explanations;
 *     hardware-lowering explanations;
 *     simulation explanations;
 *     distributed-execution explanations;
 *     security explanations;
 *     future explanation mechanisms.
 *
 * No new grammar keyword is required for every future explanation method.
 *
 * ============================================================================
 * REASONING INTEGRATION
 * ============================================================================
 *
 * Explanation can describe reasoning operations:
 *
 *     explain conclusion;
 *
 *     explain decision from evidence;
 *
 *     explain result with (reasoning_context);
 *
 * Reasoning syntax remains owned by:
 *
 *     grammar/statements/reason.g4
 *     grammar/statements/infer.g4
 *     grammar/statements/deduce.g4
 *     grammar/expressions/reasoning.g4
 *
 * This file does not duplicate reasoning syntax.
 *
 * ============================================================================
 * KNOWLEDGE INTEGRATION
 * ============================================================================
 *
 * Explanation may consume knowledge-derived expressions:
 *
 *     explain answer from knowledge.query(pattern);
 *
 * Knowledge syntax remains owned by its existing data/knowledge grammars.
 *
 * This file does not create a second knowledge language.
 *
 * ============================================================================
 * LEARNING INTEGRATION
 * ============================================================================
 *
 * Explanation may describe learned-model results:
 *
 *     explain prediction;
 *
 *     explain prediction from training_record;
 *
 *     explain model(input) with (provenance);
 *
 * Learning syntax remains owned by the learning subsystem.
 *
 * This file does not enumerate:
 *
 *     model architectures;
 *     training algorithms;
 *     optimization algorithms;
 *     neural network families;
 *     reinforcement-learning algorithms;
 *     transfer-learning mechanisms.
 *
 * Those remain semantic/library capabilities.
 *
 * ============================================================================
 * ADAPTATION INTEGRATION
 * ============================================================================
 *
 * Explanation may describe an adaptation decision:
 *
 *     explain adaptation_decision;
 *
 *     explain selected_strategy from adaptation_record;
 *
 * Explanation does NOT authorize adaptation.
 *
 * Adaptation remains governed by:
 *
 *     policies;
 *     capabilities;
 *     effects;
 *     contracts;
 *     resources;
 *     provenance;
 *     authorization.
 *
 * ============================================================================
 * UNCERTAINTY INTEGRATION
 * ============================================================================
 *
 * Explanation can consume expressions involving:
 *
 *     probability;
 *     distributions;
 *     confidence;
 *     uncertainty;
 *     intervals;
 *     belief;
 *     observations.
 *
 * The grammar imposes no:
 *
 *     probability precision;
 *     distribution cardinality;
 *     confidence scale;
 *     numerical width;
 *     tensor rank;
 *     implementation representation.
 *
 * Those concerns belong to types and semantic analysis.
 *
 * ============================================================================
 * EVIDENCE INTEGRATION
 * ============================================================================
 *
 * Evidence is represented through ordinary expressions.
 *
 * Explanation may therefore consume evidence from:
 *
 *     classical computation;
 *     quantum measurement;
 *     simulation;
 *     hardware observation;
 *     data processing;
 *     model execution;
 *     distributed computation;
 *     network services;
 *     scientific computation;
 *     compiler transformations.
 *
 * Evidence validation and trust remain downstream concerns.
 *
 * ============================================================================
 * PROVENANCE INTEGRATION
 * ============================================================================
 *
 * Explanation is intentionally compatible with the existing provenance
 * expression subsystem.
 *
 * For example:
 *
 *     explain result from provenance(record);
 *
 *     explain decision with (provenance(record));
 *
 *     explain routing_decision from compilation_provenance;
 *
 * Provenance syntax remains owned by:
 *
 *     grammar/expressions/provenance.g4
 *
 * Provenance storage and verification remain downstream.
 *
 * ============================================================================
 * DECISION INTEGRATION
 * ============================================================================
 *
 * Explanation may target decisions produced by:
 *
 *     reasoning;
 *     optimization;
 *     resource negotiation;
 *     capability negotiation;
 *     routing;
 *     scheduling;
 *     resilience;
 *     security;
 *     adaptation;
 *     deployment;
 *     quantum compilation;
 *     hardware lowering;
 *     distributed execution.
 *
 * The grammar does not create a separate decision language.
 *
 * A decision is simply an expression-level semantic subject.
 *
 * ============================================================================
 * POLICY INTEGRATION
 * ============================================================================
 *
 * Explanation may be constrained by policies:
 *
 *     explain decision with (policy);
 *
 *     explain result from evidence with (policy);
 *
 * The grammar does not evaluate policy.
 *
 * Semantic analysis determines whether explanation is:
 *
 *     permitted;
 *     restricted;
 *     redacted;
 *     incomplete;
 *     unavailable;
 *     prohibited.
 *
 * Syntax validity does not imply authorization.
 *
 * ============================================================================
 * CONTRACT INTEGRATION
 * ============================================================================
 *
 * Explanation may appear in source code governed by:
 *
 *     requires;
 *     ensures;
 *     invariant;
 *     assume;
 *     guarantee;
 *     property.
 *
 * Contract syntax remains owned by the validation/contract subsystem.
 *
 * This file does not duplicate contract syntax.
 *
 * ============================================================================
 * EFFECT INTEGRATION
 * ============================================================================
 *
 * Explanation may be semantically effectful.
 *
 * Depending on the target and implementation, explanation may require:
 *
 *     computation;
 *     IO;
 *     provenance access;
 *     evidence access;
 *     network;
 *     model inspection;
 *     simulation;
 *     reflection;
 *     external services.
 *
 * The grammar does not declare or infer those effects.
 *
 * Semantic effect analysis determines them.
 *
 * An explanation statement MUST NOT silently grant an effect merely because
 * it is syntactically valid.
 *
 * ============================================================================
 * CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Semantic analysis may derive capabilities such as:
 *
 *     capability("explanation");
 *     capability("provenance.read");
 *     capability("evidence.read");
 *     capability("model.inspect");
 *     capability("simulation.inspect");
 *
 * These are semantic capabilities, not grammar-level hardware assumptions.
 *
 * This grammar does not resolve whether any target provides those capabilities.
 *
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * Explanation may require resources including:
 *
 *     compute;
 *     memory;
 *     storage;
 *     communication;
 *     simulation resources;
 *     model resources;
 *     provenance storage;
 *     distributed resources;
 *     accelerator resources;
 *     quantum resources.
 *
 * The grammar MUST NOT define:
 *
 *     MAX_EXPLANATION_DEPTH;
 *     MAX_EVIDENCE;
 *     MAX_PROVENANCE;
 *     MAX_CONTEXT;
 *     MAX_MEMORY;
 *     MAX_CPUS;
 *     MAX_GPUS;
 *     MAX_FPGAS;
 *     MAX_QUBITS;
 *     MAX_NODES;
 *     MAX_DEVICES;
 *     or equivalent universal ceilings.
 *
 * Actual feasibility belongs to resource analysis and execution planning.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Explanation syntax is independent of computational scale.
 *
 * The same source-level explanation statement may participate in compilation
 * for:
 *
 *     tiny computational substrates;
 *     embedded systems;
 *     CPUs;
 *     multicore systems;
 *     GPUs;
 *     FPGAs;
 *     ASICs;
 *     accelerators;
 *     QPUs;
 *     simulators;
 *     HPC systems;
 *     clusters;
 *     distributed systems;
 *     cloud systems;
 *     heterogeneous systems;
 *     future computational substrates.
 *
 * The grammar expresses intent only.
 *
 * Actual explanation feasibility depends on:
 *
 *     capabilities;
 *     resources;
 *     constraints;
 *     policies;
 *     effects;
 *     available provenance;
 *     available evidence;
 *     implementation support;
 *     execution environment.
 *
 * This file therefore introduces no machine-size assumptions.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Explanation can target or consume quantum-derived expressions:
 *
 *     explain quantum_result;
 *
 *     explain measurement_result;
 *
 *     explain routing_decision from compilation_record;
 *
 *     explain quantum_result with (provenance);
 *
 * This grammar does NOT define:
 *
 *     quantum gates;
 *     qubits;
 *     physical qubits;
 *     topology;
 *     routing;
 *     scheduling;
 *     calibration;
 *     QEC;
 *     resilience;
 *     ZQN;
 *     HAL;
 *     vendor-specific QPU syntax.
 *
 * Quantum semantics remain downstream.
 *
 * The canonical quantum path remains:
 *
 *     source
 *       ->
 *     lexer
 *       ->
 *     parser
 *       ->
 *     domain-neutral AST
 *       ->
 *     quantum semantic model
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
 *     QEC / resilience
 *       ->
 *     ZQN
 *       ->
 *     HAL
 *       ->
 *     target realization
 *
 * Explanation can consume artifacts produced along this pipeline without
 * becoming a quantum implementation grammar.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Explanation can describe:
 *
 *     hardware intent;
 *     synthesis decisions;
 *     placement decisions;
 *     routing decisions;
 *     timing decisions;
 *     resource negotiation;
 *     simulation results;
 *     verification results;
 *     lowering decisions.
 *
 * It does NOT encode:
 *
 *     bus widths;
 *     register widths;
 *     fixed memory sizes;
 *     fixed device counts;
 *     fixed clock counts;
 *     FPGA region identifiers;
 *     ASIC-specific physical assumptions;
 *     vendor-specific hardware inventories.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Explanation can describe:
 *
 *     distributed decisions;
 *     task placement;
 *     communication choices;
 *     scheduling;
 *     resilience;
 *     recovery;
 *     service selection;
 *     consistency decisions.
 *
 * No fixed number of:
 *
 *     nodes;
 *     workers;
 *     actors;
 *     tasks;
 *     channels;
 *     services
 *
 * is encoded.
 *
 * ============================================================================
 * INTEROPERABILITY INTEGRATION
 * ============================================================================
 *
 * Explanation targets may refer to values produced through:
 *
 *     FFI;
 *     ABI;
 *     foreign functions;
 *     external data;
 *     SQL;
 *     JSON;
 *     XML;
 *     service interfaces;
 *     external models.
 *
 * Interoperability syntax remains owned by its respective dialect or
 * interoperability grammar.
 *
 * ============================================================================
 * METAPROGRAMMING INTEGRATION
 * ============================================================================
 *
 * Explanation may target compile-time or generated artifacts:
 *
 *     explain generated_artifact;
 *
 *     explain transformation_record;
 *
 *     explain syntax_tree;
 *
 *     explain generated_type;
 *
 * Reflection and metaprogramming remain separately owned.
 *
 * Explanation does not grant reflection or code-generation capabilities.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar creates parser contexts only.
 *
 * The domain-neutral AST should represent the semantic intent approximately
 * as:
 *
 *     ExplanationStatement
 *         target
 *         source?
 *         context*
 *         source_span
 *
 * The AST MUST preserve:
 *
 *     target;
 *     optional source;
 *     ordered context;
 *     source spans;
 *     source ordering.
 *
 * The AST MUST NOT directly encode:
 *
 *     explanation algorithm;
 *     explanation engine;
 *     target hardware;
 *     physical device;
 *     QPU;
 *     physical qubit;
 *     CPU;
 *     GPU;
 *     FPGA;
 *     ASIC;
 *     network node;
 *     backend identifier.
 *
 * Existing AST architecture must be extended through the repository's
 * established compatibility process.
 *
 * This grammar does not require a particular Rust AST implementation.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * A syntactically valid explanation statement does NOT imply that:
 *
 *     the target exists;
 *     the target is explainable;
 *     evidence exists;
 *     provenance exists;
 *     an explanation mechanism exists;
 *     the caller is authorized;
 *     the explanation is complete;
 *     the explanation is truthful;
 *     the explanation is deterministic;
 *     the explanation is reproducible;
 *     the target is executable.
 *
 * Semantic analysis must independently establish those properties.
 *
 * Explanation should be treated as a semantic request whose realization may
 * succeed, partially succeed, or fail according to the applicable contracts,
 * capabilities, policies and resources.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * The source structure must remain traceable.
 *
 * Downstream provenance may associate an explanation with:
 *
 *     target;
 *     source;
 *     context;
 *     evidence;
 *     provenance;
 *     decision;
 *     transformation;
 *     source span;
 *     language version;
 *     semantic version;
 *     compiler transformation;
 *     execution record.
 *
 * This grammar does not generate provenance records itself.
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser diagnostics include:
 *
 *     missing EXPLAIN;
 *     missing target;
 *     malformed source clause;
 *     malformed context clause;
 *     missing closing parenthesis;
 *     malformed context list;
 *     missing statement terminator;
 *     unexpected clause order.
 *
 * Semantic diagnostics remain downstream:
 *
 *     unknown target;
 *     unavailable evidence;
 *     unavailable provenance;
 *     unsupported explanation mechanism;
 *     insufficient capability;
 *     insufficient resources;
 *     forbidden policy;
 *     effect violation;
 *     invalid target type;
 *     unavailable backend;
 *     unavailable simulation;
 *     unavailable introspection.
 *
 * The parser MUST NOT use semantic predicates to produce these semantic
 * decisions.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Given identical:
 *
 *     source;
 *     lexer vocabulary;
 *     grammar;
 *     parser configuration;
 *
 * this grammar MUST produce equivalent parse-tree structure.
 *
 * Parsing MUST NOT inspect:
 *
 *     hardware;
 *     resources;
 *     filesystem;
 *     network;
 *     clock;
 *     randomness;
 *     runtime state;
 *     target availability.
 *
 * ============================================================================
 * SAFETY CONTRACT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no embedded Rust;
 *     no semantic actions;
 *     no semantic predicates;
 *     no filesystem access;
 *     no network access;
 *     no runtime execution;
 *     no hardware access;
 *     no unsafe Rust requirement.
 *
 * Generated parser integration must remain compatible with:
 *
 *     Rust 1.97;
 *     Rust 1.97.1;
 *     Rust 2021;
 *
 * and must use safe Rust only.
 *
 * ============================================================================
 * COMPILER CONTRACT
 * ============================================================================
 *
 * The compiler may use an explanation request to expose or produce:
 *
 *     semantic derivations;
 *     transformation traces;
 *     optimization decisions;
 *     lowering decisions;
 *     resource decisions;
 *     capability decisions;
 *     routing decisions;
 *     scheduling decisions;
 *     resilience decisions;
 *     provenance records.
 *
 * This grammar does not require any particular explanation representation.
 *
 * Explanation output may be:
 *
 *     structured data;
 *     diagnostics;
 *     source-level explanation;
 *     machine-readable provenance;
 *     tooling output;
 *     human-readable documentation;
 *     formal evidence;
 *     future explanation representations.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar does NOT create an explanation-specific IR.
 *
 * The semantic representation should preserve:
 *
 *     explanation intent;
 *     target;
 *     source;
 *     context;
 *     provenance;
 *     policy;
 *     effects;
 *     capabilities;
 *     resource requirements.
 *
 * Downstream consumers may attach the explanation request to:
 *
 *     classical semantic representation;
 *     quantum::ir;
 *     HDL/hardware representation;
 *     distributed representation;
 *     accelerator representation;
 *     future domain representations.
 *
 * Explanation MUST NOT create a competing quantum IR.
 *
 * ============================================================================
 * TARGET INDEPENDENCE
 * ============================================================================
 *
 * This file must remain valid regardless of whether the target eventually
 * executes on:
 *
 *     embedded hardware;
 *     CPU;
 *     multicore CPU;
 *     GPU;
 *     FPGA;
 *     ASIC;
 *     accelerator;
 *     QPU;
 *     simulator;
 *     HPC;
 *     cluster;
 *     distributed system;
 *     cloud;
 *     heterogeneous system;
 *     future computational substrate.
 *
 * The grammar describes the request.
 *
 * Target realization is downstream.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/expressions/expressions.g4
 *
 * Consumes:
 *
 *     EXPLAIN
 *     FROM
 *     WITH
 *     LPAREN
 *     RPAREN
 *     COMMA
 *     SEMICOLON
 *
 * and:
 *
 *     expression
 *
 * EXPORTS:
 *
 *     explainStatement
 *     explanationTarget
 *     explanationSourceClause
 *     explanationContextClause
 *     explanationContextList
 *     explanationContext
 *
 * CONSUMED_BY:
 *
 *     grammar/statements/statements.g4
 *
 * AST_OWNER:
 *
 *     repository domain-neutral frontend AST
 *
 * SEMANTIC_OWNER:
 *
 *     explanation semantic subsystem
 *
 * TYPE_OWNER:
 *
 *     canonical type subsystem
 *
 * EFFECT_OWNER:
 *
 *     canonical effect subsystem
 *
 * CAPABILITY_OWNER:
 *
 *     canonical capability subsystem
 *
 * RESOURCE_OWNER:
 *
 *     canonical resource subsystem
 *
 * CONTRACT_OWNER:
 *
 *     canonical validation/contract subsystem
 *
 * POLICY_OWNER:
 *
 *     canonical policy subsystem
 *
 * PROVENANCE_OWNER:
 *
 *     canonical provenance subsystem
 *
 * IR_OWNER:
 *
 *     canonical semantic/domain IR pipeline
 *
 * QUANTUM_IR_OWNER:
 *
 *     quantum::ir
 *
 * TEST_OWNER:
 *
 *     grammar/tests/statements/explain/
 *
 * SPEC_OWNER:
 *
 *     grammar/specification/
 *     grammar/spec/
 *
 * ============================================================================
 * IMPORT CONTRACT
 * ============================================================================
 *
 * This grammar imports the canonical expression composition grammar:
 *
 *     Expressions
 *
 * It MUST NOT import:
 *
 *     Statements
 *     Domains
 *
 * because the universal statement composition grammar consumes this file.
 *
 * Dependency direction is:
 *
 *     Expressions
 *          ^
 *          |
 *     Explain
 *          ^
 *          |
 *     Statements
 *
 * This prevents circular grammar dependencies.
 *
 * ============================================================================
 * GRAMMAR
 * ============================================================================
 */

parser grammar Explain;

options {
    tokenVocab = ZamaniLexer;
}

import
    Expressions
    ;

/*
 * ============================================================================
 * PUBLIC STATEMENT ENTRY
 * ============================================================================
 *
 * Exactly one public statement-level entry point is owned by this grammar.
 *
 * ============================================================================
 */

explainStatement
    : EXPLAIN
      explanationTarget
      explanationSourceClause?
      explanationContextClause?
      SEMICOLON
    ;

/*
 * ============================================================================
 * TARGET
 * ============================================================================
 *
 * One complete canonical expression.
 *
 * ============================================================================
 */

explanationTarget
    : expression
    ;

/*
 * ============================================================================
 * SOURCE
 * ============================================================================
 *
 * One complete canonical expression.
 *
 * ============================================================================
 */

explanationSourceClause
    : FROM
      expression
    ;

/*
 * ============================================================================
 * CONTEXT
 * ============================================================================
 *
 * One or more ordered canonical expressions.
 *
 * Empty context and trailing commas are intentionally rejected.
 *
 * ============================================================================
 */

explanationContextClause
    : WITH
      LPAREN
      explanationContextList
      RPAREN
    ;

explanationContextList
    : explanationContext
      (
          COMMA
          explanationContext
      )*
    ;

explanationContext
    : expression
    ;

/*
 * ============================================================================
 * POSITIVE TEST CONTRACT
 * ============================================================================
 *
 * The repository conformance suite should accept:
 *
 *     explain result;
 *
 *     explain decision;
 *
 *     explain quantum_result;
 *
 *     explain hardware_state;
 *
 *     explain simulation_result;
 *
 *     explain result from evidence;
 *
 *     explain decision from provenance_record;
 *
 *     explain quantum_result from measurement_record;
 *
 *     explain result with (provenance);
 *
 *     explain decision with (evidence, policy);
 *
 *     explain result from evidence with (provenance, policy);
 *
 *     explain model(input);
 *
 *     explain model(input) from training_record
 *         with (provenance, policy);
 *
 *     explain routing_decision from compilation_record
 *         with (optimization, constraints);
 *
 *     explain resource_plan with (capability, requirement);
 *
 *     explain execution_plan from execution_record
 *         with (policy, provenance);
 *
 *     explain simulation_result from simulation_record
 *         with (configuration, provenance);
 *
 *     explain distributed_result from execution_record
 *         with (provenance, policy);
 *
 * The exact expression forms remain governed by the canonical expression
 * grammar.
 *
 * ============================================================================
 * NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * The repository conformance suite must reject:
 *
 *     explain;
 *
 *     explain from evidence;
 *
 *     explain with (policy);
 *
 *     explain result from;
 *
 *     explain result with;
 *
 *     explain result with ();
 *
 *     explain result with (evidence,);
 *
 *     explain result with (, evidence);
 *
 *     explain result with (evidence,, policy);
 *
 *     explain result with (evidence policy);
 *
 *     explain result with (evidence) from source;
 *
 *     explain result from source with;
 *
 *     explain result from source with ();
 *
 *     explain result
 *
 * when the enclosing language configuration requires the statement terminator.
 *
 * These are syntactic failures.
 *
 * Semantic failures such as:
 *
 *     explain unknown_value;
 *
 *     explain unavailable_artifact;
 *
 *     explain forbidden_decision;
 *
 * are NOT parser errors.
 *
 * ============================================================================
 * BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Test:
 *
 *     simple targets;
 *     nested expressions;
 *     function calls;
 *     member access;
 *     indexing;
 *     qualified names;
 *     collections;
 *     quantum-derived values;
 *     hardware-derived values;
 *     simulation-derived values;
 *     model-derived values;
 *     distributed values;
 *     provenance expressions;
 *     policy expressions;
 *     uncertainty expressions;
 *     deeply nested expressions;
 *     large context lists.
 *
 * The grammar must not introduce an artificial limit for any of these.
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Scalability tests should vary:
 *
 *     target-expression complexity;
 *     source-expression complexity;
 *     context count;
 *     context-expression complexity;
 *     nesting depth;
 *     qualified-name depth;
 *     source-file size;
 *     number of explanation statements.
 *
 * Tests MUST NOT define those sizes as language-level maxima.
 *
 * The same grammar must represent:
 *
 *     tiny explanation requests
 *
 * through:
 *
 *     very large explanation requests,
 *
 * subject only to actual compiler/parser/environment resource availability.
 *
 * ============================================================================
 * CROSS-DOMAIN TEST CONTRACT
 * ============================================================================
 *
 * Explanation must be tested with:
 *
 *     classical computation;
 *     quantum computation;
 *     hybrid computation;
 *     HDL;
 *     hardware;
 *     AI/model computation;
 *     reasoning;
 *     learning;
 *     adaptation;
 *     uncertainty;
 *     knowledge;
 *     data;
 *     distributed computation;
 *     networking;
 *     security;
 *     FFI;
 *     ABI;
 *     simulation;
 *     metaprogramming;
 *     compilation;
 *     execution.
 *
 * No domain may require modification of this grammar merely because the
 * explanation target belongs to that domain.
 *
 * ============================================================================
 * DETERMINISM TEST CONTRACT
 * ============================================================================
 *
 * Given identical:
 *
 *     source;
 *     lexer configuration;
 *     parser configuration;
 *     grammar version;
 *
 * repeated parsing must produce equivalent parse structure.
 *
 * The parser must not depend on:
 *
 *     target hardware;
 *     resource availability;
 *     runtime state;
 *     network state;
 *     filesystem state;
 *     current time;
 *     randomness.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * The public stable entry point is:
 *
 *     explainStatement
 *
 * Existing expression-level explanation calls, if present, remain distinct.
 *
 * For example:
 *
 *     explain(...)
 *
 * as an expression MUST remain owned by the expression subsystem.
 *
 * This statement grammar owns:
 *
 *     explain <expression> ... ;
 *
 * It MUST NOT redefine or replace expression-level `explain(...)`.
 *
 * This distinction prevents:
 *
 *     statement syntax
 *
 * from becoming:
 *
 *     expression syntax.
 *
 * ============================================================================
 * ERROR-RECOVERY CONTRACT
 * ============================================================================
 *
 * This grammar contains no custom parser actions.
 *
 * Standard ANTLR error recovery remains responsible for malformed input.
 *
 * The grammar must not:
 *
 *     inspect semantic state;
 *     inspect symbol tables;
 *     inspect resources;
 *     inspect capabilities;
 *     inspect hardware;
 *     execute code;
 *     invoke external services.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains:
 *
 *     no fixed hardware capacity;
 *     no fixed resource capacity;
 *     no fixed quantum capacity;
 *     no fixed number of targets;
 *     no fixed number of explanation contexts;
 *     no fixed evidence count;
 *     no fixed provenance count;
 *     no fixed model count;
 *     no fixed device count;
 *     no vendor catalogue;
 *     no finite explanation-algorithm catalogue.
 *
 * Repetition uses:
 *
 *     *
 *     +
 *
 * rather than artificial finite limits.
 *
 * ============================================================================
 * NO UNSAFE CONTRACT
 * ============================================================================
 *
 * This grammar contains no Rust code.
 *
 * It requires no:
 *
 *     unsafe function;
 *     unsafe block;
 *     unsafe trait;
 *     raw pointer;
 *     foreign-memory assumption.
 *
 * Generated Rust integration must remain safe Rust compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     [x] The file owns exactly one statement concept.
 *
 *     [x] `explainStatement` is the only public statement entry point.
 *
 *     [x] No universal `statement` rule is defined here.
 *
 *     [x] The canonical EXPLAIN token is consumed.
 *
 *     [x] No lexer token is defined here.
 *
 *     [x] Canonical Expressions are imported.
 *
 *     [x] No second expression hierarchy is created.
 *
 *     [x] Target is mandatory.
 *
 *     [x] Source is optional.
 *
 *     [x] Context is optional.
 *
 *     [x] Context is ordered.
 *
 *     [x] Empty context is rejected.
 *
 *     [x] Trailing context commas are rejected.
 *
 *     [x] Clause order is canonical.
 *
 *     [x] No semantic predicates are used.
 *
 *     [x] No embedded Rust is used.
 *
 *     [x] No unsafe Rust is required.
 *
 *     [x] No runtime execution is performed.
 *
 *     [x] No hardware is selected.
 *
 *     [x] No resource is allocated.
 *
 *     [x] No capability is resolved.
 *
 *     [x] No policy is evaluated.
 *
 *     [x] No explanation algorithm is enumerated.
 *
 *     [x] No quantum gate catalogue is introduced.
 *
 *     [x] No physical quantum topology is introduced.
 *
 *     [x] No HDL/hardware limits are introduced.
 *
 *     [x] No artificial scalability limit is introduced.
 *
 *     [x] AST ownership is explicitly defined.
 *
 *     [x] Semantic ownership is explicitly defined.
 *
 *     [x] Provenance ownership is explicitly defined.
 *
 *     [x] IR ownership is explicitly defined.
 *
 *     [x] Quantum integration terminates at the canonical semantic/quantum
 *         boundary and does not create a second quantum IR.
 *
 *     [x] Positive tests are specified.
 *
 *     [x] Negative tests are specified.
 *
 *     [x] Boundary tests are specified.
 *
 *     [x] Scalability tests are specified.
 *
 *     [x] Determinism tests are specified.
 *
 *     [x] Compatibility rules are specified.
 *
 * Repository-level production readiness additionally requires the universal
 * statement composition grammar and downstream AST/semantic implementation to
 * consume this contract.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL INVARIANT
 * ============================================================================
 *
 * This file answers:
 *
 *     "How does a Zamani program request an explanation?"
 *
 * It does NOT answer:
 *
 *     "How is the explanation generated?"
 *
 *     "Where is provenance stored?"
 *
 *     "Which evidence database is used?"
 *
 *     "Which model generates the explanation?"
 *
 *     "Which compiler backend is inspected?"
 *
 *     "Which machine produces the explanation?"
 *
 *     "Which QPU produces the result?"
 *
 *     "Which physical qubit is involved?"
 *
 *     "Which GPU or CPU is used?"
 *
 * Those concerns remain downstream.
 *
 * The architectural invariant is:
 *
 *     explanation syntax
 *          ->
 *     domain-neutral AST
 *          ->
 *     semantic explanation request
 *          ->
 *     evidence / provenance / policy / effect / capability analysis
 *          ->
 *     canonical semantic representation
 *          ->
 *     domain IR
 *          ->
 *     target realization / tooling
 *
 * Therefore explanation remains a universal language capability rather than
 * an application-specific feature.
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */
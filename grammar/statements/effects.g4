/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/statements/effects.g4
 *
 * STATUS
 * ------
 * CANONICAL EFFECT-STATEMENT COMPOSITION ADAPTER
 *
 * GRAMMAR TECHNOLOGY
 * ------------------
 * ANTLR4 parser grammar
 *
 * LANGUAGE / RUNTIME BASELINE
 * ---------------------------
 * Rust 1.97 / Rust 1.97.1
 * Rust 2021
 *
 * SAFETY
 * ------
 * This grammar:
 *
 *   - contains no embedded Rust actions;
 *   - contains no semantic predicates;
 *   - contains no unsafe Rust;
 *   - performs no I/O;
 *   - performs no filesystem access;
 *   - performs no networking;
 *   - performs no process execution;
 *   - performs no hardware discovery;
 *   - performs no runtime execution;
 *   - contains no machine-size constants;
 *   - contains no hardware-specific limits.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the STATEMENT-LAYER ADAPTER for computational effects.
 *
 * It does NOT own the effect language itself.
 *
 * The effect domain is implemented by:
 *
 *     grammar/effects/
 *
 * In particular:
 *
 *     grammar/effects/effects.g4
 *     grammar/effects/effect-declarations.g4
 *     grammar/effects/effect-sets.g4
 *     grammar/effects/effect-handling.g4
 *     grammar/effects/custom-effects.g4
 *     grammar/effects/capabilities.g4
 *     grammar/effects/quantum.g4
 *     grammar/effects/io.g4
 *     grammar/effects/network.g4
 *     ...
 *
 * This file exists because the universal statement grammar needs a stable,
 * statement-level entry point for effect operations.
 *
 * ============================================================================
 * CRITICAL OWNERSHIP RULE
 * ============================================================================
 *
 * THIS FILE OWNS ONLY:
 *
 *     effectStatement
 *
 * and the composition of effect-domain statement productions into the
 * universal statement layer.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     effect declarations
 *     effect operation declarations
 *     effect references
 *     effect sets
 *     effect clauses
 *     effect types
 *     effect capabilities
 *     effect resources
 *     effect implementations
 *     effect semantics
 *     effect handler semantics
 *     expression precedence
 *     identifiers
 *     qualified names
 *     types
 *     quantum operations
 *     hardware operations
 *     resource allocation
 *     runtime dispatch
 *
 * ============================================================================
 * WHY THIS FILE EXISTS
 * ============================================================================
 *
 * A statement such as:
 *
 *     perform quantum::Measurement(q);
 *
 * is a statement-level construct.
 *
 * However, the syntax and semantic vocabulary for effects belong to the
 * effect subsystem.
 *
 * The architecture is therefore:
 *
 *     statement composition
 *             |
 *             v
 *     effectStatement
 *             |
 *             v
 *     effect-domain grammar
 *             |
 *             v
 *     domain-neutral AST
 *             |
 *             v
 *     semantic effect analysis
 *
 * This prevents the universal statement grammar from duplicating effect
 * syntax.
 *
 * ============================================================================
 * ARCHITECTURAL PIPELINE
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     parser
 *          |
 *          v
 *     Statements
 *          |
 *          +--> effectStatement
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> name resolution
 *          +--> type analysis
 *          +--> effect analysis
 *          +--> capability analysis
 *          +--> resource analysis
 *          +--> ownership analysis
 *          +--> portability analysis
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> classical representation
 *          +--> quantum semantic representation
 *          +--> HDL/hardware representation
 *          +--> distributed representation
 *          +--> accelerator representation
 *          +--> future-domain representation
 *          |
 *          v
 *     canonical IR
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     routing / scheduling / resilience / QEC / ZQN
 *          |
 *          v
 *     HAL / target lowering
 *          |
 *          v
 *     runtime / hardware
 *
 * This grammar MUST remain upstream of all semantic and target realization
 * systems.
 *
 * ============================================================================
 * EFFECT / CAPABILITY / RESOURCE SEPARATION
 * ============================================================================
 *
 * EFFECT
 * ------
 * Describes computational interaction or externally observable computational
 * behavior.
 *
 * CAPABILITY
 * ----------
 * Describes what an execution environment can provide.
 *
 * RESOURCE
 * --------
 * Describes computational resources that may exist, be required, constrained,
 * preferred, or negotiated.
 *
 * REQUIREMENT
 * -----------
 * A condition that a valid realization must satisfy.
 *
 * CONSTRAINT
 * ----------
 * A condition restricting valid realizations.
 *
 * PREFERENCE
 * ----------
 * A preferred but non-essential realization property.
 *
 * HINT
 * ----
 * An implementation suggestion that does not change program meaning.
 *
 * Effect statements MUST NOT silently become resource-selection or
 * hardware-selection statements.
 *
 * For example:
 *
 *     perform quantum::Measurement(q);
 *
 * does NOT mean:
 *
 *     select QPU 0
 *     select physical qubit 17
 *     use a particular topology
 *     use a particular calibration
 *     use a particular backend
 *
 * Those decisions belong downstream.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Effect statements express portable computational intent.
 *
 * They MUST NOT encode machine-scale assumptions.
 *
 * There is therefore no language-level limit on:
 *
 *     - number of effect statements;
 *     - number of effect operations;
 *     - number of effect references;
 *     - number of handlers;
 *     - number of handler arms;
 *     - handler nesting;
 *     - number of resumptions;
 *     - number of effect domains;
 *     - number of quantum effects;
 *     - number of classical effects;
 *     - number of hardware effects;
 *     - number of distributed effects;
 *     - program size;
 *     - machine size.
 *
 * Repetition is expressed through grammar composition and recursion rather
 * than fixed limits.
 *
 * Practical limits imposed by:
 *
 *     - available memory;
 *     - parser implementation;
 *     - compilation resources;
 *     - runtime resources;
 *     - deployment policy
 *
 * are implementation/resource concerns, NOT language semantics.
 *
 * ============================================================================
 * OPEN-WORLD EFFECT MODEL
 * ============================================================================
 *
 * The effect language is OPEN-WORLD.
 *
 * This file MUST NOT enumerate a finite set of effect names.
 *
 * It must remain valid for names such as:
 *
 *     IO
 *     Storage
 *     Network
 *     quantum::Measurement
 *     quantum::Reset
 *     hardware::Clock
 *     accelerator::Dispatch
 *     distributed::Consensus
 *     photonic::Interaction
 *     neuromorphic::Spike
 *     future::computing::Operation
 *
 * These names are resolved by semantic analysis.
 *
 * Adding a new effect domain MUST NOT require changing this statement adapter.
 *
 * ============================================================================
 * QUANTUM CONTRACT
 * ============================================================================
 *
 * Quantum effects remain ordinary effect-domain constructs.
 *
 * Examples:
 *
 *     perform quantum::Measurement(q);
 *     perform quantum::Reset(q);
 *     perform quantum::Readout(q);
 *
 * This file does NOT define:
 *
 *     - gates;
 *     - gate sets;
 *     - QubitId;
 *     - PhysicalQubitId;
 *     - topology;
 *     - connectivity;
 *     - calibration;
 *     - pulse schedules;
 *     - noise models;
 *     - QEC codes;
 *     - ZQN faults;
 *     - routing;
 *     - scheduling;
 *     - backend selection.
 *
 * The canonical quantum semantic boundary remains:
 *
 *     quantum::ir
 *
 * The required downstream relationship is:
 *
 *     effect statement
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic quantum analysis
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     routing
 *          |
 *          v
 *     scheduling
 *          |
 *          v
 *     QEC / resilience / ZQN
 *          |
 *          v
 *     HAL
 *          |
 *          v
 *     target realization
 *
 * No second frontend-specific quantum IR is introduced here.
 *
 * ============================================================================
 * CLASSICAL CONTRACT
 * ============================================================================
 *
 * Classical effects are represented through the same open-world effect
 * mechanism.
 *
 * Examples:
 *
 *     perform IO::read(...);
 *     perform Storage::write(...);
 *     perform Memory::allocate(...);
 *
 * No CPU, register, cache, or operating-system implementation is encoded by
 * this grammar.
 *
 * ============================================================================
 * HDL / HARDWARE CONTRACT
 * ============================================================================
 *
 * Hardware-related effects may be represented as ordinary qualified effect
 * names.
 *
 * Examples:
 *
 *     perform hardware::clock(...);
 *     perform hardware::io(...);
 *     perform accelerator::dispatch(...);
 *
 * This does NOT specify:
 *
 *     - FPGA size;
 *     - ASIC resources;
 *     - bus width;
 *     - register count;
 *     - pipeline count;
 *     - device ID;
 *     - clock source;
 *     - physical topology;
 *     - placement.
 *
 * Those properties belong to hardware/resource/compiler semantics.
 *
 * ============================================================================
 * DISTRIBUTED CONTRACT
 * ============================================================================
 *
 * Distributed effects may describe:
 *
 *     communication
 *     consensus
 *     replication
 *     remote execution
 *     coordination
 *     service interaction
 *
 * Example:
 *
 *     perform distributed::communication(message);
 *
 * This grammar does NOT encode:
 *
 *     - node count;
 *     - cluster size;
 *     - endpoint count;
 *     - physical addresses;
 *     - network topology;
 *     - deployment region;
 *     - machine identity.
 *
 * ============================================================================
 * AI / DATA CONTRACT
 * ============================================================================
 *
 * AI and data domains may expose effects such as:
 *
 *     perform data::read(...);
 *     perform model::inference(...);
 *     perform accelerator::tensor(...);
 *
 * This file does not encode:
 *
 *     - model size;
 *     - tensor rank limit;
 *     - accelerator count;
 *     - GPU model;
 *     - framework;
 *     - vendor API.
 *
 * ============================================================================
 * EXPRESSION OWNERSHIP
 * ============================================================================
 *
 * The expression after an effect operation is NOT parsed by this file.
 *
 * The canonical expression grammar remains authoritative.
 *
 * For example:
 *
 *     perform quantum::Measurement(q);
 *
 * is admitted through the effect-domain statement rule, while:
 *
 *     q
 *
 *     arguments
 *
 *     nested expressions
 *
 *     calls
 *
 *     operators
 *
 * remain owned by:
 *
 *     grammar/expressions/
 *
 * This prevents a second expression grammar from appearing inside the effect
 * subsystem.
 *
 * ============================================================================
 * EFFECT-DOMAIN OWNER
 * ============================================================================
 *
 * The actual effect statement forms are owned by:
 *
 *     grammar/effects/effects.g4
 *
 * That aggregate grammar is responsible for composing the specialized effect
 * grammars.
 *
 * In particular, it may compose:
 *
 *     EffectDeclarations
 *     EffectSets
 *     EffectHandling
 *     custom effect grammars
 *     domain-specific effect grammars
 *
 * This statement adapter consumes the canonical statement-level entry point
 * exported by the effect aggregate.
 *
 * ============================================================================
 * IMPORTANT: NO DUPLICATION
 * ============================================================================
 *
 * This file MUST NOT redefine:
 *
 *     effectDeclaration
 *     effectOperationDeclaration
 *     effectReference
 *     effectReferenceList
 *     effectSet
 *     effectClause
 *     effectHandler
 *     effectHandlerPattern
 *     resumeExpression
 *     abortExpression
 *     performExpression
 *
 * if those rules are already owned by grammar/effects/ or
 * grammar/expressions/.
 *
 * The statement adapter exists solely to make the effect statement family
 * reachable from the universal statement dispatcher.
 *
 * ============================================================================
 * CANONICAL STATEMENT INTEGRATION
 * ============================================================================
 *
 * `grammar/statements/statements.g4` remains the sole owner of:
 *
 *     statement
 *
 * It should import this grammar:
 *
 *     EffectStatements
 *
 * and add:
 *
 *     | effectStatement
 *
 * to its canonical dispatcher.
 *
 * The resulting conceptual dispatcher is:
 *
 *     statement
 *         : declarationStatement
 *         | assignmentStatement
 *         | assertionStatement
 *         | controlFlowStatement
 *         | unsafeStatement
 *         | blockExpression
 *         | emptyStatement
 *         | effectStatement
 *         | expressionStatement
 *         ;
 *
 * This file MUST NOT define `statement`.
 *
 * ============================================================================
 * ROOT-GRAMMAR INTEGRATION
 * ============================================================================
 *
 * The canonical root grammar:
 *
 *     grammar/Zamani.g4
 *
 * MUST reach effect statements through:
 *
 *     program
 *       -> item
 *       -> statement
 *       -> effectStatement
 *
 * It MUST NOT independently import and dispatch effect statements through a
 * second path.
 *
 * This prevents duplicate parse-tree paths and competing statement ownership.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar creates no AST nodes itself.
 *
 * The frontend AST layer must map an effect statement to the repository's
 * domain-neutral operation/effect representation.
 *
 * At minimum, the resulting AST representation must preserve:
 *
 *     - source span;
 *     - operation/effect identity;
 *     - operand/argument structure;
 *     - syntactic modifiers;
 *     - handler structure where applicable;
 *     - nested statements/expressions;
 *     - source ordering;
 *     - attributes relevant to semantic analysis.
 *
 * The AST MUST NOT encode:
 *
 *     - physical device identity;
 *     - physical qubit identity;
 *     - CPU identity;
 *     - GPU identity;
 *     - FPGA identity;
 *     - node identity;
 *     - topology;
 *     - backend selection.
 *
 * ============================================================================
 * GENERIC OPERATION MODEL
 * ============================================================================
 *
 * Effect operations should map naturally into the repository's generic
 * operation model where appropriate:
 *
 *     Operation {
 *         name,
 *         namespace,
 *         operands,
 *         parameters,
 *         results,
 *         attributes,
 *         modifiers,
 *         effects,
 *         capabilities,
 *         source
 *     }
 *
 * This keeps effect syntax compatible with:
 *
 *     classical operations;
 *     quantum operations;
 *     HDL/hardware operations;
 *     accelerator operations;
 *     distributed operations;
 *     future domains.
 *
 * Effect statements therefore MUST NOT force a finite enum of operations.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing establishes syntactic validity only.
 *
 * Semantic analysis determines:
 *
 *     - whether an effect exists;
 *     - whether the effect is imported;
 *     - whether the operation exists;
 *     - whether the operation is callable;
 *     - whether arguments have valid types;
 *     - whether the effect is permitted;
 *     - whether required capabilities exist;
 *     - whether resource requirements can be satisfied;
 *     - whether ownership rules are satisfied;
 *     - whether the effect is compatible with the enclosing context;
 *     - whether effect composition is valid;
 *     - whether handler coverage is sufficient;
 *     - whether a quantum effect can lower to quantum::ir;
 *     - whether an HDL/hardware effect can lower to the appropriate
 *       representation.
 *
 * An unavailable effect implementation is NOT a parser error.
 *
 * It is a semantic, compilation, deployment, or runtime concern according to
 * the language's diagnostic model.
 *
 * ============================================================================
 * CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Effect syntax does not itself discover capabilities.
 *
 * Semantic analysis may associate an effect with capabilities such as:
 *
 *     quantum.measurement
 *     tensor.compute
 *     distributed.communication
 *     hardware.reconfiguration
 *
 * Capability resolution belongs downstream.
 *
 * This allows the same source program to remain portable across environments
 * with different capabilities.
 *
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * Effect statements do not allocate resources directly.
 *
 * A semantic effect may imply resource requirements, for example:
 *
 *     quantum::Measurement
 *
 * may require a measurement-capable quantum realization.
 *
 * The relationship is:
 *
 *     effect
 *       ->
 *     semantic requirement
 *       ->
 *     capability/resource analysis
 *       ->
 *     realization
 *
 * The grammar itself MUST NOT convert that into:
 *
 *     device 0
 *     qubit 17
 *     GPU 3
 *     node 8
 *
 * ============================================================================
 * CANONICAL IR CONTRACT
 * ============================================================================
 *
 * This file creates no IR.
 *
 * The downstream path is:
 *
 *     source
 *       -> lexer
 *       -> parser
 *       -> AST
 *       -> semantic analysis
 *       -> canonical semantic model
 *       -> canonical IR
 *
 * For quantum:
 *
 *     AST
 *       -> semantic quantum operation
 *       -> quantum::ir
 *
 * For classical:
 *
 *     AST
 *       -> classical semantic representation
 *       -> classical IR
 *
 * For HDL/hardware:
 *
 *     AST
 *       -> hardware semantic representation
 *       -> hardware/HDL IR
 *
 * For distributed systems:
 *
 *     AST
 *       -> distributed semantic representation
 *       -> distributed lowering
 *
 * No grammar file may directly depend on runtime IR implementation details.
 *
 * ============================================================================
 * COMPILER INTEGRATION
 * ============================================================================
 *
 * The compiler consumes semantic effect information after AST construction.
 *
 * The compiler may:
 *
 *     - specialize;
 *     - inline;
 *     - eliminate;
 *     - combine;
 *     - lower;
 *     - route;
 *     - schedule;
 *     - transform;
 *     - distribute;
 *     - map;
 *     - select implementations.
 *
 * Those are compiler decisions and MUST NOT be encoded into this statement
 * grammar.
 *
 * ============================================================================
 * RUNTIME INTEGRATION
 * ============================================================================
 *
 * Runtime dispatch is downstream.
 *
 * Runtime may determine how an effect is realized on available resources.
 *
 * This grammar does not define:
 *
 *     - queues;
 *     - threads;
 *     - workers;
 *     - devices;
 *     - runtime handles;
 *     - memory pools;
 *     - accelerator contexts;
 *     - QPU contexts.
 *
 * ============================================================================
 * DIAGNOSTICS CONTRACT
 * ============================================================================
 *
 * Syntax diagnostics belong to the parser.
 *
 * Examples:
 *
 *     malformed effect statement
 *     missing operation
 *     missing delimiter
 *     malformed handler
 *     malformed resumption
 *
 * Semantic diagnostics belong downstream.
 *
 * Examples:
 *
 *     unknown effect
 *     unknown effect operation
 *     invalid effect argument
 *     unavailable capability
 *     impossible resource requirement
 *     invalid effect composition
 *     unhandled effect
 *     invalid quantum realization
 *
 * Diagnostics must preserve source spans.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing must depend only on:
 *
 *     - token stream;
 *     - grammar version;
 *     - parser configuration.
 *
 * It MUST NOT depend on:
 *
 *     - CPU count;
 *     - GPU availability;
 *     - QPU availability;
 *     - filesystem state;
 *     - network state;
 *     - wall-clock time;
 *     - random state;
 *     - deployment topology;
 *     - runtime state.
 *
 * Identical source under identical parser configuration must produce
 * equivalent parse structure.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * This file introduces a new modular statement adapter without renaming the
 * existing effect-domain files.
 *
 * Existing effect syntax remains owned by the effect subsystem.
 *
 * Compatibility-sensitive changes include:
 *
 *     - changing `effectStatement`;
 *     - changing imported effect grammar ownership;
 *     - changing the spelling/meaning of effect statement keywords;
 *     - changing parse-tree shape relied upon by AST lowering.
 *
 * Such changes must be recorded in:
 *
 *     grammar/compatibility/
 *
 * and:
 *
 *     grammar/spec/compatibility.md
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * The file is complete only when the following categories exist.
 *
 * ---------------------------------------------------------------------------
 * POSITIVE
 * ---------------------------------------------------------------------------
 *
 *     perform effect_operation;
 *
 *     perform quantum::Measurement(q);
 *
 *     perform distributed::communication(message);
 *
 *     perform hardware::operation(value);
 *
 *     handle computation {
 *         case Effect::operation(value) => resume(value);
 *     }
 *
 *     resume;
 *
 *     resume(value);
 *
 *     abort;
 *
 *     abort(error);
 *
 * ---------------------------------------------------------------------------
 * NEGATIVE
 * ---------------------------------------------------------------------------
 *
 *     malformed perform;
 *     malformed handler;
 *     malformed handler pattern;
 *     malformed resume;
 *     malformed abort;
 *     missing required delimiter;
 *     incomplete effect operation.
 *
 * ---------------------------------------------------------------------------
 * SEMANTIC NEGATIVE
 * ---------------------------------------------------------------------------
 *
 * These belong downstream rather than in this grammar:
 *
 *     unknown effect;
 *     unknown operation;
 *     invalid argument type;
 *     unavailable capability;
 *     unavailable resource;
 *     invalid quantum operation;
 *     invalid hardware realization;
 *     invalid effect handling;
 *
 * ---------------------------------------------------------------------------
 * BOUNDARY
 * ---------------------------------------------------------------------------
 *
 * Tests must cover:
 *
 *     - empty effect handler bodies;
 *     - many handler arms;
 *     - nested handlers;
 *     - nested effect expressions;
 *     - long qualified names;
 *     - many effect arguments;
 *     - many effect statements;
 *     - large programs;
 *     - deeply nested blocks.
 *
 * ---------------------------------------------------------------------------
 * CROSS-DOMAIN
 * ---------------------------------------------------------------------------
 *
 * Tests must cover effect statements associated with:
 *
 *     classical;
 *     quantum;
 *     hybrid;
 *     HDL;
 *     hardware;
 *     distributed;
 *     AI/data;
 *     networking;
 *     security;
 *     accelerator;
 *     future domains.
 *
 * ---------------------------------------------------------------------------
 * SCALABILITY
 * ---------------------------------------------------------------------------
 *
 * No source-language limit may be established for:
 *
 *     - effect count;
 *     - operation count;
 *     - handler count;
 *     - handler-arm count;
 *     - nesting;
 *     - argument count;
 *     - effect-set size;
 *     - program size;
 *     - machine size;
 *     - qubit count;
 *     - CPU count;
 *     - GPU count;
 *     - FPGA count;
 *     - node count;
 *     - accelerator count.
 *
 * Test-environment limits are implementation constraints only.
 *
 * ---------------------------------------------------------------------------
 * DETERMINISM
 * ---------------------------------------------------------------------------
 *
 * Repeated parsing of identical token streams must produce equivalent parse
 * trees.
 *
 * ---------------------------------------------------------------------------
 * ROUND-TRIP
 * ---------------------------------------------------------------------------
 *
 * Where the frontend formatter exists:
 *
 *     source
 *       -> lexer
 *       -> parser
 *       -> AST
 *       -> formatter
 *       -> parser
 *
 * must preserve effect-statement meaning.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden:
 *
 *     MAX_EFFECT_STATEMENTS
 *     MAX_EFFECTS
 *     MAX_OPERATIONS
 *     MAX_HANDLERS
 *     MAX_HANDLER_ARMS
 *     MAX_EFFECT_ARGS
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_MEMORY
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *
 * Numeric literals appearing in source remain program semantics and are not
 * grammar limits.
 *
 * ============================================================================
 * RUST SAFETY CONTRACT
 * ============================================================================
 *
 * This grammar contains no Rust code.
 *
 * Generated and handwritten frontend/compiler code consuming this grammar must
 * support:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * without requiring `unsafe`.
 *
 * The grammar MUST NOT introduce an embedded Rust action as a shortcut for
 * semantic behavior.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [ ] `EffectStatements` compiles as an ANTLR parser grammar.
 *
 *     [ ] The canonical lexer vocabulary is consumed consistently with the
 *         production parser.
 *
 *     [ ] The effect-domain aggregate exposes the statement entry point used
 *         here.
 *
 *     [ ] No effect declaration syntax is duplicated here.
 *
 *     [ ] No effect-set syntax is duplicated here.
 *
 *     [ ] No effect-handler implementation is duplicated here.
 *
 *     [ ] No expression grammar is duplicated here.
 *
 *     [ ] `effectStatement` is reachable from `Statements.statement`.
 *
 *     [ ] `Statements` remains the sole owner of `statement`.
 *
 *     [ ] `Zamani.g4` reaches effects through the statement composition path.
 *
 *     [ ] AST lowering has a predetermined effect-statement contract.
 *
 *     [ ] Semantic effect analysis consumes the AST representation.
 *
 *     [ ] Capability/resource analysis remains downstream.
 *
 *     [ ] Quantum semantics reach canonical `quantum::ir`.
 *
 *     [ ] No second quantum IR is introduced.
 *
 *     [ ] Hardware topology remains downstream.
 *
 *     [ ] Routing remains downstream.
 *
 *     [ ] Scheduling remains downstream.
 *
 *     [ ] QEC remains downstream.
 *
 *     [ ] ZQN remains downstream.
 *
 *     [ ] Runtime dispatch remains downstream.
 *
 *     [ ] No machine/resource limits exist in this file.
 *
 *     [ ] Positive tests pass.
 *
 *     [ ] Negative tests pass.
 *
 *     [ ] Boundary tests pass.
 *
 *     [ ] Cross-domain tests pass.
 *
 *     [ ] Scalability tests pass.
 *
 *     [ ] Determinism tests pass.
 *
 *     [ ] Compatibility tests pass.
 *
 * ============================================================================
 * CANONICAL IMPLEMENTATION
 * ============================================================================
 */

parser grammar EffectStatements;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * ============================================================================
 * EFFECT-DOMAIN COMPOSITION
 * ============================================================================
 *
 * The effect aggregate is the authoritative owner of effect-domain syntax.
 *
 * This adapter imports it instead of copying its rules.
 * ============================================================================
 */

import Effects;

/*
 * ============================================================================
 * EFFECT STATEMENT
 * ============================================================================
 *
 * This is the ONLY rule owned by this file.
 *
 * The alternatives below are statement-level rules exported by the canonical
 * effect aggregate.
 *
 * The concrete syntax remains owned by grammar/effects/.
 * ============================================================================
 */

effectStatement
    : performStatement
    | handleStatement
    | resumeStatement
    | abortStatement
    ;
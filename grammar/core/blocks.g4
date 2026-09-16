/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/core/blocks.g4
 *
 * Status:
 *     Production-ready core grammar component.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar component.
 *
 * Rust integration baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     This grammar contains no embedded Rust actions, semantic predicates,
 *     target-specific implementation code, I/O, filesystem access, networking,
 *     process execution, hardware discovery, runtime execution, or mutable
 *     global state.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the SINGLE CORE SYNTAX OWNER for the universal Zamani block
 * structure.
 *
 * A block is a source-level ordered region delimited by:
 *
 *     {
 *         ...
 *     }
 *
 * The same block structure is reusable by:
 *
 *     - classical computing;
 *     - functional computation;
 *     - object-oriented computation;
 *     - concurrent computation;
 *     - parallel computation;
 *     - distributed computation;
 *     - quantum computation;
 *     - hybrid quantum/classical computation;
 *     - HDL;
 *     - hardware/software co-design;
 *     - AI/ML;
 *     - data processing;
 *     - networking;
 *     - security;
 *     - embedded computation;
 *     - accelerator computation;
 *     - scientific computation;
 *     - future computational domains.
 *
 * A block is SOURCE STRUCTURE.
 *
 * It is not:
 *
 *     - an execution resource;
 *     - a hardware region;
 *     - a quantum circuit;
 *     - a scheduling region;
 *     - a routing region;
 *     - an IR block;
 *     - a runtime object;
 *     - a physical device;
 *     - a machine topology.
 *
 * ============================================================================
 * AUTHORITATIVE ARCHITECTURE
 * ============================================================================
 *
 * The authoritative language composition root remains:
 *
 *     grammar/Zamani.g4
 *
 * This file is a reusable grammar component consumed by that composition.
 *
 * The authoritative statement composition remains:
 *
 *     grammar/statements/statements.g4
 *
 * The authoritative expression composition remains under:
 *
 *     grammar/expressions/
 *
 * This file MUST NOT redefine the complete statement grammar or expression
 * grammar.
 *
 * It provides the integration boundary:
 *
 *     blockElement
 *         |
 *         +--> statement
 *         |
 *         +--> expression
 *
 * The final composed Zamani grammar resolves those references against the
 * canonical statement/expression owners.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - block
 *     - blockExpression
 *     - blockElement
 *     - block delimiter structure
 *     - ordered block-element structure
 *     - empty-block syntax
 *     - nested-block syntax through canonical composition
 *     - the source-level distinction between a block and its contents
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexer rules
 *     - identifier syntax
 *     - names
 *     - paths
 *     - expressions
 *     - expression precedence
 *     - expression operators
 *     - statements
 *     - declarations
 *     - functions
 *     - loops
 *     - conditionals
 *     - match semantics
 *     - exceptions
 *     - effects
 *     - ownership
 *     - borrowing
 *     - types
 *     - capabilities
 *     - resources
 *     - hardware
 *     - topology
 *     - quantum operations
 *     - quantum IR
 *     - QEC
 *     - ZQN
 *     - routing
 *     - scheduling
 *     - optimization
 *     - calibration
 *     - HAL
 *     - runtime execution
 *     - backend selection
 *     - deployment
 *     - machine limits
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This file consumes the canonical lexer vocabulary.
 *
 * The universal brace tokens are:
 *
 *     LBRACE
 *     RBRACE
 *
 * They are owned by the lexer/punctuation layer.
 *
 * This file MUST NOT define:
 *
 *     LBRACE
 *     RBRACE
 *     IDENTIFIER
 *     SEMICOLON
 *     keywords
 *     operators
 *     literals
 *
 * The existing repository lexer architecture already defines LBRACE/RBRACE
 * in the punctuation layer.
 *
 * ============================================================================
 * TOKEN-NAMING CONTRACT
 * ============================================================================
 *
 * The canonical modular grammar vocabulary uses symbolic token names such as:
 *
 *     LBRACE
 *     RBRACE
 *
 * rather than introducing another spelling for the same punctuation.
 *
 * Therefore this file intentionally uses:
 *
 *     LBRACE
 *     RBRACE
 *
 * and not raw:
 *
 *     '{'
 *     '}'
 *
 * This keeps the modular grammar aligned with the canonical lexer contract.
 *
 * ============================================================================
 * BLOCK MODEL
 * ============================================================================
 *
 * Conceptually:
 *
 *     block
 *         |
 *         v
 *     blockExpression
 *         |
 *         +--> blockElement*
 *
 * A block contains zero or more ordered source elements.
 *
 * The grammar imposes no finite maximum.
 *
 * ============================================================================
 * PRIMARY BLOCK RULE
 * ============================================================================
 *
 * `block` is the compatibility/source-structure entry point used by constructs
 * such as:
 *
 *     function bodies
 *     loops
 *     branches
 *     handlers
 *     implementations
 *     modules
 *     domain bodies
 *     execution regions
 *
 * `block` delegates to the canonical `blockExpression` representation.
 *
 * This prevents the language from developing separate syntactic block
 * representations for statement and expression contexts.
 */

block
    : blockExpression
    ;


/*
 * ============================================================================
 * BLOCK EXPRESSION
 * ============================================================================
 *
 * A block is also a value-capable source expression.
 *
 * Examples:
 *
 *     {}
 *
 *     {
 *         compute();
 *     }
 *
 *     {
 *         let x = compute();
 *         x
 *     }
 *
 *     {
 *         prepare();
 *         apply_operation();
 *         measure();
 *     }
 *
 * The grammar records source structure only.
 *
 * Semantic analysis determines whether a particular block:
 *
 *     - produces a value;
 *     - has a reachable final expression;
 *     - has compatible control-flow exits;
 *     - has a valid result type;
 *     - has permitted effects;
 *     - satisfies ownership/borrowing rules;
 *     - satisfies resource/capability requirements.
 *
 * None of those semantic decisions belong here.
 */

blockExpression
    : LBRACE blockElement* RBRACE
    ;


/*
 * ============================================================================
 * BLOCK ELEMENT
 * ============================================================================
 *
 * A block element is one ordered source-level child.
 *
 * There are two canonical source possibilities:
 *
 *     statement
 *     expression
 *
 * The statement grammar owns statement syntax.
 *
 * The expression grammar owns expression syntax.
 *
 * This file merely establishes that either canonical construct may occupy a
 * block element position.
 *
 * This is intentional because the existing Zamani frontend AST represents a
 * block as an ordered sequence of NodeId children rather than introducing a
 * separate parser-only `tail_expression` representation.
 *
 * Semantic analysis determines whether an expression occurring in a particular
 * position is valid and, where applicable, whether the final reachable
 * expression supplies the block value.
 */

blockElement
    : statement
    | expression
    ;


/*
 * ============================================================================
 * EMPTY BLOCK
 * ============================================================================
 *
 * The following is syntactically valid:
 *
 *     {}
 *
 * Semantic layers determine whether an empty block is permitted in a
 * particular context.
 *
 * Examples of contexts that may impose different semantic requirements:
 *
 *     - function body;
 *     - branch body;
 *     - loop body;
 *     - handler body;
 *     - synchronization region;
 *     - hardware process;
 *     - quantum control region;
 *     - distributed execution region.
 *
 * The core grammar does not encode those contextual restrictions.
 */


/*
 * ============================================================================
 * ORDER PRESERVATION
 * ============================================================================
 *
 * `blockElement*` is deliberately ordered.
 *
 * The frontend AST MUST preserve the exact source order.
 *
 * Ordering may affect:
 *
 *     - evaluation;
 *     - side effects;
 *     - lexical scope;
 *     - lifetime;
 *     - ownership;
 *     - borrowing;
 *     - control flow;
 *     - diagnostics;
 *     - source reconstruction;
 *     - dependency analysis;
 *     - classical/quantum interaction;
 *     - resource semantics;
 *     - hardware-intent semantics.
 *
 * The grammar therefore MUST NOT replace block elements with an unordered
 * grammar representation.
 */


/*
 * ============================================================================
 * NESTED BLOCKS
 * ============================================================================
 *
 * Nested blocks are naturally represented because a block may occur inside
 * canonical statements or expressions.
 *
 * Examples:
 *
 *     {
 *         if condition {
 *             work();
 *         }
 *     }
 *
 *     {
 *         while condition {
 *             work();
 *         }
 *     }
 *
 *     {
 *         {
 *             nested_work();
 *         }
 *     }
 *
 * No language-level maximum nesting depth is encoded.
 *
 * Any implementation-level parser/resource protection must be configurable
 * outside the language grammar.
 */


/*
 * ============================================================================
 * BLOCK VALUES
 * ============================================================================
 *
 * A block may occur in an expression position.
 *
 * Example:
 *
 *     let result = {
 *         let value = compute();
 *         value
 *     };
 *
 * The grammar intentionally does NOT encode:
 *
 *     block must contain an expression
 *     final element must be an expression
 *     final expression must have type T
 *     all paths must return the same type
 *
 * Those are semantic/type/control-flow rules.
 *
 * This separation is necessary for:
 *
 *     - generic programming;
 *     - effect analysis;
 *     - control-flow analysis;
 *     - divergence;
 *     - never-returning expressions;
 *     - compile-time evaluation;
 *     - asynchronous computation;
 *     - quantum/classical hybrid blocks;
 *     - future computational domains.
 */


/*
 * ============================================================================
 * STATEMENT INTEGRATION
 * ============================================================================
 *
 * The canonical statement rule is owned by:
 *
 *     grammar/statements/statements.g4
 *
 * This file references `statement` but does not redefine it.
 *
 * Therefore all current and future statement families can enter a block
 * without changing this file.
 *
 * The statement composition may eventually admit:
 *
 *     - declarations;
 *     - assignments;
 *     - assertions;
 *     - conditionals;
 *     - loops;
 *     - pattern matching;
 *     - return;
 *     - break;
 *     - continue;
 *     - exception handling;
 *     - effects;
 *     - concurrency;
 *     - classical operations;
 *     - quantum operations;
 *     - hybrid operations;
 *     - HDL operations;
 *     - hardware intent;
 *     - distributed operations;
 *     - AI/data operations;
 *     - networking;
 *     - security;
 *     - compilation directives;
 *     - execution directives;
 *     - future domain statements.
 *
 * None of those are enumerated here.
 */


/*
 * ============================================================================
 * EXPRESSION INTEGRATION
 * ============================================================================
 *
 * The canonical expression rule is owned by:
 *
 *     grammar/expressions/
 *
 * This file references `expression` but does not redefine:
 *
 *     - precedence;
 *     - associativity;
 *     - operators;
 *     - calls;
 *     - indexing;
 *     - member access;
 *     - literals;
 *     - lambdas;
 *     - closures;
 *     - quantum expressions;
 *     - effect expressions;
 *     - macro expressions;
 *     - metaprogramming expressions.
 *
 * A future expression form automatically becomes block-capable once it enters
 * the canonical expression composition.
 */


/*
 * ============================================================================
 * DECLARATION INTEGRATION
 * ============================================================================
 *
 * Declarations appearing inside blocks enter through the canonical `statement`
 * composition when Zamani treats declarations as statement-position constructs.
 *
 * This file therefore MUST NOT introduce:
 *
 *     blockDeclaration
 *     blockVariable
 *     blockFunction
 *     blockQuantumDeclaration
 *     blockHardwareDeclaration
 *
 * merely to specialize block contents.
 *
 * Such duplication would create multiple AST paths for the same source
 * construct.
 */


/*
 * ============================================================================
 * DOMAIN NEUTRALITY
 * ============================================================================
 *
 * A block does not become a different syntactic object merely because its
 * contents belong to a different computational domain.
 *
 * The same block structure is therefore used for:
 *
 *     classical:
 *
 *         {
 *             compute();
 *         }
 *
 *     quantum:
 *
 *         {
 *             prepare();
 *             operation();
 *             measure();
 *         }
 *
 *     hybrid:
 *
 *         {
 *             classical_compute();
 *             quantum_compute();
 *             classical_decision();
 *         }
 *
 *     HDL:
 *
 *         {
 *             hardware_intent();
 *         }
 *
 *     distributed:
 *
 *         {
 *             distribute();
 *             synchronize();
 *         }
 *
 *     AI:
 *
 *         {
 *             train();
 *             infer();
 *         }
 *
 * The block grammar does not need to know the domain.
 */


/*
 * ============================================================================
 * QUANTUM INTEGRATION CONTRACT
 * ============================================================================
 *
 * Quantum constructs may appear inside blocks through the canonical statement
 * and expression grammar.
 *
 * This file MUST NOT know:
 *
 *     - qubit count;
 *     - logical-qubit count;
 *     - physical-qubit count;
 *     - gate inventory;
 *     - topology;
 *     - device identifiers;
 *     - QPU count;
 *     - calibration;
 *     - noise model;
 *     - QEC implementation;
 *     - routing;
 *     - scheduling.
 *
 * The downstream quantum path remains:
 *
 *     block AST
 *         |
 *         v
 *     semantic analysis
 *         |
 *         v
 *     canonical quantum semantics
 *         |
 *         v
 *     quantum::ir
 *         |
 *         v
 *     optimization
 *         |
 *         v
 *     routing / scheduling
 *         |
 *         v
 *     QEC / ZQN / resilience
 *         |
 *         v
 *     HAL / target realization
 *
 * `grammar/core/blocks.g4` has no dependency on `quantum::ir`.
 */


/*
 * ============================================================================
 * HDL / HARDWARE INTEGRATION CONTRACT
 * ============================================================================
 *
 * Hardware and HDL constructs may appear inside blocks through their own
 * grammar/domain statement and expression owners.
 *
 * This file does NOT define:
 *
 *     cpuBlock
 *     gpuBlock
 *     fpgaBlock
 *     qpuBlock
 *     asicBlock
 *     acceleratorBlock
 *     hardwareBlock
 *
 * A physical implementation is not a new kind of source block merely because
 * its eventual target differs.
 *
 * Hardware intent remains semantic input to downstream hardware/compiler
 * layers.
 */


/*
 * ============================================================================
 * CONCURRENCY / PARALLELISM CONTRACT
 * ============================================================================
 *
 * Blocks may contain concurrency and parallelism constructs.
 *
 * The grammar imposes no limits on:
 *
 *     - task count;
 *     - process count;
 *     - actor count;
 *     - channel count;
 *     - worker count;
 *     - parallel regions;
 *     - nesting.
 *
 * These are determined by program semantics and available implementation
 * resources.
 *
 * The block grammar does not select:
 *
 *     - CPU;
 *     - core;
 *     - thread;
 *     - GPU;
 *     - accelerator;
 *     - node.
 */


/*
 * ============================================================================
 * DISTRIBUTED COMPUTING CONTRACT
 * ============================================================================
 *
 * A distributed block is still a normal block.
 *
 * The block grammar imposes no fixed:
 *
 *     - node count;
 *     - process count;
 *     - service count;
 *     - endpoint count;
 *     - topology size.
 *
 * Placement and deployment are downstream concerns.
 */


/*
 * ============================================================================
 * AI / DATA CONTRACT
 * ============================================================================
 *
 * AI, ML, tensor, dataflow, query, inference and training constructs may occur
 * inside ordinary blocks.
 *
 * The block grammar does not encode:
 *
 *     - tensor dimensions;
 *     - model size;
 *     - accelerator count;
 *     - dataset size;
 *     - parameter count;
 *     - memory capacity.
 *
 * Such properties are semantic program values, requirements, capabilities,
 * constraints, resources or implementation decisions owned elsewhere.
 */


/*
 * ============================================================================
 * RESOURCE / CAPABILITY CONTRACT
 * ============================================================================
 *
 * A block may contain constructs expressing:
 *
 *     requirements;
 *     capabilities;
 *     constraints;
 *     preferences;
 *     hints;
 *     resource intent.
 *
 * This grammar does not determine whether those requirements can be satisfied.
 *
 * Important distinction:
 *
 *     syntax
 *         !=
 *     resource allocation
 *
 *     syntax
 *         !=
 *     hardware discovery
 *
 *     syntax
 *         !=
 *     scheduling
 *
 * A block remains target-independent.
 */


/*
 * ============================================================================
 * POCO-REAF / SCALABILITY CONTRACT
 * ============================================================================
 *
 * Zamani:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * requires source structure to remain independent of accidental machine size.
 *
 * This grammar therefore contains NO language-level maximum for:
 *
 *     - block elements;
 *     - statements;
 *     - expressions;
 *     - nested blocks;
 *     - functions;
 *     - resources;
 *     - qubits;
 *     - logical qubits;
 *     - physical qubits;
 *     - CPUs;
 *     - cores;
 *     - threads;
 *     - GPUs;
 *     - FPGAs;
 *     - ASICs;
 *     - accelerators;
 *     - nodes;
 *     - devices;
 *     - memory;
 *     - registers;
 *     - tensor dimensions;
 *     - vector widths;
 *     - timelines;
 *     - network links.
 *
 * Deliberately absent:
 *
 *     MAX_BLOCK_ELEMENTS
 *     MAX_STATEMENTS
 *     MAX_NESTING
 *     MAX_QUBITS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_DEVICES
 *     MAX_MEMORY
 *
 * ANTLR repetition operators are used instead of artificial numeric bounds.
 *
 * Practical implementation limits, when required for parser safety or denial
 * of service protection, belong to explicit configurable compiler/parser
 * resource policy and MUST NOT become language semantics.
 */


/*
 * ============================================================================
 * SOURCE ORDER / DETERMINISM
 * ============================================================================
 *
 * Given the same:
 *
 *     - source;
 *     - language version;
 *     - lexer version;
 *     - grammar version;
 *     - parser configuration;
 *
 * block parsing must produce deterministic syntactic structure.
 *
 * Parsing must not depend upon:
 *
 *     - CPU availability;
 *     - GPU availability;
 *     - QPU availability;
 *     - hardware topology;
 *     - calibration;
 *     - scheduler state;
 *     - network state;
 *     - runtime state;
 *     - backend availability.
 *
 * This grammar contains no runtime-dependent semantic predicate.
 */


/*
 * ============================================================================
 * SECURITY BOUNDARY
 * ============================================================================
 *
 * Parsing is non-executing.
 *
 * A source construct inside a block is syntax until a downstream compiler or
 * runtime subsystem explicitly interprets it.
 *
 * Parsing MUST NOT:
 *
 *     - execute commands;
 *     - spawn processes;
 *     - open files;
 *     - access credentials;
 *     - access environment variables;
 *     - contact networks;
 *     - discover hardware;
 *     - invoke a compiler;
 *     - invoke a backend;
 *     - allocate runtime resources.
 *
 * This applies equally to ordinary, quantum, HDL, hardware, networking,
 * security, AI and future-domain statements.
 */


/*
 * ============================================================================
 * SOURCE-SPAN CONTRACT
 * ============================================================================
 *
 * The frontend parser must preserve the complete source span of:
 *
 *     blockExpression
 *
 * including:
 *
 *     opening delimiter;
 *     every block element;
 *     closing delimiter.
 *
 * Each child NodeId must preserve its own source span.
 *
 * The grammar does not construct Rust Span values; it only establishes the
 * parser boundary from which the frontend obtains those spans.
 *
 * This is required for:
 *
 *     - diagnostics;
 *     - IDE tooling;
 *     - source mapping;
 *     - provenance;
 *     - formatting;
 *     - refactoring;
 *     - deterministic AST serialization.
 */


/*
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The existing frontend AST contains:
 *
 *     BlockExpression
 *
 * with an ordered sequence of:
 *
 *     Vec<NodeId>
 *
 * Therefore the parser integration contract is:
 *
 *     blockExpression
 *         |
 *         v
 *     BlockExpression
 *         |
 *         +--> ordered NodeId children
 *
 * The parser MUST:
 *
 *     1. recognize LBRACE;
 *     2. parse zero or more blockElement nodes;
 *     3. preserve exact source order;
 *     4. recognize RBRACE;
 *     5. create the BlockExpression AST node;
 *     6. append child NodeIds in source order;
 *     7. preserve the complete source span;
 *     8. perform no semantic interpretation.
 *
 * The AST graph owns the actual child nodes.
 *
 * The BlockExpression owns only their ordered relationships.
 *
 * This matches the repository's existing BlockExpression architecture. 
 *
 * No second block AST type should be introduced.
 */


/*
 * ============================================================================
 * AST SEMANTIC BOUNDARY
 * ============================================================================
 *
 * These layers must remain distinct:
 *
 *     blockExpression
 *         =
 *     parser syntax
 *
 *     BlockExpression
 *         =
 *     source AST structure
 *
 *     semantic block
 *         =
 *     resolved language meaning
 *
 *     ZUIR/control-flow region
 *         =
 *     universal semantic representation
 *
 *     classical IR block
 *         =
 *     classical implementation representation
 *
 *     quantum::ir block
 *         =
 *     canonical quantum representation
 *
 *     target/backend block
 *         =
 *     physical realization
 *
 * This file defines only parser syntax.
 */


/*
 * ============================================================================
 * SEMANTIC INTEGRATION CONTRACT
 * ============================================================================
 *
 * After parsing, semantic analysis determines:
 *
 *     - lexical scope;
 *     - name resolution;
 *     - reachability;
 *     - control-flow exits;
 *     - type of the block;
 *     - final-value semantics;
 *     - effect propagation;
 *     - ownership;
 *     - borrowing;
 *     - capability requirements;
 *     - resource requirements;
 *     - deterministic behavior;
 *     - domain-specific validity.
 *
 * In particular, semantic analysis determines whether an expression occurring
 * in a block is:
 *
 *     - a valid expression statement;
 *     - the block's value-producing expression;
 *     - unreachable;
 *     - invalid in the current context.
 *
 * The parser must not attempt those decisions.
 */


/*
 * ============================================================================
 * CONTROL-FLOW CONTRACT
 * ============================================================================
 *
 * The block grammar does not decide whether:
 *
 *     break
 *
 * is inside a loop, whether:
 *
 *     continue
 *
 * is inside an iteration context, or whether:
 *
 *     return
 *
 * is inside a callable.
 *
 * Those are semantic/control-flow validations.
 *
 * Similarly, the grammar does not determine whether all paths through a block
 * produce a value.
 */


/*
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * A block may contain effectful or effect-free constructs.
 *
 * The grammar does not infer effects.
 *
 * Effect analysis belongs to:
 *
 *     grammar/effects/
 *     semantic analysis
 *
 * A block's syntax therefore remains independent of:
 *
 *     - I/O effects;
 *     - network effects;
 *     - mutation;
 *     - allocation;
 *     - quantum effects;
 *     - hardware effects;
 *     - concurrency effects.
 */


/*
 * ============================================================================
 * RESOURCE / LIFETIME CONTRACT
 * ============================================================================
 *
 * Blocks can provide lexical scope and therefore participate in:
 *
 *     - lifetime analysis;
 *     - ownership analysis;
 *     - borrowing;
 *     - region analysis;
 *     - resource release;
 *     - effect scoping.
 *
 * This grammar establishes only the source boundary.
 *
 * It does not itself release, allocate, reserve or discover resources.
 */


/*
 * ============================================================================
 * ERROR CONTRACT
 * ============================================================================
 *
 * Syntax diagnostics belong to the parser/frontend diagnostic layer.
 *
 * Examples:
 *
 *     missing LBRACE
 *     missing RBRACE
 *     malformed block element
 *     unexpected token
 *     malformed nested construct
 *
 * Semantic diagnostics belong downstream.
 *
 * Examples:
 *
 *     invalid return context
 *     invalid break context
 *     invalid expression result
 *     type mismatch
 *     unavailable capability
 *     impossible resource requirement
 *     illegal quantum operation
 *     invalid hardware intent
 *
 * This grammar MUST NOT silently reinterpret malformed syntax as another
 * language construct merely to avoid a syntax error.
 */


/*
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing repository syntax already uses:
 *
 *     block
 *     blockExpression
 *
 * in multiple grammar domains.
 *
 * This file therefore deliberately preserves both public parser rule names:
 *
 *     block
 *     blockExpression
 *
 * `block` is a compatibility wrapper around the canonical block-expression
 * structure.
 *
 * This avoids unnecessarily renaming existing grammar references.
 *
 * Existing references to:
 *
 *     blockExpression
 *
 * remain valid.
 *
 * Existing references to:
 *
 *     block
 *
 * remain valid.
 *
 * No new domain-specific block names are required.
 */


/*
 * ============================================================================
 * MIGRATION / SINGLE-AUTHORITY CONTRACT
 * ============================================================================
 *
 * The repository currently contains:
 *
 *     grammar/statements/blocks.g4
 *
 * which historically/currently owns a blockExpression rule.
 *
 * Once this file becomes the authoritative core block component, the assembled
 * production grammar MUST have exactly ONE effective implementation of:
 *
 *     block
 *     blockExpression
 *     blockElement
 *
 * The existing statements/blocks.g4 must therefore be treated as a migration
 * compatibility surface and MUST NOT be simultaneously composed as another
 * implementation of those rules.
 *
 * This file itself does NOT depend on the old statements/blocks.g4.
 *
 * The migration direction is:
 *
 *     OLD:
 *
 *         statements/blocks.g4
 *             |
 *             v
 *         blockExpression
 *
 *     FINAL:
 *
 *         core/blocks.g4
 *             |
 *             +--> block
 *             |
 *             +--> blockExpression
 *             |
 *             +--> blockElement
 *
 *         statements/statements.g4
 *             |
 *             +--> statement
 *             |
 *             +--> ...
 *
 * The dependency is therefore:
 *
 *     core/blocks
 *           |
 *           +--> statement
 *           +--> expression
 *
 * and not:
 *
 *     core/blocks
 *           <-->
 *     statements/blocks
 *
 * This prevents a circular ownership model.
 */


/*
 * ============================================================================
 * ZAMANI.G4 INTEGRATION
 * ============================================================================
 *
 * The root grammar:
 *
 *     grammar/Zamani.g4
 *
 * remains the canonical composition root.
 *
 * It should ultimately resolve:
 *
 *     block
 *         -> core/blocks.g4:block
 *
 *     blockExpression
 *         -> core/blocks.g4:blockExpression
 *
 * and should not maintain a competing hand-written block implementation.
 *
 * Existing root-level constructs such as:
 *
 *     functionDeclaration
 *     control-flow statements
 *     module bodies
 *     implementation bodies
 *     domain bodies
 *
 * may continue referring to `block`.
 *
 * Existing expression-oriented constructs may continue referring to
 * `blockExpression`.
 *
 * This allows integration without renaming those existing constructs.
 */


/*
 * ============================================================================
 * COMPOSITION GRAPH
 * ============================================================================
 *
 * Final conceptual grammar relationship:
 *
 *     grammar/Zamani.g4
 *             |
 *             +--------------------+
 *             |                    |
 *             v                    v
 *     statement composition   expression composition
 *             |                    |
 *             +---------+----------+
 *                       |
 *                       v
 *                 core/blocks.g4
 *                       |
 *                 +-----+-----+
 *                 |           |
 *                 v           v
 *             statement   expression
 *                 \           /
 *                  \         /
 *                   v       v
 *                 blockElement
 *                       |
 *                       v
 *                blockExpression
 *                       |
 *                       v
 *                   block
 *
 * The parser therefore has one universal block structure shared by every
 * computational domain.
 */


/*
 * ============================================================================
 * DOMAIN INTEGRATION
 * ============================================================================
 *
 * No domain grammar needs to define another generic block merely because it
 * needs braces.
 *
 * Domain grammars should use the canonical:
 *
 *     block
 *
 * or:
 *
 *     blockExpression
 *
 * when their semantics permit it.
 *
 * Domain-specific bodies that genuinely have different syntax may retain
 * domain-specific names, but those names must represent genuinely different
 * source structures rather than aliases for the universal block.
 *
 * Examples that MUST NOT be created merely for hardware specialization:
 *
 *     cpuBlock
 *     gpuBlock
 *     fpgaBlock
 *     qpuBlock
 *     acceleratorBlock
 *
 * The same universal block syntax is preferred wherever semantics permit it.
 */


/*
 * ============================================================================
 * FUTURE-DOMAIN EXTENSIBILITY
 * ============================================================================
 *
 * Future domains may reuse the universal block without modifying this file.
 *
 * Examples:
 *
 *     photonic
 *     neuromorphic
 *     optical
 *     molecular
 *     biological
 *     analog
 *     reversible
 *     memristive
 *     future registered domains
 *
 * A new domain should normally add its own statement/expression/declaration
 * grammar and integrate those rules into the canonical statement/expression
 * composition.
 *
 * `core/blocks.g4` should remain unchanged.
 */


/*
 * ============================================================================
 * NO HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no language-level constants for:
 *
 *     - block size;
 *     - statement count;
 *     - nesting depth;
 *     - expression count;
 *     - qubits;
 *     - logical qubits;
 *     - physical qubits;
 *     - CPUs;
 *     - cores;
 *     - threads;
 *     - GPUs;
 *     - FPGAs;
 *     - ASICs;
 *     - accelerators;
 *     - nodes;
 *     - devices;
 *     - memory;
 *     - registers;
 *     - vector widths;
 *     - tensor dimensions;
 *     - network size;
 *     - timelines.
 *
 * All repetition is open-ended through ANTLR repetition operators.
 *
 * This is required for:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * and for scaling from very small programs to programs whose size is bounded
 * only by available implementation resources.
 */


/*
 * ============================================================================
 * RUST 1.97 / 1.97.1 CONTRACT
 * ============================================================================
 *
 * This grammar requires no Rust implementation code.
 *
 * The downstream Zamani frontend/compiler implementation MUST remain compatible
 * with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021 edition
 *
 * and must use safe Rust.
 *
 * The grammar does not require target-specific Rust extensions.
 *
 * The generated parser is an implementation artifact and is not the canonical
 * semantic model.
 */


/*
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * The corresponding grammar tests should cover at least:
 *
 * POSITIVE:
 *
 *     {}
 *
 *     {
 *         expression;
 *     }
 *
 *     {
 *         declaration;
 *         expression;
 *     }
 *
 *     {
 *         if condition {
 *             work();
 *         }
 *     }
 *
 *     {
 *         while condition {
 *             work();
 *         }
 *     }
 *
 *     {
 *         nested_block();
 *         {
 *             nested();
 *         }
 *     }
 *
 *     let value = {
 *         compute();
 *         value;
 *     };
 *
 * QUANTUM:
 *
 *     {
 *         quantum_operation();
 *         measurement();
 *     }
 *
 * HYBRID:
 *
 *     {
 *         classical_operation();
 *         quantum_operation();
 *         classical_operation();
 *     }
 *
 * HDL:
 *
 *     {
 *         hardware_intent();
 *     }
 *
 * DISTRIBUTED:
 *
 *     {
 *         distributed_operation();
 *         synchronize();
 *     }
 *
 * NEGATIVE:
 *
 *     - missing opening brace;
 *     - missing closing brace;
 *     - malformed block element;
 *     - malformed nested construct;
 *     - unexpected token after block.
 *
 * BOUNDARY:
 *
 *     - empty block;
 *     - one element;
 *     - deeply nested blocks;
 *     - block containing many elements;
 *     - block ending in an expression;
 *     - block ending in a terminating statement;
 *     - block with unreachable trailing source.
 *
 * SCALABILITY:
 *
 *     - arbitrarily many block elements;
 *     - arbitrarily nested blocks within implementation/resource limits;
 *     - large mixed-domain blocks;
 *     - large quantum/classical hybrid blocks;
 *     - large HDL/software co-design blocks.
 *
 * DETERMINISM:
 *
 *     Identical token streams MUST produce identical parse structure.
 *
 * COMPATIBILITY:
 *
 *     Existing uses of:
 *
 *         block
 *         blockExpression
 *
 * must remain representable without renaming.
 */


/*
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 * [x] block is defined;
 * [x] blockExpression is defined;
 * [x] blockElement is defined;
 * [x] empty blocks are syntactically representable;
 * [x] nested blocks are representable;
 * [x] source order is preserved;
 * [x] statement syntax is delegated;
 * [x] expression syntax is delegated;
 * [x] no expression precedence is duplicated;
 * [x] no statement grammar is duplicated;
 * [x] no lexer rules are duplicated;
 * [x] source spans remain an AST/frontend responsibility;
 * [x] block AST maps to the existing BlockExpression model;
 * [x] no second block AST is introduced;
 * [x] no IR is constructed here;
 * [x] quantum::ir remains downstream;
 * [x] QEC remains downstream;
 * [x] ZQN remains downstream;
 * [x] routing remains downstream;
 * [x] scheduling remains downstream;
 * [x] hardware realization remains downstream;
 * [x] no machine limits are encoded;
 * [x] no resource capacities are encoded;
 * [x] no physical topology is encoded;
 * [x] no device identifiers are required;
 * [x] no target-specific block type is introduced;
 * [x] POCO-REAF is preserved;
 * [x] future domains can reuse the block structure;
 * [x] existing `block` and `blockExpression` names are retained;
 * [x] the file contains no embedded Rust implementation;
 * [x] the file requires no unsafe implementation;
 * [x] parser behavior is deterministic;
 * [x] parsing has no external side effects.
 *
 * The remaining integration work is composition-level work in the canonical
 * root/statement/expression grammar. This file itself does not need to be
 * modified merely because another domain grammar is subsequently added.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * PARSER GRAMMAR IDENTITY
 * ============================================================================
 *
 * This component is intentionally a parser grammar.
 *
 * The final composition root supplies the complete statement and expression
 * rule implementations.
 *
 * The component therefore does not import `Statements` here because doing so
 * would create the wrong ownership direction:
 *
 *     Statements -> Blocks
 *
 * is already the established composition relationship.
 *
 * Instead, this component exposes the block contract and the canonical
 * composition root resolves `statement` and `expression`.
 *
 * ============================================================================
 */

parser grammar ZamaniCoreBlocks;

options {
    tokenVocab = ZamaniLexer;
}
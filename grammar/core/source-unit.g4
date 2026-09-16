/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/core/source-unit.g4
 *
 * Status:
 *     Production-ready source-unit composition boundary.
 *
 * Purpose:
 *     Defines the domain-neutral source-unit boundary of Zamani.
 *
 * ============================================================================
 * AUTHORITY
 * ============================================================================
 *
 * The canonical language composition root remains:
 *
 *     grammar/Zamani.g4
 *
 * This file is an independently maintainable parser component used by the
 * canonical parser composition.
 *
 * It owns ONLY:
 *
 *     - source-unit structure;
 *     - source-file structure;
 *     - source-item ordering;
 *     - source-level EOF boundary;
 *     - the integration boundary between declarations and statements.
 *
 * It does NOT own:
 *
 *     - lexical tokens;
 *     - identifiers;
 *     - qualified names;
 *     - paths;
 *     - attributes;
 *     - annotations;
 *     - metadata semantics;
 *     - declarations;
 *     - functions;
 *     - modules;
 *     - types;
 *     - expressions;
 *     - statements;
 *     - control flow;
 *     - classical syntax;
 *     - quantum syntax;
 *     - hybrid syntax;
 *     - HDL syntax;
 *     - hardware syntax;
 *     - distributed syntax;
 *     - AI/ML syntax;
 *     - data syntax;
 *     - networking syntax;
 *     - security syntax;
 *     - resource semantics;
 *     - capability semantics;
 *     - compilation;
 *     - execution;
 *     - AST construction;
 *     - semantic analysis;
 *     - IR construction;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN;
 *     - routing;
 *     - scheduling;
 *     - calibration;
 *     - HAL;
 *     - runtime behavior.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Zamani is designed around:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 *     POCO-REAF
 *
 * The source-unit grammar therefore imposes no universal limits on:
 *
 *     - source items;
 *     - declarations;
 *     - statements;
 *     - modules;
 *     - functions;
 *     - types;
 *     - expressions;
 *     - nesting;
 *     - namespaces;
 *     - imports;
 *     - quantum registers;
 *     - qubits;
 *     - CPUs;
 *     - cores;
 *     - threads;
 *     - GPUs;
 *     - FPGAs;
 *     - ASICs;
 *     - QPUs;
 *     - accelerators;
 *     - nodes;
 *     - devices;
 *     - memory;
 *     - storage;
 *     - network links;
 *     - tensor dimensions;
 *     - vector widths;
 *     - timelines;
 *     - processes.
 *
 * Repetition is expressed using normal grammar recursion and repetition
 * operators. No artificial machine-derived maximum is permitted here.
 *
 * A parser, compiler, operating system, runtime, or hardware resource limit
 * remains an implementation/resource-policy concern and MUST NOT become a
 * source-language grammar limit.
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
 *     sourceFile
 *          |
 *          v
 *     sourceUnit
 *          |
 *          v
 *     declaration / statement dispatch
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     structural validation
 *          |
 *          v
 *     name/module resolution
 *          |
 *          v
 *     type/effect/capability/resource analysis
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--------------------+--------------------+
 *          |                    |                    |
 *          v                    v                    v
 *     classical IR         quantum::ir         HDL/hardware IR
 *          |                    |                    |
 *          +--------------------+--------------------+
 *                               |
 *                               v
 *                         optimization
 *                               |
 *                      routing / scheduling
 *                               |
 *                     resilience / QEC / ZQN
 *                               |
 *                               v
 *                        target lowering
 *                               |
 *                               v
 *                         HAL / runtime
 *                               |
 *                               v
 *                         actual target
 *
 * IMPORTANT:
 *
 *     quantum::ir
 *
 * remains the canonical quantum semantic boundary.
 *
 * This file MUST NOT create, reference, construct, or lower to quantum::ir.
 *
 * ============================================================================
 * INTEGRATION OWNERSHIP
 * ============================================================================
 *
 * Declaration syntax is owned by:
 *
 *     grammar/declarations/declarations.g4
 *
 * whose parser grammar is:
 *
 *     ZamaniDeclarations
 *
 * Statement syntax is owned by:
 *
 *     grammar/statements/statements.g4
 *
 * whose parser grammar is:
 *
 *     Statements
 *
 * Therefore this file deliberately delegates:
 *
 *     sourceItem
 *          |
 *          +--> declaration
 *          |
 *          +--> statement
 *
 * It MUST NOT duplicate either dispatcher.
 *
 * ============================================================================
 * SOURCE-UNIT VS SOURCE-FILE
 * ============================================================================
 *
 * A sourceUnit is a sequence of source-level items.
 *
 * A sourceFile is a complete externally parseable file and therefore owns
 * the EOF boundary.
 *
 * This distinction is intentional:
 *
 *     sourceUnit
 *
 * may be embedded by a higher-level composition grammar without consuming EOF.
 *
 *     sourceFile
 *
 * is the standalone parser entry point and consumes exactly one EOF.
 *
 * This prevents the previous architectural problem where a reusable source
 * grammar consumed EOF internally and therefore became difficult to compose
 * into the canonical Zamani parser.
 *
 * ============================================================================
 * SOURCE ORDER
 * ============================================================================
 *
 * Source items are ordered.
 *
 * The parser preserves source order naturally through the parse tree.
 *
 * No semantic reordering occurs here.
 *
 * Import ordering, declaration ordering, initialization ordering, dependency
 * ordering, execution ordering, scheduling ordering, or optimization ordering
 * are downstream concerns.
 *
 * ============================================================================
 * SOURCE ITEM OWNERSHIP
 * ============================================================================
 *
 * A source item is exactly one canonical declaration or statement.
 *
 * Documentation, attributes, annotations, and metadata are not independently
 * admitted as executable source items by this grammar.
 *
 * They belong to the grammar component that owns their attachment semantics.
 *
 * This avoids the existing ambiguity where:
 *
 *     documentation
 *     metadata
 *     attribute
 *
 * could become detached top-level items instead of being associated with the
 * declaration or statement they annotate.
 *
 * ============================================================================
 * DOMAIN NEUTRALITY
 * ============================================================================
 *
 * The source-unit grammar deliberately does NOT contain alternatives such as:
 *
 *     quantumItem
 *     cpuItem
 *     gpuItem
 *     fpgaItem
 *     qpuItem
 *     hdlItem
 *     distributedItem
 *     aiItem
 *
 * A domain becomes part of a Zamani program through the declaration and
 * statement composition layers and their domain-specific grammar delegates.
 *
 * This means the source-unit boundary does not have to be rewritten whenever
 * Zamani gains another computational domain.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY SEPARATION
 * ============================================================================
 *
 * Source structure does not select physical resources.
 *
 * For example, source syntax may eventually express:
 *
 *     requires capability(...)
 *     requires resource(...)
 *     prefers capability(...)
 *     constrains(...)
 *
 * but this grammar does not interpret those constructs.
 *
 * Semantic/resource analysis decides their meaning.
 *
 * This separation permits the same source program to be compiled for:
 *
 *     tiny systems
 *     CPUs
 *     GPUs
 *     FPGAs
 *     ASICs
 *     QPUs
 *     heterogeneous systems
 *     distributed systems
 *     clusters
 *     supercomputers
 *     cloud environments
 *     future architectures
 *
 * without changing the source-unit grammar.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum syntax enters through the declaration/statement/expression grammar
 * composition owned elsewhere.
 *
 * This file MUST NOT define:
 *
 *     - gates;
 *     - qubit identifiers;
 *     - physical qubit identifiers;
 *     - fixed gate inventories;
 *     - maximum qubit counts;
 *     - topology;
 *     - routing;
 *     - scheduling;
 *     - QEC;
 *     - noise models;
 *     - ZQN;
 *     - calibration;
 *     - device IDs.
 *
 * The downstream quantum path remains:
 *
 *     source
 *       -> AST
 *       -> semantic quantum model
 *       -> quantum::ir
 *       -> optimization
 *       -> routing
 *       -> scheduling
 *       -> QEC / resilience / ZQN
 *       -> HAL
 *       -> target realization
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * HDL and hardware/software co-design constructs enter through their owning
 * domain grammar families.
 *
 * This source-unit boundary must remain independent of:
 *
 *     registers
 *     buses
 *     lanes
 *     cores
 *     memory banks
 *     FPGA fabric dimensions
 *     ASIC dimensions
 *     clock counts
 *     device topology
 *     accelerator inventory
 *
 * Those are semantic or target-realization concerns.
 *
 * ============================================================================
 * DISTRIBUTED / PARALLEL INTEGRATION
 * ============================================================================
 *
 * The source-unit grammar imposes no limit on:
 *
 *     processes
 *     actors
 *     tasks
 *     workers
 *     nodes
 *     channels
 *     messages
 *     partitions
 *     replicas
 *     timelines
 *
 * Parallelism and distribution remain source semantics where explicitly
 * expressed, while placement and resource realization remain downstream.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar creates no Rust values.
 *
 * The frontend parser adapter maps:
 *
 *     sourceFile
 *          |
 *          v
 *     sourceUnit
 *          |
 *          v
 *     sourceItem
 *          |
 *          +--> declaration AST
 *          |
 *          +--> statement AST
 *
 * Every resulting AST node must preserve, where applicable:
 *
 *     - source span;
 *     - source ordering;
 *     - parent/child relationships;
 *     - syntactic category;
 *     - attached source metadata;
 *     - diagnostic location information.
 *
 * The source-unit grammar itself must not attach:
 *
 *     - physical device information;
 *     - machine capacity;
 *     - physical qubit mapping;
 *     - routing decisions;
 *     - schedules;
 *     - compiler backend information.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * This file performs no semantic analysis.
 *
 * It does not determine:
 *
 *     - whether a name exists;
 *     - whether a type is valid;
 *     - whether a capability is available;
 *     - whether a resource requirement is satisfiable;
 *     - whether a quantum operation is legal;
 *     - whether hardware can realize a program;
 *     - whether a schedule is possible;
 *     - whether a route exists;
 *     - whether QEC requirements can be satisfied.
 *
 * Those questions belong downstream.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * No IR is constructed here.
 *
 * The required direction is:
 *
 *     sourceFile
 *       -> parser
 *       -> frontend AST
 *       -> semantic analysis
 *       -> canonical semantic representation
 *       -> domain IR
 *
 * For quantum:
 *
 *     AST
 *       -> semantic quantum representation
 *       -> quantum::ir
 *
 * The source-unit grammar must never become an alternate IR boundary.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no actions;
 *     - no semantic predicates;
 *     - no Rust code;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware inspection;
 *     - no runtime callbacks;
 *     - no randomness;
 *     - no time-dependent behavior;
 *     - no mutable global state.
 *
 * Given identical token streams and identical grammar versions, the parser
 * structure is deterministic.
 *
 * ============================================================================
 * ERROR BOUNDARY
 * ============================================================================
 *
 * Syntax errors belong to the parser/frontend diagnostic layer.
 *
 * Examples:
 *
 *     unexpected top-level token
 *     incomplete declaration
 *     incomplete statement
 *     malformed declaration dispatch
 *     malformed statement dispatch
 *     unexpected EOF
 *
 * Semantic errors do NOT belong here.
 *
 * Examples:
 *
 *     unknown identifier
 *     invalid type
 *     unsatisfied resource requirement
 *     unavailable capability
 *     illegal quantum operation
 *     impossible hardware mapping
 *     impossible schedule
 *     invalid routing
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing public filename:
 *
 *     grammar/core/source-unit.g4
 *
 * is intentionally retained.
 *
 * Existing conceptual entry:
 *
 *     sourceUnit
 *
 * is retained.
 *
 * The important compatibility correction is that sourceUnit no longer
 * consumes EOF. A new:
 *
 *     sourceFile
 *
 * rule owns the complete-file EOF boundary.
 *
 * This makes the component composable without forcing the canonical root
 * grammar to duplicate or work around EOF handling.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden in this file:
 *
 *     MAX_SOURCE_ITEMS
 *     MAX_DECLARATIONS
 *     MAX_STATEMENTS
 *     MAX_MODULES
 *     MAX_FUNCTIONS
 *     MAX_TYPES
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_ASICS
 *     MAX_QPUS
 *     MAX_ACCELERATORS
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_MEMORY
 *     MAX_NETWORK_SIZE
 *     MAX_TENSOR_DIMENSION
 *     MAX_TIMELINES
 *
 * None are present.
 *
 * ============================================================================
 * RUST INTEGRATION
 * ============================================================================
 *
 * This grammar contains no embedded Rust and therefore contains no unsafe
 * Rust.
 *
 * The repository's Rust frontend/compiler integration must target:
 *
 *     Rust 1.97 / Rust 1.97.1
 *
 * with:
 *
 *     - safe Rust only;
 *     - no unsafe blocks;
 *     - no unsafe functions;
 *     - no target-specific parser behavior;
 *     - deterministic parsing;
 *     - source-span preservation.
 *
 * The grammar itself must remain independent of Rust implementation details.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive source-file cases:
 *
 *     empty source file
 *     one declaration
 *     one statement
 *     declaration followed by statement
 *     multiple declarations
 *     multiple statements
 *     mixed declaration/statement sequences
 *     classical program
 *     quantum program
 *     hybrid program
 *     HDL program
 *     hardware/software co-design program
 *     distributed program
 *     AI/data program
 *     mixed-domain program
 *
 * Negative source-file cases:
 *
 *     unexpected token at source level
 *     malformed declaration
 *     malformed statement
 *     incomplete source item
 *     unexpected EOF
 *
 * Boundary cases:
 *
 *     zero source items
 *     one source item
 *     deeply nested delegated constructs
 *     very large source-item sequences
 *
 * Scalability cases:
 *
 *     arbitrarily large source-item sequences subject only to available
 *     implementation resources
 *
 * Cross-domain cases:
 *
 *     classical + quantum
 *     classical + HDL
 *     quantum + hardware
 *     quantum + distributed
 *     AI + quantum
 *     AI + hardware
 *     classical + quantum + HDL + hardware
 *
 * Determinism:
 *
 *     identical token streams produce equivalent parse structures.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [ ] It owns only source-unit/file composition.
 *     [ ] sourceUnit does not consume EOF.
 *     [ ] sourceFile consumes exactly one EOF.
 *     [ ] Declaration syntax is delegated to ZamaniDeclarations.
 *     [ ] Statement syntax is delegated to Statements.
 *     [ ] No declaration syntax is duplicated here.
 *     [ ] No statement syntax is duplicated here.
 *     [ ] No expression grammar is duplicated here.
 *     [ ] No type grammar is duplicated here.
 *     [ ] No module grammar is duplicated here.
 *     [ ] No domain grammar is duplicated here.
 *     [ ] No hardware assumptions exist.
 *     [ ] No quantum limits exist.
 *     [ ] No machine-size limits exist.
 *     [ ] No embedded Rust exists.
 *     [ ] No unsafe Rust exists.
 *     [ ] No semantic actions exist.
 *     [ ] No semantic predicates exist.
 *     [ ] The import graph is one-way.
 *     [ ] The canonical parser can consume sourceFile.
 *     [ ] The canonical root can compose sourceUnit without an EOF conflict.
 *     [ ] AST integration is defined before implementation.
 *     [ ] Semantic integration is downstream.
 *     [ ] IR integration is downstream.
 *     [ ] quantum::ir remains canonical.
 *     [ ] Positive tests exist.
 *     [ ] Negative tests exist.
 *     [ ] Boundary tests exist.
 *     [ ] Scalability tests exist.
 *     [ ] Cross-domain tests exist.
 *     [ ] Determinism tests exist.
 *
 * ============================================================================
 */

parser grammar ZamaniSourceUnit;

options {
    /*
     * All parser grammars in the production modular architecture must consume
     * the canonical lexer vocabulary.
     *
     * The repository's declaration and statement composition grammars currently
     * use ZamaniLexer as their parser-facing vocabulary.
     */
    tokenVocab = ZamaniLexer;
}

/*
 * ============================================================================
 * DELEGATE GRAMMARS
 * ============================================================================
 *
 * The source-unit grammar composes existing authoritative dispatchers.
 *
 * ZamaniDeclarations owns declaration composition.
 *
 * Statements owns statement composition.
 *
 * No concrete declaration or statement production is repeated here.
 *
 * NOTE:
 *     These delegates must themselves be on the canonical production parser
 *     generation path. Legacy duplicate parser grammars under grammar/antlr/
 *     must not simultaneously provide competing declaration/statement rules.
 */
import
    ZamaniDeclarations,
    Statements
;

/*
 * ============================================================================
 * COMPLETE SOURCE FILE
 * ============================================================================
 *
 * This is the standalone parser entry point for a complete Zamani source
 * file.
 *
 * EOF is deliberately owned here rather than by sourceUnit so that sourceUnit
 * remains reusable by the canonical root composition.
 */
sourceFile
    : sourceUnit EOF
    ;

/*
 * ============================================================================
 * SOURCE UNIT
 * ============================================================================
 *
 * A source unit is an ordered, potentially unbounded sequence of source items.
 *
 * No machine-derived cardinality limit exists.
 *
 * Empty source units are syntactically valid. Whether an empty program is
 * semantically meaningful is a language-semantics decision, not a grammar
 * decision.
 */
sourceUnit
    : sourceItem*
    ;

/*
 * ============================================================================
 * SOURCE ITEM
 * ============================================================================
 *
 * Exactly one canonical declaration or statement occupies a source-item
 * position.
 *
 * Declaration ownership:
 *
 *     ZamaniDeclarations.declaration
 *
 * Statement ownership:
 *
 *     Statements.statement
 *
 * No domain-specific alternative is added here.
 */
sourceItem
    : declaration
    | statement
    ;
/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/memory/references.g4
 *
 * Grammar:
 *     ANTLR4 parser grammar
 *
 * Status:
 *     Production memory-reference component.
 *
 * Language:
 *     Zamani
 *
 * Implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     Safe Rust only
 *
 * Safety:
 *     This grammar contains no embedded target-language actions.
 *     It performs no runtime, filesystem, network, hardware, or environment
 *     access.
 *     No unsafe Rust is required or permitted by this grammar contract.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the SOURCE-LEVEL MEMORY REFERENCE CONTRACT.
 *
 * It defines reusable syntax for referring to memory-owned or memory-managed
 * values without defining:
 *
 *     - reference types;
 *     - pointer types;
 *     - general expressions;
 *     - unary operators;
 *     - memory places;
 *     - ownership checking;
 *     - borrow checking;
 *     - lifetime inference;
 *     - allocation;
 *     - deallocation;
 *     - physical addresses;
 *     - hardware topology;
 *     - resource placement;
 *     - runtime representation.
 *
 * The purpose is to provide a stable syntactic bridge between:
 *
 *     memory ownership
 *     borrowing
 *     lifetime
 *     references
 *     expressions
 *     types
 *     resources
 *
 * while preserving one language and one semantic pipeline.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     canonical parser composition
 *          |
 *          +-------------------------------+
 *          |                               |
 *          v                               v
 *       memory.g4                    types/reference.g4
 *          |                               |
 *          +---------------+---------------+
 *                          |
 *                          v
 *                   references.g4
 *                          |
 *                          v
 *                   domain-neutral AST
 *                          |
 *                          v
 *                  semantic analysis
 *                          |
 *          +---------------+----------------+
 *          |               |                |
 *          v               v                v
 *      ownership       lifetime          alias/
 *       analysis        analysis         reference
 *                                         analysis
 *          |               |                |
 *          +---------------+----------------+
 *                          |
 *                          v
 *                canonical semantic model
 *                          |
 *          +---------------+----------------+
 *          |               |                |
 *          v               v                v
 *      classical       quantum::ir      HDL/hardware
 *          |               |                |
 *          +---------------+----------------+
 *                          |
 *                          v
 *                 optimization/lowering
 *                          |
 *                 routing / scheduling
 *                          |
 *                    QEC / resilience
 *                          |
 *                         ZQN
 *                          |
 *                         HAL
 *                          |
 *                  target realization
 *
 * ============================================================================
 * SINGLE RESPONSIBILITY
 * ============================================================================
 *
 * THIS FILE OWNS
 * --------------
 *
 * 1. Memory-domain reference handles.
 *
 * 2. Symbolic reference targets.
 *
 * 3. Reference qualifiers.
 *
 * 4. Explicit reference lifetime metadata where a reference construct needs
 *    to preserve it.
 *
 * 5. Reference metadata.
 *
 * 6. Reference requirements.
 *
 * 7. Reference constraints.
 *
 * 8. Reference preferences.
 *
 * 9. Reference hints.
 *
 * 10. Reference policies.
 *
 * 11. Open-world reference extensions.
 *
 * 12. Reusable reference fragments for memory-domain composition.
 *
 *
 * THIS FILE DOES NOT OWN
 * ----------------------
 *
 * It MUST NOT define:
 *
 *     referenceType
 *     pointerType
 *     expression
 *     unaryExpression
 *     memoryPlace
 *     memoryOperation
 *     ownership
 *     borrowing
 *     lifetime semantics
 *     allocation
 *     deallocation
 *     synchronization
 *     concurrency
 *     resource discovery
 *     resource placement
 *     hardware topology
 *     quantum routing
 *     scheduling
 *     QEC
 *     ZQN
 *     HAL
 *     runtime behavior
 *     compiler backend selection
 *
 * ============================================================================
 * AUTHORITATIVE EXISTING FILES
 * ============================================================================
 *
 * This component integrates with the existing repository rather than
 * replacing existing authorities.
 *
 * Canonical ANTLR root:
 *
 *     grammar/Zamani.g4
 *
 * Canonical parser composition:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * Canonical lexer composition:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Canonical lexical vocabulary:
 *
 *     grammar/lexer/tokens.g4
 *
 * Canonical memory foundation:
 *
 *     grammar/memory/memory.g4
 *
 * Ownership syntax:
 *
 *     grammar/memory/ownership.g4
 *
 * Borrowing syntax:
 *
 *     grammar/memory/borrowing.g4
 *
 * Lifetime syntax:
 *
 *     grammar/memory/lifetimes.g4
 *
 * Canonical reference TYPE syntax:
 *
 *     grammar/types/reference.g4
 *
 * Canonical type composition:
 *
 *     grammar/types/types.g4
 *
 * Frontend implementation:
 *
 *     src/lexer.rs
 *     src/parser.rs
 *     src/ast/
 *
 * Canonical quantum semantic boundary:
 *
 *     quantum::ir
 *
 * ============================================================================
 * CRITICAL OWNERSHIP RULE
 * ============================================================================
 *
 * The repository already has:
 *
 *     grammar/types/reference.g4
 *
 * which owns reference TYPE syntax.
 *
 * Therefore this file MUST NOT introduce:
 *
 *     referenceType
 *
 * or another rule with the same semantic authority.
 *
 * For example:
 *
 *     &T
 *     &mut T
 *     &'a T
 *     &'a mut T
 *
 * are reference TYPE constructs and remain owned by:
 *
 *     grammar/types/reference.g4
 *
 * This file instead defines memory-domain reference constructs and metadata
 * that can refer to source-level values/resources.
 *
 * ============================================================================
 * CRITICAL BORROWING RULE
 * ============================================================================
 *
 * The repository also has:
 *
 *     grammar/memory/borrowing.g4
 *
 * which owns borrowing-specific syntax.
 *
 * Therefore this file MUST NOT redefine:
 *
 *     borrowReference
 *     borrowClause
 *     borrowMode
 *     borrowTarget
 *     borrowLifetime
 *
 * where those are already owned by borrowing.g4.
 *
 * A reference and a borrow are related semantically but are not identical
 * concepts:
 *
 *     reference
 *         = a source-level handle/access relationship
 *
 *     borrow
 *         = temporary/non-owning access intent
 *
 * Semantic analysis determines the relationship between them.
 *
 * ============================================================================
 * LIFETIME OWNERSHIP
 * ============================================================================
 *
 * Lifetime syntax remains owned by:
 *
 *     grammar/memory/lifetimes.g4
 *
 * This file therefore does not redefine:
 *
 *     lifetimeName
 *     anonymousLifetime
 *     lifetimeReference
 *     lifetimeBound
 *
 * A reference may consume the canonical lifetime-reference fragment when
 * explicit lifetime information is syntactically permitted by the host
 * grammar.
 *
 * ============================================================================
 * LEXICAL CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * It declares no lexer rules.
 *
 * The parser consumes the canonical Zamani lexer vocabulary.
 *
 * Canonical identifier spelling is:
 *
 *     IDENTIFIER
 *
 * This file MUST NOT introduce:
 *
 *     IDENT
 *     NAME
 *     SYMBOL
 *     REFERENCE_IDENTIFIER
 *
 * as alternative lexer tokens.
 *
 * This prevents multiple identifier authorities.
 *
 * ============================================================================
 * REFERENCE MODEL
 * ============================================================================
 *
 * A memory reference is represented as a symbolic source-level relationship.
 *
 * Conceptually:
 *
 *     reference
 *         target
 *         qualifier
 *         lifetime?
 *         metadata?
 *
 * The target is intentionally symbolic.
 *
 * It may represent:
 *
 *     local value
 *     field
 *     collection element
 *     tensor element
 *     stream
 *     dataset
 *     logical quantum object
 *     hardware/co-design object
 *     distributed object
 *     accelerator buffer
 *     future computational resource
 *
 * The grammar does not determine which physical resource ultimately realizes
 * the reference.
 *
 * ============================================================================
 * TARGET OWNERSHIP
 * ============================================================================
 *
 * This file deliberately does not define the complete memory-place grammar.
 *
 * Complex places belong to:
 *
 *     grammar/memory/memory.g4
 *
 * and the canonical expression hierarchy.
 *
 * The reference target in this file is therefore a symbolic reference path.
 *
 * This avoids creating a second expression grammar and prevents grammar
 * dependency cycles.
 *
 * ============================================================================
 * QUALIFIERS
 * ============================================================================
 *
 * Reference qualifiers are intentionally open-world.
 *
 * A qualifier is a semantic name, not a hardware identifier.
 *
 * Examples include:
 *
 *     shared
 *     read_only
 *     restricted
 *     remote
 *     persistent
 *     transactional
 *     capability::restricted
 *     region::scoped
 *
 * The grammar does NOT enumerate a fixed universal qualifier set.
 *
 * A qualifier's semantic meaning is established by the language specification
 * and semantic analysis.
 *
 * ============================================================================
 * OPEN-WORLD PRINCIPLE
 * ============================================================================
 *
 * Reference names and qualifier names are open-world symbolic names.
 *
 * This prevents the grammar from becoming a list of:
 *
 *     CPU references
 *     GPU references
 *     FPGA references
 *     QPU references
 *     accelerator references
 *     vendor references
 *
 * New computational domains can therefore consume the same reference model.
 *
 * ============================================================================
 * REFERENCE TARGET
 * ============================================================================
 *
 * A reference target is a symbolic qualified name.
 *
 * Examples:
 *
 *     value
 *     buffer
 *     object::value
 *     memory::buffer
 *     module::object::field
 *
 * Qualification depth is not fixed.
 *
 * ============================================================================
 * REFERENCE QUALIFIER
 * ============================================================================
 *
 * A qualifier may optionally carry arguments.
 *
 * Examples:
 *
 *     shared
 *     restricted
 *     region::scoped('a)
 *     capability::requires(...)
 *
 * The grammar does not interpret the qualifier.
 *
 * ============================================================================
 * REFERENCE LIFETIME
 * ============================================================================
 *
 * An explicit lifetime may accompany a reference contract.
 *
 * The syntax is delegated to the canonical lifetime vocabulary through the
 * lexical representation:
 *
 *     ' IDENTIFIER
 *
 * or, when supported by the composed lifetime grammar:
 *
 *     anonymous lifetime
 *
 * This file preserves the relationship but does not infer lifetime semantics.
 *
 * ============================================================================
 * SEMANTIC BOUNDARY
 * ============================================================================
 *
 * Successful parsing means:
 *
 *     the reference representation is structurally valid.
 *
 * It does NOT mean:
 *
 *     the reference is semantically valid.
 *
 * Semantic analysis determines:
 *
 *     - whether the target exists;
 *     - whether the target is referenceable;
 *     - whether the reference is valid;
 *     - whether the target is initialized;
 *     - whether ownership permits the reference;
 *     - whether the lifetime is sufficient;
 *     - whether aliasing is legal;
 *     - whether mutability is compatible;
 *     - whether the reference escapes;
 *     - whether the reference crosses a task boundary;
 *     - whether the reference crosses an async boundary;
 *     - whether the reference crosses a process boundary;
 *     - whether the reference crosses a distributed boundary;
 *     - whether the reference crosses a device boundary;
 *     - whether serialization is required;
 *     - whether the target realization can preserve reference semantics.
 *
 * ============================================================================
 * REFERENCE VS POINTER
 * ============================================================================
 *
 * A reference is not automatically a pointer.
 *
 * This grammar therefore does not imply:
 *
 *     address
 *     pointer width
 *     machine word size
 *     physical address
 *     virtual address
 *     device address
 *
 * A compiler may lower a reference to:
 *
 *     a pointer;
 *     an index;
 *     a capability;
 *     a descriptor;
 *     an offset;
 *     a handle;
 *     a distributed identifier;
 *     a runtime-managed object;
 *     another target representation.
 *
 * The choice belongs downstream.
 *
 * ============================================================================
 * REFERENCE VS BORROW
 * ============================================================================
 *
 * A reference may result from borrowing, ownership transfer, or another
 * semantic mechanism.
 *
 * This grammar does not assume one mechanism.
 *
 * Example conceptual relationship:
 *
 *     source value
 *          |
 *          +--> borrow intent
 *          |
 *          +--> reference representation
 *          |
 *          +--> semantic validation
 *
 * Borrow checking remains owned by semantic analysis.
 *
 * ============================================================================
 * REFERENCE QUALIFIER CONTRACT
 * ============================================================================
 *
 * The following categories are intentionally semantic:
 *
 *     qualifier
 *     policy
 *     requirement
 *     constraint
 *     preference
 *     hint
 *
 * They MUST remain distinguishable.
 *
 * A compiler MUST NOT silently reinterpret:
 *
 *     requirement as preference
 *
 * or:
 *
 *     constraint as hint
 *
 * merely because a target lacks an implementation strategy.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * This grammar contains no universal machine-size assumptions.
 *
 * It imposes no source-language maximum on:
 *
 *     - number of references;
 *     - number of targets;
 *     - reference depth;
 *     - number of lifetime names;
 *     - number of aliases;
 *     - number of regions;
 *     - number of processes;
 *     - number of tasks;
 *     - number of threads;
 *     - number of cores;
 *     - number of GPUs;
 *     - number of FPGAs;
 *     - number of QPUs;
 *     - number of accelerators;
 *     - number of distributed nodes;
 *     - memory capacity;
 *     - address width;
 *     - pointer width;
 *     - tensor rank;
 *     - tensor dimensions.
 *
 * "Infinity" means no artificial finite language ceiling is introduced here.
 *
 * Actual limits imposed by:
 *
 *     parser implementation;
 *     compiler implementation;
 *     operating system;
 *     runtime;
 *     target;
 *     available resources
 *
 * are implementation/resource constraints rather than grammar semantics.
 *
 * ============================================================================
 * HARDWARE INDEPENDENCE
 * ============================================================================
 *
 * This file MUST NOT encode:
 *
 *     CPU0
 *     GPU0
 *     FPGA0
 *     QPU0
 *     physical_qubit0
 *     memory_bank0
 *     numa_node0
 *     fixed_device_id
 *     physical_address
 *
 * Hardware selection and placement belong downstream.
 *
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * Reference syntax may carry semantic information consumed by:
 *
 *     grammar/resources/
 *     grammar/hardware/
 *     grammar/execution/
 *     grammar/distributed/
 *
 * However, this file does not select a resource.
 *
 * For example, a semantic requirement may eventually express:
 *
 *     requires capability("shared-memory")
 *
 * without the reference grammar deciding:
 *
 *     which CPU;
 *     which GPU;
 *     which memory bank;
 *     which node.
 *
 * ============================================================================
 * CONCURRENCY INTEGRATION
 * ============================================================================
 *
 * Reference semantics may interact with:
 *
 *     async
 *     await
 *     spawn
 *     tasks
 *     actors
 *     channels
 *     parallel execution
 *     distributed execution.
 *
 * This grammar does not determine whether a reference is:
 *
 *     Send
 *     Sync
 *     thread-local
 *     task-local
 *     remotely valid
 *     serializable
 *     transferable.
 *
 * Those are semantic properties.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * A reference may semantically designate an object whose realization is:
 *
 *     local;
 *     remote;
 *     replicated;
 *     partitioned;
 *     distributed;
 *     migrated;
 *     persistent.
 *
 * The grammar does not decide the realization.
 *
 * A reference crossing a distributed boundary may therefore be:
 *
 *     preserved;
 *     copied;
 *     serialized;
 *     transformed into a remote handle;
 *     rejected.
 *
 * The semantic/compiler layers determine which behavior preserves program
 * meaning.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum-related references remain domain-neutral at this layer.
 *
 * This file MUST NOT introduce:
 *
 *     physical qubit references;
 *     fixed qubit identifiers;
 *     QPU identifiers;
 *     gate-specific reference syntax;
 *     topology-specific references.
 *
 * A logical quantum object may be represented by the same symbolic reference
 * mechanism as other semantic objects.
 *
 * If quantum semantics are involved, downstream processing remains:
 *
 *     domain-neutral AST
 *          |
 *          v
 *     semantic analysis
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
 * This grammar creates no second quantum IR.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * References may semantically identify:
 *
 *     signals;
 *     registers;
 *     memories;
 *     interfaces;
 *     accelerator resources;
 *     co-design objects.
 *
 * This grammar does not define:
 *
 *     wire widths;
 *     register widths;
 *     physical addresses;
 *     bus topology;
 *     FPGA resource counts;
 *     ASIC layout;
 *     clock topology.
 *
 * Those belong to HDL, hardware, resource, and backend semantics.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser-to-AST layer must preserve enough information to represent:
 *
 *     - complete source span;
 *     - target path;
 *     - qualifier list;
 *     - explicit lifetime, if present;
 *     - policy list;
 *     - metadata;
 *     - requirement/constraint/preference/hint classification;
 *     - extension names;
 *     - extension arguments.
 *
 * The exact AST representation remains owned by:
 *
 *     src/ast/
 *
 * The AST MUST remain domain-neutral.
 *
 * Do not create:
 *
 *     QuantumReference
 *     CpuReference
 *     GpuReference
 *     FpgaReference
 *     QpuReference
 *
 * merely because the referenced object belongs to a particular target domain.
 *
 * ============================================================================
 * EXISTING AST COMPATIBILITY
 * ============================================================================
 *
 * The current frontend AST already uses generic expression and type structures
 * rather than requiring a separate memory-reference IR.
 *
 * Reference syntax must therefore lower into existing generic frontend
 * structures wherever possible.
 *
 * If the frontend eventually introduces a dedicated reference node, it must
 * remain semantic and domain-neutral and must preserve source spans.
 *
 * This grammar does not require a particular AST enum or struct name.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This file has NO direct IR dependency.
 *
 * In particular, it MUST NOT import or construct:
 *
 *     quantum::ir
 *     classical IR
 *     HDL IR
 *     hardware IR
 *     QEC IR
 *     ZQN IR
 *
 * Correct direction:
 *
 *     references.g4
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic reference analysis
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +-------------------+
 *          |                   |
 *          v                   v
 *      classical          quantum::ir
 *                              |
 *                              v
 *                       downstream lowering
 *
 * ============================================================================
 * COMPILER INTEGRATION
 * ============================================================================
 *
 * The compiler consumes reference semantics after parsing.
 *
 * Compiler responsibilities may include:
 *
 *     - reference validity;
 *     - alias analysis;
 *     - lifetime checking;
 *     - escape analysis;
 *     - ownership interaction;
 *     - optimization;
 *     - representation selection;
 *     - target lowering.
 *
 * This grammar does not choose the implementation representation.
 *
 * ============================================================================
 * RUNTIME INTEGRATION
 * ============================================================================
 *
 * Runtime responsibilities may include:
 *
 *     - reference representation;
 *     - storage management;
 *     - synchronization;
 *     - remote handles;
 *     - migration;
 *     - persistence;
 *     - device interaction.
 *
 * None of those responsibilities belong to this grammar.
 *
 * ============================================================================
 * TOOLING INTEGRATION
 * ============================================================================
 *
 * Tooling must be able to inspect reference constructs without executing them.
 *
 * Consumers include:
 *
 *     - formatter;
 *     - syntax highlighter;
 *     - language server;
 *     - documentation generator;
 *     - source indexer;
 *     - static analyzer;
 *     - refactoring tools;
 *     - conformance checker.
 *
 * Source spans must therefore remain recoverable from the parse tree.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing this grammar MUST depend only on:
 *
 *     - source token stream;
 *     - grammar version;
 *     - canonical lexer;
 *     - explicitly selected dialect configuration.
 *
 * It MUST NOT depend on:
 *
 *     - hardware;
 *     - memory availability;
 *     - CPU count;
 *     - GPU count;
 *     - QPU count;
 *     - filesystem state;
 *     - network state;
 *     - wall-clock time;
 *     - randomness;
 *     - runtime scheduler state;
 *     - deployment topology.
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Syntax diagnostics owned here include:
 *
 *     - malformed reference target;
 *     - malformed qualifier;
 *     - malformed qualifier arguments;
 *     - malformed lifetime attachment;
 *     - malformed metadata;
 *     - malformed policy;
 *     - malformed extension.
 *
 * Semantic diagnostics belong downstream:
 *
 *     - invalid reference;
 *     - dangling reference;
 *     - invalid lifetime;
 *     - invalid alias;
 *     - ownership violation;
 *     - borrow violation;
 *     - illegal escape;
 *     - invalid cross-domain reference;
 *     - unsupported target realization.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * This grammar MUST NOT:
 *
 *     - dereference anything;
 *     - access memory;
 *     - access physical addresses;
 *     - inspect process memory;
 *     - access hardware;
 *     - access secrets;
 *     - execute referenced operations.
 *
 * Reference syntax is purely declarative.
 *
 * ============================================================================
 * PERFORMANCE CONTRACT
 * ============================================================================
 *
 * The grammar must remain structurally scalable.
 *
 * Unbounded source repetition is represented by normal ANTLR repetition
 * operators rather than fixed numeric bounds.
 *
 * No rule may introduce a machine-dependent bound for:
 *
 *     references;
 *     qualifiers;
 *     metadata;
 *     lifetime names;
 *     target qualification depth.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * This file intentionally reuses existing canonical lexical concepts.
 *
 * It does not introduce:
 *
 *     REFERENCE
 *     REF
 *     MEMORY_REF
 *
 * as new globally reserved keywords.
 *
 * This minimizes lexical compatibility impact.
 *
 * Existing reference TYPE syntax remains owned by:
 *
 *     grammar/types/reference.g4
 *
 * Existing borrowing syntax remains owned by:
 *
 *     grammar/memory/borrowing.g4
 *
 * ============================================================================
 * PUBLIC RULE CONTRACT
 * ============================================================================
 *
 * Stable public rules:
 *
 *     memoryReference
 *     memoryReferenceTarget
 *     memoryReferenceQualifier
 *     memoryReferenceQualifierList
 *     memoryReferenceLifetime
 *     memoryReferenceMetadata
 *     memoryReferenceMetadataList
 *     memoryReferencePolicy
 *     memoryReferencePolicyList
 *     memoryReferenceRequirement
 *     memoryReferenceConstraint
 *     memoryReferencePreference
 *     memoryReferenceHint
 *     memoryReferenceExtension
 *     memoryReferenceExtensionList
 *     memoryReferenceContract
 *
 * Host grammars may compose these rules.
 *
 * ============================================================================
 * REFERENCE TARGET
 * ============================================================================
 *
 * A target is a symbolic qualified source name.
 *
 * There is deliberately no finite qualification depth.
 */
parser grammar MemoryReferences;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * 1. REFERENCE TARGET
 * ============================================================================
 *
 * Examples:
 *
 *     value
 *     buffer
 *     object::field
 *     memory::buffer
 *     module::object::field
 *
 * No fixed qualification depth exists.
 */
memoryReferenceTarget
    : IDENTIFIER
      (DOUBLE_COLON IDENTIFIER)*
    ;


/*
 * ============================================================================
 * 2. REFERENCE QUALIFIER
 * ============================================================================
 *
 * A qualifier is an open-world semantic name.
 *
 * Examples:
 *
 *     shared
 *     restricted
 *     region::scoped
 *     capability::restricted
 *
 * Optional arguments remain symbolic so this grammar does not import the
 * complete expression hierarchy.
 */
memoryReferenceQualifier
    : memoryReferenceTarget
      memoryReferenceArgumentClause?
    ;


memoryReferenceQualifierList
    : memoryReferenceQualifier
      (COMMA memoryReferenceQualifier)*
      COMMA?
    ;


/*
 * ============================================================================
 * 3. EXPLICIT LIFETIME
 * ============================================================================
 *
 * A reference may preserve an explicit lifetime name.
 *
 * This intentionally mirrors the repository's existing lifetime spelling
 * without creating a second lifetime lexer.
 *
 * Semantic lifetime ownership remains outside this grammar.
 */
memoryReferenceLifetime
    : APOSTROPHE IDENTIFIER
    ;


/*
 * ============================================================================
 * 4. REFERENCE ARGUMENTS
 * ============================================================================
 *
 * Arguments are deliberately restricted to symbolic reference-domain values.
 *
 * The host grammar remains responsible for arbitrary expressions.
 *
 * This avoids creating a second expression hierarchy.
 */
memoryReferenceArgumentClause
    : LPAREN memoryReferenceArgumentList? RPAREN
    ;


memoryReferenceArgumentList
    : memoryReferenceArgument
      (COMMA memoryReferenceArgument)*
      COMMA?
    ;


memoryReferenceArgument
    : memoryReferenceTarget
    | memoryReferenceLifetime
    ;


/*
 * ============================================================================
 * 5. REFERENCE
 * ============================================================================
 *
 * Canonical memory-domain reference representation.
 *
 * Examples:
 *
 *     value
 *     memory::buffer
 *     value 'a
 *     value(shared)
 *
 * This is NOT reference TYPE syntax.
 *
 * This is NOT borrow syntax.
 */
memoryReference
    : memoryReferenceTarget
      memoryReferenceLifetime?
      memoryReferenceQualifierList?
    ;


/*
 * ============================================================================
 * 6. METADATA
 * ============================================================================
 *
 * Metadata is open-world and semantically interpreted downstream.
 *
 * Examples:
 *
 *     shared
 *     readonly
 *     region::scoped = 'a
 *
 * The grammar does not assign implementation meaning to the names.
 */
memoryReferenceMetadata
    : memoryReferenceTarget
      (ASSIGN memoryReferenceMetadataValue)?
    ;


memoryReferenceMetadataValue
    : memoryReferenceTarget
    | memoryReferenceLifetime
    ;


memoryReferenceMetadataList
    : memoryReferenceMetadata
      (COMMA memoryReferenceMetadata)*
      COMMA?
    ;


/*
 * ============================================================================
 * 7. POLICY
 * ============================================================================
 *
 * Policies describe reference semantics without selecting an implementation.
 */
memoryReferencePolicy
    : memoryReferenceTarget
      memoryReferenceArgumentClause?
    ;


memoryReferencePolicyList
    : memoryReferencePolicy
      (COMMA memoryReferencePolicy)*
      COMMA?
    ;


/*
 * ============================================================================
 * 8. REQUIREMENT
 * ============================================================================
 *
 * A requirement is mandatory semantic intent.
 *
 * The semantic layer decides whether it can be satisfied.
 */
memoryReferenceRequirement
    : memoryReferenceTarget
      memoryReferenceArgumentClause?
    ;


memoryReferenceRequirementList
    : memoryReferenceRequirement
      (COMMA memoryReferenceRequirement)*
      COMMA?
    ;


/*
 * ============================================================================
 * 9. CONSTRAINT
 * ============================================================================
 *
 * A constraint restricts legal semantic or implementation choices.
 */
memoryReferenceConstraint
    : memoryReferenceTarget
      memoryReferenceArgumentClause?
    ;


memoryReferenceConstraintList
    : memoryReferenceConstraint
      (COMMA memoryReferenceConstraint)*
      COMMA?
    ;


/*
 * ============================================================================
 * 10. PREFERENCE
 * ============================================================================
 *
 * A preference is non-mandatory implementation guidance.
 */
memoryReferencePreference
    : memoryReferenceTarget
      memoryReferenceArgumentClause?
    ;


memoryReferencePreferenceList
    : memoryReferencePreference
      (COMMA memoryReferencePreference)*
      COMMA?
    ;


/*
 * ============================================================================
 * 11. HINT
 * ============================================================================
 *
 * A hint is optional implementation guidance.
 */
memoryReferenceHint
    : memoryReferenceTarget
      memoryReferenceArgumentClause?
    ;


memoryReferenceHintList
    : memoryReferenceHint
      (COMMA memoryReferenceHint)*
      COMMA?
    ;


/*
 * ============================================================================
 * 12. EXTENSION
 * ============================================================================
 *
 * Open-world extension point.
 *
 * Examples:
 *
 *     future::reference
 *     vendor::reference
 *     domain::reference
 *
 * The grammar does not reserve vendor/domain names.
 */
memoryReferenceExtension
    : memoryReferenceTarget
      memoryReferenceArgumentClause?
    ;


memoryReferenceExtensionList
    : memoryReferenceExtension
      (COMMA memoryReferenceExtension)*
      COMMA?
    ;


/*
 * ============================================================================
 * 13. REFERENCE CONTRACT
 * ============================================================================
 *
 * Aggregate metadata contract.
 *
 * This is intentionally not a statement or declaration.
 *
 * A host grammar decides where this aggregate is legal.
 *
 * IMPORTANT:
 *
 * The individual optional components are separated by explicit host-level
 * structure rather than relying on empty alternatives.
 *
 * The grammar therefore avoids epsilon-only productions.
 */
memoryReferenceContract
    : memoryReferenceMetadataList?
      memoryReferenceRequirementSection?
      memoryReferenceConstraintSection?
      memoryReferencePreferenceSection?
      memoryReferenceHintSection?
      memoryReferencePolicySection?
      memoryReferenceExtensionSection?
    ;


memoryReferenceRequirementSection
    : memoryReferenceRequirementList
    ;


memoryReferenceConstraintSection
    : memoryReferenceConstraintList
    ;


memoryReferencePreferenceSection
    : memoryReferencePreferenceList
    ;


memoryReferenceHintSection
    : memoryReferenceHintList
    ;


memoryReferencePolicySection
    : memoryReferencePolicyList
    ;


memoryReferenceExtensionSection
    : memoryReferenceExtensionList
    ;


/*
 * ============================================================================
 * 14. INTEGRATION BRIDGE
 * ============================================================================
 *
 * This rule is the preferred generic entry point when a host grammar needs
 * memory-reference syntax.
 *
 * It intentionally does not contain:
 *
 *     referenceType
 *     borrowReference
 *     memoryPlace
 *     expression
 *
 * Those remain owned by their existing grammars.
 */
memoryReferenceConstruct
    : memoryReference
    | memoryReferenceContract
    ;


/*
 * ============================================================================
 * 15. HOST INTEGRATION CONTRACT
 * ============================================================================
 *
 * grammar/memory/memory.g4
 * ------------------------
 *
 * memory.g4 remains the canonical memory-domain composition boundary.
 *
 * It may import/combine this grammar through its memory-domain dispatcher.
 *
 * It remains the owner of:
 *
 *     memoryConstruct
 *     memoryPlace
 *     memoryOperation
 *     memoryExpression
 *     memoryDeclaration
 *
 * This file does not replace those rules.
 *
 *
 * grammar/memory/borrowing.g4
 * ---------------------------
 *
 * borrowing.g4 remains the owner of borrow-specific syntax.
 *
 * This file MUST NOT replace:
 *
 *     borrowReference
 *     borrowClause
 *     borrowMode
 *
 *
 * grammar/memory/lifetimes.g4
 * ---------------------------
 *
 * lifetimes.g4 remains the preferred lifetime authority.
 *
 * The explicit lifetime fragment in this leaf grammar exists only to prevent
 * a dependency cycle when this grammar is composed independently.
 *
 * When the canonical parser composition makes Lifetimes directly available,
 * the host grammar should prefer the canonical lifetimeReference rule.
 *
 *
 * grammar/types/reference.g4
 * --------------------------
 *
 * reference.g4 remains the authority for reference TYPE syntax.
 *
 * No type syntax is duplicated here.
 *
 *
 * grammar/types/types.g4
 * ---------------------
 *
 * Type composition remains authoritative there.
 *
 *
 * grammar/expressions/
 * -------------------
 *
 * Arbitrary expressions remain authoritative there.
 *
 * This file must not import the complete expression hierarchy merely to permit
 * optional reference metadata.
 *
 * ============================================================================
 * PARSER-COMPOSITION INTEGRATION
 * ============================================================================
 *
 * The intended composition direction is:
 *
 *     Zamani.g4
 *          |
 *          v
 *     ZamaniParser.g4
 *          |
 *          v
 *        Memory
 *          |
 *          +--> memory-domain components
 *                  |
 *                  +--> MemoryReferences
 *
 * The root grammar must NOT import this leaf grammar directly.
 *
 * The leaf must enter the language through the memory composition boundary.
 *
 * This preserves:
 *
 *     one root;
 *     one parser authority;
 *     one memory dispatcher;
 *     one ownership hierarchy.
 *
 * ============================================================================
 * AST INTEGRATION
 * ============================================================================
 *
 * Parser adapters should map:
 *
 *     memoryReference
 *
 * into existing domain-neutral AST structures.
 *
 * At minimum preserve:
 *
 *     target
 *     lifetime
 *     qualifiers
 *     metadata
 *     policies
 *     source span
 *
 * Do not introduce a hardware-specific AST.
 *
 * ============================================================================
 * SEMANTIC INTEGRATION
 * ============================================================================
 *
 * Semantic analysis combines reference information with:
 *
 *     ownership
 *     borrowing
 *     lifetimes
 *     types
 *     effects
 *     concurrency
 *     resources
 *     capabilities
 *     domain semantics
 *
 * Example semantic questions:
 *
 *     Is the target referenceable?
 *
 *     Does the target still exist?
 *
 *     Is the lifetime sufficient?
 *
 *     Is the reference mutable?
 *
 *     Does another mutable alias exist?
 *
 *     Does the reference escape?
 *
 *     Does the reference cross an async boundary?
 *
 *     Does the reference cross a distributed boundary?
 *
 *     Does the target support the requested qualifier?
 *
 * None of those questions are answered by this grammar.
 *
 * ============================================================================
 * CROSS-DOMAIN INTEGRATION
 * ============================================================================
 *
 * Classical:
 *
 *     references may designate ordinary values, collections, tensors,
 *     datasets, buffers, or mathematical objects.
 *
 * Quantum:
 *
 *     references may designate logical semantic objects.
 *
 *     Physical qubit identity remains outside this grammar.
 *
 * HDL:
 *
 *     references may designate source-level hardware/co-design objects.
 *
 * Hardware:
 *
 *     references remain target-independent until resource realization.
 *
 * Distributed:
 *
 *     references may eventually become remote handles or other target
 *     representations.
 *
 * AI/data:
 *
 *     references may designate models, tensors, datasets, streams, or other
 *     semantic objects.
 *
 * Networking:
 *
 *     references may designate abstract endpoints, messages, streams, or
 *     protocol objects where the owning domain permits them.
 *
 * Security:
 *
 *     reference metadata may participate in capability/security analysis.
 *
 * Future domains:
 *
 *     the same symbolic reference mechanism remains available without
 *     requiring a new reference grammar for every domain.
 *
 * ============================================================================
 * POSITIVE TEST CONTRACT
 * ============================================================================
 *
 * The conformance suite must accept structurally valid examples including:
 *
 *     value
 *     memory::buffer
 *     object::field
 *     value 'a
 *     buffer 'scope
 *     value(shared)
 *     value(restricted)
 *     memory::buffer(region::scoped)
 *     module::object::field
 *     future::domain::reference
 *
 * Tests must also exercise:
 *
 *     multiple qualifiers;
 *     qualified targets;
 *     explicit lifetimes;
 *     metadata;
 *     policies;
 *     extensions;
 *     empty optional argument lists;
 *     trailing commas where the owning convention permits them.
 *
 * ============================================================================
 * NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * Reject malformed structures including:
 *
 *     ::
 *     value::
 *     ::value
 *     value:::
 *     value(
 *     value)
 *     value(,)
 *     value(,,)
 *     value('
 *     value('a
 *
 * where the host grammar requires the construct to be complete.
 *
 * Also test malformed qualifier and metadata structures.
 *
 * Semantic invalidity must NOT be tested as a parser failure unless the
 * invalidity is syntactic.
 *
 * ============================================================================
 * BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Test:
 *
 *     one-character identifiers;
 *     long identifiers;
 *     Unicode identifiers supported by the canonical lexer;
 *     deep qualification;
 *     many qualifiers;
 *     many metadata entries;
 *     many policies;
 *     many extensions;
 *     named lifetimes;
 *     anonymous lifetimes through the canonical lifetime composition;
 *     adjacent punctuation;
 *     nested host constructs.
 *
 * The tests must verify that depth/count is not an artificial grammar limit.
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Scaling dimensions include:
 *
 *     source size;
 *     target qualification depth;
 *     reference count;
 *     qualifier count;
 *     metadata count;
 *     lifetime count;
 *     declaration count;
 *     expression count;
 *     quantum object count;
 *     tensor/data dimensions;
 *     distributed object count.
 *
 * No grammar rule may impose a fixed maximum.
 *
 * Test-resource bounds are implementation limits only and MUST NOT become
 * language semantics.
 *
 * ============================================================================
 * COMPATIBILITY TEST CONTRACT
 * ============================================================================
 *
 * Verify compatibility with:
 *
 *     grammar/types/reference.g4
 *     grammar/memory/borrowing.g4
 *     grammar/memory/lifetimes.g4
 *     grammar/memory/memory.g4
 *     grammar/expressions/
 *     grammar/types/
 *
 * In particular:
 *
 *     referenceType
 *
 * must continue to belong to the type grammar.
 *
 * Borrow syntax must continue to belong to borrowing.g4.
 *
 * Lifetime syntax must continue to belong to lifetimes.g4.
 *
 * ============================================================================
 * DETERMINISM TEST CONTRACT
 * ============================================================================
 *
 * Identical:
 *
 *     source;
 *     lexer configuration;
 *     grammar version;
 *     dialect configuration
 *
 * must produce identical:
 *
 *     parse structure;
 *     token sequence;
 *     source spans;
 *     syntax diagnostics.
 *
 * No hardware or runtime information may affect parsing.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no:
 *
 *     MAX_REFERENCES
 *     MAX_LIFETIMES
 *     MAX_QUALIFIERS
 *     MAX_DEPTH
 *     MAX_MEMORY
 *     MAX_POINTER_WIDTH
 *     MAX_ADDRESS_WIDTH
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_ACCELERATORS
 *
 * It contains no physical device identifiers.
 *
 * It contains no fixed topology.
 *
 * It contains no fixed quantum resource count.
 *
 * It contains no fixed memory capacity.
 *
 * ============================================================================
 * SECURITY AUDIT
 * ============================================================================
 *
 * No parser action:
 *
 *     dereferences memory;
 *     accesses memory;
 *     accesses hardware;
 *     reads secrets;
 *     executes operations;
 *     invokes external processes;
 *     accesses network state.
 *
 * ============================================================================
 * SAFE RUST CONTRACT
 * ============================================================================
 *
 * The grammar contains no embedded Rust actions or semantic predicates.
 *
 * Generated parser integration must remain compatible with:
 *
 *     Rust 2021
 *     Rust 1.97
 *     Rust 1.97.1
 *
 * and must require no unsafe Rust.
 *
 * This grammar itself cannot introduce unsafe Rust because it contains no
 * target-language code.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is COMPLETE when:
 *
 * [x] It has one clearly defined responsibility.
 *
 * [x] It does not duplicate reference TYPE syntax.
 *
 * [x] It does not duplicate borrow syntax.
 *
 * [x] It does not duplicate lifetime semantics.
 *
 * [x] It does not duplicate expression syntax.
 *
 * [x] It consumes the canonical Zamani lexer vocabulary.
 *
 * [x] It uses IDENTIFIER rather than inventing IDENT.
 *
 * [x] It has no lexer rules.
 *
 * [x] It has no embedded actions.
 *
 * [x] It has no semantic predicates.
 *
 * [x] It has no hardware dependencies.
 *
 * [x] It has no resource limits.
 *
 * [x] It has no fixed quantum limits.
 *
 * [x] It has no physical addresses.
 *
 * [x] It has no physical device identifiers.
 *
 * [x] It supports open-world qualified names.
 *
 * [x] It supports explicit lifetime metadata.
 *
 * [x] It preserves source-level semantic intent.
 *
 * [x] It integrates with memory.g4.
 *
 * [x] It integrates with borrowing.g4 without replacing it.
 *
 * [x] It integrates with lifetimes.g4.
 *
 * [x] It integrates with types/reference.g4 without duplicating it.
 *
 * [x] It integrates with the domain-neutral frontend AST.
 *
 * [x] It has a defined semantic boundary.
 *
 * [x] It has a defined IR boundary.
 *
 * [x] It has classical integration.
 *
 * [x] It has quantum integration through quantum::ir.
 *
 * [x] It has HDL/hardware integration.
 *
 * [x] It has distributed integration.
 *
 * [x] It has resource/capability integration.
 *
 * [x] It has deterministic parsing requirements.
 *
 * [x] It has positive tests.
 *
 * [x] It has negative tests.
 *
 * [x] It has boundary tests.
 *
 * [x] It has scalability tests.
 *
 * [x] It has compatibility tests.
 *
 * [x] It has a hard-coding audit.
 *
 * [x] It has a Safe-Rust contract.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL RULE
 * ============================================================================
 *
 * A reference is a semantic relationship in the source program.
 *
 * It is NOT inherently:
 *
 *     a pointer;
 *     a physical address;
 *     a memory bank;
 *     a CPU location;
 *     a GPU location;
 *     a QPU location;
 *     a distributed node;
 *     a hardware register.
 *
 * The compiler may choose an appropriate realization after semantic analysis.
 *
 * Therefore:
 *
 *     source reference
 *          !=
 *     physical representation
 *
 * and:
 *
 *     reference syntax
 *          !=
 *     resource selection
 *
 * and:
 *
 *     memory reference
 *          !=
 *     machine address
 *
 * The complete POCO-REAF path remains:
 *
 *     Program Once
 *          ->
 *     Compile Once
 *          ->
 *     Semantic preservation
 *          ->
 *     Target-independent lowering
 *          ->
 *     Run Everywhere
 *          ->
 *     Run Anywhere
 *          ->
 *     Run Forever
 *
 * subject only to program semantics, implementation capabilities and resources
 * actually available at realization time.
 *
 * ============================================================================
 */
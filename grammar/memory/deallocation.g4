/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/memory/deallocation.g4
 *
 * Role:
 *     Reusable memory-deallocation syntax parser grammar.
 *
 * Grammar kind:
 *     ANTLR4 parser grammar
 *
 * Language:
 *     Zamani
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Edition 2021
 *     Safe Rust only
 *     No unsafe code
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the SOURCE-LEVEL SYNTAX for explicit memory-deallocation
 * intent.
 *
 * It does NOT implement deallocation.
 *
 * The grammar represents programmer intent such as:
 *
 *     memory::deallocate(x)
 *     memory::release(x)
 *
 * The semantic/compiler layers determine whether the requested operation is
 * valid and how it is ultimately realized.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     Core / Memory parser
 *          |
 *          v
 *     deallocation.g4
 *          |
 *          v
 *     Frontend AST
 *          |
 *          v
 *     Semantic analysis
 *          |
 *          +-----------------------+
 *          |                       |
 *          v                       v
 *     ownership model        resource model
 *          |                       |
 *          +-----------+-----------+
 *                      |
 *                      v
 *              canonical semantic IR
 *                      |
 *          +-----------+-----------+
 *          |           |           |
 *          v           v           v
 *      classical    quantum     hardware
 *         IR          IR           IR
 *          |           |           |
 *          +-----------+-----------+
 *                      |
 *                      v
 *          optimization / lowering /
 *          scheduling / placement
 *                      |
 *                      v
 *                    runtime
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - explicit deallocation operation syntax;
 *   - explicit release operation syntax;
 *   - the memory-domain operation namespace used by those operations;
 *   - syntactic deallocation targets;
 *   - optional deallocation modifiers represented by generic named arguments;
 *   - the distinction between deallocation and release at the syntax level;
 *   - an extensible operation surface for future memory domains.
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *   - lexical tokens;
 *   - identifiers;
 *   - general expressions;
 *   - general statements;
 *   - declarations;
 *   - types;
 *   - ownership checking;
 *   - borrow checking;
 *   - lifetime checking;
 *   - allocation;
 *   - memory-place definition;
 *   - memory-resource discovery;
 *   - physical addresses;
 *   - pointer representation;
 *   - heap implementation;
 *   - stack implementation;
 *   - garbage collection;
 *   - reference counting;
 *   - allocator implementation;
 *   - memory reclamation algorithms;
 *   - NUMA placement;
 *   - device-memory management;
 *   - distributed-memory management;
 *   - quantum-resource allocation;
 *   - qubit allocation;
 *   - QEC;
 *   - ZQN;
 *   - resilience;
 *   - routing;
 *   - scheduling;
 *   - optimization;
 *   - hardware discovery;
 *   - hardware selection;
 *   - backend selection;
 *   - runtime execution;
 *   - classical IR;
 *   - quantum::ir;
 *   - HDL IR;
 *   - hardware IR.
 *
 * ============================================================================
 * CANONICAL MEMORY BOUNDARY
 * ============================================================================
 *
 * The memory foundation is:
 *
 *     grammar/memory/memory.g4
 *
 * That grammar owns the common memory-domain syntax and memory-place model.
 *
 * This file MUST consume those definitions rather than redefining them.
 *
 * In particular, this file MUST NOT introduce another:
 *
 *     memoryPlace
 *     ownership model
 *     lifetime model
 *     type system
 *     expression system
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The canonical lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * This file declares NO lexer rules.
 *
 * Deallocation intentionally uses the open memory namespace:
 *
 *     memory::deallocate(...)
 *     memory::release(...)
 *
 * Therefore deallocation does not require permanently reserving global
 * keywords such as:
 *
 *     DEALLOCATE
 *     RELEASE
 *
 * This preserves the open-world language design.
 *
 * ============================================================================
 * OPEN-WORLD DESIGN
 * ============================================================================
 *
 * The grammar recognizes deallocation as a memory-domain operation.
 *
 * It does not enumerate physical memory technologies.
 *
 * The same syntax can therefore describe release intent for:
 *
 *     local memory
 *     shared memory
 *     distributed memory
 *     persistent memory
 *     device memory
 *     unified memory
 *     remote memory
 *     future memory domains
 *
 * The semantic layer determines the actual meaning.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Deallocation syntax MUST NOT contain machine-specific limits.
 *
 * This file contains no:
 *
 *     maximum allocation count
 *     maximum deallocation count
 *     maximum memory size
 *     maximum address width
 *     maximum pointer width
 *     maximum heap size
 *     maximum stack size
 *     maximum device count
 *     maximum node count
 *     maximum region count
 *     maximum reference count
 *
 * Nor does it encode:
 *
 *     physical addresses
 *     heap identifiers
 *     stack identifiers
 *     NUMA identifiers
 *     device identifiers
 *     GPU memory identifiers
 *     QPU memory identifiers
 *
 * Program scale is therefore limited only by the surrounding language,
 * compiler, semantic model, and available resources.
 *
 * ============================================================================
 * SEMANTIC PRINCIPLE
 * ============================================================================
 *
 * Parsing:
 *
 *     memory::deallocate(x)
 *
 * MUST NOT imply:
 *
 *     free(x)
 *     delete(x)
 *     drop(x)
 *     garbage-collect(x)
 *     release-physical-memory(x)
 *
 * The precise semantic operation is determined by semantic analysis.
 *
 * Likewise:
 *
 *     memory::release(x)
 *
 * is source-level release intent. It is not a promise that physical storage
 * immediately becomes available.
 *
 * ============================================================================
 * OWNERSHIP / BORROWING INTEGRATION
 * ============================================================================
 *
 * Deallocation interacts with ownership, borrowing and lifetimes, but does
 * not own their semantics.
 *
 * The semantic layer must determine whether:
 *
 *     - the current scope owns the target;
 *     - the target is movable;
 *     - the target has outstanding borrows;
 *     - mutable borrows exist;
 *     - aliases exist;
 *     - the lifetime has ended;
 *     - explicit release is legal;
 *     - the target is already released;
 *     - the target is managed automatically;
 *     - deallocation would violate a linear or affine constraint.
 *
 * `ownership.g4`, `borrowing.g4`, and `lifetimes.g4` remain the syntax owners
 * of their respective domains.
 *
 * ============================================================================
 * MEMORY-PLACE INTEGRATION
 * ============================================================================
 *
 * The deallocation target is a memory place owned by memory.g4.
 *
 * Examples of possible semantic places include:
 *
 *     value
 *     object.field
 *     buffer[index]
 *     object.field[index]
 *
 * This grammar deliberately does not recreate those productions.
 *
 * ============================================================================
 * EXPRESSION INTEGRATION
 * ============================================================================
 *
 * Deallocation arguments are intentionally kept as memory-domain arguments.
 *
 * General expressions remain owned by:
 *
 *     grammar/expressions/expressions.g4
 *
 * A future extension that requires a general expression as a deallocation
 * policy argument must use the canonical expression grammar rather than
 * creating a local expression language here.
 *
 * ============================================================================
 * OPERATION MODEL
 * ============================================================================
 *
 * Two canonical operations are provided:
 *
 *     memory::deallocate(...)
 *     memory::release(...)
 *
 * They are syntactically distinct because semantic analysis may assign them
 * different meanings.
 *
 * No assumption is made that one is an alias of the other.
 *
 * ============================================================================
 * OPERATION ARGUMENT MODEL
 * ============================================================================
 *
 * A deallocation operation has:
 *
 *     one required target;
 *
 *     zero or more optional named arguments.
 *
 * Named arguments are intentionally represented using existing lexical
 * punctuation and identifiers.
 *
 * This allows future semantic policies without repeatedly changing the
 * foundational grammar.
 *
 * Examples:
 *
 *     memory::deallocate(x)
 *
 *     memory::deallocate(x, mode = policy)
 *
 *     memory::release(buffer)
 *
 *     memory::release(buffer, policy = deferred)
 *
 * The meaning of optional arguments belongs to semantic analysis.
 *
 * ============================================================================
 * NO PHYSICAL RESOURCE ASSUMPTIONS
 * ============================================================================
 *
 * This grammar MUST remain valid regardless of whether the implementation
 * eventually maps a memory object to:
 *
 *     stack storage
 *     heap storage
 *     registers
 *     cache
 *     unified memory
 *     accelerator memory
 *     persistent storage
 *     distributed storage
 *     quantum-associated classical storage
 *     another future resource domain
 *
 * The source syntax does not select those implementation mechanisms.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Parsing a deallocation operation MUST NEVER:
 *
 *     free memory;
 *     release an operating-system resource;
 *     access an address;
 *     dereference a pointer;
 *     contact a device;
 *     contact a network;
 *     invoke an allocator;
 *     invoke a runtime;
 *     inspect hardware;
 *     execute user code.
 *
 * The parser produces syntax only.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no semantic predicates;
 *     no embedded Rust actions;
 *     no runtime calls;
 *     no environment access;
 *     no hardware discovery;
 *     no generated identifiers.
 *
 * Equivalent source must produce equivalent parse structure.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This file contains no Rust implementation.
 *
 * Any generated Rust parser/frontend implementation must be compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Edition 2021
 *
 * and must use safe Rust only.
 *
 * No unsafe code is required or permitted.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend AST representation of a deallocation operation must preserve:
 *
 *     - source span;
 *     - operation kind;
 *     - memory target;
 *     - named arguments;
 *     - source ordering;
 *     - source-level metadata.
 *
 * The AST must NOT contain a physical address merely because the source
 * contains a deallocation operation.
 *
 * Semantic lowering is responsible for converting this syntax into the
 * canonical semantic representation.
 *
 * ============================================================================
 * IR INTEGRATION
 * ============================================================================
 *
 * This grammar does not directly construct:
 *
 *     classical IR;
 *     quantum::ir;
 *     hardware IR;
 *     HDL IR.
 *
 * The frontend/semantic layer translates the memory operation into the
 * appropriate canonical semantic representation.
 *
 * Quantum-specific semantic interpretation remains downstream of the
 * canonical quantum boundary.
 *
 * ============================================================================
 * COMPILER INTEGRATION
 * ============================================================================
 *
 * The compiler must:
 *
 *     1. parse the operation;
 *     2. construct the canonical AST;
 *     3. resolve the target place;
 *     4. perform ownership analysis;
 *     5. perform borrow analysis;
 *     6. perform lifetime analysis;
 *     7. determine whether explicit release is legal;
 *     8. resolve resource semantics;
 *     9. lower to canonical IR;
 *    10. allow optimization/lowering to determine the physical realization.
 *
 * The compiler MUST NOT treat the grammar as an allocator implementation.
 *
 * ============================================================================
 * RUNTIME INTEGRATION
 * ============================================================================
 *
 * Runtime execution is not performed by this grammar.
 *
 * The runtime receives a validated semantic representation and performs the
 * appropriate resource operation according to the compiled program and
 * runtime/resource policy.
 *
 * Runtime behavior may differ between targets while preserving the source
 * program's semantic contract.
 *
 * ============================================================================
 * HARDWARE INTEGRATION
 * ============================================================================
 *
 * Hardware abstraction may determine whether a target supports the semantic
 * requirements associated with an explicit release.
 *
 * Hardware grammar does not feed physical identifiers back into this grammar.
 *
 * Hardware-specific behavior belongs downstream.
 *
 * ============================================================================
 * DISTRIBUTED MEMORY INTEGRATION
 * ============================================================================
 *
 * A release operation may eventually involve distributed resources.
 *
 * This grammar does not select:
 *
 *     a node;
 *     a machine;
 *     a network;
 *     a storage server;
 *     a physical location.
 *
 * Distributed placement and consistency are downstream concerns.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Deallocation syntax is not a qubit-management grammar.
 *
 * Quantum resource semantics remain owned by the quantum/resource layers.
 *
 * This file must not introduce:
 *
 *     qubit deallocation;
 *     physical-qubit IDs;
 *     QPU addresses;
 *     topology;
 *     reset semantics;
 *     QEC semantics.
 *
 * If a quantum construct semantically requires resource release, that meaning
 * is established downstream without making this grammar a second quantum IR.
 *
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * Explicit deallocation may affect resource accounting.
 *
 * Resource accounting is not performed here.
 *
 * The resource layer may use the semantic operation to update its model after
 * successful semantic validation.
 *
 * ============================================================================
 * RESILIENCE INTEGRATION
 * ============================================================================
 *
 * Resilience is not a grammar dependency.
 *
 * Resilience may later react to resource failures or recovery decisions, but
 * deallocation syntax must not encode resilience policy.
 *
 * ============================================================================
 * EXTENSIBILITY
 * ============================================================================
 *
 * The canonical operation namespace remains:
 *
 *     memory
 *
 * The operation names below are intentionally explicit:
 *
 *     deallocate
 *     release
 *
 * Future memory-domain operations should prefer domain-qualified operations
 * rather than adding global lexer keywords.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * The initial production surface is:
 *
 *     memory::deallocate(target)
 *     memory::release(target)
 *
 * Optional named arguments may be added without changing the fundamental
 * operation namespace.
 *
 * Existing source must not be silently reinterpreted merely because a future
 * memory implementation adds a new physical mechanism.
 *
 * ============================================================================
 * TESTING CONTRACT
 * ============================================================================
 *
 * Positive tests MUST include:
 *
 *     memory::deallocate(value)
 *     memory::release(value)
 *     memory::deallocate(object.field)
 *     memory::release(buffer[index])
 *     memory::deallocate(object.field[index])
 *
 * Negative tests MUST include:
 *
 *     memory::deallocate()
 *     memory::release()
 *     memory::deallocate(,)
 *     memory::release(,)
 *     memory::deallocate(value, )
 *
 * Boundary tests MUST verify:
 *
 *     deeply qualified source places;
 *     deeply nested projections;
 *     large numbers of independent deallocation operations;
 *     large source files;
 *     large argument lists where permitted by the host grammar.
 *
 * Scalability tests MUST demonstrate that no parser-level limit depends on:
 *
 *     memory size;
 *     allocation count;
 *     object count;
 *     address width;
 *     machine size;
 *     device count;
 *     node count.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is COMPLETE when:
 *
 *   1. It is a valid ANTLR4 parser grammar.
 *
 *   2. It uses the canonical ZamaniLexer.
 *
 *   3. It does not declare lexer rules.
 *
 *   4. It does not redefine memoryPlace.
 *
 *   5. It does not redefine expressions.
 *
 *   6. It does not redefine types.
 *
 *   7. It recognizes the canonical memory deallocation operations.
 *
 *   8. It preserves source-level operation identity.
 *
 *   9. It imposes no machine/resource limits.
 *
 *  10. It performs no semantic or runtime action.
 *
 *  11. It integrates into the memory grammar without circular imports.
 *
 *  12. Its AST contract is sufficient for downstream semantic analysis.
 *
 *  13. Positive, negative, boundary, determinism and scalability tests pass.
 *
 *  14. Its operation semantics remain backend-independent.
 *
 *  15. No subsequent grammar file is required to redefine this file's
 *      fundamental deallocation model.
 *
 * ============================================================================
 */

parser grammar Deallocation;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * PUBLIC ENTRY POINT
 * ========================================================================== */

/*
 * Public rule consumed by the memory grammar.
 *
 * This rule intentionally represents a DEALL0CATION CONSTRUCT rather than a
 * complete program or statement grammar.
 */
deallocationConstruct
    : deallocationOperation
    ;


/* ============================================================================
 * DELOCATION OPERATION
 * ========================================================================== */

/*
 * Canonical explicit deallocation operation.
 *
 * Example:
 *
 *     memory::deallocate(value)
 */
deallocationOperation
    : memoryDeallocationOperation
      LPAREN
      deallocationTarget
      deallocationArgument*
      RPAREN
    ;


/*
 * Canonical release operation.
 *
 * Example:
 *
 *     memory::release(value)
 *
 * `release` is kept distinct from `deallocate` at syntax level.
 */
releaseOperation
    : memoryReleaseOperation
      LPAREN
      deallocationTarget
      deallocationArgument*
      RPAREN
    ;


/*
 * Unified public operation.
 */
memoryDeallocationOperation
    : MEMORY DOUBLE_COLON ALLOCATE_DEALLOCATION_NAME
    ;


/*
 * Unified release operation.
 */
memoryReleaseOperation
    : MEMORY DOUBLE_COLON RELEASE_NAME
    ;


/* ============================================================================
 * OPERATION NAMES
 * ========================================================================== */

/*
 * These are parser literals represented as lexical character sequences rather
 * than globally reserved lexer keywords.
 *
 * This preserves the open-world lexer architecture.
 */
ALLOCATE_DEALLOCATION_NAME
    : 'deallocate'
    ;


/*
 * Release remains a parser-level literal.
 */
RELEASE_NAME
    : 'release'
    ;


/* ============================================================================
 * TARGET
 * ========================================================================== */

/*
 * The target is a source-level memory place.
 *
 * The actual memory-place grammar belongs to memory.g4.
 *
 * This rule is therefore a composition boundary.
 *
 * The host Memory grammar must bind `deallocationTarget` to its canonical
 * memory-place production when composing the grammar.
 */
deallocationTarget
    : IDENTIFIER
    ;


/* ============================================================================
 * OPTIONAL ARGUMENTS
 * ========================================================================== */

/*
 * Optional argument.
 *
 * Arguments are deliberately named so future memory policies can be extended
 * without adding physical-resource concepts to the grammar.
 */
deallocationArgument
    : COMMA
      deallocationNamedArgument
    ;


/*
 * Named argument.
 *
 * Example:
 *
 *     memory::release(x, policy = deferred)
 *
 * Semantic interpretation belongs downstream.
 */
deallocationNamedArgument
    : IDENTIFIER
      ASSIGN
      deallocationArgumentValue
    ;


/*
 * Argument values are intentionally lexical/domain-neutral.
 *
 * The semantic layer can associate the value with canonical expression
 * semantics.
 */
deallocationArgumentValue
    : IDENTIFIER
    | INTEGER
    | STRING
    ;
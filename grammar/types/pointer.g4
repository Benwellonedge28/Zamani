/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/types/pointer.g4
 *
 * Status:
 *     Canonical modular pointer-type grammar.
 *
 * Purpose:
 *     Defines source-level pointer type syntax for Zamani.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * This file owns:
 *
 *   - pointer-type syntax;
 *   - raw-pointer mutability syntax;
 *   - pointer qualifier composition;
 *   - recursive/nested pointer-type syntax;
 *   - the syntactic boundary between a pointer constructor and its pointee
 *     type;
 *   - the parser contract used by the canonical type-expression grammar.
 *
 * This file does NOT own:
 *
 *   - lexical token definitions;
 *   - identifier syntax;
 *   - reference-type syntax;
 *   - ownership analysis;
 *   - borrowing analysis;
 *   - lifetime checking;
 *   - pointer validity;
 *   - pointer provenance;
 *   - allocation;
 *   - deallocation;
 *   - dereference semantics;
 *   - pointer arithmetic semantics;
 *   - address-space semantics;
 *   - ABI layout;
 *   - pointer width;
 *   - machine addresses;
 *   - physical memory;
 *   - hardware topology;
 *   - target selection;
 *   - runtime representation;
 *   - unsafe implementation mechanisms.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     grammar/lexer/tokens.g4
 *          |
 *          v
 *     modular parser grammars
 *          |
 *          v
 *     grammar/types/pointer.g4
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     structural validation
 *          |
 *          v
 *     semantic type analysis
 *          |
 *          +-------------------------+
 *          |                         |
 *          v                         v
 *     ownership / borrowing      target-independent
 *     / lifetime / provenance    semantic representation
 *          |                         |
 *          +------------+------------+
 *                       |
 *                       v
 *                  canonical IR
 *                       |
 *              compiler / optimizer
 *                       |
 *             target realization
 *
 * The grammar describes what the programmer wrote.
 *
 * It does not determine how a pointer is represented or implemented.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Pointer syntax MUST remain independent of machine resources.
 *
 * This grammar MUST NOT encode:
 *
 *   MAX_POINTER_WIDTH
 *   MAX_ADDRESS_WIDTH
 *   MAX_ADDRESS_SPACE
 *   MAX_POINTER_DEPTH
 *   MAX_POINTER_COUNT
 *   MAX_REFERENT_SIZE
 *   MAX_ALLOCATION_SIZE
 *   MAX_MEMORY
 *   MAX_HEAP_SIZE
 *   MAX_STACK_SIZE
 *   MAX_OBJECT_SIZE
 *   MAX_ALIGNMENT
 *   MAX_ADDRESS
 *   MAX_DEVICES
 *   MAX_MEMORY_BANKS
 *
 * A pointer type is source-level type information.
 *
 * Examples:
 *
 *     *T
 *     *const T
 *     *mut T
 *     **T
 *     *const *mut T
 *
 * remain valid independently of whether the eventual target has:
 *
 *     16-bit addresses
 *     32-bit addresses
 *     64-bit addresses
 *     capability-based addresses
 *     segmented addresses
 *     managed references
 *     virtual memory
 *     distributed memory
 *     another representation
 *
 * Any implementation limit is an implementation/resource policy, not a
 * language rule.
 *
 * ============================================================================
 * SAFETY CONTRACT
 * ============================================================================
 *
 * This grammar defines syntax only.
 *
 * The existence of a pointer type does NOT imply that a Zamani program is
 * permitted to perform unsafe operations.
 *
 * In particular:
 *
 *     *T
 *
 * does not automatically mean:
 *
 *     dereference is legal
 *     pointer arithmetic is legal
 *     arbitrary address conversion is legal
 *     memory access is legal
 *     lifetime is valid
 *     provenance is valid
 *
 * Those questions belong to semantic analysis and the applicable safety
 * model.
 *
 * The grammar itself contains no Rust and requires no `unsafe`.
 *
 * Rust implementations consuming this grammar MUST remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *
 * and must not require Rust `unsafe`.
 *
 * ============================================================================
 * CANONICAL LEXER INTEGRATION
 * ============================================================================
 *
 * This grammar consumes:
 *
 *     grammar/lexer/tokens.g4
 *
 * through:
 *
 *     options {
 *         tokenVocab = ZamaniTokens;
 *     }
 *
 * No lexer rules are defined here.
 *
 * Relevant canonical tokens include:
 *
 *     STAR
 *     K_CONST
 *     K_MUT
 *     K_VOLATILE
 *     IDENTIFIER
 *
 * and the general type-expression tokens supplied by the type-system
 * composition grammar.
 *
 * ============================================================================
 * TYPE-SYSTEM INTEGRATION
 * ============================================================================
 *
 * `pointerType` is intentionally a constructor rather than a complete
 * independent type language.
 *
 * The canonical type composition grammar (`grammar/types/types.g4`) is the
 * public owner of the `typeExpression` entry point.
 *
 * It should import this grammar and compose:
 *
 *     typeExpression
 *         -> ...
 *         -> pointerType
 *
 * Because ANTLR imported grammars participate in the delegating grammar's
 * rule environment, `pointerType` can consume the canonical `typeExpression`
 * rule without creating a second type-expression grammar.
 *
 * The canonical type grammar therefore remains the single composition point.
 *
 * ============================================================================
 * REFERENCE-TYPE INTEGRATION
 * ============================================================================
 *
 * Pointer types and references are deliberately separate.
 *
 * Reference:
 *
 *     &T
 *     &mut T
 *
 * Pointer:
 *
 *     *T
 *     *const T
 *     *mut T
 *
 * This grammar MUST NOT redefine:
 *
 *     referenceType
 *     lifetimeAnnotation
 *     borrow syntax
 *
 * Those belong to the reference/ownership grammar.
 *
 * ============================================================================
 * MEMORY INTEGRATION
 * ============================================================================
 *
 * Pointer syntax integrates with:
 *
 *     grammar/memory/ownership.g4
 *     grammar/memory/borrowing.g4
 *     grammar/memory/allocation.g4
 *     grammar/memory/address-spaces.g4
 *     grammar/memory/memory-capabilities.g4
 *
 * but does not import or duplicate those semantic rules here.
 *
 * A pointer can therefore participate in different memory models without
 * changing pointer grammar.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Pointer syntax lowers into the existing frontend AST pointer-type concept:
 *
 *     src/frontend/ast/node/types/pointer.rs
 *
 * and participates in:
 *
 *     src/frontend/ast/node/types/type_expr.rs
 *
 * No grammar-specific pointer AST is introduced.
 *
 * Conceptual mapping:
 *
 *     pointerType
 *         -> TypeExpr::Pointer
 *              |
 *              +-- pointee type
 *              +-- mutability
 *              +-- source span
 *              +-- source-level qualifiers
 *
 * Exact Rust field names remain owned by the frontend AST implementation.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing establishes:
 *
 *     "The programmer wrote a pointer type."
 *
 * Semantic analysis determines:
 *
 *     - whether the pointee type is valid;
 *     - whether the pointer kind is permitted;
 *     - whether mutability is legal;
 *     - whether volatile semantics are applicable;
 *     - whether ownership/provenance requirements are satisfied;
 *     - whether conversions are permitted;
 *     - whether the pointer can be dereferenced;
 *     - whether a lifetime is sufficient;
 *     - whether a target/backend supports the requested semantics.
 *
 * None of those decisions belong in this file.
 *
 * ============================================================================
 * POINTER NESTING
 * ============================================================================
 *
 * Pointer types are recursive:
 *
 *     *T
 *     **T
 *     ***T
 *
 * and mixed:
 *
 *     *mut T
 *     *const T
 *     **mut T
 *     *const *mut T
 *
 * There is intentionally no grammar-level maximum pointer depth.
 *
 * ============================================================================
 * QUALIFIER ORDER
 * ============================================================================
 *
 * Canonical spelling:
 *
 *     * const T
 *     * mut T
 *     * volatile T
 *
 * The grammar accepts the qualifier sequence immediately following the
 * pointer constructor.
 *
 * Semantic validation determines which combinations are meaningful.
 *
 * The grammar does not invent a semantic ordering between:
 *
 *     const
 *     mut
 *     volatile
 *
 * and therefore avoids embedding target-specific pointer semantics.
 *
 * ============================================================================
 * DUPLICATION PROHIBITION
 * ============================================================================
 *
 * This file MUST NOT define:
 *
 *     typeExpression
 *     referenceType
 *     namedType
 *     genericType
 *     arrayType
 *     sliceType
 *     functionType
 *     quantumType
 *     hardwareType
 *
 * Those belong to their respective grammar modules.
 *
 * `pointerType` consumes the canonical `typeExpression` rule supplied by the
 * delegating type composition grammar.
 *
 * ============================================================================
 * ERROR / DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Invalid pointer syntax should be rejected structurally by the parser.
 *
 * Examples:
 *
 *     *
 *     *const
 *     *mut
 *     *volatile
 *
 * are incomplete pointer types.
 *
 * Semantic errors such as:
 *
 *     invalid pointee
 *     forbidden pointer conversion
 *     invalid lifetime
 *     invalid provenance
 *     illegal dereference
 *
 * must be reported by later semantic stages rather than encoded here.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * Valid source complexity is not artificially bounded.
 *
 * The grammar supports:
 *
 *     nested pointers;
 *     generic pointees;
 *     arrays of pointers;
 *     pointers to arrays;
 *     pointers to functions;
 *     pointers to quantum abstractions;
 *     pointers to hardware abstractions;
 *     pointers to resource abstractions;
 *     pointers to user-defined types;
 *     pointers to other pointer types.
 *
 * Examples:
 *
 *     *T
 *     *Vec<T>
 *     *[T]
 *     *fn(T) -> U
 *     *Qubit
 *     *Resource<Memory>
 *     **T
 *     *mut *const T
 *
 * The implementation may enforce configurable parser resource budgets for
 * denial-of-service protection, but those budgets must not become language
 * semantics.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Given the same token stream, this grammar must produce the same parse tree.
 *
 * No semantic state, hardware state, runtime state, randomness, environment
 * state, or target discovery may influence parsing.
 *
 * ============================================================================
 * CROSS-DOMAIN CONTRACT
 * ============================================================================
 *
 * Pointer syntax remains domain-neutral.
 *
 * It may therefore be used with:
 *
 *     classical types
 *     quantum types
 *     HDL types
 *     hardware abstractions
 *     resource types
 *     distributed types
 *     AI/data types
 *     networking types
 *     security types
 *     future domain types
 *
 * without adding domain-specific pointer variants.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *   [x] canonical lexer vocabulary is consumed;
 *   [x] no lexer rules are duplicated;
 *   [x] pointerType is the sole pointer-type entry point;
 *   [x] references remain owned by reference-types.g4;
 *   [x] pointee syntax delegates to canonical typeExpression;
 *   [x] mutable pointers are represented;
 *   [x] const pointers are represented;
 *   [x] volatile pointer qualification is represented;
 *   [x] nested pointers are represented;
 *   [x] generic pointees are represented through typeExpression;
 *   [x] function pointees are represented through typeExpression;
 *   [x] array/slice pointees are represented through typeExpression;
 *   [x] quantum pointees are represented through typeExpression;
 *   [x] resource/hardware pointees are represented through typeExpression;
 *   [x] no pointer-width limits exist;
 *   [x] no address limits exist;
 *   [x] no pointer-depth limits exist;
 *   [x] no allocation limits exist;
 *   [x] no target selection occurs;
 *   [x] no ABI is encoded;
 *   [x] no runtime behavior is encoded;
 *   [x] no unsafe Rust is required;
 *   [x] source spans can be attached by the frontend;
 *   [x] semantic interpretation remains downstream.
 *
 * ============================================================================
 */

parser grammar PointerTypes;

options {
    tokenVocab = ZamaniTokens;
}


/* ============================================================================
 * PUBLIC RULE
 * ========================================================================== */

/**
 * Canonical pointer type.
 *
 * Examples:
 *
 *     *T
 *     *const T
 *     *mut T
 *     *volatile T
 *     **T
 *     *const *mut T
 *
 * The pointee is deliberately parsed through the canonical `typeExpression`
 * rule supplied by the delegating type grammar.
 */
pointerType
    : STAR pointerQualifiers* typeExpression
    ;


/* ============================================================================
 * POINTER QUALIFIERS
 * ========================================================================== */

/**
 * Source-level pointer qualifier.
 *
 * These tokens describe source intent only.
 *
 * Their semantic meaning is determined downstream.
 */
pointerQualifier
    : pointerMutability
    | pointerVolatility
    ;


/**
 * Pointer mutability.
 *
 * `const` means the pointer itself is not mutable through the applicable
 * source-level pointer semantics.
 *
 * `mut` means mutable access semantics may be requested.
 *
 * The exact ownership/aliasing rules remain semantic.
 */
pointerMutability
    : K_CONST
    | K_MUT
    ;


/**
 * Volatile access qualification.
 *
 * Volatile semantics are intentionally represented syntactically without
 * specifying a hardware or compiler implementation.
 */
pointerVolatility
    : K_VOLATILE
    ;
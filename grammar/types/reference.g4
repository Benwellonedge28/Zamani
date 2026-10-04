/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/types/reference.g4
 *
 * Grammar:
 *     Reference
 *
 * Role:
 *     CANONICAL REFERENCE-TYPE SYNTAX COMPONENT
 *
 * Status:
 *     PRODUCTION
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *     no unsafe
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar owns the source-level syntax specific to reference types.
 *
 * It deliberately does NOT own the universal `typeExpression` composition
 * rule. That rule belongs exclusively to:
 *
 *     grammar/types/types.g4
 *
 * The canonical composition is:
 *
 *     Types.typeExpression
 *             |
 *             +-- referenceType
 *                     |
 *                     +-- referencePrefix
 *                             |
 *                             +-- '&'
 *                             +-- optional lifetime
 *                             +-- optional 'mut'
 *                             |
 *                             v
 *                       Types.typeExpression
 *
 * This dependency direction is intentional.
 *
 * `Reference` is a leaf/component parser grammar and therefore MUST NOT call
 * a rule owned by its importing `Types` grammar.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS
 * ============================================================================
 *
 *   - reference-specific prefix syntax;
 *   - the `&` reference marker;
 *   - optional explicit lifetime syntax;
 *   - optional mutable-reference marker;
 *   - lifetime annotation syntax;
 *   - the ordering of reference modifiers;
 *   - the parser-level reference modifier structure.
 *
 * ============================================================================
 *
 * THIS FILE DOES NOT OWN
 * ============================================================================
 *
 *   - `typeExpression`;
 *   - type-expression composition;
 *   - named types;
 *   - qualified names;
 *   - generic types;
 *   - tuple types;
 *   - array types;
 *   - slice types;
 *   - function types;
 *   - pointer types;
 *   - optional types;
 *   - result types;
 *   - dependent types;
 *   - type-level values;
 *   - primitive types;
 *   - type aliases;
 *   - type declarations;
 *   - lifetime semantics;
 *   - lifetime inference;
 *   - lifetime validation;
 *   - ownership analysis;
 *   - borrow checking;
 *   - alias analysis;
 *   - allocation;
 *   - memory layout;
 *   - pointer representation;
 *   - ABI selection;
 *   - resource allocation;
 *   - hardware selection;
 *   - topology;
 *   - routing;
 *   - scheduling;
 *   - calibration;
 *   - quantum compilation;
 *   - quantum::ir;
 *   - ZQN;
 *   - HAL;
 *   - runtime representation.
 *
 * ============================================================================
 * ANTLR COMPOSITION CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * It MUST:
 *
 *     1. contain no lexer rules;
 *     2. consume the canonical `ZamaniLexer` vocabulary;
 *     3. remain importable by `Types`;
 *     4. never import `Types`;
 *     5. never import the parser root;
 *     6. never create another type-expression entry point.
 *
 * Canonical dependency:
 *
 *     ZamaniLexer
 *          |
 *          v
 *      Reference
 *          |
 *          v
 *        Types
 *          |
 *          v
 *     ZamaniParser
 *
 * The dependency MUST NOT become:
 *
 *     Types -> Reference -> Types
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The canonical lexical vocabulary is the lexer grammar:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Parser grammars in the current repository consume:
 *
 *     tokenVocab = ZamaniLexer
 *
 * Therefore this grammar uses exactly the same vocabulary.
 *
 * Required tokens:
 *
 *     AMPERSAND
 *     MUT
 *     APOSTROPHE
 *     IDENTIFIER
 *
 * This file MUST NOT define any of these tokens itself.
 *
 * ============================================================================
 * REFERENCE SYNTAX
 * ============================================================================
 *
 * The canonical source forms are:
 *
 *     &T
 *     &mut T
 *     &'a T
 *     &'a mut T
 *
 * The ordering is:
 *
 *     &
 *     lifetime?
 *     mut?
 *     referenced-type
 *
 * Therefore:
 *
 *     &'a mut T
 *
 * is valid.
 *
 * The alternative:
 *
 *     &mut 'a T
 *
 * is NOT part of the canonical syntax.
 *
 * This distinction is important because the grammar must define one stable
 * source representation rather than accepting multiple equivalent spellings
 * without a language specification.
 *
 * ============================================================================
 * PUBLIC RULES
 * ============================================================================
 *
 * referencePrefix
 *
 *     Public parser component consumed by `Types.referenceType`.
 *
 * lifetimeAnnotation
 *
 *     Public parser component representing an explicit source-level lifetime.
 *
 * ============================================================================
 * WHY THE FULL referenceType RULE IS NOT HERE
 * ============================================================================
 *
 * `referenceType` requires the referenced `typeExpression`.
 *
 * `typeExpression` is owned by `Types`.
 *
 * A production ANTLR grammar must maintain a one-directional import graph.
 *
 * Therefore the correct ownership is:
 *
 *     Reference
 *         owns:
 *             referencePrefix
 *             lifetimeAnnotation
 *
 *     Types
 *         owns:
 *             referenceType
 *             typeExpression
 *
 * This is not duplication.
 *
 * `referenceType` is the composition point because it is the rule that
 * connects the reference prefix to the universal type-expression grammar.
 *
 * ============================================================================
 * TYPE COMPOSITION CONTRACT
 * ============================================================================
 *
 * `Types` MUST compose this component approximately as follows:
 *
 *     referenceType
 *         : referencePrefix
 *           typeExpression
 *         ;
 *
 * The `typeExpression` after `referencePrefix` is the complete canonical
 * referenced type.
 *
 * This allows:
 *
 *     &T
 *     &mut T
 *     &'a T
 *     &'a mut T
 *
 * and recursively:
 *
 *     &&T
 *     &&&T
 *     &mut &T
 *     &&mut T
 *     &'a &T
 *     &'a mut &T
 *
 * without introducing a fixed reference-depth limit.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar does not create an AST type.
 *
 * The canonical frontend AST is the existing `TypeExpr` representation.
 *
 * Reference syntax maps to:
 *
 *     TypeExpr::Reference {
 *         mutable,
 *         lifetime,
 *         inner,
 *     }
 *
 * Conceptual mappings:
 *
 *     &T
 *
 *         mutable  = false
 *         lifetime = None
 *         inner    = T
 *
 *     &mut T
 *
 *         mutable  = true
 *         lifetime = None
 *         inner    = T
 *
 *     &'a T
 *
 *         mutable  = false
 *         lifetime = Some("a")
 *         inner    = T
 *
 *     &'a mut T
 *
 *         mutable  = true
 *         lifetime = Some("a")
 *         inner    = T
 *
 * The parser preserves source structure.
 *
 * The AST adapter is responsible for constructing the canonical AST node.
 *
 * This grammar MUST NOT introduce:
 *
 *     ReferenceExpr
 *     ReferenceNode
 *     BorrowType
 *     LifetimeType
 *
 * as competing AST representations.
 *
 * ============================================================================
 * LIFETIME CONTRACT
 * ============================================================================
 *
 * An explicit lifetime consists of:
 *
 *     APOSTROPHE IDENTIFIER
 *
 * Examples:
 *
 *     'a
 *     'buffer
 *     'scope
 *     'input
 *
 * The lifetime name is preserved as source-level information.
 *
 * This grammar does NOT determine:
 *
 *     - lifetime validity;
 *     - lifetime relationships;
 *     - outlives relationships;
 *     - lifetime inference;
 *     - lifetime elision;
 *     - ownership;
 *     - borrowing;
 *     - aliasing;
 *     - concurrency safety.
 *
 * Those are semantic responsibilities.
 *
 * ============================================================================
 * IDENTIFIER CONTRACT
 * ============================================================================
 *
 * Lifetime names use the canonical IDENTIFIER token.
 *
 * This prevents the grammar from creating a second identifier vocabulary.
 *
 * Lifetime names are not a separate lexical namespace at this layer.
 *
 * Semantic analysis may impose the language's namespace and naming rules.
 *
 * ============================================================================
 * MUTABILITY CONTRACT
 * ============================================================================
 *
 * The `MUT` token is the canonical mutable-reference marker.
 *
 * This grammar does not decide whether a mutable reference is semantically
 * legal.
 *
 * For example:
 *
 *     &mut T
 *
 * is syntactically valid.
 *
 * Whether a particular expression may produce a mutable reference is a
 * semantic ownership/borrowing question.
 *
 * ============================================================================
 * REFERENCE VS POINTER
 * ============================================================================
 *
 * This file owns reference syntax only.
 *
 * Reference:
 *
 *     &T
 *     &mut T
 *     &'a T
 *     &'a mut T
 *
 * Pointer syntax is owned by:
 *
 *     grammar/types/pointer.g4
 *
 * Pointer examples:
 *
 *     *T
 *     *mut T
 *
 * The two constructs MUST NOT be merged into one parser rule merely because
 * both may eventually be represented using target-dependent indirection.
 *
 * ============================================================================
 * MEMORY CONTRACT
 * ============================================================================
 *
 * A reference is not an allocation instruction.
 *
 * Reference syntax does not imply:
 *
 *     stack allocation;
 *     heap allocation;
 *     static storage;
 *     virtual memory;
 *     physical memory;
 *     shared memory;
 *     device memory;
 *     persistent memory;
 *     distributed memory.
 *
 * Memory operations are owned by:
 *
 *     grammar/memory/
 *
 * A reference may describe an abstract relationship to a value regardless of
 * the eventual storage realization.
 *
 * ============================================================================
 * MEMORY-GRAMMAR INTEGRATION
 * ============================================================================
 *
 * Existing memory grammar owns memory-place and borrowing operations.
 *
 * Reference type syntax MUST NOT duplicate:
 *
 *     memoryPlace
 *     memoryBorrow
 *     allocation
 *     deallocation
 *     memory lifetime operations
 *
 * In particular:
 *
 *     grammar/memory/references.g4
 *
 * and related memory grammars remain downstream/source-operation concerns.
 *
 * A type reference and a memory borrow are related semantically but are not
 * interchangeable syntactic constructs.
 *
 * ============================================================================
 * GENERIC INTEGRATION
 * ============================================================================
 *
 * The referenced type is delegated to the canonical `Types.typeExpression`.
 *
 * Therefore the reference grammar does not need to enumerate generic forms.
 *
 * Valid examples include:
 *
 *     &Vec<T>
 *     &Map<K, V>
 *     &Result<T, E>
 *     &Tensor<T>
 *     &Tensor<T>[N, M]
 *
 * The actual generic and dependent-type syntax remains owned by `Types`.
 *
 * ============================================================================
 * QUALIFIED TYPE INTEGRATION
 * ============================================================================
 *
 * Qualified types are handled by the canonical type-expression grammar.
 *
 * Examples:
 *
 *     &quantum::State
 *     &hardware::Resource
 *     &distributed::Value
 *     &hdl::Signal
 *     &ai::Model
 *
 * This file does not need to know what those namespaces mean.
 *
 * ============================================================================
 * DOMAIN-NEUTRALITY CONTRACT
 * ============================================================================
 *
 * References can target any valid source-level type.
 *
 * This includes types representing:
 *
 *     classical values;
 *     quantum values;
 *     HDL structures;
 *     hardware abstractions;
 *     resources;
 *     accelerators;
 *     tensors;
 *     models;
 *     distributed values;
 *     network abstractions;
 *     user-defined types;
 *     future domain types.
 *
 * Domain-specific meaning is determined downstream.
 *
 * No domain-specific reference syntax is permitted here.
 *
 * ============================================================================
 * QUANTUM CONTRACT
 * ============================================================================
 *
 * Examples such as:
 *
 *     &Qubit
 *     &quantum::State
 *     &'a LogicalQubit
 *
 * remain source-level type constructs.
 *
 * They MUST NOT encode:
 *
 *     physical qubit IDs;
 *     QPU IDs;
 *     coupling maps;
 *     calibration;
 *     routing;
 *     scheduling;
 *     decomposition;
 *     error-correction implementation;
 *     ZQN representation;
 *     HAL representation.
 *
 * Quantum semantic lowering remains:
 *
 *     source
 *         |
 *         v
 *     frontend AST
 *         |
 *         v
 *     semantic quantum model
 *         |
 *         v
 *     quantum::ir
 *         |
 *         v
 *     optimization
 *         |
 *         v
 *     decomposition / routing
 *         |
 *         v
 *     scheduling
 *         |
 *         v
 *     resilience / QEC
 *         |
 *         v
 *     ZQN
 *         |
 *         v
 *     HAL
 *         |
 *         v
 *     target realization
 *
 * This grammar does not participate in physical quantum-resource selection.
 *
 * ============================================================================
 * HDL / HARDWARE CONTRACT
 * ============================================================================
 *
 * Reference syntax does not encode:
 *
 *     bus width;
 *     register width;
 *     register count;
 *     memory capacity;
 *     device count;
 *     topology size;
 *     pipeline depth;
 *     clock count;
 *     physical placement.
 *
 * Such information is either source-level type information owned elsewhere or
 * target information discovered downstream.
 *
 * ============================================================================
 * DISTRIBUTED CONTRACT
 * ============================================================================
 *
 * A reference to a distributed abstraction does not determine whether the
 * referenced value is:
 *
 *     local;
 *     remote;
 *     replicated;
 *     partitioned;
 *     serialized;
 *     migrated;
 *     synchronized.
 *
 * Those decisions belong to distributed semantics and execution.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY CONTRACT
 * ============================================================================
 *
 * A reference to a resource-like type does not itself acquire a resource.
 *
 * For example:
 *
 *     &hardware::Resource
 *
 * does not mean:
 *
 *     allocate hardware;
 *     select hardware;
 *     reserve hardware;
 *     discover hardware;
 *     bind a device.
 *
 * Resource requirements and capabilities remain owned by:
 *
 *     grammar/resources/
 *
 * and their semantic/runtime consumers.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Parsing a reference type produces no runtime effect.
 *
 * This grammar does not itself declare:
 *
 *     io;
 *     network;
 *     mutation;
 *     allocation;
 *     deallocation;
 *     measurement;
 *     randomness;
 *     native execution;
 *     FFI;
 *     learning;
 *     adaptation;
 *     reflection.
 *
 * If the semantic use of a reference creates or requires an effect, that is
 * determined downstream.
 *
 * ============================================================================
 * CONTRACT / POLICY CONTRACT
 * ============================================================================
 *
 * Reference syntax does not establish:
 *
 *     requires;
 *     ensures;
 *     invariant;
 *     assume;
 *     guarantee;
 *     property;
 *     policy;
 *     authorization.
 *
 * Those systems may constrain the use of a reference semantically.
 *
 * This grammar remains responsible only for parsing the reference syntax.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * The parser must preserve sufficient source structure for the frontend to
 * attach source spans to:
 *
 *     '&';
 *     lifetime annotation;
 *     'mut';
 *     referenced type.
 *
 * Provenance generation itself belongs to the frontend/compiler provenance
 * system.
 *
 * This grammar MUST NOT create timestamps, IDs, hashes or environment-derived
 * values.
 *
 * ============================================================================
 * POCO-REAF / SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar contains no language-level resource ceiling.
 *
 * It MUST NOT define limits for:
 *
 *     reference depth;
 *     reference count;
 *     pointer width;
 *     memory;
 *     CPU count;
 *     core count;
 *     thread count;
 *     GPU count;
 *     FPGA count;
 *     accelerator count;
 *     QPU count;
 *     qubit count;
 *     node count;
 *     device count;
 *     tensor rank;
 *     tensor dimensions;
 *     network size;
 *     program size.
 *
 * In particular, this file MUST NOT contain:
 *
 *     MAX_REFERENCE_DEPTH
 *     MAX_REFERENCE_COUNT
 *     MAX_POINTER_WIDTH
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QUBITS
 *     MAX_NODES
 *     MAX_DEVICES
 *
 * Recursive reference types are therefore structurally open-ended:
 *
 *     &T
 *     &&T
 *     &&&T
 *     &&&&T
 *     &mut &T
 *     &&mut T
 *     &'a &T
 *     &'a mut &T
 *
 * Practical compiler resource limits may exist for protection against
 * pathological input, but such limits belong to explicit compiler/resource
 * policy and MUST NOT become source-language semantics.
 *
 * ============================================================================
 * IMPLEMENTATION RESOURCE POLICY
 * ============================================================================
 *
 * A compiler implementation may need configurable protection against:
 *
 *     excessive parse-tree depth;
 *     excessive AST depth;
 *     excessive memory consumption;
 *     excessive compilation time;
 *     adversarial input.
 *
 * Such protection MUST:
 *
 *     1. be external to this grammar;
 *     2. be explicitly configurable where appropriate;
 *     3. produce diagnostics rather than silently changing semantics;
 *     4. not redefine the legal source language;
 *     5. not become a universal hardware ceiling.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing reference syntax must depend only on:
 *
 *     source text;
 *     canonical lexer vocabulary;
 *     parser grammar;
 *     language version;
 *     explicitly selected grammar configuration.
 *
 * It MUST NOT depend on:
 *
 *     CPU availability;
 *     GPU availability;
 *     QPU availability;
 *     memory topology;
 *     network state;
 *     filesystem state;
 *     wall-clock time;
 *     randomness;
 *     environment variables;
 *     scheduler state;
 *     deployment topology.
 *
 * ============================================================================
 * SAFETY CONTRACT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no embedded Rust;
 *     - no semantic predicates;
 *     - no target-language actions;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware discovery;
 *     - no runtime execution;
 *     - no unsafe code.
 *
 * The Rust frontend consuming the generated parser remains subject to:
 *
 *     Rust 2021
 *     Rust 1.97
 *     Rust 1.97.1
 *     safe Rust
 *     no unsafe
 *
 * ============================================================================
 * ERROR CONTRACT
 * ============================================================================
 *
 * Structural errors include incomplete reference prefixes such as:
 *
 *     &
 *     &mut
 *     &'
 *     &'a
 *     &' mut T
 *
 * The final type is required by the `Types.referenceType` composition rule.
 *
 * This grammar does not manufacture semantic error messages.
 *
 * Diagnostics belong to the parser/frontend diagnostic layer so that:
 *
 *     source span;
 *     expected token;
 *     actual token;
 *     language version;
 *     recovery information
 *
 * can be reported consistently.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Stable reference syntax is:
 *
 *     &T
 *     &mut T
 *     &'a T
 *     &'a mut T
 *
 * New reference modifiers MUST NOT be added directly to this grammar without
 * first defining:
 *
 *     specification;
 *     AST representation;
 *     semantic meaning;
 *     compatibility behavior;
 *     diagnostics;
 *     tests.
 *
 * A future modifier should extend the reference-specific component rather
 * than introduce a second reference grammar.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/lexer/
 *
 * EXPORTS:
 *
 *     referencePrefix
 *     lifetimeAnnotation
 *
 * CONSUMED_BY:
 *
 *     grammar/types/types.g4
 *
 * AST_OWNER:
 *
 *     Existing frontend `TypeExpr`
 *
 * SEMANTIC_OWNER:
 *
 *     Existing semantic type/ownership/lifetime analysis
 *
 * IR_OWNER:
 *
 *     Canonical semantic/IR lowering
 *
 * QUANTUM_IR_OWNER:
 *
 *     quantum::ir, when a referenced type participates in quantum semantics
 *
 * SPEC_OWNER:
 *
 *     grammar/spec/
 *     grammar/specification/
 *
 * TEST_OWNER:
 *
 *     grammar/tests/
 *     type/reference conformance tests
 *
 * COMPATIBILITY_OWNER:
 *
 *     grammar/compatibility/
 *
 * ============================================================================
 * REQUIRED TYPES.G4 INTEGRATION
 * ============================================================================
 *
 * `grammar/types/types.g4` must:
 *
 *     1. import `Reference`;
 *     2. remove its competing local `lifetimeAnnotation` rule;
 *     3. remove its old standalone reference implementation;
 *     4. compose the reference prefix with the canonical `typeExpression`.
 *
 * The resulting canonical rule is:
 *
 *     referenceType
 *         : referencePrefix
 *           typeExpression
 *         ;
 *
 * This makes `Types` the sole type-expression composition owner while
 * `Reference` remains the sole owner of reference-specific syntax.
 *
 * ============================================================================
 * REQUIRED IMPORT
 * ============================================================================
 *
 * In `grammar/types/types.g4`:
 *
 *     import Reference;
 *
 * No other grammar should import `Types` merely to obtain reference syntax.
 *
 * ============================================================================
 * REQUIRED TOKEN CONSISTENCY
 * ============================================================================
 *
 * This file deliberately uses:
 *
 *     MUT
 *
 * rather than:
 *
 *     K_MUT
 *
 * because the actual canonical type grammar currently uses `MUT`, and
 * introducing another spelling would create a lexical integration defect.
 *
 * Likewise this file uses:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * rather than:
 *
 *     tokenVocab = ZamaniTokens;
 *
 * because the actual canonical parser hierarchy consumes `ZamaniLexer`.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive parser cases:
 *
 *     &T
 *     &mut T
 *     &'a T
 *     &'a mut T
 *     &&T
 *     &&&T
 *     &mut &T
 *     &&mut T
 *     &'a &T
 *     &'a mut &T
 *
 * Generic:
 *
 *     &Vec<T>
 *     &Map<K, V>
 *     &Result<T, E>
 *     &Tensor<T>
 *
 * Qualified:
 *
 *     &quantum::State
 *     &hardware::Resource
 *     &distributed::Value
 *     &hdl::Signal
 *
 * Nested:
 *
 *     &&Vec<T>
 *     &'a &Result<T, E>
 *     &mut &'a T
 *
 * IMPORTANT:
 *
 * `&mut 'a T` is NOT a canonical positive test.
 *
 * Negative/incomplete cases:
 *
 *     &
 *     &mut
 *     &'
 *     &'
 *     &'a
 *     &' mut T
 *
 * Boundary cases:
 *
 *     reference to every canonical type family;
 *     nested references;
 *     references to generic types;
 *     references to qualified types;
 *     references to dependent/value-parameterized types;
 *     references to user-defined types;
 *     references to domain-defined types.
 *
 * Scalability cases:
 *
 *     generated nested reference structures;
 *     generated generic reference structures;
 *     large symbolic type expressions.
 *
 * Scalability tests MUST NOT assert an arbitrary maximum reference depth.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no:
 *
 *     machine width;
 *     pointer width;
 *     memory capacity;
 *     resource count;
 *     device count;
 *     quantum count;
 *     processor count;
 *     tensor rank limit;
 *     network size limit.
 *
 * Any future change introducing such a constant requires architectural review
 * because it would violate the language's portability contract.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     1. It is a parser grammar named `Reference`.
 *
 *     2. It consumes `ZamaniLexer`.
 *
 *     3. It defines no lexer rules.
 *
 *     4. It owns `referencePrefix`.
 *
 *     5. It owns `lifetimeAnnotation`.
 *
 *     6. It uses the canonical `MUT` token.
 *
 *     7. It uses the canonical `AMPERSAND` token.
 *
 *     8. It uses the canonical `APOSTROPHE` token.
 *
 *     9. It uses the canonical `IDENTIFIER` token.
 *
 *    10. It does not call `typeExpression`.
 *
 *    11. It can be imported by `Types` without a cyclic grammar dependency.
 *
 *    12. `Types.referenceType` composes the prefix with `typeExpression`.
 *
 *    13. `TypeExpr::Reference` remains the sole AST representation.
 *
 *    14. Pointer syntax remains owned by the pointer grammar.
 *
 *    15. Memory operations remain owned by the memory grammar.
 *
 *    16. Lifetime semantics remain downstream.
 *
 *    17. No target-specific representation is encoded.
 *
 *    18. No quantum hardware information is encoded.
 *
 *    19. No resource ceiling is encoded.
 *
 *    20. No machine-size constant is encoded.
 *
 *    21. Nested references remain structurally open-ended.
 *
 *    22. The grammar contains no actions, predicates or unsafe implementation.
 *
 *    23. Positive, negative, boundary and scalability tests exist.
 *
 *    24. Integration with the actual `Types` grammar is verified.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * CANONICAL PARSER GRAMMAR
 * ============================================================================
 */

parser grammar Reference;


/*
 * ============================================================================
 * CANONICAL LEXER VOCABULARY
 * ============================================================================
 *
 * All tokens are supplied by the repository's canonical lexer.
 */

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * REFERENCE PREFIX
 * ============================================================================
 *
 * This is the complete reference-specific portion of a reference type.
 *
 * The referenced type itself is deliberately NOT parsed here because
 * `typeExpression` belongs to the importing `Types` grammar.
 *
 * Canonical forms:
 *
 *     &
 *     &mut
 *     &'a
 *     &'a mut
 *
 * The enclosing `Types.referenceType` then appends:
 *
 *     typeExpression
 *
 * resulting in:
 *
 *     &T
 *     &mut T
 *     &'a T
 *     &'a mut T
 *
 * The grammar order is intentionally lifetime-before-mutability.
 */

referencePrefix
    : AMPERSAND
      lifetimeAnnotation?
      MUT?
    ;


/*
 * ============================================================================
 * EXPLICIT LIFETIME
 * ============================================================================
 *
 * Canonical source form:
 *
 *     'identifier
 *
 * Examples:
 *
 *     'a
 *     'buffer
 *     'scope
 *
 * The leading apostrophe is syntax.
 * The lifetime name is an ordinary canonical IDENTIFIER token.
 *
 * Semantic lifetime handling is downstream.
 */

lifetimeAnnotation
    : APOSTROPHE
      IDENTIFIER
    ;
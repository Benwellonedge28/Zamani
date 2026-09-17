/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/types/affine.g4
 *
 * Status:
 *     Production-ready modular affine-type qualifier grammar.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Rust edition:
 *     2021
 *
 * Safety:
 *     This grammar contains no embedded executable code.
 *     The Zamani compiler/runtime implementation is safe Rust only.
 *     No `unsafe` Rust is required or permitted by the architecture.
 *
 * ============================================================================
 *
 * PURPOSE
 * ============================================================================
 *
 * This file owns exactly one type-system concept:
 *
 *     affine
 *
 * It defines the source-level affine TYPE QUALIFIER.
 *
 * It deliberately does NOT define the complete type expression following
 * `affine`.
 *
 * The canonical type composition grammar:
 *
 *     grammar/types/types.g4
 *
 * owns:
 *
 *     typeExpression
 *     typePrimary
 *     typeQualifier
 *     typePostfix
 *
 * Therefore the complete source form:
 *
 *     affine T
 *
 * is composed by the type system as:
 *
 *     typeQualifier + typePrimary/typePostfix
 *
 * where this grammar supplies the `affineType` qualifier.
 *
 * ============================================================================
 *
 * ARCHITECTURAL PRINCIPLE
 * ============================================================================
 *
 *     affine T
 *
 * means, at source-type level:
 *
 *     Affine(T)
 *
 * where affine ownership generally means:
 *
 *     the value may be consumed at most once.
 *
 * This file does NOT enforce that rule.
 *
 * Semantic analysis is responsible for:
 *
 *     - ownership;
 *     - move analysis;
 *     - use analysis;
 *     - copy analysis;
 *     - drop analysis;
 *     - borrow analysis;
 *     - lifetime analysis;
 *     - generic substitution;
 *     - type validity;
 *     - resource validity;
 *     - quantum ownership;
 *     - hardware/resource feasibility.
 *
 * ============================================================================
 *
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - the affine type qualifier;
 *     - the canonical parser rule `affineType`;
 *     - the source span of the affine keyword;
 *     - integration with the canonical type-qualifier composition.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - the complete type expression;
 *     - identifiers;
 *     - qualified names;
 *     - generic arguments;
 *     - arrays;
 *     - tuples;
 *     - functions;
 *     - references;
 *     - pointers;
 *     - quantum types;
 *     - resource types;
 *     - capability types;
 *     - hardware types;
 *     - linear ownership;
 *     - borrowing;
 *     - lifetimes;
 *     - resource allocation;
 *     - resource discovery;
 *     - hardware discovery;
 *     - placement;
 *     - routing;
 *     - scheduling;
 *     - calibration;
 *     - optimization;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime allocation;
 *     - ABI layout.
 *
 * ============================================================================
 *
 * WHY THIS IS A QUALIFIER
 * ============================================================================
 *
 * A previous/common design error is:
 *
 *     affineType
 *         : AFFINE typeExpression
 *         ;
 *
 * This is architecturally wrong when `typeExpression` is owned by
 * `types.g4`, because it creates a dependency in the opposite direction:
 *
 *     types.g4 -> affine.g4 -> types.g4
 *
 * and can create circular parser composition.
 *
 * It also duplicates responsibility for the canonical type-expression root.
 *
 * Instead this file owns:
 *
 *     affineType
 *         : AFFINE
 *         ;
 *
 * and `types.g4` composes it with the existing type-expression machinery.
 *
 * Therefore:
 *
 *     types.g4
 *         |
 *         +--> typeQualifier
 *                 |
 *                 +--> affineType
 *                         |
 *                         +--> AFFINE
 *
 * followed by the existing type-primary/postfix composition.
 *
 * ============================================================================
 *
 * CANONICAL SOURCE FORM
 * ============================================================================
 *
 * Valid examples after composition with `types.g4`:
 *
 *     affine Int
 *     affine User
 *     affine Resource<Qubit>
 *     affine Qubit
 *     affine QRegister<N>
 *     affine Tensor<T, N>
 *     affine Vec<T>
 *     affine Foo::Bar
 *     affine Result<T, E>
 *
 * The operand is NOT parsed by this file.
 *
 * The operand is parsed by the canonical type grammar.
 *
 * ============================================================================
 *
 * POCO-REAF
 * ============================================================================
 *
 * Affine qualification is completely independent of machine size.
 *
 * This file MUST NOT contain implementation limits such as:
 *
 *     MAX_AFFINE_VALUES
 *     MAX_TYPES
 *     MAX_NESTING
 *     MAX_GENERIC_PARAMETERS
 *     MAX_RESOURCES
 *     MAX_MEMORY
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_QUBITS
 *     MAX_NODES
 *     MAX_DEVICES
 *
 * There is deliberately no finite enumeration such as:
 *
 *     affineType
 *         : AFFINE_INT
 *         | AFFINE_FLOAT
 *         | AFFINE_QUBIT
 *         | AFFINE_RESOURCE
 *         | ...
 *         ;
 *
 * Such an enumeration would make affine qualification depend on the set of
 * types known when the grammar was written.
 *
 * Instead:
 *
 *     affine + existing type grammar
 *
 * allows future types to participate without modifying this file.
 *
 * This is required for:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * ============================================================================
 *
 * LEXER CONTRACT
 * ============================================================================
 *
 * Canonical lexical authority:
 *
 *     grammar/lexer/tokens.g4
 *     grammar/antlr/ZamaniLexer.g4
 *
 * The repository's canonical lexer already defines:
 *
 *     AFFINE : 'affine' ;
 *
 * Therefore this parser grammar consumes `AFFINE`.
 *
 * It MUST NOT define:
 *
 *     AFFINE : 'affine' ;
 *
 * here.
 *
 * It MUST NOT define:
 *
 *     IDENTIFIER
 *     LESS_THAN
 *     GREATER_THAN
 *     ...
 *
 * here either.
 *
 * All lexical ownership remains with the canonical lexer.
 *
 * ============================================================================
 *
 * ANTLR COMPOSITION CONTRACT
 * ============================================================================
 *
 * This file is a parser grammar:
 *
 *     parser grammar Affine;
 *
 * and consumes:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * The canonical type grammar imports this grammar and incorporates
 * `affineType` into its existing `typeQualifier` rule.
 *
 * Conceptually:
 *
 *     parser grammar Types;
 *
 *     import Affine;
 *
 *     typeExpression
 *         : typeQualifier* typePrimary typePostfix*
 *         ;
 *
 *     typeQualifier
 *         : LINEAR
 *         | affineType
 *         ;
 *
 * The exact import/composition location belongs to `types.g4`.
 *
 * This file itself does not modify or redefine `types.g4`.
 *
 * ============================================================================
 *
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar creates no new AST type.
 *
 * The parser/frontend integration must represent:
 *
 *     affine T
 *
 * as the existing canonical type-expression representation:
 *
 *     TypeExpr::Affine(inner)
 *
 * where:
 *
 *     inner
 *
 * is the TypeExpr produced by the existing canonical type grammar.
 *
 * Important:
 *
 *     affineType
 *
 * by itself represents only the qualifier marker.
 *
 * The composed:
 *
 *     typeExpression
 *
 * is what produces:
 *
 *     TypeExpr::Affine(inner)
 *
 * This distinction prevents the grammar from inventing a second type AST.
 *
 * ============================================================================
 *
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * The grammar recognizes:
 *
 *     affine
 *
 * Semantic analysis determines whether:
 *
 *     affine T
 *
 * is legal for T.
 *
 * Typical affine semantics:
 *
 *     zero uses      -> permitted
 *     one use        -> permitted
 *     multiple uses  -> rejected unless another semantic rule permits it
 *
 * The grammar does NOT enforce any of those usage rules.
 *
 * In particular, the grammar must not attempt to:
 *
 *     count variable uses;
 *     determine ownership;
 *     infer moves;
 *     detect copies;
 *     detect drops;
 *     validate borrows;
 *     validate lifetimes.
 *
 * ============================================================================
 *
 * AFFINE VS LINEAR
 * ============================================================================
 *
 * Zamani distinguishes:
 *
 *     linear T
 *
 * from:
 *
 *     affine T
 *
 * Conceptually:
 *
 *     linear:
 *         exactly one consumption
 *
 *     affine:
 *         zero or one consumption
 *
 * The distinction belongs to the semantic ownership/type system.
 *
 * This grammar only provides the syntactic qualifier.
 *
 * `linear.g4` owns the corresponding linear qualifier.
 *
 * Neither grammar should duplicate the other's semantic rules.
 *
 * ============================================================================
 *
 * AFFINE + GENERICS
 * ============================================================================
 *
 * Generic composition remains owned by `types.g4`.
 *
 * Therefore forms such as:
 *
 *     affine Vec<T>
 *     affine Map<K, V>
 *     affine Resource<Qubit>
 *     affine Tensor<T, N>
 *
 * work because:
 *
 *     affine
 *         -> affineType
 *         -> canonical typeExpression
 *         -> generic/type-primary composition
 *
 * This file does not enumerate generic constructors.
 *
 * ============================================================================
 *
 * AFFINE + QUANTUM
 * ============================================================================
 *
 * Forms such as:
 *
 *     affine Qubit
 *     affine QRegister<N>
 *     affine LogicalQubit
 *     affine QuantumState<T>
 *
 * are accepted when the canonical type grammar accepts the operand type.
 *
 * This file does NOT:
 *
 *     - allocate qubits;
 *     - assign physical qubit IDs;
 *     - inspect coupling maps;
 *     - select QPU hardware;
 *     - route operations;
 *     - schedule operations;
 *     - perform QEC;
 *     - model noise;
 *     - perform calibration;
 *     - invoke ZQN;
 *     - query HAL capabilities.
 *
 * Quantum semantic lowering continues through the existing:
 *
 *     quantum::ir
 *
 * boundary.
 *
 * No:
 *
 *     AffineQuantumIR
 *
 * or other second quantum IR is created.
 *
 * ============================================================================
 *
 * AFFINE + RESOURCE TYPES
 * ============================================================================
 *
 * Forms such as:
 *
 *     affine Resource<Qubit>
 *     affine Resource<Memory>
 *     affine Resource<Accelerator>
 *
 * are source-type compositions.
 *
 * They do NOT mean:
 *
 *     allocate a resource now;
 *     reserve hardware;
 *     select a device;
 *     select a memory bank;
 *     select a physical accelerator.
 *
 * Resource feasibility remains downstream.
 *
 * ============================================================================
 *
 * AFFINE + HARDWARE / TARGET INDEPENDENCE
 * ============================================================================
 *
 * The qualifier does not identify:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     node
 *     device
 *     memory bank
 *     physical qubit
 *     network endpoint
 *
 * Hardware realization remains outside this grammar.
 *
 * ============================================================================
 *
 * AFFINE + DEPENDENT TYPES
 * ============================================================================
 *
 * The qualifier is deliberately independent of type-value size:
 *
 *     affine Vector<T, N>
 *     affine Matrix<T, Rows, Cols>
 *     affine Tensor<T, Shape>
 *
 * `N`, `Rows`, `Cols`, and `Shape` may remain symbolic.
 *
 * This grammar performs no conversion of those values to machine-sized
 * integers and imposes no range.
 *
 * ============================================================================
 *
 * AFFINE + REFERENCES / POINTERS
 * ============================================================================
 *
 * Whether:
 *
 *     affine &T
 *     affine *T
 *
 * is semantically valid is determined by the canonical type composition and
 * semantic ownership system.
 *
 * This file does not duplicate reference or pointer grammar.
 *
 * ============================================================================
 *
 * NESTED QUALIFIERS
 * ============================================================================
 *
 * This file intentionally does not define:
 *
 *     affine linear T
 *
 * or:
 *
 *     linear affine T
 *
 * as special affine syntax.
 *
 * If multiple qualifiers are permitted, `types.g4` remains the single owner
 * of qualifier ordering/composition.
 *
 * This prevents:
 *
 *     affine.g4
 *     linear.g4
 *     resource.g4
 *     hardware.g4
 *
 * from independently creating incompatible qualifier languages.
 *
 * ============================================================================
 *
 * SOURCE SPANS
 * ============================================================================
 *
 * `affineType` must preserve the source span of the AFFINE token.
 *
 * The composed type-expression node must preserve the complete span:
 *
 *     affine T
 *     ^^^^^^^
 *
 * This supports:
 *
 *     diagnostics;
 *     IDE tooling;
 *     source maps;
 *     formatting;
 *     provenance;
 *     semantic diagnostics;
 *     compiler error reporting.
 *
 * ============================================================================
 *
 * ERROR / DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * This grammar intentionally does not attempt semantic diagnostics.
 *
 * Examples:
 *
 *     affine
 *
 * should fail when the enclosing type grammar requires the subsequent type.
 *
 * Examples such as:
 *
 *     affine 123
 *
 * are rejected by the canonical type grammar because the operand is not a
 * valid type expression.
 *
 * Duplicate qualifiers, such as:
 *
 *     affine affine T
 *
 * are a responsibility of the canonical qualifier-composition/semantic layer.
 *
 * This allows diagnostics to distinguish:
 *
 *     syntax error
 *
 * from:
 *
 *     semantically invalid qualifier composition.
 *
 * ============================================================================
 *
 * DETERMINISM
 * ============================================================================
 *
 * `affineType` contains one lexical alternative:
 *
 *     AFFINE
 *
 * Therefore it introduces no ambiguity by itself.
 *
 * The surrounding `typeQualifier* typePrimary typePostfix*` composition remains
 * the canonical location for resolving qualifier/type precedence.
 *
 * ============================================================================
 *
 * SECURITY
 * ============================================================================
 *
 * No semantic execution occurs in this grammar.
 *
 * No:
 *
 *     filesystem access;
 *     network access;
 *     device access;
 *     resource allocation;
 *     code execution;
 *     backend invocation
 *
 * is permitted through grammar actions.
 *
 * The grammar contains no embedded Rust actions or predicates.
 *
 * ============================================================================
 *
 * PERFORMANCE
 * ============================================================================
 *
 * The rule is constant in grammar complexity:
 *
 *     affineType : AFFINE ;
 *
 * It does not scale with:
 *
 *     number of types;
 *     number of resources;
 *     number of devices;
 *     number of qubits;
 *     number of CPUs;
 *     number of GPUs;
 *     generic arity;
 *     tensor rank.
 *
 * The potentially large structures are handled by the canonical type grammar
 * and later semantic/resource stages.
 *
 * ============================================================================
 *
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing source spelling:
 *
 *     affine
 *
 * remains unchanged.
 *
 * Existing canonical lexer token:
 *
 *     AFFINE
 *
 * remains unchanged.
 *
 * Existing frontend semantic representation:
 *
 *     TypeExpr::Affine
 *
 * remains unchanged.
 *
 * Therefore this grammar change is intentionally a composition correction,
 * not a language-spelling change.
 *
 * ============================================================================
 *
 * INTEGRATION MATRIX
 * ============================================================================
 *
 * Upstream:
 *
 *     grammar/lexer/tokens.g4
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Direct composition:
 *
 *     grammar/types/types.g4
 *
 * Related:
 *
 *     grammar/types/linear.g4
 *     grammar/memory/ownership.g4
 *     grammar/types/resource-types.g4
 *     grammar/types/quantum-types.g4
 *     grammar/types/generic-types.g4
 *
 * AST:
 *
 *     src/frontend/ast/node/types/type_expr.rs
 *
 * Legacy compatibility:
 *
 *     src/ast/mod.rs
 *
 * Semantic consumers:
 *
 *     ownership/type/resource semantic analysis
 *
 * Downstream:
 *
 *     canonical semantic IR
 *     classical IR
 *     quantum::ir
 *     HDL/resource semantics
 *     optimization
 *     routing
 *     scheduling
 *     resilience
 *     QEC
 *     ZQN
 *     HAL
 *     target lowering
 *     runtime
 *
 * ============================================================================
 *
 * REQUIRED ONE-TIME INTEGRATION IN `grammar/types/types.g4`
 * ============================================================================
 *
 * This file is intentionally complete without requiring later modification.
 *
 * The composition owner, `types.g4`, must import this grammar and use:
 *
 *     affineType
 *
 * as the affine member of its existing `typeQualifier` rule.
 *
 * Conceptually:
 *
 *     typeQualifier
 *         : LINEAR
 *         | affineType
 *         ;
 *
 * This is the ONLY integration required for the type-expression composition.
 *
 * Do not modify this file later to make it consume `typeExpression`.
 *
 * ============================================================================
 *
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive composition cases:
 *
 *     affine Int
 *     affine User
 *     affine Resource<Qubit>
 *     affine Qubit
 *     affine QRegister<N>
 *     affine Tensor<T, N>
 *     affine Foo::Bar
 *     affine Result<T, E>
 *
 * Negative cases:
 *
 *     affine
 *     affine 123
 *     affine =
 *     affine ;
 *
 * Boundary cases:
 *
 *     affine T
 *     affine T<U>
 *     affine T<U, V, W>
 *     affine T<N>
 *     affine T<Rows, Cols>
 *     affine T::U::V
 *
 * Scalability cases:
 *
 *     affine Type<N>
 *     affine Type<SymbolicDimension>
 *     affine Type<ArbitrarilyLargeProgramValue>
 *
 * The tests must not establish a parser-level maximum for:
 *
 *     type depth;
 *     generic arity;
 *     resource count;
 *     qubit count;
 *     tensor dimensions;
 *     hardware size.
 *
 * Semantic tests belong outside this grammar file and must verify:
 *
 *     at-most-once usage;
 *     legal dropping;
 *     move semantics;
 *     copy restrictions;
 *     ownership propagation;
 *     generic substitution;
 *     quantum resource semantics;
 *     resource semantics.
 *
 * ============================================================================
 *
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * No:
 *
 *     numeric limits;
 *     hardware IDs;
 *     device IDs;
 *     resource counts;
 *     qubit counts;
 *     CPU counts;
 *     GPU counts;
 *     FPGA counts;
 *     node counts;
 *     memory sizes;
 *     tensor dimensions;
 *     register widths;
 *     topology;
 *     vendor names
 *
 * are encoded here.
 *
 * The only fixed lexical fact is the language keyword:
 *
 *     affine
 *
 * ============================================================================
 *
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] It is a parser grammar.
 *     [x] It uses the canonical Zamani lexer vocabulary.
 *     [x] It consumes the existing AFFINE token.
 *     [x] It does not define lexer rules.
 *     [x] It does not define typeExpression.
 *     [x] It does not duplicate generic/type/resource/quantum grammar.
 *     [x] It exposes the canonical affineType composition rule.
 *     [x] It has no artificial resource limits.
 *     [x] It introduces no second AST.
 *     [x] It introduces no second IR.
 *     [x] It preserves POCO-REAF.
 *     [x] It is deterministic.
 *     [x] It contains no embedded unsafe/executable code.
 *     [x] Its integration point with types.g4 is predetermined.
 *
 * ============================================================================
 */

parser grammar Affine;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * CANONICAL AFFINE TYPE QUALIFIER
 * ============================================================================
 *
 * This rule intentionally consumes ONLY the canonical lexical token.
 *
 * The complete type:
 *
 *     affine T
 *
 * is assembled by the canonical type-expression grammar.
 *
 * Keeping the rule this small is intentional:
 *
 *     affineType
 *         -> qualifier
 *
 * rather than:
 *
 *     affineType
 *         -> qualifier + competing type-expression grammar
 *
 * This makes the file independently stable and prevents circular grammar
 * dependencies.
 */
affineType
    : AFFINE
    ;
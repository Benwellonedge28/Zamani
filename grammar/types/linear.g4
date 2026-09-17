/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/types/linear.g4
 *
 * Grammar:
 *     LinearTypes
 *
 * Status:
 *     Production-ready modular SOURCE-TYPE grammar component.
 *
 * Purpose:
 *     Defines the source-level syntax for linear and affine type qualifiers.
 *
 * ============================================================================
 * ARCHITECTURAL CONTRACT
 * ============================================================================
 *
 * This file is the sole modular grammar owner for the lexical-to-syntactic
 * interpretation of the Zamani linearity keywords:
 *
 *     linear
 *     affine
 *
 * It deliberately does NOT own the complete type-expression grammar.
 *
 * Complete type-expression composition remains owned by:
 *
 *     grammar/types/types.g4
 *
 * This avoids a circular ANTLR dependency:
 *
 *     Types -> LinearTypes
 *
 * while preventing:
 *
 *     LinearTypes -> Types -> LinearTypes
 *
 * The integration boundary is therefore:
 *
 *     linear.g4
 *          |
 *          v
 *     linearityQualifier
 *          |
 *          v
 *     types.g4
 *          |
 *          v
 *     typeExpression
 *          |
 *          v
 *     frontend AST TypeExpr
 *          |
 *          v
 *     semantic type analysis
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +-----------------------------+
 *          |                             |
 *          v                             v
 *     classical/resource            quantum::ir
 *       semantics                    semantics
 *          |                             |
 *          +-------------+---------------+
 *                        |
 *                        v
 *                 canonical IR
 *                        |
 *             optimization / lowering
 *                        |
 *          routing / scheduling / resilience
 *                        |
 *                    QEC / ZQN
 *                        |
 *                       HAL
 *                        |
 *                 target realization
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - linearQualifier;
 *   - affineQualifier;
 *   - linearityQualifier;
 *   - source-level distinction between `linear` and `affine`;
 *   - syntactic ownership/linearity qualification.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - lexer token definitions;
 *   - identifier syntax;
 *   - keywords;
 *   - punctuation;
 *   - complete typeExpression;
 *   - primitive types;
 *   - named types;
 *   - generic types;
 *   - tuples;
 *   - arrays;
 *   - slices;
 *   - references;
 *   - pointers;
 *   - functions;
 *   - optional types;
 *   - result types;
 *   - dependent types;
 *   - quantum types;
 *   - classical types;
 *   - HDL types;
 *   - resource types;
 *   - capability types;
 *   - hardware types;
 *   - type aliases;
 *   - generic parameters;
 *   - name resolution;
 *   - type inference;
 *   - type checking;
 *   - ownership checking;
 *   - borrow checking;
 *   - resource allocation;
 *   - hardware discovery;
 *   - physical resource selection;
 *   - quantum allocation;
 *   - routing;
 *   - scheduling;
 *   - calibration;
 *   - QEC;
 *   - ZQN;
 *   - HAL;
 *   - compiler backend selection;
 *   - runtime representation;
 *   - ABI layout.
 *
 * ============================================================================
 * AUTHORITATIVE SPECIFICATION
 * ============================================================================
 *
 * The semantic meaning of linear and affine types is NOT defined here.
 *
 * This grammar is subordinate to:
 *
 *     grammar/specification/types.md
 *     grammar/spec/type-system.md
 *
 * Those documents define type meaning and ownership/resource semantics.
 *
 * This file defines only how the programmer spells the corresponding
 * source-level qualifiers.
 *
 * ============================================================================
 * CANONICAL AST CONTRACT
 * ============================================================================
 *
 * This grammar does not construct AST nodes itself.
 *
 * The canonical frontend representation remains:
 *
 *     src/frontend/ast/node/types/type_expr.rs
 *
 * The semantic type representation is downstream.
 *
 * A complete source-level type such as:
 *
 *     linear Buffer
 *
 * is structurally represented as a type expression whose linearity qualifier
 * is subsequently lowered/validated as:
 *
 *     TypeExpr::Linear(...)
 *
 * Likewise:
 *
 *     affine Buffer
 *
 * maps to the canonical:
 *
 *     TypeExpr::Affine(...)
 *
 * The exact AST construction mechanism belongs to the parser/frontend AST
 * adapter and MUST NOT be duplicated inside this grammar.
 *
 * ============================================================================
 * WHY THIS FILE DOES NOT PARSE THE OPERAND
 * ============================================================================
 *
 * A tempting design would be:
 *
 *     linearType
 *         : LINEAR typeExpression
 *         ;
 *
 * That is intentionally NOT used here.
 *
 * If `types.g4` imports this grammar, that design would require this grammar
 * to import `types.g4` in order to resolve `typeExpression`, producing a
 * circular parser-grammar dependency.
 *
 * Instead:
 *
 *     linearQualifier
 *         : LINEAR
 *         ;
 *
 * and:
 *
 *     affineQualifier
 *         : AFFINE
 *         ;
 *
 * are composed by the canonical `typeExpression` grammar.
 *
 * This makes this file independently complete and composable.
 *
 * ============================================================================
 * SOURCE FORMS
 * ============================================================================
 *
 * The canonical prefix forms are:
 *
 *     linear T
 *     affine T
 *
 * where `T` is supplied by the surrounding type-expression grammar.
 *
 * Examples:
 *
 *     linear Resource
 *     affine Resource
 *     linear quantum::Qubit
 *     affine quantum::Qubit
 *     linear Buffer<T>
 *     affine Buffer<T>
 *
 * The operand is deliberately not restricted to a fixed domain.
 *
 * A linearity qualifier may therefore apply to any semantically valid type
 * whose type-system contract permits that qualifier.
 *
 * ============================================================================
 * LINEAR SEMANTIC INTENT
 * ============================================================================
 *
 * `linear` expresses a source-level ownership/resource discipline in which the
 * relevant value/resource must obey its declared linear usage contract.
 *
 * This file does NOT decide:
 *
 *     - exactly how many times a value may be consumed;
 *     - whether copying is legal;
 *     - whether moving is legal;
 *     - whether destruction counts as consumption;
 *     - whether a resource is quantum;
 *     - whether a resource is hardware-backed;
 *     - whether a resource is distributed;
 *     - whether a resource requires a capability;
 *     - how the compiler implements the discipline.
 *
 * Those are semantic/type-system questions.
 *
 * ============================================================================
 * AFFINE SEMANTIC INTENT
 * ============================================================================
 *
 * `affine` expresses a source-level ownership/resource discipline in which
 * unused values/resources are permitted while prohibited duplication/reuse
 * remains a semantic concern.
 *
 * This grammar does not encode those rules.
 *
 * Semantic analysis owns:
 *
 *     use checking
 *     move checking
 *     duplication checking
 *     destruction checking
 *     lifetime checking
 *     resource-flow checking
 *
 * ============================================================================
 * LINEAR VS AFFINE
 * ============================================================================
 *
 * The grammar intentionally keeps the two qualifiers distinct:
 *
 *     linearQualifier : LINEAR ;
 *
 *     affineQualifier : AFFINE ;
 *
 * This prevents semantic meaning from being inferred from arbitrary
 * identifier spelling and gives diagnostics/tooling a stable syntactic
 * category.
 *
 * ============================================================================
 * MUTUAL EXCLUSIVITY
 * ============================================================================
 *
 * `linear` and `affine` are alternative ownership disciplines.
 *
 * A complete type expression MUST NOT be treated as having both:
 *
 *     linear
 *     affine
 *
 * ownership qualifiers simultaneously unless a future language specification
 * explicitly defines such composition.
 *
 * The grammar therefore exposes a single:
 *
 *     linearityQualifier
 *
 * choice.
 *
 * Structural/semantic validation should diagnose duplicate ownership
 * qualifiers if a surrounding qualifier system permits repeated qualifiers.
 *
 * ============================================================================
 * QUALIFIER COMPOSITION
 * ============================================================================
 *
 * Other type qualifiers may exist in Zamani.
 *
 * Examples include concepts associated with:
 *
 *     mutability
 *     constness
 *     purity
 *     effects
 *     capabilities
 *     resource policies
 *     temporal properties
 *
 * This file MUST NOT absorb those concepts.
 *
 * Instead:
 *
 *     typeQualifier
 *         |
 *         +-- linearityQualifier
 *         +-- other canonical qualifier
 *         +-- ...
 *
 * remains composed by `types.g4`.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Linear and affine types are particularly important for quantum/resource
 * semantics, but this grammar remains domain-neutral.
 *
 * Examples:
 *
 *     linear Qubit
 *     affine Qubit
 *     linear quantum::Qubit
 *     linear quantum::LogicalQubit
 *
 * are syntactically ordinary type expressions.
 *
 * This grammar MUST NOT decide:
 *
 *     physical qubit identity
 *     QPU selection
 *     coupling topology
 *     gate decomposition
 *     routing
 *     scheduling
 *     calibration
 *     noise model
 *     error-correction implementation
 *
 * Quantum semantic lowering ultimately crosses the existing canonical:
 *
 *     quantum::ir
 *
 * boundary.
 *
 * No second quantum IR may be introduced for linear types.
 *
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * Linear/affine qualifiers may apply to resource abstractions such as:
 *
 *     linear Resource<T>
 *     affine Resource<T>
 *     linear Memory<T, N>
 *     linear Accelerator<A>
 *     linear quantum::Qubit
 *
 * The grammar does not determine whether the requested resource exists.
 *
 * Resource availability is resolved by:
 *
 *     semantic analysis
 *     capability analysis
 *     resource management
 *     compiler/runtime target realization
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * This grammar contains NO implementation capacity limits.
 *
 * It MUST NOT introduce:
 *
 *     MAX_LINEAR_VALUES
 *     MAX_AFFINE_VALUES
 *     MAX_LINEAR_RESOURCES
 *     MAX_AFFINE_RESOURCES
 *     MAX_RESOURCE_COUNT
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_DEVICES
 *     MAX_TYPE_DEPTH
 *     MAX_GENERIC_ARITY
 *
 * A linear or affine type may therefore participate in programs ranging from
 * very small embedded computations to arbitrarily large computations subject
 * only to actual semantic validity and available compilation/execution
 * resources.
 *
 * "Unbounded" means that this grammar imposes no artificial finite language
 * ceiling. It does not claim that physical machines have infinite resources.
 *
 * ============================================================================
 * NO HARD-CODED HARDWARE
 * ============================================================================
 *
 * This file must never encode:
 *
 *     cpu0
 *     gpu0
 *     qpu0
 *     device0
 *     physical_qubit_0
 *     node0
 *
 * as special ownership types or qualifiers.
 *
 * Such identifiers remain source identifiers.
 *
 * Hardware realization occurs downstream.
 *
 * ============================================================================
 * LEXER INTEGRATION
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * It defines NO lexer rules.
 *
 * The canonical lexer already owns:
 *
 *     LINEAR
 *     AFFINE
 *
 * through:
 *
 *     grammar/lexer/keywords.g4
 *
 * The canonical lexer composition is responsible for assembling those tokens
 * into the production token vocabulary.
 *
 * This grammar must never redefine:
 *
 *     LINEAR
 *     AFFINE
 *
 * locally.
 *
 * Likewise, it must not create aliases such as:
 *
 *     LINEAR_KEYWORD
 *     AFFINE_KEYWORD
 *
 * because those would create competing token identities.
 *
 * ============================================================================
 * TOKEN CONTRACT
 * ============================================================================
 *
 * Required canonical parser-visible tokens:
 *
 *     LINEAR
 *     AFFINE
 *
 * No additional token is required by this grammar.
 *
 * The grammar deliberately does not depend on:
 *
 *     IDENTIFIER
 *     DOUBLE_COLON
 *     LESS_THAN
 *     GREATER_THAN
 *     QUESTION_MARK
 *     AMPERSAND
 *     STAR
 *
 * because the operand type belongs to `types.g4`.
 *
 * This sharply minimizes lexical coupling and makes this file independently
 * completable.
 *
 * ============================================================================
 * ANTLR INTEGRATION
 * ============================================================================
 *
 * The grammar is a parser grammar:
 *
 *     parser grammar LinearTypes;
 *
 * Its token vocabulary must be the canonical composed Zamani lexer vocabulary.
 *
 * The production lexer architecture documented in:
 *
 *     grammar/lexer/tokens.g4
 *
 * is the lexical composition authority.
 *
 * If the repository's final generated parser vocabulary is exposed through:
 *
 *     ZamaniLexer
 *
 * then the generated token vocabulary adapter MUST map the canonical
 * `LINEAR`/`AFFINE` tokens without changing their language identity.
 *
 * This file itself must not introduce a second lexer.
 *
 * ============================================================================
 * TYPES.G4 INTEGRATION
 * ============================================================================
 *
 * `grammar/types/types.g4` remains the public type-expression composition
 * grammar.
 *
 * It should import this grammar and delegate:
 *
 *     typeQualifier
 *         : linearityQualifier
 *         | ...
 *         ;
 *
 * or an equivalent canonical qualifier-composition rule.
 *
 * It MUST NOT independently redeclare:
 *
 *     LINEAR
 *     AFFINE
 *
 * as parser alternatives.
 *
 * The important ownership boundary is:
 *
 *     linear.g4
 *         -> linearityQualifier
 *
 *     types.g4
 *         -> typeExpression composition
 *
 *     frontend AST
 *         -> TypeExpr::Linear / TypeExpr::Affine
 *
 * ============================================================================
 * ROOT-GRAMMAR INTEGRATION
 * ============================================================================
 *
 * `grammar/Zamani.g4` remains the language composition root.
 *
 * It must consume the canonical `typeExpression` from `types.g4`.
 *
 * It must NOT:
 *
 *     - define another linear type grammar;
 *     - define another affine type grammar;
 *     - create a second ownership-type AST;
 *     - inspect hardware;
 *     - perform ownership checking.
 *
 * ============================================================================
 * MEMORY INTEGRATION
 * ============================================================================
 *
 * `grammar/memory/ownership.g4` may describe ownership-related syntax outside
 * the type-expression position.
 *
 * It must NOT create a competing definition of:
 *
 *     linearQualifier
 *     affineQualifier
 *
 * inside the type grammar.
 *
 * Its relationship is:
 *
 *     memory/ownership.g4
 *          |
 *          v
 *     ownership-related language constructs
 *
 * while:
 *
 *     types/linear.g4
 *          |
 *          v
 *     type-level linearity qualification
 *
 * If an ownership declaration needs a linear/affine qualifier, it should
 * delegate to this canonical type-level qualifier where the syntax is a type
 * qualifier.
 *
 * ============================================================================
 * SEMANTIC INTEGRATION
 * ============================================================================
 *
 * The grammar produces syntax.
 *
 * Semantic analysis is responsible for:
 *
 *     - determining whether the operand type supports linearity;
 *     - tracking ownership;
 *     - detecting illegal duplication;
 *     - detecting illegal reuse;
 *     - checking consumption;
 *     - checking moves;
 *     - checking destruction;
 *     - checking lifetime interactions;
 *     - checking resource flow;
 *     - checking capability requirements;
 *     - checking cross-domain constraints.
 *
 * Errors such as:
 *
 *     LINEAR_RESOURCE_REUSED
 *     LINEAR_RESOURCE_NOT_CONSUMED
 *     INVALID_OWNERSHIP
 *
 * are semantic diagnostics, not lexical diagnostics.
 *
 * ============================================================================
 * TYPE-EXPRESSION EXAMPLES
 * ============================================================================
 *
 * Valid source-level forms, subject to semantic type validity:
 *
 *     linear Buffer
 *     affine Buffer
 *     linear quantum::Qubit
 *     affine quantum::LogicalQubit
 *     linear Resource<T>
 *     affine Resource<T>
 *     linear Memory<T, N>
 *
 * The grammar intentionally does not care whether these names denote:
 *
 *     classical resources
 *     quantum resources
 *     HDL resources
 *     distributed resources
 *     accelerator resources
 *     future resources
 *
 * ============================================================================
 * INVALID SYNTAX EXAMPLES
 * ============================================================================
 *
 * These are not valid uses of the linearity qualifier grammar:
 *
 *     linear
 *     affine
 *
 * when a complete `typeExpression` requires an operand.
 *
 * However, recognition of the keyword itself remains the lexical responsibility
 * of the lexer.
 *
 * Additional malformed combinations are handled by the surrounding type
 * grammar and parser recovery.
 *
 * ============================================================================
 * SEMANTICALLY INVALID BUT SYNTACTICALLY VALID EXAMPLES
 * ============================================================================
 *
 * Examples:
 *
 *     linear int
 *
 * may be syntactically valid but semantically invalid if the type system
 * specifies that `int` is not a resource-bearing linear type.
 *
 * Likewise:
 *
 *     affine bool
 *
 * may be syntactically recognized while semantic analysis decides whether the
 * qualification is meaningful.
 *
 * This distinction is deliberate.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * The grammar contains:
 *
 *     no semantic predicates;
 *     no target-dependent alternatives;
 *     no runtime actions;
 *     no filesystem operations;
 *     no network operations;
 *     no hardware discovery;
 *     no mutable parser state;
 *     no generated identifiers.
 *
 * Given the same token sequence, the same linearity parse structure is
 * produced.
 *
 * ============================================================================
 * SOURCE SPANS / DIAGNOSTICS
 * ============================================================================
 *
 * Because `LINEAR` and `AFFINE` are canonical lexer tokens, the parser can
 * preserve their source spans for:
 *
 *     diagnostics
 *     IDE highlighting
 *     formatting
 *     semantic ownership errors
 *     source maps
 *     provenance
 *
 * The grammar must not normalize or replace the source spelling.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * This grammar:
 *
 *     - performs no I/O;
 *     - executes no source code;
 *     - performs no hardware discovery;
 *     - performs no network access;
 *     - performs no filesystem access;
 *     - contains no semantic actions;
 *     - contains no unsafe code;
 *     - contains no target-specific behavior.
 *
 * The Rust compiler/toolchain integration remains:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *     stable Rust
 *     no unsafe
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * The following parser-level tests should be covered through the canonical
 * `typeExpression` entry point.
 *
 * Positive:
 *
 *     linear T
 *     affine T
 *     linear quantum::Qubit
 *     affine quantum::LogicalQubit
 *     linear Resource<T>
 *     affine Resource<T>
 *     linear Memory<T, N>
 *     affine Buffer
 *
 * Boundary:
 *
 *     linear T
 *     affine T
 *     deeply qualified type after linear
 *     deeply nested generic type after affine
 *     symbolic resource type after linear
 *
 * Scalability:
 *
 *     linear T
 *     linear A::B::C::D::T
 *     linear Resource<...>
 *     linear type expressions whose semantic resource cardinalities are
 *     represented symbolically rather than by grammar limits
 *
 * Negative parser cases:
 *
 *     linear
 *     affine
 *     linear affine T
 *     affine linear T
 *
 * The final status of the last two cases depends on the complete qualifier
 * composition grammar. They MUST NOT silently acquire two conflicting
 * ownership disciplines.
 *
 * Semantic negative cases:
 *
 *     linear int
 *     affine bool
 *
 * only if the semantic type system declares those operands ineligible.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Historical monolithic grammar forms such as:
 *
 *     linearType : 'linear' typeExpr ;
 *     affineType : 'affine' typeExpr ;
 *
 * are represented by the modular qualifier contract here.
 *
 * The source spelling remains:
 *
 *     linear <type-expression>
 *     affine <type-expression>
 *
 * so this modularization does not require a source-language rename.
 *
 * Legacy parser grammars such as:
 *
 *     grammar/antlr/ZamaniParser.g4
 *     grammar/antlr/Types.g4
 *
 * must not remain competing production owners.
 *
 * They should either:
 *
 *     - delegate to the canonical modular grammar;
 *     - be generated compatibility artifacts; or
 *     - be explicitly marked obsolete.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains:
 *
 *     ZERO hardware limits
 *     ZERO machine-size constants
 *     ZERO fixed resource counts
 *     ZERO fixed quantum counts
 *     ZERO physical identifiers
 *     ZERO vendor identifiers
 *     ZERO backend assumptions
 *     ZERO topology assumptions
 *     ZERO runtime assumptions
 *
 * The only fixed spellings are the language-level semantic keywords:
 *
 *     linear
 *     affine
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *   [x] LINEAR is consumed from the canonical lexer.
 *   [x] AFFINE is consumed from the canonical lexer.
 *   [x] No lexer rule is defined here.
 *   [x] linearQualifier is owned here.
 *   [x] affineQualifier is owned here.
 *   [x] linearityQualifier is owned here.
 *   [x] Complete typeExpression remains owned by types.g4.
 *   [x] No circular grammar dependency is required.
 *   [x] No identifier token is duplicated.
 *   [x] No hardware limit is encoded.
 *   [x] No quantum physical realization is encoded.
 *   [x] No resource allocation is encoded.
 *   [x] No ownership checking is encoded.
 *   [x] AST responsibility remains with the canonical frontend AST.
 *   [x] TypeExpr::Linear / TypeExpr::Affine remain semantic/source AST
 *       integration targets.
 *   [x] quantum::ir remains the canonical quantum semantic boundary.
 *   [x] Rust 1.97 / 1.97.1 compatibility is preserved.
 *   [x] No unsafe Rust is introduced.
 *   [x] The file is independently understandable and completable.
 *
 * ============================================================================
 */

parser grammar LinearTypes;

options {
    /*
     * Canonical lexical vocabulary.
     *
     * Do NOT replace this with a local lexer grammar and do NOT redefine
     * LINEAR/AFFINE in this file.
     */
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * 1. PUBLIC LINEARITY QUALIFIER
 * ========================================================================== */

/**
 * Canonical linear ownership qualifier.
 *
 * This rule owns the parser-level interpretation of the `linear` keyword.
 *
 * It deliberately does not consume the operand type.
 *
 * The surrounding canonical `typeExpression` grammar supplies that operand.
 */
linearQualifier
    : LINEAR
    ;


/* ============================================================================
 * 2. PUBLIC AFFINE QUALIFIER
 * ========================================================================== */

/**
 * Canonical affine ownership qualifier.
 *
 * This rule owns the parser-level interpretation of the `affine` keyword.
 *
 * It deliberately does not consume the operand type.
 */
affineQualifier
    : AFFINE
    ;


/* ============================================================================
 * 3. UNIFIED LINEARITY QUALIFIER
 * ========================================================================== */

/**
 * Canonical ownership-linearity choice.
 *
 * Exactly one of the two source-level ownership disciplines is selected at
 * this syntactic position:
 *
 *     linear
 *     affine
 *
 * The operand remains owned by `types.g4`.
 */
linearityQualifier
    : linearQualifier
    | affineQualifier
    ;
/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/expressions/patterns.g4
 *
 * GRAMMAR
 * -------
 * Patterns
 *
 * STATUS
 * ------
 * CANONICAL SHARED PATTERN-SYNTAX GRAMMAR
 *
 * PURPOSE
 * -------
 * This file is the single reusable source-syntax owner for Zamani patterns.
 *
 * It is deliberately placed under expressions/ because patterns are reusable
 * expression-level syntax, while the same pattern language is consumed by:
 *
 *     - match expressions;
 *     - match statements;
 *     - guards/conditional matching where applicable;
 *     - future pattern-oriented language constructs.
 *
 * This file owns PATTERN SYNTAX.
 *
 * It does NOT own:
 *
 *     - match-expression syntax;
 *     - match-statement syntax;
 *     - match-arm body syntax;
 *     - general expression precedence;
 *     - statement syntax;
 *     - type-system semantics;
 *     - name resolution;
 *     - exhaustiveness;
 *     - reachability;
 *     - overlap analysis;
 *     - binding compatibility;
 *     - ownership;
 *     - borrowing;
 *     - effects;
 *     - capabilities;
 *     - resources;
 *     - policies;
 *     - provenance;
 *     - IR;
 *     - optimization;
 *     - routing;
 *     - scheduling;
 *     - quantum realization;
 *     - HDL realization;
 *     - hardware selection;
 *     - runtime execution.
 *
 * ============================================================================
 * IMPLEMENTATION BASELINE
 * ============================================================================
 *
 * Rust:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Edition 2021
 *     Safe Rust only
 *     No unsafe Rust
 *
 * ANTLR:
 *
 *     ANTLR4 parser grammar
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     source
 *        |
 *        v
 *     ZamaniLexer
 *        |
 *        v
 *     parser
 *        |
 *        +----------------------+
 *        |                      |
 *        v                      v
 *     expression             pattern
 *        |                      |
 *        +----------+-----------+
 *                   |
 *                   v
 *           domain-neutral AST
 *                   |
 *                   v
 *          structural validation
 *                   |
 *                   v
 *          semantic analysis
 *                   |
 *          +--------+---------+
 *          |        |         |
 *          v        v         v
 *        types    effects   resources
 *          |        |         |
 *          +--------+---------+
 *                   |
 *                   v
 *          canonical semantic model
 *                   |
 *          +--------+---------+
 *          |        |         |
 *          v        v         v
 *      classical quantum::ir HDL/hardware
 *          |        |         |
 *          +--------+---------+
 *                   |
 *                   v
 *        optimization/lowering
 *                   |
 *            routing/scheduling
 *                   |
 *          resilience/QEC/ZQN
 *                   |
 *                  HAL
 *                   |
 *             target realization
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * There MUST be exactly one authoritative effective `pattern` rule in the
 * assembled production parser.
 *
 * This file owns:
 *
 *     pattern
 *     patternAtom
 *     wildcardPattern
 *     bindingPattern
 *     literalPattern
 *     tuplePattern
 *     sequencePattern
 *     sequencePatternElements
 *     sequencePatternElement
 *     restPattern
 *     structPattern
 *     structPatternFieldList
 *     structPatternField
 *     variantPattern
 *     rangePattern
 *     rangePatternEndpoint
 *     orPattern
 *     patternAlternative
 *     referencePattern
 *     typePattern
 *     parenthesizedPattern
 *     patternList
 *     patternListTail
 *
 * No other grammar may redefine these rules as an independent implementation.
 *
 * In particular:
 *
 *     grammar/statements/pattern-matching.g4
 *
 * must consume these rules rather than define a competing pattern grammar.
 *
 * ============================================================================
 * IMPORTANT INTEGRATION RULE
 * ============================================================================
 *
 * This file intentionally does NOT define:
 *
 *     matchExpression
 *     matchStatement
 *     matchArm
 *     guardClause
 *
 * Those constructs belong to their respective match composition grammar.
 *
 * This separation prevents:
 *
 *     expression match
 *             +
 *     statement match
 *
 * from producing two pattern languages.
 *
 * The intended dependency direction is:
 *
 *     Patterns
 *        ^
 *        |
 *     MatchExpressions
 *        ^
 *        |
 *     expression composition
 *
 * and:
 *
 *     Patterns
 *        ^
 *        |
 *     PatternMatching
 *        ^
 *        |
 *     Statements
 *
 * `Patterns` MUST NOT import the complete statement grammar.
 *
 * ============================================================================
 * LEXER AUTHORITY
 * ============================================================================
 *
 * All tokens are supplied by:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * which is backed by:
 *
 *     grammar/lexer/
 *
 * This file contains NO lexer rules.
 *
 * This file MUST NOT invent parser-side aliases for lexer tokens.
 *
 * Canonical tokens used here include:
 *
 *     UNDERSCORE
 *     AMPERSAND
 *     PIPE
 *     COLON
 *     COMMA
 *     DOT_DOT
 *     DOT_DOT_EQ
 *     LPAREN
 *     RPAREN
 *     LBRACKET
 *     RBRACKET
 *     LBRACE
 *     RBRACE
 *
 * Literal and identifier/type tokens are consumed through their canonical
 * parser rules rather than being duplicated here.
 *
 * ============================================================================
 * TOKEN CORRECTIONS
 * ============================================================================
 *
 * The previous pattern implementation used:
 *
 *     RANGE_EXCLUSIVE
 *     RANGE_INCLUSIVE
 *
 * Those are NOT the canonical lexical tokens.
 *
 * The canonical operator grammar defines:
 *
 *     DOT_DOT     -> ..
 *     DOT_DOT_EQ  -> ..=
 *
 * Therefore range patterns MUST use:
 *
 *     DOT_DOT
 *     DOT_DOT_EQ
 *
 * This file does not introduce:
 *
 *     RANGE_EXCLUSIVE
 *     RANGE_INCLUSIVE
 *
 * ============================================================================
 * POCO-REAF / SCALABILITY
 * ============================================================================
 *
 * Pattern syntax contains no universal finite machine limits.
 *
 * In particular, this grammar MUST NOT impose limits on:
 *
 *     pattern count
 *     tuple arity
 *     sequence length
 *     OR alternatives
 *     struct fields
 *     nesting depth
 *     range endpoint magnitude
 *     value width
 *     tensor dimensions
 *     tensor rank
 *     quantum count
 *     CPU count
 *     GPU count
 *     FPGA count
 *     node count
 *     memory capacity
 *     device count
 *
 * There are deliberately no:
 *
 *     MAX_PATTERN_DEPTH
 *     MAX_PATTERN_FIELDS
 *     MAX_PATTERN_ELEMENTS
 *     MAX_OR_ALTERNATIVES
 *     MAX_TUPLE_ARITY
 *     MAX_SEQUENCE_LENGTH
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *
 * Any implementation resource limitation is external to the language
 * definition and belongs to compiler/runtime resource policy.
 *
 * "Scale to infinity" therefore means:
 *
 *     no artificial language-level finite ceiling.
 *
 * It does NOT mean that a finite machine is expected to possess infinite
 * memory, time, storage, or processing capacity.
 *
 * ============================================================================
 * TARGET INDEPENDENCE
 * ============================================================================
 *
 * Patterns describe source-level semantic structure.
 *
 * They MUST NOT select:
 *
 *     physical CPU
 *     CPU core
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     QPU
 *     physical qubit
 *     memory bank
 *     network node
 *     device address
 *     hardware topology
 *
 * A pattern can match a semantic value representing any of those concepts,
 * but the pattern grammar itself remains unaware of physical realization.
 *
 * ============================================================================
 * DOMAIN NEUTRALITY
 * ============================================================================
 *
 * The same pattern system can operate over:
 *
 *     classical values
 *     numerical values
 *     records
 *     variants
 *     collections
 *     tensors
 *     symbolic values
 *     probabilistic values
 *     knowledge values
 *     inference results
 *     learning results
 *     measurement results
 *     quantum/classical values
 *     resource descriptions
 *     capability descriptions
 *     distributed values
 *     HDL semantic objects
 *     hardware abstraction objects
 *     future computational values
 *
 * No domain receives a separate pattern language.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The grammar produces parse structure only.
 *
 * The frontend AST remains the authoritative representation of source
 * structure.
 *
 * Existing repository pattern nodes use the generic pattern classification:
 *
 *     CoreNodeKind::Pattern
 *
 * Pattern-specific nodes preserve:
 *
 *     - node identity;
 *     - source span;
 *     - source ordering;
 *     - child ordering;
 *     - pattern shape;
 *     - referenced child IDs;
 *     - metadata.
 *
 * This grammar must not require a target-specific AST such as:
 *
 *     QuantumPattern
 *     GPUPattern
 *     FPGA Pattern
 *     CPUPattern
 *
 * unless such a distinction is explicitly introduced at the semantic layer.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for:
 *
 *     - determining the scrutinee type;
 *     - resolving names;
 *     - determining whether a binding is valid;
 *     - determining binding types;
 *     - checking binding consistency;
 *     - checking literal compatibility;
 *     - checking tuple/sequence structure;
 *     - checking record/variant structure;
 *     - checking range endpoint types;
 *     - checking range endpoint constancy/evaluability;
 *     - checking reference semantics;
 *     - checking type-pattern semantics;
 *     - checking OR-pattern compatibility;
 *     - checking exhaustiveness;
 *     - checking reachability;
 *     - checking overlap;
 *     - checking guard interaction;
 *     - determining effects;
 *     - determining capabilities;
 *     - determining resource requirements;
 *     - applying policies;
 *     - producing provenance.
 *
 * This grammar MUST NOT attempt any of these operations.
 *
 * ============================================================================
 * PATTERN MATCHING MODEL
 * ============================================================================
 *
 * Conceptually:
 *
 *     scrutinee
 *          |
 *          v
 *       pattern
 *          |
 *          +--> bindings
 *          |
 *          +--> predicate
 *          |
 *          v
 *      optional guard
 *          |
 *          v
 *       arm result
 *
 * Pattern matching is therefore a semantic operation, not merely a textual
 * comparison.
 *
 * ============================================================================
 * PATTERN PRECEDENCE
 * ============================================================================
 *
 * Pattern syntax uses explicit structural levels.
 *
 * The intended conceptual precedence is:
 *
 *     OR pattern
 *         |
 *         v
 *     pattern atom
 *
 * Compound pattern contents are recursively composed from `pattern`.
 *
 * Parenthesized patterns can explicitly override grouping.
 *
 * Range syntax is contained within a single range-pattern construct.
 *
 * No pattern rule is permitted to recurse indirectly through the complete
 * expression grammar in a way that creates:
 *
 *     pattern -> expression -> ... -> pattern
 *
 * This is important because ANTLR supports direct left recursion but does not
 * support arbitrary indirect recursion through grammar imports.
 *
 * ============================================================================
 * GENERAL PATTERN ENTRY
 * ============================================================================
 *
 * `pattern` is the only public universal pattern entry point.
 *
 * It is deliberately implemented through `orPattern` so that:
 *
 *     a | b | c
 *
 * is structurally one pattern with ordered alternatives.
 *
 * ============================================================================
 */

parser grammar Patterns;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * 1. PUBLIC PATTERN ENTRY
 * ========================================================================== */

/*
 * A pattern is an OR-pattern or a single pattern atom.
 *
 * OR patterns are handled structurally rather than by recursively calling
 * `pattern` on both sides. This prevents unnecessary indirect recursion and
 * keeps the grammar's pattern composition predictable.
 */
pattern
    : orPattern
    ;


/* ============================================================================
 * 2. OR PATTERN
 * ========================================================================== */

/*
 * Examples:
 *
 *     0 | 1
 *     Some(x) | None
 *     red | green | blue
 *
 * Source order is preserved.
 *
 * Semantic analysis determines:
 *
 *     - overlap;
 *     - reachability;
 *     - exhaustiveness;
 *     - binding compatibility;
 *     - type compatibility.
 *
 * No semantic simplification occurs here.
 */
orPattern
    : patternAlternative
      (PIPE patternAlternative)*
    ;


/*
 * An OR-pattern with zero `|` tokens is still a valid pattern.
 *
 * This avoids defining two competing public pattern roots while allowing the
 * semantic layer to represent a single alternative uniformly.
 */
patternAlternative
    : patternAtom
    ;


/* ============================================================================
 * 3. PATTERN ATOM
 * ========================================================================== */

/*
 * Ordering is deliberately structural:
 *
 *     wildcard
 *     literal
 *     range
 *     reference
 *     tuple
 *     sequence
 *     struct
 *     variant
 *     type
 *     binding
 *     parenthesized
 *
 * Identifier-starting patterns are intentionally separated into:
 *
 *     structPattern
 *     variantPattern
 *     typePattern
 *     bindingPattern
 *
 * Their exact semantic interpretation is downstream.
 */
patternAtom
    : wildcardPattern
    | literalPattern
    | rangePattern
    | referencePattern
    | tuplePattern
    | sequencePattern
    | structPattern
    | variantPattern
    | typePattern
    | bindingPattern
    | parenthesizedPattern
    ;


/* ============================================================================
 * 4. WILDCARD
 * ========================================================================== */

/*
 * Canonical wildcard:
 *
 *     _
 *
 * A wildcard introduces no binding.
 */
wildcardPattern
    : UNDERSCORE
    ;


/* ============================================================================
 * 5. BINDING
 * ========================================================================== */

/*
 * Canonical simple binding:
 *
 *     value
 *
 * Binding semantics are determined downstream.
 *
 * In particular, this grammar does not decide:
 *
 *     mutability
 *     ownership
 *     borrowing
 *     lifetime
 *     type
 *     storage
 *     register allocation
 *     hardware placement
 */
bindingPattern
    : identifier
    ;


/* ============================================================================
 * 6. LITERAL
 * ========================================================================== */

/*
 * Pattern literals reuse the canonical Zamani literal grammar.
 *
 * No second literal language is created here.
 */
literalPattern
    : literal
    ;


/* ============================================================================
 * 7. TUPLE PATTERN
 * ========================================================================== */

/*
 * Examples:
 *
 *     (a, b)
 *     (first, second, third)
 *     (1, value)
 *
 * A tuple requires at least one comma.
 *
 * Therefore:
 *
 *     (x)
 *
 * is a parenthesized pattern, not a tuple pattern.
 *
 * Trailing commas are accepted.
 */
tuplePattern
    : LPAREN pattern COMMA patternListTail? COMMA? RPAREN
    ;


/* ============================================================================
 * 8. SEQUENCE / ARRAY PATTERN
 * ========================================================================== */

/*
 * Examples:
 *
 *     []
 *     [x]
 *     [x, y]
 *     [head, tail]
 *     [head, middle, tail]
 *
 * Sequence syntax is structural.
 *
 * It does not imply:
 *
 *     fixed memory capacity
 *     fixed vector width
 *     fixed hardware array width
 *     fixed register width
 *     fixed accelerator capacity
 */
sequencePattern
    : LBRACKET sequencePatternElements? RBRACKET
    ;


sequencePatternElements
    : sequencePatternElement
      (COMMA sequencePatternElement)*
      COMMA?
    ;


sequencePatternElement
    : restPattern
    | pattern
    ;


/*
 * Rest patterns represent the remaining sequence portion.
 *
 * Examples:
 *
 *     [head, ...tail]
 *
 * The identifier after ELLIPSIS is optional so that a discard-rest form can
 * be represented if the language semantics permit it:
 *
 *     [..._]
 *
 * Semantic validation determines whether a particular rest form is valid.
 */
restPattern
    : ELLIPSIS bindingPattern
    | ELLIPSIS wildcardPattern
    ;


/* ============================================================================
 * 9. STRUCT PATTERN
 * ========================================================================== */

/*
 * Examples:
 *
 *     Point { x: px, y: py }
 *     User { name, age }
 *
 * Type/name resolution remains semantic.
 */
structPattern
    : qualifiedName
      LBRACE
      structPatternFieldList?
      RBRACE
    ;


structPatternFieldList
    : structPatternField
      (COMMA structPatternField)*
      COMMA?
    ;


structPatternField
    : identifier
      (COLON pattern)?
    ;


/* ============================================================================
 * 10. VARIANT / ENUM PATTERN
 * ========================================================================== */

/*
 * Examples:
 *
 *     Some(value)
 *     None
 *     Error(code)
 *     Point(x, y)
 *
 * Struct-like variant payloads are also permitted:
 *
 *     Message { payload }
 *
 * The semantic layer determines whether the qualified name denotes a variant,
 * constructor, type, or another pattern-bearing entity.
 */
variantPattern
    : qualifiedName
      (
          LPAREN patternList? RPAREN
        | LBRACE structPatternFieldList? RBRACE
      )
    ;


/* ============================================================================
 * 11. RANGE PATTERN
 * ========================================================================== */

/*
 * Canonical range spellings:
 *
 *     start .. end
 *     start ..= end
 *
 * Open-ended forms are also supported:
 *
 *     .. end
 *     ..= end
 *     start ..
 *     start ..=
 *
 * This is intentionally structural.
 *
 * The semantic layer determines whether:
 *
 *     - the endpoint type is ordered;
 *     - the endpoint is valid;
 *     - an endpoint is constant/evaluable;
 *     - the range is inclusive/exclusive;
 *     - the range is meaningful for the scrutinee type.
 *
 * IMPORTANT:
 *
 * Range operators are the canonical lexer tokens:
 *
 *     DOT_DOT
 *     DOT_DOT_EQ
 *
 * There is no:
 *
 *     RANGE_EXCLUSIVE
 *     RANGE_INCLUSIVE
 *
 * token.
 */
rangePattern
    : rangePatternEndpoint DOT_DOT rangePatternEndpoint
    | rangePatternEndpoint DOT_DOT_EQ rangePatternEndpoint
    | DOT_DOT rangePatternEndpoint
    | DOT_DOT_EQ rangePatternEndpoint
    | rangePatternEndpoint DOT_DOT
    | rangePatternEndpoint DOT_DOT_EQ
    ;


rangePatternEndpoint
    : literal
    | identifier
    | qualifiedName
    | parenthesizedPatternEndpoint
    ;


parenthesizedPatternEndpoint
    : LPAREN expression RPAREN
    ;


/*
 * The endpoint grammar intentionally accepts a semantic expression only
 * inside explicit parentheses.
 *
 * This prevents a range pattern from consuming an unrestricted complete
 * expression such as:
 *
 *     a | b
 *
 * as the endpoint of:
 *
 *     a | b .. c
 *
 * The parser therefore retains a clear structural boundary between:
 *
 *     pattern OR
 *
 * and:
 *
 *     range operator.
 *
 * Whether an endpoint expression is permitted semantically remains a
 * type/constant-evaluation question.
 */


/* ============================================================================
 * 12. REFERENCE PATTERN
 * ========================================================================== */

/*
 * Canonical reference-pattern shape:
 *
 *     &pattern
 *
 * Semantic analysis determines:
 *
 *     ownership
 *     borrowing
 *     lifetime
 *     reference compatibility
 *     mutability
 *
 * The grammar only records the structure.
 */
referencePattern
    : AMPERSAND pattern
    ;


/* ============================================================================
 * 13. TYPE PATTERN
 * ========================================================================== */

/*
 * Canonical type-pattern shape:
 *
 *     pattern : Type
 *
 * The type expression belongs to the canonical type subsystem.
 *
 * This grammar only establishes the structural relationship.
 */
typePattern
    : patternAtomBase COLON typeExpression
    ;


/*
 * `patternAtomBase` exists specifically to prevent:
 *
 *     typePattern
 *         -> pattern
 *             -> typePattern
 *
 * from becoming indirect left recursion.
 *
 * It intentionally contains the non-type pattern forms that can occur on the
 * left side of a type pattern.
 */
patternAtomBase
    : wildcardPattern
    | literalPattern
    | referencePattern
    | tuplePattern
    | sequencePattern
    | structPattern
    | variantPattern
    | bindingPattern
    | parenthesizedPattern
    ;


/* ============================================================================
 * 14. PARENTHESIZED PATTERN
 * ========================================================================== */

/*
 * Example:
 *
 *     (x)
 *     (x | y)
 *
 * A comma inside parentheses belongs to tuplePattern.
 */
parenthesizedPattern
    : LPAREN pattern RPAREN
    ;


/* ============================================================================
 * 15. PATTERN LIST
 * ========================================================================== */

/*
 * Shared by:
 *
 *     tuple patterns
 *     variant patterns
 *     other compound patterns
 *
 * No fixed element count is encoded.
 */
patternList
    : pattern
      (COMMA pattern)*
      COMMA?
    ;


patternListTail
    : pattern
      (COMMA pattern)*
    ;


/* ============================================================================
 * 16. AST / SEMANTIC OWNERSHIP NOTES
 * ============================================================================
 *
 * The following source forms map conceptually to the existing domain-neutral
 * pattern AST family:
 *
 *     _                   -> WildcardPattern
 *     name                -> BindingPattern
 *     literal             -> LiteralPattern
 *     (a, b)              -> TuplePattern
 *     [a, b]              -> Sequence/Array pattern
 *     Type { ... }        -> StructPattern
 *     Variant(...)        -> VariantPattern
 *     a .. b              -> RangePattern
 *     a ..= b             -> RangePattern
 *     &pattern            -> ReferencePattern
 *     pattern : Type      -> TypePattern
 *     (pattern)           -> ParenthesizedPattern
 *     a | b               -> OrPattern
 *
 * The exact Rust type names are owned by:
 *
 *     src/frontend/ast/node/patterns/
 *
 * This grammar does not assume that every syntactic pattern must have a
 * distinct top-level Rust enum variant.
 *
 * ============================================================================
 * RANGE AST INTEGRATION
 * ============================================================================
 *
 * The existing RangePattern AST contract represents:
 *
 *     optional start endpoint
 *     optional end endpoint
 *     bound kind
 *
 * Therefore:
 *
 *     start .. end
 *         -> start=Some, end=Some, Exclusive
 *
 *     start ..= end
 *         -> start=Some, end=Some, Inclusive
 *
 *     .. end
 *         -> start=None, end=Some, Exclusive
 *
 *     ..= end
 *         -> start=None, end=Some, Inclusive
 *
 *     start ..
 *         -> start=Some, end=None, Exclusive
 *
 *     start ..=
 *         -> start=Some, end=None, Inclusive
 *
 * Semantic validation decides whether open-ended ranges are legal for the
 * matched type.
 *
 * ============================================================================
 * OR-PATTERN AST INTEGRATION
 * ============================================================================
 *
 * OR patterns preserve source ordering:
 *
 *     a | b | c
 *
 * is structurally represented as a composition of OR-pattern nodes rather
 * than a grammar-level fixed-size array.
 *
 * Therefore no artificial:
 *
 *     MAX_OR_ALTERNATIVES
 *
 * exists.
 *
 * The AST may represent an arbitrary number of alternatives through recursive
 * composition.
 *
 * ============================================================================
 * SOURCE ORDER
 * ============================================================================
 *
 * Pattern ordering is observable where alternatives overlap.
 *
 * The parser MUST preserve:
 *
 *     - OR alternative order;
 *     - tuple element order;
 *     - sequence element order;
 *     - struct field order as written;
 *     - variant payload order;
 *     - range endpoint positions;
 *     - reference nesting;
 *     - parenthesized structure.
 *
 * The grammar performs no sorting or canonicalization.
 *
 * ============================================================================
 * BINDINGS
 * ============================================================================
 *
 * Binding names remain syntax-level identifiers.
 *
 * Semantic analysis determines:
 *
 *     - whether the binding is permitted;
 *     - its inferred/declared type;
 *     - ownership;
 *     - lifetime;
 *     - mutability;
 *     - scope;
 *     - whether all OR alternatives bind compatible names/types.
 *
 * The grammar MUST NOT inspect the symbol table.
 *
 * ============================================================================
 * GUARD INTEGRATION
 * ============================================================================
 *
 * Guards are deliberately NOT owned here.
 *
 * The canonical match grammar owns:
 *
 *     guardClause
 *
 * whose body is an ordinary Zamani expression.
 *
 * Conceptually:
 *
 *     pattern
 *       |
 *       +--> optional guard
 *       |
 *       +--> arm body
 *
 * A guard is not another pattern.
 *
 * This allows guards to use ordinary:
 *
 *     arithmetic
 *     comparisons
 *     logical expressions
 *     function calls
 *     data operations
 *     reasoning predicates
 *     knowledge queries
 *     uncertainty values
 *     measurement results
 *     capability predicates
 *
 * while leaving their semantic legality to the effect/type/capability system.
 *
 * ============================================================================
 * CONTRACT INTEGRATION
 * ============================================================================
 *
 * Patterns may occur inside constructs governed by:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * This grammar does not interpret those contracts.
 *
 * ============================================================================
 * EFFECT INTEGRATION
 * ============================================================================
 *
 * Pattern matching syntax itself does not automatically introduce an effect.
 *
 * Effects may originate from:
 *
 *     - endpoint expressions where permitted;
 *     - semantic conversion;
 *     - guard expressions;
 *     - scrutinee evaluation;
 *     - arm results.
 *
 * The effect system determines legality.
 *
 * ============================================================================
 * CAPABILITY / RESOURCE INTEGRATION
 * ============================================================================
 *
 * Pattern syntax contains no target-resource requirements.
 *
 * A pattern may nevertheless match a value describing:
 *
 *     capability
 *     resource
 *     topology
 *     execution state
 *     resilience state
 *     measurement result
 *     hardware abstraction
 *
 * Resource/capability analysis happens after parsing.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Patterns can match values produced by quantum computation.
 *
 * For example, a semantic quantum measurement may produce a value that is
 * subsequently matched:
 *
 *     measurement
 *         |
 *         v
 *       match
 *         |
 *         v
 *       pattern
 *
 * The pattern grammar does not know whether a value came from:
 *
 *     classical execution
 *     quantum execution
 *     simulation
 *     hardware
 *     distributed execution
 *
 * Quantum lowering remains:
 *
 *     semantic model
 *          |
 *          v
 *     quantum::ir
 *
 * No quantum-specific pattern IR is introduced here.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Patterns can describe semantic structures used by HDL/hardware computation.
 *
 * The grammar does not determine whether a match eventually becomes:
 *
 *     muxing
 *     combinational logic
 *     sequential control
 *     predicate logic
 *     state transition
 *     host/device control
 *
 * That belongs to semantic lowering and backend realization.
 *
 * ============================================================================
 * AI / KNOWLEDGE / REASONING INTEGRATION
 * ============================================================================
 *
 * Pattern matching can consume values produced by:
 *
 *     inference
 *     deduction
 *     reasoning
 *     knowledge queries
 *     learning
 *     adaptation
 *     uncertainty
 *     probability
 *     evidence
 *     agents
 *
 * No AI-specific pattern syntax is introduced.
 *
 * This keeps the pattern system universal.
 *
 * ============================================================================
 * DISTRIBUTED / CONCURRENT INTEGRATION
 * ============================================================================
 *
 * Patterns may match:
 *
 *     messages
 *     events
 *     task states
 *     execution outcomes
 *     service responses
 *     distributed values
 *
 * No node count or topology limit belongs in this grammar.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Parsing a pattern MUST NOT:
 *
 *     - execute code;
 *     - resolve external resources;
 *     - inspect hardware;
 *     - access credentials;
 *     - open files;
 *     - contact networks;
 *     - invoke devices;
 *     - invoke quantum hardware;
 *     - invoke an HDL simulator.
 *
 * Semantic validation may later apply:
 *
 *     security policies
 *     capability policies
 *     sandbox policies
 *     provenance requirements
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Given identical:
 *
 *     source token sequence
 *     grammar version
 *     lexer version
 *
 * parsing must be deterministic.
 *
 * It must not depend on:
 *
 *     time
 *     randomness
 *     environment variables
 *     filesystem state
 *     network state
 *     hardware availability
 *     runtime state
 *     scheduler state
 *     backend selection
 *
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * Structural syntax errors are reported by the normal ANTLR parser error
 * mechanism.
 *
 * This grammar does not embed custom actions.
 *
 * Semantic diagnostics are downstream.
 *
 * Examples of semantic errors include:
 *
 *     incompatible binding sets
 *     impossible range
 *     invalid range endpoint type
 *     non-exhaustive match
 *     unreachable alternative
 *     overlapping alternatives
 *     invalid reference pattern
 *     invalid type pattern
 *
 * ============================================================================
 * ERROR RECOVERY
 * ============================================================================
 *
 * ANTLR's standard parser recovery remains authoritative.
 *
 * This grammar must not embed recovery actions or target-language callbacks.
 *
 * ============================================================================
 * COMPILER / IR CONTRACT
 * ============================================================================
 *
 * Patterns do not lower directly to a machine-specific IR.
 *
 * The path is:
 *
 *     pattern syntax
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic pattern model
 *          |
 *          +------------------------+
 *          |            |           |
 *          v            v           v
 *      classical     quantum::ir   HDL/hardware
 *          |            |           |
 *          +------------+-----------+
 *                       |
 *                       v
 *                 optimization
 *                       |
 *                lowering/routing
 *                       |
 *                  scheduling
 *                       |
 *                target realization
 *
 * Match decision structures may eventually become:
 *
 *     branch control
 *     decision trees
 *     predication
 *     lookup structures
 *     dataflow
 *     distributed dispatch
 *     hardware selection logic
 *     dynamic quantum/classical control
 *
 * provided semantic equivalence is preserved.
 *
 * ============================================================================
 * PROVENANCE
 * ============================================================================
 *
 * Source provenance must preserve enough information to identify:
 *
 *     pattern
 *     pattern alternative
 *     binding
 *     range endpoint
 *     struct field
 *     variant payload
 *     source ordering
 *
 * This supports:
 *
 *     diagnostics
 *     debugging
 *     verification
 *     optimization explanation
 *     reproducibility
 *     audit
 *     semantic decision records
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing source forms that this grammar is intended to preserve include:
 *
 *     _
 *     value
 *     0
 *     true
 *     (a, b)
 *     [a, b]
 *     Point { x: px, y: py }
 *     Some(value)
 *     &value
 *     value .. limit
 *     value ..= limit
 *     value | other
 *     (value)
 *
 * Existing syntax that used:
 *
 *     RANGE_EXCLUSIVE
 *     RANGE_INCLUSIVE
 *
 * is normalized to the canonical:
 *
 *     DOT_DOT
 *     DOT_DOT_EQ
 *
 * lexical vocabulary.
 *
 * ============================================================================
 * MACRO / METAPROGRAMMING INTEGRATION
 * ============================================================================
 *
 * Macro expansion may generate pattern syntax.
 *
 * Generated patterns MUST re-enter the ordinary parser/AST/semantic pipeline.
 *
 * This grammar provides no semantic bypass for generated source.
 *
 * Reflection/metaprogramming cannot use this grammar to bypass:
 *
 *     type checking
 *     ownership
 *     effects
 *     capabilities
 *     resources
 *     contracts
 *     policies
 *     provenance
 *
 * ============================================================================
 * NO HARD-CODED DOMAIN INVENTORY
 * ============================================================================
 *
 * This grammar deliberately does not enumerate:
 *
 *     quantum gates
 *     hardware models
 *     CPU architectures
 *     GPU vendors
 *     FPGA families
 *     AI frameworks
 *     databases
 *     network protocols
 *
 * Pattern syntax is reusable across all of them.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] exactly one public `pattern` owner exists;
 *     [x] pattern syntax is reusable by expression and statement matching;
 *     [x] match expression syntax is not duplicated here;
 *     [x] match statement syntax is not duplicated here;
 *     [x] guards are not duplicated here;
 *     [x] canonical lexer tokens are consumed;
 *     [x] nonexistent range token aliases are not used;
 *     [x] no lexer rules exist here;
 *     [x] no semantic actions exist here;
 *     [x] no semantic predicates exist here;
 *     [x] no hardware assumptions exist here;
 *     [x] no universal resource limits exist here;
 *     [x] source ordering is preserved;
 *     [x] AST integration is domain-neutral;
 *     [x] range AST integration is defined;
 *     [x] OR-pattern integration is defined;
 *     [x] quantum integration is defined;
 *     [x] HDL integration is defined;
 *     [x] AI/data integration is domain-neutral;
 *     [x] POCO-REAF is preserved;
 *     [x] safe Rust remains the implementation contract;
 *     [x] Rust 1.97/1.97.1 remains supported.
 *
 * The repository integration is complete only after the existing duplicate
 * pattern productions in:
 *
 *     grammar/statements/pattern-matching.g4
 *
 * have been replaced by imports/consumption of this grammar's rules.
 *
 * ============================================================================
 * REQUIRED INTEGRATION CHANGES
 * ============================================================================
 *
 * The following changes are deliberately NOT performed in this file:
 *
 * 1. grammar/statements/pattern-matching.g4
 *
 *    Remove its duplicate definitions of:
 *
 *        pattern
 *        wildcardPattern
 *        bindingPattern
 *        literalPattern
 *        tuplePattern
 *        arrayPattern
 *        structPattern
 *        enumPattern
 *        rangePattern
 *        orPattern
 *        referencePattern
 *        typePattern
 *        parenthesizedPattern
 *        patternList
 *
 *    Replace them with:
 *
 *        import Patterns;
 *
 *    Keep statement/match ownership there for:
 *
 *        matchStatement
 *        matchArm
 *        guardClause
 *
 * 2. grammar/expressions/match.g4
 *
 *    It should consume the shared:
 *
 *        matchArm
 *
 *    without redefining pattern rules.
 *
 * 3. grammar/statements/statements.g4
 *
 *    It already imports:
 *
 *        Expressions
 *        PatternMatching
 *
 *    so the shared pattern rules become available through the statement
 *    composition hierarchy once PatternMatching imports Patterns.
 *
 * 4. grammar/antlr/ZamaniParser.g4
 *
 *    No direct leaf-pattern import should be added.
 *
 *    The existing dispatcher architecture remains authoritative.
 *
 * 5. grammar/antlr/ZamaniLexer.g4
 *
 *    No change is required for this file.
 *
 *    Range patterns consume the existing:
 *
 *        DOT_DOT
 *        DOT_DOT_EQ
 *
 *    tokens.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/lexer/
 *     canonical identifier/name rules
 *     canonical literal rules
 *     canonical type-expression rule
 *     canonical expression rule only where explicitly referenced by
 *         parenthesizedPatternEndpoint
 *
 * EXPORTS:
 *
 *     pattern
 *     patternAtom
 *     wildcardPattern
 *     bindingPattern
 *     literalPattern
 *     tuplePattern
 *     sequencePattern
 *     sequencePatternElements
 *     sequencePatternElement
 *     restPattern
 *     structPattern
 *     structPatternFieldList
 *     structPatternField
 *     variantPattern
 *     rangePattern
 *     rangePatternEndpoint
 *     orPattern
 *     patternAlternative
 *     referencePattern
 *     typePattern
 *     parenthesizedPattern
 *     patternList
 *     patternListTail
 *
 * CONSUMED_BY:
 *
 *     grammar/statements/pattern-matching.g4
 *     grammar/expressions/match.g4
 *     future pattern-consuming grammars
 *
 * AST_OWNER:
 *
 *     src/frontend/ast/node/patterns/
 *
 * SEMANTIC_OWNER:
 *
 *     semantic analysis
 *
 * IR_OWNER:
 *
 *     canonical semantic model
 *     quantum::ir for quantum-derived semantics
 *
 * TEST_OWNER:
 *
 *     grammar/tests/
 *     expression/match conformance tests
 *     statement/match conformance tests
 *
 * SPEC_OWNER:
 *
 *     grammar/specification/
 *     grammar/spec/
 *
 * ============================================================================
 * IMPORTANT BUILD INTEGRATION NOTE
 * ============================================================================
 *
 * This is a reusable parser grammar, not the complete parser root.
 *
 * It is therefore expected to be resolved through the repository's ANTLR
 * grammar-library path and imported by the canonical composition grammar.
 *
 * The build must make:
 *
 *     grammar/expressions/
 *
 * available as an ANTLR grammar-library location.
 *
 * No generated parser file should be manually edited to integrate this file.
 *
 * ============================================================================
 * SAFETY
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no Rust;
 *     - no embedded actions;
 *     - no semantic predicates;
 *     - no I/O;
 *     - no networking;
 *     - no filesystem access;
 *     - no hardware discovery;
 *     - no device discovery;
 *     - no runtime execution;
 *     - no unsafe code.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * ONE PATTERN LANGUAGE
 * ONE PATTERN AUTHORITY
 * ONE DOMAIN-NEUTRAL AST
 * MANY MATCH CONSUMERS
 * MANY COMPUTATIONAL DOMAINS
 * NO MACHINE-SIZE CEILINGS
 *
 * ============================================================================
 */
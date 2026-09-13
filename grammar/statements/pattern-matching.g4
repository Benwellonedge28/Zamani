/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/statements/pattern-matching.g4
 *
 * Grammar:
 *     PatternMatching
 *
 * Status:
 *     Production-ready modular parser grammar.
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     No embedded Rust actions.
 *     No semantic predicates.
 *     No unsafe code.
 *     No filesystem access.
 *     No networking.
 *     No runtime execution.
 *     No hardware discovery.
 *     No device discovery.
 *     No mutable global parser state.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar owns Zamani's source-level pattern-matching syntax.
 *
 * It provides the canonical parser boundary for:
 *
 *     - match statements;
 *     - match arms;
 *     - patterns;
 *     - wildcard patterns;
 *     - binding patterns;
 *     - literal patterns;
 *     - tuple patterns;
 *     - array patterns;
 *     - struct patterns;
 *     - enum/variant patterns;
 *     - range patterns;
 *     - OR patterns;
 *     - reference patterns;
 *     - type patterns;
 *     - parenthesized patterns;
 *     - match guards.
 *
 * Pattern syntax is domain-neutral.
 *
 * The same pattern machinery can therefore participate in:
 *
 *     classical computation
 *     quantum/classical control
 *     HDL descriptions
 *     hardware control
 *     distributed computation
 *     data processing
 *     AI/ML programs
 *     accelerator programs
 *     future Zamani domains
 *
 * ============================================================================
 * ARCHITECTURAL PRINCIPLE
 * ============================================================================
 *
 *     source
 *        |
 *        v
 *     ZamaniLexer
 *        |
 *        v
 *     PatternMatching
 *        |
 *        v
 *     frontend AST
 *        |
 *        v
 *     name/type/effect/capability analysis
 *        |
 *        v
 *     semantic representation
 *        |
 *        +--> classical IR
 *        +--> quantum::ir
 *        +--> HDL/hardware IR
 *        +--> distributed/control/data IR
 *        |
 *        v
 *     optimization / lowering / routing / scheduling
 *        |
 *        v
 *     runtime / target realization
 *
 * This grammar owns syntax only.
 *
 * It does NOT determine:
 *
 *     - whether a pattern is exhaustive;
 *     - whether patterns overlap;
 *     - whether a binding is irrefutable;
 *     - whether a range is valid for a particular type;
 *     - whether a quantum value may be destructured;
 *     - whether a hardware value may be matched;
 *     - whether a resource exists;
 *     - how a match executes;
 *     - how a match is optimized;
 *     - which machine executes it.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Pattern syntax must describe semantic structure rather than machine
 * characteristics.
 *
 * Patterns MUST NOT encode:
 *
 *     - CPU counts;
 *     - GPU counts;
 *     - FPGA counts;
 *     - QPU counts;
 *     - qubit limits;
 *     - register limits;
 *     - memory limits;
 *     - hardware topology;
 *     - device identifiers;
 *     - physical addresses;
 *     - deployment topology;
 *     - fixed accelerator counts.
 *
 * A pattern such as:
 *
 *     [a, b, c, ...]
 *
 * describes source-level structure.
 *
 * It does not imply a fixed hardware resource count.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - matchStatement;
 *     - matchArm;
 *     - guardClause;
 *     - pattern;
 *     - wildcardPattern;
 *     - bindingPattern;
 *     - literalPattern;
 *     - tuplePattern;
 *     - arrayPattern;
 *     - structPattern;
 *     - structPatternFieldList;
 *     - structPatternField;
 *     - enumPattern;
 *     - rangePattern;
 *     - orPattern;
 *     - referencePattern;
 *     - typePattern;
 *     - parenthesizedPattern;
 *     - patternList;
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexer tokens;
 *     - identifiers;
 *     - qualified-name syntax;
 *     - literal syntax;
 *     - type-expression syntax;
 *     - general expression syntax;
 *     - block syntax;
 *     - declarations;
 *     - function syntax;
 *     - variable declarations;
 *     - quantum IR;
 *     - classical IR;
 *     - HDL IR;
 *     - resource models;
 *     - hardware topology;
 *     - scheduling;
 *     - routing;
 *     - optimization;
 *     - QEC;
 *     - ZQN;
 *     - resilience;
 *     - runtime behavior.
 *
 * ============================================================================
 * IMPORTANT REPOSITORY INTEGRATION
 * ============================================================================
 *
 * The legacy monolithic:
 *
 *     grammar/antlr/Core.g4
 *
 * currently contains the pattern/match productions.
 *
 * The modular grammar must become the authoritative owner of those
 * productions.
 *
 * During grammar assembly, Core.g4 MUST NOT retain competing definitions of:
 *
 *     matchStatement
 *     matchArm
 *     pattern
 *     wildcardPattern
 *     bindingPattern
 *     literalPattern
 *     tuplePattern
 *     arrayPattern
 *     structPattern
 *     enumPattern
 *     rangePattern
 *     orPattern
 *     referencePattern
 *     typePattern
 *     parenthesizedPattern
 *
 * The migration target is:
 *
 *     Core / statement composition
 *             |
 *             +--> PatternMatching.matchStatement
 *                              |
 *                              +--> pattern
 *
 * This prevents two grammars from becoming competing sources of truth.
 *
 * ============================================================================
 * ANTLR INTEGRATION
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * The lexer remains authoritative for all tokens.
 *
 * Therefore this file MUST NOT define lexer rules.
 *
 * The canonical lexer vocabulary is:
 *
 *     ZamaniLexer
 *
 * ============================================================================
 */

parser grammar PatternMatching;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * MATCH STATEMENT
 * ============================================================================
 *
 * Canonical statement form:
 *
 *     match expression {
 *         pattern => expression,
 *         pattern => { ... },
 *     }
 *
 * At least one arm is required.
 *
 * Exhaustiveness is a semantic-analysis responsibility.
 */
matchStatement
    : MATCH expression LBRACE matchArm+ RBRACE
    ;


/*
 * ============================================================================
 * MATCH ARM
 * ============================================================================
 *
 * A match arm consists of:
 *
 *     pattern
 *     optional guard
 *     =>
 *     expression OR block
 *     optional comma
 *
 * The guard remains an expression because the expression grammar owns
 * expression semantics.
 */
matchArm
    : pattern
      guardClause?
      FAT_ARROW
      (expression | block)
      COMMA?
    ;


/*
 * ============================================================================
 * MATCH GUARD
 * ============================================================================
 *
 * Guards are deliberately expressed through the normal expression grammar.
 *
 * This permits guards to use:
 *
 *     function calls
 *     comparisons
 *     logical expressions
 *     arithmetic
 *     type-related predicates
 *     classical values
 *     measurement results
 *     domain-specific semantic predicates
 *
 * without embedding those concepts into the pattern grammar.
 */
guardClause
    : WHEN expression
    ;


/*
 * ============================================================================
 * PATTERN ROOT
 * ============================================================================
 *
 * A pattern is a source-level structural matcher.
 *
 * Alternative ordering is intentional:
 *
 *     1. wildcard
 *     2. literal
 *     3. compound patterns
 *     4. named/type-qualified patterns
 *     5. parenthesized patterns
 *
 * Semantic ambiguity between forms that begin with an identifier is resolved
 * by the concrete syntax and later semantic analysis, not by machine-specific
 * assumptions.
 */
pattern
    : wildcardPattern
    | literalPattern
    | tuplePattern
    | arrayPattern
    | structPattern
    | enumPattern
    | rangePattern
    | orPattern
    | referencePattern
    | typePattern
    | bindingPattern
    | parenthesizedPattern
    ;


/*
 * ============================================================================
 * WILDCARD PATTERN
 * ============================================================================
 *
 * Matches a value without introducing a binding.
 *
 * Example:
 *
 *     _
 *
 * Ownership of the underscore token remains with the lexer.
 */
wildcardPattern
    : UNDERSCORE
    ;


/*
 * ============================================================================
 * BINDING PATTERN
 * ============================================================================
 *
 * A binding pattern introduces a source-level name.
 *
 * Example:
 *
 *     value
 *
 * The grammar does not determine:
 *
 *     mutability
 *     ownership
 *     borrowing
 *     lifetime
 *     type
 *     storage
 *     register allocation
 *     hardware placement
 *
 * Those belong downstream.
 */
bindingPattern
    : identifier
    ;


/*
 * ============================================================================
 * LITERAL PATTERN
 * ============================================================================
 *
 * Literal syntax remains owned by the canonical literal grammar.
 *
 * This allows pattern matching to use existing Zamani literals rather than
 * creating a second literal language.
 */
literalPattern
    : literal
    ;


/*
 * ============================================================================
 * TUPLE PATTERN
 * ============================================================================
 *
 * Examples:
 *
 *     (a, b)
 *     (first, second, third)
 *     (1, x)
 *
 * A one-element parenthesized expression is intentionally NOT treated as a
 * tuple. A tuple requires at least one comma.
 *
 * Trailing commas are accepted.
 */
tuplePattern
    : LPAREN pattern COMMA patternListTail? COMMA? RPAREN
    ;


/*
 * ============================================================================
 * ARRAY / SEQUENCE PATTERN
 * ============================================================================
 *
 * Examples:
 *
 *     []
 *     [x]
 *     [x, y]
 *     [head, tail]
 *
 * This syntax describes source-level sequence structure.
 *
 * It does not encode:
 *
 *     fixed hardware array size;
 *     memory capacity;
 *     vector width;
 *     register count;
 *     number of physical elements.
 */
arrayPattern
    : LBRACKET patternList? RBRACKET
    ;


/*
 * ============================================================================
 * STRUCT PATTERN
 * ============================================================================
 *
 * Examples:
 *
 *     Point { x: px, y: py }
 *     User { name, age }
 *
 * The type/name resolution remains downstream.
 */
structPattern
    : qualifiedName
      LBRACE structPatternFieldList? RBRACE
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


/*
 * ============================================================================
 * ENUM / VARIANT PATTERN
 * ============================================================================
 *
 * Examples:
 *
 *     Some(value)
 *     Error(code)
 *     Point(x, y)
 *     Message { payload }
 *
 * Qualified names remain owned by the core name grammar.
 */
enumPattern
    : qualifiedName
      (
          LPAREN patternList? RPAREN
        | LBRACE structPatternFieldList? RBRACE
      )
    ;


/*
 * ============================================================================
 * RANGE PATTERN
 * ============================================================================
 *
 * Range patterns describe a semantic interval.
 *
 * The endpoints are expressions because the endpoint's type and constant
 * evaluability are semantic concerns.
 *
 * Canonical forms supported:
 *
 *     start .. end
 *     start ..= end
 *
 * The grammar intentionally does not define an upper bound or iteration count.
 *
 * No:
 *
 *     MAX_RANGE_SIZE
 *     MAX_RANGE_ENDPOINT
 *     MAX_PATTERN_VALUES
 *
 * may be introduced here.
 */
rangePattern
    : expression RANGE_EXCLUSIVE expression
    | expression RANGE_INCLUSIVE expression
    ;


/*
 * ============================================================================
 * OR PATTERN
 * ============================================================================
 *
 * Examples:
 *
 *     0 | 1
 *     Some(x) | None
 *     Red | Green | Blue
 *
 * The semantic layer determines:
 *
 *     reachability;
 *     binding compatibility;
 *     exhaustiveness;
 *     overlap;
 *     type compatibility.
 */
orPattern
    : patternAlternative (PIPE patternAlternative)+
    ;


patternAlternative
    : wildcardPattern
    | literalPattern
    | tuplePattern
    | arrayPattern
    | structPattern
    | enumPattern
    | rangePattern
    | referencePattern
    | typePattern
    | bindingPattern
    | parenthesizedPattern
    ;


/*
 * ============================================================================
 * REFERENCE PATTERN
 * ============================================================================
 *
 * Reference matching is syntax only.
 *
 * The semantic layer determines whether a referenced value/type can be
 * matched and what ownership/borrowing consequences exist.
 *
 * Canonical source shape:
 *
 *     &pattern
 *
 * The lexer owns AMPERSAND.
 */
referencePattern
    : AMPERSAND pattern
    ;


/*
 * ============================================================================
 * TYPE PATTERN
 * ============================================================================
 *
 * A type pattern associates a pattern with a type expression.
 *
 * Canonical source shape:
 *
 *     pattern : Type
 *
 * The exact semantic interpretation is downstream.
 *
 * This grammar therefore reuses the canonical typeExpression rule.
 */
typePattern
    : pattern COLON typeExpression
    ;


/*
 * ============================================================================
 * PARENTHESIZED PATTERN
 * ============================================================================
 *
 * Parentheses may group a pattern without introducing a tuple.
 *
 * Example:
 *
 *     (x | y)
 *
 * Tuple syntax remains distinct because tuplePattern requires a comma.
 */
parenthesizedPattern
    : LPAREN pattern RPAREN
    ;


/*
 * ============================================================================
 * PATTERN LIST
 * ============================================================================
 *
 * Shared by tuple, enum and other compound patterns.
 *
 * No fixed element count is encoded.
 */
patternList
    : pattern (COMMA pattern)* COMMA?
    ;


patternListTail
    : pattern (COMMA pattern)*
    ;
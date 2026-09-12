/*
 * Zamani — Comparison Expression Grammar
 *
 * File:
 *   grammar/expressions/comparison.g4
 *
 * Status:
 *   Production parser-grammar module.
 *
 * Purpose:
 *   Defines the syntactic layer for relational and equality comparison.
 *
 * Architectural ownership:
 *
 *   comparison.g4 OWNS:
 *     - equality-expression syntax
 *     - relational/comparison-expression syntax
 *     - comparison operator grouping
 *     - precedence between relational and equality operators
 *
 *   comparison.g4 DOES NOT OWN:
 *     - primary expressions
 *     - literals
 *     - identifiers
 *     - calls
 *     - indexing
 *     - member access
 *     - arithmetic
 *     - shifts
 *     - logical operators
 *     - bitwise operators
 *     - assignment
 *     - ranges
 *     - type checking
 *     - overload resolution
 *     - implicit conversions
 *     - quantum semantics
 *     - classical semantics
 *     - HDL semantics
 *     - resource semantics
 *     - target selection
 *     - hardware capabilities
 *     - IR construction
 *     - runtime behavior
 *
 * Integration model:
 *
 *   lexer
 *       |
 *       v
 *   token stream
 *       |
 *       v
 *   primary/prefix/postfix/arithmetic/shift
 *       |
 *       v
 *   comparison.g4
 *       |
 *       v
 *   logical/bitwise/range/assignment layers
 *       |
 *       v
 *   AST
 *       |
 *       v
 *   semantic analysis
 *       |
 *       +--> type checking
 *       +--> capability/effect checking
 *       +--> quantum semantic validation
 *       +--> HDL semantic validation
 *       +--> resource validation
 *       |
 *       v
 *   canonical IR
 *
 * IMPORTANT:
 *
 * This grammar deliberately does not decide whether two operands are
 * semantically comparable. That belongs to semantic analysis.
 *
 * Examples of semantic questions deliberately outside this grammar:
 *
 *   - whether Int < Float is legal;
 *   - whether two matrices may be compared;
 *   - whether two quantum states may be compared;
 *   - whether a hardware resource can be compared;
 *   - whether a tensor supports equality;
 *   - whether a user-defined type implements an equality relation;
 *   - whether ordering exists for a type;
 *   - whether comparison is exact, approximate, symbolic, or domain-specific.
 *
 * Those decisions must be represented by the type/semantic system and
 * eventually lowered into the appropriate IR operation.
 *
 * Scalability:
 *
 * No resource count, machine size, qubit count, topology, register count,
 * vector width, device count, or hardware-specific limit occurs here.
 *
 * This grammar accepts expressions of arbitrary syntactic depth subject
 * only to the parser/runtime resource limits imposed by the compilation
 * environment.
 *
 * Rust:
 *
 * This grammar contains no Rust implementation code.
 * Generated parser integration must remain compatible with Rust 1.97 and
 * Rust 1.97.1 and must not require unsafe Rust in Zamani-owned code.
 *
 * Compatibility:
 *
 * The repository's documented expression precedence is:
 *
 *   Equality:
 *       == !=
 *
 *   Comparison:
 *       < <= > >=
 *
 * Comparison binds more tightly than equality.
 *
 * Therefore:
 *
 *   a < b == c < d
 *
 * parses as:
 *
 *   (a < b) == (c < d)
 *
 * rather than:
 *
 *   a < (b == c) < d
 *
 * Chaining comparisons is intentionally NOT given special syntactic
 * treatment here. A sequence such as:
 *
 *   a < b < c
 *
 * is parsed according to ordinary binary-expression structure and its
 * semantic legality is determined by the type/semantic layer.
 *
 * This avoids silently introducing Python-like chained-comparison
 * semantics into the language.
 *
 * ANTLR architecture:
 *
 * This file is intentionally a parser grammar rather than a second
 * combined grammar. The lexer must be the single owner of operator tokens.
 *
 * The master parser must import this grammar module after the repository
 * has separated the lexer from the parser.
 *
 * Expected lexer token contract:
 *
 *   EQ        == 
 *   NE        !=
 *   LT        <
 *   LE        <=
 *   GT        >
 *   GE        >=
 *
 * The lexer owns their spelling.
 * This grammar owns their syntactic role.
 */

parser grammar ZamaniComparison;

options {
    /*
     * The production parser architecture must provide a single authoritative
     * lexer vocabulary.
     *
     * The intended integrated lexer is the repository's Zamani lexer.
     *
     * This token vocabulary must eventually be generated from the canonical
     * lexer layer rather than duplicated here.
     */
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * PUBLIC ENTRY RULE
 * ============================================================================
 *
 * comparisonExpression
 *
 * Public entry point for the comparison/equality precedence layer.
 *
 * The rule intentionally accepts an already-parsed higher-precedence
 * expression through `shiftExpression`.
 *
 * This prevents comparison.g4 from owning arithmetic, product, sum, or
 * shift syntax.
 *
 * Precedence:
 *
 *     shiftExpression
 *          |
 *          v
 *     relationalExpression
 *          |
 *          v
 *     equalityExpression
 *
 * Equality therefore has lower precedence than relational comparison.
 */
comparisonExpression
    : equalityExpression
    ;


/*
 * ============================================================================
 * EQUALITY
 * ============================================================================
 *
 * Equality has lower precedence than relational comparison.
 *
 * Examples:
 *
 *     a == b
 *     a != b
 *     a < b == c < d
 *
 * The last example is structurally:
 *
 *     (a < b) == (c < d)
 *
 * because relationalExpression is the operand layer beneath equality.
 *
 * Repetition is used rather than direct left recursion because this grammar
 * is intended to be composable as an imported parser module and because
 * equality expressions are associative at the syntactic level.
 *
 * Semantic analysis MUST NOT assume that equality itself is mathematically
 * associative for arbitrary user-defined operations. The parser merely
 * constructs the binary expression structure.
 */
equalityExpression
    : relationalExpression
      (
          equalityOperator
          relationalExpression
      )*
    ;


/*
 * ============================================================================
 * RELATIONAL / ORDERING COMPARISON
 * ============================================================================
 *
 * Relational comparison binds more tightly than equality.
 *
 * Supported operators:
 *
 *     <
 *     <=
 *     >
 *     >=
 *
 * Examples:
 *
 *     a < b
 *     a <= b
 *     a > b
 *     a >= b
 *
 * The operands are shift expressions because shift operators have higher
 * precedence than relational operators according to the Zamani precedence
 * model.
 *
 * This file does not own shiftExpression itself.
 */
relationalExpression
    : shiftExpression
      (
          relationalOperator
          shiftExpression
      )*
    ;


/*
 * ============================================================================
 * EQUALITY OPERATORS
 * ============================================================================
 *
 * The lexer is authoritative for the exact spelling of these operators.
 *
 * The parser only groups the tokens into the equality category.
 */
equalityOperator
    : EQ
    | NE
    ;


/*
 * ============================================================================
 * RELATIONAL OPERATORS
 * ============================================================================
 *
 * The lexer is authoritative for operator spelling.
 *
 * Keeping the operator vocabulary in the lexer gives the repository one
 * authoritative token definition and prevents different expression grammar
 * modules from silently assigning different token identities to the same
 * operator.
 */
relationalOperator
    : LT
    | LE
    | GT
    | GE
    ;


/*
 * ============================================================================
 * SEMANTIC INTEGRATION CONTRACT
 * ============================================================================
 *
 * The parser produces comparison syntax only.
 *
 * Semantic analysis must map:
 *
 *     EQ -> equality operation
 *     NE -> inequality operation
 *     LT -> less-than operation
 *     LE -> less-than-or-equal operation
 *     GT -> greater-than operation
 *     GE -> greater-than-or-equal operation
 *
 * The semantic representation must preserve:
 *
 *     - source span
 *     - operator kind
 *     - left operand
 *     - right operand
 *     - source/module identity where available
 *     - generic/type context where applicable
 *     - diagnostic provenance
 *
 * The grammar must never convert these into a machine-specific operation.
 *
 *
 * ============================================================================
 * TYPE SYSTEM CONTRACT
 * ============================================================================
 *
 * The type checker owns questions such as:
 *
 *     Can T == T?
 *     Can T != T?
 *     Can T < T?
 *     Can T <= T?
 *     Can T > T?
 *     Can T >= T?
 *
 * It also owns:
 *
 *     - numeric promotion
 *     - generic constraints
 *     - trait/interface based comparison
 *     - user-defined equality
 *     - user-defined ordering
 *     - symbolic comparison
 *     - approximate comparison
 *     - aggregate comparison
 *     - domain-specific comparison
 *
 * No such policy belongs in this grammar.
 *
 *
 * ============================================================================
 * QUANTUM INTEGRATION CONTRACT
 * ============================================================================
 *
 * Quantum expressions may occur as operands because the operand is supplied
 * by the higher-level expression grammar.
 *
 * This grammar does NOT declare:
 *
 *     qubit comparison
 *     state comparison
 *     observable comparison
 *     measurement comparison
 *     logical-qubit comparison
 *     physical-qubit comparison
 *
 * Whether any of those are legal is a semantic concern.
 *
 * In particular, this grammar must never introduce machine-dependent rules
 * such as:
 *
 *     q[0] == q[1]
 *
 * as a special case.
 *
 * Qubit identifiers and quantum values remain ordinary expression operands
 * from this grammar's perspective.
 *
 * Semantic lowering eventually integrates with the canonical
 * `quantum::ir` boundary where applicable. This grammar must never create
 * a competing quantum IR.
 *
 *
 * ============================================================================
 * HDL INTEGRATION CONTRACT
 * ============================================================================
 *
 * Hardware expressions may participate in comparison syntax where the HDL
 * semantic layer permits it.
 *
 * Examples of potentially valid higher-level expressions include:
 *
 *     signal_a == signal_b
 *     counter < limit
 *     state == expected_state
 *
 * The grammar does not determine whether these operations represent:
 *
 *     - simulation-time computation
 *     - combinational logic
 *     - sequential logic
 *     - synthesis constraints
 *     - assertions
 *     - verification properties
 *
 * HDL semantic analysis owns that distinction.
 *
 *
 * ============================================================================
 * RESOURCE / CAPABILITY INTEGRATION CONTRACT
 * ============================================================================
 *
 * Resource expressions may syntactically appear as operands:
 *
 *     available < required
 *     capacity >= requirement
 *
 * The grammar does not define resource units or hardware capacity.
 *
 * Those belong to the resource/capability model.
 *
 * Consequently this file contains no:
 *
 *     MAX_QUBITS
 *     MAX_CORES
 *     MAX_DEVICES
 *     MAX_THREADS
 *     MAX_MEMORY
 *     MAX_NODES
 *
 * or equivalent constants.
 *
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The AST layer should represent these nodes generically as binary
 * expressions or as a comparison-specific expression node.
 *
 * Recommended semantic distinction:
 *
 *     EqualityOperator:
 *         Eq
 *         Ne
 *
 *     RelationalOperator:
 *         Lt
 *         Le
 *         Gt
 *         Ge
 *
 * The parser must not force a particular AST representation if the existing
 * repository AST already has an authoritative equivalent.
 *
 * The existing AST remains the owner of the concrete node representation.
 *
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar produces no IR.
 *
 * The lowering layer consumes the AST and maps comparison operations into
 * the canonical appropriate IR operation.
 *
 * Classical comparisons may lower to the classical IR.
 *
 * Quantum-related comparisons may lower through the quantum semantic
 * pipeline when their meaning is quantum-specific.
 *
 * HDL comparisons may lower through the HDL/hardware pipeline.
 *
 * Resource comparisons may lower through resource/constraint semantics.
 *
 * The grammar remains independent of those target-specific decisions.
 *
 *
 * ============================================================================
 * OPTIMIZATION CONTRACT
 * ============================================================================
 *
 * Optimizers may simplify comparison expressions after semantic analysis.
 *
 * Examples:
 *
 *     x == x
 *     x < x
 *     !(a == b)
 *
 * Such transformations are NOT performed by this grammar.
 *
 * Optimizers must respect the semantic properties of the operand types and
 * must never assume mathematical identities for arbitrary overloaded or
 * effectful operations.
 *
 *
 * ============================================================================
 * SCHEDULING CONTRACT
 * ============================================================================
 *
 * This grammar has no scheduling dependency.
 *
 * Comparison syntax must parse without knowledge of:
 *
 *     hardware timing
 *     gate duration
 *     clock period
 *     qubit topology
 *     resource availability
 *     execution queue
 *
 * Scheduling consumes semantic/IR representations later in the pipeline.
 *
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * This grammar has no runtime dependency.
 *
 * Runtime values determine the actual result of a comparison.
 *
 * Parsing must remain independent of:
 *
 *     device
 *     backend
 *     deployment
 *     execution location
 *     machine size
 *     processor count
 *     qubit count
 *     memory capacity
 *
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * For a fixed lexer token stream, this grammar must produce deterministic
 * parse structure.
 *
 * No runtime state, hardware state, random state, environment variable,
 * filesystem state, network state, or backend capability may affect parsing.
 *
 *
 * ============================================================================
 * ERROR RECOVERY
 * ============================================================================
 *
 * Syntax errors involving comparison operators are parser errors.
 *
 * Examples:
 *
 *     a ==
 *     < b
 *     a <=
 *     a !=
 *
 * The semantic layer must not be invoked to repair malformed syntax.
 *
 * ANTLR's configured error strategy remains responsible for reporting and
 * recovering from malformed token sequences.
 *
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * This grammar:
 *
 *     - performs no I/O;
 *     - accesses no filesystem;
 *     - accesses no network;
 *     - invokes no external command;
 *     - evaluates no expression;
 *     - executes no user code;
 *     - performs no hardware discovery.
 *
 * Therefore parsing comparison syntax cannot itself trigger computation or
 * external effects.
 *
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing Zamani source forms remain supported:
 *
 *     a == b
 *     a != b
 *     a < b
 *     a <= b
 *     a > b
 *     a >= b
 *
 * The grammar intentionally does not introduce:
 *
 *     ===
 *     !==
 *
 * because those operators are not part of the current documented Zamani
 * comparison vocabulary.
 *
 * Future operators must be added through an explicit language-versioned
 * compatibility change rather than silently changing the meaning of an
 * existing token sequence.
 *
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There is no semantic upper bound encoded by this grammar on:
 *
 *     expression size
 *     operand complexity
 *     identifier length
 *     nesting depth
 *     number of comparisons
 *     number of source declarations
 *     number of quantum resources
 *     number of classical resources
 *     number of hardware resources
 *     number of distributed resources
 *
 * Practical limits are compilation-environment resource limits and belong
 * outside the language semantics.
 *
 *
 * ============================================================================
 * FILE COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 * 1. ZamaniLexer provides the six authoritative operator tokens:
 *
 *        EQ NE LT LE GT GE
 *
 * 2. The master parser imports this parser grammar.
 *
 * 3. `comparisonExpression` is used at the appropriate precedence level.
 *
 * 4. `shiftExpression` is the immediate higher-precedence operand layer.
 *
 * 5. No other grammar file independently defines comparison precedence.
 *
 * 6. The AST preserves comparison operator identity.
 *
 * 7. Semantic analysis owns comparability/type legality.
 *
 * 8. IR lowering owns target-specific representation.
 *
 * 9. Quantum semantics continue through the canonical quantum IR boundary.
 *
 * 10. No hardware or machine limits are encoded here.
 *
 * 11. No Rust unsafe code is required.
 *
 * 12. Positive, negative, precedence, boundary, and cross-domain tests pass.
 */
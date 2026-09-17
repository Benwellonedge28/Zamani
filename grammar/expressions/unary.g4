/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/expressions/unary.g4
 *
 * Status:
 *     Canonical production unary-expression grammar component.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Edition 2021
 *
 * Safety:
 *     This grammar contains no embedded Rust code.
 *     Zamani compiler/runtime implementations consuming this grammar MUST
 *     remain safe Rust and MUST compile with:
 *
 *         #![forbid(unsafe_code)]
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the sole grammar component responsible for PREFIX UNARY
 * expression syntax.
 *
 * It defines:
 *
 *     unaryExpression
 *     unaryOperator
 *
 * It does NOT define:
 *
 *     expression
 *     postfixExpression
 *     binary expressions
 *     assignment
 *     types
 *     semantic operator legality
 *     operator overload resolution
 *     ownership
 *     borrowing
 *     effects
 *     resources
 *     capabilities
 *     quantum semantics
 *     hardware semantics
 *     IR
 *     runtime behavior
 *
 * ============================================================================
 * ARCHITECTURAL PIPELINE
 * ============================================================================
 *
 *     source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       | canonical operator tokens
 *       v
 *     Unary parser component
 *       |
 *       v
 *     domain-neutral frontend AST
 *       |
 *       v
 *     structural validation
 *       |
 *       v
 *     semantic analysis
 *       |
 *       +--> type checking
 *       +--> overload resolution
 *       +--> ownership / borrowing
 *       +--> effect checking
 *       +--> capability checking
 *       +--> resource checking
 *       |
 *       v
 *     canonical semantic representation
 *       |
 *       +--> classical representation
 *       +--> quantum::ir
 *       +--> HDL/hardware representation
 *       +--> distributed/data/AI representations
 *       |
 *       v
 *     optimization / lowering
 *       |
 *       +--> routing
 *       +--> scheduling
 *       +--> resilience
 *       +--> QEC
 *       +--> ZQN
 *       |
 *       v
 *     HAL / target realization
 *
 * This grammar MUST NOT bypass the frontend AST or create an IR.
 *
 * ============================================================================
 * AUTHORITY
 * ============================================================================
 *
 * Lexical operator spelling and token identity are owned by:
 *
 *     grammar/lexer/operators.g4
 *     grammar/lexer/operators.md
 *
 * This grammar consumes those tokens.
 *
 * This grammar MUST NOT declare lexer rules.
 *
 * The complete expression precedence hierarchy is owned by the expression
 * composition layer.
 *
 * The frontend AST owns the source-level AST representation.
 *
 * Semantic analysis owns operator meaning and legality.
 *
 * ============================================================================
 * CANONICAL OPERATOR CONTRACT
 * ============================================================================
 *
 * Prefix unary operators currently supported by the canonical operator
 * vocabulary are:
 *
 *     +x
 *     -x
 *     !x
 *     ~x
 *     &x
 *     *x
 *     ++x
 *     --x
 *
 * Their canonical lexer tokens are:
 *
 *     PLUS
 *     MINUS
 *     NOT
 *     TILDE
 *     AMPERSAND
 *     STAR
 *     INCREMENT
 *     DECREMENT
 *
 * This grammar MUST NOT invent aliases such as:
 *
 *     LOGICAL_NOT
 *     BIT_NOT
 *     ADDRESS_OF
 *     DEREFERENCE
 *     PREFIX_INCREMENT
 *     PREFIX_DECREMENT
 *
 * Semantic code may classify the operator later.
 *
 * ============================================================================
 * PREFIX / POSTFIX OWNERSHIP
 * ============================================================================
 *
 * Prefix:
 *
 *     ++x
 *     --x
 *     +x
 *     -x
 *     !x
 *     ~x
 *     &x
 *     *x
 *
 * belongs here.
 *
 * Postfix:
 *
 *     x++
 *     x--
 *
 * MUST belong to the canonical postfix-expression component.
 *
 * This file MUST NOT define postfix operators.
 *
 * ============================================================================
 * OPERAND OWNERSHIP
 * ============================================================================
 *
 * `postfixExpression` is supplied by the expression composition layer.
 *
 * This file deliberately references it rather than redefining it.
 *
 * The dependency is therefore:
 *
 *     Unary
 *       |
 *       +--> postfixExpression
 *
 * The expression composition grammar determines how the imported unary
 * component is positioned relative to multiplicative/additive/etc. levels.
 *
 * ============================================================================
 * RECURSION / ASSOCIATIVITY
 * ============================================================================
 *
 * Prefix unary operators are right-associative.
 *
 * Therefore:
 *
 *     !!!x
 *
 * parses structurally as:
 *
 *     !(!(!x))
 *
 * and:
 *
 *     --++x
 *
 * parses structurally as:
 *
 *     --(++x)
 *
 * There is deliberately no grammar-level finite unary nesting limit.
 *
 * ============================================================================
 * POCO-REAF / SCALABILITY
 * ============================================================================
 *
 * This grammar MUST NOT encode machine or implementation limits such as:
 *
 *     MAX_UNARY_DEPTH
 *     MAX_EXPRESSION_DEPTH
 *     MAX_POINTER_DEPTH
 *     MAX_REFERENCE_DEPTH
 *     MAX_OPERANDS
 *     MAX_QUBITS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_ACCELERATORS
 *
 * A program may contain arbitrarily many unary expressions subject only to
 * implementation/resource policies.
 *
 * "Infinity" here means:
 *
 *     no artificial language-level finite bound.
 *
 * Actual parser/compiler resource limits MUST remain implementation policy
 * rather than becoming language semantics.
 *
 * ============================================================================
 * TARGET INDEPENDENCE
 * ============================================================================
 *
 * The following are intentionally NOT represented by this grammar:
 *
 *     CPU identity
 *     GPU identity
 *     FPGA identity
 *     QPU identity
 *     physical qubit identity
 *     physical address
 *     register width
 *     bus width
 *     memory capacity
 *     hardware topology
 *     device count
 *     clock frequency
 *     accelerator count
 *
 * Unary syntax describes source-level computation.
 *
 * Hardware realization occurs later.
 *
 * ============================================================================
 * DOMAIN NEUTRALITY
 * ============================================================================
 *
 * The same syntax may be used with:
 *
 *     integers
 *     floating-point values
 *     vectors
 *     matrices
 *     tensors
 *     symbolic values
 *     references
 *     resources
 *     hardware-described values
 *     classical values
 *     quantum-related values
 *     HDL values
 *     distributed values
 *     future domain values
 *
 * The grammar does not decide whether an operator is valid for a type.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Unary syntax is intentionally quantum-neutral.
 *
 * A quantum-aware expression may syntactically contain a unary expression,
 * but this grammar does not decide whether that operation is meaningful for
 * a quantum value.
 *
 * It MUST NOT encode:
 *
 *     physical qubit mappings
 *     logical-to-physical mappings
 *     gate implementations
 *     QPU topology
 *     calibration
 *     noise
 *     QEC
 *     ZQN
 *     routing
 *     scheduling
 *
 * If semantic analysis determines that an expression participates in quantum
 * computation, lowering proceeds through the canonical:
 *
 *     quantum::ir
 *
 * boundary.
 *
 * This grammar MUST NOT introduce another quantum IR.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Unary operators may syntactically operate on HDL/hardware-domain values:
 *
 *     ~signal
 *     !enable
 *     &signal
 *     *reference
 *     -offset
 *
 * This grammar does not determine:
 *
 *     signal width
 *     register width
 *     address width
 *     FPGA family
 *     ASIC technology
 *     physical clock
 *     physical resource
 *     target architecture
 *
 * Such information belongs to semantic/resource/hardware/target layers.
 *
 * ============================================================================
 * MEMORY / OWNERSHIP INTEGRATION
 * ============================================================================
 *
 * `&` and `*` are syntactically unary operators.
 *
 * Their meaning may correspond to:
 *
 *     borrowing
 *     reference creation
 *     address formation
 *     dereference
 *     domain-specific reference semantics
 *
 * depending on the operand type and language semantics.
 *
 * This grammar MUST NOT duplicate:
 *
 *     grammar/types/reference-types.g4
 *
 * or:
 *
 *     grammar/memory/borrowing.g4
 *
 * Reference type syntax and ownership semantics are downstream contracts.
 *
 * ============================================================================
 * MUTATION INTEGRATION
 * ============================================================================
 *
 * `++` and `--` are syntactically valid prefix operators.
 *
 * Semantic analysis determines whether the operand is:
 *
 *     assignable
 *     mutable
 *     uniquely owned
 *     borrow-compatible
 *     effect-compatible
 *     concurrency-safe
 *
 * This grammar does not perform those checks.
 *
 * ============================================================================
 * CONSTANT-EVALUATION INTEGRATION
 * ============================================================================
 *
 * Unary expressions may occur in compile-time expressions.
 *
 * Examples:
 *
 *     -1
 *     +1
 *     ~mask
 *     !condition
 *
 * This grammar does not perform:
 *
 *     constant folding
 *     overflow checking
 *     arbitrary-precision evaluation
 *     symbolic evaluation
 *     target-specific evaluation
 *
 * Those belong downstream.
 *
 * No machine integer width is implied by this grammar.
 *
 * ============================================================================
 * EXTENSIBILITY
 * ============================================================================
 *
 * The universal grammar deliberately uses the canonical operator token set
 * rather than enumerating semantic types.
 *
 * Therefore:
 *
 *     -T
 *
 * does not require separate grammar rules for:
 *
 *     -Int
 *     -Float
 *     -Vector
 *     -Matrix
 *     -Tensor
 *     -Symbolic
 *     -HardwareValue
 *
 * New semantic types do not require changes to this grammar merely because
 * they support an already-existing unary operator.
 *
 * New core unary operators require:
 *
 *     1. lexical specification;
 *     2. canonical lexer token;
 *     3. compatibility decision;
 *     4. this grammar update;
 *     5. AST/operator contract;
 *     6. semantic contract;
 *     7. diagnostics;
 *     8. tests.
 *
 * ============================================================================
 * ERROR BOUNDARY
 * ============================================================================
 *
 * Parser errors:
 *
 *     -
 *     !
 *     ~
 *     &
 *     *
 *     ++
 *     --
 *
 * without a valid operand are syntax errors.
 *
 * Semantic errors include:
 *
 *     ++immutable
 *     --immutable
 *     *non_reference
 *     &invalid_operand
 *     !unsupported_type
 *     ~unsupported_type
 *
 * when those combinations are not semantically defined.
 *
 * The parser MUST NOT turn semantic type errors into grammar-specific rules.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * For the same:
 *
 *     source token stream
 *     grammar version
 *     canonical lexer
 *
 * this grammar must produce the same parse structure.
 *
 * Parsing MUST NOT depend on:
 *
 *     CPU count
 *     memory capacity
 *     GPU availability
 *     QPU availability
 *     hardware topology
 *     calibration state
 *     network state
 *     runtime scheduler state
 *     backend selection
 *     target availability
 *     wall-clock time
 *     randomness
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Every successful unary parse maps to the existing native frontend unary
 * AST contract:
 *
 *     src/frontend/ast/node/expressions/unary.rs
 *
 * The AST requires:
 *
 *     operator
 *     operand
 *     source span
 *
 * The AST intentionally stores an open operator spelling rather than a
 * closed semantic enum.
 *
 * Therefore this grammar MUST preserve the operator token/spelling required
 * by the parser-to-AST adapter.
 *
 * The grammar does not construct the AST.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis receives:
 *
 *     operator
 *     operand
 *
 * and determines:
 *
 *     name resolution
 *     type validity
 *     overload resolution
 *     conversions
 *     ownership
 *     borrowing
 *     effects
 *     capabilities
 *     resource requirements
 *     domain legality
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar has no IR dependency.
 *
 * In particular:
 *
 *     unary.g4 -> quantum::ir
 *
 * is prohibited.
 *
 * The legal direction is:
 *
 *     unary syntax
 *         |
 *         v
 *     frontend AST
 *         |
 *         v
 *     semantic analysis
 *         |
 *         v
 *     canonical semantic representation
 *         |
 *         +--> quantum::ir
 *         +--> classical representation
 *         +--> HDL/hardware representation
 *
 * ============================================================================
 * COMPILER CONTRACT
 * ============================================================================
 *
 * Adding a new compiler backend MUST NOT require changing this grammar if
 * the source-level unary language remains unchanged.
 *
 * Backend-specific lowering belongs downstream.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * This grammar has no runtime dependency.
 *
 * ============================================================================
 * TOOLING CONTRACT
 * ============================================================================
 *
 * Formatters, syntax highlighters, language servers, linters, refactoring
 * tools, and source analyzers should consume the canonical parser structure
 * rather than reimplementing unary syntax independently.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing canonical spellings:
 *
 *     +
 *     -
 *     !
 *     ~
 *     &
 *     *
 *     ++
 *     --
 *
 * must not be silently assigned new meanings.
 *
 * Any spelling change requires:
 *
 *     language-version policy
 *     migration policy
 *     compatibility tests
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no:
 *
 *     hardware IDs
 *     qubit IDs
 *     CPU counts
 *     GPU counts
 *     FPGA counts
 *     node counts
 *     memory sizes
 *     register widths
 *     tensor limits
 *     vector limits
 *     expression-depth constants
 *     operand-count constants
 *     backend-specific constants
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive:
 *
 *     +x
 *     -x
 *     !x
 *     ~x
 *     &x
 *     *x
 *     ++x
 *     --x
 *
 * Nested:
 *
 *     !!x
 *     ~~x
 *     --++x
 *     ++--x
 *     -~x
 *     !~x
 *     **x
 *     &&x
 *
 * Composition:
 *
 *     -x + y
 *     !(x == y)
 *     ~mask & value
 *     *ptr + offset
 *     ++x * y
 *
 * Parenthesized operands:
 *
 *     -(x + y)
 *     !(a == b)
 *     ~(mask | other)
 *
 * Chained postfix operands:
 *
 *     -value.field
 *     !call()
 *     *array[index]
 *     &object.member
 *
 * Negative syntax:
 *
 *     +
 *     -
 *     !
 *     ~
 *     &
 *     *
 *     ++
 *     --
 *
 * Boundary/scalability:
 *
 *     arbitrarily nested unary operators within implementation resources.
 *
 * Semantic-negative tests MUST NOT be placed in this grammar's syntax tests.
 * They belong to semantic analysis tests.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] only prefix unary syntax is owned here;
 *     [x] postfix syntax is not duplicated;
 *     [x] lexer rules are not duplicated;
 *     [x] canonical token names are consumed;
 *     [x] unary nesting is unbounded at language level;
 *     [x] no hardware limits exist;
 *     [x] no resource limits exist;
 *     [x] no semantic rules are embedded;
 *     [x] no quantum IR is created;
 *     [x] canonical quantum::ir remains downstream;
 *     [x] AST mapping is predetermined;
 *     [x] ownership/reference semantics remain downstream;
 *     [x] ++/-- are included;
 *     [x] deterministic parsing is preserved;
 *     [x] diagnostics have clear syntax/semantic boundaries;
 *     [x] integration points are explicitly defined;
 *     [x] positive/negative/boundary/scalability tests are defined.
 *
 * ============================================================================
 */

parser grammar Unary;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * PREFIX UNARY EXPRESSION
 * ============================================================================
 *
 * Right recursion is intentional.
 *
 *     unaryOperator unaryExpression
 *
 * means:
 *
 *     - -x
 *
 * is structurally:
 *
 *     -(-x)
 *
 * The base case delegates to the canonical postfix expression layer.
 *
 * `postfixExpression` MUST be supplied by the importing expression grammar.
 */
unaryExpression
    : unaryOperator unaryExpression
    | postfixExpression
    ;


/* ============================================================================
 * PREFIX UNARY OPERATORS
 * ============================================================================
 *
 * Token identity is owned by the canonical lexer.
 *
 * Semantic interpretation is deliberately deferred.
 */
unaryOperator
    : PLUS
    | MINUS
    | NOT
    | TILDE
    | AMPERSAND
    | STAR
    | INCREMENT
    | DECREMENT
    ;
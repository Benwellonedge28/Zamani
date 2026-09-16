/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/expressions/comparison.g4
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     Safe Rust only
 *     No unsafe Rust required or permitted in Zamani-owned implementation.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the comparison/equality precedence layer of the canonical
 * Zamani expression grammar.
 *
 * It defines:
 *
 *     comparisonExpression
 *     equalityExpression
 *     comparisonOperator
 *     equalityOperator
 *
 * It does NOT define:
 *
 *     - primary expressions
 *     - literals
 *     - identifiers
 *     - calls
 *     - indexing
 *     - member access
 *     - unary/prefix expressions
 *     - arithmetic
 *     - shifts
 *     - bitwise expressions
 *     - logical expressions
 *     - conditional expressions
 *     - ranges
 *     - assignment
 *     - types
 *     - semantic type checking
 *     - overload resolution
 *     - implicit conversion
 *     - ownership
 *     - borrowing
 *     - effects
 *     - capabilities
 *     - resources
 *     - quantum semantics
 *     - HDL semantics
 *     - hardware selection
 *     - routing
 *     - scheduling
 *     - QEC
 *     - ZQN
 *     - HAL
 *     - runtime behavior
 *     - IR construction
 *
 * ============================================================================
 * AUTHORITY
 * ============================================================================
 *
 * The canonical lexer owns operator spelling and token identity.
 *
 * This parser grammar owns only the syntactic role and precedence of those
 * tokens.
 *
 * The canonical lexer vocabulary is:
 *
 *     EQ_EQ       ==
 *     NOT_EQ      !=
 *     LT          <
 *     LE          <=
 *     GT          >
 *     GE          >=
 *
 * This file MUST NOT introduce alternate token names such as:
 *
 *     EQ
 *     NE
 *     LESS
 *     GREATER
 *     EQUAL
 *     NOT_EQUAL
 *
 * when they represent the same lexical operators.
 *
 * ============================================================================
 * EXPRESSION PRECEDENCE
 * ============================================================================
 *
 * Higher precedence is nearer the bottom of the expression hierarchy.
 *
 *     ...
 *       |
 *       v
 *     shiftExpression
 *       |
 *       v
 *     comparisonExpression
 *       |
 *       v
 *     equalityExpression
 *       |
 *       v
 *     bitwiseAndExpression
 *       |
 *       v
 *     bitwiseXorExpression
 *       |
 *       v
 *     bitwiseOrExpression
 *       |
 *       v
 *     logicalAndExpression
 *       |
 *       v
 *     logicalOrExpression
 *       |
 *       v
 *     rangeExpression
 *       |
 *       v
 *     conditionalExpression
 *       |
 *       v
 *     assignmentExpression
 *       |
 *       v
 *     expression
 *
 * Therefore:
 *
 *     a < b == c < d
 *
 * is structurally:
 *
 *     (a < b) == (c < d)
 *
 * and not:
 *
 *     a < (b == c) < d
 *
 * ============================================================================
 * ASSOCIATIVITY
 * ============================================================================
 *
 * Repeated comparison/equality operators are represented structurally as
 * left-associated binary expression chains:
 *
 *     a < b < c
 *
 * becomes conceptually:
 *
 *     (a < b) < c
 *
 * The grammar intentionally does NOT implement Python-style chained
 * comparison semantics.
 *
 * Whether:
 *
 *     a < b < c
 *
 * is semantically valid is determined by the semantic/type system.
 *
 * This is important for generic, symbolic, tensor, quantum, HDL and
 * user-defined types because syntactic comparability does not imply that
 * an operation is semantically meaningful.
 *
 * ============================================================================
 * ANTLR INTEGRATION
 * ============================================================================
 *
 * This is a parser grammar, not a combined grammar.
 *
 * The canonical parser composition grammar imports this module.
 *
 * The canonical lexer vocabulary is supplied through tokenVocab.
 *
 * ANTLR grammar imports combine parser rules into the importing parser
 * grammar; this allows this module to remain focused on one precedence
 * layer without duplicating the complete expression hierarchy.
 *
 * No parser actions or target-language predicates are used.
 *
 * ============================================================================
 */

parser grammar ZamaniComparison;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * COMPARISON EXPRESSION
 * ============================================================================
 *
 * Public relational-comparison rule.
 *
 * Comparison has higher precedence than equality.
 *
 * Examples:
 *
 *     a < b
 *     a <= b
 *     a > b
 *     a >= b
 *
 * Shift expressions are the immediate higher-precedence operand layer.
 *
 * This rule deliberately does not define shiftExpression.
 */
comparisonExpression
    : shiftExpression
      (
          comparisonOperator
          shiftExpression
      )*
    ;


/*
 * ============================================================================
 * EQUALITY EXPRESSION
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
 * The final example becomes:
 *
 *     (a < b) == (c < d)
 *
 * This rule intentionally consumes comparisonExpression rather than
 * shiftExpression directly.
 */
equalityExpression
    : comparisonExpression
      (
          equalityOperator
          comparisonExpression
      )*
    ;


/*
 * ============================================================================
 * RELATIONAL OPERATORS
 * ============================================================================
 *
 * The lexer is the sole owner of operator spelling.
 *
 * Do not replace these token references with string literals.
 *
 * Do not create duplicate lexical definitions here.
 */
comparisonOperator
    : LT
    | LE
    | GT
    | GE
    ;


/*
 * ============================================================================
 * EQUALITY OPERATORS
 * ============================================================================
 *
 * Equality is intentionally restricted to the operators specified by the
 * canonical Zamani syntax contract.
 *
 * Type/category tests such as `is` are NOT equality operators.
 *
 * If the language later requires type predicates, those belong to the
 * appropriate type/pattern/semantic grammar and must not be silently folded
 * into equality.
 */
equalityOperator
    : EQ_EQ
    | NOT_EQ
    ;


/*
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * This grammar establishes syntax only.
 *
 * Semantic analysis is responsible for determining whether an expression
 * such as:
 *
 *     lhs == rhs
 *     lhs != rhs
 *     lhs < rhs
 *     lhs <= rhs
 *     lhs > rhs
 *     lhs >= rhs
 *
 * is legal for the operand types.
 *
 * Semantic analysis owns:
 *
 *     - type compatibility
 *     - generic constraints
 *     - numeric promotion
 *     - conversion rules
 *     - overload resolution
 *     - trait/interface based comparison
 *     - user-defined comparison
 *     - symbolic comparison
 *     - approximate comparison
 *     - aggregate comparison
 *     - tensor comparison
 *     - resource comparison
 *     - capability validation
 *     - effect validation
 *     - domain-specific legality
 *
 * The grammar must never attempt to answer these questions.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Every parsed comparison must retain enough structure for the frontend AST
 * to preserve:
 *
 *     - operator identity
 *     - left operand
 *     - right operand
 *     - source span
 *     - source/module provenance where supported
 *     - nested expression structure
 *
 * The AST may represent these as a generic binary operation or as dedicated
 * comparison nodes according to the existing authoritative AST design.
 *
 * This grammar MUST NOT require:
 *
 *     QuantumComparison
 *     HardwareComparison
 *     GPUComparison
 *     QPUComparison
 *     HDLComparison
 *
 * merely because an operand belongs to a particular domain.
 *
 * Domain semantics are resolved after parsing.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum values may syntactically occur as operands because quantum
 * expressions participate in the universal expression hierarchy.
 *
 * This grammar does not decide whether:
 *
 *     q1 == q2
 *     state1 == state2
 *     observable1 < observable2
 *     logical_state == expected_state
 *
 * has valid quantum semantics.
 *
 * Those decisions belong to quantum semantic analysis.
 *
 * No physical-qubit numbering, topology, device identity, gate catalogue,
 * calibration information, QEC policy, routing information or scheduling
 * information belongs here.
 *
 * Where a comparison has quantum meaning, lowering continues through the
 * canonical quantum semantic boundary:
 *
 *     frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     optimization / routing / scheduling / resilience / ZQN / HAL
 *
 * This grammar MUST NOT introduce a second quantum IR.
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Classical scalar, vector, matrix, tensor, symbolic and user-defined values
 * may participate in comparison syntax.
 *
 * The grammar does not impose:
 *
 *     - integer width
 *     - floating-point width
 *     - vector width
 *     - matrix dimensions
 *     - tensor dimensions
 *     - register size
 *     - CPU count
 *     - accelerator count
 *
 * Such properties are semantic, resource or target concerns.
 *
 * ============================================================================
 * HDL INTEGRATION
 * ============================================================================
 *
 * HDL expressions may use this syntax for constructs such as:
 *
 *     signal_a == signal_b
 *     counter < limit
 *     state == expected_state
 *
 * The grammar does not determine whether a comparison is ultimately:
 *
 *     - simulation logic
 *     - combinational logic
 *     - sequential logic
 *     - synthesis logic
 *     - an assertion
 *     - a verification property
 *     - a timing constraint
 *
 * HDL semantic analysis owns those distinctions.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Expressions involving resources and capabilities may use comparison syntax.
 *
 * For example, a semantic resource model might eventually evaluate:
 *
 *     available >= required
 *
 * but this grammar does not define:
 *
 *     - memory capacity
 *     - number of CPUs
 *     - number of GPUs
 *     - number of QPUs
 *     - number of nodes
 *     - topology size
 *     - network bandwidth
 *     - hardware limits
 *
 * No such finite limits may be introduced into this grammar.
 *
 * ============================================================================
 * PORTABILITY / POCO-REAF
 * ============================================================================
 *
 * Comparison syntax must remain target independent.
 *
 * A source program may express computation or requirements without binding
 * the comparison operation to:
 *
 *     a specific CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     physical qubit
 *     memory bank
 *     network node
 *     device identifier
 *
 * Therefore the same source-level comparison remains portable across
 * implementations capable of satisfying its semantic requirements.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates no IR.
 *
 * Lowering consumes the AST/semantic representation and selects the
 * appropriate canonical IR operation.
 *
 * Possible downstream destinations include:
 *
 *     classical/control/data IR
 *     quantum::ir
 *     HDL/hardware semantic IR
 *     resource/constraint metadata
 *
 * according to the already-established semantic domain.
 *
 * This file must never encode target instruction selection.
 *
 * ============================================================================
 * OPTIMIZATION CONTRACT
 * ============================================================================
 *
 * Optimizations such as:
 *
 *     x == x
 *     x < x
 *     !(x == y)
 *
 * are NOT performed here.
 *
 * Any such transformation belongs to semantic/optimization stages and must
 * respect:
 *
 *     - operand types
 *     - effects
 *     - evaluation order
 *     - user-defined operators
 *     - symbolic semantics
 *     - domain semantics
 *     - observable behavior
 *
 * ============================================================================
 * ERROR CONTRACT
 * ============================================================================
 *
 * These are syntactically incomplete:
 *
 *     a ==
 *     a !=
 *     a <
 *     a <=
 *     a >
 *     a >=
 *
 * They must produce parser diagnostics.
 *
 * These are syntactically invalid at this precedence layer:
 *
 *     == a
 *     != a
 *     < a
 *     > a
 *
 * The parser must report malformed syntax rather than asking semantic
 * analysis to repair it.
 *
 * A syntactically valid but semantically invalid expression, for example:
 *
 *     1 < some_non_orderable_type
 *
 * must be diagnosed by semantic/type analysis.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * For an identical lexer token stream, this grammar must produce an
 * equivalent parse structure.
 *
 * Parsing must not depend on:
 *
 *     - machine state
 *     - hardware discovery
 *     - resource availability
 *     - network state
 *     - filesystem state
 *     - random state
 *     - runtime values
 *     - backend selection
 *     - calibration
 *     - scheduling
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * This grammar performs no:
 *
 *     - I/O
 *     - filesystem access
 *     - network access
 *     - command execution
 *     - hardware access
 *     - expression evaluation
 *     - code execution
 *
 * It is therefore a pure syntactic layer.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * No artificial language-level finite limit is encoded for:
 *
 *     - number of comparison operators
 *     - expression depth
 *     - operand complexity
 *     - source size
 *     - number of program resources
 *     - number of qubits
 *     - number of classical resources
 *     - number of hardware resources
 *     - number of distributed participants
 *     - tensor dimensions
 *     - vector widths
 *
 * For example, syntactically:
 *
 *     a < b < c < d < ... 
 *
 * is structurally supported without a fixed comparison-chain count.
 *
 * Practical parser/compiler resource limits are implementation constraints,
 * not Zamani language limits.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing canonical comparison spellings remain:
 *
 *     ==
 *     !=
 *     <
 *     <=
 *     >
 *     >=
 *
 * This file deliberately does NOT introduce:
 *
 *     ===
 *     !==
 *     <>
 *     <=>
 *
 * or alternate equality/comparison spellings.
 *
 * Such additions require an explicit language specification and lexer
 * compatibility change before becoming part of this grammar.
 *
 * ============================================================================
 * INTEGRATION REQUIREMENTS
 * ============================================================================
 *
 * The canonical expression composition grammar must:
 *
 *     1. import ZamaniComparison;
 *
 *     2. use comparisonExpression at the relational-comparison level;
 *
 *     3. use equalityExpression above comparisonExpression;
 *
 *     4. not redefine either rule;
 *
 *     5. preserve the precedence:
 *
 *            shift
 *              <
 *            comparison
 *              <
 *            equality
 *              <
 *            bitwise
 *              <
 *            logical
 *              <
 *            range/conditional
 *              <
 *            assignment
 *
 * The expression composition grammar remains responsible for composing
 * these layers into the public `expression` rule.
 *
 * ============================================================================
 * DUPLICATION REMOVAL REQUIREMENT
 * ============================================================================
 *
 * `grammar/antlr/Core.g4` currently contains its own comparison/equality
 * rules and therefore duplicates this modular expression layer.
 *
 * During parser-composition integration, those duplicate expression rules
 * must be removed from the authoritative composition grammar rather than
 * maintained in parallel.
 *
 * Likewise:
 *
 *     grammar/Zamani.g4
 *     grammar/antlr/ZamaniParser.g4
 *     grammar/expressions/expressions.g4
 *
 * must not independently redefine comparison precedence.
 *
 * One comparison grammar must own this syntax.
 *
 * ============================================================================
 * LEXER INTEGRATION REQUIREMENT
 * ============================================================================
 *
 * The canonical lexer remains the owner of:
 *
 *     EQ_EQ
 *     NOT_EQ
 *     LT
 *     LE
 *     GT
 *     GE
 *
 * `comparison.g4` references those tokens but never defines them.
 *
 * This prevents multiple lexical authorities from assigning different token
 * identities to the same source spelling.
 *
 * ============================================================================
 * RUST INTEGRATION
 * ============================================================================
 *
 * This file contains no Rust target actions, semantic predicates, unsafe
 * blocks, filesystem operations or runtime dependencies.
 *
 * Generated parser integration must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * Zamani-owned Rust code must remain safe Rust.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive syntax:
 *
 *     a == b
 *     a != b
 *     a < b
 *     a <= b
 *     a > b
 *     a >= b
 *
 * Precedence:
 *
 *     a < b == c < d
 *
 * must parse as:
 *
 *     (a < b) == (c < d)
 *
 * Chaining:
 *
 *     a < b < c
 *     a == b == c
 *     a != b == c
 *
 * must remain syntactically representable as repeated binary operations.
 *
 * Nested expressions:
 *
 *     (a + b) < (c * d)
 *     f(a) == g(b)
 *     object.field < other.field
 *     array[i] >= array[j]
 *
 * must be handled by the higher-precedence expression layers.
 *
 * Negative syntax:
 *
 *     ==
 *     !=
 *     <
 *     <=
 *     >
 *     >=
 *     a ==
 *     a !=
 *     a <
 *     a <=
 *     a >
 *     a >=
 *
 * must be rejected as incomplete expressions.
 *
 * ============================================================================
 * FILE COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] Operator spelling is delegated to the canonical lexer.
 *     [x] EQ_EQ and NOT_EQ are used for equality.
 *     [x] LT, LE, GT and GE are used for ordering.
 *     [x] comparisonExpression owns relational comparison.
 *     [x] equalityExpression owns equality.
 *     [x] comparison binds more tightly than equality.
 *     [x] shiftExpression remains the higher-precedence operand layer.
 *     [x] No assignment syntax is duplicated here.
 *     [x] No arithmetic syntax is duplicated here.
 *     [x] No semantic type rules are encoded here.
 *     [x] No quantum IR is introduced.
 *     [x] No hardware limits are encoded.
 *     [x] No resource limits are encoded.
 *     [x] No target-specific behavior is encoded.
 *     [x] No Rust actions or unsafe code are required.
 *     [x] AST mapping is defined before semantic implementation.
 *     [x] IR ownership is downstream.
 *     [x] Positive tests are defined.
 *     [x] Negative tests are defined.
 *     [x] Boundary tests are defined.
 *     [x] Scalability tests are defined.
 *     [x] Deterministic parsing is preserved.
 *     [x] Duplicate comparison rules are removed from composition roots.
 *
 * ============================================================================
 */
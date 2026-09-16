/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/expressions/bitwise.g4
 *
 * Status:
 *     Canonical production parser component for bitwise expressions.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     Safe Rust only
 *
 * Safety:
 *     This grammar contains no embedded Rust code, actions, predicates,
 *     I/O, runtime calls, hardware access, or unsafe code.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the source-language precedence and associativity of the
 * binary bitwise operators:
 *
 *     &
 *     ^
 *     |
 *
 * The precedence hierarchy owned here is:
 *
 *     bitwiseOrExpression
 *             |
 *             v
 *     bitwiseXorExpression
 *             |
 *             v
 *     bitwiseAndExpression
 *             |
 *             v
 *     equalityExpression
 *
 * Therefore:
 *
 *     a | b ^ c & d
 *
 * is structurally:
 *
 *     a | (b ^ (c & d))
 *
 * This file defines syntax only.
 *
 * It does not determine whether the operands are:
 *
 *     integers
 *     arbitrary-precision integers
 *     fixed-width integers
 *     bit vectors
 *     masks
 *     packed values
 *     SIMD/vector values
 *     tensors
 *     hardware signals
 *     registers
 *     accelerator values
 *     user-defined values
 *
 * Those decisions belong to semantic analysis.
 *
 * ============================================================================
 * OWNS
 * ============================================================================
 *
 * This file owns:
 *
 *     - bitwiseExpression
 *     - bitwiseOrExpression
 *     - bitwiseXorExpression
 *     - bitwiseAndExpression
 *     - bitwise OR precedence
 *     - bitwise XOR precedence
 *     - bitwise AND precedence
 *     - left-associative repetition within each bitwise level
 *     - composition with equalityExpression
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     - lexical operator spelling
 *     - lexer tokens
 *     - identifiers
 *     - literals
 *     - primary expressions
 *     - postfix expressions
 *     - unary expressions
 *     - arithmetic expressions
 *     - shift expressions
 *     - comparison expressions
 *     - equality expressions
 *     - logical expressions
 *     - conditional expressions
 *     - ranges
 *     - assignments
 *     - types
 *     - type checking
 *     - overload resolution
 *     - conversions
 *     - constant evaluation
 *     - ownership
 *     - borrowing
 *     - effects
 *     - capabilities
 *     - resources
 *     - hardware selection
 *     - hardware topology
 *     - scheduling
 *     - routing
 *     - optimization
 *     - QEC
 *     - ZQN
 *     - HAL
 *     - calibration
 *     - runtime behavior
 *     - ABI selection
 *     - backend selection
 *     - IR construction
 *     - machine-specific limits
 *
 * ============================================================================
 * LEXER AUTHORITY
 * ============================================================================
 *
 * The canonical lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * The lexer owns the spelling and identity of the bitwise operators.
 *
 * Canonical tokens consumed here:
 *
 *     AMPERSAND    &
 *     CARET        ^
 *     PIPE         |
 *
 * This file MUST NOT redefine those tokens and MUST NOT replace them with
 * string literals.
 *
 * The separation is:
 *
 *     source characters
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     canonical operator token
 *          |
 *          v
 *     this grammar
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     canonical semantic representation / IR
 *
 * ============================================================================
 * PRECEDENCE CONTRACT
 * ============================================================================
 *
 * The universal expression hierarchy around this file is:
 *
 *     logicalOrExpression
 *             |
 *     logicalAndExpression
 *             |
 *     bitwiseOrExpression       <-- this file
 *             |
 *     bitwiseXorExpression      <-- this file
 *             |
 *     bitwiseAndExpression      <-- this file
 *             |
 *     equalityExpression
 *             |
 *     comparisonExpression
 *             |
 *     shiftExpression
 *             |
 *     additiveExpression
 *             |
 *     multiplicativeExpression
 *             |
 *     prefix/unary expression
 *             |
 *     postfix expression
 *             |
 *     primary expression
 *
 * Thus:
 *
 *     &
 *
 * binds more tightly than:
 *
 *     ^
 *
 * which binds more tightly than:
 *
 *     |
 *
 * The equality/comparison/shift layers are supplied by their respective
 * expression grammar components.
 *
 * ============================================================================
 * ASSOCIATIVITY
 * ============================================================================
 *
 * Each binary bitwise operator level is left associative.
 *
 * Therefore:
 *
 *     a & b & c
 *
 * has the structural form:
 *
 *     (a & b) & c
 *
 * Likewise:
 *
 *     a ^ b ^ c
 *
 * becomes:
 *
 *     (a ^ b) ^ c
 *
 * and:
 *
 *     a | b | c
 *
 * becomes:
 *
 *     (a | b) | c
 *
 * Iterative repetition is deliberately used instead of recursive
 * left-recursive chains.
 *
 * This permits arbitrary-length source-level chains without introducing a
 * grammar-defined operator-count limit.
 *
 * ============================================================================
 * PUBLIC ENTRY RULE
 * ============================================================================
 *
 * `bitwiseExpression` is the public entry rule for this component.
 *
 * The canonical expression composition layer may consume this rule when
 * composing the complete expression hierarchy.
 *
 * The individual precedence rules remain public parser rules because the
 * higher-level logical-expression grammar needs `bitwiseOrExpression`.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * This grammar depends on:
 *
 *     equalityExpression
 *
 * supplied by:
 *
 *     grammar/expressions/comparison.g4
 *
 * The dependency direction is:
 *
 *     higher-precedence/lower-level expressions
 *             |
 *             v
 *     comparison.g4
 *             |
 *             v
 *     bitwise.g4
 *             |
 *             v
 *     logical-expression composition
 *
 * More precisely, in terms of precedence:
 *
 *     equalityExpression
 *             ^
 *             |
 *     bitwiseAndExpression
 *             ^
 *             |
 *     bitwiseXorExpression
 *             ^
 *             |
 *     bitwiseOrExpression
 *             ^
 *             |
 *     logicalAndExpression
 *             ^
 *             |
 *     logicalOrExpression
 *
 * `bitwise.g4` MUST NOT define `equalityExpression`.
 *
 * `comparison.g4` MUST NOT depend on `bitwise.g4`.
 *
 * `bitwise.g4` MUST NOT import the complete expression composition grammar.
 *
 * This prevents cyclic grammar dependencies.
 *
 * ============================================================================
 * ANTLR COMPOSITION CONTRACT
 * ============================================================================
 *
 * This is a parser grammar, not a combined lexer/parser grammar.
 *
 * Its header is intentionally:
 *
 *     parser grammar ZamaniBitwise;
 *
 * and its lexical vocabulary is supplied through:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * The composing expression grammar is responsible for importing this module.
 *
 * The module itself does not define lexer rules.
 *
 * ANTLR parser-grammar composition must remain the mechanism used to combine
 * these independent precedence layers.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar does not construct the frontend AST.
 *
 * The parser output must preserve:
 *
 *     - operator identity
 *     - left operand
 *     - right operand
 *     - operand ordering
 *     - source span
 *     - nested precedence structure
 *
 * The domain-neutral frontend AST may normalize a parser chain into generic
 * binary-operation nodes.
 *
 * Conceptually:
 *
 *     a | b ^ c & d
 *
 * becomes:
 *
 *     BitwiseOr(
 *         a,
 *         BitwiseXor(
 *             b,
 *             BitwiseAnd(c, d)
 *         )
 *     )
 *
 * The actual AST representation must follow the repository's canonical
 * frontend AST contract rather than introducing a grammar-specific AST type.
 *
 * This grammar MUST NOT require domain-specific nodes such as:
 *
 *     QuantumBitwiseOperation
 *     GPUBitwiseOperation
 *     FPGAOperation
 *     HDLBitwiseOperation
 *
 * merely because the operands belong to those domains.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing establishes only syntactic structure.
 *
 * Semantic analysis determines whether a particular bitwise expression is
 * legal for its resolved operand types.
 *
 * Examples that may be semantically supported by different type systems
 * include:
 *
 *     integer & integer
 *     bit_vector & bit_vector
 *     mask & value
 *     signal_a ^ signal_b
 *     vector_a | vector_b
 *
 * Semantic analysis owns:
 *
 *     - operand type compatibility
 *     - width compatibility
 *     - arbitrary-width integer semantics
 *     - signed/unsigned semantics
 *     - bit-vector semantics
 *     - tensor/vector semantics
 *     - broadcasting rules
 *     - user-defined operators
 *     - trait/interface constraints
 *     - conversions
 *     - result type
 *     - overflow behavior where applicable
 *     - symbolic semantics
 *     - domain-specific legality
 *
 * This grammar MUST NOT encode any of those rules.
 *
 * ============================================================================
 * ARBITRARY WIDTH / SCALABILITY
 * ============================================================================
 *
 * This file imposes no source-language limit on:
 *
 *     - integer width
 *     - bit-vector width
 *     - signal width
 *     - tensor dimension
 *     - vector length
 *     - number of chained operators
 *     - number of operands
 *     - expression depth
 *     - source-file size
 *
 * For example, the grammar does not contain:
 *
 *     MAX_BITS
 *     MAX_WIDTH
 *     MAX_VECTOR_WIDTH
 *     MAX_SIGNAL_WIDTH
 *     MAX_OPERATORS
 *     MAX_EXPRESSION_DEPTH
 *
 * or equivalent limits.
 *
 * "Infinity" here means that the language grammar introduces no artificial
 * finite limit. Actual compilation and execution remain bounded by available
 * resources and implementation policy.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Bitwise syntax describes computation rather than a particular machine.
 *
 * This grammar MUST NOT encode:
 *
 *     - CPU count
 *     - core count
 *     - thread count
 *     - GPU count
 *     - FPGA count
 *     - QPU count
 *     - register count
 *     - physical register width
 *     - memory capacity
 *     - device count
 *     - node count
 *     - network topology
 *     - accelerator topology
 *     - physical qubit identifiers
 *
 * The same source expression may therefore participate in compilation for
 * different target systems when the semantic requirements can be satisfied.
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Classical expressions may use:
 *
 *     &
 *     ^
 *     |
 *
 * for supported scalar, arbitrary-precision, bit-vector, packed, vector,
 * tensor, mask, or user-defined types.
 *
 * The grammar does not determine which of these types exist.
 *
 * Type and capability systems determine that downstream.
 *
 * ============================================================================
 * HDL INTEGRATION
 * ============================================================================
 *
 * HDL expressions may use the same source-level syntax for hardware-oriented
 * semantic values, including signals, masks, buses, vectors and other
 * hardware-described values.
 *
 * This grammar does not decide:
 *
 *     - signal width
 *     - register width
 *     - bus width
 *     - FPGA resource usage
 *     - ASIC implementation
 *     - clock frequency
 *     - timing
 *     - placement
 *     - routing
 *     - synthesis strategy
 *
 * Those belong to the HDL/hardware semantic and lowering layers.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Bitwise operators remain domain-neutral.
 *
 * This grammar MUST NOT interpret:
 *
 *     &
 *     ^
 *     |
 *
 * as quantum gates or quantum hardware operations.
 *
 * It MUST NOT:
 *
 *     - allocate qubits
 *     - measure qubits
 *     - select physical qubits
 *     - select QPUs
 *     - perform routing
 *     - perform scheduling
 *     - invoke QEC
 *     - invoke ZQN
 *     - invoke HAL
 *     - select a quantum backend
 *
 * If semantic analysis determines that an expression has a valid quantum
 * meaning, downstream lowering must continue through the canonical:
 *
 *     quantum::ir
 *
 * boundary.
 *
 * This grammar must never create a second quantum IR.
 *
 * ============================================================================
 * CROSS-DOMAIN INTEGRATION
 * ============================================================================
 *
 * The same syntax may participate in:
 *
 *     classical computing
 *     quantum/classical hybrid programs
 *     HDL
 *     accelerator programming
 *     AI/data processing
 *     distributed computation
 *     networking/data manipulation
 *     security/cryptographic computation
 *
 * Domain interpretation is downstream from parsing.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates no IR.
 *
 * The frontend AST and semantic model provide the bridge:
 *
 *     source
 *       |
 *       v
 *     parser
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic analysis
 *       |
 *       +----------------------+-----------------------+
 *       |                      |                       |
 *       v                      v                       v
 *   classical            quantum::ir             HDL/hardware
 *       |                      |                       |
 *       +----------------------+-----------------------+
 *                              |
 *                              v
 *                    optimization / lowering
 *
 * The appropriate downstream IR is selected from semantic information.
 *
 * ============================================================================
 * COMPILER INTEGRATION
 * ============================================================================
 *
 * The compiler must treat these operators as source-level operations.
 *
 * Target instruction selection belongs after:
 *
 *     parsing
 *     AST construction
 *     semantic analysis
 *     type resolution
 *     capability/resource analysis
 *     canonical IR construction
 *
 * The grammar MUST NOT select:
 *
 *     CPU instruction
 *     GPU instruction
 *     FPGA primitive
 *     ASIC gate
 *     QPU instruction
 *     vendor intrinsic
 *
 * ============================================================================
 * OPTIMIZATION CONTRACT
 * ============================================================================
 *
 * Optimizations are downstream.
 *
 * Examples include:
 *
 *     constant folding
 *     algebraic simplification
 *     vectorization
 *     masking optimization
 *     hardware lowering
 *     instruction selection
 *
 * None of these transformations belongs in this grammar.
 *
 * Optimizers must preserve the language's semantic rules, including user
 * defined operations and observable effects.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * The grammar performs no runtime evaluation.
 *
 * It must not:
 *
 *     - inspect runtime values
 *     - inspect hardware
 *     - allocate memory
 *     - perform I/O
 *     - invoke devices
 *     - access QPUs
 *     - access GPUs
 *     - access FPGAs
 *     - schedule work
 *
 * ============================================================================
 * ERROR CONTRACT
 * ============================================================================
 *
 * The parser must reject incomplete or malformed bitwise syntax.
 *
 * Examples:
 *
 *     a &
 *     a ^
 *     a |
 *
 *     & b
 *     ^ b
 *     | b
 *
 *     a & & b
 *     a ^ ^ b
 *     a | | b
 *
 * These are syntax errors.
 *
 * A syntactically valid expression whose operands are semantically
 * incompatible is NOT a grammar error.
 *
 * Example:
 *
 *     value_of_type_A & value_of_type_B
 *
 * is accepted syntactically and checked by semantic analysis.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Given the same token stream and grammar version, parsing must produce the
 * same structural result.
 *
 * Parsing MUST NOT depend on:
 *
 *     - time
 *     - randomness
 *     - environment variables
 *     - filesystem state
 *     - network state
 *     - hardware discovery
 *     - resource availability
 *     - runtime scheduler state
 *     - calibration
 *     - backend selection
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * This grammar performs no:
 *
 *     - filesystem access
 *     - network access
 *     - command execution
 *     - dynamic code execution
 *     - hardware access
 *     - runtime evaluation
 *
 * It contains no embedded target-language actions.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing canonical operator spellings remain:
 *
 *     &
 *     ^
 *     |
 *
 * represented by:
 *
 *     AMPERSAND
 *     CARET
 *     PIPE
 *
 * No alternate operator spelling is introduced here.
 *
 * New operator spellings require coordinated changes to:
 *
 *     lexer specification
 *     lexer grammar
 *     syntax specification
 *     parser grammar
 *     AST contract
 *     semantic contract
 *     compatibility tests
 *
 * ============================================================================
 * DUPLICATION PROHIBITION
 * ============================================================================
 *
 * These rules must have one canonical owner:
 *
 *     bitwiseExpression
 *     bitwiseOrExpression
 *     bitwiseXorExpression
 *     bitwiseAndExpression
 *
 * They MUST NOT also be independently defined in:
 *
 *     grammar/expressions/expression.g4
 *     grammar/expressions/expressions.g4
 *     grammar/expressions/binary.g4
 *     grammar/antlr/Core.g4
 *     grammar/antlr/ZamaniParser.g4
 *     grammar/Zamani.g4
 *
 * The root/composition grammars should import or compose this module.
 *
 * Existing duplicate definitions are integration debt and must be removed
 * from the authoritative composition path when this module is activated.
 *
 * ============================================================================
 * EXPRESSION COMPOSITION CONTRACT
 * ============================================================================
 *
 * The intended complete hierarchy is:
 *
 *     expression
 *         |
 *     assignmentExpression
 *         |
 *     conditionalExpression
 *         |
 *     rangeExpression
 *         |
 *     logicalOrExpression
 *         |
 *     logicalAndExpression
 *         |
 *     bitwiseOrExpression       <-- this file
 *         |
 *     bitwiseXorExpression      <-- this file
 *         |
 *     bitwiseAndExpression      <-- this file
 *         |
 *     equalityExpression
 *         |
 *     comparisonExpression
 *         |
 *     shiftExpression
 *         |
 *     additiveExpression
 *         |
 *     multiplicativeExpression
 *         |
 *     prefixExpression
 *         |
 *     postfixExpression
 *         |
 *     primaryExpression
 *
 * The direction is from lower binding strength at the top to higher binding
 * strength toward the bottom.
 *
 * ============================================================================
 * INTEGRATION WITH CURRENT REPOSITORY
 * ============================================================================
 *
 * The repository currently has bitwise rules duplicated in the broader
 * expression grammar. The modular architecture should make this file their
 * sole owner.
 *
 * Required integration:
 *
 *     1. `grammar/expressions/expressions.g4`
 *        must stop defining:
 *
 *            bitwiseOrExpression
 *            bitwiseXorExpression
 *            bitwiseAndExpression
 *
 *        and instead compose/import this module.
 *
 *     2. The canonical expression composition must continue to expose:
 *
 *            logicalAndExpression
 *            bitwiseOrExpression
 *
 *        at the logical/bitwise boundary.
 *
 *     3. `grammar/expressions/comparison.g4`
 *        remains the owner of:
 *
 *            equalityExpression
 *            comparisonExpression
 *
 *        and therefore supplies the lower dependency:
 *
 *            equalityExpression
 *
 *     4. `grammar/expressions/arithmetic.g4`
 *        and lower expression modules must remain below the comparison layer.
 *
 *     5. `grammar/antlr/Core.g4`
 *        must not remain a second authoritative owner of this bitwise
 *        hierarchy.
 *
 *     6. `grammar/antlr/ZamaniParser.g4`
 *        must not independently redefine these rules if it is retained as
 *        a compatibility/composition grammar.
 *
 *     7. `grammar/Zamani.g4`
 *        must consume the canonical expression composition rather than
 *        introduce another bitwise hierarchy.
 *
 * ============================================================================
 * FEATURE TRACEABILITY
 * ============================================================================
 *
 * This feature has the following conceptual traceability:
 *
 *     lexer token
 *         ->
 *     parser rule
 *         ->
 *     frontend AST operation
 *         ->
 *     semantic operator resolution
 *         ->
 *     canonical semantic representation / IR
 *         ->
 *     compiler lowering
 *         ->
 *     target realization
 *
 * Every implementation stage must preserve source provenance/source spans
 * where the repository's frontend contract requires them.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive syntax tests MUST include:
 *
 *     a & b
 *     a ^ b
 *     a | b
 *
 *     a & b & c
 *     a ^ b ^ c
 *     a | b | c
 *
 *     a | b ^ c
 *     a ^ b & c
 *     a | b ^ c & d
 *
 *     (a | b) & c
 *     a | (b & c)
 *     (a ^ b) | (c & d)
 *
 *     foo() & bar()
 *     value[index] ^ mask
 *     object.field | flags
 *
 *     nested expression operands
 *     literal operands accepted by lower expression layers
 *
 * Precedence tests MUST establish:
 *
 *     a | b ^ c
 *
 * as:
 *
 *     a | (b ^ c)
 *
 *     a ^ b & c
 *
 * as:
 *
 *     a ^ (b & c)
 *
 *     a | b ^ c & d
 *
 * as:
 *
 *     a | (b ^ (c & d))
 *
 * Associativity tests MUST establish:
 *
 *     a & b & c
 *         ==
 *     (a & b) & c
 *
 *     a ^ b ^ c
 *         ==
 *     (a ^ b) ^ c
 *
 *     a | b | c
 *         ==
 *     (a | b) | c
 *
 * Negative tests MUST include:
 *
 *     a &
 *     a ^
 *     a |
 *     & a
 *     ^ a
 *     | a
 *     a & & b
 *     a ^ ^ b
 *     a | | b
 *
 * Boundary tests MUST include:
 *
 *     long bitwise chains
 *     deeply nested expressions
 *     large source expressions
 *     large operands accepted by lower layers
 *
 * Scalability tests MUST NOT establish an artificial maximum operator count,
 * operand width, vector width, tensor dimension, or source size.
 *
 * Determinism tests MUST parse identical token streams repeatedly and verify
 * equivalent parse structure.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden in this file:
 *
 *     MAX_BITS
 *     MAX_WIDTH
 *     MAX_VECTOR_WIDTH
 *     MAX_SIGNAL_WIDTH
 *     MAX_REGISTER_WIDTH
 *     MAX_OPERATORS
 *     MAX_EXPRESSION_DEPTH
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_MEMORY
 *
 * No hardware/resource quantity is used to determine syntactic validity.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [ ] canonical lexer tokens are used;
 *     [ ] no lexer rules are duplicated;
 *     [ ] bitwise AND/XOR/OR precedence is unambiguous;
 *     [ ] each bitwise level is left associative;
 *     [ ] equalityExpression is consumed from comparison.g4;
 *     [ ] no lower expression layer is duplicated;
 *     [ ] no higher expression layer is duplicated;
 *     [ ] no circular grammar dependency exists;
 *     [ ] no target-language action exists;
 *     [ ] no unsafe Rust is involved;
 *     [ ] AST preservation requirements are documented;
 *     [ ] semantic responsibilities are documented;
 *     [ ] IR responsibilities are documented;
 *     [ ] quantum::ir remains the quantum semantic boundary;
 *     [ ] HDL remains downstream semantic responsibility;
 *     [ ] resource/hardware limits are not encoded;
 *     [ ] POCO-REAF remains target independent;
 *     [ ] positive tests exist;
 *     [ ] negative tests exist;
 *     [ ] boundary tests exist;
 *     [ ] scalability tests exist;
 *     [ ] determinism tests exist;
 *     [ ] compatibility tests exist;
 *     [ ] duplicate authoritative definitions are removed from composition;
 *     [ ] the canonical expression composition imports this module.
 *
 * ============================================================================
 */

parser grammar ZamaniBitwise;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * PUBLIC BITWISE ENTRY
 * ============================================================================
 *
 * This entry point represents the complete bitwise precedence layer.
 *
 * It begins at the highest-precedence bitwise operation and returns the
 * complete bitwise expression.
 */
bitwiseExpression
    : bitwiseOrExpression
    ;


/*
 * ============================================================================
 * BITWISE OR
 * ============================================================================
 *
 * Lowest-precedence bitwise operator.
 *
 * Example:
 *
 *     a | b ^ c & d
 *
 * parses through:
 *
 *     a | (b ^ (c & d))
 */
bitwiseOrExpression
    : bitwiseXorExpression
      (
          PIPE
          bitwiseXorExpression
      )*
    ;


/*
 * ============================================================================
 * BITWISE XOR
 * ============================================================================
 *
 * Higher precedence than bitwise OR and lower precedence than bitwise AND.
 */
bitwiseXorExpression
    : bitwiseAndExpression
      (
          CARET
          bitwiseAndExpression
      )*
    ;


/*
 * ============================================================================
 * BITWISE AND
 * ============================================================================
 *
 * Highest-precedence binary bitwise operator.
 *
 * Its operands begin at equalityExpression because comparison/equality
 * operators bind more tightly than the bitwise operators.
 */
bitwiseAndExpression
    : equalityExpression
      (
          AMPERSAND
          equalityExpression
      )*
    ;
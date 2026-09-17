/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/expressions/literals.g4
 *
 * Status:
 *     Canonical modular expression parser grammar.
 *
 * Purpose:
 *     Defines the expression-level composition of Zamani literals.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     source
 *       |
 *       v
 *     canonical lexer
 *       |
 *       v
 *     literal tokens
 *       |
 *       v
 *     this parser grammar
 *       |
 *       v
 *     domain-neutral frontend AST
 *       |
 *       v
 *     structural / semantic analysis
 *       |
 *       v
 *     canonical semantic model
 *       |
 *       +----------------------+----------------------+
 *       |                      |                      |
 *       v                      v                      v
 *   Classical IR          quantum::ir          HDL/Hardware IR
 *       |                      |                      |
 *       +----------------------+----------------------+
 *                              |
 *                              v
 *                   optimization / lowering
 *                              |
 *                   routing / scheduling
 *                              |
 *                    resilience / QEC / ZQN
 *                              |
 *                             HAL
 *                              |
 *                       target realization
 *
 * IMPORTANT:
 *
 * This file owns SOURCE-LEVEL EXPRESSION COMPOSITION ONLY.
 *
 * It does not own:
 *
 *   - lexical token definitions;
 *   - Unicode policy;
 *   - identifier syntax;
 *   - numeric lexical syntax;
 *   - string escape syntax;
 *   - character escape syntax;
 *   - semantic types;
 *   - constant evaluation;
 *   - numeric conversion;
 *   - overflow policy;
 *   - runtime representation;
 *   - resource allocation;
 *   - hardware selection;
 *   - physical qubit allocation;
 *   - QEC;
 *   - ZQN;
 *   - routing;
 *   - scheduling;
 *   - optimization;
 *   - canonical quantum::ir.
 *
 * ============================================================================
 * POCO-REAF / SCALABILITY
 * ============================================================================
 *
 * Literal syntax MUST NOT establish universal machine-dependent limits.
 *
 * This grammar therefore contains no:
 *
 *     MAX_INTEGER_BITS
 *     MAX_DECIMAL_DIGITS
 *     MAX_FLOAT_DIGITS
 *     MAX_STRING_LENGTH
 *     MAX_ARRAY_LENGTH
 *     MAX_QUANTUM_STATE_SIZE
 *     MAX_QUBITS
 *     MAX_RESOURCE_COUNT
 *     MAX_MEMORY
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *     MAX_DEVICE_COUNT
 *
 * A source program may contain values of any magnitude that are expressible
 * by the canonical lexical layer and representable by the implementation's
 * configured parsing/resource policy.
 *
 * Any implementation/resource limit is NOT a language-level literal limit.
 *
 * ============================================================================
 * OWNERSHIP CONTRACT
 * ============================================================================
 *
 * OWNS:
 *
 *   - literalExpression;
 *   - the expression-level classification of canonical literal tokens;
 *   - the stable parser boundary consumed by expression.g4;
 *   - preservation of literal-category distinctions for AST lowering.
 *
 * DOES NOT OWN:
 *
 *   - literal token spelling;
 *   - token definitions;
 *   - lexical validation;
 *   - literal decoding;
 *   - numeric representation;
 *   - arbitrary-precision implementation;
 *   - semantic typing;
 *   - constant folding;
 *   - collection literals;
 *   - tuple literals;
 *   - array literals;
 *   - map literals;
 *   - range expressions;
 *   - indexing;
 *   - member access;
 *   - calls;
 *   - unary operators;
 *   - binary operators;
 *   - assignment;
 *   - domain-specific semantic IR.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The canonical lexer owns literal tokenization.
 *
 * The parser consumes the canonical parser-facing lexer vocabulary.
 *
 * The repository's modular lexer architecture identifies:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * as the canonical lexer assembly boundary.
 *
 * Literal families are separately owned under:
 *
 *     grammar/lexer/
 *
 * including:
 *
 *     numeric-literals.g4
 *     string-literals.g4
 *     character-literals.g4
 *     boolean-literals.g4
 *     quantum-literals.g4
 *     hardware-literals.g4
 *     duration-literals.g4
 *     size-literals.g4
 *
 * This parser file MUST NOT redefine any of those lexical rules.
 *
 * ============================================================================
 * TOKEN VOCABULARY
 * ============================================================================
 *
 * The expression grammar consumes the canonical literal token categories:
 *
 *     INTEGER_LITERAL
 *     DECIMAL_LITERAL
 *     FLOAT_LITERAL
 *     STRING_LITERAL
 *     CHAR_LITERAL
 *     BOOLEAN_LITERAL
 *     NIL_LITERAL
 *     QUANTUM_LITERAL
 *     COMPLEX_LITERAL
 *     DURATION_LITERAL
 *     SIZE_LITERAL
 *     HARDWARE_LITERAL
 *
 * Boolean and null/nil semantics remain downstream semantic concerns.
 *
 * This grammar does not introduce alternate token names such as:
 *
 *     TRUE
 *     FALSE
 *     NULL
 *     INTEGER
 *     FLOAT
 *     STRING
 *     CHARACTER
 *     COMPLEX
 *
 * merely as aliases.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser must preserve the literal category so the domain-neutral AST
 * can distinguish source forms without prematurely selecting a runtime type.
 *
 * Conceptually:
 *
 *     LiteralExpression
 *         |
 *         +-- IntegerLiteral
 *         +-- DecimalLiteral
 *         +-- FloatLiteral
 *         +-- StringLiteral
 *         +-- CharacterLiteral
 *         +-- BooleanLiteral
 *         +-- NilLiteral
 *         +-- QuantumLiteral
 *         +-- ComplexLiteral
 *         +-- DurationLiteral
 *         +-- SizeLiteral
 *         +-- HardwareLiteral
 *
 * The exact AST types are owned by the existing frontend AST implementation.
 *
 * This grammar MUST NOT introduce:
 *
 *     QuantumGateLiteral
 *     PhysicalQubitLiteral
 *     CpuLiteral
 *     GpuLiteral
 *     FpgaLiteral
 *     TensorLiteral<N>
 *     HardwareLiteral<fixed-device>
 *
 * or other target/domain-specific AST variants merely because a literal is
 * consumed by a particular domain.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Literal parsing establishes syntactic category only.
 *
 * Semantic analysis determines:
 *
 *   - literal type;
 *   - numeric representation;
 *   - signedness;
 *   - precision;
 *   - scale;
 *   - unit meaning;
 *   - validity;
 *   - conversions;
 *   - constant evaluation;
 *   - resource implications;
 *   - domain interpretation.
 *
 * For example:
 *
 *     42
 *
 * is not inherently:
 *
 *     i32
 *     i64
 *     u32
 *     u64
 *     usize
 *     native integer
 *
 * Likewise:
 *
 *     1.0
 *
 * is not inherently:
 *
 *     f32
 *     f64
 *     IEEE-754
 *
 * unless a later type/semantic rule establishes that interpretation.
 *
 * ============================================================================
 * NUMERIC INTEGRATION
 * ============================================================================
 *
 * Numeric lexical ownership belongs to grammar/lexer/numeric-literals.g4.
 *
 * The parser therefore consumes:
 *
 *     INTEGER_LITERAL
 *     DECIMAL_LITERAL
 *     FLOAT_LITERAL
 *     COMPLEX_LITERAL
 *
 * without reimplementing their lexical structure.
 *
 * Numeric signs remain expression operators.
 *
 * For example:
 *
 *     -42
 *
 * is structurally:
 *
 *     unary operator
 *         +
 *     integer literal
 *
 * rather than a parser-level signed-literal production.
 *
 * This preserves the existing separation between numeric literals and unary
 * expressions.
 *
 * ============================================================================
 * STRING / CHARACTER INTEGRATION
 * ============================================================================
 *
 * String and character lexical syntax is owned by the lexer.
 *
 * This grammar merely exposes those tokens as literal expressions.
 *
 * It does not:
 *
 *   - decode escapes;
 *   - normalize Unicode;
 *   - impose an encoding;
 *   - impose a maximum string length;
 *   - select an in-memory representation.
 *
 * ============================================================================
 * BOOLEAN / NIL INTEGRATION
 * ============================================================================
 *
 * Boolean syntax is represented by BOOLEAN_LITERAL.
 *
 * Nil syntax is represented by NIL_LITERAL.
 *
 * This avoids making parser syntax responsible for whether:
 *
 *     nil
 *     null
 *
 * represents:
 *
 *   - an optional value;
 *   - a nullable reference;
 *   - an absence value;
 *   - another semantic construct.
 *
 * Those distinctions belong to the type and semantic systems.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * QUANTUM_LITERAL is a SOURCE-LEVEL quantum literal.
 *
 * Parsing it does not:
 *
 *   - allocate a physical qubit;
 *   - choose a QPU;
 *   - choose a simulator;
 *   - select a gate set;
 *   - choose topology;
 *   - choose calibration;
 *   - select a device;
 *   - impose a qubit limit.
 *
 * Quantum semantics continue through:
 *
 *     frontend AST
 *          |
 *          v
 *     semantic quantum model
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     routing / scheduling
 *          |
 *          v
 *     QEC / resilience / ZQN
 *          |
 *          v
 *     HAL
 *          |
 *          v
 *     target
 *
 * This file therefore does not create a second quantum IR.
 *
 * ============================================================================
 * COMPLEX NUMBER INTEGRATION
 * ============================================================================
 *
 * COMPLEX_LITERAL represents the source-level complex-number lexical form
 * supplied by the canonical lexer.
 *
 * This grammar does not decide:
 *
 *     complex64
 *     complex128
 *     arbitrary precision complex
 *     symbolic complex
 *
 * Such decisions belong to semantic/type analysis.
 *
 * ============================================================================
 * DURATION INTEGRATION
 * ============================================================================
 *
 * DURATION_LITERAL represents a source-level duration quantity.
 *
 * It does not select:
 *
 *     CPU cycles
 *     quantum clock cycles
 *     scheduler slots
 *     hardware timers
 *
 * Conversion to a target timing model is downstream.
 *
 * ============================================================================
 * SIZE / RESOURCE INTEGRATION
 * ============================================================================
 *
 * SIZE_LITERAL represents a source-level size/quantity literal.
 *
 * It must not silently become:
 *
 *     host pointer size
 *     RAM capacity
 *     VRAM capacity
 *     register width
 *     physical storage capacity
 *
 * Resource analysis and target realization determine those properties.
 *
 * ============================================================================
 * HARDWARE INTEGRATION
 * ============================================================================
 *
 * HARDWARE_LITERAL represents source-level hardware/resource-oriented literal
 * syntax where the lexical layer explicitly provides such a token.
 *
 * It does NOT by itself select a physical device.
 *
 * Hardware selection, capability matching, placement, topology, calibration,
 * and deployment remain downstream responsibilities.
 *
 * ============================================================================
 * COLLECTION / COMPOSITE LITERALS
 * ============================================================================
 *
 * This file intentionally does NOT define:
 *
 *     array literals
 *     tuple literals
 *     map literals
 *     record literals
 *     tensor literals
 *
 * Those constructs may contain literalExpression, but their delimiters and
 * recursive structure belong to their respective grammar files.
 *
 * This prevents duplicate ownership and recursive grammar cycles.
 *
 * ============================================================================
 * RANGE / INDEX / MEMBER / CALL INTEGRATION
 * ============================================================================
 *
 * This file does NOT define:
 *
 *     range expressions
 *     indexing
 *     member access
 *     calls
 *     postfix expressions
 *
 * A literal can be the base of those expressions through the canonical
 * expression hierarchy.
 *
 * Examples conceptually include:
 *
 *     42
 *     "Zamani"
 *     value.member
 *     values[42]
 *     f(42)
 *     0 .. n
 *
 * but the operators and delimiters belong to their respective grammar
 * components.
 *
 * ============================================================================
 * DOMAIN NEUTRALITY
 * ============================================================================
 *
 * Literals are universal expression primitives.
 *
 * They may eventually participate in:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     hardware/software co-design
 *     distributed
 *     parallel/HPC
 *     AI/ML
 *     tensor/data
 *     networking
 *     cryptography
 *     scientific computing
 *     embedded systems
 *     accelerators
 *     future computational domains
 *
 * This file does not create one literal grammar per domain.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * The same source-level literal must retain its language-defined meaning when
 * compiled for different targets.
 *
 * Target-specific realization is allowed to differ.
 *
 * For example:
 *
 *     42
 *
 * may ultimately be represented differently on different targets while
 * retaining the same language-level value.
 *
 * Similarly:
 *
 *     1GiB
 *
 * describes a source-level quantity. It does not require every target to have
 * 1 GiB available.
 *
 * A resource requirement, if one exists, must be represented by the resource
 * and capability systems rather than hidden inside this literal grammar.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Literal parsing must be deterministic.
 *
 * Given the same:
 *
 *     token stream
 *     grammar version
 *     parser configuration
 *
 * the same parse structure must result.
 *
 * Parsing must not depend on:
 *
 *     CPU count
 *     GPU availability
 *     QPU availability
 *     memory capacity
 *     network state
 *     hardware topology
 *     runtime state
 *     random state
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Lexical errors belong to the lexer.
 *
 * Syntax errors belong to the parser.
 *
 * Semantic errors belong to semantic/type analysis.
 *
 * Examples of semantic errors include:
 *
 *     numeric value outside requested type
 *     invalid duration context
 *     unsupported hardware quantity
 *     invalid quantum-state semantics
 *     incompatible literal conversion
 *
 * This grammar must not attempt to perform semantic validation.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing canonical literal token meanings must remain stable.
 *
 * Future literal categories require:
 *
 *     - specification update;
 *     - lexer-token ownership;
 *     - parser integration;
 *     - AST contract;
 *     - semantic contract;
 *     - compatibility analysis;
 *     - positive tests;
 *     - negative tests;
 *     - boundary tests;
 *     - scalability tests.
 *
 * Adding a new domain must not require modifying this grammar merely because
 * that domain needs a new semantic value representation unless the value has
 * genuinely become a language-level literal category.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * This file is complete only when the integration test suite covers at least:
 *
 * Positive:
 *
 *     integer literals
 *     decimal literals
 *     floating literals
 *     string literals
 *     character literals
 *     boolean literals
 *     nil literals
 *     quantum literals
 *     complex literals
 *     duration literals
 *     size literals
 *     hardware literals
 *
 * Negative:
 *
 *     malformed token streams
 *     incomplete literals
 *     lexer/parser boundary failures
 *     literals incorrectly consumed as identifiers/operators
 *
 * Boundary:
 *
 *     zero
 *     very large integer source values
 *     very small/large decimal source values
 *     long strings where supported
 *     Unicode characters where supported
 *     quantum literal boundaries
 *     duration boundaries
 *     size boundaries
 *
 * Scalability:
 *
 *     arbitrarily large source-level numeric magnitude subject to lexer/parser
 *     operational budgets;
 *
 *     arbitrarily long literal-containing programs subject to implementation
 *     resource budgets;
 *
 *     no test may establish a universal maximum literal size as language
 *     semantics.
 *
 * Determinism:
 *
 *     identical token streams must produce identical parse trees.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This file contains no Rust code.
 *
 * The generated/compiler implementation must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *
 * and the repository's safe-Rust requirement.
 *
 * No unsafe Rust is required or permitted by this grammar.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * Upstream:
 *
 *     grammar/specification/lexical.md
 *     grammar/spec/syntax.md
 *     grammar/lexer/*.g4
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Downstream:
 *
 *     grammar/expressions/expression.g4
 *     grammar/expressions/primary.g4, if present
 *     frontend AST
 *     semantic analysis
 *     type inference
 *     canonical semantic model
 *     canonical/domain IR
 *
 * Cross-domain:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     hardware
 *     resources
 *     distributed
 *     AI/data
 *     networking
 *     security
 *
 * No downstream component should need to modify this file merely because it
 * changes the target hardware or runtime.
 *
 * ============================================================================
 * ANTLR4 PARSER GRAMMAR
 * ============================================================================
 */

parser grammar Literals;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * This is the ONLY expression-level literal dispatcher owned by this file.
 *
 * Other expression grammars should consume:
 *
 *     literalExpression
 *
 * rather than reproducing this token list.
 */
literalExpression
    : INTEGER_LITERAL
    | DECIMAL_LITERAL
    | FLOAT_LITERAL
    | STRING_LITERAL
    | CHAR_LITERAL
    | BOOLEAN_LITERAL
    | NIL_LITERAL
    | QUANTUM_LITERAL
    | COMPLEX_LITERAL
    | DURATION_LITERAL
    | SIZE_LITERAL
    | HARDWARE_LITERAL
    ;
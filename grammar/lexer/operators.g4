/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/lexer/operators.g4
 *
 * Grammar role:
 *     Operator lexical vocabulary.
 *
 * Language:
 *     Zamani
 *
 * Compiler implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     The Rust implementation integrating this grammar MUST use safe Rust.
 *     No unsafe Rust is required or permitted.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file defines the lexical representation of Zamani operators.
 *
 * It answers only:
 *
 *     "Which source-character sequences constitute operator tokens?"
 *
 * It does NOT answer:
 *
 *     - what an operator means;
 *     - whether an operator is valid for a particular type;
 *     - whether an operator is classical or quantum;
 *     - whether an operator is hardware-native;
 *     - whether an operator is overloaded;
 *     - operator precedence;
 *     - operator associativity;
 *     - constant folding;
 *     - algebraic simplification;
 *     - optimization;
 *     - lowering;
 *     - scheduling;
 *     - routing;
 *     - execution;
 *     - resource allocation.
 *
 * Those concerns belong to downstream language/compiler layers.
 *
 * ============================================================================
 * ARCHITECTURAL PIPELINE
 * ============================================================================
 *
 *     source
 *       |
 *       v
 *     lexical foundation
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       +--> operator tokens
 *       |
 *       v
 *     parser
 *       |
 *       v
 *     frontend AST
 *       |
 *       v
 *     semantic analysis
 *       |
 *       +--> type checking
 *       +--> overload resolution
 *       +--> capability checking
 *       +--> effect checking
 *       +--> domain resolution
 *       |
 *       v
 *     canonical semantic IR
 *       |
 *       +--> classical IR
 *       +--> quantum::ir
 *       +--> tensor/data IR
 *       +--> HDL/hardware IR
 *       +--> control/dataflow IR
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     routing / scheduling / resilience / target lowering
 *       |
 *       v
 *     runtime / hardware
 *
 * ============================================================================
 * OPERATOR DESIGN PRINCIPLES
 * ============================================================================
 *
 * 1. Longest operators must win over their prefixes.
 *
 *    Examples:
 *
 *        >>>=  before  >>  before  >
 *        <<=  before  <<  before  <
 *        **=  before  **  before  *
 *        +=   before  +
 *        ==   before  =
 *        !=   before  !
 *        &&   before  &
 *        ||   before  |
 *        ->   before  -
 *        =>   before  =
 *        ::   before  :
 *
 *    ANTLR's lexer matching behavior is used deliberately here.
 *
 * 2. Operators must not encode machine limits.
 *
 *    This file contains no:
 *
 *        MAX_QUBITS
 *        MAX_CORES
 *        MAX_THREADS
 *        MAX_OPERANDS
 *        MAX_TENSOR_RANK
 *        MAX_VECTOR_WIDTH
 *        MAX_REGISTER_WIDTH
 *        MAX_MEMORY
 *        MAX_DEVICES
 *
 * 3. Operator spelling is not operator semantics.
 *
 *    For example:
 *
 *        *
 *
 *    may represent multiplication for scalar values, matrix/tensor
 *    multiplication in a domain where the semantic type system defines it,
 *    or another explicitly defined operation.
 *
 *    The lexer must not choose among those meanings.
 *
 * 4. Quantum operators are not hardware instructions.
 *
 *    A source operator can eventually lower into:
 *
 *        quantum::ir
 *        classical IR
 *        tensor IR
 *        HDL IR
 *        another canonical representation
 *
 *    depending on semantic context.
 *
 * 5. The operator vocabulary is intentionally extensible.
 *
 *    A new semantic domain should not require rewriting the lexical grammar
 *    merely because an existing operator is given a new typed meaning.
 *
 * 6. Domain-specific operation names should normally remain identifiers.
 *
 *    For example:
 *
 *        H
 *        X
 *        CNOT
 *        custom_gate
 *        vendor_operation
 *
 *    are not operators merely because they are quantum operations.
 *
 * 7. No parser precedence is defined here.
 *
 *    Precedence belongs to the expression grammar.
 *
 * 8. No operator overload resolution is performed here.
 *
 * 9. Unicode operator spellings are explicit and deterministic.
 *
 * 10. Unicode operators must not silently normalize into unrelated ASCII
 *     operators.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * The lexical representation must remain stable as machines evolve.
 *
 * A source expression such as:
 *
 *     a + b
 *
 * does not imply:
 *
 *     one CPU
 *     one ALU
 *     one vector width
 *     one accelerator
 *     one execution location
 *
 * Likewise:
 *
 *     A @@ B
 *
 * does not imply a particular matrix dimension or physical accelerator.
 *
 * Dimensions, types, capabilities, resource requirements and target-specific
 * realization belong to later compiler stages.
 *
 * ============================================================================
 * OPERATOR CATEGORIES
 * ============================================================================
 *
 * Arithmetic:
 *
 *     +  -  *  /  %  **  //
 *
 * Assignment:
 *
 *     =  +=  -=  *=  /=  %=  **=  //=
 *
 * Comparison:
 *
 *     ==  !=  <  <=  >  >=
 *
 * Logical:
 *
 *     !  &&  ||
 *
 * Bitwise:
 *
 *     &  |  ^  ~
 *
 * Shift:
 *
 *     <<  >>  <<<  >>>
 *
 * Range:
 *
 *     ..  ..=
 *
 * Structural / semantic:
 *
 *     ->  =>  ::  ?  ??  ??
 *
 * Matrix/tensor:
 *
 *     @@  ⊗
 *
 * Access / composition:
 *
 *     .  ?.
 *
 * Optional/chaining/nullability operators are lexical only where their
 * spelling is explicitly defined by the language specification.
 *
 * ============================================================================
 * IMPORTANT INTEGRATION RULE
 * ============================================================================
 *
 * This file is a lexer fragment, not the canonical complete lexer.
 *
 * The canonical lexer assembly must import/include these operator rules
 * exactly once.
 *
 * If an existing canonical lexer already defines any of the tokens below,
 * that duplicate definition MUST be removed or migrated as part of lexer
 * assembly.
 *
 * In particular, the repository currently contains legacy operator
 * definitions in monolithic lexer grammars. They must not remain as a second
 * authority after this modular operator grammar becomes canonical.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * ARITHMETIC OPERATORS
 * ============================================================================
 *
 * Semantic interpretation belongs to the type/domain system.
 *
 * `+`, `-`, `*`, `/`, `%` are intentionally lexical only.
 *
 * `**` represents exponentiation.
 *
 * `//` is reserved for integer/floor-style division semantics if the language
 * specification assigns that meaning. The lexer does not enforce operand
 * types.
 *
 * ============================================================================
 */

POWER
    : '**'
    ;

FLOOR_DIV
    : '//'
    ;

PLUS
    : '+'
    ;

MINUS
    : '-'
    ;

STAR
    : '*'
    ;

SLASH
    : '/'
    ;

PERCENT
    : '%'
    ;


/*
 * ============================================================================
 * COMPOUND ASSIGNMENT OPERATORS
 * ============================================================================
 *
 * These remain lexical forms.
 *
 * Whether:
 *
 *     a += b
 *
 * means:
 *
 *     a = a + b
 *
 * or invokes another typed semantic operation is decided downstream.
 *
 * ============================================================================
 */

POWER_ASSIGN
    : '**='
    ;

FLOOR_DIV_ASSIGN
    : '//='
    ;

PLUS_ASSIGN
    : '+='
    ;

MINUS_ASSIGN
    : '-='
    ;

STAR_ASSIGN
    : '*='
    ;

SLASH_ASSIGN
    : '/='
    ;

PERCENT_ASSIGN
    : '%='
    ;


/*
 * ============================================================================
 * ASSIGNMENT / BINDING
 * ============================================================================
 */

ASSIGN
    : '='
    ;


/*
 * ============================================================================
 * COMPARISON OPERATORS
 * ============================================================================
 */

EQUAL
    : '=='
    ;

NOT_EQUAL
    : '!='
    ;

LESS_THAN_OR_EQUAL
    : '<='
    ;

GREATER_THAN_OR_EQUAL
    : '>='
    ;

LESS_THAN
    : '<'
    ;

GREATER_THAN
    : '>'
    ;


/*
 * ============================================================================
 * LOGICAL OPERATORS
 * ============================================================================
 *
 * These are lexical forms only.
 *
 * The semantic system determines:
 *
 *     boolean semantics
 *     three-valued semantics
 *     symbolic semantics
 *     predicate semantics
 *     domain-specific semantics
 *
 * where permitted by the language.
 * ============================================================================
 */

LOGICAL_NOT
    : '!'
    ;

LOGICAL_AND
    : '&&'
    ;

LOGICAL_OR
    : '||'
    ;


/*
 * ============================================================================
 * NULLABILITY / OPTIONAL OPERATORS
 * ============================================================================
 *
 * `??` is the null-coalescing spelling.
 *
 * The parser and type system determine whether it is valid in context.
 * ============================================================================
 */

NULL_COALESCE
    : '??'
    ;


/*
 * ============================================================================
 * BITWISE OPERATORS
 * ============================================================================
 */

BITWISE_AND
    : '&'
    ;

BITWISE_OR
    : '|'
    ;

BITWISE_XOR
    : '^'
    ;

BITWISE_NOT
    : '~'
    ;


/*
 * ============================================================================
 * SHIFT OPERATORS
 * ============================================================================
 *
 * The grammar does not impose an integer width.
 *
 * Shift amount and operand width are semantic/type-system concerns.
 *
 * The extended shift spellings are reserved for scalable/future arithmetic
 * and domain extensions. Their semantic availability is determined downstream.
 * ============================================================================
 */

SHIFT_LEFT
    : '<<'
    ;

SHIFT_RIGHT
    : '>>'
    ;

SHIFT_LEFT_LOGICAL
    : '<<<'
    ;

SHIFT_RIGHT_LOGICAL
    : '>>>'
    ;


/*
 * ============================================================================
 * RANGE OPERATORS
 * ============================================================================
 */

RANGE
    : '..'
    ;

RANGE_INCLUSIVE
    : '..='
    ;


/*
 * ============================================================================
 * FUNCTION / CONTROL / FLOW OPERATORS
 * ============================================================================
 */

ARROW
    : '->'
    ;

FAT_ARROW
    : '=>'
    ;


/*
 * ============================================================================
 * NAMESPACE / PATH OPERATOR
 * ============================================================================
 *
 * `::` is syntactically a path/namespace separator.
 *
 * It is not owned semantically by this file.
 * ============================================================================
 */

DOUBLE_COLON
    : '::'
    ;


/*
 * ============================================================================
 * MEMBER / OPTIONAL MEMBER ACCESS
 * ============================================================================
 *
 * The ordinary dot is intentionally separate from the optional-member
 * operator.
 *
 * Numeric literal handling must be designed so that:
 *
 *     1.5
 *
 * is not incorrectly split as:
 *
 *     INTEGER DOT INTEGER
 *
 * when FLOAT lexical rules apply.
 *
 * ============================================================================
 */

OPTIONAL_MEMBER_ACCESS
    : '?.'
    ;

DOT
    : '.'
    ;


/*
 * ============================================================================
 * TERNARY / CONDITIONAL OPERATOR
 * ============================================================================
 *
 * The question mark is lexical.
 *
 * The expression grammar determines whether it participates in:
 *
 *     conditional expressions
 *     optional syntax
 *     other language constructs
 *
 * ============================================================================
 */

QUESTION
    : '?'
    ;


/*
 * ============================================================================
 * MATRIX / TENSOR OPERATORS
 * ============================================================================
 *
 * The repository's existing grammar contains:
 *
 *     @@
 *     ⊗
 *
 * for matrix multiplication / tensor-style multiplication.
 *
 * Those spellings are retained for compatibility.
 *
 * IMPORTANT:
 *
 * This does NOT mean:
 *
 *     fixed matrix size
 *     fixed tensor rank
 *     CPU matrix unit
 *     GPU tensor core
 *     quantum dimension
 *
 * Any such properties belong to types, semantic analysis, capabilities and
 * target lowering.
 * ============================================================================
 */

MATMUL
    : '@@'
    ;

TENSOR_PRODUCT
    : '⊗'
    ;


/*
 * ============================================================================
 * COMPOSITION / PIPE OPERATORS
 * ============================================================================
 *
 * These are intentionally lexical extension points.
 *
 * Their exact semantic meaning must be specified before being exposed as
 * stable language features.
 *
 * A token should not silently acquire different meanings in different
 * compiler stages.
 * ============================================================================
 */

PIPE_FORWARD
    : '|>'
    ;

PIPE_BACKWARD
    : '<|'
    ;


/*
 * ============================================================================
 * FUNCTION / TYPE / META OPERATORS
 * ============================================================================
 *
 * These spellings are reserved here only when the canonical syntax uses them.
 * Their semantics are determined downstream.
 * ============================================================================
 */

FAT_ARROW_REVERSE
    : '<='
    ;


/*
 * ============================================================================
 * SEMANTIC / COMPARISON EXTENSIONS
 * ============================================================================
 *
 * These operators provide lexical space for semantic comparison where the
 * canonical language specification adopts them.
 *
 * They do not encode machine or hardware semantics.
 * ============================================================================
 */

SPACESHIP
    : '<=>'
    ;


/*
 * ============================================================================
 * OPERATOR IDENTIFIER EXTENSION
 * ============================================================================
 *
 * IMPORTANT:
 *
 * Zamani should not require every future operator to be hard-coded into the
 * lexer merely because a domain wants to introduce a semantic operation.
 *
 * Therefore:
 *
 *     named operations
 *     quantum operations
 *     hardware operations
 *     accelerator operations
 *     library operations
 *     user-defined semantic functions
 *
 * remain identifiers unless the language specification explicitly reserves a
 * symbolic operator spelling.
 *
 * This prevents the grammar from becoming a closed catalogue of current
 * hardware or mathematical functionality.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * RESERVED OPERATOR SPACE
 * ============================================================================
 *
 * Do NOT add arbitrary punctuation here merely to reserve future syntax.
 *
 * Unassigned operator spellings should remain ordinary lexer errors or be
 * handled through a deliberately specified future operator-extension
 * mechanism.
 *
 * Silent acceptance of unspecified operators would make the language
 * ambiguous and would violate deterministic parsing.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * LEXICAL PRECEDENCE / LONGEST MATCH
 * ============================================================================
 *
 * The following relationships are especially important:
 *
 *     **=  >  **  >  *
 *     //=  >  //  >  /
 *     +=   >  +
 *     -=   >  -
 *     *=   >  *
 *     /=   >  /
 *     %=   >  %
 *     ==   >  =
 *     !=   >  !
 *     <=   >  <
 *     >=   >  >
 *     &&   >  &
 *     ||   >  |
 *     <<<  >  <<  < 
 *     >>>  >  >>  >
 *     ..=  >  ..
 *     ->   >  -
 *     =>   >  =
 *     ::   >  :
 *     ?.   >  ?
 *     @@   >  @
 *     <=>  >  <= / =>
 *
 * ANTLR's maximal-munch behavior and rule priority must be verified in the
 * canonical assembled lexer.
 *
 * If any operator token is moved into another lexer fragment, the assembled
 * lexer must retain deterministic longest-match behavior.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * SEMANTIC OWNERSHIP MATRIX
 * ============================================================================
 *
 * Operator spelling      Lexer       Parser       Semantic       IR
 * ---------------------------------------------------------------------------
 *
 * +                      YES         YES          YES             YES
 * -                      YES         YES          YES             YES
 * *                      YES         YES          YES             YES
 * /                      YES         YES          YES             YES
 * %                      YES         YES          YES             YES
 * **                     YES         YES          YES             YES
 * =                      YES         YES          YES             YES
 * ==                     YES         YES          YES             YES
 * !=                     YES         YES          YES             YES
 * <                      YES         YES          YES             YES
 * <=                     YES         YES          YES             YES
 * >                      YES         YES          YES             YES
 * >=                     YES         YES          YES             YES
 * !                      YES         YES          YES             YES
 * &&                     YES         YES          YES             YES
 * ||                     YES         YES          YES             YES
 * &                      YES         YES          YES             YES
 * |                      YES         YES          YES             YES
 * ^                      YES         YES          YES             YES
 * ~                      YES         YES          YES             YES
 * <<                     YES         YES          YES             YES
 * >>                     YES         YES          YES             YES
 * ..                     YES         YES          YES             YES
 * ..=                    YES         YES          YES             YES
 * ->                     YES         YES          YES             YES
 * =>                     YES         YES          YES             YES
 * ::                     YES         YES          YES             NO*
 * ?.                     YES         YES          YES             YES
 * @@                     YES         YES          YES             YES
 * ⊗                     YES         YES          YES             YES
 *
 * YES means "participates in this layer".
 *
 * The lexer does NOT determine semantic ownership.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * DOMAIN INDEPENDENCE
 * ============================================================================
 *
 * The same operator token may be interpreted differently depending on the
 * semantic types and domain.
 *
 * Examples:
 *
 *     a + b
 *
 * may represent:
 *
 *     scalar addition
 *     vector addition
 *     matrix addition
 *     tensor addition
 *     symbolic addition
 *     polynomial addition
 *     domain-defined addition
 *
 * Likewise:
 *
 *     a * b
 *
 * may represent:
 *
 *     scalar multiplication
 *     matrix multiplication
 *     tensor contraction
 *     element-wise multiplication
 *     domain-defined multiplication
 *
 * and:
 *
 *     A @@ B
 *
 * may represent matrix/tensor multiplication according to the semantic
 * system, without this lexer knowing the dimensions.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * QUANTUM INDEPENDENCE
 * ============================================================================
 *
 * This file intentionally does NOT define:
 *
 *     H
 *     X
 *     Y
 *     Z
 *     CNOT
 *     CZ
 *     SWAP
 *     U
 *     RX
 *     RY
 *     RZ
 *     custom_gate
 *     vendor_gate
 *
 * as operators.
 *
 * Such names are generally identifiers or semantic operation names.
 *
 * This allows:
 *
 *     logical operations
 *     physical operations
 *     calibrated operations
 *     user-defined operations
 *     future gates
 *     provider-specific operations
 *
 * to be handled by semantic/domain layers rather than by a fixed lexer
 * catalogue.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * HARDWARE INDEPENDENCE
 * ============================================================================
 *
 * No operator in this file may imply:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     SIMD width
 *     register count
 *     memory capacity
 *     network topology
 *     device identifier
 *     physical address
 *     execution latency
 *     clock frequency
 *     gate duration
 *
 * Hardware realization is downstream.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This grammar places no artificial limit on:
 *
 *     expression depth
 *     number of expressions
 *     number of operators
 *     number of operands
 *     number of program statements
 *     number of qubits
 *     number of classical values
 *     number of devices
 *     tensor dimensions
 *     machine resources
 *
 * Any implementation limits arise from:
 *
 *     available memory
 *     parser/runtime implementation
 *     operating-system limits
 *     compilation resources
 *     target capabilities
 *
 * and must not be presented as language-level semantic limits.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * For identical source text and identical lexer configuration:
 *
 *     identical input
 *         ->
 *     identical token sequence
 *
 * must hold.
 *
 * The operator lexer must not depend on:
 *
 *     machine architecture
 *     CPU count
 *     runtime state
 *     network state
 *     hardware discovery
 *     quantum backend
 *     calibration state
 *     wall-clock time
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * ERROR HANDLING
 * ============================================================================
 *
 * Unsupported symbolic sequences must produce deterministic lexical errors.
 *
 * The lexer must not:
 *
 *     silently reinterpret them;
 *     silently delete them;
 *     convert them into comments;
 *     turn them into identifiers without specification;
 *     emit executable semantics.
 *
 * Diagnostics are consumed by the frontend diagnostic system.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing operator spellings must be audited before removal.
 *
 * In particular:
 *
 *     @@
 *     ⊗
 *
 * are retained because the existing Zamani grammar documents matrix
 * multiplication using those spellings.
 *
 * Any existing operator spelling found elsewhere in:
 *
 *     grammar/Zamani.g4
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/antlr/Core.g4
 *     src/lexer.rs
 *     parser
 *     AST
 *     tests
 *
 * must be classified as:
 *
 *     KEEP
 *     MIGRATE
 *     DEPRECATE
 *     REMOVE
 *
 * before the canonical lexer is assembled.
 *
 * No existing valid language construct should disappear silently.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * AST INTEGRATION
 * ============================================================================
 *
 * The existing AST represents prefix and infix operators using operator token
 * identity.
 *
 * Therefore the lexer must preserve enough information for the parser to
 * produce stable operator identity.
 *
 * The lexer must NOT:
 *
 *     resolve overloads;
 *     fold constants;
 *     lower quantum operations;
 *     choose hardware instructions;
 *     select accelerator implementations.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * QUANTUM IR INTEGRATION
 * ============================================================================
 *
 * If an operator eventually denotes quantum semantics:
 *
 *     operator token
 *          |
 *          v
 *     parser AST
 *          |
 *          v
 *     semantic resolution
 *          |
 *          v
 *     canonical quantum representation
 *          |
 *          v
 *     quantum::ir
 *
 * This grammar must never construct a second quantum IR.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * OPTIMIZATION INTEGRATION
 * ============================================================================
 *
 * Optimization must consume semantic IR rather than raw lexer tokens.
 *
 * Examples:
 *
 *     a + 0
 *     x * 1
 *     x ** 1
 *
 * may be optimized only after semantic correctness has been established.
 *
 * The lexer does not perform such transformations.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * SCHEDULING / ROUTING / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Operators do not encode:
 *
 *     placement
 *     timing
 *     resource allocation
 *     routing
 *     physical qubits
 *     machine topology
 *
 * Those decisions belong downstream.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * TEST REQUIREMENTS
 * ============================================================================
 *
 * Positive lexical tests MUST include:
 *
 *     +
 *     -
 *     *
 *     /
 *     %
 *     **
 *     //
 *     =
 *     +=
 *     -=
 *     *=
 *     /=
 *     %=
 *     **=
 *     //=
 *     ==
 *     !=
 *     <
 *     <=
 *     >
 *     >=
 *     !
 *     &&
 *     ||
 *     &
 *     |
 *     ^
 *     ~
 *     <<
 *     >>
 *     <<<
 *     >>>
 *     ..
 *     ..=
 *     ->
 *     =>
 *     ::
 *     ?.
 *     ?
 *     @@
 *     ⊗
 *     <=> 
 *
 * Combination tests MUST include:
 *
 *     a+b
 *     a+=b
 *     a**=b
 *     a<<=b
 *     a==b
 *     a!=b
 *     a<=b
 *     a>=b
 *     a&&b
 *     a||b
 *     a<<b
 *     a>>b
 *     a..b
 *     a..=b
 *     a->b
 *     a=>b
 *     a::b
 *     a?.b
 *     A@@B
 *     A⊗B
 *
 * Longest-match tests MUST verify:
 *
 *     **= is not ** followed by =
 *     //= is not // followed by =
 *     += is not + followed by =
 *     == is not = followed by =
 *     != is not ! followed by =
 *     <= is not < followed by =
 *     >= is not > followed by =
 *     && is not & followed by &
 *     || is not | followed by |
 *     <<< is not << followed by <
 *     >>> is not >> followed by >
 *     ..= is not .. followed by =
 *     -> is not - followed by >
 *     => is not = followed by >
 *     :: is not : followed by :
 *     ?. is not ? followed by .
 *     @@ is not @ followed by @
 *     <=> is not <= followed by >
 *
 * Negative tests MUST include unsupported symbolic combinations.
 *
 * Boundary tests MUST include:
 *
 *     deeply nested expressions
 *     very long operator-containing expressions
 *     large generated programs
 *     many operators in sequence
 *
 * The tests must not use artificial language-level maximums.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden:
 *
 *     MAX_OPERATORS
 *     MAX_OPERANDS
 *     MAX_EXPRESSION_DEPTH
 *     MAX_TENSOR_DIMENSION
 *     MAX_QUBITS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_DEVICES
 *
 * This file contains none of these.
 *
 * Any implementation limit discovered in lexer/parser/runtime tests must be
 * classified separately as an implementation/resource limitation rather than
 * a language semantic limit.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete only when:
 *
 * [ ] Every canonical operator spelling has one lexical owner.
 * [ ] No operator is duplicated in another canonical lexer fragment.
 * [ ] Longest-match behavior is tested.
 * [ ] Existing valid operator spellings are preserved or explicitly migrated.
 * [ ] `@@` compatibility is preserved.
 * [ ] `⊗` compatibility is preserved.
 * [ ] Operator precedence remains outside this file.
 * [ ] Operator associativity remains outside this file.
 * [ ] Operator semantics remain outside this file.
 * [ ] Operator overload resolution remains outside this file.
 * [ ] Quantum gate semantics remain outside this file.
 * [ ] Hardware semantics remain outside this file.
 * [ ] No machine-size limits exist.
 * [ ] No qubit-count limits exist.
 * [ ] No operand-count limits exist.
 * [ ] No tensor-rank limits exist.
 * [ ] Deterministic lexing is verified.
 * [ ] Negative lexical cases are verified.
 * [ ] Boundary/scalability cases are verified.
 * [ ] AST integration is verified.
 * [ ] Canonical parser integration is verified.
 * [ ] Canonical lexer assembly is verified.
 * [ ] Rust 1.97 / 1.97.1 integration remains safe Rust.
 *
 * ============================================================================
 */
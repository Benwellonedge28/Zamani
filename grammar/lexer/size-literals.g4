/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/lexer/size-literals.g4
 *
 * Role:
 *     Canonical ANTLR4 lexer component for source-level size/quantity
 *     literals.
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     This file contains ANTLR grammar only.
 *     No Rust code is embedded.
 *     No unsafe code is required or permitted.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns lexical syntax for source-level quantities whose semantic
 * dimension represents an amount of information, storage, capacity, width,
 * or data size.
 *
 * Examples:
 *
 *     8bit
 *     1byte
 *     1024B
 *     1KiB
 *     16MiB
 *     2GiB
 *     4TiB
 *     8PiB
 *     1KB
 *     10MB
 *     1GB
 *
 * The lexer records the source spelling.
 *
 * It does NOT decide:
 *
 *     - integer width;
 *     - pointer width;
 *     - register width;
 *     - CPU word size;
 *     - GPU word size;
 *     - FPGA resource size;
 *     - memory capacity;
 *     - device capacity;
 *     - address width;
 *     - bus width;
 *     - cache size;
 *     - quantum register size;
 *     - target architecture;
 *     - physical hardware availability.
 *
 * Those concepts belong to semantic analysis, resource analysis, hardware
 * capabilities, compilation, scheduling, deployment, or runtime layers.
 *
 * ============================================================================
 * POCO-REAF PRINCIPLE
 * ============================================================================
 *
 * A size literal describes a source-level quantity.
 *
 * It does NOT mean that the target machine possesses that quantity.
 *
 * For example:
 *
 *     1024MiB
 *
 * means that the program expresses a quantity of 1024 mebibytes.
 *
 * It does NOT mean:
 *
 *     allocate 1024MiB now
 *     the machine has 1024MiB
 *     the device has 1024MiB
 *     the accelerator has 1024MiB
 *     the target can satisfy 1024MiB
 *
 * Those decisions occur after lexical analysis.
 *
 * This separation is required for:
 *
 *     Program_Once
 *          ->
 *     Compile_Once
 *          ->
 *     Run_Everywhere
 *          ->
 *     Run_Anywhere
 *          ->
 *     Run_Forever
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - SIZE_LITERAL;
 *     - size-unit suffix recognition;
 *     - binary IEC size units;
 *     - decimal SI size units;
 *     - bit quantities;
 *     - byte quantities;
 *     - digit-separated size quantities;
 *     - size-literal lexical fragments;
 *     - lexical distinction between a size quantity and a plain number.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - generic numeric literals;
 *     - integer semantics;
 *     - floating-point semantics;
 *     - duration literals;
 *     - hardware addresses;
 *     - device identifiers;
 *     - resource allocation;
 *     - resource availability;
 *     - memory ownership;
 *     - memory allocation;
 *     - pointer semantics;
 *     - register semantics;
 *     - hardware topology;
 *     - target selection;
 *     - scheduling;
 *     - optimization;
 *     - routing;
 *     - quantum semantics;
 *     - QEC;
 *     - ZQN;
 *     - runtime execution;
 *     - canonical IR.
 *
 * ============================================================================
 * IMPORTANT INTEGRATION OWNERSHIP RULE
 * ============================================================================
 *
 * Generic source-level size syntax has exactly ONE lexical owner:
 *
 *     size-literals.g4
 *
 * Other grammar components must consume SIZE_LITERAL rather than independently
 * redefining:
 *
 *     1bit
 *     8bit
 *     1byte
 *     1B
 *     1KiB
 *     1MiB
 *     ...
 *
 * In particular, hardware-literals.g4 must not maintain a competing generic
 * byte/bit-size token.
 *
 * Hardware-specific semantics such as:
 *
 *     register width
 *     bus width
 *     address width
 *     memory capacity
 *     device width
 *
 * must be represented through hardware/resource semantics consuming the
 * canonical size quantity.
 *
 * ============================================================================
 * LEXICAL FORM
 * ============================================================================
 *
 * General form:
 *
 *     size-number + size-unit
 *
 * Examples:
 *
 *     8bit
 *     64bit
 *     1byte
 *     1024B
 *     1KiB
 *     10MB
 *
 * No whitespace is permitted inside a single size literal:
 *
 *     10MB
 *
 * is one SIZE_LITERAL.
 *
 * Whereas:
 *
 *     10 MB
 *
 * is a numeric literal followed by an identifier unless a higher parser-level
 * construct explicitly gives that sequence another meaning.
 *
 * ============================================================================
 * SIGN OWNERSHIP
 * ============================================================================
 *
 * A leading '+' or '-' is intentionally NOT part of SIZE_LITERAL.
 *
 * Therefore:
 *
 *     10MB
 *
 * is:
 *
 *     SIZE_LITERAL
 *
 * while:
 *
 *     -10MB
 *
 * is conceptually:
 *
 *     MINUS SIZE_LITERAL
 *
 * and:
 *
 *     +10MB
 *
 * is conceptually:
 *
 *     PLUS SIZE_LITERAL
 *
 * This keeps unary operators independent from literal syntax.
 *
 * ============================================================================
 * NUMERIC FORM
 * ============================================================================
 *
 * Size quantities use an intentionally conservative numeric form.
 *
 * Supported:
 *
 *     0B
 *     1B
 *     8B
 *     1024B
 *
 *     1KiB
 *     16MiB
 *     2GiB
 *
 *     1KB
 *     10MB
 *     100GB
 *
 * Decimal fractional quantities are supported where the unit semantics permit
 * them:
 *
 *     1.5KB
 *     0.5MiB
 *     2.25GB
 *
 * Scientific notation is also supported:
 *
 *     1e3B
 *     2.5e6B
 *     1e3KiB
 *
 * The lexer does not determine whether a fractional size is representable as
 * an integral number of bits or bytes.
 *
 * That is a semantic question.
 *
 * ============================================================================
 * DIGIT SEPARATORS
 * ============================================================================
 *
 * Underscores are permitted only between digits.
 *
 * Valid:
 *
 *     1_000B
 *     1_024B
 *     1_024KiB
 *     1_000.5MB
 *     1_0e3B
 *
 * Invalid:
 *
 *     _100B
 *     100_B
 *     1__000B
 *     1_._5MB
 *     1._5MB
 *     1e_3B
 *     1e3_B
 *
 * The grammar deliberately does not use a loose:
 *
 *     DIGIT (DIGIT | '_')*
 *
 * construction because that permits malformed separator placement.
 *
 * ============================================================================
 * INTEGER SIZE NUMBERS
 * ============================================================================
 *
 * Decimal integer quantities:
 *
 *     0
 *     1
 *     8
 *     1024
 *     1_024
 *
 * Hexadecimal, binary, and octal size quantities are intentionally not
 * introduced as separate size syntax.
 *
 * If required in the future, they must be introduced by a language
 * specification change with explicit ambiguity and compatibility analysis.
 *
 * This keeps:
 *
 *     0x100B
 *
 * from creating ambiguity with ordinary hexadecimal integer syntax followed
 * by an identifier.
 *
 * ============================================================================
 * FRACTIONAL SIZE NUMBERS
 * ============================================================================
 *
 * Decimal fractions are permitted:
 *
 *     0.5B
 *     1.5KB
 *     2.25MiB
 *
 * The lexer only recognizes the spelling.
 *
 * Semantic analysis determines whether the resulting quantity:
 *
 *     - is exact;
 *     - represents a fractional bit;
 *     - represents a fractional byte;
 *     - requires rounding;
 *     - is illegal for the consuming type;
 *     - must be converted to another unit.
 *
 * The lexer MUST NOT perform rounding.
 *
 * ============================================================================
 * EXPONENTS
 * ============================================================================
 *
 * Valid:
 *
 *     1e3B
 *     1E3B
 *     1e+3B
 *     1e-3B
 *     1.5e6B
 *     1e3KiB
 *
 * Invalid:
 *
 *     1eB
 *     1e+B
 *     1e-B
 *     1e_3B
 *     1e3_B
 *
 * Exponents have no grammar-level magnitude limit.
 *
 * ============================================================================
 * UNIT SYSTEMS
 * ============================================================================
 *
 * Two explicit decimal/binary families are supported.
 *
 * --------------------------------------------------------------------------
 * SI DECIMAL BYTE UNITS
 * --------------------------------------------------------------------------
 *
 *     B
 *     kB
 *     MB
 *     GB
 *     TB
 *     PB
 *     EB
 *     ZB
 *     YB
 *
 * These use decimal powers of 1000.
 *
 * --------------------------------------------------------------------------
 * IEC BINARY BYTE UNITS
 * --------------------------------------------------------------------------
 *
 *     KiB
 *     MiB
 *     GiB
 *     TiB
 *     PiB
 *     EiB
 *     ZiB
 *     YiB
 *
 * These use binary powers of 1024.
 *
 * --------------------------------------------------------------------------
 * BITS
 * --------------------------------------------------------------------------
 *
 *     bit
 *     kbit
 *     Mbit
 *     Gbit
 *     Tbit
 *     Pbit
 *     Ebit
 *     Zbit
 *     Ybit
 *
 * IEC binary bit units are:
 *
 *     Kibit
 *     Mibit
 *     Gibit
 *     Tibit
 *     Pibit
 *     Eibit
 *     Zibit
 *     Yibit
 *
 * ============================================================================
 * UNIT CASE POLICY
 * ============================================================================
 *
 * Unit spelling is intentionally case-sensitive.
 *
 * Therefore:
 *
 *     MB
 *
 * and:
 *
 *     mb
 *
 * are different source spellings.
 *
 * The language does not silently reinterpret:
 *
 *     mb
 *
 * as:
 *
 *     MB
 *
 * because case-insensitive interpretation could turn ordinary identifiers
 * into quantities unexpectedly.
 *
 * Canonical SI/IEC spellings are therefore preserved.
 *
 * ============================================================================
 * BIT/BYTE SEMANTIC DISTINCTION
 * ============================================================================
 *
 * These are different dimensions:
 *
 *     bit
 *
 * and:
 *
 *     byte
 *
 * A byte-oriented unit is not lexically converted into eight bits here.
 *
 * The semantic layer owns conversion.
 *
 * This is important because source-level semantic systems may need to retain:
 *
 *     original unit
 *     exactness
 *     conversion provenance
 *     dimensional information
 *
 * until type checking or constant evaluation.
 *
 * ============================================================================
 * BYTE ABBREVIATION
 * ============================================================================
 *
 * The canonical byte symbol is:
 *
 *     B
 *
 * The word:
 *
 *     byte
 *
 * is also accepted for source readability.
 *
 * Both:
 *
 *     1B
 *
 * and:
 *
 *     1byte
 *
 * are therefore SIZE_LITERAL.
 *
 * Their semantic representation should normalize to the same byte dimension.
 *
 * ============================================================================
 * BIT ABBREVIATION
 * ============================================================================
 *
 * The canonical explicit bit spelling is:
 *
 *     bit
 *
 * The single-letter:
 *
 *     b
 *
 * is intentionally NOT accepted.
 *
 * Reason:
 *
 *     b
 *
 * is a common identifier and can occur throughout general Zamani programs.
 *
 * Reserving it as a unit would unnecessarily reduce the identifier namespace.
 *
 * If a future language version adopts:
 *
 *     b
 *
 * it must be an explicit compatibility change.
 *
 * ============================================================================
 * BINARY PREFIX POLICY
 * ============================================================================
 *
 * Binary units use the unambiguous IEC forms:
 *
 *     KiB
 *     MiB
 *     GiB
 *     TiB
 *     PiB
 *     EiB
 *     ZiB
 *     YiB
 *
 * The grammar does NOT interpret:
 *
 *     KB
 *
 * as 1024 bytes.
 *
 * KB is decimal SI.
 *
 * KiB is binary IEC.
 *
 * This distinction is part of source semantics and must not be hidden by
 * target-dependent interpretation.
 *
 * ============================================================================
 * NO MACHINE LIMITS
 * ============================================================================
 *
 * There is deliberately no:
 *
 *     MAX_SIZE
 *     MAX_BYTES
 *     MAX_BITS
 *     MAX_KIB
 *     MAX_MEMORY
 *     MAX_ADDRESS_SIZE
 *     MAX_RESOURCE_SIZE
 *     MAX_DIGITS
 *     MAX_PRECISION
 *
 * The grammar uses unbounded lexical repetition:
 *
 *     +
 *
 * rather than finite repetition:
 *
 *     {1,64}
 *     {1,128}
 *     {1,256}
 *
 * A source program may therefore contain a quantity with as many digits as
 * the actual compiler/input resources can process.
 *
 * This is the grammar-level meaning of scalability.
 *
 * It does NOT imply infinite physical memory or infinite compiler resources.
 *
 * ============================================================================
 * ARBITRARY PRECISION
 * ============================================================================
 *
 * The lexer MUST preserve the source spelling.
 *
 * It must not convert:
 *
 *     999999999999999999999999999999999999B
 *
 * directly into:
 *
 *     u64
 *     i64
 *     usize
 *     f64
 *
 * merely because those types are convenient for implementation.
 *
 * A safe Rust semantic representation must preserve sufficient information
 * for later exact or policy-defined evaluation.
 *
 * ============================================================================
 * NO RUNTIME DEPENDENCY
 * ============================================================================
 *
 * This grammar does not:
 *
 *     query hardware;
 *     inspect memory;
 *     inspect devices;
 *     inspect operating systems;
 *     inspect environment variables;
 *     access filesystem;
 *     access network;
 *     inspect runtime capabilities;
 *     query quantum hardware;
 *     query accelerator availability.
 *
 * Lexical recognition is deterministic and target-independent.
 *
 * ============================================================================
 * DURATION SEPARATION
 * ============================================================================
 *
 * Duration syntax belongs to:
 *
 *     grammar/lexer/duration-literals.g4
 *
 * Therefore size-literals.g4 MUST NOT define:
 *
 *     ns
 *     us
 *     ms
 *     s
 *     min
 *     h
 *     d
 *     wk
 *
 * and must never recognize:
 *
 *     10ns
 *
 * as a size.
 *
 * This preserves the dimensional distinction between:
 *
 *     size
 *
 * and:
 *
 *     duration.
 *
 * ============================================================================
 * HARDWARE SEPARATION
 * ============================================================================
 *
 * Hardware-specific interpretation belongs to:
 *
 *     grammar/hardware/
 *     grammar/hdl/
 *     grammar/resources/
 *     hardware abstraction
 *     semantic analysis
 *
 * This file does not know whether:
 *
 *     64bit
 *
 * refers to:
 *
 *     an integer width
 *     a register width
 *     a data quantity
 *     a protocol field
 *     a memory quantity
 *     a hardware interface
 *     an HDL signal
 *
 * Context determines meaning after parsing.
 *
 * ============================================================================
 * HARDWARE LITERAL CONFLICT RESOLUTION
 * ============================================================================
 *
 * The current repository's hardware literal component contains generic
 * hardware-width/byte-width spellings such as:
 *
 *     8bit
 *     64bit
 *     1byte
 *
 * Those forms overlap this generic size-literal family.
 *
 * There must NOT be two competing lexical owners for the same source syntax.
 *
 * Required final architecture:
 *
 *     size-literals.g4
 *         |
 *         +--> SIZE_LITERAL
 *                   |
 *                   +--> semantic size quantity
 *                   |
 *                   +--> hardware semantic consumer
 *                   +--> resource semantic consumer
 *                   +--> HDL semantic consumer
 *                   +--> classical semantic consumer
 *                   +--> quantum/hybrid semantic consumer
 *
 * hardware-literals.g4 must therefore stop owning generic:
 *
 *     <number>bit
 *     <number>byte
 *
 * spellings.
 *
 * Hardware-specific syntax remains in hardware-literals.g4 only when it is
 * genuinely hardware-specific and cannot be represented as a generic size
 * quantity.
 *
 * This prevents token-order-dependent behavior in the canonical lexer.
 *
 * ============================================================================
 * CANONICAL TOKEN
 * ============================================================================
 *
 * This file exports exactly one public token for this literal family:
 *
 *     SIZE_LITERAL
 *
 * The numeric value and unit remain encoded in the token text.
 *
 * The semantic layer is responsible for parsing/canonicalizing the token
 * payload.
 *
 * The lexer must not manufacture a target-dependent semantic representation.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Conceptually:
 *
 *     SIZE_LITERAL
 *         |
 *         v
 *     source spelling
 *         |
 *         v
 *     size literal representation
 *         |
 *         v
 *     semantic quantity
 *         |
 *         +--> type checking
 *         +--> constant evaluation
 *         +--> resource analysis
 *         +--> compilation
 *         +--> hardware capability checking
 *         +--> runtime policy
 *
 * The grammar owns only the first stage.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This lexer does not define an AST node.
 *
 * Downstream AST/semantic infrastructure should represent a size quantity
 * using a canonical size-literal representation that preserves at least:
 *
 *     source spelling
 *     numeric component
 *     unit
 *     source span
 *
 * Exact internal representation belongs outside this grammar.
 *
 * The grammar must not force:
 *
 *     u64
 *     i64
 *     usize
 *     f64
 *
 * as the AST representation.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * SIZE_LITERAL does not directly create an IR node.
 *
 * Semantic analysis determines how the quantity enters the appropriate IR.
 *
 * Possible consumers include:
 *
 *     classical IR
 *     resource IR
 *     hardware IR
 *     HDL IR
 *     data IR
 *     execution constraints
 *     quantum semantic representations
 *
 * The lexer must not depend on:
 *
 *     quantum::ir
 *     QEC
 *     ZQN
 *     scheduling
 *     routing
 *     optimization
 *     runtime
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum syntax may use size quantities to express source-level dimensions
 * or resource requirements.
 *
 * Examples conceptually include:
 *
 *     state size
 *     register width
 *     encoded data size
 *     resource requirement
 *
 * However:
 *
 *     SIZE_LITERAL
 *
 * does not determine:
 *
 *     number of physical qubits
 *     number of logical qubits
 *     backend
 *     topology
 *     gate set
 *     simulator size
 *     QPU capacity
 *
 * Quantum semantic lowering ultimately remains responsible for the canonical
 * quantum::ir boundary.
 *
 * ============================================================================
 * HDL INTEGRATION
 * ============================================================================
 *
 * HDL grammar may consume SIZE_LITERAL for source-level quantities such as:
 *
 *     data width
 *     storage size
 *     interface quantity
 *     memory declaration quantity
 *     parameterized hardware quantity
 *
 * The HDL layer determines whether a particular quantity is meaningful for:
 *
 *     signal
 *     register
 *     memory
 *     interface
 *     module
 *     implementation
 *
 * This lexer does not decide.
 *
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * Resource requirements may consume SIZE_LITERAL:
 *
 *     memory requirement
 *     storage requirement
 *     data capacity
 *     transfer quantity
 *     resource preference
 *
 * The resource subsystem determines whether a target can satisfy it.
 *
 * A source-level quantity is therefore a requirement/expression, not an
 * assertion that the target possesses the resource.
 *
 * ============================================================================
 * COMPILATION INTEGRATION
 * ============================================================================
 *
 * Compile-time processing may:
 *
 *     validate units
 *     canonicalize units
 *     perform exact constant evaluation
 *     detect overflow
 *     detect incompatible dimensions
 *     lower size expressions
 *
 * It must not move those responsibilities into the lexer.
 *
 * ============================================================================
 * RUNTIME INTEGRATION
 * ============================================================================
 *
 * Runtime may consume the semantic representation for:
 *
 *     allocation
 *     buffering
 *     transfer
 *     deployment
 *     resource negotiation
 *
 * Runtime availability does not influence lexical recognition.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This file contains:
 *
 *     no semantic predicates;
 *     no actions;
 *     no target queries;
 *     no runtime queries;
 *     no filesystem operations;
 *     no network operations;
 *     no randomness;
 *     no time-dependent behavior.
 *
 * Identical source input must therefore produce identical lexical output.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * A size literal must never automatically grant:
 *
 *     memory access;
 *     device access;
 *     DMA;
 *     MMIO;
 *     privileged execution;
 *     allocation authority;
 *     resource reservation.
 *
 * Capability and authorization checks belong downstream.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Once SIZE_LITERAL is exposed to parser/AST tooling, its token identity is
 * part of the grammar compatibility surface.
 *
 * Changes to the public token or unit vocabulary require:
 *
 *     language-version update
 *     compatibility analysis
 *     lexer regression tests
 *     parser regression tests
 *     documentation update
 *     migration policy where necessary
 *
 * Existing generic numeric literals must remain valid.
 *
 * Existing duration literals must remain owned by duration-literals.g4.
 *
 * ============================================================================
 * LEXER RULE
 * ============================================================================
 *
 * IMPORTANT:
 *
 * The complete SIZE_LITERAL rule must appear before generic numeric tokens in
 * the final canonical lexer assembly, or otherwise be arranged so that the
 * canonical lexer gives the complete size literal one unambiguous owner.
 *
 * ANTLR's longest-match behavior normally causes:
 *
 *     1024KiB
 *
 * to prefer SIZE_LITERAL over a shorter INTEGER token.
 *
 * Nevertheless, lexical ownership must remain explicit rather than relying on
 * accidental import ordering.
 *
 * ============================================================================
 */

lexer grammar ZamaniSizeLiterals;

/*
 * ============================================================================
 * PUBLIC TOKEN
 * ============================================================================
 *
 * Exactly one public token is emitted by this component.
 *
 * The numeric value and unit remain part of the token text.
 *
 * Semantic analysis owns interpretation.
 */
SIZE_LITERAL
    : SIZE_NUMBER SIZE_UNIT
    ;


/*
 * ============================================================================
 * SIZE NUMBER
 * ============================================================================
 *
 * Supported:
 *
 *     10
 *     10.5
 *     .5
 *     1e3
 *     1.5e6
 *     1.5e-6
 *
 * No sign is included.
 */
fragment SIZE_NUMBER
    : DECIMAL_DIGITS
    | DECIMAL_DIGITS '.' DECIMAL_DIGITS? SIZE_EXPONENT?
    | '.' DECIMAL_DIGITS SIZE_EXPONENT?
    | DECIMAL_DIGITS SIZE_EXPONENT
    ;


/*
 * ============================================================================
 * EXPONENT
 * ============================================================================
 */
fragment SIZE_EXPONENT
    : [eE] [+-]? DECIMAL_DIGITS
    ;


/*
 * ============================================================================
 * SIZE UNITS
 * ============================================================================
 *
 * The alternatives are deliberately explicit.
 *
 * Do not replace this with a generic identifier suffix.
 *
 * Doing so would cause arbitrary identifiers to become size units and would
 * weaken lexical determinism.
 */
fragment SIZE_UNIT
    : 'Kibit'
    | 'Mibit'
    | 'Gibit'
    | 'Tibit'
    | 'Pibit'
    | 'Eibit'
    | 'Zibit'
    | 'Yibit'
    | 'KiB'
    | 'MiB'
    | 'GiB'
    | 'TiB'
    | 'PiB'
    | 'EiB'
    | 'ZiB'
    | 'YiB'
    | 'kbit'
    | 'Mbit'
    | 'Gbit'
    | 'Tbit'
    | 'Pbit'
    | 'Ebit'
    | 'Zbit'
    | 'Ybit'
    | 'bit'
    | 'kB'
    | 'MB'
    | 'GB'
    | 'TB'
    | 'PB'
    | 'EB'
    | 'ZB'
    | 'YB'
    | 'B'
    | 'byte'
    ;


/*
 * ============================================================================
 * DECIMAL DIGITS
 * ============================================================================
 *
 * A digit sequence may contain separators only between digits.
 *
 * Examples:
 *
 *     1024
 *     1_024
 *     10_000_000
 *
 * Invalid forms such as:
 *
 *     _1024
 *     1024_
 *     1__024
 *
 * are therefore not accepted by this fragment.
 */
fragment DECIMAL_DIGITS
    : DECIMAL_DIGIT (DECIMAL_DIGIT_OR_SEPARATOR* DECIMAL_DIGIT)?
    ;


fragment DECIMAL_DIGIT_OR_SEPARATOR
    : DECIMAL_DIGIT
    | '_'
    ;


fragment DECIMAL_DIGIT
    : [0-9]
    ;
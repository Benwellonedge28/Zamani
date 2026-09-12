/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/types/primitive-types.g4
 *
 * Status:
 *     Production-ready primitive-type grammar component.
 *
 * Role:
 *     Defines the source-level syntax for Zamani primitive types and
 *     primitive numeric families.
 *
 * Language:
 *     Zamani
 *
 * Implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     This grammar introduces no Rust implementation code and therefore
 *     requires no unsafe Rust. The Zamani compiler/runtime implementation
 *     consuming this grammar MUST remain safe Rust.
 *
 * ============================================================================
 *
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *                    Zamani source
 *                         |
 *                         v
 *                 ZamaniLexer
 *                         |
 *                         v
 *                 ZamaniParser
 *                         |
 *                         v
 *              primitiveTypeExpression
 *                         |
 *                         v
 *                  frontend AST
 *                         |
 *                         v
 *                semantic analysis
 *                         |
 *              +----------+----------+
 *              |                     |
 *              v                     v
 *        canonical type         capability /
 *        representation         resource analysis
 *              |
 *              v
 *        canonical IR
 *              |
 *       +------+------+----------------+
 *       |             |                |
 *       v             v                v
 *    classical     quantum          hardware
 *    lowering      lowering         lowering
 *       |             |                |
 *       +-------------+----------------+
 *                     |
 *                     v
 *             target realization
 *
 * ============================================================================
 *
 * OWNERSHIP
 * ============================================================================
 *
 * This file owns:
 *
 *   - syntax of source-level primitive type expressions;
 *   - syntax of fixed-width numeric type names;
 *   - syntax of target-dependent integer types;
 *   - syntax of semantic floating-point families;
 *   - syntax of textual primitive types;
 *   - syntax of boolean and unit/never primitives where their spelling is
 *     represented here;
 *   - stable parser-rule boundaries for primitive types.
 *
 * This file does NOT own:
 *
 *   - type identity;
 *   - name resolution;
 *   - aliases;
 *   - generic substitution;
 *   - type inference;
 *   - numeric promotion;
 *   - overflow semantics;
 *   - floating-point execution semantics;
 *   - ABI representation;
 *   - host-machine width;
 *   - register width;
 *   - pointer width;
 *   - memory layout;
 *   - hardware selection;
 *   - quantum allocation;
 *   - QEC;
 *   - ZQN;
 *   - scheduling;
 *   - routing;
 *   - optimization;
 *   - runtime representation.
 *
 * Those responsibilities belong to later semantic/compiler/runtime layers.
 *
 * ============================================================================
 *
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * A primitive type describes SOURCE-LEVEL COMPUTATIONAL MEANING.
 *
 * It must not encode:
 *
 *   - CPU count;
 *   - core count;
 *   - GPU count;
 *   - QPU count;
 *   - qubit count;
 *   - register count;
 *   - memory capacity;
 *   - physical address;
 *   - device ID;
 *   - topology;
 *   - accelerator availability.
 *
 * Fixed semantic widths such as i32 or f64 are language-level mathematical
 * contracts, not statements that the target hardware possesses a native
 * register of that width.
 *
 * Conversely:
 *
 *   isize
 *   usize
 *
 * are explicitly target-dependent semantic types.
 *
 * Their concrete representation is selected by the target/ABI layer.
 *
 * ============================================================================
 *
 * IMPORTANT LEXICAL DESIGN
 * ============================================================================
 *
 * The current Zamani lexer reserves:
 *
 *   void
 *   int
 *   float
 *   bool
 *   str
 *   string
 *   char
 *
 * It does NOT reserve:
 *
 *   i8
 *   i16
 *   i32
 *   i64
 *   i128
 *   isize
 *   u8
 *   u16
 *   u32
 *   u64
 *   u128
 *   usize
 *   f32
 *   f64
 *   f128
 *
 * This is intentional.
 *
 * Width-specific and precision-specific primitive names are ordinary
 * identifiers at the lexical layer. Semantic analysis recognizes the
 * canonical primitive vocabulary.
 *
 * This prevents the lexer from becoming an unnecessarily closed inventory
 * and leaves room for future semantic numeric formats.
 *
 * ============================================================================
 *
 * DEPENDENCY RULE
 * ============================================================================
 *
 * This is a PARSER GRAMMAR.
 *
 * It consumes tokens from ZamaniLexer.
 *
 * It must NOT duplicate lexer rules.
 *
 * The parent type grammar should import this grammar:
 *
 *     import PrimitiveTypes;
 *
 * The parent Types grammar owns the broader type-expression context.
 *
 * ============================================================================
 *
 * SEMANTIC RULE
 * ============================================================================
 *
 * The grammar accepts syntactically valid primitive-family spellings.
 *
 * Semantic analysis MUST determine whether a spelling denotes:
 *
 *   - a primitive;
 *   - a named type;
 *   - a user-defined type;
 *   - a type alias;
 *   - a generic type;
 *   - a domain-specific type;
 *   - a future extension.
 *
 * No grammar rule in this file may silently assign target-dependent meaning.
 *
 * ============================================================================
 */

parser grammar PrimitiveTypes;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * This is the sole public primitive-type entry point.
 *
 * Parent type grammars should consume `primitiveTypeExpression` rather than
 * duplicating primitive alternatives.
 *
 * ========================================================================== */

primitiveTypeExpression
    : primitiveScalarType
    | primitiveIntegerType
    | primitiveFloatType
    | primitiveTextType
    | primitiveUnitType
    | primitiveNeverType
    ;


/* ============================================================================
 * 2. SCALAR PRIMITIVES
 * ============================================================================
 *
 * These spellings are currently reserved by ZamaniLexer.
 *
 * Semantic meaning:
 *
 *     bool
 *     int
 *     float
 *
 * is determined by the canonical type system.
 *
 * In particular, `int` and `float` MUST NOT silently acquire different
 * semantics merely because compilation occurs on different hardware.
 *
 * If a language profile defines `int` or `float` as target-dependent aliases,
 * that policy must be explicit in semantic/type configuration rather than
 * being inferred from the host compiler.
 *
 * ========================================================================== */

primitiveScalarType
    : BOOL_TYPE
    | INT
    | FLOAT_TYPE
    ;


/* ============================================================================
 * 3. INTEGER PRIMITIVES
 * ============================================================================
 *
 * Zamani supports semantic integer families:
 *
 *     SignedInteger<W>
 *     UnsignedInteger<W>
 *
 * with canonical spellings:
 *
 *     i8
 *     i16
 *     i32
 *     i64
 *     i128
 *     isize
 *
 *     u8
 *     u16
 *     u32
 *     u64
 *     u128
 *     usize
 *
 * These are represented lexically as identifiers because the lexer deliberately
 * avoids creating an ever-growing keyword inventory.
 *
 * The semantic resolver MUST recognize the canonical spellings.
 *
 * No parser rule converts these names into host Rust integer types.
 *
 * ========================================================================== */

primitiveIntegerType
    : signedIntegerTypeName
    | unsignedIntegerTypeName
    ;


/* ============================================================================
 * 4. SIGNED INTEGER TYPE NAMES
 * ========================================================================== */

signedIntegerTypeName
    : primitiveIdentifier
    ;


/* ============================================================================
 * 5. UNSIGNED INTEGER TYPE NAMES
 * ========================================================================== */

unsignedIntegerTypeName
    : primitiveIdentifier
    ;


/*
 * IMPORTANT:
 *
 * The two rules above intentionally share the lexical representation.
 *
 * Semantic analysis distinguishes:
 *
 *     i8   -> SignedInteger<8>
 *     i16  -> SignedInteger<16>
 *     ...
 *
 *     u8   -> UnsignedInteger<8>
 *     u16  -> UnsignedInteger<16>
 *     ...
 *
 * This cannot safely be encoded using separate lexer tokens without turning
 * the lexer into a closed list of numeric types.
 *
 * The canonical AST/type resolver must therefore validate the identifier's
 * semantic spelling.
 */


/* ============================================================================
 * 6. CANONICAL SIGNED INTEGER FAMILY
 * ============================================================================
 *
 * This named façade exists for downstream tooling and semantic validation.
 *
 * It does not impose a maximum number of future integer widths.
 *
 * The actual spelling remains an identifier.
 *
 * ========================================================================== */

canonicalSignedIntegerName
    : primitiveIdentifier
    ;


/* ============================================================================
 * 7. CANONICAL UNSIGNED INTEGER FAMILY
 * ========================================================================== */

canonicalUnsignedIntegerName
    : primitiveIdentifier
    ;


/* ============================================================================
 * 8. FLOATING-POINT PRIMITIVES
 * ============================================================================
 *
 * Canonical source spellings currently represented by the frontend type
 * system include:
 *
 *     f32
 *     f64
 *     f128
 *
 * The lexer treats them as identifiers.
 *
 * `f128` does NOT assert that every target has native 128-bit floating-point
 * hardware.
 *
 * A backend may:
 *
 *     - implement it natively;
 *     - emulate it;
 *     - lower it to another representation;
 *     - reject execution because required resources/capabilities are absent.
 *
 * Such decisions are downstream.
 *
 * ========================================================================== */

primitiveFloatType
    : floatingTypeName
    ;


floatingTypeName
    : primitiveIdentifier
    ;


/* ============================================================================
 * 9. TEXT PRIMITIVES
 * ============================================================================
 *
 * Current lexical vocabulary:
 *
 *     str
 *     string
 *     char
 *
 * `str` and `string` are intentionally distinct source-level spellings.
 *
 * Their semantic relationship is determined by the canonical type system.
 *
 * ========================================================================== */

primitiveTextType
    : STR_TYPE
    | STRING_TYPE
    | CHAR_TYPE
    ;


/* ============================================================================
 * 10. UNIT
 * ============================================================================
 *
 * Unit is the type of a computation that produces no meaningful value.
 *
 * Canonical source spelling:
 *
 *     ()
 *
 * Unit is not equivalent to a machine integer, null pointer, or zero-sized
 * backend register merely because a target might implement it that way.
 *
 * ========================================================================== */

primitiveUnitType
    : LPAREN RPAREN
    ;


/* ============================================================================
 * 11. NEVER
 * ============================================================================
 *
 * Never is the uninhabited/bottom-like source type.
 *
 * Canonical source spelling:
 *
 *     Never
 *
 * It represents computations which do not produce a normal value.
 *
 * Examples include:
 *
 *     panic
 *     non-returning termination
 *     divergence
 *
 * Whether a particular control-flow path is proven to have Never type belongs
 * to semantic analysis.
 *
 * ========================================================================== */

primitiveNeverType
    : NEVER
    ;


/* ============================================================================
 * 12. PRIMITIVE IDENTIFIER
 * ============================================================================
 *
 * Numeric primitive spellings remain identifiers at the lexical layer.
 *
 * This rule delegates identifier syntax completely to ZamaniLexer.
 *
 * It MUST NOT introduce a second identifier definition.
 *
 * ========================================================================== */

primitiveIdentifier
    : IDENT
    ;


/* ============================================================================
 * 13. SEMANTICALLY CANONICAL PRIMITIVE NAME
 * ============================================================================
 *
 * This rule provides a stable boundary for tools that want to classify a
 * primitive-family identifier before semantic resolution.
 *
 * The grammar intentionally does not enumerate every possible future numeric
 * format.
 *
 * The semantic resolver owns canonical-name recognition.
 *
 * ========================================================================== */

primitiveNamedFamily
    : primitiveIdentifier
    ;


/* ============================================================================
 * 14. SOURCE-LEVEL PRIMITIVE FAMILY
 * ============================================================================
 *
 * This façade is useful for AST builders and parser integrations which need
 * to consume a primitive-family spelling without deciding its semantic
 * category at grammar time.
 *
 * ========================================================================== */

primitiveFamily
    : primitiveScalarType
    | primitiveIntegerType
    | primitiveFloatType
    | primitiveTextType
    | primitiveNeverType
    | primitiveUnitType
    ;


/* ============================================================================
 * 15. TYPE-SYSTEM INTEGRATION CONTRACT
 * ============================================================================
 *
 * Semantic analysis MUST map this grammar into the canonical frontend
 * representation.
 *
 * Expected conceptual mappings include:
 *
 *     bool
 *         -> PrimitiveType::Bool
 *
 *     char
 *         -> PrimitiveType::Char
 *
 *     str
 *         -> PrimitiveType::Str
 *
 *     string
 *         -> PrimitiveType::String
 *
 *     ()
 *         -> PrimitiveType::Unit
 *
 *     Never
 *         -> Never / PrimitiveType::Never according to the canonical AST
 *
 *     i8
 *         -> PrimitiveType::Int(IntegerType::Bits8)
 *
 *     i16
 *         -> PrimitiveType::Int(IntegerType::Bits16)
 *
 *     i32
 *         -> PrimitiveType::Int(IntegerType::Bits32)
 *
 *     i64
 *         -> PrimitiveType::Int(IntegerType::Bits64)
 *
 *     i128
 *         -> PrimitiveType::Int(IntegerType::Bits128)
 *
 *     isize
 *         -> PrimitiveType::Int(IntegerType::Size)
 *
 *     u8
 *         -> PrimitiveType::UInt(IntegerType::Bits8)
 *
 *     u16
 *         -> PrimitiveType::UInt(IntegerType::Bits16)
 *
 *     u32
 *         -> PrimitiveType::UInt(IntegerType::Bits32)
 *
 *     u64
 *         -> PrimitiveType::UInt(IntegerType::Bits64)
 *
 *     u128
 *         -> PrimitiveType::UInt(IntegerType::Bits128)
 *
 *     usize
 *         -> PrimitiveType::UInt(IntegerType::Size)
 *
 *     f32
 *         -> PrimitiveType::Float(FloatType::Bits32)
 *
 *     f64
 *         -> PrimitiveType::Float(FloatType::Bits64)
 *
 *     f128
 *         -> PrimitiveType::Float(FloatType::Bits128)
 *
 * This mapping is semantic/AST work and MUST NOT be implemented by this
 * grammar.
 *
 * ============================================================================
 */


/* ============================================================================
 * 16. `int` AND `float` SEMANTIC CONTRACT
 * ============================================================================
 *
 * The existing lexer reserves:
 *
 *     int
 *     float
 *
 * They are therefore accepted here.
 *
 * However, the semantic layer MUST define their meaning explicitly.
 *
 * Forbidden:
 *
 *     int -> i32 on target A
 *     int -> i64 on target B
 *
 * merely because those happen to be convenient host representations.
 *
 * If `int` is defined as a target-dependent language type, that dependency
 * must be explicit and represented in the canonical type semantics.
 *
 * If strict POCO-REAF reproducibility is required, a program should use
 * explicit semantic-width types or a language profile whose contract fixes
 * the meaning of `int`.
 *
 * ============================================================================
 */


/* ============================================================================
 * 17. ARBITRARY-PRECISION / PARAMETRIC NUMERIC TYPES
 * ============================================================================
 *
 * The primitive grammar intentionally does NOT attempt to enumerate arbitrary
 * precision numeric types.
 *
 * Future or library-defined forms such as:
 *
 *     Integer<W>
 *     SignedInteger<W>
 *     UnsignedInteger<W>
 *     Float<P>
 *     Decimal<P,S>
 *     FixedPoint<I,F>
 *
 * belong to generic/named type grammar.
 *
 * For example:
 *
 *     SignedInteger<Width>
 *     UnsignedInteger<Width>
 *     Float<Precision>
 *
 * can be parsed by the broader type grammar without modifying this file.
 *
 * This is critical for long-term extensibility.
 *
 * ============================================================================
 */


/* ============================================================================
 * 18. HARDWARE INDEPENDENCE
 * ============================================================================
 *
 * No primitive rule may encode:
 *
 *     CPU width
 *     pointer width
 *     SIMD width
 *     GPU width
 *     QPU width
 *     FPGA fabric size
 *     memory capacity
 *     register count
 *     cache size
 *     physical address size
 *
 * For example:
 *
 *     u64
 *
 * means a semantic unsigned 64-bit integer.
 *
 * It does NOT mean:
 *
 *     "use one 64-bit CPU register".
 *
 * Likewise:
 *
 *     f128
 *
 * does NOT mean:
 *
 *     "the target must have a native 128-bit floating-point register".
 *
 * ============================================================================
 */


/* ============================================================================
 * 19. QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum types are NOT primitive types in this file.
 *
 * For example:
 *
 *     Qubit
 *     QuantumState
 *     QRegister
 *     LogicalQubit
 *     PhysicalQubit
 *
 * belong to the quantum type grammar.
 *
 * The primitive grammar must not duplicate those concepts.
 *
 * This preserves the architecture:
 *
 *     primitive grammar
 *          |
 *          +--> classical primitive semantics
 *
 *     quantum grammar
 *          |
 *          +--> quantum semantic types
 *
 * Both ultimately resolve through the canonical frontend type system.
 *
 * `Qubit` must never become an alias for an integer merely because a backend
 * happens to represent qubit handles numerically.
 *
 * ============================================================================
 */


/* ============================================================================
 * 20. HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Hardware widths, ports, buses, signals and registers are NOT represented
 * by arbitrary primitive machine assumptions here.
 *
 * A hardware declaration may use a primitive semantic type where appropriate,
 * while hardware-specific width/shape/resource constraints belong to:
 *
 *     hardware grammar
 *     HDL grammar
 *     resource grammar
 *     semantic analysis
 *     target realization
 *
 * This prevents:
 *
 *     `u32`
 *
 * from silently meaning:
 *
 *     "physical 32-bit hardware register".
 *
 * ============================================================================
 */


/* ============================================================================
 * 21. RESOURCE INTEGRATION
 * ============================================================================
 *
 * Primitive values can participate in resource expressions, but this grammar
 * does not own resource semantics.
 *
 * For example:
 *
 *     usize
 *
 * is a target-dependent language integer type.
 *
 * It is NOT:
 *
 *     available_memory
 *     available_qubits
 *     device_count
 *     core_count
 *
 * Those concepts belong to resource/capability analysis.
 *
 * ============================================================================
 */


/* ============================================================================
 * 22. DETERMINISM
 * ============================================================================
 *
 * Parsing of primitive syntax must be deterministic.
 *
 * The same source token sequence must produce the same parse tree regardless
 * of:
 *
 *     target hardware
 *     compiler host
 *     operating system
 *     backend
 *     simulator
 *     runtime
 *
 * Target-dependent interpretation occurs after parsing.
 *
 * ============================================================================
 */


/* ============================================================================
 * 23. SCALABILITY
 * ============================================================================
 *
 * This grammar contains no language-level limits for:
 *
 *     integer magnitude
 *     numeric cardinality
 *     generic arity
 *     tensor rank
 *     collection size
 *     qubit count
 *     machine size
 *     resource count
 *
 * Any compiler implementation limit MUST be represented as an explicit
 * implementation/resource policy rather than a grammar constant.
 *
 * ============================================================================
 */


/* ============================================================================
 * 24. COMPILER ERROR BOUNDARY
 * ============================================================================
 *
 * This grammar should report syntactic errors.
 *
 * It MUST NOT attempt to report semantic errors such as:
 *
 *     unknown primitive name
 *     unsupported target precision
 *     unavailable hardware support
 *     illegal numeric conversion
 *     integer overflow
 *     unsupported floating-point format
 *
 * Those belong to later diagnostic phases.
 *
 * ============================================================================
 */


/* ============================================================================
 * 25. COMPATIBILITY
 * ============================================================================
 *
 * Existing source spellings must remain parseable unless explicitly deprecated
 * by the language compatibility policy.
 *
 * Current reserved primitive spellings:
 *
 *     void
 *     int
 *     float
 *     bool
 *     str
 *     string
 *     char
 *     Never
 *
 * remain accepted.
 *
 * Numeric names remain identifier-based to avoid unnecessary lexer changes.
 *
 * ============================================================================
 */
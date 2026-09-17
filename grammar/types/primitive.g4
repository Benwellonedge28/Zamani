/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/types/primitive.g4
 *
 * Status:
 *     Canonical primitive-type parser component.
 *
 * Purpose:
 *     Defines the complete source-level primitive-type syntax used by Zamani.
 *
 * Architectural role:
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     PrimitiveTypes parser component
 *          |
 *          v
 *     frontend AST TypeExpr / PrimitiveType
 *          |
 *          v
 *     semantic type resolution
 *          |
 *          v
 *     canonical semantic IR
 *          |
 *          +-----------------------------+
 *          |             |               |
 *          v             v               v
 *       classical     quantum          HDL /
 *       semantics     semantics        hardware
 *          |             |               |
 *          +-------------+---------------+
 *                        |
 *                        v
 *                 target-independent
 *                   optimization
 *                        |
 *                        v
 *               target realization
 *
 * ============================================================================
 *
 * OWNERSHIP
 * ============================================================================
 *
 * This file OWNS:
 *
 *   - the parser-level primitiveType entry point;
 *   - source-level primitive scalar types;
 *   - source-level fixed-width integer types;
 *   - source-level target-sized integer types;
 *   - source-level floating-point types;
 *   - source-level textual primitive types;
 *   - source-level character type;
 *   - source-level unit type;
 *   - source-level never type;
 *   - stable parser boundaries consumed by the broader type grammar.
 *
 * This file DOES NOT OWN:
 *
 *   - lexical token definitions;
 *   - identifier syntax;
 *   - type aliases;
 *   - named types;
 *   - generic types;
 *   - composite types;
 *   - type inference;
 *   - type unification;
 *   - numeric promotion;
 *   - overload resolution;
 *   - overflow policy;
 *   - floating-point execution;
 *   - ABI representation;
 *   - target selection;
 *   - hardware discovery;
 *   - resource discovery;
 *   - quantum physical allocation;
 *   - physical qubit mapping;
 *   - routing;
 *   - scheduling;
 *   - QEC;
 *   - ZQN;
 *   - HAL;
 *   - optimization;
 *   - runtime representation.
 *
 * ============================================================================
 *
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Primitive types describe SOURCE-LEVEL MEANING.
 *
 * They MUST NOT encode implementation limits such as:
 *
 *   MAX_CPUS
 *   MAX_CORES
 *   MAX_THREADS
 *   MAX_GPUS
 *   MAX_FPGAS
 *   MAX_QPUS
 *   MAX_QUBITS
 *   MAX_MEMORY
 *   MAX_REGISTER_WIDTH
 *   MAX_REGISTER_COUNT
 *   MAX_TENSOR_DIMENSION
 *   MAX_TENSOR_RANK
 *   MAX_NODES
 *   MAX_DEVICES
 *   MAX_ACCELERATORS
 *
 * Fixed-width types such as i32, u64, and f64 are semantic language
 * contracts. They do NOT imply that the target has a native register,
 * instruction, or hardware unit of that width.
 *
 * `isize` and `usize` are target-dependent LANGUAGE TYPES.
 *
 * Their concrete representation is selected by the semantic/target layer.
 *
 * ============================================================================
 *
 * IMPORTANT LEXICAL CONTRACT
 * ============================================================================
 *
 * The current Zamani lexer already owns tokens for:
 *
 *     VOID
 *     INT
 *     FLOAT_TYPE
 *     BOOL_TYPE
 *     STR_TYPE
 *     STRING_TYPE
 *     CHAR_TYPE
 *     NEVER
 *
 * The canonical identifier token is:
 *
 *     IDENTIFIER
 *
 * Fixed-width primitive spellings MUST NOT be accepted through IDENTIFIER
 * inside this grammar because that would make:
 *
 *     i8
 *     u8
 *     f64
 *
 * indistinguishable from:
 *
 *     MyType
 *     UserDefinedType
 *     quantum::State
 *
 * at the parser boundary.
 *
 * Therefore the canonical lexer contract must expose stable tokens for the
 * fixed-width primitive vocabulary before this grammar is composed.
 *
 * Required lexical tokens:
 *
 *     I8
 *     I16
 *     I32
 *     I64
 *     I128
 *     ISIZE
 *
 *     U8
 *     U16
 *     U32
 *     U64
 *     U128
 *     USIZE
 *
 *     F32
 *     F64
 *     F128
 *
 * These are lexical spellings, not hardware declarations.
 *
 * ============================================================================
 *
 * TOKEN OWNERSHIP
 * ============================================================================
 *
 * Token definitions belong under:
 *
 *     grammar/lexer/
 *
 * and ultimately to:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * This file MUST NOT redefine those lexer tokens.
 *
 * ============================================================================
 *
 * AST CONTRACT
 * ============================================================================
 *
 * Primitive syntax lowers to the existing canonical frontend AST primitive
 * model:
 *
 *     src/frontend/ast/node/types/primitive.rs
 *
 * Conceptual mappings:
 *
 *     unit
 *         -> PrimitiveType::Unit
 *
 *     bool
 *         -> PrimitiveType::Bool
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
 *         -> canonical Never representation
 *
 * This grammar MUST NOT introduce a second primitive-type AST.
 *
 * ============================================================================
 *
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Syntax establishes which primitive spelling occurred.
 *
 * Semantic analysis establishes:
 *
 *   - exact type identity;
 *   - legality in context;
 *   - conversions;
 *   - promotions;
 *   - arithmetic rules;
 *   - overflow behavior;
 *   - target requirements;
 *   - target capabilities;
 *   - representation;
 *   - ABI decisions.
 *
 * None of those decisions belong in this grammar.
 *
 * ============================================================================
 *
 * PORTABILITY CONTRACT
 * ============================================================================
 *
 * `i32` means exactly a 32-bit signed integer at the LANGUAGE level.
 *
 * It does not mean:
 *
 *     "use a native 32-bit CPU register".
 *
 * Likewise:
 *
 *     f128
 *
 * does not require native 128-bit floating-point hardware.
 *
 * A backend may implement a semantic primitive:
 *
 *   - natively;
 *   - through software;
 *   - through a wider representation;
 *   - through a supported accelerator;
 *   - through another valid lowering;
 *   - or reject it because the selected execution environment cannot satisfy
 *     the semantic requirements.
 *
 * Such decisions happen after parsing.
 *
 * ============================================================================
 *
 * RUST CONTRACT
 * ============================================================================
 *
 * Rust implementation baseline:
 *
 *     Rust 1.97 / Rust 1.97.1
 *
 * This grammar contains no Rust implementation code.
 *
 * The consuming compiler implementation MUST use safe Rust only.
 *
 * No unsafe Rust is required by this grammar.
 *
 * ============================================================================
 */

parser grammar PrimitiveTypes;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * Exactly one public primitive-type entry point is provided.
 *
 * The broader type grammar should import this parser grammar and use:
 *
 *     primitiveType
 *
 * It must not create another primitiveType rule.
 * ============================================================================
 */

primitiveType
    : primitiveScalarType
    | primitiveIntegerType
    | primitiveFloatType
    | primitiveTextType
    | unitType
    | neverType
    ;


/* ============================================================================
 * SCALAR TYPES
 * ============================================================================
 *
 * These tokens already belong to the canonical lexical vocabulary.
 * ============================================================================
 */

primitiveScalarType
    : BOOL_TYPE
    | INT
    | FLOAT_TYPE
    ;


/* ============================================================================
 * INTEGER TYPES
 * ============================================================================
 *
 * Fixed-width integer types are language-level semantic widths.
 *
 * There is no maximum width encoded by this grammar.
 *
 * The explicitly standardized primitive widths currently represented by the
 * frontend AST are 8, 16, 32, 64, and 128 bits, plus target-sized integers.
 *
 * Future arbitrary-width integer types belong in the generic/named type
 * system, for example:
 *
 *     SignedInteger<W>
 *     UnsignedInteger<W>
 *
 * They must NOT require adding another parser alternative here for every
 * possible W.
 * ============================================================================
 */

primitiveIntegerType
    : signedIntegerType
    | unsignedIntegerType
    ;


signedIntegerType
    : I8
    | I16
    | I32
    | I64
    | I128
    | ISIZE
    ;


unsignedIntegerType
    : U8
    | U16
    | U32
    | U64
    | U128
    | USIZE
    ;


/* ============================================================================
 * FLOATING-POINT TYPES
 * ============================================================================
 *
 * Floating-point precision is semantic.
 *
 * Native hardware support is a downstream concern.
 * ============================================================================
 */

primitiveFloatType
    : F32
    | F64
    | F128
    ;


/* ============================================================================
 * TEXT TYPES
 * ============================================================================
 *
 * `str` and `string` remain separate source-level spellings because the
 * existing frontend AST distinguishes borrowed text from owned text.
 *
 * The semantic layer owns lifetime, ownership, allocation and representation.
 * ============================================================================
 */

primitiveTextType
    : STR_TYPE
    | STRING_TYPE
    | CHAR_TYPE
    ;


/* ============================================================================
 * UNIT
 * ============================================================================
 *
 * Unit has no meaningful value.
 *
 * Canonical source spelling:
 *
 *     ()
 *
 * Unit is a semantic type, not a machine representation.
 * ============================================================================
 */

unitType
    : LPAREN RPAREN
    ;


/* ============================================================================
 * NEVER
 * ============================================================================
 *
 * Never is the uninhabited/non-returning type.
 *
 * Canonical source spelling:
 *
 *     Never
 *
 * Control-flow analysis determines whether an expression or block has Never
 * type.
 * ============================================================================
 */

neverType
    : NEVER
    ;


/* ============================================================================
 * CANONICAL FAMILY SUBRULES
 * ============================================================================
 *
 * These stable rules give downstream tooling precise parser boundaries without
 * duplicating the public primitiveType entry point.
 * ============================================================================
 */

signedIntegerPrimitive
    : I8
    | I16
    | I32
    | I64
    | I128
    | ISIZE
    ;


unsignedIntegerPrimitive
    : U8
    | U16
    | U32
    | U64
    | U128
    | USIZE
    ;


floatingPrimitive
    : F32
    | F64
    | F128
    ;


booleanPrimitive
    : BOOL_TYPE
    ;


characterPrimitive
    : CHAR_TYPE
    ;


borrowedStringPrimitive
    : STR_TYPE
    ;


ownedStringPrimitive
    : STRING_TYPE
    ;


unitPrimitive
    : LPAREN RPAREN
    ;


neverPrimitive
    : NEVER
    ;


/* ============================================================================
 * SEMANTIC CLASSIFICATION CONTRACT
 * ============================================================================
 *
 * The following table is normative for downstream AST construction.
 *
 *     TOKEN        AST SEMANTIC TYPE
 *
 *     BOOL_TYPE    PrimitiveType::Bool
 *
 *     I8           PrimitiveType::Int(IntegerType::Bits8)
 *     I16          PrimitiveType::Int(IntegerType::Bits16)
 *     I32          PrimitiveType::Int(IntegerType::Bits32)
 *     I64          PrimitiveType::Int(IntegerType::Bits64)
 *     I128         PrimitiveType::Int(IntegerType::Bits128)
 *     ISIZE        PrimitiveType::Int(IntegerType::Size)
 *
 *     U8           PrimitiveType::UInt(IntegerType::Bits8)
 *     U16          PrimitiveType::UInt(IntegerType::Bits16)
 *     U32          PrimitiveType::UInt(IntegerType::Bits32)
 *     U64          PrimitiveType::UInt(IntegerType::Bits64)
 *     U128         PrimitiveType::UInt(IntegerType::Bits128)
 *     USIZE        PrimitiveType::UInt(IntegerType::Size)
 *
 *     F32          PrimitiveType::Float(FloatType::Bits32)
 *     F64          PrimitiveType::Float(FloatType::Bits64)
 *     F128         PrimitiveType::Float(FloatType::Bits128)
 *
 *     CHAR_TYPE    PrimitiveType::Char
 *     STR_TYPE     PrimitiveType::Str
 *     STRING_TYPE  PrimitiveType::String
 *     LPAREN RPAREN PrimitiveType::Unit
 *
 * `Never` remains the canonical uninhabited type representation used by the
 * frontend type system.
 *
 * The exact Rust enum construction belongs to:
 *
 *     src/frontend/ast/node/types/primitive.rs
 *
 * and its builders/conversion layer.
 *
 * ============================================================================
 *
 * `int` AND `float`
 * ============================================================================
 *
 * `int` and `float` are retained because they are already part of the
 * language's lexical vocabulary.
 *
 * Their semantic definition MUST be explicit.
 *
 * This grammar does not silently redefine:
 *
 *     int -> i32
 *     int -> i64
 *     float -> f32
 *     float -> f64
 *
 * based on the host machine.
 *
 * If the language specification defines these as target-independent canonical
 * types, that definition belongs in the semantic type specification.
 *
 * If they are defined as target-dependent types, the target dependency must
 * be explicit in semantic analysis.
 *
 * The parser MUST NOT infer their meaning from Rust's native integer or
 * floating-point types.
 *
 * ============================================================================
 *
 * EXTENSIBILITY CONTRACT
 * ============================================================================
 *
 * Arbitrary or future numeric representations do not belong in this finite
 * primitive keyword list merely because they exist on one machine or in one
 * vendor ecosystem.
 *
 * Examples that can be represented by the broader type system include:
 *
 *     SignedInteger<W>
 *     UnsignedInteger<W>
 *     Float<P>
 *     Decimal<P, S>
 *     FixedPoint<I, F>
 *     Posit<P>
 *     CustomNumeric<F>
 *
 * Their syntax belongs to generic/named/dependent type grammar.
 *
 * Their semantics belong to the type system.
 *
 * Their implementation belongs to compiler/backend layers.
 *
 * ============================================================================
 *
 * HARDWARE INDEPENDENCE
 * ============================================================================
 *
 * This file MUST NEVER contain:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     QPU
 *     physical qubit
 *     device identifier
 *     memory-bank identifier
 *     register identifier
 *     topology
 *     vendor instruction
 *     vendor ABI
 *
 * Primitive types remain portable source-level contracts.
 *
 * ============================================================================
 *
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum source types such as:
 *
 *     Qubit
 *     LogicalQubit
 *     QRegister<N>
 *     QuantumState<T>
 *
 * are NOT primitive types.
 *
 * They belong to the quantum/type grammar.
 *
 * This boundary is deliberate:
 *
 *     primitiveType
 *          |
 *          +---- classical primitive semantics
 *
 *     quantumType
 *          |
 *          +---- quantum semantic model
 *                    |
 *                    v
 *                 quantum::ir
 *
 * This grammar therefore does not duplicate or replace `quantum::ir`.
 *
 * ============================================================================
 *
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Hardware widths, ports, buses, memories, accelerator resources and physical
 * implementation properties are not primitive types merely because they have
 * numeric values.
 *
 * Examples:
 *
 *     Bus<Width>
 *     Memory<T, Size>
 *     Accelerator<A>
 *
 * belong to the appropriate domain/resource type grammars.
 *
 * A primitive integer may be used as a parameter to those types, but this
 * grammar does not decide the hardware realization.
 *
 * ============================================================================
 *
 * RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Primitive values may participate in resource expressions:
 *
 *     requires memory >= 1024
 *     requires qubits >= N
 *
 * but primitiveType itself never performs resource discovery or validation.
 *
 * Resource requirements belong under:
 *
 *     grammar/resources/
 *
 * and hardware realization belongs under:
 *
 *     grammar/hardware/
 *
 * ============================================================================
 *
 * COMPILER INTEGRATION
 * ============================================================================
 *
 *     primitiveType
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic type resolution
 *          |
 *          v
 *     canonical semantic type
 *          |
 *          v
 *     ZUIR / canonical IR
 *          |
 *          +-----------------------+
 *          |                       |
 *          v                       v
 *     classical lowering     domain-specific lowering
 *                                  |
 *                                  v
 *                         quantum::ir / HDL / hardware
 *                                  |
 *                                  v
 *                           target realization
 *
 * This file must never lower directly to LLVM, QIR, MLIR, vendor ISA,
 * physical qubits, CPU registers, GPU registers, FPGA resources or ABI
 * layouts.
 *
 * ============================================================================
 *
 * RUNTIME INTEGRATION
 * ============================================================================
 *
 * Runtime representation is selected after semantic analysis and compilation.
 *
 * The grammar has no runtime dependency.
 *
 * Runtime code must not inspect source spellings such as `i64` to decide
 * arbitrary machine behavior. It consumes the canonical semantic/IR type
 * information produced downstream.
 *
 * ============================================================================
 *
 * DETERMINISM
 * ============================================================================
 *
 * Parsing the same source under the same language/grammar version MUST produce
 * the same primitive-type syntax tree.
 *
 * Target discovery MUST NOT influence parsing.
 *
 * ============================================================================
 *
 * DIAGNOSTICS
 * ============================================================================
 *
 * Invalid primitive spellings must be rejected by the lexical/parser/type
 * pipeline without silently converting them into a different primitive.
 *
 * Examples:
 *
 *     i256
 *     u256
 *     f256
 *
 * are NOT silently interpreted as i128/u128/f128.
 *
 * If arbitrary-width numeric types are supported, they must use the explicit
 * generic/dependent syntax defined by the type system.
 *
 * Likewise:
 *
 *     i32foo
 *
 * is an identifier according to the lexical identifier contract and must be
 * resolved as an identifier/named type, not partially interpreted as i32.
 *
 * ============================================================================
 *
 * NEGATIVE / BOUNDARY / SCALABILITY CONTRACT
 * ============================================================================
 *
 * Must reject:
 *
 *     unknown primitive spellings pretending to be canonical primitives;
 *     malformed width spellings;
 *     partially matched primitive names;
 *     hardware-specific primitive aliases without a language declaration.
 *
 * Must accept:
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
 *     f32
 *     f64
 *     f128
 *
 *     bool
 *     char
 *     str
 *     string
 *     ()
 *     Never
 *
 * Boundary/scalability:
 *
 *     The grammar contains no finite resource-size limit.
 *
 * A program may use arbitrarily many primitive-typed declarations subject
 * only to the parser/compiler/runtime resource policies outside the language
 * semantics.
 *
 * ============================================================================
 *
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *   [ ] exactly one public primitiveType entry point exists;
 *   [ ] no lexer rules are duplicated here;
 *   [ ] fixed-width primitives have dedicated lexer tokens;
 *   [ ] named types cannot be accidentally parsed as primitive types;
 *   [ ] int/float semantics are not inferred from host Rust;
 *   [ ] unit is represented explicitly;
 *   [ ] Never is represented explicitly;
 *   [ ] signed and unsigned integer families are distinct;
 *   [ ] target-sized integers remain target-dependent semantic types;
 *   [ ] floating precision remains semantic;
 *   [ ] no hardware limits exist;
 *   [ ] no quantum hardware concepts exist here;
 *   [ ] no QEC/ZQN/HAL logic exists here;
 *   [ ] no second AST exists;
 *   [ ] existing PrimitiveType AST remains canonical;
 *   [ ] downstream type resolution owns semantics;
 *   [ ] compiler/IR lowering remains downstream;
 *   [ ] runtime representation remains downstream;
 *   [ ] positive tests exist;
 *   [ ] negative tests exist;
 *   [ ] boundary tests exist;
 *   [ ] scalability tests exist;
 *   [ ] compatibility tests exist;
 *   [ ] Rust implementation remains safe Rust 1.97/1.97.1.
 *
 * ============================================================================
 */
/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/declarations/unions.g4
 *
 * Grammar:
 *     ZamaniDeclarationUnions
 *
 * Role:
 *     Canonical parser delegate for source-level union declarations.
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     This grammar has no Rust implementation code and requires no unsafe Rust.
 *
 * ============================================================================
 *
 * OWNERSHIP
 * ============================================================================
 *
 * This grammar owns:
 *
 *     - unionDeclaration
 *     - unionVariantList
 *     - unionVariant
 *     - unionVariantPayload
 *     - unionTuplePayload
 *     - unionStructPayload
 *     - unionVariantField
 *
 * This grammar does NOT own:
 *
 *     - lexical token definitions
 *     - keywords
 *     - identifiers
 *     - attributes
 *     - declaration modifiers
 *     - generic-parameter semantics
 *     - generic-parameter syntax
 *     - type-expression syntax
 *     - struct declarations
 *     - enum declarations
 *     - aliases
 *     - interfaces
 *     - traits
 *     - implementations
 *     - classes
 *     - records
 *     - pattern matching
 *     - exhaustiveness checking
 *     - name resolution
 *     - type checking
 *     - recursive-type validation
 *     - discriminant assignment
 *     - representation/layout
 *     - ABI selection
 *     - memory allocation
 *     - hardware selection
 *     - quantum routing
 *     - scheduling
 *     - QEC
 *     - ZQN
 *     - resilience
 *     - runtime behavior
 *     - classical IR
 *     - quantum::ir
 *
 * ============================================================================
 *
 * ARCHITECTURAL PIPELINE
 * ============================================================================
 *
 *     source
 *       |
 *       v
 *     canonical lexer
 *       |
 *       v
 *     parser
 *       |
 *       +--> ZamaniDeclarationUnions
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
 *       +--> name resolution
 *       +--> generic validation
 *       +--> type validation
 *       +--> recursive-type validation
 *       +--> constructor validation
 *       +--> pattern/exhaustiveness validation
 *       |
 *       v
 *     canonical semantic representation
 *       |
 *       +--> classical IR
 *       +--> quantum semantic model
 *       +--> HDL/hardware semantic model
 *       +--> resource/capability metadata
 *       |
 *       v
 *     optimization / lowering
 *       |
 *       v
 *     target realization
 *
 * Union grammar MUST remain independent of physical hardware and runtime
 * realization.
 *
 * ============================================================================
 *
 * LEXICAL AUTHORITY
 * ============================================================================
 *
 * The canonical lexical vocabulary is supplied by the Zamani lexer boundary.
 *
 * This parser therefore consumes:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * DO NOT use:
 *
 *     tokenVocab = ZamaniTokens;
 *
 * inside this parser grammar.
 *
 * ZamaniTokens is the lexical composition vocabulary underneath the canonical
 * ZamaniLexer and is not a competing parser-facing vocabulary.
 *
 * ============================================================================
 *
 * SHARED RULE CONTRACT
 * ============================================================================
 *
 * The following rules are supplied by imported/composed grammars:
 *
 *     declarationModifiers
 *     identifier
 *     genericParameterList
 *     typeExpression
 *     attribute
 *
 * They MUST NOT be redefined here.
 *
 * This prevents independent declaration grammars from developing incompatible
 * identifier, generic, attribute, or type-expression syntax.
 *
 * ============================================================================
 *
 * POCO-REAF
 * ============================================================================
 *
 * Union declarations describe semantic alternatives.
 *
 * They do not prescribe:
 *
 *     CPU count
 *     GPU count
 *     FPGA count
 *     QPU count
 *     qubit count
 *     memory capacity
 *     register width
 *     accelerator count
 *     node count
 *     topology
 *     physical address
 *     device identifier
 *     network size
 *     cluster size
 *
 * No universal implementation limit is encoded in this grammar.
 *
 * A union may therefore be used in:
 *
 *     tiny embedded systems
 *     microcontrollers
 *     CPUs
 *     multicore systems
 *     GPUs
 *     FPGAs
 *     ASICs
 *     QPUs
 *     accelerators
 *     clusters
 *     supercomputers
 *     distributed systems
 *     cloud systems
 *     future computational substrates
 *
 * subject to actual semantic validity and available target resources.
 *
 * ============================================================================
 *
 * SCALABILITY
 * ============================================================================
 *
 * The grammar imposes no finite language-level maximum on:
 *
 *     - union declarations
 *     - generic parameters
 *     - variants
 *     - tuple payload elements
 *     - named payload fields
 *     - nesting
 *     - type-expression complexity
 *     - recursive references
 *
 * Repetition is represented using ANTLR repetition operators.
 *
 * Parser-resource limits, if required for denial-of-service protection or
 * implementation resource management, belong to the compiler/tooling layer and
 * MUST NOT change the language's semantic limits.
 *
 * ============================================================================
 *
 * UNION MODEL
 * ============================================================================
 *
 * Canonical form:
 *
 *     union NAME GENERICS? = VARIANT (| VARIANT)* ;
 *
 * Examples:
 *
 *     union Result<T, E> =
 *         Ok(T)
 *       | Err(E);
 *
 *     union Optional<T> =
 *         None
 *       | Some(T);
 *
 *     union Event =
 *         Start
 *       | Data(Bytes)
 *       | Stop;
 *
 *     union Error =
 *         Simple
 *       | Detailed {
 *             code: ErrorCode,
 *             message: String,
 *         };
 *
 * ============================================================================
 *
 * VARIANT MODEL
 * ============================================================================
 *
 * A variant consists of:
 *
 *     optional attributes
 *     identifier
 *     optional payload
 *
 * Payloads are either:
 *
 *     - tuple payloads
 *     - named-field payloads
 *
 * A variant without a payload is a unit variant.
 *
 * ============================================================================
 *
 * TRAILING COMMAS
 * ============================================================================
 *
 * Tuple payloads permit trailing commas:
 *
 *     Pair(A, B,)
 *
 * Named payloads permit trailing commas:
 *
 *     Error {
 *         code: ErrorCode,
 *         message: String,
 *     }
 *
 * ============================================================================
 *
 * NAMED VARIANT FIELDS
 * ============================================================================
 *
 * Named union-variant payloads intentionally have their own field rule.
 *
 * They MUST NOT reuse a general struct-member rule that could introduce:
 *
 *     functions
 *     methods
 *     constructors
 *     destructors
 *     properties
 *     arbitrary declarations
 *
 * A union variant payload contains data fields only.
 *
 * ============================================================================
 *
 * GENERIC TYPES
 * ============================================================================
 *
 * Payload types are delegated to the canonical type-expression grammar.
 *
 * Examples:
 *
 *     union Result<T, E> =
 *         Ok(T)
 *       | Err(E);
 *
 *     union Nested<T> =
 *         Value(List<Option<T>>);
 *
 *     union QuantumResult<T> =
 *         Value(T)
 *       | Measured(Qubit)
 *       | Failure(Error);
 *
 * This grammar does not define the meaning of those types.
 *
 * ============================================================================
 *
 * RECURSION
 * ============================================================================
 *
 * Recursive union references are syntactically legal:
 *
 *     union List<T> =
 *         Nil
 *       | Cons(T, List<T>);
 *
 * Semantic analysis determines whether the recursive type is valid and
 * representable.
 *
 * ============================================================================
 *
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum types may occur in union payloads through typeExpression.
 *
 * This grammar does NOT:
 *
 *     - allocate qubits
 *     - identify physical qubits
 *     - select QPUs
 *     - select gate sets
 *     - perform routing
 *     - perform scheduling
 *     - perform calibration
 *     - perform QEC
 *     - interpret ZQN noise
 *
 * Quantum semantics ultimately cross the repository's canonical:
 *
 *     quantum::ir
 *
 * boundary downstream of parsing and semantic analysis.
 *
 * ============================================================================
 *
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Hardware and HDL types may occur in payloads through typeExpression.
 *
 * This grammar does NOT encode:
 *
 *     - physical FPGA identifiers
 *     - ASIC identifiers
 *     - physical registers
 *     - fixed device counts
 *     - fixed memory capacities
 *     - physical topology
 *     - hardware addresses
 *
 * Those are downstream realization concerns.
 *
 * ============================================================================
 *
 * CLASSICAL / AI / DATA / DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * The union grammar remains domain-neutral.
 *
 * Any domain whose types are admitted by the canonical type system may be used
 * as a union payload.
 *
 * This allows future domains to integrate without modifying this grammar merely
 * because a new semantic type exists.
 *
 * ============================================================================
 *
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar produces ANTLR parser contexts only.
 *
 * It does NOT define Rust AST structures.
 *
 * The frontend AST layer maps the parse tree conceptually into:
 *
 *     UnionDeclaration {
 *         span,
 *         modifiers,
 *         attributes,
 *         name,
 *         parameters,
 *         variants,
 *     }
 *
 * and variants into the existing canonical variant representation.
 *
 * No parallel:
 *
 *     UnionAst
 *     UnionIR
 *     UnionSemanticNode
 *
 * types should be introduced merely because this grammar exists.
 *
 * ============================================================================
 *
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for:
 *
 *     - declaration-name resolution
 *     - generic-parameter validation
 *     - duplicate variant detection
 *     - duplicate field detection
 *     - payload type resolution
 *     - recursive-type validation
 *     - constructor validation
 *     - pattern compatibility
 *     - exhaustiveness analysis
 *     - ownership/resource validation
 *     - effect validation
 *     - capability validation
 *     - representation feasibility
 *
 * ============================================================================
 *
 * DETERMINISM
 * ============================================================================
 *
 * The grammar must parse identical source/token streams deterministically.
 *
 * It must not depend on:
 *
 *     - machine identity
 *     - hardware discovery
 *     - runtime state
 *     - resource availability
 *     - network state
 *     - quantum-device state
 *     - calibration state
 *
 * ============================================================================
 *
 * DIAGNOSTICS
 * ============================================================================
 *
 * Parser diagnostics must preserve:
 *
 *     - source span
 *     - offending token
 *     - expected syntactic category
 *
 * Semantic diagnostics belong downstream.
 *
 * Examples of syntax errors include:
 *
 *     union Result<T> =
 *
 * missing variants
 *
 *     union Result<T> = Ok(T) |
 *
 * incomplete variant
 *
 *     union Result<T> = Ok(T) Err(E);
 *
 * missing separator
 *
 * Semantic errors such as duplicate variants are NOT parser errors.
 *
 * ============================================================================
 *
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing union syntax should remain source-compatible where it does not
 * conflict with the canonical grammar architecture.
 *
 * Compatibility changes must be documented in:
 *
 *     grammar/compatibility/
 *
 * and the authoritative specification.
 *
 * ============================================================================
 *
 * TEST CONTRACT
 * ============================================================================
 *
 * Required positive coverage includes:
 *
 *     union Empty = Nothing;
 *
 *     union Result<T, E> =
 *         Ok(T)
 *       | Err(E);
 *
 *     union Optional<T> =
 *         None
 *       | Some(T);
 *
 *     union Event =
 *         Start
 *       | Data(Bytes)
 *       | Stop;
 *
 *     union Error =
 *         Simple
 *       | Detailed {
 *             code: ErrorCode,
 *             message: String,
 *         };
 *
 *     union Pair<A, B> =
 *         Value(A, B,);
 *
 *     union List<T> =
 *         Nil
 *       | Cons(T, List<T>);
 *
 * Required negative coverage includes:
 *
 *     missing union keyword
 *     missing identifier
 *     missing equals sign
 *     missing variant
 *     missing variant separator
 *     unterminated tuple payload
 *     unterminated struct payload
 *     missing field type
 *     missing field separator
 *
 * Required boundary coverage includes:
 *
 *     one variant
 *     many variants
 *     one tuple field
 *     many tuple fields
 *     one named field
 *     many named fields
 *     trailing comma
 *     nested type expressions
 *     recursive references
 *     generic variants
 *
 * Required scalability coverage must verify that no language-level artificial
 * limit exists on variant count, field count, payload arity, or generic arity.
 *
 * ============================================================================
 */

parser grammar ZamaniDeclarationUnions;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * UNION DECLARATION
 * ============================================================================
 *
 * A union declaration consists of:
 *
 *     declaration modifiers
 *     optional attributes
 *     union keyword
 *     identifier
 *     optional generic parameters
 *     equals sign
 *     one or more variants
 *     semicolon
 *
 * Generic syntax, identifiers, attributes, and modifiers are supplied by
 * canonical shared grammar components.
 */
unionDeclaration
    : declarationModifiers*
      attribute*
      K_UNION
      identifier
      genericParameterList?
      EQUAL
      unionVariantList
      SEMICOLON
    ;


/*
 * ============================================================================
 * VARIANT LIST
 * ============================================================================
 *
 * One or more variants separated by '|'.
 *
 * No finite maximum is encoded.
 */
unionVariantList
    : unionVariant
      (PIPE unionVariant)*
    ;


/*
 * ============================================================================
 * VARIANT
 * ============================================================================
 *
 * A variant may be:
 *
 *     Unit
 *     Tuple(Type, Type, ...)
 *     Struct { field: Type, ... }
 *
 * Attributes belong to the variant itself.
 */
unionVariant
    : attribute*
      identifier
      unionVariantPayload?
    ;


/*
 * ============================================================================
 * VARIANT PAYLOAD
 * ============================================================================
 */
unionVariantPayload
    : unionTuplePayload
    | unionStructPayload
    ;


/*
 * ============================================================================
 * TUPLE PAYLOAD
 * ============================================================================
 *
 * Examples:
 *
 *     Some(T)
 *     Pair(A, B)
 *     Triple(A, B, C,)
 *
 * The repetition operator intentionally imposes no finite source-language
 * maximum.
 */
unionTuplePayload
    : LPAREN
      typeExpression
      (COMMA typeExpression)*
      COMMA?
      RPAREN
    ;


/*
 * ============================================================================
 * STRUCT / NAMED-FIELD PAYLOAD
 * ============================================================================
 *
 * Examples:
 *
 *     Error {
 *         code: ErrorCode,
 *         message: String,
 *     }
 *
 * Only data fields are permitted.
 */
unionStructPayload
    : LBRACE
      unionVariantField
      (COMMA unionVariantField)*
      COMMA?
      RBRACE
    ;


/*
 * ============================================================================
 * UNION VARIANT FIELD
 * ============================================================================
 *
 * This is deliberately NOT structMember.
 *
 * A union payload field owns only:
 *
 *     name : type
 *
 * and therefore cannot accidentally admit methods, constructors, properties,
 * or arbitrary declarations.
 */
unionVariantField
    : attribute*
      identifier
      COLON
      typeExpression
    ;
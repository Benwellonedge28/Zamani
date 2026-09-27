/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/interoperability/foreign-types.g4
 *
 * Grammar:
 *     InteroperabilityForeignTypes
 *
 * Status:
 *     CANONICAL FOREIGN-TYPE SYNTAX CONTRACT
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     SAFE RUST ONLY
 *     NO UNSAFE RUST
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the SOURCE-LEVEL SYNTAX for types whose implementation,
 * representation, storage, or ownership ultimately exists outside the
 * ordinary Zamani source-language type declaration boundary.
 *
 * Examples include:
 *
 *     extern type NativeHandle;
 *     extern type ForeignBuffer<T>;
 *     extern type ForeignVector<T, Shape> : Vector<T>;
 *
 * and members inside an enclosing external declaration:
 *
 *     extern "C" {
 *         type NativeHandle;
 *         type ForeignBuffer<T>;
 *     }
 *
 * A foreign type is a SOURCE-LEVEL CONTRACT.
 *
 * It is NOT:
 *
 *     - an ABI layout;
 *     - a native pointer;
 *     - a machine register;
 *     - a physical address;
 *     - a linker symbol;
 *     - a runtime object;
 *     - a dynamic-library handle;
 *     - a hardware resource;
 *     - a quantum physical object;
 *     - a QPU allocation;
 *     - an FPGA resource;
 *     - a memory allocation;
 *     - an FFI marshalling implementation.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     grammar/antlr/ZamaniLexer.g4
 *          |
 *          v
 *     ZamaniParser
 *          |
 *          v
 *     InteroperabilityForeignTypes
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     structural validation
 *          |
 *          v
 *     semantic type analysis
 *          |
 *          +----------------------+---------------------+
 *          |                      |                     |
 *          v                      v                     v
 *      Zamani types              ABI                  FFI
 *          |                      |                     |
 *          +----------------------+---------------------+
 *                                 |
 *                                 v
 *                       canonical semantic model
 *                                 |
 *              +------------------+------------------+
 *              |                  |                 |
 *              v                  v                 v
 *         classical IR       quantum::ir       HDL/hardware
 *              |                  |                 |
 *              +------------------+------------------+
 *                                 |
 *                                 v
 *                         optimization/lowering
 *                                 |
 *                                 v
 *                       target realization
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * This file is the ONLY grammar owner for:
 *
 *     source-level foreign type declarations
 *     source-level foreign type members
 *     foreign type parameter clauses
 *     foreign type representation references
 *     foreign type references
 *
 * It does NOT own the general Zamani type language.
 *
 * General type syntax remains owned by:
 *
 *     grammar/types/types.g4
 *
 * General names remain owned by:
 *
 *     grammar/core/names.g4
 *
 * General attributes remain owned by:
 *
 *     grammar/core/attributes.g4
 *
 * Foreign callable declarations remain owned by:
 *
 *     grammar/interoperability/foreign-functions.g4
 *
 * FFI boundary behavior remains owned by:
 *
 *     grammar/interoperability/ffi.g4
 *
 * ABI syntax remains owned by:
 *
 *     grammar/interoperability/abi.g4
 *
 * General interoperability composition remains owned by:
 *
 *     grammar/interoperability/interoperability.g4
 *
 * ============================================================================
 * CRITICAL NON-DUPLICATION RULE
 * ============================================================================
 *
 * This grammar MUST NOT redefine:
 *
 *     identifier
 *     qualifiedName
 *     typeExpression
 *     typeExpr
 *     attribute
 *     expression
 *     parameter
 *     parameterList
 *     ABI rules
 *     FFI marshalling rules
 *     ownership rules
 *     lifetime rules
 *     callback rules
 *     calling-convention rules
 *     linker rules
 *     runtime rules
 *
 * Those constructs have existing canonical owners.
 *
 * ============================================================================
 * LEXER AUTHORITY
 * ============================================================================
 *
 * The parser-facing lexer authority is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Therefore this grammar consumes:
 *
 *     tokenVocab = ZamaniLexer
 *
 * It MUST NOT introduce lexer rules.
 *
 * It MUST NOT use a second parser-facing vocabulary such as:
 *
 *     ZamaniTokens
 *
 * The repository currently contains older modular grammar material using
 * different vocabulary conventions. That inconsistency must be eliminated at
 * the composition layer; this new grammar follows the canonical
 * `ZamaniLexer` boundary.
 *
 * ============================================================================
 * KEYWORD POLICY
 * ============================================================================
 *
 * This grammar deliberately uses existing canonical lexical concepts:
 *
 *     EXTERN
 *     TYPE
 *
 * It does NOT introduce new universal keywords for:
 *
 *     opaque
 *     foreign
 *     native
 *     handle
 *     pointer
 *     layout
 *     symbol
 *     representation
 *     language
 *     ABI
 *     vendor
 *     backend
 *
 * An opaque foreign type is represented by the absence of a transparent
 * source-level representation:
 *
 *     extern type NativeHandle;
 *
 * A transparent semantic mapping may be expressed as:
 *
 *     extern type ForeignNumber : Number;
 *
 * Additional interoperability semantics belong to attributes and the
 * dedicated FFI/ABI subsystems.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Foreign type syntax MUST remain target-independent.
 *
 * A foreign type MUST NOT encode universal assumptions about:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     accelerator
 *     register width
 *     pointer width
 *     address width
 *     word size
 *     memory capacity
 *     storage capacity
 *     node count
 *     thread count
 *     core count
 *     device count
 *     qubit count
 *     topology
 *
 * For example:
 *
 *     extern type Handle;
 *
 * is portable source intent.
 *
 * It does NOT mean:
 *
 *     64-bit pointer
 *     32-bit pointer
 *     CPU register
 *     GPU handle
 *     QPU handle
 *
 * until a downstream semantic/ABI/target layer establishes such a mapping.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There are no language-level finite limits on:
 *
 *     foreign types
 *     type parameters
 *     generic nesting
 *     type-expression depth
 *     attributes
 *     declarations
 *     interfaces
 *     implementations
 *     domains
 *     machines
 *     devices
 *     nodes
 *     qubits
 *     memory
 *     tensor dimensions
 *     tensor rank
 *
 * Repetition is represented structurally using ANTLR repetition operators.
 *
 * This grammar contains no:
 *
 *     MAX_FOREIGN_TYPES
 *     MAX_FOREIGN_TYPE_PARAMETERS
 *     MAX_TYPE_DEPTH
 *     MAX_DEVICES
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *
 * Any hostile-input or implementation resource limits belong to explicit
 * compiler/parser policy, not language semantics.
 *
 * ============================================================================
 * OPAQUE TYPE SEMANTICS
 * ============================================================================
 *
 * The canonical opaque form is:
 *
 *     extern type Name;
 *
 * No representation is implied.
 *
 * The compiler MUST NOT infer:
 *
 *     pointer
 *     integer
 *     struct
 *     class
 *     register
 *     handle width
 *     storage layout
 *
 * from the absence of a representation.
 *
 * A representation may be supplied explicitly:
 *
 *     extern type Name : ExistingZamaniType;
 *
 * This is a semantic type relationship.
 *
 * It is NOT an ABI-layout declaration.
 *
 * ============================================================================
 * REPRESENTATION SEMANTICS
 * ============================================================================
 *
 * The optional representation clause:
 *
 *     : typeExpression
 *
 * identifies a source-level semantic representation.
 *
 * It does NOT specify:
 *
 *     byte order;
 *     alignment;
 *     calling convention;
 *     register placement;
 *     stack layout;
 *     object header;
 *     physical address;
 *     memory bank;
 *     hardware register;
 *     ABI encoding.
 *
 * Those belong to later semantic and ABI phases.
 *
 * ============================================================================
 * GENERIC FOREIGN TYPES
 * ============================================================================
 *
 * Generic parameters are source-level semantic parameters.
 *
 * Example:
 *
 *     extern type ForeignBuffer<T>;
 *
 *     extern type ForeignTensor<T, Shape>;
 *
 * Generic parameters do not imply:
 *
 *     fixed rank;
 *     fixed dimension;
 *     fixed memory capacity;
 *     fixed machine word size;
 *     fixed register width.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * A foreign type may represent a semantic object participating in quantum
 * interoperability:
 *
 *     extern type QuantumProgram;
 *     extern type QuantumResult;
 *     extern type ForeignQubit;
 *
 * The grammar does NOT define:
 *
 *     physical qubit identifiers;
 *     coupling maps;
 *     gate sets;
 *     calibration;
 *     pulse schedules;
 *     routing;
 *     scheduling;
 *     QEC;
 *     ZQN;
 *     physical QPU topology.
 *
 * If the type participates in quantum computation, semantic lowering must
 * eventually use the canonical:
 *
 *     quantum::ir
 *
 * boundary.
 *
 * No second quantum IR is permitted.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Foreign types may represent:
 *
 *     HDL interfaces
 *     accelerator handles
 *     hardware descriptors
 *     simulation objects
 *     verification objects
 *     abstract device capabilities
 *
 * The grammar MUST NOT encode:
 *
 *     wire [31:0]
 *     fixed register addresses
 *     fixed pin counts
 *     fixed FPGA capacity
 *     fixed ASIC capacity
 *     fixed accelerator count
 *
 * Hardware-specific representation is resolved downstream.
 *
 * ============================================================================
 * FFI INTEGRATION
 * ============================================================================
 *
 * `foreign-types.g4` defines WHAT a foreign type is.
 *
 * `ffi.g4` defines boundary behavior such as:
 *
 *     marshalling
 *     ownership
 *     borrowing
 *     lifetime
 *     nullability
 *     direction
 *     representation policy
 *     encoding
 *     size
 *     alignment
 *     asynchronous behavior
 *     streaming
 *     callbacks
 *     errors
 *
 * Those rules MUST NOT be copied into this grammar.
 *
 * ============================================================================
 * ABI INTEGRATION
 * ============================================================================
 *
 * ABI layout and calling convention remain outside this grammar.
 *
 * An external type may eventually be lowered through:
 *
 *     source type
 *       |
 *       v
 *     semantic foreign type
 *       |
 *       v
 *     ABI validation
 *       |
 *       v
 *     target representation
 *
 * The parser does not perform any of these later operations.
 *
 * ============================================================================
 * SECURITY / INERTNESS
 * ============================================================================
 *
 * Parsing this grammar MUST be completely inert.
 *
 * It MUST NOT:
 *
 *     open a library;
 *     open a file;
 *     access a URL;
 *     resolve a symbol;
 *     inspect hardware;
 *     inspect environment variables;
 *     execute foreign code;
 *     invoke a process;
 *     allocate native memory;
 *     dereference a pointer;
 *     access physical memory;
 *     authenticate;
 *     authorize;
 *     load a dynamic library.
 *
 * The resulting syntax tree is declarative data only.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     source text;
 *     lexer configuration;
 *     parser grammar;
 *     selected dialect configuration.
 *
 * It MUST NOT depend on:
 *
 *     hardware availability;
 *     filesystem state;
 *     network state;
 *     current time;
 *     randomness;
 *     environment variables;
 *     runtime state;
 *     target selection.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar MUST NOT introduce a second foreign-type AST hierarchy merely
 * to represent syntax.
 *
 * The parser lowers declarations into the existing domain-neutral AST graph.
 *
 * The enclosing:
 *
 *     ExternalDeclaration
 *
 * remains the canonical source-level external declaration container.
 *
 * Its children are represented by canonical NodeId references in the AST graph.
 *
 * The semantic layer is responsible for distinguishing:
 *
 *     opaque foreign type
 *     transparent foreign type
 *     generic foreign type
 *     language-specific foreign type
 *     ABI-specific representation
 *
 * from the structural declaration.
 *
 * If a future dedicated semantic foreign-type node is introduced, that node
 * belongs to the AST/semantic-model layer and MUST NOT be implemented as a
 * second grammar-owned type representation.
 *
 * ============================================================================
 * SOURCE ORDER
 * ============================================================================
 *
 * Foreign type declarations preserve source order.
 *
 * This is required for:
 *
 *     deterministic diagnostics;
 *     deterministic AST traversal;
 *     source reconstruction;
 *     tooling;
 *     reproducible compilation.
 *
 * ============================================================================
 * PUBLIC ENTRY POINTS
 * ============================================================================
 *
 * `interoperabilityForeignTypeDeclaration`
 *
 *     Complete standalone:
 *
 *         extern type Name;
 *
 * `interoperabilityForeignTypeMemberDeclaration`
 *
 *     Member inside an already-open external declaration:
 *
 *         type Name;
 *
 * `interoperabilityForeignTypeParameterClause`
 *
 *     Reusable foreign-type generic parameter syntax.
 *
 * `interoperabilityForeignTypeRepresentation`
 *
 *     Optional semantic representation:
 *
 *         : ExistingType
 *
 * `interoperabilityForeignTypeReference`
 *
 *     Symbolic reference to a foreign type.
 *
 * ============================================================================
 */

parser grammar InteroperabilityForeignTypes;

options {
    tokenVocab = ZamaniLexer;
}

import
    Names,
    Types,
    Attributes
;


/*
 * ============================================================================
 * 1. COMPLETE EXTERNAL FOREIGN-TYPE DECLARATION
 * ============================================================================
 *
 * Example:
 *
 *     extern type NativeHandle;
 *
 *     extern type ForeignBuffer<T>;
 *
 *     extern type ForeignNumber : Number;
 *
 * Attributes are source-level metadata and are owned structurally by the
 * canonical attribute grammar.
 */
interoperabilityForeignTypeDeclaration
    : attribute*
      EXTERN
      TYPE
      identifier
      interoperabilityForeignTypeParameterClause?
      interoperabilityForeignTypeRepresentation?
      SEMICOLON
    ;


/*
 * ============================================================================
 * 2. FOREIGN-TYPE MEMBER DECLARATION
 * ============================================================================
 *
 * Used inside:
 *
 *     extern "..." {
 *         type ...
 *     }
 *
 * The enclosing external declaration owns the source identity.
 */
interoperabilityForeignTypeMemberDeclaration
    : attribute*
      TYPE
      identifier
      interoperabilityForeignTypeParameterClause?
      interoperabilityForeignTypeRepresentation?
      SEMICOLON
    ;


/*
 * ============================================================================
 * 3. FOREIGN-TYPE PARAMETER CLAUSE
 * ============================================================================
 *
 * Generic parameters are semantic parameters, not machine capacities.
 *
 * Examples:
 *
 *     <T>
 *     <T, Shape>
 *     <T: Numeric>
 *     <T, Shape: ShapeConstraint>
 *
 * The parameter count is structurally unbounded.
 */
interoperabilityForeignTypeParameterClause
    : LESS
      interoperabilityForeignTypeParameter
      (
          COMMA
          interoperabilityForeignTypeParameter
      )*
      COMMA?
      GREATER
    ;


/*
 * ============================================================================
 * 4. FOREIGN-TYPE PARAMETER
 * ============================================================================
 *
 * A parameter may optionally have a type-level constraint.
 *
 * The constraint uses the canonical Zamani type grammar.
 *
 * No separate foreign-type constraint language is introduced.
 */
interoperabilityForeignTypeParameter
    : identifier
      (
          COLON
          typeExpression
      )?
    ;


/*
 * ============================================================================
 * 5. OPTIONAL SOURCE-LEVEL REPRESENTATION
 * ============================================================================
 *
 * Example:
 *
 *     extern type ForeignInteger : Int;
 *
 * The representation is semantic type syntax only.
 *
 * It does not define physical layout.
 */
interoperabilityForeignTypeRepresentation
    : COLON
      typeExpression
    ;


/*
 * ============================================================================
 * 6. FOREIGN-TYPE REFERENCE
 * ============================================================================
 *
 * A reference is simply a canonical qualified source-level name.
 *
 * It does not resolve the referenced type.
 */
interoperabilityForeignTypeReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * 7. FOREIGN-TYPE DECLARATION SET
 * ============================================================================
 *
 * Useful for composition grammars and conformance tooling.
 *
 * The repetition is intentionally unbounded by language semantics.
 */
interoperabilityForeignTypeDeclarationSet
    : interoperabilityForeignTypeDeclaration+
    ;


/*
 * ============================================================================
 * 8. FOREIGN-TYPE MEMBER SET
 * ============================================================================
 */
interoperabilityForeignTypeMemberDeclarationSet
    : interoperabilityForeignTypeMemberDeclaration*
    ;


/*
 * ============================================================================
 * 9. TYPE REFERENCE LIST
 * ============================================================================
 *
 * This is a structural helper for tools which need to inspect a sequence of
 * foreign type references.
 *
 * It does not perform name resolution.
 */
interoperabilityForeignTypeReferenceList
    : interoperabilityForeignTypeReference
      (
          COMMA
          interoperabilityForeignTypeReference
      )*
    ;


/*
 * ============================================================================
 * 10. TRANSPARENT / OPAQUE CLASSIFICATION CONTRACT
 * ============================================================================
 *
 * This helper intentionally does not introduce an `opaque` keyword.
 *
 * The semantic distinction is:
 *
 *     no representation
 *         -> opaque foreign type
 *
 *     representation present
 *         -> transparent/represented foreign type
 *
 * This avoids adding a global keyword solely for interoperability.
 */
interoperabilityForeignTypeKind
    : /* opaque: no representation was supplied */
      /* represented: a representation clause was supplied */
    ;


/*
 * ============================================================================
 * COMPLETION CONTRACT
 * ============================================================================
 *
 * OWNED:
 *
 *   - external foreign-type declarations;
 *   - foreign-type members;
 *   - foreign-type generic parameters;
 *   - optional source-level type representation;
 *   - symbolic foreign-type references.
 *
 * NOT OWNED:
 *
 *   - general Zamani types;
 *   - ABI layout;
 *   - calling conventions;
 *   - FFI marshalling;
 *   - ownership checking;
 *   - lifetime checking;
 *   - dynamic loading;
 *   - linker behavior;
 *   - runtime behavior;
 *   - hardware discovery;
 *   - quantum routing;
 *   - quantum scheduling;
 *   - QEC;
 *   - ZQN;
 *   - HAL;
 *   - target selection.
 *
 * AST:
 *
 *   grammar
 *       -> existing domain-neutral AST graph
 *       -> ExternalDeclaration / canonical child declarations
 *       -> semantic foreign-type model
 *
 * IR:
 *
 *   No IR is created by this grammar.
 *
 * Quantum:
 *
 *   Any quantum meaning eventually crosses the canonical quantum::ir boundary.
 *
 * Rust:
 *
 *   Rust 1.97 / 1.97.1
 *   Rust 2021
 *   safe Rust only
 *   no unsafe implementation requirement.
 *
 * Scalability:
 *
 *   no language-level MAX_* constants.
 *
 * Security:
 *
 *   parsing is inert.
 *
 * Determinism:
 *
 *   no environment/hardware/runtime-dependent parsing.
 *
 * ============================================================================
 */
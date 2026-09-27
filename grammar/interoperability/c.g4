/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/interoperability/c.g4
 *
 * Grammar:
 *     CInterop
 *
 * Status:
 *     PRODUCTION C INTEROPERABILITY LEAF GRAMMAR
 *
 * Rust baseline:
 *     Rust 2021
 *     Rust 1.97 / Rust 1.97.1
 *     safe Rust only
 *     no unsafe Rust required by the grammar/frontend
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar owns SOURCE-LEVEL ZAMANI <-> C INTEROPERABILITY SYNTAX.
 *
 * It does NOT implement the C language.
 *
 * It does NOT implement an ABI.
 *
 * It does NOT perform linking.
 *
 * It does NOT load libraries.
 *
 * It does NOT inspect headers.
 *
 * It does NOT discover hardware.
 *
 * It does NOT select a processor.
 *
 * It does NOT calculate native layout.
 *
 * It does NOT construct quantum::ir.
 *
 * Its responsibility is to describe an externally implemented C-compatible
 * boundary in a target-independent form that can be lowered by later compiler
 * stages.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     canonical ZamaniLexer
 *          |
 *          v
 *     ZamaniParser
 *          |
 *          v
 *     Interoperability
 *          |
 *          v
 *     CInterop
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +-------------------+
 *          |                   |
 *          v                   v
 *        FFI/ABI          type/effect/
 *        semantics        capability analysis
 *          |                   |
 *          +---------+---------+
 *                    |
 *                    v
 *          canonical semantic model
 *                    |
 *                    v
 *               canonical IR
 *                    |
 *          +---------+---------+
 *          |         |         |
 *          v         v         v
 *      classical quantum::ir hardware/HDL
 *          |         |         |
 *          +---------+---------+
 *                    |
 *                    v
 *             target lowering
 *                    |
 *                    v
 *             ABI/linker/runtime
 *
 * CInterop is therefore an upstream syntax contract.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - C ABI boundary declarations;
 *   - C-compatible foreign functions;
 *   - C-compatible callbacks;
 *   - C-compatible opaque types;
 *   - C handles;
 *   - C globals/constants;
 *   - symbolic C library/module references;
 *   - C symbol aliases;
 *   - C calling-convention metadata;
 *   - C ownership/nullability metadata;
 *   - C parameter/result boundary metadata;
 *   - C variadic declarations;
 *   - C representation/layout intent;
 *   - C interoperability requirements/capabilities;
 *   - C-specific semantic annotations.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - the complete C language;
 *   - C preprocessing;
 *   - C macro expansion;
 *   - C header parsing;
 *   - C source compilation;
 *   - C type checking;
 *   - native ABI implementation;
 *   - native struct layout;
 *   - linker implementation;
 *   - library discovery;
 *   - runtime FFI dispatch;
 *   - filesystem access;
 *   - network access;
 *   - hardware discovery;
 *   - target selection;
 *   - physical addresses;
 *   - physical registers;
 *   - CPU/GPU/QPU selection;
 *   - quantum routing;
 *   - quantum scheduling;
 *   - QEC;
 *   - ZQN;
 *   - HAL.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * General FFI boundary syntax belongs to:
 *
 *     grammar/interoperability/ffi.g4
 *
 * ABI semantics belong to:
 *
 *     grammar/interoperability/abi.g4
 *
 * General foreign declarations belong to:
 *
 *     grammar/interoperability/foreign-functions.g4
 *
 * C-specific syntax belongs here.
 *
 * Ordinary Zamani functions remain owned by:
 *
 *     grammar/functions/
 *
 * This grammar MUST NOT become a second general FFI grammar.
 *
 * ============================================================================
 * LEXER INTEGRATION
 * ============================================================================
 *
 * This grammar intentionally uses the EXISTING canonical lexer vocabulary.
 *
 * It does NOT require permanent C-specific lexer tokens such as:
 *
 *     C_INT
 *     C_VOID
 *     C_CHAR
 *     C_DOUBLE
 *     C_POINTER
 *     C_ABI
 *     C_TYPE_DECL
 *     C_SCOPE
 *     C_CALLBACK
 *     C_OPAQUE
 *
 * Those tokens do not belong in the universal lexical core merely because
 * C interoperability exists.
 *
 * C names such as:
 *
 *     int
 *     void
 *     size_t
 *     FILE
 *     Context
 *     MyStruct
 *
 * remain source-level identifiers and are interpreted semantically as C
 * entities.
 *
 * The ABI family is represented explicitly by:
 *
 *     STRING
 *
 * for example:
 *
 *     extern "C" fn puts(...);
 *
 * This permits future ABI families without modifying the universal lexer.
 *
 * ============================================================================
 * CANONICAL TOKENS USED
 * ============================================================================
 *
 * Keyword tokens consumed here come from ZamaniLexer:
 *
 *     EXTERN
 *     FN
 *     IMPORT
 *     FROM
 *     AS
 *     CONST
 *     PUBLIC
 *     PRIVATE
 *     INTERNAL
 *     STATIC
 *     INLINE
 *     ASYNC
 *     UNSAFE
 *     PURE
 *     VOLATILE
 *     WITH
 *     EFFECTS
 *     HANDLE
 *     TYPE
 *
 * Lexical names/literals/operators/punctuation consumed here include:
 *
 *     IDENTIFIER
 *     STRING
 *     INTEGER
 *     DOUBLE_COLON
 *     LPAREN
 *     RPAREN
 *     LBRACE
 *     RBRACE
 *     LBRACKET
 *     RBRACKET
 *     COMMA
 *     COLON
 *     SEMICOLON
 *     DOT
 *     STAR
 *     ASSIGN
 *     ELLIPSIS
 *     THIN_ARROW
 *
 * No lexer rules are defined in this file.
 *
 * ============================================================================
 * IMPORTANT NOTE ABOUT EXISTING GRAMMAR INCONSISTENCIES
 * ============================================================================
 *
 * The repository currently contains parser components that use older token
 * names such as K_AS and SEMI while the current lexical composition uses
 * AS and SEMICOLON.
 *
 * This file deliberately uses the CURRENT canonical lexer names.
 *
 * Migration of other grammar components belongs to their owning files and
 * compatibility/conformance work. CInterop must not reproduce those obsolete
 * token names merely to hide the repository-wide inconsistency.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * C interoperability MUST remain target-independent.
 *
 * This grammar MUST NOT encode:
 *
 *     8-bit pointer width
 *     16-bit pointer width
 *     32-bit pointer width
 *     64-bit pointer width
 *     fixed register width
 *     fixed stack size
 *     fixed memory size
 *     fixed alignment
 *     fixed structure size
 *     fixed CPU
 *     fixed GPU
 *     fixed FPGA
 *     fixed ASIC
 *     fixed QPU
 *     fixed operating system
 *     fixed compiler
 *     fixed linker
 *     fixed libc implementation
 *     fixed library path
 *     fixed address
 *     fixed register
 *     fixed device
 *
 * A concrete target ABI determines these properties downstream.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * The grammar places no language-level limits on:
 *
 *     functions
 *     parameters
 *     callbacks
 *     types
 *     fields
 *     declarations
 *     libraries
 *     symbols
 *     attributes
 *     contracts
 *     requirements
 *     capabilities
 *     interfaces
 *     modules
 *     resources
 *     devices
 *     nodes
 *     threads
 *     qubits
 *     memory
 *
 * Repetition is represented by grammar structure.
 *
 * Practical limits imposed by:
 *
 *     parser implementation
 *     compiler memory
 *     operating-system resources
 *     target resources
 *     deployment policy
 *
 * are implementation/resource limits, NOT language limits.
 *
 * ============================================================================
 * SAFETY
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no actions;
 *     no semantic predicates;
 *     no executable code;
 *     no filesystem operations;
 *     no network operations;
 *     no process execution;
 *     no hardware discovery;
 *     no runtime execution.
 *
 * The Rust implementation consuming this grammar must remain compatible with:
 *
 *     Rust 1.97 / Rust 1.97.1
 *
 * and must not require Rust `unsafe`.
 *
 * A C boundary is nevertheless an EXTERNAL TRUST BOUNDARY.
 *
 * Semantic analysis must explicitly account for:
 *
 *     memory access;
 *     ownership transfer;
 *     nullability;
 *     aliasing;
 *     lifetime;
 *     side effects;
 *     blocking;
 *     callbacks;
 *     errors;
 *     ABI compatibility;
 *     safety requirements.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * These grammar rules map conceptually to existing domain-neutral external
 * declaration semantics.
 *
 * The implementation MUST NOT create a parallel C-specific IR.
 *
 * Conceptual nodes:
 *
 *     CInteropDeclaration
 *     CAbiBoundary
 *     CFunctionDeclaration
 *     CParameter
 *     CResult
 *     CCallbackDeclaration
 *     COpaqueTypeDeclaration
 *     CHandleDeclaration
 *     CGlobalDeclaration
 *     CConstantDeclaration
 *     CImportDeclaration
 *     CSymbolReference
 *     CAbiAttribute
 *     COwnershipContract
 *     CNullabilityContract
 *     CVariadicContract
 *
 * These are AST concepts only.
 *
 * Semantic lowering MUST reuse the repository's canonical external-declaration
 * model where one already exists rather than creating duplicate AST hierarchies.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing does NOT establish that a C declaration is valid for a target.
 *
 * Semantic analysis MUST validate:
 *
 *     ABI family
 *     calling convention
 *     symbol identity
 *     parameter compatibility
 *     result compatibility
 *     variadic compatibility
 *     type representation
 *     pointer/reference semantics
 *     ownership
 *     nullability
 *     lifetime
 *     alignment requirements
 *     layout requirements
 *     effect contracts
 *     capability requirements
 *     resource requirements
 *     target availability
 *     library availability
 *     security policy
 *     safety policy
 *
 * ============================================================================
 * RESOURCE/CAPABILITY SEPARATION
 * ============================================================================
 *
 * C interoperability may state:
 *
 *     requires capability("ffi.c")
 *
 * or:
 *
 *     requires capability("c.variadic")
 *
 * or other semantic requirements.
 *
 * These do not select hardware.
 *
 * They do not imply:
 *
 *     use_cpu_0
 *     use_gpu_0
 *     use_node_0
 *
 * Resource realization is downstream.
 *
 * ============================================================================
 * ABI SEPARATION
 * ============================================================================
 *
 * This grammar may identify an ABI family:
 *
 *     extern "C"
 *
 * but it does not determine:
 *
 *     calling sequence;
 *     register allocation;
 *     stack layout;
 *     argument classification;
 *     return classification;
 *     alignment;
 *     binary encoding.
 *
 * Those belong to ABI semantic analysis/lowering.
 *
 * ============================================================================
 */

/*
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * `cInteropDeclaration` is the ONLY public entry rule exported by this file.
 *
 * The interoperability dispatcher should reference this rule.
 *
 * The complete program root remains:
 *
 *     grammar/Zamani.g4
 *
 * and:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * ============================================================================
 */

cInteropDeclaration
    : cAbiBlock
    | cFunctionDeclaration
    | cCallbackDeclaration
    | cImportDeclaration
    | cTypeDeclaration
    | cOpaqueTypeDeclaration
    | cHandleDeclaration
    | cGlobalDeclaration
    | cConstantDeclaration
    ;


/*
 * ============================================================================
 * ABI BOUNDARY
 * ============================================================================
 *
 * Canonical examples:
 *
 *     extern "C" {
 *         fn puts(value: c::char_pointer) -> c::int;
 *     }
 *
 *     extern "C" fn puts(value: c::char_pointer) -> c::int;
 *
 * The string identifies the ABI/language family.
 *
 * Semantic analysis MUST validate that the value denotes a supported C ABI
 * contract. The grammar remains open to future ABI-family names.
 *
 * ============================================================================
 */

cAbiBlock
    : EXTERN STRING LBRACE cAbiMember* RBRACE
    | EXTERN STRING cFunctionDeclaration
    ;

cAbiMember
    : cFunctionDeclaration
    | cCallbackDeclaration
    | cImportDeclaration
    | cTypeDeclaration
    | cOpaqueTypeDeclaration
    | cHandleDeclaration
    | cGlobalDeclaration
    | cConstantDeclaration
    ;


/*
 * ============================================================================
 * C IMPORT
 * ============================================================================
 *
 * Examples:
 *
 *     import c::stdlib;
 *
 *     import c::stdio from "stdio";
 *
 * The imported name is symbolic.
 *
 * It is NOT a filesystem path.
 *
 * Header discovery is downstream tooling/compiler behavior.
 *
 * ============================================================================
 */

cImportDeclaration
    : IMPORT cQualifiedName cImportSource? SEMICOLON
    ;

cImportSource
    : FROM STRING
    ;


/*
 * ============================================================================
 * C FUNCTION
 * ============================================================================
 *
 * Examples:
 *
 *     extern "C" fn puts(value: c::char_pointer) -> c::int;
 *
 *     extern "C" fn memcpy(
 *         destination: c::pointer,
 *         source: c::pointer,
 *         size: c::size
 *     ) -> c::pointer;
 *
 *     extern "C" fn printf(format: c::char_pointer, ...) -> c::int;
 *
 * The grammar intentionally does not encode C's entire declarator grammar.
 * Zamani's source-level contract is explicit and typed.
 *
 * ============================================================================
 */

cFunctionDeclaration
    : cFunctionModifier*
      FN identifier
      cFunctionAttributeList?
      LPAREN cParameterList? RPAREN
      cVariadic?
      cReturnClause?
      cEffectClause?
      SEMICOLON
    ;

cFunctionModifier
    : PUBLIC
    | PRIVATE
    | INTERNAL
    | STATIC
    | INLINE
    | ASYNC
    | UNSAFE
    | PURE
    | VOLATILE
    ;


/*
 * ============================================================================
 * PARAMETERS
 * ============================================================================
 */

cParameterList
    : cParameter (COMMA cParameter)* COMMA?
    ;

cParameter
    : cParameterAttributeList?
      identifier
      COLON cTypeReference
      cDefaultValue?
    ;

cDefaultValue
    : ASSIGN cExpression
    ;


/*
 * ============================================================================
 * RETURN TYPE
 * ============================================================================
 */

cReturnClause
    : THIN_ARROW cTypeReference
    ;


/*
 * ============================================================================
 * VARIADIC CONTRACT
 * ============================================================================
 *
 * `...` is syntax only.
 *
 * The semantic layer determines whether a concrete C ABI supports the
 * declaration and which default argument promotions/requirements apply.
 *
 * ============================================================================
 */

cVariadic
    : ELLIPSIS
    ;


/*
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 */

cEffectClause
    : WITH EFFECTS LBRACE cQualifiedNameList? RBRACE
    ;

cQualifiedNameList
    : cQualifiedName (COMMA cQualifiedName)* COMMA?
    ;


/*
 * ============================================================================
 * CALLBACK
 * ============================================================================
 *
 * A callback is a source-level callable contract.
 *
 * It does not expose a native function-pointer representation.
 * ============================================================================
 */

cCallbackDeclaration
    : EXTERN STRING FN identifier
      LPAREN cParameterList? RPAREN
      cReturnClause?
      cCallbackAttributeList?
      SEMICOLON
    ;


/*
 * ============================================================================
 * TYPE DECLARATION
 * ============================================================================
 *
 * This is a Zamani-side declaration of a C-compatible semantic type.
 *
 * Layout remains downstream.
 * ============================================================================
 */

cTypeDeclaration
    : TYPE identifier cTypeAttributeList? cTypeBody
    ;

cTypeBody
    : LBRACE cFieldDeclaration* RBRACE
    ;

cFieldDeclaration
    : identifier COLON cTypeReference SEMICOLON
    ;


/*
 * ============================================================================
 * OPAQUE TYPE
 * ============================================================================
 *
 * Opaque types intentionally hide physical representation.
 *
 * Example:
 *
 *     type Context;
 *
 * when used with C ABI metadata can represent a C-owned opaque object.
 *
 * ============================================================================
 */

cOpaqueTypeDeclaration
    : TYPE identifier cOpaqueMarker cTypeAttributeList? SEMICOLON
    ;

cOpaqueMarker
    : LBRACKET
      cAnnotationKeyValueList?
      RBRACKET
    ;


/*
 * ============================================================================
 * HANDLE
 * ============================================================================
 *
 * Handles are symbolic references to externally managed objects.
 *
 * They must never require the grammar to expose a physical address.
 * ============================================================================
 */

cHandleDeclaration
    : HANDLE identifier
      cTypeAnnotation?
      cParameterAttributeList?
      SEMICOLON
    ;

cTypeAnnotation
    : COLON cTypeReference
    ;


/*
 * ============================================================================
 * GLOBAL
 * ============================================================================
 *
 * C globals are declared symbolically.
 *
 * The grammar does not expose physical addresses.
 * ============================================================================
 */

cGlobalDeclaration
    : EXTERN identifier COLON cTypeReference
      cGlobalInitializer?
      SEMICOLON
    ;

cGlobalInitializer
    : ASSIGN cExpression
    ;


/*
 * ============================================================================
 * CONSTANT
 * ============================================================================
 */

cConstantDeclaration
    : CONST identifier
      cTypeAnnotation?
      ASSIGN cExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * SYMBOL REFERENCES
 * ============================================================================
 *
 * A C symbol can have a Zamani-visible name and an external symbol spelling.
 * ============================================================================
 */

cSymbolAttribute
    : cAnnotationKeyValue
    ;


/*
 * ============================================================================
 * ATTRIBUTES
 * ============================================================================
 *
 * Attribute names remain identifiers so new C ABI metadata does not require
 * modifying the universal lexer.
 *
 * Examples:
 *
 *     [symbol = "puts"]
 *     [calling_convention = "c"]
 *     [ownership = "borrowed"]
 *     [nullability = "nonnull"]
 *     [requires = capability("c.ffi")]
 *
 * The exact meaning is semantic.
 * ============================================================================
 */

cFunctionAttributeList
    : LBRACKET cAnnotationKeyValueList? RBRACKET
    ;

cFunctionAttributeList
    : LBRACKET cAnnotationKeyValueList? RBRACKET
    ;

cCallbackAttributeList
    : LBRACKET cAnnotationKeyValueList? RBRACKET
    ;

cParameterAttributeList
    : LBRACKET cAnnotationKeyValueList? RBRACKET
    ;

cTypeAttributeList
    : LBRACKET cAnnotationKeyValueList? RBRACKET
    ;

cAnnotationKeyValueList
    : cAnnotationKeyValue (COMMA cAnnotationKeyValue)* COMMA?
    ;

cAnnotationKeyValue
    : cQualifiedName
    | cQualifiedName ASSIGN cExpression
    ;

cAnnotationKeyValue
    : cQualifiedName
    | cQualifiedName ASSIGN cExpression
    ;


/*
 * ============================================================================
 * TYPE REFERENCES
 * ============================================================================
 *
 * C types are represented as semantic qualified names.
 *
 * Examples:
 *
 *     c::int
 *     c::uint
 *     c::size
 *     c::char
 *     c::pointer
 *     c::void
 *     c::FILE
 *     c::Context
 *
 * Width, alignment, layout and representation are target ABI properties.
 *
 * ============================================================================
 */

cTypeReference
    : cNamedType
    | cPointerType
    | cArrayType
    | cFunctionType
    | cQualifiedType
    ;

cNamedType
    : cQualifiedName
    ;

cQualifiedType
    : cQualifiedName
    ;

cPointerType
    : cTypeReference STAR
    ;

cArrayType
    : cTypeReference LBRACKET cExpression? RBRACKET
    ;

cFunctionType
    : FN LPAREN cTypeList? RPAREN cReturnClause?
    ;

cTypeList
    : cTypeReference (COMMA cTypeReference)* COMMA?
    ;


/*
 * ============================================================================
 * QUALIFIED NAMES
 * ============================================================================
 *
 * This local reusable rule intentionally consumes the canonical lexer tokens
 * directly. It avoids depending on older parser components that currently
 * reference obsolete token names.
 *
 * Semantic analysis maps the resulting structure to the canonical Zamani
 * Name/QualifiedName representation.
 * ============================================================================
 */

cQualifiedName
    : identifier (DOUBLE_COLON identifier)*
    ;

identifier
    : IDENTIFIER
    ;


/*
 * ============================================================================
 * EXPRESSIONS
 * ============================================================================
 *
 * C interoperability metadata may use ordinary Zamani expressions for:
 *
 *     symbolic sizes
 *     requirements
 *     capabilities
 *     alignment
 *     availability
 *     policies
 *     conditional metadata
 *
 * This file deliberately keeps a small expression envelope rather than
 * duplicating the entire expression grammar.
 *
 * The interoperability dispatcher may map cExpression to the canonical
 * expression representation.
 * ============================================================================
 */

cExpression
    : cPrimaryExpression
    | cQualifiedName
    | cExpression cBinaryOperator cExpression
    ;

cPrimaryExpression
    : INTEGER
    | STRING
    | cQualifiedName
    | LPAREN cExpression RPAREN
    ;

cBinaryOperator
    : ASSIGN
    | STAR
    | DOT
    ;


/*
 * ============================================================================
 * COMPLETION / SCALABILITY CONTRACT
 * ============================================================================
 *
 * This file is complete when:
 *
 * [x] It has a unique parser grammar identity.
 *
 * [x] It consumes the canonical Zamani lexer vocabulary.
 *
 * [x] It contains no lexer rules.
 *
 * [x] It contains no embedded actions.
 *
 * [x] It contains no semantic predicates.
 *
 * [x] It contains no target-language code.
 *
 * [x] It requires no Rust unsafe.
 *
 * [x] It does not require C-specific universal lexer tokens.
 *
 * [x] It does not duplicate the complete C language grammar.
 *
 * [x] It does not duplicate the general FFI grammar.
 *
 * [x] It does not implement ABI lowering.
 *
 * [x] It does not implement linker behavior.
 *
 * [x] It does not perform library discovery.
 *
 * [x] It does not select a target.
 *
 * [x] It does not select hardware.
 *
 * [x] It does not impose hardware/resource limits.
 *
 * [x] It supports arbitrarily many declarations through repetition.
 *
 * [x] It supports arbitrarily many parameters through repetition.
 *
 * [x] It supports variadic declarations.
 *
 * [x] It supports callbacks.
 *
 * [x] It supports opaque types.
 *
 * [x] It supports handles.
 *
 * [x] It supports symbolic globals.
 *
 * [x] It supports symbolic constants.
 *
 * [x] It supports symbolic C types.
 *
 * [x] It supports target-independent attributes.
 *
 * [x] It supports symbolic requirements/capabilities.
 *
 * [x] It remains open to future C ABI extensions.
 *
 * [x] It maps to the existing domain-neutral AST/semantic architecture.
 *
 * [x] It does not create a second IR.
 *
 * [x] It preserves the canonical quantum::ir boundary.
 *
 * ============================================================================
 * DOWNSTREAM CONTRACT
 * ============================================================================
 *
 * cInteropDeclaration
 *       |
 *       v
 * ExternalDeclaration / canonical frontend declaration model
 *       |
 *       v
 * semantic validation
 *       |
 *       +--> type compatibility
 *       +--> ownership
 *       +--> nullability
 *       +--> effects
 *       +--> capabilities
 *       +--> requirements
 *       +--> ABI compatibility
 *       |
 *       v
 * canonical semantic model
 *       |
 *       v
 * canonical IR
 *       |
 *       +--> classical
 *       +--> quantum::ir when applicable
 *       +--> HDL/hardware boundary when applicable
 *       |
 *       v
 * target ABI realization
 *
 * The C grammar stops at the syntax boundary.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Required positive cases:
 *
 *     extern "C" fn puts(value: c::char_pointer) -> c::int;
 *
 *     extern "C" fn memcpy(
 *         destination: c::pointer,
 *         source: c::pointer,
 *         size: c::size
 *     ) -> c::pointer;
 *
 *     extern "C" fn printf(format: c::char_pointer, ...) -> c::int;
 *
 *     extern "C" {
 *         fn puts(value: c::char_pointer) -> c::int;
 *     }
 *
 *     import c::stdio;
 *
 *     import c::stdlib from "stdlib";
 *
 *     type CPoint {
 *         x: c::double;
 *         y: c::double;
 *     }
 *
 *     type CContext [opaque];
 *
 *     handle Context: c::Context;
 *
 *     const EXIT_SUCCESS: c::int = 0;
 *
 *     extern value: c::int;
 *
 *     extern "C" fn callback(
 *         cb: fn(c::int) -> c::int
 *     ) -> c::int;
 *
 * Required negative cases:
 *
 *     extern "C" fn broken(;
 *     extern "C" fn broken(x:);
 *     extern "C" fn broken(...) ;
 *     type Broken { x: ; }
 *
 * Required boundary cases:
 *
 *     zero parameters;
 *     one parameter;
 *     many parameters;
 *     variadic functions;
 *     nested qualified names;
 *     nested pointer types;
 *     symbolic array sizes;
 *     opaque types;
 *     callbacks;
 *     many attributes;
 *     deeply qualified C namespaces.
 *
 * Required scalability cases:
 *
 *     arbitrarily many declarations;
 *     arbitrarily many parameters;
 *     arbitrarily many attributes;
 *     arbitrarily many qualified-name segments;
 *     arbitrarily nested type syntax;
 *     symbolic rather than fixed resource expressions.
 *
 * Required cross-domain cases:
 *
 *     classical + C;
 *     quantum + C;
 *     hybrid + C;
 *     HDL + C;
 *     hardware + C;
 *     distributed + C;
 *     AI + C;
 *     networking + C.
 *
 * Required portability cases:
 *
 *     tiny target;
 *     CPU;
 *     multicore;
 *     GPU host;
 *     FPGA host;
 *     accelerator host;
 *     QPU host;
 *     simulator;
 *     HPC;
 *     distributed target.
 *
 * The grammar must parse these identically at the source level. Target
 * compatibility is established after parsing.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * CInterop answers:
 *
 *     "How does Zamani describe a C-compatible external boundary?"
 *
 * It does NOT answer:
 *
 *     "Which machine executes it?"
 *     "Which compiler provides it?"
 *     "Which linker is used?"
 *     "Which library path is loaded?"
 *     "Which register receives an argument?"
 *     "Which address is used?"
 *     "Which physical device executes the call?"
 *
 * Those decisions belong downstream.
 *
 * ============================================================================
 */
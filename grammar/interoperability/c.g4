/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/interoperability/c.g4
 *
 * Domain:
 *     C interoperability / C ABI foreign-function interface
 *
 * Architectural role:
 *
 *     Zamani source
 *          |
 *          v
 *     ANTLR lexer/parser
 *          |
 *          v
 *     Zamani AST
 *          |
 *          v
 *     semantic/type/effect/capability analysis
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> classical IR
 *          +--> quantum::ir where a foreign call participates in a
 *          |    quantum/classical program
 *          +--> hardware/resource metadata where explicitly required
 *          |
 *          v
 *     compiler / lowering / linker
 *          |
 *          v
 *     C ABI / platform ABI / runtime
 *
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * This grammar defines Zamani's SOURCE-LEVEL C INTEROPERABILITY CONTRACT.
 *
 * It provides syntax for:
 *
 *   - C ABI declarations;
 *   - C-compatible foreign functions;
 *   - C-compatible foreign symbols;
 *   - C-compatible types as semantic ABI references;
 *   - parameter and result ABI annotations;
 *   - linkage declarations;
 *   - symbol naming;
 *   - calling-convention intent;
 *   - variadic functions;
 *   - C-compatible data declarations;
 *   - opaque C types;
 *   - C handles;
 *   - ownership/lifetime annotations;
 *   - nullability annotations;
 *   - pointer/reference intent;
 *   - callback declarations;
 *   - imported C libraries/modules;
 *   - linker metadata;
 *   - portability requirements;
 *   - ABI compatibility requirements;
 *   - target-independent C interoperability requirements.
 *
 * ============================================================================
 *
 * OWNERSHIP
 * ---------
 *
 * This file owns ONLY syntax for Zamani <-> C interoperability.
 *
 * ============================================================================
 *
 * DOES NOT OWN
 * ------------
 *
 * This file does NOT own:
 *
 *   - the complete C language grammar;
 *   - C preprocessing;
 *   - C macro expansion;
 *   - C header parsing;
 *   - C semantic analysis;
 *   - C type layout calculation;
 *   - sizeof evaluation;
 *   - alignof evaluation;
 *   - struct layout;
 *   - platform ABI implementation;
 *   - calling-convention implementation;
 *   - linker implementation;
 *   - symbol resolution;
 *   - library discovery;
 *   - filesystem access;
 *   - compiler invocation;
 *   - C compiler selection;
 *   - target CPU selection;
 *   - target operating-system selection;
 *   - target pointer width;
 *   - target endianness;
 *   - target register count;
 *   - target memory size;
 *   - target address;
 *   - target device;
 *   - runtime dispatch;
 *   - unsafe Rust implementation.
 *
 * Those concerns belong downstream.
 *
 * ============================================================================
 *
 * POCO-REAF
 * ---------
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * C interoperability MUST NOT turn a portable Zamani program into a
 * machine-specific program.
 *
 * Therefore this grammar does NOT encode:
 *
 *   - 32-bit pointers;
 *   - 64-bit pointers;
 *   - a fixed C ABI implementation;
 *   - a fixed operating system;
 *   - a fixed compiler;
 *   - a fixed linker;
 *   - a fixed architecture;
 *   - a fixed library path;
 *   - a fixed device;
 *   - a fixed address;
 *   - a fixed register;
 *   - a fixed structure size.
 *
 * A C ABI is an interoperability requirement.
 *
 * A concrete ABI implementation is a target/backend concern.
 *
 * ============================================================================
 *
 * IMPORTANT ABI PRINCIPLE
 * -----------------------
 *
 * "C" is a LANGUAGE/ABI FAMILY contract, not a hardware target.
 *
 * For example:
 *
 *     extern "C" fn foo(...)
 *
 * means:
 *
 *     use the C interoperability contract for this symbol.
 *
 * It does NOT mean:
 *
 *     x86
 *     x86_64
 *     ARM
 *     AArch64
 *     RISC-V
 *     Windows
 *     Linux
 *     macOS
 *     a particular libc
 *
 * Target-specific interpretation happens downstream.
 *
 * ============================================================================
 *
 * INTEGRATION CONTRACT
 * --------------------
 *
 * Expected composition dependencies:
 *
 *     core names
 *     core paths
 *     core metadata
 *     core requirements
 *     core capabilities
 *     core constraints
 *     core hints
 *     types
 *     expressions
 *     functions
 *     modules
 *     effects
 *     resources
 *
 * Expected downstream consumers:
 *
 *     frontend AST
 *     semantic analyzer
 *     type checker
 *     ABI checker
 *     capability checker
 *     effect checker
 *     compiler
 *     linker integration
 *     runtime FFI layer
 *     interoperability diagnostics
 *
 * This file MUST NOT import downstream compiler/runtime modules.
 *
 * ============================================================================
 *
 * CIRCULAR-DEPENDENCY RULE
 * ------------------------
 *
 * Forbidden:
 *
 *     c.g4
 *       -> compiler
 *       -> c.g4
 *
 *     c.g4
 *       -> runtime
 *       -> c.g4
 *
 *     c.g4
 *       -> hardware
 *       -> c.g4
 *
 *     c.g4
 *       -> C frontend
 *       -> c.g4
 *
 * The grammar is upstream of all of them.
 *
 * ============================================================================
 *
 * AST CONTRACT
 * ------------
 *
 * The parser should produce AST nodes conceptually equivalent to:
 *
 *     CInteropDeclaration
 *     CFunctionDeclaration
 *     CFunctionParameter
 *     CFunctionResult
 *     CCallbackDeclaration
 *     CForeignTypeDeclaration
 *     COpaqueTypeDeclaration
 *     CHandleDeclaration
 *     CGlobalDeclaration
 *     CConstantDeclaration
 *     CImportDeclaration
 *     CLinkageDeclaration
 *     CAbiRequirement
 *     CAbiAttribute
 *     CSymbolReference
 *     COwnershipAnnotation
 *     CNullabilityAnnotation
 *     CVariadicSpecification
 *
 * These are AST concepts, not IR types.
 *
 * ============================================================================
 *
 * SEMANTIC CONTRACT
 * -----------------
 *
 * The semantic layer MUST validate:
 *
 *   - ABI compatibility;
 *   - parameter compatibility;
 *   - return-value compatibility;
 *   - variadic-call legality;
 *   - pointer/reference compatibility;
 *   - ownership contracts;
 *   - nullability;
 *   - callback compatibility;
 *   - symbol visibility;
 *   - linkage requirements;
 *   - target ABI availability;
 *   - required compiler/runtime capabilities;
 *   - unsupported platform-specific assumptions.
 *
 * Syntax acceptance alone MUST NOT imply that a foreign call is executable.
 *
 * ============================================================================
 *
 * SAFETY
 * ------
 *
 * The Rust implementation of Zamani's compiler/parser MUST NOT require
 * Rust `unsafe`.
 *
 * Foreign C calls are inherently an external trust boundary and MUST be
 * represented explicitly in the semantic model.
 *
 * This grammar therefore permits an explicit interoperability boundary
 * without requiring implementation-side Rust `unsafe`.
 *
 * The semantic checker is responsible for determining whether an invocation
 * requires an unsafe/source-level effect, capability, or explicit trust
 * boundary according to Zamani's safety model.
 *
 * ============================================================================
 */


/* ============================================================================
 * ROOT
 * ========================================================================== */

/*
 * Composition rule:
 *
 *     cInteropDeclaration
 *
 * is imported/referenced by the authoritative interoperability composition
 * grammar.
 *
 * It is intentionally NOT added directly to every domain grammar.
 *
 * The authoritative Zamani composition layer decides where this declaration
 * is legal.
 */
cInteropDeclaration
    : cAbiDeclaration
    | cImportDeclaration
    | cLinkDeclaration
    | cFunctionDeclaration
    | cCallbackDeclaration
    | cTypeDeclaration
    | cOpaqueTypeDeclaration
    | cHandleDeclaration
    | cGlobalDeclaration
    | cConstantDeclaration
    ;


/* ============================================================================
 * C ABI DECLARATION
 * ========================================================================== */

/*
 * Examples:
 *
 *     extern "C" {
 *         fn puts(value: string) -> c.int;
 *     }
 *
 *     extern "C" fn puts(value: string) -> c.int;
 *
 * The grammar does not determine the concrete ABI implementation.
 */
cAbiDeclaration
    : EXTERN C_ABI cAbiBody
    | EXTERN C_ABI cFunctionDeclaration
    | C_ABI cAbiBody
    | C_ABI cFunctionDeclaration
    ;

cAbiBody
    : LBRACE cAbiItem* RBRACE
    ;

cAbiItem
    : cFunctionDeclaration
    | cCallbackDeclaration
    | cTypeDeclaration
    | cOpaqueTypeDeclaration
    | cHandleDeclaration
    | cGlobalDeclaration
    | cConstantDeclaration
    | cLinkageDeclaration
    ;


/* ============================================================================
 * C IMPORTS
 * ========================================================================== */

/*
 * Imports identify an external C interoperability namespace.
 *
 * They do not grant arbitrary filesystem access.
 *
 * Header/source discovery is a compiler/tooling concern.
 */
cImportDeclaration
    : IMPORT C cImportTarget cImportSource? SEMI?
    ;

cImportTarget
    : qualifiedName
    | STAR
    | LBRACE cImportSpecifierList RBRACE
    ;

cImportSpecifierList
    : cImportSpecifier (COMMA cImportSpecifier)* COMMA?
    ;

cImportSpecifier
    : identifier (AS identifier)?
    ;

cImportSource
    : FROM stringLiteral
    ;


/* ============================================================================
 * LINKAGE
 * ========================================================================== */

/*
 * Linkage describes an external symbol relationship.
 *
 * It does NOT itself invoke a linker.
 */
cLinkDeclaration
    : LINK C cLinkBody
    ;

cLinkBody
    : LBRACE cLinkageDeclaration* RBRACE
    ;

cLinkageDeclaration
    : LINKAGE cLinkageKind cLinkageValue? SEMI?
    ;

cLinkageKind
    : 'static'
    | 'dynamic'
    | 'framework'
    | 'system'
    | 'runtime'
    | 'weak'
    | 'strong'
    | qualifiedName
    ;

cLinkageValue
    : expression
    ;


/* ============================================================================
 * C FUNCTION DECLARATIONS
 * ========================================================================== */

/*
 * Examples:
 *
 *     extern "C" fn puts(value: string) -> c.int;
 *
 *     extern "C" fn memcpy(
 *         destination: c.pointer,
 *         source: c.pointer,
 *         size: usize
 *     ) -> c.pointer;
 *
 *     extern "C" fn callback(
 *         function: fn(c.int) -> c.int
 *     ) -> c.int;
 *
 * A declaration does not imply that a function exists on every target.
 *
 * Availability is a semantic/capability/linking question.
 */
cFunctionDeclaration
    : cFunctionModifiers?
      FN identifier
      cFunctionAttributes?
      LPAREN cParameterList? RPAREN
      cVariadicSpecification?
      returnType?
      cFunctionEffectClause?
      SEMI
    ;

cFunctionModifiers
    : cFunctionModifier+
    ;

cFunctionModifier
    : PUBLIC
    | PRIVATE
    | INTERNAL
    | EXTERN
    | INLINE
    | STATIC
    | ASYNC
    | UNSAFE
    | PURE
    | cAbiModifier
    ;

cAbiModifier
    : C_ABI
    | qualifiedName
    ;

cFunctionAttributes
    : LBRACKET cFunctionAttributeList? RBRACKET
    ;

cFunctionAttributeList
    : cFunctionAttribute (COMMA cFunctionAttribute)* COMMA?
    ;

cFunctionAttribute
    : cSymbolAttribute
    | cCallingConventionAttribute
    | cLinkageAttribute
    | cOwnershipAttribute
    | cNullabilityAttribute
    | cAvailabilityAttribute
    | cCapabilityAttribute
    | cRequirementAttribute
    | cHintAttribute
    | cCustomAttribute
    ;

cSymbolAttribute
    : 'symbol' COLON stringLiteral
    ;

cCallingConventionAttribute
    : 'calling_convention' COLON cCallingConvention
    ;

cLinkageAttribute
    : 'linkage' COLON cLinkageKind
    ;

cOwnershipAttribute
    : 'ownership' COLON cOwnershipKind
    ;

cNullabilityAttribute
    : 'nullability' COLON cNullabilityKind
    ;

cAvailabilityAttribute
    : 'availability' COLON expression
    ;

cCapabilityAttribute
    : 'requires_capability' COLON qualifiedName
    ;

cRequirementAttribute
    : 'requires' COLON expression
    ;

cHintAttribute
    : 'hint' COLON expression
    ;

cCustomAttribute
    : qualifiedName
    | qualifiedName ASSIGN expression
    ;


/* ============================================================================
 * C CALLING CONVENTIONS
 * ========================================================================== */

/*
 * These are semantic ABI identifiers.
 *
 * "c" is the portable C ABI family.
 *
 * Other names are permitted as qualified extensions so future platforms do
 * not require modifying this grammar merely to introduce another ABI family.
 */
cCallingConvention
    : 'c'
    | 'system'
    | 'default'
    | qualifiedName
    ;


/* ============================================================================
 * C PARAMETERS
 * ========================================================================== */

cParameterList
    : cParameter (COMMA cParameter)* COMMA?
    ;

cParameter
    : cParameterAttributes?
      parameterPattern
      COLON cTypeReference
      cParameterDefault?
    ;

cParameterAttributes
    : LBRACKET cParameterAttributeList? RBRACKET
    ;

cParameterAttributeList
    : cParameterAttribute (COMMA cParameterAttribute)* COMMA?
    ;

cParameterAttribute
    : cOwnershipAttribute
    | cNullabilityAttribute
    | cPassingAttribute
    | cAlignmentAttribute
    | cCustomAttribute
    ;

cPassingAttribute
    : 'passing' COLON cPassingKind
    ;

cPassingKind
    : 'value'
    | 'pointer'
    | 'reference'
    | 'borrowed'
    | 'owned'
    | 'out'
    | 'inout'
    | qualifiedName
    ;

cAlignmentAttribute
    : 'alignment' COLON expression
    ;

cParameterDefault
    : ASSIGN expression
    ;


/* ============================================================================
 * VARIADIC FUNCTIONS
 * ========================================================================== */

/*
 * Variadicity is a property of the external function contract.
 *
 * The grammar intentionally does not impose an arbitrary argument count.
 */
cVariadicSpecification
    : ELLIPSIS
    | 'variadic'
    | 'variadic' LPAREN cVariadicAttributeList? RPAREN
    ;

cVariadicAttributeList
    : cVariadicAttribute (COMMA cVariadicAttribute)* COMMA?
    ;

cVariadicAttribute
    : 'sentinel' COLON expression
    | 'promotions' COLON qualifiedName
    | 'format' COLON expression
    | 'checked' COLON expression
    | cCustomAttribute
    ;


/* ============================================================================
 * RETURN / EFFECTS
 * ========================================================================== */

cFunctionEffectClause
    : WITH EFFECTS LBRACE cEffectReferenceList? RBRACE
    ;

cEffectReferenceList
    : qualifiedName (COMMA qualifiedName)* COMMA?
    ;


/* ============================================================================
 * CALLBACKS
 * ========================================================================== */

/*
 * A callback is represented as a type-level interoperability contract.
 *
 * No machine-specific function pointer representation is exposed here.
 */
cCallbackDeclaration
    : CALLBACK identifier
      genericParameters?
      LPAREN cParameterList? RPAREN
      returnType?
      cCallbackAttributes?
      SEMI
    ;

cCallbackAttributes
    : LBRACKET cCallbackAttributeList? RBRACKET
    ;

cCallbackAttributeList
    : cCallbackAttribute (COMMA cCallbackAttribute)* COMMA?
    ;

cCallbackAttribute
    : cCallingConventionAttribute
    | cOwnershipAttribute
    | cNullabilityAttribute
    | cCustomAttribute
    ;


/* ============================================================================
 * C TYPES
 * ========================================================================== */

/*
 * IMPORTANT:
 *
 * These are interoperability type references, not a second Zamani type
 * system.
 *
 * Built-in names such as:
 *
 *     c.int
 *     c.uint
 *     c.char
 *     c.pointer
 *
 * are semantic ABI names.
 *
 * Their width/layout/alignment is determined by the selected target ABI.
 */
cTypeReference
    : cBuiltinType
    | cQualifiedType
    | cPointerType
    | cArrayType
    | cFunctionPointerType
    | cOpaqueTypeReference
    | typeExpression
    ;

cBuiltinType
    : C_TYPE
    | C_VOID
    | C_BOOL
    | C_CHAR
    | C_SIGNED_CHAR
    | C_UNSIGNED_CHAR
    | C_SHORT
    | C_UNSIGNED_SHORT
    | C_INT
    | C_UNSIGNED_INT
    | C_LONG
    | C_UNSIGNED_LONG
    | C_LONG_LONG
    | C_UNSIGNED_LONG_LONG
    | C_FLOAT
    | C_DOUBLE
    | C_LONG_DOUBLE
    | C_SIZE
    | C_PTRDIFF
    | C_INTPTR
    | C_UINTPTR
    | C_INTMAX
    | C_UINTMAX
    | C_WCHAR
    | C_WCHAR_T
    | C_NULLPTR
    ;

cQualifiedType
    : C_SCOPE qualifiedName
    ;

cPointerType
    : cTypeReference STAR
    ;

cArrayType
    : cTypeReference LBRACKET expression? RBRACKET
    ;

cFunctionPointerType
    : FN LPAREN cParameterTypeList? RPAREN returnType?
    ;

cParameterTypeList
    : cTypeReference (COMMA cTypeReference)* COMMA?
    ;

cOpaqueTypeReference
    : OPAQUE qualifiedName
    ;


/* ============================================================================
 * C TYPE DECLARATIONS
 * ========================================================================== */

/*
 * Zamani does not calculate C layout in the grammar.
 *
 * The semantic/type-layout subsystem is responsible for validating layout.
 */
cTypeDeclaration
    : cTypeVisibility?
      C_TYPE_DECL
      identifier
      cTypeAttributes?
      cTypeBody
    ;

cTypeVisibility
    : PUBLIC
    | PRIVATE
    | INTERNAL
    ;

cTypeAttributes
    : LBRACKET cTypeAttributeList? RBRACKET
    ;

cTypeAttributeList
    : cTypeAttribute (COMMA cTypeAttribute)* COMMA?
    ;

cTypeAttribute
    : cLayoutAttribute
    | cAbiAttribute
    | cOwnershipAttribute
    | cNullabilityAttribute
    | cAvailabilityAttribute
    | cCustomAttribute
    ;

cLayoutAttribute
    : 'layout' COLON cLayoutKind
    ;

cLayoutKind
    : 'c'
    | 'opaque'
    | 'transparent'
    | qualifiedName
    ;

cAbiAttribute
    : 'abi' COLON cCallingConvention
    ;

cTypeBody
    : LBRACE cTypeMember* RBRACE
    ;

cTypeMember
    : cFieldDeclaration
    | cConstantDeclaration
    ;

cFieldDeclaration
    : identifier COLON cTypeReference SEMI?
    ;

cConstantDeclaration
    : CONST identifier
      (COLON cTypeReference)?
      ASSIGN expression
      SEMI?
    ;


/* ============================================================================
 * OPAQUE TYPES
 * ========================================================================== */

/*
 * Opaque types are essential for POCO-REAF.
 *
 * Zamani source can carry a C handle without knowing its physical layout.
 *
 * Examples:
 *
 *     opaque c.FILE;
 *     opaque c.Context;
 *
 * The compiler/runtime may know the layout for a particular target while
 * portable Zamani source remains layout-independent.
 */
cOpaqueTypeDeclaration
    : OPAQUE cOpaqueTypeName cOpaqueTypeAttributes? SEMI
    ;

cOpaqueTypeName
    : qualifiedName
    ;

cOpaqueTypeAttributes
    : LBRACKET cOpaqueTypeAttributeList? RBRACKET
    ;

cOpaqueTypeAttributeList
    : cOpaqueTypeAttribute (COMMA cOpaqueTypeAttribute)* COMMA?
    ;

cOpaqueTypeAttribute
    : cOwnershipAttribute
    | cNullabilityAttribute
    | cAvailabilityAttribute
    | cCustomAttribute
    ;


/* ============================================================================
 * C HANDLES
 * ========================================================================== */

/*
 * Handles intentionally avoid exposing physical addresses.
 *
 * A handle may ultimately be:
 *
 *     pointer
 *     index
 *     capability
 *     token
 *     opaque runtime object
 *
 * depending on the target/runtime.
 */
cHandleDeclaration
    : HANDLE identifier
      cHandleTarget?
      cHandleAttributes?
      SEMI
    ;

cHandleTarget
    : COLON cTypeReference
    ;

cHandleAttributes
    : LBRACKET cHandleAttributeList? RBRACKET
    ;

cHandleAttributeList
    : cHandleAttribute (COMMA cHandleAttribute)* COMMA?
    ;

cHandleAttribute
    : cOwnershipAttribute
    | cNullabilityAttribute
    | cLifetimeAttribute
    | cCustomAttribute
    ;

cLifetimeAttribute
    : 'lifetime' COLON expression
    ;


/* ============================================================================
 * GLOBALS
 * ========================================================================== */

cGlobalDeclaration
    : cGlobalModifier*
      GLOBAL identifier
      COLON cTypeReference
      cGlobalInitializer?
      SEMI
    ;

cGlobalModifier
    : PUBLIC
    | PRIVATE
    | INTERNAL
    | CONST
    | VOLATILE
    | EXTERN
    ;

cGlobalInitializer
    : ASSIGN expression
    ;


/* ============================================================================
 * C CONSTANTS
 * ========================================================================== */

cConstantDeclaration
    : CONST identifier
      (COLON cTypeReference)?
      ASSIGN expression
      SEMI?
    ;


/* ============================================================================
 * SYMBOL REFERENCES
 * ========================================================================== */

cSymbolReference
    : SYMBOL stringLiteral
    ;


/* ============================================================================
 * OWNERSHIP
 * ========================================================================== */

cOwnershipAttribute
    : OWNERSHIP COLON cOwnershipKind
    ;

cOwnershipKind
    : 'borrowed'
    | 'owned'
    | 'shared'
    | 'transferred'
    | 'returned'
    | 'consumed'
    | 'retained'
    | 'none'
    | qualifiedName
    ;


/* ============================================================================
 * NULLABILITY
 * ========================================================================== */

cNullabilityKind
    : 'nullable'
    | 'nonnull'
    | 'unknown'
    | 'unspecified'
    | qualifiedName
    ;


/* ============================================================================
 * COMMON NULLABILITY ATTRIBUTE
 * ========================================================================== */

cNullabilityAttribute
    : NULLABILITY COLON cNullabilityKind
    ;


/* ============================================================================
 * CUSTOM / EXTENSIBLE ATTRIBUTES
 * ========================================================================== */

/*
 * Extensions must be namespaced.
 *
 * Example:
 *
 *     vendor::extension(...)
 *
 * This prevents the core C interoperability grammar from needing modification
 * every time a platform introduces a new annotation.
 */
cNamespacedAttribute
    : qualifiedName
    ;

cNamespacedAttributeValue
    : cNamespacedAttribute
    | cNamespacedAttribute ASSIGN expression
    ;


/* ============================================================================
 * LEXICAL/COMPOSITION CONTRACT
 * ========================================================================== */

/*
 * The following symbolic terminals/rules are expected from the authoritative
 * Zamani lexer/core composition layer:
 *
 *     EXTERN
 *     C
 *     C_ABI
 *     FN
 *     IMPORT
 *     FROM
 *     LINK
 *     LINKAGE
 *     CALLBACK
 *     OPAQUE
 *     HANDLE
 *     GLOBAL
 *     CONST
 *     OWNERSHIP
 *     NULLABILITY
 *     SYMBOL
 *     C_TYPE
 *     C_VOID
 *     C_BOOL
 *     C_CHAR
 *     C_SIGNED_CHAR
 *     C_UNSIGNED_CHAR
 *     C_SHORT
 *     C_UNSIGNED_SHORT
 *     C_INT
 *     C_UNSIGNED_INT
 *     C_LONG
 *     C_UNSIGNED_LONG
 *     C_LONG_LONG
 *     C_UNSIGNED_LONG_LONG
 *     C_FLOAT
 *     C_DOUBLE
 *     C_LONG_DOUBLE
 *     C_SIZE
 *     C_PTRDIFF
 *     C_INTPTR
 *     C_UINTPTR
 *     C_INTMAX
 *     C_UINTMAX
 *     C_WCHAR
 *     C_WCHAR_T
 *     C_NULLPTR
 *     C_SCOPE
 *     C_TYPE_DECL
 *     AS
 *     WITH
 *     EFFECTS
 *     PUBLIC
 *     PRIVATE
 *     INTERNAL
 *     STATIC
 *     INLINE
 *     ASYNC
 *     UNSAFE
 *     PURE
 *     VOLATILE
 *     LET
 *     VAR
 *     STAR
 *     ELLIPSIS
 *     ASSIGN
 *     COLON
 *     COMMA
 *     SEMI
 *     LPAREN
 *     RPAREN
 *     LBRACE
 *     RBRACE
 *     LBRACKET
 *     RBRACKET
 *
 * Generic Zamani rules expected from the composition layer:
 *
 *     identifier
 *     qualifiedName
 *     stringLiteral
 *     expression
 *     typeExpression
 *     parameterPattern
 *     parameterList
 *     genericParameters
 *     returnType
 *
 * This file deliberately does not redefine those rules.
 *
 * ============================================================================
 */


/* ============================================================================
 * SEMANTIC SCALABILITY CONTRACT
 * ========================================================================== */

/*
 * This grammar contains NO fixed:
 *
 *     function count
 *     parameter count
 *     callback count
 *     type count
 *     library count
 *     symbol count
 *     pointer width
 *     integer width
 *     architecture count
 *     operating-system count
 *     compiler count
 *     device count
 *     resource count
 *     memory size
 *     address size
 *     ABI implementation count
 *
 * Repetition is represented by ANTLR repetition operators:
 *
 *     *
 *     +
 *     ?
 *
 * and by recursive/qualified semantic structures.
 *
 * Any actual parser/resource limit is an implementation/resource constraint,
 * not a language-level scalability limit.
 */


/* ============================================================================
 * HARD-CODING PROHIBITIONS
 * ========================================================================== */

/*
 * Forbidden in this file:
 *
 *     x86
 *     x86_64
 *     arm
 *     aarch64
 *     riscv
 *     windows
 *     linux
 *     darwin
 *     sizeof = fixed number
 *     pointer_width = fixed number
 *     address_width = fixed number
 *     register_count = fixed number
 *     library_count = fixed number
 *     device_id = fixed value
 *     address = fixed value
 *
 * A qualified extension may describe a target-specific semantic requirement,
 * but target selection belongs downstream.
 */


/* ============================================================================
 * SECURITY / TRUST BOUNDARY
 * ========================================================================== */

/*
 * A C declaration MUST be treated as an external trust boundary.
 *
 * The grammar records the boundary.
 *
 * Semantic analysis must determine whether the operation:
 *
 *     reads memory
 *     writes memory
 *     consumes ownership
 *     transfers ownership
 *     may return null
 *     may throw/panic through the ABI
 *     may block
 *     may perform I/O
 *     may mutate global state
 *     may invoke callbacks
 *     may violate language-level purity
 *     requires an explicit unsafe/effect/capability boundary.
 *
 * The grammar MUST NOT silently mark every C function as pure or safe.
 */


/* ============================================================================
 * QUANTUM / CLASSICAL INTEGRATION
 * ========================================================================== */

/*
 * C interoperability may participate in a hybrid program.
 *
 * However:
 *
 *     c.g4
 *
 * does NOT own quantum semantics.
 *
 * If a C function consumes or produces data participating in a quantum
 * computation:
 *
 *     C syntax
 *        |
 *        v
 *     AST
 *        |
 *        v
 *     semantic analysis
 *        |
 *        +--> classical semantics
 *        |
 *        +--> quantum semantic boundary
 *              |
 *              v
 *          quantum::ir
 *
 * The C grammar must never construct quantum operations directly.
 *
 * QEC, ZQN, routing, scheduling, resilience and hardware selection remain
 * downstream.
 */


/* ============================================================================
 * COMPILER INTEGRATION
 * ========================================================================== */

/*
 * Compiler stages consuming this grammar:
 *
 *     1. parse
 *     2. AST construction
 *     3. name resolution
 *     4. foreign-symbol resolution
 *     5. type compatibility
 *     6. ABI compatibility
 *     7. ownership analysis
 *     8. nullability analysis
 *     9. effect analysis
 *    10. capability analysis
 *    11. target availability analysis
 *    12. lowering to canonical semantic representation
 *    13. target-specific ABI lowering
 *    14. linker integration
 *    15. runtime FFI dispatch where applicable
 *
 * This file participates ONLY in stage 1.
 */


/* ============================================================================
 * RUNTIME INTEGRATION
 * ========================================================================== */

/*
 * Runtime must receive already validated foreign-call metadata.
 *
 * Runtime MUST NOT reparse source grammar.
 *
 * Runtime MUST NOT depend directly on this .g4 file.
 *
 * Runtime may consume an ABI-neutral semantic representation containing:
 *
 *     symbol identity
 *     ABI family
 *     calling convention
 *     parameter ABI types
 *     result ABI type
 *     ownership contracts
 *     nullability
 *     effects
 *     capabilities
 *     linkage requirements
 *     target constraints
 *
 * Concrete ABI details are resolved at target lowering/runtime binding time.
 */


/* ============================================================================
 * TOOLING INTEGRATION
 * ========================================================================== */

/*
 * Tooling may use this grammar for:
 *
 *     syntax highlighting
 *     completion
 *     navigation
 *     diagnostics
 *     refactoring
 *     symbol indexing
 *     documentation generation
 *     ABI contract inspection
 *
 * Tooling MUST NOT infer target hardware properties merely from the presence
 * of an `extern "C"` declaration.
 */


/* ============================================================================
 * TESTING CONTRACT
 * ========================================================================== */

/*
 * Positive tests MUST cover:
 *
 *     simple C function
 *     C function with no parameters
 *     C function with many parameters
 *     C function with a result
 *     void result
 *     variadic function
 *     callback
 *     opaque type
 *     handle
 *     global
 *     constant
 *     imported C namespace
 *     symbol rename
 *     linkage metadata
 *     ownership metadata
 *     nullability metadata
 *     namespaced attributes
 *     generic Zamani types used at the FFI boundary
 *
 * Negative tests MUST cover:
 *
 *     missing function name
 *     malformed parameter
 *     malformed return type
 *     malformed ABI declaration
 *     malformed variadic declaration
 *     malformed linkage
 *     malformed ownership
 *     malformed nullability
 *     invalid attribute syntax
 *     malformed callback
 *     malformed opaque type
 *
 * Boundary tests MUST cover:
 *
 *     zero parameters
 *     one parameter
 *     arbitrarily large parameter lists
 *     deeply qualified symbols
 *     deeply nested type expressions
 *     many declarations
 *     many attributes
 *
 * Scalability tests MUST verify that no grammar rule imposes:
 *
 *     maximum C functions
 *     maximum parameters
 *     maximum callbacks
 *     maximum imported symbols
 *     maximum C types
 *     maximum linkage declarations
 *
 * Determinism tests MUST verify identical parse trees for identical source.
 *
 * Round-trip tests SHOULD verify:
 *
 *     source
 *       -> lexer
 *       -> parser
 *       -> AST
 *       -> formatter/serializer
 *       -> parser
 *
 * without semantic loss.
 */


/* ============================================================================
 * COMPATIBILITY CONTRACT
 * ========================================================================== */

/*
 * Existing Zamani forms:
 *
 *     extern ...
 *     foreign ...
 *     foreignFunctionCall
 *     externDecl
 *
 * MUST be migrated through an explicit compatibility layer.
 *
 * This file MUST NOT silently redefine the meaning of an existing source
 * construct.
 *
 * The authoritative compatibility policy belongs to:
 *
 *     grammar/compatibility/
 *
 * and the authoritative composition grammar decides which legacy aliases are
 * accepted during a compatibility window.
 *
 * Eventually the duplicate monolithic FFI rules in Zamani.g4 should lower to
 * these same AST semantics rather than maintain an independent FFI language.
 */


/* ============================================================================
 * COMPLETION CRITERIA
 * ========================================================================== */

/*
 * c.g4 is COMPLETE only when:
 *
 * [ ] C interoperability ownership is explicit.
 * [ ] No complete C grammar is duplicated here.
 * [ ] No hardware limits are encoded.
 * [ ] No pointer width is encoded.
 * [ ] No ABI implementation is hard-coded.
 * [ ] No target operating system is hard-coded.
 * [ ] No compiler is hard-coded.
 * [ ] No linker is hard-coded.
 * [ ] No library path is hard-coded.
 * [ ] No physical address is represented as an implicit requirement.
 * [ ] C declarations have deterministic syntax.
 * [ ] C function declarations are represented.
 * [ ] Variadic functions are represented.
 * [ ] Callbacks are represented.
 * [ ] Opaque types are represented.
 * [ ] Handles are represented.
 * [ ] C-compatible type references are represented.
 * [ ] Linkage is represented.
 * [ ] Symbol identity is represented.
 * [ ] Calling-convention intent is represented.
 * [ ] Ownership is represented.
 * [ ] Nullability is represented.
 * [ ] Capability requirements are representable.
 * [ ] Effects are representable.
 * [ ] Namespaced extensions are representable.
 * [ ] No downstream compiler/runtime dependency exists.
 * [ ] No quantum IR is created.
 * [ ] No QEC/ZQN semantics are created.
 * [ ] No scheduling/routing semantics are created.
 * [ ] No Rust `unsafe` implementation is required.
 * [ ] Existing `extern`/foreign syntax has a documented migration path.
 * [ ] Positive tests exist.
 * [ ] Negative tests exist.
 * [ ] Boundary tests exist.
 * [ ] Determinism tests exist.
 * [ ] Round-trip tests exist where formatter support exists.
 * [ ] Hard-coding audit passes.
 *
 * Only after these conditions are satisfied should the file be considered
 * independently complete.
 */
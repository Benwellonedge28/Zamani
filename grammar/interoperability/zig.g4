/*
 * =============================================================================
 * Zamani — Zig Interoperability Grammar
 * =============================================================================
 *
 * File:
 *     grammar/interoperability/zig.g4
 *
 * Purpose:
 *     Parse the Zig-language surface required to describe interoperable
 *     declarations and foreign boundaries in Zamani.
 *
 * IMPORTANT:
 *     This is NOT a second Zig compiler grammar.
 *
 *     It owns:
 *       - Zig-specific foreign declarations
 *       - Zig ABI/calling-convention syntax
 *       - exported/imported functions
 *       - extern declarations
 *       - Zig-compatible type signatures
 *       - structs/enums/unions required for FFI metadata
 *       - function pointers/callback signatures
 *       - Zig attributes/declaration modifiers relevant to interoperability
 *       - compile-time constants required to describe an ABI
 *       - Zig namespace/path syntax required by foreign declarations
 *
 *     It does NOT own:
 *       - Zamani's canonical AST
 *       - Zamani semantic IR
 *       - quantum::ir
 *       - QEC
 *       - ZQN
 *       - routing
 *       - scheduling
 *       - optimization
 *       - hardware discovery
 *       - resource discovery
 *       - runtime execution
 *       - Zig type checking
 *       - Zig borrow/ownership semantics
 *       - Zig compiler implementation
 *       - machine-specific resource limits
 *
 * Architectural direction:
 *
 *   Zig source
 *       |
 *       v
 *   Zig.g4
 *       |
 *       v
 *   interoperability AST
 *       |
 *       +--> ffi.g4
 *       |
 *       +--> abi.g4
 *       |
 *       +--> foreign-functions.g4
 *       |
 *       v
 *   canonical Zamani semantic representation
 *       |
 *       v
 *   compiler / lowering / backend / runtime
 *
 * POCO-REAF:
 *
 *   The grammar never requires:
 *       - a fixed CPU count
 *       - a fixed GPU count
 *       - a fixed FPGA count
 *       - a fixed number of devices
 *       - a fixed pointer width
 *       - a fixed address width
 *       - a fixed register count
 *       - a fixed vector width
 *       - a fixed memory size
 *       - a fixed node count
 *       - a fixed quantum-device size
 *
 * Such properties belong to target, capability, resource, ABI, hardware,
 * deployment, scheduling, or runtime contexts.
 *
 * Rust implementation requirement:
 *
 *   Rust 1.97 / 1.97.1
 *   unsafe Rust implementation code is prohibited.
 *
 * This grammar contains no target-language actions or semantic predicates.
 * Therefore the generated Rust parser does not require unsafe implementation
 * code from this grammar.
 *
 * Lexer ownership:
 *
 *   The authoritative Zamani lexer owns shared lexical definitions.
 *   This grammar intentionally does not create a second lexer.
 *
 * Expected integration:
 *
 *   tokenVocab=ZamaniLexer
 *
 * The canonical Zamani lexer/token vocabulary must expose the identifiers and
 * literal tokens consumed below. Token-name reconciliation belongs to the
 * lexer-authority migration, not to this interoperability grammar.
 * =============================================================================
 */

parser grammar Zig;

options {
    tokenVocab=ZamaniLexer;
}


/* =============================================================================
 * 1. ROOT
 * ============================================================================= */

zigSource
    : zigAttribute*
      zigContainerMember*
      EOF
    ;

zigContainerMember
    : zigAttribute*
      zigVisibility?
      zigDeclaration
    ;


/* =============================================================================
 * 2. DECLARATIONS
 * ============================================================================= */

zigDeclaration
    : zigImportDeclaration
    | zigUsingNamespaceDeclaration
    | zigExportDeclaration
    | zigExternDeclaration
    | zigFunctionDeclaration
    | zigVariableDeclaration
    | zigConstantDeclaration
    | zigTypeDeclaration
    | zigStructDeclaration
    | zigEnumDeclaration
    | zigUnionDeclaration
    | zigErrorSetDeclaration
    | zigTestDeclaration
    | zigComptimeDeclaration
    | zigContainerBlock
    ;


/* =============================================================================
 * 3. VISIBILITY
 * ============================================================================= */

zigVisibility
    : PUB
    ;


/* =============================================================================
 * 4. IMPORTS / NAMESPACES
 * ============================================================================= */

zigImportDeclaration
    : CONST zigIdentifier ASSIGN zigBuiltinCall SEMI
    ;

zigUsingNamespaceDeclaration
    : USINGNAMESPACE zigExpression SEMI
    ;

zigBuiltinCall
    : BUILTIN_IMPORT
      LPAREN
      zigExpression
      RPAREN
    ;

zigExportDeclaration
    : EXPORT
      (
          zigFunctionDeclaration
        | zigVariableDeclaration
        | zigConstantDeclaration
        | zigContainerDeclaration
      )
    ;

zigContainerDeclaration
    : zigStructDeclaration
    | zigEnumDeclaration
    | zigUnionDeclaration
    | zigErrorSetDeclaration
    ;


/* =============================================================================
 * 5. EXTERN DECLARATIONS
 * ============================================================================= */

zigExternDeclaration
    : EXTERN
      (
          zigFunctionDeclaration
        | zigVariableDeclaration
        | zigConstantDeclaration
        | zigTypeDeclaration
      )
    ;


/* =============================================================================
 * 6. FUNCTIONS
 * ============================================================================= */

zigFunctionDeclaration
    : zigFunctionModifiers*
      FN
      zigIdentifier
      zigParameterList
      zigFunctionReturnType?
      zigFunctionAttributes*
      (
          zigBlock
        | SEMI
      )
    ;

zigFunctionModifiers
    : EXPORT
    | EXTERN
    | INLINE
    | NOINLINE
    | CALLCONV
    | zigCallConv
    ;

zigParameterList
    : LPAREN
      zigParameter*
      RPAREN
    ;

zigParameter
    : zigParameterModifiers*
      zigIdentifier
      COLON
      zigTypeExpression
      zigParameterDefault?
      COMMA
    | zigParameterModifiers*
      zigIdentifier
      COLON
      zigTypeExpression
      zigParameterDefault?
    ;

zigParameterModifiers
    : COMPTIME
    | NOALIAS
    ;

zigParameterDefault
    : ASSIGN zigExpression
    ;

zigFunctionReturnType
    : ARROW
      zigTypeExpression
    ;

zigFunctionAttributes
    : zigCallConv
    | zigLinkSection
    | zigExportSymbol
    | zigNoReturn
    | zigCold
    | zigNeverInline
    | zigAlwaysInline
    ;


/* =============================================================================
 * 7. CALLING CONVENTIONS
 *
 * Calling convention names remain open-ended where the lexer/parser permits
 * them. The semantic ABI layer validates whether a convention is supported.
 * No architecture-specific convention is treated as universally available.
 * ============================================================================= */

zigCallConv
    : CALLCONV
      LPAREN
      zigIdentifier
      RPAREN
    | CALLCONV
      LPAREN
      zigStringLiteral
      RPAREN
    ;

zigLinkSection
    : LINKSECTION
      LPAREN
      zigStringLiteral
      RPAREN
    ;

zigExportSymbol
    : EXPORT_SYMBOL
      LPAREN
      zigStringLiteral
      RPAREN
    ;

zigNoReturn
    : NORETURN
    ;

zigCold
    : COLD
    ;

zigNeverInline
    : NEVER_INLINE
    ;

zigAlwaysInline
    : ALWAYS_INLINE
    ;


/* =============================================================================
 * 8. VARIABLES / CONSTANTS
 * ============================================================================= */

zigVariableDeclaration
    : VAR
      zigIdentifier
      (
          COLON zigTypeExpression
      )?
      (
          ASSIGN zigExpression
      )?
      SEMI
    ;

zigConstantDeclaration
    : CONST
      zigIdentifier
      (
          COLON zigTypeExpression
      )?
      ASSIGN
      zigExpression
      SEMI
    ;


/* =============================================================================
 * 9. TYPES
 * ============================================================================= */

zigTypeDeclaration
    : CONST
      zigIdentifier
      ASSIGN
      zigTypeExpression
      SEMI
    ;

zigTypeExpression
    : zigErrorUnionType
    ;

zigErrorUnionType
    : zigOptionalType
      (
          BANG
          zigOptionalType
      )?
    ;

zigOptionalType
    : QUESTION
      zigOptionalType
    | zigPointerType
    | zigArrayType
    | zigSliceType
    | zigFunctionType
    | zigTupleType
    | zigPrimitiveType
    | zigPathType
    | zigAnyType
    ;

zigAnyType
    : ANYTYPE
    ;

zigPrimitiveType
    : VOID
    | BOOL
    | U8
    | U16
    | U32
    | U64
    | U128
    | I8
    | I16
    | I32
    | I64
    | I128
    | ISIZE
    | USIZE
    | F16
    | F32
    | F64
    | F80
    | F128
    | C_SHORT
    | C_USHORT
    | C_INT
    | C_UINT
    | C_LONG
    | C_ULONG
    | C_LONG_LONG
    | C_ULONG_LONG
    | C_LONG_DOUBLE
    | COMPTIME_INT
    | COMPTIME_FLOAT
    ;

zigPointerType
    : STAR
      zigPointerQualifiers*
      zigTypeExpression
    ;

zigPointerQualifiers
    : CONST
    | VOLATILE
    | ALLOWZERO
    | SENTINEL
      zigExpression
    ;

zigArrayType
    : LBRACKET
      zigArrayLength?
      RBRACKET
      zigTypeExpression
    ;

zigArrayLength
    : zigExpression
    ;

zigSliceType
    : LBRACKET
      RBRACKET
      zigTypeExpression
    ;

zigFunctionType
    : FN
      zigParameterTypeList
      zigFunctionReturnType?
    ;

zigParameterTypeList
    : LPAREN
      zigTypeParameter*
      RPAREN
    ;

zigTypeParameter
    : zigParameterModifiers*
      zigTypeExpression
      COMMA
    | zigParameterModifiers*
      zigTypeExpression
    ;

zigTupleType
    : LPAREN
      zigTypeExpression
      COMMA
      (
          zigTypeExpression
          (
              COMMA
              zigTypeExpression
          )*
          COMMA?
      )?
      RPAREN
    ;

zigPathType
    : zigPath
      zigGenericArguments?
    ;

zigGenericArguments
    : LBRACKET
      zigGenericArgumentList?
      RBRACKET
    ;

zigGenericArgumentList
    : zigGenericArgument
      (
          COMMA
          zigGenericArgument
      )*
      COMMA?
    ;

zigGenericArgument
    : zigTypeExpression
    | zigExpression
    ;


/* =============================================================================
 * 10. STRUCTS
 * ============================================================================= */

zigStructDeclaration
    : STRUCT
      zigIdentifier?
      zigContainerMembersBlock
    ;

zigContainerMembersBlock
    : LBRACE
      zigContainerMember*
      RBRACE
    ;


/* =============================================================================
 * 11. ENUMS
 * ============================================================================= */

zigEnumDeclaration
    : ENUM
      zigEnumBackingType?
      zigEnumBody
    ;

zigEnumBackingType
    : LPAREN
      zigIntegerType
      RPAREN
    ;

zigIntegerType
    : U8
    | U16
    | U32
    | U64
    | U128
    | I8
    | I16
    | I32
    | I64
    | I128
    | ISIZE
    | USIZE
    | zigPathType
    ;

zigEnumBody
    : LBRACE
      zigEnumMember*
      RBRACE
    ;

zigEnumMember
    : zigAttribute*
      zigIdentifier
      zigEnumValue?
      COMMA?
    ;

zigEnumValue
    : ASSIGN
      zigExpression
    ;


/* =============================================================================
 * 12. UNIONS
 * ============================================================================= */

zigUnionDeclaration
    : UNION
      zigUnionQualifier*
      zigUnionBody
    ;

zigUnionQualifier
    : ENUM
    | PACKED
    | EXTERN
    ;

zigUnionBody
    : LBRACE
      zigUnionMember*
      RBRACE
    ;

zigUnionMember
    : zigAttribute*
      zigIdentifier
      (
          COLON
          zigTypeExpression
      )?
      zigUnionFieldValue?
      COMMA?
    ;

zigUnionFieldValue
    : ASSIGN
      zigExpression
    ;


/* =============================================================================
 * 13. ERROR SETS
 * ============================================================================= */

zigErrorSetDeclaration
    : ERROR
      zigIdentifier
      ASSIGN
      LBRACE
      zigErrorMember*
      RBRACE
    ;

zigErrorMember
    : zigIdentifier
      COMMA?
    ;


/* =============================================================================
 * 14. TEST / COMPTIME BOUNDARIES
 *
 * These are accepted because they can occur in Zig declaration contexts and
 * can be relevant when an interoperability declaration contains compile-time
 * layout information.
 *
 * Semantic execution is NOT performed by this grammar.
 * ============================================================================= */

zigTestDeclaration
    : TEST
      (
          zigStringLiteral
      )?
      zigBlock
    ;

zigComptimeDeclaration
    : COMPTIME
      (
          zigExpression
        | zigBlock
      )
    ;


/* =============================================================================
 * 15. BLOCKS
 * ============================================================================= */

zigBlock
    : LBRACE
      zigStatement*
      RBRACE
    ;

zigStatement
    : zigVariableDeclaration
    | zigConstantDeclaration
    | zigExpressionStatement
    | zigReturnStatement
    | zigBlock
    ;

zigExpressionStatement
    : zigExpression
      SEMI
    ;

zigReturnStatement
    : RETURN
      zigExpression?
      SEMI
    ;


/* =============================================================================
 * 16. EXPRESSIONS
 *
 * This is deliberately a bounded interoperability expression grammar.
 * Full Zamani expression semantics remain owned by expressions/*.g4.
 * ============================================================================= */

zigExpression
    : zigAssignmentExpression
    ;

zigAssignmentExpression
    : zigLogicalExpression
      (
          zigAssignmentOperator
          zigAssignmentExpression
      )?
    ;

zigAssignmentOperator
    : ASSIGN
    | PLUS_ASSIGN
    | MINUS_ASSIGN
    | STAR_ASSIGN
    | SLASH_ASSIGN
    | PERCENT_ASSIGN
    | AND_ASSIGN
    | OR_ASSIGN
    | XOR_ASSIGN
    | SHIFT_LEFT_ASSIGN
    | SHIFT_RIGHT_ASSIGN
    ;

zigLogicalExpression
    : zigComparisonExpression
      (
          zigLogicalOperator
          zigComparisonExpression
      )*
    ;

zigLogicalOperator
    : AND_AND
    | OR_OR
    ;

zigComparisonExpression
    : zigBitwiseExpression
      (
          zigComparisonOperator
          zigBitwiseExpression
      )*
    ;

zigComparisonOperator
    : EQUAL
    | NOT_EQUAL
    | LESS
    | LESS_EQUAL
    | GREATER
    | GREATER_EQUAL
    ;

zigBitwiseExpression
    : zigAdditiveExpression
      (
          zigBitwiseOperator
          zigAdditiveExpression
      )*
    ;

zigBitwiseOperator
    : AND
    | OR
    | XOR
    | SHIFT_LEFT
    | SHIFT_RIGHT
    ;

zigAdditiveExpression
    : zigMultiplicativeExpression
      (
          zigAdditiveOperator
          zigMultiplicativeExpression
      )*
    ;

zigAdditiveOperator
    : PLUS
    | MINUS
    ;

zigMultiplicativeExpression
    : zigUnaryExpression
      (
          zigMultiplicativeOperator
          zigUnaryExpression
      )*
    ;

zigMultiplicativeOperator
    : STAR
    | SLASH
    | PERCENT
    ;

zigUnaryExpression
    : zigUnaryOperator
      zigUnaryExpression
    | zigPostfixExpression
    ;

zigUnaryOperator
    : BANG
    | MINUS
    | PLUS
    | TILDE
    | AMPERSAND
    | STAR
    ;

zigPostfixExpression
    : zigPrimaryExpression
      zigPostfixOperation*
    ;

zigPostfixOperation
    : LPAREN
      zigArgumentList?
      RPAREN
    | LBRACKET
      zigExpression
      RBRACKET
    | DOT
      zigIdentifier
    ;

zigPrimaryExpression
    : zigLiteral
    | zigIdentifier
    | zigPath
    | zigBuiltinExpression
    | LPAREN
      zigExpression
      RPAREN
    ;


/* =============================================================================
 * 17. BUILTIN EXPRESSIONS
 *
 * Builtins are represented structurally instead of being interpreted here.
 * This keeps semantic meaning outside the grammar and permits future Zig
 * builtins without making machine/resource limits part of the grammar.
 * ============================================================================= */

zigBuiltinExpression
    : zigBuiltinIdentifier
      LPAREN
      zigArgumentList?
      RPAREN
    ;

zigBuiltinIdentifier
    : BUILTIN_CALL
    | BUILTIN_IMPORT
    | BUILTIN_C_IMPORT
    | BUILTIN_C_DEFINE
    | BUILTIN_C_INCLUDE
    | BUILTIN_TYPEOF
    | BUILTIN_SIZEOF
    | BUILTIN_ALIGNOF
    | BUILTIN_FIELD
    | BUILTIN_FIELD_PARENT
    | BUILTIN_OFFSET_OF
    | BUILTIN_FRAME_ADDRESS
    | BUILTIN_RETURN_ADDRESS
    | BUILTIN_CALLER
    | BUILTIN_TRUNC
    | BUILTIN_INT_CAST
    | BUILTIN_FLOAT_CAST
    | BUILTIN_BIT_CAST
    | BUILTIN_PTR_CAST
    | BUILTIN_INT_TO_PTR
    | BUILTIN_PTR_TO_INT
    | BUILTIN_MEMCPY
    | BUILTIN_MEMSET
    | BUILTIN_BREAKPOINT
    | BUILTIN_PREFETCH
    | BUILTIN_COMPILE_ERROR
    | BUILTIN_COMPILE_LOG
    | BUILTIN_BRANCH_HINT
    | BUILTIN_THIS
    | BUILTIN_FRAME
    | BUILTIN_WORKGROUP
    | BUILTIN_WORKITEM
    ;

zigArgumentList
    : zigExpression
      (
          COMMA
          zigExpression
      )*
      COMMA?
    ;


/* =============================================================================
 * 18. PATHS / NAMES
 * ============================================================================= */

zigPath
    : zigPathSegment
      (
          DOUBLE_COLON
          zigPathSegment
      )*
    ;

zigPathSegment
    : zigIdentifier
    | SELF
    | SUPER
    | ROOT
    ;

zigIdentifier
    : IDENTIFIER
    ;


/* =============================================================================
 * 19. LITERALS
 * ============================================================================= */

zigLiteral
    : zigStringLiteral
    | CHARACTER_LITERAL
    | INTEGER_LITERAL
    | FLOAT_LITERAL
    | BOOL_LITERAL
    | NULL_LITERAL
    | UNDEFINED_LITERAL
    ;

zigStringLiteral
    : STRING_LITERAL
    ;


/* =============================================================================
 * 20. ATTRIBUTES / DECLARATION METADATA
 *
 * Attributes are preserved syntactically for the interoperability AST.
 * Their semantics are validated by ABI/FFI semantic analysis.
 * ============================================================================= */

zigAttribute
    : HASH
      LBRACKET
      zigAttributeName
      zigAttributeArguments?
      RBRACKET
    ;

zigAttributeName
    : zigPath
    ;

zigAttributeArguments
    : LPAREN
      zigArgumentList?
      RPAREN
    ;


/* =============================================================================
 * 21. SENTINEL / ABI-RELEVANT TYPE INFORMATION
 * ============================================================================= */

zigSentinelType
    : LBRACKET
      zigExpression
      COLON
      zigExpression
      RBRACKET
      zigTypeExpression
    ;


/* =============================================================================
 * 22. FOREIGN CALLBACKS
 * ============================================================================= */

zigCallbackType
    : zigFunctionType
    ;

zigCallbackParameter
    : zigTypeExpression
    ;

zigCallbackSignature
    : FN
      LPAREN
      (
          zigCallbackParameter
          (
              COMMA
              zigCallbackParameter
          )*
      )?
      RPAREN
      zigFunctionReturnType?
    ;


/* =============================================================================
 * 23. EXTERN VARIABLE DESCRIPTORS
 * ============================================================================= */

zigExternVariable
    : EXTERN
      VAR
      zigIdentifier
      COLON
      zigTypeExpression
      SEMI
    ;


/* =============================================================================
 * 24. FOREIGN CONSTANT DESCRIPTORS
 * ============================================================================= */

zigExternConstant
    : EXTERN
      CONST
      zigIdentifier
      COLON
      zigTypeExpression
      SEMI
    ;


/* =============================================================================
 * 25. LINKAGE METADATA
 *
 * These declarations carry semantic metadata only. They do not select a
 * physical machine, device, register, address, core, accelerator, or topology.
 * ============================================================================= */

zigLinkageAttribute
    : zigLinkSection
    | zigExportSymbol
    ;
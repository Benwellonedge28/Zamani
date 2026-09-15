/*
 * Zamani — C++ Interoperability Grammar
 *
 * File:
 *     grammar/interoperability/cpp.g4
 *
 * Purpose:
 *     Define Zamani source syntax for interoperating with externally
 *     implemented C++ entities.
 *
 * Architectural boundary:
 *
 *     Zamani syntax
 *          ↓
 *     C++ interoperability parse structure
 *          ↓
 *     semantic validation
 *          ↓
 *     ABI / foreign-function resolution
 *          ↓
 *     compiler / linker / runtime
 *
 * This grammar does NOT implement the C++ language.
 *
 * It does NOT define:
 *   - the C++ parser
 *   - C++ expressions
 *   - C++ template semantics
 *   - overload resolution
 *   - C++ object layout
 *   - C++ ABI rules
 *   - platform calling conventions
 *   - linker behavior
 *   - runtime behavior
 *   - machine/resource limits
 *   - Rust implementation details
 *
 * Machine-specific properties belong to the ABI, target, capability,
 * resource, compiler, and runtime layers.
 *
 * The grammar intentionally contains no embedded target-language code,
 * semantic actions, filesystem access, network access, or unsafe code.
 *
 * Rust implementation requirement:
 *   Rust 1.97 / 1.97.1
 *   Rust 2021
 *   No unsafe Rust.
 */

parser grammar cpp;

import
    abi,
    foreign_functions,
    qualified_names,
    types;

/*
 * --------------------------------------------------------------------------
 * Top-level C++ interoperability declarations
 * --------------------------------------------------------------------------
 *
 * A C++ interoperability declaration is declarative. It tells Zamani how
 * an externally implemented C++ entity is exposed to Zamani.
 *
 * It does not cause the grammar to decide:
 *   - where the implementation lives
 *   - which compiler builds it
 *   - which linker is used
 *   - which ABI implementation is selected
 *   - which physical machine executes it
 */

cppDeclaration
    : cppNamespaceDeclaration
    | cppFunctionDeclaration
    | cppMethodDeclaration
    | cppConstructorDeclaration
    | cppDestructorDeclaration
    | cppVariableDeclaration
    | cppConstantDeclaration
    | cppTypeDeclaration
    | cppOpaqueTypeDeclaration
    | cppEnumDeclaration
    | cppTemplateDeclaration
    | cppCallbackDeclaration
    | cppLibraryDeclaration
    ;

/*
 * --------------------------------------------------------------------------
 * C++ namespace qualification
 * --------------------------------------------------------------------------
 *
 * Namespace names are semantic names, not filesystem paths.
 *
 * A namespace may contain:
 *   foo
 *   foo::bar
 *   std
 *   vendor::library
 *
 * The grammar deliberately does not enumerate namespaces.
 */

cppNamespaceDeclaration
    : 'extern'
      'cpp'
      'namespace'
      cppQualifiedName
      cppDeclarationBlock
    ;

cppDeclarationBlock
    : '{'
      cppDeclaration*
      '}'
    ;

/*
 * --------------------------------------------------------------------------
 * C++ functions
 * --------------------------------------------------------------------------
 *
 * Functions are declared in terms of Zamani-visible signatures.
 *
 * The semantic layer is responsible for determining whether the signature
 * can actually be represented by the selected C++ ABI.
 */

cppFunctionDeclaration
    : 'extern'
      'cpp'
      'fn'
      cppFunctionName
      cppParameterList
      cppReturnClause?
      cppFunctionAttributes*
      ';'
    ;

cppFunctionName
    : identifier
    | cppQualifiedName
    | cppSymbolName
    ;

/*
 * --------------------------------------------------------------------------
 * Member functions
 * --------------------------------------------------------------------------
 *
 * C++ methods are distinct from free functions because their invocation
 * ABI may depend on object representation and ABI rules.
 *
 * The grammar does not attempt to model a platform's object layout.
 */

cppMethodDeclaration
    : 'extern'
      'cpp'
      'method'
      cppQualifiedName
      cppParameterList
      cppReturnClause?
      cppMethodAttributes*
      ';'
    ;

/*
 * --------------------------------------------------------------------------
 * Constructors and destructors
 * --------------------------------------------------------------------------
 *
 * Constructors/destructors are explicitly represented because they are not
 * ordinary free functions in C++ source semantics.
 */

cppConstructorDeclaration
    : 'extern'
      'cpp'
      'constructor'
      cppQualifiedName
      cppParameterList
      cppFunctionAttributes*
      ';'
    ;

cppDestructorDeclaration
    : 'extern'
      'cpp'
      'destructor'
      cppQualifiedName
      cppFunctionAttributes*
      ';'
    ;

/*
 * --------------------------------------------------------------------------
 * Variables
 * --------------------------------------------------------------------------
 */

cppVariableDeclaration
    : 'extern'
      'cpp'
      'var'
      cppQualifiedName
      ':'
      typeExpr
      cppVariableAttributes*
      ';'
    ;

/*
 * --------------------------------------------------------------------------
 * Constants
 * --------------------------------------------------------------------------
 *
 * Constant evaluation is deliberately not performed by the grammar.
 */

cppConstantDeclaration
    : 'extern'
      'cpp'
      'const'
      cppQualifiedName
      ':'
      typeExpr
      cppConstantAttributes*
      ';'
    ;

/*
 * --------------------------------------------------------------------------
 * C++ types
 * --------------------------------------------------------------------------
 *
 * A C++ type declaration describes an externally defined type.
 *
 * The actual representation is resolved by semantic analysis and the
 * selected ABI. No C++ fundamental type widths are hard-coded here.
 */

cppTypeDeclaration
    : 'extern'
      'cpp'
      'type'
      cppQualifiedName
      cppTypeAttributes*
      ';'
    ;

/*
 * Opaque types are especially important for POCO-REAF.
 *
 * Zamani does not need to know the physical representation of an opaque
 * C++ object merely to hold or pass a valid handle to it.
 */

cppOpaqueTypeDeclaration
    : 'extern'
      'cpp'
      'opaque'
      'type'
      cppQualifiedName
      cppTypeAttributes*
      ';'
    ;

/*
 * --------------------------------------------------------------------------
 * Enumerations
 * --------------------------------------------------------------------------
 *
 * Enumeration membership and underlying representation remain subject to
 * C++ ABI validation.
 */

cppEnumDeclaration
    : 'extern'
      'cpp'
      'enum'
      cppQualifiedName
      cppEnumUnderlyingType?
      cppEnumBody?
      cppTypeAttributes*
      ';'
    ;

cppEnumUnderlyingType
    : ':'
      typeExpr
    ;

cppEnumBody
    : '{'
      cppEnumMember
      (',' cppEnumMember)*
      ','?
      '}'
    ;

cppEnumMember
    : identifier
      cppEnumValue?
    ;

cppEnumValue
    : '='
      expression
    ;

/*
 * --------------------------------------------------------------------------
 * Templates
 * --------------------------------------------------------------------------
 *
 * This is intentionally an interoperability declaration, not a complete
 * C++ template grammar.
 *
 * Zamani may refer to an externally provided template interface, while
 * template implementation and instantiation remain C++-side concerns.
 */

cppTemplateDeclaration
    : 'extern'
      'cpp'
      'template'
      cppTemplateParameterList?
      cppTemplateEntity
      cppTemplateAttributes*
      ';'
    ;

cppTemplateParameterList
    : '<'
      cppTemplateParameter
      (',' cppTemplateParameter)*
      '>'
    ;

cppTemplateParameter
    : identifier
    | identifier
      ':'
      typeExpr
    | 'type'
      identifier
    ;

cppTemplateEntity
    : 'fn'
      cppQualifiedName
      cppParameterList
      cppReturnClause?
    | 'type'
      cppQualifiedName
    | 'method'
      cppQualifiedName
      cppParameterList
      cppReturnClause?
    ;

/*
 * --------------------------------------------------------------------------
 * Callback declarations
 * --------------------------------------------------------------------------
 *
 * Callback types allow C++ libraries to call back into Zamani-managed
 * functionality where the selected execution environment permits it.
 *
 * Runtime legality and lifetime are semantic/runtime concerns.
 */

cppCallbackDeclaration
    : 'extern'
      'cpp'
      'callback'
      cppQualifiedName
      cppParameterList
      cppReturnClause?
      cppCallbackAttributes*
      ';'
    ;

/*
 * --------------------------------------------------------------------------
 * Parameter lists
 * --------------------------------------------------------------------------
 */

cppParameterList
    : '('
      cppParameter*
      ')'
    ;

cppParameter
    : cppParameterDeclaration
    | cppVariadicParameter
    ;

cppParameterDeclaration
    : identifier
      ':'
      typeExpr
      cppParameterAttributes*
    ;

cppVariadicParameter
    : '...'
    ;

/*
 * --------------------------------------------------------------------------
 * Return types
 * --------------------------------------------------------------------------
 */

cppReturnClause
    : '->'
      typeExpr
    ;

/*
 * --------------------------------------------------------------------------
 * Libraries
 * --------------------------------------------------------------------------
 *
 * A library declaration expresses a logical external dependency.
 *
 * It MUST NOT be interpreted as permission for the grammar/runtime to
 * access arbitrary filesystem paths or network locations.
 *
 * Resolution belongs to the build/deployment system and its policy.
 */

cppLibraryDeclaration
    : 'extern'
      'cpp'
      'library'
      cppLibraryName
      cppLibraryAttributes*
      ';'
    ;

cppLibraryName
    : stringLiteral
    | cppQualifiedName
    ;

/*
 * --------------------------------------------------------------------------
 * Symbol binding
 * --------------------------------------------------------------------------
 *
 * C++ names may be transformed by compiler-specific name mangling.
 *
 * The source language therefore distinguishes:
 *
 *   logical Zamani name
 *   C++ semantic name
 *   externally linked symbol
 *
 * Resolution occurs after parsing.
 */

cppSymbolBinding
    : 'symbol'
      stringLiteral
    ;

/*
 * --------------------------------------------------------------------------
 * Common C++ attributes
 * --------------------------------------------------------------------------
 *
 * Attributes are declarative metadata.
 *
 * They do not hard-code platform behavior.
 */

cppFunctionAttributes
    : cppSymbolBinding
    | cppAbiAttribute
    | cppCallingConventionAttribute
    | cppLinkageAttribute
    | cppVisibilityAttribute
    | cppExceptionAttribute
    | cppVariadicAttribute
    | cppNoReturnAttribute
    ;

cppMethodAttributes
    : cppSymbolBinding
    | cppAbiAttribute
    | cppCallingConventionAttribute
    | cppLinkageAttribute
    | cppVisibilityAttribute
    | cppExceptionAttribute
    | cppConstMethodAttribute
    | cppReferenceQualifierAttribute
    ;

cppVariableAttributes
    : cppSymbolBinding
    | cppAbiAttribute
    | cppLinkageAttribute
    | cppVisibilityAttribute
    ;

cppConstantAttributes
    : cppSymbolBinding
    | cppAbiAttribute
    | cppLinkageAttribute
    | cppVisibilityAttribute
    ;

cppTypeAttributes
    : cppAbiAttribute
    | cppVisibilityAttribute
    ;

cppCallbackAttributes
    : cppAbiAttribute
    | cppCallingConventionAttribute
    | cppExceptionAttribute
    ;

cppTemplateAttributes
    : cppAbiAttribute
    | cppVisibilityAttribute
    ;

cppLibraryAttributes
    : cppAbiAttribute
    ;

/*
 * --------------------------------------------------------------------------
 * ABI integration
 * --------------------------------------------------------------------------
 *
 * ABI selection is delegated to interoperability/abi.g4.
 *
 * This grammar accepts an ABI identity rather than enumerating all possible
 * ABIs. That is essential for future architectures and platforms.
 */

cppAbiAttribute
    : 'abi'
      qualifiedName
    ;

/*
 * Calling conventions are intentionally open-ended.
 *
 * Examples may include platform-defined conventions, but the grammar must
 * not permanently enumerate them.
 */

cppCallingConventionAttribute
    : 'calling_convention'
      qualifiedName
    ;

/*
 * Linkage identity is semantic metadata.
 */

cppLinkageAttribute
    : 'linkage'
      qualifiedName
    ;

/*
 * Visibility is target/ABI dependent.
 */

cppVisibilityAttribute
    : 'visibility'
      qualifiedName
    ;

/*
 * --------------------------------------------------------------------------
 * Exception boundaries
 * --------------------------------------------------------------------------
 *
 * C++ exceptions must not silently cross a Zamani ABI boundary.
 *
 * The grammar only declares the intended boundary policy. Validation is
 * performed by semantic analysis.
 */

cppExceptionAttribute
    : 'exceptions'
      qualifiedName
    ;

/*
 * --------------------------------------------------------------------------
 * Variadic functions
 * --------------------------------------------------------------------------
 */

cppVariadicAttribute
    : 'variadic'
    ;

/*
 * --------------------------------------------------------------------------
 * Method qualifiers
 * --------------------------------------------------------------------------
 */

cppConstMethodAttribute
    : 'const'
    ;

cppReferenceQualifierAttribute
    : 'ref_qualifier'
      qualifiedName
    ;

/*
 * --------------------------------------------------------------------------
 * No-return semantics
 * --------------------------------------------------------------------------
 */

cppNoReturnAttribute
    : 'noreturn'
    ;

/*
 * --------------------------------------------------------------------------
 * C++ qualified names
 * --------------------------------------------------------------------------
 *
 * C++ namespace qualification uses :: semantics.
 *
 * This is kept separate from ordinary Zamani filesystem/module paths.
 */

cppQualifiedName
    : cppNameSegment
      ('::' cppNameSegment)*
    ;

cppNameSegment
    : identifier
    ;

/*
 * A symbol name is kept as a string when the external linker identity
 * cannot safely be represented as a Zamani identifier.
 */

cppSymbolName
    : stringLiteral
    ;

/*
 * --------------------------------------------------------------------------
 * Shared grammar contracts
 * --------------------------------------------------------------------------
 *
 * These rules intentionally delegate to canonical Zamani grammar rules.
 *
 * No duplicate expression or type grammar is created here.
 */

identifier
    : IdentifierToken
    ;

qualifiedName
    : identifier
      ('::' identifier | '.' identifier)*
    ;

typeExpr
    : TypeExpression
    ;

expression
    : Expression
    ;

stringLiteral
    : StringLiteral
    ;

/*
 * --------------------------------------------------------------------------
 * Imported canonical token/rule contracts
 * --------------------------------------------------------------------------
 *
 * The repository's authoritative lexer/parser layer must provide:
 *
 *   IdentifierToken
 *   TypeExpression
 *   Expression
 *   StringLiteral
 *
 * These are contracts, not local token definitions.
 *
 * The final Zamani grammar integration must map these references to the
 * repository's actual canonical lexer/parser rule names.
 */

IdentifierToken
    : IdentifierToken
    ;

TypeExpression
    : TypeExpression
    ;

Expression
    : Expression
    ;

StringLiteral
    : StringLiteral
    ;
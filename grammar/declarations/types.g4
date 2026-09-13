/*
 * ============================================================================
 * Zamani Universal Computing Language
 * File: grammar/declarations/types.g4
 *
 * PURPOSE
 * -------
 * Defines SOURCE-LEVEL TYPE DECLARATIONS.
 *
 * This file owns declarations which introduce, alias, or define named types.
 *
 * It does NOT own the complete Zamani type system.
 *
 * The complete type-expression system belongs to:
 *
 *     grammar/types/
 *
 * In particular, this file does NOT define:
 *
 *   - primitive type syntax
 *   - integer widths
 *   - floating-point widths
 *   - vector widths
 *   - tensor dimensions
 *   - quantum-machine sizes
 *   - qubit counts
 *   - hardware capacities
 *   - memory capacities
 *   - CPU/GPU counts
 *   - topology
 *   - device identifiers
 *   - physical addresses
 *   - target-specific types
 *   - type-checking
 *   - type inference
 *   - generic constraint solving
 *   - ABI lowering
 *   - IR construction
 *
 * ARCHITECTURAL PRINCIPLE
 * -----------------------
 *
 *     syntax
 *        |
 *        v
 *     declaration AST
 *        |
 *        v
 *     semantic type analysis
 *        |
 *        v
 *     canonical semantic/IR representation
 *        |
 *        +-------------------------------+
 *        |                               |
 *        v                               v
 *     classical IR                  quantum::ir
 *                                        |
 *                                        v
 *                              QEC / ZQN / optimization
 *                                        |
 *                                        v
 *                              routing / scheduling / HAL
 *                                        |
 *                                        v
 *                                     runtime
 *
 * The grammar MUST NOT depend on downstream IR, hardware, scheduling,
 * optimization, QEC, ZQN, or runtime implementations.
 *
 * POCO-REAF
 * ---------
 *
 * Type declarations describe semantic type relationships.
 *
 * They MUST NOT encode accidental machine limits.
 *
 * Therefore this grammar contains no fixed:
 *
 *   - maximum type size
 *   - maximum generic parameter count
 *   - maximum fields
 *   - maximum variants
 *   - maximum nesting depth
 *   - maximum dimensions
 *   - maximum qubits
 *   - maximum hardware resources
 *   - maximum nodes
 *   - maximum devices
 *
 * Actual parser/compiler resource exhaustion is an implementation/runtime
 * concern, not a language-level grammar limit.
 *
 * OWNERSHIP
 * ---------
 *
 * OWNED:
 *
 *   - named type declarations
 *   - type aliases
 *   - type definitions
 *   - declaration-level generic parameters
 *   - declaration-level type bounds
 *   - declaration-level visibility/modifier integration
 *   - declaration-level attributes
 *
 * NOT OWNED:
 *
 *   - type expressions
 *   - primitive types
 *   - composite type expressions
 *   - function types
 *   - generic type expressions
 *   - tuple types
 *   - arrays
 *   - maps
 *   - options
 *   - results
 *   - references
 *   - resource types
 *   - quantum types
 *   - classical types
 *   - hardware types
 *   - type inference
 *   - type compatibility
 *   - type layout
 *   - ABI representation
 *   - machine representation
 *
 * Those concerns belong to grammar/types/ and later semantic/compiler layers.
 *
 * DECLARATION FAMILY
 * ------------------
 *
 * This file provides the declaration-level type abstraction used by:
 *
 *   declarations.g4
 *   modules/*
 *   functions/*
 *   statements/*
 *   classical/*
 *   quantum/*
 *   hybrid/*
 *   hdl/*
 *   hardware/*
 *   distributed/*
 *   ai/*
 *   data/*
 *
 * Domain-specific types may be introduced by their respective domains,
 * but must ultimately integrate with the common semantic type system.
 *
 * Rust integration
 * ----------------
 *
 * This grammar is language/runtime neutral.
 *
 * Zamani's Rust implementation MUST:
 *
 *   - target Rust 1.97 or Rust 1.97.1
 *   - use safe Rust
 *   - contain no unsafe blocks
 *   - contain no unsafe functions
 *   - preserve source spans
 *   - distinguish syntax errors from semantic errors
 *   - preserve deterministic parse structure
 *   - never introduce machine-size assumptions into parsing
 *
 * No Rust actions are embedded in this grammar.
 *
 * This keeps the grammar portable and allows generated parser artifacts to
 * remain implementation details.
 *
 * ============================================================================
 */

parser grammar ZamaniDeclarationTypesParser;

options {
    /*
     * Canonical lexer vocabulary.
     *
     * ZamaniLexer is the single lexical authority.
     *
     * This grammar must never introduce a second lexer vocabulary.
     */
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * ROOT TYPE DECLARATION
 * ============================================================================
 *
 * A type declaration introduces a named semantic type.
 *
 * Examples:
 *
 *     type Scalar = f64;
 *
 *     type QubitRegister = quantum<Qubit>;
 *
 *     type DeviceState = SomeExistingType;
 *
 *     type Matrix<T> = matrix<T>;
 *
 *     public type Scalar = f64;
 *
 *     type Result<T, E> = result<T, E>;
 *
 * The grammar accepts the declaration shape.
 *
 * Semantic analysis determines whether the referenced type actually exists,
 * whether it is legal, whether constraints are satisfied, and whether the
 * declaration is valid in its context.
 */

typeDeclaration
    : typeDeclarationPrefix*
      TYPE
      IDENTIFIER
      typeDeclarationGenerics?
      typeDeclarationBounds?
      typeDefinition
    ;


/*
 * ============================================================================
 * DECLARATION PREFIX
 * ============================================================================
 *
 * Prefixes are syntactic declaration modifiers/attributes.
 *
 * Their meaning is resolved semantically.
 *
 * IMPORTANT:
 *
 * This file does not define arbitrary modifier semantics.
 * The canonical declaration-prefix layer is responsible for the complete
 * modifier/visibility model.
 *
 * These alternatives intentionally remain narrow.
 */

typeDeclarationPrefix
    : PUBLIC
    | PRIVATE
    | PROTECTED
    | INTERNAL
    | ABSTRACT
    | SEALED
    | FINAL
    | PARTIAL
    | annotation
    ;


/*
 * ============================================================================
 * TYPE GENERIC PARAMETERS
 * ============================================================================
 *
 * Generic parameter count is unbounded by language design.
 *
 * Any practical limits are implementation/resource constraints rather than
 * grammar restrictions.
 *
 * Examples:
 *
 *     type Box<T> = ...
 *
 *     type Pair<A, B> = ...
 *
 *     type Tensor<T, Rank, Layout> = ...
 *
 *     type QState<T, Encoding, Space> = ...
 */

typeDeclarationGenerics
    : LESS_THAN
      typeDeclarationGenericParameter
      (
          COMMA
          typeDeclarationGenericParameter
      )*
      GREATER_THAN
    ;


typeDeclarationGenericParameter
    : IDENTIFIER
      typeGenericParameterBound?
      typeGenericParameterDefault?
    ;


/*
 * ============================================================================
 * GENERIC PARAMETER BOUNDS
 * ============================================================================
 *
 * Bounds are syntactic constraints.
 *
 * Their semantic interpretation belongs to the type system.
 */

typeGenericParameterBound
    : COLON
      typeBoundList
    ;


typeBoundList
    : typeExpression
      (
          PLUS
          typeExpression
      )*
    ;


/*
 * ============================================================================
 * GENERIC PARAMETER DEFAULTS
 * ============================================================================
 *
 * Defaults are optional source-level type expressions.
 *
 * They do not imply a machine representation.
 */

typeGenericParameterDefault
    : EQUALS
      typeExpression
    ;


/*
 * ============================================================================
 * DECLARATION-LEVEL WHERE/BOUND CLAUSE
 * ============================================================================
 *
 * This provides a scalable location for additional generic constraints
 * without embedding constraint semantics into the grammar.
 *
 * Example:
 *
 *     type Matrix<T>
 *       where T: Numeric
 *       = ...
 *
 * The semantic constraint system decides whether a bound is satisfiable.
 */

typeDeclarationBounds
    : WHERE
      typeDeclarationConstraint
      (
          COMMA
          typeDeclarationConstraint
      )*
    ;


typeDeclarationConstraint
    : typeParameterReference
      COLON
      typeBoundList
    ;


typeParameterReference
    : IDENTIFIER
    ;


/*
 * ============================================================================
 * TYPE DEFINITION
 * ============================================================================
 *
 * A named type must have an explicit definition.
 *
 * This prevents declarations such as:
 *
 *     type Foo;
 *
 * from silently introducing incomplete types.
 *
 * Incomplete/opaque/extern type declarations, if required by Zamani, must
 * receive an explicitly owned grammar in a separate declaration module rather
 * than weakening this rule.
 */

typeDefinition
    : EQUALS
      typeExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * TYPE ALIAS
 * ============================================================================
 *
 * Alias syntax is retained as an explicit named declaration form.
 *
 * This is intentionally separate from structural type definitions.
 *
 * Example:
 *
 *     alias Word = integer;
 *
 *     alias QubitRef = quantum<Qubit>;
 *
 * If Zamani ultimately chooses `type` as the sole alias keyword, the semantic
 * compatibility layer may map legacy `alias` syntax to typeDeclaration.
 *
 * That decision belongs to the language compatibility specification.
 */

typeAliasDeclaration
    : typeAliasPrefix*
      ALIAS
      IDENTIFIER
      typeDeclarationGenerics?
      typeDeclarationBounds?
      EQUALS
      typeExpression
      SEMICOLON
    ;


typeAliasPrefix
    : PUBLIC
    | PRIVATE
    | PROTECTED
    | INTERNAL
    | annotation
    ;


/*
 * ============================================================================
 * TYPE DECLARATION ITEM
 * ============================================================================
 *
 * declarations.g4 imports this rule as the type-declaration family.
 *
 * Keeping this family explicit prevents declarations.g4 from duplicating the
 * syntax of named types and aliases.
 */

typeDeclarationItem
    : typeDeclaration
    | typeAliasDeclaration
    ;


/*
 * ============================================================================
 * TYPE EXPRESSION INTEGRATION BOUNDARY
 * ============================================================================
 *
 * IMPORTANT:
 *
 * This is NOT the authoritative implementation of Zamani's type system.
 *
 * The authoritative type-expression grammar lives under:
 *
 *     grammar/types/
 *
 * This rule is an integration contract.
 *
 * In the final composed parser, declarations.g4 should import/use the
 * authoritative typeExpression rule rather than maintaining a second
 * implementation here.
 *
 * The forwarding alternatives below are deliberately minimal and are intended
 * to disappear from the composed parser once the imported type grammar is
 * authoritative.
 *
 * They are retained here as an explicit contract so this file documents the
 * exact dependency expected by the declaration layer.
 */

typeExpression
    : namedTypeExpression
    | genericTypeExpression
    | parenthesizedTypeExpression
    | tupleTypeExpression
    | arrayTypeExpression
    | referenceTypeExpression
    | functionTypeExpression
    ;


/*
 * ============================================================================
 * NAMED TYPE
 * ============================================================================
 */

namedTypeExpression
    : qualifiedTypeName
    ;


qualifiedTypeName
    : IDENTIFIER
      (
          NAMESPACE_SEPARATOR
          IDENTIFIER
      )*
    ;


/*
 * ============================================================================
 * GENERIC TYPE APPLICATION
 * ============================================================================
 *
 * Generic arity is not hard-coded.
 *
 * Examples:
 *
 *     vector<f64>
 *     matrix<f64>
 *     tensor<f64, Rank>
 *     quantum<Qubit>
 *     result<Value, Error>
 */

genericTypeExpression
    : qualifiedTypeName
      LESS_THAN
      typeArgumentList
      GREATER_THAN
    ;


typeArgumentList
    : typeExpression
      (
          COMMA
          typeExpression
      )*
    ;


/*
 * ============================================================================
 * PARENTHESIZED TYPE
 * ============================================================================
 */

parenthesizedTypeExpression
    : LEFT_PAREN
      typeExpression
      RIGHT_PAREN
    ;


/*
 * ============================================================================
 * TUPLE TYPE
 * ============================================================================
 *
 * Tuple cardinality is not bounded by the grammar.
 */

tupleTypeExpression
    : LEFT_PAREN
      typeExpression
      COMMA
      (
          typeExpression
          (
              COMMA
              typeExpression
          )*
      )?
      RIGHT_PAREN
    ;


/*
 * ============================================================================
 * ARRAY TYPE
 * ============================================================================
 *
 * Array extent is expressed semantically.
 *
 * The grammar MUST NOT impose a maximum dimension or extent.
 *
 * Examples:
 *
 *     array<T>
 *
 *     array<T, N>
 *
 *     array<T, dynamic>
 *
 *     array<T, shape>
 *
 * The complete array-type syntax belongs to grammar/types/array-types.g4.
 *
 * This forwarding form is intentionally generic.
 */

arrayTypeExpression
    : ARRAY
      LESS_THAN
      typeExpression
      (
          COMMA
          typeExpression
      )*
      GREATER_THAN
    ;


/*
 * ============================================================================
 * REFERENCE TYPE
 * ============================================================================
 *
 * Reference ownership semantics are handled by the memory/type semantic
 * system, not by this declaration grammar.
 */

referenceTypeExpression
    : REFERENCE
      LESS_THAN
      typeExpression
      GREATER_THAN
    ;


/*
 * ============================================================================
 * FUNCTION TYPE
 * ============================================================================
 *
 * Function type syntax is ultimately owned by:
 *
 *     grammar/types/function-types.g4
 *
 * This integration rule prevents declaration syntax from becoming coupled
 * to function implementation details.
 */

functionTypeExpression
    : FN
      LEFT_PAREN
      functionTypeParameterList?
      RIGHT_PAREN
      functionTypeReturn?
    ;


functionTypeParameterList
    : typeExpression
      (
          COMMA
          typeExpression
      )*
    ;


functionTypeReturn
    : ARROW
      typeExpression
    ;


/*
 * ============================================================================
 * ANNOTATION INTEGRATION
 * ============================================================================
 *
 * Annotation syntax is owned by:
 *
 *     grammar/core/annotations.g4
 *
 * This declaration grammar only consumes the annotation abstraction.
 */

annotation
    : AT
      IDENTIFIER
      annotationArguments?
    ;


annotationArguments
    : LEFT_PAREN
      annotationArgumentList?
      RIGHT_PAREN
    ;


annotationArgumentList
    : annotationArgument
      (
          COMMA
          annotationArgument
      )*
    ;


annotationArgument
    : IDENTIFIER
    | literal
    | annotation
    ;


/*
 * ============================================================================
 * LITERAL INTEGRATION
 * ============================================================================
 *
 * Literal syntax is owned by lexer/literals.g4 and the expression grammar.
 *
 * This forwarding rule exists only as an explicit declaration dependency.
 */

literal
    : INTEGER_LITERAL
    | DECIMAL_LITERAL
    | STRING_LITERAL
    | CHARACTER_LITERAL
    | BOOLEAN_LITERAL
    | QUANTUM_LITERAL
    | HARDWARE_LITERAL
    | DURATION_LITERAL
    | SIZE_LITERAL
    ;


/*
 * ============================================================================
 * DECLARATION-SCOPE INTEGRATION CONTRACT
 * ============================================================================
 *
 * declarations.g4 MUST import/use:
 *
 *     typeDeclarationItem
 *
 * It MUST NOT redefine:
 *
 *     typeDeclaration
 *     typeAliasDeclaration
 *     typeDeclarationGenerics
 *     typeDeclarationBounds
 *
 * Other declaration modules may consume typeExpression, but the canonical
 * definition must ultimately come from grammar/types/.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing succeeds when the source has valid declaration syntax.
 *
 * Semantic analysis MUST subsequently validate:
 *
 *   - duplicate type names
 *   - namespace collisions
 *   - unknown referenced types
 *   - generic parameter validity
 *   - duplicate generic parameters
 *   - illegal recursive definitions
 *   - invalid bounds
 *   - unsatisfied constraints
 *   - alias cycles
 *   - incompatible type applications
 *   - invalid domain-specific types
 *   - visibility rules
 *   - module/package ownership
 *
 * None of these are parser responsibilities.
 *
 * ============================================================================
 * RECURSION / SELF-REFERENCE
 * ============================================================================
 *
 * Recursive types are not rejected by syntax.
 *
 * Semantic analysis determines whether a recursive definition is legal.
 *
 * This is essential for:
 *
 *   - linked structures
 *   - trees
 *   - graphs
 *   - recursive algebraic types
 *   - distributed protocols
 *   - symbolic structures
 *   - future language constructs
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar deliberately contains no fixed:
 *
 *     MAX_FIELDS
 *     MAX_VARIANTS
 *     MAX_GENERIC_PARAMETERS
 *     MAX_TYPE_DEPTH
 *     MAX_DIMENSIONS
 *     MAX_TUPLE_SIZE
 *     MAX_ALIAS_DEPTH
 *     MAX_TYPE_SIZE
 *
 * The language specification therefore does not impose artificial machine
 * limits through grammar syntax.
 *
 * Practical parser/compiler limits are implementation/resource limits and
 * MUST be reported as resource failures rather than language semantics.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * This file does NOT define quantum types.
 *
 * Quantum types belong to:
 *
 *     grammar/types/quantum-types.g4
 *
 * Quantum declarations may reference those types through typeExpression.
 *
 * Semantic lowering ultimately maps valid quantum source constructs toward:
 *
 *     quantum::ir
 *
 * This file MUST NOT construct or duplicate quantum IR.
 *
 * No quantum machine size is encoded here.
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Classical scalar/vector/matrix/tensor types are defined by the common
 * type system and classical grammar layers.
 *
 * This declaration grammar only names them.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Hardware-specific types may be introduced by:
 *
 *     grammar/types/hardware-types.g4
 *     grammar/hdl/
 *     grammar/hardware/
 *
 * A declaration such as a hardware-oriented type remains a semantic type
 * reference at this layer.
 *
 * This file MUST NOT encode:
 *
 *     fixed FPGA resource counts
 *     fixed ASIC dimensions
 *     fixed bus widths
 *     fixed device addresses
 *     fixed clock counts
 *     fixed topology
 *
 * Such properties belong to hardware/resource/capability descriptions.
 *
 * ============================================================================
 * POCO-REAF INTEGRATION
 * ============================================================================
 *
 * A type declaration is portable when its semantic meaning does not depend
 * on a particular machine.
 *
 * Example:
 *
 *     type Data = array<Real>;
 *
 * does not imply:
 *
 *     N elements
 *     N bytes
 *     N cores
 *     N devices
 *
 * Those decisions are made later by compilation, resource selection,
 * scheduling, placement, or runtime adaptation.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend AST should preserve at minimum:
 *
 *   TypeDeclaration
 *     - declaration identity/name
 *     - source span
 *     - visibility/modifiers
 *     - annotations
 *     - generic parameters
 *     - generic bounds
 *     - declared type expression
 *
 *   TypeAliasDeclaration
 *     - declaration identity/name
 *     - source span
 *     - visibility/modifiers
 *     - annotations
 *     - generic parameters
 *     - generic bounds
 *     - target type expression
 *
 * The grammar MUST NOT introduce a separate runtime type representation.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar does NOT directly produce:
 *
 *     classical IR
 *     quantum::ir
 *     hardware IR
 *     scheduling IR
 *     runtime representation
 *
 * Semantic lowering owns those transformations.
 *
 * ============================================================================
 * COMPILER CONTRACT
 * ============================================================================
 *
 * Compiler stages may use the semantic type information produced from these
 * declarations to:
 *
 *   - resolve names
 *   - infer/check types
 *   - instantiate generics
 *   - validate constraints
 *   - lower domain-specific types
 *   - select legal representations
 *   - generate target-specific code
 *
 * No target-specific decision belongs in this grammar.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime does not depend directly on this grammar.
 *
 * Runtime consumes compiled semantic/IR artifacts.
 *
 * This prevents:
 *
 *     grammar -> runtime
 *
 * dependency cycles.
 *
 * ============================================================================
 * TOOLING CONTRACT
 * ============================================================================
 *
 * IDEs, formatters, language servers and documentation generators may use
 * parser/AST output from this grammar.
 *
 * They MUST NOT infer semantic validity from syntax alone.
 *
 * ============================================================================
 * ERROR CONTRACT
 * ============================================================================
 *
 * Syntax errors:
 *
 *   - missing TYPE/ALIAS
 *   - missing identifier
 *   - malformed generic parameter list
 *   - malformed bounds
 *   - missing '='
 *   - missing type expression
 *   - missing ';'
 *
 * are parser errors.
 *
 * Semantic errors:
 *
 *   - unknown type
 *   - duplicate type
 *   - cyclic alias
 *   - invalid bound
 *   - invalid generic application
 *
 * are semantic errors.
 *
 * The two categories MUST remain distinct.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing syntax:
 *
 *     type Name = ExistingType;
 *
 * remains supported.
 *
 * Existing aliases must be migrated deliberately rather than silently
 * removed.
 *
 * The legacy monolithic Zamani.g4 currently contains type declarations and
 * aliases; those constructs must be migrated into this modular ownership
 * boundary and removed from the monolithic grammar only after compatibility
 * tests pass.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden:
 *
 *   - numeric maximums
 *   - fixed generic arity
 *   - fixed tuple arity
 *   - fixed dimensions
 *   - fixed machine type lists as semantic limits
 *   - fixed qubit counts
 *   - fixed hardware capacities
 *   - fixed device IDs
 *   - fixed topology
 *
 * Allowed:
 *
 *   - language keywords
 *   - punctuation
 *   - syntactic delimiters
 *   - semantic category names when defined by the language specification
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete only when:
 *
 * [ ] typeDeclaration is the sole declaration-level owner of named `type`.
 * [ ] typeAliasDeclaration is the sole declaration-level owner of `alias`.
 * [ ] declarations.g4 imports this family instead of duplicating it.
 * [ ] grammar/types/ owns the authoritative type-expression grammar.
 * [ ] Generic arity is unbounded by language syntax.
 * [ ] No machine/resource limits exist in this file.
 * [ ] Quantum types are not duplicated here.
 * [ ] Hardware types are not duplicated here.
 * [ ] Type checking is not embedded in the parser.
 * [ ] AST ownership is defined.
 * [ ] Semantic ownership is defined.
 * [ ] IR ownership remains downstream.
 * [ ] quantum::ir remains the canonical quantum semantic boundary.
 * [ ] No runtime dependency exists.
 * [ ] No Rust actions exist in the grammar.
 * [ ] Rust integration remains safe-only and compatible with Rust 1.97/1.97.1.
 * [ ] Positive parser tests exist.
 * [ ] Negative parser tests exist.
 * [ ] Generic/boundary tests exist.
 * [ ] Quantum/classical/HDL/hardware integration tests exist.
 * [ ] Compatibility tests cover the former monolithic grammar.
 * [ ] Hard-coding audit passes.
 * [ ] Deterministic parsing tests pass.
 *
 * ============================================================================
 */
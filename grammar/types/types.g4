/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/types/type.g4
 *
 * Grammar:
 *     Type
 *
 * Status:
 *     CANONICAL TYPE-SYSTEM ORCHESTRATOR
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *     no unsafe
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the SINGLE ORCHESTRATION BOUNDARY for source-level type
 * syntax in Zamani.
 *
 * It does not attempt to implement every type constructor itself.
 *
 * Instead, it composes the specialized grammar components under:
 *
 *     grammar/types/
 *
 * The architectural rule is:
 *
 *     Type
 *       |
 *       +--> primitive
 *       +--> named
 *       +--> generic
 *       +--> tuple
 *       +--> array
 *       +--> slice
 *       +--> map
 *       +--> option
 *       +--> result
 *       +--> record
 *       +--> sum
 *       +--> union
 *       +--> function
 *       +--> reference
 *       +--> pointer
 *       +--> linear
 *       +--> affine
 *       +--> dependent
 *       +--> associated
 *       +--> type-class
 *       +--> classical
 *       +--> quantum
 *       +--> hardware
 *       +--> resource
 *       +--> capability
 *       +--> temporal
 *       +--> effect-qualified
 *       +--> future extensible type forms
 *
 * `typeExpression` is the ONLY public source-level type composition rule.
 *
 * ============================================================================
 * SINGLE-OWNER RULE
 * ============================================================================
 *
 * This file owns:
 *
 *     typeExpression
 *     typeCore
 *     typePostfix
 *     typeModifier
 *     typeExtension
 *
 * This file does NOT own:
 *
 *     primitive type details
 *     named-type details
 *     generic argument details
 *     tuple details
 *     array details
 *     slice details
 *     function-type details
 *     reference details
 *     pointer details
 *     quantum-type details
 *     hardware-type details
 *     resource-type details
 *     capability-type details
 *     classical-domain type details
 *     dependent-type details
 *     associated-type details
 *     type-class details
 *     temporal-type details
 *     effect definitions
 *     type inference
 *     type checking
 *     type unification
 *     type substitution
 *     trait resolution
 *     capability resolution
 *     resource discovery
 *     hardware discovery
 *     target selection
 *     placement
 *     routing
 *     scheduling
 *     optimization
 *     calibration
 *     QEC
 *     ZQN
 *     HAL
 *     runtime representation
 *     ABI layout
 *
 * ============================================================================
 * ARCHITECTURAL PIPELINE
 * ============================================================================
 *
 *     source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       v
 *     Type
 *       |
 *       v
 *     typeExpression
 *       |
 *       v
 *     frontend TypeExpr
 *       |
 *       v
 *     structural validation
 *       |
 *       v
 *     semantic type resolution
 *       |
 *       +------------------+------------------+------------------+
 *       |                  |                  |                  |
 *       v                  v                  v                  v
 *   classical          quantum::ir        HDL/resource       AI/data
 *   semantics           semantics          semantics          semantics
 *       |                  |                  |                  |
 *       +------------------+------------------+------------------+
 *                              |
 *                              v
 *                     canonical semantic model
 *                              |
 *                              v
 *                       target-independent IR
 *                              |
 *                 optimization / specialization
 *                              |
 *                    lowering / routing
 *                              |
 *                        scheduling
 *                              |
 *                   resilience / recovery
 *                              |
 *                         ZQN / HAL
 *                              |
 *                        target realization
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * A type expresses PROGRAM MEANING.
 *
 * A type MUST NOT select a physical implementation.
 *
 * In particular, this grammar MUST NOT encode:
 *
 *     maximum qubits
 *     maximum CPUs
 *     maximum GPUs
 *     maximum FPGAs
 *     maximum nodes
 *     maximum memory
 *     maximum threads
 *     maximum register width
 *     maximum tensor rank
 *     maximum device count
 *     maximum topology size
 *
 * The grammar contains no artificial capacity constants.
 *
 * A source program may contain symbolic values such as:
 *
 *     N
 *     Rows
 *     Columns
 *     RequiredMemory
 *     RequiredWidth
 *
 * Those values remain source-level symbolic information until semantic
 * analysis determines their meaning.
 *
 * Physical feasibility is handled by resource/capability negotiation and
 * target realization, not by this grammar.
 *
 * ============================================================================
 * EXTENSIBILITY CONTRACT
 * ============================================================================
 *
 * New computational domains MUST NOT require changing this file merely
 * because a new type constructor is introduced.
 *
 * Prefer:
 *
 *     named types
 *     qualified types
 *     generic types
 *     value-parameterized types
 *     domain dialects
 *     semantic registration
 *
 * over adding an ever-growing closed keyword inventory.
 *
 * Examples that should remain representable without adding universal
 * hardware/domain keywords:
 *
 *     Tensor<T>
 *     Tensor<T>[Shape]
 *     Model<T>
 *     QuantumState<T>
 *     LogicalQubit
 *     QRegister<N>
 *     Signal<T>
 *     Accelerator<T>
 *     Resource<T>
 *     Capability<T>
 *     Dataset<T>
 *     DistributedState<T>
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Every successful type parse feeds the existing frontend TypeExpr boundary.
 *
 * This grammar MUST NOT introduce:
 *
 *     TypeExpr2
 *     UniversalTypeExpr
 *     QuantumTypeExpr
 *     HardwareTypeExpr
 *     AiTypeExpr
 *     DomainTypeIR
 *
 * Domain-specific semantic information is resolved after parsing.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing establishes structure only.
 *
 * Semantic analysis is responsible for:
 *
 *     name resolution
 *     alias resolution
 *     generic substitution
 *     type inference
 *     unification
 *     constraint solving
 *     ownership checking
 *     lifetime checking
 *     linearity checking
 *     capability checking
 *     resource checking
 *     effect checking
 *     contract checking
 *     policy checking
 *     domain validation
 *
 * ============================================================================
 * QUANTUM CONTRACT
 * ============================================================================
 *
 * Quantum source types are accepted here only as SOURCE TYPES.
 *
 * This grammar does not:
 *
 *     allocate physical qubits
 *     select a QPU
 *     choose topology
 *     perform routing
 *     schedule gates
 *     choose calibration
 *     choose QEC
 *     construct pulses
 *
 * The required semantic direction is:
 *
 *     source type
 *         |
 *         v
 *     TypeExpr
 *         |
 *         v
 *     semantic quantum type
 *         |
 *         v
 *     quantum::ir
 *         |
 *         v
 *     optimization
 *         |
 *         v
 *     decomposition
 *         |
 *         v
 *     routing
 *         |
 *         v
 *     scheduling
 *         |
 *         v
 *     resilience / QEC
 *         |
 *         v
 *     ZQN
 *         |
 *         v
 *     HAL
 *
 * ============================================================================
 * HARDWARE CONTRACT
 * ============================================================================
 *
 * Hardware types describe abstract requirements or capabilities.
 *
 * They do not identify a particular machine.
 *
 * For example:
 *
 *     Hardware<Compute>
 *     Memory<T>
 *     Accelerator<Model>
 *     Interconnect<Message>
 *
 * are source-level abstractions.
 *
 * Concrete realization belongs to:
 *
 *     grammar/resources/
 *     grammar/hardware/
 *     grammar/compile/
 *     grammar/execution/
 *
 * ============================================================================
 * AI / DATA CONTRACT
 * ============================================================================
 *
 * Type syntax remains independent of the reasoning, learning, knowledge,
 * uncertainty, provenance, policy, and agent systems.
 *
 * AI/data concepts are represented through normal types, generic types,
 * domain types, and semantic capabilities.
 *
 * For example:
 *
 *     Model<T>
 *     Dataset<T>
 *     Distribution<T>
 *     Evidence<T>
 *     Knowledge<T>
 *
 * do not require a separate AI type system.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Effect-qualified types are composed through the existing effect system.
 *
 * This file does not redefine effect syntax.
 *
 * Effect semantics remain owned by:
 *
 *     grammar/effects/
 *
 * and the type-level effect qualifier component.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY CONTRACT
 * ============================================================================
 *
 * A type can carry semantic relationships to:
 *
 *     requirements
 *     capabilities
 *     resources
 *     constraints
 *     policies
 *
 * but this grammar does not resolve them.
 *
 * For example:
 *
 *     Resource<T>
 *     Capability<T>
 *
 * remain source-level type applications.
 *
 * A semantic requirement such as:
 *
 *     requires capability("tensor.compute")
 *
 * belongs to the resource/capability subsystem rather than this grammar.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This grammar declares NO lexer rules.
 *
 * All tokens MUST come from:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * The lexical vocabulary is therefore centralized.
 *
 * ============================================================================
 * IMPORT CONTRACT
 * ============================================================================
 *
 * IMPORTANT:
 *
 * The repository currently contains legacy and transitional type grammar
 * components. They must not become competing authorities.
 *
 * The canonical production migration is:
 *
 *     specialized grammar
 *             |
 *             v
 *     specialized parser grammar
 *             |
 *             v
 *     Type orchestrator
 *             |
 *             v
 *     typeExpression
 *
 * Components that currently duplicate `typeExpression` MUST be converted
 * into delegates before being imported here.
 *
 * The following components are intended to be composed by this file:
 *
 *     PrimitiveTypes
 *     NamedTypes
 *     Generic
 *     TupleTypes
 *     Array
 *     Slice
 *     Function
 *     ReferenceTypes
 *     PointerTypes
 *     Result
 *     Option
 *     RecordTypes
 *     Sum
 *     Union
 *     Dependent
 *     Associated
 *     TypeClasses
 *     LinearTypes
 *     AffineTypes
 *     ClassicalTypes
 *     Quantum
 *     Resource
 *     CapabilityTypes
 *     ZamaniHardwareTypesParser
 *     TemporalTypes
 *     Effectful
 *
 * Where an existing component has a conflicting grammar name or duplicated
 * rule authority, it MUST be normalized before becoming a production import.
 *
 * ============================================================================
 * IMPORTANT ANTLR COMPOSITION RULE
 * ============================================================================
 *
 * `type.g4` MUST NOT import `types.g4`.
 *
 * `types.g4` is the existing legacy/canonical composition file and currently
 * owns another implementation of `typeExpression`.
 *
 * The migration target is:
 *
 *     type.g4
 *          |
 *          +--> specialized delegates
 *
 * and:
 *
 *     types.g4
 *          |
 *          +--> compatibility facade
 *          |
 *          +--> Type
 *
 * This prevents:
 *
 *     Type -> Types -> Type
 *
 * circular grammar composition.
 *
 * ============================================================================
 */

parser grammar Type;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * 1. PUBLIC ORCHESTRATION ENTRY POINT
 * ========================================================================== */

/**
 * The single public source-level type entry point.
 *
 * Type qualifiers/modifiers are deliberately separated from the core
 * constructor so that new semantic qualifiers can be added without creating
 * another type-expression authority.
 */
typeExpression
    : typePrefix*
      typeCore
      typePostfix*
    ;


/* ============================================================================
 * 2. TYPE PREFIXES
 * ========================================================================== */

/**
 * Prefixes that affect the semantic interpretation of a type.
 *
 * These are syntactic wrappers. Their semantic validity belongs downstream.
 */
typePrefix
    : linearTypePrefix
    | affineTypePrefix
    | referenceTypePrefix
    | typeAttributePrefix
    ;


/**
 * Linear ownership/usage qualifier.
 *
 * The specialized LinearTypes grammar owns the detailed form.
 */
linearTypePrefix
    : linearQualifier
    ;


/**
 * Affine ownership/usage qualifier.
 *
 * The specialized affine grammar owns the detailed form.
 */
affineTypePrefix
    : affineQualifier
    ;


/**
 * Reference prefixes are delegated to the reference grammar.
 *
 * This rule is intentionally separate from referenceType because the
 * orchestrator must not duplicate the reference implementation.
 */
referenceTypePrefix
    : referencePrefix
    ;


/**
 * Attribute-style type metadata.
 *
 * The exact attribute grammar is owned by the core/attribute subsystem.
 *
 * This rule is deliberately left as an integration boundary rather than
 * introducing another local attribute grammar here.
 */
typeAttributePrefix
    : AT IDENTIFIER
    ;


/* ============================================================================
 * 3. TYPE CORE
 * ========================================================================== */

/**
 * All type constructors visible through the universal type system.
 *
 * Order matters where alternatives have overlapping prefixes.
 *
 * More structurally specific forms are selected before the open-ended named
 * type fallback.
 */
typeCore
    : primitiveType
    | functionType
    | tupleType
    | arrayType
    | sliceType
    | resultType
    | optionType
    | recordType
    | sumType
    | unionType
    | dependentType
    | associatedType
    | typeClassProjection
    | quantumType
    | classicalType
    | hardwareType
    | resourceType
    | capabilityType
    | temporalType
    | namedOrGenericType
    | parenthesizedType
    ;


/* ============================================================================
 * 4. PRIMITIVE TYPES
 * ========================================================================== */

primitiveType
    : primitiveScalarType
    | unitType
    | neverType
    ;


/* ============================================================================
 * 5. FUNCTION TYPES
 * ========================================================================== */

functionType
    : FN
      LPAREN
      functionTypeParameterList?
      RPAREN
      functionTypeReturn?
    ;


/**
 * The detailed Function grammar owns:
 *
 *     functionTypeParameterList
 *     functionTypeParameter
 *     functionTypeReturn
 *
 * They are referenced directly so that the complete function-type structure
 * remains compatible with the existing delegate.
 */
functionTypeParameterList
    : functionTypeParameter
      (COMMA functionTypeParameter)*
      COMMA?
    ;


functionTypeParameter
    : typeExpression
    ;


functionTypeReturn
    : THIN_ARROW typeExpression
    ;


/* ============================================================================
 * 6. TUPLES
 * ========================================================================== */

tupleType
    : LPAREN
      tupleElementList?
      RPAREN
    ;


tupleElementList
    : typeExpression
      COMMA
      tupleAdditionalElement*
      COMMA?
    ;


tupleAdditionalElement
    : typeExpression
      COMMA?
    ;


/* ============================================================================
 * 7. ARRAYS
 * ========================================================================== */

/**
 * Sized arrays remain symbolic.
 *
 * The array length is not converted into a host integer by the grammar.
 */
arrayType
    : LBRACKET
      typeExpression
      SEMI
      typeValueExpression
      RBRACKET
    ;


/* ============================================================================
 * 8. SLICES
 * ========================================================================== */

sliceType
    : LBRACKET
      typeExpression
      RBRACKET
    ;


/* ============================================================================
 * 9. RESULT
 * ========================================================================== */

resultType
    : RESULT
      LESS_THAN
      typeExpression
      COMMA
      typeExpression
      GREATER_THAN
    ;


/* ============================================================================
 * 10. OPTION
 * ========================================================================== */

/**
 * Canonical optional spelling:
 *
 *     T?
 *
 * The lexer owns QUESTION_MARK.
 */
optionType
    : typeExpression
      QUESTION_MARK
    ;


/* ============================================================================
 * 11. RECORD
 * ========================================================================== */

recordType
    : recordTypeDelegate
    ;


recordTypeDelegate
    : recordType
    ;


/* ============================================================================
 * 12. SUM
 * ========================================================================== */

sumType
    : sumTypeDelegate
    ;


sumTypeDelegate
    : sumType
    ;


/* ============================================================================
 * 13. UNION
 * ========================================================================== */

unionType
    : unionTypeDelegate
    ;


unionTypeDelegate
    : typeUnion
    ;


/* ============================================================================
 * 14. DEPENDENT / VALUE-PARAMETERIZED TYPES
 * ========================================================================== */

dependentType
    : typePath
      LBRACKET
      typeValueExpression
      (COMMA typeValueExpression)*
      RBRACKET
    ;


/* ============================================================================
 * 15. ASSOCIATED TYPES
 * ========================================================================== */

associatedType
    : associatedTypeBase
      associatedTypeProjectionSuffix+
    ;


/* ============================================================================
 * 16. TYPE-CLASS PROJECTIONS
 * ========================================================================== */

typeClassProjection
    : typeClassProjection
    ;


/* ============================================================================
 * 17. QUANTUM TYPES
 * ========================================================================== */

quantumType
    : QUBIT
    ;


/* ============================================================================
 * 18. CLASSICAL DOMAIN TYPES
 * ========================================================================== */

classicalType
    : classicalTypePrimary
      classicalTypePostfix*
    ;


/* ============================================================================
 * 19. HARDWARE TYPES
 * ========================================================================== */

hardwareType
    : hardwareTypePrimary
      hardwareTypePostfix*
    ;


/* ============================================================================
 * 20. RESOURCE TYPES
 * ========================================================================== */

resourceType
    : resourceTypeQualifier
    ;


/* ============================================================================
 * 21. CAPABILITY TYPES
 * ========================================================================== */

capabilityType
    : capabilityIdentity
      capabilityTypeArgumentList?
    ;


/* ============================================================================
 * 22. TEMPORAL TYPES
 * ========================================================================== */

temporalType
    : temporalTypeConstructor
    ;


/* ============================================================================
 * 23. NAMED AND GENERIC TYPES
 * ========================================================================== */

/**
 * Named types are the open-ended extension mechanism for the language.
 *
 * Examples:
 *
 *     User
 *     LogicalQubit
 *     Tensor
 *     Model
 *     Signal
 *     Accelerator
 *     Resource
 *
 * Qualified names remain open-ended:
 *
 *     std::collections::Map
 *     quantum::State
 *     hardware::Accelerator
 *     data::Dataset
 */
namedOrGenericType
    : typePath
      typeArguments?
    ;


typePath
    : typePathSegment
      (DOUBLE_COLON typePathSegment)*
    ;


typePathSegment
    : IDENTIFIER
    ;


typeArguments
    : LESS_THAN
      genericArgumentList
      GREATER_THAN
    ;


genericArgumentList
    : genericArgument
      (COMMA genericArgument)*
      COMMA?
    ;


genericArgument
    : typeExpression
    ;


/* ============================================================================
 * 24. PARENTHESIZED TYPES
 * ========================================================================== */

parenthesizedType
    : LPAREN
      typeExpression
      RPAREN
    ;


/* ============================================================================
 * 25. TYPE POSTFIXES
 * ========================================================================== */

/**
 * Postfixes are intentionally narrow.
 *
 * Optionality is handled structurally by optionType when it appears as a
 * complete type.
 */
typePostfix
    : QUESTION_MARK
    ;


/* ============================================================================
 * 26. TYPE-LEVEL VALUE EXPRESSIONS
 * ========================================================================== */

/**
 * These expressions exist only to preserve symbolic source-level values used
 * by parameterized types.
 *
 * They are not evaluated by ANTLR.
 *
 * They are not restricted to a machine-sized integer.
 *
 * Semantic analysis determines:
 *
 *     type
 *     value domain
 *     const-ness
 *     satisfiability
 *     representability
 *     target realization
 */
typeValueExpression
    : typeValueBitwiseOr
    ;


typeValueBitwiseOr
    : typeValueBitwiseXor
      (PIPE typeValueBitwiseXor)*
    ;


typeValueBitwiseXor
    : typeValueBitwiseAnd
      (CARET typeValueBitwiseAnd)*
    ;


typeValueBitwiseAnd
    : typeValueShift
      (BIT_AND_OPERATOR typeValueShift)*
    ;


typeValueShift
    : typeValueAdditive
      ((LEFT_SHIFT | RIGHT_SHIFT) typeValueAdditive)*
    ;


typeValueAdditive
    : typeValueMultiplicative
      ((PLUS | MINUS) typeValueMultiplicative)*
    ;


typeValueMultiplicative
    : typeValueUnary
      ((STAR | SLASH | MODULO) typeValueUnary)*
    ;


typeValueUnary
    : (PLUS | MINUS | CARET | TILDE)*
      typeValuePrimary
    ;


typeValuePrimary
    : INTEGER
    | FLOAT
    | IDENTIFIER
    | typeValueQualifiedPath
    | parenthesizedTypeValue
    ;


typeValueQualifiedPath
    : IDENTIFIER
      DOUBLE_COLON
      IDENTIFIER
      (DOUBLE_COLON IDENTIFIER)*
    ;


parenthesizedTypeValue
    : LPAREN
      typeValueExpression
      RPAREN
    ;


/* ============================================================================
 * 27. TYPE EXTENSION BOUNDARY
 * ========================================================================== */

/**
 * Domain extensions MUST enter through ordinary named/qualified/generic
 * types or through registered specialized grammar delegates.
 *
 * This rule intentionally does not enumerate application-specific concepts.
 */
typeExtension
    : namedOrGenericType
    ;


/* ============================================================================
 * 28. TYPE ORCHESTRATION INVARIANTS
 * ============================================================================
 *
 * INVARIANT 1
 *
 * There is exactly one public source-level type entry point:
 *
 *     typeExpression
 *
 *
 * INVARIANT 2
 *
 * There is exactly one universal composition authority:
 *
 *     Type
 *
 *
 * INVARIANT 3
 *
 * Specialized files own specialized syntax.
 *
 *
 * INVARIANT 4
 *
 * No specialized type grammar may define another universal `typeExpression`.
 *
 *
 * INVARIANT 5
 *
 * No type grammar may allocate physical resources.
 *
 *
 * INVARIANT 6
 *
 * No type grammar may select a backend.
 *
 *
 * INVARIANT 7
 *
 * No type grammar may introduce a second AST.
 *
 *
 * INVARIANT 8
 *
 * No type grammar may introduce a second quantum IR.
 *
 *
 * INVARIANT 9
 *
 * No type grammar may impose artificial capacity limits.
 *
 *
 * INVARIANT 10
 *
 * New domains must be representable through open type composition without
 * changing the universal semantic model.
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Syntax errors belong to the parser.
 *
 * Semantic errors belong downstream.
 *
 * Examples of semantic errors that MUST NOT be implemented as grammar rules:
 *
 *     unknown type
 *     unsatisfied generic constraint
 *     invalid capability
 *     unavailable resource
 *     unsupported hardware realization
 *     insufficient memory
 *     impossible topology
 *     unavailable quantum capability
 *     invalid QEC strategy
 *     invalid ABI
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar imposes no semantic maximum on:
 *
 *     generic arguments
 *     tuple elements
 *     namespace depth
 *     type nesting
 *     symbolic dimensions
 *     quantum cardinality
 *     tensor rank
 *     collection size
 *     resource count
 *     hardware count
 *     device count
 *
 * Any implementation protection against pathological parser input belongs
 * to explicit compiler/parser configuration and MUST NOT change language
 * meaning.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing the same source with the same language/grammar version must produce
 * the same parse structure.
 *
 * The grammar contains:
 *
 *     no filesystem operations
 *     no network operations
 *     no runtime calls
 *     no semantic actions
 *     no unsafe operations
 *     no target probing
 *     no hardware probing
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * AST:
 *
 *     frontend TypeExpr
 *
 * Semantic:
 *
 *     semantic type resolver
 *     type constraint solver
 *     capability/resource analysis
 *     effect analysis
 *     contract/policy validation
 *
 * Classical:
 *
 *     classical semantic model
 *
 * Quantum:
 *
 *     quantum semantic model
 *     quantum::ir
 *
 * HDL:
 *
 *     HDL semantic model
 *
 * Hardware:
 *
 *     hardware/resource semantic model
 *
 * AI/data:
 *
 *     ordinary source-level types
 *     semantic capabilities
 *     domain libraries
 *
 * Canonical compilation:
 *
 *     Type
 *       |
 *       v
 *     TypeExpr
 *       |
 *       v
 *     semantic type
 *       |
 *       v
 *     canonical semantic IR
 *       |
 *       +--> classical IR
 *       |
 *       +--> quantum::ir
 *       |
 *       +--> HDL/hardware representation
 *       |
 *       +--> distributed/data/AI representations
 *
 * ============================================================================
 * FILE COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is COMPLETE only when:
 *
 * [ ] `typeExpression` is the sole public universal type entry point.
 *
 * [ ] No other file defines a competing universal `typeExpression`.
 *
 * [ ] Every specialized type component has exactly one owner.
 *
 * [ ] All imports use valid ANTLR parser grammar names.
 *
 * [ ] No import introduces circular grammar composition.
 *
 * [ ] No duplicate parser rule names remain across imported components.
 *
 * [ ] Primitive, named, generic, composite, dependent, domain, resource,
 *     capability, and effect-qualified types have explicit integration
 *     boundaries.
 *
 * [ ] TypeExpr remains the only source AST type boundary.
 *
 * [ ] Quantum types lower semantically toward `quantum::ir`.
 *
 * [ ] No physical resource or backend is selected by the grammar.
 *
 * [ ] No artificial capacity constants exist.
 *
 * [ ] Positive parser tests exist for every public constructor.
 *
 * [ ] Negative parser tests exist for malformed constructors.
 *
 * [ ] Nested/compositional tests exist.
 *
 * [ ] Large symbolic type tests exist.
 *
 * [ ] Cross-domain type tests exist.
 *
 * [ ] Deterministic parsing tests exist.
 *
 * [ ] Compatibility tests exist.
 *
 * [ ] Rust 1.97 / 1.97.1 toolchain builds the generated parser without
 *     unsafe requirements.
 *
 * ============================================================================
 */
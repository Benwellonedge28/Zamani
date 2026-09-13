/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/statements/declarations.g4
 *
 * Status:
 *     Production modular declaration-statement grammar.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Lexer:
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Rust implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     This file contains grammar only.
 *     No embedded Rust actions.
 *     No semantic predicates.
 *     No unsafe code.
 *     No filesystem access.
 *     No networking.
 *     No device discovery.
 *     No runtime execution.
 *     No hardware inspection.
 *     No mutable parser-global state.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the syntax boundary for declarations that may occur in
 * statement/declaration positions.
 *
 * It establishes a stable declaration composition layer between:
 *
 *     statements.g4
 *
 * and the specialized declaration grammars:
 *
 *     grammar/functions/
 *     grammar/modules/
 *     grammar/types/
 *     grammar/core/
 *     domain-specific grammar modules
 *
 * This file deliberately does NOT become a second complete parser.
 *
 * ============================================================================
 * ARCHITECTURAL PIPELINE
 * ============================================================================
 *
 *     UTF-8 source
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     modular parser
 *          |
 *          v
 *     declarations.g4
 *          |
 *          +--> functions
 *          +--> types
 *          +--> modules
 *          +--> bindings
 *          +--> domain declarations
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     name resolution
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> type analysis
 *          +--> effect analysis
 *          +--> capability analysis
 *          +--> resource analysis
 *          +--> ownership/lifetime analysis
 *          |
 *          v
 *     canonical semantic representations
 *          |
 *          +--> classical IR
 *          +--> quantum::ir
 *          +--> HDL/hardware IR
 *          +--> control/data IR
 *          +--> resource metadata
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     routing / scheduling / lowering
 *          |
 *          v
 *     target realization
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - declarationStatement
 *   - declaration dispatch
 *   - local variable declarations
 *   - local constant declarations
 *   - type declaration dispatch
 *   - struct declarations
 *   - enum declarations
 *   - record declarations
 *   - class declarations
 *   - interface declarations
 *   - trait declarations
 *   - implementation declarations
 *   - type-alias declarations
 *   - declaration-level visibility/modifier composition
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - lexical tokens
 *   - identifier spelling
 *   - comments
 *   - attributes
 *   - expressions
 *   - expression precedence
 *   - canonical type-expression syntax
 *   - function internals
 *   - module/import/export resolution
 *   - effect semantics
 *   - capability semantics
 *   - quantum semantics
 *   - physical qubit allocation
 *   - hardware topology
 *   - resource limits
 *   - scheduling
 *   - routing
 *   - optimization
 *   - QEC
 *   - ZQN
 *   - runtime behavior
 *   - backend selection
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Declaration syntax describes program structure and semantic intent.
 *
 * It MUST NOT encode:
 *
 *   - maximum number of declarations;
 *   - maximum number of fields;
 *   - maximum number of enum variants;
 *   - maximum generic arity;
 *   - maximum nesting depth;
 *   - maximum machine size;
 *   - maximum qubit count;
 *   - maximum CPU count;
 *   - maximum GPU count;
 *   - maximum FPGA count;
 *   - maximum memory;
 *   - fixed register width;
 *   - fixed hardware topology;
 *   - fixed device identity.
 *
 * Repetition operators such as `*` and `+` are therefore intentional.
 *
 * Any practical implementation limit belongs to:
 *
 *   - parser/runtime resource policy;
 *   - compiler resource policy;
 *   - semantic analysis;
 *   - target capabilities;
 *   - execution resources.
 *
 * Such limits MUST NOT become language semantics.
 *
 * ============================================================================
 * CANONICAL LEXER CONTRACT
 * ============================================================================
 *
 * All lexical tokens come from ZamaniLexer.
 *
 * This file MUST NOT declare lexer rules.
 *
 * The canonical lexer already owns identifiers and punctuation. The parser
 * therefore consumes the canonical IDENTIFIER token through the shared
 * identifier rule supplied by the parser composition layer.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * statements/statements.g4 currently dispatches through:
 *
 *     declarationStatement
 *
 * and expects declaration-family rules such as:
 *
 *     functionDeclarationStatement
 *     typeDeclarationStatement
 *     moduleDeclarationStatement
 *     importStatement
 *     exportStatement
 *     effectDeclarationStatement
 *     domainDeclarationStatement
 *
 * Those names remain compatibility integration points.
 *
 * This file MUST NOT create a competing `statement` rule.
 *
 * `statements/statements.g4` remains the owner of statement composition.
 *
 * ============================================================================
 * SPECIALIZED OWNERSHIP
 * ============================================================================
 *
 * Functions:
 *
 *     grammar/functions/
 *
 * owns function declaration internals.
 *
 * Modules:
 *
 *     grammar/modules/
 *
 * owns module/import/export syntax.
 *
 * Types:
 *
 *     grammar/types/
 *
 * owns `typeExpression`.
 *
 * Core:
 *
 *     grammar/core/
 *
 * owns names, paths, attributes, metadata, constraints and other reusable
 * source-language infrastructure.
 *
 * This file references those canonical rules rather than duplicating them.
 *
 * ============================================================================
 * SEMANTIC BOUNDARY
 * ============================================================================
 *
 * Parsing:
 *
 *     "struct Particle { ... }"
 *
 * does not decide:
 *
 *     - memory layout;
 *     - ABI;
 *     - CPU representation;
 *     - FPGA implementation;
 *     - quantum representation;
 *     - hardware placement;
 *     - distributed placement.
 *
 * Those decisions belong downstream.
 *
 * Likewise:
 *
 *     struct
 *     class
 *     interface
 *     trait
 *     type
 *
 * are source-language constructs.
 *
 * They are not hardware constructs.
 *
 * ============================================================================
 */

parser grammar Declarations;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * 1. PUBLIC DECLARATION ENTRY POINTS
 * ========================================================================== */

/**
 * Canonical declaration statement entry point.
 *
 * This is the rule consumed by statements/statements.g4.
 *
 * It intentionally distinguishes:
 *
 *     local bindings
 *
 * from:
 *
 *     type/item declarations.
 *
 * Function/module/import/export declarations remain delegated to their
 * specialized modules.
 */
declarationStatement
    : localDeclaration
    | typeDeclarationStatement
    | functionDeclarationStatement
    | moduleDeclarationStatement
    | importStatement
    | exportStatement
    | effectDeclarationStatement
    | domainDeclarationStatement
    ;


/**
 * Declaration item used by compilation-unit/item composition.
 *
 * This rule is useful when the surrounding parser has a separate distinction
 * between top-level items and statements.
 */
declarationItem
    : typeDeclarationStatement
    | functionDeclarationStatement
    | moduleDeclarationStatement
    | importStatement
    | exportStatement
    | effectDeclarationStatement
    | domainDeclarationStatement
    ;


/* ============================================================================
 * 2. LOCAL BINDINGS
 * ========================================================================== */

/**
 * Local declaration.
 *
 * Initialization is optional for mutable/general variables only when the
 * surrounding language semantics permit it.
 *
 * The semantic layer determines:
 *
 *     - definite initialization;
 *     - mutability;
 *     - ownership;
 *     - borrowing;
 *     - lifetime;
 *     - resource ownership;
 *     - type inference.
 */
localDeclaration
    : variableDeclaration
    | constantDeclaration
    ;


/**
 * General variable declaration.
 *
 * Examples:
 *
 *     let value = expression;
 *     let value: T = expression;
 *     var value = expression;
 *     var value: T;
 *
 * The parser does not determine whether an uninitialized declaration is
 * semantically legal.
 */
variableDeclaration
    : variableModifier
      identifier
      variableTypeAnnotation?
      initializer?
      declarationTerminator
    ;


/**
 * Variable keyword.
 *
 * `let` and `var` are syntactic distinctions.
 * Semantic mutability rules are checked downstream.
 */
variableModifier
    : LET
    | VAR
    ;


/**
 * Explicit local constant.
 *
 * Examples:
 *
 *     const answer: int = expression;
 *
 * Constants require an initializer at the source level.
 */
constantDeclaration
    : CONST
      identifier
      COLON
      typeExpression
      ASSIGN
      expression
      declarationTerminator
    ;


/**
 * Optional type annotation.
 */
variableTypeAnnotation
    : COLON
      typeExpression
    ;


/**
 * Initializer.
 */
initializer
    : ASSIGN
      expression
    ;


/**
 * Declaration termination.
 *
 * The modular grammar currently uses explicit semicolon termination for
 * declarations. Automatic semicolon insertion must not be introduced here
 * without a corresponding language-specification change.
 */
declarationTerminator
    : SEMICOLON
    ;


/* ============================================================================
 * 3. TYPE DECLARATION DISPATCH
 * ========================================================================== */

/**
 * Canonical type declaration family.
 */
typeDeclarationStatement
    : typeAliasDeclaration
    | structDeclaration
    | enumDeclaration
    | recordDeclaration
    | classDeclaration
    | interfaceDeclaration
    | traitDeclaration
    | implDeclaration
    ;


/* ============================================================================
 * 4. DECLARATION MODIFIERS
 * ========================================================================== */

/**
 * Visibility.
 *
 * Multiple visibility modifiers are intentionally not accepted syntactically.
 * Semantic validation therefore does not need to resolve duplicate visibility
 * syntax.
 */
visibility
    : PUBLIC
    | PUB
    | PRIVATE
    | PROTECTED
    | INTERNAL
    ;


/**
 * Common declaration modifiers.
 *
 * Target-specific modifiers such as "gpu", "qpu", "cpu32", etc. MUST NOT be
 * added here.
 *
 * Target capabilities belong to target/resource/capability grammar layers.
 */
declarationModifier
    : STATIC
    | ABSTRACT
    | FINAL
    | VIRTUAL
    | OVERRIDE
    ;


/**
 * Zero or more modifiers.
 */
declarationModifiers
    : declarationModifier*
    ;


/**
 * Complete declaration prefix.
 */
declarationPrefix
    : visibility?
      declarationModifiers
    ;


/* ============================================================================
 * 5. TYPE ALIASES
 * ========================================================================== */

/**
 * Type alias.
 *
 * Examples:
 *
 *     type Number = int;
 *     type State = quantum::State;
 *     type Matrix<T, N, M> = Tensor<T, N, M>;
 *
 * The target representation remains a semantic concern.
 */
typeAliasDeclaration
    : declarationPrefix
      TYPE
      identifier
      genericParameters?
      ASSIGN
      typeExpression
      declarationTerminator
    ;


/* ============================================================================
 * 6. STRUCT DECLARATIONS
 * ========================================================================== */

/**
 * Struct declaration.
 *
 * No field-count limit is encoded.
 */
structDeclaration
    : declarationPrefix
      STRUCT
      identifier
      genericParameters?
      whereClause?
      LBRACE
      structField*
      RBRACE
    ;


/**
 * Struct field.
 *
 * Trailing commas are accepted.
 */
structField
    : visibility?
      identifier
      COLON
      typeExpression
      COMMA?
    ;


/* ============================================================================
 * 7. RECORD DECLARATIONS
 * ========================================================================== */

/**
 * Record is a distinct source-level declaration.
 *
 * Representation/layout remains downstream.
 */
recordDeclaration
    : declarationPrefix
      RECORD
      identifier
      genericParameters?
      whereClause?
      LBRACE
      recordField*
      RBRACE
    ;


recordField
    : visibility?
      identifier
      COLON
      typeExpression
      COMMA?
    ;


/* ============================================================================
 * 8. ENUM DECLARATIONS
 * ========================================================================== */

/**
 * Enum declaration.
 *
 * No finite variant limit is encoded.
 */
enumDeclaration
    : declarationPrefix
      ENUM
      identifier
      genericParameters?
      whereClause?
      LBRACE
      enumVariant*
      RBRACE
    ;


/**
 * Enum variants may be:
 *
 *     Unit
 *     Tuple(...)
 *     Struct { ... }
 */
enumVariant
    : identifier
      enumVariantPayload?
      COMMA?
    ;


enumVariantPayload
    : LPAREN
      enumTupleFields?
      RPAREN
    | LBRACE
      enumStructFields?
      RBRACE
    ;


enumTupleFields
    : typeExpression
      (COMMA typeExpression)*
      COMMA?
    ;


enumStructFields
    : enumStructField+
    ;


enumStructField
    : visibility?
      identifier
      COLON
      typeExpression
      COMMA?
    ;


/* ============================================================================
 * 9. CLASS DECLARATIONS
 * ========================================================================== */

/**
 * Class declaration.
 *
 * The class body is composed from member declarations.
 *
 * Function/member semantics belong to the functions/object-model layers.
 */
classDeclaration
    : declarationPrefix
      CLASS
      identifier
      genericParameters?
      inheritanceClause*
      whereClause?
      LBRACE
      classMember*
      RBRACE
    ;


/**
 * Class inheritance/implementation composition.
 *
 * Type validity is semantic.
 */
inheritanceClause
    : EXTENDS
      qualifiedNameList
    | IMPLEMENTS
      qualifiedNameList
    ;


qualifiedNameList
    : qualifiedName
      (COMMA qualifiedName)*
    ;


/**
 * Class members.
 *
 * Function declaration syntax is deliberately delegated to the canonical
 * function grammar.
 *
 * Field declarations are owned here because they introduce class state.
 */
classMember
    : classField
    | functionDeclaration
    | typeAliasDeclaration
    | nestedTypeDeclaration
    ;


classField
    : visibility?
      declarationModifiers
      identifier
      COLON
      typeExpression
      initializer?
      declarationTerminator
    ;


nestedTypeDeclaration
    : typeAliasDeclaration
    | structDeclaration
    | enumDeclaration
    | recordDeclaration
    | interfaceDeclaration
    | traitDeclaration
    ;


/* ============================================================================
 * 10. INTERFACE DECLARATIONS
 * ========================================================================== */

/**
 * Interface declaration.
 *
 * An interface specifies source-level contracts.
 *
 * It does not identify a hardware interface.
 */
interfaceDeclaration
    : declarationPrefix
      INTERFACE
      identifier
      genericParameters?
      inheritanceClause*
      whereClause?
      LBRACE
      interfaceMember*
      RBRACE
    ;


interfaceMember
    : functionDeclaration
    | typeAliasDeclaration
    | interfaceAssociatedType
    ;


interfaceAssociatedType
    : TYPE
      identifier
      genericParameters?
      typeAliasConstraint?
      declarationTerminator
    ;


/**
 * Optional associated-type constraint.
 */
typeAliasConstraint
    : COLON
      typeExpression
    ;


/* ============================================================================
 * 11. TRAIT DECLARATIONS
 * ========================================================================== */

/**
 * Trait declaration.
 *
 * Trait semantics are resolved by the type/semantic system.
 */
traitDeclaration
    : declarationPrefix
      TRAIT
      identifier
      genericParameters?
      inheritanceClause*
      whereClause?
      LBRACE
      traitMember*
      RBRACE
    ;


traitMember
    : functionDeclaration
    | typeAliasDeclaration
    | interfaceAssociatedType
    ;


/* ============================================================================
 * 12. IMPLEMENTATION DECLARATIONS
 * ========================================================================== */

/**
 * Implementation declaration.
 *
 * Supports:
 *
 *     impl Type { ... }
 *
 * and:
 *
 *     impl Trait for Type { ... }
 *
 * The semantic layer decides whether the implementation is coherent.
 */
implDeclaration
    : declarationPrefix
      IMPL
      genericParameters?
      implementationTarget
      whereClause?
      LBRACE
      implMember*
      RBRACE
    ;


implementationTarget
    : typeExpression
    | qualifiedName
      FOR
      typeExpression
    ;


implMember
    : functionDeclaration
    | typeAliasDeclaration
    | constMemberDeclaration
    ;


constMemberDeclaration
    : declarationPrefix
      CONST
      identifier
      COLON
      typeExpression
      ASSIGN
      expression
      declarationTerminator
    ;


/* ============================================================================
 * 13. FUNCTION INTEGRATION
 * ========================================================================== */

/**
 * Compatibility adapter consumed by statements/statements.g4.
 *
 * Function internals are NOT implemented here.
 *
 * The canonical function grammar owns `functionDeclaration`.
 */
functionDeclarationStatement
    : functionDeclaration
    ;


/* ============================================================================
 * 14. MODULE INTEGRATION
 * ========================================================================== */

/**
 * Compatibility adapter consumed by statements/statements.g4.
 *
 * Module syntax is owned by grammar/modules/.
 */
moduleDeclarationStatement
    : moduleDeclaration
    ;


/**
 * Import compatibility adapter.
 */
importStatement
    : importDeclaration
    ;


/**
 * Export compatibility adapter.
 */
exportStatement
    : exportDeclaration
    ;


/* ============================================================================
 * 15. EFFECT INTEGRATION
 * ========================================================================== */

/**
 * Effect declarations belong to the effect grammar.
 *
 * This file only establishes the declaration boundary.
 */
effectDeclarationStatement
    : effectDeclaration
    ;


/* ============================================================================
 * 16. DOMAIN DECLARATION INTEGRATION
 * ========================================================================== */

/**
 * Domain declarations are admitted without embedding machine assumptions.
 *
 * The specialized domain grammar owns each declaration's syntax.
 */
domainDeclarationStatement
    : quantumDeclaration
    | hardwareDeclaration
    | hdlDeclaration
    | distributedDeclaration
    | acceleratorDeclaration
    | dataDeclaration
    | aiDeclaration
    ;


/* ============================================================================
 * 17. QUANTUM DECLARATION INTEGRATION
 * ========================================================================== */

/**
 * Quantum declarations are syntax-level declarations only.
 *
 * This file does not construct quantum::ir.
 *
 * Examples of downstream semantic targets include:
 *
 *     logical qubit abstractions
 *     quantum functions
 *     circuit declarations
 *     observables
 *     quantum resources
 *
 * Physical qubit allocation, topology, gate availability, calibration and
 * backend selection remain outside this grammar.
 */
quantumDeclaration
    : quantumDeclarationBody
    ;


quantumDeclarationBody
    : quantumNamedDeclaration
    ;


quantumNamedDeclaration
    : identifier
      genericParameters?
      quantumDeclarationPayload?
    ;


quantumDeclarationPayload
    : ASSIGN
      expression
      declarationTerminator
    | LBRACE
      statement*
      RBRACE
    ;


/* ============================================================================
 * 18. HARDWARE DECLARATION INTEGRATION
 * ========================================================================== */

/**
 * Hardware declarations describe hardware intent/capability at source level.
 *
 * They MUST NOT encode a fixed number of:
 *
 *     cores
 *     devices
 *     qubits
 *     lanes
 *     registers
 *     memory bytes
 *
 * Such information belongs to resource/target/capability descriptions.
 */
hardwareDeclaration
    : hardwareDeclarationBody
    ;


hardwareDeclarationBody
    : identifier
      genericParameters?
      hardwareDeclarationPayload?
    ;


hardwareDeclarationPayload
    : ASSIGN
      expression
      declarationTerminator
    | LBRACE
      statement*
      RBRACE
    ;


/* ============================================================================
 * 19. HDL DECLARATION INTEGRATION
 * ========================================================================== */

/**
 * HDL declarations are delegated to grammar/hdl/.
 *
 * This adapter prevents HDL constructs from leaking into ordinary type
 * declarations.
 */
hdlDeclaration
    : hdlDeclarationBody
    ;


hdlDeclarationBody
    : identifier
      genericParameters?
      LBRACE
      statement*
      RBRACE
    ;


/* ============================================================================
 * 20. DISTRIBUTED DECLARATION INTEGRATION
 * ========================================================================== */

distributedDeclaration
    : distributedDeclarationBody
    ;


distributedDeclarationBody
    : identifier
      genericParameters?
      distributedDeclarationPayload?
    ;


distributedDeclarationPayload
    : ASSIGN
      expression
      declarationTerminator
    | LBRACE
      statement*
      RBRACE
    ;


/* ============================================================================
 * 21. ACCELERATOR DECLARATION INTEGRATION
 * ========================================================================== */

acceleratorDeclaration
    : acceleratorDeclarationBody
    ;


acceleratorDeclarationBody
    : identifier
      genericParameters?
      acceleratorDeclarationPayload?
    ;


acceleratorDeclarationPayload
    : ASSIGN
      expression
      declarationTerminator
    | LBRACE
      statement*
      RBRACE
    ;


/* ============================================================================
 * 22. DATA DECLARATION INTEGRATION
 * ========================================================================== */

dataDeclaration
    : dataDeclarationBody
    ;


dataDeclarationBody
    : identifier
      genericParameters?
      dataDeclarationPayload?
    ;


dataDeclarationPayload
    : ASSIGN
      expression
      declarationTerminator
    | LBRACE
      statement*
      RBRACE
    ;


/* ============================================================================
 * 23. AI DECLARATION INTEGRATION
 * ========================================================================== */

aiDeclaration
    : aiDeclarationBody
    ;


aiDeclarationBody
    : identifier
      genericParameters?
      aiDeclarationPayload?
    ;


aiDeclarationPayload
    : ASSIGN
      expression
      declarationTerminator
    | LBRACE
      statement*
      RBRACE
    ;


/* ============================================================================
 * 24. SHARED NAME INTEGRATION
 * ========================================================================== */

/**
 * Canonical identifier bridge.
 *
 * Identifier spelling remains owned by ZamaniLexer.
 *
 * The parser must not impose an identifier length limit.
 */
identifier
    : IDENTIFIER
    ;


/**
 * Qualified name.
 *
 * Namespace depth is deliberately unbounded by grammar structure.
 */
qualifiedName
    : identifier
      (DOUBLE_COLON identifier)*
    ;


/* ============================================================================
 * 25. SHARED GENERIC INTEGRATION
 * ========================================================================== */

/**
 * Generic parameter list.
 *
 * The detailed type-system implementation may replace this adapter with the
 * canonical grammar/types generic-parameter rule when the modular grammar
 * composition is finalized.
 *
 * No finite generic-parameter limit is encoded.
 */
genericParameters
    : LESS_THAN
      genericParameter
      (COMMA genericParameter)*
      COMMA?
      GREATER_THAN
    ;


genericParameter
    : identifier
      genericParameterBound*
    ;


genericParameterBound
    : COLON
      typeExpression
    ;


/* ============================================================================
 * 26. WHERE CLAUSES
 * ========================================================================== */

whereClause
    : WHERE
      wherePredicate
      (COMMA wherePredicate)*
    ;


wherePredicate
    : identifier
      COLON
      typeExpression
    ;


/* ============================================================================
 * 27. ERROR-RESISTANCE / AMBIGUITY CONTRACT
 * ========================================================================== */

/*
 * The following architectural rules are intentional:
 *
 * 1. A declaration keyword is always consumed before generic identifier
 *    alternatives.
 *
 * 2. Named types are not lexically classified as machine-specific.
 *
 * 3. Generic parameter lists use the canonical `< ... >` token sequence.
 *
 * 4. No semantic predicate is used to resolve declaration meaning.
 *
 * 5. No declaration rule examines hardware capabilities.
 *
 * 6. No declaration rule queries resource availability.
 *
 * 7. No declaration rule constructs IR.
 *
 * 8. No declaration rule executes code.
 *
 * 9. No declaration rule selects a backend.
 *
 * 10. No declaration rule contains a fixed machine limit.
 */


/* ============================================================================
 * 28. INTEGRATION CONTRACT SUMMARY
 * ========================================================================== */

/*
 * statements/statements.g4
 * ------------------------
 *
 * MUST consume:
 *
 *     declarationStatement
 *
 * It remains the owner of the overall `statement` rule.
 *
 *
 * grammar/types/*
 * ---------------
 *
 * MUST provide:
 *
 *     typeExpression
 *
 * This file does not redefine the canonical type-expression language.
 *
 *
 * grammar/functions/*
 * -------------------
 *
 * MUST provide:
 *
 *     functionDeclaration
 *
 * This file only exposes:
 *
 *     functionDeclarationStatement
 *
 *
 * grammar/modules/*
 * -----------------
 *
 * MUST provide:
 *
 *     moduleDeclaration
 *     importDeclaration
 *     exportDeclaration
 *
 *
 * grammar/core/*
 * --------------
 *
 * MUST provide reusable:
 *
 *     annotations
 *     metadata
 *     names
 *     paths
 *     constraints
 *
 *
 * grammar/quantum/*
 * -----------------
 *
 * MUST eventually provide the authoritative quantum declaration syntax.
 *
 * This file's quantum declaration adapter is intentionally not the owner of
 * quantum semantics.
 *
 *
 * grammar/hardware/*
 * ------------------
 *
 * Owns hardware declaration semantics/syntax.
 *
 *
 * grammar/hdl/*
 * --------------
 *
 * Owns HDL declaration syntax.
 *
 *
 * semantic analysis
 * -----------------
 *
 * Consumes the AST produced from these productions and performs:
 *
 *     name resolution
 *     duplicate detection
 *     visibility validation
 *     generic validation
 *     type checking
 *     ownership checking
 *     effect checking
 *     capability checking
 *     resource checking
 *     domain validation
 *
 *
 * canonical IR
 * ------------
 *
 * Receives semantic representations only.
 *
 * This grammar never directly creates:
 *
 *     quantum::ir
 *
 * QEC
 *
 * ZQN
 *
 * scheduling
 *
 * routing
 *
 * hardware execution plans
 *
 *
 * compiler/runtime
 * ----------------
 *
 * Consume lowered semantic/IR representations rather than parser nodes.
 */


/* ============================================================================
 * 29. SCALABILITY CONTRACT
 * ========================================================================== */

/*
 * This grammar intentionally has no declarations such as:
 *
 *     MAX_FIELDS
 *     MAX_VARIANTS
 *     MAX_GENERIC_PARAMETERS
 *     MAX_TYPES
 *     MAX_DECLARATIONS
 *     MAX_CLASSES
 *     MAX_TRAITS
 *     MAX_INTERFACES
 *     MAX_DEVICES
 *     MAX_QUBITS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_MEMORY
 *
 * Therefore:
 *
 *     small source
 *
 * and:
 *
 *     extremely large source
 *
 * have the same language-level declaration model.
 *
 * Practical resource exhaustion is an implementation concern and MUST be
 * represented as an explicit parser/compiler resource policy rather than
 * silently changing the language grammar.
 */


/* ============================================================================
 * 30. HARDWARE-INDEPENDENCE CONTRACT
 * ========================================================================== */

/*
 * Invalid design:
 *
 *     struct GPU256 { ... }
 *
 * because the grammar assumes a particular GPU width.
 *
 * Valid design:
 *
 *     struct Accelerator<T> { ... }
 *
 * with target/resource constraints supplied separately.
 *
 *
 * Invalid:
 *
 *     quantum_register<32>
 *
 * if `32` is being imposed by the language as a machine limit.
 *
 * Valid:
 *
 *     quantum_register<N>
 *
 * where N is a semantic/resource parameter whose validity is determined
 * downstream.
 *
 *
 * The grammar describes the declaration.
 *
 * The compiler decides the realization.
 */


/* ============================================================================
 * 31. AST CONTRACT
 * ========================================================================== */

/*
 * Every declaration production must lower to a typed frontend-AST declaration
 * node preserving at least:
 *
 *     declaration kind
 *     source span
 *     name
 *     visibility
 *     modifiers
 *     generic parameters
 *     constraints
 *     declared type
 *     initializer/body
 *     attributes/metadata
 *
 * The AST must preserve source structure without embedding target-specific
 * realization.
 *
 * Source spans must remain available for deterministic diagnostics.
 */


/* ============================================================================
 * 32. SEMANTIC CONTRACT
 * ========================================================================== */

/*
 * Parser acceptance MUST NOT imply semantic validity.
 *
 * Examples:
 *
 *     duplicate field names
 *     recursive illegal types
 *     invalid generic constraints
 *     illegal visibility
 *     conflicting modifiers
 *     invalid implementation
 *     unresolved type
 *     illegal effect
 *     unsupported capability
 *
 * are semantic diagnostics.
 *
 * They must not be solved by adding target-specific grammar restrictions.
 */


/* ============================================================================
 * 33. TEST CONTRACT
 * ========================================================================== */

/*
 * Positive tests MUST cover:
 *
 *     let x = value;
 *     var x: T;
 *     const x: T = value;
 *
 *     type A = B;
 *     type A<T> = B<T>;
 *
 *     struct A { field: T }
 *     enum A { One, Two(T), Three { value: T } }
 *     record A { field: T }
 *
 *     class A { ... }
 *     interface A { ... }
 *     trait A { ... }
 *     impl A { ... }
 *     impl Trait for A { ... }
 *
 * Generic declarations of arbitrary syntactic arity must be accepted.
 *
 * Negative tests MUST cover:
 *
 *     missing declaration name
 *     missing type
 *     missing initializer on const
 *     missing `=`
 *     malformed generic list
 *     malformed where clause
 *     malformed struct field
 *     malformed enum variant
 *     malformed implementation target
 *     duplicate delimiters
 *
 * Boundary tests MUST cover:
 *
 *     empty declaration bodies
 *     one-field declarations
 *     nested declarations
 *     deeply nested syntactic structures
 *     very large declaration lists
 *
 * Cross-domain tests MUST cover declarations that later combine with:
 *
 *     classical
 *     quantum
 *     HDL
 *     hardware
 *     distributed
 *     AI
 *     data
 *     networking
 *
 * The grammar must not introduce a domain-specific machine limit.
 */


/* ============================================================================
 * 34. COMPLETION CRITERIA
 * ========================================================================== */

/*
 * This file is complete only when:
 *
 * [ ] `declarationStatement` is the sole declaration dispatch entry point.
 *
 * [ ] statements/statements.g4 consumes this rule without redefining it.
 *
 * [ ] Function syntax is delegated to the canonical functions grammar.
 *
 * [ ] Module syntax is delegated to the canonical modules grammar.
 *
 * [ ] Type-expression syntax is delegated to grammar/types.
 *
 * [ ] Identifier spelling is delegated to ZamaniLexer.
 *
 * [ ] No lexer rules occur in this file.
 *
 * [ ] No semantic predicates occur in this file.
 *
 * [ ] No Rust actions occur in this file.
 *
 * [ ] No unsafe implementation is required.
 *
 * [ ] No hardware limit occurs in this file.
 *
 * [ ] No quantum-count limit occurs in this file.
 *
 * [ ] No CPU/GPU/FPGA/QPU count occurs in this file.
 *
 * [ ] No fixed memory/resource capacity occurs in this file.
 *
 * [ ] AST mapping exists for every declaration alternative.
 *
 * [ ] Semantic diagnostics are downstream.
 *
 * [ ] Declaration source spans are preserved.
 *
 * [ ] Positive tests exist.
 *
 * [ ] Negative tests exist.
 *
 * [ ] Boundary tests exist.
 *
 * [ ] Cross-domain tests exist.
 *
 * [ ] Parser ambiguity diagnostics are clean.
 *
 * [ ] Generated parser code builds under the project's Rust 1.97 /
 *     Rust 1.97.1 baseline.
 *
 * [ ] The compiler remains free of Rust `unsafe`.
 *
 * [ ] `quantum::ir` remains a downstream canonical semantic boundary.
 *
 * [ ] QEC, ZQN, scheduling, routing, optimization, hardware discovery and
 *     runtime remain downstream consumers rather than grammar dependencies.
 */
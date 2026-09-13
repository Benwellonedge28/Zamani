/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/declarations/traits.g4
 *
 * Grammar:
 *     ZamaniTraits
 *
 * Purpose:
 *     Canonical parser grammar for trait declarations.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *                         Zamani source
 *                              |
 *                              v
 *                       canonical lexer
 *                              |
 *                              v
 *                       canonical parser
 *                              |
 *              +---------------+----------------+
 *              |               |                |
 *              v               v                v
 *        declarations       types           functions
 *              |
 *              v
 *        ZamaniTraits
 *              |
 *              v
 *             AST
 *              |
 *              v
 *       semantic analysis
 *              |
 *       +------+------+------------------+
 *       |             |                  |
 *       v             v                  v
 *   type system   capability/effect   resource analysis
 *       |
 *       v
 *   canonical semantic IR
 *       |
 *       +-------------------+
 *       |                   |
 *       v                   v
 * classical / quantum / hardware / distributed / future domains
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - trait declaration syntax;
 *   - trait names;
 *   - trait generic parameter references;
 *   - trait inheritance syntax;
 *   - trait body syntax;
 *   - trait member dispatch;
 *   - trait method signatures;
 *   - trait default method bodies;
 *   - trait associated types;
 *   - trait associated constants;
 *   - trait properties;
 *   - trait requirements;
 *   - trait member attributes;
 *   - trait-local declarations that are explicitly permitted by the
 *     language contract.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - lexical tokens;
 *   - identifiers;
 *   - generic parameter declaration syntax;
 *   - type-expression syntax;
 *   - function parameter syntax;
 *   - function return-type syntax;
 *   - effect declaration syntax;
 *   - capability declaration syntax;
 *   - resource discovery;
 *   - hardware discovery;
 *   - quantum IR;
 *   - classical IR;
 *   - QEC;
 *   - ZQN;
 *   - routing;
 *   - scheduling;
 *   - optimization;
 *   - backend selection;
 *   - runtime execution;
 *   - implementation resolution;
 *   - trait coherence;
 *   - trait method dispatch;
 *   - type inference;
 *   - constraint solving.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Traits describe reusable semantic contracts.
 *
 * They MUST NOT encode:
 *
 *   - fixed CPU counts;
 *   - fixed core counts;
 *   - fixed thread counts;
 *   - fixed GPU counts;
 *   - fixed FPGA counts;
 *   - fixed qubit counts;
 *   - fixed register counts;
 *   - fixed memory capacities;
 *   - fixed network sizes;
 *   - fixed machine topology;
 *   - device identifiers;
 *   - physical addresses;
 *   - backend-specific resource limits.
 *
 * Trait syntax describes PROGRAM CAPABILITY and SEMANTIC REQUIREMENTS.
 *
 * Physical realization is selected downstream by:
 *
 *   capability analysis
 *   resource analysis
 *   target selection
 *   compilation
 *   routing
 *   scheduling
 *   hardware abstraction
 *   runtime
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Traits may describe contracts involving quantum types, operations,
 * capabilities, or resources.
 *
 * This grammar MUST NOT create a quantum IR.
 *
 * If a trait contains a quantum-related type or requirement:
 *
 *     source
 *       -> trait AST
 *       -> semantic analysis
 *       -> canonical quantum semantic representation
 *       -> quantum::ir
 *
 * `quantum::ir` remains the canonical quantum semantic boundary.
 *
 * This grammar therefore has NO direct dependency on:
 *
 *     quantum::ir
 *     QEC
 *     ZQN
 *     routing
 *     scheduling
 *     hardware HAL
 *     quantum optimization
 *
 * ============================================================================
 * RUST / SAFETY
 * ============================================================================
 *
 * The grammar contains no embedded Rust actions.
 *
 * The generated compiler/parser implementation MUST remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *
 * No unsafe Rust is required or permitted by this grammar architecture.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * Required parser-level contracts:
 *
 *     identifier
 *     qualifiedName
 *     genericParameterList
 *     typeReference
 *     parameterList
 *     functionReturnClause
 *     effectClause
 *     contractClause
 *     block
 *     attribute
 *     typeConstraintClause
 *     typeInheritanceClause
 *
 * These rules are OWNED by their respective canonical grammar components.
 *
 * They MUST NOT be duplicated here.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * The canonical declaration grammar should import/compose this grammar and
 * retain only the declaration dispatch:
 *
 *     | traitDeclaration
 *
 * Existing trait rules in declarations.g4 MUST be removed after this delegate
 * becomes authoritative.
 *
 * No second trait syntax may remain active in declarations.g4.
 *
 * ============================================================================
 */

parser grammar ZamaniTraits;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * 1. TRAIT DECLARATION
 * ============================================================================
 *
 * Canonical form:
 *
 *     trait Name {
 *         ...
 *     }
 *
 * Generic traits:
 *
 *     trait Name<T> {
 *         ...
 *     }
 *
 * Inherited traits:
 *
 *     trait Child extends Parent {
 *         ...
 *     }
 *
 * Multiple inheritance:
 *
 *     trait Child extends ParentA, ParentB {
 *         ...
 *     }
 *
 * The grammar does not impose an inheritance-count limit.
 *
 * Semantic analysis is responsible for:
 *
 *     - duplicate bases;
 *     - inheritance cycles;
 *     - incompatible inherited contracts;
 *     - visibility;
 *     - generic compatibility;
 *     - associated-type conflicts;
 *     - method conflicts.
 */
traitDeclaration
    : declarationModifiers
      TRAIT
      identifier
      genericParameterList?
      traitInheritanceClause?
      traitWhereClause?
      traitBody
    ;


/*
 * ============================================================================
 * 2. TRAIT INHERITANCE
 * ============================================================================
 *
 * Inheritance is a semantic relationship.
 *
 * It does not imply:
 *
 *     - implementation inheritance;
 *     - hardware inheritance;
 *     - device selection;
 *     - runtime dispatch strategy.
 */
traitInheritanceClause
    : EXTENDS
      traitSuperTypeList
    ;

traitSuperTypeList
    : traitSuperType
      (
          COMMA
          traitSuperType
      )*
    ;

traitSuperType
    : typeReference
    ;


/*
 * ============================================================================
 * 3. TRAIT WHERE CLAUSE
 * ============================================================================
 *
 * Constraints are preserved syntactically and interpreted semantically.
 *
 * Example:
 *
 *     trait Storage<T>
 *     where
 *         T: Serializable
 *     {
 *         ...
 *     }
 *
 * This grammar does not solve constraints.
 */
traitWhereClause
    : WHERE
      typeConstraintList
    ;


/*
 * ============================================================================
 * 4. TRAIT BODY
 * ============================================================================
 */

traitBody
    : LBRACE
      traitMember*
      RBRACE
    ;


/*
 * ============================================================================
 * 5. TRAIT MEMBER
 * ============================================================================
 *
 * A trait member is deliberately explicit.
 *
 * This avoids the dangerous pattern:
 *
 *     traitMember : declaration ;
 *
 * which would accidentally allow arbitrary declarations and create ambiguous
 * ownership between traits, modules, classes, implementations, effects,
 * resources, and other declaration domains.
 */
traitMember
    : traitMemberAttributes traitMethodDeclaration
    | traitMemberAttributes traitAssociatedTypeDeclaration
    | traitMemberAttributes traitAssociatedConstantDeclaration
    | traitMemberAttributes traitPropertyDeclaration
    | traitMemberAttributes traitNestedTypeDeclaration
    ;


/*
 * ============================================================================
 * 6. TRAIT MEMBER ATTRIBUTES
 * ============================================================================
 */

traitMemberAttributes
    : attribute*
    ;


/*
 * ============================================================================
 * 7. TRAIT METHOD
 * ============================================================================
 *
 * A trait method can be:
 *
 *     - a required method;
 *     - a default method.
 *
 * Required:
 *
 *     fn execute(input: Input) -> Output;
 *
 * Default:
 *
 *     fn execute(input: Input) -> Output {
 *         ...
 *     }
 *
 * A trait method body is NOT interpreted here.
 *
 * It is passed to the normal block/statement grammar.
 *
 * This preserves one canonical statement grammar.
 */
traitMethodDeclaration
    : traitMethodModifiers?
      FN
      identifier
      genericParameterList?
      LPAREN
      parameterList?
      RPAREN
      functionReturnClause?
      effectClause?
      contractClause?
      traitMethodTermination
    ;

traitMethodModifiers
    : declarationModifiers
    ;

traitMethodTermination
    : SEMICOLON
    | block
    ;


/*
 * ============================================================================
 * 8. TRAIT ASSOCIATED TYPE
 * ============================================================================
 *
 * Example:
 *
 *     type Item;
 *
 * With a bound:
 *
 *     type Item: Serializable;
 *
 * Multiple constraints remain a semantic concern.
 */
traitAssociatedTypeDeclaration
    : TYPE
      identifier
      traitAssociatedTypeConstraint?
      SEMICOLON
    ;

traitAssociatedTypeConstraint
    : COLON
      typeConstraintBoundList
    ;

typeConstraintBoundList
    : typeReference
      (
          PLUS
          typeReference
      )*
    ;


/*
 * ============================================================================
 * 9. TRAIT ASSOCIATED CONSTANT
 * ============================================================================
 *
 * Example:
 *
 *     const VERSION: Version;
 *
 * A trait-associated constant may be required:
 *
 *     const VERSION: Version;
 *
 * or given a default:
 *
 *     const VERSION: Version = 1;
 *
 * The value is parsed as an expression but not evaluated here.
 *
 * Evaluation belongs to semantic analysis / compile-time evaluation.
 */
traitAssociatedConstantDeclaration
    : CONST
      identifier
      COLON
      typeReference
      traitAssociatedConstantInitializer?
      SEMICOLON
    ;

traitAssociatedConstantInitializer
    : ASSIGN
      expression
    ;


/*
 * ============================================================================
 * 10. TRAIT PROPERTY
 * ============================================================================
 *
 * A property is a semantic contract, not a storage declaration.
 *
 * Example:
 *
 *     property state: State;
 *
 * The grammar therefore does NOT imply:
 *
 *     - memory layout;
 *     - register allocation;
 *     - hardware storage;
 *     - physical location;
 *     - cache placement.
 *
 * Access semantics are explicit.
 */
traitPropertyDeclaration
    : declarationModifiers?
      PROPERTY
      identifier
      COLON
      typeReference
      traitPropertyAccessorBlock?
      SEMICOLON?
    ;

traitPropertyAccessorBlock
    : LBRACE
      traitPropertyAccessor+
      RBRACE
    ;

traitPropertyAccessor
    : GET
      SEMICOLON
    | SET
      SEMICOLON
    ;


/*
 * ============================================================================
 * 11. NESTED TYPE DECLARATIONS
 * ============================================================================
 *
 * Traits may contain nested semantic type declarations where supported by the
 * language specification.
 *
 * The nested declaration is intentionally restricted.
 *
 * It must not recurse into the complete top-level declaration dispatcher.
 *
 * This prevents accidental declarations such as:
 *
 *     module
 *     package
 *     import
 *     hardware device
 *     deployment
 *
 * from appearing inside a trait.
 *
 * The concrete nested type grammars remain authoritative.
 */
traitNestedTypeDeclaration
    : traitNestedTypeAlias
    | traitNestedStruct
    | traitNestedEnum
    | traitNestedUnion
    | traitNestedInterface
    | traitNestedTrait
    ;

traitNestedTypeAlias
    : declarationModifiers
      TYPE
      identifier
      genericParameterList?
      ASSIGN
      typeReference
      SEMICOLON
    ;

traitNestedStruct
    : declarationModifiers
      STRUCT
      identifier
      genericParameterList?
      typeInheritanceClause?
      traitNestedStructBody
    ;

traitNestedStructBody
    : LBRACE
      traitNestedStructMember*
      RBRACE
    ;

traitNestedStructMember
    : traitMemberAttributes traitNestedField
    ;

traitNestedField
    : identifier
      COLON
      typeReference
      traitNestedFieldInitializer?
      SEMICOLON
    ;

traitNestedFieldInitializer
    : ASSIGN
      expression
    ;

traitNestedEnum
    : declarationModifiers
      ENUM
      identifier
      genericParameterList?
      traitNestedEnumBody
    ;

traitNestedEnumBody
    : LBRACE
      traitNestedEnumVariant*
      RBRACE
    ;

traitNestedEnumVariant
    : attribute*
      identifier
      traitNestedEnumVariantPayload?
      COMMA?
    ;

traitNestedEnumVariantPayload
    : LPAREN
      parameterList?
      RPAREN
    | LBRACE
      traitNestedEnumField*
      RBRACE
    ;

traitNestedEnumField
    : identifier
      COLON
      typeReference
      SEMICOLON
    ;

traitNestedUnion
    : declarationModifiers
      UNION
      identifier
      genericParameterList?
      ASSIGN
      traitNestedUnionVariants
      SEMICOLON
    ;

traitNestedUnionVariants
    : traitNestedUnionVariant
      (
          PIPE
          traitNestedUnionVariant
      )*
    ;

traitNestedUnionVariant
    : attribute*
      identifier
      traitNestedUnionVariantPayload?
    ;

traitNestedUnionVariantPayload
    : LPAREN
      parameterList?
      RPAREN
    | LBRACE
      traitNestedStructMember*
      RBRACE
    ;

traitNestedInterface
    : declarationModifiers
      INTERFACE
      identifier
      genericParameterList?
      interfaceInheritanceClause?
      LBRACE
      traitNestedInterfaceMember*
      RBRACE
    ;

traitNestedInterfaceMember
    : traitMemberAttributes
      traitNestedInterfaceMemberCore
    ;

traitNestedInterfaceMemberCore
    : traitMethodDeclaration
    | traitAssociatedTypeDeclaration
    | traitAssociatedConstantDeclaration
    | traitPropertyDeclaration
    ;

traitNestedTrait
    : traitDeclaration
    ;


/*
 * ============================================================================
 * 12. TRAIT SEMANTIC CONTRACT
 * ============================================================================
 *
 * The following are intentionally NOT grammar rules.
 *
 * They are semantic obligations for the frontend:
 *
 *     - trait names must resolve correctly;
 *     - trait generic parameters must bind correctly;
 *     - inherited traits must exist;
 *     - inheritance cycles must be rejected;
 *     - inherited members must be merged deterministically;
 *     - conflicting methods must be diagnosed;
 *     - conflicting associated types must be diagnosed;
 *     - conflicting constants must be diagnosed;
 *     - property compatibility must be checked;
 *     - method parameter types must be checked;
 *     - return types must be checked;
 *     - effects must be checked;
 *     - contracts must be checked;
 *     - generic constraints must be solved;
 *     - implementations must satisfy all required members;
 *     - default implementations must remain semantically valid;
 *     - visibility must be respected.
 *
 * None of these rules should be encoded as parser predicates.
 */


/*
 * ============================================================================
 * 13. EFFECT INTEGRATION
 * ============================================================================
 *
 * A trait method may declare effects.
 *
 * Example:
 *
 *     fn execute() -> Result
 *         effects { ... };
 *
 * The exact effect syntax is owned by grammar/effects/.
 *
 * This file only consumes `effectClause`.
 *
 * Trait grammar must never redefine:
 *
 *     effect
 *     effect sets
 *     effect handlers
 *     IO
 *     quantum effects
 *     network effects
 *     hardware effects
 */


/*
 * ============================================================================
 * 14. CONTRACT INTEGRATION
 * ============================================================================
 *
 * A trait method may declare semantic contracts.
 *
 * Example concepts:
 *
 *     requires ...
 *     ensures ...
 *     invariant ...
 *
 * The contract grammar owns their expression.
 *
 * Trait syntax merely provides the attachment point.
 */


/*
 * ============================================================================
 * 15. GENERIC INTEGRATION
 * ============================================================================
 *
 * Generic declaration syntax is owned by the generic subsystem.
 *
 * This file consumes:
 *
 *     genericParameterList
 *
 * It MUST NOT duplicate generic parameter grammar.
 *
 * Generic applications are likewise owned by the canonical type grammar.
 *
 * This prevents:
 *
 *     traits.g4 -> generic-types.g4 -> traits.g4
 *
 * dependency cycles.
 */


/*
 * ============================================================================
 * 16. TYPE INTEGRATION
 * ============================================================================
 *
 * All trait type positions use:
 *
 *     typeReference
 *
 * rather than defining a private trait type system.
 *
 * This allows trait contracts to use:
 *
 *     classical types
 *     quantum types
 *     hardware abstractions
 *     accelerator types
 *     distributed types
 *     data types
 *     future types
 *
 * without changing this grammar.
 */


/*
 * ============================================================================
 * 17. QUANTUM INTEGRATION
 * ============================================================================
 *
 * Examples of semantically legal future trait contracts include:
 *
 *     trait QuantumOperation {
 *         fn apply(...);
 *     }
 *
 *     trait QuantumResource {
 *         type State;
 *         fn measure(...) -> Result;
 *     }
 *
 *     trait ErrorCorrectable {
 *         ...
 *     }
 *
 * The grammar does NOT define:
 *
 *     qubit counts
 *     physical qubit identifiers
 *     topology
 *     gate durations
 *     calibration
 *     QPU selection
 *     error rates
 *     noise models
 *     QEC algorithms
 *
 * Such information belongs downstream.
 *
 * In particular:
 *
 *     traits.g4
 *          |
 *          v
 *       trait AST
 *          |
 *          v
 *    semantic analysis
 *          |
 *          v
 *     quantum::ir
 *
 * and never:
 *
 *     traits.g4 -> quantum::ir
 */


/*
 * ============================================================================
 * 18. HARDWARE / HDL INTEGRATION
 * ============================================================================
 *
 * A trait may describe a hardware-independent capability.
 *
 * For example:
 *
 *     trait Accelerator {
 *         ...
 *     }
 *
 *     trait StreamProcessor<T> {
 *         ...
 *     }
 *
 * But this grammar must never turn such a trait into:
 *
 *     use GPU 0
 *     use 8 cores
 *     use device X
 *     use FPGA Y
 *
 * Hardware selection belongs to:
 *
 *     capabilities
 *     requirements
 *     resources
 *     targets
 *     compilation
 *     hardware abstraction
 */


/*
 * ============================================================================
 * 19. RESOURCE INTEGRATION
 * ============================================================================
 *
 * Traits may participate in capability and resource semantics through:
 *
 *     type constraints
 *     effect clauses
 *     contract clauses
 *     attributes
 *
 * Resource quantities remain semantic expressions.
 *
 * This grammar does not impose any resource maximum.
 *
 * Examples:
 *
 *     Resource
 *     QuantumResource
 *     MemoryResource
 *     AcceleratorResource
 *
 * remain types/capabilities rather than hard-coded grammar constructs.
 */


/*
 * ============================================================================
 * 20. IMPLEMENTATION INTEGRATION
 * ============================================================================
 *
 * Trait implementations are NOT owned here.
 *
 * `impl` belongs to declarations/implementations.g4.
 *
 * That grammar consumes the canonical:
 *
 *     traitDeclaration
 *
 * semantic identity and resolves:
 *
 *     impl Trait for Type
 *
 * relationships.
 *
 * This separation is intentional:
 *
 *     traits.g4
 *          |
 *          +---- declares contract
 *                         |
 *                         v
 *                 implementations.g4
 *                         |
 *                         +---- supplies implementation
 */


/*
 * ============================================================================
 * 21. AST CONTRACT
 * ============================================================================
 *
 * The parser must produce a trait declaration structure containing, at
 * minimum, semantic-preserving source information for:
 *
 *     TraitDecl
 *       name
 *       modifiers
 *       genericParameters
 *       superTraits
 *       whereClause
 *       members
 *
 * Member variants:
 *
 *     TraitMethod
 *       name
 *       modifiers
 *       genericParameters
 *       parameters
 *       returnType
 *       effects
 *       contracts
 *       optionalDefaultBody
 *
 *     TraitAssociatedType
 *       name
 *       bounds
 *
 *     TraitAssociatedConstant
 *       name
 *       type
 *       optionalInitializer
 *
 *     TraitProperty
 *       name
 *       type
 *       accessors
 *
 *     TraitNestedType
 *       nested declaration
 *
 * The AST layer owns the concrete Rust representation.
 *
 * This grammar must not introduce a competing AST.
 */


/*
 * ============================================================================
 * 22. DETERMINISM
 * ============================================================================
 *
 * Parsing must be deterministic for the same token stream.
 *
 * Member alternatives are deliberately separated:
 *
 *     method
 *     associated type
 *     associated constant
 *     property
 *     nested type
 *
 * rather than allowing an unrestricted declaration fallback.
 *
 * No semantic lookup is required to decide the fundamental trait-member
 * alternative.
 */


/*
 * ============================================================================
 * 23. SCALABILITY
 * ============================================================================
 *
 * There is deliberately NO grammar-level maximum for:
 *
 *     trait count
 *     member count
 *     generic parameter count
 *     inheritance-list length
 *     type nesting
 *     source size
 *     implementation count
 *     associated types
 *     associated constants
 *
 * Practical limits belong to explicit compiler/resource policy.
 *
 * They must not become language semantics.
 */


/*
 * ============================================================================
 * 24. COMPATIBILITY
 * ============================================================================
 *
 * Existing syntax preserved:
 *
 *     trait Name { ... }
 *     trait Name<T> { ... }
 *     trait Child extends Parent { ... }
 *     fn method(...);
 *     fn method(...) { ... }
 *     type Item;
 *     const VALUE: Type = expression;
 *
 * Existing consumers should continue to refer to:
 *
 *     traitDeclaration
 *     traitBody
 *     traitMember
 *     traitMethodDeclaration
 *     traitAssociatedType
 *     traitConstantDeclaration
 *
 * during migration.
 *
 * Compatibility aliases MAY be retained temporarily in the composition
 * grammar, but duplicate active definitions MUST NOT remain.
 */


/*
 * ============================================================================
 * 25. VALIDATION / ERROR OWNERSHIP
 * ============================================================================
 *
 * Syntax errors belong to the parser.
 *
 * Examples:
 *
 *     trait { ... }
 *     trait Name(
 *     trait Name { fn }
 *     trait Name { type ; }
 *
 * Semantic errors belong downstream.
 *
 * Examples:
 *
 *     cyclic inheritance
 *     duplicate member
 *     conflicting inherited method
 *     invalid associated-type bound
 *     incompatible implementation
 *     unsatisfied generic constraint
 *     invalid effect relationship
 *     unavailable capability
 *
 * Do not encode semantic diagnostics as grammar actions.
 */


/*
 * ============================================================================
 * 26. SECURITY / SAFETY
 * ============================================================================
 *
 * Trait declarations cannot grant themselves:
 *
 *     unsafe execution
 *     hardware access
 *     network access
 *     filesystem access
 *     privileged capabilities
 *     quantum backend access
 *
 * Such authority must be represented through the canonical effect/capability
 * and security systems.
 *
 * In particular:
 *
 *     trait -> capability
 *
 * does not mean:
 *
 *     trait -> automatic permission.
 *
 * Capability checking remains a semantic/compiler concern.
 */


/*
 * ============================================================================
 * 27. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is COMPLETE only when:
 *
 * [ ] It is the single authoritative trait parser delegate.
 *
 * [ ] No duplicate trait parser definitions remain active in
 *     declarations.g4.
 *
 * [ ] `traitDeclaration` remains the stable public declaration entry point.
 *
 * [ ] Generic parameters come from the canonical generic grammar.
 *
 * [ ] Types come from the canonical type grammar.
 *
 * [ ] Parameters come from the canonical function grammar.
 *
 * [ ] Blocks come from the canonical statement grammar.
 *
 * [ ] Effects come from the canonical effect grammar.
 *
 * [ ] Contracts come from the canonical contract grammar.
 *
 * [ ] Attributes come from the canonical attribute grammar.
 *
 * [ ] No lexer rules are duplicated here.
 *
 * [ ] No hardware limits are encoded.
 *
 * [ ] No quantum machine assumptions are encoded.
 *
 * [ ] No direct dependency on quantum::ir exists.
 *
 * [ ] No QEC/ZQN/routing/scheduling implementation is embedded.
 *
 * [ ] No Rust action code exists.
 *
 * [ ] Generated Rust remains safe and compatible with Rust 1.97/1.97.1.
 *
 * [ ] Required positive tests exist.
 *
 * [ ] Required negative tests exist.
 *
 * [ ] Boundary/scalability tests exist.
 *
 * [ ] Cross-domain tests exist.
 *
 * [ ] Parser determinism tests exist.
 *
 * [ ] AST lowering tests exist.
 *
 * [ ] Trait-to-implementation semantic tests exist.
 *
 * [ ] Quantum trait contracts reach canonical quantum semantic lowering
 *     without creating another quantum IR.
 *
 * [ ] Classical, HDL, hardware, distributed, AI, and future types can appear
 *     through the canonical type system without modifying this grammar.
 */
/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/declarations/classes.g4
 *
 * Grammar:
 *     ZamaniClasses
 *
 * Role:
 *     AUTHORITATIVE SOURCE-LEVEL CLASS DECLARATION GRAMMAR
 *
 * Status:
 *     Production-target modular parser grammar
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     This grammar contains no embedded Rust.
 *     No actions.
 *     No semantic predicates.
 *     No filesystem access.
 *     No network access.
 *     No hardware discovery.
 *     No runtime access.
 *     No unsafe Rust is required.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the concrete syntax of Zamani `class` declarations.
 *
 * It is the ONE concrete grammar owner for:
 *
 *     classDeclaration
 *     class inheritance
 *     class implemented-contract lists
 *     class permits lists
 *     class bodies
 *     class members
 *     class fields
 *     class constructors
 *     class methods
 *     class properties
 *     class-associated types
 *     class-associated constants
 *     class nested type declarations
 *
 * The class declaration is source-level and domain-neutral.
 *
 * A class may eventually represent:
 *
 *     classical computation
 *     quantum abstractions
 *     hybrid computation
 *     HDL/co-design abstractions
 *     hardware/resource abstractions
 *     distributed objects
 *     AI/data abstractions
 *     networking abstractions
 *     security abstractions
 *     future computational domains
 *
 * The parser does not determine the eventual execution domain.
 *
 * ============================================================================
 * NON-OWNERSHIP
 * ============================================================================
 *
 * THIS FILE DOES NOT OWN:
 *
 *     lexer/token definitions
 *     identifier spelling
 *     Unicode identifier policy
 *     qualified-name spelling
 *     generic parameter semantics
 *     type-expression semantics
 *     expression semantics
 *     statement semantics
 *     block semantics
 *     function semantics outside class members
 *     module semantics
 *     namespace semantics
 *     trait semantics
 *     interface semantics
 *     implementation semantics
 *     ownership semantics
 *     borrowing semantics
 *     effect semantics
 *     resource discovery
 *     capability discovery
 *     hardware discovery
 *     hardware selection
 *     physical topology
 *     CPU selection
 *     GPU selection
 *     FPGA selection
 *     QPU selection
 *     qubit allocation
 *     routing
 *     scheduling
 *     calibration
 *     QEC
 *     ZQN
 *     HAL
 *     optimization
 *     runtime execution
 *     ABI selection
 *     object layout
 *     memory layout
 *
 * ============================================================================
 * ARCHITECTURAL PIPELINE
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     canonical parser
 *          |
 *          v
 *     classDeclaration
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     structural validation
 *          |
 *          v
 *     name/type/generic/effect/resource analysis
 *          |
 *          v
 *     canonical semantic model
 *          |
 *          +--> classical representation
 *          +--> quantum semantic representation
 *          +--> quantum::ir
 *          +--> HDL/hardware representation
 *          +--> distributed representation
 *          +--> accelerator representation
 *          +--> future-domain representations
 *          |
 *          v
 *     optimization / lowering
 *          |
 *          v
 *     routing / scheduling / resilience
 *          |
 *          v
 *     target realization
 *          |
 *          v
 *     runtime
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Class syntax MUST describe source-level semantics rather than today's
 * machine characteristics.
 *
 * This grammar therefore contains NO:
 *
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_ASICS
 *     MAX_QPUS
 *     MAX_QUBITS
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_MEMORY
 *     MAX_REGISTERS
 *     MAX_FIELDS
 *     MAX_METHODS
 *     MAX_GENERIC_PARAMETERS
 *     MAX_BASE_TYPES
 *     MAX_INTERFACES
 *     MAX_PERMITTED_TYPES
 *     MAX_NESTING
 *
 * It also contains no:
 *
 *     physical addresses
 *     device identifiers
 *     physical qubit identifiers
 *     fixed topology
 *     fixed accelerator counts
 *     fixed register widths
 *     fixed memory capacities
 *
 * Practical compiler/resource limits belong to explicit configurable
 * implementation policy and MUST NOT become source-language grammar limits.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The canonical AST contract already exists at:
 *
 *     src/frontend/ast/node/declarations/class.rs
 *
 * The grammar must preserve the following source-level information:
 *
 *     name
 *     generic parameters
 *     extends relationships
 *     implements relationships
 *     permits relationships
 *     ordered members
 *     source spans
 *     source order
 *     attributes
 *     modifiers
 *
 * The AST uses canonical NodeId references rather than duplicating child AST
 * nodes.
 *
 * This grammar therefore MUST NOT introduce:
 *
 *     ClassType
 *     QuantumClassType
 *     HardwareClassType
 *     ClassMemberIR
 *
 * or another class-specific semantic representation.
 *
 * ============================================================================
 * QUANTUM CONTRACT
 * ============================================================================
 *
 * A class may contain quantum types, quantum methods, quantum resources,
 * quantum capabilities, or other quantum-related source abstractions.
 *
 * This grammar MUST NOT:
 *
 *     allocate qubits
 *     select physical qubits
 *     select a QPU
 *     select a topology
 *     enumerate hardware gates
 *     perform routing
 *     perform scheduling
 *     perform QEC
 *     implement ZQN
 *     access HAL state
 *     construct quantum::ir
 *
 * The required direction remains:
 *
 *     class syntax
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     quantum semantic representation
 *          |
 *          v
 *     quantum::ir
 *
 * ============================================================================
 * HDL / HARDWARE CONTRACT
 * ============================================================================
 *
 * Classes may abstract hardware/software co-design concepts.
 *
 * The class grammar must not turn a class into a physical machine.
 *
 * For example:
 *
 *     class Accelerator<T> {
 *         resource: T;
 *     }
 *
 * remains source-level abstraction.
 *
 * It does not imply:
 *
 *     GPU 0
 *     FPGA 3
 *     device 7
 *     memory bank 2
 *     physical address
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * The grammar contains:
 *
 *     no actions
 *     no predicates
 *     no randomness
 *     no timestamps
 *     no mutable state
 *     no external I/O
 *
 * For a fixed token stream and grammar version, the class parse structure is
 * deterministic.
 *
 * ============================================================================
 * SOURCE ORDER
 * ============================================================================
 *
 * Source order MUST be preserved for:
 *
 *     generic parameters
 *     extends types
 *     implemented types
 *     permitted types
 *     members
 *
 * The parser MUST NOT:
 *
 *     sort
 *     deduplicate
 *     normalize
 *     reorder
 *
 * source declarations.
 *
 * Duplicate-name and semantic-conflict detection belong downstream.
 *
 * ============================================================================
 * ANTLR CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * The final canonical lexer is expected to expose the vocabulary assembled
 * through:
 *
 *     grammar/lexer/tokens.g4
 *
 * and ultimately consumed by the canonical Zamani lexer.
 *
 * During the repository's lexer migration, all parser delegates MUST converge
 * on ONE token vocabulary. No class-specific lexer may be introduced.
 *
 * ============================================================================
 */

parser grammar ZamaniClasses;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * 1. CLASS DECLARATION
 * ============================================================================
 *
 * Canonical shape:
 *
 *     class Name {
 *     }
 *
 * Generic:
 *
 *     class Box<T> {
 *     }
 *
 * Inheritance:
 *
 *     class Child extends Parent {
 *     }
 *
 * Implemented contracts:
 *
 *     class Device implements Compute, Serializable {
 *     }
 *
 * Permitted types:
 *
 *     class Base permits A, B {
 *     }
 *
 * The parser records syntax only.
 *
 * Semantic analysis determines whether:
 *
 *     - inheritance is legal;
 *     - a base is actually a class;
 *     - an implemented type is an interface/trait;
 *     - permits is applicable;
 *     - generic constraints are satisfied;
 *     - inheritance is cyclic;
 *     - visibility rules hold.
 */

classDeclaration
    : classDeclarationPrefix*
      CLASS
      identifier
      genericParameters?
      classExtendsClause?
      classImplementsClause?
      classPermitsClause?
      classWhereClause?
      classBody
    ;


/*
 * ============================================================================
 * 2. CLASS DECLARATION PREFIX
 * ============================================================================
 *
 * Prefixes are intentionally expressed through the canonical declaration
 * vocabulary.
 *
 * They are syntactic modifiers/attributes.
 *
 * Their legality and semantic combinations are validated downstream.
 *
 * Supported source-level modifiers include:
 *
 *     public
 *     pub
 *     private
 *     protected
 *     internal
 *     abstract
 *     final
 *     sealed
 *     partial
 *     static
 *     virtual
 *     override
 *
 * The grammar does not interpret these.
 */

classDeclarationPrefix
    : classDeclarationAttribute
    | classDeclarationModifier
    ;


classDeclarationAttribute
    : attribute
    ;


classDeclarationModifier
    : PUBLIC
    | PUB
    | PRIVATE
    | PROTECTED
    | INTERNAL
    | ABSTRACT
    | FINAL
    | STATIC
    | VIRTUAL
    | OVERRIDE
    | SEALED
    | PARTIAL
    ;


/*
 * ============================================================================
 * 3. EXTENDS
 * ============================================================================
 *
 * The AST already preserves `extends` as an ordered collection.
 *
 * The grammar therefore deliberately permits zero or more parent references
 * after the keyword, subject to the syntactic requirement that at least one
 * type follows EXTENDS.
 *
 * Whether multiple inheritance is semantically supported is NOT decided here.
 *
 * This preserves the source language's ability to express:
 *
 *     class C extends A
 *
 * and, where enabled by the semantic model:
 *
 *     class C extends A, B
 *
 * without hard-coding an inheritance cardinality.
 */

classExtendsClause
    : EXTENDS
      classTypeReferenceList
    ;


classTypeReferenceList
    : typeExpression
      (COMMA typeExpression)*
      COMMA?
    ;


/*
 * ============================================================================
 * 4. IMPLEMENTS
 * ============================================================================
 *
 * Implemented contracts are represented as type expressions.
 *
 * This allows:
 *
 *     interface names
 *     trait names
 *     generic contracts
 *     qualified contracts
 *     future contract forms
 *
 * without embedding those semantic categories into class syntax.
 */

classImplementsClause
    : IMPLEMENTS
      classTypeReferenceList
    ;


/*
 * ============================================================================
 * 5. PERMITS
 * ============================================================================
 *
 * `permits` is source-level closed-hierarchy metadata.
 *
 * It does NOT:
 *
 *     allocate resources
 *     select hardware
 *     constrain machine size
 *     select runtime dispatch
 *
 * The semantic layer determines whether the class is sealed/closed and
 * whether every permitted relationship is valid.
 *
 * The list is unbounded by grammar.
 */

classPermitsClause
    : PERMITS
      classTypeReferenceList
    ;


/*
 * ============================================================================
 * 6. WHERE CLAUSE
 * ============================================================================
 *
 * Generic constraints remain syntactic structure here.
 *
 * The type/constraint system decides:
 *
 *     satisfiability
 *     coherence
 *     visibility
 *     generic substitution
 *     type compatibility
 *
 * No parser predicate is used.
 */

classWhereClause
    : WHERE
      classConstraintList
    ;


classConstraintList
    : classConstraint
      (COMMA classConstraint)*
      COMMA?
    ;


classConstraint
    : identifier
      COLON
      classConstraintBoundList
    ;


classConstraintBoundList
    : typeExpression
      (PLUS typeExpression)*
    ;


/*
 * ============================================================================
 * 7. CLASS BODY
 * ============================================================================
 *
 * Empty classes are legal:
 *
 *     class Marker {}
 *
 * There is no class-member cardinality limit.
 */

classBody
    : LBRACE
      classMember*
      RBRACE
    ;


/*
 * ============================================================================
 * 8. CLASS MEMBER DISPATCH
 * ============================================================================
 *
 * Members are explicitly enumerated.
 *
 * Do NOT use:
 *
 *     classMember : declaration ;
 *
 * because that would accidentally allow unrelated declaration families:
 *
 *     module
 *     package
 *     import
 *     deployment
 *     hardware declarations
 *     domain roots
 *
 * into class bodies.
 *
 * Explicit dispatch also prevents competing ownership between class syntax and
 * the general declaration grammar.
 */

classMember
    : classMemberAttributes
      classMemberCore
    ;


classMemberAttributes
    : attribute*
    ;


classMemberCore
    : classFieldDeclaration
    | classMethodDeclaration
    | classConstructorDeclaration
    | classPropertyDeclaration
    | classAssociatedTypeDeclaration
    | classAssociatedConstantDeclaration
    | classNestedTypeDeclaration
    ;


/*
 * ============================================================================
 * 9. CLASS FIELD
 * ============================================================================
 *
 * Canonical source shape:
 *
 *     value: Type;
 *
 * or:
 *
 *     private value: Type;
 *
 * A field initializer is intentionally supported because class construction
 * semantics require a way to express source-level default state.
 *
 * The initializer is parsed as an expression only.
 *
 * It is NOT evaluated by this grammar.
 *
 * Semantic analysis decides:
 *
 *     constant evaluation
 *     initialization order
 *     side effects
 *     ownership
 *     resource requirements
 *     target representation
 */

classFieldDeclaration
    : classMemberModifiers*
      identifier
      COLON
      typeExpression
      classFieldInitializer?
      SEMICOLON
    ;


classFieldInitializer
    : ASSIGN
      expression
    ;


/*
 * ============================================================================
 * 10. CLASS METHOD
 * ============================================================================
 *
 * Methods reuse the canonical function-level syntax concepts.
 *
 * This rule is deliberately explicit rather than importing an entire
 * top-level function declaration because a class method is a distinct AST
 * context even though its signature/body semantics are shared.
 *
 * Method syntax:
 *
 *     fn compute(value: T) -> T {
 *         ...
 *     }
 *
 * Declaration/prototype form:
 *
 *     fn compute(value: T) -> T;
 *
 * The semantic layer determines whether a body-less method is legal.
 */

classMethodDeclaration
    : classMemberModifiers*
      FN
      identifier
      genericParameters?
      LPAREN
      classParameterList?
      RPAREN
      classReturnClause?
      classEffectClause?
      classContractClause*
      classMethodImplementation
    ;


classMethodImplementation
    : block
    | SEMICOLON
    ;


classParameterList
    : classParameter
      (COMMA classParameter)*
      COMMA?
    ;


classParameter
    : classParameterModifiers*
      identifier
      classParameterType?
      classParameterDefault?
    ;


classParameterModifiers
    : MUT
    ;


classParameterType
    : COLON
      typeExpression
    ;


classParameterDefault
    : ASSIGN
      expression
    ;


classReturnClause
    : ARROW
      typeExpression
    ;


/*
 * ============================================================================
 * 11. METHOD EFFECT ATTACHMENT
 * ============================================================================
 *
 * Effects remain semantic metadata.
 *
 * The class grammar accepts a generic effect attachment rather than creating
 * a class-specific effect system.
 *
 * Example:
 *
 *     fn execute() with effects {
 *         quantum,
 *         io
 *     } {
 *         ...
 *     }
 *
 * Effect names are intentionally open.
 */

classEffectClause
    : WITH
      EFFECT
      LBRACE
      classEffectReferenceList?
      RBRACE
    ;


classEffectReferenceList
    : classEffectReference
      (COMMA classEffectReference)*
      COMMA?
    ;


classEffectReference
    : identifier
      (DOUBLE_COLON identifier)*
    ;


/*
 * ============================================================================
 * 12. METHOD CONTRACTS
 * ============================================================================
 *
 * Contracts remain expressions plus semantic obligations.
 *
 * The parser does not prove them.
 */

classContractClause
    : REQUIRES
      LPAREN
      expression
      RPAREN
      SEMICOLON?
    | ENSURES
      LPAREN
      expression
      RPAREN
      SEMICOLON?
    | INVARIANT
      LPAREN
      expression
      RPAREN
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 13. CONSTRUCTORS
 * ============================================================================
 *
 * Constructors are identified structurally by their identifier followed by
 * parameter parentheses and a body.
 *
 * No CONSTRUCTOR keyword is required.
 *
 * Example:
 *
 *     class Point {
 *         Point(x: f64, y: f64) {
 *             ...
 *         }
 *     }
 *
 * The semantic analyzer MUST verify that the constructor name matches the
 * containing class name.
 *
 * The parser does not perform that name comparison.
 *
 * This avoids adding another reserved keyword solely for constructors.
 */

classConstructorDeclaration
    : classMemberModifiers*
      identifier
      LPAREN
      classParameterList?
      RPAREN
      classConstructorEffects?
      classContractClause*
      block
    ;


classConstructorEffects
    : WITH
      EFFECT
      LBRACE
      classEffectReferenceList?
      RBRACE
    ;


/*
 * ============================================================================
 * 14. CLASS PROPERTIES
 * ============================================================================
 *
 * Properties are semantic members rather than mandatory storage locations.
 *
 * Example:
 *
 *     property size: Size;
 *
 * Accessors:
 *
 *     property size: Size {
 *         get;
 *         set;
 *     }
 *
 * PROPERTY, GET and SET are required lexical additions to the canonical
 * keyword vocabulary if this production syntax is enabled.
 *
 * They are listed explicitly in the integration contract below.
 */

classPropertyDeclaration
    : PROPERTY
      identifier
      COLON
      typeExpression
      classPropertyAccessorBlock?
      SEMICOLON?
    ;


classPropertyAccessorBlock
    : LBRACE
      classPropertyAccessor+
      RBRACE
    ;


classPropertyAccessor
    : GET
      classAccessorTermination
    | SET
      classAccessorTermination
    ;


classAccessorTermination
    : SEMICOLON
    | block
    ;


/*
 * ============================================================================
 * 15. ASSOCIATED TYPES
 * ============================================================================
 *
 * Classes may declare associated semantic types where the language type model
 * permits them.
 *
 * Example:
 *
 *     class Container<T> {
 *         type Element;
 *     }
 *
 * A default associated type is intentionally not introduced here.
 *
 * If default associated types become a language feature, they require an
 * explicit type-system/compatibility contract.
 */

classAssociatedTypeDeclaration
    : TYPE
      identifier
      classAssociatedTypeBound?
      SEMICOLON
    ;


classAssociatedTypeBound
    : COLON
      classConstraintBoundList
    ;


/*
 * ============================================================================
 * 16. ASSOCIATED CONSTANTS
 * ============================================================================
 *
 * Example:
 *
 *     class Matrix {
 *         const DIMENSION: Size;
 *     }
 *
 * Optional initializer:
 *
 *     const DIMENSION: Size = 4;
 *
 * The value is source-level expression data.
 *
 * No fixed machine size is implied.
 */

classAssociatedConstantDeclaration
    : CONST
      identifier
      COLON
      typeExpression
      classAssociatedConstantInitializer?
      SEMICOLON
    ;


classAssociatedConstantInitializer
    : ASSIGN
      expression
    ;


/*
 * ============================================================================
 * 17. CLASS NESTED TYPES
 * ============================================================================
 *
 * Nested declarations are explicitly restricted to type-level constructs.
 *
 * A nested class cannot silently contain modules, packages, imports,
 * deployments, or other compilation-unit declarations.
 */

classNestedTypeDeclaration
    : classNestedClassDeclaration
    | classNestedStructDeclaration
    | classNestedEnumDeclaration
    | classNestedInterfaceDeclaration
    | classNestedTraitDeclaration
    | classNestedTypeAliasDeclaration
    ;


classNestedClassDeclaration
    : classDeclarationPrefix*
      CLASS
      identifier
      genericParameters?
      classExtendsClause?
      classImplementsClause?
      classPermitsClause?
      classWhereClause?
      classBody
    ;


classNestedStructDeclaration
    : classDeclarationPrefix*
      STRUCT
      identifier
      genericParameters?
      classWhereClause?
      classNestedStructBody
    ;


classNestedStructBody
    : LBRACE
      classNestedStructField*
      RBRACE
    ;


classNestedStructField
    : classMemberModifiers*
      identifier
      COLON
      typeExpression
      classFieldInitializer?
      SEMICOLON
    ;


classNestedEnumDeclaration
    : classDeclarationPrefix*
      ENUM
      identifier
      genericParameters?
      classNestedEnumBody
    ;


classNestedEnumBody
    : LBRACE
      classNestedEnumVariant*
      RBRACE
    ;


classNestedEnumVariant
    : attribute*
      identifier
      classNestedEnumPayload?
      classNestedEnumInitializer?
      COMMA?
    ;


classNestedEnumPayload
    : LPAREN
      classParameterList?
      RPAREN
    | LBRACE
      classNestedEnumField*
      RBRACE
    ;


classNestedEnumField
    : identifier
      COLON
      typeExpression
    ;


classNestedEnumInitializer
    : ASSIGN
      expression
    ;


classNestedInterfaceDeclaration
    : classDeclarationPrefix*
      INTERFACE
      identifier
      genericParameters?
      classInterfaceInheritanceClause?
      classWhereClause?
      classNestedInterfaceBody
    ;


classInterfaceInheritanceClause
    : EXTENDS
      classTypeReferenceList
    ;


classNestedInterfaceBody
    : LBRACE
      classNestedInterfaceMember*
      RBRACE
    ;


classNestedInterfaceMember
    : attribute*
      classNestedInterfaceMemberCore
    ;


classNestedInterfaceMemberCore
    : classMethodDeclaration
    | classPropertyDeclaration
    | classAssociatedTypeDeclaration
    | classAssociatedConstantDeclaration
    ;


classNestedTraitDeclaration
    : classDeclarationPrefix*
      TRAIT
      identifier
      genericParameters?
      classInterfaceInheritanceClause?
      classWhereClause?
      classNestedTraitBody
    ;


classNestedTraitBody
    : LBRACE
      classNestedTraitMember*
      RBRACE
    ;


classNestedTraitMember
    : attribute*
      classNestedTraitMemberCore
    ;


classNestedTraitMemberCore
    : classMethodDeclaration
    | classPropertyDeclaration
    | classAssociatedTypeDeclaration
    | classAssociatedConstantDeclaration
    ;


classNestedTypeAliasDeclaration
    : classDeclarationPrefix*
      TYPE
      identifier
      genericParameters?
      ASSIGN
      typeExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 18. SHARED CLASS MEMBER MODIFIERS
 * ============================================================================
 *
 * These are syntactic modifiers only.
 *
 * Semantic analysis determines legal combinations.
 */

classMemberModifiers
    : PUBLIC
    | PUB
    | PRIVATE
    | PROTECTED
    | INTERNAL
    | STATIC
    | MUT
    | ABSTRACT
    | FINAL
    | VIRTUAL
    | OVERRIDE
    | INLINE
    | VOLATILE
    | ASYNC
    ;


/*
 * ============================================================================
 * 19. IDENTIFIER / TYPE INTEGRATION
 * ============================================================================
 *
 * These are forwarding contracts to the canonical shared grammar.
 *
 * In the final composed grammar:
 *
 *     identifier
 *     typeExpression
 *     expression
 *     block
 *     attribute
 *     genericParameters
 *
 * MUST each have exactly one effective owner.
 *
 * These class rules intentionally do not introduce hardware-specific type
 * alternatives.
 */


/*
 * ============================================================================
 * 20. SOURCE-LEVEL RESOURCE/CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Class members may use:
 *
 *     resource<T>
 *     capability<T>
 *     quantum types
 *     hardware abstractions
 *     distributed abstractions
 *     accelerator abstractions
 *
 * through typeExpression.
 *
 * This grammar does not interpret them.
 *
 * Therefore the same class syntax remains portable across target sizes.
 */


/*
 * ============================================================================
 * 21. SEMANTIC VALIDATION BOUNDARY
 * ============================================================================
 *
 * The parser MUST NOT decide:
 *
 *     duplicate class name
 *     duplicate member name
 *     duplicate generic parameter
 *     unknown base class
 *     invalid inheritance
 *     cyclic inheritance
 *     invalid implementation
 *     invalid permits relationship
 *     unsatisfied generic constraint
 *     invalid field type
 *     invalid method signature
 *     invalid constructor
 *     invalid property accessor
 *     invalid effect
 *     invalid resource requirement
 *     unavailable capability
 *     insufficient hardware
 *     insufficient memory
 *     unavailable QPU
 *     unavailable GPU
 *     unavailable FPGA
 *
 * These are semantic/resource/compiler/runtime concerns.
 */


/*
 * ============================================================================
 * 22. DIAGNOSTIC BOUNDARY
 * ============================================================================
 *
 * Syntax errors owned here include:
 *
 *     class {}
 *     class Name { : Type; }
 *     class Name { field Type; }
 *     class Name extends {}
 *     class Name implements {}
 *     class Name permits {}
 *     class Name { fn (); }
 *
 * Semantic errors include:
 *
 *     duplicate class names
 *     duplicate fields
 *     duplicate methods
 *     invalid inheritance
 *     inheritance cycles
 *     invalid interface implementation
 *     invalid permits relationship
 *     generic constraint failure
 *     illegal override
 *     incompatible method signature
 *     invalid constructor semantics
 *     unavailable resource
 *     unavailable capability
 *
 * Error classification must remain deterministic and preserve source spans.
 */


/*
 * ============================================================================
 * 23. SCALABILITY
 * ============================================================================
 *
 * These constructs are intentionally unbounded by grammar:
 *
 *     class declarations
 *     generic parameters
 *     base types
 *     implemented interfaces
 *     permitted types
 *     fields
 *     methods
 *     constructors
 *     properties
 *     associated types
 *     associated constants
 *     nested types
 *     type nesting
 *     expression nesting
 *     parameter lists
 *
 * No finite grammar cardinality is introduced.
 *
 * Example:
 *
 *     class DistributedTensor<T, Shape, Layout, Partition, Placement, ...> {
 *         ...
 *     }
 *
 * The actual resource requirements are semantic information.
 *
 * ============================================================================
 * 24. DOMAIN-NEUTRAL EXAMPLES
 * ============================================================================
 *
 * Classical:
 *
 *     class Vector<T> {
 *         data: T;
 *     }
 *
 * Quantum:
 *
 *     class LogicalRegister<Q> {
 *         state: Q;
 *     }
 *
 * Hybrid:
 *
 *     class HybridState<C, Q> {
 *         classical: C;
 *         quantum: Q;
 *     }
 *
 * Hardware/co-design:
 *
 *     class Accelerator<R> {
 *         resource: R;
 *     }
 *
 * Distributed:
 *
 *     class DistributedBuffer<T, Placement> {
 *         data: T;
 *         placement: Placement;
 *     }
 *
 * None of these forms select a physical machine.
 */


/*
 * ============================================================================
 * 25. NO HARD-CODED HARDWARE
 * ============================================================================
 *
 * Forbidden:
 *
 *     class GPU0 { ... }
 *     class QPU0 { ... }
 *     class EightCoreMachine { ... }
 *     class ThirtyTwoQubitDevice { ... }
 *
 * as grammar-level hardware assumptions.
 *
 * Such identifiers may still be ordinary source identifiers where a developer
 * deliberately models a target-specific entity through an explicit hardware
 * or deployment API.
 *
 * The grammar itself does not know that those entities exist.
 */


/*
 * ============================================================================
 * 26. QUANTUM EXTENSIBILITY
 * ============================================================================
 *
 * No quantum gate names are hard-coded here.
 *
 * The following remain semantic identifiers:
 *
 *     H
 *     X
 *     Y
 *     Z
 *     CNOT
 *     CX
 *     U
 *     RX
 *     RY
 *     RZ
 *     custom_gate
 *     vendor_operation
 *     logical_operation
 *
 * A class can therefore abstract future quantum operations without requiring
 * a new class grammar.
 */


/*
 * ============================================================================
 * 27. CONSTRUCTOR SEMANTICS
 * ============================================================================
 *
 * The parser accepts:
 *
 *     class Point {
 *         Point(x: f64, y: f64) {}
 *     }
 *
 * Semantic analysis MUST verify:
 *
 *     constructor identifier == containing class identifier
 *
 * It must also determine:
 *
 *     overload legality
 *     initialization rules
 *     ownership rules
 *     effect rules
 *     resource rules
 *
 * No parser predicate is used to compare the names.
 */


/*
 * ============================================================================
 * 28. CLASS / TRAIT / INTERFACE BOUNDARY
 * ============================================================================
 *
 * This file owns class syntax only.
 *
 * Traits remain owned by:
 *
 *     grammar/declarations/traits.g4
 *
 * Interfaces remain owned by:
 *
 *     grammar/declarations/interfaces.g4
 *
 * Implementations remain owned by:
 *
 *     grammar/declarations/implementations.g4
 *
 * A class may reference those constructs through:
 *
 *     implements
 *     extends
 *
 * but must not inline their declaration grammars.
 *
 * This prevents circular grammar ownership.
 */


/*
 * ============================================================================
 * 29. CLASS / FUNCTION BOUNDARY
 * ============================================================================
 *
 * Top-level functions remain owned by:
 *
 *     grammar/functions/functions.g4
 *
 * Class methods are owned by this class-member grammar because the surrounding
 * declaration context is semantically significant.
 *
 * Method signature components must remain structurally compatible with the
 * canonical function system:
 *
 *     parameters
 *     generics
 *     return types
 *     effects
 *     contracts
 *     blocks
 *
 * Any semantic difference between top-level functions and methods belongs in
 * semantic analysis, not duplicated parser logic.
 */


/*
 * ============================================================================
 * 30. CLASS / TYPE BOUNDARY
 * ============================================================================
 *
 * The class grammar consumes `typeExpression`.
 *
 * It must never introduce separate:
 *
 *     classTypeExpression
 *     quantumClassTypeExpression
 *     hardwareClassTypeExpression
 *
 * because that would create competing type systems.
 */


/*
 * ============================================================================
 * 31. CLASS / AST INTEGRATION
 * ============================================================================
 *
 * The frontend adapter must map:
 *
 *     classDeclaration
 *         -> ClassDeclaration
 *
 * Fields/methods/constructors/properties/nested declarations become canonical
 * AST child nodes and are referenced from the class's `members` collection.
 *
 * The class AST already expects:
 *
 *     generic_parameters
 *     extends
 *     implements
 *     permits
 *     members
 *
 * in source order.
 *
 * No parser-local class AST may be introduced.
 */


/*
 * ============================================================================
 * 32. CLASS / SEMANTIC MODEL
 * ============================================================================
 *
 * Semantic analysis consumes the AST and determines:
 *
 *     name resolution
 *     type resolution
 *     generic substitution
 *     inheritance validity
 *     interface/trait satisfaction
 *     method overriding
 *     overload resolution
 *     visibility
 *     ownership
 *     effects
 *     capabilities
 *     resources
 *     portability
 *     domain validity
 *
 * Only after these checks may the class participate in canonical semantic IR.
 */


/*
 * ============================================================================
 * 33. CLASS / IR INTEGRATION
 * ============================================================================
 *
 * This grammar has NO direct IR dependency.
 *
 * The downstream path is:
 *
 *     ClassDeclaration AST
 *          |
 *          v
 *     semantic class/type model
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> classical IR
 *          +--> quantum::ir
 *          +--> HDL/hardware representation
 *          +--> distributed representation
 *          +--> future-domain representation
 *
 * The class grammar must never construct any of those representations.
 */


/*
 * ============================================================================
 * 34. RUNTIME INTEGRATION
 * ============================================================================
 *
 * Runtime object representation is not part of this grammar.
 *
 * Runtime concerns include:
 *
 *     allocation
 *     dispatch
 *     scheduling
 *     placement
 *     serialization
 *     distributed execution
 *     accelerator execution
 *     quantum execution
 *     resilience
 *
 * Those remain downstream.
 */


/*
 * ============================================================================
 * 35. NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * The following must be rejected syntactically:
 *
 *     class {}
 *
 *     class Name extends {}
 *
 *     class Name implements {}
 *
 *     class Name permits {}
 *
 *     class Name {
 *         value
 *     }
 *
 *     class Name {
 *         : Type;
 *     }
 *
 *     class Name {
 *         fn compute(
 *     }
 *
 *     class Name {
 *         value: ;
 *     }
 *
 *     class Name {
 *         value: Type
 *         other: Type;
 *     }
 *
 * The following must NOT be rejected merely because they are large:
 *
 *     class Huge<T1, T2, ...> { ... }
 *
 *     class Distributed<...> { ... }
 *
 *     class Quantum<...> { ... }
 *
 * Their feasibility belongs to resource/semantic analysis.
 */


/*
 * ============================================================================
 * 36. BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Must test:
 *
 *     empty class
 *     one field
 *     many fields
 *     generic class
 *     nested generics
 *     recursive types
 *     one base
 *     many bases
 *     one interface
 *     many interfaces
 *     one permitted type
 *     many permitted types
 *     empty parameter list
 *     many parameters
 *     generic methods
 *     constructors
 *     overloaded constructors
 *     properties
 *     associated types
 *     associated constants
 *     nested types
 *     quantum types
 *     hardware/resource types
 *     distributed types
 *     future-domain identifiers
 */


/*
 * ============================================================================
 * 37. DETERMINISM TEST CONTRACT
 * ============================================================================
 *
 * Parsing the same source under the same grammar/token vocabulary/version MUST
 * produce the same parse-tree structure.
 *
 * No class grammar rule may depend on:
 *
 *     machine identity
 *     hardware availability
 *     environment variables
 *     wall-clock time
 *     random values
 *     filesystem state
 *     network state
 */


/*
 * ============================================================================
 * 38. COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing class syntax represented by the repository's AST contract must map
 * to this grammar:
 *
 *     class
 *     identifier
 *     genericParameters?
 *     extendsClause?
 *     implementsClause?
 *     permitsClause?
 *     classBody
 *
 * Legacy spellings must be handled by the compatibility layer rather than by
 * creating a second class grammar.
 */


/*
 * ============================================================================
 * 39. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 * [x] `classDeclaration` has exactly one concrete owner.
 * [x] Class inheritance has exactly one concrete owner.
 * [x] `implements` syntax has exactly one concrete owner.
 * [x] `permits` syntax has exactly one concrete owner.
 * [x] Class body syntax has exactly one concrete owner.
 * [x] Class field syntax has exactly one concrete owner.
 * [x] Class method syntax has exactly one concrete owner.
 * [x] Constructor syntax has exactly one concrete owner.
 * [x] Property syntax has exactly one concrete owner.
 * [x] Associated-type syntax has exactly one concrete owner.
 * [x] Associated-constant syntax has exactly one concrete owner.
 * [x] Nested type syntax is explicitly bounded to type declarations.
 * [x] Identifier syntax is not duplicated.
 * [x] Type-expression syntax is not duplicated.
 * [x] Expression syntax is not duplicated.
 * [x] Block syntax is not duplicated.
 * [x] Lexer rules are not defined here.
 * [x] No fixed machine limit exists.
 * [x] No fixed class/member/generic limit exists.
 * [x] No quantum gate list exists.
 * [x] No hardware topology exists.
 * [x] No QEC exists.
 * [x] No ZQN exists.
 * [x] No routing exists.
 * [x] No scheduling exists.
 * [x] No HAL exists.
 * [x] No runtime behavior exists.
 * [x] No embedded Rust exists.
 * [x] No unsafe Rust is required.
 * [x] Source order is preserved.
 * [x] AST mapping is defined.
 * [x] Semantic integration is defined.
 * [x] IR integration is defined.
 * [x] Compiler/runtime boundaries are defined.
 * [x] Negative/boundary/scalability/determinism tests are defined.
 *
 * ============================================================================
 */
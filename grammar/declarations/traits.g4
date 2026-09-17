/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/declarations/traits.g4
 *
 * Grammar:
 *     Traits
 *
 * Status:
 *     Canonical production parser delegate for trait declarations.
 *
 * Implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     Safe Rust only
 *     No embedded Rust
 *     No semantic predicates
 *     No filesystem/network/runtime access
 *     No unsafe code
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the SINGLE SYNTAX OWNER for source-level trait declarations.
 *
 * A trait is a reusable source-level contract that may be implemented by
 * arbitrary source-level types.
 *
 * Examples:
 *
 *     trait Drawable {
 *         fn draw(self);
 *     }
 *
 *     trait Numeric<T extends Number> {
 *         fn add(lhs: T, rhs: T) -> T;
 *     }
 *
 *     trait QuantumOperation<Q extends Qubit> {
 *         fn apply(operation: Q);
 *     }
 *
 *     trait Serializable {
 *         type Output;
 *
 *         const VERSION: Version;
 *
 *         fn serialize(self) -> Bytes;
 *     }
 *
 * A required trait method ends with `;`.
 *
 * A method containing a body is a default trait method:
 *
 *     trait Drawable {
 *         fn draw(self) {
 *             ...
 *         }
 *     }
 *
 * No `default` keyword is required. This avoids introducing an unnecessary
 * reserved word solely for trait syntax.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - traitDeclaration
 *     - trait visibility attachment
 *     - trait declaration modifiers
 *     - trait generic parameter attachment
 *     - trait inheritance syntax
 *     - trait where-clause attachment
 *     - trait body
 *     - trait member dispatch
 *     - trait method declarations
 *     - trait associated types
 *     - trait associated constants
 *     - trait-local source ordering
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical token definitions
 *     - keyword spelling
 *     - identifiers
 *     - qualified names
 *     - generic parameter semantics
 *     - type-expression syntax
 *     - parameter syntax
 *     - expression syntax
 *     - block syntax
 *     - effect syntax
 *     - contract semantics
 *     - implementation semantics
 *     - trait coherence
 *     - name resolution
 *     - type inference
 *     - capability resolution
 *     - resource resolution
 *     - hardware discovery
 *     - target selection
 *     - routing
 *     - scheduling
 *     - optimization
 *     - QEC
 *     - ZQN
 *     - HAL
 *     - runtime execution
 *     - quantum::ir
 *     - classical IR
 *     - HDL/hardware IR
 *
 * ============================================================================
 * ARCHITECTURAL PIPELINE
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     canonical ZamaniLexer
 *          |
 *          v
 *     canonical parser
 *          |
 *          v
 *     traitDeclaration
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     structural validation
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> name resolution
 *          +--> generic/type analysis
 *          +--> trait resolution
 *          +--> coherence analysis
 *          +--> effect analysis
 *          +--> capability analysis
 *          +--> resource analysis
 *          |
 *          v
 *     canonical semantic model
 *          |
 *          +--> classical representation
 *          +--> quantum semantic representation
 *          +--> HDL/hardware representation
 *          +--> distributed representation
 *          +--> accelerator representation
 *          +--> future domain representations
 *          |
 *          v
 *     canonical IR / domain IR
 *          |
 *          v
 *     optimization
 *          |
 *          +--> routing
 *          +--> scheduling
 *          +--> resilience
 *          +--> QEC
 *          +--> ZQN
 *          |
 *          v
 *     HAL / target realization
 *          |
 *          v
 *     runtime
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Traits MUST describe portable source-level contracts.
 *
 * This grammar contains no language-level limits for:
 *
 *     traits
 *     generic parameters
 *     supertraits
 *     members
 *     methods
 *     parameters
 *     associated types
 *     associated constants
 *     inheritance depth
 *
 * It MUST NOT encode:
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
 *     MAX_TENSOR_RANK
 *     MAX_VECTOR_WIDTH
 *
 * Nor may it encode:
 *
 *     physical qubit identifiers
 *     physical device identifiers
 *     fixed topology
 *     fixed accelerator counts
 *     hardware addresses
 *     backend-specific limits
 *
 * Practical compiler/parser limits belong to explicit implementation/resource
 * policy and MUST NOT become Zamani language semantics.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * A trait may be implemented by or constrain a quantum abstraction.
 *
 * Examples:
 *
 *     trait QuantumOperation<Q extends Qubit> {
 *         fn apply(operation: Q);
 *     }
 *
 *     trait Measurable {
 *         type Result;
 *         fn measure(self) -> Result;
 *     }
 *
 * These remain source-level contracts.
 *
 * This grammar MUST NOT:
 *
 *     - enumerate physical qubits;
 *     - enumerate physical gates;
 *     - select a QPU;
 *     - select topology;
 *     - select calibration;
 *     - perform routing;
 *     - perform scheduling;
 *     - perform QEC;
 *     - implement ZQN;
 *     - construct quantum::ir.
 *
 * Required direction:
 *
 *     Trait AST
 *         |
 *         v
 *     semantic trait model
 *         |
 *         v
 *     quantum semantic representation
 *         |
 *         v
 *     quantum::ir
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The existing canonical Trait AST contains source-level:
 *
 *     Node
 *     name
 *     visibility
 *     generics
 *     supertraits
 *     members
 *     modifiers
 *     attributes
 *     annotations
 *     effects
 *     capabilities
 *
 * This grammar therefore preserves:
 *
 *     - declaration source span;
 *     - name;
 *     - generic parameter order;
 *     - supertrait order;
 *     - member order;
 *     - member source spans;
 *     - modifier structure;
 *     - attribute structure.
 *
 * The parser/frontend adapter creates NodeIds and stores children in the
 * canonical AST store.
 *
 * This grammar creates no Rust AST values directly.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for:
 *
 *     - trait-name resolution;
 *     - generic binding;
 *     - duplicate-name detection;
 *     - inheritance resolution;
 *     - inheritance-cycle detection;
 *     - supertrait compatibility;
 *     - associated-type compatibility;
 *     - associated-constant compatibility;
 *     - method compatibility;
 *     - method effect compatibility;
 *     - implementation satisfaction;
 *     - coherence;
 *     - visibility;
 *     - capability requirements;
 *     - resource requirements;
 *     - portability.
 *
 * None of those decisions are made by parser predicates.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no actions;
 *     - no semantic predicates;
 *     - no randomness;
 *     - no time dependence;
 *     - no I/O;
 *     - no hardware discovery;
 *     - no runtime execution;
 *     - no mutable global state.
 *
 * Identical token streams under identical grammar/token versions must produce
 * identical parse structures.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * This grammar consumes the canonical shared parser components:
 *
 *     Types
 *     Expressions
 *     Parameters
 *     Blocks
 *     Attributes
 *     Generics
 *     Constraints
 *
 * These components are shared syntax owners.
 *
 * This grammar MUST NOT duplicate:
 *
 *     identifier
 *     qualifiedName
 *     typeExpression
 *     expression
 *     parameterList
 *     blockExpression
 *     genericParameterList
 *     attribute
 *
 * ============================================================================
 */

parser grammar Traits;

options {
    tokenVocab = ZamaniLexer;
}

import
    Types,
    Expressions,
    Parameters,
    Blocks,
    Attributes,
    Generics,
    Constraints;


/*
 * ============================================================================
 * 1. PUBLIC TRAIT ENTRY POINT
 * ============================================================================
 *
 * Declaration examples:
 *
 *     trait Drawable {
 *         fn draw(self);
 *     }
 *
 *     pub trait Numeric<T extends Number> {
 *         fn add(lhs: T, rhs: T) -> T;
 *     }
 *
 *     trait Child extends ParentA, ParentB {
 *         ...
 *     }
 */
traitDeclaration
    : traitAttributes*
      traitVisibility?
      traitModifiers*
      TRAIT
      identifier
      genericParameterList?
      traitInheritanceClause?
      traitWhereClause?
      traitBody
    ;


/*
 * ============================================================================
 * 2. TRAIT ATTRIBUTES
 * ============================================================================
 *
 * Attributes are generic source metadata.
 *
 * Their semantics are owned by the attribute/semantic subsystem.
 */
traitAttributes
    : attribute
    ;


/*
 * ============================================================================
 * 3. TRAIT VISIBILITY
 * ============================================================================
 *
 * Visibility is source-level metadata.
 *
 * Semantic legality is determined later.
 */
traitVisibility
    : PUBLIC
    | PUB
    | PRIVATE
    | PROTECTED
    | INTERNAL
    ;


/*
 * ============================================================================
 * 4. TRAIT MODIFIERS
 * ============================================================================
 *
 * Only modifiers already present in the canonical lexer are admitted here.
 *
 * This grammar deliberately does NOT add a `default` keyword.
 *
 * A method body itself identifies a default implementation.
 */
traitModifiers
    : ABSTRACT
    | FINAL
    | SEALED
    | PARTIAL
    | PURE
    | IMMUTABLE
    | LINEAR
    | AFFINE
    | INLINE
    ;


/*
 * ============================================================================
 * 5. INHERITANCE
 * ============================================================================
 *
 *     trait Child extends Parent
 *
 *     trait Child extends ParentA, ParentB
 *
 * No inheritance-count limit is encoded.
 */
traitInheritanceClause
    : EXTENDS
      traitSupertraitList
    ;


traitSupertraitList
    : typeExpression
      (
          COMMA
          typeExpression
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 6. WHERE CLAUSE
 * ============================================================================
 *
 * Example:
 *
 *     trait Numeric<T extends Number>
 *     where
 *         T extends Addable + Comparable
 *     {
 *         ...
 *     }
 *
 * Constraint interpretation is semantic.
 */
traitWhereClause
    : WHERE
      traitWhereConstraintList
    ;


traitWhereConstraintList
    : traitWhereConstraint
      (
          COMMA
          traitWhereConstraint
      )*
      COMMA?
    ;


traitWhereConstraint
    : identifier
      EXTENDS
      traitBoundList
    ;


traitBoundList
    : typeExpression
      (
          PLUS
          typeExpression
      )*
    ;


/*
 * ============================================================================
 * 7. TRAIT BODY
 * ============================================================================
 *
 * Empty traits are legal.
 */
traitBody
    : LBRACE
      traitMember*
      RBRACE
    ;


/*
 * ============================================================================
 * 8. TRAIT MEMBER DISPATCH
 * ============================================================================
 *
 * The member set is deliberately closed.
 *
 * This prevents arbitrary declarations from leaking into trait bodies.
 */
traitMember
    : traitMemberAttributes*
      traitMethodDeclaration
    | traitMemberAttributes*
      traitAssociatedTypeDeclaration
    | traitMemberAttributes*
      traitAssociatedConstantDeclaration
    ;


traitMemberAttributes
    : attribute
    ;


/*
 * ============================================================================
 * 9. TRAIT METHOD
 * ============================================================================
 *
 * Required method:
 *
 *     fn compute(value: T) -> R;
 *
 * Default method:
 *
 *     fn compute(value: T) -> R {
 *         ...
 *     }
 *
 * A body is canonical `blockExpression` syntax.
 *
 * The grammar does not determine whether a default method is semantically
 * legal for a particular trait.
 */
traitMethodDeclaration
    : traitMethodModifiers*
      FN
      identifier
      genericParameterList?
      LPAREN
      parameterList?
      RPAREN
      traitReturnClause?
      traitMethodWhereClause?
      traitMethodEffects?
      traitMethodContracts*
      traitMethodBodyOrTerminator
    ;


traitMethodModifiers
    : STATIC
    | ABSTRACT
    | FINAL
    | VIRTUAL
    | OVERRIDE
    | ASYNC
    | CONST
    | PURE
    | IMMUTABLE
    | LINEAR
    | AFFINE
    | INLINE
    ;


traitReturnClause
    : THIN_ARROW
      typeExpression
    ;


traitMethodWhereClause
    : WHERE
      traitWhereConstraintList
    ;


/*
 * ============================================================================
 * 10. METHOD EFFECT ATTACHMENT
 * ============================================================================
 *
 * The repository currently contains an effect grammar, but its modular parser
 * vocabulary is still part of the ZamaniTokens -> ZamaniLexer migration.
 *
 * Therefore the trait grammar owns only the attachment boundary here.
 *
 * The semantic effect system remains the owner of effect meaning.
 *
 * Syntax:
 *
 *     with effects { IO, quantum::Measurement }
 *
 * Effect identities are open-world qualified names.
 */
traitMethodEffects
    : WITH
      EFFECTS
      LBRACE
      traitEffectReferenceList?
      RBRACE
    ;


traitEffectReferenceList
    : qualifiedName
      (
          COMMA
          qualifiedName
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 11. METHOD CONTRACT ATTACHMENT
 * ============================================================================
 *
 * Contract syntax is intentionally structural.
 *
 * Example:
 *
 *     contract {
 *         requires(condition);
 *         ensures(result);
 *         invariant(property);
 *     }
 *
 * The expressions remain canonical Zamani expressions.
 *
 * The parser does not evaluate contracts.
 */
traitMethodContracts
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
 * 12. METHOD BODY / TERMINATOR
 * ============================================================================
 *
 * Required method:
 *
 *     fn work();
 *
 * Default method:
 *
 *     fn work() {
 *         ...
 *     }
 *
 * This is deliberately the only distinction required by the source grammar.
 */
traitMethodBodyOrTerminator
    : SEMICOLON
    | blockExpression
    ;


/*
 * ============================================================================
 * 13. ASSOCIATED TYPE
 * ============================================================================
 *
 * Required associated type:
 *
 *     type Item;
 *
 * Bounded associated type:
 *
 *     type Item extends Serializable + Ordered;
 *
 * A default associated type is deliberately NOT accepted until specialization
 * semantics have a canonical AST and semantic contract.
 */
traitAssociatedTypeDeclaration
    : TYPE
      identifier
      traitAssociatedTypeBounds?
      SEMICOLON
    ;


traitAssociatedTypeBounds
    : EXTENDS
      traitBoundList
    ;


/*
 * ============================================================================
 * 14. ASSOCIATED CONSTANT
 * ============================================================================
 *
 * Required:
 *
 *     const VERSION: Version;
 *
 * Default:
 *
 *     const VERSION: Version = 1;
 *
 * The initializer is a source expression.
 *
 * It is not evaluated by this grammar.
 */
traitAssociatedConstantDeclaration
    : CONST
      identifier
      COLON
      typeExpression
      traitAssociatedConstantInitializer?
      SEMICOLON
    ;


traitAssociatedConstantInitializer
    : ASSIGN
      expression
    ;


/*
 * ============================================================================
 * 15. SOURCE-ORDER CONTRACT
 * ============================================================================
 *
 * Trait member order is semantically observable for:
 *
 *     diagnostics
 *     source reconstruction
 *     deterministic AST serialization
 *     tooling
 *     provenance
 *
 * The parser/AST adapter MUST preserve the order in which traitMember occurs.
 */


/*
 * ============================================================================
 * 16. AST LOWERING CONTRACT
 * ============================================================================
 *
 * Parse context             Canonical AST representation
 *
 * traitDeclaration          Trait
 *
 * trait name                source name
 *
 * genericParameterList      generic NodeIds
 *
 * traitSupertraitList       supertrait NodeIds
 *
 * traitMember               member NodeIds
 *
 * trait attributes          attribute NodeIds
 *
 * trait modifiers           modifier NodeIds
 *
 * visibility                visibility NodeId
 *
 * The existing Trait AST owns the resulting source structure.
 *
 * No alternate Trait AST may be introduced.
 */


/*
 * ============================================================================
 * 17. MEMBER LOWERING
 * ============================================================================
 *
 * traitMethodDeclaration
 *     ->
 * canonical function/method declaration AST node
 *
 * traitAssociatedTypeDeclaration
 *     ->
 * canonical associated-type/type declaration AST node
 *
 * traitAssociatedConstantDeclaration
 *     ->
 * canonical constant declaration AST node
 *
 * Child nodes are allocated in the canonical AST store.
 *
 * This grammar does not construct those Rust values directly.
 */


/*
 * ============================================================================
 * 18. SEMANTIC TRAIT CONTRACT
 * ============================================================================
 *
 * After parsing, semantic analysis must validate:
 *
 *     - trait name uniqueness;
 *     - generic parameter scope;
 *     - duplicate generic parameters;
 *     - supertrait resolution;
 *     - supertrait cycles;
 *     - duplicate inherited members;
 *     - method compatibility;
 *     - associated type compatibility;
 *     - associated constant compatibility;
 *     - visibility;
 *     - modifier combinations;
 *     - method effects;
 *     - method contracts;
 *     - implementation conformance;
 *     - coherence;
 *     - capability requirements;
 *     - resource requirements.
 *
 * None of these are parser predicates.
 */


/*
 * ============================================================================
 * 19. QUANTUM SEMANTIC CONTRACT
 * ============================================================================
 *
 * A trait may use:
 *
 *     Qubit
 *     QuantumResource
 *     LogicalQubit
 *     QuantumState<T>
 *     quantum::...
 *
 * as source-level names/types.
 *
 * The grammar imposes no quantum resource limit.
 *
 * Example:
 *
 *     trait QuantumOperation<Q extends Qubit> {
 *         fn apply(operation: Q);
 *     }
 *
 * The parser does not know:
 *
 *     - physical qubit count;
 *     - physical qubit identity;
 *     - topology;
 *     - gate duration;
 *     - calibration;
 *     - QPU;
 *     - backend;
 *     - QEC code;
 *     - ZQN fault model.
 *
 * Those are downstream concerns.
 *
 * Quantum lowering remains:
 *
 *     Trait AST
 *       ->
 *     semantic model
 *       ->
 *     quantum::ir
 *
 * There is no:
 *
 *     Traits.g4 -> quantum::ir
 */


/*
 * ============================================================================
 * 20. CLASSICAL / HDL / HARDWARE / DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Traits remain domain-neutral.
 *
 * They may describe contracts implemented by:
 *
 *     classical types
 *     numerical types
 *     tensor abstractions
 *     accelerators
 *     HDL abstractions
 *     hardware resources
 *     distributed services
 *     AI/ML abstractions
 *     networking abstractions
 *     cryptographic abstractions
 *     future computational domains
 *
 * Domain meaning is resolved semantically.
 *
 * No:
 *
 *     CpuTrait
 *     GpuTrait
 *     QpuTrait
 *     FpgaTrait
 *     NodeTrait
 *
 * syntax is introduced merely because a target exists.
 */


/*
 * ============================================================================
 * 21. IMPLEMENTATION INTEGRATION
 * ============================================================================
 *
 * implementations.g4 consumes trait identity through a qualified source name.
 *
 * Example:
 *
 *     impl Drawable for Widget {
 *         fn draw(self) {
 *             ...
 *         }
 *     }
 *
 * The implementation grammar does not import this grammar and does not embed
 * trait declarations.
 *
 * This avoids a circular:
 *
 *     Traits <-> Implementations
 *
 * parser dependency.
 *
 * Semantic analysis connects the two.
 */


/*
 * ============================================================================
 * 22. DECLARATION DISPATCH INTEGRATION
 * ============================================================================
 *
 * grammar/declarations/declarations.g4 must remain the composition owner.
 *
 * It should expose:
 *
 *     traitDeclaration
 *
 * by importing this grammar.
 *
 * It must NOT redefine trait syntax.
 */


/*
 * ============================================================================
 * 23. LEGACY GRAMMAR MIGRATION
 * ============================================================================
 *
 * The repository contains legacy/parallel syntax in:
 *
 *     grammar/Zamani.g4
 *     grammar/antlr/Core.g4
 *     grammar/antlr/ZamaniParser.g4
 *     grammar/statements/declarations.g4
 *
 * Those surfaces may remain during migration but MUST NOT simultaneously be
 * authoritative production trait grammars.
 *
 * The final production path must have one trait declaration owner:
 *
 *     grammar/declarations/traits.g4
 */


/*
 * ============================================================================
 * 24. ERROR CONTRACT
 * ============================================================================
 *
 * Parser errors include:
 *
 *     missing trait name
 *     malformed generic parameter list
 *     malformed inheritance list
 *     malformed where clause
 *     malformed method signature
 *     missing method terminator/body
 *     malformed associated type
 *     malformed associated constant
 *     malformed attribute
 *
 * Semantic errors include:
 *
 *     unresolved trait
 *     cyclic inheritance
 *     duplicate supertrait
 *     incompatible inherited method
 *     invalid associated type
 *     invalid associated constant
 *     invalid modifier combination
 *     unsatisfied implementation
 *     unavailable capability
 *     unavailable resource
 *
 * Semantic errors MUST NOT be implemented through parser predicates.
 */


/*
 * ============================================================================
 * 25. SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Conformance tests must cover:
 *
 *     empty trait
 *     one-member trait
 *     many-member trait
 *     generic trait
 *     many generic parameters
 *     many bounds
 *     multiple supertraits
 *     deeply nested generic types
 *     many associated types
 *     many associated constants
 *     many methods
 *     default methods
 *     quantum-related contracts
 *     hardware-neutral contracts
 *     distributed contracts
 *     AI/data contracts
 *
 * No test may establish an artificial maximum.
 */


/*
 * ============================================================================
 * 26. COMPLETION CRITERIA
 * ============================================================================
 *
 * traits.g4 is complete when:
 *
 * [ ] It is the only concrete trait-syntax owner.
 * [ ] declarations.g4 only dispatches to it.
 * [ ] No legacy parser remains authoritative.
 * [ ] Canonical ZamaniLexer supplies all tokens.
 * [ ] Shared type syntax comes from Types.
 * [ ] Shared expression syntax comes from Expressions.
 * [ ] Shared parameter syntax comes from Parameters.
 * [ ] Shared block syntax comes from Blocks.
 * [ ] Shared attribute syntax comes from Attributes.
 * [ ] Shared generic syntax comes from Generics.
 * [ ] Shared constraint syntax comes from Constraints.
 * [ ] No duplicated identifier/type/expression grammar exists here.
 * [ ] No quantum IR is constructed.
 * [ ] No hardware limit is encoded.
 * [ ] No physical resource is selected.
 * [ ] No QEC/ZQN/routing/scheduling logic exists.
 * [ ] AST mapping is documented.
 * [ ] Semantic mapping is documented.
 * [ ] Implementation integration is documented.
 * [ ] Positive tests exist.
 * [ ] Negative tests exist.
 * [ ] Boundary tests exist.
 * [ ] Scalability tests exist.
 * [ ] Determinism tests exist.
 * [ ] Compatibility tests exist.
 * [ ] Rust integration remains compatible with 1.97/1.97.1.
 * [ ] No unsafe Rust is required.
 *
 * ============================================================================
 */
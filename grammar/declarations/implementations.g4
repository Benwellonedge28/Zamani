/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/declarations/implementations.g4
 *
 * Grammar:
 *     ZamaniImplementations
 *
 * PURPOSE
 * -------
 * Defines the syntax of implementation declarations.
 *
 * An implementation connects a type with behavior and, optionally, with a
 * trait/interface contract.
 *
 * Examples:
 *
 *     impl Widget {
 *         fn run(self) { ... }
 *     }
 *
 *     impl Runnable for Widget {
 *         fn run(self) { ... }
 *     }
 *
 *     impl<T> Runnable for Widget<T>
 *     where
 *         T: Send
 *     {
 *         fn run(self) { ... }
 *     }
 *
 * This grammar describes the SOURCE-LEVEL relationship only.
 *
 * It does not decide:
 *
 *     - whether a trait exists;
 *     - whether a type exists;
 *     - whether an implementation is valid;
 *     - whether an implementation satisfies a trait;
 *     - whether two implementations conflict;
 *     - whether an implementation is coherent;
 *     - whether a capability is available;
 *     - whether a target has sufficient resources;
 *     - how a method is dispatched;
 *     - how code is generated;
 *     - how quantum operations are lowered;
 *     - how hardware is selected.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 * Source
 *   |
 *   v
 * Lexer
 *   |
 *   v
 * Parser
 *   |
 *   +--> declarations
 *          |
 *          +--> implementations.g4
 *   |
 *   v
 * AST
 *   |
 *   v
 * Name resolution
 *   |
 *   v
 * Type checking
 *   |
 *   v
 * Generic/constraint solving
 *   |
 *   v
 * Trait/interface resolution
 *   |
 *   v
 * Capability/effect/resource analysis
 *   |
 *   v
 * Canonical semantic IR
 *   |
 *   +--> classical IR
 *   +--> quantum::ir
 *   +--> HDL/hardware semantic representations
 *   +--> other future domain representations
 *   |
 *   v
 * Optimization
 *   |
 *   v
 * Routing / Scheduling / Target lowering
 *   |
 *   v
 * Hardware / Runtime
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - implementation declaration syntax;
 *   - implementation generic-parameter attachment;
 *   - implementation target syntax;
 *   - trait/interface implementation syntax;
 *   - inherent implementation syntax;
 *   - implementation where-clause attachment;
 *   - implementation body syntax;
 *   - implementation member dispatch;
 *   - implementation method declarations;
 *   - implementation associated type declarations;
 *   - implementation associated constant declarations;
 *   - implementation property declarations where the language permits them.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - lexer tokens;
 *   - identifiers;
 *   - paths;
 *   - generic parameter definitions;
 *   - type syntax;
 *   - expressions;
 *   - statements;
 *   - blocks;
 *   - functions outside implementation bodies;
 *   - trait declarations;
 *   - interface declarations;
 *   - classes;
 *   - structs;
 *   - quantum IR;
 *   - classical IR;
 *   - HDL IR;
 *   - QEC;
 *   - ZQN;
 *   - routing;
 *   - scheduling;
 *   - optimization;
 *   - hardware discovery;
 *   - resource discovery;
 *   - target selection;
 *   - runtime execution.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Implementations provide behavior for semantic types/contracts.
 *
 * They MUST NOT encode permanent assumptions about a machine.
 *
 * This grammar therefore contains no:
 *
 *     MAX_QUBITS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_DEVICES
 *     MAX_MEMORY
 *     MAX_NODES
 *     MAX_REGISTERS
 *     fixed topology
 *     fixed device identifiers
 *     fixed hardware addresses
 *     fixed accelerator counts
 *     fixed deployment topology
 *
 * An implementation may ultimately execute on:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     simulator
 *     embedded hardware
 *     distributed systems
 *     cloud systems
 *     future architectures
 *
 * without changing the implementation declaration merely because the target
 * machine changes.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * This grammar has NO dependency on quantum::ir.
 *
 * A type implemented by this grammar may eventually be a quantum type.
 *
 * The path is:
 *
 *     implementation syntax
 *          |
 *          v
 *     implementation AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          v
 *     quantum::ir
 *
 * NOT:
 *
 *     implementations.g4 --> quantum::ir
 *
 * Likewise, this file does not implement:
 *
 *     QEC
 *     ZQN
 *     routing
 *     scheduling
 *     hardware calibration
 *     backend selection.
 *
 * ============================================================================
 * RUST
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * It contains no embedded Rust actions.
 *
 * Therefore this grammar does not require Rust `unsafe`.
 *
 * Generated Zamani compiler/parser infrastructure must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The lexer is the lexical authority.
 *
 * This grammar consumes the canonical `ZamaniLexer` vocabulary.
 *
 * In particular, `IMPL`, `FOR`, and `WHERE` belong to the lexer layer.
 *
 * This file MUST NOT define lexer tokens.
 *
 * ============================================================================
 */

parser grammar ZamaniImplementations;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * 1. IMPLEMENTATION DECLARATION
 * ============================================================================
 *
 * There are two semantic forms:
 *
 *     inherent implementation:
 *
 *         impl Type {
 *             ...
 *         }
 *
 *     contract implementation:
 *
 *         impl Trait for Type {
 *             ...
 *         }
 *
 * The grammar preserves this distinction syntactically.
 *
 * Semantic analysis determines whether the selected form is legal.
 *
 * No implementation count is bounded.
 */
implementationDeclaration
    : declarationModifiers
      IMPL
      implementationGenericParameters?
      implementationHead
      implementationWhereClause?
      implementationBody
    ;


/*
 * ============================================================================
 * 2. GENERIC PARAMETERS
 * ============================================================================
 *
 * Generic declaration syntax remains owned by the canonical generic grammar.
 *
 * This rule exists only as the integration point.
 *
 * It MUST NOT redefine:
 *
 *     genericParameter
 *     genericTypeParameter
 *     genericValueParameter
 *     genericParameterConstraints
 */
implementationGenericParameters
    : genericParameterList
    ;


/*
 * ============================================================================
 * 3. IMPLEMENTATION HEAD
 * ============================================================================
 *
 * Explicitly distinguish:
 *
 *     impl Type
 *
 * from:
 *
 *     impl Trait for Type
 *
 * This makes the syntax deterministic and gives semantic analysis a precise
 * distinction between:
 *
 *     inherent implementation
 *
 * and:
 *
 *     trait/interface implementation.
 */
implementationHead
    : inherentImplementationHead
    | contractImplementationHead
    ;

inherentImplementationHead
    : implementationType
    ;

contractImplementationHead
    : implementationContract
      FOR
      implementationType
    ;


/*
 * ============================================================================
 * 4. IMPLEMENTATION CONTRACT
 * ============================================================================
 *
 * A contract can be a trait or interface.
 *
 * The grammar does not resolve whether the name denotes:
 *
 *     trait
 *     interface
 *     another supported contract kind
 *
 * Name/type resolution owns that decision.
 */
implementationContract
    : qualifiedName
    ;


/*
 * ============================================================================
 * 5. IMPLEMENTATION TYPE
 * ============================================================================
 *
 * The implemented type is supplied by the canonical type grammar.
 *
 * This permits implementation of:
 *
 *     primitive abstractions
 *     user types
 *     generic types
 *     quantum types
 *     classical types
 *     hardware abstractions
 *     accelerator abstractions
 *     distributed types
 *     data types
 *     future types
 *
 * without adding domain-specific implementation grammars here.
 */
implementationType
    : typeReference
    ;


/*
 * ============================================================================
 * 6. WHERE CLAUSE
 * ============================================================================
 *
 * Constraints are attached to an implementation but are not solved here.
 *
 * Example:
 *
 *     impl<T> Trait for Type<T>
 *     where
 *         T: Constraint
 *     {
 *         ...
 *     }
 *
 * The semantic constraint solver determines satisfiability.
 */
implementationWhereClause
    : WHERE
      typeConstraintList
    ;


/*
 * ============================================================================
 * 7. IMPLEMENTATION BODY
 * ============================================================================
 */

implementationBody
    : LBRACE
      implementationMember*
      RBRACE
    ;


/*
 * ============================================================================
 * 8. IMPLEMENTATION MEMBER
 * ============================================================================
 *
 * Members are explicitly enumerated.
 *
 * Do NOT replace this with:
 *
 *     implementationMember : declaration ;
 *
 * because that would allow unrelated declarations to leak into an
 * implementation body.
 *
 * Explicit alternatives preserve ownership and deterministic parsing.
 */
implementationMember
    : implementationMemberAttributes
      implementationMemberCore
    ;

implementationMemberAttributes
    : attribute*
    ;

implementationMemberCore
    : implementationMethodDeclaration
    | implementationAssociatedTypeDeclaration
    | implementationAssociatedConstantDeclaration
    | implementationPropertyDeclaration
    ;


/*
 * ============================================================================
 * 9. IMPLEMENTATION METHOD
 * ============================================================================
 *
 * An implementation method is a concrete implementation of behavior.
 *
 * Unlike a trait/interface method declaration, an implementation method
 * normally requires a body.
 *
 * Example:
 *
 *     fn run(input: Input) -> Output {
 *         ...
 *     }
 *
 * The body is delegated to the canonical block grammar.
 */
implementationMethodDeclaration
    : implementationMethodModifiers?
      FN
      identifier
      genericParameterList?
      LPAREN
      parameterList?
      RPAREN
      functionReturnClause?
      effectClause?
      contractClause?
      implementationMethodBody
    ;

implementationMethodModifiers
    : declarationModifiers
    ;

implementationMethodBody
    : block
    ;


/*
 * ============================================================================
 * 10. ASSOCIATED TYPE
 * ============================================================================
 *
 * Implementations may provide a concrete associated type required by a
 * trait/interface.
 *
 * Example:
 *
 *     impl Iterator for Collection {
 *         type Item = Element;
 *     }
 *
 * The actual semantic compatibility is checked after parsing.
 */
implementationAssociatedTypeDeclaration
    : TYPE
      identifier
      ASSIGN
      typeReference
      SEMICOLON
    ;


/*
 * ============================================================================
 * 11. ASSOCIATED CONSTANT
 * ============================================================================
 *
 * Example:
 *
 *     impl Configurable for Device {
 *         const VERSION: Version = 1;
 *     }
 *
 * The expression is parsed only.
 *
 * Compile-time evaluation and constant validity belong to semantic analysis.
 *
 * No fixed value domain is imposed here.
 */
implementationAssociatedConstantDeclaration
    : CONST
      identifier
      COLON
      typeReference
      ASSIGN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 12. IMPLEMENTATION PROPERTY
 * ============================================================================
 *
 * A property implementation provides behavior for a property contract.
 *
 * It does not dictate:
 *
 *     memory layout
 *     storage location
 *     register allocation
 *     hardware register
 *     cache placement
 *     physical address.
 *
 * Those are downstream implementation decisions.
 */
implementationPropertyDeclaration
    : declarationModifiers?
      PROPERTY
      identifier
      COLON
      typeReference
      implementationPropertyBody?
      SEMICOLON?
    ;

implementationPropertyBody
    : LBRACE
      implementationPropertyAccessor+
      RBRACE
    ;

implementationPropertyAccessor
    : GET
      block
    | SET
      implementationSetterParameter?
      block
    ;

implementationSetterParameter
    : LPAREN
      parameterList?
      RPAREN
    ;


/*
 * ============================================================================
 * 13. SEMANTIC FORM IDENTIFICATION
 * ============================================================================
 *
 * The parser produces one of:
 *
 *     InherentImplementation
 *
 * or:
 *
 *     ContractImplementation
 *
 * Semantic analysis MUST subsequently determine:
 *
 *     - whether the target type exists;
 *     - whether the contract exists;
 *     - whether the contract is a trait/interface;
 *     - whether the contract is applicable;
 *     - whether generic constraints hold;
 *     - whether the implementation is coherent;
 *     - whether the implementation conflicts with another implementation;
 *     - whether all required members are implemented;
 *     - whether signatures match;
 *     - whether associated types match;
 *     - whether constants match;
 *     - whether properties match;
 *     - whether effects are compatible;
 *     - whether visibility is legal.
 *
 * None of these are parser responsibilities.
 */


/*
 * ============================================================================
 * 14. TRAIT INTEGRATION
 * ============================================================================
 *
 * This grammar does not import or redefine trait syntax.
 *
 * The relationship is semantic:
 *
 *     trait declaration
 *          |
 *          v
 *     trait identity
 *          |
 *          v
 *     implementation declaration
 *
 * `implementationContract` uses a qualified name rather than embedding
 * `traitDeclaration`.
 *
 * This prevents:
 *
 *     implementations.g4 <-> traits.g4
 *
 * from becoming a circular parser dependency.
 */


/*
 * ============================================================================
 * 15. INTERFACE INTEGRATION
 * ============================================================================
 *
 * Interfaces use the same implementation relationship at the semantic level.
 *
 * The implementation grammar therefore does not need a separate:
 *
 *     interfaceImplementationDeclaration
 *
 * rule.
 *
 * Example:
 *
 *     impl Drawable for Widget {
 *         ...
 *     }
 *
 * Whether `Drawable` is a trait or interface is resolved semantically.
 *
 * This prevents duplicate implementation models.
 */


/*
 * ============================================================================
 * 16. GENERIC INTEGRATION
 * ============================================================================
 *
 * Generic syntax comes from the generic subsystem.
 *
 * Dependency:
 *
 *     implementations.g4
 *          |
 *          +--> genericParameterList
 *          |
 *          +--> typeReference
 *          |
 *          +--> typeConstraintList
 *
 * The implementation grammar does not implement generic substitution.
 *
 * Generic substitution belongs to semantic/type analysis.
 */


/*
 * ============================================================================
 * 17. TYPE SYSTEM INTEGRATION
 * ============================================================================
 *
 * `implementationType` delegates to `typeReference`.
 *
 * This is essential for universal computing.
 *
 * An implementation target can therefore eventually denote:
 *
 *     classical type
 *     quantum type
 *     logical resource type
 *     hardware abstraction type
 *     accelerator type
 *     distributed type
 *     AI/data type
 *     future type
 *
 * without modifying this grammar.
 */


/*
 * ============================================================================
 * 18. EFFECT INTEGRATION
 * ============================================================================
 *
 * Implementation methods may consume the canonical `effectClause`.
 *
 * This allows a method to describe semantic effects such as:
 *
 *     IO
 *     quantum
 *     hardware
 *     network
 *     distributed
 *     security
 *     future effects
 *
 * without this grammar knowing their implementation.
 *
 * Effect checking remains outside the parser.
 */


/*
 * ============================================================================
 * 19. CONTRACT INTEGRATION
 * ============================================================================
 *
 * Implementation methods may use `contractClause`.
 *
 * Contract semantics remain owned by the contract subsystem.
 *
 * This grammar only provides the syntactic attachment point.
 */


/*
 * ============================================================================
 * 20. QUANTUM INTEGRATION
 * ============================================================================
 *
 * Example semantic usage:
 *
 *     impl QuantumOperation for MyOperation {
 *         fn apply(...) {
 *             ...
 *         }
 *     }
 *
 * Nothing here assumes:
 *
 *     number of qubits
 *     number of gates
 *     physical qubit IDs
 *     QPU topology
 *     gate durations
 *     calibration values
 *     noise rates
 *     backend IDs.
 *
 * If the implementation produces quantum computation:
 *
 *     AST
 *       |
 *       v
 *     semantic lowering
 *       |
 *       v
 *     quantum::ir
 *
 * remains the canonical quantum boundary.
 */


/*
 * ============================================================================
 * 21. HARDWARE / HDL INTEGRATION
 * ============================================================================
 *
 * An implementation may implement a hardware abstraction or hardware
 * interface.
 *
 * Example:
 *
 *     impl Accelerator for MatrixEngine {
 *         ...
 *     }
 *
 * This does NOT mean:
 *
 *     use GPU #0
 *     use FPGA #3
 *     use N cores
 *     use device X
 *
 * Hardware realization belongs to:
 *
 *     hardware capabilities
 *     resource requirements
 *     target descriptions
 *     compilation
 *     placement
 *     scheduling
 *     runtime.
 */


/*
 * ============================================================================
 * 22. RESOURCE INTEGRATION
 * ============================================================================
 *
 * Implementations can participate in resource-aware compilation through
 * semantic attributes, requirements, capabilities, effects, and constraints.
 *
 * This grammar does not define resource quantities.
 *
 * In particular, there is no parser-level maximum for:
 *
 *     implementations
 *     members
 *     generic parameters
 *     constraints
 *     types
 *     devices
 *     resources
 *     qubits
 *     nodes
 *     accelerators.
 *
 * Resource availability is a compilation/runtime concern.
 */


/*
 * ============================================================================
 * 23. AST CONTRACT
 * ============================================================================
 *
 * The parser output must preserve enough information for the AST layer to
 * represent:
 *
 *     ImplementationDecl
 *
 * containing:
 *
 *     modifiers
 *     genericParameters
 *     kind
 *     contract
 *     targetType
 *     whereClause
 *     members
 *
 * where `kind` is:
 *
 *     Inherent
 *     Contract
 *
 * Members:
 *
 *     ImplementationMethod
 *     ImplementationAssociatedType
 *     ImplementationAssociatedConstant
 *     ImplementationProperty
 *
 * Source spans must be preserved by the AST implementation.
 *
 * The grammar does not define the Rust AST structs themselves.
 */


/*
 * ============================================================================
 * 24. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis must enforce at least:
 *
 *     1. Target type resolution.
 *
 *     2. Contract resolution.
 *
 *     3. Trait/interface classification.
 *
 *     4. Generic parameter validity.
 *
 *     5. Generic constraint satisfiability.
 *
 *     6. Inheritance/contract applicability.
 *
 *     7. Implementation coherence.
 *
 *     8. Duplicate implementation detection.
 *
 *     9. Duplicate member detection.
 *
 *    10. Required method coverage.
 *
 *    11. Method signature compatibility.
 *
 *    12. Return-type compatibility.
 *
 *    13. Effect compatibility.
 *
 *    14. Associated-type compatibility.
 *
 *    15. Associated-constant compatibility.
 *
 *    16. Property compatibility.
 *
 *    17. Visibility rules.
 *
 *    18. Generic specialization rules.
 *
 *    19. Contract inheritance resolution.
 *
 *    20. Capability/resource requirements.
 *
 * These MUST NOT be encoded as parser actions.
 */


/*
 * ============================================================================
 * 25. COHERENCE
 * ============================================================================
 *
 * Implementation coherence is deliberately semantic.
 *
 * The parser must accept syntactically valid declarations even when the
 * compiler later determines that two declarations conflict.
 *
 * Examples of semantic errors:
 *
 *     impl Trait for Type { ... }
 *     impl Trait for Type { ... }
 *
 * or:
 *
 *     impl<T> Trait<T> for Type<T> ...
 *
 * conflicting with another applicable implementation.
 *
 * Such conflicts require symbol/type information and therefore cannot be
 * reliably solved by this grammar alone.
 */


/*
 * ============================================================================
 * 26. METHOD SIGNATURE COMPATIBILITY
 * ============================================================================
 *
 * The grammar accepts a method declaration.
 *
 * Semantic analysis must compare it against the required contract.
 *
 * It must consider:
 *
 *     name
 *     generic parameters
 *     parameter count
 *     parameter types
 *     receiver semantics
 *     return type
 *     effects
 *     contracts
 *     visibility
 *
 * Parser acceptance does not imply semantic validity.
 */


/*
 * ============================================================================
 * 27. ASSOCIATED TYPE COMPATIBILITY
 * ============================================================================
 *
 * Example:
 *
 *     trait Container {
 *         type Item;
 *     }
 *
 *     impl Container for Collection {
 *         type Item = Element;
 *     }
 *
 * Semantic analysis must verify:
 *
 *     - `Item` exists in the contract;
 *     - it is allowed to be implemented;
 *     - the supplied type satisfies all bounds;
 *     - duplicate associated types are rejected;
 *     - the resulting substitution is coherent.
 *
 * This grammar only parses the declaration.
 */


/*
 * ============================================================================
 * 28. ASSOCIATED CONSTANT COMPATIBILITY
 * ============================================================================
 *
 * Semantic analysis verifies:
 *
 *     - the constant exists in the contract;
 *     - its type is compatible;
 *     - its initializer is valid;
 *     - compile-time evaluation succeeds where required;
 *     - no forbidden runtime dependency exists for compile-time constants.
 */


/*
 * ============================================================================
 * 29. IMPLEMENTATION PROPERTY COMPATIBILITY
 * ============================================================================
 *
 * Semantic analysis determines whether:
 *
 *     getter
 *     setter
 *     type
 *     visibility
 *     effects
 *
 * satisfy the property contract.
 *
 * The parser does not determine storage semantics.
 */


/*
 * ============================================================================
 * 30. NO IMPLEMENTATION OF `unsafe`
 * ============================================================================
 *
 * This grammar does not introduce:
 *
 *     unsafe impl
 *
 * as a special implementation mechanism.
 *
 * Rust safety remains an implementation concern and this grammar contains
 * no embedded unsafe Rust.
 *
 * If Zamani later defines a language-level unsafe capability, it must be
 * specified independently with explicit ownership, capability checking,
 * diagnostics, and compatibility rules.
 *
 * It must not silently alter the meaning of ordinary `impl`.
 */


/*
 * ============================================================================
 * 31. NO HARDWARE BINDING
 * ============================================================================
 *
 * Forbidden parser-level concepts include:
 *
 *     implementationOnCpu
 *     implementationOnGpu
 *     implementationOnQpu
 *     implementationDeviceId
 *     implementationCoreCount
 *     implementationQubitCount
 *     implementationMemorySize
 *     implementationTopology
 *
 * Hardware binding belongs elsewhere.
 */


/*
 * ============================================================================
 * 32. NO BACKEND BINDING
 * ============================================================================
 *
 * The implementation grammar does not reference:
 *
 *     IBM QPU
 *     CUDA
 *     ROCm
 *     FPGA vendor
 *     ASIC vendor
 *     simulator implementation
 *     cloud provider
 *     physical device identifier.
 *
 * Backend-specific syntax, if eventually required, belongs to an explicitly
 * versioned dialect/interop layer.
 */


/*
 * ============================================================================
 * 33. NO QUANTUM RESOURCE LIMIT
 * ============================================================================
 *
 * A quantum implementation is not limited by this grammar.
 *
 * There is no:
 *
 *     q[0]
 *     q[1]
 *     MAX_QUBITS
 *
 * or equivalent fixed topology/count assumption.
 *
 * Resource feasibility is evaluated later.
 */


/*
 * ============================================================================
 * 34. NO DISTRIBUTED-SYSTEM LIMIT
 * ============================================================================
 *
 * An implementation may eventually be realized across:
 *
 *     one process
 *     many processes
 *     one node
 *     many nodes
 *     local execution
 *     remote execution
 *     cloud execution
 *     future distributed execution.
 *
 * This grammar imposes no node count or network topology.
 */


/*
 * ============================================================================
 * 35. ERROR OWNERSHIP
 * ============================================================================
 *
 * PARSER ERRORS:
 *
 *     missing `impl`
 *     missing target type
 *     malformed generic parameters
 *     malformed `for`
 *     malformed where clause
 *     malformed body
 *     malformed member
 *
 * SEMANTIC ERRORS:
 *
 *     unknown type
 *     unknown trait
 *     unknown interface
 *     duplicate implementation
 *     unsatisfied constraint
 *     missing method
 *     incompatible method
 *     invalid associated type
 *     invalid associated constant
 *     invalid property
 *     coherence conflict
 *     capability violation
 *     resource violation.
 *
 * Keeping these categories separate is required for production diagnostics.
 */


/*
 * ============================================================================
 * 36. DETERMINISM
 * ============================================================================
 *
 * For a fixed token stream, parsing must produce a deterministic parse tree.
 *
 * The implementation head is explicitly partitioned:
 *
 *     implementationType
 *
 * versus:
 *
 *     implementationContract FOR implementationType
 *
 * This avoids semantic lookup being required to decide whether `for` belongs
 * to the syntactic construct.
 *
 * Member alternatives are likewise explicit.
 */


/*
 * ============================================================================
 * 37. SCALABILITY
 * ============================================================================
 *
 * Grammar repetition operators intentionally impose no language-defined
 * maximum on:
 *
 *     implementation count
 *     member count
 *     generic parameter count
 *     constraint count
 *     nesting depth at the language level
 *     source size.
 *
 * Compiler memory/time limits may exist as implementation/resource policy.
 *
 * They are not language semantics and MUST NOT be represented by arbitrary
 * grammar constants.
 */


/*
 * ============================================================================
 * 38. ROUND-TRIP CONTRACT
 * ============================================================================
 *
 * Where the Zamani AST printer/serializer exists:
 *
 *     source
 *       |
 *       v
 *     lexer
 *       |
 *       v
 *     parser
 *       |
 *       v
 *     AST
 *       |
 *       v
 *     printer
 *       |
 *       v
 *     parser
 *
 * must preserve implementation semantics.
 *
 * Formatting differences are acceptable.
 *
 * Semantic differences are not.
 */


/*
 * ============================================================================
 * 39. TEST CONTRACT
 * ============================================================================
 *
 * Required positive tests:
 *
 *     impl Type { ... }
 *
 *     impl Trait for Type { ... }
 *
 *     impl<T> Trait for Type<T> { ... }
 *
 *     impl<T, U> Trait<T> for Type<U> { ... }
 *
 *     impl Trait for Type where ... { ... }
 *
 *     implementation with methods
 *
 *     implementation with associated types
 *
 *     implementation with associated constants
 *
 *     implementation with properties
 *
 *     implementation with attributes
 *
 *     implementation with effects
 *
 *     implementation with contracts.
 *
 * Required negative tests:
 *
 *     impl { ... }
 *
 *     impl for Type { ... }
 *
 *     impl Trait for { ... }
 *
 *     impl Trait Type { ... }
 *
 *     impl<T Trait for Type { ... }
 *
 *     malformed where clauses
 *
 *     malformed associated types
 *
 *     malformed associated constants
 *
 *     malformed methods
 *
 *     malformed property declarations.
 *
 * Required semantic negative tests:
 *
 *     unknown target type
 *     unknown contract
 *     duplicate implementation
 *     missing required method
 *     incompatible method
 *     invalid associated type
 *     invalid associated constant
 *     unsatisfied generic constraint
 *     conflicting implementations.
 *
 * Required boundary tests:
 *
 *     empty implementation
 *     one-member implementation
 *     many members
 *     deeply nested generic types
 *     many generic parameters
 *     many constraints
 *     large implementation bodies.
 *
 * Required cross-domain tests:
 *
 *     classical type implementation
 *     quantum type implementation
 *     quantum/classical implementation
 *     hardware abstraction implementation
 *     HDL-facing implementation
 *     accelerator implementation
 *     distributed implementation
 *     AI/data implementation.
 *
 * Required scalability tests:
 *
 *     no fixed machine size
 *     no fixed qubit count
 *     no fixed device count
 *     no fixed core count
 *     no fixed node count.
 */


/*
 * ============================================================================
 * 40. INTEGRATION WITH `declarations.g4`
 * ============================================================================
 *
 * `declarations.g4` should retain ONLY the declaration dispatch entry:
 *
 *     | implementationDeclaration
 *
 * It should no longer define:
 *
 *     implementationDeclaration
 *     implementationGenerics
 *     implementationTarget
 *     implementationForClause
 *     implementationWhereClause
 *     implementationBody
 *     implementationMember
 *
 * Those rules become owned by this file.
 *
 * This is necessary because the current repository still has those
 * implementation rules inside the monolithic declarations grammar.
 */


/*
 * ============================================================================
 * 41. COMPATIBILITY ALIASES
 * ============================================================================
 *
 * During migration, old rule names may be referenced by parser tests or
 * documentation:
 *
 *     implementationGenerics
 *     implementationTarget
 *     implementationForClause
 *     implementationWhereClause
 *
 * The migration should either:
 *
 *     1. update consumers to the canonical rules here, or
 *
 *     2. temporarily provide aliases in the composition layer.
 *
 * Do NOT retain two independently implemented versions.
 *
 * There must be one semantic implementation grammar.
 */


/*
 * ============================================================================
 * 42. DOWNSTREAM IR
 * ============================================================================
 *
 * This grammar produces syntax.
 *
 * It must never construct:
 *
 *     ClassicalIR
 *     QuantumIR
 *     HardwareIR
 *     Schedule
 *     Route
 *     OptimizedCircuit
 *     QEC program
 *     ZQN model.
 *
 * The frontend/semantic layer translates the AST into the appropriate
 * canonical semantic representation.
 */


/*
 * ============================================================================
 * 43. COMPILER INTEGRATION
 * ============================================================================
 *
 * Compiler pipeline:
 *
 *     Zamani source
 *         |
 *         v
 *     ZamaniLexer
 *         |
 *         v
 *     Zamani parser
 *         |
 *         v
 *     implementationDeclaration
 *         |
 *         v
 *     AST ImplementationDecl
 *         |
 *         v
 *     symbol/name resolution
 *         |
 *         v
 *     type checking
 *         |
 *         v
 *     generic/constraint solving
 *         |
 *         v
 *     trait/interface resolution
 *         |
 *         v
 *     capability/effect/resource validation
 *         |
 *         v
 *     canonical semantic IR
 *
 * No compiler stage should require implementations.g4 to know the target
 * hardware.
 */


/*
 * ============================================================================
 * 44. RUNTIME INTEGRATION
 * ============================================================================
 *
 * Runtime MUST NOT depend directly on parser grammar rules.
 *
 * Runtime consumes compiled semantic artifacts.
 *
 * Therefore:
 *
 *     runtime --> compiled representation
 *
 * NOT:
 *
 *     runtime --> implementations.g4
 *
 * This keeps source syntax independent of runtime architecture.
 */


/*
 * ============================================================================
 * 45. TOOLING INTEGRATION
 * ============================================================================
 *
 * IDE/editor tooling may use:
 *
 *     implementationDeclaration
 *     implementationHead
 *     implementationMember
 *
 * for:
 *
 *     syntax highlighting
 *     completion
 *     navigation
 *     source indexing
 *     diagnostics
 *     formatting
 *     refactoring.
 *
 * Tooling should consume parser/AST APIs rather than duplicating the grammar.
 */


/*
 * ============================================================================
 * 46. DOCUMENTATION INTEGRATION
 * ============================================================================
 *
 * The language specification must document:
 *
 *     inherent implementations
 *     contract implementations
 *     generic implementations
 *     implementation constraints
 *     implementation members
 *     coherence
 *     trait/interface satisfaction
 *
 * Documentation must not introduce a second implementation syntax.
 *
 * The formal grammar remains the parser authority.
 */


/*
 * ============================================================================
 * 47. COMPLETION CRITERIA
 * ============================================================================
 *
 * implementations.g4 is COMPLETE only when:
 *
 * [ ] It is the single authoritative implementation syntax owner.
 *
 * [ ] declarations.g4 contains only the dispatch reference.
 *
 * [ ] No duplicate implementation grammar remains active.
 *
 * [ ] Inherent implementations parse.
 *
 * [ ] Trait/interface implementations parse.
 *
 * [ ] Generic implementations parse.
 *
 * [ ] Where clauses parse.
 *
 * [ ] Implementation methods parse.
 *
 * [ ] Associated types parse.
 *
 * [ ] Associated constants parse.
 *
 * [ ] Properties parse where specified by the language contract.
 *
 * [ ] Attributes integrate with the canonical attribute grammar.
 *
 * [ ] Parameter syntax comes from the canonical function grammar.
 *
 * [ ] Type syntax comes from the canonical type grammar.
 *
 * [ ] Generic syntax comes from the canonical generic grammar.
 *
 * [ ] Expression syntax comes from the canonical expression grammar.
 *
 * [ ] Block syntax comes from the canonical statement grammar.
 *
 * [ ] Effect syntax comes from the canonical effect grammar.
 *
 * [ ] Contract syntax comes from the canonical contract grammar.
 *
 * [ ] No lexer tokens are duplicated.
 *
 * [ ] No hardware limits are encoded.
 *
 * [ ] No quantum machine assumptions are encoded.
 *
 * [ ] No direct dependency on quantum::ir exists.
 *
 * [ ] No QEC/ZQN implementation is embedded.
 *
 * [ ] No routing/scheduling implementation is embedded.
 *
 * [ ] No backend/device selection is embedded.
 *
 * [ ] No Rust action code exists.
 *
 * [ ] No unsafe Rust is required.
 *
 * [ ] Rust 1.97/1.97.1 compatibility is maintained.
 *
 * [ ] Parser determinism is tested.
 *
 * [ ] Positive syntax tests exist.
 *
 * [ ] Negative syntax tests exist.
 *
 * [ ] Semantic validation tests exist.
 *
 * [ ] Boundary tests exist.
 *
 * [ ] Cross-domain tests exist.
 *
 * [ ] Scalability tests exist.
 *
 * [ ] Round-trip tests exist.
 *
 * [ ] AST source spans are preserved.
 *
 * [ ] Trait/interface implementation resolution occurs downstream.
 *
 * [ ] Implementation semantics can ultimately lower to the appropriate
 *     canonical IR without grammar changes.
 */
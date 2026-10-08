/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/types/user-defined-types.g4
 *
 * Grammar:
 *     ZamaniUserDefinedTypes
 *
 * Status:
 *     CANONICAL PRODUCTION TYPE COMPONENT
 *
 * Compiler baseline:
 *     Rust 1.97+
 *     Rust 2021
 *     stable Rust
 *     safe Rust only
 *     no unsafe
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar defines the SOURCE-LEVEL SYNTAX for references to
 * user-defined/named types.
 *
 * It is deliberately a TYPE grammar component.
 *
 * It does NOT define type declarations.
 *
 * Type declarations remain owned by:
 *
 *     grammar/declarations/
 *
 * including:
 *
 *     declarations/types.g4
 *     declarations/aliases.g4
 *     declarations/structs.g4
 *     declarations/records.g4
 *     declarations/enums.g4
 *     declarations/unions.g4
 *     declarations/classes.g4
 *     declarations/interfaces.g4
 *     declarations/traits.g4
 *
 * This distinction is mandatory.
 *
 * A user-defined type reference answers:
 *
 *     "Which source-level named type is being referenced?"
 *
 * A type declaration answers:
 *
 *     "Which type is being introduced?"
 *
 * Those are different language constructs and must have different owners.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     grammar/antlr/ZamaniLexer.g4
 *          |
 *          v
 *     canonical parser
 *          |
 *          v
 *     typeExpression
 *          |
 *          +------------------------------+
 *          |                              |
 *          v                              v
 *     primitive types             userDefinedType
 *                                         |
 *                                         v
 *                                   TypePath
 *                                         |
 *                                         v
 *                                canonical TypeExpr
 *                                         |
 *                                         v
 *                              structural validation
 *                                         |
 *                                         v
 *                                  name resolution
 *                                         |
 *                                         v
 *                                  type resolution
 *                                         |
 *               +-------------------------+-------------------------+
 *               |                         |                         |
 *               v                         v                         v
 *          nominal type             alias resolution          generic/type
 *          resolution                                      constraint solving
 *               |                         |                         |
 *               +-------------------------+-------------------------+
 *                                         |
 *                                         v
 *                                  Semantic Type Model
 *                                         |
 *                    +--------------------+--------------------+
 *                    |                    |                    |
 *                    v                    v                    v
 *               Classical            Quantum             HDL/Hardware
 *                    |                    |                    |
 *                    +--------------------+--------------------+
 *                                         |
 *                                         v
 *                                canonical semantic IR
 *                                         |
 *                    +--------------------+--------------------+
 *                    |                                         |
 *                    v                                         v
 *              Classical IR                               quantum::ir
 *                    |                                         |
 *                    +--------------------+--------------------+
 *                                         |
 *                                         v
 *                              optimization / lowering
 *                                         |
 *                              routing / scheduling
 *                                         |
 *                              resilience / QEC
 *                                         |
 *                                         v
 *                                         ZQN
 *                                         |
 *                                         v
 *                                         HAL
 *                                         |
 *                                         v
 *                                   target realization
 *
 * ============================================================================
 * SINGLE-OWNER RULE
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     userDefinedType
 *     userDefinedTypePath
 *     userDefinedTypeSegment
 *
 * It owns only the source syntax required to reference a user-defined type.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     typeExpression
 *     namedType
 *     genericType
 *     genericTypeApplicationSuffix
 *     genericArgumentList
 *     genericParameter declarations
 *     type bounds
 *     where clauses
 *     primitive types
 *     tuple types
 *     record declarations
 *     struct declarations
 *     enum declarations
 *     union declarations
 *     alias declarations
 *     function types
 *     reference types
 *     pointer types
 *     option types
 *     result types
 *     array types
 *     slice types
 *     linear types
 *     affine types
 *     dependent types
 *     associated-type semantics
 *     type-class/interface semantics
 *     type inference
 *     unification
 *     substitution
 *     name resolution
 *     module resolution
 *     import resolution
 *     visibility checking
 *     alias expansion
 *     recursive-type analysis
 *     representation/layout
 *     ABI
 *     serialization
 *     resource discovery
 *     capability discovery
 *     hardware discovery
 *     target selection
 *     quantum allocation
 *     quantum routing
 *     quantum scheduling
 *     QEC
 *     ZQN
 *     HAL
 *     runtime execution
 *
 * ============================================================================
 * CRITICAL DISTINCTION: REFERENCE VS DECLARATION
 * ============================================================================
 *
 * The following are DECLARATIONS:
 *
 *     struct User {
 *         id: UserId,
 *     }
 *
 *     enum State {
 *         Ready,
 *         Running,
 *     }
 *
 *     type UserId = String;
 *
 * Those constructs remain owned by grammar/declarations/.
 *
 * The following are TYPE REFERENCES:
 *
 *     User
 *     UserId
 *     State
 *     module::User
 *     package::module::State
 *
 * This file owns the latter.
 *
 * A parser must never interpret a type reference as a declaration merely
 * because the name happens to match a declared type.
 *
 * ============================================================================
 * CANONICAL AST CONTRACT
 * ============================================================================
 *
 * A successful user-defined type reference MUST map to the existing
 * source-level type representation.
 *
 * Canonical conceptual representation:
 *
 *     TypeExpr::Identifier(TypePath)
 *
 * where TypePath preserves:
 *
 *     - segment order;
 *     - source spelling;
 *     - source structure;
 *     - source span information through the surrounding AST node.
 *
 * This grammar MUST NOT introduce:
 *
 *     UserDefinedTypeExpr
 *     UserDefinedTypeAst
 *     UserTypeNode
 *     NominalTypeNode
 *     QualifiedUserTypeNode
 *
 * as competing AST hierarchies.
 *
 * The repository already has:
 *
 *     TypeExpr
 *     TypePath
 *     TypeName
 *     NamedType
 *
 * Those remain the canonical source representation.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * This grammar performs NO semantic resolution.
 *
 * A source reference such as:
 *
 *     User
 *
 * is not resolved here.
 *
 * Semantic analysis determines whether it denotes:
 *
 *     - a local nominal type;
 *     - an imported nominal type;
 *     - a module-qualified type;
 *     - a package-qualified type;
 *     - a type alias;
 *     - a generic parameter;
 *     - an associated type;
 *     - a trait/interface-associated type;
 *     - a domain-defined type;
 *     - a capability abstraction;
 *     - a resource abstraction;
 *     - a quantum type;
 *     - an HDL type;
 *     - a hardware-intent type;
 *     - another valid type declaration.
 *
 * Therefore:
 *
 *     parser recognition
 *         !=
 *     name resolution
 *         !=
 *     type resolution
 *         !=
 *     target realization
 *
 * ============================================================================
 * USER-DEFINED TYPES ARE OPEN-WORLD
 * ============================================================================
 *
 * User-defined types MUST remain extensible.
 *
 * A valid type name may be introduced by:
 *
 *     the current module
 *     another module
 *     a package
 *     an imported library
 *     a dialect
 *     a domain subsystem
 *     a future extension
 *
 * The grammar must therefore NOT enumerate type names.
 *
 * Forbidden examples:
 *
 *     UserType
 *     Tensor
 *     Qubit
 *     Matrix
 *     Signal
 *     Device
 *     Agent
 *
 * as special parser alternatives.
 *
 * Such names remain ordinary identifiers unless another language subsystem
 * explicitly requires a reserved keyword.
 *
 * ============================================================================
 * QUALIFIED TYPE PATHS
 * ============================================================================
 *
 * User-defined types may be qualified by an arbitrary number of namespace,
 * module, package, or other semantic path segments.
 *
 * Examples:
 *
 *     User
 *
 *     domain::User
 *
 *     package::domain::User
 *
 *     package::module::submodule::User
 *
 *     quantum::State
 *
 *     hdl::Signal
 *
 *     hardware::Memory
 *
 *     ai::Tensor
 *
 *     distributed::Node
 *
 * The parser preserves the ordered path.
 *
 * It does not determine what each segment means.
 *
 * ============================================================================
 * NO FIXED PATH DEPTH
 * ============================================================================
 *
 * This grammar imposes no semantic maximum on:
 *
 *     namespace depth
 *     module depth
 *     package depth
 *     qualification depth
 *     identifier length
 *     number of user-defined types
 *     number of modules
 *     number of packages
 *
 * In particular, this grammar MUST NOT contain:
 *
 *     MAX_TYPE_PATH_DEPTH
 *     MAX_NAMESPACE_DEPTH
 *     MAX_MODULE_DEPTH
 *     MAX_PACKAGE_DEPTH
 *     MAX_IDENTIFIER_LENGTH
 *     MAX_USER_TYPES
 *
 * Any implementation/resource protection belongs to an explicit compiler
 * resource policy and MUST NOT redefine language semantics.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * User-defined type syntax is target-independent.
 *
 * A user-defined type reference MUST remain meaningful independently of
 * whether it is eventually realized on:
 *
 *     a tiny embedded target
 *     a CPU
 *     a multicore CPU
 *     a GPU
 *     an FPGA
 *     an ASIC
 *     an accelerator
 *     a QPU
 *     a quantum simulator
 *     an HPC system
 *     a cluster
 *     a distributed system
 *     a cloud system
 *     a future computational target
 *
 * The type reference does not identify the physical realization.
 *
 * For example:
 *
 *     quantum::LogicalQubit
 *
 * does not identify:
 *
 *     a physical qubit;
 *     a QPU;
 *     a vendor;
 *     a coupling map;
 *     a calibration;
 *     a physical address;
 *     a physical qubit index.
 *
 * Likewise:
 *
 *     hardware::Memory
 *
 * does not select:
 *
 *     a particular RAM bank;
 *     a device;
 *     an address range;
 *     a bus;
 *     a memory controller.
 *
 * Those decisions occur downstream.
 *
 * ============================================================================
 * GENERIC INTEGRATION
 * ============================================================================
 *
 * Generic APPLICATION syntax is deliberately outside this file.
 *
 * Examples:
 *
 *     Box<User>
 *
 *     module::Box<User>
 *
 *     Result<module::User, module::Error>
 *
 * The base:
 *
 *     Box
 *
 * or:
 *
 *     module::Box
 *
 * is recognized as a user-defined type reference.
 *
 * The:
 *
 *     <User>
 *
 * portion belongs to the canonical generic type-application grammar.
 *
 * This prevents:
 *
 *     named.g4
 *     user-defined-types.g4
 *     generic.g4
 *
 * from competing to own `<...>`.
 *
 * ============================================================================
 * GENERIC PARAMETER INTEGRATION
 * ============================================================================
 *
 * A generic parameter such as:
 *
 *     T
 *
 * is syntactically indistinguishable from an ordinary named type reference.
 *
 * This is intentional.
 *
 * Semantic scope resolution determines whether:
 *
 *     T
 *
 * denotes:
 *
 *     a generic type parameter
 *
 * or:
 *
 *     a named type declaration.
 *
 * The parser MUST NOT introduce a separate grammar alternative such as:
 *
 *     genericParameterType
 *
 * because that would duplicate semantic scope information.
 *
 * ============================================================================
 * ASSOCIATED TYPE INTEGRATION
 * ============================================================================
 *
 * Associated types may use qualified paths where the surrounding language
 * semantics permit them.
 *
 * For example:
 *
 *     Trait::Output
 *
 * or another canonical associated-type form may ultimately be interpreted
 * by semantic analysis.
 *
 * This grammar records only:
 *
 *     Trait
 *     Output
 *
 * as ordered path segments.
 *
 * Associated-type semantics remain owned by:
 *
 *     generic/constraints/associated-type subsystem
 *
 * and semantic resolution.
 *
 * ============================================================================
 * ALIAS INTEGRATION
 * ============================================================================
 *
 * The parser does not distinguish:
 *
 *     UserId
 *
 * from:
 *
 *     StringAlias
 *
 * even when the latter is an alias.
 *
 * Alias identity and expansion are semantic concerns.
 *
 * This is necessary to preserve the distinction between:
 *
 *     source-level name
 *
 * and:
 *
 *     resolved semantic type.
 *
 * ============================================================================
 * NOMINAL TYPE INTEGRATION
 * ============================================================================
 *
 * Structs, records, enums, unions, classes, interfaces, traits, opaque
 * declarations and future nominal type declarations may all become semantic
 * targets of a user-defined type reference.
 *
 * The parser does not need to know which declaration family introduced the
 * referenced name.
 *
 * That information is resolved from the symbol/type environment.
 *
 * ============================================================================
 * RECURSIVE TYPE INTEGRATION
 * ============================================================================
 *
 * This grammar permits references needed for recursive and mutually recursive
 * type definitions.
 *
 * Example:
 *
 *     struct Node {
 *         next: NodeRef,
 *     }
 *
 * or:
 *
 *     struct A {
 *         value: B,
 *     }
 *
 *     struct B {
 *         value: A,
 *     }
 *
 * The parser only records names.
 *
 * Semantic analysis must determine:
 *
 *     - whether the recursion is well-formed;
 *     - whether the representation is finite;
 *     - whether indirection is required;
 *     - whether an infinite-size direct representation would result;
 *     - whether the target representation is feasible.
 *
 * The grammar MUST NOT reject recursive names merely because the declaration
 * has not yet been fully resolved.
 *
 * ============================================================================
 * MODULE / IMPORT INTEGRATION
 * ============================================================================
 *
 * Name visibility and import semantics are owned by:
 *
 *     grammar/modules/
 *
 * and the semantic module/name resolver.
 *
 * This grammar does not:
 *
 *     open files;
 *     inspect packages;
 *     resolve imports;
 *     inspect libraries;
 *     access the filesystem;
 *     access the network.
 *
 * A syntactically valid path may still produce semantic diagnostics such as:
 *
 *     unknown type
 *     inaccessible type
 *     unresolved module
 *     unresolved import
 *     ambiguous type
 *
 * Those are NOT parser errors.
 *
 * ============================================================================
 * VISIBILITY INTEGRATION
 * ============================================================================
 *
 * Visibility belongs to declaration/module semantics.
 *
 * The following must therefore remain parser-valid when syntactically
 * correct:
 *
 *     private::Type
 *     public_module::Type
 *
 * Whether the referenced declaration is accessible is decided semantically.
 *
 * ============================================================================
 * DOMAIN-NEUTRALITY
 * ============================================================================
 *
 * User-defined types may represent values or abstractions from any supported
 * or future domain.
 *
 * Examples include:
 *
 *     classical::Matrix
 *     classical::Integer
 *     quantum::Qubit
 *     quantum::LogicalQubit
 *     quantum::State
 *     hdl::Signal
 *     hdl::Clock
 *     hardware::Memory
 *     hardware::Accelerator
 *     ai::Model
 *     ai::Tensor
 *     data::Dataset
 *     distributed::Node
 *     networking::Endpoint
 *     cryptography::Key
 *
 * The grammar does not reserve those names.
 *
 * Application-specific concepts remain libraries, dialects, capabilities,
 * policies, or semantic types rather than universal keyword families.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * A user-defined type may ultimately denote or contain resource-sensitive
 * semantics.
 *
 * Examples:
 *
 *     quantum::Qubit
 *     hardware::Memory
 *     distributed::Node
 *     capability::Compute
 *     resource::Buffer
 *
 * This grammar does not evaluate those meanings.
 *
 * Resource and capability semantics are handled downstream by:
 *
 *     grammar/resources/
 *     grammar/core/capabilities.g4
 *     semantic analysis
 *     resource analysis
 *     capability negotiation
 *     execution planning
 *
 * The language may therefore express symbolic requirements such as:
 *
 *     requires qubits >= n
 *
 *     requires memory >= required_memory
 *
 *     requires capability("quantum.measurement")
 *
 *     requires capability("gpu.compute")
 *
 * without embedding any capacity into user-defined type syntax.
 *
 * ============================================================================
 * EFFECT INTEGRATION
 * ============================================================================
 *
 * A user-defined type reference itself is normally effect-free.
 *
 * Merely naming:
 *
 *     quantum::State
 *
 * or:
 *
 *     ai::Model
 *
 * does not execute anything.
 *
 * Effects belong to operations involving values of those types.
 *
 * Examples of downstream effects include:
 *
 *     quantum.measurement
 *     io
 *     network
 *     foreign
 *     native
 *     mutation
 *     randomness
 *     distributed
 *     learning
 *     adaptation
 *     reflection
 *     code_generation
 *     simulation
 *
 * This grammar MUST NOT attach effects merely because a type name contains a
 * particular word.
 *
 * ============================================================================
 * CONTRACT / POLICY / PROVENANCE INTEGRATION
 * ============================================================================
 *
 * User-defined types may participate in:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * and:
 *
 *     policies
 *     evidence
 *     provenance
 *     explanation
 *     resource constraints
 *     capability constraints
 *
 * but none of these semantics are parsed by this file.
 *
 * Type declarations and type references are consumed by the appropriate
 * validation, policy and provenance systems.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * User-defined quantum types remain source abstractions.
 *
 * This grammar MUST NOT:
 *
 *     - enumerate quantum gates;
 *     - enumerate physical qubits;
 *     - assign physical qubit IDs;
 *     - select QPUs;
 *     - encode coupling maps;
 *     - encode calibration;
 *     - route quantum operations;
 *     - schedule quantum operations;
 *     - perform QEC;
 *     - construct quantum::ir.
 *
 * Required downstream direction:
 *
 *     userDefinedType
 *          |
 *          v
 *     TypeExpr
 *          |
 *          v
 *     semantic quantum type
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     optimization / decomposition / routing / scheduling / QEC
 *
 * ============================================================================
 * HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * User-defined types may describe:
 *
 *     signals
 *     interfaces
 *     registers
 *     memories
 *     hardware modules
 *     data paths
 *     accelerator abstractions
 *     timing-related semantic structures
 *
 * but this grammar does not decide:
 *
 *     bit width
 *     physical address
 *     physical register
 *     bus assignment
 *     placement
 *     routing
 *     clock implementation
 *     FPGA resource allocation
 *     ASIC cell mapping
 *     accelerator selection
 *
 * Exact representation requirements belong to hardware/HDL representation
 * contracts and downstream realization.
 *
 * ============================================================================
 * DATA / AI / REASONING INTEGRATION
 * ============================================================================
 *
 * User-defined types can represent:
 *
 *     knowledge structures
 *     evidence
 *     provenance records
 *     models
 *     datasets
 *     uncertainty structures
 *     decision records
 *     agent state
 *     tensor abstractions
 *     reasoning results
 *
 * The type grammar does not introduce application-specific type keywords.
 *
 * For example, a user may define:
 *
 *     struct DecisionRecord {
 *         evidence: Evidence,
 *         confidence: Confidence,
 *     }
 *
 * without requiring a universal `decision` type keyword.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing of a user-defined type path MUST be deterministic.
 *
 * It must depend only on:
 *
 *     source text
 *     canonical lexical vocabulary
 *     grammar version
 *     explicitly selected compatibility profile
 *
 * It must NOT depend on:
 *
 *     hardware availability
 *     machine size
 *     runtime state
 *     random values
 *     wall-clock time
 *     filesystem state
 *     network state
 *     hash-map iteration order
 *     compiler process address
 *     physical device identity
 *
 * Source path ordering MUST always be preserved.
 *
 * ============================================================================
 * SOURCE SPAN CONTRACT
 * ============================================================================
 *
 * The parser/frontend adapter must preserve:
 *
 *     - complete source span;
 *     - first segment span where available;
 *     - final segment span where available;
 *     - individual segment ordering;
 *
 * so that diagnostics can identify the exact unresolved or inaccessible
 * reference.
 *
 * ============================================================================
 * ERROR OWNERSHIP
 * ============================================================================
 *
 * PARSER ERRORS:
 *
 *     missing identifier
 *     malformed qualification
 *     missing path segment
 *     repeated qualification separator
 *     invalid token in type-name position
 *
 * SEMANTIC ERRORS:
 *
 *     unknown type
 *     ambiguous type
 *     inaccessible type
 *     invalid import
 *     invalid module path
 *     invalid alias
 *     generic parameter mismatch
 *     invalid associated type
 *     unsatisfied type bound
 *     recursive representation failure
 *
 * RESOURCE ERRORS:
 *
 *     required resource unavailable
 *     insufficient memory
 *     insufficient quantum resources
 *
 * CAPABILITY ERRORS:
 *
 *     required capability unavailable
 *
 * POLICY ERRORS:
 *
 *     type use prohibited by policy
 *
 * BACKEND ERRORS:
 *
 *     semantic type cannot be represented by selected realization
 *
 * These categories MUST NOT be collapsed into parser errors.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * This grammar:
 *
 *     MUST NOT execute user code;
 *     MUST NOT access the filesystem;
 *     MUST NOT access the network;
 *     MUST NOT inspect hardware;
 *     MUST NOT inspect credentials;
 *     MUST NOT resolve external resources;
 *     MUST NOT perform FFI;
 *     MUST NOT perform reflection;
 *     MUST NOT invoke a runtime;
 *     MUST NOT use embedded Rust actions;
 *     MUST NOT use semantic predicates for external state;
 *     MUST NOT require unsafe Rust.
 *
 * ============================================================================
 * IMPLEMENTATION CONTRACT
 * ============================================================================
 *
 * The generated parser is consumed by a safe Rust frontend.
 *
 * Rust implementation requirements:
 *
 *     Rust 1.97 or later
 *     Rust 2021
 *     stable language features
 *     no unsafe
 *
 * The implementation MUST:
 *
 *     - preserve source order;
 *     - preserve source spans;
 *     - avoid target-specific assumptions;
 *     - distinguish syntax and semantic diagnostics;
 *     - remain deterministic;
 *     - avoid fixed semantic cardinality limits;
 *     - report compiler resource exhaustion explicitly rather than silently
 *       changing the meaning of the source program.
 *
 * Implementation algorithms for semantic processing SHOULD prefer iterative
 * worklists/stacks where necessary so deeply nested valid source does not
 * acquire an accidental language-level recursion limit from the host call
 * stack.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * It defines NO lexer rules.
 *
 * The canonical lexical authority is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * which composes the repository lexical hierarchy.
 *
 * The parser-facing vocabulary MUST therefore be:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * This file must not introduce:
 *
 *     IDENT
 *     K_IDENT
 *     USER_TYPE
 *     TYPE_NAME
 *     USER_DEFINED_TYPE_TOKEN
 *
 * as competing lexical concepts.
 *
 * The canonical identifier token is:
 *
 *     IDENTIFIER
 *
 * and the canonical qualification separator is:
 *
 *     DOUBLE_COLON
 *
 * as supplied by the repository's lexical authority.
 *
 * ============================================================================
 * GRAMMAR
 * ============================================================================
 *
 * The grammar is intentionally small.
 *
 * The small grammar is a feature, not a missing capability.
 *
 * All semantic richness comes from the canonical type/name/semantic systems,
 * not from enumerating every possible user-defined type.
 *
 * ============================================================================
 */

parser grammar ZamaniUserDefinedTypes;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * PUBLIC ENTRY POINT
 * ========================================================================== */

/*
 * A user-defined type reference is one or more identifier segments.
 *
 * Examples:
 *
 *     User
 *     UserId
 *     module::User
 *     package::module::User
 *     quantum::State
 *
 * Generic arguments are deliberately NOT parsed here.
 *
 * Therefore:
 *
 *     Box<User>
 *
 * is composed by the canonical generic type grammar as:
 *
 *     userDefinedType + genericTypeApplicationSuffix
 */
userDefinedType
    : userDefinedTypePath
    ;


/* ============================================================================
 * USER-DEFINED TYPE PATH
 * ========================================================================== */

/*
 * A path consists of one or more type-name segments separated by `::`.
 *
 * No finite semantic path-depth limit exists.
 *
 * The parser preserves order exactly.
 */
userDefinedTypePath
    : userDefinedTypeSegment
      (DOUBLE_COLON userDefinedTypeSegment)*
    ;


/* ============================================================================
 * USER-DEFINED TYPE SEGMENT
 * ========================================================================== */

/*
 * A segment is an ordinary canonical identifier.
 *
 * Whether it names:
 *
 *     a type
 *     a generic parameter
 *     an associated type
 *     a module
 *     an imported symbol
 *     an alias
 *     a domain abstraction
 *
 * is determined by semantic resolution.
 */
userDefinedTypeSegment
    : IDENTIFIER
    ;


/* ============================================================================
 * END OF GRAMMAR
 * ========================================================================== */

Integration contract

This file is intentionally not made a second type system. The following repository integration is required.

1. "grammar/types/named.g4"

"named.g4" currently owns "namedType" and "typePath". That creates overlapping ownership with the new file.

After this file is adopted, "named.g4" should become a compatibility façade rather than another implementation.

Its canonical shape should become:

parser grammar NamedTypes;

options {
    tokenVocab = ZamaniLexer;
}

import ZamaniUserDefinedTypes;

namedType
    : userDefinedType
    ;

It must no longer independently define:

typePath
typePathSegment

The semantic AST remains:

TypeExpr::Identifier(TypePath)

There is therefore no AST migration caused by this grammar change.

---

2. "grammar/types/types.g4"

"types.g4" remains the sole public type-expression composition authority.

It should continue to expose:

typeExpression

and delegate named/user-defined types through the compatibility boundary:

typeExpression
    → namedType
    → userDefinedType
    → userDefinedTypePath

It must not copy "userDefinedTypePath" into "types.g4".

It must also remain the owner of composition between:

primitive
named/user-defined
generic
tuple
array
slice
function
reference
pointer
quantum
hardware
resource
other future type families

The new file therefore expands the implementation without creating another "typeExpression".

---

3. "grammar/types/generic.g4"

No generic application grammar belongs in this file.

The intended composition is:

module::Box<User>
       │
       ├── userDefinedType
       │      └── module::Box
       │
       └── genericTypeApplicationSuffix
              └── <User>

This preserves the existing generic architecture.

There must be exactly one owner for:

genericTypeArguments
genericArgumentList
genericTypeApplicationSuffix

The generic subsystem remains responsible for:

- type arguments;
- symbolic/value arguments where supported;
- argument ordering;
- generic arity;
- generic substitution;
- bounds;
- constraints;
- specialization;
- inference.

---

4. "grammar/declarations/*"

No declaration grammar should be moved into this file.

These remain independent:

Declaration| Owner
"type" declarations| "grammar/declarations/types.g4"
aliases| "grammar/declarations/aliases.g4"
structs| "grammar/declarations/structs.g4"
records| "grammar/declarations/records.g4"
enums| "grammar/declarations/enums.g4"
unions| "grammar/declarations/unions.g4"
classes| "grammar/declarations/classes.g4"
interfaces| "grammar/declarations/interfaces.g4"
traits| "grammar/declarations/traits.g4"
implementations| corresponding declaration owner

The declaration dispatcher remains the only declaration-family composition authority.

This prevents a common architectural failure:

types/user-defined-types.g4
        ↓
starts defining struct/enum/alias syntax
        ↓
declarations/*
        ↓
second competing declaration system

That is explicitly prohibited.

---

5. AST integration

The repository already has the appropriate source-level representation:

TypeExpr
TypePath
TypeName
NamedType

The parser adapter should map:

User

to:

TypeExpr::Identifier(
    TypePath::single("User")
)

and:

a::b::User

to:

TypeExpr::Identifier(
    TypePath::from_names([
        "a",
        "b",
        "User"
    ])
)

No new AST hierarchy should be introduced.

This is especially important because the repository currently has both older AST surfaces and the newer "src/frontend/ast" architecture. The canonical frontend AST should remain authoritative; the legacy "src/ast" representation should be treated as a compatibility/migration surface rather than a reason to introduce another type representation.

---

6. Semantic integration

Semantic analysis must perform the work that the grammar deliberately does not perform:

userDefinedType
      ↓
TypePath
      ↓
name resolution
      ↓
symbol resolution
      ↓
declaration lookup
      ↓
generic environment
      ↓
alias resolution
      ↓
associated-type resolution
      ↓
visibility
      ↓
bounds/constraints
      ↓
recursive-type analysis
      ↓
semantic type

The semantic resolver must distinguish at least:

nominal type
type alias
generic parameter
associated type
opaque type
foreign/external type
domain type
resource type
capability type
quantum type
HDL/hardware type

without requiring the parser to know which one a name denotes.

---

7. Type identity

A user-defined type's semantic identity must not be:

source memory address
parser-node address
Rust pointer address
process ID
thread ID
machine ID
device ID
QPU ID
compilation order
hash-map iteration order

It should be derived by the semantic symbol/type system from stable language information such as:

module/package identity
+
declaration identity
+
qualified name
+
generic parameter context

The exact stable-ID algorithm belongs to semantic/type identity infrastructure, not this grammar.

---

8. Recursive and mutually recursive types

This grammar deliberately accepts references needed for:

struct Node {
    next: NodeRef,
}

and:

struct A {
    b: B,
}

struct B {
    a: A,
}

Semantic analysis must then build the type-dependency graph and determine whether the representation is valid.

A direct infinitely sized representation must be rejected semantically.

An indirect representation can be valid.

For example, conceptually:

Node
 └── reference → Node

can be representable, whereas an unrestricted direct recursive value:

Node
 └── Node
      └── Node
           └── ...

cannot have a finite concrete representation.

This decision does not belong in the parser.

---

9. Quantum integration

The grammar permits:

quantum::Qubit
quantum::LogicalQubit
quantum::State
quantum::Register
quantum::Observable

but none of those names receive special grammar treatment.

This preserves the data-driven quantum architecture.

There must be no:

Qubit0
Qubit1
Qubit2
...

enumeration in the grammar.

Likewise there must be no fixed quantum capacity.

The eventual pipeline remains:

user-defined type
        ↓
semantic quantum type
        ↓
quantum::ir
        ↓
optimization
        ↓
decomposition
        ↓
routing
        ↓
scheduling
        ↓
resilience / QEC
        ↓
ZQN
        ↓
HAL

---

10. Classical / HDL / hardware integration

The same grammar works for:

classical::Matrix
hdl::Signal
hdl::Clock
hardware::Memory
hardware::Accelerator
distributed::Node
networking::Endpoint
data::Dataset

No hardware-specific grammar is required.

This is essential to POCO-REAF:

source type
     ↓
semantic meaning
     ↓
resource/capability analysis
     ↓
target realization

rather than:

source type
     ↓
fixed hardware representation

---

11. Resource and capability integration

A type may eventually participate in requirements such as:

requires qubits >= n;
requires memory >= required_memory;
requires capability("quantum.measurement");
requires capability("gpu.compute");
requires capability("tensor.compute");
requires topology(required_topology);

But the user-defined-type grammar never evaluates those expressions.

The separation is:

TYPE
  ↓
semantic meaning

REQUIREMENT
  ↓
required capability/resource

REALIZATION
  ↓
available target capability/resource

This is what allows one source program to scale from a tiny realization to much larger realizations without changing the type grammar.

---

12. Effects

Referencing a type is not itself an effect.

For example:

quantum::State

does not perform quantum computation.

An operation involving that value may introduce:

quantum
measurement
io
network
foreign
native
mutation
randomness
distributed
learning
adaptation
reflection
code_generation
simulation

The effect subsystem owns those semantics.

---

13. Contracts, policies, evidence and provenance

User-defined types must remain compatible with the broader semantic model:

VALUE
 ↓
TYPE
 ↓
OPERATION
 ↓
EFFECT / CAPABILITY / RESOURCE
 ↓
REQUIREMENT
 ↓
CONSTRAINT
 ↓
POLICY
 ↓
CONTRACT
 ↓
EVIDENCE
 ↓
PROVENANCE
 ↓
SEMANTIC IR

A type may therefore participate in:

requires
ensures
invariant
assume
guarantee
property

and in:

policy
evidence
provenance
explanation
decision records

without those constructs becoming part of this grammar.

---

14. Generic reasoning, learning and adaptation integration

The broader language can introduce:

infer
deduce
reason
assert
retract
query
learn
adapt
explain
evidence
provenance

but user-defined types remain ordinary semantic inputs to those systems.

For example, a library could define:

struct Evidence {
    source: Source,
    confidence: Confidence,
}

and then reasoning/learning operations can consume that type.

No application-specific keyword needs to be added to the type grammar.

Controlled adaptation likewise remains downstream:

adaptation
    ↓
policy
    ↓
authorization
    ↓
capability
    ↓
effects
    ↓
resources
    ↓
provenance
    ↓
validated state/model change

It must never imply unrestricted self-modifying code.

---

15. Deterministic and reproducible compilation

The parser must produce the same logical type-path structure for the same source.

For:

alpha::beta::Gamma

the path must always be:

alpha
beta
Gamma

Never derive semantic ordering from unordered collections.

This is required for reproducible compilation and long-term POCO-REAF compatibility.

---

16. Scalability rule

The grammar deliberately contains:

(DOUBLE_COLON userDefinedTypeSegment)*

rather than an artificial fixed sequence such as:

identifier
(DOUBLE_COLON identifier)?
(DOUBLE_COLON identifier)?
...

There is no grammar-level maximum for:

- user-defined types;
- path segments;
- modules;
- namespaces;
- packages;
- generic parameters;
- fields;
- variants;
- type nesting;
- symbolic dimensions;
- quantum resources;
- hardware resources;
- distributed resources.

“Unbounded” means no artificial language ceiling.

It does not mean a physical compiler has infinite RAM, CPU time or storage.

If a compiler runs out of resources, it should report an explicit compiler-resource failure rather than silently changing the program's meaning.

---

17. Required tests

This file is not production-ready until the following categories exist.

Positive

User
UserId
State
module::User
package::module::User
quantum::Qubit
quantum::LogicalQubit
hdl::Signal
hardware::Memory
ai::Tensor
distributed::Node

Generic integration

Box<User>
module::Box<User>
Result<module::User, module::Error>
Map<Key, module::Value>

Nested generic integration

Outer<Inner<User>>
module::Outer<package::Inner<domain::User>>

Generic parameter integration

fn identity<T>(value: T) -> T

Here "T" must initially parse as a named type reference. Semantic scope resolution decides that it is a type parameter.

Recursive

struct Node {
    next: NodeRef,
}

Mutual recursion

struct A {
    value: B,
}

struct B {
    value: A,
}

Cross-domain

struct HybridState {
    classical: classical::State,
    quantum: quantum::State,
    signal: hdl::Signal,
    model: ai::Model,
}

Negative syntax

::Type
Type::
Type:::Nested
Type::::Nested
::

Semantic negative cases

These should parse but fail later:

missing::Type
private_module::PrivateType
ambiguous::Type
unknown::Type

Scalability

Tests must cover:

A
A::B
A::B::C
...

without embedding an arbitrary language-level depth ceiling.

Large generated paths should be used for parser stress testing.

---

18. Required integration tests

At minimum, verify the entire path:

source
  ↓
ZamaniLexer
  ↓
ZamaniUserDefinedTypes
  ↓
typeExpression
  ↓
TypeExpr
  ↓
structural validation
  ↓
name resolution
  ↓
type resolution
  ↓
generic/constraint checking
  ↓
semantic type
  ↓
Classical IR / quantum::ir / HDL semantic representation

Cross-domain tests should include:

classical type
quantum type
HDL type
hardware type
AI/data type
distributed type
hybrid type
future-domain user type

---

19. Required file ownership after integration

The resulting ownership matrix should be:

Concern| Authoritative owner
lexical identifier| "grammar/lexer/identifiers.g4"
parser identifier| canonical core names/identifier grammar
user-defined type path| "grammar/types/user-defined-types.g4"
compatibility "namedType" façade| "grammar/types/named.g4"
type composition| "grammar/types/types.g4"
generic application| "grammar/types/generic.g4"
generic declaration| generic/declaration subsystem
type constraints| "grammar/types/constraints.g4"
associated types| associated-type subsystem
struct declaration| "grammar/declarations/structs.g4"
record declaration| "grammar/declarations/records.g4"
enum declaration| "grammar/declarations/enums.g4"
union declaration| "grammar/declarations/unions.g4"
alias declaration| "grammar/declarations/aliases.g4"
type declaration dispatch| "grammar/declarations/declarations.g4"
type identity| semantic type system
name resolution| semantic/module subsystem
visibility| modules/declaration semantics
generic substitution| semantic type system
recursion validation| semantic type system
representation| semantic/backend layers
ABI| interoperability
resource requirements| resources
capabilities| capabilities/resources
effects| effects
contracts| validation
policies| policies/security
provenance| provenance
quantum meaning| quantum semantic layer
quantum IR| "quantum::ir"
HDL realization| HDL/hardware layers
target selection| compilation/resource negotiation
routing| backend/quantum compiler
scheduling| execution/compiler
QEC| quantum resilience
ZQN| quantum execution layer
HAL| hardware abstraction layer

---

20. Important correction to the existing repository

The repository currently contains multiple generations of lexical/parser vocabulary, including older references such as "IDENT" in some modular grammar files while the canonical lexical hierarchy defines "IDENTIFIER".

For this production architecture, this file intentionally uses "IDENTIFIER" and "DOUBLE_COLON" through "ZamaniLexer".

The migration should be performed consistently across the affected parser family.

Do not solve vocabulary inconsistency by adding aliases such as:

IDENT
Identifier
IDENTIFIER
TYPE_NAME
USER_TYPE

to this grammar.

That would create another lexical compatibility problem.

The canonical direction should be:

grammar/lexer/
        ↓
grammar/antlr/ZamaniLexer.g4
        ↓
all parser grammars

with one parser-facing lexical vocabulary.

---

21. No hard-coded physical universe

This file must never acquire constructs such as:

MAX_USER_TYPES
MAX_TYPE_PATH_DEPTH
MAX_GENERIC_ARITY
MAX_FIELDS
MAX_VARIANTS
MAX_QUBITS
MAX_CPUS
MAX_GPUS
MAX_FPGAS
MAX_NODES
MAX_MEMORY
MAX_THREADS
MAX_REGISTER_WIDTH
MAX_TENSOR_RANK
MAX_NETWORK_SIZE
MAX_DEVICE_COUNT

Likewise, it must never contain:

QPU0
GPU0
CPU0
physical_qubit_0
memory_bank_0
device_0
register_0

as language-level type semantics.

The source language describes meaning and requirements.

The compiler determines realization.

---

22. Rust safety contract

Nothing in this grammar requires Rust actions.

The implementation must remain compatible with:

Rust 1.97+
Rust 2021
stable Rust
no unsafe

The Rust frontend should use ordinary safe structures such as:

Vec
Box
Arc
String / Arc<str>
Result
Option

as appropriate.

No "unsafe" block, unsafe function, raw-pointer-based semantic identity, or hardware-dependent parser behavior is permitted.

Compiler resource exhaustion must be represented as a diagnostic/error condition rather than converted into a fake type-system limit.

---

23. Completion criteria

"grammar/types/user-defined-types.g4" is DONE when all of the following are true:

- [x] It owns user-defined type reference syntax.
- [x] It owns qualified user-defined type paths.
- [x] It does not own type declarations.
- [x] It does not duplicate "typeExpression".
- [x] It does not duplicate generic application syntax.
- [x] It does not duplicate identifier lexical rules.
- [x] It uses the canonical "ZamaniLexer".
- [x] It uses the canonical "IDENTIFIER" token.
- [x] It uses the canonical "DOUBLE_COLON" qualification token.
- [x] It preserves arbitrary path structure.
- [x] It imposes no artificial path-depth limit.
- [x] It imposes no machine-capacity limit.
- [x] It does not enumerate domain types.
- [x] It does not enumerate quantum operations.
- [x] It does not select hardware.
- [x] It does not allocate quantum resources.
- [x] It does not perform resource negotiation.
- [x] It does not perform capability negotiation.
- [x] It does not perform semantic name resolution.
- [x] It does not perform type inference.
- [x] It does not perform generic substitution.
- [x] It does not perform layout or ABI selection.
- [x] It maps to the existing "TypeExpr"/"TypePath" architecture.
- [x] It supports nominal, alias, generic-parameter, associated-type and future-domain references through semantic resolution.
- [x] It supports recursive and mutually recursive type references syntactically.
- [x] It remains compatible with classical, quantum, HDL, hardware, AI/data, distributed and future domains.
- [x] It preserves POCO-REAF.
- [x] It remains deterministic.
- [x] It requires no unsafe Rust.
- [x] It defines parser/semantic/resource/capability/policy error boundaries.
- [x] It defines AST integration.
- [x] It defines semantic integration.
- [x] It defines generic integration.
- [x] It defines module/import integration.
- [x] It defines quantum integration.
- [x] It defines HDL/hardware integration.
- [x] It defines resource/effect/capability integration.
- [x] It defines contract/policy/provenance integration.
- [x] It defines scalability requirements.
- [x] It defines determinism requirements.
- [x] It defines security requirements.
- [x] It defines positive tests.
- [x] It defines negative tests.
- [x] It defines semantic-boundary tests.
- [x] It defines cross-domain tests.
- [x] It defines scalability tests.
- [x] It defines the exact downstream ownership boundaries.

The key production principle is:

user-defined type syntax
        ↓
canonical TypePath
        ↓
canonical TypeExpr
        ↓
semantic name resolution
        ↓
semantic type resolution
        ↓
constraints / effects / capabilities / resources
        ↓
contracts / policies / provenance
        ↓
canonical semantic representation
        ↓
Classical IR / quantum::ir / HDL-domain representation
        ↓
optimization
        ↓
lowering
        ↓
routing
        ↓
scheduling
        ↓
resilience / QEC
        ↓
ZQN / HAL
        ↓
target

This keeps "user-defined-types.g4" small, open-ended, deterministic, and maintainable while allowing the semantic type system behind it to grow to arbitrarily many user-defined types and arbitrarily large computational domains without putting a fixed physical universe into the grammar.

::writing
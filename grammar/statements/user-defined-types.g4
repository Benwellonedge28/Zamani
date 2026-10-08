/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/statements/user-defined-types.g4
 *
 * STATUS
 * ------
 * CANONICAL USER-DEFINED-TYPE STATEMENT COMPOSITION BOUNDARY
 *
 * GRAMMAR TECHNOLOGY
 * ------------------
 * ANTLR4 parser grammar
 *
 * IMPLEMENTATION BASELINE
 * -----------------------
 * Rust 1.97 or later
 * Rust 2021
 *
 * SAFETY
 * ------
 * This grammar contains grammar definitions only.
 *
 * It contains:
 *
 *     - no embedded Rust actions;
 *     - no semantic predicates;
 *     - no unsafe code;
 *     - no filesystem access;
 *     - no networking;
 *     - no hardware discovery;
 *     - no runtime execution;
 *     - no mutable parser-global state;
 *     - no target-specific implementation;
 *     - no fixed machine-capacity constants.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the STATEMENT-LAYER COMPOSITION OWNER for user-defined types.
 *
 * It provides one stable integration boundary between:
 *
 *     grammar/statements/declarations.g4
 *
 * and the concrete declaration grammars under:
 *
 *     grammar/declarations/
 *
 * It does NOT redefine the concrete syntax of:
 *
 *     type aliases
 *     named type declarations
 *     structs
 *     records
 *     enums
 *     unions
 *     classes
 *     interfaces
 *     traits
 *     implementations
 *
 * Those constructs already have dedicated declaration grammars.
 *
 * This file composes those existing declarations into ONE statement-level
 * family.
 *
 * ============================================================================
 * ARCHITECTURE
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     ZamaniParser
 *          |
 *          v
 *     Statements
 *          |
 *          v
 *     Declarations
 *          |
 *          v
 *     UserDefinedTypes                  <-- THIS FILE
 *          |
 *          +--> type declarations
 *          +--> aliases
 *          +--> structs
 *          +--> records
 *          +--> enums
 *          +--> unions
 *          +--> classes
 *          +--> interfaces
 *          +--> traits
 *          +--> implementations
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     structural validation
 *          |
 *          +--> names
 *          +--> types
 *          +--> generics
 *          +--> ownership
 *          +--> effects
 *          +--> capabilities
 *          +--> resources
 *          +--> contracts
 *          +--> policies
 *          +--> provenance
 *          |
 *          v
 *     semantic type system
 *          |
 *          +--> classical
 *          +--> quantum
 *          +--> hybrid
 *          +--> HDL
 *          +--> hardware
 *          +--> AI/model
 *          +--> data
 *          +--> distributed
 *          +--> networking
 *          +--> future domains
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> classical IR
 *          +--> quantum::ir
 *          +--> domain-specific IR where required
 *          |
 *          v
 *     optimization / lowering / routing / scheduling
 *          |
 *          v
 *     ZQN / HAL / target realization
 *
 * This file MUST remain entirely above semantic realization.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     userDefinedTypeDeclaration
 *     userDefinedTypeStatement
 *
 * It also owns the ORDERED COMPOSITION of the user-defined-type declaration
 * family at the statement boundary.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     lexer rules
 *     token definitions
 *     identifiers
 *     qualified-name syntax
 *     attributes
 *     visibility syntax
 *     generic-parameter syntax
 *     type-expression syntax
 *     alias syntax
 *     struct syntax
 *     record syntax
 *     enum syntax
 *     union syntax
 *     class syntax
 *     interface syntax
 *     trait syntax
 *     implementation syntax
 *     function syntax
 *     module syntax
 *     expressions
 *     effects
 *     resources
 *     capabilities
 *     contracts
 *     policies
 *     provenance
 *     semantic type checking
 *     type inference
 *     type layout
 *     ABI
 *     machine representation
 *     hardware placement
 *     quantum allocation
 *     QEC
 *     scheduling
 *     routing
 *     optimization
 *     runtime execution
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * Each concrete user-defined type construct MUST have exactly one concrete
 * grammar owner.
 *
 * The ownership map is:
 *
 *     type declaration / alias
 *         -> grammar/declarations/types.g4
 *
 *     struct
 *         -> grammar/declarations/structs.g4
 *
 *     enum
 *         -> grammar/declarations/enums.g4
 *
 *     record
 *         -> grammar/declarations/records.g4
 *
 *     union
 *         -> grammar/declarations/unions.g4
 *
 *     class
 *         -> grammar/declarations/classes.g4
 *
 *     interface
 *         -> grammar/declarations/interfaces.g4
 *
 *     trait
 *         -> grammar/declarations/traits.g4
 *
 *     implementation
 *         -> grammar/declarations/implementations.g4
 *
 * THIS FILE MUST NOT redefine any of those concrete rules.
 *
 * It only composes their already-owned public rules.
 *
 * ============================================================================
 * CRITICAL DUPLICATION RULE
 * ============================================================================
 *
 * The current repository contains substantial declaration syntax directly in:
 *
 *     grammar/statements/declarations.g4
 *
 * That syntax must eventually be removed from that file and replaced by:
 *
 *     userDefinedTypeStatement
 *
 * or its canonical declaration adapter.
 *
 * In particular, declarations.g4 MUST NOT continue to define independent
 * copies of:
 *
 *     typeAliasDeclaration
 *     structDeclaration
 *     recordDeclaration
 *     enumDeclaration
 *     unionDeclaration
 *     classDeclaration
 *     interfaceDeclaration
 *     traitDeclaration
 *     implDeclaration
 *
 * after this composition boundary is integrated.
 *
 * Otherwise there would be multiple effective owners and the repository would
 * not have a clean production grammar architecture.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 *     grammar/declarations/types.g4
 *     grammar/declarations/structs.g4
 *     grammar/declarations/enums.g4
 *     grammar/declarations/records.g4
 *     grammar/declarations/unions.g4
 *     grammar/declarations/classes.g4
 *     grammar/declarations/interfaces.g4
 *     grammar/declarations/traits.g4
 *     grammar/declarations/implementations.g4
 *
 * Indirect dependencies are inherited through those declaration grammars.
 *
 * This file MUST NOT depend on:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * because the parser root consumes this composition layer; it must not be
 * imported in the opposite direction.
 *
 * ============================================================================
 * EXPORTS
 * ============================================================================
 *
 * Public parser rules exported by this file:
 *
 *     userDefinedTypeStatement
 *     userDefinedTypeDeclaration
 *
 * These are the only rules that statement composition should consume.
 *
 * ============================================================================
 * CONSUMED BY
 * ============================================================================
 *
 * Primary consumer:
 *
 *     grammar/statements/declarations.g4
 *
 * Secondary consumers may include:
 *
 *     grammar/core/compilation-unit.g4
 *
 *     grammar/modules/*
 *
 *     grammar/metaprogramming/*
 *
 *     tooling/conformance parsers
 *
 * but only through the public rules exported here.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar creates parse-tree structure only.
 *
 * It does not construct Rust AST values directly.
 *
 * The downstream AST adapter maps the concrete declaration parse nodes into
 * the repository's canonical frontend AST.
 *
 * Existing repository AST representations include constructs corresponding to:
 *
 *     TypeDeclaration
 *     TypeAlias
 *     Struct
 *     Enum
 *     Trait
 *     Impl
 *
 * and the modular frontend AST additionally has dedicated declaration nodes.
 *
 * This grammar therefore MUST NOT introduce a second semantic representation
 * merely because it introduces a statement-level composition boundary.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing a user-defined type declaration establishes only source structure.
 *
 * Semantic analysis MUST subsequently determine:
 *
 *     - whether the declared name is legal;
 *     - whether the name is unique in its namespace;
 *     - whether the declaration is visible;
 *     - whether generic parameters are valid;
 *     - whether generic parameters are unique;
 *     - whether bounds are valid;
 *     - whether bounds are satisfiable;
 *     - whether referenced types exist;
 *     - whether type applications have valid arity;
 *     - whether recursive definitions are permitted;
 *     - whether aliases form cycles;
 *     - whether structural recursion is valid;
 *     - whether traits/interfaces are coherent;
 *     - whether implementations satisfy their contracts;
 *     - whether associated types are valid;
 *     - whether associated constants are valid;
 *     - whether members are legal;
 *     - whether visibility is valid;
 *     - whether effects are valid;
 *     - whether capabilities are sufficient;
 *     - whether resource requirements can be satisfied;
 *     - whether contracts hold;
 *     - whether policies permit the declaration;
 *     - whether provenance requirements are satisfied.
 *
 * None of those checks belong in this grammar.
 *
 * ============================================================================
 * TYPE-SYSTEM CONTRACT
 * ============================================================================
 *
 * User-defined types must integrate with the ONE canonical Zamani type system.
 *
 * This file MUST NOT introduce another `typeExpression` rule.
 *
 * Type expressions are owned by:
 *
 *     grammar/types/
 *
 * A user-defined type may therefore ultimately represent or contain:
 *
 *     primitive types
 *     tuples
 *     records
 *     arrays
 *     slices
 *     maps
 *     options
 *     results
 *     references
 *     functions
 *     generic applications
 *     associated types
 *     linear types
 *     affine types
 *     dependent types
 *     probabilistic/uncertain types
 *     tensor types
 *     quantum types
 *     hardware abstractions
 *     resource abstractions
 *     domain-defined types
 *     future registered types
 *
 * without this file changing.
 *
 * ============================================================================
 * GENERIC SCALABILITY
 * ============================================================================
 *
 * Generic arity MUST remain open-ended.
 *
 * Examples include:
 *
 *     type Box<T> = ...
 *
 *     type Pair<A, B> = ...
 *
 *     type Tensor<T, Rank, Layout, Space> = ...
 *
 *     type QuantumState<T, Encoding, Space> = ...
 *
 *     struct Container<T, Policy, Layout> { ... }
 *
 * The grammar MUST NOT encode:
 *
 *     T1
 *     T2
 *     T3
 *
 * as a finite universal mechanism.
 *
 * Repetition is therefore expressed using:
 *
 *     *
 *     +
 *     ?
 *
 * and concrete arity constraints are semantic concerns.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * User-defined types describe source-level abstractions.
 *
 * They MUST NOT encode universal physical-machine limits.
 *
 * This file contains no limits for:
 *
 *     types
 *     aliases
 *     structs
 *     fields
 *     records
 *     enum variants
 *     union variants
 *     classes
 *     interfaces
 *     traits
 *     implementations
 *     generic parameters
 *     generic bounds
 *     associated types
 *     associated constants
 *     members
 *     type nesting
 *     type dimensions
 *     tensor rank
 *     qubits
 *     CPUs
 *     cores
 *     threads
 *     GPUs
 *     FPGAs
 *     QPUs
 *     accelerators
 *     nodes
 *     memory
 *     devices
 *
 * Therefore there is no:
 *
 *     MAX_TYPES
 *     MAX_FIELDS
 *     MAX_VARIANTS
 *     MAX_GENERIC_PARAMETERS
 *     MAX_TYPE_DEPTH
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_MEMORY
 *     MAX_DEVICES
 *
 * or equivalent grammar constant.
 *
 * Practical compiler resource limits are implementation policies and MUST NOT
 * become source-language semantics.
 *
 * ============================================================================
 * TARGET NEUTRALITY
 * ============================================================================
 *
 * A user-defined type may eventually be used by:
 *
 *     CPU
 *     multicore CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     QPU
 *     simulator
 *     embedded target
 *     HPC system
 *     cluster
 *     distributed system
 *     future computational substrate
 *
 * This grammar does not choose any of them.
 *
 * The source declaration expresses semantic intent.
 *
 * Resource requirements, capabilities, target discovery, specialization,
 * lowering, layout, routing and scheduling belong downstream.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * A user-defined type may name or contain quantum types.
 *
 * Examples:
 *
 *     type State = quantum::State;
 *
 *     type Register<T> = quantum::Register<T>;
 *
 *     struct HybridState<T> {
 *         classical: T,
 *         quantum: quantum::State
 *     }
 *
 * This file does NOT enumerate quantum gates or physical qubits.
 *
 * It does NOT construct quantum IR.
 *
 * The semantic pipeline remains:
 *
 *     user-defined type
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic quantum type
 *          |
 *          v
 *     quantum::ir
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * User-defined types may represent HDL/hardware abstractions, but this grammar
 * remains unaware of:
 *
 *     bus width
 *     register count
 *     memory capacity
 *     FPGA resources
 *     ASIC geometry
 *     clock count
 *     pipeline depth
 *     device count
 *     physical location
 *
 * Such properties are represented by semantic/resource/capability models.
 *
 * ============================================================================
 * AI / DATA / DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * The same user-defined-type system may represent:
 *
 *     model types
 *     tensor types
 *     graph types
 *     knowledge types
 *     distributed messages
 *     actor state
 *     network messages
 *     data schemas
 *     provenance records
 *     policy objects
 *     evidence objects
 *     uncertainty structures
 *
 * These domains do not require additional type-declaration syntax here.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Declaring a type is not itself assumed to perform an execution effect.
 *
 * Effects arise from the semantic meaning of:
 *
 *     initialization
 *     compile-time evaluation
 *     reflection
 *     code generation
 *     external resources
 *     foreign interfaces
 *     adaptation
 *     simulation
 *
 * The effect system owns those classifications.
 *
 * This file MUST NOT encode effect semantics.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * A type may eventually require capabilities through its semantic definition,
 * associated operations, attributes, policies or resource declarations.
 *
 * This file does not resolve capabilities.
 *
 * Capability negotiation belongs downstream.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Type declarations may ultimately participate in resource requirements such
 * as:
 *
 *     memory
 *     storage
 *     compute
 *     quantum resources
 *     tensor resources
 *     topology
 *     communication
 *     accelerators
 *
 * The grammar does not resolve any requirement.
 *
 * It also does not impose a resource maximum.
 *
 * ============================================================================
 * CONTRACT / POLICY CONTRACT
 * ============================================================================
 *
 * Contracts and policies may constrain user-defined types downstream.
 *
 * Examples include:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * and policy concepts such as:
 *
 *     allow
 *     forbid
 *     constrain
 *     prefer
 *
 * Those constructs have their own grammar/semantic owners.
 *
 * This file only provides the type-declaration composition boundary.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * A type declaration must retain enough source structure for downstream
 * provenance systems to identify:
 *
 *     source declaration
 *     declared name
 *     generic parameters
 *     referenced types
 *     source span
 *     declaration kind
 *
 * Transformations after parsing may additionally record:
 *
 *     derived_from
 *     transformed_by
 *     generated_by
 *     verified_by
 *     reason
 *
 * Provenance semantics belong downstream.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar does not generate IR.
 *
 * Type declarations are lowered only after:
 *
 *     parsing
 *     AST construction
 *     name resolution
 *     type resolution
 *     constraint checking
 *     semantic validation
 *
 * Depending on their use, they may contribute to:
 *
 *     classical IR
 *     quantum::ir
 *     HDL/hardware representation
 *     data representation
 *     distributed representation
 *     other future canonical IR forms
 *
 * The grammar remains independent of those representations.
 *
 * ============================================================================
 * DECLARATION FAMILY
 * ============================================================================
 *
 * `userDefinedTypeDeclaration` intentionally composes the following public
 * declaration rules:
 *
 *     typeDeclarationItem
 *     structDeclaration
 *     enumDeclaration
 *     recordDeclaration
 *     unionDeclaration
 *     classDeclaration
 *     interfaceDeclaration
 *     traitDeclaration
 *     implementationDeclaration
 *
 * The `typeDeclarationItem` rule is used for the type/alias family because
 * grammar/declarations/types.g4 already owns both:
 *
 *     typeDeclaration
 *     typeAliasDeclaration
 *
 * This avoids importing a second alias grammar and prevents duplicate
 * `typeAliasDeclaration` ownership.
 *
 * ============================================================================
 * IMPORTS
 * ============================================================================
 */

parser grammar UserDefinedTypes;

options {
    tokenVocab = ZamaniLexer;
}

import
    ZamaniDeclarationTypesParser,
    ZamaniDeclarationStructs,
    ZamaniDeclarationEnums,
    ZamaniDeclarationRecords,
    ZamaniDeclarationUnions,
    ZamaniClasses,
    Interfaces,
    Traits,
    ZamaniImplementations
    ;


/*
 * ============================================================================
 * CANONICAL USER-DEFINED-TYPE STATEMENT
 * ============================================================================
 *
 * This is the only public rule that statement composition should consume.
 *
 * The extra layer is deliberate:
 *
 *     userDefinedTypeStatement
 *          |
 *          v
 *     userDefinedTypeDeclaration
 *
 * It gives the AST/parser integration a stable statement-category boundary
 * without forcing concrete declaration grammars to know anything about the
 * statement system.
 */

userDefinedTypeStatement
    : userDefinedTypeDeclaration
    ;


/*
 * ============================================================================
 * USER-DEFINED-TYPE DECLARATION FAMILY
 * ============================================================================
 *
 * Concrete syntax is delegated entirely to the declaration grammars.
 *
 * No rule below redefines their implementation.
 */

userDefinedTypeDeclaration
    : typeDeclarationItem
    | structDeclaration
    | enumDeclaration
    | recordDeclaration
    | unionDeclaration
    | classDeclaration
    | interfaceDeclaration
    | traitDeclaration
    | implementationDeclaration
    ;


/*
 * ============================================================================
 * EXPLICIT FAMILY ADAPTERS
 * ============================================================================
 *
 * These adapters provide stable semantic/parser integration points.
 *
 * They are intentionally aliases to the concrete declaration rules.
 *
 * They do not create new syntax.
 *
 * They are useful for tooling that needs to distinguish declaration families
 * without coupling itself to the internal rule organization of each concrete
 * declaration grammar.
 */

namedTypeDeclaration
    : typeDeclaration
    ;


typeAliasTypeDeclaration
    : typeAliasDeclaration
    ;


structTypeDeclaration
    : structDeclaration
    ;


recordTypeDeclaration
    : recordDeclaration
    ;


enumTypeDeclaration
    : enumDeclaration
    ;


unionTypeDeclaration
    : unionDeclaration
    ;


classTypeDeclaration
    : classDeclaration
    ;


interfaceTypeDeclaration
    : interfaceDeclaration
    ;


traitTypeDeclaration
    : traitDeclaration
    ;


implementationTypeDeclaration
    : implementationDeclaration
    ;


/*
 * ============================================================================
 * DECLARATION-KIND DISPATCH
 * ============================================================================
 *
 * This rule is intentionally structural.
 *
 * It does not inspect identifiers, names, target domains or semantic types.
 *
 * It exists for parser consumers that need one stable family dispatcher.
 */

userDefinedTypeKind
    : namedTypeDeclaration
    | typeAliasTypeDeclaration
    | structTypeDeclaration
    | recordTypeDeclaration
    | enumTypeDeclaration
    | unionTypeDeclaration
    | classTypeDeclaration
    | interfaceTypeDeclaration
    | traitTypeDeclaration
    | implementationTypeDeclaration
    ;


/*
 * ============================================================================
 * NO SECOND TYPE SYSTEM
 * ============================================================================
 *
 * DO NOT add rules such as:
 *
 *     userDefinedTypeExpression
 *     userDefinedGenericType
 *     userDefinedStructType
 *     userDefinedQuantumType
 *     userDefinedHardwareType
 *
 * merely to make this file appear more complete.
 *
 * A declaration name becomes a type through semantic name resolution.
 *
 * The canonical type-expression system remains under:
 *
 *     grammar/types/
 *
 * This distinction is essential:
 *
 *     declaration syntax
 *          !=
 *     type-expression syntax
 *          !=
 *     semantic type identity
 *
 * ============================================================================
 * OPEN-WORLD DOMAIN CONTRACT
 * ============================================================================
 *
 * This family is intentionally open to future domains.
 *
 * A future domain can introduce a declaration grammar without modifying this
 * file only if that declaration is semantically classified as a user-defined
 * type and is exposed through the canonical declaration composition boundary.
 *
 * Domain examples include:
 *
 *     classical
 *     quantum
 *     HDL
 *     hardware
 *     accelerator
 *     AI
 *     data
 *     distributed
 *     networking
 *     cryptographic
 *     simulation
 *     future computational domains
 *
 * Domain-specific declarations MUST NOT introduce universal machine limits.
 *
 * ============================================================================
 * SOURCE-ORDER CONTRACT
 * ============================================================================
 *
 * The parser must preserve source ordering.
 *
 * In particular, downstream AST construction must retain the ordering of:
 *
 *     declarations
 *     generic parameters
 *     bounds
 *     fields
 *     variants
 *     class members
 *     interface members
 *     trait members
 *     implementation members
 *
 * Semantic normalization may canonicalize or reorder where language semantics
 * permit, but that is downstream from parsing.
 *
 * ============================================================================
 * ERROR BOUNDARY
 * ============================================================================
 *
 * Syntax errors belong to the parser.
 *
 * Examples:
 *
 *     type
 *     type =
 *     type Name =
 *     struct
 *     struct Name {
 *     enum
 *     enum Name {
 *     record
 *     record Name {
 *     union
 *     union Name {
 *     class
 *     class Name {
 *     interface
 *     interface Name {
 *     trait
 *     trait Name {
 *     impl
 *     impl Trait for
 *
 * Semantic errors do NOT belong here.
 *
 * Examples:
 *
 *     duplicate type name
 *     unknown referenced type
 *     recursive alias cycle
 *     invalid generic bound
 *     unsatisfied generic bound
 *     duplicate implementation
 *     incoherent implementation
 *     missing trait member
 *     invalid associated type
 *     invalid resource requirement
 *     unavailable capability
 *     invalid effect
 *
 * Those belong to semantic analysis and structural validation.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar is deterministic with respect to:
 *
 *     source text
 *     lexer configuration
 *     parser grammar version
 *     explicitly selected dialect configuration
 *
 * It must not depend on:
 *
 *     time
 *     randomness
 *     hardware
 *     filesystem state
 *     network state
 *     environment state
 *     runtime state
 *     target availability
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * No finite grammar cardinality is imposed on:
 *
 *     declarations
 *     fields
 *     variants
 *     members
 *     generic parameters
 *     bounds
 *     where constraints
 *     associated types
 *     associated constants
 *     nested declarations
 *
 * Repetition remains represented through parser repetition operators and
 * delegation to the owning declaration grammar.
 *
 * "Infinity" is therefore a language-level scalability principle rather than
 * an encoded integer bound.
 *
 * Actual compilation is necessarily constrained by available computational
 * resources, but such constraints remain implementation/resource policy rather
 * than language semantics.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no:
 *
 *     MAX_TYPES
 *     MAX_FIELDS
 *     MAX_VARIANTS
 *     MAX_GENERIC_PARAMETERS
 *     MAX_MEMBERS
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_DEVICES
 *     MAX_TENSOR_RANK
 *     MAX_NETWORK_SIZE
 *
 * It contains no:
 *
 *     physical qubit identifiers
 *     fixed hardware identifiers
 *     vendor-specific hardware categories
 *     fixed register widths
 *     fixed memory capacities
 *     fixed accelerator counts
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains no Rust implementation.
 *
 * Generated parser integration must remain compatible with:
 *
 *     Rust 1.97+
 *     Rust 2021
 *
 * The generated/runtime implementation must use safe Rust only.
 *
 * No unsafe code is required by this grammar.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * The following source forms must be represented by the composed parser,
 * subject to the exact concrete declaration syntax owned by their respective
 * declaration grammars.
 *
 * TYPE / ALIAS:
 *
 *     type Scalar = f64;
 *
 *     type Pair<A, B> = (A, B);
 *
 *     type QuantumState<T> = quantum::State<T>;
 *
 * STRUCT:
 *
 *     struct Particle {
 *         position: Vector;
 *         mass: Scalar;
 *     }
 *
 * RECORD:
 *
 *     record Measurement {
 *         value: Scalar;
 *         confidence: Probability;
 *     }
 *
 * ENUM:
 *
 *     enum Outcome {
 *         Success,
 *         Failure,
 *     }
 *
 * UNION:
 *
 *     union Storage {
 *         Scalar(Scalar),
 *         Vector(Vector),
 *     }
 *
 * CLASS:
 *
 *     class DeviceState {
 *         ...
 *     }
 *
 * INTERFACE:
 *
 *     interface Computable {
 *         ...
 *     }
 *
 * TRAIT:
 *
 *     trait Numeric {
 *         ...
 *     }
 *
 * IMPLEMENTATION:
 *
 *     impl Numeric for Scalar {
 *         ...
 *     }
 *
 * GENERIC:
 *
 *     struct Container<T> {
 *         value: T;
 *     }
 *
 * QUANTUM:
 *
 *     type QuantumState<T> = quantum::State<T>;
 *
 * HYBRID:
 *
 *     struct HybridState<T> {
 *         classical: T;
 *         quantum: quantum::State;
 *     }
 *
 * These tests are syntax/conformance tests.
 *
 * They do not assert that the referenced semantic types, traits, capabilities
 * or domains exist.
 *
 * ============================================================================
 * NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * The following malformed declarations must fail at the appropriate parser
 * boundary:
 *
 *     type
 *
 *     type Name
 *
 *     type = Value;
 *
 *     struct
 *
 *     struct Name {
 *
 *     enum
 *
 *     enum Name {
 *
 *     record
 *
 *     class
 *
 *     interface
 *
 *     trait
 *
 *     impl
 *
 *     impl Trait for
 *
 * Exact diagnostics belong to the parser/frontend diagnostic layer.
 *
 * ============================================================================
 * BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Test at least:
 *
 *     one type parameter
 *     many type parameters
 *     nested generic types
 *     deeply qualified names
 *     nested user-defined types
 *     empty declaration bodies where the concrete declaration grammar permits
 *     one field
 *     many fields
 *     one enum variant
 *     many variants
 *     associated types
 *     associated constants
 *     generic implementations
 *     where clauses
 *     quantum-containing types
 *     classical/quantum hybrid types
 *     HDL/hardware-associated types
 *     distributed message types
 *     AI/model types
 *     data schema types
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Scalability tests must verify that no grammar-level capacity is introduced
 * by this composition file.
 *
 * In particular, tests should exercise generated/source fixtures with:
 *
 *     large declaration families
 *     large generic signatures
 *     large field sets
 *     large variant sets
 *     nested type expressions
 *     large implementation bodies
 *
 * Any failure caused by parser memory/time limits must be classified as an
 * implementation resource limitation rather than a language cardinality rule.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * This file is a composition boundary and therefore should remain stable even
 * when an individual declaration grammar evolves internally.
 *
 * Concrete declaration grammars may add new syntax without changing this file
 * provided their public exported rule remains stable.
 *
 * If a public declaration rule must be renamed, that is a deliberate grammar
 * compatibility change and must update:
 *
 *     this file
 *     declarations.g4
 *     specification
 *     conformance metadata
 *     parser tests
 *     AST integration
 *
 * together.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * STEP 1
 * ------
 *
 * Add this grammar to:
 *
 *     grammar/statements/
 *
 * as:
 *
 *     user-defined-types.g4
 *
 * STEP 2
 * ------
 *
 * Update:
 *
 *     grammar/statements/declarations.g4
 *
 * so it imports:
 *
 *     UserDefinedTypes
 *
 * STEP 3
 * ------
 *
 * Replace the existing concrete user-defined-type alternatives in
 * `declarationStatement` with the single adapter:
 *
 *     userDefinedTypeStatement
 *
 * STEP 4
 * ------
 *
 * Remove duplicate concrete definitions from statements/declarations.g4.
 *
 * In particular, the statement declaration dispatcher MUST NOT continue to
 * own independent copies of:
 *
 *     typeAliasDeclaration
 *     structDeclaration
 *     recordDeclaration
 *     enumDeclaration
 *     unionDeclaration
 *     classDeclaration
 *     interfaceDeclaration
 *     traitDeclaration
 *     implDeclaration
 *
 * STEP 5
 * ------
 *
 * Ensure the specialized declaration grammars remain the concrete owners:
 *
 *     types.g4
 *     structs.g4
 *     enums.g4
 *     records.g4
 *     unions.g4
 *     classes.g4
 *     interfaces.g4
 *     traits.g4
 *     implementations.g4
 *
 * STEP 6
 * ------
 *
 * Ensure:
 *
 *     grammar/statements/statements.g4
 *
 * continues to receive declarations through:
 *
 *     declarationStatement
 *
 * and does NOT import this file directly.
 *
 * The dependency direction remains:
 *
 *     statements
 *          |
 *          v
 *     declarations
 *          |
 *          v
 *     user-defined-types
 *          |
 *          v
 *     concrete declarations
 *
 * STEP 7
 * ------
 *
 * Ensure:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * continues to consume the canonical statement composition rather than
 * importing individual user-defined-type grammars.
 *
 * STEP 8
 * ------
 *
 * The root:
 *
 *     grammar/Zamani.g4
 *
 * remains unchanged.
 *
 * It continues to import only:
 *
 *     ZamaniParser
 *     ZamaniLexer
 *
 * STEP 9
 * ------
 *
 * The AST layer must map the concrete declaration nodes to the existing
 * canonical AST declarations.
 *
 * This grammar MUST NOT require a new semantic `UserDefinedType` universe.
 *
 * STEP 10
 * -------
 *
 * Semantic analysis must resolve:
 *
 *     declaration identity
 *     type identity
 *     generic parameters
 *     bounds
 *     implementations
 *     associated types
 *     associated constants
 *
 * after parsing.
 *
 * STEP 11
 * -------
 *
 * The compiler must preserve user-defined type semantics through:
 *
 *     AST
 *       |
 *       v
 *     semantic type system
 *       |
 *       v
 *     canonical IR
 *       |
 *       v
 *     optimization/lowering
 *
 * with quantum-containing declarations ultimately capable of contributing to:
 *
 *     quantum::ir
 *
 * where semantically appropriate.
 *
 * STEP 12
 * -------
 *
 * Add tests under:
 *
 *     grammar/tests/parser/user-defined-types/
 *
 * and, where the repository's current test organization differs, extend the
 * existing declaration/type conformance suites rather than creating a second
 * incompatible test hierarchy.
 *
 * ============================================================================
 * FILE COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when all of the following are true:
 *
 * [ ] It is the only statement-layer composition owner for user-defined types.
 *
 * [ ] It does not redefine concrete declaration syntax.
 *
 * [ ] It imports the canonical concrete declaration grammars.
 *
 * [ ] It exports a stable userDefinedTypeStatement rule.
 *
 * [ ] It exports a stable userDefinedTypeDeclaration rule.
 *
 * [ ] Alias/type declaration ownership remains in declarations/types.g4.
 *
 * [ ] Struct ownership remains in declarations/structs.g4.
 *
 * [ ] Enum ownership remains in declarations/enums.g4.
 *
 * [ ] Record ownership remains in declarations/records.g4.
 *
 * [ ] Union ownership remains in declarations/unions.g4.
 *
 * [ ] Class ownership remains in declarations/classes.g4.
 *
 * [ ] Interface ownership remains in declarations/interfaces.g4.
 *
 * [ ] Trait ownership remains in declarations/traits.g4.
 *
 * [ ] Implementation ownership remains in declarations/implementations.g4.
 *
 * [ ] statements/declarations.g4 no longer duplicates these concrete rules.
 *
 * [ ] statements/statements.g4 continues to consume declarationStatement.
 *
 * [ ] ZamaniParser.g4 does not bypass the declaration composition boundary.
 *
 * [ ] Zamani.g4 remains only the root composition grammar.
 *
 * [ ] No fixed capacity exists anywhere in this file.
 *
 * [ ] No target-specific syntax exists in this file.
 *
 * [ ] No quantum gate enumeration exists in this file.
 *
 * [ ] No IR is constructed in this file.
 *
 * [ ] No backend is selected in this file.
 *
 * [ ] No unsafe Rust is required.
 *
 * [ ] Positive tests exist.
 *
 * [ ] Negative tests exist.
 *
 * [ ] Boundary tests exist.
 *
 * [ ] Scalability tests exist.
 *
 * [ ] Cross-domain tests exist.
 *
 * [ ] Compatibility/conformance status is recorded.
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */
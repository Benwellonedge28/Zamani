/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/declarations/user-defined-types.g4
 *
 * Grammar:
 *     ZamaniUserDefinedTypes
 *
 * Status:
 *     CANONICAL USER-DEFINED-TYPE DECLARATION COMPOSITION GRAMMAR
 *
 * Compiler baseline:
 *     Rust 1.97 or later
 *     Rust 2021
 *
 * Safety:
 *     Safe Rust only.
 *     No unsafe Rust is required by this grammar.
 *     No embedded Rust actions.
 *     No semantic predicates.
 *     No filesystem access.
 *     No network access.
 *     No hardware discovery.
 *     No target selection.
 *     No runtime execution.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the SINGLE COMPOSITION OWNER for SOURCE-LEVEL
 * USER-DEFINED-TYPE DECLARATION FAMILIES.
 *
 * It does NOT implement the concrete syntax of every UDT family.
 *
 * Concrete declaration syntax remains owned by the existing dedicated
 * declaration grammars:
 *
 *     grammar/declarations/types.g4
 *     grammar/declarations/aliases.g4
 *     grammar/declarations/structs.g4
 *     grammar/declarations/records.g4
 *     grammar/declarations/enums.g4
 *     grammar/declarations/unions.g4
 *
 * This file composes those declarations into one reusable UDT boundary.
 *
 * ============================================================================
 * WHY THIS FILE EXISTS
 * ============================================================================
 *
 * A user-defined type is a LANGUAGE-LEVEL TYPE CONCEPT.
 *
 * It is not a new type-expression language.
 *
 * It is not a second AST.
 *
 * It is not a second semantic type system.
 *
 * It is not a backend representation.
 *
 * It is not a quantum-specific type system.
 *
 * It is not a hardware-specific type system.
 *
 * The intended architecture is:
 *
 *     UDT declaration
 *          |
 *          v
 *     declaration grammar
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     canonical type resolution
 *          |
 *          v
 *     canonical semantic type
 *          |
 *          +-----------------------+
 *          |                       |
 *          v                       v
 *     classical semantics      quantum semantics
 *          |                       |
 *          v                       v
 *     classical IR             quantum::ir
 *          |                       |
 *          +-----------+-----------+
 *                      |
 *                      v
 *                target-independent
 *                  optimization
 *                      |
 *                      v
 *                 specialization
 *                      |
 *                      v
 *             resource/capability
 *                negotiation
 *                      |
 *                      v
 *              routing/scheduling
 *                      |
 *                      v
 *                 resilience
 *                      |
 *                      v
 *                     ZQN
 *                      |
 *                      v
 *                     HAL
 *                      |
 *                      v
 *              target realization
 *
 * ============================================================================
 * NORMATIVE OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - userDefinedTypeDeclaration;
 *     - user-defined-type declaration-family composition;
 *     - classification of type declarations versus aliases versus aggregates;
 *     - the reusable parser boundary for UDT declarations;
 *     - declaration-family dispatch for UDTs.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical tokens;
 *     - token spelling;
 *     - identifiers;
 *     - qualified names;
 *     - attributes;
 *     - visibility syntax;
 *     - modifiers;
 *     - generic parameter implementation;
 *     - type-expression implementation;
 *     - type paths;
 *     - primitive types;
 *     - generic type applications;
 *     - tuple types;
 *     - arrays;
 *     - maps;
 *     - references;
 *     - pointers;
 *     - quantum types;
 *     - hardware types;
 *     - resource types;
 *     - effect-qualified types;
 *     - contracts;
 *     - policies;
 *     - provenance;
 *     - class declarations;
 *     - interface declarations;
 *     - trait declarations;
 *     - implementation declarations;
 *     - resource declarations;
 *     - capability declarations;
 *     - domain declarations;
 *     - functions;
 *     - modules;
 *     - statements;
 *     - expressions;
 *     - name resolution;
 *     - type checking;
 *     - generic constraint solving;
 *     - ownership analysis;
 *     - linearity analysis;
 *     - affinity analysis;
 *     - lifetime analysis;
 *     - ABI;
 *     - memory layout;
 *     - serialization;
 *     - target discovery;
 *     - hardware selection;
 *     - physical qubit allocation;
 *     - quantum routing;
 *     - scheduling;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime execution.
 *
 * ============================================================================
 * CANONICAL SPECIFICATION
 * ============================================================================
 *
 * The semantic contract for this composition boundary is defined by:
 *
 *     grammar/spec/user-defined-types.md
 *
 * General type semantics are defined by:
 *
 *     grammar/spec/type-system.md
 *     grammar/specification/types.md
 *
 * The canonical source-level type-expression grammar is:
 *
 *     grammar/types/types.g4
 *
 * The declaration-family dispatcher is:
 *
 *     grammar/declarations/declarations.g4
 *
 * This file MUST NOT replace those authorities.
 *
 * ============================================================================
 * SINGLE TYPE-SYSTEM RULE
 * ============================================================================
 *
 * There MUST be exactly one universal source-level type-expression entry
 * point:
 *
 *     typeExpression
 *
 * That rule belongs to:
 *
 *     grammar/types/types.g4
 *
 * This file MUST NOT define:
 *
 *     typeExpression
 *     userDefinedTypeExpression
 *     udtTypeExpression
 *     universalUserTypeExpression
 *
 * or any other competing universal type root.
 *
 * A user-defined type is referenced through the normal canonical type
 * expression system.
 *
 * ============================================================================
 * UDT FAMILY MODEL
 * ============================================================================
 *
 * The UDT declaration family is intentionally limited to declarations that
 * establish or define source-level semantic types.
 *
 * Canonical UDT families:
 *
 *     named type declaration
 *     type alias
 *     struct
 *     record
 *     enum
 *     union
 *
 * These are deliberately composed here rather than reimplemented.
 *
 * ============================================================================
 * NOMINAL TYPES
 * ============================================================================
 *
 * A named type declaration establishes a declaration-level type identity.
 *
 * Semantic identity is determined downstream from stable declaration
 * information such as:
 *
 *     namespace
 *     module
 *     qualified name
 *     declaration identity
 *     generic arguments where applicable
 *
 * This file does NOT determine type identity.
 *
 * It only preserves the declaration boundary required by the semantic layer.
 *
 * ============================================================================
 * ALIASES
 * ============================================================================
 *
 * A type alias is intentionally kept distinct from a nominal type declaration.
 *
 * For example:
 *
 *     type UserId = Integer;
 *
 * and:
 *
 *     type UserId {
 *         value: Integer;
 *     }
 *
 * MUST NOT automatically acquire the same semantic meaning.
 *
 * Alias expansion, nominal identity, compatibility and conversion are
 * semantic responsibilities.
 *
 * ============================================================================
 * STRUCTS / RECORDS
 * ============================================================================
 *
 * Structs and records are UDT declaration families.
 *
 * Their concrete field syntax remains owned by:
 *
 *     structs.g4
 *     records.g4
 *
 * This composition boundary does not inspect or reinterpret fields.
 *
 * Field semantics belong to the declaration/type semantic layer.
 *
 * ============================================================================
 * ENUMS
 * ============================================================================
 *
 * Enums are UDTs whose variants are owned by:
 *
 *     enums.g4
 *
 * This file does not:
 *
 *     - assign discriminants;
 *     - select representations;
 *     - determine layout;
 *     - impose variant limits;
 *     - select a machine representation.
 *
 * ============================================================================
 * UNIONS / SUM TYPES
 * ============================================================================
 *
 * Unions are UDTs whose variant payload syntax is owned by:
 *
 *     unions.g4
 *
 * This file only composes the declaration into the UDT family.
 *
 * ============================================================================
 * OPEN-WORLD DESIGN
 * ============================================================================
 *
 * UDT composition MUST remain open-ended.
 *
 * The language MUST NOT require this file to be modified merely because a
 * future semantic type family is introduced.
 *
 * However, a new declaration family MUST NOT bypass this composition boundary
 * when it is formally classified as a core UDT declaration family.
 *
 * Future extensions may be provided through:
 *
 *     libraries
 *     dialects
 *     metadata
 *     semantic extensions
 *     capabilities
 *     resource contracts
 *     interoperability
 *
 * where appropriate.
 *
 * ============================================================================
 * DOMAIN NEUTRALITY
 * ============================================================================
 *
 * UDTs may represent semantic concepts used by:
 *
 *     classical computing
 *     scientific computing
 *     numerical computing
 *     symbolic computing
 *     quantum computing
 *     hybrid computing
 *     HDL
 *     hardware/software co-design
 *     AI
 *     machine learning
 *     tensor computation
 *     data processing
 *     distributed computing
 *     networking
 *     cryptography
 *     embedded computing
 *     accelerators
 *     HPC
 *     cloud computing
 *     future computational domains
 *
 * This file does NOT create separate grammar branches for those domains.
 *
 * For example, it MUST NOT add:
 *
 *     quantumUserDefinedType
 *     gpuUserDefinedType
 *     tensorUserDefinedType
 *     aiUserDefinedType
 *     hdlUserDefinedType
 *
 * unless a future specification establishes a genuinely distinct declaration
 * syntax that cannot be represented by the existing UDT model.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * A user-defined type may reference a quantum type through the canonical
 * type-expression system.
 *
 * Example semantic intent:
 *
 *     type Register<T> = quantum::Register<T>;
 *
 * This file does NOT:
 *
 *     - enumerate quantum gates;
 *     - enumerate physical qubits;
 *     - allocate qubits;
 *     - select QPUs;
 *     - inspect topology;
 *     - perform routing;
 *     - perform scheduling;
 *     - perform QEC;
 *     - construct quantum::ir.
 *
 * The required path is:
 *
 *     UDT declaration
 *          |
 *          v
 *     TypeExpr
 *          |
 *          v
 *     semantic type
 *          |
 *          v
 *     quantum semantic lowering
 *          |
 *          v
 *     quantum::ir
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * A UDT may reference hardware-independent semantic types.
 *
 * The declaration MUST NOT directly encode:
 *
 *     physical addresses
 *     register identifiers
 *     fixed bus widths as universal limits
 *     FPGA locations
 *     ASIC cells
 *     GPU addresses
 *     device identifiers
 *     fixed machine topology
 *
 * Such information belongs to the appropriate hardware/HDL semantic and
 * target-realization layers.
 *
 * ============================================================================
 * GENERIC INTEGRATION
 * ============================================================================
 *
 * Generic syntax remains owned by the concrete declaration grammars and the
 * canonical generic/type subsystem.
 *
 * This file MUST NOT define another generic parameter grammar.
 *
 * Generic cardinality is intentionally unbounded by language design.
 *
 * There MUST be no grammar-level:
 *
 *     MAX_GENERIC_PARAMETERS
 *     MAX_GENERIC_ARGUMENTS
 *
 * ============================================================================
 * RECURSION
 * ============================================================================
 *
 * Recursive and mutually recursive UDTs are semantic type relationships.
 *
 * This file MUST NOT attempt to expand recursive types.
 *
 * Examples:
 *
 *     type Node<T> {
 *         value: T;
 *         next: Reference<Node<T>>;
 *     }
 *
 *     type A {
 *         value: Reference<B>;
 *     }
 *
 *     type B {
 *         value: Reference<A>;
 *     }
 *
 * Recursive validity is determined downstream.
 *
 * ============================================================================
 * CONTRACT INTEGRATION
 * ============================================================================
 *
 * Contracts are not owned here.
 *
 * UDTs may participate in:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *     assert
 *
 * The contract subsystem remains authoritative.
 *
 * This file merely preserves the declaration boundary at which contracts may
 * subsequently attach according to the owning declaration grammar.
 *
 * ============================================================================
 * EFFECT INTEGRATION
 * ============================================================================
 *
 * A UDT declaration is not inherently effectful.
 *
 * In particular, defining a type MUST NOT automatically introduce:
 *
 *     io
 *     network
 *     mutation
 *     randomness
 *     native
 *     foreign
 *     distributed
 *     measurement
 *     quantum
 *     learning
 *     adaptation
 *     reflection
 *     code_generation
 *     simulation
 *
 * Effects arise from semantic operations or explicitly effectful declarations.
 *
 * ============================================================================
 * CAPABILITY INTEGRATION
 * ============================================================================
 *
 * UDTs may participate in capability-constrained semantic operations.
 *
 * This file MUST NOT resolve capabilities.
 *
 * It MUST NOT turn a type declaration into a physical device requirement.
 *
 * For example, a type may eventually be associated semantically with:
 *
 *     capability("quantum.measurement")
 *     capability("tensor.compute")
 *
 * but capability resolution belongs downstream.
 *
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * UDTs may participate in resource requirements.
 *
 * This file MUST NOT determine whether a target has those resources.
 *
 * Examples of downstream semantic requirements include:
 *
 *     memory >= required_memory
 *     qubits >= required_qubits
 *     topology(required_topology)
 *
 * No universal resource maximum is represented here.
 *
 * ============================================================================
 * POLICY INTEGRATION
 * ============================================================================
 *
 * Policies govern realization and authorization.
 *
 * This file MUST NOT implement policy evaluation.
 *
 * UDTs may be affected by policies such as:
 *
 *     deterministic execution
 *     simulation permission
 *     foreign-call restrictions
 *     resource preferences
 *     target constraints
 *
 * Policy semantics remain outside this parser grammar.
 *
 * ============================================================================
 * PROVENANCE INTEGRATION
 * ============================================================================
 *
 * UDT declarations MUST preserve sufficient parse-tree/source information for
 * downstream provenance.
 *
 * Relevant provenance may include:
 *
 *     source span
 *     declaration identity
 *     module
 *     namespace
 *     source version
 *     dialect
 *     generated/derived status
 *
 * This file does not create provenance records itself.
 *
 * ============================================================================
 * DIALECT INTEGRATION
 * ============================================================================
 *
 * A dialect MAY introduce additional type declarations only through the
 * repository's established dialect extension architecture.
 *
 * A dialect MUST NOT silently replace:
 *
 *     typeExpression
 *     userDefinedTypeDeclaration
 *
 * or redefine the meaning of an existing core UDT declaration.
 *
 * Dialect-specific declarations must remain distinguishable from core
 * declarations at the semantic/specification level.
 *
 * ============================================================================
 * FOREIGN / ABI INTEGRATION
 * ============================================================================
 *
 * Foreign types are not core UDT declarations merely because they have a
 * type-like name.
 *
 * Foreign/ABI declarations remain owned by:
 *
 *     grammar/interoperability/
 *
 * A foreign type may participate in the canonical type system after the
 * interoperability layer has established its semantic contract.
 *
 * This file MUST NOT define:
 *
 *     C ABI types
 *     C++ ABI types
 *     Rust ABI types
 *     platform ABI types
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar produces ANTLR parse-tree structure only.
 *
 * The frontend adapter MUST map each selected branch to the existing
 * domain-neutral AST representation.
 *
 * The UDT dispatcher MUST NOT introduce:
 *
 *     UserDefinedTypeAst
 *     UdtDeclarationAst
 *     UniversalTypeAst
 *     QuantumTypeDeclarationAst
 *     HardwareTypeDeclarationAst
 *
 * merely to represent this composition boundary.
 *
 * The concrete declaration grammars already establish the relevant AST
 * contracts.
 *
 * ============================================================================
 * REQUIRED AST PRESERVATION
 * ============================================================================
 *
 * The parser/frontend integration MUST preserve:
 *
 *     declaration kind
 *     source span
 *     source ordering
 *     declaration name where applicable
 *     attributes where applicable
 *     visibility where applicable
 *     modifiers where applicable
 *     generic parameters
 *     generic bounds
 *     declaration members
 *     variants
 *     child type expressions
 *     child expressions where applicable
 *     documentation where supported
 *
 * This file MUST NOT discard information merely because it does not need the
 * information for dispatch.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing establishes structural classification only.
 *
 * Later semantic analysis is responsible for:
 *
 *     name resolution
 *     namespace resolution
 *     duplicate declaration detection
 *     nominal identity
 *     alias semantics
 *     generic resolution
 *     bound checking
 *     constraint solving
 *     recursive-type validation
 *     mutually recursive-type validation
 *     structural compatibility
 *     nominal compatibility
 *     conversion checking
 *     ownership
 *     linearity
 *     affinity
 *     lifetime
 *     effects
 *     capabilities
 *     resources
 *     contracts
 *     policies
 *     provenance
 *     dialect validity
 *     foreign type validity
 *     quantum type validity
 *     HDL/hardware validity
 *     target compatibility
 *
 * ============================================================================
 * TYPE IDENTITY
 * ============================================================================
 *
 * This grammar MUST NOT determine whether a declaration is:
 *
 *     nominal
 *     structural
 *     transparent
 *     opaque
 *     alias-like
 *
 * unless that distinction is already explicit in the concrete declaration
 * syntax.
 *
 * Semantic classification remains the responsibility of the type system.
 *
 * ============================================================================
 * SCALABILITY / POCO-REAF
 * ============================================================================
 *
 * This grammar imposes NO language-level finite limit on:
 *
 *     UDT declarations
 *     fields
 *     variants
 *     generic parameters
 *     generic bounds
 *     recursive relationships
 *     declaration count
 *     type-expression size
 *     type-expression nesting
 *
 * It MUST NOT contain:
 *
 *     MAX_USER_TYPES
 *     MAX_UDTS
 *     MAX_GENERIC_PARAMETERS
 *     MAX_GENERIC_ARGUMENTS
 *     MAX_STRUCT_FIELDS
 *     MAX_RECORD_FIELDS
 *     MAX_ENUM_VARIANTS
 *     MAX_UNION_VARIANTS
 *     MAX_TYPE_DEPTH
 *     MAX_TYPE_SIZE
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_ASICS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_MEMORY
 *     MAX_TENSOR_RANK
 *     MAX_NETWORK_SIZE
 *
 * Compiler resource exhaustion is an implementation/resource-policy concern.
 *
 * If a compiler cannot safely process an input because of an operational
 * resource limit, it MUST report an implementation/resource diagnostic rather
 * than redefining the language's type semantics.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This composition grammar is deterministic with respect to:
 *
 *     source text
 *     grammar version
 *     canonical lexer
 *     explicitly selected dialect configuration
 *
 * It MUST NOT depend on:
 *
 *     wall-clock time
 *     randomness
 *     hardware
 *     filesystem enumeration
 *     network state
 *     environment state
 *     compiler thread scheduling
 *     target availability
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This file consumes the canonical lexer vocabulary:
 *
 *     ZamaniLexer
 *
 * It MUST NOT define lexer rules.
 *
 * It MUST NOT introduce:
 *
 *     IDENT
 *     ID
 *     TYPE_NAME
 *     UDT_NAME
 *     STRUCT_NAME
 *     ENUM_NAME
 *
 * as alternative lexical identities.
 *
 * Identifiers remain identifiers.
 *
 * The semantic layer determines what an identifier denotes.
 *
 * ============================================================================
 * IMPORT CONTRACT
 * ============================================================================
 *
 * The imports below intentionally import only existing concrete declaration
 * grammars.
 *
 * They do NOT import:
 *
 *     ZamaniParser
 *     Zamani
 *     Core combined grammars
 *     lexer grammars
 *
 * Parser grammars compose parser grammars.
 *
 * ============================================================================
 */

parser grammar ZamaniUserDefinedTypes;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * CONCRETE DECLARATION DELEGATES
 * ============================================================================
 *
 * IMPORTANT:
 *
 * These imports are the existing concrete owners.
 *
 * This file must never copy their rules.
 * ============================================================================
 */

import
    ZamaniDeclarationTypesParser,
    ZamaniDeclarationAliases,
    ZamaniDeclarationStructs,
    ZamaniDeclarationRecords,
    ZamaniDeclarationEnums,
    ZamaniDeclarationUnions
    ;


/*
 * ============================================================================
 * PUBLIC UDT ENTRY POINT
 * ============================================================================
 *
 * This is the ONLY public rule owned by this composition file.
 *
 * It provides one stable declaration-family boundary to declarations.g4 and
 * other declaration composition layers.
 * ============================================================================
 */

userDefinedTypeDeclaration
    : namedTypeDeclaration
    | typeAliasDeclaration
    | structDeclaration
    | recordDeclaration
    | enumDeclaration
    | unionDeclaration
    ;


/*
 * ============================================================================
 * NAMED TYPE DECLARATION
 * ============================================================================
 *
 * The concrete `typeDeclaration` rule remains owned by:
 *
 *     grammar/declarations/types.g4
 *
 * Do NOT duplicate its implementation here.
 *
 * The alias declaration is deliberately separate:
 *
 *     typeAliasDeclaration
 *
 * because aliases and nominal type declarations are semantically distinct.
 * ============================================================================
 */

namedTypeDeclaration
    : typeDeclaration
    ;


/*
 * ============================================================================
 * ALIAS DECLARATION
 * ============================================================================
 *
 * Concrete ownership:
 *
 *     grammar/declarations/aliases.g4
 *
 * No alias syntax is repeated here.
 * ============================================================================
 */


/*
 * ============================================================================
 * STRUCT DECLARATION
 * ============================================================================
 *
 * Concrete ownership:
 *
 *     grammar/declarations/structs.g4
 *
 * No field syntax is repeated here.
 * ============================================================================
 */


/*
 * ============================================================================
 * RECORD DECLARATION
 * ============================================================================
 *
 * Concrete ownership:
 *
 *     grammar/declarations/records.g4
 *
 * No record member syntax is repeated here.
 * ============================================================================
 */


/*
 * ============================================================================
 * ENUM DECLARATION
 * ============================================================================
 *
 * Concrete ownership:
 *
 *     grammar/declarations/enums.g4
 *
 * No variant syntax is repeated here.
 * ============================================================================
 */


/*
 * ============================================================================
 * UNION DECLARATION
 * ============================================================================
 *
 * Concrete ownership:
 *
 *     grammar/declarations/unions.g4
 *
 * No union payload syntax is repeated here.
 * ============================================================================
 */


/*
 * ============================================================================
 * DECLARATION DISPATCH CONTRACT
 * ============================================================================
 *
 * grammar/declarations/declarations.g4 MUST consume:
 *
 *     userDefinedTypeDeclaration
 *
 * for the UDT declaration family.
 *
 * It MUST NOT redefine:
 *
 *     userDefinedTypeDeclaration
 *     namedTypeDeclaration
 *     typeDeclaration
 *     typeAliasDeclaration
 *     structDeclaration
 *     recordDeclaration
 *     enumDeclaration
 *     unionDeclaration
 *
 * The concrete rules remain owned by their respective imported grammars.
 *
 * ============================================================================
 * OBJECT / CONTRACT DECLARATIONS
 * ============================================================================
 *
 * The following remain outside this UDT dispatcher:
 *
 *     classDeclaration
 *     interfaceDeclaration
 *     traitDeclaration
 *     implementationDeclaration
 *
 * They remain owned by the object/contract declaration family.
 *
 * Their semantic relationship to types is resolved downstream.
 *
 * This prevents this file from becoming a second object-model dispatcher.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY DECLARATIONS
 * ============================================================================
 *
 * The following remain outside this UDT dispatcher:
 *
 *     resourceDeclaration
 *     capabilityDeclaration
 *     domainDeclaration
 *
 * They are source-level declaration families with different ownership and
 * semantic contracts.
 *
 * A type may reference resources or capabilities through the canonical type
 * and semantic systems without making those declarations UDT syntax.
 *
 * ============================================================================
 * TYPE-EXPRESSION BOUNDARY
 * ============================================================================
 *
 * This file does not define or import a competing type-expression root.
 *
 * The canonical type expression remains:
 *
 *     Types.typeExpression
 *
 * Concrete declaration grammars may consume that rule according to their
 * established contracts.
 *
 * Therefore:
 *
 *     userDefinedTypeDeclaration
 *          |
 *          +--> typeDeclaration
 *          |       |
 *          |       +--> canonical type-expression system
 *          |
 *          +--> typeAliasDeclaration
 *          |       |
 *          |       +--> canonical type-expression system
 *          |
 *          +--> structDeclaration
 *          |       |
 *          |       +--> canonical type-expression system
 *          |
 *          +--> recordDeclaration
 *          |       |
 *          |       +--> canonical type-expression system
 *          |
 *          +--> enumDeclaration
 *          |       |
 *          |       +--> canonical type-expression system
 *          |
 *          +--> unionDeclaration
 *                  |
 *                  +--> canonical type-expression system
 *
 * ============================================================================
 * ERROR OWNERSHIP
 * ============================================================================
 *
 * This file does not define semantic errors.
 *
 * Parser errors belong to the ANTLR parser/conformance layer.
 *
 * Semantic errors belong downstream.
 *
 * Examples:
 *
 *     duplicate type name
 *     duplicate field name
 *     duplicate variant name
 *     unknown type
 *     recursive type cycle violation
 *     unsatisfied generic bound
 *     incompatible type
 *     illegal conversion
 *     invalid ownership
 *     invalid resource requirement
 *     unavailable capability
 *     forbidden policy
 *
 * These MUST NOT be implemented as parser-level semantic predicates here.
 *
 * ============================================================================
 * SOURCE ORDER
 * ============================================================================
 *
 * The selected declaration branch must preserve the source span and source
 * ordering of the concrete declaration.
 *
 * This composition rule does not reorder declarations.
 *
 * ============================================================================
 * PROVENANCE
 * ============================================================================
 *
 * The parse tree MUST remain sufficient for the frontend to associate:
 *
 *     declaration kind
 *     declaration span
 *     declaration source
 *     declaration name
 *
 * with downstream provenance.
 *
 * This grammar itself does not create provenance records.
 *
 * ============================================================================
 * IR BOUNDARY
 * ============================================================================
 *
 * This grammar creates NO IR.
 *
 * In particular it MUST NOT create:
 *
 *     Classical IR
 *     quantum::ir
 *     HDL IR
 *     hardware IR
 *     ZQN
 *
 * The required path is:
 *
 *     source
 *       |
 *       v
 *     parse tree
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic type
 *       |
 *       +------------------+
 *       |                  |
 *       v                  v
 *     Classical IR     quantum::ir
 *
 * ============================================================================
 * BACKEND BOUNDARY
 * ============================================================================
 *
 * This grammar does not select:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     QPU
 *     simulator
 *     HPC target
 *     cluster
 *     distributed target
 *
 * A UDT remains semantically stable across targets.
 *
 * Only realization may differ.
 *
 * ============================================================================
 * RUST INTEGRATION CONTRACT
 * ============================================================================
 *
 * The grammar itself contains no Rust code.
 *
 * The Rust frontend consuming it MUST:
 *
 *     - support Rust 1.97 or later;
 *     - use Rust 2021 or later where repository policy permits;
 *     - use safe Rust;
 *     - contain no unsafe blocks;
 *     - contain no unsafe functions;
 *     - preserve source spans;
 *     - preserve declaration order;
 *     - distinguish parse errors from semantic errors;
 *     - preserve deterministic semantic processing;
 *     - avoid target-specific assumptions in the parser.
 *
 * This grammar imposes no machine-size assumptions on the Rust frontend.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * The following test categories are REQUIRED.
 *
 * POSITIVE:
 *
 *     simple named type
 *     alias
 *     generic named type
 *     generic alias
 *     struct
 *     record
 *     enum
 *     union
 *     recursive type
 *     mutually recursive type
 *     nested generic type
 *     quantum-referencing type
 *     hardware-independent type
 *     hybrid type
 *
 * NEGATIVE:
 *
 *     malformed declaration
 *     missing name
 *     malformed generic parameters
 *     malformed alias target
 *     malformed aggregate body
 *     malformed enum variant
 *     malformed union variant
 *
 * SEMANTIC:
 *
 *     duplicate names
 *     unknown names
 *     invalid bounds
 *     invalid recursive representation
 *     invalid alias cycles
 *     invalid conversions
 *
 * SCALABILITY:
 *
 *     many declarations
 *     many fields
 *     many variants
 *     many generic parameters
 *     deeply nested symbolic types
 *     large mutually recursive declaration graphs
 *
 * PORTABILITY:
 *
 *     same UDT source across different target profiles
 *
 * DETERMINISM:
 *
 *     repeated parsing
 *     repeated semantic resolution
 *     repeated canonicalization
 *
 * CROSS-DOMAIN:
 *
 *     classical UDT
 *     quantum UDT
 *     hybrid UDT
 *     HDL/hardware UDT
 *     AI/data UDT
 *     distributed UDT
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Scalability tests MUST NOT interpret a large test fixture as a universal
 * language capacity.
 *
 * For example:
 *
 *     1000 declarations
 *
 * is a test workload, not:
 *
 *     MAX_USER_TYPES = 1000
 *
 * Similarly:
 *
 *     10000 fields
 *
 * must never become:
 *
 *     MAX_FIELDS = 10000
 *
 * Test sizes measure implementation behavior.
 *
 * They do not define language semantics.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * This file must preserve compatibility with the repository's canonical
 * declaration rule:
 *
 *     declaration
 *
 * through:
 *
 *     declarations.g4
 *
 * Existing source forms accepted by the concrete declaration grammars must
 * remain accepted unless a normative compatibility change explicitly says
 * otherwise.
 *
 * Historical syntax must be handled by the compatibility layer rather than
 * silently duplicated here.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE only when:
 *
 *     [ ] the file is the sole owner of UDT-family composition;
 *     [ ] concrete UDT syntax remains in concrete declaration grammars;
 *     [ ] no typeExpression rule is duplicated;
 *     [ ] no second type system exists;
 *     [ ] no second AST exists;
 *     [ ] no second semantic type model exists;
 *     [ ] aliases remain distinct from nominal declarations;
 *     [ ] structs remain delegated to structs.g4;
 *     [ ] records remain delegated to records.g4;
 *     [ ] enums remain delegated to enums.g4;
 *     [ ] unions remain delegated to unions.g4;
 *     [ ] generic syntax remains delegated;
 *     [ ] type-expression syntax remains delegated;
 *     [ ] class/interface/trait/implementation ownership remains unchanged;
 *     [ ] resource/capability/domain ownership remains unchanged;
 *     [ ] no hardware realization is encoded;
 *     [ ] no quantum physical realization is encoded;
 *     [ ] no artificial capacity is encoded;
 *     [ ] no unsafe Rust is required;
 *     [ ] source ordering is preserved;
 *     [ ] source spans remain recoverable;
 *     [ ] semantic validation remains downstream;
 *     [ ] canonical AST integration is defined;
 *     [ ] canonical type-system integration is defined;
 *     [ ] canonical IR boundaries are defined;
 *     [ ] quantum::ir remains the quantum semantic boundary;
 *     [ ] positive tests exist;
 *     [ ] negative tests exist;
 *     [ ] semantic tests exist;
 *     [ ] scalability tests exist;
 *     [ ] determinism tests exist;
 *     [ ] portability tests exist;
 *     [ ] cross-domain tests exist;
 *     [ ] compatibility tests exist.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * This file provides:
 *
 *     ONE UDT DECLARATION COMPOSITION BOUNDARY
 *
 * while preserving:
 *
 *     ONE LEXER
 *     ONE DECLARATION DISPATCHER
 *     ONE TYPE-EXPRESSION SYSTEM
 *     ONE DOMAIN-NEUTRAL AST
 *     ONE CANONICAL SEMANTIC TYPE SYSTEM
 *     ONE CANONICAL CLASSICAL IR
 *     ONE CANONICAL quantum::ir
 *
 * The programmer defines semantic types.
 *
 * The compiler determines how those types can be realized.
 *
 * The physical target MUST NOT redefine the source-level meaning of a UDT.
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */
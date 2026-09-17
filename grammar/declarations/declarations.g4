/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/declarations/declarations.g4
 *
 * Role:
 *     AUTHORITATIVE DECLARATION COMPOSITION GRAMMAR
 *
 * Status:
 *     Production-target / canonical declaration dispatcher
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * Safety:
 *     This grammar contains no embedded Rust.
 *     No actions.
 *     No semantic predicates.
 *     No unsafe code.
 *     No filesystem access.
 *     No network access.
 *     No runtime access.
 *     No hardware discovery.
 *     No target selection.
 *     No physical-resource selection.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the SINGLE DECLARATION COMPOSITION OWNER.
 *
 * It answers:
 *
 *     "Which source-level declaration families are legal at a
 *      declaration boundary?"
 *
 * This file MUST NOT implement the concrete syntax of those declarations.
 *
 * Concrete declaration syntax belongs to the dedicated declaration grammars.
 *
 * ============================================================================
 * CANONICAL PIPELINE
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     grammar/antlr/ZamaniLexer.g4
 *          |
 *          v
 *     canonical Zamani parser
 *          |
 *          v
 *     declaration
 *          |
 *          +--> value declarations
 *          +--> type declarations
 *          +--> aggregate declarations
 *          +--> object-model declarations
 *          +--> contract declarations
 *          +--> implementation declarations
 *          +--> domain/resource/capability declarations
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
 *          +--> names
 *          +--> types
 *          +--> effects
 *          +--> ownership
 *          +--> capabilities
 *          +--> resources
 *          +--> portability
 *          +--> domain semantics
 *          |
 *          v
 *     canonical semantic representations
 *          |
 *          +--> classical representation
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
 *     routing / scheduling / resilience / QEC / ZQN
 *          |
 *          v
 *     HAL / target realization
 *          |
 *          v
 *     runtime
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - declaration
 *     - declaration-family dispatch
 *     - declaration-family grouping
 *     - declaration-layer composition
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexer rules
 *     - token definitions
 *     - keyword spelling
 *     - identifiers
 *     - qualified names
 *     - paths
 *     - attributes
 *     - visibility
 *     - modifiers
 *     - expressions
 *     - type expressions
 *     - generic-parameter syntax
 *     - fields
 *     - variants
 *     - methods
 *     - concrete class syntax
 *     - concrete struct syntax
 *     - concrete enum syntax
 *     - concrete union syntax
 *     - concrete alias syntax
 *     - concrete interface syntax
 *     - concrete trait syntax
 *     - concrete implementation syntax
 *     - concrete resource syntax
 *     - concrete capability syntax
 *     - concrete domain syntax
 *     - functions
 *     - modules
 *     - effects
 *     - statements
 *     - quantum semantics
 *     - HDL semantics
 *     - hardware semantics
 *     - semantic analysis
 *     - AST construction
 *     - IR construction
 *     - quantum::ir
 *     - QEC
 *     - ZQN
 *     - routing
 *     - scheduling
 *     - optimization
 *     - calibration
 *     - HAL
 *     - backend selection
 *     - runtime execution
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * There MUST be exactly one declaration dispatcher at the canonical
 * declaration boundary.
 *
 * This grammar MUST NOT contain another implementation of:
 *
 *     constantDeclaration
 *     variableDeclaration
 *     typeDeclaration
 *     typeAliasDeclaration
 *     structDeclaration
 *     recordDeclaration
 *     enumDeclaration
 *     unionDeclaration
 *     classDeclaration
 *     interfaceDeclaration
 *     traitDeclaration
 *     implementationDeclaration
 *     resourceDeclaration
 *     capabilityDeclaration
 *     domainDeclaration
 *
 * The concrete implementation of each rule belongs to its dedicated owner.
 *
 * ============================================================================
 * CANONICAL LEXER
 * ============================================================================
 *
 * All imported parser grammars MUST use:
 *
 *     options {
 *         tokenVocab = ZamaniLexer;
 *     }
 *
 * The canonical lexical authority is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * No declaration grammar may introduce:
 *
 *     ZamaniTokens
 *     K_*
 *     IDENT
 *     SEMI
 *
 * as an alternative lexical vocabulary.
 *
 * Existing grammars that still use such names are migration/conformance
 * surfaces and must be normalized to the canonical lexer vocabulary.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Declarations express source-level semantics and portable computational
 * intent.
 *
 * This dispatcher contains NO universal limits such as:
 *
 *     MAX_DECLARATIONS
 *     MAX_TYPES
 *     MAX_STRUCT_FIELDS
 *     MAX_ENUM_VARIANTS
 *     MAX_GENERIC_PARAMETERS
 *     MAX_TRAIT_MEMBERS
 *     MAX_INTERFACE_MEMBERS
 *     MAX_IMPLEMENTATION_MEMBERS
 *     MAX_RESOURCES
 *     MAX_CAPABILITIES
 *     MAX_DOMAINS
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
 *     MAX_VECTOR_WIDTH
 *
 * Practical limits are compiler/runtime/resource-policy concerns and MUST NOT
 * become source-language grammar limits.
 *
 * ============================================================================
 * OPEN-WORLD DESIGN
 * ============================================================================
 *
 * The declaration dispatcher deliberately does not enumerate every future
 * computational domain.
 *
 * For example, the following must remain extensible through the canonical
 * type/name/capability/resource systems:
 *
 *     classical
 *     quantum
 *     hybrid
 *     hdl
 *     hardware
 *     distributed
 *     parallel
 *     hpc
 *     ai
 *     ml
 *     data
 *     networking
 *     cryptography
 *     embedded
 *     edge
 *     cloud
 *     accelerator
 *     optical
 *     neuromorphic
 *     biological
 *     nano
 *     future computational paradigms
 *
 * A new semantic domain must not require this dispatcher to be rewritten
 * merely because a new domain name exists.
 *
 * ============================================================================
 * DEPENDENCY DIRECTION
 * ============================================================================
 *
 * Allowed:
 *
 *     lexer
 *       |
 *       v
 *     core
 *       |
 *       +--> names
 *       +--> attributes
 *       +--> visibility
 *       +--> modifiers
 *       |
 *       v
 *     types / expressions / generics
 *       |
 *       v
 *     declaration grammars
 *       |
 *       v
 *     frontend AST
 *       |
 *       v
 *     semantic analysis
 *       |
 *       v
 *     canonical semantic representations
 *
 * Forbidden:
 *
 *     declarations.g4 -> quantum::ir
 *     declarations.g4 -> QEC
 *     declarations.g4 -> ZQN
 *     declarations.g4 -> routing
 *     declarations.g4 -> scheduling
 *     declarations.g4 -> HAL
 *     declarations.g4 -> runtime
 *     declarations.g4 -> physical hardware
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Declaration syntax may introduce or reference quantum types and declarations.
 *
 * This dispatcher MUST NOT:
 *
 *     - enumerate gates;
 *     - enumerate physical qubits;
 *     - select physical qubits;
 *     - select QPUs;
 *     - encode QPU topology;
 *     - encode calibration;
 *     - perform routing;
 *     - perform scheduling;
 *     - perform QEC;
 *     - implement ZQN;
 *     - construct quantum::ir.
 *
 * Required direction:
 *
 *     declaration
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     quantum::ir
 *
 * `quantum::ir` remains the canonical quantum semantic boundary.
 *
 * ============================================================================
 * HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * Hardware-related declarations describe source-level intent.
 *
 * They MUST NOT turn:
 *
 *     capability
 *     requirement
 *     resource
 *     constraint
 *     preference
 *     hint
 *
 * into an implicit:
 *
 *     physical device
 *     physical address
 *     fixed topology
 *     fixed accelerator
 *     fixed CPU
 *     fixed GPU
 *     fixed FPGA
 *     fixed QPU
 *
 * Target realization remains downstream.
 *
 * ============================================================================
 * DECLARATION FAMILY MAP
 * ============================================================================
 *
 * Value:
 *
 *     constantDeclaration
 *     variableDeclaration
 *
 * Named/type:
 *
 *     typeDeclaration
 *     typeAliasDeclaration
 *
 * Aggregate:
 *
 *     structDeclaration
 *     recordDeclaration
 *     enumDeclaration
 *     unionDeclaration
 *
 * Object/contract:
 *
 *     classDeclaration
 *     interfaceDeclaration
 *     traitDeclaration
 *     implementationDeclaration
 *
 * Universal execution intent:
 *
 *     resourceDeclaration
 *     capability declaration adapter
 *     domainDeclaration
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar constructs no Rust AST.
 *
 * The frontend adapter maps the selected parse-tree branch into the existing
 * domain-neutral AST.
 *
 * Every declaration branch must preserve:
 *
 *     - declaration kind
 *     - complete source span
 *     - source ordering
 *     - identifier/name
 *     - attributes
 *     - visibility
 *     - modifiers
 *     - generic parameters
 *     - declaration members
 *     - child type expressions
 *     - child expressions
 *     - documentation where supported
 *
 * No grammar file may create a competing domain AST.
 *
 * In particular there must not be:
 *
 *     QuantumDeclarationAst
 *     HardwareDeclarationAst
 *     QuantumDeclarationIR
 *     DeclarationIR
 *
 * inside this grammar layer.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing establishes syntactic structure only.
 *
 * Later semantic analysis determines:
 *
 *     - declaration uniqueness;
 *     - name resolution;
 *     - scope;
 *     - type validity;
 *     - generic validity;
 *     - bounds;
 *     - trait/interface satisfaction;
 *     - implementation coherence;
 *     - ownership;
 *     - effects;
 *     - capabilities;
 *     - resource requirements;
 *     - domain validity;
 *     - portability;
 *     - target compatibility.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY SEPARATION
 * ============================================================================
 *
 * These concepts MUST remain distinct.
 *
 * Capability:
 *
 *     what an execution environment can do.
 *
 * Resource:
 *
 *     something computation can consume/use.
 *
 * Requirement:
 *
 *     something the program requires.
 *
 * Constraint:
 *
 *     a condition that must be satisfied.
 *
 * Preference:
 *
 *     an optimization preference.
 *
 * Hint:
 *
 *     advisory implementation information.
 *
 * Target:
 *
 *     an execution context/profile.
 *
 * This dispatcher must not collapse these concepts into physical placement.
 *
 * ============================================================================
 * DECLARATION DISPATCH
 * ============================================================================
 *
 * `declaration` is the only public dispatcher owned by this file.
 *
 * Concrete syntax is entirely delegated.
 *
 * ============================================================================
 */

parser grammar ZamaniDeclarations;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * DELEGATE IMPORTS
 * ============================================================================
 *
 * These grammar names correspond to the concrete declaration owners already
 * present in the repository.
 *
 * IMPORTANT:
 *
 *     The imports below intentionally use grammar names rather than filenames.
 *
 * For example:
 *
 *     classes.g4          -> ZamaniClasses
 *     records.g4          -> ZamaniDeclarationRecords
 *     aliases.g4          -> ZamaniDeclarationAliases
 *     types.g4            -> ZamaniDeclarationTypesParser
 *
 * ============================================================================
 */

import
    Constants,
    Variables,
    ZamaniDeclarationTypesParser,
    ZamaniDeclarationAliases,
    ZamaniDeclarationStructs,
    ZamaniDeclarationEnums,
    ZamaniDeclarationUnions,
    ZamaniDeclarationRecords,
    ZamaniClasses,
    Interfaces,
    Traits,
    ZamaniImplementations,
    Resources,
    CapabilityDeclarations,
    ZamaniDomains
;


/*
 * ============================================================================
 * AUTHORITATIVE DECLARATION ENTRY POINT
 * ============================================================================
 *
 * All source-level declaration families enter through this rule.
 *
 * No concrete declaration syntax is implemented here.
 *
 * ============================================================================
 */

declaration
    : valueDeclaration
    | typeDeclarationFamily
    | aggregateDeclaration
    | objectContractDeclaration
    | resourceCapabilityDeclaration
    | domainDeclaration
    ;


/*
 * ============================================================================
 * VALUE DECLARATIONS
 * ============================================================================
 *
 * Constants and variables are separate concrete declaration owners.
 *
 * ============================================================================
 */

valueDeclaration
    : constantDeclaration
    | variableDeclaration
    ;


/*
 * ============================================================================
 * TYPE DECLARATIONS
 * ============================================================================
 *
 * `typeDeclaration` remains the named-type declaration owned by types.g4.
 *
 * `typeAliasDeclaration` remains the alias declaration owned by aliases.g4.
 *
 * ============================================================================
 */

typeDeclarationFamily
    : typeDeclaration
    | typeAliasDeclaration
    ;


/*
 * ============================================================================
 * AGGREGATE DECLARATIONS
 * ============================================================================
 *
 * Aggregates are intentionally grouped here only for dispatch.
 *
 * Their concrete syntax remains in their individual grammars.
 *
 * ============================================================================
 */

aggregateDeclaration
    : structDeclaration
    | recordDeclaration
    | enumDeclaration
    | unionDeclaration
    ;


/*
 * ============================================================================
 * OBJECT / CONTRACT DECLARATIONS
 * ============================================================================
 *
 * Classes, interfaces, traits and implementations remain independently owned.
 *
 * ============================================================================
 */

objectContractDeclaration
    : classDeclaration
    | interfaceDeclaration
    | traitDeclaration
    | implementationDeclaration
    ;


/*
 * ============================================================================
 * RESOURCE / CAPABILITY DECLARATIONS
 * ============================================================================
 *
 * Resources are source-level resource abstractions.
 *
 * Capabilities are source-level capability contracts.
 *
 * Neither selects a physical machine resource.
 *
 * The capability adapter delegates to the canonical capability grammar.
 *
 * ============================================================================
 */

resourceCapabilityDeclaration
    : resourceDeclaration
    | declarationCapability
    ;


/*
 * ============================================================================
 * DOMAIN DECLARATIONS
 * ============================================================================
 *
 * Domains are open-world source-level semantic namespaces.
 *
 * The concrete domain grammar owns domain syntax.
 *
 * ============================================================================
 */

domainDeclaration
    : domainDeclaration
    ;
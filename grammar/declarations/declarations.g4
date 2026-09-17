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
 *     Production
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     No embedded Rust.
 *     No semantic predicates.
 *     No actions.
 *     No unsafe code.
 *     No filesystem/network/runtime access.
 *     No hardware discovery.
 *     No target-specific constants.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the SINGLE COMPOSITION OWNER for the declaration family
 * currently owned by grammar/declarations/.
 *
 * It answers:
 *
 *     "Which declaration families are legal at a declaration boundary?"
 *
 * It does NOT implement the concrete syntax of those declarations.
 *
 * Concrete syntax belongs to the dedicated delegate grammars:
 *
 *     constants.g4
 *     variables.g4
 *     types.g4
 *     aliases.g4
 *     structs.g4
 *     enums.g4
 *     unions.g4
 *     interfaces.g4
 *     traits.g4
 *     implementations.g4
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - declaration
 *     - declaration-family dispatch
 *     - value-declaration composition
 *     - type-declaration composition
 *     - declaration-family integration
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - token definitions
 *     - keywords
 *     - identifiers
 *     - names
 *     - paths
 *     - attributes
 *     - visibility
 *     - modifiers
 *     - expressions
 *     - type expressions
 *     - concrete declaration syntax
 *     - functions
 *     - modules
 *     - effects
 *     - resources
 *     - capabilities
 *     - hardware
 *     - quantum semantics
 *     - HDL semantics
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
 *     declarations.g4
 *          |
 *          +--> Constants
 *          +--> Variables
 *          +--> ZamaniDeclarationTypesParser
 *          +--> ZamaniDeclarationAliases
 *          +--> ZamaniDeclarationStructs
 *          +--> ZamaniDeclarationEnums
 *          +--> ZamaniDeclarationUnions
 *          +--> Interfaces
 *          +--> Traits
 *          +--> Implementations
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
 *          +--> capabilities
 *          +--> resources
 *          +--> ownership
 *          +--> control/data flow
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
 *     routing / scheduling / resilience
 *          |
 *          v
 *     target realization
 *          |
 *          v
 *     runtime
 *
 * ============================================================================
 * CRITICAL RULE
 * ============================================================================
 *
 * This file MUST NOT redefine a rule owned by a delegate grammar.
 *
 * In particular, this file MUST NOT contain another implementation of:
 *
 *     constantDeclaration
 *     variableDeclaration
 *     typeDeclaration
 *     typeAliasDeclaration
 *     structDeclaration
 *     enumDeclaration
 *     unionDeclaration
 *     interfaceDeclaration
 *     traitDeclaration
 *     implementationDeclaration
 *
 * Doing so would create competing grammar ownership.
 *
 * ============================================================================
 * ANTLR CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * The lexical authority is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Every imported declaration grammar MUST use:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * No declaration grammar may introduce ZamaniTokens as an alternative
 * production vocabulary.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Declarations describe source-level program semantics.
 *
 * This dispatcher therefore contains no:
 *
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
 *     MAX_PROGRAM_SIZE
 *
 * Nor does it contain physical identifiers or topology assumptions.
 *
 * Resource requirements, capabilities, constraints, preferences, placement,
 * scheduling, routing and target realization remain downstream concerns.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * Declaration cardinality is unbounded by language grammar.
 *
 * The grammar uses recursive/repeating constructs supplied by its delegates.
 *
 * There is no source-language limit on:
 *
 *     declarations
 *     fields
 *     variants
 *     generic parameters
 *     trait members
 *     interface members
 *     implementation members
 *     type nesting
 *
 * Practical resource limits are implementation/resource-policy concerns and
 * MUST NOT become language constants.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This file contains:
 *
 *     - no actions;
 *     - no predicates;
 *     - no random behavior;
 *     - no time-dependent behavior;
 *     - no runtime calls;
 *     - no mutable global state.
 *
 * For a fixed lexer token stream and grammar version, declaration dispatch is
 * deterministic.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar constructs no Rust AST.
 *
 * The frontend adapter maps the selected parse-tree context into the canonical
 * domain-neutral AST.
 *
 * Every declaration must preserve:
 *
 *     - declaration kind;
 *     - source span;
 *     - source order;
 *     - name;
 *     - modifiers;
 *     - attributes;
 *     - generic parameters;
 *     - declaration members;
 *     - child types;
 *     - child expressions.
 *
 * No declaration grammar may create a second AST for a particular domain.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing establishes syntactic structure only.
 *
 * Semantic analysis subsequently determines:
 *
 *     - duplicate declarations;
 *     - name resolution;
 *     - type validity;
 *     - generic validity;
 *     - trait/interface satisfaction;
 *     - implementation coherence;
 *     - ownership;
 *     - effects;
 *     - resource requirements;
 *     - capability requirements;
 *     - domain validity;
 *     - portability;
 *     - target compatibility.
 *
 * ============================================================================
 * QUANTUM CONTRACT
 * ============================================================================
 *
 * Declaration syntax may introduce or reference quantum types and declarations.
 *
 * This file MUST NOT:
 *
 *     - enumerate physical qubits;
 *     - select a QPU;
 *     - select a topology;
 *     - select a gate implementation;
 *     - select calibration data;
 *     - perform routing;
 *     - perform scheduling;
 *     - perform QEC;
 *     - implement ZQN;
 *     - construct quantum::ir.
 *
 * The required direction is:
 *
 *     declaration syntax
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
 * ============================================================================
 * HDL / HARDWARE CONTRACT
 * ============================================================================
 *
 * Hardware/HDL declarations must remain target-independent at this layer.
 *
 * The declaration grammar must not silently turn:
 *
 *     capability
 *     requirement
 *     resource
 *     constraint
 *
 * into:
 *
 *     physical device
 *     physical address
 *     fixed topology
 *     fixed accelerator
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
 *     core syntax
 *       |
 *       v
 *     types / expressions
 *       |
 *       v
 *     declarations
 *       |
 *       v
 *     AST
 *       |
 *       v
 *     semantic analysis
 *
 * Forbidden:
 *
 *     declarations.g4 -> quantum::ir
 *     declarations.g4 -> QEC
 *     declarations.g4 -> ZQN
 *     declarations.g4 -> scheduling
 *     declarations.g4 -> routing
 *     declarations.g4 -> HAL
 *     declarations.g4 -> runtime
 *
 * ============================================================================
 * LEGACY GRAMMAR MIGRATION
 * ============================================================================
 *
 * The repository still contains declaration implementations in:
 *
 *     grammar/Zamani.g4
 *     grammar/antlr/Core.g4
 *     grammar/antlr/ZamaniParser.g4
 *     grammar/statements/declarations.g4
 *
 * Those are migration/conformance surfaces and must not remain competing
 * production owners.
 *
 * The final production parser must have one declaration dispatch path.
 *
 * The intended final relationship is:
 *
 *     canonical compilation unit
 *              |
 *              v
 *     Zamani declaration dispatcher
 *              |
 *              v
 *     this grammar
 *              |
 *              v
 *     declaration delegates
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
 * These are the concrete declaration owners already present in the repository.
 *
 * Each delegate is independently completable and independently testable.
 */
import
    Constants,
    Variables,
    ZamaniDeclarationTypesParser,
    ZamaniDeclarationAliases,
    ZamaniDeclarationStructs,
    ZamaniDeclarationEnums,
    ZamaniDeclarationUnions,
    Interfaces,
    Traits,
    Implementations;


/*
 * ============================================================================
 * AUTHORITATIVE DECLARATION ENTRY POINT
 * ============================================================================
 *
 * `declaration` is the only public declaration-dispatch rule owned here.
 *
 * No concrete declaration syntax is repeated.
 */
declaration
    : valueDeclaration
    | typeDeclarationFamily
    | traitDeclaration
    | implementationDeclaration
    ;


/*
 * ============================================================================
 * VALUE DECLARATIONS
 * ============================================================================
 *
 * Concrete syntax is owned by Constants and Variables.
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
 * Concrete syntax is supplied by the imported delegate grammars.
 *
 * The type system itself remains owned by grammar/types/.
 */
typeDeclarationFamily
    : typeDeclaration
    | typeAliasDeclaration
    | structDeclaration
    | enumDeclaration
    | unionDeclaration
    | interfaceDeclaration
    ;


/*
 * ============================================================================
 * COMPLETION CONTRACT
 * ============================================================================
 *
 * This file is COMPLETE when:
 *
 * [x] There is exactly one `declaration` dispatcher in this grammar family.
 * [x] Concrete declaration syntax is delegated.
 * [x] Constants are delegated to Constants.
 * [x] Variables are delegated to Variables.
 * [x] Named types are delegated to ZamaniDeclarationTypesParser.
 * [x] Aliases are delegated to ZamaniDeclarationAliases.
 * [x] Structs are delegated to ZamaniDeclarationStructs.
 * [x] Enums are delegated to ZamaniDeclarationEnums.
 * [x] Unions are delegated to ZamaniDeclarationUnions.
 * [x] Interfaces are delegated to Interfaces.
 * [x] Traits are delegated to Traits.
 * [x] Implementations are delegated to Implementations.
 * [x] No type-expression implementation is duplicated here.
 * [x] No identifier implementation is duplicated here.
 * [x] No expression implementation is duplicated here.
 * [x] No annotation implementation is duplicated here.
 * [x] No lexer rules exist here.
 * [x] No semantic actions exist here.
 * [x] No hardware limits exist here.
 * [x] No quantum IR exists here.
 * [x] No QEC/ZQN/routing/scheduling logic exists here.
 * [x] No Rust code is embedded here.
 * [x] No unsafe Rust is required downstream by this grammar contract.
 *
 * ============================================================================
 */
/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/declarations/declarations.g4
 *
 * Role:
 *     AUTHORITATIVE DECLARATION COMPOSITION GRAMMAR.
 *
 * Status:
 *     Production architecture.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar.
 *
 * Rust implementation baseline:
 *     Rust 1.97 / Rust 1.97.1.
 *
 * Safety:
 *     This grammar contains:
 *
 *       - no embedded Rust actions;
 *       - no semantic predicates;
 *       - no unsafe code;
 *       - no filesystem access;
 *       - no networking;
 *       - no process execution;
 *       - no hardware discovery;
 *       - no runtime execution;
 *       - no mutable global parser state;
 *       - no target-specific implementation;
 *       - no machine-size constants.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the SINGLE COMPOSITION OWNER for Zamani declarations.
 *
 * It answers one architectural question:
 *
 *     "Which declaration families are legal at a declaration boundary?"
 *
 * It DOES NOT implement the concrete syntax of those declaration families.
 *
 * Concrete declaration syntax belongs to dedicated grammar modules:
 *
 *     constants.g4
 *     variables.g4
 *     types.g4
 *     aliases.g4
 *     structs.g4
 *     enums.g4
 *     unions.g4
 *     interfaces.g4
 *
 * This separation is mandatory.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - declaration;
 *     - declaration composition;
 *     - declaration-family dispatch;
 *     - the authoritative declaration-category boundary;
 *     - declaration ordering at a source/module boundary.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexer rules;
 *     - keywords;
 *     - identifiers;
 *     - qualified names;
 *     - attributes;
 *     - visibility syntax;
 *     - modifiers;
 *     - generic parameter syntax;
 *     - type-expression syntax;
 *     - expression syntax;
 *     - constant syntax;
 *     - variable syntax;
 *     - type-alias syntax;
 *     - struct syntax;
 *     - enum syntax;
 *     - union syntax;
 *     - interface syntax;
 *     - function syntax;
 *     - module syntax;
 *     - effect syntax;
 *     - capability syntax;
 *     - resource semantics;
 *     - quantum semantics;
 *     - HDL semantics;
 *     - hardware semantics;
 *     - distributed semantics;
 *     - AI semantics;
 *     - AST construction;
 *     - semantic analysis;
 *     - canonical IR;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN;
 *     - optimization;
 *     - routing;
 *     - scheduling;
 *     - hardware discovery;
 *     - calibration;
 *     - backend selection;
 *     - runtime execution.
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
 *     canonical parser
 *          |
 *          +--> declarations.g4       <-- THIS FILE
 *          |        |
 *          |        +--> constants.g4
 *          |        +--> variables.g4
 *          |        +--> types.g4
 *          |        +--> aliases.g4
 *          |        +--> structs.g4
 *          |        +--> enums.g4
 *          |        +--> unions.g4
 *          |        +--> interfaces.g4
 *          |
 *          v
 *     frontend AST
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
 *          +--> control/data-flow
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
 *     optimization
 *          |
 *          v
 *     routing / scheduling / lowering
 *          |
 *          v
 *     target realization
 *          |
 *          v
 *     runtime
 *
 * ============================================================================
 * CRITICAL ARCHITECTURAL RULE
 * ============================================================================
 *
 * This file MUST NOT duplicate rules owned by its delegate grammars.
 *
 * In particular, this file MUST NOT define another:
 *
 *     constantDeclaration
 *     variableDeclaration
 *     typeDeclaration
 *     typeAliasDeclaration
 *     structDeclaration
 *     enumDeclaration
 *     unionDeclaration
 *     interfaceDeclaration
 *
 * Doing so would create competing grammar ownership and eventually divergent
 * AST/semantic behavior.
 *
 * ============================================================================
 * ANTLR COMPOSITION MODEL
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * The canonical lexical vocabulary remains:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * The canonical parser composition layer imports this declaration grammar.
 *
 * Declaration delegates are imported here so that this file is the single
 * declaration dispatcher rather than a second implementation of declarations.
 *
 * ============================================================================
 * IMPORT CONTRACT
 * ============================================================================
 *
 * The imported grammars provide the concrete declaration rules.
 *
 * The imports below correspond to the current repository declaration modules:
 *
 *     Constants
 *     Variables
 *     ZamaniDeclarationTypesParser
 *     ZamaniDeclarationAliases
 *     ZamaniDeclarationStructs
 *     ZamaniDeclarationEnums
 *     ZamaniDeclarationUnions
 *     Interfaces
 *
 * IMPORTANT:
 *
 * The repository currently contains migration inconsistencies in some delegate
 * grammars. In particular, unions.g4 currently refers to an older token
 * vocabulary (`ZamaniTokens`) while the canonical lexer is `ZamaniLexer`.
 *
 * That delegate MUST be migrated to the canonical lexer vocabulary before the
 * complete declaration grammar can be generated as one production parser.
 *
 * This file deliberately does NOT reproduce the union grammar as a workaround.
 *
 * ============================================================================
 * DEPENDENCY DIRECTION
 * ============================================================================
 *
 *     declarations.g4
 *          |
 *          +--> declaration delegates
 *          |
 *          +--> canonical parser dependencies
 *
 * It MUST NOT become:
 *
 *     declarations.g4
 *          |
 *          v
 *     AST
 *          |
 *          v
 *     declarations.g4
 *
 * It MUST NOT depend on:
 *
 *     quantum::ir
 *     QEC
 *     ZQN
 *     resilience
 *     optimization
 *     routing
 *     scheduling
 *     hardware discovery
 *     calibration
 *     runtime
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Declaration composition is entirely target-independent.
 *
 * This file therefore contains NO syntax for:
 *
 *     cpuDeclaration
 *     gpuDeclaration
 *     fpgaDeclaration
 *     asicDeclaration
 *     qpuDeclaration
 *     deviceDeclaration
 *     clusterDeclaration
 *     machineDeclaration
 *
 * merely because a target exists.
 *
 * A declaration describes source-level program semantics.
 *
 * Target realization belongs downstream.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This file contains no language-level limits for:
 *
 *     - declarations;
 *     - declaration families;
 *     - generic parameters;
 *     - fields;
 *     - variants;
 *     - interfaces;
 *     - types;
 *     - aliases;
 *     - devices;
 *     - qubits;
 *     - cores;
 *     - threads;
 *     - GPUs;
 *     - FPGAs;
 *     - nodes;
 *     - memory;
 *     - accelerators;
 *     - tensor dimensions;
 *     - program size.
 *
 * Repetition is expressed through normal ANTLR grammar composition.
 *
 * Practical parser/compiler limits, if required for resource protection, must
 * be explicit compiler/resource policy rather than source-language constants.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no actions;
 *     - no semantic predicates;
 *     - no random behavior;
 *     - no time-dependent behavior;
 *     - no filesystem operations;
 *     - no network operations;
 *     - no hardware inspection;
 *     - no runtime calls;
 *     - no mutable global state.
 *
 * For a fixed token stream and fixed grammar/version, declaration parsing is
 * deterministic.
 *
 * ============================================================================
 * ERROR BOUNDARY
 * ============================================================================
 *
 * THIS FILE reports only structural declaration-dispatch errors.
 *
 * Examples:
 *
 *     an otherwise valid source position contains a token sequence which does
 *     not belong to any declaration family.
 *
 * It does NOT diagnose semantic conditions such as:
 *
 *     duplicate names;
 *     invalid types;
 *     unsatisfied constraints;
 *     unavailable capabilities;
 *     impossible resources;
 *     illegal quantum operations;
 *     invalid hardware mappings;
 *     invalid scheduling;
 *     invalid routing.
 *
 * Those belong to downstream semantic/compiler layers.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar creates no Rust AST values.
 *
 * The frontend parser adapter maps the selected declaration parse context to
 * the repository's canonical declaration AST nodes.
 *
 * Examples include:
 *
 *     ConstantDeclaration
 *     VariableDeclaration
 *     TypeDeclaration
 *     TypeAlias
 *     StructDeclaration
 *     EnumDeclaration
 *     UnionDeclaration
 *     InterfaceDeclaration
 *
 * The declaration dispatcher itself must preserve:
 *
 *     - declaration kind;
 *     - source order;
 *     - source span;
 *     - child parse structure.
 *
 * It must not attach machine-specific information.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This file does NOT construct IR.
 *
 * The intended direction is:
 *
 *     declaration parse tree
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> classical IR
 *          +--> quantum::ir
 *          +--> HDL/hardware IR
 *          +--> distributed representation
 *          +--> accelerator representation
 *          +--> future-domain representation
 *
 * In particular:
 *
 *     declarations.g4 -> quantum::ir
 *
 * is FORBIDDEN.
 *
 * Quantum declarations are parsed here only as source syntax through their
 * concrete declaration family. Their semantic lowering remains downstream.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum-specific declarations may eventually be added to the declaration
 * family when their dedicated grammar modules become authoritative.
 *
 * They must follow this same boundary:
 *
 *     quantum declaration syntax
 *          |
 *          v
 *     quantum semantic analysis
 *          |
 *          v
 *     quantum::ir
 *
 * This file must never encode:
 *
 *     MAX_QUBITS
 *     physical qubit indices
 *     fixed gate inventories
 *     topology
 *     backend identifiers
 *     calibration values
 *     QEC algorithms
 *     noise channels
 *     routing decisions
 *     scheduling decisions.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Hardware and HDL declarations must enter through dedicated grammar families.
 *
 * This dispatcher must not distinguish targets such as:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *
 * unless those distinctions are actual source-language semantic constructs
 * with explicit language specifications.
 *
 * Physical implementation belongs downstream.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Resource requirements, capabilities, constraints, preferences, and hints are
 * separate semantic concepts.
 *
 * This dispatcher merely admits declaration families capable of carrying those
 * concepts.
 *
 * It must never translate:
 *
 *     requires quantum
 *
 * directly into:
 *
 *     use device X
 *     use N qubits
 *     use topology Y
 *
 * That transformation belongs to semantic analysis, compilation, routing,
 * scheduling, resource management, or execution.
 *
 * ============================================================================
 * NO CIRCULAR OWNERSHIP
 * ============================================================================
 *
 * Delegate declaration grammars MUST NOT import this composition grammar.
 *
 * The forbidden pattern is:
 *
 *     declarations.g4
 *          |
 *          v
 *     constants.g4
 *          |
 *          v
 *     declarations.g4
 *
 * The declaration dispatcher is the top of the declaration grammar family.
 *
 * ============================================================================
 * SINGLE SOURCE OF TRUTH
 * ============================================================================
 *
 * There must ultimately be exactly one active declaration dispatcher.
 *
 * The legacy declaration dispatcher embedded in:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * currently contains its own declaration alternatives and concrete declaration
 * implementations.
 *
 * During migration it may remain as a compatibility/reference grammar.
 *
 * However, the production parser generation path MUST eventually contain only
 * one authoritative declaration dispatcher.
 *
 * The migration target is:
 *
 *     ZamaniParser
 *          |
 *          +--> ZamaniDeclarations
 *                  |
 *                  +--> declaration delegates
 *
 * rather than two competing declaration implementations.
 *
 * ============================================================================
 * DECLARATION CATEGORIES
 * ============================================================================
 *
 * The canonical declaration family is intentionally separated into semantic
 * categories.
 *
 * VALUE DECLARATIONS
 *
 *     constantDeclaration
 *     variableDeclaration
 *
 * TYPE DECLARATIONS
 *
 *     typeDeclaration
 *     typeAliasDeclaration
 *     structDeclaration
 *     enumDeclaration
 *     unionDeclaration
 *     interfaceDeclaration
 *
 * Future categories may include:
 *
 *     classDeclaration
 *     traitDeclaration
 *     implementationDeclaration
 *     functionDeclaration
 *     moduleDeclaration
 *     capabilityDeclaration
 *     resourceDeclaration
 *     hardwareDeclaration
 *     quantumDeclaration
 *     dialectDeclaration
 *
 * Such categories should be added by importing their dedicated grammar owner,
 * not by implementing their syntax directly here.
 *
 * ============================================================================
 * DECLARATION RULE
 * ============================================================================
 *
 * `declaration` is the sole public composition rule exported by this grammar.
 *
 * Every concrete alternative delegates to an independently owned grammar.
 *
 * No concrete declaration syntax is repeated here.
 * ============================================================================
 */

parser grammar ZamaniDeclarations;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * DECLARATION DELEGATES
 * ============================================================================
 *
 * These imports are the concrete syntax owners.
 *
 * The grammar names correspond to the current repository modules.
 *
 * IMPORTANT:
 *
 * The delegates must all ultimately consume the same canonical lexer
 * vocabulary and compatible shared parser rules.
 */
import
    Constants,
    Variables,
    ZamaniDeclarationTypesParser,
    ZamaniDeclarationAliases,
    ZamaniDeclarationStructs,
    ZamaniDeclarationEnums,
    ZamaniDeclarationUnions,
    Interfaces;


/*
 * ============================================================================
 * AUTHORITATIVE DECLARATION DISPATCHER
 * ============================================================================
 *
 * This is deliberately a flat semantic-family dispatcher.
 *
 * It must not contain:
 *
 *     target-specific alternatives;
 *     machine-specific alternatives;
 *     duplicated concrete productions.
 *
 * Each alternative is a rule owned by a delegate grammar.
 */
declaration
    : valueDeclaration
    | typeDeclarationFamily
    ;


/*
 * ============================================================================
 * VALUE DECLARATIONS
 * ============================================================================
 *
 * These rules merely compose the concrete value-declaration delegates.
 *
 * `constantDeclaration` is owned by Constants.
 * `variableDeclaration` is owned by Variables.
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
 * `typeDeclarationFamily` is the single type-declaration composition boundary.
 *
 * It deliberately does not redefine any concrete type declaration.
 */
typeDeclarationFamily
    : namedTypeDeclaration
    | typeAliasDeclaration
    | structDeclaration
    | enumDeclaration
    | unionDeclaration
    | interfaceDeclaration
    ;


/*
 * ============================================================================
 * NAMED TYPE DECLARATION
 * ============================================================================
 *
 * The current declarations/types.g4 owns `typeDeclaration`.
 *
 * This wrapper gives the composition grammar a stable semantic category while
 * preserving that delegate's ownership.
 *
 * It does not duplicate the rule.
 */
namedTypeDeclaration
    : typeDeclaration
    ;


/*
 * ============================================================================
 * FUTURE DECLARATION EXTENSION BOUNDARY
 * ============================================================================
 *
 * New declaration families MUST be introduced through this composition layer.
 *
 * The required pattern is:
 *
 *     new-domain.g4
 *          |
 *          +--> newDomainDeclaration
 *          |
 *          v
 *     declarations.g4
 *          |
 *          +--> futureDeclarationFamily
 *
 * The new grammar must NOT copy or modify unrelated declaration syntax.
 *
 * This permits Zamani to expand into:
 *
 *     classical computing
 *     quantum computing
 *     hybrid computing
 *     HDL
 *     hardware/software co-design
 *     distributed computing
 *     AI/ML
 *     accelerators
 *     networking
 *     cryptography
 *     scientific computing
 *     embedded computing
 *     future computational paradigms
 *
 * without destabilizing the existing declaration families.
 */


/*
 * ============================================================================
 * FUTURE DECLARATION CATEGORY
 * ============================================================================
 *
 * This rule is intentionally NOT populated with speculative alternatives.
 *
 * Adding an alternative before its grammar owner exists would create an
 * incomplete parser contract.
 *
 * When a new declaration family is production-ready, add exactly one delegate
 * import and exactly one corresponding semantic-family alternative here.
 */


/*
 * ============================================================================
 * COMPLETION / INTEGRATION CONTRACT
 * ============================================================================
 *
 * This file is COMPLETE when all of the following are true:
 *
 * [ ] `declaration` is the sole authoritative declaration dispatcher.
 *
 * [ ] No concrete declaration syntax is duplicated here.
 *
 * [ ] Constants owns `constantDeclaration`.
 *
 * [ ] Variables owns `variableDeclaration`.
 *
 * [ ] Types owns `typeDeclaration`.
 *
 * [ ] Aliases owns `typeAliasDeclaration`.
 *
 * [ ] Structs owns `structDeclaration`.
 *
 * [ ] Enums owns `enumDeclaration`.
 *
 * [ ] Unions owns `unionDeclaration`.
 *
 * [ ] Interfaces owns `interfaceDeclaration`.
 *
 * [ ] All delegates consume the canonical ZamaniLexer vocabulary.
 *
 * [ ] No delegate imports ZamaniDeclarations, preventing circular grammar
 *     ownership.
 *
 * [ ] The canonical parser imports ZamaniDeclarations exactly once.
 *
 * [ ] The legacy declaration implementation in ZamaniParser.g4 is removed
 *     from the production parser-generation path.
 *
 * [ ] No duplicate declaration rule remains in the active parser.
 *
 * [ ] No duplicate type-expression grammar is introduced here.
 *
 * [ ] No duplicate identifier grammar is introduced here.
 *
 * [ ] No duplicate generic grammar is introduced here.
 *
 * [ ] No duplicate attribute grammar is introduced here.
 *
 * [ ] No duplicate expression grammar is introduced here.
 *
 * [ ] No semantic analysis occurs in this grammar.
 *
 * [ ] No AST constructors occur in this grammar.
 *
 * [ ] No IR construction occurs in this grammar.
 *
 * [ ] No quantum::ir dependency exists.
 *
 * [ ] No QEC dependency exists.
 *
 * [ ] No ZQN dependency exists.
 *
 * [ ] No scheduling dependency exists.
 *
 * [ ] No routing dependency exists.
 *
 * [ ] No hardware discovery exists.
 *
 * [ ] No runtime execution exists.
 *
 * [ ] No fixed machine/resource limits exist.
 *
 * [ ] No MAX_QUBITS/MAX_CORES/MAX_THREADS/etc. constants exist.
 *
 * [ ] Positive declaration tests pass.
 *
 * [ ] Negative declaration tests pass.
 *
 * [ ] Boundary/scalability tests pass.
 *
 * [ ] Cross-domain declaration tests pass.
 *
 * [ ] Deterministic parsing tests pass.
 *
 * [ ] Parser round-trip tests pass where a canonical printer exists.
 *
 * [ ] Rust 1.97 / 1.97.1 integration passes.
 *
 * [ ] Rust integration contains no unsafe code.
 *
 * ============================================================================
 * END
 * ============================================================================
 */
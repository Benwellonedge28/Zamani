/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/interoperability/data-layout.g4
 *
 * GRAMMAR
 * -------
 * InteroperabilityDataLayout
 *
 * STATUS
 * ------
 * CANONICAL / PRODUCTION DATA-LAYOUT CONTRACT
 *
 * LANGUAGE
 * --------
 * Zamani
 *
 * IMPLEMENTATION BASELINE
 * -----------------------
 * Rust 1.97 or later
 * Rust 2021
 * Safe Rust only
 * No unsafe Rust
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * This file owns the SOURCE-LEVEL SYNTAX for describing data-layout
 * interoperability intent at a foreign/ABI boundary.
 *
 * A data-layout contract describes how a semantic Zamani value or type is
 * expected to cross an interoperability boundary.
 *
 * It may express:
 *
 *     - representation identity;
 *     - scalar representation;
 *     - aggregate representation;
 *     - field-order intent;
 *     - field-layout intent;
 *     - size intent;
 *     - alignment intent;
 *     - stride intent;
 *     - offset intent;
 *     - byte-order intent;
 *     - bit-order intent;
 *     - address-space intent;
 *     - pointer/reference representation intent;
 *     - integer/float representation intent;
 *     - vector representation intent;
 *     - opaque representation;
 *     - transparent representation;
 *     - tagged/untagged representation intent;
 *     - discriminant representation intent;
 *     - nullable representation intent;
 *     - calling-boundary representation metadata;
 *     - layout compatibility constraints;
 *     - layout requirements;
 *     - layout capabilities;
 *     - target-independent layout properties;
 *     - symbolic layout expressions;
 *     - extensible vendor/domain metadata.
 *
 * This grammar describes DECLARATIVE INTENT.
 *
 * It does NOT calculate, allocate, or materialize a machine layout.
 *
 * ============================================================================
 * OWNS
 * ============================================================================
 *
 * This file owns:
 *
 *     dataLayoutDeclaration
 *     dataLayoutContract
 *     dataLayoutBody
 *     dataLayoutMember
 *     dataLayoutProfileReference
 *     dataLayoutTypeContract
 *     dataLayoutRepresentation
 *     dataLayoutFieldContract
 *     dataLayoutProperty
 *     dataLayoutRequirement
 *     dataLayoutCapability
 *     dataLayoutConstraint
 *     dataLayoutExpression
 *     dataLayoutReference
 *     dataLayoutOpaque
 *     dataLayoutTransparent
 *     dataLayoutCompatibility
 *
 * It owns the syntactic organization of data-layout contracts.
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     - lexer rules;
 *     - identifier spelling;
 *     - qualified-name syntax;
 *     - general expressions;
 *     - general types;
 *     - ABI identity;
 *     - ABI contracts;
 *     - FFI declarations;
 *     - foreign function declarations;
 *     - foreign type declarations;
 *     - calling conventions;
 *     - linkage;
 *     - symbol resolution;
 *     - target selection;
 *     - target architecture;
 *     - machine instruction selection;
 *     - register allocation;
 *     - stack allocation;
 *     - object-file generation;
 *     - linker implementation;
 *     - loader implementation;
 *     - physical memory allocation;
 *     - hardware discovery;
 *     - hardware placement;
 *     - quantum routing;
 *     - quantum scheduling;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime execution.
 *
 * ============================================================================
 * CRITICAL ARCHITECTURAL RULE
 * ============================================================================
 *
 * DATA LAYOUT != PHYSICAL MACHINE LAYOUT.
 *
 * A source-level data-layout contract describes an interoperability
 * representation requirement.
 *
 * The compiler may later realize that contract differently for:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     QPU
 *     simulator
 *     embedded target
 *     distributed target
 *     future target
 *
 * without changing the source program.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * This grammar MUST preserve:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Therefore this file MUST NOT define universal constants for:
 *
 *     pointer width
 *     address width
 *     register width
 *     word width
 *     alignment maximum
 *     field count
 *     aggregate size
 *     structure size
 *     vector width
 *     tensor rank
 *     address-space count
 *     memory-bank count
 *     device count
 *     CPU count
 *     GPU count
 *     FPGA count
 *     node count
 *     qubit count
 *
 * No universal numeric ceiling is defined here.
 *
 * ============================================================================
 * OPEN-WORLD DESIGN
 * ============================================================================
 *
 * Layout identities and property values are symbolic wherever possible.
 *
 * The grammar MUST NOT enumerate:
 *
 *     x86
 *     arm
 *     riscv
 *     wasm
 *     C
 *     Rust
 *     SysV
 *     Win64
 *     AAPCS
 *     vendor-specific layouts
 *
 * as the universal set of supported layouts.
 *
 * Such identities may be supplied as symbolic names or strings and resolved
 * by semantic analysis against registered interoperability descriptions.
 *
 * A future layout convention therefore does not require modifying this file.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * General type syntax remains owned by:
 *
 *     grammar/types/types.g4
 *
 * General names remain owned by:
 *
 *     grammar/core/names.g4
 *
 * General attributes remain owned by:
 *
 *     grammar/core/attributes.g4
 *
 * ABI syntax remains owned by:
 *
 *     grammar/interoperability/abi.g4
 *
 * FFI syntax remains owned by:
 *
 *     grammar/interoperability/ffi.g4
 *
 * Foreign type syntax remains owned by:
 *
 *     grammar/interoperability/foreign-types.g4
 *
 * Calling-convention syntax remains owned by:
 *
 *     grammar/interoperability/calling-conventions.g4
 *
 * Serialization/deserialization remain owned by their existing grammars.
 *
 * This file MUST NOT duplicate those language areas.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     canonical parser
 *          |
 *          v
 *     data-layout syntax
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     structural validation
 *          |
 *          v
 *     semantic interoperability analysis
 *          |
 *     +----+----------------------+-------------------+
 *     |                           |                   |
 *     v                           v                   v
 *   types                        ABI                 FFI
 *     |                           |                   |
 *     +---------------------------+-------------------+
 *                                 |
 *                                 v
 *                      canonical semantic model
 *                                 |
 *                   target-independent representation
 *                                 |
 *                        target realization
 *
 * Data-layout syntax MUST NOT bypass the semantic model.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar creates parser contexts only.
 *
 * The frontend AST owns the actual representation.
 *
 * Conceptual mapping:
 *
 *     dataLayoutDeclaration
 *         ->
 *     DataLayoutContractNode
 *         ->
 *     semantic data-layout contract
 *
 * The AST MUST preserve:
 *
 *     - source span;
 *     - symbolic layout identity;
 *     - optional associated type;
 *     - representation clauses;
 *     - property ordering where semantically observable;
 *     - symbolic expressions;
 *     - requirements;
 *     - capabilities;
 *     - constraints;
 *     - compatibility metadata;
 *     - attributes;
 *     - provenance.
 *
 * The AST MUST NOT contain:
 *
 *     - physical addresses;
 *     - selected registers;
 *     - selected stack slots;
 *     - selected memory banks;
 *     - physical device identifiers;
 *     - physical CPU identifiers;
 *     - physical GPU identifiers;
 *     - physical qubit identifiers;
 *     - backend allocation decisions.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing establishes syntax only.
 *
 * Semantic analysis determines:
 *
 *     - whether the referenced type exists;
 *     - whether the layout identity is known or permitted;
 *     - whether properties are compatible;
 *     - whether size/alignment expressions are meaningful;
 *     - whether symbolic dimensions can be resolved;
 *     - whether representation requirements are satisfiable;
 *     - whether ABI and FFI requirements agree;
 *     - whether ownership and lifetime constraints remain valid;
 *     - whether the target can realize the requested contract;
 *     - whether a conversion is required;
 *     - whether the conversion preserves program meaning.
 *
 * A target's inability to realize a layout MUST NOT become a parser failure.
 *
 * It is a semantic/compilation/realization diagnostic.
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * Data-layout contracts may refer to an existing Zamani type through the
 * canonical Type grammar.
 *
 * This grammar MUST NOT introduce:
 *
 *     DataLayoutType
 *     UniversalLayoutType
 *     AbiType2
 *     ForeignType2
 *
 * as competing type systems.
 *
 * ============================================================================
 * EXPRESSION CONTRACT
 * ============================================================================
 *
 * Layout values may be:
 *
 *     symbolic;
 *     literal;
 *     named;
 *     qualified;
 *     expression-derived;
 *     type-derived;
 *     capability-derived;
 *     resource-derived.
 *
 * The grammar therefore avoids imposing a closed numeric universe.
 *
 * Examples of semantic intent include:
 *
 *     size = sizeof(T);
 *     alignment = required_alignment;
 *     stride = element_stride;
 *     offset = field_offset;
 *
 * The exact meaning of these expressions belongs to semantic analysis.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Parsing this grammar produces no runtime effects.
 *
 * It MUST NOT:
 *
 *     - inspect hardware;
 *     - inspect host ABI;
 *     - load libraries;
 *     - resolve symbols;
 *     - allocate memory;
 *     - execute foreign code;
 *     - access the filesystem;
 *     - access the network;
 *     - probe a target.
 *
 * If a data-layout declaration eventually requires a foreign/native operation,
 * that effect is represented downstream through the existing effect system.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Data-layout requirements may refer symbolically to capabilities.
 *
 * Examples:
 *
 *     requires capability("foreign.layout");
 *     requires capability("representation.transform");
 *
 * Capability existence and satisfaction are semantic concerns.
 *
 * This grammar does not enumerate the world's capabilities.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Layout contracts may contain symbolic resource requirements where required
 * by interoperability semantics.
 *
 * Resource resolution belongs to:
 *
 *     grammar/resources/
 *
 * and downstream semantic analysis.
 *
 * This grammar MUST NOT encode fixed resource quantities as language-wide
 * limits.
 *
 * ============================================================================
 * CONTRACT CONTRACT
 * ============================================================================
 *
 * A data-layout declaration may participate in existing:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * semantics.
 *
 * This grammar does not redefine the general contract language.
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Security and deployment policies may constrain layout interoperability.
 *
 * Examples:
 *
 *     whether native representation is permitted;
 *     whether transparent representation is permitted;
 *     whether conversion is permitted;
 *     whether foreign memory is permitted.
 *
 * Policy ownership remains outside this file.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Data-layout information may participate in provenance.
 *
 * The semantic model should be capable of recording:
 *
 *     source declaration
 *     referenced layout profile
 *     transformations
 *     conversions
 *     verification
 *     target realization
 *
 * Provenance storage and audit semantics belong to the existing provenance
 * architecture.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates NO IR.
 *
 * Data-layout information is attached to the existing semantic interoperability
 * model and is lowered by the appropriate compiler/backend stages.
 *
 * This file MUST NOT introduce:
 *
 *     DataLayoutIR
 *     AbiLayoutIR
 *     ForeignLayoutIR
 *     HardwareLayoutIR
 *
 * as competing canonical IRs.
 *
 * If the representation affects quantum computation, the downstream quantum
 * semantic path remains:
 *
 *     semantic model
 *         ->
 *     quantum::ir
 *
 * This grammar itself remains domain-neutral.
 *
 * ============================================================================
 * TARGET LOWERING CONTRACT
 * ============================================================================
 *
 * Downstream compilation may derive:
 *
 *     concrete size;
 *     concrete alignment;
 *     concrete offsets;
 *     concrete strides;
 *     concrete address spaces;
 *     concrete calling-boundary representation;
 *     concrete marshaling;
 *     concrete conversion routines.
 *
 * Those values are target realization details.
 *
 * They MUST NOT be elevated into universal source-language constants.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/core/names.g4
 *     grammar/core/attributes.g4
 *     grammar/expressions/expressions.g4
 *     grammar/types/types.g4
 *
 * OPTIONAL SEMANTIC CONSUMERS:
 *
 *     grammar/interoperability/abi.g4
 *     grammar/interoperability/ffi.g4
 *     grammar/interoperability/foreign-types.g4
 *     grammar/interoperability/calling-conventions.g4
 *     grammar/interoperability/serialization.g4
 *     grammar/interoperability/deserialization.g4
 *
 * RESOURCE SEMANTIC OWNER:
 *
 *     grammar/resources/
 *
 * EFFECT SEMANTIC OWNER:
 *
 *     grammar/effects/
 *
 * CONTRACT SEMANTIC OWNER:
 *
 *     grammar/validation/
 *
 * POLICY SEMANTIC OWNER:
 *
 *     grammar/policies/
 *
 * AST OWNER:
 *
 *     existing domain-neutral frontend AST
 *
 * SEMANTIC OWNER:
 *
 *     interoperability semantic analysis
 *
 * IR OWNER:
 *
 *     canonical semantic model and downstream domain IRs
 *
 * TEST OWNER:
 *
 *     grammar/tests/interoperability/data-layout/
 *
 * SPEC OWNER:
 *
 *     grammar/spec/interoperability.md
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * 1. ABI
 *
 * grammar/interoperability/abi.g4 may consume:
 *
 *     dataLayoutContract
 *     dataLayoutReference
 *
 * ABI syntax remains owned by abi.g4.
 *
 *
 * 2. FFI
 *
 * grammar/interoperability/ffi.g4 may attach a data-layout contract to a
 * foreign boundary.
 *
 * FFI remains responsible for foreign-call syntax.
 *
 *
 * 3. FOREIGN TYPES
 *
 * grammar/interoperability/foreign-types.g4 may reference data-layout
 * information for an externally implemented type.
 *
 * Foreign-type declaration syntax remains owned by foreign-types.g4.
 *
 *
 * 4. CALLING CONVENTIONS
 *
 * grammar/interoperability/calling-conventions.g4 may associate a calling
 * convention with an ABI whose representation requirements reference this
 * grammar.
 *
 * This file does not redefine calling conventions.
 *
 *
 * 5. SERIALIZATION
 *
 * Serialization/deserialization may consume the semantic representation
 * information produced from this grammar.
 *
 * Serialization format is not the same thing as ABI data layout.
 *
 *
 * 6. TYPES
 *
 * Type references use the canonical Type grammar.
 *
 *
 * 7. EXPRESSIONS
 *
 * Symbolic values use the canonical expression grammar.
 *
 *
 * 8. RESOURCES
 *
 * Resource requirements are resolved by the existing resource subsystem.
 *
 *
 * 9. EFFECTS
 *
 * Any conversion or foreign operation requiring effects is checked by the
 * existing effect subsystem.
 *
 *
 * 10. TARGETS
 *
 * Target-specific layout realization happens only downstream.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * A data-layout contract is source-level API.
 *
 * Changes to its meaning require explicit language-version or interoperability
 * compatibility handling.
 *
 * New property names MUST NOT silently change the meaning of an existing
 * property.
 *
 * Unknown open-world properties may be preserved for later semantic handling
 * where the surrounding interoperability policy permits them.
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Required semantic diagnostics include, where applicable:
 *
 *     - unknown type reference;
 *     - invalid layout reference;
 *     - duplicate exclusive property;
 *     - incompatible representation properties;
 *     - invalid size expression;
 *     - invalid alignment expression;
 *     - invalid offset expression;
 *     - invalid stride expression;
 *     - incompatible ABI contract;
 *     - unsupported required representation;
 *     - unsatisfied capability;
 *     - unsatisfied resource requirement;
 *     - forbidden policy;
 *     - incompatible interoperability version.
 *
 * These are semantic diagnostics, not parser-side target probing.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * The grammar is intentionally open-ended.
 *
 * It permits:
 *
 *     arbitrary symbolic type names;
 *     arbitrary qualified layout identities;
 *     arbitrary property names;
 *     symbolic dimensions;
 *     symbolic sizes;
 *     symbolic alignments;
 *     symbolic offsets;
 *     symbolic strides;
 *     arbitrary field identifiers;
 *     arbitrary layout metadata.
 *
 * No grammar-level fixed collection size is defined.
 *
 * Parser/compiler implementation limits, if any, are implementation resource
 * limits and MUST NOT become language semantics.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * This grammar contains no embedded actions.
 *
 * It performs no:
 *
 *     filesystem operation;
 *     network operation;
 *     native call;
 *     dynamic loading;
 *     target probing;
 *     hardware discovery;
 *     memory allocation;
 *     code execution.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     1. It compiles as an ANTLR4 parser grammar with ZamaniLexer.
 *     2. Every imported grammar exists under its canonical path.
 *     3. Every referenced parser rule is exported by its owner.
 *     4. No lexer rules are duplicated here.
 *     5. No general type rules are duplicated here.
 *     6. No ABI rules are duplicated here.
 *     7. No FFI rules are duplicated here.
 *     8. No calling-convention rules are duplicated here.
 *     9. No target-specific constants exist here.
 *    10. No machine layout is selected during parsing.
 *    11. Positive interoperability tests pass.
 *    12. Negative semantic tests exist.
 *    13. Symbolic/scalability tests exist.
 *    14. ABI/FFI integration tests exist.
 *    15. Documentation identifies this as the canonical data-layout grammar.
 *
 * ============================================================================
 */

parser grammar InteroperabilityDataLayout;

options {
    tokenVocab = ZamaniLexer;
}

import
    Names,
    Attributes,
    Expressions,
    Type
;


/*
 * ============================================================================
 * 1. TOP-LEVEL DECLARATION
 * ============================================================================
 *
 * The explicit EXTERN token keeps a data-layout declaration distinguishable
 * from ordinary identifier-led expressions/declarations.
 *
 * Examples:
 *
 *     extern layout platform::representation {
 *         ...
 *     }
 *
 *     extern layout "external.representation" {
 *         ...
 *     }
 *
 * The layout identity is symbolic and open-ended.
 */
dataLayoutDeclaration
    : attributeList?
      EXTERN
      LAYOUT
      dataLayoutIdentity
      dataLayoutBody
    ;


/*
 * ============================================================================
 * 2. REUSABLE CONTRACT
 * ============================================================================
 *
 * Interoperability dispatchers may invoke this rule after establishing their
 * own external-boundary context.
 */
dataLayoutContract
    : LAYOUT
      dataLayoutIdentity
      dataLayoutBody
    ;


/*
 * ============================================================================
 * 3. IDENTITY
 * ============================================================================
 *
 * A layout identity may be quoted or symbolic.
 *
 * Examples:
 *
 *     "external.representation"
 *     platform::layout
 *     vendor::representation::v2
 *
 * No finite registry is encoded by this grammar.
 */
dataLayoutIdentity
    : STRING
    | qualifiedName
    ;


/*
 * ============================================================================
 * 4. BODY
 * ============================================================================
 */

dataLayoutBody
    : LBRACE
      dataLayoutMember*
      RBRACE
    ;


/*
 * ============================================================================
 * 5. MEMBER DISPATCH
 * ============================================================================
 *
 * Keyword-led members are separated from the open-world property form.
 *
 * This avoids requiring the grammar to enumerate every future layout
 * convention.
 */
dataLayoutMember
    : dataLayoutProfileReference
    | dataLayoutTypeContract
    | dataLayoutRepresentation
    | dataLayoutFieldContract
    | dataLayoutRequirement
    | dataLayoutCapability
    | dataLayoutConstraint
    | dataLayoutCompatibility
    | dataLayoutOpaque
    | dataLayoutTransparent
    | dataLayoutProperty
    | attribute
    ;


/*
 * ============================================================================
 * 6. PROFILE
 * ============================================================================
 *
 * A profile names another symbolic layout description.
 *
 * Example:
 *
 *     profile platform::representation;
 *
 * PROFILE resolution is semantic.
 */
dataLayoutProfileReference
    : PROFILE
      dataLayoutReference
      SEMICOLON
    ;


/*
 * ============================================================================
 * 7. TYPE CONTRACT
 * ============================================================================
 *
 * Associates layout intent with an existing Zamani type.
 *
 * Example:
 *
 *     type SomeType {
 *         ...
 *     }
 *
 * The canonical Type grammar remains authoritative for the type itself.
 */
dataLayoutTypeContract
    : TYPE
      typeExpression
      dataLayoutBody
    ;


/*
 * ============================================================================
 * 8. REPRESENTATION
 * ============================================================================
 *
 * A representation can be symbolic.
 *
 * Examples:
 *
 *     representation scalar;
 *     representation aggregate;
 *     representation opaque;
 *     representation platform::custom;
 *
 * The grammar deliberately does not enumerate the legal representation
 * universe.
 */
dataLayoutRepresentation
    : REPRESENTATION
      dataLayoutReference
      SEMICOLON
    ;


/*
 * ============================================================================
 * 9. FIELD CONTRACT
 * ============================================================================
 *
 * A field is identified by a normal source-level name.
 *
 * Field layout values remain symbolic.
 *
 * Examples:
 *
 *     field value {
 *         ...
 *     }
 *
 *     field namespace::value {
 *         ...
 *     }
 */
dataLayoutFieldContract
    : FIELD
      qualifiedName
      dataLayoutBody
    ;


/*
 * ============================================================================
 * 10. REQUIREMENT
 * ============================================================================
 *
 * Layout requirements remain symbolic and are checked semantically.
 *
 * Examples:
 *
 *     requires size >= required_size;
 *     requires alignment >= required_alignment;
 *
 * The expression is deliberately delegated to the canonical expression
 * grammar where possible.
 */
dataLayoutRequirement
    : REQUIRES
      dataLayoutExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 11. CAPABILITY
 * ============================================================================
 *
 * Capability identity is symbolic.
 *
 * Examples:
 *
 *     capability "foreign.layout";
 *     capability platform::layout;
 */
dataLayoutCapability
    : CAPABILITY
      dataLayoutReference
      SEMICOLON
    ;


/*
 * ============================================================================
 * 12. CONSTRAINT
 * ============================================================================
 *
 * A layout constraint limits realization without selecting a concrete target.
 *
 * Example:
 *
 *     constraint alignment <= preferred_alignment;
 *
 * Semantic validation determines whether the constraint is satisfiable.
 */
dataLayoutConstraint
    : CONSTRAINT
      dataLayoutExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 13. COMPATIBILITY
 * ============================================================================
 *
 * Compatibility metadata remains symbolic.
 *
 * Examples:
 *
 *     compatible_with platform::layout;
 *     compatibility platform::layout;
 *
 * Only the canonical keyword-led form is accepted here.
 */
dataLayoutCompatibility
    : COMPATIBLE_WITH
      dataLayoutReference
      SEMICOLON
    ;


/*
 * ============================================================================
 * 14. OPAQUE
 * ============================================================================
 *
 * OPAQUE states that the representation is intentionally not exposed through
 * the source-level layout contract.
 *
 * This is semantic metadata, not a machine pointer.
 */
dataLayoutOpaque
    : OPAQUE
      SEMICOLON
    ;


/*
 * ============================================================================
 * 15. TRANSPARENT
 * ============================================================================
 *
 * TRANSPARENT states that the semantic boundary permits the referenced
 * representation to remain exposed.
 *
 * Exact transparency guarantees are semantic and ABI-dependent.
 */
dataLayoutTransparent
    : TRANSPARENT
      SEMICOLON
    ;


/*
 * ============================================================================
 * 16. OPEN-WORLD PROPERTY
 * ============================================================================
 *
 * Generic property form:
 *
 *     property = value;
 *
 * The property name is a symbolic identifier and the value is an expression
 * or symbolic value.
 *
 * This is the central extensibility mechanism.
 *
 * It avoids grammar changes for every future:
 *
 *     size model
 *     alignment model
 *     endian model
 *     address-space model
 *     representation convention
 *     vendor property
 *     accelerator representation
 *     quantum representation
 *     HDL representation
 *     distributed representation
 *
 * Examples:
 *
 *     endian = platform::native;
 *     size = sizeof(T);
 *     alignment = required_alignment;
 *     stride = element_stride;
 *     offset = field_offset;
 *     address_space = memory::default;
 *     bit_order = platform::native;
 *
 * Semantic validation owns the legal property vocabulary.
 */
dataLayoutProperty
    : PROPERTY
      identifier
      ASSIGN
      dataLayoutExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 17. REFERENCE
 * ============================================================================
 *
 * Symbolic layout references are open-world.
 */
dataLayoutReference
    : qualifiedName
    | STRING
    ;


/*
 * ============================================================================
 * 18. EXPRESSION
 * ============================================================================
 *
 * The exact expression grammar remains owned by Expressions.
 *
 * This rule provides a local integration boundary so that future changes to
 * the general expression grammar do not require this file to duplicate
 * expression syntax.
 *
 * If the canonical Expressions grammar exports `expression`, this rule is
 * intentionally a forwarding boundary.
 */
dataLayoutExpression
    : expression
    ;
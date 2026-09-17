/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/declarations/attributes.g4
 *
 * Grammar:
 *     ZamaniDeclarationAttributes
 *
 * Status:
 *     Production / canonical declaration-attribute integration grammar
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     No embedded Rust.
 *     No semantic predicates.
 *     No actions.
 *     No filesystem access.
 *     No network access.
 *     No runtime callbacks.
 *     No hardware discovery.
 *     No unsafe code.
 *
 * ============================================================================
 *
 * PURPOSE
 * ============================================================================
 *
 * This file owns the DECLARATION-LAYER ATTACHMENT CONTRACT for attributes.
 *
 * The generic attribute syntax itself is NOT owned here.
 *
 * Generic attribute syntax is owned by:
 *
 *     grammar/core/attributes.g4
 *
 * This file exists because declarations need a stable, independently
 * composable grammar boundary describing where and how the generic attribute
 * construct is attached to declaration-family constructs.
 *
 * The architectural separation is:
 *
 *     grammar/core/attributes.g4
 *         |
 *         +--> generic attribute syntax
 *         |
 *         v
 *     grammar/declarations/attributes.g4
 *         |
 *         +--> declaration attachment contract
 *         |
 *         v
 *     declaration-family grammars
 *
 * This prevents:
 *
 *     - duplicate @ syntax;
 *     - duplicate attribute-name syntax;
 *     - duplicate attribute-value syntax;
 *     - duplicate literal syntax;
 *     - duplicate qualified-name syntax;
 *     - domain-specific attribute grammars;
 *     - competing declaration attribute implementations.
 *
 * ============================================================================
 *
 * LANGUAGE OBJECTIVES
 * ============================================================================
 *
 * This grammar participates in:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * (POCO-REAF).
 *
 * The declaration attribute layer therefore remains independent of:
 *
 *     - CPU count;
 *     - core count;
 *     - thread count;
 *     - GPU count;
 *     - FPGA count;
 *     - ASIC count;
 *     - QPU count;
 *     - qubit count;
 *     - node count;
 *     - device count;
 *     - memory capacity;
 *     - register width;
 *     - vector width;
 *     - tensor rank;
 *     - network topology;
 *     - physical addresses;
 *     - physical qubit identifiers;
 *     - accelerator identifiers;
 *     - deployment topology.
 *
 * No finite machine/resource limit is encoded by this grammar.
 *
 * ============================================================================
 *
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - declaration attribute attachment;
 *     - required declaration attribute prefixes;
 *     - optional declaration attribute prefixes;
 *     - declaration-level attribute groups;
 *     - stable declaration-facing wrapper rule names;
 *     - declaration/member attachment integration;
 *     - declaration attribute compatibility boundaries.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical token definitions;
 *     - keyword spelling;
 *     - punctuation;
 *     - identifiers;
 *     - qualified names;
 *     - literal syntax;
 *     - generic attribute syntax;
 *     - generic attribute values;
 *     - annotation syntax;
 *     - metadata semantics;
 *     - compiler directives;
 *     - capability semantics;
 *     - resource semantics;
 *     - quantum semantics;
 *     - QEC;
 *     - ZQN;
 *     - routing;
 *     - scheduling;
 *     - optimization;
 *     - HAL;
 *     - hardware discovery;
 *     - device selection;
 *     - physical placement;
 *     - AST implementation;
 *     - semantic analysis;
 *     - IR construction;
 *     - runtime execution.
 *
 * ============================================================================
 *
 * SINGLE ATTRIBUTE AUTHORITY
 * ============================================================================
 *
 * The generic attribute syntax is owned by:
 *
 *     grammar/core/attributes.g4
 *
 * Therefore this file MUST NOT define:
 *
 *     attribute
 *     attributeName
 *     qualifiedAttributeName
 *     attributeArguments
 *     attributeArgument
 *     attributeValue
 *     attributeListValue
 *     attributeMapValue
 *     attributeNestedValue
 *
 * again.
 *
 * Those rules belong to the generic attribute grammar.
 *
 * This file only provides declaration-context wrappers around them.
 *
 * ============================================================================
 *
 * ANTLR DEPENDENCY CONTRACT
 * ============================================================================
 *
 * The production declaration grammar family uses the canonical lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * The generic attribute grammar must be migrated to the same canonical
 * vocabulary during lexer consolidation.
 *
 * The final dependency graph is:
 *
 *     grammar/lexer/*
 *             |
 *             v
 *     grammar/lexer/tokens.g4
 *             |
 *             v
 *     grammar/antlr/ZamaniLexer.g4
 *             |
 *             v
 *     grammar/core/names.g4
 *             |
 *             v
 *     grammar/core/attributes.g4
 *             |
 *             v
 *     grammar/declarations/attributes.g4
 *             |
 *             v
 *     declaration-family grammars
 *
 * No declaration attribute grammar may introduce another lexer.
 *
 * ============================================================================
 *
 * IMPORTANT TOKEN INTEGRATION
 * ============================================================================
 *
 * This grammar deliberately does NOT reference raw punctuation tokens.
 *
 * It inherits the generic `attribute` rule from:
 *
 *     grammar/core/attributes.g4
 *
 * Consequently this file does not need to know whether the implementation
 * represents:
 *
 *     @
 *     ::
 *     (
 *     )
 *     [
 *     ]
 *     {
 *     }
 *     ,
 *     =
 *
 * through particular lexer token IDs.
 *
 * This is intentional.
 *
 * ============================================================================
 *
 * GENERIC ATTRIBUTE EXAMPLES
 * ============================================================================
 *
 * The inherited generic syntax permits forms such as:
 *
 *     @inline
 *     @deprecated
 *     @quantum::logical
 *     @hardware::capability
 *
 *     @inline()
 *     @resource(memory = required)
 *     @requires(capability = quantum::measurement)
 *
 *     @features([
 *         quantum::measurement,
 *         quantum::reset,
 *         quantum::control,
 *     ])
 *
 *     @resource({
 *         memory = required,
 *         accelerator = quantum,
 *     })
 *
 * The declaration grammar does not decide whether these names are valid.
 *
 * Semantic analysis decides:
 *
 *     - whether the attribute exists;
 *     - whether it may attach to a declaration;
 *     - whether its arguments are legal;
 *     - whether values have the required types;
 *     - whether it is portable;
 *     - whether it is a requirement, preference, constraint, hint, policy,
 *       or metadata;
 *     - whether target-specific meaning is valid.
 *
 * ============================================================================
 *
 * DECLARATION ATTACHMENT
 * ============================================================================
 *
 * A declaration attribute prefix is one or more attributes immediately
 * associated with the declaration that follows.
 *
 * Example:
 *
 *     @inline
 *     @pure
 *     fn compute() {
 *     }
 *
 * The grammar preserves the ordered attribute sequence.
 *
 * It does not assign semantics to attribute ordering.
 *
 * ============================================================================
 *
 * PUBLIC RULES
 * ============================================================================
 *
 * The following rules are the stable declaration-facing API:
 *
 *     declarationAttributes
 *     optionalDeclarationAttributes
 *     declarationAttributePrefix
 *     optionalDeclarationAttributePrefix
 *     declarationAttribute
 *     declarationAttributeList
 *
 * Downstream declaration grammars SHOULD use these rules instead of importing
 * the generic `attributeList` directly when they want the declaration-layer
 * contract to be explicit.
 *
 * ============================================================================
 *
 * REQUIRED DECLARATION ATTRIBUTES
 * ============================================================================
 *
 * `declarationAttributes` requires at least one attribute.
 *
 * Example:
 *
 *     @public
 *     struct Data {
 *     }
 *
 * ============================================================================
 *
 * OPTIONAL DECLARATION ATTRIBUTES
 * ============================================================================
 *
 * `optionalDeclarationAttributes` allows zero or more attributes.
 *
 * This rule is intended for declaration grammars whose declaration forms may
 * appear either with or without attributes.
 *
 * ============================================================================
 *
 * NO FIXED CARDINALITY
 * ============================================================================
 *
 * Attribute repetition uses ANTLR repetition operators.
 *
 * There is no language-level maximum for:
 *
 *     - declarations;
 *     - attributes per declaration;
 *     - qualified-name segments;
 *     - arguments;
 *     - nested values;
 *     - list elements;
 *     - map entries.
 *
 * Any implementation safety limits belong to compiler/resource policy.
 *
 * They MUST NOT become grammar constants.
 *
 * ============================================================================
 *
 * DECLARATION ATTRIBUTE SEMANTICS
 * ============================================================================
 *
 * Parsing establishes only structure.
 *
 * Semantic analysis must subsequently determine:
 *
 *     - duplicate declaration attributes;
 *     - attribute schema;
 *     - valid attachment points;
 *     - argument names;
 *     - argument cardinality;
 *     - value types;
 *     - namespace resolution;
 *     - capability requirements;
 *     - resource requirements;
 *     - effect requirements;
 *     - portability;
 *     - compiler hints;
 *     - target constraints;
 *     - security policy;
 *     - domain-specific interpretation.
 *
 * The grammar must never make these decisions.
 *
 * ============================================================================
 *
 * ATTRIBUTE CATEGORIES
 * ============================================================================
 *
 * The declaration grammar deliberately does NOT enumerate:
 *
 *     QuantumAttribute
 *     ClassicalAttribute
 *     HDLAttribute
 *     HardwareAttribute
 *     GPUAttribute
 *     FPGAAttribute
 *     QPUAttribute
 *     QECAttribute
 *     ZQNAttribute
 *     AIAttribute
 *
 * Attribute namespaces remain extensible.
 *
 * New domains therefore do not require this grammar file to be rewritten.
 *
 * ============================================================================
 *
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * A declaration may carry quantum-related metadata:
 *
 *     @quantum::logical
 *     @quantum::resource(...)
 *     @quantum::capability(...)
 *     @qec::require(...)
 *     @noise::budget(...)
 *
 * This grammar merely preserves the attribute.
 *
 * The semantic pipeline remains:
 *
 *     declaration
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic validation
 *          |
 *          v
 *     quantum semantic representation
 *          |
 *          v
 *     quantum::ir
 *          |
 *          +--> optimization
 *          +--> QEC
 *          +--> ZQN
 *          +--> routing
 *          +--> scheduling
 *          +--> HAL
 *
 * This file MUST NOT:
 *
 *     - allocate physical qubits;
 *     - choose a QPU;
 *     - select topology;
 *     - select gates;
 *     - perform routing;
 *     - perform scheduling;
 *     - perform QEC;
 *     - implement ZQN;
 *     - construct quantum::ir.
 *
 * ============================================================================
 *
 * CLASSICAL / HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * The same declaration attribute mechanism applies to:
 *
 *     classical declarations;
 *     numerical declarations;
 *     tensor declarations;
 *     AI/ML declarations;
 *     HDL declarations;
 *     hardware/software co-design declarations;
 *     distributed declarations;
 *     networking declarations;
 *     security declarations;
 *     resource declarations;
 *     interoperability declarations;
 *     dialect declarations;
 *     future domains.
 *
 * Domain-specific semantics remain outside this grammar.
 *
 * ============================================================================
 *
 * RESOURCE / CAPABILITY SEPARATION
 * ============================================================================
 *
 * Attributes may express semantic intent such as:
 *
 *     @requires(...)
 *     @capability(...)
 *     @resource(...)
 *     @constraint(...)
 *     @prefer(...)
 *     @hint(...)
 *
 * The grammar does not decide which category applies.
 *
 * In particular:
 *
 *     @requires(qubits = n)
 *
 * does not mean:
 *
 *     use physical qubits 0 through n-1
 *
 * and:
 *
 *     @prefer(accelerator = quantum)
 *
 * does not mean:
 *
 *     select QPU device 0.
 *
 * Physical realization is downstream.
 *
 * ============================================================================
 *
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar produces parse-tree structure only.
 *
 * The frontend must map declaration attributes into the canonical
 * domain-neutral AST.
 *
 * The AST must preserve:
 *
 *     - declaration attribute ordering;
 *     - attribute name;
 *     - qualified name;
 *     - argument ordering;
 *     - positional/named distinction;
 *     - scalar values;
 *     - symbolic values;
 *     - list ordering;
 *     - map ordering;
 *     - duplicate map entries;
 *     - nested attributes;
 *     - source spans.
 *
 * Recommended conceptual shape:
 *
 *     DeclarationAttributes
 *         attributes: ordered sequence
 *
 *     Attribute
 *         name
 *         arguments
 *         source_span
 *
 * The exact Rust type names belong to:
 *
 *     src/frontend/ast/
 *
 * This grammar must not introduce another AST.
 *
 * ============================================================================
 *
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Attribute attachment validity is semantic.
 *
 * Examples:
 *
 *     @field_only
 *     struct Example {}
 *
 * may be syntactically valid but semantically invalid if `field_only` is not
 * permitted on structs.
 *
 * Likewise:
 *
 *     @requires(qubits = 4)
 *
 * is syntactically valid without establishing whether four qubits are
 * available on a target.
 *
 * Resource admission belongs downstream.
 *
 * ============================================================================
 *
 * COMPILER CONTRACT
 * ============================================================================
 *
 * Validated declaration attributes may lower to:
 *
 *     - semantic metadata;
 *     - capability requirements;
 *     - resource requirements;
 *     - optimization hints;
 *     - provenance;
 *     - verification metadata;
 *     - interoperability metadata;
 *     - execution policies.
 *
 * The compiler may discard attributes whose semantic contract says they are
 * source-only metadata.
 *
 * It must not infer hardware decisions solely from parsing.
 *
 * ============================================================================
 *
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * This grammar has no runtime dependency.
 *
 * Attributes that survive compilation may influence runtime policy only after
 * semantic validation and explicit lowering.
 *
 * The grammar itself performs no runtime operation.
 *
 * ============================================================================
 *
 * TOOLING CONTRACT
 * ============================================================================
 *
 * Source spans must be preserved for:
 *
 *     - diagnostics;
 *     - IDE navigation;
 *     - formatting;
 *     - syntax highlighting;
 *     - refactoring;
 *     - documentation generation;
 *     - provenance;
 *     - source-to-AST mapping.
 *
 * Tooling must be able to identify the exact declaration and exact attribute
 * that produced a diagnostic.
 *
 * ============================================================================
 *
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing declaration grammars must migrate from:
 *
 *     attribute+
 *
 * or direct ad-hoc attribute syntax
 *
 * to:
 *
 *     declarationAttributePrefix
 *
 * where the declaration is explicitly attribute-bearing.
 *
 * Existing source forms remain structurally compatible because the underlying
 * generic `attribute` rule is unchanged.
 *
 * This migration must not introduce a second `@...` syntax.
 *
 * ============================================================================
 *
 * ANNOTATION COMPATIBILITY
 * ============================================================================
 *
 * `grammar/core/annotations.g4` is a separate historical/source-level
 * construct.
 *
 * It must not be reimplemented here.
 *
 * If annotations and attributes are eventually declared semantically
 * equivalent, the compatibility layer should normalize them downstream into
 * the canonical AST representation.
 *
 * This file must not create a third annotation/attribute syntax.
 *
 * ============================================================================
 *
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden:
 *
 *     MAX_ATTRIBUTES
 *     MAX_ATTRIBUTE_ARGUMENTS
 *     MAX_ATTRIBUTE_DEPTH
 *     MAX_ATTRIBUTE_NAME_LENGTH
 *     MAX_DECLARATION_ATTRIBUTES
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
 *     MAX_VECTOR_WIDTH
 *
 * Also forbidden:
 *
 *     physical device IDs;
 *     fixed hardware topology;
 *     fixed qubit IDs;
 *     fixed accelerator IDs;
 *     target-specific parser branches.
 *
 * ============================================================================
 *
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no actions;
 *     - no semantic predicates;
 *     - no randomness;
 *     - no filesystem access;
 *     - no network access;
 *     - no environment access;
 *     - no hardware discovery;
 *     - no runtime calls.
 *
 * For a fixed token stream and grammar version, parsing is deterministic.
 *
 * ============================================================================
 *
 * SECURITY
 * ============================================================================
 *
 * Attributes are untrusted source input.
 *
 * Successful parsing does NOT establish:
 *
 *     - authorization;
 *     - capability possession;
 *     - resource ownership;
 *     - device access;
 *     - trust;
 *     - certificate validity;
 *     - security policy compliance.
 *
 * Attribute names and values must remain inert until explicitly validated by
 * downstream semantic/security components.
 *
 * ============================================================================
 *
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 * [x] Generic attribute syntax is delegated to core/attributes.g4.
 * [x] Declaration attachment has a dedicated grammar boundary.
 * [x] Required declaration attributes are supported.
 * [x] Optional declaration attributes are supported.
 * [x] Attribute ordering is preserved by repetition.
 * [x] No attribute count limit exists in the grammar.
 * [x] No argument count limit exists in the grammar.
 * [x] No nested-depth limit exists in the grammar.
 * [x] No machine/resource limits exist.
 * [x] No hardware assumptions exist.
 * [x] No quantum physical assumptions exist.
 * [x] No second quantum IR exists.
 * [x] No lexer rules are duplicated.
 * [x] No identifier rules are duplicated.
 * [x] No literal rules are duplicated.
 * [x] No expression grammar is duplicated.
 * [x] No annotation grammar is duplicated.
 * [x] AST integration is explicitly defined.
 * [x] Semantic integration is explicitly defined.
 * [x] Compiler integration is explicitly defined.
 * [x] Runtime integration is explicitly defined.
 * [x] Tooling integration is explicitly defined.
 * [x] Compatibility migration is defined.
 * [x] Rust 1.97 / 1.97.1 compatibility is defined.
 * [x] No unsafe Rust is required.
 *
 * ============================================================================
 *
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive declaration-level examples:
 *
 *     @inline
 *     struct Data {}
 *
 *     @public
 *     @quantum::logical
 *     type QuantumState = QState;
 *
 *     @resource(memory = required)
 *     const value = 1;
 *
 *     @requires(capability = quantum::measurement)
 *     fn measure() {}
 *
 *     @hardware::capability(name = accelerator)
 *     struct AcceleratorIntent {}
 *
 *     @a @b @c
 *     struct MultiAttributed {}
 *
 *     @resource({
 *         memory = required,
 *         accelerator = quantum,
 *     })
 *     struct ResourceAware {}
 *
 * Negative declaration-attribute attachment examples:
 *
 *     @
 *     @::name
 *     @name::
 *     @name(=value)
 *     @name(key=)
 *
 * These are generic attribute syntax failures and must be rejected by the
 * canonical core attribute grammar.
 *
 * Boundary examples:
 *
 *     @a
 *     @a::b
 *     @a::b::c::d
 *     @a(v)
 *     @a(k=v)
 *     @a([v1, v2, v3])
 *     @a({k1=v1, k2=v2})
 *     @a(@b(@c))
 *
 * Scalability examples:
 *
 *     @a @b @c ... declaration
 *
 * with arbitrarily many attributes subject only to implementation resources.
 *
 * Compatibility tests must cover:
 *
 *     constants;
 *     variables;
 *     aliases;
 *     named types;
 *     structs;
 *     enums;
 *     unions;
 *     interfaces;
 *     traits;
 *     implementations;
 *     function declarations;
 *     module declarations;
 *     quantum declarations;
 *     HDL declarations;
 *     hardware declarations;
 *     resource declarations;
 *     capability declarations;
 *     AI declarations;
 *     distributed declarations;
 *     interoperability declarations;
 *     dialect declarations.
 *
 * ============================================================================
 */

parser grammar ZamaniDeclarationAttributes;

options {
    tokenVocab = ZamaniLexer;
}

import Attributes;


/*
 * ============================================================================
 * DECLARATION ATTRIBUTE ENTRY POINTS
 * ============================================================================
 */

/**
 * One or more attributes attached to a declaration.
 *
 * Example:
 *
 *     @inline
 *     @pure
 *     fn compute() {}
 *
 * The imported `attribute` rule owns the actual syntax.
 */
declarationAttributes
    : attribute+
    ;


/**
 * Zero or more declaration attributes.
 *
 * This is the normal entry point for declaration forms that may optionally
 * carry attributes.
 */
optionalDeclarationAttributes
    : attribute*
    ;


/**
 * Required declaration attribute prefix.
 *
 * Stable downstream-facing name.
 */
declarationAttributePrefix
    : declarationAttributes
    ;


/**
 * Optional declaration attribute prefix.
 */
optionalDeclarationAttributePrefix
    : optionalDeclarationAttributes
    ;


/**
 * One declaration attribute.
 *
 * This wrapper exists so declaration-family grammars can explicitly state
 * that an individual item is a declaration attribute without taking
 * ownership of generic attribute syntax.
 */
declarationAttribute
    : attribute
    ;


/**
 * One or more declaration attributes.
 *
 * This is an explicit declaration-context alias for declarationAttributes.
 */
declarationAttributeList
    : declarationAttributes
    ;


/*
 * ============================================================================
 * ATTRIBUTE-BEARING DECLARATION SHAPE
 * ============================================================================
 *
 * This rule does NOT consume the declaration itself.
 *
 * It provides a stable composition point for consumers:
 *
 *     declarationAttributePrefix declaration
 *
 * The declaration dispatcher remains the owner of `declaration`.
 */
attributedDeclarationPrefix
    : declarationAttributePrefix
    ;


/*
 * ============================================================================
 * FIELD / MEMBER ATTACHMENT SUPPORT
 * ============================================================================
 *
 * Declaration members such as struct fields, enum variants, interface
 * members, trait members and implementation members may independently choose
 * whether they permit declaration attributes.
 *
 * This grammar supplies the reusable attachment wrapper without deciding
 * which member categories permit it.
 */
memberDeclarationAttributes
    : optionalDeclarationAttributes
    ;


/*
 * ============================================================================
 * COMPLETION CONTRACT
 * ============================================================================
 *
 * This grammar:
 *
 *     - imports the canonical generic attribute grammar;
 *     - does not duplicate attribute syntax;
 *     - does not duplicate lexical tokens;
 *     - does not duplicate names;
 *     - does not duplicate literals;
 *     - does not duplicate expressions;
 *     - does not duplicate annotations;
 *     - provides declaration-specific attachment rules;
 *     - preserves arbitrary attribute cardinality;
 *     - preserves arbitrary nesting through the generic attribute grammar;
 *     - remains target independent;
 *     - remains hardware independent;
 *     - remains quantum-backend independent;
 *     - preserves POCO-REAF;
 *     - requires no unsafe Rust;
 *     - is compatible with Rust 1.97 / 1.97.1 generated-parser integration.
 *
 * ============================================================================
 */
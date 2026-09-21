/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/modules/module-attributes.g4
 *
 * Grammar:
 *     ModuleAttributes
 *
 * Status:
 *     CANONICAL MODULE-ATTRIBUTE COMPOSITION GRAMMAR
 *
 * Language:
 *     Zamani
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Rust implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Rust safety:
 *     Safe Rust only.
 *     This grammar contains no embedded Rust, no actions, no semantic
 *     predicates, and no unsafe code.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file defines the MODULE-SPECIFIC ATTACHMENT boundary for attributes.
 *
 * It does NOT redefine the general Zamani attribute language.
 *
 * The canonical generic attribute syntax is owned by:
 *
 *     grammar/core/attributes.g4
 *
 * whose parser grammar is:
 *
 *     Attributes
 *
 * This file composes that grammar and provides stable module-specific entry
 * points.
 *
 * Therefore:
 *
 *     generic attribute syntax
 *             |
 *             v
 *     core/attributes.g4
 *             |
 *             v
 *     ModuleAttributes
 *             |
 *             v
 *     modules.g4
 *
 * ============================================================================
 * PRIMARY ARCHITECTURAL RULE
 * ============================================================================
 *
 * A module attribute is an attribute attached to a module declaration.
 *
 * The module subsystem owns:
 *
 *     - where module attributes may occur;
 *     - the module-specific attribute-list boundary;
 *     - compatibility names for module attribute consumers.
 *
 * The module subsystem does NOT own:
 *
 *     - the @ syntax;
 *     - attribute names;
 *     - qualified attribute names;
 *     - positional arguments;
 *     - named arguments;
 *     - scalar values;
 *     - lists;
 *     - maps;
 *     - nested attributes;
 *     - trailing-comma behavior;
 *     - attribute value syntax.
 *
 * Those are owned by:
 *
 *     grammar/core/attributes.g4
 *
 * ============================================================================
 * AUTHORITY HIERARCHY
 * ============================================================================
 *
 * The repository-wide language authority remains:
 *
 *     grammar/DESIGN.md
 *          |
 *          v
 *     grammar/specification/
 *          |
 *          v
 *     grammar/spec/
 *          |
 *          v
 *     canonical grammar composition
 *          |
 *          +--> grammar/Zamani.g4
 *          |
 *          +--> grammar/antlr/ZamaniParser.g4
 *          |
 *          +--> grammar/modules/modules.g4
 *          |
 *          +--> grammar/core/attributes.g4
 *          |
 *          +--> this file
 *          |
 *          v
 *     lexer / parser implementation
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     canonical semantic representations / IR
 *
 * This file is therefore a syntax-composition component, not a language
 * specification authority and not a semantic implementation.
 *
 * ============================================================================
 * CANONICAL LEXICAL BOUNDARY
 * ============================================================================
 *
 * The production ANTLR lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Its lexical vocabulary is composed through:
 *
 *     grammar/lexer/tokens.g4
 *
 * whose grammar name is:
 *
 *     ZamaniTokens
 *
 * Parser grammars therefore consume the canonical token vocabulary through:
 *
 *     tokenVocab = ZamaniTokens;
 *
 * This file does not define or duplicate lexical tokens.
 *
 * In particular, this file MUST NOT define its own:
 *
 *     AT
 *     IDENTIFIER
 *     STRING_LITERAL
 *     INTEGER_LITERAL
 *     FLOAT_LITERAL
 *     CHARACTER_LITERAL
 *     TRUE
 *     FALSE
 *     NULL
 *     punctuation
 *     operators
 *
 * or equivalent duplicate lexical concepts.
 *
 * ============================================================================
 * COMPOSITION
 * ============================================================================
 *
 * Generic attributes are owned by:
 *
 *     grammar/core/attributes.g4
 *
 * Parser grammar:
 *
 *     Attributes
 *
 * This file imports:
 *
 *     Attributes
 *
 * It intentionally does NOT import:
 *
 *     QualifiedNames
 *
 * directly, because qualified names are already consumed transitively through
 * the canonical Attributes grammar.
 *
 * This keeps the dependency graph shallow and prevents module attributes from
 * accidentally becoming another owner of name syntax.
 *
 * ============================================================================
 * DEPENDENCY DIRECTION
 * ============================================================================
 *
 *     ZamaniTokens
 *          |
 *          v
 *     Names
 *          |
 *          v
 *     Attributes
 *          |
 *          v
 *     ModuleAttributes
 *          |
 *          v
 *     Modules
 *          |
 *          v
 *     canonical parser composition
 *
 * The dependency direction MUST NOT be reversed.
 *
 * The generic attribute grammar must not depend on module attributes.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - moduleAttributes;
 *     - moduleAttribute;
 *     - module-specific attribute attachment;
 *     - stable compatibility wrappers for module attribute consumers;
 *     - the distinction between zero-or-more and one-or-more module attributes.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - generic attribute syntax;
 *     - attribute names;
 *     - qualified names;
 *     - identifiers;
 *     - literals;
 *     - lists;
 *     - maps;
 *     - nested attribute values;
 *     - expressions;
 *     - declarations;
 *     - modules themselves;
 *     - imports;
 *     - exports;
 *     - namespaces;
 *     - packages;
 *     - dependencies;
 *     - versioning;
 *     - visibility semantics;
 *     - symbol resolution;
 *     - package resolution;
 *     - dependency solving;
 *     - compiler configuration;
 *     - resource allocation;
 *     - capability discovery;
 *     - target selection;
 *     - hardware discovery;
 *     - quantum routing;
 *     - quantum scheduling;
 *     - QEC;
 *     - ZQN;
 *     - calibration;
 *     - HAL;
 *     - runtime execution.
 *
 * ============================================================================
 * WHY THIS FILE MUST NOT DUPLICATE ATTRIBUTES
 * ============================================================================
 *
 * The previous implementation independently described:
 *
 *     moduleAttribute
 *     moduleAttributeArguments
 *     moduleAttributeArgument
 *     moduleAttributeArgumentValue
 *     moduleAttributeLiteralValue
 *     moduleAttributeListValue
 *     moduleAttributeNestedValue
 *
 * That creates a second attribute language.
 *
 * The repository already has a broader canonical attribute grammar in:
 *
 *     grammar/core/attributes.g4
 *
 * That grammar already provides:
 *
 *     attribute
 *     attributeList
 *     optionalAttributes
 *     attributeName
 *     qualifiedAttributeName
 *     attributeArguments
 *     attributeArgumentList
 *     attributeArgument
 *     attributeNamedArgument
 *     attributePositionalArgument
 *     attributeArgumentName
 *     attributeValue
 *     attributeScalarValue
 *     attributeNameValue
 *     attributeListValue
 *     attributeValueList
 *     attributeMapValue
 *     attributeMapEntryList
 *     attributeMapEntry
 *     attributeMapKey
 *     attributeNestedValue
 *
 * It also already defines:
 *
 *     - trailing commas;
 *     - structured lists;
 *     - structured maps;
 *     - nested attributes;
 *     - qualified names;
 *     - canonical lexical vocabulary;
 *     - generic attribute values.
 *
 * Reimplementing those rules here would make future changes require multiple
 * synchronized edits and would violate the repository's single-owner rule.
 *
 * ============================================================================
 * PUBLIC COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing module grammar consumers may use:
 *
 *     moduleAttributes
 *     moduleAttribute
 *
 * These names are deliberately retained.
 *
 * Their implementation is now a thin wrapper over the canonical generic
 * attribute grammar.
 *
 * This allows:
 *
 *     modules.g4
 *
 * to remain module-focused without creating a duplicate attribute language.
 *
 * ============================================================================
 * MODULE ATTRIBUTE LIST
 * ============================================================================
 *
 * A module may have zero or more attributes.
 *
 * Examples:
 *
 *     moduleAttributes
 *
 *     @test
 *     module example {
 *         ...
 *     }
 *
 *     @inline
 *     @experimental
 *     @quantum::logical
 *     module quantum::algorithm {
 *         ...
 *     }
 *
 * No finite number of attributes is imposed by this grammar.
 *
 * The grammar deliberately uses:
 *
 *     *
 *
 * rather than a fixed repetition count.
 *
 * ============================================================================
 */

parser grammar ModuleAttributes;

options {
    tokenVocab = ZamaniTokens;
}

import Attributes;


/*
 * ============================================================================
 * MODULE ATTRIBUTE LIST
 * ============================================================================
 *
 * Zero or more attributes that may precede/attach to a module declaration.
 *
 * This is the module-specific attachment boundary.
 *
 * It does not redefine attribute syntax.
 *
 * Examples:
 *
 *     <empty>
 *
 *     @test
 *
 *     @inline
 *     @experimental
 *
 *     @quantum::logical
 *     @resource(memory = 1GiB)
 *
 *     @compiler::profile(
 *         optimization = "portable",
 *     )
 *
 *     @hardware::capability([
 *         accelerator,
 *         quantum::measurement,
 *     ])
 *
 * The legality and meaning of those attributes are semantic concerns.
 */
moduleAttributes
    : attribute*
    ;


/*
 * ============================================================================
 * SINGLE MODULE ATTRIBUTE
 * ============================================================================
 *
 * Compatibility wrapper around the canonical generic attribute rule.
 *
 * The actual syntax is owned by:
 *
 *     Attributes.attribute
 *
 * Consequently all generic attribute features remain available here:
 *
 *     @name
 *     @name(...)
 *     @namespace::name
 *     @namespace::name(...)
 *
 * including the generic structured-value model.
 */
moduleAttribute
    : attribute
    ;


/*
 * ============================================================================
 * REQUIRED MODULE ATTRIBUTE LIST
 * ============================================================================
 *
 * This rule is provided for consumers that already know that at least one
 * module attribute is required.
 *
 * It introduces no new syntax.
 */
requiredModuleAttributes
    : attribute+
    ;


/*
 * ============================================================================
 * OPTIONAL MODULE ATTRIBUTE LIST
 * ============================================================================
 *
 * Explicit optional wrapper.
 *
 * This is intentionally equivalent to moduleAttributes.
 *
 * It exists as a stable integration boundary for grammar consumers that prefer
 * a named optional module-attribute production.
 */
optionalModuleAttributes
    : moduleAttributes
    ;


/*
 * ============================================================================
 * MODULE ATTRIBUTE ALIAS
 * ============================================================================
 *
 * Compatibility wrapper for consumers that need to distinguish an attribute
 * syntactically attached to a module from attributes attached elsewhere.
 *
 * It remains structurally identical to the canonical attribute.
 *
 * Semantic analysis determines the attachment context.
 */
moduleAttributeDeclaration
    : attribute
    ;


/*
 * ============================================================================
 * NO SPECIALIZED VALUE GRAMMAR
 * ============================================================================
 *
 * The following rules are intentionally NOT defined here:
 *
 *     moduleAttributeArguments
 *     moduleAttributeArgumentList
 *     moduleAttributeArgument
 *     moduleAttributeNamedArgument
 *     moduleAttributePositionalArgument
 *     moduleAttributeArgumentName
 *     moduleAttributeArgumentValue
 *     moduleAttributeQualifiedValue
 *     moduleAttributeIdentifierValue
 *     moduleAttributeLiteralValue
 *     moduleAttributeListValue
 *     moduleAttributeListElements
 *     moduleAttributeNestedValue
 *
 * They previously duplicated generic attribute syntax.
 *
 * Their canonical equivalents are owned by:
 *
 *     grammar/core/attributes.g4
 *
 * and should be consumed through:
 *
 *     attribute
 *     attributeArguments
 *     attributeArgumentList
 *     attributeArgument
 *     attributeNamedArgument
 *     attributePositionalArgument
 *     attributeArgumentName
 *     attributeValue
 *     attributeScalarValue
 *     attributeNameValue
 *     attributeListValue
 *     attributeValueList
 *     attributeMapValue
 *     attributeMapEntryList
 *     attributeMapEntry
 *     attributeNestedValue
 *
 * This is a deliberate architectural constraint.
 *
 * ============================================================================
 * ATTRIBUTE VALUE MODEL
 * ============================================================================
 *
 * Because this grammar delegates to Attributes, module attributes inherit the
 * canonical structural value model:
 *
 *     scalar
 *     symbolic name
 *     list
 *     map
 *     nested attribute
 *
 * Examples:
 *
 *     @module
 *
 *     @module(version = "1")
 *
 *     @module(
 *         profile = "portable",
 *     )
 *
 *     @module(
 *         capabilities = [
 *             quantum::measurement,
 *             tensor::compute,
 *         ],
 *     )
 *
 *     @module(
 *         resources = {
 *             memory = 4GiB,
 *             accelerator = quantum,
 *         },
 *     )
 *
 *     @module(
 *         metadata = @nested::attribute,
 *     )
 *
 * The parser only recognizes structure.
 *
 * It does not decide whether any particular attribute or value is legal.
 *
 * ============================================================================
 * EXPRESSION SEPARATION
 * ============================================================================
 *
 * Module attributes do not introduce a second expression language.
 *
 * The canonical generic attribute grammar intentionally treats attribute
 * values as structural metadata.
 *
 * Therefore this file must NOT add a rule such as:
 *
 *     moduleAttributeValue : expression ;
 *
 * merely to make attributes "more powerful".
 *
 * If Zamani later requires arbitrary compile-time expressions inside
 * attributes, that feature must be introduced through:
 *
 *     specification
 *         ->
 *     AST contract
 *         ->
 *     canonical compile-time-expression contract
 *         ->
 *     semantic validation
 *         ->
 *     IR/compiler integration
 *
 * It must not be approximated by another private expression grammar here.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing a module attribute does NOT mean the attribute is valid.
 *
 * Semantic analysis is responsible for:
 *
 *     - resolving the attribute name;
 *     - identifying the attribute definition;
 *     - validating its attachment to a module;
 *     - validating positional/named argument rules;
 *     - validating argument names;
 *     - validating argument count/cardinality;
 *     - validating value types;
 *     - validating nested structures;
 *     - detecting prohibited duplicate keys;
 *     - resolving referenced names;
 *     - checking feature gates;
 *     - checking version requirements;
 *     - checking capability requirements;
 *     - checking resource requirements;
 *     - checking portability;
 *     - checking security policy;
 *     - checking domain-specific constraints.
 *
 * The grammar MUST NOT perform these operations.
 *
 * ============================================================================
 * ATTRIBUTE CATEGORIES
 * ============================================================================
 *
 * Attribute names may describe many kinds of semantic metadata, including:
 *
 *     metadata
 *     compiler hints
 *     optimization hints
 *     portability requirements
 *     resource requirements
 *     capability requirements
 *     effects
 *     security policy
 *     provenance
 *     verification metadata
 *     quantum intent
 *     HDL/hardware intent
 *     distributed execution intent
 *     interoperability metadata
 *     dialect metadata
 *     compatibility metadata
 *
 * The grammar MUST NOT enumerate those categories as a finite keyword list.
 *
 * The attribute namespace is intentionally extensible.
 *
 * ============================================================================
 * REQUIREMENT / CAPABILITY / PREFERENCE SEPARATION
 * ============================================================================
 *
 * Module attributes may carry semantic metadata such as:
 *
 *     @requires(...)
 *     @capability(...)
 *     @prefer(...)
 *     @constraint(...)
 *     @resource(...)
 *
 * But syntax alone does not determine the category's implementation.
 *
 * For example:
 *
 *     @requires(qubits >= n)
 *
 * if eventually admitted by the canonical attribute-value contract, expresses
 * semantic intent.
 *
 * It does NOT mean:
 *
 *     allocate physical qubits 0 through n - 1
 *
 * Likewise:
 *
 *     @prefer(accelerator = quantum)
 *
 * does not select a particular QPU.
 *
 * Target selection, routing, scheduling, resource allocation, and deployment
 * remain downstream.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Module attributes are target-independent.
 *
 * A module may contain code intended eventually for:
 *
 *     CPU
 *     multicore CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     accelerator
 *     simulator
 *     embedded system
 *     cluster
 *     HPC environment
 *     distributed system
 *     cloud environment
 *     future target
 *
 * The module attribute grammar does not require a separate module language for
 * any of those targets.
 *
 * This supports:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * by keeping source organization independent from target realization.
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This file MUST NOT encode universal constants for:
 *
 *     MAX_MODULES
 *     MAX_ATTRIBUTES
 *     MAX_ATTRIBUTE_ARGUMENTS
 *     MAX_ATTRIBUTE_NESTING
 *     MAX_NAMESPACE_DEPTH
 *     MAX_DEPENDENCIES
 *     MAX_PACKAGES
 *     MAX_TARGETS
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_ASICS
 *     MAX_QPUS
 *     MAX_QUBITS
 *     MAX_MEMORY
 *     MAX_STORAGE
 *     MAX_ACCELERATORS
 *     MAX_NODES
 *     MAX_NETWORK_LINKS
 *     MAX_TENSOR_DIMENSIONS
 *     MAX_VECTOR_WIDTH
 *
 * No physical machine property is a grammar limit.
 *
 * A program may legitimately contain a finite numeric value as program data.
 *
 * For example:
 *
 *     @configuration(batch = 1024)
 *
 * is not equivalent to declaring:
 *
 *     MAX_BATCH = 1024
 *
 * as a language-wide limitation.
 *
 * The distinction between program data and implementation limits is preserved.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Module attributes may annotate modules containing quantum code.
 *
 * For example, syntactically:
 *
 *     @quantum::logical
 *     module quantum::algorithms {
 *         ...
 *     }
 *
 * or:
 *
 *     @quantum::capability(
 *         measurement,
 *     )
 *     module quantum::runtime {
 *         ...
 *     }
 *
 * This grammar does not:
 *
 *     - enumerate quantum gates;
 *     - define qubit identifiers;
 *     - define physical qubits;
 *     - define QPU topology;
 *     - define routing;
 *     - define scheduling;
 *     - define calibration;
 *     - define QEC;
 *     - define ZQN;
 *     - construct quantum::ir.
 *
 * The downstream quantum pipeline remains:
 *
 *     source
 *       ->
 *     lexer
 *       ->
 *     parser
 *       ->
 *     domain-neutral AST
 *       ->
 *     semantic analysis
 *       ->
 *     quantum::ir
 *       ->
 *     optimization
 *       ->
 *     decomposition
 *       ->
 *     routing
 *       ->
 *     scheduling
 *       ->
 *     QEC / resilience / ZQN
 *       ->
 *     HAL
 *       ->
 *     target realization
 *
 * Module attributes merely provide source-level metadata at the module
 * boundary.
 *
 * ============================================================================
 * CLASSICAL / HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * The same module attribute syntax may annotate:
 *
 *     classical modules
 *     numerical modules
 *     AI/ML modules
 *     HDL modules
 *     hardware/software co-design modules
 *     accelerator modules
 *     embedded modules
 *     distributed modules
 *     networking modules
 *     security modules
 *     interoperability modules
 *
 * No domain-specific module attribute grammar is required merely because a
 * new computational domain is introduced.
 *
 * Domain-specific attribute schemas belong to their semantic owners.
 *
 * ============================================================================
 * RESOURCE AND CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Attributes can carry source-level metadata referring to:
 *
 *     capabilities
 *     resource requirements
 *     resource constraints
 *     preferences
 *     deployment requirements
 *     execution policies
 *
 * But the attribute grammar never discovers whether those resources exist.
 *
 * For example:
 *
 *     @requires(capability = quantum::measurement)
 *
 * is source syntax.
 *
 * Whether the eventual target provides that capability belongs to:
 *
 *     semantic analysis
 *     resource management
 *     compiler
 *     scheduler
 *     deployment
 *     HAL
 *     runtime
 *
 * ============================================================================
 * MODULE SYSTEM INTEGRATION
 * ============================================================================
 *
 * The module declaration grammar is owned by:
 *
 *     grammar/modules/modules.g4
 *
 * That file should consume:
 *
 *     moduleAttributes
 *
 * and then continue with module identity/body syntax.
 *
 * Conceptually:
 *
 *     moduleDeclaration
 *         : moduleAttributes?
 *           moduleHeader
 *           moduleBody
 *         ;
 *
 * The exact module declaration remains owned by modules.g4.
 *
 * This file does not define:
 *
 *     moduleDeclaration
 *
 * itself.
 *
 * ============================================================================
 * IMPORT / EXPORT INTEGRATION
 * ============================================================================
 *
 * Imports are owned by:
 *
 *     grammar/modules/imports.g4
 *
 * Exports are owned by:
 *
 *     grammar/modules/exports.g4
 *
 * Module attributes may semantically describe or constrain either, but this
 * grammar does not redefine import/export syntax.
 *
 * ============================================================================
 * VISIBILITY INTEGRATION
 * ============================================================================
 *
 * Visibility syntax is owned by:
 *
 *     grammar/modules/visibility.g4
 *
 * or the repository's canonical reusable visibility grammar.
 *
 * A module attribute MUST NOT introduce another visibility system.
 *
 * For example, an attribute such as:
 *
 *     @private
 *
 * if ever supported as a semantic convention, must not silently become a
 * second visibility grammar.
 *
 * The specification must decide whether such a construct is an attribute,
 * visibility modifier, or deprecated compatibility form.
 *
 * ============================================================================
 * NAMESPACE INTEGRATION
 * ============================================================================
 *
 * Attribute names use the canonical qualified-name system through:
 *
 *     core/attributes.g4
 *         ->
 *     core/names.g4
 *
 * Therefore:
 *
 *     @quantum::logical
 *
 * and:
 *
 *     @hardware::capability
 *
 * do not require a second qualified-name implementation.
 *
 * ============================================================================
 * PACKAGE / DEPENDENCY INTEGRATION
 * ============================================================================
 *
 * Module/package attributes may carry metadata concerning:
 *
 *     package identity
 *     version
 *     dependency requirements
 *     feature requirements
 *     compatibility
 *     provenance
 *
 * But:
 *
 *     grammar/modules/packages.g4
 *     grammar/modules/dependencies.g4
 *     grammar/modules/versioning.g4
 *
 * remain responsible for their respective syntax.
 *
 * Package managers and dependency resolvers remain responsible for:
 *
 *     registry access
 *     package retrieval
 *     dependency solving
 *     lock generation
 *     artifact verification
 *     installation
 *
 * Parsing an attribute never performs package resolution.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar should lower into the repository's canonical frontend AST
 * representation for attributes.
 *
 * It MUST NOT create a module-specific duplicate attribute representation if
 * the frontend already has a canonical attribute node.
 *
 * The conceptual structure is:
 *
 *     Attribute
 *         name
 *         arguments
 *         source_span
 *
 *     AttributeArgument
 *         positional | named
 *         value
 *         source_span
 *
 *     AttributeValue
 *         scalar
 *         name
 *         list
 *         map
 *         nested_attribute
 *         source_span
 *
 * The exact Rust type names are owned by the frontend AST implementation.
 *
 * This grammar must remain independent of Rust AST implementation details.
 *
 * ============================================================================
 * SOURCE-SPAN CONTRACT
 * ============================================================================
 *
 * The complete source span of:
 *
 *     @attribute
 *
 *     @attribute(...)
 *
 * including relevant nested arguments and values must remain recoverable
 * through the parser/AST pipeline.
 *
 * Source spans are required for:
 *
 *     diagnostics
 *     IDE tooling
 *     LSP
 *     formatting
 *     refactoring
 *     provenance
 *     compatibility diagnostics
 *     semantic errors
 *     generated-code mapping
 *
 * This grammar must not discard syntactic structure required for those uses.
 *
 * ============================================================================
 * SEMANTIC ERROR CONTRACT
 * ============================================================================
 *
 * The parser may reject malformed syntax such as:
 *
 *     @
 *     @(
 *     @name(
 *     @name(
 *         =
 *     )
 *
 * but it must not attempt to decide semantic validity such as:
 *
 *     unknown attribute
 *     duplicate semantic key
 *     unsupported attribute for module
 *     incompatible attribute version
 *     unavailable capability
 *     insufficient resources
 *     forbidden deployment policy
 *
 * Those belong downstream.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing is deterministic.
 *
 * This grammar contains:
 *
 *     - no actions;
 *     - no semantic predicates;
 *     - no callbacks;
 *     - no filesystem access;
 *     - no network access;
 *     - no environment inspection;
 *     - no hardware discovery;
 *     - no runtime execution;
 *     - no randomness;
 *     - no wall-clock dependency.
 *
 * Identical source/token streams under the same language grammar must produce
 * equivalent parse structures.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Attribute syntax is untrusted source input.
 *
 * This grammar performs no:
 *
 *     command execution
 *     filesystem access
 *     network access
 *     environment inspection
 *     credential access
 *     dynamic library loading
 *     hardware access
 *     runtime execution
 *
 * Attribute names such as:
 *
 *     @execute(...)
 *     @shell(...)
 *     @network(...)
 *
 * must remain inert syntax unless and until an explicitly authorized semantic
 * or tooling subsystem assigns meaning.
 *
 * Parsing an attribute is never authorization.
 *
 * ============================================================================
 * PERFORMANCE
 * ============================================================================
 *
 * This grammar uses the canonical generic attribute grammar rather than
 * duplicating nested alternatives.
 *
 * The implementation must avoid introducing artificial complexity based on
 * domain count or hardware scale.
 *
 * Repetition is expressed through grammar structure:
 *
 *     *
 *     +
 *
 * rather than finite enumeration.
 *
 * Any implementation-level limits required to prevent resource exhaustion
 * belong to parser/tooling resource policies and must not be presented as
 * Zamani language semantic limits.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * The following public rule names are retained:
 *
 *     moduleAttributes
 *     moduleAttribute
 *
 * Additional wrappers:
 *
 *     requiredModuleAttributes
 *     optionalModuleAttributes
 *     moduleAttributeDeclaration
 *
 * are provided as stable composition boundaries.
 *
 * The old specialized value-rule names are intentionally not retained because
 * they represented a duplicate attribute language and are not safe to expose
 * as independent canonical syntax.
 *
 * If downstream code currently consumes one of those obsolete rule names, it
 * should migrate to the canonical rules from:
 *
 *     grammar/core/attributes.g4
 *
 * rather than preserving a second implementation.
 *
 * ============================================================================
 * COMPATIBILITY WITH grammar/grammar.md
 * ============================================================================
 *
 * `grammar/grammar.md` remains the implementation-conformance reference.
 *
 * This file is not a competing language specification.
 *
 * The conformance status of module attributes must ultimately be reported
 * against:
 *
 *     specification
 *     canonical grammar
 *     lexer
 *     parser
 *     AST
 *     semantic analyzer
 *     compiler/tooling
 *
 * A syntax-only implementation is not sufficient to mark a feature fully
 * implemented.
 *
 * ============================================================================
 * RELATIONSHIP TO Zamani-Grammar.md
 * ============================================================================
 *
 * `grammar/Zamani-Grammar.md` may contain historical, proposed, experimental,
 * or extended attribute concepts.
 *
 * Those concepts do not automatically become legal syntax.
 *
 * Promotion remains:
 *
 *     proposal
 *       ->
 *     specification
 *       ->
 *     AST contract
 *       ->
 *     canonical grammar
 *       ->
 *     semantic implementation
 *       ->
 *     downstream integration
 *       ->
 *     conformance tests
 *       ->
 *     stable feature
 *
 * ============================================================================
 * RELATIONSHIP TO quantum::ir
 * ============================================================================
 *
 * This grammar has no direct dependency on `quantum::ir`.
 *
 * The dependency path is:
 *
 *     module attribute
 *         ->
 *     frontend AST
 *         ->
 *     semantic analysis
 *         ->
 *     appropriate semantic metadata
 *         ->
 *     quantum semantic representation when applicable
 *         ->
 *     quantum::ir
 *
 * This prevents the module grammar from creating a second quantum IR.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This file contains no Rust implementation.
 *
 * Its generated parser integration must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and safe Rust only.
 *
 * No `unsafe` implementation is required or authorized by this grammar.
 *
 * Rust-version compatibility is an implementation/tooling contract, not a
 * grammar-level language limit.
 *
 * ============================================================================
 * VALIDATION REQUIREMENTS
 * ============================================================================
 *
 * `grammar/validation/` should verify:
 *
 *     - ModuleAttributes has exactly one generic attribute dependency;
 *     - Attributes remains the canonical generic attribute owner;
 *     - no duplicate literal grammar exists here;
 *     - no duplicate name grammar exists here;
 *     - no duplicate map/list grammar exists here;
 *     - no duplicate nested-attribute grammar exists here;
 *     - moduleAttributes accepts zero or more canonical attributes;
 *     - moduleAttribute accepts exactly one canonical attribute;
 *     - requiredModuleAttributes requires at least one attribute;
 *     - optionalModuleAttributes remains structurally optional;
 *     - no machine-size limits exist;
 *     - no target-specific hardware limits exist;
 *     - no quantum gate enumeration exists;
 *     - no runtime actions exist;
 *     - no filesystem/network access exists;
 *     - no semantic predicates exist;
 *     - parser grammar dependency direction is acyclic;
 *     - token vocabulary is canonical;
 *     - source spans remain available downstream.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive tests MUST cover at least:
 *
 *     <no attributes>
 *     @attribute
 *     @attribute()
 *     @attribute(value)
 *     @attribute(name = value)
 *     @namespace::attribute
 *     @namespace::attribute(...)
 *     multiple attributes
 *     nested attributes
 *     list values
 *     map values
 *     qualified-name values
 *     string values
 *     integer values
 *     floating-point values
 *     boolean values
 *     null values
 *     trailing commas
 *     nested structured values
 *
 * Example:
 *
 *     @portable
 *     @quantum::logical
 *     @resource(
 *         capabilities = [
 *             quantum::measurement,
 *             tensor::compute,
 *         ],
 *     )
 *     module quantum::algorithms {
 *         ...
 *     }
 *
 * Negative tests MUST cover malformed forms such as:
 *
 *     @
 *     @(
 *     @attribute(
 *     @attribute(,)
 *     @attribute(name =)
 *     @attribute(= value)
 *     @attribute([,)
 *     @attribute({,)
 *
 * Boundary tests MUST cover:
 *
 *     empty attribute argument lists
 *     one attribute
 *     many attributes
 *     one argument
 *     many arguments
 *     deeply nested structural values
 *     long qualified names
 *     large structured collections
 *     duplicate map keys
 *
 * Duplicate semantic keys should remain parseable where the generic grammar
 * permits them so semantic analysis can issue the correct diagnostic rather
 * than silently losing information.
 *
 * ============================================================================
 * SCALABILITY TESTS
 * ============================================================================
 *
 * Scalability tests must verify that grammar correctness does not depend on
 * arbitrary constants for:
 *
 *     attribute count
 *     argument count
 *     list size
 *     map size
 *     namespace depth
 *     module count
 *     dependency count
 *     target count
 *     resource count
 *     quantum resource count
 *     hardware resource count
 *
 * The tests may be resource-bounded for CI practicality, but those CI bounds
 * MUST NOT become language semantics.
 *
 * ============================================================================
 * CROSS-DOMAIN TESTS
 * ============================================================================
 *
 * Module attributes must parse consistently when modules contain:
 *
 *     classical constructs
 *     quantum constructs
 *     hybrid constructs
 *     HDL constructs
 *     hardware/software co-design
 *     distributed constructs
 *     AI/ML constructs
 *     data constructs
 *     networking constructs
 *     security constructs
 *     interoperability constructs
 *     dialect constructs
 *     macro/metaprogramming constructs
 *
 * The module attribute grammar must not need domain-specific rewrites for
 * those cases.
 *
 * ============================================================================
 * DETERMINISM TESTS
 * ============================================================================
 *
 * For identical source and grammar version:
 *
 *     parse(source) == parse(source)
 *
 * modulo explicitly documented parser implementation details that do not
 * affect semantic structure.
 *
 * Tests must ensure parsing does not depend on:
 *
 *     machine architecture
 *     CPU count
 *     GPU availability
 *     QPU availability
 *     network state
 *     filesystem state
 *     environment variables
 *     wall-clock time
 *     random state
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file passes the hard-coding audit only if it contains no universal
 * constants or grammar branches representing:
 *
 *     maximum modules
 *     maximum attributes
 *     maximum arguments
 *     maximum namespace depth
 *     maximum dependency count
 *     maximum package count
 *     maximum hardware size
 *     maximum quantum size
 *     maximum CPU/GPU/FPGA/QPU count
 *     maximum memory
 *     maximum topology
 *     fixed accelerator count
 *     fixed physical device identifiers
 *
 * Numeric literals occurring as ordinary attribute values remain source data,
 * not language-wide limits.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is COMPLETE only when all of the following are true:
 *
 * [x] Module-specific attribute attachment is owned here.
 * [x] Generic attribute syntax is owned by core/attributes.g4.
 * [x] No duplicate attribute-value grammar exists here.
 * [x] Canonical ZamaniTokens vocabulary is consumed.
 * [x] Canonical qualified-name handling is inherited.
 * [x] Module attributes support zero-or-more attributes.
 * [x] A single moduleAttribute wrapper is available.
 * [x] Required and optional wrappers are available where useful.
 * [x] Structured values remain available through Attributes.
 * [x] Lists remain available through Attributes.
 * [x] Maps remain available through Attributes.
 * [x] Nested attributes remain available through Attributes.
 * [x] Trailing-comma behavior remains canonical.
 * [x] Attribute names remain extensible.
 * [x] No domain-specific attribute keyword list is hard-coded.
 * [x] No hardware limit is encoded.
 * [x] No quantum limit is encoded.
 * [x] No fixed module/attribute/argument count is encoded.
 * [x] No expression grammar is duplicated.
 * [x] No runtime behavior is embedded.
 * [x] No semantic resolution is embedded.
 * [x] No filesystem/network behavior is embedded.
 * [x] No target discovery is embedded.
 * [x] No quantum IR is embedded.
 * [x] quantum::ir remains downstream.
 * [x] AST preservation is defined.
 * [x] source-span preservation is defined.
 * [x] semantic ownership is defined.
 * [x] compiler integration is defined.
 * [x] runtime integration is defined.
 * [x] tooling integration is defined.
 * [x] cross-domain integration is defined.
 * [x] positive tests are defined.
 * [x] negative tests are defined.
 * [x] boundary tests are defined.
 * [x] scalability tests are defined.
 * [x] compatibility tests are defined.
 * [x] determinism tests are defined.
 * [x] hard-coding audit is defined.
 * [x] Rust 1.97 / 1.97.1 compatibility is defined.
 * [x] Safe-Rust-only implementation is defined.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * This file owns ONE thing:
 *
 *     module-specific attachment of canonical attributes.
 *
 * It does not create another attribute language.
 *
 * It does not create another module language.
 *
 * It does not create another name system.
 *
 * It does not create another semantic model.
 *
 * It does not create another IR.
 *
 * It does not encode hardware limits.
 *
 * It does not encode quantum limits.
 *
 * It does not select targets.
 *
 * It does not execute anything.
 *
 * The resulting dependency is:
 *
 *     canonical attributes
 *          |
 *          v
 *     module attribute attachment
 *          |
 *          v
 *     module declaration
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic validation
 *          |
 *          v
 *     compiler / semantic IR
 *          |
 *          +-----------------------------+
 *          |                             |
 *          v                             v
 *     classical semantics          quantum::ir / HDL
 *          |                             |
 *          +-------------+---------------+
 *                        |
 *                        v
 *              optimization / lowering
 *                        |
 *              routing / scheduling /
 *              resilience / QEC / ZQN
 *                        |
 *                        v
 *                       HAL
 *                        |
 *                        v
 *                 target realization
 *
 * This preserves the Zamani POCO-REAF architecture:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * while allowing the available implementation resources to determine actual
 * scale and target realization downstream.
 *
 * ============================================================================
 */
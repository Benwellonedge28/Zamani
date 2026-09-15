/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/dialects/capabilities.g4
 *
 * Role:
 *     Dialect-scoped capability grammar adapter.
 *
 * Status:
 *     Production-ready architectural component.
 *
 * Baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     ANTLR4
 *     safe Rust only
 *     no unsafe Rust
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *                         SOURCE
 *                           |
 *                           v
 *                      ZamaniLexer
 *                           |
 *                           v
 *                     parser grammar
 *                           |
 *              +------------+------------+
 *              |                         |
 *              v                         v
 *       core::Names             core::Capabilities
 *              |                         |
 *              +------------+------------+
 *                           |
 *                           v
 *                dialects::Capabilities
 *                           |
 *                           v
 *                       AST
 *                           |
 *                           v
 *                 semantic analysis
 *                           |
 *          +----------------+----------------+
 *          |                |                |
 *          v                v                v
 *     capability        requirement       compatibility
 *      registry          analysis           analysis
 *          |
 *          v
 *       canonical semantic model
 *          |
 *     +----+----+----+----+----+
 *     |         |         |     |
 *     v         v         v     v
 * classical  quantum     HDL  hardware
 *    IR       ::ir       IR   semantic model
 *
 * ============================================================================
 * FUNDAMENTAL OWNERSHIP RULE
 * ============================================================================
 *
 * grammar/core/capabilities.g4 is the CANONICAL capability syntax owner.
 *
 * This file MUST NOT create another capability language.
 *
 * Core capability grammar owns:
 *
 *     - capability declarations;
 *     - capability references;
 *     - capability names;
 *     - capability version syntax;
 *     - capability version constraints;
 *     - capability ranges;
 *     - capability sets;
 *     - capability attributes;
 *     - generic capability lists.
 *
 * This file owns ONLY:
 *
 *     - dialect-scoped capability composition;
 *     - dialect capability declarations as a contextual adapter;
 *     - dialect capability references;
 *     - dialect capability requirement lists;
 *     - dialect capability composition;
 *     - dialect capability inheritance references;
 *     - dialect capability contract grouping.
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     - identifiers;
 *     - qualified-name syntax;
 *     - lexical keywords;
 *     - lexical literals;
 *     - generic capability syntax;
 *     - version comparison;
 *     - version solving;
 *     - capability registry implementation;
 *     - capability discovery;
 *     - hardware discovery;
 *     - target selection;
 *     - resource allocation;
 *     - resource discovery;
 *     - scheduling;
 *     - routing;
 *     - optimization;
 *     - QEC;
 *     - ZQN;
 *     - resilience;
 *     - simulation;
 *     - runtime execution;
 *     - quantum::ir;
 *     - classical IR;
 *     - HDL IR;
 *     - hardware implementation;
 *     - vendor implementation code.
 *
 * ============================================================================
 * CORE CAPABILITY BOUNDARY
 * ============================================================================
 *
 * The canonical capability grammar is:
 *
 *     grammar/core/capabilities.g4
 *
 * It owns rules including:
 *
 *     capabilityDeclaration
 *     capabilityReference
 *     capabilityName
 *     capabilityVersionClause
 *     capabilityVersionExpression
 *     capabilityVersionConstraint
 *     capabilityVersionRange
 *     capabilityVersionSet
 *     capabilityVersionReference
 *     capabilityAttributeList
 *
 * This grammar consumes those canonical rules.
 *
 * It MUST NOT redefine them.
 *
 * ============================================================================
 * NAME BOUNDARY
 * ============================================================================
 *
 * Canonical names are owned by:
 *
 *     grammar/core/names.g4
 *
 * In particular:
 *
 *     identifier
 *     qualifiedName
 *
 * are not reimplemented here.
 *
 * A capability identity such as:
 *
 *     zamani::quantum::dynamic_control
 *
 * is therefore structurally just a canonical qualified name.
 *
 * The dialect grammar does not decide what the namespace means.
 *
 * ============================================================================
 * VERSION BOUNDARY
 * ============================================================================
 *
 * Version syntax is owned by:
 *
 *     grammar/core/versioning.g4
 *
 * Capability version syntax is adapted by:
 *
 *     grammar/core/capabilities.g4
 *
 * This file does not define:
 *
 *     major versions
 *     minor versions
 *     patch versions
 *     ranges
 *     comparison semantics
 *     compatibility algorithms
 *
 * A dialect may therefore state:
 *
 *     capability quantum::dynamic_control version >= 1.0.0;
 *
 * without this file determining whether the requirement is satisfiable.
 *
 * ============================================================================
 * DIALECT BOUNDARY
 * ============================================================================
 *
 * A dialect is a source-level semantic extension contract.
 *
 * A capability inside a dialect means:
 *
 *     "This dialect declares, requires, exposes, or composes this
 *      computational capability."
 *
 * It does NOT mean:
 *
 *     "Use this physical machine."
 *
 * It does NOT select:
 *
 *     - CPU;
 *     - GPU;
 *     - FPGA;
 *     - ASIC;
 *     - QPU;
 *     - device;
 *     - physical qubit;
 *     - memory bank;
 *     - network endpoint;
 *     - topology;
 *     - scheduler;
 *     - backend.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Capability syntax must remain independent of machine scale.
 *
 * This file therefore contains no:
 *
 *     MAX_CAPABILITIES
 *     MAX_DIALECT_CAPABILITIES
 *     MAX_DEVICES
 *     MAX_QUBITS
 *     MAX_CORES
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *
 * There is no fixed limit on:
 *
 *     - capability declarations;
 *     - capability references;
 *     - dialects;
 *     - dialect composition;
 *     - requirement expressions;
 *     - namespace depth;
 *     - source size.
 *
 * Practical limits belong to compiler/resource policy.
 *
 * ============================================================================
 * OPEN-WORLD CAPABILITY MODEL
 * ============================================================================
 *
 * Capability names are deliberately open-ended.
 *
 * Examples:
 *
 *     zamani::classical::parallel
 *     zamani::quantum::dynamic_control
 *     zamani::quantum::mid_circuit_measurement
 *     zamani::hardware::reconfigurable
 *     zamani::hdl::sequential_logic
 *     zamani::ai::tensor_compute
 *     zamani::distributed::replication
 *     future::computing::unknown
 *     organization::research::experimental
 *
 * Adding a new capability MUST NOT require modifying this grammar.
 *
 * ============================================================================
 * DIALECT CAPABILITY DECLARATION
 * ============================================================================
 *
 * Canonical contextual forms include:
 *
 *     capability quantum::dynamic_control;
 *
 *     capability quantum::dynamic_control version 1.0.0;
 *
 *     capability quantum::dynamic_control version >= 1.0.0;
 *
 * The exact capability declaration syntax is inherited from
 * core/capabilities.g4.
 *
 * This file merely establishes the dialect context.
 *
 * ============================================================================
 * CAPABILITY REFERENCE
 * ============================================================================
 *
 * A reference is not a declaration.
 *
 * Example:
 *
 *     quantum::dynamic_control
 *
 * may be referenced by a dialect requirement:
 *
 *     requires quantum::dynamic_control;
 *
 * ============================================================================
 * REQUIREMENT / CAPABILITY SEPARATION
 * ============================================================================
 *
 * Capability:
 *
 *     "What semantic facility exists?"
 *
 * Requirement:
 *
 *     "What facility must exist?"
 *
 * Resource:
 *
 *     "What physical/logical resource is available or requested?"
 *
 * Constraint:
 *
 *     "What conditions must a realization satisfy?"
 *
 * Preference:
 *
 *     "Which valid realization is preferred?"
 *
 * Target:
 *
 *     "What target context is being described?"
 *
 * These concepts MUST NOT be collapsed.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * A quantum capability such as:
 *
 *     quantum::dynamic_control
 *
 * remains a capability identity.
 *
 * It does not create:
 *
 *     QubitId
 *     PhysicalQubitId
 *     GateKind
 *     QuantumOperation
 *     QuantumCircuit
 *     topology
 *     calibration
 *     pulse schedule
 *
 * Quantum semantics are lowered downstream.
 *
 * The canonical quantum semantic boundary remains:
 *
 *     quantum::ir
 *
 * ============================================================================
 * HARDWARE INDEPENDENCE
 * ============================================================================
 *
 * This file MUST NOT encode:
 *
 *     device names;
 *     device IDs;
 *     processor models;
 *     qubit counts;
 *     core counts;
 *     GPU counts;
 *     FPGA counts;
 *     ASIC identifiers;
 *     topology;
 *     memory capacity;
 *     physical addresses.
 *
 * Such information belongs to:
 *
 *     hardware/
 *     resources/
 *     compile/
 *     execution/
 *     runtime
 *
 * as appropriate.
 *
 * ============================================================================
 * SEMANTIC VALIDATION
 * ============================================================================
 *
 * This grammar establishes syntax only.
 *
 * Semantic analysis determines:
 *
 *     - whether a capability exists;
 *     - whether the capability is declared;
 *     - whether a capability is visible;
 *     - whether aliases resolve;
 *     - whether a capability version is compatible;
 *     - whether requirements are satisfiable;
 *     - whether capabilities conflict;
 *     - whether capabilities can be composed;
 *     - whether a capability applies to the enclosing dialect;
 *     - whether the execution environment provides the requirement.
 *
 * ============================================================================
 * DIALECT CAPABILITY KINDS
 * ============================================================================
 *
 * This grammar deliberately does not create a closed enumeration such as:
 *
 *     QuantumCapability
 *     CpuCapability
 *     GpuCapability
 *     FpgaCapability
 *     HdlCapability
 *
 * All such distinctions are semantic namespace/domain information.
 *
 * ============================================================================
 * CAPABILITY GROUPS
 * ============================================================================
 *
 * Dialects may group capabilities into an explicit contract:
 *
 *     capabilities {
 *         capability quantum::dynamic_control;
 *         capability quantum::measurement;
 *         capability quantum::reset;
 *     }
 *
 * The group is syntactic organization only.
 *
 * It does not create a new capability namespace.
 *
 * ============================================================================
 * CAPABILITY EXPORT
 * ============================================================================
 *
 * A dialect may expose capabilities to consumers.
 *
 * Export semantics belong to the dialect/module semantic layer.
 *
 * This grammar only preserves the syntactic distinction between:
 *
 *     capability declaration
 *
 * and:
 *
 *     capability reference.
 *
 * ============================================================================
 * CAPABILITY REQUIREMENTS
 * ============================================================================
 *
 * A dialect can require capabilities without selecting implementations.
 *
 * Example:
 *
 *     requires quantum::dynamic_control;
 *
 * The requirement may later be satisfied by:
 *
 *     a simulator
 *     a QPU
 *     a future quantum system
 *     a heterogeneous execution environment
 *
 * without changing source semantics.
 *
 * ============================================================================
 * CAPABILITY COMPOSITION
 * ============================================================================
 *
 * A dialect may compose capabilities through references.
 *
 * Example:
 *
 *     requires zamani::quantum::measurement;
 *     requires zamani::classical::control;
 *
 * Composition is semantic.
 *
 * The grammar does not evaluate the conjunction.
 *
 * ============================================================================
 * CAPABILITY CONTRACT BLOCK
 * ============================================================================
 *
 * A capability contract block provides a scalable syntactic container.
 *
 * Example:
 *
 *     capabilities {
 *         capability zamani::quantum::dynamic_control;
 *         capability zamani::quantum::measurement;
 *         capability zamani::quantum::reset;
 *     }
 *
 * No finite number of entries is allowed by design.
 *
 * ============================================================================
 * ATTRIBUTE BOUNDARY
 * ============================================================================
 *
 * Capability attributes are inherited from the canonical capability grammar.
 *
 * This file MUST NOT redefine:
 *
 *     @attribute
 *
 * syntax.
 *
 * ============================================================================
 * ANTLR SAFETY
 * ============================================================================
 *
 * This is a declarative ANTLR parser grammar.
 *
 * It contains:
 *
 *     - no embedded Rust;
 *     - no semantic predicates;
 *     - no actions;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware access;
 *     - no runtime calls;
 *     - no randomness.
 *
 * The generated parser implementation is therefore compatible with the
 * project's safe-Rust requirement.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Identical token streams must receive identical syntactic treatment.
 *
 * Semantic capability resolution is deliberately outside this grammar.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend must preserve:
 *
 *     - source span;
 *     - capability identity;
 *     - version constraint;
 *     - declaration/reference distinction;
 *     - enclosing dialect relationship;
 *     - capability grouping;
 *     - source ordering;
 *     - attribute structure.
 *
 * The AST MUST NOT resolve:
 *
 *     - hardware;
 *     - target;
 *     - resource;
 *     - backend;
 *     - runtime;
 *     - device;
 *     - capability implementation.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar produces no IR.
 *
 * Capability semantics may later become metadata on the canonical semantic
 * representation.
 *
 * This grammar MUST NOT create:
 *
 *     Quantum IR
 *     Classical IR
 *     HDL IR
 *     Hardware IR
 *     Runtime state
 *     Resource state
 *
 * ============================================================================
 * COMPILER CONTRACT
 * ============================================================================
 *
 * Compiler stages consume the semantic capability model after parsing.
 *
 * The dependency direction is:
 *
 *     grammar
 *       ->
 *     AST
 *       ->
 *     semantic capability model
 *       ->
 *     capability resolution
 *       ->
 *     resource/target analysis
 *       ->
 *     lowering
 *       ->
 *     optimization/routing/scheduling
 *       ->
 *     execution
 *
 * The compiler MUST NOT make this grammar depend on later stages.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime capability discovery is not performed by this grammar.
 *
 * Runtime state can be compared against semantic capability requirements
 * after compilation.
 *
 * ============================================================================
 * TOOLING CONTRACT
 * ============================================================================
 *
 * Tooling may use the parse tree to provide:
 *
 *     - capability completion;
 *     - navigation;
 *     - diagnostics;
 *     - documentation lookup;
 *     - refactoring;
 *     - formatting;
 *     - capability visualization.
 *
 * Tooling MUST resolve capability meaning through the semantic registry,
 * rather than hard-coding capability names in this grammar.
 *
 * ============================================================================
 * CROSS-DOMAIN CONTRACT
 * ============================================================================
 *
 * The same dialect capability mechanism must be usable by:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     hardware
 *     distributed
 *     AI
 *     data
 *     networking
 *     security
 *     accelerators
 *     future domains
 *
 * A new domain MUST NOT require changing this grammar merely to introduce
 * new capability identities.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing dialect syntax from dialects/dialects.g4 remains conceptually
 * supported:
 *
 *     capability <qualified-name> ... ;
 *
 * This file replaces the duplicated capability implementation in that
 * grammar with a canonical adapter around core/capabilities.g4.
 *
 * ============================================================================
 */

parser grammar DialectCapabilities;

options {
    tokenVocab = ZamaniLexer;
}

import Names, Capabilities;


/*
 * ============================================================================
 * 1. DIALECT CAPABILITY DECLARATION
 * ============================================================================
 *
 * The canonical declaration syntax remains owned by core/capabilities.g4.
 *
 * This wrapper establishes dialect-specific parser ownership without
 * redefining capability identity, versioning, or attributes.
 *
 * Example:
 *
 *     capability quantum::dynamic_control;
 *
 *     capability quantum::dynamic_control version 1.0.0;
 */
dialectCapabilityDeclaration
    : capabilityDeclaration
    ;


/*
 * ============================================================================
 * 2. DIALECT CAPABILITY REFERENCE
 * ============================================================================
 *
 * This is deliberately a wrapper around the canonical reference rule.
 */
dialectCapabilityReference
    : capabilityReference
    ;


/*
 * ============================================================================
 * 3. DIALECT CAPABILITY NAME
 * ============================================================================
 *
 * Capability identity remains owned by core/capabilities.g4.
 */
dialectCapabilityName
    : capabilityName
    ;


/*
 * ============================================================================
 * 4. DIALECT CAPABILITY VERSION
 * ============================================================================
 *
 * Version interpretation remains outside this grammar.
 */
dialectCapabilityVersion
    : capabilityVersionClause
    ;


/*
 * ============================================================================
 * 5. DIALECT CAPABILITY ATTRIBUTE
 * ============================================================================
 *
 * Attributes remain owned by the canonical capability grammar.
 *
 * This wrapper exists so dialect consumers have a stable contextual rule
 * without redefining attribute syntax.
 */
dialectCapabilityAttributes
    : capabilityAttributeList
    ;


/*
 * ============================================================================
 * 6. DIALECT CAPABILITY LIST
 * ============================================================================
 *
 * One or more capability references.
 *
 * There is intentionally no finite maximum.
 */
dialectCapabilityList
    : dialectCapabilityReference
      (COMMA dialectCapabilityReference)*
    ;


/*
 * ============================================================================
 * 7. OPTIONAL DIALECT CAPABILITY LIST
 * ============================================================================
 */
optionalDialectCapabilityList
    : dialectCapabilityList?
    ;


/*
 * ============================================================================
 * 8. DIALECT CAPABILITY DECLARATION LIST
 * ============================================================================
 *
 * No finite number of declarations is encoded.
 */
dialectCapabilityDeclarationList
    : dialectCapabilityDeclaration+
    ;


/*
 * ============================================================================
 * 9. OPTIONAL DIALECT CAPABILITY DECLARATION LIST
 * ============================================================================
 */
optionalDialectCapabilityDeclarationList
    : dialectCapabilityDeclaration*
    ;


/*
 * ============================================================================
 * 10. DIALECT CAPABILITY REQUIREMENT
 * ============================================================================
 *
 * A requirement refers to a capability.
 *
 * It does not select an implementation.
 *
 * Example:
 *
 *     requires quantum::dynamic_control;
 *
 * The `requires` declaration itself remains owned by the dialect grammar.
 *
 * This rule supplies the capability-specific operand.
 */
dialectCapabilityRequirement
    : dialectCapabilityReference
    ;


/*
 * ============================================================================
 * 11. DIALECT CAPABILITY REQUIREMENT LIST
 * ============================================================================
 */
dialectCapabilityRequirementList
    : dialectCapabilityRequirement
      (COMMA dialectCapabilityRequirement)*
    ;


/*
 * ============================================================================
 * 12. OPTIONAL DIALECT CAPABILITY REQUIREMENT LIST
 * ============================================================================
 */
optionalDialectCapabilityRequirementList
    : dialectCapabilityRequirementList?
    ;


/*
 * ============================================================================
 * 13. DIALECT CAPABILITY GROUP
 * ============================================================================
 *
 * Example:
 *
 *     capabilities {
 *         capability quantum::dynamic_control;
 *         capability quantum::measurement;
 *         capability quantum::reset;
 *     }
 *
 * The group does not create a new semantic capability namespace.
 */
dialectCapabilityGroup
    : CAPABILITIES
      LBRACE
      dialectCapabilityGroupMember*
      RBRACE
    ;


/*
 * ============================================================================
 * 14. DIALECT CAPABILITY GROUP MEMBER
 * ============================================================================
 *
 * A group may contain declarations or references.
 *
 * Interpretation belongs to semantic analysis.
 */
dialectCapabilityGroupMember
    : dialectCapabilityDeclaration
    | dialectCapabilityReference SEMI
    ;


/*
 * ============================================================================
 * 15. DIALECT CAPABILITY GROUP LIST
 * ============================================================================
 *
 * Useful to higher-level dialect composition grammars.
 */
dialectCapabilityGroupList
    : dialectCapabilityGroup+
    ;


/*
 * ============================================================================
 * 16. DIALECT CAPABILITY CONTRACT
 * ============================================================================
 *
 * A dialect capability contract is a structural composition of:
 *
 *     declarations
 *     requirements
 *     groups
 *
 * It remains syntactic.
 */
dialectCapabilityContract
    : dialectCapabilityContractMember*
    ;


/*
 * ============================================================================
 * 17. DIALECT CAPABILITY CONTRACT MEMBER
 * ============================================================================
 */
dialectCapabilityContractMember
    : dialectCapabilityDeclaration
    | dialectCapabilityGroup
    ;


/*
 * ============================================================================
 * 18. DIALECT CAPABILITY REFERENCE LIST
 * ============================================================================
 *
 * Named separately from generic capabilityReferenceList so semantic tooling
 * can identify the dialect context without changing the underlying identity.
 */
dialectCapabilityReferenceList
    : dialectCapabilityReference
      (COMMA dialectCapabilityReference)*
    ;


/*
 * ============================================================================
 * 19. OPTIONAL DIALECT CAPABILITY REFERENCE LIST
 * ============================================================================
 */
optionalDialectCapabilityReferenceList
    : dialectCapabilityReferenceList?
    ;


/*
 * ============================================================================
 * 20. DIALECT CAPABILITY SET
 * ============================================================================
 *
 * This is a syntactic container only.
 *
 * The semantic layer decides:
 *
 *     - conjunction;
 *     - compatibility;
 *     - conflicts;
 *     - satisfiability.
 */
dialectCapabilitySet
    : dialectCapabilityReference
      (AND dialectCapabilityReference)*
    ;


/*
 * ============================================================================
 * 21. DIALECT CAPABILITY ALTERNATIVE SET
 * ============================================================================
 *
 * Example:
 *
 *     quantum::simulator OR quantum::hardware
 *
 * This does not select a backend.
 */
dialectCapabilityAlternativeSet
    : dialectCapabilitySet
      (OR dialectCapabilitySet)*
    ;


/*
 * ============================================================================
 * 22. DIALECT CAPABILITY EXPRESSION
 * ============================================================================
 *
 * Parentheses preserve explicit grouping.
 */
dialectCapabilityExpression
    : dialectCapabilityAlternativeSet
    | LPAREN dialectCapabilityExpression RPAREN
    | NOT dialectCapabilityExpression
    ;


/*
 * ============================================================================
 * 23. DIALECT CAPABILITY PREDICATE
 * ============================================================================
 *
 * This rule deliberately remains structural.
 *
 * No capability is evaluated here.
 */
dialectCapabilityPredicate
    : dialectCapabilityExpression
    ;


/*
 * ============================================================================
 * 24. DIALECT CAPABILITY REQUIREMENT EXPRESSION
 * ============================================================================
 */
dialectCapabilityRequirementExpression
    : dialectCapabilityPredicate
    ;


/*
 * ============================================================================
 * 25. DIALECT CAPABILITY REQUIREMENT ENTRY
 * ============================================================================
 *
 * This rule intentionally does not consume `REQUIRES`.
 *
 * The enclosing dialect requirement grammar owns the declaration keyword.
 */
dialectCapabilityRequirementEntry
    : dialectCapabilityRequirementExpression
    ;


/*
 * ============================================================================
 * 26. DIALECT CAPABILITY REQUIREMENT ENTRY LIST
 * ============================================================================
 */
dialectCapabilityRequirementEntryList
    : dialectCapabilityRequirementEntry
      (COMMA dialectCapabilityRequirementEntry)*
    ;


/*
 * ============================================================================
 * 27. DIALECT CAPABILITY CONTRACT LIST
 * ============================================================================
 */
dialectCapabilityContractList
    : dialectCapabilityContract+
    ;


/*
 * ============================================================================
 * 28. DIALECT CAPABILITY DECLARATION OR REFERENCE
 * ============================================================================
 *
 * Useful for syntax where a dialect may accept either a declaration or a
 * reference depending on the surrounding construct.
 */
dialectCapabilityDeclarationOrReference
    : dialectCapabilityDeclaration
    | dialectCapabilityReference
    ;


/*
 * ============================================================================
 * 29. DIALECT CAPABILITY DECLARATION OR REFERENCE LIST
 * ============================================================================
 */
dialectCapabilityDeclarationOrReferenceList
    : dialectCapabilityDeclarationOrReference
      (COMMA dialectCapabilityDeclarationOrReference)*
    ;


/*
 * ============================================================================
 * 30. DIALECT CAPABILITY IMPORT REFERENCE
 * ============================================================================
 *
 * Import resolution belongs to modules/dialect registry semantics.
 *
 * This rule only provides a qualified capability reference.
 */
dialectCapabilityImportReference
    : dialectCapabilityReference
    ;


/*
 * ============================================================================
 * 31. DIALECT CAPABILITY EXPORT REFERENCE
 * ============================================================================
 *
 * Export semantics are outside this grammar.
 */
dialectCapabilityExportReference
    : dialectCapabilityReference
    ;


/*
 * ============================================================================
 * 32. DIALECT CAPABILITY INHERITANCE REFERENCE
 * ============================================================================
 *
 * Inheritance semantics are owned by dialect semantic analysis.
 */
dialectCapabilityInheritanceReference
    : dialectCapabilityReference
    ;


/*
 * ============================================================================
 * 33. DIALECT CAPABILITY INHERITANCE LIST
 * ============================================================================
 */
dialectCapabilityInheritanceList
    : dialectCapabilityInheritanceReference
      (COMMA dialectCapabilityInheritanceReference)*
    ;


/*
 * ============================================================================
 * 34. OPTIONAL DIALECT CAPABILITY INHERITANCE LIST
 * ============================================================================
 */
optionalDialectCapabilityInheritanceList
    : dialectCapabilityInheritanceList?
    ;


/*
 * ============================================================================
 * 35. DIALECT CAPABILITY CONTRACT ENTRY
 * ============================================================================
 *
 * Stable contextual entry point for higher-level dialect grammars.
 */
dialectCapabilityContractEntry
    : dialectCapabilityDeclaration
    | dialectCapabilityGroup
    ;


/*
 * ============================================================================
 * 36. DIALECT CAPABILITY CONTRACT ENTRY LIST
 * ============================================================================
 */
dialectCapabilityContractEntryList
    : dialectCapabilityContractEntry+
    ;
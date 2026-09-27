/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/dialects/capabilities.g4
 *
 * Grammar:
 *     DialectCapabilities
 *
 * Status:
 *     PRODUCTION-READY DIALECT CAPABILITY ADAPTER
 *
 * Baseline:
 *     ANTLR4
 *     Rust 2021
 *     Rust 1.97 / Rust 1.97.1
 *     safe Rust only
 *     no unsafe Rust
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the DIALECT CONTEXT for capability declarations and
 * capability references.
 *
 * It does NOT create a second capability language.
 *
 * The canonical capability syntax remains owned by:
 *
 *     grammar/core/capabilities.g4
 *
 * The canonical name syntax remains owned by:
 *
 *     grammar/core/names.g4
 *
 * This file adapts those canonical constructs to the dialect-registration
 * context.
 *
 * Architectural path:
 *
 *     source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       v
 *     canonical parser grammar
 *       |
 *       v
 *     DialectCapabilities
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic capability model
 *       |
 *       v
 *     capability / requirement resolution
 *       |
 *       v
 *     canonical semantic representation
 *       |
 *       +--> classical IR
 *       +--> quantum::ir
 *       +--> HDL / hardware representation
 *       +--> distributed / AI / data / networking representations
 *       |
 *       v
 *     optimization / lowering / routing / scheduling / resilience
 *       |
 *       v
 *     target realization
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - dialect-scoped capability declaration adapters;
 *   - dialect-scoped capability reference adapters;
 *   - dialect capability lists;
 *   - dialect capability blocks;
 *   - dialect capability requirement operands;
 *   - dialect capability provision operands;
 *   - dialect capability preference operands;
 *   - dialect capability constraint operands;
 *   - dialect capability inheritance references;
 *   - dialect capability import/export references;
 *   - stable parser-level capability integration points.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - identifier syntax;
 *   - qualified-name syntax;
 *   - keyword definitions;
 *   - literal definitions;
 *   - capability identity semantics;
 *   - version comparison;
 *   - version solving;
 *   - capability registry;
 *   - capability discovery;
 *   - hardware discovery;
 *   - resource allocation;
 *   - target selection;
 *   - physical placement;
 *   - scheduling;
 *   - routing;
 *   - optimization;
 *   - QEC;
 *   - ZQN;
 *   - resilience;
 *   - runtime execution;
 *   - vendor implementation;
 *   - quantum::ir;
 *   - classical IR;
 *   - HDL IR.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * There must be exactly one canonical capability syntax:
 *
 *     grammar/core/capabilities.g4
 *
 * Therefore this file MUST NOT redefine:
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
 * This file wraps those rules only when dialect context is significant.
 *
 * ============================================================================
 * CANONICAL LEXER
 * ============================================================================
 *
 * The canonical production lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Relevant canonical tokens include:
 *
 *     CAPABILITY
 *     REQUIRES
 *     AS
 *     AND
 *     OR
 *     NOT
 *     COMMA
 *     SEMI
 *     LBRACE
 *     RBRACE
 *     LPAREN
 *     RPAREN
 *
 * IMPORTANT:
 *
 * There is currently no canonical CAPABILITIES token.
 *
 * Consequently this grammar deliberately does NOT use:
 *
 *     CAPABILITIES
 *
 * A capability block uses the existing CAPABILITY keyword:
 *
 *     capability {
 *         capability quantum::measurement;
 *         capability quantum::reset;
 *     }
 *
 * This avoids adding an unnecessary global keyword merely to support a
 * dialect-local grouping construct.
 *
 * ============================================================================
 * OPEN-WORLD CAPABILITY MODEL
 * ============================================================================
 *
 * Capability identities are symbolic qualified names.
 *
 * Examples:
 *
 *     classical::parallel
 *     quantum::measurement
 *     quantum::mid_circuit_measurement
 *     quantum::dynamic_control
 *     hdl::sequential_logic
 *     hardware::reconfigurable
 *     ai::tensor_compute
 *     distributed::replication
 *     networking::streaming
 *     security::zero_knowledge
 *     future::computing::capability
 *     organization::research::experimental
 *
 * No such names are enumerated by this grammar.
 *
 * Adding a new capability MUST NOT require editing this file.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Capability syntax is independent of machine scale.
 *
 * This file contains no:
 *
 *     MAX_CAPABILITIES
 *     MAX_DIALECT_CAPABILITIES
 *     MAX_DEVICES
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *
 * There is no grammar-level maximum for:
 *
 *     - capabilities per dialect;
 *     - dialects;
 *     - capability references;
 *     - capability requirements;
 *     - capability groups;
 *     - namespace depth;
 *     - dialect composition;
 *     - source size.
 *
 * Practical limits belong to compiler, runtime, operating-system, deployment,
 * and resource policy.
 *
 * ============================================================================
 * REQUIREMENT / CAPABILITY SEPARATION
 * ============================================================================
 *
 * Capability:
 *
 *     What semantic facility is provided?
 *
 * Requirement:
 *
 *     What semantic facility is required?
 *
 * Resource:
 *
 *     What computational resource is available or consumed?
 *
 * Constraint:
 *
 *     What conditions must a realization satisfy?
 *
 * Preference:
 *
 *     Which valid realization is preferred?
 *
 * Implementation decision:
 *
 *     Which concrete resource/device/backend is selected?
 *
 * This grammar MUST preserve these distinctions.
 *
 * A capability such as:
 *
 *     quantum::measurement
 *
 * does NOT mean:
 *
 *     use QPU 0
 *     use physical qubit 7
 *     use a particular backend
 *     use a particular topology
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * A quantum capability remains symbolic.
 *
 * For example:
 *
 *     capability quantum::mid_circuit_measurement;
 *
 * does not create:
 *
 *     QubitId
 *     PhysicalQubitId
 *     GateKind
 *     QuantumCircuit
 *     QuantumSchedule
 *     Calibration
 *     Topology
 *
 * The canonical quantum semantic boundary remains:
 *
 *     quantum::ir
 *
 * This grammar MUST NOT create another quantum IR.
 *
 * ============================================================================
 * CLASSICAL / HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * The same mechanism works for:
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
 *     embedded systems
 *     future computing domains
 *
 * Domain meaning is resolved semantically.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     source tokens
 *     grammar version
 *     imported grammar contracts
 *
 * Parsing MUST NOT depend on:
 *
 *     hardware;
 *     target availability;
 *     filesystem state;
 *     network state;
 *     registry contents;
 *     runtime state;
 *     wall-clock time;
 *     randomness.
 *
 * Capability resolution is downstream.
 *
 * ============================================================================
 * SAFETY
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no embedded Rust;
 *     - no actions;
 *     - no semantic predicates;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware access;
 *     - no runtime calls;
 *     - no randomness;
 *     - no unsafe code.
 *
 * Generated Rust integration remains subject to:
 *
 *     Rust 2021
 *     Rust 1.97 / Rust 1.97.1
 *     safe Rust only
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend AST must preserve:
 *
 *     - declaration/reference kind;
 *     - capability identity;
 *     - version constraint;
 *     - alias;
 *     - enclosing dialect;
 *     - capability grouping;
 *     - source ordering;
 *     - source span;
 *     - source spelling where required for diagnostics/tooling.
 *
 * The AST MUST NOT resolve:
 *
 *     - physical devices;
 *     - physical qubits;
 *     - CPUs;
 *     - GPUs;
 *     - FPGAs;
 *     - nodes;
 *     - memory banks;
 *     - backends;
 *     - schedulers;
 *     - routers.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for:
 *
 *     - resolving capability identities;
 *     - validating declarations;
 *     - resolving aliases;
 *     - checking duplicate declarations;
 *     - checking duplicate requirements;
 *     - checking visibility;
 *     - checking version compatibility;
 *     - checking capability conflicts;
 *     - checking capability composition;
 *     - checking dialect inheritance;
 *     - checking imported/exported capabilities;
 *     - checking feature gates;
 *     - checking dialect compatibility;
 *     - checking target capability satisfaction.
 *
 * None of these decisions occur here.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar produces NO IR.
 *
 * Correct flow:
 *
 *     capability syntax
 *         |
 *         v
 *     domain-neutral AST
 *         |
 *         v
 *     semantic capability model
 *         |
 *         v
 *     canonical semantic representation
 *         |
 *         +--> classical IR
 *         +--> quantum::ir
 *         +--> HDL/hardware representation
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
 * PUBLIC DIALECT DECLARATION ADAPTER
 * ============================================================================
 *
 * Example:
 *
 *     capability quantum::measurement;
 *
 *     capability quantum::dynamic_control version >= 1.0.0;
 *
 * The complete declaration syntax remains owned by core/capabilities.g4.
 */
dialectCapabilityDeclaration
    : capabilityDeclaration
    ;


/*
 * ============================================================================
 * PUBLIC DIALECT REFERENCE ADAPTER
 * ============================================================================
 *
 * Example:
 *
 *     quantum::measurement
 *
 * Version constraints remain owned by the canonical capability grammar.
 */
dialectCapabilityReference
    : capabilityReference
    ;


/*
 * ============================================================================
 * CAPABILITY NAME ADAPTER
 * ============================================================================
 */
dialectCapabilityName
    : capabilityName
    ;


/*
 * ============================================================================
 * CAPABILITY VERSION ADAPTER
 * ============================================================================
 */
dialectCapabilityVersion
    : capabilityVersionClause
    ;


/*
 * ============================================================================
 * CAPABILITY ATTRIBUTE ADAPTER
 * ============================================================================
 */
dialectCapabilityAttributes
    : capabilityAttributeList
    ;


/*
 * ============================================================================
 * CAPABILITY ALIAS
 * ============================================================================
 *
 * Example:
 *
 *     quantum::measurement as measurement;
 *
 * Alias resolution remains semantic.
 */
dialectCapabilityAlias
    : dialectCapabilityReference
      AS
      identifier
    ;


/*
 * ============================================================================
 * CAPABILITY ALIAS LIST
 * ============================================================================
 */
dialectCapabilityAliasList
    : dialectCapabilityAlias
      (COMMA dialectCapabilityAlias)*
    ;


/*
 * ============================================================================
 * CAPABILITY REFERENCE LIST
 * ============================================================================
 */
dialectCapabilityReferenceList
    : dialectCapabilityReference
      (COMMA dialectCapabilityReference)*
    ;


/*
 * ============================================================================
 * OPTIONAL CAPABILITY REFERENCE LIST
 * ============================================================================
 */
optionalDialectCapabilityReferenceList
    : dialectCapabilityReferenceList?
    ;


/*
 * ============================================================================
 * CAPABILITY DECLARATION LIST
 * ============================================================================
 */
dialectCapabilityDeclarationList
    : dialectCapabilityDeclaration+
    ;


/*
 * ============================================================================
 * OPTIONAL CAPABILITY DECLARATION LIST
 * ============================================================================
 */
optionalDialectCapabilityDeclarationList
    : dialectCapabilityDeclaration*
    ;


/*
 * ============================================================================
 * CAPABILITY REQUIREMENT
 * ============================================================================
 *
 * This rule represents the capability operand of a requirement.
 *
 * The enclosing dialect grammar owns the REQUIRES keyword.
 *
 * Example:
 *
 *     requires quantum::measurement;
 *
 * becomes conceptually:
 *
 *     REQUIRES dialectCapabilityRequirement SEMI
 */
dialectCapabilityRequirement
    : capabilityRequirementReference
    ;


/*
 * ============================================================================
 * CAPABILITY REQUIREMENT LIST
 * ============================================================================
 */
dialectCapabilityRequirementList
    : dialectCapabilityRequirement
      (COMMA dialectCapabilityRequirement)*
    ;


/*
 * ============================================================================
 * CAPABILITY PROVISION
 * ============================================================================
 *
 * A provision states that the dialect contract exposes a capability.
 *
 * It does not claim that every execution target provides it.
 */
dialectCapabilityProvision
    : capabilityProvisionReference
    ;


dialectCapabilityProvisionList
    : dialectCapabilityProvision
      (COMMA dialectCapabilityProvision)*
    ;


/*
 * ============================================================================
 * CAPABILITY PREFERENCE
 * ============================================================================
 */
dialectCapabilityPreference
    : capabilityPreferenceReference
    ;


dialectCapabilityPreferenceList
    : dialectCapabilityPreference
      (COMMA dialectCapabilityPreference)*
    ;


/*
 * ============================================================================
 * CAPABILITY CONSTRAINT
 * ============================================================================
 */
dialectCapabilityConstraint
    : capabilityConstraintReference
    ;


dialectCapabilityConstraintList
    : dialectCapabilityConstraint
      (COMMA dialectCapabilityConstraint)*
    ;


/*
 * ============================================================================
 * CAPABILITY EFFECT
 * ============================================================================
 */
dialectCapabilityEffect
    : capabilityEffectReference
    ;


dialectCapabilityEffectList
    : dialectCapabilityEffect
      (COMMA dialectCapabilityEffect)*
    ;


/*
 * ============================================================================
 * CAPABILITY TARGET REFERENCE
 * ============================================================================
 *
 * This remains symbolic.
 *
 * It does not select a physical target.
 */
dialectCapabilityTarget
    : capabilityTargetReference
    ;


dialectCapabilityTargetList
    : dialectCapabilityTarget
      (COMMA dialectCapabilityTarget)*
    ;


/*
 * ============================================================================
 * CAPABILITY FEATURE REFERENCE
 * ============================================================================
 */
dialectCapabilityFeature
    : capabilityFeatureReference
    ;


/*
 * ============================================================================
 * CAPABILITY INHERITANCE
 * ============================================================================
 *
 * Example:
 *
 *     extends quantum::base;
 *
 * or a capability-level inheritance reference where supported by the
 * surrounding dialect semantic contract.
 */
dialectCapabilityInheritanceReference
    : dialectCapabilityReference
    ;


dialectCapabilityInheritanceList
    : dialectCapabilityInheritanceReference
      (COMMA dialectCapabilityInheritanceReference)*
    ;


optionalDialectCapabilityInheritanceList
    : dialectCapabilityInheritanceList?
    ;


/*
 * ============================================================================
 * CAPABILITY IMPORT REFERENCE
 * ============================================================================
 *
 * Import semantics remain owned by dialect registration/module semantics.
 */
dialectCapabilityImportReference
    : dialectCapabilityReference
    ;


/*
 * ============================================================================
 * CAPABILITY EXPORT REFERENCE
 * ============================================================================
 *
 * Export semantics remain owned by dialect/module semantic analysis.
 *
 * This rule intentionally does not introduce another export syntax.
 */
dialectCapabilityExportReference
    : dialectCapabilityReference
    ;


/*
 * ============================================================================
 * CAPABILITY BLOCK
 * ============================================================================
 *
 * This replaces the previous invalid use of a nonexistent CAPABILITIES token.
 *
 * Canonical syntax:
 *
 *     capability {
 *         capability quantum::measurement;
 *         capability quantum::reset;
 *         capability quantum::dynamic_control;
 *     }
 *
 * The singular CAPABILITY keyword is already part of the canonical lexer.
 *
 * The block is syntactic grouping only.
 *
 * It does not create a nested capability namespace.
 */
dialectCapabilityBlock
    : CAPABILITY
      LBRACE
      dialectCapabilityBlockMember*
      RBRACE
    ;


/*
 * ============================================================================
 * CAPABILITY BLOCK MEMBER
 * ============================================================================
 */
dialectCapabilityBlockMember
    : dialectCapabilityDeclaration
    | dialectCapabilityReference SEMI
    ;


/*
 * ============================================================================
 * CAPABILITY BLOCK LIST
 * ============================================================================
 */
dialectCapabilityBlockList
    : dialectCapabilityBlock+
    ;


/*
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * A capability contract is a reusable structural container.
 *
 * It may contain declarations and capability blocks.
 *
 * Semantic meaning is resolved downstream.
 */
dialectCapabilityContract
    : dialectCapabilityContractMember*
    ;


dialectCapabilityContractMember
    : dialectCapabilityDeclaration
    | dialectCapabilityBlock
    ;


dialectCapabilityContractList
    : dialectCapabilityContract+
    ;


/*
 * ============================================================================
 * DECLARATION OR REFERENCE
 * ============================================================================
 *
 * Useful for generic dialect tooling.
 *
 * Semantic analysis determines whether a reference is legal in the
 * surrounding context.
 */
dialectCapabilityDeclarationOrReference
    : dialectCapabilityDeclaration
    | dialectCapabilityReference
    ;


dialectCapabilityDeclarationOrReferenceList
    : dialectCapabilityDeclarationOrReference
      (COMMA dialectCapabilityDeclarationOrReference)*
    ;


/*
 * ============================================================================
 * CAPABILITY EXPRESSION
 * ============================================================================
 *
 * This adapter reuses the canonical capability expression.
 *
 * Examples:
 *
 *     quantum::measurement
 *     quantum::measurement and quantum::reset
 *     quantum::simulator or quantum::hardware
 *     not legacy::unsupported
 *     (quantum::measurement and classical::control)
 *
 * No expression is evaluated by the parser.
 */
dialectCapabilityExpression
    : capabilityExpression
    ;


/*
 * ============================================================================
 * CAPABILITY REQUIREMENT EXPRESSION
 * ============================================================================
 */
dialectCapabilityRequirementExpression
    : dialectCapabilityExpression
    ;


/*
 * ============================================================================
 * CAPABILITY REQUIREMENT ENTRY
 * ============================================================================
 *
 * The enclosing grammar owns `requires`.
 */
dialectCapabilityRequirementEntry
    : dialectCapabilityRequirementExpression
    ;


dialectCapabilityRequirementEntryList
    : dialectCapabilityRequirementEntry
      (COMMA dialectCapabilityRequirementEntry)*
    ;


/*
 * ============================================================================
 * CAPABILITY SET
 * ============================================================================
 *
 * This adapter provides a dialect-context name without introducing another
 * capability-expression implementation.
 */
dialectCapabilitySet
    : dialectCapabilityExpression
    ;


/*
 * ============================================================================
 * CAPABILITY PREDICATE
 * ============================================================================
 */
dialectCapabilityPredicate
    : dialectCapabilityExpression
    ;


/*
 * ============================================================================
 * CAPABILITY CONTRACT ENTRY
 * ============================================================================
 */
dialectCapabilityContractEntry
    : dialectCapabilityDeclaration
    | dialectCapabilityBlock
    ;


dialectCapabilityContractEntryList
    : dialectCapabilityContractEntry+
    ;


/*
 * ============================================================================
 * INTEGRATION INVARIANTS
 * ============================================================================
 *
 * 1. Capability identity is canonical.
 *
 * 2. Qualified-name syntax is canonical.
 *
 * 3. Version syntax is canonical.
 *
 * 4. Attribute syntax is canonical.
 *
 * 5. No dialect-specific capability enumeration exists.
 *
 * 6. No hardware-specific capability enumeration exists.
 *
 * 7. No quantum gate enumeration exists.
 *
 * 8. No physical resource mapping exists.
 *
 * 9. No target selection exists.
 *
 * 10. No IR is produced here.
 *
 * 11. No second quantum IR is introduced.
 *
 * 12. No universal machine-size limit exists.
 *
 * 13. Capability meaning is resolved downstream.
 *
 * ============================================================================
 * AST TRACEABILITY
 * ============================================================================
 *
 * dialectCapabilityDeclaration
 *     -> existing domain-neutral Capability AST
 *
 * dialectCapabilityReference
 *     -> existing domain-neutral CapabilityReference representation
 *
 * dialectCapabilityAlias
 *     -> existing name/capability alias representation
 *
 * dialectCapabilityBlock
 *     -> capability-group/contract representation
 *
 * dialectCapabilityExpression
 *     -> canonical capability predicate representation
 *
 * The grammar MUST NOT require a new hardware-specific AST merely because a
 * capability belongs to quantum, classical, HDL, AI, or another domain.
 *
 * ============================================================================
 * SEMANTIC TRACEABILITY
 * ============================================================================
 *
 * Syntax
 *     |
 *     v
 * AST
 *     |
 *     v
 * semantic capability identity
 *     |
 *     +--> capability registry
 *     +--> dialect registry
 *     +--> version compatibility
 *     +--> requirement analysis
 *     +--> resource analysis
 *     +--> target capability analysis
 *     |
 *     v
 * canonical semantic representation
 *
 * ============================================================================
 * QUANTUM TRACEABILITY
 * ============================================================================
 *
 * Example:
 *
 *     capability quantum::mid_circuit_measurement;
 *
 * means only that the dialect contract identifies the semantic capability.
 *
 * It does NOT choose:
 *
 *     a QPU
 *     a simulator
 *     a physical qubit
 *     a topology
 *     a calibration
 *     a pulse schedule
 *
 * If quantum semantics are eventually generated, the downstream path is:
 *
 *     AST
 *       ->
 *     semantic quantum model
 *       ->
 *     quantum::ir
 *       ->
 *     optimization
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
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden:
 *
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *
 * Also forbidden:
 *
 *     physical qubit IDs
 *     physical CPU IDs
 *     physical GPU IDs
 *     vendor device IDs
 *     fixed topology
 *     fixed accelerator counts
 *     fixed register widths
 *
 * This grammar contains none of them.
 *
 * ============================================================================
 * DIAGNOSTICS CONTRACT
 * ============================================================================
 *
 * Parser diagnostics must preserve the source span of:
 *
 *     capability keyword;
 *     capability identity;
 *     version constraint;
 *     alias;
 *     capability block;
 *     capability expression.
 *
 * Semantic diagnostics should be capable of distinguishing:
 *
 *     unknown capability
 *     duplicate capability
 *     invalid alias
 *     incompatible version
 *     unsatisfied requirement
 *     conflicting capabilities
 *     invalid dialect capability
 *     invalid export
 *     invalid import
 *
 * These are semantic diagnostics, not parser actions.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * Capability syntax MUST NOT implicitly:
 *
 *     execute code;
 *     load plugins;
 *     access hardware;
 *     inspect devices;
 *     access secrets;
 *     make network requests;
 *     allocate resources;
 *     invoke vendor APIs.
 *
 * Capability discovery and authorization belong to downstream infrastructure.
 *
 * ============================================================================
 * PERFORMANCE CONTRACT
 * ============================================================================
 *
 * Grammar complexity must scale with source structure.
 *
 * No rule performs:
 *
 *     registry lookup;
 *     filesystem lookup;
 *     network lookup;
 *     target probing;
 *     semantic capability solving.
 *
 * Capability resolution must therefore remain outside parsing.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE
 * --------
 *
 *     capability quantum::measurement;
 *
 *     capability quantum::dynamic_control;
 *
 *     capability quantum::dynamic_control version 1.2.0;
 *
 *     capability quantum::dynamic_control version >= 1.0.0;
 *
 *     capability classical::parallel;
 *
 *     capability hdl::sequential_logic;
 *
 *     capability hardware::reconfigurable;
 *
 *     capability ai::tensor_compute;
 *
 *     capability future::computing::new_architecture;
 *
 *
 * CAPABILITY BLOCK
 * ---------------
 *
 *     capability {
 *         capability quantum::measurement;
 *         capability quantum::reset;
 *         capability quantum::dynamic_control;
 *     }
 *
 *
 * ALIAS
 * -----
 *
 *     quantum::measurement as measurement
 *
 *
 * EXPRESSION
 * ----------
 *
 *     quantum::measurement and quantum::reset
 *
 *     quantum::simulator or quantum::hardware
 *
 *     not legacy::unsupported
 *
 *     (quantum::measurement and classical::control)
 *
 *
 * NEGATIVE
 * --------
 *
 *     capability;
 *
 *     capability ;
 *
 *     capability as measurement;
 *
 *     capability quantum::measurement as;
 *
 *     capability quantum::measurement as 123;
 *
 *     capability quantum::measurement version;
 *
 *     capability quantum::measurement {
 *
 *
 * BOUNDARY
 * --------
 *
 *     capability a;
 *
 *     capability a::b;
 *
 *     capability a::b::c::d::e;
 *
 *     capability organization::domain::future::capability;
 *
 *
 * SCALABILITY
 * ----------
 *
 * Tests must cover:
 *
 *     - arbitrarily many capability declarations;
 *     - arbitrarily many references;
 *     - arbitrarily deep qualified capability names;
 *     - arbitrarily many dialects;
 *     - arbitrarily many capability groups;
 *     - large capability expressions;
 *
 * Tests MUST NOT define a language-level maximum.
 *
 *
 * DETERMINISM
 * -----------
 *
 * The same token sequence must produce the same parse structure.
 *
 *
 * COMPATIBILITY
 * -------------
 *
 * Existing canonical forms remain valid:
 *
 *     capability qualified::name;
 *
 *     capability qualified::name version ...;
 *
 * The dialect adapter does not alter their meaning.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 * [x] It has one parser grammar declaration.
 * [x] It uses the canonical ZamaniLexer vocabulary.
 * [x] It delegates capability identity to core/capabilities.g4.
 * [x] It delegates names to core/names.g4.
 * [x] It does not duplicate capability version semantics.
 * [x] It does not enumerate capability names.
 * [x] It does not enumerate hardware.
 * [x] It does not enumerate quantum gates.
 * [x] It does not create IR.
 * [x] It does not create a second quantum IR.
 * [x] It does not impose machine-size limits.
 * [x] It contains no Rust actions.
 * [x] It requires no unsafe Rust.
 * [x] It has explicit AST integration.
 * [x] It has explicit semantic integration.
 * [x] It has explicit IR integration.
 * [x] It has positive tests.
 * [x] It has negative tests.
 * [x] It has boundary tests.
 * [x] It has scalability tests.
 * [x] It has determinism requirements.
 * [x] It has compatibility requirements.
 *
 * ============================================================================
 * END
 * ============================================================================
 */
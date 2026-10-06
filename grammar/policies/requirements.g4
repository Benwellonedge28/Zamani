/*
 * ============================================================================
 * ZAMANI UNIVERSAL PROGRAMMING LANGUAGE
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/policies/requirements.g4
 *
 * GRAMMAR
 * -------
 * ANTLR4 parser grammar
 *
 * GRAMMAR NAME
 * ------------
 * PolicyRequirements
 *
 * STATUS
 * ------
 * CANONICAL POLICY REQUIREMENT INTEGRATION GRAMMAR
 *
 * IMPLEMENTATION BASELINE
 * -----------------------
 * Rust 1.97+
 * Rust 2021
 * Safe Rust only
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * This file is the canonical policy-layer adapter for REQUIREMENTS.
 *
 * It does not invent a third requirement language.
 *
 * Instead, it composes the two existing requirement authorities:
 *
 *     grammar/core/requirements.g4
 *         |
 *         +--> universal semantic requirements
 *
 *     grammar/resources/requirements.g4
 *         |
 *         +--> resource realization requirements
 *
 * This file establishes the policy boundary around those requirements.
 *
 *
 * A policy requirement expresses a condition that policy evaluation,
 * compilation, execution planning, deployment, simulation, security,
 * adaptation, or another policy consumer must take into account.
 *
 * The policy layer may therefore consume requirements concerning:
 *
 *     classical computation
 *     quantum computation
 *     hybrid computation
 *     HDL
 *     hardware
 *     accelerators
 *     AI/model computation
 *     data processing
 *     distributed computation
 *     networking
 *     interoperability
 *     security
 *     execution
 *     simulation
 *     metaprogramming
 *     future computational domains
 *
 * The policy grammar remains domain-neutral.
 *
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 * The intended pipeline is:
 *
 *     source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       v
 *     ZamaniParser
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     structural validation
 *       |
 *       +--> requirements
 *       +--> constraints
 *       +--> capabilities
 *       +--> resources
 *       +--> effects
 *       +--> contracts
 *       +--> policies
 *       +--> provenance
 *       |
 *       v
 *     semantic model
 *       |
 *       +--> classical path
 *       +--> quantum semantic path
 *       +--> HDL/hardware path
 *       +--> distributed path
 *       +--> other domain paths
 *       |
 *       v
 *     target-independent planning
 *       |
 *       v
 *     optimization / lowering / routing / scheduling
 *       |
 *       v
 *     target realization
 *
 *
 * This file participates only in source parsing.
 *
 * It does not:
 *
 *     discover hardware;
 *     allocate resources;
 *     select a target;
 *     authorize an operation;
 *     evaluate policy;
 *     prove satisfiability;
 *     execute a requirement;
 *     create classical IR;
 *     create quantum::ir;
 *     create HDL IR;
 *     perform routing;
 *     perform scheduling;
 *     perform QEC;
 *     perform calibration.
 *
 *
 * ============================================================================
 * OWNS
 * ============================================================================
 *
 * This file owns ONLY the policy integration boundary for requirements:
 *
 *     policyRequirement
 *     policyRequirementClause
 *     policyRequirementList
 *     optionalPolicyRequirementList
 *     policyRequirementGroup
 *     policyRequirementGroupItem
 *
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     identifier syntax
 *     qualified-name syntax
 *     lexer tokens
 *     universal requirement semantics
 *     resource requirement semantics
 *     capability identity
 *     capability versions
 *     resource expressions
 *     arithmetic
 *     comparison semantics
 *     logical expression semantics
 *     constraints
 *     preferences
 *     permissions
 *     prohibitions
 *     contracts
 *     effects
 *     provenance
 *     security authorization
 *     target selection
 *     hardware discovery
 *     quantum operations
 *     quantum::ir
 *     HDL
 *     backend implementation
 *     runtime enforcement
 *
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * Requirement syntax MUST have exactly two canonical source authorities:
 *
 *     Requirements
 *         grammar/core/requirements.g4
 *
 *     ResourceRequirements
 *         grammar/resources/requirements.g4
 *
 * This file is an adapter.
 *
 * It MUST NOT copy their internal expression rules.
 *
 * In particular, this file MUST NOT redefine:
 *
 *     requirementExpression
 *     requirementDisjunction
 *     requirementConjunction
 *     requirementUnary
 *     requirementPrimary
 *     requirementReference
 *
 * or:
 *
 *     resourceRequirementExpression
 *     resourceCapabilityCall
 *     resourceCapabilityReference
 *     resourceExpression
 *
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/core/requirements.g4
 *     grammar/resources/requirements.g4
 *
 * Indirect dependencies:
 *
 *     grammar/core/names.g4
 *     grammar/core/capabilities.g4
 *     grammar/resources/resource-expressions.g4
 *     grammar/expressions/
 *
 *
 * IMPORTS:
 *
 *     Requirements
 *     ResourceRequirements
 *
 *
 * EXPORTS:
 *
 *     policyRequirement
 *     policyRequirementClause
 *     policyRequirementList
 *     optionalPolicyRequirementList
 *     policyRequirementGroup
 *     policyRequirementGroupItem
 *
 *
 * CONSUMED_BY:
 *
 *     grammar/policies/policy.g4
 *     grammar/policies/scopes.g4
 *     grammar/policies/constraints.g4
 *     grammar/policies/preferences.g4
 *     grammar/policies/fallbacks.g4
 *     grammar/policies/execution.g4
 *     grammar/policies/resource.g4
 *     grammar/policies/deployment.g4
 *     grammar/policies/simulation.g4
 *     grammar/policies/security.g4
 *     grammar/policies/adaptation.g4
 *     future policy adapters
 *
 *
 * AST_OWNER:
 *
 *     domain-neutral frontend AST
 *
 *
 * SEMANTIC_OWNER:
 *
 *     policy semantic model
 *     requirement semantic model
 *     resource semantic model
 *
 *
 * IR_OWNER:
 *
 *     canonical semantic representation
 *     downstream classical IR
 *     quantum::ir
 *     HDL/hardware representation
 *     execution/deployment plans
 *
 *
 * TEST_OWNER:
 *
 *     grammar/tests/policies/
 *     grammar/tests/resources/
 *     grammar/tests/validation/
 *     grammar/tests/parser/
 *     grammar/tests/semantic/
 *     grammar/tests/scalability/
 *     grammar/tests/portability/
 *     grammar/tests/boundary/
 *     grammar/tests/negative/
 *
 *
 * SPEC_OWNER:
 *
 *     grammar/spec/policies.md
 *     grammar/spec/resources.md
 *     grammar/specification/poco-reaf.md
 *
 *
 * ============================================================================
 * FUNDAMENTAL SEMANTIC DISTINCTION
 * ============================================================================
 *
 * A:
 *
 *     requirement
 *
 * is not:
 *
 *     capability
 *     permission
 *     prohibition
 *     preference
 *     resource allocation
 *     target selection
 *     authorization
 *
 *
 * Capability:
 *
 *     What a realization CAN provide.
 *
 *
 * Requirement:
 *
 *     What a program or policy NEEDS.
 *
 *
 * Resource:
 *
 *     A computational quantity or facility that can participate in
 *     realization.
 *
 *
 * Policy:
 *
 *     Governing intent controlling how otherwise valid alternatives may
 *     be selected, rejected, preferred, constrained, or adapted.
 *
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Policy requirements MUST preserve source portability.
 *
 * For example:
 *
 *     requires quantum::measurement;
 *
 * does not select a particular QPU.
 *
 *
 *     requires capability("tensor.compute");
 *
 * does not select a particular accelerator.
 *
 *
 *     requires memory >= required_memory;
 *
 * does not select a particular memory implementation.
 *
 *
 *     requires nodes >= required_nodes;
 *
 * does not select a particular cluster.
 *
 *
 * The same source-level requirement can therefore participate in realization
 * on different machines, provided semantic feasibility and available
 * resources permit it.
 *
 *
 * ============================================================================
 * OPEN-WORLD CONTRACT
 * ============================================================================
 *
 * This file MUST remain open-world.
 *
 * It MUST NOT enumerate:
 *
 *     CPU types
 *     GPU types
 *     FPGA types
 *     ASIC types
 *     QPU types
 *     accelerator types
 *     vendor devices
 *     cloud providers
 *     quantum gate sets
 *     hardware models
 *     AI models
 *     resource categories
 *     future architectures
 *
 * New semantic capabilities and resource categories must normally be
 * represented by their existing open-world requirement/resource grammars.
 *
 * A new computational domain therefore does not require this file to be
 * modified merely because a new requirement name is introduced.
 *
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar introduces NO language-level finite limit on:
 *
 *     number of policy requirements
 *     number of policy requirement groups
 *     number of requirement expressions
 *     number of alternatives
 *     number of logical operands
 *     number of nested groups
 *     qualified-name depth
 *     resource quantity magnitude
 *     resource categories
 *     capability categories
 *     computational domains
 *
 * Repetition uses ANTLR repetition operators:
 *
 *     *
 *     +
 *
 * rather than fixed cardinalities.
 *
 *
 * "Infinity" means:
 *
 *     no artificial language-defined machine-capacity ceiling.
 *
 * It does NOT mean:
 *
 *     infinite physical hardware;
 *     infinite compiler memory;
 *     infinite parser memory;
 *     infinite execution time.
 *
 * Those remain implementation and target-resource concerns.
 *
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This file MUST NOT introduce:
 *
 *     MAX_REQUIREMENTS
 *     MAX_RESOURCES
 *     MAX_CAPABILITIES
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
 * It also MUST NOT introduce indirect equivalents such as:
 *
 *     exactly N requirement alternatives;
 *     fixed hardware categories;
 *     fixed capability counts;
 *     fixed resource dimensions;
 *     fixed topology sizes.
 *
 * Numeric literals appearing in a requirement remain program values.
 *
 * For example:
 *
 *     requires qubits >= 1024;
 *
 * means that the program requires a semantic quantity whose value is 1024.
 *
 * It does NOT define a universal language limit of 1024 qubits.
 *
 *
 * ============================================================================
 * POLICY BOUNDARY
 * ============================================================================
 *
 * This file does not decide whether a requirement is:
 *
 *     satisfied;
 *     unsatisfied;
 *     unknown;
 *     conditional;
 *     overridden;
 *     permitted;
 *     prohibited.
 *
 * Those decisions belong to policy and semantic analysis.
 *
 * A policy may consume a requirement and combine it with:
 *
 *     constraints
 *     capabilities
 *     resources
 *     permissions
 *     prohibitions
 *     preferences
 *     fallbacks
 *     adaptation rules
 *     security rules
 *     execution rules
 *     deployment rules
 *     provenance
 *
 *
 * ============================================================================
 * REQUIREMENT FORM INTEGRATION
 * ============================================================================
 *
 * Universal requirement:
 *
 *     requires quantum::measurement;
 *
 *
 * Universal compound requirement:
 *
 *     requires quantum::measurement
 *              and execution::deterministic;
 *
 *
 * Resource requirement:
 *
 *     requires qubits >= logical_qubits;
 *
 *
 * Resource capability call:
 *
 *     requires capability("tensor.compute");
 *
 *
 * Resource capability reference:
 *
 *     requires capability quantum::measurement;
 *
 *
 * Versioned capability requirement:
 *
 *     requires capability quantum::dynamic_control version >= 1.2;
 *
 *
 * Both universal and resource requirements remain owned by their respective
 * canonical grammars.
 *
 *
 * ============================================================================
 * POLICY REQUIREMENT DECLARATION
 * ============================================================================
 *
 * A complete policy member is represented by:
 *
 *     policyRequirement
 *
 * The policy parser therefore does not need to know whether the requirement
 * is universal or resource-oriented before delegating to the canonical
 * requirement grammar.
 *
 * Semantic analysis MUST retain the originating requirement category.
 *
 *
 * ============================================================================
 * REUSABLE REQUIREMENT CLAUSE
 * ============================================================================
 *
 * `policyRequirementClause` is provided for policy grammars that own the
 * terminating semicolon themselves.
 *
 * Example conceptual use:
 *
 *     policyRule
 *         : ...
 *           policyRequirementClause
 *           ...
 *         ;
 *
 * The complete declaration rule remains:
 *
 *     policyRequirement
 *
 *
 * ============================================================================
 * GROUP CONTRACT
 * ============================================================================
 *
 * `policyRequirementGroup` provides a structural policy boundary.
 *
 * Example:
 *
 *     requires {
 *         quantum::measurement;
 *         execution::deterministic;
 *         qubits >= logical_qubits;
 *         capability("tensor.compute");
 *     }
 *
 * The group does not create a new semantic requirement system.
 *
 * Each member remains owned by the canonical requirement authority.
 *
 * Semantic analysis determines whether the group represents:
 *
 *     conjunction;
 *     ordered requirements;
 *     policy-local requirements;
 *     another policy-defined aggregation.
 *
 * The parser merely preserves the grouping.
 *
 *
 * ============================================================================
 * IMPORTANT GROUP OWNERSHIP RULE
 * ============================================================================
 *
 * This file does not redefine:
 *
 *     requirementExpression
 *     resourceRequirementExpression
 *
 * Group members delegate directly to:
 *
 *     requirementDeclaration
 *     resourceRequirement
 *
 * where complete requirement statements are required.
 *
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The grammar creates parser contexts only.
 *
 * The domain-neutral AST should preserve:
 *
 *     policy requirement kind;
 *     requirement expression;
 *     resource requirement expression;
 *     grouping;
 *     source ordering;
 *     source spans;
 *     nested structure.
 *
 * The AST MUST NOT contain:
 *
 *     physical device identifiers;
 *     hardware allocation;
 *     routing decisions;
 *     scheduler decisions;
 *     QEC decisions;
 *     calibration;
 *     backend instructions.
 *
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis owns:
 *
 *     requirement classification;
 *     name resolution;
 *     capability resolution;
 *     resource resolution;
 *     version compatibility;
 *     type validation;
 *     unit validation;
 *     requirement normalization;
 *     duplicate detection;
 *     contradiction detection;
 *     satisfiability;
 *     policy interaction;
 *     resource feasibility;
 *     target feasibility;
 *     provenance;
 *     diagnostics.
 *
 * The parser performs none of these operations.
 *
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * Requirement expressions are typed downstream.
 *
 * Examples:
 *
 *     qubits >= logical_qubits
 *
 * requires valid comparable semantic quantities.
 *
 *
 *     capability("tensor.compute")
 *
 * requires capability-call semantics defined by the resource subsystem.
 *
 *
 * This grammar does not determine:
 *
 *     units;
 *     dimensionality;
 *     numeric precision;
 *     resource magnitude;
 *     capability implementation.
 *
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Parsing a policy requirement has no execution effect.
 *
 * It does not:
 *
 *     allocate resources;
 *     access hardware;
 *     invoke capabilities;
 *     access the network;
 *     access the filesystem;
 *     execute foreign code;
 *     execute native code;
 *     modify runtime state.
 *
 * Effects are attached downstream to the semantic constructs being governed.
 *
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Capability identity remains owned by:
 *
 *     grammar/core/capabilities.g4
 *
 * This file MUST NOT enumerate capability names.
 *
 * Capability references are passed through unchanged to semantic analysis.
 *
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Resource requirement syntax remains owned by:
 *
 *     grammar/resources/requirements.g4
 *
 * This file does not define:
 *
 *     resource quantity syntax;
 *     resource comparison syntax;
 *     resource dimensions;
 *     capacity;
 *     availability;
 *     allocation;
 *     topology.
 *
 *
 * ============================================================================
 * CONTRACT INTEGRATION
 * ============================================================================
 *
 * Policy requirements may participate in:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *     assert
 *
 * However, this file does not own contract syntax.
 *
 * Contract semantics remain in the validation/contract subsystem.
 *
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Policy requirement parsing must preserve enough source structure for
 * downstream provenance to record:
 *
 *     source location;
 *     requirement origin;
 *     policy origin;
 *     grouping;
 *     transformation;
 *     semantic decision;
 *     verification result.
 *
 * Provenance generation itself remains downstream.
 *
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Quantum requirements may influence:
 *
 *     quantum semantic analysis;
 *     quantum resource analysis;
 *     operation selection;
 *     decomposition;
 *     routing;
 *     scheduling;
 *     resilience;
 *     QEC;
 *     ZQN;
 *     HAL.
 *
 * This grammar never creates or manipulates quantum::ir.
 *
 * The canonical quantum boundary remains:
 *
 *     source
 *       ->
 *     AST
 *       ->
 *     semantic quantum model
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
 *     resilience / QEC
 *       ->
 *     ZQN
 *       ->
 *     HAL
 *       ->
 *     hardware
 *
 *
 * ============================================================================
 * HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * Hardware-oriented requirements remain abstract.
 *
 * Examples:
 *
 *     requires bandwidth >= required_bandwidth;
 *     requires latency <= latency_budget;
 *     requires capability("hardware.streaming");
 *
 * This grammar does not define:
 *
 *     wire widths;
 *     physical cells;
 *     pin counts;
 *     register counts;
 *     FPGA fabric size;
 *     placement;
 *     routing;
 *     synthesis.
 *
 *
 * ============================================================================
 * AI / REASONING BOUNDARY
 * ============================================================================
 *
 * Requirements may express capabilities needed by:
 *
 *     inference;
 *     reasoning;
 *     learning;
 *     adaptation;
 *     uncertainty;
 *     provenance;
 *     agents;
 *     simulation.
 *
 * No AI-specific requirement syntax is created here.
 *
 * New AI capabilities remain ordinary open-world capability/requirement
 * references.
 *
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing is determined only by:
 *
 *     source token stream;
 *     grammar version;
 *     parser configuration.
 *
 * Parsing MUST NOT depend on:
 *
 *     time;
 *     randomness;
 *     hardware;
 *     filesystem state;
 *     network state;
 *     target availability;
 *     runtime state;
 *     environment state.
 *
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * This grammar MUST NOT:
 *
 *     execute requirements;
 *     authorize capabilities;
 *     bypass security;
 *     inspect credentials;
 *     discover devices;
 *     access secrets;
 *     invoke FFI;
 *     invoke native code.
 *
 * Policy enforcement is downstream.
 *
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser diagnostics are structural only.
 *
 * Examples of structural failures include:
 *
 *     requires;
 *
 *     requires quantum::;
 *
 *     requires and quantum::measurement;
 *
 *     requires quantum::measurement and;
 *
 *     requires quantum::measurement or;
 *
 *     requires (
 *         quantum::measurement;
 *
 *     requires quantum::measurement);
 *
 * Semantic failures remain downstream:
 *
 *     unknown capability;
 *     unavailable capability;
 *     unsatisfied resource;
 *     incompatible version;
 *     contradictory requirements;
 *     policy conflict;
 *     insufficient target resources;
 *     forbidden capability.
 *
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * This file introduces no alternate spelling for existing requirement syntax.
 *
 * Existing source forms remain owned by:
 *
 *     Requirements
 *     ResourceRequirements
 *
 * Historical aliases belong to:
 *
 *     grammar/compatibility/
 *
 * They MUST NOT be silently recreated here.
 *
 *
 * ============================================================================
 * ANTLR INTEGRATION
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * Canonical lexer:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * Required grammar imports:
 *
 *     Requirements
 *     ResourceRequirements
 *
 * ANTLR grammar resolution is by grammar identity, not filesystem path.
 *
 * Therefore the resource requirement grammar MUST expose the identity:
 *
 *     ResourceRequirements
 *
 * while retaining the existing source path:
 *
 *     grammar/resources/requirements.g4
 *
 *
 * ============================================================================
 * GRAMMAR DEFINITION
 * ============================================================================
 */

parser grammar PolicyRequirements;

options {
    tokenVocab = ZamaniLexer;
}

import
    Requirements,
    ResourceRequirements
;


/*
 * ============================================================================
 * 1. COMPLETE POLICY REQUIREMENT
 * ============================================================================
 *
 * A policy requirement may be either:
 *
 *     universal requirement
 *
 * or:
 *
 *     resource requirement
 *
 * The underlying syntax remains owned by the imported grammar.
 *
 * No requirement expression is duplicated here.
 *
 * ============================================================================
 */

policyRequirement
    : requirementDeclaration
    | resourceRequirement
    ;


/*
 * ============================================================================
 * 2. REUSABLE POLICY REQUIREMENT CLAUSE
 * ============================================================================
 *
 * This rule is intended for policy constructs that own the final semicolon.
 *
 * Canonical complete declarations remain represented by:
 *
 *     policyRequirement
 *
 * This rule deliberately delegates the expression itself to the canonical
 * requirement grammars.
 *
 * ============================================================================
 */

policyRequirementClause
    : requirementClause
    | REQUIRES resourceRequirementExpression
    ;


/*
 * ============================================================================
 * 3. POLICY REQUIREMENT LIST
 * ============================================================================
 *
 * There is no language-level maximum number of policy requirements.
 *
 * The surrounding policy grammar owns ordering and enclosing structure.
 *
 * ============================================================================
 */

policyRequirementList
    : policyRequirement+
    ;


optionalPolicyRequirementList
    : policyRequirement*
    ;


/*
 * ============================================================================
 * 4. POLICY REQUIREMENT GROUP
 * ============================================================================
 *
 * Structural grouping for policy consumers.
 *
 * Example:
 *
 *     requires {
 *         quantum::measurement;
 *         execution::deterministic;
 *         qubits >= logical_qubits;
 *         capability("tensor.compute");
 *     }
 *
 * The group does not create a separate requirement semantic model.
 *
 * ============================================================================
 */

policyRequirementGroup
    : REQUIRES
      LBRACE
      policyRequirementGroupItem*
      RBRACE
    ;


policyRequirementGroupItem
    : requirementDeclaration
    | resourceRequirement
    ;


/*
 * ============================================================================
 * 5. POLICY REQUIREMENT EXPRESSION BOUNDARY
 * ============================================================================
 *
 * These wrappers are provided for policy adapters that need an expression
 * without owning the complete declaration terminator.
 *
 * Universal:
 *
 *     requirementExpression
 *
 * Resource:
 *
 *     resourceRequirementExpression
 *
 * The imported grammars remain authoritative.
 * ============================================================================
 */

policyUniversalRequirementExpression
    : requirementExpression
    ;


policyResourceRequirementExpression
    : resourceRequirementExpression
    ;


/*
 * ============================================================================
 * 6. POLICY REQUIREMENT KIND
 * ============================================================================
 *
 * This wrapper provides a stable semantic classification boundary without
 * enumerating domains.
 *
 * It intentionally exposes only the two architectural requirement families:
 *
 *     universal
 *     resource
 *
 * Domain-specific names remain open-world inside those families.
 *
 * ============================================================================
 */

policyRequirementKind
    : requirementDeclaration
    | resourceRequirement
    ;


/*
 * ============================================================================
 * 7. OWNERSHIP INVARIANT
 * ============================================================================
 *
 * The following rules MUST NOT be added here:
 *
 *     requirementExpression
 *     requirementDisjunction
 *     requirementConjunction
 *     requirementUnary
 *     requirementPrimary
 *     requirementReference
 *     resourceRequirementExpression
 *     resourceCapabilityCall
 *     resourceCapabilityReference
 *     resourceExpression
 *
 * They already have canonical owners.
 *
 *
 * If requirement syntax evolves, update its canonical grammar first and
 * preserve this adapter as a thin policy integration layer.
 *
 *
 * ============================================================================
 * 8. NO SECOND POLICY REQUIREMENT LANGUAGE
 * ============================================================================
 *
 * Policy requirements MUST NOT acquire policy-specific copies of:
 *
 *     arithmetic;
 *     logical precedence;
 *     capability syntax;
 *     resource comparison;
 *     version syntax;
 *     qualified-name syntax.
 *
 * This prevents:
 *
 *     source requirement semantics
 *
 * from differing merely because the requirement appears inside a policy.
 *
 *
 * ============================================================================
 * 9. RESOURCE / UNIVERSAL SEPARATION
 * ============================================================================
 *
 * The distinction between:
 *
 *     requirementDeclaration
 *
 * and:
 *
 *     resourceRequirement
 *
 * is preserved structurally.
 *
 * Semantic analysis may normalize them into a common requirement model while
 * retaining their original source category.
 *
 *
 * ============================================================================
 * 10. EXTENSIBILITY
 * ============================================================================
 *
 * Adding a new:
 *
 *     resource;
 *     capability;
 *     accelerator;
 *     quantum facility;
 *     hardware facility;
 *     distributed facility;
 *     AI facility;
 *     data facility;
 *     networking facility;
 *     interoperability facility;
 *     future computational facility;
 *
 * MUST NOT require a new alternative in this grammar.
 *
 * Such concepts are represented through the open-world canonical requirement
 * and resource grammars.
 *
 *
 * ============================================================================
 * 11. NEGATIVE-SPACE RULE
 * ============================================================================
 *
 * This grammar MUST NOT contain:
 *
 *     target-specific device syntax;
 *     hardware identifiers;
 *     physical qubit identifiers;
 *     vendor-specific instruction syntax;
 *     fixed resource capacities;
 *     fixed topology sizes;
 *     scheduler directives;
 *     routing directives;
 *     calibration syntax;
 *     QEC syntax;
 *     backend instructions.
 *
 *
 * ============================================================================
 * 12. TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE TESTS
 * -------------
 *
 * The policy requirement integration must accept:
 *
 *     requires quantum::measurement;
 *
 *     requires execution::deterministic;
 *
 *     requires quantum::measurement
 *              and execution::deterministic;
 *
 *     requires qubits >= logical_qubits;
 *
 *     requires memory >= required_memory;
 *
 *     requires nodes >= required_nodes;
 *
 *     requires capability("quantum.measurement");
 *
 *     requires capability("tensor.compute");
 *
 *     requires capability quantum::measurement;
 *
 *     requires capability quantum::dynamic_control version >= 1.2;
 *
 *
 * GROUP TESTS
 * -----------
 *
 *     requires {
 *         quantum::measurement;
 *         execution::deterministic;
 *         qubits >= logical_qubits;
 *         capability("tensor.compute");
 *     }
 *
 *
 * OPEN-WORLD TEST
 * ---------------
 *
 *     requires future::computing::new_capability;
 *
 *
 * CROSS-DOMAIN TESTS
 * ------------------
 *
 * Requirements must remain usable with:
 *
 *     classical;
 *     quantum;
 *     hybrid;
 *     HDL;
 *     hardware;
 *     AI;
 *     data;
 *     distributed;
 *     networking;
 *     security;
 *     simulation;
 *     interoperability;
 *     metaprogramming.
 *
 *
 * NEGATIVE TESTS
 * --------------
 *
 * The following must fail structurally:
 *
 *     requires;
 *
 *     requires quantum::;
 *
 *     requires and quantum::measurement;
 *
 *     requires quantum::measurement and;
 *
 *     requires quantum::measurement or;
 *
 *     requires (
 *         quantum::measurement;
 *
 *     requires quantum::measurement);
 *
 *
 * RESOURCE NEGATIVE TESTS
 * -----------------------
 *
 * Malformed resource expressions must be rejected by
 * ResourceRequirements/ResourceExpressions rather than by this adapter.
 *
 *
 * ============================================================================
 * 13. SCALABILITY TESTS
 * ============================================================================
 *
 * Test suites MUST verify that this file introduces no artificial limit on:
 *
 *     number of policy requirements;
 *     number of grouped requirements;
 *     logical expression size;
 *     qualified-name depth;
 *     resource magnitude;
 *     capability namespace depth;
 *     computational-domain count.
 *
 * Tests should use generated source data whose size is constrained by the
 * test environment rather than by language-defined constants.
 *
 *
 * ============================================================================
 * 14. DETERMINISM TEST
 * ============================================================================
 *
 * Given identical:
 *
 *     source;
 *     lexer version;
 *     parser version;
 *     grammar version;
 *     parser configuration;
 *
 * the resulting parse structure must be equivalent.
 *
 * No runtime state may influence parsing.
 *
 *
 * ============================================================================
 * 15. SEMANTIC INTEGRATION TEST
 * ============================================================================
 *
 * The frontend must preserve enough structure to distinguish:
 *
 *     universal requirement
 *
 * from:
 *
 *     resource requirement
 *
 * while allowing the semantic layer to normalize both into the common
 * requirement representation where appropriate.
 *
 *
 * ============================================================================
 * 16. QUANTUM INTEGRATION TEST
 * ============================================================================
 *
 * A quantum policy requirement such as:
 *
 *     requires quantum::measurement;
 *
 * may affect later:
 *
 *     capability analysis;
 *     resource analysis;
 *     quantum semantic analysis;
 *     quantum::ir lowering;
 *     routing;
 *     scheduling;
 *     resilience.
 *
 * No policy parser rule may directly create:
 *
 *     QubitId;
 *     PhysicalQubitId;
 *     routing map;
 *     calibration object;
 *     QEC object.
 *
 *
 * ============================================================================
 * 17. HARDWARE / HDL INTEGRATION TEST
 * ============================================================================
 *
 * A hardware-oriented requirement such as:
 *
 *     requires bandwidth >= required_bandwidth;
 *
 * remains abstract until downstream resource and target analysis.
 *
 * No hardware capacity becomes a grammar constant.
 *
 *
 * ============================================================================
 * 18. SECURITY INTEGRATION TEST
 * ============================================================================
 *
 * A requirement such as:
 *
 *     requires security::trusted_execution;
 *
 * expresses a requirement.
 *
 * It does NOT grant permission.
 *
 * Authorization remains owned by the security subsystem.
 *
 *
 * ============================================================================
 * 19. PROVENANCE INTEGRATION TEST
 * ============================================================================
 *
 * The frontend must preserve source spans and structural relationships so
 * provenance can later record:
 *
 *     source requirement;
 *     policy containing it;
 *     semantic normalization;
 *     resolution;
 *     decision;
 *     verification;
 *     realization.
 *
 *
 * ============================================================================
 * 20. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 * [ ] Grammar identity is exactly PolicyRequirements.
 *
 * [ ] Canonical lexer is ZamaniLexer.
 *
 * [ ] Core Requirements grammar is imported.
 *
 * [ ] ResourceRequirements grammar is imported.
 *
 * [ ] No requirement expression syntax is duplicated.
 *
 * [ ] No resource-expression syntax is duplicated.
 *
 * [ ] Universal and resource requirement ownership remains distinct.
 *
 * [ ] Policy requirement syntax is reusable by policy subgrammars.
 *
 * [ ] Requirement groups preserve structure without creating a second
 *     semantic model.
 *
 * [ ] No hardware capacity is hard-coded.
 *
 * [ ] No quantum capacity is hard-coded.
 *
 * [ ] No machine count is hard-coded.
 *
 * [ ] No tensor-rank ceiling is hard-coded.
 *
 * [ ] No network-size ceiling is hard-coded.
 *
 * [ ] No fixed capability catalogue is introduced.
 *
 * [ ] No fixed resource catalogue is introduced.
 *
 * [ ] No semantic predicates are used.
 *
 * [ ] No embedded Rust is used.
 *
 * [ ] No unsafe Rust is required.
 *
 * [ ] No runtime execution occurs during parsing.
 *
 * [ ] No hardware discovery occurs during parsing.
 *
 * [ ] Positive tests pass.
 *
 * [ ] Negative tests pass.
 *
 * [ ] Boundary tests pass.
 *
 * [ ] Scalability tests pass within available implementation resources.
 *
 * [ ] Determinism tests pass.
 *
 * [ ] Cross-domain tests pass.
 *
 * [ ] ANTLR generation succeeds.
 *
 * [ ] Rust 1.97+ frontend generation/integration succeeds.
 *
 *
 * ============================================================================
 * FINAL ARCHITECTURAL RULE
 * ============================================================================
 *
 * This file is intentionally an ADAPTER, not another requirement language.
 *
 * The ownership chain is:
 *
 *     core/requirements.g4
 *             |
 *             v
 *     universal requirements
 *             |
 *             +----------------------+
 *                                    |
 *     resources/requirements.g4       |
 *             |                      |
 *             v                      v
 *     resource requirements ---> PolicyRequirements
 *                                    |
 *                                    v
 *                              policies/policy.g4
 *                                    |
 *                                    v
 *                              policy semantics
 *
 *
 * This architecture allows policy requirements to grow with Zamani without
 * creating a separate syntax for every computational domain.
 *
 * It preserves:
 *
 *     domain neutrality;
 *     open-world extensibility;
 *     resource abstraction;
 *     capability abstraction;
 *     POCO-REAF portability;
 *     target independence;
 *     quantum::ir isolation;
 *     safe Rust integration;
 *     scalable source structure.
 *
 * ============================================================================
 */
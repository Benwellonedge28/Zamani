/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/core/capabilities.g4
 *
 * Grammar:
 *     Capabilities
 *
 * Status:
 *     Canonical / production capability syntax
 *
 * Implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *
 * Safety:
 *     Pure ANTLR4 parser grammar.
 *     No embedded Rust.
 *     No semantic predicates.
 *     No runtime execution.
 *     No filesystem access.
 *     No network access.
 *     No hardware discovery.
 *     No unsafe implementation requirement.
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * This file is the canonical source-level syntax authority for Zamani
 * capabilities.
 *
 * A capability identifies an open-ended computational ability, property,
 * facility, semantic feature, execution facility, language feature, or
 * extension contract.
 *
 * Examples:
 *
 *     capability quantum::dynamic_control;
 *     capability quantum::dynamic_control version >= 1.2.0;
 *     capability compute::parallel;
 *     capability tensor::compute;
 *     capability security::trusted_execution;
 *     capability future::computing::new_capability;
 *
 * A capability describes WHAT an environment or semantic system can provide.
 *
 * It does NOT select a particular implementation.
 *
 *
 * OWNS
 * ----
 *
 * This file owns:
 *
 *     capabilityDeclaration
 *     capabilityReference
 *     capabilityName
 *     capabilityExpression
 *     capabilityVersionClause
 *     capabilityVersionExpression
 *     capabilityNameList
 *     capabilityReferenceList
 *     capabilityExpressionList
 *     capabilityAlias
 *     capabilityAliasList
 *     capabilityRequirementReference
 *     capabilityProvisionReference
 *     capabilityPreferenceReference
 *     capabilityConstraintReference
 *     capabilityEffectReference
 *     capabilityTargetReference
 *
 *
 * DOES NOT OWN
 * -------------
 *
 * This file does NOT own:
 *
 *     identifiers
 *     qualified names
 *     lexical tokens
 *     keyword spelling
 *     attributes
 *     version semantics
 *     version compatibility
 *     requirements
 *     resources
 *     resource quantities
 *     constraints
 *     preferences
 *     policies
 *     effects
 *     targets
 *     hardware discovery
 *     device selection
 *     topology
 *     quantum operations
 *     physical qubits
 *     quantum::ir
 *     HDL implementation
 *     routing
 *     scheduling
 *     calibration
 *     QEC
 *     ZQN
 *     runtime authorization
 *     backend selection
 *     execution
 *
 *
 * DEPENDS_ON
 * ----------
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/core/names.g4
 *     grammar/core/attributes.g4
 *     grammar/core/versioning.g4
 *
 *
 * EXPORTS
 * -------
 *
 * Primary:
 *
 *     capabilityDeclaration
 *     capabilityReference
 *     capabilityName
 *     capabilityExpression
 *     capabilityVersionClause
 *     capabilityVersionExpression
 *
 * Secondary reusable:
 *
 *     capabilityNameList
 *     capabilityReferenceList
 *     capabilityExpressionList
 *     capabilityAlias
 *     capabilityAliasList
 *     capabilityRequirementReference
 *     capabilityProvisionReference
 *     capabilityPreferenceReference
 *     capabilityConstraintReference
 *     capabilityEffectReference
 *     capabilityTargetReference
 *
 *
 * CONSUMED_BY
 * -----------
 *
 *     grammar/declarations/capabilities.g4
 *     grammar/core/requirements.g4
 *     grammar/resources/
 *     grammar/effects/
 *     grammar/security/
 *     grammar/policies/
 *     grammar/execution/
 *     grammar/compile/
 *     grammar/classical/
 *     grammar/quantum/
 *     grammar/hybrid/
 *     grammar/hdl/
 *     grammar/hardware/
 *     grammar/distributed/
 *     grammar/networking/
 *     grammar/ai/
 *     grammar/interoperability/
 *     grammar/dialects/
 *     grammar/metaprogramming/
 *
 *
 * AST_OWNER
 * ---------
 *
 * Domain-neutral frontend AST.
 *
 * This grammar produces parser contexts only.
 *
 * The AST should preserve:
 *
 *     capability identity
 *     version requirement
 *     alias information
 *     source span
 *     source ordering
 *
 * It MUST NOT contain physical device identity or target allocation.
 *
 *
 * SEMANTIC_OWNER
 * --------------
 *
 * Capability semantic-resolution subsystem.
 *
 * Semantic analysis determines:
 *
 *     whether a capability exists;
 *     what namespace owns it;
 *     what capability kind it represents;
 *     whether its version requirement is valid;
 *     whether it is available;
 *     whether it conflicts with another capability;
 *     whether it satisfies a requirement;
 *     whether it is authorized;
 *     whether it is realizable by an execution context.
 *
 * None of these decisions occur in this grammar.
 *
 *
 * IR_OWNER
 * --------
 *
 * No direct IR is owned here.
 *
 * Capability information may become semantic metadata consumed by:
 *
 *     canonical semantic representation
 *     classical IR
 *     quantum::ir
 *     HDL/hardware representation
 *     distributed representation
 *     execution planning
 *
 * Capability syntax MUST NOT directly create target instructions.
 *
 *
 * TEST_OWNER
 * ----------
 *
 *     grammar/tests/core/capabilities/
 *     grammar/tests/parser/
 *     grammar/tests/semantic/
 *     grammar/tests/resources/
 *     grammar/tests/quantum/
 *     grammar/tests/hardware/
 *     grammar/tests/scalability/
 *     grammar/tests/portability/
 *
 *
 * SPEC_OWNER
 * ----------
 *
 *     grammar/spec/resources.md
 *     grammar/specification/poco-reaf.md
 *     grammar/specification/grammar-authority.md
 *
 *
 * ============================================================================
 * CAPABILITY / REQUIREMENT / RESOURCE SEPARATION
 * ============================================================================
 *
 * Capability:
 *
 *     What can an environment provide?
 *
 * Requirement:
 *
 *     What does the program require?
 *
 * Resource:
 *
 *     What quantity/facility participates in realization?
 *
 * Constraint:
 *
 *     What condition must hold?
 *
 * Preference:
 *
 *     Which otherwise-valid realization is preferred?
 *
 * Policy:
 *
 *     What is permitted, prohibited, required, or preferred by governing
 *     rules?
 *
 * Target:
 *
 *     Which abstract execution/compilation context is being considered?
 *
 * These concepts MUST remain separate.
 *
 *
 * ============================================================================
 * OPEN-WORLD CONTRACT
 * ============================================================================
 *
 * Capability identities are OPEN-WORLD.
 *
 * This grammar deliberately does NOT enumerate:
 *
 *     CPU capabilities
 *     GPU capabilities
 *     FPGA capabilities
 *     ASIC capabilities
 *     accelerator capabilities
 *     QPU capabilities
 *     simulator capabilities
 *     AI capabilities
 *     networking capabilities
 *     future-domain capabilities
 *     vendor capabilities
 *
 * Therefore:
 *
 *     quantum::dynamic_control
 *     tensor::compute
 *     accelerator::matrix
 *     distributed::collectives
 *     security::trusted_execution
 *     future::architecture::new_feature
 *
 * are all representable without changing this grammar.
 *
 * A new capability normally requires:
 *
 *     registry/specification/semantic changes
 *
 * rather than:
 *
 *     grammar changes.
 *
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Capability syntax MUST remain independent of machine scale.
 *
 * This file contains NO universal hardware capacities.
 *
 * It MUST NOT encode:
 *
 *     maximum qubits
 *     maximum CPUs
 *     maximum GPUs
 *     maximum FPGAs
 *     maximum ASICs
 *     maximum nodes
 *     maximum memory
 *     maximum threads
 *     maximum tensor rank
 *     maximum register width
 *     maximum network size
 *     maximum device count
 *     maximum capability count
 *     maximum namespace depth
 *
 * Capability collections use unbounded grammar repetition.
 *
 * Practical implementation limits are determined by:
 *
 *     available memory
 *     parser resources
 *     compiler resources
 *     operating-system resources
 *     deployment resources
 *
 * Those limits are NOT language semantics.
 *
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Quantum capability names remain ordinary open-world capability identities.
 *
 * Examples:
 *
 *     quantum::measurement
 *     quantum::dynamic_control
 *     quantum::mid_circuit_measurement
 *     quantum::fault_tolerant_execution
 *
 * This file does NOT define:
 *
 *     H
 *     X
 *     Y
 *     Z
 *     CNOT
 *     physical qubits
 *     coupling maps
 *     calibration
 *     routing
 *     scheduling
 *
 * Quantum semantics eventually cross:
 *
 *     source
 *       ->
 *     domain-neutral AST
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
 *     resilience / QEC / ZQN
 *       ->
 *     HAL
 *       ->
 *     target
 *
 * Capability syntax remains completely outside physical quantum realization.
 *
 *
 * ============================================================================
 * HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * Hardware capabilities may be expressed using ordinary names:
 *
 *     hardware::reconfigurable_logic
 *     hardware::parallel_compute
 *     accelerator::tensor_compute
 *     hdl::synthesis
 *
 * This grammar does not select:
 *
 *     FPGA
 *     ASIC
 *     CPU
 *     GPU
 *     QPU
 *     vendor
 *     board
 *     device
 *     pin
 *     topology
 *
 * Hardware realization remains downstream.
 *
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Mentioning a capability has no effect by itself.
 *
 * For example:
 *
 *     capability network::communication;
 *
 * does not authorize network access.
 *
 * Effects are determined by the effect subsystem.
 *
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Capability syntax can identify a resource-related ability, but it does not
 * allocate resources.
 *
 * For example:
 *
 *     tensor::compute
 *
 * can identify a computational capability.
 *
 * It does not imply:
 *
 *     a particular GPU;
 *     a particular accelerator;
 *     a particular tensor size;
 *     a particular memory capacity;
 *     a particular device.
 *
 * Resource analysis resolves those questions.
 *
 *
 * ============================================================================
 * CONTRACT CONTRACT
 * ============================================================================
 *
 * Capability references may occur inside:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *     assert
 *
 * constructs.
 *
 * This file does not own those contract constructs.
 *
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Capability references may participate in policies:
 *
 *     permission
 *     prohibition
 *     requirement
 *     preference
 *     fallback
 *     adaptation
 *     execution
 *
 * Policy interpretation is downstream.
 *
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Capability references must remain source-traceable.
 *
 * Downstream provenance may record:
 *
 *     source capability
 *     resolved capability
 *     provider
 *     version
 *     evidence
 *     decision
 *     target realization
 *
 * This grammar does not create provenance records.
 *
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no actions;
 *     no semantic predicates;
 *     no I/O;
 *     no runtime execution;
 *     no environment inspection;
 *     no hardware inspection;
 *     no random behavior.
 *
 * Parsing therefore depends only on the token stream.
 *
 *
 * ============================================================================
 */

parser grammar Capabilities;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * Parser dependencies:
 *
 * Names:
 *     identifier
 *     qualifiedName
 *
 * Attributes:
 *     attributes
 *
 * Versioning:
 *     versionExpression
 *
 * The capability grammar deliberately reuses these authorities rather than
 * duplicating their syntax.
 */
import Names, Attributes, Versioning;


/* ============================================================================
 * CAPABILITY DECLARATION
 * ========================================================================== */

/*
 * Declares a capability identity.
 *
 * Examples:
 *
 *     capability compute::parallel;
 *
 *     capability quantum::dynamic_control version >= 1.2.0;
 *
 *     capability future::feature version stable;
 *
 * Declaration semantics are handled downstream.
 */
capabilityDeclaration
    : CAPABILITY
      capabilityName
      capabilityVersionClause?
      capabilityAttributeList?
      SEMICOLON
    ;


/* ============================================================================
 * CAPABILITY REFERENCE
 * ========================================================================== */

/*
 * References an existing or externally provided capability.
 *
 * Examples:
 *
 *     compute::parallel
 *     quantum::measurement
 *     future::architecture::feature
 *
 * A reference does not declare, provide, authorize, or allocate anything.
 */
capabilityReference
    : capabilityName
      capabilityVersionClause?
    ;


/* ============================================================================
 * CAPABILITY NAME
 * ========================================================================== */

/*
 * Capability names are canonical qualified names.
 *
 * The meaning of each namespace is semantic, not grammatical.
 */
capabilityName
    : qualifiedName
    ;


/* ============================================================================
 * CAPABILITY VERSION CLAUSE
 * ========================================================================== */

/*
 * The VERSION keyword and version-expression semantics are owned by the
 * canonical versioning subsystem.
 *
 * This wrapper exists to preserve the stable capability API:
 *
 *     capabilityVersionClause
 *
 * without duplicating version grammar.
 */
capabilityVersionClause
    : VERSION capabilityVersionExpression
    ;


/*
 * Compatibility bridge for consumers that need the capability-specific
 * exported rule name.
 *
 * The actual version syntax remains owned by Versioning.
 */
capabilityVersionExpression
    : versionExpression
    ;


/* ============================================================================
 * CAPABILITY ATTRIBUTES
 * ========================================================================== */

/*
 * Attributes remain owned by core/attributes.g4.
 */
capabilityAttributeList
    : attributes
    ;


/* ============================================================================
 * CAPABILITY NAME LIST
 * ========================================================================== */

/*
 * Non-empty list.
 *
 * There is deliberately no finite cardinality limit.
 */
capabilityNameList
    : capabilityName
      (COMMA capabilityName)*
    ;


/* ============================================================================
 * CAPABILITY REFERENCE LIST
 * ========================================================================== */

capabilityReferenceList
    : capabilityReference
      (COMMA capabilityReference)*
    ;


/* ============================================================================
 * CAPABILITY EXPRESSION
 * ========================================================================== */

/*
 * Capability expressions describe logical relationships among capability
 * references.
 *
 * Examples:
 *
 *     quantum::measurement
 *
 *     quantum::measurement and classical::control
 *
 *     quantum::measurement or simulation::quantum
 *
 *     not legacy::feature
 *
 *     (quantum::measurement and classical::control)
 *
 * This grammar captures structure only.
 *
 * It does not evaluate capability availability.
 */
capabilityExpression
    : capabilityOrExpression
    ;


capabilityOrExpression
    : capabilityAndExpression
      (OR capabilityAndExpression)*
    ;


capabilityAndExpression
    : capabilityNotExpression
      (AND capabilityNotExpression)*
    ;


capabilityNotExpression
    : NOT capabilityNotExpression
    | capabilityPrimaryExpression
    ;


capabilityPrimaryExpression
    : capabilityReference
    | LEFT_PAREN capabilityExpression RIGHT_PAREN
    ;


/* ============================================================================
 * CAPABILITY EXPRESSION LIST
 * ========================================================================== */

capabilityExpressionList
    : capabilityExpression
      (COMMA capabilityExpression)*
    ;


/* ============================================================================
 * CAPABILITY ALIAS
 * ========================================================================== */

/*
 * Generic source-level alias.
 *
 * Example:
 *
 *     quantum::dynamic_control as dynamic_control
 *
 * Alias resolution belongs to semantic analysis.
 */
capabilityAlias
    : capabilityReference AS identifier
    ;


capabilityAliasList
    : capabilityAlias
      (COMMA capabilityAlias)*
    ;


/* ============================================================================
 * CAPABILITY CONSUMER BRIDGES
 * ========================================================================== */

/*
 * These rules intentionally remain very small.
 *
 * They provide stable semantic boundaries for consuming grammars without
 * creating duplicate capability grammars.
 */


/*
 * Requirement consumer.
 *
 * This does NOT create a requirement.
 */
capabilityRequirementReference
    : capabilityReference
    ;


/*
 * Provider/provision consumer.
 *
 * This does NOT prove that the capability is actually available.
 */
capabilityProvisionReference
    : capabilityReference
    ;


/*
 * Preference consumer.
 *
 * This does NOT make the capability mandatory.
 */
capabilityPreferenceReference
    : capabilityReference
    ;


/*
 * Constraint consumer.
 *
 * Constraint semantics belong to core/constraints.g4.
 */
capabilityConstraintReference
    : capabilityReference
    ;


/*
 * Effect consumer.
 *
 * Effect semantics belong to effects/.
 */
capabilityEffectReference
    : capabilityReference
    ;


/*
 * Target consumer.
 *
 * A target reference remains abstract.
 */
capabilityTargetReference
    : capabilityReference
    ;


/* ============================================================================
 * CONSUMER LISTS
 * ========================================================================== */

capabilityRequirementList
    : capabilityRequirementReference
      (COMMA capabilityRequirementReference)*
    ;


capabilityProvisionList
    : capabilityProvisionReference
      (COMMA capabilityProvisionReference)*
    ;


capabilityPreferenceList
    : capabilityPreferenceReference
      (COMMA capabilityPreferenceReference)*
    ;


capabilityConstraintList
    : capabilityConstraintReference
      (COMMA capabilityConstraintReference)*
    ;


capabilityEffectList
    : capabilityEffectReference
      (COMMA capabilityEffectReference)*
    ;


capabilityTargetList
    : capabilityTargetReference
      (COMMA capabilityTargetReference)*
    ;


/* ============================================================================
 * OPTIONAL CONSUMER LISTS
 * ========================================================================== */

optionalCapabilityReferenceList
    : capabilityReferenceList?
    ;


optionalCapabilityExpressionList
    : capabilityExpressionList?
    ;


optionalCapabilityNameList
    : capabilityNameList?
    ;


/* ============================================================================
 * COMPLETION / INTEGRATION CONTRACT
 * ============================================================================
 *
 * This file is complete when:
 *
 * [ ] `Capabilities` is the single generic capability syntax authority.
 *
 * [ ] `tokenVocab = ZamaniLexer` remains the parser vocabulary boundary.
 *
 * [ ] `Names` owns identifier and qualified-name syntax.
 *
 * [ ] `Attributes` owns attribute syntax.
 *
 * [ ] `Versioning` owns version-expression syntax.
 *
 * [ ] No capability version grammar is duplicated here.
 *
 * [ ] No identifier grammar is duplicated here.
 *
 * [ ] No qualified-name grammar is duplicated here.
 *
 * [ ] No lexer rules exist here.
 *
 * [ ] Capability identities are open-world.
 *
 * [ ] No domain capability catalogue exists.
 *
 * [ ] No CPU/GPU/FPGA/ASIC/QPU enumeration exists.
 *
 * [ ] No physical device identity is encoded.
 *
 * [ ] No resource quantity is encoded.
 *
 * [ ] No resource capacity limit is encoded.
 *
 * [ ] No topology limit is encoded.
 *
 * [ ] No tensor-rank limit is encoded.
 *
 * [ ] No quantum-operation catalogue is encoded.
 *
 * [ ] No backend selection is encoded.
 *
 * [ ] No scheduling/routing/calibration is encoded.
 *
 * [ ] No quantum::ir dependency exists.
 *
 * [ ] No HDL backend dependency exists.
 *
 * [ ] No semantic actions exist.
 *
 * [ ] No unsafe Rust is required.
 *
 * [ ] Rust 1.97 compatibility is verified by the repository build.
 *
 * [ ] Rust 1.97.1 compatibility is verified by the repository build.
 *
 * [ ] ANTLR generation succeeds.
 *
 * [ ] Declaration integration succeeds.
 *
 * [ ] Requirement integration succeeds.
 *
 * [ ] Resource integration succeeds.
 *
 * [ ] Effect integration succeeds.
 *
 * [ ] Policy integration succeeds.
 *
 * [ ] Security integration succeeds.
 *
 * [ ] Quantum integration succeeds.
 *
 * [ ] Hardware integration succeeds.
 *
 * [ ] HDL integration succeeds.
 *
 * [ ] AI/reasoning integration succeeds.
 *
 * [ ] Distributed/networking integration succeeds.
 *
 * [ ] Interoperability integration succeeds.
 *
 * [ ] Positive tests pass.
 *
 * [ ] Negative tests pass.
 *
 * [ ] Boundary tests pass.
 *
 * [ ] Scalability tests pass.
 *
 * [ ] Determinism tests pass.
 *
 * [ ] Compatibility tests pass.
 *
 *
 * ============================================================================
 * REQUIRED POSITIVE TESTS
 * ============================================================================
 *
 *     capability compute::parallel;
 *
 *     capability quantum::measurement;
 *
 *     capability quantum::dynamic_control;
 *
 *     capability tensor::compute;
 *
 *     capability future::architecture::new_feature;
 *
 *     capability quantum::measurement version >= 1.2.0;
 *
 *     capability quantum::measurement version < 3.0.0;
 *
 *     capability security::trusted_execution version 2.0.0;
 *
 *     compute::parallel
 *
 *     quantum::measurement
 *
 *     quantum::measurement and classical::control
 *
 *     quantum::measurement or simulation::quantum
 *
 *     not legacy::feature
 *
 *     (quantum::measurement and classical::control)
 *
 *     quantum::measurement as measurement
 *
 *
 * ============================================================================
 * REQUIRED NEGATIVE TESTS
 * ============================================================================
 *
 * The parser must reject malformed structures such as:
 *
 *     capability;
 *
 *     capability :: feature;
 *
 *     capability quantum::;
 *
 *     capability ::quantum;
 *
 *     capability quantum::feature as;
 *
 *     capability quantum::feature,;
 *
 *     capability quantum::feature and;
 *
 *     capability and quantum::feature;
 *
 *     capability (quantum::feature;
 *
 *     capability quantum::feature);
 *
 * Exact semantic errors such as unknown capabilities are NOT parser errors.
 *
 *
 * ============================================================================
 * REQUIRED BOUNDARY TESTS
 * ============================================================================
 *
 * Test:
 *
 *     deeply qualified capability names;
 *     deeply nested capability expressions;
 *     large capability lists;
 *     many independent capability declarations;
 *     capability references across domains;
 *     future-domain names;
 *     Unicode identifiers accepted by the canonical lexer;
 *     capability aliases;
 *     version constraints;
 *     version ranges;
 *     capability expressions combined with requirements;
 *     capability expressions combined with policies;
 *     capability expressions combined with contracts.
 *
 * No chosen test size becomes a language-level maximum.
 *
 *
 * ============================================================================
 * REQUIRED CROSS-DOMAIN TESTS
 * ============================================================================
 *
 * At minimum:
 *
 *     classical capability
 *     numerical capability
 *     tensor capability
 *     quantum capability
 *     hybrid capability
 *     HDL capability
 *     hardware capability
 *     accelerator capability
 *     AI capability
 *     reasoning capability
 *     learning capability
 *     adaptation capability
 *     provenance capability
 *     distributed capability
 *     networking capability
 *     security capability
 *     interoperability capability
 *     simulation capability
 *     future-domain capability
 *
 * All must use the same generic capability grammar.
 *
 *
 * ============================================================================
 * FINAL ARCHITECTURAL RULE
 * ============================================================================
 *
 * This grammar answers only:
 *
 *     "What capability syntax did the programmer write?"
 *
 * It does NOT answer:
 *
 *     "Where will this run?"
 *
 *     "Which device will execute it?"
 *
 *     "How many resources are available?"
 *
 *     "Which backend should be selected?"
 *
 *     "How will quantum operations be routed?"
 *
 *     "How will HDL be synthesized?"
 *
 *     "Which accelerator should be used?"
 *
 * Those questions belong downstream.
 *
 * Therefore:
 *
 *     source capability
 *         ->
 *     domain-neutral AST
 *         ->
 *     semantic capability model
 *         ->
 *     capability/resource/requirement/policy analysis
 *         ->
 *     canonical semantic representation
 *         ->
 *     target-independent optimization
 *         ->
 *     lowering
 *         ->
 *     routing/scheduling where applicable
 *         ->
 *     resilience where applicable
 *         ->
 *     ZQN/HAL where applicable
 *         ->
 *     target realization
 *
 * This separation is required for:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * ============================================================================
 */
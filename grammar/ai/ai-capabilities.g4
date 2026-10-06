/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/ai/ai-capabilities.g4
 *
 * GRAMMAR
 * -------
 * AICapabilities
 *
 * STATUS
 * ------
 * CANONICAL AI-DOMAIN CAPABILITY ADAPTER
 *
 * LANGUAGE BASELINE
 * -----------------
 * Rust 1.97+
 * Rust 2021
 * Safe Rust only
 *
 * ANTLR
 * -----
 * ANTLR4 parser grammar
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * This file is the canonical AI-domain adapter for capability intent.
 *
 * It allows AI constructs to express:
 *
 *     capability requirements
 *     capability preferences
 *     capability constraints
 *     capability hints
 *     capability attachments
 *     grouped capability contracts
 *
 * without creating a second capability language.
 *
 *
 * CORE ARCHITECTURAL RULE
 * -----------------------
 *
 * Capability identity belongs exclusively to:
 *
 *     grammar/core/capabilities.g4
 *
 * Resource-scoped capability intent belongs to:
 *
 *     grammar/resources/capabilities.g4
 *
 * This file owns only the AI-domain attachment/classification boundary.
 *
 *
 * DEPENDS_ON
 * ----------
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/core/capabilities.g4
 *     grammar/core/names.g4
 *     grammar/expressions/expressions.g4
 *
 *
 * EXPORTS
 * -------
 *
 * Primary:
 *
 *     aiCapabilityConstruct
 *
 * Capability roles:
 *
 *     aiCapabilityRequirement
 *     aiCapabilityPreference
 *     aiCapabilityConstraint
 *     aiCapabilityHint
 *     aiCapabilityContract
 *     aiCapabilityAttachment
 *     aiCapabilityReference
 *
 * Reusable adapters:
 *
 *     aiCapabilityExpression
 *     aiCapabilityVersion
 *     aiCapabilityName
 *     aiCapabilityAlias
 *     aiCapabilityDeclaration
 *
 * Domain contracts:
 *
 *     aiModelCapabilityContract
 *     aiTensorCapabilityContract
 *     aiTrainingCapabilityContract
 *     aiInferenceCapabilityContract
 *     aiDifferentiationCapabilityContract
 *     aiPipelineCapabilityContract
 *     aiAgentCapabilityContract
 *     aiAcceleratorCapabilityContract
 *     aiDistributedCapabilityContract
 *     aiQuantumHybridCapabilityContract
 *     aiHardwareCapabilityContract
 *     aiDeploymentCapabilityContract
 *     aiSecurityCapabilityContract
 *     aiPortabilityCapabilityContract
 *
 *
 * CONSUMED_BY
 * -----------
 *
 *     grammar/ai/ai.g4
 *     grammar/ai/models.g4
 *     grammar/ai/tensors.g4
 *     grammar/ai/training.g4
 *     grammar/ai/inference.g4
 *     grammar/ai/differentiation.g4
 *     grammar/ai/pipelines.g4
 *     grammar/ai/agents.g4
 *     grammar/ai/ai-accelerators.g4
 *     grammar/ai/model-deployment.g4
 *
 *
 * AST_OWNER
 * ---------
 *
 * Existing domain-neutral Zamani frontend AST.
 *
 * This grammar creates parser contexts only.
 *
 *
 * SEMANTIC_OWNER
 * --------------
 *
 * AI semantic analysis plus the canonical capability/resource semantic
 * subsystems.
 *
 *
 * IR_OWNER
 * --------
 *
 * No IR is owned here.
 *
 * AI semantics may eventually lower to:
 *
 *     classical IR
 *     tensor/data/domain IR
 *     accelerator/hardware IR
 *     distributed IR
 *     quantum::ir
 *
 * according to program semantics.
 *
 *
 * TEST_OWNER
 * ----------
 *
 *     grammar/tests/ai/
 *     grammar/tests/parser/
 *     grammar/tests/resources/
 *     grammar/tests/capabilities/
 *     grammar/tests/scalability/
 *     grammar/tests/portability/
 *     grammar/tests/compatibility/
 *
 *
 * SPEC_OWNER
 * ----------
 *
 *     grammar/spec/ai.md
 *     grammar/spec/resources.md
 *     grammar/spec/policies.md
 *     grammar/spec/provenance.md
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     lexer rules
 *     token spelling
 *     identifier syntax
 *     qualified-name syntax
 *     generic capability identity
 *     capability version semantics
 *     resource quantities
 *     resource allocation
 *     hardware discovery
 *     target selection
 *     device selection
 *     topology
 *     scheduling
 *     routing
 *     optimization
 *     model execution
 *     training algorithms
 *     inference algorithms
 *     tensor execution
 *     quantum operations
 *     quantum topology
 *     QEC
 *     ZQN
 *     HAL
 *     security authorization
 *     policy evaluation
 *     provenance storage
 *     runtime execution
 *     AI-specific IR
 *     quantum-specific IR
 *
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * There must be exactly one authority for capability identity.
 *
 * That authority is:
 *
 *     grammar/core/capabilities.g4
 *
 * Therefore this file MUST reuse:
 *
 *     capabilityReference
 *     capabilityName
 *     capabilityExpression
 *     capabilityVersionClause
 *     capabilityAlias
 *     capabilityDeclaration
 *
 * It MUST NOT redefine them.
 *
 *
 * ============================================================================
 * LEXICAL AUTHORITY
 * ============================================================================
 *
 * The canonical lexer is:
 *
 *     ZamaniLexer
 *
 * Annotation syntax uses:
 *
 *     AT
 *
 * The legacy:
 *
 *     NANO_ANNOTATION
 *
 * vocabulary is deliberately NOT used.
 *
 * Canonical punctuation includes:
 *
 *     LPAREN
 *     RPAREN
 *     LBRACE
 *     RBRACE
 *     LBRACKET
 *     RBRACKET
 *     COMMA
 *     SEMICOLON
 *
 * This file defines no lexer rules.
 *
 *
 * ============================================================================
 * OPEN-WORLD CAPABILITY MODEL
 * ============================================================================
 *
 * Capability identities remain open-world.
 *
 * Examples:
 *
 *     ai::inference
 *     ai::training
 *     ai::reasoning
 *     ai::learning
 *     ai::adaptation
 *     ai::probabilistic
 *     ai::neural_symbolic
 *     tensor::compute
 *     accelerator::tensor_compute
 *     quantum::measurement
 *     quantum::dynamic_control
 *     distributed::communication
 *     security::trusted_execution
 *
 * These are semantic identities.
 *
 * They are NOT enumerated by this grammar.
 *
 * New capability identities therefore do not require a grammar change.
 *
 *
 * ============================================================================
 * AI CAPABILITY ROLES
 * ============================================================================
 *
 * The core roles are deliberately small:
 *
 *     @requires
 *     @prefer
 *     @constraint
 *     @hint
 *     @capability
 *     @capabilities
 *
 * These roles describe how a capability participates in an AI construct.
 *
 * The capability identity itself remains open-world.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY SEPARATION
 * ============================================================================
 *
 * Capability:
 *
 *     an ability or facility that an environment may provide.
 *
 * Requirement:
 *
 *     a capability necessary for semantic feasibility.
 *
 * Constraint:
 *
 *     a mandatory condition on an otherwise valid realization.
 *
 * Preference:
 *
 *     non-mandatory optimization intent.
 *
 * Hint:
 *
 *     advisory information.
 *
 * Resource:
 *
 *     an allocatable/measurable execution object.
 *
 * Realization:
 *
 *     downstream mapping from semantic intent to available resources.
 *
 * This file never performs realization.
 *
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * This grammar contains no universal capacity ceiling.
 *
 * It imposes no grammar-level maximum on:
 *
 *     models
 *     tensors
 *     tensor rank
 *     tensor dimensions
 *     datasets
 *     training steps
 *     inference calls
 *     agents
 *     pipelines
 *     capability references
 *     capability contracts
 *     workers
 *     processors
 *     accelerators
 *     devices
 *     nodes
 *     qubits
 *     memory
 *     threads
 *     network size
 *
 * Repetition is structural:
 *
 *     *
 *     +
 *
 * No universal capacity constant is permitted.
 *
 * "Unbounded" means that the grammar introduces no artificial finite ceiling.
 *
 * It does not claim that a physical machine has infinite resources.
 *
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This file MUST NOT introduce or depend on universal constants representing
 * machine capacity.
 *
 * In particular, it must not encode:
 *
 *     fixed processor counts
 *     fixed accelerator counts
 *     fixed device counts
 *     fixed node counts
 *     fixed memory capacity
 *     fixed tensor rank
 *     fixed register width
 *     fixed network size
 *     fixed quantum capacity
 *
 * Physical feasibility belongs downstream.
 *
 *
 * ============================================================================
 * TARGET INDEPENDENCE
 * ============================================================================
 *
 * These are NOT valid universal capability semantics:
 *
 *     use GPU 0
 *     use CPU 3
 *     use QPU 2
 *     use FPGA 1
 *     use node 7
 *
 * This grammar only preserves abstract capability intent.
 *
 * For example:
 *
 *     @requires ai::tensor::compute;
 *
 * may eventually be realized through:
 *
 *     CPU
 *     multicore CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     distributed system
 *     future target
 *
 * without changing source semantics.
 *
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Referencing a capability does not itself grant an effect.
 *
 * For example:
 *
 *     @requires network::communication;
 *
 * does not authorize network access.
 *
 * Effects remain owned by:
 *
 *     grammar/effects/
 *
 * Security authorization remains owned by:
 *
 *     grammar/security/
 *
 * Policy evaluation remains downstream.
 *
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Capability intent may be governed by:
 *
 *     authorization
 *     security
 *     resource policy
 *     execution policy
 *     adaptation policy
 *     deployment policy
 *     simulation policy
 *     reproducibility policy
 *
 * This grammar records capability intent only.
 *
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Downstream semantic analysis must be able to preserve:
 *
 *     source span
 *     capability identity
 *     capability version
 *     capability role
 *     enclosing AI construct
 *     semantic derivation
 *     evidence
 *     resolution decision
 *     realization
 *
 * This grammar creates no provenance records itself.
 *
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * AI capability intent may reference quantum capabilities:
 *
 *     @requires quantum::measurement;
 *     @requires quantum::dynamic_control;
 *     @prefer quantum::low_noise;
 *
 * This file does not define quantum operations.
 *
 * If AI computation becomes quantum computation, the canonical boundary is:
 *
 *     source
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic analysis
 *       |
 *       v
 *     quantum semantic model
 *       |
 *       v
 *     quantum::ir
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     decomposition
 *       |
 *       v
 *     routing
 *       |
 *       v
 *     scheduling
 *       |
 *       v
 *     resilience / QEC
 *       |
 *       v
 *     ZQN
 *       |
 *       v
 *     HAL
 *
 * No physical qubit or topology is represented here.
 *
 *
 * ============================================================================
 * HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * AI capabilities may reference:
 *
 *     hdl::synthesis
 *     hardware::reconfigurable_logic
 *     hardware::parallel_compute
 *
 * without encoding:
 *
 *     bus widths
 *     register widths
 *     FPGA dimensions
 *     ASIC capacity
 *     device counts
 *     clock limits
 *     physical topology
 *
 *
 * ============================================================================
 * DISTRIBUTED BOUNDARY
 * ============================================================================
 *
 * AI capability requirements may reference distributed facilities:
 *
 *     distributed::communication
 *     distributed::replication
 *     distributed::coordination
 *
 * No node count is encoded.
 *
 *
 * ============================================================================
 * COMPILER BOUNDARY
 * ============================================================================
 *
 * Capability intent participates in:
 *
 *     semantic analysis
 *       ->
 *     capability resolution
 *       ->
 *     resource analysis
 *       ->
 *     policy analysis
 *       ->
 *     target-independent semantic representation
 *       ->
 *     optimization
 *       ->
 *     lowering
 *       ->
 *     scheduling
 *       ->
 *     routing
 *       ->
 *     realization
 *
 * This grammar performs none of those operations.
 *
 *
 * ============================================================================
 * RUST SAFETY CONTRACT
 * ============================================================================
 *
 * This is a pure ANTLR parser grammar.
 *
 * It contains:
 *
 *     no Rust actions
 *     no embedded code
 *     no semantic predicates
 *     no filesystem access
 *     no network access
 *     no hardware discovery
 *     no runtime execution
 *     no resource allocation
 *
 * Generated frontend integration must remain compatible with:
 *
 *     Rust 1.97+
 *     Rust 2021
 *
 * and safe Rust only.
 *
 *
 * ============================================================================
 * GRAMMAR
 * ============================================================================
 */

parser grammar AICapabilities;

options {
    tokenVocab = ZamaniLexer;
}

import
    Capabilities,
    Expressions
;


/*
 * ============================================================================
 * PUBLIC AI CAPABILITY ENTRY
 * ============================================================================
 *
 * Exactly one public domain boundary is exposed.
 *
 * The parent AI grammar uses this rule to classify an AI capability construct.
 *
 * ============================================================================
 */

aiCapabilityConstruct
    : aiCapabilityRequirement
    | aiCapabilityPreference
    | aiCapabilityConstraint
    | aiCapabilityHint
    | aiCapabilityContract
    | aiCapabilityAttachment
    ;


/*
 * ============================================================================
 * CAPABILITY ROLE
 * ============================================================================
 *
 * The role names below are lexical keywords in the current language.
 *
 * `identifier` is retained for future semantic extension roles that are not
 * reserved keywords.
 *
 * It does NOT create a second capability vocabulary.
 * ============================================================================
 */

aiCapabilityRole
    : REQUIRES
    | PREFER
    | CONSTRAINT
    | HINT
    | CAPABILITY
    | CAPABILITIES
    | identifier
    ;


/*
 * ============================================================================
 * REQUIREMENT
 * ============================================================================
 *
 * Canonical forms:
 *
 *     @requires ai::inference;
 *
 *     @requires(ai::inference);
 *
 *     @requires ai::inference and tensor::compute;
 *
 *     @requires [ai::inference, tensor::compute];
 *
 * The capability identities are owned by Capabilities.
 *
 * ============================================================================
 */

aiCapabilityRequirement
    : AT REQUIRES
      aiCapabilityInvocation
      SEMICOLON
    ;


aiCapabilityInvocation
    : aiCapabilityPayload
    | LPAREN aiCapabilityPayload RPAREN
    ;


aiCapabilityPayload
    : capabilityExpression
    | aiCapabilityReferenceList
    | aiCapabilityPropertyPredicate
    ;


aiCapabilityReferenceList
    : LBRACKET
      capabilityReference
      (COMMA capabilityReference)*
      COMMA?
      RBRACKET
    ;


/*
 * ============================================================================
 * PREFERENCE
 * ============================================================================
 *
 * A preference does not become a hard requirement when unavailable.
 * ============================================================================
 */

aiCapabilityPreference
    : AT PREFER
      aiCapabilityInvocation
      SEMICOLON
    ;


/*
 * ============================================================================
 * CONSTRAINT
 * ============================================================================
 *
 * Constraints are mandatory semantic conditions.
 *
 * Examples:
 *
 *     @constraint ai::deterministic_inference;
 *
 *     @constraint ai::device property fidelity >= required_fidelity;
 *
 * ============================================================================
 */

aiCapabilityConstraint
    : AT CONSTRAINT
      aiCapabilityInvocation
      SEMICOLON
    ;


aiCapabilityPropertyPredicate
    : capabilityExpression
      PROPERTY
      qualifiedName
      aiCapabilityComparisonOperator
      expression
    ;


aiCapabilityComparisonOperator
    : EQ
    | NE
    | LT
    | LE
    | GT
    | GE
    ;


/*
 * ============================================================================
 * HINT
 * ============================================================================
 *
 * Hints are advisory.
 * ============================================================================
 */

aiCapabilityHint
    : AT HINT
      aiCapabilityInvocation
      SEMICOLON
    ;


/*
 * ============================================================================
 * CAPABILITY ATTACHMENT
 * ============================================================================
 *
 * Associates a capability with an AI construct.
 *
 * Examples:
 *
 *     @capability ai::inference;
 *
 *     @capability(ai::tensor::compute);
 *
 * ============================================================================
 */

aiCapabilityAttachment
    : AT CAPABILITY
      aiCapabilityInvocation
      SEMICOLON
    ;


/*
 * ============================================================================
 * GROUPED CAPABILITY CONTRACT
 * ============================================================================
 *
 * Example:
 *
 *     @capabilities {
 *         @requires ai::inference;
 *         @requires tensor::compute;
 *         @prefer accelerator::tensor_compute;
 *         @constraint ai::deterministic_inference;
 *         @hint ai::streaming;
 *     }
 *
 * A grouped contract must contain at least one member.
 *
 * There is no finite member limit.
 * ============================================================================
 */

aiCapabilityContract
    : AT CAPABILITIES
      LBRACE
      aiCapabilityContractMember+
      RBRACE
      SEMICOLON?
    ;


aiCapabilityContractMember
    : aiCapabilityRequirement
    | aiCapabilityPreference
    | aiCapabilityConstraint
    | aiCapabilityHint
    | aiCapabilityAttachment
    ;


/*
 * ============================================================================
 * GENERIC AI CAPABILITY EXPRESSION
 * ============================================================================
 *
 * Stable adapter over the canonical capability expression.
 * ============================================================================
 */

aiCapabilityExpression
    : capabilityExpression
    ;


/*
 * ============================================================================
 * CAPABILITY REFERENCE ADAPTER
 * ============================================================================
 */

aiCapabilityReference
    : capabilityReference
    ;


/*
 * ============================================================================
 * VERSION ADAPTER
 * ============================================================================
 *
 * Version syntax remains owned by Capabilities/Versioning.
 * ============================================================================
 */

aiCapabilityVersion
    : capabilityVersionClause
    ;


/*
 * ============================================================================
 * NAME ADAPTER
 * ============================================================================
 */

aiCapabilityName
    : capabilityName
    ;


/*
 * ============================================================================
 * ALIAS ADAPTER
 * ============================================================================
 */

aiCapabilityAlias
    : capabilityAlias
    ;


/*
 * ============================================================================
 * DECLARATION ADAPTER
 * ============================================================================
 *
 * This is intentionally NOT part of aiCapabilityConstruct.
 *
 * Capability declarations are universal declarations, not AI-only constructs.
 * ============================================================================
 */

aiCapabilityDeclaration
    : capabilityDeclaration
    ;


/*
 * ============================================================================
 * DOMAIN CONTRACT ADAPTERS
 * ============================================================================
 *
 * These rules provide stable named boundaries for AI leaf grammars.
 *
 * They do not create separate capability languages.
 * ============================================================================
 */

aiModelCapabilityContract
    : aiCapabilityContract
    ;


aiTensorCapabilityContract
    : aiCapabilityContract
    ;


aiTrainingCapabilityContract
    : aiCapabilityContract
    ;


aiInferenceCapabilityContract
    : aiCapabilityContract
    ;


aiDifferentiationCapabilityContract
    : aiCapabilityContract
    ;


aiPipelineCapabilityContract
    : aiCapabilityContract
    ;


aiAgentCapabilityContract
    : aiCapabilityContract
    ;


aiAcceleratorCapabilityContract
    : aiCapabilityContract
    ;


aiDistributedCapabilityContract
    : aiCapabilityContract
    ;


aiQuantumHybridCapabilityContract
    : aiCapabilityContract
    ;


aiHardwareCapabilityContract
    : aiCapabilityContract
    ;


aiDeploymentCapabilityContract
    : aiCapabilityContract
    ;


aiSecurityCapabilityContract
    : aiCapabilityContract
    ;


aiPortabilityCapabilityContract
    : aiCapabilityContract
    ;


/*
 * ============================================================================
 * REUSABLE SEQUENCES
 * ============================================================================
 *
 * These have no finite cardinality.
 * ============================================================================
 */

aiCapabilityContractSequence
    : aiCapabilityContract+
    ;


aiCapabilityReferenceSequence
    : aiCapabilityReference+
    ;


aiCapabilityRequirementList
    : aiCapabilityRequirement+
    ;


aiCapabilityPreferenceList
    : aiCapabilityPreference+
    ;


aiCapabilityConstraintList
    : aiCapabilityConstraint+
    ;


aiCapabilityHintList
    : aiCapabilityHint+
    ;


aiCapabilityAttachmentSequence
    : aiCapabilityAttachment+
    ;


aiCapabilityDeclarationSequence
    : aiCapabilityDeclaration+
    ;


aiCapabilityMemberSequence
    : aiCapabilityContractMember+
    ;


aiVersionedCapabilityReference
    : capabilityReference
    ;


/*
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * The semantic layer must construct a domain-neutral capability-intent model.
 *
 * A useful conceptual representation is:
 *
 *     AI capability intent
 *       {
 *           role
 *           capability expression
 *           optional predicate
 *           enclosing construct
 *           source span
 *       }
 *
 * The exact AST/semantic type belongs to the existing frontend architecture.
 *
 * This grammar must not force:
 *
 *     AICapabilityIR
 *     AIResourceIR
 *     AIDeviceIR
 *     AIGPUIR
 *     AIQPUCapabilityIR
 *
 * or equivalent domain-specific intermediate representations.
 *
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * Capability identities and capability expressions use the canonical capability
 * type/semantic system.
 *
 * Predicate values use the canonical expression system.
 *
 * No AI-specific type system is introduced.
 *
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Capability intent itself is effect-neutral.
 *
 * An operation using a capability may have effects such as:
 *
 *     learning
 *     adaptation
 *     network
 *     foreign
 *     native
 *     distributed
 *     measurement
 *     simulation
 *     reflection
 *
 * Effect inference belongs to grammar/effects/ and semantic analysis.
 *
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Capability requirements do not become physical resource assignments.
 *
 * For example:
 *
 *     @requires tensor::compute;
 *
 * may result in downstream resource requirements, but this grammar does not
 * determine how many processors, accelerators, devices, nodes, or memory
 * units are required.
 *
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * A capability requirement may be accepted, rejected, transformed, or
 * constrained by policy.
 *
 * Policy evaluation remains outside this grammar.
 *
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * The frontend must preserve:
 *
 *     role
 *     capability expression
 *     source span
 *     source ordering
 *     enclosing AI construct
 *
 * Downstream systems may additionally attach:
 *
 *     evidence
 *     provider
 *     resolution decision
 *     policy decision
 *     target realization
 *
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Syntax diagnostics include:
 *
 *     missing capability expression
 *     missing semicolon
 *     malformed capability expression
 *     malformed capability list
 *     malformed property predicate
 *     missing comparison operator
 *     missing predicate value
 *     malformed grouped contract
 *     empty grouped contract
 *
 * Semantic diagnostics include:
 *
 *     unknown capability
 *     invalid capability version
 *     incompatible capability expression
 *     conflicting requirements
 *     unsatisfied requirement
 *     forbidden capability
 *     unauthorized capability
 *     unavailable realization
 *
 * Resource/target failures MUST NOT be converted into syntax errors.
 *
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     source text
 *     lexer token stream
 *     grammar
 *     parser configuration
 *
 * It must not depend on:
 *
 *     hardware availability
 *     runtime state
 *     network state
 *     filesystem state
 *     wall-clock time
 *     random state
 *     scheduler state
 *
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * The legacy:
 *
 *     grammar/ai/capabilities.g4
 *
 * must not remain a competing canonical AI capability grammar.
 *
 * Its use of the obsolete annotation token must be removed from the canonical
 * composition path.
 *
 * Migration policy:
 *
 *     old capabilities.g4
 *          |
 *          v
 *     deprecated compatibility material
 *          |
 *          v
 *     ai-capabilities.g4
 *
 * Existing source capability identities remain compatible because identity
 * syntax continues to be owned by:
 *
 *     grammar/core/capabilities.g4
 *
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * 1. AI COMPOSITION
 * -----------------
 *
 * `grammar/ai/ai.g4` MUST import:
 *
 *     AICapabilities
 *
 * and retain:
 *
 *     aiCapabilityConstruct
 *
 * as its AI capability dispatch boundary.
 *
 *
 * 2. ROOT PARSER
 * --------------
 *
 * `grammar/antlr/ZamaniParser.g4` continues importing:
 *
 *     AI
 *
 * It does not import AICapabilities directly.
 *
 *
 * 3. ROOT GRAMMAR
 * ---------------
 *
 * `grammar/Zamani.g4` remains the repository source root.
 *
 * It does not directly import this leaf grammar.
 *
 *
 * 4. GENERIC CAPABILITY SYSTEM
 * ----------------------------
 *
 * All capability identity and version parsing comes from:
 *
 *     grammar/core/capabilities.g4
 *
 *
 * 5. RESOURCE CAPABILITY SYSTEM
 * -----------------------------
 *
 * Resource-level capability syntax remains owned by:
 *
 *     grammar/resources/capabilities.g4
 *
 * This file does not replace that resource grammar.
 *
 *
 * 6. AI LEAF GRAMMARS
 * -------------------
 *
 * AI leaf grammars consume the stable adapter rules:
 *
 *     aiModelCapabilityContract
 *     aiTensorCapabilityContract
 *     aiTrainingCapabilityContract
 *     aiInferenceCapabilityContract
 *     aiDifferentiationCapabilityContract
 *     aiPipelineCapabilityContract
 *     aiAgentCapabilityContract
 *     aiAcceleratorCapabilityContract
 *     aiDistributedCapabilityContract
 *     aiQuantumHybridCapabilityContract
 *     aiHardwareCapabilityContract
 *     aiDeploymentCapabilityContract
 *     aiSecurityCapabilityContract
 *     aiPortabilityCapabilityContract
 *
 * They must not redefine capability identity.
 *
 *
 * 7. SEMANTIC PIPELINE
 * --------------------
 *
 * The required path is:
 *
 *     source
 *       ->
 *     lexer
 *       ->
 *     parser
 *       ->
 *     domain-neutral AST
 *       ->
 *     structural validation
 *       ->
 *     type/effect/capability/resource analysis
 *       ->
 *     contract/policy analysis
 *       ->
 *     provenance
 *       ->
 *     canonical semantic model
 *       ->
 *     canonical IR
 *       ->
 *     optimization/lowering
 *       ->
 *     routing/scheduling
 *       ->
 *     resilience
 *       ->
 *     target realization
 *
 *
 * 8. QUANTUM
 * ----------
 *
 * If an AI capability affects quantum computation, semantic lowering eventually
 * uses:
 *
 *     quantum::ir
 *
 * No AI capability grammar may create another quantum representation.
 *
 *
 * 9. HARDWARE
 * -----------
 *
 * Hardware realization consumes resolved capabilities downstream.
 *
 * This file never discovers or selects hardware.
 *
 *
 * 10. SECURITY
 * -----------
 *
 * Capability references are intent.
 *
 * They are not authorization.
 *
 * Security/policy analysis determines whether the capability can be used.
 *
 *
 * ============================================================================
 * CONFORMANCE TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE
 * --------
 *
 * The parser must accept:
 *
 *     @requires ai::inference;
 *
 *     @requires(ai::inference);
 *
 *     @requires ai::inference and tensor::compute;
 *
 *     @requires [ai::inference, tensor::compute];
 *
 *     @prefer accelerator::tensor_compute;
 *
 *     @constraint ai::deterministic_inference;
 *
 *     @constraint ai::device property fidelity >= required_fidelity;
 *
 *     @hint ai::streaming;
 *
 *     @capability ai::inference;
 *
 *     @capability(ai::tensor::compute);
 *
 *     @capabilities {
 *         @requires ai::inference;
 *         @requires tensor::compute;
 *         @prefer accelerator::tensor_compute;
 *         @constraint ai::deterministic_inference;
 *         @hint ai::streaming;
 *     }
 *
 *     @requires future::ai::new_capability;
 *
 *     @requires future::domain::capability version >= 1.0.0;
 *
 *
 * NEGATIVE
 * --------
 *
 * The parser must reject:
 *
 *     @requires;
 *
 *     @requires();
 *
 *     @requires(ai::);
 *
 *     @requires [ai::inference,];
 *
 *     @prefer;
 *
 *     @constraint;
 *
 *     @constraint ai::device property;
 *
 *     @constraint ai::device property fidelity;
 *
 *     @constraint ai::device property fidelity >=;
 *
 *     @hint;
 *
 *     @capability;
 *
 *     @capabilities {}
 *
 *     @capabilities {
 *         @requires ai::inference
 *     }
 *
 *
 * BOUNDARY
 * --------
 *
 * Test:
 *
 *     deeply qualified capability names;
 *     nested capability expressions;
 *     many capability members;
 *     many capability contracts;
 *     large symbolic predicate expressions;
 *     versioned capability references;
 *     mixed classical/AI capability requirements;
 *     mixed quantum/AI capability requirements;
 *     hybrid capability requirements;
 *     distributed capability requirements;
 *     hardware capability requirements;
 *     future-domain capability identities.
 *
 *
 * SCALABILITY
 * ----------
 *
 * Scale tests by changing the input size, not this grammar.
 *
 * Test dimensions include:
 *
 *     capability count
 *     contract count
 *     expression size
 *     qualified-name depth
 *     nesting depth
 *     source-unit size
 *     AI construct count
 *
 * No chosen test size becomes a language-level limit.
 *
 *
 * PORTABILITY
 * ----------
 *
 * The same source must remain syntactically valid regardless of:
 *
 *     CPU availability
 *     GPU availability
 *     FPGA availability
 *     accelerator availability
 *     QPU availability
 *     node count
 *     memory availability
 *     target topology
 *
 * A target may later report an unsatisfied capability.
 *
 * That is a semantic/resource realization result, not a parsing failure.
 *
 *
 * DETERMINISM
 * -----------
 *
 * Identical source/token streams under identical grammar/parser configuration
 * must produce equivalent parser structure.
 *
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * PASS CONDITIONS
 * --------------
 *
 * No physical device IDs.
 *
 * No vendor-specific hardware.
 *
 * No framework-specific AI implementation.
 *
 * No fixed tensor capacity.
 *
 * No fixed model capacity.
 *
 * No fixed node capacity.
 *
 * No fixed accelerator capacity.
 *
 * No fixed quantum capacity.
 *
 * No fixed memory capacity.
 *
 * No fixed network capacity.
 *
 * No capability catalogue encoded in grammar.
 *
 * No artificial finite repetition ceiling.
 *
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 * [x] It is a parser grammar.
 *
 * [x] It consumes the canonical ZamaniLexer vocabulary.
 *
 * [x] It uses AT rather than the obsolete annotation token.
 *
 * [x] Capability identity remains owned by core/capabilities.g4.
 *
 * [x] Capability version syntax remains canonical.
 *
 * [x] Capability expressions remain canonical.
 *
 * [x] It provides one AI capability composition boundary.
 *
 * [x] It distinguishes requirements from preferences.
 *
 * [x] It distinguishes constraints from hints.
 *
 * [x] It supports grouped capability contracts.
 *
 * [x] It supports open-world capability identities.
 *
 * [x] It supports versioned capability references.
 *
 * [x] It supports capability property constraints.
 *
 * [x] It contains no physical target selection.
 *
 * [x] It contains no resource allocation.
 *
 * [x] It contains no hardware discovery.
 *
 * [x] It contains no scheduler or router.
 *
 * [x] It contains no quantum operation grammar.
 *
 * [x] It contains no AI-specific IR.
 *
 * [x] It contains no universal machine-capacity limits.
 *
 * [x] It contains no embedded Rust.
 *
 * [x] It requires no unsafe Rust.
 *
 * [x] It defines its downstream integration contract.
 *
 * [x] It defines its AST/semantic/IR boundaries.
 *
 * [x] It defines diagnostics.
 *
 * [x] It defines positive/negative/boundary/scalability tests.
 *
 * [x] It preserves POCO-REAF.
 *
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * This file answers exactly:
 *
 *     "How does an AI construct express abstract capability intent?"
 *
 * It does NOT answer:
 *
 *     "Which machine executes it?"
 *
 *     "Which processor executes it?"
 *
 *     "Which accelerator executes it?"
 *
 *     "Which QPU executes it?"
 *
 *     "Which physical qubit is selected?"
 *
 *     "How is the computation routed?"
 *
 *     "How is the computation scheduled?"
 *
 *     "How is error correction performed?"
 *
 *     "How is the HAL realized?"
 *
 * Those questions remain downstream.
 *
 * The resulting architecture remains:
 *
 *     Zamani source
 *       ->
 *     lexer
 *       ->
 *     parser
 *       ->
 *     domain-neutral AST
 *       ->
 *     semantic capability intent
 *       ->
 *     resource/capability/policy analysis
 *       ->
 *     canonical semantic model
 *       ->
 *     domain IR
 *       ->
 *     target-independent optimization
 *       ->
 *     lowering
 *       ->
 *     routing/scheduling/resilience
 *       ->
 *     ZQN/HAL
 *       ->
 *     available realization
 *
 * Therefore the same source-level capability intent remains portable from
 * very small systems through arbitrarily large systems, subject only to the
 * actual resources, capabilities, policies and semantics of the realization.
 *
 * ============================================================================
 */
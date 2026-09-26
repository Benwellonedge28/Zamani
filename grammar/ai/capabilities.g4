/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/ai/capabilities.g4
 *
 * Grammar:
 *     AICapabilities
 *
 * Status:
 *     CANONICAL AI-DOMAIN CAPABILITY ADAPTER GRAMMAR
 *
 * Language/runtime baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * Safety:
 *     - Grammar only.
 *     - No embedded Rust actions.
 *     - No semantic predicates.
 *     - No filesystem access.
 *     - No network access.
 *     - No hardware discovery.
 *     - No runtime execution.
 *     - No unsafe implementation.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file defines the AI-domain SOURCE SYNTAX for capability intent.
 *
 * It is an ADAPTER over the canonical capability system.
 *
 * The canonical capability authority remains:
 *
 *     grammar/core/capabilities.g4
 *
 * This file MUST NOT create a second capability identity system.
 *
 * AI capabilities describe capabilities required, preferred, constrained,
 * hinted, declared, or referenced by AI computations.
 *
 * Examples of semantic capability identities include:
 *
 *     ai::inference
 *     ai::training
 *     ai::tensor::compute
 *     ai::tensor::dynamic_shape
 *     ai::differentiation
 *     ai::distributed_training
 *     ai::model_serving
 *     ai::quantization
 *     ai::probabilistic
 *     ai::quantum_hybrid
 *     accelerator::tensor_compute
 *
 * These names are EXAMPLES OF SEMANTIC IDENTITIES.
 *
 * They are deliberately NOT enumerated by this grammar.
 *
 * Future capability identities remain legal without modifying this file.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *                         ZAMANI SOURCE
 *                              |
 *                              v
 *                         ZamaniLexer
 *                              |
 *                              v
 *                         ZamaniParser
 *                              |
 *                              v
 *                    AI capability syntax
 *                              |
 *                              v
 *                        Frontend AST
 *                              |
 *                              v
 *                      Semantic analysis
 *                              |
 *               +--------------+--------------+
 *               |              |              |
 *               v              v              v
 *          AI semantics   capabilities    resources
 *               |              |              |
 *               +--------------+--------------+
 *                              |
 *                              v
 *                    canonical semantic model
 *                              |
 *          +-------------------+-------------------+
 *          |                   |                   |
 *          v                   v                   v
 *      Classical           quantum::ir        HDL/Hardware
 *          |                   |                   |
 *          +-------------------+-------------------+
 *                              |
 *                              v
 *                         compilation
 *                              |
 *                    optimization / lowering
 *                              |
 *                    routing / scheduling
 *                              |
 *                    resilience / QEC / ZQN
 *                              |
 *                              v
 *                             HAL
 *                              |
 *                              v
 *                       target realization
 *                              |
 *                              v
 *                           runtime
 *
 * This grammar participates only in the source syntax portion.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - AI capability declaration adapters;
 *     - AI capability references;
 *     - AI capability requirement syntax;
 *     - AI capability preference syntax;
 *     - AI capability constraint syntax;
 *     - AI capability hint syntax;
 *     - AI capability contract grouping;
 *     - AI capability attachment syntax;
 *     - AI-specific syntactic classification of capability intent.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - capability identity;
 *     - generic capability declaration semantics;
 *     - generic capability version semantics;
 *     - capability registry;
 *     - capability discovery;
 *     - resource discovery;
 *     - hardware discovery;
 *     - target selection;
 *     - placement;
 *     - routing;
 *     - scheduling;
 *     - optimization;
 *     - runtime capability state;
 *     - authorization;
 *     - authentication;
 *     - security enforcement;
 *     - quantum IR;
 *     - QEC;
 *     - ZQN;
 *     - compiler implementation;
 *     - runtime implementation.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * Capability identity and generic capability syntax are owned by:
 *
 *     grammar/core/capabilities.g4
 *
 * AI capability syntax MUST reuse:
 *
 *     capabilityReference
 *     capabilityName
 *     capabilityVersionClause
 *     capabilityAlias
 *
 * where those rules are applicable.
 *
 * AI capability syntax MUST NOT redefine those rules.
 *
 * This file therefore adds AI CONTEXT, not a second capability language.
 *
 * ============================================================================
 * WHY THIS FILE EXISTS
 * ============================================================================
 *
 * A generic capability declaration answers:
 *
 *     "What capability exists or is being referenced?"
 *
 * An AI capability contract answers:
 *
 *     "How does an AI construct use that capability?"
 *
 * Examples:
 *
 *     capability ai::inference;
 *
 * versus:
 *
 *     @requires ai::inference;
 *
 * versus:
 *
 *     @prefer accelerator::tensor_compute;
 *
 * versus:
 *
 *     @constraint ai::deterministic_inference;
 *
 * The identity remains canonical.
 *
 * The AI context belongs here.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * This grammar imposes NO universal limits on:
 *
 *     models
 *     tensors
 *     parameters
 *     layers
 *     datasets
 *     inference requests
 *     training steps
 *     agents
 *     pipelines
 *     workers
 *     accelerators
 *     devices
 *     CPUs
 *     GPUs
 *     FPGAs
 *     QPUs
 *     nodes
 *     memory
 *     storage
 *     tensor dimensions
 *     tensor rank
 *     capability count
 *     capability-reference count
 *     deployment count
 *     concurrency
 *     throughput
 *
 * It MUST NOT introduce:
 *
 *     MAX_AI_CAPABILITIES
 *     MAX_MODELS
 *     MAX_TENSORS
 *     MAX_LAYERS
 *     MAX_PARAMETERS
 *     MAX_DEVICES
 *     MAX_ACCELERATORS
 *     MAX_GPUS
 *     MAX_CPUS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_TENSOR_RANK
 *     MAX_CONCURRENCY
 *
 * or equivalent artificial ceilings.
 *
 * Repetition is expressed structurally using:
 *
 *     *
 *     +
 *
 * Program-provided quantities remain program semantics.
 *
 * Hardware/environment capacity remains downstream resource state.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY SEPARATION
 * ============================================================================
 *
 * AI capability syntax MUST preserve the distinction between:
 *
 *     capability
 *     requirement
 *     constraint
 *     preference
 *     hint
 *     resource
 *     realization
 *
 * Capability:
 *
 *     what an execution environment can provide.
 *
 * Requirement:
 *
 *     what the AI computation requires.
 *
 * Constraint:
 *
 *     what conditions a valid realization must satisfy.
 *
 * Preference:
 *
 *     what valid realization is preferred.
 *
 * Hint:
 *
 *     optional optimization information.
 *
 * Resource:
 *
 *     measurable or allocatable execution capacity.
 *
 * Realization:
 *
 *     the downstream decision that maps intent to actual hardware/runtime
 *     resources.
 *
 * This grammar never performs that realization.
 *
 * ============================================================================
 * TARGET INDEPENDENCE
 * ============================================================================
 *
 * The following must NOT become required source syntax:
 *
 *     GPU 0
 *     CPU 3
 *     accelerator 7
 *     QPU 2
 *     node 11
 *     device 4
 *
 * Nor may this grammar encode:
 *
 *     RAM = 64GB
 *     VRAM = 24GB
 *     register = 32bit
 *     tensor rank <= N
 *     devices <= N
 *
 * Portable source may instead express semantic intent such as:
 *
 *     @requires ai::tensor::compute;
 *
 *     @requires accelerator::tensor_compute;
 *
 *     @prefer accelerator::low_latency;
 *
 *     @constraint ai::deterministic_inference;
 *
 * The semantic/resource layers determine whether a target satisfies that
 * intent.
 *
 * ============================================================================
 * OPEN-WORLD CAPABILITY MODEL
 * ============================================================================
 *
 * Capability identities are open-world.
 *
 * This grammar MUST NOT enumerate:
 *
 *     aiCapability
 *     gpuCapability
 *     cpuCapability
 *     npuCapability
 *     tpuCapability
 *     fpgaCapability
 *     qpuCapability
 *
 * as closed alternatives.
 *
 * A capability is represented through the canonical capability reference.
 *
 * Therefore future identities such as:
 *
 *     future::ai::new_acceleration
 *     future::ai::new_training_mode
 *     future::quantum::ai_hybrid
 *
 * do not require a grammar change.
 *
 * ============================================================================
 * AI CAPABILITY CATEGORIES
 * ============================================================================
 *
 * The following are SEMANTIC CATEGORIES, not grammar enumerations:
 *
 *     model
 *     tensor
 *     training
 *     inference
 *     differentiation
 *     optimization
 *     probabilistic
 *     symbolic
 *     neural
 *     agent
 *     pipeline
 *     deployment
 *     distributed
 *     accelerator
 *     quantum-hybrid
 *     hardware
 *     interoperability
 *     security
 *
 * Semantic analysis may classify a capability into one or more categories.
 *
 * The parser preserves the source identity.
 *
 * ============================================================================
 * ANNOTATION POLICY
 * ============================================================================
 *
 * AI capability attachment uses the canonical annotation token:
 *
 *     NANO_ANNOTATION
 *
 * Examples:
 *
 *     @requires ai::inference;
 *     @prefer accelerator::tensor_compute;
 *     @constraint ai::deterministic;
 *     @hint ai::batching;
 *
 * The exact annotation spelling remains semantic data.
 *
 * This avoids adding a new lexer keyword for every future AI capability
 * category.
 *
 * The grammar nevertheless gives the major capability roles explicit
 * structural alternatives so malformed contracts are rejected early.
 *
 * ============================================================================
 * DEPENDENCIES
 * ============================================================================
 *
 * Canonical lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Canonical capability grammar:
 *
 *     grammar/core/capabilities.g4
 *
 * Canonical types:
 *
 *     grammar/types/
 *
 * Canonical expressions:
 *
 *     grammar/expressions/
 *
 * Canonical attributes:
 *
 *     grammar/core/attributes.g4
 *
 * AI composition:
 *
 *     grammar/ai/ai.g4
 *
 * AI models:
 *
 *     grammar/ai/models.g4
 *
 * AI tensors:
 *
 *     grammar/ai/tensors.g4
 *
 * AI training:
 *
 *     grammar/ai/training.g4
 *
 * AI inference:
 *
 *     grammar/ai/inference.g4
 *
 * AI accelerators:
 *
 *     grammar/ai/ai-accelerators.g4
 *
 * Resources:
 *
 *     grammar/resources/
 *
 * Hardware:
 *
 *     grammar/hardware/
 *
 * Quantum:
 *
 *     grammar/quantum/
 *
 * Distributed:
 *
 *     grammar/distributed/
 *
 * Security:
 *
 *     grammar/security/
 *
 * ============================================================================
 * IMPORT POLICY
 * ============================================================================
 *
 * This grammar imports the canonical capability grammar so that capability
 * identity remains single-sourced.
 *
 * It also imports canonical expressions for optional capability predicates
 * and metadata values.
 *
 * No AI leaf grammar is imported merely to inspect another AI construct.
 *
 * This prevents circular grammar dependencies.
 *
 * ============================================================================
 */

parser grammar AICapabilities;

options {
    tokenVocab = ZamaniLexer;
}

import Capabilities,
       Expressions,
       Types;


/* ============================================================================
 * 1. PUBLIC AI CAPABILITY CONSTRUCT
 * ============================================================================
 *
 * This is the stable AI-domain entry point.
 *
 * It intentionally contains only explicitly capability-oriented constructs.
 *
 * Ordinary expressions do NOT enter this rule.
 * ============================================================================
 */

aiCapabilityConstruct
    : aiCapabilityRequirement
    | aiCapabilityPreference
    | aiCapabilityConstraint
    | aiCapabilityHint
    | aiCapabilityContract
    | aiCapabilityAttachment
    | aiCapabilityReference
    ;


/* ============================================================================
 * 2. AI CAPABILITY REQUIREMENT
 * ============================================================================
 *
 * Requirement means the capability is necessary for semantic feasibility.
 *
 * Example:
 *
 *     @requires ai::inference;
 *
 *     @requires accelerator::tensor_compute;
 *
 * The parser does not determine whether the capability exists.
 * ============================================================================
 */

aiCapabilityRequirement
    : NANO_ANNOTATION
      aiCapabilityRoleRequirement
      aiCapabilityRequirementPayload
      SEMI?
    ;


aiCapabilityRoleRequirement
    : identifier
    ;


aiCapabilityRequirementPayload
    : capabilityReference
    | aiCapabilityPredicate
    | aiCapabilityReferenceListPayload
    ;


/* ============================================================================
 * 3. AI CAPABILITY PREFERENCE
 * ============================================================================
 *
 * A preference is weaker than a requirement.
 *
 * It MUST NOT make an otherwise valid realization invalid merely because the
 * preference cannot be satisfied.
 * ============================================================================
 */

aiCapabilityPreference
    : NANO_ANNOTATION
      aiCapabilityRolePreference
      aiCapabilityPreferencePayload
      SEMI?
    ;


aiCapabilityRolePreference
    : identifier
    ;


aiCapabilityPreferencePayload
    : capabilityReference
    | aiCapabilityPredicate
    | aiCapabilityReferenceListPayload
    ;


/* ============================================================================
 * 4. AI CAPABILITY CONSTRAINT
 * ============================================================================
 *
 * A constraint expresses a condition that a valid realization must satisfy.
 *
 * Whether the condition is satisfiable is semantic analysis.
 * ============================================================================
 */

aiCapabilityConstraint
    : NANO_ANNOTATION
      aiCapabilityRoleConstraint
      aiCapabilityConstraintPayload
      SEMI?
    ;


aiCapabilityRoleConstraint
    : identifier
    ;


aiCapabilityConstraintPayload
    : capabilityReference
    | aiCapabilityPredicate
    | aiCapabilityReferenceListPayload
    ;


/* ============================================================================
 * 5. AI CAPABILITY HINT
 * ============================================================================
 *
 * A hint is intentionally weaker than a requirement or constraint.
 *
 * Hints may guide optimization, lowering, scheduling, or deployment without
 * changing program correctness.
 * ============================================================================
 */

aiCapabilityHint
    : NANO_ANNOTATION
      aiCapabilityRoleHint
      aiCapabilityHintPayload
      SEMI?
    ;


aiCapabilityRoleHint
    : identifier
    ;


aiCapabilityHintPayload
    : capabilityReference
    | aiCapabilityPredicate
    | aiCapabilityReferenceListPayload
    ;


/* ============================================================================
 * 6. AI CAPABILITY CONTRACT
 * ============================================================================
 *
 * A contract groups multiple capability roles around one AI computation.
 *
 * Example:
 *
 *     @capabilities {
 *         @requires ai::inference;
 *         @requires ai::tensor::compute;
 *         @prefer accelerator::tensor_compute;
 *         @hint ai::batching;
 *     }
 *
 * The annotation name remains open.
 *
 * Semantic analysis determines whether the annotation denotes a registered
 * capability-contract role.
 * ============================================================================
 */

aiCapabilityContract
    : NANO_ANNOTATION
      aiCapabilityContractBody
    ;


aiCapabilityContractBody
    : LBRACE
      aiCapabilityContractMember*
      RBRACE
    ;


aiCapabilityContractMember
    : aiCapabilityRequirement
    | aiCapabilityPreference
    | aiCapabilityConstraint
    | aiCapabilityHint
    | aiCapabilityAttachment
    ;


/* ============================================================================
 * 7. AI CAPABILITY ATTACHMENT
 * ============================================================================
 *
 * An attachment associates capability metadata with an AI construct without
 * requiring a new lexer keyword.
 *
 * Example:
 *
 *     @capability ai::inference;
 *
 * The annotation role remains semantic.
 * ============================================================================
 */

aiCapabilityAttachment
    : NANO_ANNOTATION
      aiCapabilityAttachmentPayload
      SEMI?
    ;


aiCapabilityAttachmentPayload
    : capabilityReference
    | aiCapabilityPredicate
    | aiCapabilityReferenceListPayload
    ;


/* ============================================================================
 * 8. AI CAPABILITY REFERENCE
 * ============================================================================
 *
 * Stable adapter over the canonical capability reference.
 *
 * This rule MUST NOT redefine capability identity.
 * ============================================================================
 */

aiCapabilityReference
    : capabilityReference
    ;


/* ============================================================================
 * 9. AI CAPABILITY REFERENCE LIST
 * ============================================================================
 *
 * No finite capability count is imposed.
 * ============================================================================
 */

aiCapabilityReferenceListPayload
    : LBRACKET
      capabilityReference
      (COMMA capabilityReference)*
      RBRACKET
    ;


/* ============================================================================
 * 10. AI CAPABILITY PREDICATE
 * ============================================================================
 *
 * A predicate permits a capability contract to carry an expression-valued
 * condition.
 *
 * Example semantic forms:
 *
 *     capability_expression
 *     capability_expression && other_expression
 *     capability_expression || fallback_expression
 *
 * The exact semantic interpretation belongs to capability analysis.
 *
 * General expression syntax remains owned by Expressions.
 * ============================================================================
 */

aiCapabilityPredicate
    : expression
    ;


/* ============================================================================
 * 11. AI CAPABILITY VERSION ADAPTER
 * ============================================================================
 *
 * Version syntax remains owned by the canonical capability grammar.
 *
 * This adapter exists for stable AI-domain tooling names.
 * ============================================================================
 */

aiCapabilityVersion
    : capabilityVersionClause
    ;


/* ============================================================================
 * 12. AI CAPABILITY NAME ADAPTER
 * ============================================================================
 *
 * Identity remains owned by Capabilities.
 * ============================================================================
 */

aiCapabilityName
    : capabilityName
    ;


/* ============================================================================
 * 13. AI CAPABILITY ALIAS ADAPTER
 * ============================================================================
 *
 * Alias semantics remain downstream.
 * ============================================================================
 */

aiCapabilityAlias
    : capabilityAlias
    ;


/* ============================================================================
 * 14. AI CAPABILITY DECLARATION ADAPTER
 * ============================================================================
 *
 * AI code may explicitly declare a capability using the canonical capability
 * declaration syntax.
 *
 * This is an adapter, not a second declaration system.
 * ============================================================================
 */

aiCapabilityDeclaration
    : capabilityDeclaration
    ;


/* ============================================================================
 * 15. MODEL CAPABILITY CONTRACT
 * ============================================================================
 *
 * This stable adapter gives model-related grammar/tooling a named boundary
 * without duplicating model syntax.
 *
 * Example semantic usage:
 *
 *     model capability requirements
 *
 * The actual model declaration remains owned by:
 *
 *     grammar/ai/models.g4
 * ============================================================================
 */

aiModelCapabilityContract
    : aiCapabilityContract
    ;


/* ============================================================================
 * 16. TENSOR CAPABILITY CONTRACT
 * ============================================================================
 *
 * Tensor syntax remains owned by:
 *
 *     grammar/ai/tensors.g4
 *
 * This rule only exposes a stable capability boundary.
 * ============================================================================
 */

aiTensorCapabilityContract
    : aiCapabilityContract
    ;


/* ============================================================================
 * 17. TRAINING CAPABILITY CONTRACT
 * ============================================================================
 */

aiTrainingCapabilityContract
    : aiCapabilityContract
    ;


/* ============================================================================
 * 18. INFERENCE CAPABILITY CONTRACT
 * ============================================================================
 */

aiInferenceCapabilityContract
    : aiCapabilityContract
    ;


/* ============================================================================
 * 19. DIFFERENTIATION CAPABILITY CONTRACT
 * ============================================================================
 */

aiDifferentiationCapabilityContract
    : aiCapabilityContract
    ;


/* ============================================================================
 * 20. PIPELINE CAPABILITY CONTRACT
 * ============================================================================
 */

aiPipelineCapabilityContract
    : aiCapabilityContract
    ;


/* ============================================================================
 * 21. AGENT CAPABILITY CONTRACT
 * ============================================================================
 */

aiAgentCapabilityContract
    : aiCapabilityContract
    ;


/* ============================================================================
 * 22. ACCELERATOR CAPABILITY CONTRACT
 * ============================================================================
 *
 * Accelerator identity remains open-world.
 *
 * This rule does not enumerate GPUs, TPUs, NPUs, FPGAs, ASICs, QPUs, etc.
 * ============================================================================
 */

aiAcceleratorCapabilityContract
    : aiCapabilityContract
    ;


/* ============================================================================
 * 23. DISTRIBUTED AI CAPABILITY CONTRACT
 * ============================================================================
 */

aiDistributedCapabilityContract
    : aiCapabilityContract
    ;


/* ============================================================================
 * 24. QUANTUM-HYBRID AI CAPABILITY CONTRACT
 * ============================================================================
 *
 * Quantum semantics remain outside this grammar.
 *
 * This is only a capability attachment boundary.
 *
 * Actual quantum lowering remains:
 *
 *     semantic analysis
 *          |
 *          v
 *     quantum semantic model
 *          |
 *          v
 *     quantum::ir
 * ============================================================================
 */

aiQuantumHybridCapabilityContract
    : aiCapabilityContract
    ;


/* ============================================================================
 * 25. HARDWARE-BACKED AI CAPABILITY CONTRACT
 * ============================================================================
 */

aiHardwareCapabilityContract
    : aiCapabilityContract
    ;


/* ============================================================================
 * 26. DEPLOYMENT CAPABILITY CONTRACT
 * ============================================================================
 *
 * Deployment syntax remains owned by:
 *
 *     grammar/ai/model-deployment.g4
 *
 * This rule only exposes the capability contract.
 * ============================================================================
 */

aiDeploymentCapabilityContract
    : aiCapabilityContract
    ;


/* ============================================================================
 * 27. SECURITY CAPABILITY CONTRACT
 * ============================================================================
 *
 * Security enforcement remains outside grammar.
 * ============================================================================
 */

aiSecurityCapabilityContract
    : aiCapabilityContract
    ;


/* ============================================================================
 * 28. PORTABILITY CAPABILITY CONTRACT
 * ============================================================================
 *
 * Capability requirements may participate in POCO-REAF portability analysis.
 *
 * This grammar does not decide portability.
 * ============================================================================
 */

aiPortabilityCapabilityContract
    : aiCapabilityContract
    ;


/* ============================================================================
 * 29. CAPABILITY CONTRACT SEQUENCE
 * ============================================================================
 *
 * Used by explicit composition layers.
 *
 * No finite maximum is encoded.
 * ============================================================================
 */

aiCapabilityContractSequence
    : aiCapabilityContract+
    ;


/* ============================================================================
 * 30. CAPABILITY REFERENCE SEQUENCE
 * ============================================================================
 *
 * No finite maximum is encoded.
 * ============================================================================
 */

aiCapabilityReferenceSequence
    : aiCapabilityReference+
    ;


/* ============================================================================
 * 31. CAPABILITY ATTACHMENT SEQUENCE
 * ============================================================================
 */

aiCapabilityAttachmentSequence
    : aiCapabilityAttachment+
    ;


/* ============================================================================
 * 32. CAPABILITY MEMBER SEQUENCE
 * ============================================================================
 */

aiCapabilityMemberSequence
    : aiCapabilityContractMember+
    ;


/* ============================================================================
 * 33. AI CAPABILITY DECLARATION SEQUENCE
 * ============================================================================
 */

aiCapabilityDeclarationSequence
    : aiCapabilityDeclaration+
    ;


/* ============================================================================
 * 34. CAPABILITY VERSIONED REFERENCE
 * ============================================================================
 *
 * The canonical capability reference already permits version requirements.
 *
 * This rule exists as a stable AI-domain API.
 * ============================================================================
 */

aiVersionedCapabilityReference
    : capabilityReference
    ;


/* ============================================================================
 * 35. CAPABILITY REQUIREMENT LIST
 * ============================================================================
 */

aiCapabilityRequirementList
    : aiCapabilityRequirement+
    ;


/* ============================================================================
 * 36. CAPABILITY PREFERENCE LIST
 * ============================================================================
 */

aiCapabilityPreferenceList
    : aiCapabilityPreference+
    ;


/* ============================================================================
 * 37. CAPABILITY CONSTRAINT LIST
 * ============================================================================
 */

aiCapabilityConstraintList
    : aiCapabilityConstraint+
    ;


/* ============================================================================
 * 38. CAPABILITY HINT LIST
 * ============================================================================
 */

aiCapabilityHintList
    : aiCapabilityHint+
    ;


/* ============================================================================
 * 39. EXPLICIT AI CAPABILITY BLOCK
 * ============================================================================
 *
 * This is the preferred structured form when multiple capability contracts
 * belong to the same AI construct.
 *
 * Example:
 *
 *     @capabilities {
 *         @requires ai::inference;
 *         @requires ai::tensor::compute;
 *         @prefer accelerator::tensor_compute;
 *         @constraint ai::deterministic;
 *         @hint ai::batching;
 *     }
 *
 * No implementation or target selection is implied.
 * ============================================================================
 */

aiCapabilitiesBlock
    : NANO_ANNOTATION
      LBRACE
      aiCapabilityContractMember*
      RBRACE
    ;


/* ============================================================================
 * 40. AI CAPABILITY EXPRESSION BRIDGE
 * ============================================================================
 *
 * This bridge permits other AI grammars to consume the capability syntax
 * without importing or redefining the generic capability grammar themselves.
 * ============================================================================
 */

aiCapabilityExpression
    : capabilityReference
    | aiCapabilityPredicate
    ;


/* ============================================================================
 * 41. RESOURCE BRIDGE
 * ============================================================================
 *
 * A capability may be associated semantically with resources.
 *
 * This grammar intentionally does not define resource syntax.
 *
 * Resource requirements remain owned by grammar/resources/.
 * ============================================================================
 */

aiCapabilityResourcePredicate
    : expression
    ;


/* ============================================================================
 * 42. CROSS-DOMAIN CAPABILITY BRIDGE
 * ============================================================================
 *
 * The same AI capability contract can be consumed by:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     hardware
 *     distributed
 *     networking
 *     security
 *     execution
 *     compile
 *
 * without creating separate capability languages.
 * ============================================================================
 */

aiCrossDomainCapabilityContract
    : aiCapabilityContract
    ;


/* ============================================================================
 * 43. FUTURE-DOMAIN CAPABILITY BRIDGE
 * ============================================================================
 *
 * Future computational domains remain possible without modifying the grammar.
 * ============================================================================
 */

aiFutureCapabilityContract
    : aiCapabilityContract
    ;


/* ============================================================================
 * 44. SEMANTIC OWNERSHIP BOUNDARY
 * ============================================================================
 *
 * The parser recognizes structure.
 *
 * Semantic analysis owns:
 *
 *     - capability registration;
 *     - namespace resolution;
 *     - version compatibility;
 *     - capability inheritance;
 *     - capability implication;
 *     - capability conflicts;
 *     - capability satisfiability;
 *     - target applicability;
 *     - resource implications;
 *     - security/trust;
 *     - portability;
 *     - dialect ownership;
 *     - deprecation;
 *     - availability.
 *
 * The grammar MUST NOT attempt any of these decisions.
 * ============================================================================
 */


/* ============================================================================
 * 45. CAPABILITY / RESOURCE BOUNDARY
 * ============================================================================
 *
 * A capability may semantically imply resources.
 *
 * For example:
 *
 *     ai::tensor::compute
 *
 * may imply some resource requirements on a particular target.
 *
 * But:
 *
 *     capability -> resource
 *
 * is a semantic transformation.
 *
 * It MUST NOT become:
 *
 *     capability -> hard-coded hardware size
 *
 * inside this grammar.
 * ============================================================================
 */


/* ============================================================================
 * 46. CAPABILITY / HARDWARE BOUNDARY
 * ============================================================================
 *
 * Hardware discovery is downstream.
 *
 * This grammar does not know:
 *
 *     how many CPUs exist;
 *     how many GPUs exist;
 *     how much memory exists;
 *     which accelerators exist;
 *     which QPU exists;
 *     what topology exists;
 *     what calibration state exists.
 *
 * A target can therefore satisfy or reject an AI capability requirement
 * without requiring a grammar change.
 * ============================================================================
 */


/* ============================================================================
 * 47. CAPABILITY / QUANTUM BOUNDARY
 * ============================================================================
 *
 * AI may require capabilities associated with quantum-assisted computation.
 *
 * Examples of semantic capability identities may include:
 *
 *     quantum::measurement
 *     quantum::dynamic_control
 *     quantum::mid_circuit_measurement
 *     quantum::logical_qubits
 *     quantum::error_correction
 *
 * These are identifiers, not grammar alternatives.
 *
 * This grammar does not define:
 *
 *     qubits;
 *     physical qubits;
 *     gates;
 *     topology;
 *     routing;
 *     scheduling;
 *     QEC;
 *     ZQN.
 *
 * If quantum computation is involved, the canonical semantic path remains:
 *
 *     source
 *       |
 *       v
 *     AST
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
 *     routing / scheduling / resilience / QEC / ZQN
 * ============================================================================
 */


/* ============================================================================
 * 48. CAPABILITY / HDL BOUNDARY
 * ============================================================================
 *
 * AI may require hardware-backed capabilities.
 *
 * This grammar does not describe:
 *
 *     register width;
 *     bus width;
 *     FPGA resource count;
 *     memory-bank count;
 *     physical wiring;
 *     clock topology;
 *     synthesis technology.
 *
 * Those remain owned by HDL/hardware/compiler layers.
 * ============================================================================
 */


/* ============================================================================
 * 49. CAPABILITY / DISTRIBUTED BOUNDARY
 * ============================================================================
 *
 * AI capabilities may require distributed execution properties.
 *
 * The grammar does not choose:
 *
 *     node count;
 *     machine count;
 *     network topology;
 *     placement;
 *     service discovery;
 *     scheduler;
 *     load balancer.
 *
 * These remain downstream.
 * ============================================================================
 */


/* ============================================================================
 * 50. CAPABILITY / SECURITY BOUNDARY
 * ============================================================================
 *
 * Declaring a capability does not grant authority.
 *
 * For example:
 *
 *     @requires security::secret_access;
 *
 * does not grant access to secrets.
 *
 * Authorization and trust remain owned by security/runtime systems.
 * ============================================================================
 */


/* ============================================================================
 * 51. AST CONTRACT
 * ============================================================================
 *
 * The parser/AST layer must preserve, at minimum:
 *
 *     - complete source span;
 *     - annotation span;
 *     - capability identity;
 *     - capability namespace;
 *     - optional version requirement;
 *     - capability role;
 *     - ordered contract members;
 *     - predicate expressions;
 *     - capability reference lists;
 *     - source provenance.
 *
 * The AST MUST NOT contain:
 *
 *     - physical device handles;
 *     - physical device IDs;
 *     - runtime capability tokens;
 *     - scheduler state;
 *     - routing state;
 *     - calibration state;
 *     - allocation state.
 *
 * The frontend AST remains domain-neutral.
 * ============================================================================
 */


/* ============================================================================
 * 52. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis must determine:
 *
 *     - what each capability identity means;
 *     - whether the capability exists;
 *     - whether the capability is declared by a dialect;
 *     - whether the version requirement is valid;
 *     - whether requirements conflict;
 *     - whether constraints conflict;
 *     - whether preferences are satisfiable;
 *     - whether hints are applicable;
 *     - whether capabilities imply resources;
 *     - whether the execution context satisfies requirements;
 *     - whether security policy permits the requested capability;
 *     - whether the program remains portable.
 *
 * None of these decisions belong to parsing.
 * ============================================================================
 */


/* ============================================================================
 * 53. REQUIREMENT SEMANTICS
 * ============================================================================
 *
 * Requirements are hard semantic conditions.
 *
 * If a required capability cannot be provided, downstream compilation or
 * execution planning must fail with a structured capability/resource
 * diagnostic.
 *
 * It MUST NOT become a syntax error merely because a particular target lacks
 * the capability.
 * ============================================================================
 */


/* ============================================================================
 * 54. PREFERENCE SEMANTICS
 * ============================================================================
 *
 * Preferences are weaker than requirements.
 *
 * Failure to satisfy a preference does not necessarily make the program
 * invalid.
 *
 * The compiler/runtime may use preferences during:
 *
 *     target selection;
 *     optimization;
 *     scheduling;
 *     placement;
 *     deployment.
 *
 * ============================================================================
 */


/* ============================================================================
 * 55. CONSTRAINT SEMANTICS
 * ============================================================================
 *
 * Constraints establish admissibility conditions.
 *
 * Semantic analysis determines whether the condition is:
 *
 *     satisfiable;
 *     contradictory;
 *     target-dependent;
 *     portable;
 *     unsupported.
 * ============================================================================
 */


/* ============================================================================
 * 56. HINT SEMANTICS
 * ============================================================================
 *
 * Hints are advisory.
 *
 * They MUST NOT silently change program correctness.
 *
 * A backend may ignore a hint if it cannot use it.
 * ============================================================================
 */


/* ============================================================================
 * 57. IR CONTRACT
 * ============================================================================
 *
 * This grammar produces NO IR.
 *
 * It MUST NOT introduce:
 *
 *     AICapabilityIR
 *     AIHardwareCapabilityIR
 *     AIQuantumCapabilityIR
 *
 * merely because AI capability syntax exists.
 *
 * Capability intent is lowered through the existing semantic/resource/
 * capability architecture.
 *
 * If an internal compiler representation is required, it belongs outside
 * grammar/.
 *
 * ============================================================================
 */


/* ============================================================================
 * 58. CANONICAL QUANTUM IR CONTRACT
 * ============================================================================
 *
 * AI capability syntax MUST NOT create a quantum IR.
 *
 * Where AI capability requirements affect quantum computation:
 *
 *     AI capability
 *          |
 *          v
 *     semantic capability analysis
 *          |
 *          v
 *     quantum semantic analysis
 *          |
 *          v
 *     quantum::ir
 *
 * The canonical quantum semantic boundary remains:
 *
 *     quantum::ir
 *
 * ============================================================================
 */


/* ============================================================================
 * 59. COMPILER CONTRACT
 * ============================================================================
 *
 * The compiler may use resolved capability information for:
 *
 *     target selection;
 *     specialization;
 *     optimization;
 *     lowering;
 *     scheduling;
 *     routing;
 *     deployment planning.
 *
 * It MUST NOT infer missing source semantics from target availability.
 *
 * If a source requirement is absent, a backend must not silently invent one.
 * ============================================================================
 */


/* ============================================================================
 * 60. RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime capability state is external execution data.
 *
 * The parser MUST NOT inspect:
 *
 *     hardware;
 *     environment;
 *     network;
 *     device state;
 *     accelerator state;
 *     memory state;
 *     calibration;
 *     load;
 *     availability.
 *
 * Runtime state is compared with the resolved semantic capability contract
 * downstream.
 * ============================================================================
 */


/* ============================================================================
 * 61. DETERMINISM
 * ============================================================================
 *
 * Parsing MUST be independent of:
 *
 *     - hardware;
 *     - network;
 *     - filesystem;
 *     - clock;
 *     - randomness;
 *     - environment variables;
 *     - runtime state;
 *     - target discovery.
 *
 * Given identical:
 *
 *     source;
 *     lexer version;
 *     grammar version;
 *     parser configuration;
 *
 * the parser must produce structurally equivalent output.
 * ============================================================================
 */


/* ============================================================================
 * 62. SECURITY
 * ============================================================================
 *
 * Capability syntax is untrusted input.
 *
 * This grammar:
 *
 *     - executes no code;
 *     - accesses no files;
 *     - accesses no network;
 *     - accesses no credentials;
 *     - probes no devices;
 *     - allocates no resources;
 *     - invokes no runtime capability.
 *
 * Capability authorization is never implied by parsing.
 * ============================================================================
 */


/* ============================================================================
 * 63. COMPATIBILITY
 * ============================================================================
 *
 * This grammar must remain compatible with:
 *
 *     grammar/core/capabilities.g4
 *
 * Capability identity changes are therefore governed by the canonical
 * capability/version compatibility system.
 *
 * AI-specific role changes must be classified as:
 *
 *     additive;
 *     compatible;
 *     deprecated;
 *     experimental;
 *     breaking
 *
 * according to the repository compatibility policy.
 *
 * The file must not silently redefine an existing capability.
 * ============================================================================
 */


/* ============================================================================
 * 64. DIALECT COMPATIBILITY
 * ============================================================================
 *
 * Dialects may introduce new capability identities.
 *
 * They must not silently change the meaning of a stable canonical capability.
 *
 * Dialect capability additions remain ordinary qualified capability names.
 *
 * Semantic registration occurs outside this grammar.
 * ============================================================================
 */


/* ============================================================================
 * 65. SOURCE COMPATIBILITY
 * ============================================================================
 *
 * Existing source using the canonical capability reference form remains valid
 * as long as the canonical capability grammar remains compatible.
 *
 * AI-specific adapters are additive and must not require existing source to
 * name physical devices.
 * ============================================================================
 */


/* ============================================================================
 * 66. TEST CONTRACT
 * ============================================================================
 *
 * Tests belong under the AI capability test area, for example:
 *
 *     grammar/tests/ai/capabilities/
 *
 * The tests must be organized into:
 *
 *     positive/
 *     negative/
 *     boundary/
 *     scalability/
 *     compatibility/
 *     determinism/
 *     portability/
 *
 * ============================================================================
 */


/* ============================================================================
 * 67. POSITIVE TEST CONTRACT
 * ============================================================================
 *
 * At minimum, acceptance coverage must include:
 *
 *     @requires ai::inference;
 *
 *     @requires ai::tensor::compute;
 *
 *     @requires accelerator::tensor_compute;
 *
 *     @prefer accelerator::low_latency;
 *
 *     @constraint ai::deterministic;
 *
 *     @hint ai::batching;
 *
 *     @capability ai::inference;
 *
 *     @capabilities {
 *         @requires ai::inference;
 *         @requires ai::tensor::compute;
 *         @prefer accelerator::tensor_compute;
 *         @hint ai::batching;
 *     }
 *
 *     @requires ai::inference version >= 1.0;
 *
 * where the underlying canonical capability syntax permits the version form.
 *
 * The test suite must also cover arbitrary qualified capability names.
 * ============================================================================
 */


/* ============================================================================
 * 68. CROSS-DOMAIN POSITIVE TESTS
 * ============================================================================
 *
 * Include capabilities associated semantically with:
 *
 *     classical computation;
 *     quantum computation;
 *     hybrid computation;
 *     HDL;
 *     hardware;
 *     distributed execution;
 *     networking;
 *     security;
 *     data;
 *     accelerators;
 *     future domains.
 *
 * Example:
 *
 *     @requires quantum::mid_circuit_measurement;
 *
 *     @requires hardware::tensor_acceleration;
 *
 *     @requires distributed::collective_compute;
 *
 *     @requires security::confidential_compute;
 *
 * These names remain identifiers.
 * ============================================================================
 */


/* ============================================================================
 * 69. NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * The parser must reject malformed capability contracts such as:
 *
 *     @requires;
 *
 *     @prefer;
 *
 *     @constraint;
 *
 *     @hint;
 *
 *     @capability;
 *
 *     @requires ai::;
 *
 *     @requires [;
 *
 *     @capabilities {
 *
 *     @capabilities }
 *
 * where the canonical lexer/parser structure makes them syntactically invalid.
 *
 * Semantic invalidity remains outside the parser.
 * ============================================================================
 */


/* ============================================================================
 * 70. SEMANTIC NEGATIVE TESTS
 * ============================================================================
 *
 * These must NOT be encoded as parser failures merely because they are
 * semantically invalid:
 *
 *     unknown capability;
 *     unavailable capability;
 *     unsatisfied capability version;
 *     conflicting capability requirements;
 *     insufficient hardware;
 *     unavailable accelerator;
 *     unavailable quantum capability;
 *     insufficient memory;
 *     incompatible topology.
 *
 * Those belong to semantic/resource/target validation.
 * ============================================================================
 */


/* ============================================================================
 * 71. BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Test:
 *
 *     one capability;
 *     many capabilities;
 *     deeply qualified capability names;
 *     many contract members;
 *     nested capability predicates;
 *     large source expressions;
 *     large version expressions;
 *     large capability collections.
 *
 * The grammar must not introduce a fixed boundary.
 * ============================================================================
 */


/* ============================================================================
 * 72. SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Generated tests must vary:
 *
 *     capability count;
 *     capability-reference count;
 *     contract-member count;
 *     qualified-name depth;
 *     expression size;
 *     number of AI constructs.
 *
 * Tests must verify that no language-defined maximum exists.
 *
 * Practical parser/compiler resource exhaustion may still occur, but that is
 * an implementation resource condition, not a language semantic limit.
 * ============================================================================
 */


/* ============================================================================
 * 73. PORTABILITY TEST CONTRACT
 * ============================================================================
 *
 * The same capability source must remain structurally valid when the target
 * environment changes from:
 *
 *     embedded
 *     CPU
 *     multicore
 *     GPU
 *     FPGA
 *     ASIC
 *     NPU
 *     accelerator
 *     QPU-assisted
 *     distributed
 *     HPC
 *     cloud
 *     future target
 *
 * Target feasibility is tested downstream.
 * ============================================================================
 */


/* ============================================================================
 * 74. DETERMINISM TEST CONTRACT
 * ============================================================================
 *
 * Parse the same source repeatedly with:
 *
 *     identical lexer;
 *     identical grammar;
 *     identical parser configuration;
 *
 * and verify equivalent parse structure.
 *
 * Hardware availability must not alter parser output.
 * ============================================================================
 */


/* ============================================================================
 * 75. HARD-CODING AUDIT
 * ============================================================================
 *
 * This grammar MUST pass an automated audit against universal limits and
 * physical identities.
 *
 * Forbidden examples include:
 *
 *     MAX_AI_CAPABILITIES
 *     MAX_AI_DEVICES
 *     MAX_AI_WORKERS
 *     MAX_AI_NODES
 *     MAX_AI_TENSORS
 *     MAX_AI_PARAMETERS
 *     MAX_AI_LAYERS
 *     MAX_AI_MEMORY
 *     MAX_AI_ACCELERATORS
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
 * Also forbidden as universal language assumptions:
 *
 *     GPU_0
 *     CPU_0
 *     QPU_0
 *     DEVICE_0
 *     NODE_0
 *
 * when those represent fixed physical resource identities.
 *
 * Explicit program data is not prohibited.
 *
 * For example:
 *
 *     let replicas = 8;
 *
 * is program data.
 *
 * The grammar may parse that value through ordinary expression syntax.
 *
 * What is prohibited is turning 8 into a universal compiler limit.
 * ============================================================================
 */


/* ============================================================================
 * 76. NO VENDOR LOCK-IN
 * ============================================================================
 *
 * This grammar must not enumerate:
 *
 *     CUDA
 *     ROCm
 *     TensorRT
 *     PyTorch
 *     TensorFlow
 *     JAX
 *     vendor-specific GPU models
 *     vendor-specific accelerator models
 *
 * as mandatory language capabilities.
 *
 * Vendor integration belongs to interoperability, dialect, compiler, or
 * backend layers.
 *
 * ============================================================================
 */


/* ============================================================================
 * 77. NO FRAMEWORK LOCK-IN
 * ============================================================================
 *
 * AI frameworks may provide capability implementations.
 *
 * Framework names are not required to be source-language capability names.
 *
 * A framework may satisfy:
 *
 *     ai::tensor::compute
 *
 * without changing the Zamani source language.
 * ============================================================================
 */


/* ============================================================================
 * 78. NO PHYSICAL DEVICE SELECTION
 * ============================================================================
 *
 * This grammar must not provide a source-level requirement that means:
 *
 *     use GPU 0;
 *     use CPU 3;
 *     use QPU 7;
 *     use FPGA 2;
 *
 * Physical realization is downstream.
 *
 * If a future target-specific dialect genuinely requires physical selection,
 * that must be explicit, versioned, isolated, and outside the portable
 * capability contract.
 * ============================================================================
 */


/* ============================================================================
 * 79. POCO-REAF INVARIANT
 * ============================================================================
 *
 * The same source capability contract must remain meaningful when the
 * execution scale changes.
 *
 * Example:
 *
 *     @requires ai::tensor::compute;
 *
 * may be realized by:
 *
 *     one CPU;
 *     many CPU cores;
 *     one GPU;
 *     many GPUs;
 *     an FPGA;
 *     an ASIC;
 *     an accelerator;
 *     a distributed system;
 *     a future architecture.
 *
 * The source contract does not change merely because the realization changes.
 * ============================================================================
 */


/* ============================================================================
 * 80. RESOURCE AVAILABILITY
 * ============================================================================
 *
 * "Infinity" in POCO-REAF means:
 *
 *     no artificial language ceiling.
 *
 * It does NOT mean:
 *
 *     infinite physical hardware;
 *     infinite memory;
 *     infinite execution time;
 *     guaranteed resource availability.
 *
 * A program may scale as far as actual resources and semantic constraints
 * permit.
 *
 * A resource shortage is a realization/feasibility condition, not a grammar
 * limit.
 * ============================================================================
 */


/* ============================================================================
 * 81. FRONTEND INTEGRATION
 * ============================================================================
 *
 * The frontend pipeline must preserve:
 *
 *     source span
 *          |
 *          v
 *     parse structure
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     capability semantic model
 *
 * No capability information may be silently discarded between parsing and
 * semantic analysis.
 * ============================================================================
 */


/* ============================================================================
 * 82. AI COMPOSITION INTEGRATION
 * ============================================================================
 *
 * grammar/ai/ai.g4 currently provides generic AI capability contracts.
 *
 * This file provides the stronger structured capability leaf grammar.
 *
 * The canonical composition layer should expose:
 *
 *     aiCapabilityConstruct
 *
 * without duplicating its productions.
 *
 * AI.g4 should consume this grammar at its AI capability boundary rather than
 * maintaining a second capability grammar.
 *
 * ============================================================================
 */


/* ============================================================================
 * 83. MODEL INTEGRATION
 * ============================================================================
 *
 * grammar/ai/models.g4 remains the model syntax authority.
 *
 * Model capability requirements attach to models through:
 *
 *     aiModelCapabilityContract
 *
 * The model grammar does not need to define capability identity.
 * ============================================================================
 */


/* ============================================================================
 * 84. TENSOR INTEGRATION
 * ============================================================================
 *
 * grammar/ai/tensors.g4 remains the tensor syntax authority.
 *
 * Tensor capability requirements use:
 *
 *     aiTensorCapabilityContract
 *
 * Tensor shape/rank/dimension semantics remain in the tensor/type systems.
 *
 * No tensor hardware limit belongs here.
 * ============================================================================
 */


/* ============================================================================
 * 85. TRAINING INTEGRATION
 * ============================================================================
 *
 * grammar/ai/training.g4 remains the training syntax authority.
 *
 * Training capability contracts may express requirements such as:
 *
 *     ai::training
 *     ai::distributed_training
 *     ai::automatic_differentiation
 *
 * without encoding a particular number of workers or devices.
 * ============================================================================
 */


/* ============================================================================
 * 86. INFERENCE INTEGRATION
 * ============================================================================
 *
 * grammar/ai/inference.g4 remains the inference syntax authority.
 *
 * Inference capability contracts may describe:
 *
 *     inference;
 *     batching;
 *     streaming;
 *     low-latency;
 *     deterministic execution;
 *     model serving;
 *
 * through open capability identities.
 * ============================================================================
 */


/* ============================================================================
 * 87. ACCELERATOR INTEGRATION
 * ============================================================================
 *
 * grammar/ai/ai-accelerators.g4 remains the accelerator-intent authority.
 *
 * This grammar supplies capability contracts but does not duplicate
 * accelerator syntax.
 *
 * Capability:
 *
 *     accelerator::tensor_compute
 *
 * does not imply:
 *
 *     GPU;
 *     TPU;
 *     NPU;
 *     FPGA;
 *     ASIC.
 *
 * Semantic target matching decides realization.
 * ============================================================================
 */


/* ============================================================================
 * 88. DEPLOYMENT INTEGRATION
 * ============================================================================
 *
 * grammar/ai/model-deployment.g4 may consume:
 *
 *     aiDeploymentCapabilityContract
 *
 * for deployment requirements.
 *
 * Deployment remains responsible for deployment syntax.
 *
 * This file remains responsible only for capability intent.
 * ============================================================================
 */


/* ============================================================================
 * 89. RESOURCE INTEGRATION
 * ============================================================================
 *
 * Capability and resource semantics must remain distinct.
 *
 * Example:
 *
 *     @requires ai::tensor::compute;
 *
 * may cause semantic analysis to derive a target-specific resource demand.
 *
 * That derivation occurs outside this grammar.
 *
 * The grammar does not say:
 *
 *     tensor_compute == N GPUs
 *
 * or:
 *
 *     tensor_compute == N GB memory.
 * ============================================================================
 */


/* ============================================================================
 * 90. HARDWARE INTEGRATION
 * ============================================================================
 *
 * Hardware capabilities are supplied by hardware descriptions and target
 * discovery.
 *
 * This grammar merely preserves the requested capability identity.
 *
 * Hardware availability is compared downstream.
 * ============================================================================
 */


/* ============================================================================
 * 91. DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Distributed execution may satisfy an AI capability without requiring the
 * source program to specify a fixed node count.
 *
 * Capability matching and placement remain downstream.
 * ============================================================================
 */


/* ============================================================================
 * 92. SECURITY INTEGRATION
 * ============================================================================
 *
 * Capability declarations must be subject to security policy.
 *
 * A source-level capability reference is not authorization.
 *
 * Security analysis determines whether the capability may be used.
 * ============================================================================
 */


/* ============================================================================
 * 93. TOOLING INTEGRATION
 * ============================================================================
 *
 * Language tooling may use this grammar to provide:
 *
 *     capability completion;
 *     syntax highlighting;
 *     diagnostics;
 *     formatting;
 *     navigation;
 *     documentation extraction;
 *     capability-reference indexing.
 *
 * Tooling must resolve semantic capability names through the canonical
 * registry rather than maintaining another independent list.
 * ============================================================================
 */


/* ============================================================================
 * 94. GENERATED CODE / RUST INTEGRATION
 * ============================================================================
 *
 * This grammar contains no Rust code.
 *
 * The generated parser/frontend integration must:
 *
 *     - use Rust 2021;
 *     - compile on Rust 1.97;
 *     - compile on Rust 1.97.1;
 *     - use safe Rust;
 *     - contain no unsafe implementation;
 *     - preserve source spans;
 *     - preserve source ordering;
 *     - preserve capability references;
 *     - distinguish syntax errors from semantic capability failures.
 *
 * Generated ANTLR output is generated code and must not be hand-maintained.
 *
 * The Rust compiler frontend must not compensate for malformed parse trees by
 * guessing capability semantics.
 * ============================================================================
 */


/* ============================================================================
 * 95. ERROR CONTRACT
 * ============================================================================
 *
 * Syntax errors belong to this grammar.
 *
 * Semantic errors belong to semantic/resource/capability analysis.
 *
 * Examples of syntax errors:
 *
 *     @requires;
 *     @capabilities {
 *     @requires ai::;
 *
 * Examples of semantic errors:
 *
 *     unknown capability;
 *     invalid capability version;
 *     conflicting requirements;
 *     unavailable target capability;
 *     insufficient resources;
 *     unauthorized capability.
 *
 * The diagnostic layer must preserve this distinction.
 * ============================================================================
 */


/* ============================================================================
 * 96. DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Capability diagnostics should preserve:
 *
 *     source span;
 *     capability identity;
 *     role;
 *     version requirement;
 *     semantic context;
 *     originating AI construct;
 *     relevant resource/capability reason.
 *
 * A hardware shortage should identify resource/capability feasibility rather
 * than reporting "invalid syntax".
 * ============================================================================
 */


/* ============================================================================
 * 97. PERFORMANCE CONTRACT
 * ============================================================================
 *
 * Grammar design must avoid unnecessary ambiguity and pathological recursive
 * structures.
 *
 * Capability lists use ordinary repetition.
 *
 * No semantic predicate or target lookup is permitted to resolve ambiguity.
 *
 * Parser performance may depend on implementation resource limits, but no
 * artificial language cardinality is encoded.
 * ============================================================================
 */


/* ============================================================================
 * 98. COMPLETION CRITERIA
 * ============================================================================
 *
 * THIS FILE IS COMPLETE ONLY WHEN:
 *
 * [x] It is a parser grammar.
 *
 * [x] It uses the canonical ZamaniLexer vocabulary.
 *
 * [x] It imports the canonical capability grammar.
 *
 * [x] It does not redefine capability identity.
 *
 * [x] It does not create a second capability registry.
 *
 * [x] It does not enumerate AI capabilities.
 *
 * [x] It supports open-world capability names.
 *
 * [x] It distinguishes requirement from preference.
 *
 * [x] It distinguishes constraint from hint.
 *
 * [x] It supports grouped capability contracts.
 *
 * [x] It supports arbitrary capability-reference lists.
 *
 * [x] It supports capability version requirements through the canonical
 *     capability grammar.
 *
 * [x] It preserves source-level capability intent.
 *
 * [x] It does not select physical devices.
 *
 * [x] It does not discover hardware.
 *
 * [x] It does not allocate resources.
 *
 * [x] It does not implement scheduling.
 *
 * [x] It does not implement routing.
 *
 * [x] It does not implement optimization.
 *
 * [x] It does not implement QEC.
 *
 * [x] It does not implement ZQN.
 *
 * [x] It does not create quantum::ir.
 *
 * [x] It does not create a second AI IR.
 *
 * [x] It contains no universal resource limits.
 *
 * [x] It contains no vendor lock-in.
 *
 * [x] It contains no framework lock-in.
 *
 * [x] It contains no Rust actions.
 *
 * [x] It contains no unsafe implementation.
 *
 * [x] It is deterministic with respect to parser inputs.
 *
 * [x] It preserves POCO-REAF.
 *
 * The following repository integrations must be completed before the
 * capability subsystem can be called repository-wide production-ready:
 *
 * [ ] AI composition imports/exposes `aiCapabilityConstruct`.
 *
 * [ ] AI grammar removes its weaker duplicate capability contract where the
 *     structured capability grammar supersedes it.
 *
 * [ ] `grammar/core/capabilities.g4` remains the sole capability identity
 *     authority.
 *
 * [ ] AI model grammar consumes `aiModelCapabilityContract` where appropriate.
 *
 * [ ] AI tensor grammar consumes `aiTensorCapabilityContract` where
 *     appropriate.
 *
 * [ ] AI training grammar consumes `aiTrainingCapabilityContract`.
 *
 * [ ] AI inference grammar consumes `aiInferenceCapabilityContract`.
 *
 * [ ] AI accelerator grammar consumes `aiAcceleratorCapabilityContract`.
 *
 * [ ] AI deployment grammar consumes `aiDeploymentCapabilityContract`.
 *
 * [ ] Resource grammar consumes canonical capability references rather than
 *     defining AI-specific capability identity.
 *
 * [ ] Hardware grammar consumes canonical capability references.
 *
 * [ ] Quantum grammar consumes canonical capability references.
 *
 * [ ] Distributed grammar consumes canonical capability references.
 *
 * [ ] Security analysis treats capability references as intent rather than
 *     authorization.
 *
 * [ ] Frontend AST preserves capability source spans and ordering.
 *
 * [ ] Semantic capability resolution is implemented.
 *
 * [ ] Capability version compatibility is implemented.
 *
 * [ ] Resource implication analysis is implemented.
 *
 * [ ] Target capability matching is implemented.
 *
 * [ ] Positive tests pass.
 *
 * [ ] Negative tests pass.
 *
 * [ ] Boundary tests pass.
 *
 * [ ] Scalability tests pass.
 *
 * [ ] Portability tests pass.
 *
 * [ ] Determinism tests pass.
 *
 * [ ] Compatibility tests pass.
 *
 * [ ] Rust 1.97 / 1.97.1 integration passes.
 *
 * [ ] Repository-wide no-unsafe policy passes.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * AI capability syntax describes:
 *
 *     WHAT capability the AI computation needs
 *
 * and optionally:
 *
 *     HOW strongly that capability participates in realization.
 *
 * It does NOT describe:
 *
 *     WHICH physical machine;
 *     WHICH CPU;
 *     WHICH GPU;
 *     WHICH FPGA;
 *     WHICH QPU;
 *     WHICH node;
 *     WHICH memory bank;
 *     WHICH network path;
 *     WHICH scheduler;
 *     WHICH routing strategy.
 *
 * Therefore:
 *
 *     AI source
 *          |
 *          v
 *     capability intent
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     resource/capability resolution
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          v
 *     optimization / routing / scheduling
 *          |
 *          v
 *     target realization
 *
 * remains compatible with:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * ============================================================================
 */
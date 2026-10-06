/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/ai/ai.g4
 *
 * GRAMMAR
 * -------
 * AI
 *
 * STATUS
 * ------
 * CANONICAL AI DOMAIN COMPOSITION / ORCHESTRATION GRAMMAR
 *
 * BASELINE
 * --------
 * Rust 1.97+
 * Rust 2021
 * Safe Rust only
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the SINGLE COMPOSITION ROOT for grammar/ai/.
 *
 * ai.g4 does not implement individual AI features.
 *
 * Instead, it:
 *
 *     1. imports the independently-owned AI grammars;
 *     2. exposes one stable AI parser boundary;
 *     3. dispatches source constructs to their owning grammar;
 *     4. prevents duplicate AI syntax authorities;
 *     5. provides the integration boundary between AI and the universal
 *        Zamani grammar;
 *     6. preserves the domain-neutral AST boundary;
 *     7. preserves the canonical semantic/IR architecture;
 *     8. permits future AI domains to be added without rewriting the
 *        universal language grammar;
 *     9. keeps AI independent of machine size, topology and vendor;
 *    10. supports POCO-REAF.
 *
 * The architectural direction is:
 *
 *     source
 *       |
 *       v
 *     lexer
 *       |
 *       v
 *     ZamaniParser
 *       |
 *       v
 *     AI
 *       |
 *       +-----------------------------+
 *       |                             |
 *       v                             v
 *     AI leaf grammars          universal grammar
 *       |                             |
 *       +-------------+---------------+
 *                     |
 *                     v
 *              domain-neutral AST
 *                     |
 *                     v
 *              structural validation
 *                     |
 *                     v
 *               semantic model
 *                     |
 *       +-------------+------------------+
 *       |             |                  |
 *       v             v                  v
 *     types        effects          resources
 *       |             |                  |
 *       +-------------+------------------+
 *                     |
 *                     v
 *                  policies
 *                     |
 *                     v
 *                provenance
 *                     |
 *                     v
 *               canonical IR
 *                     |
 *          +----------+----------+
 *          |                     |
 *          v                     v
 *     classical IR          quantum::ir
 *          |                     |
 *          +----------+----------+
 *                     |
 *                     v
 *              optimization
 *                     |
 *                     v
 *          lowering / routing / scheduling
 *                     |
 *                     v
 *              resilience / recovery
 *                     |
 *                     v
 *                    ZQN
 *                     |
 *                     v
 *                    HAL
 *                     |
 *                     v
 *               target realization
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - AI grammar composition;
 *     - AI grammar dispatch;
 *     - the canonical AI parser boundary;
 *     - the public AI entry rule;
 *     - integration of AI leaf grammars;
 *     - prevention of duplicate AI grammar ownership;
 *     - AI-to-universal grammar composition;
 *     - AI extension composition policy.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical tokens;
 *     - identifiers;
 *     - expressions;
 *     - types;
 *     - declarations;
 *     - statements;
 *     - model syntax;
 *     - dataset syntax;
 *     - tensor syntax;
 *     - training syntax;
 *     - inference syntax;
 *     - agent syntax;
 *     - reasoning syntax;
 *     - learning semantics;
 *     - adaptation semantics;
 *     - probability semantics;
 *     - uncertainty semantics;
 *     - knowledge semantics;
 *     - causality semantics;
 *     - explanation semantics;
 *     - provenance semantics;
 *     - policy semantics;
 *     - security enforcement;
 *     - resource discovery;
 *     - capability discovery;
 *     - target selection;
 *     - hardware discovery;
 *     - scheduling;
 *     - routing;
 *     - optimization;
 *     - runtime execution;
 *     - classical IR;
 *     - AI-specific IR;
 *     - quantum IR;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN;
 *     - HAL.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * Every AI feature MUST have exactly one grammar owner.
 *
 * ai.g4 is the composition root.
 *
 * A leaf grammar owns its syntax.
 *
 * ai.g4 merely imports and dispatches to that syntax.
 *
 * No feature may be copied into ai.g4 merely to make it visible.
 *
 * ============================================================================
 * LEXICAL AUTHORITY
 * ============================================================================
 *
 * Lexer ownership remains:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * AI.g4 MUST NOT define lexer rules.
 *
 * AI concepts should remain open-world wherever possible.
 *
 * Domain concepts such as:
 *
 *     model
 *     dataset
 *     tensor
 *     reason
 *     infer
 *     deduce
 *     learn
 *     adapt
 *     knowledge
 *     evidence
 *     explain
 *     policy
 *     provenance
 *     agent
 *
 * must not automatically become permanent lexer keywords.
 *
 * Annotation-led and identifier-led extension mechanisms remain available
 * through the canonical lexer vocabulary.
 *
 * ============================================================================
 * UNIVERSAL TYPE AUTHORITY
 * ============================================================================
 *
 * AI grammars consume the canonical type system.
 *
 * They MUST NOT introduce competing type systems such as:
 *
 *     AIType
 *     ModelType
 *     DatasetType
 *     TensorType
 *     AgentType
 *
 * unless a future semantic specification explicitly requires a named semantic
 * type, in which case the type remains represented through the canonical
 * type-system infrastructure.
 *
 * ============================================================================
 * UNIVERSAL EXPRESSION AUTHORITY
 * ============================================================================
 *
 * AI grammars consume the canonical expression grammar.
 *
 * AI.g4 MUST NOT create another expression hierarchy.
 *
 * Therefore:
 *
 *     expression
 *
 * remains the common mechanism for:
 *
 *     classical values
 *     tensor values
 *     model references
 *     datasets
 *     symbolic values
 *     probability values
 *     quantum values
 *     hybrid values
 *     distributed values
 *     future values.
 *
 * ============================================================================
 * UNIVERSAL STATEMENT AUTHORITY
 * ============================================================================
 *
 * AI bodies may use ordinary Zamani statements wherever the owning leaf
 * grammar permits them.
 *
 * AI does not create a second statement language.
 *
 * ============================================================================
 * CANONICAL AI LEAF GRAMMARS
 * ============================================================================
 *
 * The AI directory contains multiple independently-owned feature grammars.
 *
 * The composition root imports the canonical grammar identities.
 *
 * Core computational domains:
 *
 *     models.g4
 *     datasets.g4
 *     tensors.g4
 *     training.g4
 *     inference.g4
 *     differentiation.g4
 *     pipelines.g4
 *     ai-accelerators.g4
 *     model-deployment.g4
 *
 * Agent/domain-composition:
 *
 *     agents.g4
 *     multi-agent.g4
 *     cognitive.g4
 *     embedded.g4
 *
 * Reasoning:
 *
 *     reasoning.g4
 *     induction.g4
 *     deduction.g4
 *     abduction.g4
 *
 * Knowledge:
 *
 *     knowledge.g4
 *     facts.g4
 *     assertions.g4
 *     retraction.g4
 *     queries.g4
 *
 * Learning/adaptation:
 *
 *     learning.g4
 *     adaptation.g4
 *     feedback.g4
 *     transfer-learning.g4
 *     reinforcement.g4
 *     federated-learning.g4
 *
 * Neural/symbolic:
 *
 *     neural.g4
 *     symbolic.g4
 *     neural-symbolic.g4
 *
 * Uncertainty/probability:
 *
 *     uncertainty.g4
 *     probability.g4
 *     probabilistic.g4
 *     distributions.g4
 *     confidence.g4
 *
 * Causality/evidence/explanation:
 *
 *     causality.g4
 *     evidence.g4
 *     explanations.g4
 *     decisions.g4
 *     provenance.g4
 *
 * AI infrastructure:
 *
 *     ai-capabilities.g4
 *     policies.g4
 *     effects.g4
 *     parameters.g4
 *
 * ============================================================================
 * LEGACY DUPLICATE POLICY
 * ============================================================================
 *
 * The repository contains:
 *
 *     model.g4
 *     models.g4
 *
 * `models.g4` is the canonical model grammar.
 *
 * `model.g4` MUST NOT be imported by this file.
 *
 * The repository also contains:
 *
 *     agent.g4
 *     agents.g4
 *
 * `agents.g4` is the canonical agent grammar.
 *
 * `agent.g4` MUST NOT be imported here as a competing grammar.
 *
 * Legacy files may remain temporarily as compatibility material, but they
 * cannot participate in canonical composition if they duplicate parser grammar
 * identities or public rules.
 *
 * ============================================================================
 * IMPORT GRAPH
 * ============================================================================
 *
 * The intended direction is:
 *
 *     ZamaniParser
 *          |
 *          v
 *         AI
 *          |
 *     +----+-------------------------------+
 *     |                                    |
 *     v                                    v
 *  canonical AI leaves              universal grammar
 *
 * AI leaf grammars may themselves depend on canonical universal grammars.
 *
 * ai.g4 must not introduce reverse dependencies from universal grammar
 * infrastructure back into AI-specific implementations.
 *
 * ============================================================================
 */

parser grammar AI;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * CANONICAL AI IMPORTS
 * ============================================================================
 *
 * IMPORTANT:
 *
 * Imports are composition dependencies, not ownership transfers.
 *
 * Every imported grammar remains independently testable.
 *
 * ============================================================================
 */

import
    Types,
    Expressions,
    Statements,

    AIModels,
    AIDatasets,
    AITensors,
    AITraining,
    Inference,
    Agents,
    AIDifferentiable,
    AIPipelines,
    AIAccelerators,
    ModelDeployment,

    AIReasoning,
    AIInduction,
    Deduction,
    Abduction,

    AIKnowledge,
    AIFacts,
    AIAssertions,
    AIRetraction,
    AIQueries,

    AILearning,
    AIAdaptation,
    Feedback,
    AITransferLearning,
    AIReinforcement,
    AIFederatedLearning,

    Neural,
    Symbolic,
    NeuralSymbolic,

    AIUncertainty,
    AIProbability,
    AIProbabilistic,
    Distributions,
    AIConfidence,

    AICausality,
    AIEvidence,
    AIExplanations,
    AIDecisions,
    AIProvenance,

    Cognitive,
    MultiAgent,
    AIEmbedded,

    AICapabilities,
    AIPolicies,
    AIEffects,
    AIParameters
;


/*
 * ============================================================================
 * PUBLIC AI COMPOSITION ENTRY
 * ============================================================================
 *
 * Every construct entering the AI parser domain MUST pass through this rule.
 *
 * aiConstruct contains only references to independently-owned public rules.
 *
 * It does not duplicate their syntax.
 *
 * ============================================================================
 */

aiConstruct
    : aiCoreConstruct
    | aiReasoningConstruct
    | aiKnowledgeConstruct
    | aiLearningConstruct
    | aiUncertaintyConstruct
    | aiEvidenceConstruct
    | aiExplanationConstruct
    | aiCausalityConstruct
    | aiAgentConstruct
    | aiNeuralSymbolicConstruct
    | aiInfrastructureConstruct
    ;


/*
 * ============================================================================
 * CORE AI COMPUTATION
 * ============================================================================
 *
 * The core computational AI family remains separately owned.
 *
 * ============================================================================
 */

aiCoreConstruct
    : aiModelConstruct
    | datasetConstruct
    | tensorConstruct
    | trainingConstruct
    | inferenceConstruct
    | aiDifferentiationConstruct
    | pipelineConstruct
    | aiAcceleratorConstruct
    | deploymentConstruct
    ;


/*
 * ============================================================================
 * REASONING
 * ============================================================================
 *
 * Reasoning is composed from the specialized reasoning families.
 *
 * This is deliberately a semantic family rather than a single keyword list.
 *
 * ============================================================================
 */

aiReasoningConstruct
    : aiReasoningConstruct
    | aiInductionConstruct
    | deductionConstruct
    | aiAbductionConstruct
    ;


/*
 * ============================================================================
 * KNOWLEDGE
 * ============================================================================
 *
 * Knowledge operations remain separate owners:
 *
 *     facts
 *     assertions
 *     retraction
 *     queries
 *
 * ============================================================================
 */

aiKnowledgeConstruct
    : aiKnowledgeConstruct
    | fact
    | aiAssertionConstruct
    | aiRetractionConstruct
    | aiQueryConstruct
    ;


/*
 * ============================================================================
 * LEARNING / ADAPTATION
 * ============================================================================
 */

aiLearningConstruct
    : aiLearningConstruct
    | aiAdaptationConstruct
    | feedbackConstruct
    | transferLearningConstruct
    | reinforcementLearningConstruct
    | federatedLearningConstruct
    ;


/*
 * ============================================================================
 * UNCERTAINTY
 * ============================================================================
 */

aiUncertaintyConstruct
    : aiUncertaintyConstruct
    | aiProbabilityConstruct
    | aiProbabilisticConstruct
    | distributionConstruct
    | aiConfidenceConstruct
    ;


/*
 * ============================================================================
 * EVIDENCE / EXPLANATION
 * ============================================================================
 */

aiEvidenceConstruct
    : aiEvidenceConstruct
    | aiEvidenceStatement
    ;

aiExplanationConstruct
    : aiExplanationConstruct
    | aiExplanationStatement
    ;


/*
 * ============================================================================
 * CAUSALITY
 * ============================================================================
 */

aiCausalityConstruct
    : aiCausalityConstruct
    ;


/*
 * ============================================================================
 * AGENTS
 * ============================================================================
 */

aiAgentConstruct
    : agentConstruct
    | multiAgentConstruct
    | cognitiveConstruct
    | embeddedAIConstruct
    ;


/*
 * ============================================================================
 * NEURAL / SYMBOLIC COMPOSITION
 * ============================================================================
 */

aiNeuralSymbolicConstruct
    : neuralConstruct
    | symbolicConstruct
    | neuralSymbolicConstruct
    ;


/*
 * ============================================================================
 * AI INFRASTRUCTURE
 * ============================================================================
 */

aiInfrastructureConstruct
    : aiCapabilityConstruct
    | aiPolicyConstruct
    | aiEffectConstruct
    | aiParameterConstruct
    ;


/*
 * ============================================================================
 * IMPORTANT IMPLEMENTATION NOTE
 * ============================================================================
 *
 * The family wrapper rules above MUST NOT recursively reference themselves.
 *
 * Their names intentionally differ from leaf public rules where required.
 *
 * If an imported leaf already exposes the family name, the root composition
 * must use a distinct wrapper name rather than:
 *
 *     aiReasoningConstruct
 *         : aiReasoningConstruct
 *
 * because that would create immediate left recursion and/or duplicate rule
 * ownership.
 *
 * The canonical implementation therefore uses the explicit leaf rules below.
 * ============================================================================
 */


/*
 * ============================================================================
 * CANONICAL DISPATCH
 * ============================================================================
 *
 * The following rules are the actual non-recursive dispatch boundary.
 *
 * ============================================================================
 */

aiCoreConstruct
    : aiModelConstruct
    | datasetConstruct
    | tensorConstruct
    | trainingConstruct
    | inferenceConstruct
    | aiDifferentiationConstruct
    | pipelineConstruct
    | aiAcceleratorConstruct
    | deploymentConstruct
    ;

aiReasoningFamily
    : aiReasoningStatement
    | aiInductionConstruct
    | deductionConstruct
    | aiAbductionConstruct
    ;

aiKnowledgeFamily
    : aiKnowledgeConstruct
    | fact
    | aiAssertionConstruct
    | aiRetractionConstruct
    | aiQueryConstruct
    ;

aiLearningFamily
    : aiLearningConstruct
    | aiAdaptationConstruct
    | feedbackConstruct
    | transferLearningConstruct
    | reinforcementLearningConstruct
    | federatedLearningConstruct
    ;

aiUncertaintyFamily
    : aiUncertaintyConstruct
    | aiProbabilityConstruct
    | aiProbabilisticConstruct
    | distributionConstruct
    | aiConfidenceConstruct
    ;

aiEvidenceFamily
    : aiEvidenceConstruct
    ;

aiExplanationFamily
    : aiExplanationConstruct
    ;

aiCausalityFamily
    : aiCausalityConstruct
    ;

aiAgentFamily
    : agentConstruct
    | multiAgentConstruct
    | cognitiveConstruct
    | embeddedAIConstruct
    ;

aiNeuralSymbolicFamily
    : neuralConstruct
    | symbolicConstruct
    | neuralSymbolicConstruct
    ;

aiInfrastructureFamily
    : aiCapabilityConstruct
    | aiPolicyConstruct
    | aiEffectConstruct
    | aiParameterConstruct
    ;


/*
 * ============================================================================
 * FINAL AI DISPATCH
 * ============================================================================
 *
 * This is deliberately the only public composition dispatcher.
 *
 * ============================================================================
 */

aiDomainConstruct
    : aiCoreConstruct
    | aiReasoningFamily
    | aiKnowledgeFamily
    | aiLearningFamily
    | aiUncertaintyFamily
    | aiEvidenceFamily
    | aiExplanationFamily
    | aiCausalityFamily
    | aiAgentFamily
    | aiNeuralSymbolicFamily
    | aiInfrastructureFamily
    ;


/*
 * ============================================================================
 * AI DOMAIN BOUNDARY
 * ============================================================================
 *
 * ZamaniParser.g4 should consume:
 *
 *     aiDomainConstruct
 *
 * as the canonical AI parser boundary.
 *
 * If the existing root parser already consumes:
 *
 *     aiConstruct
 *
 * then aiConstruct remains the compatibility façade and delegates directly
 * to aiDomainConstruct.
 *
 * ============================================================================
 */

aiConstruct
    : aiDomainConstruct
    ;


/*
 * ============================================================================
 * UNIVERSAL VALUE BRIDGES
 * ============================================================================
 *
 * These are intentionally not part of aiDomainConstruct.
 *
 * An arbitrary expression must never become an AI construct merely because it
 * happens to occur inside an AI-capable program.
 *
 * ============================================================================
 */

aiValue
    : expression
    ;

aiType
    : typeExpression
    ;

aiExpression
    : expression
    ;

aiModelReference
    : expression
    ;

aiDatasetReference
    : expression
    ;

aiTensorReference
    : expression
    ;

aiTrainingReference
    : expression
    ;

aiInferenceReference
    : expression
    ;

aiAgentReference
    : expression
    ;

aiPipelineReference
    : expression
    ;

aiAcceleratorReference
    : expression
    ;


/*
 * ============================================================================
 * RESOURCE / CAPABILITY BOUNDARY
 * ============================================================================
 *
 * AI requirements describe intent.
 *
 * They do not identify physical resources.
 *
 * Examples of valid semantic intent include:
 *
 *     requires capability("tensor.compute")
 *     requires capability("quantum.measurement")
 *     requires memory >= required_memory
 *     requires qubits >= required_qubits
 *     requires topology(required_topology)
 *
 * The resource system decides whether the requirement can be satisfied.
 *
 * ai.g4 performs no resource discovery.
 *
 * ============================================================================
 */

aiResourceIntent
    : aiCapabilityConstruct
    ;


/*
 * ============================================================================
 * EFFECT BOUNDARY
 * ============================================================================
 *
 * AI operations can carry effects such as:
 *
 *     IO
 *     network
 *     mutation
 *     randomness
 *     native
 *     foreign
 *     distributed
 *     measurement
 *     learning
 *     adaptation
 *     reflection
 *     code generation
 *     simulation
 *
 * Effect semantics belong to the effect subsystem.
 *
 * ============================================================================
 */

aiEffectIntent
    : aiEffectConstruct
    ;


/*
 * ============================================================================
 * POLICY BOUNDARY
 * ============================================================================
 */

aiPolicyIntent
    : aiPolicyConstruct
    ;


/*
 * ============================================================================
 * PROVENANCE BOUNDARY
 * ============================================================================
 *
 * Provenance is semantic metadata.
 *
 * AI provenance can describe:
 *
 *     source
 *     derived_from
 *     generated_by
 *     transformed_by
 *     verified_by
 *     evidence
 *     decision
 *     version
 *     execution context
 *
 * ai.g4 does not construct provenance records.
 *
 * ============================================================================
 */

aiProvenanceIntent
    : aiProvenanceConstruct
    ;


/*
 * ============================================================================
 * QUANTUM INTEGRATION CONTRACT
 * ============================================================================
 *
 * AI may participate in hybrid quantum/classical computation.
 *
 * ai.g4 MUST NOT define:
 *
 *     quantum operations
 *     qubits
 *     physical qubits
 *     topology
 *     routing
 *     scheduling
 *     calibration
 *     QEC
 *     QZN/ZQN
 *     HAL
 *
 * Quantum semantics remain owned by the quantum subsystem.
 *
 * The canonical boundary remains:
 *
 *     source
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic model
 *       |
 *       v
 *     quantum::ir
 *
 * AI can contribute semantic intent to that path without introducing another
 * quantum IR.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * CLASSICAL / HDL / HARDWARE / DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * AI is one domain of the language.
 *
 * It can compose with:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     hardware
 *     distributed
 *     networking
 *     security
 *     data
 *     resources
 *     execution
 *     compilation
 *     interoperability
 *     metaprogramming
 *
 * None of those domains are redefined here.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * OPEN-WORLD EXTENSION CONTRACT
 * ============================================================================
 *
 * New AI capabilities MUST normally be added by:
 *
 *     1. creating or completing one leaf grammar;
 *     2. giving that grammar one stable parser identity;
 *     3. exposing one clearly documented public rule;
 *     4. documenting its AST contract;
 *     5. documenting its semantic contract;
 *     6. documenting effects/resources/capabilities/policies;
 *     7. documenting its canonical IR destination;
 *     8. adding positive/negative/boundary/scalability tests;
 *     9. adding that grammar to this composition root.
 *
 * The universal language root should not need to be modified merely because
 * an AI capability was added.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * DIALECT / FRAMEWORK / VENDOR BOUNDARY
 * ============================================================================
 *
 * ai.g4 intentionally does not reserve syntax for:
 *
 *     CUDA
 *     ROCm
 *     TensorFlow
 *     PyTorch
 *     JAX
 *     ONNX
 *     OpenVINO
 *     individual accelerator vendors
 *     cloud providers
 *     particular model families
 *
 * Such integrations belong to:
 *
 *     grammar/dialects/
 *     grammar/interoperability/
 *     grammar/hardware/
 *     grammar/compile/
 *     grammar/execution/
 *
 * They must lower into canonical Zamani semantics.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * POCO-REAF / SCALABILITY CONTRACT
 * ============================================================================
 *
 * ai.g4 contains NO universal machine-capacity constants.
 *
 * It imposes no language-level maximum on:
 *
 *     models
 *     parameters
 *     layers
 *     datasets
 *     records
 *     tensors
 *     tensor rank
 *     tensor dimensions
 *     training steps
 *     inference operations
 *     agents
 *     workers
 *     tasks
 *     pipeline stages
 *     accelerators
 *     devices
 *     CPUs
 *     GPUs
 *     FPGAs
 *     ASICs
 *     QPUs
 *     qubits
 *     nodes
 *     threads
 *     memory
 *     network topology
 *
 * No constants such as:
 *
 *     MAX_MODELS
 *     MAX_PARAMETERS
 *     MAX_LAYERS
 *     MAX_TENSORS
 *     MAX_TENSOR_RANK
 *     MAX_DATASET_SIZE
 *     MAX_AGENTS
 *     MAX_WORKERS
 *     MAX_NODES
 *     MAX_GPUS
 *     MAX_QUBITS
 *
 * may appear in this grammar.
 *
 * Resource limitations are properties of:
 *
 *     semantic validation
 *     compilation
 *     target capabilities
 *     deployment policy
 *     runtime resources
 *
 * rather than the language grammar.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no embedded Rust;
 *     no actions;
 *     no semantic predicates;
 *     no filesystem access;
 *     no network access;
 *     no hardware discovery;
 *     no environment inspection;
 *     no randomness;
 *     no runtime execution.
 *
 * For a fixed lexer vocabulary, grammar version and source token stream,
 * parsing is deterministic.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * SAFE-RUST CONTRACT
 * ============================================================================
 *
 * The generated parser is consumed by the Rust frontend.
 *
 * Repository requirements:
 *
 *     Rust 1.97 or later
 *     Rust 2021
 *     safe Rust
 *     no unsafe implementation requirement
 *
 * ai.g4 itself contains no Rust implementation code.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * ai.g4 produces parser contexts only.
 *
 * The AST remains domain-neutral.
 *
 * AI information preserved through parsing includes, where applicable:
 *
 *     source span
 *     construct kind
 *     names
 *     qualified names
 *     annotations
 *     parameters
 *     types
 *     expressions
 *     relationships
 *     nested structure
 *     ordering
 *     metadata
 *     provenance references
 *
 * The AST must not become tied to:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     vendor
 *     cloud provider
 *     physical topology
 *     specific ML framework
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis determines:
 *
 *     name resolution
 *     type validity
 *     shape validity
 *     model validity
 *     data compatibility
 *     reasoning validity
 *     probability validity
 *     policy validity
 *     effect validity
 *     capability requirements
 *     resource requirements
 *     provenance
 *     portability
 *     dialect compatibility
 *     quantum compatibility
 *     hardware feasibility
 *
 * Parsing must not depend upon whether a particular machine currently exists
 * or has sufficient resources.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * ai.g4 creates NO IR.
 *
 * There is intentionally no:
 *
 *     AIIR
 *     AIUniversalIR
 *     AIQuantumIR
 *     AIHardwareIR
 *     AgentIR
 *
 * AI semantics lower through the canonical semantic/IR architecture.
 *
 * Possible downstream destinations include:
 *
 *     classical IR
 *     tensor/data computation
 *     accelerator IR
 *     distributed IR
 *     hardware/HDL IR
 *     quantum::ir
 *
 * The destination is selected by semantic analysis and compilation.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * COMPILER / RUNTIME CONTRACT
 * ============================================================================
 *
 * Compiler responsibilities:
 *
 *     specialization
 *     optimization
 *     lowering
 *     target realization
 *     scheduling
 *     routing
 *     deployment planning
 *
 * Runtime responsibilities:
 *
 *     resource discovery
 *     execution
 *     monitoring
 *     recovery
 *     adaptation
 *     lifecycle
 *
 * ai.g4 performs none of these.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * REQUIRED TESTING CONTRACT
 * ============================================================================
 *
 * ai.g4 is complete only when the following are tested.
 *
 * --------------------------------------------------------------------------
 * Composition
 * --------------------------------------------------------------------------
 *
 * Every canonical AI leaf grammar must be reachable through aiConstruct.
 *
 * --------------------------------------------------------------------------
 * Positive
 * --------------------------------------------------------------------------
 *
 * Valid examples for:
 *
 *     model
 *     dataset
 *     tensor
 *     training
 *     inference
 *     differentiation
 *     pipeline
 *     accelerator
 *     deployment
 *     reasoning
 *     induction
 *     deduction
 *     abduction
 *     knowledge
 *     facts
 *     assertions
 *     retraction
 *     queries
 *     learning
 *     adaptation
 *     feedback
 *     transfer learning
 *     reinforcement learning
 *     federated learning
 *     uncertainty
 *     probability
 *     probabilistic computation
 *     distributions
 *     confidence
 *     causality
 *     evidence
 *     explanations
 *     decisions
 *     provenance
 *     agents
 *     multi-agent computation
 *     cognitive computation
 *     embedded AI
 *     neural computation
 *     symbolic computation
 *     neural-symbolic composition
 *     capabilities
 *     policies
 *     effects
 *     parameters
 *
 * --------------------------------------------------------------------------
 * Negative
 * --------------------------------------------------------------------------
 *
 * Verify that:
 *
 *     malformed constructs fail;
 *     malformed annotation structures fail;
 *     incomplete declarations fail;
 *     invalid nesting fails;
 *     ordinary arbitrary expressions are not accidentally classified as AI
 *     domain constructs;
 *     duplicate grammar ownership is not introduced.
 *
 * --------------------------------------------------------------------------
 * Boundary
 * --------------------------------------------------------------------------
 *
 * Test:
 *
 *     empty constructs where permitted;
 *     single-member constructs;
 *     many-member constructs;
 *     nested constructs;
 *     deeply nested constructs;
 *     symbolic quantities;
 *     symbolic dimensions;
 *     symbolic resource requirements;
 *     cross-domain expressions;
 *     quantum/classical hybrid composition;
 *     AI/concurrency composition;
 *     AI/distributed composition;
 *     AI/HDL composition.
 *
 * --------------------------------------------------------------------------
 * Scalability
 * --------------------------------------------------------------------------
 *
 * Tests must verify the absence of language-level ceilings.
 *
 * The grammar must not impose artificial limits on:
 *
 *     number of models;
 *     number of parameters;
 *     tensor rank;
 *     tensor dimensions;
 *     number of agents;
 *     pipeline stages;
 *     training operations;
 *     inference operations;
 *     workers;
 *     nodes;
 *     devices;
 *     accelerators.
 *
 * --------------------------------------------------------------------------
 * Determinism
 * --------------------------------------------------------------------------
 *
 * Identical source/token streams must produce equivalent parse structures.
 *
 * --------------------------------------------------------------------------
 * Portability
 * --------------------------------------------------------------------------
 *
 * The same source syntax must remain independent of:
 *
 *     CPU count;
 *     GPU count;
 *     accelerator count;
 *     node count;
 *     memory size;
 *     QPU size;
 *     physical topology.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * UPSTREAM
 * --------
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/antlr/ZamaniParser.g4
 *     grammar/core/
 *     grammar/types/
 *     grammar/expressions/
 *     grammar/statements/
 *
 * DOWNSTREAM
 * ----------
 *
 *     frontend AST
 *     structural validation
 *     semantic analysis
 *     type checking
 *     effect checking
 *     capability checking
 *     resource analysis
 *     policy evaluation
 *     provenance
 *     canonical IR
 *
 * CROSS-DOMAIN
 * ------------
 *
 *     grammar/classical/
 *     grammar/quantum/
 *     grammar/hybrid/
 *     grammar/hdl/
 *     grammar/hardware/
 *     grammar/concurrency/
 *     grammar/distributed/
 *     grammar/networking/
 *     grammar/data/
 *     grammar/security/
 *     grammar/resources/
 *     grammar/execution/
 *     grammar/compile/
 *     grammar/interoperability/
 *     grammar/dialects/
 *     grammar/metaprogramming/
 *
 * QUANTUM
 * -------
 *
 * AI semantics may lower into:
 *
 *     quantum::ir
 *
 * ai.g4 must never introduce another quantum IR.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * `aiConstruct` remains the stable public compatibility entry point.
 *
 * New AI features should be added beneath aiDomainConstruct without requiring
 * the universal root grammar to know every leaf grammar.
 *
 * Deprecated AI grammars remain outside canonical composition.
 *
 * In particular:
 *
 *     model.g4
 *     agent.g4
 *
 * must not be imported when they duplicate canonical grammar identities.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * EXTENSION PROCEDURE
 * ============================================================================
 *
 * To add a future AI feature:
 *
 *     1. create/complete its independent grammar;
 *     2. define one canonical parser grammar identity;
 *     3. define its public parser rule;
 *     4. define ownership;
 *     5. define AST mapping;
 *     6. define semantic mapping;
 *     7. define type/effect/resource/capability behavior;
 *     8. define policy/provenance behavior;
 *     9. define canonical IR destination;
 *    10. define diagnostics;
 *    11. add positive/negative/boundary/scalability tests;
 *    12. add the grammar import here;
 *    13. add exactly one dispatch alternative here.
 *
 * No application-specific keyword explosion is permitted.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * FINAL ARCHITECTURAL INVARIANT
 * ============================================================================
 *
 * ai.g4 answers one question:
 *
 *     "Which independently-owned AI grammars participate in the canonical
 *      AI parser boundary?"
 *
 * It does NOT answer:
 *
 *     "How does AI execute?"
 *     "Which hardware executes it?"
 *     "How many GPUs exist?"
 *     "How much memory exists?"
 *     "How many quantum resources exist?"
 *     "How is a model routed?"
 *     "How is a model scheduled?"
 *     "How is QEC performed?"
 *     "How is hardware calibrated?"
 *
 * Those responsibilities belong downstream.
 *
 * The portability invariant remains:
 *
 *     Program Once
 *          ->
 *     Compile Once
 *          ->
 *     Run Everywhere
 *          ->
 *     Run Anywhere
 *          ->
 *     Forever
 *
 * subject to declared semantics, requirements, capabilities, policies and
 * actual resources available during realization.
 *
 * ============================================================================
 */
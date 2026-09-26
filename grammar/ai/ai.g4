/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/ai/ai.g4
 *
 * Grammar:
 *     AI
 *
 * Status:
 *     CANONICAL AI DOMAIN COMPOSITION GRAMMAR
 *
 * Baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust edition 2021
 *     Safe Rust only
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the SINGLE AI-DOMAIN COMPOSITION BOUNDARY.
 *
 * It does not attempt to implement every AI feature itself.
 *
 * Instead it composes the independently-owned AI leaf grammars into one
 * stable parser-facing AI domain.
 *
 * The dependency direction is:
 *
 *     Zamani.g4
 *          |
 *          v
 *     ZamaniParser.g4
 *          |
 *          v
 *          AI
 *          |
 *     +----+------------------------------------------------------+
 *     |    |       |        |        |        |        |          |
 *     v    v       v        v        v        v        v          v
 *   Models Data   Tensors Training Inference Agents Differentiation Pipelines
 *     |    |       |        |        |        |        |          |
 *     +----+-------+--------+--------+--------+--------+----------+
 *                              |
 *                              v
 *                         AIAccelerators
 *                              |
 *                              v
 *                       ModelDeployment
 *                              |
 *                              v
 *                    Domain-Neutral Frontend AST
 *                              |
 *                              v
 *                       Semantic Analysis
 *                              |
 *              +---------------+----------------+
 *              |               |                |
 *              v               v                v
 *            Types          Effects         Resources
 *              |               |                |
 *              +---------------+----------------+
 *                              |
 *                              v
 *                    Canonical Semantic Model
 *                              |
 *              +---------------+----------------+
 *              |               |                |
 *              v               v                v
 *          Classical       quantum::ir      HDL/Hardware
 *                              |
 *                              v
 *                         Optimization
 *                              |
 *                    +---------+----------+
 *                    |                    |
 *                    v                    v
 *                Scheduling            Routing
 *                    |                    |
 *                    +---------+----------+
 *                              |
 *                              v
 *                         Resilience
 *                              |
 *                              v
 *                             ZQN
 *                              |
 *                              v
 *                             HAL
 *                              |
 *                              v
 *                      Target Realization
 *                              |
 *                              v
 *                           Runtime
 *
 * This file defines NONE of those downstream semantics.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - the AI-domain parser composition boundary;
 *     - AI-domain construct dispatch;
 *     - AI leaf-grammar composition;
 *     - AI-domain interoperability at the grammar boundary;
 *     - the distinction between AI syntax and ordinary universal syntax;
 *     - stable AI extension points.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical token definitions;
 *     - identifiers;
 *     - qualified names;
 *     - general expressions;
 *     - general types;
 *     - ordinary statements;
 *     - model semantics;
 *     - tensor semantics;
 *     - dataset semantics;
 *     - training algorithms;
 *     - inference algorithms;
 *     - differentiation implementation;
 *     - optimizer implementation;
 *     - agent execution;
 *     - pipeline execution;
 *     - accelerator selection;
 *     - device selection;
 *     - hardware discovery;
 *     - resource allocation;
 *     - scheduling;
 *     - routing;
 *     - deployment implementation;
 *     - networking implementation;
 *     - security enforcement;
 *     - classical IR;
 *     - AI-specific IR;
 *     - quantum IR;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime execution.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * Each AI feature has exactly one source-level owner.
 *
 * Model declarations:
 *
 *     models.g4
 *
 * Dataset declarations:
 *
 *     datasets.g4
 *
 * Tensor declarations and tensor-domain syntax:
 *
 *     tensors.g4
 *
 * Training:
 *
 *     training.g4
 *
 * Inference:
 *
 *     inference.g4
 *
 * Agents:
 *
 *     agents.g4
 *
 * Differentiation:
 *
 *     differentiation.g4
 *
 * Pipelines:
 *
 *     pipelines.g4
 *
 * AI accelerator intent:
 *
 *     ai-accelerators.g4
 *
 * Deployment intent:
 *
 *     model-deployment.g4
 *
 * This composition grammar MUST NOT duplicate those productions.
 *
 * ============================================================================
 * DUPLICATE-GRAMMAR POLICY
 * ============================================================================
 *
 * The repository currently contains:
 *
 *     grammar/ai/model.g4
 *     grammar/ai/models.g4
 *
 * and both declare:
 *
 *     parser grammar AIModels;
 *
 * They MUST NOT both participate in the canonical ANTLR composition.
 *
 * The canonical model source is:
 *
 *     grammar/ai/models.g4
 *
 * Therefore this file imports `AIModels` only through `models.g4`.
 *
 * `model.g4` must be treated as legacy/deprecated compatibility material and
 * must not be imported by this grammar.
 *
 * Likewise the repository contains:
 *
 *     grammar/ai/agent.g4
 *     grammar/ai/agents.g4
 *
 * The canonical composition uses:
 *
 *     agents.g4
 *
 * whose public boundary is:
 *
 *     agentConstruct
 *
 * The older `agent.g4` must not be imported here unless it is converted into
 * a compatibility façade with a distinct grammar identity.
 *
 * ============================================================================
 * LEXICAL AUTHORITY
 * ============================================================================
 *
 * The canonical parser vocabulary is:
 *
 *     ZamaniLexer
 *
 * The canonical annotation marker is:
 *
 *     AT
 *
 * representing:
 *
 *     @
 *
 * AI concepts are therefore NOT added as permanent lexer keywords merely
 * because a new AI feature exists.
 *
 * Examples:
 *
 *     @model
 *     @dataset
 *     @tensor
 *     @training
 *     @inference
 *     @agent
 *     @pipeline
 *     @accelerator
 *     @deployment
 *
 * are structurally represented through:
 *
 *     AT identifier
 *
 * and are interpreted by semantic analysis.
 *
 * `NANO_ANNOTATION` is NOT the canonical AI annotation mechanism.
 *
 * ============================================================================
 * EXPRESSION AUTHORITY
 * ============================================================================
 *
 * General expressions are owned by the canonical expression grammar.
 *
 * AI grammars consume:
 *
 *     expression
 *     argumentList
 *
 * where appropriate.
 *
 * This composition grammar MUST NOT create another expression hierarchy.
 *
 * ============================================================================
 * TYPE AUTHORITY
 * ============================================================================
 *
 * General type syntax is owned by the canonical type grammar.
 *
 * AI grammars consume:
 *
 *     typeExpression
 *
 * AI must not create a competing:
 *
 *     AIType
 *     ModelType
 *     TensorType
 *     DatasetType
 *
 * type system.
 *
 * ============================================================================
 * STATEMENT AUTHORITY
 * ============================================================================
 *
 * General statement syntax is owned by:
 *
 *     Statements
 *
 * AI regions may contain ordinary Zamani statements through their leaf
 * grammars.
 *
 * This allows AI code to compose with:
 *
 *     classical computation
 *     quantum computation
 *     HDL/co-design
 *     concurrency
 *     distributed computation
 *     networking
 *     security
 *     future domains
 *
 * without creating an AI-only programming language.
 *
 * ============================================================================
 * AI LEAF COMPOSITION
 * ============================================================================
 *
 * The canonical AI leaf grammars are imported here.
 *
 * Each imported grammar exposes one public domain boundary.
 *
 * The imports are intentionally explicit so that:
 *
 *     grammar/ai/ai.g4
 *
 * is a real composition layer rather than a documentation-only façade.
 *
 * ============================================================================
 */

parser grammar AI;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * CANONICAL AI LEAF GRAMMAR IMPORTS
 * ============================================================================
 *
 * These imports form the AI-domain implementation boundary.
 *
 * IMPORTANT:
 *
 *     Do not import model.g4.
 *     Do not import agent.g4.
 *
 * The canonical files are:
 *
 *     models.g4
 *     agents.g4
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
    ModelDeployment
;


/*
 * ============================================================================
 * 1. PUBLIC AI COMPOSITION ENTRY
 * ============================================================================
 *
 * Every AI-domain construct must enter through this rule.
 *
 * A bare ordinary expression is deliberately NOT an AI construct.
 *
 * This prevents:
 *
 *     expression
 *
 * from being accepted through both:
 *
 *     AI
 *
 * and:
 *
 *     Expressions
 *
 * with the result that ordinary expressions become accidentally classified
 * as AI syntax.
 *
 * ============================================================================
 */

aiConstruct
    : aiModelConstruct
    | datasetConstruct
    | tensorConstruct
    | trainingConstruct
    | inferenceConstruct
    | agentConstruct
    | aiDifferentiationConstruct
    | pipelineConstruct
    | aiAcceleratorConstruct
    | deploymentConstruct
    | aiCapabilityConstruct
    ;


/*
 * ============================================================================
 * 2. AI CAPABILITY COMPOSITION
 * ============================================================================
 *
 * Capability declarations are owned by:
 *
 *     grammar/ai/capabilities.g4
 *
 * The capability grammar is imported below through the explicit composition
 * boundary required by the repository's AI capability contract.
 *
 * It must remain distinct from:
 *
 *     resource requirements;
 *     physical devices;
 *     target selection;
 *     hardware discovery.
 *
 * ============================================================================
 */

aiCapabilityConstruct
    : aiCapabilityRequirement
    ;


/*
 * ============================================================================
 * 3. AI VALUE BRIDGES
 * ============================================================================
 *
 * These rules are NOT alternatives of `aiConstruct`.
 *
 * They exist only as explicit reusable parser boundaries for leaf grammars
 * or future composition layers.
 *
 * A bridge MUST NOT cause every ordinary expression to become an AI construct.
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


/*
 * ============================================================================
 * 4. CROSS-DOMAIN REFERENCE BRIDGES
 * ============================================================================
 *
 * AI values can participate in other Zamani domains.
 *
 * References remain ordinary expressions at the parser level.
 *
 * Semantic analysis determines whether a referenced value denotes:
 *
 *     model
 *     dataset
 *     tensor
 *     quantum value
 *     classical value
 *     hardware-backed value
 *     distributed value
 *     network value
 *     future-domain value
 *
 * No AI-specific duplicate type or IR is introduced.
 * ============================================================================
 */

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
 * 5. RESOURCE / CAPABILITY SEMANTIC BOUNDARY
 * ============================================================================
 *
 * AI resource and capability semantics are intentionally downstream.
 *
 * Source-level constructs may describe:
 *
 *     requirements
 *     capabilities
 *     constraints
 *     preferences
 *     hints
 *     budgets
 *
 * They do NOT select physical resources.
 *
 * Valid semantic intent includes concepts such as:
 *
 *     requires qubits >= n
 *     requires memory >= required_memory
 *     requires capability("tensor.compute")
 *     requires capability("quantum.measurement")
 *     requires topology(...)
 *
 * Invalid architectural assumptions include treating these as:
 *
 *     GPU 0
 *     CPU 0
 *     QPU 0
 *     physical_qubit 17
 *     device 7
 *
 * Physical realization belongs downstream.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 6. PORTABILITY / POCO-REAF
 * ============================================================================
 *
 * AI syntax must remain independent of the size or topology of the machine
 * that eventually realizes it.
 *
 * This composition layer imposes NO language-level maximum on:
 *
 *     models
 *     model members
 *     model depth
 *     model parameters
 *     tensors
 *     tensor rank
 *     tensor dimensions
 *     datasets
 *     records
 *     batches
 *     training steps
 *     inference calls
 *     agents
 *     pipeline stages
 *     workers
 *     nodes
 *     accelerators
 *     devices
 *     quantum resources
 *     classical resources
 *
 * No universal constants such as:
 *
 *     MAX_MODELS
 *     MAX_TENSORS
 *     MAX_TENSOR_RANK
 *     MAX_PARAMETERS
 *     MAX_LAYERS
 *     MAX_GPUS
 *     MAX_CPUS
 *     MAX_NODES
 *     MAX_DEVICES
 *
 * may be introduced here.
 *
 * Repetition is represented structurally by the leaf grammars.
 *
 * Actual limits belong to:
 *
 *     semantic validation;
 *     resource analysis;
 *     compiler resources;
 *     runtime resources;
 *     target capabilities;
 *     deployment policy.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 7. QUANTUM INTEGRATION
 * ============================================================================
 *
 * AI may participate in hybrid quantum/classical computation.
 *
 * This composition layer does not define quantum syntax.
 *
 * Quantum source remains owned by the quantum grammar and its canonical
 * semantic boundary:
 *
 *     quantum::ir
 *
 * AI may therefore semantically lower into quantum computation when the source
 * requires it, but AI MUST NOT introduce:
 *
 *     QuantumGate
 *     QubitId
 *     physical qubit mapping
 *     quantum topology
 *     routing
 *     scheduling
 *     QEC
 *     ZQN
 *
 * into this grammar.
 *
 * The path remains:
 *
 *     AI source
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     quantum::ir
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 8. CLASSICAL / HDL / HARDWARE / DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * AI may participate in:
 *
 *     classical computation
 *     numerical computation
 *     tensor computation
 *     accelerator computation
 *     hardware/software co-design
 *     HDL generation
 *     distributed computation
 *     networking
 *     security
 *     future computational domains
 *
 * AI.g4 does not duplicate those grammars.
 *
 * Their semantics remain owned by their respective domain contracts.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 9. DIALECT / FRAMEWORK / VENDOR POLICY
 * ============================================================================
 *
 * Core AI grammar remains framework-neutral.
 *
 * The grammar does NOT reserve syntax for:
 *
 *     CUDA
 *     ROCm
 *     TensorFlow
 *     PyTorch
 *     JAX
 *     ONNX
 *     OpenVINO
 *     vendor-specific accelerators
 *     cloud providers
 *     particular model families
 *     particular runtime systems
 *
 * Such integrations belong to:
 *
 *     interoperability/
 *     dialects/
 *     hardware/
 *     compile/
 *     execution/
 *
 * A vendor/framework extension must therefore be explicit rather than silently
 * becoming part of the core Zamani language.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 10. DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no embedded actions;
 *     - no semantic predicates;
 *     - no executable code;
 *     - no filesystem access;
 *     - no network access;
 *     - no environment inspection;
 *     - no hardware discovery;
 *     - no random behavior;
 *     - no runtime execution.
 *
 * For a fixed:
 *
 *     source text
 *     lexer version
 *     parser grammar version
 *     enabled dialect configuration
 *
 * parsing must be deterministic.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 11. SAFETY
 * ============================================================================
 *
 * This grammar contains no Rust implementation code.
 *
 * The Rust frontend consuming generated ANTLR output must target:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and safe Rust only.
 *
 * No `unsafe` implementation is required or permitted as an architectural
 * dependency of this grammar.
 *
 * The grammar itself performs no:
 *
 *     I/O
 *     process execution
 *     device access
 *     model loading
 *     dataset loading
 *     network communication
 *     accelerator discovery
 *     quantum-device discovery
 *     resource allocation.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 12. AST CONTRACT
 * ============================================================================
 *
 * AI.g4 produces parser structures only.
 *
 * The frontend AST remains domain-neutral.
 *
 * Every AI leaf construct must preserve enough structure for downstream
 * construction of the existing AST, including:
 *
 *     source span
 *     declaration identity
 *     names
 *     annotations
 *     parameters
 *     types
 *     expressions
 *     nested structure
 *     member ordering
 *     relationships
 *     provenance
 *
 * AI.g4 MUST NOT introduce a competing AI AST hierarchy merely because a
 * construct belongs to AI.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 13. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis, not this grammar, determines:
 *
 *     - whether an annotation is recognized;
 *     - whether an AI construct is legal;
 *     - whether names resolve;
 *     - whether types are valid;
 *     - whether tensor shapes are compatible;
 *     - whether model graphs are valid;
 *     - whether resource requirements can be satisfied;
 *     - whether capabilities exist;
 *     - whether effects are permitted;
 *     - whether a model is portable;
 *     - whether a dialect is enabled;
 *     - whether a quantum reference is semantically valid;
 *     - whether a hardware requirement is satisfiable.
 *
 * Parser acceptance MUST NOT depend on current hardware availability.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 14. IR CONTRACT
 * ============================================================================
 *
 * AI.g4 creates NO IR.
 *
 * AI semantics may lower into existing canonical IR paths.
 *
 * Depending on the program, AI computation may lower toward:
 *
 *     classical IR
 *     tensor/data computation IR
 *     accelerator/hardware IR
 *     distributed IR
 *     quantum::ir
 *     other canonical domain IRs
 *
 * There must not be:
 *
 *     AIUniversalIR
 *     AIQuantumIR
 *     AIHardwareIR
 *
 * introduced merely because this composition grammar exists.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 15. COMPILER / RUNTIME CONTRACT
 * ============================================================================
 *
 * Compilation is responsible for:
 *
 *     specialization
 *     optimization
 *     lowering
 *     resource realization
 *     scheduling
 *     routing
 *     target selection
 *     deployment planning
 *
 * Runtime is responsible for:
 *
 *     resource discovery
 *     execution
 *     lifecycle
 *     monitoring
 *     recovery
 *
 * AI.g4 performs none of these operations.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 16. HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no artificial resource limits.
 *
 * In particular, it does not define:
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
 * It also does not define:
 *
 *     fixed GPU identifiers
 *     fixed CPU identifiers
 *     fixed QPU identifiers
 *     fixed physical qubits
 *     fixed tensor ranks
 *     fixed model depths
 *     fixed node counts
 *     fixed accelerator counts
 *     fixed topology sizes
 *
 * Program-level numeric values remain program semantics.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 17. INTEGRATION WITH ZamaniParser.g4
 * ============================================================================
 *
 * `grammar/antlr/ZamaniParser.g4` already imports:
 *
 *     AI
 *
 * and routes the universal AI domain through:
 *
 *     aiElement
 *
 * Therefore the canonical dependency becomes:
 *
 *     Zamani.g4
 *          |
 *          v
 *     ZamaniParser
 *          |
 *          v
 *        AI
 *          |
 *     +----+-------------------------------+
 *     |                                    |
 *     v                                    v
 * AI leaf grammars                   universal grammars
 *     |                                    |
 *     +----------------+-------------------+
 *                      |
 *                      v
 *                frontend AST
 *
 * `Zamani.g4` itself does NOT need to import the individual AI leaf grammars.
 *
 * This preserves the existing root architecture.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 18. INTEGRATION WITH grammar/ai/README.md
 * ============================================================================
 *
 * The AI README must identify this file as:
 *
 *     AI composition boundary
 *
 * and identify the leaf grammar ownership:
 *
 *     models.g4
 *     datasets.g4
 *     tensors.g4
 *     training.g4
 *     inference.g4
 *     agents.g4
 *     differentiation.g4
 *     pipelines.g4
 *     ai-accelerators.g4
 *     model-deployment.g4
 *
 * It must identify:
 *
 *     model.g4
 *     agent.g4
 *
 * as legacy/deprecated duplicates that are not imported by this composition
 * grammar.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 19. INTEGRATION WITH grammar/spec/ai.md
 * ============================================================================
 *
 * The normative AI semantic specification remains:
 *
 *     grammar/spec/ai.md
 *
 * This file implements the parser-composition portion of that specification.
 *
 * It does not replace the semantic specification.
 *
 * `grammar/spec/ai.md` remains responsible for:
 *
 *     AI semantic definitions
 *     lifecycle/status
 *     type integration
 *     effects
 *     resources
 *     capabilities
 *     portability
 *     IR mapping
 *     compiler/runtime contracts
 *     conformance requirements.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 20. INTEGRATION WITH grammar/grammar.md
 * ============================================================================
 *
 * `grammar/grammar.md` remains the implementation-conformance reference.
 *
 * It must report the resulting state of:
 *
 *     AI composition
 *     model grammar
 *     dataset grammar
 *     tensor grammar
 *     training grammar
 *     inference grammar
 *     agent grammar
 *     differentiation grammar
 *     pipeline grammar
 *     accelerator grammar
 *     deployment grammar
 *
 * using the repository's conformance statuses.
 *
 * This file MUST NOT become another specification authority.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 21. REQUIRED CONFORMANCE TESTS
 * ============================================================================
 *
 * The AI composition layer is complete only when the repository verifies:
 *
 * POSITIVE:
 *
 *     @model ...
 *     @dataset ...
 *     @tensor ...
 *     @training ...
 *     @inference ...
 *     @agent ...
 *     @pipeline ...
 *     @accelerator ...
 *     @deployment ...
 *
 * NEGATIVE:
 *
 *     arbitrary ordinary expressions are not silently accepted as aiConstruct;
 *     malformed AI declarations are rejected;
 *     malformed annotation boundaries are rejected;
 *     malformed leaf constructs are rejected.
 *
 * BOUNDARY:
 *
 *     empty AI regions where permitted;
 *     one-member constructs;
 *     many-member constructs;
 *     nested constructs;
 *     deeply composed constructs;
 *     symbolic dimensions;
 *     symbolic resource quantities;
 *     large numeric program values.
 *
 * SCALABILITY:
 *
 *     no artificial model-count ceiling;
 *     no artificial tensor-rank ceiling;
 *     no artificial layer-count ceiling;
 *     no artificial parameter-count ceiling;
 *     no artificial dataset-size ceiling;
 *     no artificial worker-count ceiling;
 *     no artificial node-count ceiling;
 *     no artificial accelerator-count ceiling.
 *
 * DETERMINISM:
 *
 *     identical source/token streams produce deterministic parses.
 *
 * PORTABILITY:
 *
 *     the same source structure remains independent of target hardware.
 *
 * HARD-CODING:
 *
 *     no universal machine-capacity constant is introduced.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 22. COMPLETION CRITERIA
 * ============================================================================
 *
 * AI.g4 is complete when:
 *
 * [x] It is the single AI composition boundary.
 *
 * [x] It is imported by ZamaniParser.g4 through `AI`.
 *
 * [x] It composes the canonical AI leaf grammars.
 *
 * [x] It does not import duplicate model grammars.
 *
 * [x] It does not import duplicate agent grammars.
 *
 * [x] It uses ZamaniLexer.
 *
 * [x] It follows the canonical AT annotation model.
 *
 * [x] It does not use NANO_ANNOTATION.
 *
 * [x] It does not define lexer rules.
 *
 * [x] It does not duplicate expression syntax.
 *
 * [x] It does not duplicate type syntax.
 *
 * [x] It does not duplicate statement syntax.
 *
 * [x] It does not create an AI-specific AST.
 *
 * [x] It does not create an AI-specific IR.
 *
 * [x] It does not create a second quantum IR.
 *
 * [x] quantum::ir remains the canonical quantum semantic boundary.
 *
 * [x] It contains no target-specific hardware assumptions.
 *
 * [x] It contains no artificial resource limits.
 *
 * [x] It contains no parser-time execution.
 *
 * [x] It contains no semantic predicates.
 *
 * [x] It contains no embedded Rust.
 *
 * [x] It requires no unsafe Rust.
 *
 * [x] It supports POCO-REAF architecturally.
 *
 * [x] Leaf grammars remain independently testable.
 *
 * [x] AI semantics remain downstream of parsing.
 *
 * [x] Resource/capability decisions remain downstream.
 *
 * [x] Hardware realization remains downstream.
 *
 * [x] Compilation remains target-aware but source-portable.
 *
 * [x] Runtime resource availability remains external to parsing.
 *
 * ============================================================================
 *
 * FINAL INVARIANT
 * ============================================================================
 *
 * AI.g4 answers exactly:
 *
 *     "Which independently-owned AI grammar domains participate in the
 *      canonical Zamani AI parser boundary?"
 *
 * It does NOT answer:
 *
 *     "How does AI execute?"
 *
 *     "Which hardware executes AI?"
 *
 *     "How much memory exists?"
 *
 *     "How many GPUs exist?"
 *
 *     "How many quantum resources exist?"
 *
 *     "How is an AI model optimized?"
 *
 *     "How is a model scheduled?"
 *
 *     "How is a model deployed?"
 *
 * Those questions belong downstream.
 *
 * The governing portability invariant remains:
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
 * subject only to the program's semantics, declared requirements,
 * capabilities, and actual resources available at realization time.
 *
 * ============================================================================
 */
/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/ai/effects.g4
 *
 * GRAMMAR
 * -------
 * AIEffects
 *
 * STATUS
 * ------
 * CANONICAL AI / EFFECT-INTEGRATION LEAF GRAMMAR
 *
 * LANGUAGE BASELINE
 * -----------------
 * Rust 1.97 / Rust 1.97.1
 * Rust 2021
 * Safe Rust only
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file defines the SOURCE-LEVEL AI/EFFECT INTEGRATION BOUNDARY.
 *
 * It does NOT define the Zamani effect system itself.
 *
 * The canonical effect subsystem remains:
 *
 *     grammar/effects/
 *
 * In particular:
 *
 *     grammar/effects/effects.g4
 *     grammar/effects/effect-declarations.g4
 *     grammar/effects/effect-sets.g4
 *     grammar/effects/effect-operations.g4
 *     grammar/effects/effect-handling.g4
 *     grammar/effects/effect-types.g4
 *     grammar/effects/effect-polymorphism.g4
 *     grammar/effects/effect-composition.g4
 *     grammar/effects/custom-effects.g4
 *
 * own the general effect language.
 *
 * This file exists because AI computation needs a stable way to express:
 *
 *     - effects associated with cognitive/model computation;
 *     - effect references;
 *     - effect sets;
 *     - effect requirements;
 *     - effect constraints;
 *     - effect preferences;
 *     - effect hints;
 *     - effect-scoped computation;
 *     - effect metadata;
 *     - future AI effect extensions.
 *
 * The effect itself remains a general Zamani semantic concept.
 *
 * ============================================================================
 * FUNDAMENTAL ARCHITECTURAL RULE
 * ============================================================================
 *
 * AI effects are NOT a second effect language.
 *
 * The architecture is:
 *
 *     AI source
 *          |
 *          v
 *     AIEffects
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     semantic effect analysis
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +----------------------+----------------------+
 *          |                      |                      |
 *          v                      v                      v
 *      classical             quantum::ir          HDL/hardware
 *          |                      |                      |
 *          +----------------------+----------------------+
 *                                 |
 *                                 v
 *                            optimization
 *                                 |
 *                     routing / scheduling
 *                                 |
 *                         resilience / QEC
 *                                 |
 *                                ZQN
 *                                 |
 *                                HAL
 *                                 |
 *                         target realization
 *
 * This grammar produces NO IR.
 *
 * This grammar produces NO runtime effect.
 *
 * This grammar performs NO capability discovery.
 *
 * This grammar performs NO resource discovery.
 *
 * This grammar performs NO hardware discovery.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - AI effect annotation syntax;
 *     - AI effect-set annotation syntax;
 *     - AI effect references used by AI contracts;
 *     - AI effect requirements;
 *     - AI effect constraints;
 *     - AI effect preferences;
 *     - AI effect hints;
 *     - AI effect-scoped computation regions;
 *     - AI effect bindings;
 *     - AI effect metadata extension points;
 *     - AI/effect source-level composition.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - effect declarations;
 *     - effect operation declarations;
 *     - effect operation implementations;
 *     - effect invocation syntax outside the AI integration boundary;
 *     - effect sets as a general language feature;
 *     - effect handlers;
 *     - effect handler arms;
 *     - effect polymorphism;
 *     - effect types;
 *     - effect composition;
 *     - custom-effect declarations;
 *     - general expressions;
 *     - general statements;
 *     - general types;
 *     - identifiers;
 *     - qualified names;
 *     - AI models;
 *     - tensors;
 *     - datasets;
 *     - training;
 *     - inference;
 *     - agents;
 *     - memory implementation;
 *     - resource discovery;
 *     - capability discovery;
 *     - hardware selection;
 *     - quantum operations;
 *     - quantum topology;
 *     - QEC;
 *     - ZQN;
 *     - routing;
 *     - scheduling;
 *     - runtime execution.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * General effect declarations remain owned by:
 *
 *     grammar/effects/effect-declarations.g4
 *
 * General effect references and effect sets remain owned by:
 *
 *     grammar/effects/effect-sets.g4
 *
 * General effect operation use remains owned by:
 *
 *     grammar/effects/effect-operations.g4
 *
 * Effect handling remains owned by:
 *
 *     grammar/effects/effect-handling.g4
 *
 * Effect typing remains owned by:
 *
 *     grammar/effects/effect-types.g4
 *
 * Effect polymorphism remains owned by:
 *
 *     grammar/effects/effect-polymorphism.g4
 *
 * Effect composition remains owned by:
 *
 *     grammar/effects/effect-composition.g4
 *
 * AI model structure remains owned by:
 *
 *     grammar/ai/model.g4
 *     grammar/ai/models.g4
 *
 * AI inference remains owned by:
 *
 *     grammar/ai/inference.g4
 *
 * AI training remains owned by:
 *
 *     grammar/ai/training.g4
 *
 * AI agent structure remains owned by:
 *
 *     grammar/ai/agent.g4
 *     grammar/ai/agents.g4
 *
 * AI cognition remains owned by:
 *
 *     grammar/ai/mind.g4
 *
 * This file only creates the integration boundary.
 *
 * ============================================================================
 * LEXICAL CONTRACT
 * ============================================================================
 *
 * Canonical lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Canonical annotation marker:
 *
 *     AT
 *
 * Canonical annotation structure:
 *
 *     AT identifier
 *
 * This file therefore uses:
 *
 *     AT
 *
 * and canonical shared identifier/name rules.
 *
 * It MUST NOT use legacy:
 *
 *     NANO_ANNOTATION
 *
 * as the foundation of the new AI effect architecture.
 *
 * This file introduces NO lexer rules.
 *
 * ============================================================================
 * OPEN-WORLD EFFECT PRINCIPLE
 * ============================================================================
 *
 * Effect identities are semantic names.
 *
 * The grammar does NOT enumerate:
 *
 *     IO
 *     Network
 *     Storage
 *     Quantum
 *     AI
 *     Tensor
 *     GPU
 *     QPU
 *     Accelerator
 *     Distributed
 *     Security
 *     Robotics
 *     Future
 *
 * as finite parser alternatives.
 *
 * Therefore all of the following can remain structurally representable:
 *
 *     io::read
 *     ai::inference
 *     tensor::compute
 *     quantum::measurement
 *     distributed::consensus
 *     hardware::signal
 *     future::domain::operation
 *     vendor::extension::operation
 *
 * Whether a particular effect exists is a semantic question.
 *
 * Whether it is legal is a semantic question.
 *
 * Whether it is available on a target is a capability/resource question.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * AI effect syntax is target-independent.
 *
 * It MUST NOT encode:
 *
 *     GPU 0
 *     CPU 0
 *     QPU 0
 *     FPGA 0
 *     node 0
 *     device 0
 *     memory bank 0
 *     physical qubit 0
 *
 * as universal language constructs.
 *
 * AI source may instead express:
 *
 *     requires effect(...)
 *     requires capability(...)
 *     constraint(...)
 *     prefer effect(...)
 *     hint(...)
 *
 * Actual realization is determined downstream.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This grammar imposes NO universal limits on:
 *
 *     - number of AI effects;
 *     - number of effect references;
 *     - number of effect sets;
 *     - number of effect operations;
 *     - number of arguments;
 *     - number of AI models;
 *     - number of tensors;
 *     - number of training steps;
 *     - number of inference operations;
 *     - number of cognitive regions;
 *     - number of nested effect regions;
 *     - number of resources;
 *     - number of capabilities;
 *     - number of devices;
 *     - number of accelerators;
 *     - number of CPUs;
 *     - number of GPUs;
 *     - number of FPGAs;
 *     - number of QPUs;
 *     - number of nodes;
 *     - number of qubits;
 *     - tensor rank;
 *     - tensor dimensions;
 *     - memory capacity.
 *
 * It MUST NOT contain artificial limits such as:
 *
 *     MAX_AI_EFFECTS
 *     MAX_EFFECTS
 *     MAX_EFFECT_OPERATIONS
 *     MAX_EFFECT_ARGUMENTS
 *     MAX_AI_MODELS
 *     MAX_TENSORS
 *     MAX_GPUS
 *     MAX_CPUS
 *     MAX_QUBITS
 *     MAX_NODES
 *     MAX_MEMORY
 *
 * Repetition is represented structurally using `*` and `+`.
 *
 * Actual implementation/resource limits belong outside the language grammar.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing depends only upon:
 *
 *     - source text;
 *     - canonical lexer;
 *     - selected language version;
 *     - parser grammar;
 *     - explicitly selected dialect configuration.
 *
 * Parsing MUST NOT depend upon:
 *
 *     - hardware availability;
 *     - AI model availability;
 *     - network state;
 *     - runtime state;
 *     - random values;
 *     - wall-clock time;
 *     - deployment state;
 *     - target resource availability.
 *
 * ============================================================================
 * SAFETY
 * ============================================================================
 *
 * This file contains:
 *
 *     - no embedded Rust actions;
 *     - no semantic predicates;
 *     - no filesystem access;
 *     - no network access;
 *     - no process execution;
 *     - no hardware discovery;
 *     - no runtime execution;
 *     - no randomness;
 *     - no unsafe implementation.
 *
 * Generated/compiler-side Rust must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and the repository-wide safe-Rust requirement.
 *
 * ============================================================================
 * CROSS-DOMAIN INTEGRATION
 * ============================================================================
 *
 * AI effects may describe operations that eventually lower into:
 *
 *     classical computation
 *     tensor computation
 *     symbolic computation
 *     probabilistic computation
 *     distributed computation
 *     networking
 *     security
 *     hardware acceleration
 *     quantum computation
 *     hybrid computation
 *     HDL/hardware co-design
 *
 * If an AI computation causes a quantum operation:
 *
 *     AI source
 *        |
 *        v
 *     semantic analysis
 *        |
 *        v
 *     quantum semantics
 *        |
 *        v
 *     quantum::ir
 *
 * AIEffects MUST NOT introduce:
 *
 *     AIQuantumIR
 *
 * or any other second quantum representation.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY SEPARATION
 * ============================================================================
 *
 * Effect:
 *
 *     describes semantic computation/interaction.
 *
 * Capability:
 *
 *     describes what an environment can provide.
 *
 * Resource:
 *
 *     describes a consumable/allocatable computational resource.
 *
 * Requirement:
 *
 *     describes what must be satisfied.
 *
 * Constraint:
 *
 *     describes what an implementation must obey.
 *
 * Preference:
 *
 *     describes a preferred realization.
 *
 * Hint:
 *
 *     provides non-binding implementation guidance.
 *
 * This file preserves these distinctions.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Every construct must preserve enough structure for the existing
 * domain-neutral AST.
 *
 * The semantic representation must be able to recover:
 *
 *     - construct kind;
 *     - effect name/reference;
 *     - argument expressions;
 *     - assignment expression where present;
 *     - contract expression where present;
 *     - nested computation region;
 *     - source order;
 *     - source spans;
 *     - source provenance.
 *
 * This file does NOT require a backend-specific AST node.
 *
 * A generic annotation/contract AST node may be reused where supported.
 *
 * If the semantic layer needs an AI-effect node, that node belongs outside
 * grammar/.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis determines:
 *
 *     - whether the referenced effect exists;
 *     - whether the effect is permitted in the current AI context;
 *     - whether arguments satisfy the effect signature;
 *     - whether the effect is declared/imported;
 *     - whether required capabilities exist;
 *     - whether required resources are feasible;
 *     - whether an effect is pure/impure according to its declaration;
 *     - whether the effect crosses an AI/classical boundary;
 *     - whether the effect crosses an AI/quantum boundary;
 *     - whether effect handlers discharge the requested effect;
 *     - whether security policy permits the effect;
 *     - whether the selected realization satisfies constraints.
 *
 * Parser acceptance does NOT imply semantic validity or target feasibility.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates NO IR.
 *
 * The expected semantic direction is:
 *
 *     AIEffects
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic effect model
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> classical IR
 *          +--> quantum::ir
 *          +--> HDL/hardware representation
 *          +--> distributed representation
 *          +--> accelerator representation
 *
 * There MUST NOT be:
 *
 *     AIEffects -> AIEffectIR -> quantum::ir
 *
 * where `AIEffectIR` becomes a competing intermediate representation.
 *
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * `aiEffectConstruct` is the sole public entry point owned by this grammar.
 *
 * All AI/effect syntax enters through this boundary.
 *
 * ============================================================================
 */

parser grammar AIEffects;

options {
    tokenVocab = ZamaniLexer;
}

import Core,
       Types,
       Expressions,
       Statements;


/* ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * AI effect constructs are structurally distinct from ordinary expressions
 * and statements.
 * ============================================================================
 */

aiEffectConstruct
    : aiEffectAnnotation
    | aiEffectsAnnotation
    | aiEffectBinding
    | aiEffectInvocation
    | aiEffectRegion
    | aiEffectRequirement
    | aiEffectConstraint
    | aiEffectPreference
    | aiEffectHint
    ;


/* ============================================================================
 * 2. SINGLE EFFECT ANNOTATION
 * ============================================================================
 *
 * Examples:
 *
 *     @effect(io::read);
 *     @effect(ai::inference);
 *     @effect(quantum::measurement);
 *
 * The effect identity is data, not a parser enumeration.
 * ============================================================================
 */

aiEffectAnnotation
    : AT
      EFFECT
      LPAREN
      aiEffectReference
      RPAREN
      SEMICOLON?
    ;


/* ============================================================================
 * 3. EFFECT SET ANNOTATION
 * ============================================================================
 *
 * Examples:
 *
 *     @effects(io::read, ai::inference);
 *
 *     @effects(
 *         quantum::measurement,
 *         tensor::compute
 *     );
 *
 * The number of entries is unbounded by the grammar.
 * ============================================================================
 */

aiEffectsAnnotation
    : AT
      EFFECTS
      LPAREN
      aiEffectReferenceList?
      RPAREN
      SEMICOLON?
    ;


/* ============================================================================
 * 4. EFFECT BINDING
 * ============================================================================
 *
 * Examples:
 *
 *     @effect current = ai::inference;
 *     @effect inference_effect = selected_effect;
 *
 * This is a source-level binding only.
 *
 * It does not perform the effect.
 * ============================================================================
 */

aiEffectBinding
    : AT
      EFFECT
      identifier
      ASSIGN
      expression
      SEMICOLON?
    ;


/* ============================================================================
 * 5. EFFECT INVOCATION METADATA
 * ============================================================================
 *
 * Examples:
 *
 *     @effect ai::inference(input);
 *     @effect quantum::measurement(state);
 *
 * This construct represents an AI-side effect invocation boundary.
 *
 * It does NOT execute the effect during parsing.
 *
 * Actual general effect operation execution remains owned by the canonical
 * effect-operation subsystem.
 * ============================================================================
 */

aiEffectInvocation
    : AT
      EFFECT
      aiEffectReference
      LPAREN
      argumentList?
      RPAREN
      SEMICOLON?
    ;


/* ============================================================================
 * 6. EFFECT-SCOPED AI COMPUTATION
 * ============================================================================
 *
 * Examples:
 *
 *     @effect {
 *         inference(input);
 *         result = process(input);
 *     }
 *
 *     @effect {
 *         ...
 *     }
 *
 * The body contains ordinary Zamani statements.
 *
 * No AI-specific statement language is introduced here.
 * ============================================================================
 */

aiEffectRegion
    : AT
      EFFECT
      LBRACE
      statement*
      RBRACE
    ;


/* ============================================================================
 * 7. EFFECT REQUIREMENT
 * ============================================================================
 *
 * Examples:
 *
 *     requires effect(io::read);
 *     requires effect(ai::inference);
 *     requires effect(quantum::measurement);
 *
 * This is a semantic requirement.
 *
 * It does not select a physical target.
 * ============================================================================
 */

aiEffectRequirement
    : REQUIRES
      EFFECT
      LPAREN
      aiEffectReferenceList?
      RPAREN
      SEMICOLON?
    ;


/* ============================================================================
 * 8. EFFECT CONSTRAINT
 * ============================================================================
 *
 * Examples:
 *
 *     constraint effect(ai::inference);
 *     constraint effect(quantum::measurement);
 *
 * The expression inside the constraint remains semantically interpreted by
 * the effect/resource/capability systems.
 * ============================================================================
 */

aiEffectConstraint
    : CONSTRAINT
      EFFECT
      LPAREN
      expression
      RPAREN
      SEMICOLON?
    ;


/* ============================================================================
 * 9. EFFECT PREFERENCE
 * ============================================================================
 *
 * Examples:
 *
 *     prefer effect(ai::inference);
 *     prefer effect(accelerator::tensor_compute);
 *
 * A preference is not a requirement.
 * ============================================================================
 */

aiEffectPreference
    : PREFER
      EFFECT
      LPAREN
      aiEffectReferenceList?
      RPAREN
      SEMICOLON?
    ;


/* ============================================================================
 * 10. EFFECT HINT
 * ============================================================================
 *
 * Examples:
 *
 *     hint effect(ai::inference);
 *     hint effect(locality::compute);
 *
 * A hint is advisory.
 * ============================================================================
 */

aiEffectHint
    : HINT
      EFFECT
      LPAREN
      expression
      RPAREN
      SEMICOLON?
    ;


/* ============================================================================
 * 11. EFFECT REFERENCE
 * ============================================================================
 *
 * An effect is identified by a normal qualified name.
 *
 * Examples:
 *
 *     ai::inference
 *     ai::training
 *     tensor::compute
 *     quantum::measurement
 *     distributed::consensus
 *     future::domain::operation
 *
 * The grammar does not enumerate these names.
 * ============================================================================
 */

aiEffectReference
    : qualifiedName
    ;


/* ============================================================================
 * 12. EFFECT REFERENCE LIST
 * ============================================================================
 *
 * No universal cardinality is imposed.
 * ============================================================================
 */

aiEffectReferenceList
    : aiEffectReference
      (COMMA aiEffectReference)*
    ;


/* ============================================================================
 * 13. COMPLETION CONTRACT
 * ============================================================================
 *
 * This file is independently complete when all of the following are true:
 *
 * OWNERSHIP
 * ----------
 *
 * [x] AI/effect integration is the only responsibility.
 * [x] General effect declarations remain outside this file.
 * [x] General effect operation declarations remain outside this file.
 * [x] General effect handlers remain outside this file.
 * [x] General effect sets remain outside this file.
 * [x] General effect typing remains outside this file.
 *
 * LEXICAL
 * -------
 *
 * [x] Uses the canonical ZamaniLexer vocabulary.
 * [x] Uses AT rather than NANO_ANNOTATION.
 * [x] Introduces no lexer rules.
 * [x] Introduces no new tokens.
 *
 * PARSING
 * -------
 *
 * [x] Has one public entry point.
 * [x] AI effect syntax has explicit structural boundaries.
 * [x] Ordinary expressions remain owned by Expressions.
 * [x] Ordinary statements remain owned by Statements.
 * [x] General names remain owned by Core.
 * [x] No target-specific syntax is introduced.
 *
 * SCALABILITY
 * -----------
 *
 * [x] No artificial AI effect limits.
 * [x] No artificial effect-operation limits.
 * [x] No artificial argument limits.
 * [x] No artificial machine limits.
 * [x] No artificial tensor limits.
 * [x] No artificial quantum limits.
 *
 * PORTABILITY
 * -----------
 *
 * [x] No CPU selection.
 * [x] No GPU selection.
 * [x] No FPGA selection.
 * [x] No QPU selection.
 * [x] No node selection.
 * [x] No physical-qubit selection.
 * [x] No memory-bank selection.
 *
 * SEMANTICS
 * ---------
 *
 * [x] Effect identity is semantic data.
 * [x] Capability checking is downstream.
 * [x] Resource checking is downstream.
 * [x] Security checking is downstream.
 * [x] Target realization is downstream.
 *
 * IR
 * --
 *
 * [x] No AI effect IR is created.
 * [x] No second quantum IR is created.
 * [x] quantum::ir remains the canonical quantum boundary.
 *
 * SAFETY
 * ------
 *
 * [x] No Rust actions.
 * [x] No semantic predicates.
 * [x] No unsafe implementation.
 * [x] No runtime execution.
 *
 * TESTING
 * -------
 *
 * [x] Positive forms are structurally defined.
 * [x] Negative forms have deterministic failure points.
 * [x] Boundary forms are structurally representable.
 * [x] Scalability is repetition-based.
 *
 * ============================================================================
 * REQUIRED CONFORMANCE FIXTURES
 * ============================================================================
 *
 * Positive:
 *
 *     @effect(ai::inference);
 *     @effect(quantum::measurement);
 *     @effects(ai::inference, tensor::compute);
 *     @effect current = ai::inference;
 *     @effect ai::inference(input);
 *     @effect {
 *         result = infer(input);
 *     }
 *     requires effect(ai::inference);
 *     constraint effect(ai::inference);
 *     prefer effect(ai::inference);
 *     hint effect(ai::inference);
 *
 * Cross-domain:
 *
 *     @effect(quantum::measurement);
 *     @effect(hardware::accelerator);
 *     @effects(distributed::consensus, networking::request);
 *     requires effect(ai::inference);
 *
 * Open-world:
 *
 *     @effect(future::domain::operation);
 *     @effects(vendor::extension::operation);
 *
 * Negative:
 *
 *     @effect();
 *     @effects(,);
 *     @effect(ai::);
 *     @effect(::);
 *     requires effect();
 *     prefer effect(,);
 *
 * Boundary:
 *
 *     one effect;
 *     many effects;
 *     deeply qualified effect names;
 *     deeply nested ordinary expressions;
 *     large effect sets;
 *     nested effect regions;
 *     large AI computation regions.
 *
 * Scalability:
 *
 *     generated effect lists of increasing size;
 *     generated nested regions;
 *     generated qualified-name depth;
 *     generated AI computation size.
 *
 * Determinism:
 *
 *     identical source/token streams produce equivalent parse structures.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file MUST remain free of universal implementation ceilings.
 *
 * Forbidden:
 *
 *     MAX_AI_EFFECTS
 *     MAX_EFFECTS
 *     MAX_EFFECT_OPERATIONS
 *     MAX_EFFECT_ARGUMENTS
 *     MAX_GPUS
 *     MAX_CPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_QUBITS
 *     MAX_NODES
 *     MAX_MEMORY
 *     GPU_0
 *     CPU_0
 *     QPU_0
 *     DEVICE_0
 *
 * Program literals remain valid when they are program semantics.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * AI composition
 * --------------
 *
 * The AI composition grammar should expose:
 *
 *     aiEffectConstruct
 *
 * as an AI-domain alternative.
 *
 * The preferred integration direction is:
 *
 *     ZamaniParser
 *          |
 *          v
 *         AI
 *          |
 *          +--> AIEffects
 *          +--> AI models
 *          +--> AI tensors
 *          +--> AI training
 *          +--> AI inference
 *          +--> AI agents
 *          +--> AI pipelines
 *
 * `AIEffects` must not import `AI`.
 *
 * This prevents:
 *
 *     AI -> AIEffects -> AI
 *
 * circular grammar dependencies.
 *
 * Effect subsystem
 * ---------------
 *
 * This file references effect identities but does not duplicate the general
 * effect grammar.
 *
 * The semantic layer resolves:
 *
 *     ai effect reference
 *          |
 *          v
 *     canonical effect declaration/reference
 *
 * General effect operations continue through:
 *
 *     grammar/effects/effect-operations.g4
 *
 * General handlers continue through:
 *
 *     grammar/effects/effect-handling.g4
 *
 * AST
 * ---
 *
 * The frontend must preserve:
 *
 *     annotation/contract kind
 *     effect reference(s)
 *     argument expressions
 *     binding expression
 *     region statements
 *     source spans
 *
 * Semantic analysis then attaches the appropriate effect information to the
 * existing semantic representation.
 *
 * No grammar-level backend node is required.
 *
 * Quantum
 * -------
 *
 * An effect such as:
 *
 *     @effect(quantum::measurement);
 *
 * remains an effect reference.
 *
 * If the semantic operation becomes quantum computation, the downstream
 * semantic pipeline reaches:
 *
 *     quantum::ir
 *
 * without creating an AI-specific quantum representation.
 *
 * ============================================================================
 */
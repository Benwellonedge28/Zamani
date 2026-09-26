/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/ai/mind.g4
 *
 * GRAMMAR
 * -------
 * AIMind
 *
 * STATUS
 * ------
 * CANONICAL AI MIND / COGNITIVE-COMPUTATION LEAF GRAMMAR
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
 * This file defines the SOURCE-LEVEL STRUCTURE of a Zamani AI "mind".
 *
 * A mind is a framework-neutral cognitive computation boundary.
 *
 * It may express:
 *
 *     - cognitive state;
 *     - knowledge;
 *     - beliefs;
 *     - observations;
 *     - concepts;
 *     - memory references;
 *     - goals;
 *     - objectives;
 *     - reasoning;
 *     - inference intent;
 *     - planning intent;
 *     - decision intent;
 *     - learning intent;
 *     - reflection;
 *     - context;
 *     - attention;
 *     - perception;
 *     - prediction;
 *     - uncertainty;
 *     - hypotheses;
 *     - evidence;
 *     - explanations;
 *     - questions;
 *     - answers;
 *     - cognition pipelines;
 *     - cognitive resources;
 *     - capabilities;
 *     - constraints;
 *     - preferences;
 *     - hints;
 *     - provenance;
 *     - temporal/cross-timeline references;
 *     - classical computation;
 *     - quantum computation through ordinary semantic references;
 *     - distributed computation;
 *     - hardware/accelerator requirements through universal contracts;
 *     - future cognitive constructs through extensible annotations.
 *
 * The mind grammar describes COMPUTATIONAL INTENT AND STRUCTURE.
 *
 * It does NOT define:
 *
 *     - consciousness;
 *     - a particular AI framework;
 *     - a particular model architecture;
 *     - a particular neural-network implementation;
 *     - a particular reasoning engine;
 *     - a particular database;
 *     - a particular memory implementation;
 *     - a particular agent runtime;
 *     - a particular accelerator;
 *     - a particular CPU/GPU/FPGA/ASIC/QPU;
 *     - physical device selection;
 *     - hardware topology;
 *     - resource discovery;
 *     - scheduling;
 *     - placement;
 *     - networking implementation;
 *     - security enforcement;
 *     - training algorithms;
 *     - inference algorithms;
 *     - automatic differentiation;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime execution.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     canonical Zamani lexer
 *          |
 *          v
 *     canonical Zamani parser
 *          |
 *          v
 *     AIMind
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     structural validation
 *          |
 *          v
 *     semantic analysis
 *          |
 *     +----+----------+-----------+-------------+
 *     |               |           |             |
 *     v               v           v             v
 *   AI semantics   resources   capabilities   effects
 *     |               |           |             |
 *     +---------------+-----------+-------------+
 *                     |
 *                     v
 *             canonical semantic model
 *                     |
 *        +------------+-------------+
 *        |            |             |
 *        v            v             v
 *    classical    quantum::ir   HDL/hardware
 *        |            |             |
 *        +------------+-------------+
 *                     |
 *                     v
 *                 canonical IR
 *                     |
 *              optimization
 *                     |
 *        routing / scheduling / resilience
 *                     |
 *                 QEC / ZQN
 *                     |
 *                    HAL
 *                     |
 *              target realization
 *
 * AIMind MUST NOT construct IR.
 *
 * AIMind MUST NOT define a second AI IR.
 *
 * AIMind MUST NOT define a second quantum IR.
 *
 * `quantum::ir` remains the canonical quantum semantic boundary.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - the mind parser-domain entry point;
 *     - mind declaration structure;
 *     - mind binding structure;
 *     - mind invocation structure;
 *     - mind computation regions;
 *     - mind-local cognitive members;
 *     - cognitive-state syntax;
 *     - knowledge/belief syntax;
 *     - observation/evidence syntax;
 *     - goal/objective syntax;
 *     - reasoning/inference intent syntax;
 *     - planning/decision intent syntax;
 *     - learning/reflection intent syntax;
 *     - context/attention/perception intent syntax;
 *     - memory references;
 *     - provenance references;
 *     - cognitive resource/capability boundaries;
 *     - mind-local extension points.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical definitions;
 *     - identifier syntax;
 *     - qualified-name syntax;
 *     - general expression precedence;
 *     - general types;
 *     - general statements;
 *     - model declarations;
 *     - tensor declarations;
 *     - datasets;
 *     - training implementation;
 *     - inference implementation;
 *     - differentiation;
 *     - agent execution;
 *     - external tool execution;
 *     - memory implementation;
 *     - networking implementation;
 *     - distributed scheduling;
 *     - hardware discovery;
 *     - quantum operations;
 *     - quantum topology;
 *     - routing;
 *     - scheduling;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - compiler implementation;
 *     - runtime implementation.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * General types are owned by:
 *
 *     Types
 *
 * General expressions are owned by:
 *
 *     Expressions
 *
 * General statements are owned by:
 *
 *     Statements
 *
 * Model structure is owned by:
 *
 *     AIModels / models.g4
 *
 * Tensor structure is owned by:
 *
 *     AITensors / tensors.g4
 *
 * Dataset structure is owned by:
 *
 *     AIDatasets / datasets.g4
 *
 * Training structure is owned by:
 *
 *     AITraining / training.g4
 *
 * Inference structure is owned by:
 *
 *     Inference / inference.g4
 *
 * Agent structure is owned by:
 *
 *     AIAgent / agent.g4
 *
 * Memory semantics are owned by:
 *
 *     grammar/memory/
 *
 * Resource semantics are owned by:
 *
 *     grammar/resources/
 *
 * Hardware intent is owned by:
 *
 *     grammar/hardware/
 *
 * Distributed semantics are owned by:
 *
 *     grammar/distributed/
 *
 * Quantum semantics are owned by:
 *
 *     grammar/quantum/
 *
 * This grammar references those domains through source-level names,
 * expressions, types, statements, annotations, and semantic contracts.
 *
 * It does not duplicate them.
 *
 * ============================================================================
 * LEXICAL CONTRACT
 * ============================================================================
 *
 * The canonical lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * The canonical annotation marker is:
 *
 *     AT
 *
 * representing:
 *
 *     @
 *
 * The annotation name is parsed as:
 *
 *     identifier
 *
 * Therefore this grammar MUST use:
 *
 *     AT identifier
 *
 * and MUST NOT use:
 *
 *     NANO_ANNOTATION
 *
 * `NANO_ANNOTATION` belongs to legacy/inconsistent grammar material and is
 * intentionally not used here.
 *
 * This grammar introduces NO lexer rules.
 *
 * AI concepts remain semantic vocabulary rather than global lexer keywords.
 *
 * ============================================================================
 * OPEN-WORLD PRINCIPLE
 * ============================================================================
 *
 * Mind roles are intentionally represented structurally.
 *
 * The grammar does not enumerate a finite universe of:
 *
 *     thoughts
 *     memories
 *     beliefs
 *     reasoning algorithms
 *     planning algorithms
 *     learning algorithms
 *     cognitive architectures
 *     model types
 *     inference engines
 *     attention mechanisms
 *     perception systems
 *     decision strategies
 *
 * Semantic registries and dialects may define their meanings.
 *
 * The grammar remains stable as new cognitive methods appear.
 *
 * ============================================================================
 * POCO-REAF / SCALABILITY
 * ============================================================================
 *
 * This grammar imposes NO universal limits on:
 *
 *     minds
 *     nested minds
 *     cognitive members
 *     concepts
 *     beliefs
 *     observations
 *     goals
 *     objectives
 *     hypotheses
 *     evidence items
 *     memories
 *     reasoning steps
 *     plans
 *     decisions
 *     contexts
 *     states
 *     models
 *     tensors
 *     datasets
 *     dimensions
 *     agents
 *     distributed workers
 *     devices
 *     accelerators
 *     CPUs
 *     GPUs
 *     FPGAs
 *     QPUs
 *     nodes
 *     timelines
 *     resources
 *     capabilities
 *
 * It MUST NOT contain:
 *
 *     MAX_MINDS
 *     MAX_THOUGHTS
 *     MAX_MEMORIES
 *     MAX_BELIEFS
 *     MAX_GOALS
 *     MAX_REASONING_STEPS
 *     MAX_CONTEXTS
 *     MAX_MODELS
 *     MAX_AGENTS
 *     MAX_DEVICES
 *     MAX_GPUS
 *     MAX_CPUS
 *     MAX_QUBITS
 *     MAX_NODES
 *     MAX_MEMORY
 *
 * or equivalent universal ceilings.
 *
 * Repetition is structural:
 *
 *     *
 *     +
 *
 * where appropriate.
 *
 * Practical implementation/resource limits are not language semantics.
 *
 * ============================================================================
 * PORTABILITY
 * ============================================================================
 *
 * A mind describes portable computation.
 *
 * It MUST NOT select:
 *
 *     CPU 0
 *     GPU 0
 *     QPU 0
 *     physical qubit 0
 *     accelerator 1
 *     node 7
 *     memory bank 2
 *
 * unless such information is explicitly part of a downstream target-specific
 * realization and is not interpreted as a universal source-language limit.
 *
 * Portable source should instead express:
 *
 *     requires capability("reasoning")
 *     requires capability("tensor.compute")
 *     requires capability("quantum.measurement")
 *     requires memory >= required_memory
 *     prefers capability("accelerator.compute")
 *
 * The compiler, scheduler, resource manager, HAL and runtime determine the
 * actual realization.
 *
 * ============================================================================
 * COGNITION / AGENT SEPARATION
 * ============================================================================
 *
 * A mind is NOT an agent.
 *
 * AIMind owns:
 *
 *     cognition
 *     knowledge
 *     beliefs
 *     reasoning
 *     inference intent
 *     planning intent
 *     decision intent
 *     reflection
 *     internal state
 *
 * AIAgent owns:
 *
 *     external action
 *     tool interaction
 *     delegation
 *     agent coordination
 *     external messaging
 *     environment interaction
 *
 * A mind MAY be referenced by an agent.
 *
 * An agent MAY consume the result of a mind.
 *
 * This grammar does not make a mind an autonomous runtime actor.
 *
 * ============================================================================
 * MEMORY SEPARATION
 * ============================================================================
 *
 * A mind may reference memory.
 *
 * It does not implement memory.
 *
 * Memory operations are represented as source-level intent and resolved through
 * the canonical memory subsystem.
 *
 * This prevents:
 *
 *     AIMind -> custom memory runtime
 *
 * from becoming a second memory architecture.
 *
 * ============================================================================
 * MODEL SEPARATION
 * ============================================================================
 *
 * A mind may reference:
 *
 *     model
 *     neural model
 *     symbolic model
 *     probabilistic model
 *     quantum-derived model
 *     hybrid model
 *
 * but does not define model internals.
 *
 * Model declarations remain owned by models.g4.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * A mind may consume quantum-derived values and may request capabilities such
 * as:
 *
 *     quantum.measurement
 *     quantum.mid_circuit_measurement
 *     quantum.state_preparation
 *
 * The mind grammar MUST NOT define:
 *
 *     qubit
 *     gate
 *     circuit
 *     physical topology
 *     routing
 *     QEC
 *     ZQN
 *
 * Quantum operations remain owned by grammar/quantum/ and lower through:
 *
 *     quantum::ir
 *
 * ============================================================================
 * CLASSICAL / HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * A mind may contain ordinary Zamani statements and expressions.
 *
 * Therefore cognitive computation can compose with:
 *
 *     classical computation
 *     tensors
 *     distributed computation
 *     HDL intent
 *     hardware capability requirements
 *     networking
 *     security
 *     quantum computation
 *
 * No domain-specific copy of the general expression or statement language is
 * introduced here.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend AST must preserve sufficient information for semantic analysis.
 *
 * At minimum, a mind construct must preserve:
 *
 *     - source span;
 *     - annotation name;
 *     - mind name where present;
 *     - optional type;
 *     - optional initializer;
 *     - member order;
 *     - member source spans;
 *     - expressions;
 *     - type expressions;
 *     - referenced names;
 *     - contract expressions;
 *     - nested regions;
 *     - source provenance.
 *
 * The grammar MUST NOT require a new AST root solely because this file exists.
 *
 * If the frontend already has a generic declaration/annotation node, mind
 * constructs should lower into that existing representation.
 *
 * If a semantic cognitive node is required, it belongs in the semantic layer,
 * not in grammar/.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis determines:
 *
 *     - whether an annotation represents a mind construct;
 *     - whether a referenced model exists;
 *     - whether memory references are valid;
 *     * whether knowledge/belief values are type-correct;
 *     - whether reasoning inputs are compatible;
 *     - whether goals are well-formed;
 *     - whether decisions are permitted;
 *     - whether capabilities are available;
 *     - whether resources satisfy requirements;
 *     - whether effects are legal;
 *     - whether security policies permit requested operations;
 *     - whether cross-domain references are valid;
 *     - whether quantum values can cross the AI boundary;
 *     - whether portability constraints are satisfied.
 *
 * Parser acceptance is NOT feasibility.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar produces NO IR.
 *
 * A mind may lower to:
 *
 *     classical computation
 *     tensor computation
 *     symbolic computation
 *     probabilistic computation
 *     distributed computation
 *     quantum computation
 *     hybrid computation
 *     accelerator computation
 *
 * according to semantic analysis.
 *
 * If quantum computation is involved:
 *
 *     mind
 *       |
 *       v
 *     semantic analysis
 *       |
 *       v
 *     quantum semantics
 *       |
 *       v
 *     quantum::ir
 *
 * There MUST NOT be:
 *
 *     mind -> MindIR -> quantum::ir
 *
 * where MindIR becomes a competing semantic architecture.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     source
 *     lexer
 *     grammar
 *     parser configuration
 *
 * It MUST NOT depend on:
 *
 *     hardware discovery
 *     model availability
 *     network state
 *     runtime state
 *     random numbers
 *     resource availability
 *     deployment state
 *
 * ============================================================================
 * SAFETY
 * ============================================================================
 *
 * This grammar contains:
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
 * The implementation consuming this grammar remains:
 *
 *     Rust 2021
 *     Rust 1.97 / Rust 1.97.1
 *     safe Rust only
 *
 * ============================================================================
 * DEPENDENCIES
 * ============================================================================
 *
 * Canonical lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Canonical types:
 *
 *     grammar/types/types.g4
 *
 * Canonical expressions:
 *
 *     grammar/expressions/expressions.g4
 *
 * Canonical statements:
 *
 *     grammar/statements/statements.g4
 *
 * The leaf grammar therefore imports:
 *
 *     Types
 *     Expressions
 *     Statements
 *
 * It does not import model/training/inference/agent grammars merely to
 * reference their values.
 *
 * Cross-domain relationships are semantic relationships.
 *
 * ============================================================================
 */

parser grammar AIMind;

options {
    tokenVocab = ZamaniLexer;
}

import Types,
       Expressions,
       Statements;


/* ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * This is the only public entry point owned by this file.
 *
 * Every alternative crosses an explicit cognitive-domain boundary.
 *
 * Ordinary expressions remain ordinary expressions.
 * ============================================================================
 */

mindConstruct
    : mindDeclaration
    | mindBinding
    | mindInvocation
    | mindRegion
    | mindResourceContract
    | mindCapabilityContract
    | mindConstraintContract
    | mindPreferenceContract
    | mindHintContract
    ;


/* ============================================================================
 * 2. MIND DECLARATION
 * ============================================================================
 *
 * Canonical structural form:
 *
 *     @mind ResearchMind {
 *         ...
 *     }
 *
 * The annotation name remains syntactically open.
 *
 * Semantic analysis determines whether the annotation identifies the
 * standardized `mind` construct, a registered dialect, or an extension.
 * ============================================================================
 */

mindDeclaration
    : AT
      mindAnnotation
      identifier
      mindTypeClause?
      mindInitializer?
      mindBody
    ;

mindAnnotation
    : identifier
    ;

mindTypeClause
    : COLON
      typeExpression
    ;

mindInitializer
    : ASSIGN
      expression
    ;

mindBody
    : LBRACE
      mindMember*
      RBRACE
    ;


/* ============================================================================
 * 3. MIND BINDING
 * ============================================================================
 *
 * Examples:
 *
 *     @mind existing = other_mind;
 *     @mind cognitive_state = state;
 *
 * The binding form is intentionally expression-based.
 * ============================================================================
 */

mindBinding
    : AT
      mindBindingAnnotation
      identifier
      mindBindingTypeClause?
      ASSIGN
      expression
      SEMI?
    ;

mindBindingAnnotation
    : identifier
    ;

mindBindingTypeClause
    : COLON
      typeExpression
    ;


/* ============================================================================
 * 4. MIND INVOCATION
 * ============================================================================
 *
 * Examples:
 *
 *     @think(problem);
 *     @reason(context);
 *     @reflect(state);
 *
 * Annotation meaning is semantic.
 *
 * This does not execute anything during parsing.
 * ============================================================================
 */

mindInvocation
    : AT
      mindInvocationAnnotation
      LPAREN
      argumentList?
      RPAREN
      SEMI?
    ;

mindInvocationAnnotation
    : identifier
    ;


/* ============================================================================
 * 5. GENERIC MIND REGION
 * ============================================================================
 *
 * Provides an explicit cognitive region without forcing every future
 * cognitive construct into the lexer.
 *
 * Example:
 *
 *     @mind {
 *         @reason(problem);
 *         @reflect(state);
 *     }
 *
 * ============================================================================
 */

mindRegion
    : AT
      mindRegionAnnotation
      mindRegionBody
    ;

mindRegionAnnotation
    : identifier
    ;

mindRegionBody
    : LBRACE
      mindMember*
      RBRACE
    ;


/* ============================================================================
 * 6. MIND MEMBERS
 * ============================================================================
 *
 * Members are structural cognitive declarations or ordinary Zamani
 * statements.
 *
 * Cognitive annotation names remain open.
 * ============================================================================
 */

mindMember
    : mindNamedMember
    | mindBindingMember
    | mindInvocationMember
    | mindAssignmentMember
    | mindRegionMember
    | mindResourceContract
    | mindCapabilityContract
    | mindConstraintContract
    | mindPreferenceContract
    | mindHintContract
    | statement
    ;


/* ============================================================================
 * 7. NAMED COGNITIVE MEMBER
 * ============================================================================
 *
 * Examples:
 *
 *     @goal solve_problem;
 *     @belief hypothesis;
 *     @knowledge facts;
 *     @memory long_term;
 *     @context environment;
 *     @concept object;
 *     @observation observation;
 *
 * The annotation name is not a closed parser enumeration.
 * ============================================================================
 */

mindNamedMember
    : AT
      mindMemberAnnotation
      identifier
      mindOptionalType?
      mindOptionalInitializer?
      SEMI?
    ;

mindMemberAnnotation
    : identifier
    ;

mindOptionalType
    : COLON
      typeExpression
    ;

mindOptionalInitializer
    : ASSIGN
      expression
    ;


/* ============================================================================
 * 8. COGNITIVE BINDING MEMBER
 * ============================================================================
 *
 * This is useful when the cognitive role is an explicitly named binding.
 *
 * Example:
 *
 *     @memory recall = memory_ref;
 *
 * ============================================================================
 */

mindBindingMember
    : AT
      mindBindingMemberAnnotation
      identifier
      ASSIGN
      expression
      SEMI?
    ;

mindBindingMemberAnnotation
    : identifier
    ;


/* ============================================================================
 * 9. COGNITIVE INVOCATION MEMBER
 * ============================================================================
 *
 * Examples:
 *
 *     @reason(problem);
 *     @infer(evidence);
 *     @predict(input);
 *     @explain(result);
 *     @reflect(state);
 *
 * No implementation is selected by the grammar.
 * ============================================================================
 */

mindInvocationMember
    : AT
      mindInvocationMemberAnnotation
      LPAREN
      argumentList?
      RPAREN
      SEMI?
    ;

mindInvocationMemberAnnotation
    : identifier
    ;


/* ============================================================================
 * 10. COGNITIVE ASSIGNMENT MEMBER
 * ============================================================================
 *
 * Example:
 *
 *     @belief = hypothesis;
 *
 * The expression remains owned by Expressions.
 * ============================================================================
 */

mindAssignmentMember
    : AT
      mindAssignmentAnnotation
      ASSIGN
      expression
      SEMI?
    ;

mindAssignmentAnnotation
    : identifier
    ;


/* ============================================================================
 * 11. NESTED COGNITIVE REGION
 * ============================================================================
 *
 * Examples:
 *
 *     @reasoning {
 *         @hypothesis h;
 *         @evidence e;
 *     }
 *
 *     @reflection {
 *         ...
 *     }
 *
 * This provides unlimited structural nesting without imposing a grammar
 * nesting constant.
 * ============================================================================
 */

mindRegionMember
    : AT
      mindRegionMemberAnnotation
      mindRegionBody
    ;

mindRegionMemberAnnotation
    : identifier
    ;


/* ============================================================================
 * 12. RESOURCE REQUIREMENT
 * ============================================================================
 *
 * `requires` is a canonical semantic contract.
 *
 * Examples:
 *
 *     requires memory >= required_memory;
 *     requires reasoning_budget >= required_budget;
 *     requires qubits >= n;
 *     requires capability_value;
 *
 * The expression is interpreted semantically.
 * ============================================================================
 */

mindResourceContract
    : REQUIRES
      expression
      SEMI?
    ;


/* ============================================================================
 * 13. CAPABILITY REQUIREMENT
 * ============================================================================
 *
 * Examples:
 *
 *     capability("reasoning");
 *     capability("tensor.compute");
 *     capability("quantum.measurement");
 *
 * Capability names remain semantic values.
 * ============================================================================
 */

mindCapabilityContract
    : CAPABILITY
      expression
      SEMI?
    ;


/* ============================================================================
 * 14. CONSTRAINT
 * ============================================================================
 *
 * Constraints are binding semantic restrictions.
 *
 * They are not physical placement instructions.
 * ============================================================================
 */

mindConstraintContract
    : CONSTRAINT
      expression
      SEMI?
    ;


/* ============================================================================
 * 15. PREFERENCE
 * ============================================================================
 *
 * Preferences are weaker than requirements and constraints.
 * ============================================================================
 */

mindPreferenceContract
    : PREFER
      expression
      SEMI?
    ;


/* ============================================================================
 * 16. HINT
 * ============================================================================
 *
 * Hints are advisory.
 *
 * Ignoring a hint must not invalidate the semantic program.
 * ============================================================================
 */

mindHintContract
    : HINT
      expression
      SEMI?
    ;


/* ============================================================================
 * 17. COGNITIVE STATE ANCHOR
 * ============================================================================
 *
 * A stable structural parser anchor for state-related cognition.
 *
 * Examples:
 *
 *     @state cognitive_state;
 *     @state {
 *         ...
 *     }
 *
 * The annotation name remains open so future state models do not require
 * lexer changes.
 * ============================================================================
 */

mindState
    : AT
      mindStateAnnotation
      mindStatePayload?
      SEMI?
    ;

mindStateAnnotation
    : identifier
    ;

mindStatePayload
    : identifier
    | ASSIGN expression
    | mindRegionBody
    ;


/* ============================================================================
 * 18. KNOWLEDGE ANCHOR
 * ============================================================================
 *
 * Knowledge is represented as a source-level semantic reference or region.
 *
 * The grammar does not define a knowledge database.
 * ============================================================================
 */

mindKnowledge
    : AT
      mindKnowledgeAnnotation
      mindKnowledgePayload?
      SEMI?
    ;

mindKnowledgeAnnotation
    : identifier
    ;

mindKnowledgePayload
    : identifier
    | ASSIGN expression
    | LPAREN argumentList? RPAREN
    | mindRegionBody
    ;


/* ============================================================================
 * 19. BELIEF ANCHOR
 * ============================================================================
 */

mindBelief
    : AT
      mindBeliefAnnotation
      mindBeliefPayload?
      SEMI?
    ;

mindBeliefAnnotation
    : identifier
    ;

mindBeliefPayload
    : identifier
    | ASSIGN expression
    | LPAREN argumentList? RPAREN
    | mindRegionBody
    ;


/* ============================================================================
 * 20. MEMORY REFERENCE ANCHOR
 * ============================================================================
 *
 * Memory is referenced, not implemented.
 * ============================================================================
 */

mindMemoryReference
    : AT
      mindMemoryAnnotation
      mindMemoryPayload?
      SEMI?
    ;

mindMemoryAnnotation
    : identifier
    ;

mindMemoryPayload
    : identifier
    | ASSIGN expression
    | LPAREN argumentList? RPAREN
    | mindRegionBody
    ;


/* ============================================================================
 * 21. OBSERVATION / EVIDENCE ANCHOR
 * ============================================================================
 */

mindObservation
    : AT
      mindObservationAnnotation
      mindObservationPayload?
      SEMI?
    ;

mindObservationAnnotation
    : identifier
    ;

mindObservationPayload
    : identifier
    | ASSIGN expression
    | LPAREN argumentList? RPAREN
    | mindRegionBody
    ;


/* ============================================================================
 * 22. GOAL / OBJECTIVE ANCHOR
 * ============================================================================
 */

mindGoal
    : AT
      mindGoalAnnotation
      mindGoalPayload?
      SEMI?
    ;

mindGoalAnnotation
    : identifier
    ;

mindGoalPayload
    : identifier
    | ASSIGN expression
    | LPAREN argumentList? RPAREN
    | mindRegionBody
    ;


/* ============================================================================
 * 23. REASONING ANCHOR
 * ============================================================================
 *
 * Reasoning is an intent boundary.
 *
 * The actual reasoning algorithm is selected semantically.
 * ============================================================================
 */

mindReasoning
    : AT
      mindReasoningAnnotation
      mindReasoningPayload?
      SEMI?
    ;

mindReasoningAnnotation
    : identifier
    ;

mindReasoningPayload
    : identifier
    | ASSIGN expression
    | LPAREN argumentList? RPAREN
    | mindRegionBody
    ;


/* ============================================================================
 * 24. INFERENCE ANCHOR
 * ============================================================================
 *
 * This references inference intent without duplicating inference.g4.
 * ============================================================================
 */

mindInference
    : AT
      mindInferenceAnnotation
      mindInferencePayload?
      SEMI?
    ;

mindInferenceAnnotation
    : identifier
    ;

mindInferencePayload
    : identifier
    | ASSIGN expression
    | LPAREN argumentList? RPAREN
    | mindRegionBody
    ;


/* ============================================================================
 * 25. PLANNING ANCHOR
 * ============================================================================
 *
 * Planning here means cognitive planning intent.
 *
 * External agent execution remains owned by agent.g4 and downstream runtime
 * semantics.
 * ============================================================================
 */

mindPlanning
    : AT
      mindPlanningAnnotation
      mindPlanningPayload?
      SEMI?
    ;

mindPlanningAnnotation
    : identifier
    ;

mindPlanningPayload
    : identifier
    | ASSIGN expression
    | LPAREN argumentList? RPAREN
    | mindRegionBody
    ;


/* ============================================================================
 * 26. DECISION ANCHOR
 * ============================================================================
 */

mindDecision
    : AT
      mindDecisionAnnotation
      mindDecisionPayload?
      SEMI?
    ;

mindDecisionAnnotation
    : identifier
    ;

mindDecisionPayload
    : identifier
    | ASSIGN expression
    | LPAREN argumentList? RPAREN
    | mindRegionBody
    ;


/* ============================================================================
 * 27. LEARNING ANCHOR
 * ============================================================================
 *
 * Learning intent is referenced here.
 *
 * Training algorithms remain owned by training.g4 and the semantic/compiler
 * layers.
 * ============================================================================
 */

mindLearning
    : AT
      mindLearningAnnotation
      mindLearningPayload?
      SEMI?
    ;

mindLearningAnnotation
    : identifier
    ;

mindLearningPayload
    : identifier
    | ASSIGN expression
    | LPAREN argumentList? RPAREN
    | mindRegionBody
    ;


/* ============================================================================
 * 28. REFLECTION ANCHOR
 * ============================================================================
 */

mindReflection
    : AT
      mindReflectionAnnotation
      mindReflectionPayload?
      SEMI?
    ;

mindReflectionAnnotation
    : identifier
    ;

mindReflectionPayload
    : identifier
    | ASSIGN expression
    | LPAREN argumentList? RPAREN
    | mindRegionBody
    ;


/* ============================================================================
 * 29. CONTEXT ANCHOR
 * ============================================================================
 */

mindContext
    : AT
      mindContextAnnotation
      mindContextPayload?
      SEMI?
    ;

mindContextAnnotation
    : identifier
    ;

mindContextPayload
    : identifier
    | ASSIGN expression
    | LPAREN argumentList? RPAREN
    | mindRegionBody
    ;


/* ============================================================================
 * 30. ATTENTION / PERCEPTION ANCHORS
 * ============================================================================
 */

mindAttention
    : AT
      mindAttentionAnnotation
      mindAttentionPayload?
      SEMI?
    ;

mindAttentionAnnotation
    : identifier
    ;

mindAttentionPayload
    : identifier
    | ASSIGN expression
    | LPAREN argumentList? RPAREN
    | mindRegionBody
    ;


mindPerception
    : AT
      mindPerceptionAnnotation
      mindPerceptionPayload?
      SEMI?
    ;

mindPerceptionAnnotation
    : identifier
    ;

mindPerceptionPayload
    : identifier
    | ASSIGN expression
    | LPAREN argumentList? RPAREN
    | mindRegionBody
    ;


/* ============================================================================
 * 31. HYPOTHESIS / EVIDENCE / EXPLANATION ANCHORS
 * ============================================================================
 */

mindHypothesis
    : AT
      mindHypothesisAnnotation
      mindHypothesisPayload?
      SEMI?
    ;

mindHypothesisAnnotation
    : identifier
    ;

mindHypothesisPayload
    : identifier
    | ASSIGN expression
    | LPAREN argumentList? RPAREN
    | mindRegionBody
    ;


mindEvidence
    : AT
      mindEvidenceAnnotation
      mindEvidencePayload?
      SEMI?
    ;

mindEvidenceAnnotation
    : identifier
    ;

mindEvidencePayload
    : identifier
    | ASSIGN expression
    | LPAREN argumentList? RPAREN
    | mindRegionBody
    ;


mindExplanation
    : AT
      mindExplanationAnnotation
      mindExplanationPayload?
      SEMI?
    ;

mindExplanationAnnotation
    : identifier
    ;

mindExplanationPayload
    : identifier
    | ASSIGN expression
    | LPAREN argumentList? RPAREN
    | mindRegionBody
    ;


/* ============================================================================
 * 32. QUESTION / ANSWER ANCHORS
 * ============================================================================
 *
 * These are cognitive structures, not I/O operations.
 * ============================================================================
 */

mindQuestion
    : AT
      mindQuestionAnnotation
      mindQuestionPayload?
      SEMI?
    ;

mindQuestionAnnotation
    : identifier
    ;

mindQuestionPayload
    : identifier
    | ASSIGN expression
    | LPAREN argumentList? RPAREN
    | mindRegionBody
    ;


mindAnswer
    : AT
      mindAnswerAnnotation
      mindAnswerPayload?
      SEMI?
    ;

mindAnswerAnnotation
    : identifier
    ;

mindAnswerPayload
    : identifier
    | ASSIGN expression
    | LPAREN argumentList? RPAREN
    | mindRegionBody
    ;


/* ============================================================================
 * 33. PROVENANCE ANCHOR
 * ============================================================================
 *
 * Provenance connects cognition to source/data/model/history information.
 *
 * Actual provenance tracking belongs to the semantic/data/security systems.
 * ============================================================================
 */

mindProvenance
    : AT
      mindProvenanceAnnotation
      mindProvenancePayload?
      SEMI?
    ;

mindProvenanceAnnotation
    : identifier
    ;

mindProvenancePayload
    : identifier
    | ASSIGN expression
    | LPAREN argumentList? RPAREN
    | mindRegionBody
    ;


/* ============================================================================
 * 34. TEMPORAL / MULTI-TIMELINE BRIDGE
 * ============================================================================
 *
 * The mind may refer to temporal state, history, or timeline concepts.
 *
 * It does not implement MTS.
 *
 * No fixed number of timelines or timestamps is imposed.
 * ============================================================================
 */

mindTemporal
    : AT
      mindTemporalAnnotation
      mindTemporalPayload?
      SEMI?
    ;

mindTemporalAnnotation
    : identifier
    ;

mindTemporalPayload
    : identifier
    | ASSIGN expression
    | LPAREN argumentList? RPAREN
    | mindRegionBody
    ;


/* ============================================================================
 * 35. CROSS-DOMAIN COMPUTATION REGION
 * ============================================================================
 *
 * A mind may contain ordinary Zamani statements.
 *
 * This is the important bridge that allows:
 *
 *     classical
 *       ->
 *     AI cognition
 *       ->
 *     quantum
 *       ->
 *     measurement
 *       ->
 *     classical decision
 *       ->
 *     AI cognition
 *
 * without creating a second language.
 * ============================================================================
 */

mindComputationRegion
    : LBRACE
      statement*
      RBRACE
    ;


/* ============================================================================
 * 36. EXTENSION MEMBER
 * ============================================================================
 *
 * The ordinary member structure already provides open-world extension through
 * annotations and expressions.
 *
 * This explicit anchor exists for dialect tooling and semantic registries.
 * ============================================================================
 */

mindExtension
    : AT
      mindExtensionAnnotation
      mindExtensionPayload?
      SEMI?
    ;

mindExtensionAnnotation
    : identifier
    ;

mindExtensionPayload
    : identifier
    | ASSIGN expression
    | LPAREN argumentList? RPAREN
    | mindRegionBody
    ;


/* ============================================================================
 * 37. COMPLETION CONTRACT
 * ============================================================================
 *
 * This file is complete as an independently owned source-syntax component when:
 *
 * [x] it has one parser grammar identity;
 * [x] it has one public mindConstruct entry point;
 * [x] it imports canonical Types;
 * [x] it imports canonical Expressions;
 * [x] it imports canonical Statements;
 * [x] it consumes the canonical ZamaniLexer;
 * [x] it uses AT instead of NANO_ANNOTATION;
 * [x] it introduces no lexer tokens;
 * [x] it introduces no embedded Rust;
 * [x] it introduces no semantic predicates;
 * [x] it introduces no unsafe implementation;
 * [x] it preserves ordinary expressions;
 * [x] it preserves ordinary statements;
 * [x] it preserves ordinary types;
 * [x] it does not duplicate model grammar;
 * [x] it does not duplicate tensor grammar;
 * [x] it does not duplicate dataset grammar;
 * [x] it does not duplicate training grammar;
 * [x] it does not duplicate inference grammar;
 * [x] it does not duplicate agent grammar;
 * [x] it does not duplicate memory semantics;
 * [x] it does not duplicate resource semantics;
 * [x] it does not duplicate hardware semantics;
 * [x] it does not define quantum gates;
 * [x] it does not define physical qubits;
 * [x] it does not define QEC;
 * [x] it does not define ZQN;
 * [x] it does not create another quantum IR;
 * [x] it does not create another AI IR;
 * [x] it does not impose machine-size limits;
 * [x] it does not impose cognitive-size limits;
 * [x] it permits arbitrary structural repetition;
 * [x] it provides explicit AST-preservation requirements;
 * [x] it provides semantic ownership boundaries;
 * [x] it provides resource/capability boundaries;
 * [x] it provides classical/quantum/hardware integration boundaries;
 * [x] it supports future cognitive dialects;
 * [x] parsing remains deterministic;
 * [x] target realization remains downstream.
 *
 * ============================================================================
 */
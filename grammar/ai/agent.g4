/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/ai/agent.g4
 *
 * GRAMMAR
 * -------
 * AIAgent
 *
 * STATUS
 * ------
 * CANONICAL AI AGENT / AGENTIC-COMPUTATION LEAF GRAMMAR
 *
 * LANGUAGE BASELINE
 * -----------------
 * Rust 1.97 / Rust 1.97.1
 * Rust 2021
 * Safe Rust only
 *
 * SAFETY
 * ------
 * This file is parser grammar only.
 *
 * It contains:
 *
 *   - no embedded Rust actions;
 *   - no semantic predicates;
 *   - no unsafe code;
 *   - no filesystem access;
 *   - no network access;
 *   - no hardware discovery;
 *   - no runtime execution;
 *   - no mutable global state;
 *   - no vendor-specific implementation;
 *   - no target-specific implementation.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the SOURCE-LEVEL STRUCTURE of agentic computation.
 *
 * It represents portable agent intent including:
 *
 *   - agent declarations;
 *   - agent bindings;
 *   - goals;
 *   - objectives;
 *   - models;
 *   - tools;
 *   - memory references;
 *   - observations;
 *   - actions;
 *   - policies;
 *   - plans;
 *   - plan steps;
 *   - delegation;
 *   - coordination;
 *   - messages;
 *   - events;
 *   - checkpoints;
 *   - termination conditions;
 *   - resource requirements;
 *   - capability requirements;
 *   - constraints;
 *   - preferences;
 *   - configuration;
 *   - extensions;
 *   - arbitrary nested agent composition;
 *   - ordinary Zamani computation inside agent regions.
 *
 * The grammar intentionally uses an OPEN-WORLD agent vocabulary.
 *
 * Agent concepts are represented by:
 *
 *     @ identifier ...
 *
 * rather than by an ever-growing collection of lexer keywords.
 *
 * Semantic analysis determines whether an annotation means:
 *
 *     @agent
 *     @goal
 *     @objective
 *     @model
 *     @tool
 *     @memory
 *     @observation
 *     @action
 *     @policy
 *     @plan
 *     @step
 *     @delegate
 *     @coordinate
 *     @message
 *     @event
 *     @checkpoint
 *     @termination
 *     @requires
 *     @capability
 *     @constraint
 *     @preference
 *
 * or a registered future/domain-specific construct.
 *
 * The parser does NOT hard-code that vocabulary.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     AIAgent
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *     +----+-----------+------------+-------------+
 *     |                |            |             |
 *     v                v            v             v
 *   types          capabilities   resources     effects
 *     |                |            |             |
 *     +----------------+------------+-------------+
 *                              |
 *                              v
 *                     canonical semantic model
 *                              |
 *             +----------------+----------------+
 *             |                |               |
 *             v                v               v
 *         classical        quantum::ir       HDL/
 *         lowering         lowering         hardware
 *             |                |               |
 *             +----------------+---------------+
 *                              |
 *                              v
 *                         optimization
 *                              |
 *                         scheduling
 *                              |
 *                         resilience
 *                              |
 *                             ZQN
 *                              |
 *                             HAL
 *                              |
 *                       target realization
 *                              |
 *                             runtime
 *
 * This grammar MUST NOT create a second AI IR or agent IR.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS
 * --------------
 *
 *   - agent source construct boundary;
 *   - agent declaration structure;
 *   - agent binding structure;
 *   - agent annotation structure;
 *   - agent member structure;
 *   - agent nested composition;
 *   - agent-local bodies;
 *   - agent-local references;
 *   - agent-local type clauses;
 *   - agent-local initializer clauses;
 *   - agent-local directive arguments;
 *   - agent-local resource/capability contract boundaries;
 *   - agent-local extensibility boundaries.
 *
 * THIS FILE DOES NOT OWN
 * ----------------------
 *
 *   - lexer rules;
 *   - annotation tokenization;
 *   - identifiers;
 *   - paths;
 *   - general types;
 *   - general expressions;
 *   - general statements;
 *   - functions;
 *   - modules;
 *   - tensor semantics;
 *   - model internals;
 *   - datasets;
 *   - training;
 *   - inference algorithms;
 *   - automatic differentiation;
 *   - networking protocols;
 *   - distributed topology;
 *   - hardware topology;
 *   - accelerator implementation;
 *   - quantum operations;
 *   - quantum topology;
 *   - QEC;
 *   - ZQN;
 *   - routing;
 *   - scheduling;
 *   - calibration;
 *   - HAL;
 *   - device selection;
 *   - resource discovery;
 *   - runtime execution;
 *   - IR construction.
 *
 * ============================================================================
 * CANONICAL DEPENDENCIES
 * ============================================================================
 *
 * Annotation marker:
 *
 *     grammar/lexer/annotations.g4
 *
 * Canonical annotation structure:
 *
 *     AT identifier
 *
 * General types:
 *
 *     Types
 *
 * General expressions:
 *
 *     Expressions
 *
 * General statements:
 *
 *     Statements
 *
 * This grammar MUST reuse those contracts.
 *
 * ============================================================================
 * ANNOTATION TOKEN CORRECTION
 * ============================================================================
 *
 * IMPORTANT:
 *
 * The canonical lexer architecture owns:
 *
 *     AT : '@'
 *
 * through the annotation lexer component.
 *
 * Therefore this grammar deliberately uses:
 *
 *     AT identifier
 *
 * and MUST NOT use the legacy:
 *
 *     NANO_ANNOTATION
 *
 * representation.
 *
 * This prevents agent.g4 from reinforcing the existing AI annotation-token
 * inconsistency.
 *
 * The semantic annotation name remains open-ended.
 *
 * ============================================================================
 * OPEN-WORLD AGENT MODEL
 * ============================================================================
 *
 * This grammar does NOT enumerate agent types.
 *
 * It does NOT define a closed list of:
 *
 *     reactive agent
 *     deliberative agent
 *     planning agent
 *     autonomous agent
 *     multi-agent system
 *     embodied agent
 *     software agent
 *     quantum agent
 *     neural agent
 *     symbolic agent
 *     hybrid agent
 *
 * Such categories are semantic classifications.
 *
 * The grammar instead provides structural syntax from which those concepts
 * can be represented.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Agent source describes:
 *
 *     WHAT an agent is;
 *     WHAT it observes;
 *     WHAT it may do;
 *     WHAT goals it has;
 *     WHAT policies constrain it;
 *     WHAT resources it requires;
 *     WHAT capabilities it requires;
 *     WHAT dependencies it has;
 *     WHAT computations it performs;
 *     WHAT interactions it participates in.
 *
 * Agent source MUST NOT require a particular machine.
 *
 * It MUST NOT encode:
 *
 *     CPU 0
 *     GPU 0
 *     QPU 0
 *     FPGA 0
 *     device 0
 *     physical qubit 17
 *     node 3
 *     fixed RAM
 *     fixed VRAM
 *     fixed core count
 *     fixed worker count
 *     fixed cluster size
 *     fixed accelerator count
 *     fixed network topology
 *
 * Target realization is downstream.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This grammar imposes NO universal finite limits on:
 *
 *     - agents;
 *     - nested agents;
 *     - goals;
 *     - objectives;
 *     - tools;
 *     - memories;
 *     - observations;
 *     - actions;
 *     - plans;
 *     - steps;
 *     - branches;
 *     - delegations;
 *     - messages;
 *     - events;
 *     - checkpoints;
 *     - capabilities;
 *     - requirements;
 *     - resources;
 *     - workers;
 *     - processes;
 *     - nodes;
 *     - devices;
 *     - accelerators;
 *     - tensors;
 *     - models;
 *     - quantum resources;
 *     - classical resources.
 *
 * Repetition is structural:
 *
 *     *
 *     +
 *
 * Practical limitations may come from compiler/runtime resources, but those
 * limitations MUST NOT become language-level constants.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY SEMANTICS
 * ============================================================================
 *
 * The grammar preserves the distinction between:
 *
 *     requirement
 *     capability
 *     constraint
 *     preference
 *     hint
 *     implementation decision
 *
 * For example:
 *
 *     @requires capability("quantum.measurement");
 *
 * expresses portable semantic intent.
 *
 * It MUST NOT imply:
 *
 *     use QPU 0
 *     use physical qubit 7
 *     use vendor X
 *
 * Resource discovery, capability satisfaction, placement and scheduling are
 * downstream concerns.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * An agent may participate in quantum/classical hybrid computation through
 * ordinary expressions and statements.
 *
 * This grammar does NOT define:
 *
 *     QubitId
 *     PhysicalQubitId
 *     quantum gates
 *     quantum topology
 *     measurement semantics
 *     QEC
 *     ZQN
 *     quantum scheduling
 *
 * Quantum semantics ultimately cross:
 *
 *     quantum::ir
 *
 * The agent grammar therefore remains independent of the quantum IR.
 *
 * ============================================================================
 * CLASSICAL / HDL / DISTRIBUTED / NETWORK / SECURITY INTEGRATION
 * ============================================================================
 *
 * Agent bodies may contain ordinary Zamani statements.
 *
 * Consequently an agent can semantically compose:
 *
 *     classical computation
 *     tensor computation
 *     neural computation
 *     quantum computation
 *     measurement
 *     hardware interaction
 *     distributed computation
 *     networking
 *     secure computation
 *
 * without introducing separate agent-specific versions of those languages.
 *
 * ============================================================================
 * MODEL / TOOL / MEMORY OWNERSHIP
 * ============================================================================
 *
 * Agent syntax may reference models, tools and memory.
 *
 * However:
 *
 *     model internals      -> models.g4
 *     tensor semantics     -> tensors.g4
 *     training             -> training.g4
 *     inference            -> inference.g4
 *     differentiation      -> differentiation.g4
 *     memory               -> memory/
 *     networking           -> networking/
 *     security             -> security/
 *     distributed          -> distributed/
 *
 * Agent.g4 only describes their relationship to an agent.
 *
 * ============================================================================
 * POLICY
 * ============================================================================
 *
 * Agent policies are source-level declarations of intent/constraints.
 *
 * This grammar does not execute or evaluate policies.
 *
 * Policy enforcement belongs to:
 *
 *     semantic analysis
 *     effect analysis
 *     capability analysis
 *     resource analysis
 *     security analysis
 *     runtime policy infrastructure
 *
 * ============================================================================
 * PLAN / SCHEDULING SEPARATION
 * ============================================================================
 *
 * A plan can describe logical ordering and dependencies.
 *
 * The grammar does NOT determine:
 *
 *     execution time;
 *     processor assignment;
 *     worker assignment;
 *     physical placement;
 *     network route;
 *     quantum mapping;
 *     hardware schedule.
 *
 * Those remain compiler/runtime responsibilities.
 *
 * ============================================================================
 * CHECKPOINT SEMANTICS
 * ============================================================================
 *
 * Checkpoint syntax describes an intended checkpoint boundary.
 *
 * It does NOT guarantee that every state is serializable.
 *
 * For example, a quantum state may require:
 *
 *     measurement;
 *     logical-state support;
 *     reconstruction;
 *     provider support;
 *     QEC-aware handling;
 *
 * The semantic/runtime layer determines whether the requested checkpoint is
 * valid.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend AST MUST remain domain-neutral.
 *
 * The parser should preserve:
 *
 *     - source span;
 *     - annotation;
 *     - annotation arguments;
 *     - target/reference;
 *     - type;
 *     - initializer;
 *     - directive arguments;
 *     - nested members;
 *     - ordinary statements;
 *     - source ordering;
 *     - source provenance.
 *
 * The AST should NOT contain:
 *
 *     AgentCpuNode
 *     AgentGpuNode
 *     AgentQpuNode
 *     AgentCudaNode
 *     AgentTensorFlowNode
 *     AgentPhysicalDeviceNode
 *
 * Semantic analysis may classify the generic annotated construct as an
 * agent construct after parsing.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis owns:
 *
 *     - validating annotation names;
 *     - resolving agent declarations;
 *     - resolving references;
 *     - validating scopes;
 *     - validating duplicate names;
 *     - validating types;
 *     - validating expressions;
 *     - validating policy semantics;
 *     - validating goals/objectives;
 *     - validating tool references;
 *     - validating model references;
 *     - validating memory references;
 *     - validating delegation;
 *     - validating coordination;
 *     - validating message semantics;
 *     - validating lifecycle semantics;
 *     - validating resource requirements;
 *     - validating capabilities;
 *     - validating constraints;
 *     - validating preferences;
 *     - validating effects;
 *     - validating security requirements;
 *     - validating portability;
 *     - validating quantum interoperability.
 *
 * The parser performs none of these semantic decisions.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     - token sequence;
 *     - active grammar version.
 *
 * Parsing MUST NOT depend on:
 *
 *     - system time;
 *     - randomness;
 *     - environment variables;
 *     - filesystem state;
 *     - network state;
 *     - hardware state;
 *     - runtime state;
 *     - device availability.
 *
 * ============================================================================
 * EXTENSIBILITY
 * ============================================================================
 *
 * The principal extension mechanism is:
 *
 *     AT identifier ...
 *
 * New agent roles can therefore be added semantically without modifying:
 *
 *     - ZamaniLexer;
 *     - this grammar;
 *     - the root expression grammar.
 *
 * Framework/vendor names remain semantic/dialect concerns.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * GRAMMAR DECLARATION
 * ============================================================================
 */

parser grammar AIAgent;

options {
    tokenVocab = ZamaniLexer;
}

import Types,
       Expressions,
       Statements;


/*
 * ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * This is the only public entry point owned by this grammar.
 *
 * The canonical parser composition layer should import this rule rather than
 * importing individual implementation rules.
 */

aiAgentConstruct
    : aiAgentAnnotatedConstruct
    ;


/*
 * ============================================================================
 * 2. GENERIC ANNOTATED AGENT CONSTRUCT
 * ============================================================================
 *
 * Structural forms supported:
 *
 *     @agent Name { ... }
 *
 *     @agent Name: AgentType { ... }
 *
 *     @agent Name = expression;
 *
 *     @goal objective;
 *
 *     @goal solve(problem);
 *
 *     @model model;
 *
 *     @tool search;
 *
 *     @requires capability("...");
 *
 *     @checkpoint state;
 *
 *     @termination condition;
 *
 *     @future_extension arbitrary_payload;
 *
 * The parser deliberately does not enumerate these semantic roles.
 */

aiAgentAnnotatedConstruct
    : aiAgentAnnotation
      aiAgentSubject?
      aiAgentTypeClause?
      aiAgentInitializer?
      aiAgentTerminator
    ;


/*
 * ============================================================================
 * 3. ANNOTATION
 * ============================================================================
 *
 * Annotation names remain ordinary identifiers.
 *
 * Semantic analysis determines whether the annotation is:
 *
 *     agent
 *     goal
 *     model
 *     tool
 *     memory
 *     policy
 *     ...
 *
 * or a registered future extension.
 */

aiAgentAnnotation
    : AT
      identifier
    ;


/*
 * ============================================================================
 * 4. SUBJECT / TARGET
 * ============================================================================
 *
 * A subject can be:
 *
 *     identifier
 *
 * or:
 *
 *     callable-style expression
 *
 * This permits both:
 *
 *     @goal solve;
 *
 * and:
 *
 *     @goal solve(problem);
 *
 * while leaving the meaning to semantic analysis.
 */

aiAgentSubject
    : identifier
    | aiAgentCallSubject
    ;


aiAgentCallSubject
    : identifier
      LPAREN
      argumentList?
      RPAREN
    ;


/*
 * ============================================================================
 * 5. TYPE CLAUSE
 * ============================================================================
 *
 * General type syntax belongs to Types.
 */

aiAgentTypeClause
    : COLON
      typeExpression
    ;


/*
 * ============================================================================
 * 6. INITIALIZER
 * ============================================================================
 *
 * General expression syntax belongs to Expressions.
 */

aiAgentInitializer
    : ASSIGN
      expression
    ;


/*
 * ============================================================================
 * 7. TERMINATOR
 * ============================================================================
 *
 * A construct either:
 *
 *     - owns a nested body; or
 *     - terminates with SEMICOLON.
 *
 * This gives nested agent composition a single structural boundary.
 */

aiAgentTerminator
    : aiAgentBody
    | SEMICOLON
    ;


/*
 * ============================================================================
 * 8. AGENT BODY
 * ============================================================================
 *
 * Bodies can contain:
 *
 *     - nested agent constructs;
 *     - ordinary Zamani statements.
 *
 * No fixed member count is imposed.
 */

aiAgentBody
    : LBRACE
      aiAgentMember*
      RBRACE
    ;


/*
 * ============================================================================
 * 9. AGENT MEMBERS
 * ============================================================================
 *
 * Ordinary statements remain owned by Statements.
 *
 * This permits an agent body to perform normal Zamani computation without
 * inventing an agent-specific programming language.
 */

aiAgentMember
    : aiAgentAnnotatedConstruct
    | statement
    ;


/*
 * ============================================================================
 * 10. PUBLIC BODY CONTENT ALIAS
 * ============================================================================
 *
 * Stable helper for AST visitors and conformance tooling.
 */

aiAgentContents
    : aiAgentMember*
    ;


/*
 * ============================================================================
 * 11. AGENT EXPRESSION BRIDGE
 * ============================================================================
 *
 * No second expression grammar is created.
 */

aiAgentExpression
    : expression
    ;


/*
 * ============================================================================
 * 12. AGENT TYPE BRIDGE
 * ============================================================================
 *
 * No second type grammar is created.
 */

aiAgentType
    : typeExpression
    ;


/*
 * ============================================================================
 * 13. AGENT REFERENCE
 * ============================================================================
 */

aiAgentReference
    : identifier
    ;


/*
 * ============================================================================
 * 14. AGENT CALL
 * ============================================================================
 *
 * This is a reusable structural helper for semantic tooling.
 */

aiAgentCall
    : identifier
      LPAREN
      argumentList?
      RPAREN
    ;


/*
 * ============================================================================
 * 15. SPECIALIZED SEMANTIC BOUNDARIES
 * ============================================================================
 *
 * These aliases provide stable visitor/semantic integration points without
 * duplicating syntax.
 *
 * They are intentionally NOT separate grammars.
 *
 * Semantic validation determines whether the annotation is actually the
 * requested role.
 */


/* Agent declaration */

aiAgentDeclaration
    : aiAgentAnnotatedConstruct
    ;


/* Goal */

aiAgentGoal
    : aiAgentAnnotatedConstruct
    ;


/* Objective */

aiAgentObjective
    : aiAgentAnnotatedConstruct
    ;


/* Model reference */

aiAgentModel
    : aiAgentAnnotatedConstruct
    ;


/* Tool */

aiAgentTool
    : aiAgentAnnotatedConstruct
    ;


/* Memory */

aiAgentMemory
    : aiAgentAnnotatedConstruct
    ;


/* Observation */

aiAgentObservation
    : aiAgentAnnotatedConstruct
    ;


/* Action */

aiAgentAction
    : aiAgentAnnotatedConstruct
    ;


/* Policy */

aiAgentPolicy
    : aiAgentAnnotatedConstruct
    ;


/* Plan */

aiAgentPlan
    : aiAgentAnnotatedConstruct
    ;


/* Plan step */

aiAgentStep
    : aiAgentAnnotatedConstruct
    ;


/* Delegation */

aiAgentDelegation
    : aiAgentAnnotatedConstruct
    ;


/* Coordination */

aiAgentCoordination
    : aiAgentAnnotatedConstruct
    ;


/* Message */

aiAgentMessage
    : aiAgentAnnotatedConstruct
    ;


/* Event */

aiAgentEvent
    : aiAgentAnnotatedConstruct
    ;


/* Checkpoint */

aiAgentCheckpoint
    : aiAgentAnnotatedConstruct
    ;


/* Termination */

aiAgentTermination
    : aiAgentAnnotatedConstruct
    ;


/* Requirement */

aiAgentRequirement
    : aiAgentAnnotatedConstruct
    ;


/* Capability */

aiAgentCapability
    : aiAgentAnnotatedConstruct
    ;


/* Constraint */

aiAgentConstraint
    : aiAgentAnnotatedConstruct
    ;


/* Preference */

aiAgentPreference
    : aiAgentAnnotatedConstruct
    ;


/* Configuration */

aiAgentConfiguration
    : aiAgentAnnotatedConstruct
    ;


/* Future extension */

aiAgentExtension
    : aiAgentAnnotatedConstruct
    ;


/*
 * ============================================================================
 * 16. COMPLETION CONTRACT
 * ============================================================================
 *
 * AIAgent is complete when all of the following remain true:
 *
 * [ ] Canonical lexer remains the sole lexical authority.
 *
 * [ ] Annotation marker is consumed as AT.
 *
 * [ ] No NANO_ANNOTATION dependency exists in this file.
 *
 * [ ] Identifier syntax is delegated to the canonical identifier grammar.
 *
 * [ ] Type syntax is delegated to Types.
 *
 * [ ] Expression syntax is delegated to Expressions.
 *
 * [ ] Statement syntax is delegated to Statements.
 *
 * [ ] No agent-specific expression grammar exists.
 *
 * [ ] No agent-specific type grammar exists.
 *
 * [ ] No agent-specific statement language exists.
 *
 * [ ] No AI framework is encoded into syntax.
 *
 * [ ] No vendor is encoded into syntax.
 *
 * [ ] No CPU/GPU/FPGA/QPU is selected by syntax.
 *
 * [ ] No physical device is selected by syntax.
 *
 * [ ] No hardware topology is encoded.
 *
 * [ ] No resource capacity is hard-coded.
 *
 * [ ] No maximum agent count exists.
 *
 * [ ] No maximum goal count exists.
 *
 * [ ] No maximum tool count exists.
 *
 * [ ] No maximum plan count exists.
 *
 * [ ] No maximum step count exists.
 *
 * [ ] No maximum worker count exists.
 *
 * [ ] No maximum node count exists.
 *
 * [ ] No maximum device count exists.
 *
 * [ ] No maximum tensor rank exists.
 *
 * [ ] No maximum memory size exists.
 *
 * [ ] No fixed register width exists.
 *
 * [ ] No fixed quantum capacity exists.
 *
 * [ ] Agent bodies can contain ordinary Zamani statements.
 *
 * [ ] Nested agent composition is supported.
 *
 * [ ] Future agent roles can be introduced semantically.
 *
 * [ ] Resource requirements remain separate from implementation decisions.
 *
 * [ ] Capability requirements remain separate from physical device selection.
 *
 * [ ] Policy syntax does not execute policy logic.
 *
 * [ ] Checkpoint syntax does not assume every state is serializable.
 *
 * [ ] Quantum semantics remain outside this grammar.
 *
 * [ ] quantum::ir remains the canonical quantum boundary.
 *
 * [ ] No second AI/agent IR is introduced.
 *
 * [ ] Parsing remains deterministic.
 *
 * [ ] The parser contains no executable actions.
 *
 * [ ] The parser contains no semantic predicates.
 *
 * [ ] The resulting Rust implementation requires no unsafe code.
 *
 * [ ] Rust 1.97 / Rust 1.97.1 remains supported.
 *
 * [ ] Positive tests exist.
 *
 * [ ] Negative tests exist.
 *
 * [ ] Boundary tests exist.
 *
 * [ ] Scalability tests exist.
 *
 * [ ] Determinism tests exist.
 *
 * [ ] Compatibility tests exist.
 */
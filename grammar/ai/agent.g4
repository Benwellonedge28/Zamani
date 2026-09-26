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
 * CANONICAL AI AGENT / AGENTIC COMPUTATION LEAF GRAMMAR
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
 * This grammar owns the SOURCE-LEVEL STRUCTURE of agentic computation.
 *
 * It provides an open-world, target-independent syntax for:
 *
 *     - agents;
 *     - nested agents;
 *     - goals;
 *     - objectives;
 *     - models;
 *     - tools;
 *     - memory;
 *     - observations;
 *     - actions;
 *     - policies;
 *     - plans;
 *     - plan steps;
 *     - delegation;
 *     - coordination;
 *     - messages;
 *     - events;
 *     - checkpoints;
 *     - termination;
 *     - requirements;
 *     - capabilities;
 *     - constraints;
 *     - preferences;
 *     - configuration;
 *     - future extensions;
 *     - ordinary Zamani computation inside agent bodies.
 *
 * The grammar intentionally does NOT enumerate agent roles as parser
 * alternatives.
 *
 * Instead, the structural form:
 *
 *     AT identifier ...
 *
 * is interpreted by semantic analysis.
 *
 * Consequently the language can evolve from:
 *
 *     @agent
 *     @goal
 *     @model
 *     @tool
 *     @memory
 *     @policy
 *     @plan
 *     @observe
 *     @act
 *     @delegate
 *     @coordinate
 *     @message
 *     @requires
 *     @capability
 *
 * to future agent concepts without continuously modifying this grammar.
 *
 * ============================================================================
 * ARCHITECTURAL PIPELINE
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
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
 *          +--> name resolution
 *          +--> type analysis
 *          +--> effect analysis
 *          +--> capability analysis
 *          +--> resource analysis
 *          +--> security analysis
 *          +--> portability analysis
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> classical lowering
 *          +--> quantum lowering
 *          +--> AI/model lowering
 *          +--> distributed lowering
 *          +--> hardware realization
 *          |
 *          v
 *     canonical IR
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     routing / scheduling / resilience
 *          |
 *          v
 *     ZQN / HAL where applicable
 *          |
 *          v
 *     target realization
 *
 * This grammar MUST NOT create an Agent IR or AI-specific replacement for
 * the canonical semantic/IR architecture.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS
 * --------------
 *
 *     - agent construct boundary;
 *     - agent annotation structure;
 *     - agent construct tails;
 *     - agent references;
 *     - agent invocation structure;
 *     - agent type clauses;
 *     - agent initializer clauses;
 *     - agent bodies;
 *     - agent nested composition;
 *     - agent-local member composition;
 *     - agent expression/type bridges.
 *
 * THIS FILE DOES NOT OWN
 * ----------------------
 *
 *     - lexical token definitions;
 *     - annotation tokenization;
 *     - identifiers;
 *     - general names;
 *     - general paths;
 *     - expression precedence;
 *     - general expressions;
 *     - type-system implementation;
 *     - general statements;
 *     - functions;
 *     - modules;
 *     - tensors;
 *     - model internals;
 *     - datasets;
 *     - training;
 *     - inference;
 *     - differentiation;
 *     - networking;
 *     - distributed topology;
 *     - hardware topology;
 *     - accelerator selection;
 *     - quantum operations;
 *     - quantum topology;
 *     - QEC;
 *     - ZQN;
 *     - routing;
 *     - scheduling;
 *     - calibration;
 *     - HAL;
 *     - target selection;
 *     - resource discovery;
 *     - runtime execution;
 *     - IR construction.
 *
 * ============================================================================
 * CANONICAL DEPENDENCIES
 * ============================================================================
 *
 * Annotation marker:
 *
 *     grammar/lexer/annotations.g4
 *
 *     AT
 *
 * Identifier:
 *
 *     canonical identifier grammar
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
 * This grammar MUST consume those canonical contracts.
 *
 * ============================================================================
 * CRITICAL LEXICAL CORRECTION
 * ============================================================================
 *
 * Agent grammar MUST use:
 *
 *     AT identifier
 *
 * and MUST NOT use:
 *
 *     NANO_ANNOTATION
 *
 * NANO_ANNOTATION is a legacy/inconsistent representation found in older
 * AI grammar material.
 *
 * The canonical lexical architecture owns:
 *
 *     AT
 *
 * while the parser owns:
 *
 *     annotation name
 *
 * This keeps agent annotations extensible and prevents agent semantics from
 * becoming lexer keywords.
 *
 * ============================================================================
 * OPEN-WORLD PRINCIPLE
 * ============================================================================
 *
 * Agent annotation names are identifiers.
 *
 * This grammar therefore does NOT contain:
 *
 *     AGENT
 *     GOAL
 *     MODEL
 *     TOOL
 *     MEMORY
 *     POLICY
 *     PLAN
 *     DELEGATE
 *     OBSERVE
 *     ACTION
 *     MESSAGE
 *     ...
 *
 * as mandatory parser/lexer vocabulary.
 *
 * The semantic layer owns the registry and meaning of those concepts.
 *
 * This permits future extensions without changing the lexical foundation.
 *
 * ============================================================================
 * STRUCTURAL FORMS
 * ============================================================================
 *
 * The grammar supports structurally distinct forms.
 *
 * Annotation invocation:
 *
 *     @requires(qubits >= n);
 *     @capability(capability("quantum.measurement"));
 *     @message(recipient, payload);
 *     @observe(sensor(input)) {
 *         ...
 *     }
 *
 * Named construct:
 *
 *     @agent Researcher {
 *         ...
 *     }
 *
 *     @goal solve;
 *
 *     @model reasoning_model;
 *
 *     @tool search;
 *
 * Named invocation:
 *
 *     @goal solve(problem);
 *
 *     @tool search(query);
 *
 * Named typed construct:
 *
 *     @memory state: MemoryType;
 *
 * Named binding:
 *
 *     @model model = trained_model;
 *
 * Direct expression binding:
 *
 *     @requires = requirement_expression;
 *
 * Empty directive:
 *
 *     @checkpoint;
 *
 * Nested body:
 *
 *     @plan execution {
 *         ...
 *     }
 *
 * ============================================================================
 * WHY THE STRUCTURAL DISPATCH MATTERS
 * ============================================================================
 *
 * After:
 *
 *     AT identifier
 *
 * the next token determines the structural form:
 *
 *     LPAREN     -> annotation invocation
 *     identifier -> named construct
 *     ASSIGN     -> expression binding
 *     LBRACE     -> body
 *     SEMICOLON  -> empty construct
 *
 * This keeps the grammar deterministic without requiring semantic predicates
 * or a closed vocabulary of agent keywords.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Agent syntax describes PORTABLE COMPUTATIONAL INTENT.
 *
 * It MUST NOT encode universal assumptions about:
 *
 *     CPUs
 *     cores
 *     threads
 *     GPUs
 *     FPGAs
 *     ASICs
 *     QPUs
 *     qubits
 *     nodes
 *     devices
 *     accelerators
 *     memory
 *     storage
 *     network topology
 *     worker counts
 *     process counts
 *     tensor dimensions
 *     tensor rank
 *     model size
 *     agent count
 *     goal count
 *     tool count
 *     plan count
 *     step count
 *
 * The grammar therefore contains NO:
 *
 *     MAX_AGENTS
 *     MAX_GOALS
 *     MAX_TOOLS
 *     MAX_MODELS
 *     MAX_WORKERS
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_QUBITS
 *     MAX_MEMORY
 *     MAX_TENSOR_RANK
 *
 * or equivalent limits.
 *
 * ============================================================================
 * RESOURCE SEMANTICS
 * ============================================================================
 *
 * Portable requirements belong to semantic/resource analysis.
 *
 * Examples include:
 *
 *     @requires(qubits >= n);
 *
 *     @requires(memory >= required_memory);
 *
 *     @requires(capability("quantum.measurement"));
 *
 *     @requires(capability("gpu.compute"));
 *
 *     @requires(topology(required_topology));
 *
 * Such expressions describe requirements.
 *
 * They do NOT select:
 *
 *     QPU 0
 *     GPU 0
 *     CPU core 7
 *     physical qubit 17
 *     node 3
 *
 * Physical realization remains downstream.
 *
 * ============================================================================
 * CLASSICAL / QUANTUM / HDL / DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Agent bodies reuse:
 *
 *     statement
 *
 * from Statements.
 *
 * Therefore agent bodies can contain ordinary Zamani computation and can
 * participate semantically in:
 *
 *     classical computation;
 *     quantum computation;
 *     hybrid computation;
 *     HDL/hardware interaction;
 *     AI/ML;
 *     tensor computation;
 *     distributed execution;
 *     networking;
 *     secure computation;
 *     accelerator computation;
 *     future computational domains.
 *
 * This grammar does not duplicate any of those domains.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Agent.g4 does not define quantum syntax.
 *
 * Quantum constructs remain owned by:
 *
 *     grammar/quantum/
 *
 * and eventually cross the canonical:
 *
 *     quantum::ir
 *
 * boundary.
 *
 * Agent semantics may reference quantum capabilities and requirements, but
 * this grammar does not define:
 *
 *     QubitId
 *     physical qubits
 *     gates
 *     topology
 *     routing
 *     scheduling
 *     QEC
 *     ZQN
 *     calibration.
 *
 * ============================================================================
 * AI DOMAIN INTEGRATION
 * ============================================================================
 *
 * Agent.g4 may reference semantic objects owned by:
 *
 *     models.g4
 *     tensors.g4
 *     datasets.g4
 *     training.g4
 *     inference.g4
 *     differentiation.g4
 *     pipelines.g4
 *     ai-accelerators.g4
 *
 * through canonical expressions and types.
 *
 * Agent.g4 does not reimplement those grammars.
 *
 * ============================================================================
 * MODEL / TOOL / MEMORY INTEGRATION
 * ============================================================================
 *
 * Example:
 *
 *     @model planner;
 *
 *     @tool search;
 *
 *     @memory state: Memory<State, required_memory>;
 *
 * The parser records the structure.
 *
 * Semantic analysis resolves:
 *
 *     model references;
 *     tool references;
 *     memory references;
 *     types;
 *     capabilities;
 *     resource requirements;
 *     effects;
 *     security constraints.
 *
 * ============================================================================
 * PLAN / SCHEDULING SEPARATION
 * ============================================================================
 *
 * A source-level plan describes logical intent.
 *
 * It does NOT assign:
 *
 *     processors;
 *     workers;
 *     GPUs;
 *     QPUs;
 *     nodes;
 *     network routes;
 *     physical qubits;
 *     memory banks;
 *     clock slots.
 *
 * Scheduling and placement remain downstream.
 *
 * ============================================================================
 * CHECKPOINT SEMANTICS
 * ============================================================================
 *
 * A checkpoint construct describes an intended checkpoint boundary.
 *
 * It does NOT guarantee that arbitrary state is serializable.
 *
 * For example, quantum state handling may require:
 *
 *     measurement;
 *     logical-state support;
 *     reconstruction;
 *     provider support;
 *     QEC-aware handling.
 *
 * The semantic/runtime layers decide whether the requested checkpoint is
 * valid.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend AST MUST remain domain-neutral.
 *
 * The parser must preserve enough information for semantic analysis to see:
 *
 *     - source span;
 *     - annotation name;
 *     - optional annotation arguments;
 *     - optional named subject;
 *     - optional subject invocation;
 *     - optional type;
 *     - optional initializer;
 *     - optional body;
 *     - member ordering;
 *     - nested structure;
 *     - ordinary statements.
 *
 * The AST MUST NOT introduce target-specific nodes such as:
 *
 *     AgentCpuNode
 *     AgentGpuNode
 *     AgentQpuNode
 *     AgentCudaNode
 *     AgentPhysicalDeviceNode
 *
 * Nor should this grammar require:
 *
 *     AgentIR
 *
 * as a second intermediate representation.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis owns:
 *
 *     - annotation registration;
 *     - annotation versioning;
 *     - agent declaration validation;
 *     - reference resolution;
 *     - scope;
 *     - type checking;
 *     - expression checking;
 *     - lifecycle rules;
 *     - goal/objective validation;
 *     - model resolution;
 *     - tool resolution;
 *     - memory resolution;
 *     - policy validation;
 *     - plan validation;
 *     - delegation validation;
 *     - coordination validation;
 *     - message validation;
 *     - event validation;
 *     - checkpoint validation;
 *     - termination validation;
 *     - resource requirements;
 *     - capability requirements;
 *     - constraints;
 *     - preferences;
 *     - effects;
 *     - security;
 *     - portability;
 *     - domain interoperability.
 *
 * The parser performs none of these semantic decisions.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     - source token sequence;
 *     - grammar version.
 *
 * It MUST NOT depend on:
 *
 *     - wall-clock time;
 *     - randomness;
 *     - filesystem state;
 *     - network state;
 *     - hardware state;
 *     - runtime state;
 *     - available devices;
 *     - resource availability.
 *
 * ============================================================================
 * SAFETY
 * ============================================================================
 *
 * This is a pure parser grammar.
 *
 * It contains:
 *
 *     - no embedded Rust;
 *     - no actions;
 *     - no semantic predicates;
 *     - no unsafe code;
 *     - no filesystem operations;
 *     - no network operations;
 *     - no hardware discovery;
 *     - no runtime execution.
 *
 * The generated Rust parser/frontend must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * under the repository's safe-Rust policy.
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
 * Exactly one canonical public entry point is owned by this grammar.
 */

aiAgentConstruct
    : aiAgentAnnotatedConstruct
    ;


/*
 * ============================================================================
 * 2. GENERIC AGENT CONSTRUCT
 * ============================================================================
 *
 * The annotation name is intentionally open-world.
 */

aiAgentAnnotatedConstruct
    : aiAgentAnnotation
      aiAgentTail
    ;


/*
 * ============================================================================
 * 3. ANNOTATION
 * ============================================================================
 *
 * Canonical lexical structure:
 *
 *     AT identifier
 *
 * Example:
 *
 *     @agent
 *     @goal
 *     @requires
 *     @future_extension
 */

aiAgentAnnotation
    : AT
      identifier
    ;


/*
 * ============================================================================
 * 4. TOP-LEVEL CONSTRUCT TAIL
 * ============================================================================
 *
 * Structural first-token dispatch:
 *
 *     '(' -> direct annotation invocation
 *     identifier -> named construct
 *     '=' -> expression binding
 *     '{' -> body
 *     ';' -> empty construct
 *
 * This deliberately avoids a closed agent vocabulary.
 */

aiAgentTail
    : aiAgentDirectInvocation
    | aiAgentNamedConstruct
    | aiAgentExpressionBinding
    | aiAgentBody
    | SEMICOLON
    ;


/*
 * ============================================================================
 * 5. DIRECT ANNOTATION INVOCATION
 * ============================================================================
 *
 * Examples:
 *
 *     @requires(qubits >= n);
 *
 *     @requires(memory >= required_memory);
 *
 *     @requires(capability("gpu.compute"));
 *
 *     @requires(capability("quantum.measurement"));
 *
 *     @message(recipient, payload);
 *
 *     @observe(sensor(input)) {
 *         ...
 *     }
 *
 * The meaning of the annotation belongs to semantic analysis.
 */

aiAgentDirectInvocation
    : LPAREN
      argumentList?
      RPAREN
      aiAgentPostInvocation
    ;


aiAgentPostInvocation
    : aiAgentBody
    | SEMICOLON
    ;


/*
 * ============================================================================
 * 6. NAMED CONSTRUCT
 * ============================================================================
 *
 * Examples:
 *
 *     @agent Researcher { ... }
 *
 *     @goal solve;
 *
 *     @model planner;
 *
 *     @tool search;
 *
 *     @memory state: MemoryType;
 *
 *     @model planner = trained_model;
 *
 * The second identifier is a semantic subject/name, not a hard-coded agent
 * keyword.
 */

aiAgentNamedConstruct
    : identifier
      aiAgentNamedTail
    ;


/*
 * ============================================================================
 * 7. NAMED CONSTRUCT TAIL
 * ============================================================================
 *
 * A named construct may be:
 *
 *     invocation;
 *     typed declaration;
 *     binding;
 *     nested body;
 *     simple reference.
 *
 * The structural alternatives are determined by punctuation.
 */

aiAgentNamedTail
    : aiAgentNamedInvocation
    | aiAgentNamedDeclaration
    ;


aiAgentNamedInvocation
    : LPAREN
      argumentList?
      RPAREN
      aiAgentPostInvocation
    ;


aiAgentNamedDeclaration
    : aiAgentTypeClause?
      aiAgentInitializer?
      aiAgentOptionalBody
    ;


aiAgentOptionalBody
    : aiAgentBody
    | SEMICOLON
    ;


/*
 * ============================================================================
 * 8. EXPRESSION BINDING
 * ============================================================================
 *
 * Example:
 *
 *     @model = trained_model;
 *
 *     @configuration = configuration_expression;
 *
 * The semantic layer determines whether the annotation permits this form.
 */

aiAgentExpressionBinding
    : ASSIGN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 9. TYPE CLAUSE
 * ============================================================================
 *
 * Type syntax remains owned by Types.
 */

aiAgentTypeClause
    : COLON
      typeExpression
    ;


/*
 * ============================================================================
 * 10. INITIALIZER
 * ============================================================================
 *
 * Expression syntax remains owned by Expressions.
 */

aiAgentInitializer
    : ASSIGN
      expression
    ;


/*
 * ============================================================================
 * 11. AGENT BODY
 * ============================================================================
 *
 * Agent bodies contain an unbounded structural sequence of:
 *
 *     - nested agent constructs;
 *     - ordinary Zamani statements.
 *
 * No agent-specific mini-language is introduced.
 */

aiAgentBody
    : LBRACE
      aiAgentMember*
      RBRACE
    ;


/*
 * ============================================================================
 * 12. AGENT MEMBER
 * ============================================================================
 *
 * An agent member is either:
 *
 *     - an agent-specific annotated construct;
 *     - an ordinary Zamani statement.
 *
 * Statements remain owned by Statements.
 */

aiAgentMember
    : aiAgentAnnotatedConstruct
    | statement
    ;


/*
 * ============================================================================
 * 13. CONTENTS BRIDGE
 * ============================================================================
 *
 * Stable helper for parser visitors and conformance tooling.
 */

aiAgentContents
    : aiAgentMember*
    ;


/*
 * ============================================================================
 * 14. EXPRESSION BRIDGE
 * ============================================================================
 *
 * This does not create an agent expression language.
 */

aiAgentExpression
    : expression
    ;


/*
 * ============================================================================
 * 15. TYPE BRIDGE
 * ============================================================================
 *
 * This does not create an agent type system.
 */

aiAgentType
    : typeExpression
    ;


/*
 * ============================================================================
 * 16. REFERENCE
 * ============================================================================
 *
 * A basic agent reference uses the canonical identifier grammar.
 *
 * More complex references should use the canonical expression/path machinery
 * rather than introducing a second name-resolution system here.
 */

aiAgentReference
    : identifier
    ;


/*
 * ============================================================================
 * 17. CALL BRIDGE
 * ============================================================================
 *
 * Stable structural helper for semantic tooling.
 */

aiAgentCall
    : identifier
      LPAREN
      argumentList?
      RPAREN
    ;


/*
 * ============================================================================
 * 18. SEMANTIC ROLE BRIDGES
 * ============================================================================
 *
 * These rules deliberately alias the SAME canonical syntax.
 *
 * They are integration points for visitors and semantic analysis.
 *
 * They do NOT create separate syntactic definitions for each agent role.
 *
 * The annotation name determines the semantic role.
 */


/* Agent */

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


/* Model */

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
 * 19. INTEGRATION CONTRACT
 * ============================================================================
 *
 * ai.g4
 * -----
 *
 * The canonical AI composition grammar should expose this grammar through
 * its AI-agent boundary.
 *
 * Preferred composition:
 *
 *     AI
 *       |
 *       +--> aiAgentConstruct
 *
 * AI.g4 remains the AI-domain composition boundary.
 *
 * It must not duplicate the agent syntax.
 *
 *
 * agents.g4
 * ---------
 *
 * The existing legacy:
 *
 *     grammar/ai/agents.g4
 *
 * must NOT become a second implementation of agent syntax.
 *
 * It should be converted into a compatibility facade that delegates its
 * public agent boundary to:
 *
 *     AIAgent.aiAgentConstruct
 *
 * Existing consumers should therefore be migrated toward:
 *
 *     aiAgentConstruct
 *
 * without unnecessarily renaming the existing agents.g4 file.
 *
 *
 * ZamaniParser.g4
 * ---------------
 *
 * The universal parser should consume the AI composition boundary through:
 *
 *     AI
 *
 * rather than importing this leaf grammar directly.
 *
 * This preserves:
 *
 *     leaf
 *       ->
 *     domain dispatcher
 *       ->
 *     universal dispatcher
 *       ->
 *     ZamaniParser
 *
 * and prevents parallel composition paths.
 *
 *
 * Zamani.g4
 * ---------
 *
 * The root grammar remains the visible language composition root.
 *
 * No agent-specific rules should be copied into Zamani.g4.
 *
 *
 * Lexer
 * -----
 *
 * No new agent lexer tokens are required.
 *
 * Agent annotations use:
 *
 *     AT
 *     identifier
 *
 * This preserves the open-world architecture.
 *
 *
 * Types
 * -----
 *
 * Type expressions are delegated to:
 *
 *     typeExpression
 *
 * No AgentType grammar is introduced here.
 *
 *
 * Expressions
 * -----------
 *
 * Expressions and argument lists are delegated to:
 *
 *     expression
 *     argumentList
 *
 * No AgentExpression language is introduced here.
 *
 *
 * Statements
 * ----------
 *
 * Agent bodies delegate ordinary statements to:
 *
 *     statement
 *
 * Therefore the agent body does not become a second programming language.
 *
 * ============================================================================
 * 20. AST INTEGRATION CONTRACT
 * ============================================================================
 *
 * The parser should map structurally to the existing domain-neutral frontend
 * representation.
 *
 * Minimum information that must survive parsing:
 *
 *     annotation name
 *     annotation source span
 *     optional direct arguments
 *     optional named subject
 *     optional named invocation arguments
 *     optional type
 *     optional initializer
 *     optional body
 *     ordered members
 *     nested constructs
 *     ordinary statements
 *     source provenance
 *
 * If the existing AST has a generic:
 *
 *     attribute / annotation / annotated construct
 *
 * representation, this grammar MUST reuse it.
 *
 * Do NOT create:
 *
 *     AgentCpuNode
 *     AgentGpuNode
 *     AgentQpuNode
 *     AgentCudaNode
 *     AgentTensorFlowNode
 *     AgentPhysicalDeviceNode
 *
 * merely because those concepts may occur semantically.
 *
 * ============================================================================
 * 21. SEMANTIC INTEGRATION CONTRACT
 * ============================================================================
 *
 * Semantic analysis interprets the annotation name.
 *
 * Example:
 *
 *     @agent Researcher { ... }
 *
 * can be classified as an agent declaration.
 *
 *     @goal solve(problem);
 *
 * can be classified as a goal invocation.
 *
 *     @requires(qubits >= n);
 *
 * can be classified as a resource requirement.
 *
 *     @requires(capability("quantum.measurement"));
 *
 * can be classified as a capability requirement.
 *
 *     @model planner;
 *
 * can be resolved against the model subsystem.
 *
 *     @tool search;
 *
 * can be resolved against the tool subsystem.
 *
 * None of these classifications are parser responsibilities.
 *
 * ============================================================================
 * 22. RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Agent grammar may carry expressions representing:
 *
 *     requirements;
 *     capabilities;
 *     constraints;
 *     preferences;
 *     hints;
 *     budgets.
 *
 * Their semantics are resolved downstream.
 *
 * The grammar does NOT perform:
 *
 *     hardware discovery;
 *     resource allocation;
 *     placement;
 *     scheduling;
 *     device selection;
 *     topology discovery.
 *
 * ============================================================================
 * 23. QUANTUM INTEGRATION CONTRACT
 * ============================================================================
 *
 * An agent may semantically participate in quantum computation.
 *
 * Examples:
 *
 *     @requires(capability("quantum.measurement"));
 *
 *     @requires(qubits >= required_qubits);
 *
 *     @action measure(state);
 *
 * The quantum implementation remains responsible for:
 *
 *     quantum semantics
 *     quantum::ir
 *     decomposition
 *     routing
 *     scheduling
 *     QEC
 *     resilience
 *     ZQN
 *     HAL
 *
 * This grammar does not duplicate any of them.
 *
 * ============================================================================
 * 24. CLASSICAL / HDL / DISTRIBUTED CONTRACT
 * ============================================================================
 *
 * Agent bodies may contain ordinary statements.
 *
 * Therefore they may semantically compose with:
 *
 *     classical;
 *     quantum;
 *     hybrid;
 *     HDL;
 *     hardware;
 *     distributed;
 *     networking;
 *     security;
 *     data;
 *     concurrency;
 *     execution.
 *
 * Domain-specific meaning remains owned by the corresponding domain grammar
 * and semantic layer.
 *
 * ============================================================================
 * 25. SCALABILITY CONTRACT
 * ============================================================================
 *
 * There is NO grammar-level maximum for:
 *
 *     agents;
 *     nested agents;
 *     goals;
 *     objectives;
 *     models;
 *     tools;
 *     memories;
 *     observations;
 *     actions;
 *     policies;
 *     plans;
 *     steps;
 *     delegations;
 *     messages;
 *     events;
 *     checkpoints;
 *     requirements;
 *     capabilities;
 *     resources;
 *     workers;
 *     nodes;
 *     devices;
 *     accelerators;
 *     tensors;
 *     quantum resources;
 *     classical resources.
 *
 * Repetition is represented structurally through:
 *
 *     *
 *
 * Actual compiler/runtime resource exhaustion remains an implementation
 * concern, not a language semantic limit.
 *
 * ============================================================================
 * 26. HARD-CODING AUDIT
 * ============================================================================
 *
 * This file MUST remain free of:
 *
 *     MAX_AGENTS
 *     MAX_GOALS
 *     MAX_OBJECTIVES
 *     MAX_MODELS
 *     MAX_TOOLS
 *     MAX_MEMORIES
 *     MAX_PLANS
 *     MAX_STEPS
 *     MAX_WORKERS
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_ACCELERATORS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_QUBITS
 *     MAX_MEMORY
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *     MAX_NETWORK_SIZE
 *
 * It must also not hard-code:
 *
 *     CPU 0
 *     GPU 0
 *     QPU 0
 *     FPGA 0
 *     physical qubit 0
 *     physical device 0
 *     vendor device identifiers
 *     fixed topology.
 *
 * ============================================================================
 * 27. DETERMINISM CONTRACT
 * ============================================================================
 *
 * The same:
 *
 *     source
 *     +
 *     grammar version
 *     +
 *     lexical version
 *
 * must produce the same parse structure.
 *
 * Parsing must not inspect:
 *
 *     hardware;
 *     runtime;
 *     network;
 *     filesystem;
 *     environment;
 *     current time;
 *     random state;
 *     device availability.
 *
 * ============================================================================
 * 28. DIAGNOSTICS CONTRACT
 * ============================================================================
 *
 * The parser should report structural errors at the narrowest available
 * construct boundary.
 *
 * Examples:
 *
 *     @agent
 *
 *     -> missing construct tail
 *
 *     @agent Name {
 *
 *     -> missing closing brace
 *
 *     @goal solve(
 *
 *     -> missing closing parenthesis
 *
 *     @model Name:
 *
 *     -> incomplete type clause
 *
 *     @model Name =
 *
 *     -> missing expression
 *
 *     @requires(
 *
 *     -> incomplete argument list
 *
 * Semantic errors such as unknown model names or unsupported capabilities
 * belong to semantic analysis, not parser diagnostics.
 *
 * ============================================================================
 * 29. SECURITY CONTRACT
 * ============================================================================
 *
 * Agent grammar MUST NOT execute:
 *
 *     tools;
 *     network requests;
 *     shell commands;
 *     model inference;
 *     external programs;
 *     hardware operations;
 *     filesystem operations.
 *
 * Names and arguments are source syntax only.
 *
 * Authorization, capability checks and security policy belong downstream.
 *
 * ============================================================================
 * 30. PERFORMANCE CONTRACT
 * ============================================================================
 *
 * The grammar must:
 *
 *     - avoid semantic predicates;
 *     - avoid target-language actions;
 *     - avoid runtime lookups;
 *     - avoid hardware-dependent parsing;
 *     - reuse canonical expression/type/statement rules;
 *     - avoid duplicated domain grammars;
 *     - preserve deterministic structural dispatch.
 *
 * Parser performance must scale with actual source complexity rather than
 * artificial agent-count limits.
 *
 * ============================================================================
 * 31. TEST CONTRACT
 * ============================================================================
 *
 * Positive tests MUST include:
 *
 *     @agent Researcher {
 *         @goal solve;
 *     }
 *
 *     @agent Researcher {
 *         @model planner;
 *         @tool search;
 *         @memory state: MemoryType;
 *     }
 *
 *     @goal solve(problem);
 *
 *     @requires(qubits >= n);
 *
 *     @requires(capability("quantum.measurement"));
 *
 *     @requires(capability("gpu.compute"));
 *
 *     @message(recipient, payload);
 *
 *     @observe(sensor(input)) {
 *         ...
 *     }
 *
 *     @model planner = trained_model;
 *
 *     @configuration = configuration_expression;
 *
 *     @plan execution {
 *         @step first;
 *         @step second;
 *     }
 *
 *     nested agent constructs;
 *
 *     ordinary Zamani statements inside agent bodies.
 *
 * Negative tests MUST include:
 *
 *     missing annotation name;
 *     missing construct tail;
 *     missing identifier;
 *     malformed invocation;
 *     missing RPAREN;
 *     missing RBRACE;
 *     malformed type clause;
 *     missing initializer expression;
 *     missing semicolon;
 *     malformed argument list.
 *
 * Boundary tests MUST include:
 *
 *     empty agent body;
 *     empty invocation;
 *     deeply nested valid constructs subject only to implementation resources;
 *     large argument lists;
 *     large member sequences;
 *     large nested structures.
 *
 * Scalability tests MUST verify that no source-level maximum exists for:
 *
 *     agents;
 *     goals;
 *     tools;
 *     plans;
 *     steps;
 *     resources;
 *     nodes;
 *     devices.
 *
 * Determinism tests MUST parse identical input repeatedly and verify identical
 * parse structure.
 *
 * Compatibility tests MUST verify:
 *
 *     AT-based annotations;
 *     canonical identifier syntax;
 *     canonical type syntax;
 *     canonical expression syntax;
 *     canonical statement syntax;
 *     compatibility facade in agents.g4.
 *
 * ============================================================================
 * 32. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [ ] It is the canonical agent leaf grammar.
 *
 *     [ ] It uses AT rather than NANO_ANNOTATION.
 *
 *     [ ] It imports Types.
 *
 *     [ ] It imports Expressions.
 *
 *     [ ] It imports Statements.
 *
 *     [ ] It does not redefine identifier syntax.
 *
 *     [ ] It does not redefine expression syntax.
 *
 *     [ ] It does not redefine type syntax.
 *
 *     [ ] It does not redefine statement syntax.
 *
 *     [ ] It has one canonical public agent entry point.
 *
 *     [ ] Agent annotation names remain open-world.
 *
 *     [ ] Direct annotation invocation is supported.
 *
 *     [ ] Named agent constructs are supported.
 *
 *     [ ] Named invocation is supported.
 *
 *     [ ] Type clauses are supported.
 *
 *     [ ] Initializers are supported.
 *
 *     [ ] Nested bodies are supported.
 *
 *     [ ] Nested agent constructs are supported.
 *
 *     [ ] Ordinary statements are supported inside bodies.
 *
 *     [ ] Resource requirements can be represented.
 *
 *     [ ] Capability requirements can be represented.
 *
 *     [ ] Quantum requirements can be represented.
 *
 *     [ ] Classical computation can be represented through statements.
 *
 *     [ ] HDL/hardware interaction can be represented through statements and
 *         semantic constructs.
 *
 *     [ ] Distributed computation can be represented through shared language
 *         constructs.
 *
 *     [ ] No AI framework is hard-coded.
 *
 *     [ ] No vendor is hard-coded.
 *
 *     [ ] No hardware target is hard-coded.
 *
 *     [ ] No physical device is selected by grammar.
 *
 *     [ ] No universal capacity is hard-coded.
 *
 *     [ ] No MAX_* resource constant exists.
 *
 *     [ ] No second AI IR exists.
 *
 *     [ ] No second agent IR exists.
 *
 *     [ ] quantum::ir remains the canonical quantum boundary.
 *
 *     [ ] Parsing is deterministic.
 *
 *     [ ] Parsing contains no executable actions.
 *
 *     [ ] Parsing contains no semantic predicates.
 *
 *     [ ] Rust implementation remains safe Rust.
 *
 *     [ ] Rust 1.97 / 1.97.1 compatibility remains an implementation
 *         requirement.
 *
 *     [ ] Positive tests exist.
 *
 *     [ ] Negative tests exist.
 *
 *     [ ] Boundary tests exist.
 *
 *     [ ] Scalability tests exist.
 *
 *     [ ] Determinism tests exist.
 *
 *     [ ] Compatibility tests exist.
 *
 * ============================================================================
 */
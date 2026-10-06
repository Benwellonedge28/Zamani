/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/ai/agents.g4
 *
 * GRAMMAR
 * -------
 * Agents
 *
 * STATUS
 * ------
 * CANONICAL AI / AGENT SOURCE-GRAMMAR BOUNDARY
 *
 * LANGUAGE BASELINE
 * -----------------
 * Rust 1.97 / Rust 1.97.1
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
 * This file is the canonical source-level grammar for portable agentic
 * computation in Zamani.
 *
 * It describes agent structure and intent while deliberately leaving physical
 * realization, execution, scheduling, placement, networking, resource
 * allocation, model execution, quantum execution, and hardware realization
 * to their respective downstream subsystems.
 *
 * The grammar supports an OPEN-WORLD agent model.
 *
 * The language therefore does NOT require a permanently growing list of
 * agent-specific lexer keywords.
 *
 * Instead, agent concepts are structurally represented through:
 *
 *     @ identifier
 *
 * Examples:
 *
 *     @agent Researcher {
 *         @goal solve_problem;
 *     }
 *
 *     @model planner;
 *
 *     @tool search;
 *
 *     @memory state;
 *
 *     @goal solve(problem);
 *
 *     @observe sensor(input);
 *
 *     @act operation(value);
 *
 *     @delegate worker(task);
 *
 *     @coordinate group;
 *
 *     @message(recipient, payload);
 *
 *     @requires(capability("tensor.compute"));
 *
 *     @requires(qubits >= required_qubits);
 *
 *     @policy execution_policy {
 *         ...
 *     }
 *
 * These names are semantic identifiers, not a closed parser vocabulary.
 *
 *
 * ============================================================================
 * OWNS
 * ============================================================================
 *
 * This file owns:
 *
 *     agentConstruct
 *     agentAnnotation
 *     agentTail
 *     agentInvocation
 *     agentNamedConstruct
 *     agentNamedConstructTail
 *     agentBinding
 *     agentBody
 *     agentMember
 *     agentNestedConstruct
 *     agentTypeClause
 *     agentInitializer
 *     agentReference
 *     agentSubject
 *     agentArguments
 *     agentExpression
 *     agentBlock
 *
 * The grammar also owns the structural distinction between:
 *
 *     annotation invocation
 *     named construct
 *     binding
 *     block construct
 *     empty construct
 *
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     lexical tokens
 *     identifiers
 *     qualified-name semantics
 *     general expressions
 *     expression precedence
 *     general types
 *     declarations
 *     ordinary statements
 *     actor semantics
 *     message transport
 *     networking
 *     distributed topology
 *     model internals
 *     datasets
 *     tensors
 *     training algorithms
 *     inference algorithms
 *     reasoning algorithms
 *     learning algorithms
 *     adaptation algorithms
 *     security enforcement
 *     policy evaluation
 *     resource discovery
 *     capability discovery
 *     hardware discovery
 *     target selection
 *     scheduling
 *     routing
 *     quantum operations
 *     quantum topology
 *     QEC
 *     ZQN
 *     HAL
 *     classical IR
 *     quantum::ir
 *     agent IR
 *     runtime execution
 *
 *
 * ============================================================================
 * DEPENDENCIES
 * ============================================================================
 *
 * Lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Required lexical vocabulary:
 *
 *     AT
 *     ASSIGN
 *     COLON
 *     COMMA
 *     LBRACE
 *     RBRACE
 *     LPAREN
 *     RPAREN
 *     SEMICOLON
 *
 * Canonical parser dependencies:
 *
 *     Types
 *     Expressions
 *     Statements
 *
 * The grammar intentionally does NOT import another agent grammar.
 *
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * This file is the canonical plural-agent composition boundary.
 *
 * The repository also contains:
 *
 *     grammar/ai/agent.g4
 *
 * That file MUST NOT define a second competing canonical agent grammar.
 *
 * The repository must ultimately establish exactly one of:
 *
 *     1. agents.g4 is canonical and agent.g4 becomes a compatibility façade;
 *
 * or:
 *
 *     2. agent.g4 is removed/renamed after repository-wide migration.
 *
 * For the current architecture, this file is treated as canonical because
 * grammar/ai/ai.g4 already imports `Agents` and expects:
 *
 *     agentConstruct
 *
 * ============================================================================
 * CRITICAL CORRECTION FROM LEGACY GRAMMAR
 * ============================================================================
 *
 * DO NOT USE:
 *
 *     NANO_ANNOTATION
 *
 * Agent annotations use the canonical lexical token:
 *
 *     AT
 *
 * followed by the canonical:
 *
 *     identifier
 *
 * Therefore:
 *
 *     @agent
 *
 * is parsed structurally as:
 *
 *     AT identifier
 *
 * The semantic layer determines whether `agent` is a recognized annotation
 * role in the current language/specification version.
 *
 *
 * ============================================================================
 * OPEN-WORLD EXTENSIBILITY
 * ============================================================================
 *
 * This grammar intentionally does NOT contain rules such as:
 *
 *     AGENT
 *     GOAL
 *     MODEL
 *     TOOL
 *     MEMORY
 *     PLAN
 *     MESSAGE
 *     DELEGATE
 *     OBSERVE
 *     ACTION
 *
 * as lexer/parser keywords.
 *
 * Doing so would make every new agent concept require grammar and lexer
 * modification.
 *
 * Instead:
 *
 *     @identifier
 *
 * is the stable syntactic mechanism.
 *
 * Semantic registries can define roles such as:
 *
 *     agent
 *     goal
 *     objective
 *     model
 *     tool
 *     memory
 *     observation
 *     action
 *     plan
 *     step
 *     delegate
 *     coordinate
 *     message
 *     event
 *     checkpoint
 *     termination
 *     requires
 *     capability
 *     constraint
 *     preference
 *     policy
 *     explain
 *     evidence
 *     provenance
 *     learn
 *     adapt
 *     reason
 *
 * without expanding the core lexical vocabulary.
 *
 *
 * ============================================================================
 * STRUCTURAL MODEL
 * ============================================================================
 *
 * The grammar recognizes the following generic forms:
 *
 *     @name;
 *
 *     @name(expression);
 *
 *     @name subject;
 *
 *     @name subject(expression);
 *
 *     @name subject = expression;
 *
 *     @name subject: Type;
 *
 *     @name subject: Type = expression;
 *
 *     @name subject {
 *         ...
 *     }
 *
 *     @name subject: Type {
 *         ...
 *     }
 *
 *     @name subject = expression {
 *         ...
 *     }
 *
 * This gives the semantic layer enough structure to represent future agent
 * features without turning this grammar into a catalog of application-level
 * concepts.
 *
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser contexts are consumed by the existing domain-neutral frontend
 * AST.
 *
 * The resulting AST representation should preserve at minimum:
 *
 *     source span
 *     annotation name
 *     subject/name
 *     optional type
 *     optional arguments
 *     optional initializer
 *     optional body
 *     member order
 *     nesting
 *
 * The AST MUST NOT contain:
 *
 *     AgentCPU
 *     AgentGPU
 *     AgentQPU
 *     AgentNode
 *     AgentDevice
 *     AgentThread
 *     AgentPhysicalResource
 *     AgentCUDA
 *     AgentVendor
 *
 * or equivalent target-specific structures.
 *
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis determines:
 *
 *     whether an annotation is known;
 *     which version of the annotation applies;
 *     whether a construct is valid in its context;
 *     whether names resolve;
 *     whether types are valid;
 *     whether expressions are valid;
 *     whether effects are permitted;
 *     whether capabilities are available;
 *     whether resources satisfy requirements;
 *     whether policies authorize execution;
 *     whether delegation is valid;
 *     whether coordination is valid;
 *     whether messages are valid;
 *     whether checkpoints are meaningful;
 *     whether adaptation is authorized;
 *     whether model/tool/memory references resolve;
 *     whether the resulting computation is portable.
 *
 * None of these decisions occur in this grammar.
 *
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * Type syntax is owned by:
 *
 *     Types
 *
 * Agent-specific constructs therefore consume:
 *
 *     typeExpression
 *
 * rather than defining:
 *
 *     AgentType
 *     ModelType
 *     ToolType
 *     MemoryType
 *
 * as competing type systems.
 *
 *
 * ============================================================================
 * EXPRESSION CONTRACT
 * ============================================================================
 *
 * All agent arguments, initializers, goals, conditions, policies, resource
 * expressions, model references, tool references and similar values use the
 * canonical:
 *
 *     expression
 *
 * rule.
 *
 * This ensures:
 *
 *     classical expressions
 *     tensor expressions
 *     quantum values
 *     hybrid expressions
 *     data queries
 *     symbolic expressions
 *     probabilistic expressions
 *     future expression domains
 *
 * remain part of one language.
 *
 *
 * ============================================================================
 * STATEMENT CONTRACT
 * ============================================================================
 *
 * Agent bodies may contain ordinary Zamani statements.
 *
 * This allows an agent to compose with:
 *
 *     classical computation
 *     quantum computation
 *     hybrid computation
 *     HDL/hardware intent
 *     AI/ML
 *     concurrency
 *     distributed execution
 *     networking
 *     security
 *     data processing
 *     interoperability
 *     metaprogramming
 *     future domains
 *
 * This grammar does not duplicate those statement systems.
 *
 *
 * ============================================================================
 * ACTOR / CONCURRENCY INTEGRATION
 * ============================================================================
 *
 * Agents are NOT a second actor system.
 *
 * An agent can semantically be realized through:
 *
 *     actor execution
 *     asynchronous execution
 *     local task execution
 *     distributed execution
 *     service execution
 *     accelerator-backed execution
 *     future execution substrates
 *
 * Actor syntax and lifecycle remain owned by:
 *
 *     grammar/concurrency/
 *
 * Message semantics remain owned by:
 *
 *     grammar/networking/
 *     grammar/distributed/
 *     grammar/concurrency/
 *
 * depending on the actual construct.
 *
 * This file expresses agent intent only.
 *
 *
 * ============================================================================
 * KNOWLEDGE / REASONING / LEARNING / ADAPTATION
 * ============================================================================
 *
 * The agent boundary is intentionally compatible with generic semantic
 * operations such as:
 *
 *     reason
 *     infer
 *     deduce
 *     learn
 *     adapt
 *     query
 *     assert
 *     retract
 *     explain
 *
 * These concepts MUST NOT become hard-coded agent-only parser branches.
 *
 * For example:
 *
 *     @reason {
 *         ...
 *     }
 *
 * is structurally accepted.
 *
 * The semantic layer determines whether `reason` is a valid registered
 * operation and which reasoning subsystem owns it.
 *
 * This permits the same reasoning machinery to be used outside agents.
 *
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Agent grammar does not define quantum operations.
 *
 * It may structurally contain expressions or statements whose semantics use
 * quantum computation.
 *
 * Quantum semantics eventually cross:
 *
 *     domain-neutral AST
 *          |
 *          v
 *     semantic quantum model
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     decomposition
 *          |
 *          v
 *     routing
 *          |
 *          v
 *     scheduling
 *          |
 *          v
 *     resilience / QEC
 *          |
 *          v
 *     ZQN
 *          |
 *          v
 *     HAL
 *
 * No physical qubit, topology, gate catalog, calibration data, or QPU is
 * encoded here.
 *
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Agents may semantically coordinate hardware-oriented computation, but this
 * grammar does not define:
 *
 *     wires
 *     fixed bus widths
 *     registers
 *     pins
 *     devices
 *     boards
 *     clock limits
 *     accelerator counts
 *     FPGA families
 *     ASIC implementations
 *
 * Hardware intent remains owned by the hardware/HDL grammars.
 *
 *
 * ============================================================================
 * RESOURCE / CAPABILITY CONTRACT
 * ============================================================================
 *
 * Agent source may express resource intent through generic expressions.
 *
 * Examples:
 *
 *     @requires(qubits >= required_qubits);
 *
 *     @requires(memory >= required_memory);
 *
 *     @requires(capability("quantum.measurement"));
 *
 *     @requires(capability("tensor.compute"));
 *
 *     @requires(topology(required_topology));
 *
 * This grammar does NOT decide whether those requirements are satisfied.
 *
 * Resource/capability analysis occurs downstream.
 *
 * A requirement does NOT mean:
 *
 *     GPU 0
 *     QPU 0
 *     CPU core 7
 *     physical qubit 17
 *     node 3
 *
 * Such physical allocation would violate the portability boundary.
 *
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Agent constructs can semantically participate in effects such as:
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
 * Effects are not defined here.
 *
 * The effect subsystem determines the actual effect set.
 *
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Agent constructs can be governed by policies covering:
 *
 *     authorization
 *     capability use
 *     resource use
 *     delegation
 *     adaptation
 *     networking
 *     foreign calls
 *     reflection
 *     simulation
 *     deployment
 *
 * Policy evaluation remains downstream.
 *
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Agent constructs must remain traceable to source.
 *
 * Downstream provenance may record:
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
 * This grammar does not create provenance records.
 *
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This file owns NO intermediate representation.
 *
 * There is:
 *
 *     no AgentIR
 *     no AgentBytecode
 *     no AgentMachineIR
 *
 * Agent semantics lower through the ordinary Zamani semantic/IR pipeline.
 *
 * If a construct contains quantum computation, the canonical quantum boundary
 * remains:
 *
 *     quantum::ir
 *
 * Classical computation uses the canonical classical path.
 *
 * Distributed computation uses the distributed semantic/lowering path.
 *
 * Hardware/HDL computation uses the corresponding domain paths.
 *
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Agent syntax is independent of machine scale.
 *
 * This grammar imposes NO maximum on:
 *
 *     agents
 *     nested agents
 *     goals
 *     objectives
 *     tools
 *     memories
 *     plans
 *     steps
 *     messages
 *     events
 *     model references
 *     workers
 *     tasks
 *     nodes
 *     devices
 *     CPUs
 *     GPUs
 *     FPGAs
 *     ASICs
 *     QPUs
 *     qubits
 *     threads
 *     memory
 *     tensor rank
 *     tensor dimensions
 *     network size
 *
 * There must be no:
 *
 *     MAX_AGENTS
 *     MAX_GOALS
 *     MAX_TOOLS
 *     MAX_WORKERS
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_MEMORY
 *
 * or equivalent language-level limits.
 *
 * Repetition is represented by grammar repetition and actual limits are
 * implementation/resource concerns.
 *
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     source text
 *     grammar version
 *     lexer vocabulary
 *
 * Parsing MUST NOT depend on:
 *
 *     hardware
 *     available resources
 *     network state
 *     filesystem state
 *     environment variables
 *     wall-clock time
 *     randomness
 *     runtime state
 *
 *
 * ============================================================================
 * SAFETY CONTRACT
 * ============================================================================
 *
 * This is a pure ANTLR parser grammar.
 *
 * It contains:
 *
 *     no embedded Rust;
 *     no actions;
 *     no semantic predicates;
 *     no filesystem operations;
 *     no network operations;
 *     no hardware discovery;
 *     no runtime execution.
 *
 * Generated frontend integration must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * under the repository's safe-Rust policy.
 *
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * UPSTREAM
 * --------
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/core/...
 *     grammar/types/...
 *     grammar/expressions/...
 *     grammar/statements/...
 *
 * COMPOSITION
 * -----------
 *
 *     grammar/ai/ai.g4
 *
 * imports:
 *
 *     Agents
 *
 * and exposes:
 *
 *     agentConstruct
 *
 * DOWNSTREAM
 * ----------
 *
 *     domain-neutral AST
 *     semantic analysis
 *     type checking
 *     effect checking
 *     capability checking
 *     resource checking
 *     policy checking
 *     provenance
 *     canonical semantic model
 *     classical lowering
 *     quantum::ir
 *     distributed lowering
 *     hardware/HDL lowering
 *     execution planning
 *
 * TESTS
 * -----
 *
 *     grammar/tests/ai/
 *     grammar/tests/parser/
 *     grammar/tests/semantic/
 *     grammar/tests/resources/
 *     grammar/tests/concurrency/
 *     grammar/tests/distributed/
 *     grammar/tests/quantum/
 *     grammar/tests/hardware/
 *     grammar/tests/scalability/
 *     grammar/tests/portability/
 *
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     [ ] `Agents` generates successfully.
 *
 *     [ ] It consumes only canonical ZamaniLexer tokens.
 *
 *     [ ] It contains no NANO_ANNOTATION dependency.
 *
 *     [ ] `agentConstruct` is the stable public entry rule.
 *
 *     [ ] `AI` can import `Agents` without duplicate rule ownership.
 *
 *     [ ] `agentConstruct` is accepted by the AI composition grammar.
 *
 *     [ ] Agent annotations are open-world identifiers.
 *
 *     [ ] No agent role requires a lexer keyword.
 *
 *     [ ] General expressions come from Expressions.
 *
 *     [ ] General types come from Types.
 *
 *     [ ] General statements come from Statements.
 *
 *     [ ] Nested agent composition is supported.
 *
 *     [ ] Arbitrarily many agent members are structurally representable.
 *
 *     [ ] Arbitrarily nested agent bodies are structurally representable.
 *
 *     [ ] Resource requirements remain target-independent.
 *
 *     [ ] Capabilities remain open-world.
 *
 *     [ ] Actor/concurrency semantics remain owned elsewhere.
 *
 *     [ ] Networking semantics remain owned elsewhere.
 *
 *     [ ] Quantum semantics remain owned elsewhere.
 *
 *     [ ] quantum::ir remains the canonical quantum IR.
 *
 *     [ ] No agent-specific IR exists.
 *
 *     [ ] No physical resource is selected by parsing.
 *
 *     [ ] No machine capacity is encoded.
 *
 *     [ ] No unsafe Rust is required.
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
 *     [ ] Cross-domain tests exist.
 *
 *     [ ] Compatibility tests exist.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * GRAMMAR DECLARATION
 * ============================================================================
 */

parser grammar Agents;

options {
    tokenVocab = ZamaniLexer;
}

import
    Types,
    Expressions,
    Statements;


/*
 * ============================================================================
 * 1. PUBLIC AGENT CONSTRUCT
 * ============================================================================
 *
 * This is the public rule consumed by:
 *
 *     grammar/ai/ai.g4
 *
 * Do not rename it without updating the AI composition contract.
 */

agentConstruct
    : agentAnnotatedConstruct
    ;


/*
 * ============================================================================
 * 2. ANNOTATED AGENT CONSTRUCT
 * ============================================================================
 *
 * Every agent-level construct begins with:
 *
 *     @ identifier
 *
 * Example:
 *
 *     @agent
 *     @goal
 *     @model
 *     @future_extension
 */

agentAnnotatedConstruct
    : agentAnnotation
      agentTail
    ;


/*
 * ============================================================================
 * 3. AGENT ANNOTATION
 * ============================================================================
 *
 * Canonical lexical form:
 *
 *     AT identifier
 *
 * Semantic interpretation is downstream.
 */

agentAnnotation
    : AT
      identifier
    ;


/*
 * ============================================================================
 * 4. AGENT TAIL
 * ============================================================================
 *
 * Structural dispatch:
 *
 *     '('       -> direct invocation
 *     identifier -> named construct
 *     '='       -> expression binding
 *     ':'       -> typed anonymous construct
 *     '{'       -> anonymous block construct
 *     ';'       -> empty directive
 *
 * This keeps the grammar open-world while retaining deterministic structural
 * parsing.
 */

agentTail
    : agentInvocation
    | agentNamedConstruct
    | agentBinding
    | agentTypedConstruct
    | agentBlockConstruct
    | SEMICOLON
    ;


/*
 * ============================================================================
 * 5. DIRECT INVOCATION
 * ============================================================================
 *
 * Examples:
 *
 *     @requires(qubits >= required_qubits);
 *
 *     @requires(capability("tensor.compute"));
 *
 *     @message(recipient, payload);
 *
 *     @observe(sensor(input)) {
 *         ...
 *     }
 */

agentInvocation
    : LPAREN
      agentArgumentList?
      RPAREN
      agentInvocationTail
    ;


agentInvocationTail
    : agentBody
    | SEMICOLON
    ;


/*
 * ============================================================================
 * 6. ARGUMENT LIST
 * ============================================================================
 *
 * The canonical expression grammar owns argument expressions.
 *
 * This wrapper exists so the Agents grammar has a stable public boundary
 * without defining a second expression system.
 */

agentArgumentList
    : expression
      (
          COMMA
          expression
      )*
    ;


/*
 * ============================================================================
 * 7. NAMED CONSTRUCT
 * ============================================================================
 *
 * Examples:
 *
 *     @agent Researcher {
 *         ...
 *     }
 *
 *     @goal solve;
 *
 *     @model planner;
 *
 *     @tool search;
 *
 *     @memory state: MemoryType;
 *
 *     @plan execution {
 *         ...
 *     }
 */

agentNamedConstruct
    : identifier
      agentNamedConstructTail
    ;


agentNamedConstructTail
    : agentInvocation
    | agentTypedInitializer
    | agentTypedConstruct
    | agentInitializer
    | agentBody
    | SEMICOLON
    ;


/*
 * ============================================================================
 * 8. NAMED BINDING
 * ============================================================================
 *
 * Examples:
 *
 *     @model planner = trained_model;
 *
 *     @memory state = initial_state;
 *
 *     @goal result = objective;
 */

agentBinding
    : identifier
      ASSIGN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 9. TYPED INITIALIZER
 * ============================================================================
 *
 * Examples:
 *
 *     @memory state: Memory<State> = initial_state;
 *
 *     @tool search: SearchTool = implementation;
 */

agentTypedInitializer
    : COLON
      typeExpression
      ASSIGN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 10. TYPED CONSTRUCT
 * ============================================================================
 *
 * Examples:
 *
 *     @memory state: Memory<State>;
 *
 *     @model planner: Planner;
 *
 *     @tool search: SearchTool {
 *         ...
 *     }
 */

agentTypedConstruct
    : COLON
      typeExpression
      agentTypedTail
    ;


agentTypedTail
    : agentBody
    | SEMICOLON
    ;


/*
 * ============================================================================
 * 11. INITIALIZER
 * ============================================================================
 *
 * This rule is retained as a named integration boundary.
 */

agentInitializer
    : ASSIGN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 12. BLOCK CONSTRUCT
 * ============================================================================
 *
 * Supports:
 *
 *     @agent {
 *         ...
 *     }
 *
 *     @policy {
 *         ...
 *     }
 *
 *     @plan {
 *         ...
 *     }
 *
 * Semantic analysis determines whether an anonymous construct is legal.
 */

agentBlockConstruct
    : agentBody
    ;


/*
 * ============================================================================
 * 13. AGENT BODY
 * ============================================================================
 *
 * Agent bodies may contain:
 *
 *     nested agent constructs
 *     ordinary Zamani statements
 *
 * The grammar intentionally does not define a fixed number of members.
 */

agentBody
    : LBRACE
      agentMember*
      RBRACE
    ;


/*
 * ============================================================================
 * 14. AGENT MEMBER
 * ============================================================================
 *
 * Agent constructs are recursive.
 *
 * Ordinary Zamani statements remain owned by Statements.
 */

agentMember
    : agentConstruct
    | statement
    ;


/*
 * ============================================================================
 * 15. AGENT MEMBERS
 * ============================================================================
 *
 * Public helper for parser tooling and AST/conformance tests.
 */

agentMembers
    : agentMember*
    ;


/*
 * ============================================================================
 * 16. AGENT REFERENCE
 * ============================================================================
 *
 * References are ordinary Zamani expressions.
 *
 * This named wrapper is useful for semantic tooling without creating a
 * second reference syntax.
 */

agentReference
    : expression
    ;


/*
 * ============================================================================
 * 17. AGENT SUBJECT
 * ============================================================================
 *
 * Stable semantic boundary for the subject/name of an agent annotation.
 */

agentSubject
    : identifier
    ;


/*
 * ============================================================================
 * 18. AGENT EXPRESSION
 * ============================================================================
 *
 * Explicit bridge into the canonical expression grammar.
 */

agentExpression
    : expression
    ;


/*
 * ============================================================================
 * 19. AGENT TYPE
 * ============================================================================
 *
 * Explicit bridge into the canonical type grammar.
 */

agentType
    : typeExpression
    ;


/*
 * ============================================================================
 * 20. AGENT BODY ALIAS
 * ============================================================================
 *
 * Stable helper for tools that want a named body boundary.
 */

agentBlock
    : agentBody
    ;
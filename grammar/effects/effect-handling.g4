/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/effects/effect-handling.g4
 *
 * Status:
 *     Canonical modular production grammar for EFFECT HANDLING.
 *
 * Authority:
 *     This file is the sole owner of source-level effect-handler syntax.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Rust integration baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust edition 2021
 *
 * Safety:
 *     This grammar contains no embedded Rust actions, semantic predicates,
 *     filesystem access, network access, runtime calls, hardware access,
 *     or unsafe code.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns ONLY the source syntax required to handle computational
 * effects.
 *
 * It provides:
 *
 *     - handle expressions;
 *     - handle statements;
 *     - handler computation boundaries;
 *     - handler bodies;
 *     - handler arms;
 *     - handler operation patterns;
 *     - handler pattern arguments;
 *     - handler guards;
 *     - handler results;
 *     - handler arm lists.
 *
 * This file deliberately does NOT own:
 *
 *     - effect declarations;
 *     - effect operation declarations;
 *     - effect sets;
 *     - effect references;
 *     - effect invocation/perform syntax;
 *     - ordinary expressions;
 *     - ordinary statements;
 *     - types;
 *     - identifiers;
 *     - qualified names;
 *     - capabilities;
 *     - resources;
 *     - hardware;
 *     - target selection;
 *     - routing;
 *     - scheduling;
 *     - QEC;
 *     - ZQN;
 *     - quantum::ir;
 *     - runtime dispatch;
 *     - continuation implementation.
 *
 * ============================================================================
 * CRITICAL OWNERSHIP RULE
 * ============================================================================
 *
 * This file replaces the former duplicated handler rules that existed in
 * grammar/effects/effects.g4.
 *
 * The following rules MUST have exactly one owner in the modular grammar:
 *
 *     handleExpression
 *     handleStatement
 *     effectComputation
 *     effectHandlerBody
 *     effectHandlerList
 *     effectHandlerArm
 *     effectHandlerPattern
 *     effectHandlerArguments
 *     effectHandlerGuard
 *     effectHandlerResult
 *
 * grammar/effects/effects.g4 MUST NOT redefine them.
 *
 * grammar/expressions/effects.g4 MUST consume or delegate to them.
 *
 * grammar/statements/effects.g4 MUST consume or delegate to them.
 *
 * grammar/effects/README.md MUST identify this file as the handler-syntax
 * owner.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *                         Zamani source
 *                              |
 *                              v
 *                            lexer
 *                              |
 *                              v
 *                    modular ANTLR grammar
 *                              |
 *                              v
 *                effect-handling.g4  <--- THIS FILE
 *                              |
 *                              v
 *                        frontend AST
 *                              |
 *             +----------------+----------------+
 *             |                |                |
 *             v                v                v
 *       name/type         effect analysis   capability/
 *       resolution                         resource analysis
 *                              |
 *                              v
 *                    canonical semantic model
 *                              |
 *                +-------------+-------------+
 *                |             |             |
 *                v             v             v
 *          classical IR   quantum::ir   HDL/hardware IR
 *                              |
 *                              v
 *                   optimization/lowering
 *                              |
 *                +-------------+-------------+
 *                |             |             |
 *                v             v             v
 *             routing     scheduling       resilience
 *                              |
 *                              v
 *                           QEC/ZQN
 *                              |
 *                              v
 *                             HAL
 *                              |
 *                              v
 *                      target realization
 *
 * This file is syntax only.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Effect handlers express how a program structurally handles an effect.
 *
 * They do NOT encode:
 *
 *     - a physical CPU;
 *     - a physical GPU;
 *     - a physical FPGA;
 *     - a physical QPU;
 *     - a physical qubit;
 *     - a device identifier;
 *     - a memory bank;
 *     - a network node;
 *     - a cluster topology;
 *     - a fixed accelerator;
 *     - a routing decision;
 *     - a scheduling decision;
 *     - a calibration decision.
 *
 * Therefore the same handler syntax remains valid for:
 *
 *     tiny systems
 *     embedded systems
 *     CPUs
 *     multicore systems
 *     GPUs
 *     FPGAs
 *     ASICs
 *     quantum processors
 *     simulators
 *     heterogeneous systems
 *     distributed systems
 *     clusters
 *     cloud systems
 *     future computational substrates
 *
 * subject to the semantic requirements and capabilities of the selected
 * realization.
 *
 * ============================================================================
 * NO ARTIFICIAL LIMITS
 * ============================================================================
 *
 * This grammar contains NO language-level limits for:
 *
 *     handlers
 *     handler arms
 *     handler nesting
 *     effect operations
 *     effect arguments
 *     effect domains
 *     namespaces
 *     program size
 *     machines
 *     CPUs
 *     cores
 *     threads
 *     GPUs
 *     FPGAs
 *     QPUs
 *     qubits
 *     nodes
 *     memory
 *     accelerators
 *     timelines
 *
 * In particular, this grammar MUST NOT introduce:
 *
 *     MAX_HANDLERS
 *     MAX_HANDLER_ARMS
 *     MAX_EFFECTS
 *     MAX_HANDLER_DEPTH
 *     MAX_HANDLER_ARGUMENTS
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_MEMORY
 *
 * or equivalent artificial language limits.
 *
 * Actual parser/compiler resource exhaustion is an implementation concern,
 * not a language semantic limit.
 *
 * ============================================================================
 * OPEN-WORLD EFFECT OPERATIONS
 * ============================================================================
 *
 * Effect operation identity is represented by the canonical qualified-name
 * grammar.
 *
 * Examples:
 *
 *     Read
 *     Storage::read
 *     Network::send
 *     quantum::Measurement
 *     quantum::Reset
 *     qec::Correction
 *     zqn::Observation
 *     distributed::Consensus
 *     accelerator::compute
 *     future::domain::operation
 *     vendor::extension::operation
 *
 * This grammar MUST NOT enumerate effect operations.
 *
 * New effect domains therefore do not require modifications to this file.
 *
 * Semantic analysis determines whether a referenced operation actually exists
 * and is legal.
 *
 * ============================================================================
 * EFFECT / CAPABILITY / RESOURCE SEPARATION
 * ============================================================================
 *
 * An effect handler describes semantic handling.
 *
 * It does NOT grant capabilities.
 *
 * It does NOT allocate resources.
 *
 * It does NOT select hardware.
 *
 * It does NOT prove target availability.
 *
 * It does NOT authorize external effects.
 *
 * Those responsibilities belong downstream to:
 *
 *     effect analysis
 *     capability analysis
 *     resource analysis
 *     security analysis
 *     target selection
 *     runtime/HAL
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum effect operations remain ordinary names.
 *
 * Example:
 *
 *     handle computation {
 *         case quantum::Measurement(q) => result
 *     }
 *
 * This grammar does NOT define:
 *
 *     QubitId
 *     PhysicalQubitId
 *     GateKind
 *     coupling maps
 *     topology
 *     calibration
 *     noise models
 *     QEC codes
 *     ZQN state
 *     routing
 *     scheduling.
 *
 * Quantum semantics remain downstream and ultimately use:
 *
 *     quantum::ir
 *
 * as the canonical quantum semantic boundary.
 *
 * No second quantum IR is introduced by this grammar.
 *
 * ============================================================================
 * CROSS-DOMAIN INTEGRATION
 * ============================================================================
 *
 * The same handler syntax can structurally handle effects from:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     hardware
 *     distributed
 *     AI
 *     data
 *     networking
 *     security
 *     memory
 *     concurrency
 *     accelerators
 *     interoperability
 *     future domains.
 *
 * Examples:
 *
 *     handle computation {
 *         case io::read(source) => value
 *     }
 *
 *     handle computation {
 *         case quantum::Measurement(q) => result
 *     }
 *
 *     handle computation {
 *         case distributed::send(peer, message) => result
 *     }
 *
 *     handle computation {
 *         case accelerator::compute(input) => result
 *     }
 *
 *     handle computation {
 *         case hdl::signal(event) => result
 *     }
 *
 * No domain-specific handler grammar is required for these cases.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing MUST depend only on:
 *
 *     - the input token stream;
 *     - the selected grammar;
 *     - the selected language/version contract.
 *
 * Parsing MUST NOT depend on:
 *
 *     - hardware discovery;
 *     - runtime state;
 *     - resource availability;
 *     - filesystem state;
 *     - network state;
 *     - wall-clock time;
 *     - randomness;
 *     - backend enumeration;
 *     - target selection.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Parsing a handler MUST NOT execute the handled computation.
 *
 * Parsing MUST NOT:
 *
 *     - invoke an effect;
 *     - access a file;
 *     - access a network;
 *     - access a device;
 *     - access a secret;
 *     - allocate runtime resources;
 *     - mutate compiler/runtime state.
 *
 * Handler authorization and runtime dispatch are downstream concerns.
 *
 * ============================================================================
 * ANTLR COMPOSITION CONTRACT
 * ============================================================================
 *
 * Current repository composition uses:
 *
 *     tokenVocab = ZamaniTokens
 *
 * This file deliberately preserves that currently deployed modular grammar
 * vocabulary contract.
 *
 * The repository contains a broader migration toward a single canonical
 * ZamaniLexer vocabulary. That migration must be performed consistently across
 * the grammar tree rather than changing this file independently.
 *
 * Shared parser rules are imported from:
 *
 *     Core
 *     Types
 *     Expressions
 *
 * Required shared rules include:
 *
 *     identifier
 *     qualifiedName
 *     expression
 *     blockExpression
 *     argumentList
 *
 * This file MUST NOT redefine those rules.
 *
 * ============================================================================
 * HANDLER SYNTAX
 * ============================================================================
 *
 * Canonical structure:
 *
 *     handle <computation> {
 *         case <operation-pattern> => <result>
 *     }
 *
 * Example:
 *
 *     handle read_data(key) {
 *         case Storage::read(key) => value
 *     }
 *
 * The semantic meaning of the computation and handler is determined later.
 *
 * ============================================================================
 */

parser grammar EffectHandling;

options {
    tokenVocab = ZamaniTokens;
}

import Core, Types, Expressions;


/*
 * ============================================================================
 * 1. HANDLE EXPRESSION
 * ============================================================================
 *
 * Expression-oriented effect handler.
 *
 * Example:
 *
 *     let result =
 *         handle read_data(key) {
 *             case Storage::read(key) => value
 *         };
 *
 * The resulting type is determined by semantic analysis.
 */

handleExpression
    : HANDLE
      effectComputation
      effectHandlerBody
    ;


/*
 * ============================================================================
 * 2. HANDLE STATEMENT
 * ============================================================================
 *
 * Statement-oriented adapter.
 *
 * The universal statement grammar remains the owner of `statement`.
 *
 * This rule exists so statement composition can reference the handler
 * construct without copying the handler implementation.
 */

handleStatement
    : HANDLE
      effectComputation
      effectHandlerBody
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 3. HANDLED COMPUTATION
 * ============================================================================
 *
 * A handled computation may be an ordinary expression or a block expression.
 *
 * This grammar does not introduce a second computation grammar.
 */

effectComputation
    : blockExpression
    | expression
    ;


/*
 * ============================================================================
 * 4. HANDLER BODY
 * ============================================================================
 *
 * A handler body contains zero or more handler arms.
 *
 * Whether an empty handler is semantically valid is determined downstream.
 *
 * There is deliberately no fixed handler-arm limit.
 */

effectHandlerBody
    : LBRACE
      effectHandlerList?
      RBRACE
    ;


/*
 * ============================================================================
 * 5. HANDLER ARM LIST
 * ============================================================================
 *
 * One or more arms.
 *
 * The optional body-level list allows the parser to preserve an empty handler
 * as a structurally valid construct. Semantic analysis may reject it when
 * the selected language profile requires at least one arm.
 */

effectHandlerList
    : effectHandlerArm+
    ;


/*
 * ============================================================================
 * 6. HANDLER ARM
 * ============================================================================
 *
 * Canonical form:
 *
 *     case Storage::read(key) => value
 *
 * Guarded form:
 *
 *     case Storage::read(key) when key != missing => value
 *
 * A trailing comma is accepted to support formatting and generated source.
 */

effectHandlerArm
    : CASE
      effectHandlerPattern
      effectHandlerGuard?
      FAT_ARROW
      effectHandlerResult
      COMMA?
    ;


/*
 * ============================================================================
 * 7. HANDLER OPERATION PATTERN
 * ============================================================================
 *
 * The operation identity is an ordinary qualified name.
 *
 * Optional arguments provide the operation-pattern binding/matching surface.
 *
 * Examples:
 *
 *     Read
 *     Storage::read
 *     Storage::read(key)
 *     quantum::Measurement(q)
 *     distributed::send(peer, message)
 *
 * This rule intentionally does not enumerate operation names.
 */

effectHandlerPattern
    : qualifiedName
      effectHandlerArguments?
    ;


/*
 * ============================================================================
 * 8. HANDLER PATTERN ARGUMENTS
 * ============================================================================
 *
 * Pattern arguments reuse the canonical expression argument syntax.
 *
 * This keeps handler matching integrated with the universal expression model.
 *
 * The semantic layer determines whether each argument is:
 *
 *     - a binding;
 *     - a literal pattern;
 *     - a value expression;
 *     - a destructuring form;
 *     - otherwise invalid.
 *
 * The grammar does not hard-code those semantic classifications.
 */

effectHandlerArguments
    : LPAREN
      argumentList?
      RPAREN
    ;


/*
 * ============================================================================
 * 9. HANDLER GUARD
 * ============================================================================
 *
 * Example:
 *
 *     case Storage::read(key)
 *         when key != missing
 *         => value
 *
 * The guard is an ordinary Zamani expression.
 *
 * Type checking, purity/effect constraints, and legality are semantic
 * responsibilities.
 */

effectHandlerGuard
    : WHEN
      expression
    ;


/*
 * ============================================================================
 * 10. HANDLER RESULT
 * ============================================================================
 *
 * A handler arm may return:
 *
 *     - an expression;
 *     - a block expression.
 *
 * Examples:
 *
 *     case Storage::read(key) => value
 *
 *     case Storage::read(key) => {
 *         transform(value)
 *     }
 *
 * Result typing is performed downstream.
 */

effectHandlerResult
    : blockExpression
    | expression
    ;


/*
 * ============================================================================
 * 11. HANDLER PATTERN BOUNDARY
 * ============================================================================
 *
 * Stable named boundary for AST/tooling consumers.
 *
 * This rule is intentionally an alias rather than another pattern grammar.
 */

effectHandlerOperationPattern
    : effectHandlerPattern
    ;


/*
 * ============================================================================
 * 12. HANDLER BODY BOUNDARY
 * ============================================================================
 *
 * Stable named boundary for AST/tooling consumers.
 *
 * No second body syntax is introduced.
 */

effectHandlerBlock
    : effectHandlerBody
    ;


/*
 * ============================================================================
 * 13. HANDLER COMPUTATION BOUNDARY
 * ============================================================================
 *
 * Stable named boundary for AST/tooling consumers.
 */

effectHandlingComputation
    : effectComputation
    ;


/*
 * ============================================================================
 * 14. HANDLER ARM BOUNDARY
 * ============================================================================
 *
 * Stable named boundary for tooling.
 */

effectHandlingArm
    : effectHandlerArm
    ;


/*
 * ============================================================================
 * 15. HANDLER LIST BOUNDARY
 * ============================================================================
 *
 * Stable named boundary for tooling.
 */

effectHandlingArms
    : effectHandlerList
    ;


/*
 * ============================================================================
 * 16. SEMANTICALLY NAMED HANDLER ENTRY
 * ============================================================================
 *
 * This alias allows aggregate grammars and validation tooling to refer to
 * handler syntax without taking ownership of the underlying rules.
 */

effectHandler
    : effectHandlerArm
    ;


/*
 * ============================================================================
 * 17. EFFECT HANDLER CONSTRUCT
 * ============================================================================
 *
 * This rule is the canonical modular integration point for consumers that
 * need either expression- or statement-oriented handling.
 *
 * The universal expression and statement grammars remain responsible for
 * deciding which context is legal.
 */

effectHandlingConstruct
    : handleExpression
    | handleStatement
    ;


/*
 * ============================================================================
 * 18. SOURCE-LEVEL RESUMPTION
 * ============================================================================
 *
 * IMPORTANT:
 *
 * The current canonical keyword vocabulary does not establish `resume` as a
 * dedicated effect-handler token. Therefore this file MUST NOT invent a
 * K_RESUME token.
 *
 * Resumption syntax, when supported by the language specification, should
 * initially pass through the ordinary expression grammar:
 *
 *     resume(value)
 *
 * or another approved expression form.
 *
 * Semantic effect analysis may recognize the corresponding operation in a
 * handler-result context.
 *
 * This keeps lexical ownership centralized and prevents a second keyword
 * vocabulary from appearing in this file.
 *
 * No physical/runtime continuation model is implied by the syntax.
 */


/*
 * ============================================================================
 * 19. SOURCE-LEVEL ABORT
 * ============================================================================
 *
 * The same rule applies to `abort`.
 *
 * This file deliberately does NOT reference a K_ABORT token because the
 * current canonical lexer contract does not establish one.
 *
 * If the language specification promotes `resume` or `abort` to reserved
 * keywords, the lexer specification MUST be updated first and this grammar
 * can then consume the canonical tokens.
 *
 * Until then, these concepts remain ordinary expression-level names rather
 * than fabricated lexical tokens.
 */


/*
 * ============================================================================
 * 20. AST CONTRACT
 * ============================================================================
 *
 * The parser must preserve enough structure for the frontend AST to represent:
 *
 *     - handled computation;
 *     - handler arms;
 *     - operation identity;
 *     - pattern arguments;
 *     - optional guard;
 *     - result expression/block;
 *     - source ordering;
 *     - source spans.
 *
 * Conceptually:
 *
 *     EffectHandler
 *         computation
 *         arms[]
 *         span
 *
 *     EffectHandlerArm
 *         operation
 *         arguments[]
 *         guard?
 *         result
 *         span
 *
 * Exact Rust AST types are owned by:
 *
 *     src/frontend/ast/
 *
 * This grammar MUST NOT create:
 *
 *     QuantumHandler
 *     QPUHandler
 *     GPUHandler
 *     PhysicalQubitHandler
 *     VendorHandler
 *     HardwareHandler
 *
 * or any backend-specific AST type.
 *
 * ============================================================================
 * 21. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing establishes structural validity only.
 *
 * Semantic analysis determines:
 *
 *     - whether the handled computation is valid;
 *     - which effects it may produce;
 *     - whether each handler operation resolves;
 *     - whether operation arguments match;
 *     - whether pattern bindings are valid;
 *     - whether guards are valid;
 *     - whether handlers are reachable;
 *     - whether handlers overlap;
 *     - whether required handlers are present;
 *     - whether handling is exhaustive where required;
 *     - whether the result types agree;
 *     - whether effects escape the handler;
 *     - whether resumption is permitted;
 *     - whether capabilities are available;
 *     - whether resources are satisfiable.
 *
 * None of those checks belong in this grammar.
 *
 * ============================================================================
 * 22. EFFECT ROW / EFFECT SET INTEGRATION
 * ============================================================================
 *
 * The handler grammar does not define effect rows or effect sets.
 *
 * Those remain owned by:
 *
 *     grammar/effects/effect-sets.g4
 *     grammar/types/effectful.g4
 *
 * Conceptually:
 *
 *     computation effects
 *           |
 *           v
 *     effect analysis
 *           |
 *           v
 *     handler coverage/discharge
 *           |
 *           v
 *     remaining effect set
 *
 * A handler does not automatically erase all effects.
 *
 * Semantic analysis determines which effects are actually discharged.
 *
 * ============================================================================
 * 23. CAPABILITY INTEGRATION
 * ============================================================================
 *
 * A handler does not grant a capability.
 *
 * Example:
 *
 *     handle computation {
 *         case quantum::Measurement(q) => result
 *     }
 *
 * does not itself establish:
 *
 *     capability("quantum.measurement")
 *
 * Capability requirements are resolved through the resource/capability
 * subsystem.
 *
 * ============================================================================
 * 24. RESOURCE INTEGRATION
 * ============================================================================
 *
 * A handler does not allocate or reserve physical resources.
 *
 * The handler grammar therefore does not contain:
 *
 *     physical device IDs
 *     core IDs
 *     GPU IDs
 *     FPGA IDs
 *     QPU IDs
 *     physical qubit IDs
 *     memory-bank IDs
 *     node IDs
 *
 * Resource realization occurs downstream.
 *
 * ============================================================================
 * 25. QUANTUM IR INTEGRATION
 * ============================================================================
 *
 * A quantum effect handler is still represented by the domain-neutral
 * frontend AST.
 *
 * The semantic pipeline remains:
 *
 *     handler syntax
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     effect/type/capability analysis
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     routing
 *          |
 *          v
 *     scheduling
 *          |
 *          v
 *     QEC / resilience / ZQN
 *          |
 *          v
 *     HAL
 *          |
 *          v
 *     target
 *
 * This grammar MUST NOT create another quantum IR.
 *
 * ============================================================================
 * 26. CLASSICAL / HDL / HYBRID INTEGRATION
 * ============================================================================
 *
 * Effect handling is domain-neutral.
 *
 * A handler may structurally surround:
 *
 *     classical computation
 *     quantum computation
 *     HDL/co-design computation
 *     accelerator computation
 *     distributed computation
 *     AI computation
 *     networking
 *     security operations
 *     data processing
 *     hybrid computation
 *
 * The handler grammar does not need to know which domain the computation
 * belongs to.
 *
 * ============================================================================
 * 27. DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser-level diagnostics include malformed structures such as:
 *
 *     handle
 *     handle computation
 *     handle computation {
 *     handle computation { case }
 *     handle computation { case Read(x) }
 *     handle computation { case Read(x) => }
 *
 * Semantic diagnostics belong downstream and include:
 *
 *     unknown effect;
 *     unknown operation;
 *     inaccessible operation;
 *     invalid operation pattern;
 *     invalid binding;
 *     duplicate/unreachable handler;
 *     invalid guard;
 *     non-exhaustive handler where exhaustiveness is required;
 *     incompatible handler result;
 *     illegal effect escape;
 *     illegal resumption;
 *     missing capability;
 *     unsatisfied resource requirement;
 *     unsupported target.
 *
 * The parser MUST NOT report a target/resource failure as a syntax failure.
 *
 * ============================================================================
 * 28. SOURCE-SPAN CONTRACT
 * ============================================================================
 *
 * The grammar must preserve ordinary ANTLR token/context provenance so the
 * frontend can associate diagnostics with:
 *
 *     `handle`
 *     handled computation
 *     `case`
 *     operation pattern
 *     pattern arguments
 *     guard
 *     `=>`
 *     handler result
 *     complete handler body.
 *
 * The frontend AST is responsible for converting parser contexts into the
 * repository's canonical SourceSpan representation.
 *
 * ============================================================================
 * 29. COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing valid handler forms remain structurally supported:
 *
 *     handle computation {
 *         case Storage::read(key) => result
 *     }
 *
 *     handle computation {
 *         case quantum::Measurement(q) => result
 *     }
 *
 *     handle computation {
 *         case distributed::send(peer, message) => result
 *     }
 *
 * No fixed operation catalogue is introduced.
 *
 * No physical-target syntax is introduced.
 *
 * No vendor-specific handler syntax is introduced.
 *
 * ============================================================================
 * 30. AGGREGATE INTEGRATION
 * ============================================================================
 *
 * grammar/effects/effects.g4
 * --------------------------------
 *
 * MUST import/compose this grammar rather than redefine:
 *
 *     handleExpression
 *     handleStatement
 *     effectHandlerBody
 *     effectHandler
 *     effectHandlerPattern
 *     effectHandlerResult
 *
 * grammar/expressions/effects.g4
 * --------------------------------
 *
 * MUST delegate handler expressions to:
 *
 *     handleExpression
 *
 * and MUST NOT define a second handler expression.
 *
 * grammar/statements/effects.g4
 * --------------------------------
 *
 * MUST delegate handler statements to:
 *
 *     handleStatement
 *
 * and MUST NOT redefine handler bodies or arms.
 *
 * grammar/Zamani.g4
 * --------------------------------
 *
 * MUST remain the composition root.
 *
 * It should reach handlers through the expression/statement composition path.
 *
 * ============================================================================
 * 31. IMPORTANT DUPLICATE-RULE CLEANUP
 * ============================================================================
 *
 * The current repository's grammar/effects/effects.g4 contains its own:
 *
 *     handleStatement
 *     effectHandlerBody
 *     effectHandler
 *     effectHandlerPattern
 *     resumeExpression
 *     resumeStatement
 *     abortExpression
 *     abortStatement
 *
 * Those definitions MUST be removed from the aggregate ownership layer when
 * this file becomes canonical.
 *
 * The aggregate should compose the specialized grammar instead.
 *
 * This is an integration change, not a rename.
 *
 * ============================================================================
 * 32. EFFECT OPERATIONS INTEGRATION
 * ============================================================================
 *
 * Effect operation invocation remains separate from handler syntax.
 *
 * The operation-use grammar:
 *
 *     grammar/effects/effect-operations.g4
 *
 * owns effect-operation invocation/reference syntax.
 *
 * This file only consumes operation identity inside a handler pattern.
 *
 * No operation invocation rule is duplicated here.
 *
 * ============================================================================
 * 33. EFFECT DECLARATION INTEGRATION
 * ============================================================================
 *
 * Effect declarations remain owned by:
 *
 *     grammar/effects/effect-declarations.g4
 *
 * A handler pattern may refer to an operation declared there, but this file
 * does not reproduce its declaration syntax.
 *
 * ============================================================================
 * 34. EFFECT SET INTEGRATION
 * ============================================================================
 *
 * Effect sets remain owned by:
 *
 *     grammar/effects/effect-sets.g4
 *
 * This file does not redefine:
 *
 *     effectSet
 *     effectReference
 *     effectReferenceList
 *
 * ============================================================================
 * 35. NO HARD-CODED DOMAIN VOCABULARY
 * ============================================================================
 *
 * Do NOT add alternatives such as:
 *
 *     | STORAGE_READ
 *     | NETWORK_SEND
 *     | QUANTUM_MEASURE
 *     | GPU_COMPUTE
 *     | FPGA_SIGNAL
 *     | QPU_RESET
 *
 * Such a design would make every new effect domain require grammar changes.
 *
 * The correct model is:
 *
 *     qualifiedName
 *
 * interpreted by semantic analysis.
 *
 * ============================================================================
 * 36. NO BACKEND LEAKAGE
 * ============================================================================
 *
 * Handler syntax MUST NOT contain:
 *
 *     physical_qubit(...)
 *     gpu_0(...)
 *     cpu_7(...)
 *     fpga_3(...)
 *     qpu_2(...)
 *     node_17(...)
 *
 * as universal language constructs.
 *
 * If target-specific syntax is ever required, it belongs under the explicit
 * target/dialect/interoperability architecture and must not contaminate the
 * portable effect-handler grammar.
 *
 * ============================================================================
 * 37. SCALABILITY
 * ============================================================================
 *
 * Repetition operators are used instead of fixed-count alternatives.
 *
 * The grammar imposes no semantic maximum on:
 *
 *     handler arms
 *     nesting
 *     operation domains
 *     namespaces
 *     arguments
 *     source size.
 *
 * Compiler/parser implementation limits must remain configurable resource
 * policies and MUST NOT become language semantics.
 *
 * ============================================================================
 * 38. PERFORMANCE
 * ============================================================================
 *
 * The grammar intentionally:
 *
 *     - reuses canonical expression rules;
 *     - reuses canonical name rules;
 *     - avoids semantic predicates;
 *     - avoids embedded actions;
 *     - avoids operation enumerations;
 *     - avoids target-specific alternatives;
 *     - avoids duplicate expression grammars.
 *
 * This keeps parser complexity proportional to the source structure rather
 * than the number of supported hardware/domain operations.
 *
 * ============================================================================
 * 39. SAFE RUST INTEGRATION
 * ============================================================================
 *
 * This grammar contains no Rust implementation code.
 *
 * Generated/handwritten Rust consuming it MUST remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * Production Rust MUST NOT use:
 *
 *     unsafe
 *     unsafe fn
 *     unsafe impl
 *     unsafe trait
 *     unsafe blocks
 *
 * where the repository's no-unsafe policy applies.
 *
 * This grammar MUST NOT use Rust actions as a mechanism for bypassing the
 * semantic/compiler architecture.
 *
 * ============================================================================
 * 40. TEST CONTRACT
 * ============================================================================
 *
 * Positive tests MUST include at least:
 *
 *     handle computation {
 *         case Storage::read(key) => value
 *     }
 *
 *     handle computation {
 *         case quantum::Measurement(q) => result
 *     }
 *
 *     handle computation {
 *         case distributed::send(peer, message) => result
 *     }
 *
 *     handle computation {
 *         case future::domain::operation(value) => result
 *     }
 *
 *     handle computation {
 *         case Storage::read(key) when key != missing => value
 *     }
 *
 *     handle computation {
 *         case Storage::read(key) => {
 *             transform(key)
 *         }
 *     }
 *
 * Negative tests MUST include:
 *
 *     handle
 *
 *     handle computation
 *
 *     handle computation {
 *
 *     handle computation {
 *         case
 *     }
 *
 *     handle computation {
 *         case Read(x)
 *     }
 *
 *     handle computation {
 *         case Read(x) =>
 *     }
 *
 * Boundary tests MUST include:
 *
 *     - empty handler;
 *     - one arm;
 *     - many arms;
 *     - nested handlers;
 *     - deeply qualified operation names;
 *     - large argument lists;
 *     - large source files;
 *     - Unicode identifiers where permitted by the identifier grammar.
 *
 * Cross-domain tests MUST include:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     hardware
 *     distributed
 *     AI
 *     data
 *     networking
 *     security
 *
 * Scalability tests MUST verify that no artificial handler/effect/domain
 * count is introduced.
 *
 * Determinism tests MUST parse identical source deterministically.
 *
 * Compatibility tests MUST verify stable existing handler syntax.
 *
 * ============================================================================
 * 41. HARD-CODING AUDIT
 * ============================================================================
 *
 * This file passes the architectural hard-coding audit when:
 *
 *     [ ] no physical device identifiers are encoded;
 *     [ ] no hardware counts are encoded;
 *     [ ] no qubit counts are encoded;
 *     [ ] no CPU counts are encoded;
 *     [ ] no GPU counts are encoded;
 *     [ ] no FPGA counts are encoded;
 *     [ ] no QPU counts are encoded;
 *     [ ] no network node counts are encoded;
 *     [ ] no memory capacities are encoded;
 *     [ ] no fixed handler count exists;
 *     [ ] no fixed effect catalogue exists;
 *     [ ] no vendor operation catalogue exists;
 *     [ ] no backend-specific AST is implied;
 *     [ ] no quantum gate enumeration exists;
 *     [ ] no physical routing syntax exists;
 *     [ ] no scheduling syntax exists;
 *     [ ] no QEC implementation exists;
 *     [ ] no ZQN implementation exists.
 *
 * ============================================================================
 * 42. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [ ] it is the sole handler-syntax owner;
 *     [ ] effect declarations remain elsewhere;
 *     [ ] effect operation invocation remains elsewhere;
 *     [ ] effect sets remain elsewhere;
 *     [ ] canonical names are reused;
 *     [ ] canonical expressions are reused;
 *     [ ] duplicate handler rules are removed from effects.g4;
 *     [ ] expressions/effects.g4 delegates to this grammar;
 *     [ ] statements/effects.g4 delegates to this grammar;
 *     [ ] Zamani.g4 reaches this grammar through composition;
 *     [ ] AST mapping is defined;
 *     [ ] semantic mapping is defined;
 *     [ ] capability integration is defined;
 *     [ ] resource integration is defined;
 *     [ ] quantum::ir integration is defined;
 *     [ ] no second quantum IR is introduced;
 *     [ ] QEC remains downstream;
 *     [ ] ZQN remains downstream;
 *     [ ] routing remains downstream;
 *     [ ] scheduling remains downstream;
 *     [ ] HAL remains downstream;
 *     [ ] runtime dispatch remains downstream;
 *     [ ] no artificial scalability limit exists;
 *     [ ] no backend is hard-coded;
 *     [ ] no unsafe Rust is introduced;
 *     [ ] positive tests exist;
 *     [ ] negative tests exist;
 *     [ ] boundary tests exist;
 *     [ ] cross-domain tests exist;
 *     [ ] scalability tests exist;
 *     [ ] determinism tests exist;
 *     [ ] compatibility tests exist.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * A handler describes:
 *
 *     WHAT effect behavior the source program handles.
 *
 * It does NOT describe:
 *
 *     WHERE the effect executes,
 *     HOW the effect is physically realized,
 *     WHICH resource instance performs it,
 *     WHICH hardware performs it,
 *     WHICH topology is used,
 *     WHICH scheduler is used,
 *     WHICH route is selected,
 *     WHICH QEC implementation is selected,
 *     WHICH ZQN realization is selected.
 *
 * Therefore:
 *
 *     Program Once
 *          ->
 *     Compile Once
 *          ->
 *     Analyze effects
 *          ->
 *     Match capabilities/resources
 *          ->
 *     Lower semantic representation
 *          ->
 *     Optimize
 *          ->
 *     Route
 *          ->
 *     Schedule
 *          ->
 *     Apply resilience/QEC/ZQN
 *          ->
 *     HAL
 *          ->
 *     Target realization
 *
 * remains compatible with:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * ============================================================================
 */
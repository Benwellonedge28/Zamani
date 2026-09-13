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
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Rust integration baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     This grammar contains no embedded Rust actions, semantic predicates,
 *     filesystem access, network access, runtime calls, or unsafe code.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns SOURCE-LEVEL EFFECT-HANDLING SYNTAX.
 *
 * It defines:
 *
 *     - handle expressions/statements;
 *     - handler bodies;
 *     - handler arms;
 *     - handler operation patterns;
 *     - handler pattern arguments;
 *     - handler guards;
 *     - handler result expressions;
 *     - effect computation boundaries;
 *     - handler-local binding syntax;
 *
 * It deliberately does NOT define:
 *
 *     - effect declarations;
 *     - effect operation declarations;
 *     - effect sets;
 *     - effect references;
 *     - capability semantics;
 *     - resource semantics;
 *     - hardware selection;
 *     - runtime dispatch;
 *     - continuation implementation;
 *     - quantum semantics;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN;
 *     - routing;
 *     - scheduling;
 *     - optimization;
 *     - resilience;
 *     - backend selection.
 *
 * ============================================================================
 * ARCHITECTURAL BOUNDARY
 * ============================================================================
 *
 *                         Zamani source
 *                              |
 *                              v
 *                         Zamani lexer
 *                              |
 *                              v
 *                    modular parser grammars
 *                              |
 *                              v
 *                 effect-handling.g4
 *                              |
 *                              v
 *                         frontend AST
 *                              |
 *                 +------------+------------+
 *                 |            |            |
 *                 v            v            v
 *             name/type    effect       capability
 *             resolution   analysis     analysis
 *                 |            |            |
 *                 +------------+------------+
 *                              |
 *                              v
 *                    canonical semantic model
 *                              |
 *                 +------------+------------+
 *                 |            |            |
 *                 v            v            v
 *          classical IR    quantum::ir   resource metadata
 *                              |
 *                              v
 *             optimization / routing / scheduling
 *                              |
 *                              v
 *                    target lowering/runtime
 *
 * This grammar is syntax-only.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * OWNS:
 *
 *     handleExpression
 *     handleStatement
 *     effectHandlerBody
 *     effectHandlerArm
 *     effectHandlerPattern
 *     effectHandlerArguments
 *     effectHandlerGuard
 *     effectHandlerBinding
 *     effectHandlerResult
 *     effectComputation
 *     effectHandlerList
 *
 * DOES NOT OWN:
 *
 *     identifier
 *     qualifiedName
 *     expression
 *     blockExpression
 *     argumentList
 *     typeExpression
 *     effectReference
 *     effectSet
 *     effectDeclaration
 *     effectOperationDeclaration
 *     capability
 *     resource
 *     hardware
 *     quantum
 *     QEC
 *     ZQN
 *     IR
 *     runtime
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Effect handlers describe how source-level computational effects are
 * structurally handled.
 *
 * They MUST NOT encode physical realization.
 *
 * The same handler source must remain structurally valid regardless of whether
 * the handled computation is eventually realized by:
 *
 *     - a tiny embedded system;
 *     - a CPU;
 *     - many CPUs;
 *     - a GPU;
 *     - an FPGA;
 *     - an ASIC;
 *     - a quantum processor;
 *     - a simulator;
 *     - a distributed system;
 *     - a heterogeneous system;
 *     - a future computational substrate.
 *
 * This grammar therefore contains no:
 *
 *     MAX_HANDLERS
 *     MAX_HANDLER_ARMS
 *     MAX_EFFECTS
 *     MAX_HANDLER_DEPTH
 *     MAX_HANDLER_ARGUMENTS
 *     MAX_RESOURCE_COUNT
 *     MAX_QUBITS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_DEVICES
 *
 * or equivalent machine-specific limits.
 *
 * ============================================================================
 * OPEN-WORLD EFFECT MODEL
 * ============================================================================
 *
 * Effect operation names are ordinary qualified names.
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
 *     future::domain::operation
 *
 * This grammar does not enumerate any of them.
 *
 * New effect domains therefore do not require modification of this file.
 *
 * ============================================================================
 * SEMANTIC SEPARATION
 * ============================================================================
 *
 * Syntax:
 *
 *     handle computation {
 *         case Storage::read(key) => result
 *     }
 *
 * Semantic analysis determines:
 *
 *     - which effect is being handled;
 *     - whether the operation exists;
 *     - whether the handler is applicable;
 *     - whether arguments match;
 *     - whether the handler is exhaustive;
 *     - whether the handler may resume;
 *     - whether resumption is legal;
 *     - whether the result type is valid;
 *     - whether required capabilities exist;
 *     - whether required resources can be realized.
 *
 * The parser performs none of those operations.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Quantum effect handlers are represented using ordinary names.
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
 *     topology
 *     calibration
 *     noise
 *     QEC codes
 *     ZQN faults
 *
 * Quantum semantics are lowered downstream to the canonical quantum semantic
 * boundary:
 *
 *     quantum::ir
 *
 * ============================================================================
 * EFFECT / CAPABILITY / RESOURCE SEPARATION
 * ============================================================================
 *
 * Handling an effect does not grant a capability.
 *
 * For example:
 *
 *     handle computation {
 *         case FileSystem::read(path) => value
 *     }
 *
 * does not grant filesystem permission.
 *
 * Likewise:
 *
 *     handle computation {
 *         case quantum::Measurement(q) => value
 *     }
 *
 * does not select a QPU, topology, device, or qubit.
 *
 * Capability authorization belongs to security/capability analysis.
 *
 * Resource realization belongs to resource analysis and target lowering.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no semantic predicates;
 *     - no embedded actions;
 *     - no I/O;
 *     - no network access;
 *     - no hardware discovery;
 *     - no runtime calls;
 *     - no random behavior.
 *
 * Given the same deterministic token stream, parsing is deterministic.
 *
 * ============================================================================
 * ANTLR COMPOSITION CONTRACT
 * ============================================================================
 *
 * The canonical lexer supplies the token vocabulary.
 *
 * Shared syntax is imported from the canonical parser grammars.
 *
 * Required shared rules:
 *
 *     identifier
 *     qualifiedName
 *     expression
 *     blockExpression
 *     argumentList
 *
 * This grammar MUST NOT duplicate those rules.
 *
 * Effect references remain owned by:
 *
 *     grammar/effects/effect-sets.g4
 *
 * Effect declarations remain owned by:
 *
 *     grammar/effects/effect-declarations.g4
 *
 * Effect-set syntax remains owned by:
 *
 *     grammar/effects/effect-sets.g4
 *
 * The aggregate:
 *
 *     grammar/effects/effects.g4
 *
 * imports/composes these grammars and MUST NOT create competing handler rules.
 *
 * ============================================================================
 * IMPORTANT TOKEN CONTRACT
 * ============================================================================
 *
 * The current canonical lexer provides the effect/control tokens:
 *
 *     K_EFFECT
 *     K_PERFORM
 *     K_HANDLE
 *     K_CASE
 *
 * This grammar therefore consumes those canonical tokens.
 *
 * This file MUST NOT invent alternative token names such as:
 *
 *     HANDLE_EFFECT
 *     EFFECT_CASE
 *     RESUME_TOKEN
 *     ABORT_TOKEN
 *
 * If Zamani adopts explicit `resume` or `abort` syntax, those keywords MUST
 * first be established by the canonical lexer and language-versioning
 * specification. They are intentionally NOT fabricated here.
 *
 * ============================================================================
 * HANDLER MODEL
 * ============================================================================
 *
 * A handler has the general structure:
 *
 *     handle <computation> {
 *         case <operation-pattern> => <handler-result>
 *         ...
 *     }
 *
 * A handler may contain zero or more arms syntactically.
 *
 * Whether an empty handler is semantically useful is determined later.
 *
 * No handler-count limit exists.
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
 * Expression-oriented handler.
 *
 * Example:
 *
 *     handle computation {
 *         case Storage::read(key) => value
 *     }
 *
 * The result type is determined by semantic analysis.
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
 * Statement-oriented handler.
 *
 * A separate rule is retained so the AST can distinguish expression and
 * statement contexts without duplicating handler syntax.
 */

handleStatement
    : HANDLE
      effectComputation
      effectHandlerBody
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 3. EFFECT COMPUTATION
 * ============================================================================
 *
 * The computation being handled belongs to the canonical expression/block
 * grammar.
 *
 * No second computation language is introduced here.
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
 * No fixed number of handler arms is imposed.
 */

effectHandlerBody
    : LBRACE
      effectHandlerList?
      RBRACE
    ;


/*
 * ============================================================================
 * 5. HANDLER LIST
 * ============================================================================
 *
 * One or more handler arms.
 *
 * Repetition is unbounded at the language level.
 */

effectHandlerList
    : effectHandlerArm
      (
          effectHandlerArm
      )*
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
 * Optional guards provide source-level conditional handling:
 *
 *     case Storage::read(key) when key != empty => value
 *
 * Guard meaning is semantic.
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
 * 7. HANDLER PATTERN
 * ============================================================================
 *
 * A handler pattern identifies an effect operation.
 *
 * Examples:
 *
 *     Read
 *     Storage::read
 *     Storage::read(key)
 *     quantum::Measurement(q)
 *
 * The effect namespace remains open-ended.
 */

effectHandlerPattern
    : qualifiedName
      effectHandlerArguments?
    ;


/*
 * ============================================================================
 * 8. HANDLER ARGUMENTS
 * ============================================================================
 *
 * Arguments use canonical expression syntax.
 *
 * This permits:
 *
 *     Read(key)
 *     Read(buffer[index])
 *     quantum::Measurement(q)
 *     distributed::send(node, message)
 *
 * without creating a second expression grammar.
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
 * Optional condition associated with a handler arm.
 *
 * Example:
 *
 *     case Read(key) when key != missing => value
 *
 * The expression is evaluated semantically, not by the parser.
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
 * A handler arm may produce either a block or an expression.
 *
 * Example:
 *
 *     case Read(key) => value
 *
 * or:
 *
 *     case Read(key) => {
 *         value
 *     }
 */

effectHandlerResult
    : blockExpression
    | expression
    ;


/*
 * ============================================================================
 * 11. HANDLER BINDING
 * ============================================================================
 *
 * Explicit binding boundary for tooling and AST construction.
 *
 * This rule does not change the operation-pattern syntax.
 *
 * Example:
 *
 *     key
 *
 * The semantic layer determines whether the binding is legal and what it
 * binds to.
 */

effectHandlerBinding
    : identifier
    ;


/*
 * ============================================================================
 * 12. HANDLER BINDING LIST
 * ============================================================================
 *
 * A reusable binding collection.
 *
 * No finite binding count is imposed.
 */

effectHandlerBindingList
    : effectHandlerBinding
      (
          COMMA
          effectHandlerBinding
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 13. HANDLER PATTERN LIST
 * ============================================================================
 *
 * This rule is useful for tooling and future pattern-composition constructs.
 *
 * It does not define semantic OR/AND behavior.
 */

effectHandlerPatternList
    : effectHandlerPattern
      (
          COMMA
          effectHandlerPattern
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 14. EFFECT HANDLING COMPUTATION
 * ============================================================================
 *
 * Stable parser-tree boundary for tooling.
 */

effectHandlingComputation
    : effectComputation
    ;


/*
 * ============================================================================
 * 15. EFFECT HANDLING BLOCK
 * ============================================================================
 *
 * Stable parser-tree boundary for AST construction.
 */

effectHandlingBlock
    : effectHandlerBody
    ;
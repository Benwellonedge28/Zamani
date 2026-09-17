/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/expressions/effects.g4
 *
 * Status:
 *     Canonical production expression grammar component for effect use.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Rust implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Edition 2021
 *     Safe Rust only
 *     No unsafe Rust required or permitted.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns SOURCE-LEVEL EFFECT EXPRESSIONS.
 *
 * It deliberately owns only expression forms in which a computation
 * explicitly performs an effect operation.
 *
 * Primary form:
 *
 *     perform Effect::operation(arguments...)
 *
 * Examples:
 *
 *     perform IO::read(path)
 *     perform Storage::read(key)
 *     perform Network::send(endpoint, message)
 *     perform quantum::Measurement(q)
 *     perform qec::Correction(state)
 *     perform zqn::Observation(signal)
 *     perform future::domain::operation(value)
 *
 * Effect identities are OPEN-WORLD.
 *
 * The grammar MUST NOT enumerate:
 *
 *     IO
 *     Network
 *     Quantum
 *     Measurement
 *     QEC
 *     ZQN
 *     GPU
 *     CPU
 *     FPGA
 *     Storage
 *     ...
 *
 * Such names are ordinary source-level qualified names.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     effectExpression
 *     effectPerformExpression
 *     effectInvocation
 *     effectInvocationArguments
 *
 * THIS FILE DOES NOT OWN:
 *
 *     effect declarations
 *     effect operation declarations
 *     effect sets
 *     effect handlers
 *     handler arms
 *     capabilities
 *     resources
 *     hardware
 *     scheduling
 *     routing
 *     optimization
 *     QEC
 *     ZQN
 *     HAL
 *     quantum::ir
 *     runtime dispatch
 *
 * Those remain owned by their respective grammar/domain contracts.
 *
 * Effect declarations:
 *
 *     grammar/effects/effect-declarations.g4
 *
 * Effect sets:
 *
 *     grammar/effects/effect-sets.g4
 *
 * Effect handlers:
 *
 *     grammar/effects/effect-handling.g4
 *
 * Effect domain extensions:
 *
 *     grammar/effects/*.g4
 *
 * ============================================================================
 * ARCHITECTURAL BOUNDARY
 * ============================================================================
 *
 * Source
 *   |
 *   v
 * Lexer
 *   |
 *   v
 * Parser
 *   |
 *   v
 * Domain-neutral frontend AST
 *   |
 *   v
 * Effect analysis
 *   |
 *   v
 * Canonical semantic model / ZUIR
 *   |
 *   +--------------------+--------------------+
 *   |                    |                    |
 *   v                    v                    v
 * Classical IR       quantum::ir       HDL / Hardware IR
 *   |                    |                    |
 *   +--------------------+--------------------+
 *                        |
 *                        v
 *               optimization / lowering
 *                        |
 *               routing / scheduling
 *                        |
 *                 resilience / QEC / ZQN
 *                        |
 *                       HAL
 *                        |
 *                 target realization
 *
 * This grammar MUST remain upstream of all target-specific realization.
 *
 * ============================================================================
 * EFFECT / CAPABILITY / RESOURCE SEPARATION
 * ============================================================================
 *
 * An effect says WHAT computational behavior occurs.
 *
 * A capability says WHAT an execution environment can provide.
 *
 * A resource says WHAT computational resource exists or is requested.
 *
 * A requirement says WHAT must be satisfied.
 *
 * A constraint says WHAT conditions must hold.
 *
 * A preference says WHICH valid realization is preferred.
 *
 * A hint suggests an implementation direction without becoming semantic
 * necessity.
 *
 * Therefore:
 *
 *     perform quantum::Measurement(q)
 *
 * does NOT mean:
 *
 *     use QPU 0
 *     use physical qubit 7
 *     use topology X
 *     use vendor Y
 *
 * Those decisions belong downstream.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * This grammar imposes no universal finite limit on:
 *
 *     effect operations
 *     effect arguments
 *     effect nesting
 *     expression depth
 *     namespace depth
 *     program size
 *     effect count
 *     quantum resources
 *     classical resources
 *     hardware resources
 *     distributed participants
 *     accelerator count
 *
 * There is deliberately no:
 *
 *     MAX_EFFECTS
 *     MAX_EFFECT_ARGUMENTS
 *     MAX_EFFECT_DEPTH
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *
 * or equivalent language-level limit.
 *
 * Practical implementation budgets remain compiler/parser/runtime policy and
 * are not part of the language semantics.
 *
 * ============================================================================
 * OPEN-WORLD EFFECT IDENTITIES
 * ============================================================================
 *
 * Effect operation identity is represented by the canonical qualified-name
 * system.
 *
 * Examples:
 *
 *     IO::read
 *     Storage::write
 *     quantum::Measurement
 *     qec::Correction
 *     zqn::Observation
 *     distributed::Consensus
 *     accelerator::tensor
 *     future::domain::operation
 *
 * No closed operation catalogue is embedded here.
 *
 * Adding a new domain therefore does not require changing this grammar.
 *
 * ============================================================================
 * EXPRESSION INTEGRATION CONTRACT
 * ============================================================================
 *
 * The canonical expression grammar already owns:
 *
 *     expression
 *     postfixExpression
 *     argumentList
 *     primaryExpression
 *     blockExpression
 *
 * This file MUST NOT redefine any of them.
 *
 * Effect expressions are inserted into the canonical expression hierarchy
 * through the expression composition layer.
 *
 * The integration point is:
 *
 *     primaryExpression
 *         |
 *         +--> effectExpression
 *
 * The effect expression then owns only:
 *
 *     perform ...
 *
 * The arguments are ordinary Zamani expressions.
 *
 * This guarantees that:
 *
 *     perform IO::read(a + b)
 *
 * and:
 *
 *     perform quantum::Measurement(register[index])
 *
 * use exactly the same expression semantics as the rest of Zamani.
 *
 * ============================================================================
 * PRECEDENCE
 * ============================================================================
 *
 * `perform` is a prefix expression construct.
 *
 * Its operand is an effect invocation rather than a general arbitrary
 * expression.
 *
 * This avoids introducing a recursive:
 *
 *     perform expression
 *
 * production that could accidentally consume surrounding operators.
 *
 * For example:
 *
 *     perform IO::read(x) + y
 *
 * is parsed as:
 *
 *     (perform IO::read(x)) + y
 *
 * rather than:
 *
 *     perform (IO::read(x) + y)
 *
 * If a future language version wants the latter semantics, explicit
 * parenthesization remains available.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser must expose a distinct syntactic node/context for effect use.
 *
 * Conceptually:
 *
 *     EffectExpression
 *         kind = Perform
 *         operation = QualifiedName
 *         arguments = Expression[]
 *         source_span = SourceSpan
 *
 * The frontend AST remains domain-neutral.
 *
 * The grammar MUST NOT create:
 *
 *     QuantumGate
 *     PhysicalQubit
 *     GPUOperation
 *     CPUInstruction
 *     VendorOperation
 *
 * or another backend-specific AST type.
 *
 * The effect operation identity is represented generically.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing establishes only that the source has valid effect-expression
 * structure.
 *
 * Semantic analysis determines:
 *
 *     - whether the effect exists;
 *     - whether it is imported;
 *     - whether it is user-defined;
 *     - whether it is available in the selected language version;
 *     - whether the arguments are valid;
 *     - whether argument types match;
 *     - what result type is produced;
 *     - what effects are propagated;
 *     - what capabilities are required;
 *     - what resources are required;
 *     - whether the operation is permitted;
 *     - whether the operation crosses a domain boundary.
 *
 * None of those decisions belong in this grammar.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum effects use exactly the same open-world effect syntax.
 *
 * Examples:
 *
 *     perform quantum::Measurement(q)
 *     perform quantum::Reset(q)
 *     perform quantum::DynamicControl(condition)
 *     perform qec::Correction(state)
 *     perform zqn::Observation(signal)
 *
 * This file does NOT define:
 *
 *     QubitId
 *     PhysicalQubitId
 *     GateKind
 *     topology
 *     calibration
 *     noise model
 *     QEC code
 *     ZQN fault
 *     routing
 *     scheduling
 *
 * Quantum semantic lowering remains:
 *
 *     effect expression
 *         |
 *         v
 *     semantic analysis
 *         |
 *         v
 *     quantum::ir
 *         |
 *         v
 *     optimization / routing / scheduling / QEC / ZQN
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing must depend only on:
 *
 *     - the input token sequence;
 *     - the active grammar/version contract.
 *
 * It must NOT depend on:
 *
 *     - wall-clock time;
 *     - randomness;
 *     - environment variables;
 *     - hardware discovery;
 *     - device state;
 *     - network state;
 *     - runtime scheduling.
 *
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * Structural errors should be reported by the normal ANTLR/frontend
 * diagnostic pipeline.
 *
 * Examples of malformed syntax include:
 *
 *     perform
 *     perform ;
 *     perform ::
 *     perform IO::
 *     perform IO::read(
 *     perform IO::read(,)
 *
 * Semantic errors such as:
 *
 *     unknown effect
 *     unavailable capability
 *     invalid argument type
 *     unavailable resource
 *
 * belong to semantic analysis rather than this grammar.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * The grammar performs no I/O and no runtime dispatch.
 *
 * Parsing:
 *
 *     MUST NOT execute effects.
 *     MUST NOT resolve devices.
 *     MUST NOT access secrets.
 *     MUST NOT contact networks.
 *     MUST NOT access files.
 *     MUST NOT invoke hardware.
 *
 * `perform` is source syntax only.
 *
 * ============================================================================
 * COMPILER / RUNTIME INTEGRATION
 * ============================================================================
 *
 * The expected pipeline is:
 *
 *     effectPerformExpression
 *             |
 *             v
 *     frontend AST effect operation
 *             |
 *             v
 *     semantic effect model
 *             |
 *             v
 *     canonical semantic representation / ZUIR
 *             |
 *             +--> classical domain
 *             +--> quantum::ir
 *             +--> HDL / hardware domain
 *             +--> distributed domain
 *             +--> accelerator domain
 *             +--> future domain
 *             |
 *             v
 *     optimization / lowering
 *             |
 *             v
 *     runtime / target realization
 *
 * The runtime is the component that eventually realizes the effect.
 *
 * ============================================================================
 * ANTLR TOKEN CONTRACT
 * ============================================================================
 *
 * Canonical token vocabulary:
 *
 *     grammar/lexer/tokens.g4
 *
 * Canonical effect tokens include:
 *
 *     K_PERFORM
 *
 * The grammar MUST NOT invent:
 *
 *     K_EFFECTS
 *     K_RESUME
 *     K_ABORT
 *     HANDLE_EFFECT
 *     EFFECT_CALL
 *
 * unless those tokens are first introduced into the canonical lexer and
 * language specification.
 *
 * ============================================================================
 * NO DUPLICATION CONTRACT
 * ============================================================================
 *
 * This file MUST NOT define:
 *
 *     effectDeclaration
 *     effectOperationDeclaration
 *     effectSet
 *     effectHandler
 *     effectHandlerArm
 *     effectHandlerPattern
 *     handleExpression
 *
 * Those belong to the effect declaration/handler grammars.
 *
 * ============================================================================
 */

parser grammar EffectExpressions;

options {
    tokenVocab = ZamaniTokens;
}


/*
 * ============================================================================
 * 1. EFFECT EXPRESSION
 * ============================================================================
 *
 * Public expression-level entry point.
 *
 * This rule intentionally contains only expression-side effect operations.
 */
effectExpression
    : effectPerformExpression
    ;


/*
 * ============================================================================
 * 2. PERFORM EXPRESSION
 * ============================================================================
 *
 * Canonical source form:
 *
 *     perform IO::read(path)
 *
 *     perform quantum::Measurement(q)
 *
 *     perform future::domain::operation(value)
 *
 * `perform` introduces effectful computation but does not itself specify
 * target realization.
 */
effectPerformExpression
    : K_PERFORM
      effectInvocation
    ;


/*
 * ============================================================================
 * 3. EFFECT INVOCATION
 * ============================================================================
 *
 * The operation identity is an ordinary qualified name.
 *
 * The operation may have zero or more ordinary Zamani expression arguments.
 *
 * Examples:
 *
 *     IO::flush
 *     IO::read(path)
 *     quantum::Measurement(q)
 *     distributed::send(node, message)
 */
effectInvocation
    : qualifiedName
      effectInvocationArguments?
    ;


/*
 * ============================================================================
 * 4. EFFECT INVOCATION ARGUMENTS
 * ============================================================================
 *
 * The canonical expression grammar owns the general `argumentList`.
 *
 * This wrapper exists to give the AST/parser tooling a stable effect-specific
 * boundary without creating a second argument grammar.
 */
effectInvocationArguments
    : LPAREN
      argumentList?
      RPAREN
    ;


/*
 * ============================================================================
 * 5. EFFECT OPERATION REFERENCE
 * ============================================================================
 *
 * Reference-only form used by composition/tooling grammars.
 *
 * It does not perform the operation.
 */
effectOperationReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * 6. EFFECT OPERATION CALL
 * ============================================================================
 *
 * Stable named boundary for tools that need to distinguish the operation
 * invocation from the leading `perform` keyword.
 */
effectOperationCall
    : effectInvocation
    ;
/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/effects/effect-operations.g4
 *
 * Status:
 *     Canonical modular production grammar for EFFECT OPERATION USE.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Language/runtime baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust edition 2021
 *
 * Safety:
 *     This grammar contains:
 *
 *       - no embedded Rust actions;
 *       - no semantic predicates;
 *       - no filesystem access;
 *       - no network access;
 *       - no runtime calls;
 *       - no hardware discovery;
 *       - no unsafe code.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the SOURCE-LEVEL SYNTAX for referring to and invoking an
 * effect operation.
 *
 * It deliberately separates:
 *
 *     effect operation declaration
 *
 * from:
 *
 *     effect operation use/invocation.
 *
 * Effect operation declarations remain owned by:
 *
 *     grammar/effects/effect-declarations.g4
 *
 * Effect sets remain owned by:
 *
 *     grammar/effects/effect-sets.g4
 *
 * Effect handlers remain owned by:
 *
 *     grammar/effects/effect-handling.g4
 *
 * Expression-level composition remains owned by:
 *
 *     grammar/expressions/effects.g4
 *
 * Statement-level composition remains owned by:
 *
 *     grammar/statements/effects.g4
 *
 * This file therefore provides reusable operation-use syntax without creating
 * a second effect language.
 *
 * ============================================================================
 * CORE ARCHITECTURAL RULE
 * ============================================================================
 *
 * An effect operation is identified by a normal Zamani qualified name.
 *
 * Examples:
 *
 *     IO::read
 *     Storage::write
 *     Network::send
 *     quantum::Measurement
 *     quantum::Reset
 *     qec::Correction
 *     zqn::Observation
 *     accelerator::Tensor
 *     distributed::Consensus
 *     hardware::Clock
 *     future::domain::operation
 *     vendor::extension::operation
 *
 * The grammar MUST NOT enumerate effect operations.
 *
 * There is therefore deliberately no grammar such as:
 *
 *     effectOperation
 *         : READ
 *         | WRITE
 *         | MEASURE
 *         | RESET
 *         | ...
 *         ;
 *
 * Operation identity is DATA.
 *
 * Its meaning is determined by semantic analysis.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     effectOperationReference
 *     effectInvocation
 *     effectInvocationArguments
 *     effectOperationCall
 *     effectOperationUse
 *     performEffectOperation
 *     effectOperationArguments
 *
 * THIS FILE DOES NOT OWN:
 *
 *     effectDeclaration
 *     effectOperationDeclaration
 *     effectBody
 *     effectParameterList
 *     effectSet
 *     effectHandler
 *     effectHandlerArm
 *     effectHandlerPattern
 *     expression
 *     argumentList
 *     identifier
 *     qualifiedName
 *     typeExpression
 *     capabilities
 *     resources
 *     hardware targets
 *     quantum gates
 *     physical qubits
 *     routing
 *     scheduling
 *     QEC
 *     ZQN
 *     canonical IR
 *     runtime dispatch
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Effect operation syntax expresses:
 *
 *     WHAT semantic operation is requested.
 *
 * It does not express:
 *
 *     WHICH physical device performs it;
 *     WHICH CPU performs it;
 *     WHICH GPU performs it;
 *     WHICH FPGA performs it;
 *     WHICH QPU performs it;
 *     WHICH physical qubit is selected;
 *     WHICH network node is selected;
 *     WHICH memory bank is selected;
 *     WHICH accelerator instance is selected;
 *     WHICH topology is selected.
 *
 * Therefore:
 *
 *     perform quantum::Measurement(q)
 *
 * remains portable.
 *
 * It may eventually be realized by:
 *
 *     - a simulator;
 *     - a QPU;
 *     - a hybrid target;
 *     - a distributed service;
 *     - an accelerator;
 *     - a future computational substrate.
 *
 * The grammar does not decide which realization is used.
 *
 * ============================================================================
 * OPEN-WORLD EXTENSIBILITY
 * ============================================================================
 *
 * New effect domains do not require grammar changes.
 *
 * Examples:
 *
 *     perform photonic::interference(state);
 *     perform neuromorphic::spike(signal);
 *     perform accelerator::tensor_compute(tensor);
 *     perform distributed::consensus(value);
 *     perform robotics::motion(command);
 *     perform future::computing::operation(value);
 *
 * The parser accepts these because they are names.
 *
 * Semantic analysis may subsequently reject:
 *
 *     - unknown effects;
 *     - unknown operations;
 *     - unavailable versions;
 *     - unavailable capabilities;
 *     - invalid arguments;
 *     - invalid effect context.
 *
 * Those are NOT parser responsibilities.
 *
 * ============================================================================
 * EFFECT / CAPABILITY / RESOURCE SEPARATION
 * ============================================================================
 *
 * EFFECT:
 *
 *     What computation does or may do?
 *
 * CAPABILITY:
 *
 *     What an execution environment can provide.
 *
 * RESOURCE:
 *
 *     What computational resource is available or requested.
 *
 * REQUIREMENT:
 *
 *     What must be satisfied for realization.
 *
 * CONSTRAINT:
 *
 *     What a valid realization must obey.
 *
 * PREFERENCE:
 *
 *     Which valid realization is preferred.
 *
 * HINT:
 *
 *     Non-binding implementation guidance.
 *
 * TARGET:
 *
 *     The eventual realization environment.
 *
 * This file only describes effect-operation syntax.
 *
 * It must not silently turn an effect operation into a hardware requirement.
 *
 * ============================================================================
 * QUANTUM ARCHITECTURAL BOUNDARY
 * ============================================================================
 *
 * Quantum operations use exactly the same generic operation syntax.
 *
 * Examples:
 *
 *     perform quantum::Measurement(q);
 *     perform quantum::Reset(q);
 *     perform quantum::Readout(q);
 *     perform quantum::DynamicControl(condition);
 *     perform qec::Correction(state);
 *     perform zqn::Observation(signal);
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
 *     routing
 *     scheduling.
 *
 * The semantic pipeline remains:
 *
 *     source
 *       |
 *       v
 *     lexer
 *       |
 *       v
 *     parser
 *       |
 *       v
 *     frontend AST
 *       |
 *       v
 *     semantic effect analysis
 *       |
 *       v
 *     canonical semantic representation
 *       |
 *       v
 *     quantum::ir
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     routing
 *       |
 *       v
 *     scheduling
 *       |
 *       v
 *     QEC / resilience / ZQN
 *       |
 *       v
 *     HAL
 *       |
 *       v
 *     target realization
 *
 * This file MUST NOT create another quantum IR.
 *
 * ============================================================================
 * CLASSICAL / HDL / HARDWARE / DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * The same operation syntax supports:
 *
 *     perform io::read(source);
 *     perform memory::allocate(size);
 *     perform accelerator::compute(data);
 *     perform hardware::signal(value);
 *     perform distributed::send(peer, message);
 *     perform networking::request(endpoint);
 *     perform security::authorize(subject);
 *     perform data::transform(dataset);
 *     perform ai::infer(model, input);
 *     perform hdl::simulate(signal);
 *
 * None of these names imply a physical implementation.
 *
 * Semantic domain analysis occurs downstream.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * The grammar imposes no semantic limit on:
 *
 *     - number of effect operations;
 *     - number of effect invocations;
 *     - number of arguments;
 *     - number of nested invocations;
 *     - qualified-name depth;
 *     - number of namespaces;
 *     - number of domains;
 *     - program size;
 *     - effect-set size;
 *     - handler count;
 *     - machine size;
 *     - CPU count;
 *     - GPU count;
 *     - FPGA count;
 *     - QPU count;
 *     - qubit count;
 *     - node count;
 *     - accelerator count;
 *     - memory capacity.
 *
 * The grammar MUST NOT contain:
 *
 *     MAX_EFFECT_OPERATIONS
 *     MAX_EFFECT_ARGUMENTS
 *     MAX_EFFECT_DEPTH
 *     MAX_EFFECTS
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
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *
 * or equivalent language-level limits.
 *
 * Practical parser/compiler limits are implementation resource policies.
 *
 * They must not change the language semantics.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     - the token stream;
 *     - the active grammar;
 *     - the language version selected for parsing.
 *
 * Parsing MUST NOT depend on:
 *
 *     - hardware discovery;
 *     - runtime state;
 *     - device state;
 *     - network state;
 *     - wall-clock time;
 *     - randomness;
 *     - environment variables;
 *     - resource availability.
 *
 * Resource availability is evaluated after parsing.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * The keyword:
 *
 *     perform
 *
 * is SOURCE SYNTAX ONLY.
 *
 * Parsing a perform expression MUST NOT:
 *
 *     - execute the effect;
 *     - invoke a runtime;
 *     - access a device;
 *     - access a file;
 *     - access a network;
 *     - access a secret;
 *     - allocate a resource;
 *     - modify external state.
 *
 * Runtime dispatch belongs downstream.
 *
 * ============================================================================
 * ANTLR COMPOSITION CONTRACT
 * ============================================================================
 *
 * Canonical lexer:
 *
 *     grammar/lexer/tokens.g4
 *
 * Canonical token vocabulary:
 *
 *     ZamaniTokens
 *
 * This grammar therefore uses:
 *
 *     tokenVocab = ZamaniTokens
 *
 * Shared syntax is imported from:
 *
 *     Core
 *     Expressions
 *
 * Core provides:
 *
 *     identifier
 *     qualifiedName
 *     attributes
 *     visibility
 *     and other language-wide name syntax.
 *
 * Expressions provides:
 *
 *     expression
 *     argumentList
 *     and other canonical expression syntax.
 *
 * This grammar MUST NOT redefine those rules.
 *
 * ============================================================================
 * IMPORTANT COMPOSITION RULE
 * ============================================================================
 *
 * `effect-declarations.g4` owns:
 *
 *     effectOperationDeclaration
 *
 * This file owns:
 *
 *     effectOperationReference
 *     effectInvocation
 *     effectOperationUse
 *
 * That separation is intentional.
 *
 * Declaration:
 *
 *     effect Storage {
 *         fn read(key: Key) -> Value;
 *     }
 *
 * Use:
 *
 *     perform Storage::read(key);
 *
 * A declaration and an invocation are different AST/semantic events.
 *
 * ============================================================================
 * EXPRESSION INTEGRATION
 * ============================================================================
 *
 * The preferred expression-level integration is:
 *
 *     grammar/expressions/effects.g4
 *
 * which may expose:
 *
 *     effectPerformExpression
 *
 * by delegating to:
 *
 *     effectOperationUse
 *
 * This avoids maintaining two implementations of:
 *
 *     perform <qualified-name>(<arguments>)
 *
 * ============================================================================
 * STATEMENT INTEGRATION
 * ============================================================================
 *
 * `grammar/statements/effects.g4` may expose:
 *
 *     effectStatement
 *
 * and delegate its perform branch to the expression-level effect grammar or
 * directly to:
 *
 *     effectOperationUse
 *
 * The universal statement grammar remains the owner of:
 *
 *     statement
 *
 * This file MUST NOT define `statement`.
 *
 * ============================================================================
 * HANDLER INTEGRATION
 * ============================================================================
 *
 * `grammar/effects/effect-handling.g4` owns handler syntax.
 *
 * Handler patterns may consume:
 *
 *     effectOperationReference
 *
 * but must not redefine it.
 *
 * This permits:
 *
 *     handle computation {
 *         case quantum::Measurement(q) => result
 *     }
 *
 * without duplicating qualified-name operation syntax.
 *
 * ============================================================================
 * EFFECT-SET INTEGRATION
 * ============================================================================
 *
 * `grammar/effects/effect-sets.g4` owns:
 *
 *     effectReference
 *     effectReferenceList
 *     effectSet
 *
 * This file does not redefine effect sets.
 *
 * A reference in an effect set:
 *
 *     effects { quantum::Measurement }
 *
 * and an invocation:
 *
 *     perform quantum::Measurement(q)
 *
 * share the same qualified-name foundation but have different syntactic
 * ownership.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser must expose enough structure for the frontend AST to represent:
 *
 *     operation identity;
 *     operation arguments;
 *     source span;
 *     source ordering;
 *     invocation context.
 *
 * Conceptually:
 *
 *     EffectOperationUse
 *         operation: QualifiedName
 *         arguments: Expression[]
 *         span: SourceSpan
 *
 * The exact AST type remains owned by:
 *
 *     src/frontend/ast/
 *
 * This grammar MUST NOT introduce:
 *
 *     QuantumGate
 *     PhysicalQubit
 *     GPUOperation
 *     CPUInstruction
 *     VendorOperation
 *
 * or any other backend-specific AST type.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing proves structural validity only.
 *
 * Semantic analysis determines:
 *
 *     - whether the operation resolves;
 *     - whether the operation belongs to a declared effect;
 *     - whether the operation is imported;
 *     - whether the operation is accessible;
 *     - whether the operation version is valid;
 *     - whether arguments satisfy the operation signature;
 *     - what result type is produced;
 *     - which effects are propagated;
 *     - which capabilities are required;
 *     - which resources are required;
 *     - whether a handler discharges the effect;
 *     - whether the operation crosses a domain boundary;
 *     - whether a quantum operation must lower to quantum::ir;
 *     - whether an HDL/hardware operation can be realized.
 *
 * None of these decisions belong in this grammar.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * Effect-operation syntax does not itself constitute an IR.
 *
 * The downstream representation may contain semantic effect metadata.
 *
 * The expected direction is:
 *
 *     EffectOperationUse
 *         |
 *         v
 *     semantic effect operation
 *         |
 *         v
 *     canonical semantic representation / ZUIR
 *         |
 *         +--> classical IR
 *         +--> quantum::ir
 *         +--> HDL/hardware representation
 *         +--> distributed representation
 *         +--> accelerator representation
 *         +--> future domain representation
 *
 * The grammar MUST NOT construct any of these representations.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing syntax:
 *
 *     perform IO::read(path)
 *
 * remains supported.
 *
 * Existing effect names remain open-world.
 *
 * Existing qualified names remain governed by the canonical name grammar.
 *
 * No vendor-specific operation list is introduced.
 *
 * No fixed quantum gate list is introduced.
 *
 * No physical-device syntax is introduced.
 *
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * Parser-level malformed syntax includes:
 *
 *     perform
 *     perform ;
 *     perform ::
 *     perform IO::
 *     perform IO::read(
 *     perform IO::read(,)
 *     perform IO::read(a b)
 *
 * Semantic diagnostics include:
 *
 *     unknown effect;
 *     unknown operation;
 *     inaccessible operation;
 *     wrong argument count;
 *     invalid argument type;
 *     invalid result context;
 *     unavailable capability;
 *     unavailable resource;
 *     unsupported target;
 *     invalid quantum semantic operation.
 *
 * The latter categories MUST be diagnosed downstream.
 *
 * ============================================================================
 * SOURCE SPAN CONTRACT
 * ============================================================================
 *
 * The parser context must preserve source locations through the normal ANTLR
 * token/context mechanism.
 *
 * The frontend must be able to associate the complete operation use with a
 * source span covering:
 *
 *     perform
 *     operation identity
 *     argument list
 *
 * without inventing a second source-location system.
 *
 * ============================================================================
 * ROUND-TRIP CONTRACT
 * ============================================================================
 *
 * If the formatter/printer supports this syntax:
 *
 *     source
 *       -> lexer
 *       -> parser
 *       -> AST
 *       -> formatter
 *       -> parser
 *
 * must preserve operation identity and argument meaning.
 *
 * Source spelling normalization may be permitted according to the language
 * formatter contract.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file passes the hard-coding rule only if it contains no universal
 * implementation limits.
 *
 * Forbidden examples:
 *
 *     MAX_EFFECT_OPERATIONS = ...
 *     MAX_EFFECT_ARGUMENTS = ...
 *     MAX_QUBITS = ...
 *     MAX_CPUS = ...
 *     MAX_GPUS = ...
 *     MAX_FPGAS = ...
 *     MAX_NODES = ...
 *     DEVICE_0
 *     QPU_0
 *     GPU_0
 *     CPU_0
 *
 * Numeric literals appearing in user source are program data and are not
 * language limits.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive tests:
 *
 *     perform IO::read(path);
 *     perform IO::flush;
 *     perform Storage::write(key, value);
 *     perform quantum::Measurement(q);
 *     perform quantum::Reset(q);
 *     perform qec::Correction(state);
 *     perform zqn::Observation(signal);
 *     perform accelerator::TensorCompute(tensor);
 *     perform distributed::Consensus(value);
 *     perform future::domain::operation(value);
 *
 * Zero-argument:
 *
 *     perform IO::flush;
 *
 * Single argument:
 *
 *     perform IO::read(path);
 *
 * Multiple arguments:
 *
 *     perform Network::send(endpoint, message);
 *
 * Nested expressions:
 *
 *     perform IO::write(transform(a + b));
 *
 * Generic/structured expression arguments:
 *
 *     perform accelerator::compute<T>(value);
 *
 * where the canonical expression grammar permits the corresponding syntax.
 *
 * Negative syntax tests:
 *
 *     perform;
 *     perform ;
 *     perform ::
 *     perform IO::
 *     perform IO::read(
 *     perform IO::read(,)
 *
 * Boundary tests:
 *
 *     empty argument list;
 *     one argument;
 *     many arguments;
 *     long qualified names;
 *     deeply nested expressions;
 *     nested effect operations;
 *     multiple effect operations in one block;
 *     nested handlers.
 *
 * Cross-domain tests:
 *
 *     classical;
 *     quantum;
 *     hybrid;
 *     HDL;
 *     hardware;
 *     distributed;
 *     AI;
 *     data;
 *     networking;
 *     security;
 *     accelerators;
 *     future domains.
 *
 * Scalability tests:
 *
 *     no artificial effect count;
 *     no artificial operation count;
 *     no artificial argument count;
 *     no artificial namespace depth;
 *     no artificial machine size;
 *     no artificial resource size.
 *
 * Determinism tests:
 *
 *     identical token streams produce equivalent parse trees.
 *
 * Compatibility tests:
 *
 *     legacy supported `perform <qualified-name>(...)` forms remain accepted.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [ ] It compiles as an ANTLR4 parser grammar.
 *     [ ] It consumes the canonical ZamaniTokens vocabulary.
 *     [ ] It imports canonical Core/Expressions syntax.
 *     [ ] It defines no lexical rules.
 *     [ ] It defines no identifier grammar.
 *     [ ] It defines no qualified-name grammar.
 *     [ ] It defines no type grammar.
 *     [ ] It defines no general expression grammar.
 *     [ ] It defines no effect declarations.
 *     [ ] It defines no effect sets.
 *     [ ] It defines no effect handlers.
 *     [ ] It defines no capabilities.
 *     [ ] It defines no resources.
 *     [ ] It defines no hardware topology.
 *     [ ] It defines no quantum gate enumeration.
 *     [ ] It defines no quantum IR.
 *     [ ] It defines no QEC.
 *     [ ] It defines no ZQN.
 *     [ ] It defines no routing.
 *     [ ] It defines no scheduling.
 *     [ ] It defines no runtime dispatch.
 *     [ ] It has no machine-size limits.
 *     [ ] It remains open-world.
 *     [ ] It preserves source-level operation identity.
 *     [ ] It provides a stable integration point for expression grammar.
 *     [ ] It provides a stable integration point for statement grammar.
 *     [ ] It provides a stable integration point for handler grammar.
 *     [ ] Positive tests pass.
 *     [ ] Negative tests pass.
 *     [ ] Boundary tests pass.
 *     [ ] Cross-domain tests pass.
 *     [ ] Scalability tests pass.
 *     [ ] Determinism tests pass.
 *     [ ] Compatibility tests pass.
 *     [ ] Rust-generated frontend remains compatible with Rust 1.97/1.97.1.
 *     [ ] No unsafe code is required.
 *
 * ============================================================================
 * CANONICAL IMPLEMENTATION
 * ============================================================================
 */

parser grammar EffectOperations;

options {
    tokenVocab = ZamaniTokens;
}

import Core, Expressions;


/*
 * ============================================================================
 * 1. EFFECT OPERATION REFERENCE
 * ============================================================================
 *
 * A reference identifies an operation without invoking it.
 *
 * Examples:
 *
 *     IO::read
 *     quantum::Measurement
 *     future::domain::operation
 */

effectOperationReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * 2. EFFECT INVOCATION
 * ============================================================================
 *
 * An invocation consists of:
 *
 *     operation identity
 *     optional argument list
 *
 * The operation itself is always represented by a canonical qualified name.
 *
 * Examples:
 *
 *     IO::flush
 *     IO::read(path)
 *     quantum::Measurement(q)
 */

effectInvocation
    : effectOperationReference
      effectInvocationArguments?
    ;


/*
 * ============================================================================
 * 3. EFFECT INVOCATION ARGUMENTS
 * ============================================================================
 *
 * The actual argument expressions remain owned by the canonical expression
 * grammar.
 *
 * This rule is only a syntactic boundary around the shared argument list.
 */

effectInvocationArguments
    : LPAREN
      argumentList?
      RPAREN
    ;


/*
 * ============================================================================
 * 4. EFFECT OPERATION CALL
 * ============================================================================
 *
 * Named integration point for tools and expression grammars that need to
 * distinguish an operation call from a bare operation reference.
 */

effectOperationCall
    : effectInvocation
    ;


/*
 * ============================================================================
 * 5. EFFECT OPERATION USE
 * ============================================================================
 *
 * This is the reusable operation-use construct.
 *
 * It deliberately does not introduce a statement or expression hierarchy.
 *
 * Higher-level grammars decide whether the operation is used as:
 *
 *     an expression;
 *     a statement;
 *     a handler pattern;
 *     another effect construct.
 */

effectOperationUse
    : effectOperationCall
    ;


/*
 * ============================================================================
 * 6. PERFORM EFFECT OPERATION
 * ============================================================================
 *
 * Canonical source form:
 *
 *     perform IO::read(path)
 *
 *     perform quantum::Measurement(q)
 *
 *     perform distributed::Consensus(value)
 *
 * The `perform` keyword establishes effectful source semantics.
 *
 * Runtime execution remains downstream.
 */

performEffectOperation
    : K_PERFORM
      effectOperationUse
    ;


/*
 * ============================================================================
 * 7. PERFORM EFFECT OPERATION STATEMENT
 * ============================================================================
 *
 * Statement-level adapter.
 *
 * The universal statement grammar remains responsible for deciding where
 * effect statements are legal.
 */

performEffectOperationStatement
    : performEffectOperation
      SEMICOLON?
    ;
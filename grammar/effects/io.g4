/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/effects/io.g4
 *
 * Status:
 *     Production modular grammar for the IO effect domain.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Runtime/compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
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
 * This file provides the IO-specific syntactic composition layer of Zamani's
 * general effect system.
 *
 * IMPORTANT:
 *
 * This file does NOT define a closed list of IO operations.
 *
 * It deliberately does NOT make any of these grammar keywords:
 *
 *     io
 *     read
 *     write
 *     open
 *     close
 *     stdin
 *     stdout
 *     stderr
 *     file
 *     socket
 *     pipe
 *     stream
 *     console
 *     device
 *     serial
 *
 * Those are domain names and/or effect operation names, not fundamental
 * language syntax.
 *
 * For example, all of the following remain possible without modifying this
 * grammar:
 *
 *     effect IO;
 *
 *     effect io::File;
 *
 *     effect io::Stream;
 *
 *     effect io::Console;
 *
 *     effect io::Device;
 *
 *     effect io::Database;
 *
 *     effect io::CustomTransport;
 *
 *     effect application::custom_io;
 *
 * The meaning of those names belongs to semantic analysis, capability
 * analysis, resource analysis, and the appropriate runtime/backend.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 * Source
 *   |
 *   v
 * ZamaniTokens
 *   |
 *   v
 * Core / Types / Expressions
 *   |
 *   v
 * Effect grammar
 *   |
 *   +--> effects.g4
 *   +--> effect-declarations.g4
 *   +--> effect-sets.g4
 *   +--> effect-handling.g4
 *   +--> capabilities.g4
 *   |
 *   v
 * Frontend AST
 *   |
 *   +--> name resolution
 *   +--> type checking
 *   +--> effect checking
 *   +--> capability checking
 *   +--> resource checking
 *   |
 *   v
 * Canonical semantic representation
 *   |
 *   +--> classical IR
 *   +--> quantum::ir
 *   +--> HDL/hardware representation
 *   +--> distributed representation
 *   |
 *   v
 * optimization / routing / scheduling / resilience
 *   |
 *   v
 * target lowering
 *   |
 *   v
 * runtime / hardware
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - IO-domain syntax composition;
 *   - syntax for explicitly classifying an operation as IO;
 *   - IO operation references;
 *   - IO operation invocation composition;
 *   - IO effect-set composition;
 *   - IO handler composition;
 *   - IO operation declaration composition;
 *   - source-level IO mode/specifier syntax;
 *   - syntactic IO effect aliases/compositions where supported by the
 *     aggregate grammar.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - the generic effect system;
 *   - effect declaration fundamentals;
 *   - generic effect sets;
 *   - generic effect handlers;
 *   - identifier spelling;
 *   - qualified-name spelling;
 *   - expressions;
 *   - argument lists;
 *   - types;
 *   - functions;
 *   - modules;
 *   - filesystem semantics;
 *   - networking semantics;
 *   - device semantics;
 *   - file descriptors;
 *   - sockets;
 *   - paths;
 *   - OS handles;
 *   - memory allocation;
 *   - buffering;
 *   - encoding;
 *   - permissions;
 *   - authentication;
 *   - capabilities;
 *   - resource allocation;
 *   - scheduling;
 *   - execution;
 *   - runtime dispatch;
 *   - backend selection;
 *   - hardware discovery;
 *   - CPU/GPU/QPU selection;
 *   - quantum IR;
 *   - QEC;
 *   - ZQN;
 *   - resilience.
 *
 * ============================================================================
 * OPEN-WORLD CONTRACT
 * ============================================================================
 *
 * IO is intentionally NOT a closed enumeration.
 *
 * The grammar therefore does not contain rules such as:
 *
 *     ioRead
 *     ioWrite
 *     ioOpen
 *     ioClose
 *
 * with fixed keywords.
 *
 * Instead, an IO operation is identified structurally by a normal qualified
 * source name.
 *
 * Examples:
 *
 *     io::read
 *     io::write
 *     io::file::open
 *     io::stream::receive
 *     io::database::query
 *     custom::transport::send
 *
 * This allows future IO domains without grammar modification.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * IO syntax expresses COMPUTATIONAL INTENT.
 *
 * It does not select:
 *
 *     - an operating system;
 *     - a filesystem;
 *     - a device;
 *     - a file descriptor;
 *     - a socket implementation;
 *     - a network interface;
 *     - a CPU;
 *     - a GPU;
 *     - an FPGA;
 *     - a QPU;
 *     - a machine topology;
 *     - a memory capacity;
 *     - a buffer size;
 *     - a node count.
 *
 * Therefore:
 *
 *     perform io::read(source)
 *
 * expresses an IO operation but does not prescribe where or how that operation
 * must be realized.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This grammar imposes no language-level limits on:
 *
 *     - number of IO effects;
 *     - number of IO operations;
 *     - number of operation arguments;
 *     - number of handlers;
 *     - number of effect references;
 *     - number of nested IO expressions;
 *     - number of IO domains;
 *     - program size;
 *     - data size;
 *     - device count;
 *     - node count;
 *     - machine count.
 *
 * Repetition is represented using ANTLR repetition operators.
 *
 * Resource limits, parser limits, memory limits, execution limits, and backend
 * limits belong to explicit compiler/runtime resource policies.
 *
 * ============================================================================
 * EFFECT / CAPABILITY / RESOURCE SEPARATION
 * ============================================================================
 *
 * Effect:
 *
 *     Describes what kind of computational interaction may occur.
 *
 * Capability:
 *
 *     Describes what an execution environment is able or authorized to provide.
 *
 * Resource:
 *
 *     Describes computational resources available to or requested by an
 *     execution.
 *
 * Constraint:
 *
 *     Describes conditions that must be satisfied.
 *
 * Preference:
 *
 *     Describes a preferred but not necessarily mandatory realization.
 *
 * This file only describes IO syntax.
 *
 * It must never silently convert:
 *
 *     IO effect
 *
 * into:
 *
 *     specific resource
 *
 * or:
 *
 *     specific device.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend AST should preserve:
 *
 *     - source span;
 *     - qualified IO operation name;
 *     - argument expressions;
 *     - optional IO mode;
 *     - optional operation metadata;
 *     - syntactic effect relationship.
 *
 * The grammar must not force the AST to contain:
 *
 *     - OS file descriptors;
 *     - physical device IDs;
 *     - memory addresses;
 *     - sockets;
 *     - backend IDs;
 *     - thread IDs;
 *     - machine IDs.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar does not create an IR.
 *
 * After parsing:
 *
 *     AST
 *       ->
 *     semantic analysis
 *       ->
 *     canonical effect representation
 *       ->
 *     appropriate domain IR
 *
 * IO may eventually lower into:
 *
 *     classical IR
 *     distributed IR
 *     networking IR
 *     hardware IR
 *     accelerator IR
 *
 * depending on semantic meaning.
 *
 * The grammar must never create a second IR for IO.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * IO can occur in hybrid quantum-classical programs.
 *
 * Examples include:
 *
 *     perform io::read(input);
 *     perform quantum::measure(q);
 *     perform io::write(result);
 *
 * The IO grammar must remain independent of quantum semantics.
 *
 * In particular, this file does NOT define:
 *
 *     QubitId
 *     PhysicalQubitId
 *     GateKind
 *     quantum topology
 *     measurement semantics
 *     QEC
 *     ZQN
 *
 * Quantum semantic lowering remains under the canonical:
 *
 *     quantum::ir
 *
 * boundary.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * IO can also represent hardware/software interaction:
 *
 *     perform io::device::read(channel);
 *
 * or:
 *
 *     perform hardware::interface::transfer(data);
 *
 * The grammar does not decide whether such an operation becomes:
 *
 *     CPU code
 *     DMA
 *     FPGA logic
 *     ASIC logic
 *     bus transaction
 *     network transfer
 *     accelerator command
 *
 * Those decisions belong downstream.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar:
 *
 *     - performs no IO itself;
 *     - contains no actions;
 *     - contains no semantic predicates;
 *     - contains no random behavior;
 *     - contains no target-dependent branches;
 *     - contains no runtime calls.
 *
 * Parsing is therefore independent of the machine on which the parser runs.
 *
 * ============================================================================
 * ANTLR COMPOSITION CONTRACT
 * ============================================================================
 *
 * Canonical lexer:
 *
 *     grammar/lexer/tokens.g4
 *
 * Lexer grammar:
 *
 *     ZamaniTokens
 *
 * Canonical parser dependencies:
 *
 *     Core
 *     Types
 *     Expressions
 *
 * This file deliberately consumes those shared rules instead of redefining
 * them.
 *
 * ============================================================================
 */

parser grammar IO;

options {
    tokenVocab = ZamaniTokens;
}

import Core, Types, Expressions;


/*
 * ============================================================================
 * 1. IO DOMAIN REFERENCE
 * ============================================================================
 *
 * An IO domain is a qualified source-level name.
 *
 * Examples:
 *
 *     io
 *     io::file
 *     io::stream
 *     application::input
 *     device::serial
 *
 * No particular name is reserved by this grammar.
 */
ioDomainReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * 2. IO OPERATION REFERENCE
 * ============================================================================
 *
 * Examples:
 *
 *     io::read
 *     io::write
 *     io::file::open
 *     io::stream::receive
 *     custom::transport::send
 *
 * The grammar does not enumerate operation names.
 */
ioOperationReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * 3. IO OPERATION ARGUMENTS
 * ============================================================================
 *
 * IO arguments are ordinary Zamani expressions.
 *
 * This is essential for composability:
 *
 *     perform io::write(buffer);
 *
 *     perform io::write(transform(data));
 *
 *     perform io::read(target[index]);
 *
 * The IO grammar does not create a second expression language.
 */
ioOperationArguments
    : LPAREN
      argumentList?
      RPAREN
    ;


/*
 * ============================================================================
 * 4. IO OPERATION INVOCATION
 * ============================================================================
 *
 * An IO invocation is a source-level operation reference followed by optional
 * arguments.
 *
 * Examples:
 *
 *     io::read(input)
 *     io::write(output)
 *     io::file::open(path)
 *
 * Whether the operation exists is a semantic question.
 */
ioOperationInvocation
    : ioOperationReference
      ioOperationArguments?
    ;


/*
 * ============================================================================
 * 5. IO PERFORM EXPRESSION
 * ============================================================================
 *
 * Explicit effect execution:
 *
 *     perform io::read(input)
 *     perform io::write(output)
 *
 * `perform` is already a generic effect-system keyword.
 *
 * This rule only constrains the operand to the IO-domain operation form.
 */
ioPerformExpression
    : K_PERFORM
      ioOperationInvocation
    ;


/*
 * ============================================================================
 * 6. IO PERFORM STATEMENT
 * ============================================================================
 */
ioPerformStatement
    : ioPerformExpression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 7. IO EFFECT REFERENCE
 * ============================================================================
 *
 * Examples:
 *
 *     IO
 *     io
 *     io::File
 *     io::Network
 *     application::Input
 *
 * No built-in effect name is imposed.
 */
ioEffectReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * 8. IO EFFECT LIST
 * ============================================================================
 *
 * Example:
 *
 *     with effects {
 *         io::read,
 *         io::write
 *     }
 *
 * The list has no fixed size.
 */
ioEffectReferenceList
    : ioEffectReference
      (COMMA ioEffectReference)*
      COMMA?
    ;


/*
 * ============================================================================
 * 9. IO EFFECT SET
 * ============================================================================
 */
ioEffectSet
    : LBRACE
      ioEffectReferenceList?
      RBRACE
    ;


/*
 * ============================================================================
 * 10. IO EFFECT CLAUSE
 * ============================================================================
 *
 * Example:
 *
 *     with effects {
 *         io::read,
 *         io::write
 *     }
 *
 * This remains syntactic information only.
 */
ioEffectClause
    : K_WITH
      EFFECTS
      ioEffectSet
    ;


/*
 * ============================================================================
 * 11. IO OPERATION DECLARATION
 * ============================================================================
 *
 * This permits domain-specific IO effect declarations while reusing the
 * canonical type/expression grammar.
 *
 * Example:
 *
 *     effect io::Stream {
 *         fn receive(buffer: Buffer) -> Result;
 *     }
 *
 * IMPORTANT:
 *
 * The surrounding generic effect declaration is owned by
 * effect-declarations.g4.
 *
 * This rule only defines the operation-signature composition used by an IO
 * domain.
 */
ioOperationDeclaration
    : ioOperationAttributes*
      K_FN
      identifier
      ioGenericParameters?
      LPAREN
      ioParameterList?
      RPAREN
      ioReturnClause?
      ioWhereClause?
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 12. IO OPERATION ATTRIBUTES
 * ============================================================================
 *
 * `async` is a source-level declaration modifier.
 *
 * It does not mean:
 *
 *     one thread
 *     one core
 *     one queue
 *     one CPU
 *     one device.
 */
ioOperationAttributes
    : K_ASYNC
    | attribute
    ;


/*
 * ============================================================================
 * 13. IO PARAMETER LIST
 * ============================================================================
 */
ioParameterList
    : ioParameter
      (COMMA ioParameter)*
      COMMA?
    ;


/*
 * ============================================================================
 * 14. IO PARAMETER
 * ============================================================================
 *
 * Parameters use normal Zamani types.
 */
ioParameter
    : ioParameterModifier*
      identifier
      ioParameterType?
      ioParameterDefault?
    ;


/*
 * ============================================================================
 * 15. IO PARAMETER MODIFIER
 * ============================================================================
 */
ioParameterModifier
    : K_MUT
    ;


/*
 * ============================================================================
 * 16. IO PARAMETER TYPE
 * ============================================================================
 */
ioParameterType
    : COLON
      typeExpression
    ;


/*
 * ============================================================================
 * 17. IO PARAMETER DEFAULT
 * ============================================================================
 *
 * Default values are ordinary Zamani expressions.
 */
ioParameterDefault
    : EQUALS
      expression
    ;


/*
 * ============================================================================
 * 18. IO RETURN CLAUSE
 * ============================================================================
 */
ioReturnClause
    : THIN_ARROW
      typeExpression
    ;


/*
 * ============================================================================
 * 19. IO GENERIC PARAMETERS
 * ============================================================================
 *
 * There is deliberately no finite generic arity.
 */
ioGenericParameters
    : LESS_THAN
      ioGenericParameter
      (COMMA ioGenericParameter)*
      COMMA?
      GREATER_THAN
    ;


/*
 * ============================================================================
 * 20. IO GENERIC PARAMETER
 * ============================================================================
 */
ioGenericParameter
    : identifier
      ioGenericBounds?
    ;


/*
 * ============================================================================
 * 21. IO GENERIC BOUNDS
 * ============================================================================
 */
ioGenericBounds
    : COLON
      ioGenericBound
      (PLUS ioGenericBound)*
    ;


/*
 * ============================================================================
 * 22. IO GENERIC BOUND
 * ============================================================================
 *
 * Bounds are type expressions.
 */
ioGenericBound
    : typeExpression
    ;


/*
 * ============================================================================
 * 23. IO WHERE CLAUSE
 * ============================================================================
 *
 * Delegates constraint syntax to the canonical shared grammar.
 */
ioWhereClause
    : whereClause
    ;


/*
 * ============================================================================
 * 24. IO HANDLER PATTERN
 * ============================================================================
 *
 * Examples:
 *
 *     io::read(value)
 *     io::write(value)
 *     io::file::open(path)
 *
 * The operation name remains open-world.
 */
ioHandlerPattern
    : ioOperationReference
      ioOperationArguments?
    ;


/*
 * ============================================================================
 * 25. IO HANDLER ARM
 * ============================================================================
 *
 * Example:
 *
 *     case io::read(source) => resume(value)
 *
 * Handler semantics are owned by effect-handling/semantic analysis.
 */
ioHandlerArm
    : CASE
      ioHandlerPattern
      FAT_ARROW
      (
          blockExpression
        | expression
      )
      COMMA?
    ;


/*
 * ============================================================================
 * 26. IO HANDLER
 * ============================================================================
 */
ioHandler
    : LBRACE
      ioHandlerArm*
      RBRACE
    ;


/*
 * ============================================================================
 * 27. IO HANDLING EXPRESSION
 * ============================================================================
 *
 * Example:
 *
 *     handle computation {
 *         case io::read(source) => resume(value)
 *     }
 *
 * `handle` itself belongs to the general effect language.
 */
ioHandleExpression
    : K_HANDLE
      expression
      ioHandler
    ;


/*
 * ============================================================================
 * 28. IO RESOURCE OPERATION
 * ============================================================================
 *
 * An IO resource operation may have a resource expression as its first
 * argument, but the grammar does not assume what that resource represents.
 *
 * Examples:
 *
 *     io::read(resource, buffer)
 *     io::write(resource, data)
 *
 * This rule intentionally remains structural.
 */
ioResourceOperation
    : ioOperationReference
      ioOperationArguments
    ;


/*
 * ============================================================================
 * 29. IO ASYNCHRONOUS OPERATION
 * ============================================================================
 *
 * Async syntax is represented through the normal expression language.
 *
 * This rule does not prescribe a scheduling implementation.
 */
ioAsyncOperation
    : K_ASYNC
      ioOperationInvocation
    ;


/*
 * ============================================================================
 * 30. IO COMPOSITION
 * ============================================================================
 *
 * Allows an aggregate grammar to consume any IO-domain effect construct
 * without introducing a closed IO catalogue.
 */
ioConstruct
    : ioPerformStatement
    | ioHandleExpression
    | ioOperationDeclaration
    | ioEffectClause
    | ioAsyncOperation
    ;


/*
 * ============================================================================
 * ARCHITECTURAL INVARIANTS
 * ============================================================================
 *
 * The following invariants MUST remain true:
 *
 * 1. No fixed IO operation catalogue.
 *
 * 2. No K_IO token is required.
 *
 * 3. No filesystem-specific syntax is required.
 *
 * 4. No operating-system-specific syntax is required.
 *
 * 5. No device IDs are encoded.
 *
 * 6. No resource counts are encoded.
 *
 * 7. No memory capacities are encoded.
 *
 * 8. No machine topology is encoded.
 *
 * 9. No runtime calls are made from grammar actions.
 *
 * 10. No unsafe Rust is introduced.
 *
 * 11. IO does not define a second expression language.
 *
 * 12. IO does not define an IR.
 *
 * 13. IO does not define capability semantics.
 *
 * 14. IO does not define resource semantics.
 *
 * 15. IO does not define hardware semantics.
 *
 * 16. IO does not define quantum semantics.
 *
 * 17. IO remains composable with classical, quantum, HDL, distributed,
 *     networking, AI, accelerator, and future domains.
 *
 * 18. The grammar remains valid regardless of the number of available
 *     machines/resources.
 *
 * 19. The grammar remains valid when new IO domains are introduced.
 *
 * 20. New IO operation names must not require grammar modification.
 *
 * ============================================================================
 */
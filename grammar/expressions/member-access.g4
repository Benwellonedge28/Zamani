/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/expressions/member-access.g4
 *
 * Status:
 *     Canonical production member-access syntax contract.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar.
 *
 * Rust implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021 edition.
 *
 * Safety:
 *     The Zamani compiler implementation MUST use safe Rust only.
 *     No `unsafe` Rust is required or permitted by the compiler implementation.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the syntax of postfix member and qualified-member selection.
 *
 * It provides a reusable postfix suffix which can be composed by the
 * canonical postfix-expression grammar.
 *
 * Conceptually:
 *
 *     base
 *       |
 *       +--> . member
 *       |
 *       +--> :: member
 *
 * Examples:
 *
 *     value.field
 *     object.method
 *     tensor.shape
 *     circuit.operations
 *     module.item
 *     namespace.symbol
 *     type::associated_item
 *     value.field.method
 *     value.field[index]
 *     value.method(arg).field
 *
 * Chaining is deliberately owned by the enclosing postfix grammar rather than
 * recursively by this file.
 *
 * ============================================================================
 * ARCHITECTURAL PRINCIPLE
 * ============================================================================
 *
 * This grammar describes SOURCE SYNTAX ONLY.
 *
 * It does NOT decide what a member means.
 *
 * Semantic analysis determines whether a selected name denotes:
 *
 *     - a field;
 *     - a property;
 *     - a method;
 *     - an associated function;
 *     - an associated constant;
 *     - a type member;
 *     - a namespace member;
 *     - a module member;
 *     - a trait/interface member;
 *     - a capability;
 *     - a resource;
 *     - a hardware abstraction;
 *     - a quantum object;
 *     - an HDL object;
 *     - a distributed service;
 *     - an AI/data object;
 *     - a dialect-defined member;
 *     - another future semantic entity.
 *
 * This file MUST NOT perform name resolution.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - member-access suffix syntax;
 *     - dot member selection;
 *     - double-colon qualified/associated selection;
 *     - the member-name syntactic boundary;
 *     - source-order preservation of the selected name;
 *     - the integration contract consumed by postfix expressions.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - identifiers;
 *     - lexical token definitions;
 *     - qualified-name declarations;
 *     - modules;
 *     - namespaces;
 *     - types;
 *     - fields;
 *     - methods;
 *     - functions;
 *     - calls;
 *     - indexing;
 *     - operators;
 *     - expression precedence;
 *     - semantic name resolution;
 *     - visibility;
 *     - inheritance;
 *     - traits;
 *     - interfaces;
 *     - overload resolution;
 *     - generic inference;
 *     - ownership;
 *     - borrowing;
 *     - effects;
 *     - capabilities;
 *     - resource allocation;
 *     - hardware discovery;
 *     - quantum-device discovery;
 *     - routing;
 *     - scheduling;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - compiler backend selection;
 *     - runtime dispatch.
 *
 * ============================================================================
 * LEXER AUTHORITY
 * ============================================================================
 *
 * The canonical lexical authority is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * This grammar therefore consumes the existing canonical tokens:
 *
 *     DOT
 *     DOUBLE_COLON
 *     IDENTIFIER
 *
 * It MUST NOT redefine any lexer token.
 *
 * In particular, this file deliberately does NOT invent:
 *
 *     DOT_TOKEN
 *     MEMBER_DOT
 *     SCOPE
 *     QUESTION_MARK
 *     LESS
 *     GREATER
 *     MEMBER_IDENTIFIER
 *
 * The existing lexer defines:
 *
 *     DOUBLE_COLON : '::' ;
 *     DOT          : '.' ;
 *
 * and:
 *
 *     IDENTIFIER
 *
 * as the general identifier token.
 *
 * ============================================================================
 * TOKEN / GRAMMAR INTEGRATION
 * ============================================================================
 *
 * The current canonical lexer exposes:
 *
 *     DOT
 *     DOUBLE_COLON
 *     QUESTION
 *
 * but does not expose a separate QUESTION_MARK token.
 *
 * Therefore this file MUST use:
 *
 *     DOT
 *     DOUBLE_COLON
 *
 * for member selection.
 *
 * Optional/null-aware member access is intentionally NOT added here.
 *
 * If Zamani later standardizes:
 *
 *     value?.field
 *
 * it must first be specified as a language feature, assigned a stable lexer
 * contract, assigned semantic and AST contracts, and then integrated into the
 * postfix grammar.
 *
 * This file must not silently invent that feature.
 *
 * ============================================================================
 * PUBLIC CONTRACT
 * ============================================================================
 *
 * The public suffix rule is:
 *
 *     memberAccessSuffix
 *
 * It represents exactly one member-selection operation.
 *
 * Examples:
 *
 *     .field
 *     .method
 *     ::Item
 *     ::AssociatedType
 *
 * It does NOT consume the base expression.
 *
 * Therefore:
 *
 *     value.field
 *
 * is composed conceptually as:
 *
 *     postfixExpression
 *         -> primaryExpression
 *         -> memberAccessSuffix
 *
 * ============================================================================
 * DOT MEMBER ACCESS
 * ============================================================================
 *
 * Dot selection:
 *
 *     value.member
 *
 * is represented by:
 *
 *     DOT memberName
 *
 * This is the ordinary postfix/member-selection form.
 *
 * Examples:
 *
 *     object.field
 *     object.method
 *     tensor.shape
 *     matrix.rows
 *     signal.value
 *     circuit.operations
 *     service.request
 *     accelerator.run
 *
 * The grammar does not determine whether the selected member is callable.
 *
 * For example:
 *
 *     object.run
 *
 * and:
 *
 *     object.run(...)
 *
 * are syntactically composed by different postfix operations.
 *
 * Call parsing belongs to:
 *
 *     grammar/expressions/calls.g4
 *
 * ============================================================================
 * DOUBLE-COLON SELECTION
 * ============================================================================
 *
 * Double-colon selection:
 *
 *     owner::member
 *
 * is represented by:
 *
 *     DOUBLE_COLON memberName
 *
 * This supports source-level constructs such as:
 *
 *     Type::associated
 *     Namespace::item
 *     Module::item
 *     Trait::associated
 *     Resource::capability
 *
 * The grammar deliberately does not decide whether `::` means:
 *
 *     - namespace qualification;
 *     - module qualification;
 *     - associated-item selection;
 *     - type qualification;
 *     - trait qualification;
 *     - another language-defined semantic relation.
 *
 * That distinction belongs to name and semantic resolution.
 *
 * ============================================================================
 * MEMBER NAME
 * ============================================================================
 *
 * A member name is currently an ordinary canonical identifier:
 *
 *     IDENTIFIER
 *
 * This is intentional.
 *
 * The grammar does NOT create a second identifier grammar.
 *
 * Identifier spelling, Unicode policy, normalization policy, reserved words,
 * and lexical compatibility remain owned by the canonical lexer/specification.
 *
 * ============================================================================
 * KEYWORD MEMBERS
 * ============================================================================
 *
 * This file does not automatically permit reserved keywords as member names.
 *
 * For example:
 *
 *     value.type
 *
 * is accepted only if the lexer/parser vocabulary makes `type` available as
 * an identifier/member name in that syntactic position.
 *
 * The semantic system must not be forced to treat every keyword as a member
 * identifier.
 *
 * If Zamani eventually requires keyword member names, that must be handled
 * through an explicit lexical/specification contract rather than by adding
 * ad-hoc alternatives here.
 *
 * ============================================================================
 * CHAINING
 * ============================================================================
 *
 * This file intentionally parses ONE suffix:
 *
 *     .member
 *     ::member
 *
 * It does not recursively define:
 *
 *     memberAccessSuffix
 *         : memberAccessSuffix ...
 *
 * This avoids creating a second postfix-expression hierarchy.
 *
 * The canonical postfix grammar owns repetition:
 *
 *     postfixExpression
 *         : primaryExpression postfixPart*
 *         ;
 *
 * Therefore:
 *
 *     a.b.c.d
 *
 * is represented as an ordered postfix sequence:
 *
 *     a
 *       -> .b
 *       -> .c
 *       -> .d
 *
 * No maximum member-chain depth is encoded.
 *
 * ============================================================================
 * POCO-REAF / SCALABILITY
 * ============================================================================
 *
 * Member access introduces NO fixed machine or language resource limits.
 *
 * It does not encode:
 *
 *     MAX_MEMBERS
 *     MAX_MEMBER_DEPTH
 *     MAX_NAMESPACE_DEPTH
 *     MAX_MODULE_DEPTH
 *     MAX_TYPE_DEPTH
 *     MAX_RESOURCE_COUNT
 *     MAX_DEVICE_COUNT
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_THREADS
 *     MAX_TENSOR_RANK
 *
 * Therefore a program may contain a member chain whose practical size is
 * determined only by the implementation's available resources and explicitly
 * documented parser/compiler resource policies.
 *
 * "Infinity" here means:
 *
 *     the language grammar introduces no artificial finite maximum.
 *
 * It does NOT mean that an implementation has infinite memory, stack space,
 * compilation time, storage, or execution resources.
 *
 * ============================================================================
 * TARGET INDEPENDENCE
 * ============================================================================
 *
 * Member syntax must remain independent of the target machine.
 *
 * These are all ordinary source-level names:
 *
 *     cpu
 *     gpu
 *     qpu
 *     fpga
 *     accelerator
 *     memory
 *     network
 *     quantum
 *     tensor
 *
 * The presence of a member such as:
 *
 *     accelerator.compute
 *
 * does NOT select:
 *
 *     accelerator 0
 *     GPU 0
 *     QPU 0
 *     a physical device
 *     a physical memory bank
 *     a physical network node
 *
 * Target realization belongs downstream to:
 *
 *     semantic analysis
 *         ->
 *     resource/capability analysis
 *         ->
 *     canonical IR
 *         ->
 *     optimization
 *         ->
 *     routing/scheduling
 *         ->
 *     HAL/backend
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum entities use the same generic member-access mechanism.
 *
 * Examples:
 *
 *     circuit.operations
 *     circuit.measurements
 *     register.state
 *     logical_qubit.metadata
 *     operation.parameters
 *
 * The grammar MUST NOT enumerate quantum gates.
 *
 * It must NOT contain rules such as:
 *
 *     memberName
 *         : H
 *         | X
 *         | Y
 *         | Z
 *         | CNOT
 *         | ...
 *         ;
 *
 * Quantum operation names remain extensible identifiers unless explicitly
 * reserved by the canonical lexical specification.
 *
 * Quantum semantics remain downstream.
 *
 * The canonical quantum semantic boundary remains:
 *
 *     quantum::ir
 *
 * This grammar does not create:
 *
 *     QuantumMemberIR
 *     QuantumMemberAST
 *     QuantumGate enum
 *     PhysicalQubitMember
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Classical objects use the same member mechanism.
 *
 * Examples:
 *
 *     vector.length
 *     matrix.rows
 *     tensor.shape
 *     dataset.schema
 *     stream.next
 *
 * The grammar does not distinguish these semantic categories.
 *
 * ============================================================================
 * HDL INTEGRATION
 * ============================================================================
 *
 * HDL semantic objects may use the same member-selection syntax:
 *
 *     module.port
 *     module.signal
 *     module.clock
 *     interface.protocol
 *     pipeline.stage
 *
 * This grammar does not encode:
 *
 *     bit width;
 *     clock frequency;
 *     physical pin;
 *     FPGA resource;
 *     ASIC cell;
 *     placement;
 *     routing.
 *
 * Those belong to HDL/hardware semantic and lowering layers.
 *
 * ============================================================================
 * HARDWARE / RESOURCE INTEGRATION
 * ============================================================================
 *
 * Hardware and resource abstractions may expose members:
 *
 *     device.capabilities
 *     resource.capacity
 *     topology.links
 *     accelerator.memory
 *
 * The grammar remains target-independent.
 *
 * A member name does not imply a concrete resource exists.
 *
 * Capability/resource validation is performed after parsing.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Distributed objects may use:
 *
 *     service.endpoint
 *     cluster.members
 *     node.resources
 *     channel.metadata
 *
 * No fixed node count, topology, or endpoint count is represented here.
 *
 * ============================================================================
 * AI / DATA INTEGRATION
 * ============================================================================
 *
 * Generic member access supports:
 *
 *     model.parameters
 *     model.layers
 *     dataset.schema
 *     tensor.shape
 *     pipeline.stages
 *     agent.memory
 *
 * Framework-specific semantics do not belong in this grammar.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Every member-access suffix must lower into the existing domain-neutral
 * frontend AST.
 *
 * Conceptually:
 *
 *     MemberAccess {
 *         kind,
 *         name,
 *         source_span
 *     }
 *
 * where `kind` distinguishes the syntactic separator:
 *
 *     Dot
 *     DoubleColon
 *
 * The exact Rust representation is owned by:
 *
 *     src/frontend/ast/
 *
 * This grammar MUST NOT define a Rust AST type.
 *
 * The AST must preserve:
 *
 *     - selected member name;
 *     - separator kind;
 *     - complete source span;
 *     - source ordering;
 *     - enough information for diagnostics;
 *     - enough information for source-preserving tooling.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing answers:
 *
 *     "Is this syntactically a member-selection suffix?"
 *
 * Semantic analysis answers:
 *
 *     "What entity does this member select?"
 *
 * Semantic analysis owns:
 *
 *     - name resolution;
 *     - namespace resolution;
 *     - module resolution;
 *     - type-member resolution;
 *     - associated-item resolution;
 *     - field lookup;
 *     - method lookup;
 *     - property lookup;
 *     - visibility checking;
 *     - inheritance lookup;
 *     - trait/interface lookup;
 *     - generic substitution;
 *     - overload resolution;
 *     - capability resolution;
 *     - resource resolution;
 *     - quantum semantic validation;
 *     - HDL semantic validation;
 *     - hardware capability validation.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * Member access does not constitute an independent IR.
 *
 * It lowers through the normal frontend pipeline:
 *
 *     member syntax
 *         ->
 *     domain-neutral AST
 *         ->
 *     semantic model
 *         ->
 *     canonical IR
 *
 * Depending on semantic context, a member access may eventually contribute to:
 *
 *     - classical IR;
 *     - quantum::ir;
 *     - HDL/hardware IR;
 *     - data IR;
 *     - distributed IR;
 *     - another canonical domain representation.
 *
 * This file MUST NOT choose the final IR.
 *
 * ============================================================================
 * COMPILER INTEGRATION
 * ============================================================================
 *
 * Compiler passes may use the semantic meaning of a member access for:
 *
 *     - constant evaluation;
 *     - field projection;
 *     - method dispatch;
 *     - associated-item resolution;
 *     - capability access;
 *     - resource access;
 *     - optimization;
 *     - lowering;
 *     - dead-member elimination;
 *     - inlining;
 *     - domain-specific lowering.
 *
 * None of these operations belongs in the grammar.
 *
 * ============================================================================
 * RUNTIME INTEGRATION
 * ============================================================================
 *
 * Runtime behavior is determined only after semantic analysis and lowering.
 *
 * A member access may ultimately become:
 *
 *     - a field projection;
 *     - a method call target;
 *     - a resource lookup;
 *     - a capability query;
 *     - a runtime object operation;
 *     - a compile-time-resolved reference.
 *
 * Parsing MUST NEVER execute member access.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * Parsing:
 *
 *     object.secret
 *
 * must never inspect an actual object.
 *
 * Parsing:
 *
 *     system.execute
 *
 * must never execute a command.
 *
 * Parsing:
 *
 *     device.quantum
 *
 * must never contact a device.
 *
 * Parsing:
 *
 *     network.endpoint
 *
 * must never perform network access.
 *
 * Parsing:
 *
 *     file.contents
 *
 * must never open a file.
 *
 * The grammar is pure source analysis.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing must depend only on:
 *
 *     - source token sequence;
 *     - active grammar/language version.
 *
 * It MUST NOT depend on:
 *
 *     - system time;
 *     - randomness;
 *     - filesystem state;
 *     - network state;
 *     - environment variables;
 *     - installed hardware;
 *     - device availability;
 *     - scheduler state;
 *     - runtime state.
 *
 * The same source token sequence under the same language version must produce
 * the same parse structure.
 *
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * The parser should naturally diagnose malformed forms such as:
 *
 *     value.
 *     value..
 *     value::
 *     value::.
 *     .member
 *
 * depending on the surrounding expression context.
 *
 * Semantic diagnostics are deliberately outside this file.
 *
 * Examples of semantic errors that MUST NOT be encoded here:
 *
 *     unknown member;
 *     inaccessible member;
 *     private member access;
 *     invalid associated item;
 *     missing capability;
 *     unavailable hardware resource;
 *     invalid quantum operation;
 *     invalid HDL connection.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing Zamani syntax already recognizes:
 *
 *     value.member
 *     value::member
 *
 * through the canonical parser's member-expression contract.
 *
 * This file preserves those spellings.
 *
 * It therefore does not introduce a source-language rename or migration.
 *
 * The integration task is to replace the duplicated parser rule with this
 * reusable member-access contract once the modular expression composition is
 * enabled.
 *
 * ============================================================================
 * INTEGRATION WITH postfix.g4
 * ============================================================================
 *
 * `grammar/expressions/postfix.g4` owns postfix composition.
 *
 * It should consume this rule conceptually as:
 *
 *     postfixMember
 *         : memberAccessSuffix
 *         ;
 *
 * or directly:
 *
 *     postfixPart
 *         : memberAccessSuffix
 *         | ...
 *         ;
 *
 * The postfix grammar remains responsible for:
 *
 *     primaryExpression postfixPart*
 *
 * This file remains responsible for exactly one member-selection suffix.
 *
 * ============================================================================
 * INTEGRATION WITH expressions.g4
 * ============================================================================
 *
 * The canonical expression hierarchy owns precedence:
 *
 *     expression
 *       ->
 *     assignment
 *       ->
 *     ...
 *       ->
 *     prefix
 *       ->
 *     postfix
 *       ->
 *     primary
 *
 * This file MUST NOT define:
 *
 *     expression
 *     assignmentExpression
 *     prefixExpression
 *     postfixExpression
 *     primaryExpression
 *
 * It is a leaf-level postfix component.
 *
 * ============================================================================
 * INTEGRATION WITH calls.g4
 * ============================================================================
 *
 * Calls remain separate:
 *
 *     value.method(arg)
 *
 * structurally becomes:
 *
 *     primary/value
 *       -> .method
 *       -> (arg)
 *
 * Member access therefore MUST NOT consume:
 *
 *     LPAREN
 *     argumentList
 *     RPAREN
 *
 * Call syntax belongs to:
 *
 *     grammar/expressions/calls.g4
 *
 * ============================================================================
 * INTEGRATION WITH indexing.g4
 * ============================================================================
 *
 * Indexing remains separate:
 *
 *     value.field[index]
 *
 * structurally becomes:
 *
 *     value
 *       -> .field
 *       -> [index]
 *
 * Member access therefore MUST NOT consume:
 *
 *     LBRACKET
 *     expression
 *     RBRACKET
 *
 * Index syntax belongs to:
 *
 *     grammar/expressions/indexing.g4
 *
 * ============================================================================
 * INTEGRATION WITH names / modules
 * ============================================================================
 *
 * This file consumes one member identifier.
 *
 * Longer semantic paths remain representable through repeated postfix member
 * selection:
 *
 *     a::b::c
 *
 * or:
 *
 *     a.b.c
 *
 * The exact distinction between namespace qualification and associated-item
 * selection belongs to semantic analysis.
 *
 * ============================================================================
 * NO DUPLICATE QUALIFIED-NAME GRAMMAR
 * ============================================================================
 *
 * Do not add:
 *
 *     qualifiedName
 *         : IDENTIFIER (DOUBLE_COLON IDENTIFIER)*
 *         ;
 *
 * here.
 *
 * A qualified-name grammar belongs to the canonical name/path subsystem.
 *
 * This file represents postfix selection, not declaration/import path syntax.
 *
 * ============================================================================
 * FUTURE EXTENSIBILITY
 * ============================================================================
 *
 * Future language features MUST NOT be added here merely because they involve
 * a dot.
 *
 * Examples:
 *
 *     optional chaining;
 *     safe navigation;
 *     method references;
 *     namespace imports;
 *     reflective selection;
 *     dynamic member selection.
 *
 * Each feature requires its own:
 *
 *     specification;
 *     token contract, if necessary;
 *     AST contract;
 *     semantic contract;
 *     IR integration;
 *     diagnostics;
 *     compatibility policy;
 *     tests.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains:
 *
 *     no machine limits;
 *     no hardware IDs;
 *     no physical addresses;
 *     no qubit limits;
 *     no CPU limits;
 *     no GPU limits;
 *     no FPGA limits;
 *     no node limits;
 *     no thread limits;
 *     no tensor limits;
 *     no fixed member count;
 *     no fixed chain depth.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive syntax cases:
 *
 *     value.field
 *     value.method
 *     value::associated
 *     object.field.method
 *     object.field[index]
 *     object.method(arg)
 *     a.b.c.d
 *     a::b::c
 *     circuit.operations
 *     tensor.shape
 *     module.item
 *     device.capabilities
 *     service.endpoint
 *     model.parameters
 *     pipeline.stages
 *
 * Negative syntax cases:
 *
 *     value.
 *     value::
 *     value..field
 *     value::.field
 *     .field
 *
 * Boundary cases:
 *
 *     a.b
 *     a.b.c
 *     a.b.c.d
 *     a::b
 *     a::b::c
 *
 * Scalability cases:
 *
 *     very long member chains;
 *     generated member names;
 *     deeply nested postfix expressions;
 *     large source units.
 *
 * No test may establish an artificial universal maximum.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] Uses canonical ZamaniLexer tokens.
 *     [x] Defines only member-selection syntax.
 *     [x] Does not duplicate expression precedence.
 *     [x] Does not duplicate postfix composition.
 *     [x] Does not duplicate identifier syntax.
 *     [x] Supports `.` member selection.
 *     [x] Supports `::` qualified/associated selection.
 *     [x] Supports arbitrary chaining through postfix composition.
 *     [x] Does not impose finite member-chain limits.
 *     [x] Does not encode hardware limits.
 *     [x] Does not enumerate quantum operations.
 *     [x] Does not create a quantum IR.
 *     [x] Preserves target independence.
 *     [x] Defines AST integration in advance.
 *     [x] Defines semantic integration in advance.
 *     [x] Defines IR integration in advance.
 *     [x] Defines compiler integration in advance.
 *     [x] Defines runtime boundaries in advance.
 *     [x] Defines testing requirements in advance.
 *     [x] Does not require Rust `unsafe`.
 *
 * ============================================================================
 */

parser grammar member_access;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * PUBLIC MEMBER-ACCESS SUFFIX
 * ========================================================================== */

/**
 * Exactly one postfix member-selection operation.
 *
 * The base expression is owned by postfix.g4.
 *
 * Examples:
 *
 *     .field
 *     .method
 *     ::Item
 *     ::AssociatedType
 */
memberAccessSuffix
    : DOT memberName
    | DOUBLE_COLON memberName
    ;


/* ============================================================================
 * MEMBER NAME
 * ========================================================================== */

/**
 * A member name uses the canonical Zamani identifier token.
 *
 * No second identifier grammar is introduced here.
 */
memberName
    : IDENTIFIER
    ;
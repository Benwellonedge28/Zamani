/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/effects/effects.g4
 *
 * Status:
 *     Canonical modular production grammar for effect syntax.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Language/runtime baseline:
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
 * This file owns the SOURCE SYNTAX of Zamani computational effects.
 *
 * An effect describes a kind of computational interaction or externally
 * observable computational behavior.
 *
 * Examples:
 *
 *     effect IO;
 *     effect Storage;
 *     effect Network;
 *     effect quantum::Measurement;
 *
 *     effect Storage {
 *         fn read(key: Key) -> Value;
 *         fn write(key: Key, value: Value) -> Unit;
 *     }
 *
 *     fn read(key: Key) -> Value
 *         with effects { IO, Storage }
 *     {
 *         ...
 *     }
 *
 * Effects are intentionally OPEN-WORLD.
 *
 * The grammar MUST NOT contain a closed enumeration of:
 *
 *     IO
 *     Network
 *     GPU
 *     CPU
 *     FPGA
 *     QPU
 *     Quantum
 *     QEC
 *     ZQN
 *     Storage
 *     etc.
 *
 * Such names are source-level identifiers and are interpreted by semantic
 * analysis and registered language/domain facilities.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - effect declarations;
 *   - effect operation declarations;
 *   - effect declaration signatures;
 *   - effect references;
 *   - effect reference lists;
 *   - effect sets;
 *   - effect clauses;
 *   - effect invocation syntax;
 *   - effect handler syntax;
 *   - effect handler arms;
 *   - effect handler patterns;
 *   - resumption syntax;
 *   - effect-abort syntax;
 *   - effect-level generic syntax;
 *   - syntactic effect composition.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - lexical token definitions;
 *   - identifiers;
 *   - qualified-name syntax;
 *   - types;
 *   - expressions;
 *   - capability declarations;
 *   - capability discovery;
 *   - resource declarations;
 *   - resource allocation;
 *   - resource limits;
 *   - hardware discovery;
 *   - backend selection;
 *   - placement;
 *   - topology;
 *   - routing;
 *   - scheduling;
 *   - optimization;
 *   - calibration;
 *   - quantum gates;
 *   - QubitId;
 *   - PhysicalQubitId;
 *   - quantum::ir;
 *   - QEC;
 *   - ZQN;
 *   - resilience;
 *   - runtime dispatch;
 *   - effect implementation.
 *
 * ============================================================================
 * ARCHITECTURAL PRINCIPLE
 * ============================================================================
 *
 *     SOURCE
 *        |
 *        v
 *     lexer
 *        |
 *        v
 *     effect parser
 *        |
 *        v
 *     frontend AST
 *        |
 *        +--> name resolution
 *        +--> type analysis
 *        +--> effect analysis
 *        +--> capability analysis
 *        +--> requirement analysis
 *        +--> resource analysis
 *        |
 *        v
 *     canonical semantic representation
 *        |
 *        +--> classical IR
 *        +--> quantum::ir
 *        +--> HDL/hardware representation
 *        +--> effect metadata
 *        |
 *        v
 *     optimization
 *        |
 *        v
 *     routing / scheduling / resilience / ZQN
 *        |
 *        v
 *     target lowering
 *        |
 *        v
 *     runtime / hardware
 *
 * This grammar MUST NOT reverse this dependency direction.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Effects describe WHAT computation may interact with.
 *
 * They do not describe WHERE or HOW the interaction is realized.
 *
 * Therefore this grammar imposes no language-level limit on:
 *
 *   - number of effects;
 *   - number of operations;
 *   - number of effect references;
 *   - number of handler arms;
 *   - number of parameters;
 *   - number of generic parameters;
 *   - effect-set size;
 *   - handler nesting;
 *   - program size;
 *   - machine size;
 *   - qubit count;
 *   - CPU count;
 *   - GPU count;
 *   - FPGA count;
 *   - node count;
 *   - memory capacity.
 *
 * Practical parser/compiler limits are implementation/resource-policy
 * concerns and MUST NOT become source-language semantics.
 *
 * ============================================================================
 * EFFECT / CAPABILITY / RESOURCE SEPARATION
 * ============================================================================
 *
 * EFFECT:
 *     What computational interaction occurs?
 *
 * CAPABILITY:
 *     What can an execution environment provide?
 *
 * RESOURCE:
 *     What computational resource exists or is requested?
 *
 * REQUIREMENT:
 *     What must be satisfied for a valid realization?
 *
 * CONSTRAINT:
 *     What conditions must a realization obey?
 *
 * PREFERENCE:
 *     Which valid realization is preferred?
 *
 * HINT:
 *     Which implementation direction is suggested without becoming semantic
 *     necessity?
 *
 * Effects MUST NOT silently become hardware-selection syntax.
 *
 * Example:
 *
 *     effect quantum::Measurement;
 *
 * does NOT mean:
 *
 *     use QPU X
 *     use N qubits
 *     use topology Y
 *     use backend Z
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Quantum effects may be represented by ordinary qualified names:
 *
 *     quantum::Measurement
 *     quantum::Reset
 *     quantum::DynamicControl
 *     quantum::Readout
 *
 * This grammar does NOT define quantum semantics.
 *
 * It does not define:
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
 * Quantum lowering remains downstream.
 *
 * The canonical quantum semantic boundary remains:
 *
 *     quantum::ir
 *
 * ============================================================================
 * OPEN-WORLD EXTENSIBILITY
 * ============================================================================
 *
 * Future domains require no modification to this grammar:
 *
 *     effect photonic::interaction;
 *     effect neuromorphic::spike;
 *     effect accelerator::tensor;
 *     effect distributed::consensus;
 *     effect future::computing::operation;
 *
 * Unknown effect identities remain syntactically valid.
 *
 * Semantic analysis may subsequently reject an unknown or unavailable effect.
 *
 * ============================================================================
 * ANTLR COMPOSITION CONTRACT
 * ============================================================================
 *
 * Canonical lexical vocabulary:
 *
 *     grammar/lexer/tokens.g4
 *
 * Therefore:
 *
 *     tokenVocab = ZamaniTokens
 *
 * MUST be used.
 *
 * Shared syntax MUST be imported from canonical modular grammars.
 *
 * This file MUST NOT redefine:
 *
 *     identifier
 *     qualifiedName
 *     attributes
 *     visibility
 *     type expressions
 *     generic parameters
 *     parameter lists
 *     return types
 *     expressions
 *     argument lists
 *     block expressions
 *
 * ============================================================================
 */

parser grammar Effects;

options {
    tokenVocab = ZamaniTokens;
}


/*
 * ============================================================================
 * 1. EFFECT DECLARATIONS
 * ============================================================================
 *
 * Forms:
 *
 *     effect IO;
 *
 *     effect IO<T>;
 *
 *     effect Storage {
 *         fn read(key: Key) -> Value;
 *     }
 *
 *     effect Read(Key) -> Value;
 *
 * The declaration introduces source-level effect identity only.
 */

effectDeclaration
    : attributes?
      visibility?
      K_EFFECT
      qualifiedName
      genericParameters?
      effectDeclarationSignature?
      effectBody?
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 2. EFFECT DECLARATION SIGNATURE
 * ============================================================================
 *
 * A compact signature form.
 *
 *     effect Read(Key);
 *     effect Read(Key) -> Value;
 */

effectDeclarationSignature
    : LPAREN parameterList? RPAREN
      returnType?
    ;


/*
 * ============================================================================
 * 3. EFFECT BODY
 * ============================================================================
 *
 * Zero or more operations are permitted.
 *
 * No fixed operation count is encoded.
 */

effectBody
    : LBRACE
      effectOperation*
      RBRACE
    ;


/*
 * ============================================================================
 * 4. EFFECT OPERATIONS
 * ============================================================================
 *
 * Example:
 *
 *     effect Storage {
 *         fn read(key: Key) -> Value;
 *         fn write(key: Key, value: Value) -> Unit;
 *     }
 *
 * Operations are declarations, not implementations.
 */

effectOperation
    : attributes?
      visibility?
      K_ASYNC?
      K_FN
      identifier
      genericParameters?
      LPAREN parameterList? RPAREN
      returnType?
      whereClause?
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 5. REUSABLE EFFECT OPERATION SIGNATURE
 * ============================================================================
 */

effectOperationSignature
    : K_ASYNC?
      K_FN
      identifier
      genericParameters?
      LPAREN parameterList? RPAREN
      returnType?
      whereClause?
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 6. EFFECT REFERENCES
 * ============================================================================
 *
 * Examples:
 *
 *     IO
 *     Storage
 *     storage::Read
 *     quantum::Measurement
 *     future::domain::Effect
 */

effectReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * 7. EFFECT REFERENCE LIST
 * ============================================================================
 *
 * The repetition is intentionally unbounded by language semantics.
 */

effectReferenceList
    : effectReference
      (COMMA effectReference)*
      COMMA?
    ;


/*
 * ============================================================================
 * 8. EFFECT SET
 * ============================================================================
 *
 * Example:
 *
 *     { IO, Storage, Network }
 *
 * The parser preserves the syntactic collection.
 *
 * Semantic normalization belongs downstream.
 */

effectSet
    : LBRACE
      effectReferenceList?
      RBRACE
    ;


/*
 * ============================================================================
 * 9. EFFECT CLAUSE
 * ============================================================================
 *
 * Example:
 *
 *     fn read() -> Value
 *         with effects { IO, Storage }
 *
 * `with effects` is source-level effect metadata.
 *
 * It does not allocate resources and does not select a target.
 */

effectClause
    : K_WITH
      K_EFFECTS
      effectSet
    ;


/*
 * ============================================================================
 * 10. EFFECT REQUIREMENT
 * ============================================================================
 *
 * Named composition form for aggregate grammars.
 *
 * This remains an effect-level requirement, not a resource requirement.
 */

effectRequirement
    : K_WITH
      K_EFFECTS
      effectSet
    ;


/*
 * ============================================================================
 * 11. EFFECT INVOCATION
 * ============================================================================
 *
 * Example:
 *
 *     perform Storage::read(key);
 *     perform quantum::Measurement(q);
 *
 * The expression grammar owns the expression after `perform`.
 *
 * This prevents effects from introducing a second expression language.
 */

performExpression
    : K_PERFORM
      expression
    ;


performStatement
    : performExpression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 12. EFFECT HANDLING
 * ============================================================================
 *
 * Example:
 *
 *     handle computation {
 *         case Storage::read(key) => resume(value);
 *     }
 *
 * The semantic/runtime interpretation is downstream.
 */

handleStatement
    : K_HANDLE
      expression
      effectHandlerBody
    ;


/*
 * ============================================================================
 * 13. EFFECT HANDLER BODY
 * ============================================================================
 */

effectHandlerBody
    : LBRACE
      effectHandler*
      RBRACE
    ;


/*
 * ============================================================================
 * 14. EFFECT HANDLER ARM
 * ============================================================================
 *
 * Example:
 *
 *     case Storage::read(key) => resume(value);
 *
 * A handler arm contains either a block or expression.
 */

effectHandler
    : K_CASE
      effectHandlerPattern
      FAT_ARROW
      (
          blockExpression
        | expression
      )
      COMMA?
    ;


/*
 * ============================================================================
 * 15. EFFECT HANDLER PATTERN
 * ============================================================================
 *
 * Examples:
 *
 *     Read
 *     Read(key)
 *     Storage::Read(key)
 *     quantum::Measurement(q)
 *
 * Operation identity remains open-world.
 */

effectHandlerPattern
    : qualifiedName
      (
          LPAREN argumentList? RPAREN
      )?
    ;


/*
 * ============================================================================
 * 16. HANDLER CLAUSE
 * ============================================================================
 *
 * Named separately for integration with statement/declaration grammars.
 */

handlerClause
    : K_HANDLE
      expression
      effectHandlerBody
    ;


/*
 * ============================================================================
 * 17. RESUMPTION
 * ============================================================================
 *
 * Forms:
 *
 *     resume;
 *     resume(value);
 *     resume value;
 *
 * The semantic analysis layer determines whether resumption is legal.
 *
 * The parser does not assume a particular runtime continuation model.
 */

resumeExpression
    : K_RESUME
      (
          LPAREN argumentList? RPAREN
        | expression
      )?
    ;


resumeStatement
    : resumeExpression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 18. ABORT
 * ============================================================================
 *
 * `abort` terminates the current effectful computation at the language
 * semantic level.
 *
 * It does not prescribe a machine-specific implementation.
 */

abortExpression
    : K_ABORT
      expression?
    ;


abortStatement
    : abortExpression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 19. EFFECT COMPOSITION
 * ============================================================================
 *
 * Effect composition is represented syntactically by effect sets.
 *
 * This rule exists as an integration point for semantic tooling and aggregate
 * grammar consumers.
 */

effectComposition
    : effectSet
    ;


/*
 * ============================================================================
 * 20. EFFECT REFERENCE EXPRESSION
 * ============================================================================
 *
 * Explicitly names an effect as a semantic reference without performing it.
 *
 * Example:
 *
 *     IO
 *     quantum::Measurement
 */

effectReferenceExpression
    : effectReference
    ;


/*
 * ============================================================================
 * 21. EFFECT MEMBER REFERENCE
 * ============================================================================
 *
 * An effect operation can be referenced through a qualified effect name.
 *
 * Example:
 *
 *     Storage::read
 *     quantum::Measurement
 *
 * No operation catalogue is embedded in the grammar.
 */

effectOperationReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * 22. EFFECT DECLARATION MEMBER
 * ============================================================================
 *
 * Kept as a named integration rule so future aggregate grammars can consume
 * effect members without duplicating effect-operation syntax.
 */

effectMember
    : effectOperation
    ;


/*
 * ============================================================================
 * 23. EFFECT ITEM
 * ============================================================================
 *
 * Generic integration point for declaration-oriented aggregate grammars.
 */

effectItem
    : effectDeclaration
    | effectOperation
    ;


/*
 * ============================================================================
 * 24. EFFECT STATEMENT
 * ============================================================================
 *
 * This rule provides one canonical entry point for statement-oriented
 * integration.
 */

effectStatement
    : performStatement
    | handleStatement
    | resumeStatement
    | abortStatement
    ;


/*
 * ============================================================================
 * 25. EFFECT-SPECIFIC TYPE SIGNATURE
 * ============================================================================
 *
 * Effect signatures intentionally reuse the canonical type system.
 *
 * No local type grammar is introduced.
 */

effectSignature
    : LPAREN parameterList? RPAREN
      returnType?
    ;


/*
 * ============================================================================
 * 26. EFFECT DECLARATION ITEM
 * ============================================================================
 *
 * This is intentionally declaration-oriented and contains no implementation
 * semantics.
 */

effectDeclarationItem
    : effectDeclaration
    ;


/*
 * ============================================================================
 * 27. EFFECT OPERATION ITEM
 * ============================================================================
 */

effectOperationItem
    : effectOperation
    ;


/*
 * ============================================================================
 * 28. EFFECT HANDLER ITEM
 * ============================================================================
 */

effectHandlerItem
    : effectHandler
    ;


/*
 * ============================================================================
 * 29. EFFECT REFERENCE ITEM
 * ============================================================================
 */

effectReferenceItem
    : effectReference
    ;


/*
 * ============================================================================
 * 30. EFFECT SET ITEM
 * ============================================================================
 */

effectSetItem
    : effectSet
    ;


/*
 * ============================================================================
 * 31. EFFECT INTEGRATION ROOT
 * ============================================================================
 *
 * This is the canonical modular entry point for tooling that wants to parse
 * an individual effect construct.
 *
 * The complete program parser remains responsible for determining where such
 * constructs may occur in a complete Zamani compilation unit.
 */

effectConstruct
    : effectDeclaration
    | effectOperation
    | effectClause
    | performStatement
    | handleStatement
    | resumeStatement
    | abortStatement
    | effectReference
    | effectSet
    ;


/*
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * The parser produces syntax.
 *
 * Downstream semantic analysis determines:
 *
 *   - whether an effect exists;
 *   - whether an effect reference resolves;
 *   - whether an effect operation exists;
 *   - whether an effect is allowed in its context;
 *   - whether effect sets compose legally;
 *   - whether an effect is pure/impure;
 *   - whether an effect conflicts with another effect;
 *   - whether capabilities satisfy the effect;
 *   - whether resources satisfy the effect;
 *   - whether constraints are satisfied;
 *   - whether the effect can be lowered to a target;
 *   - whether quantum effects require quantum::ir;
 *   - whether hardware realization is possible.
 *
 * None of those decisions belong here.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend AST should preserve:
 *
 *   - effect identity;
 *   - qualified-name structure;
 *   - declaration modifiers;
 *   - generic parameters;
 *   - operation signatures;
 *   - effect-set membership;
 *   - source ordering;
 *   - handler-arm ordering;
 *   - handler patterns;
 *   - resume/abort syntax;
 *   - source spans;
 *   - diagnostics-relevant source spelling.
 *
 * This grammar does NOT define Rust AST structures.
 *
 * ============================================================================
 * CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Capability syntax remains owned by:
 *
 *     grammar/core/capabilities.g4
 *     grammar/effects/capabilities.g4
 *
 * This file does not duplicate capability identifiers or capability versions.
 *
 * A semantic effect may subsequently be associated with capabilities such as:
 *
 *     quantum::measurement
 *     quantum::readout
 *     accelerator::tensor
 *     distributed::consensus
 *     hardware::clocked_logic
 *
 * without modifying this grammar.
 *
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * Effect syntax does not encode:
 *
 *     device 0
 *     64 qubits
 *     32 cores
 *     8 GPUs
 *     ring topology
 *     fixed memory
 *
 * Resource requirements belong to the resource grammar and semantic model.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * An effect such as:
 *
 *     quantum::Measurement
 *
 * may eventually contribute semantic metadata to quantum compilation.
 *
 * The path is:
 *
 *     effects.g4
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic effect model
 *          |
 *          v
 *     quantum semantic analysis
 *          |
 *          v
 *     quantum::ir
 *
 * This grammar MUST NOT construct quantum::ir directly.
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Classical effects are represented identically to all other effect domains.
 *
 * Examples:
 *
 *     IO
 *     Storage
 *     Memory
 *     Process
 *     FileSystem
 *
 * The grammar does not assume a particular operating system or CPU.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Hardware-related effects may be named:
 *
 *     hardware::clock
 *     hardware::io
 *     hardware::reconfiguration
 *     accelerator::dispatch
 *
 * They remain semantic names.
 *
 * Hardware topology, placement, timing, routing, scheduling, and physical
 * resources remain owned by downstream hardware/compiler subsystems.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Distributed effects may be represented by:
 *
 *     distributed::communication
 *     distributed::consensus
 *     distributed::replication
 *     distributed::remote_execution
 *
 * The grammar does not encode node counts, addresses, regions, or topology.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime dispatch is downstream.
 *
 * Runtime failures MUST NOT be represented as parser failures.
 *
 * An unavailable effect implementation is a semantic/runtime concern.
 *
 * ============================================================================
 * RESILIENCE CONTRACT
 * ============================================================================
 *
 * Resilience may consume effect metadata when deciding whether a recovery
 * strategy preserves program semantics.
 *
 * This file does not implement:
 *
 *     retry
 *     restart
 *     rollback
 *     reroute
 *     remap
 *     reschedule
 *     recompile
 *     backend switching
 *     quarantine
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no embedded actions;
 *     - no semantic predicates;
 *     - no I/O;
 *     - no network operations;
 *     - no hardware discovery;
 *     - no runtime calls;
 *     - no random operations;
 *     - no machine-specific state.
 *
 * Parsing therefore depends only on the supplied token stream and grammar.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * No production rule in this file encodes:
 *
 *     MAX_EFFECTS
 *     MAX_OPERATIONS
 *     MAX_HANDLERS
 *     MAX_HANDLER_DEPTH
 *     MAX_EFFECT_SET_SIZE
 *     MAX_PARAMETERS
 *     MAX_GENERIC_ARITY
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *
 * Repetition and recursive composition remain open-ended.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete only when:
 *
 * [ ] It uses the canonical ZamaniTokens vocabulary.
 * [ ] It contains no embedded Rust actions.
 * [ ] It contains no unsafe code.
 * [ ] It contains no semantic predicates.
 * [ ] It does not duplicate lexical rules.
 * [ ] It does not duplicate type rules.
 * [ ] It does not duplicate expression rules.
 * [ ] It does not define capabilities.
 * [ ] It does not define resources.
 * [ ] It does not define hardware.
 * [ ] It does not define quantum IR.
 * [ ] It does not define QEC.
 * [ ] It does not define ZQN.
 * [ ] It does not define routing.
 * [ ] It does not define scheduling.
 * [ ] It does not define runtime dispatch.
 * [ ] It remains open-world.
 * [ ] It contains no machine-size limit.
 * [ ] Positive effect tests pass.
 * [ ] Negative effect tests pass.
 * [ ] Cross-domain effect tests pass.
 * [ ] Quantum-effect syntax tests pass.
 * [ ] Hardware-effect syntax tests pass.
 * [ ] Distributed-effect syntax tests pass.
 * [ ] Handler/resume/abort tests pass.
 * [ ] Parser determinism tests pass.
 * [ ] AST source-span tests pass.
 * [ ] Round-trip tests pass where the frontend printer supports them.
 * [ ] Legacy duplicate effect grammar is removed from the authoritative build.
 *
 * ============================================================================
 */
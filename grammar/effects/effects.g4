/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/effects/effects.g4
 *
 * Status:
 *     Canonical modular production grammar for Zamani effects.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Runtime/compiler baseline:
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
 * This file is the authoritative SYNTAX owner for the Zamani effect system.
 *
 * Effects describe computational effects and source-level effect intent.
 *
 * Examples include:
 *
 *     effect IO;
 *     effect Storage;
 *     effect Network;
 *     effect QuantumMeasurement;
 *     effect UserDefined<T>;
 *
 *     fn read() -> Value
 *         with effects { IO, Storage }
 *
 *     perform Storage::read(key);
 *
 *     handle computation {
 *         case Storage::read(key) => resume(value);
 *     }
 *
 * The grammar deliberately uses an OPEN-WORLD model.
 *
 * There is no finite built-in effect catalogue.
 *
 * Therefore this grammar does NOT define:
 *
 *     IO
 *     Network
 *     Quantum
 *     GPU
 *     CPU
 *     FPGA
 *     ZQN
 *     QEC
 *     Storage
 *
 * as closed grammar enumerations.
 *
 * They are ordinary source-level names.
 *
 * ============================================================================
 * ARCHITECTURAL BOUNDARY
 * ============================================================================
 *
 * Source
 *   |
 *   v
 * Zamani lexer
 *   |
 *   v
 * effect grammar
 *   |
 *   v
 * frontend AST
 *   |
 *   +--> name resolution
 *   +--> type analysis
 *   +--> effect analysis
 *   +--> capability analysis
 *   +--> resource analysis
 *   |
 *   v
 * canonical semantic representation
 *   |
 *   +--> classical IR
 *   +--> quantum::ir
 *   +--> HDL/hardware representation
 *   +--> resource/effect metadata
 *   |
 *   v
 * optimization
 *   |
 *   v
 * routing / scheduling / resilience / ZQN
 *   |
 *   v
 * target lowering
 *   |
 *   v
 * runtime / hardware
 *
 * THIS FILE OWNS:
 *
 *   - effect declarations;
 *   - effect operation declarations;
 *   - effect references;
 *   - effect reference lists;
 *   - effect clauses;
 *   - effect sets;
 *   - effect invocation syntax;
 *   - effect handler syntax;
 *   - handler patterns;
 *   - continuation/resumption syntax;
 *   - effect-specific abort syntax;
 *   - effect signature syntax;
 *   - effect generic syntax at the effect boundary;
 *   - syntactic effect composition.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - lexical token definitions;
 *   - identifier spelling;
 *   - qualified-name spelling;
 *   - type semantics;
 *   - type checking;
 *   - capability discovery;
 *   - capability authorization;
 *   - resource allocation;
 *   - resource limits;
 *   - hardware discovery;
 *   - backend selection;
 *   - CPU/GPU/QPU selection;
 *   - QPU topology;
 *   - physical qubits;
 *   - quantum gate inventories;
 *   - quantum::ir;
 *   - QEC;
 *   - ZQN;
 *   - resilience;
 *   - routing;
 *   - scheduling;
 *   - calibration;
 *   - optimization;
 *   - execution;
 *   - runtime effect dispatch;
 *   - effect implementation.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Effect syntax describes WHAT computational behavior a program may perform.
 *
 * It must not encode WHERE or HOW that behavior is implemented.
 *
 * The same source effect declaration must remain valid when the implementation
 * executes on:
 *
 *   - an embedded processor;
 *   - a CPU;
 *   - a multicore system;
 *   - a GPU;
 *   - an FPGA;
 *   - an ASIC;
 *   - a quantum processor;
 *   - a simulator;
 *   - a distributed system;
 *   - a cloud system;
 *   - a heterogeneous system;
 *   - a future computational substrate.
 *
 * No grammar-level limits are imposed on:
 *
 *   - number of effects;
 *   - number of operations;
 *   - number of parameters;
 *   - number of handlers;
 *   - nesting depth;
 *   - effect-set size;
 *   - generic arity;
 *   - handler-arm count.
 *
 * Practical implementation limits belong to explicit compiler/parser/runtime
 * resource policies and MUST NOT become language semantics.
 *
 * ============================================================================
 * EFFECT VS CAPABILITY VS RESOURCE
 * ============================================================================
 *
 * Effect:
 *
 *     What kind of computational interaction can occur?
 *
 * Capability:
 *
 *     What can an execution environment provide?
 *
 * Resource:
 *
 *     What computational resource is available/requested?
 *
 * Constraint:
 *
 *     What conditions must a realization satisfy?
 *
 * Preference:
 *
 *     Which valid realization is preferred?
 *
 * Effects MUST NOT be used as an implicit hardware-selection mechanism.
 *
 * For example:
 *
 *     effect QuantumMeasurement;
 *
 * does NOT mean:
 *
 *     use QPU X
 *     use N qubits
 *     use topology Y
 *     use backend Z
 *
 * Those decisions belong downstream.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Quantum effects may be named by ordinary effect references:
 *
 *     quantum::Measurement
 *     quantum::Reset
 *     quantum::DynamicControl
 *     quantum::Readout
 *
 * This grammar does NOT define quantum semantics.
 *
 * In particular it does not define:
 *
 *     QubitId
 *     PhysicalQubitId
 *     GateKind
 *     topology
 *     calibration
 *     noise model
 *     QEC code
 *     ZQN fault
 *
 * Quantum semantic lowering remains downstream and the canonical quantum
 * semantic boundary remains:
 *
 *     quantum::ir
 *
 * ============================================================================
 * OPEN WORLD
 * ============================================================================
 *
 * Effect identities are names.
 *
 * New domains therefore require no modification to this grammar:
 *
 *     effect future::photonic::measurement;
 *     effect neuromorphic::spike;
 *     effect distributed::consensus;
 *     effect accelerator::tensor;
 *     effect custom::domain::operation;
 *
 * The grammar does not need to know what these names mean.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar:
 *
 *   - has no semantic predicates;
 *   - has no embedded actions;
 *   - performs no I/O;
 *   - performs no network access;
 *   - performs no hardware discovery;
 *   - performs no runtime dispatch;
 *   - performs no random operations.
 *
 * Given a deterministic token stream, parsing is deterministic.
 *
 * ============================================================================
 * IMPORT CONTRACT
 * ============================================================================
 *
 * The modular effects grammar consumes canonical shared rules supplied by the
 * rest of the grammar architecture:
 *
 *     identifier
 *     qualifiedName
 *     attributes
 *     attribute
 *     typeExpression
 *     genericParameters
 *     parameterList
 *     returnType
 *     whereClause
 *     expression
 *     argumentList
 *     blockExpression
 *     visibility
 *
 * These rules MUST NOT be duplicated here.
 *
 * The canonical aggregate parser must import this grammar and remove any
 * duplicate effect ownership from legacy parser grammars.
 *
 * In particular, the old effect declaration rules in the monolithic
 * grammar/antlr/Core.g4 and the legacy grammar/antlr/Effects.g4 must not
 * remain competing authoritative definitions.
 *
 * ============================================================================
 */

parser grammar Effects;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * ============================================================================
 * IMPORTS
 * ============================================================================
 *
 * These names are intentionally architectural dependencies rather than
 * reimplementations.
 *
 * The aggregate grammar build must place these parser grammars on the ANTLR
 * grammar source path.
 *
 * Core supplies:
 *     identifiers
 *     qualified names
 *     visibility
 *     attributes
 *     shared declaration infrastructure
 *
 * Types supplies:
 *     type expressions
 *
 * Expressions supplies:
 *     canonical expressions
 *     argument lists
 *     block expressions
 */
import Core, Types, Expressions;


/*
 * ============================================================================
 * 1. EFFECT DECLARATIONS
 * ============================================================================
 *
 * An effect declaration introduces an open-world effect identity and may
 * optionally declare its operations.
 *
 * Supported forms:
 *
 *     effect IO;
 *
 *     effect IO<T>;
 *
 *     effect Read<T>(key: T) -> Value;
 *
 *     effect Storage {
 *         fn read(key: Key) -> Value;
 *         fn write(key: Key, value: Value) -> Unit;
 *     }
 *
 * The declaration itself has no runtime implementation.
 */

effectDeclaration
    : attributes?
      visibility?
      EFFECT
      identifier
      genericParameters?
      effectDeclarationSignature?
      effectBody?
      SEMI?
    ;


/*
 * ============================================================================
 * 2. EFFECT DECLARATION SIGNATURE
 * ============================================================================
 *
 * A compact declaration-level signature is useful for effects whose operation
 * surface is represented as one primary effect signature.
 *
 * Examples:
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
 * The body contains zero or more effect operation declarations.
 *
 * No fixed operation count is imposed.
 */

effectBody
    : LBRACE
      effectOperation*
      RBRACE
    ;


/*
 * ============================================================================
 * 4. EFFECT OPERATION
 * ============================================================================
 *
 * An operation is a named member of an effect declaration.
 *
 * Example:
 *
 *     effect Storage {
 *         fn read(key: Key) -> Value;
 *         fn write(key: Key, value: Value) -> Unit;
 *     }
 *
 * The operation is a declaration, not an implementation.
 */

effectOperation
    : attributes?
      visibility?
      ASYNC?
      FN
      identifier
      genericParameters?
      LPAREN parameterList? RPAREN
      returnType?
      whereClause?
      SEMI?
    ;


/*
 * ============================================================================
 * 5. EFFECT OPERATION SIGNATURE
 * ============================================================================
 *
 * A reusable signature form is provided for semantic tooling and aggregate
 * grammars that need to inspect an effect operation without a body.
 */

effectOperationSignature
    : ASYNC?
      FN
      identifier
      genericParameters?
      LPAREN parameterList? RPAREN
      returnType?
      whereClause?
      SEMI?
    ;


/*
 * ============================================================================
 * 6. EFFECT REFERENCES
 * ============================================================================
 *
 * An effect reference identifies an effect without declaring it.
 *
 * Examples:
 *
 *     IO
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
 * No finite effect-set size is encoded.
 */

effectReferenceList
    : effectReference
      (COMMA effectReference)*
      COMMA?
    ;


/*
 * ============================================================================
 * 8. EFFECT CLAUSE
 * ============================================================================
 *
 * Function/declaration-level effect requirement:
 *
 *     fn read() -> Value
 *         with effects { IO, Storage }
 *
 * Effect clauses express source-level effect requirements.
 *
 * They do not allocate resources and do not select a backend.
 */

effectClause
    : WITH
      EFFECTS
      LBRACE
      effectReferenceList?
      RBRACE
    ;


/*
 * ============================================================================
 * 9. EFFECT REQUIREMENT
 * ============================================================================
 *
 * Alias-level composition rule for grammar consumers that need an explicitly
 * named requirement.
 *
 * This is syntax composition only.
 */

effectRequirement
    : WITH
      EFFECTS
      effectSet
    ;


/*
 * ============================================================================
 * 10. EFFECT SET
 * ============================================================================
 *
 * Example:
 *
 *     effects { IO, Network, Storage }
 *
 * Whether the semantic representation is normalized as a set, ordered row,
 * multiset, capability collection, or another structure belongs downstream.
 */

effectSet
    : LBRACE
      effectReferenceList?
      RBRACE
    ;


/*
 * ============================================================================
 * 11. EFFECT INVOCATION
 * ============================================================================
 *
 * Effects are invoked explicitly with `perform`.
 *
 * Examples:
 *
 *     perform Read(key);
 *     perform Storage::read(key);
 *     perform quantum::Measurement(q);
 *
 * The expression after `perform` belongs to the canonical expression grammar.
 *
 * This prevents effects from creating a second expression language.
 */

performExpression
    : PERFORM
      expression
    ;


performStatement
    : performExpression
      SEMI?
    ;


/*
 * ============================================================================
 * 12. EFFECT HANDLING
 * ============================================================================
 *
 * A handler establishes syntax for handling an effectful computation.
 *
 * Example:
 *
 *     handle computation {
 *         case Storage::read(key) => resume(value);
 *     }
 *
 * Handler semantics are resolved later.
 */

handleStatement
    : HANDLE
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
 *     case Read(key) => resume(value)
 *
 * A handler arm may contain an expression or block.
 *
 * Handler behavior is semantic/runtime territory and is deliberately absent
 * from this grammar.
 */

effectHandler
    : CASE
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
 * Supported forms:
 *
 *     Read
 *     Read(key)
 *     Storage::Read(key)
 *     quantum::Measurement(q)
 *
 * The operation name remains open-ended.
 */

effectHandlerPattern
    : qualifiedName
      (
          LPAREN argumentList? RPAREN
      )?
    ;


/*
 * ============================================================================
 * 16. EFFECT HANDLER CLAUSE
 * ============================================================================
 *
 * Named separately for grammar composition.
 */

handlerClause
    : HANDLE
      expression
      effectHandlerBody
    ;


/*
 * ============================================================================
 * 17. RESUMPTION
 * ============================================================================
 *
 * `resume` represents a source-level continuation operation.
 *
 * Examples:
 *
 *     resume;
 *     resume(value);
 *     resume value;
 *
 * The grammar does NOT guarantee that a particular runtime supports
 * resumable effects.
 *
 * Semantic analysis determines whether the enclosing handler and effect
 * permit resumption.
 */

resumeExpression
    : RESUME
      (
          LPAREN argumentList? RPAREN
        | expression
      )?
    ;


resumeStatement
    : resumeExpression
      SEMI?
    ;


/*
 * ============================================================================
 * 18. ABORT
 * ============================================================================
 *
 * `abort` terminates the current effectful computation at the semantic level.
 *
 * The grammar does not define how a backend implements the termination.
 */

abortExpression
    : ABORT
      expression?
    ;


abortStatement
    : abortExpression
      SEMI?
    ;


/*
 * ============================================================================
 * 19. EFFECT TYPE REFERENCE
 * ============================================================================
 *
 * Effect references may participate in generic semantic structures.
 *
 * Examples:
 *
 *     IO
 *     IO<Value>
 *     quantum::Measurement<Qubit>
 */

effectTypeReference
    : qualifiedName
      genericArguments?
    ;


/*
 * ============================================================================
 * 20. EFFECT GENERIC ARGUMENTS
 * ============================================================================
 *
 * Generic arguments are delegated to the canonical type grammar.
 *
 * This rule does not impose an arity limit.
 */

genericArguments
    : LESS_THAN
      typeExpression
      (COMMA typeExpression)*
      COMMA?
      GREATER_THAN
    ;


/*
 * ============================================================================
 * 21. EFFECT COMPOSITION
 * ============================================================================
 *
 * The grammar records effect references.
 *
 * It does not define algebraic semantics such as:
 *
 *     union
 *     intersection
 *     subtraction
 *     normalization
 *     commutativity
 *     idempotence
 *
 * Those properties belong to semantic effect analysis.
 */

effectComposition
    : effectReferenceList
    ;


/*
 * ============================================================================
 * 22. EFFECT NAME
 * ============================================================================
 *
 * These named rules provide stable parser-tree boundaries for frontend
 * tooling without creating new identifier syntax.
 */

effectName
    : identifier
    ;


effectDeclarationName
    : identifier
    ;


effectOperationName
    : identifier
    ;


qualifiedEffectName
    : qualifiedName
    ;


/*
 * ============================================================================
 * 23. EFFECT COMPUTATION
 * ============================================================================
 *
 * A computation is deliberately delegated to the canonical expression/block
 * grammar.
 *
 * No second computation language is introduced.
 */

effectComputation
    : blockExpression
    | expression
    ;


/*
 * ============================================================================
 * 24. EFFECT ARGUMENT PATTERN
 * ============================================================================
 *
 * Handler argument patterns currently consume canonical expressions.
 *
 * Pattern-specific binding semantics belong to the semantic/frontend layer.
 */

effectArgumentPattern
    : expression
    ;


/*
 * ============================================================================
 * 25. EFFECT BINDING
 * ============================================================================
 *
 * Named binding boundary for effect-aware frontend tooling.
 */

effectBinding
    : identifier
    ;


/*
 * ============================================================================
 * 26. EFFECT ATTRIBUTE
 * ============================================================================
 *
 * Effect attributes use the canonical attribute grammar.
 *
 * This grammar intentionally does not define built-in meanings for:
 *
 *     @pure
 *     @resumable
 *     @async
 *     @quantum
 *     @hardware
 *     @distributed
 *     @security
 *     @resource
 *
 * Their semantics belong to semantic analysis.
 */

effectAttribute
    : attribute
    ;


/*
 * ============================================================================
 * 27. EFFECT ATTRIBUTE LIST
 * ============================================================================
 */

effectAttributeList
    : attribute+
    ;


/*
 * ============================================================================
 * 28. EFFECT REFERENCE WITH ATTRIBUTES
 * ============================================================================
 *
 * This boundary allows future grammar consumers to attach source metadata
 * without changing effect identity syntax.
 */

annotatedEffectReference
    : effectAttributeList?
      effectReference
    ;


/*
 * ============================================================================
 * 29. ANNOTATED EFFECT REFERENCE LIST
 * ============================================================================
 */

annotatedEffectReferenceList
    : annotatedEffectReference
      (COMMA annotatedEffectReference)*
      COMMA?
    ;


/*
 * ============================================================================
 * 30. EFFECT DECLARATION LIST
 * ============================================================================
 *
 * No fixed declaration count is encoded.
 */

effectDeclarationList
    : effectDeclaration+
    ;


/*
 * ============================================================================
 * 31. EFFECT OPERATION LIST
 * ============================================================================
 */

effectOperationList
    : effectOperation+
    ;


/*
 * ============================================================================
 * 32. OPTIONAL EFFECT REFERENCE LIST
 * ============================================================================
 */

optionalEffectReferenceList
    : effectReferenceList?
    ;


/*
 * ============================================================================
 * 33. OPTIONAL EFFECT SET
 * ============================================================================
 */

optionalEffectSet
    : effectSet?
    ;


/*
 * ============================================================================
 * 34. EFFECT SIGNATURE
 * ============================================================================
 *
 * This is a reusable signature without an implementation body.
 */

effectSignature
    : LPAREN parameterList? RPAREN
      returnType?
    ;


/*
 * ============================================================================
 * 35. EFFECT OPERATION GROUP
 * ============================================================================
 *
 * A group is purely syntactic and imposes no runtime grouping semantics.
 */

effectOperationGroup
    : LBRACE
      effectOperation*
      RBRACE
    ;


/*
 * ============================================================================
 * 36. EFFECT HANDLER GROUP
 * ============================================================================
 */

effectHandlerGroup
    : LBRACE
      effectHandler*
      RBRACE
    ;


/*
 * ============================================================================
 * 37. EFFECT REQUIREMENT LIST
 * ============================================================================
 *
 * Explicitly named composition boundary.
 */

effectRequirementList
    : effectRequirementItem
      (COMMA effectRequirementItem)*
      COMMA?
    ;


effectRequirementItem
    : effectReference
    ;


/*
 * ============================================================================
 * 38. EFFECT USE
 * ============================================================================
 *
 * Generic source-level effect use.
 *
 * This is deliberately separate from declaration and invocation.
 */

effectUse
    : effectReference
    ;


/*
 * ============================================================================
 * 39. EFFECT USE LIST
 * ============================================================================
 */

effectUseList
    : effectUse
      (COMMA effectUse)*
      COMMA?
    ;


/*
 * ============================================================================
 * 40. EFFECT HANDLER CASE LIST
 * ============================================================================
 */

effectHandlerCaseList
    : effectHandler+
    ;


/*
 * ============================================================================
 * 41. EFFECT HANDLER PATTERN LIST
 * ============================================================================
 */

effectHandlerPatternList
    : effectHandlerPattern
      (COMMA effectHandlerPattern)*
      COMMA?
    ;


/*
 * ============================================================================
 * 42. EFFECT-RELATED EXPRESSION
 * ============================================================================
 *
 * This rule gives tooling a stable syntax boundary while retaining the
 * canonical expression grammar.
 */

effectRelatedExpression
    : performExpression
    | resumeExpression
    | abortExpression
    | expression
    ;


/*
 * ============================================================================
 * 43. EFFECT-RELATED STATEMENT
 * ============================================================================
 */

effectRelatedStatement
    : performStatement
    | resumeStatement
    | abortStatement
    | handleStatement
    ;


/*
 * ============================================================================
 * 44. EFFECT DECLARATION CONTRACT
 * ============================================================================
 *
 * Frontend AST contract:
 *
 *     EffectDeclaration
 *         name
 *         generic_parameters
 *         signature
 *         operations
 *         attributes
 *         visibility
 *         source_span
 *
 * The AST must NOT contain:
 *
 *     hardware_id
 *     device_id
 *     physical_qubit
 *     topology
 *     calibration
 *     backend
 *     scheduler_state
 *     resource_allocation
 *     runtime_handle
 *
 * Those belong downstream.
 */


/*
 * ============================================================================
 * 45. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for:
 *
 *     - resolving effect names;
 *     - resolving namespaces;
 *     - validating effect declarations;
 *     - validating duplicate operations;
 *     - validating operation signatures;
 *     - checking effect availability;
 *     - checking effect propagation;
 *     - checking handler coverage;
 *     - checking resumability;
 *     - checking effect compatibility;
 *     - checking capability requirements;
 *     - checking resource requirements;
 *     - producing semantic diagnostics;
 *     - lowering effect metadata to the canonical semantic representation.
 *
 * The parser MUST NOT perform any of those operations.
 */


/*
 * ============================================================================
 * 46. HARD-CODING CONTRACT
 * ============================================================================
 *
 * Forbidden in this grammar:
 *
 *     MAX_EFFECTS
 *     MAX_EFFECT_OPERATIONS
 *     MAX_EFFECT_PARAMETERS
 *     MAX_EFFECT_HANDLERS
 *     MAX_EFFECT_DEPTH
 *     MAX_HANDLER_DEPTH
 *     MAX_EFFECT_SET_SIZE
 *     MAX_RESOURCE_COUNT
 *     MAX_QUBITS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_DEVICES
 *     MAX_NODES
 *
 * No physical machine property may appear in this grammar as a language
 * limitation.
 */


/*
 * ============================================================================
 * 47. SECURITY CONTRACT
 * ============================================================================
 *
 * An effect declaration does NOT grant authorization.
 *
 * For example:
 *
 *     effect FileSystem;
 *
 * does not itself grant filesystem access.
 *
 * Authorization belongs to capability/security analysis and runtime policy.
 *
 * Likewise:
 *
 *     effect Network;
 *
 * does not grant network access.
 *
 * This prevents source syntax from becoming an implicit security boundary.
 */


/*
 * ============================================================================
 * 48. RESOURCE CONTRACT
 * ============================================================================
 *
 * Effects may be associated semantically with resource requirements.
 *
 * This grammar does not allocate resources.
 *
 * Examples:
 *
 *     effect quantum::Measurement;
 *     effect distributed::Communication;
 *     effect accelerator::TensorExecution;
 *
 * do not prescribe:
 *
 *     device count;
 *     topology;
 *     memory size;
 *     qubit count;
 *     network width;
 *     accelerator count.
 *
 * Those values are determined by downstream resource/capability analysis.
 */


/*
 * ============================================================================
 * 49. QUANTUM INTEGRATION CONTRACT
 * ============================================================================
 *
 * Quantum effects are represented as open-world qualified names.
 *
 * Example:
 *
 *     with effects {
 *         quantum::Measurement,
 *         quantum::Reset
 *     }
 *
 * The frontend may lower these effect references into semantic metadata
 * associated with a quantum computation.
 *
 * Any actual quantum operation semantics MUST cross the canonical:
 *
 *     quantum::ir
 *
 * boundary.
 *
 * This grammar must never instantiate:
 *
 *     QubitId
 *     PhysicalQubitId
 *     GateKind
 *     topology
 *     calibration
 *
 * or any equivalent machine-specific object.
 */


/*
 * ============================================================================
 * 50. ZQN / QEC / RESILIENCE CONTRACT
 * ============================================================================
 *
 * Effects may describe intent associated with:
 *
 *     noise;
 *     measurement;
 *     recovery;
 *     resilience;
 *     distributed execution;
 *
 * but this grammar does not define their algorithms.
 *
 * ZQN owns fault/noise semantics.
 * QEC owns detection/correction algorithms.
 * Resilience owns adaptation/recovery decisions.
 *
 * The effect grammar only records source syntax.
 */


/*
 * ============================================================================
 * 51. SCHEDULING CONTRACT
 * ============================================================================
 *
 * Effect syntax does not prescribe:
 *
 *     execution order;
 *     duration;
 *     start time;
 *     resource reservation;
 *     placement;
 *     routing;
 *     pulse schedule.
 *
 * If an effect requires timing/resource information, semantic analysis passes
 * the resulting metadata to scheduling/resource subsystems.
 */


/*
 * ============================================================================
 * 52. HARDWARE / HDL CONTRACT
 * ============================================================================
 *
 * Hardware-related effects may be expressed through open-world names:
 *
 *     hardware::clock;
 *     hardware::io;
 *     hardware::dma;
 *     hardware::interrupt;
 *     accelerator::execute;
 *
 * without embedding physical hardware details.
 *
 * HDL semantics remain owned by the HDL grammar/semantic layers.
 */


/*
 * ============================================================================
 * 53. DISTRIBUTED COMPUTING CONTRACT
 * ============================================================================
 *
 * Distributed effects may describe:
 *
 *     communication;
 *     synchronization;
 *     consensus;
 *     replication;
 *     remote execution;
 *
 * but this grammar does not prescribe:
 *
 *     node count;
 *     network topology;
 *     transport;
 *     provider;
 *     deployment.
 */


/*
 * ============================================================================
 * 54. DETERMINISM CONTRACT
 * ============================================================================
 *
 * There are:
 *
 *     no semantic predicates;
 *     no actions;
 *     no dynamic token generation;
 *     no external state;
 *     no I/O;
 *     no randomness.
 *
 * The same source/token stream therefore has deterministic syntactic
 * interpretation.
 */


/*
 * ============================================================================
 * 55. ERROR CONTRACT
 * ============================================================================
 *
 * Malformed effect syntax must remain malformed.
 *
 * Examples that must be rejected:
 *
 *     effect;
 *     effect ();
 *     effect Name(;
 *     effect Name { fn ; }
 *     perform;
 *     handle;
 *     case => value;
 *
 * Error recovery belongs to the parser/runtime integration.
 *
 * This grammar must not silently reinterpret malformed input as another
 * construct.
 */


/*
 * ============================================================================
 * 56. COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Effect names are open-ended.
 *
 * Therefore adding a new effect namespace must not require a grammar change.
 *
 * Existing programs using ordinary effect names remain syntactically valid
 * across future hardware generations and computational domains.
 *
 * Breaking changes to effect syntax require an explicit language-version
 * migration policy outside this file.
 */


/*
 * ============================================================================
 * 57. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *   [ ] It compiles as part of the canonical ANTLR grammar composition.
 *   [ ] It consumes only canonical lexer tokens.
 *   [ ] It does not define lexer rules.
 *   [ ] It does not duplicate identifier syntax.
 *   [ ] It does not duplicate type syntax.
 *   [ ] It does not duplicate expression syntax.
 *   [ ] It does not duplicate capability syntax.
 *   [ ] It does not define runtime behavior.
 *   [ ] It contains no hardware limits.
 *   [ ] It contains no fixed effect catalogue.
 *   [ ] It contains no machine-specific identifiers.
 *   [ ] It contains no unsafe Rust.
 *   [ ] Effect declarations parse.
 *   [ ] Effect operation declarations parse.
 *   [ ] Effect references parse.
 *   [ ] Effect clauses parse.
 *   [ ] Effect invocation parses.
 *   [ ] Effect handlers parse.
 *   [ ] Handler patterns parse.
 *   [ ] Resumption parses.
 *   [ ] Abort syntax parses.
 *   [ ] Generic effects parse.
 *   [ ] Qualified effect names parse at arbitrary depth.
 *   [ ] Arbitrary effect-set sizes parse.
 *   [ ] Arbitrary operation counts parse.
 *   [ ] Cross-domain effect names remain open-ended.
 *   [ ] Negative syntax is rejected deterministically.
 *   [ ] The resulting AST can preserve source spans.
 *   [ ] Semantic analysis can lower effect metadata independently.
 *   [ ] No downstream hardware decision is embedded in syntax.
 *
 * ============================================================================
 */
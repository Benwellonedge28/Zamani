/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/ai/embedded.g4
 *
 * Grammar:
 *     AIEmbedded
 *
 * Status:
 *     Canonical AI / embedded-computation leaf grammar.
 *
 * Language baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust edition 2021
 *
 * Safety:
 *     - Parser grammar only.
 *     - No embedded Rust actions.
 *     - No semantic predicates.
 *     - No unsafe Rust requirement.
 *     - No filesystem access.
 *     - No network access.
 *     - No hardware discovery.
 *     - No runtime execution.
 *     - No vendor-specific implementation.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file defines the source-level syntax boundary for AI computation
 * intended to be suitable for embedded, edge, constrained, heterogeneous,
 * or otherwise resource-aware execution environments.
 *
 * "Embedded" is a COMPUTATION/DEPLOYMENT INTENT.
 *
 * It is NOT a fixed hardware class.
 *
 * The eventual target may be:
 *
 *     - a tiny embedded processor;
 *     - a microcontroller;
 *     - an embedded CPU;
 *     - a DSP;
 *     - an NPU;
 *     - a GPU;
 *     - an FPGA;
 *     - an ASIC;
 *     - a quantum-assisted system;
 *     - a heterogeneous accelerator;
 *     - a distributed edge system;
 *     - a simulator;
 *     - a future computational substrate.
 *
 * The grammar does not decide which one is used.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     canonical parser
 *          |
 *          +--> AIEmbedded
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> type analysis
 *          +--> effect analysis
 *          +--> resource analysis
 *          +--> capability analysis
 *          +--> portability analysis
 *          +--> security analysis
 *          +--> deployment analysis
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> classical semantics
 *          +--> AI semantics
 *          +--> quantum semantics
 *          +--> hardware semantics
 *          +--> distributed semantics
 *          |
 *          v
 *     canonical IR
 *          |
 *          v
 *     optimization / lowering
 *          |
 *          +--> specialization
 *          +--> scheduling
 *          +--> placement
 *          +--> routing
 *          +--> resilience
 *          |
 *          v
 *     HAL / backend
 *          |
 *          v
 *     target realization
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Embedded AI syntax MUST preserve:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * The source program expresses:
 *
 *     WHAT computation is required;
 *     WHAT capabilities are required;
 *     WHAT resources are required;
 *     WHAT constraints apply;
 *     WHAT preferences exist;
 *     WHAT deployment properties matter.
 *
 * It does NOT inherently express:
 *
 *     WHICH CPU;
 *     WHICH MCU;
 *     WHICH GPU;
 *     WHICH NPU;
 *     WHICH FPGA;
 *     WHICH ASIC;
 *     WHICH device;
 *     WHICH core;
 *     WHICH accelerator;
 *     WHICH memory bank;
 *     WHICH physical address;
 *     WHICH vendor;
 *     WHICH topology.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This grammar contains no artificial universal capacity limits.
 *
 * It does NOT define:
 *
 *     MAX_MODELS
 *     MAX_TENSORS
 *     MAX_PARAMETERS
 *     MAX_LAYERS
 *     MAX_AGENTS
 *     MAX_INPUTS
 *     MAX_OUTPUTS
 *     MAX_WORKERS
 *     MAX_DEVICES
 *     MAX_ACCELERATORS
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *
 * Repetition is represented structurally by `*` or `?`.
 *
 * Actual feasibility is determined by:
 *
 *     program semantics;
 *     compiler representation;
 *     declared requirements;
 *     target capabilities;
 *     available resources;
 *     deployment policy;
 *     runtime conditions.
 *
 * ============================================================================
 * HARDWARE INDEPENDENCE
 * ============================================================================
 *
 * The following are deliberately NOT parser-level special cases:
 *
 *     microcontroller
 *     CPU
 *     GPU
 *     NPU
 *     TPU
 *     FPGA
 *     ASIC
 *     QPU
 *     DSP
 *     CUDA
 *     ROCm
 *     vendor-specific accelerators
 *
 * Such names may occur as identifiers or values where permitted by the
 * general language, but this grammar does not reserve them as hardware
 * primitives.
 *
 * ============================================================================
 * AI INTEGRATION
 * ============================================================================
 *
 * Existing AI grammar owns AI concepts such as:
 *
 *     models
 *     tensors
 *     datasets
 *     training
 *     inference
 *     differentiation
 *     pipelines
 *     agents
 *     accelerators
 *
 * This grammar does NOT redefine those concepts.
 *
 * Instead it provides a deployment/execution envelope around them.
 *
 * For example, an embedded region may contain:
 *
 *     @model model;
 *     @inference(input);
 *     @requires(capability("tensor.compute"));
 *
 * The meaning of `model`, `inference`, tensors, and capabilities is owned by
 * their respective semantic/domain systems.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY MODEL
 * ============================================================================
 *
 * Embedded AI commonly requires resource-aware compilation.
 *
 * The grammar therefore permits generic annotation forms such as:
 *
 *     @requires(...)
 *     @capability(...)
 *     @constraint(...)
 *     @preference(...)
 *     @hint(...)
 *
 * Examples:
 *
 *     @requires(memory >= required_memory)
 *     @requires(capability("tensor.compute"))
 *     @requires(capability("inference"))
 *     @requires(latency <= budget)
 *     @preference(capability("accelerator.compute"))
 *
 * These are EXPRESSIONS.
 *
 * The grammar does not decide whether a target satisfies them.
 *
 * ============================================================================
 * MEMORY
 * ============================================================================
 *
 * Embedded AI may use multiple memory classes.
 *
 * This grammar therefore permits memory intent to be expressed through
 * ordinary types, expressions, and annotations.
 *
 * It does NOT hard-code:
 *
 *     RAM capacity;
 *     cache capacity;
 *     register width;
 *     scratchpad size;
 *     SRAM size;
 *     VRAM size;
 *     persistent-storage size.
 *
 * ============================================================================
 * QUANTUM INTEROPERABILITY
 * ============================================================================
 *
 * Embedded AI may coexist with quantum computation.
 *
 * This grammar does not define:
 *
 *     qubits;
 *     quantum gates;
 *     quantum topology;
 *     QEC;
 *     ZQN;
 *     physical qubits.
 *
 * Those remain owned by the quantum domain and ultimately the canonical
 * `quantum::ir` boundary.
 *
 * Embedded AI may reference quantum computation through shared expressions,
 * declarations, annotations, and domain composition.
 *
 * ============================================================================
 * CLASSICAL / HDL / HARDWARE INTEROPERABILITY
 * ============================================================================
 *
 * Embedded AI may coexist with:
 *
 *     classical computation;
 *     HDL;
 *     hardware intent;
 *     accelerators;
 *     distributed execution;
 *     networking;
 *     security;
 *     data processing.
 *
 * This grammar does not duplicate those grammars.
 *
 * ============================================================================
 * SYNTAX MODEL
 * ============================================================================
 *
 * The canonical structural form is:
 *
 *     @embedded name {
 *         ...
 *     }
 *
 * Generic embedded constructs may also use:
 *
 *     @embedded name;
 *
 *     @embedded name = expression;
 *
 *     @embedded(name);
 *
 *     @embedded(name, ...);
 *
 * This keeps the syntax extensible without making every future embedded
 * concept a lexer keyword.
 *
 * ============================================================================
 * ANNOTATION MODEL
 * ============================================================================
 *
 * The canonical lexer emits:
 *
 *     AT
 *
 * followed by the canonical identifier/name token.
 *
 * This grammar therefore consumes:
 *
 *     AT IDENTIFIER
 *
 * rather than:
 *
 *     NANO_ANNOTATION
 *
 * The semantic layer determines whether an annotation name such as:
 *
 *     embedded
 *     model
 *     inference
 *     requires
 *     capability
 *     memory
 *     constraint
 *     preference
 *     hint
 *
 * is valid in its particular context.
 *
 * ============================================================================
 */

parser grammar AIEmbedded;

options {
    tokenVocab = ZamaniLexer;
}

import Types,
       Expressions,
       Statements;


/* ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * `embeddedConstruct` is the public leaf entry point.
 *
 * The AI composition grammar may delegate to this rule.
 *
 * This rule does not consume EOF.
 *
 * The root program grammar owns EOF.
 */
embeddedConstruct
    : embeddedDeclaration
    | embeddedBinding
    | embeddedInvocation
    | embeddedRegion
    | embeddedContract
    ;


/* ============================================================================
 * EMBEDDED DECLARATION
 * ============================================================================
 *
 * Examples:
 *
 *     @embedded Vision {
 *         ...
 *     }
 *
 *     @embedded Vision : EmbeddedModel {
 *         ...
 *     }
 *
 *     @embedded Vision = model;
 *
 * The annotation name remains syntactically extensible.
 */
embeddedDeclaration
    : AT IDENTIFIER
      embeddedDeclarationTail
    ;


/* ============================================================================
 * DECLARATION TAIL
 * ============================================================================
 */
embeddedDeclarationTail
    : embeddedTypeClause?
      embeddedInitializer?
      embeddedBody
    | embeddedTypeClause?
      embeddedInitializer
      SEMICOLON
    | embeddedTypeClause
      SEMICOLON
    ;


/* ============================================================================
 * EMBEDDED BINDING
 * ============================================================================
 *
 * Provides an explicit binding form:
 *
 *     @embedded target = expression;
 *
 * The expression may refer to:
 *
 *     model
 *     pipeline
 *     inference function
 *     tensor
 *     device-independent resource
 *     deployment description
 *     other semantic object
 *
 * Physical realization remains downstream.
 */
embeddedBinding
    : AT IDENTIFIER
      embeddedTypeClause?
      ASSIGN
      expression
      SEMICOLON
    ;


/* ============================================================================
 * EMBEDDED INVOCATION
 * ============================================================================
 *
 * Generic invocation form:
 *
 *     @embedded(model);
 *     @inference(input);
 *     @deploy(model, policy);
 *
 * The grammar does not prescribe a finite operation list.
 */
embeddedInvocation
    : AT IDENTIFIER
      LPAREN
      embeddedArgumentList?
      RPAREN
      SEMICOLON
    ;


/* ============================================================================
 * EMBEDDED REGION
 * ============================================================================
 *
 * A region creates an explicit structural boundary for embedded computation.
 *
 * Example:
 *
 *     @embedded inference {
 *         ...
 *     }
 *
 * Nested declarations and ordinary Zamani statements are allowed.
 *
 * There is no fixed maximum number of members.
 */
embeddedRegion
    : AT IDENTIFIER
      embeddedRegionPrefix?
      embeddedBody
    ;


/* ============================================================================
 * REGION PREFIX
 * ============================================================================
 *
 * Supports:
 *
 *     @embedded inference { ... }
 *     @embedded inference : Model { ... }
 *     @embedded inference = model { ... }
 *
 * Semantic validation determines whether the selected form is meaningful.
 */
embeddedRegionPrefix
    : embeddedTypeClause?
      embeddedInitializer?
    ;


/* ============================================================================
 * EMBEDDED CONTRACT
 * ============================================================================
 *
 * Contract annotations allow resource/capability/constraint intent to remain
 * extensible.
 *
 * Examples:
 *
 *     @requires(capability("tensor.compute"));
 *     @requires(memory >= required_memory);
 *     @capability(capability_expression);
 *     @constraint(latency <= budget);
 *     @preference(accelerator);
 *     @hint(locality);
 *
 * The annotation name is not hard-coded.
 *
 * Semantic analysis owns the contract vocabulary.
 */
embeddedContract
    : AT IDENTIFIER
      LPAREN
      embeddedArgumentList?
      RPAREN
      SEMICOLON
    ;


/* ============================================================================
 * TYPE CLAUSE
 * ============================================================================
 *
 * Uses the canonical type grammar.
 *
 * This file does not redefine:
 *
 *     generic types
 *     tensors
 *     memory types
 *     references
 *     function types
 *     quantum types
 *     hardware types
 *
 * Those remain owned by the shared type system.
 */
embeddedTypeClause
    : COLON typeExpression
    ;


/* ============================================================================
 * INITIALIZER
 * ============================================================================
 */
embeddedInitializer
    : ASSIGN expression
    ;


/* ============================================================================
 * ARGUMENT LIST
 * ============================================================================
 *
 * Uses the canonical expression grammar.
 *
 * No fixed argument count is imposed.
 */
embeddedArgumentList
    : expression
      (COMMA expression)*
    ;


/* ============================================================================
 * BODY
 * ============================================================================
 *
 * Embedded regions can contain:
 *
 *     nested embedded constructs;
 *     AI constructs;
 *     declarations;
 *     expressions through ordinary statements;
 *     resource/capability contracts;
 *     classical computation;
 *     domain statements;
 *     future domain extensions.
 *
 * Actual semantic legality is determined downstream.
 */
embeddedBody
    : LBRACE
      embeddedMember*
      RBRACE
    ;


/* ============================================================================
 * BODY MEMBER
 * ============================================================================
 *
 * `statement` remains the canonical universal statement rule.
 *
 * This file does not create another statement hierarchy.
 */
embeddedMember
    : embeddedConstruct
    | statement
    ;
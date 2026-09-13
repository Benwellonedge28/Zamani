/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/statements/unsafe.g4
 *
 * Status:
 *     Canonical production grammar component for Zamani unsafe regions.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Rust integration baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Rust safety:
 *     The Zamani compiler/parser implementation MUST use safe Rust only.
 *
 *     This grammar contains:
 *         - no embedded Rust actions;
 *         - no semantic predicates;
 *         - no Rust code;
 *         - no unsafe Rust;
 *         - no I/O;
 *         - no filesystem access;
 *         - no networking;
 *         - no hardware discovery;
 *         - no runtime execution;
 *         - no mutable global state.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the syntax of a Zamani unsafe region:
 *
 *     unsafe {
 *         ...
 *     }
 *
 * An unsafe region is an explicit source-level semantic boundary indicating
 * that the enclosed operations may require capabilities, effects, permissions,
 * proof obligations, or safety checks that are not available in ordinary
 * safe source context.
 *
 * IMPORTANT:
 *
 *     `unsafe` in Zamani source code is NOT Rust `unsafe`.
 *
 * This grammar does not permit or generate unsafe Rust code.
 *
 * The Rust implementation of Zamani remains safe Rust.
 *
 * ============================================================================
 * CANONICAL SYNTAX
 * ============================================================================
 *
 * The ONLY canonical unsafe statement syntax is:
 *
 *     unsafe blockExpression
 *
 * Examples:
 *
 *     unsafe {
 *         operation();
 *     }
 *
 *     unsafe {
 *         let value = operation();
 *         consume(value);
 *     }
 *
 *     if condition {
 *         unsafe {
 *             privileged_operation();
 *         }
 *     }
 *
 * Nested unsafe regions are syntactically possible:
 *
 *     unsafe {
 *         operation_a();
 *
 *         unsafe {
 *             operation_b();
 *         }
 *     }
 *
 * The semantic analyzer determines whether nested unsafe regions are useful,
 * redundant, prohibited, or otherwise constrained.
 *
 * ============================================================================
 * REJECTED LEGACY FORMS
 * ============================================================================
 *
 * The following historical forms MUST NOT be part of the canonical grammar:
 *
 *     unsafe!
 *     unsafe(...)
 *     unsafe(evas: ...)
 *
 * They are removed from the production syntax because they introduce multiple
 * meanings for the same safety boundary and do not correspond to a stable
 * language-level ownership model.
 *
 * Existing source using those forms must be migrated explicitly by the
 * compatibility/migration subsystem rather than silently accepted by this
 * grammar.
 *
 * ============================================================================
 * ARCHITECTURAL PRINCIPLE
 * ============================================================================
 *
 * Source
 *   |
 *   v
 * canonical lexer
 *   |
 *   v
 * canonical parser
 *   |
 *   v
 * unsafeStatement             <-- THIS FILE
 *   |
 *   v
 * frontend AST
 *   |
 *   v
 * semantic analysis
 *   |
 *   +--> safety analysis
 *   +--> effect analysis
 *   +--> capability analysis
 *   +--> ownership analysis
 *   +--> resource analysis
 *   +--> domain validation
 *   |
 *   v
 * canonical semantic representation
 *   |
 *   +--> classical representation
 *   +--> quantum::ir
 *   +--> HDL/hardware representation
 *   +--> distributed representation
 *   +--> accelerator representation
 *   |
 *   v
 * optimization
 *   |
 *   v
 * routing / scheduling / lowering
 *   |
 *   v
 * target realization
 *   |
 *   v
 * runtime / hardware
 *
 * This file owns syntax only.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - unsafeStatement
 *     - the source-level `unsafe` keyword followed by a canonical block
 *     - the syntactic boundary of an unsafe region
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - the UNSAFE lexer token
 *     - braces
 *     - identifiers
 *     - expressions
 *     - statements generally
 *     - blockExpression
 *     - effect declarations
 *     - capabilities
 *     - permissions
 *     - resource requirements
 *     - ownership
 *     - borrowing
 *     - lifetimes
 *     - quantum semantics
 *     - quantum::ir
 *     - QEC
 *     - ZQN
 *     - hardware topology
 *     - hardware discovery
 *     - calibration
 *     - routing
 *     - scheduling
 *     - optimization
 *     - runtime execution
 *     - backend selection
 *     - target selection
 *     - Rust unsafe code
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The canonical lexer owns:
 *
 *     UNSAFE
 *
 * The canonical punctuation lexer owns:
 *
 *     LBRACE
 *     RBRACE
 *
 * This file MUST NOT define:
 *
 *     UNSAFE
 *     'unsafe'
 *     LBRACE
 *     RBRACE
 *
 * or any other lexer rule.
 *
 * Canonical lexical ownership currently exists in:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * and the repository's modular lexer definitions.
 *
 * ============================================================================
 * BLOCK CONTRACT
 * ============================================================================
 *
 * `blockExpression` is owned by:
 *
 *     grammar/statements/blocks.g4
 *
 * This file MUST NOT redefine:
 *
 *     blockExpression
 *     blockElement
 *     statement
 *
 * The unsafe construct therefore composes with the canonical block grammar:
 *
 *     unsafe
 *       |
 *       v
 *     blockExpression
 *       |
 *       +--> canonical statement sequence
 *
 * Consequently every statement that is legal in a normal canonical block can
 * also be syntactically represented inside an unsafe region.
 *
 * Whether a particular statement is permitted there is a semantic question.
 *
 * ============================================================================
 * STATEMENT COMPOSITION CONTRACT
 * ============================================================================
 *
 * The canonical statement dispatcher is responsible for admitting this rule.
 *
 * Conceptually:
 *
 *     statement
 *         |
 *         +--> safetyStatement
 *                 |
 *                 +--> unsafeStatement
 *
 * or, where the repository's composition layer does not use a separate
 * safetyStatement category:
 *
 *     statement
 *         |
 *         +--> unsafeStatement
 *
 * `unsafe.g4` MUST NOT redefine the canonical `statement` rule.
 *
 * `unsafe.g4` MUST NOT redefine `controlFlowStatement`.
 *
 * `unsafe.g4` MUST NOT redefine `blockExpression`.
 *
 * ============================================================================
 * PRIMARY RULE
 * ============================================================================
 */

/**
 * Canonical Zamani unsafe statement.
 *
 * Syntax:
 *
 *     unsafe { ... }
 *
 * The body is always a canonical blockExpression.
 *
 * No machine size, hardware type, resource count, or target property is
 * encoded in this rule.
 */
unsafeStatement
    : UNSAFE blockExpression
    ;


/*
 * ============================================================================
 * UNSAFE REGION SEMANTICS
 * ============================================================================
 *
 * The grammar establishes only the existence and extent of the region.
 *
 * Semantic analysis determines:
 *
 *     - whether the surrounding context permits an unsafe region;
 *     - which safety obligations apply;
 *     - which capabilities are required;
 *     - which effects are required;
 *     - which operations require explicit authorization;
 *     - whether required permissions exist;
 *     - whether the operation is statically provable safe;
 *     - whether an unsafe boundary is redundant;
 *     - whether the unsafe region violates policy;
 *     - whether the operation is target-portable;
 *     - whether a target-specific implementation is permitted;
 *     - whether the resulting program remains semantically valid.
 *
 * The parser MUST NOT perform any of those checks.
 *
 * ============================================================================
 * WHAT `unsafe` DOES NOT MEAN
 * ============================================================================
 *
 * An unsafe region does NOT automatically mean:
 *
 *     - disable the type checker;
 *     - disable borrow checking;
 *     - disable ownership checking;
 *     - disable effect checking;
 *     - disable capability checking;
 *     - bypass security policy;
 *     - bypass resource validation;
 *     - execute arbitrary machine instructions;
 *     - access arbitrary memory;
 *     - access arbitrary hardware;
 *     - select a physical device;
 *     - bypass quantum safety;
 *     - bypass QEC;
 *     - bypass ZQN;
 *     - bypass scheduling;
 *     - bypass routing;
 *     - bypass runtime policy;
 *     - generate unsafe Rust.
 *
 * The semantic subsystem must explicitly define which obligations are relaxed
 * or enabled by the unsafe boundary.
 *
 * ============================================================================
 * SAFETY MODEL
 * ============================================================================
 *
 * A Zamani unsafe region should be treated as an explicit semantic capability
 * boundary.
 *
 * Conceptually:
 *
 *     safe context
 *          |
 *          v
 *     unsafe {
 *         operation
 *     }
 *          |
 *          v
 *     safe context
 *
 * The region does not automatically transfer arbitrary authority to the
 * enclosed operations.
 *
 * The semantic analyzer remains responsible for determining the exact
 * capabilities and permissions required.
 *
 * ============================================================================
 * CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Unsafe operations may require capabilities represented by the language's
 * capability system.
 *
 * Examples conceptually include:
 *
 *     memory.raw_access
 *     hardware.direct_control
 *     device.register_access
 *     foreign.unverified
 *     quantum.backend_specific
 *     timing.precise_control
 *     distributed.failure_override
 *
 * These are examples of semantic categories only.
 *
 * They MUST NOT become hard-coded tokens or mandatory capability names in this
 * grammar.
 *
 * Capability vocabulary belongs to the capability/effect/semantic layers.
 *
 * This preserves extensibility for future computing domains.
 *
 * ============================================================================
 * EFFECT INTEGRATION
 * ============================================================================
 *
 * An unsafe region may contain effectful operations.
 *
 * The grammar does not determine which effects are allowed.
 *
 * The semantic pipeline is:
 *
 *     parsed unsafe region
 *          |
 *          v
 *     effect analysis
 *          |
 *          v
 *     capability analysis
 *          |
 *          v
 *     safety validation
 *
 * An unsafe region must therefore never be interpreted as:
 *
 *     "all effects allowed"
 *
 * unless a separately specified semantic policy explicitly establishes that
 * behavior.
 *
 * ============================================================================
 * OWNERSHIP / BORROWING INTEGRATION
 * ============================================================================
 *
 * Unsafe syntax does not itself redefine ownership, borrowing, or lifetime
 * rules.
 *
 * The semantic analyzer determines whether operations inside an unsafe region
 * satisfy the language's memory model.
 *
 * This means:
 *
 *     unsafe
 *
 * is not equivalent to:
 *
 *     disable ownership
 *
 * and not equivalent to:
 *
 *     disable lifetime checking.
 *
 * If Zamani provides explicit operations that require unsafe authorization,
 * those operations must identify their requirements through semantic metadata
 * or capability contracts rather than through additional unsafe grammar
 * variants.
 *
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * Unsafe syntax contains no physical resource limits.
 *
 * It does NOT encode:
 *
 *     maximum memory
 *     maximum devices
 *     maximum cores
 *     maximum threads
 *     maximum accelerators
 *     maximum qubits
 *     maximum nodes
 *     maximum registers
 *     maximum hardware objects
 *
 * Resource requirements belong to the resource/capability/compiler layers.
 *
 * An unsafe operation may require additional resources, but that requirement
 * must be represented semantically rather than embedded in this grammar.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum operations may occur inside an unsafe region if the quantum semantic
 * layer permits them.
 *
 * Example:
 *
 *     unsafe {
 *         quantum_operation();
 *     }
 *
 * The grammar does not determine:
 *
 *     - qubit count;
 *     - logical qubit count;
 *     - physical qubit count;
 *     - backend;
 *     - topology;
 *     - gate inventory;
 *     - pulse implementation;
 *     - calibration;
 *     - noise model;
 *     - QEC strategy;
 *     - scheduling;
 *     - routing.
 *
 * Those concerns remain downstream.
 *
 * The pipeline remains:
 *
 *     Zamani source
 *         |
 *         v
 *     parser
 *         |
 *         v
 *     frontend AST
 *         |
 *         v
 *     semantic analysis
 *         |
 *         v
 *     canonical quantum semantic representation
 *         |
 *         v
 *     quantum::ir
 *         |
 *         v
 *     optimization / routing / scheduling / execution
 *
 * `unsafe.g4` MUST NOT depend directly on `quantum::ir`.
 *
 * It MUST NOT define a quantum IR.
 *
 * It MUST NOT encode physical qubit identifiers.
 *
 * It MUST NOT encode a fixed gate inventory.
 *
 * It MUST NOT contain QEC or ZQN syntax merely because a quantum operation
 * happens to occur inside an unsafe region.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Hardware and HDL statements may occur inside an unsafe block when their
 * canonical statement grammars permit them.
 *
 * Example:
 *
 *     unsafe {
 *         hardware_operation();
 *     }
 *
 * or:
 *
 *     unsafe {
 *         configure_signal();
 *     }
 *
 * The unsafe grammar does not distinguish:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     embedded device
 *     accelerator
 *
 * Physical realization belongs downstream.
 *
 * There must be no:
 *
 *     unsafeGpuStatement
 *     unsafeFpgaStatement
 *     unsafeQpuStatement
 *     unsafeCpuStatement
 *
 * merely because the eventual target differs.
 *
 * ============================================================================
 * DISTRIBUTED / NETWORK / AI / FUTURE DOMAIN INTEGRATION
 * ============================================================================
 *
 * Unsafe blocks are domain-neutral.
 *
 * The same syntax can enclose operations belonging to:
 *
 *     classical computing
 *     quantum computing
 *     hybrid computing
 *     HDL
 *     hardware
 *     distributed computing
 *     networking
 *     AI/ML
 *     data processing
 *     scientific computing
 *     accelerators
 *     embedded systems
 *     future computational domains
 *
 * Domain-specific meaning is established after parsing.
 *
 * ============================================================================
 * NESTED UNSAFE REGIONS
 * ============================================================================
 *
 * The grammar permits nesting naturally:
 *
 *     unsafe {
 *         outer();
 *
 *         unsafe {
 *             inner();
 *         }
 *     }
 *
 * No fixed nesting depth is encoded.
 *
 * Whether nested unsafe regions are:
 *
 *     permitted
 *     redundant
 *     warned about
 *     prohibited
 *
 * is a semantic/lint/policy decision.
 *
 * This distinction is important because parser scalability must not be
 * confused with semantic policy.
 *
 * ============================================================================
 * EMPTY UNSAFE REGIONS
 * ============================================================================
 *
 * The grammar permits:
 *
 *     unsafe {}
 *
 * because `blockExpression` permits zero or more block elements.
 *
 * Semantic analysis or linting may diagnose an empty unsafe region as:
 *
 *     redundant
 *     unnecessary
 *     suspicious
 *
 * but syntax must not impose an artificial restriction unless the language
 * specification explicitly chooses to make empty blocks invalid.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This file contains no language-level limits on:
 *
 *     - unsafe block count;
 *     - statements per unsafe block;
 *     - nested unsafe regions;
 *     - program size;
 *     - expression size;
 *     - quantum operations;
 *     - qubits;
 *     - devices;
 *     - processors;
 *     - cores;
 *     - threads;
 *     - accelerators;
 *     - nodes;
 *     - memory;
 *     - hardware resources.
 *
 * Therefore the syntax scales from:
 *
 *     tiny embedded programs
 *
 * through:
 *
 *     classical systems
 *     heterogeneous systems
 *     quantum programs
 *     HDL
 *     distributed systems
 *     HPC
 *
 * and onward to future execution models.
 *
 * Any practical parser resource limit must be external, configurable, and
 * implementation-specific.
 *
 * It must NOT become part of the source-language semantics.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * The unsafe construct describes an explicit semantic boundary:
 *
 *     unsafe { operation }
 *
 * It does NOT describe:
 *
 *     a machine
 *     a device
 *     a processor
 *     a topology
 *     a fixed resource count
 *     a backend
 *     a vendor
 *     a deployment environment.
 *
 * Therefore the same source construct can be compiled for different target
 * environments while retaining the same source-level semantic intent.
 *
 * Target realization is downstream.
 *
 * This supports:
 *
 *     Program_Once
 *     Compile_Once
 *     Run_Everywhere
 *     Anywhere
 *     Forever
 *
 * subject to the semantic requirements and capabilities of the chosen
 * execution environment.
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This grammar MUST NOT introduce:
 *
 *     MAX_UNSAFE_BLOCKS
 *     MAX_UNSAFE_DEPTH
 *     MAX_OPERATIONS
 *     MAX_RESOURCES
 *     MAX_QUBITS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_ACCELERATORS
 *
 * It MUST NOT introduce:
 *
 *     device identifiers
 *     physical addresses
 *     topology identifiers
 *     vendor-specific hardware names
 *     backend-specific limits
 *
 * Any such information belongs to target/resource/capability configuration.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * Parsing an unsafe region MUST NOT itself grant authority.
 *
 * The parser only records:
 *
 *     "the programmer explicitly marked this region unsafe."
 *
 * Authorization remains downstream.
 *
 * This prevents a source parser from accidentally becoming a security
 * enforcement bypass.
 *
 * In particular, the parser must not:
 *
 *     execute the body;
 *     inspect the host;
 *     access hardware;
 *     resolve device permissions;
 *     open files;
 *     access networks;
 *     invoke foreign code.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend AST must preserve at minimum:
 *
 *     - the fact that the region is explicitly unsafe;
 *     - the ordered body;
 *     - the source span;
 *     - the nested child structure.
 *
 * Conceptually:
 *
 *     UnsafeBlock {
 *         body: BlockExpression,
 *         source_span: Span
 *     }
 *
 * The exact Rust type and NodeId representation belong to the frontend AST
 * subsystem.
 *
 * This grammar MUST NOT define Rust AST structures.
 *
 * The parser/frontend is responsible for mapping:
 *
 *     unsafeStatement
 *
 * into the canonical AST representation.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis should be able to distinguish:
 *
 *     ordinary block
 *
 * from:
 *
 *     explicitly unsafe block.
 *
 * The semantic representation should retain this distinction long enough for:
 *
 *     safety checking
 *     capability checking
 *     effect checking
 *     diagnostics
 *     auditing
 *     provenance
 *     policy enforcement
 *     optimization legality
 *     target lowering
 *
 * to operate correctly.
 *
 * The unsafe marker must not be discarded immediately after parsing.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This file creates NO IR.
 *
 * The intended flow is:
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
 *     AST
 *       |
 *       v
 *     semantic analysis
 *       |
 *       v
 *     canonical semantic representation
 *       |
 *       +--> classical IR
 *       +--> quantum::ir
 *       +--> HDL/hardware IR
 *       +--> distributed/data representations
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     scheduling / routing / lowering
 *       |
 *       v
 *     target
 *
 * There must be no:
 *
 *     grammar -> quantum::ir
 *     grammar -> runtime
 *     grammar -> hardware discovery
 *
 * dependency.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing this construct depends only on:
 *
 *     source text
 *     lexer definition
 *     parser grammar
 *     parser configuration
 *
 * It MUST NOT depend on:
 *
 *     CPU count
 *     GPU availability
 *     QPU availability
 *     FPGA availability
 *     machine topology
 *     calibration
 *     queue state
 *     scheduler state
 *     network state
 *     runtime state
 *     backend state
 *     random state
 *
 * Identical source and parser configuration must produce identical syntax
 * structure.
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * The parser/frontend diagnostic subsystem should provide source-local errors
 * for malformed unsafe constructs, including:
 *
 *     unsafe
 *     unsafe (
 *     unsafe !
 *     unsafe identifier
 *     unsafe expression
 *     unsafe EOF
 *     unsafe { ... missing }
 *
 * The grammar itself MUST NOT contain embedded diagnostic actions.
 *
 * Error recovery remains owned by the parser/frontend diagnostic subsystem.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Canonical syntax:
 *
 *     unsafe { ... }
 *
 * is compatible with the current canonical parser definition:
 *
 *     unsafeStatement
 *         : UNSAFE blockExpression
 *         ;
 *
 * The following legacy forms are intentionally not preserved as canonical
 * syntax:
 *
 *     unsafe!
 *     unsafe(...)
 *     unsafe(evas: ...)
 *
 * Migration tooling may recognize those forms in a dedicated compatibility
 * grammar or source migration layer, but this production grammar must not
 * reintroduce them.
 *
 * ============================================================================
 * INTEGRATION WITH CANONICAL PARSER
 * ============================================================================
 *
 * The canonical parser currently contains an equivalent unsafe rule.
 *
 * The repository must converge on ONE owner.
 *
 * Preferred final ownership:
 *
 *     grammar/statements/unsafe.g4
 *
 * Therefore the canonical parser assembly must:
 *
 *     1. import/include this grammar component;
 *     2. expose `unsafeStatement` from this component;
 *     3. remove the duplicate `unsafeStatement` production from the monolithic
 *        parser grammar;
 *     4. keep `statement` composition in the canonical statement dispatcher;
 *     5. ensure the generated parser contains exactly one effective
 *        `unsafeStatement` rule.
 *
 * The migration must NOT leave:
 *
 *     ZamaniParser.g4::unsafeStatement
 *
 * and:
 *
 *     UnsafeStatementsParser::unsafeStatement
 *
 * competing as independent definitions in the final assembled grammar.
 *
 * ============================================================================
 * INTEGRATION WITH statements/statements.g4
 * ============================================================================
 *
 * The canonical statement dispatcher must admit this rule.
 *
 * Preferred structure:
 *
 *     statement
 *         : ...
 *         | safetyStatement
 *         | ...
 *         ;
 *
 *     safetyStatement
 *         : unsafeStatement
 *         ;
 *
 * If the repository chooses not to introduce a separate safety category, the
 * simpler form is:
 *
 *     statement
 *         : ...
 *         | unsafeStatement
 *         | ...
 *         ;
 *
 * `unsafe.g4` itself MUST NOT own the dispatcher.
 *
 * ============================================================================
 * INTEGRATION WITH blocks.g4
 * ============================================================================
 *
 * `unsafeStatement` consumes:
 *
 *     blockExpression
 *
 * from:
 *
 *     grammar/statements/blocks.g4
 *
 * This establishes:
 *
 *     unsafeStatement
 *         -> blockExpression
 *             -> blockElement*
 *                 -> statement
 *
 * The dependency direction must remain one-way at the assembled grammar level.
 *
 * No duplicated block rule is permitted here.
 *
 * ============================================================================
 * INTEGRATION WITH EFFECTS
 * ============================================================================
 *
 * If Zamani's effect system provides unsafe-related effects, those effects
 * should be checked semantically after parsing.
 *
 * This grammar must not hard-code effect names.
 *
 * ============================================================================
 * INTEGRATION WITH SECURITY
 * ============================================================================
 *
 * Security policy may determine whether an unsafe region is:
 *
 *     allowed
 *     warned
 *     denied
 *     restricted
 *
 * based on source context, module policy, declared capabilities, build policy,
 * or deployment policy.
 *
 * The grammar must remain independent of those decisions.
 *
 * ============================================================================
 * INTEGRATION WITH QUANTUM / QEC / ZQN
 * ============================================================================
 *
 * No direct grammar dependency exists.
 *
 * If an unsafe quantum operation is parsed:
 *
 *     parser
 *       |
 *       v
 *     AST unsafe region
 *       |
 *       v
 *     semantic analysis
 *       |
 *       v
 *     quantum semantic lowering
 *       |
 *       v
 *     quantum::ir
 *
 * QEC and ZQN remain separate owners:
 *
 *     QEC
 *         -> error detection/correction
 *
 *     ZQN
 *         -> noise/fault semantics
 *
 * `unsafe.g4` does not redefine either.
 *
 * ============================================================================
 * INTEGRATION WITH HARDWARE / SCHEDULING / ROUTING
 * ============================================================================
 *
 * Unsafe syntax does not select a hardware target.
 *
 * The following remain downstream:
 *
 *     capability discovery
 *     hardware selection
 *     resource allocation
 *     routing
 *     scheduling
 *     calibration
 *     execution
 *
 * An unsafe block can therefore remain portable across different hardware
 * configurations where the required semantic capability exists.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE TESTS
 * ============================================================================
 *
 * The parser must accept:
 *
 *     unsafe {}
 *
 *     unsafe {
 *     }
 *
 *     unsafe {
 *         operation();
 *     }
 *
 *     unsafe {
 *         let value = compute();
 *         consume(value);
 *     }
 *
 *     if condition {
 *         unsafe {
 *             operation();
 *         }
 *     }
 *
 *     unsafe {
 *         unsafe {
 *             nested();
 *         }
 *     }
 *
 *     unsafe {
 *         quantum_operation();
 *     }
 *
 *     unsafe {
 *         hardware_operation();
 *     }
 *
 *     unsafe {
 *         distributed_operation();
 *     }
 *
 * ============================================================================
 * NEGATIVE TESTS
 * ============================================================================
 *
 * The parser must reject:
 *
 *     unsafe
 *
 *     unsafe!
 *
 *     unsafe()
 *
 *     unsafe(...)
 *
 *     unsafe(identifier)
 *
 *     unsafe identifier
 *
 *     unsafe expression
 *
 *     unsafe(evas: expression) {}
 *
 *     unsafe [
 *     ]
 *
 *     unsafe (
 *     )
 *
 *     unsafe {
 *
 * with a missing closing brace.
 *
 * ============================================================================
 * BOUNDARY TESTS
 * ============================================================================
 *
 * Tests must include:
 *
 *     - empty unsafe block;
 *     - one-statement unsafe block;
 *     - deeply nested unsafe blocks;
 *     - many statements;
 *     - nested blocks;
 *     - nested control flow;
 *     - mixed classical/quantum statements;
 *     - mixed quantum/hardware statements;
 *     - large source programs.
 *
 * The tests must not encode artificial maximums such as:
 *
 *     MAX_UNSAFE_DEPTH
 *     MAX_UNSAFE_STATEMENTS
 *
 * as language semantics.
 *
 * ============================================================================
 * SCALABILITY TESTS
 * ============================================================================
 *
 * The grammar must be tested with unsafe regions containing arbitrarily many
 * source statements subject only to the test runner's external resource
 * availability.
 *
 * The grammar must not impose a fixed limit on:
 *
 *     statements
 *     nested regions
 *     expressions
 *     operations
 *     qubits
 *     devices
 *     resources
 *     nodes
 *
 * ============================================================================
 * CROSS-DOMAIN TESTS
 * ============================================================================
 *
 * At minimum test syntactic composition with:
 *
 *     classical + unsafe
 *     quantum + unsafe
 *     classical + quantum + unsafe
 *     HDL + unsafe
 *     hardware + unsafe
 *     quantum + hardware + unsafe
 *     distributed + unsafe
 *     AI + unsafe
 *     data + unsafe
 *     networking + unsafe
 *
 * These tests validate that the unsafe construct remains domain-neutral.
 *
 * ============================================================================
 * ROUND-TRIP TESTS
 * ============================================================================
 *
 * Where the frontend printer/serializer exists:
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
 *     AST
 *       |
 *       v
 *     printer
 *       |
 *       v
 *     parser
 *
 * must preserve the unsafe-region semantic distinction.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file must contain:
 *
 *     no fixed resource count;
 *     no fixed hardware count;
 *     no fixed quantum count;
 *     no fixed nesting limit;
 *     no fixed statement limit;
 *     no device identifier;
 *     no physical address;
 *     no topology assumption;
 *     no vendor assumption;
 *     no backend assumption.
 *
 * The only fixed lexical keyword is:
 *
 *     unsafe
 *
 * because that is language syntax, not a machine limitation.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is COMPLETE only when:
 *
 * [ ] `unsafeStatement` is the sole canonical unsafe syntax owner.
 *
 * [ ] Only `UNSAFE` from the canonical lexer is consumed.
 *
 * [ ] No lexer rule exists in this file.
 *
 * [ ] `blockExpression` is consumed from the canonical block grammar.
 *
 * [ ] No duplicate block grammar exists here.
 *
 * [ ] No `statement` dispatcher is defined here.
 *
 * [ ] No Rust actions exist.
 *
 * [ ] No semantic predicates exist.
 *
 * [ ] No unsafe Rust exists.
 *
 * [ ] No I/O exists.
 *
 * [ ] No filesystem access exists.
 *
 * [ ] No networking exists.
 *
 * [ ] No hardware discovery exists.
 *
 * [ ] No runtime execution exists.
 *
 * [ ] No machine-size assumptions exist.
 *
 * [ ] No fixed resource limits exist.
 *
 * [ ] No fixed quantum limits exist.
 *
 * [ ] No quantum IR is defined.
 *
 * [ ] `quantum::ir` remains downstream.
 *
 * [ ] QEC remains downstream.
 *
 * [ ] ZQN remains downstream.
 *
 * [ ] Hardware abstraction remains downstream.
 *
 * [ ] Scheduling remains downstream.
 *
 * [ ] Routing remains downstream.
 *
 * [ ] Optimization remains downstream.
 *
 * [ ] The canonical parser has exactly one effective unsafeStatement rule.
 *
 * [ ] `statements.g4` admits `unsafeStatement`.
 *
 * [ ] Legacy `unsafe!` syntax is not accepted by the production grammar.
 *
 * [ ] Legacy `unsafe(evas: ...)` syntax is not accepted by the production
 *     grammar.
 *
 * [ ] Positive parser tests pass.
 *
 * [ ] Negative parser tests pass.
 *
 * [ ] Boundary tests pass.
 *
 * [ ] Cross-domain parser tests pass.
 *
 * [ ] Round-trip tests preserve the unsafe semantic boundary.
 *
 * [ ] Rust 1.97 / Rust 1.97.1 generation and integration pass.
 *
 * [ ] The resulting Rust implementation remains free of unsafe code.
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */

parser grammar UnsafeStatementsParser;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * CANONICAL UNSAFE STATEMENT
 * ============================================================================
 *
 * `UNSAFE` is owned by ZamaniLexer.
 *
 * `blockExpression` is owned by grammar/statements/blocks.g4.
 *
 * No target-specific or semantic information is encoded here.
 */

unsafeStatement
    : UNSAFE blockExpression
    ;
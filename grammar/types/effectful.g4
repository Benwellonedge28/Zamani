/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/types/effectful.g4
 *
 * Grammar:
 *     Effectful
 *
 * Status:
 *     CANONICAL production parser component for effect-qualified types.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * Safety:
 *     This grammar contains no embedded Rust actions, semantic predicates,
 *     filesystem access, network access, runtime calls, or unsafe code.
 *
 * The compiler implementation MUST remain safe Rust only.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the SYNTAX of effect qualification attached to a type.
 *
 * An effectful type communicates that the semantic behavior associated with
 * the type includes an explicitly declared effect context.
 *
 * This file deliberately does NOT define:
 *
 *     - the general type-expression grammar;
 *     - primitive types;
 *     - named types;
 *     - generic types;
 *     - tuple types;
 *     - array types;
 *     - reference types;
 *     - pointer types;
 *     - resource types;
 *     - capability types;
 *     - quantum types;
 *     - classical types;
 *     - HDL types;
 *     - hardware types;
 *     - function declarations;
 *     - function bodies;
 *     - effect declarations;
 *     - effect operations;
 *     - effect semantics;
 *     - effect inference;
 *     - effect subtyping;
 *     - effect propagation;
 *     - capability resolution;
 *     - resource allocation;
 *     - hardware selection;
 *     - scheduling;
 *     - routing;
 *     - optimization;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime dispatch.
 *
 * ============================================================================
 * ARCHITECTURAL REASON
 * ============================================================================
 *
 * The repository already has a canonical type-expression composition layer.
 *
 * It also has a separate canonical effect system:
 *
 *     grammar/effects/
 *
 * and a canonical effect-set grammar:
 *
 *     grammar/effects/effect-sets.g4
 *
 * Therefore this file MUST NOT attempt to recreate either system.
 *
 * In particular, this file MUST NOT define:
 *
 *     typeExpression
 *
 * because doing so would create a competing type-expression authority.
 *
 * It also MUST NOT import the aggregate Types grammar if Types imports this
 * file, because that would create:
 *
 *     Types -> Effectful -> Types
 *
 * circular grammar composition.
 *
 * Instead, this file defines a reusable TYPE EFFECT QUALIFIER boundary.
 *
 * The canonical type composition layer consumes this boundary.
 *
 * ============================================================================
 * AUTHORITY
 * ============================================================================
 *
 * Normative semantic authority:
 *
 *     grammar/spec/type-system.md
 *
 * Normative effect authority:
 *
 *     grammar/spec/effects.md
 *
 * Type syntax authority:
 *
 *     grammar/types/
 *
 * Effect syntax authority:
 *
 *     grammar/effects/
 *
 * Canonical lexical vocabulary:
 *
 *     grammar/lexer/tokens.g4
 *
 * Canonical lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Parser composition root:
 *
 *     grammar/Zamani.g4
 *
 * Implementation-conformance reference:
 *
 *     grammar/grammar.md
 *
 * Historical/design material:
 *
 *     grammar/Zamani-Grammar.md
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     effectfulTypeQualifier
 *     effectfulTypeClause
 *     effectfulTypeEffectSet
 *
 * It therefore owns the syntactic attachment point between:
 *
 *     Type
 *
 * and:
 *
 *     Effect Set
 *
 * THIS FILE DOES NOT OWN:
 *
 *     typeExpression
 *     effectSet
 *     effectReference
 *     effect declarations
 *     effect operations
 *     effect handlers
 *     effect inference
 *     effect subtyping
 *     capability semantics
 *     resource semantics
 *     hardware semantics
 *     quantum semantics
 *
 * ============================================================================
 * DEPENDENCY DIRECTION
 * ============================================================================
 *
 * The required dependency direction is:
 *
 *     canonical lexer
 *           |
 *           v
 *     type/effect syntax
 *           |
 *           +----------------------+
 *           |                      |
 *           v                      v
 *     type-expression       effectfulTypeQualifier
 *           |                      |
 *           +----------+-----------+
 *                      |
 *                      v
 *              canonical type model
 *                      |
 *                      v
 *                 semantic analysis
 *                      |
 *                      v
 *                  canonical IR
 *
 * There MUST NOT be:
 *
 *     Effectful -> Types -> Effectful
 *
 * or:
 *
 *     Effectful -> runtime
 *
 * or:
 *
 *     Effectful -> quantum::ir
 *
 * The grammar remains upstream of all semantic realization.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Effect qualification is semantic intent.
 *
 * It MUST NOT encode target-specific realization.
 *
 * This grammar therefore imposes no language-level limit on:
 *
 *     - number of effects;
 *     - number of effect references;
 *     - number of effect-qualified types;
 *     - number of nested type expressions;
 *     - number of generic parameters;
 *     - number of generic effect parameters;
 *     - number of functions;
 *     - number of modules;
 *     - number of quantum resources;
 *     - number of classical resources;
 *     - number of HDL resources;
 *     - number of distributed participants;
 *     - number of accelerators;
 *     - number of devices.
 *
 * The grammar MUST NOT contain:
 *
 *     MAX_EFFECTS
 *     MAX_EFFECT_REFERENCES
 *     MAX_EFFECT_DEPTH
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_THREADS
 *     MAX_MEMORY
 *
 * or equivalent language-level limits.
 *
 * Practical parser/compiler resource limits remain implementation policy.
 *
 * ============================================================================
 * EFFECT / CAPABILITY / RESOURCE SEPARATION
 * ============================================================================
 *
 * An effect describes:
 *
 *     WHAT computational behavior is observable.
 *
 * A capability describes:
 *
 *     WHAT an execution environment can provide.
 *
 * A resource describes:
 *
 *     WHAT computational resource is available or requested.
 *
 * A requirement describes:
 *
 *     WHAT must be satisfied.
 *
 * A constraint describes:
 *
 *     WHAT condition must hold.
 *
 * A preference describes:
 *
 *     WHICH valid realization is preferred.
 *
 * This file therefore MUST NOT make an effectful type imply:
 *
 *     a specific CPU;
 *     a specific GPU;
 *     a specific FPGA;
 *     a specific QPU;
 *     a physical qubit;
 *     a physical address;
 *     a memory bank;
 *     a network node;
 *     a hardware topology;
 *     a vendor;
 *     a backend.
 *
 * ============================================================================
 * OPEN-WORLD EFFECT MODEL
 * ============================================================================
 *
 * The effect set itself is owned by:
 *
 *     grammar/effects/effect-sets.g4
 *
 * Effect identities remain open-world.
 *
 * Examples include:
 *
 *     IO
 *     Network
 *     Storage
 *     Quantum
 *     Security
 *     quantum::Measurement
 *     quantum::Reset
 *     qec::Correction
 *     zqn::Observation
 *     distributed::Consensus
 *     accelerator::Tensor
 *     future::domain::effect
 *
 * This grammar MUST NOT enumerate these names.
 *
 * A new effect domain therefore does not require modifying this file merely
 * because a new semantic effect exists.
 *
 * ============================================================================
 * CANONICAL SOURCE FORM
 * ============================================================================
 *
 * The canonical effect-qualified type attachment is:
 *
 *     <type> with effect <effect-set>
 *
 * Examples:
 *
 *     Resource with effect { IO }
 *
 *     Buffer<T> with effect {
 *         IO,
 *         Network,
 *     }
 *
 *     Qubit with effect {
 *         quantum::Measurement,
 *     }
 *
 *     fn(Qubit) -> Result with effect {
 *         quantum::Measurement,
 *     }
 *
 * The exact base type is supplied by the canonical type-expression grammar.
 *
 * This file owns only the effect qualification.
 *
 * ============================================================================
 * WHY `WITH EFFECT` IS USED
 * ============================================================================
 *
 * The canonical lexer already owns:
 *
 *     WITH
 *     EFFECT
 *     EFFECTS
 *
 * and the existing function grammar already recognizes effect attachment as a
 * distinct source-level concern.
 *
 * This file therefore reuses the established vocabulary instead of inventing:
 *
 *     EFFECTFUL
 *     EFFECTFUL_TYPE
 *     EFFECT_TYPE
 *
 * lexer tokens.
 *
 * No new keyword is introduced by this file.
 *
 * ============================================================================
 * SINGULAR / PLURAL EFFECT KEYWORD
 * ============================================================================
 *
 * The canonical form uses:
 *
 *     with effect { ... }
 *
 * `EFFECTS` is intentionally not silently accepted here as a second spelling.
 *
 * This keeps the type qualifier syntax singular and avoids creating two
 * equivalent source syntaxes without a specification-level reason.
 *
 * If the language specification later standardizes:
 *
 *     with effects { ... }
 *
 * as the canonical spelling, that is a language-version change and must be
 * coordinated through the compatibility specification rather than silently
 * adding another alternative here.
 *
 * ============================================================================
 * EMPTY EFFECT SET
 * ============================================================================
 *
 * The canonical effect-set grammar permits:
 *
 *     {}
 *
 * Therefore:
 *
 *     T with effect {}
 *
 * is syntactically valid.
 *
 * It MUST NOT automatically become synonymous with:
 *
 *     pure T
 *
 * or:
 *
 *     no effects
 *
 * unless the normative semantic specification explicitly defines that
 * equivalence.
 *
 * The parser preserves the distinction.
 *
 * ============================================================================
 * DUPLICATE EFFECTS
 * ============================================================================
 *
 * Duplicate effect references are syntactically valid when accepted by the
 * canonical effect-set grammar.
 *
 * Example:
 *
 *     T with effect {
 *         IO,
 *         IO,
 *     }
 *
 * Semantic analysis is responsible for canonicalization.
 *
 * The grammar MUST NOT attempt to decide whether duplicate semantic identities
 * are redundant.
 *
 * ============================================================================
 * EFFECT ORDER
 * ============================================================================
 *
 * Source order is preserved by the parse tree.
 *
 * Semantic effect-set equality is determined by canonical effect identity,
 * not source formatting.
 *
 * The grammar MUST NOT impose ordering semantics on effects.
 *
 * ============================================================================
 * EFFECTFUL FUNCTION TYPES
 * ============================================================================
 *
 * Function types are owned by:
 *
 *     grammar/types/function.g4
 *
 * That file already defines the canonical:
 *
 *     functionType
 *
 * and explicitly delegates effectful function-type integration to the effect
 * system.
 *
 * Therefore this file provides the reusable suffix:
 *
 *     effectfulTypeClause
 *
 * so the function-type composition layer can construct:
 *
 *     functionType
 *         + effectfulTypeClause?
 *
 * without duplicating effect syntax.
 *
 * The resulting semantic type can represent:
 *
 *     FunctionType
 *         parameters
 *         returnType
 *         effects
 *
 * without creating:
 *
 *     QuantumFunctionType
 *     GPUFunctionType
 *     HDLFunctionType
 *     VendorFunctionType
 *
 * ============================================================================
 * GENERAL EFFECTFUL TYPES
 * ============================================================================
 *
 * This grammar permits the same effect qualification mechanism to be used by
 * any semantic type category for which the type system authorizes effect
 * qualification.
 *
 * Examples may include:
 *
 *     T with effect { IO }
 *
 *     Stream<T> with effect { Network }
 *
 *     QuantumState with effect { quantum::Measurement }
 *
 *     HardwareSignal with effect { hardware::Timing }
 *
 *     Distributed<T> with effect { distributed::Communication }
 *
 * Whether a particular base type is legally effect-qualified is a semantic
 * question.
 *
 * The parser must not encode a closed list of eligible types.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum effect qualification is generic.
 *
 * Example:
 *
 *     Qubit with effect {
 *         quantum::Measurement,
 *     }
 *
 * The grammar does NOT define:
 *
 *     QubitId
 *     PhysicalQubitId
 *     GateKind
 *     coupling maps
 *     topology
 *     calibration
 *     noise models
 *     QEC codes
 *     ZQN faults
 *     routing
 *     scheduling
 *
 * Quantum semantics remain:
 *
 *     source
 *       |
 *       v
 *     frontend AST
 *       |
 *       v
 *     semantic effect/type analysis
 *       |
 *       v
 *     quantum::ir
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     routing / scheduling / QEC / ZQN
 *       |
 *       v
 *     HAL / target realization
 *
 * `quantum::ir` remains the canonical quantum semantic boundary.
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Classical types can use the same effect qualifier.
 *
 * Example:
 *
 *     Buffer with effect {
 *         IO,
 *         Network,
 *     }
 *
 * The grammar does not need a separate:
 *
 *     ClassicalEffectfulType
 *
 * because effectfulness is a cross-domain type property.
 *
 * ============================================================================
 * HDL INTEGRATION
 * ============================================================================
 *
 * Hardware/HDL semantic types may be effect-qualified where the semantic type
 * system permits it.
 *
 * Example:
 *
 *     HardwareSignal with effect {
 *         hardware::Timing,
 *     }
 *
 * The grammar does not select:
 *
 *     FPGA
 *     ASIC
 *     CPU
 *     GPU
 *
 * and does not encode physical timing implementation.
 *
 * ============================================================================
 * DISTRIBUTED / NETWORK / AI INTEGRATION
 * ============================================================================
 *
 * The same syntax remains available for:
 *
 *     distributed types;
 *     network-aware types;
 *     accelerator types;
 *     data types;
 *     AI/ML types;
 *     security types;
 *     future domain types.
 *
 * New domains must not require modification of this grammar simply to attach
 * an effect set.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar creates syntax contexts only.
 *
 * The frontend AST should preserve the source structure in the existing
 * domain-neutral type representation.
 *
 * Conceptually:
 *
 *     TypeExpr
 *         base
 *         effect_qualification?
 *
 * where the effect qualification contains:
 *
 *     effect-set source structure
 *     source span
 *
 * The exact Rust AST representation is owned by:
 *
 *     src/frontend/ast/
 *
 * This grammar MUST NOT introduce:
 *
 *     QuantumEffectfulType
 *     GPUEffectfulType
 *     HDLEffectfulType
 *     VendorEffectfulType
 *
 * or another domain-specific AST hierarchy.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing establishes only:
 *
 *     "this type has an explicit source-level effect qualification."
 *
 * Semantic analysis determines:
 *
 *     - whether the base type exists;
 *     - whether the base type may be effect-qualified;
 *     - whether every referenced effect exists;
 *     - whether effects are imported;
 *     - whether effect identities are compatible;
 *     - whether duplicate effects normalize;
 *     - whether effect parameters are valid;
 *     - whether effect polymorphism applies;
 *     - whether effect inclusion/subtyping holds;
 *     - whether effect requirements are satisfied;
 *     - whether the type is usable in the enclosing context.
 *
 * The grammar performs none of these checks.
 *
 * ============================================================================
 * EFFECT POLYMORPHISM
 * ============================================================================
 *
 * Effect-polymorphic type expressions remain possible because the effect set
 * grammar is open-world and the semantic system can resolve effect parameters.
 *
 * For example, a future canonical semantic type may contain an abstract effect
 * parameter:
 *
 *     T with effect { E }
 *
 * where `E` is introduced by an enclosing generic/effect context.
 *
 * This file does not introduce a new syntax for effect variables.
 *
 * Generic/effect parameter declaration remains owned by the appropriate
 * generic/effect grammar.
 *
 * ============================================================================
 * EFFECT INFERENCE
 * ============================================================================
 *
 * This grammar does not infer effects.
 *
 * For example:
 *
 *     T
 *
 * must not be rewritten by the parser into:
 *
 *     T with effect { IO }
 *
 * because an expression elsewhere happens to perform IO.
 *
 * Effect inference belongs to semantic analysis.
 *
 * Explicit source annotations remain available for:
 *
 *     API contracts;
 *     verification;
 *     optimization;
 *     diagnostics;
 *     security analysis;
 *     interoperability;
 *     documentation.
 *
 * ============================================================================
 * EFFECT SUBTYPING / INCLUSION
 * ============================================================================
 *
 * The parser does not decide:
 *
 *     { IO } ⊆ { IO, Network }
 *
 * or any other effect inclusion relation.
 *
 * Such relations belong to semantic analysis and the normative effect/type
 * specifications.
 *
 * ============================================================================
 * CAPABILITY SEPARATION
 * ============================================================================
 *
 * This:
 *
 *     T with effect {
 *         quantum::Measurement,
 *     }
 *
 * means that the semantic type is associated with quantum measurement
 * behavior.
 *
 * It does NOT mean:
 *
 *     requires capability("quantum.measurement")
 *
 * although semantic analysis may derive such a requirement.
 *
 * Capability derivation belongs to capability/resource analysis.
 *
 * ============================================================================
 * RESOURCE SEPARATION
 * ============================================================================
 *
 * This:
 *
 *     T with effect { Network }
 *
 * does not specify:
 *
 *     network count;
 *     node count;
 *     link count;
 *     bandwidth;
 *     physical address;
 *     topology.
 *
 * Resource requirements belong to:
 *
 *     grammar/resources/
 *     grammar/hardware/
 *     grammar/compile/
 *     grammar/execution/
 *
 * ============================================================================
 * HARDWARE INDEPENDENCE
 * ============================================================================
 *
 * Effectful types MUST remain portable.
 *
 * This grammar must not contain:
 *
 *     cpuType
 *     gpuType
 *     fpgaType
 *     qpuType
 *     physicalDeviceType
 *     physicalQubitType
 *
 * merely to represent effects.
 *
 * Hardware-specific types, if semantically required, are supplied by the
 * canonical hardware/resource type system.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * Effect qualification lowers through:
 *
 *     source type syntax
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic type/effect model
 *          |
 *          v
 *     canonical semantic representation / ZUIR
 *          |
 *          +-------------------+
 *          |                   |
 *          v                   v
 *     classical IR        quantum::ir
 *          |                   |
 *          +---------+---------+
 *                    |
 *                    v
 *               target lowering
 *
 * This grammar MUST NOT create an independent:
 *
 *     EffectIR
 *     QuantumEffectIR
 *     HardwareEffectIR
 *
 * solely for parsing.
 *
 * ============================================================================
 * COMPILER INTEGRATION
 * ============================================================================
 *
 * The compiler consumes the semantic effect-qualified type after type/effect
 * checking.
 *
 * It may use effect information for:
 *
 *     - legality checking;
 *     - optimization;
 *     - effect propagation;
 *     - specialization;
 *     - inlining decisions;
 *     - concurrency analysis;
 *     - resource analysis;
 *     - domain lowering;
 *     - verification;
 *     - interoperability.
 *
 * The compiler must not infer a particular machine solely from this grammar.
 *
 * ============================================================================
 * RUNTIME INTEGRATION
 * ============================================================================
 *
 * Runtime realization is downstream.
 *
 * Effectful type information may influence runtime behavior only after
 * semantic lowering.
 *
 * The grammar itself MUST NOT:
 *
 *     - execute an effect;
 *     - invoke a device;
 *     - allocate a resource;
 *     - open a network connection;
 *     - access a filesystem;
 *     - access a QPU;
 *     - access a GPU;
 *     - invoke a vendor SDK.
 *
 * ============================================================================
 * TOOLING INTEGRATION
 * ============================================================================
 *
 * The parse tree must preserve enough information for:
 *
 *     - syntax highlighting;
 *     - formatting;
 *     - IDE completion;
 *     - refactoring;
 *     - diagnostics;
 *     - source-to-source transformation;
 *     - documentation generation;
 *     - semantic indexing.
 *
 * Source spans must remain recoverable for:
 *
 *     WITH
 *     EFFECT
 *     effect set
 *
 * and the complete effect-qualified type.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This grammar uses the canonical lexer vocabulary:
 *
 *     tokenVocab = ZamaniTokens
 *
 * It MUST NOT define lexer rules.
 *
 * The canonical lexical ownership is:
 *
 *     grammar/lexer/
 *
 * In particular, this file reuses existing canonical tokens:
 *
 *     WITH
 *     EFFECT
 *     LBRACE
 *     RBRACE
 *
 * and the tokens consumed by:
 *
 *     effectSet
 *
 * from:
 *
 *     grammar/effects/effect-sets.g4
 *
 * No local fallback literal such as:
 *
 *     'with'
 *     'effect'
 *
 * is introduced.
 *
 * This prevents a second lexical authority.
 *
 * ============================================================================
 * EFFECT-SET INTEGRATION
 * ============================================================================
 *
 * The canonical effect-set grammar owns:
 *
 *     effectSet
 *
 * Therefore this grammar imports:
 *
 *     EffectSets
 *
 * and consumes its canonical `effectSet` rule.
 *
 * It MUST NOT redefine:
 *
 *     effectSet
 *     effectReference
 *     effectReferenceList
 *
 * or any other effect-set rule.
 *
 * ============================================================================
 * ANTLR COMPOSITION
 * ============================================================================
 *
 * This file is intentionally a small delegate grammar.
 *
 * Canonical composition is:
 *
 *     grammar/lexer/
 *          |
 *          v
 *     grammar/antlr/ZamaniLexer.g4
 *          |
 *          v
 *     grammar/types/effectful.g4
 *          |
 *          +---- grammar/effects/effect-sets.g4
 *          |
 *          v
 *     canonical type-expression composition
 *          |
 *          v
 *     grammar/Zamani.g4
 *
 * The aggregate parser is responsible for combining the type-expression rule
 * with this qualifier.
 *
 * This file does not import the aggregate `Types` grammar.
 *
 * ============================================================================
 * INTEGRATION WITH `grammar/types/function.g4`
 * ============================================================================
 *
 * `grammar/types/function.g4` owns:
 *
 *     functionType
 *
 * and currently has the canonical shape:
 *
 *     fn(parameters) -> returnType
 *
 * Its effectful extension must be composed as:
 *
 *     functionType
 *         functionTypeEffectClause?
 *
 * where:
 *
 *     functionTypeEffectClause
 *         -> effectfulTypeClause
 *
 * or an equivalent one-way composition boundary.
 *
 * The important invariant is:
 *
 *     function.g4
 *         |
 *         v
 *     Effectful
 *
 * and NOT:
 *
 *     Function
 *         <-->
 *     Effectful
 *
 * circularly.
 *
 * ============================================================================
 * INTEGRATION WITH `grammar/effects/effect-sets.g4`
 * ============================================================================
 *
 * This file consumes the canonical:
 *
 *     effectSet
 *
 * rule.
 *
 * Therefore:
 *
 *     grammar/effects/effect-sets.g4
 *
 * remains the sole owner of:
 *
 *     effectReference
 *     effectReferenceList
 *     effectSet
 *
 * This file only attaches that set to a type.
 *
 * ============================================================================
 * INTEGRATION WITH `grammar/expressions/effects.g4`
 * ============================================================================
 *
 * Expression-level effect use:
 *
 *     perform quantum::Measurement(q)
 *
 * remains owned by:
 *
 *     grammar/expressions/effects.g4
 *
 * Type-level effect qualification:
 *
 *     T with effect { quantum::Measurement }
 *
 * is owned by this file.
 *
 * These are intentionally distinct.
 *
 * Expression effect use answers:
 *
 *     WHAT operation occurs?
 *
 * Type effect qualification answers:
 *
 *     WHAT effect context is part of this type?
 *
 * The two must not be merged into one parser rule.
 *
 * ============================================================================
 * INTEGRATION WITH `grammar/effects/effects.g4`
 * ============================================================================
 *
 * Effect declaration/use semantics remain owned by:
 *
 *     grammar/effects/effects.g4
 *
 * This file only consumes the effect-set syntax required for type
 * qualification.
 *
 * ============================================================================
 * INTEGRATION WITH `grammar/types/function.g4`
 * ============================================================================
 *
 * An effect-qualified function type may therefore eventually have the semantic
 * shape:
 *
 *     FunctionType {
 *         parameters,
 *         return_type,
 *         effects
 *     }
 *
 * without introducing a separate:
 *
 *     EffectfulFunctionType
 *
 * AST hierarchy unless the existing semantic model explicitly requires such a
 * distinction.
 *
 * ============================================================================
 * INTEGRATION WITH QUANTUM
 * ============================================================================
 *
 * A quantum function type can therefore carry effects without any quantum
 * grammar duplication:
 *
 *     fn(Qubit) -> Measurement
 *         with effect {
 *             quantum::Measurement
 *         }
 *
 * The parser remains domain-neutral.
 *
 * Quantum-specific interpretation occurs after semantic resolution.
 *
 * ============================================================================
 * INTEGRATION WITH RESOURCE / HARDWARE SYSTEMS
 * ============================================================================
 *
 * Effectful types must remain separate from:
 *
 *     requires
 *     capability
 *     resource
 *     target
 *     placement
 *     topology
 *
 * For example:
 *
 *     T with effect { Network }
 *
 * does not itself mean:
 *
 *     requires network resource X
 *
 * The resource/capability systems may derive requirements later.
 *
 * ============================================================================
 * VERSIONING
 * ============================================================================
 *
 * This file is syntax-versioned only.
 *
 * Adding a new effect name does NOT require changing this grammar.
 *
 * Adding a new effect domain does NOT require changing this grammar.
 *
 * Adding a new quantum effect does NOT require changing this grammar.
 *
 * Adding a new hardware effect does NOT require changing this grammar.
 *
 * A grammar version change is required only if the source syntax of effect
 * qualification changes.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing type syntax that does not contain an effect qualifier remains
 * unchanged.
 *
 * For example:
 *
 *     int
 *     Buffer<T>
 *     Qubit
 *     fn(int) -> int
 *
 * remain ordinary types.
 *
 * Effect qualification is additive:
 *
 *     T
 *
 * may additionally become:
 *
 *     T with effect { E }
 *
 * subject to semantic validation.
 *
 * No existing type spelling is reinterpreted by this grammar.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar:
 *
 *     - contains no actions;
 *     - contains no semantic predicates;
 *     - performs no I/O;
 *     - performs no network access;
 *     - performs no hardware discovery;
 *     - performs no random operations;
 *     - performs no semantic inference.
 *
 * The same token sequence under the same grammar/version produces the same
 * parse structure.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Parsing an effectful type MUST NOT cause the effect to execute.
 *
 * In particular:
 *
 *     T with effect { IO }
 *
 * does NOT perform IO.
 *
 *     T with effect { Network }
 *
 * does NOT contact a network.
 *
 *     T with effect { quantum::Measurement }
 *
 * does NOT access quantum hardware.
 *
 * The grammar is purely declarative.
 *
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * Structural diagnostics belong to the parser/frontend.
 *
 * Examples:
 *
 *     T with
 *
 *     T with effect
 *
 *     T with effect {
 *
 *     T with effect { , }
 *
 *     T with effect { IO
 *
 *     T with effect { IO, }
 *
 * The final example is valid if accepted by the canonical effect-set grammar.
 *
 * Semantic diagnostics belong downstream.
 *
 * Examples:
 *
 *     invalid effect identity
 *     unavailable effect
 *     incompatible effect
 *     effect not permitted for base type
 *     invalid effect parameter
 *     unsatisfied effect requirement
 *
 * ============================================================================
 * PERFORMANCE
 * ============================================================================
 *
 * The grammar uses delegation and repetition rather than enumerating effects.
 *
 * No semantic lookup occurs during parsing.
 *
 * No external resources are accessed.
 *
 * This permits parser performance to scale with source size without making
 * parsing complexity depend on:
 *
 *     hardware;
 *     network;
 *     QPU state;
 *     GPU state;
 *     resource discovery;
 *     runtime state.
 *
 * ============================================================================
 * POSITIVE TEST CONTRACT
 * ============================================================================
 *
 * The following forms are structurally valid provided their base types are
 * valid in the surrounding canonical type grammar:
 *
 *     T with effect {}
 *
 *     T with effect { IO }
 *
 *     T with effect { IO, Network }
 *
 *     T with effect {
 *         IO,
 *         Network,
 *     }
 *
 *     Qubit with effect {
 *         quantum::Measurement,
 *     }
 *
 *     Buffer<T> with effect {
 *         IO,
 *     }
 *
 *     Distributed<T> with effect {
 *         distributed::Consensus,
 *     }
 *
 *     FutureType with effect {
 *         future::domain::operation,
 *     }
 *
 * ============================================================================
 * NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * The following must be rejected structurally:
 *
 *     T with
 *
 *     T with effect
 *
 *     T with effect {
 *
 *     T with effect { , IO }
 *
 *     T with effect { IO,, Network }
 *
 *     T with effect { IO Network }
 *
 *     T with effect } 
 *
 *     T with effect { IO
 *
 * ============================================================================
 * BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Test:
 *
 *     empty effect set;
 *     one effect;
 *     many effects;
 *     qualified effects;
 *     deeply qualified effect names;
 *     duplicate effects;
 *     trailing commas;
 *     nested generic base types;
 *     function types;
 *     quantum types;
 *     HDL types;
 *     distributed types;
 *     resource types;
 *     capability-bearing types;
 *     future domain types.
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * The grammar must permit effect sets whose size is limited only by available
 * implementation resources.
 *
 * There must be no test asserting:
 *
 *     "N effects is the maximum."
 *
 * Instead test progressively larger valid effect sets until an explicit
 * implementation resource policy is reached.
 *
 * A resource-exhaustion condition must be reported as an implementation
 * resource failure, not as a language-level type restriction.
 *
 * ============================================================================
 * DETERMINISM TEST CONTRACT
 * ============================================================================
 *
 * The same source must produce the same parse structure regardless of:
 *
 *     CPU;
 *     GPU;
 *     FPGA;
 *     QPU;
 *     operating system;
 *     memory topology;
 *     network topology;
 *     runtime state;
 *     device availability.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no:
 *
 *     machine counts;
 *     device IDs;
 *     qubit IDs;
 *     memory sizes;
 *     register widths;
 *     topology sizes;
 *     accelerator counts;
 *     node counts;
 *     thread counts;
 *     fixed effect catalog;
 *     fixed effect cardinality;
 *     vendor identifiers;
 *     backend identifiers.
 *
 * The effect vocabulary remains open-world.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [ ] canonical lexer vocabulary is used;
 *     [ ] no lexer rules are duplicated;
 *     [ ] effectSet comes from effect-sets.g4;
 *     [ ] effectSet is not redefined;
 *     [ ] typeExpression is not redefined;
 *     [ ] Types -> Effectful circularity is avoided;
 *     [ ] function-type integration is defined;
 *     [ ] expression effect syntax remains separate;
 *     [ ] effect declaration syntax remains separate;
 *     [ ] effect semantics remain downstream;
 *     [ ] AST mapping is documented;
 *     [ ] semantic mapping is documented;
 *     [ ] IR mapping is documented;
 *     [ ] compiler integration is documented;
 *     [ ] runtime integration is documented;
 *     [ ] quantum integration is documented;
 *     [ ] classical integration is documented;
 *     [ ] HDL integration is documented;
 *     [ ] distributed integration is documented;
 *     [ ] resource/capability separation is preserved;
 *     [ ] no artificial hardware limits exist;
 *     [ ] no fixed effect catalogue exists;
 *     [ ] positive tests exist;
 *     [ ] negative tests exist;
 *     [ ] boundary tests exist;
 *     [ ] scalability tests exist;
 *     [ ] compatibility tests exist;
 *     [ ] determinism tests exist;
 *     [ ] security requirements are satisfied;
 *     [ ] Rust implementation remains compatible with Rust 1.97/1.97.1;
 *     [ ] no unsafe Rust is introduced;
 *     [ ] no second type authority exists.
 *
 * ============================================================================
 */

parser grammar Effectful;

options {
    tokenVocab = ZamaniTokens;
}

import EffectSets;


/*
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * `effectfulTypeClause` is the canonical reusable type-level effect
 * qualification.
 *
 * It is deliberately a suffix rather than a complete type expression.
 *
 * This prevents a circular dependency with the canonical type-expression
 * grammar.
 */
effectfulTypeClause
    : WITH
      EFFECT
      effectfulTypeEffectSet
    ;


/*
 * ============================================================================
 * EFFECT SET
 * ============================================================================
 *
 * The actual effect-set grammar is owned by:
 *
 *     grammar/effects/effect-sets.g4
 *
 * This wrapper provides a stable type-system integration boundary without
 * duplicating the effect-set grammar.
 */
effectfulTypeEffectSet
    : effectSet
    ;


/*
 * ============================================================================
 * QUALIFIER ALIAS
 * ============================================================================
 *
 * `effectfulTypeQualifier` is the semantic-name-friendly integration rule for
 * the type composition layer.
 *
 * It intentionally delegates to the canonical clause.
 */
effectfulTypeQualifier
    : effectfulTypeClause
    ;
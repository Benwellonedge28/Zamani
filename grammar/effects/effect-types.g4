/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/effects/effect-types.g4
 *
 * Grammar:
 *     EffectTypes
 *
 * Status:
 *     CANONICAL MODULAR PRODUCTION GRAMMAR FOR EFFECT-QUALIFIED TYPES
 *
 * Language:
 *     Zamani
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * Safety:
 *     This grammar contains no embedded Rust actions, semantic predicates,
 *     filesystem access, network access, runtime calls, hardware discovery,
 *     target selection, or unsafe code.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the SOURCE-LEVEL SYNTAX that attaches an effect context to
 * an already-existing type expression.
 *
 * It is deliberately a small, dependency-safe boundary between:
 *
 *     grammar/types/
 *
 * and:
 *
 *     grammar/effects/
 *
 * It does NOT become a second type grammar.
 *
 * It does NOT become a second effect-set grammar.
 *
 * It does NOT define effect declarations.
 *
 * It does NOT define effect operations.
 *
 * It does NOT define effect semantics.
 *
 * It does NOT define capability semantics.
 *
 * It does NOT define resource requirements.
 *
 * It does NOT define hardware targets.
 *
 * ============================================================================
 * PRIMARY SOURCE FORM
 * ============================================================================
 *
 * The canonical form represented here is:
 *
 *     <type-expression> with effect <effect-set>
 *
 * Examples:
 *
 *     Buffer<T> with effect { IO }
 *
 *     Resource with effect {
 *         Storage,
 *         Network,
 *     }
 *
 *     Qubit with effect {
 *         quantum::Measurement,
 *     }
 *
 *     Result<Value, Error> with effect {
 *         IO,
 *         Security,
 *     }
 *
 *     fn(Qubit) -> Result<Value, Error> with effect {
 *         quantum::Measurement,
 *     }
 *
 * The base type is owned by the canonical type grammar.
 *
 * The effect set is owned by:
 *
 *     grammar/effects/effect-sets.g4
 *
 * This file owns only their syntactic attachment.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *                         Zamani source
 *                              |
 *                              v
 *                     canonical Zamani lexer
 *                              |
 *                              v
 *                     parser composition
 *                              |
 *                 +------------+-------------+
 *                 |                          |
 *                 v                          v
 *          canonical type syntax       effect syntax
 *                 |                          |
 *                 |                 effect-sets.g4
 *                 |                          |
 *                 +------------+-------------+
 *                              |
 *                              v
 *                     effect-qualified type
 *                              |
 *                              v
 *                         frontend AST
 *                              |
 *                +-------------+-------------+
 *                |             |             |
 *                v             v             v
 *             type          effect       capability
 *            analysis       analysis       analysis
 *                |             |             |
 *                +-------------+-------------+
 *                              |
 *                              v
 *                     semantic representation
 *                              |
 *                +-------------+-------------+
 *                |             |             |
 *                v             v             v
 *           classical      quantum::ir    HDL/hardware
 *                              |
 *                              v
 *                    canonical IR pipeline
 *                              |
 *                  optimization / lowering
 *                              |
 *                  routing / scheduling
 *                              |
 *                  QEC / resilience / ZQN
 *                              |
 *                             HAL
 *                              |
 *                       target realization
 *
 * This file MUST remain upstream of all semantic realization.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     effectQualifiedType
 *     effectTypeQualifier
 *     effectTypeClause
 *     effectTypeQualifierList
 *     effectTypeQualification
 *
 * These rules describe the syntactic relationship between:
 *
 *     typeExpression
 *
 * and:
 *
 *     effectSet
 *
 * THIS FILE DOES NOT OWN:
 *
 *     typeExpression
 *     typeCore
 *     typePostfix
 *     type arguments
 *     generic type syntax
 *     effectSet
 *     effectReference
 *     effect declarations
 *     effect operations
 *     effect handlers
 *     effect inference
 *     effect subtyping
 *     effect normalization
 *     capability declarations
 *     capability resolution
 *     resource declarations
 *     resource allocation
 *     requirements
 *     constraints
 *     preferences
 *     hardware selection
 *     target selection
 *     routing
 *     scheduling
 *     QEC
 *     ZQN
 *     HAL
 *     runtime dispatch
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * This file MUST NOT redefine any rule owned by:
 *
 *     grammar/types/types.g4
 *     grammar/effects/effect-sets.g4
 *
 * In particular, it MUST NOT define:
 *
 *     typeExpression
 *     effectSet
 *     effectReference
 *
 * This prevents:
 *
 *     Type -> EffectTypes -> Type
 *
 * and prevents:
 *
 *     EffectTypes -> competing effect-set grammar
 *
 * ============================================================================
 * DEPENDENCY DIRECTION
 * ============================================================================
 *
 * The intended dependency direction is:
 *
 *     ZamaniLexer
 *          |
 *          v
 *     EffectTypes
 *          |
 *          +--------------------+
 *          |                    |
 *          v                    v
 *     Types contract       EffectSets contract
 *          |                    |
 *          +---------+----------+
 *                    |
 *                    v
 *            frontend AST
 *                    |
 *                    v
 *            semantic analysis
 *
 * This grammar MUST NOT depend on:
 *
 *     semantic analysis
 *     compiler
 *     runtime
 *     hardware
 *     quantum::ir
 *     QEC
 *     ZQN
 *     HAL
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Effect-qualified types express portable semantic intent.
 *
 * They MUST NOT encode physical implementation limits.
 *
 * This grammar therefore imposes no language-level limit on:
 *
 *     - number of effect-qualified types;
 *     - number of effects in an effect set;
 *     - number of type qualifications;
 *     - number of generic parameters;
 *     - number of type parameters;
 *     - number of nested type expressions;
 *     - number of modules;
 *     - number of functions;
 *     - number of quantum resources;
 *     - number of classical resources;
 *     - number of HDL resources;
 *     - number of accelerators;
 *     - number of distributed participants;
 *     - number of target devices.
 *
 * The grammar MUST NOT define:
 *
 *     MAX_EFFECTS
 *     MAX_EFFECT_SET_SIZE
 *     MAX_EFFECT_QUALIFIERS
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_DEVICES
 *     MAX_TENSOR_RANK
 *     MAX_TENSOR_DIMENSION
 *
 * or equivalent language-level limits.
 *
 * Practical parser/compiler resource limits belong to explicit implementation
 * policy and MUST NOT become language semantics.
 *
 * ============================================================================
 * EFFECT / CAPABILITY / RESOURCE SEPARATION
 * ============================================================================
 *
 * EFFECT
 *     Describes computational behavior that is observable or semantically
 *     relevant.
 *
 * CAPABILITY
 *     Describes what an execution environment can provide.
 *
 * RESOURCE
 *     Describes computational resources that exist or are requested.
 *
 * REQUIREMENT
 *     Describes what must be satisfied for a realization to be valid.
 *
 * CONSTRAINT
 *     Describes conditions a realization must obey.
 *
 * PREFERENCE
 *     Describes a preferred realization among valid realizations.
 *
 * HINT
 *     Provides implementation guidance without changing program meaning.
 *
 * An effect-qualified type MUST NOT silently become:
 *
 *     a hardware requirement;
 *     a device selection;
 *     a physical placement;
 *     a resource allocation;
 *     a scheduling directive;
 *     a routing directive.
 *
 * ============================================================================
 * OPEN-WORLD EFFECT MODEL
 * ============================================================================
 *
 * Effect identities are open-world.
 *
 * The parser must not enumerate effect names such as:
 *
 *     IO
 *     Network
 *     Storage
 *     Quantum
 *     Measurement
 *     GPU
 *     CPU
 *     FPGA
 *     QPU
 *     Security
 *     Cryptography
 *     Distributed
 *     AI
 *
 * as a closed grammar catalogue.
 *
 * Qualified effect names remain ordinary semantic identities through the
 * canonical effect-set grammar.
 *
 * Examples:
 *
 *     quantum::Measurement
 *     qec::Correction
 *     zqn::Observation
 *     distributed::Consensus
 *     accelerator::Tensor
 *     photonic::Interaction
 *     neuromorphic::Spike
 *     future::computing::operation
 *
 * A future effect domain therefore does not require modifying this grammar.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Quantum effects may appear in an effect-qualified type:
 *
 *     Qubit with effect {
 *         quantum::Measurement,
 *     }
 *
 * This grammar does NOT define:
 *
 *     QubitId
 *     PhysicalQubitId
 *     GateKind
 *     physical topology
 *     calibration
 *     noise
 *     QEC codes
 *     decoder algorithms
 *     ZQN faults
 *
 * Quantum semantic lowering remains downstream.
 *
 * The canonical quantum semantic boundary remains:
 *
 *     quantum::ir
 *
 * No second quantum IR may be introduced by this grammar.
 *
 * ============================================================================
 * TYPE-SYSTEM BOUNDARY
 * ============================================================================
 *
 * The base type is supplied by the canonical type system.
 *
 * This grammar therefore accepts:
 *
 *     typeExpression
 *
 * but does not define it.
 *
 * Examples of possible base types include:
 *
 *     T
 *     Buffer<T>
 *     Tensor<T, Shape>
 *     Qubit
 *     QRegister<N>
 *     Result<Value, Error>
 *     fn(Input) -> Output
 *     Resource<Capability>
 *
 * Whether any particular type is semantically legal with a particular effect
 * is determined by semantic analysis.
 *
 * ============================================================================
 * TYPE QUALIFICATION SEMANTICS
 * ============================================================================
 *
 * The grammar preserves the source distinction between:
 *
 *     T
 *
 * and:
 *
 *     T with effect { E }
 *
 * Semantic analysis determines:
 *
 *     - whether effect qualification is permitted for the type;
 *     - whether the effect set is valid;
 *     - whether duplicate effects normalize;
 *     - whether effects are inherited;
 *     - whether effects participate in type identity;
 *     - whether effect qualification affects subtyping;
 *     - whether effect qualification affects generic substitution;
 *     - whether effect qualification participates in function compatibility.
 *
 * The parser MUST NOT make those decisions.
 *
 * ============================================================================
 * EMPTY EFFECT SET
 * ============================================================================
 *
 * The canonical effect-set grammar may accept:
 *
 *     {}
 *
 * Therefore:
 *
 *     T with effect {}
 *
 * is syntactically representable when the canonical effect-set grammar permits
 * it.
 *
 * This grammar MUST NOT reinterpret it as:
 *
 *     pure T
 *
 * or:
 *
 *     no effects
 *
 * unless the normative semantic specification explicitly establishes that
 * equivalence.
 *
 * ============================================================================
 * DUPLICATE EFFECTS
 * ============================================================================
 *
 * This file does not reject duplicate effect references.
 *
 * For example, if effect-sets.g4 accepts:
 *
 *     T with effect {
 *         IO,
 *         IO,
 *     }
 *
 * then this file accepts the qualified type.
 *
 * Semantic normalization belongs downstream.
 *
 * This prevents parser syntax from becoming an accidental effect-set algebra.
 *
 * ============================================================================
 * SOURCE ORDER
 * ============================================================================
 *
 * Parse-tree/source order is preserved.
 *
 * Semantic effect-set equality and normalization are downstream concerns.
 *
 * The parser MUST NOT sort effect references.
 *
 * The parser MUST NOT deduplicate effect references.
 *
 * The parser MUST NOT canonicalize namespace aliases.
 *
 * ============================================================================
 * MULTIPLE QUALIFIERS
 * ============================================================================
 *
 * This file intentionally defines ONE effect qualification boundary:
 *
 *     typeExpression with effect effectSet
 *
 * It does not silently permit an arbitrary repeated chain such as:
 *
 *     T with effect { A } with effect { B }
 *
 * unless the normative type specification explicitly adopts repeated
 * qualification as a language feature.
 *
 * This avoids introducing two competing source models:
 *
 *     one combined effect set
 *
 * versus:
 *
 *     an ordered sequence of effect qualifications.
 *
 * If multiple effect clauses are required later, they should be represented
 * by the canonical effect-set composition instead.
 *
 * ============================================================================
 * ANNOTATION / ATTRIBUTE SEPARATION
 * ============================================================================
 *
 * This grammar does not attach arbitrary attributes to effect-qualified types.
 *
 * General attributes belong to the canonical attributes grammar.
 *
 * Effect metadata belongs to the semantic effect model.
 *
 * This prevents:
 *
 *     effect-types.g4
 *
 * from becoming another declaration/attribute grammar.
 *
 * ============================================================================
 * GENERIC TYPE INTEGRATION
 * ============================================================================
 *
 * Generic type arguments remain owned by the type grammar.
 *
 * This grammar must therefore support source forms such as:
 *
 *     Buffer<T> with effect { IO }
 *
 *     Tensor<T, Shape> with effect { accelerator::Tensor }
 *
 * without inspecting or interpreting the generic arguments.
 *
 * The effect qualifier applies to the complete preceding type expression.
 *
 * ============================================================================
 * FUNCTION-TYPE INTEGRATION
 * ============================================================================
 *
 * A function type may itself carry an effect qualification where the
 * normative type system permits it.
 *
 * Conceptually:
 *
 *     fn(Input) -> Output with effect { IO }
 *
 * This grammar does not redefine function types.
 *
 * The canonical function-type grammar remains responsible for the function
 * type itself.
 *
 * Semantic analysis determines whether an effect-qualified function type is
 * compatible with:
 *
 *     function declarations;
 *     closures;
 *     generic functions;
 *     higher-order functions;
 *     callbacks;
 *     foreign interfaces.
 *
 * ============================================================================
 * RESOURCE AND HARDWARE INTEGRATION
 * ============================================================================
 *
 * An effect-qualified type may eventually participate in resource analysis.
 *
 * For example:
 *
 *     QuantumState<T> with effect {
 *         quantum::Measurement,
 *     }
 *
 * does NOT select:
 *
 *     a QPU;
 *     a physical qubit;
 *     a backend;
 *     a topology;
 *     a calibration;
 *     a device identifier.
 *
 * Resource and capability analysis happens after parsing.
 *
 * ============================================================================
 * AI / HDL / DISTRIBUTED / NETWORKING INTEGRATION
 * ============================================================================
 *
 * The same syntax remains valid across domains.
 *
 * Examples:
 *
 *     Tensor<T, Shape> with effect {
 *         accelerator::Tensor,
 *     }
 *
 *     Signal<T> with effect {
 *         hdl::Clock,
 *     }
 *
 *     Message<T> with effect {
 *         distributed::Network,
 *     }
 *
 *     Model<T> with effect {
 *         ai::Inference,
 *     }
 *
 *     Packet<T> with effect {
 *         networking::Transmit,
 *     }
 *
 * No domain-specific grammar extension is required merely because a new
 * effect namespace is introduced.
 *
 * ============================================================================
 * INTEROPERABILITY
 * ============================================================================
 *
 * Effect-qualified types may be lowered to external representations such as:
 *
 *     C
 *     C++
 *     Rust
 *     WebAssembly
 *     OpenQASM
 *     QIR
 *     HDL
 *     vendor-specific targets
 *
 * Such lowering is downstream.
 *
 * The grammar must not encode external ABI layout or target-specific effect
 * implementations.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser must preserve enough structure for the frontend AST to represent:
 *
 *     base type
 *     effect qualification
 *     effect-set source span
 *     complete qualified-type source span
 *
 * Conceptual mapping:
 *
 *     effectQualifiedType
 *         ->
 *     existing TypeExpr / effect-qualification representation
 *
 * The grammar MUST NOT introduce a second TypeExpr hierarchy.
 *
 * The grammar MUST NOT introduce:
 *
 *     QuantumEffectType
 *     HardwareEffectType
 *     GPUEffectType
 *     QPUEffectType
 *
 * merely because an effect has a domain-specific meaning.
 *
 * ============================================================================
 * SOURCE-SPAN CONTRACT
 * ============================================================================
 *
 * The parser must preserve source locations for:
 *
 *     - the base type;
 *     - the `with` keyword;
 *     - the `effect` keyword;
 *     - the effect set;
 *     - the complete qualified type.
 *
 * Source spans are consumed downstream by:
 *
 *     diagnostics;
 *     AST construction;
 *     semantic analysis;
 *     IDE tooling;
 *     formatting;
 *     refactoring;
 *     provenance;
 *     compatibility tooling.
 *
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * Syntax diagnostics belong to the parser.
 *
 * Semantic diagnostics belong to semantic analysis.
 *
 * Parser-level malformed forms include:
 *
 *     T with
 *     T with effect
 *     T with effect {
 *     T with effect {
 *         IO
 *     }
 *
 * when the effect-set grammar requires a closing delimiter.
 *
 * Semantic errors include:
 *
 *     unknown effect;
 *     unavailable effect;
 *     illegal effect for type;
 *     invalid effect parameter;
 *     incompatible effect-qualified type;
 *     forbidden effect propagation.
 *
 * This grammar must not perform semantic validation.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no semantic predicates;
 *     - no embedded actions;
 *     - no runtime callbacks;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware discovery;
 *     - no target selection;
 *     - no randomness.
 *
 * The parse result therefore depends only on:
 *
 *     source token stream;
 *     selected grammar version;
 *     imported grammar contracts.
 *
 * ============================================================================
 * ANTLR COMPOSITION CONTRACT
 * ============================================================================
 *
 * Canonical production lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Canonical lexer vocabulary:
 *
 *     grammar/lexer/tokens.g4
 *
 * Parser grammars in the production hierarchy consume:
 *
 *     tokenVocab = ZamaniLexer
 *
 * This file therefore uses:
 *
 *     tokenVocab = ZamaniLexer
 *
 * It MUST NOT redefine lexical tokens.
 *
 * ============================================================================
 * IMPORT CONTRACT
 * ============================================================================
 *
 * This grammar requires the canonical effect-set syntax.
 *
 * It imports:
 *
 *     EffectSets
 *
 * It does NOT import:
 *
 *     Types
 *     Effects
 *     ZamaniParser
 *     runtime grammars
 *     semantic grammars
 *
 * The base type is supplied by the consuming type composition grammar.
 *
 * This is intentional.
 *
 * If this file imported Types while Types imported EffectTypes, the result
 * would be a circular grammar dependency:
 *
 *     Types -> EffectTypes -> Types
 *
 * Therefore the type grammar owns composition, while this file owns only the
 * reusable effect qualification suffix.
 *
 * ============================================================================
 * REUSABLE QUALIFIER CONTRACT
 * ============================================================================
 *
 * The central reusable rule is:
 *
 *     effectTypeQualifier
 *
 * A type composition grammar may therefore consume:
 *
 *     typeExpression
 *     effectTypeQualifier?
 *
 * without duplicating effect syntax.
 *
 * ============================================================================
 * CANONICAL COMPOSITION
 * ============================================================================
 *
 * The intended integration is:
 *
 *     grammar/types/types.g4
 *
 * or the appropriate type-composition component:
 *
 *     typeExpression
 *         : ...
 *           effectTypeQualifier?
 *         ;
 *
 * The effect-qualified suffix is therefore parsed exactly once.
 *
 * ============================================================================
 * EXISTING `grammar/types/effectful.g4`
 * ============================================================================
 *
 * The repository already contains:
 *
 *     grammar/types/effectful.g4
 *
 * That file currently occupies an overlapping architectural boundary.
 *
 * To avoid two authorities:
 *
 *     effectful.g4
 *
 * MUST become a compatibility/delegation wrapper around this file, or its
 * effect-qualification rules must be removed in favor of the canonical
 * `effectTypeQualifier` defined here.
 *
 * It MUST NOT independently define a competing effect qualifier.
 *
 * No filename rename is required.
 *
 * ============================================================================
 * EXISTING EFFECT GRAMMAR INTEGRATION
 * ============================================================================
 *
 * This file does not replace:
 *
 *     grammar/effects/effect-declarations.g4
 *     grammar/effects/effect-sets.g4
 *     grammar/effects/effect-handling.g4
 *     grammar/effects/effects.g4
 *
 * Their responsibilities remain:
 *
 * effect-declarations.g4
 *     Effect declarations and declared operations.
 *
 * effect-sets.g4
 *     Effect references and effect collections.
 *
 * effect-handling.g4
 *     Effect handler syntax.
 *
 * effects.g4
 *     Aggregate effect grammar/composition.
 *
 * effect-types.g4
 *     Effect qualification attached to a type.
 *
 * ============================================================================
 * EFFECT DECLARATION INTEGRATION
 * ============================================================================
 *
 * This grammar MUST NOT define:
 *
 *     effectDeclaration
 *
 * because that belongs to:
 *
 *     grammar/effects/effect-declarations.g4
 *
 * A declaration such as:
 *
 *     effect IO;
 *
 * is therefore distinct from:
 *
 *     Buffer<T> with effect { IO }
 *
 * The former declares an effect.
 *
 * The latter references an effect as part of a type qualification.
 *
 * ============================================================================
 * EFFECT SET INTEGRATION
 * ============================================================================
 *
 * `effectTypeQualifier` consumes the canonical:
 *
 *     effectSet
 *
 * from:
 *
 *     EffectSets
 *
 * This means changes to effect-set syntax occur in one place only.
 *
 * This file MUST NOT duplicate:
 *
 *     effectReference
 *     effectReferenceList
 *     effectSet
 *
 * ============================================================================
 * NO RESOURCE HARD-CODING
 * ============================================================================
 *
 * The following are explicitly forbidden in this file:
 *
 *     fixed qubit counts;
 *     fixed CPU counts;
 *     fixed GPU counts;
 *     fixed FPGA counts;
 *     fixed QPU counts;
 *     fixed accelerator counts;
 *     fixed memory sizes;
 *     fixed register widths;
 *     fixed topology sizes;
 *     fixed node counts;
 *     fixed network widths;
 *     fixed tensor ranks;
 *     fixed tensor dimensions.
 *
 * Numeric literals remain ordinary source-language semantics when they appear
 * in types or effect parameters.
 *
 * The grammar must never reinterpret a number as a universal hardware limit.
 *
 * ============================================================================
 * PERFORMANCE / SCALABILITY
 * ============================================================================
 *
 * This grammar uses direct composition rather than semantic lookups.
 *
 * It must not:
 *
 *     resolve names;
 *     allocate compiler resources based on target hardware;
 *     query capability registries;
 *     query hardware;
 *     normalize effect sets;
 *     sort effect references;
 *     deduplicate effect references.
 *
 * This keeps parsing deterministic and allows downstream implementations to
 * choose resource-aware data structures without changing source syntax.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * This grammar must remain a pure parser grammar.
 *
 * It must not:
 *
 *     execute arbitrary source expressions;
 *     evaluate effect parameters;
 *     invoke external commands;
 *     load plugins;
 *     inspect files;
 *     inspect environment variables;
 *     access secrets;
 *     contact network services;
 *     discover devices.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * The stable source form is:
 *
 *     <type-expression> with effect <effect-set>
 *
 * Existing source forms implemented by:
 *
 *     grammar/types/effectful.g4
 *
 * must be mapped to this rule without silently changing their semantic
 * meaning.
 *
 * Any spelling change must be handled through the compatibility/versioning
 * layer rather than by accepting multiple undocumented spellings indefinitely.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive tests MUST cover:
 *
 *     T with effect {}
 *
 *     T with effect { IO }
 *
 *     T with effect { IO, Network }
 *
 *     Buffer<T> with effect { Storage }
 *
 *     Qubit with effect { quantum::Measurement }
 *
 *     Result<Value, Error> with effect {
 *         IO,
 *         Security,
 *     }
 *
 *     Tensor<T, Shape> with effect {
 *         accelerator::Tensor,
 *     }
 *
 *     Message<T> with effect {
 *         distributed::Network,
 *     }
 *
 *     deeply qualified effects;
 *
 *     large effect sets;
 *
 *     deeply nested generic types;
 *
 *     symbolic type parameters.
 *
 * Negative tests MUST cover malformed syntax such as:
 *
 *     T with
 *
 *     T with effect
 *
 *     T with effect {
 *
 *     T with effect {
 *         IO
 *
 *     T with effect } 
 *
 *     with effect { IO }
 *
 *     effect { IO }
 *
 * where these forms are invalid under the enclosing type grammar.
 *
 * Boundary tests MUST cover:
 *
 *     empty effect set;
 *     one effect;
 *     many effects;
 *     trailing commas where effect-sets.g4 permits them;
 *     deeply qualified effect names;
 *     long identifiers;
 *     deeply nested generic types;
 *     source-order preservation.
 *
 * Scalability tests MUST verify:
 *
 *     no fixed effect-set cardinality;
 *     no fixed type-qualification count;
 *     no hardware-dependent parsing;
 *     deterministic parsing;
 *     no artificial machine-size limits.
 *
 * Cross-domain tests MUST cover:
 *
 *     classical;
 *     quantum;
 *     HDL;
 *     hardware;
 *     AI;
 *     distributed;
 *     networking;
 *     security;
 *     accelerator;
 *     future/vendor namespaces.
 *
 * Compatibility tests MUST verify:
 *
 *     effectful.g4 consumers;
 *     types.g4 consumers;
 *     effect-sets.g4 consumers;
 *     aggregate effects grammar;
 *     Rust frontend parser;
 *     AST effect/type representation.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete only when:
 *
 *     [ ] effectQualifiedType is uniquely owned;
 *     [ ] effectTypeQualifier is uniquely owned;
 *     [ ] effectSet remains owned by EffectSets;
 *     [ ] typeExpression remains owned by Types;
 *     [ ] no circular grammar dependency exists;
 *     [ ] canonical ZamaniLexer vocabulary is consumed;
 *     [ ] no lexical rules are duplicated;
 *     [ ] no effect catalogue is hard-coded;
 *     [ ] no hardware limits exist;
 *     [ ] no target decisions exist;
 *     [ ] source spans are preserved by the parser architecture;
 *     [ ] AST mapping is defined;
 *     [ ] semantic mapping is defined;
 *     [ ] IR destination is downstream;
 *     [ ] quantum::ir remains canonical;
 *     [ ] QEC remains downstream;
 *     [ ] ZQN remains downstream;
 *     [ ] HAL remains downstream;
 *     [ ] positive tests exist;
 *     [ ] negative tests exist;
 *     [ ] boundary tests exist;
 *     [ ] scalability tests exist;
 *     [ ] determinism tests exist;
 *     [ ] compatibility tests exist;
 *     [ ] cross-domain tests exist;
 *     [ ] no Rust unsafe integration is required;
 *     [ ] effectful.g4 no longer competes with this file;
 *     [ ] aggregate effects.g4 no longer duplicates this ownership;
 *     [ ] the feature can be completed without reopening this file merely
 *         because another downstream subsystem changes.
 *
 * ============================================================================
 * GRAMMAR
 * ============================================================================
 */

parser grammar EffectTypes;

options {
    tokenVocab = ZamaniLexer;
}

import EffectSets;


/*
 * ============================================================================
 * PUBLIC EFFECT-QUALIFIED TYPE SUFFIX
 * ============================================================================
 *
 * This is the principal reusable rule.
 *
 * It deliberately contains no base type.
 *
 * The consuming type grammar owns:
 *
 *     typeExpression
 *
 * and may compose:
 *
 *     typeExpression effectTypeQualifier?
 *
 * This avoids a Types <-> EffectTypes circular dependency.
 */

effectTypeQualifier
    : effectTypeClause
    ;


/*
 * ============================================================================
 * EFFECT TYPE CLAUSE
 * ============================================================================
 *
 * Canonical form:
 *
 *     with effect { ... }
 *
 * The effect-set syntax is entirely delegated to EffectSets.
 */

effectTypeClause
    : K_WITH
      K_EFFECT
      effectSet
    ;


/*
 * ============================================================================
 * NAMED ALIAS
 * ============================================================================
 *
 * `effectTypeQualification` is provided as a stable integration name for
 * AST/parser tooling that wants a noun-like rule without redefining the
 * actual syntax.
 *
 * It is intentionally an alias to the canonical qualifier.
 */

effectTypeQualification
    : effectTypeQualifier
    ;


/*
 * ============================================================================
 * LIST FORM
 * ============================================================================
 *
 * Some type-composition tooling operates on a sequence of type qualifiers.
 *
 * The language itself currently defines one canonical qualification boundary.
 *
 * This rule is an integration helper and MUST NOT be used by the canonical
 * type grammar to silently introduce repeated `with effect` clauses.
 */

effectTypeQualifierList
    : effectTypeQualifier
    ;
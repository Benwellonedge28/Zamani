/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/types/temporal.g4
 *
 * Grammar:
 *     TemporalTypes
 *
 * Status:
 *     Production-ready modular SOURCE-TYPE grammar component.
 *
 * Purpose:
 *     Defines the source-level temporal type constructor used by Zamani's
 *     type system.
 *
 * Canonical source form:
 *
 *     MTS<T>
 *
 * where T is supplied by the canonical type-expression grammar.
 *
 * ============================================================================
 * ARCHITECTURAL CONTRACT
 * ============================================================================
 *
 * This file owns the syntactic identity of the temporal type constructor.
 *
 * It deliberately does NOT own the complete type-expression grammar.
 *
 * The complete type-expression composition remains owned by:
 *
 *     grammar/types/types.g4
 *
 * This boundary is intentional.
 *
 * A tempting implementation would be:
 *
 *     temporalType
 *         : MTS LT typeExpression GT
 *         ;
 *
 * directly inside this file.
 *
 * That would make this grammar depend on the complete Types grammar when
 * Types itself imports this grammar, creating a circular grammar dependency.
 *
 * Instead this file owns:
 *
 *     temporalTypeConstructor
 *
 * and the canonical type composition grammar owns:
 *
 *     temporalType
 *         : temporalTypeConstructor
 *           LT
 *           typeExpression
 *           GT
 *         ;
 *
 * This makes this file independently complete and composable.
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
 *     parser
 *          |
 *          v
 *     temporal type syntax
 *          |
 *          v
 *     frontend AST TypeExpr::Temporal
 *          |
 *          v
 *     structural validation
 *          |
 *          v
 *     semantic temporal-type analysis
 *          |
 *          +-----------------------+-----------------------+
 *          |                       |                       |
 *          v                       v                       v
 *      classical              quantum::ir             HDL/data
 *       semantics              semantics              semantics
 *          |                       |                       |
 *          +-----------------------+-----------------------+
 *                                  |
 *                                  v
 *                         canonical semantic IR
 *                                  |
 *                     optimization / target lowering
 *                                  |
 *                  routing / scheduling / resilience
 *                                  |
 *                         ZQN / temporal noise
 *                                  |
 *                                 HAL
 *                                  |
 *                         target realization
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - temporalTypeConstructor;
 *   - the source-level distinction between an MTS temporal type constructor
 *     and ordinary named/generic types;
 *   - the canonical MTS type-constructor token boundary;
 *   - syntactic integration metadata for temporal types;
 *   - the temporal-type grammar contract.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - lexer token definitions;
 *   - identifier syntax;
 *   - the complete typeExpression;
 *   - generic type argument parsing;
 *   - arrays;
 *   - slices;
 *   - tuples;
 *   - functions;
 *   - references;
 *   - pointers;
 *   - dependent types;
 *   - quantum type syntax;
 *   - hardware type syntax;
 *   - resource type syntax;
 *   - capability type syntax;
 *   - temporal runtime behavior;
 *   - temporal state storage;
 *   - MTS timeline allocation;
 *   - timeline scheduling;
 *   - causality checking;
 *   - time evaluation;
 *   - temporal arithmetic;
 *   - temporal noise;
 *   - ZQN implementation;
 *   - QEC;
 *   - routing;
 *   - calibration;
 *   - HAL;
 *   - target selection;
 *   - runtime representation;
 *   - ABI layout.
 *
 * ============================================================================
 * AUTHORITATIVE DOCUMENTS
 * ============================================================================
 *
 * Source syntax and type meaning are governed by the repository's existing
 * language/type contracts, including:
 *
 *     grammar/spec/syntax.md
 *     grammar/spec/type-system.md
 *     grammar/specification/types.md
 *
 * This file supplies the modular grammar implementation for that contract.
 *
 * It must not silently introduce a new temporal language.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This is a PARSER grammar.
 *
 * It defines NO lexer rules.
 *
 * The canonical lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * and its documented lexical composition is under:
 *
 *     grammar/lexer/
 *
 * The temporal constructor uses the already-established:
 *
 *     MTS
 *
 * token.
 *
 * The lexer also contains:
 *
 *     ZAMANI
 *     SASA
 *
 * but those tokens have broader temporal/Sankofa language meanings and are
 * intentionally NOT treated as aliases for the MTS type constructor here.
 *
 * In particular:
 *
 *     MTS<T>
 *
 * is a type constructor.
 *
 * `zamani` and `sasa` are not silently reinterpreted as types by this grammar.
 *
 * This avoids conflating:
 *
 *     temporal type semantics
 *
 * with:
 *
 *     temporal program/control-flow semantics.
 *
 * ============================================================================
 * CANONICAL TOKEN CONTRACT
 * ============================================================================
 *
 * Required parser-visible token:
 *
 *     MTS
 *
 * No new lexer token is required by this file.
 *
 * The surrounding type composition grammar supplies:
 *
 *     LT
 *     GT
 *     typeExpression
 *
 * according to the canonical parser vocabulary.
 *
 * This file MUST NOT introduce:
 *
 *     TEMPORAL
 *     TEMPORAL_TYPE
 *     MTS_TYPE
 *     TEMPORAL_KEYWORD
 *
 * aliases merely to support this grammar.
 *
 * One source spelling must have one canonical lexical identity.
 *
 * ============================================================================
 * CANONICAL SOURCE FORM
 * ============================================================================
 *
 * The established temporal type form is:
 *
 *     MTS<T>
 *
 * Examples:
 *
 *     MTS<int>
 *     MTS<State>
 *     MTS<QuantumState>
 *     MTS<Resource<T>>
 *     MTS<Vec<T>>
 *     MTS<quantum::State>
 *
 * The inner type is deliberately unrestricted at the grammar component level.
 *
 * `T` is supplied by `types.g4`.
 *
 * ============================================================================
 * WHY THIS FILE DOES NOT PARSE THE OPERAND
 * ============================================================================
 *
 * The complete operand belongs to `types.g4`.
 *
 * This file therefore does NOT define:
 *
 *     temporalType
 *         : MTS LT typeExpression GT
 *         ;
 *
 * as its public rule.
 *
 * Instead:
 *
 *     temporalTypeConstructor
 *         : MTS
 *         ;
 *
 * is the independent boundary.
 *
 * The composition layer then performs:
 *
 *     temporalTypeConstructor
 *          +
 *     LT
 *          +
 *     typeExpression
 *          +
 *     GT
 *
 * This is the same modular principle already used by the repository's
 * specialized type grammars.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar does not create an AST.
 *
 * The canonical frontend AST remains:
 *
 *     src/frontend/ast/node/types/type_expr.rs
 *
 * A complete source type:
 *
 *     MTS<T>
 *
 * must ultimately map to the existing:
 *
 *     TypeExpr::Temporal(...)
 *
 * representation.
 *
 * The inner `T` remains an ordinary canonical `TypeExpr`.
 *
 * Conceptually:
 *
 *     MTS<T>
 *        |
 *        v
 *     Temporal(T)
 *
 * No temporal-specific second AST is permitted.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing answers:
 *
 *     "Did the programmer write an MTS temporal type?"
 *
 * Semantic analysis answers:
 *
 *     "What does that temporal type mean in this program?"
 *
 * Semantic analysis may determine:
 *
 *   - the temporal domain;
 *   - timeline semantics;
 *   - snapshot semantics;
 *   - causality constraints;
 *   - temporal validity;
 *   - temporal ownership;
 *   - temporal resource requirements;
 *   - compatibility with effects;
 *   - compatibility with quantum operations;
 *   - compatibility with distributed execution;
 *   - compatibility with persistent/Sankofa state;
 *   - lowering requirements.
 *
 * None of those decisions belong in this grammar.
 *
 * ============================================================================
 * MTS SEMANTIC BOUNDARY
 * ============================================================================
 *
 * `MTS<T>` represents a SOURCE-LEVEL temporal abstraction.
 *
 * It does NOT mean:
 *
 *     - one fixed timeline;
 *     - one fixed timestamp width;
 *     - one fixed clock frequency;
 *     - one fixed clock domain;
 *     - one fixed number of timelines;
 *     - one fixed amount of temporal storage;
 *     - one fixed number of snapshots;
 *     - one fixed runtime queue;
 *     - one physical clock;
 *     - one processor;
 *     - one QPU;
 *     - one simulator.
 *
 * Those are implementation/target concerns.
 *
 * ============================================================================
 * TEMPORAL TYPE VS TEMPORAL CONTROL
 * ============================================================================
 *
 * Zamani contains broader temporal concepts including:
 *
 *     MTS
 *     zamani
 *     sasa
 *     temporal state
 *     causality
 *     snapshots
 *     timeline operations
 *     temporal noise
 *     temporal reasoning
 *
 * This file owns ONLY:
 *
 *     MTS<T>
 *
 * as a source-level TYPE constructor.
 *
 * It must not absorb:
 *
 *     zamani blocks;
 *     sasa blocks;
 *     timeline statements;
 *     temporal expressions;
 *     temporal runtime commands;
 *     causal-control statements.
 *
 * Those belong to their respective grammar domains.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Temporal types may wrap quantum abstractions:
 *
 *     MTS<Qubit>
 *     MTS<QuantumState>
 *     MTS<LogicalQubit>
 *     MTS<QuantumRegister>
 *
 * The grammar does not decide whether these forms are semantically legal.
 *
 * If the inner type is quantum:
 *
 *     MTS<QuantumType>
 *          |
 *          v
 *     frontend TypeExpr::Temporal
 *          |
 *          v
 *     semantic quantum/temporal analysis
 *          |
 *          v
 *     quantum::ir
 *
 * `quantum::ir` remains the canonical quantum semantic boundary.
 *
 * This grammar must never create:
 *
 *     TemporalQuantumIR
 *     TemporalQubitIR
 *     MTSQuantumIR
 *
 * merely because a temporal type wraps a quantum type.
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Examples:
 *
 *     MTS<int>
 *     MTS<float>
 *     MTS<Vec<T>>
 *     MTS<Matrix<T, R, C>>
 *
 * are source-level type forms.
 *
 * Their concrete storage, representation and execution strategy remain
 * downstream decisions.
 *
 * ============================================================================
 * HDL INTEGRATION
 * ============================================================================
 *
 * Temporal types may wrap HDL-level semantic abstractions where the language
 * specification permits them.
 *
 * Examples can include future/source-level abstractions such as:
 *
 *     MTS<Signal<T>>
 *     MTS<StateRegister<T>>
 *
 * without this grammar knowing whether the target is:
 *
 *     FPGA
 *     ASIC
 *     simulator
 *     reconfigurable device
 *     future hardware
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * A temporal type may contain a distributed semantic type:
 *
 *     MTS<DistributedState<T>>
 *
 * The grammar does not determine:
 *
 *     node count;
 *     placement;
 *     replication;
 *     consistency;
 *     transport;
 *     topology.
 *
 * Those concerns belong to distributed/resource/runtime semantics.
 *
 * ============================================================================
 * SANKOFA INTEGRATION
 * ============================================================================
 *
 * The repository contains Sankofa-related temporal memory concepts.
 *
 * Temporal types may therefore eventually wrap semantic memory abstractions,
 * but this grammar must remain independent of the Sankofa runtime.
 *
 * For example:
 *
 *     MTS<KnowledgeState>
 *
 * is source-level syntax.
 *
 * The grammar does not execute:
 *
 *     remember
 *     recall
 *     learn
 *
 * and does not access persistent memory.
 *
 * ============================================================================
 * MTS RUNTIME INTEGRATION
 * ============================================================================
 *
 * The repository already contains runtime MTS infrastructure responsible for
 * timeline lifecycle, temporal state management and causality-related runtime
 * behavior.
 *
 * The grammar must not duplicate that implementation.
 *
 * The integration chain is:
 *
 *     MTS<T>
 *        |
 *        v
 *     TypeExpr::Temporal
 *        |
 *        v
 *     semantic temporal type
 *        |
 *        v
 *     canonical semantic/IR representation
 *        |
 *        v
 *     compiler lowering
 *        |
 *        v
 *     runtime MTS implementation
 *
 * ============================================================================
 * FRONTIER / TEMPORAL IR INTEGRATION
 * ============================================================================
 *
 * The repository already contains temporal/goal IR infrastructure.
 *
 * This grammar does not directly construct that IR.
 *
 * The correct boundary is:
 *
 *     grammar
 *        |
 *        v
 *     frontend AST
 *        |
 *        v
 *     semantic temporal model
 *        |
 *        v
 *     appropriate canonical IR
 *
 * A grammar rule must never directly select a runtime IR opcode.
 *
 * ============================================================================
 * ZQN INTEGRATION
 * ============================================================================
 *
 * Temporal noise is already modeled in the quantum/ZQN subsystem.
 *
 * A temporal type may participate in a semantic program that eventually
 * requires temporal noise analysis.
 *
 * The grammar does NOT own:
 *
 *     noise models;
 *     drift;
 *     temporal correlations;
 *     temporal sampling;
 *     calibration;
 *     fault classification.
 *
 * Those remain ZQN/noise semantics.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * This file contains NO machine/resource limits.
 *
 * It MUST NOT define:
 *
 *     MAX_TIMELINES
 *     MAX_TEMPORAL_DEPTH
 *     MAX_SNAPSHOTS
 *     MAX_TIMESTAMPS
 *     MAX_TEMPORAL_VALUES
 *     MAX_MTS_BRANCHES
 *     MAX_MTS_HISTORY
 *     MAX_TEMPORAL_STORAGE
 *     MAX_CLOCKS
 *     MAX_CLOCK_DOMAINS
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_MEMORY
 *
 * A temporal type can therefore participate in programs from tiny embedded
 * computations to arbitrarily large computations subject only to:
 *
 *     language semantics;
 *     compiler policy;
 *     available compilation resources;
 *     available execution resources;
 *     target capabilities.
 *
 * "Unbounded" here means:
 *
 *     no artificial finite ceiling is encoded by this grammar.
 *
 * It does NOT claim that physical machines possess infinite resources.
 *
 * ============================================================================
 * NO HARD-CODED HARDWARE
 * ============================================================================
 *
 * This grammar must never encode target identities such as:
 *
 *     cpu0
 *     gpu0
 *     qpu0
 *     fpga0
 *     node0
 *     memory_bank0
 *     physical_qubit0
 *
 * as temporal types.
 *
 * Such names, if legal in some program context, remain ordinary source-level
 * identifiers and are interpreted by downstream semantics.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY SEPARATION
 * ============================================================================
 *
 * A temporal type is NOT itself a resource request.
 *
 * These concepts must remain separate:
 *
 *     Type:
 *         MTS<T>
 *
 *     Requirement:
 *         requires temporal capability ...
 *
 *     Capability:
 *         capability("temporal.snapshot")
 *
 *     Preference:
 *         prefer persistent temporal storage
 *
 *     Implementation decision:
 *         select concrete runtime mechanism
 *
 * The grammar must not collapse these concepts.
 *
 * ============================================================================
 * TYPE NESTING
 * ============================================================================
 *
 * Temporal types may be nested through the surrounding type grammar where the
 * language type system permits them.
 *
 * Examples include:
 *
 *     MTS<T>
 *     MTS<Resource<T>>
 *     MTS<QuantumState<T>>
 *     MTS<Vec<T>>
 *
 * This component does not establish a finite nesting depth.
 *
 * Semantic validation may reject invalid compositions.
 *
 * ============================================================================
 * SOURCE SPANS / DIAGNOSTICS
 * ============================================================================
 *
 * The parser/frontend adapter must retain source spans covering:
 *
 *     MTS
 *     <
 *     inner type
 *     >
 *
 * so diagnostics can identify the complete temporal type.
 *
 * This grammar performs no semantic diagnostics.
 *
 * Examples of downstream semantic diagnostics may include:
 *
 *     INVALID_TEMPORAL_TYPE_ARGUMENT
 *     TEMPORAL_TYPE_NOT_SUPPORTED_IN_CONTEXT
 *     INVALID_TEMPORAL_COMPOSITION
 *     TEMPORAL_RESOURCE_REQUIREMENT_UNSATISFIED
 *
 * These are semantic/resource diagnostics, not lexer diagnostics.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar:
 *
 *   - contains no actions;
 *   - contains no semantic predicates;
 *   - performs no I/O;
 *   - performs no runtime calls;
 *   - does not inspect hardware;
 *   - does not inspect environment variables;
 *   - does not depend on clock state;
 *   - does not depend on random state.
 *
 * Therefore parsing of the same token stream is deterministic.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * This grammar contains no Rust code and no `unsafe`.
 *
 * It does not:
 *
 *   - execute temporal operations;
 *   - allocate runtime timelines;
 *   - access persistent memory;
 *   - access network resources;
 *   - access hardware;
 *   - evaluate user code.
 *
 * Generated Rust parser integration must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *
 * and the repository's safe-Rust requirement.
 *
 * ============================================================================
 * PERFORMANCE
 * ============================================================================
 *
 * The public rule is a single-token constructor boundary:
 *
 *     temporalTypeConstructor
 *         : MTS
 *         ;
 *
 * It introduces no unbounded local repetition and no semantic predicate.
 *
 * The surrounding `types.g4` owns recursive type composition.
 *
 * This keeps this component small and predictable while allowing the complete
 * type language to remain extensible.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing syntax:
 *
 *     MTS<T>
 *
 * remains the canonical temporal-type spelling.
 *
 * This file does NOT introduce:
 *
 *     Temporal<T>
 *     temporal<T>
 *     zamani<T>
 *     sasa<T>
 *
 * as additional type spellings.
 *
 * Those would require an explicit language-specification decision and
 * corresponding lexer/compatibility work.
 *
 * In particular, the existing `ZAMANI` and `SASA` lexer tokens are not silently
 * repurposed as temporal-type aliases.
 *
 * ============================================================================
 * INTEGRATION WITH grammar/types/types.g4
 * ============================================================================
 *
 * `grammar/types/types.g4` should import this grammar and compose:
 *
 *     temporalType
 *         : temporalTypeConstructor
 *           LT
 *           typeExpression
 *           GT
 *         ;
 *
 * Its `typePrimary` should contain:
 *
 *     temporalType
 *
 * exactly once.
 *
 * The old direct implementation:
 *
 *     temporalType
 *         : MTS LT typeExpression GT
 *         ;
 *
 * should be replaced by the delegated constructor form so this file becomes
 * the single owner of the MTS temporal constructor.
 *
 * ============================================================================
 * INTEGRATION WITH grammar/antlr/Types.g4
 * ============================================================================
 *
 * The legacy/implementation-conformance ANTLR type grammar already exposes
 * `temporalType`.
 *
 * Its temporal production should converge on the same canonical structure:
 *
 *     temporalType
 *         : temporalTypeConstructor
 *           LT
 *           typeExpression
 *           GT
 *         ;
 *
 * No second temporal grammar should be introduced.
 *
 * ============================================================================
 * INTEGRATION WITH grammar/antlr/Core.g4
 * ============================================================================
 *
 * `Core.g4` currently contains the established equivalent:
 *
 *     temporalType
 *         : MTS LT typeExpression GT
 *         ;
 *
 * During grammar consolidation, that rule becomes a composition/conformance
 * consumer of this modular component rather than a competing owner.
 *
 * ============================================================================
 * INTEGRATION WITH grammar/Zamani.g4
 * ============================================================================
 *
 * `grammar/Zamani.g4` remains the composition root.
 *
 * It should consume the canonical type expression from:
 *
 *     grammar/types/types.g4
 *
 * It must not introduce another:
 *
 *     temporalType
 *
 * rule.
 *
 * ============================================================================
 * INTEGRATION WITH grammar/lexer/
 * ============================================================================
 *
 * Required canonical token:
 *
 *     MTS
 *
 * Existing temporal lexical vocabulary:
 *
 *     MTS
 *     ZAMANI
 *     SASA
 *
 * remains owned by the lexer.
 *
 * This file does not define or duplicate those tokens.
 *
 * ============================================================================
 * INTEGRATION WITH grammar/spec/
 * ============================================================================
 *
 * This file must remain consistent with:
 *
 *     grammar/spec/syntax.md
 *     grammar/spec/type-system.md
 *
 * If those documents change the canonical spelling of temporal types, the
 * compatibility process must update the language contract before changing
 * this grammar.
 *
 * ============================================================================
 * INTEGRATION WITH FRONTEND AST
 * ============================================================================
 *
 * Parser/frontend mapping:
 *
 *     temporalType
 *          |
 *          v
 *     TypeExpr::Temporal(inner)
 *
 * Source spans must be retained by the parser/AST adapter.
 *
 * The grammar itself contains no AST construction actions.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * The following categories must be covered by repository conformance tests.
 *
 * POSITIVE:
 *
 *     MTS<int>
 *     MTS<float>
 *     MTS<State>
 *     MTS<QuantumState>
 *     MTS<quantum::State>
 *     MTS<Resource<T>>
 *     MTS<Vec<T>>
 *
 * NEGATIVE:
 *
 *     MTS
 *     MTS<
 *     MTS>
 *     MTS<>
 *
 * and malformed nested type expressions.
 *
 * BOUNDARY:
 *
 *     deeply nested valid source-level temporal types;
 *     symbolic generic arguments;
 *     domain-neutral inner types;
 *     quantum inner types;
 *     resource inner types.
 *
 * SCALABILITY:
 *
 * The grammar must not contain a test that establishes a maximum legal:
 *
 *     temporal nesting depth;
 *     number of temporal types;
 *     timeline count;
 *     snapshot count;
 *     qubit count;
 *     node count;
 *     memory size.
 *
 * Tests may exercise implementation-policy limits separately from language
 * semantics.
 *
 * DETERMINISM:
 *
 * Repeated parsing of identical source must produce equivalent parse
 * structure and diagnostics.
 *
 * COMPATIBILITY:
 *
 * Existing `MTS<T>` source programs must continue to parse through the
 * canonical type-expression path.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file passes the language-level hard-coding requirements because it
 * contains:
 *
 *     - no numeric capacity limit;
 *     - no hardware identifier;
 *     - no physical resource identifier;
 *     - no fixed timeline count;
 *     - no fixed timestamp representation;
 *     - no fixed storage size;
 *     - no fixed clock width;
 *     - no vendor name;
 *     - no backend name;
 *     - no quantum hardware assumption.
 *
 * The only reserved constructor is the already-existing semantic token:
 *
 *     MTS
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is COMPLETE when:
 *
 *   [x] It has one clear ownership boundary.
 *   [x] It consumes the canonical lexer.
 *   [x] It introduces no lexer rules.
 *   [x] It does not duplicate typeExpression.
 *   [x] It avoids circular grammar dependencies.
 *   [x] It preserves the established MTS<T> syntax.
 *   [x] It maps to TypeExpr::Temporal through the frontend adapter.
 *   [x] It does not create a second temporal AST.
 *   [x] It does not create a temporal IR.
 *   [x] It does not select hardware.
 *   [x] It does not impose resource limits.
 *   [x] It is deterministic.
 *   [x] It contains no embedded unsafe Rust.
 *   [x] It is compatible with Rust 1.97/1.97.1 generated-parser integration.
 *   [x] Its downstream integration points are specified in advance.
 *   [x] Its test contract is specified.
 *
 * The surrounding composition grammars still have to consume this component,
 * but this file itself does not require later modification merely because
 * those consumers evolve.
 *
 * ============================================================================
 * PUBLIC RULE
 * ============================================================================
 */

parser grammar TemporalTypes;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * TEMPORAL TYPE CONSTRUCTOR
 * ============================================================================
 *
 * This is intentionally only the constructor.
 *
 * The surrounding canonical type grammar supplies the operand:
 *
 *     temporalType
 *         : temporalTypeConstructor
 *           LT
 *           typeExpression
 *           GT
 *         ;
 *
 * This rule therefore remains independently compilable and avoids a circular
 * dependency between TemporalTypes and Types.
 *
 * ============================================================================
 */
temporalTypeConstructor
    : MTS
    ;
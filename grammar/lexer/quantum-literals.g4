 /*
  * ============================================================================
  * Zamani Programming Language
  * ============================================================================
  *
  * File:
  *     grammar/lexer/quantum-literals.g4
  *
  * Grammar:
  *     ZamaniQuantumLiterals
  *
  * Role:
  *     Canonical lexical owner for source-level quantum literal notation.
  *
  * Lexer:
  *     ANTLR
  *
  * Compiler baseline:
  *     Rust 1.97 / Rust 1.97.1
  *
  * Safety:
  *     This grammar contains no target-language actions.
  *     It requires no Rust `unsafe`.
  *     The Zamani compiler implementation MUST use safe Rust only.
  *
  * ============================================================================
  *
  * ARCHITECTURAL CONTRACT
  * ============================================================================
  *
  * This grammar defines lexical forms that represent quantum values or
  * quantum-literal notation at the SOURCE LANGUAGE boundary.
  *
  * It does NOT define the quantum semantic model.
  *
  * It does NOT define:
  *
  *     - quantum gates;
  *     - gate sets;
  *     - QubitId;
  *     - PhysicalQubitId;
  *     - logical-qubit allocation;
  *     - physical-qubit allocation;
  *     - quantum topology;
  *     - QPU architecture;
  *     - simulator architecture;
  *     - state-vector storage;
  *     - tensor-network storage;
  *     - amplitudes' machine precision;
  *     - maximum state dimension;
  *     - maximum number of qubits;
  *     - measurement hardware;
  *     - error correction;
  *     - noise;
  *     - calibration;
  *     - scheduling;
  *     - routing;
  *     - optimization;
  *     - runtime execution;
  *     - canonical quantum IR.
  *
  * Those concerns belong to later semantic and compilation layers.
  *
  * ============================================================================
  *
  * POCO-REAF PRINCIPLE
  * ============================================================================
  *
  * Source quantum literals describe language semantics.
  *
  * They MUST NOT encode accidental properties of the machine on which the
  * program eventually executes.
  *
  * Therefore this grammar contains no:
  *
  *     MAX_QUBITS
  *     MAX_STATE_DIMENSION
  *     MAX_AMPLITUDES
  *     MAX_REGISTER_SIZE
  *     MAX_QUBIT_INDEX
  *     MAX_PRECISION
  *     MAX_VECTOR_WIDTH
  *     DEVICE_ID
  *     QPU_ID
  *     TOPOLOGY_ID
  *
  * or equivalent hidden restrictions.
  *
  * ============================================================================
  *
  * QUANTUM LITERAL PHILOSOPHY
  * ============================================================================
  *
  * A quantum literal is a SOURCE representation.
  *
  * It is not a promise about how the value is physically represented.
  *
  * For example:
  *
  *     |0⟩
  *
  * denotes a source-level basis-state literal.
  *
  * It MUST NOT imply:
  *
  *     - one specific physical qubit;
  *     - one particular simulator;
  *     - a fixed state-vector representation;
  *     - a fixed floating-point format;
  *     - a fixed Hilbert-space storage strategy;
  *     - a particular QPU.
  *
  * ============================================================================
  *
  * OWNERSHIP
  * ============================================================================
  *
  * OWNS:
  *
  *     - canonical lexical syntax for compact quantum-state literals;
  *     - tokenization of those literals;
  *     - lexical separation of quantum literal notation from ordinary tokens.
  *
  * DOES NOT OWN:
  *
  *     - quantum operations;
  *     - gate names;
  *     - circuits;
  *     - qubit declarations;
  *     - quantum types;
  *     - measurement semantics;
  *     - observables;
  *     - dynamic circuits;
  *     - QEC semantics;
  *     - ZQN semantics;
  *     - hardware capabilities;
  *     - target constraints.
  *
  * ============================================================================
  *
  * WHY THIS IS NOT A GATE GRAMMAR
  * ============================================================================
  *
  * Quantum operations such as:
  *
  *     H
  *     X
  *     Y
  *     Z
  *     CNOT
  *     CZ
  *     SWAP
  *     U
  *     custom_operation
  *     vendor_operation
  *
  * MUST NOT be enumerated here.
  *
  * Operation identity belongs to the quantum syntax/semantic layer.
  *
  * This preserves extensibility for:
  *
  *     standard operations
  *     user-defined operations
  *     parameterized operations
  *     logical operations
  *     calibrated operations
  *     future operations
  *     dialect operations
  *     backend-specific lowering
  *
  * without requiring lexical changes every time a new operation is introduced.
  *
  * ============================================================================
  *
  * WHY THIS IS NOT A QUBIT GRAMMAR
  * ============================================================================
  *
  * Constructs such as:
  *
  *     q[0]
  *     q[1]
  *
  * are NOT quantum literals.
  *
  * They are references to quantum resources and belong to the quantum
  * expression/reference grammar.
  *
  * Consequently this file does not impose:
  *
  *     q[0]
  *     q[1]
  *     q[31]
  *
  * or any other fixed index boundary.
  *
  * ============================================================================
  *
  * CANONICAL QUANTUM IR BOUNDARY
  * ============================================================================
  *
  * This grammar stops at lexical representation.
  *
  * The intended pipeline is:
  *
  *     source
  *       |
  *       v
  *     ZamaniQuantumLiterals
  *       |
  *       v
  *     ZamaniLexer
  *       |
  *       v
  *     parser
  *       |
  *       v
  *     frontend AST
  *       |
  *       v
  *     semantic analysis
  *       |
  *       v
  *     quantum::ir
  *       |
  *       +--> optimization
  *       +--> routing
  *       +--> scheduling
  *       +--> QEC integration
  *       +--> ZQN/fault semantics
  *       +--> hardware realization
  *       +--> runtime
  *
  * The grammar MUST NEVER become a second quantum IR.
  *
  * ============================================================================
  *
  * LITERAL MODEL
  * ============================================================================
  *
  * The canonical compact quantum-state literal syntax supported here is:
  *
  *     |0⟩
  *     |1⟩
  *     |+⟩
  *     |-⟩
  *
  * These are intentionally limited to the compact basis/superposition
  * notation already established by the existing Zamani lexical architecture.
  *
  * More general quantum states MUST NOT be forced into a giant lexer rule.
  *
  * For example, expressions representing arbitrary amplitudes should be
  * composed from normal Zamani expressions and quantum semantic constructs,
  * rather than creating an unbounded specialized lexical syntax.
  *
  * ============================================================================
  *
  * GENERAL STATE EXPRESSIONS
  * ============================================================================
  *
  * The following are deliberately NOT QUANTUM_LITERAL tokens:
  *
  *     |alpha⟩
  *     |psi⟩
  *     |state⟩
  *     |a,b⟩
  *     |ψ⟩
  *
  * unless a future language specification explicitly establishes a lexical
  * notation for them.
  *
  * This avoids turning identifiers, symbolic expressions, and semantic state
  * descriptions into an ever-growing lexer vocabulary.
  *
  * ============================================================================
  *
  * BRA/KET DESIGN
  * ============================================================================
  *
  * The compact ket forms:
  *
  *     |0⟩
  *     |1⟩
  *     |+⟩
  *     |-⟩
  *
  * are lexical literals.
  *
  * A general bra:
  *
  *     ⟨0|
  *     ⟨1|
  *
  * is NOT automatically treated as a separate literal family by this file.
  *
  * General bra/ket algebra belongs to the quantum expression/semantic layer.
  *
  * This distinction prevents the lexer from becoming an implementation of
  * quantum linear algebra.
  *
  * ============================================================================
  *
  * UNICODE
  * ============================================================================
  *
  * The ket closing character:
  *
  *     ⟩
  *
  * is part of the canonical lexical notation.
  *
  * The lexer MUST treat the exact Unicode scalar sequence as source syntax.
  *
  * Unicode normalization MUST NOT be silently performed here.
  *
  * Identifier normalization, if adopted by Zamani, belongs to the canonical
  * identifier/name-resolution specification.
  *
  * ============================================================================
  *
  * CHARACTER SET
  * ============================================================================
  *
  * The supported compact quantum-state symbols are intentionally explicit:
  *
  *     0
  *     1
  *     +
  *     -
  *
  * This does not limit the number of qubits in a program.
  *
  * It only defines the finite vocabulary of the four compact state symbols.
  *
  * ============================================================================
  *
  * NO MACHINE LIMITS
  * ============================================================================
  *
  * A source program can contain an arbitrary number of quantum literals,
  * subject only to limits imposed by the compiler/runtime/resource environment
  * rather than this grammar.
  *
  * This grammar does not define:
  *
  *     MAX_LITERALS
  *     MAX_QUBITS
  *     MAX_CIRCUITS
  *     MAX_STATE_SIZE
  *     MAX_PROGRAM_SIZE
  *
  * ============================================================================
  *
  * LEXICAL ERROR BEHAVIOR
  * ============================================================================
  *
  * A malformed compact ket must not be silently converted into another
  * quantum literal.
  *
  * Examples:
  *
  *     |2⟩
  *     |00⟩
  *     |01⟩
  *     |a⟩
  *     |0>
  *     |0
  *
  * are NOT accepted by QUANTUM_LITERAL.
  *
  * Their eventual classification is determined by the complete lexer and
  * parser architecture.
  *
  * Semantic validation MUST NOT be hidden inside this lexer rule.
  *
  * ============================================================================
  *
  * TOKEN PRIORITY
  * ============================================================================
  *
  * QUANTUM_LITERAL must be integrated into the canonical lexer without
  * creating ambiguous ownership with:
  *
  *     INTEGER
  *     IDENTIFIER
  *     PIPE
  *     PLUS
  *     MINUS
  *     punctuation
  *
  * The canonical lexer assembly is responsible for the final token ordering
  * and vocabulary integration.
  *
  * ============================================================================
  *
  * IMPORTANT ANTLR ARCHITECTURAL NOTE
  * ============================================================================
  *
  * This file is a specialized lexer grammar and is aggregated by:
  *
  *     grammar/lexer/literals.g4
  *
  * through:
  *
  *     ZamaniQuantumLiterals
  *
  * The canonical public lexer remains the repository's canonical lexer.
  *
  * Parser grammars MUST NOT directly depend on this specialized lexer.
  *
  * ============================================================================
  *
  * NO DUPLICATE QUANTUM_LITERAL
  * ============================================================================
  *
  * Once this grammar is integrated into the canonical lexer architecture,
  * other lexer grammars MUST NOT independently define:
  *
  *     QUANTUM_LITERAL
  *
  * This includes legacy definitions in:
  *
  *     grammar/antlr/ZamaniLexer.g4
  *     grammar/antlr/Core.g4
  *     grammar/Zamani.g4
  *
  * Those definitions must be reconciled with the canonical lexer authority.
  *
  * ============================================================================
  *
  * QUANTUM SEMANTICS ARE OUTSIDE THIS FILE
  * ============================================================================
  *
  * The following are intentionally absent:
  *
  *     gate semantics
  *     unitary validation
  *     state normalization
  *     amplitude validation
  *     entanglement semantics
  *     measurement semantics
  *     observable semantics
  *     QEC semantics
  *     noise semantics
  *     hardware constraints
  *
  * This keeps lexical analysis deterministic and target-independent.
  *
  * ============================================================================
  *
  * QEC INTEGRATION
  * ============================================================================
  *
  * This file does not define error-corrected state literals.
  *
  * QEC owns:
  *
  *     detection
  *     correction
  *     code definitions
  *     syndrome semantics
  *     logical/physical error relationships
  *
  * A quantum literal may eventually enter a QEC-aware compilation pipeline,
  * but that does not change its lexical meaning.
  *
  * ============================================================================
  *
  * ZQN INTEGRATION
  * ============================================================================
  *
  * ZQN owns quantum noise/fault semantics.
  *
  * This file does not encode:
  *
  *     noise probabilities
  *     fault locations
  *     correlated faults
  *     leakage
  *     loss
  *     erasure
  *     calibration errors
  *
  * A literal remains independent of the noise model used by a target.
  *
  * ============================================================================
  *
  * HARDWARE INTEGRATION
  * ============================================================================
  *
  * Hardware capabilities are discovered and represented by the hardware
  * abstraction/resource layers.
  *
  * This file MUST NOT encode:
  *
  *     QPU names
  *     processor generations
  *     topology
  *     coupling maps
  *     native gates
  *     physical qubit IDs
  *     calibration values
  *     device addresses.
  *
  * ============================================================================
  *
  * SCHEDULING INTEGRATION
  * ============================================================================
  *
  * Quantum literals contain no scheduling information.
  *
  * Duration, timing, alignment, delays, ordering, and resource occupancy
  * belong to the scheduling/temporal layers.
  *
  * ============================================================================
  *
  * OPTIMIZATION INTEGRATION
  * ============================================================================
  *
  * The optimizer may transform the semantic representation derived from a
  * quantum literal.
  *
  * The optimizer MUST NOT require this lexical grammar to know the target
  * machine.
  *
  * ============================================================================
  *
  * AST CONTRACT
  * ============================================================================
  *
  * This file produces tokens only.
  *
  * The frontend parser/AST layer may map:
  *
  *     |0⟩
  *
  * to a canonical quantum-state literal AST node.
  *
  * The exact AST type/name is owned by the frontend AST architecture.
  *
  * This grammar MUST NOT define Rust structs or duplicate AST definitions.
  *
  * ============================================================================
  *
  * SEMANTIC CONTRACT
  * ============================================================================
  *
  * Semantic analysis determines the meaning of the parsed literal.
  *
  * For example:
  *
  *     |0⟩
  *
  * may semantically represent the canonical zero computational-basis state.
  *
  * This file does not determine:
  *
  *     - storage;
  *     - allocation;
  *     - target;
  *     - precision;
  *     - execution strategy.
  *
  * ============================================================================
  *
  * IR CONTRACT
  * ============================================================================
  *
  * No IR is produced here.
  *
  * The frontend's semantic lowering is responsible for mapping the AST into
  * the repository's canonical IR.
  *
  * Quantum semantics MUST cross the established:
  *
  *     quantum::ir
  *
  * boundary.
  *
  * This grammar MUST NOT introduce a second quantum representation.
  *
  * ============================================================================
  *
  * DETERMINISM
  * ============================================================================
  *
  * For a fixed character stream, lexer configuration, and grammar version:
  *
  *     |0⟩ -> QUANTUM_LITERAL
  *     |1⟩ -> QUANTUM_LITERAL
  *     |+⟩ -> QUANTUM_LITERAL
  *     |-⟩ -> QUANTUM_LITERAL
  *
  * deterministically.
  *
  * No runtime state or hardware discovery participates in lexical recognition.
  *
  * ============================================================================
  *
  * VERSIONING
  * ============================================================================
  *
  * The literal spellings are part of the language's lexical compatibility
  * surface.
  *
  * Changing them requires coordinated language-versioning changes.
  *
  * A new quantum notation should be introduced through an explicit language
  * evolution process rather than silently changing this rule.
  *
  * ============================================================================
  *
  * TEST CONTRACT
  * ============================================================================
  *
  * Positive lexical cases:
  *
  *     |0⟩
  *     |1⟩
  *     |+⟩
  *     |-⟩
  *
  * Negative lexical cases:
  *
  *     |2⟩
  *     |a⟩
  *     |00⟩
  *     |01⟩
  *     |0>
  *     |0
  *     0⟩
  *
  * Boundary cases:
  *
  *     multiple adjacent literals;
  *     literals separated by whitespace;
  *     literals inside expressions;
  *     literals inside quantum declarations;
  *     literals inside classical/quantum control expressions.
  *
  * Integration cases:
  *
  *     classical + quantum;
  *     quantum + measurement;
  *     quantum + classical condition;
  *     quantum + dynamic circuit;
  *     quantum + QEC;
  *     quantum + resource requirements.
  *
  * These integration tests verify semantic interoperability, not ownership
  * leakage into this lexer.
  *
  * ============================================================================
  *
  * ROUND-TRIP CONTRACT
  * ============================================================================
  *
  * Where a canonical source printer exists:
  *
  *     source
  *       ->
  *     lexer
  *       ->
  *     parser
  *       ->
  *     AST
  *       ->
  *     printer
  *       ->
  *     parser
  *
  * must preserve the intended quantum-literal semantics.
  *
  * ============================================================================
  *
  * SCALABILITY AUDIT
  * ============================================================================
  *
  * This grammar contains no resource-dependent finite bounds.
  *
  * The finite list:
  *
  *     0
  *     1
  *     +
  *     -
  *
  * is a lexical vocabulary, not a machine-size limitation.
  *
  * Therefore:
  *
  *     one literal
  *     many literals
  *     many circuits
  *     many modules
  *     many quantum resources
  *
  * remain governed by the surrounding compiler/runtime resources rather than
  * by an arbitrary quantum literal ceiling.
  *
  * ============================================================================
  *
  * COMPLETION CRITERIA
  * ============================================================================
  *
  * This file is complete when:
  *
  *     [x] It has one clear lexical owner.
  *     [x] It defines only source-level quantum literal syntax.
  *     [x] It does not define quantum operations.
  *     [x] It does not define qubit references.
  *     [x] It does not define quantum types.
  *     [x] It does not define quantum IR.
  *     [x] It does not define QEC.
  *     [x] It does not define ZQN.
  *     [x] It does not define hardware.
  *     [x] It does not define scheduling.
  *     [x] It does not define optimization.
  *     [x] It imposes no machine-size limits.
  *     [x] It contains no target-specific identifiers.
  *     [x] It uses no Rust actions.
  *     [x] It requires no unsafe Rust.
  *     [x] It composes through literals.g4.
  *     [x] It has deterministic lexical behavior.
  *     [x] It has explicit negative/error boundaries.
  *     [x] It has a predefined AST/semantic/IR integration contract.
  *
  * ============================================================================
  */

lexer grammar ZamaniQuantumLiterals;


/*
 * ============================================================================
 * COMPACT QUANTUM-STATE LITERALS
 * ============================================================================
 *
 * Canonical forms:
 *
 *     |0⟩
 *     |1⟩
 *     |+⟩
 *     |-⟩
 *
 * The literal is deliberately kept as one lexical token.
 *
 * Its semantic interpretation is performed after parsing.
 *
 * No physical machine assumption is encoded.
 * ============================================================================
 */

QUANTUM_LITERAL
    : '|'
      QUANTUM_STATE_SYMBOL
      '⟩'
    ;


/*
 * ============================================================================
 * COMPACT STATE SYMBOLS
 * ============================================================================
 *
 * These symbols are the deliberately finite lexical vocabulary of the compact
 * quantum-state literal notation.
 *
 * They are NOT a limit on:
 *
 *     - qubit count;
 *     - register count;
 *     - circuit size;
 *     - state dimension;
 *     - program size.
 * ============================================================================
 */

fragment QUANTUM_STATE_SYMBOL
    : '0'
    | '1'
    | '+'
    | '-'
    ;
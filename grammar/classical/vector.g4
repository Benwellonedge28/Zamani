/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/classical/vector.g4
 *
 * Grammar:
 *     Vector
 *
 * Status:
 *     CANONICAL CLASSICAL VECTOR VALUE GRAMMAR
 *
 * Purpose:
 *     Define target-independent source syntax for classical vector values,
 *     vector literals, vector comprehensions, vector indexing/slicing
 *     integration boundaries, and vector-domain classification.
 *
 * ============================================================================
 * IMPLEMENTATION BASELINE
 * ============================================================================
 *
 * Rust:
 *     1.97 / 1.97.1
 *
 * Edition:
 *     2021
 *
 * Safety:
 *     Safe Rust only.
 *
 * This grammar contains:
 *
 *     - no embedded Rust;
 *     - no semantic predicates;
 *     - no target-language actions;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware discovery;
 *     - no runtime execution;
 *     - no unsafe Rust requirement.
 *
 * ============================================================================
 * ARCHITECTURAL ROLE
 * ============================================================================
 *
 * This file owns SOURCE-LEVEL VECTOR VALUE SYNTAX.
 *
 * It does NOT own:
 *
 *     - lexical tokens;
 *     - identifiers;
 *     - general expression precedence;
 *     - general calls;
 *     - general indexing semantics;
 *     - type syntax;
 *     - declarations;
 *     - statements;
 *     - vector algorithms;
 *     - vector storage;
 *     - memory allocation;
 *     - SIMD realization;
 *     - GPU realization;
 *     - FPGA realization;
 *     - accelerator selection;
 *     - distributed placement;
 *     - scheduling;
 *     - routing;
 *     - optimization;
 *     - classical IR;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime execution.
 *
 * ============================================================================
 * AUTHORITY AND INTEGRATION
 * ============================================================================
 *
 * Lexical authority:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Canonical lexical composition:
 *
 *     grammar/lexer/tokens.g4
 *
 * General expression authority:
 *
 *     grammar/expressions/expressions.g4
 *
 * Classical scalar value authority:
 *
 *     grammar/classical/scalar.g4
 *
 * Classical type authority:
 *
 *     grammar/types/classical.g4
 *
 * Classical domain composition:
 *
 *     grammar/classical/classical.g4
 *
 * Universal parser composition:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * Universal ANTLR root:
 *
 *     grammar/Zamani.g4
 *
 * Frontend semantic representation:
 *
 *     domain-neutral frontend AST
 *
 * Canonical quantum semantic boundary:
 *
 *     quantum::ir
 *
 * Vector semantics therefore follow:
 *
 *     source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       v
 *     ZamaniParser
 *       |
 *       v
 *     Vector
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic analysis
 *       |
 *       +----------------------+----------------------+
 *       |                      |                      |
 *       v                      v                      v
 * classical semantics   resource/capability     cross-domain semantics
 *       |                      |                      |
 *       +----------------------+----------------------+
 *                              |
 *                              v
 *                       canonical semantic model
 *                              |
 *                              v
 *                         Classical IR
 *                              |
 *                              v
 *                    optimization / lowering
 *                              |
 *                              v
 *                  scheduling / placement / runtime
 *                              |
 *                              v
 *                       target realization
 *
 * ============================================================================
 * CRITICAL DESIGN RULE
 * ============================================================================
 *
 * Vector syntax describes PORTABLE SEMANTIC DATA.
 *
 * It must never become a description of a particular vector processor.
 *
 * Therefore this grammar contains no universal limits for:
 *
 *     vector length
 *     element count
 *     SIMD lanes
 *     vector register width
 *     GPU lanes
 *     GPU threads
 *     FPGA pipeline width
 *     accelerator width
 *     memory capacity
 *     register count
 *     node count
 *     device count
 *
 * A vector of semantic length N is not equivalent to:
 *
 *     N registers
 *     N SIMD lanes
 *     N GPU threads
 *     N devices
 *
 * Those are downstream implementation decisions.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Zamani source describes:
 *
 *     WHAT the vector means.
 *
 * It does not prescribe:
 *
 *     WHERE it is stored;
 *     HOW it is partitioned;
 *     WHICH processor evaluates it;
 *     WHICH accelerator evaluates it;
 *     WHICH device owns it;
 *     WHICH physical memory bank stores it;
 *     WHICH SIMD width realizes it.
 *
 * Consequently the same source may be lowered to:
 *
 *     scalar iteration
 *     contiguous storage
 *     segmented storage
 *     SIMD/vector instructions
 *     GPU execution
 *     FPGA pipelines
 *     accelerator execution
 *     streaming execution
 *     distributed execution
 *     future execution models
 *
 * subject to semantic correctness and available resources.
 *
 * ============================================================================
 * OPEN-WORLD VECTOR MODEL
 * ============================================================================
 *
 * Vector element types and vector operations are intentionally OPEN WORLD.
 *
 * The grammar must not contain an exhaustive list such as:
 *
 *     vectorAdd
 *     vectorSub
 *     vectorMul
 *     vectorFFT
 *     vectorSVD
 *     vectorNormalize
 *     ...
 *
 * Those are operations/libraries/semantic capabilities rather than necessarily
 * new language syntax.
 *
 * Likewise this grammar does not reserve:
 *
 *     vector
 *     repeat
 *     concat
 *     generate
 *
 * as lexer keywords.
 *
 * A call such as:
 *
 *     vector(x, n)
 *
 * remains ordinary Zamani expression syntax and is resolved semantically.
 *
 * This prevents library evolution from requiring lexer changes.
 *
 * ============================================================================
 * VECTOR VALUE SYNTAX
 * ============================================================================
 *
 * This grammar owns syntax that is genuinely vector-shaped:
 *
 *     [a, b, c]
 *
 *     [x * x for x in values]
 *
 *     [x * x for x in values if x > 0]
 *
 * It does not claim ownership of generic function-call syntax.
 *
 * Therefore:
 *
 *     vector(x, n)
 *
 * is an ordinary expression call.
 *
 * Semantic analysis may recognize it as a vector constructor, but this grammar
 * does not manufacture a second call syntax merely for that purpose.
 *
 * ============================================================================
 * EMPTY VECTORS
 * ============================================================================
 *
 * The following is valid syntax:
 *
 *     []
 *
 * Its semantic validity is determined downstream.
 *
 * An empty vector does not by itself determine its element type.
 *
 * Semantic analysis may obtain that type from:
 *
 *     contextual typing;
 *     explicit type annotation;
 *     generic inference;
 *     expected type;
 *     construction context;
 *     later constraints.
 *
 * ============================================================================
 * TRAILING COMMA
 * ============================================================================
 *
 * Vector literals permit:
 *
 *     [a,]
 *     [a, b,]
 *
 * in addition to:
 *
 *     [a]
 *     [a, b]
 *
 * This is a syntax convenience only.
 *
 * It does not change vector semantics.
 *
 * ============================================================================
 * VECTOR COMPREHENSIONS
 * ============================================================================
 *
 * Vector comprehensions are source-level constructs:
 *
 *     [expression for identifier in expression]
 *
 * and:
 *
 *     [expression for identifier in expression if expression]
 *
 * They do NOT imply a particular execution model.
 *
 * They may later be lowered to:
 *
 *     scalar iteration;
 *     parallel iteration;
 *     SIMD;
 *     GPU execution;
 *     accelerator execution;
 *     distributed execution;
 *     streaming;
 *     another implementation.
 *
 * ============================================================================
 * COMPREHENSION BINDING
 * ============================================================================
 *
 * The binding variable is parsed using the canonical identifier rule supplied
 * by Expressions.
 *
 * This file deliberately DOES NOT redeclare:
 *
 *     identifier
 *
 * because doing so would create a second identifier authority.
 *
 * ============================================================================
 * COMPREHENSION SOURCE
 * ============================================================================
 *
 * The source after `in` is an ordinary expression.
 *
 * Therefore it may denote:
 *
 *     vector
 *     range
 *     collection
 *     stream
 *     iterator
 *     lazy sequence
 *     distributed sequence
 *     device-resident sequence
 *     user-defined iterable
 *     future sequence abstraction
 *
 * Semantic analysis determines whether the expression is iterable and what
 * iteration semantics apply.
 *
 * ============================================================================
 * FILTER SEMANTICS
 * ============================================================================
 *
 * The optional `if` expression is ordinary expression syntax.
 *
 * The parser does not decide whether filtering is:
 *
 *     eager;
 *     lazy;
 *     vectorized;
 *     parallel;
 *     distributed;
 *     fused;
 *     streamed.
 *
 * Those are downstream decisions.
 *
 * ============================================================================
 * RANGE / SLICE INTEGRATION
 * ============================================================================
 *
 * The general expression grammar already owns range operators:
 *
 *     DOT_DOT
 *     DOT_DOT_EQ
 *
 * Vector slicing therefore uses a vector-domain wrapper around range-capable
 * selectors without redefining the range precedence hierarchy.
 *
 * Supported vector slice forms include:
 *
 *     v[start..end]
 *     v[start..=end]
 *     v[..end]
 *     v[start..]
 *     v[..]
 *
 * Slice semantics are NOT determined here.
 *
 * ============================================================================
 * MULTI-DIMENSIONAL INDEXING
 * ============================================================================
 *
 * This file does not create a separate multidimensional indexing language.
 *
 * A vector-domain index selector may contain one or more expressions:
 *
 *     v[i]
 *     v[i, j]
 *     v[i, j, k]
 *
 * The semantic layer determines whether the referenced value is:
 *
 *     vector;
 *     matrix;
 *     tensor;
 *     multidimensional collection;
 *     user-defined indexed value.
 *
 * No rank limit is imposed.
 *
 * ============================================================================
 * TYPE BOUNDARY
 * ============================================================================
 *
 * Vector VALUE syntax is distinct from vector TYPE syntax.
 *
 * Vector type syntax remains owned by:
 *
 *     grammar/types/classical.g4
 *
 * Examples:
 *
 *     Vector<T>
 *     Vector<T, N>
 *     Vector<T, shape>
 *
 * are type-level constructs.
 *
 * Examples:
 *
 *     [a, b, c]
 *
 *     [f(x) for x in values]
 *
 * are value-level constructs.
 *
 * This grammar MUST NOT infer or construct a vector type.
 *
 * ============================================================================
 * EXPRESSION BOUNDARY
 * ============================================================================
 *
 * Elements and comprehension expressions use the canonical:
 *
 *     expression
 *
 * rule from Expressions.
 *
 * This grammar therefore does not redefine:
 *
 *     arithmetic;
 *     comparison;
 *     logical operators;
 *     bitwise operators;
 *     calls;
 *     member access;
 *     unary operators;
 *     assignment;
 *     conditional expressions;
 *     range precedence.
 *
 * ============================================================================
 * SCALAR INTEGRATION
 * ============================================================================
 *
 * Scalar values remain ordinary expressions.
 *
 * A vector may contain:
 *
 *     integer values;
 *     floating-point values;
 *     booleans;
 *     characters;
 *     strings;
 *     user-defined scalar values;
 *     symbolic values;
 *     function results;
 *     compile-time values;
 *     quantum-derived classical values;
 *     hardware parameters;
 *     future domain-defined values.
 *
 * Scalar legality is determined by semantic analysis.
 *
 * Scalar.g4 remains the scalar-value classification boundary.
 *
 * ============================================================================
 * CLASSICAL / QUANTUM INTEGRATION
 * ============================================================================
 *
 * A vector may carry classical data produced by quantum computation, for
 * example:
 *
 *     measurement results;
 *     expectation values;
 *     probability data;
 *     parameter sets;
 *     classical feed-forward values.
 *
 * This grammar does not import or depend upon quantum::ir.
 *
 * Quantum semantics remain:
 *
 *     quantum syntax
 *         ->
 *     domain-neutral AST
 *         ->
 *     semantic analysis
 *         ->
 *     quantum::ir
 *
 * The vector grammar only accepts the resulting source-level expressions.
 *
 * ============================================================================
 * CLASSICAL / HDL INTEGRATION
 * ============================================================================
 *
 * Vector syntax may participate in hardware/software co-design through the
 * shared expression and type systems.
 *
 * This file does not define:
 *
 *     ports;
 *     wires;
 *     buses;
 *     registers;
 *     clocks;
 *     physical widths;
 *     memory banks;
 *     placement.
 *
 * HDL and hardware grammars remain authoritative for those concepts.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Vector size may be a semantic requirement.
 *
 * Example:
 *
 *     vector(data, n)
 *
 * means only that the resulting semantic value contains n elements if the
 * surrounding semantics define that call as a vector constructor.
 *
 * It does NOT mean:
 *
 *     allocate n registers;
 *     allocate n SIMD lanes;
 *     allocate n GPU threads;
 *     allocate n devices.
 *
 * Resource analysis is downstream.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar lowers to the existing domain-neutral frontend AST.
 *
 * It must not create a Vector-specific Rust AST solely because the grammar is
 * located in the classical/vector domain.
 *
 * Existing generic AST concepts include:
 *
 *     Expression::Array
 *     Expression::Identifier
 *     Expression::Call
 *     Expression::Index
 *     Expression::Range
 *     Expression::MemberAccess
 *
 * and related generic constructs.
 *
 * A vector literal may therefore be represented using the existing generic
 * array/sequence expression representation, with semantic analysis deciding
 * that it is a vector.
 *
 * A vector comprehension requires the frontend's canonical comprehension
 * representation. If the current AST does not yet expose one, the parser
 * integration layer must map the construct into the repository's canonical
 * generic collection/comprehension representation rather than creating a
 * competing vector-only AST.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis determines:
 *
 *     - element type;
 *     - homogeneity;
 *     - conversions;
 *     - vector length;
 *     - symbolic length;
 *     - runtime length;
 *     - shape;
 *     - mutability;
 *     - ownership;
 *     - borrowing;
 *     - effects;
 *     - iteration legality;
 *     - index legality;
 *     - slice legality;
 *     - resource requirements;
 *     - capability requirements;
 *     - target-independent vector semantics.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar constructs NO IR.
 *
 * After semantic analysis, vector semantics may lower to the repository's
 * canonical classical/data representation and subsequently to Classical IR.
 *
 * There is no vector-specific IR introduced here.
 *
 * Quantum-derived values continue through the existing quantum semantic
 * boundary when their semantics are quantum.
 *
 * ============================================================================
 * COMPILER CONTRACT
 * ============================================================================
 *
 * The compiler may realize the same source-level vector through:
 *
 *     scalar execution;
 *     loops;
 *     SIMD;
 *     GPU;
 *     FPGA;
 *     accelerator;
 *     streaming;
 *     distributed execution;
 *     memory-resident execution;
 *     lazy execution;
 *     future targets.
 *
 * Vector.g4 must remain unchanged when a new backend is introduced unless the
 * new backend introduces genuinely new SOURCE LANGUAGE semantics.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime representation is not specified by this grammar.
 *
 * Runtime may select representations according to:
 *
 *     compiled semantics;
 *     ownership;
 *     lifetime;
 *     memory availability;
 *     target capability;
 *     execution policy;
 *     resource negotiation.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     source token sequence;
 *     selected grammar/language version.
 *
 * Parsing does not depend on:
 *
 *     hardware;
 *     runtime state;
 *     resource availability;
 *     filesystem state;
 *     network state;
 *     wall-clock time;
 *     randomness;
 *     backend availability.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden in this grammar:
 *
 *     MAX_VECTOR_LENGTH
 *     MAX_ELEMENTS
 *     MAX_RANK
 *     MAX_DIMENSIONS
 *     MAX_LANES
 *     MAX_SIMD_WIDTH
 *     MAX_THREADS
 *     MAX_CORES
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_ACCELERATORS
 *     MAX_MEMORY
 *     MAX_REGISTERS
 *     fixed physical device identifiers
 *     fixed hardware addresses
 *     fixed topology
 *
 * No finite vector-size maximum is encoded.
 *
 * Structural repetition uses ANTLR repetition constructs:
 *
 *     *
 *     ?
 *     +
 *
 * rather than finite enumeration.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing canonical tokens are preserved.
 *
 * This grammar consumes:
 *
 *     ZamaniLexer
 *
 * and does not define replacement tokens.
 *
 * No new token is required for vector syntax.
 *
 * Existing stable token names such as:
 *
 *     IDENTIFIER
 *     LBRACKET
 *     RBRACKET
 *     COMMA
 *     DOT_DOT
 *     DOT_DOT_EQ
 *     FOR
 *     IN
 *     IF
 *
 * remain owned by the canonical lexer.
 *
 * If a token spelling changes in the future, the compatibility process must
 * update the canonical lexical layer rather than creating a local alias here.
 *
 * ============================================================================
 * VALIDATION CONTRACT
 * ============================================================================
 *
 * grammar/validation/ must verify:
 *
 *     - this file is parser-only;
 *     - tokenVocab is ZamaniLexer;
 *     - no lexer rules occur here;
 *     - no identifier rule is duplicated;
 *     - no vector type grammar is duplicated;
 *     - no expression precedence is duplicated;
 *     - no fixed vector limit exists;
 *     - no hardware topology is encoded;
 *     - no backend-specific syntax is required;
 *     - comprehension syntax uses canonical FOR/IN/IF tokens;
 *     - range syntax uses canonical DOT_DOT/DOT_DOT_EQ tokens;
 *     - the grammar remains deterministic;
 *     - imported grammar dependencies resolve;
 *     - all public rules have AST/semantic integration coverage.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE:
 *
 *     []
 *     [1]
 *     [1, 2]
 *     [x, y, z]
 *     [x + y, f(z)]
 *     [x,]
 *     [x, y,]
 *     [x * x for x in values]
 *     [x * x for x in values if x > 0]
 *     v[i]
 *     v[i, j]
 *     v[start..end]
 *     v[start..=end]
 *     v[..end]
 *     v[start..]
 *     v[..]
 *
 * VECTOR-CONSTRUCTOR INTEGRATION:
 *
 *     vector(x)
 *     vector(x, n)
 *     vector(value, symbolic_size)
 *
 * These are parsed by the canonical expression-call grammar, not by a
 * vector-specific call grammar.
 *
 * VECTOR OPERATION INTEGRATION:
 *
 *     repeat(x, n)
 *     concat(a, b)
 *     concat(a, b, c)
 *     generate(f, n)
 *
 * These are likewise ordinary expression calls whose vector meaning is
 * determined semantically.
 *
 * NEGATIVE:
 *
 *     [,
 *     [,]
 *     [1 2]
 *     [x for in values]
 *     [x for x values]
 *     [x for x in]
 *     [x for x in values if]
 *     v[]
 *     v[1 2]
 *     v[1..2
 *     [1, 2
 *
 * BOUNDARY:
 *
 *     empty vector;
 *     singleton vector;
 *     long element sequence;
 *     deeply nested element expressions;
 *     symbolic vector size;
 *     runtime vector size;
 *     empty slice;
 *     open-ended slice;
 *     multi-dimensional index syntax.
 *
 * SCALABILITY:
 *
 * Verify that the grammar imposes no fixed:
 *
 *     element count;
 *     vector length;
 *     index rank;
 *     comprehension source size;
 *     nesting count;
 *     machine width;
 *     SIMD width;
 *     accelerator count;
 *     memory capacity.
 *
 * CROSS-DOMAIN:
 *
 *     vector + scalar;
 *     vector + classical numerical expression;
 *     vector + quantum-derived classical expression;
 *     vector + data expression;
 *     vector + distributed value;
 *     vector + accelerator-oriented expression;
 *     vector + resource/capability expression.
 *
 * DETERMINISM:
 *
 * Identical source/token streams must produce identical parse structures
 * regardless of:
 *
 *     CPU;
 *     GPU;
 *     FPGA;
 *     QPU;
 *     memory size;
 *     resource availability;
 *     runtime environment.
 *
 * ============================================================================
 * INDEPENDENT COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is independently complete when:
 *
 *     [x] The filename remains grammar/classical/vector.g4.
 *
 *     [x] The grammar name is Vector.
 *
 *     [x] It is parser-only.
 *
 *     [x] It consumes ZamaniLexer.
 *
 *     [x] It imports only the canonical Expressions parser grammar.
 *
 *     [x] It does not duplicate identifier syntax.
 *
 *     [x] It does not duplicate expression precedence.
 *
 *     [x] It does not duplicate vector type syntax.
 *
 *     [x] It does not define vector-specific lexer tokens.
 *
 *     [x] Vector literals support arbitrary syntactic element counts.
 *
 *     [x] Empty vectors are representable.
 *
 *     [x] Trailing commas are supported.
 *
 *     [x] Vector comprehensions are supported.
 *
 *     [x] Filtered comprehensions are supported.
 *
 *     [x] Slice syntax supports closed and open ranges.
 *
 *     [x] Multi-dimensional index selectors are representable.
 *
 *     [x] Generic vector constructors remain ordinary expressions.
 *
 *     [x] Generic vector operations remain ordinary expressions.
 *
 *     [x] No hardware limit is encoded.
 *
 *     [x] No resource limit is encoded.
 *
 *     [x] No target is selected.
 *
 *     [x] No IR is created.
 *
 *     [x] No quantum IR is duplicated.
 *
 *     [x] No QEC/ZQN/HAL implementation is introduced.
 *
 *     [x] AST ownership is explicitly downstream.
 *
 *     [x] Semantic ownership is explicitly downstream.
 *
 *     [x] Compiler/runtime ownership is explicitly downstream.
 *
 *     [x] Compatibility ownership is explicitly defined.
 *
 *     [x] Positive/negative/boundary/scalability/determinism/cross-domain
 *         tests are specified.
 *
 * ============================================================================
 * REQUIRED EXTERNAL INTEGRATION
 * ============================================================================
 *
 * This file is independently complete as a vector parser grammar.
 *
 * The repository composition must connect it through:
 *
 *     grammar/classical/classical.g4
 *          |
 *          +--> import Vector
 *          |
 *          +--> classical vector-domain dispatch
 *
 * and then:
 *
 *     grammar/antlr/ZamaniParser.g4
 *          |
 *          +--> Classical
 *
 * No other file needs to modify the vector grammar to add a newly introduced
 * backend, device, accelerator, SIMD width, memory size, or hardware topology.
 *
 * The semantic/frontend implementation must map:
 *
 *     vectorLiteral
 *     vectorComprehension
 *     vectorFilteredComprehension
 *     vectorAccess
 *     vectorSlice
 *
 * into the existing domain-neutral AST representation.
 *
 * If a generic AST comprehension node is not yet available, that is an AST
 * implementation task; it is NOT a reason to create a competing Vector AST.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL INVARIANT
 * ============================================================================
 *
 * This file answers:
 *
 *     "What source syntax expresses a classical vector value?"
 *
 * It does not answer:
 *
 *     "How is that vector stored?"
 *
 *     "How many hardware lanes exist?"
 *
 *     "Which GPU executes it?"
 *
 *     "Which FPGA implements it?"
 *
 *     "Which accelerator owns it?"
 *
 *     "Which machine executes it?"
 *
 * Those decisions belong downstream.
 *
 * Therefore:
 *
 *     Program Once
 *          ->
 *     Compile Once
 *          ->
 *     Run Everywhere
 *          ->
 *     Run Anywhere
 *          ->
 *     Forever
 *
 * remains compatible with vector semantics, subject to program correctness,
 * implementation capabilities, and actual resources available at realization
 * time.
 *
 * ============================================================================
 */

parser grammar Vector;

options {
    tokenVocab = ZamaniLexer;
}

import Expressions;


/*
 * ============================================================================
 * 1. PUBLIC VECTOR VALUE ENTRY
 * ============================================================================
 *
 * The public vector entry deliberately recognizes only syntax whose shape
 * establishes a vector-domain construct.
 *
 * Ordinary identifiers and ordinary function calls are not claimed to be
 * vectors by the parser. Semantic analysis determines whether a referenced
 * expression has vector type.
 */
vector
    : vectorComprehension
    | vectorFilteredComprehension
    | vectorLiteral
    ;


/*
 * ============================================================================
 * 2. VECTOR LITERAL
 * ============================================================================
 *
 * Examples:
 *
 *     []
 *     [x]
 *     [x, y]
 *     [x, y, z]
 *     [x,]
 *     [x, y,]
 *
 * There is no finite element-count limit.
 */
vectorLiteral
    : LBRACKET vectorElements? RBRACKET
    ;


/*
 * ============================================================================
 * 3. VECTOR ELEMENTS
 * ============================================================================
 *
 * Every element is a canonical Zamani expression.
 *
 * Consequently vector elements may themselves contain arbitrary legal
 * expressions without this grammar duplicating expression precedence.
 */
vectorElements
    : expression
      (COMMA expression)*
      COMMA?
    ;


/*
 * ============================================================================
 * 4. VECTOR COMPREHENSION
 * ============================================================================
 *
 * Example:
 *
 *     [x * x for x in values]
 *
 * The binding identifier is the canonical expression-layer identifier rule.
 *
 * The source expression after `in` is an ordinary expression and may represent
 * any semantically iterable value.
 */
vectorComprehension
    : LBRACKET
      expression
      FOR
      identifier
      IN
      expression
      RBRACKET
    ;


/*
 * ============================================================================
 * 5. FILTERED VECTOR COMPREHENSION
 * ============================================================================
 *
 * Example:
 *
 *     [x * x for x in values if x > 0]
 */
vectorFilteredComprehension
    : LBRACKET
      expression
      FOR
      identifier
      IN
      expression
      IF
      expression
      RBRACKET
    ;


/*
 * ============================================================================
 * 6. VECTOR ACCESS
 * ============================================================================
 *
 * This is a semantic-domain wrapper around canonical expression syntax.
 *
 * Examples:
 *
 *     v[i]
 *     v[i, j]
 *     v[i, j, k]
 *
 * No rank limit is imposed.
 */
vectorAccess
    : expression
      LBRACKET
      vectorIndexSelectors
      RBRACKET
    ;


vectorIndexSelectors
    : expression
      (COMMA expression)*
      COMMA?
    ;


/*
 * ============================================================================
 * 7. VECTOR SLICE
 * ============================================================================
 *
 * Supported forms:
 *
 *     v[start..end]
 *     v[start..=end]
 *     v[..end]
 *     v[start..]
 *     v[..]
 *
 * Slice semantics remain downstream.
 */
vectorSlice
    : expression
      LBRACKET
      vectorSliceSelector
      RBRACKET
    ;


vectorSliceSelector
    : expression DOT_DOT expression
    | expression DOT_DOT_EQ expression
    | DOT_DOT expression
    | DOT_DOT_EQ expression
    | expression DOT_DOT
    | expression DOT_DOT_EQ
    | DOT_DOT
    ;


/*
 * ============================================================================
 * 8. VECTOR INDEX-OR-SLICE DOMAIN EXPRESSION
 * ============================================================================
 *
 * This rule gives semantic consumers one stable vector-domain boundary without
 * creating a second general expression grammar.
 */
vectorIndexedExpression
    : vectorAccess
    | vectorSlice
    ;


/*
 * ============================================================================
 * 9. VECTOR DOMAIN EXPRESSION
 * ============================================================================
 *
 * A vector-domain expression is either an explicitly vector-shaped source
 * construct or a vector indexing/slicing construct.
 *
 * A plain identifier is intentionally NOT included here.
 *
 * Whether:
 *
 *     v
 *
 * is a vector is determined by name/type resolution in semantic analysis.
 *
 * Likewise:
 *
 *     vector(x, n)
 *     repeat(x, n)
 *     concat(a, b)
 *     generate(f, n)
 *
 * remain ordinary expression calls and are classified semantically.
 */
vectorExpression
    : vector
    | vectorIndexedExpression
    ;


/*
 * ============================================================================
 * 10. VECTOR REFERENCE BOUNDARY
 * ============================================================================
 *
 * A vector reference is not a distinct lexical construct.
 *
 * This wrapper exists for domain dispatchers that need a named semantic
 * boundary while still using the canonical identifier rule.
 *
 * It deliberately does not redefine identifier syntax.
 */
vectorReference
    : identifier
    ;


/*
 * ============================================================================
 * 11. VECTOR VALUE BOUNDARY
 * ============================================================================
 *
 * This rule is intentionally broader than `vectorExpression`.
 *
 * It exists for domain composition where a vector value may already be known
 * semantically through a reference or ordinary expression.
 *
 * Syntax remains canonical Zamani expression syntax.
 */
vectorValue
    : vectorExpression
    | vectorReference
    | expression
    ;
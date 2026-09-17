/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/expressions/indexing.g4
 *
 * Status:
 *     Canonical production indexing-expression component.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar.
 *
 * Rust implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021 edition
 *     Safe Rust only.
 *     No unsafe Rust is required or permitted by the compiler implementation.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the SOURCE-LEVEL SYNTAX of postfix indexing and slicing.
 *
 * It is intentionally a suffix grammar.
 *
 * The postfix-expression composition layer owns the expression being indexed
 * and the ordering/chaining of postfix operations.
 *
 * This file owns:
 *
 *     value[index]
 *     value[i, j]
 *     value[i][j]
 *     value[start:end]
 *     value[start:end:step]
 *     value[:]
 *     value[::step]
 *     value[:end]
 *     value[start:]
 *     value[start:end]
 *
 * The grammar is domain-neutral.
 *
 * The same syntax can therefore be used by semantic analysis for:
 *
 *     arrays
 *     slices
 *     vectors
 *     matrices
 *     tensors
 *     maps
 *     strings
 *     collections
 *     streams
 *     classical memory
 *     symbolic data
 *     quantum registers
 *     logical qubit collections
 *     hardware abstractions
 *     HDL structures
 *     distributed data
 *     AI/ML tensors
 *     user-defined indexable types
 *     future computational domains
 *
 * The grammar does NOT decide which meaning applies.
 *
 * ============================================================================
 * ARCHITECTURAL PIPELINE
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     grammar/antlr/ZamaniLexer.g4
 *          |
 *          v
 *     canonical parser composition
 *          |
 *          v
 *     expression
 *          |
 *          v
 *     postfixExpression
 *          |
 *          +--> indexingSuffix
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     structural validation
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> indexability
 *          +--> type checking
 *          +--> bounds
 *          +--> range semantics
 *          +--> ownership
 *          +--> effects
 *          +--> capabilities
 *          +--> resource requirements
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +----------------------+-----------------------+
 *          |                      |                       |
 *          v                      v                       v
 *     classical IR          quantum::ir            HDL/hardware IR
 *          |                      |                       |
 *          +----------------------+-----------------------+
 *                                 |
 *                                 v
 *                       optimization / lowering
 *                                 |
 *                       routing / scheduling
 *                                 |
 *                       resilience / QEC / ZQN
 *                                 |
 *                                HAL
 *                                 |
 *                         target realization
 *
 * IMPORTANT:
 *
 *     quantum::ir
 *
 * remains the canonical quantum semantic boundary.
 *
 * This file MUST NOT introduce:
 *
 *     - a quantum indexing IR;
 *     - a physical-qubit indexing model;
 *     - a hardware-memory indexing model;
 *     - a tensor-specific AST;
 *     - a QEC representation;
 *     - a routing representation;
 *     - a scheduling representation.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - indexing suffix syntax;
 *     - bracket delimiters as parser tokens;
 *     - index argument lists;
 *     - comma-separated index components;
 *     - scalar/dynamic index expressions;
 *     - range/slice components inside indexing;
 *     - open slice bounds;
 *     - optional slice steps;
 *     - preservation of source ordering;
 *     - the `indexingSuffix` postfix integration boundary.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - the base expression;
 *     - primary expressions;
 *     - identifiers;
 *     - literals;
 *     - arithmetic precedence;
 *     - logical precedence;
 *     - conditional expressions;
 *     - assignment;
 *     - function calls;
 *     - member access;
 *     - declarations;
 *     - type definitions;
 *     - semantic indexability;
 *     - bounds checking;
 *     - ownership;
 *     - borrowing;
 *     - effects;
 *     - capabilities;
 *     - resource allocation;
 *     - hardware selection;
 *     - quantum semantics;
 *     - physical qubit mapping;
 *     - routing;
 *     - scheduling;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - optimization;
 *     - runtime execution;
 *     - ABI selection.
 *
 * ============================================================================
 * POSTFIX COMPOSITION CONTRACT
 * ============================================================================
 *
 * `grammar/expressions/postfix.g4` owns:
 *
 *     postfixExpression
 *     postfixPart
 *
 * Conceptually:
 *
 *     postfixExpression
 *         : primaryExpression postfixPart*
 *         ;
 *
 * and:
 *
 *     postfixPart
 *         : callSuffix
 *         | indexingSuffix
 *         | memberSuffix
 *         | ...
 *         ;
 *
 * This file therefore MUST NOT define:
 *
 *     postfixExpression
 *
 * This file MUST NOT import:
 *
 *     postfix.g4
 *
 * This prevents:
 *
 *     Postfix -> Indexing -> Postfix
 *
 * dependency cycles.
 *
 * The public integration rule is:
 *
 *     indexingSuffix
 *
 * ============================================================================
 * LEXER AUTHORITY
 * ============================================================================
 *
 * The canonical lexical authority is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * This parser grammar defines NO lexer rules.
 *
 * The canonical tokens used by this file are:
 *
 *     LBRACKET
 *     RBRACKET
 *     COMMA
 *     COLON
 *
 * Range operators remain owned by the canonical lexer:
 *
 *     DOT_DOT
 *     DOT_DOT_EQ
 *
 * However, range operators are NOT required for slice syntax in this file.
 * Slice syntax uses COLON.
 *
 * IMPORTANT:
 *
 * The repository's canonical lexer defines:
 *
 *     LBRACKET : '[' ;
 *     RBRACKET : ']' ;
 *     COMMA    : ',' ;
 *     COLON    : ':' ;
 *
 * Therefore this grammar MUST NOT use:
 *
 *     LBRACK
 *     RBRACK
 *
 * or introduce aliases for them.
 *
 * ============================================================================
 * CORE INDEXING MODEL
 * ============================================================================
 *
 * One indexing suffix has exactly one pair of brackets:
 *
 *     LBRACKET
 *         indexArgumentList?
 *     RBRACKET
 *
 * Multiple indexing operations are composed by `postfixExpression`.
 *
 * Therefore:
 *
 *     value[i][j][k]
 *
 * is represented structurally as:
 *
 *     value
 *       -> indexingSuffix(i)
 *       -> indexingSuffix(j)
 *       -> indexingSuffix(k)
 *
 * This file MUST NOT attempt to consume the base expression.
 *
 * ============================================================================
 * SCALABILITY / POCO-REAF
 * ============================================================================
 *
 * There is NO language-level maximum for:
 *
 *     index dimensions;
 *     index components;
 *     chained indexing operations;
 *     collection size;
 *     tensor rank;
 *     qubit count;
 *     register size;
 *     memory size;
 *     number of resources;
 *     number of devices;
 *     number of nodes;
 *     number of processors;
 *     number of accelerators;
 *     vector width;
 *     tensor dimensions;
 *     distributed participants.
 *
 * Repetition is represented structurally:
 *
 *     (...)* 
 *
 * and:
 *
 *     (...)? 
 *
 * No grammar constant such as:
 *
 *     MAX_INDEXES
 *     MAX_DIMENSIONS
 *     MAX_INDEX_DEPTH
 *     MAX_TENSOR_RANK
 *     MAX_QUBITS
 *     MAX_MEMORY
 *
 * is permitted.
 *
 * "Infinity" means that the language grammar imposes no artificial finite
 * semantic ceiling.
 *
 * Actual parser/compiler/runtime limits remain implementation/resource
 * constraints and MUST NOT become universal language limits.
 *
 * ============================================================================
 * INDEXING FORMS
 * ============================================================================
 *
 * Single index:
 *
 *     value[i]
 *
 * Multiple indices:
 *
 *     value[i, j]
 *     value[i, j, k]
 *     value[i, j, k, ...]
 *
 * Chained indices:
 *
 *     value[i][j]
 *
 * Slice:
 *
 *     value[start:end]
 *
 * Open-start:
 *
 *     value[:end]
 *
 * Open-end:
 *
 *     value[start:]
 *
 * Entire range:
 *
 *     value[:]
 *
 * Stepped slice:
 *
 *     value[start:end:step]
 *
 * Open-start stepped slice:
 *
 *     value[:end:step]
 *
 * Open-end stepped slice:
 *
 *     value[start::step]
 *
 * Entire stepped slice:
 *
 *     value[::step]
 *
 * The semantic layer determines which forms are legal for the indexed type.
 *
 * ============================================================================
 * INDEX ARGUMENT MODEL
 * ============================================================================
 *
 * An index argument is either:
 *
 *     ordinary expression
 *
 * or:
 *
 *     slice expression
 *
 * The grammar does not determine whether an ordinary expression is:
 *
 *     integer index;
 *     string key;
 *     enum key;
 *     symbolic index;
 *     runtime-computed index;
 *     distributed partition;
 *     quantum logical index;
 *     hardware abstraction index;
 *     user-defined key.
 *
 * Those meanings belong to semantic analysis.
 *
 * ============================================================================
 * NO FIXED INDEX ARITY
 * ============================================================================
 *
 * The following must all remain structurally representable:
 *
 *     a[i]
 *     a[i, j]
 *     a[i, j, k]
 *     a[i, j, k, l]
 *     a[i, j, k, l, ...]
 *
 * The grammar MUST NOT define separate finite rules such as:
 *
 *     index1
 *     index2
 *     index3
 *     index4
 *
 * as a universal limit.
 *
 * ============================================================================
 * INDEX EXPRESSION INTEGRATION
 * ============================================================================
 *
 * Index expressions consume the canonical `expression` rule.
 *
 * This is intentional.
 *
 * For example:
 *
 *     a[i + 1]
 *     a[f(x)]
 *     a[x * y]
 *     a[condition ? left : right]
 *     a[(i + offset)]
 *     a[compute_index(data)]
 *
 * all use the canonical expression hierarchy.
 *
 * This file MUST NOT duplicate:
 *
 *     additiveExpression
 *     multiplicativeExpression
 *     logicalOrExpression
 *     conditionalExpression
 *     postfixExpression
 *     primaryExpression
 *
 * This prevents competing precedence authorities.
 *
 * ============================================================================
 * RANGE / SLICE INTEGRATION
 * ============================================================================
 *
 * Slice syntax is intentionally distinct from the general range-expression
 * syntax.
 *
 * General ranges use:
 *
 *     ..
 *     ..=
 *
 * and are owned by the range-expression layer.
 *
 * Index slices use:
 *
 *     :
 *
 * because:
 *
 *     a[start:end]
 *
 * describes selection bounds within an indexing operation.
 *
 * The slice grammar preserves:
 *
 *     start present/absent
 *     end present/absent
 *     step present/absent
 *
 * It does NOT synthesize defaults.
 *
 * In particular, the parser MUST NOT turn:
 *
 *     [:]
 *
 * into:
 *
 *     [0:MAX]
 *
 * or:
 *
 *     [0:infinity]
 *
 * Such interpretation belongs to semantic analysis.
 *
 * ============================================================================
 * SLICE STEP
 * ============================================================================
 *
 * The optional third component:
 *
 *     start:end:step
 *
 * is syntactic metadata.
 *
 * It does not imply:
 *
 *     integer arithmetic;
 *     positive step;
 *     negative step;
 *     unit step;
 *     finite iteration;
 *     materialization.
 *
 * Semantic analysis determines:
 *
 *     step type;
 *     zero-step validity;
 *     sign semantics;
 *     ordering;
 *     termination;
 *     laziness;
 *     materialization.
 *
 * ============================================================================
 * EMPTY INDEX LIST
 * ============================================================================
 *
 * `value[]` is deliberately NOT accepted by this grammar.
 *
 * An empty bracket pair is not an index operation.
 *
 * If the language later needs an operation with empty brackets, it MUST be
 * specified as a separate construct with an explicit semantic contract.
 *
 * This avoids silently assigning an interpretation such as:
 *
 *     value[]
 *
 * meaning:
 *
 *     all elements
 *
 *     or:
 *
 *     inferred index
 *
 *     or:
 *
 *     allocation.
 *
 * The established all-elements slice form is:
 *
 *     value[:]
 *
 * ============================================================================
 * TRAILING COMMAS
 * ============================================================================
 *
 * A trailing comma is NOT accepted in an index list:
 *
 *     value[i,]
 *
 * unless the language specification explicitly standardizes such syntax.
 *
 * The current indexing contract treats commas as separators between actual
 * index arguments.
 *
 * This avoids introducing a second meaning for:
 *
 *     value[i,]
 *
 * compared with:
 *
 *     value[i]
 *
 * If trailing index commas are adopted later, the change belongs to the
 * compatibility/versioning specification and must preserve AST meaning.
 *
 * ============================================================================
 * NESTED INDEXING
 * ============================================================================
 *
 * Nested indexing is naturally represented by postfix composition.
 *
 * Examples:
 *
 *     a[i][j]
 *     tensor[i, j][k]
 *     matrix[row][column]
 *     data[key][subkey]
 *
 * Each bracket pair remains a distinct postfix operation.
 *
 * The parser MUST preserve the source order.
 *
 * ============================================================================
 * INDEXING + CALLS
 * ============================================================================
 *
 * Indexing and calls may be freely composed through the postfix layer:
 *
 *     f(x)[i]
 *     a[i](x)
 *     object.method(x)[i]
 *     tensor[i].field()
 *     factory()(i)[j]
 *
 * This file owns only the bracket portion.
 *
 * Calls remain owned by:
 *
 *     grammar/expressions/calls.g4
 *
 * Member access remains owned by:
 *
 *     grammar/expressions/postfix.g4
 *
 * ============================================================================
 * INDEXING + MEMBER ACCESS
 * ============================================================================
 *
 * Examples:
 *
 *     value[i].field
 *     value[i].method(x)
 *     value.field[i]
 *     value.field[i].other[j]
 *
 * The ordering is preserved by postfix composition.
 *
 * This file MUST NOT attempt to consume:
 *
 *     DOT
 *     DOUBLE_COLON
 *
 * as member-access syntax.
 *
 * ============================================================================
 * CLASSICAL COMPUTING INTEGRATION
 * ============================================================================
 *
 * Indexing may semantically operate on:
 *
 *     arrays;
 *     vectors;
 *     matrices;
 *     tensors;
 *     slices;
 *     strings;
 *     maps;
 *     records;
 *     memory abstractions;
 *     sparse structures;
 *     symbolic structures;
 *     streams.
 *
 * No classical machine limit is encoded.
 *
 * For example:
 *
 *     matrix[row, column]
 *
 * does not impose a maximum matrix dimension.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Indexing may syntactically select from quantum abstractions:
 *
 *     q[i]
 *     q[i, j]
 *     q[start:end]
 *
 * The grammar does NOT determine whether `q` is:
 *
 *     logical qubit register;
 *     quantum state container;
 *     logical resource;
 *     symbolic register;
 *     runtime quantum object;
 *     another user-defined abstraction.
 *
 * Semantic analysis determines the meaning.
 *
 * IMPORTANT:
 *
 *     q[0]
 *
 * does NOT mean:
 *
 *     physical_qubit(0)
 *
 * and:
 *
 *     q[0:n]
 *
 * does NOT establish a physical qubit range.
 *
 * Physical realization remains downstream:
 *
 *     semantic quantum model
 *         ->
 *     quantum::ir
 *         ->
 *     optimization
 *         ->
 *     routing
 *         ->
 *     scheduling
 *         ->
 *     QEC / resilience / ZQN
 *         ->
 *     HAL
 *         ->
 *     target hardware
 *
 * This grammar introduces no competing quantum IR.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Indexing can be used for:
 *
 *     parameterized buses;
 *     arrays of signals;
 *     memories;
 *     generated hardware structures;
 *     register abstractions;
 *     tensor hardware;
 *     accelerator data.
 *
 * Example:
 *
 *     memory[address]
 *
 * is source-level indexing.
 *
 * It does not select a physical memory bank unless downstream semantics
 * explicitly establish such a mapping.
 *
 * No physical address width is encoded.
 *
 * ============================================================================
 * DISTRIBUTED COMPUTING
 * ============================================================================
 *
 * Indexing can represent:
 *
 *     partition selection;
 *     shard selection;
 *     task-domain selection;
 *     worker-domain selection;
 *     distributed collection access.
 *
 * It does not impose:
 *
 *     maximum node count;
 *     fixed cluster size;
 *     fixed shard count;
 *     fixed worker count.
 *
 * ============================================================================
 * AI / DATA / TENSOR INTEGRATION
 * ============================================================================
 *
 * Examples:
 *
 *     tensor[i]
 *     tensor[i, j]
 *     tensor[i, j, k]
 *     tensor[start:end, feature]
 *
 * remain syntactically generic.
 *
 * The grammar does not encode:
 *
 *     maximum rank;
 *     maximum shape;
 *     maximum batch size;
 *     maximum tensor dimension;
 *     accelerator count;
 *     memory capacity.
 *
 * Semantic/type/resource analysis determines feasibility.
 *
 * ============================================================================
 * POCO-REAF RESOURCE SEPARATION
 * ============================================================================
 *
 * Index syntax describes WHAT is selected.
 *
 * It does not describe WHICH physical resource performs the selection.
 *
 * Distinguish:
 *
 *     semantic requirement
 *     capability requirement
 *     preference
 *     hint
 *     implementation decision
 *
 * For example:
 *
 *     data[index]
 *
 * expresses an indexing operation.
 *
 * It does not express:
 *
 *     use_gpu_0
 *     use_memory_bank_3
 *     use_qpu_1
 *     use_physical_qubit_7
 *
 * Those are downstream target-realization concerns.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Every indexing suffix MUST lower to the existing domain-neutral frontend
 * AST.
 *
 * Conceptually:
 *
 *     IndexOperation {
 *         arguments,
 *         source_span
 *     }
 *
 * Each argument should preserve enough structure to distinguish:
 *
 *     Index(expression)
 *
 * from:
 *
 *     Slice {
 *         start?,
 *         end?,
 *         step?
 *     }
 *
 * The exact Rust representation belongs to:
 *
 *     src/frontend/ast/
 *
 * This grammar MUST NOT introduce:
 *
 *     QuantumIndex
 *     TensorIndex
 *     MatrixIndex
 *     HardwareIndex
 *     PhysicalQubitIndex
 *     MemoryBankIndex
 *
 * as domain-specific frontend AST variants.
 *
 * The AST MUST preserve:
 *
 *     - source order;
 *     - complete source span;
 *     - bracket span;
 *     - argument spans;
 *     - slice delimiter spans;
 *     - omitted-bound presence/absence;
 *     - expression nesting.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis owns:
 *
 *     - determining whether the base value is indexable;
 *     - determining index arity;
 *     - determining index types;
 *     - resolving map keys;
 *     - validating bounds;
 *     - validating slices;
 *     - validating steps;
 *     - determining whether negative indices are supported;
 *     - determining whether open bounds are supported;
 *     - determining whether indexing is mutable;
 *     - ownership/borrowing;
 *     - effect checking;
 *     - capability checking;
 *     - resource checking;
 *     - quantum legality;
 *     - HDL legality;
 *     - distributed legality;
 *     - tensor shape compatibility.
 *
 * The parser MUST NOT perform these checks.
 *
 * ============================================================================
 * BOUNDS AND NUMERIC MAGNITUDE
 * ============================================================================
 *
 * The parser accepts index expressions without imposing numeric limits.
 *
 * Examples:
 *
 *     a[0]
 *     a[999999999999999999999999]
 *     a[index]
 *     a[index + offset]
 *
 * Whether a particular numeric value is representable by the semantic type
 * is a type/semantic concern.
 *
 * The grammar MUST NOT define:
 *
 *     MAX_INDEX_VALUE
 *     MAX_ARRAY_SIZE
 *     MAX_TENSOR_DIMENSION
 *     MAX_QUANTUM_REGISTER
 *
 * ============================================================================
 * LAZINESS / MATERIALIZATION
 * ============================================================================
 *
 * Indexing syntax does not require materialization.
 *
 * A semantic implementation may represent:
 *
 *     data[start:end]
 *
 * as:
 *
 *     lazy view;
 *     slice descriptor;
 *     iterator;
 *     symbolic selection;
 *     distributed partition;
 *     vectorized operation;
 *     hardware view;
 *     another representation.
 *
 * The grammar remains unchanged.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     source token sequence;
 *     active grammar version.
 *
 * Parsing MUST NOT depend on:
 *
 *     CPU count;
 *     GPU count;
 *     QPU availability;
 *     memory capacity;
 *     device topology;
 *     filesystem state;
 *     network state;
 *     randomness;
 *     system time;
 *     runtime scheduler state.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Parsing an index operation MUST NOT execute the base expression.
 *
 * The parser MUST NOT:
 *
 *     - access memory;
 *     - access devices;
 *     - inspect hardware;
 *     - access files;
 *     - access networks;
 *     - invoke functions;
 *     - invoke quantum hardware;
 *     - invoke HDL simulators;
 *     - invoke compiler backends.
 *
 * ============================================================================
 * ERROR / DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Syntax errors include:
 *
 *     missing `]`;
 *     missing expression after comma;
 *     repeated comma;
 *     malformed slice;
 *     malformed slice step;
 *     malformed bracket structure.
 *
 * Semantic errors include:
 *
 *     non-indexable value;
 *     invalid index type;
 *     wrong index arity;
 *     invalid bounds;
 *     invalid step;
 *     unsupported slice;
 *     out-of-range index;
 *     invalid quantum selection;
 *     invalid hardware/resource selection.
 *
 * Semantic errors MUST be diagnosed downstream.
 *
 * Diagnostics should retain source spans for:
 *
 *     complete indexing operation;
 *     each index expression;
 *     each slice bound;
 *     each colon;
 *     opening bracket;
 *     closing bracket.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing canonical forms remain:
 *
 *     value[index]
 *     value[index, other]
 *     value[index][other]
 *     value[start:end]
 *     value[:end]
 *     value[start:]
 *     value[:]
 *     value[start:end:step]
 *     value[:end:step]
 *     value[start::step]
 *     value[::step]
 *
 * No new physical-resource meaning may be attached to existing syntax.
 *
 * Any future extension must be versioned through the repository's
 * compatibility/versioning mechanism.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive syntax:
 *
 *     value[i]
 *     value[i, j]
 *     value[i, j, k]
 *     value[i][j]
 *     value[i][j][k]
 *
 * Expression indices:
 *
 *     value[i + 1]
 *     value[f(x)]
 *     value[(i + offset)]
 *     value[condition ? a : b]
 *
 * Slices:
 *
 *     value[start:end]
 *     value[:end]
 *     value[start:]
 *     value[:]
 *
 * Stepped slices:
 *
 *     value[start:end:step]
 *     value[:end:step]
 *     value[start::step]
 *     value[::step]
 *
 * Nested:
 *
 *     tensor[i, j][k]
 *     matrix[row][column].field
 *     factory()[i][j]
 *     object.field[i].method()
 *
 * Quantum/domain-neutral:
 *
 *     q[i]
 *     q[start:end]
 *     tensor[i, j, k]
 *     memory[address]
 *     accelerator.buffers[i]
 *
 * Negative syntax:
 *
 *     value[
 *     value[i
 *     value[, i]
 *     value[i,,j]
 *     value[i,]
 *     value[: :]
 *     value[start:end:step:extra]
 *
 * Boundary:
 *
 *     one index;
 *     multiple indices;
 *     open slice;
 *     closed slice;
 *     stepped slice;
 *     nested postfix chains;
 *     dynamically computed indices;
 *     very large source-level index expressions.
 *
 * Scalability:
 *
 *     no fixed index count;
 *     no fixed index depth;
 *     no fixed tensor rank;
 *     no fixed collection size;
 *     no fixed qubit count;
 *     no fixed memory size.
 *
 * Determinism:
 *
 *     identical source + grammar version => identical parse structure.
 *
 * Compatibility:
 *
 *     existing indexing syntax remains structurally stable.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden language-level limits include:
 *
 *     MAX_INDEXES
 *     MAX_INDEX_DEPTH
 *     MAX_DIMENSIONS
 *     MAX_TENSOR_RANK
 *     MAX_ARRAY_SIZE
 *     MAX_MEMORY
 *     MAX_QUBITS
 *     MAX_REGISTERS
 *     MAX_NODES
 *     MAX_DEVICES
 *
 * None are represented by this grammar.
 *
 * No hardware identifiers are represented.
 *
 * No physical addresses are represented.
 *
 * No physical qubit identifiers are represented.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] canonical lexer token names are consumed;
 *     [x] no local lexer rules exist;
 *     [x] no `LBRACK`/`RBRACK` aliases exist;
 *     [x] `indexingSuffix` is the public postfix integration boundary;
 *     [x] the base expression is NOT owned here;
 *     [x] canonical `expression` is consumed for index expressions;
 *     [x] scalar/dynamic indices are supported;
 *     [x] arbitrary index-list arity is supported;
 *     [x] chained indexing is supported by postfix composition;
 *     [x] open slices are supported;
 *     [x] closed slices are supported;
 *     [x] stepped slices are supported;
 *     [x] omitted bounds remain omitted in the parse tree;
 *     [x] no artificial hardware/resource limit exists;
 *     [x] no quantum gate inventory exists;
 *     [x] no physical qubit mapping exists;
 *     [x] no quantum IR is introduced;
 *     [x] AST integration is defined;
 *     [x] semantic integration is defined;
 *     [x] compiler integration is defined;
 *     [x] runtime integration is defined;
 *     [x] tooling/diagnostic integration is defined;
 *     [x] positive tests are defined;
 *     [x] negative tests are defined;
 *     [x] boundary tests are defined;
 *     [x] scalability tests are defined;
 *     [x] determinism requirements are defined;
 *     [x] compatibility requirements are defined;
 *     [x] hard-coding audit is defined.
 *
 * ============================================================================
 * CANONICAL GRAMMAR
 * ============================================================================
 *
 * This grammar intentionally contains ONLY the indexing suffix and the
 * index/slice structures consumed inside its brackets.
 *
 * The base expression remains owned by the canonical expression hierarchy.
 *
 * ============================================================================
 */

parser grammar Indexing;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * 1. PUBLIC POSTFIX INTEGRATION
 * ========================================================================== */

/**
 * Canonical indexing postfix suffix.
 *
 * Consumed by:
 *
 *     grammar/expressions/postfix.g4
 *
 * Conceptually:
 *
 *     postfixPart
 *         : callSuffix
 *         | indexingSuffix
 *         | memberSuffix
 *         | ...
 *         ;
 *
 * The expression being indexed is supplied by `postfixExpression`.
 *
 * Examples:
 *
 *     value[i]
 *     value[i, j]
 *     value[:]
 *     value[start:end]
 */
indexingSuffix
    : LBRACKET indexArgumentList RBRACKET
    ;


/* ============================================================================
 * 2. INDEX ARGUMENT LIST
 * ========================================================================== */

/**
 * One or more index arguments.
 *
 * Empty brackets are intentionally excluded.
 *
 * No finite index arity is imposed.
 */
indexArgumentList
    : indexArgument (COMMA indexArgument)*
    ;


/* ============================================================================
 * 3. INDEX ARGUMENT
 * ========================================================================== */

/**
 * An index argument is either:
 *
 *     an ordinary expression
 *
 * or:
 *
 *     a slice expression.
 *
 * Slice syntax is selected by the presence of COLON.
 */
indexArgument
    : indexSlice
    | expression
    ;


/* ============================================================================
 * 4. SLICE EXPRESSION
 * ========================================================================== */

/**
 * Slice/index-range syntax.
 *
 * Supported:
 *
 *     [:]
 *     [start:]
 *     [:end]
 *     [start:end]
 *     [::step]
 *     [start::step]
 *     [:end:step]
 *     [start:end:step]
 *
 * The surrounding brackets are owned by `indexingSuffix`.
 *
 * The expression alternatives intentionally consume the canonical `expression`
 * rule rather than recreating arithmetic/logical/conditional precedence.
 *
 * The optional components preserve omission rather than synthesizing values.
 */
indexSlice
    : indexSliceBound? COLON indexSliceBound? indexSliceStep?
    ;


/* ============================================================================
 * 5. OPTIONAL SLICE STEP
 * ========================================================================== */

/**
 * A step exists only after the second COLON.
 *
 * Therefore:
 *
 *     start:end
 *
 * and:
 *
 *     start:end:step
 *
 * remain structurally distinct.
 *
 * Likewise:
 *
 *     ::step
 *
 * represents an omitted start and end with an explicit step.
 */
indexSliceStep
    : COLON expression
    ;


/* ============================================================================
 * 6. SLICE BOUND
 * ========================================================================== */

/**
 * A slice bound is a complete Zamani expression.
 *
 * This permits:
 *
 *     a[i + 1]
 *     a[start + offset:end * scale]
 *     a[compute_start():compute_end()]
 *
 * without duplicating expression precedence here.
 */
indexSliceBound
    : expression
    ;
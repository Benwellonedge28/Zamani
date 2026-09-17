/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/expressions/arrays.g4
 *
 * Status:
 *     Canonical production-ready array-expression grammar component.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar delegate.
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
 * This file owns the SOURCE-LEVEL SYNTAX of array expressions.
 *
 * It deliberately implements only the array-literal expression boundary:
 *
 *     []
 *     [a]
 *     [a, b]
 *     [a, b, c]
 *     [[a, b], [c, d]]
 *     [f(x), g(y), h(z)]
 *
 * Array elements are canonical Zamani expressions.
 *
 * This means array literals can contain:
 *
 *     literals
 *     identifiers
 *     unary expressions
 *     binary expressions
 *     conditional expressions
 *     ranges
 *     calls
 *     indexing
 *     member access
 *     tuples
 *     nested arrays
 *     quantum expressions
 *     classical expressions
 *     hybrid expressions
 *     HDL expressions
 *     resource expressions
 *     data expressions
 *     AI/ML expressions
 *     future expression extensions
 *
 * This file does NOT own:
 *
 *     array indexing
 *     array types
 *     array semantics
 *     array memory layout
 *     array allocation
 *     array bounds
 *     array capacity
 *     tensor semantics
 *     vector semantics
 *     hardware mapping
 *     quantum mapping
 *     routing
 *     scheduling
 *     QEC
 *     ZQN
 *     HAL
 *     backend lowering
 *     runtime representation
 *
 * ============================================================================
 * NORMATIVE SYNTAX CONTRACT
 * ============================================================================
 *
 * The normative syntax specification defines:
 *
 *     ArrayExpression ::=
 *         "["
 *         [ Expression { "," Expression } ]
 *         "]"
 *
 * This file is the ANTLR implementation of that contract.
 *
 * Therefore:
 *
 *     []
 *
 * is valid.
 *
 *     [a]
 *
 * is valid.
 *
 *     [a, b]
 *
 * is valid.
 *
 *     [a, b, c]
 *
 * is valid.
 *
 *     [[a], [b]]
 *
 * is valid.
 *
 * The following are NOT array expressions under this contract:
 *
 *     [a,]
 *     [a, b,]
 *
 * Trailing commas therefore remain intentionally rejected here unless the
 * normative language specification is explicitly changed through the
 * compatibility/versioning process.
 *
 * This is important because the existing generic argument-list machinery
 * permits trailing commas in some contexts. Array literals have their own
 * syntax contract and must not inherit unrelated argument-list behavior.
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
 *     canonical expression grammar
 *          |
 *          +--------------------------+
 *          |                          |
 *          v                          v
 *     primaryExpression         arrayExpression
 *                                     |
 *                                     v
 *                              arrayElementList
 *                                     |
 *                                     v
 *                                expression
 *                                     |
 *                                     v
 *                            domain-neutral AST
 *                                     |
 *                                     v
 *                         structural validation
 *                                     |
 *                                     v
 *                           semantic analysis
 *                                     |
 *                                     v
 *                         canonical semantic model
 *                                     |
 *                    +----------------+----------------+
 *                    |                |                |
 *                    v                v                v
 *               classical       quantum::ir      HDL/hardware
 *                    |                |                |
 *                    +----------------+----------------+
 *                                     |
 *                                     v
 *                           optimization / lowering
 *                                     |
 *                           routing / scheduling
 *                                     |
 *                           resilience / QEC / ZQN
 *                                     |
 *                                    HAL
 *                                     |
 *                             target realization
 *
 * IMPORTANT:
 *
 * `quantum::ir` remains the canonical quantum semantic boundary.
 *
 * This file must never create:
 *
 *     QuantumArrayIR
 *     QuantumArrayAST
 *     PhysicalQubitArray
 *     QECArray
 *     HardwareArrayIR
 *
 * or any equivalent domain-specific duplicate representation.
 *
 * ============================================================================
 * SINGLE RESPONSIBILITY
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - array-expression delimiters;
 *     - array element sequencing;
 *     - comma-separated array elements;
 *     - empty array expressions;
 *     - single-element array expressions;
 *     - multi-element array expressions;
 *     - recursive/nested array expressions through `expression`;
 *     - the stable `arrayExpression` integration rule.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - expression precedence;
 *     - primary expressions generally;
 *     - identifiers;
 *     - literals;
 *     - tuples;
 *     - calls;
 *     - member access;
 *     - indexing;
 *     - ranges;
 *     - unary operators;
 *     - binary operators;
 *     - assignments;
 *     - array types;
 *     - slices;
 *     - maps;
 *     - tensor types;
 *     - memory allocation;
 *     - semantic type checking;
 *     - ownership;
 *     - borrowing;
 *     - resource management;
 *     - hardware selection;
 *     - quantum physical mapping;
 *     - scheduling;
 *     - routing;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - optimization;
 *     - runtime execution.
 *
 * ============================================================================
 * COMPOSITION CONTRACT
 * ============================================================================
 *
 * The canonical expression composition grammar owns:
 *
 *     expression
 *     assignmentExpression
 *     ...
 *     postfixExpression
 *     primaryExpression
 *
 * This file provides:
 *
 *     arrayExpression
 *
 * The canonical expression grammar must import/use this delegate as part of
 * `primaryExpression`.
 *
 * Conceptually:
 *
 *     primaryExpression
 *         : ...
 *         | arrayExpression
 *         | ...
 *         ;
 *
 * This file MUST NOT define:
 *
 *     expression
 *     postfixExpression
 *     primaryExpression
 *
 * Doing so would create a second expression hierarchy.
 *
 * ============================================================================
 * DEPENDENCY-DIRECTION CONTRACT
 * ============================================================================
 *
 * The dependency direction is:
 *
 *     expression composition
 *             |
 *             v
 *       arrayExpression
 *             |
 *             v
 *        expression
 *
 * At ANTLR composition time, `expression` is supplied by the canonical
 * expression grammar.
 *
 * Therefore this file intentionally does NOT import the canonical Expression
 * grammar back into itself.
 *
 * DO NOT add:
 *
 *     import Expression;
 *
 * here if the canonical expression grammar already imports this delegate.
 *
 * That would create a grammar-composition cycle:
 *
 *     Expression -> Arrays -> Expression
 *
 * The composition root is responsible for assembling the delegates.
 *
 * ============================================================================
 * LEXER AUTHORITY
 * ============================================================================
 *
 * This file contains NO lexer rules.
 *
 * The canonical lexical authority remains the repository's canonical lexer
 * assembly.
 *
 * Required parser-facing tokens:
 *
 *     LBRACKET
 *     RBRACKET
 *     COMMA
 *
 * No aliases such as:
 *
 *     LBRACK
 *     RBRACK
 *
 * may be introduced here.
 *
 * Array elements consume:
 *
 *     expression
 *
 * from the canonical expression hierarchy.
 *
 * This file therefore does not duplicate:
 *
 *     INTEGER_LITERAL
 *     IDENTIFIER
 *     STRING_LITERAL
 *     FLOAT_LITERAL
 *     QUANTUM_LITERAL
 *     BOOLEAN_LITERAL
 *     ...
 *
 * Literal tokenization remains the lexer responsibility.
 *
 * ============================================================================
 * ARRAY SYNTAX
 * ============================================================================
 *
 * Empty array:
 *
 *     []
 *
 * Single element:
 *
 *     [x]
 *
 * Multiple elements:
 *
 *     [x, y]
 *     [x, y, z]
 *
 * Nested:
 *
 *     [[x, y], [z, w]]
 *
 * Arbitrarily nested:
 *
 *     [[[x]]]
 *
 * Computed elements:
 *
 *     [f(x), g(y)]
 *
 * Expressions:
 *
 *     [a + b, c * d]
 *
 * Conditional:
 *
 *     [condition ? a : b, value]
 *
 * Quantum:
 *
 *     [q0, q1, q2]
 *
 * Hybrid:
 *
 *     [measure(q), classical_value]
 *
 * The grammar does not distinguish these cases.
 *
 * ============================================================================
 * ARRAY ELEMENT SEMANTICS
 * ============================================================================
 *
 * Every element is parsed using the canonical:
 *
 *     expression
 *
 * rule.
 *
 * This is deliberate.
 *
 * The array grammar must NOT create specialized alternatives such as:
 *
 *     arrayIntegerElement
 *     arrayFloatElement
 *     arrayQuantumElement
 *     arrayTensorElement
 *     arrayHardwareElement
 *
 * because doing so would move semantic typing into the grammar and fragment
 * the language.
 *
 * Semantic analysis determines whether all elements are compatible with the
 * expected array type or whether the language permits heterogeneous arrays.
 *
 * ============================================================================
 * NESTED ARRAYS
 * ============================================================================
 *
 * Nested arrays are naturally supported because:
 *
 *     arrayExpression
 *         -> arrayElementList
 *             -> expression
 *                 -> primaryExpression
 *                     -> arrayExpression
 *
 * Examples:
 *
 *     []
 *     [[]]
 *     [[1]]
 *     [[1, 2], [3, 4]]
 *     [[[1], [2]], [[3], [4]]]
 *
 * No explicit nesting-depth constant is permitted.
 *
 * ============================================================================
 * SCALABILITY / POCO-REAF
 * ============================================================================
 *
 * The grammar imposes NO universal maximum on:
 *
 *     array element count;
 *     array nesting;
 *     number of arrays;
 *     array dimensionality;
 *     expression size;
 *     program size;
 *     classical values;
 *     quantum values;
 *     tensor dimensions;
 *     tensor rank;
 *     qubit count;
 *     CPU count;
 *     GPU count;
 *     FPGA count;
 *     accelerator count;
 *     memory capacity;
 *     node count;
 *     process count;
 *     thread count;
 *     resource count.
 *
 * No constructs such as:
 *
 *     MAX_ARRAY_ELEMENTS
 *     MAX_ARRAY_DIMENSIONS
 *     MAX_ARRAY_DEPTH
 *     MAX_TENSOR_RANK
 *     MAX_QUBITS
 *     MAX_MEMORY
 *
 * may appear in this grammar.
 *
 * ANTLR repetition operators express unbounded language structure:
 *
 *     (COMMA expression)*
 *
 * Actual limits may exist at:
 *
 *     lexer resource policy;
 *     parser resource policy;
 *     compiler resource policy;
 *     available memory;
 *     available storage;
 *     target capability;
 *     runtime resources;
 *     deployment policy.
 *
 * Those are implementation/resource constraints, not language syntax limits.
 *
 * This distinction is essential to POCO-REAF.
 *
 * ============================================================================
 * "INFINITY" INTERPRETATION
 * ============================================================================
 *
 * Zamani's scalability requirement means that this grammar does not establish
 * an artificial finite upper bound.
 *
 * It does NOT mean that a physical machine has infinite memory or that a
 * compiler can allocate infinite storage.
 *
 * Therefore:
 *
 *     language capability
 *
 * and:
 *
 *     available implementation resources
 *
 * remain separate.
 *
 * A program containing one million array elements is not syntactically invalid
 * merely because one particular compiler invocation cannot allocate enough
 * memory to process it.
 *
 * A configured compiler resource policy may reject such an input operationally,
 * but that policy must not be encoded as an array-language restriction.
 *
 * ============================================================================
 * NO MACHINE-SPECIFIC SEMANTICS
 * ============================================================================
 *
 * The following are NOT determined by this grammar:
 *
 *     element size;
 *     pointer size;
 *     memory alignment;
 *     physical address;
 *     cache placement;
 *     NUMA placement;
 *     GPU memory;
 *     FPGA block RAM;
 *     QPU memory;
 *     quantum register allocation;
 *     physical qubit mapping;
 *     network placement;
 *     distributed ownership;
 *     accelerator selection.
 *
 * For example:
 *
 *     [1, 2, 3]
 *
 * is source-level array syntax.
 *
 * It is not inherently:
 *
 *     stack allocation;
 *     heap allocation;
 *     GPU allocation;
 *     FPGA memory;
 *     quantum memory;
 *     contiguous physical memory.
 *
 * Those decisions are downstream.
 *
 * ============================================================================
 * CLASSICAL COMPUTING INTEGRATION
 * ============================================================================
 *
 * Arrays may represent, after semantic analysis:
 *
 *     scalar collections;
 *     vectors;
 *     matrices;
 *     tensors;
 *     scientific data;
 *     numerical data;
 *     symbolic data;
 *     records;
 *     streams;
 *     application data.
 *
 * The syntax remains identical.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Arrays may syntactically contain quantum expressions:
 *
 *     [q0, q1, q2]
 *
 * or nested structures:
 *
 *     [[q0, q1], [q2, q3]]
 *
 * Whether these represent:
 *
 *     logical qubit collections;
 *     quantum state components;
 *     handles;
 *     symbolic values;
 *     another quantum abstraction
 *
 * is semantic.
 *
 * This grammar does NOT:
 *
 *     - allocate qubits;
 *     - choose physical qubits;
 *     - choose a QPU;
 *     - select a topology;
 *     - perform routing;
 *     - schedule operations;
 *     - perform QEC;
 *     - model noise;
 *     - select calibration;
 *     - select a vendor.
 *
 * The downstream path remains:
 *
 *     source
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic quantum representation
 *       |
 *       v
 *     quantum::ir
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     routing
 *       |
 *       v
 *     scheduling
 *       |
 *       v
 *     QEC / resilience / ZQN
 *       |
 *       v
 *     HAL
 *       |
 *       v
 *     target realization
 *
 * ============================================================================
 * HYBRID COMPUTING INTEGRATION
 * ============================================================================
 *
 * An array can contain values from hybrid computations:
 *
 *     [classical_value, quantum_value, measurement]
 *
 * Whether heterogeneous values are legal is a semantic/type-system decision.
 *
 * The grammar must remain neutral.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Arrays can appear in HDL/hardware source constructs where an expression is
 * permitted.
 *
 * Examples may eventually include:
 *
 *     [signal_a, signal_b]
 *     [register_a, register_b]
 *     [parameter_x, parameter_y]
 *
 * This grammar does NOT establish:
 *
 *     bus widths;
 *     register widths;
 *     FPGA resource counts;
 *     ASIC cell counts;
 *     pin counts;
 *     physical addresses;
 *     memory-bank counts.
 *
 * ============================================================================
 * AI / DATA INTEGRATION
 * ============================================================================
 *
 * Arrays are intentionally generic enough to participate in:
 *
 *     datasets;
 *     feature vectors;
 *     tensors;
 *     model parameters;
 *     batches;
 *     sequences;
 *     embeddings;
 *     symbolic structures.
 *
 * Tensor semantics belong to the type/data/semantic layers.
 *
 * The array grammar must not become an AI-framework grammar.
 *
 * ============================================================================
 * DISTRIBUTED / PARALLEL INTEGRATION
 * ============================================================================
 *
 * Array syntax does not imply:
 *
 *     local memory;
 *     distributed memory;
 *     replicated data;
 *     partitioned data;
 *     sharded data;
 *     GPU storage.
 *
 * These are semantic/resource/deployment decisions.
 *
 * Thus:
 *
 *     [a, b, c]
 *
 * remains portable across:
 *
 *     single-machine;
 *     multicore;
 *     accelerator;
 *     distributed;
 *     heterogeneous;
 *     edge;
 *     cloud;
 *     future targets.
 *
 * ============================================================================
 * ARRAY TYPES ARE NOT OWNED HERE
 * ============================================================================
 *
 * This file defines ARRAY EXPRESSIONS.
 *
 * It does not define:
 *
 *     [T]
 *     [T; N]
 *
 * or any other type syntax.
 *
 * Array type syntax belongs under:
 *
 *     grammar/types/
 *
 * The expression:
 *
 *     [1, 2, 3]
 *
 * and a type such as:
 *
 *     [int]
 *
 * are separate language concepts and must not share grammar ownership.
 *
 * ============================================================================
 * INDEXING IS NOT OWNED HERE
 * ============================================================================
 *
 * Array creation:
 *
 *     [a, b, c]
 *
 * is owned here.
 *
 * Indexing:
 *
 *     array[i]
 *
 * is owned by:
 *
 *     grammar/expressions/indexing.g4
 *
 * This distinction is critical.
 *
 * Otherwise `arrays.g4` would compete with `indexing.g4` for bracket syntax.
 *
 * The expression:
 *
 *     values[i]
 *
 * is therefore composed as:
 *
 *     values
 *       -> indexingSuffix(i)
 *
 * whereas:
 *
 *     [values]
 *
 * is:
 *
 *     arrayExpression
 *
 * ============================================================================
 * RANGE INTEGRATION
 * ============================================================================
 *
 * An array element may itself be a range expression:
 *
 *     [start .. end]
 *
 * or:
 *
 *     [start ..= end]
 *
 * because the element consumes canonical `expression`.
 *
 * This file does not define range syntax.
 *
 * Range syntax remains owned by the range expression component.
 *
 * ============================================================================
 * TUPLE INTEGRATION
 * ============================================================================
 *
 * Tuple expressions may be array elements:
 *
 *     [(a, b), (c, d)]
 *
 * The tuple grammar remains responsible for tuple syntax.
 *
 * This file does not import or duplicate tuple syntax.
 *
 * ============================================================================
 * CALL / MEMBER / INDEX INTEGRATION
 * ============================================================================
 *
 * Array elements may contain arbitrary canonical expressions:
 *
 *     [f(x), object.field, values[i]]
 *
 * This file does not define those operations.
 *
 * It delegates their parsing to the canonical expression hierarchy.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The existing native frontend AST already defines the array expression
 * classification:
 *
 *     CoreNodeKind::ArrayExpression
 *
 * and:
 *
 *     ExpressionKind::Array { elements }
 *
 * Therefore this grammar MUST lower structurally to that existing AST concept.
 *
 * No new `ArrayExpression` AST type should be created merely because this
 * grammar is being modularized.
 *
 * The AST must preserve:
 *
 *     - array source span;
 *     - element order;
 *     - element count;
 *     - every element child NodeId;
 *     - nesting structure.
 *
 * Child expressions remain AST graph nodes rather than being embedded directly
 * in this grammar.
 *
 * ============================================================================
 * AST MAPPING
 * ============================================================================
 *
 * Grammar:
 *
 *     arrayExpression
 *
 * maps to:
 *
 *     ExpressionKind::Array {
 *         elements
 *     }
 *
 * where `elements` is the ordered collection of child expression NodeIds
 * established by the existing AST builder.
 *
 * The grammar does not know the Rust representation.
 *
 * The parser is responsible for constructing the AST node.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing establishes only:
 *
 *     "This source is syntactically an array expression."
 *
 * Semantic analysis determines:
 *
 *     element types;
 *     array element compatibility;
 *     inferred array type;
 *     explicit array type compatibility;
 *     ownership;
 *     borrowing;
 *     mutability;
 *     effects;
 *     resource requirements;
 *     allocation strategy;
 *     layout;
 *     laziness;
 *     materialization;
 *     domain-specific meaning.
 *
 * The grammar must not perform those checks.
 *
 * ============================================================================
 * HOMOGENEITY
 * ============================================================================
 *
 * This grammar intentionally does not require:
 *
 *     same-type elements
 *
 * because that is a semantic/type-system rule.
 *
 * Therefore both:
 *
 *     [1, 2, 3]
 *
 * and:
 *
 *     [a, b]
 *
 * are syntactically valid.
 *
 * Whether:
 *
 *     [1, "two", q]
 *
 * is semantically valid is determined by the type system.
 *
 * The grammar must not encode a closed element-type universe.
 *
 * ============================================================================
 * CONSTANT EVALUATION
 * ============================================================================
 *
 * This grammar does not require array elements to be compile-time constants.
 *
 * Therefore:
 *
 *     [f(), x, compute()]
 *
 * is syntactically valid.
 *
 * Whether the context requires compile-time evaluation is a semantic/compiler
 * policy decision.
 *
 * ============================================================================
 * RESOURCE SEMANTICS
 * ============================================================================
 *
 * An array literal does not automatically imply a particular allocation model.
 *
 * For example:
 *
 *     [1, 2, 3]
 *
 * may eventually be:
 *
 *     materialized;
 *     stack-backed;
 *     heap-backed;
 *     static;
 *     lazy;
 *     streamed;
 *     distributed;
 *     accelerator-resident;
 *     transformed away by optimization.
 *
 * This grammar chooses none of those.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Array parsing must be deterministic.
 *
 * The parse result may depend only on:
 *
 *     token sequence;
 *     canonical grammar;
 *     grammar version.
 *
 * It must not depend on:
 *
 *     CPU availability;
 *     GPU availability;
 *     QPU availability;
 *     memory size;
 *     network state;
 *     device topology;
 *     runtime state;
 *     random state;
 *     current time.
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Lexical errors belong to the lexer.
 *
 * Syntax errors belong to the parser.
 *
 * Semantic errors belong to semantic/type analysis.
 *
 * Examples of syntax errors:
 *
 *     [
 *     [a
 *     [a b]
 *     [, a]
 *     [a,,b]
 *     [a,]
 *
 * The final example is intentionally invalid under the normative array syntax
 * contract.
 *
 * Examples of semantic errors:
 *
 *     incompatible element types;
 *     invalid array type;
 *     unsupported resource usage;
 *     invalid quantum array semantics;
 *     unavailable target capability.
 *
 * Those must NOT be encoded as parser alternatives.
 *
 * ============================================================================
 * ERROR RECOVERY
 * ============================================================================
 *
 * ANTLR's normal parser recovery mechanisms remain authoritative.
 *
 * This grammar should not add target-specific recovery behavior.
 *
 * A malformed array should produce diagnostics associated with:
 *
 *     opening `[`;
 *     offending element;
 *     missing comma;
 *     unexpected comma;
 *     missing closing `]`.
 *
 * Source spans are supplied by the lexer/parser infrastructure.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing normative syntax:
 *
 *     []
 *     [a]
 *     [a, b]
 *     [a, b, c]
 *
 * must remain valid.
 *
 * Existing syntax must not silently change meaning.
 *
 * The following are intentionally NOT promoted here without a specification
 * change:
 *
 *     [a,]
 *     [a, b,]
 *     [a; n]
 *     [a .. b]
 *     [:]
 *
 * In particular:
 *
 *     [a; n]
 *
 * must not be introduced as a repeated/fill array initializer unless the
 * language specification explicitly adopts it.
 *
 * Similarly:
 *
 *     [:]
 *
 * is indexing/slicing syntax, not an array literal.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file passes the language-level hard-coding requirement because it
 * contains no finite machine/resource constants.
 *
 * Forbidden examples include:
 *
 *     MAX_ARRAY_ELEMENTS
 *     MAX_ARRAY_DIMENSIONS
 *     MAX_ARRAY_DEPTH
 *     MAX_TENSOR_RANK
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_MEMORY
 *     MAX_NODES
 *     MAX_THREADS
 *
 * Program literals themselves remain legal:
 *
 *     [1, 2, 3]
 *
 * or:
 *
 *     [1024, 2048]
 *
 * A program value is not the same thing as an implementation limit.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * This grammar:
 *
 *     - defines no executable semantic action;
 *     - performs no I/O;
 *     - performs no filesystem access;
 *     - performs no network access;
 *     - performs no hardware discovery;
 *     - performs no runtime execution;
 *     - contains no unsafe Rust;
 *     - contains no native code;
 *     - contains no vendor-specific behavior.
 *
 * Malformed input must remain data processed by the parser.
 *
 * ============================================================================
 * PERFORMANCE / SCALABILITY
 * ============================================================================
 *
 * The element-list production:
 *
 *     expression (COMMA expression)*
 *
 * scales structurally with the number of source elements.
 *
 * No recursive rule is needed merely to represent a flat array element list.
 *
 * Nested arrays remain naturally recursive through the canonical expression
 * hierarchy.
 *
 * Compiler implementation resource limits must be configurable outside this
 * grammar.
 *
 * The grammar itself must not transform resource exhaustion into a language
 * semantic maximum.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Production conformance tests must cover:
 *
 * POSITIVE:
 *
 *     []
 *     [x]
 *     [x, y]
 *     [x, y, z]
 *     [1 + 2, 3 * 4]
 *     [f(x), g(y)]
 *     [[1, 2], [3, 4]]
 *     [[[x]]]
 *     [q0, q1]
 *     [measure(q), result]
 *     [value[index], value[index + 1]]
 *     [condition ? a : b]
 *
 * NEGATIVE:
 *
 *     [
 *     [x
 *     [x y]
 *     [, x]
 *     [x,,y]
 *     [x,]
 *     [x, y,]
 *     ]
 *
 * BOUNDARY:
 *
 *     []
 *     [x]
 *     [x, y]
 *     nested arrays
 *     arrays containing complex expressions
 *     arrays containing quantum expressions
 *     arrays containing domain extensions
 *
 * SCALABILITY:
 *
 *     generated arrays with increasing element counts;
 *     deeply nested arrays;
 *     large expressions as elements;
 *     arrays containing dynamically computed values.
 *
 * No scalability test may encode an artificial universal maximum.
 *
 * DETERMINISM:
 *
 *     identical token streams -> identical parse structures.
 *
 * COMPATIBILITY:
 *
 *     every stable array source form remains accepted across compatible
 *     grammar/compiler versions.
 *
 * ============================================================================
 * INTEGRATION WITH EXISTING REPOSITORY FILES
 * ============================================================================
 *
 * This file integrates with:
 *
 *     grammar/expressions/expression.g4
 *         owns canonical expression precedence/composition.
 *
 *     grammar/expressions/expressions.g4
 *         legacy expression composition surface; must not become another
 *         independent array authority.
 *
 *     grammar/expressions/indexing.g4
 *         owns `value[index]` and slicing; arrays.g4 does not duplicate it.
 *
 *     grammar/expressions/tuples.g4
 *         owns tuple-expression syntax used inside array elements.
 *
 *     grammar/expressions/literals.g4
 *         owns literal-expression syntax/token classification.
 *
 *     grammar/lexer/
 *         owns lexical token definitions.
 *
 *     grammar/Zamani.g4
 *         remains the canonical top-level ANTLR composition root.
 *
 *     grammar/spec/syntax.md
 *         owns normative source syntax.
 *
 *     src/frontend/ast/node/node_kind.rs
 *         already owns ArrayExpression classification.
 *
 *     src/frontend/ast/node/expressions/expression.rs
 *         already maps array expressions to the source AST's expression model.
 *
 *     src/frontend/ast/node/builders/expression_builder.rs
 *         already builds `ExpressionKind::Array { elements }`.
 *
 *     src/parser.rs / frontend parser
 *         constructs the existing Array AST representation from this syntax.
 *
 *     semantic analysis
 *         resolves element types, compatibility, ownership, effects, and
 *         resource meaning.
 *
 *     canonical semantic model / ZUIR
 *         receives the semantic array representation.
 *
 *     quantum::ir
 *         is used only if semantic analysis establishes a quantum meaning;
 *         this grammar does not create a quantum representation.
 *
 * ============================================================================
 * NO-RE-EDIT INTEGRATION GUARANTEE
 * ============================================================================
 *
 * This file is independently complete when:
 *
 *     1. `arrayExpression` is the sole public array-expression rule.
 *     2. Empty arrays are supported.
 *     3. Single-element arrays are supported.
 *     4. Multi-element arrays are supported.
 *     5. Nested arrays are supported.
 *     6. Elements use canonical `expression`.
 *     7. No expression precedence is duplicated.
 *     8. No indexing syntax is duplicated.
 *     9. No array type syntax is duplicated.
 *    10. No lexer rules are duplicated.
 *    11. No fixed element-count limit exists.
 *    12. No fixed dimensionality limit exists.
 *    13. No hardware limit exists.
 *    14. No quantum hardware assumption exists.
 *    15. The existing Array AST contract is reused.
 *    16. Semantic validation remains downstream.
 *    17. Canonical IR lowering remains downstream.
 *    18. Positive/negative/boundary/scalability tests are defined.
 *    19. Determinism is preserved.
 *    20. Compatibility behavior is explicitly defined.
 *
 * Once this contract is complete, unrelated changes to:
 *
 *     quantum/
 *     hdl/
 *     hardware/
 *     ai/
 *     distributed/
 *     networking/
 *     security/
 *     scheduling/
 *     routing/
 *     QEC/
 *     ZQN/
 *     HAL/
 *
 * do not require this file to be rewritten merely because those domains evolve.
 *
 * ============================================================================
 */

parser grammar Arrays;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * PUBLIC ARRAY EXPRESSION
 * ============================================================================
 *
 * Normative form:
 *
 *     []
 *     [expression]
 *     [expression, expression, ...]
 *
 * No trailing comma is accepted.
 */
arrayExpression
    : LBRACKET RBRACKET
    | LBRACKET arrayElementList RBRACKET
    ;


/* ============================================================================
 * ARRAY ELEMENT LIST
 * ============================================================================
 *
 * One or more expressions separated by commas.
 *
 * The `*` repetition is intentionally unbounded by language semantics.
 */
arrayElementList
    : expression (COMMA expression)*
    ;
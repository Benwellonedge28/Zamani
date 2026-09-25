/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * File:
 *     grammar/hdl/arrays.g4
 *
 * Status:
 *     Canonical HDL array-expression parser delegate.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Rust implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     Safe Rust only
 *     No unsafe Rust
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the HDL-specific composition boundary for ARRAY LITERALS.
 *
 * It defines the source syntax required to express a finite sequence of HDL
 * expressions as an array value.
 *
 * Examples:
 *
 *     []
 *     [a]
 *     [a, b]
 *     [a, b, c]
 *     [[a, b], [c, d]]
 *     [signal_a, signal_b]
 *     [register_a, register_b]
 *     [parameter_a + offset, parameter_b * scale]
 *
 * This file is deliberately limited to ARRAY EXPRESSION SYNTAX.
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
 *     canonical HDL parser
 *          |
 *          v
 *     hdlExpression
 *          |
 *          +-----------------------+
 *          |                       |
 *          v                       v
 *     normal HDL primary      hdlArrayLiteral
 *                                  |
 *                                  v
 *                           hdlArrayElementList
 *                                  |
 *                                  v
 *                            hdlExpression
 *                                  |
 *                                  v
 *                         domain-neutral AST
 *                                  |
 *                                  v
 *                         semantic analysis
 *                                  |
 *                    +-------------+-------------+
 *                    |             |             |
 *                    v             v             v
 *               classical      quantum       hardware
 *                                  |
 *                                  v
 *                         canonical semantic IR
 *                                  |
 *                         downstream lowering
 *
 * This file is syntax only.
 *
 * ============================================================================
 * AUTHORITY
 * ============================================================================
 *
 * Normative architectural authority:
 *
 *     grammar/DESIGN.md
 *
 * HDL specification:
 *
 *     grammar/spec/hdl.md
 *
 * HDL composition root:
 *
 *     grammar/hdl/hdl.g4
 *
 * General array-expression specification:
 *
 *     grammar/expressions/arrays.g4
 *
 * General array type grammar:
 *
 *     grammar/types/array.g4
 *
 * Canonical HDL semantic ownership remains outside this file.
 *
 * ============================================================================
 * SINGLE OWNERSHIP RULE
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - hdlArrayLiteral;
 *     - the comma-separated HDL array-element sequence;
 *     - empty HDL array literals;
 *     - one-element HDL array literals;
 *     - multi-element HDL array literals;
 *     - recursive array literals through hdlExpression;
 *     - the HDL parser integration boundary for array literals.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - array types;
 *     - array declarations;
 *     - array indexing;
 *     - array slicing;
 *     - memory declarations;
 *     - memory dimensions;
 *     - vector types;
 *     - tensor types;
 *     - signal declarations;
 *     - register declarations;
 *     - expressions generally;
 *     - expression precedence;
 *     - identifiers;
 *     - literals;
 *     - semantic type checking;
 *     - allocation;
 *     - memory layout;
 *     - physical memory;
 *     - FPGA resources;
 *     - ASIC resources;
 *     - hardware placement;
 *     - routing;
 *     - scheduling;
 *     - synthesis;
 *     - optimization;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime execution.
 *
 * ============================================================================
 * CRITICAL DISTINCTION: ARRAY EXPRESSION VS ARRAY TYPE
 * ============================================================================
 *
 * This file handles VALUES.
 *
 * Example:
 *
 *     [a, b, c]
 *
 * Array TYPE syntax remains owned by:
 *
 *     grammar/types/array.g4
 *
 * Example conceptual type:
 *
 *     Array<T, N>
 *
 * or whatever canonical Zamani type syntax is established by the type
 * specification.
 *
 * This file MUST NOT define array type syntax.
 *
 * ============================================================================
 * CRITICAL DISTINCTION: ARRAY LITERAL VS INDEXING
 * ============================================================================
 *
 * This file owns:
 *
 *     [a, b, c]
 *
 * The canonical indexing grammar owns:
 *
 *     values[i]
 *
 * Therefore this file MUST NOT define:
 *
 *     hdlIndexSuffix
 *     hdlIndexExpression
 *     array[i]
 *
 * Indexing remains a postfix-expression concern.
 *
 * The distinction is:
 *
 *     [a, b, c]
 *          ^
 *          array literal
 *
 *     values[i]
 *           ^
 *           indexing
 *
 * ============================================================================
 * CRITICAL DISTINCTION: ARRAY LITERAL VS MEMORY DIMENSIONS
 * ============================================================================
 *
 * A memory declaration such as:
 *
 *     memory data: Word [DEPTH];
 *
 * contains a MEMORY DIMENSION.
 *
 * That syntax remains owned by:
 *
 *     grammar/hdl/memories.g4
 *
 * It MUST NOT be moved into this file.
 *
 * Likewise:
 *
 *     signal data: Word [WIDTH];
 *
 * uses HDL/type/range syntax and is not an array literal.
 *
 * ============================================================================
 * LEXER AUTHORITY
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * It contains no lexer rules.
 *
 * It consumes the canonical Zamani lexer vocabulary:
 *
 *     ZamaniLexer
 *
 * Required punctuation:
 *
 *     LBRACKET
 *     RBRACKET
 *     COMMA
 *
 * No local aliases are permitted.
 *
 * Do NOT introduce:
 *
 *     LBRACK
 *     RBRACK
 *     ARRAY_COMMA
 *
 * or any equivalent duplicate token vocabulary.
 *
 * ============================================================================
 * EXPRESSION AUTHORITY
 * ============================================================================
 *
 * Array elements are HDL expressions:
 *
 *     hdlExpression
 *
 * This is intentional.
 *
 * The current canonical HDL parser defines its own HDL expression hierarchy
 * rooted at:
 *
 *     hdlExpression
 *
 * Therefore this delegate must use that existing boundary rather than
 * inventing another expression grammar or forcing a cyclic import into the
 * current HDL composition.
 *
 * The composition relationship is:
 *
 *     hdl.g4
 *         |
 *         +--> arrays.g4
 *                   |
 *                   +--> hdlExpression
 *
 * The delegating HDL grammar owns hdlExpression.
 *
 * This delegate consumes it.
 *
 * ============================================================================
 * NO SECOND EXPRESSION GRAMMAR
 * ============================================================================
 *
 * DO NOT define any of the following here:
 *
 *     expression
 *     hdlExpression
 *     hdlAssignmentExpression
 *     hdlConditionalExpression
 *     hdlLogicalOrExpression
 *     hdlAdditiveExpression
 *     hdlMultiplicativeExpression
 *     hdlUnaryExpression
 *     hdlPostfixExpression
 *     hdlPrimaryExpression
 *
 * Those rules remain owned by the canonical HDL expression hierarchy.
 *
 * ============================================================================
 * ARRAY SYNTAX CONTRACT
 * ============================================================================
 *
 * Canonical forms:
 *
 *     []
 *
 *     [a]
 *
 *     [a, b]
 *
 *     [a, b, c]
 *
 *     [[a, b], [c, d]]
 *
 *     [[[a]]]
 *
 *     [signal_a, signal_b]
 *
 *     [register_a + 1, register_b + 1]
 *
 *     [parameter_a, parameter_b * 2]
 *
 * Each element is an hdlExpression.
 *
 * ============================================================================
 * TRAILING COMMA POLICY
 * ============================================================================
 *
 * The previous inline hdlArrayLiteral in grammar/hdl/hdl.g4 accepted:
 *
 *     [a,]
 *
 * and:
 *
 *     [a, b,]
 *
 * while the canonical general expression-array grammar deliberately rejects
 * trailing commas.
 *
 * The HDL array delegate follows the canonical array-expression contract and
 * therefore DOES NOT accept a trailing comma.
 *
 * Accepted:
 *
 *     []
 *     [a]
 *     [a, b]
 *
 * Rejected:
 *
 *     [a,]
 *     [a, b,]
 *
 * This removes an accidental HDL-only grammar discrepancy.
 *
 * If Zamani later standardizes trailing commas for all array expressions,
 * that change must occur through the language specification and compatibility
 * process rather than by silently changing this delegate.
 *
 * ============================================================================
 * EMPTY ARRAYS
 * ============================================================================
 *
 * Empty arrays are syntactically valid:
 *
 *     []
 *
 * Their type, element type, inference rules, and whether a contextual type is
 * required are semantic/type-system concerns.
 *
 * This grammar does NOT attempt to infer:
 *
 *     Array<Unknown>
 *
 * or any other semantic type.
 *
 * ============================================================================
 * HOMOGENEITY / HETEROGENEITY
 * ============================================================================
 *
 * The grammar deliberately permits:
 *
 *     [a, b, c]
 *
 * without requiring all elements to have the same semantic type.
 *
 * Semantic analysis decides whether:
 *
 *     - elements must have a common type;
 *     - implicit conversion is permitted;
 *     - heterogeneous values are legal;
 *     - a contextual type is required;
 *     - the array represents a hardware aggregate;
 *     - the array represents ordinary data.
 *
 * The parser must not encode those decisions.
 *
 * ============================================================================
 * RECURSIVE ARRAYS
 * ============================================================================
 *
 * Nested arrays are supported through the existing hdlExpression hierarchy.
 *
 * Example:
 *
 *     [[a, b], [c, d]]
 *
 * is parsed as:
 *
 *     hdlArrayLiteral
 *         |
 *         +--> hdlExpression
 *         |       |
 *         |       +--> hdlArrayLiteral
 *         |
 *         +--> hdlExpression
 *                 |
 *                 +--> hdlArrayLiteral
 *
 * No artificial nesting limit is introduced.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This grammar imposes no semantic maximum on:
 *
 *     array elements;
 *     nested arrays;
 *     arrays per design;
 *     array dimensions;
 *     expression size;
 *     module count;
 *     signal count;
 *     register count;
 *     memory count;
 *     hardware block count;
 *     generated structures;
 *     device count;
 *     accelerator count;
 *     CPU count;
 *     GPU count;
 *     FPGA count;
 *     QPU count;
 *     node count;
 *     thread count;
 *     memory capacity;
 *     tensor rank.
 *
 * Repetition is expressed using ANTLR grammar repetition:
 *
 *     (COMMA hdlExpression)*
 *
 * This means the language has no artificial array-size ceiling.
 *
 * It does NOT mean physical machines possess infinite resources.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Array syntax describes a source-level value.
 *
 * It does not prescribe where that value must reside.
 *
 * Therefore:
 *
 *     [a, b, c]
 *
 * does NOT inherently mean:
 *
 *     CPU memory
 *     GPU memory
 *     FPGA BRAM
 *     ASIC SRAM
 *     distributed memory
 *     quantum memory
 *     contiguous memory
 *     stack memory
 *     heap memory
 *
 * The semantic/compiler/backend layers may determine an appropriate
 * realization from:
 *
 *     program semantics;
 *     type information;
 *     resource requirements;
 *     capabilities;
 *     constraints;
 *     preferences;
 *     target availability;
 *     optimization objectives.
 *
 * ============================================================================
 * HDL INTEGRATION
 * ============================================================================
 *
 * HDL arrays can contain expressions involving:
 *
 *     signals;
 *     registers;
 *     parameters;
 *     constants;
 *     function calls;
 *     arithmetic;
 *     logical operations;
 *     comparisons;
 *     conditional expressions;
 *     indexing;
 *     member access;
 *     nested arrays;
 *     future HDL expression extensions.
 *
 * This delegate does not need to know the individual expression categories.
 *
 * ============================================================================
 * HARDWARE INTEGRATION
 * ============================================================================
 *
 * Array syntax can participate in hardware descriptions such as:
 *
 *     [lane_a, lane_b, lane_c]
 *
 * or:
 *
 *     [register_a, register_b]
 *
 * The array does NOT imply a physical implementation.
 *
 * In particular, this grammar does not choose:
 *
 *     LUTs;
 *     BRAMs;
 *     DSPs;
 *     SRAM banks;
 *     register files;
 *     physical wires;
 *     physical buses;
 *     pins;
 *     routing paths;
 *     placement regions.
 *
 * ============================================================================
 * PARAMETERIZATION
 * ============================================================================
 *
 * Parameterized values are naturally supported because array elements are
 * expressions.
 *
 * Examples:
 *
 *     [base + i, base + j]
 *
 *     [WIDTH, DEPTH]
 *
 *     [lane_count, lane_count * 2]
 *
 * The parser does not evaluate these expressions.
 *
 * Evaluation belongs to semantic analysis/constant evaluation where required.
 *
 * ============================================================================
 * GENERATE INTEGRATION
 * ============================================================================
 *
 * A generate construct may use array expressions as part of its parameters,
 * conditions, bounds, or generated hardware intent.
 *
 * This file does not own generate syntax.
 *
 * Generate syntax remains owned by:
 *
 *     grammar/hdl/generate.g4
 *
 * The two integrate through hdlExpression.
 *
 * ============================================================================
 * SIGNAL / REGISTER INTEGRATION
 * ============================================================================
 *
 * Signal and register declarations may contain array-valued expressions where
 * the declaration grammar permits an initializer or expression.
 *
 * This file does not own signal or register declaration syntax.
 *
 * Ownership remains:
 *
 *     grammar/hdl/signals.g4
 *     grammar/hdl/registers.g4
 *
 * ============================================================================
 * MEMORY INTEGRATION
 * ============================================================================
 *
 * Memory declarations are distinct from array literals.
 *
 * Memory declaration:
 *
 *     memory data: Word [DEPTH];
 *
 * Array literal:
 *
 *     [value_a, value_b, value_c]
 *
 * The first is logical storage declaration syntax.
 *
 * The second is a source-level value.
 *
 * `grammar/hdl/memories.g4` remains authoritative for memory declarations.
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * An HDL array may contain classical expressions.
 *
 * The array syntax itself remains domain-neutral.
 *
 * Classical semantic interpretation occurs downstream.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * HDL array expressions may syntactically contain values associated with
 * quantum/hybrid computation when the surrounding semantic context permits it.
 *
 * Example:
 *
 *     [q0, q1]
 *
 * or:
 *
 *     [measurement_a, measurement_b]
 *
 * Whether those values represent:
 *
 *     qubit references;
 *     measurement results;
 *     handles;
 *     symbolic values;
 *     resource references;
 *     another semantic abstraction
 *
 * is decided by semantic analysis.
 *
 * This file MUST NOT:
 *
 *     - allocate qubits;
 *     - select physical qubits;
 *     - select a QPU;
 *     - create a quantum IR;
 *     - perform routing;
 *     - perform scheduling;
 *     - perform QEC;
 *     - perform noise analysis;
 *     - perform calibration;
 *     - select a vendor.
 *
 * The canonical quantum semantic boundary remains:
 *
 *     quantum::ir
 *
 * ============================================================================
 * HYBRID INTEGRATION
 * ============================================================================
 *
 * HDL array expressions may occur in a hybrid source program containing:
 *
 *     classical computation;
 *     quantum computation;
 *     hardware descriptions;
 *     resource requirements;
 *     execution policies.
 *
 * This delegate remains syntax-only and domain-neutral.
 *
 * ============================================================================
 * AI / DATA INTEGRATION
 * ============================================================================
 *
 * HDL arrays may carry values later interpreted as:
 *
 *     feature collections;
 *     tensor data;
 *     model parameters;
 *     accelerator inputs;
 *     structured data;
 *     streams.
 *
 * Tensor rank, shape, storage format, and accelerator mapping are not parser
 * concerns.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * An array expression does not imply:
 *
 *     local;
 *     replicated;
 *     sharded;
 *     partitioned;
 *     distributed;
 *     remote.
 *
 * Distribution semantics remain downstream.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * This file does not perform resource resolution.
 *
 * If an array contributes to a resource requirement, the surrounding semantic
 * layer may derive resource demand from the resulting AST.
 *
 * Examples of downstream concerns:
 *
 *     storage demand;
 *     bandwidth demand;
 *     accelerator demand;
 *     communication demand;
 *     synthesis cost.
 *
 * No physical resource limit belongs in this grammar.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser must expose enough structure for the frontend AST builder to
 * represent:
 *
 *     Array {
 *         elements: [Expression, ...],
 *         source_span: Span
 *     }
 *
 * The exact Rust AST type remains owned by:
 *
 *     src/frontend/ast/
 *
 * This grammar MUST NOT require a domain-specific AST such as:
 *
 *     HdlArray
 *     HardwareArray
 *     FpgaArray
 *     QuantumArray
 *     PhysicalSignalArray
 *
 * unless the repository's domain-neutral AST architecture explicitly defines
 * such a type as a semantic representation rather than a duplicate grammar
 * representation.
 *
 * The preferred representation remains the common array-expression node.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis owns:
 *
 *     - element type checking;
 *     - contextual type inference;
 *     - homogeneous/heterogeneous legality;
 *     - constant evaluation;
 *     - shape inference where applicable;
 *     - dimensional consistency where applicable;
 *     - resource analysis;
 *     - hardware interpretation;
 *     - quantum interpretation;
 *     - data interpretation;
 *     - target compatibility.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar produces no IR.
 *
 * Array values must lower through the repository's canonical semantic path.
 *
 * Conceptually:
 *
 *     hdlArrayLiteral
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic array/value representation
 *          |
 *          v
 *     canonical hardware/semantic IR
 *          |
 *          +--> optimization
 *          +--> scheduling
 *          +--> synthesis
 *          +--> placement
 *          +--> routing
 *          +--> target lowering
 *
 * Quantum-containing expressions continue through the canonical:
 *
 *     quantum::ir
 *
 * boundary where appropriate.
 *
 * ============================================================================
 * NO PHYSICAL ARRAY MODEL
 * ============================================================================
 *
 * This grammar MUST NOT introduce syntax for:
 *
 *     physical array address;
 *     physical bus number;
 *     physical bank;
 *     FPGA BRAM number;
 *     ASIC memory instance;
 *     GPU memory address;
 *     CPU register number;
 *     physical qubit number;
 *     hardware lane identifier.
 *
 * Such information belongs to explicit target/deployment representations, not
 * portable core HDL array syntax.
 *
 * ============================================================================
 * NO HARD-CODED LIMITS
 * ============================================================================
 *
 * Forbidden universal language limits include:
 *
 *     MAX_ARRAY_ELEMENTS
 *     MAX_ARRAY_DIMENSIONS
 *     MAX_ARRAY_DEPTH
 *     MAX_ARRAY_WIDTH
 *     MAX_ARRAY_COUNT
 *     MAX_MEMORY
 *     MAX_REGISTER_WIDTH
 *     MAX_PORTS
 *     MAX_SIGNALS
 *     MAX_MODULES
 *     MAX_DEVICES
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_QUBITS
 *     MAX_NODES
 *
 * None are defined here.
 *
 * A numeric literal such as:
 *
 *     [1, 2, 3]
 *
 * is program data.
 *
 * It must never become a parser implementation limit.
 *
 * ============================================================================
 * OPERATIONAL RESOURCE LIMITS
 * ============================================================================
 *
 * A compiler may impose configurable operational safeguards against:
 *
 *     memory exhaustion;
 *     parser exhaustion;
 *     pathological nesting;
 *     denial-of-service input;
 *     excessive generated designs.
 *
 * Such limits are implementation policies.
 *
 * They must NOT be represented by this grammar as semantic array limits.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * For a fixed:
 *
 *     source;
 *     lexer version;
 *     parser version;
 *     grammar version;
 *     dialect configuration;
 *
 * parsing must be deterministic.
 *
 * This file performs no:
 *
 *     filesystem access;
 *     network access;
 *     hardware discovery;
 *     randomness;
 *     environment inspection;
 *     runtime execution.
 *
 * ============================================================================
 * ERROR HANDLING
 * ============================================================================
 *
 * The grammar itself contains no executable error-recovery actions.
 *
 * The parser implementation must preserve normal ANTLR diagnostics and source
 * positions.
 *
 * Examples:
 *
 *     [a,]
 *
 * should produce a normal syntax diagnostic because the array contract
 * requires an expression after every comma.
 *
 * Likewise:
 *
 *     [a b]
 *
 * must not be silently converted into two array elements.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * This file:
 *
 *     - contains no embedded Rust;
 *     - contains no semantic actions;
 *     - contains no predicates requiring unsafe code;
 *     - performs no I/O;
 *     - performs no command execution;
 *     - performs no hardware access.
 *
 * Rust infrastructure integrating this grammar must remain:
 *
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust
 *     no unsafe
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * This delegate intentionally preserves the public rule name:
 *
 *     hdlArrayLiteral
 *
 * so that the existing HDL expression hierarchy can transition from the
 * inline implementation to the delegated implementation without changing the
 * semantic concept.
 *
 * The important compatibility change is removal of the accidental trailing
 * comma acceptance from the old inline rule.
 *
 * If compatibility policy requires accepting trailing commas in a future
 * language version, that must be specified explicitly and applied consistently
 * across array-expression syntax.
 *
 * ============================================================================
 * INTEGRATION WITH grammar/hdl/hdl.g4
 * ============================================================================
 *
 * `grammar/hdl/hdl.g4` is the composition root.
 *
 * It must import this delegate:
 *
 *     import Arrays;
 *
 * The existing inline rule:
 *
 *     hdlArrayLiteral
 *         : LBRACKET
 *           (
 *               hdlExpression
 *               (COMMA hdlExpression)*
 *               COMMA?
 *           )?
 *           RBRACKET
 *         ;
 *
 * MUST be removed from hdl.g4.
 *
 * The existing:
 *
 *     hdlPrimaryExpression
 *         : ...
 *         | hdlArrayLiteral
 *         ;
 *
 * remains valid and now resolves to this delegate's rule.
 *
 * Do not create a wrapper with a second name such as:
 *
 *     hdlArrayExpression
 *
 * unless the complete HDL specification intentionally changes the public
 * parser contract.
 *
 * ============================================================================
 * INTEGRATION WITH grammar/expressions/arrays.g4
 * ============================================================================
 *
 * `grammar/expressions/arrays.g4` remains the canonical general-language
 * array-expression grammar.
 *
 * This file does NOT import it directly because the existing HDL expression
 * hierarchy is the parser expression boundary used by hdl.g4.
 *
 * The two grammars share the same conceptual contract:
 *
 *     [expression, expression, ...]
 *
 * but have different ownership boundaries:
 *
 *     expressions/arrays.g4
 *         general Zamani expressions
 *
 *     hdl/arrays.g4
 *         HDL parser expression composition
 *
 * They MUST NOT both be composed into the same parser under duplicate rule
 * names.
 *
 * ============================================================================
 * INTEGRATION WITH grammar/types/array.g4
 * ============================================================================
 *
 * `grammar/types/array.g4` owns array TYPE syntax.
 *
 * This file owns array VALUE syntax.
 *
 * No imports or duplicate type rules are required.
 *
 * ============================================================================
 * INTEGRATION WITH grammar/hdl/memories.g4
 * ============================================================================
 *
 * `memories.g4` owns memory declarations and memory dimensions.
 *
 * This file must not redefine:
 *
 *     hdlMemoryDeclaration
 *     hdlMemoryDimension
 *
 * A memory dimension such as:
 *
 *     [DEPTH]
 *
 * is not an array literal.
 *
 * ============================================================================
 * INTEGRATION WITH grammar/hdl/signals.g4
 * ============================================================================
 *
 * Signal declarations remain owned by signals.g4.
 *
 * Array-valued signal initializers may consume:
 *
 *     hdlExpression
 *
 * and therefore naturally reach:
 *
 *     hdlArrayLiteral
 *
 * through the canonical HDL expression hierarchy.
 *
 * ============================================================================
 * INTEGRATION WITH grammar/hdl/registers.g4
 * ============================================================================
 *
 * Register declarations remain owned by registers.g4.
 *
 * Register initializers may use array expressions where their semantic type
 * permits them.
 *
 * ============================================================================
 * INTEGRATION WITH grammar/hdl/generate.g4
 * ============================================================================
 *
 * Generate syntax remains owned by generate.g4.
 *
 * Array expressions may occur in generate parameters, conditions, or
 * expression-bearing positions exposed by that grammar.
 *
 * ============================================================================
 * INTEGRATION WITH grammar/hardware/
 * ============================================================================
 *
 * Hardware target grammars may consume semantic information derived from HDL
 * arrays.
 *
 * They must not modify this grammar to encode target-specific array limits.
 *
 * ============================================================================
 * INTEGRATION WITH grammar/resources/
 * ============================================================================
 *
 * Resource requirements derived from array values belong to the resource
 * semantic layer.
 *
 * This grammar only provides the source structure.
 *
 * ============================================================================
 * INTEGRATION WITH AST / SEMANTICS / IR
 * ============================================================================
 *
 * Required pipeline:
 *
 *     source
 *       |
 *       v
 *     canonical lexer
 *       |
 *       v
 *     hdl.g4
 *       |
 *       v
 *     hdlArrayLiteral
 *       |
 *       v
 *     frontend AST
 *       |
 *       v
 *     semantic analysis
 *       |
 *       v
 *     canonical semantic/hardware representation
 *       |
 *       +-------------------------------+
 *       |               |               |
 *       v               v               v
 *   classical       hardware        quantum
 *                                       |
 *                                       v
 *                                  quantum::ir
 *
 * No array-specific IR is introduced here.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * This file is complete only when tests cover:
 *
 * POSITIVE:
 *
 *     []
 *     [a]
 *     [a, b]
 *     [a, b, c]
 *     [a + b, c * d]
 *     [[a, b], [c, d]]
 *     [[[a]]]
 *     [signal_a, register_b]
 *     [parameter_a, parameter_b * 2]
 *     [call(x), member.field]
 *
 * NEGATIVE:
 *
 *     [a,]
 *     [a, b,]
 *     [a b]
 *     [, a]
 *     [a,,b]
 *     [a, ,b]
 *     [)
 *     [a
 *
 * BOUNDARY:
 *
 *     empty array;
 *     one element;
 *     many elements;
 *     nested arrays;
 *     expressions with nested calls;
 *     expressions with indexing;
 *     expressions with member access;
 *     expressions containing parameters.
 *
 * SCALABILITY:
 *
 *     large symbolic element sequences;
 *     deeply nested source structures where permitted by the implementation;
 *     generated array-valued expressions;
 *     large numeric literals;
 *     symbolic expressions rather than hard-coded machine limits.
 *
 * CROSS-DOMAIN:
 *
 *     HDL + classical;
 *     HDL + quantum;
 *     HDL + hybrid;
 *     HDL + hardware;
 *     HDL + resources;
 *     HDL + distributed;
 *     HDL + AI/data.
 *
 * DETERMINISM:
 *
 *     repeated parsing of identical source produces equivalent parse results.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains:
 *
 *     no maximum element count;
 *     no maximum dimension count;
 *     no maximum nesting depth;
 *     no hardware capacity;
 *     no device identifier;
 *     no physical memory size;
 *     no fixed bus width;
 *     no fixed register width;
 *     no fixed FPGA resource count;
 *     no fixed ASIC resource count;
 *     no fixed QPU resource count.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] hdlArrayLiteral has one owner;
 *     [x] no lexer rules are duplicated;
 *     [x] hdlExpression is reused;
 *     [x] array types remain outside this file;
 *     [x] indexing remains outside this file;
 *     [x] memory dimensions remain outside this file;
 *     [x] nested arrays are supported;
 *     [x] empty arrays are supported;
 *     [x] arbitrary element counts are supported;
 *     [x] no artificial hardware limits exist;
 *     [x] no physical hardware is selected;
 *     [x] no vendor syntax is required;
 *     [x] no quantum IR is created;
 *     [x] quantum::ir remains canonical;
 *     [x] AST integration is defined;
 *     [x] semantic integration is defined;
 *     [x] IR integration is defined;
 *     [x] resource integration is defined;
 *     [x] diagnostics are defined;
 *     [x] compatibility behavior is defined;
 *     [x] deterministic parsing is preserved;
 *     [x] safe Rust 1.97/1.97.1 remains sufficient;
 *     [x] no unsafe Rust is required.
 *
 * ============================================================================
 * PUBLIC RULES
 * ============================================================================
 *
 * The only public integration rule owned by this file is:
 *
 *     hdlArrayLiteral
 *
 * The element-list rule is an implementation detail of this delegate.
 *
 * ============================================================================
 */

parser grammar Arrays;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * HDL ARRAY LITERAL
 * ============================================================================
 *
 * Public integration rule.
 *
 * Examples:
 *
 *     []
 *     [a]
 *     [a, b]
 *     [a, b, c]
 *     [[a, b], [c, d]]
 *
 * No trailing comma is accepted.
 *
 * ============================================================================
 */

hdlArrayLiteral
    : LBRACKET
      hdlArrayElementList?
      RBRACKET
    ;


/*
 * ============================================================================
 * ARRAY ELEMENT LIST
 * ============================================================================
 *
 * One or more HDL expressions separated by commas.
 *
 * The repetition operator introduces no language-level finite maximum.
 *
 * ============================================================================
 */

hdlArrayElementList
    : hdlExpression
      (
          COMMA
          hdlExpression
      )*
    ;
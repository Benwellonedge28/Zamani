/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/statements/bindings.g4
 *
 * STATUS
 * ------
 * CANONICAL REUSABLE BINDING GRAMMAR
 *
 * GRAMMAR TECHNOLOGY
 * ------------------
 * ANTLR4 parser grammar
 *
 * IMPLEMENTATION BASELINE
 * -----------------------
 * Rust 1.97 / Rust 1.97.1
 * Rust edition 2021
 * Safe Rust only.
 *
 * The grammar itself contains:
 *
 *     - no embedded Rust;
 *     - no semantic predicates;
 *     - no actions;
 *     - no filesystem access;
 *     - no networking;
 *     - no hardware discovery;
 *     - no runtime execution;
 *     - no target selection;
 *     - no unsafe Rust.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the REUSABLE SOURCE-LEVEL BINDING ABSTRACTION.
 *
 * A binding associates a source-level name or binding pattern with a value,
 * type, parameter, iterator element, match value, resource description, or
 * another semantic entity.
 *
 * This grammar deliberately separates:
 *
 *     BINDING
 *
 * from:
 *
 *     DECLARATION
 *     ASSIGNMENT
 *     PARAMETER
 *     LOOP
 *     MATCH
 *     TYPE
 *     EXPRESSION
 *
 * Those surrounding constructs own their respective delimiters and semantics.
 *
 * The binding grammar supplies the reusable binding shape.
 *
 * ============================================================================
 * ARCHITECTURAL PIPELINE
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     binding grammar                    <-- THIS FILE
 *          |
 *          v
 *     parse tree
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     structural validation
 *          |
 *          v
 *     name / type / ownership analysis
 *          |
 *          +-----------------------------+
 *          |                             |
 *          v                             v
 *     resource/capability             effect analysis
 *          |                             |
 *          +-------------+---------------+
 *                        |
 *                        v
 *               canonical semantic model
 *                        |
 *             +----------+-----------+
 *             |          |           |
 *             v          v           v
 *         classical   quantum::ir   HDL/hardware
 *             |          |           |
 *             +----------+-----------+
 *                        |
 *                        v
 *                 optimization
 *                        |
 *                 routing/scheduling
 *                        |
 *                 resilience/QEC/ZQN
 *                        |
 *                       HAL
 *                        |
 *                 target realization
 *
 * This file remains entirely upstream of target realization.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS
 * --------------
 *
 *     binding
 *     bindingPattern
 *     bindingIdentifier
 *     bindingWildcard
 *     bindingTuple
 *     bindingArray
 *     bindingReference
 *     bindingRest
 *     bindingList
 *     bindingModifier
 *
 * THIS FILE DOES NOT OWN
 * ----------------------
 *
 *     let / var / const declaration keywords as declaration constructs
 *     assignment
 *     assignment expressions
 *     expression precedence
 *     types
 *     literals
 *     general pattern matching
 *     function declarations
 *     function parameter lists
 *     loops
 *     match statements
 *     ownership semantics
 *     borrowing semantics
 *     mutability semantics
 *     lifetime semantics
 *     resource allocation
 *     capability resolution
 *     quantum semantics
 *     HDL semantics
 *     hardware topology
 *     scheduling
 *     routing
 *     QEC
 *     ZQN
 *     HAL
 *     runtime behavior
 *
 * Surrounding grammars own their syntactic framing.
 *
 * ============================================================================
 * WHY THIS FILE IS NECESSARY
 * ============================================================================
 *
 * The current repository has several places that need binding syntax:
 *
 *     grammar/declarations/variables.g4
 *     grammar/statements/loops.g4
 *     grammar/functions/parameters.g4
 *     grammar/statements/pattern-matching.g4
 *     future resource/capability declarations
 *     future data destructuring
 *     future distributed bindings
 *     future AI/data bindings
 *
 * Without a common binding abstraction, each subsystem can gradually invent
 * incompatible forms.
 *
 * This file prevents that duplication.
 *
 * The surrounding construct decides what the binding means.
 *
 * For example:
 *
 *     let x = value;
 *
 * uses a binding inside a declaration.
 *
 *     for x in values { ... }
 *
 * uses a binding inside iteration.
 *
 *     fn f(x: T) { ... }
 *
 * uses a binding inside a parameter.
 *
 *     match value {
 *         x => ...
 *     }
 *
 * uses a binding inside a pattern.
 *
 * The binding shape remains reusable.
 *
 * ============================================================================
 * TARGET INDEPENDENCE
 * ============================================================================
 *
 * Binding syntax is completely target-independent.
 *
 * It MUST NOT distinguish:
 *
 *     cpuBinding
 *     gpuBinding
 *     fpgaBinding
 *     qpuBinding
 *     acceleratorBinding
 *     physicalQubitBinding
 *     registerBinding
 *     memoryBankBinding
 *     deviceBinding
 *     nodeBinding
 *
 * A source identifier may have any semantic meaning later.
 *
 * For example:
 *
 *     q
 *     gpu
 *     accelerator
 *     device
 *     node
 *     memory
 *     qubit
 *
 * are ordinary source-level names.
 *
 * The binding grammar does not determine what they represent.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Bindings are part of the source-level semantic model.
 *
 * They therefore MUST remain valid independently of eventual realization on:
 *
 *     tiny embedded systems
 *     CPUs
 *     multicore CPUs
 *     GPUs
 *     FPGAs
 *     ASICs
 *     QPUs
 *     quantum simulators
 *     accelerators
 *     clusters
 *     HPC systems
 *     distributed systems
 *     cloud systems
 *     future architectures
 *
 * The grammar introduces no machine-dependent limit.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There are NO grammar constants for:
 *
 *     MAX_BINDINGS
 *     MAX_VARIABLES
 *     MAX_PARAMETERS
 *     MAX_PATTERN_DEPTH
 *     MAX_TUPLE_ARITY
 *     MAX_ARRAY_BINDINGS
 *     MAX_NODES
 *     MAX_THREADS
 *     MAX_CORES
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_QUBITS
 *     MAX_MEMORY
 *     MAX_DEVICES
 *     MAX_ACCELERATORS
 *
 * Repetition uses ANTLR repetition operators.
 *
 * Recursive structures are used where source structure is recursively defined.
 *
 * "Infinity" means that this grammar introduces no artificial finite language
 * limit. Actual limits remain implementation, compiler, runtime, deployment,
 * and available-resource constraints.
 *
 * ============================================================================
 * LEXICAL AUTHORITY
 * ============================================================================
 *
 * The canonical production lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * The lexer composes:
 *
 *     grammar/lexer/tokens.g4
 *
 * Parser grammars consume:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * This file MUST NOT define lexical rules.
 *
 * In particular, it MUST NOT redefine:
 *
 *     IDENTIFIER
 *     UNDERSCORE
 *     MUT
 *     AMPERSAND
 *     COMMA
 *     LPAREN
 *     RPAREN
 *     LBRACKET
 *     RBRACKET
 *     DOT_DOT
 *     DOT_DOT_EQ
 *
 * ============================================================================
 * IMPORTS
 * ============================================================================
 *
 * This grammar intentionally imports only the canonical name grammar.
 *
 * General expressions and types are NOT imported here because a binding is
 * intentionally independent of declaration-level initializers and type
 * annotations.
 *
 * Declaration grammars, parameter grammars, and loop grammars compose:
 *
 *     binding
 *
 * with their own:
 *
 *     typeExpression
 *     expression
 *     declaration terminator
 *     parameter delimiter
 *     loop delimiter
 *
 * This avoids creating a dependency cycle:
 *
 *     binding -> expression -> assignment -> statement -> binding
 *
 * The binding layer remains structurally foundational.
 *
 * ============================================================================
 */

parser grammar Bindings;

options {
    tokenVocab = ZamaniLexer;
}

import Names;


/*
 * ============================================================================
 * PUBLIC ROOT
 * ============================================================================
 *
 * `binding` is the only public root required by consumers.
 *
 * A consumer should import this grammar and use:
 *
 *     binding
 *
 * rather than reproducing binding syntax.
 *
 * ============================================================================
 */

binding
    : bindingModifier* bindingPattern
    ;


/*
 * ============================================================================
 * BINDING MODIFIERS
 * ============================================================================
 *
 * `mut` expresses source-level mutability intent.
 *
 * It does NOT mean:
 *
 *     mutable hardware
 *     mutable physical memory
 *     mutable register
 *     mutable qubit
 *
 * Those meanings are semantic and domain-specific.
 *
 * Repetition is intentionally permitted here so future source-level modifier
 * composition can be introduced without changing the binding root.
 *
 * Semantic validation MUST reject modifier combinations that are invalid for
 * a particular binding context.
 *
 * For example, if the language specification eventually restricts `mut` to
 * one occurrence, that is a semantic/structural validation rule rather than a
 * machine-size restriction.
 */
bindingModifier
    : MUT
    ;


/*
 * ============================================================================
 * BINDING PATTERN
 * ============================================================================
 *
 * A binding pattern describes which source-level names receive values.
 *
 * It deliberately supports structural destructuring without depending on
 * value semantics.
 *
 * Examples:
 *
 *     x
 *     _
 *     (x, y)
 *     [x, y]
 *     [head, ...tail]
 *     &x
 *     &mut x
 *
 * Whether a particular pattern is legal in a particular context is a semantic
 * question.
 *
 * For example:
 *
 *     let (x, y) = value;
 *
 * may require the RHS type to be destructurable.
 *
 * The grammar does not make that determination.
 */
bindingPattern
    : bindingIdentifier
    | bindingWildcard
    | bindingTuple
    | bindingArray
    | bindingReference
    ;


/*
 * ============================================================================
 * SIMPLE IDENTIFIER BINDING
 * ============================================================================
 *
 * The lexical identifier is owned by the canonical lexer.
 *
 * Name structure is owned by the canonical Names grammar.
 *
 * This rule therefore does not redefine identifier syntax.
 */
bindingIdentifier
    : identifier
    ;


/*
 * ============================================================================
 * WILDCARD BINDING
 * ============================================================================
 *
 * `_` represents an intentionally ignored binding.
 *
 * It does not introduce a source-level name.
 *
 * Semantic analysis determines whether an ignored value is legal and whether
 * the surrounding construct requires a binding.
 */
bindingWildcard
    : UNDERSCORE
    ;


/*
 * ============================================================================
 * TUPLE BINDING
 * ============================================================================
 *
 * Examples:
 *
 *     (x, y)
 *     (x, y, z)
 *     ((x, y), z)
 *
 * The grammar places no fixed arity limit.
 *
 * A singleton parenthesized binding is deliberately NOT treated as a tuple:
 *
 *     (x)
 *
 * remains structurally distinguishable from:
 *
 *     (x,)
 *
 * The trailing-comma distinction is therefore preserved for semantic analysis.
 */
bindingTuple
    : LPAREN bindingPatternList? COMMA? RPAREN
    ;


/*
 * ============================================================================
 * ARRAY / SEQUENCE BINDING
 * ============================================================================
 *
 * Examples:
 *
 *     [x, y]
 *     [head, ...tail]
 *     []
 *
 * There is no fixed sequence length.
 *
 * Array/sequence semantics are determined downstream.
 */
bindingArray
    : LBRACKET bindingArrayElements? RBRACKET
    ;


/*
 * ============================================================================
 * ARRAY ELEMENTS
 * ============================================================================
 *
 * A binding list may contain ordinary bindings and at most one rest binding
 * syntactically.
 *
 * The grammar intentionally does not attempt to encode all semantic rules for
 * rest placement. Those rules are validated downstream.
 *
 * Examples:
 *
 *     [a, b]
 *     [a, b, ...rest]
 *     [a, ...rest, b]
 *
 * The last form may be rejected by semantic validation if the language
 * specifies that a rest binding must be terminal.
 */
bindingArrayElements
    : bindingArrayElement (COMMA bindingArrayElement)* COMMA?
    ;


bindingArrayElement
    : bindingPattern
    | bindingRest
    ;


/*
 * ============================================================================
 * REST BINDING
 * ============================================================================
 *
 * `...name` captures an unbounded remainder of a structural sequence.
 *
 * The grammar does not impose a maximum sequence length.
 *
 * This is intentionally semantic rather than hardware-specific.
 */
bindingRest
    : DOT_DOT DOT_DOT? bindingPattern
    ;


/*
 * ============================================================================
 * REFERENCE BINDING
 * ============================================================================
 *
 * Reference binding describes source-level reference/destructuring intent.
 *
 * Examples:
 *
 *     &x
 *     &mut x
 *
 * This syntax does not imply a physical address.
 *
 * It does not select:
 *
 *     memory bank
 *     register
 *     device memory
 *     GPU memory
 *     FPGA memory
 *     QPU memory
 *
 * Those decisions belong downstream.
 *
 * `mut` remains a source-level semantic modifier.
 */
bindingReference
    : AMPERSAND bindingModifier* bindingPattern
    ;


/*
 * ============================================================================
 * REUSABLE BINDING LIST
 * ============================================================================
 *
 * Used by consumers such as:
 *
 *     tuple bindings
 *     future destructuring declarations
 *     future binding-aware constructs
 *
 * No finite list length is imposed.
 */
bindingPatternList
    : bindingPattern (COMMA bindingPattern)*
    ;


/*
 * ============================================================================
 * SEMANTIC BOUNDARY
 * ============================================================================
 *
 * This grammar deliberately accepts structure and does NOT perform semantic
 * validation.
 *
 * Semantic analysis owns:
 *
 *     - scope;
 *     - declaration identity;
 *     - shadowing;
 *     - duplicate names;
 *     - wildcard behavior;
 *     - mutability;
 *     - ownership;
 *     - borrowing;
 *     - lifetimes;
 *     - type compatibility;
 *     - destructuring validity;
 *     - irrefutability;
 *     - exhaustiveness;
 *     - move/copy semantics;
 *     - linear/affine semantics;
 *     - effect requirements;
 *     - capability requirements;
 *     - resource requirements;
 *     - quantum/classical boundaries;
 *     - HDL semantics;
 *     - distributed semantics;
 *     - accelerator semantics.
 *
 * Examples that may be syntactically valid but semantically invalid include:
 *
 *     (x, x)
 *     let (x, y) = scalar;
 *     for (x, y) in scalar { ... }
 *     match value { (x, y) => ... }
 *
 * The parser must not encode those type- or context-dependent restrictions.
 *
 * ============================================================================
 * NAME RESOLUTION
 * ============================================================================
 *
 * The binding grammar creates source-level name structure only.
 *
 * It does NOT resolve:
 *
 *     x
 *     module::x
 *     type::x
 *
 * to declarations.
 *
 * Name resolution occurs after parsing.
 *
 * ============================================================================
 * OWNERSHIP / BORROWING
 * ============================================================================
 *
 * A binding is not itself an ownership operation.
 *
 * For example:
 *
 *     let x = value;
 *
 * may later be interpreted as:
 *
 *     move
 *     copy
 *     borrow
 *     reference
 *     shared ownership
 *     linear consumption
 *     affine consumption
 *     domain-specific transfer
 *
 * depending on the semantic type and context.
 *
 * The grammar must not choose among these.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * A binding may eventually bind:
 *
 *     ordinary data
 *     tensors
 *     streams
 *     resources
 *     capabilities
 *     quantum values
 *     measurement results
 *     hardware abstractions
 *     distributed handles
 *     accelerator values
 *     AI models
 *     data pipelines
 *     future domain values
 *
 * The grammar does not distinguish them.
 *
 * For example:
 *
 *     let q = quantum_value;
 *
 * remains structurally identical to:
 *
 *     let x = classical_value;
 *
 * Resource discovery and capability negotiation occur downstream.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum bindings remain ordinary bindings.
 *
 * Examples:
 *
 *     let q = allocate_qubit();
 *     let result = measure(q);
 *     let (a, b) = measure_pair();
 *
 * The grammar MUST NOT:
 *
 *     - enumerate qubits;
 *     - enumerate gates;
 *     - assign physical qubit identifiers;
 *     - impose a qubit maximum;
 *     - encode topology;
 *     - encode calibration;
 *     - perform routing;
 *     - perform scheduling;
 *     - perform QEC;
 *     - implement ZQN;
 *     - select a QPU.
 *
 * The downstream path remains:
 *
 *     binding
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic analysis
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
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * The same binding syntax may appear around:
 *
 *     signal values
 *     registers
 *     ports
 *     memories
 *     accelerator values
 *     hardware configuration values
 *
 * The grammar MUST NOT encode:
 *
 *     bus widths
 *     register counts
 *     physical addresses
 *     clock frequencies
 *     FPGA resource counts
 *     ASIC dimensions
 *     device identifiers
 *     topology sizes
 *
 * ============================================================================
 * DISTRIBUTED / HPC INTEGRATION
 * ============================================================================
 *
 * Binding syntax remains identical for:
 *
 *     local values
 *     remote values
 *     distributed handles
 *     tasks
 *     actors
 *     channels
 *     services
 *     cluster resources
 *
 * The binding grammar never assigns a value to a particular node.
 *
 * ============================================================================
 * AI / DATA INTEGRATION
 * ============================================================================
 *
 * The same syntax can bind:
 *
 *     tensors
 *     datasets
 *     models
 *     parameters
 *     gradients
 *     streams
 *     agents
 *     inference results
 *
 * The grammar does not encode a framework or accelerator API.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Binding parsing depends only on:
 *
 *     source token sequence
 *     grammar version
 *     imported grammar versions
 *
 * It MUST NOT depend on:
 *
 *     time
 *     randomness
 *     environment variables
 *     filesystem state
 *     network state
 *     hardware state
 *     runtime state
 *     resource availability
 *
 * ============================================================================
 * DIAGNOSTIC BOUNDARY
 * ============================================================================
 *
 * Syntax diagnostics belong to the parser.
 *
 * Examples:
 *
 *     let ();
 *     let (, x);
 *     let [,...x];
 *     let ...;
 *     let &;
 *
 * Semantic diagnostics belong downstream.
 *
 * Examples:
 *
 *     duplicate binding names
 *     invalid rest placement
 *     binding immutable value as mutable
 *     destructuring incompatible type
 *     illegal linear binding
 *     illegal quantum binding
 *     unavailable capability
 *     unavailable resource
 *
 * ============================================================================
 * SOURCE-SPAN CONTRACT
 * ============================================================================
 *
 * Every binding parse context preserves the token interval from its first
 * consumed token through its final consumed token.
 *
 * AST construction MUST preserve:
 *
 *     binding span
 *     pattern span
 *     identifier span
 *     modifier span
 *     rest span
 *     reference span
 *     child-binding ordering
 *
 * This grammar does not construct spans itself.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar must map into the repository's existing domain-neutral AST
 * binding/pattern structures.
 *
 * It MUST NOT introduce domain-specific AST nodes such as:
 *
 *     QuantumBinding
 *     GPUBinding
 *     FPGARegisterBinding
 *     QPUBinding
 *     HardwareBinding
 *
 * A binding is syntax.
 *
 * Its semantic interpretation is determined after parsing.
 *
 * If the existing frontend AST lacks a reusable binding-pattern representation,
 * that AST gap must be resolved in the AST layer rather than by adding semantic
 * behavior to this grammar.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar produces NO IR.
 *
 * Binding semantics may participate in:
 *
 *     classical IR
 *     quantum::ir
 *     HDL/hardware IR
 *     distributed IR
 *     accelerator IR
 *     future semantic representations
 *
 * The grammar must not create a binding-specific IR.
 *
 * ============================================================================
 * COMPILER CONTRACT
 * ============================================================================
 *
 * Compiler lowering may transform bindings into:
 *
 *     SSA values
 *     storage locations
 *     references
 *     dataflow edges
 *     tuple projections
 *     destructuring operations
 *     resource handles
 *     distributed handles
 *     quantum semantic values
 *
 * Such lowering is downstream.
 *
 * Binding syntax itself must remain unchanged.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime systems determine how the semantic binding is materialized.
 *
 * Possible realizations include:
 *
 *     registers
 *     stack storage
 *     heap storage
 *     memory regions
 *     GPU memory
 *     FPGA storage
 *     distributed memory
 *     quantum runtime state
 *     simulator state
 *     accelerator storage
 *
 * The grammar does not select any of these.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing simple bindings remain valid:
 *
 *     x
 *
 * Existing mutable source-level binding form remains valid:
 *
 *     mut x
 *
 * Existing variable declaration syntax remains owned by:
 *
 *     grammar/declarations/variables.g4
 *
 * Therefore this file MUST NOT replace the `let` / `var` declaration framing.
 *
 * Instead:
 *
 *     variables.g4
 *          |
 *          v
 *     binding
 *          |
 *          +--> optional declaration type
 *          +--> optional initializer
 *          +--> declaration terminator
 *
 * The migration must preserve existing source compatibility.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * UPSTREAM
 * --------
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/lexer/*
 *     grammar/core/names.g4
 *
 * DOWNSTREAM DIRECT CONSUMERS
 * ---------------------------
 *
 *     grammar/declarations/variables.g4
 *     grammar/functions/parameters.g4
 *     grammar/statements/loops.g4
 *     grammar/statements/pattern-matching.g4
 *
 * FUTURE CONSUMERS
 * ----------------
 *
 *     grammar/resources/*
 *     grammar/hardware/*
 *     grammar/distributed/*
 *     grammar/ai/*
 *     grammar/data/*
 *     grammar/networking/*
 *     grammar/interoperability/*
 *
 * SEMANTIC CONSUMERS
 * ------------------
 *
 *     name resolution
 *     type checking
 *     ownership analysis
 *     borrow checking
 *     effect analysis
 *     capability analysis
 *     resource analysis
 *
 * IR CONSUMERS
 * ------------
 *
 *     classical semantic representation
 *     quantum::ir
 *     HDL/hardware representation
 *     distributed representation
 *     accelerator representation
 *
 * BACKEND CONSUMERS
 * -----------------
 *
 *     optimization
 *     routing
 *     scheduling
 *     resilience
 *     QEC
 *     ZQN
 *     HAL
 *     target realization
 *
 * ============================================================================
 * INTEGRATION RULE
 * ============================================================================
 *
 * Consumers MUST import and use:
 *
 *     binding
 *
 * rather than reproducing:
 *
 *     IDENTIFIER
 *     MUT IDENTIFIER
 *     tuple binding
 *     array binding
 *     reference binding
 *     rest binding
 *
 * locally.
 *
 * This creates one source of truth.
 *
 * ============================================================================
 * IMPORTANT MIGRATION RULE
 * ============================================================================
 *
 * This file does NOT immediately delete or silently override existing
 * declarations.
 *
 * The following files currently own overlapping concepts:
 *
 *     grammar/declarations/variables.g4
 *     grammar/functions/parameters.g4
 *     grammar/statements/loops.g4
 *     grammar/statements/pattern-matching.g4
 *
 * Integration must proceed in this order:
 *
 *     1. Add Bindings grammar.
 *
 *     2. Validate Bindings independently.
 *
 *     3. Import Bindings into each direct consumer.
 *
 *     4. Replace only duplicated binding productions.
 *
 *     5. Preserve surrounding delimiters and declaration syntax.
 *
 *     6. Remove the duplicated productions.
 *
 *     7. Add conformance tests.
 *
 *     8. Verify generated parser rule names.
 *
 *     9. Verify AST mappings.
 *
 *     10. Verify semantic analysis.
 *
 *     11. Verify Rust frontend compatibility.
 *
 * No consumer should simultaneously maintain a second authoritative binding
 * grammar.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * The feature is not complete merely because ANTLR accepts this file.
 *
 * Required lexical tests:
 *
 *     x
 *     _
 *     mut
 *     & 
 *     ...
 *
 * Required positive binding tests:
 *
 *     x
 *     mut x
 *     _
 *     (x, y)
 *     (x, (y, z))
 *     [x, y]
 *     [head, ...tail]
 *     &x
 *     &mut x
 *
 * Required negative syntax tests:
 *
 *     ()
 *     (, x)
 *     [,...x]
 *     ...
 *     &
 *
 * The exact status of `()` depends on whether unit-like structural patterns
 * become part of the canonical binding specification; tests must follow the
 * specification rather than compiler convenience.
 *
 * Required contextual integration tests:
 *
 *     let x = value;
 *     let mut x = value;
 *     let (x, y) = value;
 *     let [head, ...tail] = value;
 *     for x in values { ... }
 *     fn f(x: T) { ... }
 *     fn f(mut x: T) { ... }
 *     match value { x => ... }
 *
 * Required semantic tests:
 *
 *     duplicate names
 *     immutable/mutable conflicts
 *     invalid destructuring
 *     invalid reference binding
 *     invalid rest placement
 *     type mismatch
 *     ownership violations
 *     borrowing violations
 *     linear/affine violations
 *
 * Required scalability tests:
 *
 *     deeply nested bindings
 *     long binding lists
 *     large destructuring patterns
 *     long identifiers
 *     many independent bindings
 *
 * No test may establish an artificial hardware limit.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no:
 *
 *     MAX_*
 *     fixed hardware counts
 *     fixed resource counts
 *     device identifiers
 *     physical addresses
 *     physical qubit identifiers
 *     CPU identifiers
 *     GPU identifiers
 *     FPGA identifiers
 *     QPU identifiers
 *     topology limits
 *     tensor limits
 *     memory limits
 *
 * Numeric literals are not used as language capacity controls.
 *
 * ============================================================================
 * DETERMINISTIC COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is COMPLETE when:
 *
 *     [ ] It compiles as an ANTLR parser grammar.
 *     [ ] It consumes the canonical ZamaniLexer vocabulary.
 *     [ ] It has no embedded actions.
 *     [ ] It has no semantic predicates.
 *     [ ] It has no Rust dependencies.
 *     [ ] It has no unsafe implementation requirement.
 *     [ ] It owns exactly one reusable binding abstraction.
 *     [ ] It introduces no declaration duplication.
 *     [ ] It introduces no assignment duplication.
 *     [ ] It introduces no type grammar duplication.
 *     [ ] It introduces no expression grammar duplication.
 *     [ ] It introduces no hardware limits.
 *     [ ] It introduces no quantum limits.
 *     [ ] It preserves source-level portability.
 *     [ ] It has complete AST mapping.
 *     [ ] It has complete semantic integration.
 *     [ ] It has complete conformance tests.
 *     [ ] It has negative and boundary tests.
 *     [ ] It has scalability tests.
 *     [ ] It has determinism tests.
 *     [ ] It has compatibility tests.
 *
 * ============================================================================
 * END
 * ============================================================================
 */
/*
================================================================================
ZAMANI — RANGE EXPRESSION GRAMMAR
File: grammar/expressions/range.g4

Status:
    Canonical modular expression grammar component.

Purpose:
    Defines the syntax of portable range expressions without defining their
    semantic meaning, iteration strategy, collection materialization, resource
    consumption, or target-specific implementation.

Language goals:
    - Universal classical computing
    - Quantum computing
    - Hybrid quantum/classical computing
    - HDL and hardware/software co-design
    - Embedded and systems programming
    - Parallel/HPC computing
    - Distributed computing
    - AI/ML and tensor/data computation
    - Networking
    - Scientific computing
    - Future computational domains

POCO-REAF:
    Program Once, Compile Once, Run Everywhere, Anywhere, Forever.

SCALABILITY:
    This grammar imposes NO language-level maximum on:
      - range length
      - numeric magnitude
      - index magnitude
      - iteration count
      - tensor/array dimensions
      - memory size
      - number of resources
      - number of processing elements
      - number of quantum/classical resources
      - distributed nodes
      - hardware devices
      - nesting depth
      - number of range expressions

    Any finite limit encountered during parsing, semantic analysis,
    compilation, optimization, scheduling, or execution is an implementation
    or resource constraint and MUST NOT be encoded here as a universal
    language limit.

================================================================================
OWNERSHIP CONTRACT
================================================================================

This file OWNS:

    - range-expression syntax
    - inclusive range syntax
    - exclusive range syntax
    - open-start range syntax
    - open-end range syntax
    - range-expression composition with the lower expression layer
    - the syntactic representation of range bounds

This file DOES NOT OWN:

    - assignment syntax
    - compound assignment syntax
    - arithmetic operators
    - comparison operators
    - equality operators
    - logical operators
    - bitwise operators
    - shift operators
    - unary operators
    - postfix operators
    - indexing syntax
    - function calls
    - collection syntax
    - iteration statements
    - loop semantics
    - iterator construction
    - range materialization
    - numeric conversion
    - type checking
    - overflow behavior
    - underflow behavior
    - endpoint ordering
    - step calculation
    - empty-range determination
    - infinite-range determination
    - quantum semantics
    - hardware semantics
    - scheduling
    - resource allocation
    - optimization
    - runtime execution

Those responsibilities belong to the appropriate expression, type,
semantic-analysis, IR, compiler, runtime, resource, quantum, HDL, and
execution components.

================================================================================
AUTHORITY
================================================================================

Canonical authority hierarchy:

    language specification
        ↓
    modular grammar components
        ↓
    grammar/Zamani.g4 composition
        ↓
    lexer/parser implementation
        ↓
    domain-neutral frontend AST
        ↓
    semantic model
        ↓
    canonical/domain IR
        ↓
    compiler/runtime

This file MUST NOT become a competing language specification.

grammar/Zamani-Grammar.md is not an independent syntax authority.

grammar/grammar.md describes implementation conformance and MUST NOT define
syntax independently of the canonical grammar/specification.

================================================================================
LEXER CONTRACT
================================================================================

This parser grammar consumes the canonical modular Zamani token vocabulary.

The lexer owns the spelling and tokenization of range delimiters.

Expected canonical range delimiter tokens:

    RANGE_EXCLUSIVE
    RANGE_INCLUSIVE

The canonical lexer MUST map the source spellings:

    ..
    ..=

to those tokens respectively.

No literal operator spelling is defined in this parser grammar.

This prevents parser components from silently creating a second lexical
authority.

If the repository's canonical lexer uses different token names, those names
must be established in grammar/lexer/tokens.g4 and then used consistently
throughout the expression grammar. This file must not invent a second token
vocabulary.

================================================================================
EXPRESSION INTEGRATION
================================================================================

Range expressions sit above the expression layer they consume and below any
higher-level expression construct that must bind less tightly.

The intended conceptual hierarchy is:

    expression
        ↓
    assignmentExpression
        ↓
    conditional / pipeline / other higher-level expression layers
        ↓
    rangeExpression
        ↓
    logical / comparison / bitwise / shift / arithmetic layers
        ↓
    prefix / postfix / primary expressions

The exact composition is owned by grammar/expressions/expression.g4.

This file therefore MUST NOT redefine:

    expression
    assignmentExpression
    conditionalExpression
    comparisonExpression
    additiveExpression
    multiplicativeExpression
    prefixExpression
    postfixExpression
    primaryExpression

unless an explicit future architectural change makes this file the owner of
one of those rules.

================================================================================
RANGE MODEL
================================================================================

A range has two independently optional endpoints and an endpoint-boundary
mode.

Supported forms:

    start .. end
    start ..= end
    start ..
    start ..=
    .. end
    ..= end
    ..
    ..=

The semantic layer determines which forms are meaningful for a particular
rangeable type.

Examples of syntactically representable forms include:

    0 .. 10
    0 ..= 10
    1 .. n
    1 ..= n
    start .. end
    start ..=
    .. end
    ..= end
    ..

The grammar does not decide whether an omitted endpoint means:

    - an unbounded mathematical interval
    - a collection boundary
    - an iterator boundary
    - a type-defined default
    - an error

That is semantic/type-system responsibility.

================================================================================
BOUNDARY SEMANTICS
================================================================================

RANGE_EXCLUSIVE:

    start .. end

represents an exclusive upper boundary.

RANGE_INCLUSIVE:

    start ..= end

represents an inclusive upper boundary.

For open ranges, the absence of an endpoint is preserved in the AST.

The grammar MUST NOT silently insert:

    0
    1
    MIN
    MAX
    machine_word_max
    type_max
    infinity

as an omitted endpoint.

Such defaults, when meaningful, are semantic/type-specific.

================================================================================
STEP SEMANTICS
================================================================================

This grammar deliberately does NOT encode a step.

There is no universal syntax here such as:

    start .. step .. end

unless and until a separate language-wide range-step specification is
established.

A range is therefore syntactically a boundary expression, not an iteration
algorithm.

Step behavior may be represented later by:

    iterator semantics
    library operations
    typed range abstractions
    domain-specific semantic capabilities

This avoids confusing:

    range description

with:

    execution strategy.

A compiler may lower a range to an iterator, loop, vector operation,
distributed partition, tensor slice, HDL generate domain, or another
representation depending on semantic analysis and target capabilities.

================================================================================
AST CONTRACT
================================================================================

Every parsed range expression MUST preserve:

    - range kind
    - lower-bound presence/absence
    - upper-bound presence/absence
    - lower-bound expression
    - upper-bound expression
    - source span of the complete range
    - source spans of both bounds
    - operator source span
    - syntactic nesting/order

Conceptually:

    RangeExpression {
        kind:
            Exclusive
            | Inclusive,

        start:
            Optional<Expression>,

        end:
            Optional<Expression>,

        source:
            SourceSpan
    }

The exact Rust AST type is owned by the existing domain-neutral frontend AST.

This grammar MUST NOT introduce:

    IntegerRange
    QuantumRange
    TensorRange
    HardwareRange
    MemoryRange
    PhysicalQubitRange

or other domain-specific AST variants.

A range is a generic language construct.

================================================================================
SEMANTIC CONTRACT
================================================================================

Semantic analysis MUST determine:

    - whether the bound expressions are valid
    - whether the bound types are range-compatible
    - whether the two bounds have compatible types
    - whether required conversions are legal
    - whether the range is finite or unbounded
    - whether endpoint ordering is valid
    - whether the range can be represented lazily
    - whether materialization is requested elsewhere
    - whether the range is valid for its consuming operation

Semantic analysis MUST NOT assume that all ranges are integer ranges.

Potential rangeable domains may include:

    integers
    arbitrary-precision integers
    characters
    enumerations
    timestamps
    symbolic values
    fixed-point values
    rational values
    user-defined ordered domains
    tensor/index domains
    distributed partitions
    hardware-generation domains
    quantum-index domains

Whether a particular domain supports ranges is a type-system/semantic
decision.

================================================================================
ARITHMETIC INTEGRATION
================================================================================

Range bounds consume the lower expression hierarchy.

Therefore expressions such as:

    a + b .. c * d
    start .. count + offset
    2 ** n .. limit

are structurally composed from the canonical arithmetic grammar rather than
duplicating arithmetic rules here.

This file MUST NOT redefine:

    additiveExpression
    multiplicativeExpression
    exponentExpression

Arithmetic precedence remains owned by:

    grammar/expressions/arithmetic.g4

================================================================================
COMPARISON INTEGRATION
================================================================================

Range syntax is distinct from comparison syntax.

For example:

    a < b

is a comparison expression.

Whereas:

    a .. b

is a range expression.

The range grammar MUST NOT absorb:

    <
    <=
    >
    >=
    ==
    !=

as range delimiters.

A bound may itself contain a comparison only when permitted by the
parenthesized/lower expression grammar.

The semantic layer determines the resulting type and validity.

================================================================================
ASSIGNMENT INTEGRATION
================================================================================

Assignment remains outside range construction.

Examples:

    x = 0 .. n
    range = start ..= end

are assignment expressions whose right-hand side contains a range.

This grammar MUST NOT define:

    =
    +=
    -=
    *=
    /=
    %=

or any other assignment operator.

Those remain owned by:

    grammar/expressions/assignment.g4

The assignment grammar consumes the range expression through the canonical
expression hierarchy.

================================================================================
POSTFIX / INDEXING INTEGRATION
================================================================================

A range may be used by consumers such as indexing or slicing where those
consumers explicitly accept range expressions.

Examples conceptually include:

    array[start .. end]
    tensor[..]
    buffer[start ..= end]

The range grammar MUST NOT define brackets or indexing.

Those belong to:

    grammar/expressions/indexing.g4

This separation permits the same RangeExpression AST to be consumed by
classical arrays, tensors, data structures, HDL structures, quantum
register/index abstractions, and future domains.

================================================================================
LOOP INTEGRATION
================================================================================

Range syntax is not a loop.

A consumer may later use:

    for i in 0 .. n { ... }

but:

    0 .. n

remains a range expression.

The range grammar MUST NOT define:

    for
    while
    loop
    parallel
    distribute
    iterate

or execution semantics.

Those belong to statements/concurrency/distributed/execution components.

================================================================================
CLASSICAL INTEGRATION
================================================================================

Classical consumers may use ranges for:

    - array indexing
    - slices
    - loops
    - tensor dimensions
    - numerical domains
    - data processing
    - partitioning
    - vector operations
    - scientific computation

The grammar remains independent of the implementation.

No CPU width, register width, integer width, vector width, memory size, or
array-size maximum is encoded here.

================================================================================
QUANTUM INTEGRATION
================================================================================

Quantum consumers may use range expressions for semantic constructs such as:

    - logical register selections
    - symbolic qubit/index domains
    - operation target sets
    - measurement selections
    - parameter domains
    - generated circuit regions

The grammar MUST NOT map a range directly to physical qubits.

For example:

    q[0 .. n]

must not imply:

    physical_qubit(0)
    physical_qubit(1)
    ...
    physical_qubit(n)

Physical realization remains downstream.

The pipeline remains:

    source
      ↓
    frontend AST
      ↓
    semantic quantum model
      ↓
    quantum::ir
      ↓
    optimization
      ↓
    routing
      ↓
    scheduling
      ↓
    QEC / resilience / ZQN
      ↓
    HAL
      ↓
    physical target

This grammar introduces no competing quantum IR.

================================================================================
HDL INTEGRATION
================================================================================

HDL consumers may use ranges for:

    - parameterized buses
    - array dimensions
    - generated hardware structures
    - address/index domains
    - pipeline structures
    - memory declarations

A range such as:

    0 .. width

does not impose a universal maximum width.

The actual width is determined by the program, type system, hardware
constraints, compilation target, and available resources.

================================================================================
HARDWARE / RESOURCE INTEGRATION
================================================================================

Range expressions may describe portable sets/domains.

They MUST NOT silently become physical placement directives.

For example:

    0 .. n

does not mean:

    physical_device_0 through physical_device_n

unless an explicit downstream semantic construct requests physical
placement.

Requirements, capabilities, preferences, hints, and implementation
decisions remain distinct.

================================================================================
DISTRIBUTED INTEGRATION
================================================================================

A range can represent an abstract domain used for:

    - partitioning
    - indexing
    - sharding
    - task generation
    - collective operations

It MUST NOT establish a fixed cluster size.

For example:

    0 .. nodes

does not impose a maximum number of nodes.

Actual placement and distribution are downstream concerns.

================================================================================
AI / DATA / TENSOR INTEGRATION
================================================================================

Ranges may participate in:

    - tensor slices
    - dataset partitions
    - batch domains
    - sequence windows
    - feature/index domains
    - dataflow partitions

The grammar MUST NOT encode fixed tensor rank, shape, batch size, or
accelerator count.

Those belong to type/semantic/resource systems.

================================================================================
LAZINESS AND MATERIALIZATION
================================================================================

A range expression describes a domain.

It does not require immediate materialization into memory.

This distinction is essential for scalability.

For example:

    0 .. enormous_value

may semantically represent a compact range even when materializing every
element would exceed available memory.

The compiler/runtime may choose:

    lazy iterator
    streaming execution
    vectorized execution
    distributed execution
    symbolic representation
    hardware generation
    another target-specific representation

provided the observable language semantics are preserved.

The grammar itself makes no materialization decision.

================================================================================
INFINITE / UNBOUNDED REPRESENTATIONS
================================================================================

Open-ended syntax such as:

    start ..
    .. end
    ..

must remain syntactically representable without imposing a finite bound.

Whether an open range denotes an infinite domain, a contextual boundary,
or an invalid standalone value is determined by semantic context.

The parser MUST preserve the distinction between:

    omitted endpoint

and:

    explicitly specified endpoint.

================================================================================
DETERMINISM
================================================================================

Given identical:

    - source token stream
    - grammar version
    - parser configuration

the grammar must produce the same parse structure.

No parsing decision may depend on:

    - CPU count
    - GPU count
    - QPU availability
    - memory size
    - hardware topology
    - runtime state
    - network state
    - random state

================================================================================
ERROR / DIAGNOSTIC CONTRACT
================================================================================

Syntax errors belong to the parser.

Examples of syntactically invalid forms include malformed range delimiters
or incomplete constructs where the surrounding expression grammar cannot
legally complete them.

Semantic errors do NOT belong in this grammar.

Examples:

    incompatible bound types
    unsupported rangeable type
    invalid ordering
    overflow during a required conversion
    unsupported infinite range in a particular consumer
    illegal range materialization

must be reported by semantic/type/resource analysis.

Diagnostics should preserve source spans for:

    - lower bound
    - range operator
    - upper bound
    - complete range expression

================================================================================
COMPATIBILITY CONTRACT
================================================================================

Range syntax is part of the canonical expression language.

Any future syntax extension MUST preserve the existing interpretation of:

    start .. end

and

    start ..= end

unless a language-version compatibility decision explicitly changes it.

Potential future additions such as stepped ranges must not silently change
the meaning of existing two-bound ranges.

Compatibility changes must be recorded through the repository's canonical
compatibility/versioning mechanism.

================================================================================
NO HARD-CODING CONTRACT
================================================================================

This grammar MUST NOT contain constructs such as:

    MAX_RANGE
    MAX_RANGE_LENGTH
    MAX_RANGE_VALUE
    MAX_INDEX
    MAX_DIMENSION
    MAX_ELEMENTS
    MAX_ITERATIONS
    MAX_TENSOR_SIZE
    MAX_QUANTUM_RANGE
    MAX_NODES

Nor may it encode fixed endpoint types merely because they match current
hardware.

All resource limitations belong to:

    semantic analysis
    resource analysis
    compiler configuration
    runtime
    target capabilities
    deployment environment

and are not language grammar limits.

================================================================================
RUST CONTRACT
================================================================================

The grammar contains no Rust semantic actions and therefore does not require
unsafe Rust.

Generated parser/frontend integration MUST remain compatible with:

    Rust 1.97
    Rust 1.97.1

The Rust implementation MUST use safe Rust only.

No `unsafe` block or `unsafe` implementation is required by this grammar.

================================================================================
TEST CONTRACT
================================================================================

The canonical conformance suite must include at least:

POSITIVE:

    a .. b
    a ..= b
    a ..
    .. b
    ..= b
    ..
    ..=
    0 .. n
    0 ..= n
    start + offset .. end * scale
    expression .. expression
    nested range-bound expressions where permitted

PRECEDENCE:

    a + b .. c
    a .. b + c
    a * b .. c
    a .. b * c
    a ** b .. c
    a .. b ** c

The expected parse must prove that range boundaries consume the intended
lower expression layer without duplicating arithmetic precedence.

ASSIGNMENT:

    x = a .. b
    x = a ..= b

NEGATIVE:

    a ...
    a ..= = b
    a .. .. b
    a ..= .. b

and malformed forms rejected by the surrounding expression grammar.

BOUNDARY:

    0 .. 0
    0 ..= 0
    negative-bound .. positive-bound
    symbolic .. symbolic
    very large representable bounds
    nested/parenthesized bounds
    open-start range
    open-end range
    fully open range

SCALABILITY:

    extremely long syntactically valid expressions
    arbitrarily large representable bounds
    arbitrarily many range expressions in a program
    nested consumers
    parameterized dimensions

No test may establish a language-level maximum.

DETERMINISM:

    identical token streams must yield identical parse trees.

COMPATIBILITY:

    canonical historical range syntax must continue to parse identically
    across compatible grammar versions.

================================================================================
INDEPENDENT COMPLETION CRITERIA
================================================================================

This file is complete when all of the following are true:

    [ ] Range syntax is completely defined.
    [ ] Inclusive and exclusive boundaries are unambiguous.
    [ ] Open endpoints are represented explicitly.
    [ ] Canonical lexer tokens are used.
    [ ] No lexer rules are duplicated here.
    [ ] No assignment rules are duplicated here.
    [ ] No arithmetic rules are duplicated here.
    [ ] No comparison rules are duplicated here.
    [ ] No postfix/indexing rules are duplicated here.
    [ ] No loop/iteration semantics are embedded here.
    [ ] Precedence is explicitly documented.
    [ ] Associativity is explicitly documented.
    [ ] AST mapping is predetermined.
    [ ] Source-span preservation is predetermined.
    [ ] Semantic responsibilities are predetermined.
    [ ] Canonical IR integration is predetermined.
    [ ] Quantum integration terminates at semantic/quantum::ir boundaries.
    [ ] HDL integration remains target-independent.
    [ ] Resource/capability semantics remain downstream.
    [ ] No fixed hardware/resource limit exists.
    [ ] No finite range-size limit exists.
    [ ] No fixed numeric-width assumption exists.
    [ ] Diagnostics ownership is defined.
    [ ] Compatibility ownership is defined.
    [ ] Positive tests are defined.
    [ ] Negative tests are defined.
    [ ] Boundary tests are defined.
    [ ] Scalability tests are defined.
    [ ] Determinism tests are defined.
    [ ] Rust 1.97/1.97.1 integration remains safe.
    [ ] No unsafe Rust requirement exists.
    [ ] Canonical expression composition imports this grammar.
    [ ] Legacy expression grammars do not independently redefine range
        precedence.

================================================================================
CANONICAL RULES
================================================================================
*/

parser grammar Range;

options {
    tokenVocab = ZamaniTokens;
}

/*
------------------------------------------------------------------------------
Range expression

The range operator binds at the range-expression level.

The operands are lower-level expressions supplied by the canonical expression
composition grammar.

The imported/composing grammar must provide the rule used for range bounds.

The rule name `assignmentExpression` is deliberately NOT used here because
assignment belongs above range expressions in the canonical expression
hierarchy.

The lower operand rule must therefore be the expression layer immediately
below range syntax as defined by grammar/expressions/expression.g4.
------------------------------------------------------------------------------
*/

rangeExpression
    : rangeStart? rangeOperator rangeEnd?
    ;

/*
------------------------------------------------------------------------------
Range start/end

The exact lower expression rule is supplied by the canonical expression
composition grammar.

These wrapper rules exist so the AST/parser integration can distinguish:

    no start
    present start

from:

    no end
    present end

without introducing semantic defaults.

The composition grammar must connect `rangeBoundExpression` to the canonical
lower expression layer.

This avoids making range.g4 own arithmetic/comparison/shift/etc.
------------------------------------------------------------------------------
*/

rangeStart
    : rangeBoundExpression
    ;

rangeEnd
    : rangeBoundExpression
    ;

/*
------------------------------------------------------------------------------
Range operator

Exclusive:
    ..

Inclusive:
    ..=

The lexical spelling is owned by grammar/lexer/tokens.g4.
------------------------------------------------------------------------------
*/

rangeOperator
    : RANGE_EXCLUSIVE
    | RANGE_INCLUSIVE
    ;

/*
------------------------------------------------------------------------------
Range bound

This rule is intentionally an integration hook.

The canonical expression composition grammar must bind this rule to the
expression layer immediately below rangeExpression.

It MUST NOT be connected to assignmentExpression, because doing so would make
assignment part of a range endpoint and create precedence ambiguity.

The canonical composition should therefore establish:

    rangeExpression
        → rangeBoundExpression
        → lower expression hierarchy

with the concrete lower rule supplied by expression.g4.

No semantic behavior is encoded here.
------------------------------------------------------------------------------
*/

rangeBoundExpression
    : /* supplied by canonical expression composition */
    ;
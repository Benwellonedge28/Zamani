Zamani Expression Grammar Conformance Contract

File: "grammar/expressions/conformance.md"
Status: Normative / Production
Language: Zamani
Grammar technology: ANTLR4
Implementation baseline: Rust 1.97 / Rust 1.97.1
Rust edition: 2021
Safety requirement: Safe Rust only; no "unsafe" Rust
Portability goal: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)

---

1. Purpose

This document is the authoritative conformance and integration contract for the Zamani expression subsystem.

It defines what must be true for the complete expression layer to be considered production-ready.

The expression subsystem is not merely a collection of ".g4" files.

It is one integrated language subsystem spanning:

language specification
        ↓
lexical specification
        ↓
canonical lexer
        ↓
expression grammar
        ↓
parser composition
        ↓
domain-neutral frontend AST
        ↓
structural validation
        ↓
semantic analysis
        ↓
canonical semantic model / ZUIR
        ↓
domain IR
        ↓
compiler
        ↓
runtime
        ↓
target realization

Expressions must therefore remain independent of:

- CPU model;
- GPU model;
- FPGA model;
- QPU model;
- physical qubit numbering;
- hardware topology;
- memory capacity;
- register width;
- accelerator count;
- node count;
- thread count;
- machine size;
- runtime state;
- scheduler state;
- calibration state;
- backend availability.

Those concerns are downstream.

---

2. Production Definition

The expression subsystem is production-ready only when:

1. there is one canonical expression entry point;
2. there is one authoritative precedence hierarchy;
3. every operator has one canonical lexical identity;
4. no expression grammar invents parser-only token aliases;
5. conditional expressions have one owner;
6. range expressions have one owner;
7. assignment has one owner;
8. postfix expressions have one owner;
9. unary expressions have one owner;
10. primary expressions have one owner;
11. specialized domains consume the canonical expression language;
12. domain grammars do not redefine general expressions;
13. expression syntax maps to the existing domain-neutral AST;
14. expression semantics are resolved downstream;
15. quantum expressions lower through the existing "quantum::ir";
16. expressions do not create a second quantum IR;
17. no hardware/resource limit is encoded into expression syntax;
18. parser behavior is deterministic;
19. parsing performs no execution or external I/O;
20. source spans survive parsing;
21. diagnostics identify the relevant source range;
22. positive tests exist;
23. negative tests exist;
24. boundary tests exist;
25. scalability tests exist;
26. determinism tests exist;
27. compatibility tests exist;
28. Rust 1.97/1.97.1 compatibility is maintained;
29. the compiler implementation uses no "unsafe";
30. the expression subsystem can be completed without later semantic redesign caused by another expression file.

---

3. Authority Model

The expression subsystem follows this authority hierarchy:

grammar/specification/syntax.md
            ↓
grammar/specification/semantics.md
            ↓
grammar/spec/syntax.md
            ↓
grammar/lexer/*
            ↓
grammar/antlr/ZamaniLexer.g4
            ↓
grammar/expressions/*
            ↓
grammar/Zamani.g4
            ↓
src/lexer.rs
            ↓
src/parser.rs
            ↓
src/frontend/ast/*
            ↓
semantic analysis
            ↓
canonical semantic model / ZUIR
            ↓
domain IR
            ↓
compiler
            ↓
runtime

"grammar/Zamani-Grammar.md" is not allowed to introduce syntax independently.

"grammar/grammar.md" is an implementation-conformance reference, not a competing specification.

"grammar/DESIGN.md" defines architecture.

This file defines expression-subsystem conformance.

---

4. Existing Repository State That Must Be Resolved

The repository currently contains:

grammar/expressions/expressions.g4
grammar/expressions/conditionals.g4
grammar/expressions/range.g4
grammar/expressions/ranges.g4

There is currently an ownership conflict.

"expressions.g4" contains its own "rangeExpression" and "rangeOperator" composition.

"ranges.g4" separately defines the range expression and explicitly describes itself as the production modular range-expression grammar.

"range.g4" also defines range-expression syntax and describes itself as a canonical modular component.

This cannot remain as three active authorities.

Likewise, "conditionals.g4" is explicitly intended to be the single modular owner of "conditionalExpression".

Therefore the production architecture is:

expressions.g4
    = expression composition authority

conditionals.g4
    = conditional-expression syntax authority

ranges.g4
    = range-expression syntax authority

range.g4
    = compatibility/reference file only
      OR removed from active grammar composition

No unnecessary filename rename is required.

The important requirement is that only one file defines each public parser rule in the actual parser composition.

---

5. Canonical Expression Composition

The canonical expression graph is:

expression
    ↓
assignmentExpression
    ↓
conditionalExpression
    ↓
rangeExpression
    ↓
logicalOrExpression
    ↓
logicalAndExpression
    ↓
bitwiseOrExpression
    ↓
bitwiseXorExpression
    ↓
bitwiseAndExpression
    ↓
equalityExpression
    ↓
relationalExpression
    ↓
shiftExpression
    ↓
additiveExpression
    ↓
multiplicativeExpression
    ↓
prefixExpression
    ↓
postfixExpression
    ↓
primaryExpression

This hierarchy must exist exactly once.

Specialized files may provide individual components but must not create another complete expression hierarchy.

---

6. Canonical Public Rule

There must be exactly one public:

expression

rule.

The canonical owner is:

grammar/expressions/expressions.g4

All consumers must use:

expression

rather than inventing:

conditionExpression
valueExpression
quantumExpression
hardwareExpression
aiExpression
resourceExpression
tensorExpression

as competing general-purpose expression systems.

Domain-specific expression rules may exist only where they provide domain syntax that cannot be represented by the universal expression layer.

---

7. Expression Ownership Table

Concern| Authoritative owner
"expression"| "expressions.g4"
assignment composition| "expressions.g4" / "assignment.g4" contract
conditional expression| "conditionals.g4"
range expression| "ranges.g4"
logical operators| "expressions.g4" / logical component
bitwise operators| bitwise component
equality| comparison component
relational comparison| comparison component
shifts| shift component
arithmetic| arithmetic component
unary| "unary.g4"
postfix| "postfix.g4"
calls| "calls.g4"
indexing| "indexing.g4"
member access| "member-access.g4"
literals| literal grammar
primary expressions| canonical expression composition
precedence| "conformance.md" + normative precedence specification
token spelling| canonical lexer
AST| "src/frontend/ast/"
semantic meaning| semantic analysis
quantum meaning| quantum semantic layer / "quantum::ir"
resource meaning| resource/capability analysis
target realization| compiler/backend/runtime

No row may have two active authorities.

---

8. Canonical Precedence

The expression precedence order is:

Level| Category| Associativity
1| assignment| right
2| conditional| right
3| range| non-associative
4| logical OR| left
5| logical AND| left
6| bitwise OR| left
7| bitwise XOR| left
8| bitwise AND| left
9| equality| non-associative
10| relational| non-associative
11| shift| left
12| additive| left
13| multiplicative| left
14| prefix/unary| right
15| postfix| left
16| primary/atomic| not applicable

This hierarchy must be reflected consistently by:

- grammar;
- parser;
- AST nesting;
- semantic analysis;
- tests;
- diagnostics;
- documentation.

No specialized grammar may silently establish another precedence.

---

9. Assignment

Assignment is the lowest-precedence ordinary expression operator.

Examples:

x = y
x += y
x -= y
x *= y
x /= y
x %= y
x &= y
x |= y
x ^= y

Assignment is right-associative:

a = b = c

must structurally represent:

a = (b = c)

The grammar may recognize a syntactic assignment target.

Semantic analysis decides whether the target is actually assignable.

For example:

42 = x

may reach semantic validation as an invalid assignment target.

The grammar must not encode machine-specific lvalue restrictions.

---

10. Conditional Expressions

"grammar/expressions/conditionals.g4" owns:

conditionalExpression
ifExpression
elseIfExpressionBranch
elseExpressionBranch
ternaryConditionalExpression

The current file explicitly establishes itself as that owner.

The canonical composition must therefore consume that component exactly once.

The expression composition must not define a second:

conditionalExpression

rule.

The following forms must have deterministic structures:

condition ? a : b

and:

a ? b : c ? d : e

If Zamani specifies right associativity, the latter becomes:

a ? b : (c ? d : e)

Structured conditionals:

if condition {
    value_a
} else {
    value_b
}

remain expressions when used in expression position.

Statement-level conditionals belong to:

grammar/statements/

and must not redefine value-producing conditional syntax.

---

11. Conditional Composition Constraint

The current "conditionals.g4" consumes "expression" in its ternary rule.

This creates a potential recursive composition problem if the canonical "expression" rule also directly delegates to "conditionalExpression".

The production implementation must therefore establish one of the following explicit parser architectures:

expression
    → assignmentExpression
        → conditionalExpression
            → ...

with ternary branches consuming the appropriate lower/conditional expression boundary,

or an equivalent ANTLR-safe architecture that produces exactly the specified associativity without uncontrolled recursion.

The implementation MUST NOT retain an accidental cycle:

expression
 → conditionalExpression
 → ternary
 → expression
 → conditionalExpression
 → ...

merely because the files happen to compile independently.

This must be verified by grammar-generation tests.

---

12. Range Authority

Only one active file owns:

rangeExpression

The production authority should be:

grammar/expressions/ranges.g4

because the existing "ranges.g4" already explicitly defines itself as the production modular range-expression grammar and establishes "DOT_DOT" / "DOT_DOT_EQ" as the canonical lexical tokens.

"range.g4" must not remain an independently composed second implementation of the same public rule.

Do not rename it merely for cosmetic reasons.

Instead:

- remove it from active parser composition if redundant;
- or convert it into a compatibility/reference contract;
- or make it a non-authoritative wrapper that delegates to "ranges.g4".

It must never define a second independently compiled "rangeExpression".

---

13. Range Composition

The canonical range forms are:

start .. end
start ..= end

start ..
start ..=

.. end
..= end

..
..=

The parser preserves whether each endpoint exists.

It must never replace an omitted endpoint with:

0
1
MIN
MAX
type_max
machine_word_max
infinity

or any target-specific value.

---

14. Range Endpoint Boundary

Range endpoints must consume the appropriate lower expression layer.

They must not recursively consume unrestricted:

expression

if doing so re-enters assignment, conditional, or range parsing.

This is specifically important for avoiding:

expression
    → range
        → expression
            → range
                → ...

The range grammar must therefore establish a stable precedence boundary.

---

15. Range Scalability

The expression grammar must not define:

MAX_RANGE_LENGTH
MAX_RANGE_VALUE
MAX_INDEX
MAX_ITERATIONS
MAX_ELEMENTS
MAX_DIMENSIONS

or equivalent limits.

This allows:

0 .. n

to remain valid independently of whether "n" represents:

- 10;
- 10 million;
- 10 billion;
- a symbolic value;
- an implementation-supported arbitrary-precision value;
- a distributed domain.

Materialization is not parsing.

A range may eventually become:

- an iterator;
- a lazy domain;
- a slice;
- a tensor domain;
- a distributed partition;
- a hardware generation domain;
- a quantum index domain;
- a symbolic mathematical interval.

The expression grammar must not choose the implementation.

---

16. Equality

Canonical equality operators:

==
!=

Equality is non-associative unless the language specification explicitly adds comparison chaining.

Therefore:

a == b == c

must not silently acquire an undocumented interpretation.

If chaining is eventually supported, it must be an explicit semantic/language feature with its own AST and specification contract.

---

17. Relational Operators

Canonical relational operators:

<
<=
>
>=

Relational expressions are non-associative by default.

Therefore:

a < b < c

must not silently become:

(a < b) < c

or:

a < (b < c)

without an explicit language rule.

Parentheses provide explicit grouping.

---

18. Logical Operators

Canonical logical operators include:

&&
||

They remain semantic logical operations.

The expression grammar establishes structure.

Semantic analysis establishes:

- operand validity;
- result type;
- short-circuit semantics;
- effect behavior;
- evaluation guarantees.

Short-circuit behavior must not be encoded as a parser-side side effect.

---

19. Bitwise Operators

Canonical bitwise operators include:

&
^
|

Their semantic interpretation depends on the operand types.

The same syntax may eventually operate over:

- integers;
- bit vectors;
- symbolic values;
- hardware signals;
- tensor values;
- domain-specific computational objects.

The grammar must not create:

QuantumBitAnd
HardwareBitAnd
TensorBitAnd

parser-level operator hierarchies.

---

20. Shift Operators

Canonical shifts:

<<
>>

The expression subsystem must use the token names defined by the canonical lexer.

The parser must not invent aliases such as:

SHIFT_LEFT
SHIFT_RIGHT

if the authoritative lexer instead establishes another spelling.

The current repository contains token-vocabulary documentation in the expression grammar that must be checked against the actual canonical lexer before parser generation. The lexer is authoritative for token identity.

A conformance test must fail if an expression grammar references a token that is absent from the canonical lexer.

---

21. Lexer Authority

Expression grammars contain no lexer rules.

The canonical lexical authority is:

grammar/antlr/ZamaniLexer.g4

The repository explicitly identifies that file as the canonical lexical grammar.

Expression grammar files consume tokens.

They do not define alternative token names merely for convenience.

For every expression operator:

source spelling
→ canonical lexer token
→ expression grammar use
→ AST operator identity
→ semantic operator

must be traceable.

---

22. Token-Conformance Requirement

For every token referenced by an expression grammar:

token X

must exist in the canonical lexer.

The conformance test must verify:

expression grammar token set
    ⊆
canonical lexer token set

Any missing token is a hard conformance failure.

Any duplicate lexical spelling with different token identity must also be investigated.

---

23. AST Contract

Expressions lower into the existing domain-neutral frontend AST.

The expression grammar must never introduce a second AST model.

Every expression node must preserve, where applicable:

- complete source span;
- operator;
- operands;
- operand order;
- nesting;
- call arguments;
- index expressions;
- member access;
- literal meaning;
- generic arguments;
- attributes;
- modifiers;
- syntactic source form where required for diagnostics/refactoring.

The AST is not:

QuantumExpression
ClassicalExpression
GPUExpression
CPUExpression
HDLExpression

unless the existing frontend architecture explicitly requires a semantic domain node downstream.

The parser-level expression representation remains generic.

---

24. AST Mapping

Every public grammar construct must have a predetermined AST contract before being marked complete.

Required mapping:

grammar rule
    ↓
frontend AST node
    ↓
semantic model
    ↓
canonical IR

Examples:

a + b

must become a generic operation/binary expression representation.

f(a, b)

must become a generic call representation.

a[i]

must become a generic indexing representation.

a.b

must become generic member access.

a .. b

must become a generic range representation.

No domain-specific AST duplication is permitted.

---

25. Semantic Contract

The grammar answers:

«Is this structurally a Zamani expression?»

Semantic analysis answers:

«What does this expression mean?»

Semantic analysis owns:

- name resolution;
- type checking;
- generic inference;
- overload resolution;
- conversion;
- ownership;
- borrowing;
- effect checking;
- capability checking;
- resource requirements;
- domain legality;
- quantum legality;
- hardware capability requirements;
- interoperability.

The grammar must not attempt to perform these operations.

---

26. Resource and Capability Separation

Expression syntax must distinguish semantic computation from target realization.

These concepts remain separate:

requirement
constraint
capability
preference
hint
implementation decision

For example:

requires capability("quantum.measurement")

is fundamentally different from:

map q0 -> physical_qubit(17)

The first is portable intent.

The second is a downstream implementation decision.

Expression syntax must not collapse those layers.

---

27. POCO-REAF Requirement

The expression subsystem must permit one source program to scale across:

tiny machine
small machine
large machine
cluster
HPC system
GPU system
FPGA system
QPU system
hybrid system
distributed system
future target

without requiring expression syntax to be rewritten merely because the target size changes.

The grammar must not encode:

MAX_CPUS
MAX_CORES
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_QUBITS
MAX_QPUS
MAX_NODES
MAX_MEMORY
MAX_ACCELERATORS
MAX_TENSOR_RANK
MAX_VECTOR_WIDTH
MAX_REGISTER_WIDTH

as universal language constraints.

---

28. What "Infinity" Means

For the grammar:

«Infinity means absence of an artificial language-level finite capacity.»

It does not mean that:

- physical machines have infinite memory;
- compilers have infinite memory;
- runtimes have infinite execution time;
- hardware has infinite resources.

Actual limitations belong to:

- implementation;
- resource availability;
- target capability;
- deployment policy;
- runtime environment.

The language must not confuse those implementation limits with the language's semantic model.

---

29. Quantum Integration

Expressions are intentionally reusable by the quantum subsystem.

Examples:

theta + phi
angle * scale
condition ? q_a : q_b
q[start .. end]
operation(theta)(q)

The expression grammar must not enumerate quantum gates.

It must not define:

X
Y
Z
H
CNOT
CX
RX
RY
RZ

as universal expression grammar alternatives merely because they are common operations.

A quantum operation is represented semantically through the existing generic operation model.

The pipeline remains:

Zamani source
    ↓
expression grammar
    ↓
frontend AST
    ↓
semantic quantum analysis
    ↓
quantum::ir
    ↓
optimization
    ↓
routing
    ↓
scheduling
    ↓
QEC / resilience
    ↓
ZQN
    ↓
HAL
    ↓
target realization

The expression grammar must not create a second quantum IR.

---

30. Quantum Scalability

Expression syntax must not assume:

2 qubits
5 qubits
32 qubits
64 qubits
128 qubits
1024 qubits

or any other finite universal quantum size.

A program may express symbolic or resource-dependent computation.

For example:

for q in register {
    ...
}

or:

select q[start .. end]

must remain independent of the eventual physical QPU size.

Physical mapping belongs to routing/HAL/deployment.

---

31. Classical Integration

The same expression layer must support:

- scalar computation;
- arbitrary supported integer representations;
- floating-point computation;
- vector operations;
- matrix operations;
- tensor operations;
- symbolic computation;
- numerical computation;
- scientific computation;
- signal processing;
- control computation.

The expression grammar should represent generic structure.

Library and intrinsic functionality belongs downstream.

The grammar must not become a dictionary of every mathematical function.

For example, functions such as:

fft
svd
gradient
integrate
optimize

should not automatically become parser keywords.

They may be ordinary names/calls with semantic capabilities supplied by the relevant library or intrinsic subsystem.

---

32. AI / ML Integration

AI and ML constructs must consume the same expression system.

Expressions may describe:

- tensor operations;
- model parameters;
- datasets;
- transformations;
- differentiable computation;
- symbolic computation;
- inference;
- training;
- probabilistic values;
- agent computation.

Framework-specific semantics do not belong in the core expression grammar.

The expression system must remain independent of:

- CUDA;
- ROCm;
- vendor APIs;
- a particular ML framework;
- a particular accelerator.

---

33. Tensor and Data Scalability

Expressions must not encode fixed tensor dimensions.

Invalid architecture:

tensor1024
matrix1024
vector256

as universal grammar concepts.

Valid architecture:

tensor
matrix
vector
shape
index
range

with dimensions determined by:

- source program;
- types;
- values;
- symbolic constraints;
- resources;
- target capabilities.

No universal maximum tensor rank or element count belongs in expression syntax.

---

34. HDL Integration

HDL constructs may consume ordinary expressions for:

- widths;
- indices;
- parameters;
- generate conditions;
- timing expressions;
- addresses;
- state transitions;
- hardware configuration.

For example:

width - 1
index + offset
condition && enable

must use the same expression language.

The expression grammar must not create a separate software arithmetic language and hardware arithmetic language.

Semantic analysis determines hardware interpretation.

---

35. Hardware Independence

An expression such as:

index + offset

must not implicitly select:

CPU register
GPU register
FPGA register
physical qubit
memory bank
device address

Hardware selection belongs downstream.

Expressions describe computation.

Hardware realization is a later decision.

---

36. Distributed Computing

Expressions must work over abstract distributed data and computation.

They may eventually represent:

- partition identifiers;
- data indices;
- task domains;
- collective-operation parameters;
- distributed tensor dimensions;
- symbolic node domains.

But the expression grammar must not impose a fixed node count.

For example:

0 .. nodes

does not mean a cluster with a predefined maximum number of nodes.

---

37. Networking

Expressions may represent:

- addresses;
- ports;
- message fields;
- protocol parameters;
- routing metrics;
- timeouts;
- stream offsets.

However, the expression grammar must not bind expressions directly to physical network interfaces.

Network realization remains a semantic/runtime concern.

---

38. Effects

Expressions may participate in effectful operations.

The parser establishes syntax.

Effect analysis establishes:

- which effects are produced;
- which effects are required;
- whether an effect is permitted;
- whether an operation is pure;
- whether evaluation ordering matters.

The expression grammar must not execute effects.

---

39. Concurrency

Expressions must remain usable in:

- asynchronous functions;
- task bodies;
- parallel computation;
- actor systems;
- data-parallel computation;
- distributed execution.

The expression parser itself must remain deterministic and single-purpose.

It must not inspect runtime scheduling state to parse an expression.

---

40. Determinism

For identical:

source
language version
grammar version
parser configuration

the parser must produce the same structural result.

Parsing must not depend on:

- current time;
- random state;
- environment variables;
- filesystem contents;
- network state;
- CPU count;
- GPU count;
- QPU availability;
- memory size;
- scheduler state;
- calibration state;
- backend availability.

---

41. Non-Execution Requirement

Expression parsing is non-executing.

Parsing:

system.run(command)

must not execute the command.

The parser must not:

- open files;
- contact networks;
- load secrets;
- invoke commands;
- inspect hardware;
- call quantum devices;
- run HDL simulators;
- invoke compiler backends;
- execute user code.

Macros/metaprogramming must have explicit separate phases and security contracts.

---

42. Source Spans

Every expression construct must preserve source location information sufficient for:

- diagnostics;
- IDE navigation;
- refactoring;
- semantic errors;
- compiler errors;
- provenance;
- source-to-IR mapping.

At minimum, the implementation must be able to identify:

complete expression span
operator span
operand spans
nested expression spans

For ranges:

lower span
operator span
upper span
complete range span

For calls:

callee span
argument spans
comma/separator locations where needed

For conditionals:

condition span
then branch span
else-if condition spans
else branch span

---

43. Diagnostics

Syntax diagnostics belong to the parser.

Semantic diagnostics belong to semantic analysis.

Resource diagnostics belong to resource analysis.

Target diagnostics belong to target realization.

The expression grammar must not emit misleading semantic errors.

Examples:

unknown variable

is semantic.

incompatible operand types

is semantic/type-system.

insufficient QPU capability

is resource/target analysis.

malformed operator sequence

is syntax.

---

44. Error Recovery

Error recovery must not change the language's valid parse semantics.

The parser should recover sufficiently for tooling where supported, but recovered structures must not be mistaken for valid programs.

The AST must distinguish:

valid source

from:

recovered/incomplete source

where the existing frontend architecture requires this distinction.

---

45. No Semantic Predicates

Expression grammar files must not use semantic predicates to inspect:

- hardware;
- types;
- runtime state;
- symbol tables;
- resource availability;
- QPU state;
- backend availability.

The parser must remain context-independent except for syntactic grammar state.

This also keeps the ANTLR grammar compatible with safe Rust implementation requirements.

---

46. Safe Rust Requirement

The expression subsystem must be implementable with:

Rust 1.97
Rust 1.97.1
Edition 2021

using safe Rust.

No expression-parser feature may require:

unsafe

or unsafe FFI merely to parse or construct expression ASTs.

Any unsafe code elsewhere in the repository must not be introduced as a prerequisite for expression grammar conformance.

The expression subsystem itself must not add "unsafe".

---

47. No Target-Specific Expression Rules

The following are prohibited as universal expression grammar concepts:

cpu0
gpu0
fpga0
qpu0
qubit0
core0
thread0
memory_bank0
device0
node0

unless they occur as ordinary user-defined identifiers or explicit downstream target descriptions.

The grammar must not reserve these concepts merely to simplify a particular backend.

---

48. Generic Operations

The expression system should support generic operation structure.

For example:

operation(argument_1, argument_2)

is preferable to introducing a parser rule for every known operation.

This enables:

classical operation
quantum operation
HDL operation
AI operation
data operation
accelerator operation
future operation

to share the same syntactic foundation.

Semantic analysis resolves the operation.

---

49. No Domain-Specific Expression Forks

The following architecture is prohibited:

ClassicalExpression
QuantumExpression
HDLExpression
AIExpression
GPUExpression
DistributedExpression

each independently defining:

+
-
*
/
call
index
member access
conditional
range

The correct architecture is:

one universal expression language
        ↓
domain-neutral AST
        ↓
semantic interpretation
        ↓
domain IR

---

50. Parser Composition

The repository's ANTLR architecture must have an explicit composition mechanism.

Simply placing:

conditionals.g4
ranges.g4
arithmetic.g4

in the same directory does not integrate them.

The build must establish exactly how those parser grammars are:

- imported;
- combined;
- generated;
- referenced;
- tested.

A grammar file is not integrated merely because another file mentions its path in documentation.

---

51. Duplicate Rule Detection

Production validation must fail if two active grammar sources define the same public parser rule.

Examples:

expression
conditionalExpression
rangeExpression
assignmentExpression
postfixExpression
primaryExpression

must each have one active owner.

The validator must detect duplicate definitions across:

grammar/expressions/
grammar/antlr/
grammar/Zamani.g4

and any generated parser grammar source.

---

52. Range Duplicate Resolution

The current repository contains both:

range.g4
ranges.g4

and the current "expressions.g4" also contains range composition.

Production conformance requires:

ranges.g4
    = active range syntax owner

and:

expressions.g4
    = composition owner

Therefore "expressions.g4" must not independently redefine the implementation of "rangeExpression" if "ranges.g4" supplies it.

"range.g4" must not independently compete with "ranges.g4".

The filename itself does not have to be renamed.

Its active parser ownership must simply be resolved.

---

53. Conditional Duplicate Resolution

The current "conditionals.g4" explicitly identifies itself as the single modular owner of conditional expressions.

Therefore:

conditionals.g4
    = conditional-expression owner

and any existing:

conditional-expressions.g4

must not define another active "conditionalExpression".

Since the repository discussion has already established that "conditional-expressions.g4" is unused, it may be removed rather than maintained as dead grammar.

No compatibility wrapper is necessary if repository-wide reference validation confirms that nothing consumes it.

---

54. Expression File Independence Contract

Every expression grammar file must be independently completable.

Before declaring a file complete, its contract must already specify:

File
Purpose
Status
Owns
Does Not Own
Inputs
Outputs
Dependencies
Upstream Contracts
Downstream Consumers
Public Grammar Contract
AST Contract
Semantic Contract
IR Integration
Compiler Integration
Runtime Integration
Tooling Integration
Cross-Domain Integration
Positive Tests
Negative Tests
Boundary Tests
Scalability Tests
Determinism Tests
Compatibility Tests
Diagnostics
Security
Performance
Hard-Coding Audit
Completion Criteria

A file must not be marked complete with:

«"AST mapping will be decided later."»

or:

«"The compiler will determine this later."»

The exact integration boundary must already be specified.

---

55. Expression Completion Contract

An expression feature is complete only when all of the following exist:

✓ normative syntax
✓ lexical token mapping
✓ parser rule
✓ precedence
✓ associativity
✓ ambiguity resolution
✓ source-span behavior
✓ AST mapping
✓ semantic contract
✓ error boundary
✓ diagnostics
✓ IR mapping
✓ compiler consumer
✓ runtime consumer where applicable
✓ tooling contract
✓ positive tests
✓ negative tests
✓ boundary tests
✓ scalability tests
✓ determinism tests
✓ compatibility tests
✓ hard-coding audit

---

56. Classical IR Integration

A classical expression may lower through:

frontend AST
    ↓
semantic model
    ↓
classical IR

The expression grammar does not prescribe:

- instruction selection;
- register allocation;
- CPU architecture;
- SIMD width;
- cache layout;
- memory layout.

Those belong downstream.

---

57. Quantum IR Integration

A quantum-related expression must eventually lower through:

frontend AST
    ↓
semantic quantum model
    ↓
quantum::ir

The expression grammar must never create:

ExpressionQuantumIR
QuantumExpressionIR
GrammarQuantumIR

or another competing representation.

"quantum::ir" remains the canonical quantum semantic boundary.

---

58. HDL / Hardware IR Integration

Hardware-oriented expressions lower through the existing hardware/HDL semantic pipeline.

The grammar must not require:

fixed bit width
fixed bus width
fixed register count
fixed pipeline depth
fixed accelerator count

as universal language restrictions.

---

59. Resource Integration

Expression syntax may provide values used by resource requirements.

For example:

required_memory = n * element_size

may be a valid semantic computation.

The expression itself does not decide whether sufficient memory exists.

That belongs to resource analysis.

This distinction is essential for POCO-REAF.

---

60. Compilation Integration

The compiler may use expression semantics for:

- constant evaluation;
- specialization;
- optimization;
- lowering;
- vectorization;
- parallelization;
- distribution;
- accelerator mapping;
- quantum optimization;
- hardware generation.

But optimization must preserve observable expression semantics.

The grammar does not prescribe an implementation strategy.

---

61. Runtime Integration

Runtime behavior is downstream.

Expression parsing must not know:

- available processors;
- active devices;
- memory pressure;
- scheduler state;
- network topology;
- calibration state.

Runtime may use semantic information produced from expressions, but the parser must remain independent.

---

62. Constant Evaluation

Constant expressions may be evaluated by a separate semantic/compile-time subsystem.

The grammar must not execute them while parsing.

For example:

let x = 2 + 3;

may later be constant-folded to "5".

That does not mean the parser executes arithmetic.

---

63. Overflow

Overflow is not a grammar concern.

The expression grammar accepts structurally valid literals and arithmetic syntax.

The semantic/type system determines:

- representability;
- arbitrary precision;
- overflow policy;
- checked arithmetic;
- wrapping;
- saturation;
- symbolic arithmetic.

No fixed machine integer width should be silently imposed by the grammar.

---

64. Numeric Scalability

The expression grammar must not impose artificial numeric limits such as:

MAX_INT
MAX_FLOAT_DIGITS
MAX_LITERAL_BITS
MAX_EXPONENT
MAX_DECIMAL_PLACES

unless such a limit is an explicit language specification requirement rather than an implementation limitation.

The parser must distinguish:

lexically invalid number

from:

semantically unrepresentable number for a chosen type

---

65. Collection Scalability

Expression syntax must support structurally repeatable:

[a, b, c, ...]
(a, b, c, ...)
{...}

without enumerated element-count limits.

No expression grammar may encode:

MAX_TUPLE_ELEMENTS
MAX_ARRAY_ELEMENTS
MAX_ARGUMENTS
MAX_INDEXES

as a universal language limit.

Actual resource exhaustion remains an implementation concern.

---

66. Call Scalability

Function calls must support arbitrary argument lists permitted by available resources.

The grammar must not encode:

MAX_ARGUMENTS = 16

or another arbitrary universal ceiling.

Semantic/type checking determines whether a particular callable accepts the supplied arguments.

---

67. Indexing Scalability

Index expressions must support repeated/multi-dimensional forms where the language specification permits them.

No artificial universal maximum on:

index dimensions
index nesting
slice depth

may be encoded.

The semantic/type system determines whether a value is indexable.

---

68. Postfix Scalability

Postfix chains must remain structurally repeatable.

Examples:

a.b.c.d
f(a)(b)(c)
value[i][j][k]
object.method(a).field[index]

must not be rejected merely because a fixed parser-level nesting number has been reached.

Implementation stack/resource limits must not become language semantics.

---

69. Prefix Scalability

Nested prefix expressions must not have a language-level arbitrary limit.

For example:

!!!!value

is structurally repeatable subject to the language's actual operator semantics.

The implementation must still protect itself against resource exhaustion, but those implementation protections must not alter the normative language model.

---

70. Expression Depth

There must be no arbitrary grammar constant such as:

MAX_EXPRESSION_DEPTH = 1024

embedded into the language definition.

An implementation may have a configurable resource budget to prevent denial-of-service conditions.

Such a budget is:

implementation policy

not:

language semantics

---

71. Security Resource Limits

A parser implementation may enforce operational limits for:

- memory;
- wall-clock time;
- recursion;
- token count;
- source size.

Such limits must be:

- explicit;
- configurable where appropriate;
- documented;
- distinguishable from syntax errors;
- incapable of changing the meaning of valid programs within supported resource budgets.

They must never be encoded as universal grammar limits.

---

72. Deterministic Resource Handling

Resource exhaustion must produce deterministic diagnostics for the same configured parser policy.

A parser must not behave differently merely because:

machine A has 8 GB RAM
machine B has 128 GB RAM

unless the implementation explicitly uses an external resource budget.

The language semantics remain unchanged.

---

73. Domain Neutrality

The expression subsystem must be shared by:

classical
quantum
hybrid
HDL
hardware
embedded
systems
distributed
parallel/HPC
AI/ML
data
networking
cryptography
scientific computing
edge
cloud
future computational domains

A new domain should normally consume existing expression constructs rather than fork the expression language.

---

74. Domain Extension Rule

A domain may introduce new expression syntax only when:

1. the syntax has genuine language-level semantics;
2. ordinary expressions cannot express the required structure;
3. the syntax is specified;
4. the AST mapping exists;
5. semantic rules exist;
6. IR mapping exists;
7. compiler consumers exist;
8. tests exist;
9. the extension does not duplicate existing expression operators;
10. portability remains intact.

---

75. Generic Operation Rule

Prefer:

operation(args...)

over:

specialKeywordForEveryOperation

when the difference is semantic/library-level rather than grammatical.

This is especially important for:

- quantum gates;
- mathematical operations;
- AI operations;
- accelerator operations;
- vendor operations;
- hardware primitives.

This allows the language to scale without requiring a grammar edit for every new operation introduced by future technology.

---

76. Quantum Operation Rule

The expression grammar must not enumerate known quantum gates.

Do not create grammar alternatives such as:

H
X
Y
Z
CNOT
CX
RX
RY
RZ

as the universal quantum expression grammar.

Operation identity belongs to semantic data.

This permits:

H(q)
custom_gate(q)
vendor.operation(q)
operation(theta)(q)

without changing the core expression grammar.

---

77. Future-Proofing

The expression subsystem must allow future computational paradigms without requiring a redesign of the universal expression hierarchy.

Examples include:

- new quantum operation families;
- new accelerators;
- new tensor models;
- new distributed execution models;
- new hardware generation systems;
- new AI paradigms;
- new memory models;
- new computational substrates.

Future domains should normally attach semantics downstream of the same expression AST.

---

78. Interoperability

Expressions may be lowered to:

- classical IR;
- "quantum::ir";
- HDL/hardware IR;
- other canonical domain representations.

External formats such as:

- OpenQASM;
- QIR;
- LLVM;
- MLIR;
- HDL formats;
- foreign-language interfaces

are interoperability targets.

They are not alternate canonical Zamani expression ASTs.

---

79. Tooling

The expression contract must support:

- formatter;
- syntax highlighting;
- parser diagnostics;
- IDE navigation;
- semantic highlighting;
- refactoring;
- source mapping;
- language server support;
- documentation generation.

Tooling must consume the same canonical expression model.

It must not maintain a second precedence implementation.

---

80. Formatter Requirement

A formatter must preserve semantic grouping.

For example:

a + b * c

must not be reformatted as though it meant:

(a + b) * c

Parentheses may be inserted when needed to make grouping explicit.

Formatter precedence must derive from the same authoritative precedence contract.

---

81. Pretty-Printer Round Trip

Where a Zamani expression can be serialized back into source, the following must hold:

parse
    ↓
AST
    ↓
print
    ↓
parse

must preserve semantic structure.

Tests must compare AST/semantic structure rather than only raw text where formatting differences are permitted.

---

82. Round-Trip Requirement

For every stable expression construct:

source
→ lexer
→ parser
→ AST
→ formatter/printer
→ parser

must preserve:

- operator;
- operands;
- nesting;
- associativity;
- range boundaries;
- conditional branches;
- calls;
- indexing;
- member access;
- literal values.

---

83. Negative Tests

The expression subsystem must test malformed constructs.

Examples include:

a +
a *
a /
a ? b
a ? b :
a .. 
.. 
a == b == c
a < b < c
a = = b
a && || b
f(
a[
a.b.

The exact accepted/rejected set must follow the normative syntax specification.

Negative tests must verify deterministic diagnostics.

---

84. Boundary Tests

Boundary tests must include:

empty source where expression is required
single literal
single identifier
deeply nested parentheses
deep postfix chain
deep unary chain
large argument list
large tuple
large collection
large range
nested conditionals
nested calls
nested indexes
mixed operators
mixed domains

No test may encode an arbitrary "maximum supported" value as the language definition.

---

85. Scalability Tests

Scalability tests must demonstrate that the grammar is structurally independent of fixed machine capacity.

Test families should include generated expressions with increasing:

operand count
operator count
nesting depth
argument count
index dimensions
range complexity
conditional depth
call-chain depth
collection size

The test harness may impose practical CI resource limits.

Those CI limits must not become language rules.

---

86. Determinism Tests

For identical source and parser configuration:

parse(source)

must produce the same structural result.

Run repeated parses and compare:

- token sequence;
- AST;
- source spans;
- diagnostics.

No hardware discovery may affect the result.

---

87. Compatibility Tests

Every stable expression construct requires tests against the supported language-version matrix.

At minimum:

current version
previous compatible version

where those versions are supported by the repository.

Changes to:

- precedence;
- associativity;
- operator spelling;
- range syntax;
- conditional syntax

must be treated as language compatibility changes.

---

88. Lexer/Parser Conformance

The expression grammar must be tested against the actual canonical lexer.

For every operator:

source spelling
→ token
→ parser rule
→ AST operator

must be verified.

The test must fail when:

- token is missing;
- token name is wrong;
- token spelling is ambiguous;
- parser expects an obsolete alias.

---

89. Grammar/AST Conformance

Every expression parser rule must map to an existing AST representation.

The validator must report:

grammar rule with no AST mapping

as incomplete.

Likewise:

AST expression node with no grammar/source mapping

must be reported.

This prevents syntax and AST from drifting apart.

---

90. AST/Semantic Conformance

Every AST expression node must have a semantic contract.

No node may depend on:

«semantic behavior to be decided later.»

The contract must identify:

- type checking;
- name resolution;
- effects;
- capabilities;
- resource requirements;
- domain lowering.

---

91. Semantic/IR Conformance

Every expression that has semantic meaning must identify its IR destination.

Possible destinations include:

classical IR
quantum::ir
HDL/hardware IR
dataflow representation
distributed representation

A construct may lower differently depending on semantic context.

The grammar must not choose the backend.

---

92. No Duplicate Quantum IR

The expression subsystem must never introduce:

ExpressionQuantumIR
QuantumExpressionIR
QuantumGrammarIR

The canonical quantum boundary remains:

quantum::ir

All quantum expression semantics must eventually integrate there.

---

93. Compiler Integration

The compiler must consume semantic/IR information rather than raw grammar rules.

The expression grammar must not directly invoke:

- optimizer;
- router;
- scheduler;
- QEC;
- ZQN;
- HAL;
- backend.

The compiler pipeline remains responsible for those stages.

---

94. Runtime Integration

Runtime receives compiled semantic/IR representations.

It must not parse expression syntax as a substitute for compilation.

If runtime interpretation is supported, it must use the same semantic model and preserve the same expression semantics.

---

95. No Hardware-Dependent Parsing

The following must never influence expression parsing:

number of CPUs
number of cores
number of threads
number of GPUs
number of FPGAs
number of QPUs
number of qubits
memory capacity
network topology
accelerator count
device identifiers

This is a mandatory POCO-REAF invariant.

---

96. Hard-Coding Audit

Every expression grammar file must be checked for:

MAX_
MIN_
DEFAULT_
FIXED_
LIMIT_
COUNT_
WIDTH_
SIZE_
QUANTUM_
CPU_
GPU_
FPGA_
QPU_
NODE_
THREAD_
REGISTER_
MEMORY_

Occurrences are not automatically forbidden because legitimate semantic names may exist.

Each occurrence must be classified:

language semantic constant
implementation constant
test fixture
documentation example
forbidden hardware limit

A universal hardware/resource ceiling is a conformance failure.

---

97. No Hidden Limits

Do not replace explicit hard-coded limits with disguised grammar enumeration.

Forbidden examples:

argument
    : expr
    | expr COMMA expr
    | expr COMMA expr COMMA expr
    ...

when the grammar is intended to support arbitrary argument lists.

Correct:

argumentList
    : expression (COMMA expression)*
    ;

Likewise, do not enumerate:

qubit0
qubit1
qubit2
...

to create an apparent scalable quantum grammar.

---

98. Repetition Principle

Whenever a construct is conceptually unbounded, use structural repetition:

*
+

or equivalent grammar composition.

Do not encode a finite list of possible sizes.

This applies to:

- arguments;
- operands;
- indexes;
- tuple elements;
- collection elements;
- conditional branches;
- postfix chains;
- operator chains;
- domain lists;
- generic arguments.

---

99. Parser Performance

Production grammar must avoid pathological ambiguity.

Validation must include:

- ambiguity detection;
- unreachable rule detection;
- duplicate rule detection;
- left-recursion validation;
- token conflict detection;
- precedence validation.

Performance optimizations must not change language semantics.

---

100. Resource Exhaustion Protection

The parser implementation may use safe resource-budget mechanisms to protect against maliciously large inputs.

Such mechanisms must be:

- safe Rust;
- deterministic;
- explicitly documented;
- separate from language semantics.

A resource exhaustion failure must not be reported as:

«invalid Zamani syntax»

when the syntax itself is valid but the configured parser budget was exceeded.

---

101. No Unsafe Parser Implementation

The expression parser and its supporting Rust code must not require:

unsafe

for:

- token handling;
- AST construction;
- expression parsing;
- source spans;
- diagnostics;
- precedence handling;
- parser state.

Rust 1.97/1.97.1 safe abstractions must be used.

---

102. Repository Integration

The expression subsystem must integrate with:

grammar/Zamani.g4
grammar/specification/syntax.md
grammar/specification/semantics.md
grammar/spec/syntax.md
grammar/spec/type-system.md
grammar/spec/lexical.md

grammar/antlr/ZamaniLexer.g4

grammar/lexer/tokens.md
grammar/lexer/operators.md
grammar/lexer/precedence.md

grammar/expressions/expressions.g4
grammar/expressions/conditionals.g4
grammar/expressions/ranges.g4
grammar/expressions/range.g4
grammar/expressions/arithmetic.g4
grammar/expressions/comparison.g4
grammar/expressions/logical.g4
grammar/expressions/bitwise.g4
grammar/expressions/shift.g4
grammar/expressions/unary.g4
grammar/expressions/postfix.g4
grammar/expressions/calls.g4
grammar/expressions/indexing.g4
grammar/expressions/member-access.g4

src/lexer.rs
src/parser.rs
src/frontend/ast/

and downstream:

semantic analysis
ZUIR/canonical semantic model
classical IR
quantum::ir
HDL/hardware IR
optimization
routing
scheduling
resilience
QEC
ZQN
HAL
compiler
runtime

The expression subsystem does not own those downstream implementations.

---

103. Cross-Domain Integration

The same expression AST must be consumable by:

classical
quantum
hybrid
HDL
hardware
AI
data
distributed
networking
security
scientific
embedded
systems
future domains

A domain integration must specify:

expression construct
↓
AST representation
↓
semantic interpretation
↓
domain IR
↓
consumer

It must not introduce a second expression language.

---

104. Example: Classical

Source:

let result = a + b * c;

Expected structural interpretation:

let result = (a + (b * c));

because multiplication binds more tightly than addition.

---

105. Example: Assignment

Source:

a = b + c * d;

Structure:

a = (b + (c * d))

not:

(a = b) + (c * d)

---

106. Example: Conditional

Source:

result = condition ? a + b : c * d;

Structure:

result =
    condition
        ? (a + b)
        : (c * d)

---

107. Example: Range

Source:

values[start + offset .. limit * scale]

The range bounds use their permitted lower expression hierarchy:

(start + offset)
..
(limit * scale)

The range does not absorb an outer assignment.

---

108. Example: Quantum

Source:

apply(operation(theta), q[index]);

The expression layer provides:

call
call
index

It does not determine:

- whether "operation" is quantum;
- whether "q" is a quantum register;
- whether "index" identifies a logical qubit;
- how that qubit maps physically;
- how the operation is scheduled.

Those are semantic and downstream concerns.

---

109. Example: Hardware

Source:

width - 1

The expression grammar does not decide whether "width" is:

- FPGA bus width;
- memory width;
- tensor dimension;
- software array length;
- quantum register size.

The type/semantic context decides.

---

110. Example: Distributed

Source:

partition_start .. partition_end

The expression grammar represents the range.

It does not decide:

- node count;
- placement;
- network topology;
- sharding strategy.

Those belong to distributed/resource/compile/runtime systems.

---

111. Example: AI/Data

Source:

tensor[batch_start .. batch_end, feature_start .. feature_end]

The expression grammar provides:

index
range
range

It does not impose:

maximum batch size
maximum tensor rank
maximum feature count
maximum accelerator count

---

112. Example: HDL

Source:

signal[index + offset]

The expression grammar provides arithmetic and indexing.

HDL semantics determine:

- signal type;
- width;
- timing;
- synthesis meaning.

---

113. Integration With "grammar.md"

"grammar/grammar.md" must describe what the implementation actually accepts.

It must not redefine expression precedence independently.

The authoritative relationship is:

specification
    ↓
expression conformance
    ↓
canonical grammar
    ↓
implementation
    ↓
grammar.md

If "grammar.md" conflicts with this contract, the implementation or generated reference must be corrected.

This file is not updated merely to rationalize an incorrect parser.

---

114. Integration With "Zamani-Grammar.md"

"grammar/Zamani-Grammar.md" may contain broader proposed or historical expression concepts.

Those concepts become stable language syntax only after passing:

specification
↓
expression contract
↓
grammar
↓
AST
↓
semantics
↓
IR
↓
tests

A feature appearing in "Zamani-Grammar.md" does not automatically become legal expression syntax.

---

115. Integration With "Zamani.g4"

"grammar/Zamani.g4" is the composition root.

It must expose the canonical expression entry:

expression

exactly once.

It must not copy the complete expression hierarchy into another set of rules.

Domain grammar components must connect to the canonical expression layer.

---

116. Integration With the Lexer

The canonical lexer:

grammar/antlr/ZamaniLexer.g4

owns tokenization.

The expression grammar consumes its tokens.

The repository's lexer documentation already identifies this file as the canonical lexical grammar.

The parser must not create alternate token identities.

---

117. Integration With "src/lexer.rs"

"src/lexer.rs" must remain lexically conformant with the canonical grammar.

For every expression operator:

ANTLR lexer token
↔
Rust lexer token

must be checked.

A mismatch is a repository-level conformance failure.

---

118. Integration With "src/parser.rs"

"src/parser.rs" must produce the same precedence and associativity specified here.

The parser implementation must not silently introduce another precedence table.

If the Rust parser uses a Pratt parser or precedence-climbing implementation, its binding powers must be derived from this contract.

---

119. Integration With "src/frontend/ast/"

The AST must remain domain-neutral.

Expression parsing must not require:

QuantumGate
GpuOperation
FpgaOperation
CpuOperation

just to represent syntax.

Generic operations and expression nodes are preferred.

Domain-specific semantic interpretation happens downstream.

---

120. Integration With Semantic Analysis

Semantic analysis must consume the expression AST and determine:

meaning
types
effects
capabilities
resources
domain
validity

It must not require grammar files to contain semantic implementation.

---

121. Integration With Optimization

Optimization may transform expression-derived IR.

Examples:

constant folding
algebraic simplification
dead computation elimination
vectorization
parallelization
quantum optimization

Optimization must preserve language semantics.

The grammar must not encode optimization decisions.

---

122. Integration With Scheduling

Expression syntax must not select:

CPU core
GPU stream
QPU
physical qubit
FPGA resource
network node

Scheduling determines execution ordering/resource allocation after semantic lowering.

---

123. Integration With Routing

For quantum/hardware expressions, routing may map abstract operations/resources to physical topology.

Expression syntax remains target-independent.

---

124. Integration With QEC

Quantum expression syntax can express quantum computation.

QEC determines error-correction realization.

The expression grammar must not encode:

specific physical-code layout
fixed code distance
fixed number of ancillas
fixed physical qubit map

as universal syntax.

---

125. Integration With ZQN

ZQN remains responsible for fault/noise semantics.

Expression grammar does not implement noise models.

Expressions may provide semantic inputs to those systems, but do not duplicate their IR.

---

126. Integration With HAL

HAL exposes actual device capabilities/state.

Expression parsing must not inspect HAL.

HAL must consume downstream semantic/compiled representations.

---

127. Integration With Runtime

Runtime receives compiled representations.

It must not reinterpret expression syntax differently depending on the target.

The same source-level expression semantics must survive target lowering.

---

128. Versioning

Expression syntax is versioned as part of Zamani.

Changes to:

- precedence;
- associativity;
- operator spelling;
- range syntax;
- conditional syntax;
- expression forms

must be versioned and documented.

A compatibility change must not be silently introduced by editing a grammar file.

---

129. Feature Lifecycle

Every new expression feature follows:

proposal
    ↓
specification
    ↓
lexical contract
    ↓
expression conformance
    ↓
grammar
    ↓
AST
    ↓
semantic model
    ↓
IR
    ↓
compiler/runtime
    ↓
tests
    ↓
stable

A feature is not production merely because its ".g4" file parses.

---

130. Required Test Matrix

The expression subsystem requires:

tests/
    expressions/
        lexical/
        precedence/
        associativity/
        assignment/
        conditionals/
        ranges/
        logical/
        bitwise/
        comparison/
        arithmetic/
        unary/
        postfix/
        calls/
        indexing/
        member-access/
        literals/
        mixed/
        quantum/
        classical/
        hybrid/
        hdl/
        hardware/
        ai/
        data/
        distributed/
        networking/
        negative/
        boundary/
        scalability/
        determinism/
        compatibility/

Existing directories/files must be reused where they already serve the purpose.

Do not create duplicate test structures unnecessarily.

---

131. Required Precedence Tests

At minimum:

a + b * c
a * b + c
a << b + c
a < b == c
a & b == c
a && b || c
a .. b
a ? b : c
a = b + c

must have explicit AST/grouping expectations.

---

132. Required Associativity Tests

Test:

a + b + c
a * b * c
a << b << c
a && b && c
a || b || c
a = b = c
a ? b : c ? d : e

with exact expected grouping.

---

133. Required Non-Associativity Tests

Test:

a < b < c
a <= b <= c
a == b == c
a != b != c
a .. b .. c

according to the normative language rule.

No accidental left-association may be introduced simply because an ANTLR "(...)*" repetition happens to be convenient.

---

134. Mixed-Domain Tests

The same expression structure must be tested in:

classical
quantum
hybrid
HDL
AI
data
distributed

contexts where those contexts are supported.

The AST structure should remain generic.

---

135. Repository-Wide Reference Search

Before marking the expression subsystem complete, search the repository for:

expression
conditionalExpression
rangeExpression
assignmentExpression
precedence
EQ_EQ
NOT_EQ
LESS
LE
GREATER
GE
SHIFT_LEFT
SHIFT_RIGHT
DOT_DOT
DOT_DOT_EQ

and verify every reference points to the correct authority.

No stale documentation may claim that a removed/obsolete file is canonical.

---

136. Generated Artifact Rule

Generated parser/lexer artifacts must never become a second source of truth.

If generated files are checked into the repository, they must be explicitly marked generated.

They must be reproducible from the canonical grammar.

Manual edits to generated expression parsers are prohibited.

---

137. Build Reproducibility

Expression grammar generation must be reproducible from:

canonical lexer
canonical parser grammar
language version
ANTLR toolchain version
generation configuration

The same inputs must produce equivalent parser artifacts.

---

138. No Environment-Dependent Grammar

Grammar generation must not change based on:

- host CPU;
- host GPU;
- installed QPU;
- operating system hardware;
- memory size;
- current directory;
- network availability.

Build configuration may select supported targets, but expression syntax remains unchanged.

---

139. Completion Criteria for "expressions.g4"

"expressions.g4" is complete only when:

[ ] owns expression composition
[ ] does not duplicate conditional grammar
[ ] does not duplicate range grammar
[ ] has one assignment boundary
[ ] has one precedence hierarchy
[ ] references canonical lexer tokens
[ ] has no lexer rules
[ ] has no semantic actions
[ ] has no hardware assumptions
[ ] has no fixed resource limits
[ ] has AST mapping
[ ] has semantic mapping
[ ] has IR mapping
[ ] has parser integration
[ ] has positive tests
[ ] has negative tests
[ ] has boundary tests
[ ] has scalability tests
[ ] has determinism tests
[ ] has compatibility tests

---

140. Completion Criteria for "conditionals.g4"

"conditionals.g4" is complete only when:

[ ] owns conditionalExpression
[ ] owns structured if expressions
[ ] owns ternary expressions
[ ] does not define statement conditionals
[ ] has no duplicate conditional grammar
[ ] has an ANTLR-safe composition boundary
[ ] has explicit ternary associativity
[ ] has source-span contract
[ ] has AST contract
[ ] has semantic contract
[ ] has IR integration contract
[ ] has positive tests
[ ] has negative tests
[ ] has boundary tests
[ ] has scalability tests
[ ] has determinism tests
[ ] has compatibility tests

---

141. Completion Criteria for "ranges.g4"

"ranges.g4" is complete only when:

[ ] is the single active range grammar
[ ] uses canonical lexer tokens
[ ] preserves endpoint presence
[ ] supports inclusive/exclusive upper bounds
[ ] has stable precedence boundary
[ ] does not consume unrestricted expression recursively
[ ] has AST contract
[ ] has semantic contract
[ ] has classical integration
[ ] has quantum integration
[ ] has HDL integration
[ ] has data/AI integration
[ ] has distributed integration
[ ] has resource integration
[ ] has no physical mapping
[ ] has no fixed range limit
[ ] has positive tests
[ ] has negative tests
[ ] has boundary tests
[ ] has scalability tests
[ ] has determinism tests
[ ] has compatibility tests

---

142. Completion Criteria for "range.g4"

"range.g4" is complete only if it has exactly one of these roles:

A. active canonical owner
B. compatibility/reference wrapper
C. historical documentation
D. removed because redundant

It must not remain an independently active duplicate of "ranges.g4".

The repository must choose one role and document it.

Given the current architecture, "ranges.g4" should be the active modular range grammar and "range.g4" should not independently define the same public rule.

---

143. Completion Criteria for the Entire Expression Subsystem

The subsystem is production-ready only when:

[ ] exactly one expression authority
[ ] exactly one conditional authority
[ ] exactly one range authority
[ ] exactly one assignment authority
[ ] exactly one precedence hierarchy
[ ] exactly one lexical authority
[ ] no parser token aliases
[ ] no duplicate public rules
[ ] no competing expression grammars
[ ] domain-neutral AST
[ ] semantic separation
[ ] canonical IR integration
[ ] quantum::ir integration
[ ] no second quantum IR
[ ] no hardware limits
[ ] no fixed resource limits
[ ] POCO-REAF preserved
[ ] deterministic parsing
[ ] safe Rust
[ ] Rust 1.97/1.97.1 compatibility
[ ] source spans
[ ] diagnostics
[ ] error recovery contract
[ ] formatter compatibility
[ ] round-trip tests
[ ] positive tests
[ ] negative tests
[ ] boundary tests
[ ] scalability tests
[ ] determinism tests
[ ] compatibility tests
[ ] repository-wide reference validation
[ ] generated artifacts reproducible

---

144. Final Production Architecture

The final expression subsystem is:

                    Zamani Source
                          |
                          v
                canonical lexer
                          |
                          v
                expression parser
                          |
                          v
              +----------------------+
              |  Universal Expression|
              |      Language         |
              +----------------------+
                          |
             +------------+-------------+
             |            |             |
             v            v             v
          classical     quantum       HDL
             |            |             |
             |       quantum::ir       |
             |            |             |
             +------------+-------------+
                          |
                          v
                  semantic analysis
                          |
                          v
                 canonical semantic
                       model
                          |
             +------------+-------------+
             |            |             |
             v            v             v
          classical    quantum       hardware/
             IR           IR            HDL IR
             |            |             |
             +------------+-------------+
                          |
                          v
                    optimization
                          |
                 +--------+--------+
                 |        |        |
                 v        v        v
              routing scheduling resilience
                          |
                          v
                         QEC
                          |
                          v
                         ZQN
                          |
                          v
                         HAL
                          |
                          v
                 target realization
                          |
        +---------+-------+-------+---------+
        |         |       |       |         |
       CPU       GPU     FPGA    QPU      Future

The key invariant is:

ONE LANGUAGE
ONE EXPRESSION MODEL
ONE DOMAIN-NEUTRAL AST
ONE SEMANTIC BOUNDARY
ONE CANONICAL QUANTUM IR
MANY TARGETS

not:

one grammar per machine
one grammar per domain
one AST per backend
one IR per quantum frontend

---

145. Final POCO-REAF Invariant

A valid Zamani expression describes what computation means, not the accidental characteristics of today's machine.

Therefore:

Program
   ↓
Parse once
   ↓
Compile semantic meaning once
   ↓
Resolve capabilities/resources at deployment
   ↓
Lower to target
   ↓
Execute

must remain possible without rewriting the source expression merely because the available machine changes.

The expression grammar must therefore remain:

unbounded in language structure
target-independent
resource-independent
domain-neutral
deterministic
composable
semantically typed downstream
IR-integrable
quantum::ir compatible
safe-Rust implementable

with no artificial universal limits.

---

146. Definition of Done

"grammar/expressions/conformance.md" is satisfied when every expression grammar file can answer all of these questions before implementation is considered complete:

What syntax does this file own?

What syntax does it explicitly not own?

Which lexer tokens does it consume?

Where is precedence defined?

Where is associativity defined?

What exact AST node receives the construct?

What source spans are preserved?

What semantic rules consume the AST node?

Which canonical IR receives it?

Which compiler subsystem consumes that IR?

Which runtime subsystem eventually consumes it?

Which domain-specific systems may consume it?

How does quantum syntax reach quantum::ir?

How does the construct remain target-independent?

What prevents hardware limits from entering the grammar?

What happens for malformed input?

What happens at semantic failure?

What happens at resource exhaustion?

What are the positive tests?

What are the negative tests?

What are the boundary tests?

What are the scalability tests?

What are the determinism tests?

What are the compatibility tests?

What prevents another file from defining the same construct?

What proves the feature works with Rust 1.97/1.97.1?

What proves the implementation requires no unsafe Rust?

If any answer is missing, that feature is not production-complete.

The expression subsystem is complete only when these contracts are satisfied before downstream files are allowed to redefine or reinterpret the same syntax.

---

147. Non-Negotiable Rules

The following are permanent expression architecture invariants:

1. "expression" has one canonical authority.
2. Precedence has one canonical authority.
3. The lexer has one canonical authority.
4. "conditionalExpression" has one canonical authority.
5. "rangeExpression" has one canonical authority.
6. Specialized grammars consume universal expressions rather than recreating them.
7. Expressions lower to the existing domain-neutral AST.
8. Expressions do not create a second quantum IR.
9. Quantum semantics ultimately cross the "quantum::ir" boundary.
10. Grammar never performs hardware discovery.
11. Grammar never chooses physical devices.
12. Grammar never hard-codes resource ceilings.
13. Grammar never hard-codes qubit limits.
14. Grammar never hard-codes CPU/GPU/FPGA/node limits.
15. Grammar never executes user code.
16. Parser behavior is deterministic.
17. Parser implementation is safe Rust.
18. Rust 1.97/1.97.1 remains supported.
19. Every feature has an advance integration contract.
20. Every production feature has complete conformance tests.
21. Existing filenames are preserved unless there is a concrete architectural reason to remove a duplicate.
22. Duplicate files are resolved by ownership, not by maintaining competing implementations.
23. "Zamani-Grammar.md" cannot silently introduce syntax.
24. "grammar.md" cannot become a second authority.
25. POCO-REAF remains a language-level architectural invariant.

---

Final expression ownership

The intended final ownership is therefore:

grammar/expressions/
│
├── conformance.md       ← THIS CONTRACT
│
├── expressions.g4       ← universal expression composition
│
├── conditionals.g4      ← conditional expressions
│
├── ranges.g4            ← canonical range expressions
│
├── range.g4             ← compatibility/reference only;
│                           never competing active authority
│
├── assignment.g4        ← assignment details where retained
├── arithmetic.g4        ← arithmetic details
├── comparison.g4        ← comparison details
├── logical.g4           ← logical details
├── bitwise.g4           ← bitwise details
├── shift.g4             ← shift details
├── unary.g4             ← unary details
├── postfix.g4           ← postfix details
├── calls.g4             ← calls
├── indexing.g4          ← indexing
├── member-access.g4    ← member access
├── literals.g4          ← expression literals
└── ...

Every one of those files must plug into the same contract rather than becoming another expression language.

That is the production boundary for "grammar/expressions/": syntax is centralized, semantics are downstream, quantum meaning crosses "quantum::ir", and no target/resource limitation becomes a language limitation.
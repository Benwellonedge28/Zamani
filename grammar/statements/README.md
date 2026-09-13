Zamani Statement Grammar

Status

Production architecture specification

This directory defines the statement-level syntax of the Zamani programming language.

The statement grammar is part of the canonical Zamani frontend grammar and is designed for:

- classical computing;
- quantum computing;
- hybrid quantum-classical computing;
- HDL;
- hardware/software co-design;
- embedded computing;
- parallel computing;
- distributed computing;
- HPC;
- AI/ML;
- accelerator programming;
- networking;
- scientific computing;
- systems programming;
- future computational domains.

The statement grammar is intentionally independent of the physical machine on which a program eventually executes.

The governing language objective is:

«Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)»

The source language describes computation and intent. Target realization is performed by downstream semantic, compilation, scheduling, routing, hardware, and runtime layers.

---

1. Purpose

"grammar/statements/" owns the syntactic representation of statements.

It answers:

«What syntactic forms may occur where Zamani expects a statement?»

It does not answer:

«How is that statement implemented on a particular machine?»

Those are different architectural responsibilities.

The intended pipeline is:

Zamani source
    |
    v
canonical lexer
    |
    v
statement grammar
    |
    v
parse tree
    |
    v
frontend AST
    |
    v
semantic analysis
    |
    +--> names
    +--> types
    +--> effects
    +--> capabilities
    +--> ownership
    +--> control-flow analysis
    +--> resource validation
    |
    v
canonical semantic representation
    |
    +--> classical representation
    +--> quantum::ir
    +--> HDL representation
    +--> hardware representation
    +--> distributed representation
    +--> accelerator representation
    +--> future-domain representation
    |
    v
optimization
    |
    v
routing / scheduling / lowering
    |
    v
target realization
    |
    v
runtime / hardware

The grammar must never bypass this architecture.

---

2. Fundamental ownership rule

This directory owns syntax only.

It must not become a semantic execution subsystem.

Statement grammar owns

- statement composition;
- statement-family dispatch;
- block structure;
- conditional statement syntax;
- loop statement syntax;
- assignment statement syntax;
- declaration-statement syntax;
- return syntax;
- break syntax;
- continue syntax;
- exception syntax;
- assertion syntax;
- pattern-matching statement syntax;
- unsafe-region syntax;
- future statement families through explicit grammar extension.

Statement grammar does not own

- AST implementation;
- semantic analysis;
- type checking;
- ownership checking;
- borrowing;
- capability evaluation;
- effect execution;
- resource allocation;
- resource discovery;
- hardware discovery;
- hardware calibration;
- quantum compilation;
- quantum IR;
- QEC algorithms;
- ZQN fault/noise semantics;
- resilience decisions;
- optimization;
- routing;
- scheduling;
- target selection;
- runtime execution;
- device selection;
- machine topology;
- physical qubit allocation;
- CPU/core/thread allocation;
- GPU allocation;
- FPGA allocation;
- network deployment.

This ownership boundary is mandatory.

---

3. Canonical statement composition

"grammar/statements/statements.g4" is the single composition owner for statements.

There must be exactly one authoritative effective "statement" rule in the assembled production parser.

Individual statement grammars must not independently redefine the canonical "statement" rule.

The architecture is:

statement
    |
    +--> expressionStatement
    |
    +--> assignmentStatement
    |
    +--> declarationStatement
    |
    +--> assertionStatement
    |
    +--> conditionalStatement
    |
    +--> loopStatement
    |
    +--> patternMatchStatement
    |
    +--> returnStatement
    |
    +--> breakStatement
    |
    +--> continueStatement
    |
    +--> exceptionStatement
    |
    +--> unsafeStatement
    |
    +--> blockStatement
    |
    +--> future statement families

The exact composition must correspond to the actual assembled grammar.

No statement family should become authoritative merely because it defines a rule named "statement".

---

4. Current statement components

The production statement directory currently contains the following architectural components:

grammar/statements/
├── README.md
├── statements.g4
├── blocks.g4
├── declarations.g4
├── assignments.g4
├── conditionals.g4
├── loops.g4
├── pattern-matching.g4
├── returns.g4
├── breaks.g4
├── continues.g4
├── assertions.g4
├── exceptions.g4
└── unsafe.g4

Each file has one primary responsibility.

---

5. "statements.g4"

Purpose

Canonical statement composition.

Owns

- "statement";
- statement-family dispatch;
- generic expression-statement admission;
- composition of statement categories.

Does not own

- concrete conditional syntax;
- concrete loop syntax;
- block syntax;
- assignment syntax;
- declaration syntax;
- return syntax;
- exception syntax;
- unsafe syntax;
- expression precedence.

Integration

It integrates with:

lexer
expressions/
statements/*
frontend parser
AST
semantic analysis

It must not directly depend semantically on:

quantum::ir
QEC
ZQN
resilience
routing
scheduling
hardware
runtime

The existing repository already treats this file as the canonical statement-composition boundary.

---

6. "blocks.g4"

Purpose

Own canonical block syntax.

Owns

- "{ ... }";
- block composition;
- statement sequences;
- block-level structural recursion.

Does not own

- individual statements;
- statement semantics;
- expression precedence;
- control-flow analysis.

The relationship is:

statement
    |
    v
block
    |
    v
statement*

This recursive relationship is intentional.

It is not a semantic dependency cycle.

Blocks must not duplicate statement definitions.

---

7. "conditionals.g4"

Purpose

Own statement-level conditional control flow.

Owns

- "if";
- "else if";
- "else".

Does not own

- conditional expressions;
- expression precedence;
- block syntax;
- boolean type semantics.

The repository already distinguishes statement-level conditionals from expression-level conditionals. That distinction must remain permanent.

Architecture:

statement
    |
    v
conditionalStatement
    |
    v
ifStatement
    |
    +--> expression
    +--> block
    +--> else-if*
    +--> else?

No finite maximum number of "else if" clauses may be encoded.

---

8. "loops.g4"

Purpose

Own statement-level iteration syntax.

Owns

- "while";
- "do while";
- "for";
- iterator-style "for";
- loop control syntax specific to the loop construct.

Does not own

- "break";
- "continue";
- iterator implementation;
- collection implementation;
- concurrency;
- scheduling;
- parallel execution.

The current loop grammar already follows the intended architecture by delegating expressions and blocks rather than defining another expression language.

The grammar must not encode:

MAX_ITERATIONS
MAX_LOOP_DEPTH
MAX_THREADS
MAX_CORES
MAX_WORKERS

or equivalent constants.

---

9. "assignments.g4"

Purpose

Own statement-level assignment syntax.

Owns

- assignment statement forms;
- assignment operators where statement-level;
- compound assignment syntax if part of the language.

Does not own

- assignment expression semantics;
- mutability checking;
- ownership;
- borrowing;
- type compatibility;
- storage allocation.

Assignment semantics are determined downstream.

The statement grammar must not duplicate assignment-expression precedence.

---

10. "declarations.g4"

Purpose

Own declarations that are legal in statement position.

Owns

- local declaration syntax;
- declaration-statement composition;
- integration with declaration grammar.

Does not own

The full declaration ecosystem.

Definitions belonging to:

grammar/declarations/
grammar/functions/
grammar/modules/
grammar/types/

must remain owned there.

"statements/declarations.g4" is an integration boundary, not a second declaration language.

---

11. "returns.g4"

Purpose

Own return-statement syntax.

Owns

return;
return expression;

and future return syntax approved by the language specification.

Does not own

- function validity;
- return-type checking;
- control-flow completeness;
- callable analysis.

For example:

return;

may be syntactically valid while being semantically invalid inside a non-unit-returning function.

That distinction belongs to semantic analysis.

---

12. "breaks.g4"

Purpose

Own "break" syntax.

Semantic boundary

The grammar does not determine whether a "break" is legal at its location.

For example:

break;

is syntactically recognizable.

Whether it occurs inside an appropriate breakable construct is a semantic/control-flow validation issue.

No grammar rule should attempt to encode arbitrary nesting depth.

---

13. "continues.g4"

Purpose

Own "continue" syntax.

Semantic boundary

The grammar recognizes the statement.

Semantic analysis determines whether the current control-flow context permits "continue".

No target-specific meaning is allowed.

---

14. "assertions.g4"

Purpose

Own statement-level assertion syntax.

Owns

- runtime/source assertions;
- assertion message syntax where specified;
- assertion statement structure.

Does not own

- proof;
- theorem proving;
- compile-time evaluation;
- runtime policy;
- optimization assumptions.

Compile-time assertions must remain separate from runtime statement assertions.

---

15. "exceptions.g4"

Purpose

Own exception/control-transfer syntax.

Owns

- "try";
- "catch";
- "finally";
- "throw";
- associated statement structure.

Does not own

- exception implementation;
- runtime unwinding;
- stack representation;
- hardware exception handling;
- recovery policy.

Exception semantics must be target-independent at source level.

---

16. "pattern-matching.g4"

Purpose

Own statement-level pattern matching.

Owns

- "match";
- cases;
- guards;
- pattern dispatch structure.

Does not own

- pattern type checking;
- exhaustiveness analysis;
- unreachable-case analysis;
- value semantics.

Pattern syntax must integrate with the canonical type and expression grammars.

---

17. "unsafe.g4"

Purpose

Own Zamani's explicit unsafe-region syntax.

Critical distinction

Zamani's "unsafe" construct is a Zamani language feature.

It is not Rust's "unsafe".

The grammar implementation itself must contain:

- no Rust "unsafe";
- no embedded Rust actions;
- no raw-pointer execution;
- no FFI execution;
- no host process execution.

Rust 1.97 / Rust 1.97.1 is the implementation baseline, but Rust implementation safety must remain independent of whether a Zamani program contains an "unsafe" region.

Therefore:

Zamani unsafe syntax
        ≠
Rust unsafe implementation

The grammar merely recognizes the source-language construct.

Semantic analysis must subsequently determine:

- what capability the unsafe region requests;
- what operations it permits;
- what restrictions apply;
- what provenance is required;
- whether the construct is permitted in the current compilation/security context.

---

18. Expression statements

A canonical expression must be allowed in statement position where the language specification permits it.

Examples include:

compute();
value;
measure(q);
tensor_operation();
hardware_operation();
distributed_operation();

The statement grammar must not duplicate expression syntax.

The architecture is:

expression
    |
    v
expressionStatement
    |
    v
statement

Expression precedence belongs exclusively to:

grammar/expressions/

---

19. Statement termination

Ordinary statement termination must have one authoritative definition.

The statement grammar must not independently invent multiple termination systems.

Unless the language specification explicitly changes this rule, ordinary statement forms use the canonical semicolon-based termination where required.

Automatic semicolon insertion must not be silently introduced.

Any future change to termination rules requires coordinated updates to:

grammar/specification/
grammar/lexer/
grammar/statements/
grammar/tests/
parser/frontend
compatibility documentation

---

20. No domain-specific statement forks

The statement grammar must not introduce separate syntax merely because a target differs.

Do not create constructs such as:

cpuStatement
gpuStatement
fpgaStatement
asicStatement
qpuStatement
clusterStatement
cloudStatement
embeddedStatement

merely to represent target differences.

A source-level operation should describe its semantic intent.

Target realization occurs downstream.

For example:

compute(data);

may ultimately be realized on:

CPU
GPU
FPGA
ASIC
quantum-classical accelerator
cluster
cloud infrastructure
future hardware

without changing the source statement merely because the target changed.

---

21. Quantum integration

Statement syntax may control quantum computation.

For example, a statement body may eventually contain quantum operations:

if ready {
    quantum_operation();
}

or:

while condition {
    measure();
}

The statement grammar must not know:

- number of qubits;
- physical qubit count;
- logical qubit count;
- topology;
- gate inventory;
- device identifier;
- backend;
- calibration;
- QEC implementation;
- ZQN noise model;
- routing;
- scheduling.

The pipeline is:

statement grammar
      |
      v
AST
      |
      v
semantic analysis
      |
      v
quantum semantic representation
      |
      v
quantum::ir
      |
      v
optimization
      |
      v
routing
      |
      v
scheduling
      |
      v
hardware realization

The grammar must never construct "quantum::ir".

---

22. QEC integration

QEC is not statement grammar ownership.

A statement may eventually invoke or control a QEC-related operation, but:

grammar/statements/

must not define QEC algorithms.

QEC owns:

- error detection;
- syndrome processing;
- correction;
- decoding;
- logical-error handling.

The statement grammar only recognizes the syntactic structure necessary to express the program.

---

23. ZQN integration

ZQN owns quantum noise/fault semantics.

Statement grammar must not define:

- noise channels;
- correlated faults;
- leakage;
- loss;
- erasure;
- calibration noise;
- fault probabilities.

A statement may semantically interact with ZQN-aware execution, but this is downstream.

---

24. Resilience integration

Resilience is a decision/orchestration layer.

Statement grammar must not encode:

retry
restart
rollback
reroute
reschedule
recompile
switch-backend
quarantine

as hardware recovery semantics merely because those actions exist downstream.

If Zamani eventually provides explicit source-level resilience constructs, those constructs must receive a dedicated language specification and grammar owner.

They must not be smuggled into generic statement rules.

---

25. Scheduling integration

Statements establish program-level ordering semantics.

They do not establish:

- gate timing;
- pulse timing;
- machine cycles;
- physical placement;
- resource reservations;
- queue positions;
- hardware schedules.

The downstream relationship is:

statement
    |
    v
semantic control flow
    |
    v
IR
    |
    v
optimization
    |
    v
routing
    |
    v
scheduling

Scheduling must preserve the semantics established by the source program.

---

26. Hardware and HDL integration

Statements may appear in hardware-oriented programs.

However, statement syntax must not encode fixed:

- register counts;
- FPGA resource counts;
- ASIC dimensions;
- clock counts;
- cores;
- memory sizes;
- buses;
- physical addresses;
- topology.

Hardware-specific semantics belong to:

grammar/hdl/
grammar/hardware/
grammar/resources/

and their downstream semantic/compiler layers.

---

27. Classical integration

Statements must support ordinary classical control flow without restricting future computational domains.

The same statement model must support:

scalar computation
vector computation
matrix computation
tensor computation
symbolic computation
numerical computation
parallel computation
accelerator computation

without introducing separate control-flow languages for each.

---

28. Concurrency and parallelism

Statement syntax may participate in concurrency.

However, statement grammar must not hard-code:

MAX_THREADS
MAX_TASKS
MAX_WORKERS
MAX_NODES
MAX_CHANNELS

Concurrency semantics belong to:

grammar/concurrency/

while execution/resource decisions belong downstream.

The source program expresses semantic concurrency.

The execution system decides how available resources realize it.

---

29. Distributed computing

Distributed execution must not require a separate statement grammar merely because execution crosses machines.

Statements must remain portable across:

local
embedded
single-machine
multicore
multi-device
cluster
cloud
distributed
future environments

Deployment topology belongs to resource, execution, and deployment layers.

---

30. Resource independence

No statement grammar file may contain machine-size constants.

Forbidden examples include:

MAX_QUBITS = 32
MAX_QUBITS = 64
MAX_CORES = 128
MAX_THREADS = 1024
MAX_DEVICES = 16
MAX_NODES = 1000

The grammar must also avoid hidden equivalents such as:

q[0]
q[1]
device0
device1
core0
core1
gpu0
gpu1

unless those are merely ordinary user identifiers and not language-imposed resources.

---

31. Resource semantics belong elsewhere

The language must distinguish:

requirement
constraint
capability
preference
hint
resource
target
placement

For example:

requires quantum;

must not inherently mean:

use device X;
use exactly N qubits;
use topology Y;

Statement syntax must remain independent of these physical choices.

---

32. POCO-REAF

The statement grammar is a fundamental part of POCO-REAF.

A statement represents source-level meaning.

It must not encode transient implementation characteristics.

The intended relationship is:

One source program
        |
        v
One semantic meaning
        |
        +----> CPU
        +----> GPU
        +----> FPGA
        +----> ASIC
        +----> QPU
        +----> simulator
        +----> cluster
        +----> cloud
        +----> heterogeneous system
        +----> future architecture

The program should not require syntactic rewriting merely because the available machine changes.

---

33. AST contract

The grammar produces parser contexts.

It does not construct Rust AST values.

The frontend AST layer must preserve, as appropriate:

- statement kind;
- source span;
- source order;
- child relationships;
- labels;
- expressions;
- declarations;
- block structure;
- syntactic metadata.

Every statement must map to exactly one canonical AST representation or to an explicitly documented desugaring.

No statement grammar file may silently create a second AST model.

---

34. Semantic contract

The grammar establishes syntactic validity.

Semantic analysis establishes semantic validity.

Examples of semantic errors that do not belong in grammar:

break outside loop
continue outside loop
return outside function
invalid return type
unknown variable
invalid assignment
immutable value modified
invalid ownership
invalid borrow
invalid capability
unavailable resource
invalid quantum operation
invalid hardware requirement
invalid effect

This separation must be preserved.

---

35. IR contract

The statement grammar creates no IR.

The intended architecture is:

source
  ↓
lexer
  ↓
parser
  ↓
AST
  ↓
semantic analysis
  ↓
canonical semantic representation
  ↓
domain IR
  ↓
optimization
  ↓
routing / scheduling / lowering
  ↓
target

For quantum computation:

AST
  ↓
semantic quantum analysis
  ↓
quantum::ir

"quantum::ir" remains the canonical quantum semantic boundary.

No statement grammar file may define a competing quantum representation.

---

36. Compiler integration

Statement grammar integrates with the compiler through:

lexer
parser
AST
semantic analysis
type checker
effect checker
capability checker
control-flow analysis
resource validation
IR lowering
optimization
routing
scheduling
code generation

The grammar itself must remain independent from implementation-specific compiler algorithms.

---

37. Runtime integration

There must be no direct grammar-to-runtime dependency.

The correct relationship is:

grammar
  ↓
AST
  ↓
semantic model
  ↓
IR
  ↓
compiler/lowering
  ↓
runtime

This prevents runtime requirements from contaminating source syntax.

---

38. ANTLR integration

The grammar uses ANTLR4-compatible grammar structure.

ANTLR grammar composition must remain deterministic.

Parser grammars should use the canonical lexer vocabulary.

The grammar must not embed executable Rust actions.

Do not use:

@members
@init
@after
semantic predicates
embedded Rust
filesystem actions
network actions
runtime callbacks

to implement language semantics.

ANTLR is responsible for syntactic recognition.

The Rust compiler/frontend is responsible for interpretation of the parse tree.

---

39. Rust 1.97 / Rust 1.97.1

The surrounding Zamani implementation must support:

Rust 1.97
Rust 1.97.1

No Rust "unsafe" is permitted.

This requirement applies to the implementation surrounding the grammar as well as any generated/handwritten Rust integration code.

The grammar itself contains no Rust execution.

---

40. Determinism

Parsing must not depend on:

- hardware;
- CPU count;
- GPU availability;
- QPU availability;
- filesystem state;
- network state;
- calibration;
- scheduler state;
- runtime state;
- backend state;
- randomness;
- wall-clock time.

Given the same:

source
lexer version
grammar version
parser configuration

the parser must produce the same structural result.

---

41. Diagnostics

Syntax errors belong to the parser/frontend diagnostic system.

Diagnostics should preserve source locations.

Examples include:

unexpected token
missing statement
missing semicolon
missing block
unterminated block
malformed loop
malformed conditional
malformed match
malformed declaration
malformed return
invalid statement syntax
unexpected EOF

The grammar should not embed target-specific diagnostic logic.

Semantic diagnostics are downstream.

---

42. Error recovery

Parser recovery must be implemented by the parser/frontend infrastructure.

Grammar files must not contain ad-hoc error recovery actions.

Recovery must preserve:

- deterministic behavior;
- source locations;
- useful diagnostics;
- error ordering;
- parser safety.

A malformed statement must not cause the parser to execute user code.

---

43. Security boundary

The statement grammar must perform no:

- filesystem access;
- network access;
- process spawning;
- shell execution;
- hardware discovery;
- device probing;
- calibration lookup;
- runtime execution.

Parsing source must therefore remain a pure syntactic operation.

---

44. Scalability

The language must not define arbitrary source-level machine limits.

Do not hard-code maximum:

- statements;
- blocks;
- branches;
- loops;
- nested statements;
- expressions;
- devices;
- qubits;
- cores;
- threads;
- nodes;
- GPUs;
- FPGAs;
- accelerators;
- memory;
- program size.

This does not mean that implementations can physically consume infinite memory.

It means the language grammar must not impose artificial machine-specific semantic limits.

Implementation resource limits, where necessary, must be explicit configuration/resource-policy concerns rather than language semantics.

---

45. "Infinity" interpretation

"Scale to infinity" means:

«no artificial finite machine-size ceiling is encoded by the language.»

Actual execution remains bounded by available resources.

Therefore:

language scalability
        ≠
infinite physical resources

The grammar must permit programs whose scale is limited only by:

- available memory;
- compiler resources;
- execution resources;
- target capabilities;
- explicitly declared user/resource constraints.

---

46. Recursion and nesting

Recursive statement structures are permitted.

Examples include:

if {
    while {
        if {
            ...
        }
    }
}

No arbitrary language constant should define a maximum nesting depth.

A parser implementation may enforce operational resource limits for safety, but such limits must remain external to the language's semantic grammar.

---

47. Statement-family extension

Adding a new statement family requires all of the following:

1. dedicated grammar ownership;
2. explicit syntax specification;
3. lexer/token contract;
4. statement-composition integration;
5. AST mapping;
6. semantic contract;
7. IR/lowering contract where applicable;
8. positive tests;
9. negative tests;
10. boundary tests;
11. cross-domain tests;
12. compatibility review;
13. documentation.

The new family must not redefine an existing statement family.

---

48. Future computing domains

The grammar must remain extensible.

Future domains may include technologies that do not currently exist.

The architecture must therefore avoid making today's hardware categories the permanent organizing principle of the statement language.

New domain syntax should normally be added as a semantic extension rather than modifying generic control-flow constructs.

For example, a future accelerator should normally reuse:

if
while
for
match
return
block

rather than require:

futureAcceleratorIf
futureAcceleratorLoop

---

49. Cross-domain compatibility

The statement grammar must support combinations such as:

classical + quantum
classical + HDL
classical + hardware
quantum + classical
quantum + distributed
quantum + HDL
quantum + hardware
AI + quantum
AI + hardware
AI + distributed
classical + quantum + distributed
classical + quantum + HDL + hardware

The statement grammar should not need to know the domain composition.

The semantic layer determines whether the combination is legal.

---

50. Ownership matrix

Concern| Owner
Statement composition| "statements.g4"
Blocks| "blocks.g4"
Assignment statements| "assignments.g4"
Conditionals| "conditionals.g4"
Loops| "loops.g4"
Match statements| "pattern-matching.g4"
Declarations in statement position| "declarations.g4"
Returns| "returns.g4"
Break| "breaks.g4"
Continue| "continues.g4"
Assertions| "assertions.g4"
Exceptions| "exceptions.g4"
Unsafe syntax| "unsafe.g4"
Expressions| "grammar/expressions/"
Types| "grammar/types/"
Names| "grammar/core/"
Functions| "grammar/functions/"
Modules| "grammar/modules/"
Quantum syntax| "grammar/quantum/"
HDL syntax| "grammar/hdl/"
Hardware syntax| "grammar/hardware/"
Resources| "grammar/resources/"
AST| frontend AST subsystem
Semantic analysis| semantic-analysis subsystem
Quantum canonical semantics| "quantum::ir"
QEC| QEC subsystem
Noise/fault semantics| ZQN
Optimization| optimization subsystem
Routing| routing subsystem
Scheduling| scheduling subsystem
Hardware realization| hardware HAL/compiler
Runtime| runtime subsystem

---

51. Dependency direction

The dependency graph must remain acyclic at the semantic subsystem level.

lexer
  ↓
core syntax
  ↓
types / expressions
  ↓
statements
  ↓
declarations / functions / modules
  ↓
semantic analysis
  ↓
canonical representations
  ↓
IR
  ↓
optimization
  ↓
routing
  ↓
scheduling
  ↓
hardware lowering
  ↓
runtime

A grammar component must never require:

runtime → grammar
IR → grammar
hardware → grammar
scheduler → grammar

as a semantic dependency.

---

52. Grammar import rules

ANTLR delegate grammars may import other grammar components where structurally necessary.

However:

«An imported grammar must not redefine the authoritative concept owned by its parent composition layer.»

For example:

statements.g4

owns:

statement

Therefore a delegate grammar must not independently redefine the authoritative "statement" rule.

Likewise:

blocks.g4

owns block syntax.

No conditional/loop grammar should redefine "{ ... }".

---

53. No duplicate language concepts

The following must each have one authoritative owner:

statement
block
expression
assignment expression
type
identifier
pattern
return
break
continue
if statement
loop statement

If the repository contains duplicates, they must be:

- removed;
- merged;
- converted into delegates;
- or explicitly retained as migration-only artifacts.

Two grammars must never silently define different meanings for the same language construct.

---

54. Legacy monolithic grammar

The historical "grammar/Zamani.g4" contains a large monolithic statement system.

The modular grammar must supersede duplicated statement definitions once the modular parser is authoritative.

The migration path is:

legacy Zamani.g4
        |
        v
inventory existing syntax
        |
        v
map syntax to modular owners
        |
        v
validate compatibility
        |
        v
modular grammar becomes authoritative
        |
        v
legacy duplicate productions removed/deprecated

No existing valid Zamani syntax should disappear silently.

---

55. Compatibility requirements

Before changing a statement construct:

1. identify the old syntax;
2. identify its consumers;
3. identify its intended meaning;
4. determine whether the syntax is valid;
5. preserve it where valid;
6. migrate it where structurally misplaced;
7. document changed syntax;
8. add migration tests;
9. update compatibility documentation.

Breaking changes require explicit language-version policy.

---

56. Testing architecture

Every statement grammar component requires tests.

The minimum test categories are:

positive
negative
boundary
cross-domain
compatibility
determinism
round-trip
scalability

Tests belong under:

grammar/tests/

and should be organized by statement family.

---

57. Positive tests

Every valid statement form requires at least one positive test.

Examples:

if condition {}
if condition {} else {}
while condition {}
do {} while condition;
for item in items {}
return;
return value;
break;
continue;
try {} catch (...) {}
match value {}
assert condition;

The exact syntax must follow the canonical grammar.

---

58. Negative tests

Negative tests must cover malformed syntax.

Examples:

if {}
if condition
while
for
return (
break malformed
continue malformed
try
catch
match

Also test malformed nesting and incomplete EOF input.

---

59. Semantic-boundary tests

Parser tests must not accidentally become semantic tests.

For example:

break;

should be syntactically parseable.

A separate semantic test should determine:

break outside loop

is invalid.

This preserves the grammar/semantic boundary.

---

60. Cross-domain tests

Statement grammar must be tested around domain syntax.

Examples:

if classical_condition {
    classical_operation();
}

if quantum_condition {
    quantum_operation();
}

while condition {
    hardware_operation();
}

for item in data {
    quantum_operation();
}

if ready {
    distributed_operation();
}

These tests verify that statement syntax is domain-neutral.

---

61. Quantum scalability tests

The grammar tests must verify that source syntax does not contain arbitrary quantum-size assumptions.

Test programs must be able to describe semantic structures involving resource expressions without requiring:

q[0]
q[1]

as hidden grammar assumptions.

There must be no parser-level maximum qubit count.

---

62. Hardware scalability tests

Verify that statement syntax does not impose:

MAX_CORES
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_DEVICES
MAX_NODES

or equivalent constraints.

---

63. Determinism tests

Given the same source and grammar version:

parse(source)

must produce structurally equivalent parse results.

No test may depend on:

- machine topology;
- execution timing;
- hardware availability;
- random values;
- network state.

---

64. Round-trip tests

Where a canonical formatter/printer exists:

source
  ↓
lexer
  ↓
parser
  ↓
AST
  ↓
printer
  ↓
parser

must preserve intended semantics.

Formatting differences alone must not be treated as semantic differences.

---

65. Fuzz testing

Statement grammar should support parser fuzzing.

Fuzz tests should exercise:

- random statement nesting;
- malformed blocks;
- malformed control flow;
- large statement sequences;
- long identifiers;
- deeply nested valid structures;
- invalid token sequences;
- EOF boundaries.

The parser must not:

- execute code;
- access the network;
- access files;
- probe hardware;
- invoke runtime operations.

---

66. Resource-exhaustion testing

Scalability testing must distinguish language limits from implementation limits.

Test increasingly large:

- statement sequences;
- nested blocks;
- loop chains;
- conditional chains;
- match cases.

The grammar must not reject a construct because of an arbitrary machine-size constant.

Operational resource limits must be explicit parser/compiler policy.

---

67. Hard-coding audit

Every statement grammar file must be audited for:

MAX_*
LIMIT_*
COUNT_*
DEVICE_*
CPU_*
GPU_*
QUBIT_*
THREAD_*
NODE_*

and equivalent literal restrictions.

Each discovered limitation must be classified as:

1. language semantic requirement;
2. parser implementation requirement;
3. compiler resource policy;
4. target-specific requirement;
5. test-only limitation;
6. documentation-only limitation;
7. accidental hard-coding.

Accidental hard-coding must be removed.

---

68. Completion contract for every statement grammar file

A statement grammar file is not complete merely because ANTLR accepts it.

It is complete only when all of the following are true:

- ownership is documented;
- non-ownership is documented;
- lexer dependencies are documented;
- expression dependencies are documented;
- block dependencies are documented;
- AST contract is defined;
- semantic boundary is defined;
- IR boundary is defined;
- compiler integration is defined;
- runtime non-dependency is defined;
- scalability has been audited;
- hard-coding has been audited;
- compatibility has been considered;
- positive tests exist;
- negative tests exist;
- boundary tests exist;
- cross-domain tests exist where applicable;
- deterministic parsing is verified;
- no Rust "unsafe" is required;
- no embedded executable grammar actions are required;
- no machine-specific assumptions exist;
- no duplicate authoritative rule exists.

---

69. Completion contract for this directory

"grammar/statements/" is production-ready only when:

[ ] exactly one canonical statement composition exists
[ ] every statement family has one owner
[ ] no duplicate statement grammar remains authoritative
[ ] all grammar imports are deterministic
[ ] all lexer tokens come from the canonical lexer
[ ] expression syntax is delegated to expressions/
[ ] type syntax is delegated to types/
[ ] names are delegated to core/
[ ] blocks have one owner
[ ] declarations have one owner
[ ] functions have one owner
[ ] modules have one owner
[ ] quantum syntax remains outside generic statement ownership
[ ] HDL syntax remains outside generic statement ownership
[ ] hardware semantics remain outside generic statement ownership
[ ] QEC remains outside grammar ownership
[ ] ZQN remains outside grammar ownership
[ ] resilience remains outside grammar ownership
[ ] routing remains outside grammar ownership
[ ] scheduling remains outside grammar ownership
[ ] optimization remains outside grammar ownership
[ ] runtime remains outside grammar ownership
[ ] no machine-size constants exist
[ ] no hardware topology is hard-coded
[ ] no fixed qubit count exists
[ ] no fixed core/thread count exists
[ ] no fixed device count exists
[ ] no unsafe Rust is used
[ ] no embedded executable actions exist
[ ] diagnostics preserve source locations
[ ] positive tests exist
[ ] negative tests exist
[ ] boundary tests exist
[ ] scalability tests exist
[ ] cross-domain tests exist
[ ] deterministic parsing is verified
[ ] compatibility is documented
[ ] legacy grammar migration is complete or explicitly tracked

---

70. Integration checklist before declaring this README complete

The implementation of this directory must be checked against:

grammar/README.md
grammar/Zamani.g4
grammar/Zamani-Grammar.md
grammar/specification/
grammar/lexer/
grammar/core/
grammar/types/
grammar/expressions/
grammar/declarations/
grammar/functions/
grammar/modules/
grammar/effects/
grammar/memory/
grammar/concurrency/
grammar/classical/
grammar/quantum/
grammar/hybrid/
grammar/hdl/
grammar/hardware/
grammar/distributed/
grammar/ai/
grammar/data/
grammar/networking/
grammar/security/
grammar/resources/
grammar/compile/
grammar/execution/
grammar/interoperability/
grammar/dialects/
grammar/macros/
grammar/metaprogramming/
grammar/validation/
grammar/compatibility/
grammar/tests/

The repository's existing architecture already explicitly connects the statements layer with the expression, type, quantum, HDL, hardware, and compilation layers, so this README treats those relationships as integration contracts rather than inventing another parallel architecture.

---

71. Required implementation order

The statement subsystem should be implemented in dependency order.

Phase 1 — composition foundation

statements.g4

Establish the authoritative composition contract.

Phase 2 — structural foundation

blocks.g4

Establish canonical block/statement recursion.

Phase 3 — basic control flow

conditionals.g4
loops.g4
breaks.g4
continues.g4
returns.g4

Phase 4 — state-changing statements

assignments.g4
declarations.g4

Phase 5 — advanced control flow

pattern-matching.g4
exceptions.g4
assertions.g4

Phase 6 — explicit safety boundary

unsafe.g4

Phase 7 — integration

Integrate the complete statement family with:

expressions/
types/
core/
declarations/
functions/
modules/
effects/
memory/
concurrency/
quantum/
hybrid/
hdl/
hardware/
distributed/
resources/
compile/
execution/

Phase 8 — repository validation

Validate against:

legacy Zamani.g4
canonical grammar authority
AST
semantic analysis
IR
quantum::ir
compiler
tests
documentation

No later subsystem should require changing the fundamental ownership model of an already completed statement grammar file.

---

72. Architectural invariant

The following invariant must never be violated:

«Statements describe source-level computation and control flow. They do not describe the physical machine that will execute them.»

Therefore:

Zamani statement
        |
        v
semantic meaning
        |
        +--> classical machine
        +--> quantum machine
        +--> HDL implementation
        +--> accelerator
        +--> distributed system
        +--> heterogeneous machine
        +--> future architecture

The statement grammar remains stable while target realization evolves.

---

73. Final production principle

"grammar/statements/" must remain:

syntax-first
semantic-boundary-safe
target-independent
hardware-independent
quantum-aware but quantum-IR-independent
deterministic
extensible
versionable
testable
safe
resource-neutral
scalable

The grammar must never become a hidden hardware description language, scheduler, optimizer, runtime, quantum compiler, or resource manager.

The final architectural rule is:

«Zamani statements express what the program means, not what machine happens to execute it.»

That is the statement-level foundation required for:

From Atom to Everywhere

and:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever

with practical execution scale determined by available resources rather than arbitrary limits embedded in the language grammar.
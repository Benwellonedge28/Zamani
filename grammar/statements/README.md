Zamani Statements Grammar

Status

Production architecture / normative subsystem contract

This directory defines the statement-level syntax layer of the Zamani programming language.

It is part of the canonical grammar architecture and participates in:

Zamani source
    ↓
lexical analysis
    ↓
parser
    ↓
domain-neutral frontend AST
    ↓
semantic analysis
    ↓
canonical semantic models / IR
    ↓
optimization and lowering
    ↓
routing / scheduling / resilience / ZQN / HAL
    ↓
target realization

The statements grammar is therefore responsible for describing program actions and control structure, not for deciding how those actions are physically realized.

The statements subsystem must support Zamani's:

«Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)»

principle.

A valid Zamani program must be expressible independently of the size, topology, vendor, architecture, or physical limits of the machine on which it may eventually execute.

---

1. Purpose

"grammar/statements/" owns the syntax and composition contracts for Zamani statements.

Statements are constructs that describe things such as:

- bindings;
- declarations used in statement position;
- assignments;
- control flow;
- loops;
- pattern matching;
- returns;
- assertions;
- blocks;
- concurrency;
- effects;
- resource requirements;
- quantum operations;
- HDL actions;
- domain-specific operations;
- execution intent;
- portable resource and capability requirements.

The directory does not own:

- lexical token definitions;
- universal expression precedence;
- type semantics;
- AST implementation;
- semantic analysis;
- canonical IR implementation;
- quantum IR;
- QEC implementation;
- ZQN implementation;
- routing;
- scheduling;
- calibration;
- HAL implementation;
- hardware topology;
- runtime resource discovery;
- vendor-specific execution;
- physical machine limits.

Those responsibilities remain with their respective repository subsystems.

---

2. Architectural Role

The statements subsystem is a syntax layer, not a semantic execution engine.

Its responsibility ends at producing a deterministic parser representation from which the frontend can construct the domain-neutral AST.

The architectural boundary is:

                 statements/
                       │
                       ▼
              Parser statement tree
                       │
                       ▼
                Frontend AST
                       │
                       ▼
              Semantic analysis
                       │
                       ▼
       Canonical semantic representation
                       │
              ┌────────┼────────┐
              ▼        ▼        ▼
        Classical   quantum::ir   HDL
              │        │        │
              └────────┼────────┘
                       ▼
             Optimization/lowering
                       ▼
       routing / scheduling / QEC / ZQN
                       ▼
                      HAL
                       ▼
               target realization

The statements grammar must never bypass this architecture.

---

3. Authority

The statements directory is governed by the repository-wide grammar authority model.

The authority order is:

grammar/DESIGN.md
        ↓
grammar/specification/
        ↓
grammar/spec/
        ↓
grammar/Zamani.g4
        ↓
grammar/statements/*
        ↓
frontend AST
        ↓
semantic implementation
        ↓
canonical IR

The following files must not become competing authorities:

- "grammar/Zamani-Grammar.md"
- "grammar/grammar.md"
- old monolithic grammar fragments
- examples
- tests
- generated documentation

Their roles are:

"grammar/DESIGN.md"

Normative architecture.

"grammar/specification/"

Normative language specification.

"grammar/spec/"

Formal feature and subsystem contracts.

"grammar/Zamani.g4"

Canonical ANTLR composition root.

"grammar/statements/*.g4"

Modular statement syntax.

"grammar/grammar.md"

Implementation-conformance reference.

"grammar/Zamani-Grammar.md"

Historical/extended language design reference. Its contents do not automatically become legal syntax.

"grammar/tests/"

Executable conformance evidence.

---

4. Directory Ownership

The statement subsystem should contain the following responsibilities.

statements/
├── README.md
├── statement.g4
├── bindings.g4
├── control-flow.g4
├── loops.g4
├── match.g4
├── exceptions.g4
├── returns.g4
├── assertions.g4
├── blocks.g4
├── concurrency.g4
├── resource-statements.g4
├── effect-statements.g4
├── domains.g4
├── quantum-statements.g4
└── hdl-statements.g4

Existing files must be retained where they already exist.

New files should only be created when a real responsibility requires them.

Do not create duplicate files merely to make the directory tree look complete.

---

5. File Responsibility Matrix

File| Owns| Must not own
"README.md"| subsystem contract and integration rules| executable grammar
"statement.g4"| universal statement composition| domain implementation
"bindings.g4"| binding/assignment statement syntax| type checking
"control-flow.g4"| conditional/control syntax| runtime scheduling
"loops.g4"| iteration syntax| fixed iteration limits
"match.g4"| pattern matching syntax| pattern semantics
"exceptions.g4"| exception/error-control syntax| runtime recovery implementation
"returns.g4"| return/yield/exit statement syntax| ABI implementation
"assertions.g4"| assertion syntax| verification engine
"blocks.g4"| statement blocks/scopes in statement position| scope semantics
"concurrency.g4"| portable concurrency constructs| thread/core counts
"resource-statements.g4"| resource requirements/preferences/constraints| resource allocation
"effect-statements.g4"| effect-related statement syntax| effect runtime
"domains.g4"| domain statement composition/dispatch| domain-specific semantics
"quantum-statements.g4"| quantum statement syntax boundary| quantum IR/QEC/routing
"hdl-statements.g4"| HDL statement syntax boundary| synthesis/physical implementation

---

6. Universal Statement Contract

Every statement grammar file must satisfy this contract.

Each statement construct must define:

1. Purpose.
2. Syntax.
3. Accepted tokens.
4. Precedence interaction, if applicable.
5. Associativity, if applicable.
6. Source-span behavior.
7. AST mapping.
8. Semantic validation requirements.
9. Canonical semantic/IR mapping.
10. Diagnostics.
11. Positive tests.
12. Negative tests.
13. Boundary tests.
14. Scalability tests.
15. Determinism tests.
16. Compatibility tests.
17. Cross-domain integration.
18. Hard-coding audit.
19. Security implications where applicable.
20. Performance implications where applicable.

A statement feature is not complete merely because ANTLR accepts it.

---

7. Universal Statement Entry Point

"statement.g4" is the central statement composition boundary.

Conceptually:

statement
    ├── binding
    ├── control flow
    ├── loop
    ├── match
    ├── exception
    ├── return
    ├── assertion
    ├── block
    ├── concurrency
    ├── resource
    ├── effect
    └── domain

The exact rule names must follow the canonical grammar already established by the repository.

No domain grammar should silently create a second universal "statement" rule.

The universal statement entry point must have one owner.

---

8. Composition Rule

"statement.g4" should compose specialized statement grammars.

It should not duplicate their detailed rules.

The desired architecture is:

statement.g4
    │
    ├── bindings.g4
    ├── control-flow.g4
    ├── loops.g4
    ├── match.g4
    ├── exceptions.g4
    ├── returns.g4
    ├── assertions.g4
    ├── blocks.g4
    ├── concurrency.g4
    ├── resource-statements.g4
    ├── effect-statements.g4
    └── domains.g4
              │
              ├── quantum-statements.g4
              ├── hdl-statements.g4
              └── domain-specific statement boundaries

A rule must have one authoritative owner.

If two files define semantically equivalent statement rules, one must be removed or converted into a composition/delegation layer.

---

9. "statement.g4"

Purpose

Provide the canonical statement-level composition point.

Owns

- universal statement dispatch;
- statement-level composition;
- common statement termination;
- integration of specialized statement grammars.

Does not own

- expressions;
- types;
- declarations;
- quantum operation details;
- HDL details;
- hardware details;
- resource implementation;
- runtime behavior.

Inputs

Lexer tokens and imported grammar rules.

Outputs

Statement parser contexts consumed by the frontend AST builder.

Integration

lexer
  ↓
statement.g4
  ↓
frontend AST
  ↓
semantic analysis

"Zamani.g4" imports/composes the statement entry point.

The frontend parser must not implement an independent statement dispatcher that competes with this file.

---

10. "bindings.g4"

Purpose

Define binding and assignment statements.

Examples of semantic categories include:

let
var
const
assignment
destructuring
rebinding
pattern binding

The grammar must not decide whether a binding is legal for a particular type.

That belongs to semantic analysis.

Scalability

Bindings must not impose fixed:

- object counts;
- variable counts;
- array sizes;
- register counts;
- memory sizes.

A program may contain any number of bindings supported by available compiler/runtime resources.

---

11. "control-flow.g4"

Owns syntax for constructs such as:

- conditional branches;
- structured control flow;
- conditional execution;
- domain-neutral branching.

It must not encode:

- processor branch limits;
- fixed nesting depth;
- hardware branch units;
- GPU warp sizes;
- quantum control hardware.

The semantic layer decides whether a construct can be lowered to a target.

---

12. "loops.g4"

Owns iteration syntax.

Supported semantics should be able to represent:

- finite iteration;
- condition-based iteration;
- collection iteration;
- ranges;
- potentially unbounded/streaming iteration where the language specification permits it;
- parallel iteration where separately defined.

The grammar must not establish universal limits such as:

MAX_ITERATIONS = 1024

or:

MAX_LOOP_DEPTH = 64

Such values, if required by an individual target, belong to target capability negotiation.

---

13. "match.g4"

Owns pattern-matching syntax.

It should support the language's canonical pattern model without creating independent type or expression systems.

Pattern semantics belong to the type/semantic subsystem.

The grammar must not duplicate:

- type matching;
- exhaustiveness analysis;
- ownership analysis;
- reachability analysis.

---

14. "exceptions.g4"

Owns syntax for:

- throwing/raising an error;
- catching/handling;
- recovery constructs;
- propagation where specified.

It must not define runtime implementation.

Exception semantics must integrate with:

effects/
execution/
validation/
diagnostics/

Where Zamani's effect system represents failures explicitly, exception syntax must map to the canonical effect/error semantics rather than creating a competing error model.

---

15. "returns.g4"

Owns:

- "return";
- expression return;
- empty return where legal;
- function exit;
- generator/yield forms if those are part of the canonical language;
- structured result propagation where specified.

It must not own calling conventions or ABI layout.

Those belong to:

functions/
interoperability/
compile/

---

16. "assertions.g4"

Assertions are statements expressing programmer-visible correctness requirements.

They may be used for:

- runtime assertions;
- compile-time assertions where specified;
- invariants;
- contracts;
- verification conditions.

The grammar must not implement the verification engine.

Integration:

assertion syntax
    ↓
AST assertion
    ↓
semantic validation
    ↓
verification / runtime assertion / optimization

Assertions must remain portable.

---

17. "blocks.g4"

Owns statement block syntax.

Blocks may contain arbitrary numbers of statements subject to available compiler/runtime resources.

No universal fixed maximum shall be encoded.

For example, the grammar must not impose:

block contains <= 1024 statements

or:

nesting <= 256

unless such a limit is explicitly a parser implementation safety policy and is not presented as a language semantic restriction.

---

18. "concurrency.g4"

Concurrency syntax must describe computation and coordination without assuming a machine topology.

Valid semantic concepts include:

parallel
spawn
async
await
task
actor
channel
pipeline
parallel iteration
synchronization
collective operation

The grammar must not encode:

8 threads
16 cores
32 workers
1 GPU
4 GPUs
1024 nodes

as universal language limits.

Instead:

parallel

means the computation admits parallel realization.

The compiler/runtime may determine the actual execution width.

---

19. Resource Statements

"resource-statements.g4" is the statement-level interface to the resource model.

It must distinguish:

requirement
constraint
capability requirement
preference
hint
budget
negotiation
placement intent
scaling policy

These are not interchangeable.

For example:

requires capability("quantum.measurement")

means that a semantic capability is required.

It does not mean:

use physical device 0

Likewise:

requires memory >= required_memory

expresses a requirement.

It must not establish a universal memory maximum.

---

20. Requirement vs Implementation Decision

This distinction is mandatory.

Portable requirement

requires qubits >= n

Capability

requires capability("quantum.mid_circuit_measurement")

Preference

prefer capability("accelerator.compute")

Hint

hint execution.parallel

Implementation decision

map logical resource → physical resource

The first four can participate in portable program semantics.

The final category belongs downstream to compilation/deployment/target realization.

---

21. "effect-statements.g4"

Effect statements must integrate with the canonical effect system.

They must not create a second effect representation.

The path is:

statement syntax
    ↓
effect AST representation
    ↓
effect semantic analysis
    ↓
canonical effect model
    ↓
compiler/runtime handling

Effects may represent:

- IO;
- state;
- concurrency;
- resource use;
- communication;
- quantum effects;
- timing;
- nondeterminism;
- security-sensitive actions;
- external interaction.

The grammar does not implement any effect.

---

22. "domains.g4"

"domains.g4" is the domain composition boundary.

It should not become a second monolithic statement grammar.

It should delegate to domain-specific statement boundaries.

Conceptually:

domains.g4
    │
    ├── classical
    ├── quantum
    ├── hybrid
    ├── HDL
    ├── distributed
    ├── AI
    ├── data
    ├── networking
    ├── security
    ├── hardware
    ├── resources
    ├── compile
    └── execution

The exact imports/rule names must follow the canonical ANTLR composition architecture.

No domain should silently redefine the universal statement rule.

---

23. Quantum Statement Integration

"quantum-statements.g4" owns only quantum statement syntax.

It must integrate with:

grammar/quantum/
src/quantum/frontend/
src/quantum/ir/
quantum semantic analysis
optimization
routing
scheduling
QEC
resilience
ZQN
HAL

The grammar must not duplicate "quantum::ir".

The canonical path is:

Zamani quantum statement
        ↓
domain-neutral AST
        ↓
quantum semantic analysis
        ↓
quantum::ir
        ↓
optimization
        ↓
decomposition
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
target

---

24. No Fixed Quantum Gate Enumeration

The statements grammar must not define a universal list such as:

H
X
Y
Z
CNOT
...

as the complete language of quantum operations.

Quantum operation names are semantic data.

The syntax must support the canonical operation model already established by the repository.

Conceptually:

operation name
+
operands
+
parameters
+
results
+
attributes/modifiers

This permits:

apply H ...
apply custom_operation ...
apply vendor.operation ...
apply parameterized_operation(...)

without forcing the parser to know every possible present or future operation.

This is essential for:

- future quantum operations;
- vendor operations;
- research operations;
- logical operations;
- decomposed operations;
- pulse-level intent;
- domain extensions.

---

25. Quantum Scalability

The grammar must not impose limits on:

- number of qubits;
- number of registers;
- number of operations;
- circuit depth;
- number of controls;
- number of measurements;
- number of classical feed-forward operations;
- number of quantum kernels;
- number of quantum devices.

Program values such as:

n

may determine resource requirements.

That is different from imposing a compiler language limit.

---

26. HDL Statement Integration

"hdl-statements.g4" owns HDL statement syntax.

It integrates with:

grammar/hdl/
grammar/hardware/
grammar/resources/
grammar/compile/
grammar/execution/

HDL syntax describes hardware intent.

It must not silently encode a particular physical implementation.

Avoid universal constructs whose meaning depends on today's device limits.

For example, physical width must remain parameterizable where width is part of program semantics.

The grammar must not impose an arbitrary universal:

[31:0]

limit.

---

27. Hardware and Resource Separation

Hardware-related statements must distinguish:

hardware intent
resource requirement
capability
constraint
deployment decision
physical realization

For example:

requires capability("gpu.compute")

is portable.

A statement that identifies a particular physical GPU belongs to a target-specific deployment layer unless the language explicitly defines physical targeting as a non-portable feature.

The distinction must be visible in the AST and semantic model.

---

28. Hybrid Computing

Statements must support programs that combine domains.

A valid program may conceptually contain:

classical computation
        ↓
quantum operation
        ↓
measurement
        ↓
classical decision
        ↓
quantum operation
        ↓
classical result

The parser must not require programmers to write separate languages for these stages.

Hybrid semantics are integrated through:

hybrid/
quantum/
classical/
types/
effects/
resources/
execution/

---

29. Cross-Domain Integration

The statements subsystem must support composition across:

- classical;
- quantum;
- hybrid;
- HDL;
- hardware;
- distributed;
- AI;
- data;
- networking;
- security;
- resource;
- compile;
- execution domains.

The grammar must avoid creating incompatible mini-languages.

All domains share:

- identifiers;
- names;
- paths;
- expressions;
- types;
- blocks;
- attributes;
- modifiers;
- source locations;
- diagnostics;
- effects;
- capabilities;
- resource requirements.

---

30. AI and Data Statements

AI and data statements must remain semantic categories rather than framework-specific parser languages.

The grammar should not hard-code:

- PyTorch;
- TensorFlow;
- JAX;
- CUDA;
- ROCm;
- vendor model APIs;
- specific model architectures;

as universal language syntax.

The statement layer may describe concepts such as:

train
infer
transform
pipeline
dataset operation
model operation
tensor operation

where these are genuine Zamani semantic constructs.

Framework implementation belongs downstream.

---

31. Distributed Statements

Distributed statements must not assume:

- fixed node counts;
- fixed cluster sizes;
- fixed topology;
- fixed process counts;
- fixed network bandwidth;
- fixed accelerator counts.

A program may express:

parallel
replicate
partition
communicate
collect
synchronize

without stating how many physical machines must exist.

Actual placement is resolved later.

---

32. Networking Statements

Networking syntax should describe:

- communication;
- endpoints;
- services;
- channels;
- requests;
- responses;
- streaming;
- protocol intent.

Physical addresses and topology must remain abstract whenever possible.

Network-specific realization belongs downstream.

---

33. Security Statements

Security statements must integrate with:

security/
effects/
capabilities/
resources/
interoperability/

Security syntax may describe:

- identity;
- authorization;
- policy;
- capability;
- trust;
- secure computation;
- provenance.

The parser must not turn every cryptographic algorithm into a reserved keyword.

---

34. Expressions Are Not Statements

Statement grammars must reuse the canonical expression grammar.

Do not duplicate:

expression
assignment expression
call expression
literal
type expression

inside every statement file.

For example:

if (expression) statement

should reference the canonical expression rule.

This prevents expression precedence from diverging between domains.

---

35. Types Are Not Statements

Statement grammar must reference the canonical type system.

It must not define competing versions of:

type
generic type
function type
quantum type
tensor type
resource type
capability type

Type syntax belongs under:

grammar/types/

Semantic type validation belongs downstream.

---

36. Declarations Are Not Statements

Where declarations may syntactically occur in statement position, the statement layer should compose the canonical declaration rules.

It must not redefine:

struct
class
enum
interface
trait
resource
capability
module

Declarations remain owned by:

grammar/declarations/

---

37. Source Spans

Every statement construct must preserve sufficient source information for diagnostics.

The parser/AST pipeline must be able to associate statements with:

- source file;
- start position;
- end position;
- line/column information where available;
- relevant child spans.

Source locations must not be discarded merely because a statement is domain-specific.

This is required for:

- compiler diagnostics;
- semantic errors;
- IDE tooling;
- debugging;
- provenance;
- verification;
- source-to-IR mapping.

---

38. Diagnostics

Every statement family must define expected diagnostics for:

- malformed syntax;
- missing operands;
- malformed delimiters;
- invalid statement structure;
- invalid combinations;
- unexpected tokens;
- unsupported syntax;
- ambiguous syntax where ambiguity cannot be resolved;
- deprecated syntax.

Semantic errors belong downstream but should retain the original statement source span.

Diagnostics must be deterministic.

---

39. Determinism

For the same:

source
+
language version
+
dialect configuration
+
declared feature set

the parser must produce the same parse structure.

The grammar must not depend on:

- machine size;
- CPU count;
- GPU count;
- network topology;
- runtime state;
- random values;
- physical device discovery.

Hardware discovery occurs after parsing.

---

40. Scalability Requirements

The statements grammar must scale in semantic expressiveness without imposing artificial resource limits.

It must support programs containing arbitrarily many:

- statements;
- blocks;
- bindings;
- loops;
- branches;
- matches;
- tasks;
- channels;
- resource requirements;
- quantum operations;
- HDL operations;
- domain transitions;

subject to actual implementation resources.

The grammar itself must not establish artificial semantic maxima.

Examples of prohibited universal language limits:

MAX_STATEMENTS
MAX_BLOCK_DEPTH
MAX_THREADS
MAX_CORES
MAX_GPUS
MAX_QUBITS
MAX_NODES
MAX_CHANNELS
MAX_OPERATIONS
MAX_TENSOR_DIMENSIONS

---

41. "Infinity" and Resource Availability

"Infinity" means that the language does not impose a fixed architectural maximum.

It does not mean that a physical machine can execute an actually infinite computation.

The correct model is:

language capability
        ↓
program requirements
        ↓
available resources
        ↓
compiler/runtime decisions
        ↓
actual execution

If resources are insufficient, the compiler/runtime may:

- reject the target;
- report unmet requirements;
- specialize;
- partition;
- distribute;
- schedule;
- spill;
- lower;
- approximate where explicitly permitted;
- negotiate another target.

The parser must not decide any of those outcomes.

---

42. Hard-Coding Policy

Every statement grammar file must pass a hard-coding audit.

The following are prohibited as universal language restrictions:

MAX_CPU
MAX_GPU
MAX_FPGA
MAX_QPU
MAX_QUBIT
MAX_CORE
MAX_THREAD
MAX_NODE
MAX_MEMORY
MAX_REGISTER
MAX_VECTOR_WIDTH
MAX_TENSOR_DIMENSION
MAX_TIMELINE
MAX_CHANNEL
MAX_ACCELERATOR

Also prohibited are fixed universal physical identifiers such as:

cpu0
gpu0
qpu0
qubit0
node0
memory_bank0

unless they occur as ordinary programmer data rather than language-defined physical limits.

---

43. Program Data vs Language Limits

The hard-coding rule does not prohibit constants.

This is valid:

let n = 1024;

because "1024" may be program data.

This is different from:

MAX_QUBITS = 1024

being a universal language rule.

Likewise:

Tensor<1024, 1024>

may be valid program semantics.

But the grammar must not reject:

Tensor<2048, 2048>

merely because the language designer selected 1024 as an implementation maximum.

---

44. AST Integration Contract

Every statement rule must have a predetermined AST mapping.

The required path is:

grammar rule
    ↓
parser context
    ↓
domain-neutral AST
    ↓
semantic representation
    ↓
canonical IR

No statement may be added to the grammar with the assumption:

«"The AST can be figured out later."»

That creates downstream rework and competing representations.

---

45. Generic Operation Model

Where statement syntax describes operations, the frontend should use the repository's domain-neutral operation model rather than creating domain-specific parser-only operation types.

The intended conceptual structure is:

Operation {
    name,
    namespace,
    operands,
    parameters,
    results,
    attributes,
    modifiers,
    effects,
    capabilities,
    source
}

The grammar may construct the syntax required to populate this model.

It must not introduce a competing semantic operation hierarchy.

---

46. Quantum AST / IR Boundary

For quantum statements:

quantum statement
    ↓
generic frontend operation
    ↓
semantic quantum operation
    ↓
quantum::ir

Do not create:

quantum statement
    ↓
new QuantumStatementIR
    ↓
quantum::ir

That would duplicate the canonical quantum semantic boundary.

---

47. HDL AST / IR Boundary

HDL statements similarly follow:

HDL syntax
    ↓
frontend AST
    ↓
hardware semantic analysis
    ↓
canonical hardware/HDL representation
    ↓
synthesis/lowering/verification

The parser must not perform synthesis.

---

48. Effects and Capabilities

Statements may declare or invoke capabilities, but capability checking is semantic.

For example:

requires capability("quantum.measurement")

is syntax.

Whether the selected target provides that capability is a semantic/compiler/runtime question.

Likewise:

requires capability("gpu.compute")

does not mean the parser selects a GPU.

---

49. Compile-Time and Runtime Separation

Statements must distinguish:

compile-time intent
runtime behavior
target realization

Compilation constructs belong to:

compile/

Runtime constructs belong to:

execution/

The statement subsystem only provides composition points where the language specification says these constructs are legal.

---

50. Interoperability

Statements that interact with foreign systems must use the interoperability subsystem.

The grammar must not create independent language semantics for:

- C;
- C++;
- Rust;
- Python;
- WebAssembly;
- OpenQASM;
- QIR;
- LLVM;
- MLIR;
- vendor-specific HDLs.

These are interoperability targets/formats.

They are not alternative canonical Zamani semantic models.

---

51. Dialects

Dialect-specific statements must remain explicitly scoped.

A dialect must declare:

dialect name
version
feature set
syntax extension
semantic extension
AST mapping
compatibility requirements

A dialect must not silently modify the meaning of core statements.

Core Zamani remains portable.

---

52. Versioning

Every statement syntax extension must identify:

- language version;
- feature status;
- compatibility status;
- deprecation status where applicable.

Feature states are:

PROPOSED
EXPERIMENTAL
STABLE
DEPRECATED
REMOVED
HISTORICAL

The precise status vocabulary must remain synchronized with the repository-wide specification.

---

53. Feature Promotion

A proposed statement cannot become stable merely because grammar syntax exists.

The promotion path is:

proposal
   ↓
specification
   ↓
semantic contract
   ↓
AST contract
   ↓
canonical grammar
   ↓
semantic implementation
   ↓
IR integration
   ↓
compiler/runtime integration
   ↓
tests
   ↓
compatibility validation
   ↓
STABLE

---

54. Feature Manifest Integration

Where the repository adopts feature manifests under:

grammar/specification/features/

each statement feature should have a corresponding contract.

The manifest should identify:

feature id
name
status
version
grammar file
grammar rule
lexer tokens
AST nodes
semantic rules
IR mapping
compiler consumers
runtime consumers
capabilities
resource requirements
positive tests
negative tests
boundary tests
scalability tests
compatibility
hard-coding policy

This makes each statement feature independently completable.

---

55. Independence-First Development

A file must be completed against its contracts before dependent files are changed.

For each statement grammar file:

1. define responsibility
2. define dependencies
3. define outputs
4. define AST contract
5. define semantic contract
6. define IR contract
7. define diagnostics
8. define tests
9. audit scalability
10. audit hard-coding
11. validate grammar
12. mark complete

A later file must consume the published contract rather than forcing the earlier file to be redesigned.

---

56. No Retroactive Semantic Drift

Once a statement file is marked complete, another domain must not change its meaning merely to accommodate itself.

For example, a quantum grammar must not redefine the meaning of:

if
while
return
parallel
match

to solve a quantum-specific problem.

Instead, the quantum subsystem must integrate through the established universal semantics.

---

57. Domain-Specific Extensions

A domain may extend the language only when:

1. the syntax expresses genuine domain semantics;
2. it has a defined AST mapping;
3. semantic behavior is specified;
4. the canonical IR boundary is known;
5. diagnostics are defined;
6. tests exist;
7. compatibility is defined;
8. no existing construct is unnecessarily duplicated.

---

58. Mathematical Operations

The statement subsystem must not become a list of mathematical library functions.

Operations such as:

- FFT;
- SVD;
- gradient descent;
- matrix decomposition;
- optimization;
- statistical operations;

should generally be represented through:

generic operation
+
typed operands
+
parameters
+
capabilities

when they do not require dedicated language semantics.

Libraries and semantic capabilities should own the implementation.

---

59. AI Framework Independence

The statements grammar must remain independent of particular AI frameworks.

The language may describe:

model
train
infer
dataset
tensor operation
pipeline
agent
optimization

when these are canonical Zamani concepts.

The grammar must not require a specific implementation framework.

---

60. Resource Negotiation

A statement may express a requirement or preference.

For example:

requires capability("tensor.compute")
prefer accelerator

The compiler/runtime may negotiate among available resources.

The statement parser must not resolve the negotiation.

---

61. Deployment

Deployment decisions belong downstream.

The source program describes portable intent.

The compiler/runtime may determine:

CPU
GPU
FPGA
ASIC
QPU
accelerator
cluster
HPC
cloud
edge
future architecture

according to capabilities and constraints.

The statements grammar remains unchanged.

---

62. Safety and Security

Grammar changes must not create parser-level escape hatches around:

- ownership;
- effects;
- capabilities;
- resource authorization;
- security policy;
- provenance;
- validation.

Macros and metaprogramming must also eventually pass through the same semantic validation pipeline.

---

63. Rust Compatibility

The grammar is ANTLR grammar source, not Rust source.

Nevertheless, all generated and consuming frontend code must remain compatible with the repository's supported Rust toolchain:

Rust 1.97
Rust 1.97.1

The grammar subsystem must therefore avoid assumptions that require newer Rust functionality.

The generated parser/frontend must use safe Rust only.

unsafe

is prohibited.

No grammar feature may require unsafe Rust for correctness.

---

64. ANTLR Compatibility

The statement grammar must remain compatible with the repository's selected ANTLR Rust generation stack.

The repository's existing ANTLR version/tooling is authoritative.

Do not introduce grammar constructs merely because they work with another ANTLR runtime if they are incompatible with the project's actual Rust target.

Grammar composition must use one consistent ANTLR architecture.

---

65. Generated Code

Generated parser/lexer output must not become a second source of truth.

The authoritative sources are:

grammar/*.g4
specification/*
spec/*

Generated artifacts must be reproducible.

Generated files should not be manually edited.

---

66. Testing Contract

Every statement feature requires:

positive
negative
boundary
scalability
determinism
compatibility
cross-domain
diagnostic

tests as applicable.

---

67. Positive Tests

Positive tests must demonstrate legal syntax.

Examples should cover:

- minimal form;
- normal form;
- nested form;
- generic form;
- cross-domain form;
- parameterized form;
- large symbolic/resource values.

---

68. Negative Tests

Negative tests must verify rejection of malformed syntax.

They should include:

- missing operands;
- malformed delimiters;
- invalid statement structure;
- invalid nesting;
- malformed resource expressions;
- malformed quantum operation syntax;
- malformed HDL syntax;
- ambiguous constructs where ambiguity is prohibited.

Negative tests must not confuse semantic rejection with syntax rejection.

---

69. Boundary Tests

Boundary tests should test:

- empty lists where legal;
- singleton lists;
- large lists;
- nested blocks;
- nested control flow;
- deeply composed expressions;
- large symbolic values;
- arbitrary operation names;
- arbitrary resource expressions.

Do not turn a test boundary into a language maximum.

---

70. Scalability Tests

Scalability tests must verify that the grammar does not encode artificial machine-size restrictions.

Test families should vary:

number of statements
number of blocks
number of bindings
number of operations
number of quantum operations
number of resources
number of parallel tasks
number of distributed participants
tensor dimensions
hardware parameters

The test suite should validate that syntax remains parameterized.

---

71. Determinism Tests

The same source must produce the same parser result repeatedly.

Tests should detect:

- ambiguous alternatives;
- nondeterministic behavior;
- accidental dependence on rule ordering;
- inconsistent domain dispatch.

---

72. Compatibility Tests

Every statement change must be checked against:

previous language versions
grammar/Zamani.g4
grammar/grammar.md
src/lexer.rs
src/parser.rs
frontend AST
semantic analyzer
canonical IR
existing examples
existing tests

Existing legal syntax should not be broken without an explicit compatibility decision.

---

73. Cross-Domain Tests

Required combinations include, where supported:

classical + quantum
classical + HDL
quantum + hardware
quantum + resources
quantum + concurrency
HDL + hardware
AI + data
AI + accelerator
distributed + networking
distributed + resources
security + networking
compile + hardware
execution + resources

The objective is to prove that domains compose rather than become isolated mini-languages.

---

74. Repository Integration Matrix

The statements subsystem integrates with:

Subsystem| Integration
"lexer/"| tokens
"core/"| names, attributes, blocks
"expressions/"| statement expressions
"types/"| type references
"declarations/"| declaration statements
"functions/"| returns/calls/function control
"modules/"| module-level statements
"effects/"| effect statements
"memory/"| memory-related statements
"concurrency/"| parallel/async statements
"classical/"| classical operations
"quantum/"| quantum operations
"hybrid/"| classical/quantum composition
"hdl/"| HDL statements
"hardware/"| target-independent hardware intent
"resources/"| requirements/capabilities
"distributed/"| distributed operations
"ai/"| AI/ML semantics
"data/"| data operations
"networking/"| communication
"security/"| security semantics
"compile/"| compilation intent
"execution/"| runtime intent
"interoperability/"| external formats
"dialects/"| controlled extensions
"validation/"| grammar validation
"compatibility/"| versioning
"tests/"| conformance
frontend AST| syntax-to-AST
semantic analysis| meaning
canonical IR| lowering
compiler| target realization
runtime| execution

---

75. Relationship to "Zamani.g4"

"Zamani.g4" remains the canonical top-level grammar composition root.

The statement subsystem must not replace it.

The intended relationship is:

Zamani.g4
   │
   ├── declarations
   ├── expressions
   ├── types
   └── statements
          │
          ├── universal statements
          └── domain statements

"Zamani.g4" should compose statement syntax.

The statement directory owns the modular statement implementation.

---

76. Relationship to "grammar.md"

"grammar/grammar.md" describes what the implementation actually accepts.

Statement grammar status must therefore be represented using the repository's conformance vocabulary:

SPECIFIED
IMPLEMENTED
PARTIALLY IMPLEMENTED
PLANNED
DEPRECATED

A statement must not be documented as implemented merely because its design appears in "Zamani-Grammar.md".

---

77. Relationship to "Zamani-Grammar.md"

"Zamani-Grammar.md" is not an automatic syntax authority.

Features from that document must go through:

proposal
→ specification
→ AST
→ canonical grammar
→ semantic implementation
→ IR
→ tests

before becoming stable Zamani statements.

Historical or experimental statement ideas remain clearly marked.

---

78. Relationship to "src/parser.rs"

The Rust parser implementation must consume the canonical grammar architecture.

It must not silently maintain a second statement grammar.

Any discrepancy must be recorded in the compatibility/conformance layer.

---

79. Relationship to "src/lexer.rs"

The statement grammar may consume tokens, but token definitions remain owned by the lexer layer.

Known duplicate lexical concepts such as:

Question / QuestionMark
Ampersand / BitAnd

must be resolved at the lexical authority level rather than patched independently in every statement grammar.

---

80. Relationship to Frontend AST

The frontend AST is domain-neutral.

Statement grammar rules must map to AST constructs without introducing:

- LLVM-specific nodes;
- QIR-specific nodes;
- MLIR-specific nodes;
- vendor-specific nodes;
- physical hardware nodes.

Quantum-specific semantics may be attached downstream through the established semantic boundary.

---

81. Relationship to Canonical Quantum IR

Quantum statement syntax ultimately lowers to:

quantum::ir

There must be exactly one canonical quantum semantic boundary.

The statements directory must never introduce another quantum IR.

---

82. Relationship to QEC, ZQN, Routing and Scheduling

Statements express intent.

They do not implement:

QEC
routing
scheduling
calibration
ZQN
HAL

The downstream pipeline determines how the requested computation is physically realized.

This preserves the separation:

what the programmer means

from:

how a particular machine executes it

---

83. POCO-REAF Contract

The statements subsystem is considered POCO-REAF compatible only if a statement's meaning remains independent of the target machine's:

- CPU count;
- core count;
- thread count;
- GPU count;
- FPGA size;
- QPU size;
- qubit count;
- memory capacity;
- register width;
- vector width;
- accelerator count;
- node count;
- topology;
- physical device identifiers.

The same source may therefore be compiled for different resource configurations.

---

84. Scaling Model

The intended model is:

one source program
        ↓
one semantic program
        ↓
one compilation model
        ↓
many possible resource configurations
        ↓
many possible target realizations

For example:

tiny embedded machine
        ↓
single CPU
        ↓
multicore CPU
        ↓
GPU
        ↓
FPGA
        ↓
ASIC
        ↓
QPU
        ↓
accelerator
        ↓
cluster
        ↓
HPC
        ↓
cloud
        ↓
future architecture

The statement syntax does not need to change merely because the target changes.

---

85. Physical Resource Selection

If physical resource selection is required, it must be represented as a downstream implementation decision unless the language explicitly declares a construct as non-portable.

The portable program should prefer:

capability
requirement
constraint
preference
hint

over physical identifiers.

---

86. Error Handling and Resource Failure

The statement grammar may express recovery policy where the language specifies such constructs.

Actual resource failure handling belongs to semantic/runtime systems.

For example:

recover
retry
fallback

may describe portable policy.

The runtime decides how recovery is implemented.

---

87. No Runtime Logic in Grammar

ANTLR grammar actions must not be used to implement:

- scheduling;
- allocation;
- quantum routing;
- QEC;
- device selection;
- networking;
- hardware discovery;
- runtime execution.

The grammar should remain declarative.

---

88. No Vendor Lock-In

The statement subsystem must not make vendor-specific hardware a requirement for understanding the core language.

Vendor functionality should enter through:

capabilities
dialects
interoperability
target profiles
backend extensions

with explicit compatibility rules.

---

89. Maintainability Rule

Every grammar file should be understandable in isolation.

A developer opening a statement grammar file must be able to determine:

Purpose
Owns
Does not own
Inputs
Outputs
Dependencies
Imported rules
AST mapping
Semantic mapping
IR mapping
Diagnostics
Tests
Compatibility
Scalability
Hard-coding policy
Completion criteria

without having to reverse-engineer another file.

This is a mandatory maintainability requirement.

---

90. Completion Checklist

A statement grammar file is DONE only when all applicable items below are complete.

Specification

- [ ] Purpose defined.
- [ ] Scope defined.
- [ ] Syntax specified.
- [ ] Feature status specified.
- [ ] Version compatibility defined.

Grammar

- [ ] Canonical rule names established.
- [ ] No duplicate ownership.
- [ ] No unintended ambiguity.
- [ ] No unintended left recursion.
- [ ] Correct expression integration.
- [ ] Correct type integration.
- [ ] Correct block integration.

AST

- [ ] AST mapping defined.
- [ ] Source spans defined.
- [ ] Attributes/modifiers mapped.
- [ ] Error representation defined.

Semantics

- [ ] Semantic rules defined.
- [ ] Invalid combinations defined.
- [ ] Capability requirements defined.
- [ ] Resource requirements defined.
- [ ] Effects defined.

IR

- [ ] Canonical IR destination identified.
- [ ] No duplicate IR created.
- [ ] Lowering responsibility identified.

Compiler

- [ ] Compiler consumer identified.
- [ ] Optimization interaction identified.
- [ ] Target-lowering interaction identified.

Runtime

- [ ] Runtime consumer identified where applicable.
- [ ] Resource discovery boundary identified.
- [ ] Scheduling boundary identified.
- [ ] Deployment boundary identified.

Scalability

- [ ] No fixed hardware limits.
- [ ] No fixed resource counts.
- [ ] No fixed topology.
- [ ] No fixed device IDs.
- [ ] No artificial operation limits.
- [ ] Large symbolic/program values supported.

Testing

- [ ] Positive tests.
- [ ] Negative tests.
- [ ] Boundary tests.
- [ ] Scalability tests.
- [ ] Determinism tests.
- [ ] Compatibility tests.
- [ ] Cross-domain tests where applicable.
- [ ] Diagnostic tests.

Safety

- [ ] No unsafe Rust requirement.
- [ ] No parser-side execution.
- [ ] No semantic bypass.
- [ ] No vendor lock-in.
- [ ] No hidden target dependency.

---

91. Definition of Production Ready

The "statements/" subsystem is production-ready only when:

Every statement has one owner
        AND
Every statement has a specification
        AND
Every statement has an AST mapping
        AND
Every statement has semantic rules
        AND
Every statement has an IR destination
        AND
Every statement has diagnostics
        AND
Every statement has conformance tests
        AND
Every statement has compatibility rules
        AND
Every statement passes scalability audits
        AND
No statement encodes artificial hardware limits
        AND
No statement creates a competing semantic model
        AND
The complete subsystem composes through Zamani.g4

---

92. Final Architecture

The finished statement architecture is:

                         Zamani.g4
                             │
                             ▼
                     statement.g4
                             │
        ┌────────────┬───────┴────────┬───────────────┐
        ▼            ▼                ▼               ▼
    bindings     control          loops           match
        │            │                │               │
        └────────────┴────────────────┴───────────────┘
                             │
        ┌────────────────────┼─────────────────────────┐
        ▼                    ▼                         ▼
   concurrency           resources                 effects
        │                    │                         │
        └────────────────────┼─────────────────────────┘
                             ▼
                         domains.g4
                             │
       ┌────────────┬────────┼────────┬─────────────┐
       ▼            ▼        ▼        ▼             ▼
   classical     quantum    hybrid    HDL       distributed
       │            │        │        │             │
       └────────────┴────────┴────────┴─────────────┘
                             │
                             ▼
                    domain-neutral AST
                             │
                             ▼
                    semantic analysis
                             │
             ┌───────────────┼────────────────┐
             ▼               ▼                ▼
       Classical IR     quantum::ir       HDL/Hardware
             │               │                │
             └───────────────┼────────────────┘
                             ▼
                   optimization/lowering
                             │
             ┌───────────────┼────────────────┐
             ▼               ▼                ▼
          routing        scheduling        resilience
                             │
                             ▼
                            ZQN
                             │
                             ▼
                            HAL
                             │
                             ▼
                    target realization

---

93. Core Principle

The statement grammar must describe what the program means, not what today's machine happens to look like.

Therefore:

Zamani statements
        =
portable computational intent

while:

resource discovery
capability negotiation
optimization
routing
scheduling
QEC
ZQN
HAL
deployment
physical realization

remain downstream concerns.

This separation is the foundation required for Zamani to scale from the smallest supported computational substrate to arbitrarily large available resources while preserving:

«Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.»

---

94. Non-Negotiable Rules

The following rules apply to every current and future statement grammar:

1. One statement concept has one authoritative owner.
2. No competing universal statement grammar.
3. "Zamani.g4" remains the canonical composition root.
4. "statement.g4" remains the statement composition boundary.
5. "domains.g4" remains a domain dispatch/composition boundary.
6. Detailed domain syntax remains in domain-owned grammar files.
7. Expressions come from the canonical expression grammar.
8. Types come from the canonical type grammar.
9. Declarations come from the canonical declaration grammar.
10. AST mappings are defined before implementation is considered complete.
11. Semantic mappings are defined before implementation is considered complete.
12. IR mappings are defined before implementation is considered complete.
13. Quantum syntax lowers through the canonical "quantum::ir".
14. No second quantum IR may be introduced.
15. No fixed universal quantum-gate enumeration.
16. No fixed machine/resource/topology limits.
17. No physical device identifiers as universal language assumptions.
18. No framework-specific AI language.
19. No vendor-specific hardware dependency in core syntax.
20. No parser-side execution.
21. No unsafe Rust requirement.
22. Rust consumers remain compatible with Rust 1.97/1.97.1.
23. Every feature has positive, negative, boundary and scalability coverage.
24. Cross-domain composition is tested.
25. Existing filenames are retained unless there is a compelling architectural reason to change them.
26. Historical design documents do not silently become syntax authorities.
27. Generated documentation does not become a grammar authority.
28. Semantic requirements remain separate from implementation decisions.
29. Resource availability is resolved downstream.
30. The same source program must remain meaningful across different target scales.

---

95. Completion Statement

When this README's contract is satisfied, "grammar/statements/" is not merely a collection of ANTLR files.

It is a formally bounded statement-language subsystem with:

clear ownership
+
stable composition
+
AST traceability
+
semantic traceability
+
IR traceability
+
compiler integration
+
runtime integration
+
cross-domain integration
+
diagnostics
+
compatibility
+
determinism
+
scalability
+
POCO-REAF portability

The resulting architecture permits Zamani statements to remain stable while implementation moves from:

atom
→ embedded
→ CPU
→ multicore
→ GPU
→ FPGA
→ ASIC
→ QPU
→ accelerator
→ cluster
→ HPC
→ distributed/cloud
→ future computational substrates

without requiring the statement grammar to encode the physical limits of any particular generation of hardware.
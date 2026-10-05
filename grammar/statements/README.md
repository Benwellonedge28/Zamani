Zamani Statements Grammar

Path: "grammar/statements/"
Language: Zamani
Status: Production architecture and statement-subsystem contract
Implementation baseline: Rust 2021, Rust 1.97 / Rust 1.97.1
Safety requirement: Zamani's Rust implementation MUST NOT use "unsafe"
Portability model: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)

---

1. Purpose

"grammar/statements/" defines the statement-level syntax boundary of Zamani.

It describes program actions and control structures without deciding how those actions are implemented on a particular CPU, GPU, FPGA, ASIC, QPU, accelerator, embedded system, cluster, supercomputer, network, cloud environment, simulator, or future computational substrate.

The statement subsystem participates in:

Zamani source
    │
    ▼
lexical analysis
    │
    ▼
canonical ANTLR grammar
    │
    ▼
statement grammar
    │
    ▼
domain-neutral frontend AST
    │
    ▼
structural validation
    │
    ├── names
    ├── types
    ├── ownership
    ├── effects
    ├── capabilities
    ├── resources
    ├── contracts
    ├── policies
    └── provenance
    │
    ▼
semantic model
    │
    ├── classical
    ├── quantum
    ├── hybrid
    ├── HDL
    ├── hardware
    ├── AI/model computation
    ├── data
    ├── distributed
    ├── networking
    ├── accelerators
    └── future domains
    │
    ▼
canonical IR / domain IR
    │
    ├── classical representation
    ├── quantum::ir
    └── other domain representations
    │
    ▼
optimization
    │
    ▼
lowering
    │
    ▼
routing / scheduling / resilience
    │
    ▼
ZQN / HAL
    │
    ▼
target realization

This directory therefore owns syntax composition, not execution.

---

2. Core Architectural Principle

The statement grammar MUST describe intent and program structure, not physical machine realization.

A statement may express:

- computation;
- sequencing;
- branching;
- iteration;
- matching;
- binding;
- assignment;
- return;
- failure handling;
- assertion;
- reasoning;
- knowledge operations;
- learning;
- controlled adaptation;
- contracts;
- policies;
- effects;
- capabilities;
- resource requirements;
- concurrency;
- simulation intent;
- quantum computation;
- hybrid computation;
- HDL intent;
- domain operations.

The grammar MUST NOT require the programmer to know the eventual machine.

Therefore:

logical computation
        ↓
semantic requirements
        ↓
capability negotiation
        ↓
resource negotiation
        ↓
specialization
        ↓
target realization

rather than:

source statement
        ↓
specific CPU/GPU/QPU/device

---

3. Authority Model

The statement subsystem follows the repository-wide grammar authority model.

The authority hierarchy is:

grammar/DESIGN.md
        ↓
grammar/specification/
        ↓
grammar/spec/
        ↓
grammar/Zamani.g4
        ↓
grammar/antlr/
        ↓
grammar/statements/
        ↓
frontend AST
        ↓
semantic implementation
        ↓
canonical/domain IR

"grammar/DESIGN.md"

Owns repository-wide grammar architecture.

"grammar/specification/"

Owns normative human-readable language specifications.

"grammar/spec/"

Owns machine-checkable feature contracts and formal subsystem rules.

"grammar/Zamani.g4"

Owns the top-level canonical ANTLR composition.

It MUST NOT become a second statement implementation.

"grammar/antlr/ZamaniParser.g4"

Owns the generated-parser composition boundary where applicable.

"grammar/statements/statements.g4"

Owns the single universal "statement" parser rule for the statement subsystem.

"grammar/statements/*.g4"

Own individual statement families.

"grammar/grammar.md"

Describes implementation/conformance status.

"grammar/Zamani-Grammar.md"

Contains historical, proposed, extended, experimental, deprecated, or otherwise non-authoritative language material.

Its contents do not automatically become legal Zamani syntax.

"grammar/tests/"

Provides executable conformance evidence.

---

4. The Existing Composition Root Must Be Retained

The repository already contains:

grammar/statements/statements.g4

This file is the canonical statement composition root.

Therefore:

«Do not introduce "grammar/statements/statement.g4" as another universal statement root.»

The singular concept is:

statements.g4
    └── statement

not:

statement.g4
statements.g4

with two competing definitions.

This is an important correction to previous planning.

"statements.g4" MUST remain the sole owner of the universal:

statement

rule in the assembled production grammar.

---

5. Directory Ownership

"grammar/statements/" owns statement syntax and statement-level composition.

It does not own:

- lexical definitions;
- token definitions;
- identifier definitions;
- expression precedence;
- type semantics;
- type inference;
- ownership checking;
- borrow checking;
- semantic analysis;
- AST implementation;
- IR implementation;
- quantum IR;
- QEC;
- routing;
- scheduling;
- calibration;
- hardware discovery;
- resource allocation;
- target selection;
- runtime execution;
- vendor APIs;
- ABI implementation;
- FFI implementation;
- machine topology;
- physical capacity.

Those belong to their existing repository subsystems.

---

6. Existing Statement Files

The repository currently contains a substantially broader statement directory than a minimal statement grammar.

Existing statement-level files include, among others:

README.md

statements.g4

assignments.g4
bindings.g4
blocks.g4
breaks.g4
conditionals.g4
concurrency.g4
continues.g4
control-flow.g4
declarations.g4
domains.g4
effects.g4
exceptions.g4
explain.g4
hdl.g4
infer.g4
learn.g4
loops.g4
match.g4
pattern-matching.g4
policy.g4
quantum.g4
query.g4
reason.g4
retract.g4
returns.g4
resource.g4
sandbox.g4
simulate.g4
contract.g4
deduce.g4
adapt.g4
assertions.g4
unsafe.g4

The exact current file inventory is repository state and MUST remain authoritative.

The presence of multiple files does not mean every file is independently authoritative.

The production rule is:

«One semantic responsibility → one authoritative grammar owner → composition through adapters/composition grammars.»

---

7. Statement Ownership Matrix

File| Owns| Does not own
"statements.g4"| universal statement composition| concrete feature syntax
"assignments.g4"| assignment statements| type checking
"bindings.g4"| binding syntax| type semantics
"blocks.g4"| statement-level block compatibility where applicable| scope semantics
"conditionals.g4"| conditional syntax| control-flow analysis
"control-flow.g4"| control-flow composition| target scheduling
"loops.g4"| iteration syntax| iteration limits imposed by hardware
"match.g4"| match syntax| exhaustiveness/type analysis
"pattern-matching.g4"| pattern syntax if retained as a leaf/composition owner| duplicate match ownership
"breaks.g4"| break syntax| loop implementation
"continues.g4"| continue syntax| scheduler behavior
"returns.g4"| return/exit syntax| ABI
"exceptions.g4"| exception syntax| runtime exception engine
"assertions.g4"| assertion syntax| proof/verification engine
"concurrency.g4"| concurrency composition| physical worker allocation
"effects.g4"| effect statement boundary| effect execution
"resource.g4"| resource statement boundary| resource allocation
"reason.g4"| generic reasoning statement family| reasoning algorithms
"infer.g4"| inference leaf only if retained| universal reasoning ownership
"deduce.g4"| deduction leaf only if retained| universal reasoning ownership
"query.g4"| query statement boundary| database implementation
"retract.g4"| knowledge retraction syntax| knowledge-store implementation
"learn.g4"| learning statement boundary| ML algorithm implementation
"adapt.g4"| controlled adaptation syntax| unrestricted self-modification
"contract.g4"| contract statement boundary| verification engine
"policy.g4"| policy statement boundary| authorization engine
"explain.g4"| explanation request syntax| explanation generation
"simulate.g4"| simulation intent| simulator implementation
"sandbox.g4"| sandbox intent| sandbox runtime
"domains.g4"| domain statement composition| domain semantics
"quantum.g4"| quantum statement boundary| quantum IR/QEC/routing
"hdl.g4"| HDL statement boundary| synthesis/physical implementation
"declarations.g4"| declaration statement integration| declaration semantics
"unsafe.g4"| legacy/proposed unsafe-language boundary if retained| unsafe Rust implementation

Where two files currently own overlapping syntax, the repository MUST converge to one owner.

---

8. Universal Statement Rule

"statements.g4" owns:

statement

There MUST be exactly one effective universal statement rule after all ANTLR imports are composed.

Specialized grammars expose specialized rules.

For example:

statement
├── declarationStatement
├── assignmentStatement
├── assertionStatement
├── controlFlowStatement
├── concurrencyStatement
├── effectStatement
├── resourceStatement
├── reasoningStatement
├── domainStatement
├── blockStatement
└── expressionStatement

The exact rule names must follow the actual repository grammar.

The important invariant is ownership, not arbitrary naming.

---

9. No Duplicate Statement Roots

The following MUST NOT coexist as independent universal roots:

statement

in:

statements.g4
control-flow.g4
domains.g4
concurrency.g4
quantum.g4
hdl.g4

Domain files must instead expose specialized rules.

For example:

quantumStatement

rather than another:

statement

---

10. Composition Direction

Dependencies flow downward:

leaf grammar
    ↓
feature composition
    ↓
statement composition
    ↓
parser composition

Never upward:

statement grammar
    X
    ↓
ZamaniParser

and never cyclically:

A → B → C → A

ANTLR imports MUST remain acyclic.

---

11. Statement-to-AST Boundary

The statement grammar produces parser contexts.

It does not define the AST.

The frontend path is:

lexer
    ↓
ANTLR parser
    ↓
statement parser context
    ↓
domain-neutral AST

The AST MUST remain domain-neutral.

The AST must not contain parser-level assumptions such as:

physical_gpu
physical_qubit
qec_distance
warp_width
fpga_region
cpu_core

unless those are explicitly semantic values in a target-specific representation outside the universal source AST.

---

12. Expression Integration

Statements reuse:

grammar/expressions/

for expressions.

The statement grammar must not recreate:

- arithmetic;
- boolean expressions;
- function calls;
- indexing;
- member access;
- lambda syntax;
- query expressions;
- type expressions;
- quantum expression syntax.

If an expression can appear as the operand of a statement, the canonical expression grammar owns it.

The boundary is:

statement
    ↓
expression

not:

statement
    └── private expression grammar

---

13. Type Integration

Statements consume types through:

grammar/types/

and downstream semantic analysis.

Statement grammar MUST NOT decide:

- type compatibility;
- generic substitution;
- ownership;
- lifetime;
- linearity;
- affinity;
- dependent constraints;
- associated types;
- type-class resolution;
- type inference.

For example, whether:

return value;

is type-correct belongs to semantic analysis.

The grammar only recognizes the structure.

---

14. Declaration Integration

"declarations.g4" is the statement-level boundary for declaration constructs where declarations are permitted in statement position.

It integrates with:

grammar/declarations/
grammar/types/
grammar/functions/
grammar/modules/
grammar/core/

The statement subsystem must not duplicate declaration grammar.

---

15. Assignment and Binding Integration

Assignments and bindings must remain distinct semantic concepts.

A binding may introduce a name.

An assignment may update an existing location/value.

The statement grammar must preserve enough structure for semantic analysis to distinguish:

declaration
binding
assignment
reassignment
destructuring
pattern binding

The grammar must not decide whether mutation is legal.

That belongs to:

types/
effects/
memory/
semantic analysis

---

16. Control Flow

The control-flow subsystem includes:

- conditional execution;
- loops;
- match;
- break;
- continue;
- return;
- exception control;
- structured control flow.

The canonical composition is:

control-flow.g4
    ├── conditionals.g4
    ├── loops.g4
    ├── match.g4
    ├── breaks.g4
    ├── continues.g4
    ├── returns.g4
    └── exceptions.g4

The exact import architecture must match the repository's actual ANTLR grammar names.

No leaf should be imported twice through competing composition paths.

---

17. Loops and Unbounded Scalability

Loops must not encode machine limits.

Forbidden as language semantics:

MAX_LOOP_DEPTH
MAX_ITERATIONS
MAX_LOOP_BODY

A loop may have:

- finite iteration;
- condition-controlled iteration;
- collection iteration;
- range iteration;
- streaming behavior where specified;
- parallel iteration where specified.

Physical execution limits belong downstream.

For example:

program requests iteration
        ↓
semantic analysis
        ↓
resource planning
        ↓
target realization

A target may be unable to execute a program because of available resources, but that does not change the language's universal loop semantics.

---

18. Pattern Matching

Pattern matching integrates:

grammar/statements/match.g4
grammar/statements/pattern-matching.g4
grammar/expressions/
grammar/types/

There MUST be one authoritative pattern representation.

The statement grammar must not independently implement:

- type narrowing;
- exhaustiveness;
- unreachable-pattern analysis;
- ownership analysis;
- refinement checking.

Those belong to semantic analysis.

---

19. Contracts and Assertions

Statements may express:

assert
requires
ensures
invariant
assume
guarantee
property

where these are part of the canonical language specification.

Their semantic flow is:

statement syntax
    ↓
contract/assertion AST
    ↓
validation
    ↓
semantic checking
    ↓
verification/runtime enforcement/optimization

The grammar MUST NOT implement the proof engine.

Contracts must remain portable.

A contract can constrain program meaning without describing a particular physical implementation.

---

20. Reasoning

The statement subsystem supports generic reasoning through the canonical reasoning family.

Conceptually:

reason
infer
deduce

should converge on a common semantic reasoning representation.

The grammar should not create independent semantic universes for:

infer
deduce
reason

If the repository retains:

infer.g4
deduce.g4

they must be subordinate leaf grammars or compatibility components of the canonical reasoning composition.

The preferred dependency is:

reasoning syntax
      ↓
reason.g4
      ↓
reasoning AST
      ↓
semantic reasoning model

Reasoning algorithms are not grammar rules.

They may be implemented through:

- libraries;
- semantic engines;
- theorem provers;
- model runtimes;
- inference engines;
- optimization systems;
- domain-specific reasoning systems.

---

21. Knowledge Operations

The statement layer may support generic knowledge operations such as:

assert
retract
query

where those forms are established by the canonical language specification.

They must integrate with:

grammar/data/
grammar/ai/
grammar/validation/
grammar/spec/

The statement grammar must not decide whether the underlying store is:

- a graph;
- a relational database;
- an in-memory structure;
- a distributed store;
- a knowledge engine;
- a hardware-backed store;
- a future implementation.

The AST represents intent.

---

22. Learning

Learning statements must describe learning intent rather than a fixed algorithm catalog.

A learning construct may semantically involve:

input
data
model
objective
algorithm
constraints
resources
capabilities
effects
policy
provenance

The statement grammar MUST NOT enumerate every possible learning algorithm.

For example, adding a future learning algorithm must not require changing the universal statement grammar merely because the algorithm is new.

---

23. Controlled Adaptation

"adapt.g4" represents controlled adaptation.

Adaptation MUST NOT mean unrestricted compiler or runtime self-modification.

The semantic path is:

adaptation request
    ↓
policy
    ↓
authorization
    ↓
capability check
    ↓
effect check
    ↓
resource check
    ↓
provenance
    ↓
validated state/model/strategy change

Adaptation may apply to:

- models;
- strategies;
- plans;
- execution choices;
- resource preferences;
- routing choices;
- recovery strategies.

The grammar itself does not perform adaptation.

---

24. Explainability and Decisions

"explain.g4" provides statement-level syntax for requesting or declaring explanations.

It may be used for:

- model decisions;
- reasoning;
- compiler transformations;
- optimization choices;
- resource allocation;
- quantum transformations;
- routing;
- scheduling;
- security decisions;
- adaptation decisions.

The statement grammar does not generate explanations.

It provides a portable representation of explanation intent.

---

25. Evidence and Provenance

Statement constructs that produce reasoning, learning, adaptation, decisions, or transformations must remain compatible with the repository's provenance model.

Provenance may record:

source
derived_from
generated_by
transformed_by
verified_by
reason
evidence
decision
version
time

The statement grammar does not own provenance storage.

It only provides syntax where provenance-related statements are part of the language.

---

26. Uncertainty

Uncertainty belongs primarily to:

grammar/types/
grammar/expressions/
grammar/ai/
grammar/data/

rather than becoming a completely separate statement language.

Statements may consume uncertainty-bearing values.

For example, a reasoning or decision statement can operate on a value whose semantic type expresses:

probability
distribution
confidence
belief
uncertainty

The statement grammar must not hard-code a probability implementation.

---

27. Policy Integration

Policies may constrain:

- execution;
- security;
- resource use;
- adaptation;
- simulation;
- deployment;
- network operations;
- quantum execution;
- distributed execution.

Statement syntax belongs in:

grammar/statements/policy.g4

where appropriate.

Policy semantics belong in:

grammar/policies/
grammar/security/
grammar/resources/
grammar/effects/

A policy must not directly select physical hardware from the statement grammar.

---

28. Resource Requirements

"resource.g4" is the statement-layer interface to:

grammar/resources/

The resource model distinguishes:

requirement
capability
constraint
budget
preference
hint
negotiation
placement intent
scaling intent

These are different semantic categories.

For example:

requires capability("quantum.measurement")

is a capability requirement.

requires memory >= required_memory

is a resource requirement.

prefer capability("accelerator.compute")

is a preference.

None of these means:

use physical device 0

unless a separate target-specific facility explicitly requests physical placement.

---

29. POCO-REAF Resource Model

POCO-REAF requires the source language to describe program meaning without imposing a universal machine size.

Therefore statements MUST NOT contain universal ceilings for:

qubits
CPUs
GPUs
FPGAs
ASICs
nodes
memory
threads
workers
tensor rank
register width
network size
device count
channels
actors
parallel regions
storage

Resource requirements may depend on program values and symbolic quantities.

For example:

requires qubits >= required_qubits

is portable.

A grammar rule such as:

qubits : INTEGER[1..1024]

is not portable if "1024" is merely a machine limitation.

---

30. Concurrency

Concurrency syntax belongs to:

grammar/statements/concurrency.g4
grammar/concurrency/

Possible semantic concepts include:

- task;
- spawn;
- async;
- await;
- actor;
- channel;
- synchronization;
- cancellation;
- parallel computation;
- pipeline;
- collective operation;
- structured concurrency;
- distributed concurrency.

Concurrency MUST remain independent of physical execution width.

The source program may express:

parallel computation

without saying:

8 threads
16 cores
4 GPUs
1024 workers

The compiler/runtime determines an admissible realization.

---

31. AI-Agent Integration

Agent syntax must reuse the existing concurrency model.

Preferred architecture:

AI agent
    ↓
actor/task abstraction
    ↓
message/channel
    ↓
concurrency subsystem
    ↓
scheduler

The statement grammar must not create a second actor runtime.

An agent may combine:

reasoning
knowledge
learning
adaptation
planning
communication
policy
provenance

but those capabilities remain semantically composed from existing subsystems.

---

32. Simulation

"simulate.g4" expresses simulation intent.

Simulation may cover:

- classical computation;
- quantum computation;
- hardware behavior;
- distributed systems;
- AI/model behavior;
- fault scenarios;
- performance;
- resource behavior.

Simulation is an execution strategy.

It is not a second language.

The path is:

simulation statement
    ↓
domain-neutral AST
    ↓
semantic simulation intent
    ↓
execution planning
    ↓
appropriate simulator

The grammar does not implement the simulator.

---

33. Sandbox

"sandbox.g4" provides syntax for sandbox intent where defined by the specification.

Sandbox policies may constrain:

effects
capabilities
resources
network
filesystem
native calls
FFI
reflection
adaptation
code generation

The statement grammar must not itself enforce the sandbox.

Enforcement belongs to:

grammar/security/
execution/
runtime
capability system

---

34. Quantum Integration

Quantum statements must integrate with:

grammar/quantum/
grammar/hybrid/
grammar/resources/
grammar/effects/
grammar/execution/

and the compiler's quantum pipeline.

The canonical path is:

Zamani quantum statement
        ↓
domain-neutral AST
        ↓
quantum semantic model
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
target realization

"grammar/statements/" MUST NOT define a second quantum IR.

---

35. Quantum Operation Extensibility

The statement grammar MUST NOT enumerate a permanent universal list of quantum operations.

It must not assume:

H
X
Y
Z
CNOT

are the complete universe of quantum operations.

The syntax must permit operation descriptions based on semantic data such as:

operation name
operands
parameters
results
attributes
modifiers

This permits:

- standard operations;
- custom operations;
- vendor operations;
- parameterized operations;
- logical operations;
- decomposed operations;
- future operations.

Adding a new quantum operation should normally be a semantic/dialect/operation-registry change, not a redesign of the universal statement grammar.

---

36. Quantum Scalability

The grammar must not impose maximum values for:

qubits
registers
operations
circuit depth
controls
measurements
devices
kernels
classical feed-forward operations

Program-defined values may express resource requirements.

Physical feasibility is determined later.

---

37. Hybrid Computing

The statement layer must permit the same source program to combine:

classical
quantum
AI/model computation
HDL
accelerators
distributed execution

For example:

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
AI/model processing

No separate source language is required.

The integration is semantic.

---

38. HDL Integration

"hdl.g4" provides the statement boundary for HDL-related constructs.

It integrates with:

grammar/hdl/
grammar/hardware/
grammar/resources/
grammar/compile/
grammar/execution/

The statement grammar must describe hardware intent rather than force a particular implementation.

It must not impose universal constructs such as:

wire [31:0]

as a universal language limitation.

Parameterized hardware descriptions remain parameterized.

Physical implementation belongs to synthesis, placement, routing, timing, and target realization.

---

39. Hardware Separation

Statement syntax must distinguish:

intent
requirement
capability
constraint
preference
placement
physical realization

For example:

requires capability("tensor.compute")

does not mean:

run_on_gpu(0)

The former is portable.

The latter is target-specific and belongs outside universal statement semantics unless explicitly introduced by a target-specific dialect.

---

40. Effects

Statement-level effects integrate with:

grammar/effects/

Potential effects include:

io
network
mutation
randomness
native
foreign
distributed
measurement
learning
adaptation
reflection
code_generation
simulation

The grammar only represents effect-related syntax.

It does not execute effects.

Semantic effect checking occurs after AST construction.

---

41. Error Handling

"exceptions.g4" provides structural syntax for failure handling.

The semantic model must integrate with:

grammar/effects/
grammar/execution/
grammar/validation/
diagnostics

Parser errors are structural.

Examples:

missing handler
malformed catch structure
malformed propagation syntax

Resource failures are not parser errors.

Examples:

insufficient memory
unavailable capability
unavailable QPU
unsupported topology
resource exhaustion

Those belong downstream.

---

42. Return and Control Transfer

"returns.g4", "breaks.g4", and "continues.g4" provide structured control transfer.

They must not implement:

- calling conventions;
- ABI layout;
- stack layout;
- register allocation;
- physical return registers.

Those belong to:

grammar/functions/
grammar/interoperability/
grammar/compile/
backend

---

43. Expression Statements

Where expressions are permitted in statement position:

expression
    ↓
expressionStatement

The statement grammar must not duplicate expression grammar.

Expression precedence remains owned by:

grammar/expressions/

---

44. Domain Dispatch

"domains.g4" is the domain composition boundary.

It may compose statement families for:

classical
quantum
hybrid
HDL
AI/model
data
distributed
networking
hardware
security
execution
compile

It must not become a second monolithic universal statement grammar.

The dependency direction is:

domain statement
    ↓
domains.g4
    ↓
statements.g4

not:

domain grammar
    ↓
another universal statement

---

45. Dialect Integration

Dialect-specific statement syntax belongs under:

grammar/dialects/

where appropriate.

A dialect may extend Zamani syntax only through the defined dialect mechanism.

A dialect MUST NOT:

- bypass the canonical AST;
- bypass semantic validation;
- create a private universal statement rule;
- create a private effect system;
- create a private resource system;
- create a private quantum IR;
- select physical hardware during parsing;
- require unsafe Rust.

---

46. Macros and Metaprogramming

Macro-generated statements must enter the same validation pipeline as handwritten statements.

The required path is:

macro source
    ↓
macro expansion
    ↓
canonical statement grammar / AST
    ↓
semantic validation

Macro expansion must not provide an escape from:

- type checking;
- effect checking;
- capability checking;
- resource checking;
- contracts;
- policies;
- provenance;
- portability validation.

---

47. FFI and ABI

FFI is owned by:

grammar/interoperability/

and associated compiler/runtime subsystems.

Statements may invoke foreign functionality only through the canonical FFI boundary.

Statement grammar must not define ABI layout.

An FFI statement ultimately participates in:

statement
    ↓
AST
    ↓
effect/capability analysis
    ↓
FFI semantic model
    ↓
ABI lowering

FFI should carry appropriate effects and capabilities.

---

48. Reflection and Compile-Time Operations

Reflection and metaprogramming belong to:

grammar/metaprogramming/
grammar/macros/
grammar/compile/

Statements may expose compile-time operations where specified.

They must remain explicitly represented so that semantic analysis can distinguish:

compile-time
run-time
simulation-time
deployment-time

No statement may silently execute arbitrary host-language code during parsing.

---

49. Determinism

The statement grammar must be deterministic with respect to the same:

source
lexer configuration
grammar version
dialect configuration

Repeated parsing must produce equivalent parser structure.

No grammar action may depend on:

- wall-clock time;
- filesystem state;
- network state;
- hardware discovery;
- random state;
- runtime state.

---

50. Source Spans

Statement parser contexts must preserve source locations through the existing lexer/parser infrastructure.

The statement subsystem must not reconstruct source locations from textual guesses.

The canonical frontend remains responsible for:

file
offset
line
column
span
diagnostic location

This is essential for production diagnostics and tooling.

---

51. Diagnostics

The grammar should produce structurally meaningful parser errors.

Diagnostics should eventually distinguish:

syntax error
semantic error
type error
effect error
capability error
resource error
contract violation
policy violation
portability failure
target feasibility failure
runtime failure

The statement grammar itself is responsible only for syntax-level failures.

---

52. Security Requirements

The grammar MUST be declarative.

It must not:

- perform I/O;
- access the filesystem;
- access the network;
- inspect hardware;
- execute user programs;
- access secrets;
- invoke foreign functions;
- allocate target resources;
- mutate runtime state.

ANTLR actions and predicates must not introduce hidden execution behavior.

---

53. Rust Safety Requirements

The Rust implementation surrounding this grammar MUST use:

Rust 2021
Rust 1.97
Rust 1.97.1

and MUST NOT require "unsafe".

This README does not authorize unsafe Rust.

The grammar itself should contain no embedded Rust actions.

The safe architecture is:

grammar
    ↓
generated parser
    ↓
safe Rust frontend
    ↓
AST
    ↓
semantic analysis

---

54. Scalability Requirements

The statement grammar must scale with the program, not with a fixed machine model.

It must support source programs whose size and complexity grow according to available implementation resources.

There must be no language-level ceiling for:

- number of statements;
- number of blocks;
- number of bindings;
- number of functions invoked;
- number of branches;
- loop iterations;
- nested structures;
- concurrent tasks;
- actors;
- channels;
- devices;
- nodes;
- quantum operations;
- qubits;
- hardware components;
- data objects.

Implementation limits may exist for practical reasons, but such limits MUST NOT be encoded as language semantics.

---

55. "Infinity" and Resource Reality

"Infinity" in the POCO-REAF requirement means that the grammar must not impose an artificial finite universal capacity.

It does not mean that a physical compiler, machine, simulator, or runtime has mathematically infinite resources.

The correct model is:

language capacity
    ≠
physical capacity

A program may remain unchanged while a target reports:

capability unavailable
resource insufficient
unsupported constraint

rather than requiring source rewriting merely because the target is smaller.

---

56. Resource Negotiation

The statement layer participates in:

requirements
    ↓
capabilities
    ↓
constraints
    ↓
preferences
    ↓
negotiation
    ↓
execution planning

The statement grammar does not choose the final realization.

This enables the same source program to be considered for:

tiny embedded target
CPU
multicore
GPU
FPGA
ASIC
accelerator
QPU
simulator
HPC
cluster
distributed environment
cloud
future computational substrate

without changing the statement grammar.

---

57. Reproducibility

Statements that influence:

- randomness;
- learning;
- adaptation;
- scheduling;
- distributed behavior;
- simulation;

must remain compatible with the repository's reproducibility model.

Reproducibility metadata belongs downstream.

The grammar should preserve the necessary structure without embedding implementation-specific random seeds, machine identifiers, or physical topology.

---

58. Provenance

Statement transformations may require provenance.

The semantic pipeline may record:

source statement
    ↓
AST
    ↓
semantic transformation
    ↓
optimization
    ↓
lowering
    ↓
target realization

with relationships such as:

derived_from
generated_by
transformed_by
verified_by
selected_because
constrained_by

The statement grammar itself does not own provenance storage.

---

59. Compatibility

Statement syntax must follow repository language-version rules.

Compatibility must distinguish:

stable
experimental
proposed
deprecated
historical
not implemented

A statement should not silently change meaning between versions.

If syntax changes, the compatibility subsystem must define:

version
migration
deprecation
diagnostic
compatibility behavior

The statement README records architecture; it does not independently define language-version policy.

---

60. Legacy and Duplicate Files

The current statement directory contains overlapping concepts such as:

conditionals.g4
control-flow.g4

match.g4
pattern-matching.g4

infer.g4
deduce.g4
reason.g4

assignments.g4
bindings.g4

statements.g4

This is not automatically wrong, but each overlap must have an explicit ownership decision.

The production rule is:

one canonical owner
        +
optional compatibility/delegation leaf
        +
one composition path

There must not be two grammars independently accepting the same construct with different parse trees.

---

61. Required Consolidation Rules

Before declaring this directory production-ready:

Control flow

control-flow.g4

must be the composition owner for control-flow leaves.

Reasoning

reason.g4

must be the canonical reasoning composition boundary where appropriate.

"infer.g4" and "deduce.g4" must not independently compete with it.

Matching

Either:

match.g4

or:

pattern-matching.g4

must own the relevant canonical syntax.

If both remain, one must explicitly compose/delegate to the other.

Statements

statements.g4

must remain the sole universal statement composition root.

Concurrency

"concurrency.g4" under the statement directory must integrate with the canonical:

grammar/concurrency/

rather than creating a second concurrency language.

Quantum

"quantum.g4" must delegate to:

grammar/quantum/

and ultimately:

quantum::ir

HDL

"hdl.g4" must delegate to:

grammar/hdl/
grammar/hardware/

and must not implement synthesis.

---

62. Integration Contract for Every Statement File

Every ".g4" file under this directory MUST have an explicit file contract containing:

PURPOSE
OWNS
DOES_NOT_OWN

DEPENDS_ON
IMPORTS
EXPORTS
CONSUMED_BY

LEXER_DEPENDENCIES
GRAMMAR_DEPENDENCIES

AST_MAPPING
SEMANTIC_MAPPING
TYPE_MAPPING
EFFECT_MAPPING
CAPABILITY_MAPPING
RESOURCE_MAPPING
CONTRACT_MAPPING
POLICY_MAPPING
PROVENANCE_MAPPING

IR_DESTINATION

QUANTUM_BOUNDARY
HDL_BOUNDARY
DOMAIN_BOUNDARY
BACKEND_BOUNDARY

DIAGNOSTICS

POSITIVE_TESTS
NEGATIVE_TESTS
BOUNDARY_TESTS
SCALABILITY_TESTS
DETERMINISM_TESTS
COMPATIBILITY_TESTS
CROSS_DOMAIN_TESTS

HARD_CODING_AUDIT

COMPLETION_CRITERIA

This contract is deliberately explicit.

A file should be considered complete without requiring another file to be reopened merely because an unrelated feature was added elsewhere.

---

63. Dependency Contract

Each statement grammar should document:

DEPENDS_ON:
EXPORTS:
CONSUMED_BY:
AST_OWNER:
SEMANTIC_OWNER:
TYPE_OWNER:
EFFECT_OWNER:
RESOURCE_OWNER:
POLICY_OWNER:
PROVENANCE_OWNER:
IR_OWNER:
TEST_OWNER:
SPEC_OWNER:

For example:

grammar/statements/quantum.g4

DEPENDS_ON:
    grammar/lexer/
    grammar/core/
    grammar/expressions/
    grammar/quantum/

EXPORTS:
    quantumStatement

AST_OWNER:
    frontend AST

SEMANTIC_OWNER:
    quantum semantic layer

IR_OWNER:
    quantum::ir

TEST_OWNER:
    grammar/tests/quantum/

SPEC_OWNER:
    grammar/spec/quantum.md

The exact paths must follow the repository's actual implementation.

---

64. Integration with "grammar/Zamani.g4"

"grammar/Zamani.g4" remains the top-level composition root.

Its responsibility is to assemble the language.

It must not duplicate the statement definitions contained in:

grammar/statements/

The desired relationship is:

Zamani.g4
    ↓
statements composition
    ↓
statement

not:

Zamani.g4
    ├── statement A
    └── statement B

---

65. Integration with "grammar/antlr/"

The ANTLR composition layer must ensure that:

ZamaniLexer
    ↓
ZamaniParser
    ↓
Statements

uses one coherent token vocabulary and one statement composition path.

The statement grammars must not define lexer rules.

Lexer ownership remains:

grammar/lexer/
grammar/antlr/ZamaniLexer.g4
src/lexer.rs

according to the repository's authority model.

---

66. Integration with "src/lexer.rs"

The lexer is responsible for producing the tokens consumed by the statement grammar.

Any new statement keyword must first be justified against:

grammar/lexer/keywords.md
grammar/lexer/tokens.md
src/lexer.rs

A statement feature MUST NOT introduce an isolated token definition.

If a word can remain an identifier without ambiguity, it should not automatically become a reserved keyword.

This prevents keyword explosion and improves language evolution.

---

67. Integration with "src/parser.rs"

"src/parser.rs" is the executable frontend parser implementation where applicable.

Its statement parsing must remain consistent with the canonical grammar.

There must not be:

ANTLR grammar says A
Rust parser says B

for the same stable feature.

Conformance work must compare:

grammar
lexer
parser
AST

and identify discrepancies rather than silently accepting divergence.

---

68. Integration with "src/ast/mod.rs"

Statement parser contexts must map into the domain-neutral AST.

The AST should preserve semantic intent.

It should not prematurely encode:

CPU
GPU
QPU
FPGA
physical qubit
vendor device
routing decision
QEC decision
calibration

Those belong downstream.

---

69. Integration with Semantic Analysis

After AST construction:

statement
    ↓
AST
    ↓
name resolution
    ↓
type analysis
    ↓
effect analysis
    ↓
capability analysis
    ↓
resource analysis
    ↓
contract analysis
    ↓
policy analysis
    ↓
provenance
    ↓
semantic model

The statement grammar must preserve enough structure for every required analysis stage.

---

70. Integration with Classical IR

Classical statements may lower into the repository's canonical classical representation.

The statement grammar must not depend on:

- LLVM;
- a particular ISA;
- a particular CPU;
- a particular operating system.

The statement represents source intent.

---

71. Integration with "quantum::ir"

Quantum statements must eventually lower into the canonical:

quantum::ir

They must not introduce:

statement-level quantum IR

as a competing representation.

The canonical boundary remains:

AST
    ↓
quantum semantic model
    ↓
quantum::ir

---

72. Integration with HDL

HDL statements eventually participate in:

hardware intent
    ↓
validation
    ↓
simulation
    ↓
verification
    ↓
synthesis
    ↓
placement
    ↓
routing
    ↓
physical realization

The statement grammar must stop at source syntax.

---

73. Integration with Execution

Statement syntax must remain separate from execution.

Execution owns:

scheduling
placement
resource allocation
retry
recovery
adaptation
simulation
deployment

The statement grammar only represents the source-level intent that those systems consume.

---

74. Integration with Resilience

A statement may participate in execution strategies involving:

retry
recover
fallback
degrade
escalate
reject

but the grammar does not implement the resilience state machine.

The downstream execution/resilience system remains authoritative.

---

75. Integration with Distributed Computing

Distributed statements must integrate with:

grammar/distributed/
grammar/networking/
grammar/concurrency/
grammar/resources/
grammar/security/

The source program must not require a fixed number of nodes.

A topology requirement is semantic information.

It is not a universal physical topology.

---

76. Integration with Networking

Network-related statement syntax must preserve:

effect(network)
capability(network.*)
resource requirements
security policy

The grammar must not embed a specific network interface, address, machine, or fixed network size into the universal statement language.

---

77. Integration with Security

Security-sensitive statements must integrate with:

grammar/security/
grammar/policies/
grammar/resources/
grammar/effects/

Security policy must not be bypassed by:

- macros;
- metaprogramming;
- FFI;
- reflection;
- adaptation;
- simulation;
- dialects.

---

78. Integration with Data

Data/query statements integrate with:

grammar/data/
grammar/types/
grammar/expressions/
grammar/interoperability/

SQL, JSON, XML, graph, and other external formats remain dialect/interoperability concerns where appropriate.

The universal statement grammar should not become a catalog of every external data language.

---

79. Integration with AI and Model Computation

AI/model statements may compose:

reason
query
learn
adapt
explain
assert
contract
policy

with ordinary Zamani statements.

The language should not require separate statement forms for every application domain.

For example:

computer vision
sentiment analysis
robotics
time-series analysis
graph learning

should normally be libraries, models, capabilities, or domain extensions rather than universal statement keywords.

---

80. Integration with Neural-Symbolic Computation

Neural-symbolic computation should be expressible through composition of:

model operations
+
reasoning
+
knowledge
+
learning
+
data
+
classical computation

rather than a second programming language.

---

81. Integration with Type-System Features

Statements may consume richer type-system facilities from:

grammar/types/

including, where supported:

- generics;
- constraints;
- associated types;
- linear types;
- affine types;
- dependent constraints;
- pattern types;
- result types;
- uncertainty-bearing types.

The statement grammar does not implement those type systems.

---

82. No Application-Specific Keyword Explosion

The statement subsystem MUST NOT become a catalog of application concepts.

The following types of concepts should not become universal statement keywords merely because an application may use them:

payment
administration
legal action
specific business operation
specific computer-vision algorithm
specific robotics algorithm
specific blockchain operation
specific VR feature
specific AR feature
specific sentiment classifier
specific neural architecture

Instead use:

libraries
dialects
capabilities
types
models
policies
services
APIs

This is essential to keeping Zamani a universal language.

---

83. No Compiler-Implementation Leakage

The grammar must not expose implementation details merely because the current compiler happens to use them.

For example, this is inappropriate as universal syntax:

use_llvm
use_mlir
use_backend_7
use_thread_pool_4
use_gpu_0
use_qpu_1

unless explicitly defined as a target-specific dialect.

The universal language describes intent.

---

84. No Fixed Capacity Constants

The statement subsystem MUST NOT introduce:

MAX_QUBITS
MAX_CPUS
MAX_GPUS
MAX_FPGAS
MAX_NODES
MAX_MEMORY
MAX_THREADS
MAX_TENSOR_RANK
MAX_REGISTER_WIDTH
MAX_NETWORK_SIZE
MAX_DEVICE_COUNT

or equivalent aliases.

The same prohibition applies to hidden equivalents such as:

1024 workers
256 nested blocks
64 devices
32-bit universal register
8 GPUs
16 cores

when those values represent implementation limitations rather than program semantics.

---

85. Numeric Values Are Not Automatically Limits

A source program may legitimately contain:

1024
4096
1000000

or any other valid numeric value.

The hard-coding prohibition is about artificial grammar-level capacity restrictions, not about forbidding ordinary program data.

Therefore:

value = 1024;

is unrelated to:

maximum_workers = 1024;

as a language restriction.

---

86. Error Boundary

Parser errors include:

malformed syntax
missing delimiter
invalid statement structure
unexpected token
invalid grammar composition

Semantic errors include:

type mismatch
invalid ownership
invalid effect
missing capability
insufficient resources
contract violation
policy violation
invalid quantum operation
invalid hardware intent

Runtime errors include:

execution failure
device failure
communication failure
resource exhaustion
recovery failure

These categories must not be conflated.

---

87. Testing Architecture

Statement tests must exist at multiple levels.

grammar/tests/
├── lexical/
├── parser/
├── ast/
├── semantic/
├── statements/
├── contracts/
├── resources/
├── effects/
├── policies/
├── provenance/
├── concurrency/
├── quantum/
├── hybrid/
├── hdl/
├── ai/
├── distributed/
├── interoperability/
├── scalability/
├── portability/
├── compatibility/
├── negative/
└── boundary/

The exact repository test layout remains authoritative.

---

88. Required Positive Tests

At minimum, statement integration tests must cover:

- declarations;
- bindings;
- assignments;
- expressions as statements;
- blocks;
- conditionals;
- loops;
- match;
- break;
- continue;
- return;
- exceptions;
- assertions;
- concurrency;
- resource requirements;
- effects;
- reasoning;
- knowledge operations;
- learning;
- adaptation;
- policies;
- contracts;
- explanations;
- simulation;
- sandbox intent;
- quantum statements;
- HDL statements;
- hybrid statements;
- domain statements.

---

89. Required Negative Tests

Negative tests must verify rejection of:

- malformed statements;
- incomplete blocks;
- invalid statement terminators;
- invalid control-flow structure;
- malformed contracts;
- malformed resource requirements;
- malformed concurrency;
- malformed quantum statement syntax;
- malformed HDL statement syntax;
- duplicate or ambiguous grammar paths;
- unsupported syntax.

Semantic invalidity must remain distinguishable from syntax invalidity.

---

90. Boundary Tests

Boundary tests must cover:

statement alone
statement in block
statement after declaration
statement before declaration
nested statements
deeply nested valid structures
large statement sequences
large blocks
mixed-domain programs
mixed classical/quantum programs
mixed AI/quantum programs
mixed HDL/software programs
distributed/concurrent programs
macro-generated statements
dialect statements

---

91. Scalability Tests

Scalability tests must increase source complexity without asserting an arbitrary maximum.

Test dimensions include:

number of statements
block size
nesting depth
number of bindings
number of branches
number of loops
number of concurrent regions
number of actors
number of channels
number of domain operations
number of quantum operations
number of HDL declarations

The test objective is:

scale until available implementation resources are exhausted

rather than:

must stop at N

---

92. Determinism Tests

Given equivalent:

source
grammar version
lexer configuration
dialect configuration

parsing must be deterministic.

The parser must not depend on:

hardware
network
clock
randomness
filesystem
runtime state
device discovery

---

93. Cross-Domain Tests

At least one integrated test must combine:

classical computation
+
reasoning
+
learning
+
adaptation
+
contract
+
policy
+
resource requirement
+
capability
+
effect
+
concurrency
+
quantum operation
+
measurement
+
hybrid control
+
HDL/hardware intent
+
provenance

The objective is to demonstrate that these constructs share one statement architecture rather than creating independent languages.

---

94. POCO-REAF Integration Test

A mandatory integration test should demonstrate:

one source program
       ↓
same source representation
       ↓
semantic analysis
       ↓
resource/capability negotiation
       ↓
different target realization

without rewriting the source statement grammar.

The test should validate that the language separates:

what the program means

from:

where/how it executes

---

95. Quantum POCO-REAF Test

A quantum source program should be expressible without encoding a fixed physical machine size.

The program may require:

capability("quantum.measurement")

or:

qubits >= required_qubits

but the grammar must not define a maximum number of qubits.

The same source may target:

small simulator
larger simulator
small QPU
larger QPU
distributed quantum system
future QPU

subject to feasibility.

---

96. HDL POCO-REAF Test

An HDL source program should preserve hardware intent while allowing parameterization.

The statement grammar must not assume:

fixed register width
fixed bus width
fixed memory size
fixed FPGA size
fixed number of processing elements

unless those values are explicit program semantics rather than universal grammar limitations.

---

97. Resource Failure Semantics

If a target cannot satisfy:

requires capability(...)
requires memory >= ...
requires qubits >= ...
requires topology(...)

the compiler/runtime must report the appropriate feasibility failure.

It must not silently change the program's meaning.

Possible downstream outcomes may include:

ACCEPT
DEGRADED_ACCEPT
RETRY
RECOVER
ESCALATE
REJECT

according to the repository's execution/resilience model.

---

98. Statement Grammar and Resilience

The statement grammar does not own resilience.

It only permits source constructs that can participate in resilience.

The execution layer remains responsible for:

detect
evaluate
retry
recover
fallback
degrade
quarantine
escalate
reject

---

99. Statement Grammar and Scheduling

Statements describe dependencies and intent.

Scheduling belongs downstream.

A source statement MUST NOT mean:

execute on physical processor X at time Y

unless a target-specific dialect explicitly defines that semantics.

Universal statements should instead expose the information required by scheduling:

dependencies
effects
resources
capabilities
constraints
preferences
ordering requirements

---

100. Statement Grammar and Optimization

The grammar must preserve semantics while allowing optimization.

Examples include:

dead-code elimination
loop transformation
parallelization
fusion
vectorization
quantum optimization
hardware specialization
data movement optimization

The grammar does not implement these transformations.

---

101. Statement Grammar and Provenance of Optimization

Where optimization changes program structure, provenance may record:

original statement
optimization
reason
derived statement/IR
verification

This is particularly important for:

- safety;
- scientific computing;
- AI decisions;
- quantum compilation;
- hardware compilation;
- reproducible builds.

---

102. Safe Rust Requirement Across the Repository

The statement grammar must be compatible with a safe Rust implementation.

The implementation must not introduce:

unsafe

as a requirement.

The parser/AST/semantic pipeline should use safe Rust abstractions.

The grammar itself must not embed target-language actions requiring unsafe operations.

---

103. Repository Compatibility

This README is intentionally compatible with the repository's existing:

grammar/
antlr/
lexer/
core/
types/
expressions/
declarations/
functions/
modules/
effects/
resources/
validation/
policies/
execution/
concurrency/
classical/
quantum/
hybrid/
hdl/
hardware/
distributed/
networking/
ai/
data/
interoperability/
metaprogramming/
macros/
compatibility/
tests/
spec/
specification/

No parallel universal architecture should be introduced under "grammar/statements/".

---

104. Required Integration Changes Around This README

This README establishes the contract; it does not silently modify other files.

The following repository invariants must be reconciled against it:

104.1 "statements.g4"

Must remain the sole owner of:

statement

104.2 "control-flow.g4"

Must be the composition boundary for control-flow leaves.

104.3 "concurrency.g4"

Must connect to the canonical concurrency subsystem rather than becoming a second concurrency implementation.

104.4 "reason.g4"

Must become the canonical statement-level reasoning composition boundary.

104.5 "infer.g4" and "deduce.g4"

Must not compete with the reasoning composition root.

104.6 "match.g4" and "pattern-matching.g4"

Must have one canonical ownership path.

104.7 "quantum.g4"

Must delegate into "grammar/quantum/" and ultimately "quantum::ir".

104.8 "hdl.g4"

Must delegate into the HDL/hardware semantic pipeline.

104.9 "unsafe.g4"

Must not make unsafe Rust necessary.

If it represents a source-language feature that is not part of the stable universal language contract, it must remain isolated behind the appropriate experimental/legacy/dialect boundary until formally specified.

104.10 "Zamani.g4"

Must compose the statement subsystem rather than duplicate its rules.

---

105. What This README Does Not Do

This file does not:

- define parser rules;
- define lexer tokens;
- define AST structs;
- define semantic algorithms;
- define quantum operations;
- define hardware topology;
- define QEC;
- define scheduling;
- define routing;
- define runtime execution;
- define compiler backends;
- define vendor APIs;
- define resource discovery;
- define physical machine limits.

It defines the contract and integration architecture of the statement layer.

---

106. File Completion Criteria

"grammar/statements/README.md" is complete when:

- [ ] statement ownership is unambiguous;
- [ ] "statements.g4" is identified as the canonical statement composition root;
- [ ] no competing "statement.g4" is required;
- [ ] every existing statement family has an ownership boundary;
- [ ] duplicate statement ownership is explicitly identified;
- [ ] control-flow composition is defined;
- [ ] concurrency integration is defined;
- [ ] resource integration is defined;
- [ ] effect integration is defined;
- [ ] contract integration is defined;
- [ ] policy integration is defined;
- [ ] reasoning integration is defined;
- [ ] knowledge integration is defined;
- [ ] learning integration is defined;
- [ ] adaptation integration is defined;
- [ ] provenance integration is defined;
- [ ] explanation integration is defined;
- [ ] simulation integration is defined;
- [ ] sandbox integration is defined;
- [ ] quantum integration is defined;
- [ ] HDL integration is defined;
- [ ] hybrid integration is defined;
- [ ] AI/model integration is defined;
- [ ] distributed integration is defined;
- [ ] interoperability integration is defined;
- [ ] AST boundaries are defined;
- [ ] semantic boundaries are defined;
- [ ] IR boundaries are defined;
- [ ] "quantum::ir" remains canonical;
- [ ] no physical hardware is required by universal syntax;
- [ ] no artificial resource ceilings are introduced;
- [ ] no fixed machine topology is introduced;
- [ ] no application-specific keyword explosion is introduced;
- [ ] safe Rust is maintained;
- [ ] Rust 1.97 / 1.97.1 compatibility is maintained;
- [ ] positive tests are defined;
- [ ] negative tests are defined;
- [ ] boundary tests are defined;
- [ ] scalability tests are defined;
- [ ] determinism tests are defined;
- [ ] compatibility tests are defined;
- [ ] cross-domain tests are defined;
- [ ] POCO-REAF tests are defined.

---

107. Production-Readiness Criteria for the Entire Statements Subsystem

The directory itself is production-ready only when every statement feature can be traced through:

SPECIFICATION
    ↓
LEXER
    ↓
GRAMMAR
    ↓
AST
    ↓
STRUCTURAL VALIDATION
    ↓
TYPE ANALYSIS
    ↓
EFFECT ANALYSIS
    ↓
CAPABILITY ANALYSIS
    ↓
RESOURCE ANALYSIS
    ↓
CONTRACT ANALYSIS
    ↓
POLICY ANALYSIS
    ↓
PROVENANCE
    ↓
SEMANTIC MODEL
    ↓
CANONICAL IR
    ↓
DOMAIN IR
    ↓
OPTIMIZATION
    ↓
LOWERING
    ↓
ROUTING
    ↓
SCHEDULING
    ↓
RESILIENCE
    ↓
ZQN
    ↓
HAL
    ↓
TARGET

A grammar rule being accepted by ANTLR is therefore not sufficient for production readiness.

---

108. Final Architecture

The completed statement architecture is:

                         Zamani source
                              │
                              ▼
                           Lexer
                              │
                              ▼
                      ZamaniParser
                              │
                              ▼
                    statements.g4
                              │
                 ┌────────────┼─────────────┐
                 │            │             │
                 ▼            ▼             ▼
            Core syntax   Control flow   Domain syntax
                 │            │             │
                 ├────────────┼─────────────┤
                 │            │             │
                 ▼            ▼             ▼
            Concurrency    Contracts     Quantum
            Resources      Policies      HDL
            Effects        Reasoning     Hybrid
            Assertions     Learning      AI/model
            Knowledge      Adaptation    Distributed
                 │            │             │
                 └────────────┼─────────────┘
                              │
                              ▼
                       Domain-neutral AST
                              │
                              ▼
                       Semantic analysis
                              │
       ┌──────────────────────┼──────────────────────┐
       │                      │                      │
       ▼                      ▼                      ▼
   Classical              quantum::ir             HDL
       │                      │                      │
       └──────────────────────┼──────────────────────┘
                              │
                              ▼
                         Optimization
                              │
                              ▼
                           Lowering
                              │
                              ▼
                    Routing / Scheduling
                              │
                              ▼
                       Resilience / QEC
                              │
                              ▼
                          ZQN / HAL
                              │
                              ▼
                     Target realization

The fundamental invariant is:

SOURCE SEMANTICS
       ≠
TARGET HARDWARE

Zamani statements describe what the program means and what it requires.

The rest of the toolchain determines how that meaning can be realized on the resources that are actually available.

That separation is what allows the statement layer to support the full range from extremely small systems through heterogeneous, distributed, quantum, hardware, and future computational systems without embedding an artificial machine ceiling into the language.

---

109. Final Statement-Layer Invariants

The following are mandatory:

1. "grammar/statements/statements.g4" owns the universal "statement" rule.
2. No second universal statement rule may exist.
3. Specialized grammars own specialized statement syntax.
4. Composition grammars own composition.
5. Expressions remain owned by "grammar/expressions/".
6. Types remain owned by "grammar/types/".
7. Resources remain owned by "grammar/resources/".
8. Effects remain owned by "grammar/effects/".
9. Policies remain owned by "grammar/policies/".
10. Contracts remain owned by "grammar/validation/".
11. Provenance remains a cross-cutting semantic concern.
12. Concurrency reuses the canonical concurrency subsystem.
13. AI agents reuse concurrency primitives.
14. Reasoning uses one canonical semantic model.
15. Knowledge operations reuse the data/knowledge model.
16. Learning remains algorithm-independent.
17. Adaptation is policy- and capability-controlled.
18. Explanation is semantic intent, not a runtime implementation.
19. Simulation is an execution strategy.
20. Sandbox is a policy/security boundary.
21. Quantum statements ultimately lower through "quantum::ir".
22. Quantum operation names are extensible.
23. HDL statements describe hardware intent.
24. Physical hardware selection is downstream.
25. FFI participates in effects/capabilities.
26. Macros cannot bypass validation.
27. Dialects cannot bypass the canonical AST/semantic pipeline.
28. Parser syntax must not perform execution.
29. Parser syntax must not inspect hardware.
30. Parser syntax must not allocate resources.
31. No universal machine-capacity constant is permitted.
32. No fixed topology is permitted.
33. No artificial resource ceiling is permitted.
34. The AST remains domain-neutral.
35. Semantic analysis determines legality.
36. Resource analysis determines feasibility.
37. Target lowering determines realization.
38. Safe Rust is mandatory.
39. Rust 1.97 / 1.97.1 remains supported.
40. POCO-REAF remains the governing portability objective.

---

110. Definition of Done

The statements subsystem is considered production-ready only when:

Every statement
      ↓
has one owner
      ↓
has one canonical parse path
      ↓
maps to the domain-neutral AST
      ↓
has defined semantic ownership
      ↓
has defined type/effect/resource/capability behavior
      ↓
has defined contract/policy behavior where applicable
      ↓
has provenance behavior where applicable
      ↓
has a canonical IR destination
      ↓
has diagnostics
      ↓
has positive tests
      ↓
has negative tests
      ↓
has boundary tests
      ↓
has scalability tests
      ↓
has determinism tests
      ↓
has compatibility tests
      ↓
has cross-domain tests
      ↓
has no hard-coded machine ceiling
      ↓
has no unsafe Rust dependency
      ↓
preserves POCO-REAF

The resulting architecture is therefore not a collection of disconnected statement grammars. It is one universal statement subsystem with strict ownership, composable syntax, domain-neutral semantics, resource/capability negotiation, and target-independent execution intent.
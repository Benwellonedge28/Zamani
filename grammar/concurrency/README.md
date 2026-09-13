Zamani Concurrency Grammar

1. Purpose

"grammar/concurrency/" defines the syntax-level concurrency domain of the Zamani programming language.

Its purpose is to allow Zamani programs to express portable computational concurrency and parallelism while remaining independent of:

- CPU architecture;
- CPU core count;
- thread count;
- worker count;
- GPU count;
- accelerator count;
- FPGA resources;
- QPU resources;
- cluster size;
- node count;
- machine topology;
- network topology;
- memory capacity;
- operating system;
- runtime implementation;
- scheduler implementation;
- deployment environment.

The concurrency grammar describes computational intent.

It does not describe the physical machine used to realize that intent.

This is fundamental to Zamani's:

«Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)»

model.

---

2. Architectural principle

Zamani concurrency follows:

source syntax
    ↓
lexer
    ↓
parser
    ↓
frontend AST
    ↓
semantic analysis
    ↓
effect analysis
    ↓
resource/capability analysis
    ↓
canonical program representation / IR
    ↓
optimization
    ↓
routing / placement
    ↓
scheduling
    ↓
hardware/backend lowering
    ↓
runtime

Concurrency grammar must stop at the parser/AST boundary.

It must never become a runtime scheduler.

---

3. Core rule

The language describes:

WHAT computation means

rather than:

HOW MANY physical resources happen to execute it

For example, a concurrent computation may eventually execute using:

1 execution resource
4 CPU cores
many CPU cores
GPU execution
FPGA execution
distributed execution
quantum/classical execution
heterogeneous execution
future execution resources

without requiring a different source program merely because the available resources changed.

---

4. POCO-REAF requirements

The concurrency grammar MUST NOT encode a fixed maximum for:

tasks
parallel operations
threads
workers
cores
devices
nodes
actors
channels
queues
execution contexts
parallel regions
concurrent regions
distributed participants

There must be no constructs such as:

MAX_THREADS
MAX_TASKS
MAX_WORKERS
MAX_CORES
MAX_NODES
MAX_DEVICES
MAX_ACTORS
MAX_CHANNELS

inside the grammar.

Likewise, the grammar must not contain assumptions such as:

one task = one thread
one task = one CPU core
one actor = one OS process
one worker = one core
one parallel operation = one device

Those are implementation decisions.

---

5. Meaning of "infinity"

"Infinity" means that the language imposes no artificial finite machine-scale ceiling.

Actual execution remains bounded by resources available to:

- the lexer/parser;
- compiler;
- optimizer;
- scheduler;
- runtime;
- deployment environment;
- target hardware.

For example:

1 task

and

1,000,000 tasks

may both be valid language programs.

Whether the compiler/runtime can execute them is a resource and implementation question, not a grammar restriction.

---

6. Directory ownership

The directory is organized around independent concurrency domains.

grammar/concurrency/
├── README.md
├── concurrency.g4
├── tasks.g4
├── futures.g4
├── actors.g4
├── channels.g4
├── synchronization.g4
├── parallel.g4
├── data-parallel.g4
├── task-parallel.g4
└── cancellation.g4

The files have distinct ownership.

---

7. "concurrency.g4"

Owns

"concurrency.g4" owns the composition boundary for the concurrency domain.

It establishes the stable parser-facing concurrency entry points.

It may classify concurrency constructs into their respective domains.

It may expose integration rules used by the canonical parser.

Does not own

It must not reimplement:

- task syntax;
- future syntax;
- actor syntax;
- channel syntax;
- synchronization syntax;
- parallel syntax;
- data-parallel syntax;
- task-parallel syntax;
- cancellation syntax.

Those belong to their dedicated files.

Completion condition

"concurrency.g4" is complete when the canonical parser can identify concurrency constructs without duplicating the grammar owned by the individual domain files.

---

8. "tasks.g4"

Owns

Task-oriented concurrency syntax:

- spawning;
- awaiting;
- task-oriented concurrent computation;
- task bodies;
- task composition.

The existing task grammar explicitly defines itself as the parser-level owner of task spawning, awaiting and task-oriented parallel scopes.

Does not own

It must not own:

- task scheduling;
- worker allocation;
- executor implementation;
- thread pools;
- hardware topology;
- runtime queues;
- resource discovery;
- routing;
- optimization;
- QEC;
- ZQN;
- quantum IR.

Those remain downstream responsibilities.

---

9. "futures.g4"

Owns

Syntax for asynchronous result/future abstractions.

The grammar may express:

future
awaitable
deferred computation
asynchronous result

where those concepts have been formally established by the language specification.

Does not own

It does not define:

- a particular Future implementation;
- a particular executor;
- thread pools;
- event loops;
- OS primitives;
- network transport.

A future may ultimately be implemented locally, remotely, distributedly, or on another computational substrate.

---

10. "actors.g4"

Owns

Actor-model syntax once the actor syntax has a canonical lexical and language-specification contract.

Possible semantic concepts include:

actor declaration
actor instance
actor message interaction
actor lifecycle

Does not own

It must not define:

- operating-system processes;
- threads;
- process IDs;
- machine placement;
- network addresses;
- fixed actor counts.

The repository already has a compiler language-specification component specifically concerned with actor concurrency.

That specification must be reconciled with this grammar rather than creating a second language definition.

---

11. "channels.g4"

Owns

Syntax for channel/message-passing constructs.

The channel abstraction must remain semantic rather than assuming:

queue implementation
buffer size
network transport
thread implementation

Does not own

It does not determine:

- queue implementation;
- channel capacity unless capacity is genuinely semantic;
- network topology;
- transport protocol;
- scheduler;
- OS synchronization primitive.

---

12. "synchronization.g4"

Owns

Syntax for explicit synchronization concepts that are formally part of Zamani.

Examples may include:

synchronization boundaries
coordination
join/wait semantics
barrier-like semantic operations

Only constructs with an established language-level contract belong here.

Does not own

It does not implement:

- mutexes;
- futexes;
- spinlocks;
- OS locks;
- hardware atomics;
- scheduling algorithms.

Those belong to lower layers.

The repository already has concurrency-related low-level primitives and standard-library synchronization facilities; the grammar must not duplicate those implementations.

---

13. "parallel.g4"

Owns

General parallel-computation syntax.

Parallelism represents intent and permitted concurrency.

For example, conceptually:

parallel {
    computation_a()
    computation_b()
}

means that independent work may be exposed to the compiler for concurrent execution.

It does not promise that both operations physically execute simultaneously.

---

14. "data-parallel.g4"

Owns

Data-parallel computation syntax.

The grammar may express that an operation applies independently across a logical data domain.

It must not hard-code:

SIMD width
vector width
GPU warp size
GPU block size
GPU count
accelerator count

The repository already has a dedicated "data-parallel.g4", so "concurrency.g4" must compose it rather than recreate its rules.

---

15. "task-parallel.g4"

Owns

Task-level parallelism.

It describes:

independent tasks
dependencies
task relationships
parallel task intent

It must not select:

worker count
thread count
core count
machine
device
node
topology

Those are derived later.

---

16. "cancellation.g4"

Owns

Syntax for cancellation semantics.

Cancellation must represent the program's semantic request to stop or invalidate eligible computation.

It must not implement:

- thread interruption;
- OS signals;
- process termination;
- device cancellation;
- network cancellation;
- scheduler cancellation algorithms.

Those are runtime/backend concerns.

---

17. Lexical ownership

The concurrency directory does not own lexical definitions.

The canonical lexer owns:

- keywords;
- identifiers;
- literals;
- punctuation;
- operators.

The repository currently identifies concurrency vocabulary such as:

async
await
spawn
parallel

in its grammar ecosystem. The grammar documentation also identifies concurrency around "async", "await", and "spawn".

A concurrency grammar file must never silently create a new keyword merely by writing an identifier into a parser rule.

For example, this is forbidden as an implicit language-extension mechanism:

identifier { ... }

where semantic tooling later guesses that the identifier means:

actor
channel
task
worker
thread

New reserved words require coordinated language evolution.

---

18. Parser ownership

The canonical parser must have exactly one authoritative path for concurrency syntax.

The repository currently contains another concurrency grammar under:

grammar/antlr/Concurrency.g4

which describes itself as a reusable ANTLR concurrency grammar.

This creates a potential duplicate grammar authority.

The production architecture must resolve this to:

canonical lexer
      ↓
canonical parser
      ↓
grammar/concurrency/*

and not:

canonical parser
      ↓
grammar/concurrency/*

plus

canonical parser
      ↓
grammar/antlr/Concurrency.g4

There must ultimately be one authoritative syntax path.

---

19. ANTLR composition rule

The existence of multiple ".g4" files does not by itself make their parser rules visible to one another.

Therefore the final parser architecture must explicitly establish one of the supported ANTLR composition mechanisms.

The repository must choose and document one authoritative mechanism, such as:

parser grammar imports

or:

canonical parser owns integration rules and incorporates domain grammar

or another explicitly supported ANTLR composition architecture.

The decision must be recorded in the grammar authority documentation.

Individual files must never assume an undeclared cross-file rule dependency.

---

20. AST integration

The grammar produces syntax.

It does not create the final concurrency semantic model.

The frontend AST must represent the structure of parsed concurrency operations.

A concurrency AST node should preserve enough information for downstream analysis to determine:

construct kind
source span
operand/body
ordering
nested computation
source attributes

It must not secretly contain:

thread ID
CPU core ID
GPU ID
device ID
worker ID
machine ID

unless such data is genuinely part of an explicitly target-specific language construct.

---

21. Semantic integration

Semantic analysis determines whether parsed concurrency is valid.

Examples:

await non-awaitable-value

may be syntactically valid but semantically invalid.

Likewise:

parallel {
    conflicting_mutation()
}

may require semantic rejection if the memory/effect system determines that the operations cannot legally execute concurrently.

Therefore:

parse success

does not mean:

program semantically valid

---

22. Type-system integration

Concurrency participates in the type system but does not own it.

The type system determines concepts such as:

awaitable<T>
future<T>
task result
channel<T>
actor message types
parallel operation types

where those types are part of the Zamani type model.

The concurrency grammar must not define a duplicate type system.

---

23. Memory integration

Concurrency must integrate with the canonical memory model.

This includes:

- ownership;
- borrowing;
- lifetimes;
- aliasing;
- mutation;
- shared state;
- synchronization requirements.

Concurrency syntax must not create a second ownership model.

---

24. Effect integration

Concurrency constructs may generate effects such as:

async
concurrent
parallel
communication
synchronization
cancellation
distributed execution

The effect system owns the representation and checking of those effects.

The grammar only identifies their syntactic origin.

---

25. Resource integration

Resource analysis determines what is required to execute the concurrency semantics.

Potential resources include:

compute capacity
memory
communication
latency budget
energy
reliability
accelerator capabilities
quantum capabilities
distributed capabilities

A resource requirement is different from a physical resource selection.

For example:

requires parallel_execution

must not mean:

use 16 CPU cores

unless the program explicitly requests a semantic constraint that requires that physical property.

---

26. Capability integration

Capabilities describe what a target can provide.

Concurrency may require capabilities such as:

asynchronous execution
parallel execution
communication
distributed execution
cancellation
synchronization

The grammar must not discover capabilities itself.

Capability discovery belongs to the target/resource/hardware layers.

---

27. Scheduling integration

The grammar does not schedule anything.

The downstream scheduler determines:

ordering
resource allocation
execution timing
dependency satisfaction
placement
parallel realization

A "parallel" construct must therefore be interpreted as semantic intent.

A target with insufficient parallel resources may legally serialize work if doing so preserves program semantics.

A target with abundant resources may exploit more parallelism.

This is central to POCO-REAF.

---

28. Quantum integration

Concurrency may coordinate quantum and classical computation.

For example, a semantic program may conceptually contain:

classical computation
        ↓
quantum computation
        ↓
classical analysis
        ↓
another quantum computation

or multiple independent quantum computations.

The concurrency grammar must not own:

- qubits;
- physical qubits;
- logical qubits;
- quantum gates;
- quantum circuits;
- quantum operations;
- quantum IR;
- QEC;
- ZQN;
- quantum routing;
- quantum scheduling.

The canonical "quantum::ir" remains the quantum semantic boundary.

Thus:

quantum syntax
    ↓
quantum AST/semantic analysis
    ↓
quantum::ir

while concurrency remains orthogonal.

---

29. HDL integration

Concurrency may surround hardware computations, but HDL syntax remains owned by:

grammar/hdl/

The concurrency grammar must not redefine:

- clocks;
- signals;
- wires;
- registers;
- hardware processes;
- hardware modules.

Hardware concurrency has different semantic rules from software concurrency and must be interpreted by the HDL/hardware semantic layers.

---

30. Distributed integration

Concurrency and distribution are separate dimensions.

A computation can be:

concurrent but local

or:

concurrent and distributed

Therefore concurrency must not assume that concurrency implies multiple machines.

Distributed grammar owns:

- nodes;
- services;
- communication;
- replication;
- placement;
- distributed semantics.

Concurrency supplies computational relationships that may later be distributed.

---

31. Runtime integration

The repository already contains a runtime concurrency subsystem responsible for runtime concurrency functionality.

The runtime may implement concurrency through:

coroutines
event loops
tasks
workers
threads
work stealing
queues
accelerator streams
distributed execution
remote execution

The source grammar must remain independent of which implementation is chosen.

---

32. Standard-library integration

The standard library contains concurrency-oriented functionality.

Grammar must not duplicate standard-library APIs.

For example:

grammar

should recognize the language syntax for an operation, while:

stdlib

provides library-level abstractions.

---

33. Compiler integration

Compiler processing should be:

source
 ↓
lexer
 ↓
parser
 ↓
AST
 ↓
name resolution
 ↓
type checking
 ↓
effect checking
 ↓
concurrency analysis
 ↓
resource/capability analysis
 ↓
canonical IR
 ↓
optimization
 ↓
scheduling
 ↓
lowering

The compiler must never need to reparse the original source to determine runtime concurrency behavior.

---

34. Runtime integration

Runtime execution consumes compiled semantic representations.

It must not depend directly on:

grammar/concurrency/*.g4

The dependency must therefore be:

grammar
   ↓
AST
   ↓
semantic model / IR
   ↓
compiler
   ↓
runtime

and never:

runtime
   ↓
grammar

This prevents architectural cycles.

---

35. Resilience integration

Concurrency can fail at runtime because of:

- resource exhaustion;
- executor failure;
- backend failure;
- distributed failure;
- cancellation;
- unavailable execution resources.

The resilience layer may decide to:

retry
restart
resume
rollback
reschedule
reroute
switch backend
quarantine resource
abort

The concurrency grammar does not own those decisions.

---

36. Determinism

Parsing must be deterministic.

Given identical:

source
language version
grammar version
lexer configuration

the parser must produce the same syntax structure.

Runtime scheduling nondeterminism is separate from parser determinism.

The grammar must never depend on:

thread scheduling
hardware discovery
hash-map iteration
device availability
runtime timing

---

37. Error handling

Syntax errors must be reported by the parser/diagnostic layer.

Examples include:

await
spawn
parallel

with missing required operands/bodies.

Semantic errors belong to semantic analysis.

Resource errors belong to resource analysis.

Runtime failures belong to runtime/resilience.

Errors must not be incorrectly collapsed into generic parser failures.

---

38. Compatibility

Existing valid Zamani concurrency constructs must be preserved unless an explicit language-version migration says otherwise.

Compatibility work must distinguish:

syntax compatibility
AST compatibility
semantic compatibility
IR compatibility
runtime compatibility
source compatibility

A syntax change must not silently alter the meaning of an existing concurrency program.

---

39. Versioning

Concurrency syntax is versioned as part of the Zamani language.

A future concurrency feature must have:

language-version decision
keyword decision
grammar rule
AST representation
semantic specification
diagnostics
compiler integration
runtime integration
tests
documentation
compatibility policy

before becoming a stable language feature.

Experimental constructs must not silently become permanent syntax.

---

40. Security

Concurrency syntax must not bypass:

- capability checks;
- permission checks;
- resource policies;
- isolation;
- memory safety;
- execution boundaries.

The grammar itself does not provide security.

Security analysis belongs downstream.

---

41. Safe Rust requirement

The grammar contains no Rust execution code.

Where Zamani's lexer/parser/compiler implementation is written in Rust:

Rust 1.97
or
Rust 1.97.1

must be supported.

The compiler implementation must use safe Rust.

"unsafe" is prohibited unless a separate repository-wide architectural decision explicitly changes this requirement.

Concurrency syntax must never require unsafe Rust.

---

42. Forbidden machine assumptions

The entire directory must remain free of assumptions such as:

thread_count = 8
worker_count = 16
core_count = 32
gpu_count = 4
node_count = 64
queue_size = 1024
channel_capacity = 256
max_tasks = 1000000

A number is not automatically forbidden.

The distinction is:

language syntax number

versus:

machine capacity number

For example, a user-defined numeric value may be valid program data.

A hidden compiler capacity must never be represented as language syntax.

---

43. Semantic requirements versus implementation decisions

This distinction is mandatory.

Semantic

these operations may execute concurrently

Implementation

execute them on four workers

Semantic

this computation must wait for that result

Implementation

block a thread

or:

suspend a coroutine

or:

enqueue an event

are implementation decisions.

The grammar owns the first category.

It must not own the second.

---

44. Cross-domain model

Concurrency must remain composable with every major Zamani domain.

classical
    ↕
concurrency

quantum
    ↕
concurrency

HDL
    ↕
concurrency

hardware
    ↕
concurrency

AI
    ↕
concurrency

data
    ↕
concurrency

distributed
    ↕
concurrency

networking
    ↕
concurrency

resilience
    ↕
concurrency

No domain should need to duplicate the concurrency grammar.

---

45. Example conceptual program

A portable concurrent program might express:

spawn {
    classical_work()
}

spawn {
    quantum_work()
}

await results

The source does not specify whether the implementation uses:

one CPU
many CPUs
CPU + GPU
CPU + QPU
distributed workers
cloud resources
future hardware

That is precisely the intended architecture.

---

46. Scalability model

The grammar must support:

tiny program

through:

very large program

without changing syntax merely because the machine becomes larger.

Likewise:

one concurrent operation

and:

many concurrent operations

must use the same semantic constructs.

The grammar must not introduce a special "large machine" language.

---

47. Parser complexity

The grammar must avoid unnecessary ambiguity.

Particular care must be taken with constructs involving:

expression
block
statement
function call
closure
async
await
spawn
parallel

A concurrency rule must not greedily consume ordinary expressions in a way that prevents other grammar domains from parsing correctly.

Concurrency constructs should have identifiable syntactic boundaries.

---

48. No arbitrary extension syntax

The grammar must not solve extensibility by accepting arbitrary identifiers such as:

foo computation
bar computation
whatever computation

and leaving their meaning to runtime interpretation.

Future extensibility belongs to:

dialects
versioning
capabilities
namespaces
attributes
semantic extension points

with explicit contracts.

---

49. Testing requirements

Every concurrency grammar file requires tests.

Tests must exist under:

grammar/tests/concurrency/

and where appropriate:

grammar/tests/positive/
grammar/tests/negative/
grammar/tests/boundary/
grammar/tests/cross-domain/
grammar/tests/scalability/
grammar/tests/determinism/
grammar/tests/roundtrip/

---

50. Positive tests

At minimum:

spawn expression
spawn block
await expression
parallel block
nested concurrency
concurrency inside functions
concurrency with generic code
concurrency with classical computation
concurrency with quantum computation
concurrency with distributed computation

---

51. Negative tests

Test:

spawn
await
parallel

without required operands.

Also test:

malformed concurrency expressions
malformed nesting
invalid delimiter combinations
ambiguous constructs
unknown pseudo-keywords

and ensure ordinary identifiers cannot accidentally become concurrency keywords.

---

52. Boundary tests

Test very large syntactic structures.

Examples:

deeply nested concurrent regions
large task dependency structures
large parallel regions
large source files
large expressions
large concurrency compositions

No artificial machine-scale maximum may appear.

---

53. Cross-domain tests

Required combinations include:

classical + concurrency
quantum + concurrency
classical + quantum + concurrency
HDL + concurrency
hardware + concurrency
distributed + concurrency
AI + concurrency
data + concurrency
networking + concurrency
quantum + distributed + concurrency
quantum + hardware + concurrency
classical + quantum + HDL + concurrency

---

54. Round-trip tests

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

must preserve concurrency semantics.

Formatting differences are acceptable.

Semantic changes are not.

---

55. Determinism tests

Repeated parsing of the same source must produce equivalent:

token sequence
parse tree
AST
diagnostic ordering

where deterministic ordering is part of the frontend contract.

---

56. Hard-coding audit

Every change to this directory must be checked for:

MAX_*
DEFAULT_* resource capacities
fixed thread counts
fixed worker counts
fixed core counts
fixed device counts
fixed node counts
fixed topology
fixed queue capacity
fixed channel capacity
fixed accelerator capacity
fixed quantum capacity

Each finding must be classified as:

1. language semantic requirement;
2. target-specific requirement;
3. resource constraint;
4. implementation limitation;
5. accidental hard-coding;
6. test-only limitation;
7. documentation-only limitation.

Accidental hard-coding must be removed.

---

57. File completion contract

A concurrency grammar file is not complete merely because ANTLR accepts it.

It is complete only when all of the following are established:

Purpose
Ownership
Non-ownership
Lexer contract
Parser contract
AST contract
Semantic contract
Type contract
Effect contract
Resource contract
Capability contract
Compiler contract
IR contract
Runtime contract
Cross-domain contract
Compatibility contract
Error contract
Scalability contract
Security contract
Testing contract
Documentation contract
Hard-coding audit

have been satisfied.

---

58. Dependency order

Concurrency implementation must proceed in dependency order.

Recommended order:

1. language specification
        ↓
2. lexical vocabulary
        ↓
3. core names/types/expressions/statements
        ↓
4. tasks.g4
        ↓
5. futures.g4
        ↓
6. parallel.g4
        ↓
7. data-parallel.g4
        ↓
8. task-parallel.g4
        ↓
9. actors.g4
        ↓
10. channels.g4
        ↓
11. synchronization.g4
        ↓
12. cancellation.g4
        ↓
13. concurrency.g4 composition root
        ↓
14. canonical parser integration
        ↓
15. AST integration
        ↓
16. semantic/effect/type integration
        ↓
17. resource/capability integration
        ↓
18. compiler/IR integration
        ↓
19. scheduling integration
        ↓
20. runtime integration
        ↓
21. cross-domain integration tests

The exact ordering must be adjusted if repository inspection identifies a different concrete dependency.

---

59. Integration graph

The intended architecture is:

                   Zamani Source
                         |
                         v
                  Canonical Lexer
                         |
                         v
                  Canonical Parser
                         |
              +----------+----------+
              |                     |
              v                     v
        Ordinary Syntax       Concurrency Root
                                    |
              +----------+----------+----------+
              |          |          |          |
              v          v          v          v
            Tasks      Futures   Parallel   Data/Task
                                                Parallel
              |
              +----------+----------+----------+
                         |
                         v
                    Frontend AST
                         |
             +-----------+-----------+
             |           |           |
             v           v           v
          Types       Effects     Capabilities
             |           |           |
             +-----------+-----------+
                         |
                         v
                Semantic Program Model
                         |
                         v
                    Canonical IR
                         |
       +-----------------+------------------+
       |                 |                  |
       v                 v                  v
   Classical          Quantum          Distributed
       |                 |                  |
       +-----------------+------------------+
                         |
                         v
                    Optimization
                         |
                         v
                   Routing/Placement
                         |
                         v
                     Scheduling
                         |
                         v
                    Resilience
                         |
                         v
                       Runtime

---

60. Dependency prohibition

The following dependency directions are forbidden:

runtime → grammar
scheduler → grammar
hardware → grammar
quantum::ir → grammar
ZQN → grammar
QEC → grammar

The correct direction is:

grammar → AST/semantic representation

and then:

semantic representation → IR/compiler/runtime subsystems

This prevents circular architecture.

---

61. Quantum boundary

Concurrency must never create an alternative quantum representation.

The canonical boundary remains:

quantum syntax
      ↓
quantum semantic analysis
      ↓
quantum::ir

Concurrency may refer semantically to quantum computation.

It must not become a second quantum IR.

---

62. Hardware boundary

Concurrency must never become a hardware-discovery mechanism.

The hardware layer determines:

available resources
capabilities
topology
calibration
device state

Concurrency determines:

what computation may proceed concurrently

These are separate concerns.

---

63. Scheduling boundary

Concurrency defines potential concurrency.

Scheduling determines realizable execution.

Therefore:

parallel

does not mean:

run simultaneously regardless of resources

It means the compiler may exploit legal parallelism subject to:

dependencies
effects
memory rules
resource constraints
target capabilities
scheduling policy

---

64. Runtime boundary

Runtime decides how semantic concurrency is implemented.

Possible implementations include:

single-context execution
cooperative scheduling
preemptive scheduling
threads
coroutines
event loops
distributed workers
accelerator queues
heterogeneous execution

The grammar remains unchanged.

This is a core POCO-REAF property.

---

65. Future-proofing

Future concurrency models must be able to integrate without changing existing semantics unnecessarily.

Potential future models include:

dataflow
actor systems
reactive systems
distributed actors
speculative execution
transactional concurrency
hardware task graphs
quantum-classical concurrent execution
neuromorphic execution
photonic execution
future computational substrates

A new model should normally be added as an independently owned grammar domain rather than expanding "concurrency.g4" into an unmaintainable monolith.

---

66. Repository integration requirements

Before merging a concurrency grammar change, verify:

grammar/spec/
grammar/lexer/
grammar/core/
grammar/types/
grammar/expressions/
grammar/statements/
grammar/functions/
grammar/effects/
grammar/memory/
grammar/resources/
grammar/classical/
grammar/quantum/
grammar/hybrid/
grammar/hdl/
grammar/hardware/
grammar/distributed/
grammar/ai/
grammar/networking/
grammar/security/

for conflicts.

Also verify corresponding compiler/runtime consumers.

The repository already contains concurrency-specific compiler and runtime components, so the grammar must be aligned with them rather than inventing an unrelated concurrency model.

---

67. Production-readiness checklist

Architecture

- [ ] One canonical concurrency grammar authority.
- [ ] No duplicate parser authority.
- [ ] Clear ownership of every construct.
- [ ] No circular dependencies.
- [ ] No runtime dependency from grammar.
- [ ] No scheduler dependency from grammar.

Lexical

- [ ] All keywords have canonical lexical ownership.
- [ ] No accidental keyword creation.
- [ ] Identifier handling is deterministic.

Syntax

- [ ] Tasks are defined once.
- [ ] Futures are defined once.
- [ ] Parallelism is defined once.
- [ ] Data parallelism is defined once.
- [ ] Task parallelism is defined once.
- [ ] Actors are defined once.
- [ ] Channels are defined once.
- [ ] Synchronization is defined once.
- [ ] Cancellation is defined once.

Semantics

- [ ] AST contract exists.
- [ ] Type contract exists.
- [ ] Effect contract exists.
- [ ] Capability contract exists.
- [ ] Resource contract exists.
- [ ] Memory contract exists.

POCO-REAF

- [ ] No fixed thread maximum.
- [ ] No fixed task maximum.
- [ ] No fixed worker maximum.
- [ ] No fixed core maximum.
- [ ] No fixed device maximum.
- [ ] No fixed node maximum.
- [ ] No topology assumptions.
- [ ] No hardware-specific source requirement.
- [ ] Same semantic program can target different scales.

Quantum

- [ ] No duplicated quantum IR.
- [ ] No QEC ownership.
- [ ] No ZQN ownership.
- [ ] No physical-qubit assumptions.
- [ ] Quantum concurrency remains composable.

Hardware/HDL

- [ ] No hardware discovery.
- [ ] No device IDs.
- [ ] No fixed hardware topology.
- [ ] No fixed accelerator count.
- [ ] HDL remains independently owned.

Safety

- [ ] Rust 1.97 supported.
- [ ] Rust 1.97.1 supported.
- [ ] No unsafe Rust required.
- [ ] Compiler implementation remains safe Rust.

Testing

- [ ] Positive tests.
- [ ] Negative tests.
- [ ] Boundary tests.
- [ ] Cross-domain tests.
- [ ] Scalability tests.
- [ ] Determinism tests.
- [ ] Round-trip tests.
- [ ] Compatibility tests.
- [ ] Hard-coding audit.

---

68. Definition of done

"grammar/concurrency/" is production-ready only when:

Every syntax construct has one owner.
Every owner has a documented contract.
Every cross-file dependency is explicit.
Every parser integration point is defined.
Every AST mapping is defined.
Every semantic interpretation is defined.
Every downstream consumer is identified.
Every forbidden dependency is documented.
Every scalability constraint is externalized.
Every machine-specific property is externalized.
Every concurrency feature has tests.
Every compatibility decision is documented.
Every duplicate grammar authority is resolved.

The final architectural invariant is:

Zamani source
     ↓
portable concurrency intent
     ↓
canonical semantics
     ↓
target-independent IR
     ↓
target-aware compilation
     ↓
resource-aware scheduling
     ↓
hardware/runtime realization

Therefore:

«Concurrency syntax describes what computations may happen concurrently; it does not dictate the machine resources used to make that happen.»

This is the concurrency-domain expression of:

«Zamani — From Atom to Everywhere»

and:

«Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF).»
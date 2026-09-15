Zamani Hybrid Computing Specification

Path: "grammar/spec/hybrid.md"
Status: Normative
Domain: Hybrid and heterogeneous computation
Language: Zamani
Language-version authority: "grammar/specification/"
Grammar authority: "grammar/Zamani.g4" plus its canonical composed grammar components
Hybrid grammar root: "grammar/hybrid/hybrid.g4"
Implementation baseline: Rust 1.97 / Rust 1.97.1, Rust 2021
Safety requirement: No "unsafe" Rust
Canonical quantum semantic boundary: "quantum::ir"
Canonical hybrid control-flow semantic boundary: repository quantum/classical IR control-flow facilities
Primary hybrid grammar directory: "grammar/hybrid/"

---

1. Purpose

This document is the normative specification contract for hybrid computation in Zamani.

Hybrid computation is not a separate programming language embedded inside Zamani.

It is the ability of one Zamani program to express computation whose semantic behavior crosses, combines, coordinates, or composes multiple computational domains.

At minimum, this includes:

- classical computation;
- quantum computation;
- classical control of quantum computation;
- quantum results consumed by classical computation;
- accelerator computation;
- heterogeneous computation;
- hardware/software co-design;
- distributed computation involving heterogeneous resources;
- AI/ML computation interacting with classical or quantum computation;
- data processing across computational domains;
- future computational domains.

The hybrid system MUST allow a programmer to describe what the computation means without requiring the programmer to encode the physical machine that will eventually execute it.

The central portability goal is:

«Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)»

subject to:

- program semantics;
- language-version compatibility;
- required capabilities;
- available resources;
- target support;
- explicitly declared constraints;
- execution-environment guarantees.

"Everywhere" and "anywhere" do not mean that impossible execution is guaranteed. They mean that a conforming source program does not require source rewriting merely because the implementation target, scale, topology, or available compatible resources changes.

---

2. Normative Language

The following terms are normative:

- MUST
- MUST NOT
- REQUIRED
- SHALL
- SHALL NOT
- SHOULD
- SHOULD NOT
- MAY
- OPTIONAL

If this document conflicts with implementation-specific behavior, the implementation is non-conforming.

If this document conflicts with an implementation-generated reference, the generated reference is non-authoritative.

If this document conflicts with experimental material in "grammar/Zamani-Grammar.md", the experimental material is non-authoritative.

---

3. Authority Hierarchy

Hybrid syntax participates in the repository-wide language authority hierarchy:

grammar/specification/
        │
        ▼
grammar/spec/*.md
        │
        ▼
grammar/Zamani.g4
        │
        ▼
canonical grammar components
        │
        ▼
lexer / parser
        │
        ▼
frontend AST
        │
        ▼
semantic analysis
        │
        ▼
canonical semantic representation / IR
        │
        ├───────────────┐
        ▼               ▼
 classical semantics   quantum::ir
        │               │
        └───────┬───────┘
                ▼
        hybrid dependencies
                │
                ▼
 optimization / lowering
                │
        ┌───────┼────────┐
        ▼       ▼        ▼
     routing scheduling resilience
        │       │        │
        └───────┼────────┘
                ▼
               ZQN
                │
                ▼
               HAL
                │
                ▼
      target realization
                │
                ▼
        compiler / runtime

"grammar/hybrid/" is therefore upstream of semantic interpretation.

It MUST NOT become:

- a second AST;
- a second semantic IR;
- a second quantum IR;
- a runtime;
- a scheduler;
- a router;
- a QEC implementation;
- a noise model;
- a hardware inventory;
- a device-selection implementation.

---

4. Hybrid Computing Definition

A hybrid program is a program whose semantic graph contains computation from more than one computational domain or whose execution requires interaction between different computational domains.

Examples include:

classical → quantum
quantum → classical
classical → accelerator
accelerator → classical
quantum → accelerator
accelerator → quantum
classical → quantum → classical
classical → accelerator → quantum
classical → quantum → accelerator → classical

The domain graph is not limited to these examples.

Future domains MAY participate without requiring the universal language grammar to be redesigned.

Hybrid computation includes both:

4.1 Spatially hybrid computation

Different portions of a program execute on different computational resources.

4.2 Temporally hybrid computation

Different domains execute at different points in an execution dependency chain.

For example:

prepare classical data
        ↓
execute quantum operation
        ↓
measure
        ↓
obtain classical result
        ↓
perform classical calculation
        ↓
conditionally execute another quantum operation

4.3 Nested hybrid computation

A domain may contain another domain interaction.

Example:

classical computation
    └── quantum region
          └── classical predicate
                └── quantum operation

4.4 Repeated hybrid computation

A hybrid region MAY execute:

- once;
- repeatedly;
- conditionally;
- iteratively;
- concurrently;
- asynchronously;
- speculatively;
- as part of a distributed computation.

No grammar-level repetition limit is permitted.

---

5. Architectural Ownership

5.1 This specification owns

This specification owns:

- hybrid-domain semantics;
- domain-crossing semantics;
- classical/quantum interaction semantics;
- accelerator interaction semantics;
- cross-domain data semantics;
- cross-domain dependency semantics;
- hybrid control semantics;
- synchronization semantics;
- hybrid resource intent;
- hybrid capability intent;
- hybrid portability requirements;
- hybrid scalability requirements;
- semantic distinction between requirements and implementation decisions;
- hybrid AST integration requirements;
- hybrid IR integration requirements;
- hybrid compiler integration requirements;
- hybrid runtime integration requirements;
- hybrid diagnostics requirements;
- hybrid conformance requirements.

5.2 This specification does not own

This specification does NOT own:

- global lexical syntax;
- global identifier syntax;
- global operator definitions;
- global precedence;
- general expression grammar;
- general type grammar;
- general statement grammar;
- general declaration grammar;
- AST implementation;
- name resolution implementation;
- type inference implementation;
- optimization algorithms;
- routing algorithms;
- scheduling algorithms;
- physical qubit placement;
- QEC implementation;
- QEC decoding;
- noise modelling;
- ZQN implementation;
- hardware discovery;
- HAL implementation;
- device enumeration;
- physical memory allocation;
- runtime transport implementation;
- vendor SDK implementation;
- accelerator driver implementation;
- network implementation;
- deployment orchestration implementation.

Those belong to their respective repository subsystems.

---

6. Existing Hybrid Grammar Files

The existing hybrid directory is:

grammar/hybrid/
├── README.md
├── hybrid.g4
├── classical-quantum.g4
├── quantum-classical-control.g4
├── accelerator-interoperability.g4
└── hybrid-resources.g4

These files MUST remain the primary hybrid grammar components unless a future architecture decision explicitly supersedes them.

The current repository already defines this directory as a production-oriented hybrid grammar layer, with the stated principle that syntax describes portable computation and intent while semantic interpretation, resource realization, compilation, scheduling, routing, optimization, and execution remain downstream.

The files have the following responsibilities:

File| Responsibility
"hybrid.g4"| Hybrid composition boundary
"classical-quantum.g4"| Classical/quantum domain crossings
"quantum-classical-control.g4"| Classical control over quantum execution
"accelerator-interoperability.g4"| Generic accelerator interaction
"hybrid-resources.g4"| Hybrid resource/capability intent

No file may silently assume responsibility belonging to another file.

---

7. Universal Grammar Integration

Hybrid grammar MUST reuse the universal language foundations.

It MUST integrate with:

grammar/core/
grammar/types/
grammar/expressions/
grammar/statements/
grammar/declarations/
grammar/functions/
grammar/modules/
grammar/effects/
grammar/memory/
grammar/concurrency/
grammar/resources/
grammar/hardware/
grammar/compile/
grammar/execution/
grammar/classical/
grammar/quantum/
grammar/hdl/
grammar/distributed/
grammar/ai/
grammar/data/
grammar/networking/
grammar/security/
grammar/interoperability/

Hybrid grammar MUST NOT duplicate those domains.

For example, hybrid grammar MUST NOT define a second:

expression
type
function
if
while
for
module
identifier
attribute

grammar.

It MAY provide domain-specific wrappers around universal constructs where parser composition requires them.

---

8. Hybrid Grammar Composition

The root:

grammar/hybrid/hybrid.g4

MUST be a composition layer.

Its responsibilities are:

- hybrid construct dispatch;
- hybrid block composition;
- domain-interaction composition;
- cross-domain construct selection;
- resource-intent composition;
- accelerator interaction composition;
- control-flow composition.

It MUST NOT duplicate detailed rules owned by:

classical-quantum.g4
quantum-classical-control.g4
accelerator-interoperability.g4
hybrid-resources.g4

The universal root:

grammar/Zamani.g4

remains the final language composition root.

Conceptually:

Zamani.g4
    │
    ├── universal constructs
    │
    ├── classical
    │
    ├── quantum
    │
    ├── HDL
    │
    ├── AI
    │
    └── hybrid
            │
            ├── classical ↔ quantum
            ├── quantum ↔ classical control
            ├── accelerator interoperability
            └── hybrid resources

There MUST NOT be a second root grammar competing with "Zamani.g4".

---

9. Domain Identity

Hybrid semantics MUST identify the participating domains without making domains equivalent to physical devices.

A domain can be represented conceptually as:

classical
quantum
hdl
accelerator
distributed
ai
data
network
security
future

The list is extensible.

A domain identifier means:

«a semantic computational domain»

not:

«a particular physical device.»

Therefore:

quantum

does not mean:

QPU #0

and:

accelerator

does not mean:

GPU #0

and:

classical

does not mean:

CPU core #0

---

10. Domain Crossing

A domain crossing is a semantic relationship in which data, control, effects, resources, or execution dependencies move between computational domains.

A crossing MAY involve:

- values;
- references;
- parameters;
- measurement results;
- control predicates;
- streams;
- buffers;
- tensors;
- handles;
- capabilities;
- effects;
- execution dependencies;
- resource requirements.

The crossing MUST be represented explicitly in the semantic model whenever it affects observable semantics, scheduling, synchronization, ownership, lifetime, or resource requirements.

---

11. Classical → Quantum

A classical computation MAY provide:

- quantum operation parameters;
- rotation angles;
- amplitudes;
- symbolic parameters;
- loop bounds;
- predicates;
- register selections;
- resource requirements;
- runtime values;
- calibration-independent parameters;
- algorithmic data.

Example conceptual relationship:

classical value
      ↓
quantum operation parameter
      ↓
quantum::ir

The grammar MUST NOT require the parameter to be a compile-time constant unless another language rule explicitly requires it.

The semantic analyzer determines whether a particular quantum operation accepts:

- compile-time values;
- runtime classical values;
- symbolic values;
- measurement-derived values.

---

12. Quantum → Classical

Quantum computation MAY produce classical information through mechanisms including:

- measurement;
- expectation evaluation;
- observable evaluation;
- result extraction;
- completion status;
- error/status metadata where semantically exposed.

Conceptually:

quantum operation
      ↓
measurement/result
      ↓
classical value

The resulting classical value MUST be represented in the semantic model with its:

- type;
- source;
- lifetime;
- dependency;
- availability point;
- source location.

The grammar MUST NOT merely treat a quantum result as an untyped string or comment.

---

13. Measurement-Driven Classical Feedback

A quantum computation MAY produce classical data that determines later computation.

For example:

quantum operation
      ↓
measurement
      ↓
classical predicate
      ↓
branch
      ↓
quantum operation

This is a semantic dependency.

It MUST survive:

parser
→ AST
→ semantic analysis
→ IR
→ optimization
→ scheduling
→ runtime

The repository's canonical control-flow IR already explicitly supports measurement-driven classical feedback and classical predicates in target-independent hybrid control flow.

The grammar MUST therefore preserve the structure required to construct those semantic dependencies.

---

14. Classical Control of Quantum Operations

Classical control MAY include:

- "if";
- "else";
- loops;
- pattern matching;
- guards;
- predicates;
- dynamically produced values;
- measurement results;
- externally supplied classical values where permitted.

The hybrid grammar MUST reuse universal control-flow syntax whenever possible.

It MUST NOT create a second "if", "while", or "match" language merely for quantum operations.

A hybrid-specific wrapper MAY identify that the controlled body contains quantum operations.

Semantic analysis determines legality.

---

15. Quantum Control of Classical Computation

Quantum computation MAY indirectly control classical computation through classicalized results.

Examples include:

measure
expect
observe
result
status

The exact operation semantics are owned by the quantum specification and canonical quantum semantic layer.

Hybrid grammar owns the relationship between the resulting value and the classical consumer.

---

16. Quantum Operations

Hybrid grammar MUST NOT enumerate quantum gates.

It MUST NOT contain a universal architecture such as:

H | X | Y | Z | CNOT | SWAP | ...

as the only representation of quantum operations.

Quantum operation semantics belong to:

grammar/spec/quantum.md
grammar/quantum/
quantum::ir

Hybrid grammar should represent the relationship:

classical computation
        ↓
quantum operation reference

without duplicating the quantum operation taxonomy.

This allows:

- standard gates;
- parameterized operations;
- custom operations;
- logical operations;
- future operations;
- pulse-level operations;
- analog operations;
- vendor-independent operations;
- dialect-defined operations;

to participate in hybrid programs without modifying hybrid control grammar.

---

17. Canonical Quantum Boundary

All quantum portions of hybrid computation MUST ultimately integrate with:

quantum::ir

"grammar/hybrid/" MUST NOT define:

HybridQuantumIR
HybridCircuitIR
HybridGateIR
HybridQubitIR

as replacement semantic models.

The required direction is:

hybrid source
     ↓
frontend AST
     ↓
semantic analysis
     ↓
quantum semantics
     ↓
quantum::ir

Cross-domain relationships must be represented in the canonical semantic/control-flow structures rather than by creating a second quantum representation.

---

18. Canonical Classical Boundary

Classical portions MUST integrate with the repository's canonical classical representation.

Hybrid grammar MUST NOT create a second classical IR merely because a classical operation occurs near a quantum operation.

The semantic representation MUST retain:

- value dependencies;
- control dependencies;
- effects;
- resource requirements;
- ordering requirements;
- synchronization requirements.

---

19. Cross-Domain Dependency Graph

Hybrid semantics MUST be representable as a dependency graph.

Conceptually:

Node:
    semantic computation

Edge:
    data dependency
    control dependency
    ordering dependency
    synchronization dependency
    resource dependency
    ownership/lifetime dependency

Example:

classical_prepare
       │
       │ data
       ▼
quantum_parameterized_operation
       │
       │ measurement dependency
       ▼
measurement
       │
       │ classical result
       ▼
classical_predicate
       │
       │ control dependency
       ▼
quantum_operation

The graph MUST NOT be tied to:

- CPU IDs;
- GPU IDs;
- QPU IDs;
- thread IDs;
- physical qubit IDs;
- memory addresses;
- network addresses.

Those are downstream realization details.

---

20. Synchronization

Hybrid programs may require synchronization between domains.

Synchronization semantics MUST distinguish at least:

- data availability;
- control availability;
- completion;
- ordering;
- visibility;
- ownership transfer;
- resource release.

The grammar MAY express explicit synchronization intent where required by the language.

However, grammar MUST NOT prescribe a particular runtime synchronization mechanism.

The compiler/runtime MAY implement synchronization using:

- barriers;
- events;
- futures;
- queues;
- channels;
- command buffers;
- runtime tokens;
- device synchronization;
- distributed synchronization;
- hardware mechanisms.

The implementation must preserve the specified semantics.

---

21. Asynchronous Hybrid Computation

Hybrid operations MAY be asynchronous.

Examples include:

submit quantum work
await result
continue classical computation

or:

submit accelerator work
perform classical computation
await accelerator result

The grammar MUST reuse the universal concurrency and asynchronous computation model.

Hybrid grammar MUST NOT create a separate asynchronous language.

Semantic analysis determines:

- whether an operation is asynchronous;
- what value represents completion;
- what dependencies exist;
- what effects are observable.

---

22. Futures and Deferred Results

A cross-domain result MAY be represented semantically as a deferred value.

Conceptually:

Future<T>

The exact type syntax belongs to the universal type system.

Hybrid semantics define how a deferred result participates in:

- data dependencies;
- synchronization;
- ownership;
- effects;
- control flow.

A future MUST NOT be interpreted as a physical device queue merely because a particular backend implements it that way.

---

23. Accelerator Interoperability

The existing:

grammar/hybrid/accelerator-interoperability.g4

owns portable accelerator interaction syntax.

Accelerators include, but are not limited to:

- GPU;
- FPGA;
- DSP;
- tensor accelerator;
- AI accelerator;
- ASIC-backed accelerator;
- reconfigurable accelerator;
- quantum accelerator;
- future accelerator classes.

The grammar MUST remain accelerator-class generic.

---

24. Accelerator Portability

A program SHOULD be able to express:

requires accelerator capability

without expressing:

use GPU 0

The distinction is:

semantic requirement
        ≠
resource selection

and:

capability requirement
        ≠
device identity

The compiler/runtime may choose an implementation satisfying the requirement.

---

25. Accelerator Fallback

Where the semantic operation has a valid portable fallback, the compiler MAY lower:

accelerator operation

to:

CPU implementation

or another compatible implementation.

However, if the program explicitly requires a capability that cannot be emulated without changing observable semantics, the compiler MUST diagnose the unavailable requirement.

Grammar is not responsible for deciding this.

---

26. Heterogeneous Execution

Hybrid programs MAY combine:

CPU
GPU
FPGA
ASIC
DSP
QPU
distributed nodes
future accelerators

but these names represent capability classes, not mandatory physical inventory.

A program MUST NOT require source rewriting merely because:

- an accelerator is added;
- an accelerator is removed;
- the accelerator count changes;
- memory size changes;
- core count changes;
- vector width changes;
- topology changes.

---

27. Resource Intent

The existing:

grammar/hybrid/hybrid-resources.g4

owns syntax for hybrid resource intent.

Resource expressions MUST distinguish:

1. requirement;
2. constraint;
3. capability;
4. preference;
5. hint;
6. policy;
7. target;
8. placement;
9. implementation decision.

These concepts MUST NOT be conflated.

---

28. Resource Requirement

A resource requirement means:

«execution is semantically dependent upon a resource property.»

Conceptual examples:

requires capability("quantum.measurement")
requires capability("accelerator.compute")
requires memory >= required_memory
requires latency <= allowed_latency
requires reliability >= required_reliability

These describe semantic intent.

They do not select a physical resource.

---

29. Resource Preference

A preference means:

«a realization is preferred when available but is not necessarily required.»

For example:

prefer accelerator
prefer quantum
prefer low_latency
prefer energy_efficiency

The compiler/runtime MAY select another valid implementation.

---

30. Resource Hint

A hint is advisory.

A hint MUST NOT change program semantics unless the language explicitly defines it as a semantic constraint.

Examples include:

hint vectorize
hint parallelize
hint locality
hint cache
hint accelerator

The optimizer may ignore an advisory hint.

---

31. Implementation Decisions

Implementation decisions include:

- physical device selection;
- physical qubit mapping;
- memory placement;
- register allocation;
- thread assignment;
- GPU block selection;
- FPGA placement;
- network route;
- accelerator instance;
- scheduling timestamp.

These MUST NOT be required in portable source unless the program explicitly belongs to a target-specific deployment or hardware-description context.

Even then, the distinction between portable source and target-specific source MUST remain explicit.

---

32. No Hard-Coded Hardware Limits

The hybrid grammar MUST NOT impose universal limits such as:

MAX_CPUS
MAX_CORES
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_ACCELERATORS
MAX_QPUS
MAX_QUBITS
MAX_MEMORY
MAX_NODES
MAX_DEVICES
MAX_DOMAINS
MAX_OPERATIONS
MAX_CROSSINGS
MAX_TENSOR_DIMENSION
MAX_VECTOR_LENGTH
MAX_PIPELINE_DEPTH
MAX_CONCURRENCY

as language-expression limits.

Likewise, it MUST NOT require:

CPU0
GPU0
FPGA0
QPU0
NODE0

as the fundamental model of hybrid computation.

---

33. No Artificial Domain Limit

The number of computational domains in a program MUST NOT be a fixed grammar limit.

The semantic model must permit:

domain A ↔ domain B

and:

domain A ↔ domain B ↔ domain C ↔ ...

subject only to actual finite implementation resources and explicitly supplied validation policies.

The grammar is extensible.

---

34. No "Infinite Machine" Claim

"Scale to infinity" is interpreted correctly as:

«no artificial finite language ceiling is imposed where the semantic model can represent an arbitrarily large finite computation.»

Rust runtime data structures remain finite.

Actual limits may arise from:

- available memory;
- address space;
- compiler resources;
- runtime resources;
- target resources;
- operating-system constraints;
- network capacity;
- energy;
- execution time;
- deployment policy.

Those are resource limits, not language grammar limits.

---

35. Hybrid Data Movement

Hybrid data movement MAY involve:

classical memory
accelerator memory
quantum-associated state/results
distributed memory
persistent data
streams
buffers
tensors
records

The grammar MUST describe semantic data relationships.

It MUST NOT prescribe:

- physical address;
- memory bank;
- cache;
- PCIe location;
- NUMA node;
- DMA channel;
- vendor-specific queue.

Those belong downstream.

---

36. Ownership Across Domains

When a value crosses domains, the semantic system MUST preserve its ownership/lifetime rules.

Potential transitions include:

borrow
share
move
copy
serialize
transfer
materialize
consume
produce

The hybrid grammar MUST reuse the universal memory and ownership model.

It MUST NOT silently invent a second ownership model.

---

37. Type Safety Across Domains

Cross-domain values MUST be type checked.

Examples:

classical scalar → quantum parameter
measurement result → classical Boolean
classical tensor → accelerator tensor
accelerator result → classical tensor

Semantic analysis MUST determine whether conversions are:

- implicit;
- explicit;
- prohibited;
- lossy;
- exact;
- asynchronous;
- capability-dependent.

Grammar alone MUST NOT perform semantic type checking.

---

38. Explicit Conversion

If a domain crossing changes representation or semantics, the language SHOULD require an explicit conversion unless a safe implicit conversion is defined by the universal type system.

Examples of semantic categories:

classical → quantum_parameter
quantum_result → classical
host_buffer → accelerator_buffer
accelerator_buffer → host_buffer
distributed_value → local_value

The exact type and conversion syntax belongs to the universal type/expressions specifications.

---

39. Effects Across Domains

Hybrid computation may involve effects including:

- IO;
- communication;
- measurement;
- randomness;
- device interaction;
- allocation;
- synchronization;
- persistence;
- external calls;
- nondeterministic execution.

Effects MUST be represented through the universal effect system.

Hybrid grammar MUST NOT invent an independent effect language.

The existing:

grammar/effects/
grammar/spec/effects.md

remain authoritative for effect semantics.

---

40. Determinism

Hybrid computation MAY be:

- deterministic;
- probabilistic;
- quantum-probabilistic;
- nondeterministic;
- externally nondeterministic;
- explicitly randomized.

The language MUST distinguish these semantic properties where they affect observable behavior.

Compiler transformations MUST NOT silently convert a deterministic program into one with additional observable nondeterminism.

Conversely, a program explicitly permitting nondeterminism MUST NOT require a deterministic hardware execution strategy.

---

41. Reproducibility

Where reproducibility is requested, the semantic contract MUST identify what must remain reproducible.

Possible levels include:

same logical result
same classical result
same measurement distribution
same execution trace
same deterministic schedule
same artifact
same provenance

The grammar expresses intent.

Compiler and runtime systems enforce the actual reproducibility guarantees.

---

42. Classical Control and Quantum Dynamic Control

Hybrid control MUST preserve dependency ordering.

Example:

measure q
if result {
    apply operation
}

The semantic dependency is:

measure
   ↓
classical result
   ↓
predicate
   ↓
operation

The compiler MUST NOT reorder the controlled operation before the measurement when that would alter semantics.

The scheduler MUST receive the dependency.

The runtime MUST execute according to the compiled dependency.

---

43. Looping Across Domains

Hybrid loops MAY contain:

- classical operations;
- quantum operations;
- accelerator operations;
- measurements;
- classical predicates;
- resource operations.

Examples:

for iteration in iterations {
    quantum_region
    result = measure(...)
    classical_update
}

The loop count MAY be:

- static;
- runtime-known;
- symbolic;
- data-dependent;
- measurement-dependent.

No universal loop-count limit belongs in grammar.

---

44. Dynamic Allocation

Hybrid programs MAY require resources dynamically.

Examples:

allocate logical state
execute
release

The grammar MUST NOT impose a maximum number of:

- qubits;
- buffers;
- accelerators;
- workers;
- resources.

Resource management is downstream.

---

45. Logical Versus Physical Resources

The source-level hybrid model MUST distinguish:

logical resource

from:

physical resource

Examples:

logical qubit
physical qubit
logical accelerator
physical accelerator
logical memory
physical memory
logical node
physical node

Portable source should generally express logical resources and capabilities.

Physical resources are selected downstream.

The repository's quantum scheduling architecture likewise establishes canonical logical/physical qubit identities rather than allowing independent scheduling abstractions to invent new identifiers.

---

46. Quantum Resource Integration

Hybrid grammar MAY express quantum requirements such as:

requires capability("quantum")
requires capability("quantum.measurement")
requires capability("quantum.dynamic_control")

It MUST NOT choose:

physical_qubit(17)

as the default portable representation.

Physical mapping belongs to routing/HAL/backend systems.

---

47. QEC Integration

Hybrid source MAY express QEC-related intent where supported by the quantum specification.

Examples of semantic intent include:

requires fault_tolerance
requires error_correction
requires logical_reliability

The grammar MUST NOT implement:

- syndrome extraction;
- decoding;
- correction;
- code construction;
- physical qubit layout;
- QEC scheduling.

QEC remains a downstream semantic/implementation subsystem.

---

48. ZQN Integration

ZQN owns fault/noise semantics.

Hybrid grammar MUST NOT create another noise model.

If hybrid source contains a requirement related to:

- noise;
- reliability;
- fault tolerance;
- error budget;

the grammar records syntax.

Semantic analysis converts it into the appropriate semantic requirement.

ZQN determines applicable fault/noise semantics later.

---

49. Scheduling Integration

Hybrid grammar MUST preserve dependencies required by scheduling.

Scheduling may need to know:

A must happen before B
A produces value required by B
A and B may execute concurrently
A and B require mutually exclusive resources
A and B require synchronization
A is conditional on B

The grammar does not assign physical time.

It does not assign:

t = 17
core = 4
qubit = 12
stream = 3

as universal execution decisions.

---

50. Routing Integration

Hybrid grammar MUST NOT perform physical routing.

For quantum operations:

logical operation
      ↓
quantum::ir
      ↓
routing
      ↓
physical realization

For distributed/accelerator computation:

logical communication
      ↓
semantic communication
      ↓
placement/routing
      ↓
physical realization

The same principle applies to future domains.

---

51. Optimization Integration

Hybrid optimizers MAY transform:

classical + quantum

or:

classical + accelerator

or:

classical + quantum + accelerator

provided semantic equivalence is preserved.

Examples include:

- constant propagation;
- algebraic simplification;
- quantum circuit optimization;
- accelerator fusion;
- tensor fusion;
- communication reduction;
- dead computation elimination;
- parallelization;
- batching.

The grammar does not own these transformations.

---

52. Hardware Abstraction Integration

The hardware layer translates abstract requirements into actual capabilities.

Conceptually:

source requirement
        ↓
semantic requirement
        ↓
capability matching
        ↓
resource selection
        ↓
target realization

Hybrid grammar MUST NOT directly inspect hardware inventory.

---

53. Runtime Integration

Runtime receives compiled representations.

Runtime may perform:

- resource acquisition;
- asynchronous submission;
- synchronization;
- result collection;
- retries where semantically permitted;
- monitoring;
- recovery;
- device communication;
- execution lifecycle management.

None of these mechanisms should be embedded into grammar productions.

---

54. Resilience Integration

Hybrid computation can cross failure domains.

The resilience subsystem may handle:

- degraded resources;
- unavailable accelerators;
- failed quantum devices;
- failed distributed nodes;
- transient communication errors;
- recovery;
- escalation;
- quarantine.

The hybrid grammar MUST NOT become a retry implementation.

A source-level resilience policy is semantic intent.

Actual recovery remains downstream.

---

55. Distributed Hybrid Computing

Hybrid programs MAY distribute different computational domains across different resources.

Examples:

classical coordinator
      ↓
distributed accelerator workers
      ↓
quantum execution
      ↓
classical aggregation

The language MUST NOT require a fixed number of nodes.

The programmer expresses logical distributed intent.

Placement is downstream.

---

56. Network Integration

Cross-domain computation MAY require communication.

The networking layer owns:

- endpoints;
- protocols;
- connections;
- transport;
- routing;
- serialization.

Hybrid grammar owns only the semantic fact that a cross-domain computation requires communication.

Physical network details belong downstream.

---

57. AI/ML Integration

Hybrid computation MAY combine:

classical
quantum
AI/ML
accelerators
data

Examples:

classical preprocessing
        ↓
AI model
        ↓
quantum subroutine
        ↓
classical training update

The AI grammar owns AI-specific source concepts.

The hybrid grammar owns their interaction.

Hybrid grammar MUST NOT become a second AI language.

---

58. Data Integration

Hybrid values MAY include:

- tensors;
- matrices;
- arrays;
- streams;
- datasets;
- measurement results;
- classical records;
- accelerator buffers.

Data semantics belong to:

grammar/data/
grammar/spec/

Hybrid semantics define the relationship between domains.

---

59. HDL and Hardware Co-Design

Hybrid computation MAY interact with HDL/hardware intent.

For example:

algorithm
    ↓
accelerator intent
    ↓
HDL implementation
    ↓
software control

The source program MAY describe the relationship between:

- software;
- accelerator;
- hardware module;
- communication;
- memory;
- timing;
- verification properties.

The hybrid grammar MUST NOT duplicate HDL syntax.

HDL remains authoritative under:

grammar/hdl/
grammar/spec/hdl.md

---

60. Compile-Time Versus Runtime Domain Crossing

A domain crossing MAY occur:

- at compile time;
- at specialization time;
- at execution time;
- dynamically during execution.

The semantic model MUST preserve this distinction.

For example:

compile-time constant

is different from:

runtime classical parameter

which is different from:

measurement-derived parameter

The compiler MUST NOT treat them as interchangeable when doing so changes semantics.

---

61. Static and Dynamic Hybrid Programs

Zamani hybrid programs MUST support both:

Static hybrid computation

The complete interaction structure is known before execution.

Dynamic hybrid computation

The future interaction structure depends on runtime information.

Dynamic behavior MAY depend upon:

- classical input;
- measurement results;
- data-dependent control flow;
- resource availability where explicitly permitted;
- runtime capabilities.

The grammar MUST permit both models without forcing all hybrid computation into static circuits.

---

62. Capability Negotiation

The program MAY specify capabilities rather than devices.

Conceptually:

requires capability("quantum.dynamic_control")

The implementation performs:

required capability
        ↓
available capabilities
        ↓
compatible implementation

If multiple implementations satisfy the requirement, selection is downstream.

This is essential for POCO-REAF.

---

63. Capability Failure

If an execution target does not satisfy a required capability:

1. compilation MAY select another compatible target;
2. compilation MAY select a valid fallback;
3. runtime MAY select another compatible resource;
4. otherwise the system MUST produce a clear diagnostic.

The grammar itself MUST NOT hard-code one device as the only valid implementation.

---

64. Hybrid Fallback Semantics

A fallback is legal only when it preserves the source-level semantic contract.

For example:

accelerator preferred

may fall back to CPU.

But:

requires hardware-specific capability

cannot silently fall back if that would change semantics.

Therefore:

preference

and:

requirement

must remain distinguishable in the AST and semantic model.

---

65. Error Handling

Hybrid diagnostics MUST distinguish:

Syntax errors

Examples:

- missing delimiter;
- malformed domain construct;
- incomplete block;
- invalid token sequence.

Semantic errors

Examples:

- incompatible type;
- invalid domain crossing;
- unavailable required capability;
- invalid lifetime;
- illegal control dependency.

Resource errors

Examples:

- insufficient resources;
- unmet capacity requirement.

Target errors

Examples:

- target cannot realize required capability.

Runtime errors

Examples:

- execution failure;
- communication failure;
- resource loss.

The grammar must not turn all failures into parser errors.

---

66. Source Locations

Every hybrid AST construct MUST preserve source location information.

At minimum:

start
end

or the repository's canonical source-span representation.

Locations MUST survive:

lexer
→ parser
→ AST
→ semantic model
→ IR

Diagnostics must be able to identify:

- domain crossing;
- producer;
- consumer;
- invalid control;
- invalid resource expression.

---

67. AST Contract

Hybrid grammar MUST lower to the domain-neutral frontend AST.

The AST MUST preserve enough information to represent:

domain
operation
operands
parameters
results
attributes
modifiers
effects
capabilities
resource requirements
control dependencies
source span

The generic operation architecture preferred by Zamani MUST remain applicable.

Hybrid grammar MUST NOT require a giant:

enum HybridOperation {
    ClassicalToQuantum,
    QuantumToClassical,
    GPU,
    FPGA,
    ...
}

if the generic operation model can represent the semantics.

---

68. Semantic Contract

Semantic analysis MUST determine:

- participating domains;
- valid domain crossings;
- type compatibility;
- ownership;
- effects;
- capability requirements;
- resource requirements;
- synchronization;
- control dependencies;
- execution ordering;
- static/dynamic status;
- deterministic/reproducible requirements.

The parser MUST NOT attempt to make these decisions merely by matching syntax.

---

69. IR Contract

Hybrid semantics MUST lower into canonical domain IRs and explicit cross-domain relationships.

Conceptually:

Hybrid AST
     ↓
Semantic model
     ↓
Classical semantics ───────┐
                           ├── dependency graph
Quantum semantics ─────────┤
                           │
Accelerator semantics ─────┤
                           │
Other domains ─────────────┘
     ↓
Canonical IR

Quantum portions MUST eventually use:

quantum::ir

No second quantum IR is permitted.

---

70. Control-Flow IR Integration

Structured hybrid control flow MUST integrate with the repository's canonical control-flow IR.

The existing control-flow implementation explicitly describes itself as a target-independent representation for hybrid classical/quantum programs and supports branches, loops, predicates, measurement-driven feedback, operation references, and scalable control-flow representation.

Therefore hybrid grammar MUST preserve:

- branch structure;
- loop structure;
- operation references;
- predicate dependencies;
- logical resource references;
- structured transfers;
- source provenance.

---

71. Operation Identity

Hybrid grammar MUST NOT assign physical identities to operations.

Operations should ultimately be associated with canonical semantic identities such as the repository's "OperationId" model.

The grammar itself may only preserve the source construct from which that identity is created.

Adding a new quantum gate, accelerator operation, or future domain operation MUST NOT require redesigning hybrid control flow.

---

72. Resource Identity

Resource identity has two layers:

logical/resource intent identity

and:

physical realization identity

Hybrid source should primarily use the former.

Physical identities MAY appear in:

- deployment specifications;
- target-specific hardware descriptions;
- diagnostics;
- backend metadata;
- debugging;
- explicit non-portable code.

They MUST NOT become the universal hybrid programming model.

---

73. Interoperability

Hybrid computation MAY interoperate with:

- OpenQASM;
- QIR;
- LLVM-based systems;
- MLIR-based systems;
- C;
- C++;
- Rust;
- Python;
- WebAssembly;
- HDL;
- accelerator APIs;
- vendor systems.

These are interoperability boundaries.

They are NOT canonical Zamani semantic models.

The pipeline remains:

external representation
        ↓
import/adaptation
        ↓
Zamani semantic representation
        ↓
canonical IR

and the reverse for export.

---

74. Vendor Independence

Hybrid grammar MUST NOT require:

CUDA
ROCm
SYCL
OpenCL
vendor-QPU-name
vendor-FPGA-name
vendor-driver-name

as core language syntax.

Vendor-specific capabilities belong in:

- interoperability;
- dialects;
- hardware descriptions;
- backend metadata;
- target profiles.

Portable Zamani programs remain vendor-independent.

---

75. Dialect Integration

A dialect MAY extend hybrid syntax.

A dialect MUST declare:

- name;
- version;
- owner;
- syntax extension;
- semantic extension;
- AST mapping;
- IR mapping;
- compatibility;
- capability requirements.

A dialect MUST NOT silently change the meaning of stable core hybrid syntax.

---

76. Security

Cross-domain communication MAY have security implications.

Hybrid semantics MUST be compatible with:

identity
authorization
capabilities
secrets
provenance
secure communication
secure computation

Security semantics belong to:

grammar/security/

Hybrid grammar MUST NOT invent a competing security model.

---

77. Provenance

Hybrid transformations SHOULD preserve provenance.

The system SHOULD be able to trace:

source construct
      ↓
AST node
      ↓
semantic operation
      ↓
IR operation
      ↓
lowered operation
      ↓
target operation

This is particularly important when:

- classical controls quantum operations;
- a quantum result controls classical computation;
- accelerators are inserted;
- fallback implementations are selected;
- optimizations cross domain boundaries.

---

78. Observability

Hybrid execution MAY expose:

- operation identity;
- domain;
- timing;
- resource usage;
- measurement results;
- accelerator status;
- communication events;
- failures;
- recovery events.

Observability is downstream of grammar.

The grammar only needs to preserve source-level constructs required to associate runtime events with source provenance.

---

79. Cancellation

Asynchronous hybrid operations MAY be cancellable.

Cancellation semantics belong to concurrency/runtime specifications.

Hybrid grammar MUST preserve cancellation-related source intent where the language exposes it.

Cancellation MUST NOT silently corrupt:

- resource ownership;
- quantum state semantics;
- classical state;
- transactional semantics.

---

80. Transactions and Atomicity

A hybrid region MAY require transactional or atomic semantics where supported.

The semantic model MUST define what atomicity means.

It MUST NOT assume that a hardware instruction is automatically atomic across domains.

For example:

classical update
+
accelerator submission
+
quantum operation

does not automatically constitute one physical atomic operation.

---

81. Memory and Data Coherence

When a value crosses domains, the semantic model MUST define the required visibility/coherence guarantees.

Possible semantic states include:

shared
copied
moved
borrowed
materialized
serialized
eventually-consistent
synchronized

The actual mechanism is implementation-defined.

---

82. Timing Semantics

Hybrid source MAY express semantic timing requirements such as:

latency <= bound
deadline <= bound
timeout <= bound

It MUST NOT assume:

CPU cycle
GPU clock
QPU clock
physical timestamp

unless explicitly defined by a target-specific language layer.

Scheduling owns physical timing.

---

83. Real-Time Hybrid Computation

Hybrid programs MAY have real-time or bounded-latency requirements.

The compiler/runtime MUST determine whether the selected target can satisfy those requirements.

If no compatible realization exists, the system MUST diagnose the failure rather than silently changing semantics.

---

84. Near-Time and Deferred Execution

The language SHOULD distinguish, where semantically relevant:

immediate interaction
near-time interaction
deferred interaction
batch execution
asynchronous execution

This distinction matters particularly for:

- measurement-driven quantum programs;
- accelerator execution;
- distributed computation.

The grammar expresses the semantic mode.

The runtime chooses the actual mechanism.

---

85. Hybrid Concurrency

Independent domain computations MAY execute concurrently.

Example:

classical computation A
        │
        ├── independent
        │
        ▼
quantum computation B

accelerator computation C

The compiler MAY execute independent computations concurrently.

If ordering is semantically required, that dependency MUST be preserved.

---

86. Parallel Scaling

The language MUST support scaling across:

one worker
multiple workers
many workers
distributed workers
heterogeneous workers
future execution resources

without requiring source rewriting.

The number of workers is a runtime/compiler resource decision unless explicitly part of program semantics.

---

87. Nested Parallelism

Hybrid computation MAY contain nested parallel regions.

For example:

parallel classical
    ├── quantum operation
    ├── accelerator operation
    └── classical operation

The grammar MUST NOT impose a fixed nesting depth.

---

88. Data Parallel Hybrid Computation

The language MAY express data-parallel hybrid computation.

Conceptually:

for each input:
    classical preprocessing
    quantum computation
    classical postprocessing

The compiler MAY transform this into:

- batching;
- vectorization;
- accelerator execution;
- distributed execution;
- parallel quantum execution where legal.

Such transformation is downstream.

---

89. Task Parallel Hybrid Computation

Independent hybrid tasks MAY execute concurrently.

The semantic model MUST represent task dependencies.

The scheduler determines:

- worker count;
- placement;
- order;
- resource allocation.

---

90. Pipeline Hybrid Computation

Hybrid pipelines MAY contain:

classical stage
→ quantum stage
→ accelerator stage
→ classical stage

Each stage MUST expose semantic input/output dependencies.

The compiler MAY pipeline them.

The grammar MUST NOT encode fixed pipeline hardware.

---

91. Batch Execution

A hybrid operation MAY be semantically batchable.

Batch size MUST NOT be a universal grammar limit.

The compiler/runtime may choose batch size based on:

- available resources;
- target capabilities;
- latency requirements;
- memory;
- throughput;
- scheduling policy.

---

92. Hybrid Streams

Hybrid computation MAY consume or produce streams.

Examples:

classical stream
→ accelerator
→ quantum sampling
→ classical aggregation

Stream semantics belong to the universal/data/concurrency models.

Hybrid grammar represents the domain relationship.

---

93. Backpressure

When streams cross domains, the system MAY require backpressure.

The semantic model MUST distinguish:

producer rate
consumer capacity
buffering policy
blocking behavior
dropping policy

The grammar MUST NOT hard-code buffer sizes.

---

94. Resource Exhaustion

Resource exhaustion is not a grammar error unless the source explicitly violates a compile-time resource contract.

For example:

requires memory >= requirement

may be statically checked if the environment is known.

Otherwise resource matching remains a compiler/runtime concern.

---

95. Graceful Degradation

A program MAY specify that certain capabilities are preferred rather than mandatory.

For example:

prefer accelerator

allows fallback.

But:

requires capability(...)

does not permit silent degradation if the requirement is semantically mandatory.

This distinction MUST survive into the semantic representation.

---

96. Hardware Scaling

The same source program SHOULD remain valid when moving between:

tiny embedded system
single CPU
multicore CPU
GPU system
FPGA system
heterogeneous workstation
cluster
cloud
quantum computer
quantum-classical system
future machine

provided the target satisfies the semantic requirements.

No source rewrite should be required merely because the resource scale changes.

---

97. Embedded Hybrid Computing

Hybrid programs MAY target resource-constrained systems.

The source semantics remain the same.

A compiler MAY specialize:

- memory layout;
- computation;
- scheduling;
- communication;
- accelerator use.

The language MUST NOT introduce a separate restricted hybrid grammar merely because the target is small.

---

98. HPC Hybrid Computing

The same language MUST support large-scale hybrid HPC workloads.

Possible combinations include:

CPU + GPU
CPU + FPGA
CPU + QPU
GPU + QPU
CPU + GPU + QPU
distributed heterogeneous systems

No universal machine-size assumptions are permitted.

---

99. Cloud Hybrid Computing

Hybrid computation MAY execute through remote services.

Remote execution MUST remain semantically distinct from:

physical device identity

The source can express:

requires remote capability

without embedding a fixed service endpoint.

---

100. Edge/Cloud Portability

A program SHOULD be able to move between:

edge
local
cluster
cloud
remote accelerator
remote QPU

without source modification where semantic requirements remain satisfied.

Deployment policy chooses the realization.

---

101. Hybrid Security Boundaries

Cross-domain transitions MAY be security boundaries.

The semantic model SHOULD preserve:

- trust domain;
- identity;
- authorization;
- data classification;
- capability;
- provenance.

Actual enforcement belongs to security/runtime infrastructure.

---

102. Hybrid Exceptions and Failures

A failure in one domain MAY affect another domain.

The semantic model MUST distinguish:

operation failure
resource failure
communication failure
capability failure
timeout
cancellation
semantic failure

The runtime/resilience system determines recovery according to the program's policies.

---

103. Exception Safety

Cross-domain failures MUST NOT silently violate language guarantees for:

- ownership;
- resource lifetime;
- transactions;
- deterministic state;
- cleanup;
- cancellation.

The grammar itself does not implement cleanup.

---

104. Resource Lifetime

Resources acquired for a hybrid operation MUST have explicit semantic lifetime rules.

For example:

acquire
use
synchronize
release

The runtime may implement this using different mechanisms.

No physical resource identifier is required at the source level.

---

105. Capability Lifetime

Capabilities MAY be:

- static;
- dynamically acquired;
- temporarily available;
- revoked;
- unavailable.

The semantic model MUST allow runtime capability state without changing the meaning of the program's static syntax.

---

106. Hybrid Compilation

Compilation SHOULD proceed through:

source
  ↓
lexer
  ↓
parser
  ↓
AST
  ↓
structural validation
  ↓
semantic analysis
  ↓
capability/resource analysis
  ↓
canonical semantic representation
  ↓
classical IR / quantum::ir / other domain IR
  ↓
cross-domain dependency preservation
  ↓
optimization
  ↓
routing
  ↓
scheduling
  ↓
resilience / ZQN
  ↓
HAL
  ↓
target lowering

Hybrid grammar ends before target-specific realization.

---

107. Compilation Specialization

A compiler MAY specialize a hybrid program according to:

- target capabilities;
- available resources;
- optimization profile;
- deployment constraints;
- performance goals;
- reliability goals;
- power constraints;
- memory;
- communication characteristics.

Specialization MUST preserve semantic intent.

---

108. Separate Semantic Requirements from Implementation Decisions

This is a mandatory architectural rule.

Semantic requirement

requires quantum measurement

Capability requirement

requires capability("quantum.measurement")

Preference

prefer accelerator

Constraint

latency <= bound

Implementation decision

use device X
map resource Y to physical resource Z

The first four are portable source intent.

The last is normally downstream realization.

---

109. Generic Operation Model

Hybrid grammar SHOULD use generic operation concepts rather than enumerating every possible operation.

Conceptually:

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

This allows future domains to participate without redesigning the hybrid grammar.

---

110. Extensibility

Adding a new computational domain SHOULD require:

1. domain specification;
2. domain grammar component;
3. AST mapping;
4. semantic contract;
5. IR mapping;
6. capability/resource mapping;
7. compiler integration;
8. runtime integration;
9. conformance tests.

It SHOULD NOT require rewriting existing classical/quantum hybrid grammar.

---

111. Future Computational Domains

The architecture MUST permit future domains that do not currently exist.

Examples could include:

- neuromorphic;
- photonic;
- molecular;
- biological;
- optical;
- analog;
- probabilistic;
- reversible;
- memristive;
- spatial;
- temporal;
- emergent computational systems.

The hybrid model should treat these as semantic domains rather than requiring a finite list of devices.

---

112. Nano and Other Domains

If nano-computing or other specialized domains become first-class Zamani domains, their interaction with classical/quantum/accelerator computation MUST use the same hybrid contracts.

The hybrid grammar MUST NOT require special-case parser architecture for each future domain.

---

113. Sankofa Integration

Sankofa-style concepts such as:

- memory;
- recall;
- learning;
- history;
- temporal computation;
- provenance;
- reasoning;

may participate in hybrid computation.

Their domain semantics remain owned by their respective specifications.

Hybrid grammar only describes their interactions with other domains.

---

114. Multi-Timeline Integration

If multiple timelines are supported by Zamani, hybrid execution MAY occur across timelines.

The hybrid model MUST preserve:

- timeline identity;
- dependency;
- observation;
- fork;
- merge;
- synchronization;
- provenance.

There MUST be no fixed number of timelines or branches.

---

115. Speculative Hybrid Computation

Speculative execution MAY be used downstream.

The source semantic model MUST distinguish:

speculative computation

from:

observable committed computation

A speculative optimization MUST NOT change observable semantics.

---

116. Verification

Hybrid constructs SHOULD support verification of:

- type correctness;
- resource requirements;
- capability requirements;
- domain crossings;
- ownership;
- control dependencies;
- deterministic behavior;
- provenance;
- target compatibility.

Verification MUST occur before target realization where possible.

---

117. Formal Invariants

A conforming implementation MUST maintain these invariants:

Invariant 1 — Single language

Hybrid computation is part of Zamani, not a separate language.

Invariant 2 — Single AST

Hybrid syntax uses the domain-neutral frontend AST.

Invariant 3 — Single quantum semantic boundary

Quantum semantics ultimately use "quantum::ir".

Invariant 4 — No hardware semantics in grammar

Physical machine details are downstream.

Invariant 5 — Explicit dependencies

Cross-domain data/control dependencies are preserved.

Invariant 6 — No artificial resource ceilings

Grammar does not impose machine-size limits.

Invariant 7 — Capability/resource separation

Requirements are not device selections.

Invariant 8 — Deterministic parsing

Equivalent valid source has deterministic syntactic interpretation.

Invariant 9 — Source provenance

Hybrid constructs retain source locations.

Invariant 10 — Extensibility

New domains can participate without redesigning the entire language.

---

118. Grammar Validation Requirements

The hybrid grammar MUST be checked for:

- ambiguity;
- unreachable rules;
- duplicate rules;
- duplicate token dependencies;
- left recursion;
- precedence conflicts;
- keyword collisions;
- accidental parser nondeterminism;
- excessive parser backtracking;
- missing source spans;
- missing AST mappings;
- missing semantic mappings;
- missing IR mappings;
- hard-coded limits.

---

119. Hard-Coding Audit

Every hybrid grammar change MUST be checked for suspicious constructs including:

MAX_QUBITS
MAX_CPUS
MAX_CORES
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_ACCELERATORS
MAX_DEVICES
MAX_NODES
MAX_MEMORY
MAX_STREAMS
MAX_PIPELINES
MAX_DOMAINS
MAX_OPERATIONS

Also check for hard-coded physical identifiers:

CPU0
GPU0
FPGA0
QPU0
NODE0

Such identifiers may appear only when they are explicitly part of a target-specific layer.

They MUST NOT establish universal language semantics.

---

120. Boundary Testing

Boundary tests MUST include:

- one classical operation;
- one quantum operation;
- one accelerator operation;
- one crossing;
- many crossings;
- deeply nested hybrid regions;
- large dependency graphs;
- runtime-dependent values;
- measurement-dependent control;
- asynchronous operations;
- concurrent domain execution;
- distributed domain execution;
- very large resource expressions;
- symbolic resource requirements.

No test may encode an artificial universal maximum.

---

121. Scalability Testing

Scalability tests MUST verify that the grammar remains structurally valid as program size increases.

Test dimensions include:

number of operations
number of domain crossings
number of classical values
number of quantum values
number of accelerator operations
number of dependencies
number of nested regions
number of concurrent tasks
number of resource requirements
number of domains

The test suite SHOULD generate increasingly large finite programs.

The absence of an artificial grammar limit is the requirement.

---

122. Determinism Testing

The parser MUST deterministically interpret valid hybrid syntax.

Tests SHOULD verify:

- repeated parsing produces equivalent trees;
- equivalent source produces equivalent structural representation;
- no ambiguous domain crossing is silently selected;
- source spans remain stable;
- grammar changes do not introduce accidental parse alternatives.

---

123. Negative Testing

Negative tests MUST include:

- malformed domain crossings;
- missing operands;
- missing results;
- malformed resource expressions;
- malformed capability expressions;
- invalid delimiters;
- incomplete control constructs;
- malformed asynchronous constructs;
- malformed accelerator constructs;
- invalid hybrid block structure.

Semantic errors SHOULD be tested separately from syntax errors.

---

124. Compatibility Testing

Compatibility tests MUST verify alignment between:

grammar/spec/hybrid.md
grammar/hybrid/*.g4
grammar/Zamani.g4
lexer
parser
frontend AST
semantic model
IR
compiler
runtime

A hybrid feature is not production-complete merely because ANTLR accepts it.

---

125. Feature Completion Contract

Every hybrid feature MUST define:

Feature ID
Feature name
Status
Language version
Syntax owner
Grammar rule
Lexer dependencies
AST node/mapping
Semantic rules
IR mapping
Classical integration
Quantum integration
Accelerator integration
Resource integration
Capability integration
Compiler consumers
Runtime consumers
Tooling consumers
Diagnostics
Positive tests
Negative tests
Boundary tests
Scalability tests
Determinism tests
Compatibility tests
Hard-coding audit

A feature MUST NOT be marked complete until all required contracts exist.

---

126. File Completion Contract

Every hybrid grammar file MUST independently document or have an externally stable contract for:

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
Compatibility Tests
Determinism Tests
Hard-Coding Audit
Diagnostics
Security
Performance
Completion Criteria

This prevents the need to reopen a completed file merely because another file was later implemented.

---

127. "hybrid.g4" Completion Contract

"hybrid.g4" is complete only when:

- composition rules are defined;
- no detailed rules are duplicated;
- dependencies are stable;
- AST mapping is predetermined;
- semantic mapping is predetermined;
- IR integration is predetermined;
- resource integration is predetermined;
- diagnostics are specified;
- all subordinate hybrid grammars are integrated;
- root "Zamani.g4" integration is documented;
- positive tests pass;
- negative tests pass;
- boundary tests pass;
- scalability tests pass;
- determinism tests pass;
- hard-coding audit passes.

---

128. "classical-quantum.g4" Completion Contract

This file is complete only when it supports the complete required semantic relationship between classical and quantum computation, including:

- classical parameters to quantum operations;
- quantum results to classical values;
- measurement results;
- control dependencies;
- data dependencies;
- nested crossings;
- repeated crossings;
- dynamic values;
- asynchronous values where supported;
- source provenance;
- AST mapping;
- semantic mapping;
- "quantum::ir" integration;
- classical IR integration;
- scheduler dependency integration;
- runtime boundary;
- diagnostics;
- tests.

It MUST NOT enumerate quantum gates.

---

129. "quantum-classical-control.g4" Completion Contract

This file is complete only when it supports:

- measurement-driven control;
- classical predicates;
- compound predicates;
- conditional quantum operations;
- nested control;
- loops;
- dynamic control;
- dependency preservation;
- AST mapping;
- semantic mapping;
- canonical control-flow IR integration;
- scheduler integration;
- runtime integration;
- capability diagnostics;
- scalability tests.

---

130. "accelerator-interoperability.g4" Completion Contract

This file is complete only when it supports:

- generic accelerator intent;
- accelerator computation regions;
- data movement intent;
- input/output semantics;
- capability requirements;
- resource requirements;
- accelerator preferences;
- asynchronous operation where supported;
- heterogeneous combinations;
- fallback semantics;
- AST mapping;
- semantic mapping;
- compiler integration;
- runtime integration;
- no vendor lock-in;
- no fixed accelerator count;
- no fixed device IDs.

---

131. "hybrid-resources.g4" Completion Contract

This file is complete only when it distinguishes:

requirement
constraint
capability
preference
hint
target
placement
implementation decision

and preserves all information required by the resource system.

It MUST NOT allocate resources.

It MUST NOT discover hardware.

It MUST NOT select physical devices.

---

132. Compiler Integration Contract

The compiler MUST:

1. parse hybrid constructs;
2. construct domain-neutral AST nodes;
3. validate structure;
4. perform type analysis;
5. perform effect analysis;
6. perform capability analysis;
7. perform resource analysis;
8. construct semantic dependencies;
9. lower classical portions;
10. lower quantum portions;
11. preserve "quantum::ir";
12. preserve cross-domain dependencies;
13. optimize;
14. route where required;
15. schedule;
16. apply resilience/ZQN policies;
17. lower through HAL;
18. generate target code/artifacts.

---

133. Runtime Integration Contract

The runtime MUST be able to consume compiled hybrid representations that express:

- domain;
- operation;
- dependencies;
- data;
- synchronization;
- resources;
- capabilities;
- execution policy;
- provenance.

Runtime MUST determine actual resources dynamically when the source does not explicitly constrain them.

---

134. Tooling Integration Contract

Hybrid syntax MUST support:

- syntax highlighting;
- formatting;
- parser diagnostics;
- source navigation;
- AST inspection;
- semantic diagnostics;
- code completion;
- refactoring;
- documentation lookup;
- source-to-IR provenance where supported.

Tooling MUST consume the canonical grammar and AST contracts.

It MUST NOT maintain a separate unofficial hybrid grammar.

---

135. Documentation Integration

The following files must remain synchronized through defined contracts:

grammar/spec/hybrid.md
grammar/hybrid/README.md
grammar/hybrid/hybrid.g4
grammar/hybrid/classical-quantum.g4
grammar/hybrid/quantum-classical-control.g4
grammar/hybrid/accelerator-interoperability.g4
grammar/hybrid/hybrid-resources.g4
grammar/spec/quantum.md
grammar/spec/classical.md
grammar/spec/resources.md
grammar/spec/semantics.md
grammar/spec/portability.md
grammar/spec/type-system.md

Synchronization means contractual consistency.

It does NOT mean copying the same prose into every file.

---

136. "grammar/Zamani-Grammar.md" Integration

"grammar/Zamani-Grammar.md" may contain broader hybrid proposals.

However:

Zamani-Grammar.md

MUST NOT automatically make a feature normative.

A proposed hybrid feature must progress through:

proposal
  ↓
semantic definition
  ↓
AST contract
  ↓
IR contract
  ↓
grammar
  ↓
implementation
  ↓
tests
  ↓
compatibility review
  ↓
stable

---

137. "grammar/grammar.md" Integration

"grammar/grammar.md" is an implementation-conformance reference.

It MUST identify whether each hybrid feature is:

specified
implemented
partially implemented
experimental
deprecated
not implemented

It MUST NOT override this normative specification.

---

138. "grammar/spec/quantum.md" Integration

"grammar/spec/quantum.md" owns quantum semantics.

Hybrid specification owns:

quantum ↔ classical
quantum ↔ accelerator
quantum ↔ other domains

relationships.

There must be no duplication of:

- gate semantics;
- measurement semantics;
- quantum type semantics;
- QEC algorithms;
- noise semantics.

---

139. "grammar/spec/classical.md" Integration

"grammar/spec/classical.md" owns classical semantics.

Hybrid specification owns the interaction between classical semantics and other domains.

There must be no second classical numeric or collection model.

---

140. "grammar/spec/resources.md" Integration

"grammar/spec/resources.md" owns universal resource semantics.

Hybrid resources MUST conform to that model.

Hybrid-specific resource syntax may add context but MUST NOT redefine:

requirement
capability
constraint
preference
hint

with incompatible meanings.

---

141. "grammar/spec/portability.md" Integration

POCO-REAF is a repository-wide portability principle.

Hybrid computation is one of its primary use cases.

Hybrid source MUST remain target-independent except where the programmer explicitly enters a target-specific domain.

---

142. "grammar/spec/semantics.md" Integration

General semantic rules for:

- evaluation;
- scope;
- effects;
- determinism;
- concurrency;
- ownership;
- resource behavior

remain under "semantics.md".

This document specializes those rules for cross-domain computation.

---

143. Security and Privacy

Hybrid grammar implementations MUST NOT expose:

- credentials;
- secrets;
- private hardware information;
- authentication material

merely through source parsing.

Sensitive runtime information belongs to secure runtime infrastructure.

---

144. Rust Implementation Requirements

All supporting Rust implementation MUST target:

Rust 1.97
Rust 1.97.1
Rust 2021

and MUST compile without requiring nightly-only language features.

All Rust implementation MUST use:

#![forbid(unsafe_code)]

or the repository's equivalent crate/module-level enforcement.

No "unsafe" code is permitted.

---

145. Rust Portability

The hybrid grammar contract MUST NOT depend upon:

- platform-specific pointer sizes;
- target-specific integer widths;
- architecture-specific SIMD;
- operating-system-specific APIs;
- vendor-specific runtime assumptions.

Such implementation details belong behind defined abstraction boundaries.

---

146. Arithmetic Safety

Any hybrid resource/accounting implementation MUST use checked arithmetic where overflow could invalidate:

- resource counts;
- operation counts;
- dependency counts;
- buffer sizes;
- memory calculations;
- scheduling calculations.

Overflow MUST produce a defined error rather than silently wrapping when the semantic calculation requires exactness.

---

147. Parser Safety

The parser MUST reject malformed syntax safely.

Parser failure MUST NOT:

- panic on ordinary malformed user input;
- access invalid memory;
- rely upon "unsafe";
- silently reinterpret malformed hybrid constructs.

---

148. Resource Safety

A resource requirement MUST NOT itself cause allocation.

For example:

requires memory >= N

describes a requirement.

The grammar parser must not allocate "N" units of memory.

Resource allocation belongs downstream.

---

149. Semantic Safety

A domain crossing MUST NOT bypass:

- type checking;
- ownership checking;
- capability checking;
- effect checking;
- resource checking;
- security checking.

Macros and metaprogramming MUST NOT provide a hidden bypass.

---

150. Macro Integration

Macros MAY generate hybrid constructs.

Macro expansion MUST occur before the semantic validation stage that requires the expanded structure.

Generated constructs MUST receive the same:

- syntax validation;
- type checking;
- semantic validation;
- capability checking;
- resource analysis;
- IR lowering

as directly written constructs.

---

151. Metaprogramming Integration

Metaprogramming MAY inspect or generate hybrid structures.

It MUST NOT directly mutate canonical IR in a way that bypasses language invariants unless an explicitly defined compiler-internal API is used.

---

152. Performance Requirements

The grammar should remain scalable as the source program grows.

Implementation SHOULD avoid:

- unnecessary backtracking;
- repeated reparsing;
- duplicated semantic traversal;
- quadratic domain-crossing analysis where avoidable;
- storing redundant representations.

The grammar itself must remain declarative.

---

153. Memory Requirements

Hybrid grammar processing MUST scale with actual input size and implementation resources.

There MUST NOT be hidden allocations proportional to a hard-coded maximum number of:

- qubits;
- devices;
- accelerators;
- nodes;
- operations.

---

154. Incremental Compilation

The architecture SHOULD support incremental analysis.

Changing a classical region SHOULD NOT require rebuilding unrelated quantum or accelerator semantics when dependency analysis proves they are independent.

Changing a quantum operation SHOULD trigger reanalysis of dependent hybrid constructs.

The dependency graph should make such invalidation explicit.

---

155. Parallel Compilation

Compilation of independent hybrid regions MAY be parallelized.

The semantic result MUST remain deterministic where the language requires deterministic compilation semantics.

---

156. Canonical Ordering

Where hybrid constructs do not impose an order, the semantic representation MUST NOT invent an observable order merely because parser traversal happens to be sequential.

Where an order is semantically required, it MUST be explicit.

This is important for deterministic compilation and scalable parallelization.

---

157. Commutativity

Optimizers MAY exploit semantic commutativity where proven.

The grammar does not encode commutativity itself unless it is a language-level construct.

Quantum, classical, accelerator, and data semantics remain responsible for proving applicable transformations.

---

158. Side Effects

The compiler MUST NOT reorder hybrid operations across observable effects unless the effect system proves that the transformation is legal.

Potential effects include:

- measurement;
- IO;
- mutation;
- synchronization;
- external communication;
- randomness;
- resource acquisition;
- resource release.

---

159. Observable Behavior

Hybrid optimization is correct only when observable behavior is preserved.

Observable behavior may include:

- classical outputs;
- quantum measurement distributions;
- externally visible communication;
- resource guarantees;
- timing guarantees where explicitly semantic;
- security properties;
- deterministic ordering where required.

---

160. Testing Matrix

The hybrid conformance suite MUST include at least:

Classical ↔ Quantum

- classical parameter → quantum;
- measurement → classical;
- measurement-controlled quantum operation;
- classical loop around quantum region;
- quantum region inside classical conditional.

Classical ↔ Accelerator

- accelerator invocation;
- accelerator result;
- asynchronous accelerator operation;
- fallback;
- capability requirement.

Quantum ↔ Accelerator

- quantum accelerator intent;
- accelerator-produced classical result;
- quantum-controlled accelerator interaction where semantically supported.

Three-way

classical ↔ quantum ↔ accelerator

Distributed

classical ↔ distributed accelerator ↔ quantum

Future-domain extensibility

A synthetic domain extension MUST be representable without changing existing hybrid semantics.

---

161. Positive Test Requirements

Positive tests MUST cover:

- smallest legal hybrid program;
- normal hybrid program;
- deeply nested hybrid program;
- multiple crossings;
- repeated crossings;
- asynchronous crossings;
- dynamic control;
- symbolic parameters;
- generic values;
- resource requirements;
- capability requirements;
- preferences;
- hints;
- large finite programs;
- domain combinations.

---

162. Negative Test Requirements

Negative tests MUST cover:

- malformed syntax;
- malformed crossing;
- missing required operand;
- invalid resource syntax;
- malformed capability syntax;
- invalid block structure;
- illegal parser-level combinations.

Semantic invalidity should be tested separately.

---

163. Boundary Test Requirements

Boundary tests MUST cover:

- empty legal regions;
- one-element collections;
- one crossing;
- many crossings;
- nested control;
- large identifiers;
- large expressions;
- large dependency graphs;
- symbolic dimensions;
- large resource values;
- zero where legal;
- invalid zero where prohibited;
- maximum representable implementation values.

The language itself MUST NOT establish arbitrary small limits.

---

164. Compatibility Test Requirements

Compatibility tests MUST compare:

specification
grammar
lexer
parser
AST
semantic analysis
IR
compiler
runtime

for every hybrid feature.

Any change to public grammar rule names or semantic construct shapes requires compatibility review.

---

165. Conformance Levels

Hybrid implementation MAY expose:

SyntaxConformant
AstConformant
SemanticConformant
IrConformant
CompilerConformant
RuntimeConformant
TargetConformant

A feature MUST NOT be reported as fully supported merely because it reaches the parser.

---

166. Production Readiness Gate

"grammar/spec/hybrid.md" and the hybrid grammar are production-ready only when:

- authority is unambiguous;
- grammar composition is deterministic;
- universal grammar components are reused;
- domain crossings are explicitly represented;
- classical semantics remain canonical;
- quantum semantics remain canonical;
- "quantum::ir" remains the quantum boundary;
- resource intent is separated from implementation;
- hardware is downstream;
- no universal hardware limits are encoded;
- AST mappings are complete;
- semantic mappings are complete;
- IR mappings are complete;
- compiler consumers are identified;
- runtime consumers are identified;
- diagnostics are defined;
- interoperability is defined;
- positive tests exist;
- negative tests exist;
- boundary tests exist;
- scalability tests exist;
- determinism tests exist;
- compatibility tests exist;
- no "unsafe" Rust is required;
- Rust 1.97 / 1.97.1 compatibility is maintained.

---

167. Definition of "Done"

A hybrid feature is DONE only if:

syntax
  +
lexer contract
  +
parser contract
  +
AST contract
  +
semantic contract
  +
resource contract
  +
capability contract
  +
IR contract
  +
compiler contract
  +
runtime contract
  +
tooling contract
  +
cross-domain contract
  +
diagnostics
  +
positive tests
  +
negative tests
  +
boundary tests
  +
scalability tests
  +
determinism tests
  +
compatibility tests
  +
hard-coding audit

are all complete.

A feature is NOT done merely because its ".g4" rule parses.

---

168. Required Final Architecture

The complete hybrid pipeline SHALL be:

                    Zamani Source
                         │
                         ▼
                  grammar/Zamani.g4
                         │
                         ▼
                       Lexer
                         │
                         ▼
                       Parser
                         │
                         ▼
                  Domain-Neutral AST
                         │
                         ▼
              Structural Validation
                         │
                         ▼
                 Semantic Analysis
                         │
        ┌────────────────┼─────────────────┐
        │                │                 │
        ▼                ▼                 ▼
    Classical         Quantum          Accelerator
    semantics         semantics        semantics
        │                │                 │
        │                ▼                 │
        │           quantum::ir           │
        │                │                 │
        └────────────────┼─────────────────┘
                         │
                         ▼
              Cross-Domain Dependencies
                         │
                         ▼
             Capability / Resource Analysis
                         │
                         ▼
                    Optimization
                         │
              ┌──────────┼──────────┐
              │          │          │
              ▼          ▼          ▼
           Routing   Scheduling  Resilience
              │          │          │
              └──────────┼──────────┘
                         ▼
                        ZQN
                         │
                         ▼
                        HAL
                         │
                         ▼
                Target Realization
                         │
       ┌─────────┬───────┼───────┬─────────┐
       ▼         ▼       ▼       ▼         ▼
      CPU       GPU     FPGA     QPU      Future
       │         │       │       │       targets
       └─────────┴───────┼───────┴─────────┘
                         │
                         ▼
                      Runtime

---

169. POCO-REAF Guarantee

The hybrid architecture is considered POCO-REAF compliant when the following statement is true:

«A Zamani program expresses its computational semantics, required capabilities, constraints, correctness requirements, and resource intent without encoding unnecessary assumptions about the physical machine that will execute it.»

Consequently, the implementation may scale the same source program across:

atom
↓
single device
↓
embedded system
↓
single CPU
↓
multicore system
↓
GPU
↓
FPGA
↓
accelerator
↓
quantum processor
↓
heterogeneous system
↓
cluster
↓
distributed system
↓
cloud
↓
future computational substrate

subject to actual resource availability and semantic compatibility.

---

170. Fundamental Rule

The most important rule of this specification is:

«Hybrid Zamani syntax describes relationships between computations, not the physical machines executing those computations.»

Therefore:

WHAT

belongs to the source program.

WHICH MACHINE

belongs to compilation/deployment.

HOW IT IS OPTIMIZED

belongs to optimization.

WHERE IT RUNS

belongs to placement/routing/deployment.

WHEN IT RUNS

belongs to scheduling.

HOW ERRORS ARE CORRECTED

belongs to QEC/resilience.

HOW NOISE/FAULTS ARE MODELLED

belongs to ZQN.

HOW THE DEVICE IS CONTROLLED

belongs to HAL/backend/runtime.

The hybrid grammar MUST preserve these boundaries.

---

171. Final Integration Rule

No future hybrid grammar file may be added merely because a new hardware product, framework, library, accelerator, QPU, AI system, or vendor API exists.

A new file is justified only when it introduces a distinct language-level semantic responsibility that cannot be cleanly represented by an existing universal or domain grammar component.

When a new file is justified, it MUST first define:

Purpose
Owns
Does Not Own
Inputs
Outputs
Dependencies
Upstream Contracts
Downstream Consumers
Grammar Contract
AST Contract
Semantic Contract
IR Contract
Compiler Contract
Runtime Contract
Tooling Contract
Cross-Domain Contract
Tests
Scalability
Compatibility
Hard-Coding Policy
Completion Criteria

before implementation begins.

That is the required mechanism for keeping each file independently completable and preventing cascading rework.

---

172. Final Architectural Statement

Zamani hybrid computing is therefore not:

Classical language
    +
Quantum language
    +
GPU language
    +
FPGA language

It is:

                    ONE ZAMANI LANGUAGE
                           │
             ┌─────────────┼─────────────┐
             │             │             │
         Classical       Quantum       Hardware
             │             │             │
             └─────────────┼─────────────┘
                           │
                      Hybrid Semantics
                           │
             ┌─────────────┼──────────────┐
             │             │              │
            AI        Distributed      Data/Network
             │             │              │
             └─────────────┼──────────────┘
                           │
                 Capability / Resource
                           │
                 Canonical Semantic IR
                           │
             ┌─────────────┼─────────────┐
             │             │             │
       Classical IR    quantum::ir     HDL/Other IR
             │             │             │
             └─────────────┼─────────────┘
                           │
                    Optimization
                           │
              Routing / Scheduling
                           │
                    Resilience / ZQN
                           │
                          HAL
                           │
                  Target Realization
                           │
                Everywhere / Anywhere

The result is a single scalable semantic language in which classical, quantum, accelerator, HDL, distributed, AI, data, networking, and future computation can participate in one program without turning the grammar into a collection of hardware-specific sublanguages.

This specification is complete only when the existing hybrid grammar files, canonical "Zamani.g4", frontend AST, semantic layer, canonical IRs, compiler, runtime, and conformance tests implement these contracts consistently.
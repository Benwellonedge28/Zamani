Zamani Semantic Specification

Path: "grammar/spec/semantics.md"
Language: Zamani
Semantic specification version: 2.0
Status: Normative production semantic contract
Implementation baseline: Rust 1.97 / Rust 1.97.1
Implementation safety: Safe Rust only; "unsafe" Rust is prohibited
Primary semantic principle: target-independent, resource-parametric, capability-driven computation
Scalability principle: semantic scale is not bounded by artificial language constants
Portability principle: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)

---

0. Document Status and Authority

This document defines the normative semantic meaning of Zamani programs.

It is not:

- a lexer specification;
- a parser grammar;
- an AST implementation;
- a hardware manual;
- a backend specification;
- a QEC implementation;
- a scheduler implementation;
- a routing implementation;
- a runtime implementation;
- a vendor API;
- a hardware capability database.

Those responsibilities belong to the corresponding contracts elsewhere in the repository.

This document defines the semantic contract that connects those layers.

The authoritative relationship is:

grammar/specification/language.md
        │
        ├── language identity and overall model
        │
grammar/spec/lexical.md
        │
        ├── tokenization
        │
grammar/spec/syntax.md
        │
        ├── syntactic structure
        │
        ▼
grammar/Zamani.g4
        │
        ▼
src/lexer.rs
        │
        ▼
src/parser.rs
        │
        ▼
src/frontend/ast/
        │
        ▼
Structural validation
        │
        ▼
Name / scope / type / effect / resource / capability analysis
        │
        ▼
Canonical semantic model
        │
        ├────────────── classical semantics
        ├────────────── quantum semantics
        ├────────────── HDL semantics
        ├────────────── hybrid semantics
        ├────────────── AI/data semantics
        ├────────────── distributed semantics
        └────────────── other domain semantics
        │
        ▼
Canonical IRs
        │
        ├── classical IR
        ├── quantum::ir
        └── appropriate hardware/domain IR
        │
        ▼
Verification
        │
        ▼
Optimization
        │
        ▼
Routing / scheduling / resilience / QEC / ZQN
        │
        ▼
HAL / target realization
        │
        ▼
Runtime / execution

No downstream implementation may silently redefine the source-language meaning specified here.

---

1. Normative Terminology

The following terms are normative:

- MUST — mandatory.
- MUST NOT — prohibited.
- SHOULD — recommended unless a documented technical reason exists otherwise.
- SHOULD NOT — normally prohibited unless justified.
- MAY — permitted.
- IMPLEMENTATION-DEFINED — determined by the implementation and documented.
- RESOURCE-DEPENDENT — dependent on available execution or compilation resources.
- TARGET-DEPENDENT — dependent on a selected target.
- OBSERVABLE — potentially visible through the defined program semantics.
- SEMANTICALLY INVALID — violates Zamani language rules.
- UNREPRESENTABLE — a valid Zamani meaning cannot be represented by a selected target under the requested constraints.
- RESOURCE-UNSATISFIABLE — required resources/capabilities cannot be satisfied.
- EXPLICIT FAILURE — failure represented by a defined semantic mechanism.
- UNSPECIFIED — implementation choice whose variation cannot be observed through the defined semantics.
- UNDEFINED BEHAVIOR — behavior not specified by the language.

Zamani MUST minimize the last category.

Ordinary valid programs MUST NOT depend on undefined behavior.

---

2. Semantic Authority

The semantic authority order is:

1. this document;
2. the normative language specification;
3. normative lexical and syntax specifications;
4. the type-system specification;
5. AST invariants;
6. semantic analysis rules;
7. canonical semantic model;
8. canonical IR contracts;
9. verified transformations;
10. target-independent lowering;
11. target-specific realization.

Historical or aspirational documents, including broad design documents, examples, experimental dialects, or legacy grammar descriptions, MUST NOT override this contract.

In particular:

- "grammar/Zamani.g4" defines syntax;
- "grammar/spec/lexical.md" defines lexical meaning;
- "grammar/spec/syntax.md" defines syntactic structure;
- "grammar/spec/type-system.md" defines type-system rules;
- "grammar/spec/compatibility.md" defines compatibility policy;
- "grammar/spec/semantics.md" defines semantic meaning.

"grammar/grammar.md" MUST describe implementation conformance rather than silently becoming another semantic authority.

"grammar/Zamani-Grammar.md" MAY preserve historical, experimental, or aspirational designs, but a feature does not become normative merely because it appears there.

---

3. Fundamental Semantic Model

A Zamani program is a computation expressed independently of a particular physical machine.

The semantic interpretation of a program is conceptually:

Meaning =
    Program
  + Inputs
  + Declared Semantic Configuration
  + Required Capabilities
  + Applicable Effects
  + Resource Requirements
  + Observable Environment

A target realization is:

Realization =
    Meaning
  + Available Resources
  + Available Capabilities
  + Target Constraints
  + Backend Policies

The realization MUST preserve the meaning.

Therefore:

Source meaning ≠ hardware implementation

and:

resource selection ≠ source semantics

---

4. POCO-REAF

Zamani is designed around:

«Program Once, Compile Once, Run Everywhere, Anywhere, Forever.»

POCO-REAF means that a source program SHOULD express portable computational intent without requiring the programmer to rewrite the program for every scale or target.

POCO-REAF does not mean that every program is executable on every physical machine.

For example, a program requiring a quantum capability cannot execute on a target that provides no compatible quantum realization unless a valid alternative semantic realization exists.

Therefore:

portable source
      ↓
semantic requirements
      ↓
capability/resource negotiation
      ↓
target realization

A target MAY reject the program when its requirements cannot be satisfied.

It MUST NOT silently change its meaning merely to make execution possible.

---

5. Semantic Portability

A portable program MUST NOT implicitly depend on:

- a specific CPU;
- a specific GPU;
- a specific QPU;
- a specific FPGA;
- a specific accelerator;
- a specific physical qubit;
- a specific register number;
- a specific memory address;
- a specific number of cores;
- a specific number of threads;
- a specific number of nodes;
- a specific network topology;
- a specific vendor;
- a specific operating system;
- a specific gate set;
- a specific instruction set;
- a specific cache hierarchy;
- a specific tensor width;
- a specific vector width;
- a specific storage device.

Target-dependent behavior is permitted only when explicitly introduced through a target-dependent semantic mechanism.

---

6. Requirements, Capabilities, Constraints, Preferences, Hints, Decisions

Zamani MUST distinguish these concepts.

6.1 Requirement

A requirement states what must be true for the program's requested semantics.

Example:

requires capability("quantum.measurement")

A requirement is semantic.

6.2 Capability

A capability describes something a realization can provide.

Examples:

quantum.measurement
tensor.compute
distributed.execution
persistent.storage
high_precision.arithmetic

Capabilities MUST be represented abstractly.

6.3 Constraint

A constraint restricts valid realizations.

Example:

requires precision >= p

The actual representation is determined later.

6.4 Preference

A preference recommends an implementation strategy but does not alter correctness.

Example:

prefer accelerator("tensor")

A backend MAY ignore a preference when necessary.

6.5 Hint

A hint may help optimization but MUST NOT alter semantic validity.

6.6 Implementation Decision

An implementation decision belongs downstream.

For example:

logical qubit q
        ↓
routing
        ↓
physical qubit 17

The physical assignment is not source-level semantic identity.

---

7. Resource-Parametric Semantics

Every resource-consuming semantic operation is interpreted relative to an abstract resource environment.

Conceptually:

ResourceEnvironment {
    compute_capacity
    memory_capacity
    storage_capacity
    communication_capacity
    quantum_capacity
    accelerator_capacity
    concurrency_capacity
    timing_capacity
    precision_capacity
    energy_capacity
    reliability_capacity
    domain_capabilities
}

This is a conceptual model, not a mandatory Rust structure.

The actual resource model belongs to the compiler/resource-management subsystem.

No fixed language-level constants define the maximum:

qubits
CPUs
cores
threads
GPUs
FPGAs
nodes
memory
storage
tensor dimensions
vector widths
registers
timelines
processes
devices
accelerators

---

8. Semantic Scalability

Zamani's scalability principle is:

«The language must not impose artificial finite machine-size limits.»

"Infinity" means unbounded by language design.

It does not mean that physical resources are infinite.

Therefore:

semantic scalability = not artificially bounded by the language
physical scalability = bounded by available resources

A program may express computation whose resource requirement is determined dynamically or symbolically.

If the requested computation exceeds available resources, the implementation MUST produce an explicit diagnostic or defined runtime resource failure.

It MUST NOT:

- silently truncate;
- silently reduce precision;
- silently reduce qubit count;
- silently reduce tensor dimensions;
- silently drop distributed work;
- silently substitute another algorithm with different semantics.

---

9. Resource Quantity Semantics

Resource quantities MUST be represented without unnecessary source-language limits.

For example:

n qubits

does not imply that "n" must fit into a fixed hardware-specific integer type.

Resource quantities are semantic values.

An implementation MAY impose internal representation limits, but those limits are implementation constraints rather than language semantics.

If an implementation limit is reached, it MUST produce an explicit diagnostic.

The diagnostic SHOULD identify:

- requested quantity;
- representable quantity;
- affected operation;
- source span;
- target/resource context;
- whether another target or execution strategy may satisfy the requirement.

---

10. Values and Computations

A value is semantic data.

A computation transforms semantic state.

Conceptually:

Computation : State → Result

A computation may also:

- produce effects;
- consume resources;
- create resources;
- transfer ownership;
- observe external state;
- perform quantum measurement;
- communicate;
- diverge;
- fail.

The exact implementation is irrelevant unless explicitly observable.

---

11. Observable Semantics

Two implementations are semantically equivalent when no behavior permitted by the language can distinguish them.

Observable behavior may include:

- returned values;
- explicit errors;
- observable effects;
- I/O;
- externally visible state changes;
- communication;
- synchronization;
- quantum measurement outcomes;
- declared nondeterminism;
- timing constraints when timing is explicitly semantic;
- resource failures when resource behavior is explicitly part of the program contract.

Internal implementation details are not observable unless explicitly exposed by Zamani semantics.

---

12. Determinism

Zamani distinguishes:

1. deterministic semantics;
2. explicit nondeterminism;
3. unspecified implementation choices.

12.1 Deterministic computation

For identical:

- source semantics;
- inputs;
- relevant semantic environment;
- explicit configuration;
- external observations;

a deterministic computation MUST produce the same observable result.

12.2 Explicit nondeterminism

Nondeterminism MAY arise from:

- declared randomness;
- quantum measurement;
- concurrency;
- distributed execution;
- external systems;
- explicitly nondeterministic APIs.

Such nondeterminism MUST be semantically represented.

12.3 Implementation freedom

The compiler MAY choose different:

- instruction sequences;
- data layouts;
- schedules;
- gate decompositions;
- physical mappings;
- parallelization strategies;
- memory placements;

provided observable semantics remain valid.

---

13. Evaluation Order

Evaluation order is semantic whenever changing the order can change observable behavior.

Pure independent computations MAY be reordered.

Effects MUST respect their semantic ordering requirements.

The optimizer MUST NOT reorder operations when doing so changes:

- observable effects;
- synchronization;
- ownership;
- resource validity;
- quantum measurement semantics;
- dependencies;
- explicitly specified timing;
- determinism.

---

14. Purity

A pure computation:

- does not mutate externally observable state;
- does not perform undeclared effects;
- does not depend on hidden mutable state;
- produces its result solely from its semantic inputs.

Pure computations MAY be:

- memoized;
- duplicated;
- eliminated;
- reordered;
- parallelized;

when these transformations preserve resource, ownership, termination, and other applicable semantic rules.

---

15. Effects

Effects describe observable behavior beyond pure value transformation.

Possible effect categories include:

- I/O;
- state;
- time;
- randomness;
- filesystem;
- networking;
- process interaction;
- concurrency;
- hardware interaction;
- quantum execution;
- measurement;
- external services;
- persistent storage;
- distributed communication.

The list is extensible.

An effect name MUST identify semantic behavior, not a particular implementation library.

For example:

effect quantum.measurement

does not mean:

call vendor_qpu_api(...)

---

16. Effect Declaration

Declaring an effect does not perform it.

Conceptually:

effect Measurement

declares a semantic effect.

Performing the effect creates an effectful computation.

Handling the effect changes its realization within a defined scope.

---

17. Effect Handlers

An effect handler MUST preserve:

- effect identity;
- value flow;
- failure semantics;
- control-flow semantics;
- ownership;
- resource rules;
- observable ordering.

A handler MAY be implemented as:

- direct calls;
- dispatch;
- state machines;
- continuations;
- static specialization;
- runtime services;
- distributed services.

These are implementation decisions.

---

18. Types

Types describe semantic properties.

A type determines:

- valid operations;
- valid values;
- conversions;
- ownership;
- resource behavior;
- effect compatibility;
- generic constraints;
- domain compatibility.

Types MUST NOT silently encode a hardware representation.

For example:

Int

is a semantic integer type.

It is not automatically:

i32

or:

i64

unless the type contract explicitly states that representation.

---

19. Type Inference

Inference MAY infer omitted information when the constraints determine a unique valid result.

Inference MUST be:

- deterministic;
- target-independent;
- reproducible;
- independent of hash-map iteration;
- independent of backend ordering;
- independent of available hardware.

If inference is ambiguous, the compiler MUST issue a diagnostic.

---

20. Generics

Generic definitions are parameterized computations.

A generic declaration MUST be valid for every parameterization satisfying its declared constraints.

The compiler MAY:

- monomorphize;
- specialize;
- erase;
- box;
- vectorize;
- distribute;
- otherwise lower the generic computation.

Specialization MUST preserve semantic meaning.

---

21. Dependent Semantics

Where dependent types or value-level type relationships are supported, semantic analysis owns:

- well-formedness;
- dependency;
- scope;
- equality;
- constraint solving;
- universe/level rules where applicable;
- compile-time evaluation;
- termination restrictions.

The parser merely recognizes syntax.

The semantic system MUST determine whether the dependency is valid.

---

22. Optional Values

"Optional<T>" represents:

Some(T)
None

"None" MUST NOT be treated as an arbitrary value of "T".

Operations requiring "T" MUST establish that a value exists.

---

23. Result Values

"Result<T, E>" represents:

Ok(T)
Err(E)

Errors MUST NOT silently become ordinary values.

Backends MAY use any representation preserving the distinction.

---

24. Never

"Never" represents a computation that does not produce an ordinary value.

It may represent:

- non-returning failure;
- explicit termination;
- divergent computation where the language explicitly permits divergence;
- other formally specified non-returning control flow.

A "Never" expression may satisfy a control-flow position requiring another type because it produces no value.

---

25. Memory Semantics

Zamani memory semantics are abstract.

The language distinguishes concepts such as:

- value;
- reference;
- owned resource;
- borrowed resource;
- shared resource;
- linear resource;
- affine resource;
- external resource;
- persistent resource;
- remote resource.

A semantic reference does not necessarily mean a native pointer.

A backend MAY implement it as:

- pointer;
- index;
- handle;
- capability;
- table entry;
- distributed identifier;
- runtime object;
- other safe representation.

---

26. Ownership

Where ownership semantics apply, each owned resource has a logical ownership state.

Ownership transitions MUST be checked semantically.

The compiler MUST reject invalid:

- use-after-move;
- ownership duplication;
- destruction;
- aliasing;
- resource reuse;
- ownership transfer.

Ownership MUST remain independent of physical memory placement.

---

27. Borrowing

A borrow creates a semantic access relationship without transferring ownership.

The semantic checker MUST ensure that a borrow cannot outlive or violate the resource's ownership rules.

A backend MAY implement borrowing using:

- references;
- handles;
- indices;
- runtime guards;
- static elimination;
- other safe mechanisms.

---

28. Linear Resources

A linear value MUST be consumed exactly once.

This applies to any resource whose semantic contract declares linearity.

The analysis MUST track linearity through:

- assignment;
- calls;
- branches;
- loops;
- closures;
- pattern matching;
- asynchronous operations;
- effect handlers;
- quantum operations;
- distributed operations.

---

29. Affine Resources

An affine resource MUST be consumed at most once.

Dropping an affine resource is legal when its type contract permits it.

---

30. Resource Lifetime

Resource lifetime is semantic.

A resource MUST NOT become semantically inaccessible while an active operation still depends upon it.

The implementation MAY use:

- static lifetime analysis;
- runtime reference management;
- ownership tracking;
- region management;
- distributed leases;
- target-specific resource handles.

---

31. Control Flow

All control-flow constructs MUST have well-defined semantic successors.

This includes:

- "if";
- loops;
- "match";
- "return";
- "break";
- "continue";
- function calls;
- asynchronous suspension;
- effect handling;
- exception/failure paths where supported;
- quantum/classical control flow.

The semantic analyzer MUST establish valid control-flow before target lowering.

---

32. Branch Semantics

A branch evaluates its condition according to the condition's semantics.

Only the selected branch is semantically executed unless the language explicitly defines speculative, parallel, or transactional execution.

Compiler speculation MUST NOT make an unselected branch's effects observable.

---

33. Loop Semantics

Loops are semantically unbounded unless a program-level bound exists.

The grammar and semantics MUST NOT impose a maximum iteration count.

A backend MAY optimize loops using:

- unrolling;
- vectorization;
- parallelization;
- hardware acceleration;
- distributed execution.

The transformation MUST preserve semantics.

---

34. Termination

Zamani does not assume that every computation terminates.

Termination MAY be:

- guaranteed;
- proven;
- constrained;
- runtime-dependent;
- intentionally divergent.

Where a language feature requires termination, semantic analysis MUST enforce the applicable rule.

Compile-time evaluation MUST have a defined termination/resource policy and MUST NOT be allowed to consume unbounded compiler resources silently.

---

35. Pattern Matching

Pattern matching determines whether a value conforms to a pattern.

Exhaustiveness MUST be checked where the type system requires it.

Impossible patterns SHOULD be diagnosed.

Pattern semantics MUST be independent of physical representation.

---

36. Name Resolution

Name resolution establishes the meaning of identifiers.

Resolution MUST be deterministic.

The semantic resolver MUST account for:

- lexical scope;
- module scope;
- imports;
- exports;
- aliases;
- declarations;
- generic parameters;
- pattern bindings;
- local bindings;
- namespace qualification;
- dialect namespaces where applicable.

Ambiguous names MUST produce diagnostics.

---

37. Modules

Modules provide semantic namespaces and dependency boundaries.

Module semantics include:

- identity;
- visibility;
- imports;
- exports;
- aliases;
- dependency resolution;
- version compatibility;
- initialization semantics where applicable.

Module resolution MUST NOT depend on physical deployment topology.

---

38. Initialization

Initialization order MUST be defined where initialization has observable effects.

Pure declarations MAY be reordered.

Effectful initialization MUST respect declared dependency and ordering semantics.

Circular initialization MUST either be valid under an explicit rule or produce a diagnostic.

---

39. Constant and Compile-Time Evaluation

Compile-time evaluation is semantic execution performed before ordinary runtime execution.

It MUST be:

- deterministic where required;
- resource bounded;
- side-effect restricted;
- reproducible;
- target-independent unless explicitly target-aware;
- safe.

Compile-time evaluation MUST NOT silently access arbitrary external state.

A compile-time computation that cannot complete within implementation limits MUST fail explicitly.

---

40. Runtime Effects

Runtime effects MUST be represented through semantic effect contracts.

A runtime service may provide the implementation.

For example:

semantic network.send

may be implemented through:

- local networking;
- remote networking;
- accelerator communication;
- distributed transport.

The semantic identity remains "network.send".

---

41. Errors and Failures

Zamani distinguishes:

- ordinary values;
- "Result" failures;
- effect failures;
- panic/non-returning failure;
- resource failure;
- capability failure;
- cancellation;
- timeout;
- communication failure;
- compile-time errors.

These MUST NOT be silently conflated.

---

42. Cancellation

Cancellation is a semantic event when supported.

A cancellation-aware operation MUST define:

- cancellation observation points;
- resource cleanup;
- ownership consequences;
- effect behavior;
- propagation;
- final state.

Cancellation MUST NOT cause resource leaks or ownership violations.

---

43. Concurrency

Concurrency represents multiple potentially overlapping computations.

The language MUST NOT assume a fixed number of:

- threads;
- cores;
- workers;
- tasks;
- actors;
- processes.

Concurrency semantics are independent of implementation degree of parallelism.

---

44. Parallelism

Parallel execution is semantically valid when operations are independent under the applicable:

- data dependencies;
- ownership rules;
- effects;
- synchronization;
- resource constraints.

A backend MAY execute a parallel computation:

- sequentially;
- concurrently;
- vectorized;
- on multiple processors;
- on accelerators;
- distributed across machines.

If sequential execution and parallel execution are observationally equivalent, either is valid.

---

45. Synchronization

Synchronization operations establish semantic ordering or coordination.

Examples include:

- barriers;
- locks;
- channels;
- futures;
- events;
- joins;
- atomic operations;
- distributed coordination.

The implementation MUST preserve their defined happens-before relationships.

---

46. Distributed Semantics

Distributed computation MUST NOT depend on a fixed node count.

A distributed program describes:

- logical participants;
- communication;
- partitioning;
- replication;
- consistency;
- failure behavior;
- placement requirements;
- resource requirements.

Physical nodes are selected later.

The same semantic program MAY execute on:

1 node
2 nodes
N nodes

when its declared semantics permit that realization.

---

47. Distributed Identity

Logical identities MUST NOT be confused with physical machine identifiers.

For example:

logical worker

is not:

machine 17

unless the source explicitly requests target-dependent placement.

---

48. Communication Semantics

Communication operations MUST define:

- sender;
- receiver;
- message/value;
- ordering;
- delivery semantics;
- failure behavior;
- cancellation;
- ownership transfer where applicable.

The physical transport is target-dependent.

---

49. Data Semantics

Data structures describe semantic values.

They MUST NOT impose artificial machine-size limits.

Collections may be:

- finite;
- dynamically sized;
- lazily evaluated;
- distributed;
- persistent;
- streamed.

A compiler MAY choose an implementation appropriate to available resources.

---

50. Numeric Semantics

Numeric operations MUST distinguish:

- mathematical semantics;
- representation semantics;
- overflow behavior;
- precision;
- rounding;
- exceptional values.

The language MUST NOT silently assume a particular machine width unless the type explicitly specifies one.

---

51. Integer Semantics

Integer types MUST define:

- signedness;
- range;
- overflow behavior;
- conversions;
- comparison;
- arithmetic semantics.

If arbitrary-precision integers are supported, their semantics MUST remain independent of machine word size.

If a fixed-width integer is requested, the width is program semantics rather than a global compiler limitation.

---

52. Floating-Point Semantics

Floating-point types MUST define their semantic model.

Where IEEE-compatible behavior is promised, that promise belongs to the numeric type contract.

Backend transformations MUST preserve required:

- rounding;
- exceptional values;
- NaN behavior;
- signed zero;
- overflow;
- underflow;

to the extent specified by the type.

---

53. Vector, Matrix and Tensor Semantics

Vector, matrix, and tensor dimensions are semantic values.

The language MUST NOT impose arbitrary global maximum dimensions.

For example:

Tensor<T, shape>

does not imply a fixed maximum rank or fixed maximum dimension.

The compiler MAY map a tensor onto:

- CPU;
- GPU;
- TPU-like accelerator;
- FPGA;
- distributed memory;
- quantum-classical hybrid system;
- future accelerator.

---

54. Symbolic Computation

Symbolic expressions represent symbolic mathematical meaning rather than machine instructions.

Symbolic transformations MUST preserve mathematical semantics under the applicable domain assumptions.

The compiler MUST distinguish:

- exact symbolic result;
- approximate numerical result;
- implementation approximation.

---

55. AI and Machine Learning Semantics

AI constructs describe semantic computation.

They MAY include:

- models;
- tensors;
- datasets;
- training;
- inference;
- optimization;
- differentiable computation;
- probabilistic computation;
- agents;
- symbolic reasoning.

The language MUST NOT become dependent on a specific framework.

Frameworks belong to interoperability/backend layers.

---

56. Differentiation

Automatic or symbolic differentiation, where supported, is a semantic transformation.

A differentiation transformation MUST preserve the specified mathematical semantics.

The implementation MAY use:

- forward mode;
- reverse mode;
- symbolic differentiation;
- numerical differentiation where explicitly permitted;
- accelerator execution.

---

57. Probabilistic Semantics

Probability-bearing computations MUST define:

- random variables;
- distributions;
- sampling;
- conditioning;
- independence/dependence;
- reproducibility semantics where seeds are explicit.

A random generator is an effect unless the language explicitly defines it as pure relative to an explicit random state.

---

58. Quantum Semantics

Quantum semantics are first-class Zamani semantics.

The grammar describes quantum syntax.

The semantic layer determines what the syntax means.

The canonical downstream quantum semantic boundary is:

quantum::ir

Zamani MUST NOT introduce a competing frontend-specific quantum IR that duplicates the canonical "quantum::ir".

---

59. Quantum State

A quantum state represents a valid state under the quantum semantic model.

The semantic model MUST NOT assume:

- a fixed number of qubits;
- a fixed register width;
- a fixed basis;
- a fixed physical device;
- a fixed gate set.

---

60. Logical and Physical Quantum Identity

The language MUST distinguish:

logical qubit identity

from:

physical qubit identity

The canonical quantum IR already contains concepts such as "QubitId" and "PhysicalQubitId".

These MUST NOT be duplicated in grammar-level semantic types.

The frontend creates semantic logical operations.

Routing later determines physical realization.

---

61. Quantum Operations

Quantum operations are semantic operations identified by:

- operation identity;
- operands;
- parameters;
- controls/modifiers;
- effects;
- result/value behavior;
- source provenance.

The semantic model MUST NOT be a fixed enumeration of today's gates.

Therefore semantics MUST support:

- standard gates;
- parameterized gates;
- composite operations;
- custom operations;
- dialect operations;
- future operations;
- vendor operations through explicit interoperability mechanisms.

An operation name is semantic data.

A backend determines how that operation is realized.

---

62. Quantum Gate Decomposition

Gate decomposition is a lowering transformation.

It MUST NOT occur during parsing merely because the parser knows a gate name.

For example:

logical operation
      ↓
quantum::ir
      ↓
target-aware decomposition

The decomposition MUST preserve quantum semantics.

---

63. Quantum Controls and Modifiers

Controls, inverses, adjoints, powers, repetitions, and related modifiers are semantic transformations.

The compiler MAY lower them into target-supported operations.

The semantic meaning MUST remain stable.

---

64. Quantum Measurement

Measurement is an observable effect.

Measurement MUST NOT be treated as an ordinary pure function unless the semantic contract explicitly models the relevant observation.

Measurement may:

- produce classical information;
- alter quantum state;
- introduce nondeterminism;
- synchronize classical and quantum computation.

The semantics MUST define these effects before backend lowering.

---

65. Mid-Circuit Measurement

Mid-circuit measurement is valid when supported by the quantum semantic model.

Its results MAY influence classical control flow.

The compiler MUST preserve the dependency:

quantum measurement
        ↓
classical value
        ↓
classical decision
        ↓
quantum operation

---

66. Quantum Reset

Reset is a quantum state transformation.

It MUST be distinguished from:

- deallocation;
- measurement;
- classical assignment.

Its exact physical realization is backend-specific.

---

67. Quantum Channels and Noise

Noise is semantic only when represented explicitly.

The semantic model MAY represent:

- noise channels;
- decoherence;
- stochastic error;
- correlated error;
- fault models.

The grammar does not itself implement a noise model.

---

68. Quantum Error Correction

QEC is a downstream semantic/implementation concern.

The source language MAY express:

- fault-tolerance requirements;
- logical operation intent;
- error-correction requirements;
- reliability constraints.

But:

grammar
≠
QEC implementation

The QEC subsystem owns actual correction and decoding algorithms.

The semantic contract passes the required information downstream.

---

69. ZQN Integration

ZQN remains the semantic layer for the applicable fault/noise model.

Zamani semantics MAY express:

- noise requirements;
- fault tolerance;
- reliability;
- error budgets;
- robustness requirements.

The semantic analyzer MUST NOT duplicate ZQN's internal fault model.

The integration boundary is:

Zamani semantic requirements
        ↓
canonical IR metadata/contracts
        ↓
ZQN

---

70. Routing Integration

Routing owns physical realization of logical operations.

The semantic layer MUST NOT choose physical qubit mappings unless the program explicitly expresses a target-dependent requirement.

Normal flow:

logical quantum operation
        ↓
quantum::ir
        ↓
routing
        ↓
physical realization

---

71. Scheduling Integration

Scheduling owns:

- temporal ordering;
- resource conflicts;
- placement in time;
- duration;
- dependency constraints;
- target-specific timing.

Semantic source code may express timing requirements.

It MUST NOT hard-code today's device schedule.

---

72. Calibration Integration

Calibration is target-specific.

The semantic layer may express a requirement such as:

requires calibrated capability(...)

but calibration data itself belongs to the calibration/HAL subsystem.

---

73. HAL Integration

The Hardware Abstraction Layer owns actual target capabilities and state.

Semantic analysis MUST consume abstract capability information rather than vendor-specific implementation details.

HAL MAY expose:

- available devices;
- supported operations;
- resource quantities;
- timing;
- calibration;
- reliability;
- communication;
- memory;
- accelerator capabilities.

The semantic meaning of the program MUST remain independent of the HAL implementation.

---

74. Hybrid Classical-Quantum Semantics

Hybrid programs combine classical and quantum computation.

The semantic model MUST support:

classical
    ↓
quantum
    ↓
measurement
    ↓
classical
    ↓
quantum

The boundary between domains MUST be explicit in the semantic model.

Classical values MUST NOT be silently interpreted as quantum state.

Quantum measurement MUST NOT silently become ordinary deterministic assignment.

---

75. HDL Semantics

HDL constructs describe hardware intent.

They MAY describe:

- modules;
- ports;
- signals;
- state machines;
- combinational logic;
- sequential logic;
- clocks;
- timing;
- memories;
- pipelines;
- interfaces;
- protocols;
- verification properties;
- synthesis intent.

HDL semantics MUST distinguish:

hardware intent

from:

physical implementation

---

76. HDL Parameterization

Hardware descriptions MUST be parameterizable.

No universal fixed width is permitted unless it is part of the declared program semantics.

For example:

width = W

is semantic.

A compiler-wide rule:

MAX_WIDTH = 1024

is not semantic and MUST NOT exist as a universal language restriction.

---

77. Hardware Semantics

Hardware intent MAY express:

- compute capability;
- memory capability;
- communication requirements;
- timing constraints;
- power constraints;
- thermal constraints;
- reliability;
- accelerator intent.

It MUST NOT silently identify a particular physical device.

---

78. Hardware Placement

Placement is normally downstream.

For example:

logical accelerator
        ↓
resource negotiation
        ↓
target realization

A source program MAY explicitly request placement when portability is intentionally relaxed.

Such constructs MUST be clearly classified as target-dependent.

---

79. Networking Semantics

Network operations represent semantic communication.

They MUST define relevant:

- endpoint identity;
- message;
- ordering;
- reliability;
- timeout;
- cancellation;
- security requirements;
- failure behavior.

Physical IP addresses, ports, routes, and network cards are implementation details unless explicitly exposed through a target-dependent API.

---

80. Security Semantics

Security constructs represent:

- identity;
- authentication;
- authorization;
- confidentiality;
- integrity;
- provenance;
- trust;
- policy;
- secure computation.

The grammar MUST NOT make a particular cryptographic library the semantic authority.

Algorithms MAY be selected by semantic requirement or explicit target policy.

---

81. Cryptographic Semantics

A cryptographic operation MUST define its semantic security contract.

The implementation MAY use different optimized implementations where the declared contract remains satisfied.

A cryptographic algorithm name is semantic only when the program explicitly requires that algorithm.

---

82. Provenance

Semantic transformations SHOULD preserve provenance.

Provenance SHOULD include:

- source location;
- declaration identity;
- transformation identity;
- originating operation;
- relevant semantic version;
- compilation identity where applicable.

Provenance MUST NOT change program meaning.

---

83. Source Spans

Every semantic diagnostic MUST be capable of identifying the affected source region when such a region exists.

The frontend AST already carries source-location concepts.

Semantic transformations MUST retain enough provenance to map diagnostics back to source.

Generated or synthesized constructs SHOULD carry origin information.

---

84. Diagnostics

Diagnostics MUST be deterministic and actionable.

A semantic diagnostic SHOULD include:

code
severity
message
primary span
secondary spans
semantic category
related declaration
required capability/resource where applicable
actual capability/resource where applicable
suggested remediation where applicable

Diagnostics MUST NOT depend on random iteration order.

---

85. Capability Checking

Capability checking asks:

«Can this requested semantic operation be realized under the selected compilation/execution environment?»

Capabilities MUST be abstract.

Examples:

quantum.measurement
quantum.mid_circuit_control
distributed.communication
tensor.acceleration
persistent.storage
hardware.synthesis

Capability absence MUST produce an explicit failure.

---

86. Capability Negotiation

When multiple realizations satisfy a requirement, the implementation MAY select any valid realization.

Selection MUST respect:

- requirements;
- constraints;
- effects;
- correctness;
- declared preferences;
- security;
- determinism requirements.

A preference MUST NOT override a requirement.

---

87. Resource Negotiation

Resource negotiation determines whether a target can satisfy resource requirements.

It MUST distinguish:

required
available
preferred
optional

For example:

requires memory >= M

does not mean:

allocate exactly M bytes

unless exact allocation is explicitly semantic.

---

88. Resource Exhaustion

Resource exhaustion MUST be explicit.

Possible outcomes include:

- compilation failure;
- runtime failure;
- retry;
- recovery;
- degradation where explicitly allowed;
- alternate realization;
- distributed scaling.

The implementation MUST NOT silently alter the requested computation.

---

89. Scaling Semantics

Scaling MAY occur along any supported dimension:

- data size;
- number of operations;
- number of qubits;
- tensor dimensions;
- workers;
- nodes;
- accelerators;
- memory;
- storage;
- communication;
- execution time.

The source program SHOULD remain unchanged when only the available resource scale changes.

---

90. Elastic Execution

Where supported, the runtime MAY dynamically acquire or release resources.

Dynamic scaling MUST preserve semantic behavior.

Resource acquisition is an effect when observable.

---

91. Time Semantics

Time MUST be distinguished from implementation performance.

A source program that does not declare timing requirements MUST NOT acquire semantic meaning from accidental execution speed.

Timing becomes semantic when explicitly required.

Examples include:

- deadlines;
- periods;
- latency bounds;
- synchronization;
- real-time constraints.

---

92. Physical Units

Quantities with physical meaning SHOULD use explicit units.

Semantic units MUST not depend on the host machine's native representation.

Conversions MUST preserve the specified physical meaning.

---

93. Precision Semantics

Precision requirements are semantic when declared.

Examples:

requires precision >= P

or an explicitly specified numeric type.

A backend MAY use higher precision than required.

It MUST NOT silently use insufficient precision when doing so violates the program's declared semantics.

---

94. Approximation

Approximation MUST be explicit.

An approximation MAY be:

- exact within a declared tolerance;
- probabilistic;
- numerical;
- heuristic.

A backend MUST NOT silently replace exact semantics with approximation unless the language contract explicitly permits it.

---

95. Optimization

Optimization is semantics-preserving transformation.

An optimizer MUST preserve:

- values;
- effects;
- ownership;
- resource obligations;
- quantum meaning;
- control flow;
- synchronization;
- error behavior;
- explicit nondeterminism;
- required determinism;
- security properties where semantic.

Optimization MAY:

- remove dead code;
- inline;
- specialize;
- vectorize;
- parallelize;
- fuse;
- tile;
- distribute;
- decompose;
- reorder independent operations.

---

96. Optimization Legality

Every optimization MUST have a semantic proof obligation or equivalent verifier guarantee.

The implementation MUST NOT assume:

"probably equivalent"

is sufficient.

Transformations that cannot establish semantic equivalence MUST NOT be applied as ordinary optimizations.

---

97. Canonical Semantic Model

Between AST and domain-specific IR there MUST be a semantic representation capable of expressing:

- resolved names;
- types;
- effects;
- ownership;
- resources;
- capabilities;
- control flow;
- domain identity;
- provenance;
- semantic constraints.

This model is not a replacement for "quantum::ir".

It is the semantic bridge from frontend meaning to the canonical downstream IR architecture.

---

98. IR Integration

The semantic layer MUST lower into the repository's existing canonical IR boundaries.

For quantum:

AST
 ↓
semantic quantum operation
 ↓
quantum::ir

For classical computation:

AST
 ↓
semantic classical operation
 ↓
canonical classical IR

For HDL/hardware:

AST
 ↓
semantic hardware intent
 ↓
appropriate hardware/domain IR

The grammar MUST NOT create a parallel quantum IR merely to accommodate syntax.

---

99. Generic Operation Semantics

Operations should be semantically represented generically where possible.

Conceptually:

Operation {
    name
    namespace
    operands
    parameters
    results
    attributes
    modifiers
    effects
    capabilities
    source
}

This permits:

- classical operations;
- quantum operations;
- tensor operations;
- hardware operations;
- domain extensions;
- future operations.

The exact Rust structure belongs to the AST/semantic/IR implementation contracts.

---

100. Domain Extensibility

New domains MUST integrate through the existing semantic model.

A new domain MUST define:

1. syntax;
2. AST mapping;
3. semantic rules;
4. types;
5. effects;
6. resources;
7. capabilities;
8. diagnostics;
9. IR mapping;
10. compiler consumers;
11. runtime consumers;
12. compatibility rules;
13. positive tests;
14. negative tests;
15. boundary tests;
16. scalability tests.

A new domain MUST NOT create an isolated semantic universe.

---

101. Dialects

Dialects MAY extend syntax and semantics.

Every dialect MUST declare:

- identity;
- version;
- namespace;
- syntax extensions;
- semantic extensions;
- AST mapping;
- capability requirements;
- IR mapping;
- compatibility;
- feature status.

A dialect MUST NOT silently redefine core Zamani semantics.

---

102. Interoperability

Foreign formats and languages are interoperability boundaries.

Examples include:

- OpenQASM;
- QIR;
- LLVM-related representations;
- HDL formats;
- C/C++;
- Rust;
- Python;
- WebAssembly;
- vendor-specific formats.

Interoperability MUST be represented as translation.

A foreign representation MUST NOT become the canonical Zamani semantic model.

---

103. Foreign Functions

Foreign functions MUST declare sufficient semantic information to allow safe integration.

The semantic system MUST know, where applicable:

- parameter types;
- return types;
- effects;
- ownership;
- resource requirements;
- failure behavior;
- calling convention;
- ABI requirements.

The Rust implementation MUST remain within the safe-Rust requirement.

---

104. Safe Rust Requirement

The Zamani implementation MUST compile without "unsafe" Rust.

This applies to:

- lexer;
- parser;
- AST;
- semantic analysis;
- type checker;
- IR;
- optimizer;
- compiler;
- runtime;
- quantum subsystem;
- QEC;
- ZQN integration;
- routing;
- scheduling;
- HAL;
- tooling;
- LSP;
- package management.

No semantic feature may require an "unsafe" implementation strategy.

If an external dependency exposes unsafe internals internally, that does not authorize Zamani source or Zamani-owned implementation code to use "unsafe".

---

105. Rust Version Baseline

The implementation target is:

Rust 1.97
Rust 1.97.1

Semantic behavior MUST NOT depend on unstable Rust language features unless the repository explicitly establishes a separate experimental policy.

The semantic specification itself remains language-level and MUST NOT be coupled to Rust implementation details.

---

106. Memory Safety

Memory safety MUST be established through safe Rust and semantic ownership/type rules.

The compiler MUST NOT rely on:

- unchecked pointer arithmetic;
- invalid references;
- lifetime violations;
- use-after-free;
- data races;
- unvalidated memory aliasing.

---

107. Concurrency Safety

Safe Rust requirements apply to compiler and runtime concurrency.

The implementation MUST use safe abstractions for:

- synchronization;
- channels;
- task execution;
- shared state;
- cancellation;
- asynchronous execution.

Semantic concurrency rules remain language-level.

---

108. Quantum Resource Safety

Quantum resources MUST be tracked semantically.

This includes:

- logical qubits;
- physical realization metadata;
- quantum registers;
- borrowed quantum resources;
- ownership;
- measurement state;
- reset state;
- operation dependencies.

The frontend MUST NOT invent duplicate "QubitId" or "PhysicalQubitId" concepts when the canonical quantum IR already owns them.

---

109. Quantum Resource Limits

There MUST be no universal:

MAX_QUBITS

in language semantics.

Any resource limit comes from:

- selected target;
- resource environment;
- explicit program constraint;
- implementation capacity.

A source program may request a number of logical qubits determined by program data.

---

110. Hardware Limits

The same rule applies to:

MAX_CPUS
MAX_CORES
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_NODES
MAX_MEMORY
MAX_STORAGE
MAX_ACCELERATORS
MAX_REGISTER_WIDTH
MAX_TENSOR_DIMENSION

Such constants MUST NOT define the language.

---

111. Topology

Topology is an implementation concern unless explicitly declared as semantic intent.

The semantic model may describe requirements such as:

requires connectivity(...)

but the physical topology is discovered by routing/resource analysis.

---

112. Placement

Placement is normally a downstream realization.

A logical resource MAY be mapped to a physical resource.

That mapping MUST NOT alter logical identity.

---

113. Scheduling

Scheduling MUST operate after semantic meaning has been established.

Scheduling MAY consider:

- dependencies;
- resource availability;
- durations;
- calibration;
- routing;
- parallelism;
- reliability;
- energy;
- thermal limits.

Scheduling MUST NOT change program meaning.

---

114. Resilience

Resilience semantics may include:

- retry;
- recovery;
- checkpoint;
- degradation;
- failover;
- quarantine;
- escalation.

The semantic meaning of resilience policies MUST be explicit.

Resilience implementation belongs to runtime/compiler infrastructure.

---

115. Fault Semantics

Faults MUST be distinguished from ordinary program results.

A fault may become:

- an explicit error;
- a recoverable effect;
- a resource failure;
- a capability failure;
- a resilience event.

The exact category MUST follow the applicable contract.

---

116. Deterministic Builds

When reproducible compilation is requested, the implementation MUST make semantic compilation independent of:

- hash iteration order;
- thread scheduling;
- wall-clock time;
- local machine identity;
- random seeds unless declared;
- filesystem ordering;
- network ordering.

Build metadata MUST be separated from program meaning.

---

117. Semantic Hashing and Identity

Where semantic hashes are used, the hash MUST represent canonical semantic content rather than:

- memory addresses;
- process IDs;
- random allocation;
- host-specific paths;
- target-specific incidental details.

Canonicalization rules MUST be deterministic.

---

118. Provenance and Reproducibility

Semantic transformations SHOULD preserve provenance.

A transformation chain SHOULD be reconstructable:

source
 ↓
AST
 ↓
semantic model
 ↓
IR
 ↓
optimized IR
 ↓
target realization

The compiler SHOULD retain sufficient metadata for verification and diagnostics.

---

119. Macros

Macros transform syntax.

Macro expansion MUST occur before the semantic phase that requires the expanded meaning.

Macros MUST NOT bypass:

- type checking;
- ownership checking;
- effect checking;
- resource checking;
- capability checking;
- security checking.

Macro-generated syntax MUST carry source provenance.

---

120. Metaprogramming

Metaprogramming is semantic compile-time computation.

It MUST be:

- bounded;
- deterministic where required;
- safe;
- capability-restricted;
- explicitly typed where applicable.

Metaprogramming MUST NOT create an unrestricted escape from semantic analysis.

---

121. Reflection

Reflection MAY inspect semantic program information when explicitly supported.

Reflection MUST respect:

- visibility;
- privacy;
- security;
- type rules;
- compilation phase boundaries.

Reflection MUST NOT silently reveal secrets or private implementation state.

---

122. Security and Capabilities

Capabilities SHOULD follow least privilege.

A semantic capability grants permission to perform a class of operations.

Capabilities MUST be distinguishable from:

- resources;
- types;
- values;
- implementation handles.

A backend MUST NOT grant more semantic authority than the source program requested unless the runtime security model explicitly requires it.

---

123. Secrets

Secrets MUST be treated as protected semantic values.

The compiler MUST NOT unnecessarily place secret values into:

- diagnostics;
- source maps;
- logs;
- provenance;
- cache keys;
- reproducibility records.

---

124. External State

External state is an effect.

A program depending on external state MUST declare or otherwise semantically account for that dependency.

Compiler transformations MUST NOT assume external state is immutable unless the semantic contract guarantees it.

---

125. I/O

I/O is observable.

The compiler MUST preserve required ordering and failure semantics.

Pure optimization MUST NOT duplicate or eliminate I/O merely because an operation appears computationally redundant.

---

126. Filesystem

Filesystem semantics MUST distinguish:

- logical path;
- physical path;
- storage resource;
- permissions;
- persistence.

A semantic path does not necessarily identify a local filesystem path.

---

127. Time and Clocks

Clock access is an effect.

The implementation MUST distinguish:

- logical time;
- monotonic time;
- wall-clock time;
- physical timing constraints.

A backend MUST NOT substitute one for another when the distinction is semantically observable.

---

128. Randomness

Randomness is semantically distinct from deterministic computation.

An explicit seed MAY make a computation reproducible.

The random source itself MAY remain an effect.

---

129. Quantum Randomness

Quantum measurement may produce nondeterministic results even without classical pseudorandomness.

This MUST be represented as quantum semantic behavior, not ordinary random-number generation.

---

130. Temporal / Multi-Timeline Semantics

Where temporal or multi-timeline constructs are supported, the semantics MUST treat timelines as logical execution histories.

There is no fixed maximum number of timelines.

Forking creates a logically distinct execution history.

Merging MUST define:

- compatibility;
- state reconciliation;
- conflicts;
- provenance;
- observable result.

---

131. Snapshot and Checkpoint Semantics

A snapshot captures the semantic state permitted by the relevant contract.

It MUST NOT imply that the physical implementation can serialize every internal hardware state.

A backend may use:

- memory checkpoints;
- distributed snapshots;
- quantum-specific supported mechanisms;
- recomputation;
- persistent storage.

---

132. Distributed Checkpoints

Distributed checkpoint semantics MUST preserve:

- ownership;
- consistency;
- communication state where required;
- resource identity;
- provenance.

---

133. Nano and Physical-Domain Semantics

If nano/atom/material domains are enabled, they describe semantic models and operations.

They MUST NOT hard-code:

- a finite periodic table implementation;
- a fixed number of particles;
- a fixed physical simulation grid;
- a fixed material database.

Physical realization remains backend/domain specific.

---

134. Domain Composition

A program may combine domains.

Examples:

classical + quantum
classical + HDL
AI + accelerator
distributed + quantum
AI + quantum
data + networking
HDL + software

The semantic model MUST preserve domain boundaries while allowing explicit composition.

---

135. Cross-Domain Values

A value crossing domains MUST undergo explicit semantic adaptation where required.

Examples:

classical value → quantum parameter
quantum measurement → classical value
tensor → accelerator operation
hardware signal → software-visible event
distributed message → local value

Implicit domain conversion MUST NOT silently change meaning.

---

136. Cross-Domain Effects

Cross-domain operations MUST carry their applicable effects.

For example:

quantum measurement

may carry:

quantum
measurement
classical-observation

effects.

---

137. Semantic Contracts for Grammar Files

Every new grammar feature MUST have a predetermined semantic contract.

The contract MUST identify:

grammar rule
AST representation
semantic representation
type behavior
effect behavior
resource behavior
capability requirements
IR mapping
compiler consumers
runtime consumers
diagnostics
positive tests
negative tests
boundary tests
scalability tests
compatibility
hard-coding audit

No grammar feature is production-complete without this information.

---

138. AST Integration Contract

The AST is structural, not target-specific.

A grammar feature MUST map to an existing AST abstraction whenever possible.

For example, generic operations SHOULD map to generic operation structures rather than a growing enumeration such as:

QuantumGate::X
QuantumGate::H
QuantumGate::CNOT
...

unless the AST contract explicitly requires a closed semantic enumeration.

The AST MUST NOT become a second IR.

---

139. Semantic Analysis Pipeline

The semantic pipeline SHOULD conceptually be:

AST
 ↓
source validation
 ↓
module resolution
 ↓
name resolution
 ↓
declaration validation
 ↓
type inference
 ↓
type checking
 ↓
generic constraint solving
 ↓
effect analysis
 ↓
ownership/resource analysis
 ↓
capability analysis
 ↓
domain validation
 ↓
control-flow validation
 ↓
constant evaluation
 ↓
semantic normalization
 ↓
canonical semantic model

Each stage MUST have deterministic inputs and outputs.

---

140. Semantic Phase Independence

A completed semantic feature MUST NOT depend on an implementation detail that a later phase has not yet established.

For example:

The semantic checker MUST NOT require a physical qubit mapping to determine whether:

logical quantum operation

is semantically valid.

The mapping occurs later.

---

141. Backend Independence

Semantic analysis MUST NOT ask:

Which CPU instruction executes this?

when determining the meaning of a source program.

It may ask:

Can the selected realization satisfy this semantic requirement?

That is capability/resource analysis, not source semantics.

---

142. Target-Specific Semantics

Target-specific source constructs MAY exist, but they MUST be explicitly identifiable.

They MUST NOT contaminate portable semantics.

A target-specific construct SHOULD declare:

- target identity;
- version;
- capability;
- resource assumptions;
- compatibility;
- fallback behavior.

---

143. Fallback Semantics

A fallback is valid only when it preserves declared semantics.

Examples:

GPU → CPU
QPU → simulator
distributed → local
accelerator → software implementation

A fallback MUST NOT be used if it changes the declared semantics without explicit permission.

---

144. Simulation

Simulation is a possible realization.

A simulator MUST preserve the semantic contract of the simulated operation to the degree promised by the selected simulation model.

Simulation MUST NOT become the semantic definition of quantum operations.

---

145. Verification

Every canonical IR MUST be verified before optimization/lowering.

Verification MUST establish at least:

- structural validity;
- type validity;
- ownership validity;
- resource validity;
- effect validity;
- domain validity;
- control-flow validity;
- provenance consistency;
- required invariants.

Quantum IR verification additionally owns quantum-specific invariants.

---

146. Semantic Equivalence

Two programs/IRs are semantically equivalent when they have the same observable behavior under the same semantic environment, within the equivalence relation specified for the relevant domain.

For probabilistic and quantum programs, equivalence MUST account for the appropriate distribution/outcome semantics rather than requiring identical physical execution traces.

---

147. Quantum Equivalence

Quantum transformations MUST preserve the relevant quantum semantic relation.

Equivalent circuits MAY differ in:

- gate decomposition;
- operation ordering where commuting;
- physical mapping;
- scheduling;
- pulse implementation;
- error-correction encoding.

The semantic outcome must remain equivalent under the applicable model.

---

148. Resource Equivalence

Changing the amount of available resources MUST NOT change semantics merely because a different implementation is selected.

For example:

small target
large target

may produce different schedules.

They MUST represent the same requested computation.

---

149. Graceful Resource Scaling

When a program is resource-parametric, a backend SHOULD attempt:

1. direct realization;
2. optimized realization;
3. parallel realization;
4. distributed realization;
5. alternate supported implementation;
6. simulation where explicitly permitted;
7. explicit resource failure.

The backend MUST NOT silently change semantics.

---

150. Infinite or Unbounded Data

Where lazy, streaming, symbolic, or potentially infinite structures are supported, the semantics MUST distinguish:

- finite value;
- lazy value;
- infinite computation;
- potentially non-terminating computation.

No parser-level finite bound should be imposed merely because the implementation uses finite memory.

---

151. Streaming Semantics

Streams represent sequences of values produced over time.

A stream's semantics MUST define:

- ordering;
- completion;
- cancellation;
- failure;
- backpressure where applicable.

The physical buffer size is implementation-defined.

---

152. Persistence

Persistent data semantics distinguish logical persistence from a particular storage device.

A persistent value MAY be realized through:

- local storage;
- distributed storage;
- replicated storage;
- remote storage;
- database;
- other durable mechanisms.

---

153. Transactions

Transactions MUST define:

- atomicity;
- visibility;
- consistency requirements;
- commit;
- rollback;
- failure semantics.

Physical transaction mechanisms are implementation details.

---

154. Scheduling and Semantic Time

Scheduling may alter execution order only where semantic dependencies permit.

A scheduler MUST respect:

- dependency graph;
- effects;
- ownership;
- synchronization;
- quantum dependencies;
- timing constraints;
- resource constraints.

---

155. Routing and Semantic Identity

Routing MUST NOT change logical resource identity.

For example:

logical q

remains the same logical resource even if mapped to:

physical q17

and later:

physical q23

The mapping belongs to realization.

---

156. Calibration and Semantic Meaning

Calibration affects realization quality.

Calibration information MUST NOT redefine the meaning of a source-level operation.

A calibration failure MAY produce a capability/resource/runtime failure according to the relevant contract.

---

157. Resilience and Semantic Meaning

Resilience mechanisms MUST preserve semantics.

Retries MUST be safe only when retrying the operation does not violate:

- effects;
- ownership;
- idempotence;
- quantum state;
- external side effects.

The runtime MUST NOT blindly retry arbitrary operations.

---

158. Idempotence

An operation is idempotent only if repeating it has the defined equivalent effect.

The semantic system SHOULD record idempotence when required for:

- retries;
- distributed execution;
- resilience;
- caching;
- optimization.

---

159. Caching

Caching is semantics-preserving only for computations whose results remain valid under the semantic environment.

A cache MUST account for relevant:

- inputs;
- effects;
- external state;
- dependencies;
- semantic version;
- resource/environment assumptions.

---

160. Memoization

Pure deterministic functions are candidates for memoization.

Effectful computations MUST NOT be memoized as if they were pure.

---

161. Security-Preserving Optimization

Optimizations MUST NOT remove semantic security guarantees.

Examples:

- authentication;
- authorization;
- secrecy;
- integrity;
- constant-time requirements when explicitly semantic;
- provenance.

---

162. Diagnostic Stability

Diagnostic categories and codes SHOULD remain stable across compatible versions.

Text may improve, but tooling SHOULD rely on structured diagnostic codes rather than exact message strings.

---

163. Compatibility

A language-version change MUST NOT silently alter the meaning of existing valid source unless explicitly classified as a breaking change.

Compatibility rules belong jointly to:

grammar/spec/compatibility.md
grammar/compatibility/

This document defines the semantic requirement that compatible versions preserve existing meaning.

---

164. Deprecated Semantics

Deprecated constructs MAY remain accepted for compatibility.

Their semantic meaning MUST remain defined while they are supported.

A deprecated feature MUST NOT become silently reinterpreted as another feature.

---

165. Feature Lifecycle

A semantic feature moves through:

proposed
experimental
implemented
validated
stable
deprecated
removed

A feature MUST NOT be marked stable until its:

- syntax;
- AST;
- semantic contract;
- IR integration;
- compiler integration;
- runtime integration;
- diagnostics;
- tests;
- compatibility behavior;

are defined.

---

166. Feature Manifests

Where feature manifests are used, each manifest SHOULD record:

id
name
status
version
syntax
tokens
ast
semantic_rules
types
effects
resources
capabilities
ir
compiler
runtime
diagnostics
tests
compatibility
hard_coding_policy

This enables independent feature completion.

---

167. Hard-Coding Prohibition

The following are prohibited as universal language semantics:

MAX_QUBITS
MAX_CPUS
MAX_CORES
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_NODES
MAX_MEMORY
MAX_STORAGE
MAX_ACCELERATORS
MAX_TIMELINES
MAX_TENSOR_RANK
MAX_VECTOR_WIDTH
MAX_REGISTER_WIDTH

Likewise prohibited are implicit assumptions such as:

q0
q1
q2
gpu0
cpu0
node0

being universally meaningful physical resources.

Such identifiers may exist in explicitly target-specific APIs, but they MUST NOT become universal language assumptions.

---

168. Semantic Constants Versus Implementation Limits

This distinction is mandatory.

Valid:

let n = 1024;

because "1024" is program data.

Potentially valid:

Tensor<f64, [1024, 1024]>

because the dimensions are semantic program parameters.

Invalid as a universal language rule:

MAX_TENSOR_DIMENSION = 1024

when that number is merely an implementation limitation.

---

169. Compiler Limits

An implementation MAY have finite limits because physical computers are finite.

Such limits MUST be:

- documented;
- detected;
- reported;
- distinguished from language rules.

A compiler MUST NOT present an implementation limitation as if it were a Zamani semantic restriction.

---

170. Runtime Limits

The same principle applies to runtime limits.

A runtime may run out of:

- memory;
- compute;
- storage;
- devices;
- network capacity;
- time;
- power.

Such exhaustion MUST produce explicit behavior.

---

171. No Silent Semantic Degradation

The implementation MUST NOT silently change:

precision
qubit count
data size
algorithm
security level
fault tolerance
correctness guarantee
ordering
result type

to fit available resources.

Any degradation must be explicitly permitted by the program's semantics.

---

172. Resource-Aware Compilation

The compiler MAY use resource information to select among equivalent implementations.

For example:

same semantic operation
        ↓
small target → implementation A
large target → implementation B
accelerator → implementation C
distributed → implementation D

All implementations MUST satisfy the same semantic contract.

---

173. Capability-Aware Compilation

Likewise:

capability available
        ↓
optimized implementation

or:

capability absent
        ↓
alternate valid implementation

or:

no valid implementation
        ↓
explicit diagnostic

---

174. Semantic Negotiation

Negotiation MUST never redefine the program.

It selects a valid realization from the set:

ValidRealizations(program, environment)

If the set is empty, execution/compilation MUST fail explicitly.

---

175. Formal Semantic Model

Conceptually, define:

⟦P⟧ρ,σ,κ,ε

where:

- "P" = program;
- "ρ" = resolved environment;
- "σ" = semantic state;
- "κ" = available capabilities/resources;
- "ε" = effects/environment.

The result is a semantic computation:

Result<Value, Failure>

possibly with:

Effects
ResourceTransitions
Observations

The exact mathematical formalization MAY evolve without changing the language architecture.

---

176. Semantic State

Semantic state may include:

- variable bindings;
- ownership;
- resources;
- logical quantum state;
- external references;
- effect state;
- distributed state;
- persistent state;
- temporal state.

Physical machine state is not automatically semantic state.

---

177. State Transitions

A semantic operation transforms state according to its contract.

For example:

state_before
      │
      ▼
semantic operation
      │
      ├── value
      ├── effects
      ├── resource transitions
      └── state_after

---

178. Resource Transitions

Operations may:

- acquire resources;
- release resources;
- consume resources;
- produce resources;
- transfer ownership;
- reserve resources.

The semantic resource transition MUST be preserved even if the backend uses a different implementation.

---

179. Capability Failure

If a capability is required but unavailable, the implementation MUST distinguish:

invalid program

from:

valid program, unavailable realization

This distinction is essential to POCO-REAF.

---

180. Compile-Time Versus Runtime Capability Failure

A capability MAY be known at compile time.

Then compilation SHOULD fail early.

If capability availability is dynamic, the runtime MAY report capability failure.

The semantic meaning remains unchanged.

---

181. Resource Requirements and Program Meaning

A resource requirement is part of semantic intent when explicitly declared.

For example:

requires memory >= M

means that execution on an environment below "M" does not satisfy the requested semantics.

The compiler MAY choose any implementation satisfying the requirement.

---

182. Resource Preferences

Preferences do not make a program invalid when ignored.

For example:

prefer accelerator("tensor")

may be ignored when no suitable accelerator exists.

The implementation SHOULD expose whether the preference was honored when tooling requires this information.

---

183. Resource Hints

Hints are optimization information.

Ignoring a hint MUST NOT change correctness.

---

184. Hardware Co-Design

A Zamani program may contain both software and hardware intent.

The semantic boundary is:

software meaning
+
hardware intent
+
resource/capability constraints

The compiler then determines whether a valid co-designed realization exists.

---

185. Hardware Verification

HDL/hardware semantics MAY include assertions and verification properties.

These properties are semantic contracts.

A synthesis backend MUST NOT discard a required property merely because it cannot implement the property directly.

---

186. Simulation Versus Synthesis

Simulation and synthesis are different realizations.

A source-level hardware meaning MUST remain independent of either.

---

187. Future Hardware

The semantic model MUST remain extensible to future substrates, including:

- new CPU architectures;
- new accelerators;
- new QPU technologies;
- photonic systems;
- neuromorphic systems;
- optical systems;
- molecular/nano systems;
- future distributed substrates.

A new target SHOULD require backend integration rather than language redesign.

---

188. Future Domains

A new domain MUST be expressible through:

syntax
→ AST
→ semantic contract
→ capabilities/resources/effects
→ canonical IR mapping
→ backend

The addition MUST NOT require modifying unrelated domain semantics merely to recognize the new domain.

---

189. Inter-Domain Isolation

A domain MUST NOT accidentally redefine a core construct.

For example:

operation

must retain universal semantic meaning even when used by:

- classical;
- quantum;
- HDL;
- AI;
- data;
- networking.

Domain-specific interpretation is attached through explicit semantic metadata.

---

190. Semantic Metadata

Metadata MAY annotate:

- operations;
- declarations;
- resources;
- capabilities;
- types;
- modules;
- source locations;
- provenance;
- optimization intent.

Metadata MUST NOT silently change ordinary semantics unless its contract explicitly says that it does.

---

191. Attributes

Attributes are semantic only when defined by a normative contract.

Unknown attributes SHOULD produce a diagnostic unless the applicable extension policy permits them.

---

192. Namespaces

Namespaces prevent accidental semantic collisions.

Domain operations SHOULD use qualified identities where necessary:

quantum.measure
network.send
tensor.matmul
hardware.synthesize

A backend MUST NOT infer semantic equivalence merely from matching short names.

---

193. Operation Identity

Operation identity is determined by:

namespace
name
signature
semantic contract
version

not merely textual spelling.

---

194. Operation Parameters

Operation parameters are semantic values.

A parameter MUST be checked for:

- type;
- domain;
- constraints;
- ownership;
- effects;
- resource implications.

---

195. Operation Results

Operation results MUST have declared semantic types.

If an operation has multiple results, result ordering MUST be defined.

---

196. Operation Effects

An operation's effects MUST be known to semantic analysis.

An operation that performs I/O cannot be treated as pure merely because its implementation is a function call.

---

197. Operation Resource Behavior

Operations MAY consume or produce resources.

The semantic resource contract MUST specify:

- required resources;
- consumed resources;
- produced resources;
- borrowed resources;
- optional resources;
- capability requirements.

---

198. Generic Operations

Generic operations MAY be specialized for a target.

Specialization MUST preserve:

- type semantics;
- effects;
- ownership;
- resource behavior;
- domain behavior.

---

199. Optimization Across Domains

Cross-domain optimization is allowed only where the equivalence relation is defined.

For example:

classical preprocessing
+
quantum operation

may be fused only when the fusion preserves both classical and quantum semantics.

---

200. Verification Boundary

Before a semantic program enters canonical IR, it MUST satisfy all applicable semantic invariants.

After IR construction, the IR verifier MUST validate its own stronger invariants.

A backend MUST never receive an invalid canonical IR.

---

201. Semantic Errors

Semantic errors include:

- unresolved name;
- ambiguous name;
- invalid type;
- invalid conversion;
- invalid ownership;
- invalid borrow;
- invalid effect;
- invalid capability;
- unsatisfied required resource;
- invalid quantum operation;
- invalid domain crossing;
- invalid control flow;
- invalid generic constraint;
- invalid dependent relation;
- invalid module visibility.

---

202. Error Recovery

Error recovery is primarily a frontend/tooling concern.

Semantic analysis SHOULD continue after recoverable errors where useful for diagnostics.

However, no invalid semantic result may be passed to code generation as if it were valid.

---

203. Partial Programs

Editor tooling MAY represent incomplete programs.

Incomplete syntax/AST is not equivalent to a valid program.

Tooling may use error-tolerant structures, but production compilation MUST distinguish:

incomplete
invalid
valid

---

204. Deterministic Semantic Analysis

Semantic analysis MUST be deterministic for identical inputs and semantic configuration.

It MUST NOT depend on:

- map iteration order;
- thread scheduling;
- machine-specific hash seeds;
- nondeterministic discovery order.

---

205. Parallel Semantic Analysis

The compiler MAY parallelize semantic analysis.

Parallel analysis MUST produce the same semantic result as valid sequential analysis.

---

206. Caching Semantic Analysis

Semantic results MAY be cached.

Cache keys MUST include every semantic input that can affect the result.

Caches MUST NOT allow stale semantics to masquerade as current semantics.

---

207. Incremental Compilation

Incremental semantic analysis MAY reuse unaffected results.

Dependencies MUST be tracked accurately.

A changed semantic input MUST invalidate all dependent results.

---

208. Reproducibility

A reproducible compilation SHOULD produce semantically equivalent output regardless of:

- host machine;
- compiler process ordering;
- filesystem ordering;
- hash randomization;
- unrelated resource availability.

---

209. Backend Conformance

A backend is conforming only if it preserves the semantic contract.

A backend MAY fail to support a feature.

It MUST report unsupported capability rather than changing the feature's meaning.

---

210. Runtime Conformance

A runtime is conforming only if observable behavior matches the semantic specification.

Performance differences are permitted.

Semantic differences are not.

---

211. Compiler Conformance

The compiler is conforming when:

valid source
→ valid semantic model
→ valid canonical IR

and every transformation preserves the semantic contract.

---

212. Grammar Conformance

The grammar is conforming when syntactic constructs accepted by "Zamani.g4" map to valid AST structures covered by the semantic contract.

Grammar acceptance MUST NOT imply semantic validity.

For example, syntax may be structurally valid while:

- a type is invalid;
- a capability is unavailable;
- a resource requirement is unsatisfiable;
- a name is unresolved.

---

213. Lexer Conformance

The lexer MUST produce the token representation required by the canonical syntax contract.

Lexical recognition MUST NOT invent semantic meaning.

---

214. AST Conformance

Every AST node MUST have a known semantic interpretation or be explicitly classified as syntax-only/tooling-only.

No orphan AST node may exist without an integration contract.

---

215. IR Conformance

Every semantic construct marked as implemented MUST have a defined IR mapping.

A feature MUST NOT be marked production-ready merely because the parser accepts it.

---

216. Runtime Conformance

Every runtime-visible semantic feature MUST have defined runtime behavior or an explicit compilation/lowering path that removes it before runtime.

---

217. Test Requirements

Every semantic feature MUST have:

positive tests
negative tests
boundary tests
scalability tests
compatibility tests
diagnostic tests
determinism tests

Domain features SHOULD additionally have:

resource tests
capability tests
cross-domain tests
IR mapping tests
backend conformance tests

---

218. Scalability Tests

Scalability tests MUST test absence of artificial limits.

Examples:

small resource count
larger resource count
symbolic resource count
dynamic resource count
distributed scale
accelerator scale
quantum scale
tensor scale

The tests MUST NOT establish a maximum merely because the test fixture uses a particular number.

---

219. Boundary Tests

Boundary tests SHOULD cover:

- zero;
- one;
- minimum representable values;
- large values;
- empty collections;
- maximal supported implementation values;
- dynamic values;
- resource exhaustion;
- unavailable capability.

---

220. Negative Tests

Negative tests MUST verify that invalid semantics are rejected.

Examples:

- unresolved names;
- invalid types;
- ownership violation;
- invalid quantum target;
- unavailable capability;
- impossible resource requirement;
- invalid effect usage;
- invalid module import;
- unsafe semantic construct;
- invalid domain crossing.

---

221. Compatibility Tests

Compatibility tests MUST ensure that supported historical source remains semantically stable.

A deprecated syntax MUST continue to map to the documented meaning while supported.

---

222. Hard-Coding Audit

Every semantic feature MUST undergo a hard-coding audit.

The audit MUST inspect for accidental assumptions about:

hardware size
device count
topology
qubit count
CPU count
GPU count
memory size
register width
tensor dimensions
thread count
node count
timeline count

A feature fails the audit if it converts an implementation limit into a language rule.

---

223. Resource Audit

Every resource-consuming semantic operation MUST identify:

resource kind
minimum requirement
optional requirement
consumption
production
ownership
failure mode
scaling behavior

---

224. Capability Audit

Every capability-dependent operation MUST identify:

capability identity
required version where applicable
required semantic behavior
fallback
failure mode

---

225. Cross-Domain Audit

A cross-domain feature MUST specify:

source domain
destination domain
conversion
effects
ownership
resource behavior
IR mapping
failure

---

226. Security Audit

Every feature touching:

- secrets;
- identity;
- external systems;
- networking;
- hardware;
- persistent data;
- cryptography;

MUST undergo a security review.

---

227. Performance Semantics

Performance is normally not semantic.

However, explicitly declared:

- deadlines;
- latency requirements;
- throughput requirements;
- power budgets;
- resource budgets;

may become semantic constraints.

A backend that cannot satisfy such a requirement MUST report failure.

---

228. Energy Semantics

Energy requirements MAY be expressed as constraints.

They MUST NOT be interpreted as exact physical energy consumption unless the target contract provides a defined measurement model.

---

229. Thermal Semantics

Thermal constraints MAY be expressed as resource/target requirements.

Actual thermal management belongs to the target runtime/HAL.

---

230. Reliability Semantics

Reliability requirements MAY be expressed semantically.

Examples:

requires reliability >= R
requires fault_tolerance(...)

Actual implementation belongs to QEC, ZQN, runtime, HAL, and backend systems.

---

231. Quality-of-Service

QoS requirements MAY constrain realization.

QoS MUST be clearly distinguished from correctness.

Failure to satisfy QoS is not automatically permission to alter correctness semantics.

---

232. Scheduling Policies

Scheduling policies MAY be expressed as preferences or requirements.

A preference does not alter correctness.

A requirement may make a target realization invalid.

---

233. Placement Policies

Placement policies MAY be:

- abstract;
- preferred;
- required;
- target-specific.

They MUST be explicitly classified.

---

234. Deployment Semantics

Deployment describes how a semantic program is made executable in an environment.

Deployment MUST NOT redefine program meaning.

It MAY determine:

- packaging;
- placement;
- service topology;
- resource acquisition;
- startup;
- shutdown;
- recovery.

---

235. Execution Lifecycle

Where applicable:

created
initialized
ready
running
paused
recovering
completed
failed
cancelled

must have defined semantics.

---

236. Recovery

Recovery MUST preserve semantic invariants.

A recovery mechanism MUST account for:

- ownership;
- effects;
- state;
- resource allocation;
- external side effects;
- idempotence.

---

237. Retry

Retry MUST NOT be universally assumed safe.

An operation may be retried only when:

- explicitly declared retry-safe;
- transactionally protected;
- idempotent;
- or otherwise proven safe under its semantic contract.

This is particularly important for:

- I/O;
- distributed operations;
- quantum execution;
- hardware control.

---

238. Checkpointing

Checkpointing MUST preserve the semantic state required to resume correctly.

A checkpoint does not necessarily serialize all physical machine state.

---

239. Observability

Tracing, profiling, logging, and debugging are normally non-semantic effects unless explicitly exposed to the program.

Instrumentation MUST NOT change ordinary program meaning.

---

240. Profiling

Profiling MAY alter performance.

It MUST NOT alter semantic results.

---

241. Debugging

Debug-only constructs SHOULD be explicitly classified.

A debugger MUST NOT become an implicit semantic dependency of production programs.

---

242. Logging

Logging is an effect.

A compiler MUST NOT duplicate or remove observable logging merely because it appears redundant.

---

243. Testing Semantics

Testing constructs MUST be distinguishable from ordinary production computation where applicable.

Assertions MAY be semantic if their failure behavior is defined.

---

244. Assertions

An assertion states a condition that must hold.

Failure MUST have a defined semantic outcome.

An optimizer MAY remove a proven assertion only when doing so preserves the defined failure/observable behavior.

---

245. Contracts

Preconditions, postconditions, invariants, and other contracts are semantic when enabled.

They MUST be distinguishable from comments.

---

246. Formal Verification

Formal proofs MAY establish semantic properties.

Proof infrastructure is not itself the semantic authority unless the language explicitly defines proof-carrying constructs.

---

247. Proof Irrelevance

Where proofs are used only to establish correctness, their runtime representation MAY be erased when the language contract permits.

Erasure MUST preserve required runtime behavior.

---

248. Compile-Time Proofs

Compile-time proof checking MUST remain bounded by implementation resources.

Failure to complete proof checking MUST be explicit.

---

249. Semantic Versioning

Semantic changes MUST be versioned.

Changes that alter the meaning of valid programs are breaking changes unless explicitly opt-in.

---

250. Feature Gates

Experimental semantics MUST be explicitly gated.

An experimental feature MUST NOT silently change stable semantics.

---

251. Historical Compatibility

Historical constructs may be preserved through compatibility layers.

Compatibility layers MUST lower to current canonical semantics.

They MUST NOT create a permanent second semantic model.

---

252. Deprecated Quantum Syntax

Legacy quantum syntax MUST lower into the canonical quantum semantic model.

It MUST NOT create legacy quantum IR.

---

253. Deprecated Hardware Syntax

Legacy hardware syntax MUST lower into current hardware/resource/capability semantics.

---

254. Semantic Normalization

Before canonical IR construction, equivalent syntactic forms SHOULD be normalized.

Examples:

syntactic sugar
macro expansion
legacy syntax
equivalent operators

Normalization MUST preserve provenance.

---

255. Canonicalization

Canonicalization MUST be deterministic.

Equivalent semantic constructs SHOULD produce equivalent canonical forms where practical.

---

256. Semantic Identity

A semantic identity MUST NOT depend on:

- memory addresses;
- object allocation order;
- process IDs;
- host paths;
- physical device IDs;
- nondeterministic map order.

---

257. Physical Identity

Physical identity may exist after target selection.

It MUST remain distinct from logical program identity.

---

258. Compiler Architecture Boundary

The semantic specification stops at:

meaning + valid canonical representation

It does not dictate:

- instruction selection algorithms;
- register allocators;
- scheduler algorithms;
- routing algorithms;
- QEC decoders;
- pulse synthesis;
- vendor APIs.

Those systems consume the semantic contract.

---

259. Runtime Architecture Boundary

The runtime owns execution of already-lowered semantic operations.

It MUST NOT redefine the language semantics.

---

260. HAL Boundary

HAL owns target capabilities and state.

It MUST expose enough information for valid realization without forcing the semantic layer to know vendor internals.

---

261. QEC Boundary

QEC owns correction and decoding.

Semantic source constructs express requirements/intent.

---

262. ZQN Boundary

ZQN owns fault/noise semantics.

Semantic source constructs provide requirements and metadata.

---

263. Routing Boundary

Routing owns physical mapping.

Semantic source constructs describe logical operations and requirements.

---

264. Scheduling Boundary

Scheduling owns execution timing and resource ordering.

Semantic source constructs describe applicable constraints.

---

265. Optimization Boundary

Optimization owns representation-preserving transformations.

Semantic analysis establishes the invariants optimization must preserve.

---

266. Resource Manager Boundary

Resource management owns actual resource accounting.

The semantic layer describes resource requirements.

The resource manager determines availability.

---

267. Cancellation Boundary

Cancellation semantics are established by this contract.

The runtime implements cancellation.

The semantic model determines its effect on:

- ownership;
- resources;
- control flow;
- failure.

---

268. Replay and Determinism

Where replay is supported, replay records MUST preserve sufficient semantic information to reproduce the requested behavior.

Replay data MUST NOT contain unnecessary secrets.

Replay identity MUST remain independent of physical memory addresses.

---

269. Serialization

Serialization MUST preserve semantic value identity where the serialized type contract promises it.

Serialization format is an interoperability concern.

---

270. Schema Evolution

Persisted semantic data MUST support versioning when long-term compatibility is required.

A schema change MUST define migration semantics.

---

271. API Stability

Semantic APIs consumed by compiler/runtime components SHOULD have explicit versioning.

A private implementation detail MUST NOT accidentally become a public semantic contract.

---

272. Repository Integration Matrix

The semantic specification integrates with the repository as follows:

Component| Semantic responsibility
"grammar/spec/lexical.md"| token meaning
"grammar/spec/syntax.md"| syntactic structure
"grammar/spec/type-system.md"| type rules
"grammar/spec/compatibility.md"| compatibility
"grammar/Zamani.g4"| syntax composition
"src/lexer.rs"| lexical implementation
"src/parser.rs"| parsing implementation
"src/frontend/ast/"| structural AST
semantic analysis| meaning validation
canonical semantic model| resolved language meaning
"src/quantum/ir/"| canonical quantum semantic/IR boundary
QEC subsystem| quantum error correction
ZQN| fault/noise semantics
routing| physical realization
scheduling| execution ordering/timing
optimization| semantics-preserving transformation
HAL| hardware capability/state
resource management| actual resource accounting
runtime| execution
tests| conformance

---

273. Existing Quantum IR Integration

The existing "src/quantum/ir/" hierarchy is the canonical integration point for quantum semantics.

The semantic layer MUST reuse its established concepts rather than creating:

frontend quantum IR
grammar quantum IR
semantic quantum IR
backend quantum IR

as competing representations.

The intended architecture is:

Zamani syntax
      ↓
frontend AST
      ↓
semantic quantum model
      ↓
src/quantum/ir
      ↓
quantum optimization
      ↓
routing
      ↓
scheduling
      ↓
QEC / ZQN / resilience
      ↓
HAL
      ↓
target

---

274. Existing AST Integration

The AST remains structural and target-neutral.

The semantic system consumes AST information.

The AST MUST NOT become responsible for:

- routing;
- scheduling;
- calibration;
- physical mapping;
- QEC;
- vendor APIs;
- hardware topology.

---

275. "grammar/spec/syntax.md" Integration

Syntax MUST answer:

«Can this source form be parsed?»

Semantics MUST answer:

«What does the parsed structure mean?»

Syntax MUST NOT encode semantic resource limits.

Semantics MUST NOT duplicate grammar productions.

---

276. "grammar/spec/type-system.md" Integration

The type-system specification owns detailed type rules.

This document owns how those types participate in:

- values;
- ownership;
- effects;
- resources;
- capabilities;
- domain integration.

If a conflict exists, the normative type-system contract MUST be reconciled with this semantic contract before implementation is considered production-ready.

---

277. "grammar/spec/compatibility.md" Integration

Compatibility defines version transitions.

This document requires those transitions to preserve semantic meaning unless a breaking change is explicitly declared.

---

278. "grammar/specification/semantics.md" Integration

If "grammar/specification/semantics.md" is retained as a human-facing specification, it MUST summarize or reference this normative contract rather than creating a competing semantic authority.

---

279. "grammar/Zamani-Grammar.md" Integration

"Zamani-Grammar.md" may contain:

- historical syntax;
- proposed syntax;
- experimental syntax;
- broad language ideas.

Only features promoted through the production lifecycle become semantic language features.

---

280. "grammar/grammar.md" Integration

"grammar.md" SHOULD report implementation status:

specified
implemented
partially implemented
experimental
deprecated
unsupported

It MUST NOT silently declare an aspirational feature implemented.

---

281. Domain Grammar Integration

The semantic meaning of:

classical/
quantum/
hybrid/
hdl/
hardware/
distributed/
ai/
data/
networking/
security/

MUST flow through the same semantic principles.

Each domain MUST define its own:

- values;
- operations;
- effects;
- capabilities;
- resources;
- IR mapping.

---

282. Compile Integration

"grammar/compile/" describes compilation intent.

Compilation intent MUST NOT become source-level hardware binding unless explicitly target-specific.

---

283. Execution Integration

"grammar/execution/" describes execution policies.

The runtime implements them.

---

284. Resources Integration

"grammar/resources/" describes:

- requirements;
- capabilities;
- budgets;
- constraints;
- preferences;
- hints;
- scaling.

This semantic specification defines their meaning.

---

285. Hardware Integration

"grammar/hardware/" describes abstract hardware intent.

The HAL and backend resolve actual hardware.

---

286. Interoperability Integration

"grammar/interoperability/" handles external representations.

Translation MUST preserve semantics or explicitly document limitations.

---

287. Dialect Integration

"grammar/dialects/" provides controlled extension.

A dialect MUST identify the semantic contract it adds.

---

288. Validation Integration

"grammar/validation/" MUST verify:

- semantic coverage;
- AST coverage;
- IR coverage;
- scalability;
- hard-coding rules;
- determinism;
- compatibility.

---

289. Test Integration

"grammar/tests/" MUST include semantic conformance suites.

The test hierarchy SHOULD include:

lexical/
syntax/
types/
expressions/
statements/
quantum/
classical/
hdl/
hardware/
hybrid/
distributed/
ai/
data/
networking/
security/
negative/
boundary/
scalability/
determinism/
compatibility/

---

290. Production Completeness Requirement

A semantic feature is not production-ready merely because:

- the grammar accepts it;
- the parser constructs an AST;
- an example exists.

A feature is production-ready only when:

syntax
✓
AST
✓
semantic rules
✓
type rules
✓
effects
✓
resources
✓
capabilities
✓
diagnostics
✓
canonical IR
✓
compiler integration
✓
runtime integration
✓
domain integration
✓
positive tests
✓
negative tests
✓
boundary tests
✓
scalability tests
✓
determinism tests
✓
compatibility tests
✓
hard-coding audit
✓
safe-Rust audit

are complete.

---

291. Independent File Completion Contract

Every grammar/specification file SHOULD be independently completable.

Its header SHOULD document:

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
Grammar Contract
AST Contract
Semantic Contract
IR Integration
Compiler Integration
Runtime Integration
Tooling Integration
Cross-Domain Integration
Diagnostics
Positive Tests
Negative Tests
Boundary Tests
Scalability Tests
Compatibility Tests
Determinism Tests
Hard-Coding Audit
Security Audit
Completion Criteria

This prevents the situation where completing one file requires reopening it after unrelated files change.

---

292. Semantic Feature Completion

A feature is independently complete when all integration contracts are known in advance.

Later implementation work may implement those contracts, but the semantic definition itself MUST NOT depend on an unspecified future representation.

---

293. No Circular Authority

The following circular relationship is prohibited:

grammar defines AST
AST defines grammar

Likewise:

semantic specification defines IR
IR retroactively changes semantic meaning

The intended direction is:

language semantics
        ↓
syntax
        ↓
AST
        ↓
semantic validation
        ↓
IR
        ↓
realization

---

294. Semantic Source of Truth

For semantic meaning, this document is authoritative.

For a conflict:

syntax vs semantics

syntax must be changed or reconciled.

For:

AST vs semantics

AST must be changed or adapted.

For:

IR vs semantics

IR integration must be corrected.

For:

backend vs semantics

backend behavior must be corrected or declared unsupported.

---

295. Implementation Failure Policy

When implementation cannot satisfy the semantic contract, it MUST fail explicitly.

It MUST NOT:

- guess;
- silently truncate;
- silently reinterpret;
- silently substitute;
- silently ignore;
- silently downgrade correctness.

---

296. Unsupported Feature Policy

An unsupported feature MUST be reported as unsupported.

The compiler MUST NOT pretend that syntax acceptance means implementation support.

---

297. Future-Proofing

The semantic model MUST be extensible without requiring changes to the fundamental principles:

target independence
resource parametrization
capability negotiation
explicit effects
safe implementation
canonical IR boundaries
semantic preservation

---

298. Semantic Stability

Once a feature becomes stable, future compiler optimizations MUST preserve its semantic contract.

Implementation changes MUST NOT require source rewrites merely because the target architecture changes.

---

299. POCO-REAF Acceptance Criteria

POCO-REAF is satisfied architecturally when:

1. source meaning is target-independent;
2. resource limits are not hard-coded into the language;
3. capabilities are explicit;
4. target realization occurs downstream;
5. logical and physical resources are distinct;
6. optimization preserves semantics;
7. routing preserves semantics;
8. scheduling preserves semantics;
9. QEC preserves semantics;
10. ZQN preserves semantics;
11. HAL provides realization capabilities;
12. resource failure is explicit;
13. unsupported targets fail explicitly;
14. source programs need not be rewritten merely because resource scale changes;
15. the compiler/runtime implementation uses safe Rust.

---

300. Final Semantic Architecture

The complete Zamani semantic architecture is:

                         ZAMANI SOURCE
                              │
                              ▼
                       Lexical Analysis
                              │
                              ▼
                           Parser
                              │
                              ▼
                      Frontend AST
                              │
                              ▼
                    Structural Validation
                              │
                              ▼
                       Name Resolution
                              │
                              ▼
                      Semantic Analysis
                              │
             ┌────────────────┼────────────────┐
             │                │                │
          Types            Effects          Resources
             │                │                │
             ├────────────────┼────────────────┤
             │                │                │
        Ownership        Capabilities       Domains
             │                │                │
             └────────────────┼────────────────┘
                              │
                              ▼
                  Canonical Semantic Model
                              │
             ┌────────────────┼────────────────┐
             │                │                │
        Classical          Quantum            HDL
             │                │                │
             │          quantum::ir            │
             │                │                │
             └────────────────┼────────────────┘
                              │
                              ▼
                         IR Verification
                              │
                              ▼
                       Optimization
                              │
             ┌────────────────┼────────────────┐
             │                │                │
          Routing          Scheduling       Resilience
             │                │                │
             │              QEC              │
             │                │               │
             └────────────────┼───────────────┘
                              │
                              ▼
                             ZQN
                              │
                              ▼
                             HAL
                              │
                              ▼
                    Target Realization
                              │
        ┌─────────────┬───────┼────────┬────────────┐
        │             │       │        │            │
       CPU           GPU     FPGA     QPU      Distributed
        │             │       │        │            │
        └─────────────┴───────┴────────┴────────────┘
                              │
                              ▼
                           Runtime
                              │
                              ▼
                          Observable
                           Behavior

The central rule is:

«Zamani source code specifies what computation means and what semantic requirements must be satisfied. It does not encode today's machine limits, physical topology, vendor implementation, routing decision, schedule, calibration, QEC implementation, or backend instruction selection.»

Therefore:

Program Once
      ↓
Semantic Meaning
      ↓
Compile Once
      ↓
Resource/Capability-Aware Realization
      ↓
Run Everywhere / Anywhere
      ↓
Without Source Rewriting Merely Because Scale or Hardware Changed

This is the semantic foundation required for Zamani's atom-to-everywhere and POCO-REAF goals.
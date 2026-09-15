Zamani Semantic Specification

Path: "grammar/specification/semantics.md"
Language: Zamani
Specification: Semantic Contract
Status: Normative / Production Target
Semantic Version: 1.0
Compiler Baseline: Rust 1.97 / Rust 1.97.1
Rust Edition: 2021
Rust Safety Policy: Production Zamani implementation MUST use safe Rust and MUST NOT contain Rust "unsafe" code.
Primary Portability Objective: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)
Scalability Objective: From atom to everywhere, bounded by program semantics, representational requirements, explicit policies, and resources actually available to the implementation and target.

---

1. Purpose

This document defines the normative semantic meaning of the Zamani programming language.

It defines what a syntactically valid Zamani program means independently of:

- CPU model;
- CPU count;
- core count;
- thread count;
- GPU model;
- GPU count;
- FPGA model;
- QPU model;
- QPU size;
- physical qubit numbering;
- quantum native gate set;
- classical memory capacity;
- accelerator count;
- distributed node count;
- network topology;
- operating system;
- simulator implementation;
- compiler optimization strategy;
- vendor runtime;
- hardware topology;
- calibration data;
- routing strategy;
- scheduling strategy;
- error-correction implementation;
- ZQN implementation;
- HAL implementation.

This specification establishes the semantic boundary between:

source syntax
    ↓
AST
    ↓
semantic analysis
    ↓
canonical semantic representation
    ↓
canonical IR
    ↓
optimization / lowering
    ↓
target realization

The grammar describes syntax.

The AST describes source structure.

The semantic layer determines meaning and validity.

The canonical IR represents the resulting computation.

Backends determine how that computation is realized.

---

2. Normative Language

The following terms are normative:

- MUST — mandatory.
- MUST NOT — prohibited.
- SHOULD — recommended unless a documented compatibility or correctness reason prevents it.
- SHOULD NOT — normally prohibited unless explicitly justified.
- MAY — permitted.
- IMPLEMENTATION DEFINED — determined by the implementation and documented.
- TARGET DEFINED — determined by target capabilities without changing source semantics.
- RESOURCE DEPENDENT — dependent on resources available to compilation or execution.
- EXPLICIT — represented by the program or an explicit compilation/execution policy.
- IMPLICIT — inferred by semantic rules.
- UNDEFINED — MUST NOT be used for ordinary Zamani program behavior.
- UNREPRESENTABLE — the source meaning exists but a particular target cannot directly represent it.
- UNSATISFIABLE — explicit program requirements cannot be satisfied by the selected resource environment.
- UNSUPPORTED — the implementation does not yet implement a specified construct.

A conforming implementation MUST prefer a precise diagnostic over silently assigning arbitrary meaning.

---

3. Authority and Integration

The semantic specification is one part of a single language authority chain.

The repository MUST maintain the following relationship:

grammar/specification/language.md
              │
              ▼
grammar/specification/lexical.md
              │
              ▼
grammar/specification/syntax.md
              │
              ▼
grammar/specification/semantics.md
              │
              ▼
grammar/Zamani.g4
              │
              ├──────────────► src/lexer.rs
              │
              └──────────────► src/parser.rs
                                  │
                                  ▼
                              src/ast/
                                  │
                                  ▼
                         semantic analysis
                                  │
                                  ▼
                       canonical semantic model
                                  │
                                  ▼
                              IR layer
                                  │
                 ┌────────────────┼────────────────┐
                 ▼                ▼                ▼
             Classical       quantum::ir        HDL/Hardware
                 │                │                │
                 └────────────────┼────────────────┘
                                  ▼
                           optimization
                                  │
                    ┌─────────────┼──────────────┐
                    ▼             ▼              ▼
                 routing       scheduling      resilience
                    │             │              │
                    └─────────────┼──────────────┘
                                  ▼
                                 ZQN
                                  │
                                  ▼
                                 HAL
                                  │
                                  ▼
                           target realization

No layer may silently redefine another layer's semantic authority.

---

4. Relationship to Existing Repository Files

4.1 "grammar/Zamani.g4"

"Zamani.g4" is the canonical ANTLR syntax representation.

It MUST:

- recognize syntax;
- preserve semantic structure;
- expose constructs required by this specification;
- remain target-independent.

It MUST NOT:

- perform semantic type checking;
- perform resource allocation;
- perform quantum routing;
- perform scheduling;
- perform QEC;
- perform ZQN fault analysis;
- perform hardware selection;
- encode physical resource ceilings.

---

4.2 "grammar/grammar.md"

"grammar.md" is an implementation-conformance reference.

It MUST distinguish at minimum:

SPECIFIED
LEXICALLY_IMPLEMENTED
PARSED
AST_IMPLEMENTED
SEMANTICALLY_IMPLEMENTED
IR_IMPLEMENTED
BACKEND_IMPLEMENTED
TESTED
STABLE
EXPERIMENTAL
DEPRECATED

It MUST NOT silently become a second semantic authority.

---

4.3 "grammar/Zamani-Grammar.md"

"Zamani-Grammar.md" may contain:

- historical constructs;
- proposed constructs;
- experimental constructs;
- NIMBUS concepts;
- Universal-Trinity concepts;
- Sankofa concepts;
- MTS concepts;
- nano concepts;
- AI concepts;
- future paradigms.

A construct appearing there is not automatically legal Zamani.

Promotion requires:

design
→ specification
→ lexical contract
→ syntax
→ AST
→ semantics
→ IR
→ implementation
→ tests
→ compatibility
→ stable

---

5. Semantic Ownership

The semantic layer OWNS:

- name resolution;
- scope;
- declaration validity;
- type checking;
- generic constraints;
- value-dependent constraints;
- ownership;
- borrowing;
- linearity;
- affinity;
- effects;
- capability requirements;
- resource requirements;
- resource constraints;
- resource preferences;
- control-flow validity;
- pattern exhaustiveness;
- module visibility;
- overload resolution;
- constant evaluation;
- compile-time evaluation;
- quantum semantic validity;
- hybrid semantic validity;
- HDL semantic validity;
- deterministic-semantics validation;
- explicit nondeterminism;
- temporal validity;
- failure semantics;
- semantic diagnostics.

The semantic layer DOES NOT OWN:

- lexical tokenization;
- parsing;
- physical hardware allocation;
- physical qubit mapping;
- pulse calibration;
- scheduling implementation;
- QEC algorithms;
- ZQN fault models;
- backend instruction selection;
- runtime device control.

---

6. Fundamental Semantic Principle

Zamani programs describe computational intent rather than fixed implementation.

The semantic distinction is:

WHAT
    = program meaning

WHY
    = semantic requirements and guarantees

WHICH CAPABILITY
    = capability requirements

WHICH CONSTRAINT
    = explicit semantic/resource constraints

WHICH PREFERENCE
    = optimization guidance

WHERE
    = target realization

WHEN
    = scheduling

HOW
    = implementation/backend realization

A portable program SHOULD express:

requires capability("quantum.measurement")
requires capability("tensor.compute")
requires memory >= required_memory
prefer accelerator("quantum")
requires reliability >= required_reliability

rather than:

use_qpu_0
use_gpu_2
use_core_7
use_physical_qubit_31
use_memory_bank_4

Physical realization belongs downstream unless explicitly requested by the program.

---

7. POCO-REAF

POCO-REAF means:

Program Once
      ↓
Compile Once
      ↓
Run Everywhere
      ↓
Anywhere
      ↓
Forever

The semantic requirement is not that every physical target can execute every program.

Instead:

«A Zamani program MUST retain the same source-level meaning when the implementation target changes, unless the program explicitly contains target-dependent semantics.»

Therefore:

source semantics
≠
hardware configuration

A program requiring more resources than a target possesses is still a valid program.

The compiler/runtime MUST distinguish:

program invalid

from:

program valid but target infeasible

A target-infeasible program MUST NOT be silently rewritten into a different computation.

---

8. Scalability and the Meaning of "Infinity"

Zamani's scalability goal is semantic unboundedness rather than literal infinite physical allocation.

Therefore:

language scalability
=
absence of artificial language-level resource ceilings

while:

physical scalability
=
available resources + implementation capabilities

A conforming implementation MUST NOT impose artificial language limits such as:

MAX_QUBITS
MAX_CPUS
MAX_CORES
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_QPUS
MAX_NODES
MAX_MEMORY
MAX_STORAGE
MAX_VECTOR_WIDTH
MAX_REGISTER_WIDTH
MAX_TENSOR_RANK
MAX_TIMELINES
MAX_AGENTS
MAX_DEVICES
MAX_OPERATIONS

unless the value is explicitly an implementation/resource/security policy rather than a language semantic limit.

---

9. Constants Are Not Hard-Coding

The no-hard-coding rule does not prohibit program constants.

Valid:

let n = 1024;
allocate n elements;

Here "1024" is program semantics.

Invalid:

MAX_QUBITS = 1024

if it means that Zamani cannot represent a computation requiring more than 1024 qubits.

Likewise:

matrix<1024, 1024>

may be valid program semantics.

The implementation MUST NOT infer:

matrix dimensions <= 1024

as a universal language restriction.

---

10. Requirement, Capability, Constraint, Preference, Hint, Realization

These concepts MUST remain distinct.

10.1 Requirement

A requirement is necessary for semantic execution.

Example:

requires qubits >= n

Failure to satisfy it makes a target infeasible.

---

10.2 Capability

A capability describes something an environment must support.

Example:

requires capability("quantum.mid_circuit_measurement")

Capabilities are semantic properties.

They are not vendor names.

---

10.3 Constraint

A constraint restricts a valid realization.

Example:

requires latency <= budget

---

10.4 Preference

A preference guides implementation but does not define correctness.

Example:

prefer accelerator("quantum")

---

10.5 Hint

A hint provides optimization information.

Example:

hint locality

A hint MUST NOT change observable program meaning.

---

10.6 Realization

A realization chooses concrete resources.

Example:

logical qubit → physical qubit

This belongs to routing/placement/backend infrastructure.

---

11. Semantic Environment

Semantic evaluation is parameterized by an abstract environment.

Conceptually:

SemanticEnvironment {
    bindings
    types
    effects
    capabilities
    resource_requirements
    resource_constraints
    module_environment
    target_independent_policies
    compile_time_context
    execution_model
}

A semantic environment MUST NOT contain hidden assumptions about one specific target.

Target information MAY enter semantic analysis only through explicit capability/resource interfaces.

---

12. Resource Environment

Resource feasibility is represented conceptually by:

ResourceEnvironment {
    compute
    memory
    storage
    quantum
    accelerator
    communication
    concurrency
    timing
    precision
    energy
    reliability
    domain_capabilities
}

This is a semantic abstraction.

It MUST NOT be represented by fixed language constants.

The actual repository resource infrastructure, including "ResourceManager", remains an implementation/runtime concern.

The semantic layer produces requirements and constraints that resource infrastructure evaluates.

---

13. Resource Satisfaction

For every resource requirement:

program requirement
        ↓
semantic requirement
        ↓
resource/capability resolver
        ↓
target environment
        ↓
satisfied / unsatisfied / unknown

The semantic layer MUST NOT directly allocate resources.

The result MUST distinguish:

Satisfied
Unsatisfied
Unknown
Deferred

"Unknown" MUST NOT mean "Accepted".

A compilation mode MAY permit deferred feasibility checking where target information is intentionally unavailable.

---

14. Name Resolution

Every identifier has a semantic binding.

Resolution MUST be:

- deterministic;
- scope-aware;
- module-aware;
- visibility-aware;
- namespace-aware;
- version-aware where required.

An unresolved identifier MUST produce a diagnostic.

An implementation MUST NOT silently create an implicit global symbol.

---

15. Scope

Zamani scopes include, where applicable:

- module scope;
- package scope;
- type scope;
- function scope;
- parameter scope;
- block scope;
- pattern scope;
- loop scope;
- handler scope;
- compile-time scope;
- macro scope;
- generated-code scope;
- domain scope.

Scope nesting is not subject to a language-level maximum.

Implementations MAY have resource/security limits, but those are not semantic limits.

---

16. Shadowing

A nested binding MAY shadow an outer binding where the syntax permits it.

Shadowing MUST:

- affect only the appropriate lexical scope;
- preserve the outer binding for references outside the inner scope;
- remain deterministic.

An implementation MUST NOT resolve identifiers according to hash-map ordering or unrelated compiler traversal order.

---

17. Symbol Identity

A semantic symbol MUST have stable identity independent of its textual name alone.

Conceptually:

SymbolId
ScopeId
ModuleId
DeclarationId
SourceSpan

Textual names are lookup keys, not complete semantic identities.

This is necessary for:

- diagnostics;
- IDE tooling;
- incremental compilation;
- cross-module references;
- macro expansion;
- reproducibility;
- provenance.

---

18. Type Semantics

A type describes semantic properties.

A type MUST NOT be defined solely by the machine representation selected by a backend.

Type categories include:

primitive
compound
generic
function
tuple
array
slice
reference
resource
linear
affine
optional
result
never
unit
quantum
classical
temporal
dependent
effectful
capability
external
domain-specific

Additional types MAY be introduced through controlled language extensions.

---

19. Representation Independence

The same semantic type MAY have different physical representations.

For example:

Reference<T>

may become:

pointer
handle
index
capability
remote identifier
table entry
distributed reference

depending on the target.

The source-level meaning remains unchanged.

---

20. Integer Semantics

Integer semantics MUST distinguish:

- mathematical integer intent;
- fixed-width integer types;
- arbitrary-precision integer types where provided;
- machine-sized integer types.

"int" MUST NOT silently imply that all computations are mathematically unbounded.

Explicit fixed-width types such as:

i8
i16
i32
i64
i128
u8
u16
u32
u64
u128

retain their explicit overflow semantics.

A future or existing arbitrary-precision type MUST be represented semantically rather than implemented by silently changing fixed-width behavior.

---

21. Size and Index Semantics

Operations such as:

len(x)
size(x)
index
count
rank
dimension

MUST use a semantically appropriate size/index type.

The language specification MUST NOT define semantic correctness in terms of one host-specific integer width.

Therefore the current implementation pattern:

len -> i64
sizeof -> i64

MUST NOT be treated as the universal semantic contract.

The implementation MUST use the repository's canonical size/index type once that type is established.

---

22. Type Inference

Type inference MAY infer omitted types.

Inference MUST be:

- deterministic;
- constraint-based;
- target-independent;
- reproducible;
- independent of declaration traversal order.

Inference MUST NOT silently select a type merely because it happens to be convenient for the host machine.

If multiple incompatible solutions remain, compilation MUST produce an ambiguity diagnostic.

---

23. Unknown Types

"Unknown" is an internal analysis state.

It MUST NOT be treated as a permanent semantic type unless Zamani explicitly defines a dynamic type.

Therefore:

Type::Unknown

MUST NOT be used as a general mechanism for accepting semantically incomplete programs.

Unknown means:

analysis has not yet established the type

not:

anything is legal

At the completion of semantic analysis, every required type position MUST be:

resolved
or
explicitly dynamic
or
diagnosed

---

24. Untyped Parameters

A parameter without an explicit type MAY be inferred.

For:

fn f(x) {
    ...
}

the semantic interpretation is:

infer x from constraints

It MUST NOT automatically mean:

x: unrestricted dynamic

If inference cannot produce a valid unique interpretation, compilation MUST fail.

---

25. Generics

Generics may parameterize over:

- types;
- values;
- compile-time properties;
- effects;
- capabilities;
- resources;
- domains.

Generic validity MUST be checked against declared constraints.

Specialization is an implementation transformation.

It MUST preserve generic semantics.

---

26. Dependent Semantics

Dependent constructs may express relationships between types and values.

The parser recognizes syntax.

Semantic analysis validates:

- dependency;
- scope;
- equality;
- constraints;
- well-formedness;
- compile-time evaluation;
- termination requirements;
- representability.

The semantic checker MUST NOT rely on arbitrary runtime execution to establish compile-time truths.

---

27. Ownership

Where ownership semantics apply:

- every owned resource has one logical owner;
- ownership transfer is explicit;
- invalid reuse is rejected;
- destruction occurs according to semantic lifetime;
- copying is allowed only where the type permits it.

Ownership is a semantic concept.

The backend may implement it using:

- moves;
- reference counting;
- handles;
- regions;
- static allocation;
- dynamic allocation;
- distributed ownership.

---

28. Borrowing

Borrowing creates a temporary semantic relationship to an owned value/resource.

A valid borrow MUST obey the applicable aliasing and lifetime rules.

The semantic model MUST distinguish:

shared borrow
exclusive/mutable borrow
ownership
move
copy

A borrow checker implementation may exist in "src/compiler/borrow_checker".

The semantic specification, not an unfinished checker implementation, defines correctness.

---

29. Linear Semantics

A linear resource MUST be consumed exactly once.

This applies to resources such as:

- explicitly linear values;
- quantum resources where the type system marks them linear;
- unique capabilities;
- exclusive handles;
- certain hardware resources;
- transactional resources.

Semantic checking MUST follow linear usage through:

- assignments;
- branches;
- loops;
- function calls;
- returns;
- closures;
- async operations;
- pattern matching;
- effect handlers.

---

30. Affine Semantics

An affine resource MUST be consumed at most once.

It MAY be dropped without use if its type semantics permit dropping.

The checker MUST distinguish affine semantics from linear semantics.

---

31. Branches and Resource Usage

A linear resource cannot be consumed on one control-flow path and reused on another invalid path.

For:

if condition {
    consume(x)
} else {
    ...
}
use(x)

the semantic analyzer MUST determine whether "x" remains valid after the merge.

A simplistic global usage counter is insufficient.

Semantic resource analysis MUST be path-aware.

---

32. Functions

A function has semantic components:

name
parameters
generic parameters
parameter types
return type
effects
resource requirements
capability requirements
body
visibility
calling semantics

The body MUST be checked against its declared contract.

---

33. Return Semantics

Every return path MUST be compatible with the function return type.

If a function is non-returning, its return semantics MUST be represented by "Never" or an equivalent explicitly defined semantic construct.

A compiler MUST NOT infer successful return from unreachable or unvalidated paths.

---

34. Control Flow

Zamani control flow includes, where supported:

- "if";
- loops;
- "for";
- "while";
- "loop";
- "match";
- "return";
- "break";
- "continue";
- structured concurrency;
- dynamic quantum control;
- effect handling.

Semantic analysis MUST construct a valid control-flow graph or equivalent representation.

A single global Boolean such as:

in_loop: bool

is insufficient as the complete semantic model for nested loops, closures, async contexts, or control-flow regions.

The implementation MUST track the appropriate semantic context stack.

---

35. Pattern Matching

Pattern matching MUST validate:

- pattern type;
- bindings;
- ownership;
- exhaustiveness;
- unreachable patterns;
- guards;
- branch result types.

If a match is not exhaustive and the language does not define a fallback, semantic analysis MUST reject it.

---

36. Effects

Effects describe observable computational behavior.

Examples include:

io
state
time
randomness
network
filesystem
hardware
quantum
measurement
concurrency
external
persistent
distributed

An effect is semantic information.

It is not merely a runtime annotation.

---

37. Effect Polymorphism

Functions MAY be generic over effects.

For example:

fn compute<T, E>(...)

may carry an effect constraint.

Effect inference MUST remain deterministic.

A function requiring an effect MUST NOT silently become pure.

---

38. Effect Handlers

An effect handler defines how an effect is interpreted within a scope.

A handler MUST preserve:

- effect identity;
- value flow;
- control flow;
- resource semantics;
- failure semantics.

Backends MAY implement handlers through:

- direct calls;
- state machines;
- continuations;
- runtime dispatch;
- static specialization.

These are implementation choices.

---

39. Failure Semantics

Zamani distinguishes:

ordinary value
Result failure
effect failure
panic/non-returning failure
resource failure
capability failure
compile-time error
target infeasibility
runtime environment failure

These MUST NOT be silently conflated.

---

40. No Ordinary Undefined Behavior

Zamani MUST NOT use C/C++-style undefined behavior as a normal optimization mechanism.

An invalid semantic condition MUST result in one of:

compile-time diagnostic
explicit runtime failure
explicit effect
explicit nondeterminism
explicit resource exhaustion
explicit capability failure

The optimizer MUST NOT exploit an unspecified condition as permission to arbitrarily change observable program behavior.

---

41. Determinism

Zamani distinguishes:

41.1 Deterministic computation

Given identical:

- source;
- semantic inputs;
- declared environment;
- explicit configuration;
- external observations;

the result MUST be identical.

---

41.2 Explicit nondeterminism

Nondeterminism MAY arise from:

- randomness;
- quantum measurement;
- concurrency;
- distributed races where explicitly modeled;
- external systems;
- declared nondeterministic effects.

It MUST be semantically visible.

---

41.3 Implementation freedom

The compiler MAY choose different:

- registers;
- instructions;
- gate decompositions;
- schedules;
- memory placement;
- parallel execution;
- routing;
- backend strategies.

provided semantic observables remain equivalent.

---

42. Observability

Semantic equivalence is determined by the program's declared observable behavior.

Observables may include:

- returned values;
- externally visible output;
- explicitly observable state;
- declared effects;
- measurement outcomes;
- synchronization behavior where semantically specified;
- errors;
- resource guarantees;
- timing constraints when explicitly semantic;
- security properties;
- provenance.

An optimizer MUST preserve all specified observables.

---

43. Compile-Time Evaluation

Compile-time evaluation MAY evaluate expressions required for:

- constants;
- type constraints;
- generic specialization;
- compile-time assertions;
- generated structures;
- resource expressions.

Compile-time evaluation MUST be:

- deterministic unless explicitly modeled otherwise;
- bounded by explicit compiler policy;
- free from unintended target dependence;
- free from hidden environmental dependence.

Compile-time evaluation MUST NOT silently execute arbitrary target operations.

---

44. Modules

Modules establish:

- namespaces;
- declarations;
- imports;
- exports;
- visibility;
- dependency relationships;
- version constraints.

Module resolution MUST be deterministic.

Circular dependencies MUST either be explicitly supported by the language semantics or diagnosed.

---

45. Package and Version Semantics

Package identity MUST be separate from source declaration identity.

Version constraints MUST be evaluated according to the language/package specification.

A dependency change MUST NOT silently change the semantic meaning of an already-resolved package graph without explicit version resolution.

---

46. Concurrency

Concurrency is semantic only where observable behavior depends on it.

Zamani supports conceptual forms such as:

- tasks;
- async computation;
- actors;
- channels;
- parallel regions;
- data parallelism;
- task parallelism;
- pipelines;
- reductions;
- distributed computation.

The language MUST NOT assume a fixed number of workers.

A program may state:

parallel

without meaning:

exactly 8 threads

---

47. Parallel Semantics

Parallel execution MUST preserve declared ordering and synchronization constraints.

Operations that are mathematically independent MAY be parallelized.

Operations with semantic dependencies MUST retain those dependencies.

The compiler MAY execute:

1 worker

or:

many workers

where semantics permit.

---

48. Explicit Ordering

Ordering is semantic when the program declares an ordering dependency.

Examples:

happens-before
depends-on
await
barrier
synchronize
sequence

An optimizer MUST preserve such relationships.

---

49. Memory Semantics

Zamani memory is a semantic model, not a direct description of one machine's address space.

Memory categories MAY include:

- local;
- shared;
- distributed;
- persistent;
- accelerator;
- quantum;
- external;
- transactional.

A backend MAY map these to:

- RAM;
- VRAM;
- cache;
- NUMA memory;
- remote memory;
- storage;
- device memory;
- runtime-managed memory.

No source-level universal capacity limit may be inferred.

---

50. Address Semantics

A source-level address abstraction MUST NOT automatically imply a raw native pointer.

If a low-level address concept exists, it MUST have an explicitly specified semantic domain.

A backend may lower it to a pointer, handle, capability, offset, or other representation.

---

51. Resource Semantics

Resources are values or capabilities whose availability matters to computation.

Examples:

- memory;
- storage;
- compute;
- qubits;
- logical qubits;
- communication;
- accelerators;
- timing budget;
- energy budget;
- reliability;
- precision.

Resource requirements are semantic declarations.

Resource allocation is downstream.

---

52. Resource Quantities

Resource quantities MAY be:

- literals;
- variables;
- symbolic expressions;
- inferred values;
- generic parameters;
- runtime values where supported.

The language MUST NOT require resource quantities to be known at parse time.

---

53. Resource Constraints

A resource constraint describes a valid realization condition.

Examples:

requires memory >= required_memory
requires qubits >= logical_qubits
requires latency <= latency_budget
requires reliability >= minimum_reliability

The compiler/resource subsystem determines satisfiability.

---

54. Capability Semantics

Capabilities describe what an environment can perform.

Examples:

quantum.measurement
quantum.mid_circuit_measurement
quantum.dynamic_control
tensor.compute
distributed.communication
hardware.reconfiguration
secure-computation

Capability identifiers MUST be semantic identifiers.

Vendor-specific identifiers belong to target capability metadata.

---

55. Capability Negotiation

A compilation environment MAY provide a capability set.

Semantic analysis determines:

required capabilities
        ∩
available capabilities

If requirements cannot be satisfied, the implementation MUST report:

- missing capability;
- affected source span;
- affected operation;
- target/context;
- whether a legal fallback exists.

---

56. Hardware Independence

The semantic layer MUST NOT own:

- device IDs;
- physical qubit numbers;
- physical cores;
- GPU indices;
- FPGA locations;
- DMA channels;
- physical memory banks;
- vendor queue IDs.

Those belong to target realization.

---

57. Hardware Intent

Zamani MAY express hardware intent.

For example:

requires capability("gpu.compute")
prefer accelerator("gpu")
requires memory >= n

The semantic meaning is capability/resource intent.

It is not an instruction to select a particular physical device.

---

58. Hardware/Software Co-Design

Zamani may express a single semantic program containing:

software computation
hardware intent
communication
memory requirements
timing constraints
verification properties
accelerator intent

The semantics MUST allow these domains to interact without creating independent languages.

---

59. Classical Computing Semantics

Classical computation includes:

- scalar computation;
- integer computation;
- floating-point computation;
- vector computation;
- matrix computation;
- tensor computation;
- symbolic computation;
- numerical methods;
- scientific computation;
- signal processing;
- optimization;
- control.

Mathematical operations MUST be represented semantically.

The grammar MUST NOT become a list of every library algorithm.

For example:

fft
svd
gradient_descent
cholesky

may be library/intrinsic operations whose semantic contracts are resolved after parsing.

---

60. Numerical Semantics

Numerical operations MUST specify:

- operand domains;
- result domains;
- precision behavior;
- rounding behavior where observable;
- exceptional behavior;
- overflow/underflow behavior;
- determinism requirements.

Backend hardware may use different implementations provided specified numerical semantics are preserved.

---

61. Symbolic Semantics

Symbolic expressions represent symbolic values rather than necessarily executing immediately.

The semantic system MUST distinguish:

symbolic value
compile-time value
runtime value

A symbolic expression may be lowered to:

- runtime computation;
- compile-time computation;
- solver invocation;
- canonical IR;
- deferred computation.

---

62. Tensor Semantics

Tensor shape and rank are semantic properties.

They MUST NOT be bounded by arbitrary language constants.

Shapes may be:

- static;
- dynamic;
- symbolic;
- dependent;
- runtime-known.

Operations MUST validate shape compatibility semantically.

---

63. Quantum Semantic Boundary

Quantum syntax is part of Zamani.

Quantum semantic meaning is represented through the canonical:

quantum::ir

boundary.

The grammar MUST NOT create a second quantum IR.

The AST MAY contain source-oriented quantum structure.

Semantic lowering MUST eventually produce the canonical quantum semantic representation.

---

64. Canonical Quantum Identity

The canonical quantum identity types remain owned by:

quantum::ir::qubit::QubitId
quantum::ir::qubit::PhysicalQubitId

where the repository's canonical modules define them.

No frontend or grammar subsystem may introduce another competing "QubitId".

The historical/incorrect path:

quantum::ir::qubits::QubitId

MUST NOT become a new canonical type.

---

65. Quantum Operation Semantics

Quantum operations MUST be represented generically.

The language MUST NOT define semantic completeness through an exhaustive gate enumeration such as:

H
X
Y
Z
CNOT
...

A quantum operation semantically contains, as applicable:

operation identity
namespace
operands
parameters
results
modifiers
conditions
effects
capabilities
source information

The canonical quantum IR operation model remains the semantic destination.

This is consistent with the repository's existing single-"Operation" boundary.

---

66. Unknown Quantum Operations

A source-level operation name may refer to:

- standard operation;
- user-defined operation;
- imported operation;
- dialect operation;
- backend-provided capability;
- future operation.

Unknown operations MUST NOT be silently accepted as semantically valid unless the language explicitly supports late-bound operations.

If late binding is supported, the operation MUST carry an explicit unresolved/late-bound semantic status.

---

67. Quantum Qubit Semantics

A logical qubit is a semantic resource.

A program may express:

q: Qubit

or a parameterized collection of qubits.

The language MUST NOT define a universal maximum qubit count.

A target with insufficient resources produces resource infeasibility, not a different semantic program.

---

68. Quantum Register Semantics

A quantum register is a logical collection.

Its size MAY be:

- literal;
- symbolic;
- generic;
- runtime determined where supported.

The grammar MUST NOT require fixed register sizes.

---

69. Quantum State Semantics

A quantum state is represented abstractly.

The semantic model MUST NOT assume that a source-level state is stored as:

2^n

machine values.

That is an implementation choice.

A backend may use:

- state vectors;
- stabilizer representations;
- tensor networks;
- decision diagrams;
- physical hardware;
- logical qubits;
- hybrid representations.

---

70. Measurement

Measurement is an explicitly effectful quantum operation.

Measurement MAY introduce nondeterminism.

The semantic model MUST represent:

quantum state
    ↓
measurement
    ↓
classical result

The resulting classical value may participate in later control flow.

---

71. Mid-Circuit Measurement

Mid-circuit measurement is semantically distinct from final observation where the result affects subsequent computation.

The semantic model MUST preserve:

measurement
→ classical result
→ condition
→ subsequent operation

This information MUST survive lowering into "quantum::ir".

---

72. Dynamic Quantum Control

Dynamic quantum programs MAY use:

measurement
→ classical decision
→ quantum operation

The semantic checker MUST validate:

- classical result availability;
- control-flow legality;
- quantum resource validity;
- ordering;
- effects.

---

73. Quantum Reset

Reset changes quantum state semantics.

It MUST be represented explicitly.

Reset is not equivalent to ordinary classical assignment.

---

74. Quantum Control and Adjoint Semantics

If Zamani supports:

control
adjoint
inverse
power
repeat

the semantic system MUST validate whether the operation admits the requested transformation.

A backend may synthesize the resulting operation.

The semantic layer MUST not assume every operation is invertible.

---

75. Quantum Measurement Nondeterminism

Quantum measurement is explicit nondeterminism.

A deterministic compiler MUST NOT replace a measurement with a deterministic result unless a semantic proof establishes that the outcome is fixed.

---

76. Quantum Effects

Quantum operations may carry effects such as:

QuantumState
Measurement
Reset
Noise
ExternalDevice

The effect model MUST distinguish pure symbolic quantum transformations from operations requiring actual quantum execution or observation.

---

77. Quantum Resources

Quantum semantic requirements MAY include:

- number of logical qubits;
- operation capabilities;
- connectivity requirements;
- measurement capability;
- reset capability;
- dynamic control;
- fault tolerance;
- precision;
- coherence requirements;
- reliability requirements.

These are requirements, not physical allocation decisions.

---

78. Logical vs Physical Qubits

The semantic model MUST distinguish:

logical qubit

from:

physical qubit

Logical qubits belong to program semantics.

Physical qubits belong to target realization.

Routing determines mapping.

QEC determines error-correction structures.

The grammar MUST NOT collapse these layers.

---

79. Quantum Routing

Routing owns physical realization of logical quantum operations.

Routing MAY:

- insert swaps;
- map logical to physical qubits;
- choose paths;
- use target connectivity;
- transform equivalent implementations.

Routing MUST NOT change the semantic meaning of the source program.

---

80. Quantum Scheduling

Scheduling owns:

- operation ordering;
- timing;
- resource occupancy;
- dependencies;
- ASAP/ALAP decisions;
- dynamic timing;
- delays;
- synchronization.

Scheduling MUST NOT redefine operation semantics.

The scheduler consumes canonical quantum IR.

---

81. Quantum Optimization

Quantum optimization MAY:

- cancel equivalent operations;
- fuse operations;
- decompose operations;
- reorder commuting operations;
- reduce depth;
- reduce resource usage.

Every transformation MUST preserve semantic observables.

---

82. Quantum Error Correction

QEC is NOT a grammar responsibility.

The semantic layer may express requirements such as:

requires fault_tolerance
requires error_correction(...)
requires logical_reliability >= threshold

QEC determines how those requirements are implemented.

QEC MUST NOT create another quantum semantic IR.

---

83. QEC Resource Limits

Existing QEC implementation structures such as "QecLimits" and "ResourceManager" are implementation/resource policy.

They MUST NOT become universal Zamani language limits.

For example:

QecLimits.max_qubits

may constrain one decoder/resource policy.

It MUST NOT imply:

Zamani supports at most max_qubits qubits

---

84. ZQN

ZQN owns quantum fault/noise semantics downstream of the canonical quantum semantic representation.

The semantic layer may express:

noise tolerance
fault model requirements
reliability requirements
error budget

ZQN determines the applicable fault/noise representation.

The grammar MUST NOT duplicate ZQN's internal model.

---

85. HAL

HAL owns target hardware abstraction.

HAL determines:

- available capabilities;
- target state;
- supported operations;
- physical resources;
- calibration state;
- device health;
- target-specific execution interfaces.

The semantic layer produces requirements.

HAL determines whether a target can satisfy them.

---

86. Resilience

Quantum resilience is an orchestration concern.

Semantic intent may specify:

requires reliability
requires fault_tolerance
requires recovery_policy

Resilience may then coordinate:

QEC
ZQN
HAL
routing
scheduling
optimization
runtime

The semantic layer MUST NOT implement retry/recovery algorithms.

---

87. Hybrid Quantum-Classical Semantics

Hybrid computation is one semantic program.

The semantic flow may be:

classical computation
        ↓
quantum computation
        ↓
measurement
        ↓
classical computation
        ↓
quantum computation

The boundary between classical and quantum values MUST be explicit.

Quantum values MUST NOT be silently coerced into classical values except through defined operations such as measurement.

---

88. HDL Semantics

HDL constructs describe hardware intent.

Semantic HDL constructs may include:

- modules;
- ports;
- signals;
- nets;
- registers;
- combinational logic;
- sequential logic;
- clocks;
- reset;
- timing;
- state machines;
- pipelines;
- memories;
- interfaces;
- protocols;
- assertions;
- verification properties.

HDL semantics MUST remain target-independent unless a construct explicitly requests a target-dependent feature.

---

89. Hardware Description vs Hardware Realization

Zamani HDL describes:

what hardware behavior is required

rather than:

which exact FPGA/ASIC/device location must implement it

Synthesis and physical implementation determine:

- placement;
- routing;
- cell selection;
- timing closure;
- physical resources.

---

90. HDL Parameterization

Widths, depths, dimensions, and capacities MAY be parameterized.

The language MUST NOT impose artificial universal values such as:

MAX_SIGNAL_WIDTH = 32
MAX_MEMORY_DEPTH = 1024

unless explicitly defined as an implementation policy.

---

91. AI/ML Semantics

AI constructs may include:

- models;
- tensors;
- datasets;
- training;
- inference;
- differentiation;
- probabilistic computation;
- symbolic reasoning;
- agents;
- model deployment.

The semantic layer defines meaning.

Frameworks such as vendor-specific ML systems MUST NOT become core language semantics.

---

92. Data Semantics

Data constructs may represent:

- records;
- collections;
- streams;
- tables;
- datasets;
- tensors;
- schemas;
- transformations;
- queries;
- pipelines.

Data provenance MAY be semantic where explicitly declared.

---

93. Distributed Semantics

Distributed computation may include:

- processes;
- actors;
- services;
- channels;
- messages;
- replication;
- partitioning;
- consistency;
- transactions;
- collective operations.

The number of nodes MUST NOT be a language-level fixed constant.

---

94. Distributed Identity

A distributed identity MUST remain abstract unless the source explicitly requires a concrete endpoint.

The compiler/runtime may map:

logical process

to:

physical node
container
VM
device
edge resource
cloud resource

---

95. Networking Semantics

Networking constructs describe:

- endpoints;
- protocols;
- communication;
- streams;
- requests;
- responses;
- services;
- routing intent;
- capability requirements.

A semantic endpoint need not be a literal IP address.

Target-specific addresses belong to deployment/runtime configuration unless explicitly part of program semantics.

---

96. Security Semantics

Security properties are semantic requirements.

Examples include:

requires authenticated_channel
requires confidential_data
requires integrity
requires capability("secure-computation")

Cryptographic algorithm selection may be explicit or implementation-defined according to the security contract.

The semantic layer MUST preserve declared security properties.

---

97. Cryptographic Semantics

Cryptographic operations MUST define:

- input domains;
- output domains;
- key relationships;
- security properties;
- failure semantics;
- determinism/randomness requirements.

The language SHOULD prefer semantic cryptographic abstractions over one keyword per algorithm.

---

98. Memory / Sankofa Semantics

Sankofa concepts may include:

- remember;
- recall;
- learning;
- temporal memory;
- history;
- provenance;
- wisdom;
- consensus.

These are semantic operations, not parser-managed runtime state.

A construct such as:

remember(...)

must ultimately map to a defined semantic effect or capability.

The semantic specification MUST NOT treat a placeholder builtin as proof that the feature is implemented.

---

99. Temporal Semantics

If Zamani supports temporal or MTS concepts, the semantic model MUST distinguish:

logical time
physical time
event ordering
observation
speculation
rollback
fork
merge

No fixed number of timelines may be assumed.

No fixed timestamp width may be assumed by the language semantics.

---

100. Metaprogramming

Metaprogramming operates on explicitly defined semantic representations.

It MUST NOT bypass:

- type checking;
- effect checking;
- resource validation;
- security validation;
- ownership validation.

Generated code MUST undergo the same required semantic validation as ordinary source code.

---

101. Macros

Macros may transform syntax.

A macro MUST NOT silently alter semantic rules.

Macro expansion must preserve:

- source provenance;
- hygiene;
- diagnostics;
- scope;
- type/effect/resource validation.

The expanded result MUST be semantically analyzed.

---

102. Dialects

A dialect is an explicit language extension.

Every dialect MUST identify:

name
version
syntax extensions
semantic extensions
AST mapping
IR mapping
capabilities
compatibility
feature gates

A dialect MUST NOT silently redefine core Zamani semantics.

Dialect operations MUST have deterministic semantic meaning.

---

103. Interoperability

Foreign representations may include:

- OpenQASM;
- QIR;
- LLVM-related formats;
- MLIR-related formats;
- HDL formats;
- C/C++;
- Rust;
- Python;
- WASM;
- other explicitly supported formats.

These are interoperability representations.

They MUST NOT replace Zamani's canonical semantic model.

---

104. OpenQASM Integration

OpenQASM syntax MUST be parsed into Zamani's source/semantic structures.

It MUST NOT become a second quantum semantic authority.

The integration is:

OpenQASM source
      ↓
OpenQASM frontend AST
      ↓
Zamani semantic model
      ↓
quantum::ir

---

105. QIR Integration

QIR is a downstream/interoperability representation.

Zamani semantic meaning MUST be established before QIR lowering.

The architecture MUST NOT become:

Zamani
→ QIR semantics

as the primary language definition.

Instead:

Zamani
→ Zamani semantic model
→ quantum::ir
→ QIR/backend interoperability

where appropriate.

---

106. Semantic IR Boundary

The semantic IR is the stable representation between source semantics and target realization.

It MUST:

- preserve observable semantics;
- preserve source provenance;
- preserve relevant effects;
- preserve resource requirements;
- preserve capability requirements;
- preserve quantum meaning;
- preserve control dependencies;
- preserve ownership constraints.

---

107. Canonical Quantum IR Boundary

The quantum domain MUST converge on:

quantum::ir

There MUST NOT be:

frontend quantum IR
scheduler quantum IR
routing quantum IR
QEC quantum IR
ZQN quantum IR

as independent semantic authorities.

Adapters MAY exist for compatibility, but they MUST preserve canonical type identity.

The repository's existing "program/operation.rs" already establishes this principle by re-exporting the canonical operation rather than defining a second "Operation".

---

108. Canonical Operation

The canonical quantum operation represents semantic operation identity.

At minimum, semantic operation information may include:

OperationId
OperationClass
OperationBody
operands
parameters
conditions
effects
source provenance

The exact Rust representation is owned by the canonical IR implementation.

Grammar semantics MUST NOT dictate backend-specific Rust structures.

---

109. IR Verification

Before optimization or target lowering, IR MUST be verified.

Verification includes:

- type validity;
- operation validity;
- resource references;
- dependency validity;
- control-flow validity;
- quantum invariants;
- ownership invariants;
- effect invariants;
- provenance validity.

Invalid IR MUST NOT reach target lowering.

---

110. Optimization Semantics

Optimization is semantics-preserving transformation.

An optimizer MUST preserve:

- values;
- effects;
- failures;
- ownership;
- resource requirements;
- quantum observables;
- measurement behavior;
- declared nondeterminism;
- security properties;
- explicit timing guarantees.

Optimization MAY change implementation strategy.

---

111. Routing Semantics

Routing changes realization.

It MAY alter:

- physical placement;
- communication paths;
- inserted operations;
- resource assignments.

It MUST NOT alter logical semantics.

---

112. Scheduling Semantics

Scheduling assigns execution order/time to a valid computation.

Scheduling MAY depend on:

- target capabilities;
- resource availability;
- timing constraints;
- dependencies;
- calibration;
- reliability;
- communication.

Scheduling MUST NOT become a new source-language semantic authority.

---

113. Compilation Semantics

Compilation is a transformation from one semantic representation to another.

A compiler pass MUST specify:

input representation
output representation
preserved invariants
introduced invariants
failure conditions
provenance behavior
determinism requirements

A pass that cannot prove semantic preservation MUST NOT be considered a production optimization.

---

114. Target Selection

Target selection occurs after target-independent semantic analysis.

The compiler MAY choose:

- CPU;
- GPU;
- FPGA;
- QPU;
- simulator;
- distributed system;
- accelerator;
- future target.

The source program SHOULD remain unchanged.

---

115. Target Infeasibility

If a target cannot satisfy program requirements:

source remains valid
target realization fails

The diagnostic MUST distinguish:

syntax error
semantic error
unsupported compiler feature
missing capability
insufficient resources
unsupported target
runtime failure

---

116. Fallbacks

A compiler MAY use a legal fallback.

Examples:

QPU → simulator
GPU → CPU
accelerator → software implementation
distributed → local execution
native operation → synthesized operation

A fallback is legal only if semantic equivalence is preserved.

A fallback MUST NOT silently weaken explicit correctness/security/resource guarantees.

---

117. Simulation

Simulation is a target realization.

Quantum simulation MUST preserve the requested semantic behavior subject to explicitly documented simulation limitations.

A simulator MUST NOT redefine the quantum language.

---

118. Runtime Semantics

Runtime is responsible for:

- execution;
- resource acquisition;
- external effects;
- device communication;
- runtime scheduling where applicable;
- failures;
- observability;
- lifecycle.

Runtime MUST NOT reinterpret source syntax.

---

119. Runtime Resource Exhaustion

Runtime resource exhaustion is an explicit failure.

Examples:

out of memory
resource unavailable
device unavailable
communication unavailable
execution deadline exceeded

The runtime MUST NOT silently produce a different computation.

---

120. Runtime Nondeterminism

Runtime nondeterminism MUST be explicit or derive from an explicitly nondeterministic semantic effect.

A backend implementation detail MUST NOT accidentally make deterministic source semantics nondeterministic.

---

121. Error Model

Semantic diagnostics MUST include, where available:

diagnostic code
severity
message
primary source span
secondary spans
semantic category
related declaration
suggested correction
provenance

Diagnostics MUST be deterministic.

---

122. Error Categories

At minimum:

LEXICAL
SYNTAX
NAME
TYPE
EFFECT
OWNERSHIP
LINEARITY
CAPABILITY
RESOURCE
CONTROL_FLOW
MODULE
PATTERN
QUANTUM
HYBRID
HDL
SECURITY
COMPATIBILITY
UNSUPPORTED
TARGET
RUNTIME

---

123. Source Spans

Every semantic diagnostic MUST retain source provenance.

Semantic objects SHOULD retain source provenance where practical.

Generated code MUST preserve mapping back to its originating source.

---

124. Provenance

Semantic provenance records may identify:

source file
source span
macro expansion
dialect
module
declaration
transformation
IR origin

Provenance MUST NOT alter semantic behavior.

---

125. Reproducibility

Given identical:

source
dependency graph
compiler version
semantic configuration
explicit inputs

semantic analysis MUST produce the same result.

Target-specific optimization MAY differ after semantic analysis.

---

126. Hashing and Identity

Semantic identities used for reproducibility MUST NOT depend on:

- hash-map iteration order;
- process address;
- random memory address;
- machine-specific pointer identity;
- unstable traversal order.

Canonical serialization/hashing belongs to the appropriate IR/serialization infrastructure.

---

127. Security

Semantic analysis MUST validate security-relevant language constructs before lowering.

Security checks MUST NOT depend solely on backend behavior.

Capability-based operations MUST declare their required capabilities.

Secrets MUST NOT become ordinary diagnostics or provenance text.

---

128. Safe Rust Requirement

The Zamani compiler/runtime/tooling implementation MUST be written using safe Rust.

The following are prohibited in production implementation:

unsafe
unsafe fn
unsafe impl
unsafe trait
unsafe { ... }

The implementation SHOULD enforce this through:

#![forbid(unsafe_code)]

where appropriate.

No semantic requirement in this document requires Rust "unsafe".

---

129. Source-Level "unsafe"

The semantic meaning of a source-language token named "unsafe" is separate from Rust implementation safety.

If Zamani retains "unsafe" for compatibility:

- it MUST have a complete semantic contract;
- it MUST have explicit capabilities;
- it MUST have diagnostics;
- it MUST have security rules;
- it MUST have tests.

Parsing the word "unsafe" MUST NOT itself grant unrestricted behavior.

If no complete semantic model exists, unsupported source-level unsafe constructs MUST be rejected explicitly.

---

130. Resource Security Limits

Implementations MAY impose explicit operational limits for:

- denial-of-service prevention;
- memory safety;
- compiler resource protection;
- recursion depth;
- input size;
- compilation time;
- runtime quotas.

Such limits MUST be:

- implementation policy;
- documented;
- configurable where appropriate;
- distinguishable from language semantics.

They MUST NOT become hidden language ceilings.

---

131. Stack and Recursion Semantics

The language MUST NOT define correctness based on a fixed host stack size.

Recursive programs may fail due to explicit runtime resource exhaustion.

Where recursion depth is semantically bounded, the bound MUST be explicit.

---

132. Collection Semantics

Collections MAY be:

- arrays;
- vectors;
- maps;
- sets;
- streams;
- distributed collections;
- lazy collections.

Their semantic size is not universally bounded.

An implementation may reject a specific allocation because resources are insufficient.

---

133. Stream Semantics

A stream may be:

- finite;
- infinite;
- externally sourced;
- lazily produced;
- distributed.

An infinite semantic stream does not require an infinite physical allocation.

The implementation may process it incrementally.

---

134. Lazy Semantics

Lazy evaluation MAY defer computation.

Deferral MUST preserve observable semantics.

Effects MUST occur according to the language's specified effect/evaluation rules.

---

135. Evaluation Order

Where order is observable, evaluation order MUST be specified.

Where order is explicitly unspecified, the compiler MAY choose an order only if all allowed choices are semantically equivalent.

The implementation MUST NOT accidentally rely on Rust collection ordering.

---

136. Aliasing

Aliasing is semantic where it affects observable behavior.

The type/effect/ownership system MUST identify illegal aliasing.

Backends may implement aliasing differently.

---

137. Conversions

Implicit conversion is permitted only where explicitly defined.

Conversions MUST NOT silently:

- truncate values;
- change quantum meaning;
- discard effects;
- weaken security;
- discard resource requirements.

---

138. Equality

Equality semantics MUST be defined per type category.

Pointer identity, structural equality, semantic equality, and approximate numerical equality MUST NOT be silently conflated.

---

139. Floating-Point Semantics

Floating-point operations MUST document:

- NaN behavior;
- infinity behavior;
- signed zero where relevant;
- rounding;
- overflow;
- underflow;
- comparison.

Optimizers MUST preserve specified floating-point guarantees.

---

140. Approximation

If a language construct permits approximation, the approximation contract MUST be explicit.

Examples:

approximate
tolerance
error_bound
precision

A backend MUST NOT introduce an approximation where exact semantics were requested.

---

141. Probabilistic Semantics

Probabilistic programs MUST distinguish:

random source
distribution
sample
probability
expectation
observation

Randomness is an explicit effect.

---

142. Agent Semantics

AI/agent constructs may define:

- goals;
- policies;
- actions;
- observations;
- memory;
- learning;
- inference.

Agent behavior MUST still obey the core semantic rules for:

- effects;
- resources;
- security;
- determinism;
- failure.

---

143. Scientific Computing

Scientific operations MUST retain units, dimensions, precision, and domain constraints where the language provides those concepts.

The semantic model MUST NOT assume a fixed scientific precision or hardware vector width.

---

144. Unit Semantics

If units are supported:

meter
second
joule
...

must represent semantic dimensions.

Unit conversion MUST be validated semantically.

---

145. Timing Semantics

Timing may be:

non-semantic optimization metadata
semantic constraint
hard requirement

The distinction MUST be explicit.

A "prefer low_latency" hint is not equivalent to:

requires latency <= X

---

146. Energy Semantics

Energy constraints MAY be represented semantically.

A backend may optimize energy consumption.

The semantic model MUST distinguish:

required energy bound
preferred energy bound
estimated energy

---

147. Reliability Semantics

Reliability may be expressed as a requirement or constraint.

Examples:

requires reliability >= R
requires fault_tolerance

Reliability realization belongs to downstream resilience/QEC/ZQN/HAL systems.

---

148. Fault Semantics

Faults must be distinguished from ordinary program errors.

A fault model may describe:

noise
device fault
communication fault
hardware failure
runtime infrastructure failure

ZQN owns domain-specific quantum fault/noise semantics.

---

149. Recovery Semantics

Recovery policies may include:

retry
recover
restart
checkpoint
fallback
escalate
reject

These are execution/resilience semantics when explicitly exposed.

They MUST NOT become hidden behavior of ordinary expressions.

---

150. Cancellation

Cancellation is a semantic effect where supported.

Cancellation MUST define:

- propagation;
- cleanup;
- resource release;
- child-task behavior;
- observable failure.

---

151. Transactions

Transactional constructs MUST define:

- begin;
- commit;
- rollback;
- isolation;
- failure semantics.

Backends may implement transactions differently.

---

152. Persistence

Persistent state is distinct from ordinary memory.

Persistence semantics MUST define:

- visibility;
- lifetime;
- durability guarantees;
- failure behavior.

---

153. External Resources

External resources include:

- files;
- devices;
- network endpoints;
- databases;
- QPUs;
- accelerators.

They are represented as capabilities/resources.

Ownership and lifetime MUST be explicit.

---

154. Device Discovery

Device discovery is a runtime/compilation capability.

Source semantics SHOULD express:

requires capability(...)

rather than requiring a hard-coded device identifier.

---

155. Device Identity

Concrete device identity is target metadata.

If source code explicitly requests a device identity, that construct becomes target-dependent by definition.

Such use MUST be clearly distinguished from portable source semantics.

---

156. Portability Classes

The semantic system SHOULD classify constructs as:

PORTABLE
CAPABILITY_DEPENDENT
RESOURCE_DEPENDENT
TARGET_DEPENDENT
VENDOR_DEPENDENT
EXPERIMENTAL

This classification assists POCO-REAF diagnostics.

---

157. Semantic Compatibility

A language-version change is compatible if old valid source retains the same meaning.

A change is breaking if it changes:

- parsing;
- binding;
- type meaning;
- effect meaning;
- ownership;
- quantum meaning;
- failure meaning;
- observable results.

---

158. Deprecated Semantics

Deprecated constructs MUST retain defined semantics during the compatibility period.

Deprecation MUST NOT mean:

parser accepts it
but semantic meaning is undefined

---

159. Experimental Semantics

Experimental constructs MUST be feature-gated or otherwise explicitly marked.

They MUST NOT silently become stable semantics.

---

160. Semantic Feature Lifecycle

Every semantic feature follows:

PROPOSED
    ↓
DESIGNED
    ↓
SPECIFIED
    ↓
GRAMMAR
    ↓
AST
    ↓
SEMANTIC IMPLEMENTATION
    ↓
IR
    ↓
COMPILER
    ↓
RUNTIME/BACKEND
    ↓
CONFORMANCE TESTS
    ↓
EXPERIMENTAL
    ↓
STABLE

A syntax-only feature is not production-complete.

---

161. Feature Contract

Every major feature MUST have a corresponding contract containing:

Feature ID
Name
Status
Syntax
Tokens
AST mapping
Semantic rules
Type rules
Effect rules
Resource rules
Capability rules
IR mapping
Compiler consumers
Runtime consumers
Diagnostics
Positive tests
Negative tests
Boundary tests
Scalability tests
Determinism tests
Compatibility rules
Hard-coding audit

This allows a feature to be completed independently.

---

162. AST Integration Contract

Every semantic construct MUST have a predetermined AST relationship.

The contract is:

grammar production
        ↓
AST node
        ↓
semantic interpretation
        ↓
canonical representation

Semantic implementation MUST NOT require redesigning the grammar after another unrelated subsystem is completed.

---

163. Generic AST Principle

The AST MUST remain source/domain structural.

It MUST NOT become a second quantum backend IR.

For operations, the preferred semantic direction is conceptually:

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

rather than an exhaustive:

enum QuantumGate {
    H,
    X,
    Y,
    Z,
    CNOT,
    ...
}

---

164. Semantic Model vs AST

The AST preserves what the programmer wrote.

The semantic model preserves what the program means.

Therefore:

AST:
source structure

Semantic model:
resolved meaning

The semantic layer may resolve:

- names;
- overloads;
- types;
- effects;
- resources;
- capabilities;
- domains;
- quantum operation identity.

---

165. Semantic Model vs IR

The semantic model answers:

Is this program valid and what does it mean?

The IR answers:

What canonical computation should downstream infrastructure transform?

The semantic model may therefore contain analysis information that does not survive into the final executable IR.

---

166. Provenance Through Lowering

Semantic provenance MUST be retained through lowering where diagnostics, verification, debugging, or reproducibility require it.

For example:

source span
→ AST node
→ semantic object
→ IR operation
→ backend object

must remain traceable.

---

167. Cross-Domain Semantics

Classical, quantum, HDL, AI, distributed, networking, security, and data constructs share:

- identifiers;
- types;
- expressions;
- declarations;
- effects;
- resources;
- capabilities;
- modules;
- provenance;
- diagnostics.

They MUST NOT each invent independent versions of these foundations.

---

168. Domain Isolation

A domain may add:

domain-specific syntax
domain-specific types
domain-specific effects
domain-specific capabilities
domain-specific semantic rules

but MUST integrate into the universal semantic model.

---

169. Domain Interoperability

Cross-domain values MUST have explicitly defined boundaries.

Examples:

classical → quantum parameter
quantum measurement → classical value
tensor → accelerator operation
HDL signal → software-visible interface
distributed value → local value
AI model → accelerator

Implicit cross-domain conversions MUST NOT be invented.

---

170. Hardware Co-Design Semantic Boundary

The co-design model is:

program intent
      ↓
semantic requirements
      ↓
hardware intent
      ↓
hardware/compiler IR
      ↓
synthesis/lowering
      ↓
physical realization

Software and hardware constructs may share source-level types and contracts without sharing one physical representation.

---

171. Compile Once

POCO-REAF's "Compile Once" requirement means the semantic program should be stable enough to permit retargeting without source rewriting.

The implementation MAY:

- specialize;
- re-optimize;
- re-route;
- reschedule;
- select a different backend;
- use a different runtime.

Those transformations occur after source semantics are established.

---

172. Recompilation and Retargeting

A compiler MAY retain a canonical semantic artifact that can be lowered to multiple targets.

The semantic artifact MUST NOT contain accidental target-specific assumptions.

---

173. Cache Semantics

Cached semantic/IR artifacts MUST be invalidated when any semantic input changes.

Cache identity SHOULD include:

language version
compiler semantic version
source identity
dependency identity
feature configuration
dialect configuration
semantic configuration

Target-specific artifacts MUST be separately identified.

---

174. Incremental Compilation

Incremental semantic analysis MUST preserve the same result as full analysis.

An incremental compiler MUST NOT allow stale symbol/type/effect/resource information to alter program meaning.

---

175. Parallel Semantic Analysis

Semantic analysis MAY be parallelized.

Parallel analysis MUST produce deterministic results.

The result MUST NOT depend on thread scheduling.

---

176. Compiler Diagnostics and Parallelism

Diagnostics MUST be canonically ordered.

The implementation MUST NOT expose arbitrary concurrent traversal order as diagnostic ordering.

---

177. Resource Analysis

Resource analysis determines:

what resources are required

It does not necessarily determine:

where resources will be allocated

Resource estimation MAY be conservative.

If an estimate is conservative, it MUST NOT falsely claim that a valid target is semantically invalid without an explicitly documented approximation policy.

---

178. Resource Scaling

A resource expression may depend on program values.

For example:

memory = f(n)
qubits = g(problem_size)

The language MUST support symbolic/resource-parametric semantics where required.

---

179. Capability Scaling

Capabilities may be:

- required;
- optional;
- preferred;
- fallback-capable.

This enables one semantic program to execute across heterogeneous systems.

---

180. Resource Negotiation

Compilation/runtime may negotiate:

requirements
capabilities
preferences
constraints

The negotiation result MUST NOT change source semantics.

---

181. Semantic Fallback Contracts

A fallback must declare:

original capability
replacement capability
semantic equivalence
performance implications
resource implications
observable differences, if any

If exact equivalence is impossible, the fallback MUST NOT be silently applied.

---

182. No Hidden Hardware Semantics

The compiler MUST NOT infer source semantics from:

current machine
host CPU
host memory
installed GPU
installed QPU
environment variables
device discovery

unless the language explicitly declares an environment-dependent semantic feature.

---

183. Environment-Dependent Programs

If a program explicitly reads environmental state, that dependency is part of its semantic effect.

Examples:

device discovery
clock
random source
environment variable
filesystem
network

Such dependencies MUST be represented as effects/capabilities.

---

184. Security of Environment Access

Environment access MUST be explicit.

The compiler MUST NOT silently inject environment-dependent behavior into otherwise deterministic programs.

---

185. Semantics of "requires"

"requires" establishes a semantic precondition/requirement.

It MUST NOT itself allocate resources.

A requirement MAY be evaluated:

- at compile time;
- at deployment;
- at runtime;

depending on what information is available.

---

186. Semantics of "prefer"

"prefer" supplies an optimization preference.

A compiler MAY ignore it when satisfying stronger requirements.

Ignoring a preference MUST NOT invalidate the program.

---

187. Semantics of "hint"

"hint" supplies optimization information.

Hints MUST NOT change correctness.

An incorrect hint MUST either:

- be ignored safely;
- be diagnosed;
- or be explicitly documented as a semantic assertion.

---

188. Semantics of "assert"

An assertion establishes a runtime or compile-time condition according to its context.

An assertion failure MUST have specified failure semantics.

Assertions MUST NOT be optimized away when their effects are observable.

---

189. Semantic Contracts

Functions, modules, operations, and resources MAY carry contracts.

A contract may state:

preconditions
postconditions
invariants
effects
resources
capabilities

Contract checking may be static, dynamic, or hybrid according to the contract.

---

190. Compile-Time Assertions

Compile-time assertions MUST be decidable from compile-time information.

If a compile-time assertion depends on unavailable target information, it MUST be deferred or diagnosed rather than guessed.

---

191. Runtime Assertions

Runtime assertions evaluate against runtime values.

Failure is an explicit runtime effect.

---

192. Verification

Semantic verification may include:

- type proofs;
- resource proofs;
- ownership proofs;
- effect proofs;
- quantum validity;
- contract validation;
- invariant validation.

Verification MUST NOT depend on undefined behavior.

---

193. Formal Equivalence

Two programs/IR fragments are semantically equivalent only if they have equivalent specified observables.

Textual similarity is insufficient.

IR structural similarity is insufficient.

Backend similarity is irrelevant to source semantic equivalence.

---

194. Optimization Legality

A transformation is legal if:

observable(original)
=
observable(transformed)

under the applicable semantic environment.

For nondeterministic computations, equivalence means preservation of the specified distribution/behavior rather than identical internal random choices.

---

195. Quantum Optimization Legality

Quantum transformations MUST preserve:

- quantum state semantics;
- measurement behavior;
- classical feed-forward;
- declared effects;
- resource requirements where semantically required.

Equivalent circuit transformations are permitted.

---

196. HDL Optimization Legality

HDL transformations MUST preserve specified hardware behavior and timing/verification properties where those are semantic.

---

197. Distributed Optimization Legality

Distributed transformations MUST preserve:

- consistency guarantees;
- ordering guarantees;
- message semantics;
- failure semantics;
- transaction semantics.

---

198. AI Optimization Legality

AI transformations MUST preserve declared model semantics.

Approximation is permitted only when explicitly allowed.

---

199. Security-Preserving Optimization

Optimizers MUST NOT remove or weaken explicit security requirements.

---

200. Resource-Preserving Optimization

An optimizer may reduce resource use.

It MUST NOT exceed hard semantic resource constraints.

It MAY improve preferences.

---

201. Resource Tradeoffs

An optimization may trade:

time
memory
energy
communication
precision
hardware usage

only within explicit semantic constraints.

Preferences guide tradeoffs.

---

202. Semantic Scheduling Constraints

Timing constraints MUST remain distinguishable from scheduling hints.

For example:

requires latency <= L

is stronger than:

prefer low latency

---

203. Physical Calibration

Calibration is not semantic program syntax.

Calibration data is target/runtime state.

A source program may require:

calibration capability

but does not own calibration data.

---

204. Device Health

Device health is target state.

A source program may require:

requires device_reliability >= R

The runtime/HAL determines whether the current target satisfies it.

---

205. Quantum Resilience Decision Semantics

If resilience policy is exposed semantically, decisions may include:

ACCEPT
DEGRADED_ACCEPT
RETRY
RECOVER
ESCALATE
REJECT

The semantic layer defines the meaning of a requested policy.

The resilience subsystem determines execution.

---

206. Cancellation and Recovery

Recovery MUST preserve ownership/resource cleanup.

A failed quantum operation MUST NOT leave the semantic model believing a resource is valid if the runtime has lost it.

---

207. Checkpointing

Checkpoint semantics may preserve a recoverable program state.

A checkpoint MUST contain sufficient semantic information to restore the specified state.

Backend-specific checkpoint formats remain implementation-specific.

---

208. Persistent Provenance

Long-lived computation may retain provenance.

Provenance is metadata unless explicitly made observable.

---

209. Source Compatibility

Adding a new domain MUST NOT silently reinterpret existing source syntax.

If a new keyword conflicts with an existing identifier, compatibility rules MUST specify the migration.

---

210. Keyword Expansion

A concept MUST NOT become a keyword merely because it exists semantically.

Prefer compositional constructs where possible.

This is especially important for:

- quantum gates;
- mathematics;
- AI algorithms;
- vendor operations;
- hardware names;
- protocols;
- cryptographic algorithms.

---

211. Semantic Validation of Builtins

Builtins such as:

print
println
assert
panic
len
sizeof
recall
remember
learn
infer

MUST have explicit semantic contracts.

A builtin appearing in "src/semantic.rs" is not itself proof of complete language support.

Each builtin requires:

type contract
effect contract
failure contract
resource contract
capability contract
IR lowering
runtime behavior
tests

---

212. Placeholder Implementations

Placeholder implementations MUST NOT be treated as normative semantics.

Examples include:

empty engine
no-op verifier
placeholder cognitive engine
unused checker
generic Unknown type

The specification remains authoritative until the implementation is complete.

---

213. Borrow Checker Integration

The semantic implementation may use the repository's borrow checker.

The final architecture MUST make borrow checking part of semantic validation rather than instantiate it without integrating its result.

The checker MUST return structured semantic facts or diagnostics.

Unused analysis MUST NOT be treated as successful validation.

---

214. Symbol Table Integration

The symbol table MUST support:

- nested scopes;
- namespaces;
- modules;
- overload sets;
- generic declarations;
- imported symbols;
- shadowing;
- declaration identity;
- source provenance.

A simple "HashMap<String, Symbol>" may be an implementation component but MUST NOT be assumed to contain the complete semantic model.

---

215. Deterministic Symbol Resolution

If multiple declarations match, overload resolution MUST use explicit language rules.

Hash-map ordering MUST NEVER determine which declaration wins.

---

216. Generic Symbol Resolution

Generic declarations require constraint solving.

The semantic system MUST distinguish:

declared generic
inferred generic parameter
specialized generic
unsatisfied constraint
ambiguous specialization

---

217. Module Resolution

Module imports MUST be resolved before dependent semantic analysis.

Circularity handling must be deterministic.

---

218. Effects and Ownership Across Calls

Function signatures MUST carry relevant:

- effects;
- ownership;
- resource;
- capability

information.

A caller cannot silently ignore a required effect/capability/resource contract.

---

219. Quantum Ownership

Where quantum values are represented as owned/linear resources, ownership semantics MUST prevent illegal duplication.

For example, copying a unique logical qubit identity as an ordinary unrestricted value is invalid if its semantic type prohibits duplication.

---

220. Quantum Classical Boundary

Measurement creates a classical observation.

The semantic system MUST prevent:

classical_value = quantum_state

unless the language explicitly defines such a representation.

The ordinary conversion is:

quantum measurement
→ classical result

---

221. Quantum No-Cloning

If the semantic model represents an unknown quantum state, it MUST NOT permit arbitrary cloning.

This is a semantic restriction, not a backend optimization.

---

222. Quantum Entanglement

Entanglement is a property of quantum state semantics.

The compiler MUST NOT model an entangled state as independent classical values unless a valid transformation establishes equivalence.

---

223. Quantum Measurement Ordering

Measurements that affect later computation MUST preserve ordering.

Independent measurements may be reordered only where semantic equivalence is established.

---

224. Quantum Noise

Noise may be:

- explicit program semantics;
- target property;
- simulation model;
- ZQN model.

These categories MUST remain distinct.

---

225. Fault-Tolerant Semantics

Fault tolerance is a requirement/property.

The semantic model may state:

requires fault_tolerance

QEC/ZQN/routing/scheduling determine implementation.

---

226. Hardware Gate Sets

A source operation need not belong to the target's native gate set.

The backend MAY synthesize it.

Failure occurs only when no valid implementation satisfies the semantic contract.

---

227. Pulse Semantics

Pulse-level constructs may be semantic if Zamani explicitly supports them.

The semantic model may identify:

pulse intent
duration
amplitude
phase
frequency
channel intent

Physical DAC/channel allocation belongs to HAL/backend.

---

228. HDL Timing

HDL timing constraints are semantic when explicitly declared.

Synthesis may optimize implementation while preserving them.

---

229. Distributed Failure Semantics

Distributed programs must distinguish:

node failure
network failure
message loss
timeout
partition
application failure
resource exhaustion

The language may expose these as explicit effects.

---

230. Network Semantics

Network communication is an effect unless explicitly modeled as a pure abstraction.

Communication failure MUST have defined semantics.

---

231. AI Resource Semantics

AI programs may require:

tensor compute
memory
accelerator
precision
training resources
distributed communication

These are requirements/capabilities, not fixed hardware assumptions.

---

232. Data Resource Semantics

Large data does not imply a fixed memory representation.

A backend may use:

- streaming;
- partitioning;
- out-of-core execution;
- distributed storage;
- accelerator memory.

The source semantics remain unchanged.

---

233. Streaming and Backpressure

If streaming is supported, backpressure MUST be semantically defined.

A backend may implement it with:

- queues;
- flow control;
- credits;
- runtime scheduling.

---

234. Security Capability Semantics

Security capabilities may include:

authentication
authorization
confidentiality
integrity
secure execution
zero knowledge
provenance

The compiler MUST NOT silently remove required security properties.

---

235. Capability Leakage

A function that requires a capability MUST NOT silently expose that capability to callers without semantic propagation.

Capabilities behave like semantic effects/resources.

---

236. Effect and Capability Composition

A composed operation inherits the effects/capabilities of its components unless the semantic rules explicitly discharge them.

---

237. Resource Composition

When composing operations, resource requirements MUST compose according to declared semantics.

The compiler MAY optimize shared resource usage.

It MUST NOT undercount required resources.

---

238. Resource Lifetime

Resource requirements may be:

static
dynamic
scoped
temporal
persistent

Lifetime must be explicit where it affects correctness.

---

239. Resource Ownership

A resource may be:

owned
borrowed
shared
leased
external

The semantic layer validates the appropriate lifetime/ownership contract.

---

240. Resource Leasing

If a runtime resource is leased, the lease itself is a semantic effect/resource.

Expiration MUST have explicit failure semantics.

---

241. Capability Revocation

Capabilities may be revoked at runtime where supported.

A revoked capability MUST result in explicit failure or defined recovery.

---

242. Semantic Security Boundary

Semantic validation MUST happen before target-specific operations are trusted.

The backend cannot be the only security validation layer.

---

243. Compilation Profiles

A compilation profile may select:

- optimization policies;
- diagnostics;
- portability policies;
- resource policies;
- feature gates.

A profile MUST NOT silently change language semantics.

---

244. Target Profiles

A target profile describes capabilities/resources.

It MUST NOT redefine core Zamani types or operations.

---

245. Deployment Semantics

Deployment may determine:

- placement;
- replication;
- environment;
- runtime;
- device selection.

Deployment MUST consume semantic requirements rather than rewrite source semantics.

---

246. Runtime Placement

Placement maps logical entities to physical resources.

Examples:

logical process → node
logical qubit → physical qubit
logical tensor → memory/accelerator
logical task → worker

Placement is downstream of semantic analysis.

---

247. No Fixed Topology

The language MUST NOT assume a universal:

ring
mesh
grid
star
fully-connected network

unless topology is explicitly part of a semantic requirement.

---

248. Topology Requirements

A program may require:

connectivity
latency
bandwidth
diameter
routing property

These are resource/capability constraints.

The target resolver determines whether they can be satisfied.

---

249. Semantic Resource Negotiation

Negotiation may select an implementation satisfying:

requirements
constraints
preferences
capabilities

The selected realization MUST preserve semantics.

---

250. Compiler Context

The semantic compiler context SHOULD conceptually contain:

LanguageVersion
FeatureSet
ModuleEnvironment
SymbolEnvironment
TypeEnvironment
EffectEnvironment
CapabilityRequirements
ResourceRequirements
Diagnostics
Provenance
SemanticPolicies

Target hardware details belong in a later target context.

---

251. Separation of Semantic and Target Context

The compiler MUST distinguish:

SemanticContext

from:

TargetContext

The semantic context answers:

what does this mean?

The target context answers:

can this target realize it?

---

252. Target Context

A target context MAY contain:

architecture
capabilities
resources
topology
native operations
timing
calibration
runtime

It MUST NOT mutate source semantics.

---

253. Semantic Caching

Semantic analysis results may be cached.

The cache MUST be invalidated when semantic inputs change.

---

254. Thread Safety

Semantic analysis SHOULD avoid mutable global state.

Parallel semantic passes MUST operate on explicit contexts.

---

255. Global State

Language semantics MUST NOT depend on process-global mutable state.

This is essential for:

- reproducibility;
- parallel compilation;
- deterministic builds;
- testing;
- IDE operation.

---

256. Concurrency in the Compiler

The compiler itself may execute passes concurrently.

Compiler concurrency MUST NOT change semantic results.

---

257. Diagnostics Ordering

Diagnostics must have deterministic ordering based on canonical source/semantic ordering.

---

258. Testing Contract

Every semantic feature MUST have:

positive tests
negative tests
boundary tests
scalability tests
determinism tests
compatibility tests

Domain features MUST also have integration tests.

---

259. Positive Tests

Positive tests establish that valid source receives the expected semantic interpretation.

---

260. Negative Tests

Negative tests establish that invalid source is rejected for the correct reason.

A parser rejection must not be mistaken for a semantic rejection.

---

261. Boundary Tests

Boundary tests include:

- empty values;
- zero;
- one;
- maximum representable implementation values;
- symbolic values;
- nested scopes;
- deeply composed constructs;
- unusual Unicode;
- large programs.

No test may accidentally establish a universal artificial ceiling.

---

262. Scalability Tests

Scalability tests must test parameterized growth rather than one fixed maximum.

Examples:

1 qubit
2 qubits
N qubits

1 task
N tasks

1 node
N nodes

1 tensor dimension
N dimensions

The tests validate absence of semantic assumptions, not infinite physical allocation.

---

263. Determinism Tests

The same semantic input MUST produce:

same bindings
same types
same effects
same requirements
same diagnostics
same semantic IR

regardless of internal parallelization.

---

264. Quantum Semantic Tests

Quantum tests MUST include:

- one qubit;
- multiple qubits;
- symbolic qubit counts;
- parameterized registers;
- generic operations;
- custom operations;
- measurement;
- reset;
- mid-circuit measurement;
- classical feed-forward;
- control;
- adjoint;
- dynamic control;
- logical/physical distinction;
- capability requirements;
- insufficient-resource diagnostics;
- no fixed qubit ceiling.

---

265. Classical Semantic Tests

Classical tests MUST include:

- scalar values;
- arbitrary collection sizes;
- generic functions;
- numeric operations;
- vector/matrix/tensor operations;
- symbolic computation;
- concurrency;
- resource constraints.

---

266. HDL Semantic Tests

HDL tests MUST include:

- parameterized widths;
- parameterized memories;
- modules;
- interfaces;
- timing;
- sequential logic;
- combinational logic;
- assertions;
- synthesis intent;
- verification properties.

---

267. Hybrid Tests

Hybrid tests MUST include:

classical
→ quantum
→ measurement
→ classical
→ quantum

and verify that no implicit invalid conversion occurs.

---

268. Resource Tests

Resource tests MUST verify:

requirement
capability
constraint
preference
hint
realization

remain semantically distinct.

---

269. Hard-Coding Audit

Semantic implementation MUST be checked for artificial limits.

The audit MUST search for:

MAX_QUBITS
MAX_CPUS
MAX_CORES
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_QPUS
MAX_NODES
MAX_MEMORY
MAX_TENSOR
MAX_REGISTER
MAX_TIMELINE

and equivalent hidden constants.

A constant is allowed only when it is:

- a genuine program value;
- an implementation/security policy;
- a target-specific capability;
- an explicitly documented resource policy.

---

270. Hard-Coded Physical Identity Audit

The semantic layer MUST reject architecture that treats:

qubit0
gpu0
cpu0
node0
memory_bank0

as universal language entities.

Physical identifiers belong downstream.

---

271. Repository Integration Matrix

The semantic specification integrates with:

Component| Semantic responsibility
"grammar/Zamani.g4"| syntax
"grammar/specification/lexical.md"| lexical meaning boundary
"grammar/specification/syntax.md"| syntax contract
"grammar/specification/semantics.md"| canonical semantic contract
"grammar/spec/type-system.md"| detailed type contract
"grammar/spec/compatibility.md"| compatibility
"grammar/resources/"| resource syntax
"grammar/hardware/"| hardware intent syntax
"grammar/quantum/"| quantum syntax
"grammar/hdl/"| HDL syntax
"grammar/hybrid/"| hybrid syntax
"grammar/ai/"| AI syntax
"grammar/distributed/"| distributed syntax
"src/lexer.rs"| executable tokenization
"src/parser.rs"| executable parsing
"src/ast/"| source AST
"src/semantic.rs"| semantic implementation
"src/ir_gen.rs"| AST/semantic lowering
"src/ir_verify.rs"| IR verification
"src/quantum/ir/"| canonical quantum semantic/IR boundary
"src/quantum/scheduling/"| scheduling
"src/quantum/resilience/"| resilience
"src/quantum/error_correction/"| QEC
"src/quantum/zqn/"| quantum fault/noise semantics
HAL| target capability/state
routing| physical realization
optimization| semantics-preserving transformation
runtime| execution

---

272. Existing "src/semantic.rs" Conformance Requirements

The current semantic implementation contains useful foundations including:

- symbol tables;
- type inference;
- scopes;
- semantic errors;
- ownership/linear tracking;
- class/trait/struct handling;
- builtin registration.

However, production conformance requires the implementation to evolve toward this specification.

Specifically:

1. "Type::Unknown" MUST be analysis-state, not universal acceptance.

2. "len"/"sizeof" MUST NOT establish a universal "i64" semantic contract.

3. "usage_tracker: HashMap<String, usize>" MUST NOT be the complete model for path-sensitive linearity.

4. "in_loop: bool" MUST NOT be the complete model for nested/control-flow context.

5. Borrow-checking results MUST actually participate in semantic validity.

6. Placeholder semantic engines MUST NOT constitute successful semantic validation.

7. Builtins MUST have complete contracts.

8. Semantic errors MUST use structured categories/codes.

9. Symbol resolution MUST support the complete module/namespace model.

10. Resource/capability/effect information MUST survive into the canonical semantic representation.

The specification is not required to mirror temporary implementation structures.

---

273. Existing Quantum IR Conformance

The existing repository's canonical operation boundary MUST remain authoritative.

The structured program namespace MUST re-export/use the canonical operation type rather than create another.

Therefore the semantic pipeline is:

AST Operation
      ↓
semantic Operation
      ↓
quantum::ir::operation::Operation

not:

AST Operation
      ↓
FrontendQuantumOperation
      ↓
SchedulerQuantumOperation
      ↓
RoutingQuantumOperation
      ↓
QecQuantumOperation

unless those are explicitly adapters over the same canonical semantic type.

---

274. Existing Resource/QEC Conformance

Resource-management implementations such as:

ResourceManager
QecLimits
CancellationToken

are downstream implementation mechanisms.

The semantic specification may consume their concepts through abstract contracts.

It MUST NOT make their current implementation limits into language limits.

---

275. Existing Scheduler Conformance

Scheduling MUST consume canonical semantic/IR operations.

It MUST NOT redefine quantum operation identity.

It MAY add:

start time
duration
dependency
resource assignment

without changing the operation's semantic identity.

---

276. Existing Routing Conformance

Routing MUST consume logical identities from the canonical quantum IR.

Physical identity remains downstream.

No scheduler/routing subsystem may introduce a second "QubitId".

---

277. Existing ZQN Conformance

ZQN consumes quantum semantic/IR information and target/noise information.

It MUST NOT become a source-language parser or second quantum semantic authority.

---

278. Existing HAL Conformance

HAL provides:

capabilities
resources
state
calibration
health
execution interfaces

The semantic layer queries these through defined interfaces.

HAL MUST NOT modify the meaning of a valid source program.

---

279. Canonical Data Flow

The complete production data flow is:

Zamani source
    ↓
lexer
    ↓
tokens
    ↓
parser
    ↓
AST
    ↓
structural validation
    ↓
name/module resolution
    ↓
type analysis
    ↓
effect analysis
    ↓
ownership/linearity analysis
    ↓
resource analysis
    ↓
capability analysis
    ↓
domain semantic analysis
    ↓
quantum / classical / HDL / hybrid validation
    ↓
canonical semantic model
    ↓
canonical IR
    ↓
IR verification
    ↓
optimization
    ↓
routing / lowering
    ↓
scheduling
    ↓
resilience / QEC / ZQN where applicable
    ↓
HAL
    ↓
target realization
    ↓
runtime

---

280. Semantic Completeness Criterion

A semantic feature is production-ready only when all of the following exist:

syntax
AST mapping
name resolution
type rules
effect rules
resource rules
capability rules
ownership rules where applicable
semantic validation
diagnostics
canonical representation
IR mapping
compiler integration
runtime/backend integration
positive tests
negative tests
boundary tests
scalability tests
determinism tests
compatibility tests
hard-coding audit

---

281. File Independence Contract

This file itself is complete when:

- it defines semantic ownership;
- it defines semantic categories;
- it defines the AST boundary;
- it defines the IR boundary;
- it defines quantum integration;
- it defines classical integration;
- it defines HDL integration;
- it defines hybrid integration;
- it defines resources;
- it defines capabilities;
- it defines effects;
- it defines ownership;
- it defines determinism;
- it defines failure;
- it defines scalability;
- it defines POCO-REAF;
- it defines compatibility;
- it defines safety;
- it defines testing;
- it identifies downstream integration contracts.

Other files may implement these contracts, but they do not need to redefine the semantic authority.

---

282. Required Downstream Contracts

The following files/subsystems MUST conform to this document:

grammar/specification/syntax.md
grammar/spec/type-system.md
grammar/spec/compatibility.md
grammar/Zamani.g4
grammar/grammar.md
src/lexer.rs
src/parser.rs
src/ast/
src/semantic.rs
src/ir_gen.rs
src/ir_verify.rs
src/quantum/ir/
src/quantum/scheduling/
src/quantum/resilience/
src/quantum/error_correction/
src/quantum/zqn/
src/quantum/hal/
grammar/resources/
grammar/hardware/
grammar/quantum/
grammar/classical/
grammar/hybrid/
grammar/hdl/
grammar/ai/
grammar/distributed/
grammar/networking/
grammar/security/

---

283. Semantic Invariants

The following are global invariants.

Invariant 1 — One language

There is exactly one canonical Zamani language.

Invariant 2 — One semantic authority

This semantic contract and its implemented semantic rules define source meaning.

Invariant 3 — One canonical quantum IR

"quantum::ir" is the canonical quantum semantic/IR boundary.

Invariant 4 — No artificial hardware ceiling

Language semantics contain no fixed machine-size limits.

Invariant 5 — No hidden target dependence

Semantic meaning does not depend on the host or target unless explicitly declared.

Invariant 6 — Deterministic analysis

Semantic analysis is reproducible.

Invariant 7 — No silent semantic weakening

Resource/capability failures cannot silently change the program.

Invariant 8 — No undefined ordinary behavior

Invalid semantics produce explicit diagnostics or explicitly specified failure.

Invariant 9 — Safe implementation

Production compiler/runtime code contains no Rust "unsafe".

Invariant 10 — Source provenance

Semantic objects remain traceable to source.

Invariant 11 — Optimization preserves semantics

Optimizations cannot change observable meaning.

Invariant 12 — Backend separation

Backends realize semantics; they do not redefine them.

---

284. Production Readiness Checklist

"grammar/specification/semantics.md" is satisfied only when the repository can demonstrate:

[ ] One language authority
[ ] One semantic authority
[ ] Canonical lexical contract
[ ] Canonical syntax contract
[ ] Canonical AST contract
[ ] Canonical semantic model
[ ] Canonical quantum::ir boundary
[ ] Classical semantic integration
[ ] Quantum semantic integration
[ ] HDL semantic integration
[ ] Hybrid semantic integration
[ ] AI semantic integration
[ ] Distributed semantic integration
[ ] Networking semantic integration
[ ] Security semantic integration
[ ] Resource semantics
[ ] Capability semantics
[ ] Effect semantics
[ ] Ownership semantics
[ ] Linear semantics
[ ] Affine semantics
[ ] Determinism semantics
[ ] Explicit nondeterminism
[ ] Failure semantics
[ ] Temporal semantics
[ ] Provenance
[ ] Diagnostics
[ ] Compatibility
[ ] Scalability
[ ] POCO-REAF
[ ] No artificial hardware ceilings
[ ] No duplicate quantum IR
[ ] No duplicate QubitId
[ ] No duplicate Operation type
[ ] No hidden target dependence
[ ] No Rust unsafe
[ ] Positive tests
[ ] Negative tests
[ ] Boundary tests
[ ] Scalability tests
[ ] Determinism tests
[ ] Compatibility tests
[ ] End-to-end compiler tests

---

285. Final Semantic Contract

The fundamental Zamani semantic rule is:

«A Zamani program describes a computation and its explicit semantic requirements, not an accidental description of the machine on which that computation happens to execute.»

Therefore:

PROGRAM
   ↓
MEANING
   ↓
REQUIREMENTS
   ↓
CAPABILITIES
   ↓
CANONICAL SEMANTIC REPRESENTATION
   ↓
CANONICAL IR
   ↓
OPTIMIZATION
   ↓
REALIZATION

and never:

PROGRAM
   ↓
CURRENT MACHINE
   ↓
FIXED HARDWARE ASSUMPTIONS
   ↓
LANGUAGE MEANING

The compiler/runtime may scale the realization from:

one value
one operation
one qubit
one CPU
one accelerator
one device

to:

large classical systems
large quantum systems
large heterogeneous systems
distributed systems
hybrid systems
future computational substrates

without requiring the source program to be rewritten merely because the available resources changed.

The semantic system therefore establishes the foundation for:

Program Once
Compile Once
Run Everywhere
Anywhere
Forever

subject to the actual semantic requirements of the program, the capabilities of the chosen environment, explicit constraints, and the resources actually available.

No implementation is permitted to convert a temporary machine limitation into a universal Zamani language limitation.

No backend is permitted to redefine source meaning.

No domain is permitted to create a competing semantic universe.

No quantum subsystem is permitted to create a competing canonical quantum IR.

No optimization is permitted to change observable semantics.

No resource manager is permitted to redefine language resource semantics.

No scheduler is permitted to redefine operation meaning.

No router is permitted to redefine logical computation.

No QEC implementation is permitted to redefine quantum source semantics.

No ZQN implementation is permitted to redefine the language.

No HAL implementation is permitted to redefine the source program.

The universal boundary is:

                    ZAMANI SOURCE
                          │
                          ▼
                     LEXER/PARSER
                          │
                          ▼
                          AST
                          │
                          ▼
                 SEMANTIC ANALYSIS
                          │
       ┌──────────────────┼──────────────────┐
       │                  │                  │
       ▼                  ▼                  ▼
    TYPES             EFFECTS           RESOURCES
       │                  │                  │
       └──────────────────┼──────────────────┘
                          ▼
                CANONICAL SEMANTIC MODEL
                          │
          ┌───────────────┼────────────────┐
          │               │                │
          ▼               ▼                ▼
      CLASSICAL       QUANTUM            HDL
                         │
                         ▼
                    quantum::ir
          │               │                │
          └───────────────┼────────────────┘
                          ▼
                     CANONICAL IR
                          │
                          ▼
                     OPTIMIZATION
                          │
              ┌───────────┼───────────┐
              ▼           ▼           ▼
           ROUTING     SCHEDULING   RESILIENCE
              │           │           │
              └───────────┼───────────┘
                          ▼
                         ZQN
                          │
                          ▼
                         HAL
                          │
                          ▼
                  TARGET REALIZATION
                          │
          ┌───────────────┼────────────────┐
          ▼               ▼                ▼
         CPU             GPU              QPU
          │               │                │
          └───────────────┼────────────────┘
                          ▼
                     FUTURE TARGETS

That boundary is the semantic foundation on which the rest of "grammar/" and the Zamani compiler must converge.
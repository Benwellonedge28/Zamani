Zamani Memory Grammar

Production Architecture, Ownership, Integration, Scalability, and Conformance Contract

Path: "grammar/memory/README.md"

Domain: Memory language syntax

Language: Zamani

Grammar technology: ANTLR4

Implementation baseline: Rust 1.97 / Rust 1.97.1, Edition 2021

Safety requirement: Safe Rust only; "unsafe" is prohibited.

Architectural objective: "Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever" (POCO-REAF)

Scope: Source-level memory intent for classical, quantum-classical, hardware, distributed, accelerator, embedded, HPC, and future computing environments.

---

1. Purpose

The "grammar/memory/" subsystem defines the source-language syntax for expressing memory-related intent in Zamani.

It exists so that a Zamani program can express concepts such as:

- ownership;
- borrowing;
- lifetimes;
- allocation intent;
- deallocation intent;
- shared memory;
- distributed memory;
- memory spaces;
- memory regions;
- memory requirements;
- memory constraints;
- memory preferences;
- memory hints;
- memory operations;
- memory-qualified locations;
- memory-related annotations;
- memory policies;
- future memory abstractions.

The grammar must remain independent of any particular machine.

The fundamental principle is:

«Zamani source describes what memory semantics the program requires, not the accidental physical characteristics of the machine currently executing it.»

Therefore the memory grammar must remain usable from the smallest supported environment through arbitrarily large systems, subject only to the resources actually available and the semantic requirements of the program.

---

2. POCO-REAF Principle

The memory grammar is part of Zamani's POCO-REAF architecture:

Program Once
      ↓
Portable Zamani source
      ↓
Compile Once
      ↓
Architecture-independent semantic representation
      ↓
Target-specific lowering
      ↓
Run Everywhere / Anywhere
      ↓
Available hardware and runtime resources
      ↓
Run Forever
      ↓
Versioned and extensible semantic interpretation

Memory syntax must therefore survive changes in:

- CPU architecture;
- GPU architecture;
- FPGA architecture;
- ASIC architecture;
- quantum hardware;
- accelerator architecture;
- cache hierarchy;
- memory hierarchy;
- address width;
- NUMA topology;
- distributed topology;
- storage technology;
- runtime implementation;
- operating system;
- deployment environment;
- future computing architectures.

A source program must not require rewriting merely because the available machine has a different memory organization.

---

3. Scope of This Directory

The memory grammar owns syntax.

It does not own the complete semantic meaning of memory operations.

The boundary is:

Source
  ↓
Lexer
  ↓
Memory grammar
  ↓
Parser
  ↓
AST
  ↓
Semantic analysis
  ↓
Canonical semantic representation
  ↓
Resource / ownership / effects / execution models
  ↓
IR lowering
  ↓
Optimization
  ↓
Scheduling
  ↓
Hardware realization
  ↓
Runtime

The grammar must not bypass this pipeline.

---

4. Directory Structure

The intended memory domain is:

grammar/memory/
├── README.md
├── memory.g4
├── ownership.g4
├── borrowing.g4
├── lifetimes.g4
├── allocation.g4
├── deallocation.g4
├── shared-memory.g4
├── distributed-memory.g4
└── memory-constraints.g4

These files have different responsibilities.

They must not duplicate one another.

---

5. File Ownership Matrix

File| Primary responsibility
"README.md"| Architecture, contracts, integration, conformance
"memory.g4"| Canonical memory-domain syntax and composition boundary
"ownership.g4"| Ownership-specific syntax
"borrowing.g4"| Borrow-specific syntax
"lifetimes.g4"| Lifetime syntax
"allocation.g4"| Allocation-intent syntax
"deallocation.g4"| Deallocation/release-intent syntax
"shared-memory.g4"| Shared-memory syntax
"distributed-memory.g4"| Distributed-memory syntax
"memory-constraints.g4"| Memory requirements/constraints/preferences/hints

No file may silently become the owner of another file's semantic domain.

---

6. "memory.g4"

Purpose

"memory.g4" is the canonical generic memory grammar entry point.

It establishes the vocabulary and composition boundary used by specialized memory grammars.

Owns

"memory.g4" owns:

- memory-domain entry points;
- generic memory constructs;
- generic memory statements;
- generic memory expressions;
- memory declarations/integration points;
- memory-qualified names;
- memory paths;
- memory places;
- memory operation invocation syntax;
- memory operation arguments;
- memory annotation syntax;
- memory-space references;
- memory-region references;
- generic ownership markers where required by the architecture;
- generic lifetime references;
- memory type annotations;
- generic memory resource-intent composition;
- extension points.

Does not own

It does not own:

- ownership checking;
- borrow checking;
- lifetime inference;
- allocation algorithms;
- allocator implementation;
- garbage collection;
- reference counting;
- physical addresses;
- cache hierarchy;
- NUMA discovery;
- device discovery;
- physical memory mapping;
- scheduling;
- placement;
- routing;
- optimization;
- quantum IR;
- QEC;
- ZQN;
- resilience;
- runtime execution.

Integration

"memory.g4" integrates with:

grammar/lexer/*
grammar/core/*
grammar/types/*
grammar/expressions/*
grammar/statements/*
grammar/declarations/*
grammar/effects/*
grammar/resources/*

The parser must ultimately produce frontend AST structures rather than a memory-specific IR.

---

7. "ownership.g4"

Purpose

Defines source syntax expressing ownership semantics.

Examples may include ownership modes such as:

linear
affine
owned

where those concepts are actually defined by the language specification.

Owns

- ownership modifiers;
- ownership annotations;
- ownership-related source markers;
- ownership transfer syntax if part of the language;
- ownership declaration syntax.

Does not own

It does not implement:

- ownership checking;
- move analysis;
- alias analysis;
- escape analysis;
- runtime reference counting;
- garbage collection;
- allocation strategy.

Those belong to semantic/compiler/runtime layers.

Integration

Consumes:

memory.g4
core/names
types/*
expressions/*

Produces syntax information consumed by:

frontend AST
semantic ownership analysis
type checking
resource analysis
compiler lowering

---

8. "borrowing.g4"

Purpose

Defines source syntax for borrowing and temporary access.

Potential forms include:

&value
&mut value
<'a>

provided those forms are formally adopted by the Zamani language specification.

Owns

- borrow markers;
- mutable-borrow markers;
- borrow-related syntax;
- lifetime references associated with borrows.

Does not own

It does not own:

- borrow validity;
- alias rules;
- mutable-alias rules;
- lifetime inference;
- escape analysis;
- data-race analysis.

These are semantic responsibilities.

Integration

borrowing.g4
      ↓
AST
      ↓
type/ownership analysis
      ↓
semantic validation
      ↓
canonical representation

The grammar must preserve source locations for precise diagnostics.

---

9. "lifetimes.g4"

Purpose

Defines lifetime names and lifetime relationships at the syntax level.

A lifetime is a symbolic semantic identifier, not a machine timer.

Examples:

'a
'scope
'region
'transaction

Owns

- lifetime identifiers;
- lifetime annotations;
- lifetime clauses;
- lifetime relationship syntax.

Does not own

It must never define:

- fixed lifetime counts;
- physical duration;
- scheduler timing;
- allocation duration;
- garbage collection timing;
- hardware retention duration.

A lifetime such as "'a" does not mean a number of nanoseconds or clock cycles.

Integration

Consumed by:

borrowing.g4
ownership.g4
types/*
semantic lifetime analysis

---

10. "allocation.g4"

Purpose

Defines syntax expressing allocation intent.

The source language may express that a value, object, buffer, region, tensor, quantum-associated classical object, or other resource requires storage.

The syntax must describe intent rather than implementation.

Conceptually:

memory::allocate(...)
memory::reserve(...)
memory::acquire(...)

The exact vocabulary must be governed by the language specification.

Owns

- allocation-operation syntax;
- allocation policies;
- allocation intent;
- allocation requirements;
- allocation preferences;
- symbolic size/extent expressions;
- alignment requirements where semantically meaningful;
- memory-space requests;
- region association.

Does not own

It does not select:

- heap implementation;
- stack implementation;
- allocator;
- physical address;
- NUMA node;
- memory bank;
- GPU memory bank;
- device;
- cache;
- physical page;
- DMA engine.

Those are target/compiler/runtime decisions.

---

11. "deallocation.g4"

Purpose

Defines source-level release/deallocation intent.

Owns

- release syntax;
- deallocation operations;
- explicit lifetime termination markers where specified;
- release policies.

Does not own

It does not determine:

- physical reclamation;
- allocator behavior;
- page reclamation;
- garbage collection;
- reference counting;
- hardware memory release.

Those belong downstream.

The grammar must also avoid assuming that every memory model requires explicit deallocation.

---

12. "shared-memory.g4"

Purpose

Defines syntax for expressing shared-memory intent.

It may represent concepts such as:

- shared ownership;
- shared access;
- shared regions;
- shared buffers;
- synchronization requirements;
- shared-memory policies.

Does not own

It does not determine:

- cache coherence implementation;
- cache-line size;
- NUMA topology;
- hardware coherence protocol;
- physical memory bank;
- synchronization implementation.

Those are target capabilities.

---

13. "distributed-memory.g4"

Purpose

Defines source-level distributed-memory intent.

It must support portable descriptions of memory whose realization may span:

- processes;
- devices;
- nodes;
- clusters;
- remote execution domains;
- future distributed architectures.

Critical scalability rule

No fixed node count may be encoded.

Invalid architectural assumptions include:

node[0]
node[1]
node[2]

when used as an implicit language-level topology.

Instead, source syntax should permit symbolic placement or requirements.

For example, conceptually:

requires memory distributed
requires memory locality(...)
prefer memory placement(...)

The exact syntax must be defined by the resource/distributed grammar contracts.

Does not own

It does not own:

- cluster discovery;
- node discovery;
- network routing;
- process placement;
- distributed scheduling;
- replication algorithms;
- consistency implementation.

---

14. "memory-constraints.g4"

Purpose

Defines memory-related:

- requirements;
- constraints;
- preferences;
- hints;
- capabilities requested by the program.

These categories must remain distinct.

For example:

requires memory.capacity(...)
constraint memory.latency(...)
prefer memory.locality(...)
hint memory.reuse(...)

The actual keywords must come from the canonical core/resource grammar.

Requirement

A memory requirement must express a semantic or resource requirement.

A preference must not be treated as a mandatory requirement.

A hint must not silently become a correctness requirement.

A constraint must be distinguishable from a preference.

---

15. Memory Spaces

A memory space is a semantic abstraction.

Possible spaces may include:

- local;
- shared;
- distributed;
- persistent;
- device;
- accelerator;
- remote;
- unified;
- managed;
- custom;
- future-defined spaces.

The grammar must remain open to future memory spaces.

It must not contain an exhaustive machine-dependent list if such a list would prevent future extensions.

A memory-space name should therefore be representable symbolically.

---

16. Memory Regions

A memory region represents a semantic grouping or lifetime domain.

It may be used for:

- lifetime organization;
- ownership organization;
- allocation grouping;
- isolation;
- resource policy;
- semantic locality.

A region must not automatically imply:

- a physical memory bank;
- a NUMA node;
- a virtual-memory region;
- a page;
- a cache region.

Those interpretations belong to later compilation stages.

---

17. Memory Places

A memory place identifies a source-level location.

Examples may include:

value
object.field
array[index]
object.field[index]
namespace::object

The grammar must preserve sufficient structure for semantic analysis to distinguish:

- local variables;
- fields;
- indexed values;
- slices;
- references;
- resources;
- symbolic locations.

Physical addresses must not be part of ordinary portable memory-place syntax.

---

18. Memory Operations

Memory operations must be open-ended.

The grammar must not require a closed list of every possible future memory operation.

Conceptually valid forms include:

memory::allocate(...)
memory::release(...)
memory::share(...)
memory::map(...)
memory::migrate(...)
memory::prefetch(...)
memory::persist(...)

provided that semantic registration establishes their meaning.

This enables future memory technologies without repeatedly restructuring the core grammar.

However, open-world syntax must not become an escape hatch for invalid programs.

Unknown operations must be diagnosed during semantic resolution where appropriate.

---

19. Expression Integration

Memory arguments must consume the canonical expression grammar.

Therefore memory operations can receive:

- constants;
- variables;
- arithmetic expressions;
- symbolic expressions;
- runtime values;
- computed extents;
- ranges;
- generic values;
- capability predicates;
- resource expressions.

The memory grammar must not duplicate expression precedence.

This prevents divergence between:

foo(...)

and:

memory::operation(...)

expression semantics.

---

20. Type Integration

Memory syntax must consume the canonical type grammar.

The memory grammar must not redefine:

- primitive types;
- arrays;
- tuples;
- references;
- generics;
- quantum types;
- hardware types;
- resource types.

Where a memory construct needs a type, it must reference the canonical type rule.

This prevents competing definitions of reference or resource types.

---

21. Ownership Boundary

The memory grammar may represent ownership syntax.

It must not implement ownership semantics.

The architectural boundary is:

ownership syntax
       ↓
AST
       ↓
type / ownership analysis
       ↓
semantic validation
       ↓
IR

This distinction is mandatory.

---

22. Quantum Integration

The memory grammar must remain compatible with quantum programming.

However, it must not redefine quantum semantics.

The architecture is:

Zamani quantum source
        ↓
quantum grammar
        ↓
frontend AST
        ↓
semantic analysis
        ↓
quantum::ir

"quantum::ir" remains the canonical quantum semantic boundary.

The memory grammar must therefore not create:

MemoryQuantumIR
QuantumMemoryIR
QuantumMemoryGate

or equivalent duplicate semantic structures.

Quantum state storage, qubit allocation, physical qubit placement, QEC resources, and quantum hardware memory behavior belong to the relevant quantum/compiler/hardware subsystems.

---

23. QEC Integration

The memory grammar must not own quantum error correction.

It may express generic memory/resource intent where required by a quantum program, but:

memory grammar
≠
QEC

QEC owns error-detection/correction semantics.

The grammar must not define:

- code distance;
- syndrome extraction;
- decoder algorithms;
- physical qubit counts;
- logical-to-physical mapping.

Those belong to QEC and downstream compilation.

---

24. ZQN Integration

ZQN describes quantum noise and fault semantics.

The memory grammar must not duplicate ZQN.

It must not define:

- noise channels;
- fault models;
- leakage models;
- loss models;
- correlated fault models.

If memory-related faults affect a quantum execution, they are interpreted by the appropriate ZQN/QEC/resilience layers.

---

25. Resource Integration

Memory requirements must integrate with the universal resource model.

The distinction is:

Requirement
Constraint
Capability
Preference
Hint

These must never be conflated.

For example:

requires memory locality

does not necessarily mean:

use NUMA node 0

and:

prefer device memory

does not mean:

device memory must exist

unless semantic analysis explicitly establishes that relationship.

---

26. Hardware Integration

Hardware realization belongs downstream.

The memory grammar must not select:

- CPU;
- GPU;
- FPGA;
- ASIC;
- quantum processor;
- memory controller;
- NUMA node;
- physical address;
- memory bank.

Hardware capability information comes from the hardware abstraction layer and compilation context.

The compiler determines how portable memory intent can be realized.

---

27. Scheduling Integration

Scheduling must remain separate.

Memory syntax may express:

requires locality
prefer reuse
requires availability

but it must not directly schedule operations.

The pipeline is:

memory intent
     ↓
semantic representation
     ↓
resource analysis
     ↓
scheduling
     ↓
target realization

Memory grammar must never contain fixed timing grids, cycle counts, or machine schedules unless timing is explicitly part of the language semantics.

---

28. Optimization Integration

Optimization consumes semantic information downstream.

The grammar must not perform:

- allocation optimization;
- lifetime optimization;
- buffer reuse;
- memory coalescing;
- placement optimization;
- cache optimization.

These belong to optimization/compiler stages.

The grammar only preserves the information needed for those stages.

---

29. Runtime Integration

Runtime receives compiled semantic/target representations.

The runtime may determine:

- actual allocation;
- available resources;
- dynamic placement;
- memory migration;
- device memory;
- distributed placement;
- reclamation;
- execution policy.

The grammar must never depend directly on runtime APIs.

This prevents:

grammar → runtime → grammar

circular dependencies.

---

30. AST Contract

Every memory syntax node must preserve enough information for later semantic analysis.

At minimum, where applicable:

- source span;
- operation name;
- qualified path;
- arguments;
- named arguments;
- memory place;
- memory space;
- region;
- ownership marker;
- borrow marker;
- lifetime identifier;
- type expression;
- resource requirement;
- constraint;
- preference;
- hint;
- annotations;
- modifiers;
- source ordering.

The AST must preserve semantic distinctions rather than flattening different constructs into strings.

---

31. AST Does Not Equal IR

The memory AST is a representation of source syntax.

It is not the canonical compiler IR.

The architecture must remain:

Grammar
  ↓
Parser
  ↓
AST
  ↓
Semantic analysis
  ↓
Canonical semantic IR

No memory grammar file may directly construct or define a replacement IR.

---

32. Semantic Contract

Semantic analysis is responsible for determining:

- whether ownership is valid;
- whether a move is legal;
- whether borrowing is valid;
- whether lifetimes are compatible;
- whether mutable aliases are legal;
- whether a memory operation exists;
- whether arguments have valid types;
- whether a memory space is compatible;
- whether a requirement is satisfiable;
- whether a constraint is satisfiable;
- whether a preference is actionable;
- whether a hint is meaningful;
- whether resource requirements are compatible;
- whether the requested semantics can be lowered.

The parser must not perform these checks.

---

33. Scalability Contract

The memory grammar must scale without semantic maximums.

It must not hard-code:

MAX_MEMORY
MAX_ALLOCATIONS
MAX_REGIONS
MAX_REFERENCES
MAX_LIFETIMES
MAX_MEMORY_SPACES
MAX_NODES
MAX_DEVICES
MAX_BUFFERS
MAX_ADDRESS_BITS

There must be no grammar-level assumptions such as:

memory[0..1024]

meaning that 1024 is a language maximum.

A numeric literal is a program value.

It must not silently become a hardware maximum.

---

34. "Infinity" Interpretation

"Infinity" means:

«No artificial language-level upper bound where the semantic model does not require one.»

Actual execution remains bounded by:

- available memory;
- compiler resources;
- runtime resources;
- hardware;
- operating-system limits;
- deployment policy;
- provider constraints.

These limitations must remain implementation/resource constraints rather than becoming arbitrary Zamani syntax restrictions.

---

35. No Physical Address Semantics

Portable Zamani source must not depend on physical addresses.

The grammar must not require source programs to encode:

0x00000000
0x80000000
0x...

as ordinary memory allocation semantics.

When low-level address manipulation is genuinely required by a specialized systems/hardware dialect, it must be explicitly represented as a target-dependent or unsafe-capability-controlled language feature rather than silently contaminating the portable memory model.

The memory grammar itself remains portable.

---

36. No Fixed Machine Topology

The grammar must not encode assumptions about:

- number of NUMA nodes;
- number of memory banks;
- number of devices;
- number of cluster nodes;
- number of accelerators;
- number of memory controllers.

Topology is discovered or supplied through:

hardware capabilities
resource model
target description
deployment configuration
runtime context

---

37. No Hidden Hardware Selection

Memory syntax such as:

device
shared
distributed
local

must not silently select a specific physical device.

Semantic intent and physical realization are separate.

For example:

requires memory space(device)

means the program requires a suitable device memory capability.

It does not mean:

GPU 0

or any other fixed device.

---

38. Generic Sizes and Extents

Memory dimensions must support symbolic and computed values.

Examples conceptually include:

allocate(n)
allocate(rows * columns)
allocate(shape)
allocate(dynamic_extent)

The grammar must not require compile-time constants unless a specific language construct semantically requires one.

This is essential for:

- tensors;
- matrices;
- scientific computing;
- AI;
- dynamic workloads;
- distributed data;
- quantum-classical workloads.

---

39. Memory Safety

Zamani's memory grammar must preserve enough structure to enable strong semantic safety.

The grammar itself must not claim to prove safety.

Safety validation occurs later.

The production pipeline should support:

syntax validation
      ↓
type validation
      ↓
ownership validation
      ↓
borrow validation
      ↓
lifetime validation
      ↓
resource validation
      ↓
effect validation
      ↓
lowering

---

40. Safe Rust Requirement

All compiler/frontend implementation associated with this grammar must use:

Rust 1.97
or
Rust 1.97.1

with:

Edition 2021

and:

unsafe = prohibited

No grammar implementation may require Rust "unsafe".

If generated parser infrastructure introduces unsafe code, the integration must be rejected or isolated and replaced with a safe implementation strategy.

---

41. Lexer Boundary

The memory grammar does not own token spelling.

Tokens belong to the canonical lexer.

Memory grammar files must consume canonical tokens for:

- identifiers;
- literals;
- operators;
- punctuation;
- annotations;
- keywords.

Specialized memory files must not independently redefine common lexer tokens.

This prevents lexer/parser drift.

---

42. Keyword Policy

Memory-specific keywords must be introduced only when there is a demonstrated semantic need.

Prefer symbolic/open-world constructs where appropriate.

Avoid creating a huge reserved-word namespace merely because future memory technologies might exist.

Vendor/future/experimental names should use the language's dialect/extension mechanisms rather than forcing permanent core keywords.

---

43. Expression Boundary

"grammar/expressions/" remains authoritative for general expressions.

Memory grammar files may reference:

expression
expressionList

but must not redefine arithmetic or operator precedence.

This guarantees that memory expressions behave consistently with all other Zamani expressions.

---

44. Type Boundary

"grammar/types/" remains authoritative for types.

Memory-specific files must reference the canonical type grammar.

They must not create competing definitions for:

- reference;
- pointer;
- array;
- tuple;
- generic;
- resource;
- quantum;
- hardware.

---

45. Effects Boundary

Memory operations can have effects.

However:

memory grammar
≠
effect system

Effects belong to:

grammar/effects/

The memory grammar may expose syntactic composition points consumed by effect analysis.

---

46. Concurrency Boundary

Shared memory may interact with concurrency.

But:

memory/shared-memory.g4
≠
grammar/concurrency/

Concurrency owns:

- tasks;
- synchronization;
- channels;
- parallel execution;
- cancellation;
- concurrency semantics.

Memory owns memory-specific intent.

Cross-domain semantic analysis connects them.

---

47. Distributed Boundary

Distributed memory is not the same thing as distributed execution.

Therefore:

distributed-memory.g4

must not redefine:

- nodes;
- services;
- messaging;
- replication;
- distributed scheduling.

Those belong to "grammar/distributed/".

---

48. HDL Boundary

HDL memory constructs may require:

- registers;
- memories;
- pipelines;
- clocked storage.

These are owned by:

grammar/hdl/

The memory grammar must provide compositional semantics where required but must not duplicate HDL constructs.

---

49. AI/Data Boundary

Tensor and dataset memory requirements may be expressed through the common memory/resource system.

The memory grammar must not duplicate:

AI tensor semantics
dataset semantics

Those belong to:

grammar/ai/
grammar/data/
grammar/classical/

The shared type/expression/resource model provides integration.

---

50. Interoperability

Foreign systems may have different memory models.

Interoperability belongs to:

grammar/interoperability/

The memory grammar must not become a C/C++/Rust-specific memory grammar.

Foreign memory semantics should be represented through explicit interoperability constructs.

---

51. Dialects and Extensions

Future memory technologies must be extensible.

The preferred architecture is:

core memory semantics
        +
dialect/extension mechanism
        +
semantic registration

rather than continually expanding the core grammar with every vendor or future memory technology.

Dialect syntax must remain versionable.

---

52. Error Handling

The parser must provide precise diagnostics for malformed memory syntax.

Diagnostics should identify:

- source location;
- unexpected token;
- expected construct;
- relevant memory grammar rule;
- contextual information where available.

The grammar must not encode semantic errors as comments.

Invalid syntax must be rejected.

Unknown semantic memory operations should be diagnosed at semantic resolution rather than silently ignored.

---

53. Determinism

Parsing must be deterministic.

The same source text under the same grammar/version must produce equivalent syntax trees.

There must be no dependency on:

- machine topology;
- available memory;
- runtime state;
- hardware discovery;
- scheduling;
- network state.

Memory grammar parsing is a pure language-processing concern.

---

54. Versioning

Memory syntax is part of the Zamani language version.

Changes must distinguish:

Compatible additions

New constructs that do not reinterpret existing valid programs.

Breaking changes

Changes that alter the meaning or validity of existing source.

Deprecations

Old constructs retained temporarily with documented migration paths.

Reserved space

Names reserved for future evolution.

Every breaking change requires an explicit compatibility policy.

---

55. Backward Compatibility

Existing valid memory syntax must not be silently removed.

Before modifying a construct:

1. identify existing consumers;
2. identify its intended semantics;
3. determine whether it is valid;
4. preserve valid behavior;
5. migrate misplaced behavior;
6. deprecate incompatible syntax;
7. document the migration;
8. update conformance tests.

---

56. Hard-Coding Audit

Every memory grammar file must be audited for:

- fixed memory limits;
- fixed address widths;
- fixed region counts;
- fixed lifetime counts;
- fixed allocation counts;
- fixed node counts;
- fixed device counts;
- fixed memory-space counts;
- fixed topology;
- fixed hardware identifiers;
- fixed allocation sizes;
- fixed compiler assumptions.

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

57. Security Boundary

Memory syntax must not silently grant privileged hardware access.

Capabilities and permissions belong to the relevant security/capability system.

A memory operation must not automatically imply:

- kernel privilege;
- DMA access;
- physical memory access;
- device ownership;
- unrestricted remote memory access.

Those require explicit semantic and security authorization.

---

58. Completion Contract for "memory.g4"

"memory.g4" is complete only when:

- the canonical memory entry point exists;
- generic memory constructs are defined;
- specialized grammars can compose without duplication;
- names use canonical naming rules;
- expressions use canonical expressions;
- types use canonical types;
- AST information is preservable;
- no semantic checking occurs in the parser;
- no machine limits are hard-coded;
- no physical addresses are required;
- no device counts are fixed;
- no quantum IR is duplicated;
- no QEC/ZQN semantics are duplicated;
- no runtime dependency exists;
- parser diagnostics are deterministic;
- positive tests exist;
- negative tests exist;
- boundary tests exist;
- cross-domain tests exist;
- scalability tests exist.

---

59. Completion Contract for "ownership.g4"

Complete when:

- ownership syntax is defined;
- ownership syntax composes with memory syntax;
- canonical types are consumed;
- AST ownership information is preserved;
- no ownership checker exists in grammar;
- no allocator assumptions exist;
- no fixed ownership counts exist;
- positive and negative tests pass;
- cross-domain ownership tests pass.

---

60. Completion Contract for "borrowing.g4"

Complete when:

- borrow syntax is complete;
- mutable borrowing syntax is complete if supported;
- lifetime references compose correctly;
- expression/place integration works;
- AST source spans are preserved;
- invalid borrow syntax is rejected;
- semantic borrow checking remains downstream;
- scalability limits are absent;
- tests cover nested and complex expressions.

---

61. Completion Contract for "lifetimes.g4"

Complete when:

- lifetime identifiers are represented;
- lifetime annotations compose with borrowing and types;
- lifetime names remain symbolic;
- no physical time interpretation exists;
- no fixed lifetime count exists;
- deterministic parsing works;
- negative syntax tests exist.

---

62. Completion Contract for "allocation.g4"

Complete when:

- allocation intent is representable;
- symbolic sizes are supported;
- computed sizes are supported;
- memory spaces can be expressed;
- requirements/constraints/preferences remain distinct;
- no allocator is selected by grammar;
- no physical address is required;
- no fixed capacity is encoded;
- resource integration is preserved.

---

63. Completion Contract for "deallocation.g4"

Complete when:

- release/deallocation intent is representable;
- ownership/lifetime integration works;
- explicit release does not imply one universal runtime model;
- semantic validation remains downstream;
- no physical reclamation algorithm is encoded.

---

64. Completion Contract for "shared-memory.g4"

Complete when:

- shared-memory intent is representable;
- ownership integration exists;
- synchronization composition exists where required;
- no cache topology is assumed;
- no coherence protocol is encoded;
- no fixed participant count exists.

---

65. Completion Contract for "distributed-memory.g4"

Complete when:

- distributed memory intent is representable;
- symbolic placement is supported;
- no fixed node count exists;
- no fixed cluster topology exists;
- distributed execution remains separately owned;
- resource integration exists;
- negative and scalability tests pass.

---

66. Completion Contract for "memory-constraints.g4"

Complete when:

- requirements are distinguishable from constraints;
- constraints are distinguishable from preferences;
- preferences are distinguishable from hints;
- expressions can be used as values;
- resource integration exists;
- no target is hard-coded;
- no device ID is required;
- semantic satisfiability remains downstream.

---

67. Required Tests

The memory grammar must have dedicated tests under:

grammar/tests/memory/

Tests should cover:

basic memory
ownership
borrowing
lifetimes
allocation
deallocation
shared memory
distributed memory
memory spaces
memory regions
memory constraints
memory requirements
memory preferences
memory hints
memory operations
memory places
memory annotations

---

68. Positive Tests

Positive examples must include:

local memory
shared memory
distributed memory
symbolic allocation
dynamic allocation
computed extents
ownership
borrowing
lifetimes
memory requirements
memory constraints
memory preferences
memory hints
generic memory operations

---

69. Negative Tests

Negative tests must verify rejection of malformed constructs such as:

- missing operation arguments;
- malformed memory paths;
- invalid delimiters;
- malformed lifetime identifiers;
- malformed borrow syntax;
- invalid annotation syntax;
- invalid memory declarations;
- malformed named arguments.

Semantic-invalid examples should be distinguished from syntactically-invalid examples.

---

70. Boundary Tests

Boundary tests must cover:

- empty memory constructs;
- deeply nested memory expressions;
- long qualified names;
- large symbolic expressions;
- many independent memory operations;
- many nested regions;
- many lifetime references;
- very large source files.

Tests must not use artificial maxima as language semantics.

---

71. Scalability Tests

Scalability testing must verify that grammar syntax remains independent of:

memory capacity
allocation count
node count
device count
memory-space count
region count
lifetime count
address width
hardware topology

The tests may generate increasingly large programs.

The expected property is that no language-level artificial machine limit is encountered.

---

72. Cross-Domain Tests

Required integration tests include:

classical + memory
quantum + memory
hybrid + memory
HDL + memory
hardware + memory
distributed + memory
AI + memory
data + memory
concurrency + memory
resources + memory
effects + memory

At least one complete end-to-end fixture should combine:

classical
+
quantum
+
memory
+
distributed
+
hardware
+
resource constraints

without requiring machine-specific source semantics.

---

73. Quantum Cross-Domain Test

A quantum-memory test must verify that memory syntax can coexist with quantum syntax without creating a second quantum semantic representation.

The expected architecture is:

Zamani source
      ↓
parser
      ↓
AST
      ↓
semantic analysis
      ↓
quantum::ir

The memory grammar must remain a syntax contributor, not a quantum compiler.

---

74. HDL Cross-Domain Test

An HDL-memory test must verify that memory constructs can coexist with:

- hardware modules;
- signals;
- registers;
- clocks;
- timing;
- memories;
- pipelines.

The test must verify that physical implementation remains downstream.

---

75. Distributed Cross-Domain Test

A distributed-memory test must verify that the source can express memory intent without assuming:

N nodes
N devices
fixed topology
fixed network

---

76. Determinism Tests

The same input must produce equivalent parser results repeatedly.

Tests should include:

- small programs;
- large programs;
- nested constructs;
- cross-domain programs;
- long symbolic names;
- large memory expressions.

---

77. Round-Trip Tests

Where a Zamani AST printer/serializer exists:

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

must preserve intended memory semantics.

Formatting differences are acceptable where the language permits them.

Semantic changes are not.

---

78. Integration with Compiler

The compiler must consume the AST and perform semantic lowering.

The memory grammar must not directly select target-specific compiler passes.

Compiler integration includes:

AST
 ↓
type analysis
 ↓
ownership analysis
 ↓
resource analysis
 ↓
effect analysis
 ↓
canonical semantic representation
 ↓
target lowering

---

79. Integration with Runtime

Runtime integration must consume compiled representations.

The runtime must be free to choose an appropriate realization based on available capabilities.

The source grammar must not encode runtime implementation assumptions.

---

80. Integration with Resource Management

Memory resource requirements must be lowered into the repository's canonical resource-management system.

The grammar must preserve:

- requested capability;
- constraints;
- preferences;
- hints;
- symbolic quantities;
- provenance.

It must not itself discover resources.

---

81. Integration with Scheduling

Memory-related scheduling implications must be represented semantically and consumed by scheduling.

The grammar must not schedule memory operations.

---

82. Integration with Optimization

Optimization may transform memory operations while preserving semantics.

The grammar must not encode optimizer behavior.

---

83. Integration with Hardware Abstraction

Hardware abstraction supplies:

- available memory spaces;
- capacities;
- access capabilities;
- supported operations;
- topology;
- placement information.

The memory grammar must consume none of this at parse time.

---

84. Integration with Resilience

Memory failures may participate in resilience decisions.

However:

memory grammar
≠
resilience

Resilience decides actions such as:

- retry;
- restart;
- resume;
- remap;
- reroute;
- reschedule;
- switch backend;
- quarantine;
- abort.

The grammar merely expresses portable memory semantics.

---

85. Dependency Graph

The memory domain should be implemented in this dependency order:

specification/*
      ↓
lexer/*
      ↓
core/names
core/paths
      ↓
types/*
      ↓
expressions/*
      ↓
memory/memory.g4
      ↓
memory/lifetimes.g4
      ↓
memory/ownership.g4
      ↓
memory/borrowing.g4
      ↓
memory/allocation.g4
      ↓
memory/deallocation.g4
      ↓
memory/shared-memory.g4
      ↓
memory/distributed-memory.g4
      ↓
memory/memory-constraints.g4
      ↓
AST
      ↓
semantic analysis
      ↓
resources/effects
      ↓
canonical IR
      ↓
compiler
      ↓
scheduling/optimization/hardware
      ↓
runtime

The exact parser composition mechanism must follow the repository's actual ANTLR architecture.

---

86. Integration Graph

                    ┌──────────────┐
                    │   Lexer      │
                    └──────┬───────┘
                           │
                           ▼
                    ┌──────────────┐
                    │ memory/*.g4  │
                    └──────┬───────┘
                           │
                           ▼
                    ┌──────────────┐
                    │     AST      │
                    └──────┬───────┘
                           │
          ┌────────────────┼─────────────────┐
          ▼                ▼                 ▼
      Type system     Ownership         Effects
          │             analysis          │
          └──────────────┼─────────────────┘
                         ▼
                 Resource analysis
                         │
                         ▼
                Canonical semantic IR
                         │
          ┌──────────────┼──────────────────┐
          ▼              ▼                  ▼
    Classical IR    quantum::ir       HDL/Hardware IR
          │              │                  │
          └──────────────┼──────────────────┘
                         ▼
              Optimization / Routing
                         │
                         ▼
                     Scheduling
                         │
                         ▼
                  Hardware HAL
                         │
                         ▼
                      Runtime

There must be no reverse dependency from these downstream systems into the grammar.

---

87. Forbidden Dependencies

The memory grammar must not depend on:

runtime implementation
hardware discovery
specific backend
specific QPU
specific GPU
specific CPU
specific FPGA
specific ASIC
scheduler implementation
optimizer implementation
QEC implementation
ZQN implementation
resilience implementation
physical topology
physical addresses

---

88. Repository Consistency Rules

Before declaring the memory grammar complete, verify consistency with:

grammar/Zamani.g4
grammar/lexer/*
grammar/core/*
grammar/types/*
grammar/expressions/*
grammar/statements/*
grammar/declarations/*
grammar/effects/*
grammar/resources/*
grammar/classical/*
grammar/quantum/*
grammar/hybrid/*
grammar/hdl/*
grammar/hardware/*
grammar/distributed/*
grammar/ai/*
grammar/data/*
grammar/compile/*
grammar/execution/*
grammar/interoperability/*

Any conflicting ownership must be resolved before completion.

---

89. Documentation Consistency

The following must remain consistent:

grammar/README.md
grammar/Zamani-Grammar.md
grammar/grammar.md
grammar/memory/README.md

The canonical grammar authority must be explicitly identified.

Documentation must not describe constructs that the grammar cannot parse.

The grammar must not expose undocumented permanent language semantics.

---

90. Generated Artifacts

Generated ANTLR parser artifacts must not become manually maintained sources of truth.

The source grammar remains authoritative.

Generated files must be reproducible.

A clean build must regenerate them deterministically.

---

91. Production Readiness Checklist

The memory grammar is production-ready only when all of the following are true.

Language

- [ ] Memory syntax has one authoritative grammar architecture.
- [ ] Memory constructs are composable.
- [ ] Memory syntax is target-independent.
- [ ] Ownership syntax is defined.
- [ ] Borrowing syntax is defined.
- [ ] Lifetime syntax is defined.
- [ ] Allocation intent is defined.
- [ ] Deallocation intent is defined.
- [ ] Shared memory is defined.
- [ ] Distributed memory is defined.
- [ ] Memory constraints are defined.

Architecture

- [ ] AST integration is defined.
- [ ] Type integration is defined.
- [ ] Expression integration is defined.
- [ ] Resource integration is defined.
- [ ] Effect integration is defined.
- [ ] Compiler integration is defined.
- [ ] Runtime integration is defined.
- [ ] Hardware integration is defined.
- [ ] Scheduling integration is defined.
- [ ] Optimization integration is defined.

Quantum

- [ ] No quantum IR is duplicated.
- [ ] "quantum::ir" remains canonical.
- [ ] QEC remains outside memory grammar.
- [ ] ZQN remains outside memory grammar.
- [ ] Physical qubit allocation remains downstream.

Scalability

- [ ] No fixed memory maximum exists.
- [ ] No fixed allocation maximum exists.
- [ ] No fixed region maximum exists.
- [ ] No fixed lifetime maximum exists.
- [ ] No fixed memory-space maximum exists.
- [ ] No fixed node maximum exists.
- [ ] No fixed device maximum exists.
- [ ] No fixed topology exists.
- [ ] No physical address is required.
- [ ] No architecture-specific source assumption exists.

Safety

- [ ] Rust 1.97/1.97.1 compatible.
- [ ] Edition 2021 compatible.
- [ ] No "unsafe".
- [ ] Parser does not perform semantic authorization.
- [ ] Security capabilities remain downstream.
- [ ] Invalid syntax produces diagnostics.

Testing

- [ ] Positive tests.
- [ ] Negative tests.
- [ ] Boundary tests.
- [ ] Scalability tests.
- [ ] Determinism tests.
- [ ] Round-trip tests.
- [ ] Cross-domain tests.
- [ ] Quantum integration tests.
- [ ] HDL integration tests.
- [ ] Distributed integration tests.
- [ ] Resource integration tests.

---

92. Final Ownership Principle

The memory subsystem must preserve the following separation:

Grammar
    = syntax

AST
    = syntax structure

Semantic analysis
    = meaning and validity

Ownership analysis
    = ownership rules

Type system
    = type rules

Resource system
    = resource requirements/capabilities

Effects
    = computational effects

Canonical IR
    = semantic representation

Optimization
    = implementation improvement

Scheduling
    = ordering/timing

Hardware abstraction
    = physical capabilities

Runtime
    = execution

No layer should silently absorb another layer's responsibility.

---

93. Final Scalability Principle

The memory grammar must embody:

«A Zamani program describes memory semantics and intent, not the memory architecture of the machine on which it happens to run.»

Therefore:

one source program
        ↓
one semantic meaning
        ↓
many memory architectures
        ↓
many machine sizes
        ↓
many hardware configurations
        ↓
many deployment environments
        ↓
future architectures

The grammar must scale from:

atom

to:

embedded system
CPU
multicore
GPU
FPGA
ASIC
quantum-classical system
cluster
supercomputer
distributed system
cloud
future computing architecture

without introducing artificial source-language limits.

---

94. Final Definition of Done

"grammar/memory/" is complete only when:

1. Every memory grammar file has a single documented owner.
2. Every file has predefined upstream and downstream contracts.
3. No file requires later architectural redesign merely because another memory file is implemented.
4. No memory grammar file duplicates another domain's authority.
5. No memory grammar file creates a second IR.
6. "quantum::ir" remains the canonical quantum semantic boundary.
7. Resource discovery remains downstream.
8. Hardware realization remains downstream.
9. Scheduling remains downstream.
10. Optimization remains downstream.
11. Runtime behavior remains downstream.
12. Memory semantics remain portable.
13. Source syntax contains no accidental hardware limits.
14. Symbolic and dynamic resource quantities remain representable.
15. Future memory technologies can be introduced through extension/dialect mechanisms.
16. Parser behavior is deterministic.
17. Diagnostics are precise.
18. Compatibility is versioned.
19. Tests cover normal, invalid, boundary, scalability, and cross-domain cases.
20. Rust 1.97/1.97.1 and Edition 2021 requirements are satisfied.
21. No "unsafe" implementation is permitted.
22. The entire grammar remains compatible with POCO-REAF.

---

95. Architectural Invariant

The following invariant must never be violated:

Zamani memory syntax
        ↓
portable semantic intent
        ↓
resource/capability interpretation
        ↓
target-independent representation
        ↓
target-specific realization

Never:

Zamani memory syntax
        ↓
specific hardware

The first architecture enables:

Program Once → Compile Once → Run Everywhere → Run Anywhere → Run Forever.

The second architecture would make Zamani dependent on temporary machine characteristics and therefore violates POCO-REAF.

---

Canonical principle

«Memory is a semantic resource, not a fixed machine shape.»

«Zamani describes what memory behavior the computation requires; the compiler, resource system, scheduler, hardware abstraction, and runtime determine how that intent is realized on the available machine.»

This is the governing contract for every file under "grammar/memory/".
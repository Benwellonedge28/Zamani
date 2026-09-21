Zamani Memory Grammar

Production Architecture, Ownership, Integration, Scalability, and Conformance Contract

Path: "grammar/memory/"
Language: Zamani
Grammar technology: ANTLR4
Rust baseline: Rust 1.97 / Rust 1.97.1
Rust edition: 2021
Safety: Safe Rust only; "unsafe" is prohibited
Primary architectural objective: Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)
Scope: Source-level memory semantics, memory intent, memory requirements, and memory-related language constructs
Status: Production architecture contract

---

1. Purpose

"grammar/memory/" defines the source-language memory model of Zamani.

Memory is not merely an implementation detail. It is a cross-domain semantic foundation shared by:

- classical computing;
- systems programming;
- embedded computing;
- parallel computing;
- distributed computing;
- HPC;
- GPU and accelerator computing;
- FPGA/ASIC hardware/software co-design;
- quantum-classical computing;
- AI/ML;
- tensor/data processing;
- persistent computing;
- networking;
- security;
- edge/cloud computing;
- future computational architectures.

The memory grammar therefore MUST express portable memory meaning and intent, while leaving physical realization to later compiler, resource-management, scheduling, hardware, HAL, and runtime stages.

The fundamental invariant is:

Zamani source
    ↓
memory semantics / intent
    ↓
semantic analysis
    ↓
resource and capability analysis
    ↓
canonical semantic representation
    ↓
IR
    ↓
optimization / placement / scheduling
    ↓
target realization

The memory grammar MUST NOT turn a particular machine's physical memory architecture into a universal Zamani language limitation.

---

2. Production Goals

The memory subsystem is production-ready only when it provides:

1. one canonical memory syntax;
2. deterministic parsing;
3. complete lexical integration;
4. complete AST traceability;
5. semantic-analysis contracts;
6. resource/capability integration;
7. ownership and borrowing integration;
8. lifetime integration;
9. allocation/deallocation integration;
10. abstract address-space support;
11. shared-memory support;
12. distributed-memory support;
13. accelerator-memory support;
14. quantum-classical memory support;
15. persistence support;
16. memory constraints and requirements;
17. memory preferences and hints;
18. cross-domain integration;
19. diagnostics;
20. compatibility/versioning;
21. positive tests;
22. negative tests;
23. boundary tests;
24. scalability tests;
25. determinism tests;
26. round-trip tests where supported;
27. hard-coding audits;
28. safe-Rust implementation compatibility;
29. no duplicated IR;
30. no physical-resource assumptions.

A ".g4" file is not considered complete merely because ANTLR accepts it.

A memory feature is complete only when its entire contract exists:

Specification
    ↓
Tokens
    ↓
Grammar
    ↓
Parser
    ↓
AST
    ↓
Semantic analysis
    ↓
Resource/capability analysis
    ↓
Canonical semantic representation
    ↓
IR
    ↓
Compiler
    ↓
Runtime/backend where applicable
    ↓
Tests
    ↓
Diagnostics
    ↓
Compatibility
    ↓
Scalability audit
    ↓
Hard-coding audit

---

3. POCO-REAF Memory Principle

Zamani memory semantics must support:

«Program Once → Compile Once → Run Everywhere → Anywhere → Forever»

A memory-aware Zamani program must be capable of being realized on:

- a tiny embedded target;
- one CPU;
- many CPUs;
- a multicore processor;
- a GPU;
- an FPGA;
- an ASIC;
- an accelerator;
- a quantum-classical system;
- a simulator;
- a distributed system;
- an HPC cluster;
- a cloud deployment;
- a future computational architecture.

The source program must not require semantic rewriting merely because the target has a different memory hierarchy.

For example, these are different concepts:

requires memory capability("persistent")

and:

use storage device 3

The first describes portable program intent.

The second describes a particular realization.

The memory grammar is responsible for the first category. Target-specific realization belongs downstream.

---

4. Scalability Requirement

The memory grammar MUST scale from the smallest useful memory computation to arbitrarily large computations subject only to:

- program semantics;
- explicitly declared semantic requirements;
- representation capabilities;
- compiler policies;
- runtime policies;
- target capabilities;
- actual available resources.

The grammar MUST NOT impose universal limits such as:

MAX_MEMORY
MAX_ALLOCATIONS
MAX_REGIONS
MAX_REFERENCES
MAX_LIFETIMES
MAX_MEMORY_SPACES
MAX_ADDRESS_BITS
MAX_DEVICES
MAX_NODES
MAX_ACCELERATORS
MAX_BUFFERS
MAX_OBJECTS
MAX_THREADS
MAX_PROCESSES

Likewise it MUST NOT encode:

32-bit memory
64-bit memory
1024 memory regions
4096 allocations
8 memory spaces
16 devices

as universal language restrictions.

A target may have such limitations.

A compiler profile may expose such limitations.

A runtime may report such limitations.

A particular program may explicitly require a particular size.

But none of those facts may silently become a universal grammar limitation.

---

5. Program Constants vs Artificial Limits

The prohibition on hard-coding does not prohibit ordinary program data.

This is valid:

let n = 1024;
allocate n elements;

because "1024" is program semantics.

This is not valid as a universal implementation rule:

MAX_MEMORY = 1024

when it means Zamani cannot express a larger computation.

Likewise:

Memory<T, 1024>

may be valid program semantics.

But this is prohibited:

Zamani memory objects may never exceed 1024 elements.

The same distinction applies to:

- memory capacity;
- address width;
- allocation count;
- region count;
- object count;
- process count;
- thread count;
- device count;
- accelerator count;
- node count;
- quantum resources;
- tensor dimensions;
- vector widths;
- register widths;
- storage capacity.

---

6. Existing Memory Directory

The current repository already contains the memory-domain files below. They are retained; unnecessary renaming is prohibited.

grammar/memory/
├── README.md
├── memory.g4
├── ownership.g4
├── borrowing.g4
├── lifetimes.g4
├── references.g4
├── allocation.g4
├── deallocation.g4
├── regions.g4
├── address-spaces.g4
├── shared-memory.g4
├── distributed-memory.g4
├── accelerator-memory.g4
├── quantum-memory.g4
├── persistence.g4
├── memory-capabilities.g4
└── memory-constraints.g4

The directory may gain additional files only when a real independent responsibility exists.

Do not create empty files merely to satisfy a proposed directory tree.

Every file must own a real contract.

---

7. File Ownership Matrix

File| Owns| Does not own
"README.md"| memory architecture and integration contract| executable grammar
"memory.g4"| foundational memory syntax| ownership checking, allocation implementation
"ownership.g4"| ownership syntax| ownership analysis
"borrowing.g4"| borrowing syntax| borrow checking
"lifetimes.g4"| lifetime syntax| lifetime inference/runtime timing
"references.g4"| source memory references| pointer/reference implementation
"allocation.g4"| allocation intent| allocator selection
"deallocation.g4"| release/deallocation intent| physical reclamation
"regions.g4"| semantic regions| physical memory regions
"address-spaces.g4"| abstract address-space syntax| physical addresses
"shared-memory.g4"| shared-memory intent| cache-coherence implementation
"distributed-memory.g4"| distributed-memory intent| node topology and placement
"accelerator-memory.g4"| accelerator-memory intent| GPU/device selection
"quantum-memory.g4"| quantum-classical memory intent| quantum IR/QEC/ZQN
"persistence.g4"| persistence/durability intent| storage-device selection
"memory-capabilities.g4"| memory-specific capability composition| global capability identity
"memory-constraints.g4"| memory requirements/constraints/preferences/hints| physical resource discovery

No file may silently take ownership of another file's responsibilities.

---

8. Repository Authority

The memory grammar participates in the repository-wide authority model.

The authority chain is:

language specification
        ↓
lexical specification
        ↓
grammar/lexer/
        ↓
canonical Zamani token vocabulary
        ↓
grammar/memory/
        ↓
canonical parser composition
        ↓
frontend AST
        ↓
semantic analysis
        ↓
resource/capability analysis
        ↓
canonical semantic representation
        ↓
IR
        ↓
compiler
        ↓
runtime / backend / HAL

The memory grammar is therefore not an independent language.

---

9. Existing Root Authorities

9.1 "grammar/DESIGN.md"

"grammar/DESIGN.md" owns the architecture of the grammar subsystem.

It defines:

- authority;
- composition;
- integration;
- portability;
- POCO-REAF;
- hard-coding rules;
- domain boundaries;
- AST/semantic/IR boundaries.

The memory subsystem MUST conform to it.

---

9.2 "grammar/Zamani.g4"

"Zamani.g4" remains the canonical root ANTLR composition surface.

The memory grammar must not create a competing root grammar.

Memory-specific productions belong under "grammar/memory/".

The root composition layer determines how memory constructs participate in the complete Zamani program.

---

9.3 "grammar/grammar.md"

"grammar.md" is the implementation-conformance reference.

It must distinguish statuses such as:

SPECIFIED
IMPLEMENTED
PARTIALLY IMPLEMENTED
PLANNED
DEPRECATED

A memory construct appearing in "Zamani-Grammar.md" does not become valid merely because it is documented there.

---

9.4 "grammar/Zamani-Grammar.md"

This remains the extended/historical/design reference.

It may contain:

- future memory concepts;
- Sankofa concepts;
- temporal memory;
- historical memory;
- learning/memory concepts;
- experimental memory models;
- future architectures.

Such material becomes stable Zamani syntax only after:

proposal
 ↓
specification
 ↓
AST contract
 ↓
canonical grammar
 ↓
semantic implementation
 ↓
IR integration
 ↓
tests
 ↓
compatibility review
 ↓
stable

---

10. Canonical Token Rule

The memory grammars MUST reuse the repository's existing canonical lexer/token vocabulary.

The memory subsystem must not independently redefine tokens already owned by the lexer.

This includes existing concepts such as:

- identifiers;
- qualified names;
- literals;
- punctuation;
- separators;
- operators;
- keywords;
- delimiters;
- expression operators.

Where a genuinely new lexical concept is required, it must first be added to the canonical lexical authority and then consumed by memory grammar rules.

Do not create memory-local aliases for existing tokens merely because they have a memory-related meaning.

For example, memory syntax must reuse the canonical token for an identifier rather than introducing:

MEMORY_IDENTIFIER

when ordinary Zamani identifiers already provide the required lexical behavior.

Likewise, do not create duplicate tokens equivalent to existing concepts.

The previous repository-wide token cleanup remains applicable:

- duplicate "Question" / "QuestionMark" semantics must not be reintroduced;
- duplicate "Ampersand" / "BitAnd" concepts must remain resolved according to the canonical lexer contract;
- literal syntax must agree with the actual lexer;
- memory grammar must never assume a token that the canonical lexer cannot produce.

---

11. "memory.g4" — Foundational Memory Grammar

"memory.g4" is the foundational memory-domain grammar.

It owns reusable memory syntax and composition boundaries.

It may define concepts including:

- memory constructs;
- memory declarations;
- memory bindings;
- memory operations;
- memory places;
- memory paths;
- memory regions;
- memory spaces;
- memory qualifications;
- memory annotations;
- memory resource intent;
- memory requirements;
- memory constraints;
- memory preferences;
- memory hints;
- memory extension points.

It MUST NOT implement:

- ownership analysis;
- borrow checking;
- lifetime inference;
- allocation algorithms;
- garbage collection;
- reference counting;
- physical address assignment;
- hardware discovery;
- cache discovery;
- NUMA discovery;
- device selection;
- routing;
- scheduling;
- optimization;
- QEC;
- ZQN;
- HAL behavior;
- runtime execution.

The current "memory.g4" establishes the correct target-independent direction and must remain the foundation.

---

12. Universal Rule Reuse

Memory grammars MUST consume universal grammar concepts rather than redefining them.

Where provided by the canonical composition, memory grammar should reuse:

identifier
qualifiedName
expression
expressionList
typeExpression
attribute
modifier
visibility
path
literal

The exact rule names are determined by the repository's canonical grammar.

Do not create a second:

identifier
expression
type
qualifiedName

inside memory.

This prevents grammar divergence.

---

13. Ownership — "ownership.g4"

"ownership.g4" owns source-level ownership syntax.

The semantic model may represent concepts such as:

owned
moved
transferred
shared
linear
affine
borrowed

The grammar only represents source intent.

Ownership validity belongs to semantic analysis.

The pipeline is:

ownership syntax
    ↓
AST ownership information
    ↓
type/resource/ownership analysis
    ↓
semantic validation

The grammar must not implement:

- move checking;
- alias analysis;
- escape analysis;
- ownership inference;
- reference counting;
- garbage collection;
- allocator behavior.

No:

MAX_OWNERS
MAX_MOVES
MAX_ALIASES

may be introduced.

---

14. Borrowing — "borrowing.g4"

"borrowing.g4" owns source-level borrowing syntax.

Where supported by the canonical Zamani type system, it may express concepts analogous to:

&value
&mut value

and lifetime-associated forms.

Borrow validity belongs to semantic analysis.

The grammar must support arbitrary composition without imposing:

MAX_BORROWS
MAX_REFERENCES
MAX_NESTING

as universal language limits.

Borrow checking must remain outside ANTLR grammar actions.

---

15. Lifetimes — "lifetimes.g4"

A lifetime is a semantic relationship, not necessarily physical time.

It may describe:

- lexical scope;
- ownership scope;
- region lifetime;
- resource lifetime;
- transaction lifetime;
- execution lifetime.

A lifetime identifier must not imply a fixed machine clock.

The grammar must not encode:

nanoseconds
cycles
ticks
fixed retention periods

unless those are ordinary program values in a separately defined timing model.

Lifetime inference and validity belong to semantic analysis.

---

16. References — "references.g4"

"references.g4" owns source-level memory-reference constructs.

References may point to:

- variables;
- fields;
- indexed elements;
- slices;
- regions;
- abstract memory objects;
- other memory places.

It integrates with:

grammar/types/
grammar/expressions/
grammar/memory/memory.g4

It must not create an independent pointer/reference IR.

Physical pointer representation is a backend concern.

---

17. Allocation — "allocation.g4"

"allocation.g4" owns allocation intent.

Allocation may express:

- symbolic sizes;
- computed sizes;
- extents;
- shapes;
- memory spaces;
- regions;
- semantic requirements;
- alignment requirements where meaningful;
- persistence requirements;
- capability requirements.

Examples are conceptual:

allocate n elements
allocate Memory<T, size>
allocate region
reserve memory satisfying requirement

The exact legal syntax is determined by the canonical specification and existing grammar.

The grammar must not select:

- stack;
- heap;
- allocator;
- page;
- memory bank;
- NUMA node;
- GPU memory;
- DMA engine;
- physical address.

Those are downstream realization decisions.

---

18. Deallocation — "deallocation.g4"

"deallocation.g4" owns explicit release/deallocation intent.

It must not assume that every target uses explicit physical deallocation.

It must not encode:

- garbage collection algorithms;
- reference-count implementations;
- page reclamation;
- physical memory release;
- allocator internals.

The semantic model determines whether explicit release is valid and what it means.

---

19. Regions — "regions.g4"

A memory region is an abstract semantic grouping.

A region may represent:

- lifetime;
- ownership;
- allocation grouping;
- isolation;
- locality;
- policy;
- persistence;
- security;
- transactional scope.

A region does not automatically mean:

- NUMA node;
- cache;
- memory bank;
- page;
- physical address range.

Physical realization is downstream.

There is no universal maximum number of regions.

---

20. Address Spaces — "address-spaces.g4"

"address-spaces.g4" defines abstract memory-space semantics.

Potential semantic categories include:

local
shared
remote
device
accelerator
persistent
managed
unified
distributed
custom

These are semantic categories, not necessarily a closed enumeration.

The grammar must support open-ended extension where the canonical capability/name system permits it.

The grammar must never impose a universal:

32-bit address
64-bit address
128-bit address

limit.

A program may explicitly require an address representation as program semantics. That is different from the language imposing a global address width.

---

21. Shared Memory — "shared-memory.g4"

"shared-memory.g4" owns source-level shared-memory intent.

It may express:

- shared access;
- shared ownership;
- shared regions;
- shared buffers;
- visibility requirements;
- synchronization relationships;
- consistency requirements.

Concurrency semantics remain integrated with:

grammar/concurrency/

The grammar must not encode:

- cache-line size;
- cache-coherence protocol;
- NUMA topology;
- cache hierarchy;
- physical memory bank.

Those belong to target/resource analysis.

---

22. Distributed Memory — "distributed-memory.g4"

"distributed-memory.g4" owns distributed-memory intent.

It may describe memory that is:

- remote;
- replicated;
- partitioned;
- distributed;
- shared across execution domains;
- fault tolerant;
- persistent;
- consistency constrained.

It must not hard-code:

node0
node1
node2

as the universal topology.

Nor may it define:

MAX_NODES
MAX_MEMORY_NODES
MAX_DEVICES

as language limits.

Integration belongs with:

grammar/distributed/
grammar/execution/
grammar/networking/
grammar/resources/

---

23. Accelerator Memory — "accelerator-memory.g4"

"accelerator-memory.g4" describes memory intent associated with abstract accelerators.

It may express:

- accelerator-visible memory;
- device-accessible memory;
- managed memory;
- shared accelerator memory;
- transfer intent;
- accelerator memory capabilities;
- locality preferences;
- synchronization requirements.

It must not encode:

GPU 0
GPU 1
CUDA device 0
ROCm device 0
FPGA 0

as portable language constructs.

Vendor/device-specific syntax belongs to explicit interoperability or dialect systems.

---

24. Quantum Memory — "quantum-memory.g4"

"quantum-memory.g4" handles memory semantics relevant to quantum-classical programs.

It must integrate with the existing canonical quantum architecture.

The canonical quantum semantic boundary remains:

quantum::ir

The pipeline is:

Zamani source
    ↓
lexer
    ↓
parser
    ↓
domain-neutral AST
    ↓
semantic analysis
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
hardware

"quantum-memory.g4" MUST NOT create:

QuantumMemoryIR
QuantumGateIR
PhysicalQubitIR
MemoryQuantumIR

or another quantum representation.

It must not own:

- "QubitId";
- physical qubit mapping;
- QEC;
- ZQN;
- routing;
- calibration;
- physical device allocation.

Those remain owned by their established subsystems.

---

25. Persistence — "persistence.g4"

"persistence.g4" owns persistence and durability intent.

It may express:

- persistent data;
- durable regions;
- retention requirements;
- durability properties;
- recovery-related persistence;
- persistence capability requirements.

It must not select:

SSD
NVRAM
MRAM
filesystem
storage controller
physical storage device

unless a separate explicit target-specific dialect is being used.

Portable source remains abstract.

---

26. Memory Capabilities — "memory-capabilities.g4"

"memory-capabilities.g4" is the bridge between memory semantics and the repository-wide capability model.

It MUST NOT become a second capability system.

The intended ownership chain is:

grammar/core/capabilities.g4
        ↓
canonical capability identity/reference
        ↓
grammar/resources/capabilities.g4
        ↓
resource capability semantics
        ↓
grammar/memory/memory-capabilities.g4
        ↓
memory-domain association

Memory capability syntax must therefore reuse the canonical capability representation.

---

27. Open-World Capabilities

Memory capabilities must be extensible.

Examples may include:

memory::atomic
memory::coherent
memory::persistent
memory::durable
memory::shared
memory::distributed
memory::remote_access
memory::mapped
memory::unified
memory::managed
memory::transactional
memory::encrypted
memory::protected
memory::recoverable

These are examples, not necessarily a closed built-in enumeration.

The grammar must allow future or dialect-defined capability identities through the canonical qualified-name/capability model.

For example:

future::memory::new_architecture
vendor::memory::extension
domain::memory::capability

must be representable where the general capability system permits it.

Do not create:

memoryCapability
    : ATOMIC
    | COHERENT
    | PERSISTENT
    | ...
    ;

as a closed universal list.

That would require grammar modification for every new capability.

---

28. Requirement / Capability / Constraint / Preference / Hint

The memory subsystem MUST preserve the following semantic distinctions.

Requirement

A property that must be satisfied.

requires memory capability memory::persistent

Capability

A property an environment can provide.

memory::persistent

Constraint

A condition that a valid realization must satisfy.

memory constraint durability >= required_durability

Preference

A preferred realization.

prefer memory capability memory::unified

Hint

Optimization guidance that does not change program meaning.

hint memory locality

Realization

A downstream physical mapping.

logical memory region
    ↓
physical memory resource

The grammar must not collapse these concepts into one generic construct if doing so would destroy semantic distinctions.

---

29. Memory Constraints — "memory-constraints.g4"

"memory-constraints.g4" owns memory-specific resource intent.

It may express:

- minimum requirements;
- maximum semantic constraints;
- capability requirements;
- capacity requirements;
- latency constraints;
- bandwidth constraints;
- locality requirements;
- persistence requirements;
- consistency requirements;
- reliability requirements;
- security requirements;
- preferences;
- hints.

The grammar must not decide whether a target satisfies a constraint.

That is resource feasibility analysis.

For example:

requires memory capacity >= required_capacity

means the program requires a capacity.

It does not mean:

allocate physical RAM immediately

---

30. Memory Operations Must Remain Extensible

The memory language should avoid becoming a dictionary of every possible memory operation.

Conceptual operations may include:

allocate
release
map
unmap
share
migrate
prefetch
persist
flush
protect
synchronize

but operation identity should remain semantic data where the language architecture permits it.

Do not add a new keyword merely because a library or hardware vendor introduces a new memory operation.

A new operation becomes core syntax only when it has language-level semantics that genuinely require grammar support.

---

31. Memory Type Integration

Memory constructs integrate with the canonical type system.

Potential source-level forms include:

Memory<T, size>
Memory<T, shape>
Reference<T>
Region<T>

where those forms are actually specified by the canonical type system.

The memory grammar must not independently redefine generic types, type parameters, array syntax, tensor syntax, or type constraints.

The type layer owns:

type structure
generic parameters
type constraints
dependent relationships
resource-aware types

The memory layer owns memory-specific semantic composition.

---

32. Tensor and Data Integration

Memory must support arbitrarily shaped data where the type/data systems permit it.

For example:

Tensor<T, shape>

must describe a program-level shape.

It must not impose:

MAX_TENSOR_RANK
MAX_TENSOR_DIMENSION
MAX_ELEMENTS

as universal grammar limitations.

Integration:

data/
types/
expressions/
memory/
resources/

must converge through semantic analysis rather than competing type systems.

---

33. Concurrency Integration

Memory and concurrency are related but not identical.

Memory owns:

- memory sharing;
- memory visibility;
- memory lifetime;
- memory ownership;
- memory capability.

Concurrency owns:

- tasks;
- scheduling;
- synchronization;
- channels;
- parallel execution;
- actors;
- execution domains.

The semantic layer combines these facts.

Do not move concurrency semantics into "memory/".

---

34. Distributed Integration

Distributed memory integrates with:

grammar/distributed/
grammar/networking/
grammar/execution/
grammar/resources/

Memory grammar may state:

remote
distributed
replicated
consistent
persistent
fault-tolerant

but must not determine:

node 7
rack 3
network link 12
cluster size 64

unless such information is explicitly part of a target-specific deployment description outside portable source semantics.

---

35. Hardware / HDL Integration

Memory is fundamental to hardware/software co-design.

HDL may describe:

- memories;
- interfaces;
- ports;
- registers;
- buffers;
- pipelines;
- timing;
- verification properties.

Memory grammar describes software-visible semantic intent.

The relationship is:

Zamani memory semantics
        ↓
semantic/resource requirements
        ↓
hardware/HDL analysis
        ↓
hardware representation
        ↓
synthesis/lowering
        ↓
target

The memory grammar must not directly encode physical:

- BRAM count;
- SRAM count;
- DRAM banks;
- cache levels;
- register-file size;
- FPGA block counts.

Such facts belong to target capabilities.

---

36. Security Integration

Memory syntax may participate in security requirements such as:

- protected memory;
- isolated memory;
- encrypted memory;
- secret data;
- access control;
- capability-restricted access;
- trusted memory;
- secure persistence.

However, a memory capability MUST NOT silently grant:

- kernel privilege;
- DMA;
- unrestricted physical memory access;
- device ownership;
- privileged address-space access;
- secret-memory access.

Authorization belongs to the security/capability semantic layer.

---

37. Persistence and Recovery Integration

Persistent memory semantics may integrate with:

execution/
distributed/
security/
data/
resources/

A source-level persistence requirement may be satisfied by different target implementations.

For example:

persistent region

could eventually be realized using:

- persistent memory;
- storage;
- replicated memory;
- transactional backing;
- distributed persistence;
- a future persistence architecture.

The source semantics remain stable.

---

38. AI / ML / Data Integration

AI and data workloads may impose memory requirements involving:

- large tensors;
- streaming;
- persistence;
- sharing;
- distribution;
- accelerator accessibility;
- locality;
- checkpointing;
- model state;
- dataset state.

Memory grammar must not encode a particular framework such as:

CUDA
ROCm
PyTorch
TensorFlow

as core language semantics.

Framework integration belongs to interoperability/dialect/backend layers.

---

39. Sankofa / Extended Memory Concepts

The broader Zamani design contains memory-like concepts involving:

- recall;
- history;
- learning;
- temporal information;
- provenance;
- wisdom;
- persistent state;
- inter-memory;
- consensus.

These must not be conflated with physical memory.

The memory directory may eventually expose syntax for such concepts where they are formally specified, but their semantics must remain distinct.

For example:

program memory
persistent state
historical state
knowledge memory
temporal state

are different semantic concepts from:

RAM
cache
device memory
physical storage

The grammar must preserve that distinction.

---

40. Temporal / Multi-Timeline Integration

If MTS functionality is promoted into the stable language, memory may interact with:

- snapshots;
- checkpoints;
- state versions;
- temporal regions;
- fork/merge;
- rewind;
- speculative state.

Those concepts must remain semantic.

There must be no fixed:

MAX_TIMELINES
MAX_SNAPSHOTS
MAX_BRANCHES

in the grammar.

---

41. Metaprogramming Integration

Macros and metaprogramming may generate memory constructs.

Generated source MUST re-enter the normal validation pipeline:

generated syntax
    ↓
lexer
    ↓
parser
    ↓
AST
    ↓
semantic validation

Macros must not bypass:

- type validation;
- ownership validation;
- capability validation;
- resource validation;
- security checks.

No macro is allowed to introduce an alternate memory semantics.

---

42. Interoperability

Memory constructs may interoperate with:

- C;
- C++;
- Rust;
- WebAssembly;
- LLVM-family representations;
- accelerator APIs;
- HDL;
- foreign runtimes.

These are interoperability targets.

They are not the canonical Zamani memory model.

The mapping is:

Zamani memory semantics
        ↓
canonical semantic representation
        ↓
interoperability lowering
        ↓
foreign representation

Never:

foreign representation
        ↓
becomes Zamani's canonical memory semantics

without an explicit semantic mapping.

---

43. AST Contract

Every memory grammar construct must have a predetermined AST mapping.

The required relationship is:

grammar rule
    ↓
AST node / generic AST construct
    ↓
semantic memory construct
    ↓
canonical IR representation

The grammar must not introduce syntax whose AST meaning is undefined.

The preferred architecture is domain-neutral AST representation.

For example, memory capability syntax should become a generic capability/resource intent representation rather than a backend-specific node.

Conceptually:

memory capability
        ↓
AST capability/resource intent
        ↓
semantic capability resolution
        ↓
memory resource semantics

---

44. No Memory-Specific IR

The memory grammar MUST NOT create an independent memory IR merely because memory has many constructs.

Memory semantics should integrate with the repository's canonical semantic/IR architecture.

The eventual representation may contribute to:

- classical IR;
- resource IR;
- memory-related semantic state;
- HDL/hardware representation;
- quantum semantic representation where relevant.

But the memory grammar must not introduce another competing intermediate representation.

---

45. Quantum IR Boundary

Where memory constructs participate in quantum computation:

memory syntax
    ↓
domain-neutral AST
    ↓
semantic analysis
    ↓
quantum::ir

The memory grammar does not lower directly to "quantum::ir".

It also does not create:

memory_quantum_ir
quantum_memory_ir

This preserves the established canonical quantum architecture.

---

46. Resource Integration

Memory requirements must enter the common resource model.

The relationship is:

program intent
    ↓
memory requirement
    ↓
resource requirement graph
    ↓
capability resolution
    ↓
target feasibility
    ↓
placement
    ↓
scheduling
    ↓
realization

The memory grammar does not perform resource discovery.

---

47. Hardware Capability Resolution

A memory capability expressed by source code is not itself a physical resource.

Example:

requires memory capability memory::persistent

The compiler/resource system may discover that the target provides persistence through:

persistent-memory
storage-backed-memory
replicated-memory
distributed-memory
future-memory

The source program does not need to know which implementation satisfies the capability.

This is essential to POCO-REAF.

---

48. Error Handling

Memory grammar errors must be reported with:

- source location;
- offending construct;
- expected syntax;
- actual syntax where available;
- stable diagnostic identity;
- relevant context.

The grammar itself must not perform semantic recovery that changes program meaning.

Semantic errors belong to semantic analysis.

Examples:

unknown memory capability
unsatisfied memory requirement
invalid ownership relationship
invalid lifetime
incompatible address space
unsupported target capability

must be distinguishable from syntax errors.

---

49. Diagnostics Integration

Diagnostics should distinguish:

LEXICAL_ERROR
SYNTAX_ERROR
MEMORY_SYNTAX_ERROR
TYPE_ERROR
OWNERSHIP_ERROR
BORROW_ERROR
LIFETIME_ERROR
CAPABILITY_ERROR
RESOURCE_ERROR
MEMORY_CONSTRAINT_ERROR
PORTABILITY_ERROR
TARGET_FEASIBILITY_ERROR

The exact repository diagnostic taxonomy remains authoritative.

Memory grammar must not invent a conflicting error framework.

---

50. Determinism

The memory grammar must be deterministic.

It must:

- contain no parser actions;
- perform no I/O;
- perform no network access;
- perform no resource discovery;
- perform no hardware discovery;
- perform no allocation decisions;
- use no random behavior;
- contain no semantic side effects.

Given the same token stream, the grammar must produce equivalent parse structure.

Semantic analysis must also be deterministic wherever the language contract requires deterministic diagnostics/results.

---

51. Security Boundary

The memory grammar is not a security authority.

Parsing:

memory capability privileged::access

must not itself grant privilege.

Capability authorization happens later.

No source construct may bypass:

- security policy;
- capability authorization;
- ownership rules;
- resource policy;
- sandboxing;
- target security constraints.

---

52. Safe Rust Requirement

The grammar itself contains no executable Rust.

The Rust implementation consuming it MUST remain compatible with:

Rust 1.97
Rust 1.97.1
Rust 2021

No "unsafe" Rust is permitted for the memory grammar implementation.

This includes:

- lexer integration;
- parser integration;
- AST construction;
- semantic analysis;
- diagnostics;
- memory-domain validation;
- conformance tests.

Safe abstractions must be used.

---

53. Performance and Scalability

The memory grammar must support large source programs without embedding artificial language ceilings.

Scalability testing should vary:

- number of memory declarations;
- number of memory objects;
- number of references;
- number of ownership relationships;
- number of regions;
- number of capabilities;
- number of requirements;
- expression complexity;
- nesting depth;
- program size.

Implementation resource limits may exist, but they must be explicit implementation/resource limits rather than language semantics.

If a parser/backend has a practical limit, it must report that limit explicitly rather than silently changing the language specification.

---

54. Positive Tests

Memory tests must cover at least:

memory declarations
memory bindings
memory references
ownership
borrowing
lifetimes
allocation
deallocation
regions
address spaces
shared memory
distributed memory
accelerator memory
quantum memory
persistent memory
memory capabilities
memory requirements
memory constraints
memory preferences
memory hints

Tests must also cover integration with:

types
expressions
functions
modules
effects
concurrency
classical
quantum
hybrid
HDL
hardware
resources
distributed
AI
data
networking
security
execution
interoperability
dialects
macros
metaprogramming

---

55. Negative Tests

Negative tests must distinguish syntax failure from semantic failure.

Syntax tests should include:

- malformed memory declarations;
- malformed memory paths;
- malformed capability references;
- missing operands;
- malformed expressions;
- malformed type expressions;
- malformed region clauses;
- malformed allocation forms;
- malformed ownership syntax;
- malformed borrowing syntax;
- malformed lifetime syntax.

Semantic tests should include:

- invalid ownership;
- invalid borrow;
- invalid lifetime;
- invalid capability;
- unsatisfied requirement;
- incompatible address space;
- invalid resource relationship;
- invalid security capability;
- invalid persistence requirement.

---

56. Boundary Tests

Boundary tests must exercise:

- empty memory constructs where syntax permits;
- singleton memory objects;
- deeply nested memory expressions;
- deeply nested ownership;
- deeply nested regions;
- long qualified capability names;
- large capability lists;
- large resource expressions;
- large symbolic sizes;
- long programs;
- mixed-domain memory constructs.

No test may accidentally establish an artificial maximum.

---

57. Scalability Tests

The conformance suite should generate cases with progressively increasing:

N memory objects
N references
N regions
N capabilities
N requirements
N resource constraints
N program statements

where "N" is test-generated rather than hard-coded as a language maximum.

The expected property is:

language acceptance remains semantically defined
until actual implementation/resource limits are reached.

A test must never assert:

N > 1024 is invalid because Zamani memory supports only 1024 objects

unless that is an explicitly documented implementation resource limit unrelated to language semantics.

---

58. Cross-Domain Tests

At minimum, the memory conformance suite must contain:

classical + memory
quantum + memory
hybrid + memory
HDL + memory
hardware + memory
distributed + memory
AI + memory
data + memory
networking + memory
security + memory
effects + memory
resources + memory
concurrency + memory
execution + memory
interoperability + memory

The same memory construct must preserve its semantic meaning across these contexts.

---

59. Quantum Cross-Domain Tests

Quantum-memory tests must cover:

classical state + quantum operation
quantum result + classical memory
measurement + memory
mid-circuit measurement + memory
classical feed-forward + memory
logical quantum resources + memory requirements
quantum resource capabilities + memory capabilities

The tests must verify that:

memory syntax
    ↓
AST
    ↓
semantic analysis
    ↓
quantum::ir

does not create a second quantum representation.

---

60. HDL Cross-Domain Tests

HDL-memory tests must cover:

memory interface
memory region
buffer
register-like semantic storage
parameterized storage
shared storage
persistent storage
memory capability
memory requirement

Tests must not make:

BRAM count
SRAM count
register width
memory bank count

universal Zamani grammar limits.

---

61. Round-Trip Tests

Where AST serialization/printer infrastructure exists:

source
  ↓
lexer
  ↓
parser
  ↓
AST
  ↓
serializer/printer
  ↓
parser

must preserve memory semantics.

Whitespace and formatting may change.

Program meaning must not.

---

62. Compatibility

Memory grammar compatibility must track:

language specification
grammar/Zamani.g4
grammar/memory/*
grammar/antlr/ZamaniLexer.g4
grammar/antlr/ZamaniParser.g4
src/lexer.rs
src/parser.rs
src/ast/
semantic analysis
IR
compiler
runtime

A change to memory syntax must identify:

1. lexical impact;
2. parser impact;
3. AST impact;
4. semantic impact;
5. IR impact;
6. compiler impact;
7. runtime impact;
8. compatibility impact;
9. test impact.

Do not silently change one representation while leaving the others inconsistent.

---

63. Feature Completion Contract

A memory feature is DONE only when all applicable items below are satisfied.

[ ] Specification exists
[ ] Ownership is explicit
[ ] Does-not-own boundary is explicit
[ ] Existing tokens are reused
[ ] New tokens, if any, are registered centrally
[ ] Grammar is deterministic
[ ] Grammar is unambiguous
[ ] AST mapping is defined
[ ] Semantic mapping is defined
[ ] Resource mapping is defined
[ ] Capability mapping is defined
[ ] IR mapping is defined
[ ] Compiler integration is defined
[ ] Runtime integration is defined where applicable
[ ] Diagnostics are defined
[ ] Positive tests exist
[ ] Negative tests exist
[ ] Boundary tests exist
[ ] Scalability tests exist
[ ] Cross-domain tests exist
[ ] Determinism tests exist
[ ] Round-trip tests exist where applicable
[ ] Compatibility is documented
[ ] Hard-coding audit passes
[ ] No duplicate grammar authority exists
[ ] No duplicate AST exists
[ ] No duplicate IR exists
[ ] No physical hardware limit is encoded
[ ] Rust 1.97 compatibility is preserved
[ ] Rust 1.97.1 compatibility is preserved
[ ] No unsafe Rust is introduced

Only after these conditions are satisfied may the feature be marked stable.

---

64. Hard-Coding Audit

The memory subsystem MUST continuously audit for constructs resembling:

MAX_MEMORY
MAX_ALLOCATIONS
MAX_REGIONS
MAX_REFERENCES
MAX_LIFETIMES
MAX_MEMORY_SPACES
MAX_ADDRESS_BITS
MAX_DEVICES
MAX_NODES
MAX_ACCELERATORS
MAX_BUFFERS

and fixed physical constructs such as:

memory_bank_0
memory_bank_1
device_0
device_1
numa_0
numa_1
gpu_memory_0
qpu_memory_0

These may appear in tests that explicitly test target-specific interoperability, but must never become universal portable memory semantics.

The audit must distinguish:

program constant

from:

language implementation limit

and from:

target capability

---

65. What the Memory Grammar Must Never Own

The memory grammar must never become responsible for:

- physical memory discovery;
- physical memory allocation;
- hardware discovery;
- device enumeration;
- NUMA discovery;
- cache discovery;
- DMA execution;
- scheduling;
- routing;
- placement;
- calibration;
- QEC;
- noise modeling;
- quantum gate lowering;
- physical qubit assignment;
- compiler optimization;
- runtime execution;
- network transport;
- cryptographic authorization;
- allocator implementation.

It describes source-level semantics and intent only.

---

66. Integration With Compiler Stages

The intended pipeline is:

Zamani source
      ↓
canonical lexer
      ↓
canonical parser
      ↓
memory AST constructs
      ↓
name resolution
      ↓
type analysis
      ↓
ownership analysis
      ↓
borrow analysis
      ↓
lifetime analysis
      ↓
effect analysis
      ↓
resource analysis
      ↓
capability analysis
      ↓
memory semantic validation
      ↓
canonical semantic representation
      ↓
IR
      ↓
optimization
      ↓
placement
      ↓
routing
      ↓
scheduling
      ↓
backend lowering
      ↓
HAL
      ↓
runtime
      ↓
actual target

No physical memory decision occurs at the grammar layer.

---

67. Integration With POCO-REAF

The decisive boundary is:

                    PORTABLE
                       │
                       ▼
              Zamani source program
                       │
                       ▼
              memory semantic intent
                       │
                       ▼
            capability/resource model
                       │
                       ▼
                canonical IR
                       │
                       ▼
              compiler optimization
                       │
                       ▼
            target-independent plan
                       │
                 REALIZATION
                       │
          ┌────────────┼────────────┐
          ▼            ▼            ▼
         CPU          GPU          FPGA
          │            │            │
          ├────────────┼────────────┤
          ▼            ▼            ▼
         QPU       distributed    future

A memory feature is architecturally correct when this separation remains intact.

---

68. Example Semantic Separation

Consider:

requires memory capability memory::persistent;

The source says:

persistent memory semantics are required.

It does not say:

use SSD 0
use NVRAM 2
use memory bank 7
use physical address 0x...

Similarly:

requires memory capability memory::shared;

does not mean:

use NUMA node 1

And:

requires memory capability memory::accelerator_access;

does not mean:

use GPU 0

The compiler/resource system determines whether and how the requirement can be satisfied.

---

69. Open-Ended Future Architecture

The memory grammar must remain capable of representing future memory architectures without requiring a redesign of the core language.

Possible future systems may introduce:

- new memory technologies;
- new persistence models;
- new address spaces;
- new coherence models;
- new accelerator memory;
- new distributed memory;
- new quantum-classical memory relationships;
- new secure memory;
- new temporal memory;
- new storage-memory hybrids.

The language should represent new semantics through:

qualified names
capabilities
attributes
resource requirements
dialects
extension points

where appropriate, instead of adding a new keyword for every invention.

---

70. Dialect Boundary

A vendor-specific memory feature must not silently become core Zamani syntax.

A dialect must explicitly identify:

dialect name
dialect version
owner
syntax extension
semantic extension
AST mapping
IR mapping
compatibility
feature gate
capabilities

A dialect must not bypass:

- AST validation;
- semantic analysis;
- resource analysis;
- security validation;
- IR verification.

---

71. No Vendor Lock-In

The following must not become core memory syntax:

CUDA memory
ROCm memory
specific FPGA primitive
specific cache instruction
specific CPU memory instruction
specific QPU memory primitive
specific accelerator API

Such features belong to:

interoperability/
dialects/
backend/
HAL

The core memory model remains vendor-neutral.

---

72. Maintainability Rule

Each memory file must be independently completable.

For every file, the implementation owner must be able to answer:

What does this file own?
What does it not own?
Which existing tokens does it consume?
Which grammar rules does it consume?
Which rules depend on it?
Which AST constructs represent it?
Which semantic analysis consumes it?
Which resource/capability analysis consumes it?
Which IR receives its meaning?
Which compiler stages consume the result?
Which runtime/backend consumes the result?
Which tests prove it?
Which diagnostics prove it?
Which compatibility rules apply?
Which scalability guarantees apply?
Which hard-coding audit applies?

The answers must be known before the file is considered complete.

This prevents the situation where completing one memory file requires repeatedly rewriting it because another subsystem was defined later.

---

73. Integration Contract for Every Existing Memory File

"memory.g4"

Upstream: lexer, core names, types, expressions
Downstream: all specialized memory grammars, AST
Owns: foundational memory syntax
Does not own: semantic memory behavior
Completion: universal memory syntax has one canonical owner.

"ownership.g4"

Upstream: core/type syntax
Downstream: ownership analysis
Owns: ownership syntax
Does not own: ownership checking
Completion: ownership syntax maps completely to semantic ownership.

"borrowing.g4"

Upstream: types, expressions, ownership
Downstream: borrow checker
Owns: borrow syntax
Does not own: borrow validity
Completion: all borrow forms have semantic mappings.

"lifetimes.g4"

Upstream: names/core syntax
Downstream: lifetime analysis
Owns: lifetime notation
Does not own: lifetime inference
Completion: lifetime references are unambiguous and source-located.

"references.g4"

Upstream: expressions/types
Downstream: type/ownership analysis
Owns: memory-reference syntax
Does not own: physical pointer representation
Completion: all reference forms map to canonical AST semantics.

"allocation.g4"

Upstream: expressions/types/memory
Downstream: resource and allocation analysis
Owns: allocation intent
Does not own: allocator implementation
Completion: symbolic and computed allocation requirements are supported.

"deallocation.g4"

Upstream: memory/reference/ownership
Downstream: lifetime/resource analysis
Owns: release intent
Does not own: physical reclamation
Completion: release semantics are explicitly defined.

"regions.g4"

Upstream: names/types/memory
Downstream: lifetime/resource analysis
Owns: semantic regions
Does not own: physical topology
Completion: region semantics are target-independent.

"address-spaces.g4"

Upstream: memory/types/capabilities
Downstream: resource/backend analysis
Owns: abstract address-space intent
Does not own: physical address widths
Completion: address-space semantics are open-ended.

"shared-memory.g4"

Upstream: memory/concurrency
Downstream: concurrency/resource analysis
Owns: sharing intent
Does not own: coherence implementation
Completion: shared-memory semantics compose with concurrency.

"distributed-memory.g4"

Upstream: memory/distributed/resource
Downstream: distributed placement
Owns: distributed memory intent
Does not own: node topology
Completion: no fixed node/device counts.

"accelerator-memory.g4"

Upstream: memory/resources/hardware
Downstream: accelerator lowering
Owns: accelerator memory intent
Does not own: accelerator identity
Completion: vendor-neutral accelerator semantics.

"quantum-memory.g4"

Upstream: memory/quantum/capabilities
Downstream: semantic quantum analysis
Owns: quantum-memory source intent
Does not own: quantum IR/QEC/ZQN/routing
Completion: canonical "quantum::ir" remains the only quantum semantic boundary.

"persistence.g4"

Upstream: memory/resources
Downstream: persistence/resource analysis
Owns: persistence intent
Does not own: storage hardware
Completion: durability semantics are portable.

"memory-capabilities.g4"

Upstream: core capabilities/resources
Downstream: capability/resource analysis
Owns: memory-specific capability association
Does not own: global capability identity
Completion: open-world capabilities work without duplicated capability grammar.

"memory-constraints.g4"

Upstream: resources/capabilities/memory
Downstream: resource feasibility
Owns: memory requirements/constraints/preferences/hints
Does not own: resource discovery
Completion: requirement, constraint, preference, and hint remain distinguishable.

---

74. Production Acceptance Checklist

The "grammar/memory/" subsystem is production-ready only when:

Architecture

[ ] one canonical memory grammar architecture
[ ] no competing memory root grammar
[ ] no unnecessary renames
[ ] existing memory files retained
[ ] real ownership for every file

Lexer

[ ] existing canonical tokens reused
[ ] no duplicate token concepts
[ ] new tokens centrally registered
[ ] lexer and grammar agree
[ ] literals agree with lexer implementation

Syntax

[ ] memory syntax is deterministic
[ ] no ambiguous memory constructs
[ ] no accidental duplicate rules
[ ] universal expressions/types/names reused

AST

[ ] every memory construct has an AST mapping
[ ] no memory-specific duplicate AST architecture
[ ] source spans preserved
[ ] generated syntax re-enters normal AST validation

Semantics

[ ] ownership mapped
[ ] borrowing mapped
[ ] lifetime mapped
[ ] references mapped
[ ] allocation mapped
[ ] deallocation mapped
[ ] regions mapped
[ ] address spaces mapped
[ ] sharing mapped
[ ] distribution mapped
[ ] accelerator memory mapped
[ ] quantum memory mapped
[ ] persistence mapped

Resources

[ ] requirements distinguished from capabilities
[ ] constraints distinguished from preferences
[ ] hints distinguished from requirements
[ ] physical realization remains downstream
[ ] no physical resource discovery in grammar

Quantum

[ ] no duplicate quantum IR
[ ] quantum::ir remains canonical
[ ] no physical qubit mapping in memory grammar
[ ] no QEC implementation in grammar
[ ] no ZQN implementation in grammar

Hardware

[ ] no fixed memory banks
[ ] no fixed cache sizes
[ ] no fixed register widths
[ ] no fixed FPGA resources
[ ] no fixed GPU resources
[ ] no fixed QPU resources
[ ] no fixed physical address width

Scalability

[ ] no MAX_MEMORY
[ ] no MAX_ALLOCATIONS
[ ] no MAX_REGIONS
[ ] no MAX_REFERENCES
[ ] no MAX_DEVICES
[ ] no MAX_NODES
[ ] no MAX_ADDRESS_BITS
[ ] no universal hardware limits

Testing

[ ] positive tests
[ ] negative tests
[ ] boundary tests
[ ] scalability tests
[ ] determinism tests
[ ] cross-domain tests
[ ] compatibility tests
[ ] round-trip tests where applicable

Implementation

[ ] Rust 1.97 supported
[ ] Rust 1.97.1 supported
[ ] Rust 2021 supported
[ ] safe Rust only
[ ] no unsafe

---

75. Final Architectural Invariant

The entire memory subsystem must preserve this invariant:

                         ZAMANI SOURCE
                              │
                              ▼
                       MEMORY SEMANTICS
                              │
                              ▼
                 MEMORY REQUIREMENTS / INTENT
                              │
                              ▼
                CAPABILITY / RESOURCE ANALYSIS
                              │
                              ▼
                   CANONICAL SEMANTIC MODEL
                              │
                 ┌────────────┼────────────┐
                 │            │            │
                 ▼            ▼            ▼
             Classical     Quantum        HDL/
               IR         quantum::ir    Hardware
                 │            │            │
                 └────────────┼────────────┘
                              │
                              ▼
                         OPTIMIZATION
                              │
                              ▼
                   PLACEMENT / ROUTING
                              │
                              ▼
                         SCHEDULING
                              │
                              ▼
                    RESILIENCE / QEC
                              │
                              ▼
                             ZQN
                              │
                              ▼
                             HAL
                              │
                              ▼
                    TARGET REALIZATION
                              │
            ┌─────────┬───────┼───────┬─────────┐
            ▼         ▼       ▼       ▼         ▼
           CPU       GPU     FPGA     QPU      Future

The memory grammar defines what the program means about memory.

It does not decide where the memory physically exists.

It does not decide which device provides it.

It does not decide which address is used.

It does not decide which memory bank is selected.

It does not decide which allocator is used.

It does not decide how many machines exist.

It does not decide how a quantum device realizes the computation.

It does not duplicate "quantum::ir".

It does not implement QEC.

It does not implement ZQN.

It does not perform scheduling or routing.

It does not impose artificial resource ceilings.

It does not introduce unsafe Rust.

The fundamental production invariant is therefore:

MEMORY MEANING
      ≠
PHYSICAL MEMORY REALIZATION

and:

PORTABLE PROGRAM INTENT
      ≠
TARGET-SPECIFIC IMPLEMENTATION

This separation is mandatory for Zamani's:

«Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever»

architecture.

A memory grammar that preserves this boundary can scale from the smallest embedded computation to arbitrarily large classical, quantum, hybrid, accelerator, distributed, HDL, HPC, AI, and future systems according to the resources actually available, without turning today's hardware limitations into tomorrow's programming-language limitations.
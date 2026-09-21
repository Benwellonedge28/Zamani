Worked for 43s

I inspected the actual current repository files, including the existing grammar/memory/ directory, memory.g4, memory-capabilities.g4, memory-constraints.g4, grammar/core/capabilities.g4, the canonical grammar/antlr/ZamaniLexer.g4, and the resource grammar architecture.

The main correction needed in the current memory/README.md is that its file inventory is out of date: the repository already contains additional memory grammars such as address-spaces.g4, accelerator-memory.g4, quantum-memory.g4, persistence.g4, references.g4, and regions.g4. The README also needs to explicitly integrate memory-capabilities.g4 with the canonical core/capabilities.g4 and resources/capabilities.g4 rather than treating capabilities as a memory-owned model.

Below is the complete replacement for:

github.com/Benwellonedge28/Zamani/grammar/memory/README.md

# Zamani Memory Grammar

Production Architecture, Ownership, Integration, Scalability, and Conformance Contract

**Path:** `grammar/memory/`  
**Language:** Zamani  
**Grammar technology:** ANTLR4  
**Implementation baseline:** Rust 1.97 / Rust 1.97.1  
**Rust edition:** 2021  
**Safety:** Safe Rust only; `unsafe` is prohibited  
**Primary architectural objective:** Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)  
**Domain:** Source-level memory semantics and memory-related intent  
**Status:** Normative memory-domain architecture and completion contract

---

## 1. Purpose

The `grammar/memory/` subsystem defines the source-language syntax required for expressing memory-related semantics and intent in Zamani.

It is a foundational domain shared by:

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
- future computational architectures.

The memory grammar must allow a program to express **what memory semantics it requires** without encoding the accidental physical characteristics of the machine on which the program happens to execute.

The fundamental rule is:

> Zamani source describes portable memory semantics, requirements, capabilities, constraints, preferences, and intent. Compiler, resource-management, hardware, scheduling, routing, HAL, and runtime systems determine physical realization later.

Therefore the memory grammar must not become a description of today's RAM, cache, NUMA, GPU memory, QPU infrastructure, or any other fixed hardware architecture.

---

# 2. POCO-REAF Requirement

Memory is a critical part of:

> **Program Once → Compile Once → Run Everywhere → Anywhere → Forever**

The same source program must remain semantically meaningful when executed on:

- a tiny embedded system;
- a single CPU;
- a multicore system;
- a GPU;
- an FPGA;
- an ASIC;
- an accelerator;
- a quantum-classical system;
- a distributed machine;
- an HPC cluster;
- a cloud deployment;
- a simulator;
- a future computational architecture.

The source program must not need to be rewritten merely because the underlying memory architecture changes.

For example, source intent may express:

```text
requires memory capability memory::persistent;
requires memory capability memory::shared;
requires memory capability memory::remote_access;

without selecting:

GPU 0
NUMA node 3
memory bank 7
physical address 0x...

The latter decisions belong downstream.


---

3. Authority Model

The memory grammar participates in the repository's single-language architecture.

The authority chain is:

language specification
        |
        v
canonical lexical specification
        |
        v
grammar/lexer/*
        |
        v
grammar/antlr/ZamaniLexer.g4
        |
        v
grammar/antlr/ZamaniParser.g4
        |
        v
grammar/memory/*
        |
        v
frontend AST
        |
        v
semantic analysis
        |
        v
canonical semantic representation / IR
        |
        +-------------------+
        |                   |
        v                   v
   classical IR        quantum::ir
        |                   |
        +---------+---------+
                  |
                  v
        optimization / lowering
                  |
          +-------+-------+
          |       |       |
          v       v       v
       routing scheduling resilience
                          |
                          v
                         ZQN
                          |
                         HAL
                          |
                          v
                  target realization

The memory grammar is therefore a syntax layer, not an independent semantic or runtime architecture.


---

4. Repository Authorities

The following existing repository authorities must be preserved.

4.1 Lexer authority

Canonical production lexer:

grammar/antlr/ZamaniLexer.g4

The modular lexical source lives under:

grammar/lexer/

The memory grammar does not create its own lexer.

Memory-specific .g4 files consume the existing Zamani token vocabulary.


---

4.2 Parser authority

Canonical parser composition:

grammar/antlr/ZamaniParser.g4

The memory grammar is composed into the canonical parser.

grammar/memory/memory.g4 is the canonical memory-domain parser foundation.

The root parser, not an individual memory grammar, decides how memory syntax participates in the complete Zamani language.


---

4.3 Capability authority

Capability identity and capability-reference syntax belong to:

grammar/core/capabilities.g4

Resource-side capability relationships belong to:

grammar/resources/capabilities.g4

Memory-specific capability use belongs to:

grammar/memory/memory-capabilities.g4

This produces the required separation:

core/capabilities.g4
        |
        | capability identity/reference
        v
resources/capabilities.g4
        |
        | resource capability intent
        v
memory/memory-capabilities.g4
        |
        | memory-domain association
        v
memory semantics

memory-capabilities.g4 must never redefine the canonical capability identity model.


---

5. Actual Memory Directory

The current repository already contains the following memory-domain files:

grammar/memory/
├── README.md
├── accelerator-memory.g4
├── address-spaces.g4
├── allocation.g4
├── borrowing.g4
├── deallocation.g4
├── distributed-memory.g4
├── lifetimes.g4
├── memory-capabilities.g4
├── memory-constraints.g4
├── memory.g4
├── ownership.g4
├── persistence.g4
├── quantum-memory.g4
├── references.g4
├── regions.g4
└── shared-memory.g4

This inventory is authoritative for this directory.

The README must not describe a smaller or competing directory structure.

No existing file should be renamed merely to make the architecture appear cleaner.


---

6. Memory File Ownership Matrix

File	Owns	Does not own

memory.g4	foundational memory syntax, places, operations, spaces, generic memory constructs	ownership checking, allocation implementation, hardware
ownership.g4	ownership syntax	ownership analysis
borrowing.g4	borrow syntax	borrow checking
lifetimes.g4	lifetime syntax	lifetime inference or physical time
references.g4	memory-reference syntax	pointer/reference runtime representation
allocation.g4	allocation intent	allocator selection
deallocation.g4	release/deallocation intent	physical reclamation
regions.g4	region syntax	physical memory regions
address-spaces.g4	abstract address-space syntax	physical addresses
shared-memory.g4	shared-memory intent	cache-coherence implementation
distributed-memory.g4	distributed-memory intent	distributed scheduling/topology
accelerator-memory.g4	accelerator-memory intent	GPU/accelerator selection
quantum-memory.g4	quantum-classical memory intent	quantum IR/QEC/ZQN
persistence.g4	persistence/durability intent	storage hardware
memory-capabilities.g4	memory-specific capability associations	capability identity/registry
memory-constraints.g4	memory requirements/constraints/preferences/hints	resource discovery
README.md	architecture, ownership, integration, conformance	executable grammar


No file may silently acquire another file's ownership.


---

7. memory.g4 — Canonical Foundation

memory.g4 is the foundational memory grammar.

It owns the reusable memory-domain vocabulary and composition boundary.

It may provide constructs for:

memory declarations;

memory places;

memory paths;

memory spaces;

memory regions;

memory operations;

memory operation arguments;

memory annotations;

memory-qualified constructs;

generic memory subjects;

memory-domain extension points.


It does not own:

ownership analysis;

borrow checking;

lifetime inference;

allocation algorithms;

garbage collection;

reference counting;

physical addresses;

cache hierarchy;

NUMA discovery;

hardware discovery;

device selection;

scheduling;

routing;

optimization;

quantum IR;

QEC;

ZQN;

runtime execution.


The existing memory.g4 already establishes the important architecture:

memory syntax
    |
    v
canonical parser
    |
    v
AST
    |
    v
semantic analysis

It must remain target-independent.


---

8. ownership.g4

ownership.g4 owns source syntax for ownership.

Possible semantic categories include:

owned;

linear;

affine;

moved;

transferred;

shared;

borrowed.


Only constructs actually standardized by the language specification may become permanent syntax.

The grammar does not perform:

move checking;

alias analysis;

escape analysis;

ownership inference;

reference counting;

garbage collection;

allocation optimization.


The integration is:

ownership syntax
      |
      v
frontend AST
      |
      v
ownership analysis
      |
      v
type/resource validation
      |
      v
canonical semantic representation


---

9. borrowing.g4

borrowing.g4 owns borrow syntax.

Where the language specification adopts such forms, it may represent concepts such as:

&value
&mut value
'lifetime

The grammar does not determine whether a borrow is semantically valid.

Borrow validity belongs to semantic analysis.

It must support arbitrary nesting and composition without grammar-level limits.

It must not impose:

MAX_BORROWS
MAX_REFERENCES
MAX_LIFETIMES


---

10. lifetimes.g4

A lifetime is a symbolic semantic relationship.

It is not a physical clock duration.

Examples may include:

'a
'scope
'region
'transaction

A lifetime grammar must not encode:

nanoseconds;

clock cycles;

fixed lifetime counts;

hardware retention times;

scheduler timing.


Lifetime meaning belongs to semantic analysis.

The parser only preserves the symbolic lifetime information.


---

11. references.g4

references.g4 owns source-level memory-reference syntax.

It may represent references to:

variables;

fields;

indexed values;

slices;

regions;

abstract memory objects;

other source-level memory places.


It must integrate with:

grammar/types/
grammar/expressions/
grammar/memory/memory.g4

It must not redefine the canonical type system.

It must not introduce an independent pointer/reference IR.


---

12. allocation.g4

allocation.g4 owns source-level allocation intent.

Examples may conceptually include:

memory::allocate(...)
memory::reserve(...)
memory::acquire(...)

provided those forms are part of the canonical specification.

Allocation syntax may express:

symbolic sizes;

computed sizes;

extents;

shapes;

memory spaces;

regions;

requirements;

preferences;

alignment requirements where semantically meaningful.


It must not choose:

heap implementation;

stack implementation;

allocator;

physical page;

memory bank;

NUMA node;

GPU memory;

physical device;

DMA engine.


Those are downstream decisions.


---

13. deallocation.g4

deallocation.g4 owns source-level release/deallocation intent.

It must support memory models where explicit release is meaningful without assuming that every Zamani target uses explicit deallocation.

It must not encode:

garbage-collection algorithms;

allocator internals;

physical page reclamation;

hardware memory release;

reference-count implementation.



---

14. regions.g4

A memory region is a semantic grouping.

It may represent:

ownership scope;

lifetime scope;

allocation grouping;

isolation;

semantic locality;

policy grouping.


A region does not automatically mean:

NUMA node;

memory bank;

cache region;

page;

physical address range.


Physical realization is downstream.

No fixed number of regions is permitted.


---

15. address-spaces.g4

Address spaces are abstract semantic concepts.

They may distinguish things such as:

local;

shared;

remote;

device;

accelerator;

persistent;

managed;

unified;

distributed;

custom/future-defined spaces.


The grammar must not require a physical address.

It must not define a universal address width.

It must not encode:

32-bit addresses
64-bit addresses
128-bit addresses

as a universal machine limitation.

If a specific width is semantically required by a program, it is program data or a target-specific requirement, not an implicit grammar maximum.


---

16. shared-memory.g4

This file owns shared-memory source intent.

It may express:

shared access;

shared ownership;

shared regions;

shared buffers;

synchronization requirements;

sharing policies.


It must not define:

cache-line size;

coherence protocol;

NUMA topology;

physical memory bank;

cache hierarchy.


Concurrency semantics remain owned by:

grammar/concurrency/

The semantic layer connects shared memory and concurrency.


---

17. distributed-memory.g4

Distributed memory describes memory semantics that may be realized across:

processes;

devices;

nodes;

clusters;

remote execution domains;

future distributed architectures.


It must not hard-code:

node[0]
node[1]
node[2]

as a language topology.

Nor may it define:

MAX_NODES
MAX_DEVICES
MAX_MEMORY_NODES

Distributed execution remains owned by:

grammar/distributed/
grammar/execution/
grammar/networking/


---

18. accelerator-memory.g4

This file owns memory intent associated with abstract accelerator environments.

It may express concepts such as:

accelerator-accessible memory;

device-visible memory;

shared accelerator memory;

managed accelerator memory;

memory-transfer intent;

accelerator memory capabilities.


It must never mean:

GPU 0
GPU 1
CUDA device 0
ROCm device 0
FPGA 0

unless a separate explicitly target-specific dialect has intentionally introduced such semantics.

Portable Zamani source remains abstract.


---

19. quantum-memory.g4

Quantum-memory syntax exists only for memory concepts relevant to quantum-classical computation.

It must not create another quantum IR.

The canonical quantum semantic boundary remains:

quantum::ir

The pipeline remains:

Zamani source
      |
      v
parser
      |
      v
domain-neutral AST
      |
      v
semantic analysis
      |
      v
quantum::ir

quantum-memory.g4 must not own:

QubitId;

PhysicalQubitId;

QuantumGate;

QEC;

ZQN;

routing;

calibration;

physical qubit allocation.


Those belong to their established subsystems.


---

20. persistence.g4

persistence.g4 owns source syntax for persistent/durable memory semantics.

It may express concepts such as:

persistence;

durability;

retention intent;

durable regions;

persistent ownership;

recovery-related persistence requirements.


It must not select:

SSD;

NVRAM;

MRAM;

storage device;

filesystem;

physical medium.


The semantic/resource/hardware layers determine realization.


---

21. memory-capabilities.g4

This file is the memory-domain bridge to the canonical capability system.

It must not create a second capability identity system.

The current architecture correctly imports:

Capabilities
Memory
ResourceCapabilities
Expressions

and uses the canonical ZamaniLexer token vocabulary.

That architecture must remain.

The capability hierarchy is:

grammar/core/capabilities.g4
        |
        | capability identity/reference
        v
grammar/resources/capabilities.g4
        |
        | resource capability relationships
        v
grammar/memory/memory-capabilities.g4
        |
        | memory-domain association
        v
memory semantics

Memory capabilities remain open-world.

Examples:

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
future::memory::new_architecture
vendor::memory::extension

These are examples, not a closed enumeration.

The grammar must never become:

memoryCapability
    : ATOMIC
    | COHERENT
    | PERSISTENT
    | ...
    ;

That would make future capabilities require grammar modification.


---

22. Capability vs Resource vs Requirement

The memory subsystem must preserve these distinctions.

Capability

What an environment can provide.

memory::persistent

Requirement

What the program requires.

requires memory capability memory::persistent;

Constraint

A condition that a valid realization must satisfy.

constraint memory capability memory::coherent;

Preference

A preferred realization.

preference memory capability memory::local;

Hint

Advisory information.

hint memory capability memory::prefetchable;

Resource

A realizable computational resource.

resource::memory

Implementation decision

Which actual device or physical resource realizes the request.

This belongs downstream.

These concepts must never be collapsed.


---

23. Capability Subject

A capability may be associated with an abstract memory subject.

Examples include:

memory place
memory space
memory region
resource::memory
symbolic memory resource

The subject must not be interpreted by the parser as:

physical device;

physical memory bank;

NUMA node;

physical address;

GPU index;

QPU index.


That resolution belongs to semantic/resource/hardware analysis.


---

24. Open-World Memory Operations

Memory operations must remain extensible.

The language may eventually support operations such as:

memory::allocate(...)
memory::release(...)
memory::share(...)
memory::map(...)
memory::migrate(...)
memory::prefetch(...)
memory::persist(...)
memory::flush(...)
memory::protect(...)

but the grammar must not become a closed dictionary of every future memory operation.

Operation identity should remain semantic data wherever the language architecture permits.

Unknown operations must not be silently accepted as semantically valid merely because the grammar can parse them.

The semantic layer resolves operation identity.


---

25. memory-constraints.g4

This file owns memory-specific resource intent.

It must distinguish:

requirement
constraint
preference
hint

For example:

requires memory capability memory::persistent;

constraint memory::latency < required_latency;

preference memory capability memory::local;

hint memory::reuse;

The exact syntax remains governed by the canonical grammar and token vocabulary.

The important invariant is semantic distinction.

A preference must never silently become a mandatory requirement.

A hint must never silently become a correctness condition.


---

26. Token Policy

The memory subsystem must use the existing canonical token vocabulary.

The current architecture has:

grammar/antlr/ZamaniLexer.g4
        |
        v
grammar/lexer/tokens.g4
        |
        v
canonical Zamani tokens

Memory grammars must consume those tokens.

The memory subsystem must not create duplicate versions of:

identifiers;

qualified names;

punctuation;

comparison operators;

literals;

expression operators;

general keywords.


Existing tokens such as the repository's K_MEMORY, K_CAPABILITY, K_REQUIRES, K_CONSTRAINT, K_PREFERENCE, K_HINT, K_AVAILABLE, K_WHEN, K_ASSERT, K_IMPLIES, K_EXCLUDES, K_RESOURCE, K_PROPERTY, EQ, NE, LT, LE, GT, GE, SEMI, LBRACE, RBRACE, and COMMA must be reused where appropriate.

New tokens are allowed only when:

1. the language specification requires a genuinely new lexical category;


2. no existing token represents the concept;


3. the token belongs in the canonical lexer;


4. its spelling and compatibility behavior are specified;


5. all parser consumers and tests are updated.



A memory file must never introduce a local lexer merely for convenience.


---

27. Expression Integration

Memory grammars consume the canonical expression grammar.

They must not redefine:

arithmetic;

boolean expressions;

comparison precedence;

function calls;

indexing;

ranges;

literals;

general operators.


For example:

allocate(rows * columns)

must use the same expression semantics as:

compute(rows * columns)

This is essential for deterministic language-wide semantics.


---

28. Type Integration

Memory grammars consume the canonical type grammar.

They must not redefine:

arrays;

slices;

tuples;

references;

pointers;

generics;

resource types;

quantum types;

hardware types.


For example:

Memory<T, size>

must be interpreted through the canonical type/value/resource architecture.

The grammar must not turn a type parameter into a machine limit.


---

29. Memory and Types

The following kinds of source-level types may be meaningful:

Memory<T>
Memory<T, size>
Buffer<T>
Region<T>
Reference<T>
Shared<T>
Distributed<T>
Persistent<T>

where supported by the canonical type system.

A size such as:

1024

is a program value.

It must not be interpreted as:

MAX_MEMORY = 1024


---

30. Memory and Concurrency

Memory and concurrency are related but distinct.

grammar/memory/

owns memory semantics.

grammar/concurrency/

owns:

tasks;

parallelism;

synchronization;

channels;

actors;

scheduling semantics;

concurrency control.


Semantic analysis connects the two.

Memory grammar must not duplicate concurrency grammar.


---

31. Memory and Effects

Memory operations may produce effects.

Effects remain owned by:

grammar/effects/

The architecture is:

memory syntax
      |
      v
AST
      |
      v
effect analysis

The memory parser must not implement effect checking.


---

32. Memory and Resources

Memory resource intent integrates with:

grammar/resources/

The resource system determines:

requirements;

capabilities;

constraints;

preferences;

hints;

quantities;

availability;

resource relationships.


The memory subsystem provides domain-specific memory meaning.

The two must not become competing resource systems.


---

33. Memory and Hardware

Hardware realization belongs to:

grammar/hardware/

The memory grammar must not select:

CPU;

GPU;

FPGA;

ASIC;

QPU;

memory controller;

memory bank;

NUMA node;

cache;

physical address.


Instead:

memory intent
      |
      v
resource analysis
      |
      v
hardware capabilities
      |
      v
compiler/runtime realization


---

34. Memory and Distributed Computing

Distributed memory does not equal distributed execution.

Therefore:

memory/distributed-memory.g4

must not redefine:

nodes;

services;

messages;

distributed processes;

replication;

collective operations;

distributed scheduling.


Those belong to:

grammar/distributed/
grammar/networking/
grammar/execution/


---

35. Memory and HDL

HDL memory constructs belong to:

grammar/hdl/

They may involve:

registers;

memories;

ports;

pipelines;

timing;

clock domains;

hardware interfaces.


Memory-domain grammar may provide shared semantic foundations but must not duplicate HDL syntax.

The implementation may eventually map the same source-level memory intent to:

software memory
hardware memory
accelerator memory
distributed memory

without changing the semantic source program.


---

36. Memory and AI/Data

Memory is fundamental to:

tensors;

datasets;

model parameters;

training;

inference;

data pipelines;

streaming.


However:

grammar/memory/

must not redefine AI or data semantics.

Those remain owned by:

grammar/ai/
grammar/data/
grammar/classical/

Memory provides common storage/resource semantics.


---

37. Memory and Quantum Computing

Memory may coexist with:

quantum registers;

measurement results;

classical feed-forward;

quantum-classical data;

quantum control metadata;

simulation state.


But memory syntax must not define quantum semantics.

The canonical boundary remains:

quantum::ir

The memory grammar must never create a competing:

QuantumMemoryIR
MemoryQuantumIR

or equivalent.


---

38. Memory and QEC

QEC remains outside the memory grammar.

The memory grammar must not implement:

code distance;

syndrome extraction;

decoder algorithms;

logical-to-physical mapping;

physical qubit placement.


Those remain downstream QEC/compiler responsibilities.


---

39. Memory and ZQN

ZQN remains responsible for quantum noise/fault semantics.

Memory grammar must not define:

noise channels;

leakage;

loss;

correlated faults;

decoder behavior;

resilience actions.


If memory semantics affect quantum resilience, that information flows through semantic analysis into the existing ZQN/QEC/resilience architecture.


---

40. AST Contract

Every accepted memory construct must preserve sufficient source information for the domain-neutral AST.

Where applicable, the AST must retain:

source span;

construct kind;

qualified names;

memory operation;

operands;

expressions;

types;

memory place;

memory space;

memory region;

ownership information;

borrow information;

lifetime references;

capability references;

resource references;

requirement/constraint/preference/hint distinction;

annotations;

modifiers;

source ordering.


The grammar must not flatten semantically different constructs into an unstructured string.


---

41. AST Is Not IR

The memory grammar produces syntax information.

It does not define a memory IR.

The required architecture is:

grammar
   |
   v
parser
   |
   v
domain-neutral AST
   |
   v
semantic analysis
   |
   v
canonical semantic representation
   |
   v
canonical IR

There must not be a second memory-specific compiler IR merely because the memory grammar has its own directory.


---

42. Semantic Contract

Semantic analysis, not parsing, determines:

whether an ownership transfer is legal;

whether borrowing is valid;

whether lifetimes are compatible;

whether a memory operation exists;

whether arguments have valid types;

whether a memory space is compatible;

whether a capability exists;

whether a capability version is compatible;

whether a requirement is satisfiable;

whether a constraint is satisfiable;

whether a preference is actionable;

whether a hint is meaningful;

whether capabilities conflict;

whether memory semantics are compatible with effects;

whether a resource realization exists;

whether the construct can be lowered.


The parser must not perform these semantic decisions.


---

43. No Hardware Limits

The memory grammar MUST NOT contain language-level limits such as:

MAX_MEMORY
MAX_HEAP
MAX_STACK
MAX_ALLOCATIONS
MAX_REGIONS
MAX_REFERENCES
MAX_LIFETIMES
MAX_MEMORY_SPACES
MAX_ADDRESS_BITS
MAX_DEVICES
MAX_NODES
MAX_GPUS
MAX_FPGAS
MAX_QUBITS
MAX_BUFFERS

Nor may equivalent limits be hidden inside grammar alternatives.

Bad:

memorySpace
    : SPACE0
    | SPACE1
    | SPACE2
    ;

Good:

qualifiedName

with semantic interpretation downstream.


---

44. No Fixed Topology

The memory grammar must not assume a fixed:

number of memory banks;

number of NUMA nodes;

number of devices;

number of GPUs;

number of accelerators;

number of distributed nodes;

number of memory controllers.


Topology is supplied later by:

resource discovery
hardware description
target context
deployment configuration
runtime context


---

45. No Physical Address Requirement

Portable memory syntax must not require:

0x00000000
0x80000000
0x...

as ordinary memory semantics.

Physical address manipulation, if ever supported, must belong to an explicitly target-specific systems/hardware mechanism with separately specified safety and capability semantics.

The portable memory grammar remains abstract.


---

46. Symbolic and Computed Sizes

Memory extents must support symbolic and computed values.

Examples:

allocate(n)
allocate(rows * columns)
allocate(shape)
allocate(required_size)
allocate(dynamic_extent)

The grammar must not require a fixed compile-time machine capacity.

This is essential for:

dynamic workloads;

tensors;

AI;

scientific computing;

distributed data;

embedded systems;

HPC;

quantum-classical workloads.



---

47. "Infinity" and Scalability

"Scale to infinity" means:

> The language does not impose an arbitrary finite upper bound where the semantic model itself does not require one.



Actual execution remains limited by available:

memory;

compute resources;

compiler resources;

runtime resources;

hardware capabilities;

deployment policies;

operating-system policies;

provider policies;

security policies.


These are environment/resource constraints, not grammar limits.


---

48. Memory Safety

The grammar must preserve enough information to support:

syntax validation
      |
      v
type validation
      |
      v
ownership validation
      |
      v
borrow validation
      |
      v
lifetime validation
      |
      v
effect validation
      |
      v
resource validation
      |
      v
semantic lowering

The grammar itself does not prove memory safety.

Semantic/compiler analysis does.


---

49. Rust Implementation Contract

All Rust implementation associated with this grammar must target:

Rust 1.97
or
Rust 1.97.1

Edition 2021

and must use safe Rust.

The repository requirement is:

unsafe = prohibited

The .g4 files must remain free of embedded Rust actions.

Generated/parser integration must not require handwritten unsafe code.


---

50. Determinism

Memory parsing must be deterministic.

The same:

source
+
grammar version
+
lexer version

must produce equivalent syntax trees.

Parsing must not depend on:

available memory;

machine topology;

hardware discovery;

runtime state;

network state;

scheduling;

resource availability.


Resource availability is evaluated after parsing.


---

51. Versioning

Memory syntax is part of the Zamani language version.

Changes must be classified as:

Compatible addition

Adds new syntax without changing the meaning of valid existing programs.

Breaking change

Changes validity or meaning of existing source.

Deprecation

Retains existing syntax temporarily with migration guidance.

Experimental

Available under explicit experimental status.

Dialect extension

Introduced through the dialect mechanism rather than silently becoming universal syntax.

Every breaking change requires a compatibility entry.


---

52. Existing Syntax Preservation

Existing valid memory syntax must not be silently removed.

Before changing a memory construct:

1. identify its current owner;


2. identify current consumers;


3. determine whether it is specified;


4. determine whether it is implemented;


5. preserve valid semantics;


6. migrate only where ownership is incorrect;


7. deprecate rather than silently remove when compatibility requires it;


8. update tests;


9. update grammar.md;


10. update the relevant specification.



No unnecessary file rename is permitted.


---

53. Important Current Repository Correction

The previous memory README described only a subset of the actual memory files.

The actual repository contains additional files:

accelerator-memory.g4
address-spaces.g4
memory-capabilities.g4
persistence.g4
quantum-memory.g4
references.g4
regions.g4

These are now first-class members of the memory architecture.

The README must therefore not revert to the older incomplete inventory.


---

54. Current memory-capabilities.g4 Integration Correction

The existing memory-capabilities.g4 correctly follows the open-world capability architecture by importing:

Capabilities
Memory
ResourceCapabilities
Expressions

and using:

tokenVocab = ZamaniLexer;

This must be preserved.

The capability identity remains owned by:

grammar/core/capabilities.g4

Resource capability relationships remain owned by:

grammar/resources/capabilities.g4

Memory-specific association remains owned by:

grammar/memory/memory-capabilities.g4

No third capability registry may be created under memory/.


---

55. memory-capabilities.g4 Completion Invariant

The memory capability grammar must have a single stable public entry point:

memoryCapabilities

and a memory capability item dispatcher.

All memory capability constructs must be reachable from that composition boundary.

Any helper rule that is intended to be public must either:

1. be reachable through memoryCapabilityItem; or


2. be explicitly documented as a composition-only rule.



For example, if:

memoryCapabilitySubjectPropertyAssertion

is intended to be accepted as a memory capability item, it must be included in the public item dispatch.

No production rule may become accidentally unreachable.


---

56. Capability Grammar Scalability

Capability lists must use structural repetition.

Valid architecture:

capabilityReference
    (COMMA capabilityReference)*
    COMMA?

Invalid architecture:

capabilityList
    : capability
    | capability COMMA capability
    | capability COMMA capability COMMA capability
    ;

The first has no arbitrary semantic count.

The second creates an artificial finite grammar ceiling.


---

57. Memory Operation Scalability

Memory operations must similarly use repetition and canonical expressions.

There must be no grammar-level limit on:

number of memory operations;

number of arguments;

number of regions;

number of memory objects;

number of references;

number of capabilities.


Actual resource limits remain outside the grammar.


---

58. Security Boundary

Memory syntax must not silently grant privileged access.

A memory construct must not automatically grant:

kernel access;

physical memory access;

DMA;

device ownership;

unrestricted remote memory;

protected-memory bypass;

privileged address access.


Security and authorization belong to:

grammar/security/

and downstream semantic/security systems.


---

59. Compiler Integration

Memory syntax must flow through the existing compiler architecture:

Zamani source
      |
      v
lexer
      |
      v
parser
      |
      v
AST
      |
      v
type analysis
      |
      v
ownership analysis
      |
      v
borrow/lifetime analysis
      |
      v
effect analysis
      |
      v
resource/capability analysis
      |
      v
canonical semantic representation
      |
      v
canonical IR
      |
      v
optimization
      |
      v
target lowering

The memory grammar must not directly select compiler passes.


---

60. Runtime Integration

The runtime consumes compiled representations.

Runtime may determine:

actual memory allocation;

dynamic placement;

migration;

device memory;

distributed placement;

reclamation;

recovery;

runtime resource acquisition.


The source grammar must not depend directly on runtime APIs.

No:

grammar -> runtime -> grammar

dependency is permitted.


---

61. Scheduling Integration

Memory semantics may affect scheduling.

However:

memory grammar != scheduler

The correct flow is:

memory intent
      |
      v
semantic representation
      |
      v
resource analysis
      |
      v
scheduler
      |
      v
target realization

The memory grammar must not contain a fixed machine schedule.


---

62. Optimization Integration

The optimizer may transform:

allocations;

releases;

memory placement;

reuse;

migration;

buffering;

data movement.


provided semantics are preserved.

The grammar does not encode optimizer behavior.


---

63. Hardware Abstraction Integration

Hardware/HAL layers may discover:

memory spaces;

capacities;

access properties;

bandwidth;

latency;

topology;

coherence;

persistence;

accelerator access;

supported operations.


The memory grammar must not perform this discovery.


---

64. Resilience Integration

Memory failures may participate in resilience.

However:

memory grammar != resilience

Resilience remains responsible for decisions such as:

retry;

recovery;

remapping;

rescheduling;

migration;

backend switching;

quarantine;

abort.


The grammar merely preserves portable intent.


---

65. Forbidden Dependencies

Memory grammar files must not depend on:

specific CPU
specific GPU
specific FPGA
specific ASIC
specific QPU
specific physical qubit
specific NUMA node
specific memory bank
specific physical address
specific cache
specific allocator
specific scheduler
specific optimizer
specific runtime
specific QEC implementation
specific ZQN implementation
specific HAL implementation


---

66. Dependency Direction

The intended dependency direction is:

specification/*
      |
      v
lexer/*
      |
      v
core/*
      |
      +----> types/*
      |
      +----> expressions/*
      |
      v
memory/memory.g4
      |
      +----> memory/lifetimes.g4
      +----> memory/ownership.g4
      +----> memory/borrowing.g4
      +----> memory/references.g4
      +----> memory/allocation.g4
      +----> memory/deallocation.g4
      +----> memory/regions.g4
      +----> memory/address-spaces.g4
      +----> memory/shared-memory.g4
      +----> memory/distributed-memory.g4
      +----> memory/accelerator-memory.g4
      +----> memory/quantum-memory.g4
      +----> memory/persistence.g4
      +----> memory/memory-capabilities.g4
      +----> memory/memory-constraints.g4
      |
      v
AST
      |
      v
semantic analysis
      |
      +----> resources
      +----> effects
      +----> concurrency
      +----> distributed
      +----> quantum
      +----> hardware
      |
      v
canonical semantic representation
      |
      v
IR
      |
      v
compiler
      |
      v
runtime / HAL

Downstream systems must not become parser dependencies.


---

67. Cross-Domain Integration

Memory must integrate with at least:

classical
quantum
hybrid
hdl
hardware
distributed
ai
data
networking
security
concurrency
effects
resources
compile
execution
interoperability
dialects

The memory grammar must not duplicate their syntax.


---

68. Required Quantum Integration Test

At minimum, the repository must contain a fixture combining:

classical computation
+
memory capability/requirement
+
quantum computation
+
measurement
+
classical feed-forward

The expected semantic path is:

Zamani source
      |
      v
parser
      |
      v
domain-neutral AST
      |
      v
semantic analysis
      |
      v
quantum::ir

The memory grammar must not create a parallel quantum representation.


---

69. Required HDL Integration Test

A memory/HDL fixture must combine, where supported:

hardware module
+
memory declaration
+
memory intent
+
signals
+
registers
+
timing
+
verification

The grammar must not convert physical implementation constraints into universal language limits.


---

70. Required Distributed Integration Test

A distributed-memory fixture must demonstrate that the program can express distributed memory without requiring:

N nodes
N devices
fixed network topology
fixed memory-bank topology
fixed cluster size

The realization is determined later.


---

71. Required AI/Data Integration Test

A fixture must demonstrate:

tensor/data
+
symbolic extent
+
memory requirement
+
resource capability

without encoding a particular:

GPU
VRAM size
tensor dimension limit
accelerator count


---

72. Required Test Categories

Memory tests must exist under the repository's canonical test architecture.

At minimum:

tests/
├── memory/
│   ├── positive/
│   ├── negative/
│   ├── boundary/
│   ├── scalability/
│   ├── determinism/
│   ├── roundtrip/
│   ├── compatibility/
│   └── cross-domain/

If the existing test organization uses a different established layout, preserve it rather than creating a competing test hierarchy.


---

73. Positive Tests

Positive tests must cover:

basic memory;

memory places;

memory spaces;

memory regions;

references;

ownership;

borrowing;

lifetimes;

allocation;

deallocation;

persistence;

shared memory;

distributed memory;

accelerator memory;

quantum-classical memory;

capabilities;

requirements;

constraints;

preferences;

hints;

symbolic sizes;

computed sizes;

qualified names;

open-world capability names.



---

74. Negative Tests

Negative tests must distinguish syntax errors from semantic errors.

Syntax tests include:

missing delimiters;

malformed paths;

malformed lifetime syntax;

malformed capability syntax;

malformed argument lists;

malformed annotations;

malformed memory declarations;

malformed property expressions.


Semantic-invalid programs should be tested separately so that parser failures are not confused with semantic-analysis failures.


---

75. Boundary Tests

Boundary tests must include:

empty optional constructs;

deeply nested expressions;

long qualified names;

many memory operations;

many regions;

many lifetime references;

many capabilities;

large symbolic expressions;

large generated programs;

complex cross-domain programs.


No boundary test may define an artificial language maximum.


---

76. Scalability Tests

Scalability tests must vary:

number of memory operations;

number of allocations;

number of regions;

number of references;

number of lifetimes;

number of capabilities;

symbolic memory extents;

distributed memory declarations;

accelerator memory declarations.


The expected property is:

> No grammar-level machine-capacity ceiling exists.



The test harness may of course be bounded by the test machine's resources.

That test-machine limitation must never become language semantics.


---

77. Hard-Coding Audit

Every memory grammar file must be audited for:

fixed memory capacities;

fixed address widths;

fixed region counts;

fixed allocation counts;

fixed lifetime counts;

fixed reference counts;

fixed device counts;

fixed node counts;

fixed memory-space counts;

fixed accelerator counts;

fixed topology;

fixed physical identifiers.


Each finding must be classified as:

1. language semantic requirement;


2. explicit program value;


3. target-specific requirement;


4. resource constraint;


5. implementation limitation;


6. test-only limitation;


7. accidental hard-coding.



Accidental hard-coding must be removed.


---

78. Important Distinction: Program Constants vs Machine Limits

This is valid:

let n = 1024;
allocate(n);

because 1024 is program data.

This is not valid as a universal language architecture:

MAX_MEMORY = 1024;

Likewise:

Tensor<Float, 1024, 1024>

may be valid program semantics.

But:

Tensor dimensions may never exceed 1024

is a prohibited universal grammar limitation.

The same rule applies to:

memory;

qubits;

CPUs;

cores;

threads;

GPUs;

FPGAs;

nodes;

tensor dimensions;

registers;

accelerators;

timelines.



---

79. Requirement vs Implementation Decision

This distinction is mandatory.

Requirement

requires memory capability memory::persistent;

Resource constraint

constraint memory::latency <= required_latency;

Preference

preference memory capability memory::local;

Hint

hint memory::reuse;

Implementation decision

map object -> physical_memory_bank(...);

The first four can be portable source intent.

The last belongs to a target-specific realization layer unless explicitly introduced by a target dialect.


---

80. Diagnostics

Memory grammar diagnostics must preserve:

source position;

offending token;

expected syntax;

relevant grammar context;

deterministic error behavior.


Semantic diagnostics belong downstream and must distinguish:

syntax error
type error
ownership error
borrow error
lifetime error
capability error
resource error
effect error
target realization error

The parser must not disguise semantic failures as syntax failures.


---

81. Generated Artifacts

ANTLR-generated files are derived artifacts.

They are not sources of truth.

The authoritative source is:

grammar/**/*.g4

Generated parser/lexer artifacts must be reproducible.

A clean build must be able to regenerate them deterministically.

Generated artifacts must not be manually edited as part of normal development.


---

82. ANTLR Composition

ANTLR grammar composition must follow the repository's established architecture.

The canonical production lexer is:

grammar/antlr/ZamaniLexer.g4

The canonical production parser is:

grammar/antlr/ZamaniParser.g4

Memory grammars are parser components.

They must not create another production lexer.

The composition model must remain compatible with ANTLR's grammar/import architecture.


---

83. No Parallel Memory Language

The memory directory must not become a separate language.

There must not be:

MemoryLanguage
MemoryParser
MemoryAST
MemoryIR
MemoryRuntime

as a competing language stack.

Instead:

Zamani
  |
  +-- memory domain

Memory is a domain of the same Zamani language.


---

84. Dialect Integration

Future memory technologies may require syntax extensions.

They should normally use:

grammar/dialects/

rather than modifying core memory syntax for every vendor or experimental feature.

A dialect must define:

name;

version;

owner;

syntax additions;

semantic meaning;

AST mapping;

IR mapping;

compatibility;

feature status.


A dialect must not silently redefine core memory semantics.


---

85. Interoperability

Foreign memory models belong to:

grammar/interoperability/

The memory grammar must not become a C/C++/Rust-specific grammar.

Foreign memory semantics must enter through explicit interoperability contracts.


---

86. Specification Integration

The memory grammar must remain consistent with:

grammar/DESIGN.md
grammar/README.md
grammar/specification/
grammar/spec/
grammar/grammar.md
grammar/Zamani-Grammar.md

The authority hierarchy is:

DESIGN.md
    |
    v
normative specification
    |
    v
canonical grammar
    |
    v
implementation-conformance reference

Zamani-Grammar.md is not allowed to silently introduce permanent syntax.


---

87. Implementation Status

The README describes the production contract.

It does not by itself make every .g4 file production-complete.

A memory grammar file is implemented only when:

syntax
+
lexer compatibility
+
ANTLR composition
+
AST mapping
+
semantic mapping
+
IR mapping
+
diagnostics
+
tests
+
hard-coding audit
+
compatibility

have all been satisfied.

The implementation-conformance state must be reflected in:

grammar/grammar.md

using the repository's status vocabulary:

SPECIFIED
IMPLEMENTED
PARTIALLY IMPLEMENTED
PLANNED
DEPRECATED


---

88. Per-File Independent Completion Contract

Every memory .g4 file must be independently completable.

Before marking a file complete, the author must be able to answer all of these questions without waiting for another file to be redesigned:

File:
Purpose:
Status:

Owns:
Does not own:

Existing tokens used:
New tokens required:
Why each new token is necessary:

Grammar entry points:
Public rules:
Internal rules:

Dependencies:
Imported grammars:
Consumed canonical rules:

AST mapping:
Semantic mapping:
Canonical IR mapping:

Compiler consumers:
Runtime consumers:
Resource consumers:
Hardware consumers:

Cross-domain consumers:

Positive tests:
Negative tests:
Boundary tests:
Scalability tests:
Determinism tests:
Round-trip tests:
Compatibility tests:

Hard-coding audit:
Security audit:
Diagnostics audit:

Completion criteria:

This contract exists specifically to prevent the workflow:

finish file A
    |
    v
change file B
    |
    v
return to file A
    |
    v
rewrite file A

Instead, the integration contract must be established before the file is declared complete.


---

89. Completion Contract: memory.g4

Complete only when:

canonical memory foundation exists;

memory places are defined;

memory operations are defined;

memory spaces are represented;

memory regions are represented;

canonical names are consumed;

canonical expressions are consumed;

canonical types are consumed;

memory capability integration exists;

resource integration exists;

no semantic analysis occurs in grammar;

no machine limit exists;

no physical address is required;

no physical device is selected;

no duplicate IR exists;

positive tests exist;

negative tests exist;

boundary tests exist;

scalability tests exist;

cross-domain tests exist.



---

90. Completion Contract: ownership.g4

Complete only when:

ownership syntax is defined;

memory composition works;

canonical type syntax is consumed;

ownership information reaches the AST;

ownership analysis remains downstream;

no allocator assumptions exist;

no machine limits exist;

positive/negative tests pass;

cross-domain tests pass.



---

91. Completion Contract: borrowing.g4

Complete only when:

borrow syntax is defined;

mutable borrowing is defined if standardized;

lifetime references compose;

expressions/places compose;

source spans are preserved;

malformed syntax is rejected;

borrow checking remains downstream;

scalability limits are absent;

nested/complex expressions are tested.



---

92. Completion Contract: lifetimes.g4

Complete only when:

symbolic lifetime identifiers work;

lifetime annotations compose with types;

lifetime references compose with borrowing;

lifetime relationships are representable;

physical time is never assumed;

no fixed lifetime count exists;

deterministic parsing is verified.



---

93. Completion Contract: references.g4

Complete only when:

source-level references are represented;

canonical expressions are consumed;

canonical types are consumed;

places are structurally preserved;

reference semantics remain downstream;

no target pointer representation is encoded;

no fixed address width exists.



---

94. Completion Contract: allocation.g4

Complete only when:

allocation intent is representable;

symbolic sizes work;

computed sizes work;

memory spaces work;

memory regions work;

capabilities can be associated;

resource intent can be associated;

allocator selection remains downstream;

physical address is not required;

no capacity limit exists.



---

95. Completion Contract: deallocation.g4

Complete only when:

release intent is representable;

ownership/lifetime integration exists;

explicit release remains semantically optional where appropriate;

physical reclamation is downstream;

allocator implementation is not encoded.



---

96. Completion Contract: regions.g4

Complete only when:

region declarations/references are defined;

ownership/lifetime integration works;

regions remain abstract;

no physical topology is implied;

no region-count limit exists.



---

97. Completion Contract: address-spaces.g4

Complete only when:

abstract address spaces are representable;

canonical names are consumed;

target-specific spaces can be extended;

physical addresses are not required;

address width is not hard-coded;

resource/hardware interpretation remains downstream.



---

98. Completion Contract: shared-memory.g4

Complete only when:

shared-memory intent is representable;

ownership integration exists;

concurrency integration exists;

synchronization can be expressed through the correct subsystem;

no cache topology is encoded;

no coherence protocol is hard-coded;

no participant-count limit exists.



---

99. Completion Contract: distributed-memory.g4

Complete only when:

distributed memory intent is representable;

symbolic placement is possible;

resource requirements are possible;

no fixed node count exists;

no fixed topology exists;

distributed execution remains separately owned;

scalability tests pass.



---

100. Completion Contract: accelerator-memory.g4

Complete only when:

accelerator-memory intent is representable;

accelerator capabilities are open-ended;

CPU/GPU/FPGA/QPU identities are not hard-coded;

no device count is hard-coded;

resource negotiation remains downstream;

target selection remains downstream.



---

101. Completion Contract: quantum-memory.g4

Complete only when:

quantum-classical memory intent is representable;

quantum syntax composes;

no duplicate quantum IR exists;

quantum::ir remains canonical;

QEC remains outside the memory grammar;

ZQN remains outside the memory grammar;

physical qubit allocation remains downstream.



---

102. Completion Contract: persistence.g4

Complete only when:

persistence intent is representable;

durability semantics are distinguishable;

resource capability integration exists;

storage implementation remains downstream;

no storage technology is required by portable syntax.



---

103. Completion Contract: memory-capabilities.g4

Complete only when:

Capabilities remains canonical;

ResourceCapabilities remains canonical;

Expressions remains canonical;

Memory remains canonical;

capability identity is not duplicated;

capability names remain open-world;

requirements are distinct from constraints;

preferences are distinct from hints;

availability is distinct from requirement;

assertions are distinct from implementation;

implications/exclusions remain semantic relationships;

capability subject association remains abstract;

every intended public rule is reachable;

no fixed capability enumeration exists;

no physical device is selected;

no resource is allocated;

no hardware discovery occurs;

positive tests exist;

negative tests exist;

scalability tests exist.



---

104. Completion Contract: memory-constraints.g4

Complete only when:

requirements are distinct;

constraints are distinct;

preferences are distinct;

hints are distinct;

expressions are canonical;

resource integration exists;

capabilities integrate correctly;

no physical target is selected;

no machine capacity is hard-coded.



---

105. Required End-to-End Pipeline

The memory subsystem is production-ready only when the complete chain works:

Zamani source
      |
      v
ZamaniLexer
      |
      v
ZamaniParser
      |
      v
memory grammar
      |
      v
domain-neutral AST
      |
      v
type analysis
      |
      v
ownership analysis
      |
      v
borrow/lifetime analysis
      |
      v
effect analysis
      |
      v
resource/capability analysis
      |
      v
canonical semantic representation
      |
      +--------------------+
      |                    |
      v                    v
 classical             quantum::ir
      |                    |
      +---------+----------+
                |
                v
          optimization
                |
        +-------+-------+
        |       |       |
        v       v       v
     routing scheduling resilience
                        |
                        v
                       ZQN
                        |
                        v
                       HAL
                        |
                        v
               target realization


---

106. Production Test Matrix

The final memory subsystem must be tested across:

Category	Required

Lexical	Yes
Syntax	Yes
AST	Yes
Semantic	Yes
Positive	Yes
Negative	Yes
Boundary	Yes
Scalability	Yes
Determinism	Yes
Round-trip	Yes
Compatibility	Yes
Classical integration	Yes
Quantum integration	Yes
Hybrid integration	Yes
HDL integration	Yes
Hardware integration	Yes
Distributed integration	Yes
AI integration	Yes
Data integration	Yes
Concurrency integration	Yes
Effects integration	Yes
Resource integration	Yes
Security integration	Yes
Interoperability integration	Yes



---

107. Production Readiness Checklist

Architecture

[ ] One authoritative memory grammar architecture.

[ ] memory.g4 remains the memory foundation.

[ ] ZamaniParser.g4 remains the parser composition root.

[ ] ZamaniLexer.g4 remains the production lexer.

[ ] No competing memory language exists.

[ ] No duplicate memory IR exists.


Lexer

[ ] Existing canonical tokens are reused.

[ ] No duplicate memory lexer exists.

[ ] New tokens are justified and centrally owned.

[ ] Keyword additions are minimized.

[ ] Identifier extensibility is preserved.


Memory semantics

[ ] Memory places are defined.

[ ] Memory spaces are defined.

[ ] Memory regions are defined.

[ ] References are defined.

[ ] Ownership is defined.

[ ] Borrowing is defined.

[ ] Lifetimes are defined.

[ ] Allocation intent is defined.

[ ] Deallocation intent is defined.

[ ] Shared memory is defined.

[ ] Distributed memory is defined.

[ ] Accelerator memory is defined.

[ ] Quantum-classical memory is defined.

[ ] Persistence is defined.

[ ] Memory capabilities are defined.

[ ] Memory constraints are defined.


Integration

[ ] Core names integrate.

[ ] Expressions integrate.

[ ] Types integrate.

[ ] Effects integrate.

[ ] Concurrency integrates.

[ ] Resources integrate.

[ ] Hardware integrates.

[ ] Classical integrates.

[ ] Quantum integrates.

[ ] Hybrid integrates.

[ ] HDL integrates.

[ ] Distributed computing integrates.

[ ] AI/data integrate.

[ ] Compiler integrates.

[ ] Runtime integrates.


Quantum

[ ] quantum::ir remains canonical.

[ ] No MemoryQuantumIR.

[ ] No QuantumMemoryIR.

[ ] No duplicated quantum gate model.

[ ] QEC remains downstream.

[ ] ZQN remains downstream.

[ ] Routing remains downstream.

[ ] Physical qubit allocation remains downstream.


POCO-REAF

[ ] No fixed memory capacity.

[ ] No fixed allocation count.

[ ] No fixed region count.

[ ] No fixed reference count.

[ ] No fixed lifetime count.

[ ] No fixed memory-space count.

[ ] No fixed node count.

[ ] No fixed device count.

[ ] No fixed topology.

[ ] No physical address requirement.

[ ] No fixed address width.

[ ] No hardware-specific source dependency.


Capability architecture

[ ] core/capabilities.g4 owns capability identity.

[ ] resources/capabilities.g4 owns resource capability relationships.

[ ] memory/memory-capabilities.g4 owns memory-specific capability association.

[ ] No closed capability enumeration.

[ ] Requirements differ from capabilities.

[ ] Constraints differ from preferences.

[ ] Preferences differ from hints.

[ ] Capability resolution remains downstream.


Safety

[ ] Rust 1.97 compatible.

[ ] Rust 1.97.1 compatible.

[ ] Rust 2021.

[ ] No unsafe.

[ ] No embedded Rust actions in grammar.

[ ] Security authorization remains downstream.


Validation

[ ] Positive tests.

[ ] Negative tests.

[ ] Boundary tests.

[ ] Scalability tests.

[ ] Determinism tests.

[ ] Round-trip tests.

[ ] Compatibility tests.

[ ] Cross-domain tests.

[ ] Hard-coding audit.

[ ] ANTLR grammar validation.

[ ] Parser composition validation.

[ ] AST coverage.

[ ] Semantic coverage.

[ ] IR coverage.



---

108. Definition of Done

grammar/memory/ is not complete merely because every .g4 file exists.

The directory is complete only when:

Specification
     +
Lexer
     +
Parser
     +
AST
     +
Semantic Analysis
     +
Resource/Capability Analysis
     +
Canonical IR
     +
Compiler
     +
Runtime
     +
Tests
     +
Compatibility
     +
Scalability Audit

form one traceable system.

Every memory construct must have a known path:

source syntax
    |
    v
grammar rule
    |
    v
AST representation
    |
    v
semantic representation
    |
    v
canonical IR
    |
    v
compiler consumer
    |
    v
runtime/hardware realization

There must be no:

grammar rule
    |
    v
??? unknown semantic meaning

and no:

grammar
    |
    v
target-specific implementation

shortcut.


---

109. Final Architectural Rule

The memory grammar exists to express:

WHAT memory semantics mean
WHAT memory properties are required
WHAT capabilities are required
WHAT constraints apply
WHAT preferences are desired
WHAT hints may help realization

It does not permanently encode:

WHICH CPU
WHICH GPU
WHICH FPGA
WHICH ASIC
WHICH QPU
WHICH NUMA NODE
WHICH MEMORY BANK
WHICH PHYSICAL ADDRESS
WHICH CACHE
WHICH ALLOCATOR
WHICH DEVICE
WHICH NODE

Those decisions belong to later stages.

The fundamental Zamani architecture is therefore:

Zamani Source
                      |
                      v
                Memory Grammar
                      |
                      v
                Domain-Neutral AST
                      |
                      v
              Semantic Analysis
                      |
          +-----------+-----------+
          |           |           |
          v           v           v
        Types      Resources    Effects
          |           |           |
          +-----------+-----------+
                      |
                      v
             Canonical Semantic Model
                      |
          +-----------+-----------+
          |                       |
          v                       v
    Classical IR             quantum::ir
          |                       |
          +-----------+-----------+
                      |
                      v
               Optimization
                      |
          +-----------+-----------+
          |           |           |
          v           v           v
       Routing    Scheduling   Resilience
                                  |
                                  v
                                 ZQN
                                  |
                                  v
                                 HAL
                                  |
                                  v
                         Target Realization
                                  |
            +----------+----------+----------+
            |          |          |          |
           CPU        GPU        FPGA       QPU
            |          |          |          |
            +----------+----------+----------+
                                  |
                                  v
                           Future Targets

The memory grammar is therefore an open, target-independent semantic syntax layer—not a description of a particular memory architecture.

That is the memory-domain contract required for Zamani to preserve POCO-REAF while scaling from the smallest machine to arbitrarily large systems subject only to actual program semantics, capabilities, policies, and available resources.

I also found one concrete issue that this README now makes explicit: the current `memory-capabilities.g4` contains `memoryCapabilitySubjectPropertyAssertion`, but it is not currently part of the `memoryCapabilityItem` dispatcher. That should be resolved in the grammar file when we harden it; the README now makes the “every intended public rule must be reachable” invariant explicit rather than allowing that drift to remain hidden.0
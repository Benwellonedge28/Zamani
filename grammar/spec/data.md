Zamani Data Semantics Specification

Path: "grammar/spec/data.md"
Language: Zamani
Specification role: Normative production data-semantics contract
Specification status: Production / normative
Specification version: 1.0
Grammar technology: ANTLR4-compatible parser grammars
Implementation baseline: Rust 1.97 / Rust 1.97.1
Rust edition: Rust 2021
Rust safety: Safe Rust only; "unsafe" Rust is prohibited
Primary portability principle: Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)
Scalability principle: No artificial language-level data-size, cardinality, dimension, partition, replica, node, stream, or throughput ceiling
Canonical quantum boundary: "quantum::ir"
Primary syntax package: "grammar/data/"

---

0. Document Contract

0.1 Purpose

This document defines the normative semantic contract for data in the Zamani programming language.

It defines the meaning of:

- data values;
- data types as consumed by the data subsystem;
- schemas;
- records;
- collections;
- sequences;
- streams;
- datasets;
- sources;
- sinks;
- views;
- transformations;
- queries;
- pipelines;
- joins;
- grouping;
- aggregation;
- windows;
- partitioning;
- distribution;
- replication;
- consistency;
- materialization;
- serialization;
- deserialization;
- validation;
- provenance;
- lineage;
- data movement;
- data availability;
- data ownership;
- data lifetime;
- data resource requirements;
- data capabilities;
- data constraints;
- data preferences;
- data hints;
- data failures;
- determinism;
- ordering;
- reproducibility;
- incremental computation;
- checkpointing;
- data interoperability;
- classical/quantum data boundaries;
- AI/ML data;
- scientific data;
- distributed data;
- hardware/accelerator data;
- future data domains.

The specification is deliberately independent of any particular:

- CPU;
- GPU;
- FPGA;
- ASIC;
- QPU;
- accelerator;
- memory technology;
- filesystem;
- database;
- object store;
- cloud provider;
- network;
- cluster;
- storage device;
- vendor API;
- physical address;
- machine topology.

The data model describes semantic data and computation intent.

It does not prescribe the physical realization.

---

1. Authority and Repository Integration

1.1 Authority hierarchy

The authoritative relationship is:

grammar/DESIGN.md
        │
        ▼
grammar/specification/language.md
        │
        ├── grammar/spec/lexical.md
        ├── grammar/spec/syntax.md
        ├── grammar/spec/type-system.md
        ├── grammar/spec/semantics.md
        ├── grammar/spec/effects.md
        ├── grammar/spec/resources.md
        └── grammar/spec/data.md
                │
                ▼
        grammar/data/*.g4
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
        structural validation
                │
                ▼
        semantic analysis
                │
                ▼
        canonical semantic model
                │
        ┌───────┼────────┐
        ▼       ▼        ▼
   classical quantum   HDL/
      IR       ::ir    hardware IR
        │       │        │
        └───────┼────────┘
                ▼
          optimization
                │
       ┌────────┼────────┐
       ▼        ▼        ▼
    routing  scheduling resilience
       │        │        │
       └────────┼────────┘
                ▼
              ZQN
                │
               HAL
                │
                ▼
         target realization
                │
                ▼
             runtime

This document is authoritative for data semantics.

It does not replace:

- "grammar/spec/lexical.md";
- "grammar/spec/syntax.md";
- "grammar/spec/type-system.md";
- "grammar/spec/semantics.md";
- "grammar/spec/effects.md";
- "grammar/spec/resources.md";
- "grammar/spec/compatibility.md".

The current repository already establishes "type-system.md" as the normative type-system contract and "semantics.md" as the normative overall semantic contract. The data specification therefore specializes those contracts rather than redefining them.

---

2. Ownership

2.1 This file owns

This file owns:

- semantic meaning of data abstractions;
- data invariants;
- logical data identity;
- logical data equality;
- schema semantics;
- record semantics;
- collection semantics;
- sequence semantics;
- stream semantics;
- source/sink semantics;
- transformation semantics;
- query semantics;
- pipeline semantics;
- ordering semantics;
- cardinality semantics;
- partition semantics;
- replication semantics;
- distribution semantics;
- consistency semantics;
- materialization semantics;
- serialization intent semantics;
- validation semantics;
- provenance semantics;
- lineage semantics;
- data movement semantics;
- data ownership/lifetime semantics;
- data failure semantics;
- data determinism;
- reproducibility;
- incremental computation semantics;
- checkpoint semantics;
- data resource intent;
- data capability requirements;
- data interoperability;
- cross-domain data boundaries;
- data-specific diagnostics;
- data-specific conformance requirements.

2.2 This file does not own

It does not own:

- lexer tokens;
- lexical spelling;
- general expression precedence;
- general type formation;
- general effects;
- general resource semantics;
- parser implementation;
- AST storage layout;
- runtime memory allocation;
- physical storage layout;
- database implementation;
- filesystem implementation;
- network transport implementation;
- serialization codec implementation;
- compression implementation;
- scheduling algorithms;
- routing algorithms;
- optimization algorithms;
- hardware discovery;
- accelerator discovery;
- quantum hardware;
- QEC implementation;
- ZQN implementation;
- HAL implementation;
- physical qubit mapping;
- physical node placement;
- vendor-specific APIs;
- target-specific machine limits.

---

3. Existing Data Grammar Integration

The repository currently provides:

grammar/data/
├── README.md
├── data.g4
├── schemas.g4
├── records.g4
├── collections.g4
├── streams.g4
├── serialization.g4
└── transformations.g4

These files are retained.

No unnecessary renaming is required.

The current "data.g4" explicitly describes itself as the data grammar's integration point, while "transformations.g4" establishes a dedicated transformation grammar and explicitly avoids provider-specific or hardware-specific execution semantics.

The normative ownership is:

File| Owns
"grammar/data/data.g4"| Data package composition and public integration entry points
"grammar/data/schemas.g4"| Schema syntax
"grammar/data/records.g4"| Record syntax
"grammar/data/collections.g4"| Collection syntax
"grammar/data/streams.g4"| Stream syntax
"grammar/data/serialization.g4"| Serialization/deserialization syntax
"grammar/data/transformations.g4"| Transformation/query syntax
"grammar/spec/data.md"| Semantic meaning and cross-file contracts
"grammar/data/README.md"| Package architecture and navigation

No specialized grammar file may silently redefine semantics that belong here.

---

4. Production Data Model

A Zamani data value is a semantic value independent of its physical representation.

Conceptually:

DataValue =
    Scalar
  | Record
  | Collection
  | Sequence
  | Stream
  | Tensor
  | Dataset
  | StructuredValue
  | OpaqueValue
  | DomainValue

A physical implementation MAY represent the same semantic value using:

- registers;
- stack memory;
- heap memory;
- contiguous buffers;
- segmented buffers;
- distributed storage;
- database records;
- object storage;
- device memory;
- accelerator memory;
- quantum-classical host memory;
- compressed representation;
- encoded representation;
- lazy computation;
- materialized representation.

The representation is not itself part of the source-level semantic identity unless explicitly made observable through a language contract.

---

5. Data Identity

Every data value has a semantic identity determined by its language-visible meaning.

Identity MUST NOT depend on:

- pointer address;
- memory address;
- process ID;
- thread ID;
- machine ID;
- physical device ID;
- database row address;
- filesystem inode;
- network location;
- compiler allocation order.

An implementation MAY maintain internal identifiers such as:

DataId
SchemaId
RecordId
DatasetId
StreamId
PipelineId
PartitionId
LineageId
ProvenanceId

Those are implementation identities unless explicitly exposed through a portable semantic abstraction.

---

6. Data Equality

Zamani distinguishes:

semantic equality
representation equality
identity equality
ordering equality

Semantic equality means that two values have the same language-defined meaning.

Representation equality means their physical representations are identical.

Identity equality means they refer to the same logical entity when the applicable abstraction has identity.

These MUST NOT be conflated.

For example:

[1, 2, 3]

and a segmented representation containing the same logical sequence may be semantically equal without having the same physical representation.

---

7. Data Schema Semantics

A schema defines logical structure.

A schema may define:

- fields;
- field names;
- field types;
- optionality;
- nullability;
- defaults;
- constraints;
- keys;
- uniqueness;
- derived fields;
- annotations;
- semantic metadata;
- version information.

A schema does not inherently define:

- database tables;
- memory layout;
- file layout;
- packet layout;
- physical alignment;
- storage blocks;
- indexes;
- network locations.

Those are downstream realizations.

---

8. Schema Evolution

Schema evolution MUST support:

- additive changes;
- compatible field changes;
- explicit breaking changes;
- field renaming;
- field removal;
- type evolution;
- default introduction;
- constraint evolution;
- version migration.

Schema compatibility MUST be explicitly classified.

At minimum:

compatible
conditionally-compatible
breaking
unknown

A compiler/runtime MUST NOT silently treat a breaking schema change as compatible.

---

9. Records

A record is a logical product of named fields.

For:

record Person {
    name: String;
    age: Integer;
}

the semantic record contains:

name : String
age  : Integer

A record is not:

- a CPU register;
- an HDL register;
- a quantum register;
- a database row by definition.

There is no language-level universal maximum for:

- field count;
- nesting;
- record count;
- record size.

Implementation limits are resource limits, not semantic limits.

---

10. Collections

A collection is a logical grouping of values.

Collections may be:

- ordered;
- unordered;
- unique;
- non-unique;
- mutable;
- immutable;
- lazy;
- eager;
- persistent;
- ephemeral.

The semantic contract MUST distinguish collection properties.

For example:

ordered

means iteration order is semantically relevant.

unordered

means no particular order is semantically guaranteed.

An implementation may choose any representation preserving the declared semantics.

---

11. Sequences

A sequence is an ordered logical data abstraction.

Its semantic properties include:

- element type;
- logical order;
- cardinality;
- indexing semantics;
- iteration semantics.

A sequence may be physically represented as:

- an array;
- linked structure;
- segmented storage;
- distributed partitions;
- generated values;
- lazy computation;
- persistent storage.

The source program does not need to know which representation is selected.

---

12. Cardinality

Cardinality is a semantic property.

A cardinality MAY be:

- zero;
- finite;
- symbolic;
- parameterized;
- runtime-determined;
- unbounded where the abstraction permits it.

The grammar and semantic model MUST NOT impose artificial maxima.

The language must distinguish:

semantic cardinality

from:

physical capacity

For example:

dataset.length

may have a mathematically meaningful result while a particular implementation may fail because available resources are insufficient.

That is a resource failure, not necessarily a semantic failure.

---

13. Unbounded and Streaming Data

A stream may be:

- bounded;
- unbounded;
- finite but unknown;
- replayable;
- non-replayable;
- ordered;
- unordered;
- lossless;
- lossy.

An unbounded stream does not mean an implementation must allocate infinite storage.

It means the semantic source is not required to have a finite predetermined end.

Execution may use:

- incremental processing;
- windows;
- checkpoints;
- bounded state;
- external persistence;
- backpressure;
- distributed processing.

---

14. Stream Ordering

Ordering is explicit.

A stream declared ordered MUST preserve the specified semantic order.

An unordered stream MUST NOT acquire accidental ordering guarantees merely because a particular implementation happens to preserve order.

This prevents programs from depending on implementation accidents.

---

15. Stream Replayability

A replayable stream permits semantically equivalent re-observation according to its source contract.

A non-replayable stream may expose each element only according to its consumption semantics.

Replayability is distinct from persistence.

A persistent stream may still be non-replayable if its semantic contract does not expose replay.

---

16. Lossless and Lossy Data

A lossless data path MUST preserve all data required by the semantic contract.

A lossy path MAY discard information according to explicitly defined semantics.

Loss must never be silently introduced by:

- scheduling;
- optimization;
- distribution;
- serialization;
- backend lowering.

If loss is semantically observable, it must be represented explicitly.

---

17. Sources

A data source produces logical data.

A source may represent:

- local data;
- generated data;
- persistent data;
- external data;
- network data;
- sensor data;
- experiment data;
- quantum measurement data;
- AI datasets;
- distributed data.

A source declaration does not inherently select:

- filesystem;
- database;
- network provider;
- device;
- machine.

The source identity must remain abstract unless a program explicitly requires a concrete external resource.

---

18. Sinks

A sink consumes logical data.

Examples include:

- files;
- databases;
- streams;
- services;
- devices;
- external APIs;
- experiment systems.

The language-level sink contract describes the semantic destination.

Physical placement and implementation are downstream.

---

19. Endpoint Semantics

An endpoint may be:

logical
symbolic
named
URI-like
capability-selected
resource-selected
implementation-resolved

Endpoint syntax MUST NOT require a particular provider.

A source such as:

from "logical://dataset/customer"

does not imply a particular cloud, database, filesystem, or machine.

---

20. Data Transformations

A transformation maps one logical data value to another.

Conceptually:

T : InputData → OutputData

Transformations include:

- map;
- flat-map;
- filter;
- reduce;
- fold;
- scan;
- group;
- aggregate;
- partition;
- repartition;
- sort;
- distinct;
- project;
- rename;
- derive;
- cast;
- reshape;
- flatten;
- explode;
- collect;
- join;
- union;
- intersection;
- difference;
- concatenate;
- sample;
- batch;
- limit;
- take;
- drop;
- window.

The existing transformation grammar already provides these categories and intentionally keeps execution placement outside grammar ownership.

---

21. Transformation Purity

A transformation MAY be:

pure
effectful
stateful
nondeterministic
resource-dependent
externally dependent

The classification is determined through the general effect system.

Data grammar MUST NOT create a second effect system.

For example:

map

is syntactic transformation intent.

Whether the supplied lambda performs I/O, mutation, randomness, quantum measurement, or network access is determined by semantic/effect analysis.

---

22. Map

For:

map(data, f)

the semantic meaning is application of "f" to each logical element according to the source ordering and transformation semantics.

The compiler may implement mapping through:

- sequential execution;
- vectorization;
- SIMD;
- GPU execution;
- FPGA execution;
- distributed execution;
- quantum-assisted computation where semantically valid;
- future accelerators.

The transformation semantics remain unchanged.

---

23. Filter

For:

filter(data, predicate)

elements satisfying the predicate are retained.

Ordering follows the source and filter semantics.

Filtering MUST NOT introduce accidental reordering.

---

24. Reduce and Fold

"reduce" combines elements according to the declared reduction operation.

"fold" additionally provides an initial accumulator.

A reduction is safely parallelizable only when its semantic operation permits the corresponding reassociation.

The compiler MUST NOT reorder a non-associative or otherwise order-sensitive operation merely for performance.

---

25. Scan

A scan preserves intermediate accumulation states.

This distinguishes it semantically from reduce.

If:

scan([a,b,c], init, f)

is evaluated, the observable sequence of intermediate states is part of the result.

---

26. Grouping

Grouping forms logical groups according to a key expression.

The grammar does not define:

- hash-table implementation;
- sort implementation;
- partition count;
- node placement;
- memory layout.

Those belong to downstream implementation.

---

27. Partitioning

Partitioning expresses a logical decomposition.

For example:

partition(data by key)

means values are logically grouped according to "key".

It does not mean:

16 partitions
64 machines
node 0
node 1

unless such quantities are explicitly part of a program-level semantic requirement.

Even then, those quantities constrain execution; they do not become language-wide limits.

---

28. Repartitioning

Repartitioning changes the logical distribution organization of data.

It may be used for:

- joins;
- grouping;
- load balancing;
- distributed computation;
- locality;
- fault tolerance.

The compiler determines the physical realization.

---

29. Distribution

Distributed data is semantically one logical data abstraction even when physically represented in many places.

A distributed collection MUST preserve its declared:

- identity;
- type;
- ordering;
- consistency;
- visibility;
- failure semantics.

Physical distribution is not semantic duplication unless replication is explicitly defined.

---

30. Replication

Replication creates multiple physical representations of logical data.

Replication semantics must distinguish:

logical identity
physical copies
consistency
visibility
failure behavior

The language MUST NOT assume a universal replica count.

A program may express a requirement such as:

replication >= r

where "r" is a program-level value.

That is different from establishing:

MAX_REPLICAS = 3

as a language limitation.

---

31. Consistency

Consistency describes observable relationships among views of shared data.

The specification must permit future consistency models without requiring one universal implementation.

Possible semantic categories include:

- strong;
- causal;
- session;
- eventual;
- monotonic;
- application-defined.

A particular model becomes part of a program's semantics only when selected or required.

---

32. Concurrency and Data Races

Data operations participating in concurrent execution MUST obey the general ownership, memory, type, and effect systems.

The data specification does not create a second concurrency model.

Mutable shared data requires the applicable synchronization/ownership semantics.

An implementation MUST NOT introduce observable data races merely because it parallelizes a transformation.

---

33. Determinism

A data computation is deterministic when equal semantic inputs under equal semantic environment conditions produce equal observable results.

Determinism must account for:

- ordering;
- floating-point policy;
- randomness;
- external state;
- concurrency;
- distributed execution;
- time;
- quantum measurement;
- network input.

Parallelization MUST preserve deterministic semantics when the source computation is specified as deterministic.

---

34. Floating-Point Data

Floating-point operations inherit their semantics from "grammar/spec/type-system.md" and "grammar/spec/semantics.md".

The data layer MUST NOT silently alter:

- precision;
- rounding;
- NaN behavior;
- infinity behavior;
- comparison;
- reproducibility.

A compiler MAY use a faster representation only when it preserves the applicable semantic contract.

---

35. Numeric Scalability

Data semantics MUST support:

- fixed-width numeric values;
- arbitrary-width integers where supported by the type system;
- symbolic quantities;
- parameterized dimensions;
- runtime dimensions.

The data specification MUST NOT define universal limits based on:

u32
u64
usize

or any host integer representation.

"usize" may be used internally for indexing host collections, but it MUST NOT define Zamani's semantic cardinality.

This is consistent with the existing type-system requirement that semantic quantities not silently overflow merely because a host implementation uses a bounded indexing type.

---

36. Tensor and Multidimensional Data

Tensor data is represented using the canonical type/shape system.

The data grammar MUST NOT define a fixed maximum rank.

Valid semantic dimensions may be:

- literal;
- symbolic;
- generic;
- dependent;
- runtime-known;
- dynamically determined.

A tensor's:

logical shape
element type
layout intent
storage representation

must remain distinguishable.

---

37. Sparse Data

Sparse representations are semantic data abstractions when explicitly selected.

A sparse matrix/tensor MUST preserve its mathematical or logical meaning independently of whether it is implemented using:

- CSR;
- CSC;
- COO;
- blocked storage;
- compressed representation;
- distributed storage;
- accelerator-specific storage.

Representation changes are valid only when semantics are preserved.

---

38. Lazy Data

A lazy data value represents deferred computation.

Lazy evaluation MUST preserve observable semantics.

The implementation may choose:

- eager evaluation;
- lazy evaluation;
- partial evaluation;
- memoization;

only where the effect and observability contracts permit the transformation.

An effectful computation MUST NOT be silently duplicated merely because its data expression appears reusable.

---

39. Materialization

Materialization converts a logical computation/value into a retained representation.

Materialization does not inherently mean:

- RAM;
- disk;
- database;
- object storage;
- GPU memory;
- FPGA memory;
- QPU memory.

The target system chooses an appropriate representation based on:

- semantic requirements;
- resource availability;
- capabilities;
- compiler policy;
- runtime policy.

---

40. Serialization

Serialization maps a logical value into an external representation.

Conceptually:

serialize : T → Representation
deserialize : Representation → Result<T, Error>

Serialization MUST define:

- schema compatibility;
- encoding identity;
- version;
- failure semantics;
- determinism where required;
- preservation guarantees.

Serialization implementation is outside the grammar.

---

41. Canonical Serialization

A canonical serialization profile MAY be defined for values requiring:

- hashing;
- signatures;
- reproducible builds;
- provenance;
- deterministic interchange;
- content-addressed storage.

Canonical serialization MUST define ordering and representation rules sufficiently to avoid accidental implementation-dependent output.

---

42. Data Validation

Validation may occur at:

compile time
construction time
transformation time
materialization time
deserialization time
runtime

Validation failures must be represented through the language's explicit error/result/effect mechanisms.

Invalid external data MUST NOT silently become valid semantic data through implicit coercion.

---

43. Constraints

A data constraint expresses a semantic property.

Examples:

field > 0

length(data) >= n

key is unique

Constraints are not automatically execution instructions.

A compiler may use proven constraints for optimization, but an optimization must preserve semantics.

---

44. Data Contracts

A data contract defines expected properties between producers and consumers.

A contract may cover:

- schema;
- type;
- cardinality;
- ordering;
- nullability;
- constraints;
- provenance;
- version;
- consistency;
- failure behavior.

Contract violations must have defined diagnostics and runtime behavior.

---

45. Provenance

Provenance describes where a logical value came from.

Provenance MAY include:

- source identity;
- transformation identity;
- input lineage;
- schema version;
- execution context;
- declared parameters;
- relevant environment information.

Provenance MUST NOT require exposing secrets.

Sensitive credentials, authentication tokens, private keys, or equivalent secrets MUST NOT become automatic provenance data.

---

46. Lineage

Lineage describes dependency relationships between outputs and inputs.

For example:

output
  ← transformation
      ← dataset A
      ← dataset B

Lineage is distinct from provenance:

lineage = dependency relationship
provenance = origin/context information

The two may be represented separately.

---

47. Incremental Computation

A transformation MAY support incremental evaluation.

If supported, the semantic model must distinguish:

full computation
incremental update
checkpoint
recomputation

Incremental execution is valid only when it preserves the declared semantics.

The compiler may choose incremental execution automatically where equivalence is proven.

---

48. Checkpointing

Checkpointing captures sufficient state to resume computation according to the applicable execution contract.

A checkpoint does not necessarily imply:

- filesystem storage;
- database storage;
- one specific serialization format.

Checkpoint semantics belong to execution/runtime contracts.

Data grammar merely provides any required source-level intent.

---

49. Fault and Failure Semantics

Data operations may fail because of:

- invalid data;
- unavailable source;
- unavailable sink;
- schema mismatch;
- resource exhaustion;
- capability mismatch;
- network failure;
- storage failure;
- serialization failure;
- consistency failure;
- cancellation;
- timeout;
- external failure.

Failures MUST be distinguishable where the program can meaningfully react to them.

A physical resource shortage must not be misclassified as a type error.

The existing resource specification already distinguishes semantic requirements from resource realization and allows resource-unsatisfied execution to be reported separately.

---

50. Resource Integration

Data operations may declare resource intent.

Examples:

requires capability("stream.processing")

requires capability("distributed.data")

requires capability("persistent.storage")

requires memory(required_memory)

The resource system owns the meaning of resource requirements.

The data specification merely identifies where data operations consume those semantics.

The distinction MUST remain:

requirement
constraint
preference
hint
implementation decision

---

51. Resource Availability

A valid program may be impossible to execute on a particular target because resources are insufficient.

That does not invalidate the source program.

The implementation should distinguish:

SEMANTICALLY_INVALID

from:

RESOURCE_UNSATISFIABLE

and:

CAPABILITY_UNAVAILABLE

and:

EXECUTION_FAILURE

---

52. POCO-REAF Data Contract

POCO-REAF means a data program should not require source rewriting merely because its available realization changes.

The following must remain portable:

data semantics
schema
transformations
queries
logical partitioning
logical distribution
logical replication
logical resource intent

The following are downstream realization decisions:

CPU
GPU
FPGA
ASIC
QPU
memory hierarchy
node placement
storage location
network route
thread count
worker count
partition placement
physical replica placement

---

53. No Artificial Universal Limits

The language MUST NOT define:

MAX_RECORDS
MAX_FIELDS
MAX_COLLECTION_SIZE
MAX_STREAM_LENGTH
MAX_DATASET_SIZE
MAX_TENSOR_RANK
MAX_TENSOR_ELEMENTS
MAX_PARTITIONS
MAX_REPLICAS
MAX_NODES
MAX_WORKERS
MAX_THREADS
MAX_BATCH_SIZE
MAX_PIPELINE_STAGES
MAX_JOIN_INPUTS
MAX_SCHEMA_DEPTH

as universal language limits.

Implementation-specific limits MAY exist.

They must be classified as implementation/resource policies.

---

54. Tiny-to-Large Scalability

The same semantic program must be capable of execution on:

single value
single record
small collection
large dataset
distributed dataset
massively parallel dataset
future computational substrates

subject only to:

- semantic validity;
- target capabilities;
- actual resources;
- declared constraints;
- execution feasibility.

The grammar must not need different source languages for different scales.

---

55. Infinity Clarification

“Scale to infinity” means:

«No artificial finite language ceiling is imposed where the underlying semantic abstraction is naturally extensible.»

It does not require a physical implementation to provide infinite:

- memory;
- storage;
- bandwidth;
- compute;
- nodes;
- time.

Physical execution remains resource-bounded.

The language remains semantically scalable.

---

56. Distributed Data

Distributed data semantics MUST remain independent of physical topology.

The language may express:

distributed
partition
replicate
rebalance
consistency
checkpoint

but must not require:

node 0
node 1
node 2

as the universal model.

Physical placement belongs to deployment/runtime/resource systems.

---

57. Parallel Data Processing

A data transformation may be parallelized when its semantics permit.

Examples:

map
filter
group
reduce
scan
partition
join

The compiler must preserve:

- ordering where observable;
- associativity requirements;
- floating-point guarantees;
- side-effect semantics;
- error behavior;
- resource contracts.

Parallelization must be a semantics-preserving transformation.

---

58. Deterministic Parallelism

When a data computation is deterministic, an implementation MAY use:

- arbitrary task partitioning;
- vectorization;
- distributed execution;
- GPU execution;
- accelerator execution;

provided the observable result remains within the language's specified semantics.

No fixed worker count is part of the source-language meaning.

---

59. Join Semantics

Joins combine logically related data.

The semantic model must distinguish:

- inner join;
- left join;
- right join;
- full join;
- semi join;
- anti join;
- cross join;
- application-defined join.

Join execution may use:

- hashing;
- sorting;
- indexing;
- distributed exchange;
- streaming;
- nested evaluation;
- accelerator execution.

The algorithm is not grammar semantics.

---

60. Aggregation

Aggregation combines multiple values into a logical result.

The aggregation contract must specify:

- input type;
- accumulator type;
- output type;
- initialization;
- combination;
- ordering requirements;
- empty-input behavior;
- failure behavior.

An aggregation that depends on order MUST NOT be silently parallelized as though it were associative.

---

61. Windowing

Windows define logical subsets of streaming or sequential data.

Supported conceptual forms include:

tumbling
sliding
session
count
custom

Window size and step are semantic expressions.

They are not machine buffer sizes.

A window may be implemented using:

- bounded memory;
- persistent state;
- distributed state;
- incremental processing.

---

62. Sampling

Sampling semantics must explicitly distinguish:

- deterministic sampling;
- probabilistic sampling;
- weighted sampling;
- without-replacement sampling;
- with-replacement sampling.

Randomness is governed by the general effect system.

A random sample MUST NOT become deterministic merely because one backend happens to use a deterministic implementation.

---

63. Sorting

Sorting semantics must define:

- comparison;
- ordering;
- stability where requested;
- null policy;
- error behavior.

The implementation may use any sorting algorithm preserving the semantic contract.

---

64. Deduplication

Deduplication removes semantically duplicate values according to the applicable equality relation.

If ordering is observable, the result must preserve the specified ordering semantics.

The implementation may use:

- hashing;
- sorting;
- distributed structures;
- streaming state.

---

65. Casting and Conversion

A data cast is valid only when permitted by the type system.

Conversions may be:

lossless
lossy
checked
unchecked
fallible
explicit

Lossy conversion MUST be explicit when required by the type system.

No backend may silently change conversion semantics.

---

66. Data and General Type System

The data subsystem MUST reuse the canonical Zamani type system.

It MUST NOT create:

DataType
DataInteger
DataBool
DataString

as a parallel universal type hierarchy when the corresponding canonical types already exist.

Instead:

data syntax
    ↓
canonical type system

The existing type-system contract explicitly requires one unified type system across classical, quantum, hybrid, HDL, distributed, AI, and data domains.

---

67. Data and General Expression System

Data syntax may introduce data-specific expressions.

It MUST NOT redefine:

- arithmetic precedence;
- boolean semantics;
- function calls;
- universal operators;
- assignment;
- general lambda semantics;
- universal indexing rules.

Where an operation is general-purpose, the canonical expression system owns it.

---

68. Data and Effects

Data operations may produce effects including:

io
state
allocation
network
storage
randomness
nondeterminism
external
distributed
quantum
hardware
foreign

The data subsystem reuses "grammar/spec/effects.md".

It does not define a second effect hierarchy.

The effect system already treats I/O, state, resource operations, quantum computation, networking, distributed execution, randomness, accelerators, and foreign functionality as semantic effects.

---

69. Data and Ownership

Data ownership follows the canonical ownership/type/memory system.

A data value may be:

owned
borrowed
shared
immutable
mutable
linear
affine

depending on the applicable type/effect contract.

The data grammar must not invent a separate ownership model.

---

70. Data and Quantum Computing

Data semantics must support quantum-classical workflows.

Examples include:

parameters
measurement results
expectation values
classical control values
experiment metadata
optimization data
training data
calibration observations

The data layer MUST NOT own:

- qubit identity;
- quantum gate semantics;
- quantum circuit semantics;
- physical qubit placement;
- QEC;
- noise implementation;
- QZN/ZQN implementation;
- quantum scheduling;
- quantum routing.

Quantum semantics remain under the quantum subsystem.

"quantum::ir" remains the canonical quantum semantic boundary.

---

71. Measurement Data

Quantum measurement results become ordinary semantic data only after crossing the defined quantum/classical boundary.

The crossing must preserve:

- result type;
- measurement semantics;
- provenance;
- ordering;
- probabilistic behavior;
- relevant effects;
- resource requirements.

Measurement randomness MUST NOT be erased by data lowering.

---

72. Data to Quantum

Classical data may provide:

- operation parameters;
- control values;
- algorithm inputs;
- state-preparation parameters;
- optimization parameters.

Lowering into quantum operations belongs to the quantum semantic subsystem.

The data grammar does not become a quantum gate grammar.

---

73. Quantum to Data

Quantum computations may produce:

- measurement values;
- expectation values;
- statistical samples;
- tomography data;
- experiment metadata.

These are data outputs.

The physical quantum realization remains downstream.

---

74. HDL and Hardware Integration

Data values may cross into HDL/hardware computation.

Examples:

input data
output data
streaming interfaces
buffers
tensor data
control data
configuration data

The data specification does not define:

- wires;
- ports;
- registers;
- clocks;
- state machines;
- physical buses.

Those belong to "grammar/hdl/" and "grammar/hardware/".

---

75. AI/ML Integration

Data is foundational to AI/ML.

The data layer supports general abstractions such as:

- datasets;
- records;
- tensors;
- streams;
- batches;
- transformations;
- provenance;
- schemas;
- pipelines.

AI-specific semantics belong to "grammar/ai/".

The data grammar must not become coupled to:

- one ML framework;
- one neural architecture;
- one tensor backend;
- one accelerator;
- one model format.

---

76. Scientific Data

Scientific computation may use:

- multidimensional data;
- symbolic data;
- numerical data;
- time series;
- physical measurements;
- simulation output;
- experimental data.

The data layer provides generic semantics.

Domain-specific scientific meaning belongs to the appropriate domain contracts.

---

77. Networking Integration

Network-backed data is still logically data.

The data layer may describe:

source
sink
stream
endpoint
request/response data
serialization

The networking subsystem owns:

- protocol semantics;
- transport;
- routing;
- sockets;
- service discovery;
- network capabilities.

The data layer must not duplicate networking semantics.

---

78. Storage Independence

The same logical data abstraction may be stored in:

memory
persistent storage
distributed storage
database
object storage
device memory
accelerator memory

Storage choice is downstream.

Source syntax must not require a particular storage implementation unless storage itself is explicitly part of the semantic contract.

---

79. Query Semantics

A query is a logical transformation over data.

The language may provide query operations such as:

select
filter
project
join
group
aggregate
sort
window
distinct

Query semantics are defined independently of SQL.

SQL interoperability may be provided through "grammar/interoperability/".

SQL is not the canonical Zamani data semantic model.

---

80. Data Pipeline Semantics

A pipeline is a composition of data operations.

Conceptually:

input
  ↓
stage
  ↓
stage
  ↓
stage
  ↓
output

A pipeline's meaning is the composition of its stages.

The compiler may fuse stages when semantics permit.

Fusion MUST preserve:

- errors;
- effects;
- ordering;
- provenance;
- resource requirements;
- determinism.

---

81. Pipeline Optimization

The optimizer MAY:

- fuse transformations;
- reorder independent transformations;
- vectorize;
- parallelize;
- distribute;
- cache;
- materialize;
- eliminate redundant transformations.

But only semantics-preserving transformations are valid.

The data specification does not define optimization algorithms.

---

82. Provenance-Preserving Optimization

Optimization MUST NOT silently remove provenance that the source program declared observable.

If provenance is non-observable metadata, implementations MAY compact or transform it according to the provenance contract.

---

83. Data Movement

Data movement is distinct from data transformation.

Examples:

move
copy
transfer
stream
send
receive

Movement may have effects and resource requirements.

The compiler may eliminate unnecessary movement when doing so preserves observable semantics.

---

84. Copy Semantics

A logical copy creates an independent logical value where the type/effect system defines copyability.

For non-copyable values, the appropriate ownership transfer or borrowing semantics apply.

Physical copy elimination is permitted only when semantic identity and observability are preserved.

---

85. Zero-Copy Implementations

A backend MAY implement a logical copy without physically copying bytes when:

- ownership semantics remain valid;
- mutation isolation remains valid;
- observable identity remains valid;
- lifetime remains valid.

Zero-copy is an implementation optimization, not a source-level guarantee unless explicitly requested.

---

86. Data Lifetime

Data lifetime may be:

temporary
scope-bound
owned
persistent
session
stream
application
external

Lifetime semantics must integrate with memory/resource semantics.

The data grammar must not directly manage allocation.

---

87. Data Persistence

Persistence means data survives the relevant semantic lifetime boundary.

It does not prescribe:

- disk;
- database;
- object store;
- NVRAM;
- distributed replication.

Persistence guarantees must be backed by a suitable runtime/resource capability.

---

88. Data Availability

Availability is distinct from persistence.

Persistent data may temporarily be unavailable.

The language may therefore distinguish:

persistent
available
replicated
recoverable
durable

These are not synonyms.

---

89. Data Durability

Durability describes preservation across the failures covered by the selected contract.

The actual failure domain is part of the resource/execution semantics.

A source program must not assume a stronger durability guarantee than the declared capability provides.

---

90. Data Security

Data may carry security-related metadata or requirements.

Examples:

confidential
integrity_required
authenticated
encrypted
restricted

Security semantics integrate with "grammar/security/".

The data grammar must not implement cryptographic algorithms.

---

91. Sensitive Data

Sensitive values MUST NOT automatically enter:

- compiler diagnostics;
- provenance;
- logs;
- traces;
- generated source;
- error messages.

The implementation must preserve the security/effect contracts.

---

92. Data Privacy

Privacy constraints may be semantic requirements.

Examples include:

data must not leave domain

data must remain encrypted

data may only be processed under capability X

Actual enforcement belongs to security/resource/runtime systems.

---

93. Data and Capability Negotiation

A program may require a capability.

For example:

requires capability("distributed.streams")

The runtime/compiler may determine whether the target can satisfy it.

Capability negotiation must not require source rewriting.

---

94. Target Independence

The same data program may lower differently on:

CPU
GPU
FPGA
ASIC
QPU-assisted system
embedded device
single machine
cluster
cloud
future substrate

The semantic result remains governed by this specification.

---

95. Hardware Resource Scaling

Data dimensions may scale according to available resources.

For example:

dataset
tensor
collection
stream
pipeline

may grow without a grammar-defined ceiling.

The implementation must distinguish:

program asks for larger data

from:

target cannot currently satisfy the request

---

96. Data and Scheduling

Scheduling owns:

- execution order;
- temporal placement;
- resource allocation timing;
- concurrency scheduling.

The data grammar does not schedule transformations.

A source-level data ordering requirement is semantic input to scheduling, not scheduling itself.

---

97. Data and Routing

Routing owns physical movement and placement.

Data partitioning syntax describes logical partitioning.

Routing determines physical realization.

This distinction is mandatory for POCO-REAF.

---

98. Data and Resilience

Data operations may declare:

- retryability;
- checkpointability;
- recoverability;
- replication requirements.

The resilience subsystem owns actual recovery orchestration.

Data grammar must not implement retry loops as an implicit runtime mechanism.

---

99. Data and ZQN

ZQN remains responsible for fault/noise semantics where applicable.

Data semantics may carry observations or metadata associated with:

- noisy computation;
- experiment results;
- error information;
- provenance.

The data layer does not become the fault model.

---

100. Data and HAL

HAL determines concrete hardware capabilities/state.

The data subsystem may request a capability such as:

capability("high_bandwidth_data")

but must not select a particular hardware device.

---

101. Data IR Integration

Every data construct that survives semantic lowering must map to the canonical semantic model.

The required direction is:

data syntax
    ↓
domain-neutral AST
    ↓
data semantic model
    ↓
canonical IR / domain IR

There must not be:

data grammar
    ↓
database-specific IR

as the universal semantic boundary.

Database-specific lowering is downstream.

---

102. AST Contract

Every data syntax construct requires a predetermined AST mapping.

Examples:

schema
    → generic declaration/schema node

record
    → generic record/data declaration node

collection
    → generic data binding/collection node

stream
    → generic stream node

transform
    → generic operation/transformation node

query
    → generic data operation node

pipeline
    → generic computation graph/pipeline node

The AST must remain domain-neutral.

It must not contain vendor-specific database nodes merely because a backend exists.

---

103. Generic Operation Integration

Where appropriate, transformations should use the generic operation model already required by the broader frontend architecture:

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

This avoids creating a giant enumeration such as:

DataOperation::Map
DataOperation::Filter
DataOperation::Join
...

when the generic semantic operation model can represent the operation without losing semantic information.

Dedicated semantic nodes remain appropriate when an operation has unique invariants that cannot safely be represented generically.

---

104. Semantic Validation

Semantic analysis must validate:

- schema references;
- field existence;
- field types;
- nullability;
- key validity;
- collection element types;
- stream element types;
- transformation input/output compatibility;
- lambda compatibility;
- join compatibility;
- aggregation validity;
- ordering requirements;
- window validity;
- resource requirements;
- capability requirements;
- effect requirements;
- ownership/lifetime;
- serialization compatibility;
- provenance requirements.

---

105. Type Errors

Examples:

DATA_FIELD_NOT_FOUND
DATA_TYPE_MISMATCH
DATA_SCHEMA_MISMATCH
DATA_INVALID_CAST
DATA_INVALID_JOIN
DATA_INVALID_AGGREGATION
DATA_INVALID_WINDOW
DATA_INVALID_COLLECTION_OPERATION
DATA_INVALID_STREAM_OPERATION

These are semantic/type diagnostics.

They are not runtime resource failures.

---

106. Resource Errors

Examples:

DATA_RESOURCE_UNSATISFIABLE
DATA_CAPABILITY_UNAVAILABLE
DATA_STORAGE_UNAVAILABLE
DATA_BANDWIDTH_UNAVAILABLE
DATA_MEMORY_UNAVAILABLE
DATA_DISTRIBUTION_UNAVAILABLE

These must remain distinguishable from syntax and type errors.

---

107. Runtime Data Errors

Examples:

DATA_SOURCE_FAILURE
DATA_SINK_FAILURE
DATA_DESERIALIZATION_FAILURE
DATA_SERIALIZATION_FAILURE
DATA_SCHEMA_VERSION_FAILURE
DATA_CHECKPOINT_FAILURE
DATA_STREAM_FAILURE
DATA_CONSISTENCY_FAILURE
DATA_TIMEOUT
DATA_CANCELLED

The exact taxonomy may be extended without breaking existing semantic categories.

---

108. Diagnostics

Every data diagnostic must include, where available:

- diagnostic code;
- source span;
- primary message;
- relevant data symbol;
- expected property;
- actual property;
- related source locations;
- suggested correction when safe;
- provenance context when permitted.

Diagnostics MUST NOT expose secrets.

---

109. Source Spans

Every parser-created data construct must preserve source location information.

At minimum:

start position
end position
source identity

The data semantic model should preserve source provenance sufficiently for diagnostics.

---

110. Error Recovery

Parser recovery must be owned by parser infrastructure.

Data grammar should provide structurally recoverable rules where practical.

Recovery MUST NOT silently convert malformed data semantics into valid data.

---

111. Compatibility

Changes to data syntax or semantics must follow "grammar/spec/compatibility.md".

Compatibility categories should include:

source-compatible
AST-compatible
semantic-compatible
IR-compatible
runtime-compatible
breaking

A syntax change is not automatically semantically compatible.

---

112. Versioning

Data schemas, serialization formats, and language features must have independent version concepts.

Do not conflate:

Zamani language version
schema version
serialization version
dataset version
pipeline version
runtime version

Each must be represented only where semantically required.

---

113. Interoperability

Data interoperability may include:

- JSON;
- CSV;
- binary formats;
- columnar formats;
- database protocols;
- streaming protocols;
- scientific formats;
- AI model/data formats;
- OpenQASM/QIR boundary data;
- HDL interfaces;
- foreign-language interfaces.

These belong to interoperability contracts.

They must not become the canonical Zamani data semantics.

---

114. SQL Integration

SQL may be supported as an interoperability/query dialect.

SQL syntax MUST NOT replace the canonical Zamani data model.

The canonical architecture is:

SQL
 ↓
interop frontend
 ↓
semantic translation
 ↓
Zamani data model
 ↓
canonical IR

not:

SQL
 ↓
canonical Zamani semantics

---

115. Database Integration

A database is one possible data realization.

The data semantics must remain usable without any database.

This allows the same source to target:

- in-memory computation;
- file-backed data;
- distributed storage;
- databases;
- accelerators;
- streaming systems;
- future substrates.

---

116. Data Views

A view represents a logical derived data value.

A view may be:

lazy
materialized
incremental
persistent
temporary

The semantics must distinguish logical identity from physical materialization.

---

117. Data Contracts and Pipelines

A pipeline may specify contracts at:

input
stage
boundary
output

A compiler may validate compatibility between stages before execution.

This permits independent completion of pipeline components without requiring later grammar rewrites.

---

118. Cross-Domain Composition

Data operations must be composable with:

classical
quantum
hybrid
HDL
hardware
AI
distributed
networking
security
scientific
embedded
accelerator
future domains

The data layer remains a shared semantic substrate.

It does not import every domain grammar.

Domain integration occurs through explicit contracts.

---

119. Dependency Direction

The dependency graph must remain acyclic:

core
  ↓
types / expressions
  ↓
data
  ↓
semantic model
  ↓
IR
  ↓
compiler
  ↓
runtime

Data grammar MUST NOT import:

runtime
HAL
QEC
ZQN
scheduler
router
backend implementation

Data syntax must not become coupled to downstream implementation.

---

120. Independent File Completion Contract

Each existing data grammar file must be independently completable.

"grammar/data/data.g4"

Complete when:

- all public data entry points are defined;
- specialized productions are delegated;
- no duplicated schema/record/collection/stream/serialization/transformation rules remain;
- root integration is documented;
- AST mappings are defined;
- semantic mappings are defined;
- diagnostics are defined;
- tests are defined;
- no hard-coded limits remain.

"grammar/data/schemas.g4"

Complete when:

- schema syntax is complete;
- field syntax is integrated with canonical "typeExpr";
- constraints are defined;
- defaults are defined;
- evolution hooks exist;
- AST contract is known;
- semantic contract is known;
- compatibility contract is known.

"grammar/data/records.g4"

Complete when:

- record syntax is complete;
- canonical field/type rules are reused;
- nested records are supported;
- generic records are supported where type system permits;
- no physical register semantics exist;
- AST/semantic/IR mappings are predetermined.

"grammar/data/collections.g4"

Complete when:

- collection kinds are defined;
- ordering is explicit;
- uniqueness is explicit;
- mutability is explicit;
- laziness is explicit;
- cardinality remains unbounded semantically;
- AST/semantic/IR mappings are defined.

"grammar/data/streams.g4"

Complete when:

- bounded/unbounded semantics exist;
- ordering is explicit;
- replayability is explicit;
- loss semantics are explicit;
- window/batch/checkpoint hooks are defined;
- backpressure semantics integrate with execution;
- no fixed stream size exists.

"grammar/data/serialization.g4"

Complete when:

- serialize/deserialize intent is defined;
- schema/version contracts are defined;
- failure behavior is defined;
- canonical serialization hooks are defined;
- implementation formats remain downstream.

"grammar/data/transformations.g4"

Complete when:

- transformation syntax is complete;
- transformation composition is deterministic;
- map/filter/reduce/fold/scan semantics are mapped;
- joins/grouping/windows are mapped;
- resource/effect integration is predetermined;
- optimization boundaries are documented;
- no execution backend is embedded.

---

121. Required Feature Traceability

Every data feature must be traceable:

Feature
  ↓
spec/data.md
  ↓
data/*.g4
  ↓
Zamani.g4
  ↓
AST
  ↓
semantic model
  ↓
IR
  ↓
compiler
  ↓
runtime
  ↓
tests

A feature is not production-complete if any required link is undefined.

---

122. Required Feature Manifest

Where the repository's feature-manifest system is introduced, every significant data feature should have a corresponding manifest.

Example:

id: data.transform.map
name: map
status: stable

syntax:
  grammar: grammar/data/transformations.g4
  rule: dataMapExpression

ast:
  kind: operation

semantic:
  input: sequence<T>
  output: sequence<U>
  effects: inherited_from_function

ir:
  operation: data.map

compiler:
  consumers:
    - optimization
    - parallelization
    - vectorization
    - distribution

runtime:
  consumers:
    - execution

tests:
  positive: true
  negative: true
  boundary: true
  scalability: true
  determinism: true

hard_coding:
  universal_limits: forbidden

This makes the feature independently completable.

---

123. Hard-Coding Audit

The following are prohibited as universal data-language limits:

MAX_ROWS
MAX_COLUMNS
MAX_ELEMENTS
MAX_RECORDS
MAX_FIELDS
MAX_DATASETS
MAX_STREAMS
MAX_STREAM_LENGTH
MAX_PARTITIONS
MAX_REPLICAS
MAX_NODES
MAX_WORKERS
MAX_THREADS
MAX_BATCH_SIZE
MAX_TENSOR_RANK
MAX_TENSOR_ELEMENTS
MAX_PIPELINE_STAGES
MAX_JOIN_INPUTS
MAX_SCHEMA_DEPTH

Suspicious constructs such as:

partition_count = 16
replicas = 3
nodes = 64

must be reviewed according to whether the value is:

program semantics
resource requirement
resource preference
implementation default
universal language limit

Only the last category is prohibited.

---

124. Semantic Requirement vs Implementation Decision

This distinction is mandatory.

Requirement

requires persistent storage

Capability

requires capability("persistent.data")

Constraint

requires latency < L

Preference

prefer distributed

Hint

hint locality(key)

Implementation decision

store on device 7

The first five may be source-level semantic intent.

The final item belongs downstream unless explicitly part of a target-specific dialect.

---

125. Determinism Tests

Every deterministic transformation requires tests proving that:

- equivalent inputs produce equivalent outputs;
- legal execution reorderings do not alter observable results;
- serialization is stable where promised;
- distributed execution preserves required semantics;
- optimization preserves results.

---

126. Scalability Tests

The test suite must include conceptual scaling dimensions:

1 element
many elements
symbolic cardinality
large cardinality
many fields
deep structures
large tensors
large streams
many transformations
many partitions
many logical replicas
distributed execution
heterogeneous execution

Tests must not define the largest supported value as a language limit.

---

127. Boundary Tests

Required boundary categories include:

- empty collection;
- singleton collection;
- empty stream;
- one-element stream;
- null/optional values;
- duplicate values;
- duplicate keys;
- missing fields;
- incompatible schemas;
- zero-length dimensions;
- symbolic dimensions;
- maximum implementation-supported values;
- resource exhaustion;
- unavailable capability;
- failed source;
- failed sink;
- serialization failure;
- cancellation.

---

128. Negative Tests

Required negative tests include:

unknown field
invalid schema
invalid transformation type
invalid join
invalid aggregation
invalid window
invalid cast
invalid stream operation
invalid serialization
incompatible schema
missing capability
invalid resource requirement
invalid ownership
invalid effect

---

129. Compatibility Tests

Compatibility tests must cover:

old source → new compiler
new source → supported old compiler
schema version transitions
serialization version transitions
AST compatibility
IR compatibility
dialect compatibility
interoperability compatibility

---

130. Security Tests

Data tests must verify:

- sensitive values are not leaked through diagnostics;
- provenance does not expose secrets;
- serialization does not silently bypass security constraints;
- unauthorized data access is rejected;
- capability requirements are enforced downstream;
- external endpoints cannot bypass declared policies.

---

131. Performance Contract

The data specification MUST NOT require a particular implementation complexity unless complexity itself is part of the semantic contract.

For example, the language does not promise:

map = O(1)
join = O(n)
sort = O(n log n)

merely because an implementation commonly achieves those bounds.

Performance contracts belong to explicit profiles/claims where required.

---

132. Memory Contract

The data semantics MUST NOT assume:

- contiguous memory;
- fixed word size;
- fixed pointer size;
- one memory space;
- one address space;
- one storage tier.

Memory realization belongs to "grammar/spec/resources.md", memory semantics, compiler, runtime, and target systems.

---

133. Zero-Copy and Movement Optimization

A compiler MAY eliminate data movement where:

- aliasing remains valid;
- ownership remains valid;
- effects remain valid;
- observable identity remains valid;
- synchronization remains valid.

This permits efficient implementations without exposing physical memory architecture in the language.

---

134. Future Data Models

The data semantic model must permit future abstractions such as:

- distributed tensors;
- quantum-generated datasets;
- neuromorphic data;
- probabilistic data;
- symbolic data;
- temporal data;
- event-sourced data;
- knowledge graphs;
- scientific meshes;
- sparse hyperscale data;
- future computational substrates.

New data domains should extend the canonical model rather than fork it.

---

135. Dialect Integration

A dialect may add data syntax.

A dialect MUST declare:

name
version
syntax extension
semantic extension
AST mapping
IR mapping
compatibility
capabilities

A dialect MUST NOT silently change core data semantics.

---

136. Interoperability Boundary

External data systems must lower through explicit interoperability adapters.

Example:

external format
      ↓
interop parser
      ↓
Zamani data semantic model
      ↓
canonical IR

Not:

external format
      ↓
hidden backend-specific semantics

---

137. Runtime Integration

The runtime consumes the result of semantic lowering.

It is responsible for:

- acquiring data resources;
- opening sources;
- opening sinks;
- executing transformations;
- managing streams;
- handling checkpoints;
- handling runtime failures;
- reporting resource failures;
- respecting capabilities;
- enforcing applicable policies.

The grammar does none of these.

---

138. Compiler Integration

The compiler consumes data semantic operations and may:

- specialize;
- fuse;
- vectorize;
- parallelize;
- distribute;
- schedule;
- lower;
- serialize;
- select representations.

Every transformation must preserve this specification.

---

139. Tooling Integration

Tooling should be able to:

- syntax-highlight data constructs;
- inspect schemas;
- inspect pipeline graphs;
- inspect transformation types;
- show lineage;
- display resource requirements;
- show capability requirements;
- display diagnostics;
- inspect provenance where permitted.

Tooling must consume canonical semantic information rather than reimplementing data semantics independently.

---

140. Documentation Integration

"grammar/data/README.md" documents package structure.

"grammar/spec/data.md" defines normative semantics.

"grammar/grammar.md" documents implementation conformance.

"grammar/Zamani-Grammar.md" remains historical/proposed material until features are promoted.

No documentation file may silently become a second semantic authority.

---

141. Implementation Safety

Zamani-owned Rust integration MUST use:

#![forbid(unsafe_code)]

where applicable.

No data grammar implementation may require:

unsafe

for correctness.

No parser action may perform:

- filesystem I/O;
- network I/O;
- runtime execution;
- hardware discovery;
- resource allocation.

ANTLR grammar remains declarative.

---

142. Rust 1.97 / 1.97.1 Compatibility

The data implementation MUST remain compatible with:

Rust 1.97
Rust 1.97.1
Rust 2021

The implementation must not depend on APIs introduced after the selected baseline.

Where generated parser code has toolchain constraints, those constraints must be documented and tested against the repository's pinned toolchain.

---

143. No Unsafe Semantic Escape Hatch

The data subsystem must not expose an API whose correctness requires callers to use "unsafe".

Unsafe target-specific implementation may not be smuggled into the language semantics.

The language remains safe even when the backend is specialized for hardware.

---

144. Canonical Integration With "Zamani.g4"

The canonical root grammar must provide one data entry point.

Conceptually:

statement
    : ...
    | dataStmt
    | ...
    ;

or the equivalent existing composition rule.

The root grammar must not duplicate every data production.

"data.g4" remains the data package integration facade.

This is consistent with the current repository's stated intent that "dataStmt" be the root integration boundary.

---

145. Canonical Integration With "grammar.md"

"grammar/grammar.md" must document:

specified
implemented
partially implemented
planned
deprecated

for every data feature.

It must not redefine the semantics established here.

---

146. Canonical Integration With "Zamani-Grammar.md"

"Zamani-Grammar.md" may contain broader proposed data concepts.

A concept becomes normative only after:

proposal
 ↓
semantic contract
 ↓
syntax contract
 ↓
AST contract
 ↓
IR contract
 ↓
implementation
 ↓
tests
 ↓
promotion

Appearance in "Zamani-Grammar.md" alone is insufficient.

---

147. Data Feature Completion Criteria

A data feature is production-ready only when all applicable items are complete:

[ ] normative semantic definition
[ ] syntax rule
[ ] lexer/token contract
[ ] AST mapping
[ ] semantic model mapping
[ ] type-system integration
[ ] effect integration
[ ] resource integration
[ ] capability integration
[ ] IR mapping
[ ] compiler consumers
[ ] runtime consumers
[ ] diagnostics
[ ] source spans
[ ] positive tests
[ ] negative tests
[ ] boundary tests
[ ] scalability tests
[ ] determinism tests
[ ] compatibility tests
[ ] security tests
[ ] hard-coding audit
[ ] documentation
[ ] interoperability contract where applicable

---

148. Data Package Completion Criteria

"grammar/data/" is production-complete when:

1. "data.g4" is only the integration facade.
2. Specialized files own specialized syntax.
3. No duplicated data grammar authority exists.
4. Data syntax reuses canonical types.
5. Data syntax reuses canonical expressions.
6. Data semantics are defined here.
7. AST mappings are complete.
8. Semantic mappings are complete.
9. IR mappings are complete.
10. Compiler consumers are known.
11. Runtime consumers are known.
12. Quantum/classical boundaries are defined.
13. HDL boundaries are defined.
14. AI boundaries are defined.
15. Distributed semantics are defined.
16. Resource semantics are defined.
17. Capability semantics are defined.
18. Effects are integrated.
19. Determinism is defined.
20. Failure categories are defined.
21. Provenance and lineage are defined.
22. Schema evolution is defined.
23. Serialization semantics are defined.
24. No universal resource limits are encoded.
25. Rust integration is safe.
26. Rust 1.97/1.97.1 compatibility is verified.
27. Positive tests pass.
28. Negative tests pass.
29. Boundary tests pass.
30. Scalability tests pass.
31. Compatibility tests pass.
32. The complete grammar → AST → semantics → IR → compiler → runtime chain is traceable.

---

149. Final Normative Invariants

The following invariants are mandatory.

Invariant 1 — One data semantic model

Zamani MUST have one canonical data semantic model.

Invariant 2 — No duplicate type system

Data MUST use the canonical Zamani type system.

Invariant 3 — No duplicate expression system

Data MUST use the canonical expression system.

Invariant 4 — No physical topology in data semantics

Data semantics MUST NOT encode machine topology.

Invariant 5 — No artificial scale limits

Data semantics MUST NOT define arbitrary universal limits.

Invariant 6 — Logical before physical

Logical data intent precedes physical realization.

Invariant 7 — Resource realization is downstream

Resource availability is resolved by resource/compiler/runtime systems.

Invariant 8 — Quantum boundary remains canonical

Quantum semantics lower through "quantum::ir".

Invariant 9 — No domain cycles

Data grammar must not import downstream domain implementations.

Invariant 10 — Safe Rust

Zamani-owned Rust integration uses no "unsafe".

Invariant 11 — Semantic determinism

Implementations must preserve declared deterministic behavior.

Invariant 12 — Explicit failure

Observable failure must use defined failure semantics.

Invariant 13 — Provenance is controlled

Provenance must not expose secrets.

Invariant 14 — Implementation freedom

Equivalent physical representations are permitted when semantic equivalence is preserved.

Invariant 15 — POCO-REAF

A valid portable data program must not require source rewriting merely because the available computational realization changes.

---

150. Canonical End-to-End Data Architecture

The final architecture is:

                         ZAMANI SOURCE
                              │
                              ▼
                     grammar/Zamani.g4
                              │
                              ▼
                         Shared Lexer
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
        ┌─────────────────────────────────────────┐
        │         Semantic Analysis               │
        │                                         │
        │ types / effects / ownership             │
        │ capabilities / resources                │
        │ schemas / transformations               │
        │ ordering / determinism                  │
        │ provenance / lineage                    │
        └─────────────────────────────────────────┘
                              │
                              ▼
                 Canonical Data Semantics
                              │
              ┌───────────────┼────────────────┐
              │               │                │
              ▼               ▼                ▼
        Classical         Quantum           HDL/
           IR            quantum::ir       Hardware IR
              │               │                │
              └───────────────┼────────────────┘
                              │
                              ▼
                         Optimization
                              │
                    ┌─────────┼─────────┐
                    │         │         │
                    ▼         ▼         ▼
                 Routing  Scheduling  Resilience
                    │         │         │
                    └─────────┼─────────┘
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
          ┌──────────┬───────┼───────┬──────────┐
          ▼          ▼       ▼       ▼          ▼
         CPU        GPU     FPGA    QPU      Future
          │          │       │       │       substrates
          └──────────┴───────┴───────┴──────────┘
                              │
                              ▼
                           Runtime
                              │
                              ▼
                         Actual Data

The essential property is that the source describes what the data means and what computation is required, while the compiler/runtime determine how and where it is realized.

That separation is what allows the same data program to scale from a tiny local computation to distributed, heterogeneous, accelerated, quantum-assisted, or future computational systems without introducing artificial grammar limits.
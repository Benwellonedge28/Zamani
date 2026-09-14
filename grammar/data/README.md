Zamani Data Grammar

Path: "grammar/data/"

Status: Production architecture / normative package contract

Language: Zamani

Grammar technology: ANTLR-compatible parser grammars

Rust integration target: Rust 1.97 / Rust 1.97.1

Rust safety: "unsafe" is prohibited in Zamani-owned integration code.

Architectural principle: Data syntax describes portable data semantics and intent. It does not encode accidental limitations of the machine, storage system, network, database, accelerator, or deployment environment.

---

1. Purpose

"grammar/data/" defines the Zamani language-level syntax for data computation.

It is responsible for expressing portable data semantics that can participate in:

- classical computation;
- quantum/classical hybrid computation;
- scientific computing;
- AI/ML;
- tensor and numerical computation;
- streaming computation;
- distributed computation;
- parallel computation;
- HPC;
- embedded systems;
- edge/cloud computation;
- hardware/software co-design;
- storage-independent computation;
- network-independent data movement;
- accelerator computation;
- future computational models.

The package exists so that a Zamani program can describe what data means and what should happen to it without requiring the source program to be rewritten for every machine or deployment.

The governing model is:

«Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF).»

---

2. Core architectural rule

The data grammar describes:

- data;
- structure;
- relationships;
- transformations;
- queries;
- streams;
- sources;
- sinks;
- schemas;
- records;
- collections;
- pipelines;
- serialization intent;
- validation intent;
- movement intent;
- materialization intent;
- partitioning intent;
- distribution intent;
- replication intent;
- consistency intent;
- provenance and lineage intent;
- logical resource requirements;
- logical constraints;
- logical preferences;
- logical hints.

It does not describe a particular implementation of those concepts.

For example:

requires distributed;

may express a semantic execution requirement.

It must not implicitly mean:

use 64 nodes
use provider X
use cluster Y
use device Z

Likewise:

partition by customer_id;

expresses partitioning semantics.

It does not specify:

- partition count;
- node count;
- machine topology;
- storage locations;
- network topology;
- physical addresses;
- database shards;
- provider-specific partitions.

Those decisions belong downstream.

---

3. Ownership

3.1 This package owns

"grammar/data/" owns the syntax necessary to express:

Data declarations

- schemas;
- records;
- collections;
- sequences;
- streams;
- sources;
- sinks;
- views;
- transformations;
- pipelines;
- contracts;
- logical data resources.

Data expressions

- data references;
- data literals;
- collection expressions;
- record expressions;
- transformations;
- queries;
- projections;
- filtering;
- grouping;
- aggregation;
- joins;
- ordering;
- windowing;
- serialization expressions;
- materialization expressions;
- logical data movement.

Data intent

- validation;
- partitioning;
- distribution;
- replication;
- consistency;
- materialization;
- provenance;
- lineage;
- resource requirements;
- preferences;
- constraints;
- hints.

Data interoperability

The package provides syntax hooks through which data semantics can participate in:

- classical computation;
- quantum-classical workflows;
- AI;
- distributed execution;
- hardware acceleration;
- networking;
- serialization;
- external interfaces.

---

4. Non-ownership

"grammar/data/" does not own:

- lexer implementation;
- token definitions;
- universal type semantics;
- expression semantics;
- general statements;
- general control flow;
- function semantics;
- memory allocation;
- physical memory layout;
- database engines;
- SQL execution;
- filesystem implementations;
- object stores;
- network protocols;
- network transport;
- serialization implementations;
- compression implementations;
- storage engines;
- query execution engines;
- hardware discovery;
- hardware topology;
- accelerator discovery;
- quantum hardware;
- quantum IR;
- classical IR;
- optimization algorithms;
- routing;
- scheduling;
- resource discovery;
- runtime dispatch;
- cloud-provider APIs;
- provider-specific execution;
- QEC algorithms;
- ZQN fault models;
- hardware calibration.

The grammar establishes syntax.

Semantic analysis establishes meaning.

IR establishes canonical representation.

Compilation establishes target realization.

Scheduling establishes temporal/resource ordering.

Runtime establishes execution.

---

5. Repository boundary

The intended architecture is:

Zamani source
    |
    v
Shared Zamani lexer
    |
    v
Zamani parser
    |
    v
grammar/data/*
    |
    v
Parse tree
    |
    v
AST / syntax model
    |
    v
Semantic analysis
    |
    v
Canonical data/type representation
    |
    +------------------+
    |                  |
    v                  v
Classical IR       Quantum semantic boundary
    |                  |
    +--------+---------+
             |
             v
       Optimization
             |
             v
        Scheduling
             |
             v
    Resource / capability
             |
             v
       Hardware / runtime
             |
             v
      Execution backend

"grammar/data/" must never create a circular dependency back into the grammar.

The correct dependency direction is:

Grammar
  ↓
AST / semantic analysis
  ↓
IR
  ↓
compiler
  ↓
runtime

Never:

Grammar
  ↓
Runtime
  ↓
Grammar

---

6. Canonical files

The data grammar is decomposed into specialized grammar components.

Expected package:

grammar/data/
├── README.md
├── data.g4
├── schemas.g4
├── records.g4
├── collections.g4
├── streams.g4
├── serialization.g4
└── transformations.g4

Additional files may be introduced only when a new responsibility cannot be cleanly owned by an existing component.

Do not create files merely to increase directory size.

---

7. File ownership matrix

File| Primary responsibility
"data.g4"| Package integration facade and data-level entry points
"schemas.g4"| Logical schema declarations
"records.g4"| Logical record declarations and record-specific syntax
"collections.g4"| Logical collection declarations
"streams.g4"| Stream declarations and stream-specific operations
"serialization.g4"| Serialization/deserialization intent
"transformations.g4"| Data transformation and query-oriented syntax
"README.md"| Architecture, ownership, contracts, integration and completion criteria

Each grammar file must have exactly one primary ownership domain.

---

8. "data.g4" contract

"data.g4" is the integration facade.

It must not become a monolithic grammar.

It should delegate specialized constructs to:

schemas.g4
records.g4
collections.g4
streams.g4
serialization.g4
transformations.g4

For example:

data.g4
  ├── dataSchemaDeclaration
  ├── dataRecordDeclaration
  ├── dataCollectionDeclaration
  ├── dataStreamDeclaration
  ├── serializationConstruct
  └── dataTransformation

The specialized files own their respective syntax.

"data.g4" owns composition and integration.

---

9. Schema contract

"schemas.g4" owns logical schemas.

A schema describes data structure.

It does not prescribe:

- database tables;
- memory layouts;
- physical files;
- network packets;
- storage blocks;
- machine registers;
- hardware buses.

A schema must remain portable across implementations.

Schema semantics may later be lowered into:

- in-memory structures;
- database structures;
- serialized formats;
- distributed representations;
- accelerator representations;
- hardware structures.

Those decisions are downstream.

---

10. Record contract

"records.g4" owns logical records.

The existing repository explicitly defines "dataRecordDeclaration" as its canonical integration rule, with "data.g4" expected to delegate rather than redefine record syntax.

A logical data record must not be confused with:

- CPU registers;
- hardware registers;
- quantum registers;
- HDL registers;
- database implementation rows;
- physical memory records.

The record grammar must not impose limits on:

- field count;
- record count;
- record size;
- nesting depth;
- machine width;
- memory capacity.

Any such limitation is downstream.

---

11. Collection contract

"collections.g4" owns collection declarations.

The repository already defines collection syntax as a delegated responsibility and requires "data.g4" to delegate to its canonical collection declaration rule.

Collections are logical abstractions.

They must not encode:

MAX_ELEMENTS
MAX_COLLECTION_SIZE
MAX_MEMORY
MAX_PARTITIONS
MAX_NODES

A collection can therefore be implemented as:

- a local structure;
- a distributed structure;
- a database collection;
- an accelerator representation;
- a streamed representation;
- a persistent structure;
- an ephemeral structure.

The implementation is selected later.

---

12. Stream contract

"streams.g4" owns stream syntax.

The existing stream grammar establishes "streamDeclaration" as the public delegated entry rule and deliberately does not consume "EOF", because it is designed to be embedded by higher-level grammar rules.

Streams must support logical concepts such as:

- bounded streams;
- unbounded streams;
- ordered streams;
- unordered streams;
- replayable streams;
- non-replayable streams;
- lossless streams;
- lossy streams;
- transformations;
- filtering;
- mapping;
- windows;
- batching;
- buffering;
- sampling;
- deduplication;
- checkpoints.

The grammar must not specify:

- fixed buffer sizes;
- fixed stream lengths;
- fixed throughput;
- fixed node counts;
- fixed partitions.

---

13. Transformation contract

"transformations.g4" owns data transformation syntax.

It may express operations such as:

- map;
- filter;
- flat-map;
- project;
- select;
- aggregate;
- reduce;
- group;
- join;
- sort;
- window;
- reshape;
- transpose;
- partition;
- repartition;
- merge;
- union;
- difference;
- sample;
- deduplicate.

Transformations express logical computation.

They do not specify execution strategy.

For example:

data.map(f)

must not imply:

CPU execution
GPU execution
single-thread execution
specific vector width
specific memory layout

The compiler may choose the appropriate implementation.

---

14. Serialization contract

"serialization.g4" owns serialization syntax.

The repository already defines it as an integration facade that "data.g4" should delegate to rather than redefine serialization syntax.

Serialization syntax may express intent such as:

serialize
deserialize
encode
decode
format
schema
version

It must not implement the serializer.

Possible downstream implementations include:

- binary serialization;
- text serialization;
- canonical serialization;
- streaming serialization;
- zero-copy representations where supported;
- quantum/classical boundary serialization;
- distributed serialization.

The grammar remains implementation-independent.

---

15. Data and type-system integration

Data grammar must reuse the canonical type system.

It must not create a second type system.

Where the repository owns:

typeExpr
genericParameters
primitive types
composite types
function types
array types
map types
tensor types
resource types
quantum types
hardware types

data grammar references those constructs.

Therefore:

data grammar
      |
      v
canonical type grammar

not:

data grammar
      |
      v
data-specific duplicate type system

This prevents semantic divergence.

---

16. Expression integration

Data expressions must integrate with the canonical expression grammar.

Data grammar may introduce data-specific expression forms, but it must not redefine:

- arithmetic precedence;
- general boolean semantics;
- function calls;
- generic expression semantics;
- assignment semantics;
- universal operators.

Where an operation is generally applicable, the universal expression system owns it.

Data grammar only adds data-specific syntax.

---

17. Identifier integration

Identifiers must come from the authoritative Zamani lexer/core grammar.

Do not define another identifier lexer.

Names must remain capable of representing arbitrarily large logical namespaces without grammar-level resource assumptions.

---

18. Numeric scalability

Data grammar must not impose machine-dependent numeric limits.

The grammar must not encode assumptions such as:

32-bit only
64-bit only
maximum dataset = X
maximum dimension = Y
maximum index = Z

Numeric literal syntax is lexical/type-system responsibility.

Semantic validity belongs to type checking.

Runtime representation belongs to the selected target.

---

19. Tensor and scientific-data integration

The data layer must be capable of representing logical data that later becomes:

- vectors;
- matrices;
- tensors;
- sparse structures;
- symbolic data;
- numerical datasets;
- scientific datasets;
- ML tensors.

Tensor dimensionality must not be hard-coded.

For example, the grammar must not assume:

tensor<2>
tensor<3>
tensor<4>

as a universal maximum.

The number of dimensions is a semantic property.

---

20. AI integration

The AI grammar owns AI-specific constructs.

The data grammar provides general-purpose data structures used by AI.

Therefore:

AI grammar
    ↓
data grammar
    ↓
canonical type/data semantics

AI-specific concepts such as:

- models;
- training;
- inference;
- datasets;
- agents;
- differentiation

must not be duplicated inside "grammar/data/".

Conversely, data transformations must remain usable by AI without requiring the data grammar to know about every future AI architecture.

---

21. Quantum integration

The data grammar must remain independent of quantum hardware.

It can provide data structures consumed by quantum/classical programs.

For example:

measurement results
parameter sets
optimization data
classical control data
experiment metadata

may flow through the data model.

However:

"grammar/data/" must not define:

- qubit IDs;
- physical qubits;
- quantum gates;
- QEC algorithms;
- noise models;
- quantum topology;
- quantum hardware configuration.

Those belong to the quantum grammar and downstream quantum subsystems.

The canonical "quantum::ir" remains the quantum semantic boundary.

---

22. Quantum-classical integration

A data value may cross the quantum/classical boundary.

The architecture must support:

classical value
      ↓
quantum-classical operation
      ↓
quantum execution
      ↓
measurement
      ↓
data value
      ↓
classical computation

The grammar does not define how this crossing is implemented.

Semantic lowering must preserve:

- types;
- ownership;
- provenance;
- ordering;
- measurement semantics;
- errors;
- resource requirements.

---

23. HDL integration

Data syntax may describe logical information consumed or produced by hardware programs.

It must not redefine:

- wires;
- ports;
- registers;
- clocks;
- hardware processes;
- state machines.

Those belong to "grammar/hdl/".

The boundary is:

data semantics
     ↓
HDL semantic lowering
     ↓
hardware representation

not:

data grammar → physical hardware

---

24. Hardware independence

The data grammar must remain independent of:

- CPU model;
- GPU model;
- FPGA family;
- ASIC;
- quantum processor;
- accelerator;
- memory controller;
- device ID;
- bus address;
- PCI address;
- physical memory address.

Hardware capabilities may constrain execution later.

They must not silently become syntax restrictions.

---

25. Distributed computing

Data syntax must support logical distributed semantics.

Possible semantics include:

- distributed collections;
- distributed streams;
- partitioning;
- replication;
- consistency;
- remote sources;
- remote sinks;
- lineage;
- checkpoints.

The grammar must not specify:

node 0
node 1
node 2

as a universal execution model.

Neither should it hard-code:

replicas = 3
partitions = 16
nodes = 64

unless a number is explicitly part of the user's semantic requirement.

Even then, the number is a program constraint, not a grammar-level machine limit.

---

26. Resource integration

Data programs may express:

requirement
constraint
preference
hint
capability
resource
performance
latency
energy
reliability
scalability
portability

These concepts must remain distinct.

For example:

requires low latency

is different from:

prefers accelerator

which is different from:

requires capability X

which is different from:

use device Y

The first three can be portable semantic requirements.

The last one is target-specific and must be explicitly represented as such.

---

27. No hidden hard-coding

The package must never define grammar-level constants such as:

MAX_RECORDS
MAX_FIELDS
MAX_COLLECTIONS
MAX_STREAMS
MAX_PARTITIONS
MAX_REPLICAS
MAX_NODES
MAX_DATASET_SIZE
MAX_PIPELINE_STAGES
MAX_TENSOR_DIMENSIONS

No grammar rule may contain an equivalent hidden restriction.

Repetition operators should remain unbounded at the language level unless a finite cardinality is itself meaningful program semantics.

For example:

field*

is preferable to a fixed enumeration of fields.

---

28. Resource availability

"Unlimited" in POCO-REAF means:

«The language does not impose an artificial machine-scale ceiling.»

It does not mean that physical machines have infinite resources.

Actual execution remains bounded by:

- available memory;
- storage;
- compute;
- network capacity;
- hardware capability;
- runtime policy;
- scheduler capacity;
- compilation resources;
- execution time;
- operating-system limits;
- target constraints.

The grammar must not confuse these implementation limits with language semantics.

---

29. Scalability model

The same data source should be expressible regardless of deployment scale:

one value
    ↓
one record
    ↓
small collection
    ↓
large collection
    ↓
distributed collection
    ↓
stream
    ↓
cluster
    ↓
HPC
    ↓
cloud
    ↓
future execution architecture

No source rewrite should be required merely because the amount of available hardware changes.

---

30. Determinism

Parsing must be deterministic.

The grammar must avoid:

- unnecessary ambiguity;
- overlapping alternatives;
- uncontrolled recursive ambiguity;
- backend-dependent parsing;
- semantic side effects in parser actions.

The grammar must contain no executable target-language actions.

Semantic decisions belong outside the parser.

---

31. Parser safety

The data grammar must:

- perform no filesystem access;
- perform no network access;
- perform no environment discovery;
- perform no hardware discovery;
- perform no runtime calls;
- perform no database access;
- perform no cloud API calls;
- contain no unsafe Rust;
- avoid target-language actions.

A parser must remain a parser.

---

32. Rust requirements

Repository-owned Rust integration for this grammar must support:

Rust 1.97
Rust 1.97.1

and must contain no:

unsafe

The grammar itself must remain independent of Rust implementation details.

Generated parser code must be isolated from handwritten semantic integration.

---

33. AST contract

The grammar produces syntax information.

The AST layer must preserve:

- source locations;
- declarations;
- expressions;
- types;
- transformations;
- data relationships;
- annotations;
- requirements;
- constraints;
- provenance;
- ordering where semantically relevant.

The AST must not prematurely choose:

- database engines;
- storage formats;
- physical devices;
- execution nodes;
- scheduling decisions;
- hardware placement.

---

34. Semantic-analysis contract

Semantic analysis must validate:

- names;
- scopes;
- types;
- generic parameters;
- schema compatibility;
- record compatibility;
- collection element types;
- stream element types;
- transformation compatibility;
- query correctness;
- serialization compatibility;
- requirements;
- constraints;
- capabilities;
- effects;
- resource semantics.

The parser must not attempt to perform these checks.

---

35. IR contract

The grammar must never become a second IR.

The intended boundary is:

Zamani syntax
      ↓
AST
      ↓
semantic model
      ↓
canonical IR

The data grammar must not directly construct:

- scheduler graphs;
- routing graphs;
- hardware instruction sequences;
- quantum circuits;
- database execution plans.

Those belong downstream.

---

36. Optimization integration

Optimization consumes semantic/IR representations.

Data grammar does not perform optimization.

Potential downstream optimizations include:

- fusion;
- projection elimination;
- filter pushdown;
- transformation simplification;
- common-subexpression elimination;
- vectorization;
- tensor optimization;
- accelerator mapping;
- distributed optimization.

These must never alter the meaning of the source program.

---

37. Scheduling integration

Scheduling is downstream.

The grammar may express:

latency preference
ordering requirement
deadline
stream ordering
dependency

but it does not determine:

- execution timestamps;
- machine slots;
- physical resources;
- device assignment;
- scheduling algorithm.

The existing scheduling subsystem owns scheduling semantics.

---

38. Resilience integration

Data grammar may express semantic requirements relevant to resilience.

For example:

- durability;
- replayability;
- checkpoint intent;
- consistency;
- recovery constraints.

It must not own the resilience decision engine.

Resilience determines actions such as:

- retry;
- restart;
- resume;
- rollback;
- reroute;
- reschedule;
- recompile;
- switch backend.

Data syntax merely provides the semantic information required by downstream systems.

---

39. ZQN integration

ZQN describes quantum noise/fault semantics.

Data grammar must not duplicate ZQN.

Where data is associated with:

- measurements;
- experiment results;
- fault telemetry;
- calibration information;

the data model may carry those values.

ZQN remains responsible for quantum fault semantics.

---

40. QEC integration

Data grammar may represent:

- syndrome data;
- measurement data;
- correction metadata;
- experiment records.

It does not implement QEC.

QEC remains the owner of:

- detection;
- correction;
- code semantics;
- decoder semantics;
- logical error handling.

---

41. Serialization boundary

The distinction must remain:

data serialization intent
        ≠
serialization implementation

For example:

serialize value using format F

is syntax.

The implementation of format F belongs downstream.

This permits the same program to run against:

- local storage;
- remote storage;
- distributed storage;
- embedded storage;
- accelerator memory;
- future storage systems.

---

42. Provenance and lineage

Data syntax must be capable of expressing logical provenance.

The semantic model should be able to track:

source
  ↓
transformation
  ↓
derived value
  ↓
materialization
  ↓
consumer

The grammar must not embed implementation-specific tracing systems.

Telemetry and observability remain downstream.

---

43. Security boundary

Data syntax must not silently grant:

- filesystem access;
- network access;
- database access;
- cloud access;
- hardware access.

Permissions and capabilities belong to the security/capability layers.

A source declaration is not itself an authorization.

---

44. Privacy

Data syntax may express logical privacy properties.

It must not implement cryptography.

Cryptographic operations belong to the security/interoperability layers.

Privacy semantics may later be lowered to:

- encryption;
- access control;
- isolation;
- anonymization;
- secure execution;
- differential privacy;
- other supported mechanisms.

---

45. Versioning

Data syntax must be versionable.

Versioning must distinguish:

language version
grammar version
data schema version
serialized representation version
backend implementation version

These must not be conflated.

A data schema evolving from version A to B must not require changing the core language version.

---

46. Compatibility

Compatibility policy must distinguish:

Source compatibility

Old Zamani source continues to parse.

Semantic compatibility

Old source retains its meaning.

AST compatibility

AST representation can evolve without changing language meaning.

IR compatibility

IR versions can evolve independently.

Serialization compatibility

Data representations can migrate independently.

Backend compatibility

The same semantic program can target different implementations.

---

47. Deprecation

Deprecated data syntax must follow the repository-wide compatibility policy.

A deprecated construct must have:

- reason;
- replacement;
- version introduced;
- version deprecated;
- migration guidance;
- diagnostic behavior;
- removal policy.

Deprecated syntax must not disappear silently.

---

48. Diagnostics

The parser/semantic pipeline should provide diagnostics capable of identifying:

- invalid data declaration;
- duplicate field;
- unknown field;
- invalid type;
- incompatible schema;
- invalid transformation;
- invalid stream operation;
- invalid serialization;
- invalid constraint;
- invalid requirement;
- invalid resource expression.

Diagnostics must include source locations.

Diagnostics must not expose provider-specific implementation errors as language syntax errors.

---

49. Testing requirements

The package is incomplete without tests.

Tests must cover every public grammar rule.

Required categories:

tests/data/
├── declarations
├── schemas
├── records
├── collections
├── streams
├── transformations
├── serialization
├── expressions
├── positive
├── negative
├── boundary
├── scalability
├── cross-domain
├── determinism
└── roundtrip

The exact physical test directory may follow the repository's global grammar-test organization.

---

50. Positive tests

Positive tests must cover:

- empty/minimal valid constructs where legal;
- normal declarations;
- deeply nested logical structures;
- generic data;
- transformations;
- pipelines;
- streams;
- schemas;
- records;
- collections;
- serialization;
- distributed intent;
- resource intent;
- quantum/classical data exchange;
- AI data flows;
- HDL/data integration.

---

51. Negative tests

Negative tests must verify rejection of:

- malformed declarations;
- malformed transformations;
- invalid delimiters;
- malformed types;
- malformed schema members;
- malformed stream operators;
- malformed serialization;
- invalid grammar combinations;
- ambiguous syntax;
- accidental backend syntax;
- unsupported provider-specific syntax where prohibited.

---

52. Boundary tests

Boundary tests must deliberately exercise:

- one value;
- one field;
- many fields;
- deeply nested structures;
- long identifiers;
- long qualified names;
- large expression trees;
- large pipelines;
- large transformation chains;
- large schemas;
- large collections;
- large stream descriptions.

No test may assume a universal fixed maximum merely because a test fixture is small.

---

53. Scalability tests

Tests must verify that grammar syntax does not impose artificial limits on:

- record count;
- field count;
- collection count;
- stream count;
- pipeline stages;
- transformations;
- schema members;
- partitions;
- replicas;
- nodes;
- datasets;
- tensor dimensions.

The test suite should use generated cases where appropriate.

The grammar's scalability must not depend on enumerating a fixed number of cases.

---

54. Cross-domain tests

At minimum test:

classical + data
quantum + data
classical + quantum + data
AI + data
AI + quantum + data
HDL + data
hardware + data
distributed + data
networking + data
security + data
resource + data
execution + data

Also test a complete heterogeneous program combining:

classical
+
quantum
+
AI
+
data
+
distributed
+
hardware

The parser must recognize the composition without making one domain subordinate to another.

---

55. POCO-REAF tests

The most important test class is semantic portability.

A logical data program should be representable independently of:

CPU count
GPU count
FPGA count
quantum device
node count
memory capacity
storage capacity
network topology
provider
device identifier
deployment location

The test suite should demonstrate that the same source semantics can feed multiple target configurations without grammar modification.

---

56. Determinism tests

The same source must produce the same:

tokens
parse structure
AST structure
diagnostic classification

independent of:

- machine size;
- available hardware;
- execution environment;
- backend;
- provider.

Runtime resource discovery must never affect parsing.

---

57. Round-trip tests

Where a canonical formatter/printer exists:

source
  ↓
lexer
  ↓
parser
  ↓
AST
  ↓
formatter
  ↓
parser

must preserve semantic meaning.

Whitespace and formatting may change.

Meaning must not.

---

58. Hard-coding audit

Every release must audit the package for:

MAX_*
fixed counts
fixed widths
fixed capacities
fixed devices
fixed nodes
fixed partitions
fixed replicas
fixed memory
fixed storage
fixed topology
fixed addresses
fixed providers

Each finding must be classified as:

1. language semantic requirement;
2. explicit user constraint;
3. target constraint;
4. runtime constraint;
5. implementation limitation;
6. test fixture limitation;
7. accidental hard-coding.

Accidental hard-coding must be removed.

---

59. Provider independence

The grammar must not make any cloud/database/storage provider a language primitive unless explicitly defined as an interoperability dialect.

Provider-specific syntax belongs under:

grammar/dialects/

or:

grammar/interoperability/

and must not contaminate portable data semantics.

---

60. Database independence

A database is an implementation target.

The language's data model must not become equivalent to SQL.

Zamani must be able to represent data computation without requiring:

- relational databases;
- SQL;
- tables;
- rows;
- fixed schemas.

Likewise, SQL interoperability may exist without making SQL the canonical Zamani data model.

---

61. Storage independence

The same logical data abstraction may be lowered to:

RAM
persistent storage
distributed storage
object storage
database
streaming system
accelerator memory
quantum/classical interface
embedded storage
future storage

The grammar must not select one.

---

62. Memory independence

Data grammar must not encode:

- pointer widths;
- alignment;
- cache size;
- memory hierarchy;
- NUMA topology;
- address width;
- memory capacity.

Memory grammar and semantic analysis own memory semantics.

Execution/hardware layers own physical realization.

---

63. Networking independence

A data source/sink may eventually map to a network endpoint.

The data grammar must not define:

- IP addresses;
- ports;
- routing;
- network cards;
- topology;
- protocol implementations.

Networking grammar and runtime own those concepts.

---

64. Parallelism

Data transformations must remain amenable to parallel execution.

For example:

map
filter
reduce
group
aggregate
partition
join

may be lowered to:

- SIMD;
- multicore;
- GPU;
- FPGA;
- distributed;
- accelerator;
- future parallel architectures.

The grammar must describe semantics, not the parallelization strategy.

---

65. Lazy versus eager semantics

Where the language supports lazy/eager data abstractions, the distinction must be semantic.

It must not dictate:

- thread count;
- execution engine;
- scheduling policy;
- memory allocation strategy.

The runtime/compiler chooses the implementation subject to semantic correctness.

---

66. Ordering

Ordering must be explicit when semantically meaningful.

The language must distinguish:

ordered
unordered

where necessary.

The implementation must not accidentally infer deterministic ordering from:

- machine topology;
- thread scheduling;
- storage implementation;
- network ordering.

---

67. Consistency

Consistency declarations belong to logical data semantics.

Possible semantic categories may include:

- strong;
- eventual;
- causal;
- session;
- application-defined.

The grammar must not map them directly to a particular database's implementation.

---

68. Replication

Replication expresses intent.

It must not require a fixed universal replica count.

If a user explicitly requests a replication factor, that is a program-level constraint.

It is not a grammar implementation limit.

---

69. Partitioning

Partitioning must support semantic forms such as:

partition by key
partition by expression
partition by range
partition by logical domain

The grammar must not prescribe:

N partitions

as an implementation maximum.

---

70. Materialization

Materialization expresses whether and when a logical result should become persistent or reusable.

It must not determine:

- disk;
- RAM;
- database;
- object store;
- device memory.

The compiler/runtime selects the implementation.

---

71. Resource hints

Hints must remain non-semantic unless explicitly promoted to requirements.

For example:

prefer accelerator

must not mean:

require accelerator

This distinction is essential for POCO-REAF.

---

72. Requirements versus constraints

A requirement says:

«The program cannot be correctly executed without this property.»

A constraint says:

«An otherwise valid implementation must not violate this condition.»

A preference says:

«Prefer this implementation when possible.»

A hint says:

«This information may help optimization but does not change semantics.»

These must never be conflated.

---

73. Data semantics and physical realization

The central transformation is:

logical data intent
        ↓
semantic validation
        ↓
canonical representation
        ↓
target-independent optimization
        ↓
target/resource-aware lowering
        ↓
scheduling
        ↓
runtime realization

This separation is mandatory.

---

74. Integration with compiler

The compiler must consume semantic data representations rather than raw grammar rules.

The grammar should therefore expose stable parse-tree entry points.

Changing the implementation of a storage backend must not require changing the grammar.

Changing the scheduler must not require changing the grammar.

Changing the quantum hardware backend must not require changing the grammar.

---

75. Integration with runtime

Runtime receives compiled representations and runtime metadata.

The runtime may discover:

- available memory;
- devices;
- accelerators;
- network resources;
- storage;
- quantum backends;
- capabilities.

Such discovery must never modify the meaning of source syntax.

---

76. Integration with hardware abstraction

Hardware abstraction may provide:

capabilities
resources
limits
performance
latency
availability

Data grammar can express requirements/preferences against those abstractions.

It must not embed the hardware abstraction implementation.

---

77. Integration with scheduling

Scheduling may use data semantics to determine:

- dependencies;
- movement;
- ordering;
- resource requirements;
- latency sensitivity;
- parallelism opportunities.

The scheduler owns the resulting schedule.

The data grammar does not.

---

78. Integration with optimization

Optimization may transform data operations while preserving semantic equivalence.

The grammar remains unchanged.

Examples:

filter + map

may become a fused implementation.

map + reduce

may become an accelerator operation.

The source-level semantic meaning remains stable.

---

79. Tooling integration

Tooling should be able to consume the grammar for:

- syntax highlighting;
- completion;
- diagnostics;
- formatting;
- navigation;
- documentation generation;
- refactoring;
- semantic inspection;
- language-server support.

Tooling must use the same grammar authority.

It must not maintain a separate undocumented syntax definition.

---

80. Documentation integration

The following must remain synchronized:

grammar/data/README.md
grammar/data/*.g4
grammar/Zamani-Grammar.md
grammar/grammar.md
grammar/README.md

The grammar files remain the executable syntax authority.

Documentation explains the language and architecture.

No documentation-only construct should be presented as implemented syntax.

---

81. Grammar authority

The repository must maintain one authoritative language syntax.

Specialized grammar files are modular components of that authority.

They are not independent competing languages.

The integration chain is:

Zamani.g4 / authoritative lexer vocabulary
          ↓
specialized grammar components
          ↓
root parser

Generated artifacts must never become a second source of truth.

---

82. Integration rule for specialized files

Every specialized ".g4" file must document:

Purpose
Owns
Does not own
Public entry rules
Token dependencies
Type dependencies
Expression dependencies
AST expectations
Semantic expectations
Downstream consumers
Tests
Scalability policy
Hard-coding audit
Completion criteria

A file is not complete merely because ANTLR accepts it.

---

83. Independent completion contract

A completed data grammar file must remain complete when another data grammar file is implemented later.

Therefore each file must establish its integration points before implementation.

For example:

"records.g4" must not later be redesigned merely because "collections.g4" is added.

"streams.g4" must not later redefine transformations owned by "transformations.g4".

"serialization.g4" must not later redefine data types.

"data.g4" must delegate rather than duplicate.

---

84. Dependency order

The recommended implementation order is:

1. grammar/data/README.md
       ↓
2. shared lexer/core contracts
       ↓
3. schemas.g4
       ↓
4. records.g4
       ↓
5. collections.g4
       ↓
6. streams.g4
       ↓
7. transformations.g4
       ↓
8. serialization.g4
       ↓
9. data.g4 integration facade
       ↓
10. AST integration
       ↓
11. semantic validation
       ↓
12. canonical data representation
       ↓
13. compiler integration
       ↓
14. optimization
       ↓
15. scheduling
       ↓
16. runtime/resource integration
       ↓
17. cross-domain tests

The exact order may be adjusted only when repository dependencies require it.

---

85. Completion criteria for "grammar/data/"

The package is complete only when:

- every public grammar rule has an owner;
- no construct has two authoritative owners;
- no lexer rules are duplicated;
- no type system is duplicated;
- no expression system is duplicated;
- no IR is created in grammar;
- data semantics are backend-independent;
- quantum semantics remain owned by the quantum subsystem;
- "quantum::ir" remains the canonical quantum boundary;
- hardware remains downstream;
- scheduling remains downstream;
- optimization remains downstream;
- runtime discovery remains downstream;
- resource requirements remain distinct from implementation choices;
- parser behavior is deterministic;
- diagnostics are source-located;
- positive tests exist;
- negative tests exist;
- boundary tests exist;
- scalability tests exist;
- cross-domain tests exist;
- round-trip tests exist where supported;
- hard-coding audits pass;
- Rust integration supports Rust 1.97/1.97.1;
- no Zamani-owned Rust code uses "unsafe";
- documentation agrees with implemented syntax;
- generated artifacts are not treated as source authority.

---

86. Definition of production-ready

"grammar/data/" is production-ready only when all of the following are true:

Portable semantics
        +
Deterministic parsing
        +
Explicit ownership
        +
Stable integration contracts
        +
Canonical type integration
        +
Canonical AST integration
        +
Canonical IR boundary
        +
No backend leakage
        +
No accidental hard-coding
        +
Strong diagnostics
        +
Version compatibility
        +
Scalability validation
        +
Cross-domain validation
        +
Repository integration
        =
Production-ready Zamani data grammar

---

87. Final invariant

The following invariant must hold forever:

«A Zamani data program describes data and computation, not the accidental limitations of the machine currently available.»

Therefore:

one source program
        ↓
one semantic meaning
        ↓
many data sizes
        ↓
many machines
        ↓
many memory configurations
        ↓
many storage systems
        ↓
many accelerators
        ↓
many distributed deployments
        ↓
many hardware architectures
        ↓
future execution environments

without requiring the developer to rewrite the semantic program merely because the available resources changed.

---

88. POCO-REAF invariant

The data grammar participates in:

Program Once
      ↓
Compile Once
      ↓
Run Everywhere
      ↓
Run Anywhere
      ↓
Run Forever

by ensuring that:

semantic intent

is separated from:

physical realization

The language therefore remains scalable from the smallest meaningful computation to arbitrarily large executions permitted by available resources.

---

89. Final ownership statement

"grammar/data/" is the portable language-level data semantics syntax layer.

It is not:

a database language
a storage engine
a distributed engine
a networking engine
a hardware description
a quantum IR
a scheduler
an optimizer
a runtime

It is the language boundary through which Zamani programs describe data computation in a machine-independent way.

The architectural rule is:

«Describe the data and its intended computation once. Let semantic analysis, compilation, optimization, scheduling, resource management, hardware abstraction, and runtime systems determine how that meaning is realized on the available machine.»

That is the data-layer requirement for Zamani: From Atom to Everywhere and POCO-REAF.
Zamani Data Grammar

Path: "grammar/data/"
Language: Zamani
Repository: "Benwellonedge28/Zamani"
Default branch inspected: "main"
Rust baseline: Rust 1.97 / Rust 1.97.1
Rust edition: Rust 2021
Rust safety: Zamani-owned production Rust MUST NOT use "unsafe"
Grammar technology: ANTLR-compatible parser grammars
Primary objective: "Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)"

---

1. Status

This document is the normative architectural and integration contract for the "grammar/data/" package.

It defines:

- the responsibility of the data grammar;
- ownership boundaries between data grammar files;
- integration with the canonical Zamani lexer;
- integration with the canonical Zamani parser;
- integration with the frontend AST;
- integration with semantic analysis;
- integration with the canonical semantic/IR architecture;
- integration with classical computation;
- integration with quantum/classical computation;
- integration with AI;
- integration with HDL and hardware/software co-design;
- integration with distributed execution;
- integration with networking;
- integration with resources and capabilities;
- integration with compilation and execution;
- compatibility requirements;
- diagnostics;
- determinism;
- scalability;
- security and privacy boundaries;
- testing;
- hard-coding prevention;
- independent file completion.

This file does not claim that every data feature currently implemented in the repository is production-complete.

A grammar feature is production-complete only when its complete pipeline exists:

Specification
    ↓
Lexical contract
    ↓
Grammar
    ↓
Lexer
    ↓
Parser
    ↓
Frontend AST
    ↓
Structural validation
    ↓
Name/module resolution
    ↓
Type analysis
    ↓
Effect analysis
    ↓
Resource/capability analysis
    ↓
Semantic representation
    ↓
Canonical IR
    ↓
Optimization/lowering
    ↓
Scheduling/routing where applicable
    ↓
Compiler/backend
    ↓
Runtime
    ↓
Tests

A parser-only feature is therefore not automatically a stable language feature.

---

2. Repository Inspection Baseline

The current repository was inspected before defining this contract.

Relevant existing files include:

grammar/DESIGN.md
grammar/README.md
grammar/Zamani.g4
grammar/grammar.md
grammar/Zamani-Grammar.md

grammar/data/README.md
grammar/data/data.g4
grammar/data/queries.g4
grammar/data/datasets.g4
grammar/data/transformations.g4
grammar/data/tables.g4
grammar/data/collections.g4
grammar/data/streams.g4
grammar/data/serialization.g4

The inspected repository also contains the broader grammar domains for:

ai/
classical/
compile/
concurrency/
core/
declarations/
dialects/
distributed/
effects/
execution/
expressions/
functions/
hardware/
hdl/
hybrid/
interoperability/
lexer/
macros/
memory/
metaprogramming/
modules/
networking/
quantum/
resources/
security/
spec/
specification/
statements/
tests/
types/
validation/

The data package therefore MUST integrate with the existing architecture instead of creating another independent language subsystem.

---

3. Current Data Grammar Reality

The current data package is already more developed than a minimal facade.

The inspected "data.g4" contains a public:

dataStmt

with common data declarations and statements.

It currently contains constructs for concepts including:

schema
record
collection
sequence
stream
source
sink
pipeline
view
transform
contract
partition
distribution
replication
provenance

It also references specialized constructs.

The specialized files currently expose different public entry conventions.

Examples include:

queries.g4
    → dataQueryConstruct

datasets.g4
    → datasetConstruct

transformations.g4
    → dataTransformationConstruct

tables.g4
    → tableDeclaration
    → tableExpression
    → tableStatement

collections.g4
    → dataCollectionDeclaration
    → collectionExpression
    → collectionStatement

streams.g4
    → streamDeclaration

serialization.g4
    → serializationUnit

These differences MUST be treated as an existing integration fact.

The solution is not to rename all existing files.

The solution is to establish stable integration contracts and, where necessary, provide compatibility wrappers.

---

4. Important Existing Integration Differences

The current data grammars do not all use exactly the same shared-rule vocabulary.

Examples observed include combinations of:

expression
typeExpr
typeExpression
identifier
IDENTIFIER
qualifiedName

They also use both symbolic token names and literal syntax in different places.

This is an integration concern.

It MUST NOT be solved by creating another lexer or another universal expression/type system.

The canonical architecture remains:

canonical lexer
        ↓
canonical parser vocabulary
        ↓
canonical expressions
        ↓
canonical types
        ↓
data grammar

Where an existing specialized grammar currently uses a compatibility spelling, the integration layer MUST normalize it rather than creating a competing language authority.

---

5. Scope

"grammar/data/" defines the language-level syntax of data computation and data intent.

It covers logical concepts such as:

- values;
- records;
- schemas;
- collections;
- datasets;
- streams;
- tables;
- sources;
- sinks;
- queries;
- transformations;
- pipelines;
- views;
- serialization;
- materialization;
- provenance;
- lineage;
- validation;
- partitioning;
- distribution;
- replication;
- consistency;
- data contracts;
- data movement;
- data quality;
- logical data resources;
- data requirements;
- data capabilities;
- data preferences;
- data hints.

The package MUST remain extensible to:

- classical computing;
- scientific computing;
- numerical computing;
- symbolic computing;
- HPC;
- AI/ML;
- tensor computing;
- quantum/classical workflows;
- distributed systems;
- streaming systems;
- embedded systems;
- edge systems;
- cloud systems;
- accelerator systems;
- hardware/software co-design;
- future computational architectures.

---

6. Core Principle

The data grammar describes:

WHAT the data means
WHAT computation is required
WHAT transformations are intended
WHAT relationships exist
WHAT guarantees are required
WHAT resources are logically required
WHAT capabilities are required
WHAT constraints apply
WHAT preferences exist
WHAT provenance must be preserved

It does not describe:

WHICH CPU
WHICH GPU
WHICH FPGA
WHICH QPU
WHICH database server
WHICH storage device
WHICH memory bank
WHICH physical address
WHICH network node
WHICH cloud provider
WHICH accelerator instance

Those belong downstream.

The architectural boundary is:

Data source semantics
        ↓
Data AST
        ↓
Data semantic model
        ↓
Canonical IR
        ↓
Optimization
        ↓
Resource/capability analysis
        ↓
Scheduling
        ↓
Target lowering
        ↓
Runtime

---

7. POCO-REAF

The data layer participates directly in:

Program
   ↓
Once
   ↓
Compile
   ↓
Once
   ↓
Run
   ↓
Everywhere
   ↓
Anywhere
   ↓
Forever

The formal invariant is:

«A data program MUST describe logical data computation independently of the accidental characteristics of the machine on which it will eventually execute.»

A program may therefore scale from:

one value

to:

one record
one collection
one dataset
one stream
large datasets
distributed datasets
HPC workloads
accelerated workloads
heterogeneous workloads
future computational systems

without changing its semantic algorithm merely because available resources changed.

---

8. Meaning of "Infinity"

"Infinity" in POCO-REAF does not mean that physical computers possess infinite resources.

It means:

«The Zamani language MUST NOT impose an artificial finite ceiling where the underlying semantic model could naturally scale.»

Actual execution remains bounded by:

- available memory;
- available storage;
- compiler resources;
- runtime resources;
- target capabilities;
- execution time;
- operating-system policies;
- scheduler capacity;
- network capacity;
- physical device capabilities;
- explicitly declared constraints.

These are resource/implementation constraints, not language-level artificial limits.

---

9. Hard-Coding Prohibition

The data grammar MUST NOT define universal constants such as:

MAX_RECORDS
MAX_FIELDS
MAX_COLUMNS
MAX_COLLECTIONS
MAX_DATASETS
MAX_STREAMS
MAX_PIPELINE_STAGES
MAX_TRANSFORMATIONS
MAX_PARTITIONS
MAX_REPLICAS
MAX_NODES
MAX_MEMORY
MAX_STORAGE
MAX_TENSOR_RANK
MAX_TENSOR_DIMENSIONS
MAX_DEVICES
MAX_ACCELERATORS
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_QUBITS

Nor may it encode hidden equivalents such as:

field1
field2
...
field32

as the language's universal data model.

Likewise, it MUST NOT assume:

64 GB memory
24 GB accelerator memory
32-bit registers
1024 nodes
16 partitions
3 replicas
fixed tensor rank
fixed stream length
fixed number of pipeline stages

unless such values are explicitly supplied as program semantics.

---

10. Program Values Are Not Language Limits

This is valid:

let n = 1024;

This is also valid:

allocate n;

The value "1024" is program data.

What is prohibited is:

MAX_DATASET_SIZE = 1024

when that value means Zamani cannot represent a larger dataset.

Therefore:

program value

and:

language implementation limit

MUST remain separate concepts.

---

11. Resource Requirements

The data grammar may express logical resource requirements.

Examples:

requires memory >= required_memory
requires capability("distributed.data")
requires capability("streaming.data")
requires capability("accelerated.data")
requires capability("persistent.storage")

These express semantic requirements.

They do not specify physical realization.

For example:

requires capability("distributed.data")

MUST NOT silently become:

use 64 nodes

The actual resource manager determines realization.

---

12. Requirement / Constraint / Capability / Preference / Hint

The data layer MUST preserve the distinction between:

Requirement

The program cannot correctly execute without the property.

requires capability("streaming.data")

Constraint

A valid realization must satisfy the condition.

requires latency <= budget

Capability

The target must provide a capability.

requires capability("distributed.partitioning")

Preference

Prefer a realization but permit alternatives.

prefer capability("accelerated.data")

Hint

Provide optimization information without changing semantics.

hint locality

Implementation decision

A target-specific realization selected downstream.

Examples include:

physical storage location
device selection
node placement
memory bank
network route

The data grammar MUST NOT confuse these categories.

---

13. Ownership

"grammar/data/" owns:

Data structure

- schemas;
- records;
- collections;
- datasets;
- streams;
- tables;
- views;
- logical sources;
- logical sinks.

Data computation

- transformations;
- queries;
- projections;
- filtering;
- grouping;
- aggregation;
- joins;
- ordering;
- windowing;
- partitioning;
- reshaping;
- data movement intent.

Data lifecycle

- loading;
- saving;
- materialization;
- persistence intent;
- caching intent;
- serialization intent.

Data correctness

- validation;
- constraints;
- contracts;
- quality requirements;
- consistency intent.

Data lineage

- provenance;
- lineage;
- version intent;
- schema evolution intent.

Data distribution

- logical partitioning;
- replication intent;
- distribution intent;
- locality intent.

---

14. Non-Ownership

The data grammar does NOT own:

- lexical implementation;
- identifiers;
- universal expressions;
- universal types;
- general control flow;
- functions;
- modules;
- ownership;
- physical memory;
- database engines;
- filesystem implementations;
- object stores;
- network protocols;
- network routing;
- hardware discovery;
- hardware topology;
- quantum hardware;
- quantum gates;
- quantum IR;
- QEC;
- ZQN;
- HAL;
- scheduling implementation;
- optimization implementation;
- accelerator implementation;
- cloud-provider implementation;
- SQL execution engine;
- storage engine;
- cryptographic implementation;
- runtime implementation.

The distinction is:

grammar
    → syntax

semantic analysis
    → meaning

IR
    → canonical representation

compiler
    → realization

runtime
    → execution

---

15. Existing File Ownership

The existing filenames MUST be retained unless a concrete technical reason requires otherwise.

The intended ownership is:

File| Responsibility
"README.md"| Package architecture and integration contract
"data.g4"| Common data-domain syntax and existing data entry boundary
"queries.g4"| Query syntax
"datasets.g4"| Dataset-specific syntax
"transformations.g4"| Generic data transformation syntax
"tables.g4"| Table-specific syntax
"collections.g4"| Collection-specific syntax
"streams.g4"| Stream-specific syntax
"serialization.g4"| Serialization/deserialization syntax

Additional files MAY be created when a responsibility cannot be cleanly owned by an existing file.

Do not create files merely for organizational appearance.

---

16. "data.g4"

"data.g4" is the common data-domain grammar boundary.

Its current public entry is:

dataStmt

It currently contains common declarations and statements.

It may own:

dataDeclaration
dataStatement
dataReference
qualifiedDataName
dataSchemaDecl
dataRecordDecl
dataCollectionDecl
dataSequenceDecl
dataStreamDecl
dataSourceDecl
dataSinkDecl
dataPipelineDecl
dataViewDecl
dataTransformDecl
dataContractDecl
dataPartitionDecl
dataDistributionDecl
dataReplicationDecl
dataProvenanceDecl

It MUST NOT become a second implementation of every specialized data grammar.

Specialized constructs should remain owned by their dedicated files.

---

17. Specialized Grammar Integration

The package currently exposes several different public rule names.

The integration contract is therefore:

dataStmt
    ↓
common data construct
    OR
specialized data construct

with the following established mappings:

queries.g4
    → dataQueryConstruct

datasets.g4
    → datasetConstruct

transformations.g4
    → dataTransformationConstruct

tables.g4
    → tableDeclaration / tableExpression / tableStatement

collections.g4
    → dataCollectionDeclaration / collectionExpression / collectionStatement

streams.g4
    → streamDeclaration

serialization.g4
    → serializationUnit

These names are integration identifiers, not a requirement to rename the existing rules.

If the root composition layer needs a single normalized entry point, a compatibility wrapper MUST be added at the appropriate composition boundary rather than forcing unrelated grammar files to be rewritten.

---

18. No Duplicate Type System

Data grammar MUST use the canonical Zamani type system.

It MUST NOT define another independent type system.

The intended dependency is:

grammar/types/
       ↓
canonical type expression
       ↓
grammar/data/

Data may therefore use types representing:

scalar
record
collection
stream
tensor
array
map
generic
resource
quantum
hardware
future types

without owning the semantics of those types.

---

19. No Duplicate Expression System

Data grammar MUST use the canonical expression system.

It MUST NOT redefine:

- arithmetic precedence;
- boolean operators;
- comparison semantics;
- function calls;
- assignment;
- indexing;
- member access;
- general lambdas;
- universal literals.

Data-specific syntax may consume:

expression

or its canonical equivalent.

The data grammar owns the context, not the universal expression language.

---

20. Identifier Contract

Identifiers MUST come from the canonical Zamani lexical system.

Data grammar MUST NOT create another identifier lexer.

Qualified names must use the repository's canonical naming model.

The data layer must support arbitrarily large logical namespaces subject only to implementation resources.

---

21. Schema Contract

A schema describes logical structure.

A schema MAY describe:

- fields;
- field types;
- optionality;
- nullability;
- defaults;
- constraints;
- keys;
- indexes as logical intent;
- annotations;
- evolution metadata.

A schema MUST NOT silently mean:

database table
memory layout
wire protocol
hardware register map
physical storage block

Those are downstream representations.

---

22. Record Contract

A data record is a logical data construct.

It MUST NOT be confused with:

- CPU registers;
- HDL registers;
- quantum registers;
- database implementation rows;
- memory records.

The grammar MUST NOT impose limits on:

- field count;
- record count;
- record size;
- nesting depth;
- collection size.

Any practical limit belongs to implementation/resource analysis.

---

23. Collection Contract

Collections are logical abstractions.

A collection may eventually be realized as:

local memory
persistent storage
database
distributed collection
stream-backed collection
accelerator representation
embedded representation
future representation

The grammar MUST NOT select one representation unless explicitly using an interoperability dialect.

No universal:

MAX_ELEMENTS
MAX_COLLECTION_SIZE
MAX_PARTITIONS

may exist.

---

24. Dataset Contract

Datasets are logical data resources.

A dataset may be:

local
remote
persistent
ephemeral
stream-derived
generated
distributed
partitioned
versioned
materialized
virtual

The dataset grammar MUST describe semantics rather than storage implementation.

A dataset size is program/resource information, not a grammar maximum.

---

25. Query Contract

The existing "queries.g4" owns query syntax.

Its responsibilities include concepts such as:

- selection;
- projection;
- filtering;
- grouping;
- aggregation;
- ordering;
- joins;
- windows;
- common table expressions;
- set operations;
- query properties;
- query execution intent.

The query grammar MUST remain independent of a specific database engine.

SQL interoperability may exist, but SQL MUST NOT become the canonical Zamani data model merely because some query targets use SQL.

---

26. Transformation Contract

The existing "transformations.g4" owns generic data transformation syntax.

Transformations may represent operations such as:

map
filter
reduce
fold
scan
group
join
sort
distinct
flatten
explode
pivot
unpivot
aggregate
window
resample
interpolate
normalize
standardize
encode
decode
project
select
rename
drop
fill
replace
derive
compute
validate
sample
split
batch
shuffle
cache
materialize
persist
load
save
serialize
deserialize

These are semantic operations.

The grammar MUST NOT encode how they execute.

For example:

map(f)

does not imply:

CPU
GPU
FPGA
single-threaded
one node
specific vector width
specific memory layout

The compiler may select an implementation preserving semantics.

---

27. Stream Contract

The existing "streams.g4" owns stream-specific syntax.

Streams may be:

- bounded;
- unbounded;
- ordered;
- unordered;
- replayable;
- non-replayable;
- lossless;
- lossy;
- persistent;
- ephemeral.

The grammar MUST NOT hard-code:

maximum events
maximum event size
maximum throughput
maximum buffer
maximum partitions
maximum consumers
maximum producers

Stream execution may eventually use:

one process
many processes
one machine
many machines
CPU
GPU
FPGA
accelerator
distributed runtime
future runtime

without changing the source semantics.

---

28. Table Contract

The existing "tables.g4" owns table-specific syntax.

Tables are logical data structures.

They MUST NOT automatically imply:

SQL database
specific database vendor
fixed number of columns
fixed row size
fixed partition count
fixed index count

Logical indexes and ordering may be represented as intent.

Physical indexing remains downstream.

---

29. Serialization Contract

The existing "serialization.g4" owns serialization syntax.

Serialization may express:

serialize
deserialize
encode
decode
format
schema
version
compatibility
framing
compression intent
integrity intent

The grammar does not implement serialization.

Possible implementations include:

binary
text
streaming
canonical
zero-copy
distributed
accelerator-specific
quantum/classical boundary
future formats

without requiring changes to the language's semantic data model.

---

30. Data Pipelines

A pipeline represents logical data computation.

A pipeline may contain arbitrarily many logical stages:

source
    ↓
transform
    ↓
filter
    ↓
join
    ↓
aggregate
    ↓
materialize
    ↓
sink

The grammar MUST NOT establish a maximum number of stages.

Pipeline parallelism belongs to compiler/runtime scheduling.

---

31. Data Mining Integration

Data mining is a data computation domain.

A future or existing:

grammar/data/mining.g4

MUST integrate through the same package contract.

It may express:

- mining sources;
- features;
- targets;
- labels;
- models;
- mining methods;
- objectives;
- metrics;
- preprocessing;
- validation;
- evaluation;
- sampling;
- model parameters;
- provenance;
- explainability;
- distributed execution intent;
- streaming/incremental mining;
- resource/capability requirements.

It MUST NOT create:

a second dataset grammar
a second tensor grammar
a second type system
a second query language
a second IR
a hardware-specific mining model

Mining must consume the existing data abstractions.

---

32. AI Integration

AI grammar belongs under the AI domain.

The data package provides general data structures used by AI.

The relationship is:

AI
 ↓
data
 ↓
canonical types / expressions
 ↓
semantic model

AI-specific concepts such as:

model
training
inference
agent
neural architecture
differentiation

should not be duplicated inside generic data grammar.

Likewise, data transformations must remain usable by future AI systems without requiring the data grammar to understand every AI framework.

---

33. Tensor Integration

Tensor syntax belongs to the canonical tensor/type/data architecture rather than being duplicated in every data subsystem.

The number of tensor dimensions MUST NOT be hard-coded.

Valid program semantics may contain:

Tensor<T, shape>

or another canonical tensor representation.

The grammar must not impose:

MAX_TENSOR_RANK
MAX_TENSOR_DIMENSION

Tensor operations may be lowered to:

CPU
SIMD
GPU
FPGA
accelerator
distributed execution
quantum/classical workflows
future hardware

where semantically applicable.

Repository inspection note: the requested "grammar/data/tensors.g4" path did not resolve on the inspected default branch. If tensor grammar exists under another current path, that existing authority MUST be used rather than creating a duplicate. If a tensor grammar is added, it must first establish its ownership and integration contract here and with "grammar/types/".

---

34. Quantum Integration

The data layer may carry classical data associated with quantum computation.

Examples include:

measurement results
parameter values
optimization results
experiment metadata
classical control data
calibration metadata
provenance

The data grammar MUST NOT own:

qubits
physical qubits
quantum gates
quantum topology
QEC
noise models
pulse implementation
quantum hardware
quantum::ir

Those remain owned by the quantum subsystem.

The canonical architecture is:

data semantics
      ↓
classical semantic representation
      ↓
hybrid semantic analysis
      ↓
quantum semantic boundary
      ↓
quantum::ir

No second quantum IR may be created in "grammar/data/".

---

35. Quantum-Classical Data Flow

A valid heterogeneous workflow may look conceptually like:

classical data
      ↓
parameter preparation
      ↓
quantum computation
      ↓
measurement
      ↓
data result
      ↓
classical analysis
      ↓
optimization
      ↓
next quantum execution

The data grammar describes the data-side semantics.

Quantum semantics remain owned by the quantum subsystem.

The semantic layer must preserve:

- types;
- ownership;
- provenance;
- ordering;
- measurement meaning;
- error semantics;
- resource requirements.

---

36. HDL Integration

The data grammar may describe logical data consumed or produced by HDL computation.

It MUST NOT redefine:

wire
port
register
clock
state machine
hardware process

Those belong to "grammar/hdl/".

The boundary is:

logical data
    ↓
HDL semantic interpretation
    ↓
hardware representation

not:

data grammar
    ↓
physical hardware

---

37. Hardware Independence

The data grammar MUST remain independent of:

CPU model
GPU model
FPGA family
ASIC
QPU
device identifier
memory bank
PCI address
physical memory address
network card
cloud provider
database server

A compiler may use hardware capabilities to select a realization.

The source-level data semantics must remain unchanged.

---

38. Distributed Data

Data syntax must support logical distributed concepts such as:

- partitioning;
- replication;
- distribution;
- locality;
- consistency;
- remote sources;
- remote sinks;
- distributed streams;
- distributed collections;
- checkpoints;
- lineage.

The grammar MUST NOT impose:

node 0
node 1
node 2

as the universal model.

Likewise:

replicas = 3
partitions = 16
nodes = 64

are valid only when explicitly expressed as program-level requirements or constraints.

They are never grammar-level maximums.

---

39. Partitioning

Partitioning describes logical data organization.

Examples include:

partition by key
partition by expression
partition by range
partition by logical domain

The grammar does not determine:

physical partition count
physical node placement
network topology
storage location

Those decisions belong downstream.

---

40. Replication

Replication expresses logical intent.

The grammar MUST NOT define a universal maximum replica count.

If a program explicitly requires a replication factor, that is program semantics.

For example:

requires replication >= desired_replication

is different from:

the language supports at most N replicas

The second is prohibited as a universal grammar limitation.

---

41. Consistency

Logical consistency semantics may include:

strong
eventual
causal
session
application-defined

where these are defined by the canonical semantic specification.

The data grammar MUST NOT equate these concepts with one database vendor's implementation.

---

42. Ordering

The grammar must distinguish ordering when ordering is semantically observable.

Possible logical properties include:

ordered
unordered

An unordered data abstraction MUST NOT accidentally become deterministic merely because a particular implementation happens to return data in a particular order.

Likewise, an ordered abstraction MUST preserve the specified ordering semantics through parallel and distributed lowering.

---

43. Lazy and Eager Semantics

Where supported, lazy/eager semantics are semantic properties.

They MUST NOT imply:

thread count
memory size
scheduler implementation
execution engine
device

A compiler may fuse or reorder operations when semantic guarantees permit.

---

44. Materialization

Materialization expresses a semantic lifecycle property.

It does not select:

RAM
disk
database
object store
accelerator memory
distributed storage

The implementation determines where and how materialization occurs.

---

45. Provenance and Lineage

Data grammar may express:

provenance
lineage
source identity
transformation history
version
derivation

These properties are important for:

- reproducibility;
- auditing;
- scientific computing;
- AI training;
- distributed computation;
- security;
- compliance;
- debugging.

The grammar does not implement provenance storage.

---

46. Security Boundary

Data grammar may express logical security or privacy requirements.

It MUST NOT implement cryptography.

For example:

requires capability("protected.data")

may be valid semantic intent.

Actual implementation may use:

encryption
isolation
authorization
secure execution
privacy mechanisms
secure computation

through the security/runtime layers.

Data syntax is not an authorization system.

---

47. Networking Boundary

A data source or sink may eventually map to a network endpoint.

The data grammar MUST NOT own:

IP addressing
routing
network interfaces
transport implementation
network topology
socket implementation

These belong to networking/runtime layers.

---

48. Storage Independence

Logical data may eventually be represented by:

RAM
persistent storage
object storage
database
distributed storage
streaming infrastructure
accelerator memory
embedded storage
future storage

The data grammar must remain storage-independent.

Changing the storage backend must not require changing the logical data program.

---

49. Memory Independence

The data grammar MUST NOT encode:

pointer width
address width
alignment
cache size
NUMA topology
RAM capacity
VRAM capacity
memory-bank count

Memory semantics belong to "grammar/memory/" and the semantic/compiler layers.

Physical memory realization belongs downstream.

---

50. Classical Computing Integration

Data operations should be capable of lowering to:

scalar execution
vector execution
multicore execution
SIMD
GPU execution
FPGA acceleration
accelerators
HPC

without changing the logical data program.

For example:

map
filter
reduce
aggregate
join

may be parallelized when semantics permit.

The data grammar does not dictate the parallelization strategy.

---

51. Parallelism

The data grammar MUST NOT require a fixed worker count.

The program may express:

parallel

without requiring:

8 threads
16 cores
4 GPUs
64 nodes

unless such a number is explicitly part of the program's semantic requirements.

The compiler/runtime determines the available parallel realization.

---

52. Compiler Integration

The compiler MUST consume semantic data representations.

It MUST NOT infer program meaning from undocumented grammar conventions.

The intended pipeline is:

Zamani source
    ↓
lexer
    ↓
parser
    ↓
data parse tree
    ↓
frontend AST
    ↓
semantic data model
    ↓
canonical IR
    ↓
optimization
    ↓
target lowering

Changing a storage backend must not require changing the grammar.

Changing a scheduler must not require changing the grammar.

Changing a database engine must not require changing the grammar.

Changing a quantum backend must not require changing the data grammar.

---

53. AST Contract

"grammar/data/" produces syntax.

It MUST NOT define a parallel data AST implementation.

Every construct must have a predetermined semantic mapping.

Conceptually:

dataSchemaDecl
    ↓
domain-neutral declaration/data AST
    ↓
semantic schema

dataRecordDecl
    ↓
domain-neutral declaration/data AST
    ↓
semantic record

dataCollectionDecl
    ↓
domain-neutral declaration/data AST
    ↓
semantic collection

dataStreamDecl
    ↓
domain-neutral declaration/data AST
    ↓
semantic stream

dataTransformationConstruct
    ↓
domain-neutral operation AST
    ↓
semantic transformation

Exact Rust type names remain owned by the canonical frontend AST.

---

54. AST Information Preservation

The AST MUST preserve every source property needed downstream.

No grammar feature may be accepted and then silently discard:

- source location;
- names;
- arguments;
- type information;
- attributes;
- requirements;
- constraints;
- capabilities;
- preferences;
- ordering semantics;
- provenance;
- version information;
- explicit resource requirements.

If information is semantically meaningful, it must survive into the semantic representation.

---

55. Canonical IR

The data grammar MUST NOT create a separate data-specific compiler IR unless the repository's canonical IR architecture explicitly defines one.

The intended boundary is:

Data syntax
    ↓
Data AST
    ↓
Data semantics
    ↓
Canonical semantic/IR boundary

The resulting representation may subsequently be lowered to:

classical execution
GPU
FPGA
distributed execution
streaming execution
accelerators
quantum/classical workflows
hardware
future targets

without modifying the source-level semantics.

---

56. Quantum IR Boundary

If data participates in a hybrid program:

data
   ↓
hybrid semantic analysis
   ↓
quantum semantics
   ↓
quantum::ir

"quantum::ir" remains the canonical quantum semantic boundary.

"grammar/data/" MUST NOT create:

DataQuantumIR
QuantumDataIR
MiningQuantumIR
TensorQuantumIR

as competing representations.

---

57. Optimization

Optimization belongs downstream.

The compiler may transform:

filter
+
map

into a fused operation.

It may transform:

map
+
reduce

into an accelerator implementation.

It may transform:

partition
+
aggregate

into a distributed strategy.

Provided semantic equivalence is preserved, these are implementation transformations rather than grammar changes.

---

58. Scheduling

Scheduling belongs downstream.

The scheduler determines:

- execution order;
- parallel execution;
- resource assignment;
- data movement timing;
- device utilization;
- synchronization;
- distributed coordination.

The grammar describes semantic dependencies and constraints.

It does not own the resulting physical schedule.

---

59. Runtime

Runtime may discover:

available memory
available storage
devices
accelerators
network resources
distributed resources
quantum resources
capabilities
availability

Runtime discovery MUST NOT modify the meaning of the source program.

If a target cannot satisfy requirements, the system must:

report a precise diagnostic

or use an explicitly permitted alternative realization.

It must not silently change the data computation.

---

60. Hardware Capability Integration

Hardware/resource systems may report:

capability
resource
capacity
availability
performance
latency
energy
reliability
topology

Data programs may express requirements against these abstractions.

For example:

requires capability("accelerated.data")

is portable.

A physical device selection such as:

use device X

is target realization and must not become implicit language semantics.

---

61. AI / ML Data Portability

The data layer must support AI systems without depending on a particular framework.

The same logical dataset should be usable by:

classical ML
neural computation
symbolic computation
probabilistic computation
distributed training
inference
quantum-assisted workflows
future AI systems

The grammar must not make:

PyTorch
TensorFlow
JAX
framework-specific APIs
provider-specific services

part of the canonical Zamani data language.

Such integrations belong under interoperability or dialects.

---

62. Scientific Computing

The data grammar must support data semantics used by:

- simulations;
- numerical computation;
- statistics;
- signal processing;
- scientific datasets;
- symbolic computation;
- experimental data;
- HPC.

Scientific data size must remain resource-driven.

The grammar must not encode a maximum matrix, tensor, dataset, or experiment size.

---

63. Embedded and Edge Computing

Small targets may have severe physical constraints.

These are target constraints.

They MUST NOT redefine the language.

A program may be valid while a particular embedded target is unable to satisfy:

required memory
required capability
required storage
required precision
required execution guarantees

The compiler should report the resource incompatibility.

---

64. Cloud Independence

Cloud execution is a deployment concern.

The canonical data grammar MUST NOT require a particular cloud provider.

Provider-specific data features belong under:

grammar/dialects/

or:

grammar/interoperability/

and must be explicitly identified as non-portable or conditionally portable.

---

65. Database Independence

Zamani's data model MUST NOT become synonymous with SQL.

SQL interoperability is allowed.

The canonical language must remain capable of expressing data computation without requiring:

SQL
relational databases
tables
rows
fixed relational schemas

A database is one possible implementation target.

---

66. Serialization Independence

Serialization format is an implementation/interoperability concern.

A program may specify serialization intent without becoming tied to one format.

The architecture supports:

logical value
    ↓
serialization intent
    ↓
semantic validation
    ↓
format selection
    ↓
backend implementation

Changing the serializer should not change the logical data model.

---

67. Versioning

Data versioning must distinguish:

Zamani language version
grammar version
data schema version
serialized representation version
backend version
runtime version

These MUST NOT be conflated.

A schema evolution should not require a language-version change unless language syntax or semantics actually changed.

---

68. Compatibility

Compatibility has multiple dimensions.

Source compatibility

Existing valid source continues to parse.

Semantic compatibility

Existing source retains its meaning.

AST compatibility

AST migrations preserve semantic information.

IR compatibility

IR changes preserve required semantics.

Serialization compatibility

Serialized data can evolve according to declared compatibility policies.

Backend compatibility

The same semantic program can target different implementations.

---

69. Deprecation

Deprecated data syntax must be registered through the repository's compatibility system.

A deprecated construct requires:

- reason;
- version introduced;
- version deprecated;
- replacement;
- migration guidance;
- diagnostic behavior;
- removal policy.

Deprecated syntax must not silently disappear.

---

70. Diagnostics

Data diagnostics must include source locations.

Potential diagnostics include:

invalid data declaration
invalid schema member
duplicate field
unknown field
invalid type
incompatible schema
invalid transformation
invalid query
invalid stream operation
invalid serialization
invalid partition specification
invalid resource requirement
invalid capability requirement
invalid constraint
invalid provenance
invalid data contract

The parser should report syntax errors.

Semantic analysis should report semantic errors.

Resource analysis should report resource incompatibilities.

Backend errors must not be disguised as grammar errors.

---

71. Determinism

Parsing must be deterministic.

The same source must produce the same:

tokens
parse structure
AST
diagnostic classification

independent of:

CPU
GPU
FPGA
QPU
node count
memory capacity
database
cloud provider
runtime
target

Hardware discovery MUST NOT participate in parsing.

---

72. Parser Purity

Data grammar files MUST contain no target-language runtime actions.

They MUST NOT perform:

- filesystem access;
- database access;
- network access;
- hardware discovery;
- environment inspection;
- cloud calls;
- runtime execution.

The ".g4" files must remain declarative grammar specifications.

No embedded "unsafe" Rust is permitted.

The Rust integration must remain compatible with:

Rust 1.97
Rust 1.97.1
Rust 2021

---

73. Security

The grammar must not contain executable backend logic.

This prevents source files from causing parser-time:

filesystem operations
network operations
hardware operations
database operations
runtime execution

Security-sensitive behavior belongs downstream under explicit capability and security policies.

---

74. Privacy

Data syntax may express privacy requirements.

Examples conceptually include:

requires capability("protected.data")
requires capability("private.computation")

The grammar does not implement:

encryption
anonymization
differential privacy
secure enclaves
secure multiparty computation
zero-knowledge protocols

Those belong to security/interoperability/runtime implementations.

---

75. Resource Scalability

No grammar-level limit may exist for:

records
fields
collections
datasets
streams
queries
transformations
pipeline stages
partitions
replicas
nodes
data sources
data sinks
schema members
logical data dependencies

ANTLR/parser implementation limits are implementation constraints, not language semantics.

If an implementation has a practical parser limitation, that limitation must be documented separately and must not be represented as a language rule.

---

76. Deep Structures

The grammar must permit nested structures wherever semantics allow.

Examples include:

collection of records
record containing collections
dataset containing structured records
stream of collections
nested schemas
nested transformations
nested queries
pipelines containing pipelines where semantically supported

No artificial nesting constant should be introduced.

---

77. Large Programs

The data grammar must not assume a fixed number of:

data declarations
queries
pipelines
stages
transformations
schemas
records
sources
sinks
contracts

Use grammar repetition rather than finite enumeration wherever the semantics are naturally unbounded.

---

78. Source Independence

Data semantics MUST NOT depend on:

file size
machine size
memory layout
database type
storage implementation
network implementation
device count
processor count

The source describes logical computation.

---

79. Cross-Domain Integration Matrix

The data package must integrate with:

Domain| Data role
Classical| Input/output and computation data
Quantum| Measurement/control/parameter data
Hybrid| Classical/quantum boundary data
AI| Training/inference/model data
HDL| Hardware input/output data
Hardware| Resource and data-movement intent
Distributed| Partitioning, replication, consistency
Networking| Logical sources/sinks
Security| Privacy/protection requirements
Memory| Logical storage/lifetime semantics
Compile| Optimization and target realization
Execution| Runtime data lifecycle
Interoperability| External formats and APIs
Dialects| Provider/framework-specific extensions

No domain may create a competing data language.

---

80. Data + Quantum Example

Conceptually:

dataset parameters
        ↓
classical preprocessing
        ↓
quantum computation
        ↓
measurement
        ↓
measurement dataset
        ↓
aggregation
        ↓
optimization

The data grammar owns the data side.

The quantum grammar owns the quantum side.

The hybrid semantic layer owns the boundary.

The canonical "quantum::ir" remains the quantum IR boundary.

---

81. Data + AI Example

Conceptually:

dataset
    ↓
validation
    ↓
preprocessing
    ↓
feature transformation
    ↓
training
    ↓
evaluation
    ↓
model
    ↓
inference
    ↓
result dataset

Data syntax must remain usable regardless of the eventual AI implementation.

---

82. Data + HDL Example

Conceptually:

logical input data
       ↓
hardware computation
       ↓
logical output data

The data grammar describes the logical data.

The HDL grammar describes the hardware computation.

The compiler performs the integration.

---

83. Data + Distributed Example

Conceptually:

logical dataset
       ↓
partition intent
       ↓
parallel transformation
       ↓
aggregation
       ↓
logical result

The runtime may realize this on:

one machine
many machines
CPU
GPU
FPGA
accelerator
HPC
cloud
future distributed systems

without changing the logical data program.

---

84. Data + Hardware Capability Example

A portable program may express:

requires capability("distributed.data")
requires capability("accelerated.data")
requires memory >= required_memory

The source does not need to say:

use GPU 0
use node 4
use FPGA 2
use memory bank 7

The latter are implementation decisions.

---

85. Data + Compilation

Compilation may perform:

data semantics
    ↓
fusion
    ↓
vectorization
    ↓
parallelization
    ↓
partitioning
    ↓
device selection
    ↓
scheduling

The data grammar remains unchanged.

---

86. Data + Runtime

Runtime may dynamically discover:

resources
capabilities
availability
performance
storage
network
devices

Such discovery affects realization.

It MUST NOT alter source semantics.

---

87. Data + Resilience

The data layer may expose logical resilience requirements.

For example:

requires reliability(...)
requires capability("checkpointing")
requires capability("recoverable.data")

Actual resilience states and execution outcomes remain owned by the execution/resilience architecture.

Where the repository uses the established resilience vocabulary:

Unknown
Healthy
Degraded
Unstable
Unavailable
Recovering
Quarantined
Retired

and outcomes:

ACCEPT
DEGRADED_ACCEPT
RETRY
RECOVER
ESCALATE
REJECT

the data layer may consume those semantics through established interfaces but MUST NOT redefine them.

---

88. Data + ZQN

ZQN/fault/noise semantics remain outside the data grammar.

Data metadata may participate in:

experiment provenance
measurement data
fault records
execution metadata

but "grammar/data/" MUST NOT become the ZQN language.

---

89. Data + QEC

Data grammar may represent:

experiment data
syndrome results
measurement results
logical result data

but MUST NOT implement QEC.

QEC remains downstream.

---

90. Data Mining + Scaling

Data mining must be able to operate over:

one record
small dataset
large dataset
distributed dataset
stream
large stream
HPC dataset
accelerated dataset
future data system

without grammar-level changes.

The mining algorithm may require resources.

The language must not impose artificial limits.

---

91. Data Contracts

A data contract may specify:

input schema
output schema
validity constraints
quality requirements
privacy requirements
resource requirements
compatibility requirements
provenance requirements

The contract describes semantics.

It does not implement the data processing engine.

---

92. Data Quality

Quality semantics may include:

validity
completeness
consistency
accuracy
freshness
uniqueness
application-defined properties

Actual measurement and enforcement belong to semantic/runtime systems.

---

93. Data Validation

Validation syntax expresses logical validation intent.

The validator may eventually run:

compile time
deployment time
runtime
stream processing
distributed execution

depending on semantic requirements.

The grammar itself performs no validation beyond syntax.

---

94. Data Provenance

Provenance should be representable independently of storage backend.

A provenance record may conceptually identify:

source
transformation
version
dependency
derivation
execution
result

The runtime/compiler determines how provenance is stored.

---

95. Data Movement

Logical data movement may be expressed without specifying physical transport.

The grammar must distinguish:

logical movement

from:

physical network routing
physical memory movement
DMA
device transfer
PCIe
specific transport protocol

Physical movement belongs downstream.

---

96. Caching

Caching is an optimization/lifecycle concern.

The grammar may express cache intent.

It MUST NOT specify:

cache size
cache level
cache device
cache address
cache line width

unless such information is explicitly part of a target-specific dialect.

---

97. Persistence

Persistence intent is semantic.

The implementation may use:

local storage
database
object storage
distributed storage
embedded storage
future persistent medium

without changing the source semantics.

---

98. Provider Independence

Provider-specific syntax MUST NOT enter canonical data syntax merely because a provider is popular.

Examples of provider-specific concerns include:

provider-specific database commands
cloud-specific storage APIs
vendor-specific streaming APIs
vendor-specific accelerator APIs

These belong under:

grammar/dialects/

or:

grammar/interoperability/

with explicit compatibility status.

---

99. Dialect Rules

A data dialect MUST declare:

name
version
owner
syntax extensions
semantic extensions
AST mapping
IR mapping
compatibility
feature gates
portability classification

A dialect MUST NOT silently change canonical Zamani semantics.

---

100. Interoperability

Interoperability formats may include:

SQL
CSV
JSON
binary formats
external data APIs
database interfaces
stream interfaces
foreign-language data structures

These are interoperability concerns.

They are not competing canonical data models.

---

101. Grammar Composition

The canonical architecture is:

grammar/Zamani.g4
        ↓
canonical parser composition
        ↓
dataStmt
        ↓
specialized data entry points

The root grammar MUST remain the only canonical language composition root.

There must not be:

grammar/Zamani.g4
grammar/antlr/Zamani.g4

both claiming authority.

---

102. No Circular Dependency

The dependency direction must remain:

lexer
   ↓
parser grammar
   ↓
AST
   ↓
semantic analysis
   ↓
IR
   ↓
compiler
   ↓
runtime

Never:

runtime
   ↓
grammar
   ↓
runtime

The grammar must remain independent of runtime execution.

---

103. No Grammar-to-Hardware Dependency

The data grammar must not import or inspect:

CPU information
GPU information
FPGA information
QPU information
memory information
network topology
storage topology

Hardware discovery is downstream.

---

104. Independent Completion Contract

This README establishes the contract required for each data grammar file to be completed independently.

Every ".g4" file MUST define, either in its own header or through this package contract:

Purpose
Status
Owns
Does Not Own
Public Entry Rules
Token Dependencies
Expression Dependencies
Type Dependencies
AST Contract
Semantic Contract
IR Contract
Compiler Integration
Runtime Integration
Cross-Domain Integration
Diagnostics
Security
Performance
Positive Tests
Negative Tests
Boundary Tests
Scalability Tests
Compatibility Tests
Determinism Tests
Hard-Coding Audit
Completion Criteria

A file is complete when those contracts are known before dependent files are modified.

---

105. "queries.g4" Completion Contract

Before marking "queries.g4" complete, verify:

✓ query ownership defined
✓ public entry rule defined
✓ canonical expression integration defined
✓ canonical type integration defined
✓ AST mapping defined
✓ semantic query model defined
✓ canonical IR path defined
✓ database independence defined
✓ ordering semantics defined
✓ null/absence semantics defined
✓ aggregation semantics defined
✓ join semantics defined
✓ window semantics defined
✓ distributed lowering boundary defined
✓ diagnostics defined
✓ tests defined
✓ scalability audit passed
✓ hard-coding audit passed

---

106. "datasets.g4" Completion Contract

Verify:

✓ dataset ownership
✓ source integration
✓ schema integration
✓ feature/label/target integration where applicable
✓ partition semantics
✓ transformations
✓ versioning
✓ lineage
✓ validation
✓ materialization
✓ caching
✓ resource requirements
✓ AST mapping
✓ semantic mapping
✓ IR mapping
✓ diagnostics
✓ scalability
✓ compatibility

---

107. "transformations.g4" Completion Contract

Verify:

✓ generic operation model
✓ arbitrary operation composition
✓ canonical expression integration
✓ generic arguments
✓ options
✓ requirements
✓ preferences
✓ hints
✓ properties
✓ metadata
✓ provenance
✓ contracts
✓ input/output semantics
✓ AST mapping
✓ semantic mapping
✓ canonical IR mapping
✓ deterministic parsing
✓ no fixed operation-count limits
✓ no provider dependency

---

108. "tables.g4" Completion Contract

Verify:

✓ table declaration
✓ column declaration
✓ logical keys
✓ logical constraints
✓ logical indexes
✓ ordering
✓ partitioning
✓ clustering
✓ distribution
✓ source/type integration
✓ AST mapping
✓ semantic mapping
✓ database independence
✓ scalability
✓ diagnostics
✓ compatibility

---

109. "collections.g4" Completion Contract

Verify:

✓ collection declaration
✓ generic types
✓ collection expressions
✓ indexing
✓ slicing
✓ member access
✓ requirements
✓ constraints
✓ capabilities
✓ preferences
✓ placement intent
✓ performance intent
✓ reliability
✓ portability
✓ provenance
✓ validation
✓ AST mapping
✓ semantic mapping
✓ no fixed element limits

---

110. "streams.g4" Completion Contract

Verify:

✓ stream declaration
✓ source
✓ sink
✓ configuration
✓ keys
✓ partitioning
✓ ordering
✓ watermarks
✓ windows
✓ policies
✓ properties
✓ annotations
✓ AST mapping
✓ semantic mapping
✓ streaming capability model
✓ distributed compatibility
✓ no fixed stream size
✓ no fixed event count
✓ no fixed partition count

---

111. "serialization.g4" Completion Contract

Verify:

✓ serialization statement
✓ deserialization statement
✓ serialization expression
✓ target type
✓ format reference
✓ schema reference
✓ version
✓ compatibility
✓ integrity
✓ compression intent
✓ encoding intent
✓ byte order intent where semantically required
✓ AST mapping
✓ semantic mapping
✓ interoperability boundary
✓ no serializer implementation in grammar

---

112. Data Mining File Completion Contract

For:

grammar/data/mining.g4

the independent contract must include:

✓ mining ownership
✓ source integration
✓ dataset integration
✓ stream integration
✓ feature integration
✓ target/label integration
✓ method abstraction
✓ objective abstraction
✓ metric abstraction
✓ preprocessing
✓ validation
✓ evaluation
✓ parameters
✓ constraints
✓ requirements
✓ capabilities
✓ preferences
✓ hints
✓ provenance
✓ explainability intent
✓ model integration
✓ distributed execution intent
✓ incremental/online intent
✓ AST mapping
✓ semantic mapping
✓ canonical IR mapping
✓ no fixed algorithm enumeration
✓ no fixed dataset size
✓ no fixed feature count
✓ no fixed model size
✓ no fixed worker count
✓ no framework dependency

---

113. Mining Algorithm Extensibility

The grammar MUST NOT become a dictionary of every known mining algorithm.

Avoid a permanently growing structure such as:

miningMethod
    : decisionTree
    | randomForest
    | svm
    | neuralNetwork
    | ...

Instead, the language should permit generic semantic method specifications.

This allows future algorithms without grammar redesign.

---

114. Generic Data Operations

Where an operation is generally applicable, the universal operation system should own it.

Examples:

map
filter
reduce
sort
group
join
aggregate

Data grammar should specialize only the semantic context.

This avoids duplication between:

data
AI
classical
distributed
streaming
mining
tensor

grammars.

---

115. Mathematical Data Operations

Mathematical operations already present elsewhere in the repository must not be duplicated unnecessarily inside data grammar.

Operations such as:

matrix operations
tensor operations
statistics
optimization
linear algebra
signal processing

should use the canonical mathematical/type/operation infrastructure.

Data grammar consumes them as expressions or domain operations.

---

116. Error Recovery

The parser should recover from syntax errors without executing arbitrary semantic behavior.

Diagnostics should retain source spans.

The data grammar should avoid unnecessary ambiguity and overlapping alternatives.

Semantic errors must be separated from parser errors.

---

117. Source Spans

Every data construct that reaches the AST must retain sufficient source-location information to support:

diagnostics
IDE navigation
formatting
refactoring
provenance
compiler diagnostics
migration tooling

The grammar itself should not manufacture source locations.

The lexer/parser infrastructure owns them.

---

118. Tooling

The same grammar authority should support:

- syntax highlighting;
- autocomplete;
- formatting;
- diagnostics;
- navigation;
- refactoring;
- documentation;
- semantic inspection;
- language-server functionality.

Tooling MUST NOT maintain an undocumented second data grammar.

---

119. Documentation Authority

The authority hierarchy is:

grammar/DESIGN.md
        ↓
grammar/specification/
        ↓
grammar/spec/
        ↓
grammar/Zamani.g4 + canonical grammar components
        ↓
lexer/parser implementation
        ↓
AST
        ↓
semantic implementation
        ↓
IR
        ↓
compiler/runtime

"grammar/Zamani-Grammar.md" remains historical/extended design material.

"grammar/grammar.md" remains implementation-conformance documentation.

Neither may silently introduce new canonical syntax.

---

120. Feature Lifecycle

A new data feature follows:

Zamani-Grammar.md
        ↓
feature proposal
        ↓
semantic design
        ↓
AST contract
        ↓
canonical grammar
        ↓
lexer compatibility
        ↓
semantic implementation
        ↓
IR mapping
        ↓
compiler integration
        ↓
runtime integration
        ↓
tests
        ↓
stable

A feature is not stable merely because it parses.

---

121. Testing Structure

The global test organization should contain data tests equivalent to:

grammar/tests/
└── data/
    ├── positive/
    ├── negative/
    ├── boundary/
    ├── scalability/
    ├── determinism/
    ├── compatibility/
    ├── diagnostics/
    ├── roundtrip/
    └── cross-domain/

The exact repository test location may follow the existing global convention.

No duplicate test authority should be created merely for the data package.

---

122. Positive Tests

Positive tests must cover:

- minimal valid data constructs;
- schemas;
- records;
- collections;
- datasets;
- streams;
- tables;
- queries;
- transformations;
- pipelines;
- serialization;
- provenance;
- lineage;
- partitioning;
- replication;
- distributed intent;
- resource intent;
- AI/data integration;
- quantum/data integration;
- HDL/data integration;
- networking/data integration;
- security/data integration.

---

123. Negative Tests

Negative tests must verify rejection of:

- malformed declarations;
- malformed schema fields;
- invalid types;
- invalid transformations;
- malformed queries;
- invalid joins;
- malformed windows;
- malformed streams;
- malformed serialization;
- malformed requirements;
- malformed constraints;
- malformed resource expressions;
- accidental backend-specific syntax where prohibited.

---

124. Boundary Tests

Boundary tests should exercise:

one value
one field
many fields
deeply nested data
long identifiers
large qualified names
large schemas
large transformation chains
large query structures
large pipeline descriptions
large logical collections
large stream descriptions

The test suite must not establish an artificial language maximum.

---

125. Scalability Tests

Scalability tests must verify that the grammar does not impose limits on:

field count
record count
collection count
dataset count
stream count
pipeline stage count
transformation count
query clause count
partition count
replica count
node count
tensor dimensions
logical data dependencies

Generated tests should be used where appropriate.

---

126. Determinism Tests

Given identical source:

source A

the parser must produce the same:

token stream
parse structure
AST
diagnostic classification

independent of:

machine
hardware
resource availability
backend
runtime
provider

---

127. Round-Trip Tests

Where formatting exists:

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

Program meaning must not.

---

128. Cross-Domain Tests

At minimum, test:

classical + data
AI + data
quantum + data
classical + quantum + data
AI + quantum + data
HDL + data
hardware + data
distributed + data
networking + data
security + data
resources + data
execution + data

Also test a heterogeneous program combining:

classical
quantum
AI
data
distributed
hardware

The domains must coexist without one becoming an accidental replacement for the others.

---

129. POCO-REAF Tests

The same logical data program should be tested against conceptual target profiles differing in:

CPU count
GPU count
FPGA availability
QPU availability
node count
memory capacity
storage capacity
network topology
accelerator availability
deployment environment

The parser and semantic source contract must remain unchanged.

Target incompatibility must be reported downstream.

---

130. Hard-Coding Audit

Every release must inspect the entire data package for:

MAX_*
fixed counts
fixed widths
fixed capacities
fixed devices
fixed nodes
fixed partitions
fixed replicas
fixed storage
fixed memory
fixed topology
fixed addresses
fixed providers

Every finding must be classified as:

1. Program semantic value
2. Explicit user requirement
3. Explicit user constraint
4. Target constraint
5. Runtime constraint
6. Implementation limitation
7. Test fixture limitation
8. Accidental language hard-coding

Only the final category must necessarily be removed.

---

131. Examples of Prohibited Hard-Coding

Do not introduce:

MAX_ROWS = 1_000_000
MAX_FIELDS = 256
MAX_STREAMS = 1024
MAX_PARTITIONS = 128
MAX_REPLICAS = 16
MAX_NODES = 1024
MAX_TENSOR_RANK = 8
MAX_DATASET_SIZE = ...

Do not encode hidden limits through grammar alternatives.

For example, avoid structures equivalent to:

field1 field2 field3 ... field32

when the semantic model permits arbitrary fields.

---

132. Explicit Program Constraints Remain Valid

The prohibition does not prevent developers from writing explicit requirements.

For example:

requires memory >= required_memory
requires replicas >= desired_replicas
requires partitions >= required_partitions

These are program semantics.

The distinction is:

program requirement
        ≠
language maximum

---

133. Performance

Grammar performance matters, but optimization MUST NOT weaken semantics.

The implementation should avoid unnecessary:

- ambiguity;
- pathological recursion;
- duplicate alternatives;
- unnecessary lookahead;
- parser actions;
- backend calls.

Grammar performance optimization must preserve the same accepted language.

---

134. Parser Resource Limits

ANTLR, Rust, operating systems, and machines may impose implementation limits.

These are not language semantics.

If a parser implementation cannot parse a particular extremely large source because of an implementation resource exhaustion, that must be treated as an implementation/resource issue rather than silently defining a Zamani language maximum.

---

135. Security of Parser Resources

The parser implementation should defend against pathological input where appropriate through implementation-level resource management.

However, such defenses must not redefine the language's semantic model.

A runtime/parser budget is distinct from:

MAX_DATASET_SIZE
MAX_RECORDS
MAX_FIELDS

as language rules.

---

136. Safe Rust Requirement

The grammar files contain no Rust implementation code.

All Zamani-owned Rust integration must remain compatible with:

Rust 1.97
Rust 1.97.1
Rust 2021

and must not use:

unsafe

Production grammar integration MUST NOT introduce unsafe parser actions.

---

137. Lexer Integration

The canonical lexer remains the source of token meaning.

The inspected "src/lexer.rs" already contains the repository's token system and keyword map.

The data grammar MUST NOT create a second lexer.

The existing lexer/parser discrepancies must be resolved through the repository's canonical lexical contract rather than by making data grammar silently invent token semantics.

---

138. Token Naming Compatibility

Where existing parser grammars use:

IDENTIFIER
expression
typeExpr

while others use:

identifier
typeExpression
qualifiedName

the integration architecture must provide a canonical compatibility mapping.

Do not create duplicate lexical concepts simply because two grammar files currently use different names.

---

139. Canonical Shared Vocabulary

The final canonical data integration should converge conceptually on:

canonical identifier
canonical qualified name
canonical expression
canonical type expression
canonical annotation
canonical block
canonical argument list
canonical source span

Existing specialized grammars should be adapted at explicit integration boundaries rather than creating another global vocabulary.

---

140. No Unnecessary Renaming

Existing filenames remain authoritative unless there is a concrete technical blocker.

In particular, do not rename merely for aesthetics:

data.g4
queries.g4
datasets.g4
transformations.g4
tables.g4
collections.g4
streams.g4
serialization.g4
README.md

The problem is integration consistency, not filename spelling.

---

141. No Parallel Data Grammar

Do not create:

data2.g4
universal-data.g4
advanced-data.g4
new-data.g4

as competing authorities.

A new grammar file is justified only when it owns a distinct semantic responsibility.

---

142. No Parallel Data IR

Do not create:

DataIR
UniversalDataIR
MiningIR
StreamIR
DatasetIR

as undocumented competing compiler IRs.

The canonical semantic/IR architecture remains the repository's single integration boundary.

---

143. No Backend Leakage

Data grammar must not contain:

CUDA syntax
ROCm syntax
vendor GPU APIs
vendor FPGA primitives
database-specific execution plans
cloud provider APIs
physical QPU instructions
specific storage-engine internals

unless explicitly isolated under an interoperability/dialect boundary.

---

144. Data and Future Hardware

A future machine may provide:

new accelerator
new memory model
new storage technology
new network
new processor
new quantum architecture
new heterogeneous architecture

The same Zamani data semantics should remain representable.

This is a core POCO-REAF invariant.

---

145. Data and Future Algorithms

The grammar must not need to be rewritten every time a new algorithm is invented.

Generic constructs should allow:

operation
method
strategy
objective
metric
transform

to remain extensible.

New algorithms can then be introduced through:

libraries
intrinsics
semantic capabilities
dialects
interoperability

where appropriate.

---

146. Data and Future Formats

The language must not need a new core keyword for every future data format.

A generic serialization/interoperability model should allow future formats to be integrated without destabilizing the canonical data language.

---

147. Data and Future Storage

The data grammar must remain usable if future systems replace:

RAM
disks
databases
object stores
distributed filesystems
current network storage

with new storage technologies.

Storage realization belongs downstream.

---

148. Data and Future Distribution Models

The same logical data model must be able to scale from:

single process

to:

multicore
GPU
FPGA
distributed cluster
HPC
cloud
edge
future distributed architecture

without requiring source-level hardware rewrites.

---

149. Compatibility With "grammar/DESIGN.md"

This package MUST obey the architecture defined by:

grammar/DESIGN.md

particularly:

- one language;
- deterministic parsing;
- no artificial hardware limits;
- separation of syntax and semantics;
- resource/capability separation;
- canonical IR boundaries;
- quantum "quantum::ir";
- POCO-REAF;
- domain-neutral frontend AST;
- downstream hardware realization.

---

150. Compatibility With "grammar/README.md"

The package must remain subordinate to the global grammar authority.

"grammar/README.md" defines the repository-wide navigation and authority model.

This file specializes that model for data.

It must not redefine the global language architecture.

---

151. Compatibility With "grammar/grammar.md"

"grammar/grammar.md" describes implementation conformance.

The data package must therefore be capable of being classified as:

SPECIFIED
IMPLEMENTED
PARTIALLY IMPLEMENTED
PLANNED
DEPRECATED

No data feature should be described as implemented merely because a grammar rule exists.

---

152. Compatibility With "Zamani-Grammar.md"

"Zamani-Grammar.md" may contain broader or historical data concepts.

Those concepts are not automatically canonical.

Promotion must follow:

Zamani-Grammar.md
    ↓
proposal
    ↓
semantic design
    ↓
AST contract
    ↓
canonical grammar
    ↓
implementation
    ↓
IR
    ↓
tests
    ↓
stable

---

153. Compatibility With "src/lexer.rs"

The data grammar must remain compatible with the canonical lexer vocabulary.

Any missing data keyword must be handled through the canonical lexical architecture.

A data grammar must not silently assume a token that the actual lexer cannot produce.

---

154. Compatibility With "src/parser.rs"

The repository currently contains a handwritten parser in addition to the ANTLR grammar architecture.

Therefore conformance must distinguish:

ANTLR grammar acceptance

from:

Rust parser acceptance

A feature is not fully implemented merely because one parser recognizes it.

The compatibility system must report divergence.

---

155. First-Divergence Rule

When a data feature fails integration:

specification
    ↓
lexer
    ↓
grammar
    ↓
parser
    ↓
AST
    ↓
semantics
    ↓
IR

identify the first boundary where the contracts diverge.

Do not compensate downstream for an upstream defect.

For example:

grammar accepts X
lexer rejects X

is primarily a lexical integration defect.

Do not make semantic analysis guess what the lexer discarded.

---

156. No Silent Semantic Loss

A parser must never accept:

requirement
constraint
capability
preference
provenance
ordering
schema metadata

and then silently discard it before semantic analysis.

Every semantically meaningful field must have a documented downstream destination.

---

157. Completion Definition

"grammar/data/README.md" is complete as the package contract when:

✓ ownership is defined
✓ non-ownership is defined
✓ current specialized files are identified
✓ public integration rules are identified
✓ shared-rule dependencies are defined
✓ AST contract is defined
✓ semantic contract is defined
✓ IR contract is defined
✓ compiler integration is defined
✓ runtime integration is defined
✓ cross-domain integration is defined
✓ POCO-REAF is defined
✓ scalability policy is defined
✓ hard-coding policy is defined
✓ compatibility policy is defined
✓ diagnostics are defined
✓ security boundaries are defined
✓ test categories are defined
✓ independent completion contract is defined
✓ no filename renames are required
✓ no second grammar authority is created
✓ no second data type system is created
✓ no second data IR is created
✓ quantum::ir remains canonical
✓ Rust 1.97/1.97.1 is established
✓ unsafe Rust is prohibited

---

158. Package Production-Readiness Gate

The package itself is production-ready only when:

Specification
      +
Lexical conformance
      +
Grammar conformance
      +
Parser conformance
      +
AST conformance
      +
Semantic conformance
      +
Canonical IR integration
      +
Compiler integration
      +
Runtime integration
      +
Diagnostics
      +
Positive tests
      +
Negative tests
      +
Boundary tests
      +
Scalability tests
      +
Determinism tests
      +
Compatibility tests
      +
Hard-coding audit
      +
Cross-domain tests

all pass for the applicable implementation profile.

---

159. Final Architecture

The complete data architecture is:

                       ZAMANI SOURCE
                              │
                              ▼
                     Canonical Lexer
                              │
                              ▼
                     Canonical Parser
                              │
                              ▼
                       dataStmt
                              │
             ┌────────────────┼────────────────┐
             │                │                │
          Schemas          Datasets          Streams
             │                │                │
          Records          Queries        Transformations
             │                │                │
       Collections          Tables        Serialization
             │                │                │
             └────────────────┼────────────────┘
                              │
                              ▼
                     Frontend AST
                              │
                              ▼
                   Semantic Data Model
                              │
             ┌────────────────┼────────────────┐
             │                │                │
          Types           Resources        Capabilities
             │                │                │
             └────────────────┼────────────────┘
                              │
                              ▼
                     Canonical IR
                              │
          ┌───────────────────┼───────────────────┐
          │                   │                   │
      Classical            Quantum              HDL
          │                   │                   │
          │              quantum::ir             │
          │                   │                   │
          └───────────────────┼───────────────────┘
                              │
                              ▼
                         Optimization
                              │
                    ┌─────────┼─────────┐
                    │         │         │
                 Parallel  Distributed  Accelerated
                    │         │         │
                    └─────────┼─────────┘
                              │
                              ▼
                         Scheduling
                              │
                              ▼
                    Resource / Capability
                              │
                              ▼
                       Target Lowering
                              │
             ┌────────────────┼────────────────┐
             │                │                │
            CPU              GPU             FPGA
             │                │                │
             ├────────────────┼────────────────┤
             │                │                │
            QPU          Distributed       Future
             │                │                │
             └────────────────┼────────────────┘
                              │
                              ▼
                           Runtime
                              │
                              ▼
                          Execution

---

160. Final POCO-REAF Invariant

The data layer MUST preserve this invariant:

Logical Data Program
        ↓
Stable Meaning
        ↓
Many Data Sizes
        ↓
Many Execution Scales
        ↓
Many Machines
        ↓
Many Storage Systems
        ↓
Many Accelerators
        ↓
Many Distributed Deployments
        ↓
Future Computing Architectures

without requiring the developer to rewrite the semantic program merely because the available resources changed.

---

161. Final Hard-Coding Invariant

The permanent rule is:

«A Zamani data grammar rule MUST describe data semantics, not the accidental capacity of the machine currently available.»

Therefore the grammar must never turn:

today's memory
today's CPU count
today's GPU count
today's node count
today's database
today's storage
today's accelerator
today's quantum device
today's tensor hardware

into tomorrow's language limits.

---

162. Final Integration Invariant

The permanent dependency direction is:

Zamani source
     ↓
lexer
     ↓
parser
     ↓
data grammar
     ↓
frontend AST
     ↓
semantic analysis
     ↓
resource/capability analysis
     ↓
canonical semantic representation
     ↓
canonical IR
     ↓
optimization
     ↓
routing / scheduling / resilience where applicable
     ↓
ZQN where applicable
     ↓
HAL
     ↓
target realization
     ↓
runtime

The data grammar MUST NOT reverse this dependency.

---

163. Final Quantum Invariant

Whenever data participates in quantum computation:

Data
 ↓
Hybrid semantics
 ↓
Quantum semantics
 ↓
quantum::ir

There must remain one canonical quantum IR boundary.

The data grammar must never become a second quantum compiler.

---

164. Final Safety Invariant

The production implementation must remain:

Rust 2021
Rust 1.97 / 1.97.1
safe Rust
no unsafe

Grammar files contain no executable backend actions.

Parser-time execution of external systems is prohibited.

---

165. Final Data-Layer Statement

"grammar/data/" is the portable data-computation language boundary of Zamani.

It is not:

a database engine
a SQL engine
a storage engine
a streaming engine
a distributed engine
a networking stack
a hardware description language
a quantum IR
an optimizer
a scheduler
a runtime

It is the source-language layer through which Zamani describes:

data
+
structure
+
relationships
+
transformations
+
queries
+
streams
+
pipelines
+
provenance
+
validation
+
distribution intent
+
resource intent
+
capability requirements

in a target-independent way.

The governing architectural rule is:

«Describe the data and its computation once. Preserve its semantic meaning through the AST, semantic model, canonical IR, optimization, scheduling, resource analysis, hardware abstraction, and runtime. Let the compiler and runtime determine how that meaning is realized on the resources actually available.»

That is the data-layer foundation required for:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever

and for Zamani's goal of scaling from the smallest meaningful data computation to arbitrarily large executions permitted by program semantics and available resources.

End of "grammar/data/README.md".
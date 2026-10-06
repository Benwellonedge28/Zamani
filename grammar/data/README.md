# Zamani Data Grammar
 
Path: `grammar/data/README.md`
 
Repository: `Benwellonedge28/Zamani`
 
Language: Zamani
 
Grammar technology: ANTLR4
 
Rust implementation baseline: Rust 1.97.1 or later
 
Rust edition: Rust 2021
 
Rust safety requirement: Safe Rust only; production Rust code MUST NOT use `unsafe`
 
Primary portability objective: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)
 
Status: **Normative data-subsystem architecture and integration contract**
 
***
 
## 1. Purpose
 
The `grammar/data/` directory defines the data-domain grammar composition and integration architecture for Zamani.
 
The data subsystem must support data computation ranging from the smallest useful value or record to arbitrarily large computations, datasets, tensors, streams, graphs, knowledge bases, distributed data systems, scientific workloads, AI workloads, quantum/classical workloads, hardware observations, and future data-oriented computational domains.
 
The data subsystem is part of **one Zamani programming language**.
 
It is not a separate data programming language.
 
It MUST integrate with the same:
 
- lexical system;
- names and identifiers;
- expressions;
- statements;
- declarations;
- types;
- functions;
- modules;
- effects;
- capabilities;
- resources;
- contracts;
- policies;
- provenance;
- semantic model;
- canonical IR;
- compiler;
- runtime;
- interoperability system;
- compatibility system.

 
The directory MUST therefore provide a maintainable composition boundary while avoiding duplicated syntax and competing semantic representations.
 
***
 
# 2. Architectural Position
 
The data subsystem participates in the complete Zamani pipeline:
 
```text
Zamani source
     |
     v
Canonical lexer
     |
     v
Canonical parser
     |
     v
Domain-neutral AST
     |
     v
Structural validation
     |
     +-------------------+
     |                   |
     v                   v
Type analysis       Name/module resolution
     |
     v
Semantic analysis
     |
     +-------------------------------+
     |        |         |            |
     v        v         v            v
 Effects  Capabilities Resources  Contracts
     |        |         |            |
     +--------+---------+------------+
                    |
                    v
              Policy analysis
                    |
                    v
              Provenance
                    |
                    v
          Canonical semantic model
                    |
          +---------+---------+
          |                   |
          v                   v
     Classical/data       Hybrid/quantum
     realization          realization
          |                   |
          +---------+---------+
                    |
                    v
             Canonical IR
                    |
                    v
       Target-independent optimization
                    |
                    v
          Lowering / specialization
                    |
                    v
       Routing / placement / scheduling
                    |
                    v
              Target realization
                    |
       +------------+-------------+
       |            |             |
      CPU          GPU           FPGA
       |            |             |
      ASIC       accelerator      QPU
       |            |             |
       +------------+-------------+
                    |
          embedded / HPC /
       cluster / distributed /
             cloud / future
```
 
The data grammar only participates in the **source-language and parser portion** of this architecture.
 
It does not perform semantic analysis, resource discovery, scheduling, storage allocation, database execution, hardware selection, or runtime execution.
 
***
 
# 3. Core Architectural Principle
 
The data grammar expresses **portable computational intent**.
 
It must not encode a particular realization.
 
For example, data syntax may describe:
 
- a dataset;
- a transformation;
- a query;
- a stream;
- a schema;
- a collection;
- a graph;
- a knowledge relation;
- an uncertain value;
- provenance;
- a data requirement;
- a data capability;
- a data constraint;
- a data policy.

 
It must not require:
 
- a particular CPU;
- a particular GPU;
- a particular storage device;
- a particular database vendor;
- a particular cloud provider;
- a particular network;
- a particular accelerator;
- a particular QPU;
- a particular physical address;
- a particular memory bank;
- a fixed machine topology.

 
Target realization belongs downstream.
 
***
 
# 4. POCO-REAF Contract
 
The data subsystem is a participant in:
 
```text
Program Once
     |
Compile Once
     |
Run Everywhere
     |
Run Anywhere
     |
Forever
```
 
subject to:
 
- semantic validity;
- type correctness;
- effect requirements;
- capability availability;
- resource availability;
- contracts;
- policies;
- compatibility;
- target feasibility.

 
A data program should remain source-compatible when its realization changes from:
 
```text
tiny embedded system
       |
       v
single CPU
       |
       v
multicore CPU
       |
       v
GPU
       |
       v
accelerator
       |
       v
FPGA
       |
       v
distributed system
       |
       v
HPC
       |
       v
cloud
       |
       v
future target
```
 
The source language describes the computation.
 
The compiler and runtime determine the realization.
 
***
 
# 5. No Artificial Capacity Ceilings
 
The data grammar MUST NOT introduce universal capacity limits.
 
The following categories MUST remain open-ended:
 
- datasets;
- records;
- fields;
- columns;
- rows;
- collections;
- streams;
- schemas;
- graph nodes;
- graph edges;
- query terms;
- joins;
- transformations;
- pipeline stages;
- partitions;
- replicas;
- sources;
- sinks;
- tensor dimensions;
- tensor rank;
- evidence;
- provenance;
- uncertainty metadata;
- knowledge relations;
- agents;
- distributed participants;
- storage;
- memory;
- devices;
- accelerators;
- network resources.

 
The grammar MUST NOT introduce universal constants equivalent to:
 
```text
MAX_ROWS
MAX_COLUMNS
MAX_FIELDS
MAX_DATASETS
MAX_PARTITIONS
MAX_REPLICAS
MAX_STREAMS
MAX_PIPELINE_STAGES
MAX_NODES
MAX_MEMORY
MAX_STORAGE
MAX_DEVICES
MAX_TENSOR_RANK
MAX_QUBITS
MAX_GPUS
MAX_FPGAS
MAX_THREADS
```
 
or any renamed equivalent.
 
A programmer-written number is program data.
 
For example:
 
```text
1024
```
 
must never become a compiler-wide capacity limit merely because it appears in a data program.
 
Actual limits belong to:
 
- implementation resources;
- target capabilities;
- resource policies;
- compiler configuration;
- runtime state;
- deployment constraints;
- operating-system constraints;
- physical feasibility.

 
The absence of a language ceiling means **no artificial semantic ceiling**, not a claim of physically infinite hardware.
 
***
 
# 6. Single-Authority Rule
 
Every source-language concept must have one canonical syntax owner.
 
The data directory MUST NOT create duplicate syntax for concepts already owned elsewhere.
 
In particular:
 

|Concept|Canonical owner|
|---|---|
|General expressions|`grammar/expressions/`|
|General types|`grammar/types/`|
|Knowledge expressions|`grammar/expressions/knowledge.g4`|
|Uncertainty expressions|`grammar/expressions/uncertainty.g4`|
|Provenance semantics/syntax|canonical provenance subsystem|
|Requirements|`grammar/resources/` / core resource contracts|
|Capabilities|`grammar/resources/`|
|Contracts|`grammar/validation/`|
|Policies|`grammar/policies/`|
|Effects|`grammar/effects/`|
|AI learning|`grammar/ai/`|
|Quantum operations|`grammar/quantum/`|
|SQL|dialect/interoperability subsystem|
|JSON|interoperability/dialect subsystem|
|XML|interoperability/dialect subsystem|
 
The data directory provides **domain adapters and composition boundaries** where necessary.
 
***
 
# 7. Directory Ownership
 
`grammar/data/` owns data-domain source syntax and composition.
 
It may contain specialized grammars for:
 
- data declarations;
- datasets;
- queries;
- transformations;
- pipelines;
- tables;
- collections;
- streams;
- schemas;
- knowledge integration;
- uncertainty integration;
- provenance integration;
- serialization;
- persistence;
- mining;
- lineage;
- data movement;
- data partitioning;
- data distribution;
- data replication;
- data requirements;
- data constraints;
- data preferences;
- future data-domain features.

 
It does not own the implementation of those concepts.
 
***
 
# 8. What `grammar/data/data.g4` Owns
 
`grammar/data/data.g4` is the data-domain **orchestrator**.
 
It owns:
 
- `dataStmt`;
- data declaration dispatch;
- data statement dispatch;
- data expression boundaries;
- data-domain composition;
- compatibility wrappers;
- integration between data leaf grammars.

 
It MUST NOT duplicate specialized syntax.
 
The current architecture correctly establishes `Data` as a composition grammar:
 
```antlr
parser grammar Data;
```
 
with:
 
```antlr
options {
    tokenVocab = ZamaniLexer;
}
```
 
The universal parser owns the final program boundary and EOF handling.
 
`data.g4` MUST NOT become a second root grammar.
 
***
 
# 9. Data Leaf Grammar Rule
 
Each specialized data grammar should own one coherent feature boundary.
 
The preferred structure is:
 
```text
grammar/data/
    data.g4
        |
        +-- queries.g4
        +-- datasets.g4
        +-- transformations.g4
        +-- pipelines.g4
        +-- tables.g4
        +-- collections.g4
        +-- streams.g4
        +-- schemas.g4
        +-- knowledge.g4
        +-- uncertainty.g4
        +-- provenance.g4
        +-- serialization.g4
        +-- persistence.g4
        +-- mining.g4
        +-- originality.g4
        +-- ...
```
 
A leaf grammar:
 
1. has one clear owner;
2. imports only its actual grammar dependencies;
3. uses the canonical lexer;
4. exports documented public rules;
5. does not create a second AST;
6. does not create a second IR;
7. does not select hardware;
8. does not perform execution;
9. has explicit integration contracts;
10. has positive, negative, boundary and scalability tests.

 
***
 
# 10. Existing Data Feature Families
 
The current data architecture identifies the following major feature families:
 
### Core data
 
- data declarations;
- data assignments;
- data expressions;
- data references.

 
### Data structures
 
- records;
- collections;
- sequences;
- tables;
- datasets;
- streams.

 
### Data organization
 
- schemas;
- partitions;
- distribution;
- replication;
- materialization.

 
### Data computation
 
- queries;
- transformations;
- pipelines;
- mining.

 
### Data semantics
 
- knowledge;
- uncertainty;
- provenance;
- lineage;
- contracts;
- policies.

 
### Interoperability
 
- serialization;
- external formats;
- database/dialect integration;
- foreign data sources.

 
These features must share the universal Zamani semantic foundation.
 
***
 
# 11. Knowledge Integration
 
Knowledge source syntax is not owned by the data directory.
 
The canonical knowledge syntax is owned by:
 
```text
grammar/expressions/knowledge.g4
```
 
The data adapter is:
 
```text
grammar/data/knowledge.g4
```
 
Its role is to expose canonical knowledge syntax to the data domain.
 
The architecture is:
 
```text
expressions/knowledge.g4
          |
          v
knowledgeExpression
          |
          v
data/knowledge.g4
          |
          v
dataKnowledgeConstruct
          |
          v
data.g4
          |
          v
ZamaniParser
```
 
The data subsystem MUST NOT create another syntax for:
 
- assert;
- retract;
- query;
- lookup;
- update;
- knowledge terms;
- knowledge patterns.

 
Knowledge may represent:
 
- scientific facts;
- observations;
- relationships;
- configuration;
- hardware information;
- compiler information;
- resource information;
- provenance;
- model information;
- distributed state;
- application information;
- future computational information.

 
Knowledge is therefore not restricted to AI.
 
***
 
# 12. Uncertainty Integration
 
Uncertainty is a universal computational abstraction.
 
The canonical source-level uncertainty syntax belongs to:
 
```text
grammar/expressions/uncertainty.g4
```
 
The data directory MUST NOT duplicate that grammar.
 
The data adapter is:
 
```text
grammar/data/uncertainty.g4
```
 
Its purpose is to expose canonical uncertainty expressions to the data domain.
 
The architecture is:
 
```text
expressions/uncertainty.g4
             |
             v
uncertaintyExpression
             |
             v
data/uncertainty.g4
             |
             v
dataUncertaintyConstruct
             |
             v
dataUncertaintyStmt
             |
             v
data.g4
             |
             v
ZamaniParser
```
 
The canonical uncertainty expression supports the general conceptual forms:
 
```text
uncertain(value)
uncertainty(value)
```
 
and extensible metadata such as:
 
```text
uncertain(value, confidence: c)
uncertain(value, probability: p)
uncertainty(value, distribution: d)
uncertainty(value, evidence: e)
uncertainty(value, provenance: p)
```
 
The data adapter must not duplicate these rules.
 
***
 
# 13. Required Uncertainty Integration Correction
 
The current `data.g4` data-statement dispatcher contains:
 
```antlr
| dataUncertaintyStmt
```
 
and imports:
 
```antlr
ZamaniDataUncertainty
```
 
Therefore `grammar/data/uncertainty.g4` MUST expose the rule expected by the orchestrator.
 
The production integration must establish:
 
```text
dataUncertaintyConstruct
```
 
as the canonical data-domain adapter and:
 
```text
dataUncertaintyStmt
```
 
as the data-statement boundary consumed by `data.g4`.
 
The adapter MUST NOT reimplement `uncertaintyExpression`.
 
The intended relationship is:
 
```text
uncertaintyExpression
        |
        v
dataUncertaintyConstruct
        |
        v
dataUncertaintyStmt
        |
        v
dataStatement
```
 
This is a composition correction, not a second uncertainty language.
 
The exact semicolon policy must be owned by the data-statement boundary rather than duplicated inside `uncertaintyExpression`.
 
***
 
# 14. Data Uncertainty Semantics
 
An uncertainty expression in a data context may represent:
 
- uncertain measurements;
- statistical values;
- probabilistic records;
- confidence metadata;
- uncertain query results;
- uncertain model outputs;
- uncertain sensor data;
- simulation results;
- hardware observations;
- quantum measurement results;
- distributed observations;
- scientific estimates.

 
The grammar does not decide:
 
- probability representation;
- precision;
- statistical algorithm;
- distribution algorithm;
- sampling strategy;
- random-number generation;
- numerical representation;
- target hardware.

 
Those decisions belong downstream.
 
***
 
# 15. Quantum/Data Boundary
 
Data and quantum computing must remain composable without creating a second quantum data language.
 
For example, data may contain or derive from:
 
```text
quantum measurement
```
 
and quantum computation may consume:
 
```text
data values
datasets
parameters
models
uncertain values
knowledge
provenance
```
 
The semantic path remains:
 
```text
data expression
      |
      v
domain-neutral AST
      |
      v
semantic analysis
      |
      v
quantum semantic model
      |
      v
quantum::ir
      |
      v
optimization
      |
      v
decomposition
      |
      v
routing
      |
      v
scheduling
      |
      v
resilience / QEC
      |
      v
ZQN
      |
      v
HAL
```
 
`grammar/data/` MUST NOT create:
 
```text
DataQuantumIR
QuantumDataIR
DataQubitIR
```
 
or equivalent competing representations.
 
***
 
# 16. AI/Data Boundary
 
AI data features must use the same data model.
 
AI may consume:
 
- datasets;
- streams;
- tensors;
- schemas;
- knowledge;
- uncertain values;
- provenance;
- query results.

 
Data may consume results from:
 
- inference;
- learning;
- reasoning;
- adaptation;
- agents;
- models.

 
The boundary is semantic composition.
 
The data grammar must not create AI-specific replacements for ordinary data concepts.
 
***
 
# 17. Reasoning and Knowledge
 
Generic reasoning may operate on data:
 
```text
infer
deduce
reason
```
 
and knowledge operations may operate on data:
 
```text
assert
retract
query
```
 
The data subsystem must consume those universal semantic constructs rather than define independent data-only variants.
 
This allows the same computation to operate across:
 
- scientific data;
- hardware data;
- AI data;
- quantum observations;
- distributed data;
- security information;
- compiler metadata.

 
***
 
# 18. Learning and Adaptation
 
Learning and adaptation are not data-storage concepts.
 
They belong to their respective semantic owners.
 
However, data is a common input/output medium.
 
The integration is:
 
```text
data
 |
 +--> learning
 |
 +--> inference
 |
 +--> reasoning
 |
 +--> adaptation
 |
 +--> simulation
 |
 +--> quantum/hybrid computation
 |
 v
semantic result
```
 
The data grammar MUST NOT define a new syntax for every learning algorithm.
 
Algorithms remain open-world entities represented through:
 
- identifiers;
- qualified names;
- operations;
- libraries;
- capabilities;
- metadata;
- dialects.

 
***
 
# 19. Query Architecture
 
The data query subsystem owns query syntax.
 
Query syntax must not be duplicated in:
 
- knowledge grammar;
- uncertainty grammar;
- AI grammar;
- provenance grammar;
- data orchestrator.

 
External query languages such as SQL must remain dialect/interoperability features.
 
The architecture is:
 
```text
external query language
          |
          v
dialect parser
          |
          v
Zamani query semantic model
          |
          v
canonical representation
          |
          v
target realization
```
 
The universal Zamani language must not become a database-specific language.
 
***
 
# 20. SQL Integration
 
SQL belongs under dialect/interoperability infrastructure.
 
It must not become the universal data grammar.
 
Recommended boundary:
 
```text
grammar/dialects/sql/
```
 
or the repository's existing dialect/interoperability location.
 
SQL should lower into Zamani's generic data/query semantic model where appropriate.
 
The data subsystem must remain capable of expressing computations that have nothing to do with SQL.
 
***
 
# 21. JSON and XML Integration
 
JSON and XML are interoperability/data-representation formats.
 
They should not become universal Zamani syntax.
 
Their integration should use the existing:
 
```text
grammar/interoperability/
grammar/dialects/
```
 
architecture.
 
The data subsystem consumes the resulting semantic values.
 
***
 
# 22. Tensor Integration
 
Tensor semantics must remain open-ended.
 
The grammar MUST NOT define:
 
```text
MAX_TENSOR_RANK
MAX_TENSOR_DIMENSION
MAX_TENSOR_ELEMENTS
```
 
or equivalents.
 
Tensor shape may be:
 
- static;
- dynamic;
- symbolic;
- inferred;
- dependent on runtime data;
- dependent on resource availability;
- generated by computation.

 
Tensor syntax and type semantics belong to the canonical tensor/type/AI architecture.
 
The data subsystem should consume those semantics rather than create a competing tensor type system.
 
***
 
# 23. Graph Integration
 
Graph computation should support arbitrary graph structure without hard-coded graph limits.
 
Graph concepts may include:
 
- vertices;
- edges;
- labels;
- weights;
- attributes;
- paths;
- relationships;
- transformations;
- queries.

 
Graph size is determined by:
 
- program semantics;
- resource availability;
- target capability;
- execution policy.

 
It must not be constrained by grammar constants.
 
***
 
# 24. Streaming Integration
 
Streams must be able to represent:
 
- finite streams;
- long-running streams;
- distributed streams;
- sensor streams;
- network streams;
- quantum measurement streams;
- simulation streams;
- AI data streams.

 
The grammar must not impose a universal maximum stream length.
 
Backpressure, buffering, scheduling, transport and resource management belong downstream.
 
***
 
# 25. Dataset Integration
 
Datasets must support arbitrary logical composition.
 
The grammar must not assume:
 
- a fixed number of columns;
- a fixed number of rows;
- a fixed schema width;
- a fixed number of records;
- a fixed partition count;
- a fixed replica count.

 
Physical representation belongs to the implementation.
 
***
 
# 26. Schema Integration
 
Schemas describe logical structure.
 
They must remain independent from physical storage.
 
A schema must not encode:
 
- memory addresses;
- physical partitions;
- device IDs;
- storage vendor;
- CPU architecture;
- database engine.

 
Schema validation belongs to semantic/type validation.
 
***
 
# 27. Data Provenance
 
Data provenance must integrate with the universal provenance system.
 
It should be possible to preserve:
 
- source;
- derivation;
- transformation;
- generation;
- verification;
- evidence;
- decision;
- version;
- timestamp;
- policy context where applicable.

 
The data subsystem must not create a competing provenance semantic model.
 
The architecture should be:
 
```text
universal provenance
        |
        +---- data
        +---- AI
        +---- quantum
        +---- compilation
        +---- security
        +---- execution
```
 
Data lineage is a data-domain consumer of the universal provenance model.
 
***
 
# 28. Data Contracts
 
Data operations may participate in:
 
```text
requires
ensures
invariant
assume
guarantee
property
```
 
The data grammar should consume the universal validation/contract subsystem.
 
It must not create an independent contract language.
 
Examples of semantic intent include:
 
```text
requires schema_compatible(input);
ensures result_is_valid(result);
invariant data_consistency(state);
```
 
The exact contract syntax is owned by `grammar/validation/`.
 
***
 
# 29. Data Policies
 
Data may be constrained by policies governing:
 
- privacy;
- provenance;
- access;
- transformation;
- retention;
- network usage;
- storage;
- replication;
- security;
- external services;
- AI processing;
- quantum processing;
- distribution;
- simulation.

 
Policy syntax belongs to the universal policy system.
 
Data grammar consumes policy semantics.
 
***
 
# 30. Effects
 
Data operations may produce effects including:
 
- IO;
- network;
- mutation;
- randomness;
- foreign calls;
- native operations;
- distributed execution;
- measurement;
- simulation.

 
The data grammar must not infer effects merely from syntax.
 
Semantic resolution determines the actual effect set.
 
For example:
 
```text
read(data_source)
```
 
may have an IO or network effect depending on its resolved source.
 
The parser must not make that determination.
 
***
 
# 31. Capabilities
 
Data operations may require capabilities such as:
 
```text
data.read
data.write
data.query
data.stream
data.transform
data.distributed
data.storage
data.network
data.provenance
data.uncertainty
data.knowledge
```
 
These are capability semantics, not grammar-level hardware assumptions.
 
New capabilities should be open-world.
 
The language must not require a parser modification merely because a new data backend introduces a new capability.
 
***
 
# 32. Resource Integration
 
Data resource requirements must use the universal resource model.
 
Examples of semantic requirements include:
 
```text
requires capability("data.query");
requires capability("distributed.compute");
requires memory >= required_memory;
requires storage >= required_storage;
requires topology(required_topology);
```
 
The grammar must not encode:
 
```text
RAM = 64GB
GPU = device_0
nodes = 8
threads = 32
```
 
as universal realization rules.
 
A numeric resource requirement is a program-level requirement.
 
It is not a compiler-wide capacity constant.
 
***
 
# 33. Distributed Data
 
Distributed data must integrate with:
 
```text
grammar/distributed/
grammar/concurrency/
grammar/networking/
grammar/resources/
```
 
The data grammar must not assume a fixed number of:
 
- nodes;
- replicas;
- partitions;
- workers;
- processes;
- devices.

 
Distribution strategy is a compiler/runtime decision subject to semantic requirements and policy.
 
***
 
# 34. Hardware/Data Boundary
 
Data may represent:
 
- sensor data;
- device observations;
- hardware telemetry;
- timing information;
- simulation results;
- accelerator outputs;
- quantum measurements.

 
It must not encode a universal physical hardware model.
 
Hardware capabilities belong to:
 
```text
grammar/hardware/
```
 
and resource semantics.
 
***
 
# 35. HDL/Data Boundary
 
HDL may produce or consume data.
 
Examples include:
 
- simulation data;
- waveform data;
- verification results;
- hardware measurements;
- synthesis metadata.

 
The data grammar must not duplicate HDL syntax.
 
The boundary is:
 
```text
HDL intent
    |
    v
HDL semantic model
    |
    v
simulation / synthesis / realization
    |
    v
data results
```
 
***
 
# 36. Interoperability
 
Data interoperability must support external systems without contaminating the universal grammar.
 
Relevant boundaries include:
 
```text
grammar/interoperability/
grammar/dialects/
```
 
Potential formats and systems include:
 
- SQL;
- JSON;
- XML;
- foreign data systems;
- ABI/FFI;
- external services;
- domain-specific formats.

 
Each external format needs its own ownership and versioning contract.
 
***
 
# 37. AST Contract
 
Data grammar constructs must lower into the existing domain-neutral frontend AST.
 
The data subsystem MUST NOT create a competing universal AST such as:
 
```text
DataAST
DatabaseAST
GPUDataAST
QuantumDataAST
StorageAST
```
 
unless a narrowly scoped internal representation is explicitly approved by the AST architecture.
 
The AST must preserve the semantic information required by downstream analysis, including where applicable:
 
- source span;
- operation kind;
- names;
- qualified names;
- expressions;
- types;
- attributes;
- modifiers;
- ordering;
- metadata;
- requirements;
- constraints;
- provenance references.

 
The exact Rust representation belongs to the frontend AST owner.
 
***
 
# 38. Semantic Contract
 
Semantic analysis must determine:
 
- whether a data construct is meaningful;
- whether referenced names exist;
- whether types are compatible;
- whether schemas are compatible;
- whether transformations are valid;
- whether query expressions are valid;
- whether uncertainty metadata is valid;
- whether knowledge operations are valid;
- whether effects are permitted;
- whether capabilities are available;
- whether resources are satisfiable;
- whether contracts hold;
- whether policies authorize execution;
- whether provenance requirements are satisfied.

 
Parsing must not perform these checks.
 
***
 
# 39. IR Contract
 
The data grammar creates **no canonical IR**.
 
Data syntax must lower through the universal semantic model.
 
The semantic pipeline is:
 
```text
data source syntax
       |
       v
domain-neutral AST
       |
       v
semantic data model
       |
       v
canonical semantic representation
       |
       +----------------------+
       |                      |
       v                      v
classical/data          hybrid/quantum
       |                      |
       +----------+-----------+
                  |
                  v
          canonical IR/domain IR
                  |
                  v
        target-independent optimization
                  |
                  v
             realization
```
 
There must not be a competing:
 
```text
DataIR
DatabaseIR
StorageIR
AIDataIR
QuantumDataIR
```
 
unless explicitly established as a domain-specific lowering representation beneath the canonical semantic boundary.
 
***
 
# 40. Quantum IR Boundary
 
When data participates in quantum computation:
 
```text
data
 |
 v
semantic model
 |
 v
quantum semantics
 |
 v
quantum::ir
```
 
`quantum::ir` remains the canonical quantum representation.
 
Data grammar does not own quantum IR.
 
It does not own:
 
- qubit allocation;
- routing;
- decomposition;
- scheduling;
- QEC;
- calibration;
- physical qubit mapping;
- ZQN;
- HAL.

 
***
 
# 41. Determinism
 
Parsing must be deterministic.
 
The data parser must depend only on:
 
- source tokens;
- grammar version;
- parser configuration;
- explicitly defined language compatibility settings.

 
Parsing must not depend on:
 
- current hardware;
- current memory;
- current network;
- current database;
- current storage;
- current time;
- random state;
- target selection;
- scheduler state;
- external services.

 
Runtime data operations may be nondeterministic where their semantic contract permits it.
 
That is a runtime/semantic property, not parser behavior.
 
***
 
# 42. Security Boundary
 
Parsing is non-executing.
 
Data grammar files MUST NOT:
 
- open files;
- query databases;
- contact networks;
- execute SQL;
- invoke external services;
- invoke models;
- inspect hardware;
- allocate physical storage;
- execute foreign code;
- execute arbitrary reflection;
- access credentials.

 
Those operations occur only after the appropriate semantic/effect/capability/policy checks.
 
***
 
# 43. Metaprogramming
 
Data constructs may be inspected by:
 
```text
reflection
introspection
compile-time computation
syntax-tree generation
code generation
```
 
through the universal metaprogramming subsystem.
 
Data grammar must not execute metaprogramming itself.
 
Any reflective or generated data operation must remain subject to:
 
- effects;
- capabilities;
- resource requirements;
- contracts;
- policies;
- provenance.

 
***
 
# 44. Compatibility
 
Every data grammar feature must have a compatibility status.
 
Recommended statuses:
 
```text
STABLE
EXPERIMENTAL
PROPOSED
DEPRECATED
HISTORICAL
NOT_IMPLEMENTED
```
 
Compatibility belongs to:
 
```text
grammar/compatibility/
```
 
A deprecated data construct must have:
 
- replacement;
- compatibility behavior;
- diagnostic policy;
- migration guidance;
- version information.

 
A data feature must not silently change meaning between language versions.
 
***
 
# 45. Dependency Contract
 
Every data grammar file MUST document:
 
```text
DEPENDS_ON:
EXPORTS:
CONSUMED_BY:
AST_OWNER:
SEMANTIC_OWNER:
TYPE_OWNER:
EFFECT_OWNER:
CAPABILITY_OWNER:
RESOURCE_OWNER:
CONTRACT_OWNER:
POLICY_OWNER:
PROVENANCE_OWNER:
IR_OWNER:
TEST_OWNER:
SPEC_OWNER:
COMPATIBILITY_OWNER:
```
 
This is mandatory for independent-file development.
 
The objective is:
 
> A developer can complete one file without later reopening it merely because another feature file was implemented.

 
If a dependency changes incompatibly, the dependency contract—not undocumented assumptions—determines what must change.
 
***
 
# 46. Feature Contract Required in Every `.g4`
 
Every production data grammar file must contain a documented feature contract covering:
 
### Purpose
 
What the file implements.
 
### Owns
 
Exact grammar rules owned by the file.
 
### Does Not Own
 
Rules deliberately delegated elsewhere.
 
### Public Rules
 
Rules consumed by other grammars.
 
### Private Rules
 
Internal implementation rules.
 
### Lexer Dependencies
 
Canonical tokens consumed.
 
### Grammar Dependencies
 
Imported parser grammars.
 
### AST Contract
 
Expected AST representation.
 
### Semantic Contract
 
Meaning of the construct.
 
### Type Contract
 
Type-system interaction.
 
### Effect Contract
 
Effects produced or required.
 
### Capability Contract
 
Capabilities required.
 
### Resource Contract
 
Resource requirements.
 
### Contract Contract
 
Interaction with requirements, guarantees and properties.
 
### Policy Contract
 
Applicable policies.
 
### Provenance Contract
 
Required provenance.
 
### IR Contract
 
Canonical lowering destination.
 
### Quantum Boundary
 
Quantum integration, if applicable.
 
### HDL Boundary
 
HDL integration, if applicable.
 
### Backend Boundary
 
How downstream compilation consumes it.
 
### Diagnostics
 
Required diagnostics.
 
### Positive Tests
 
Valid syntax.
 
### Negative Tests
 
Invalid syntax.
 
### Boundary Tests
 
Cross-domain and edge cases.
 
### Scalability Tests
 
Large/general/open-ended cases.
 
### Compatibility
 
Versioning and migration.
 
### Integration
 
Upstream and downstream dependencies.
 
### Completion Criteria
 
Exact definition of done.
 
***
 
# 47. Required Data Grammar File Inventory
 
The production data directory should converge toward a structure such as:
 
```text
grammar/data/
├── README.md
├── data.g4
├── datasets.g4
├── queries.g4
├── transformations.g4
├── pipelines.g4
├── tables.g4
├── collections.g4
├── streams.g4
├── schemas.g4
├── knowledge.g4
├── uncertainty.g4
├── provenance.g4
├── serialization.g4
├── persistence.g4
├── mining.g4
├── originality.g4
├── lineage.g4
├── requirements.g4
├── constraints.g4
├── preferences.g4
├── movement.g4
├── materialization.g4
├── partitioning.g4
└── replication.g4
```
 
Only files that represent real canonical feature boundaries should be created.
 
Do not create files merely to increase the apparent size of the subsystem.
 
***
 
# 48. Existing-File Preservation
 
Existing major filenames should be preserved.
 
The productionization process must not rename:
 
```text
data.g4
knowledge.g4
provenance.g4
```
 
merely for aesthetic reasons.
 
If a new ownership model requires migration, it must be explicitly documented through compatibility/migration contracts.
 
***
 
# 49. No Application-Specific Keyword Explosion
 
The data grammar must remain universal.
 
Do not introduce universal keywords merely for application concepts such as:
 
- computer vision;
- sentiment;
- robotics;
- payments;
- administration;
- legal workflows;
- blockchain;
- VR;
- AR;
- domain-specific business operations.

 
These belong in:
 
- libraries;
- dialects;
- capabilities;
- policies;
- services;
- applications.

 
The universal data grammar should express the computational primitives those applications need.
 
***
 
# 50. Open-World Extension Principle
 
Future data operations should normally be representable using:
 
- identifiers;
- qualified names;
- generic operations;
- expressions;
- metadata;
- attributes;
- capabilities;
- dialects;
- libraries.

 
Adding a new database algorithm should not require changing the universal grammar.
 
Adding a new statistical method should not require changing the universal grammar.
 
Adding a new accelerator should not require changing the universal grammar.
 
Adding a new storage architecture should not require changing the universal grammar.
 
Adding a new quantum-classical data workflow should not require creating another data language.
 
***
 
# 51. Error Model
 
The data subsystem must distinguish at least:
 
```text
LEXICAL_ERROR
SYNTAX_ERROR
STRUCTURAL_ERROR
NAME_ERROR
TYPE_ERROR
SEMANTIC_ERROR
EFFECT_ERROR
CAPABILITY_ERROR
RESOURCE_ERROR
CONTRACT_ERROR
POLICY_ERROR
PROVENANCE_ERROR
COMPATIBILITY_ERROR
TARGET_FEASIBILITY_ERROR
RUNTIME_ERROR
```
 
A syntax error must not be used to report a missing GPU.
 
A resource error must not be reported as a grammar error.
 
A policy rejection must not be reported as a parser failure.
 
This separation is essential for POCO-REAF.
 
***
 
# 52. Diagnostics
 
Data diagnostics should identify:
 
- source location;
- feature;
- rule;
- semantic category;
- relevant symbol;
- requirement;
- constraint;
- capability;
- policy;
- provenance;
- suggested resolution where appropriate.

 
Diagnostics must not expose internal implementation assumptions as if they were language rules.
 
***
 
# 53. Testing Architecture
 
The data subsystem requires tests at multiple levels.
 
Recommended structure:
 
```text
grammar/tests/data/
├── lexical/
├── parser/
├── ast/
├── semantic/
├── types/
├── effects/
├── capabilities/
├── resources/
├── contracts/
├── policies/
├── provenance/
├── knowledge/
├── uncertainty/
├── datasets/
├── queries/
├── transformations/
├── pipelines/
├── schemas/
├── streams/
├── distributed/
├── quantum/
├── hybrid/
├── ai/
├── interoperability/
├── scalability/
├── portability/
├── compatibility/
├── determinism/
├── security/
├── negative/
└── boundary/
```
 
***
 
# 54. Mandatory Positive Tests
 
Every data feature must have valid examples.
 
Examples include:
 
```text
data declaration
data assignment
data query
data transformation
data pipeline
data schema
data collection
data stream
data knowledge
data uncertainty
data provenance
data requirement
data constraint
data preference
```
 
Each example must be tested through the actual canonical parser path.
 
***
 
# 55. Mandatory Negative Tests
 
Tests must cover:
 
- missing operands;
- invalid delimiters;
- malformed expressions;
- invalid names;
- invalid type combinations;
- invalid schema combinations;
- invalid query forms;
- invalid uncertainty metadata;
- invalid knowledge operations;
- invalid contract usage;
- invalid policy usage;
- unsupported capabilities;
- unsatisfied resource requirements;
- incompatible versions.

 
Syntax-invalid and semantically-invalid cases must be tested separately.
 
***
 
# 56. Boundary Tests
 
Boundary testing must include combinations such as:
 
```text
data + AI
data + quantum
data + hybrid
data + HDL
data + distributed
data + networking
data + uncertainty
data + provenance
data + knowledge
data + contracts
data + policies
data + simulation
data + FFI
data + metaprogramming
```
 
Examples should include:
 
```text
quantum measurement -> data
data -> quantum parameter
AI inference -> data
data -> learning
uncertain data -> reasoning
knowledge -> data query
distributed stream -> AI
simulation -> dataset
hardware observation -> uncertain value
```
 
***
 
# 57. Scalability Tests
 
Scalability testing must verify that grammar structure does not impose artificial ceilings.
 
Tests should cover:
 
- large record definitions;
- large collections;
- large schemas;
- many transformation stages;
- many pipeline stages;
- deeply nested expressions;
- many query components;
- many metadata fields;
- large graph descriptions;
- large knowledge structures;
- deeply nested uncertainty;
- large provenance chains;
- large distributed data descriptions.

 
The test suite must not mistake the finite test machine's limits for language-level limits.
 
***
 
# 58. Cross-Domain Test
 
A mandatory integration test should combine:
 
```text
data
+
reasoning
+
knowledge
+
uncertainty
+
learning
+
adaptation
+
contracts
+
provenance
+
policies
+
resource requirements
+
classical computation
+
quantum computation
+
hybrid computation
+
parallelism
+
distribution
+
simulation
+
hardware intent
```
 
The expected pipeline is:
 
```text
source
  |
  v
lexer
  |
  v
parser
  |
  v
domain-neutral AST
  |
  v
structural validation
  |
  v
type analysis
  |
  v
effect analysis
  |
  v
capability analysis
  |
  v
resource analysis
  |
  v
contract analysis
  |
  v
policy analysis
  |
  v
provenance
  |
  v
canonical semantic model
  |
  +----------+----------+
  |                     |
  v                     v
classical             quantum::ir
  |                     |
  +----------+----------+
             |
             v
      target realization
```
 
This is a critical POCO-REAF integration test.
 
***
 
# 59. Reproducibility
 
Data parsing and semantic analysis should be reproducible when the language configuration and source are the same.
 
The data grammar must not depend on:
 
- current database state;
- current hardware;
- current network;
- current storage;
- random state.

 
Runtime data acquisition may of course depend on external state when explicitly permitted by the program's effects and policies.
 
***
 
# 60. Provenance Requirements
 
Data transformations should preserve provenance where the semantic contract requires it.
 
For example:
 
```text
source
  |
  v
dataset
  |
  v
transformation
  |
  v
derived dataset
```
 
should be capable of retaining:
 
```text
derived_from
transformed_by
generated_by
verified_by
source
version
reason
evidence
```
 
The exact provenance representation belongs to the canonical provenance subsystem.
 
***
 
# 61. Resource and Capability Negotiation
 
The data subsystem must support the distinction:
 
```text
requirement
constraint
capability
preference
hint
```
 
For example:
 
```text
requires capability("data.query");
```
 
is different from:
 
```text
prefers capability("accelerated.data.query");
```
 
and different from:
 
```text
constrains latency < required_latency;
```
 
and different from:
 
```text
requires memory >= required_memory;
```
 
The compiler/runtime determines the actual realization.
 
***
 
# 62. No Hidden Hardware Assumptions
 
The data grammar must never silently assume:
 
```text
CPU
GPU
FPGA
ASIC
QPU
NPU
TPU
specific memory size
specific storage size
specific node count
specific network topology
specific device count
```
 
The same data computation must remain semantically expressible across different target classes.
 
***
 
# 63. Safe Rust Requirement
 
The grammar files themselves contain no Rust implementation code.
 
The generated parser/compiler implementation must nevertheless obey:
 
```text
Rust 1.97+
Rust 2021
Safe Rust only
```
 
No `unsafe` Rust is required or permitted for production implementation under this architecture.
 
ANTLR actions and predicates should not be used to smuggle unsafe behavior or target-dependent execution into the grammar.
 
Prefer pure grammar composition.
 
***
 
# 64. Parser Purity
 
Data grammar parsing MUST remain free of side effects.
 
The parser must not:
 
- query a database;
- inspect available resources;
- contact a network;
- execute transformations;
- run an AI model;
- sample uncertainty;
- execute quantum operations;
- invoke hardware;
- allocate physical storage;
- call foreign code.

 
Parsing creates syntax.
 
Semantic analysis creates meaning.
 
Compilation creates realizations.
 
Runtime performs execution.
 
***
 
# 65. Integration with `grammar/Zamani.g4`
 
`grammar/Zamani.g4` remains the root composition boundary.
 
The data subsystem must enter the universal parser through the canonical parser composition hierarchy.
 
The root grammar must not duplicate:
 
```text
dataStmt
dataDeclaration
dataStatement
dataUncertaintyStmt
dataKnowledgeStmt
```
 
or any specialized data rule.
 
The root only composes the data subsystem.
 
***
 
# 66. Integration with `grammar/antlr/ZamaniLexer.g4`
 
The data grammars consume the canonical:
 
```text
ZamaniLexer
```
 
They must not introduce local lexer rules.
 
New domain concepts should not automatically become global keywords.
 
If an operation can be represented by an identifier, qualified name, metadata field, library symbol, or dialect construct, that approach should be preferred over adding a universal keyword.
 
***
 
# 67. Integration with `grammar/lexer/`
 
The canonical lexer registry must remain the lexical source of truth.
 
Data grammar files consume canonical tokens.
 
They must not create:
 
```text
DataIdentifier
DataKeyword
DataProbability
DataQueryToken
DataStorageToken
```
 
when ordinary canonical lexical categories already suffice.
 
***
 
# 68. Integration with `grammar/expressions/`
 
The data subsystem must reuse universal expressions.
 
Particularly important shared expression domains include:
 
- uncertainty;
- knowledge;
- reasoning;
- patterns;
- queries;
- provenance;
- policy;
- function calls;
- indexing;
- member access;
- literals;
- identifiers.

 
A data grammar should not reproduce the universal expression hierarchy.
 
***
 
# 69. Integration with `grammar/types/`
 
Data values must use the canonical type system.
 
Relevant types may include:
 
- scalar types;
- records;
- tuples;
- collections;
- maps;
- sequences;
- streams;
- tensors;
- uncertain values;
- option/result;
- generic types;
- constrained types;
- dependent/parameterized types where supported.

 
The data grammar does not create a separate data type universe.
 
***
 
# 70. Integration with `grammar/effects/`
 
Data effects must be resolved through the universal effect system.
 
Possible effects include:
 
```text
io
network
mutation
randomness
foreign
native
distributed
measurement
simulation
```
 
The actual effect set comes from semantic resolution.
 
***
 
# 71. Integration with `grammar/resources/`
 
Data requirements must flow into the common resource analysis system.
 
The data grammar must not solve:
 
- placement;
- allocation;
- scheduling;
- hardware discovery;
- storage allocation.

 
Those belong downstream.
 
***
 
# 72. Integration with `grammar/validation/`
 
Data constructs may participate in:
 
```text
requires
ensures
invariant
assume
guarantee
property
assertion
refinement
evidence
```
 
The data subsystem consumes these universal semantics.
 
***
 
# 73. Integration with `grammar/policies/`
 
Data execution may be constrained by:
 
- security;
- privacy;
- provenance;
- resource;
- deployment;
- execution;
- adaptation;
- simulation policies.

 
Policy ownership remains outside the data leaf grammars.
 
***
 
# 74. Integration with `grammar/ai/`
 
AI data flows through ordinary data semantics.
 
AI-specific operations should not create duplicate:
 
- dataset syntax;
- stream syntax;
- query syntax;
- uncertainty syntax;
- provenance syntax;
- knowledge syntax.

 
AI consumes and produces universal data values.
 
***
 
# 75. Integration with `grammar/quantum/`
 
Quantum computation may consume or produce data.
 
Data grammar must remain independent of physical quantum realization.
 
The quantum subsystem remains responsible for:
 
- quantum operations;
- qubits;
- measurement;
- circuits;
- dynamic behavior;
- QEC;
- routing;
- scheduling;
- quantum capabilities.

 
***
 
# 76. Integration with `grammar/hybrid/`
 
Hybrid computation provides the bridge:
 
```text
classical
   |
   v
data
   |
   v
quantum
   |
   v
measurement
   |
   v
data
```
 
The data grammar must support the common semantic values used by that pipeline without creating hybrid-specific duplicate data syntax.
 
***
 
# 77. Integration with `grammar/distributed/`
 
Distributed data must use the common:
 
- actors;
- tasks;
- channels;
- messages;
- services;
- topology;
- consistency;
- resilience;
- fault tolerance.

 
Data grammar should describe intent.
 
The distributed subsystem realizes that intent.
 
***
 
# 78. Integration with `grammar/networking/`
 
Network-backed data operations must participate in:
 
- network effects;
- endpoint semantics;
- capability checks;
- resource requirements;
- security policies;
- provenance.

 
Data grammar must not implement network protocols.
 
***
 
# 79. Integration with `grammar/interoperability/`
 
Foreign data systems must cross the explicit interoperability boundary.
 
The architecture is:
 
```text
Zamani
   |
   v
foreign declaration / dialect
   |
   v
ABI / protocol / format
   |
   v
capability + effect + policy validation
   |
   v
external system
```
 
External integration must not silently bypass the effect/security model.
 
***
 
# 80. Integration with Metaprogramming
 
Reflection and code generation may inspect data syntax and metadata.
 
They must remain explicitly controlled by:
 
- reflection capabilities;
- metaprogramming effects;
- security policy;
- provenance.

 
Data grammar itself remains declarative and non-executing.
 
***
 
# 81. File Completion Standard
 
A data grammar file is not considered complete merely because ANTLR accepts it.
 
It is complete only when:
 
```text
Specification
      |
      v
Lexer
      |
      v
Grammar
      |
      v
AST
      |
      v
Semantic model
      |
      v
Types
      |
      v
Effects
      |
      v
Capabilities
      |
      v
Resources
      |
      v
Contracts
      |
      v
Policies
      |
      v
Provenance
      |
      v
Canonical IR
      |
      v
Tests
```
 
has an identified integration contract.
 
***
 
# 82. Definition of Done for `grammar/data/`
 
The data subsystem is production-ready only when all applicable data features have:
 
- one syntax authority;
- one AST representation;
- one semantic owner;
- one canonical IR path;
- explicit type integration;
- explicit effect integration;
- explicit capability integration;
- explicit resource integration;
- explicit contract integration;
- explicit policy integration;
- explicit provenance integration;
- explicit compatibility status;
- positive tests;
- negative tests;
- boundary tests;
- scalability tests;
- determinism tests;
- cross-domain tests;
- portability tests;
- security tests.

 
***
 
# 83. Definition of Done for `data/uncertainty.g4`
 
The uncertainty adapter is complete when:
 
- `ZamaniDataUncertainty` is the grammar identity;
- it uses `tokenVocab = ZamaniLexer`;
- it imports the canonical uncertainty expression grammar;
- it does not duplicate `uncertaintyExpression`;
- it exports a data-domain uncertainty boundary;
- it satisfies the rule expected by `data.g4`;
- it introduces no new uncertainty keywords;
- it introduces no fixed uncertainty limits;
- it introduces no probability representation;
- it introduces no distribution catalogue;
- it introduces no data-specific uncertainty AST;
- it introduces no uncertainty-specific IR;
- it integrates with the existing semantic uncertainty model;
- it preserves compatibility;
- it has parser tests;
- it has semantic tests;
- it has cross-domain tests;
- it has scalability tests.

 
***
 
# 84. Definition of Done for `data/knowledge.g4`
 
The knowledge adapter is complete when:
 
- `ZamaniDataKnowledge` is the grammar identity;
- canonical knowledge syntax remains owned by `expressions/knowledge.g4`;
- no knowledge syntax is duplicated;
- the adapter is imported by `data.g4`;
- its public boundary is consumed by the data orchestrator;
- AST ownership remains domain-neutral;
- semantic ownership remains canonical;
- knowledge works across data, AI, classical, quantum/hybrid and distributed contexts.

 
***
 
# 85. Definition of Done for `data/provenance.g4`
 
The provenance adapter is complete when:
 
- it does not create a second provenance language;
- canonical universal provenance remains authoritative;
- data lineage remains data-specific only where necessary;
- provenance metadata survives data transformations;
- provenance integrates with evidence and policy;
- provenance reaches the semantic representation;
- no physical storage assumptions are introduced.

 
***
 
# 86. Required Repository-Level Validation
 
Before declaring `grammar/data/` production-ready, run the relevant repository validation using the required Rust toolchain.
 
At minimum:
 
```text
cargo fmt --check
cargo check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```
 
plus the repository's ANTLR grammar generation/conformance commands.
 
The exact ANTLR generation command must come from the repository's existing build/tooling configuration rather than being invented by the grammar documentation.
 
The implementation must be verified with:
 
```text
Rust >= 1.97
Rust 2021
safe Rust only
```
 
***
 
# 87. Independent-File Development Rule
 
Each data grammar file must be capable of being completed independently.
 
Before declaring a file done, its author must know:
 
```text
WHO CONSUMES THIS FILE?
WHAT DOES IT EXPORT?
WHAT DOES IT DEPEND ON?
WHO OWNS THE AST?
WHO OWNS THE SEMANTICS?
WHO OWNS THE TYPES?
WHO OWNS THE EFFECTS?
WHO OWNS THE CAPABILITIES?
WHO OWNS THE RESOURCES?
WHO OWNS THE CONTRACTS?
WHO OWNS THE POLICIES?
WHO OWNS THE PROVENANCE?
WHO OWNS THE IR?
WHO OWNS THE TESTS?
WHO OWNS THE SPECIFICATION?
```
 
These answers must be documented in the file itself.
 
This prevents the situation where a supposedly completed grammar must later be redesigned merely because another subsystem was implemented.
 
***
 
# 88. Change Propagation Rule
 
A change in one subsystem must not automatically cause unrelated data grammars to be rewritten.
 
For example:
 
A new AI algorithm MUST NOT require rewriting:
 
```text
data/datasets.g4
```
 
A new quantum operation MUST NOT require rewriting:
 
```text
data/uncertainty.g4
```
 
A new storage device MUST NOT require rewriting:
 
```text
data/schemas.g4
```
 
A new accelerator MUST NOT require rewriting:
 
```text
data/streams.g4
```
 
A new uncertainty metadata field SHOULD NOT require rewriting every data consumer.
 
This is achieved through stable semantic boundaries and open-world identifiers.
 
***
 
# 89. Architectural Invariant
 
The following invariant must hold:
 
```text
One language
     |
     v
One lexical authority
     |
     v
One parser composition hierarchy
     |
     v
One domain-neutral AST
     |
     v
One semantic foundation
     |
     +---- Types
     +---- Effects
     +---- Capabilities
     +---- Resources
     +---- Contracts
     +---- Policies
     +---- Provenance
     |
     v
Domain semantics
     |
     +---- Classical
     +---- Quantum
     +---- AI
     +---- HDL
     +---- Data
     +---- Distributed
     +---- Networking
     +---- Hybrid
     |
     v
Canonical semantic representation
     |
     +---- Classical IR
     +---- quantum::ir
     +---- domain-specific lowerings
     |
     v
Target realization
```
 
The data subsystem is one domain within this model.
 
It must never become a competing language architecture.
 
***
 
# 90. Final Data Architecture
 
The intended final architecture is:
 
```text
                         Zamani
                            |
             +--------------+--------------+
             |                             |
       Universal Core                Domain Systems
             |                             |
       Types / Values              +-------+--------+
       Operations                  |       |        |
       Expressions                 Data   AI     Quantum
       Effects                    |       |        |
       Capabilities               |       |        |
       Resources                  +-------+--------+
       Contracts                          |
       Policies                           |
       Provenance                         |
             |                            |
             +-------------+--------------+
                           |
                   Semantic Model
                           |
             +-------------+-------------+
             |                           |
        Classical/data              quantum::ir
             |                           |
             +-------------+-------------+
                           |
                 Target-independent
                    optimization
                           |
                    lowering/routing
                           |
                     scheduling
                           |
                  resilience/recovery
                           |
                    target realization
                           |
        +----------+-------+-------+----------+
        |          |       |       |          |
       CPU        GPU     FPGA    ASIC       QPU
        |          |       |       |          |
        +----------+-------+-------+----------+
                           |
                HPC / cluster / cloud /
                 distributed / future
```
 
The data grammar therefore remains:
 
**portable, domain-neutral, open-ended, capability-driven, resource-aware, provenance-aware, policy-aware, and target-independent.**
 
***
 
# 91. Final Architectural Rules
 
The following rules are mandatory for all future work under `grammar/data/`:
 
1. **One syntax owner per feature.**
2. **No duplicate uncertainty syntax.**
3. **No duplicate knowledge syntax.**
4. **No duplicate provenance language.**
5. **No data-specific competing AST.**
6. **No data-specific competing canonical IR.**
7. **No hardware-specific grammar limits.**
8. **No universal capacity constants.**
9. **No fixed tensor rank.**
10. **No fixed dataset size.**
11. **No fixed stream size.**
12. **No fixed graph size.**
13. **No fixed partition count.**
14. **No fixed replica count.**
15. **No fixed node count.**
16. **No fixed device count.**
17. **No parser-time execution.**
18. **No hidden hardware discovery.**
19. **No vendor-specific universal syntax.**
20. **No application-specific keyword explosion.**
21. **Reuse universal types.**
22. **Reuse universal effects.**
23. **Reuse universal capabilities.**
24. **Reuse universal resources.**
25. **Reuse universal contracts.**
26. **Reuse universal policies.**
27. **Reuse universal provenance.**
28. **Preserve domain-neutral AST semantics.**
29. **Preserve `quantum::ir` as the canonical quantum boundary.**
30. **Keep SQL/JSON/XML at interoperability/dialect boundaries.**
31. **Use identifiers and qualified names for open-world extension.**
32. **Require positive, negative, boundary and scalability tests.**
33. **Require cross-domain tests.**
34. **Require deterministic parsing.**
35. **Require compatibility contracts.**
36. **Require every feature file to document its complete integration contract.**
37. **Use Rust 1.97.1 or later.**
38. **Use Rust 2021.**
39. **Production Rust must remain safe Rust.**
40. **Never confuse physical resource limitations with language-level semantic limits.**

 
***
 
# 92. Completion Statement
 
`grammar/data/` can be considered production-ready only when the data grammar is no longer merely a collection of parser files, but a fully integrated source-language subsystem whose constructs can be traced through:
 
```text
SPECIFICATION
      |
      v
LEXER
      |
      v
GRAMMAR
      |
      v
AST
      |
      v
SEMANTICS
      |
      v
TYPE CHECKING
      |
      v
EFFECT CHECKING
      |
      v
CAPABILITY CHECKING
      |
      v
RESOURCE CHECKING
      |
      v
CONTRACT CHECKING
      |
      v
POLICY CHECKING
      |
      v
PROVENANCE
      |
      v
CANONICAL SEMANTIC MODEL
      |
      v
CANONICAL IR
      |
      v
OPTIMIZATION
      |
      v
LOWERING
      |
      v
ROUTING
      |
      v
SCHEDULING
      |
      v
RESILIENCE
      |
      v
TARGET REALIZATION
```
 
The decisive architectural property is:
 
> **Data syntax describes what the program means, not the size, vendor, topology, architecture, or physical realization of the machine executing it.**

 
That is what allows the data subsystem to participate in Zamani's goal of scaling from tiny computations to arbitrarily large computations wherever the required semantic capabilities and physical resources are available.
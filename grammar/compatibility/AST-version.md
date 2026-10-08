Zamani AST Version and Compatibility Contract

Path: "grammar/compatibility/AST-version.md"
Status: Normative
Language: Zamani
Repository: "Benwellonedge28/Zamani"
Rust edition: 2021
Rust baseline: Rust 1.97 or later
Production safety: Rust "unsafe" MUST NOT be used
Primary architecture: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)

---

1. Purpose

This document defines the normative compatibility contract for the Zamani frontend Abstract Syntax Tree (AST) schema.

It establishes:

- what constitutes the Zamani AST schema;
- what an AST version identifies;
- how AST versions evolve;
- how AST compatibility is determined;
- how AST schema changes interact with source language versions;
- how AST schema changes interact with grammar versions;
- how AST schema changes interact with semantic versions;
- how AST schema changes interact with canonical IR versions;
- how AST serialization versions remain separate;
- how node-local schema versions remain separate;
- how AST migrations are performed;
- how AST compatibility is tested;
- how AST provenance is preserved;
- how AST compatibility supports POCO-REAF;
- how the AST remains independent of target hardware and resource capacity.

The fundamental principle is:

«The AST version describes the structural contract of the frontend AST. It does not define the Zamani language, compiler, hardware, runtime, or canonical IR.»

The AST is a source-level representation of programmer intent.

It MUST remain independent of:

- CPU architecture;
- GPU architecture;
- FPGA architecture;
- ASIC implementation;
- accelerator implementation;
- QPU vendor;
- physical qubit numbering;
- quantum topology;
- calibration;
- routing;
- scheduling;
- QEC implementation;
- ZQN implementation;
- HAL implementation;
- runtime implementation.

---

2. Normative terminology

The following terms are normative:

Term| Meaning
MUST| Mandatory
MUST NOT| Prohibited
SHOULD| Recommended unless a documented reason exists
SHOULD NOT| Discouraged unless justified
MAY| Permitted
AST| Frontend abstract syntax tree
AST schema| Logical structural contract of the native frontend AST
AST schema version| Version identifying that logical AST structural contract
AST serialization format| Representation used to encode an AST outside memory
node-local schema version| Version of one AST node-family representation
language version| Version of Zamani source-language semantics
grammar version| Version of concrete grammar representation
semantic version| Version of the semantic-model contract
IR version| Version of a canonical intermediate representation contract
dialect version| Version of an explicitly versioned language extension
migration| Explicit transformation between incompatible AST representations
compatibility| Ability to consume an AST under a defined compatibility contract without changing its specified meaning

---

3. Authority and ownership

AST versioning MUST NOT create another language authority.

The repository authority relationship is:

grammar/DESIGN.md
        │
        ▼
normative language specifications
        │
        ├── grammar/specification/
        └── grammar/spec/
        │
        ▼
canonical grammar
grammar/Zamani.g4
        │
        ▼
lexer / parser
        │
        ▼
native frontend AST
src/frontend/ast/
        │
        ▼
structural validation
        │
        ▼
semantic analysis
        │
        ▼
semantic model
        │
        ├── classical
        ├── quantum
        ├── HDL
        ├── AI
        ├── data
        ├── distributed
        └── hybrid
        │
        ▼
canonical IR
        │
        ├── Classical IR
        └── quantum::ir
        │
        ▼
optimization / lowering
        │
        ▼
routing / scheduling / resilience / QEC / ZQN
        │
        ▼
HAL / target realization

3.1 This file owns

"grammar/compatibility/AST-version.md" owns:

- AST schema identity;
- aggregate AST schema versioning;
- AST compatibility classification;
- AST version evolution;
- AST compatibility requirements;
- AST migration requirements;
- compatibility relationships between AST schema versions;
- AST-version diagnostics;
- AST-version conformance requirements;
- AST-version integration requirements.

3.2 This file does not own

This file MUST NOT own:

- source-language syntax;
- language-version numbering;
- lexer token definitions;
- parser grammar;
- semantic meaning;
- type semantics;
- effect semantics;
- capability semantics;
- resource semantics;
- policy semantics;
- provenance semantics;
- canonical IR definitions;
- quantum operation semantics;
- quantum routing;
- QEC;
- ZQN;
- scheduling;
- hardware realization;
- runtime behavior;
- ABI definitions;
- serialization encoding mechanisms;
- JSON encoding;
- binary encoding;
- compiler release numbering.

Those concerns remain owned by their respective repository authorities.

---

4. Existing repository integration

The AST-version contract MUST integrate with the repository's existing architecture rather than creating duplicate mechanisms.

Existing component| Responsibility| AST-version relationship
"grammar/DESIGN.md"| Overall grammar architecture| Defines architectural boundary
"grammar/specification/language-version.md"| Language-version authority| AST version MUST remain independent
"grammar/spec/compatibility.md"| Cross-layer compatibility| Consumes AST compatibility classifications
"grammar/compatibility/versions.md"| Version taxonomy| Defines version-layer separation
"grammar/compatibility/feature-gates.md"| Feature availability| Gates AST-producing features where required
"grammar/compatibility/migrations.md"| Migration procedures| Owns migration process
"grammar/compatibility/deprecated.md"| Deprecation lifecycle| Defines AST-related deprecation lifecycle
"grammar/compatibility/compatibility-matrix.md"| Cross-layer matrix| Records AST compatibility relationships
"grammar/compatibility/frontend-conformance.md"| Frontend conformance| Verifies source → parser → AST
"grammar/grammar.md"| Implementation status| Reports AST implementation status
"grammar/validation/ast-coverage.md"| AST coverage| Verifies grammar-to-AST traceability
"src/frontend/ast/"| Native frontend AST| Canonical AST implementation
"src/frontend/ast/node/serialization/version.rs"| AST serialization version types| Provides implementation-level serialization/version separation
"src/frontend/ast/node/serialization/schema.rs"| Serialization schema| Separate from logical AST schema
"src/frontend/ast/node/metadata.rs"| AST metadata| Carries version metadata
"src/frontend/ast/node/visitors/"| AST traversal| Has its own visitor schema contract
"src/frontend/ast/node/program/"| Program-level AST nodes| Participates in aggregate AST schema
semantic layer| Semantic interpretation| Consumes AST; does not define AST version
Classical IR| Classical canonical representation| Downstream of AST
"quantum::ir"| Canonical quantum semantic boundary| Downstream of AST
HDL semantic/IR layer| Hardware semantics| Downstream of AST
compiler/lowering| Target realization| Must not redefine AST
tests| Conformance evidence| Validate the complete AST contract

---

5. The AST version layers

Zamani MUST distinguish all of the following.

Language Version
       │
       ▼
Grammar Version
       │
       ▼
Frontend AST Schema Version
       │
       ├── node-local schema versions
       │
       ├── AST visitor API/schema version
       │
       └── AST metadata contract
       │
       ▼
AST Serialization Schema Version
       │
       ▼
AST Serialization Format Version
       │
       ▼
Semantic Model Version
       │
       ▼
Canonical IR Versions
       │
       ├── Classical IR
       └── quantum::ir

These versions MUST NOT be collapsed into one number.

---

6. AST schema version

The AST schema version identifies the logical aggregate structure of the native Zamani frontend AST.

It answers:

«Which structural AST contract is this AST instance conforming to?»

It covers the aggregate AST contract including, where applicable:

- node identity;
- node kinds;
- node relationships;
- fields;
- field meaning;
- field optionality;
- field multiplicity;
- discriminants;
- structural invariants;
- source-span representation;
- metadata required by downstream semantic analysis;
- generic constructs;
- patterns;
- expressions;
- statements;
- declarations;
- modules;
- types;
- effects;
- capabilities;
- resources;
- contracts;
- policies;
- provenance;
- domain-neutral quantum constructs;
- HDL constructs;
- classical constructs;
- hybrid constructs;
- AI/data constructs.

It does NOT identify:

- the concrete grammar;
- the source language version;
- a serialized encoding;
- a compiler release;
- a target architecture.

---

7. Canonical AST implementation

Repository evidence establishes the native frontend AST under:

src/frontend/ast/

and particularly:

src/frontend/ast/node/

The AST-version contract MUST therefore refer to the native frontend AST rather than inventing a second AST hierarchy.

The canonical direction is:

source
  ↓
lexer
  ↓
parser
  ↓
src/frontend/ast/
  ↓
structural validation
  ↓
semantic analysis

An alternative AST implementation MUST NOT be introduced merely for versioning.

---

8. Aggregate AST schema versus serialization schema

This distinction is mandatory.

The repository already contains separate implementation concepts for:

AST schema
AST serialization schema
AST serialization format
node-local schema
visitor schema

They MUST remain separate.

8.1 Logical AST schema

The logical AST schema describes:

what nodes exist
what fields they contain
how they relate
what information they preserve

8.2 Serialization schema

The serialization schema describes:

what fields are persisted
what field identifiers are used
which fields are required
which fields are optional
how extensions are represented
how unknown fields are handled

8.3 Serialization format

The serialization format describes:

how the schema is encoded

Examples may include:

- JSON;
- binary;
- another explicitly specified interchange format.

A serialization-format change MUST NOT automatically imply a logical AST-schema change.

Conversely, a logical AST change MAY require serialization changes.

---

9. Node-local schema versions

Individual AST nodes MAY expose node-local schema versions.

Examples already present in the repository include node-family schema-version constants.

These versions MUST NOT be treated as the aggregate AST schema version.

The hierarchy is:

AST schema version
        │
        ├── Program node schema
        ├── Module node schema
        ├── Expression node schema
        ├── Statement node schema
        ├── Type node schema
        ├── Pattern node schema
        ├── Domain node schema
        └── other node-family schemas

A node-local change MUST be evaluated to determine whether it changes:

1. only that node-family contract;
2. the aggregate AST schema;
3. the serialized schema;
4. the semantic contract;
5. the language contract.

The implementation MUST NOT assume that all five change together.

---

10. AST visitor schema version

The AST visitor infrastructure has its own schema/version contract.

A visitor API change MUST NOT automatically increment:

- language version;
- grammar version;
- aggregate AST schema version;
- serialization format version.

A visitor change affects AST traversal infrastructure unless it changes the structural AST contract itself.

Therefore:

AST schema
      ≠
AST visitor schema

The visitor contract remains an implementation-facing compatibility layer.

---

11. AST metadata version information

AST metadata MAY contain multiple version identities.

A valid AST metadata model may need to distinguish:

language_version
grammar_version
ast_schema_version
serialization_schema_version
serialization_format_version
compiler_version
semantic_model_version
dialect_versions

The metadata MUST NOT use one field to represent all of these.

Every version field MUST have an explicit namespace and meaning.

For example:

language_version
ast_schema_version
serialization_schema_version

are three different values.

---

12. AST version identifier

The canonical AST schema version SHOULD use semantic-version-style components:

MAJOR.MINOR.PATCH

For example:

1.0.0
1.1.0
1.1.1
2.0.0

These numbers describe the AST structural contract.

They do not describe computational capacity.

They MUST NOT encode:

- number of CPUs;
- number of qubits;
- memory capacity;
- number of devices;
- GPU count;
- node count;
- tensor dimensions;
- network size.

---

13. Major AST schema changes

An AST schema MAJOR increment is required when a stable AST consumer cannot correctly interpret the new structure under the previous contract.

Examples include:

- removing a stable node kind;
- changing a stable node discriminant incompatibly;
- removing a required field;
- changing a field from one incompatible semantic category to another;
- changing the meaning of a stable field;
- changing child ownership incompatibly;
- changing structural invariants incompatibly;
- changing source information in a way that breaks required consumers;
- changing AST representation such that a consumer can no longer recover the specified source meaning.

A major AST change MUST have:

- explicit migration policy;
- compatibility classification;
- diagnostics;
- test fixtures;
- migration tests;
- updated compatibility matrix;
- semantic-impact analysis.

---

14. Minor AST schema changes

A MINOR increment MAY be used for a backward-compatible structural extension.

Examples include:

- adding an optional field;
- adding a new explicitly extensible node attribute;
- adding an extension namespace;
- adding metadata that old consumers can safely ignore;
- adding a new representational capability without changing existing node meaning.

A minor change MUST NOT cause an existing valid AST to be reinterpreted differently.

If an older consumer cannot safely ignore the addition, the change is not backward-compatible and MUST be classified accordingly.

---

15. Patch AST schema changes

A PATCH increment is appropriate for compatible corrections that preserve the structural and semantic contract.

Examples include:

- correcting schema documentation;
- correcting metadata descriptions;
- correcting validation documentation;
- correcting a non-semantic schema manifest defect;
- correcting deterministic schema ordering without changing field identity;
- correcting implementation metadata while preserving the actual contract.

A patch version MUST NOT silently change the meaning of existing AST data.

---

16. Compatibility classes

AST compatibility MUST be evaluated explicitly.

At minimum, the following classifications SHOULD be supported:

AST_EXACT
AST_BACKWARD_COMPATIBLE
AST_FORWARD_COMPATIBLE
AST_MIGRATABLE
AST_INCOMPATIBLE
AST_UNKNOWN

16.1 AST_EXACT

The consumer and producer use the same AST schema contract.

16.2 AST_BACKWARD_COMPATIBLE

A newer consumer can consume ASTs produced under an older compatible schema without migration.

16.3 AST_FORWARD_COMPATIBLE

An older consumer can consume a newer AST only when the newer schema explicitly guarantees that the new information is safely ignorable.

This MUST NOT be assumed merely because the version difference is minor.

16.4 AST_MIGRATABLE

The representations differ, but a documented deterministic migration exists.

16.5 AST_INCOMPATIBLE

The representations cannot be safely consumed without an explicit semantic migration.

16.6 AST_UNKNOWN

The implementation does not know the schema.

Unknown AST versions MUST NOT be silently interpreted as known versions.

---

17. Unknown versions

An implementation MUST reject an unknown AST schema version unless an explicitly registered compatibility mechanism can safely handle it.

The implementation MUST NOT:

unknown version
      ↓
guess nearest version
      ↓
continue

Instead:

unknown version
      ↓
diagnose
      ↓
identify supported versions
      ↓
offer explicit migration if available

This prevents silent structural corruption.

---

18. No silent reinterpretation

A consumer MUST NOT silently reinterpret an AST under a different schema.

For example:

AST 2.x

MUST NOT automatically be treated as:

AST 1.x

unless an explicit compatibility rule states that this exact relationship is safe.

Version similarity is not evidence of compatibility.

---

19. AST version versus language version

These are distinct.

Zamani language version
        ≠
AST schema version

A language-version change MAY leave the AST schema unchanged.

An AST schema change MAY occur without changing source-language semantics.

For example:

Language 1.2
AST 1.3

can be valid.

Likewise:

Language 2.0
AST 1.3

can be valid if the same AST representation remains sufficient.

The repository's "grammar/specification/language-version.md" remains the authority for language versions.

This file only defines the AST side of the relationship.

---

20. AST version versus grammar version

These are also distinct.

grammar version
       ≠
AST schema version

A grammar implementation may change internally while producing the same canonical AST.

Conversely, the AST may change while the concrete grammar remains unchanged if additional structural information becomes necessary.

The compatibility pipeline is:

grammar
   ↓
parse tree
   ↓
AST

Each boundary has its own contract.

---

21. AST version versus semantic model version

The AST is a structural representation.

The semantic model assigns meaning.

Therefore:

AST structure
     ↓
semantic interpretation

A semantic-model change MUST NOT be disguised as an AST schema change.

For example, changing the meaning of an effect while keeping the same AST node structure is a semantic compatibility event, not necessarily an AST-schema event.

Likewise, changing an AST node field does not automatically redefine the language semantics.

Both compatibility dimensions MUST be evaluated independently.

---

22. AST version versus canonical IR

The AST is upstream of canonical IR.

AST
 ↓
semantic analysis
 ↓
canonical semantic representation
 ↓
IR

The AST schema MUST NOT depend on a particular IR representation.

The canonical quantum boundary remains:

quantum::ir

The AST MUST NOT contain fields whose only purpose is to encode:

- physical qubit assignments;
- vendor-specific gate handles;
- routing decisions;
- pulse schedules;
- QEC layouts;
- hardware calibration;
- target-specific instructions.

Those belong downstream.

---

23. AST version versus Classical IR

The AST MUST remain independent from the representation chosen by the canonical classical IR.

A classical AST construct may eventually lower to:

Classical IR

without requiring the AST to know:

- instruction selection;
- register allocation;
- machine registers;
- CPU instruction sets;
- GPU instruction sets;
- accelerator instructions.

---

24. AST version versus HDL

HDL constructs may be represented in the frontend AST.

However, the AST MUST describe source-level hardware intent rather than physical realization.

It MUST NOT require:

physical FPGA resource index
physical ASIC cell
vendor primitive identifier
fixed bus width
fixed number of lanes
fixed hardware count

unless such information is explicitly part of source semantics and intentionally target-specific.

---

25. AST version versus AI and reasoning features

Reasoning, knowledge, learning, adaptation, uncertainty, provenance, evidence, explanations, policies, agents, neural-symbolic composition and related features MUST be represented using the universal AST architecture.

The AST MUST NOT become an application-specific AST.

For example, application concepts such as:

- computer vision;
- sentiment analysis;
- robotics;
- payments;
- administration;
- legal workflows;
- virtual/augmented reality;
- blockchain applications;

MUST NOT create universal AST node families merely because a library happens to use them.

Libraries, dialects, capabilities, policies and domain services own such application-specific concepts.

---

26. AST representation of extensible quantum operations

The AST MUST support generic quantum operation representation.

It MUST NOT require a closed AST enumeration equivalent to:

H
X
Y
Z
CNOT
...

A generic quantum AST representation SHOULD preserve:

operation identity
operation kind/specifier
targets
parameters
results
attributes
modifiers
effects
capabilities
resources
source provenance

The AST MUST preserve the semantic information needed to reach "quantum::ir".

New quantum operations MUST be addable through the established extensibility/dialect/metadata mechanism without requiring artificial changes to the universal AST merely because a new physical operation exists.

---

27. Source provenance

Every AST node that participates in diagnostics or semantic transformation MUST preserve sufficient source provenance.

At minimum, the architecture SHOULD preserve:

source identity
source span
node identity
parent/child relationship

Where required, provenance MAY additionally contain:

macro origin
generated origin
dialect origin
migration origin
transformation origin

AST version changes MUST NOT silently discard source information required for:

- diagnostics;
- semantic analysis;
- migration;
- explainability;
- provenance;
- reproducibility;
- tooling.

---

28. Node identity

AST node identity and AST schema identity are different concepts.

NodeId
   ≠
AST schema version

A "NodeId" identifies an AST node instance.

The schema version identifies the representation contract.

Node identifiers MUST NOT encode:

- memory addresses;
- target hardware IDs;
- physical qubit IDs;
- process IDs;
- timestamps;
- random compatibility state.

Node identity MUST remain deterministic where the AST architecture requires deterministic identity.

---

29. AST extensions

The AST MAY support versioned extensions.

Extensions MUST have:

- stable extension identifier;
- owning specification;
- version;
- namespace;
- compatibility classification;
- structural contract;
- semantic contract;
- migration policy;
- test coverage.

Extensions MUST NOT silently modify the meaning of existing fields.

An extension MUST NOT become a hidden universal field dump.

---

30. Extension namespaces

Extension identity SHOULD use stable qualified namespaces.

Conceptually:

zamani.<domain>.<extension>

or another repository-approved namespace scheme.

The exact namespace authority remains with the repository's extension/dialect system.

Extension namespaces MUST NOT collide accidentally.

An extension version MUST remain distinct from:

- language version;
- AST schema version;
- compiler version;
- target version.

---

31. Dialect interaction

A dialect MAY extend the AST.

The dialect MUST declare:

dialect identity
dialect version
language compatibility
AST compatibility
semantic compatibility
IR mapping
migration policy

A dialect MUST NOT silently alter the core AST meaning.

Dialect-specific AST nodes MUST have an explicit owner.

---

32. Feature-gate interaction

A feature gate determines whether a defined feature is available.

It does not define AST semantics.

The relationship is:

feature specification
        ↓
feature identity
        ↓
feature gate
        ↓
grammar
        ↓
AST
        ↓
semantic analysis

A feature MUST NOT be considered supported merely because:

- its grammar parses;
- a feature gate exists;
- an AST enum variant exists.

The feature must have complete downstream integration.

---

33. AST coverage integration

"grammar/validation/ast-coverage.md" is the principal validation companion.

Every supported grammar construct MUST be traceable through:

specification
    ↓
grammar
    ↓
parser
    ↓
AST
    ↓
semantic model
    ↓
canonical IR

AST-version compatibility MUST therefore be checked as part of AST coverage.

A grammar rule without a valid AST representation is not complete.

An AST node without an authoritative source-language origin is also not complete unless it is explicitly an internal/generated AST node.

---

34. AST generated/internal nodes

The frontend MAY contain internally generated AST structures.

Such nodes MUST be clearly classified as:

SOURCE_AST
DERIVED_AST
INTERNAL_AST
TOOLING_AST

A generated/internal node MUST NOT accidentally become part of the public source AST contract.

If it is serialized or exposed to external consumers, its schema ownership MUST be explicitly declared.

---

35. Serialization compatibility

The existing serialization implementation separates:

AST schema version
serialization schema version
serialization format version

That separation MUST remain.

A serializer MUST record enough metadata to determine:

what schema was serialized
how it was encoded
what compatibility rules apply

A deserializer MUST validate these values before interpreting payload fields.

---

36. Unknown serialization fields

Unknown serialized fields MAY be accepted only when the serialization schema explicitly defines them as safely ignorable.

Otherwise:

unknown required field
       ↓
reject

and not:

unknown field
       ↓
ignore
       ↓
guess meaning

The policy MUST distinguish:

- unknown optional extension;
- unknown required field;
- unknown node kind;
- unknown discriminant;
- unknown schema version.

---

37. Required-field evolution

Changing a field from:

optional

to:

required

is generally compatibility-sensitive.

Changing:

required

to:

optional

may be backward-compatible for producers but can be incompatible for consumers that require the field.

Each direction MUST be evaluated separately.

---

38. Field removal

Removing a stable field requires an explicit compatibility classification.

A field MUST NOT be removed merely because current compiler code no longer uses it.

Before removal, verify:

- semantic consumers;
- serialization consumers;
- tooling;
- IDE integrations;
- migration tools;
- external AST consumers;
- debugging/provenance tools.

If old information remains necessary for compatibility, the field MUST be retained or migrated explicitly.

---

39. Field renaming

A field rename MUST NOT be treated as a harmless documentation change when the field identifier is externally observable.

The migration MUST distinguish:

old field identity
new field identity
semantic equivalence
serialization identity

A compatibility alias MAY be provided where appropriate.

---

40. Discriminant changes

Changing an AST enum/discriminant is compatibility-sensitive.

The implementation MUST evaluate:

- serialized identity;
- parser construction;
- visitor behavior;
- semantic dispatch;
- pattern matching;
- migration;
- external tooling.

A discriminant MUST NOT be reused for a different semantic meaning without an explicit incompatible version/migration.

---

41. Child relationship changes

Changing:

child A belongs to node X

into:

child A belongs to node Y

may change structural semantics even if all fields remain present.

Such a change MUST be evaluated as an AST schema compatibility event.

---

42. Ordering semantics

AST collections MUST explicitly define whether ordering is:

- semantically significant;
- source-preserving;
- deterministic but semantically irrelevant;
- canonicalized.

Examples include:

module items
function parameters
generic parameters
attributes
arguments
statements
quantum targets
operation parameters
record fields

Changing ordering semantics MUST be compatibility-reviewed.

---

43. Optional versus absent values

The AST MUST distinguish where required:

absent
present with empty value
present with default
unknown
not applicable

These states MUST NOT be collapsed merely to simplify serialization.

The exact representation belongs to the AST implementation and schema.

---

44. AST defaults

Adding a default value MAY be backward-compatible only if the default is semantically identical to absence under the previous schema.

A default MUST NOT silently introduce new semantics.

Defaults MUST be specified in the AST contract, not guessed by individual consumers.

---

45. AST migration model

When AST versions are incompatible, migration MUST be explicit.

The migration pipeline is:

AST V_old
   │
   ▼
validate old schema
   │
   ▼
migration
   │
   ▼
validate new schema
   │
   ▼
AST V_new

Migration MUST NOT bypass validation.

---

46. Migration properties

A production AST migration SHOULD be:

- deterministic;
- explicit;
- reproducible;
- idempotent where practical;
- source-aware;
- provenance-preserving;
- semantically conservative;
- testable;
- version-addressable.

Migration MUST NOT silently discard information unless the compatibility contract explicitly permits that loss.

---

47. Migration provenance

A migrated AST SHOULD retain provenance describing:

source AST version
target AST version
migration identity
migration version
migration tool/compiler identity
migration diagnostics

This is especially important for:

- reproducible builds;
- debugging;
- compiler caches;
- long-lived artifacts;
- tooling;
- research/scientific workflows.

---

48. Migration and source semantics

An AST migration MUST preserve source-level meaning.

The desired invariant is:

semantic(AST_old)
    =
semantic(migrate(AST_old))

subject to explicitly documented compatibility exceptions.

If semantic preservation cannot be guaranteed, the migration MUST report the affected construct rather than silently continuing.

---

49. Migration versus lowering

AST migration is not lowering.

AST migration
    =
AST representation V1 → AST representation V2

while:

lowering
    =
semantic representation → target/IR representation

These operations MUST remain separate.

AST migration MUST NOT introduce:

- physical register allocation;
- physical qubit mapping;
- instruction selection;
- routing;
- scheduling;
- calibration;
- target-specific code.

---

50. Migration versus optimization

Optimization MUST NOT be hidden inside AST migration.

Migration preserves representation compatibility.

Optimization changes representation for execution efficiency while preserving semantics.

Therefore:

migration
    ≠
optimization

---

51. Backward compatibility requirements

A newer AST consumer SHOULD accept older compatible AST schemas when explicitly declared.

For example:

AST 1.1
consumer supports 1.0 → 1.1

may be valid.

The compatibility relation MUST be machine-checkable.

It MUST NOT be inferred solely from major/minor numbers.

---

52. Forward compatibility requirements

Forward compatibility MUST be explicitly declared.

A consumer supporting:

AST 1.0

MUST NOT automatically accept:

AST 1.1

unless the schema contract states that the added representation is safely ignorable.

Unknown nodes that can affect semantics MUST cause rejection.

---

53. Compatibility matrix integration

"grammar/compatibility/compatibility-matrix.md" MUST contain AST compatibility relationships.

At minimum it SHOULD distinguish:

Producer| Consumer| Result
same AST version| same AST version| exact
older compatible AST| newer consumer| backward-compatible
newer compatible AST| older consumer| forward-compatible only when explicitly guaranteed
incompatible AST| consumer with migration| migratable
incompatible AST| no migration| reject
unknown AST| consumer| reject unless explicit extension mechanism applies

The matrix MUST remain synchronized with migration declarations.

---

54. Compatibility with source programs

AST compatibility MUST NOT be confused with source compatibility.

A source program can remain valid even if its AST representation changes.

For example:

source
  ↓
AST 1

may become:

source
  ↓
AST 2

while the source-language meaning remains unchanged.

Therefore:

AST breaking change

does not necessarily mean:

language breaking change

The impact MUST be evaluated independently.

---

55. Compatibility with compiler versions

Compiler versions are implementation versions.

A compiler MAY support:

AST 1.x
AST 2.x

simultaneously.

The compiler version MUST NOT be embedded into AST semantics.

Compiler upgrades MUST NOT automatically invalidate AST artifacts unless their declared AST support changes.

---

56. Compatibility with runtime and target versions

AST compatibility MUST be target-independent.

An AST does not become incompatible because a target:

- lacks memory;
- lacks quantum capability;
- lacks a GPU;
- lacks an accelerator;
- has a different topology;
- has fewer resources;
- has different hardware;
- is a simulator rather than physical hardware.

Those are downstream feasibility questions.

---

57. Resource and capability separation

The AST MUST preserve resource and capability requirements where they are source semantics.

For example:

requires qubits >= n;
requires memory >= required_memory;
requires capability("quantum.measurement");
requires capability("gpu.compute");
requires capability("tensor.compute");
requires topology(required_topology);

These requirements are not AST-version limits.

The AST represents the requirement.

The semantic/resource system evaluates it.

The target system realizes it.

---

58. No artificial scalability ceilings

The AST-version contract MUST NOT establish universal limits such as:

MAX_QUBITS
MAX_CPUS
MAX_GPUS
MAX_FPGAS
MAX_ASICS
MAX_QPUS
MAX_NODES
MAX_MEMORY
MAX_THREADS
MAX_REGISTER_WIDTH
MAX_TENSOR_RANK
MAX_NETWORK_SIZE
MAX_DEVICE_COUNT
MAX_AST_NODES
MAX_MODULES
MAX_STATEMENTS
MAX_PROGRAM_SIZE

The AST MUST introduce no language-level ceiling based on such constants.

The architecture is:

source requirements
       ↓
AST representation
       ↓
semantic requirements
       ↓
available implementation resources
       ↓
target capabilities
       ↓
realization

---

59. "Infinity" interpretation

The AST cannot create physically infinite memory or infinitely large machines.

Therefore, production semantics interpret "infinity" architecturally.

The requirement means:

«The AST schema MUST NOT impose an arbitrary finite computational or hardware capacity ceiling.»

Actual execution remains bounded by:

- representable values;
- compiler resources;
- configured operational policies;
- available memory;
- available compute;
- target capabilities;
- physical constraints;
- runtime constraints.

A finite implementation resource limit MUST NOT be presented as a language-level AST limitation.

---

60. Resource exhaustion

An implementation MAY impose operational resource budgets to protect itself.

For example, a compiler invocation MAY configure:

memory budget
time budget
diagnostic budget
AST traversal budget
serialization budget

Such budgets MUST be:

- explicit;
- implementation/configuration-level;
- distinguishable from AST semantics;
- diagnosable;
- non-normative as language capacity.

A traversal budget MUST NOT be interpreted as:

maximum AST size supported by Zamani

---

61. Existing visitor limits

The repository's AST visitor infrastructure already distinguishes operational traversal limits from language semantics.

That distinction MUST remain.

A default such as:

no semantic traversal limit

is compatible with POCO-REAF.

An explicitly configured traversal limit is an execution policy, not an AST schema ceiling.

---

62. AST memory representation

The AST implementation MUST use representations appropriate for the available resources.

It MUST NOT encode a universal fixed array capacity for:

- children;
- statements;
- modules;
- operations;
- declarations;
- qubits;
- expressions;
- fields;
- attributes.

Dynamic or otherwise scalable representations SHOULD be used where the semantic cardinality is unbounded by language design.

---

63. Large-program behavior

The implementation SHOULD support large ASTs without semantic redesign.

Large programs MUST NOT require:

AST schema v_large

merely because they contain more source constructs.

AST version identifies representation structure, not program size.

---

64. Small-target behavior

The same AST schema MUST remain usable when compilation occurs on a constrained environment, subject to configured operational resources.

A constrained target MAY reject compilation because of insufficient resources.

That does not make the AST incompatible.

---

65. Cross-domain AST compatibility

The AST version contract applies across all supported computational domains.

It MUST accommodate:

Classical
Quantum
HDL
Hybrid
AI
Data
Distributed
Networking
Security
Concurrency
Accelerators
Scientific
Future domains

The AST MUST preserve domain-neutral foundations:

values
types
operations
expressions
statements
declarations
effects
capabilities
resources
contracts
policies
provenance

Domain-specific semantic meaning is resolved after AST construction.

---

66. Classical AST compatibility

Classical AST changes MUST preserve:

- expression structure;
- statement structure;
- declaration structure;
- type information;
- generic information;
- ownership-related information;
- effects;
- source provenance.

Changes affecting those structures MUST receive compatibility classification.

---

67. Quantum AST compatibility

Quantum AST changes MUST preserve all information necessary for:

quantum semantic validation
        ↓
quantum::ir

A quantum AST change MUST be checked for:

- operation identity;
- target ordering;
- parameter ordering;
- measurement semantics;
- result relationships;
- modifiers;
- attributes;
- dynamic behavior;
- effects;
- resource requirements;
- capability requirements;
- provenance.

The AST MUST NOT encode a fixed physical quantum machine.

---

68. HDL AST compatibility

HDL AST changes MUST preserve the source-level information needed for:

- hardware intent;
- signals;
- state;
- parameterization;
- timing intent;
- verification;
- simulation;
- synthesis semantics.

Physical realization remains downstream.

---

69. Hybrid AST compatibility

Hybrid constructs MUST preserve the relationship between domains.

For example:

classical control
       ↓
quantum operation
       ↓
measurement
       ↓
classical result
       ↓
decision

An AST migration MUST NOT accidentally break domain boundaries.

---

70. AI and knowledge AST compatibility

Reasoning, knowledge, learning, adaptation, uncertainty, evidence, explanations, decisions and agents MUST use the same AST compatibility framework.

An AST node such as:

reason
learn
adapt
query
assert
retract

must have:

- defined syntax;
- defined AST representation;
- semantic ownership;
- effect contract;
- capability contract where applicable;
- resource contract where applicable;
- policy contract where applicable;
- provenance contract;
- IR mapping;
- compatibility status.

A parser-only AST addition is not production-ready.

---

71. Contracts, policies and provenance

AST representations for:

requires
ensures
invariant
assume
guarantee
property
policy
evidence
provenance

MUST preserve all information required by downstream validation.

The AST MUST NOT reduce these constructs to unstructured text if downstream semantics require structured data.

---

72. Controlled adaptation

If an AST represents adaptation, it MUST preserve the information needed to enforce:

policy
authorization
effect
capability
resource requirements
provenance

An AST representation MUST NOT turn adaptation into unrestricted implicit self-modification.

---

73. Determinism

AST construction MUST be deterministic for the same:

source
language version
grammar contract
dialect configuration
feature configuration

unless explicit nondeterminism is part of the language semantics.

The AST MUST NOT depend on:

- memory addresses;
- process IDs;
- wall-clock timestamps;
- random values;
- target-specific device enumeration;

for its structural identity.

---

74. Reproducibility

For reproducible compilation, the AST pipeline SHOULD preserve enough information to establish:

source identity
language version
grammar version
dialect versions
feature configuration
AST schema version
compiler/toolchain identity

AST schema identity MUST be stable across machines.

---

75. Serialization determinism

Where AST serialization is intended to be reproducible:

- field ordering MUST be deterministic;
- schema metadata MUST be deterministic;
- node ordering MUST be deterministic where semantically required;
- random identifiers MUST NOT be used for schema identity;
- timestamps MUST NOT alter semantic AST identity.

---

76. AST hashing and caching

If AST hashes are used for caching, they MUST be based on a canonical representation.

The hash identity SHOULD include all semantics necessary to distinguish incompatible ASTs.

It MUST NOT depend on:

- memory address;
- pointer identity;
- process-specific state;
- machine-specific layout;
- unstable map ordering.

The cache key SHOULD distinguish at least:

AST schema version
language version
dialect versions
feature configuration
AST content

where required by the cache design.

---

77. AST equality

AST equality MUST define whether it means:

structural equality
source-preserving equality
semantic equality
serialized equality

These are different concepts.

The AST schema MUST NOT imply that structural equality guarantees semantic equivalence unless the semantic contract explicitly proves that relationship.

---

78. AST compatibility and provenance

When an AST is transformed, the implementation SHOULD retain:

origin AST version
destination AST version
transformation identity
source span mapping

This is important for diagnostics and long-lived compiler pipelines.

---

79. Security requirements

AST consumers MUST treat AST input as potentially untrusted when ASTs can be loaded from outside the compiler process.

The decoder MUST:

- validate schema identity;
- validate field structure;
- reject malformed discriminants;
- avoid panics for malformed input;
- avoid unchecked assumptions;
- enforce explicitly configured operational resource budgets;
- avoid code execution during decoding;
- avoid target-specific side effects.

AST deserialization MUST NOT execute:

- native code;
- FFI calls;
- network calls;
- reflection actions;
- adaptation;
- compiler plugins;

merely because those constructs appear in an AST.

---

80. Safe Rust

The production implementation MUST use safe Rust.

The AST versioning implementation MUST NOT require:

unsafe

The repository SHOULD enforce:

#![forbid(unsafe_code)]

where appropriate for AST compatibility/versioning modules.

No AST compatibility feature may depend on:

- raw-pointer manipulation;
- unchecked memory access;
- architecture-specific memory assumptions.

---

81. Rust version independence

AST schema versions MUST NOT be derived from:

rustc version
Cargo version
LLVM version
ANTLR runtime version
operating-system version
CPU architecture

The implementation baseline is:

Rust 1.97 or later
Rust 2021
safe Rust

A Rust toolchain upgrade MUST NOT automatically change the AST schema version.

---

82. ANTLR integration

ANTLR grammar changes MUST be evaluated against AST compatibility.

The path is:

grammar/specification
       ↓
grammar/Zamani.g4
       ↓
ANTLR parse tree
       ↓
AST construction
       ↓
native AST schema

A change to the ANTLR grammar that produces the same AST contract is not necessarily an AST schema change.

A change that requires new or changed AST structure is.

---

83. Rust parser integration

The executable Rust parser MUST produce ASTs conforming to the declared AST schema.

The parser MUST NOT:

- create undocumented AST variants;
- silently discard required source information;
- create target-specific AST fields;
- use a different AST schema interpretation from the ANTLR conformance model.

Rust parser and ANTLR conformance tests SHOULD use common fixtures.

---

84. AST-to-semantic integration

Every stable AST construct MUST have a defined semantic consumer.

The required path is:

AST node
   ↓
structural validation
   ↓
type/effect/capability/resource validation
   ↓
semantic interpretation

A node without semantic ownership MUST NOT be marked stable.

---

85. AST-to-IR integration

Every stable AST construct that has executable semantics MUST have a documented route to canonical semantic representation.

The route MAY pass through an intermediate semantic model.

The required invariant is:

AST
 ↓
semantic model
 ↓
canonical IR

For quantum constructs:

AST
 ↓
quantum semantic model
 ↓
quantum::ir

No competing universal quantum IR may be introduced by the AST compatibility system.

---

86. AST compatibility and optimization

AST versioning occurs before optimization.

Optimization MUST NOT modify the AST schema merely because a backend optimization exists.

For example:

GPU optimization

does not require:

AST GPU version

and:

QPU routing

does not require:

AST physical-QPU version

---

87. AST compatibility and scheduling

Scheduling is downstream.

AST schema compatibility MUST NOT encode:

- fixed timing slots;
- fixed scheduler widths;
- fixed processor counts;
- fixed hardware topology.

Scheduling consumes semantic and resource information after AST construction.

---

88. AST compatibility and resilience

Resilience behavior belongs downstream.

AST may preserve source-level resilience intent such as:

retry
recover
fallback
degraded
policy

where those are language constructs.

The AST MUST NOT encode a particular hardware resilience implementation.

---

89. AST compatibility and QEC

Quantum error correction implementation is downstream.

AST may represent source-level QEC intent where the language specification explicitly supports it.

The AST MUST NOT hard-code:

- a particular code;
- fixed code distance;
- fixed physical-qubit arrangement;
- vendor-specific decoder;
- calibration model.

---

90. Compatibility diagnostics

AST compatibility failures MUST produce structured diagnostics.

A diagnostic SHOULD identify:

error code
producer AST version
consumer supported versions
affected node/field
compatibility classification
required migration
migration availability
source/provenance location when available

Example categories:

AST_VERSION_UNKNOWN
AST_VERSION_UNSUPPORTED
AST_SCHEMA_INCOMPATIBLE
AST_FIELD_UNKNOWN
AST_REQUIRED_FIELD_MISSING
AST_NODE_KIND_UNKNOWN
AST_MIGRATION_REQUIRED
AST_MIGRATION_UNAVAILABLE
AST_EXTENSION_UNSUPPORTED
AST_SCHEMA_CORRUPT

Exact diagnostic identifiers MUST be coordinated with the repository's diagnostics specification.

---

91. No silent fallback

The implementation MUST NOT silently:

- downgrade AST versions;
- upgrade AST versions;
- drop required fields;
- reinterpret unknown nodes;
- replace unknown nodes with generic placeholders;
- change discriminants;
- discard semantic information.

Any such behavior must be an explicitly specified compatibility mechanism.

---

92. Deprecated AST structures

Deprecated AST representations MAY remain supported for migration.

Deprecation MUST identify:

deprecated version
replacement version
migration path
removal policy
compatibility status
tests

Deprecation ownership remains coordinated with:

grammar/compatibility/deprecated.md

---

93. Reserved AST structures

Reserved AST node identities MUST NOT accidentally become active semantics.

Reserved identifiers may be held for future use only through an explicit compatibility/extension policy.

A reserved node identity MUST NOT be reused for a different meaning without a compatibility decision.

---

94. Version negotiation

When an AST producer and consumer have different supported versions, negotiation SHOULD follow:

producer version
       ↓
consumer supported versions
       ↓
compatibility matrix
       ↓
migration availability
       ↓
selected representation

Negotiation MUST be deterministic.

If no safe relationship exists:

reject

not:

guess

---

95. Version ranges

Version ranges MAY be used for compatibility declarations.

However, ranges MUST NOT imply compatibility without the underlying schema contract.

For example:

AST >=1.0,<2.0

is meaningful only if the compatibility matrix confirms that relationship.

Version numbers are identifiers, not proof of compatibility.

---

96. AST compatibility and feature gates

A feature gate MAY require a minimum AST schema version when implementation architecture requires it.

However:

feature gate
     ≠
AST version

The gate controls availability.

The AST version identifies representation.

---

97. AST compatibility and migrations

The authoritative migration procedure is:

grammar/compatibility/migrations.md

This document supplies AST-specific migration requirements.

The migration system MUST maintain a directed compatibility graph:

AST 1.0
   │
   ├──→ AST 1.1
   │
   └──→ AST 2.0

AST 1.1
   └──→ AST 2.0

A migration path MUST be explicit.

---

98. Migration chains

The implementation MAY support:

AST 1.0
 → 1.1
 → 2.0

instead of implementing every direct pair.

However, the migration planner MUST verify:

- semantic preservation;
- deterministic behavior;
- supported intermediate versions;
- provenance;
- error handling.

A migration chain MUST NOT silently apply incompatible transformations.

---

99. Migration idempotence

Where practical:

migrate(AST V)

to the target version SHOULD be idempotent.

Applying the same migration twice MUST NOT corrupt the AST.

---

100. Migration validation

Every migration MUST validate:

Before migration

schema identity
node identities
field validity
extension validity
structural invariants

After migration

target schema identity
node validity
field validity
structural invariants
semantic preservation conditions

A migration that produces an invalid target AST MUST fail.

---

101. AST compatibility tests

Every AST schema release MUST have:

1. exact-version tests;
2. backward-compatibility tests;
3. forward-compatibility tests where promised;
4. migration tests;
5. rejection tests;
6. malformed-input tests;
7. unknown-version tests;
8. unknown-node tests;
9. unknown-field tests;
10. provenance tests;
11. deterministic serialization tests where applicable;
12. source-span preservation tests.

---

102. Required AST test hierarchy

The repository SHOULD maintain:

grammar/tests/ast/
├── schema/
├── versions/
├── compatibility/
├── migrations/
├── serialization/
├── metadata/
├── nodes/
├── source-spans/
├── extensions/
├── domains/
├── determinism/
├── diagnostics/
├── negative/
├── boundary/
├── scalability/
└── cross-domain/

Existing test directories MAY be used instead where equivalent ownership already exists.

The purpose is ownership clarity, not directory proliferation for its own sake.

---

103. Positive tests

Positive tests MUST prove that valid ASTs:

- validate;
- expose their declared schema version;
- preserve required structure;
- reach semantic analysis;
- reach canonical IR where applicable.

---

104. Negative tests

Negative tests MUST prove rejection of:

- unknown major versions;
- unsupported schema versions;
- malformed version values;
- incompatible nodes;
- invalid fields;
- missing required fields;
- invalid discriminants;
- incompatible extensions;
- invalid migrations.

---

105. Boundary tests

Boundary tests MUST include:

AST version transitions
node-family transitions
optional-field transitions
extension transitions
dialect transitions
language-version transitions
grammar-version transitions
serialization-version transitions
semantic-model transitions

---

106. Scalability tests

Scalability tests MUST verify that AST versioning does not impose artificial limits.

Tests SHOULD exercise progressively larger:

- programs;
- module graphs;
- AST node counts;
- declaration counts;
- expression trees;
- quantum operation sequences;
- classical operations;
- HDL structures;
- AI/data structures;
- distributed structures.

The test harness MUST NOT treat a finite test-machine capacity as a language-level AST limit.

---

107. Cross-domain tests

The AST compatibility suite MUST include combinations such as:

classical + quantum
classical + HDL
classical + AI
AI + quantum
AI + distributed
quantum + HDL
quantum + simulation
contracts + provenance
policies + resources
effects + FFI
learning + adaptation
reasoning + knowledge
hybrid + hardware

The purpose is to detect AST structures that work independently but conflict when composed.

---

108. Determinism tests

For the same:

source
language version
grammar version
dialect configuration
feature configuration
compiler compatibility context

AST construction SHOULD produce structurally equivalent ASTs.

Where serialized output is promised deterministic, it MUST be deterministic.

---

109. Compatibility fixtures

Every stable AST schema version SHOULD have canonical fixtures.

For example:

fixtures/ast/v1/
fixtures/ast/v2/

Fixtures MUST be immutable compatibility evidence once released.

A fixture change MUST be treated as a compatibility event.

---

110. Golden AST tests

Golden tests MAY be used for:

- AST structure;
- serialization;
- migration output;
- diagnostics;
- source spans.

Golden files MUST record their schema version explicitly.

A golden fixture without version metadata MUST NOT be used as long-term compatibility evidence.

---

111. AST schema manifest

The implementation SHOULD expose a machine-readable AST schema manifest containing at least:

schema identity
schema version
supported versions
node identities
field identities
extension identities
compatibility relationships
migration identifiers

The manifest MUST be deterministic.

---

112. AST schema fingerprint

A schema fingerprint MAY be generated from the canonical schema manifest.

If implemented, the fingerprint MUST:

- be deterministic;
- exclude machine addresses;
- exclude timestamps;
- exclude random values;
- exclude compiler process identity;
- represent the schema contract rather than a particular target.

A fingerprint MUST NOT replace the human-readable version.

---

113. Public AST API compatibility

The public Rust API of the AST implementation has a separate compatibility dimension from the logical AST schema.

For example:

Rust API compatibility
        ≠
AST schema compatibility

A Rust method may change while serialized AST compatibility remains intact.

Conversely, a stable Rust API can expose a changed AST schema.

Both must be evaluated independently.

---

114. Internal Rust module versions

Individual Rust modules MAY expose schema/API constants.

These MUST NOT be assumed to be the aggregate AST schema version.

The ownership hierarchy is:

aggregate AST schema
      ↓
node-family schema
      ↓
module API/schema

Each layer must declare its meaning.

---

115. AST compatibility and tooling

AST consumers may include:

- compiler;
- formatter;
- IDE;
- debugger;
- linter;
- static analyzer;
- documentation generator;
- refactoring tool;
- migration tool;
- visualization tool;
- provenance tool.

A schema change MUST consider applicable tooling consumers.

---

116. AST compatibility and external tools

External consumers MUST NOT be promised compatibility unless the repository explicitly declares an AST interchange contract.

Internal AST representation and public interchange representation may be different.

If the native AST is exposed externally, its stability requirements MUST be explicitly documented.

---

117. AST ABI separation

AST schema compatibility MUST NOT be confused with ABI compatibility.

AST
 ↓
semantic model
 ↓
IR
 ↓
ABI

ABI is downstream.

Changing an ABI does not automatically change the AST schema.

Changing the AST schema does not automatically change an ABI.

---

118. AST runtime separation

Runtime compatibility is downstream of the AST.

An AST consumer MUST NOT reject an AST merely because the runtime is a different version unless the runtime dependency is explicitly part of the compilation contract.

---

119. AST target independence

An AST MUST remain target-independent.

A target MAY report:

capability unavailable
resource insufficient
unsupported backend

without invalidating the AST itself.

---

120. POCO-REAF requirement

The AST architecture contributes to POCO-REAF by preserving program meaning independently from target realization.

The intended path is:

Zamani source
      ↓
frontend AST
      ↓
semantic model
      ↓
canonical IR
      ↓
target-independent optimization
      ↓
target realization

The AST MUST NOT encode assumptions that force source rewriting when moving between:

tiny target
CPU
multicore
GPU
FPGA
ASIC
accelerator
QPU
simulator
HPC
cluster
distributed system
future target

---

121. AST compatibility and resource negotiation

Resource requirements belong in semantic/resource contracts.

The AST MAY preserve:

requires
prefer
constrain
allow
forbid

where these constructs are part of the language.

But the AST MUST NOT resolve them to physical hardware.

For example:

requires capability("gpu.compute");

is source-level intent.

The AST records it.

Resource analysis determines feasibility.

Target realization determines the concrete resource.

---

122. No physical identity in portable AST

Portable AST nodes MUST NOT require:

physical_cpu_id
physical_gpu_id
physical_fpga_id
physical_qpu_id
physical_qubit_id
physical_node_id

unless a deliberately target-specific dialect explicitly defines such source semantics.

Even then, such constructs MUST remain clearly target-specific rather than contaminating the universal AST model.

---

123. Future-proofing

The AST schema MUST support future domains through extensibility rather than premature enumeration.

The architecture SHOULD prefer:

generic operation
generic domain
qualified identity
attributes
parameters
results
effects
capabilities
resources
extensions

over hard-coded universal enumerations.

This is especially important for:

- future quantum operations;
- future accelerators;
- new computational models;
- new hardware classes;
- new distributed architectures;
- new AI methods;
- new data models.

---

124. What constitutes an AST schema change

The following SHOULD trigger AST compatibility review:

- adding/removing a node;
- changing node identity;
- changing field identity;
- changing field type;
- changing field cardinality;
- changing field optionality;
- changing child relationships;
- changing discriminants;
- changing ordering semantics;
- changing source-span semantics;
- changing extension representation;
- changing required metadata;
- changing structural invariants.

The following do not necessarily change the AST schema:

- compiler optimization;
- target support;
- scheduler implementation;
- routing implementation;
- QEC implementation;
- backend optimization;
- Rust compiler upgrade;
- parser implementation rewrite that produces the same AST contract.

---

125. Compatibility review procedure

Every AST schema change MUST follow:

1. Identify affected node(s)
        ↓
2. Identify structural change
        ↓
3. Identify semantic impact
        ↓
4. Identify serialization impact
        ↓
5. Identify tooling impact
        ↓
6. Classify compatibility
        ↓
7. Select version transition
        ↓
8. Define migration if needed
        ↓
9. Update tests
        ↓
10. Update compatibility matrix
        ↓
11. Validate frontend conformance
        ↓
12. Validate semantic handoff
        ↓
13. Validate canonical IR handoff
        ↓
14. Record provenance

---

126. Required change-control record

Every production AST schema change SHOULD record:

change_id
reason
affected_nodes
old_schema
new_schema
language_impact
grammar_impact
serialization_impact
semantic_impact
IR_impact
tooling_impact
migration
deprecation
tests
compatibility classification
provenance

---

127. Definition of done for an AST schema release

An AST schema version is complete only when all applicable items are satisfied.

Authority

- [ ] AST ownership is explicit.
- [ ] No competing AST authority exists.
- [ ] Specification is updated.
- [ ] Compatibility classification exists.

Structure

- [ ] All affected nodes are specified.
- [ ] Fields are specified.
- [ ] Optionality is specified.
- [ ] Cardinality is specified.
- [ ] Ordering is specified.
- [ ] Discriminants are specified.
- [ ] Source provenance is specified.

Frontend

- [ ] Lexer/parser can construct the AST.
- [ ] ANTLR and Rust frontend agree.
- [ ] Source spans are preserved.
- [ ] Diagnostics are correct.
- [ ] AST coverage passes.

Semantics

- [ ] Semantic owner is identified.
- [ ] Type behavior is defined.
- [ ] Effect behavior is defined.
- [ ] Capability behavior is defined where applicable.
- [ ] Resource behavior is defined where applicable.
- [ ] Contract behavior is defined where applicable.
- [ ] Policy behavior is defined where applicable.
- [ ] Provenance behavior is defined.

IR

- [ ] Canonical semantic mapping exists.
- [ ] Classical constructs reach the canonical classical representation.
- [ ] Quantum constructs reach "quantum::ir".
- [ ] HDL constructs reach their canonical semantic boundary.
- [ ] No duplicate canonical quantum IR exists.

Compatibility

- [ ] Version classification is complete.
- [ ] Migration exists where required.
- [ ] Deprecated representations are tracked.
- [ ] Compatibility matrix is updated.
- [ ] Unknown-version behavior is tested.
- [ ] No silent reinterpretation exists.

Scalability

- [ ] No artificial hardware ceiling exists.
- [ ] No artificial AST cardinality ceiling exists.
- [ ] Operational limits are distinguished from semantic limits.
- [ ] Large-structure tests exist.
- [ ] Small-resource behavior is correctly diagnosed.

Safety

- [ ] Rust 1.97+ compatibility is verified.
- [ ] Rust 2021 is maintained.
- [ ] No Rust "unsafe" is used.
- [ ] Malformed AST input cannot silently execute code.
- [ ] Deserialization is validated.

Testing

- [ ] Positive tests pass.
- [ ] Negative tests pass.
- [ ] Boundary tests pass.
- [ ] Compatibility tests pass.
- [ ] Migration tests pass.
- [ ] Determinism tests pass.
- [ ] Scalability tests pass.
- [ ] Cross-domain tests pass.
- [ ] Diagnostics tests pass.

---

128. Required file contract

This document itself has the following integration contract.

"grammar/compatibility/AST-version.md"

Purpose

Define the aggregate native frontend AST schema version and its compatibility rules.

Owns

- AST schema version;
- AST compatibility classification;
- AST migration requirements;
- AST version diagnostics;
- AST compatibility tests;
- AST version integration.

Does not own

- language version;
- grammar version;
- serialization encoding;
- semantic model;
- IR;
- ABI;
- runtime;
- hardware.

Depends on

grammar/DESIGN.md
grammar/specification/language-version.md
grammar/spec/compatibility.md
grammar/compatibility/versions.md
grammar/compatibility/feature-gates.md
grammar/compatibility/migrations.md
grammar/compatibility/deprecated.md
grammar/compatibility/compatibility-matrix.md
grammar/compatibility/frontend-conformance.md
grammar/validation/ast-coverage.md
src/frontend/ast/

Exports

AST schema identity
AST version semantics
AST compatibility classifications
AST migration requirements
AST version completion criteria

Consumed by

grammar/spec/compatibility.md
grammar/compatibility/compatibility-matrix.md
grammar/compatibility/migrations.md
grammar/compatibility/deprecated.md
grammar/compatibility/frontend-conformance.md
grammar/validation/ast-coverage.md
src/frontend/ast/node/serialization/
compiler compatibility tooling
AST migration tooling
AST conformance tests

AST owner

src/frontend/ast/

Semantic owner

The semantic-analysis subsystem consumes the AST and owns meaning.

IR owner

Canonical IR owners, including:

Classical IR
quantum::ir
HDL/hardware semantic/IR boundary

Test owner

grammar/tests/ast/
grammar/tests/compatibility/

or their existing equivalent repository locations.

Specification owner

grammar/compatibility/AST-version.md

for AST compatibility policy, coordinated with:

grammar/spec/compatibility.md
grammar/specification/language-version.md

---

129. Required companion-file integration

The following files do not need to duplicate this document.

"grammar/specification/language-version.md"

Must continue to own:

language version identity
language version evolution
language-level compatibility

It MUST reference the AST version as a separate layer.

---

"grammar/compatibility/versions.md"

Must continue to own the repository-wide version taxonomy.

It MUST classify:

language
grammar
AST
semantic
IR
compiler
artifact
ABI
runtime
target
dialect

as distinct version dimensions.

---

"grammar/spec/compatibility.md"

Must consume this AST contract as the AST compatibility authority.

It MUST NOT redefine AST version numbers.

---

"grammar/compatibility/compatibility-matrix.md"

Must record actual AST compatibility relationships.

It MUST NOT infer compatibility from numeric similarity alone.

---

"grammar/compatibility/migrations.md"

Must own the operational migration process.

This document supplies the AST-specific migration requirements.

---

"grammar/compatibility/deprecated.md"

Must own the lifecycle of deprecated AST representations where applicable.

---

"grammar/compatibility/feature-gates.md"

Must own feature availability.

It may refer to minimum/required AST schema support but MUST NOT become an AST-version authority.

---

"grammar/compatibility/frontend-conformance.md"

Must verify:

source
 ↓
lexer
 ↓
parser
 ↓
AST

and ensure both parser implementations conform to the same AST contract.

---

"grammar/validation/ast-coverage.md"

Must verify:

grammar
 ↓
AST
 ↓
semantic model
 ↓
IR

and must report missing AST mappings.

---

"grammar/grammar.md"

Must report AST implementation status.

It MUST NOT define AST compatibility independently.

---

"grammar/DESIGN.md"

Remains the top-level architecture authority.

This document implements its AST-layer requirements.

---

130. Required implementation alignment

The existing Rust AST implementation already contains separate concepts for:

AST schema version
serialization schema/version
node-local schema versions
visitor schema version
metadata

Those distinctions MUST be preserved.

The implementation MUST NOT collapse them into:

AST_VERSION

as a universal version number.

Instead, the conceptual model is:

language_version
        │
grammar_version
        │
ast_schema_version
        │
├── node_schema_versions
├── visitor_schema_version
├── metadata_schema_version
└── serialization_schema_version
        │
serialization_format_version
        │
semantic_model_version
        │
IR versions

---

131. Recommended canonical metadata model

Where AST metadata needs version information, the conceptual contract SHOULD be:

AstMetadata
├── language_version
├── grammar_version
├── ast_schema_version
├── serialization_schema_version
├── serialization_format_version
├── compiler_version
├── dialect_versions
├── feature_configuration
└── provenance

Not every field must exist in every AST representation.

However, if a field exists, its ownership and semantics MUST be explicit.

---

132. Recommended machine-readable compatibility record

A compatibility record SHOULD conceptually contain:

producer:
  ast_schema: <version>

consumer:
  supported_ast_schema:
    - <version/range>

compatibility:
  classification: <classification>

migration:
  required: <true|false>
  identity: <migration-id>

provenance:
  source: <origin>

The actual serialization format belongs to the implementation.

---

133. Production AST compatibility pipeline

The production pipeline is:

Zamani source
        │
        ▼
language version resolution
        │
        ▼
grammar version resolution
        │
        ▼
lexer
        │
        ▼
parser
        │
        ▼
frontend AST
        │
        ▼
AST schema validation
        │
        ├── version validation
        ├── structural validation
        ├── source-span validation
        └── extension validation
        │
        ▼
semantic analysis
        │
        ├── type checking
        ├── effect checking
        ├── capability checking
        ├── resource checking
        ├── contract checking
        ├── policy checking
        └── provenance
        │
        ▼
canonical semantic model
        │
        ├── Classical
        ├── Quantum
        ├── HDL
        ├── AI
        ├── Data
        ├── Distributed
        └── Hybrid
        │
        ▼
canonical IR
        │
        ├── Classical IR
        └── quantum::ir
        │
        ▼
optimization
        │
        ▼
lowering
        │
        ▼
routing
        │
        ▼
scheduling
        │
        ▼
resilience / QEC / ZQN
        │
        ▼
HAL
        │
        ▼
target

The AST compatibility contract applies only to the AST boundary.

---

134. Production readiness criteria

"grammar/compatibility/AST-version.md" is production-ready only when:

1. AST schema identity is unambiguous.
2. AST schema version is independent from language version.
3. AST schema version is independent from grammar version.
4. AST schema version is independent from serialization format.
5. AST schema version is independent from node-local versions.
6. AST schema version is independent from visitor schema version.
7. AST schema version is independent from compiler version.
8. AST schema version is independent from semantic-model version.
9. AST schema version is independent from IR versions.
10. AST schema version is independent from ABI/runtime/target versions.
11. Compatibility classifications are explicit.
12. Unknown versions are rejected safely.
13. Silent reinterpretation is prohibited.
14. Migration requirements are explicit.
15. Deprecation integration is explicit.
16. Feature-gate integration is explicit.
17. AST coverage integration is explicit.
18. Frontend conformance integration is explicit.
19. Serialization integration is explicit.
20. Classical integration is explicit.
21. Quantum integration is explicit.
22. "quantum::ir" remains the canonical quantum boundary.
23. HDL integration is explicit.
24. Hybrid integration is explicit.
25. AI/data integration is explicit.
26. Resource/capability separation is explicit.
27. No universal resource ceilings exist.
28. Operational resource limits remain distinct from language capacity.
29. Rust 1.97+ compatibility is maintained.
30. Rust 2021 is maintained.
31. Rust "unsafe" is prohibited.
32. Positive tests exist.
33. Negative tests exist.
34. Boundary tests exist.
35. Migration tests exist.
36. Compatibility tests exist.
37. Determinism tests exist.
38. Scalability tests exist.
39. Cross-domain tests exist.
40. Diagnostics are structured.
41. Provenance is preserved.
42. The AST can evolve without unnecessarily breaking source semantics.
43. Every public dependency and integration boundary is documented.
44. No second AST authority has been introduced.

---

135. Final invariant

The most important AST invariant is:

Zamani source meaning
        ↓
     frontend AST
        ↓
semantic interpretation
        ↓
canonical representation
        ↓
target realization

The AST is the stable structural bridge between source and semantics.

It MUST NOT become:

source
 ↓
hardware-specific AST
 ↓
vendor-specific AST
 ↓
target-specific semantics

The correct architecture is:

                 ZAMANI SOURCE
                       │
                       ▼
              Language / Grammar
                       │
                       ▼
               Frontend AST
                       │
          ┌────────────┼────────────┐
          │            │            │
          ▼            ▼            ▼
      Classical      Quantum       HDL
          │            │            │
          │            ▼            │
          │       quantum::ir       │
          │            │            │
          └────────────┼────────────┘
                       │
                       ▼
                Semantic Model
                       │
          ┌────────────┼────────────┐
          │            │            │
          ▼            ▼            ▼
     Classical IR quantum::ir HDL/Hardware
          │            │            │
          └────────────┼────────────┘
                       │
                       ▼
                 Optimization
                       │
                       ▼
                    Lowering
                       │
                       ▼
              Routing / Scheduling
                       │
                       ▼
             Resilience / QEC / ZQN
                       │
                       ▼
                      HAL
                       │
          ┌────────────┼────────────┐
          ▼            ▼            ▼
         CPU          GPU          FPGA
          │            │            │
          ├────────────┼────────────┤
          ▼            ▼            ▼
         ASIC      Accelerator      QPU
          │            │            │
          └────────────┼────────────┘
                       ▼
              HPC / Distributed
                       │
                       ▼
                 Future Targets

The AST schema version exists to protect the integrity of the frontend structural boundary while allowing every downstream representation to evolve independently.

The central rule is therefore:

«Change the AST schema only when the AST structural contract changes. Change the language version only when language meaning changes. Change the grammar version only when the grammar representation changes. Change the serialization version only when serialization changes. Change the IR version only when the IR contract changes. Never use one version number as a substitute for another.»

This separation is what allows Zamani to evolve indefinitely while preserving source meaning, POCO-REAF, cross-domain compilation, explicit compatibility, safe Rust implementation, and scalability from the smallest practical computation to computations limited only by their actual semantics and available resources.
Zamani Semantic Versioning Specification

File: "grammar/compatibility/semantic-version.md"
Status: Normative
Scope: Semantic-contract versioning across the Zamani language pipeline
Language: Zamani
Repository: "github.com/Benwellonedge28/Zamani/"
Rust baseline: Rust 1.97.1 or later
Rust edition: 2021
Rust safety requirement: Production Rust implementation MUST use safe Rust; Rust "unsafe" MUST NOT be required or used
Primary architectural objective: Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)

---

1. Purpose

This document defines the normative semantic-versioning contract for Zamani.

It specifies how semantic contracts evolve, how semantic compatibility is classified, how incompatible semantic changes are detected, how migrations are represented, and how semantic-version information integrates with:

- language versions;
- grammar versions;
- lexical versions;
- frontend AST versions;
- semantic-model versions;
- type-system versions;
- effect-system versions;
- capability versions;
- resource-contract versions;
- policy versions;
- provenance versions;
- canonical IR versions;
- "quantum::ir";
- classical semantic representations;
- HDL/hardware semantic representations;
- ABI versions;
- runtime versions;
- dialect versions;
- compiler versions;
- artifact versions;
- target versions;
- migration versions;
- deprecation lifecycle;
- feature gates;
- compatibility matrices;
- reproducibility;
- diagnostics;
- conformance testing.

This document does not define the meaning of individual language constructs.

It defines how an already-defined semantic contract is versioned.

The fundamental distinction is:

«Language versioning identifies the language contract. Semantic versioning identifies the compatibility state of a semantic contract within that language architecture.»

These concepts MUST NOT be collapsed into one version number.

---

2. Normative terminology

The following terms are normative:

- MUST / SHALL — mandatory.
- MUST NOT / SHALL NOT — prohibited.
- SHOULD — recommended unless a documented architectural reason exists otherwise.
- SHOULD NOT — discouraged unless explicitly justified.
- MAY — permitted.
- REQUIRED — mandatory.
- OPTIONAL — permitted but not required.

The following terms have specific meanings in this document.

2.1 Semantic contract

A semantic contract defines the meaning and compatibility obligations of a computational construct or semantic subsystem after syntax has been parsed.

A semantic contract may define:

- types;
- values;
- operations;
- evaluation;
- ownership;
- effects;
- capabilities;
- resources;
- contracts;
- policies;
- provenance;
- concurrency;
- distributed behavior;
- classical computation;
- quantum computation;
- HDL intent;
- hybrid computation;
- interoperability behavior.

2.2 Semantic version

A semantic version identifies a particular compatibility contract for a semantic subsystem.

2.3 Semantic version domain

A semantic version domain is the explicitly named semantic subsystem being versioned.

Examples include:

- "core.semantics";
- "types.semantics";
- "effects.semantics";
- "resources.semantics";
- "quantum.semantics";
- "quantum.ir";
- "hdl.semantics";
- "concurrency.semantics";
- "distributed.semantics";
- "ai.semantics";
- "interoperability.semantics".

A semantic version MUST always identify its domain.

2.4 Semantic compatibility

Semantic compatibility means that two versions preserve the specified meaning and guarantees required by the compatibility contract being compared.

2.5 Semantic equivalence

Two representations are semantically equivalent when the applicable specification establishes that they produce equivalent observable behavior for the relevant semantic domain and contract.

2.6 Semantic break

A semantic break is a change that invalidates a previously promised semantic guarantee or changes the specified meaning of a previously compatible construct.

2.7 Semantic migration

A semantic migration is an explicit, specified transformation between semantic-contract versions.

2.8 Semantic feature

A semantic feature is a named language capability whose meaning is defined independently of a particular hardware target.

---

3. Authority

This document does not create a second language specification.

The repository authority relationship is:

grammar/DESIGN.md
        │
        ▼
grammar/specification/
        │
        ├── language specification
        ├── language-version specification
        └── domain semantic specifications
        │
        ▼
grammar/spec/
        │
        ├── semantic contracts
        ├── compatibility contracts
        ├── type contracts
        ├── effect contracts
        ├── resource contracts
        └── domain contracts
        │
        ▼
grammar/compatibility/
        │
        ├── versions.md
        ├── semantic-version.md
        ├── feature-gates.md
        ├── compatibility-matrix.md
        ├── frontend-conformance.md
        ├── migrations.md
        ├── deprecated.md
        └── reserved.md
        │
        ▼
grammar/Zamani.g4
        │
        ▼
lexer / parser
        │
        ▼
frontend AST
        │
        ▼
semantic analysis
        │
        ▼
canonical semantic representation
        │
        ├── classical semantics
        ├── quantum semantics
        ├── HDL/hardware semantics
        ├── AI semantics
        └── other domain semantics
        │
        ▼
canonical IR
        │
        ├── classical IR
        ├── quantum::ir
        └── domain-specific canonical IR boundaries
        │
        ▼
optimization / lowering
        │
        ├── routing
        ├── scheduling
        ├── resilience
        ├── QEC
        └── ZQN
        │
        ▼
HAL / runtime / target realization

Each layer owns its own contract.

No lower layer MAY silently redefine a semantic contract owned by a higher layer.

---

4. Ownership of this file

4.1 This file owns

"grammar/compatibility/semantic-version.md" owns:

- semantic-version terminology;
- semantic-version identity;
- semantic-version syntax;
- semantic-version domains;
- semantic-version compatibility classes;
- semantic-version comparison;
- semantic-version bump rules;
- semantic compatibility guarantees;
- semantic break classification;
- semantic compatibility metadata;
- semantic-version negotiation;
- semantic-version validation;
- semantic-version migration references;
- semantic-version/deprecation interaction;
- semantic-version/feature-gate interaction;
- semantic-version reproducibility requirements;
- semantic-version diagnostics;
- semantic-version testing requirements.

4.2 This file does not own

This file does NOT own:

- language version numbering;
- concrete grammar rules;
- lexical token definitions;
- AST structure definitions;
- individual semantic meanings;
- canonical IR definitions;
- "quantum::ir" internal representation;
- ABI implementation;
- runtime implementation;
- target discovery;
- hardware topology;
- routing algorithms;
- scheduling algorithms;
- QEC algorithms;
- ZQN implementation;
- calibration;
- compiler optimization algorithms;
- physical-resource limits.

Those remain owned by their respective specifications.

---

5. Relationship to existing versioning documents

The semantic-versioning system MUST remain separate from the other version domains.

File / domain| Owns
"grammar/specification/language-version.md"| Language identity and language-version meaning
"grammar/compatibility/versions.md"| Repository-wide version taxonomy and compatibility policy
"grammar/compatibility/semantic-version.md"| Semantic-contract versioning
"grammar/compatibility/feature-gates.md"| Feature availability and gate lifecycle
"grammar/compatibility/migrations.md"| Migration procedures
"grammar/compatibility/deprecated.md"| Deprecation lifecycle
"grammar/compatibility/compatibility-matrix.md"| Cross-layer compatibility relationships
"grammar/compatibility/frontend-conformance.md"| Source → token → parse tree → AST conformance
"grammar/compatibility/reserved.md"| Reserved syntax and namespace space
"grammar/spec/compatibility.md"| Normative cross-layer compatibility contract
"grammar/validation/compatibility-rules.md"| Compatibility validation
"grammar/grammar.md"| Implementation/conformance status
"grammar/DESIGN.md"| Architecture and ownership boundaries

No file may use the term “semantic version” to mean a different version domain without an explicit qualification.

---

6. Why semantic versioning is separate

Zamani contains multiple independently evolving contracts.

For example:

Language
Grammar
Lexer
AST
Semantic Model
Type System
Effect System
Resource Model
Policy Model
Provenance Model
Classical IR
quantum::ir
HDL IR
ABI
Runtime
Dialect
Compiler
Artifact
Target

A change to one does not necessarily imply a change to all others.

For example:

compiler implementation update

does not automatically require:

language semantic-version change

Similarly:

new GPU backend

does not automatically require:

language version change

and:

new QPU topology

does not automatically require:

quantum semantic-version change

unless the language semantics themselves change.

---

7. Semantic version identity

Every independently versioned semantic contract MUST have:

semantic-domain
semantic-version
compatibility-status

Conceptually:

<semantic-domain>@<MAJOR>.<MINOR>.<PATCH>

For example:

core.semantics@1.2.0
types.semantics@2.0.0
quantum.semantics@1.4.1
quantum.ir@3.0.0

These examples are illustrative identifiers, not claims about current repository versions.

The repository MUST maintain one canonical registry for actual versions.

A semantic version MUST NOT be interpreted without its semantic domain.

Therefore:

1.2.0

by itself is insufficient to identify a semantic contract.

---

8. Semantic version format

The canonical semantic version format is:

MAJOR.MINOR.PATCH

An implementation MAY additionally support explicitly specified pre-release and build metadata:

MAJOR.MINOR.PATCH-PRERELEASE
MAJOR.MINOR.PATCH+BUILD
MAJOR.MINOR.PATCH-PRERELEASE+BUILD

Pre-release and build metadata MUST NOT silently alter the semantic meaning of the stable version.

Build metadata MUST NOT be used to conceal semantic incompatibility.

---

9. Semantic version components

9.1 MAJOR

Increment MAJOR when a previously stable semantic contract becomes incompatible.

Examples:

- changing the meaning of a stable operation;
- changing type compatibility incompatibly;
- changing ownership semantics incompatibly;
- changing effect meaning incompatibly;
- changing concurrency guarantees incompatibly;
- changing quantum measurement semantics incompatibly;
- changing quantum state semantics incompatibly;
- changing HDL timing semantics incompatibly;
- changing distributed consistency guarantees incompatibly;
- changing resource requirement meaning incompatibly;
- changing capability meaning incompatibly;
- changing policy interpretation incompatibly;
- changing provenance guarantees incompatibly.

A major semantic version MUST identify the affected semantic domain.

9.2 MINOR

Increment MINOR when a semantic contract is extended compatibly.

Examples:

- adding a new semantic capability;
- adding a new operation whose interaction with existing semantics is backward-compatible;
- adding new optional metadata;
- adding a new capability class;
- adding a new resource-expression form without changing existing interpretation;
- adding a new semantic domain extension behind an explicit feature mechanism;
- adding additional provenance information while preserving existing required fields.

A minor semantic change MUST NOT change the established meaning of existing stable constructs.

9.3 PATCH

Increment PATCH for compatible corrections.

Examples:

- correcting an implementation mismatch with an unchanged specification;
- correcting an incomplete diagnostic;
- correcting metadata;
- correcting a serialization defect without changing semantic interpretation;
- correcting deterministic behavior while preserving specified results;
- correcting documentation that does not alter the contract;
- correcting an incorrectly classified compatibility record.

A patch change MUST NOT change stable semantic meaning.

---

10. Semantic-version decision rule

When evaluating a change, the following order MUST be used:

Did the specified meaning of an existing stable construct change?
        │
       YES
        │
        ▼
     MAJOR
        │
       NO
        ▼
Did the compatibility contract gain new behavior without
changing existing stable meaning?
        │
       YES
        │
        ▼
     MINOR
        │
       NO
        ▼
Is the change a compatible correction?
        │
       YES
        │
        ▼
     PATCH

If the classification is unclear, the change MUST NOT be silently classified as PATCH.

The change MUST undergo compatibility review.

---

11. Semantic compatibility dimensions

Semantic compatibility MUST be evaluated independently for:

1. value semantics;
2. operation semantics;
3. evaluation semantics;
4. type semantics;
5. ownership semantics;
6. lifetime semantics;
7. effect semantics;
8. capability semantics;
9. resource semantics;
10. contract semantics;
11. policy semantics;
12. provenance semantics;
13. concurrency semantics;
14. distributed semantics;
15. classical numerical semantics;
16. quantum state semantics;
17. quantum measurement semantics;
18. quantum dynamic-control semantics;
19. HDL timing semantics;
20. hardware-intent semantics;
21. interoperability semantics;
22. error/failure semantics;
23. determinism semantics;
24. reproducibility semantics;
25. security semantics;
26. adaptation semantics;
27. simulation semantics.

A semantic-version claim MUST identify which dimensions it covers.

---

12. Semantic compatibility classes

The repository SHOULD use explicit compatibility classes.

At minimum:

SEMANTIC_EXACT
SEMANTIC_BACKWARD_COMPATIBLE
SEMANTIC_FORWARD_COMPATIBLE
SEMANTIC_EXTENDED
SEMANTIC_MIGRATABLE
SEMANTIC_DEPRECATED
SEMANTIC_INCOMPATIBLE
SEMANTIC_UNKNOWN
SEMANTIC_UNSUPPORTED

12.1 SEMANTIC_EXACT

The two versions define the same semantic contract for the compared feature set.

12.2 SEMANTIC_BACKWARD_COMPATIBLE

Programs conforming to the older contract retain their specified meaning under the newer contract.

12.3 SEMANTIC_FORWARD_COMPATIBLE

A newer semantic representation can be consumed by an older implementation without losing or changing required semantics.

This MUST NOT be claimed merely because unknown data is ignored.

12.4 SEMANTIC_EXTENDED

The newer contract adds semantic capabilities without invalidating existing stable semantics.

12.5 SEMANTIC_MIGRATABLE

The contracts are not directly compatible, but an explicit semantics-preserving or explicitly declared semantics-changing migration exists.

12.6 SEMANTIC_DEPRECATED

The older contract remains recognized but is scheduled for removal or replacement.

12.7 SEMANTIC_INCOMPATIBLE

The contracts cannot safely interoperate without changing semantics.

12.8 SEMANTIC_UNKNOWN

The relationship has not been established.

Unknown MUST NOT be treated as compatible.

12.9 SEMANTIC_UNSUPPORTED

The implementation does not provide the required semantic contract.

Unsupported MUST NOT be converted into successful compatibility.

---

13. Fundamental invariant

The fundamental semantic compatibility invariant is:

«A compatible implementation MUST NOT silently change the specified meaning of valid source programs or valid semantic artifacts.»

If a semantic change is intentional, it MUST be:

1. identified;
2. classified;
3. versioned;
4. documented;
5. diagnosable;
6. tested;
7. migratable where practical;
8. represented in compatibility metadata.

No compiler optimization, backend limitation, runtime behavior, or target limitation may silently override this rule.

---

14. No silent reinterpretation

The following are prohibited:

old semantic meaning
        ↓
new compiler
        ↓
different meaning
        ↓
no diagnostic

Instead:

old semantic contract
        ↓
compatibility resolver
        ↓
compatible
        │
        └──> continue

or

incompatible
        │
        ├──> migrate explicitly
        ├──> select declared compatibility mode
        └──> reject with diagnostic

A parser accepting old syntax does not establish semantic compatibility.

---

15. Source compatibility versus semantic compatibility

The following MUST be distinguished:

source-compatible

and:

semantic-compatible

A program may parse successfully while its meaning changes.

Therefore:

successful parsing
≠
semantic compatibility

Similarly:

AST structural compatibility
≠
semantic compatibility

and:

IR structural compatibility
≠
semantic compatibility

Semantic equivalence MUST be established by the semantic contract.

---

16. Language-version integration

"grammar/specification/language-version.md" owns the language version.

Semantic versions operate beneath or alongside the language-version contract.

The relationship is:

Language Version
       │
       ├── lexical contract
       ├── grammar contract
       ├── semantic contracts
       ├── type contracts
       ├── effect contracts
       ├── resource contracts
       └── compatibility guarantees

A language release MAY contain multiple semantic-contract versions.

Conversely, a semantic subsystem MAY evolve independently where the language contract permits it.

A semantic version MUST NOT be substituted for the language version in source-level language declarations.

---

17. Grammar-version integration

Grammar compatibility and semantic compatibility are different.

For example:

grammar version changes

may be caused by:

- parser refactoring;
- grammar modularization;
- ambiguity correction;
- parse-tree restructuring.

If the resulting AST and semantic meaning remain unchanged, the semantic version MAY remain unchanged.

Conversely:

grammar unchanged

does not guarantee semantic stability.

A semantic change can occur through:

- semantic analysis;
- type rules;
- effect rules;
- evaluation rules;
- canonical semantic lowering.

Therefore:

grammar compatibility
≠
semantic compatibility

---

18. AST integration

The frontend AST is a structural contract.

Semantic versioning MUST distinguish:

AST representation change

from:

semantic meaning change

An AST field may be reorganized without changing source semantics.

Conversely, an unchanged AST structure can acquire incompatible semantics.

Every semantic-version change affecting AST interpretation MUST identify:

AST node
field
semantic interpretation
compatibility impact
migration

The AST MUST preserve enough information to implement the declared semantic contract.

Semantic meaning MUST NOT be reconstructed from ambiguous or discarded syntax.

---

19. Semantic-model integration

The semantic model is the primary consumer of this document.

A semantic model MUST declare:

semantic-domain
semantic-version
supported-language-versions
dependencies
provided-contracts
consumed-contracts
compatibility
migration
deprecation-status

A semantic implementation MUST NOT claim a semantic version for which its implementation and conformance suite are incomplete.

---

20. Type-system integration

Type semantics are independently compatibility-sensitive.

Examples include:

- primitive types;
- generic types;
- bounds;
- associated types;
- linear types;
- affine types;
- dependent constraints;
- function types;
- references;
- arrays;
- slices;
- maps;
- tensor types;
- uncertainty types;
- probability types;
- distribution types;
- quantum types;
- hardware-oriented types.

A change that alters the meaning of a stable type relationship MUST be classified as a semantic compatibility change.

Type syntax alone does not determine compatibility.

---

21. Effect-system integration

Effect semantics MUST participate in semantic versioning.

Examples include:

io
network
mutation
randomness
native
foreign
distributed
measurement
quantum
learning
adaptation
reflection
code_generation
simulation

Changing the meaning of an existing stable effect can be a semantic MAJOR change.

For example:

effect(network)

MUST NOT silently acquire broader privileges in a newer semantic version.

Effect compatibility MUST remain explicit.

---

22. Capability integration

Capabilities describe what an execution environment can provide.

Examples:

capability("quantum.measurement")
capability("gpu.compute")
capability("tensor.compute")

Capability identifiers are not semantic-version identifiers.

A capability MAY have its own versioned contract.

For example:

capability-contract:
    gpu.compute@1.x

does not mean:

language@1.x

Capability versions MUST NOT alter the meaning of the language silently.

---

23. Resource integration

Resource requirements are separate from semantic versions.

Valid portable intent may include:

requires qubits >= n;
requires memory >= required_memory;
requires capability("quantum.measurement");
requires capability("gpu.compute");
requires capability("tensor.compute");
requires topology(required_topology);

These requirements describe program needs.

They do not define a semantic-version ceiling.

The semantic-version system MUST NOT encode:

MAX_QUBITS
MAX_CPUS
MAX_GPUS
MAX_FPGAS
MAX_NODES
MAX_MEMORY
MAX_THREADS
MAX_REGISTER_WIDTH
MAX_TENSOR_RANK
MAX_NETWORK_SIZE
MAX_DEVICE_COUNT

or equivalent universal limits.

---

24. POCO-REAF requirement

Semantic versioning MUST preserve POCO-REAF.

The semantic contract describes:

what computation means

rather than:

how one particular machine realizes it

Therefore semantic compatibility MUST NOT depend on:

- CPU count;
- GPU count;
- FPGA count;
- QPU count;
- physical qubit numbering;
- memory capacity;
- register width;
- node count;
- cluster size;
- topology size;
- device count;
- accelerator count.

A target MAY reject a program because it cannot satisfy its declared requirements.

That is a target/resource failure, not automatically a semantic-version failure.

---

25. Quantum semantic versioning

Quantum semantics MUST have an explicit semantic-version contract.

The canonical relationship is:

Zamani source
      ↓
frontend AST
      ↓
quantum semantic model
      ↓
quantum::ir
      ↓
optimization
      ↓
routing
      ↓
scheduling
      ↓
QEC / ZQN
      ↓
HAL

"quantum::ir" remains the canonical quantum semantic boundary.

This document MUST NOT introduce another quantum IR.

---

26. Quantum semantic compatibility

Quantum semantic compatibility includes, where applicable:

- state semantics;
- qubit semantics;
- register semantics;
- operation semantics;
- parameter semantics;
- measurement semantics;
- reset semantics;
- dynamic control;
- classical feed-forward;
- channels;
- noise;
- observables;
- resource requirements;
- error-correction intent;
- fault-tolerance semantics;
- hybrid classical/quantum control.

Changing the meaning of any stable quantum semantic construct MUST be versioned explicitly.

---

27. Quantum operation extensibility

Quantum semantic versioning MUST NOT require a universal fixed list of operations.

The grammar may represent operations through data-driven concepts such as:

operationSpecifier
targets
parameters
results
attributes
modifiers

New quantum operations may be supplied through controlled metadata or dialect mechanisms.

Adding a new operation that does not alter existing operation semantics MAY be a MINOR semantic change.

Changing the meaning of an existing stable operation requires MAJOR classification for the affected semantic domain.

---

28. Classical semantic versioning

Classical semantics include, where applicable:

- numerical behavior;
- control flow;
- function calls;
- memory semantics;
- ownership;
- concurrency;
- deterministic evaluation;
- exceptions/errors;
- generic computation;
- symbolic computation.

Changes affecting stable observable behavior MUST be classified semantically even if the grammar remains unchanged.

---

29. HDL and hardware semantic versioning

HDL/hardware semantics MUST distinguish:

hardware intent

from:

physical realization

Semantic versioning may cover:

- signal semantics;
- timing semantics;
- state semantics;
- interface semantics;
- protocol semantics;
- memory semantics;
- parameterization;
- verification semantics;
- synthesis intent.

It MUST NOT version a physical device merely because the semantic model can target it.

A new FPGA, ASIC, accelerator, or physical topology is normally a target/backend concern.

---

30. Distributed semantic versioning

Distributed semantics may include:

- actors;
- messages;
- channels;
- ordering;
- consistency;
- fault semantics;
- failure handling;
- distributed transactions;
- topology-independent intent.

Changing the meaning of message ordering or consistency guarantees can be a MAJOR semantic change.

Changing the physical cluster size is not inherently a semantic-version change.

---

31. AI and reasoning semantic versioning

AI-related semantics MAY include:

- inference;
- reasoning;
- deduction;
- induction;
- abduction;
- knowledge;
- learning;
- adaptation;
- uncertainty;
- probability;
- confidence;
- evidence;
- provenance;
- explanation;
- decisions;
- neural-symbolic composition;
- agent semantics.

These are semantic capabilities of the language.

They MUST be versioned according to their semantic contracts.

Application-specific domains remain libraries, dialects, capabilities, policies, or external services rather than becoming universal semantic-version categories.

---

32. Adaptation semantics

Adaptation is especially compatibility-sensitive.

The semantic contract MUST distinguish:

controlled adaptation

from:

unrestricted semantic reinterpretation

An adaptation operation MAY require:

- policy;
- capability;
- effect;
- resource requirements;
- authorization;
- provenance;
- validation.

A newer implementation MUST NOT silently broaden adaptation authority.

Any change to the guarantees surrounding adaptation requires explicit semantic compatibility analysis.

---

33. Contract semantics

The semantic-version system MUST account for:

requires
ensures
invariant
assume
guarantee
property
assertion
precondition
postcondition
refinement
proof
evidence

Changing whether a contract is:

- checked;
- assumed;
- guaranteed;
- advisory;
- runtime-enforced;
- compile-time-enforced

can be a semantic compatibility change.

Contract enforcement behavior MUST therefore be explicitly versioned.

---

34. Policy semantics

Policies may constrain:

- resources;
- capabilities;
- execution;
- security;
- adaptation;
- simulation;
- deployment;
- networking;
- interoperability.

A policy-version change MUST NOT silently broaden permissions.

For example:

forbid effect("network")

MUST continue to prohibit the corresponding effect under a compatible semantic contract.

---

35. Provenance semantics

Provenance semantics may include:

source
derived_from
generated_by
transformed_by
verified_by
reason
evidence
decision
version
timestamp

Changing the meaning of required provenance fields is semantic-version-sensitive.

Provenance metadata MUST NOT be silently discarded when the semantic contract requires it.

---

36. Determinism

Semantic versioning MUST account for determinism.

If a stable semantic contract guarantees deterministic behavior, a new semantic version MUST NOT silently introduce nondeterminism.

If nondeterminism is intentionally introduced, the compatibility contract MUST explicitly state:

- source of nondeterminism;
- effect classification;
- reproducibility implications;
- seed/control mechanism where applicable;
- migration requirements.

---

37. Reproducibility

A semantic version MUST be sufficient to identify the semantic contract needed to reproduce a compilation or semantic interpretation.

A reproducibility record SHOULD identify, where applicable:

language version
grammar version
semantic-domain versions
AST version
IR versions
dialect versions
compiler version
feature-gate state
policy versions
resource/capability assumptions
target contract

A compiler MUST NOT claim reproducibility solely because the same compiler binary was used.

---

38. Compiler-version integration

Compiler versions are independent from semantic versions.

A compiler MAY support:

semantic-domain@1.x
semantic-domain@2.x

simultaneously.

Compiler implementation version MUST NOT be used as a substitute for semantic compatibility.

The compiler MUST expose unsupported semantic contracts through structured diagnostics.

---

39. Runtime-version integration

Runtime compatibility is separate.

A semantic contract may be stable while a runtime changes.

For example:

semantic contract
    ↓
runtime implementation A

and:

semantic contract
    ↓
runtime implementation B

may both be valid.

A runtime MAY reject an artifact because it requires an unavailable runtime contract.

It MUST NOT silently change semantic meaning.

---

40. ABI-version integration

ABI compatibility is distinct from semantic compatibility.

The relationship is:

Zamani semantic model
        ↓
foreign/interoperability contract
        ↓
ABI
        ↓
target/runtime

A changed ABI MUST NOT be hidden behind an unchanged semantic-version claim.

FFI and foreign calls MUST carry the appropriate:

- effect;
- capability;
- ABI;
- data-layout;
- calling-convention;
- provenance;
- compatibility metadata.

---

41. Dialect-version integration

Dialects MUST have explicit identities.

Conceptually:

dialect-name@version

A dialect MUST declare:

owner
namespace
version
language-version requirements
semantic dependencies
grammar dependencies
AST mapping
semantic mapping
IR mapping
compatibility policy
migration policy
deprecation policy

A dialect MUST NOT silently redefine core Zamani semantics.

---

42. Feature-gate integration

Feature gates answer:

Is this defined feature enabled here?

Semantic versions answer:

What semantic contract does this feature implement?

These MUST remain separate.

The relationship is:

feature identity
      ↓
feature gate
      ↓
semantic contract
      ↓
semantic version

A feature gate MUST NOT become an alternate semantic-version mechanism.

---

43. Migration integration

A semantic break MUST reference:

grammar/compatibility/migrations.md

when migration is available.

A migration MUST specify:

- source semantic version;
- destination semantic version;
- affected constructs;
- transformation;
- semantic-preservation guarantee;
- known semantic differences;
- diagnostics;
- limitations;
- test cases;
- rollback/rejection behavior.

A migration MUST NOT claim semantic preservation when it merely makes source syntax parse.

---

44. Deprecation integration

Deprecation MUST be explicit.

The relationship is:

STABLE
   ↓
DEPRECATED
   ↓
REMOVAL-ELIGIBLE
   ↓
REMOVED

A deprecated semantic contract MUST retain enough metadata to determine:

- original semantic version;
- replacement;
- migration path;
- deprecation status;
- removal policy.

Removal of a stable semantic behavior requires appropriate major compatibility treatment unless the governing language contract explicitly defines another mechanism.

---

45. Reserved-space integration

Reserved syntax exists to preserve future evolution.

Reserved identifiers MUST NOT automatically receive semantic meaning.

The reserved-space policy belongs to:

grammar/compatibility/reserved.md

Semantic-versioning MUST consume reserved-space decisions rather than independently reserving syntax.

---

46. Compatibility matrix integration

Every semantic-version relationship MUST be representable in:

grammar/compatibility/compatibility-matrix.md

At minimum:

Producer| Consumer| Domain| Relationship| Migration
semantic A| semantic A| same| exact| none
semantic A| semantic newer| same| backward-compatible| none
semantic A| semantic newer| same| migratable| required
semantic A| semantic newer| same| incompatible| reject
semantic newer| semantic older| same| unknown| reject unless declared

Unknown relationships MUST fail closed.

---

47. Compatibility negotiation

Semantic compatibility negotiation MUST be deterministic.

Conceptually:

producer semantic contract
        ↓
consumer supported contracts
        ↓
exact match?
        │
        ├── YES → ACCEPT
        │
        └── NO
             ↓
       compatible range?
             │
             ├── YES → ACCEPT
             │
             └── NO
                  ↓
             migration?
                  │
                  ├── YES → MIGRATE
                  │
                  └── NO → REJECT

The resolver MUST NOT guess.

---

48. Unknown versions

An unknown semantic version MUST NOT be treated as:

latest known version

or:

nearest compatible version

or:

older version

unless an explicit compatibility rule authorizes that mapping.

Unknown semantic versions MUST produce a deterministic compatibility result.

Default behavior MUST be fail-closed.

---

49. Forward compatibility

A newer semantic contract MUST NOT be assumed to be forward-compatible with older implementations.

Forward compatibility requires an explicit contract.

The following is insufficient:

old compiler ignores unknown fields

Ignoring information can cause semantic loss.

Forward compatibility MAY only be declared where omitted information is proven irrelevant under the applicable semantic contract.

---

50. Backward compatibility

Backward compatibility means:

«A program or semantic artifact valid under the older contract retains its specified meaning under the newer contract.»

Backward compatibility MUST be established at the semantic level, not merely through parsing.

---

51. Semantic extension

A semantic extension may be MINOR when:

old semantics
+
new semantics

coexist without changing existing behavior.

For example:

new capability
new operation
new optional metadata
new domain extension

may be added without breaking existing semantics.

The extension MUST NOT introduce an implicit reinterpretation of existing constructs.

---

52. Semantic tightening

A semantic tightening is potentially breaking.

Examples:

- formerly accepted behavior becomes invalid;
- a formerly optional guarantee becomes mandatory;
- an effect becomes newly prohibited;
- a resource requirement becomes mandatory;
- a policy becomes stricter;
- an operation loses a previously valid input.

Semantic tightening MUST undergo compatibility review.

It MUST NOT automatically be classified as PATCH merely because it improves correctness.

---

53. Semantic relaxation

A semantic relaxation MAY be compatible, but it must still be analyzed.

Examples:

- allowing a previously rejected valid value;
- permitting a broader type;
- accepting additional capability forms.

The change MUST NOT alter the behavior of previously valid programs.

---

54. Error and failure semantics

Error behavior is part of semantic compatibility when it is observable or specified.

Examples:

reject
retry
recover
escalate
degrade
fail

A change in failure semantics may require a semantic-version change.

This is especially important for:

- distributed systems;
- quantum execution;
- adaptive execution;
- resilience;
- sandboxing;
- FFI;
- networking;
- resource negotiation.

---

55. Resilience semantics

The existing resilience model includes states such as:

Unknown
Healthy
Degraded
Unstable
Unavailable
Recovering
Quarantined
Retired

and outcomes such as:

ACCEPT
DEGRADED_ACCEPT
RETRY
RECOVER
ESCALATE
REJECT

Changing the specified meaning of these semantic outcomes MUST be versioned appropriately.

A hardware implementation may add new physical states without changing the language contract, provided the mapping remains compatible.

---

56. Resource failure versus semantic failure

The following MUST remain distinct:

SEMANTIC_INVALID

versus:

RESOURCE_UNAVAILABLE

versus:

CAPABILITY_UNAVAILABLE

versus:

TARGET_INCOMPATIBLE

versus:

RUNTIME_UNAVAILABLE

For example:

requires capability("quantum.measurement")

failing on a target does not mean the source program has invalid semantics.

It means the selected target cannot satisfy the program's requirements.

---

57. No hardware-bound semantic versions

Semantic versions MUST NOT be created merely for:

- CPU models;
- GPU models;
- FPGA families;
- ASIC implementations;
- QPU devices;
- memory sizes;
- cluster sizes;
- node counts;
- network sizes;
- accelerator counts.

Those belong to target/capability/resource contracts.

---

58. No fixed scalability limits

This document MUST NOT establish universal semantic ceilings.

In particular, it MUST NOT define maximum values for:

qubits
logical qubits
physical qubits
CPUs
cores
threads
GPUs
FPGAs
ASIC resources
accelerators
nodes
devices
memory
register width
vector width
tensor rank
tensor dimension
network participants
program size
module count
function count
operation count

A concrete compiler or runtime may encounter implementation limits.

Such limits MUST be treated as implementation/resource constraints rather than universal language semantics.

---

59. Tiny-to-large execution

Semantic compatibility MUST remain independent of execution scale.

The same semantic contract may describe:

tiny computation

and:

large computation

provided the program's semantic requirements are satisfied.

The compiler/backend may specialize the implementation according to available resources.

The semantic contract MUST remain stable.

---

60. Target realization

The target realization pipeline is:

portable semantics
        ↓
resource analysis
        ↓
capability negotiation
        ↓
target selection
        ↓
specialization
        ↓
lowering
        ↓
routing
        ↓
scheduling
        ↓
resilience
        ↓
HAL
        ↓
execution

Semantic versioning stops at the semantic contract.

Target realization MUST NOT silently rewrite the semantic contract.

---

61. Canonical IR integration

Semantic compatibility MUST identify the canonical IR contract used to preserve semantics.

The general relationship is:

source
  ↓
AST
  ↓
semantic model
  ↓
canonical IR

The IR MAY evolve independently.

Therefore:

semantic-version
≠
IR-version

An IR migration MAY be required even when source semantics remain stable.

---

62. "quantum::ir" integration

For quantum semantics:

quantum source
    ↓
quantum semantic analysis
    ↓
quantum::ir

"quantum::ir" is the canonical quantum semantic boundary.

This document MUST NOT define a competing:

quantum-semantic-version-IR

or another quantum intermediate representation.

If "quantum::ir" changes incompatibly, its own IR compatibility contract MUST identify the impact on semantic consumers.

---

63. Optimization independence

Compiler optimization MUST preserve the semantic contract.

An optimization may change:

implementation
instruction selection
memory placement
parallelization
quantum decomposition
operation ordering where permitted
resource usage

without changing semantic version.

However, if an optimization changes specified observable behavior, the optimization is invalid under that semantic contract.

---

64. Routing independence

Routing is downstream from semantics.

For quantum systems:

logical operation
        ↓
quantum::ir
        ↓
routing
        ↓
physical mapping

A new routing algorithm does not automatically change semantic version.

A routing implementation MUST preserve the semantic contract.

---

65. Scheduling independence

Scheduling may adapt to:

- latency;
- throughput;
- resource availability;
- device constraints;
- topology;
- timing;
- power;
- thermal constraints.

Scheduling changes do not automatically constitute semantic-version changes.

They become semantic changes only when observable language guarantees are altered.

---

66. QEC independence

Quantum error correction is downstream.

QEC changes do not automatically require a language semantic-version change.

If a language-level fault-tolerance contract changes, the affected semantic domain MUST be versioned.

QEC implementation details remain owned by the QEC subsystem.

---

67. ZQN independence

ZQN handles applicable fault/noise semantics downstream.

A change to ZQN implementation does not automatically change the source semantic version.

If the language explicitly exposes a stable noise/fault semantic contract, that contract must be versioned independently.

---

68. ABI and FFI compatibility

Every FFI boundary MUST identify:

semantic contract
ABI contract
data layout
calling convention
effect
capabilities
provenance

A semantic-version bump MUST NOT be used to conceal an ABI break.

An ABI break MUST be classified independently.

---

69. Serialization compatibility

Semantic artifacts MAY be serialized.

Serialization format and semantic meaning are separate.

Therefore:

semantic version
≠
serialization format version

A serialization format may evolve while preserving semantic meaning.

Conversely, identical serialization does not guarantee semantic compatibility.

---

70. Metadata requirements

Every production semantic contract SHOULD expose machine-readable metadata containing at least:

domain
version
status
language-version compatibility
dependencies
provided semantics
consumed semantics
AST dependency
IR dependency
feature-gate dependencies
migration reference
deprecation status
compatibility classification
test suite identity
provenance identity

The metadata MUST be deterministic.

---

71. Semantic-version registry

The repository SHOULD maintain a canonical semantic-version registry.

Conceptually:

semantic domains
├── core
├── types
├── effects
├── resources
├── contracts
├── policies
├── provenance
├── classical
├── quantum
├── quantum::ir
├── hdl
├── hardware
├── distributed
├── networking
├── ai
├── data
└── interoperability

The registry MUST identify which domains are:

stable
experimental
deprecated
planned
unsupported

The registry MUST NOT claim implementation status that the compiler has not demonstrated.

---

72. Dependency versioning

A semantic contract MAY depend on other semantic contracts.

For example:

quantum.semantics
    depends on
core.semantics
types.semantics
effects.semantics
resources.semantics

Dependencies MUST identify compatible ranges.

A semantic dependency MUST NOT be satisfied by an unknown version merely because its name matches.

---

73. Dependency graph

Semantic compatibility forms a graph:

core
 ├── types
 ├── effects
 ├── resources
 ├── contracts
 ├── policies
 └── provenance
       │
       ├── classical
       ├── quantum
       │     └── quantum::ir
       ├── HDL
       ├── distributed
       ├── AI
       └── interoperability

The compatibility resolver MUST detect:

- cycles where prohibited;
- missing dependencies;
- incompatible ranges;
- unknown versions;
- conflicting contracts;
- unsupported combinations.

---

74. Compatibility closure

A semantic contract is compatible only if its required semantic dependencies are also compatible.

Therefore:

A compatible
+
B incompatible
=
A cannot claim fully compatible deployment

unless A has an explicit independent fallback that preserves the required semantic guarantees.

---

75. Feature status

Semantic-version metadata MUST integrate with feature lifecycle:

PROPOSED
DESIGNED
EXPERIMENTAL
IMPLEMENTED
STABLE
DEPRECATED
REMOVED

A feature MUST NOT be declared STABLE solely because:

- grammar exists;
- parser recognizes it;
- AST node exists;
- documentation exists.

Stable status requires the full semantic contract and conformance path.

---

76. Production semantic feature contract

Every stable semantic feature MUST identify:

Feature ID
Semantic Domain
Semantic Version
Purpose
Owns
Does Not Own
Syntax Owner
AST Owner
Semantic Owner
Type Contract
Effect Contract
Capability Contract
Resource Contract
Contract Contract
Policy Contract
Provenance Contract
IR Contract
Quantum Boundary
HDL Boundary
Backend Boundary
Compatibility
Migration
Deprecation
Diagnostics
Tests

---

77. File-level integration contract

This file itself has the following contract.

"DEPENDS_ON"

grammar/DESIGN.md
grammar/specification/language-version.md
grammar/spec/compatibility.md
grammar/compatibility/versions.md
grammar/compatibility/feature-gates.md
grammar/compatibility/compatibility-matrix.md
grammar/compatibility/migrations.md
grammar/compatibility/deprecated.md
grammar/compatibility/reserved.md
grammar/validation/compatibility-rules.md

"EXPORTS"

semantic-version terminology
semantic-version syntax
semantic-version compatibility classes
semantic-version bump rules
semantic compatibility rules
semantic negotiation rules
semantic migration requirements
semantic testing requirements

"CONSUMED_BY"

grammar/compatibility/compatibility-matrix.md
grammar/compatibility/migrations.md
grammar/compatibility/deprecated.md
grammar/compatibility/feature-gates.md
grammar/spec/compatibility.md
grammar/validation/compatibility-rules.md
grammar/grammar.md
compiler compatibility tooling
semantic conformance tooling
release tooling

"AST_OWNER"

Not owned by this document.

AST ownership remains with the frontend AST subsystem.

"SEMANTIC_OWNER"

The applicable domain semantic specification and implementation.

"IR_OWNER"

The applicable canonical IR subsystem.

For quantum semantics:

quantum::ir

remains authoritative.

"TEST_OWNER"

grammar/tests/compatibility/

and applicable domain-specific compatibility tests.

"SPEC_OWNER"

grammar/spec/compatibility.md
grammar/specification/language-version.md

for their respective concerns.

---

78. Semantic-version resolution algorithm

A production resolver MUST conceptually perform:

1. Parse semantic-domain identity.
2. Parse semantic-version identity.
3. Validate canonical representation.
4. Resolve dependencies.
5. Resolve language-version compatibility.
6. Resolve feature gates.
7. Resolve deprecation state.
8. Resolve migration requirements.
9. Compare producer and consumer contracts.
10. Determine compatibility class.
11. Reject unknown/incompatible contracts.
12. Produce deterministic diagnostics.

The resolver MUST NOT use:

latest available

as an implicit compatibility policy.

---

79. Deterministic resolution

Given identical:

source
language version
semantic versions
feature gates
dialects
compatibility metadata
compiler configuration

the semantic compatibility result MUST be deterministic.

The resolver MUST NOT depend on:

- hash-map iteration order;
- machine-specific ordering;
- network response order;
- target enumeration order;
- nondeterministic plugin discovery;
- unspecified filesystem ordering.

---

80. Network independence

Semantic compatibility resolution SHOULD be possible from local compatibility metadata.

A network lookup MAY supplement metadata.

It MUST NOT silently change the result.

For reproducible compilation:

same semantic inputs

MUST produce:

same compatibility decision

independent of external service availability.

---

81. Security

Semantic-version parsing MUST be safe against malformed input.

The implementation MUST:

- validate input;
- reject malformed versions;
- avoid panics;
- avoid unchecked memory operations;
- avoid Rust "unsafe";
- avoid code execution;
- avoid network-dependent semantic interpretation;
- avoid implicit version substitution.

Untrusted version metadata MUST be treated as data.

---

82. Rust implementation requirements

The production Zamani implementation MUST support:

Rust 1.97.1 or later
Rust edition 2021

The implementation MUST use safe Rust.

No semantic-version implementation may require:

unsafe

or equivalent unsafe behavior.

The implementation SHOULD use explicit domain types rather than passing raw strings throughout the compiler.

Conceptually:

SemanticDomain
SemanticVersion
SemanticVersionRange
SemanticCompatibility
SemanticDependency
SemanticMigration
SemanticStatus

These are implementation concepts; their exact Rust representation is owned by the compiler source tree.

---

83. Version parsing

Version parsing MUST be:

- deterministic;
- total over its declared input domain;
- explicit about malformed input;
- independent of hardware;
- independent of target capacity;
- independent of network availability.

Malformed versions MUST produce structured diagnostics.

The parser MUST NOT silently truncate version components.

---

84. Version ordering

Semantic-version ordering MUST be defined independently from:

- lexical string ordering;
- compiler version ordering;
- Git commit ordering;
- release-date ordering;
- target version ordering.

For example:

10.0.0

must not be ordered lexically before:

2.0.0

merely because of string comparison.

---

85. Pre-release ordering

If pre-release versions are supported, their ordering MUST be explicitly defined.

A pre-release MUST NOT automatically satisfy a stable-version dependency unless the compatibility contract explicitly permits it.

For example:

1.2.0-beta

MUST NOT silently satisfy:

1.2.0

merely because the numeric components match.

---

86. Build metadata

Build metadata MUST NOT alter semantic compatibility.

For example:

1.2.3+build-a

and:

1.2.3+build-b

represent the same semantic version unless another independently declared artifact contract distinguishes them.

Build metadata may identify:

- compiler build;
- provenance;
- reproducibility record;
- packaging information.

It MUST NOT conceal semantic differences.

---

87. Version ranges

Version ranges MAY be used for dependencies.

Examples:

>=1.0,<2.0

or equivalent canonical repository syntax.

Range semantics MUST be deterministic.

Open-ended ranges MUST NOT accidentally accept incompatible future major versions.

A dependency declared as:

>=1.0

MUST NOT be interpreted as:

all future versions regardless of compatibility

unless the repository explicitly defines that behavior.

---

88. Compatibility claims

Every compatibility claim MUST answer:

Compatible with what?
At which layer?
For which semantic domain?
Under which version?
With which guarantees?

The statement:

compatible

without those dimensions is insufficient for production tooling.

---

89. Semantic compatibility report

Tooling SHOULD be able to produce a structured report similar to:

Domain:
    quantum.semantics

Producer:
    2.1.0

Consumer:
    2.3.0

Result:
    SEMANTIC_BACKWARD_COMPATIBLE

Migration:
    none

Dependencies:
    core.semantics compatible
    types.semantics compatible
    effects.semantics compatible
    resources.semantics compatible

Feature gates:
    satisfied

Target:
    not evaluated by semantic compatibility resolver

The exact tooling format belongs to the compiler/tooling subsystem.

---

90. Separation from target compatibility

Semantic compatibility MUST finish before target realization compatibility.

The conceptual pipeline is:

semantic compatibility
        ↓
IR compatibility
        ↓
compiler compatibility
        ↓
target capability compatibility
        ↓
resource feasibility
        ↓
deployment

A target failure MUST NOT be misreported as a semantic-version failure.

---

91. Example: quantum target

Suppose source semantics require:

requires capability("quantum.measurement");

A target lacking that capability may produce:

CAPABILITY_UNAVAILABLE

It MUST NOT produce:

SEMANTIC_VERSION_INCOMPATIBLE

unless the semantic contract itself cannot be represented.

---

92. Example: memory

Suppose a program expresses:

requires memory >= required_memory;

A target with insufficient memory may reject execution.

This does not mean:

language semantic version incompatible

It means:

resource requirement unsatisfied

The semantic meaning remains unchanged.

---

93. Example: GPU

A program may require:

requires capability("gpu.compute");

A CPU-only target may fail capability negotiation.

The source semantic contract remains valid.

No semantic version change is required merely because the target lacks a GPU.

---

94. Example: future hardware

A future target may expose capabilities unknown when the language version was created.

The semantic architecture MUST allow the target to implement the existing semantic contract without requiring a source-language rewrite.

This is essential for POCO-REAF.

---

95. Semantic approximation

An implementation MUST NOT silently approximate semantics.

For example:

exact quantum measurement

MUST NOT silently become:

classical random-number generation

unless an explicit approximation/alternative semantic mode exists and the source program author has requested or permitted it.

Any approximation mode MUST have:

- explicit identity;
- semantic contract;
- version;
- effect;
- policy;
- provenance;
- diagnostics;
- compatibility rules.

---

96. Semantic fallback

Fallback is permitted only when explicitly specified.

Conceptually:

preferred semantic realization
        ↓
unavailable
        ↓
declared fallback
        ↓
compatibility check
        ↓
execute

Fallback MUST NOT silently change required guarantees.

---

97. Deterministic fallback

If multiple semantic fallbacks are available, selection MUST be deterministic according to an explicit policy.

The resolver MUST NOT select a fallback based merely on:

- enumeration order;
- hash order;
- arbitrary backend availability;
- network timing.

---

98. Provenance of semantic migrations

Every semantic migration SHOULD record:

source semantic version
destination semantic version
migration identity
migration tool/version
source provenance
transformation provenance
validation result
known semantic differences

A migrated artifact MUST remain traceable to its original semantic contract.

---

99. Migration validation

A migration is production-ready only if:

source artifact
      ↓
migration
      ↓
destination artifact
      ↓
semantic validation

succeeds.

Migration tooling MUST reject transformations that cannot establish the required semantic guarantees.

---

100. Deprecated semantic versions

A semantic version MAY become deprecated.

Deprecation MUST identify:

- deprecated domain;
- deprecated version;
- reason;
- replacement;
- migration;
- removal policy;
- compatibility window;
- diagnostics.

Deprecation MUST NOT silently alter semantics.

---

101. Removal

When a semantic contract is removed:

removed version

MUST remain identifiable in historical compatibility metadata.

Tooling SHOULD retain enough information to produce useful diagnostics such as:

semantic contract no longer supported
replacement: ...
migration: ...

---

102. Experimental semantic contracts

Experimental semantics MUST have explicit status.

Experimental contracts MAY change incompatibly without a stable MAJOR promise, but such changes MUST still be:

- identified;
- documented;
- diagnosable;
- tested;
- versioned;
- excluded from stable compatibility claims.

Experimental status MUST NOT be used to conceal undocumented changes.

---

103. Stable semantic contracts

A semantic contract may be marked STABLE only when:

- specification exists;
- implementation exists;
- compatibility identity exists;
- AST mapping exists;
- semantic mapping exists;
- IR mapping exists;
- required compiler support exists;
- diagnostics exist;
- migration policy exists;
- deprecation policy exists;
- positive tests exist;
- negative tests exist;
- boundary tests exist;
- scalability tests exist;
- determinism tests exist;
- compatibility tests exist;
- cross-domain tests exist where applicable.

---

104. Required test categories

Every semantic-version implementation MUST test:

version parsing
version comparison
version ordering
range resolution
exact compatibility
backward compatibility
forward compatibility
extension compatibility
incompatibility
unknown versions
unsupported versions
pre-release handling
build metadata
dependency resolution
feature-gate interaction
migration selection
deprecation handling
diagnostics
determinism
reproducibility

---

105. Semantic regression tests

Every stable semantic contract MUST have regression tests covering:

old valid program
    ↓
new implementation
    ↓
same specified semantics

Tests MUST NOT merely verify that parsing succeeds.

---

106. Negative tests

Negative tests MUST verify rejection of:

- malformed semantic versions;
- unknown semantic domains;
- incompatible versions;
- missing dependencies;
- invalid version ranges;
- unsupported migrations;
- incompatible feature-gate combinations;
- deprecated versions beyond their supported window;
- semantically unsafe fallback;
- silent version substitution.

---

107. Boundary tests

Boundary tests MUST cover:

language ↔ semantic
grammar ↔ semantic
AST ↔ semantic
semantic ↔ IR
semantic ↔ quantum::ir
semantic ↔ ABI
semantic ↔ runtime
semantic ↔ dialect
semantic ↔ target
semantic ↔ resources
semantic ↔ capabilities

---

108. Cross-domain tests

At minimum, compatibility testing SHOULD include:

classical
quantum
hybrid
HDL
AI
data
distributed
networking
interoperability
security
metaprogramming
simulation
adaptive execution

The tests MUST verify that independent domains continue to share the common semantic foundation.

---

109. POCO-REAF scalability tests

Scalability tests MUST demonstrate that semantic compatibility does not depend on artificial resource ceilings.

Tests SHOULD vary:

- logical problem size;
- number of logical resources;
- tensor dimensions;
- quantum register sizes;
- distributed participants;
- data sizes;
- execution resources.

The test suite MUST not encode the tested size as a universal language maximum.

For example:

test with N

means:

test instance size = N

not:

language maximum = N

---

110. Tiny-target tests

The compatibility suite SHOULD include small targets or abstract execution environments.

The purpose is to prove that semantic contracts remain valid at small scale.

A small target MAY reject a resource requirement.

That rejection MUST remain separate from semantic compatibility.

---

111. Large-target tests

The compatibility suite SHOULD include progressively larger semantic workloads where infrastructure permits.

No tested size may be promoted into a language-wide ceiling.

The semantic contract remains symbolic and resource-independent.

---

112. Future-target tests

The repository SHOULD include abstract target fixtures representing capabilities not tied to today's hardware.

This verifies that semantic contracts are expressed in terms of:

intent
requirements
capabilities
constraints
policies

rather than physical device assumptions.

---

113. Compiler implementation conformance

The compiler MUST validate:

semantic metadata
↓
supported version
↓
dependencies
↓
feature gates
↓
migration
↓
semantic implementation
↓
IR implementation

A compiler MUST NOT advertise a semantic version that it cannot actually implement.

---

114. Generated artifacts

Generated parser or compiler artifacts are derived outputs.

They MUST NOT become semantic-version authorities.

The authority remains:

specification
↓
semantic contract
↓
implementation

Generated files MUST carry enough metadata to identify their source contracts where practical.

---

115. Source provenance

Semantic compilation SHOULD preserve:

source location
source version
semantic domain
semantic version
feature gates
dialects
migration

This supports:

- diagnostics;
- reproducibility;
- debugging;
- auditability;
- explanation;
- compatibility analysis.

---

116. Diagnostics

Semantic-version diagnostics MUST identify:

diagnostic code
semantic domain
producer version
consumer version
compatibility result
affected feature
required migration
deprecation status

Diagnostics MUST distinguish:

unknown
unsupported
incompatible
deprecated
migratable
resource unavailable
capability unavailable
target unavailable

---

117. Fail-closed compatibility

The following MUST fail closed unless explicitly specified otherwise:

unknown semantic version
unknown semantic domain
missing compatibility metadata
missing required dependency
ambiguous migration
invalid version range
unsupported migration
unverified forward compatibility
unverified semantic equivalence

The implementation MUST NOT guess.

---

118. No silent downgrade

A newer semantic contract MUST NOT silently downgrade to an older semantic contract.

For example:

requested 3.x
available 2.x

does not imply:

use 2.x

unless an explicit compatibility/migration rule authorizes it.

---

119. No silent upgrade

Likewise:

requested 2.x
available 3.x

does not imply:

use 3.x

unless backward compatibility is explicitly established and the version-resolution contract permits it.

---

120. Version pinning

Production artifacts SHOULD be able to pin semantic contracts where reproducibility requires it.

A pin identifies:

semantic domain
version
compatibility mode

Pinning MUST NOT pin physical resources unless the artifact explicitly contains a target/deployment contract.

---

121. Floating compatibility ranges

Development environments MAY use compatibility ranges.

Production/reproducible builds SHOULD record the resolved semantic versions.

A floating range MUST be resolved deterministically.

---

122. Semantic lock information

A reproducible build record SHOULD contain:

language version
grammar version
AST contract version
semantic-domain versions
IR versions
dialect versions
feature gates
compiler version
relevant policy versions
migration records

Target/resource state MAY be separately recorded.

---

123. Version provenance

Every semantic-version release SHOULD have provenance identifying:

source specification revision
implementation revision
tests
compatibility matrix
migration records
deprecation records
release identity

This allows semantic contracts to be audited independently from implementation binaries.

---

124. Change-control process

A semantic change MUST follow:

proposal
   ↓
semantic impact analysis
   ↓
compatibility classification
   ↓
version decision
   ↓
specification update
   ↓
AST/semantic/IR impact analysis
   ↓
migration analysis
   ↓
implementation
   ↓
tests
   ↓
compatibility matrix update
   ↓
release

A semantic version MUST NOT be changed merely because a source file changed.

---

125. Semantic impact analysis

Every proposed semantic change MUST answer:

What meaning changes?
What existing programs are affected?
What semantic guarantees change?
What AST data is affected?
What IR data is affected?
What effects change?
What capabilities change?
What resources change?
What policies change?
What provenance changes?
What dialects depend on it?
What migrations are possible?
What targets are affected?

---

126. Version bump checklist

Before approving a semantic-version bump:

MAJOR

Confirm:

- existing stable meaning changed;
- compatibility break documented;
- migration assessed;
- diagnostics defined;
- compatibility matrix updated;
- tests updated.

MINOR

Confirm:

- existing meaning unchanged;
- new behavior is additive;
- dependencies remain compatible;
- feature gates are explicit;
- tests demonstrate backward compatibility.

PATCH

Confirm:

- no stable meaning changed;
- correction is compatible;
- regression tests exist;
- deterministic behavior is preserved.

---

127. Repository integration matrix

File| Semantic-version integration
"grammar/DESIGN.md"| Defines architectural boundary
"grammar/specification/language-version.md"| Defines language-version identity
"grammar/spec/compatibility.md"| Defines cross-layer compatibility
"grammar/compatibility/versions.md"| Defines version taxonomy
"grammar/compatibility/feature-gates.md"| Defines feature availability
"grammar/compatibility/compatibility-matrix.md"| Records compatibility relationships
"grammar/compatibility/migrations.md"| Defines migrations
"grammar/compatibility/deprecated.md"| Defines deprecation
"grammar/compatibility/reserved.md"| Protects future syntax/namespace space
"grammar/compatibility/frontend-conformance.md"| Protects frontend meaning preservation
"grammar/validation/compatibility-rules.md"| Validates implementation consistency
"grammar/grammar.md"| Reports implementation status
"grammar/Zamani-Grammar.md"| Historical/proposed material
"grammar/resources/"| Resource requirements/capabilities
"grammar/effects/"| Effect semantics
"grammar/validation/"| Contracts and semantic validation
"grammar/quantum/"| Quantum source semantics
"grammar/hybrid/"| Hybrid semantic integration
"grammar/hdl/"| Hardware-description semantics
"grammar/ai/"| AI/reasoning/learning/adaptation semantics
"grammar/interoperability/"| FFI/ABI semantics
"grammar/dialects/"| Extension semantic contracts
"grammar/tests/"| Compatibility verification

---

128. Required additions if absent

The semantic-version architecture SHOULD add or reconcile the following machine-readable concepts if they are not already represented elsewhere:

SemanticDomain
SemanticVersion
SemanticVersionRange
SemanticDependency
SemanticCompatibility
SemanticCompatibilityClass
SemanticStatus
SemanticMigrationRef
SemanticDeprecation
SemanticFeatureIdentity

These SHOULD have one canonical owner.

They MUST NOT be independently redefined by each domain.

---

129. Suggested repository organization

The compatibility subsystem should converge toward:

grammar/compatibility/
├── versions.md
├── semantic-version.md
├── grammar-version.md
├── ast-version.md
├── ir-version.md
├── feature-gates.md
├── compatibility.md
├── compatibility-matrix.md
├── frontend-conformance.md
├── migrations.md
├── deprecated.md
├── reserved.md
├── dialects.md
└── ...

Each version document MUST have a distinct responsibility.

In particular:

language version
≠
grammar version
≠
AST version
≠
semantic version
≠
IR version
≠
ABI version
≠
runtime version
≠
dialect version
≠
compiler version
≠
target version

---

130. No version-domain conflation

The following substitutions are prohibited:

compiler version used as language version
grammar version used as semantic version
AST version used as language version
IR version used as semantic version
runtime version used as ABI version
target version used as language version
dialect version used as core language version

Each version must remain attributable to its owner.

---

131. Compatibility provenance chain

A production semantic artifact SHOULD be traceable through:

source
 ↓
language version
 ↓
grammar version
 ↓
AST version
 ↓
semantic-domain versions
 ↓
IR versions
 ↓
compiler version
 ↓
dialect versions
 ↓
artifact version
 ↓
target contract
 ↓
runtime

Not every artifact requires every layer, but every applicable layer MUST be identifiable.

---

132. Reverse traceability

The repository SHOULD support reverse tracing:

target failure
   ↓
artifact
   ↓
IR
   ↓
semantic contract
   ↓
AST
   ↓
source

This enables precise diagnostics instead of generic “version mismatch” failures.

---

133. Compatibility and diagnostics ownership

Semantic-version diagnostics MUST be generated at the compatibility layer.

Semantic meaning diagnostics remain owned by semantic analysis.

Resource diagnostics remain owned by resource validation.

Target diagnostics remain owned by target compatibility.

This prevents:

all failures → version mismatch

from becoming the default error model.

---

134. Security invariants

Semantic versioning MUST NOT bypass:

- type checking;
- effect checking;
- capability checking;
- resource validation;
- contract validation;
- policy validation;
- provenance;
- authorization;
- sandboxing.

A version declaration MUST NOT grant authority.

---

135. Metaprogramming and reflection

Metaprogramming MAY generate semantic constructs.

Generated constructs MUST still resolve against the declared semantic contracts.

Reflection MUST NOT permit a program to silently switch its semantic version during execution.

Compile-time generation MUST record the applicable semantic contract where required for reproducibility.

---

136. Simulation

Simulation is an execution strategy.

A simulation backend MUST consume the same semantic contract unless an explicitly declared simulation approximation changes semantics.

A simulator MUST NOT silently reinterpret a semantic operation merely because it is being simulated.

---

137. Adaptive execution

Adaptive execution may select among implementations.

The semantic contract remains authoritative.

Conceptually:

semantic intent
       ↓
candidate realizations
       ↓
capability/resource/policy evaluation
       ↓
selected realization

The selected realization MUST preserve required semantic guarantees.

---

138. Deterministic semantic selection

If multiple realizations satisfy the same semantic contract, selection MAY be target-policy dependent.

However, reproducible builds MUST record enough policy information to reproduce the decision when reproducibility is claimed.

---

139. Compatibility with future domains

Future computing domains MUST be able to introduce:

domain identity
semantic contract
semantic version
AST mapping
IR mapping
capability model
resource model
effects
policies
provenance
compatibility
tests

without redefining core semantic-versioning rules.

This allows Zamani to expand beyond currently known computational architectures.

---

140. What semantic versioning must never become

This document MUST NOT become:

- a hardware catalog;
- a target registry;
- a resource allocator;
- a QEC specification;
- a routing specification;
- a scheduler;
- a runtime specification;
- a compiler optimization specification;
- a dialect grammar;
- an application framework;
- a second language specification.

Its sole architectural purpose is semantic-contract evolution and compatibility.

---

141. Production readiness checklist

"semantic-version.md" is production-ready only when all of the following are true:

- [ ] Semantic versioning has one authoritative definition.
- [ ] Language versioning remains separate.
- [ ] Grammar versioning remains separate.
- [ ] AST versioning remains separate.
- [ ] IR versioning remains separate.
- [ ] ABI versioning remains separate.
- [ ] Runtime versioning remains separate.
- [ ] Dialect versioning remains separate.
- [ ] Compiler versioning remains separate.
- [ ] Target versioning remains separate.
- [ ] Semantic domains have explicit identities.
- [ ] Semantic versions are deterministic.
- [ ] Compatibility classes are defined.
- [ ] MAJOR/MINOR/PATCH rules are defined.
- [ ] Unknown versions fail closed.
- [ ] Silent downgrade is prohibited.
- [ ] Silent upgrade is prohibited.
- [ ] Silent reinterpretation is prohibited.
- [ ] Migration references are explicit.
- [ ] Deprecation references are explicit.
- [ ] Feature gates remain separate.
- [ ] Resource requirements remain separate.
- [ ] Capability requirements remain separate.
- [ ] Policies remain separate.
- [ ] Provenance remains traceable.
- [ ] "quantum::ir" remains the canonical quantum semantic boundary.
- [ ] No competing quantum IR is introduced.
- [ ] No hardware capacity ceiling is encoded.
- [ ] POCO-REAF remains preserved.
- [ ] Rust 1.97.1 or later is supported.
- [ ] Rust 2021 remains supported.
- [ ] Rust "unsafe" is prohibited.
- [ ] Compatibility resolution is deterministic.
- [ ] Semantic migration is tested.
- [ ] Deprecation is tested.
- [ ] Unknown-version behavior is tested.
- [ ] Cross-domain semantic compatibility is tested.
- [ ] Scalability tests do not become universal limits.
- [ ] Reproducibility metadata is defined.
- [ ] Security invariants are enforced.
- [ ] Diagnostics distinguish semantic, resource, capability, runtime, and target failures.

---

142. Definition of done for this file

This file is complete when:

semantic contract
       ↓
semantic domain
       ↓
semantic version
       ↓
compatibility class
       ↓
dependency resolution
       ↓
feature-gate resolution
       ↓
migration/deprecation resolution
       ↓
AST/IR integration
       ↓
compiler validation
       ↓
tests

is completely specified.

No later modification to another compatibility document should require redefining the fundamental semantic-version model in this file.

If another document changes an ownership boundary, that document MUST instead update its own contract and the compatibility matrix.

---

143. Final semantic-versioning model

The final Zamani compatibility architecture is:

                    LANGUAGE VERSION
                           │
          ┌────────────────┼────────────────┐
          │                │                │
       GRAMMAR            AST          SEMANTICS
          │                │                │
          │                │        ┌───────┼────────┐
          │                │        │       │        │
          │                │      Types  Effects  Resources
          │                │        │       │        │
          │                │        └───────┼────────┘
          │                │                │
          └────────────────┼────────────────┘
                           │
                           ▼
                CANONICAL SEMANTIC MODEL
                           │
             ┌─────────────┼─────────────┐
             │             │             │
             ▼             ▼             ▼
        Classical      Quantum        HDL/
        Semantics      Semantics      Hardware
             │             │             │
             │             ▼             │
             │        quantum::ir        │
             │             │             │
             └─────────────┼─────────────┘
                           │
                           ▼
                    OPTIMIZATION
                           │
                           ▼
                     LOWERING
                           │
              ┌────────────┼────────────┐
              ▼            ▼            ▼
           ROUTING     SCHEDULING    RESILIENCE
                                        │
                                   ┌────┴────┐
                                   ▼         ▼
                                  QEC       ZQN
                                   │         │
                                   └────┬────┘
                                        ▼
                                       HAL
                                        │
                                        ▼
                                  TARGET/RUNTIME

Semantic versions therefore sit at the correct boundary:

WHAT THE COMPUTATION MEANS

while the downstream system determines:

HOW THAT MEANING IS REALIZED

on available computational resources.

---

144. Final non-negotiable rules

1. Semantic versioning MUST be independent of language versioning.

2. Semantic versioning MUST be independent of grammar versioning.

3. Semantic versioning MUST be independent of AST versioning.

4. Semantic versioning MUST be independent of IR versioning.

5. Semantic versioning MUST be independent of ABI versioning.

6. Semantic versioning MUST be independent of runtime versioning.

7. Semantic versioning MUST be independent of compiler versioning.

8. Semantic versioning MUST be independent of target versioning.

9. Dialect versions MUST remain separately identifiable.

10. Unknown semantic versions MUST NOT be silently accepted.

11. Unknown semantic versions MUST NOT be silently downgraded.

12. Unknown semantic versions MUST NOT be silently upgraded.

13. Compatible implementations MUST NOT silently reinterpret semantics.

14. Semantic migrations MUST be explicit.

15. Deprecations MUST be explicit.

16. Feature gates MUST NOT replace semantic versioning.

17. Resource requirements MUST NOT be encoded as semantic-version limits.

18. Capability availability MUST NOT be confused with semantic compatibility.

19. Target infeasibility MUST NOT be reported as semantic invalidity.

20. "quantum::ir" MUST remain the canonical quantum semantic boundary.

21. A second quantum IR MUST NOT be introduced.

22. Quantum operation semantics MUST remain extensible.

23. No universal hardware/resource ceiling may be encoded.

24. POCO-REAF MUST remain a semantic portability architecture rather than a hardware-specific promise.

25. Rust 1.97.1 or later MUST be supported.

26. Rust 2021 MUST remain the implementation edition.

27. Production Rust MUST remain safe Rust; "unsafe" MUST NOT be required or used.

28. Semantic compatibility resolution MUST be deterministic.

29. Semantic compatibility MUST be testable.

30. Every stable semantic contract MUST have specification, implementation, compatibility, migration, provenance, and conformance coverage.

31. Every semantic change MUST be classified before release.

32. A version number MUST never be used to conceal an architectural boundary violation.

33. The semantic contract MUST remain portable from the smallest meaningful computation to arbitrarily large computations permitted by actual resources.

34. Physical realization MUST remain downstream of portable semantic intent.

35. Future computational domains MUST be able to integrate through explicit semantic contracts rather than forcing redesign of the universal versioning architecture.

---

145. Final conformance statement

The purpose of semantic versioning in Zamani is not merely to label releases.

It is to preserve a precise chain of meaning:

SOURCE INTENT
     ↓
LANGUAGE CONTRACT
     ↓
GRAMMAR CONTRACT
     ↓
AST CONTRACT
     ↓
SEMANTIC CONTRACT
     ↓
CANONICAL IR
     ↓
TARGET-INDEPENDENT TRANSFORMATION
     ↓
TARGET REALIZATION

The semantic-version layer guarantees that the meaning represented at the semantic boundary is explicitly identifiable, comparable, migratable, testable, reproducible, and protected from silent reinterpretation.

The resulting architecture is:

Program
   ↓
portable semantic intent
   ↓
semantic version
   ↓
semantic validation
   ↓
resource/capability negotiation
   ↓
canonical IR
   ↓
optimization
   ↓
lowering
   ↓
routing
   ↓
scheduling
   ↓
resilience
   ↓
QEC/ZQN where applicable
   ↓
HAL
   ↓
available target

Therefore:

«Zamani semantic versioning versions meaning, not machines.»

A new machine does not require a new language merely because its hardware is different.

A larger machine does not require a new semantic contract merely because it has more resources.

A smaller machine does not change the program's meaning merely because it cannot satisfy the program's requirements.

A new compiler implementation does not automatically create a new language.

A new backend does not automatically create a semantic break.

A new quantum device does not automatically create a new quantum language.

A new computational domain can be added through an explicit semantic contract.

This is the compatibility foundation required for:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)

while keeping the semantic boundary independent of present-day hardware and implementation limitations.
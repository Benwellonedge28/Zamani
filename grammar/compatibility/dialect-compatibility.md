Zamani Dialect Compatibility Contract

Path: "grammar/compatibility/dialect-compatibility.md"
Status: Normative
Language: Zamani
Scope: Dialect identity, dialect versioning, core-language compatibility, dialect composition, dependencies, feature gates, syntax compatibility, AST compatibility, semantic compatibility, IR compatibility, interoperability, migration, deprecation, vendor extensions, experimental extensions, forward/backward compatibility, deterministic resolution, scalability, POCO-REAF, and integration with the complete Zamani compilation pipeline
Grammar technology: ANTLR4-compatible
Rust implementation baseline: Rust 1.97 / Rust 1.97.1
Rust edition: 2021
Rust safety requirement: Production implementation MUST use safe Rust; Rust "unsafe" MUST NOT be required or used by the Zamani compiler implementation
Primary portability objective: "Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever" (POCO-REAF)

---

0. Document Contract

0.1 Purpose

This document defines the normative compatibility contract for Zamani dialects.

It establishes how a dialect may:

- extend Zamani;
- specialize Zamani;
- compose with another dialect;
- depend on another dialect;
- provide capabilities;
- require capabilities;
- introduce syntax;
- introduce semantic constructs;
- identify lowering requirements;
- evolve independently;
- interoperate with other dialects;
- remain compatible with the core Zamani language;
- remain compatible with other versions of itself;
- migrate deprecated constructs;
- participate in experimental language evolution;
- coexist with future computational domains.

This document also defines what dialect compatibility does not mean.

A dialect is not:

- a separate programming language;
- a hardware target;
- a physical device;
- a CPU/GPU/QPU selector;
- a scheduler;
- a router;
- a resource allocator;
- a QEC implementation;
- a ZQN implementation;
- a runtime;
- a duplicate AST;
- a duplicate semantic universe;
- a duplicate quantum IR.

---

0.2 Fundamental principle

Zamani remains one language.

A dialect extends the Zamani language contract without replacing the common language foundation.

The required relationship is:

Zamani core language
        │
        ├── dialect A
        ├── dialect B
        ├── dialect C
        └── future dialects

not:

Zamani
 ├── Language A
 ├── Language B
 ├── Language C
 └── unrelated languages

A dialect MUST therefore preserve the common Zamani frontend and semantic architecture.

---

0.3 Compatibility invariant

The fundamental invariant is:

«A compatible dialect implementation MUST NOT silently change the specified meaning of valid source code.»

If a dialect intentionally changes meaning, that change MUST be explicitly classified and versioned.

It MUST NOT be hidden behind:

- target selection;
- compiler version;
- optimization level;
- backend selection;
- hardware availability;
- runtime selection;
- parser implementation details.

---

0.4 Dialect compatibility is layered

Dialect compatibility MUST be evaluated independently at each applicable layer:

Dialect identity
       ↓
Dialect version
       ↓
Core-language compatibility
       ↓
Lexical compatibility
       ↓
Syntax compatibility
       ↓
AST compatibility
       ↓
Name/module compatibility
       ↓
Type compatibility
       ↓
Effect compatibility
       ↓
Capability compatibility
       ↓
Resource compatibility
       ↓
Semantic compatibility
       ↓
Canonical IR compatibility
       ↓
Artifact compatibility
       ↓
Runtime compatibility
       ↓
Target compatibility
       ↓
Execution compatibility

A dialect MAY therefore be:

- source-compatible but target-infeasible;
- syntax-compatible but semantically incompatible;
- AST-compatible but IR-incompatible;
- semantically compatible but resource-infeasible;
- core-language-compatible but dialect-version-incompatible;
- dialect-compatible but runtime-incompatible.

These conditions MUST NOT be conflated.

---

1. Authority and Ownership

1.1 Repository authority

Dialect compatibility MUST follow the repository's existing authority model:

grammar/DESIGN.md
        ↓
grammar/specification/
        ↓
grammar/spec/
        ↓
grammar/Zamani.g4
        ↓
lexer / parser
        ↓
frontend AST
        ↓
semantic analysis
        ↓
canonical semantic representation
        ↓
canonical IR
        ↓
optimization / lowering
        ↓
routing / scheduling / resilience / QEC / ZQN
        ↓
HAL
        ↓
runtime / target realization

Dialect compatibility does not create another authority layer above the language specification.

---

1.2 File ownership

File / subsystem| Owns| Does not own
"grammar/DESIGN.md"| architecture and boundaries| dialect-specific compatibility records
"grammar/specification/"| normative language specification| implementation details
"grammar/spec/compatibility.md"| general compatibility semantics| individual dialect contracts
"grammar/compatibility/versions.md"| language-version policy| dialect-specific compatibility implementation
"grammar/compatibility/migrations.md"| migration procedures| dialect syntax
"grammar/compatibility/deprecated.md"| deprecation lifecycle| dialect grammar
"grammar/compatibility/feature-gates.md"| feature availability/lifecycle| dialect semantic meaning
"grammar/compatibility/compatibility-matrix.md"| repository-wide compatibility relationships| dialect grammar
"grammar/compatibility/dialect-compatibility.md"| dialect compatibility policy| dialect concrete syntax
"grammar/dialects/README.md"| dialect architecture| compatibility implementation algorithms
"grammar/dialects/dialects.g4"| dialect grammar façade/composition| compatibility resolution
"grammar/dialects/registration.g4"| registration syntax| compatibility algorithms
"grammar/dialects/*" component grammars| component syntax| semantic compatibility
"grammar/Zamani.g4"| canonical grammar composition| dialect runtime behavior
"src/frontend/ast/"| domain-neutral AST| physical realization
semantic analysis| dialect resolution and compatibility| lexical parsing
canonical IR| semantic representation| dialect registration syntax
"quantum::ir"| canonical quantum semantic boundary| dialect registry
QEC| quantum error correction| dialect compatibility
ZQN| fault/noise semantics| dialect registration
routing| physical realization| source dialect semantics
scheduling| execution scheduling| source dialect semantics
HAL| target/device realization| source compatibility
runtime| execution| language definition

---

1.3 Existing dialect grammar relationship

The existing dialect architecture defines:

- "grammar/dialects/dialects.g4" as the composition façade;
- "grammar/dialects/registration.g4" as the registration grammar;
- separate component ownership for names, versioning, capabilities, compatibility, vendors, and experimental extensions.

This contract MUST preserve that architecture.

The compatibility file MUST NOT duplicate those grammar productions.

---

2. Definition of a Dialect

A dialect is a named, versioned, explicitly scoped extension contract of Zamani.

A dialect MAY introduce:

- additional syntax;
- additional declarations;
- additional expressions;
- additional statements;
- additional types;
- additional effects;
- additional capabilities;
- additional semantic operations;
- additional resource requirements;
- additional interoperability mappings;
- additional domain-specific semantics.

A dialect MUST ultimately map its semantics into the common Zamani compilation architecture.

---

2.1 Open-world dialect model

Dialect identities MUST be open-world.

The grammar MUST NOT maintain a closed enumeration such as:

quantum
openqasm
qiskit
verilog
cuda
...

The existing dialect grammar correctly uses symbolic qualified names rather than a closed list.

Valid structural forms may include:

quantum::standard
quantum::openqasm
classical::numeric
hdl::rtl
hardware::fpga
ai::tensor
distributed::messaging
organization::domain::dialect
vendor::domain::extension
future::computing::dialect

These are examples of structure, not a closed registry.

A new dialect MUST be introducible without modifying the core dialect grammar merely because its identity is new.

---

2.2 Dialect identity

A dialect identity MUST be represented as symbolic language information.

It MUST NOT inherently represent:

- a filesystem path;
- a URL;
- a device address;
- a network endpoint;
- a physical location;
- a physical CPU;
- a physical GPU;
- a physical FPGA;
- a physical QPU;
- a physical qubit;
- a memory bank;
- a scheduler instance.

Dialect identity and target identity are separate concepts.

---

3. Dialect Identity Contract

Every stable dialect MUST have a unique identity.

The semantic identity consists of:

dialect namespace
+
dialect name

The version is separate:

dialect identity
+
dialect version

The language version remains separately identified:

Zamani language version
+
dialect identity/version

These MUST NOT be collapsed into one version number.

---

3.1 Dialect identity stability

Once a dialect identity becomes stable:

- its meaning MUST NOT silently change;
- its namespace MUST NOT be silently reassigned;
- its identity MUST NOT be reused for unrelated semantics;
- incompatible evolution MUST receive an explicit version transition;
- migration MUST be documented where practical.

---

3.2 Identity collision

Two dialects MUST NOT silently claim the same canonical identity.

If two independently discovered declarations claim the same identity, semantic validation MUST produce a deterministic conflict diagnostic.

Possible conflict classes include:

DIALECT_IDENTITY_CONFLICT
DIALECT_VERSION_CONFLICT
DIALECT_OWNER_CONFLICT
DIALECT_SOURCE_CONFLICT
DIALECT_REGISTRY_CONFLICT

The exact diagnostic representation belongs to the diagnostics subsystem.

---

4. Dialect Versioning

4.1 Dialect versions are independent

A dialect version is independent from:

- Zamani language version;
- compiler version;
- Rust version;
- runtime version;
- IR version;
- target version;
- hardware generation;
- device firmware;
- QPU generation.

For example:

Zamani language: 1.0.0
dialect: quantum::example
dialect version: 3.2.0
compiler: 0.x.y
Rust: 1.97.1

is valid.

---

4.2 Dialect version format

Stable dialects SHOULD use:

MAJOR.MINOR.PATCH

unless their owning specification explicitly defines another versioning scheme.

A dialect version parser MUST remain capable of representing version components without arbitrary language-level ceilings.

The compatibility system MUST NOT define:

MAX_VERSION
MAX_DIALECT_VERSION
MAX_VERSION_COMPONENT

as language semantics.

---

4.3 Dialect major versions

A dialect MAJOR version change is required when the dialect intentionally introduces incompatible stable semantic meaning.

Examples:

- changing the meaning of an existing dialect operation;
- removing stable dialect syntax;
- changing stable type semantics;
- changing stable effect semantics;
- changing resource requirement meaning;
- changing dialect-specific evaluation semantics;
- changing a stable lowering contract incompatibly.

---

4.4 Dialect minor versions

A dialect MINOR version MAY introduce backward-compatible functionality.

Examples:

- new optional constructs;
- new capabilities;
- new optional attributes;
- new additive operations;
- new interoperability mappings;
- new non-conflicting syntax;
- new domain features.

Existing stable dialect programs MUST retain their specified meaning.

---

4.5 Dialect patch versions

A dialect PATCH version MAY correct:

- diagnostics;
- documentation;
- conformance errors;
- parser bugs;
- compatibility metadata;
- implementation bugs that preserve semantics;
- deterministic behavior bugs that preserve semantics.

A patch version MUST NOT silently alter stable dialect semantics.

---

5. Core-Language Compatibility

A dialect MUST declare which Zamani language contract it requires.

Conceptually:

dialect
    requires
        Zamani language contract

The dialect MUST NOT silently assume a different core language version.

---

5.1 Core-language range

A dialect MAY support a range of Zamani language versions.

For example:

Zamani >= 1.0.0, < 2.0.0

The exact source syntax is owned by the existing versioning grammar.

This document defines the semantic requirement only.

---

5.2 Dialect/core compatibility

A dialect is compatible with a core language version only when:

1. the dialect syntax remains representable;
2. its AST mapping remains valid;
3. its semantic assumptions remain valid;
4. its capability model remains valid;
5. its resource semantics remain valid;
6. its lowering contract remains valid;
7. its required IR boundary remains valid;
8. its diagnostics remain meaningful;
9. no prohibited reinterpretation occurs.

Parsing successfully is insufficient.

---

5.3 Future core versions

A compiler MUST NOT assume that a future Zamani language version is compatible with a dialect merely because the dialect source still parses.

If future semantics are unknown:

UNKNOWN_CORE_VERSION

MUST be diagnosed unless an explicit compatibility contract covers the version.

---

6. Dialect Dependency Compatibility

A dialect MAY depend on another dialect.

The dependency graph is conceptually:

dialect A
   ↓
dialect B
   ↓
dialect C

There is no language-level limit on:

- dependency count;
- dependency depth;
- number of composed dialects;
- number of imported dialects.

Actual implementation resource limits remain implementation constraints.

---

6.1 Dependency identity

A dependency MUST identify:

- dialect identity;
- acceptable version range;
- compatibility requirements;
- optional feature requirements where applicable.

A dependency MUST NOT silently resolve to an incompatible version.

---

6.2 Dependency resolution

Dialect dependency resolution MUST be:

- deterministic;
- explicit;
- reproducible;
- cycle-aware;
- conflict-aware;
- version-aware.

The resolver MUST produce the same resolution for the same:

source
+
language version
+
dialect declarations
+
dependency metadata
+
compatibility policy

under the same resolver contract.

---

6.3 Dependency conflicts

A conflict MUST be diagnosed rather than silently selecting an incompatible interpretation.

Example:

dialect A requires B >= 1.0, < 2.0
dialect C requires B >= 3.0, < 4.0

If no compatible intersection exists:

DIALECT_DEPENDENCY_CONFLICT

MUST be produced.

The compiler MUST NOT silently choose one requirement.

---

6.4 Dependency cycles

Cycles MAY be represented syntactically if the grammar permits them.

Semantic resolution MUST detect cycles.

Example:

A → B
B → C
C → A

MUST produce a deterministic cycle diagnostic.

Cycle detection MUST NOT require fixed maximum dependency depth.

---

7. Dialect Composition

The existing dialect grammar permits multiple inherited/composed dialect references.

This contract makes that behavior normative.

A dialect MAY compose multiple dialects:

dialect hybrid::computing
    extends classical::numeric,
            quantum::standard,
            distributed::execution

The grammar MUST NOT define a maximum number.

---

7.1 Composition semantics

Composition MUST distinguish:

1. syntax composition;
2. semantic composition;
3. capability composition;
4. resource composition;
5. type composition;
6. effect composition;
7. lowering composition.

These MUST NOT automatically be treated as identical operations.

---

7.2 Composition conflicts

If two composed dialects define incompatible meanings for the same stable construct, the compiler MUST reject the composition unless the dialect contract explicitly defines deterministic resolution.

Possible conflict classes include:

DIALECT_SYNTAX_CONFLICT
DIALECT_NAME_CONFLICT
DIALECT_TYPE_CONFLICT
DIALECT_EFFECT_CONFLICT
DIALECT_CAPABILITY_CONFLICT
DIALECT_SEMANTIC_CONFLICT
DIALECT_LOWERING_CONFLICT
DIALECT_IR_CONFLICT

The compiler MUST NOT resolve semantic conflicts by:

- declaration order;
- filesystem order;
- hash-map iteration order;
- compiler implementation accident;
- target availability.

---

7.3 Explicit precedence

A dialect composition MAY define explicit precedence where the language specification permits it.

Precedence MUST be:

- declared;
- deterministic;
- versioned;
- testable.

Implicit precedence is prohibited for semantic conflicts.

---

8. Syntax Compatibility

Dialect syntax MUST integrate into the canonical Zamani grammar.

The architecture is:

Zamani.g4
    ↓
dialect façade
    ↓
dialect component grammar
    ↓
frontend AST

A dialect MUST NOT establish a competing program root.

---

8.1 Dialect syntax MUST NOT fork the language

A dialect MUST NOT introduce a separate:

- lexer architecture;
- parser architecture;
- source-unit architecture;
- AST architecture;
- semantic universe.

Dialect syntax is an extension of Zamani.

---

8.2 Keyword compatibility

Dialect-specific keywords SHOULD preferably be:

- contextual;
- namespaced;
- explicitly activated;
- feature-gated;
- otherwise designed to minimize collision with core identifiers.

A dialect MUST NOT silently turn a previously valid core identifier into a globally reserved keyword.

If a collision is unavoidable, the compatibility impact MUST be explicitly classified.

---

8.3 Token ownership

Dialect grammar MUST reuse canonical lexical ownership.

A dialect MUST NOT redefine core tokens merely because it needs a domain-specific meaning.

Examples requiring explicit lexical coordination include:

Question / QuestionMark
Ampersand / BitAnd

Any compatibility alias MUST identify:

- canonical token;
- dialect alias;
- source compatibility status;
- migration status;
- parser behavior.

---

8.4 Operator compatibility

A dialect MUST NOT silently change the precedence or associativity of a stable core operator.

Dialect operators SHOULD be:

- uniquely identifiable;
- namespaced where appropriate;
- feature-gated where necessary;
- mapped to an explicit AST operation.

---

9. AST Compatibility

Every stable dialect construct MUST have a predetermined AST mapping.

The required path is:

dialect grammar
      ↓
parse tree
      ↓
domain-neutral frontend AST
      ↓
semantic analysis

The dialect MUST NOT require a parallel frontend AST universe.

---

9.1 AST neutrality

The frontend AST MUST remain domain-neutral where required by the repository architecture.

A dialect-specific AST node MAY exist only where the repository's AST architecture explicitly permits it.

The existence of a quantum, HDL, AI, or hardware dialect MUST NOT force physical target information into the portable AST.

---

9.2 Forbidden portable AST dependencies

A portable dialect AST MUST NOT require:

- physical CPU IDs;
- GPU IDs;
- FPGA IDs;
- QPU IDs;
- physical qubit assignments;
- calibration records;
- fixed hardware topology;
- runtime handles;
- scheduler state;
- device-local addresses.

Those belong downstream.

---

9.3 AST compatibility classification

An AST change MUST identify whether it is:

ADDITIVE
OPTIONAL
NORMALIZED
MIGRATABLE
BREAKING
REMOVED

The classification MUST be recorded in compatibility metadata.

---

10. Semantic Compatibility

Dialect compatibility is primarily semantic.

A dialect is compatible only when the specified meaning remains compatible.

Syntax alone is insufficient.

---

10.1 Semantic compatibility includes

At minimum:

- type meaning;
- ownership;
- borrowing;
- effects;
- evaluation order;
- concurrency semantics;
- resource semantics;
- capability semantics;
- numerical semantics;
- determinism;
- error behavior;
- quantum semantics;
- measurement semantics;
- state semantics;
- hardware-intent semantics;
- interoperability semantics.

---

10.2 Semantic reinterpretation is prohibited

The following pattern is prohibited:

old dialect operation
        ↓
new compiler
        ↓
different meaning

unless the change is explicitly versioned and classified as breaking.

---

10.3 Semantic normalization

Different dialect syntax MAY normalize to the same semantic operation.

For example:

dialect-A operation
dialect-B operation
core Zamani operation

may all lower to the same semantic representation when their specified meaning is equivalent.

This is preferred over creating unnecessary duplicate IR forms.

---

11. Capability Compatibility

Capabilities are symbolic semantic contracts.

Examples:

quantum::dynamic_control
quantum::measurement
gpu::compute
tensor::compute
distributed::messaging
hardware::programmable_logic

A capability does NOT identify a specific machine.

---

11.1 Capability meaning

The statement:

requires quantum::dynamic_control

means:

«the selected realization must provide the required semantic capability.»

It does not mean:

use QPU X
use device Y
use physical qubit 17

---

11.2 Capability evolution

Changing the meaning of an existing stable capability is a compatibility-sensitive change.

Adding a new capability is normally additive.

Removing a capability requires:

- migration;
- replacement;
- compatibility classification;
- versioning;
- diagnostics.

---

11.3 Capability aliases

Aliases MAY exist.

An alias MUST identify:

- canonical capability;
- alias identity;
- version;
- lifecycle;
- migration path.

Two capability names MUST NOT silently acquire different meanings depending on the target.

---

12. Resource Compatibility

Resource requirements are not dialect versions.

A dialect may express:

requires qubits >= n
requires memory >= required_memory
requires capability("tensor.compute")

but compatibility MUST NOT turn those requirements into language ceilings.

---

12.1 No artificial resource limits

The dialect compatibility system MUST NOT define language-level limits such as:

MAX_DIALECTS
MAX_EXTENSIONS
MAX_CAPABILITIES
MAX_REQUIREMENTS
MAX_DEPENDENCIES
MAX_NAMESPACE_DEPTH
MAX_QUBITS
MAX_CPUS
MAX_CORES
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_QPUS
MAX_NODES
MAX_MEMORY
MAX_REGISTER_WIDTH
MAX_VECTOR_WIDTH
MAX_TENSOR_RANK
MAX_NETWORK_SIZE
MAX_DEVICE_COUNT
MAX_TIMELINES
MAX_PROGRAM_SIZE

These MUST NOT be hidden under alternate names either.

---

12.2 Scale semantics

The compatibility architecture MUST distinguish:

language expressiveness
        ≠
dialect expressiveness
        ≠
implementation capacity
        ≠
target capacity
        ≠
currently tested capacity

A test using a finite number of dialects or resources MUST NOT establish a language limit.

---

12.3 Finite representation

POCO-REAF does not require a false claim of mathematical infinity.

The correct guarantee is:

«Dialects MUST NOT impose arbitrary language-level ceilings. Actual execution is bounded by representational requirements, explicit program requirements, implementation capabilities, target capabilities, and resources available to the selected realization.»

---

13. Feature-Gate Compatibility

Dialect features MAY interact with:

"grammar/compatibility/feature-gates.md"

Feature gates MUST represent feature availability/lifecycle.

They MUST NOT be used to encode:

- hardware capacity;
- physical device identity;
- topology;
- scheduling;
- routing;
- runtime allocation.

---

13.1 Dialect feature identity

Every gated dialect feature SHOULD have a stable feature identity.

The feature identity MUST remain independent from:

- source file location;
- compiler implementation type;
- hardware target;
- Rust version.

---

13.2 Experimental dialect features

Experimental dialect features MUST explicitly identify:

- feature identity;
- dialect identity;
- dialect version;
- status;
- core-language compatibility;
- syntax;
- AST mapping;
- semantic mapping;
- IR mapping;
- capability requirements;
- resource requirements;
- migration policy;
- compatibility class;
- tests.

Grammar presence alone MUST NOT make an experimental feature stable.

---

14. Vendor Dialects

Vendor dialects are permitted.

They MUST remain extensions of Zamani rather than silently becoming vendor-specific core language.

A vendor dialect MAY describe:

- vendor-specific operations;
- vendor-specific capabilities;
- vendor-specific interoperability;
- vendor-specific optimization hints;
- vendor-specific execution features.

---

14.1 Vendor dialect hardware independence

A vendor dialect MUST distinguish:

vendor semantic capability

from:

specific physical device

For example:

vendor::quantum::dynamic_measurement

may identify a semantic capability.

It MUST NOT inherently mean:

device = vendor-device-123

unless the construct is explicitly part of a downstream target description rather than portable dialect semantics.

---

14.2 Vendor dialect compatibility

Vendor dialects MUST declare:

- vendor namespace;
- dialect identity;
- version;
- supported Zamani language range;
- dependencies;
- capabilities;
- compatibility;
- lifecycle;
- migration policy.

---

15. Experimental Dialects

Experimental dialects MAY exist without being stable.

They MUST be explicitly classified.

Recommended lifecycle:

PROPOSED
   ↓
DESIGNED
   ↓
EXPERIMENTAL
   ↓
IMPLEMENTED
   ↓
STABLE
   ↓
DEPRECATED
   ↓
REMOVED

An experimental dialect MUST NOT be represented as stable merely because:

- grammar exists;
- parser accepts it;
- a compiler branch supports it;
- a vendor implements it.

---

15.1 Experimental compatibility

Experimental dialects MUST NOT provide implicit long-term compatibility guarantees.

However, they still require:

- deterministic behavior;
- explicit versioning;
- explicit diagnostics;
- explicit feature status;
- AST contracts;
- semantic contracts;
- migration information where practical.

---

16. Dialect Deprecation

Dialect deprecation is governed by:

"grammar/compatibility/deprecated.md"

This document defines the dialect-specific compatibility consequences.

A deprecated dialect MUST retain enough identity information for deterministic migration.

Deprecation metadata SHOULD include:

dialect identity
dialect version
deprecation status
reason
replacement
migration path
first deprecated version
removal eligibility

---

16.1 Deprecation is not removal

A deprecated dialect is not automatically removed.

The compiler MAY:

- accept it;
- warn;
- provide migration;
- reject it in a later version.

These transitions MUST be explicitly documented.

---

16.2 Deprecation MUST preserve semantics

A migration MUST preserve source semantics where the replacement contract promises compatibility.

A migration MUST NOT silently change:

- resource requirements;
- quantum measurement semantics;
- effect behavior;
- ownership;
- type meaning;
- concurrency meaning;
- hardware-independent program intent.

---

17. Migration

Dialect migration follows:

"grammar/compatibility/migrations.md"

A migration MUST identify:

1. source dialect;
2. source version;
3. destination dialect;
4. destination version;
5. changed constructs;
6. semantic differences;
7. automated transformations where available;
8. manual transformations;
9. diagnostics;
10. compatibility classification;
11. tests.

---

17.1 Migration classes

Dialect migration MAY be:

LEXICAL
SYNTAX
AST
TYPE
EFFECT
SEMANTIC
CAPABILITY
RESOURCE
IR
INTEROPERABILITY
DEPRECATION
REMOVAL

A migration MUST NOT describe a semantic change as merely syntactic when meaning changes.

---

18. Canonical IR Compatibility

Dialects MUST converge into the repository's canonical semantic architecture.

A dialect MUST NOT create a duplicate canonical IR merely because it introduces a new syntax.

---

18.1 Quantum dialect boundary

Quantum dialects MUST eventually lower through:

dialect syntax
      ↓
frontend AST
      ↓
semantic quantum representation
      ↓
quantum::ir

"quantum::ir" remains the canonical quantum semantic boundary.

A dialect MUST NOT introduce:

dialect-specific quantum IR

as a competing canonical semantic authority.

---

18.2 Classical dialect boundary

Classical dialects MUST lower into the canonical classical semantic/IR architecture defined elsewhere.

---

18.3 HDL/hardware dialect boundary

HDL and hardware dialects MUST preserve the separation between:

hardware intent

and:

physical realization

The dialect MUST NOT embed universal fixed hardware limits.

---

18.4 Hybrid dialect boundary

Hybrid dialects MAY combine classical and quantum semantics.

They MUST preserve the common semantic boundary:

classical semantics
        +
quantum semantics
        ↓
canonical semantic representation
        ↓
classical IR / quantum::ir / other canonical boundaries

---

19. QEC Compatibility Boundary

Dialect compatibility MUST NOT become QEC implementation.

A quantum dialect MAY express:

error-correction intent
fault-tolerance requirements
noise assumptions
reliability requirements

but:

- QEC algorithms remain owned by QEC;
- code construction remains owned by QEC;
- correction strategy remains downstream;
- physical implementation remains downstream.

---

20. ZQN Compatibility Boundary

Dialect compatibility MUST NOT implement ZQN.

A dialect MAY describe fault/noise-related semantic requirements.

ZQN remains responsible for fault/noise semantics according to the repository architecture.

Dialect compatibility MUST ensure that dialect changes do not silently change established ZQN meaning.

---

21. Routing Compatibility Boundary

A dialect MUST NOT make routing decisions part of portable dialect semantics unless routing is explicitly observable language semantics.

For quantum programs:

logical operation

is distinct from:

physical mapping

For example:

logical q0

MUST NOT be compatibility-equivalent to:

physical qubit 17

The latter is downstream target realization.

---

22. Scheduling Compatibility Boundary

A dialect MAY express timing or ordering requirements.

It MUST NOT embed a scheduler implementation.

Changing a scheduler MUST NOT require dialect migration when the observable program semantics remain unchanged.

---

23. Hardware Compatibility

Dialect compatibility MUST remain independent from hardware generations.

A new:

- CPU;
- GPU;
- FPGA;
- ASIC;
- QPU;
- accelerator;
- cluster;
- network topology;

MUST NOT require a dialect major version merely because the hardware is new.

A hardware capability MAY introduce a new target capability.

That is not automatically a dialect semantic change.

---

23.1 Hardware realization

The required separation is:

dialect semantics
        ↓
program requirements
        ↓
capability resolution
        ↓
resource resolution
        ↓
target realization

The prohibited reverse dependency is:

hardware limit
        ↓
dialect meaning
        ↓
language semantics

---

24. POCO-REAF Compatibility

Dialect compatibility MUST preserve:

Program Once
      ↓
Portable Dialect Semantics
      ↓
Canonical Semantic Representation
      ↓
Compile Once
      ↓
Target Adaptation
      ↓
Run Everywhere
      ↓
Run Anywhere
      ↓
Run Forever

POCO-REAF does not mean that one machine-code binary is guaranteed to execute unchanged on every architecture.

It means that source-level semantics remain portable and can be realized by compatible future implementations.

---

24.1 Future hardware

A future hardware platform SHOULD be able to consume existing dialect semantics through:

existing source
      ↓
existing dialect contract
      ↓
future compiler
      ↓
future lowering
      ↓
future target

without requiring a source rewrite merely because the hardware is new.

---

24.2 Future computational paradigms

A future computational paradigm SHOULD be introduced through:

dialect
+
capabilities
+
resource contracts
+
semantic model
+
canonical IR integration

rather than requiring a fork of the Zamani language.

---

25. Interoperability Compatibility

A dialect MAY interoperate with:

- OpenQASM;
- QIR;
- LLVM-based systems;
- MLIR-based systems;
- HDL formats;
- foreign languages;
- external data formats;
- vendor representations.

Interoperability formats are not automatically canonical Zamani semantics.

The translation contract MUST specify:

source semantics
      ↓
translation
      ↓
destination semantics

and MUST identify whether translation is:

lossless
lossy
partial
approximate
target-dependent
implementation-defined

---

25.1 Lossy translation

A lossy translation MUST NOT be represented as semantic equivalence.

The compiler MUST identify the loss where it affects observable semantics.

---

25.2 Dialect import/export

Importing an external dialect or format MUST NOT automatically grant stable Zamani compatibility.

The imported semantics MUST be validated against the applicable dialect contract.

---

26. Deterministic Compatibility Resolution

Dialect compatibility resolution MUST be deterministic.

Given the same:

source
language version
dialect declarations
dialect versions
dependencies
feature gates
compatibility metadata

the compatibility resolver MUST produce the same result under the same implementation contract.

---

26.1 Deterministic ordering

Compatibility resolution MUST NOT depend on:

- filesystem enumeration order;
- hash-map iteration order;
- network response ordering;
- machine-specific discovery order;
- thread scheduling;
- target enumeration order.

If multiple compatible resolutions exist, the language/compiler specification MUST define how the resolution is selected or require explicit disambiguation.

---

26.2 Ambiguity

If compatibility cannot be determined deterministically, compilation MUST fail with a structured diagnostic.

The compiler MUST NOT silently choose one interpretation.

---

27. Compatibility Classes

A dialect relationship SHOULD be classified using explicit dimensions.

Recommended classification:

EXACT
BACKWARD_COMPATIBLE
FORWARD_COMPATIBLE
SOURCE_COMPATIBLE
SYNTAX_COMPATIBLE
AST_COMPATIBLE
SEMANTICALLY_COMPATIBLE
IR_COMPATIBLE
ARTIFACT_COMPATIBLE
RUNTIME_COMPATIBLE
TARGET_COMPATIBLE
MIGRATABLE
EXPERIMENTAL
DEPRECATED
INCOMPATIBLE
UNKNOWN

These labels are compatibility classifications, not feature-quality ratings.

---

27.1 "UNKNOWN" is not "COMPATIBLE"

If compatibility cannot be established, it MUST NOT be reported as compatible.

The implementation MUST distinguish:

compatible

from:

unknown

and:

incompatible

---

28. Compatibility Declaration Semantics

The existing "registration.g4" permits declarative compatibility information.

That syntax represents metadata.

It does not itself prove compatibility.

The semantic pipeline is:

compatibility declaration
        ↓
parse
        ↓
AST
        ↓
semantic compatibility resolver
        ↓
compatibility result

The grammar MUST NOT implement compatibility algorithms.

---

28.1 Compatibility declaration MUST identify scope

A compatibility declaration SHOULD be interpreted against an explicit target such as:

dialect
version
feature
core language
AST contract
semantic contract
IR contract
artifact
runtime

A generic compatibility statement MUST NOT be interpreted as universally compatible across every layer.

---

28.2 Compatibility declarations MUST NOT override reality

A dialect MUST NOT declare itself compatible with:

Zamani 2.0

and thereby force a compiler to accept incompatible semantics.

Compatibility declarations are claims/contracts to be validated, not authority to bypass validation.

---

29. Dialect Metadata

Dialect metadata MAY include:

identity
version
owner
stability
language compatibility
dependencies
capabilities
requirements
extensions
syntax
semantics
lowering
compatibility
deprecation
documentation
provenance

The metadata system MUST remain extensible.

---

29.1 Unknown metadata

Unknown metadata MAY be preserved structurally where the grammar and tooling permit it.

However, unknown metadata MUST NOT be treated as known semantics.

This distinction is required for forward compatibility.

---

30. Dialect Ownership and Provenance

A stable dialect SHOULD identify its ownership/provenance.

Ownership metadata MUST NOT be used to bypass compatibility.

A dialect remains subject to the same:

- semantic validation;
- compatibility rules;
- safety rules;
- migration rules;
- lifecycle rules.

---

31. Security Compatibility

Dialect compatibility MUST consider security semantics.

A dialect MUST NOT silently weaken:

- authorization;
- identity;
- provenance;
- capability enforcement;
- isolation;
- secret handling;
- integrity guarantees.

A security-semantic change MUST be compatibility-classified.

---

32. Determinism and Reproducibility

Dialect resolution SHOULD be reproducible.

A reproducible compilation record SHOULD identify:

Zamani language version
dialect identities
dialect versions
dependency versions
feature-gate state
compatibility decisions
semantic configuration

Target realization MAY vary when the program explicitly permits target adaptation.

The language semantics MUST remain stable.

---

33. Artifact Compatibility

Artifacts produced from dialect-enabled programs SHOULD carry sufficient metadata to identify:

- Zamani language version;
- dialect identities;
- dialect versions;
- relevant feature states;
- artifact format version;
- applicable IR version.

An artifact MUST NOT be interpreted under an incompatible dialect contract without explicit translation or migration.

---

34. ABI Compatibility

ABI compatibility is distinct from dialect compatibility.

A dialect MAY specify ABI requirements.

ABI details remain owned by the appropriate interoperability/compiler subsystem.

A change in ABI MUST NOT automatically imply a dialect language-version change unless observable language semantics change.

---

35. Runtime Compatibility

Runtime compatibility is separate from dialect compatibility.

A runtime MUST NOT silently reinterpret a dialect semantic contract.

If a runtime lacks required capabilities, the failure MUST be classified as:

- runtime incompatibility;
- capability failure;
- resource failure;
- target failure;

rather than silently changing the source meaning.

---

36. Target Compatibility

Target compatibility answers:

«Can this target realize the already-defined program semantics?»

It does not answer:

«What does this dialect mean?»

The order is:

dialect semantics
      ↓
program semantics
      ↓
requirements
      ↓
capabilities
      ↓
resources
      ↓
target realization

---

37. Compatibility and Simulation

A dialect program MAY execute on a simulator when the simulator implements the required semantics.

Changing:

QPU

to:

quantum simulator

MUST NOT by itself require a different dialect version.

Likewise, moving between:

CPU
GPU
FPGA
ASIC
QPU
HPC
cluster
cloud
future accelerator

does not automatically change dialect semantics.

---

38. Compatibility and Heterogeneous Execution

A dialect-enabled program MAY span multiple computational domains.

For example:

classical
+
quantum
+
AI
+
HDL
+
distributed

may coexist under one Zamani language version.

Each dialect remains versioned independently while the program maintains one coherent source-level language contract.

---

39. Diagnostics

Dialect compatibility failures MUST be structured and deterministic.

Diagnostics SHOULD identify:

error class
dialect identity
dialect version
required language version
actual language version
dependency
required version
available version
compatibility dimension
source span
migration/replacement where applicable

---

39.1 Required diagnostic categories

At minimum, implementations SHOULD distinguish:

UNKNOWN_DIALECT
DIALECT_VERSION_UNSUPPORTED
DIALECT_CORE_VERSION_INCOMPATIBLE
DIALECT_DEPENDENCY_MISSING
DIALECT_DEPENDENCY_CONFLICT
DIALECT_DEPENDENCY_CYCLE
DIALECT_IDENTITY_CONFLICT
DIALECT_SYNTAX_CONFLICT
DIALECT_AST_CONFLICT
DIALECT_TYPE_CONFLICT
DIALECT_EFFECT_CONFLICT
DIALECT_CAPABILITY_CONFLICT
DIALECT_RESOURCE_CONFLICT
DIALECT_SEMANTIC_CONFLICT
DIALECT_IR_CONFLICT
DIALECT_RUNTIME_INCOMPATIBLE
DIALECT_TARGET_INFEASIBLE
DIALECT_FEATURE_UNAVAILABLE
DIALECT_FEATURE_EXPERIMENTAL
DIALECT_FEATURE_DEPRECATED
DIALECT_MIGRATION_REQUIRED
DIALECT_COMPATIBILITY_UNKNOWN

Exact numeric/error-code ownership belongs to the diagnostics system.

---

40. Negative Compatibility Rules

The compiler MUST reject:

- unknown required stable dialects;
- unsupported dialect versions;
- incompatible core-language ranges;
- incompatible dependency ranges;
- dependency cycles;
- unresolved semantic conflicts;
- incompatible AST contracts;
- incompatible IR contracts;
- incompatible stable semantics;
- ambiguous dialect resolution;
- undeclared experimental semantics where declaration is required;
- removed dialect features without migration;
- contradictory capability contracts;
- incompatible dialect compositions.

The compiler MUST NOT silently downgrade or reinterpret such cases.

---

41. Scalability Contract

The dialect compatibility system MUST scale from the smallest useful program to arbitrarily large programs subject only to actual representation and available resources.

There MUST be no language-level maximum for:

- dialect count;
- dependency count;
- composition count;
- extension count;
- capability count;
- requirement count;
- metadata count;
- namespace depth;
- version-expression complexity;
- compatibility relationships;
- dialect members;
- source size;
- number of computational domains.

---

41.1 No hard-coded scaling assumptions

The implementation MUST NOT convert convenient test or implementation values into language restrictions.

For example:

tested with 10 dialects

does not mean:

maximum 10 dialects

Likewise:

tested with N dependencies

does not define:

MAX_DEPENDENCIES = N

---

41.2 Resource exhaustion

If the implementation cannot process a program because of actual resource exhaustion, the failure MUST be distinguished from language incompatibility.

Examples:

out of memory
compiler resource exhaustion
target resource exhaustion
execution resource exhaustion

MUST NOT be reported as:

dialect incompatible

unless the dialect contract itself is incompatible.

---

42. Rust 1.97 / 1.97.1 Requirements

The compatibility implementation MUST support:

Rust 1.97
Rust 1.97.1
Rust edition 2021

The implementation MUST use safe Rust.

No dialect compatibility feature may require:

unsafe

The compatibility subsystem MUST be implementable using safe:

- ownership;
- borrowing;
- enums;
- structs;
- collections;
- iterators;
- deterministic algorithms;
- error types;
- parser/AST infrastructure.

---

42.1 ANTLR boundary

"grammar/dialects/*.g4" files MUST remain declarative grammar.

They MUST NOT embed:

- Rust actions;
- unsafe code;
- filesystem operations;
- network operations;
- hardware callbacks;
- runtime callbacks;
- compatibility algorithms.

Compatibility algorithms belong downstream in semantic/compiler tooling.

---

43. Integration with "grammar/dialects/dialects.g4"

"dialects.g4" remains the public composition façade.

This document requires that it:

1. expose the dialect boundary;
2. compose independently owned dialect grammar components;
3. avoid duplicate compatibility rules;
4. avoid defining semantic compatibility algorithms;
5. preserve open-world dialect identities;
6. preserve the single-language architecture.

No compatibility policy in this file requires replacing the existing façade.

---

44. Integration with "grammar/dialects/registration.g4"

"registration.g4" remains responsible for registration syntax.

It already supports structural concepts for:

- registration;
- imports;
- use;
- extension;
- requirements;
- provisions;
- compatibility metadata;
- deprecation metadata;
- syntax descriptors;
- semantic descriptors;
- lowering descriptors;
- open properties.

This document defines how those structures are interpreted semantically.

The grammar MUST NOT be expanded merely to encode every future compatibility concept as a new reserved keyword.

---

45. Integration with "grammar/dialects/versioning.g4"

Dialect version syntax MUST remain structurally owned by the dialect/versioning grammar.

Semantic version comparison belongs to compatibility infrastructure.

This separation is mandatory:

grammar
    = structure

semantic compatibility
    = meaning

---

46. Integration with "grammar/dialects/compatibility.g4"

If "grammar/dialects/compatibility.g4" is present, it MUST own concrete compatibility declaration syntax.

This file owns the meaning and policy of those declarations.

The two files MUST NOT duplicate ownership.

---

47. Integration with "grammar/compatibility/versions.md"

"versions.md" owns general version policy.

This document specializes that policy for dialects.

Therefore:

versions.md
    ↓
general version rules

dialect-compatibility.md
    ↓
dialect-specific application

If the two conflict:

- "versions.md" owns general language-version policy;
- this document owns dialect-specific compatibility application.

Neither may redefine the other's domain.

---

48. Integration with "grammar/compatibility/migrations.md"

"migrations.md" owns migration procedure.

This document identifies when dialect migration is required.

The migration implementation MUST consume:

dialect identity
source version
destination version
compatibility classification
semantic changes

---

49. Integration with "grammar/compatibility/deprecated.md"

"deprecated.md" owns lifecycle.

This document defines dialect-specific compatibility consequences of deprecation.

A dialect MUST NOT remain silently stable after being marked deprecated.

---

50. Integration with "grammar/compatibility/feature-gates.md"

Feature gates own feature availability.

Dialect compatibility consumes feature-gate results.

The dependency direction is:

dialect
   ↓
feature requirements
   ↓
feature-gate resolution
   ↓
compatibility validation

Feature gates MUST NOT become hardware selectors.

---

51. Integration with "grammar/compatibility/compatibility-matrix.md"

The compatibility matrix owns cross-layer relationships.

This document provides the detailed dialect contract used by that matrix.

The matrix MUST identify at least:

core language
dialect
dialect version
syntax
AST
semantic
capability
resource
IR
runtime
target

for dialect compatibility claims.

---

52. Integration with "grammar/spec/compatibility.md"

"grammar/spec/compatibility.md" remains the general normative compatibility specification.

This file specializes that contract for dialects.

The relationship is:

spec/compatibility.md
        ↓
general compatibility semantics
        ↓
compatibility/dialect-compatibility.md
        ↓
dialect-specific operational contract

---

53. Integration with "grammar/grammar.md"

"grammar.md" remains implementation-conformance documentation.

It MUST report dialect status accurately.

For every dialect feature it SHOULD distinguish:

SPECIFIED
IMPLEMENTED
PARTIALLY IMPLEMENTED
PLANNED
DEPRECATED
REMOVED

The presence of a dialect grammar rule MUST NOT automatically cause "STABLE" status.

---

54. Integration with "grammar/Zamani-Grammar.md"

"Zamani-Grammar.md" remains the historical/extended design source.

A dialect described there MUST NOT become stable solely because it appears there.

Promotion remains:

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
IR integration
   ↓
tests
   ↓
stable

---

55. Integration with "grammar/Zamani.g4"

"Zamani.g4" remains the canonical ANTLR composition root.

Dialect compatibility MUST NOT create another root grammar.

The integration is:

Zamani.g4
    ↓
dialect façade
    ↓
dialect grammar

not:

Zamani.g4
    +
dialect parser
    +
separate language parser

---

56. Integration with Lexer

Dialect lexical extensions MUST conform to the canonical lexer contract.

The lexer MUST remain deterministic.

Dialect-specific lexical changes MUST be evaluated for:

- identifier collisions;
- keyword collisions;
- token identity;
- Unicode behavior;
- operator conflicts;
- literal conflicts.

---

57. Integration with Parser

The parser MUST produce deterministic parse structures for dialect-enabled source.

Dialect compatibility MUST NOT depend on:

- parser implementation ordering;
- alternative ordering accidentally changing semantics;
- hidden parser state;
- target discovery.

---

58. Integration with Frontend AST

Every stable dialect feature MUST have:

grammar rule
AST mapping
source-span mapping
semantic mapping

before it is considered complete.

---

59. Integration with Semantic Analysis

Semantic analysis owns:

- dialect lookup;
- version resolution;
- dependency resolution;
- compatibility checking;
- composition;
- conflict detection;
- feature-gate validation;
- capability validation;
- resource validation;
- semantic normalization.

The grammar MUST NOT perform these operations.

---

60. Integration with Canonical IR

Every dialect feature that reaches compilation MUST have a defined canonical representation.

The dialect layer MUST NOT create an unnecessary IR island.

The required principle is:

dialect
   ↓
common semantic model
   ↓
canonical IR

---

61. Integration with Quantum

Quantum dialects MUST converge through:

dialect syntax
      ↓
domain-neutral AST
      ↓
quantum semantic analysis
      ↓
quantum::ir

The dialect compatibility layer MUST NOT create:

- a second quantum IR;
- a second physical-qubit model;
- fixed gate enumerations;
- hardware-specific quantum limits.

---

62. Integration with Classical Computing

Classical dialects MUST preserve common:

- type semantics;
- expression semantics;
- ownership;
- effects;
- resource contracts;
- canonical representation.

A classical dialect MUST NOT hard-code a particular CPU generation as part of portable language meaning.

---

63. Integration with HDL

HDL dialects MUST distinguish:

hardware intent

from:

specific implementation

For example:

parameterized width

is distinct from:

universal fixed register width

The dialect compatibility layer MUST preserve that distinction.

---

64. Integration with AI and Tensor Computing

AI/tensor dialects MUST remain independent of specific frameworks.

A dialect may express:

- tensors;
- models;
- training;
- inference;
- differentiation;
- probabilistic operations;
- accelerator capabilities.

Framework implementation remains downstream.

---

65. Integration with Distributed Computing

Distributed dialects MUST remain independent of fixed node counts.

The dialect may express:

replication
partitioning
communication
consistency
collective computation
fault tolerance

but MUST NOT define a universal node maximum.

---

66. Integration with Networking

Networking dialects MAY express:

- protocol requirements;
- communication semantics;
- endpoint abstractions;
- streaming;
- distributed communication.

A dialect MUST NOT turn a source-level protocol abstraction into a physical network address unless explicitly operating inside a target-specific layer.

---

67. Integration with Security

Security dialects MUST preserve:

- identity semantics;
- authorization semantics;
- capability semantics;
- provenance;
- integrity;
- isolation.

Security weakening MUST be treated as semantic incompatibility when it changes observable guarantees.

---

68. Integration with Interoperability

Dialect interoperability MUST specify whether an imported/exported representation preserves:

syntax
types
effects
resources
capabilities
semantics
IR meaning

An external format MUST NOT automatically become a Zamani dialect authority.

---

69. Integration with Validation

"grammar/validation/" MUST validate:

- dialect identity;
- duplicate dialect identities;
- version constraints;
- dependency consistency;
- dependency cycles;
- feature-gate state;
- compatibility declarations;
- syntax collisions;
- AST coverage;
- semantic coverage;
- IR coverage;
- scalability;
- hard-coded capacity assumptions;
- deterministic resolution.

---

70. Required Hard-Coding Audit

Dialect compatibility implementation MUST reject or flag universal capacity assumptions such as:

MAX_DIALECTS
MAX_EXTENSIONS
MAX_DEPENDENCIES
MAX_CAPABILITIES
MAX_REQUIREMENTS
MAX_NAMESPACE_DEPTH
MAX_QUBITS
MAX_CPUS
MAX_GPUS
MAX_FPGAS
MAX_QPUS
MAX_NODES
MAX_MEMORY
MAX_THREADS
MAX_TENSOR_RANK
MAX_REGISTER_WIDTH
MAX_NETWORK_SIZE
MAX_DEVICE_COUNT

The audit MUST also look for disguised equivalents.

For example:

dialect_count <= 32
dependency_count <= 64

is equally prohibited when presented as a language rule.

---

71. Test Contract

Every stable dialect MUST have:

positive tests
negative tests
boundary tests
scalability tests
compatibility tests
migration tests
deprecation tests
determinism tests
AST tests
semantic tests
IR tests

where applicable.

---

71.1 Positive tests

Must cover:

- registration;
- import;
- use;
- aliases;
- version constraints;
- composition;
- capabilities;
- requirements;
- extensions;
- compatibility metadata;
- vendor dialects;
- experimental dialects.

---

71.2 Negative tests

Must reject:

- unknown dialect;
- incompatible version;
- dependency conflict;
- dependency cycle;
- semantic conflict;
- incompatible core language;
- incompatible AST contract;
- incompatible IR contract;
- ambiguous composition;
- unsupported stable feature;
- invalid migration.

---

71.3 Boundary tests

Must cover:

- smallest valid dialect;
- empty optional metadata;
- one dependency;
- multiple dependencies;
- nested namespaces;
- multiple composed dialects;
- large metadata sets;
- large dependency graphs;
- version boundaries;
- major/minor/patch transitions.

No boundary test may imply an artificial language maximum.

---

71.4 Scalability tests

Scalability tests MUST demonstrate that implementation limits are not language limits.

They SHOULD vary:

dialect count
dependency graph size
composition size
extension count
capability count
metadata count
namespace depth
source size

The tests MUST record implementation/resource constraints separately from language semantics.

---

72. Compatibility Test Matrix

At minimum, compatibility tests MUST cover:

Source| Core language| Dialect| Expected result
valid| compatible| compatible| accept
valid| compatible| incompatible dialect version| reject
valid| unsupported| valid dialect| reject
valid| future unknown| valid dialect| reject unless explicitly supported
valid| compatible| experimental opt-in| accept when enabled
valid| compatible| deprecated| accept/warn according to policy
valid| compatible| removed| reject/migrate
valid| compatible| conflicting dialects| reject
valid| compatible| compatible composition| accept
valid| compatible| missing dependency| reject
valid| compatible| cyclic dependency| reject

---

73. Compatibility with Existing Source

A stable dialect implementation MUST preserve existing valid source according to its declared compatibility contract.

A dialect change MUST NOT silently alter unrelated core Zamani source.

For example, adding:

quantum::future

MUST NOT change the meaning of unrelated:

classical

programs.

---

74. Dialect Isolation

Dialect implementation MUST be isolated enough that an unused dialect cannot silently alter core semantics.

An unused dialect SHOULD NOT:

- redefine core operators;
- change core type meaning;
- change core evaluation;
- change unrelated keywords;
- change quantum semantics;
- change hardware semantics.

---

75. Dialect Activation

Activation MUST be explicit where required by the feature lifecycle.

A dialect SHOULD be activated through a defined source/module/project mechanism.

Implicit activation based solely on:

- target hardware;
- compiler discovery;
- installed vendor;
- runtime environment;

MUST NOT silently change source semantics.

---

76. Target-Driven Dialect Discovery

A compiler MAY discover that a target supports a dialect.

That discovery MUST NOT automatically modify the source program's language meaning.

Target discovery may contribute:

available capability
available lowering
available runtime
available backend

It MUST NOT redefine:

dialect semantics

---

77. Resource-Driven Compatibility

Resource availability MUST NOT decide whether a dialect is semantically compatible.

For example:

dialect compatible

and:

target has insufficient memory

are different results.

The correct result is:

dialect compatible
target resource-infeasible

when appropriate.

---

78. Capability-Driven Compatibility

Likewise:

dialect compatible

and:

required capability unavailable

are different results.

The compiler MUST preserve that distinction.

---

79. Deterministic Multi-Dialect Resolution

For:

A
B
C

the resolver MUST compute compatibility from declared relationships rather than incidental order.

If:

A + B

is valid but:

A + C

is not, the result MUST be determined from compatibility contracts.

---

80. No Hidden Compatibility Through Declaration Order

This is prohibited:

dialect A
dialect B

meaning one thing, while:

dialect B
dialect A

means another thing solely because the parser or resolver encountered declarations in a different order.

If ordering is semantically significant, it MUST be explicitly specified by the language contract.

---

81. No Hidden Compatibility Through Hardware

This is prohibited:

if GPU exists:
    dialect means A

if QPU exists:
    dialect means B

Hardware may determine realization capability.

It MUST NOT silently redefine dialect meaning.

---

82. No Hidden Compatibility Through Compiler Version

This is prohibited:

compiler version X:
dialect operation means A

compiler version Y:
same dialect/version/source means B

unless the language/dialect version contract explicitly permits the change.

---

83. Version Resolution Algorithm Contract

A production implementation SHOULD perform dialect resolution in this conceptual order:

1. parse core language version
2. parse dialect identities
3. resolve dialect versions
4. validate core-language compatibility
5. resolve dialect dependencies
6. detect dependency cycles
7. resolve feature gates
8. validate syntax compatibility
9. validate AST contracts
10. validate type/effect contracts
11. validate capability/resource contracts
12. validate semantic compatibility
13. validate canonical IR compatibility
14. produce deterministic compatibility result
15. continue to compiler lowering

Hardware discovery MUST NOT occur before semantic compatibility in a way that changes the language meaning.

---

84. Compatibility Result Model

The semantic compatibility layer SHOULD conceptually produce a structured result containing:

dialect_identity
dialect_version
language_version
compatibility_status
compatible_layers
incompatible_layers
unknown_layers
dependencies
feature_gates
capabilities
resource_requirements
migration_required
deprecation_status
diagnostics

The concrete Rust structure belongs to the semantic/compiler implementation.

---

85. No Fixed Compatibility Result Size

The implementation MUST NOT impose a language-level maximum on:

- compatible layers;
- dependencies;
- diagnostics;
- capabilities;
- requirements;
- compatibility records.

Actual collection/resource limits remain implementation constraints.

---

86. Safe Rust Implementation Guidance

The implementation SHOULD prefer safe deterministic structures such as:

enums
structs
owned values
borrowed references
Vec
slice
HashMap where deterministic output is explicitly controlled
BTreeMap/BTreeSet where ordered resolution is required
Result
Option
iterators

If unordered collections are used, externally observable diagnostic and resolution ordering MUST be normalized.

---

87. Completion Contract for This File

This file is complete only when all of the following are satisfied.

Identity

- [ ] Dialect identity is defined.
- [ ] Identity is open-world.
- [ ] Identity is distinct from target identity.
- [ ] Identity collision behavior is defined.

Versioning

- [ ] Dialect version is distinct from Zamani version.
- [ ] Major/minor/patch behavior is defined.
- [ ] Future versions are handled explicitly.
- [ ] Version ranges are semantically distinct from resources.

Compatibility

- [ ] Core-language compatibility is defined.
- [ ] Syntax compatibility is defined.
- [ ] AST compatibility is defined.
- [ ] Semantic compatibility is defined.
- [ ] IR compatibility is defined.
- [ ] Runtime compatibility is defined.
- [ ] Target compatibility is separated.
- [ ] Capability compatibility is separated.
- [ ] Resource compatibility is separated.

Composition

- [ ] Dependencies are defined.
- [ ] Dependency conflicts are defined.
- [ ] Dependency cycles are defined.
- [ ] Composition conflicts are defined.
- [ ] Deterministic resolution is required.

Lifecycle

- [ ] Experimental dialects are defined.
- [ ] Stable dialects are defined.
- [ ] Deprecated dialects are defined.
- [ ] Removed dialects are defined.
- [ ] Migration is defined.

Architecture

- [ ] "Zamani.g4" remains canonical.
- [ ] "dialects.g4" remains the façade.
- [ ] "registration.g4" remains registration syntax owner.
- [ ] frontend AST remains domain-neutral where required.
- [ ] canonical "quantum::ir" remains protected.
- [ ] QEC remains downstream.
- [ ] ZQN remains downstream.
- [ ] routing remains downstream.
- [ ] scheduling remains downstream.
- [ ] HAL remains downstream.

POCO-REAF

- [ ] Dialects do not select hardware.
- [ ] Dialects do not encode physical topology.
- [ ] Dialects do not encode fixed device identities.
- [ ] Future hardware can consume existing semantics.
- [ ] Future computational paradigms can be introduced through extensions.

Scalability

- [ ] No fixed dialect-count limit.
- [ ] No fixed dependency-count limit.
- [ ] No fixed namespace-depth limit.
- [ ] No fixed extension-count limit.
- [ ] No fixed capability-count limit.
- [ ] No fixed resource-count limit.
- [ ] No hidden hardware ceilings.
- [ ] Test capacity is separated from language capacity.
- [ ] Actual resource exhaustion is distinguished from compatibility failure.

Safety

- [ ] Rust 1.97 is supported.
- [ ] Rust 1.97.1 is supported.
- [ ] Rust 2021 is supported.
- [ ] Production implementation uses safe Rust.
- [ ] No "unsafe" requirement exists.

Testing

- [ ] Positive tests defined.
- [ ] Negative tests defined.
- [ ] Boundary tests defined.
- [ ] Scalability tests defined.
- [ ] Compatibility tests defined.
- [ ] Migration tests defined.
- [ ] Deprecation tests defined.
- [ ] Determinism tests defined.
- [ ] AST tests defined.
- [ ] Semantic tests defined.
- [ ] IR tests defined.

---

88. File Integration Contract

This file owns

- dialect compatibility policy;
- dialect compatibility dimensions;
- dialect/core-language compatibility;
- dialect version compatibility;
- dialect dependency compatibility;
- dialect composition compatibility;
- dialect lifecycle compatibility;
- dialect migration compatibility;
- dialect capability compatibility;
- dialect resource compatibility;
- dialect AST/semantic/IR compatibility;
- dialect forward/backward compatibility;
- dialect deterministic-resolution requirements.

This file does not own

- concrete dialect syntax;
- token definitions;
- core grammar;
- parser implementation;
- AST implementation;
- semantic implementation;
- version-comparison algorithms;
- migration algorithms;
- QEC;
- ZQN;
- routing;
- scheduling;
- HAL;
- runtime;
- hardware topology;
- physical-device capabilities.

---

89. Inputs

This file consumes contracts from:

grammar/DESIGN.md
grammar/specification/
grammar/spec/
grammar/compatibility/versions.md
grammar/compatibility/migrations.md
grammar/compatibility/deprecated.md
grammar/compatibility/feature-gates.md
grammar/compatibility/compatibility-matrix.md
grammar/dialects/README.md
grammar/dialects/dialects.g4
grammar/dialects/registration.g4

It also relies on the established frontend architecture:

lexer
parser
AST
semantic analysis
canonical IR
quantum::ir
compiler
runtime
HAL

---

90. Outputs

This document provides:

dialect compatibility semantics
dialect lifecycle compatibility
dialect version compatibility
dialect composition rules
dialect dependency rules
dialect migration requirements
dialect scalability guarantees
dialect POCO-REAF constraints
dialect integration contracts

---

91. Downstream Consumers

The contract is consumed by:

dialect semantic resolver
feature-gate resolver
version resolver
compatibility validator
AST validator
semantic analyzer
IR validator
compiler
migration tooling
diagnostic tooling
documentation generation
conformance tests
artifact validation
runtime compatibility validation

---

92. Final Architectural Invariant

The entire dialect architecture MUST preserve:

                    Zamani
                       │
                Core language
                       │
             ┌─────────┴─────────┐
             │                   │
          Dialect A           Dialect B
             │                   │
             └─────────┬─────────┘
                       │
              Common frontend AST
                       │
              Semantic validation
                       │
              Canonical semantics
                       │
          ┌────────────┼─────────────┐
          │            │             │
     Classical     quantum::ir    HDL/Hardware
          │            │             │
          └────────────┼─────────────┘
                       │
             Optimization/lowering
                       │
          ┌────────────┼─────────────┐
          │            │             │
       Routing     Scheduling      QEC/ZQN
          │            │             │
          └────────────┼─────────────┘
                       │
                      HAL
                       │
                Target realization

The forbidden architecture is:

Dialect
   ↓
separate language
   ↓
separate AST
   ↓
separate semantic universe
   ↓
separate quantum IR
   ↓
vendor hardware

---

93. Ultimate Compatibility Rule

The ultimate rule is:

«A Zamani dialect describes portable language meaning; it does not define the machine on which that meaning happens to execute.»

Therefore:

Dialect version
        ↓
Language semantics
        ↓
Portable program meaning
        ↓
Capabilities
        ↓
Resource requirements
        ↓
Target adaptation
        ↓
Physical realization

and never:

Physical hardware
        ↓
Hardware limitation
        ↓
Dialect meaning
        ↓
Source-language restriction

The compatibility system MUST preserve this separation at every scale.

A dialect must therefore be capable of participating in Zamani programs ranging from:

tiny computation

through:

embedded
CPU
multicore
GPU
FPGA
ASIC
accelerator
QPU
HPC
cluster
distributed
cloud
future computational substrate

without turning any particular present-day resource configuration into a permanent language limit.

---

94. POCO-REAF Final Statement

The production dialect architecture is complete only when:

Program Once
      ↓
Stable Zamani Semantics
      ↓
Versioned Dialect Semantics
      ↓
Canonical Semantic Representation
      ↓
Compile Once
      ↓
Capability / Resource Adaptation
      ↓
Target-Specific Lowering
      ↓
Run Everywhere
      ↓
Run Anywhere
      ↓
Run Forever

subject only to:

semantic compatibility
+
implementation support
+
capability availability
+
resource availability
+
physical constraints

and never to arbitrary language-level ceilings.

The resulting invariant is:

«One Zamani language. Many dialects. One semantic foundation. No artificial scale ceiling. No duplicate quantum IR. No hardware-driven language semantics. Deterministic compatibility. Safe Rust. Future-proof extension. POCO-REAF.»

---

95. Production Completion Gate

"grammar/compatibility/dialect-compatibility.md" MUST be considered production-ready only when the repository can demonstrate the following complete chain:

dialect specification
        ↓
dialect identity
        ↓
dialect version
        ↓
core-language compatibility
        ↓
dialect dependency resolution
        ↓
feature-gate validation
        ↓
dialect grammar
        ↓
lexer compatibility
        ↓
parser compatibility
        ↓
frontend AST
        ↓
semantic compatibility
        ↓
capability/resource validation
        ↓
canonical semantic representation
        ↓
canonical IR
        ↓
quantum::ir where applicable
        ↓
optimization/lowering
        ↓
routing/scheduling/resilience/QEC/ZQN
        ↓
HAL
        ↓
runtime
        ↓
target realization
        ↓
compatibility/conformance tests

No layer may silently bypass another layer's compatibility contract.

A dialect is not production-ready merely because it parses.

It is production-ready when its syntax, AST, semantics, capabilities, resources, IR boundary, lifecycle, versioning, migration, diagnostics, compatibility, scalability, and downstream realization are all explicitly connected and tested.

End of "grammar/compatibility/dialect-compatibility.md".
Zamani Compilation Grammar

Production Orchestrator, Ownership Contract, Integration Contract, Scalability Contract, and POCO-REAF Architecture

Path: "grammar/compile/README.md"
Repository: "Benwellonedge28/Zamani"
Language: Zamani
Grammar technology: ANTLR4-compatible grammar
Rust baseline: Rust 1.97 or later
Rust edition: 2021
Safety requirement: production Rust MUST use safe Rust; "unsafe" MUST NOT be used
Primary portability objective: "Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever" (POCO-REAF)

---

1. Purpose

"grammar/compile/" is the source-level compilation-intent subsystem of Zamani.

Its purpose is to provide one coherent, extensible, target-independent language interface through which a Zamani program may express:

- compilation intent;
- target intent;
- target-selection policy;
- compilation profiles;
- compiler features;
- compile-time evaluation;
- conditional compilation;
- specialization;
- optimization intent;
- lowering intent;
- code-generation intent;
- cross-compilation intent;
- artifact intent;
- caching intent;
- deterministic-build intent;
- reproducibility intent;
- provenance intent;
- deployment intent.

This directory is not the compiler implementation.

It does not:

- discover hardware;
- enumerate devices;
- allocate resources;
- execute compiler passes;
- implement optimization algorithms;
- implement lowering algorithms;
- construct physical machine code;
- route quantum operations;
- schedule hardware operations;
- perform quantum error correction;
- construct or execute ZQN;
- implement the HAL;
- execute programs;
- access hardware directly;
- impose universal hardware limits.

The central rule is:

«The compilation grammar describes intent. The compiler determines realization.»

---

2. Production Status

This README is the orchestration and ownership contract for every file under "grammar/compile/".

It is not itself a claim that every implementation behind these grammar files is already complete.

A compilation feature is production-complete only when the complete dependency chain exists:

specification
    ↓
lexical contract
    ↓
grammar
    ↓
lexer
    ↓
parser
    ↓
domain-neutral AST
    ↓
structural validation
    ↓
name/module resolution
    ↓
type analysis
    ↓
effect analysis
    ↓
resource analysis
    ↓
capability analysis
    ↓
contract analysis
    ↓
policy analysis
    ↓
portability analysis
    ↓
compilation semantic model
    ↓
canonical IR / semantic representation
    ↓
optimization
    ↓
specialization
    ↓
lowering
    ↓
routing / scheduling where applicable
    ↓
resilience / QEC / ZQN where applicable
    ↓
HAL
    ↓
backend / runtime
    ↓
artifact
    ↓
deployment where requested
    ↓
verification
    ↓
conformance tests

Parsing syntax successfully is therefore not sufficient for declaring a feature production-ready.

---

3. Mission

The compilation subsystem provides the language boundary between:

portable Zamani program intent

and:

compiler-controlled realization

The source program may express:

what must be true
what is preferred
what is permitted
what is prohibited
what may be specialized
what may be optimized
what must remain reproducible
what must remain deterministic
what artifacts are required
what target capabilities are necessary
what resources are required
what transformations are allowed

The compiler subsequently determines:

how
where
when
with which implementation
using which available resources
using which compatible target
through which lowering path
through which backend

This separation is fundamental to POCO-REAF.

---

4. The Compilation Architecture

The authoritative compilation path is:

Zamani source
      │
      ▼
canonical lexer
      │
      ▼
canonical parser
      │
      ▼
grammar/compile/compile.g4
      │
      ▼
domain-neutral AST
      │
      ▼
structural validation
      │
      ├── names/modules
      ├── types
      ├── effects
      ├── resources
      ├── capabilities
      ├── contracts
      ├── policies
      ├── provenance
      └── portability
      │
      ▼
semantic compilation model
      │
      ├───────────────────────┬────────────────────────┐
      ▼                       ▼                        ▼
 classical semantics     quantum semantics       HDL/hardware
      │                       │                        │
      │                  quantum::ir                   │
      │                       │                        │
      └───────────────────────┼────────────────────────┘
                              ▼
                         optimization
                              │
                              ▼
                         specialization
                              │
                              ▼
                           lowering
                              │
                    ┌─────────┴─────────┐
                    ▼                   ▼
                 classical          quantum
                    │                   │
                    │             decomposition
                    │             routing
                    │             scheduling
                    │             resilience
                    │             QEC
                    │             ZQN
                    │                   │
                    └─────────┬─────────┘
                              ▼
                             HAL
                              │
                              ▼
                          realization
                              │
                ┌─────────────┼─────────────┐
                ▼             ▼             ▼
              CPU/GPU       FPGA/ASIC      QPU
                │             │             │
                └─────────────┼─────────────┘
                              ▼
                    accelerator / HPC /
                  cluster / distributed /
                     edge / cloud / future

"grammar/compile/" owns only the source compilation-intent boundary.

---

5. The Single Composition Root

There MUST be exactly one canonical source-level compilation composition root.

That root is:

grammar/compile/compile.g4

The root Zamani grammar remains:

grammar/Zamani.g4

Therefore:

grammar/Zamani.g4
        │
        ▼
grammar/compile/compile.g4
        │
        ├── intent.g4
        ├── compilation.g4
        ├── profiles.g4
        ├── features.g4
        ├── feature-selection.g4
        ├── compile-time.g4
        ├── conditional-compilation.g4
        ├── target.g4
        ├── target-selection.g4
        ├── specialization.g4
        ├── optimization.g4
        ├── lowering.g4
        ├── code-generation.g4
        ├── cross-compilation.g4
        ├── artifacts.g4
        ├── caching.g4
        ├── deterministic-builds.g4
        ├── reproducibility.g4
        ├── provenance.g4
        └── deployment.g4

There MUST NOT be:

compile.g4 = language A
compilation.g4 = language B

There is one Zamani language and one compilation-intent model.

---

6. Why "compilation.g4" Exists

The repository currently contains both:

compile.g4
compilation.g4

This is an architectural risk because their names imply potentially competing ownership.

The production rule is:

"compile.g4"

Owns:

- canonical composition;
- public compilation dispatch;
- stable integration boundary;
- delegation to compilation feature grammars.

"compilation.g4"

Owns:

- subordinate compilation specification/orchestration structures that are not the public composition root;
- compatibility structures where required;
- compilation-plan composition where it is semantically distinct from the top-level "Compile" dispatch.

It MUST NOT create:

- a second root parser;
- a second compilation AST;
- a second compilation semantic model;
- a second IR;
- duplicate target syntax;
- duplicate optimization syntax;
- duplicate lowering syntax.

If a production audit demonstrates that a rule in "compilation.g4" duplicates a rule owned elsewhere, that duplicate rule MUST be removed or converted into a delegation/compatibility boundary.

---

7. Authority Model

The repository-wide authority chain is:

grammar/DESIGN.md
        ↓
grammar/specification/
        ↓
grammar/spec/
        ↓
grammar/Zamani.g4
        ↓
grammar/compile/*.g4
        ↓
src/lexer.rs
        ↓
src/parser.rs
        ↓
src/ast/
        ↓
semantic analysis
        ↓
canonical semantic representation
        ↓
canonical IR
        ↓
domain IR
        ↓
compiler
        ↓
backend/runtime

The responsibilities are distinct.

"grammar/DESIGN.md"

Owns repository-wide grammar architecture.

"grammar/specification/"

Owns normative human-readable language specifications.

"grammar/spec/"

Owns machine-oriented and feature-oriented contracts.

"grammar/Zamani.g4"

Owns canonical Zamani grammar composition.

"grammar/compile/*.g4"

Own source-level compilation syntax only.

"src/lexer.rs"

Owns the actual Rust lexical implementation.

"src/parser.rs"

Owns the actual Rust parser implementation.

"src/ast/"

Owns the domain-neutral frontend AST.

Semantic layer

Owns interpretation and validation of compilation intent.

Canonical IR

Owns compiler-independent program representation.

"src/quantum/ir/"

Owns the canonical quantum IR boundary.

Compiler/backend/runtime

Own realization.

No file under "grammar/compile/" may silently become authoritative over another layer.

---

8. Fundamental Ownership Rule

Every compilation grammar file MUST answer these questions independently:

What do I own?

What do I explicitly not own?

Which rules do I export?

Which rules do I consume?

Which lexer tokens do I consume?

Which AST representation receives my constructs?

Which semantic model interprets them?

Which resource/capability/effect systems consume them?

Which policy system constrains them?

Which canonical IR receives their meaning?

Which compiler subsystem realizes them?

Which runtime/backend subsystem consumes the result?

Which specification defines their meaning?

Which tests prove their correctness?

No file is considered complete until those boundaries are known.

---

9. Standard Feature Contract

Every ".g4" file under this directory MUST have a feature contract documenting:

PURPOSE
OWNS
DOES_NOT_OWN
PUBLIC_RULES
PRIVATE_RULES
LEXER_DEPENDENCIES
GRAMMAR_DEPENDENCIES
AST_CONTRACT
SEMANTIC_CONTRACT
TYPE_CONTRACT
EFFECT_CONTRACT
RESOURCE_CONTRACT
CAPABILITY_CONTRACT
CONTRACT_INTERACTION
POLICY_INTERACTION
PROVENANCE_CONTRACT
PORTABILITY_CONTRACT
IR_CONTRACT
QUANTUM_BOUNDARY
HDL_BOUNDARY
BACKEND_BOUNDARY
DIAGNOSTICS
POSITIVE_TESTS
NEGATIVE_TESTS
BOUNDARY_TESTS
SCALABILITY_TESTS
DETERMINISM_TESTS
REPRODUCIBILITY_TESTS
COMPATIBILITY
INTEGRATION
COMPLETION_CRITERIA

Where a category does not apply, the file MUST explicitly say:

Not applicable.

It must not leave ownership ambiguous.

---

10. Standard Dependency Contract

Each file MUST document:

DEPENDS_ON:
EXPORTS:
CONSUMED_BY:
AST_OWNER:
SEMANTIC_OWNER:
TYPE_OWNER:
EFFECT_OWNER:
RESOURCE_OWNER:
CAPABILITY_OWNER:
POLICY_OWNER:
PROVENANCE_OWNER:
IR_OWNER:
SPEC_OWNER:
TEST_OWNER:

Dependency direction MUST remain acyclic at the architecture level.

A grammar file MUST NOT import semantic implementation code.

A semantic implementation MUST NOT need to modify grammar syntax merely to discover target hardware.

---

11. Current Compile Directory

The current production orchestration surface is:

grammar/compile/
├── README.md
├── compile.g4
├── compilation.g4
├── intent.g4
├── profiles.g4
├── features.g4
├── feature-selection.g4
├── compile-time.g4
├── conditional-compilation.g4
├── target.g4
├── target-selection.g4
├── specialization.g4
├── optimization.g4
├── lowering.g4
├── code-generation.g4
├── cross-compilation.g4
├── artifacts.g4
├── caching.g4
├── deterministic-builds.g4
├── reproducibility.g4
├── provenance.g4
└── deployment.g4

These files form one subsystem.

No new compilation grammar root should be added merely because another compilation concern is introduced.

---

12. "compile.g4"

Owns

"compile.g4" is the canonical composition root.

It owns:

- public compilation grammar entry;
- compilation construct dispatch;
- composition of all compile subgrammars;
- stable rule names used by "grammar/Zamani.g4";
- integration of compilation clauses;
- compilation declaration boundaries.

Does not own

It does not own:

- individual target syntax;
- optimization algorithms;
- target discovery;
- resource discovery;
- hardware discovery;
- quantum operations;
- QEC;
- ZQN;
- HAL;
- backend code generation.

Integration

grammar/Zamani.g4
        ↓
Compile
        ↓
compile/*.g4

The root must remain deliberately thin.

Completion

Done when:

- there is exactly one public compilation composition path;
- every compile feature is reachable through that path;
- no feature has a competing root;
- parser ambiguity tests pass;
- all public rules have documented ownership.

---

13. "intent.g4"

Owns

General source-level compilation intent.

It is an adapter/boundary, not another grammar universe.

It may compose intent concerning:

- target;
- features;
- optimization;
- specialization;
- lowering;
- artifacts;
- reproducibility;
- deployment;
- provenance.

Does not own

It does not duplicate those feature grammars.

Integration

Compile
   ↓
CompileIntent
   ↓
feature-specific grammar

Completion

Done when it acts only as a stable delegation boundary and contains no duplicated semantic definitions.

---

14. "compilation.g4"

Owns

Subordinate compilation-specification structures where a compilation plan must be represented as a semantic unit.

It may represent:

- compilation-unit composition;
- compilation-plan structure;
- relationships between compilation-intent clauses;
- ordered or constrained compilation phases when such ordering is part of source semantics;
- reusable compilation specifications.

Does not own

It must not duplicate:

- target;
- optimization;
- specialization;
- lowering;
- artifacts;
- caching;
- reproducibility;
- deployment.

Integration

compile.g4
    ↓
compilation.g4
    ↓
feature grammars

It must never become a second root.

---

15. "profiles.g4"

Owns

Named, reusable compilation profiles.

A profile may describe semantic policy concerning:

- optimization;
- diagnostics;
- determinism;
- reproducibility;
- specialization;
- lowering;
- code generation;
- artifact production;
- portability;
- verification.

Does not own

It does not define:

- hardware inventories;
- physical targets;
- optimizer implementations;
- runtime configuration;
- device allocation.

Integration

profiles
   ↓
semantic profile
   ↓
compiler planning

Profiles must remain extensible.

No profile may secretly encode a universal physical machine.

---

16. "features.g4"

Owns

The syntax and semantic naming boundary for compilation features.

Features may represent capabilities of the language/toolchain such as:

classical
quantum
hybrid
hdl
distributed
tensor
accelerator
simulation
verification

Feature names are identifiers or extensible semantic names.

Critical rule

Do not create one grammar alternative for every future compiler feature.

Bad:

feature
    : CPU
    | GPU
    | FPGA
    | QPU
    | ...

Preferred:

feature(identifier)

with semantic resolution downstream.

This keeps the language open-ended.

---

17. "feature-selection.g4"

Owns

Selection and requirement of compilation features.

It distinguishes:

required feature
preferred feature
optional feature
disabled feature
conditional feature

Does not own

It does not implement feature discovery.

Integration

feature-selection
        ↓
semantic feature requirements
        ↓
capability/target analysis
        ↓
compiler plan

Unknown features must not require grammar redesign when they are represented through extensible semantic identifiers.

---

18. "compile-time.g4"

Owns

Source constructs whose evaluation or transformation is explicitly requested at compile time.

Examples include:

- compile-time expressions;
- compile-time assertions;
- compile-time conditions;
- compile-time evaluation;
- compile-time generation;
- compile-time specialization.

Does not own

It does not own general expressions.

Those remain under:

grammar/expressions/

Integration

compile-time syntax
        ↓
AST
        ↓
compile-time semantic validation
        ↓
safe evaluation/generation
        ↓
resulting semantic representation

Compile-time execution must be controlled, deterministic where required, resource-bounded by actual available resources, and governed by declared effects/capabilities/policies.

---

19. "conditional-compilation.g4"

Owns

Conditional inclusion or selection of compilation constructs based on declared semantic compilation information.

Valid sources of conditions include:

- language features;
- declared configuration;
- target capability predicates;
- compilation profile;
- language/toolchain version;
- explicitly supplied semantic facts;
- resource/capability predicates where the language permits them.

Does not own

It must not directly inspect:

- physical CPUs;
- physical GPU counts;
- physical QPU identifiers;
- filesystem state;
- environment variables;
- device inventories.

Such information must be represented by the appropriate downstream semantic context.

Integration

condition
   ↓
semantic predicate
   ↓
compilation context
   ↓
selected source branch

---

20. "target.g4"

Owns

Source-level target intent.

It can express:

- target classes;
- acceptable realization categories;
- target properties;
- target requirements;
- target constraints;
- target preferences;
- target hints;
- target capability requirements;
- portability intent.

Does not own

It does not own:

- physical device selection;
- device enumeration;
- hardware discovery;
- allocation;
- placement;
- routing;
- scheduling.

Correct abstraction

source intent
    ↓
acceptable realization

not:

source
    ↓
physical device #N

---

21. "target-selection.g4"

Owns

Selection policy among semantically acceptable target realizations.

It may represent:

- requirements;
- preferences;
- priorities;
- constraints;
- fallback rules;
- compatibility criteria;
- scoring objectives;
- failure policy;
- deterministic selection policy.

Does not own

It does not implement:

- discovery;
- ranking algorithms;
- allocation;
- hardware access.

The compiler/runtime infrastructure performs those operations.

Integration

target intent
      +
selection policy
      ↓
target-resolution request
      ↓
capability/resource resolver
      ↓
candidate realizations
      ↓
selected realization

---

22. Resource Integration

Compilation intent MUST consume the common resource model under:

grammar/resources/

The compilation grammar may express:

requires memory >= required_memory;
requires qubits >= required_qubits;
requires capability("tensor.compute");
requires capability("quantum.measurement");
requires topology(required_topology);

The precise syntax must follow the authoritative resource grammar rather than inventing another requirement language.

The compiler then resolves:

requirement
    ↓
capability/resource analysis
    ↓
candidate realization

No compilation grammar file may define a universal capacity ceiling.

---

23. Capability Integration

Capabilities are semantic properties.

Examples:

quantum.measurement
quantum.mid_circuit_measurement
tensor.compute
parallel.compute
distributed.communication
hdl.synthesis
simulation
native.execution

Capability identifiers must be extensible.

The compilation grammar must not require grammar edits every time a new accelerator, compiler feature, quantum technology, or hardware capability appears.

---

24. "optimization.g4"

Owns

Optimization intent.

It may represent:

- optimization objectives;
- optimization constraints;
- optimization preferences;
- optimization profiles;
- permitted transformations;
- prohibited transformations;
- approximation policy;
- optimization budgets;
- reproducibility requirements;
- deterministic requirements;
- pass-selection intent.

Does not own

It does not implement optimizer algorithms.

The architecture is:

optimization intent
       ↓
semantic optimization model
       ↓
optimization planner
       ↓
optimization passes

Existing compiler optimization infrastructure must consume this model rather than being duplicated by grammar code.

Optimization must preserve program semantics unless the program explicitly declares a permitted semantic relaxation.

---

25. "specialization.g4"

Owns

Specialization intent.

It covers specialization based on semantic information such as:

- known values;
- types;
- capabilities;
- resource properties;
- compilation profile;
- target constraints;
- domain information.

Does not own

It does not define physical implementation.

The same source program must remain semantically stable while the compiler specializes it for available resources.

---

26. "lowering.g4"

Owns

Source-level intent governing semantic lowering.

It may express:

- permitted abstraction-level changes;
- required semantic preservation;
- representation preferences;
- decomposition preferences;
- lowering constraints;
- lowering policies.

Does not own

It does not implement:

- machine lowering;
- quantum decomposition;
- routing;
- scheduling;
- HDL synthesis;
- backend code generation.

Those remain downstream.

---

27. "code-generation.g4"

Owns

Source-level code-generation intent.

It may express requirements for:

- representation class;
- generated artifacts;
- ABI interaction;
- calling conventions;
- symbol visibility;
- debugging information;
- profiling information;
- linkage;
- external representation;
- code-generation policy.

Does not own

It does not generate machine code.

It must remain target-neutral.

---

28. "cross-compilation.g4"

Owns

Explicit cross-compilation intent.

It can represent:

- source environment;
- destination semantic environment;
- destination target intent;
- ABI requirements;
- compatibility requirements;
- artifact portability;
- cross-compilation policy.

Does not own

It does not perform cross compilation.

It does not discover or install toolchains.

Integration

source context
      ↓
cross-compilation intent
      ↓
semantic compatibility
      ↓
target/capability resolution
      ↓
lowering
      ↓
code generation

Cross-compilation must not turn target-specific facts into universal source-language restrictions.

---

29. "artifacts.g4"

Owns

Source-level artifact intent.

An artifact may represent a requested result of:

- compilation;
- analysis;
- verification;
- optimization;
- lowering;
- code generation;
- packaging;
- deployment preparation;
- interoperability.

It may express:

- artifact identity;
- artifact relationships;
- representation;
- format intent;
- dependencies;
- scope;
- lifetime;
- metadata;
- provenance requirements.

Does not own

It does not implement:

- storage;
- hashing;
- signing;
- packaging;
- caching;
- deployment;
- execution.

Integration

artifact intent
      ↓
artifact semantic model
      ↓
compiler/artifact subsystem
      ↓
artifact generation

---

30. "caching.g4"

Owns

Source-level intent governing reusable compilation results.

Caching must be based on semantic identity, not accidental machine identity.

Cache validity may depend on:

source identity
dependency identity
compiler identity
language version
grammar/AST version
semantic model version
IR version
dialect versions
relevant compilation policy
relevant target constraints
relevant capability assumptions
relevant toolchain inputs

Does not own

It does not:

- access cache storage;
- perform hashing;
- communicate with cache servers;
- inspect hardware;
- bypass semantic validation;
- implement eviction;
- decide cache validity independently.

Caching must never change program meaning.

---

31. "deterministic-builds.g4"

Owns

Source-level deterministic-build requirements.

It defines intent concerning:

- deterministic inputs;
- deterministic ordering;
- deterministic dependency resolution;
- deterministic artifact identity;
- deterministic compiler behavior;
- deterministic generated representations.

Does not own

It does not implement deterministic compilation.

Determinism belongs to the compiler/toolchain implementation and must be verified by tests.

A deterministic request MUST NOT pretend that nondeterministic external inputs can automatically become deterministic.

The compiler must report when declared determinism cannot be satisfied.

---

32. "reproducibility.g4"

Owns

Source-level reproducibility intent.

Reproducibility concerns whether equivalent compilation inputs and declared compilation context produce equivalent results according to the selected reproducibility contract.

It may express:

- reproducibility requirements;
- reproducibility constraints;
- reproducibility preferences;
- reproducibility inputs;
- reproducibility properties;
- reproducibility profiles.

Does not own

It does not implement:

- hashing;
- artifact comparison;
- dependency retrieval;
- environment capture;
- compiler execution;
- cache implementation.

Reproducibility consumes information from:

provenance
deterministic builds
artifacts
caching
compatibility
toolchain

---

33. "provenance.g4"

Owns

Source-level compilation provenance intent.

Provenance can describe:

source
input
dependency
origin
derivation
transformation
compiler
toolchain
profile
policy
target intent
artifact
decision
evidence
version
schema
scope

Does not own

It does not implement provenance storage or auditing.

Integration

Compilation provenance must be able to flow through:

source
 ↓
AST
 ↓
semantic model
 ↓
IR
 ↓
optimization
 ↓
lowering
 ↓
artifact
 ↓
deployment

This enables explanations of compiler decisions, reproducibility, debugging, scientific traceability, security auditing, and target realization.

---

34. "deployment.g4"

Owns

Source-level deployment intent at the compilation boundary.

It may describe:

- deployment class;
- artifact relationships;
- deployment constraints;
- deployment requirements;
- portability requirements;
- deployment policy;
- rollout/fallback intent where semantically appropriate.

Does not own

It does not:

- deploy to machines;
- contact infrastructure;
- provision nodes;
- allocate cloud resources;
- start processes;
- manage physical devices.

Deployment execution belongs to downstream tooling/runtime infrastructure.

---

35. Compile-Time and Runtime Separation

The compile subsystem MUST maintain a strict boundary between:

compile-time intent

and:

runtime behavior

Compilation may determine:

- static specialization;
- static optimization;
- artifact generation;
- compile-time conditions;
- target compatibility;
- resource feasibility where information is available.

Runtime may determine:

- dynamic resources;
- runtime availability;
- resilience state;
- dynamic scheduling;
- runtime adaptation;
- recovery;
- execution-time fallback.

A source-level compilation declaration must not accidentally become a runtime instruction.

---

36. Quantum Integration

The compilation grammar is domain-neutral.

Quantum-specific semantics belong under:

grammar/quantum/

and the compiler's canonical quantum semantic boundary:

quantum::ir

The integration is:

compile intent
       ↓
semantic model
       ↓
quantum operation semantics
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
resilience / QEC
       ↓
ZQN
       ↓
HAL
       ↓
QPU or simulator

The compilation grammar MUST NOT enumerate a fixed universe of quantum devices or physical qubits.

For example:

requires capability("quantum.measurement")
requires qubits >= required_qubits

describes requirements.

It does not select:

physical_qubit_37

as a universal language construct.

---

37. Classical Integration

Classical compilation consumes the same compilation model.

The pipeline is:

classical source semantics
       ↓
canonical semantic model
       ↓
classical IR
       ↓
optimization
       ↓
specialization
       ↓
lowering
       ↓
code generation
       ↓
target realization

The same source-level compilation intent can therefore be realized on:

- tiny systems;
- CPUs;
- multicore systems;
- vector systems;
- GPUs;
- accelerators;
- distributed systems;
- HPC systems;
- cloud systems;
- future architectures.

The grammar does not define the size of any of these systems.

---

38. HDL Integration

HDL compilation intent integrates with:

grammar/hdl/
grammar/hardware/
grammar/resources/
grammar/capabilities/
grammar/validation/

The source expresses hardware intent.

The downstream toolchain determines:

simulation
verification
synthesis
mapping
placement
timing
physical realization

The compilation grammar MUST NOT introduce artificial signal-width, device-count, memory-size, or topology ceilings.

---

39. Hybrid Integration

Hybrid programs may combine:

classical computation
quantum computation
AI computation
tensor computation
accelerator computation
HDL/hardware computation
distributed computation

Compilation intent must remain shared.

For example:

source
   ↓
semantic model
   ├── classical operations
   ├── quantum operations
   ├── tensor operations
   ├── hardware intent
   └── distributed operations

The compiler then selects appropriate domain representations.

Quantum portions converge on:

quantum::ir

rather than introducing another quantum IR owned by "grammar/compile/".

---

40. AI, Reasoning, Learning, Knowledge, and Adaptation Integration

Compilation syntax remains domain-neutral while integrating with the wider Zamani semantic architecture.

Compilation may consume semantic properties produced by:

- reasoning;
- knowledge;
- inference;
- learning;
- adaptation;
- uncertainty;
- evidence;
- provenance;
- policies.

For example, an optimization or target-selection decision may carry:

reason
evidence
confidence
provenance
policy

The compilation grammar does not need separate AI compiler syntax for each application.

Application-level capabilities remain extensible semantic constructs, libraries, dialects, or runtime systems.

---

41. Resource Abstraction

The most important scalability principle is:

program requirement
       ↓
resource abstraction
       ↓
capability resolution
       ↓
target realization

The source may state:

requires memory >= required_memory;
requires capability("tensor.compute");
requires capability("quantum.measurement");
requires topology(required_topology);

The source must not state universal machine limits.

The compiler may discover that:

target A

satisfies the requirements while:

target B

does not.

The program remains unchanged.

---

42. Requirement vs Preference

Compilation intent must preserve semantic strength.

Requirement

Must be satisfied.

requires capability("quantum.measurement");

Constraint

Must satisfy a predicate.

requires memory >= required_memory;

Preference

Preferred but replaceable.

prefer capability("accelerated.compute");

Hint

Advisory only.

hint parallel;

Prohibition

A realization must not violate it.

forbid capability("native.execution");

These categories must never be collapsed into one ambiguous mechanism.

---

43. POCO-REAF

POCO-REAF is an architectural property, not a requirement to add a "poco_reaf" keyword.

The source program should express stable computational meaning.

Compilation determines a compatible realization.

The same source can therefore be considered for:

tiny embedded target
CPU
multicore CPU
GPU
FPGA
ASIC
accelerator
QPU
quantum simulator
HPC
cluster
distributed infrastructure
edge infrastructure
cloud infrastructure
future computational systems

provided semantic requirements can be satisfied.

POCO-REAF does not mean:

«Every program must execute on every machine.»

It means:

«Hardware scale and target realization should not require rewriting a semantically portable source program merely because the available implementation changes.»

If a target cannot satisfy a mandatory requirement, the compiler must report an explicit incompatibility or follow an explicitly permitted fallback.

It must never silently change program meaning.

---

44. Scalability Contract

The compilation grammar MUST NOT establish artificial ceilings for:

operations
values
functions
types
modules
dependencies
targets
devices
resources
capabilities
threads
tasks
processes
nodes
CPUs
cores
GPUs
FPGAs
ASICs
accelerators
QPUs
qubits
memory
storage
registers
register width
vector width
tensor rank
tensor dimensions
network size
topology size
artifacts
compiler stages
optimization passes
dependencies

The list is illustrative, not exhaustive.

Scalability is bounded only by:

- program semantics;
- representation requirements;
- explicit program constraints;
- compiler implementation limits;
- actual memory/resources available;
- operating environment;
- target capabilities;
- target resource availability.

---

45. No Artificial Grammar Cardinalities

The grammar must not encode:

target0
target1
...
target15

as the universal target model.

Nor:

qubit0
qubit1
...
qubit127

as the universal quantum model.

Nor:

cpu0
cpu1
...

as the universal processor model.

Collections must be represented through the language's normal collection, identifier, expression, type, and semantic mechanisms.

---

46. Program Constants Are Different

The prohibition concerns compiler-imposed universal ceilings, not ordinary program data.

For example:

let n = 1024;

is valid program data.

Likewise a program may deliberately define a bounded algorithm.

The problem is a compiler rule such as:

Zamani supports at most 1024 qubits.

That type of limit is prohibited from becoming a universal grammar rule.

---

47. Target Independence

The compilation grammar MUST NOT leak physical implementation details backward into source semantics.

The following distinction is mandatory:

logical requirement
        ≠
physical resource

and:

target capability
        ≠
device identity

and:

target intent
        ≠
target allocation

and:

optimization preference
        ≠
optimization implementation

and:

lowering intent
        ≠
lowering algorithm

and:

artifact intent
        ≠
artifact storage

---

48. Determinism

Where a source program requests deterministic compilation, the compiler must ensure deterministic behavior for all semantically relevant inputs.

This includes, where applicable:

- ordering;
- dependency resolution;
- specialization;
- optimization decisions;
- generated identifiers;
- artifact identity;
- serialized semantic representations;
- provenance ordering.

Unspecified ordering MUST NOT be allowed to accidentally alter deterministic artifacts.

Determinism must be verified by tests rather than merely documented.

---

49. Reproducibility

Reproducibility is broader than deterministic parsing.

A reproducible build requires control over all relevant semantic inputs.

Potential inputs include:

source
dependencies
language version
grammar version
compiler version
toolchain
profiles
policies
dialects
semantic configuration
target requirements
capability assumptions
resource assumptions
generated inputs

A compiler must distinguish:

reproducible

from:

deterministic

and from:

portable

These concepts overlap but are not identical.

---

50. Provenance

Compilation decisions should be traceable when provenance is requested or required.

A provenance chain may resemble:

source
  ↓
parsed representation
  ↓
semantic fact
  ↓
optimization decision
  ↓
specialization decision
  ↓
lowering decision
  ↓
target decision
  ↓
artifact

Each transformation may record:

source
reason
evidence
decision
transformation
tool
version
policy
input identity
output identity

This supports:

- debugging;
- reproducibility;
- auditing;
- scientific computation;
- compiler explanations;
- security;
- hardware realization analysis.

---

51. Caching

Compilation caching must be semantic.

A cache MUST NOT treat an artifact as valid merely because:

- the host machine is identical;
- the CPU name is identical;
- the device name is identical;
- a physical identifier is identical;
- a filesystem path is identical.

Validity must derive from relevant semantic identities and compilation inputs.

Cache reuse must never bypass:

semantic validation
compatibility validation
policy validation
artifact validation
provenance validation

where those validations are required by the active contract.

---

52. Compilation Artifacts

An artifact has an identity independent of where it happens to be stored.

The compilation system must distinguish:

artifact identity

from:

storage location

and:

artifact representation

from:

backend implementation

This is necessary for portable build caches, distributed builds, reproducibility, and POCO-REAF.

---

53. Cross-Compilation

Cross-compilation must preserve source semantics.

The source describes:

source program
+
destination requirements

The compiler determines:

compatible destination realization

Cross compilation must integrate with:

types
effects
resources
capabilities
ABI
interoperability
target selection
lowering
code generation
artifacts
provenance
reproducibility

---

54. Backend Independence

The compilation grammar must not encode vendor-specific backend implementations into the core language.

Vendor-specific functionality should enter through:

- capabilities;
- dialects;
- interoperability;
- backend registrations;
- semantic metadata;
- external toolchain contracts.

The grammar should not need to be rewritten for every future processor, accelerator, QPU, FPGA family, compiler backend, or execution system.

---

55. Rust Requirements

The repository's Rust implementation baseline is:

Rust 1.97 or later
Rust edition 2021

Production compiler implementation MUST use safe Rust.

"unsafe" MUST NOT be introduced into the compiler/frontend merely to implement compilation grammar functionality.

This includes:

- parsing;
- AST construction;
- compilation-intent analysis;
- compilation planning;
- validation;
- provenance;
- deterministic-build logic;
- reproducibility metadata;
- target/capability requests;
- artifact metadata.

Where performance matters, the implementation should use safe abstractions, ownership, borrowing, iterators, efficient collections, parallel-safe architecture where appropriate, and measured optimization.

---

56. Grammar Does Not Contain Rust Actions

The ".g4" files should remain declarative grammar specifications.

They MUST NOT embed Rust implementation logic merely to:

- inspect hardware;
- perform optimization;
- allocate memory;
- invoke backends;
- execute commands;
- access files;
- access networks;
- discover devices;
- construct physical target mappings.

The grammar generates parser structure.

Rust owns implementation semantics.

---

57. AST Contract

Compilation grammar constructs must map into a domain-neutral AST.

The AST must describe source meaning without encoding:

- LLVM internals;
- QIR internals;
- vendor machine topology;
- physical qubit mappings;
- QEC layouts;
- routing decisions;
- calibration data;
- backend implementation details.

A compilation-intent AST may contain semantic concepts such as:

CompilationIntent
TargetIntent
TargetRequirement
TargetPreference
CompilationProfile
FeatureSelection
OptimizationIntent
SpecializationIntent
LoweringIntent
ArtifactIntent
CachingIntent
DeterminismIntent
ReproducibilityIntent
ProvenanceIntent
CrossCompilationIntent
DeploymentIntent

The exact Rust AST names must follow the existing "src/ast/" architecture rather than creating a second AST hierarchy inside "grammar/compile/".

---

58. Semantic Contract

After parsing:

AST
 ↓
structural validation
 ↓
semantic compilation model

Semantic analysis must resolve:

- names;
- references;
- profiles;
- features;
- target predicates;
- requirements;
- capabilities;
- resources;
- effects;
- contracts;
- policies;
- provenance;
- portability;
- compatibility.

A syntactically valid compilation declaration may still be semantically invalid.

---

59. Type Integration

Compilation constructs may refer to values, expressions, predicates, types, and resources.

They must therefore integrate with:

grammar/types/
grammar/expressions/
grammar/declarations/
grammar/functions/
grammar/core/

The compilation grammar must not redefine general expressions or types.

---

60. Effect Integration

Compilation-time operations can have effects.

Potential effects include:

compile_time
code_generation
reflection
native
foreign
filesystem
network
randomness
simulation
learning
adaptation

The actual effect vocabulary is owned by:

grammar/effects/

Compilation grammar constructs must consume that shared effect model.

---

61. Policy Integration

Compilation decisions may be constrained by policies.

The common policy model may govern:

optimization
target selection
resource selection
deployment
security
adaptation
simulation
code generation
foreign calls
native operations
reproducibility

The compilation grammar must consume the common policy abstraction rather than inventing a second policy language.

---

62. Contract Integration

Compilation constructs can participate in:

requires
ensures
invariant
assume
guarantee
property
assertion

The compilation subsystem does not own those universal contract semantics.

Those belong to:

grammar/validation/

Compilation-specific contracts are semantic consumers of the common contract system.

---

63. Capability Negotiation

The compilation architecture uses:

program requirement
        ↓
capability requirement
        ↓
candidate target capabilities
        ↓
resource feasibility
        ↓
selection policy
        ↓
realization

This is the core mechanism enabling scale independence.

For example:

requires capability("tensor.compute");

does not mean:

use GPU X

It means:

the realization must provide the required semantic capability

---

64. Fallback Semantics

A fallback must be explicit and semantics-preserving.

Possible compiler outcomes include:

accept
select alternative realization
specialize
lower
simulate
distribute
retry
recover
reject

A fallback MUST NOT silently weaken program semantics.

For example, if a program requires a physical quantum capability and simulation is not semantically permitted, the compiler must reject an incompatible target rather than silently simulate.

---

65. Adaptive Compilation

Compilation may produce adaptive plans where permitted.

Adaptation can use:

capabilities
resources
policies
provenance
contracts
target information
runtime feedback

But compilation intent remains separate from unrestricted self-modifying behavior.

The compiler must know whether a transformation is:

compile-time
deployment-time
runtime

and enforce the corresponding policy.

---

66. Distributed Compilation

Compilation must be scalable to distributed build environments without changing source semantics.

The grammar therefore must remain independent of:

- fixed worker counts;
- fixed node counts;
- fixed machine identities;
- fixed build topology.

Distributed compilation infrastructure may parallelize:

dependency analysis
semantic analysis
optimization
specialization
lowering
code generation
artifact generation
verification

subject to deterministic and reproducibility contracts.

---

67. Incremental Compilation

The compilation architecture should support incremental compilation through semantic dependency identity.

A changed source unit should invalidate only the affected compilation products where semantic dependencies permit.

This integrates with:

caching.g4
artifacts.g4
provenance.g4
reproducibility.g4

Incrementality must never allow stale semantic artifacts to bypass required validation.

---

68. Compilation Graph

The compiler should conceptually maintain a dependency graph:

source units
    ↓
module graph
    ↓
semantic dependency graph
    ↓
compilation dependency graph
    ↓
IR dependency graph
    ↓
artifact dependency graph

The grammar describes source intent.

The compiler constructs and evaluates the graph.

No universal graph-size limit may be encoded in grammar.

---

69. Compilation Plan

The semantic compilation plan should be conceptually:

CompilationPlan
├── source identity
├── language context
├── feature requirements
├── target intent
├── target-selection policy
├── resource requirements
├── capability requirements
├── effects
├── contracts
├── policies
├── optimization intent
├── specialization intent
├── lowering intent
├── cross-compilation intent
├── artifact intent
├── caching intent
├── determinism intent
├── reproducibility intent
├── provenance intent
└── deployment intent

This is a semantic model, not a new source grammar.

---

70. Canonical IR Boundary

Compilation grammar does not own IR.

The compiler must lower semantic compilation intent into the repository's canonical intermediate representation architecture.

The boundary is:

source
 ↓
AST
 ↓
semantic model
 ↓
canonical IR

For quantum:

semantic quantum model
        ↓
quantum::ir

There must not be:

compile IR
quantum compile IR
AI compile IR
hardware compile IR

unless those are explicitly established as domain representations downstream of the canonical semantic boundary.

---

71. Optimization Pipeline

The compilation subsystem feeds optimization but does not implement it.

Conceptually:

canonical semantic representation
        ↓
optimization intent
        ↓
optimization planner
        ↓
optimization pipeline
        ↓
optimized representation

For quantum:

quantum::ir
   ↓
quantum optimization
   ↓
decomposition
   ↓
routing
   ↓
scheduling

The compilation grammar must never encode a finite list of all optimization algorithms.

---

72. Lowering Pipeline

Lowering is:

portable semantic representation
        ↓
domain representation
        ↓
target-compatible representation
        ↓
backend representation

The number and identity of lowering stages must remain extensible.

The grammar must not impose:

stage1
stage2
stage3
...
stageN

as a universal ceiling.

---

73. Scheduling and Routing Boundary

Compilation grammar does not own routing or scheduling.

For quantum:

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
resilience/QEC
 ↓
ZQN
 ↓
HAL

For other domains, equivalent target realization pipelines may exist.

The compilation grammar only expresses intent that downstream systems consume.

---

74. Hardware Boundary

Hardware descriptions belong under:

grammar/hardware/

Hardware capabilities are discovered or declared downstream.

Compilation intent may ask:

requires capability(...)
requires resource(...)
requires topology(...)

but must not redefine hardware descriptions.

This creates the correct two-way relationship:

program
  │
  │ requires
  ▼
capability/resource model
  ▲
  │ provides
  │
hardware/target

---

75. Interoperability Boundary

Compilation may interact with:

grammar/interoperability/
grammar/dialects/

including external representations and ABI contracts.

Compilation intent may request an artifact suitable for an external environment.

It must not make an external format part of the universal core unless that format is intentionally standardized as Zamani syntax.

---

76. Dialects

Future and domain-specific compilation capabilities should preferentially enter through extensible dialect mechanisms rather than continual modification of the universal compilation grammar.

Examples may include:

vendor-specific compilation policy
domain-specific accelerator
specialized scientific compilation
external data representation
specialized hardware flow

A dialect must declare:

identity
version
syntax
semantic contract
capabilities
effects
resources
compatibility
lowering boundary
artifact boundary

The core grammar remains stable.

---

77. Versioning

Compilation grammar compatibility must distinguish:

language version
grammar version
AST version
semantic model version
IR version
dialect version
compiler version
backend version
artifact schema version

A compiler must not confuse these.

A compatible source language does not necessarily imply compatible generated artifacts.

---

78. Diagnostics

Every compilation feature must produce structured diagnostics.

Diagnostics should carry, where applicable:

severity
code
message
source span
feature
expected condition
actual condition
requirement
unsatisfied capability
unsatisfied resource
policy violation
contract violation
compatibility information
suggested remediation
provenance

Diagnostics must be stable enough for tooling and tests.

---

79. Error Categories

Compilation errors should distinguish at least:

syntax error
invalid compilation construct
unknown feature
unknown profile
invalid target intent
unsatisfied requirement
unsatisfied capability
resource infeasibility
policy violation
contract violation
effect violation
unsupported specialization
unsupported lowering
cross-compilation incompatibility
artifact conflict
reproducibility failure
determinism failure
provenance failure
backend incompatibility

A parser error must not be used when the actual problem is semantic.

---

80. Testing Contract

Every compilation grammar feature requires:

positive tests
negative tests
boundary tests
scalability tests
integration tests
compatibility tests
determinism tests
reproducibility tests

Where applicable:

cross-target tests
cross-domain tests
quantum tests
HDL tests
distributed tests
interoperability tests

---

81. Positive Tests

Positive tests prove valid constructs parse and reach the expected semantic boundary.

Each feature must have:

minimal valid form
normal valid form
composed valid form
parameterized valid form
large symbolic form

---

82. Negative Tests

Negative tests must prove rejection of:

- malformed syntax;
- duplicate ownership;
- invalid references;
- incompatible requirements;
- invalid capability expressions;
- policy violations;
- contract violations;
- impossible compilation requests;
- incompatible target requirements.

---

83. Boundary Tests

Boundary tests must combine compile features with other grammar domains.

Examples:

compile + types
compile + effects
compile + resources
compile + capabilities
compile + contracts
compile + policies
compile + provenance
compile + classical
compile + quantum
compile + hybrid
compile + HDL
compile + distributed
compile + interoperability

---

84. Scalability Tests

Scalability tests must not merely test one large fixed number.

They should verify that structures remain general as their size grows.

Test families should include:

small
larger
very large
symbolically sized
resource-derived
dynamically resolved

The test suite itself must not turn a test fixture's chosen number into a language limit.

---

85. Determinism Tests

Compile the same semantic inputs repeatedly and verify:

same semantic result
same deterministic artifact identity
same deterministic ordering
same deterministic provenance representation

where determinism is requested.

---

86. Reproducibility Tests

Rebuild equivalent inputs under controlled equivalent contexts and verify the selected reproducibility contract.

Tests must identify differences in:

source
dependencies
toolchain
profiles
policies
dialects
target assumptions
compiler version
artifact schema

when those differences are semantically relevant.

---

87. POCO-REAF Integration Test

The most important integration test should represent one source program combining:

classical computation
quantum computation
hybrid control
tensor computation
AI/learning semantics
reasoning
knowledge
adaptation
parallelism
resource requirements
capability requirements
effects
contracts
policies
provenance
simulation
artifact generation
target selection

The expected architecture is:

source
 ↓
lexer
 ↓
parser
 ↓
AST
 ↓
structural validation
 ↓
semantic model
 ↓
resource analysis
 ↓
capability analysis
 ↓
effect analysis
 ↓
contract analysis
 ↓
policy analysis
 ↓
provenance
 ↓
canonical IR
 ├── classical
 └── quantum::ir
 ↓
optimization
 ↓
specialization
 ↓
lowering
 ↓
routing/scheduling where applicable
 ↓
resilience/QEC/ZQN where applicable
 ↓
HAL
 ↓
target realization

The source program must not need to be rewritten merely because realization changes.

---

88. Repository Integration Matrix

Compilation file| Primary owner| Consumes| Produces| Downstream
"compile.g4"| composition| all compile grammars| compile AST boundary| parser/AST
"compilation.g4"| compilation-plan composition| intent/features| compilation specification| semantic compiler
"intent.g4"| intent adapter| feature grammars| intent boundary| semantic compiler
"profiles.g4"| profiles| core/expressions| profile AST| planning
"features.g4"| feature declarations| identifiers| feature intent| capability analysis
"feature-selection.g4"| feature selection| features/capabilities| selection intent| semantic planner
"compile-time.g4"| compile-time constructs| expressions/types| compile-time AST| evaluator/generator
"conditional-compilation.g4"| conditions| expressions/features| conditional intent| semantic selection
"target.g4"| target intent| resources/capabilities| target intent| target resolver
"target-selection.g4"| selection policy| target/resource/capability model| selection policy| resolver
"specialization.g4"| specialization intent| types/features/targets| specialization intent| specialization planner
"optimization.g4"| optimization intent| profiles/policies| optimization intent| optimizer
"lowering.g4"| lowering intent| semantic/target context| lowering intent| lowering pipeline
"code-generation.g4"| code-generation intent| artifacts/ABI| generation intent| backend
"cross-compilation.g4"| cross compilation| source/target/ABI| cross-build intent| compiler/toolchain
"artifacts.g4"| artifact intent| provenance/reproducibility| artifact intent| artifact subsystem
"caching.g4"| cache intent| artifact/provenance| cache policy| cache subsystem
"deterministic-builds.g4"| determinism intent| reproducibility/provenance| deterministic contract| build system
"reproducibility.g4"| reproducibility| provenance/artifacts| reproducibility contract| build system
"provenance.g4"| compile provenance| all relevant semantic metadata| provenance intent| audit/provenance
"deployment.g4"| deployment intent| artifacts/targets/policies| deployment intent| deployment/runtime

This matrix is the baseline ownership contract.

---

89. Dependency Direction

The intended dependency direction is:

core
 ↓
types / expressions / declarations
 ↓
resources / capabilities / effects / validation / policies
 ↓
compile intent
 ↓
semantic model
 ↓
canonical IR
 ↓
domain IR
 ↓
optimization
 ↓
lowering
 ↓
target realization
 ↓
runtime/backend

The following direction is prohibited:

grammar
 ↓
physical backend
 ↓
hardware-specific grammar mutation

Likewise:

grammar
 ↓
IR implementation
 ↓
grammar modification

must not become a circular architecture.

---

90. File Independence Rule

Each compilation grammar file must be independently completable.

When a file is declared complete, its contract must already specify:

its owner
its public rules
its dependencies
its AST representation
its semantic representation
its resource/capability interaction
its policy interaction
its provenance interaction
its IR destination
its downstream consumers
its tests
its compatibility behavior

Later implementation of another subsystem must not require reopening the file merely to discover its intended integration boundary.

Changes should instead occur in the owning downstream subsystem.

---

91. No Duplicate Universal Concepts

The following concepts must have one repository-wide semantic authority:

identifier
type
expression
effect
resource
capability
requirement
constraint
preference
policy
contract
provenance
artifact
target

"grammar/compile/" may consume them.

It must not silently redefine them.

---

92. Extensibility Rule

Future computational technologies must be representable without redesigning the universal grammar.

The architecture must support future:

processors
accelerators
memory technologies
quantum technologies
network technologies
AI systems
HDL technologies
simulation systems
distributed systems

through:

semantic capabilities
resource descriptions
dialects
profiles
policies
interoperability
backend registrations

rather than universal grammar enumeration.

---

93. No Vendor Lock-In

The compilation grammar must not contain universal syntax whose meaning is inseparable from a single:

CPU vendor
GPU vendor
FPGA vendor
ASIC vendor
QPU vendor
cloud provider
compiler backend
runtime

Vendor-specific behavior must be represented through extensible semantic boundaries.

---

94. Compile Once vs Run Everywhere

POCO-REAF consists of separate properties:

Program once

Source semantics are stable.

Compile once

A reusable compilation representation/artifact may be produced when its contract permits.

Run everywhere

The artifact or source can be realized wherever compatibility requirements are satisfied.

Anywhere

Realization is not tied to one provider or physical environment.

Forever

Compatibility, versioning, provenance, and migration mechanisms preserve semantic meaning across evolving implementations.

This does not mean that a binary generated for one architecture must execute unchanged on every unrelated architecture.

It means the program's portable semantics remain authoritative, and realization may be regenerated or adapted without rewriting those semantics.

---

95. Recompilation Is Not Semantic Rewriting

If a target changes:

source

should remain stable.

The compiler may change:

specialization
lowering
optimization
routing
scheduling
backend
artifact

as necessary.

That is not a violation of POCO-REAF.

The violation would be requiring the developer to rewrite the program merely because the hardware scale or realization changed.

---

96. Resource Failure

If a target cannot satisfy:

requirement

the compiler must distinguish:

temporarily unavailable

from:

semantically incompatible

from:

insufficient resource

from:

unsupported capability

from:

unsupported transformation

The result must be explicit.

Silent semantic degradation is prohibited.

---

97. Simulation

Simulation is a realization strategy, not a separate language.

Compilation may request or permit simulation where appropriate.

The same semantic program may therefore be realized as:

classical simulation
quantum simulation
hardware simulation
distributed simulation
fault simulation
performance simulation

The simulator must consume the canonical semantic/IR representation.

---

98. Verification

Compilation intent may request verification artifacts or properties.

Verification must integrate with:

grammar/validation/
grammar/specification/
grammar/spec/
provenance
contracts

Verification results should be capable of being associated with:

source
semantic object
IR object
artifact
decision
provenance

---

99. Security

Compilation grammar must integrate with:

grammar/security/

Security-sensitive compilation intent may constrain:

native execution
foreign calls
filesystem access
network access
reflection
code generation
deployment
artifact trust

The compilation grammar must consume the security policy model instead of inventing a second security system.

---

100. Build-System Integration

The compilation subsystem must integrate with the repository's build/toolchain systems.

The conceptual interface is:

source
 ↓
compile intent
 ↓
semantic compilation plan
 ↓
build planner
 ↓
compiler
 ↓
artifact graph
 ↓
cache
 ↓
verification
 ↓
deployment

The grammar itself must remain independent of the build machine.

---

101. Rust Compiler Integration

The Rust implementation should expose a structured compilation model rather than passing raw parser strings throughout the compiler.

Conceptually:

ANTLR/Zamani parser
        ↓
frontend AST
        ↓
validated semantic model
        ↓
CompilationPlan

The implementation should use typed Rust structures and enums.

Avoid:

Stringly-typed compilation state

where semantic concepts are represented only as arbitrary strings.

Identifiers may be extensible strings, but their semantic containers must remain typed.

---

102. No "unsafe"

The compilation implementation MUST satisfy:

cargo check
cargo test
cargo clippy

under the repository's supported Rust version without introducing "unsafe".

Production compilation components should be designed so safe Rust is sufficient for:

- parsing;
- validation;
- planning;
- resource analysis;
- capability analysis;
- artifact identity;
- provenance;
- deterministic ordering;
- compilation graph management.

---

103. Memory and Large-Program Scalability

Scalability must not depend on fixed-size compiler arrays.

Prefer structures whose capacity grows according to actual requirements.

Examples include:

Vec<T>
VecDeque<T>
HashMap<K, V>
BTreeMap<K, V>
HashSet<T>
BTreeSet<T>
iterators
streaming interfaces
incremental representations

where appropriate.

No compiler structure should contain an artificial universal capacity solely for convenience.

---

104. Large Compilation Graphs

The compiler should support:

incremental compilation
lazy resolution
parallel analysis
memoization
semantic caching
streaming where practical
partitioned compilation
artifact reuse

without changing source semantics.

The grammar itself remains independent of the implementation strategy.

---

105. Deterministic Collection Handling

Whenever semantic output depends on iteration order, the compiler must make ordering explicit.

Do not allow hash-map iteration order to accidentally determine:

- artifact identity;
- generated names;
- optimization ordering;
- provenance ordering;
- diagnostics ordering.

Use deterministic ordering where the active contract requires it.

---

106. Conformance Metadata

Every compile feature should be traceable through a conformance status.

Recommended states:

SPECIFIED
LEXICALLY_SUPPORTED
PARSED
AST_SUPPORTED
SEMANTICALLY_SUPPORTED
TYPE_SUPPORTED
EFFECT_SUPPORTED
RESOURCE_SUPPORTED
CAPABILITY_SUPPORTED
POLICY_SUPPORTED
PROVENANCE_SUPPORTED
IR_SUPPORTED
COMPILER_SUPPORTED
BACKEND_SUPPORTED
TESTED
STABLE
EXPERIMENTAL
DEPRECATED

A feature must not be marked "STABLE" merely because the grammar parses it.

---

107. Documentation Integration

Each compilation feature must have:

grammar contract
specification reference
AST mapping
semantic mapping
compiler mapping
test mapping
compatibility status

The authoritative human specification belongs under:

grammar/specification/

The feature contract belongs under:

grammar/spec/

"grammar/compile/README.md" orchestrates them.

---

108. Generated Documentation

Where practical, repository tooling should derive compilation conformance information from machine-readable metadata.

The generated status documentation must distinguish:

syntax exists

from:

compiler exists

from:

backend exists

from:

production-tested

This prevents documentation from overstating implementation maturity.

---

109. Completion Criteria for "grammar/compile/"

The directory is production-ready only when:

1. "compile.g4" is the sole composition root.
2. Every compile grammar has one documented owner.
3. No two files define competing semantic concepts.
4. Every public rule is reachable from the canonical root.
5. Every public rule maps to the domain-neutral AST.
6. Every AST node has a semantic owner.
7. Compilation semantics consume shared types/effects/resources/capabilities/contracts/policies.
8. Compilation intent does not directly select physical hardware.
9. No artificial universal resource limits exist.
10. Quantum semantics converge on "quantum::ir".
11. Optimization is delegated to compiler infrastructure.
12. Lowering is delegated to compiler infrastructure.
13. Routing and scheduling remain downstream.
14. QEC and ZQN remain downstream.
15. HAL and backend realization remain downstream.
16. Caching is semantic and provenance-aware.
17. Deterministic builds are actually tested.
18. Reproducibility is actually tested.
19. Cross-compilation is tested.
20. Artifact identity is independent of storage location.
21. Diagnostics are structured.
22. Compatibility behavior is defined.
23. Every file has positive, negative, boundary, and scalability tests.
24. Integration tests cover classical, quantum, hybrid, HDL, distributed, AI, and interoperability paths where applicable.
25. Rust implementation is compatible with Rust 1.97 or later.
26. Production Rust contains no "unsafe".
27. "cargo check" succeeds.
28. "cargo test" succeeds.
29. "cargo clippy" succeeds under the supported toolchain.
30. No circular grammar/semantic/IR dependency exists.
31. No vendor-specific implementation has become universal language syntax.
32. Future targets can be added without modifying the fundamental compilation model.

---

110. Required Per-File Completion Checklist

A file under "grammar/compile/" is DONE only when all applicable items are true:

[ ] Purpose defined
[ ] Ownership defined
[ ] Non-ownership defined
[ ] Public rules defined
[ ] Private rules defined
[ ] Lexer dependencies defined
[ ] Grammar dependencies defined
[ ] AST contract defined
[ ] Semantic contract defined
[ ] Type interaction defined
[ ] Effect interaction defined
[ ] Resource interaction defined
[ ] Capability interaction defined
[ ] Contract interaction defined
[ ] Policy interaction defined
[ ] Provenance interaction defined
[ ] Portability interaction defined
[ ] IR destination defined
[ ] Quantum boundary defined where applicable
[ ] HDL boundary defined where applicable
[ ] Backend boundary defined
[ ] Diagnostics defined
[ ] Positive tests exist
[ ] Negative tests exist
[ ] Boundary tests exist
[ ] Scalability tests exist
[ ] Determinism tests exist where applicable
[ ] Reproducibility tests exist where applicable
[ ] Compatibility behavior defined
[ ] Specification linked
[ ] AST owner identified
[ ] Semantic owner identified
[ ] Test owner identified
[ ] No duplicate ownership
[ ] No artificial capacity limits
[ ] No unsafe implementation dependency
[ ] Integration path verified
[ ] Completion criteria satisfied

---

111. Recommended Implementation Order

The compile subsystem should be implemented according to dependency order rather than filename order.

Stage 1 — Authority

README.md
compile.g4
compilation.g4

Establish the single composition architecture.

Stage 2 — Universal intent

intent.g4
profiles.g4
features.g4
feature-selection.g4

Stage 3 — Compile-time semantics

compile-time.g4
conditional-compilation.g4

Stage 4 — Target/resource integration

target.g4
target-selection.g4

Stage 5 — Transformation intent

specialization.g4
optimization.g4
lowering.g4
code-generation.g4

Stage 6 — Cross-environment compilation

cross-compilation.g4

Stage 7 — Artifact lifecycle

artifacts.g4
caching.g4

Stage 8 — Build integrity

deterministic-builds.g4
reproducibility.g4
provenance.g4

Stage 9 — Deployment boundary

deployment.g4

Stage 10 — Repository integration

Integrate with:

lexer
parser
AST
semantic analysis
resources
capabilities
effects
validation
policies
classical
quantum
hybrid
HDL
hardware
execution
distributed
networking
interoperability
dialects
toolchain
runtime

Stage 11 — Conformance

Run the complete compile test matrix.

---

112. What Must Never Be Added to "grammar/compile/"

Do not add grammar constructs merely to represent:

specific CPU model
specific GPU model
specific FPGA model
specific ASIC model
specific QPU model
physical qubit number
physical memory address
fixed device count
fixed CPU count
fixed GPU count
fixed node count
fixed thread count
fixed tensor rank
fixed register width
fixed network size
vendor-specific backend implementation
routing algorithm
scheduling algorithm
QEC implementation
HAL implementation
machine-code instruction set implementation

Such details belong downstream.

---

113. What Belongs in Compilation Intent

The compilation grammar SHOULD express:

requirements
capabilities
constraints
preferences
hints
policies
profiles
optimization objectives
specialization conditions
lowering constraints
artifact requirements
reproducibility requirements
determinism requirements
provenance requirements
cross-compilation intent
deployment intent

These are stable semantic concepts.

---

114. What Belongs in Libraries or Dialects

Application-specific functionality should normally remain outside universal compilation syntax.

Examples include:

computer vision
sentiment analysis
robotics frameworks
blockchain applications
payment systems
administrative workflows
legal workflows
VR/AR applications
specialized scientific models
specific AI model architectures
specific vendor frameworks

The universal language should provide primitives through which those systems can be implemented.

---

115. The Universal Compilation Model

The conceptual semantic center of compilation is:

PROGRAM
   │
   ▼
INTENT
   │
   ├── REQUIREMENT
   ├── CAPABILITY
   ├── RESOURCE
   ├── CONSTRAINT
   ├── PREFERENCE
   ├── HINT
   ├── EFFECT
   ├── CONTRACT
   ├── POLICY
   ├── EVIDENCE
   └── PROVENANCE
   │
   ▼
COMPILATION PLAN
   │
   ├── FEATURE SELECTION
   ├── TARGET INTENT
   ├── SPECIALIZATION
   ├── OPTIMIZATION
   ├── LOWERING
   ├── CODE GENERATION
   ├── ARTIFACTS
   ├── CACHE
   ├── DETERMINISM
   ├── REPRODUCIBILITY
   └── DEPLOYMENT
   │
   ▼
SEMANTIC REPRESENTATION
   │
   ├── CLASSICAL
   ├── QUANTUM
   ├── HDL/HARDWARE
   ├── AI
   ├── DATA
   ├── DISTRIBUTED
   └── HYBRID
   │
   ▼
CANONICAL IR
   │
   ├── classical representation
   └── quantum::ir
   │
   ▼
OPTIMIZATION
   │
   ▼
SPECIALIZATION
   │
   ▼
LOWERING
   │
   ▼
TARGET REALIZATION

---

116. The Meaning of "Everywhere"

"Everywhere" must be understood semantically.

It does not mean that every target has identical physical characteristics.

It means that the program is expressed above physical implementation details whenever its semantics permit that abstraction.

For example:

requires capability("parallel.compute");

can be satisfied by different implementations.

The compiler may choose:

multicore CPU
GPU
accelerator
distributed workers
future parallel architecture

without changing the source meaning.

---

117. The Meaning of "Forever"

Long-term portability requires more than grammar stability.

The repository must preserve:

language compatibility
AST compatibility
semantic compatibility
IR compatibility
dialect compatibility
artifact schemas
provenance
migration rules
deprecation rules

Therefore POCO-REAF depends on the whole architecture, not "grammar/compile/" alone.

---

118. Final Architecture Contract

The complete Zamani model is:

                         ZAMANI SOURCE
                               │
                               ▼
                            LEXER
                               │
                               ▼
                            PARSER
                               │
                               ▼
                         DOMAIN-NEUTRAL AST
                               │
                               ▼
                     STRUCTURAL VALIDATION
                               │
          ┌────────────────────┼─────────────────────┐
          ▼                    ▼                     ▼
        TYPES                EFFECTS             CONTRACTS
          │                    │                     │
          ├──────────────┬─────┴─────────────┬──────┤
          ▼              ▼                   ▼      ▼
      RESOURCES      CAPABILITIES         POLICIES PROVENANCE
          │              │                   │      │
          └──────────────┴─────────┬─────────┴──────┘
                                   ▼
                       SEMANTIC COMPILATION MODEL
                                   │
                                   ▼
                         COMPILATION PLAN
                                   │
       ┌──────────────┬────────────┼────────────┬──────────────┐
       ▼              ▼            ▼            ▼              ▼
    TARGET         FEATURES     OPTIMIZE    SPECIALIZE      LOWER
       │              │            │            │              │
       └──────────────┴────────────┴────────────┴──────────────┘
                                   │
                                   ▼
                            CANONICAL IR
                                   │
                    ┌──────────────┴──────────────┐
                    ▼                             ▼
             CLASSICAL IR                    quantum::ir
                    │                             │
                    └──────────────┬──────────────┘
                                   ▼
                              OPTIMIZATION
                                   │
                              SPECIALIZATION
                                   │
                                LOWERING
                                   │
                       ┌───────────┴───────────┐
                       ▼                       ▼
                  classical                 quantum
                                                │
                                      decomposition/routing
                                                │
                                           scheduling
                                                │
                                      resilience/QEC/ZQN
                                                │
                       ┌────────────────────────┘
                       ▼
                      HAL
                       │
          ┌────────────┼─────────────┐
          ▼            ▼             ▼
        CPU/GPU     FPGA/ASIC       QPU
          │            │             │
          └────────────┼─────────────┘
                       ▼
                ACCELERATORS / HPC
                       │
                DISTRIBUTED / CLOUD
                       │
                FUTURE REALIZATIONS

The source remains the semantic authority.

The compiler determines realization.

---

119. Final Invariants

The following are non-negotiable invariants of "grammar/compile/":

Invariant 1 — One language

There is one Zamani language.

Invariant 2 — One root

"grammar/compile/compile.g4" is the single compilation composition root.

Invariant 3 — One owner per concept

No competing grammar authorities.

Invariant 4 — Domain-neutral AST

Compilation grammar never becomes a physical-machine AST.

Invariant 5 — Semantic separation

Intent is separated from realization.

Invariant 6 — No artificial scale ceilings

No universal fixed hardware/resource capacities.

Invariant 7 — Extensibility

Future targets and capabilities do not require redesigning the universal grammar.

Invariant 8 — Canonical quantum boundary

Quantum semantics converge on "quantum::ir".

Invariant 9 — No compiler implementation in grammar

Grammar describes syntax.

Invariant 10 — No backend implementation in grammar

Backends realize semantic intent.

Invariant 11 — Explicit failure

Unsatisfied requirements are reported explicitly.

Invariant 12 — No silent semantic degradation

Fallbacks must be explicitly permitted and semantics-preserving.

Invariant 13 — Deterministic when requested

Deterministic compilation must be implemented and tested.

Invariant 14 — Reproducible when requested

Reproducibility must be implemented and tested.

Invariant 15 — Provenance-aware

Important compilation transformations and decisions can be traced when requested.

Invariant 16 — Safe Rust

Rust 1.97+ / Rust 2021 / no "unsafe".

Invariant 17 — Independent file completion

Every grammar file has a complete predefined integration contract.

Invariant 18 — Repository-wide integration

Compilation intent must integrate with the existing AST, semantic, resource, capability, effect, validation, policy, IR, compiler, quantum, hardware, execution, runtime, and tooling architecture.

---

120. Definition of DONE

"grammar/compile/README.md" is satisfied when this document accurately remains the orchestration contract for the entire "grammar/compile/" directory.

"grammar/compile/" itself is DONE only when every file satisfies its individual feature contract and the complete path is verified:

SPECIFICATION
      ↓
LEXER
      ↓
GRAMMAR
      ↓
PARSER
      ↓
AST
      ↓
STRUCTURAL VALIDATION
      ↓
SEMANTICS
      ↓
TYPE ANALYSIS
      ↓
EFFECT ANALYSIS
      ↓
RESOURCE ANALYSIS
      ↓
CAPABILITY ANALYSIS
      ↓
CONTRACT ANALYSIS
      ↓
POLICY ANALYSIS
      ↓
PORTABILITY ANALYSIS
      ↓
PROVENANCE
      ↓
COMPILATION PLAN
      ↓
CANONICAL IR
      ↓
OPTIMIZATION
      ↓
SPECIALIZATION
      ↓
LOWERING
      ↓
ROUTING / SCHEDULING WHERE APPLICABLE
      ↓
RESILIENCE / QEC / ZQN WHERE APPLICABLE
      ↓
HAL
      ↓
BACKEND
      ↓
ARTIFACT
      ↓
DEPLOYMENT WHERE REQUESTED
      ↓
VERIFICATION
      ↓
TESTS

The final standard is therefore not:

«"The compilation grammar parses."»

The final standard is:

«Zamani compilation intent has one authoritative grammar boundary, maps cleanly into a domain-neutral semantic model, remains independent of physical machine scale, integrates with every required repository subsystem, can be independently completed file-by-file, preserves program meaning across target realization, and provides the foundation required for Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.»
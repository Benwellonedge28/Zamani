Zamani Grammar Conformance Contract

Path: "grammar/CONFORMANCE.md"
Repository: "Benwellonedge28/Zamani"
Language: Zamani
Scope: Complete "grammar/" production conformance, integration, portability, scalability, compatibility, and freeze gates
Status: Normative
Authority: Production conformance contract
Rust edition: 2021
Minimum Rust version: 1.97
Rust safety: Safe Rust only; "unsafe" is forbidden
Architecture: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)

---

1. Purpose

This document defines the normative production-conformance contract for the entire "grammar/" subsystem.

It establishes the conditions under which:

- an individual grammar file is complete;
- a grammar directory is complete;
- a language feature is implemented;
- a language feature is stable;
- a dependency is valid;
- a parser boundary is valid;
- a lexer boundary is valid;
- a grammar can be frozen;
- a freeze stage can advance;
- the complete "grammar/" directory can be declared production-ready.

This document integrates:

- "grammar/DESIGN.md";
- "grammar/MANIFEST.yaml";
- "grammar/OWNERSHIP.yaml";
- "grammar/DEPENDENCIES.yaml";
- "grammar/STATUS.yaml";
- "grammar/FREEZE.md";
- "grammar/specification/";
- "grammar/spec/";
- "grammar/Zamani.g4";
- "grammar/antlr/";
- "grammar/lexer/";
- all grammar domains;
- the Rust frontend;
- the domain-neutral AST;
- semantic validation;
- canonical IR;
- "quantum::ir";
- compiler/runtime integration;
- conformance tests;
- compatibility;
- scalability;
- POCO-REAF.

This document is a gate, not merely a checklist.

A feature MUST NOT be declared production-ready merely because its ".g4" file parses successfully.

---

2. Authority and precedence

The following authority hierarchy is mandatory.

grammar/DESIGN.md
        |
        v
grammar/specification/
        |
        v
grammar/spec/
        |
        +-----------------------------+
        |                             |
        v                             v
grammar/OWNERSHIP.yaml       grammar/DEPENDENCIES.yaml
        |                             |
        +--------------+--------------+
                       |
                       v
                grammar/STATUS.yaml
                       |
                       v
                grammar/FREEZE.md
                       |
                       v
              grammar/CONFORMANCE.md
                       |
                       v
             grammar/antlr/
                       |
                       v
                grammar/*/
                       |
                       v
                 Rust frontend
                       |
                       v
                 AST / Semantic
                       |
                       v
               Canonical IR

2.1 Authority rules

"DESIGN.md" owns architectural principles.

"specification/" owns normative human-readable language meaning.

"spec/" owns machine-oriented language contracts.

"OWNERSHIP.yaml" owns symbol ownership.

"DEPENDENCIES.yaml" owns dependency relationships.

"STATUS.yaml" owns lifecycle and implementation status.

"FREEZE.md" owns freeze procedure.

"CONFORMANCE.md" owns production conformance gates.

"MANIFEST.yaml" owns repository inventory and manifest metadata.

"Zamani.g4" owns only root grammar composition.

"antlr/ZamaniLexer.g4" owns the public ANTLR lexer boundary.

"antlr/ZamaniParser.g4" owns the public ANTLR parser boundary.

"lexer/" owns lexical contracts and token definitions.

Domain directories own their explicitly assigned syntax.

No document or grammar file may silently override another authority.

Contradictions MUST be resolved explicitly.

---

3. Normative terminology

The following terms are mandatory.

MUST

Absolute requirement.

MUST NOT

Absolute prohibition.

SHOULD

Strong recommendation.

SHOULD NOT

Strong restriction unless justified.

MAY

Permitted behavior.

Production-ready

A feature or subsystem satisfying every applicable conformance gate.

Frozen

A feature whose contract and all required dependencies are stable and whose future downstream additions cannot require semantic modification.

Dependency

A declared consumption of another stable contract.

Owner

The single authoritative owner of a symbol, rule, token, semantic construct, or contract.

Consumer

A component that uses an exported contract without owning it.

Portable meaning

Program semantics independent of a particular physical target.

Realization

Mapping portable semantics to a concrete execution environment.

Capability

An execution environment property that may be required by a program.

Resource

A quantity or property consumed or required by execution.

Constraint

A mandatory restriction on realization.

Preference

A non-mandatory realization preference.

Policy

Rules governing permitted, required, prohibited, or fallback behavior.

Effect

Observable or semantically relevant computational consequence.

Contract

A machine-checkable semantic obligation.

Provenance

Information describing origin, evidence, derivation, transformation, or decision history.

---

4. Global production invariant

The complete grammar is conformant only when:

one language
+
one authority model
+
one lexical authority
+
one parser composition boundary
+
one domain-neutral AST
+
one shared semantic foundation
+
one effect model
+
one capability model
+
one resource model
+
one contract model
+
one policy model
+
one provenance model
+
canonical domain IRs
+
target-independent semantics
+
versioned compatibility
+
complete tests
+
safe Rust
+
open-ended scalability

All components MUST agree with this model.

---

5. Production status is gated

The repository MUST NOT be declared:

production_ready: true

until all mandatory gates pass.

A component MAY be "STABLE" while the entire grammar remains non-production.

The status of the entire grammar is the conjunction of all mandatory production gates.

Formally:

GRAMMAR_PRODUCTION_READY =
    AUTHORITY_PASS
    AND INVENTORY_PASS
    AND OWNERSHIP_PASS
    AND DEPENDENCY_PASS
    AND LEXER_PASS
    AND PARSER_PASS
    AND AST_PASS
    AND SEMANTIC_PASS
    AND IR_PASS
    AND DIAGNOSTIC_PASS
    AND COMPATIBILITY_PASS
    AND PORTABILITY_PASS
    AND SCALABILITY_PASS
    AND HARD_CODING_PASS
    AND SAFETY_PASS
    AND TEST_PASS
    AND FREEZE_PASS

One mandatory failure prevents production readiness.

---

6. File-level completion contract

Every production ".g4", normative ".md", machine contract ".yaml", and applicable conformance artifact MUST have a complete contract.

At minimum it MUST identify:

PURPOSE
STATUS
AUTHORITY
OWNS
DOES_NOT_OWN
INPUTS
OUTPUTS
DEPENDS_ON
IMPORTS
EXPORTS
CONSUMED_BY
AST_OWNER
SEMANTIC_OWNER
IR_OWNER
RUNTIME_OWNER
TOOLING_OWNER
DIAGNOSTIC_OWNER
TEST_OWNER
COMPATIBILITY_CONTRACT
SCALABILITY_CONTRACT
POCO_REAF_CONTRACT
HARD_CODING_AUDIT
AMBIGUITY_CONTRACT
ERROR_HANDLING
CONFORMANCE_REQUIREMENTS
FREEZE_CRITERIA

A file lacking required ownership or integration information is not freeze-eligible.

---

7. Inventory conformance

"grammar/MANIFEST.yaml" MUST provide a machine-verifiable inventory.

Every production file MUST be represented by one of:

ACTIVE
PLANNED
EXPERIMENTAL
DEPRECATED
HISTORICAL
QUARANTINED
REJECTED

No file may exist in an unexplained state.

The manifest MUST detect:

- missing files;
- undocumented files;
- duplicate paths;
- invalid paths;
- malformed filenames;
- files outside their declared ownership domain;
- orphaned grammar files;
- unreferenced composition roots;
- undeclared generated artifacts.

A directory MUST NOT be declared frozen while undocumented files remain.

---

8. Ownership conformance

Every exported symbol MUST have exactly one owner.

The validator MUST reject:

zero owners
multiple owners
implicit owners
directory-only ownership
consumer-as-owner declarations

Example:

quantumOperation
    owner: quantum/operations.g4

Other files may consume "quantumOperation".

They MUST NOT redefine it.

Ownership MUST be recorded in:

grammar/OWNERSHIP.yaml

and MUST agree with the file-level contract.

---

9. Dependency conformance

"grammar/DEPENDENCIES.yaml" is authoritative for dependency relationships.

Every dependency MUST have:

source
target
kind
reason
direction
status
contract
freeze requirement

Supported dependency kinds include:

grammar_import
lexical_contract
syntax_contract
ast_contract
semantic_contract
ir_contract
diagnostic_contract
compatibility_contract
documentation_reference
test_contract
tooling_contract
repository_integration

Dependencies MUST form an acyclic production graph.

A cycle MUST be resolved by extracting the shared contract into a lower-level authority.

Mutual dependency between two grammar domains is not an acceptable final architecture.

---

10. Dependency direction

The production dependency direction is:

authority
    ↓
registry/contracts
    ↓
lexical
    ↓
core/foundation
    ↓
types/memory
    ↓
expressions
    ↓
functions/declarations/statements/modules
    ↓
effects/resources/capabilities/policies/security
    ↓
classical/data
    ↓
concurrency/distributed/networking
    ↓
hardware/execution/compile
    ↓
quantum/HDL
    ↓
hybrid
    ↓
AI and higher domains
    ↓
dialects/interoperability
    ↓
metaprogramming/macros
    ↓
compatibility
    ↓
root composition

This ordering is a dependency model, not permission to bypass ownership.

A lower layer MUST NOT depend on an upper layer merely to obtain functionality that belongs in a shared lower contract.

---

11. Forbidden dependency directions

The following are production failures.

11.1 Core to domain

core → quantum
core → HDL
core → AI
core → vendor

11.2 Grammar to backend

grammar → GPU implementation
grammar → QPU implementation
grammar → FPGA implementation
grammar → ASIC implementation
grammar → runtime scheduler
grammar → physical routing implementation

11.3 Grammar to physical inventory

grammar → device inventory
grammar → machine count
grammar → physical qubit inventory
grammar → physical memory inventory

11.4 Production grammar to tests

grammar → tests

Tests consume grammar.

Grammar does not depend on tests.

11.5 Production grammar to examples

Examples consume grammar.

Grammar MUST NOT import examples.

11.6 Production grammar to validation implementation

Validation tooling consumes grammar metadata.

Grammar syntax MUST NOT depend on validator implementation.

---

12. Lexer conformance

There MUST be one public lexical boundary:

grammar/antlr/ZamaniLexer.g4

The lexical architecture MUST be:

lexer/tokens.g4
       ↓
lexer/* lexical contracts
       ↓
lexer/lexer.g4
       ↓
antlr/ZamaniLexer.g4
       ↓
Rust lexer/frontend

Every emitted token MUST have:

name
spelling
category
owner
reserved/contextual status
compatibility identity
test coverage

The lexer MUST NOT contain duplicate semantic token identities.

Tokens that appear similar MUST be retained separately only when their lexical or semantic distinction is documented.

---

13. Lexer hard gates

The lexer fails conformance if:

- two files own the same token;
- a token is emitted without registry ownership;
- a registered token can never be emitted without explanation;
- a public token has no compatibility identity;
- token behavior differs between the declared lexer and Rust frontend without documented compatibility handling;
- domain-specific tokens leak into universal lexical space without justification;
- a token embeds physical hardware capacity;
- a token requires a finite machine configuration;
- lexical behavior depends on runtime hardware.

---

14. Parser conformance

There MUST be one public parser boundary:

grammar/antlr/ZamaniParser.g4

It MUST compose modular grammar ownership.

The parser root MUST NOT become a second domain-specific grammar.

"grammar/Zamani.g4" MUST remain a composition root.

It MUST NOT become a storage location for:

- quantum operations;
- hardware limits;
- AI semantics;
- HDL semantics;
- backend instructions;
- target-specific constants;
- IR definitions.

---

15. Root grammar gate

"grammar/Zamani.g4" is conformant only when it:

- has one root composition role;
- has no competing root;
- contains no duplicated leaf semantics;
- defines no token vocabulary;
- defines no physical resource limits;
- defines no vendor-specific operation set;
- delegates domain syntax;
- exposes the universal program structure;
- reaches EOF deterministically.

---

16. Parser correctness

The parser MUST pass:

reachability
termination
ambiguity
precedence
associativity
left-recursion policy
import resolution
token resolution
rule ownership
error recovery
source-span preservation

Any intentional ambiguity MUST be explicitly documented.

Accidental ambiguity is a production failure.

---

17. AST conformance

Every meaning-bearing syntax construct MUST have a defined mapping to the domain-neutral AST.

The mapping MUST be recorded through the repository's AST contract system.

A grammar production is incomplete if:

syntax exists
AND
AST mapping does not exist

unless the production is explicitly declared syntax-only metadata.

AST mappings MUST preserve:

- source location;
- source identity;
- names;
- types;
- effects;
- capabilities;
- resources;
- constraints;
- preferences;
- policies;
- contracts;
- provenance;
- domain information;
- compatibility information where applicable.

---

18. AST neutrality

The AST MUST remain domain-neutral.

A quantum AST node MUST NOT directly encode:

specific QPU
physical qubit number
vendor calibration
device-specific routing

An HDL AST node MUST NOT directly encode a particular physical FPGA.

An AI AST node MUST NOT require a particular accelerator.

The AST represents source meaning.

Target realization occurs later.

---

19. Semantic conformance

Every semantic construct MUST identify its semantic owner.

Semantic validation MUST cover, where applicable:

names
types
values
effects
capabilities
resources
requirements
constraints
preferences
contracts
policies
provenance
uncertainty
authorization
portability
compatibility

Syntax alone is insufficient evidence of semantic implementation.

---

20. Type-system conformance

The type system MUST be target-independent.

Types MUST describe meaning.

Types MUST NOT silently encode:

- physical register width;
- physical memory size;
- device count;
- fixed hardware lane count;
- physical qubit count;
- vendor-specific storage limits.

Parameterized values MUST remain semantic parameters.

For example:

Vector<T, N>

does not mean a particular CPU register width.

Likewise:

Qubit[n]

does not imply physical qubit identifiers.

---

21. Effect conformance

Every effectful operation MUST identify its effect semantics.

Effects MUST integrate with:

types
capabilities
resources
policies
security
contracts
provenance

The effect system MUST distinguish at least the repository-supported categories such as:

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
hardware

An effect MUST NOT bypass policy or capability validation merely because its syntax is accepted.

---

22. Capability conformance

Capabilities MUST be symbolic.

Valid examples include:

capability("quantum.measurement")
capability("gpu.compute")
capability("tensor.compute")

The grammar MUST NOT require a particular provider unless the programmer explicitly expresses such a requirement through a supported target/dialect contract.

Capability names MUST NOT encode hidden finite assumptions.

---

23. Resource conformance

Resources MUST be represented symbolically.

Valid forms include:

requires qubits >= n
requires memory >= required_memory
requires capability("quantum.measurement")
requires capability("gpu.compute")
requires capability("tensor.compute")
requires topology(...)

Also supported semantic categories include:

requires
prefer
constrain
allow
forbid

Resource requirements describe program needs.

They do not select physical hardware.

---

24. Absolute hard-capacity prohibition

The production grammar MUST NOT define universal finite capacity constants such as:

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

Equivalent disguised forms are also prohibited.

The hard-coding validator MUST search for:

- literal capacity constants;
- bounded enumerations pretending to be universal;
- fixed-size device sets;
- fixed physical identifiers;
- fixed vendor inventories;
- finite grammar alternatives representing current hardware;
- numeric bounds presented as language-wide limits.

---

25. What is allowed

The following model is conformant:

requires memory >= required_memory

requires qubits >= n

requires capability("quantum.measurement")

requires capability("gpu.compute")

requires topology(...)

prefer capability("tensor.compute")

constrain latency <= allowed_latency

The values remain program-level requirements.

The realization layer decides whether and how those requirements can be satisfied.

---

26. Scalability conformance

The language MUST be open-ended with respect to resource scale.

The grammar MUST NOT impose an artificial upper bound on:

- computation size;
- number of values;
- number of tasks;
- number of nodes;
- number of devices;
- number of qubits;
- memory;
- tensor dimensions;
- network participants;
- accelerator instances;
- hardware modules.

Actual execution MAY be constrained by:

- available resources;
- target capabilities;
- implementation limits;
- operating-system limits;
- physical limits;
- execution policy.

Those environmental limits MUST NOT become universal language semantics.

---

27. Atom-to-everywhere conformance

The same semantic program MUST be capable of representing computations ranging from:

tiny
embedded
single-device
multicore
accelerated
distributed
HPC
cluster
cloud
quantum
hybrid
future target

without requiring the language core to be redesigned around a finite target inventory.

Scaling MUST be parameterized by program semantics and realization context.

---

28. POCO-REAF conformance

POCO-REAF requires four independent properties.

28.1 Program Once

The programmer expresses semantic intent rather than unnecessary physical realization details.

28.2 Compile Once

The compiler produces or preserves a target-independent portable semantic artifact.

That artifact MUST preserve, where applicable:

language version
feature set
AST version
semantic version
IR version
dialect versions
type information
effect information
resource requirements
capability requirements
constraints
preferences
policies
contracts
provenance
reproducibility information

28.3 Run Everywhere

A realization MUST be selected through:

discover
→ match requirements
→ apply constraints
→ evaluate capabilities
→ apply policies
→ rank preferences
→ select realization
→ lower
→ route
→ schedule
→ execute

28.4 Anywhere

A fallback MUST be explicit.

The implementation MUST NOT silently replace an operation with an incompatible approximation.

28.5 Forever

Long-term compatibility requires:

language versioning
AST versioning
semantic versioning
IR versioning
dialect versioning
migration rules
deprecation rules
reproducibility rules

---

29. Fallback conformance

Fallback behavior MUST be explicit.

A program MAY permit:

allow fallback

but a fallback MUST have defined semantic compatibility.

Possible outcomes include:

accept
degraded_accept
retry
recover
escalate
reject

A fallback MUST NOT silently change program semantics.

Approximation MUST be explicitly permitted where applicable.

---

30. Domain conformance

Every domain MUST integrate with common language foundations.

Domains include:

classical
quantum
hdl
hybrid
ai
data
concurrency
distributed
networking
hardware
execution
compile
nano

A domain MUST NOT create a parallel:

type system
effect system
resource system
capability system
policy system
AST universe
compatibility system

unless the new structure is explicitly a shared extension governed by the common architecture.

---

31. Classical conformance

"classical/" MUST build on:

core
types
expressions
functions
memory
effects
resources
capabilities
policies

It MAY provide:

- numeric computation;
- vectors;
- matrices;
- tensors;
- symbolic computation;
- scientific computation;
- statistics;
- signal processing;
- optimization;
- accelerator-oriented semantics.

It MUST remain target-independent.

---

32. Quantum conformance

Quantum grammar MUST be data-driven.

The canonical model is:

quantumOperation
operationSpecifier
targets
controls
parameters
results
attributes
modifiers

The grammar MUST NOT require permanent enumeration of every known operation.

An operation specifier MAY identify:

builtin
custom
vendor
parameterized
dialect
future

Adding a new quantum operation MUST NOT require redesigning the universal parser architecture.

---

33. Quantum semantic boundary

Quantum syntax MUST flow through:

Zamani source
    ↓
domain-neutral AST
    ↓
quantum semantic model
    ↓
quantum::ir
    ↓
optimization/lowering
    ↓
target realization

Quantum grammar MUST NOT directly encode:

- physical qubit assignment;
- calibration;
- hardware routing;
- vendor device inventory;
- scheduling implementation.

---

34. Quantum resource conformance

Quantum resource declarations MUST express requirements such as:

requires qubits >= n
requires capability("quantum.measurement")
requires capability("quantum.reset")
requires topology(...)

They MUST NOT encode a universal physical QPU size.

---

35. Quantum error-correction conformance

Quantum error-correction syntax MAY express:

- logical qubits;
- code requirements;
- distance;
- syndrome intent;
- decoder requirements;
- fault-tolerance requirements;
- resilience requirements.

The grammar MUST NOT bind those declarations to a particular physical device.

---

36. HDL conformance

HDL syntax MUST describe hardware intent.

It MAY cover:

modules
ports
signals
nets
registers
memories
combinational logic
sequential logic
clocks
reset
timing
pipelines
interfaces
protocols
state machines
assertions
verification
synthesis intent
physical intent

It MUST NOT impose universal fixed widths or finite device inventories.

---

37. Hybrid conformance

"hybrid/" MUST integrate already-defined semantics.

It MUST NOT redefine:

classical types
quantum operations
HDL semantics
hardware semantics
resource semantics
effect semantics

Hybrid features MUST express the boundaries between established domains.

---

38. AI conformance

AI functionality MUST use common:

types
expressions
effects
resources
capabilities
policies
contracts
provenance
data
concurrency
distributed execution

AI syntax MAY express:

infer
deduce
reason
learn
adapt
evidence
confidence
probability
uncertainty
decision
planning
models
training

Application-specific concepts MUST remain outside universal core grammar.

They belong in libraries, dialects, capabilities, policies, or application layers.

---

39. Adaptation conformance

Adaptive execution MUST NOT mean unrestricted self-modifying code.

An adaptive operation MUST be governed by applicable:

policy
capability
effect
resource
authorization
provenance
contract
security rules

The conformance validator MUST reject adaptation paths that bypass these controls.

---

40. Data conformance

"data/" MUST build on common types and expressions.

It MAY define:

- collections;
- schemas;
- tables;
- graphs;
- datasets;
- streams;
- transformations;
- pipelines;
- provenance;
- serialization;
- persistence.

External data formats MUST remain dialect/interoperability concerns.

---

41. Distributed conformance

Distributed syntax MUST express semantic intent.

It MUST NOT require fixed:

node count
device count
message count
network size
cluster size

Placement and realization MUST be determined downstream.

---

42. Networking conformance

Networking MUST use:

capabilities
resources
effects
security
policies
provenance

Networking syntax MUST NOT hard-code a finite universe of devices, addresses, endpoints, or channels.

---

43. Memory conformance

Memory semantics MUST distinguish:

source memory meaning

from:

physical memory capacity

The grammar MAY express:

- ownership;
- borrowing;
- references;
- lifetimes;
- regions;
- shared memory;
- distributed memory;
- persistent memory;
- accelerator memory;
- quantum memory.

It MUST NOT define a universal physical memory maximum.

---

44. Hardware conformance

Hardware grammar MUST describe:

capability
resource
topology
timing
memory
power
thermal
reliability
placement intent
performance constraints

Hardware selection MUST remain late-bound unless the program explicitly requests a supported target-specific contract.

---

45. Execution conformance

"execution/" owns execution intent, not physical implementation.

It MAY express:

context
dispatch
placement
scheduling intent
parallelism
simulation
adaptive execution
checkpointing
recovery
resilience
observation
profiling
tracing
lifecycle
deployment

The actual scheduler and runtime remain downstream.

---

46. Compilation conformance

"compile/" MUST support a target-independent compilation model.

It MUST account for:

language version
feature set
target requirements
capabilities
resources
constraints
preferences
policies
reproducibility
artifact identity
lowering
specialization
compatibility

Compilation MUST NOT force source syntax to identify a physical machine unless explicitly required.

---

47. Policy conformance

Policies MUST govern:

permissions
prohibitions
requirements
constraints
preferences
fallback
adaptation
execution
deployment
security
resources
simulation
provenance

Policies MUST NOT silently redefine type or domain semantics.

---

48. Security conformance

Security syntax MUST integrate with:

identity
authorization
capabilities
sandbox
trust
audit
privacy
secrets
cryptography
signatures
provenance
policy

Grammar acceptance MUST NOT be treated as authorization.

Authorization remains a semantic/runtime concern.

---

49. Dialect conformance

Every dialect MUST have:

identity
namespace
version
dependencies
capabilities
syntax boundary
semantic mapping
compatibility
registration
status

A dialect MUST NOT silently modify universal core semantics.

Adding a dialect MUST NOT require rewriting unrelated core grammar.

---

50. Interoperability conformance

Interoperability MUST distinguish:

external syntax
ABI
API
FFI
calling convention
foreign types
serialization
linkage
external functions

Parsing an external format does not make that format part of Zamani core syntax.

External semantics MUST be mapped explicitly into Zamani semantic contracts.

---

51. Metaprogramming conformance

Reflection, generation, introspection, quotation, and compile-time computation MUST respect:

effects
capabilities
policies
security
sandbox
provenance

Metaprogramming MUST NOT bypass:

- type validation;
- semantic validation;
- policy validation;
- ownership validation;
- compatibility validation.

---

52. Macro conformance

Macros MUST preserve semantic validation.

Macro expansion MUST be:

deterministic where required
hygienic where applicable
source-mapped
diagnosable
version-aware
policy-aware

Macros MUST NOT create hidden syntax that bypasses the canonical parser/semantic pipeline.

---

53. Validation subsystem conformance

"validation/" is the grammar firewall.

It MUST provide checks for:

inventory
ownership
dependencies
imports
exports
duplicates
cycles
reachability
ambiguity
precedence
source spans
AST mappings
semantic mappings
IR mappings
diagnostics
hard-coded capacities
domain leakage
path hygiene
compatibility
scalability
POCO-REAF

Validation implementation MUST consume grammar contracts.

Production grammar MUST NOT depend on validation implementation.

---

54. Duplicate-concept conformance

Conceptually duplicate files MUST be detected.

Examples include patterns such as:

closure / closures
lambda / lambdas
map / map-types
option / option-types
array / array-types

A duplicate-looking file may remain only when its distinction is explicitly documented.

The distinction MUST include:

different owner
different purpose
different exports
different consumers
different semantic meaning

Otherwise one canonical owner MUST be selected and the other path must be removed, deprecated, or converted into an explicit compatibility alias.

---

55. Path hygiene

Production paths MUST:

- use "/";
- be deterministic;
- contain no control characters;
- contain no unintended whitespace;
- contain no ambiguous case-only duplicates;
- contain no accidental duplicate naming;
- not depend on operating-system-specific path semantics.

Suspicious or malformed paths MUST be quarantined until resolved.

No malformed path may participate in a frozen dependency graph.

---

56. Diagnostics conformance

Every production syntax feature MUST define invalid forms.

Diagnostics MUST preserve:

source span
diagnostic identity
severity
category
message
relevant symbol/rule
expected form where available
compatibility context where applicable

Diagnostic ownership MUST be unique.

The same invalid construct MUST NOT produce arbitrary ownership-dependent diagnostics across parser paths.

---

57. Error-layer conformance

Errors MUST be classified at the earliest correct layer.

Required distinction:

LEXICAL_ERROR
SYNTAX_ERROR
STRUCTURAL_ERROR
NAME_ERROR
TYPE_ERROR
EFFECT_ERROR
CAPABILITY_ERROR
RESOURCE_ERROR
CONTRACT_ERROR
POLICY_ERROR
PROVENANCE_ERROR
COMPATIBILITY_ERROR
LOWERING_ERROR
TARGET_ERROR
RUNTIME_ERROR

The implementation MAY add finer categories.

It MUST NOT collapse semantically different failures into misleading categories.

---

58. Compatibility conformance

Stable language behavior MUST be versioned.

Compatibility MUST cover:

language
lexer
parser
AST
semantic model
IR
quantum::ir
dialects
features
diagnostics where stability is promised

Changes MUST be classified as:

compatible
additive
deprecating
breaking
migration-required

A breaking change MUST NOT be silently introduced under a stable version.

---

59. Forward compatibility

Future extensions MUST have reserved extension points where required.

The grammar SHOULD prefer data-driven forms when an open-ended domain is expected to grow.

This particularly applies to:

quantum operations
capabilities
resource kinds
dialects
effects
hardware capabilities
accelerator descriptions
future execution models

The language MUST NOT require a core grammar rewrite merely because a new member of an open-ended semantic category is introduced.

---

60. AST/semantic/IR completeness

For every production feature:

syntax
   ↓
AST
   ↓
semantic model
   ↓
canonical IR

MUST be traceable.

A traceability record MUST identify:

grammar rule
AST representation
semantic representation
IR representation
diagnostics
tests

Where no IR representation is applicable, the feature MUST explicitly state:

IR_OWNER: NOT_APPLICABLE

and explain why.

---

61. Canonical IR conformance

The compiler MUST NOT create competing semantic IRs.

The architecture permits:

Classical IR
quantum::ir
HDL/hardware domain IR

and downstream lowering IRs where justified.

However:

source syntax → vendor-specific IR

is non-conformant.

The required route is:

source
→ AST
→ semantic model
→ canonical IR
→ optimization/lowering
→ target realization

---

62. Quantum IR conformance

Quantum constructs MUST converge through:

quantum::ir

A quantum grammar file MUST NOT define a second canonical quantum IR.

Vendor or backend-specific representations MAY exist after the canonical boundary.

---

63. Target independence

The grammar MUST NOT depend on:

specific CPU model
specific GPU model
specific FPGA model
specific ASIC model
specific QPU model
specific operating system
specific cloud provider
specific device inventory
specific physical topology

unless the dependency is explicitly expressed as a dialect or target-specific contract.

Even then, target-specific behavior MUST NOT contaminate universal grammar semantics.

---

64. Capability negotiation

Target realization MUST conceptually follow:

discover
    ↓
collect capabilities
    ↓
collect resources
    ↓
evaluate requirements
    ↓
apply constraints
    ↓
apply policies
    ↓
evaluate preferences
    ↓
select realization
    ↓
specialize
    ↓
lower
    ↓
route
    ↓
schedule
    ↓
execute

The grammar expresses the program's requirements and intent.

It does not perform physical discovery.

---

65. Topology conformance

Topology expressions MUST describe abstract requirements.

Examples include:

topology(...)

They MUST NOT require source code to enumerate every physical connection unless that physical topology is intentionally part of a target-specific contract.

Routing remains downstream.

---

66. Determinism and reproducibility

Where deterministic execution or compilation is requested, the language/toolchain MUST preserve sufficient metadata to reproduce:

source identity
language version
feature versions
dialect versions
compiler identity
semantic configuration
relevant policies
resource/capability decisions
randomness configuration
provenance
artifact identity

Nondeterminism MUST be represented through effects and policies where semantically relevant.

---

67. Provenance conformance

Evidence, explanation, decisions, transformations, and generated artifacts MUST have traceable provenance where the applicable feature requires it.

Provenance MUST be capable of connecting:

source
→ AST
→ semantic decision
→ optimization
→ lowering
→ realization
→ artifact

The provenance model MUST NOT depend on a specific target.

---

68. Contract conformance

The language contract system MUST support, where implemented:

requires
ensures
invariant
assume
guarantee
property
assert

Contracts MUST have:

scope
condition
source location
semantic owner
diagnostics
evaluation policy
compatibility behavior

Contracts MUST NOT be reduced to comments.

---

69. Policy/effect/capability interaction

A feature requiring privileged behavior MUST be validated across all applicable dimensions.

Conceptually:

requested operation
        |
        +--> effect
        |
        +--> capability
        |
        +--> resource
        |
        +--> policy
        |
        +--> authorization
        |
        +--> provenance

Missing authorization MUST prevent the operation when authorization is required.

---

70. Rust conformance

The implementation MUST satisfy:

Rust edition = 2021
minimum Rust = 1.97
unsafe = forbidden

Production Rust MUST NOT contain:

unsafe { ... }

or equivalent unsafe constructs.

The conformance gate MUST scan the relevant Rust implementation.

The project MUST also validate dependencies so that introducing a new dependency does not silently undermine the safe-Rust requirement.

Where an external boundary requires inherently unsafe machinery, the Zamani language implementation MUST expose a safe validated abstraction rather than exposing unsafe behavior through the language frontend.

---

71. Rust integration conformance

Grammar changes that affect implementation MUST be traceable to the Rust frontend.

At minimum, applicable integration includes:

src/lexer.rs
src/parser.rs
src/ast/

and, when applicable:

src/semantic/
src/ir/
src/quantum/
src/compile/

A grammar feature MUST NOT be marked "IMPLEMENTED" merely because the ANTLR grammar compiles.

---

72. Test ownership

Every production feature MUST have an identified test owner.

Tests MUST be organized by semantic responsibility.

Required categories include:

lexical
syntax
AST
semantic
types
effects
resources
capabilities
contracts
policies
security
classical
quantum
HDL
hybrid
AI
data
concurrency
distributed
networking
hardware
execution
compile
dialects
interoperability
macros
metaprogramming
compatibility
portability
scalability
diagnostics
negative
fuzz
regression

---

73. Positive tests

Every production feature MUST have valid examples.

Positive tests MUST demonstrate:

- canonical syntax;
- composition;
- nesting;
- parameters;
- relevant attributes;
- source locations;
- AST mapping;
- semantic mapping where applicable.

---

74. Negative tests

Every production feature MUST have invalid examples.

Negative tests MUST cover applicable:

missing operands
invalid types
invalid names
invalid effects
missing capabilities
insufficient resources
policy violations
contract violations
invalid modifiers
invalid combinations
ambiguous syntax
unsupported target requirements
compatibility violations

---

75. Boundary tests

Boundary tests MUST cover:

empty forms
minimal forms
nested forms
large symbolic forms
large generated forms
deep nesting
large identifiers
large numeric values
large collections
many operations
many resources
many capabilities
many modules
large distributed descriptions
large quantum programs
large HDL structures

The purpose is to detect artificial language limits.

---

76. Scalability tests

Scalability tests MUST verify that source semantics are not tied to a fixed machine size.

A scalable test SHOULD use symbolic quantities.

Conceptually:

N = symbolic

rather than:

N = permanently fixed hardware capacity

The same semantic source MUST be tested against multiple realization scales where practical.

---

77. POCO-REAF portability tests

The conformance suite MUST test the same source program against multiple realization classes where supported:

tiny
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
distributed
future-capability mock target

The exact physical availability of a target is not a grammar requirement.

The test harness MAY use capability/resource simulators.

The semantic result MUST remain equivalent unless the source explicitly permits a semantic distinction.

---

78. Fuzzing conformance

Production grammar MUST be fuzz-tested where tooling permits.

Fuzzing MUST target:

lexer
parser
expression precedence
type syntax
nested declarations
quantum operations
HDL constructs
resource expressions
contracts
policies
dialects
macro input
malformed source
large source
boundary source

Fuzzing MUST not reveal:

- parser crashes;
- nontermination;
- uncontrolled memory growth caused by ordinary valid input;
- inconsistent tokenization;
- inconsistent AST mapping;
- diagnostic panics.

---

79. Differential conformance

Where multiple frontend implementations exist, they MUST be tested against the same corpus.

For example:

ANTLR lexer
↔ Rust lexer

and:

ANTLR parser
↔ Rust parser/frontend

The implementations MAY differ internally.

They MUST agree on the language contract.

---

80. Golden conformance

Canonical fixtures SHOULD exist under:

grammar/golden/

with applicable categories:

lexer/
parser/
ast/
semantic/
diagnostics/

Golden outputs MUST be versioned.

A deliberate change MUST include an explicit compatibility decision.

---

81. Example conformance

Canonical examples SHOULD exist under:

grammar/examples/

including at minimum:

minimal.zm
classical.zm
quantum.zm
hdl.zm
hybrid.zm
ai.zm
distributed.zm
networking.zm
resource-aware.zm
policy-aware.zm
adaptive.zm
poco-reaf.zm

Examples are executable conformance fixtures where the implementation supports them.

---

82. Conformance fixture requirements

Every canonical example MUST identify:

purpose
features exercised
required capabilities
required resources
expected semantic result
expected diagnostics if negative
compatibility version
target assumptions

No example may accidentally imply a universal hardware limit.

---

83. Documentation conformance

Every stable grammar feature MUST be documented in the appropriate authority.

Documentation MUST agree with:

grammar
ownership
dependencies
status
AST mapping
semantic mapping
IR mapping
tests
compatibility

Documentation MUST NOT claim implementation that the status registry does not support.

---

84. Specification consistency

The validator MUST compare:

specification/
spec/
grammar/
OWNERSHIP.yaml
DEPENDENCIES.yaml
STATUS.yaml

for contradictions.

Examples of contradictions include:

spec says keyword exists
grammar does not accept it

grammar accepts feature
spec forbids it

STATUS says STABLE
tests are absent

OWNERSHIP assigns rule A
grammar defines rule A under owner B

DEPENDENCIES permits cycle
ANTLR import creates cycle

Any unresolved contradiction blocks freeze.

---

85. Historical document conformance

"grammar/Zamani-Grammar.md" is not allowed to become an accidental second language authority.

If it contains syntax or semantics not represented in the normative specification, each item MUST be classified as:

STABLE
PROPOSED
EXPERIMENTAL
DEPRECATED
HISTORICAL
NOT_IMPLEMENTED

Historical material MUST NOT silently become active syntax.

---

86. Status consistency

"grammar/STATUS.yaml" MUST agree with actual evidence.

The following are insufficient evidence by themselves:

file exists
rule exists
comment says implemented
README says supported
example exists
ANTLR generates code

Implementation status requires the applicable downstream evidence.

---

87. Freeze eligibility

A file is "READY" for freezing only when:

authority = resolved
owner = resolved
dependencies = resolved
imports = resolved
exports = resolved
AST mapping = complete
semantic mapping = complete
IR mapping = complete where applicable
diagnostics = complete
tests = complete
compatibility = complete
scalability audit = pass
POCO-REAF audit = pass
hard-coding audit = pass
ambiguity audit = pass
path audit = pass
Rust integration = pass where applicable

---

88. Freeze invariant

A frozen file MUST NOT require semantic modification merely because a downstream consumer or target is added.

Adding:

new GPU
new QPU
new FPGA
new CPU
new accelerator
new backend
new dialect
new runtime
new target

MUST NOT require editing a frozen universal grammar file solely to recognize the new target.

The new target belongs downstream.

---

89. Allowed post-freeze changes

A frozen file MAY receive:

mechanical generated changes
documentation corrections that do not change semantics
tooling metadata corrections
security fixes
explicitly versioned language changes

A semantic modification requires a controlled unfreeze/version transition.

---

90. Downstream addition rule

Adding a new consumer MUST NOT require editing the producer.

For example:

new backend
    ↓
consumes existing canonical IR

rather than:

new backend
    ↓
modify core grammar

This is a mandatory architectural property.

---

91. Upstream dependency change rule

If a frozen file gains a new semantic dependency:

dependency graph changes
        ↓
affected contract is reopened
        ↓
affected downstream dependents are reevaluated
        ↓
compatibility decision
        ↓
new freeze

A dependency cannot be silently introduced after freeze.

---

92. Directory freeze

A directory can be frozen only when:

- every active file has a valid status;
- every file has one owner;
- all imports resolve;
- dependency closure passes;
- no unresolved duplicate concept exists;
- all required mappings exist;
- all tests pass;
- all required audits pass;
- all child files satisfy freeze criteria;
- no undocumented file remains;
- no forbidden hard-coded capacity exists.

---

93. Freeze-stage dependency order

The recommended freeze sequence is:

FREEZE 0
Inventory and audit

FREEZE 1
Authority and specification

FREEZE 2
Lexical system

FREEZE 3
Core names/modules/declarations

FREEZE 4
Type system and memory

FREEZE 5
Expressions

FREEZE 6
Functions and statements

FREEZE 7
Effects/resources/capabilities

FREEZE 8
Contracts/policies/security

FREEZE 9
Classical/data

FREEZE 10
Concurrency/distributed/networking

FREEZE 11
Hardware/execution/compile

FREEZE 12
Quantum

FREEZE 13
HDL

FREEZE 14
Hybrid

FREEZE 15
AI

FREEZE 16
Interoperability/dialects

FREEZE 17
Macros/metaprogramming

FREEZE 18
Compatibility

FREEZE 19
Root composition

FINAL
grammar/ frozen

A stage MUST NOT be frozen while mandatory dependencies remain unstable.

---

94. Stage completion contract

A stage is complete only when:

stage specification complete
stage inventory complete
stage ownership complete
stage dependencies complete
stage imports resolve
stage exports resolve
stage AST mappings complete
stage semantic mappings complete
stage IR mappings complete
stage diagnostics complete
stage tests pass
stage hard-coding audit passes
stage portability audit passes
stage scalability audit passes
stage compatibility audit passes
stage freeze audit passes

---

95. Root-composition freeze

The root is intentionally frozen last.

The final sequence is:

lexer
    ↓
core
    ↓
types
    ↓
expressions
    ↓
statements/functions
    ↓
semantic control plane
    ↓
domains
    ↓
dialects/interoperability
    ↓
compatibility
    ↓
ANTLR parser root
    ↓
Zamani.g4

This prevents the root grammar from becoming a dumping ground for unresolved semantics.

---

96. Production gate: architecture

The following MUST pass:

[ ] One language architecture
[ ] One authority hierarchy
[ ] One lexical authority
[ ] One parser boundary
[ ] One domain-neutral AST
[ ] One shared semantic foundation
[ ] Canonical IR architecture
[ ] Canonical quantum IR boundary
[ ] No competing semantic architecture
[ ] No unresolved architectural contradiction

---

97. Production gate: repository

[ ] Complete inventory
[ ] Every file classified
[ ] Every directory classified
[ ] No unexplained files
[ ] No malformed paths
[ ] No duplicate paths
[ ] No unresolved duplicate concepts
[ ] Generated artifacts identified
[ ] Historical artifacts identified
[ ] Quarantined artifacts isolated

---

98. Production gate: ownership

[ ] Every exported token has one owner
[ ] Every exported rule has one owner
[ ] Every semantic construct has one owner
[ ] Every diagnostic has one owner
[ ] Every compatibility contract has one owner
[ ] No duplicate ownership
[ ] No implicit ownership

---

99. Production gate: dependencies

[ ] Every dependency declared
[ ] Every dependency resolves
[ ] No production dependency cycle
[ ] ANTLR imports are acyclic
[ ] No lower-to-upper dependency violation
[ ] No grammar-to-backend dependency
[ ] No grammar-to-test dependency
[ ] No grammar-to-example dependency
[ ] No physical-inventory dependency

---

100. Production gate: lexer

[ ] One lexer boundary
[ ] One token registry
[ ] Every token documented
[ ] Every token owned
[ ] Token compatibility defined
[ ] Contextual keywords defined
[ ] Duplicate token audit passes
[ ] Lexer/Rust behavior reconciled

---

101. Production gate: parser

[ ] One parser root
[ ] All imports resolve
[ ] All exported rules owned
[ ] No accidental ambiguity
[ ] Precedence validated
[ ] Associativity validated
[ ] Reachability validated
[ ] Error recovery validated
[ ] Source spans preserved

---

102. Production gate: AST

[ ] Every meaning-bearing rule mapped
[ ] Domain-neutral AST preserved
[ ] Source spans preserved
[ ] Names preserved
[ ] Types preserved
[ ] Effects preserved
[ ] Capabilities preserved
[ ] Resources preserved
[ ] Policies preserved
[ ] Contracts preserved
[ ] Provenance preserved

---

103. Production gate: semantics

[ ] Type semantics complete
[ ] Effect semantics complete
[ ] Capability semantics complete
[ ] Resource semantics complete
[ ] Contract semantics complete
[ ] Policy semantics complete
[ ] Provenance semantics complete
[ ] Security semantics complete
[ ] Domain semantics complete

---

104. Production gate: IR

[ ] Canonical IR identified
[ ] Classical IR mapping complete
[ ] quantum::ir mapping complete
[ ] HDL/hardware mapping complete where applicable
[ ] No hidden competing IR
[ ] No syntax-to-vendor-IR shortcut
[ ] IR compatibility defined

---

105. Production gate: portability

[ ] No fixed machine capacity
[ ] No fixed device inventory
[ ] Symbolic resource requirements
[ ] Capability negotiation
[ ] Abstract topology
[ ] Target-independent semantics
[ ] Explicit fallback
[ ] Incompatibility diagnostics
[ ] Target-specific behavior isolated downstream

---

106. Production gate: scalability

[ ] Tiny execution supported by semantics
[ ] Large execution represented without grammar redesign
[ ] Symbolic scale supported
[ ] No artificial resource ceiling
[ ] No finite device universe
[ ] No fixed thread maximum
[ ] No fixed memory maximum
[ ] No fixed qubit maximum
[ ] No fixed node maximum
[ ] No fixed tensor maximum
[ ] No fixed network maximum

---

107. Production gate: safety

[ ] Rust 2021
[ ] Rust >= 1.97
[ ] unsafe forbidden
[ ] No unsafe production Rust
[ ] Safe abstraction at foreign boundaries
[ ] Dependency safety reviewed
[ ] Compiler checks pass
[ ] Lint checks pass

---

108. Production gate: compatibility

[ ] Language version defined
[ ] Lexer version defined
[ ] Parser version defined
[ ] AST version defined
[ ] Semantic version defined
[ ] IR version defined
[ ] Quantum IR version defined
[ ] Dialect version defined
[ ] Feature compatibility defined
[ ] Migration rules defined
[ ] Deprecation rules defined

---

109. Production gate: diagnostics

[ ] Lexical diagnostics
[ ] Syntax diagnostics
[ ] Structural diagnostics
[ ] Name diagnostics
[ ] Type diagnostics
[ ] Effect diagnostics
[ ] Capability diagnostics
[ ] Resource diagnostics
[ ] Contract diagnostics
[ ] Policy diagnostics
[ ] Compatibility diagnostics
[ ] Lowering diagnostics
[ ] Target diagnostics

---

110. Production gate: tests

[ ] Unit tests
[ ] Lexer tests
[ ] Parser tests
[ ] AST tests
[ ] Semantic tests
[ ] Type tests
[ ] Effect tests
[ ] Resource tests
[ ] Capability tests
[ ] Contract tests
[ ] Policy tests
[ ] Domain tests
[ ] Negative tests
[ ] Regression tests
[ ] Golden tests
[ ] Fuzz tests
[ ] Differential tests
[ ] Portability tests
[ ] Scalability tests

---

111. Hard-coding audit

The audit MUST search the complete repository for universal capacity assumptions.

At minimum, inspect for concepts corresponding to:

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

The audit MUST also detect semantically equivalent disguises.

Examples of non-conformant patterns include:

qubits = 128
devices = 8
threads <= 32
memory <= fixed_value
register_width = fixed_value
nodes = fixed_value

when presented as universal language limits.

A numerical literal is not automatically forbidden.

The question is whether it is incorrectly elevated into universal language semantics.

---

112. Acceptable constants

Constants MAY exist when they are:

- program values;
- test parameters;
- example parameters;
- symbolic defaults explicitly defined by the language;
- implementation configuration;
- target-specific metadata;
- dialect-specific constraints;
- physical measurements;
- user-selected constraints.

They MUST NOT masquerade as universal language capacity.

---

113. Ambiguity audit

Every grammar directory MUST pass:

duplicate-rule analysis
unreachable-rule analysis
left-recursion analysis
precedence analysis
ambiguity analysis
import-cycle analysis
token ambiguity analysis

Intentional ambiguity MUST be explicitly justified and bounded.

---

114. Semantic-boundary audit

The validator MUST identify where each decision occurs.

Example:

Decision| Grammar| AST| Semantic| Compiler| Runtime
Identifier syntax| ✓| | | | 
Type meaning| | ✓| ✓| | 
Resource requirement| ✓| ✓| ✓| | 
Capability requirement| ✓| ✓| ✓| | 
Physical device selection| | | | ✓| ✓
Physical qubit assignment| | | | ✓| ✓
Routing| | | | ✓| ✓
Scheduling| | | | ✓| ✓
Calibration| | | | | ✓
Hardware discovery| | | | | ✓

A grammar file that performs a downstream decision fails this audit.

---

115. Vendor isolation

Vendor-specific behavior MUST remain behind:

dialects
interoperability
capabilities
target contracts
backend lowering
runtime adapters

Universal grammar MUST NOT acquire vendor-specific syntax merely to support a new device.

---

116. Future-target conformance

The grammar MUST remain valid when a new target is introduced.

A conformance test SHOULD simulate an unknown future target with:

new capabilities
new resource kinds
new topology
new execution model

The grammar architecture passes if the new target can consume established semantic contracts without modifying universal grammar semantics.

---

117. New-operation conformance

For open-ended operation domains, a new operation MUST be addable through:

metadata
dialect registration
operation registry
semantic registration
capability declaration

where applicable.

A new operation MUST NOT require a core parser rewrite unless its syntax genuinely introduces a new syntactic category.

---

118. New-domain conformance

A new computational domain MUST be integrable through:

domain syntax
domain semantic mapping
common types
common effects
common capabilities
common resources
common policies
common provenance
canonical IR boundary
tests
compatibility

Adding the domain MUST NOT require rewriting unrelated existing domain semantics.

---

119. New-backend conformance

A new backend MUST consume established semantic or IR contracts.

It MUST NOT require:

new universal keyword
new universal token
new universal type
new universal resource limit
new universal grammar root

unless the capability is genuinely language-wide and has passed the normal language-change process.

---

120. Conformance evidence

Every PASS result MUST have evidence.

Evidence MAY include:

test result
validator result
generated artifact
AST snapshot
semantic snapshot
IR snapshot
compatibility record
review record
static analysis result
fuzzing result

A checkbox without evidence is not a conformance result.

---

121. Failure handling

If any mandatory gate fails:

status = FAIL
freeze = BLOCKED

The failure MUST identify:

file
rule
owner
stage
dependency
diagnostic
required correction

A failure MUST NOT be hidden by changing status to "EXPERIMENTAL".

---

122. Quarantine

A problematic file MAY be marked:

QUARANTINED

when it cannot yet participate safely in the production grammar.

A quarantined file:

- MUST NOT be imported by frozen production grammar;
- MUST NOT export stable symbols;
- MUST NOT satisfy a production dependency;
- MUST have an owner;
- MUST have a reason;
- MUST have a resolution plan.

---

123. Orphan detection

A grammar file is an orphan when:

it is not imported
AND
it is not a documented composition root
AND
it is not a documented compatibility artifact
AND
it is not a documented historical/experimental artifact

Orphans MUST be classified or removed before directory freeze.

---

124. Dead-export detection

An exported rule with no consumer MUST be classified.

Possible states:

public API
future extension point
test-only
compatibility
deprecated
historical
unused

An unexplained dead export blocks stable freeze.

---

125. Public API conformance

A grammar rule becomes part of the public language API only when:

owned
documented
specified
tested
AST-mapped
semantically mapped where applicable
compatibility-mapped

Internal helper rules MUST be clearly distinguished from public rules.

---

126. Generated-file conformance

Generated grammar or documentation artifacts MUST identify:

generated: true
generator
source authority
generation method
do-not-edit status

Generated files MUST NOT become accidental authorities.

---

127. Machine-checkable traceability

The production system SHOULD be able to traverse:

feature
→ grammar file
→ owner
→ dependencies
→ exported rule
→ AST mapping
→ semantic mapping
→ IR mapping
→ diagnostic
→ compatibility
→ tests
→ freeze status

A missing edge is a conformance defect.

---

128. Minimum feature traceability record

Each production feature MUST be traceable to:

feature:
  syntax:
  owner:
  specification:
  grammar:
  ast:
  semantic:
  effects:
  capabilities:
  resources:
  contracts:
  policies:
  provenance:
  ir:
  diagnostics:
  compatibility:
  tests:
  status:
  freeze:

Fields that do not apply MUST explicitly say:

NOT_APPLICABLE

rather than being silently omitted when the omission would be ambiguous.

---

129. Cross-file integration rule

When completing a file, its contract MUST identify all known integration points in advance.

A later file MAY consume the existing exported contract.

A later file MUST NOT require the earlier file to be semantically rewritten merely to establish a normal consumer relationship.

This is a fundamental freeze invariant.

---

130. Completion definition for an individual file

A file is DONE only when:

[ ] Purpose complete
[ ] Authority identified
[ ] Owner identified
[ ] Non-ownership identified
[ ] Inputs identified
[ ] Outputs identified
[ ] Dependencies declared
[ ] Imports declared
[ ] Exports declared
[ ] Consumers declared
[ ] AST owner declared
[ ] Semantic owner declared
[ ] IR owner declared
[ ] Runtime owner declared
[ ] Tooling owner declared
[ ] Diagnostic owner declared
[ ] Test owner declared
[ ] Compatibility contract complete
[ ] Scalability contract complete
[ ] POCO-REAF contract complete
[ ] Hard-coding audit complete
[ ] Ambiguity audit complete
[ ] Error behavior complete
[ ] Positive tests complete
[ ] Negative tests complete
[ ] Boundary tests complete
[ ] Integration tests complete where applicable
[ ] Documentation complete
[ ] Status recorded
[ ] Freeze criteria satisfied

---

131. Completion definition for a directory

A directory is DONE only when:

[ ] all files classified
[ ] all files owned
[ ] all dependencies resolved
[ ] all imports resolved
[ ] all exports resolved
[ ] all AST mappings resolved
[ ] all semantic mappings resolved
[ ] all IR mappings resolved
[ ] all diagnostics resolved
[ ] all compatibility records resolved
[ ] all tests pass
[ ] hard-coding audit passes
[ ] ambiguity audit passes
[ ] path audit passes
[ ] scalability audit passes
[ ] POCO-REAF audit passes
[ ] no unresolved duplicate concepts
[ ] no orphaned active grammar
[ ] freeze record complete

---

132. Complete grammar freeze

"grammar/" may be declared fully frozen only when:

[ ] authority hierarchy frozen
[ ] inventory frozen
[ ] ownership frozen
[ ] dependency graph frozen
[ ] lexical architecture frozen
[ ] parser architecture frozen
[ ] AST contracts frozen
[ ] semantic contracts frozen
[ ] canonical IR contracts frozen
[ ] quantum::ir boundary frozen
[ ] diagnostics frozen
[ ] compatibility frozen
[ ] portability verified
[ ] scalability verified
[ ] hard-coding audit passes
[ ] Rust safety audit passes
[ ] all mandatory tests pass
[ ] all freeze stages pass
[ ] no unresolved quarantine blocks production
[ ] no unresolved duplicate owner exists
[ ] no unresolved dependency cycle exists

---

133. Final production statement

The grammar subsystem is production-ready only when the repository can truthfully establish:

Zamani source
    ↓
one canonical lexical architecture
    ↓
one canonical parser architecture
    ↓
one domain-neutral AST
    ↓
one shared semantic foundation
    ↓
types
effects
capabilities
resources
contracts
policies
provenance
    ↓
domain semantics
    ↓
Classical IR / quantum::ir / HDL-Hardware IR
    ↓
target-independent optimization
    ↓
lowering
    ↓
capability/resource negotiation
    ↓
routing
    ↓
scheduling
    ↓
resilience
    ↓
ZQN
    ↓
HAL
    ↓
available execution environment

The source language MUST describe portable meaning.

The realization system MUST determine how that meaning is executed.

The grammar MUST NOT encode arbitrary physical capacity limits.

The compiler/runtime MUST reject impossible realizations rather than silently changing semantics.

The language MUST support scaling from the smallest feasible execution environment to arbitrarily large feasible environments without redesigning the universal grammar around a finite hardware inventory.

---

134. Final acceptance criteria

The following statement is the final production gate:

«"grammar/" is production-ready only when every normative language construct has exactly one authority, exactly one owner, a resolved dependency path, a defined AST mapping, a defined semantic mapping, a defined IR mapping where applicable, deterministic diagnostics, compatibility behavior, conformance evidence, scalability evidence, POCO-REAF evidence, and no forbidden hardware-capacity assumption.»

And:

«A frozen file MUST remain semantically complete when new downstream consumers, devices, accelerators, execution environments, dialects, or future computational targets are added.»

And:

«New capabilities MUST be integrated through stable contracts, not by destabilizing already-frozen language foundations.»

And:

«Rust 1.97 or later, Rust 2021, and safe Rust are mandatory implementation constraints.»

---

135. Required relationship with repository control files

The following relationship is mandatory:

MANIFEST.yaml
    owns inventory
        ↓
OWNERSHIP.yaml
    owns symbol ownership
        ↓
DEPENDENCIES.yaml
    owns dependency graph
        ↓
STATUS.yaml
    owns lifecycle/result status
        ↓
CONFORMANCE.md
    defines conformance gates
        ↓
FREEZE.md
    defines freeze procedure
        ↓
grammar files
    implement the language contracts

These files MUST remain mutually consistent.

No file may silently redefine another file's authority.

---

136. Final freeze rule

The decisive rule is:

«No file is frozen merely because its local grammar works. A file is frozen only when its complete upstream contract, downstream integration contract, semantic boundary, scalability model, compatibility behavior, diagnostics, tests, and production conformance evidence are complete.»

Once frozen:

«Adding a downstream feature, backend, target, device, dialect, or consumer MUST NOT require semantic modification of the frozen file merely to integrate that addition.»

This is the required mechanism for building Zamani stage-by-stage without accumulating architectural rework.

---

137. Conformance result vocabulary

Automated tooling MUST use the following result vocabulary consistently:

PASS
FAIL
BLOCKED
NOT_APPLICABLE
NOT_TESTED
PARTIAL
QUARANTINED

A stage MUST NOT be marked "PASS" when any mandatory child result is "FAIL", "BLOCKED", or "NOT_TESTED".

---

138. Production declaration

The repository MAY declare:

production_ready: true
freeze_allowed: true

only after this document's mandatory gates have been executed and all required evidence is recorded in the corresponding machine-readable status and contract files.

Until then:

production_ready: false
freeze_allowed: false

is the correct repository state.

---

139. End state

When all gates pass, "grammar/" becomes the stable source-language contract.

The compiler, runtime, target adapters, QPU integrations, GPU integrations, FPGA integrations, ASIC integrations, distributed systems, simulators, operating systems, and future execution environments may evolve independently behind the established semantic and IR boundaries.

The language therefore remains:

portable
scalable
target-independent
resource-aware
capability-aware
policy-aware
effect-aware
contract-aware
provenance-aware
versioned
extensible
safe

while preserving:

Program Once
Compile Once
Run Everywhere
Anywhere
Forever

without embedding an artificial finite hardware universe into the language itself.
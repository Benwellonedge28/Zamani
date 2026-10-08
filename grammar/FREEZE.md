Zamani Grammar Freeze Specification

Path: "grammar/FREEZE.md"
Document Kind: Normative Freeze Protocol
Status: Production-Ready Specification
Authority: "grammar/specification/" + this freeze protocol
Applies To: "grammar/" and all descendants
Language: Zamani
Minimum Rust: Rust "1.97" or later
Rust Edition: "2021"
Unsafe Rust: Forbidden
Scalability: Resource-bounded only; no universal finite capacity
Portability Model: Target-independent
POCO-REAF: Required

---

1. Purpose

This document defines the complete procedure for making the Zamani "grammar/" directory production-ready and progressively freezing it so that completed language stages do not require semantic rework when later stages, domains, hardware targets, compiler backends, runtimes, or implementations are added.

The freeze system exists to guarantee:

1. one authoritative language definition;
2. one lexical authority;
3. one parser composition root;
4. one ownership authority;
5. one dependency graph;
6. one domain-neutral AST contract;
7. one shared semantic model;
8. explicit domain boundaries;
9. explicit AST, semantic, and IR mappings;
10. target-independent source semantics;
11. symbolic resource requirements;
12. capability-based target realization;
13. no hard-coded machine capacities;
14. deterministic dependency ordering;
15. reproducible language evolution;
16. backward/forward compatibility rules;
17. Rust "1.97+" compatibility;
18. complete prohibition of "unsafe";
19. no accidental semantic coupling between domains;
20. no downstream change requiring semantic edits to a properly frozen upstream file.

Freeze is therefore a dependency-closure guarantee, not merely a source-control operation.

---

2. Core Freeze Principle

A grammar unit may be frozen only when:

«Everything it depends on is stable enough to serve as a permanent contract, and everything it exports is completely specified for all required downstream consumers.»

A frozen unit must not later require semantic modification merely because:

- a new domain is added;
- a new compiler backend is added;
- a new processor is added;
- a new accelerator is added;
- a new QPU is added;
- a new FPGA is added;
- a new ASIC is added;
- a new operating system is added;
- a new runtime is added;
- a larger machine is used;
- a smaller machine is used;
- a distributed environment is introduced;
- a new dialect is introduced;
- a new implementation strategy is introduced.

A downstream consumer may be added without modifying the frozen producer when the producer's existing contract is sufficient.

---

3. What "Frozen" Means

"FROZEN" means that the semantic contract of a grammar unit is closed.

A frozen unit has:

- an identified owner;
- an explicit non-ownership boundary;
- resolved dependencies;
- stable imports;
- stable exports;
- unique exported symbols;
- complete AST mappings;
- complete semantic mappings;
- complete IR mappings where applicable;
- complete diagnostics;
- complete compatibility behavior;
- complete source-span behavior;
- complete error behavior;
- complete effect/resource/capability/policy integration;
- complete provenance requirements;
- complete scalability audit;
- complete POCO-REAF audit;
- complete hard-coding audit;
- complete ambiguity audit;
- complete conformance tests;
- complete negative tests;
- complete integration tests;
- no unresolved cycles;
- no unexplained duplicate concepts;
- no unresolved path anomalies;
- no unsafe Rust dependency or requirement;
- no dependency on implementation-specific capacity;
- no unresolved TODO that affects language semantics.

A frozen file is not merely "currently working."

It is contract-complete.

---

4. Freeze Status Model

Every grammar unit MUST have one of these statuses:

UNASSESSED
INVENTORIED
DESIGNED
IMPLEMENTED
INTEGRATED
VALIDATED
FREEZE_READY
FROZEN
DEPRECATED
QUARANTINED
REJECTED
HISTORICAL

4.1 UNASSESSED

The file exists but has not yet been evaluated.

No downstream freeze may depend on it.

4.2 INVENTORIED

The file has:

- a canonical path;
- a declared kind;
- an owner;
- a status;
- identified exports;
- identified imports;
- identified dependencies.

Semantic freezing is not permitted.

4.3 DESIGNED

The file has a complete language-design contract but may not yet have a complete implementation.

4.4 IMPLEMENTED

The grammar or specification content exists and satisfies its declared design.

4.5 INTEGRATED

All declared integration boundaries have been connected.

4.6 VALIDATED

The file has passed:

- syntax checks;
- dependency checks;
- ownership checks;
- ambiguity checks;
- mapping checks;
- scalability checks;
- hard-coding checks;
- conformance tests.

4.7 FREEZE_READY

All freeze prerequisites pass, but the repository freeze gate has not yet been formally applied.

4.8 FROZEN

The unit is contractually closed.

Only permitted non-semantic maintenance may occur.

4.9 DEPRECATED

The unit remains recognized for compatibility but must not receive new semantics.

4.10 QUARANTINED

The unit exists in the repository but cannot participate in the production dependency graph.

Examples include:

- duplicate concepts;
- malformed paths;
- obsolete experimental grammar;
- unresolved ownership;
- ambiguous semantics;
- historical artifacts.

4.11 REJECTED

The unit is explicitly not part of the Zamani language.

4.12 HISTORICAL

The unit documents prior designs and has no normative language authority.

---

5. Authority Hierarchy

The freeze process MUST preserve this authority hierarchy:

language principles
        |
        v
grammar/specification/
        |
        v
grammar/spec/
        |
        v
grammar/OWNERSHIP.yaml
        |
        v
grammar/DEPENDENCIES.yaml
        |
        v
grammar/STATUS.yaml
        |
        v
modular grammar contracts
        |
        v
grammar/antlr/
        |
        v
grammar/Zamani.g4
        |
        v
Rust lexer/parser
        |
        v
domain-neutral AST
        |
        v
semantic model
        |
        v
canonical IR
        |
        v
compiler/runtime

The following are not competing language authorities:

grammar/README.md
grammar/Zamani-Grammar.md
grammar/reference/
grammar/tests/
grammar/examples/
grammar/golden/

"README.md" is navigation.

"Zamani-Grammar.md" is historical/extended reference material unless a specific section is explicitly promoted into the normative specification.

Tests demonstrate conformance; they do not define semantics.

---

6. Required Control Files

The freeze system depends on these repository-level contracts:

grammar/MANIFEST.yaml
grammar/OWNERSHIP.yaml
grammar/DEPENDENCIES.yaml
grammar/STATUS.yaml
grammar/CONFORMANCE.md
grammar/FREEZE.md
grammar/DESIGN.md
grammar/specification/
grammar/spec/

These files have distinct responsibilities.

File| Responsibility
"MANIFEST.yaml"| Complete repository inventory
"OWNERSHIP.yaml"| One-owner-per-symbol contract
"DEPENDENCIES.yaml"| Canonical dependency DAG
"STATUS.yaml"| Feature/file implementation status
"CONFORMANCE.md"| Production conformance requirements
"FREEZE.md"| Freeze protocol
"DESIGN.md"| Architectural design
"specification/"| Human normative language authority
"spec/"| Machine-oriented contracts

No one file may silently assume the responsibilities of another.

---

7. Required Contract for Every Production File

Every production ".g4", normative ".yaml", and normative specification unit MUST declare or be covered by an equivalent machine-readable contract containing:

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

A file may inherit directory-level contracts only where the inherited contract is mechanically unambiguous.

A file-specific exception MUST be explicitly declared.

---

8. Ownership Freeze Rule

Every exported grammar symbol MUST have exactly one owner.

Formally:

symbol -> exactly one authoritative owner

The following are forbidden:

symbol -> owner A
symbol -> owner B

unless one is explicitly marked as:

- compatibility alias;
- deprecated alias;
- generated facade;
- historical reference.

Even then, only one unit owns the semantic meaning.

---

9. Dependency Freeze Rule

All production dependencies MUST form a directed acyclic graph.

Cycles are forbidden.

If:

A -> B
B -> C
C -> A

is discovered, the architecture MUST be refactored.

The preferred correction is:

A -> Shared Contract
B -> Shared Contract
C -> Shared Contract

rather than introducing mutual dependencies.

---

10. Dependency Categories

"DEPENDENCIES.yaml" MUST distinguish at least:

antlr_import
semantic
contract
authority_reference
tooling
test
generated
repository_integration
compatibility

A documentation relationship MUST NOT be represented as an ANTLR import.

A test dependency MUST NOT become a production grammar dependency.

A runtime implementation MUST NOT become a grammar dependency.

---

11. Production Dependency Direction

The default dependency direction is:

lexical
   |
   v
core
   |
   v
types
   |
   v
expressions
   |
   +----------------------+
   |                      |
   v                      v
declarations          functions/modules
   |                      |
   +----------+-----------+
              |
              v
        statements/memory
              |
              v
 effects/resources/capabilities
              |
              v
       contracts/policies/security
              |
              v
       classical/data
              |
              v
 concurrency/distributed/networking
              |
              v
 hardware/execution/compile
              |
       +------+------+
       |             |
       v             v
    quantum         HDL
       |             |
       +------+------+
              |
              v
           hybrid
              |
              v
              AI
              |
              v
      dialects/interoperability
              |
              v
     macros/metaprogramming
              |
              v
        compatibility
              |
              v
       root composition

This is a default architectural ordering.

A file-specific dependency is permitted only when it is explicitly justified and does not create semantic leakage or a cycle.

---

12. Root Composition Freeze

The final parser architecture MUST have these boundaries:

grammar/lexer/tokens.g4
        |
        v
grammar/lexer/lexer.g4
        |
        v
grammar/antlr/ZamaniLexer.g4
        |
        v
grammar/antlr/ZamaniParser.g4
        |
        v
grammar/Zamani.g4

The root grammar is a composition root.

It is not a semantic owner for every domain.

---

13. Lexer Freeze

The lexer may freeze only after:

- every token has exactly one owner;
- every token has a spelling or lexical rule;
- token category is known;
- contextual/reserved behavior is known;
- compatibility behavior is known;
- deprecation behavior is known;
- token tests exist;
- token collisions are resolved;
- literal classes are resolved;
- operator precedence implications are documented;
- generated lexer validation succeeds.

No domain grammar may silently introduce a competing public token.

---

14. Parser Freeze

The parser may freeze only after:

- every exported rule has an owner;
- all imports resolve;
- all parser roots are known;
- all domain dispatchers are registered;
- all ambiguities are resolved or intentionally specified;
- precedence is deterministic;
- left recursion is intentional and ANTLR-compatible;
- unreachable rules are removed or explicitly justified;
- error recovery behavior is defined;
- source-span preservation is defined;
- AST mappings exist.

---

15. "grammar/Zamani.g4" Freeze Contract

"Zamani.g4" MUST remain a thin composition layer.

It may own only universal dispatch/composition such as:

program
sourceUnit
item
universal declaration dispatch
universal statement dispatch
universal expression dispatch
universal type dispatch
domain dispatch
EOF

It MUST NOT own:

- token definitions;
- hardware limits;
- quantum operation inventories;
- vendor-specific syntax;
- AI application semantics;
- HDL implementation semantics;
- physical scheduling;
- routing;
- calibration;
- backend-specific IR.

It freezes only after every subordinate parser contract is stable.

---

16. AST Freeze

No grammar stage is fully frozen until its mapping into the domain-neutral AST is known.

Every production rule that contributes semantic structure MUST specify:

source rule
AST node
source spans
children
attributes
modifiers
effects
capabilities
resources
contracts
policies
provenance

A grammar rule without a defined semantic representation is not freeze-ready.

---

17. Semantic Model Freeze

The AST MUST NOT be treated as the final semantic representation.

The architecture is:

source
  |
  v
parse tree
  |
  v
domain-neutral AST
  |
  v
structural validation
  |
  +-- types
  +-- effects
  +-- capabilities
  +-- resources
  +-- contracts
  +-- policies
  +-- provenance
  |
  v
semantic model

A grammar stage cannot freeze if its semantics depend on undocumented interpretation performed later by an arbitrary backend.

---

18. IR Freeze

Domain syntax MUST NOT directly depend on vendor-specific IR.

The required pattern is:

Zamani syntax
    |
    v
domain-neutral AST
    |
    v
semantic domain model
    |
    v
canonical domain IR
    |
    v
optimization/lowering
    |
    v
target realization

Required canonical domains include, where applicable:

Classical IR
quantum::ir
HDL/Hardware IR

No competing hidden IR may be introduced by an individual grammar directory.

---

19. Domain Freeze Rule

Every domain MUST depend on shared semantic infrastructure.

Domains MUST reuse:

types
expressions
effects
resources
capabilities
contracts
policies
provenance

Domains MUST NOT create incompatible parallel versions of these systems.

For example:

quantum resource model
AI resource model
HDL resource model

must ultimately map to the common resource contract.

Domain-specific extensions are allowed.

Parallel semantic universes are not.

---

20. Classical Freeze

"classical/" may freeze only when its semantics are expressed through the common language model.

It MUST NOT hard-code:

- processor counts;
- register widths;
- memory capacities;
- vector lane counts;
- cache sizes;
- fixed accelerator counts.

Parameterized widths and resource requirements are permitted.

---

21. Quantum Freeze

"quantum/" may freeze only after:

types
expressions
effects
resources
capabilities
policies
hardware contracts
execution contracts

are stable.

Quantum operations MUST use a data-driven operation model.

The grammar MUST NOT require permanent enumeration of every possible operation.

The generic model is:

quantumOperation
operationSpecifier
targets
controls
parameters
results
attributes
modifiers

Operation identity may be:

builtin
custom
parameterized
vendor
dialect
future

Adding a new operation MUST NOT require redesigning the universal grammar.

---

22. Physical Quantum Resource Boundary

The source language may express requirements such as:

requires qubits >= n
requires capability("quantum.measurement")
requires capability("quantum.error_correction")
requires topology(...)

It MUST NOT encode universal physical limits.

The grammar MUST NOT define:

MAX_QUBITS

or any equivalent fixed global capacity.

Physical allocation belongs downstream.

---

23. HDL Freeze

"hdl/" may freeze only after:

- hardware capability semantics are stable;
- resource semantics are stable;
- timing semantics are stable;
- execution intent is stable;
- type integration is stable.

HDL syntax may express arbitrary parameterized widths and structures.

It MUST NOT impose universal limits such as:

32-bit
64-bit
8 lanes
1024 registers

as language-wide capacity restrictions.

---

24. Hybrid Freeze

"hybrid/" freezes only after its constituent domains are frozen sufficiently to provide stable contracts.

It may compose:

classical
quantum
HDL/hardware
accelerator
host/device

but MUST NOT redefine the semantics of those domains.

---

25. AI Freeze

"ai/" must use common:

types
data
effects
resources
capabilities
policies
provenance
contracts
concurrency
distributed

AI-specific operations must not create a separate type, effect, resource, or policy system.

Application-specific concepts belong in:

libraries
dialects
capabilities
policies
applications

rather than the universal core grammar.

---

26. Data Freeze

"data/" must remain independent of any one external data language.

External formats such as:

SQL
JSON
XML

are interoperability/dialect concerns.

The common language owns semantic data operations, not every external syntax.

---

27. Concurrency and Distribution Freeze

The source language MUST describe computational intent independently of physical execution units.

A computation may later realize as:

task
thread
actor
future
pipeline
GPU work
accelerator work
distributed process
cluster computation

The grammar must not require fixed quantities of these units.

---

28. Hardware Freeze

Hardware grammar describes:

capability
resource
topology
memory class
timing requirement
power requirement
reliability requirement
performance requirement
placement intent

It does not own:

physical inventory
device discovery
calibration
routing algorithms
scheduling algorithms
device allocation

Those belong downstream.

---

29. Execution Freeze

"execution/" owns execution intent and contracts.

It MUST NOT hard-code:

- available device counts;
- machine topology;
- CPU counts;
- GPU counts;
- QPU counts;
- physical memory limits.

Execution realization remains target-dependent.

---

30. Compile Freeze

"compile/" must define the target-independent compilation contract.

The portable compilation artifact MUST be capable of carrying:

language version
feature versions
AST/semantic version
IR version
dialect versions
resource requirements
capability requirements
constraints
preferences
effects
policies
contracts
provenance
reproducibility metadata
target-independent semantic representation

"Compile once" therefore means producing a portable semantic artifact.

It does not mean that one immutable machine-code binary can execute on every physically incompatible future machine.

---

31. POCO-REAF Freeze Contract

The grammar is POCO-REAF compliant only if:

Program Once

The programmer expresses portable semantics rather than one mandatory physical realization.

Compile Once

Compilation can produce a target-independent semantic artifact with versioned contracts.

Run Everywhere

A realization may be selected according to available capabilities and resources.

Anywhere

Fallback is explicit and semantically constrained.

Forever

Language, AST, semantic model, IR, dialect, and compatibility versions have defined migration and compatibility policies.

---

32. Resource Scalability Freeze

The language MUST scale from tiny systems to arbitrarily large systems subject to actual available resources.

Therefore:

resource quantity = symbolic/parameterized

not:

resource quantity = grammar-defined maximum

Allowed:

requires memory >= required_memory
requires qubits >= n
requires capability("gpu.compute")
requires capability("tensor.compute")
requires topology(...)

Also allowed:

prefer ...
constrain ...
allow ...
forbid ...

Forbidden:

MAX_MEMORY
MAX_THREADS
MAX_GPUS
MAX_CPUS
MAX_FPGAS
MAX_QUBITS
MAX_NODES
MAX_DEVICE_COUNT
MAX_TENSOR_RANK

and semantically equivalent disguises.

---

33. Hard-Coding Freeze Gate

Every production freeze MUST pass a hard-coding audit.

The audit searches for:

MAX_*
MIN_*
FIXED_*
DEFAULT_*_COUNT
*_COUNT = literal
*_WIDTH = literal
*_CAPACITY = literal

and equivalent semantic patterns.

The audit must inspect:

- ".g4";
- ".yaml";
- ".yml";
- ".md";
- examples;
- tests;
- generator metadata;
- source mappings.

A test fixture may intentionally use a finite value as test data, but it must never present that value as a universal language capacity.

---

34. Capability Freeze

Capability references MUST remain abstract.

Examples:

capability("gpu.compute")
capability("quantum.measurement")
capability("tensor.compute")
capability("network.low_latency")

The grammar must not encode the implementation identity unless the programmer explicitly requests a supported implementation-specific constraint.

Even then, implementation-specific selection must remain an extension rather than becoming the universal semantic contract.

---

35. Topology Freeze

Topology syntax may express abstract constraints.

Examples include:

requires topology(...)

The grammar MUST NOT assume a fixed number of:

nodes
devices
links
qubits
processors
accelerators

Topology realization belongs to compilation/runtime infrastructure.

---

36. Adaptation Freeze

Any source-level adaptation facility MUST be governed by:

policy
capability
effect
resource
authorization
provenance
contract

Adaptation MUST NOT implicitly authorize arbitrary self-modification.

A frozen adaptation grammar must therefore specify:

- what may change;
- who authorizes it;
- what effects are generated;
- what resources are required;
- how changes are recorded;
- how provenance is preserved;
- what policies constrain it;
- how failure is handled.

---

37. Reflection and Code Generation Freeze

Reflection and code generation are controlled capabilities.

They MUST integrate with:

effects
capabilities
policies
security
sandbox
provenance

Macros and metaprogramming MUST NOT bypass semantic validation.

Generated code must enter the normal validation pipeline.

---

38. Security Freeze

Security syntax defines intent.

It does not perform authorization itself.

The freeze contract must distinguish:

declared permission
requested capability
policy
authorization decision
runtime enforcement
audit

Runtime enforcement belongs outside grammar.

---

39. Dialect Freeze

Every dialect must define:

identity
namespace
version
dependencies
syntax
semantic mapping
capabilities
compatibility
registration
status
provenance

A dialect MUST NOT silently alter core semantics.

A dialect may extend the language only through an explicit extension contract.

---

40. Interoperability Freeze

Foreign formats and languages must remain boundaries.

Interoperability contracts must identify:

format
version
ABI
API
calling convention
data representation
serialization
foreign types
ownership
error model
effects
security
provenance
compatibility

Parsing a foreign format does not make that format part of Zamani core semantics.

---

41. Compatibility Freeze

Before any unit is frozen, its compatibility behavior must be declared.

At minimum:

language version
grammar version
lexer version
AST version
semantic version
IR version
dialect version
feature version
deprecation policy
migration policy

A semantic-breaking change requires a new version and cannot silently modify a frozen contract.

---

42. Versioning Rule

A frozen contract MUST NOT be changed invisibly.

Changes fall into:

Non-semantic

Examples:

- comments;
- formatting;
- documentation corrections;
- generator implementation changes that preserve output;
- tooling improvements.

These may be permitted without semantic unfreeze.

Compatible semantic extension

A new consumer or dialect may use an already-frozen export without modifying the producer.

Contract extension

A new semantic export or changed behavior requires versioned contract evolution.

Breaking change

Requires:

new version
migration specification
compatibility entry
updated conformance corpus
affected-stage revalidation

---

43. Frozen Dependency Rule

If:

A -> B

and "A" is frozen, adding a new consumer:

C -> A

MUST NOT require changing "A".

If adding "C" requires changing "A", one of the following is true:

1. "A" was frozen too early;
2. the abstraction boundary is incorrect;
3. the new functionality belongs in a new extension contract;
4. a shared contract must be extracted;
5. the change is genuinely a new language version.

The default response is not to edit the frozen producer.

---

44. New File Rule

A new file may be added to a frozen directory only if it conforms to that directory's frozen extension contract.

It must not:

- redefine existing symbols;
- modify existing semantics;
- introduce an undeclared dependency;
- bypass ownership;
- bypass validation;
- introduce a competing parser root;
- introduce a competing token authority;
- introduce a fixed capacity.

If the new file requires a semantic change to an existing frozen contract, it belongs to a new language/contract version.

---

45. New Domain Rule

A future computational domain must be able to be added without rewriting the universal language core.

A new domain should integrate through:

types
expressions
effects
resources
capabilities
policies
contracts
provenance
AST
semantic model
canonical IR
dialect/extension registration

It must not require:

core rewrite
lexer redesign
resource-model fork
effect-system fork
AST fork
semantic-model fork

unless the language itself is intentionally versioned.

---

46. Freeze Dependency Closure

Before freezing unit "X", recursively evaluate:

X
|
+-- direct dependencies
|
+-- dependencies of dependencies
|
+-- ownership
|
+-- AST mappings
|
+-- semantic mappings
|
+-- IR mappings
|
+-- diagnostics
|
+-- compatibility
|
+-- tests

The entire required closure must satisfy the freeze prerequisites.

No hidden dependency is permitted.

---

47. Freeze Readiness Matrix

Every production file MUST pass:

Gate| Required
Canonical path| Yes
Owner| Yes
Non-owner boundary| Yes
Imports| Yes
Exports| Yes
Dependency closure| Yes
AST mapping| Yes
Semantic mapping| Yes
IR mapping| Where applicable
Diagnostics| Yes
Source spans| Yes
Effects| Where applicable
Capabilities| Where applicable
Resources| Where applicable
Policies| Where applicable
Contracts| Where applicable
Provenance| Where applicable
Compatibility| Yes
Scalability audit| Yes
POCO-REAF audit| Yes
Hard-coding audit| Yes
Ambiguity audit| Yes
Positive tests| Yes
Negative tests| Yes
Integration tests| Yes
Documentation| Yes
Rust compatibility| Yes
Unsafe audit| Yes

---

48. Freeze Gate: Lexer

The lexer stage freezes only when:

[ ] token ownership complete
[ ] token spelling complete
[ ] contextual keyword rules complete
[ ] literal rules complete
[ ] operator rules complete
[ ] token collisions resolved
[ ] lexical ambiguity resolved
[ ] lexer diagnostics defined
[ ] token compatibility defined
[ ] generated lexer succeeds
[ ] lexer conformance passes
[ ] no domain-specific token leakage

---

49. Freeze Gate: Core

Core freezes only when:

[ ] names complete
[ ] identifiers complete
[ ] paths complete
[ ] visibility complete
[ ] source-unit structure complete
[ ] compilation-unit structure complete
[ ] universal attributes complete
[ ] generic requirements complete
[ ] capability references complete
[ ] policy references complete
[ ] version declarations complete
[ ] AST mappings complete
[ ] semantic mappings complete

---

50. Freeze Gate: Types

Types freeze only when:

[ ] primitive types
[ ] named types
[ ] generic types
[ ] function types
[ ] compound types
[ ] references
[ ] ownership-related types
[ ] linear/affine semantics
[ ] bounds
[ ] constraints
[ ] associated types
[ ] user-defined types
[ ] resource types
[ ] quantum types
[ ] hardware-related types

have explicit semantic boundaries and do not encode physical capacity.

---

51. Freeze Gate: Expressions

Expressions freeze only when:

[ ] precedence complete
[ ] associativity complete
[ ] literal expressions complete
[ ] name/path expressions complete
[ ] calls complete
[ ] indexing complete
[ ] member access complete
[ ] lambdas complete
[ ] closures complete
[ ] blocks complete
[ ] conditional expressions complete
[ ] match expressions complete
[ ] query expressions complete
[ ] domain expressions mapped
[ ] diagnostics complete

Duplicate expression grammars must be resolved before freeze.

---

52. Freeze Gate: Statements and Functions

Statements/functions freeze only when:

[ ] declaration interaction complete
[ ] control flow complete
[ ] function declarations complete
[ ] parameters complete
[ ] return semantics complete
[ ] generic functions complete
[ ] contracts complete
[ ] effects complete
[ ] resource requirements complete
[ ] capability requirements complete
[ ] asynchronous semantics complete
[ ] distributed semantics complete
[ ] foreign function boundaries complete

---

53. Freeze Gate: Effects

Effects freeze only when:

[ ] effect identity defined
[ ] parameters defined
[ ] composition defined
[ ] subeffect relationships defined
[ ] effect inference rules defined
[ ] capability relationship defined
[ ] policy relationship defined
[ ] diagnostics defined
[ ] provenance behavior defined

---

54. Freeze Gate: Resources

Resources freeze only when:

[ ] resource identity defined
[ ] symbolic quantity defined
[ ] requirements defined
[ ] constraints defined
[ ] preferences defined
[ ] allowances defined
[ ] prohibitions defined
[ ] capability relationship defined
[ ] topology relationship defined
[ ] fallback behavior defined
[ ] no fixed capacity exists

---

55. Freeze Gate: Policies and Security

These freeze only when:

[ ] policy syntax defined
[ ] scope defined
[ ] inheritance defined
[ ] precedence defined
[ ] conflict handling defined
[ ] authorization boundary defined
[ ] sandbox boundary defined
[ ] capability relationship defined
[ ] effect relationship defined
[ ] provenance defined
[ ] diagnostics defined

---

56. Freeze Gate: Classical and Data

These freeze only when:

[ ] common types reused
[ ] common expressions reused
[ ] effects reused
[ ] resources reused
[ ] capabilities reused
[ ] policies reused
[ ] data provenance defined
[ ] scaling is symbolic
[ ] accelerator independence maintained

---

57. Freeze Gate: Concurrency, Distributed, Networking

These freeze only when:

[ ] execution intent defined
[ ] abstract task model defined
[ ] synchronization defined
[ ] messaging defined
[ ] topology requirements defined
[ ] placement intent defined
[ ] failure semantics defined
[ ] recovery semantics defined
[ ] no fixed node/thread/device capacity exists

---

58. Freeze Gate: Hardware, Execution, Compile

These freeze only when:

[ ] target-independent intent defined
[ ] capability negotiation defined
[ ] resource negotiation defined
[ ] abstract topology defined
[ ] target selection boundary defined
[ ] lowering boundary defined
[ ] scheduling boundary defined
[ ] routing boundary defined
[ ] runtime discovery boundary defined
[ ] calibration boundary defined
[ ] reproducibility defined
[ ] artifact contract defined

---

59. Freeze Gate: Quantum

Quantum freezes only when:

[ ] generic operation model complete
[ ] operation specifier complete
[ ] target model complete
[ ] control model complete
[ ] parameter model complete
[ ] measurement model complete
[ ] state model complete
[ ] dynamic-circuit model complete
[ ] channel/noise model complete
[ ] error-correction intent complete
[ ] resource integration complete
[ ] capability integration complete
[ ] effect integration complete
[ ] policy integration complete
[ ] provenance integration complete
[ ] quantum::ir mapping complete
[ ] no fixed physical capacity exists

---

60. Freeze Gate: HDL

HDL freezes only when:

[ ] module model complete
[ ] port model complete
[ ] signal/net model complete
[ ] sequential semantics complete
[ ] combinational semantics complete
[ ] clock semantics complete
[ ] reset semantics complete
[ ] timing semantics complete
[ ] pipeline semantics complete
[ ] memory semantics complete
[ ] verification semantics complete
[ ] synthesis intent defined
[ ] hardware resource semantics integrated
[ ] no universal physical width/capacity limit exists

---

61. Freeze Gate: Hybrid

Hybrid freezes only when:

[ ] classical boundary complete
[ ] quantum boundary complete
[ ] hardware boundary complete
[ ] data exchange complete
[ ] synchronization complete
[ ] host/device semantics complete
[ ] resource negotiation complete
[ ] fallback rules complete
[ ] provenance complete

---

62. Freeze Gate: AI

AI freezes only when:

[ ] inference semantics defined
[ ] learning semantics defined
[ ] reasoning semantics defined
[ ] evidence semantics defined
[ ] provenance semantics defined
[ ] uncertainty semantics defined
[ ] model semantics defined
[ ] adaptation semantics defined
[ ] policy integration defined
[ ] resource integration defined
[ ] effect integration defined
[ ] distributed integration defined
[ ] no application-specific core dependency exists

---

63. Freeze Gate: Interoperability and Dialects

These freeze only when:

[ ] extension identity defined
[ ] versioning defined
[ ] registration defined
[ ] compatibility defined
[ ] semantic mapping defined
[ ] provenance defined
[ ] external ownership defined
[ ] foreign effects defined
[ ] foreign resource requirements defined
[ ] boundary diagnostics defined

---

64. Freeze Gate: Macros and Metaprogramming

These freeze only when:

[ ] expansion model defined
[ ] hygiene defined
[ ] token interaction defined
[ ] AST interaction defined
[ ] compile-time execution defined
[ ] reflection effects defined
[ ] code-generation effects defined
[ ] sandbox requirements defined
[ ] capability requirements defined
[ ] provenance defined
[ ] semantic validation cannot be bypassed

---

65. Freeze Gate: Compatibility

Compatibility freezes only when:

[ ] language version defined
[ ] grammar version defined
[ ] lexer version defined
[ ] AST version defined
[ ] semantic version defined
[ ] IR version defined
[ ] dialect version defined
[ ] deprecation policy defined
[ ] migration policy defined
[ ] compatibility matrix defined

---

66. Freeze Gate: Root Composition

The root composition stage is frozen LAST.

Required:

[ ] lexer root stable
[ ] parser root stable
[ ] every dispatcher stable
[ ] every imported grammar stable
[ ] no duplicate root
[ ] no domain semantics in root
[ ] generated parser succeeds
[ ] complete corpus parses
[ ] invalid corpus rejects correctly
[ ] AST generation succeeds
[ ] diagnostics are stable

---

67. Required Test Pyramid

Every freeze stage must pass the applicable test levels:

L0  lexical
L1  lexer
L2  parser
L3  AST
L4  structural validation
L5  semantic
L6  type/effect/resource
L7  domain semantics
L8  IR mapping
L9  compiler integration
L10 runtime/target integration
L11 portability
L12 scalability
L13 fuzzing
L14 differential/conformance

A stage does not need a physical-target test when its semantics are intentionally target-independent, but it MUST have a test proving that it remains target-independent.

---

68. Negative Tests Are Mandatory

Every frozen feature must have rejection tests for invalid uses.

Examples:

missing required capability
insufficient resource requirement
forbidden capability
invalid policy
invalid effect
invalid type
invalid quantum target
invalid HDL connection
invalid dialect version
invalid compatibility version
invalid provenance
invalid adaptation authorization

Negative tests are part of the language contract.

---

69. Ambiguity Freeze

A grammar may not freeze while it contains unexplained ambiguity.

The validation process must identify:

unreachable rules
duplicate alternatives
prefix ambiguity
precedence conflicts
associativity conflicts
lexer/parser conflicts
contextual keyword conflicts
dialect conflicts

An intentional ambiguity must be explicitly resolved by:

- precedence;
- context;
- disambiguation rule;
- dialect boundary;
- semantic validation.

"ANTLR happens to accept it" is not sufficient.

---

70. Duplicate Concept Freeze

Before freezing any directory, similarly named files must be classified.

Examples requiring explicit resolution include:

closure.g4
closures.g4

lambda.g4
lambdas.g4

map.g4
map-types.g4

option.g4
option-types.g4

array.g4
array-types.g4

Each pair must become one of:

distinct semantics
canonical + compatibility alias
canonical + deprecated alias
merged
quarantined
historical
rejected

No unexplained duplicate semantic ownership is allowed.

---

71. Path Hygiene Freeze

Production grammar paths MUST:

- use canonical "/";
- contain no control characters;
- contain no accidental trailing whitespace;
- avoid unexplained whitespace;
- avoid case-only collisions;
- avoid duplicate singular/plural paths without justification;
- avoid ambiguous generated/artifact names;
- avoid absolute paths;
- avoid "..";
- avoid platform-specific separators.

Malformed or suspicious paths are quarantined until resolved.

---

72. Generated Files

Generated files MUST be distinguishable from authoritative sources.

Generated output MUST NOT become an authority.

The relationship must be:

authoritative source
       |
       v
generator
       |
       v
generated artifact

not:

generated artifact
       |
       v
authoritative source

Generated changes do not constitute language changes when their authoritative inputs remain unchanged.

---

73. Rust Integration Freeze

The grammar must integrate with the Rust frontend without requiring unsafe code.

Required baseline:

Rust >= 1.97
edition = "2021"
unsafe = forbidden

The freeze process must validate:

cargo check
cargo test
cargo clippy

using the repository's applicable configurations.

Any unsafe Rust introduced by grammar tooling, parser integration, generators, validators, or generated integration code is a freeze failure unless the project explicitly changes this language-level requirement.

For the current Zamani architecture, the requirement is:

unsafe = forbidden

---

74. Rust Frontend Integration Boundary

The grammar integrates with the repository Rust frontend through stable contracts.

At minimum:

grammar lexer
    ->
src/lexer.rs

grammar parser
    ->
src/parser.rs

grammar AST contract
    ->
src/ast/mod.rs and owned AST modules

The grammar MUST NOT depend on Rust implementation details merely to define language semantics.

Rust code consumes grammar contracts.

The dependency direction remains:

grammar contract
        |
        v
Rust implementation

not:

Rust implementation detail
        |
        v
language grammar semantics

unless that behavior is explicitly promoted into the normative language specification.

---

75. Toolchain Freeze

The freeze system must remain compatible with the declared toolchain.

Minimum:

Rust 1.97+
Rust 2021
ANTLR-compatible grammar generation
antlr-rust 0.3.0-beta compatibility where retained by the repository

Dependency upgrades must not silently alter language semantics.

Toolchain changes that alter generated parser behavior require conformance revalidation.

---

76. Unsafe-Code Freeze

The production grammar system MUST NOT depend on "unsafe".

This includes:

grammar tooling
validators
generators
parser integration
AST integration
test infrastructure
supporting Rust implementation

A dependency that introduces required unsafe behavior is a release/freeze blocker unless the dependency can be replaced or safely isolated without violating the project's no-unsafe requirement.

---

77. Repository Integration Rule

Every grammar stage must declare integration with:

lexer
parser
AST
semantic analysis
IR
compiler
runtime
diagnostics
tests
documentation
compatibility
tooling

A stage cannot be marked complete merely because its ".g4" files compile.

---

78. Freeze Metadata

"STATUS.yaml" MUST represent the current status.

"OWNERSHIP.yaml" MUST represent ownership.

"DEPENDENCIES.yaml" MUST represent dependencies.

"MANIFEST.yaml" MUST represent inventory.

"FREEZE.md" defines the rules.

No status may be inferred solely from:

- file existence;
- directory name;
- parser generation;
- number of tests;
- implementation percentage.

---

79. Freeze Transaction

A stage freeze is performed as one logical transaction.

The sequence is:

1. inventory
2. normalize paths
3. establish ownership
4. establish dependencies
5. establish exports
6. establish AST mappings
7. establish semantic mappings
8. establish IR mappings
9. establish diagnostics
10. establish compatibility
11. establish tests
12. run validation
13. run hard-coding audit
14. run scalability audit
15. run POCO-REAF audit
16. run Rust/toolchain validation
17. verify dependency closure
18. verify no cycles
19. verify no unresolved ownership
20. verify no unresolved imports
21. mark FREEZE_READY
22. freeze
23. record freeze version

A failed gate prevents the freeze.

---

80. Freeze Commit Requirements

A freeze commit must include the complete contract changes required for the stage.

At minimum:

grammar source
ownership metadata
dependency metadata
status metadata
AST mappings
semantic mappings
IR mappings
diagnostics
compatibility
tests
documentation

A stage MUST NOT be declared frozen while required contract changes remain in later commits.

---

81. Freeze Certificate

Each frozen stage SHOULD have a machine-verifiable record containing:

stage
version
timestamp
language_version
grammar_version
dependency_graph_version
ownership_version
status_version
ast_version
semantic_version
ir_version
toolchain_version
rust_version
unsafe_policy
test_result
hardcoding_audit
ambiguity_audit
scalability_audit
poco_reaf_audit
freeze_result

The certificate records what was frozen, not merely that a directory was edited.

---

82. Freeze Immutability

Once frozen:

Allowed without semantic unfreeze

- spelling corrections that do not alter tokens;
- comments;
- formatting;
- documentation clarification;
- generated artifact regeneration with identical authoritative semantics;
- tooling improvements that preserve contracts;
- test harness improvements;
- CI improvements.

Requires revalidation

- grammar alternative changes;
- token changes;
- precedence changes;
- AST shape changes;
- semantic changes;
- effect changes;
- resource semantics changes;
- capability changes;
- policy changes;
- IR changes;
- compatibility changes.

Requires explicit versioning when breaking

Any change that invalidates an existing consumer.

---

83. No Downstream Re-edit Principle

The following must be true:

Frozen Stage N
      |
      +----> Stage N+1
      |
      +----> Stage N+2
      |
      +----> future domain
      |
      +----> future hardware

must not imply:

Stage N
   |
   +--> edit Stage N because Stage N+1 was added

Instead:

Stage N
   |
   +--> stable contract
          |
          +--> Stage N+1
          +--> Stage N+2
          +--> future extensions

If this property cannot be achieved, the abstraction boundary is not ready to freeze.

---

84. New Hardware Must Not Break a Frozen Grammar

A new:

CPU
GPU
FPGA
ASIC
QPU
accelerator
simulator
HPC system
cluster
cloud environment
future computational device

must integrate through:

capabilities
resources
topology
lowering
routing
scheduling
HAL
runtime

and must not require changing universal source syntax solely to describe the device.

---

85. New Scale Must Not Break a Frozen Grammar

The same source semantics must remain valid as scale changes.

The architecture must support:

tiny
small
medium
large
very large
distributed
massively parallel
future

without introducing language-wide capacity constants.

Scale is a realization property.

---

86. "Atom to Everywhere" Freeze Requirement

A source construct is freeze-ready only if its semantic meaning does not depend on an assumed machine scale.

For example:

computation

must remain meaningful whether realized as:

one operation
many operations
one processor
many processors
one accelerator
many accelerators
one node
many nodes

The grammar describes the computation.

The compiler/runtime determines realization.

---

87. Resource Availability Principle

Resource availability is an environmental input.

The grammar may declare:

required resources
acceptable constraints
preferred resources
allowed fallbacks
forbidden realizations

The grammar must not declare:

the maximum resources the universe permits

The latter is outside the language.

---

88. Capability Negotiation Boundary

Capability negotiation occurs after source semantics have been established.

The conceptual pipeline is:

discover
   |
   v
match requirements
   |
   v
apply constraints
   |
   v
apply policies
   |
   v
rank preferences
   |
   v
select realization
   |
   v
lower
   |
   v
route
   |
   v
schedule
   |
   v
execute

None of these physical decisions should be embedded as mandatory behavior in the universal grammar.

---

89. Fallback Freeze

Fallback must be explicit.

A program may permit:

allow fallback

but the fallback must specify semantic policy.

Possible outcomes include:

accept
degraded accept
retry
recover
escalate
reject

An implementation must not silently replace one semantic model with an approximation unless the language contract permits it.

---

90. Resilience Freeze

Resilience semantics must remain abstract.

Recognized states may include:

Unknown
Healthy
Degraded
Unstable
Unavailable
Recovering
Quarantined
Retired

These are semantic/runtime states.

They are not hardware inventory limits.

---

91. Provenance Freeze

Every adaptive, reflective, generated, learned, foreign, or externally influenced operation requiring provenance MUST define:

origin
version
source
authorization
policy
inputs
dependencies
transformation
result
timestamp/context where applicable
reproducibility metadata where applicable

Provenance must survive lowering where the semantic contract requires it.

---

92. Determinism and Reproducibility

The freeze process must distinguish:

deterministic semantics

from:

deterministic physical execution

A program may be semantically deterministic while physical execution varies.

Where reproducibility is requested, the language/compiler contract must identify relevant:

seed
version
dialect
resource constraints
optimization profile
environment
provenance

without hard-coding a machine.

---

93. Validation Firewall

"validation/" is a consumer of grammar contracts.

Production grammar must not import validator implementation code.

The relationship is:

grammar + contracts
        |
        v
validation
        |
        v
pass/fail

not:

grammar
   <-->
validation implementation

which could create architectural cycles.

---

94. Tests Are Consumers

The following MUST NOT be production grammar dependencies:

tests/
conformance/
examples/
golden/

They consume the grammar.

The grammar must never import test definitions.

---

95. Specification Is Not an ANTLR Dependency

"specification/" and "spec/" define normative contracts.

They are not imported as ANTLR grammars.

The relationship is validated by tooling.

This prevents a circular architecture in which the language grammar must import its own specification implementation.

---

96. Reference Material Is Non-Normative

"reference/" may contain:

- explanatory material;
- examples;
- historical designs;
- implementation notes.

It must not silently redefine language semantics.

Any conflict with the normative specification must be resolved in favor of the normative authority.

---

97. Quarantine Rule

A file MUST be quarantined if it has:

- unresolved ownership;
- unresolved dependency;
- malformed path;
- duplicate semantic ownership;
- unresolved ambiguity;
- direct vendor coupling in universal grammar;
- hard-coded capacity;
- missing required semantic mapping;
- missing compatibility contract;
- unsafe implementation requirement;
- unexplained historical status.

Quarantine is preferable to silently integrating uncertain material.

---

98. Experimental Features

Experimental features may exist without being frozen.

They MUST be clearly marked:

EXPERIMENTAL

Experimental features:

- must not redefine stable semantics;
- must not create hidden dependencies;
- must not become implicit core syntax;
- must have an extension boundary;
- must have an exit strategy.

---

99. Deprecated Features

A deprecated feature remains parseable only according to its compatibility contract.

Deprecation requires:

replacement
version
diagnostic
migration guidance
removal policy

Deprecated features must not become new dependencies for stable grammar.

---

100. Historical Features

Historical material must remain outside the active production dependency graph.

It may document:

- previous syntax;
- rejected designs;
- previous architecture;
- migration history.

Historical material does not define current semantics.

---

101. Production Freeze Stages

The recommended freeze order is:

FREEZE 0
Inventory and architecture

FREEZE 1
Authority and specification

FREEZE 2
Lexical foundation

FREEZE 3
Core names/modules/declarations

FREEZE 4
Type system

FREEZE 5
Expressions

FREEZE 6
Statements/functions

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
Grammar directory freeze

---

102. FREEZE 0 — Inventory

No semantic feature work should be considered frozen before inventory.

Required:

[ ] complete file inventory
[ ] canonical paths
[ ] file classification
[ ] ownership candidates
[ ] dependency candidates
[ ] duplicate detection
[ ] malformed path detection
[ ] orphan detection
[ ] historical detection
[ ] generated-file detection
[ ] status classification

"MANIFEST.yaml" is the inventory authority.

---

103. FREEZE 1 — Authority

Complete and validate:

DESIGN.md
specification/
spec/
CONFORMANCE.md
FREEZE.md

Required:

[ ] authority hierarchy fixed
[ ] scope fixed
[ ] language principles fixed
[ ] portability model fixed
[ ] scalability model fixed
[ ] POCO-REAF model fixed
[ ] compatibility model fixed
[ ] extension model fixed

No later stage may redefine these fundamentals silently.

---

104. FREEZE 2 — Lexer

Freeze:

lexer/tokens.g4
lexer/
antlr/ZamaniLexer.g4

only after lexical closure.

---

105. FREEZE 3 — Core

Freeze:

core/
modules/
declarations/

only after the lexical layer is frozen.

---

106. FREEZE 4 — Types

Freeze:

types/

only after core contracts are frozen.

---

107. FREEZE 5 — Expressions

Freeze:

expressions/

only after core and types are frozen.

---

108. FREEZE 6 — Statements and Functions

Freeze:

statements/
functions/
memory/

when their dependencies are stable.

---

109. FREEZE 7 — Semantic Control Plane

Freeze:

effects/
resources/

including capabilities and resource negotiation contracts.

---

110. FREEZE 8 — Governance and Security

Freeze:

policies/
security/

including contracts, authorization, sandboxing, provenance, and policy interaction.

---

111. FREEZE 9 — Classical and Data

Freeze:

classical/
data/

against the common semantic infrastructure.

---

112. FREEZE 10 — Concurrency and Distributed Systems

Freeze:

concurrency/
distributed/
networking/

with abstract execution and topology semantics.

---

113. FREEZE 11 — Hardware and Compilation

Freeze:

hardware/
execution/
compile/

after resource/capability contracts are stable.

---

114. FREEZE 12 — Quantum

Freeze:

quantum/

only after all common dependencies are frozen.

---

115. FREEZE 13 — HDL

Freeze:

hdl/

after hardware and execution contracts are frozen.

---

116. FREEZE 14 — Hybrid

Freeze:

hybrid/

only after the constituent domains are stable.

---

117. FREEZE 15 — AI

Freeze:

ai/

against stable common semantic infrastructure.

---

118. FREEZE 16 — Interoperability and Dialects

Freeze:

interoperability/
dialects/

only after core semantics are stable.

---

119. FREEZE 17 — Metaprogramming

Freeze:

macros/
metaprogramming/

only after AST and semantic contracts are stable.

---

120. FREEZE 18 — Compatibility

Freeze:

compatibility/

against the completed language contract.

---

121. FREEZE 19 — Root Composition

Only now freeze:

antlr/ZamaniLexer.g4
antlr/ZamaniParser.g4
Zamani.g4

where final root composition is mechanically verified.

---

122. Final Grammar Freeze

The entire "grammar/" directory may be declared frozen only when:

[ ] authority frozen
[ ] inventory complete
[ ] ownership complete
[ ] dependency DAG complete
[ ] all imports resolve
[ ] all exports have owners
[ ] no semantic dependency cycles
[ ] lexer frozen
[ ] parser frozen
[ ] AST contract frozen
[ ] semantic contract frozen
[ ] canonical IR contracts frozen
[ ] effects frozen
[ ] resources frozen
[ ] capabilities frozen
[ ] contracts frozen
[ ] policies frozen
[ ] provenance frozen
[ ] classical frozen
[ ] data frozen
[ ] concurrency frozen
[ ] distributed frozen
[ ] networking frozen
[ ] hardware frozen
[ ] execution frozen
[ ] compile frozen
[ ] quantum frozen
[ ] HDL frozen
[ ] hybrid frozen
[ ] AI frozen
[ ] dialects frozen
[ ] interoperability frozen
[ ] macros frozen
[ ] metaprogramming frozen
[ ] compatibility frozen
[ ] diagnostics complete
[ ] hard-coding audit passes
[ ] scalability audit passes
[ ] POCO-REAF audit passes
[ ] ambiguity audit passes
[ ] path hygiene passes
[ ] Rust 1.97+ validation passes
[ ] unsafe audit passes
[ ] conformance suite passes
[ ] negative suite passes
[ ] fuzz/robustness suite passes
[ ] portability suite passes
[ ] scalability suite passes
[ ] generated lexer succeeds
[ ] generated parser succeeds
[ ] Rust frontend integration succeeds
[ ] no downstream re-edit requirement remains

---

123. Final Freeze Invariant

Once "grammar/" is frozen:

«The fundamental Zamani source-language semantics are versioned, target-independent, dependency-closed, contract-complete, and stable enough that new hardware, new scale, new implementations, new runtimes, new compiler backends, new accelerators, new QPUs, new distributed environments, and new dialects can be integrated without modifying frozen language semantics merely to accommodate the new realization.»

This does not prohibit future language evolution.

Future semantic evolution occurs through:

new version
+
explicit compatibility contract
+
migration
+
revalidation

rather than silent mutation of a frozen language stage.

---

124. Final POCO-REAF Invariant

The complete architecture must preserve:

Zamani Source
      |
      v
Portable Meaning
      |
      +-- Types
      +-- Effects
      +-- Capabilities
      +-- Resources
      +-- Contracts
      +-- Policies
      +-- Provenance
      |
      v
Domain-Neutral AST
      |
      v
Semantic Model
      |
      +--------------------+
      |                    |
      v                    v
Classical IR         quantum::ir
      |                    |
      +---------+----------+
                |
                v
       Target-Independent
          Optimization
                |
                v
       Capability/Resource
          Negotiation
                |
                v
             Lowering
                |
                v
             Routing
                |
                v
           Scheduling
                |
                v
              ZQN
                |
                v
              HAL
                |
       +--------+---------+
       |        |         |
      CPU      GPU      FPGA
       |        |         |
      ASIC   Accelerator QPU
       |        |         |
       +--------+---------+
                |
                v
       HPC / Cluster /
       Distributed / Future

The source program expresses meaning.

The environment supplies resources and capabilities.

The compiler determines realization.

The runtime executes that realization.

No finite machine capacity becomes a permanent language limit.

---

125. Final Engineering Rule

The following rule is mandatory for all future grammar work:

«No new production grammar feature may be accepted unless its ownership, dependency, AST mapping, semantic mapping, IR mapping, diagnostics, compatibility, scalability, POCO-REAF behavior, hard-coding audit, tests, and integration contract are defined before implementation is considered complete.»

And:

«No stage may be frozen until every dependency required by that stage is itself frozen or explicitly covered by a stable external contract.»

And:

«No downstream addition may require semantic modification of an already-frozen upstream contract unless the language is intentionally versioned.»

This is the mechanism that allows Zamani to be built one stage at a time, freeze each completed stage, and continue expanding the language without turning the existing grammar into a permanently moving target.
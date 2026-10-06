Zamani Dialects

Path: "grammar/dialects/README.md"
Repository: "Benwellonedge28/Zamani"
Language: Zamani
Grammar technology: ANTLR4-compatible grammars
Rust baseline: Rust 1.97 or later
Rust edition: Rust 2021
Rust implementation safety: Safe Rust only; production Rust code MUST NOT use "unsafe"
Architecture: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)
Status: Normative dialect-subsystem architecture and integration contract

---

1. Purpose

"grammar/dialects/" defines how Zamani can support controlled language extensions without creating competing languages, competing ASTs, competing semantic models, or competing permanent IRs.

A dialect is an explicitly identified, versioned, namespaced extension contract of Zamani.

A dialect MAY provide:

- additional syntax;
- additional annotations;
- domain-specific declarations;
- domain-specific expressions;
- domain-specific statements;
- domain-specific metadata;
- domain-specific semantic constraints;
- interoperability syntax;
- vendor extensions;
- experimental facilities;
- compatibility adapters;
- source-format adapters.

A dialect MUST NOT silently redefine the meaning of Zamani core constructs.

A dialect MUST NOT become a separate programming language merely because it introduces additional syntax.

The fundamental model is:

                         ZAMANI
                            |
              +-------------+-------------+
              |             |             |
           core          domains       dialects
              |             |             |
              +-------------+-------------+
                            |
                    common semantics
                            |
          +-----------------+------------------+
          |                 |                  |
        types             effects          resources
          |                 |                  |
          +-----------------+------------------+
                            |
                    capabilities
                            |
                       contracts
                            |
                        policies
                            |
                      provenance
                            |
                 canonical semantic model
                            |
              +-------------+-------------+
              |                           |
        classical IR                 quantum::ir
              |                           |
              +-------------+-------------+
                            |
                target-independent work
                            |
          lowering / optimization / routing
                            |
                      scheduling
                            |
                 resilience / recovery
                            |
                           ZQN
                            |
                           HAL
                            |
       +----------+----------+----------+----------+
       |          |          |          |          |
      CPU        GPU        FPGA       ASIC       QPU
       |          |          |          |          |
       +----------+----------+----------+----------+
                            |
                  embedded / HPC / cluster /
                  distributed / cloud / future

The dialect subsystem is therefore an extension mechanism, not a second compiler architecture.

---

2. Normative terminology

The following terms are normative.

MUST

The requirement is mandatory.

MUST NOT

The behavior is prohibited.

SHOULD

The behavior is strongly recommended. Deviations require justification.

MAY

The behavior is permitted but optional.

Core

Language functionality defined by Zamani's universal language contract.

Dialect

A controlled extension of Zamani.

Dialect declaration

Source syntax identifying or configuring dialect participation.

Dialect definition

The complete specification of a dialect.

Dialect registration

Making a dialect discoverable to tooling/compiler infrastructure.

Dialect implementation

The parser, semantic, AST, lowering, interoperability and/or backend implementation associated with a dialect.

Dialect registry

The semantic/tooling catalogue used to resolve dialect identity and metadata.

External format

A language or data format whose syntax is accepted or translated through interoperability infrastructure.

Examples include:

- SQL;
- XML;
- JSON;
- OpenQASM;
- Verilog;
- VHDL;
- SystemVerilog;
- assembly;
- vendor configuration formats.

An external format is not automatically a Zamani dialect.

Vendor extension

A dialect or interoperability extension owned by a vendor or implementation provider.

Experimental dialect

A dialect whose contract is not yet stable.

Canonical semantic representation

The repository's target-independent semantic model into which dialect constructs are lowered.

Canonical IR

The repository's established IR boundary for a computational domain.

For quantum computation this is:

quantum::ir

---

3. Fundamental rule: one Zamani language

The following architecture is prohibited:

Zamani
├── Zamani language
├── Quantum language
├── AI language
├── HDL language
├── GPU language
├── Vendor language
└── SQL language

The required architecture is:

                         Zamani
                            |
             +--------------+--------------+
             |              |              |
          classical       quantum         HDL
             |              |              |
             +--------------+--------------+
                            |
                         hybrid
                            |
             +--------------+--------------+
             |              |              |
             AI           data       distributed
             |              |              |
             +--------------+--------------+
                            |
                    common semantics
                            |
                       dialects
                            |
                  canonical representations

Every dialect MUST share Zamani's universal foundations.

These include:

- source locations;
- identifiers;
- names;
- modules;
- imports;
- declarations;
- expressions;
- statements;
- types;
- functions;
- effects;
- capabilities;
- resources;
- requirements;
- constraints;
- contracts;
- policies;
- provenance;
- diagnostics;
- compatibility;
- versioning;
- semantic validation.

A dialect may add domain meaning, but it does not receive permission to create an incompatible universe.

---

4. Relationship to the rest of "grammar/"

The dialect subsystem is one part of the complete grammar architecture.

The ownership hierarchy is:

grammar/DESIGN.md
        |
        +--> grammar/specification/
        |
        +--> grammar/spec/
        |
        +--> grammar/lexer/
        |
        +--> grammar/Zamani.g4
        |
        +--> grammar/antlr/
        |
        +--> grammar/core/
        |
        +--> grammar/types/
        |
        +--> grammar/expressions/
        |
        +--> grammar/statements/
        |
        +--> grammar/declarations/
        |
        +--> grammar/resources/
        |
        +--> grammar/effects/
        |
        +--> grammar/validation/
        |
        +--> grammar/policies/
        |
        +--> grammar/dialects/        <-- this subsystem
        |
        +--> grammar/interoperability/
        |
        +--> grammar/quantum/
        |
        +--> grammar/classical/
        |
        +--> grammar/hdl/
        |
        +--> grammar/hardware/
        |
        +--> grammar/hybrid/
        |
        +--> grammar/ai/
        |
        +--> grammar/data/
        |
        +--> grammar/concurrency/
        |
        +--> grammar/distributed/
        |
        +--> grammar/networking/
        |
        +--> grammar/security/
        |
        +--> grammar/metaprogramming/
        |
        +--> grammar/execution/
        |
        +--> grammar/compile/
        |
        +--> grammar/tests/
        |
        v
Rust frontend
        |
        v
AST
        |
        v
semantic model
        |
        v
canonical IR

The dialect subsystem MUST respect the ownership contracts of those directories.

---

5. Existing dialect files and ownership

The current dialect directory contains the following major components:

grammar/dialects/
├── README.md
├── capabilities.g4
├── compatibility.g4
├── declaration.g4
├── dialect.g4
├── dialects.g4
├── experimental.g4
├── exports.g4
├── extension-points.g4
├── imports.g4
├── namespaces.g4
├── registration.g4
├── registry.md
├── sql.g4
├── vendor.g4
├── versioning.g4
└── xml.g4

Their responsibilities MUST remain separated.

File| Responsibility
"README.md"| Architecture, ownership, integration, lifecycle and production contract
"dialect.g4"| Public dialect syntax boundary
"dialects.g4"| Composition of dialect constructs
"declaration.g4"| Dialect declaration context
"registration.g4"| Registration metadata/context
"namespaces.g4"| Dialect namespace structure
"imports.g4"| Dialect import/use relationships
"exports.g4"| Dialect export visibility
"versioning.g4"| Dialect-specific use of canonical version syntax
"compatibility.g4"| Compatibility declarations and relationships
"capabilities.g4"| Dialect capability declarations/adapters
"extension-points.g4"| Extension attachment points
"vendor.g4"| Vendor extension context
"experimental.g4"| Experimental extension context
"registry.md"| Registry architecture and metadata contract
"sql.g4"| SQL interoperability/dialect syntax
"xml.g4"| XML interoperability/dialect syntax

No file may silently acquire another file's ownership.

---

6. Critical correction: dialects versus interoperability formats

Not every external language or format should be treated as a Zamani dialect.

There are three distinct categories.

6.1 Native Zamani dialect

A native dialect extends Zamani syntax and semantics.

Example:

quantum::extended

It produces Zamani AST and semantic constructs directly.

---

6.2 Source-language interoperability dialect

An external programming language can be accepted through a dialect adapter.

Example:

openqasm

The external source is translated into Zamani's canonical semantic model.

The external language does not become Zamani's universal syntax.

---

6.3 Data/interchange format

A format such as:

JSON
XML
CSV
SQL

is generally an interoperability format rather than a core Zamani dialect.

Its parser belongs under:

grammar/interoperability/

or an explicitly registered dialect adapter where appropriate.

The architectural path is:

external format
       |
       v
format parser
       |
       v
interoperability model
       |
       v
Zamani semantic model
       |
       v
canonical IR

This prevents the root language from becoming a catalogue of every external syntax.

---

7. Root grammar boundary

"grammar/Zamani.g4" remains the canonical root ANTLR composition boundary.

The dialect subsystem MUST NOT require "Zamani.g4" to enumerate every dialect.

The root grammar MUST NOT contain a finite list such as:

dialect
    : QUANTUM
    | OPENQASM
    | CUDA
    | SQL
    | XML
    | VENDOR_X
    ;

That design is not scalable.

Instead, the root grammar consumes the generic dialect composition.

Conceptually:

Zamani.g4
    |
    v
ZamaniParser
    |
    +--> core
    +--> types
    +--> expressions
    +--> statements
    +--> declarations
    +--> resources
    +--> effects
    +--> validation
    +--> policies
    +--> dialects
    +--> interoperability
    +--> domains

A newly registered dialect MUST NOT require modifying the root grammar merely to add its name.

---

8. Open-world dialect identity

Dialect identities MUST be open-world.

A dialect identity is data, not a universal enumeration.

Examples:

quantum::standard
quantum::dynamic
quantum::openqasm
classical::numeric
classical::scientific
hdl::rtl
hardware::fpga
ai::tensor
data::stream
distributed::messaging
vendor::example::quantum
organization::research::extension
future::computing::dialect

These are structural examples.

They are NOT built-in reserved dialect names.

Adding:

future::new_domain::new_dialect

must not require modifying the universal grammar.

Whether the dialect exists, is trusted, is available, is compatible, or is usable is determined by semantic/registry infrastructure.

---

9. Dialect identity is not hardware identity

A dialect identity MUST NOT be equivalent to:

- CPU model;
- GPU model;
- FPGA part number;
- ASIC identifier;
- QPU identifier;
- physical qubit identifier;
- memory-bank identifier;
- machine address;
- deployment location;
- physical node identifier.

For example:

quantum::standard

means a source-level language contract.

It does NOT mean:

QPU-17

or:

127 physical qubits

Physical realization belongs downstream.

---

10. Dialect declaration

A dialect declaration identifies participation in a dialect.

Conceptually:

dialect quantum::standard;

or a richer form:

dialect quantum::standard
    version >= 1.0;

The exact syntax is owned by:

grammar/dialects/dialect.g4
grammar/dialects/declaration.g4
grammar/dialects/versioning.g4

The declaration MUST NOT itself:

- discover hardware;
- select a physical QPU;
- allocate memory;
- select a CPU;
- perform routing;
- schedule operations;
- perform optimization;
- execute code.

---

11. Dialect registration

Registration and declaration are different concepts.

Declaration

A source program says:

«I use this dialect contract.»

Registration

Tooling/compiler infrastructure says:

«This dialect contract exists and can be resolved.»

Registration MAY contain:

- identity;
- namespace;
- version;
- owner;
- status;
- specification location;
- implementation location;
- feature metadata;
- compatibility metadata;
- capability metadata;
- conformance status;
- provenance;
- migration information.

Registration MUST NOT redefine source semantics.

---

12. Registry is not grammar authority

"grammar/dialects/registry.md" describes registry architecture.

The registry MUST NOT become a second language specification.

The distinction is:

grammar/specification/
        |
        | defines meaning
        v
dialect contract
        |
        | registered as metadata
        v
registry
        |
        | resolved by tooling
        v
implementation

The registry can say:

dialect exists
version = ...
status = stable
implementation = ...

It cannot silently change:

what the source program means

---

13. Dialect lifecycle

Every dialect MUST have an explicit lifecycle status.

Allowed statuses are:

proposed
experimental
stable
deprecated
historical
rejected
removed

Status MUST be explicit.

Parsing a dialect does not make it stable.

A dialect is production-ready only after completing its full contract.

Required lifecycle:

proposal
   |
   v
specification
   |
   v
identity
   |
   v
grammar
   |
   v
AST contract
   |
   v
semantic contract
   |
   v
type/effect/resource integration
   |
   v
capability integration
   |
   v
canonical IR integration
   |
   v
lowering
   |
   v
implementation
   |
   v
positive tests
   |
   v
negative tests
   |
   v
boundary tests
   |
   v
scalability tests
   |
   v
compatibility tests
   |
   v
determinism tests
   |
   v
diagnostic tests
   |
   v
hard-coding audit
   |
   v
production acceptance

No automatic promotion is permitted.

---

14. Stable does not mean "parser accepts it"

A dialect is NOT stable merely because:

ANTLR parses it

Production stability requires:

syntax
+
AST
+
semantics
+
types
+
effects
+
resources
+
capabilities
+
contracts
+
policies
+
provenance
+
IR
+
lowering
+
compatibility
+
tests

where applicable.

---

15. Syntax ownership

A dialect may introduce syntax only through an explicit extension point.

The dialect MUST identify:

extension identity
extension point
syntax
AST representation
semantic representation
compatibility behavior
diagnostics

A dialect MUST NOT silently redefine:

if
match
fn
type
struct
module
import
requires
ensures
capability

or any other core construct.

If core syntax is intentionally extensible, the extension mechanism must be explicitly defined by the relevant core grammar/specification.

---

16. Metadata-only dialect extensions

Not every dialect extension needs new semantics.

A dialect may attach metadata to an existing construct.

For example:

@dialect("organization::research::extension")
fn compute(...) { ... }

Such an extension can remain metadata-only if that is its defined contract.

The dialect MUST state whether its extension is:

syntax-only
metadata-only
semantic
type-system
effect-system
resource-system
capability-system
contract-system
policy-system
lowering
interoperability

This classification prevents accidental coupling.

---

17. AST contract

Every dialect construct that survives parsing MUST have an explicit AST contract.

The required pipeline is:

dialect syntax
      |
      v
ANTLR parse tree
      |
      v
domain-neutral frontend AST
      |
      v
dialect semantic model

The AST contract MUST identify:

- grammar rule;
- AST node;
- fields;
- child nodes;
- optional values;
- attributes;
- source span;
- identifiers;
- generic parameters;
- literal representation;
- semantic identity;
- diagnostic information.

A dialect MUST NOT create an unrelated permanent AST architecture unless explicitly authorized by the global AST design.

---

18. AST neutrality

The frontend AST must represent program meaning without prematurely embedding physical realization.

For example, a dialect AST should not encode:

physical_qubit = 37

when the source expresses only:

logical qubit

Similarly it should not encode:

run_on_gpu_7

when the program expresses only:

requires capability("gpu.compute")

Physical realization belongs downstream.

---

19. Semantic contract

Parsing establishes structure.

Semantic analysis establishes meaning.

Therefore:

parser accepts

does not mean:

dialect is semantically valid

Semantic analysis must determine:

- whether the dialect exists;
- whether it is visible;
- whether the namespace resolves;
- whether the requested version is valid;
- whether dependencies exist;
- whether dependencies are compatible;
- whether extensions are valid;
- whether capabilities are known;
- whether requirements are satisfiable;
- whether policies permit the construct;
- whether effects are permitted;
- whether the construct has a canonical lowering;
- whether conflicting dialects exist;
- whether the dialect is enabled.

These decisions MUST NOT be implemented as arbitrary parser hacks.

---

20. Version ownership

"grammar/dialects/versioning.g4" is an adapter to the canonical version model.

The universal version grammar remains owned by the core versioning subsystem.

The dialect version grammar MUST NOT create another independent version language.

The architecture is:

grammar/core/versioning.g4
             |
             v
canonical version model
             |
             v
grammar/dialects/versioning.g4
             |
             v
dialect version context

Dialect versioning MUST NOT encode:

- CPU model;
- GPU model;
- FPGA model;
- ASIC model;
- QPU model;
- physical qubit count;
- memory capacity;
- node count;
- topology;
- calibration;
- deployment location.

Those belong to resource/capability/target infrastructure.

---

21. Compatibility

Dialect compatibility is a semantic relationship.

The grammar records its structure.

Semantic infrastructure determines:

compatible
incompatible
requires migration
deprecated
unsupported

Compatibility MUST account for:

- dialect version;
- Zamani language version;
- AST compatibility;
- semantic compatibility;
- IR compatibility;
- dependency compatibility;
- feature availability;
- migration rules.

Compatibility MUST NOT be inferred solely from a parser accepting both constructs.

---

22. Imports

Dialect imports belong to:

grammar/dialects/imports.g4

A dialect import identifies a dialect contract.

It does not directly load arbitrary executable code.

Conceptually:

import dialect quantum::standard;

or:

use dialect quantum::standard version >= 1.0;

The exact surface syntax remains owned by the grammar.

Resolution occurs later.

The parser MUST NOT access:

- filesystem state;
- network state;
- package servers;
- hardware;
- credentials.

---

23. Exports

Dialect exports belong to:

grammar/dialects/exports.g4

Exports control what a dialect makes visible to consuming modules.

Export semantics MUST integrate with:

modules
namespaces
visibility
compatibility
package resolution

Exports MUST NOT bypass:

- security;
- capability checks;
- semantic validation;
- version compatibility.

---

24. Namespaces

"grammar/dialects/namespaces.g4" owns dialect namespace syntax.

Namespaces MUST be:

- hierarchical;
- symbolic;
- extensible;
- deterministic;
- independent of physical topology.

A namespace such as:

vendor::domain::extension

does not imply any specific hardware.

Namespace depth MUST NOT be artificially capped.

There MUST be no:

MAX_NAMESPACE_DEPTH

or equivalent parser restriction.

---

25. Capability integration

"grammar/dialects/capabilities.g4" MUST remain an adapter to the universal capability model.

A dialect may require:

requires capability("quantum.measurement");

or:

requires capability("tensor.compute");

or:

requires capability("hardware.synthesis");

Capability identifiers are symbolic.

They are not physical device identifiers.

The semantic path is:

dialect
   |
   v
capability requirement
   |
   v
capability analysis
   |
   v
target capability inventory
   |
   v
realization

The grammar MUST NOT decide whether a target possesses the capability.

---

26. Resource integration

Dialect resource requirements MUST use the repository's universal resource model.

Examples:

requires memory >= required_memory;
requires qubits >= required_qubits;
requires capability("quantum.measurement");
requires topology(required_topology);

These express intent.

They do not hard-code physical limits.

The grammar MUST NOT contain:

MAX_QUBITS
MAX_MEMORY
MAX_NODES
MAX_GPUS
MAX_THREADS

or hidden equivalents.

---

27. Requirements versus capabilities

These concepts must remain distinct.

Requirement

Something the program needs.

requires capability("quantum.measurement");

Capability

Something the target can provide.

quantum.measurement

Constraint

Something that must remain true.

requires latency <= budget;

Preference

A preferred realization.

prefer accelerator;

Hint

Optimization guidance.

hint locality;

Realization

The concrete target decision.

logical operation
    ->
selected physical resource

Dialect syntax MUST NOT collapse these concepts into one mechanism.

---

28. Effects

Dialect constructs that introduce effects MUST integrate with:

grammar/effects/

Potential effects include:

io
network
native
foreign
mutation
randomness
measurement
quantum
learning
adaptation
reflection
code_generation
simulation
distributed

A dialect MUST declare the relevant effect contract.

The dialect parser does not execute the effect.

---

29. Contracts

Dialect constructs may participate in:

requires
ensures
invariant
assume
guarantee
property
assert

The contract model belongs to the universal validation/contract architecture.

A dialect MUST NOT create a separate contract language.

The relationship is:

dialect construct
       |
       v
universal contract model
       |
       +--> semantic validation
       +--> verification
       +--> diagnostics
       +--> optimization
       +--> runtime checking where applicable

---

30. Policies

Dialect behavior can be restricted by universal policies.

Policies may govern:

- allowed capabilities;
- forbidden effects;
- resource use;
- adaptation;
- reflection;
- foreign calls;
- network operations;
- deployment;
- simulation;
- vendor extensions;
- experimental features.

The policy mechanism is shared.

A dialect MUST NOT invent an incompatible security-policy system.

---

31. Provenance

Dialect transformations MUST preserve provenance where required.

Provenance may include:

source
derived_from
dialect
dialect_version
transformed_by
reason
evidence
verification
compiler_version
semantic_version
IR_version

The dialect subsystem MUST integrate with the repository-wide provenance model.

A dialect parser should preserve enough source information for downstream provenance and diagnostics.

---

32. Determinism

Dialect parsing MUST be deterministic.

Parsing MUST depend only on:

- source text;
- selected grammar version;
- lexical rules;
- parser rules;
- explicitly supplied dialect configuration.

Parsing MUST NOT depend on:

- wall-clock time;
- randomness;
- hardware;
- filesystem state;
- network state;
- environment variables;
- runtime state;
- target availability.

Two identical inputs under identical grammar/configuration MUST produce equivalent parse results.

---

33. No parser-side execution

Dialect grammars MUST NOT perform:

- filesystem access;
- network access;
- hardware discovery;
- package installation;
- compiler execution;
- runtime execution;
- secret access;
- random selection;
- target selection.

ANTLR grammar files are syntax specifications.

Execution belongs to compiler/runtime infrastructure.

---

34. Quantum dialects

Quantum dialects must preserve the generic quantum operation model.

The dialect grammar MUST NOT enumerate every quantum operation.

This is prohibited:

quantumOperation
    : H
    | X
    | Y
    | Z
    | CNOT
    | SWAP
    | ...
    ;

The scalable model is:

operation identity
parameters
operands
results
attributes
modifiers
effects
capabilities
requirements
source

Therefore future operations can be introduced without changing the universal grammar.

The semantic pipeline is:

dialect quantum operation
          |
          v
generic quantum operation
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
QEC / resilience
          |
          v
ZQN
          |
          v
HAL
          |
          v
target

The dialect grammar MUST NOT own:

- physical qubit mapping;
- coupling maps;
- calibration;
- QPU topology;
- QEC implementation;
- scheduling;
- routing.

---

35. OpenQASM and similar external quantum formats

An external quantum format MUST be treated as an interoperability/source adapter unless explicitly standardized as a native Zamani dialect.

The architecture is:

external quantum source
        |
        v
external-format parser
        |
        v
format semantic model
        |
        v
Zamani quantum semantic model
        |
        v
quantum::ir

The external format MUST NOT become the universal Zamani grammar.

This allows Zamani to consume current and future quantum languages without coupling the core grammar to one external syntax.

---

36. HDL dialects

HDL-related dialects may express:

- hardware intent;
- signals;
- interfaces;
- parameterized structures;
- state machines;
- timing intent;
- pipelines;
- memories;
- verification;
- synthesis intent.

They MUST integrate with:

grammar/hdl/
grammar/hardware/
grammar/validation/
grammar/resources/
grammar/capabilities/

A dialect MUST NOT impose artificial universal hardware dimensions.

For example, a parameterized width should remain parameterized when the semantic intent is scalable.

---

37. Classical dialects

Classical dialects may express:

- numerical computation;
- scientific computation;
- symbolic computation;
- vector/tensor operations;
- domain-specific algorithms;
- optimization intent.

They MUST continue to use universal:

- types;
- effects;
- resources;
- capabilities;
- contracts;
- policies;
- provenance.

A classical dialect MUST NOT make a particular ISA or CPU topology part of universal semantics.

---

38. AI and data dialects

AI/data dialects may expose semantic concepts such as:

model
dataset
training
inference
tensor
stream
pipeline
agent
knowledge
reasoning
uncertainty

These concepts must integrate with the universal semantic model.

Framework-specific details remain external.

A dialect MUST NOT require a particular:

- GPU;
- accelerator;
- tensor-core generation;
- ML framework;
- vendor runtime.

The compiler may specialize later.

---

39. Distributed dialects

Distributed dialects may represent:

- services;
- actors;
- channels;
- messages;
- collectives;
- replication;
- partitioning;
- consistency;
- fault tolerance;
- placement intent.

They MUST integrate with:

grammar/concurrency/
grammar/distributed/
grammar/networking/
grammar/resources/
grammar/capabilities/
grammar/security/

There MUST be no artificial universal limits such as:

MAX_NODES
MAX_PROCESSES
MAX_SERVICES
MAX_CHANNELS

---

40. Vendor extensions

Vendor extensions belong to:

grammar/dialects/vendor.g4

A vendor extension MUST identify:

- vendor namespace;
- dialect identity;
- version;
- compatibility;
- capabilities;
- semantics;
- provenance;
- implementation status.

Vendor syntax MUST NOT silently enter Zamani core.

A vendor extension may expose specialized capabilities while preserving a portable source contract.

For example:

requires capability("vendor.domain.feature");

is preferable to embedding a physical device identity in the language.

---

41. Experimental extensions

Experimental syntax belongs to:

grammar/dialects/experimental.g4

Experimental extensions MUST:

- have explicit status;
- have explicit ownership;
- have version information;
- have diagnostics;
- have migration expectations;
- be distinguishable from stable syntax;
- be excluded from stability claims.

Experimental acceptance MUST NOT imply permanent language compatibility.

---

42. Dialect extension points

"extension-points.g4" defines where dialect functionality may attach.

Extension points should be structural rather than hard-coded lists.

Examples include:

declaration extension
expression extension
statement extension
type extension
attribute extension
module extension
operator extension
literal extension
annotation extension
resource extension
capability extension
contract extension
policy extension

An extension point MUST have a clear owner.

No extension point may permit arbitrary semantic replacement of the core language.

---

43. Dialect composition

Multiple dialects may participate in one program.

Composition must be explicit.

Conceptually:

program
 |
 +--> dialect A
 |
 +--> dialect B
 |
 +--> dialect C

The semantic system must determine:

- compatibility;
- conflicts;
- precedence;
- dependencies;
- namespace collisions;
- capability requirements;
- effect interactions;
- type interactions;
- lowering interactions.

The parser should establish structure.

It must not silently resolve semantic conflicts.

---

44. Dialect conflict resolution

If two dialects attempt incompatible extensions, the compiler must produce a deterministic diagnostic.

Examples of conflicts:

same syntax / different meaning
incompatible type interpretation
incompatible operator semantics
conflicting namespace ownership
incompatible version requirements
conflicting effect declarations
incompatible lowering

A conflict MUST NOT be resolved by arbitrary parser precedence.

Resolution rules must be specified.

---

45. Dialect precedence

Dialect precedence MUST NOT become a hidden global ordering.

Where precedence is needed, it must be explicitly defined by the relevant extension contract.

The system should prefer:

explicit qualification

over:

implicit global precedence

For example:

namespace::operation

is preferable to silently choosing one of multiple meanings.

---

46. Generic identifiers remain extensible

Domain-specific names should remain identifiers unless they genuinely require lexical reservation.

For example, names such as:

vendor_gate
future_accelerator
tensor_model
robot_controller
custom_protocol
quantum_algorithm

should normally remain identifiers.

The dialect subsystem MUST NOT turn every domain vocabulary item into a reserved keyword.

This keeps the language extensible and avoids lexical exhaustion.

---

47. Keyword discipline

A new dialect keyword is justified only when all of the following hold:

1. Ordinary identifiers are insufficient.
2. The syntax genuinely requires reserved lexical treatment.
3. The keyword has stable language-level meaning.
4. Its ownership is documented.
5. Compatibility consequences are documented.
6. Lexer integration exists.
7. Parser integration exists.
8. AST integration exists.
9. Semantic integration exists.
10. Conformance tests exist.

Application-specific vocabulary should normally remain library/API identifiers.

---

48. No application-specific keyword explosion

The dialect subsystem MUST NOT become a list of keywords for every possible application.

The language does not need universal keywords for:

computer vision
sentiment
robotics
payments
administration
legal workflows
VR
AR
blockchain
specific businesses
specific institutions
specific products
specific applications

Those concepts belong in:

libraries
modules
APIs
dialects
services
policies
data models
application code

The universal grammar should expose computational primitives.

---

49. Dialects and reasoning/learning/adaptation

Zamani's universal reasoning, knowledge, learning and adaptation facilities must not be duplicated inside each dialect.

The common semantic concepts are owned by the corresponding universal subsystem.

A dialect can consume them.

For example:

dialect-specific operation
       |
       v
reasoning semantic model
       |
       v
universal reasoning representation

Likewise:

dialect
   |
   +--> learning
   +--> uncertainty
   +--> evidence
   +--> explanation
   +--> provenance
   +--> adaptation

These remain shared language concepts.

---

50. Interoperability boundary

The dialect subsystem and interoperability subsystem must cooperate without duplicating responsibilities.

Use:

grammar/dialects/

for Zamani dialect identity and controlled extension.

Use:

grammar/interoperability/

for external language/data-format integration.

Examples:

SQL
XML
JSON
assembly
foreign ABI
external quantum formats
HDL formats

The general boundary is:

external syntax
      |
      v
interop parser
      |
      v
interop semantic model
      |
      v
Zamani semantic model

or, when appropriate:

external syntax
      |
      v
registered dialect adapter
      |
      v
Zamani AST

The choice must be documented per integration.

---

51. SQL

SQL must not become part of universal Zamani syntax.

The existing:

grammar/dialects/sql.g4

must be treated as an SQL integration boundary.

SQL semantics remain SQL semantics.

The integration path should be:

SQL source
   |
   v
SQL grammar
   |
   v
SQL semantic representation
   |
   v
Zamani data/query semantic model
   |
   v
execution/interop layer

The SQL integration MUST NOT impose database-specific limits on Zamani.

---

52. XML

The existing:

grammar/dialects/xml.g4

must remain an XML interoperability boundary.

XML syntax is not universal Zamani syntax.

The XML integration should preserve:

- namespaces;
- qualified names;
- attributes;
- elements;
- text;
- document structure;
- references;
- declarations;
- source locations where required.

External entity resolution, network access and security-sensitive resolution belong outside the grammar.

---

53. FFI and ABI

Foreign-function support belongs primarily under:

grammar/interoperability/

The dialect subsystem may identify a dialect context, but it must not duplicate:

ABI
calling convention
foreign types
linkage
data layout

FFI calls must integrate with:

effects
capabilities
security
provenance
type checking
ABI resolution

A foreign call may therefore require an effect such as:

foreign

and a capability such as:

native.call

depending on the universal specification.

---

54. Canonical IR rule

A dialect MUST NOT create a competing permanent IR merely because its syntax differs.

Required:

dialect syntax
      |
      v
Zamani AST
      |
      v
semantic model
      |
      v
canonical IR

For quantum:

dialect
   |
   v
quantum semantic model
   |
   v
quantum::ir

For classical:

dialect
   |
   v
classical semantic model
   |
   v
classical IR

For HDL/hardware:

dialect
   |
   v
hardware semantic model
   |
   v
HDL/hardware representation

Dialect-specific temporary representations MAY exist inside an implementation when justified, but they MUST NOT become competing language authorities.

---

55. Canonical operation model

Dialect operations should map to a generic operation model where possible.

A generic operation can conceptually contain:

name
namespace
operands
parameters
results
attributes
modifiers
effects
capabilities
requirements
source
provenance

This is particularly important for:

- quantum operations;
- accelerator operations;
- tensor operations;
- vendor operations;
- hardware operations;
- distributed operations.

The generic representation prevents future operation growth from requiring grammar redesign.

---

56. POCO-REAF

Dialects MUST preserve the POCO-REAF architecture.

A source program expresses:

meaning
requirements
constraints
capabilities
preferences
policies

The compiler determines realization.

Therefore:

dialect
   |
   v
portable semantics
   |
   v
requirements/capabilities
   |
   v
target negotiation
   |
   v
specialization
   |
   v
lowering
   |
   v
routing
   |
   v
scheduling
   |
   v
resilience
   |
   v
execution

The source should not need rewriting merely because:

machine A

is replaced by:

machine B

provided both satisfy the required semantic contract.

---

57. Dialects must not select physical hardware

The following is prohibited as universal dialect semantics:

use_gpu("device7")
use_qpu("qpu17")
use_cpu("core3")
use_fpga("board5")

when the intent is portable computation.

Instead:

requires capability("gpu.compute");

or:

prefer capability("accelerated.compute");

or an equivalent target-independent contract should be used.

Physical selection belongs to downstream target realization.

---

58. Scalability

The dialect architecture is open-ended.

There must be no language-level limit on:

- dialect count;
- extension count;
- namespace depth;
- dialect dependencies;
- version components;
- capabilities;
- requirements;
- imports;
- exports;
- operations;
- dialect members;
- semantic metadata;
- dialect combinations.

There must be no:

MAX_DIALECTS
MAX_EXTENSIONS
MAX_NAMESPACE_DEPTH
MAX_DIALECT_DEPENDENCIES
MAX_CAPABILITIES
MAX_REQUIREMENTS
MAX_IMPORTS
MAX_EXPORTS

or hidden equivalents.

Practical compiler limits may exist because of:

- available memory;
- process address space;
- implementation integer types;
- compiler budgets;
- operating-system limits.

Such limits must not be presented as language semantics.

---

59. "Infinity" and physical reality

POCO-REAF and scalable dialects mean:

«no artificial language ceiling.»

They do not mean:

«physical resources are infinite.»

A program may scale to the largest computation that its semantic requirements and available resources permit.

If a target cannot satisfy:

requires capability("quantum.measurement");

the compiler/runtime must report that fact or use an explicitly permitted fallback.

It must not silently change the program's meaning.

---

60. Dialect fallback

Fallbacks must be explicit.

A dialect may specify:

preferred implementation
fallback implementation

but fallback behavior MUST preserve semantic meaning.

For example:

quantum computation
       |
       +--> QPU
       |
       +--> quantum simulator
       |
       +--> explicitly permitted alternative

The fallback mechanism must be owned by execution/compilation semantics rather than hidden in parser behavior.

---

61. Dialect feature gates

Feature gating MAY be used for:

- experimental features;
- optional compiler functionality;
- compatibility transitions;
- implementation availability.

Feature gates MUST NOT be used to encode physical resource limits.

A gate such as:

feature("dynamic-circuits")

is conceptually different from:

maximum-qubits = 64

The former is a feature contract.

The latter is a physical/operational constraint and belongs elsewhere.

---

62. Dialect dependencies

Dialect dependencies must be explicit.

A dependency can identify:

dialect
version requirement
compatibility requirement
capability requirement
semantic dependency

Dependencies MUST be resolved semantically.

A dialect MUST NOT assume that another dialect is present merely because its name appears in source text.

---

63. Circular dependencies

Dialect dependency cycles must be detected deterministically.

The grammar may represent dependency declarations.

Semantic analysis determines whether the resulting dependency graph is valid.

The system must not recurse indefinitely because of a malformed dialect dependency graph.

Diagnostics must identify the dependency cycle.

---

64. Dialect isolation

A dialect should expose only its intended public surface.

Private implementation details must not automatically become language syntax.

The dialect contract should distinguish:

public
internal
experimental
deprecated

where applicable.

This is particularly important for vendor extensions.

---

65. Security

Dialect loading and resolution must not implicitly execute untrusted code.

The grammar itself must remain side-effect free.

Dialect metadata should be treated as untrusted until validated.

Security-sensitive decisions belong to:

grammar/security/
compiler security infrastructure
package/dependency infrastructure
runtime security

A dialect MUST NOT bypass security controls by declaring a capability.

Declaring:

requires capability("native.execute");

does not grant that capability.

It only states a requirement.

---

66. Capability declaration is not authorization

This distinction is mandatory.

requires capability("network");

means:

«this program requires networking capability.»

It does NOT mean:

«this program is authorized to use networking.»

Authorization belongs to security/policy infrastructure.

Therefore:

requirement
    !=
capability
    !=
authorization

---

67. Provenance of dialect transformations

Every meaningful transformation should be traceable where provenance is enabled.

Example:

source
  |
  v
dialect construct
  |
  | transformed by
  v
canonical semantic operation
  |
  | lowered by
  v
canonical IR

The provenance system may record:

source span
dialect identity
dialect version
transformation
compiler version
semantic version
IR version
reason
evidence

This is especially important for:

- AI;
- quantum;
- hardware;
- optimization;
- security;
- scientific computing.

---

68. Diagnostics

Every dialect must have deterministic diagnostics.

Diagnostics should identify:

- dialect;
- dialect version;
- source span;
- offending construct;
- expected contract;
- actual condition;
- related declaration;
- dependency;
- capability;
- requirement;
- compatibility issue;
- migration information where available.

Diagnostics MUST NOT depend on target hardware discovery during parsing.

---

69. Error ownership

Errors must be reported by the correct subsystem.

Error| Owner
invalid token| lexer
invalid syntax| parser
unknown dialect identity| dialect resolution
unknown dialect version| dialect/version resolution
incompatible dialect| semantic compatibility
invalid type| type system
invalid effect| effect system
unavailable capability| capability analysis
insufficient resource| resource analysis
forbidden operation| policy/security
invalid lowering| compiler/IR
target infeasibility| target/resource layer
runtime failure| runtime

This prevents grammar files from accumulating semantic logic.

---

70. Rust integration

The repository currently targets Rust 2021 and declares Rust 1.97.

The dialect subsystem MUST remain compatible with:

Rust 1.97+
Rust 2021

Production Rust implementation MUST use safe Rust.

No dialect feature may require:

unsafe

inside the Rust compiler.

ANTLR grammar files themselves contain no Rust execution logic.

Generated parser integration must therefore remain compatible with the repository's selected ANTLR Rust toolchain.

---

71. Rust frontend synchronization

The repository has a Rust frontend independent of the ANTLR grammar.

Therefore:

ANTLR grammar

and:

src/lexer.rs
src/parser.rs
src/ast/

must not silently diverge.

A dialect feature is not considered implemented merely because its ".g4" file exists.

A complete implementation requires the applicable synchronization:

specification
   |
   v
ANTLR grammar
   |
   v
Rust lexer/parser
   |
   v
AST
   |
   v
semantic analysis
   |
   v
canonical IR

The conformance matrix in "grammar/grammar.md" remains the authoritative implementation-status reference.

---

72. No false implementation claims

Documentation MUST distinguish:

specified
implemented
tested
stable

A grammar file describing a future feature does not mean the Rust compiler already supports it.

A registry entry does not mean the backend exists.

A dialect declaration does not mean target realization exists.

This distinction is mandatory for production readiness.

---

73. Dialect conformance levels

Each dialect should report conformance at independent levels:

SPECIFIED
LEXER_IMPLEMENTED
PARSER_IMPLEMENTED
AST_IMPLEMENTED
SEMANTIC_IMPLEMENTED
TYPE_IMPLEMENTED
EFFECT_IMPLEMENTED
RESOURCE_IMPLEMENTED
CAPABILITY_IMPLEMENTED
CONTRACT_IMPLEMENTED
POLICY_IMPLEMENTED
PROVENANCE_IMPLEMENTED
IR_IMPLEMENTED
LOWERING_IMPLEMENTED
BACKEND_IMPLEMENTED
TESTED
STABLE

A dialect may be:

PARSER_IMPLEMENTED

without being:

BACKEND_IMPLEMENTED

The status must remain explicit.

---

74. Required dialect feature contract

Every dialect feature MUST have the following contract documented before it is considered complete.

Feature Contract

Purpose

Owns

Does Not Own

Identity

Namespace

Version

Lifecycle Status

Dependencies

Lexer Dependencies

Grammar Dependencies

Public Rules

Private Rules

AST Contract

Semantic Contract

Type Contract

Effect Contract

Capability Contract

Resource Contract

Requirement Contract

Constraint Contract

Policy Contract

Provenance Contract

Diagnostics

Compatibility

Migration

Canonical IR Destination

Lowering Destination

Runtime Integration

Target Integration

Positive Tests

Negative Tests

Boundary Tests

Scalability Tests

Determinism Tests

Compatibility Tests

Security Tests

Hard-Coding Audit

Completion Criteria

This contract makes a feature independently maintainable.

---

75. File-level ownership contract

Every ".g4" file under this directory MUST document:

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

Example:

File:
grammar/dialects/versioning.g4

DEPENDS_ON:
grammar/core/versioning.g4
grammar/core/names.g4

EXPORTS:
dialectVersionDeclaration
dialectVersionConstraint

AST_OWNER:
frontend AST / dialect version nodes

SEMANTIC_OWNER:
dialect compatibility resolver

TYPE_OWNER:
grammar/types/

EFFECT_OWNER:
none

CAPABILITY_OWNER:
none

RESOURCE_OWNER:
none

CONTRACT_OWNER:
grammar/validation/

POLICY_OWNER:
grammar/policies/

PROVENANCE_OWNER:
compiler provenance

IR_OWNER:
semantic metadata; no independent IR

TEST_OWNER:
grammar/tests/dialects/versioning/

SPEC_OWNER:
grammar/specification/language-version.md
grammar/specification/compatibility.md

The purpose is to establish integration before implementation, rather than discovering dependencies afterward.

---

76. "dialect.g4"

"grammar/dialects/dialect.g4" is the public dialect syntax boundary.

It MUST own:

- dialect declaration composition;
- public dialect syntax entry points;
- integration of declaration/import/export/version/namespace components.

It MUST NOT own:

- physical hardware selection;
- semantic compatibility algorithms;
- package installation;
- runtime execution;
- quantum routing;
- scheduling;
- QEC;
- target discovery.

Its dependencies should remain directed toward the dialect component grammars and canonical core grammar.

---

77. "dialects.g4"

"grammar/dialects/dialects.g4" is the dialect composition layer.

It should compose:

declaration
imports
exports
namespaces
registration
versioning
compatibility
capabilities
extension-points
vendor
experimental

It MUST NOT duplicate those files' detailed rules.

It should act as a composition boundary.

---

78. "declaration.g4"

Owns dialect declaration context.

It must preserve:

- dialect identity;
- optional version information;
- declaration metadata;
- source location.

It does not resolve whether the dialect exists.

---

79. "registration.g4"

Owns registration syntax/context.

Registration metadata may describe:

identity
owner
version
status
namespace
dependencies
features
capabilities
compatibility
provenance

It must not become executable configuration.

---

80. "namespaces.g4"

Owns namespace syntax.

It must support hierarchical names without fixed depth.

It must integrate with:

grammar/core/names.g4
grammar/core/qualified-names.g4

where those files exist.

It must not duplicate the universal identifier grammar.

---

81. "imports.g4"

Owns dialect import/use syntax.

It must integrate with:

grammar/modules/
grammar/core/names/
grammar/compatibility/

It must not directly resolve packages or access external systems.

---

82. "exports.g4"

Owns dialect export declarations.

It must integrate with module visibility and namespace resolution.

It must not bypass security or capability checks.

---

83. "versioning.g4"

Acts as a context adapter around core versioning.

It must not define a second version language.

Version comparison and compatibility remain semantic operations.

---

84. "compatibility.g4"

Owns the syntax for expressing compatibility relationships.

It does not decide compatibility.

Semantic analysis determines whether the relationship is satisfied.

---

85. "capabilities.g4"

Owns dialect-specific capability declaration/adaptation syntax.

It consumes the universal capability vocabulary/model.

It must not invent a physical-device selection system.

---

86. "extension-points.g4"

Defines where extensions can attach.

It must remain structural.

It must not allow arbitrary replacement of core semantics.

---

87. "vendor.g4"

Defines vendor extension context.

Vendor identifiers must remain namespaced.

Vendor extensions must be explicitly distinguishable from stable Zamani core.

---

88. "experimental.g4"

Defines experimental extension context.

Experimental syntax must be identifiable and must not silently become stable.

---

89. "registry.md"

The registry document must define:

- identity;
- ownership;
- version;
- lifecycle;
- specification;
- implementation;
- compatibility;
- capabilities;
- conformance;
- provenance;
- migration;
- security/trust metadata where applicable.

It must explicitly state that registry metadata does not override language semantics.

---

90. SQL integration

"grammar/dialects/sql.g4" must be treated as an integration adapter.

It must not redefine:

SELECT
INSERT
UPDATE
DELETE

as Zamani core keywords.

SQL-specific vocabulary belongs to SQL syntax.

The SQL subsystem should integrate with:

grammar/data/
grammar/interoperability/
grammar/dialects/

as appropriate.

---

91. XML integration

"grammar/dialects/xml.g4" must remain isolated from the universal Zamani grammar.

XML names, attributes and structure must remain XML concepts until translated into the appropriate interoperability/data representation.

No XML-specific physical limits may be introduced.

---

92. Future external formats

The architecture must allow future formats without modifying the core dialect grammar merely to enumerate them.

Potential future integrations include:

new database query languages
new quantum source formats
new HDL formats
new accelerator languages
new serialization formats
new scientific formats
new vendor formats

Each should be registered through metadata and implemented through the appropriate adapter.

---

93. Dialect-specific operators

A dialect may define an operator only when the operator has stable language-level meaning.

Operator extensions must integrate with:

grammar/lexer/operators
grammar/expressions
types
semantic analysis
diagnostics
precedence

A dialect must not silently assign a different meaning to an existing operator.

---

94. Dialect-specific literals

New literal families must integrate with the canonical literal architecture.

Examples may include:

quantum-specific literals
resource literals
duration literals
domain literals
tensor literals

They must have:

- lexer contract;
- parser contract;
- AST contract;
- type contract;
- semantic contract;
- diagnostics;
- tests.

Literal size must not create artificial universal limits.

---

95. Dialect-specific types

Dialect-specific types must integrate with Zamani's universal type system.

They must specify:

type identity
parameters
constraints
operations
conversion rules
ownership
effects
resource semantics
serialization
interop

A dialect MUST NOT create an incompatible type universe.

---

96. Dialect-specific effects

If a dialect operation introduces an effect, it must use the common effect model.

For example:

dialect operation
       |
       v
effect("foreign")

rather than inventing a second effect mechanism.

---

97. Dialect-specific resource semantics

A dialect may declare logical resource requirements.

Examples:

requires memory >= required_memory;
requires qubits >= required_qubits;
requires capability("tensor.compute");
requires topology(required_topology);

The dialect MUST NOT resolve physical allocation.

---

98. Dialect-specific contracts

A dialect may add domain-specific contract predicates.

Those predicates must integrate with the universal contract model.

They should produce semantic conditions that can be:

- checked;
- propagated;
- verified;
- diagnosed;
- preserved through lowering where required.

---

99. Dialect-specific policies

Dialect policies must integrate with the universal policy model.

A dialect must not create an independent authorization system.

The policy hierarchy is:

source intent
    |
    v
dialect policy
    |
    v
universal policy
    |
    v
security/authorization

---

100. Dialect-specific provenance

A dialect must preserve its identity through transformations when provenance is required.

Example:

dialect = quantum::example
version = 1.2
operation = ...
transformation = ...

This allows transformed IR to remain explainable.

---

101. Dialects and explanation

Where the language supports explanation/decision records, dialect transformations should expose sufficient provenance to explain:

- why an extension was selected;
- which compatibility rule was applied;
- which lowering was selected;
- which capability was required;
- which fallback was used;
- which optimization transformed the operation.

Explanation remains a universal semantic facility rather than a dialect-specific logging format.

---

102. Dialects and adaptive execution

A dialect may declare that an operation supports adaptive execution.

For example:

detect
evaluate
select
retry
recover
fallback

But adaptive behavior must integrate with the universal execution model.

The dialect grammar must not embed runtime state machines.

Runtime states such as:

Healthy
Degraded
Unstable
Unavailable
Recovering
Quarantined
Retired

belong to execution/resilience semantics.

---

103. Dialects and simulation

A dialect may support simulation semantics.

Simulation is an execution strategy, not another language.

The architecture is:

dialect program
     |
     v
canonical semantic model
     |
     +--> physical execution
     |
     +--> simulation

This permits classical, quantum, HDL and hardware simulation without creating parallel languages.

---

104. Dialects and sandboxing

Dialect operations that can perform:

- native calls;
- foreign calls;
- network operations;
- filesystem operations;
- reflection;
- code generation;
- adaptation;

must integrate with the sandbox/policy system.

A dialect cannot grant itself unrestricted authority.

---

105. Dialect composition with quantum/classical hybrid programs

A hybrid program may combine dialects:

classical
+
quantum
+
AI
+
tensor
+
distributed

The architecture must remain:

one AST
   |
   v
common semantic model
   |
   +--> classical representation
   |
   +--> quantum::ir
   |
   +--> tensor/data representation
   |
   +--> distributed representation

There must not be a separate AST or IR for every combination.

---

106. Dialect composition with HDL

A program may combine:

classical control
+
quantum control
+
HDL intent
+
hardware capabilities

The dialect layer provides syntax boundaries.

Semantic analysis determines how those domains interact.

Lowering determines target realization.

---

107. Dialect composition with AI

AI-related dialect constructs must consume universal:

reasoning
knowledge
learning
adaptation
uncertainty
evidence
explanation
provenance
policy
agent

They must not create an AI-only language.

---

108. Dialect composition with data

Data dialects should integrate with:

types
schemas
queries
streams
provenance
serialization
interoperability

Data format syntax remains distinct from universal program syntax.

---

109. Dialect composition with networking

Networking dialects must integrate with:

network effects
capabilities
security
policies
distributed semantics
resource requirements

Network addresses and endpoints are runtime/data values, not dialect identities.

---

110. Dialect composition with security

Dialect use must be subject to:

capabilities
authorization
policies
sandboxing
provenance
audit

A dialect cannot bypass those mechanisms.

---

111. Dialect composition with metaprogramming

A dialect may participate in:

reflection
compile-time evaluation
code generation
syntax trees
macros

but generated dialect constructs must still pass:

lexical validation
syntax validation
AST validation
semantic validation
type checking
effect checking
capability checking
resource checking
policy checking

---

112. Dialect composition with macros

Macros may generate dialect syntax only when the target dialect is explicitly available.

Generated source must not silently activate arbitrary dialects.

The compiler must retain provenance:

generated_by
source_macro
source_span
dialect
version

where required.

---

113. Dialect composition with packages

A package may provide dialect implementations.

Package metadata must not override source-level dialect meaning.

Package resolution belongs to the package/build infrastructure.

Dialect registry metadata should identify implementation availability.

---

114. Dialect composition with compiler versions

A dialect contract must declare compatibility with the language/compiler contracts it requires.

The compiler must distinguish:

language compatibility
dialect compatibility
compiler implementation compatibility
IR compatibility
backend compatibility

These are not interchangeable.

---

115. Dialect composition with target capabilities

The correct separation is:

source dialect
      |
      v
semantic requirement
      |
      v
capability resolver
      |
      v
available target capabilities
      |
      v
realization

The dialect must not contain a hard-coded list of target devices.

---

116. Target-independent dialect design

A production dialect should remain meaningful when:

hardware changes

provided its semantic contract remains satisfied.

For example:

requires capability("tensor.compute");

is target-independent.

This is preferable to:

requires gpu("specific-model");

unless the latter is intentionally an explicit target constraint rather than a universal dialect semantic.

---

117. Physical constraints

Physical constraints belong downstream.

Examples:

- actual memory;
- actual qubit count;
- actual coupling;
- actual FPGA resources;
- actual GPU availability;
- actual network topology;
- actual thermal budget.

The dialect can express logical requirements.

Resource negotiation determines feasibility.

---

118. Hard-coding audit

Every dialect file must be audited for:

fixed counts
fixed capacities
finite enumerations
physical identifiers
target-specific assumptions
application-specific keyword lists
implicit limits
hidden parser ceilings
fixed namespace depth
fixed dependency count
fixed extension count

Forbidden universal names include:

MAX_QUBITS
MAX_CPUS
MAX_GPUS
MAX_FPGAS
MAX_NODES
MAX_MEMORY
MAX_THREADS
MAX_TENSOR_RANK
MAX_REGISTER_WIDTH
MAX_NETWORK_SIZE
MAX_DEVICE_COUNT
MAX_DIALECTS
MAX_EXTENSIONS

Equivalent disguised limits are also prohibited.

---

119. Scalability patterns

Prefer:

item*

or:

item+

over fixed repetitions.

Prefer:

symbolic identity

over finite enumeration.

Prefer:

capability("...")

over physical-device lists.

Prefer:

parameterized semantic representation

over fixed dimensions.

Prefer:

metadata-driven registration

over root-grammar modification.

---

120. No universal catalogue

The dialect subsystem must not attempt to enumerate every future:

- vendor;
- accelerator;
- QPU;
- CPU;
- GPU;
- FPGA;
- HDL;
- AI framework;
- data format;
- protocol;
- algorithm;
- operation.

Universal syntax should describe stable computational concepts.

Specific entities should be represented by names, metadata, libraries, registries and dialects.

---

121. Test architecture

Dialect tests must be divided into:

grammar/tests/dialects/
├── lexical/
├── parser/
├── ast/
├── semantic/
├── types/
├── effects/
├── resources/
├── capabilities/
├── contracts/
├── policies/
├── provenance/
├── versioning/
├── compatibility/
├── registration/
├── namespaces/
├── imports/
├── exports/
├── vendor/
├── experimental/
├── quantum/
├── classical/
├── hdl/
├── ai/
├── data/
├── distributed/
├── interoperability/
├── scalability/
├── determinism/
├── security/
├── positive/
├── negative/
└── boundary/

The exact existing test location may differ, but the conceptual test ownership must be preserved.

---

122. Positive tests

Every dialect must have valid examples covering:

- minimal declaration;
- version;
- namespace;
- import;
- export;
- registration;
- extension;
- capability;
- compatibility;
- domain-specific constructs.

---

123. Negative tests

Every dialect must test:

- malformed identity;
- invalid namespace;
- invalid version;
- incompatible version;
- unknown dependency;
- duplicate declaration;
- conflicting extension;
- invalid capability;
- invalid syntax;
- unsupported construct;
- prohibited extension.

---

124. Boundary tests

Boundary tests must combine dialects with:

- core syntax;
- modules;
- types;
- effects;
- resources;
- capabilities;
- contracts;
- policies;
- provenance;
- quantum;
- HDL;
- AI;
- distributed;
- interoperability.

---

125. Scalability tests

Scalability tests must verify that the implementation does not contain artificial ceilings.

Test increasingly large symbolic structures such as:

many dialect declarations
many namespaces
many dependencies
many capabilities
many requirements
many extensions
many imports
many exports
large version expressions
large dialect metadata

The tests should stress actual resource availability rather than asserting an arbitrary maximum.

---

126. Determinism tests

Given identical:

source
grammar version
dialect configuration
compiler configuration

the parser must produce equivalent results.

Tests must verify that parsing does not change based on:

- machine type;
- hardware availability;
- time;
- randomness;
- environment variables;
- filesystem state;
- network state.

---

127. Compatibility tests

Compatibility tests must cover:

same version
compatible version
incompatible version
deprecated version
migration-required version
missing version
future version

The expected result must be deterministic.

---

128. Security tests

Security tests must verify that dialect processing does not:

- execute arbitrary code;
- access the network;
- access the filesystem;
- bypass capability checks;
- bypass policies;
- silently invoke foreign functions;
- access secrets;
- perform hardware discovery during parsing.

---

129. Integration tests

At minimum, dialect integration must be tested through:

source
  |
  v
lexer
  |
  v
parser
  |
  v
AST
  |
  v
structural validation
  |
  v
semantic model
  |
  v
type/effect analysis
  |
  v
resource/capability analysis
  |
  v
contract/policy analysis
  |
  v
provenance
  |
  v
canonical IR

Where a domain applies, continue through:

lowering
routing
scheduling
resilience
ZQN
HAL
target

---

130. Required cross-domain test

A production dialect architecture must support a combined program containing, where applicable:

dialect declaration
+
classical computation
+
reasoning
+
learning
+
uncertainty
+
tensor operation
+
quantum operation
+
measurement
+
resource requirements
+
capabilities
+
effects
+
contracts
+
policies
+
provenance
+
simulation
+
parallelism
+
distributed execution
+
hardware intent

The expected architecture is:

source
   |
   v
lexer
   |
   v
parser
   |
   v
AST
   |
   v
semantic validation
   |
   +--> types
   +--> effects
   +--> capabilities
   +--> resources
   +--> contracts
   +--> policies
   +--> provenance
   |
   v
canonical semantic model
   |
   +--> classical representation
   |
   +--> quantum::ir
   |
   +--> HDL/hardware representation
   |
   +--> AI/data representation
   |
   +--> distributed representation
   |
   v
target-independent optimization
   |
   v
lowering
   |
   v
routing
   |
   v
scheduling
   |
   v
resilience
   |
   v
target realization

---

131. Production readiness checklist

A dialect is production-ready only when all applicable items are complete.

Authority

- [ ] Specification exists.
- [ ] Ownership is explicit.
- [ ] No competing specification exists.
- [ ] No competing grammar authority exists.

Identity

- [ ] Identity is stable.
- [ ] Namespace is defined.
- [ ] Version is defined.
- [ ] Lifecycle status is defined.

Syntax

- [ ] Lexer integration exists.
- [ ] Parser integration exists.
- [ ] Extension point is explicit.
- [ ] No syntax duplication exists.

AST

- [ ] AST mapping exists.
- [ ] Source spans are preserved.
- [ ] No competing AST exists.

Semantics

- [ ] Semantic model exists.
- [ ] Type interactions are defined.
- [ ] Effect interactions are defined.
- [ ] Resource interactions are defined.
- [ ] Capability interactions are defined.
- [ ] Contract interactions are defined.
- [ ] Policy interactions are defined.
- [ ] Provenance interactions are defined.

IR

- [ ] Canonical IR destination exists.
- [ ] No competing permanent IR exists.
- [ ] Lowering contract exists.

Compatibility

- [ ] Version compatibility is defined.
- [ ] Migration policy exists.
- [ ] Deprecation policy exists.
- [ ] Diagnostics exist.

Security

- [ ] Capability requirements are separate from authorization.
- [ ] Sandbox behavior is defined where required.
- [ ] No parser-side execution exists.
- [ ] No secret access exists.

Scalability

- [ ] No artificial capacity ceiling exists.
- [ ] No finite universal dialect enumeration exists.
- [ ] No fixed namespace depth exists.
- [ ] No fixed extension count exists.
- [ ] No physical hardware ceiling exists.

Testing

- [ ] Positive tests.
- [ ] Negative tests.
- [ ] Boundary tests.
- [ ] Scalability tests.
- [ ] Determinism tests.
- [ ] Compatibility tests.
- [ ] Security tests.
- [ ] Cross-domain tests.

Rust

- [ ] Rust 2021.
- [ ] Rust 1.97 or later.
- [ ] No "unsafe".
- [ ] Generated parser integration compiles.
- [ ] Reference frontend integration is verified.

Only when the applicable checklist is complete may the dialect be labelled "stable".

---

132. Required repository integration

The dialect subsystem integrates with the following existing areas.

grammar/core/
    |
    +--> names
    +--> identifiers
    +--> versioning
    +--> attributes
    +--> metadata

grammar/types/
    |
    +--> dialect-defined types

grammar/expressions/
    |
    +--> dialect-defined expressions

grammar/statements/
    |
    +--> dialect-defined statements

grammar/declarations/
    |
    +--> dialect-defined declarations

grammar/modules/
    |
    +--> dialect imports/exports

grammar/resources/
    |
    +--> requirements

grammar/effects/
    |
    +--> dialect effects

grammar/validation/
    |
    +--> contracts

grammar/policies/
    |
    +--> policy restrictions

grammar/security/
    |
    +--> authorization/sandboxing

grammar/provenance/
    |
    +--> dialect transformation history

grammar/interoperability/
    |
    +--> external formats

grammar/classical/
    |
    +--> classical domain semantics

grammar/quantum/
    |
    +--> quantum semantics / quantum::ir

grammar/hybrid/
    |
    +--> cross-domain semantics

grammar/hdl/
    |
    +--> HDL semantics

grammar/hardware/
    |
    +--> hardware capabilities

grammar/ai/
    |
    +--> reasoning/learning/knowledge

grammar/data/
    |
    +--> data/query semantics

grammar/concurrency/
    |
    +--> actor/task semantics

grammar/distributed/
    |
    +--> distributed semantics

grammar/networking/
    |
    +--> network semantics

grammar/metaprogramming/
    |
    +--> generated dialect constructs

grammar/execution/
    |
    +--> adaptive/simulation execution

grammar/compile/
    |
    +--> compilation/lowering

grammar/compatibility/
    |
    +--> language compatibility

grammar/tests/
    |
    +--> conformance

---

133. Integration ownership matrix

The following matrix is normative.

Concern| Dialects owns?| Actual owner
Dialect identity syntax| Yes| "grammar/dialects/"
Core identifiers| No| "grammar/core/" / lexer
Core version syntax| No| core versioning
Dialect version context| Yes| "dialects/versioning.g4"
Type semantics| No| "grammar/types/"
Effects| No| "grammar/effects/"
Resources| No| "grammar/resources/"
Capabilities| Adapter| universal capability subsystem
Contracts| No| "grammar/validation/"
Policies| No| "grammar/policies/"
Security| No| "grammar/security/"
Provenance| No| provenance subsystem
Quantum semantics| No| "grammar/quantum/"
Quantum IR| No| "quantum::ir"
HDL semantics| No| "grammar/hdl/"
Hardware realization| No| "grammar/hardware/"
Runtime execution| No| "grammar/execution/" / runtime
Target selection| No| compiler/resource layer
Routing| No| quantum/backend layer
Scheduling| No| execution/backend layer
QEC| No| quantum resilience layer
ZQN| No| quantum backend
HAL| No| hardware abstraction layer
External SQL syntax| Adapter| interoperability/SQL
External XML syntax| Adapter| interoperability/XML
Package resolution| No| package/build infrastructure
AST| No| frontend AST
Canonical IR| No| compiler IR subsystem

This prevents responsibility creep.

---

134. Independence-first implementation order

Dialect implementation should proceed independently before integration.

Step 1 — documentation contract

Complete:

grammar/dialects/README.md

This file.

Step 2 — dialect identity

Complete:

dialect.g4
declaration.g4
namespaces.g4

Step 3 — versioning

Complete:

versioning.g4
compatibility.g4

Step 4 — module visibility

Complete:

imports.g4
exports.g4

Step 5 — registration

Complete:

registration.g4
registry.md

Step 6 — extension system

Complete:

extension-points.g4
vendor.g4
experimental.g4

Step 7 — capabilities

Complete:

capabilities.g4

Step 8 — domain integrations

Integrate with:

quantum/
classical/
hdl/
hardware/
ai/
data/
distributed/
interoperability/

Step 9 — semantic integration

Integrate:

AST
types
effects
resources
capabilities
contracts
policies
provenance

Step 10 — canonical IR

Integrate with the appropriate canonical representation.

Quantum MUST terminate at:

quantum::ir

where applicable.

Step 11 — lowering

Integrate with compiler lowering.

Step 12 — target realization

Integrate with:

routing
scheduling
resilience
ZQN
HAL
backend

where applicable.

Step 13 — conformance

Complete the full dialect test matrix.

---

135. Definition of DONE for each dialect file

A dialect file is not done merely because its grammar compiles.

A file is DONE when:

purpose documented
        +
ownership documented
        +
non-ownership documented
        +
dependencies documented
        +
exports documented
        +
AST contract documented
        +
semantic owner documented
        +
integration documented
        +
diagnostics documented
        +
tests documented
        +
scalability audited
        +
hard-coding audited
        +
compatibility documented

and the implementation satisfies the contract.

After another independent file changes, this file should not need to be reopened merely to rediscover its ownership or integration contract.

Changes are required only when the contract itself changes.

---

136. Definition of DONE for "grammar/dialects/README.md"

This README is complete when it provides:

- dialect architecture;
- ownership;
- integration boundaries;
- lifecycle;
- versioning;
- compatibility;
- registration;
- namespaces;
- imports;
- exports;
- extension points;
- vendor extensions;
- experimental extensions;
- capability integration;
- resource integration;
- effect integration;
- contract integration;
- policy integration;
- provenance integration;
- interoperability boundaries;
- quantum integration;
- classical integration;
- HDL integration;
- AI/data integration;
- distributed integration;
- Rust requirements;
- no-unsafe requirement;
- POCO-REAF requirements;
- scalability requirements;
- hard-coding prohibitions;
- AST boundary;
- canonical IR boundary;
- testing requirements;
- production-readiness requirements.

No dialect-specific physical hardware limit belongs in this README.

---

137. What this architecture deliberately prevents

This architecture prevents the following failure modes.

Failure 1 — dialect becomes another language

Prevented by shared AST, semantics, types, effects, resources and IR.

Failure 2 — root grammar becomes a catalogue

Prevented by open-world dialect identities.

Failure 3 — every new vendor requires root grammar changes

Prevented by symbolic namespaces and registration.

Failure 4 — every quantum operation becomes a keyword

Prevented by generic operation semantics.

Failure 5 — physical hardware leaks into source grammar

Prevented by capability/resource separation.

Failure 6 — external languages become Zamani core

Prevented by the interoperability boundary.

Failure 7 — parser performs semantic decisions

Prevented by parser/semantic ownership separation.

Failure 8 — registry becomes language authority

Prevented by the authority hierarchy.

Failure 9 — application vocabulary explodes the lexer

Prevented by identifier-first extensibility.

Failure 10 — dialects create incompatible IRs

Prevented by canonical IR contracts.

Failure 11 — portability is confused with infinite hardware

Prevented by explicit resource feasibility semantics.

Failure 12 — Rust safety is compromised

Prevented by the mandatory Rust 1.97+ / safe-Rust requirement.

---

138. Final architecture

The complete dialect architecture is:

                         ZAMANI SOURCE
                              |
                              v
                         CANONICAL LEXER
                              |
                              v
                       grammar/Zamani.g4
                              |
                              v
                     DIALECT COMPOSITION
                              |
             +----------------+----------------+
             |                |                |
             v                v                v
        declarations      namespaces       imports/exports
             |                |                |
             +----------------+----------------+
                              |
                              v
                       dialect identity
                              |
                              v
                     version / compatibility
                              |
                              v
                       extension points
                              |
                +-------------+-------------+
                |                           |
                v                           v
             vendor                    experimental
                |                           |
                +-------------+-------------+
                              |
                              v
                      DOMAIN-NEUTRAL AST
                              |
                              v
                    STRUCTURAL VALIDATION
                              |
                              v
                     SEMANTIC RESOLUTION
                              |
        +---------------------+----------------------+
        |          |           |          |          |
        v          v           v          v          v
      types     effects    resources capabilities contracts
        |          |           |          |          |
        +----------+-----------+----------+----------+
                              |
                              v
                           policies
                              |
                              v
                         provenance
                              |
                              v
                   CANONICAL SEMANTIC MODEL
                              |
          +-------------------+-------------------+
          |                   |                   |
          v                   v                   v
      classical           quantum             HDL/hardware
         IR              quantum::ir           representation
          |                   |                   |
          +-------------------+-------------------+
                              |
                              v
                    TARGET-INDEPENDENT WORK
                              |
                              v
                         OPTIMIZATION
                              |
                              v
                          LOWERING
                              |
                              v
                 RESOURCE / CAPABILITY NEGOTIATION
                              |
                              v
                           ROUTING
                              |
                              v
                         SCHEDULING
                              |
                              v
                    RESILIENCE / RECOVERY
                              |
                              v
                             ZQN
                              |
                              v
                             HAL
                              |
             +----------------+----------------+
             |        |       |       |        |
             v        v       v       v        v
            CPU      GPU    FPGA    ASIC      QPU
             |        |       |       |        |
             +--------+-------+-------+--------+
                              |
                              v
                    embedded / HPC / cluster /
                    distributed / cloud / future

The central rule is:

«A dialect extends Zamani's expression of computational intent; it does not redefine the universal language, physical machine, resource universe, or compiler architecture.»

That makes the dialect system compatible with POCO-REAF: a program can use a dialect to express richer intent while the compiler remains responsible for mapping that intent onto whatever compatible resources are actually available.

---

139. Final production invariant

For every production dialect, the repository must be able to answer all of these questions without ambiguity:

What is the dialect?

Who owns it?

What version is it?

Where is it specified?

Where is its syntax?

Where is its lexer integration?

Where is its AST representation?

Where is its semantic representation?

What types does it use?

What effects does it produce?

What capabilities does it require?

What resources does it require?

What contracts apply?

What policies apply?

What provenance is preserved?

What canonical IR receives it?

How is it lowered?

How is it tested?

How is it versioned?

How is it migrated?

How is it deprecated?

How is it secured?

How does it compose with other dialects?

How does it scale?

What physical assumptions does it avoid?

What happens when its requirements cannot be satisfied?

Which file owns each responsibility?

Which files consume its exported rules?

Which specification is authoritative?

If any of these questions cannot be answered, the dialect is not yet production-ready.

---

140. Non-negotiable invariants

The following invariants apply to every current and future dialect:

1. One Zamani language.
2. One authoritative dialect architecture.
3. Open-world dialect identity.
4. No finite universal dialect catalogue.
5. No physical hardware identities in universal dialect semantics.
6. No artificial resource ceilings.
7. No duplicate core type system.
8. No duplicate effect system.
9. No duplicate resource system.
10. No duplicate capability system.
11. No duplicate contract system.
12. No duplicate policy system.
13. No duplicate provenance system.
14. No competing permanent AST architecture.
15. No competing permanent canonical IR.
16. Quantum semantics converge on "quantum::ir".
17. External formats remain interoperability boundaries unless explicitly promoted to native dialects.
18. Parser logic remains deterministic and side-effect free.
19. Semantic decisions remain outside the grammar.
20. Capability requirements do not grant authorization.
21. Resource requirements do not select physical hardware.
22. Vendor extensions remain explicitly namespaced.
23. Experimental extensions remain explicitly experimental.
24. Application concepts do not become universal keywords without justification.
25. Rust implementation remains Rust 1.97+ and safe Rust.
26. No Rust "unsafe".
27. Every production feature has a complete integration contract.
28. Every production feature has positive, negative and boundary tests.
29. Scalability is constrained by actual resources, not arbitrary language ceilings.
30. Changing hardware must not require changing portable program meaning.

---

141. Completion statement

"grammar/dialects/README.md" is therefore the architectural contract for the dialect subsystem.

It does not claim that every existing ".g4" file, Rust parser, AST component, semantic implementation, IR lowering or backend has already achieved the production status described here.

Instead, it establishes the exact conditions under which each one can legitimately be declared complete.

The implementation target is:

SPECIFICATION
      ↓
IDENTITY
      ↓
LEXER
      ↓
GRAMMAR
      ↓
AST
      ↓
SEMANTICS
      ↓
TYPES
      ↓
EFFECTS
      ↓
CAPABILITIES
      ↓
RESOURCES
      ↓
CONTRACTS
      ↓
POLICIES
      ↓
PROVENANCE
      ↓
CANONICAL IR
      ↓
LOWERING
      ↓
OPTIMIZATION
      ↓
ROUTING
      ↓
SCHEDULING
      ↓
RESILIENCE
      ↓
ZQN / HAL
      ↓
TARGET
      ↓
CONFORMANCE

A dialect is production-ready only when its applicable path through this pipeline is implemented, tested, deterministic, compatible, scalable, secure, and free of artificial hard-coded ceilings.
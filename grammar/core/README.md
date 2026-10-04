Zamani Core Grammar

Path: "grammar/core/README.md"
Language: Zamani
Layer: Universal, domain-neutral grammar foundation
Status: Production architecture contract
Rust baseline: Rust 2021, Rust 1.97.1
Safety: Safe Rust only; no "unsafe" implementation requirement
Primary portability objective: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)

---

1. Purpose

"grammar/core/" defines the foundational, domain-neutral syntax contracts shared by the entire Zamani language.

The core grammar is the common syntactic foundation for:

- classical computation;
- quantum computation;
- hybrid computation;
- HDL;
- hardware/software co-design;
- embedded systems;
- distributed computing;
- parallel/HPC computing;
- AI and machine learning;
- symbolic reasoning;
- knowledge systems;
- data and tensor computation;
- networking;
- cryptography;
- security;
- interoperability;
- metaprogramming;
- simulation;
- accelerators;
- future computational domains.

The core layer defines portable source meaning and structure.

It does not define a physical machine.

The central rule is:

«Core grammar expresses what a Zamani program means and what properties its realization must satisfy. It does not decide how a particular machine realizes that meaning.»

---

2. Authority

The core directory is part of the canonical grammar hierarchy.

The authority chain is:

normative specification
        ↓
grammar/specification/
        ↓
grammar/spec/
        ↓
feature contracts
        ↓
grammar/antlr/
        ↓
grammar/Zamani.g4
        ↓
domain-neutral AST
        ↓
semantic analysis
        ↓
canonical semantic model
        ↓
canonical IR
        ↓
compiler/runtime/backend

The following distinction is mandatory:

Normative specification

Defines language meaning.

ANTLR grammar

Defines accepted source syntax.

AST

Defines domain-neutral structural representation.

Semantic layer

Determines meaning, validity, types, effects, capabilities, resources, contracts, policies, and portability.

IR

Represents semantically validated computation.

Backend/runtime

Determines realization.

No core ".g4" file may bypass these boundaries.

---

3. Relationship to the Complete Grammar

The top-level architecture is:

grammar/Zamani.g4
        │
        ├── grammar/antlr/ZamaniParser.g4
        │       │
        │       ├── grammar/core/
        │       ├── grammar/types/
        │       ├── grammar/expressions/
        │       ├── grammar/statements/
        │       ├── grammar/declarations/
        │       ├── grammar/functions/
        │       ├── grammar/modules/
        │       ├── grammar/effects/
        │       ├── grammar/resources/
        │       ├── grammar/validation/
        │       ├── grammar/policies/
        │       ├── grammar/concurrency/
        │       ├── grammar/execution/
        │       ├── grammar/classical/
        │       ├── grammar/quantum/
        │       ├── grammar/hybrid/
        │       ├── grammar/hdl/
        │       ├── grammar/hardware/
        │       ├── grammar/distributed/
        │       ├── grammar/ai/
        │       ├── grammar/data/
        │       ├── grammar/networking/
        │       ├── grammar/security/
        │       ├── grammar/interoperability/
        │       ├── grammar/dialects/
        │       ├── grammar/macros/
        │       └── grammar/metaprogramming/
        │
        └── grammar/antlr/ZamaniLexer.g4
                │
                └── canonical lexical hierarchy

"grammar/Zamani.g4" remains the single complete-program composition root.

It must not directly import every feature directory.

The parser composition hierarchy owns those imports.

---

4. What "grammar/core/" Owns

The core layer owns universal syntax shared across domains.

It owns:

- compilation-unit structure;
- source-unit structure;
- identifiers;
- names;
- qualified names;
- paths;
- attributes;
- annotations;
- metadata;
- version declarations;
- capability references;
- requirement syntax;
- constraint syntax;
- preference/hint syntax;
- policy syntax;
- universal source directives;
- universal declarative modifiers where applicable.

It may provide generic syntactic building blocks consumed by other grammar domains.

---

5. What "grammar/core/" Does Not Own

Core does not own:

- classical computation semantics;
- quantum operation semantics;
- "quantum::ir";
- HDL semantics;
- hardware realization;
- physical topology;
- routing;
- scheduling;
- optimization;
- calibration;
- QEC;
- ZQN;
- HAL;
- device discovery;
- resource allocation;
- backend selection;
- runtime state;
- actor scheduling;
- network routing;
- AI model execution;
- training algorithms;
- compiler optimization algorithms.

The core grammar must never become a second semantic or backend layer.

---

6. POCO-REAF Contract

Zamani is designed around:

Program Once
      ↓
Compile Once
      ↓
Realize according to capabilities/resources/policies
      ↓
Run Everywhere
      ↓
Run Anywhere
      ↓
Remain portable across future environments

POCO-REAF means that the source program describes stable computation and its semantic obligations rather than embedding assumptions about one machine.

The source may express:

requirements
constraints
capabilities
preferences
hints
policies
contracts
effects
provenance

The source should not have to be rewritten merely because realization changes from:

tiny embedded system
CPU
multicore CPU
GPU
FPGA
ASIC
accelerator
QPU
quantum simulator
HPC system
cluster
distributed environment
cloud
future hardware

provided the target can legally satisfy the program's semantics.

---

7. Scalability Principle

The language has no universal finite computational scale.

The core grammar must therefore not define artificial ceilings for:

- source size;
- declaration count;
- function count;
- module count;
- namespace depth;
- qualified-name depth;
- requirement count;
- policy count;
- constraint count;
- capability count;
- expression depth;
- computational domains;
- devices;
- CPUs;
- cores;
- threads;
- GPUs;
- FPGAs;
- accelerators;
- QPUs;
- qubits;
- nodes;
- processes;
- agents;
- channels;
- memory;
- storage;
- tensor rank;
- tensor dimensions;
- register width;
- network size;
- topology size.

The actual implementation may have resource limits imposed by:

- available memory;
- compiler resources;
- runtime resources;
- operating-system limits;
- target capabilities;
- physical feasibility;
- explicitly declared program constraints.

Those are implementation or execution conditions, not universal language ceilings.

---

8. Hard-Coding Prohibition

No core grammar file may introduce universal capacity constants.

In particular, core grammar must never establish fixed limits equivalent to:

maximum qubits
maximum CPUs
maximum GPUs
maximum FPGAs
maximum nodes
maximum memory
maximum threads
maximum tensor rank
maximum register width
maximum network size
maximum devices

A number appearing in source code remains source semantics.

For example:

let n = 1024;

may be valid program data.

It does not establish a universal Zamani capacity.

Similarly:

requires qubits >= n;

is a source-level requirement.

It does not hard-code the maximum number of qubits that Zamani can represent.

---

9. Domain Neutrality

Core syntax must remain independent of computational domain.

For example:

requires quantum::measurement;

and:

requires tensor::compute;

and:

requires distributed::communication;

are all expressions of the same universal requirement mechanism.

The core grammar must not create separate grammar architectures for:

QuantumRequirements
GpuRequirements
CpuRequirements
AIRequirements
HdlRequirements

Domain-specific meanings are registered by semantic owners.

---

10. Open-World Design

Core grammar must be open-world.

A new capability, resource property, vendor extension, computational domain, accelerator class, quantum technology, or future execution model must not require modifying the core grammar merely because its name is new.

Prefer:

domain::capability
domain::feature
vendor::extension
future::capability

over a finite grammar catalogue.

The grammar defines the structure.

Semantic registries define known meanings.

---

11. Core File Inventory

The intended production core is:

grammar/core/
├── README.md
├── compilation-unit.g4
├── source-unit.g4
├── identifiers.g4
├── names.g4
├── paths.g4
├── qualified-names.g4
├── attributes.g4
├── annotations.g4
├── modifiers.g4
├── metadata.g4
├── versioning.g4
├── capabilities.g4
├── requirements.g4
├── constraints.g4
├── policies.g4
├── preferences.g4
├── hints.g4
└── pragmas.g4

Only create a file when it has a distinct ownership boundary.

Do not split files merely to increase file count.

If two concepts have inseparable ownership, they may remain together.

---

12. "compilation-unit.g4"

Purpose

Defines the complete Zamani source compilation boundary.

Owns

- top-level source composition;
- compilation-unit structure;
- source item ordering;
- complete-input boundary.

Does not own

- lexer definitions;
- individual declarations;
- expressions;
- types;
- quantum semantics;
- HDL semantics;
- AST implementation;
- semantic validation.

Dependencies

Consumes the canonical parser vocabulary.

Exports

A complete compilation-unit rule.

Integration

Consumed by:

grammar/antlr/ZamaniParser.g4
grammar/Zamani.g4

The final public program rule must ultimately enforce:

sourceUnit EOF

so an accepted prefix cannot masquerade as a complete program.

Scalability

No fixed maximum number of source items.

Completion

Done when a complete Zamani source file has exactly one authoritative root.

---

13. "source-unit.g4"

Purpose

Defines source-level units beneath the compilation boundary.

Owns

- source item grouping;
- source-level ordering;
- source directives where applicable.

Does not own

- filesystem access;
- package downloading;
- dependency resolution;
- module loading;
- semantic resolution.

Integration

Feeds the domain-neutral AST.

Scalability

No fixed source-unit size or item count.

---

14. "identifiers.g4"

Purpose

Defines reusable identifier syntax.

Owns

- identifier structure;
- identifier components;
- identifier escaping if standardized.

Does not own

- symbol tables;
- scope;
- name lookup;
- declaration resolution.

Integration

Consumed by:

names.g4
qualified-names.g4
declarations/
modules/
types/
expressions/
statements/

Compatibility

Keyword reservation is determined by the canonical lexer.

The parser must not invent alternate keyword identities.

---

15. "names.g4"

Purpose

Defines canonical names.

Owns

- simple names;
- name references;
- reusable name structures.

Does not own

- semantic symbol resolution;
- namespace lookup;
- runtime identity.

Integration

All domains requiring a source-level name consume this common abstraction.

---

16. "paths.g4"

Purpose

Defines language-level paths.

Owns

- path components;
- source/module paths;
- path composition.

Does not own

- filesystem operations;
- network access;
- package downloading;
- dependency resolution.

A syntactically valid path does not imply permission to access the corresponding external resource.

---

17. "qualified-names.g4"

Purpose

Defines arbitrary qualified names.

Conceptually:

namespace::name
namespace::subnamespace::name
domain::feature::operation

Scalability

Use repetition rather than a finite number of qualification levels.

There must be no grammar rule equivalent to:

name::name::name

as the maximum supported depth.

Does not own

- namespace existence;
- symbol resolution;
- module loading;
- capability discovery.

---

18. "attributes.g4"

Purpose

Defines structured source attributes.

Owns

- attribute name;
- attribute arguments;
- attribute structure;
- attachment syntax.

Does not own

- attribute interpretation;
- optimization;
- backend behavior;
- runtime behavior.

Every semantic attribute must have a documented owner.

---

19. "annotations.g4"

Purpose

Defines annotations that communicate source-level metadata or intent.

Annotations may describe:

- semantics;
- tooling;
- diagnostics;
- compatibility;
- optimization intent;
- provenance;
- target-specific intent;
- dialect extensions.

Critical rule

An annotation is not automatically executable behavior.

Semantic interpretation belongs to the owning subsystem.

---

20. "modifiers.g4"

Purpose

Defines reusable syntactic modifiers.

Owns

- modifier composition;
- modifier ordering where semantically relevant;
- generic modifier attachment.

Does not own

The meaning of a domain-specific modifier.

For example, a quantum modifier's semantics belong to the quantum semantic subsystem.

---

21. "metadata.g4"

Purpose

Defines structured source metadata.

Metadata may represent:

- documentation;
- provenance references;
- language information;
- compatibility information;
- reproducibility declarations;
- source descriptors.

Does not own

- storage;
- signing;
- runtime telemetry;
- artifact databases.

---

22. "versioning.g4"

Purpose

Defines source-visible version and compatibility declarations.

Owns

- language version;
- grammar version;
- semantic version references;
- compatibility ranges where standardized.

Does not own

- compiler migration;
- package resolution;
- runtime negotiation.

Versioning must never encode temporary hardware characteristics.

---

23. "capabilities.g4"

Purpose

Defines references to capabilities.

A capability answers:

«What can an environment or semantic domain provide?»

Examples:

quantum::measurement
quantum::dynamic_control
tensor::compute
distributed::collectives
execution::deterministic
security::trusted_execution

Does not own

- capability discovery;
- device probing;
- hardware selection;
- allocation;
- scheduling.

Open-world rule

Capability names are not exhaustively enumerated by this grammar.

---

24. "requirements.g4"

Purpose

Defines source-level requirements.

A requirement answers:

«What must be true for this program or operation to be legally realized?»

Existing "requirements.g4" already establishes the correct architectural distinction between requirements and resource realization. That separation must be preserved. "Current requirements grammar" (https://reference-url-citation.invalid/1)

Examples:

requires quantum::measurement;
requires execution::deterministic;
requires distributed::communication;
requires security::trusted_execution;

Resource-specific expressions belong under the resource subsystem rather than being duplicated here.

Does not own

- satisfiability;
- resource allocation;
- hardware selection;
- scheduling;
- routing.

---

25. "constraints.g4"

Purpose

Defines conditions restricting valid realization.

Examples of semantic categories include:

- latency;
- precision;
- reliability;
- ordering;
- energy;
- communication;
- timing;
- memory properties.

Does not own

- scheduling;
- optimization;
- target selection.

A constraint is an input to those systems.

---

26. "preferences.g4"

Purpose

Defines non-mandatory preferred realization properties.

Example:

prefer low_latency;
prefer energy_efficiency;
prefer accelerator;

A preference must not silently become a requirement.

If a target cannot satisfy a preference, semantic validity remains unchanged unless the source explicitly declared a requirement instead.

---

27. "hints.g4"

Purpose

Defines non-binding implementation guidance.

A hint may help optimization without becoming part of program correctness.

Example conceptual meaning:

hint prefer_parallel;

Rule

Hints must never change the semantic result of a valid program.

---

28. "policies.g4"

Purpose

Defines the universal source-level policy structure.

Policies are important enough to be a first-class core abstraction.

A policy can express relationships among:

- requirements;
- constraints;
- permissions;
- prohibitions;
- preferences;
- fallbacks;
- adaptation;
- execution intent;
- security intent;
- deployment intent;
- simulation intent;
- provenance requirements.

Core rule

"policies.g4" defines policy syntax and structure.

It does not implement:

- authorization;
- security enforcement;
- resource allocation;
- scheduling;
- deployment;
- simulation;
- runtime adaptation.

Those remain downstream semantic responsibilities.

Open-world rule

Policy subjects, capabilities, effects, resources, and domain names must not be exhaustively enumerated in the core grammar.

Integration

The policy syntax is consumed by:

grammar/security/
grammar/resources/
grammar/execution/
grammar/compile/
grammar/deployment/
grammar/validation/
grammar/ai/
grammar/quantum/
grammar/hybrid/

where applicable.

The core policy model must remain domain-neutral.

---

29. "pragmas.g4"

Purpose

Defines explicit source-level implementation directives where such directives are part of the language contract.

Critical restriction

Pragmas must not become an unrestricted escape hatch around:

- type checking;
- effects;
- capabilities;
- resources;
- policies;
- security;
- contracts;
- portability.

Every pragma must have an identified semantic owner.

Unknown pragmas should be rejected or handled by a documented dialect mechanism rather than silently ignored.

---

30. Core Policy Model

The universal conceptual model is:

Policy
├── scope
├── condition
├── requirements
├── constraints
├── permissions
├── prohibitions
├── preferences
├── fallbacks
├── adaptation
├── execution
├── deployment
├── simulation
└── provenance obligations

The parser records structure.

Semantic analysis determines:

- applicability;
- precedence;
- conflicts;
- authorization;
- satisfiability;
- interaction with effects;
- interaction with capabilities;
- interaction with resources.

---

31. Policy Ownership Boundary

The core grammar must not create separate policy languages for:

security policy
quantum policy
resource policy
AI policy
deployment policy
network policy
simulation policy

Instead:

core policy syntax
        ↓
semantic policy model
        ↓
specialized policy consumers

This prevents policy fragmentation.

---

32. Requirement / Constraint / Preference / Policy Separation

These concepts must remain distinct.

Concept| Meaning
Requirement| Must be satisfied
Constraint| Restricts legal realization
Preference| Preferred but not necessarily mandatory
Hint| Non-binding implementation guidance
Permission| Explicitly allows an action
Prohibition| Explicitly forbids an action
Policy| Structured rule governing how these concepts interact
Capability| What an environment can provide
Resource| What may be consumed or required

The semantic layer must preserve these distinctions.

---

33. Integration With Resources

Resource syntax belongs to:

grammar/resources/

Core grammar provides the universal structural boundary.

The architecture is:

source requirement
        ↓
resource requirement
        ↓
capability analysis
        ↓
resource feasibility
        ↓
negotiation
        ↓
execution planning
        ↓
target realization

Core grammar must not perform any of these downstream decisions.

---

34. Integration With Effects

Effects belong to:

grammar/effects/

A core requirement or policy may reference an effect-related property.

For example, a policy may conceptually prohibit an effect:

forbid effect("network");

The core grammar preserves the policy structure.

The effects subsystem determines:

- effect identity;
- effect propagation;
- effect checking;
- effect compatibility.

---

35. Integration With Contracts

Contracts belong primarily to:

grammar/validation/

Core constructs may be consumed by contracts.

The conceptual relationship is:

requires
ensures
invariant
assume
guarantee
property
assert

The core grammar must not duplicate the contract semantic model.

---

36. Integration With Security

Security owns:

authorization
trust
sandboxing
security policies
security capabilities
audit
security provenance

Core policies can provide the common syntax.

The security subsystem interprets security-specific policy semantics.

For example:

forbid capability("native.execute");
forbid effect("network");

is a policy structure.

Whether those restrictions are enforceable is a semantic/runtime responsibility.

---

37. Integration With AI and Reasoning

The universal core must allow semantic subsystems to express requirements and policies for:

- inference;
- deduction;
- reasoning;
- learning;
- adaptation;
- uncertainty;
- provenance;
- explanations;
- evidence;
- agents.

Core must not introduce AI-specific grammar for every application.

The architecture is:

generic core construct
        ↓
AI semantic consumer
        ↓
AI semantic model
        ↓
canonical IR / execution

---

38. Integration With Quantum Computing

Core grammar may express quantum requirements such as:

requires quantum::measurement;
requires quantum::dynamic_control;
requires quantum::mid_circuit_measurement;

But core does not own:

- quantum operations;
- qubits;
- circuit semantics;
- physical qubits;
- topology;
- routing;
- calibration;
- QEC;
- ZQN.

Quantum syntax belongs under:

grammar/quantum/

The canonical semantic quantum boundary remains:

quantum::ir

The pipeline is:

Zamani source
    ↓
domain-neutral AST
    ↓
semantic quantum model
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
QEC/resilience
    ↓
ZQN
    ↓
HAL
    ↓
target

---

39. Integration With HDL and Hardware

Core constructs can express:

- hardware requirements;
- capabilities;
- constraints;
- preferences;
- policies;
- provenance.

The core grammar must not establish universal:

- bus widths;
- register widths;
- device counts;
- memory sizes;
- topology sizes;
- pipeline depths;
- clock counts.

Those are source semantics only when explicitly declared by a particular program/type or target profile.

---

40. Integration With Distributed Computing

Core requirements and policies must work equally for:

single-process
multithreaded
multinode
cluster
distributed
heterogeneous
cloud
future distributed systems

No core grammar rule may require a fixed node count.

Distributed semantics belong to:

grammar/distributed/
grammar/concurrency/
grammar/networking/

---

41. Integration With Interoperability

Core names, paths, attributes, metadata, requirements, capabilities, and policies may be consumed by:

grammar/interoperability/

FFI/ABI syntax itself belongs there.

Core must not contain:

- ABI layouts;
- calling conventions;
- foreign object formats;
- platform-specific linkage rules.

---

42. Integration With Dialects

A dialect may extend Zamani syntax.

However:

core
    ↓
dialect

must remain one-way.

A dialect must not redefine the meaning of a core construct.

Dialect extensions must declare:

- identity;
- version;
- ownership;
- compatibility;
- lexical dependencies;
- grammar dependencies;
- AST representation;
- semantic representation;
- diagnostics;
- tests.

---

43. AST Contract

Core grammar rules map into a domain-neutral AST.

Core AST nodes may conceptually include:

CompilationUnit
SourceUnit
Identifier
Name
QualifiedName
Path
Attribute
Annotation
Metadata
VersionDeclaration
CapabilityReference
Requirement
Constraint
Preference
Hint
Policy

The concrete Rust AST remains owned by the frontend AST subsystem.

No ".g4" file may define Rust AST structures.

---

44. Domain-Neutral AST Requirement

The AST must not contain target implementation details such as:

- vendor topology;
- physical qubit maps;
- physical routing;
- QEC schedules;
- calibration records;
- machine-specific register layouts;
- backend instruction selection.

Those belong downstream.

This is essential to POCO-REAF.

---

45. Semantic Contract

Semantic analysis consumes core AST nodes and resolves:

- names;
- scopes;
- versions;
- capabilities;
- requirements;
- constraints;
- preferences;
- policies;
- conflicts;
- contracts;
- effects;
- resources;
- provenance;
- compatibility.

The parser must not attempt these tasks.

---

46. IR Contract

Core constructs normally do not become executable operations directly.

Instead:

core syntax
    ↓
core AST
    ↓
semantic model
    ↓
requirements/capabilities/resources/policies
    ↓
execution/compilation planning
    ↓
canonical IR

A requirement may influence compilation without becoming an instruction.

A policy may constrain optimization without becoming an instruction.

A provenance record may accompany a transformation without becoming an instruction.

---

47. Provenance Contract

Core AST nodes must preserve enough source information for downstream provenance.

At minimum, the frontend must preserve:

- source span;
- structural ordering;
- source identity where available;
- relevant attributes;
- declaration relationship.

Downstream systems may attach:

- evidence;
- verification;
- transformation history;
- decisions;
- compiler version;
- semantic version;
- target realization;
- reproducibility information.

---

48. Determinism

Core parsing must be deterministic.

Parsing must depend on:

- source text;
- canonical lexer;
- grammar;
- parser configuration;
- explicitly selected dialect configuration.

Parsing must not depend on:

- wall-clock time;
- randomness;
- hardware;
- filesystem state;
- network state;
- environment variables;
- runtime state;
- target availability.

No semantic action should be embedded in core ANTLR grammar.

---

49. Safety

The core grammar must contain:

- no embedded Rust;
- no "unsafe";
- no filesystem access;
- no network access;
- no hardware discovery;
- no environment inspection;
- no secret access;
- no runtime execution;
- no random behavior.

The Rust frontend consuming the generated grammar must remain compatible with:

Rust 2021
Rust 1.97.1

and safe Rust.

---

50. Lexer Boundary

The canonical lexer is:

grammar/antlr/ZamaniLexer.g4

The core parser grammars must consume its token vocabulary.

Core parser files must not redefine lexical tokens.

The lexical architecture remains:

ZamaniLexer.g4
        ↓
lexer composition
        ↓
canonical tokens
        ↓
core parser grammars

If a new keyword is genuinely required, it must be introduced through the canonical lexical ownership system rather than privately by a core parser.

---

51. Keyword Policy

Do not add a keyword simply because a semantic feature exists.

Prefer identifiers and qualified names when a keyword is unnecessary.

A keyword should exist only when:

1. it improves syntactic disambiguation;
2. it is part of the normative language surface;
3. reserving it is justified;
4. compatibility impact is understood;
5. lexical tests exist.

This prevents keyword explosion.

---

52. Integration With the Existing Lexer

The current repository deliberately places the canonical lexical hierarchy behind:

grammar/antlr/ZamaniLexer.g4

Core grammar files must consume that vocabulary rather than defining parallel lexical systems.

The lexer must remain independent of:

- quantum hardware;
- CPU hardware;
- GPU hardware;
- AI model catalogues;
- vendor identifiers;
- runtime configuration.

---

53. Grammar Dependency Rules

Every ".g4" file in "grammar/core/" must document:

DEPENDS_ON:
EXPORTS:
CONSUMED_BY:
AST_OWNER:
SEMANTIC_OWNER:
IR_OWNER:
TEST_OWNER:
SPEC_OWNER:

Example:

DEPENDS_ON:
  grammar/antlr/ZamaniLexer.g4
  grammar/core/names.g4

EXPORTS:
  policyDeclaration
  policyRule
  policyScope

CONSUMED_BY:
  grammar/security/
  grammar/resources/
  grammar/execution/
  grammar/compile/
  grammar/validation/

AST_OWNER:
  domain-neutral frontend AST

SEMANTIC_OWNER:
  semantic policy model

IR_OWNER:
  semantic planning / policy consumers

TEST_OWNER:
  grammar/tests/policies/

SPEC_OWNER:
  grammar/spec/policies.md

This contract is mandatory.

---

54. No Circular Grammar Ownership

Core grammars must not create circular semantic ownership.

For example:

core → quantum → core

must not become a semantic dependency.

If quantum syntax needs a core construct, quantum consumes core.

If core needs quantum-specific meaning, that meaning must be represented abstractly and resolved downstream.

---

55. Dependency Direction

The preferred direction is:

lexer
  ↓
core
  ↓
universal grammar
  ↓
domain grammar
  ↓
AST
  ↓
semantic model
  ↓
IR
  ↓
backend

Not:

backend
  ↓
core grammar

and not:

hardware
  ↓
source grammar

---

56. Cross-Domain Composition

A Zamani source file may contain multiple computational domains.

For example:

classical computation
+
tensor computation
+
reasoning
+
quantum computation
+
HDL intent
+
distributed execution
+
security policy
+
provenance

The core grammar must permit such composition through shared structures.

There must not be separate incompatible root languages.

---

57. Policy and Capability Composition

The common semantic relationship is:

program
  ↓
operation
  ↓
effects
  ↓
capabilities
  ↓
requirements
  ↓
constraints
  ↓
policies
  ↓
contracts
  ↓
provenance
  ↓
semantic realization

This provides the foundation for portable execution.

---

58. Resource Independence

Core grammar must not know how much of a resource exists.

For example:

requires memory >= required_memory;

does not mean that core knows the available memory.

Similarly:

requires qubits >= required_qubits;

does not mean that core knows the target's qubit count.

The semantic/resource system determines feasibility.

---

59. Capability Negotiation

The architecture must distinguish:

required capability
        ↓
available capabilities
        ↓
compatibility
        ↓
negotiation
        ↓
legal realization

Negotiation is not parser responsibility.

The parser preserves intent.

---

60. Fallbacks

Fallback syntax may be expressed through policies or execution-specific grammar.

The core grammar must preserve the distinction between:

required
preferred
optional
fallback

A fallback may be used only where semantic equivalence is established.

The compiler must never silently replace a computation with a different computation merely because the preferred target is unavailable.

---

61. Adaptation

Adaptation belongs semantically to execution/AI/runtime systems.

Core policy syntax may express the policy governing adaptation.

The architecture must be:

adaptation request
        ↓
policy
        ↓
authorization
        ↓
capability/resource analysis
        ↓
validation
        ↓
provenance
        ↓
authorized adaptation

Unrestricted self-modifying behavior must not be implied by generic policy syntax.

---

62. Reproducibility

Core metadata/versioning/policy structures must support reproducibility declarations.

Reproducibility may include:

- source identity;
- language version;
- grammar version;
- semantic version;
- dependency identity;
- dialect identity;
- policy identity;
- provenance;
- deterministic-execution requirements.

Reproducibility does not require identical physical hardware.

---

63. Compatibility

Core grammar changes must follow compatibility rules.

Every change must classify itself as:

compatible
additive
behavior-changing
syntax-breaking
deprecated
removed
experimental

A token rename must not silently alter AST identity.

A grammar restructuring must preserve semantic meaning where compatibility requires it.

---

64. Deprecation

Deprecated constructs must have:

- canonical replacement;
- compatibility policy;
- diagnostic behavior;
- migration documentation;
- tests.

Deprecated syntax must not be silently reinterpreted as unrelated syntax.

---

65. Error Handling

Core grammar diagnostics must distinguish:

Lexical error

Owned by lexer.

Syntax error

Owned by parser.

Structural error

Produced after parsing.

Semantic error

Produced by semantic analysis.

Capability error

Produced during capability analysis.

Resource error

Produced during resource analysis.

Policy conflict

Produced during policy analysis.

Target incompatibility

Produced during target negotiation.

Do not encode downstream semantic errors as parser hacks.

---

66. Negative Parsing Rule

A parser success must mean:

«The source is structurally valid according to the active grammar.»

It must not mean:

«The source is executable on the current machine.»

Those are different questions.

---

67. Testing Architecture

Core grammar requires tests at several levels:

grammar/tests/
├── lexical/
├── parser/
├── ast/
├── semantic/
├── contracts/
├── capabilities/
├── resources/
├── policies/
├── provenance/
├── compatibility/
├── scalability/
├── portability/
├── negative/
└── boundary/

Core-specific tests should be placed in the appropriate existing test hierarchy rather than creating duplicate test systems.

---

68. Required Positive Tests

At minimum test:

- minimal compilation unit;
- source units;
- names;
- qualified names;
- paths;
- attributes;
- annotations;
- metadata;
- version declarations;
- capability references;
- requirements;
- constraints;
- preferences;
- hints;
- policies;
- combinations of all of the above;
- nested structures;
- cross-domain source.

---

69. Required Negative Tests

Test:

- incomplete source;
- missing delimiters;
- invalid qualification;
- malformed attributes;
- malformed metadata;
- invalid version structure;
- malformed requirement;
- malformed constraint;
- malformed policy;
- duplicate syntax where prohibited;
- invalid modifier ordering;
- trailing unparsed input.

---

70. Boundary Tests

Core must be tested with combinations such as:

classical + quantum
classical + HDL
quantum + hybrid
AI + quantum
AI + distributed
security + networking
resources + quantum
resources + hardware
contracts + policies
provenance + compilation
simulation + execution
metaprogramming + core
interoperability + policies

---

71. Scalability Tests

Scalability tests must use increasing values of:

- source length;
- declaration count;
- module count;
- qualification depth;
- requirement count;
- policy count;
- constraint count;
- expression depth;
- domain composition.

The tests must verify that no artificial grammar ceiling exists.

They must not encode a "maximum supported" number as a language rule.

---

72. Portability Tests

The same source-level program should be tested against different target profiles.

The test must distinguish:

source remains valid

from:

target can currently realize it

A target incompatibility must not require rewriting the source unless the program's semantics themselves require a different implementation.

---

73. Determinism Tests

Given identical:

source
grammar version
lexer configuration
dialect configuration

the parser must produce structurally equivalent results.

The parser must not vary because of:

- hardware;
- time;
- randomness;
- environment;
- filesystem;
- network;
- runtime state.

---

74. Cross-Domain Integration Test

At least one canonical integration source must combine:

classical computation
tensor/data computation
reasoning
learning
adaptation
quantum operation
measurement
concurrency
distributed execution
resource requirements
capabilities
effects
contracts
provenance
policy
simulation
hardware intent

The expected pipeline is:

source
 ↓
lexer
 ↓
parser
 ↓
domain-neutral AST
 ↓
structural validation
 ↓
name/type analysis
 ↓
effect analysis
 ↓
capability analysis
 ↓
resource analysis
 ↓
contract analysis
 ↓
policy analysis
 ↓
provenance
 ↓
canonical semantic model
 ↓
classical IR / quantum::ir / domain semantic representations
 ↓
execution planning

---

75. Integration With "grammar/antlr/ZamaniParser.g4"

"ZamaniParser.g4" is the parser composition authority.

Core files must be integrated there through the existing parser hierarchy.

"grammar/Zamani.g4" must remain a thin complete-language composition root.

It must not be modified merely because an individual core feature is added if the parser composition layer can absorb that feature.

The integration chain should remain:

Zamani.g4
    ↓
ZamaniParser.g4
    ↓
core grammar hierarchy

The root should not become a list of every feature file.

---

76. Integration With "grammar/antlr/ZamaniLexer.g4"

Core parser grammars consume the canonical token vocabulary.

If a new core keyword is required:

core semantic requirement
        ↓
lexical specification
        ↓
canonical lexer
        ↓
parser grammar
        ↓
AST

Do not add private lexer tokens to individual core parser files.

---

77. Integration With "grammar/specification/"

Every stable core construct must have normative specification coverage.

At minimum:

grammar/specification/
├── core.md
├── names.md
├── requirements.md
├── constraints.md
├── capabilities.md
├── policies.md
└── versioning.md

Exact filenames may follow the repository's established specification organization.

The key requirement is one authoritative semantic specification for each feature family.

---

78. Integration With "grammar/spec/"

Machine-oriented contracts should document:

- AST shape;
- semantic invariants;
- capability model;
- resource model;
- policy model;
- compatibility;
- provenance;
- POCO-REAF requirements.

Recommended specifications include:

grammar/spec/resources.md
grammar/spec/effects.md
grammar/spec/policies.md
grammar/spec/provenance.md
grammar/spec/poco-reaf.md

---

79. Integration With AST

Core grammar changes are not complete when ANTLR accepts syntax.

The feature is complete only when:

grammar
 ↓
parse tree
 ↓
AST

has a defined and tested representation.

No grammar construct should be accepted indefinitely without a documented AST mapping.

---

80. Integration With Semantic Analysis

Every core construct must have a semantic owner.

For example:

qualifiedName
    → name resolution

requirement
    → requirement semantic model

capability
    → capability semantic model

constraint
    → constraint semantic model

policy
    → policy semantic model

version
    → compatibility semantic model

No semantic ambiguity may be left to "whatever the backend does."

---

81. Integration With IR

Core grammar does not define a universal executable IR.

Its constructs become semantic inputs to the compiler.

For example:

requirement
    ↓
capability/resource analysis
    ↓
execution plan

rather than:

requirement
    ↓
machine instruction

---

82. Quantum IR Boundary

The canonical quantum boundary remains:

quantum::ir

Core must never introduce another quantum frontend IR merely to represent core requirements.

If a core policy affects quantum execution:

core policy
 ↓
semantic policy
 ↓
quantum semantic analysis
 ↓
quantum::ir

---

83. HDL Boundary

If a core requirement affects HDL:

core requirement
 ↓
semantic requirement
 ↓
HDL semantic model
 ↓
synthesis/simulation/verification pipeline

The core grammar must not become an HDL backend.

---

84. Hardware Boundary

Hardware realization remains downstream.

Core syntax may express intent such as:

requires capability("hardware.description");

but must not determine:

which FPGA
which ASIC
which CPU
which GPU
which accelerator

unless a target-specific deployment layer explicitly requires such a constraint.

---

85. Resource Negotiation Boundary

The compiler/runtime may perform:

requirement
 ↓
capability discovery
 ↓
candidate realization
 ↓
constraint filtering
 ↓
preference ranking
 ↓
policy evaluation
 ↓
execution plan

Core grammar participates only in expressing the initial source intent.

---

86. File Completion Contract

Every core ".g4" file is considered DONE only when all of the following exist:

[ ] Purpose documented
[ ] Ownership documented
[ ] Non-ownership documented
[ ] Dependencies documented
[ ] Exports documented
[ ] Consumers documented
[ ] Lexer dependencies documented
[ ] AST contract documented
[ ] Semantic contract documented
[ ] Type interaction documented
[ ] Effect interaction documented
[ ] Capability interaction documented
[ ] Resource interaction documented
[ ] Contract interaction documented
[ ] Policy interaction documented
[ ] Provenance interaction documented
[ ] IR destination documented
[ ] Quantum boundary documented where applicable
[ ] HDL boundary documented where applicable
[ ] Backend boundary documented
[ ] Diagnostics documented
[ ] Positive tests exist
[ ] Negative tests exist
[ ] Boundary tests exist
[ ] Scalability tests exist
[ ] Determinism tests exist
[ ] Compatibility behavior documented
[ ] Hard-coding audit completed

---

87. Independence-First Development Rule

Core files must be developed in dependency order.

Recommended order:

1. identifiers.g4
2. names.g4
3. qualified-names.g4
4. paths.g4
5. attributes.g4
6. annotations.g4
7. modifiers.g4
8. metadata.g4
9. versioning.g4
10. capabilities.g4
11. requirements.g4
12. constraints.g4
13. preferences.g4
14. hints.g4
15. policies.g4
16. source-unit.g4
17. compilation-unit.g4

This prevents higher-level files from becoming unstable because foundational syntax is not defined.

---

88. "Done Once" File Rule

A completed core file must not depend on undocumented assumptions about future files.

Before marking a file complete, its README/header must already specify:

upstream dependency
downstream consumer
AST mapping
semantic owner
IR owner
test owner
spec owner
compatibility contract
extension mechanism

If another feature is added later, that feature should consume the existing public contract rather than forcing unrelated changes.

If the public contract itself must change, that is a deliberate architectural change and must be versioned.

---

89. Extension Rule

New computational domains should normally add:

new semantic namespace
new domain grammar
new capabilities
new resource requirements
new policies
new tests

rather than modify core syntax.

For example, a future accelerator should be representable through:

accelerator::new_capability

without adding a universal keyword solely for that accelerator.

---

90. No Application Keyword Explosion

The core grammar must not accumulate application-specific keywords.

Application concepts such as:

- computer vision operations;
- sentiment analysis;
- robotics algorithms;
- payment systems;
- administrative operations;
- legal workflows;
- blockchain applications;
- VR/AR features;
- specific AI architectures;

should normally be represented through:

libraries
dialects
data models
semantic capabilities
policies
services
applications

The core language remains computationally universal.

---

91. Generic Reasoning Integration

Reasoning constructs such as:

infer
deduce
reason

belong to the reasoning subsystem.

Core provides the reusable infrastructure:

names
attributes
requirements
capabilities
policies
provenance

Reasoning semantics remain outside core.

---

92. Knowledge Integration

Knowledge operations such as:

assert
retract
query

belong to knowledge/data/reasoning subsystems.

Core provides the universal structures they can consume.

Knowledge representation must not require core grammar changes for every new knowledge domain.

---

93. Learning and Adaptation Integration

Learning and adaptation are semantic operations.

They may consume:

requirements
capabilities
effects
resources
policies
contracts
provenance

Core must not define individual learning algorithms.

A future algorithm should not require a new universal keyword.

---

94. Uncertainty Integration

Uncertainty and probability belong to the appropriate type/AI/data semantic layers.

Core metadata, attributes, policies, contracts, and provenance may carry uncertainty-related information.

Core must not prescribe a particular probability implementation.

---

95. Explainability and Evidence

Evidence, explanations, and decision records are cross-domain concerns.

They may explain:

- AI decisions;
- compiler transformations;
- resource decisions;
- optimization choices;
- quantum transformations;
- hardware mapping;
- security decisions.

Core provides structural support through:

metadata
attributes
policies
provenance references

The explanation engine itself is not part of core grammar.

---

96. Actor and Agent Integration

The core grammar must not create a second actor system.

Agent semantics should use:

concurrency/actors
concurrency/messages
distributed/

An AI agent is a semantic role built on the existing concurrency architecture.

---

97. Simulation Integration

Simulation belongs to:

grammar/execution/

Core may express:

- simulation policy;
- simulation requirements;
- simulation metadata;
- simulation provenance.

Simulation remains an execution strategy, not a second source language.

---

98. Metaprogramming Integration

Reflection and metaprogramming belong to:

grammar/metaprogramming/
grammar/macros/

Core attributes and names may be consumed by these systems.

Metaprogramming must respect:

- type safety;
- effects;
- capabilities;
- policies;
- provenance;
- reproducibility.

---

99. Interoperability Integration

FFI and ABI belong to:

grammar/interoperability/

Core provides:

- names;
- attributes;
- metadata;
- capabilities;
- effects;
- policies.

An external call may require a capability and produce a foreign/native effect.

---

100. Compatibility Integration

Compatibility belongs to:

grammar/compatibility/

Core versioning supplies the source-level representation.

Compatibility analysis determines whether two versions can interact.

---

101. Repository Integration Matrix

The final relationship should be:

Core feature| Primary consumer
Compilation unit| parser root
Source unit| AST/frontend
Identifier| names/declarations/types
Name| resolution
Qualified name| modules/namespaces
Path| modules/interoperability
Attribute| semantic/tooling systems
Annotation| semantic/tooling systems
Modifier| declarations/domain grammars
Metadata| provenance/tooling
Version| compatibility
Capability| resource/semantic analysis
Requirement| resources/semantic analysis
Constraint| validation/resources/execution
Preference| planning
Hint| optimization/planning
Policy| security/resources/execution/validation
Pragmas| explicitly owned compiler features

---

102. Canonical Semantic Flow

All core constructs should ultimately participate in:

SOURCE
  ↓
LEXER
  ↓
PARSER
  ↓
DOMAIN-NEUTRAL AST
  ↓
STRUCTURAL VALIDATION
  ↓
NAME RESOLUTION
  ↓
TYPE ANALYSIS
  ↓
EFFECT ANALYSIS
  ↓
CAPABILITY ANALYSIS
  ↓
RESOURCE ANALYSIS
  ↓
CONTRACT ANALYSIS
  ↓
POLICY ANALYSIS
  ↓
PROVENANCE
  ↓
CANONICAL SEMANTIC MODEL
  ↓
IR
  ↓
OPTIMIZATION
  ↓
LOWERING
  ↓
ROUTING / SCHEDULING
  ↓
RESILIENCE
  ↓
TARGET REALIZATION

No core grammar file may skip directly from syntax to physical execution.

---

103. Current Repository Integration Requirements

The current repository architecture should retain:

grammar/Zamani.g4
    ↓
ZamaniParser
    ↓
core/domain parser hierarchy

and:

grammar/antlr/ZamaniLexer.g4
    ↓
canonical lexical hierarchy

The existing root grammar is already designed as a thin composition boundary. It should remain that way. "Current Zamani grammar root" (https://reference-url-citation.invalid/2)

The core README therefore does not instruct developers to add every new core file directly to "grammar/Zamani.g4".

Integration belongs at the appropriate parser-composition layer.

---

104. Rust Integration Contract

The grammar layer must remain independent of Rust implementation details.

The generated frontend must support:

Rust 2021
Rust 1.97.1

with:

no unsafe Rust

No ".g4" file may contain Rust actions merely to compensate for missing semantic architecture.

Semantic behavior belongs in Rust frontend/semantic modules.

---

105. Build Contract

Grammar changes must be validated through the repository's ANTLR generation/build process.

Required validation:

ANTLR grammar generation
        ↓
Rust generated sources
        ↓
Rust compiler
        ↓
AST integration
        ↓
parser tests
        ↓
semantic tests

A grammar that generates successfully but cannot be consumed by the Rust frontend is not production-ready.

---

106. Important Manifest Integration Note

The current repository manifest should use a valid single Rust version declaration.

The intended baseline is:

rust-version = "1.97.1"

The manifest must not contain prose such as:

rust-version = "1.97" or "1.97.1"

because that is not a valid Cargo value.

This is a repository-level integration correction, not a grammar rule.

---

107. Hard-Coding Audit

Before a core file is marked complete, search it and its dependencies for:

maximum
minimum
limit
capacity
count
width
depth
size
device
qubit
CPU
GPU
FPGA
node
thread
register
tensor
network

Each occurrence must be classified as:

semantic program value
documentation example
target profile
implementation resource
or prohibited universal limit

A universal hardware capacity must never be hidden in prose, parser alternatives, semantic predicates, or helper rules.

---

108. Open-World Audit

For every new core construct ask:

1. Does adding a new domain require changing this grammar?
2. Does adding a new capability require changing this grammar?
3. Does adding a new accelerator require changing this grammar?
4. Does adding a new quantum operation require changing this grammar?
5. Does adding a new vendor require changing this grammar?
6. Does adding a new deployment environment require changing this grammar?

The desired answer is:

No

unless the new feature changes the actual universal language syntax.

---

109. Grammar Quality Rules

Every core grammar file must:

- use parser grammar ownership correctly;
- use lowercase parser rule names;
- consume canonical lexer tokens;
- avoid embedded actions;
- avoid semantic predicates unless explicitly justified by the language architecture;
- avoid duplicate rules;
- avoid duplicate lexical definitions;
- avoid target-specific syntax;
- avoid finite domain catalogues;
- avoid hidden resource limits;
- avoid backend logic;
- preserve source structure;
- remain deterministic.

ANTLR grammar structure must follow the standard grammar model and naming rules.

---

110. Review Checklist

A core grammar change is reviewable only when the change includes:

[ ] specification
[ ] grammar
[ ] AST mapping
[ ] semantic owner
[ ] integration owner
[ ] tests
[ ] negative tests
[ ] boundary tests
[ ] scalability test
[ ] compatibility classification
[ ] hard-coding audit
[ ] provenance implications
[ ] effect implications
[ ] capability implications
[ ] resource implications
[ ] policy implications

---

111. Production-Readiness Definition

"grammar/core/" is production-ready when:

[ ] Every core feature has one owner.
[ ] No core concept has competing grammar authorities.
[ ] The lexer has one canonical authority.
[ ] The parser has one canonical composition hierarchy.
[ ] The AST is domain-neutral.
[ ] Semantic ownership is explicit.
[ ] Requirements and capabilities remain distinct.
[ ] Constraints and preferences remain distinct.
[ ] Policies remain distinct from enforcement.
[ ] Resources remain distinct from capabilities.
[ ] Effects remain distinct from permissions.
[ ] Provenance remains available across transformations.
[ ] Quantum semantics converge on quantum::ir.
[ ] Hardware realization remains downstream.
[ ] No universal hardware ceilings exist.
[ ] No finite domain catalogue controls extensibility.
[ ] New capabilities can be added without changing core syntax.
[ ] New domains can be added without creating a new root language.
[ ] Parser behavior is deterministic.
[ ] Rust integration uses safe Rust.
[ ] Rust 1.97.1 is supported.
[ ] Positive tests exist.
[ ] Negative tests exist.
[ ] Boundary tests exist.
[ ] Scalability tests exist.
[ ] Portability tests exist.
[ ] Compatibility tests exist.
[ ] Cross-domain tests exist.
[ ] Documentation and specification agree.

---

112. Final Architecture

The core grammar should ultimately provide this universal foundation:

                    ZAMANI CORE
                         │
        ┌────────────────┼────────────────┐
        │                │                │
       Names          Metadata         Versioning
        │                │                │
        └────────────────┼────────────────┘
                         │
        ┌────────────────┼────────────────┐
        │                │                │
  Capabilities      Requirements      Constraints
        │                │                │
        └────────────────┼────────────────┘
                         │
              Preferences / Hints
                         │
                       Policies
                         │
                      Contracts
                         │
                       Effects
                         │
                    Provenance
                         │
                         ▼
              DOMAIN-NEUTRAL AST
                         │
                         ▼
                SEMANTIC MODEL
                         │
       ┌─────────────────┼─────────────────┐
       │                 │                 │
   Classical          Quantum             HDL
       │                 │                 │
       │            quantum::ir            │
       │                 │                 │
       └─────────────────┼─────────────────┘
                         │
                  Optimization
                         │
                Lowering / Planning
                         │
             Routing / Scheduling
                         │
                Resilience / QEC
                         │
                    ZQN / HAL
                         │
                         ▼
       CPU / GPU / FPGA / ASIC / QPU
       Embedded / HPC / Cluster / Cloud
       Distributed / Accelerator / Future

The central invariant is:

«Zamani source expresses portable computation and its semantic obligations; downstream systems determine how that computation is realized with the resources and capabilities actually available.»

That is the foundation required for scaling from the smallest meaningful computation to arbitrarily large systems without making today's hardware the permanent limit of tomorrow's language.

---

113. Final Ownership Rule

The most important rule for every future contributor is:

ONE CONCEPT
    ↓
ONE SYNTACTIC OWNER
    ↓
ONE AST CONTRACT
    ↓
ONE SEMANTIC OWNER
    ↓
ONE CANONICAL IR DESTINATION
    ↓
MANY VALID TARGET REALIZATIONS

Do not solve a new feature by adding another grammar authority.

Do not solve a hardware problem by hard-coding a language limit.

Do not solve a semantic problem by adding parser hacks.

Do not solve a backend problem by contaminating the AST.

Do not solve an application problem by adding universal keywords.

Do not solve a scalability problem by replacing an unbounded abstraction with a fixed catalogue.

The core grammar exists to make the rest of Zamani extensible without repeatedly reopening the language foundation.

---

114. Definition of DONE for This README

This README is complete when it serves as the permanent architectural contract for "grammar/core/" and answers, before implementation begins:

What does each core file own?
What does each core file not own?
What does each file depend on?
Who consumes each file?
What AST does each construct produce?
Who owns its semantics?
Where does it influence IR?
How does it interact with effects?
How does it interact with capabilities?
How does it interact with resources?
How does it interact with contracts?
How does it interact with policies?
How is provenance preserved?
How does it interact with quantum::ir?
How does it interact with HDL?
How is it integrated with the parser?
How is it integrated with the lexer?
How is it tested?
How is it scaled?
How is it kept portable?
How is compatibility handled?
How is hard-coding prevented?
What makes the file DONE?

Once those contracts are established, individual core grammar files can be implemented independently and integrated through stable interfaces rather than repeatedly rewritten because an unrelated domain was added later.
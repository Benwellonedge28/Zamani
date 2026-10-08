Zamani Policy Semantic Specification

Path: "grammar/spec/policies.md"
Specification role: Normative machine-oriented semantic contract for universal policies
Status: Production-target normative specification
Language: Zamani
Grammar technology: ANTLR4
Implementation baseline: Rust 1.97 or later, Rust 2021
Rust safety: Production Rust MUST use safe Rust; "unsafe" MUST NOT be used
Architecture: Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)
Scalability: No artificial language-level capacity ceiling
Canonical quantum boundary: "quantum::ir"
Canonical parser composition root: "grammar/Zamani.g4"
Canonical policy grammar owner: "grammar/policies/policy.g4"

---

1. Purpose

This specification defines the normative semantic model for policies in the Zamani programming language.

A policy is a declarative set of governing rules that constrains, permits, requires, prefers, prohibits, selects, adapts, or otherwise qualifies the realization of already-defined program semantics.

A policy does not redefine what a program means.

A policy governs how a semantically valid program may be analyzed, realized, adapted, executed, deployed, simulated, secured, or otherwise operated within an explicitly defined policy scope.

The policy system exists to provide a common semantic mechanism across:

- classical computation;
- quantum computation;
- HDL;
- hardware;
- accelerators;
- AI/ML;
- data;
- distributed computation;
- networking;
- concurrency;
- simulation;
- interoperability;
- security;
- deployment;
- adaptive execution;
- reproducible execution;
- future computational domains.

The policy system MUST remain domain-neutral.

A domain MAY define policy adapters, but MUST consume the common policy model rather than creating a competing policy language.

---

2. Architectural Authority

The policy architecture follows the authority model defined by:

grammar/DESIGN.md
        |
        v
grammar/specification/
        |
        v
grammar/spec/
        |
        +--> grammar/policies/
        |
        v
domain-neutral AST
        |
        v
semantic policy model

The authorities are:

Concern| Authority
Repository architecture| "grammar/DESIGN.md"
Human language specification| "grammar/specification/"
Machine semantic contracts| "grammar/spec/"
Policy source syntax| "grammar/policies/policy.g4"
Policy payload syntax| dedicated files under "grammar/policies/"
Lexer| "grammar/antlr/ZamaniLexer.g4"
Token registry| "grammar/lexer/"
Root grammar composition| "grammar/Zamani.g4"
AST| existing domain-neutral AST implementation
Semantic policy model| compiler semantic layer
Canonical classical representation| Classical IR
Canonical quantum representation| "quantum::ir"
Physical realization| compiler/runtime/HAL/target layers

No policy file may silently become a second authority.

---

3. Core Definition

A policy is:

Policy =
    identity
  + scope
  + applicability
  + ordered rules
  + semantic predicates
  + obligations
  + permissions
  + prohibitions
  + requirements
  + constraints
  + preferences
  + fallbacks
  + metadata
  + provenance

A policy is declarative.

It does not directly execute an operation.

The semantic pipeline is:

Policy source
    |
    v
lexer
    |
    v
policy parser
    |
    v
domain-neutral AST
    |
    v
structural validation
    |
    +--> types
    +--> effects
    +--> capabilities
    +--> resources
    +--> contracts
    +--> policy structure
    +--> provenance
    |
    v
semantic policy model
    |
    v
policy evaluation / normalization
    |
    v
execution or compilation planning
    |
    +--> classical IR
    +--> quantum::ir
    +--> HDL/hardware representation
    +--> distributed representation
    +--> other domain representations
    |
    v
optimization
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
resilience / recovery / QEC
    |
    v
ZQN
    |
    v
HAL
    |
    v
target realization

Policy syntax MUST never directly produce target instructions.

---

4. Fundamental Invariants

The following invariants are mandatory.

4.1 Policy does not redefine program meaning

A policy MUST NOT silently transform:

program meaning A

into:

program meaning B

merely because target resources, capabilities, or preferences differ.

If a requested realization cannot satisfy the program's semantic requirements, the implementation MUST either:

1. find another semantically equivalent valid realization;
2. use an explicitly permitted semantic adaptation;
3. use an explicitly declared fallback;
4. report infeasibility.

It MUST NOT silently change semantics.

---

4.2 Policy is not a hardware description

Policies MUST NOT require fixed universal physical identifiers such as:

CPU 0
GPU 0
QPU 0
node 0
device 0
memory bank 0
FPGA 0

unless such identifiers are explicitly supplied by a target-specific dialect or deployment environment.

Portable policy MUST instead express semantic properties such as:

capability("gpu.compute")
capability("quantum.measurement")
capability("tensor.compute")
resource("memory")
resource("bandwidth")
topology(...)

---

4.3 Policy is not resource allocation

A policy may express:

requires memory >= required_memory;
prefer capability("tensor.compute");

but policy evaluation MUST NOT itself allocate physical memory, devices, nodes, QPUs, FPGA fabric, or other resources.

Resource allocation belongs to downstream realization.

---

4.4 Policy is not authorization

A policy statement such as:

allow action;

does not by itself establish that the caller is authorized to perform that action.

Authorization requires the security subsystem to validate:

- identity;
- authority;
- credentials;
- capability;
- trust;
- scope;
- policy;
- security state.

Therefore:

policy permission

and:

security authorization

are separate semantic concepts.

---

4.5 Policy is not capability

A policy may require a capability:

requires capability("quantum.measurement");

but the policy does not create the capability.

Capability availability is established by the target/environment.

Therefore:

policy requirement
    !=
capability availability

---

4.6 Policy is not an effect

A policy may constrain effects:

forbid effect("network");

but does not itself perform the effect.

Effect meaning remains owned by:

grammar/spec/effects.md
grammar/effects/

---

4.7 Policy is not a contract

Policies may reference contracts and may impose policy obligations concerning contracts.

They MUST NOT redefine the semantic meaning of:

- "requires";
- "ensures";
- "invariant";
- "assume";
- "guarantee";
- "property";
- assertions;
- proofs.

Contract semantics remain owned by the validation/contract subsystem.

---

5. Open-World Policy Vocabulary

The policy model MUST be open-world.

The grammar MUST NOT enumerate every possible:

- resource;
- capability;
- device;
- processor;
- accelerator;
- quantum operation;
- network protocol;
- AI model;
- deployment platform;
- execution strategy;
- optimization;
- simulation technique;
- future hardware feature.

Policy identifiers SHOULD therefore be represented by qualified names and semantic identifiers.

For example:

capability("quantum.measurement")
capability("gpu.compute")
capability("tensor.compute")
capability("accelerator.matrix")
resource("memory")
resource("bandwidth")
execution::deterministic
deployment::portable
quantum::fault_tolerant

Future names MUST be addable without modifying the universal policy grammar.

---

6. Policy Identity

Every named policy has a stable semantic identity.

Conceptually:

PolicyId =
    qualified_name
    + optional version

A policy identity MUST NOT depend on:

- source-file position;
- physical target;
- machine address;
- process identifier;
- runtime thread identifier.

A policy MAY have aliases through a compatibility layer, but aliases MUST preserve semantic identity.

---

7. Policy Scope

A policy MUST have a semantic scope.

Possible scopes include:

- program;
- module;
- package;
- declaration;
- function;
- block;
- expression;
- operation;
- execution;
- simulation;
- deployment;
- resource;
- capability;
- domain;
- policy composition;
- implementation-defined scope.

The universal policy grammar MUST remain open to additional scopes.

A scope MUST be represented semantically rather than by hard-coded finite scope categories.

---

8. Applicability

A policy may have an applicability condition.

Conceptually:

policy P applies_when condition

The condition MUST be evaluated only after the relevant semantic information is available.

Applicability may depend on:

- program properties;
- types;
- effects;
- capabilities;
- resources;
- execution context;
- target properties;
- deployment context;
- policy context;
- contracts;
- provenance;
- execution state.

Policy applicability MUST NOT require the parser to inspect physical hardware.

---

9. Policy Rule

A policy consists of zero or more ordered rules.

Conceptually:

when condition => action;

A rule contains:

condition
action
optional clauses
source provenance

The parser records structure.

Semantic analysis determines whether the rule is meaningful and applicable.

---

10. Policy Rule Ordering

Policy order MUST be preserved through parsing.

Semantic normalization MAY canonicalize policies, but it MUST preserve enough provenance to reconstruct:

- source order;
- source spans;
- original policy identity;
- rule origin;
- composition origin.

Order MUST NOT be used accidentally as an implicit priority unless the policy model explicitly establishes ordering semantics.

---

11. Requirements

Requirements express conditions that MUST be satisfied.

Examples:

requires capability("quantum.measurement");
requires capability("tensor.compute");
requires memory >= required_memory;
requires topology(required_topology);

Requirements are hard constraints.

If a requirement cannot be satisfied, the policy is not satisfied.

A compiler MUST NOT convert an unsatisfied requirement into a preference.

A runtime MUST NOT silently ignore an unsatisfied requirement.

---

12. Requirements and Resources

Resource semantics are owned by:

grammar/spec/resources.md
grammar/resources/

Policy semantics only attach requirements to policy scope.

The separation is:

policy
    |
    +--> resource requirement
              |
              v
        resource analysis
              |
              v
        feasibility

Policy syntax MUST NOT duplicate resource quantity semantics.

---

13. Capabilities

Capabilities describe properties that a realization may provide.

Policies MAY:

- require capabilities;
- prefer capabilities;
- prohibit capabilities;
- constrain their use;
- condition behavior upon capabilities.

For example:

requires capability("quantum.measurement");
prefer capability("gpu.compute");
forbid capability("native.execute");

The policy subsystem MUST NOT invent whether a target actually possesses a capability.

That determination belongs to capability discovery and target analysis.

---

14. Constraints

Constraints restrict acceptable realizations.

Examples include:

constrain latency <= required_latency;
constrain topology(required_topology);
constrain effect("network");

A constraint is stronger than a preference.

A constraint that cannot be satisfied makes the corresponding realization invalid.

Constraints MUST NOT silently become preferences.

---

15. Preferences

Preferences express desirable but non-mandatory choices.

Example:

prefer execution::deterministic;
prefer capability("tensor.compute");
prefer resource("bandwidth");

Preferences:

- MUST NOT be treated as requirements;
- MUST NOT make an otherwise valid realization invalid merely because the preference cannot be satisfied;
- MAY be ranked;
- MAY participate in optimization;
- MUST NOT change program semantics.

If multiple preferences conflict, the semantic model MUST retain the conflict unless an explicit precedence mechanism resolves it.

---

16. Hints

A hint is advisory implementation information.

A hint:

- MUST NOT alter observable program semantics;
- MUST NOT override a requirement;
- MUST NOT override a prohibition;
- MUST NOT override a contract;
- MUST NOT create a capability;
- MUST NOT create a resource.

Hints MAY be ignored by an implementation.

---

17. Permissions

A policy permission declares that an action is permitted under the policy.

Conceptually:

allow action;
permit action;

Permission semantics are contextual.

A permission MUST NOT imply:

- identity authentication;
- security authorization;
- capability availability;
- resource availability;
- successful execution.

A permission can only be effective when all other semantic conditions are satisfied.

---

18. Prohibitions

A prohibition states that a behavior MUST NOT occur within the applicable policy scope.

Conceptually:

forbid effect("network");
deny capability("native.execute");

A prohibition has higher semantic force than a preference.

A prohibited action MUST NOT be selected merely because it would otherwise be efficient.

---

19. Permission/Prohibition Conflict

If an applicable policy simultaneously produces:

allow X

and:

forbid X

the policy system MUST NOT resolve the conflict nondeterministically.

The semantic analyzer MUST report a policy conflict unless an explicit precedence mechanism establishes a valid resolution.

No implementation may resolve such a conflict merely by:

- source order;
- hash-map order;
- iteration order;
- thread scheduling;
- backend order.

---

20. Obligations

An obligation requires an action or property to occur.

An obligation is stronger than a preference.

Conceptually:

oblige provenance;
oblige verification;
oblige reproducibility;

Obligations MUST be represented in the semantic policy model even when enforcement is deferred.

An implementation MUST distinguish:

required

from:

preferred

and:

informational

---

21. Fallbacks

A fallback defines an alternative realization when the primary policy path cannot be satisfied.

A fallback MUST be explicit.

Example conceptual structure:

fallback {
    primary: capability("quantum.compute");
    alternative: capability("classical.simulation");
}

A fallback MUST satisfy all applicable rules.

A fallback MUST NOT silently alter semantic meaning.

A fallback that changes semantic guarantees MUST be rejected unless the program explicitly permits that adaptation.

---

22. Fallback Ordering

Fallbacks have deterministic order.

The semantic representation MUST preserve:

fallback priority
fallback condition
fallback target intent
fallback provenance

Fallback selection MUST be deterministic for a deterministic policy context.

It MUST NOT depend on unordered data structures.

---

23. Negotiation

Policies MAY declare negotiation intent.

Negotiation can concern:

- capabilities;
- resources;
- execution strategies;
- deployment;
- simulation;
- interoperability;
- distributed execution;
- hardware realization.

Policy negotiation does not itself perform negotiation.

The downstream negotiation system MUST consume the normalized semantic policy.

---

24. Selection

Policy selection expresses a preference or constraint over semantically valid alternatives.

Selection MUST NOT mean:

select physical device X

unless the source explicitly belongs to a target/deployment dialect where physical identity is meaningful.

Portable selection should instead describe properties:

select capability("tensor.compute");
select topology(required_topology);
select execution::deterministic;

---

25. Determinism

A policy may require deterministic behavior.

Determinism applies to the semantic behavior of the relevant scope.

A deterministic policy MUST consider all relevant sources of nondeterminism, including where applicable:

- randomness;
- scheduling;
- concurrency races;
- distributed ordering;
- network ordering;
- hardware nondeterminism;
- quantum measurement;
- adaptive decisions;
- external state;
- time;
- environment observation.

A policy declaring deterministic execution does not magically make nondeterministic computation deterministic.

The semantic analyzer MUST either establish compatibility or report that the policy cannot be satisfied.

---

26. Reproducibility

Reproducibility is distinct from determinism.

A reproducibility policy may require preservation of:

- source identity;
- compiler identity;
- compiler version;
- grammar version;
- semantic version;
- dependency identity;
- configuration;
- policy set;
- resource requirements;
- capability assumptions;
- random seeds where applicable;
- execution metadata;
- provenance;
- artifact identity.

Reproducibility MUST be evaluated using the provenance and compatibility systems.

---

27. Provenance

Policies MAY require provenance.

Examples:

require provenance;
require decision provenance;
require transformation provenance;

Policy provenance MUST integrate with:

grammar/spec/provenance.md

The provenance system MUST record, where applicable:

policy identity
policy version
rule identity
rule source span
policy composition
policy decision
inputs
evidence
resource context
capability context
contract context
effect context
compiler transformation
realization decision

A policy decision MUST remain explainable after lowering.

---

28. Policy Decisions

Policy evaluation produces a semantic decision.

A decision SHOULD conceptually contain:

decision_id
policy_id
scope
applicable_rules
satisfied_requirements
violated_requirements
active_constraints
active_prohibitions
active_permissions
active_preferences
selected_fallback
selected_adaptation
evidence
provenance
diagnostics

The actual Rust representation belongs to the semantic implementation rather than this grammar specification.

---

29. Policy Decision Classes

The semantic policy system SHOULD distinguish at least:

Applicable
NotApplicable
Satisfied
Unsatisfied
Permitted
Prohibited
Conflicted
Infeasible
RequiresAdaptation
RequiresFallback
Indeterminate

Implementations MAY add additional states.

No finite enumeration should be treated as the ultimate universe of possible policy states.

---

30. Conflict Resolution

Policy conflicts MUST be explicit.

The semantic analyzer MUST detect conflicts among:

- requirements;
- constraints;
- permissions;
- prohibitions;
- obligations;
- preferences;
- fallbacks;
- adaptations;
- contracts;
- effects;
- capabilities;
- resource conditions.

A conforming implementation MUST NOT resolve conflicts through accidental implementation ordering.

Explicit resolution mechanisms MAY include:

- policy precedence;
- policy scope;
- explicit priority;
- specificity;
- explicit override;
- composition semantics.

When no valid resolution exists, compilation MUST fail with a policy conflict diagnostic.

---

31. Priority

Priority is semantic metadata.

A higher-priority policy MAY override a lower-priority policy only when:

1. the policy system explicitly permits such override;
2. the scopes overlap;
3. the policy types are override-compatible;
4. the resulting policy remains internally consistent;
5. provenance records the override.

Priority MUST NOT permit:

- violation of language semantics;
- violation of mandatory security constraints;
- violation of contracts;
- use of unavailable capabilities;
- use of unavailable resources.

---

32. Policy Composition

Policies MAY be composed.

Composition operations MAY include:

include
extend
compose
override
exclude

Composition MUST produce a semantic policy graph or equivalent normalized representation.

Composition MUST preserve provenance.

For every derived rule, the implementation MUST be able to identify its contributing policies.

---

33. Cyclic Policy Composition

Policy composition MUST detect cycles.

For example:

A extends B
B extends C
C extends A

MUST NOT result in unbounded semantic recursion.

The implementation MUST diagnose the cycle.

Policy composition depth MUST NOT be hard-coded to an arbitrary universal value.

Resource-aware or implementation-defined limits MAY exist, but they are implementation constraints rather than language semantics.

---

34. Policy Expressions

Policy expressions reuse the canonical Zamani expression system.

The policy grammar MUST NOT create an independent expression language.

Therefore:

policyExpression

must ultimately resolve to the repository's canonical expression representation.

Policy expressions MAY reference:

- values;
- types where semantically permitted;
- resources;
- capabilities;
- effects;
- contracts;
- execution state;
- provenance;
- policy variables;
- qualified names;
- dialect-defined semantic values.

---

35. Policy Variables

Policy evaluation MAY bind variables from:

- program metadata;
- semantic analysis;
- resource discovery;
- capability discovery;
- execution context;
- deployment context;
- policy parameters.

Bindings MUST be explicitly scoped.

An unbound policy variable MUST produce a semantic diagnostic where the value is required.

---

36. Policy Metadata

Policy metadata is open-ended.

Metadata MAY identify:

- version;
- author;
- domain;
- applicability;
- provenance;
- compatibility;
- documentation;
- migration information;
- policy class;
- dialect;
- extension.

Metadata MUST NOT become an implicit semantic rule merely because it exists.

Unknown metadata MUST be handled according to the repository's extension/compatibility rules.

---

37. Resource Integration

The policy system integrates with:

grammar/spec/resources.md
grammar/resources/

The relationship is:

Policy
   |
   +--> requirement
   +--> constraint
   +--> preference
   |
   v
Resource semantic model
   |
   v
Resource feasibility

A policy MUST NOT define physical resource capacities.

The language MUST NOT contain universal constants such as:

MAX_MEMORY
MAX_CPUS
MAX_GPUS
MAX_QUBITS
MAX_FPGAS
MAX_NODES
MAX_THREADS
MAX_DEVICE_COUNT

No equivalent hidden limit may be introduced under another name.

---

38. Capability Integration

Capability semantics integrate with the capability subsystem.

A policy may express:

requires capability("quantum.measurement");
requires capability("gpu.compute");
requires capability("tensor.compute");

Capability discovery happens downstream.

The policy analyzer MUST distinguish:

required capability

from:

available capability

and:

authorized capability

These are separate facts.

---

39. Effect Integration

Policy evaluation MUST integrate with:

grammar/spec/effects.md

A policy may constrain effects:

forbid effect("network");
forbid effect("native");
permit effect("simulation");

The semantic analyzer MUST compare:

declared effects
actual effects
policy-allowed effects

An effect prohibited by an applicable policy MUST make the corresponding execution path invalid.

---

40. Contract Integration

Policies MAY reference contract obligations.

Policy analysis MUST integrate with:

requires
ensures
invariant
assume
guarantee
property
assertion

The relationship is:

program
  |
  +--> contracts
  |
  +--> policies
         |
         v
     policy validation

Policies MUST NOT weaken a mandatory program contract unless the language specification explicitly defines such a transformation and preserves semantic correctness.

---

41. Security Integration

Security policy is a specialization of the universal policy model.

Security semantics remain owned by:

grammar/security/
grammar/spec/security.md

The universal policy layer provides:

- rules;
- scopes;
- permissions;
- prohibitions;
- applicability;
- provenance;
- composition.

The security subsystem provides:

- authentication;
- authorization;
- trust;
- identity;
- credentials;
- security capabilities;
- enforcement;
- isolation;
- sandbox implementation.

This separation is mandatory.

---

42. Sandbox Integration

A policy MAY constrain sandbox behavior.

Possible constraints include:

forbid effect("network");
forbid effect("native");
forbid effect("foreign");
forbid effect("reflection");
forbid effect("code_generation");

Sandbox implementation belongs to:

grammar/security/
grammar/execution/

The policy layer declares intent only.

---

43. Adaptation

Policy-controlled adaptation is permitted.

However:

adaptation != unrestricted self-modifying code

Adaptation MUST be governed by:

- policy;
- authorization;
- capabilities;
- effects;
- resources;
- contracts;
- provenance;
- compatibility;
- validation.

A valid adaptation pipeline is:

detect condition
      |
      v
evaluate policy
      |
      v
verify authorization
      |
      v
verify capabilities/resources
      |
      v
verify contracts
      |
      v
perform permitted adaptation
      |
      v
record provenance
      |
      v
validate resulting semantic state

An adaptation MUST NOT bypass policy analysis.

---

44. Simulation

Policies MAY select or constrain simulation.

Simulation is an execution strategy.

It may represent:

- classical computation;
- quantum computation;
- HDL;
- hardware;
- AI/ML;
- distributed computation;
- fault behavior;
- performance behavior;
- resource behavior.

Simulation MUST preserve the semantics required by the applicable policy.

A simulator is not automatically equivalent to a physical realization.

---

45. Quantum Policy Integration

Quantum policies MUST remain domain-neutral at the universal policy layer.

Examples:

requires capability("quantum.measurement");
prefer capability("quantum.error_correction");
constrain quantum::topology(required_topology);

The policy system MUST NOT enumerate quantum gates.

It MUST NOT define:

H
X
Y
Z
CNOT

as universal policy constructs.

Quantum operations belong to the quantum grammar and semantic model.

Quantum semantics eventually cross:

quantum semantic model
        |
        v
quantum::ir

Policy metadata MUST survive this boundary where relevant.

---

46. Classical Policy Integration

Classical policies MAY constrain:

- numerical execution;
- memory behavior;
- concurrency;
- vectorization;
- tensor computation;
- accelerator usage;
- determinism;
- reproducibility;
- numerical precision;
- execution strategy.

The policy system MUST NOT define CPU instruction sets as universal policy syntax.

Target-specific instruction selection belongs downstream.

---

47. HDL and Hardware Integration

Hardware policies MAY constrain:

- timing intent;
- latency;
- throughput;
- power;
- resource classes;
- reliability;
- simulation;
- synthesis requirements;
- verification requirements;
- deployment conditions.

They MUST NOT impose universal fixed hardware sizes.

For example, a policy may require:

requires resource("memory") >= required_memory;

but MUST NOT require a universal fixed memory size.

---

48. Distributed Integration

Policies MAY govern distributed execution:

prefer distributed::resilient;
requires capability("distributed.execution");
constrain topology(required_topology);

The policy model MUST NOT require a fixed universal node count.

The number of nodes is a realization property.

---

49. Networking Integration

Networking policy MAY govern:

- network effects;
- protocol requirements;
- endpoint constraints;
- transport requirements;
- security requirements;
- bandwidth;
- latency;
- reliability;
- isolation.

Network policy MUST integrate with:

grammar/networking/
grammar/spec/networking.md
grammar/spec/effects.md
grammar/spec/resources.md
grammar/spec/security.md

Network routing itself is not owned by policy syntax.

---

50. AI, Learning, and Reasoning Integration

Policies MAY govern semantic operations involving:

- inference;
- reasoning;
- learning;
- adaptation;
- knowledge;
- uncertainty;
- agents;
- explainability;
- evidence;
- provenance.

For example:

require provenance;
require evidence;
forbid effect("uncontrolled_adaptation");
prefer execution::reproducible;

AI policy MUST remain a specialization of universal policy semantics.

The policy subsystem MUST NOT become an AI-only policy language.

---

51. Interoperability

Policies MAY constrain FFI/ABI and external interoperability.

Examples:

forbid effect("native");
forbid effect("foreign");
require capability("abi.compatible");

FFI/ABI semantics remain owned by:

grammar/interoperability/
grammar/spec/interoperability.md

Policy syntax MUST NOT duplicate ABI declarations.

---

52. Deterministic Policy Evaluation

Policy evaluation itself MUST be deterministic when its inputs are deterministic.

Given identical:

policy
program semantic state
resource facts
capability facts
contract facts
effect facts
execution context
provenance context

the policy evaluator MUST produce the same semantic decision.

Policy evaluation MUST NOT depend on:

- hash-map iteration order;
- thread timing;
- unspecified filesystem ordering;
- unspecified network ordering;
- random selection.

If nondeterministic policy evaluation is explicitly required, that nondeterminism MUST be represented semantically.

---

53. Policy Normalization

Before downstream consumption, policies SHOULD be normalized.

Normalization MAY include:

- canonical qualified names;
- duplicate elimination;
- composition expansion;
- scope resolution;
- precedence resolution;
- requirement canonicalization;
- preference normalization;
- provenance attachment.

Normalization MUST preserve semantic equivalence.

Normalization MUST preserve source provenance.

---

54. Policy Equivalence

Two policies are semantically equivalent only when they produce equivalent policy decisions for all relevant valid contexts.

Textual equality is not sufficient.

Ordering differences are not automatically irrelevant.

Metadata differences are not necessarily semantic.

Implementations MUST NOT claim semantic equivalence merely because two policies parse successfully.

---

55. Policy Monotonicity

Where applicable, adding a requirement or prohibition MUST NOT make an invalid realization valid.

Adding a preference MUST NOT make an otherwise invalid realization valid.

Adding a prohibition MUST NOT create a new permission.

Adding a requirement MUST NOT remove a program requirement.

This establishes predictable policy composition.

---

56. Policy Non-Interference

A policy MUST NOT alter unrelated program semantics.

For example, a policy concerning:

resource("bandwidth")

MUST NOT silently change:

- integer semantics;
- quantum state semantics;
- HDL signal semantics;
- function return types;
- memory ownership semantics.

Policy influence must remain within its declared semantic scope.

---

57. Policy and POCO-REAF

Policy is one of the mechanisms enabling POCO-REAF.

The intended model is:

portable source
      |
      v
portable semantic intent
      |
      +--> requirements
      +--> capabilities
      +--> resources
      +--> constraints
      +--> preferences
      +--> policies
      +--> contracts
      +--> effects
      +--> provenance
      |
      v
target-independent semantic representation
      |
      v
target realization

The source program remains unchanged while the realization may vary.

For example, the same semantic program may be realized on:

tiny embedded target
CPU
multicore CPU
GPU
FPGA
ASIC
accelerator
QPU
simulator
HPC system
cluster
distributed system
cloud
future computational substrate

provided the target can satisfy the program's semantic requirements.

---

58. Scaling Model

Zamani policies MUST scale with the computation rather than imposing a language-level maximum.

The policy model therefore MUST NOT define:

maximum policy rules
maximum policy clauses
maximum requirements
maximum resources
maximum capabilities
maximum targets
maximum nodes
maximum devices
maximum qubits
maximum processors

The grammar may use repetition constructs such as:

*
+

where appropriate.

Implementation resource limits MAY exist, but they MUST be implementation/runtime limits rather than language semantics.

An implementation MUST distinguish:

implementation resource exhaustion

from:

language-level semantic rejection

---

59. "Infinity" and Feasibility

The language does not require literal infinite physical resources.

"Scalable to infinity" means:

«The policy model imposes no artificial finite computational-size ceiling; realizations may scale to arbitrarily large feasible sizes subject to actual available resources, semantic requirements, implementation limits, and physical laws.»

A target with insufficient resources MUST report infeasibility rather than causing the language to acquire a hidden universal capacity limit.

---

60. Target Independence

Policy source MUST NOT depend on a physical target unless target dependence is explicitly declared.

Portable:

requires capability("tensor.compute");

Non-portable target-specific policy:

requires deployment::vendor_specific_target(...);

Target-specific constructs belong to dialects or deployment specifications.

The universal policy model remains target-neutral.

---

61. Specialization

A generic policy MAY be specialized for a target.

Specialization MUST:

1. preserve program semantics;
2. preserve mandatory policy requirements;
3. preserve mandatory contracts;
4. respect prohibitions;
5. respect security constraints;
6. preserve provenance;
7. identify target-specific assumptions.

Specialization MUST NOT rewrite the source policy invisibly.

---

62. Policy Adaptation and Semantic Equivalence

An adaptive policy may select among equivalent realizations.

For example:

quantum realization
        |
        | unavailable
        v
classical simulation

This is valid only if:

1. the source program permits simulation;
2. the simulation satisfies the relevant contract;
3. the effects remain compatible;
4. policy permits the fallback;
5. provenance records the adaptation.

A compiler MUST NOT assume that simulation and physical execution are automatically semantically equivalent.

---

63. Policy and Optimization

Optimization MAY use policy preferences.

Optimization MUST NOT violate:

- requirements;
- constraints;
- prohibitions;
- contracts;
- effect restrictions;
- security restrictions.

A preference may guide optimization but MUST NOT override mandatory semantics.

---

64. Policy and Scheduling

Scheduling consumes normalized policy information.

Policy syntax does not schedule.

The scheduler MAY use:

- preferences;
- resource constraints;
- capability requirements;
- latency requirements;
- reliability requirements;
- determinism requirements;
- topology constraints.

The scheduler MUST preserve all mandatory semantic obligations.

---

65. Policy and Routing

Routing consumes policy constraints and requirements.

Policy syntax does not perform routing.

Quantum routing, network routing, accelerator placement, and distributed placement belong to their respective realization systems.

---

66. Policy and Resilience

Policies MAY specify resilience intent:

prefer recovery;
require resilient_execution;
fallback simulation;
retry permitted;
escalation required;

Resilience semantics integrate with the existing execution/resilience model.

Policy MUST NOT duplicate runtime recovery-state definitions.

---

67. Policy and QEC

Quantum policies MAY constrain QEC-related requirements.

They MUST NOT implement QEC.

For example:

requires capability("quantum.error_correction");
prefer quantum::fault_tolerant;

may participate in planning.

Actual:

- code selection;
- syndrome extraction;
- decoding;
- correction;
- routing;
- QPU execution

remain outside policy syntax.

---

68. Policy and Provenance Across IR

Policy decisions MUST remain traceable across:

AST
  |
  v
semantic model
  |
  +--> Classical IR
  |
  +--> quantum::ir
  |
  +--> HDL/hardware representation
  |
  v
lowering
  |
  v
target realization

A policy decision that affects an IR transformation SHOULD carry provenance identifying:

- originating policy;
- originating rule;
- decision;
- evidence;
- semantic inputs;
- transformation;
- resulting constraint or property.

---

69. AST Contract

The domain-neutral AST MUST represent policy semantics without embedding backend objects.

The AST MUST be capable of representing at least:

PolicyDeclaration
PolicyIdentity
PolicyScope
PolicyApplicability
PolicyRule
PolicyCondition
PolicyAction
PolicyRequirement
PolicyConstraint
PolicyCapability
PolicyResource
PolicyPermission
PolicyProhibition
PolicyObligation
PolicyPreference
PolicyFallback
PolicySelection
PolicyNegotiation
PolicyAdaptation
PolicySimulation
PolicyDeterminism
PolicyReproducibility
PolicyComposition
PolicyMetadata
PolicyReference
PolicyProvenance

Exact Rust type names belong to the AST implementation.

---

70. AST Ownership

Policy AST nodes MUST be domain-neutral.

They MUST NOT contain:

GpuHandle
QpuHandle
CpuCoreId
FpgaInstance
VendorDeviceId
NetworkSocket
RawPointer

or equivalent physical backend objects.

Such information belongs downstream.

---

71. Source Span Preservation

Every policy declaration and policy rule MUST preserve source-location information.

At minimum, diagnostics MUST be capable of identifying:

- file;
- start location;
- end location;
- policy identity;
- rule identity where applicable.

Source spans MUST survive AST construction.

---

72. Semantic Policy Model

The semantic model MUST distinguish:

declared policy
applicable policy
normalized policy
evaluated policy
effective policy
policy decision

These are not interchangeable.

A parser result is not automatically an effective policy.

---

73. IR Contract

Policies SHOULD NOT become a separate universal IR.

Instead, policy information MUST be represented as semantic attributes, constraints, requirements, effects, metadata, or policy-derived decisions attached to the canonical semantic representation.

Where policy information affects classical computation, it may influence:

Classical IR

Where it affects quantum computation, it may influence:

quantum::ir

Where it affects HDL/hardware, it may influence the corresponding hardware representation.

A separate competing policy IR MUST NOT be introduced without architectural approval.

---

74. Quantum IR Contract

Policy information affecting quantum semantics MUST cross the quantum boundary through:

quantum::ir

Policy implementation MUST NOT create a second quantum policy IR.

The semantic information crossing the boundary MUST be sufficient to preserve:

- requirements;
- constraints;
- capabilities;
- effects;
- contracts;
- relevant provenance;
- adaptation/fallback intent.

---

75. Diagnostics

Policy diagnostics MUST distinguish syntax from semantics.

Syntax diagnostics

Examples:

malformed policy declaration
malformed policy rule
malformed condition
malformed policy expression
malformed composition

Semantic diagnostics

Examples:

unknown policy reference
conflicting policy rules
unsatisfied requirement
unavailable capability
unsatisfied resource requirement
conflicting prohibition and permission
invalid policy composition
cyclic composition
invalid fallback
forbidden effect
contract incompatibility
unauthorized adaptation
non-reproducible realization
incompatible target realization

The parser MUST NOT report semantic failures as parser errors.

---

76. Diagnostics Must Be Actionable

A diagnostic SHOULD include:

policy
rule
source span
problem
semantic context
related policy
required condition
actual condition
possible resolution
provenance

Diagnostics MUST avoid arbitrary implementation-dependent wording where stable diagnostic identifiers are available.

---

77. Error Recovery

Parser error recovery MUST NOT manufacture valid policy semantics from malformed input.

A recovered parse tree may be used for diagnostics, but it MUST NOT be treated as a valid semantic policy without successful semantic validation.

---

78. Versioning

Policies MUST support explicit semantic versioning where policy compatibility requires it.

Versioning MUST distinguish:

policy identity
policy syntax version
policy semantic version
language version
grammar version
AST version
IR version
dialect version
target capability version

Changing syntax does not automatically mean changing policy semantics.

---

79. Compatibility

Compatibility is governed by:

grammar/spec/compatibility.md
grammar/compatibility/

Deprecated policy syntax MUST NOT silently change semantic meaning.

Migration MUST be explicit and provenance-preserving.

Compatibility adapters MAY translate old policy syntax into the canonical policy model.

---

80. Legacy Policy Grammar Layers

The repository currently contains policy-related grammar layers under both:

grammar/core/
grammar/policies/

The production semantic authority MUST remain singular.

The canonical policy declaration/policy-specific grammar owner is:

grammar/policies/policy.g4

"grammar/core/policies.g4" MUST NOT become an independent competing semantic authority.

It MAY remain as:

- a compatibility adapter;
- a shared core composition layer;
- a legacy import surface;
- a migration bridge;

provided that it delegates to or remains semantically equivalent to the canonical policy model.

Likewise, "grammar/policies/policies.g4" MUST be treated as a composition/orchestration layer if retained. It MUST NOT redefine the semantics already owned by "policy.g4".

This avoids three competing policy languages.

---

81. Grammar Integration

The canonical policy grammar is:

grammar/policies/policy.g4

It owns policy declaration and universal policy structure.

Supporting files under:

grammar/policies/

own specialized payload syntax.

Examples include:

requirements.g4
constraints.g4
permissions.g4
prohibitions.g4
preferences.g4
fallbacks.g4
adaptation.g4
execution.g4
security.g4
resource.g4
deployment.g4
simulation.g4
provenance.g4

A supporting file MUST NOT define:

policyDeclaration
policyBody
policyMember

as competing authorities.

---

82. Domain Adapter Integration

Domain policy adapters MAY exist under:

grammar/ai/
grammar/quantum/
grammar/hdl/
grammar/hardware/
grammar/execution/
grammar/distributed/
grammar/networking/
grammar/security/

Each adapter MUST:

1. consume the common policy model;
2. remain domain-specific only in its payload;
3. avoid redefining universal policy syntax;
4. preserve provenance;
5. preserve requirements and constraints;
6. preserve policy identity;
7. avoid target-specific assumptions unless explicitly declared.

---

83. Lexer Integration

Policy keywords MUST use the canonical lexer:

grammar/antlr/ZamaniLexer.g4

and token registry:

grammar/lexer/

The policy grammar MUST NOT define lexer rules.

No policy subdirectory may create an independent token vocabulary.

---

84. Root Grammar Integration

The root composition remains:

grammar/Zamani.g4

The root grammar MUST compose policy syntax without reproducing policy semantics.

The root grammar MUST remain small and domain-neutral.

---

85. Semantic Analysis Order

Policy analysis SHOULD occur after enough structural information exists to evaluate:

types
effects
capabilities
resources
contracts
scope
provenance

A recommended sequence is:

parse
  |
  v
AST
  |
  v
name resolution
  |
  v
type analysis
  |
  v
effect analysis
  |
  v
resource analysis
  |
  v
capability analysis
  |
  v
contract analysis
  |
  v
policy normalization
  |
  v
policy evaluation
  |
  v
provenance
  |
  v
semantic realization planning

Implementations MAY optimize the order where semantic equivalence is preserved.

---

86. Policy Evaluation Inputs

Policy evaluation MUST consume explicit semantic inputs.

Possible inputs include:

program semantic model
scope
types
effects
requirements
resources
capabilities
contracts
execution context
deployment context
target facts
provenance
other applicable policies

Policy evaluation MUST NOT secretly query unrelated global state.

---

87. Policy Evaluation Outputs

Policy evaluation MUST produce structured results.

At minimum, downstream consumers need:

applicable rules
requirements
constraints
prohibitions
permissions
obligations
preferences
fallbacks
adaptation permissions
decision state
diagnostics
provenance

---

88. External State

If policy evaluation depends on external state, that dependency MUST be explicit.

Examples:

resource availability
capability discovery
deployment state
network state
hardware state
runtime state

The dependency MUST be represented in the evaluation context.

This is required for reproducibility and explainability.

---

89. Policy Caching

Implementations MAY cache normalized or evaluated policies.

A cache key MUST include every semantic input that can affect the decision.

A cache MUST NOT cause a stale policy decision to be reused after relevant semantic inputs change.

Caching is an implementation concern and MUST NOT alter policy semantics.

---

90. Policy Concurrency

Policy evaluation MAY occur concurrently.

Concurrent evaluation MUST remain deterministic with respect to equivalent inputs.

Shared policy state MUST use safe Rust synchronization.

No "unsafe" implementation is permitted.

---

91. Rust Safety Contract

Any Rust implementation associated with this specification MUST satisfy:

#![forbid(unsafe_code)]

or an equivalent project-wide prohibition.

Production implementation MUST NOT use:

- "unsafe";
- raw pointer manipulation;
- unsafe FFI wrappers;
- unsafe synchronization primitives.

FFI boundaries MUST be implemented through safe abstractions whose unsafety, if required by an external implementation, remains outside the policy semantic implementation and does not leak into policy evaluation.

---

92. Resource-Aware Scalability

Policy evaluation MUST be designed so that implementation complexity scales with actual policy/context size.

The language MUST NOT impose arbitrary fixed limits such as:

MAX_POLICY_RULES
MAX_POLICY_DEPTH
MAX_POLICY_REFERENCES
MAX_POLICY_TARGETS
MAX_POLICY_RESOURCES

where those limits would become semantic restrictions.

Implementations MAY protect themselves from denial-of-service or resource exhaustion, but such limits MUST be:

1. implementation/runtime controls;
2. configurable where appropriate;
3. diagnostically distinguishable from language semantics;
4. not represented as universal language capacities.

---

93. Memory and Streaming

Large policy sets MAY require streaming, incremental parsing, or external storage.

The semantic model MUST NOT assume that all policy information must reside in one fixed-size in-memory structure.

Where possible, implementations SHOULD support:

- incremental parsing;
- incremental semantic analysis;
- lazy policy loading;
- policy indexing;
- deterministic partitioning;
- bounded-memory processing.

These are implementation strategies and MUST preserve semantic equivalence.

---

94. Policy Security

Policy input MUST be treated as untrusted data unless its trust is established elsewhere.

A policy MUST NOT gain authority merely because it contains:

allow
permit
override
admin
root
trusted

Trust is established by the security/authorization subsystem.

---

95. Policy Provenance and Trust

Policy provenance SHOULD distinguish:

declared by source
derived by composition
generated by tooling
provided by environment
provided by target
provided by administrator/security system

A derived policy MUST NOT be represented as though it were directly declared by the program.

---

96. Explainability

A policy decision SHOULD be explainable.

An explanation should be able to answer:

Which policy applied?
Which rule applied?
Why did it apply?
Which requirement was satisfied?
Which requirement failed?
Which capability was required?
Which capability was available?
Which constraint was active?
Which prohibition blocked the action?
Which preference influenced selection?
Which fallback was selected?
Which adaptation occurred?
What evidence supported the decision?

This integrates with the universal provenance model.

---

97. Policy Auditability

Where auditing is required, the policy system SHOULD produce an auditable record containing:

policy identity
policy version
rule identity
evaluation context identity
decision
decision reason
evidence
provenance
resulting realization constraint

Audit storage and cryptographic integrity remain outside this semantic specification.

---

98. Policy and Reproducibility

A reproducible build SHOULD preserve the effective policy set.

The reproducibility record SHOULD identify:

policy identities
policy versions
policy composition
policy normalization
policy decisions
resource assumptions
capability assumptions
compiler version
grammar version
semantic version
IR version

A build MUST NOT claim full reproducibility if relevant policy inputs were unknown or uncontrolled.

---

99. Policy and Deployment

Deployment policies may constrain:

- environments;
- capabilities;
- resources;
- security;
- networking;
- reproducibility;
- availability;
- resilience;
- simulation;
- execution modes.

Deployment policy does not perform deployment.

Deployment realization belongs to the deployment subsystem.

---

100. Policy and Future Hardware

A new hardware class MUST be able to consume policy semantics without requiring modification to the universal policy model merely because the hardware is new.

For example, a future accelerator may expose:

capability("future.domain.compute");
resource("future.domain.capacity");

without requiring a new universal policy grammar.

This is mandatory for long-term scalability.

---

101. Vendor Neutrality

Universal policies MUST NOT require vendor-specific names.

Vendor-specific policy belongs in a dialect or deployment profile.

A vendor-specific extension MUST declare:

dialect identity
version
compatibility
semantic mapping
fallback behavior
provenance

---

102. Application Neutrality

The universal policy system MUST remain independent of application-specific domains.

Application features such as:

- vision;
- sentiment analysis;
- robotics;
- blockchain;
- VR/AR;
- payments;
- administration;
- legal workflows;

MUST NOT require universal policy keywords.

They SHOULD be implemented through:

libraries
dialects
capabilities
policies
services
application-level abstractions

The policy system supplies the universal semantic mechanism.

---

103. No Semantic Keyword Explosion

The policy model MUST prefer open-world semantic identifiers over adding a new keyword for every future concept.

For example:

capability("some.future.capability")
resource("some.future.resource")
effect("some.future.effect")
policy::some_extension

is preferable to adding a permanent universal keyword for every concept.

This preserves grammar stability.

---

104. Policy Extensions

Extensions MAY introduce new policy actions or metadata.

An extension MUST identify:

extension namespace
version
owner
semantic contract
grammar owner
AST representation
semantic representation
IR interaction
compatibility rules
tests

Unknown extensions MUST be diagnosed according to extension/compatibility rules.

---

105. Extension Isolation

An extension MUST NOT alter the meaning of an existing universal policy construct without an explicit language-versioned semantic change.

Extensions MAY add meaning.

They MUST NOT silently redefine:

requires
forbid
allow
prefer
fallback
policy

---

106. Policy Testing Requirements

Every production policy feature MUST have:

1. lexical tests where applicable;
2. parser tests;
3. AST tests;
4. semantic tests;
5. positive tests;
6. negative tests;
7. boundary tests;
8. conflict tests;
9. provenance tests;
10. compatibility tests;
11. deterministic evaluation tests;
12. scalability tests;
13. cross-domain tests.

---

107. Required Positive Tests

At minimum:

policy p {
    requires capability("quantum.measurement");
}

policy p {
    requires capability("gpu.compute");
    prefer execution::deterministic;
}

policy p {
    requires memory >= required_memory;
}

policy p {
    prefer capability("tensor.compute");
    fallback execution::simulation;
}

policy p {
    forbid effect("network");
}

policy p {
    require provenance;
    prefer execution::reproducible;
}

The exact surface syntax MUST follow the canonical current grammar; examples in this specification describe semantic intent.

---

108. Required Negative Tests

The implementation MUST reject or diagnose:

conflicting allow/forbid

unsatisfied mandatory requirement

unknown required capability

invalid fallback

cyclic policy composition

unauthorized adaptation

policy weakening a mandatory contract

policy violating an applicable prohibition

invalid target-specific policy in a universal context

---

109. Boundary Tests

Boundary tests MUST cover combinations such as:

policy + resource
policy + capability
policy + effect
policy + contract
policy + provenance
policy + security
policy + quantum
policy + classical
policy + HDL
policy + AI
policy + distributed
policy + networking
policy + simulation
policy + adaptation

---

110. Cross-Domain Integration Test

A production conformance suite MUST include a policy that simultaneously expresses:

resource requirement
capability requirement
effect restriction
contract requirement
provenance requirement
determinism preference
fallback
adaptation constraint
simulation permission
distributed execution constraint
quantum requirement
classical fallback

The test MUST verify:

source
  |
  v
AST
  |
  v
semantic policy model
  |
  v
validation
  |
  v
policy evaluation
  |
  +--> Classical IR
  |
  +--> quantum::ir
  |
  +--> hardware/HDL representation

without requiring separate policy languages.

---

111. Scalability Tests

The test suite MUST exercise policy sets that grow progressively in:

- rule count;
- composition depth;
- scope count;
- requirement count;
- capability count;
- resource references;
- provenance records;
- fallback alternatives;
- policy expressions.

Tests MUST not establish a universal maximum.

The objective is to demonstrate that policy semantics remain correct as implementation resources increase.

---

112. Determinism Tests

The same policy/context input MUST produce identical:

normalized policy
policy decision
diagnostic classification
policy provenance

across repeated runs.

Where ordering is semantically meaningful, that ordering MUST remain stable.

---

113. Concurrency Tests

Policy evaluation under concurrent compilation or execution planning MUST produce equivalent decisions to serial evaluation for equivalent contexts.

Race-dependent semantic decisions are prohibited.

---

114. Property Tests

Property-based tests SHOULD verify:

adding a preference does not create a requirement
adding a prohibition cannot create permission
adding an unsatisfied requirement cannot produce satisfaction
policy composition preserves provenance
normalization preserves semantics
reordering semantically unordered rules preserves semantics
ordered rules preserve required ordering

---

115. Fuzzing

Policy parsers SHOULD be fuzz-tested for:

- malformed declarations;
- deeply nested composition;
- large policy expressions;
- invalid UTF-8 handling at input boundaries;
- pathological qualified names;
- repeated clauses;
- ambiguous constructs;
- large metadata;
- cyclic references.

Fuzzing failures MUST NOT result in unsafe Rust.

---

116. Compatibility Tests

Compatibility tests MUST verify:

old policy syntax
    |
    v
compatibility adapter
    |
    v
canonical semantic policy

The resulting semantic policy MUST preserve the documented meaning.

---

117. Completion Criteria

"grammar/spec/policies.md" is complete when:

- the semantic policy model is explicitly defined;
- policy ownership is unambiguous;
- policy grammar ownership is unambiguous;
- requirements are delegated to resource/core semantics;
- capabilities are delegated to capability semantics;
- effects are delegated to effect semantics;
- contracts are delegated to contract semantics;
- security authorization remains security-owned;
- resource allocation remains resource-owned;
- target selection remains downstream;
- routing remains downstream;
- scheduling remains downstream;
- QEC remains quantum/runtime-owned;
- ZQN remains downstream;
- HAL remains downstream;
- provenance is preserved;
- AST requirements are defined;
- canonical IR interaction is defined;
- "quantum::ir" interaction is defined;
- policy conflicts are deterministic;
- policy composition is defined;
- policy precedence is defined;
- fallback semantics are defined;
- adaptation semantics are controlled;
- simulation semantics are defined;
- reproducibility semantics are defined;
- deterministic evaluation is defined;
- scalability rules are defined;
- no artificial hardware capacities exist;
- open-world vocabulary is defined;
- extension rules are defined;
- compatibility is defined;
- diagnostics are defined;
- testing requirements are defined;
- Rust implementation requirements are defined;
- "unsafe" is prohibited;
- cross-domain integration is defined.

---

118. File-Level Integration Contract

This file MUST be treated as complete independently of later implementation work.

Its direct semantic dependencies are:

grammar/DESIGN.md
grammar/spec/semantics.md
grammar/spec/resources.md
grammar/spec/effects.md
grammar/spec/provenance.md
grammar/spec/security.md
grammar/spec/portability.md
grammar/spec/compatibility.md
grammar/spec/type-system.md

Its syntax consumers are:

grammar/policies/policy.g4
grammar/policies/*.g4
grammar/expressions/policy.g4
grammar/statements/policy.g4
grammar/execution/policies.g4
grammar/security/policies.g4
grammar/ai/policies.g4
grammar/quantum/policies.g4
grammar/networking/policies.g4
grammar/distributed/policies.g4

Its downstream semantic consumers are:

AST
semantic validation
type analysis
effect analysis
resource analysis
capability analysis
contract analysis
security analysis
provenance
compiler planning
execution planning
deployment
simulation
classical IR
quantum::ir
HDL/hardware realization
routing
scheduling
resilience
ZQN
HAL

No downstream file needs to redefine policy semantics after this specification is accepted.

---

119. Required Repository Corrections

The following repository consistency corrections are required to make the policy architecture production-ready.

119.1 Establish one canonical policy grammar owner

Canonical:

grammar/policies/policy.g4

Existing policy composition files may remain, but they MUST delegate rather than redefine universal policy semantics.

---

119.2 Resolve "grammar/core/policies.g4"

"grammar/core/policies.g4" currently contains a substantial universal policy grammar.

It MUST be classified as one of:

compatibility adapter
composition adapter
legacy compatibility surface

or migrated so that "grammar/policies/policy.g4" becomes the sole owner.

It MUST NOT remain a competing semantic authority.

---

119.3 Resolve "grammar/policies/policies.g4"

If retained, this file MUST be an orchestrator.

It MUST NOT independently define another:

policyDeclaration
policyBody
policyMember

semantic authority.

---

119.4 Resolve policy AST duplication

All policy grammar files MUST map into one domain-neutral policy AST model.

No domain adapter may create an incompatible second policy AST.

---

119.5 Resolve policy provenance duplication

"grammar/policies/provenance.g4" owns policy-specific provenance payload syntax.

Universal provenance remains owned by:

grammar/spec/provenance.md

The semantic implementation MUST unify these into the universal provenance model.

---

119.6 Resolve policy/resource duplication

"grammar/policies/resource.g4" MUST remain a policy adapter.

It MUST NOT redefine:

resource identity
resource quantity
resource capacity
resource feasibility

Those remain resource-owned.

---

119.7 Resolve policy/security duplication

Policy permissions and prohibitions are generic policy concepts.

Security authorization remains owned by:

grammar/security/
grammar/spec/security.md

A generic policy permission MUST NOT be interpreted as a security credential.

---

120. Required Semantic Ownership Table

Feature| Policy owns?| Actual owner
Policy identity| Yes| Policy system
Policy scope| Yes| Policy system
Policy rules| Yes| Policy system
Requirements| References/attaches| Resource/core semantic system
Constraints| References/attaches| Constraint semantic system
Capabilities| References/attaches| Capability system
Resources| References/attaches| Resource system
Permissions| Yes, declaratively| Policy system
Security authorization| No| Security
Prohibitions| Yes, declaratively| Policy system
Preferences| Yes| Policy system
Fallbacks| Yes| Policy/execution semantics
Adaptation intent| Yes| Policy/execution semantics
Simulation intent| Yes| Execution
Determinism intent| Yes| Determinism/execution
Reproducibility intent| Yes| Reproducibility/provenance
Effects| References| Effect system
Contracts| References| Validation/contract system
Provenance| Attaches| Universal provenance
Hardware discovery| No| Hardware/target layer
Resource allocation| No| Resource/runtime layer
Routing| No| Routing
Scheduling| No| Scheduling
QEC| No| Quantum/runtime
ZQN| No| ZQN
HAL| No| HAL
Backend code generation| No| Backend
Vendor implementation| No| Dialect/backend

---

121. Final Policy Architecture

The production architecture is:

                    ZAMANI POLICY SYSTEM
                           |
                           v
                  grammar/policies/
                           |
                           v
                 canonical policy syntax
                           |
                           v
                    domain-neutral AST
                           |
                           v
                  structural validation
                           |
          +----------------+----------------+
          |                |                |
          v                v                v
        types           effects         contracts
          |                |                |
          +----------------+----------------+
                           |
             +-------------+-------------+
             |             |             |
             v             v             v
         resources    capabilities   provenance
             |             |             |
             +-------------+-------------+
                           |
                           v
                   policy normalization
                           |
                           v
                    policy evaluation
                           |
          +----------------+----------------+
          |                |                |
          v                v                v
       classical       quantum            HDL
          |                |                |
          v                v                v
   Classical IR       quantum::ir     hardware IR
          |                |                |
          +----------------+----------------+
                           |
                           v
                    optimization
                           |
                           v
                       lowering
                           |
                           v
                 routing / scheduling
                           |
                           v
                  resilience / recovery
                           |
                           v
                         ZQN
                           |
                           v
                         HAL
                           |
                           v
                 target realization

---

122. Final Normative Rules

The following rules are mandatory.

Rule 1

A policy expresses governing intent.

Rule 2

A policy does not itself execute anything.

Rule 3

A policy does not allocate resources.

Rule 4

A policy does not create capabilities.

Rule 5

A policy permission does not itself constitute authorization.

Rule 6

A policy preference is not a requirement.

Rule 7

A policy prohibition cannot be silently ignored.

Rule 8

A policy requirement cannot be silently weakened.

Rule 9

A fallback cannot silently change program semantics.

Rule 10

Adaptation must be explicitly governed.

Rule 11

Policy conflicts must be deterministic and diagnosable.

Rule 12

Policy evaluation must preserve provenance.

Rule 13

Policy syntax must remain open-world.

Rule 14

Universal policy syntax must not enumerate hardware.

Rule 15

Universal policy syntax must not encode fixed computational capacities.

Rule 16

Policies must remain independent of physical realization.

Rule 17

Domain policy adapters must consume the universal policy model.

Rule 18

There must be one semantic policy authority.

Rule 19

Policy information affecting quantum computation must integrate through "quantum::ir".

Rule 20

Policy information affecting classical computation must integrate through Classical IR.

Rule 21

Policy decisions must not silently change program meaning.

Rule 22

Implementation resource exhaustion must not be represented as a universal language capacity.

Rule 23

Production Rust implementing policy semantics must use Rust 1.97 or later, Rust 2021, and safe Rust only.

Rule 24

Production policy implementation MUST NOT use "unsafe".

Rule 25

The policy model must scale from the smallest feasible computation to arbitrarily large feasible computation without an artificial language-level ceiling.

---

123. Production Definition

The Zamani policy subsystem is production-ready when a policy can be traced completely through:

POLICY SOURCE
     |
     v
LEXER
     |
     v
PARSER
     |
     v
DOMAIN-NEUTRAL AST
     |
     v
NAME RESOLUTION
     |
     v
TYPE ANALYSIS
     |
     v
EFFECT ANALYSIS
     |
     v
RESOURCE ANALYSIS
     |
     v
CAPABILITY ANALYSIS
     |
     v
CONTRACT ANALYSIS
     |
     v
POLICY NORMALIZATION
     |
     v
POLICY EVALUATION
     |
     v
PROVENANCE
     |
     v
SEMANTIC MODEL
     |
     +------------------+
     |                  |
     v                  v
CLASSICAL IR       quantum::ir
     |                  |
     +------------------+
              |
              v
        OPTIMIZATION
              |
              v
          LOWERING
              |
              v
      ROUTING / SCHEDULING
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
              v
           TARGET

with:

- no duplicated policy authority;
- no fixed machine-size assumptions;
- no hidden capacity constants;
- no silent semantic changes;
- deterministic policy evaluation;
- explicit conflict handling;
- explicit fallback handling;
- controlled adaptation;
- resource/capability separation;
- security authorization separation;
- effect integration;
- contract integration;
- provenance preservation;
- classical IR integration;
- "quantum::ir" integration;
- HDL/hardware integration;
- distributed integration;
- simulation integration;
- interoperability integration;
- compatibility support;
- scalable implementation;
- complete positive/negative/boundary/scalability/conformance tests;
- Rust 1.97+;
- Rust 2021;
- no "unsafe".

This specification therefore defines policies as a universal semantic control plane over Zamani computation, rather than as another domain-specific programming language.

The fundamental invariant is:

PROGRAM MEANING
      |
      v
POLICY CONSTRAINTS
      |
      v
RESOURCE / CAPABILITY / EFFECT / CONTRACT VALIDATION
      |
      v
TARGET-INDEPENDENT REALIZATION
      |
      v
TARGET-SPECIFIC IMPLEMENTATION

That separation is mandatory for POCO-REAF.
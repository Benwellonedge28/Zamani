Zamani Policy Grammar

Status

Production architecture / policy-subsystem integration contract

Implementation baseline:

- Rust 2021
- Rust 1.97+
- Safe Rust only
- No "unsafe" Rust
- ANTLR4 grammar
- Target-independent source syntax
- Open-world semantic vocabulary
- POCO-REAF compliant

---

1. Purpose

"grammar/policies/" contains the policy-specific source grammar layer of Zamani.

Policies express declarative governing intent that may influence:

- requirements
- constraints
- capabilities
- resources
- permissions
- prohibitions
- preferences
- fallbacks
- selection
- negotiation
- execution
- adaptation
- security
- sandboxing
- simulation
- determinism
- reproducibility
- effects
- contracts
- provenance
- deployment
- distributed execution
- networking
- classical computation
- quantum computation
- hybrid computation
- HDL/hardware intent
- learning
- reasoning
- knowledge
- uncertainty
- agents
- interoperability
- future computational domains

The policy subsystem does not implement those systems.

It expresses portable policy intent which is subsequently interpreted by the semantic, compilation, execution, security, resource, capability, provenance, and domain subsystems.

---

2. Architectural Principle

The policy subsystem follows this architecture:

Zamani source
    |
    v
lexer
    |
    v
policy grammar
    |
    v
domain-neutral AST
    |
    v
structural validation
    |
    v
semantic policy model
    |
    +-------------------+
    |                   |
    v                   v
policy analysis    provenance
    |
    +---------+---------+---------+---------+
    |         |         |         |         |
    v         v         v         v         v
resources capabilities effects contracts security
    |
    v
target-independent compilation planning
    |
    +----------------------+----------------------+
    |                      |                      |
    v                      v                      v
classical IR          quantum::ir            HDL/hardware
    |                      |                      |
    +----------------------+----------------------+
                           |
                           v
                   optimization/lowering
                           |
                           v
                   routing/scheduling
                           |
                           v
                    resilience/recovery
                           |
                           v
                       ZQN/HAL
                           |
                           v
                    target realization

The grammar is therefore not the execution engine.

---

3. POCO-REAF Requirement

Policies MUST preserve:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever

A policy must describe portable intent, not a physical realization.

For example:

requires capability("quantum.measurement");
requires memory >= required_memory;
prefer execution::deterministic;

must not implicitly mean:

use CPU 0
use GPU 0
use QPU 0
use node 0
use exactly N devices

The compiler/runtime is responsible for determining a valid realization from:

- requirements
- capabilities
- resources
- constraints
- preferences
- effects
- contracts
- policies
- target availability
- execution state
- optimization
- scheduling
- resilience

The source policy remains portable.

---

4. What This Directory Owns

"grammar/policies/" owns policy-specific source syntax.

It owns the structure necessary to express policy declarations and their policy-domain payloads.

The principal authority is:

grammar/policies/policy.g4

Supporting grammars provide specialized payload syntax.

---

5. What This Directory Does Not Own

This directory does not own:

- lexer rules
- token definitions
- identifiers
- qualified-name semantics
- general expressions
- general type syntax
- universal resource semantics
- universal capability semantics
- universal constraint semantics
- contract semantics
- effect semantics
- security enforcement
- hardware discovery
- physical placement
- routing
- scheduling
- calibration
- quantum error correction
- ZQN
- HAL
- runtime policy enforcement
- classical IR
- "quantum::ir"
- HDL IR
- backend implementation
- device-specific limits
- vendor-specific hardware catalogues

Those responsibilities belong elsewhere.

---

6. Single Policy Authority

There must be exactly one source-level policy authority.

That authority is:

grammar/policies/policy.g4

Other grammars under this directory MUST NOT create competing policy declarations.

They should instead own specialized payloads.

Conceptually:

policy.g4
    |
    +-- requirements.g4
    +-- constraints.g4
    +-- permissions.g4
    +-- prohibitions.g4
    +-- preferences.g4
    +-- fallbacks.g4
    +-- adaptation.g4
    +-- execution.g4
    +-- security.g4
    +-- resource.g4
    +-- deployment.g4
    +-- simulation.g4
    +-- provenance.g4
    +-- constitution.g4
    +-- other policy payloads

No specialized file may redefine:

policyDeclaration
policyBody
policyMember

unless explicitly designated as an adapter/composition grammar.

---

7. Current Policy Files

The policy subsystem is organized around the following responsibilities.

File| Responsibility
"policy.g4"| Canonical policy declaration and outer policy-member syntax
"scopes.g4"| Policy scope/applicability payload
"requirements.g4"| Policy-specific requirement payload
"constraints.g4"| Policy-specific constraint payload
"permissions.g4"| Policy permission payload
"prohibitions.g4"| Policy prohibition payload
"preferences.g4"| Policy preference payload
"fallbacks.g4"| Policy fallback payload
"adaptation.g4"| Policy-controlled adaptation payload
"execution.g4"| Execution-policy payload
"security.g4"| Security-policy payload
"resource.g4"| Resource-policy payload
"deployment.g4"| Deployment-policy payload
"simulation.g4"| Simulation-policy payload
"provenance.g4"| Provenance-policy payload
"constitution.g4"| Foundational/constitutional policy composition and governance payload

The exact set may grow as new policy domains become mature.

New files must follow the ownership rules in this document.

---

8. File Ownership Contract

Every ".g4" file under this directory MUST explicitly document:

PURPOSE
OWNS
DOES NOT OWN
DEPENDS_ON
EXPORTS
CONSUMED_BY
AST_OWNER
SEMANTIC_OWNER
IR_OWNER
TEST_OWNER
SPEC_OWNER

It must additionally document:

LEXER DEPENDENCIES
GRAMMAR DEPENDENCIES
TYPE CONTRACT
EFFECT CONTRACT
CAPABILITY CONTRACT
RESOURCE CONTRACT
CONTRACT CONTRACT
POLICY CONTRACT
PROVENANCE CONTRACT
QUANTUM BOUNDARY
HDL BOUNDARY
BACKEND BOUNDARY
DIAGNOSTICS
COMPATIBILITY
SCALABILITY
COMPLETION CRITERIA

This requirement prevents a supposedly completed file from becoming dependent on undocumented assumptions introduced later.

---

9. Dependency Direction

Policy grammar dependencies must flow downward.

Preferred direction:

policy.g4
    |
    v
policy payload grammars
    |
    v
core / expressions / names

Not:

policy.g4 <--> payload.g4

Circular ANTLR grammar dependencies are prohibited.

For example:

Policy
  -> ProvenancePolicy

is valid.

The reverse:

ProvenancePolicy
  -> Policy

is prohibited.

---

10. Canonical Lexer Boundary

All policy grammars use the canonical Zamani lexer vocabulary.

They must use:

options {
    tokenVocab = ZamaniLexer;
}

Policy grammars must not define lexer rules.

They must not introduce local tokens such as:

POLICY_TOKEN: ...;

or equivalent local lexical authorities.

The canonical lexical authority remains under:

grammar/lexer/
grammar/antlr/

---

11. Keyword Policy

A new policy concept must not automatically become a reserved keyword.

Before adding a keyword, determine whether the concept can be represented through:

- an identifier
- a qualified name
- an expression
- an attribute
- metadata
- a policy property
- a dialect extension
- a capability
- a semantic registry

Open-world concepts should normally remain symbolic.

For example, the grammar should permit future semantic dimensions such as:

execution::deterministic
quantum::resilience
hardware::energy_efficiency
learning::adaptation
reasoning::explainability
future::policy_dimension
vendor::extension

without requiring a new core keyword for every new concept.

---

12. No Hard-Coded Capacity Limits

The policy grammar MUST NOT define universal limits for:

- qubits
- CPUs
- GPUs
- FPGAs
- ASIC resources
- nodes
- devices
- threads
- memory
- tensor rank
- register width
- network size
- topology size
- storage
- channels
- actors
- agents
- policy members
- policy clauses
- metadata entries
- capability dimensions
- resource dimensions

The grammar must use symbolic expressions.

Correct:

requires qubits >= required_qubits;
requires memory >= required_memory;
requires capability("tensor.compute");
requires topology(required_topology);

Incorrect architecture:

MAX_QUBITS
MAX_CPUS
MAX_GPUS
MAX_NODES
MAX_MEMORY
MAX_THREADS

Physical limitations belong to target/resource discovery and feasibility analysis, not language syntax.

---

13. Resource Abstraction

Policies may refer to resource requirements and preferences.

The policy grammar does not determine whether a resource is supplied by:

- an embedded device
- CPU
- multicore system
- GPU
- FPGA
- ASIC
- accelerator
- QPU
- simulator
- workstation
- HPC system
- cluster
- cloud
- distributed system
- future computational platform

The semantic resource subsystem resolves those references.

Policy syntax therefore remains independent of physical scale.

---

14. Capability Abstraction

Capabilities describe what a realization can provide.

Examples include:

capability("quantum.measurement")
capability("tensor.compute")
capability("distributed.collective")
capability("hardware.synthesis")
capability("network.stream")
capability("learning.adaptation")

The policy grammar must not enumerate every possible capability.

Capabilities are open-world semantic values.

New capabilities may be introduced by:

- core language evolution
- dialects
- libraries
- target descriptions
- hardware descriptions
- execution environments
- future domains

without changing the policy grammar.

---

15. Policy Versus Requirement

A requirement states what must be satisfied.

A preference states what is desirable.

A constraint restricts validity.

A permission permits an action.

A prohibition forbids an action.

A fallback provides an alternative.

A policy composes these declarations into governing intent.

They must remain semantically distinct.

For example:

requirement
    = necessary condition

constraint
    = validity restriction

preference
    = advisory choice

permission
    = allowed behavior

prohibition
    = disallowed behavior

fallback
    = alternative semantic path

A preference must never silently become a requirement.

A permission must never silently bypass security authorization.

A policy must never silently manufacture a capability.

---

16. Policy Expressions

Policy payloads should consume the canonical expression grammar.

They must not create independent expression languages.

The preferred dependency is:

policy payload
    |
    v
Expressions
    |
    v
canonical expression semantics

This allows policy expressions to evolve together with the language.

---

17. Policy Properties

Open-world policy properties are encouraged.

For example:

policy execution {
    deterministic = true;
    reproducible = true;
    execution::mode = preferred_mode;
    future::policy_dimension = desired_value;
}

The grammar parses the structure.

Semantic analysis determines whether a property is:

- standard
- dialect-defined
- target-defined
- experimental
- deprecated
- unknown
- invalid
- conflicting

Unknown properties must not automatically become parser errors when the language contract permits open-world extension.

---

18. Policy Scope

Policy applicability must be explicit.

A policy may semantically apply to:

- a declaration
- a function
- a module
- an expression
- an operation
- a computation region
- an execution plan
- a compilation unit
- a deployment
- a simulation
- a domain
- a capability boundary

The scope grammar must describe applicability, not physical placement.

---

19. Policy Composition

Policy composition must support semantic relationships such as:

extends
override
composition
inheritance
applicability
precedence
fallback

Composition must be resolved semantically.

The grammar must not encode a fixed numeric policy hierarchy.

Policy precedence should be represented as a semantic ordering or partial ordering rather than a language-defined finite priority scale.

---

20. Constitutional Policy Layer

"constitution.g4" is a policy governance layer, not a second policy language.

It may establish foundational policy relationships such as:

- foundational principles
- invariants
- mandatory requirements
- prohibitions
- guarantees
- governance metadata
- amendment relationships
- applicability
- authority
- provenance
- precedence
- compatibility
- review requirements

It must reuse existing policy concepts rather than redefine them.

The intended relationship is:

constitution
    |
    v
policy governance
    |
    v
ordinary policies
    |
    v
semantic policy model

It must not create a second:

policyDeclaration
policyExpression
requirementExpression
constraintExpression

language.

---

21. Contracts

Policies may interact with the validation subsystem.

Relevant concepts include:

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

The policy grammar must reference those concepts where appropriate.

It must not duplicate their universal grammar.

The ownership boundary is:

grammar/validation/
    |
    v
contract syntax/semantics

grammar/policies/
    |
    v
policy references/bindings

---

22. Effects

Policy can govern effects.

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

The policy grammar does not implement effects.

The effect subsystem determines their meaning.

Policy merely expresses governing intent.

---

23. Security Boundary

Policy and security are related but distinct.

Policy can express:

allow
permit
forbid
deny
sandbox
trust
authorization intent

but grammar/policies must not implement:

- authentication
- cryptography
- credential verification
- runtime authorization
- trust evaluation
- secure hardware enforcement

Those remain under:

grammar/security/

The relationship is:

policy intent
    |
    v
security semantic analysis
    |
    v
authorization/trust/runtime enforcement

---

24. Sandbox Boundary

Sandbox policy may constrain:

- effects
- capabilities
- resources
- networking
- filesystem interaction
- native calls
- FFI
- reflection
- code generation
- adaptation

The grammar remains target-neutral.

A policy must not encode a particular operating system, container implementation, hypervisor, processor, or security product.

---

25. Adaptation

Controlled adaptation is supported because adaptive computation is a universal capability.

However:

adaptation != unrestricted self-modification

An adaptation policy should be evaluated through:

policy
    |
    v
authorization
    |
    v
capability analysis
    |
    v
effect analysis
    |
    v
resource analysis
    |
    v
validation
    |
    v
provenance
    |
    v
adaptive execution

The grammar itself does not execute adaptation.

---

26. Determinism and Reproducibility

Policies may express intent concerning:

determinism
reproducibility
auditability
traceability
provenance

Parsing itself must remain deterministic.

Parsing must not depend on:

- hardware
- available CPUs
- available GPUs
- QPU state
- network state
- filesystem state
- runtime state
- randomness
- wall-clock time

Reproducibility semantics are handled downstream.

---

27. Simulation

Policy simulation is an execution concern.

The policy layer may govern simulation of:

- classical computation
- quantum computation
- HDL
- hardware
- distributed systems
- AI/model execution
- fault behavior
- performance
- resource behavior

The policy grammar must not create a second simulation language.

The boundaries are:

policy simulation intent
        |
        v
execution simulation
        |
        +--> classical simulation
        +--> quantum simulation
        +--> HDL simulation
        +--> hardware simulation
        +--> distributed simulation
        +--> model simulation

---

28. Quantum Boundary

Policy syntax must remain independent of physical quantum implementation.

Policies may express requirements such as:

capability::quantum::measurement
quantum::resilience
quantum::dynamic_control
quantum::fidelity

but must not encode:

- physical qubit identifiers
- coupling maps
- routing paths
- pulse schedules
- calibration data
- vendor QPU topology
- physical gate implementations
- QEC schedules

The semantic pipeline remains:

policy
    |
    v
semantic model
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
QEC/resilience
    |
    v
ZQN
    |
    v
HAL
    |
    v
QPU

---

29. HDL and Hardware Boundary

Policy may describe hardware intent such as:

hardware::clocking
hardware::pipeline_support
hardware::memory_interface
hardware::accelerated_compute
hardware::synthesis

but it must not encode universal:

- register widths
- bus widths
- FPGA capacities
- ASIC cell limits
- device counts
- placement maps
- physical routing
- timing implementation

Those belong to hardware/HDL semantic and backend layers.

---

30. AI and Knowledge Integration

The language-wide semantic model may use policy to govern:

- reasoning
- inference
- deduction
- knowledge
- learning
- adaptation
- uncertainty
- evidence
- explanations
- decisions
- agents
- neural-symbolic computation

These are not separate policy languages.

For example:

policy learning {
    requires capability("learning.train");
    provenance = required;
    reproducible = true;
}

The policy subsystem expresses the governance intent.

The AI subsystem owns AI semantics.

---

31. Multi-Agent Integration

Agent policies must integrate with the existing concurrency architecture.

The preferred path is:

agent policy
    |
    v
AI semantic model
    |
    v
actor semantics
    |
    v
message passing
    |
    v
scheduler/runtime

The policy grammar must not create a second actor model.

---

32. Provenance

Policy decisions must remain traceable.

The semantic provenance chain should be capable of recording:

source
derived_from
generated_by
transformed_by
verified_by
reason
evidence
decision
version
execution context

The grammar expresses provenance policy intent.

The provenance subsystem records actual provenance.

---

33. Interoperability

Policies may govern:

- FFI
- ABI
- foreign functions
- external types
- data interchange
- generated bindings
- external artifacts
- dialect interoperability

But SQL, JSON, XML and similar formats must remain dialect/interoperability concerns.

The policy subsystem should consume their semantic representation rather than embedding those grammars here.

---

34. Metaprogramming Boundary

Policy may constrain:

- reflection
- introspection
- compile-time execution
- code generation
- syntax-tree generation
- type-level computation

However, the actual metaprogramming syntax belongs to:

grammar/metaprogramming/
grammar/macros/
grammar/compile/

Policy provides governance.

---

35. Classical Computing

Classical computation consumes the same policy abstractions:

requirements
capabilities
resources
constraints
effects
contracts
policies
provenance

There must not be an AI-specific policy model and a separate classical policy model.

---

36. Distributed Computing

Distributed policies may govern:

- placement intent
- topology requirements
- communication
- consistency
- fault handling
- retries
- recovery
- collective behavior
- service selection

They must never encode a universal maximum node count.

For example:

requires topology(required_topology);

is portable.

A language-wide fixed node limit is not.

---

37. Networking

Network-related policy should participate in:

effect(network)
capability(network.*)
resource requirements
security policy
provenance

The policy grammar should not define protocol implementations.

Protocol syntax belongs to networking/dialect/interoperability layers.

---

38. Deployment

Deployment policies describe intent such as:

- portability
- availability
- resilience
- reproducibility
- capability requirements
- environment requirements
- deployment preferences

They do not select a physical machine in the source grammar.

---

39. Compiler Boundary

Policies influence compilation planning.

The compiler may use policy information for:

resource analysis
capability resolution
optimization legality
specialization
lowering
target selection
scheduling
deployment

But policy syntax does not directly generate machine instructions.

---

40. Canonical IR Boundary

Policy information must first become part of the semantic model.

Preferred path:

policy syntax
    |
    v
AST
    |
    v
semantic policy model
    |
    v
canonical compilation model
    |
    +--> classical IR
    |
    +--> quantum::ir
    |
    +--> HDL/hardware representation
    |
    +--> distributed execution representation
    |
    +--> other domain IR

No policy grammar should directly emit vendor IR.

---

41. AST Contract

Policy AST nodes must remain domain-neutral.

They may contain:

- policy identity
- policy members
- expressions
- requirements
- constraints
- capabilities
- resources
- permissions
- prohibitions
- preferences
- fallbacks
- composition
- scope
- metadata
- provenance references

They must not contain:

- physical device objects
- QPU topology
- CPU instruction objects
- GPU kernels
- FPGA routing
- calibration data
- QEC schedules
- vendor backend objects

Those belong downstream.

---

42. Semantic Contract

Semantic analysis must determine:

- name resolution
- policy scope
- policy composition
- inheritance
- overrides
- conflicts
- requirement satisfaction
- constraint satisfiability
- capability availability
- resource feasibility
- effect compatibility
- contract compatibility
- security compatibility
- provenance requirements
- target-independent policy consequences

A parser success does not imply semantic validity.

---

43. Error Classification

Errors must be classified correctly.

Parser errors

Examples:

- malformed policy declaration
- missing name
- missing braces
- malformed expression
- missing semicolon
- malformed property

Semantic errors

Examples:

- unknown policy reference
- conflicting policies
- unsatisfiable requirement
- unavailable capability
- incompatible effect
- invalid policy composition
- unauthorized override
- impossible resource requirement

A target lacking a capability should normally produce a semantic/resource feasibility result, not a parser error.

---

44. Scalability Contract

Policy grammar structures must use unbounded grammar repetition where appropriate:

policyMember*

rather than fixed cardinalities.

There must be no language-defined maximum for:

- policy count
- policy members
- policy clauses
- policy properties
- metadata entries
- qualified-name depth
- expression complexity
- policy nesting
- policy composition
- resource dimensions
- capability dimensions

Actual limits may arise from available:

- memory
- processing time
- operating-system resources
- implementation limits
- compiler configuration

Those are implementation/resource limits, not language ceilings.

---

45. Determinism Contract

Parsing must be deterministic for a fixed:

source
+
grammar version
+
lexer version
+
compatibility configuration

It must not depend upon:

- hardware availability
- target hardware
- runtime state
- network state
- wall-clock time
- randomness
- external device state

---

46. Compatibility Contract

Existing policy syntax must remain compatible unless a deliberate language-version transition is specified.

When syntax is replaced:

current
    |
    v
deprecated
    |
    v
compatibility support
    |
    v
removed only through an explicit language-version policy

Historical syntax belongs under:

grammar/compatibility/

It must not be silently reintroduced into the canonical policy grammar.

---

47. Production File Contract

Every policy ".g4" file is considered complete only when all of the following are documented:

Purpose
Owns
Does Not Own
Dependencies
Exports
Consumers
AST owner
Semantic owner
IR owner
Test owner
Specification owner
Lexer contract
Expression contract
Type contract
Effect contract
Capability contract
Resource contract
Contract contract
Policy contract
Provenance contract
Quantum boundary
HDL boundary
Backend boundary
Diagnostics
Compatibility
Positive tests
Negative tests
Boundary tests
Scalability tests
Determinism tests
Cross-domain tests
Completion criteria

---

48. Recommended Test Layout

Policy tests should live under:

grammar/tests/policies/

Recommended structure:

grammar/tests/policies/
├── declarations/
├── scopes/
├── requirements/
├── constraints/
├── permissions/
├── prohibitions/
├── preferences/
├── fallbacks/
├── adaptation/
├── execution/
├── security/
├── resource/
├── deployment/
├── simulation/
├── provenance/
├── constitution/
├── composition/
├── conflicts/
├── compatibility/
├── positive/
├── negative/
├── boundary/
├── scalability/
├── determinism/
└── cross-domain/

---

49. Required Policy Test Classes

Every policy feature requires at least:

Positive tests

Valid syntax and valid combinations.

Negative tests

Malformed syntax and invalid structural combinations.

Boundary tests

Interactions with other language domains.

Scalability tests

Large symbolic policy structures without language-level ceilings.

Determinism tests

Repeated parsing produces equivalent structural results.

Compatibility tests

Historical/current syntax behavior.

Cross-domain tests

At minimum:

classical
quantum
hybrid
HDL
hardware
AI
data
distributed
networking
security
simulation
deployment
interoperability
metaprogramming

---

50. Mandatory Cross-Domain Policy Test

At least one integration test should combine:

requirements
+
capabilities
+
resources
+
constraints
+
preferences
+
contracts
+
effects
+
security
+
provenance
+
adaptation
+
simulation
+
classical computation
+
quantum computation
+
HDL/hardware intent
+
AI reasoning/learning
+
distributed execution

The expected pipeline is:

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
semantic policy model
  |
  v
resource analysis
  |
  v
capability analysis
  |
  v
effect analysis
  |
  v
contract analysis
  |
  v
security analysis
  |
  v
provenance
  |
  v
compilation planning
  |
  +--> classical IR
  |
  +--> quantum::ir
  |
  +--> HDL/hardware
  |
  +--> distributed plan

---

51. Integration Matrix

Policy feature| Primary owner| Semantic consumer
Declaration| "policy.g4"| Policy semantic model
Scope| "scopes.g4"| Scope resolver
Requirements| "requirements.g4"| Resource/capability analysis
Constraints| "constraints.g4"| Validation/planning
Permissions| "permissions.g4"| Security
Prohibitions| "prohibitions.g4"| Security/validation
Preferences| "preferences.g4"| Planning
Fallbacks| "fallbacks.g4"| Adaptive execution
Adaptation| "adaptation.g4"| Execution
Execution| "execution.g4"| Execution planner
Security| "security.g4"| Security subsystem
Resource| "resource.g4"| Resource subsystem
Deployment| "deployment.g4"| Deployment planner
Simulation| "simulation.g4"| Simulation subsystem
Provenance| "provenance.g4"| Provenance subsystem
Constitution| "constitution.g4"| Governance/policy semantics

---

52. Relationship to "grammar/core/"

The core subsystem owns universal abstractions such as:

requirements
constraints
capabilities
metadata
names
expressions

Policy grammars bind those concepts into policy syntax.

Therefore:

core/requirements.g4
        |
        v
policies/requirements.g4

means:

universal requirement
        +
policy context

It does not mean a second requirement language.

---

53. Relationship to "grammar/resources/"

Resource grammars describe resource concepts.

Policy grammars describe policies governing resources.

Therefore:

resources
    |
    v
resource semantic model
    ^
    |
policies

A policy may express:

requires memory >= required_memory;
prefer resource::latency <= target_latency;

but resource realization remains downstream.

---

54. Relationship to "grammar/effects/"

Effects describe what computation may do.

Policies can constrain those effects.

Example conceptual relationship:

effect(network)
        ^
        |
network policy
        |
        v
security/resource/capability analysis

Neither subsystem should duplicate the other.

---

55. Relationship to "grammar/validation/"

Validation owns universal contracts and assertions.

Policies can:

- reference contracts
- constrain contracts
- require guarantees
- establish applicability

but must not recreate contract syntax.

---

56. Relationship to "grammar/security/"

Security owns enforcement semantics.

Policy owns governance intent.

This distinction is mandatory.

policy
    |
    v
security policy model
    |
    v
authorization/trust enforcement

---

57. Relationship to "grammar/execution/"

Execution owns runtime/execution intent.

Policy controls execution through declarative constraints.

For example:

policy execution {
    deterministic = true;
    reproducible = true;
    fallback = execution::recovery;
}

Execution planning decides how that intent is realized.

---

58. Relationship to Quantum Computing

Policy must remain above physical quantum realization.

The policy may express:

requires capability("quantum.measurement");
requires capability("quantum.dynamic_control");
prefer quantum::resilience;

The quantum compiler then maps semantic requirements through:

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
QEC/resilience
    |
    v
ZQN/HAL

---

59. Relationship to HDL

Policy can constrain HDL/hardware intent.

For example:

requires capability("hardware.synthesis");
prefer hardware::timing;

The policy does not decide physical synthesis.

---

60. Relationship to AI

Reasoning, knowledge, learning, adaptation, uncertainty, evidence and explainability are semantic capabilities.

Policy can govern them without introducing application-specific keywords.

For example:

policy learning {
    requires capability("learning.train");
    reproducible = true;
    provenance = required;
}

The learning subsystem determines the meaning of training.

---

61. Relationship to Interoperability

Policies may govern foreign boundaries:

ffi
abi
foreign
external
data exchange
generated bindings

Such policies must participate in effect and security analysis.

The policy grammar does not define ABI layouts or foreign calling conventions.

---

62. Relationship to Metaprogramming

Reflection and generation can be governed by policy.

For example, policy may require explicit authorization before:

reflection
code generation
compile-time execution
external loading

Actual metaprogramming semantics remain elsewhere.

---

63. No Application-Specific Keyword Explosion

The policy subsystem must not become a catalogue of application domains.

Do not add dedicated universal policy keywords merely for:

- computer vision
- sentiment analysis
- robotics
- payment systems
- legal workflows
- administration
- blockchain
- VR
- AR
- specific AI models
- specific vendors
- specific protocols

Those concepts should be implemented through:

libraries
dialects
capabilities
policies
services
semantic registries
applications

This preserves Zamani's universality.

---

64. Open-World Extension Model

Future policy concepts should normally enter through:

qualified names
expressions
properties
metadata
capabilities
dialects
semantic registries

rather than through endless core grammar changes.

For example:

future::policy::concept
vendor::extension::property
domain::specialized::requirement
research::experimental::constraint

can remain syntactically representable without requiring the universal parser to know every future concept.

---

65. Domain Neutrality

Policy syntax must not know whether a policy ultimately applies to:

atom-scale computation
embedded systems
CPU
multicore CPU
GPU
FPGA
ASIC
accelerator
QPU
simulator
HPC
cluster
cloud
distributed system
future hardware

The semantic capability/resource system determines feasibility.

---

66. No Physical Assumptions

The policy subsystem must never assume:

fixed CPU width
fixed GPU count
fixed QPU size
fixed FPGA capacity
fixed memory size
fixed node count
fixed network size
fixed tensor rank
fixed register width

The absence of such assumptions is necessary for POCO-REAF.

---

67. Rust Contract

The grammar contains no Rust semantic actions.

Generated Rust integration must remain:

Rust 2021
Rust 1.97+
safe Rust
no unsafe

The grammar must not perform:

- filesystem access
- network access
- device discovery
- FFI
- runtime callbacks
- scheduling
- hardware inspection
- resource allocation

Those belong to downstream implementation layers.

---

68. Generated Parser Contract

ANTLR generation must be reproducible.

The build system must:

1. resolve canonical lexer vocabulary;
2. resolve grammar imports;
3. generate parser sources;
4. generate Rust bindings;
5. compile under the supported Rust version;
6. run policy conformance tests;
7. run cross-domain tests.

A grammar file is not considered production-ready merely because ANTLR accepts its syntax.

---

69. Completion Criteria for "grammar/policies/"

The policy subsystem is production-ready only when:

- [ ] "policy.g4" is the single policy declaration authority.
- [ ] Every specialized payload has exactly one owner.
- [ ] No policy grammar defines lexer tokens.
- [ ] No policy grammar creates a second expression language.
- [ ] No policy grammar creates a second resource language.
- [ ] No policy grammar creates a second capability language.
- [ ] No policy grammar creates a second contract language.
- [ ] No circular grammar imports exist.
- [ ] All imports resolve through the canonical ANTLR grammar path.
- [ ] All generated parsers compile.
- [ ] Rust generation succeeds on Rust 1.97+.
- [ ] Generated integration uses safe Rust.
- [ ] No "unsafe" code is required by grammar integration.
- [ ] Policy syntax remains target-independent.
- [ ] No physical machine capacities are hard-coded.
- [ ] No finite hardware catalogue is embedded.
- [ ] No quantum gate catalogue is embedded.
- [ ] No application-specific keyword explosion exists.
- [ ] Open-world policy properties are supported where appropriate.
- [ ] Requirements remain distinct from preferences.
- [ ] Constraints remain distinct from requirements.
- [ ] Permissions remain distinct from authorization enforcement.
- [ ] Prohibitions remain distinct from runtime enforcement.
- [ ] Fallbacks remain distinct from runtime recovery.
- [ ] Adaptation remains controlled.
- [ ] Provenance remains traceable.
- [ ] Policy composition is deterministic.
- [ ] Policy conflicts are semantically diagnosed.
- [ ] Resource feasibility is downstream.
- [ ] Capability negotiation is downstream.
- [ ] Quantum realization remains behind "quantum::ir".
- [ ] HDL realization remains downstream.
- [ ] Backend-specific information does not enter the AST.
- [ ] Positive tests pass.
- [ ] Negative tests pass.
- [ ] Boundary tests pass.
- [ ] Scalability tests pass.
- [ ] Determinism tests pass.
- [ ] Compatibility tests pass.
- [ ] Cross-domain tests pass.
- [ ] Constitution integration is acyclic and explicit.
- [ ] Policy documentation matches actual grammar ownership.
- [ ] "grammar/spec/policies.md" is the normative semantic specification.

---

70. Definition of DONE for Individual Policy Files

A policy grammar file is considered DONE when its own contract is complete and verified.

Completion must not depend on another future file being rewritten.

The file must already state:

what it owns
what it does not own
what it imports
what imports it
what AST it produces
what semantic model consumes it
what IR consumes that semantic model
what tests prove it
what specification governs it
how it scales
how it remains portable
how it interacts with quantum
how it interacts with HDL
how it interacts with resources
how it interacts with capabilities
how it interacts with effects
how it interacts with contracts
how it interacts with provenance
how it interacts with security
how compatibility works

If a future file needs to consume it, that should be an integration operation—not a reason to redesign its foundational syntax.

---

71. Required Integration Order

Policy implementation should proceed in dependency order.

Stage 1 — Foundation

lexer/tokens.g4
lexer/keywords.g4
core/names.g4
core/qualified-names.g4
expressions/

Stage 2 — Universal semantic foundations

core/requirements.g4
core/constraints.g4
core/capabilities.g4
resources/
effects/
validation/

Stage 3 — Policy authority

policies/policy.g4
policies/scopes.g4

Stage 4 — Policy payloads

policies/requirements.g4
policies/constraints.g4
policies/permissions.g4
policies/prohibitions.g4
policies/preferences.g4
policies/fallbacks.g4

Stage 5 — Execution/governance

policies/adaptation.g4
policies/execution.g4
policies/security.g4
policies/resource.g4
policies/deployment.g4
policies/simulation.g4
policies/provenance.g4
policies/constitution.g4

Stage 6 — Domain integration

classical/
quantum/
hybrid/
hdl/
hardware/
ai/
distributed/
networking/
interoperability/
metaprogramming/

Stage 7 — Conformance

tests/policies/
tests/cross-domain/
tests/scalability/
tests/compatibility/

---

72. Canonical Semantic Model

The policy subsystem ultimately contributes to a universal semantic model:

VALUE
  |
TYPE
  |
OPERATION
  |
+-----------------------------+
|                             |
EFFECT                    CAPABILITY
|                             |
+--------------+--------------+
               |
            RESOURCE
               |
          REQUIREMENT
               |
           CONSTRAINT
               |
             POLICY
               |
            CONTRACT
               |
            EVIDENCE
               |
          PROVENANCE
               |
            DECISION
               |
        SEMANTIC MODEL

That model can then feed:

classical
quantum
HDL
hardware
AI
distributed
networking
data
interoperability
future domains

without requiring separate policy languages.

---

73. Final Architecture

The complete policy relationship is:

                    Zamani Program
                          |
                          v
                       Lexer
                          |
                          v
                     policy.g4
                          |
          +---------------+----------------+
          |               |                |
          v               v                v
     requirements    constraints      capabilities
          |               |                |
          +---------------+----------------+
                          |
          +---------------+----------------+
          |               |                |
          v               v                v
     preferences      permissions      prohibitions
          |               |                |
          +---------------+----------------+
                          |
          +---------------+----------------+
          |               |                |
          v               v                v
       fallback       adaptation       execution
          |               |                |
          +---------------+----------------+
                          |
          +---------------+----------------+
          |               |                |
          v               v                v
      security       simulation       provenance
                          |
                          v
                     constitution
                          |
                          v
                 Semantic Policy Model
                          |
        +-----------------+------------------+
        |                 |                  |
        v                 v                  v
     Resources       Capabilities         Effects
        |                 |                  |
        +-----------------+------------------+
                          |
                          v
                      Contracts
                          |
                          v
                     Validation
                          |
                          v
                      Planning
                          |
          +---------------+----------------+
          |               |                |
          v               v                v
     Classical       quantum::ir       HDL/Hardware
          |               |                |
          +---------------+----------------+
                          |
                          v
                  Target-independent
                     optimization
                          |
                          v
                   lowering/routing
                          |
                          v
                    scheduling/QEC
                          |
                          v
                       ZQN/HAL
                          |
                          v
                  Target realization

---

74. Architectural Invariant

The policy subsystem must always preserve these invariants:

ONE policy declaration authority
ONE canonical expression language
ONE universal requirement model
ONE universal constraint model
ONE capability model
ONE resource model
ONE contract model
ONE effect model
ONE provenance model
ONE domain-neutral AST
ONE semantic policy model
ONE canonical quantum::ir boundary
NO physical capacity ceiling
NO vendor lock-in in source syntax
NO application-specific keyword explosion
NO parser-time hardware assumptions
NO runtime behavior in grammar
NO unsafe Rust requirement

---

75. Final Principle

"grammar/policies/" exists to answer:

«What computational behavior, resource realization, execution strategy, security posture, adaptation strategy, provenance requirement, or governance rule does the program declare?»

It does not answer:

«Which physical machine must execute it?»

That distinction is essential to POCO-REAF.

A Zamani policy therefore describes intent and constraints while the rest of the compiler/runtime determines a valid realization from the resources and capabilities actually available.

Consequently, the same source policy can remain valid across:

atom-scale
embedded
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
cloud
future computational systems

without introducing language-level capacity ceilings.

The policy grammar is therefore a portable semantic governance layer, not a hardware-selection language and not a second programming language.
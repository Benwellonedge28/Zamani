Zamani Hybrid Grammar

Production Specification

Path: "grammar/hybrid/"

Status: Production architecture

Language: Zamani

Grammar technology: ANTLR

Implementation ecosystem: Rust 1.97 / Rust 1.97.1

Safety requirement: No "unsafe" Rust

Architectural principle: Syntax describes portable computation and intent; semantic interpretation, resource realization, compilation, scheduling, routing, optimization, and execution belong to downstream systems.

Scalability principle: No grammar production may impose a fixed machine, device, resource, topology, qubit, core, thread, accelerator, memory, or deployment limit.

POCO-REAF:

«Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever»

---

1. Purpose

"grammar/hybrid/" defines the syntax required for programs whose computation crosses two or more computational domains, particularly:

- classical computation
- quantum computation
- hardware/accelerator computation
- heterogeneous computation
- future computational domains

Hybrid syntax allows Zamani source code to express relationships between computational domains without embedding assumptions about the physical machine on which those domains will eventually execute.

The hybrid grammar exists to express source-level concepts such as:

- classical values controlling quantum operations
- quantum measurements producing classical values
- classical computation consuming quantum results
- quantum computation consuming classical parameters
- classical and quantum regions interacting
- accelerator-backed computation
- heterogeneous execution intent
- resource requirements spanning multiple domains
- domain crossings
- synchronization boundaries
- data movement between computational domains
- portable execution constraints

The grammar does not decide:

- which processor is used
- which quantum processor is used
- how many qubits exist
- which GPU is used
- how many accelerators exist
- how quantum operations are routed
- how operations are scheduled
- which QEC code is selected
- which noise model applies
- which backend is selected
- how calibration is performed
- how a circuit is optimized
- how a hardware device is discovered
- how runtime recovery occurs

Those responsibilities belong to other Zamani subsystems.

---

2. Architectural Role

The hybrid grammar occupies this conceptual position:

Zamani Source
     │
     ▼
Canonical Lexer
     │
     ▼
ANTLR Parser
     │
     ▼
Hybrid Syntax
     │
     ▼
AST / Syntax Representation
     │
     ▼
Semantic Analysis
     │
     ├── Type System
     ├── Effect System
     ├── Capability System
     ├── Resource Analysis
     └── Domain Validation
     │
     ▼
Canonical Semantic IR
     │
     ├── Classical IR
     └── quantum::ir
     │
     ▼
Optimization
     │
     ▼
Routing
     │
     ▼
Scheduling
     │
     ▼
Hardware Abstraction
     │
     ▼
Compilation
     │
     ▼
Runtime / Execution

The grammar is therefore upstream of semantic interpretation.

It must never become a hidden replacement for:

- AST definitions
- semantic analysis
- canonical IR
- "quantum::ir"
- QEC
- ZQN
- scheduling
- routing
- optimization
- hardware HAL
- runtime

---

3. Ownership

3.1 This directory owns

"grammar/hybrid/" owns:

- hybrid source syntax
- grammar productions describing domain interaction
- syntactic structure of classical/quantum boundaries
- syntactic structure of accelerator interoperability
- syntactic resource expressions specific to hybrid operations
- syntactic annotations needed to identify hybrid intent
- parser-level representation of domain relationships

---

4. Explicit Non-Ownership

This directory does not own:

Canonical quantum semantics

Owned by the quantum semantic/IR subsystem, with:

quantum::ir

remaining the canonical quantum semantic boundary.

Quantum error correction

Owned by QEC.

The grammar may represent source-level QEC intent or declarations where the language specification requires them, but must not implement QEC algorithms.

Quantum noise

Owned by ZQN.

The hybrid grammar must not define a competing noise model.

Hardware discovery

Owned by hardware/HAL infrastructure.

Device selection

Owned by target/resource/compilation/runtime policy.

Routing

Owned by routing.

Scheduling

Owned by scheduling.

Optimization

Owned by optimization.

Runtime recovery

Owned by resilience/runtime systems.

Simulation

Owned by simulation infrastructure.

Resource discovery

Owned by hardware/runtime/resource-management infrastructure.

Physical topology

Owned by hardware and routing.

AST implementation

The grammar produces parser structures consumed by the AST/frontend layer; it does not become the AST.

IR implementation

The grammar does not define canonical IR structures.

---

5. Production Files

The hybrid directory consists of:

grammar/hybrid/
├── README.md
├── hybrid.g4
├── classical-quantum.g4
├── quantum-classical-control.g4
├── accelerator-interoperability.g4
└── hybrid-resources.g4

Each file has a single architectural responsibility.

---

6. "hybrid.g4"

Purpose

"hybrid.g4" is the composition grammar for hybrid computation.

It defines the high-level entry points that combine the dedicated hybrid grammar components.

It must not duplicate their detailed productions.

Owns

- hybrid declaration/construct composition
- hybrid computation entry points
- hybrid block composition
- hybrid-domain composition
- references to classical/quantum interaction constructs
- references to accelerator interaction constructs
- references to hybrid resource constructs

Does Not Own

- classical expressions
- quantum gate syntax
- quantum measurement syntax
- accelerator-specific syntax
- physical device syntax
- resource implementation
- AST classes
- IR nodes

Inputs

Imports/references the canonical grammar components for:

- core syntax
- expressions
- statements
- types
- classical syntax
- quantum syntax
- hardware/resource syntax

Outputs

Parser contexts representing hybrid constructs.

Upstream contracts

It depends on stable parser rules for:

- identifiers
- qualified names
- expressions
- statements
- types
- blocks
- declarations
- quantum operations
- classical operations
- resource expressions

Downstream consumers

- parser/frontend
- AST construction
- semantic analyzer
- type checker
- effect checker
- capability checker
- resource analyzer
- compiler frontend

Public grammar contract

"hybrid.g4" must expose stable, documented entry rules for hybrid constructs.

The exact rule names become part of the grammar compatibility surface.

AST contract

Each hybrid construct must preserve:

- source span
- domain identity
- operation relationship
- operands
- results
- control relationships
- resource annotations
- relevant attributes
- explicit/implicit synchronization semantics

The grammar must not silently discard syntax that semantic analysis may require.

Semantic contract

The grammar distinguishes syntax from meaning.

For example:

classical -> quantum
quantum -> classical
classical <-> accelerator
quantum <-> accelerator

are syntactic relationships.

Whether a particular relationship is semantically legal is determined downstream.

IR integration

Hybrid syntax must lower into canonical semantic representations.

Quantum portions must ultimately integrate with:

quantum::ir

rather than a hybrid-specific duplicate quantum IR.

Classical portions must integrate with the repository's canonical classical representation.

The hybrid layer represents relationships between these semantic domains.

Compiler integration

The compiler must be able to:

1. parse hybrid syntax
2. construct the AST
3. perform semantic validation
4. lower classical portions
5. lower quantum portions
6. preserve cross-domain dependencies
7. produce the appropriate canonical IR
8. continue into optimization/routing/scheduling/hardware compilation

Runtime integration

Runtime behavior must not be encoded directly into grammar productions.

Runtime consumes compiled semantic representations and execution metadata.

Tooling integration

The grammar must support:

- syntax highlighting
- formatting
- source navigation
- diagnostics
- parser recovery
- IDE tooling
- source mapping
- syntax-aware refactoring

Cross-domain integration

The composition grammar must permit:

classical ↔ quantum
classical ↔ accelerator
quantum ↔ accelerator
classical ↔ quantum ↔ accelerator

without requiring machine-specific syntax.

Tests

Require:

- minimal hybrid program
- nested hybrid program
- multiple domain crossings
- parameterized interactions
- empty optional sections where legal
- large hybrid programs
- arbitrary identifiers
- deeply nested valid constructs

Negative tests

Reject:

- malformed domain transitions
- incomplete hybrid blocks
- invalid delimiters
- malformed expressions
- invalid syntax combinations

Semantic invalidity should be rejected by semantic analysis rather than incorrectly encoded as parser-only restrictions.

Boundary tests

Verify that no grammar limit exists on:

- number of hybrid blocks
- number of domain crossings
- number of operations
- nesting depth beyond parser/runtime implementation limits
- resource expressions

Compatibility

Changes to public rule names or construct shapes require language-version review.

Scalability

No:

MAX_QUBITS
MAX_ACCELERATORS
MAX_DEVICES
MAX_DOMAINS
MAX_OPERATIONS

or equivalent grammar limits.

Hard-coding audit

Search for:

- literal machine IDs
- fixed device names
- fixed qubit counts
- fixed accelerator counts
- fixed topology
- fixed memory
- fixed core counts

None may occur as scalability assumptions.

Completion criteria

Complete only when:

- all delegated grammar components have stable contracts
- no duplicate productions exist
- parser composition succeeds
- semantic boundary is documented
- AST mapping is defined
- cross-domain tests pass
- scalability audit passes
- compatibility rules are documented

---

7. "classical-quantum.g4"

Purpose

Defines syntax for interaction where classical computation and quantum computation exchange values, parameters, control information, or results.

Owns

- classical-to-quantum invocation relationships
- quantum-to-classical result relationships
- classical parameter binding for quantum operations
- measurement/result consumption
- explicit classical/quantum boundaries
- domain crossing expressions

Does Not Own

- general classical expressions
- quantum gate definitions
- quantum measurement implementation
- type checking
- runtime transport
- QEC
- scheduling

Inputs

Consumes existing:

- expression grammar
- type grammar
- function/call grammar
- quantum operation grammar
- measurement grammar
- identifiers
- blocks
- statements

Outputs

Parser contexts representing classical/quantum interaction.

AST contract

Preserve:

- source locations
- producer domain
- consumer domain
- values
- bindings
- operation references
- control dependencies
- result dependencies
- explicit synchronization markers where present

Semantic contract

The semantic layer determines:

- whether values are type-compatible
- whether classical data may influence a quantum operation
- whether quantum results may cross into classical computation
- whether an operation requires synchronization
- whether the operation is statically or dynamically controlled

IR integration

Quantum operations lower through the canonical quantum IR.

Classical values remain represented by the canonical classical semantic layer.

The crossing itself must become an explicit semantic dependency rather than an informal comment.

QEC integration

If the source references logical quantum operations or QEC-protected regions, this file only provides syntax.

QEC determines:

- code
- syndrome extraction
- decoding
- correction
- logical-state representation

The grammar does not implement those mechanisms.

ZQN integration

Noise/fault semantics are not defined here.

ZQN may later consume semantic execution information.

Scheduling integration

Cross-domain dependencies must be preserved so scheduling can determine valid execution order.

The grammar must not assign physical timestamps.

Runtime integration

Runtime determines actual synchronization, transport, buffering, and dispatch.

Tests

Test:

- classical parameter → quantum operation
- quantum measurement → classical value
- classical conditional → quantum operation
- quantum result → classical expression
- repeated crossings
- nested crossings
- large numbers of crossings

Negative tests

Reject syntactically incomplete crossings.

Do not encode backend-specific semantic restrictions as grammar errors.

Scalability

No fixed number of:

- classical values
- quantum values
- crossings
- operations
- registers

Completion criteria

Complete when every supported classical/quantum crossing has:

- grammar
- AST mapping
- semantic contract
- IR mapping
- scheduling dependency definition
- runtime boundary definition
- positive tests
- negative tests
- scalability tests

---

8. "quantum-classical-control.g4"

Purpose

Defines syntax for classical control over quantum computation, including dynamic-circuit-style control where supported by Zamani semantics.

Owns

- classical conditions attached to quantum execution
- classical branching around quantum operations
- measurement-dependent control syntax
- dynamic quantum/classical control relationships
- control expressions

Does Not Own

- general control-flow grammar
- classical boolean semantics
- quantum measurement semantics
- backend capability checking
- execution scheduling

Inputs

Depends on:

- expressions
- conditionals
- blocks
- quantum operations
- measurement
- identifiers
- types

AST contract

Preserve:

- condition expression
- controlled quantum operation
- source locations
- dependency relationships
- branch/block structure

Semantic contract

Semantic analysis determines:

- condition type
- availability of the referenced classical result
- whether dynamic control is permitted
- whether the target supports the required execution model

Capability integration

A backend lacking a required dynamic capability must be rejected or transformed by downstream compilation policy, not by a grammar hard-code.

Scheduling integration

The resulting semantic dependency must be visible to scheduling.

For example:

measurement
     ↓
classical result
     ↓
condition
     ↓
quantum operation

must remain explicit.

Runtime integration

Runtime executes the compiled control dependency.

QEC integration

QEC may produce or consume classical syndrome information.

This grammar only expresses the source-level control relationship.

Tests

Include:

- simple measurement-controlled operation
- nested control
- multiple conditions
- compound conditions
- classical values from multiple sources
- repeated dynamic regions

Negative tests

Reject malformed conditions and malformed controlled blocks.

Boundary tests

No fixed condition count or control nesting count.

Completion criteria

Complete when dynamic control is syntactically represented without embedding target-specific capability assumptions.

---

9. "accelerator-interoperability.g4"

Purpose

Defines syntax for portable interaction between Zamani computation and accelerator domains.

Accelerators may include:

- GPU
- FPGA
- ASIC-backed accelerator
- tensor accelerator
- AI accelerator
- DSP
- future accelerator classes

The grammar must remain generic.

Owns

- accelerator invocation syntax
- accelerator computation regions
- data transfer intent
- accelerator interface references
- accelerator capability requirements
- portable accelerator interoperability constructs

Does Not Own

- GPU hardware details
- FPGA architecture
- ASIC implementation
- device enumeration
- vendor-specific device IDs
- physical addresses
- memory topology
- kernel implementation

Critical scalability rule

Never encode:

GPU0
GPU1
GPU2

as intrinsic language architecture.

Nor:

MAX_GPUS
MAX_ACCELERATORS

nor any equivalent.

A source program can express:

requires accelerator

or an appropriate capability/resource expression.

The compiler/runtime resolves the actual resource.

AST contract

Preserve:

- accelerator role
- operation
- inputs
- outputs
- requirements
- constraints
- preferences
- capabilities
- data movement semantics

Resource integration

Accelerator requirements must integrate with the universal resource model.

A requirement is not a device selection.

Hardware integration

Hardware abstraction resolves:

logical accelerator requirement
        ↓
available capability
        ↓
target implementation

Compiler integration

Compilation may lower accelerator operations to:

- CPU implementation
- GPU implementation
- FPGA implementation
- ASIC implementation
- future target

when semantic compatibility permits.

Runtime integration

Runtime resolves actual execution resources.

Tests

Test accelerator-agnostic syntax.

Test heterogeneous combinations:

classical + accelerator
quantum + accelerator
classical + quantum + accelerator

Negative tests

Reject malformed accelerator regions.

Do not reject valid source merely because a particular hardware class is unavailable.

Completion criteria

Complete when accelerator syntax is independent of concrete hardware inventory.

---

10. "hybrid-resources.g4"

Purpose

Defines hybrid-specific resource syntax.

The central rule is:

«A source-level resource expression describes intent and requirements, not an inventory of currently available hardware.»

Owns

Syntax for:

- requirements
- constraints
- capabilities
- preferences
- hints
- targets where semantically appropriate
- performance requirements
- latency requirements
- reliability requirements
- energy requirements
- scalability requirements
- portability requirements

Does Not Own

- resource discovery
- resource allocation
- scheduling
- placement
- hardware inventory
- device selection policy
- runtime resource management

Required conceptual separation

These concepts must remain distinct:

requirement
constraint
capability
preference
hint
target
placement

For example:

requires quantum

must not inherently mean:

use device X

and:

requires accelerator

must not inherently mean:

use GPU 0

AST contract

Preserve resource expressions without prematurely resolving them.

Semantic contract

Semantic analysis determines whether expressions are:

- valid
- contradictory
- satisfiable
- target-independent
- target-specific
- advisory

Compiler integration

Compilation receives resource requirements and resolves them against a target/environment.

Runtime integration

Runtime may discover capabilities and negotiate execution.

Scheduling integration

Scheduling may consume relevant resource constraints after semantic lowering.

Hardware integration

Hardware HAL provides actual capabilities.

Resilience integration

Resilience may react when actual resources become unavailable or degraded.

The grammar itself does not perform recovery.

Scalability

Never define fixed resource counts.

Resource quantities must be represented through semantic expressions rather than parser limits.

Tests

Test:

- requirements
- constraints
- preferences
- hints
- multiple resources
- heterogeneous resource expressions
- scalable quantities
- symbolic/resource-derived quantities

Negative tests

Reject syntactically invalid resource expressions.

Semantic contradictions belong to semantic validation.

Completion criteria

Complete when resource syntax can describe arbitrary future resource classes without changing the grammar's foundational architecture.

---

11. Cross-File Integration Contract

The hybrid files integrate as follows:

                    hybrid.g4
                        │
          ┌─────────────┼──────────────┐
          │             │              │
          ▼             ▼              ▼
 classical-quantum   quantum-       accelerator-
      .g4            classical-       interop
                     control.g4         .g4
          │             │              │
          └─────────────┼──────────────┘
                        ▼
                 hybrid-resources.g4
                        │
                        ▼
                 Semantic Analysis
                        │
              ┌─────────┴─────────┐
              ▼                   ▼
       Classical semantics    quantum::ir
              │                   │
              └─────────┬─────────┘
                        ▼
                 Hybrid semantic
                    dependency
                        │
                        ▼
               Optimization / Routing
                        │
                        ▼
                   Scheduling
                        │
                        ▼
                 Hardware / Target
                        │
                        ▼
                    Runtime

No file may introduce a dependency in the reverse direction.

---

12. Integration With "grammar/quantum/"

Hybrid grammar consumes quantum syntax.

It must not duplicate:

- qubit grammar
- gate grammar
- operation grammar
- measurement grammar
- quantum type grammar
- dynamic-circuit grammar

Those remain owned by:

grammar/quantum/

Hybrid syntax references those constructs.

---

13. Integration With "grammar/classical/"

Hybrid grammar consumes classical syntax.

It must not redefine:

- expressions
- statements
- scalar types
- vectors
- matrices
- tensors
- classical functions
- control flow

Those remain owned by the classical/core grammar layers.

---

14. Integration With "grammar/hardware/"

Hybrid grammar may express hardware/accelerator requirements.

It must not define:

- actual devices
- topology
- physical placement
- addresses
- hardware inventory

Those remain hardware-layer concerns.

---

15. Integration With "grammar/resources/"

"hybrid-resources.g4" must reuse the universal resource vocabulary rather than inventing a competing resource model.

The preferred conceptual dependency is:

resources/
     │
     ▼
universal resource semantics
     │
     ▼
hybrid-resources.g4

Hybrid-specific syntax may specialize the relationship between resources and computational domains, but must not duplicate resource ownership.

---

16. Integration With Effects

Hybrid computation frequently crosses effect boundaries.

Effects may include:

- quantum
- hardware
- accelerator
- I/O
- distributed
- networking
- security

The grammar records syntactic effect declarations where appropriate.

Effect checking belongs to semantic analysis.

---

17. Integration With Types

Hybrid operations must preserve type information.

Examples include:

classical scalar
classical tensor
quantum state
measurement result
logical qubit
resource handle
accelerator value

The grammar only parses the type syntax.

The type system determines compatibility.

---

18. Integration With Concurrency

Hybrid computation may execute concurrently.

The grammar must therefore permit legal composition with:

- tasks
- futures
- parallel regions
- synchronization
- cancellation
- distributed execution

Concurrency semantics remain owned by "grammar/concurrency/" and the semantic/runtime layers.

---

19. Integration With Scheduling

Hybrid syntax must preserve all dependencies required for scheduling.

The grammar must never encode:

time = 10
duration = fixed
device = fixed
slot = fixed

unless such values are explicitly part of the program's semantics.

Scheduling determines actual execution timing.

---

20. Integration With Optimization

Hybrid constructs must lower into representations optimization can inspect.

Optimization may:

- fuse compatible operations
- eliminate redundant transfers
- specialize computation
- transform classical/quantum boundaries
- select equivalent implementations

The grammar must not perform optimization.

---

21. Integration With Routing

Routing resolves physical realization.

The hybrid grammar must not encode physical connectivity.

For example, a source-level quantum interaction does not specify:

physical qubit 3 ↔ physical qubit 7

unless explicit physical placement is genuinely part of the program's declared semantics.

---

22. Integration With "quantum::ir"

This is a mandatory boundary.

The flow is:

Hybrid syntax
     ↓
AST
     ↓
Semantic analysis
     ↓
Quantum semantic extraction
     ↓
quantum::ir

The hybrid grammar must never become an alternative quantum representation.

No hybrid grammar file may introduce a second:

- "QuantumGate"
- "QubitId"
- quantum operation representation
- physical-qubit identity system

where those concepts already belong to canonical quantum IR.

---

23. Integration With QEC

Hybrid programs may interact with error-corrected quantum computation.

The grammar may represent source-level concepts such as:

- logical quantum computation
- protected regions
- syndrome-related classical control
- correction-related control

but must not implement:

- stabilizer decoding
- syndrome extraction algorithms
- correction algorithms
- code-distance computation
- physical error models

Those belong to QEC and related semantic/IR layers.

---

24. Integration With ZQN

ZQN owns quantum noise/fault semantics.

Hybrid grammar must not create:

HybridNoiseModel
HybridFaultModel

merely because computation crosses domains.

If noise information is present in source syntax, it must integrate with the canonical ZQN model.

---

25. Integration With Resilience

Resilience operates above execution subsystems.

Hybrid syntax may provide information useful for resilience, such as:

- requirements
- reliability constraints
- retry-safe declarations where supported
- resource preferences

The grammar must not implement recovery.

Resilience decides whether to:

Retry
Restart
Resume
Rollback
Remap
Reroute
Reschedule
Recompile
Reoptimize
ChangeQEC
Mitigate
SwitchBackend
QuarantineResource
Abort

---

26. POCO-REAF Requirements

The hybrid grammar must satisfy all five dimensions.

Program Once

A developer describes the computation rather than a particular machine.

Compile Once

Compilation produces stable semantic representations wherever the target-independent semantics permit.

Run Everywhere

The same program may target different execution environments.

Run Anywhere

The execution environment may be:

- local
- embedded
- remote
- distributed
- cloud
- classical
- quantum
- heterogeneous

Run Forever

The grammar must remain extensible.

Future domains must be addable without rewriting existing hybrid semantics.

---

27. Future-Domain Extensibility

The hybrid architecture must not assume that only classical and quantum computation can exist.

Future domains may include:

- neuromorphic
- photonic
- analog
- molecular
- optical
- biological
- cryogenic
- probabilistic
- reconfigurable
- unknown future architectures

The grammar should therefore model domain interaction generically.

Avoid a design in which every new computational domain requires rewriting the fundamental hybrid grammar.

---

28. No Physical Resource Hard-Coding

The following are prohibited as intrinsic hybrid grammar limitations:

MAX_QUBITS
MAX_CORES
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_ACCELERATORS
MAX_NODES
MAX_DEVICES
MAX_MEMORY
MAX_REGISTERS
MAX_DOMAINS

Also prohibited:

- fixed device IDs
- fixed topology
- fixed hardware addresses
- vendor-specific physical assumptions
- fixed cluster sizes
- fixed accelerator inventory

Quantities appearing in source programs must represent program semantics, constraints, requirements, or explicit resource expressions—not parser-imposed ceilings.

---

29. Resource Scaling

A hybrid program must be able to express resource quantities that scale with available resources.

Examples of valid conceptual forms include:

requires resources(...)
requires capability(...)
prefers resource(...)
constrains resource(...)

The actual syntax must follow the canonical resource grammar.

The grammar must not turn a resource quantity into a fixed compile-time architecture assumption unless the source explicitly declares such semantics.

---

30. Diagnostics

Parser diagnostics must identify:

- unexpected token
- incomplete hybrid construct
- malformed domain crossing
- malformed control expression
- malformed resource expression
- missing delimiter
- invalid syntactic nesting

Diagnostics should include:

- source span
- offending token
- expected syntax
- stable diagnostic identifier where supported
- actionable explanation

Semantic errors must remain distinguishable from syntax errors.

---

31. Error Recovery

ANTLR parser recovery must not silently convert malformed hybrid syntax into valid semantics.

Recovery should:

- preserve source locations
- avoid cascading diagnostics where practical
- recover at well-defined synchronization points
- never fabricate machine resources
- never fabricate quantum operations
- never fabricate classical results

---

32. Determinism

Given identical source and grammar version:

source → lexer → parser

must produce deterministic syntax structures.

No grammar production may depend on:

- current hardware
- runtime state
- device availability
- network state
- random values
- current time

---

33. Security

The grammar must not provide implicit authority to:

- access hardware
- access files
- access networks
- access devices
- execute arbitrary native code

Syntax describing such capabilities must still pass through semantic capability/security validation.

No grammar construct should bypass the security model.

---

34. Versioning

Hybrid syntax must be version-aware through the canonical Zamani language-version mechanism.

Do not independently invent incompatible hybrid versions.

Version changes must distinguish:

- additive syntax
- syntax correction
- semantic change
- deprecated syntax
- removed syntax

Backward-compatible syntax should remain parseable according to the documented compatibility policy.

---

35. Vendor Extensions

Vendor-specific hybrid features must not contaminate the portable core grammar.

Use the dialect/extension mechanism for:

- vendor-specific operations
- vendor-specific accelerator features
- vendor-specific quantum capabilities
- vendor-specific execution hints

Portable Zamani syntax remains independent.

---

36. Testing Architecture

Tests must exist at multiple levels.

Unit grammar tests

Each grammar file receives focused tests.

Integration tests

Test complete hybrid programs.

Cross-domain tests

Required combinations include:

classical + quantum
classical + accelerator
quantum + accelerator
classical + quantum + accelerator
classical + quantum + HDL
quantum + hardware
quantum + distributed
classical + quantum + distributed
AI + quantum
AI + accelerator

Negative tests

Malformed syntax must fail predictably.

Boundary tests

Test:

- minimal programs
- large programs
- deeply nested hybrid regions
- many domain crossings
- large resource expressions
- large identifier sets
- large expression graphs

The tests must verify absence of arbitrary language-level limits.

---

37. Round-Trip Tests

Where Zamani provides a canonical printer/serializer:

Source
  ↓
Lexer
  ↓
Parser
  ↓
AST
  ↓
Printer
  ↓
Parser

must preserve intended semantics.

Formatting differences are acceptable where the language permits them.

Semantic changes are not.

---

38. Hard-Coding Audit

Every hybrid grammar change must be audited for:

Machine assumptions

- CPU counts
- GPU counts
- accelerator counts
- quantum processor sizes

Quantum assumptions

- qubit counts
- register limits
- topology
- fixed gate connectivity

Hardware assumptions

- addresses
- device IDs
- ports
- physical resources

Runtime assumptions

- fixed execution environments
- fixed queue sizes
- fixed deployment sizes

Parser assumptions

- fixed construct counts
- fixed domain counts
- artificial nesting restrictions

Every finding must be classified as:

1. language semantic requirement
2. target-specific requirement
3. resource constraint
4. implementation limitation
5. accidental hard-coding
6. test-only limitation
7. documentation-only limitation

Accidental hard-coding must be removed.

---

39. Rust Requirements

The grammar infrastructure integrated with this directory must remain compatible with:

Rust 1.97
Rust 1.97.1

No "unsafe" Rust is permitted.

Grammar files themselves are ANTLR grammar specifications, but all generated/runtime integration must respect the repository's Rust safety and dependency policies.

Generated code must not be modified manually unless the repository's generation workflow explicitly requires controlled patches.

---

40. ANTLR Requirements

The hybrid grammar must:

- use the canonical Zamani lexer architecture
- avoid duplicated token definitions
- avoid duplicated keyword definitions
- avoid token-name collisions
- avoid unreachable productions
- avoid ambiguous alternatives where practical
- document unavoidable ambiguities
- preserve source locations
- provide deterministic parsing

Grammar composition must be compatible with the repository's ANTLR generation process.

---

41. Dependency Direction

The allowed dependency direction is:

lexer
  ↓
core
  ↓
types
  ↓
expressions
  ↓
statements
  ↓
declarations/functions/modules
  ↓
effects/memory/concurrency
  ↓
classical/quantum
  ↓
hybrid
  ↓
HDL/hardware/distributed/AI/etc.
  ↓
compile/execution/interoperability

Hybrid must not create a dependency cycle back into core lexical definitions.

---

42. File Completion Rule

A hybrid file is not complete merely because ANTLR accepts it.

A file is complete only when:

- syntax is defined
- ownership is explicit
- dependencies are stable
- AST contract is defined
- semantic contract is defined
- IR integration is defined
- compiler integration is defined
- runtime boundary is defined
- cross-domain behavior is defined
- diagnostics are defined
- compatibility is defined
- scalability is audited
- hard-coding is audited
- positive tests exist
- negative tests exist
- boundary tests exist
- integration tests exist where applicable
- documentation is consistent

---

43. Definition of Done

The entire "grammar/hybrid/" directory is production-ready only when:

- all five grammar files compile
- there are no duplicate ownership definitions
- the canonical lexer is used
- parser composition is deterministic
- classical/quantum boundaries are explicit
- quantum syntax integrates through "quantum::ir"
- resource syntax integrates with the universal resource model
- accelerator syntax is target-independent
- dynamic classical/quantum control is represented correctly
- AST contracts are stable
- semantic boundaries are documented
- compiler integration is verified
- scheduling dependencies are preserved
- hardware details remain outside portable semantics
- runtime details remain outside grammar
- QEC remains outside grammar implementation
- ZQN remains outside grammar implementation
- resilience remains outside grammar implementation
- no unsafe Rust is required
- no artificial scalable-resource limits exist
- compatibility behavior is documented
- positive tests pass
- negative tests pass
- boundary tests pass
- cross-domain tests pass
- determinism tests pass
- round-trip tests pass where supported
- hard-coding audit passes

---

44. Implementation Order

The files should be implemented in this order:

1. hybrid-resources.g4
        ↓
2. classical-quantum.g4
        ↓
3. quantum-classical-control.g4
        ↓
4. accelerator-interoperability.g4
        ↓
5. hybrid.g4
        ↓
6. hybrid test suite
        ↓
7. repository-wide grammar integration

Why

"hybrid-resources.g4" establishes the target-independent resource vocabulary used by the other hybrid constructs.

"classical-quantum.g4" establishes the primary cross-domain data boundary.

"quantum-classical-control.g4" builds on classical/quantum values and control.

"accelerator-interoperability.g4" extends the domain-crossing model to heterogeneous accelerators.

"hybrid.g4" is implemented last because it is the composition layer and should not force later structural changes to its delegates.

---

45. Integration Freeze

Before beginning "hybrid.g4", the following contracts must be frozen:

identifier syntax
expression syntax
type syntax
statement syntax
classical operation syntax
quantum operation syntax
measurement syntax
resource expression syntax
capability syntax
effect syntax

Once frozen, "hybrid.g4" should only compose them.

This prevents the common failure mode where the composition grammar becomes a second owner of lower-level syntax.

---

46. Architectural Invariants

The following invariants must never be violated.

Invariant 1

Grammar describes syntax.

Invariant 2

AST represents parsed source structure.

Invariant 3

Semantic analysis determines meaning.

Invariant 4

Canonical IR represents compiler semantics.

Invariant 5

"quantum::ir" remains the canonical quantum semantic boundary.

Invariant 6

QEC owns quantum error correction.

Invariant 7

ZQN owns quantum noise/fault semantics.

Invariant 8

Optimization owns optimization.

Invariant 9

Routing owns physical realization.

Invariant 10

Scheduling owns temporal/resource ordering.

Invariant 11

Hardware HAL owns hardware capabilities and state.

Invariant 12

Runtime owns execution.

Invariant 13

Resilience owns adaptation/recovery decisions.

Invariant 14

Portable source syntax must not depend on a particular machine.

Invariant 15

No scalable resource may have an arbitrary grammar-imposed maximum.

---

47. POCO-REAF Acceptance Test

The definitive hybrid-language test is:

«Can the same hybrid source program retain the same semantic meaning while being compiled for different available combinations of classical processors, quantum processors, accelerators, hardware resources, and distributed environments?»

The answer must be yes, provided the target environment satisfies the program's semantic requirements.

The source must not need to be rewritten merely because:

number of qubits changes
number of CPU cores changes
number of GPUs changes
number of accelerators changes
hardware topology changes
machine size changes
deployment size changes
quantum backend changes
classical backend changes
accelerator backend changes

Target-specific constraints belong to target/resource/compilation configuration.

---

48. Final Principle

"grammar/hybrid/" is the language boundary for heterogeneous computation.

It must enable:

Classical
    ↕
Quantum
    ↕
Accelerator
    ↕
Hardware
    ↕
Distributed execution
    ↕
Future computational domains

without turning any particular machine into the definition of the language.

The governing principle is:

«Zamani describes what computation means and what capabilities it requires; downstream compilation and execution determine how that computation is realized on the resources actually available.»

Therefore:

One Program
     ↓
One Semantic Meaning
     ↓
One Portable Compilation Model
     ↓
Many Architectures
     ↓
Many Hardware Configurations
     ↓
Many Scales
     ↓
Many Execution Environments
     ↓
Future Platforms

This is the required hybrid-grammar foundation for:

Zamani — From Atom to Everywhere

and:

POCO-REAF

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
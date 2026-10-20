Zamani HDL Intermediate Representation (HDLIR) Specification

Canonical path: "src/hdl/hdlir/HDLIR-SPEC.md"
Repository: "Benwellonedge28/Zamani"
Status: Normative architecture and implementation contract
Language: Zamani
Rust edition: 2021
Minimum Rust version: 1.97 or later
Safety: Safe Rust only; "unsafe" is prohibited
Primary objective: "Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever" (POCO-REAF)
Scope: Hardware-domain IR semantics, data model, identity, symbolic types and values, behavioral and structural hardware, verification, analysis, transformations, lowering, serialization, dialects, interoperability, scalability, determinism, security, testing, and repository integration.

Implementation status: This document specifies required behavior. It does not claim that the specified implementation, adapters, passes, or target backends already exist.

---

1. Purpose

HDLIR is Zamani's canonical hardware-domain intermediate representation. It represents validated hardware computation, structure, behavior, connectivity, temporal state, and portable hardware implementation intent independently of a particular physical realization.

HDLIR MUST support hardware descriptions ranging from small combinational circuits to large hierarchical designs, subject to available resources, supported semantics, representable values, and explicitly configured operational budgets.

HDLIR MUST support the architectural objective:

Program once → establish semantic identity → compile through compatible representations → realize on compatible targets → preserve the declared meaning across environments and future implementations.

The objective is not to guarantee that every design can run on every machine. It is to ensure that a design is not unnecessarily tied to the machine available when it was written, and that a target which cannot satisfy its requirements is rejected or reported as infeasible without silently changing its meaning.

HDLIR MUST support the progressive addition of hardware models and implementation strategies without requiring unrelated parts of Zamani to be rewritten merely because a new hardware technology becomes available.

This specification defines the contract. Implementations MUST be demonstrated through code, tests, and integration evidence before a feature is described as implemented or production-ready.

2. Normative language

The terms MUST, MUST NOT, REQUIRED, SHOULD, SHOULD NOT, and MAY are normative.

- MUST / MUST NOT: Mandatory for conformance.
- SHOULD / SHOULD NOT: Expected unless a documented technical justification establishes an exception.
- MAY: Permitted under the applicable contract.
- Semantic preservation: Retaining the behavior and observable properties required by the source program and its contracts.
- Portable intent: Hardware computation and requirements expressed independently of unnecessary target-specific assumptions.
- Realization: Mapping a validated design to an implementation on a concrete target.
- Incomplete result: A result for which one or more required computations or checks did not finish.
- Dialect: A registered, versioned extension with explicit semantics and compatibility rules.
- Operational budget: A configurable limit on compiler work, memory consumption, recursion, verification, or another implementation activity. It is not a universal language-level hardware limit.

3. Repository authority and integration

3.1 Normative authority

HDLIR MUST conform to the existing repository's language and architecture authorities.

Authority| Responsibility
"grammar/DESIGN.md"| Repository-wide grammar architecture, language boundaries, safety, and ownership.
"grammar/spec/hdl.md"| HDL semantics, portability, requirements, constraints, and hardware-domain boundaries.
"grammar/Zamani.g4"| Canonical language grammar composition.
"grammar/hdl/hdl.g4"| HDL grammar composition and HDL syntax integration.
"grammar/antlr/ZamaniLexer.g4"| Public ANTLR lexer boundary.
"grammar/lexer/tokens.g4"| Canonical token-composition boundary.
Applicable grammar delegates under "grammar/hdl/"| Syntax owned by individual HDL constructs.
"grammar/specification/semantic-model.md"| Source-language meaning, effects, requirements, capabilities, and semantic ownership.
"grammar/validation/semantic-boundaries.md"| Separation of syntax, semantics, compilation, and realization.
"src/frontend/ast/"| Canonical frontend AST and source locations.
"src/frontend/semantic/"| Canonical semantic-analysis and semantic-model boundary, according to the repository's current migration state.
"src/hdl/api.rs"| Existing public HDL parser and validation API.
"src/hdl/ids.rs"| Existing HDL identity integration and reuse of canonical identities.
"src/hdl/"| HDL composition root and hardware-domain integration.
"src/zuir/ZUIR-SPEC.md"| Universal intermediate representation and cross-domain representation contracts.
Existing downstream realization modules| Their actual, verified interfaces for optimization, scheduling, allocation, placement, routing, and target-specific lowering.

This table establishes authority; it does not assert that every listed component is complete or that every future adapter already exists.

When two specifications conflict, implementation MUST NOT silently choose whichever interpretation is convenient. The conflict MUST be documented, the relevant owners identified, and the authoritative specification updated or the implementation contract explicitly reconciled before dependent APIs are frozen.

3.2 HDLIR owns

All HDLIR-owned implementation MUST reside under "src/hdl/hdlir/".

This includes, when implemented:

- HDLIR design, module, port, operation, value, region, block, and connectivity representations.
- HDL-specific logical types and symbolic hardware-width descriptions.
- Hardware constants, symbolic parameters, and elaboration-independent expressions.
- Behavioral processes, sequential state, clocks, resets, and abstract memory semantics.
- The HDLIR schema and dialect registration interfaces.
- Checked construction APIs.
- Structural, type, connectivity, process, and clock verification.
- HDL-domain analyses and explicitly contracted transformations.
- HDLIR serialization, artifact compatibility, and fingerprints.
- HDL-specific lowering and interchange adapters.
- HDLIR conformance, malformed-input, scalability, and integration tests.

Subdirectories MUST be created when they establish meaningful ownership boundaries and are needed for maintainability. Speculative empty directories MUST NOT be created merely to make a proposed tree look complete.

3.3 HDLIR does not own

HDLIR MUST NOT become the authoritative owner of:

- Source tokenization, grammar productions, parser recovery, or AST construction.
- General source name resolution or the universal semantic model.
- A replacement for canonical source spans, diagnostics, or semantic identity allocators.
- A second universal IR competing with ZUIR.
- A second quantum IR competing with "src/quantum/ir/".
- Physical FPGA pins, ASIC cells, calibrated device properties, or runtime device handles as portable semantic identities.
- Hardware discovery, target-provider access, deployment credentials, or runtime resource ownership.
- Physical placement, routing, clock-tree synthesis, technology mapping, or timing closure.
- Backend scheduling algorithms or physical resource allocation.
- A promise that a valid HDLIR design is synthesizable or executable by every target.

HDLIR may describe logical requirements and constraints relevant to those stages. The stages responsible for satisfying them remain downstream.

3.4 Dependency direction

The intended dependency direction is:

Zamani source
    |
    v
Canonical lexer and grammar
    |
    v
Canonical frontend AST
    |
    v
Semantic analysis and HDL validation
    |
    +------------------------+
    |                        |
    v                        v
ZUIR core              HDL-domain facts
    |                        |
    +------------+-----------+
                 |
                 v
         src/hdl/hdlir/
                 |
                 v
     HDL analysis and optimization
                 |
                 v
       scheduling and planning
                 |
                 v
   target-specific lowering and mapping
                 |
                 v
 placement / routing / realization
                 |
                 v
 hardware, simulator, or supported target

This diagram describes architectural responsibilities, not a claim that every edge has already been implemented.

A conversion between ZUIR and HDLIR MUST have a defined semantic contract. A stage MUST NOT be introduced into the dependency graph until its required input and output types and failure modes are known.

HDLIR MUST NOT require the frontend parser or semantic analyzer to depend on physical realization merely to parse or validate a source program.

4. POCO-REAF and semantic portability

4.1 Fundamental invariant

The fundamental invariant is:

The meaning of a program is not the same thing as the physical resources available to implement it.

The same validated design may be considered for different FPGAs, ASIC flows, accelerators, hardware simulators, reconfigurable systems, and future hardware architectures.

Different realizations MAY differ in internal representation, implementation strategy, scheduling, physical organization, resource usage, and performance. They MUST preserve the design's required observable semantics and declared contracts.

A target that cannot satisfy a required property MUST produce an explicit infeasibility, unsupported-capability, or lowering failure. It MUST NOT silently drop the requirement or substitute a behaviorally different implementation.

4.2 Requirements and implementation guidance

The following categories MUST remain distinct:

- Requirement: A condition necessary for a valid realization.
- Capability: A property or operation that an implementation target can support.
- Constraint: A condition a permitted realization must satisfy.
- Preference: A non-mandatory optimization or selection criterion.
- Hint: Optional implementation guidance that cannot change semantics.
- Policy: A rule controlling permitted implementation, resource use, authorization, or execution.
- Contract: A verifiable assumption, guarantee, invariant, precondition, or postcondition.

A preference MUST NOT be treated as a hard requirement unless the governing contract explicitly says so. A hint MUST NOT authorize a semantic change. A missing capability MUST NOT automatically invalidate otherwise valid source semantics.

4.3 Portability boundaries

Portable HDLIR MUST distinguish logical hardware intent from target-specific commitments.

Examples of portable intent include:

- A signal with a declared logical type and width.
- A module with a parameterized interface.
- A clocked state transition with explicit reset behavior.
- A memory with specified read, write, initialization, and collision semantics.
- A requirement for a minimum throughput or maximum latency.
- A requirement for a particular supported operation or capability.

Examples of target-specific commitments include:

- A vendor-specific primitive.
- A particular physical pin or device identifier.
- A selected FPGA resource site.
- A technology library cell.
- A placement or routing decision.
- A calibrated physical timing value.

Target-specific information MAY be represented through explicitly registered extensions or physical-intent dialects. It MUST NOT silently become a universal semantic assumption.

5. Scalability and the meaning of "no hard-coded limits"

5.1 No arbitrary universal hardware ceilings

HDLIR MUST NOT introduce fixed universal hardware maxima such as:

"MAX_SIGNALS", "MAX_PORTS", "MAX_MODULES", "MAX_INSTANCES", "MAX_WIDTH", "MAX_CLOCK_DOMAINS", "MAX_MEMORIES", "MAX_PIPELINE_STAGES", "MAX_DEVICES", or equivalent constants that restrict otherwise valid designs without a semantic or representational justification.

No implementation may assume that every design fits in a fixed amount of host RAM, that every target has a fixed register width, or that all host and target integer widths are identical.

5.2 Symbolic representation

Quantities MUST remain symbolic when their values are not yet known or when evaluating them would be premature.

Examples include:

- Parameterized bit widths.
- Parameterized array and aggregate dimensions.
- Module parameters.
- Symbolic resource requirements.
- Clock-period and throughput expressions.
- Memory dimensions.
- Elaboration-dependent indices and sizes.

A symbolic expression MUST NOT be forced into a concrete host integer simply because a downstream implementation currently uses one.

When a stage needs a concrete value, it MUST evaluate the expression using the owning semantic rules, check representability, and report unresolved expressions or overflow explicitly.

5.3 Finite representation versus semantic limits

All actual implementations use finite storage, identifiers, and execution resources. HDLIR MUST distinguish those engineering facts from universal language-level capacity restrictions.

A concrete identifier encoding MAY have a finite range. If so:

1. The range and exhaustion behavior MUST be documented.
2. Allocation MUST use checked arithmetic.
3. Overflow MUST NOT wrap.
4. Issued identities MUST NOT be silently reused within their validity scope.
5. Exhaustion MUST produce an explicit error or incomplete result.
6. Symbolic quantities MUST remain available where a concrete integer is not required.

The existence of a finite ID encoding MUST NOT be used to justify a fixed maximum number of hardware signals or modules in the language specification.

5.4 Explicit operational budgets

Operational budgets MAY limit activities such as:

- Source or artifact decoding.
- Memory consumption.
- Analysis work.
- Verification work.
- Transformation work.
- Recursion or nesting.
- Diagnostic production.
- Elaboration.
- Canonicalization or serialization.

Budgets MUST be explicit inputs to the responsible operation or its documented configuration. Their defaults, if any, MUST be documented as operational defaults rather than language-semantic ceilings.

When a required activity exhausts its budget, the result MUST distinguish exhaustion from successful completion. The implementation MUST NOT truncate required work and report complete verification.

A caller MAY choose an unlimited budget where the API and environment permit it. This does not promise infinite execution or infinite memory.

5.5 Resource-proportional behavior

Implementations SHOULD process designs incrementally and avoid unnecessary duplication of large graphs or artifacts. They SHOULD support streaming or bounded-memory paths where practical and semantically appropriate.

Algorithms MUST document their complexity and material allocation behavior when those properties affect production use.

Stress tests MUST distinguish:

- Semantic invalidity.
- Unsupported representation or schema.
- Concrete value overflow.
- Operational-budget exhaustion.
- Host resource exhaustion.
- Target infeasibility.

These are different outcomes and MUST NOT be collapsed into one generic "design too large" result.

6. Core representation model

HDLIR MUST define a coherent representation for the following concepts.

6.1 Design

A design is the top-level HDLIR artifact and owns or references a well-defined set of logical modules, declarations, schema information, and design metadata.

The design MUST define:

- Its identity and identity scope.
- The schema and dialect versions required to interpret it.
- Its module and top-level entry-point relationships.
- Its semantic and source provenance.
- Its verification state or the means to obtain verification results.
- Its compatibility and serialization contract.

A design identity MUST NOT be confused with a physical device identity or a persistent cross-build identity unless a separate explicit contract establishes that property.

6.2 Module

A module is a reusable logical hardware unit with a signature, parameters, declarations, and a body or implementation reference.

Module hierarchy MUST represent logical composition, not physical device topology.

Modules MAY be generic or parameterized. Specialization MUST be an explicit, checked transformation and MUST preserve the module's declared semantics for valid parameter bindings.

Recursive or cyclic module references MUST follow a defined rule. If a form is prohibited by the applicable HDL semantics, verification MUST reject it with a structured diagnostic. If a form is supported, its elaboration and termination requirements MUST be explicit.

6.3 Ports and interfaces

Ports represent typed logical boundaries between modules or between a design and its environment.

Port contracts MUST specify:

- Identity and name-resolution provenance.
- Direction and any permitted bidirectional behavior.
- Logical type and width.
- Ordering rules where order is semantically significant.
- Parameter-dependent dimensions.
- Connection and compatibility rules.
- Relevant clock, reset, protocol, or timing contracts.

Port order MUST NOT depend on unspecified map iteration or filesystem traversal order.

A port is not a physical pin. Pin assignment belongs to target-specific realization.

6.4 Operations

An operation is a registered hardware-domain computation or structural action.

An operation instance MUST carry enough information to interpret its semantics under its registered operation definition, including as applicable:

- Operation identity.
- Schema or dialect identity and version.
- Operand and result references.
- Attributes and type parameters.
- Region or block relationships.
- Declared effects.
- Source provenance.
- Verification and interface obligations.

The core MUST NOT hard-code a closed universal enumeration of every operation that could ever exist.

Operation definitions MUST be registered through a governed schema or dialect mechanism. A registry MUST define duplicate-registration behavior, version compatibility, definition lookup, and validation.

Unknown operations MUST NOT be treated as understood operations. They MAY be preserved opaquely only when the artifact contract explicitly permits safe round-trip preservation without semantic interpretation. Any operation required for execution, verification, or transformation that cannot be interpreted MUST produce an explicit unsupported-feature outcome.

6.5 Values, signals, and state

HDLIR MUST distinguish at least these semantic categories:

- Computed value: A value produced by an operation.
- SSA value: A value defined once within the scope and dominance rules of an SSA-style region, if that region uses SSA.
- Logical signal: A hardware-level connection or named value whose behavior is governed by hardware semantics.
- State element: State retained across a defined event or transition.
- Memory: State with explicit access and collision semantics.
- External interface value: A value supplied or observed through a module or design boundary.

These categories MUST NOT be collapsed into one generic integer ID or one unqualified value representation if doing so loses their distinct semantics.

SSA-like dataflow MAY be used where it fits the relevant operation dialect. It MUST NOT erase persistent state, event-driven behavior, clocked updates, or hardware signal identity.

A register is not merely an ordinary temporary value. A memory is not merely an unbounded collection of SSA values. A wire is not necessarily a sequential state element.

The specification of every dialect MUST identify which value and state categories it uses and the applicable validity rules.

6.6 Regions and blocks

Regions and blocks provide structured scopes for operations where required by a dialect.

Their contracts MUST define:

- Ownership and containment.
- Legal operation placement.
- Block argument and result behavior, where applicable.
- Terminator rules.
- Control-flow and region-entry semantics.
- Value visibility, dominance, and use restrictions where applicable.
- Nesting and recursion behavior.
- Verification requirements.

A design MUST NOT rely on accidental object addresses or mutable host-language references to establish semantic identity.

7. Type system and symbolic hardware widths

7.1 Type authority

HDLIR owns hardware-domain IR type descriptions only to the extent that they are necessary to represent validated hardware semantics.

The canonical frontend semantic model remains authoritative for source-level type meaning. HDLIR MUST reuse or map those types through explicit conversion contracts rather than independently redefining source-language type checking.

7.2 Bit widths

Bit-width representation MUST distinguish:

- A concrete, evaluated width.
- A symbolic width expression.
- An unresolved width whose dependencies are known.
- An invalid width expression.
- A width whose evaluation exceeds the current representation or operational budget.

A concrete width MUST be positive or otherwise satisfy the exact zero-width rule defined by the owning HDL specification. Implementations MUST NOT silently choose a rule for zero-width values.

Width addition, multiplication, indexing, slicing, offset calculation, and conversion MUST use checked operations. Arithmetic overflow MUST be reported rather than wrapped.

HDLIR MUST support symbolic width relationships until the responsible elaboration or specialization stage can legally resolve them.

The current HDL semantic API exposes "HdlWidth" and documents a concrete-width representation based on "u128". HDLIR integration MUST respect the actual API contract. It MUST NOT assume that arbitrary-precision concrete widths are already supported by that validator. If the existing API cannot express a required width, the limitation MUST be documented and a coordinated semantic/API migration designed before the feature is claimed as supported.

Symbolic widths and concrete-width representation limits are different concerns.

7.3 Signedness and bit patterns

Signedness MUST be explicit wherever it affects interpretation or operations.

HDLIR MUST distinguish a bit pattern from its signed or unsigned interpretation. It MUST define truncation, extension, comparison, shifts, arithmetic overflow, and conversion behavior for each supported operation.

A transformation MUST NOT silently change signedness or discard significant bits.

7.4 Logic states

The type system MUST specify the supported logic domains, including whether a type uses two-state or four-state logic.

If unknown or high-impedance states are supported by the relevant dialect, their semantics MUST be preserved through operations, transformations, simulation, and export. A backend that cannot represent those states MUST report an explicit incompatibility or apply an explicitly permitted conversion contract.

It MUST NOT silently collapse semantically meaningful logic states.

7.5 Aggregate types

Aggregate types MAY include arrays, tuples, records, packed or unpacked structures, and other constructs defined by the HDL specification.

Their dimensions, field counts, and nesting MUST NOT be constrained by arbitrary universal hardware maxima.

Any concrete host representation limit MUST be checked and reported. Symbolic dimensions MUST remain symbolic until evaluation is required.

7.6 Clocks and resets as types or contracts

Clock and reset information MUST retain its declared semantics, including applicable polarity, active level, edge behavior, reset behavior, and association with stateful operations.

The representation MAY use dedicated types, attributes, interfaces, or typed contracts, but their ownership and verification rules MUST be explicit.

HDLIR does not own clock-tree synthesis, physical clock allocation, or synchronizer insertion.

7.7 Memory types

Abstract memory types MUST represent logical shape, element type, address interpretation, port contracts, read/write behavior, initialization semantics, and collision behavior as required by the language.

Memory types MUST NOT assume a particular physical memory technology, bank count, device capacity, or vendor primitive.

8. Constants, expressions, and parameters

8.1 Constants

Constants MUST have canonical, unambiguous representations for their declared types.

Constant encoding MUST preserve exact bit width, signedness, and supported logic states. Invalid encodings MUST be rejected explicitly.

Canonical equality MUST distinguish semantic equality from equality of incidental serialization or memory layout.

8.2 Symbolic expressions

Symbolic expressions MAY describe widths, dimensions, parameters, compile-time values, and other expressions permitted by the HDL specification.

The expression system MUST define:

- Supported operators and operand types.
- Dependency tracking.
- Evaluation context.
- Checked arithmetic.
- Division-by-zero and invalid-operation behavior.
- Unresolved-symbol behavior.
- Deterministic evaluation where required.
- Provenance for derived values.
- Operational-budget exhaustion.

HDLIR MUST NOT create a second general-purpose source expression language where the canonical AST or semantic model already owns that meaning. A distinct IR expression representation is permissible only when its conversion and semantic relationship to the canonical expression are explicit.

8.3 Parameters and specialization

Parameters MUST define their identity, type, default, constraints, and binding rules.

Specialization MUST:

1. Validate parameter bindings.
2. Evaluate only expressions permitted by the relevant semantic contract.
3. Use checked arithmetic.
4. Preserve unresolved expressions when resolution is not required.
5. Report unsatisfied constraints explicitly.
6. Preserve provenance from the generic design to the specialized result.
7. Avoid silently mutating the original design unless mutation is explicitly part of the API contract.

9. Structural connectivity

9.1 Connections

Connectivity represents logical relationships among ports, signals, operation results, module instances, and other connectable entities.

Every connection MUST have a well-defined source, destination or endpoint set, type relationship, and direction rule.

The model MUST specify whether a connection denotes a continuous assignment, structural binding, resolved net, or another relationship. These meanings MUST NOT be inferred from incidental data structure layout.

9.2 Driver and sink rules

Verification MUST check the driver and sink rules defined by the applicable dialect, including:

- Undriven required inputs.
- Dangling references.
- Illegal multiple drivers.
- Incompatible widths or types.
- Invalid directionality.
- Illegal connections across module boundaries.
- Unsupported resolution behavior.
- Invalid or missing declarations.

Multiple-driver behavior MUST be explicit. A design with legal resolved nets MUST NOT be rejected merely because it has multiple drivers if the selected dialect defines those semantics. Conversely, an ordinary single-driver connection MUST NOT silently acquire wired-resolution semantics.

9.3 Hierarchy and connectivity

Module instances MUST bind actual connections to declared interface contracts.

Hierarchy verification MUST validate module references, parameter bindings, port completeness, type compatibility, and permitted recursive structure.

Physical hierarchy, placement, and routing are downstream responsibilities.

10. Behavioral and sequential semantics

10.1 Processes

Behavioral processes MUST have explicit execution and update semantics.

The process model MUST distinguish the relevant kinds of behavior, such as:

- Combinational evaluation.
- Clock-triggered evaluation.
- Event-triggered evaluation.
- Sequential state updates.
- Initialization behavior where supported.
- Other registered behavioral extensions.

Each process kind MUST specify its legal statements, triggers, assignments, effects, and verification rules.

10.2 Assignment semantics

Where the source language distinguishes blocking, nonblocking, continuous, or other assignment categories, HDLIR MUST preserve those distinctions.

A transformation MUST NOT reorder assignments merely because the resulting graph remains structurally valid.

It MUST preserve read-after-write relationships, observable state transitions, and any other behavior required by the applicable language semantics.

10.3 Clocked state

Sequential behavior MUST specify:

- Triggering clock and edge.
- Reset condition and reset priority.
- Initial or reset state where defined.
- State-update behavior.
- Relevant dependencies and ordering.
- Observable outputs and effects.

Clocked behavior MUST NOT be modeled solely as ordinary combinational control flow.

10.4 Multiple clock domains

The representation MUST allow multiple logical clock domains without a fixed universal domain count.

Verification SHOULD identify invalid clock references, incompatible clock relationships, and crossings that violate declared safety contracts.

Clock-domain analysis MUST distinguish a detected crossing from proof that the crossing is safe. The analysis MUST NOT claim that a synchronizer exists or is sufficient unless the corresponding implementation evidence establishes it.

Physical clock-tree design, timing closure, and synchronizer insertion remain downstream.

10.5 Memories

Memory behavior MUST preserve:

- Read and write access semantics.
- Port relationships.
- Address and element types.
- Read-during-write behavior.
- Write collision behavior.
- Initialization and reset behavior where specified.
- Observable state transitions.

A memory implementation MAY later become registers, SRAM, BRAM, distributed memory, an external memory, or another technology. The realization MUST satisfy the abstract memory contract.

10.6 Timing and event semantics

HDLIR MUST distinguish semantic timing from implementation timing.

Timing that is part of the source program's required behavior MUST be represented through an explicit semantic contract.

Implementation estimates, clock periods, delays, throughput targets, and timing constraints MUST be labeled according to their meaning and authority. An estimate MUST NOT be represented as a guaranteed physical delay.

Event ordering, concurrency, and scheduling semantics MUST be specified sufficiently to determine which reorderings are valid.

11. Schema, operations, and dialect extensibility

11.1 Schema registry

The schema subsystem MUST define registration and lookup contracts for operation definitions, type definitions, interfaces, and dialect metadata.

It MUST specify:

- Identity and namespace rules.
- Version compatibility.
- Duplicate registration.
- Definition replacement or immutability.
- Dependency resolution.
- Required versus optional extensions.
- Unknown-definition behavior.
- Deterministic registry interpretation.
- Error reporting.

The registry MUST NOT depend on filesystem traversal order or unspecified map iteration.

11.2 Operation descriptors

An operation descriptor MUST state the information needed to construct and validate an operation, including applicable operand and result constraints, type rules, effects, attributes, regions, interfaces, and verification obligations.

Operation implementations MUST NOT be registered as trusted solely because their names are present. Registration MUST validate the descriptor's required metadata and compatibility contract.

A plugin or extension MUST NOT bypass mandatory verification by supplying an operation descriptor.

11.3 Interfaces

Reusable interfaces MAY express common obligations such as terminator behavior, constant-like behavior, memory access, region containment, connectivity, or effect declarations.

An interface defines a contract; it MUST NOT assume that every implementing operation has identical behavior.

11.4 Dialects

Dialects MUST have explicit identities, versions, ownership, and compatibility rules.

The initial planned responsibilities may include:

- Structural composition.
- Behavioral processes.
- Sequential and stateful operations.
- Abstract memory.
- Explicit target-aware physical intent.

These are architectural categories, not claims that the corresponding modules exist today.

Future analog/mixed-signal, high-level-synthesis, reconfigurable-fabric, or other hardware domains MAY be introduced as separate dialects when their semantics justify doing so.

Adding a dialect MUST NOT require rewriting the core graph merely to add a new operation family. It MUST NOT weaken the meaning of existing operations.

11.5 Extension security

Extension metadata and serialized operation descriptions MUST be treated as untrusted input.

An extension MUST NOT be permitted to execute arbitrary code during decoding or verification unless a separate, explicit execution and trust contract exists.

Unknown required extensions MUST cause an explicit unsupported-schema or unsupported-feature result.

Unknown optional extensions MAY be preserved opaquely only under the documented compatibility rules.

12. Identity, provenance, and source maps

12.1 Identity reuse

HDLIR MUST reuse canonical AST and semantic identities where their existing types and lifetime contracts are appropriate.

Before introducing a new identity type, its owner MUST establish that no canonical identity already represents the same concept.

The existing "src/hdl/ids.rs" contract intentionally reuses canonical identity types for AST nodes, declarations, modules, and designs where applicable, and defines additional HDL-specific identities only where needed. HDLIR MUST integrate with that contract rather than create a parallel allocator.

HDLIR-specific operation, value, block, region, schema, or other IDs MAY be introduced when their concepts have no suitable canonical identity.

Distinct semantic entity kinds SHOULD use distinct types to prevent accidental interchange.

12.2 Identity scope

Every identity MUST have a documented namespace and lifetime.

Numeric equality across unrelated designs or allocator instances MUST NOT be interpreted as identity.

Allocation MUST be explicit, checked, and non-wrapping. Identity exhaustion MUST produce a structured failure.

IDs MUST NOT be derived from pointer addresses, memory layout, or nondeterministic iteration order.

Persistent cross-build identity MUST be a separate, versioned contract; ordinary in-memory IDs MUST NOT implicitly promise persistence.

12.3 Provenance

HDLIR MUST preserve traceability to source and semantic origins wherever that information is available.

Provenance SHOULD identify:

- Original source location or canonical AST node.
- Relevant semantic declaration or fact.
- The lowering or transformation that produced an entity.
- Parent or contributing origins for synthesized or merged entities.
- Relevant schema and dialect versions.
- Verification or analysis evidence where applicable.

A synthesized operation MAY have multiple origins or no direct source token. The representation MUST handle this without inventing a false single-source location.

12.4 Source locations

HDLIR MUST reuse canonical frontend source spans and source-file identities. It MUST NOT introduce a competing source-span system.

If an HDLIR source map is needed, it MUST map HDLIR entities to canonical source locations and transformation provenance rather than redefine those locations.

Missing provenance MUST be represented explicitly.

13. Diagnostics and error semantics

13.1 Structured failures

HDLIR failures MUST be machine-readable and actionable.

The error model MUST distinguish at least:

- Invalid or inconsistent IR.
- Type or width mismatch.
- Invalid connectivity.
- Invalid process, state, or clock semantics.
- Unknown or incompatible schema.
- Unsupported required operation or dialect.
- Malformed artifact.
- Invalid parameter binding.
- Checked-arithmetic overflow or unrepresentable value.
- Operational-budget exhaustion.
- Cancelled or incomplete processing.
- Failed semantic-preservation obligation.
- Lossy or unsupported lowering.
- Target infeasibility, when supplied by the responsible realization boundary.

These categories MUST remain distinct from parser errors and source-level semantic diagnostics owned by the frontend.

13.2 Diagnostic contract

Each diagnostic SHOULD include a stable code, severity, explanation, relevant entity references, canonical source provenance where available, and useful remediation context.

Diagnostics MUST NOT depend on nondeterministic traversal order where stable ordering is promised.

Diagnostic production MAY itself be budgeted. If diagnostics are truncated, the result MUST indicate that fact.

13.3 No false success

A failed or incomplete required verification MUST NOT be reported as successful verification.

An unsupported operation MUST NOT be silently skipped when its semantics affect the design.

A target infeasibility MUST NOT automatically be classified as invalid source.

A lowering failure MUST NOT be treated as proof that the original design is semantically invalid.

13.4 Safe failure behavior

Ordinary invalid user input MUST be handled through explicit result types, not panics.

Malformed serialized artifacts MUST NOT cause undefined behavior, unbounded allocation, or unchecked arithmetic.

The implementation SHOULD avoid panics for malformed graphs and extension data. Any internal assertion used for a proven programmer invariant MUST be distinguished from validation of untrusted input.

14. Builders and construction transactions

14.1 Checked construction

Builders MUST provide documented APIs for creating designs, modules, ports, values, operations, regions, blocks, and connectivity.

A builder MUST either:

- Produce an object satisfying the invariants guaranteed by that builder's contract; or
- Return a structured error describing the failure.

If multi-step construction requires temporary invalid states, the API MUST distinguish unfinished construction from finalized or verified IR.

14.2 Transactions and rollback

If a builder supports transactions, its contract MUST define atomicity, commit, rollback, and behavior after failure.

A failed transaction MUST NOT leave the design in an undocumented partially mutated state.

If rollback is not supported, the API MUST state which partial changes may remain and how callers can recover safely.

14.3 Deterministic construction

Construction order MUST NOT become semantic unless the specification explicitly makes it semantic.

Builders SHOULD support stable canonical output for equivalent designs, with clearly documented exceptions for identity allocation or provenance that intentionally depends on construction history.

15. Verification

15.1 Verification result

Verification MUST produce an explicit result describing which checks were requested, which checks completed, which checks passed or failed, and whether the result is complete.

A successful result certifies only the invariants actually covered by that verification contract.

It does not prove that a design can be synthesized, that a target is available, that placement and routing will succeed, or that physical timing closure has been achieved.

15.2 Required verification areas

The implementation MUST define verification contracts for:

1. Structure: Ownership, identity uniqueness within scope, graph integrity, references, module hierarchy, regions, and blocks.
2. Types: Operand and result types, signedness, symbolic widths, aggregate compatibility, and operation constraints.
3. Connectivity: Drivers, sinks, ports, module bindings, dangling connections, and type compatibility.
4. Processes: Process kinds, legal assignments, execution rules, and control-flow requirements.
5. State and clocks: Clock and reset references, sequential-update semantics, and declared clock-domain contracts.
6. Memory: Access semantics, port rules, collision behavior, initialization, and state consistency.
7. Schema: Operation and type definitions, dialect versions, extension requirements, and interface obligations.
8. Resources and contracts: Internal consistency of symbolic requirements, constraints, and resource expressions.
9. Provenance and compatibility: Required origin information, schema compatibility, and supported artifact rules.

Each check MUST define whether it is mandatory, optional, unsupported, or incomplete for a given design.

15.3 Verification order

The verification orchestrator SHOULD order checks so that foundational structural errors are reported before checks that depend on invalid references.

The ordering and diagnostic aggregation policy MUST be deterministic where promised.

A later check MUST NOT reinterpret a failure in an earlier required check as success.

15.4 Verification limits

A verifier MAY use explicit work and memory budgets.

Budget exhaustion MUST yield an incomplete or failed result, not a successful verification status.

Verification MUST NOT allocate physical devices or perform placement or routing merely to validate target-independent HDLIR.

16. Analyses

Analyses compute reusable facts; they do not replace the canonical IR or automatically mutate it.

HDLIR SHOULD provide separately contracted analyses for:

- Dependencies among operations, values, modules, and memories.
- Reads, writes, state changes, and other declared effects.
- Symbolic width relationships and evaluable width facts.
- Clock-domain relationships and crossing information.
- Logical resource requirements and estimates.
- Connectivity properties.
- Module and operation reachability.
- Relevant transformation prerequisites.

Every analysis MUST define its inputs, outputs, invalidation rules, determinism guarantees, and budget-exhaustion behavior.

An analysis result MUST distinguish established facts from estimates, assumptions, unknown values, and incomplete work.

A logical resource estimate MUST NOT be described as an actual physical allocation. A clock-domain crossing report MUST NOT be described as proof of timing safety unless the required proof is established.

Analysis caches MUST include the inputs and schema/configuration versions that affect the result. Stale cached facts MUST NOT be reused as though they applied to a changed design.

17. Transformations and semantic preservation

17.1 General contract

Every transformation MUST define:

- Preconditions.
- Postconditions.
- Required analyses.
- Configuration and budget behavior.
- Error and incomplete-result behavior.
- Invalidated analyses.
- Preserved semantic properties.
- Provenance updates.
- Determinism guarantees.
- Tests establishing its contract.

A transformation MUST NOT be considered correct merely because its output passes structural verification.

17.2 Initial transformations

The architecture MAY introduce transformations such as:

- Canonicalization.
- Semantics-preserving simplification.
- Dead-code elimination.
- Parameter specialization.

Each transformation MUST be implemented and registered only after its contract and required dependencies are established.

17.3 Canonicalization

Canonicalization MUST normalize only those aspects of representation that the specification declares semantically irrelevant.

It MUST NOT reorder behavior-sensitive operations, alter process semantics, discard provenance required by the contract, or turn an unsupported operation into a supported one.

Equivalent designs SHOULD yield stable canonical representations where equivalence and canonicalization are defined.

17.4 Simplification

Simplification MUST use the registered operation semantics and applicable type rules.

Algebraic identities MUST NOT be applied where they change observable behavior, including where logic-state, overflow, event, side-effect, or timing semantics make the identity invalid.

17.5 Dead-code elimination

Dead-code elimination MUST preserve externally observable behavior, state transitions, required connectivity, externally visible interfaces, and any contractual or diagnostic obligations that the pass is required to retain.

An operation MUST NOT be removed merely because its result is unused if the operation has observable effects or contributes to a required hardware behavior.

17.6 Specialization

Specialization MUST validate parameter bindings and preserve the meaning of the generic design for the binding being specialized.

Unresolved expressions, invalid constraints, overflow, and resource-budget exhaustion MUST be reported explicitly.

17.7 Equivalence claims

A transformation MAY be documented as preserving a particular property based on its design and tests. It MUST NOT claim formal equivalence unless a suitable equivalence method actually establishes the stated property.

Where proof is incomplete, the result MUST state that limitation.

18. ZUIR and frontend lowering

18.1 Frontend boundary

HDLIR MUST consume the actual public contracts exposed by the canonical frontend and existing HDL API.

The implementation MUST inspect and map the real structures and rules of "HdlProgram", "HdlSignal", "HdlProcess", "HdlWidth", "HdlSourceLocation", "HdlValidationResult", and related types before implementing the corresponding adapters.

It MUST NOT assume that every parsed construct already has a complete semantic representation or lowering implementation.

HDLIR MUST NOT parse source text, tokenize source, or redo general name resolution as a substitute for the canonical frontend.

18.2 Semantic-to-HDLIR lowering

Lowering from validated HDL semantics MUST define a mapping for each supported construct.

For each mapping, document:

- Source semantic construct.
- Required validated facts.
- HDLIR representation.
- Type and width handling.
- Effects and temporal behavior.
- Provenance propagation.
- Unsupported or unresolved cases.
- Semantic-preservation obligations.
- Relevant tests.

A construct that cannot be represented faithfully MUST fail explicitly or use an approved extension whose semantics are defined.

18.3 ZUIR-to-HDLIR lowering

ZUIR is the canonical universal IR. HDLIR is the hardware-domain IR.

Conversion from ZUIR to HDLIR MUST be limited to operations whose hardware interpretation is defined and whose required semantic information is available.

General-purpose computation MUST NOT be assumed to be hardware-realizable merely because it appears in ZUIR.

If a ZUIR operation has no valid HDLIR mapping, the converter MUST return a structured unsupported-mapping or feasibility-related outcome. It MUST NOT invent hardware behavior.

18.4 HDLIR-to-ZUIR conversion

Conversion from HDLIR to ZUIR MUST preserve semantics when a faithful representation exists.

Hardware-specific behavior that cannot be expressed by the current ZUIR contract MUST use an explicitly approved extension or produce a structured losslessness failure.

The converter MUST NOT silently discard clocked state, logic-state distinctions, timing semantics, memory collision behavior, or other observable properties.

18.5 Integration order

The implementation sequence MUST be:

1. Freeze the HDLIR core representation and semantic contracts.
2. Audit the actual frontend and HDL public APIs.
3. Define the semantic-to-HDLIR mapping table.
4. Implement and test lowering from validated HDL semantics.
5. Define supported ZUIR operation mappings.
6. Implement the ZUIR adapters.
7. Add end-to-end tests that exercise real repository inputs.
8. Document unsupported constructs and incomplete downstream capabilities.

Successful conversion proves only that the defined conversion completed. It does not prove synthesis, placement, routing, or execution succeeded.

19. Serialization and compatibility

19.1 Artifact format

The persistent HDLIR format MUST be versioned independently of the Rust crate version, source-language version, grammar version, and HDL API schema version.

The artifact contract MUST define:

- Format identification.
- Format and schema versions.
- Required dialects and operation definitions.
- Numeric and symbolic value encoding.
- Canonical ordering where promised.
- Provenance representation.
- Integrity and malformed-input handling.
- Unknown extension behavior.
- Compatibility and migration policy.

The concrete encoding—text, binary, or another format—MUST be selected and specified before the serializer and decoder are frozen.

19.2 Deterministic encoding

When deterministic encoding is promised, equivalent canonical designs under the same schema, dialect versions, and relevant configuration MUST produce equivalent serialized artifacts.

Encoding MUST NOT depend on pointer values, memory addresses, unspecified map iteration, host endianness, or incidental allocation order.

Fields declared semantic MUST be distinguished from provenance-only or incidental fields in fingerprint and equality contracts.

19.3 Decoder security

The decoder MUST treat artifacts as untrusted input.

It MUST validate lengths, references, schema identities, nesting, arithmetic, and resource consumption before relying on decoded data.

It MUST NOT trust artifact-provided counts or sizes as safe allocation requests without checked validation and budget enforcement.

Malformed artifacts MUST return structured errors rather than cause undefined behavior or uncontrolled resource consumption.

19.4 Migrations

Every supported migration MUST identify its source and destination versions, preconditions, changes, and semantic-preservation guarantees.

Migrations MUST NOT silently reinterpret old artifacts under new operation semantics.

If faithful migration is impossible, the decoder or migration API MUST report incompatibility and preserve the original artifact when the caller's workflow permits it.

19.5 Compatibility

Compatibility rules MUST distinguish:

- Source-language compatibility.
- Semantic compatibility.
- In-memory API compatibility.
- Serialized-artifact compatibility.
- Dialect and operation-definition compatibility.
- Target compatibility.

A change to one versioning system MUST NOT implicitly change another.

20. Fingerprints and caching

Fingerprints MUST be deterministic for the inputs included in their documented contract.

The specification MUST distinguish at least:

- Structural fingerprint.
- Semantic fingerprint, if implementable under the established semantic-equivalence contract.
- Artifact or content fingerprint.
- Cache key for a particular analysis or transformation.

A fingerprint MUST include all schema, dialect, pass, configuration, and semantic inputs that affect the result.

It MUST NOT use a hash match as a substitute for semantic-equivalence proof.

Hash algorithm selection and evolution MUST be versioned when fingerprints are used in persistent artifacts or cache keys.

Cache invalidation MUST occur when any relevant dependency changes. An incomplete or failed analysis MUST NOT be cached as a successful complete result.

21. Interoperability

External-format adapters MUST be isolated from the core IR and MUST have explicit format and version contracts.

An importer MUST distinguish malformed external input from a valid external construct that the adapter does not support.

An exporter MUST report when the requested output format cannot represent a construct faithfully.

No adapter may claim full compliance with SystemVerilog, VHDL, an interchange standard, or a vendor format solely because a file or module with that name exists.

Initial support MUST be based on actual parsers, serializers, standards coverage, and verified mappings in the repository.

External-format provenance MUST be preserved wherever possible.

22. Physical-intent and target boundaries

HDLIR MAY represent target-aware intent when explicitly requested, but such intent MUST remain distinguishable from portable hardware semantics.

Target discovery and capability evidence belong to the actual target or realization boundary. Physical placement, routing, resource allocation, and technology mapping belong to their owning downstream components.

A target-aware plan MUST identify the target description and relevant evidence or version when that information affects its result.

HDLIR MUST NOT claim that a design is physically feasible merely because its logical representation is valid.

Resource analysis MUST distinguish:

- Declared requirements.
- Estimated demand.
- Available capability evidence.
- Allocated resources.
- Verified physical results.

These categories MUST NOT be treated as interchangeable.

A target's inability to satisfy a design requirement MUST yield an explicit result, not an implicit change in source or IR semantics.

23. Determinism and concurrency

The same canonical input, schema and dialect definitions, relevant configuration, and pass versions SHOULD yield the same canonical output whenever deterministic behavior is promised.

Deterministic behavior MUST NOT depend on:

- Filesystem traversal order.
- Hash-map iteration order.
- Hardware discovery order.
- Thread scheduling.
- Pointer values.
- Unspecified host state.

Parallel implementation MAY be used when it preserves the defined semantics and determinism contract.

Where nondeterminism is part of the hardware or source semantics, it MUST be represented explicitly rather than accidentally introduced by the compiler.

Concurrent mutation of an IR MUST follow a documented synchronization or ownership contract. An API MUST NOT imply thread safety that its actual data structures and invariants do not support.

24. Security and safe Rust

HDLIR production Rust MUST use safe Rust only.

The implementation MUST enforce the repository's safety policy with appropriate crate- or module-level attributes, including "#![forbid(unsafe_code)]" where applicable.

Production HDLIR MUST NOT use:

- "unsafe" blocks or unsafe functions.
- Unchecked indexing as a substitute for validation.
- Unchecked arithmetic for user-controlled values.
- Pointer-derived identity.
- Panics for ordinary malformed external artifacts.
- Unbounded allocation based on untrusted metadata.
- Silent truncation of required work.
- Implicit execution of untrusted extension code.

All externally supplied designs, schemas, extension metadata, and serialized artifacts MUST be treated as potentially malformed.

The implementation MUST use explicit errors, checked conversions, checked arithmetic, and bounded operational work where required.

Rust edition 2021 and Rust 1.97 or later are required. The code MUST be tested on the repository's supported toolchain rather than merely declaring a compatible "rust-version".

25. Required module contracts

The files below describe the initial implementation boundaries. A file MUST be added only when its contract, dependencies, and implementation need are established. This list does not claim that these files already exist.

25.1 Specification and composition

"HDLIR-SPEC.md"

Owns the normative HDLIR semantics, ownership boundaries, architecture, and conformance requirements.

Integrates with the existing grammar, semantic model, HDL API, identity contracts, and ZUIR specification.

Must be completed before dependent implementation contracts are frozen.

"compatibility.md"

Owns HDLIR API, schema, artifact, migration, and dialect compatibility policy.

Integrates with "version.rs", schema registration, serialization, and fingerprints.

Must define compatibility categories and migration failure behavior before the persistent format is frozen.

"mod.rs"

Owns the HDLIR composition root, implemented module declarations, deliberate public exports, and stable entry points.

Must not implement the IR itself or register speculative modules.

The current "src/hdl/mod.rs" test references "hdlir::Placeholder", while the current "src/hdl/hdlir/mod.rs" contains only a placeholder description. The implementation milestone MUST resolve this mismatch through a deliberate, tested module contract. It MUST NOT assume that the IR already exists.

25.2 Independent foundations

"config.rs"

Owns explicit operational budgets and deterministic processing options.

Integrates with builders, verification, analysis, passes, and serialization.

Must distinguish operational budgets from semantic requirements and physical hardware capacities.

"version.rs"

Owns HDLIR schema and artifact version representations and compatibility checks.

Integrates with "compatibility.md", schema registration, and serialization.

Must remain distinct from language, grammar, crate, and HDL API versions.

"errors.rs"

Owns typed HDLIR failures and error composition.

Integrates with builders, verification, analyses, transformations, lowering, and serialization.

Must preserve the distinction between invalid IR, unsupported features, incomplete work, and target infeasibility.

"diagnostics.rs"

Owns structured HDLIR diagnostic presentation and stable diagnostic codes where a distinct HDLIR diagnostic type is necessary.

Must reuse canonical source locations and integrate with existing diagnostic contracts rather than replace frontend diagnostics.

"ids.rs"

Owns only HDLIR identities not already represented by appropriate canonical identities.

Must reuse the identity infrastructure in "src/hdl/ids.rs" and canonical frontend semantic identity types.

Must define scope, allocation, exhaustion, determinism, and persistence boundaries.

"source_map.rs"

Owns mapping from HDLIR entities to canonical source locations and transformation provenance.

Must not define a competing source-span type.

"fingerprint.rs"

Owns structural and artifact fingerprint contracts and the inputs to deterministic cache keys.

Must define included and excluded fields, schema-version dependencies, and cache invalidation.

Must not claim semantic equivalence merely from matching fingerprints.

25.3 Schema, types, and values

"schema/mod.rs"

Composes the schema subsystem and exports its deliberate API.

"schema/registry.rs"

Owns registration, lookup, compatibility, duplicate-definition checks, and extension resolution.

"schema/operation.rs"

Owns operation descriptors and their verification obligations.

"schema/types.rs"

Owns schema-level type descriptors and versioned type-extension contracts.

"schema/interfaces.rs"

Owns reusable operation-interface contracts and their validation rules.

"types/mod.rs"

Composes the HDLIR type subsystem.

"types/bit_width.rs"

Owns symbolic and concrete HDLIR width descriptions, checked evaluation, and width relationships.

Must integrate with the existing "HdlWidth" contract without silently assuming arbitrary-precision concrete widths are supported.

"types/logic.rs"

Owns the supported HDL logic domains and their legal operations, including explicit two-state/four-state distinctions where applicable.

"types/signal.rs"

Owns HDLIR logical signal types and type-level compatibility rules.

Must distinguish logical signals from physical pins.

"types/aggregate.rs"

Owns aggregate type descriptions, dimensions, and compatibility rules.

Must not introduce arbitrary universal rank or field-count ceilings.

"types/clock.rs"

Owns logical clock/reset type or contract descriptions.

Must not implement clock-tree synthesis or physical allocation.

"types/memory.rs"

Owns abstract memory shape, element, port, and access type contracts.

Must not assume a particular physical memory technology.

"values/mod.rs"

Composes constant and symbolic value representations.

"values/constants.rs"

Owns canonical constant representations, exact-width values, and malformed-constant errors.

"values/expressions.rs"

Owns HDLIR-specific symbolic expressions only where the canonical frontend expression representation is not the appropriate IR representation.

Must define evaluation and overflow semantics.

"values/parameters.rs"

Owns parameter declarations, bindings, constraints, and specialization inputs.

Must integrate with canonical semantic facts and the specialization transformation.

25.4 Core graph

"ir/mod.rs"

Composes the IR graph and exposes its deliberate API.

"ir/design.rs"

Owns top-level design structure, module registry, design metadata, and schema references.

"ir/module.rs"

Owns logical modules, signatures, parameters, declarations, and bodies or implementation references.

"ir/ports.rs"

Owns typed logical ports and interface bindings.

"ir/operations.rs"

Owns registered operation instances, operands, results, attributes, effects, and region relationships.

Must not hard-code the universal operation catalogue.

"ir/values.rs"

Owns SSA values and other computed-value references as defined by the core model.

Must preserve the distinction between computed values, persistent signals, state elements, and memories.

"ir/regions.rs"

Owns nested region containment, scope, and region-level invariants.

"ir/blocks.rs"

Owns ordered operation sequences, block arguments, and block boundaries for applicable dialects.

"ir/control_flow.rs"

Owns supported control-flow terminators and branch contracts.

Must distinguish combinational control flow from clocked behavior.

"ir/connectivity.rs"

Owns logical connection relationships and their endpoint semantics.

"ir/processes.rs"

Owns behavioral processes, trigger contracts, and assignment categories.

"ir/clocks.rs"

Owns logical clock/reset associations and clock-domain relationships in the IR.

"ir/memories.rs"

Owns abstract memory instances, ports, and access behavior.

25.5 Construction, verification, and analysis

"builder/mod.rs"

Composes checked construction APIs and defines finalized versus intermediate construction states.

"builder/design.rs"

Owns checked design construction and module registration.

"builder/module.rs"

Owns checked module signature, parameter, declaration, and body construction.

"builder/operation.rs"

Owns checked operation construction using registered descriptors.

"verify/mod.rs"

Owns verification orchestration and complete/incomplete result reporting.

"verify/structure.rs"

Verifies graph integrity, identity scope, containment, and references.

"verify/types.rs"

Verifies types, widths, signedness, and operation type constraints.

"verify/connectivity.rs"

Verifies endpoint validity, driver/sink rules, directionality, and compatibility.

"verify/processes.rs"

Verifies process legality and behavioral semantics.

"verify/clocks.rs"

Verifies clock/reset references and applicable clock-domain contracts.

"verify/resources.rs"

Verifies resource-expression consistency and symbolic requirements without allocating physical resources.

"analysis/mod.rs"

Composes reusable analyses and defines their completeness and invalidation contracts.

"analysis/dependencies.rs"

Computes dependency relationships without replacing the canonical graph.

"analysis/effects.rs"

Computes or summarizes reads, writes, state changes, and registered effects.

"analysis/widths.rs"

Evaluates widths when permitted and preserves unresolved symbolic relationships.

"analysis/clocks.rs"

Analyzes clock-domain relationships without claiming physical timing closure.

"analysis/resources.rs"

Estimates logical resource requirements and clearly labels estimates, assumptions, and uncertainty.

25.6 Passes and transformations

"passes/mod.rs"

Defines pass registration and invocation contracts.

"passes/manager.rs"

Owns pass ordering, execution, cancellation, budget handling, and invalidation propagation.

"passes/contracts.rs"

Defines pass preconditions, postconditions, required analyses, and preservation obligations.

"passes/preservation.rs"

Defines which semantics, effects, state transitions, constraints, and provenance must survive transformations.

"transform/mod.rs"

Composes implemented transformations.

"transform/canonicalize.rs"

Owns semantics-preserving canonicalization.

"transform/simplify.rs"

Owns simplifications justified by registered semantics.

"transform/dead_code.rs"

Owns removal of operations proven irrelevant to required observable behavior.

"transform/specialize.rs"

Owns checked parameter substitution and specialization.

No transformation file may claim correctness solely because the resulting IR passes structural verification.

25.7 Lowering

"lowering/mod.rs"

Composes lowering APIs and shared outcomes.

"lowering/from_semantic.rs"

Lowers actual validated frontend HDL semantic contracts into HDLIR.

Must preserve types, state semantics, effects, and source provenance.

"lowering/from_zuir.rs"

Maps supported ZUIR constructs into HDLIR using explicit hardware interpretation contracts.

"lowering/to_zuir.rs"

Maps HDLIR constructs into ZUIR where meaning can be preserved.

"lowering/contracts.rs"

Owns mapping coverage, preconditions, postconditions, unsupported mappings, and semantic-preservation obligations.

25.8 Serialization and extensions

"serialization/mod.rs"

Composes artifact read/write APIs.

"serialization/format.rs"

Defines the versioned artifact envelope and schema metadata.

"serialization/encode.rs"

Encodes IR under the documented canonicalization and determinism contract.

"serialization/decode.rs"

Decodes untrusted artifacts with validation, checked arithmetic, and explicit resource budgets.

"serialization/migration.rs"

Owns explicit version-to-version artifact migrations.

"dialects/mod.rs"

Registers implemented dialects and exposes extension metadata.

"dialects/structural/mod.rs"

Owns structural composition operations and contracts.

"dialects/behavioral/mod.rs"

Owns behavioral operation definitions and process semantics.

"dialects/sequential/mod.rs"

Owns sequential operations and state-update contracts.

"dialects/memory/mod.rs"

Owns abstract memory operations and access semantics.

"dialects/physical_intent/mod.rs"

Owns explicitly target-aware intent that is separate from portable semantic meaning.

25.9 Interchange and tests

"interchange/mod.rs"

Composes external-format adapters and shared conversion outcomes.

"interchange/import.rs"

Imports supported formats while preserving provenance and reporting unsupported constructs.

"interchange/export.rs"

Exports only constructs representable under the target format's documented semantics.

"tests/mod.rs"

Composes HDLIR cross-module tests without creating a competing test framework.

"tests/fixtures.rs"

Defines canonical designs, boundary cases, and malformed-artifact fixtures.

"tests/conformance.rs"

Tests implemented schema, type, graph, dialect, and verification contracts.

"tests/determinism.rs"

Tests canonical representations, serialization, and fingerprints.

"tests/scalability.rs"

Tests progressively larger graphs, symbolic dimensions, and explicit resource exhaustion.

"tests/malformed_input.rs"

Tests malformed graphs, artifacts, identities, schemas, and extensions.

"tests/compatibility.rs"

Tests supported old artifacts, migrations, and incompatibility behavior.

Focused unit tests MUST also live alongside the implementations they test.

26. File freeze and integration policy

Every HDLIR source file MUST establish its integration contract before implementation is considered complete.

The contract MUST identify:

1. Purpose and implementation status.
2. Normative specification authority.
3. What the file owns and does not own.
4. Public types and API behavior.
5. Dependencies on existing repository modules.
6. Planned consumers and how they use the API.
7. Invariants and failure behavior.
8. Scalability and operational-budget behavior.
9. Determinism and compatibility guarantees.
10. Positive, negative, boundary, malformed-input, and stress tests as applicable.
11. Security and safe-Rust requirements.
12. Objective freeze criteria.

Before a file is frozen, its dependencies MUST exist or be represented by an already agreed interface contract. Public APIs MUST NOT depend on unspecified constructors, assumed fields, or unverified future modules.

A frozen file MUST NOT require routine redesign merely because a dependent file is implemented. Integration defects MUST still be corrected through a deliberate, reviewed change.

26.1 No speculative public APIs

A proposed file in this specification is not automatically part of the Rust module tree.

The composition root MUST register only modules that exist and whose public boundaries have been deliberately approved.

A future module MUST NOT be referenced by downstream code before its interface contract is agreed.

26.2 One authoritative implementation

For every IR responsibility, the repository MUST identify one authoritative owner.

A new HDLIR module MUST NOT duplicate an existing frontend AST, semantic model, source-span system, canonical identity allocator, universal IR, or quantum IR.

If an existing implementation is incomplete or unsuitable, its owner and integration path MUST be identified before a replacement is introduced.

27. Required implementation sequence

The implementation MUST follow dependency order rather than simply creating files in alphabetical or filesystem order.

Stage 0 — Repository and authority audit

Inspect the actual branch and relevant files:

- "src/hdl/hdlir/mod.rs"
- "src/hdl/mod.rs"
- "src/hdl/api.rs"
- "src/hdl/ids.rs"
- The canonical HDL semantic implementation and its real type definitions.
- The canonical frontend AST and semantic identity definitions.
- "grammar/DESIGN.md"
- "grammar/spec/hdl.md"
- "grammar/hdl/hdl.g4" and its relevant delegates.
- "grammar/Zamani.g4"
- "grammar/antlr/ZamaniLexer.g4"
- "grammar/lexer/tokens.g4"
- "grammar/specification/semantic-model.md"
- "grammar/validation/semantic-boundaries.md"
- "src/zuir/ZUIR-SPEC.md"
- The actual Cargo configuration and existing tests.

Record the current build/test baseline. Resolve the "Placeholder" reference discrepancy and identify any other concrete integration problems.

Exit condition: The existing authority, dependencies, API contracts, and known baseline failures are documented. No assumed API remains an undocumented dependency.

Stage 1 — Specification and compatibility freeze

Finalize this specification, the compatibility policy, and the composition-root contract.

Decide and document the relationship between SSA values, persistent signals, sequential state, and memory before implementing the core graph.

Define the initial artifact and schema-versioning strategy before serializer design.

Exit condition: The core model and cross-file boundaries are stable enough for independent foundation work.

Stage 2 — Independent foundations

Implement and test:

1. Configuration and operational budgets.
2. Version and compatibility primitives.
3. Structured errors.
4. Diagnostics integration.
5. Canonical identity reuse and HDLIR-specific identities.
6. Source maps and provenance.
7. Fingerprint contracts.

Exit condition: Dependent modules can use stable foundational APIs without inventing their own versions.

Stage 3 — Schema, types, and values

Implement schema registration and descriptors, then hardware types and symbolic values.

Exit condition: Operation registration, type equality, symbolic-width handling, checked evaluation, compatibility, and extension behavior have tests.

Stage 4 — Core IR

Implement design, module, ports, operations, values, regions, blocks, control flow, connectivity, processes, clocks, and memories according to the agreed model.

Exit condition: A design can be constructed using the documented representation without confusing combinational values, signals, state, and memory.

Stage 5 — Builders, verification, and analyses

Implement checked construction, required verifiers, explicit incomplete results, and reusable analyses.

Exit condition: Valid and invalid designs produce deterministic, actionable results, and budget exhaustion cannot be mistaken for success.

Stage 6 — Transformations

Implement the pass contract and manager, then individual transformations with explicit preservation obligations.

Exit condition: Each pass has focused tests, regression coverage, and documented preservation guarantees.

Stage 7 — Frontend and ZUIR integration

Implement semantic lowering first, then ZUIR conversion in each supported direction.

Exit condition: Actual repository inputs can reach HDLIR through verified adapters, and unsupported mappings fail explicitly.

Stage 8 — Serialization and dialects

Implement versioned artifacts, safe decoding, migration, canonicalization, and required dialects.

Exit condition: Artifacts round-trip under the documented contract, malformed inputs are rejected safely, and compatibility failures are explicit.

Stage 9 — Interchange and end-to-end tests

Implement only justified external-format adapters. Add tests across frontend, HDLIR, ZUIR, and supported downstream stages.

Exit condition: Supported workflows have reproducible evidence and documented unsupported cases.

Stage 10 — Production qualification

Run all required repository checks, HDLIR-specific conformance tests, scalability tests, malformed-input tests, and compatibility tests.

Exit condition: Readiness is supported by actual test and integration evidence rather than by file count or specification completeness.

28. Required conformance and regression coverage

28.1 Core cases

The test suite MUST cover:

- Minimal valid design.
- Multiple modules and instances.
- Valid and invalid port connections.
- Concrete and symbolic widths.
- Signed and unsigned operations.
- Supported two-state and four-state logic.
- Aggregate types and parameterized dimensions.
- Combinational processes.
- Sequential state transitions.
- Clock and reset behavior.
- Multiple clock domains.
- Memory read/write and collision behavior.
- Registered operation and dialect extensions.
- Unknown required and optional extensions.
- Provenance propagation.
- Deterministic canonicalization and fingerprints.
- Serialization and migration.
- Unsupported lowering.
- Incomplete verification and explicit budget exhaustion.

28.2 Negative cases

Tests MUST verify explicit failure for:

- Duplicate or invalid identities within their scope.
- Dangling references.
- Invalid graph containment.
- Invalid operation descriptors.
- Type or width mismatch.
- Arithmetic overflow.
- Invalid parameter bindings.
- Illegal driver/sink relationships.
- Invalid process semantics.
- Missing clock or reset references.
- Unsupported required dialects.
- Malformed serialized artifacts.
- Unsupported lossy exports.
- Failed required transformations.
- Cancellation or budget exhaustion during mandatory checks.

28.3 Scalability cases

Scalability tests SHOULD exercise progressively larger designs and symbolic quantities without defining a universal maximum design size.

The suite MUST distinguish a failure caused by a configured budget from a semantic rejection or representation limitation.

It SHOULD test large operation graphs, wide logical values, deep module hierarchies, large port sets, many clock domains, parameterized memories, and large artifact decoding workloads.

Tests MUST NOT require every test machine to construct the largest possible design. They SHOULD use configurable workload sizes and report the tested sizes and resource conditions.

28.4 Repository fixtures

Where compatible with the actual grammar and frontend, integration tests SHOULD use representative repository fixtures such as:

- "minimal.zm"
- "classical.zm"
- "quantum.zm"
- "hybrid.zm"
- "hdl.zm"

HDL-specific fixtures MUST cover parameterized hardware, combinational and sequential behavior, memory, hierarchy, multiple clocks, invalid connectivity, and unsupported mappings.

A fixture MUST NOT be claimed to test HDLIR unless it actually passes through the HDLIR implementation under test.

28.5 Property-based and fuzz testing

Where appropriate, HDLIR SHOULD use property-based or fuzz testing for builders, decoders, verifiers, canonicalizers, and supported conversions.

Properties SHOULD include:

- No panic on malformed external input.
- Checked arithmetic never silently wraps.
- Serialization round-trips preserve the documented artifact semantics.
- Canonicalization is deterministic under its defined equivalence conditions.
- Verification never reports complete success after a required check fails or remains incomplete.
- Unsupported operations are never silently dropped.

29. Production-readiness gates

Gate| Required evidence
G0 — Authority| Repository audit, ownership rules, and core semantic model agreed.
G1 — Foundations| Configuration, errors, versions, identities, provenance, and fingerprints tested.
G2 — Representation| Schema, types, values, design graph, state, connectivity, and memory implemented.
G3 — Verification| Required structural, type, connectivity, process, clock, and schema checks pass conformance tests.
G4 — Frontend integration| Actual validated HDL inputs lower correctly with preserved semantics and provenance.
G5 — ZUIR integration| Supported mappings are tested; unsupported mappings fail explicitly.
G6 — Transformations| Each implemented pass has contract and preservation evidence.
G7 — Artifact stability| Serialization, decoding, versioning, and migrations pass compatibility tests.
G8 — End-to-end| Representative designs traverse supported pipeline stages without silent semantic loss.
G9 — Scalability and security| Large workloads, symbolic values, malformed inputs, and resource exhaustion are handled explicitly.
G10 — Release qualification| Supported-toolchain checks, tests, linting, documentation, and downstream compatibility pass.

A gate MUST NOT be marked complete merely because its files exist. It requires the specified evidence.

30. Required repository checks

Run the following commands against the actual repository checkout using the supported Rust toolchain:

cargo fmt --all -- --check
cargo check --all-targets
cargo test --all-targets
cargo clippy --all-targets -- -D warnings

Also run HDLIR-specific unit, integration, conformance, determinism, scalability, malformed-input, and compatibility tests as they are implemented.

If a check fails because of an unrelated pre-existing repository defect, record the exact failure and establish the baseline. Do not silently attribute unrelated failures to HDLIR, and do not claim the repository passes.

No build, test, lint, synthesis, or physical timing result may be claimed unless the corresponding process was actually run and its result recorded.

31. Definition of done for this specification

This specification is ready to serve as the HDLIR implementation authority when:

1. Its ownership boundaries agree with the actual repository authorities.
2. The distinction between ZUIR and HDLIR is explicit.
3. SSA values, persistent signals, sequential state, and memory have distinct semantics.
4. Symbolic widths and parameters are first-class representation concerns.
5. Concrete representational limits are checked and documented rather than confused with language-level hardware limits.
6. Requirements, capabilities, constraints, preferences, hints, policies, and contracts remain distinct.
7. Unknown operations and dialects have explicit compatibility behavior.
8. Verification, analysis, and transformation guarantees are separate.
9. Incomplete work cannot be reported as successful verification.
10. Frontend lowering and ZUIR conversion have explicit semantic-preservation obligations.
11. Identity, provenance, serialization, and compatibility rules are defined.
12. Safe Rust and malformed-input requirements are explicit.
13. Each planned implementation file has an agreed contract and dependency order.
14. Tests and production-readiness gates are measurable.
15. No claim of implementation or production readiness exceeds the evidence available.

32. Final invariant

The governing invariant of Zamani HDLIR is:

One declared hardware meaning; many valid representations and realizations; no silent change of semantics.

The source program establishes its meaning and requirements.

The canonical frontend establishes validated source facts.

ZUIR represents universal computational meaning.

HDLIR represents the hardware-domain semantics and structures that are validly expressible in its contract.

Analysis determines facts and estimates.

Transformations change representations only under explicit preservation obligations.

Downstream realization selects and implements a target-specific solution.

A target may be unable to realize a particular design. A compiler may run out of resources. A transformation may fail. A schema may be unsupported. These outcomes MUST be explicit and MUST NOT be misreported as successful semantic preservation.

The physical implementation may change. The compiler strategy may change. The representation may evolve under versioned compatibility rules. The available resources may change. New hardware may become available.

The semantic meaning of the program MUST NOT change merely because the machine changed.

That is the architectural foundation of Zamani HDLIR and its contribution to:

Zamani — From Atom to Everywhere.

Program Once → Compile Once → Run Everywhere → Run Anywhere → Run Forever.
AST Coverage Specification

File: "grammar/validation/ast-coverage.md"
Status: Normative validation specification
Version: 1.0.0 (proposed)
Language: Zamani
Implementation language: Rust 1.97 / 1.97.1
Safety: Safe Rust only; "unsafe" is prohibited
Scope: Grammar-to-AST structural and semantic coverage
Architecture: Specification → Lexer → Parser → AST → Semantic Analysis → Canonical IR → Compiler → Runtime
Portability objective: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)

---

1. Purpose

This document defines the complete production requirements for validating AST coverage throughout the Zamani grammar and compiler.

AST coverage validation ensures that every supported language construct has a defined, reachable, unambiguous, and testable representation in the frontend AST.

The validator must detect:

1. Grammar rules without AST representations.
2. AST nodes without authoritative grammar origins.
3. Parser alternatives that cannot produce their intended AST nodes.
4. AST variants that are never constructed.
5. AST constructs that cannot represent all information required by semantic analysis.
6. Domain constructs incorrectly represented using unrelated AST variants.
7. Quantum operations incorrectly restricted to fixed gate enumerations.
8. Hardware-specific implementation details leaking into portable AST structures.
9. Missing source locations.
10. Lost source information during AST construction.
11. Missing semantic and canonical IR mappings.
12. Inconsistent AST contracts across the repository.
13. Unsupported features incorrectly marked as implemented.
14. Feature manifests that disagree with implementation.
15. Coverage gaps in positive, negative, boundary, scalability, determinism, and compatibility tests.

The validator must establish whether the frontend can represent the complete language contract without requiring artificial hardware limits.

AST coverage is not merely a percentage of parser rules exercised by tests.

It is a traceability and correctness property across the complete language pipeline.

---

2. Authority and ownership

2.1 Normative ownership

This document owns:

- AST coverage terminology.
- AST coverage classification.
- Grammar-to-AST traceability requirements.
- AST-to-semantic traceability requirements.
- AST-to-IR traceability requirements.
- AST coverage validation rules.
- Coverage evidence requirements.
- Coverage failure classification.
- Coverage acceptance criteria.
- Integration contracts for the AST validator.

It does not own:

- The complete language grammar.
- The authoritative AST implementation.
- Lexer token definitions.
- Semantic analysis algorithms.
- Quantum IR implementation.
- Hardware backend implementation.
- Runtime execution.
- QEC implementation.
- ZQN implementation.
- HAL implementation.
- Scheduling or routing algorithms.

Those responsibilities remain with their existing authoritative components.

2.2 Authority hierarchy

The following hierarchy is mandatory:

Component| Authority
"DESIGN.md"| Normative architecture
"specification/"| Normative language specification
"spec/"| Formal language contracts
"Zamani.g4"| Canonical ANTLR composition root
"src/lexer.rs"| Actual lexical implementation
"src/parser.rs"| Actual parser implementation
"src/frontend/ast/"| Canonical frontend AST, if present
"src/ast/"| Existing AST implementation, if this remains the repository's canonical location
Semantic analyzer| Semantic interpretation
"quantum::ir"| Canonical quantum IR boundary
Compiler and runtime| Lowering and execution
"grammar/grammar.md"| Generated implementation-conformance reference
"Zamani-Grammar.md"| Historical and extended design reference

The actual canonical AST directory must be established from repository evidence.

This document does not authorize creating a second AST implementation.

2.3 Conflict resolution

If two files define incompatible AST contracts:

1. Identify the owning authority.
2. Identify the affected feature.
3. Record the discrepancy.
4. Reject silent substitution.
5. Correct the non-authoritative document or implementation.
6. Update the feature manifest.
7. Run the complete affected validation suite.
8. Record the resulting conformance evidence.

The validator must not resolve conflicts by arbitrarily selecting whichever representation happens to compile.

---

3. Core definitions

3.1 AST

An Abstract Syntax Tree is the structured representation of a parsed Zamani program.

It preserves the information needed for subsequent compiler phases while removing irrelevant syntactic details.

An AST must preserve:

- Construct identity.
- Child relationships.
- Operand ordering.
- Relevant literal values.
- Names and qualified paths.
- Generic arguments.
- Attributes and modifiers.
- Source provenance.
- Relevant syntactic distinctions.
- Explicitly required domain metadata.

An AST must not silently discard information required for semantic interpretation.

3.2 AST coverage

AST coverage measures whether every authoritative, supported language construct has a complete and verified path from source syntax to its canonical frontend representation.

A construct is covered only when its complete required traceability chain exists.

Specification
      |
      v
Grammar rule
      |
      v
Lexer/parser implementation
      |
      v
AST representation
      |
      v
Semantic interpretation
      |
      v
Canonical IR mapping
      |
      v
Conformance evidence

A construct that parses successfully but produces an incorrect AST is not covered.

A construct that produces an AST but loses required source information is not covered.

A construct with an AST but no defined semantic interpretation must not be reported as fully implemented.

3.3 Coverage is not implementation status

The validator must distinguish:

- Syntax specified.
- Syntax accepted.
- AST constructed.
- AST structurally valid.
- AST semantically supported.
- AST lowered into canonical IR.
- End-to-end execution supported.

These states must never be collapsed into a single boolean called "implemented".

3.4 Coverage is not a hardware limit

AST coverage must not introduce artificial limits on:

- Qubits.
- Quantum registers.
- CPUs.
- GPUs.
- FPGAs.
- QPUs.
- Memory.
- Threads.
- Distributed nodes.
- Tensor rank.
- Register width.
- Timelines.
- Network size.
- Hardware device count.

The validator may have configurable operational budgets for its own execution, but these budgets must never become language or AST expressiveness limits.

---

4. Goals and non-goals

4.1 Goals

The system must:

1. Establish complete grammar-to-AST traceability.
2. Validate AST schema completeness.
3. Detect missing AST constructors.
4. Detect unreachable AST variants.
5. Detect unrepresented grammar alternatives.
6. Validate domain-specific AST contracts.
7. Verify source-span preservation.
8. Validate semantic mapping declarations.
9. Validate canonical IR mapping declarations.
10. Verify feature lifecycle consistency.
11. Generate machine-readable coverage evidence.
12. Support incremental validation.
13. Support deterministic validation.
14. Scale according to available resources.
15. Integrate with existing Rust frontend architecture.
16. Preserve backward compatibility where promised.
17. Support future computing domains without changing universal AST foundations.

4.2 Non-goals

This validator does not:

- Prove that a program is mathematically correct.
- Prove that every semantic transformation is correct.
- Prove that hardware can execute a program.
- Guarantee that unlimited physical resources exist.
- Execute quantum circuits.
- Perform quantum routing.
- Perform quantum error correction.
- Schedule hardware.
- Replace the parser.
- Replace semantic analysis.
- Generate a duplicate AST.
- Enforce a fixed number of grammar rules.
- Enforce a fixed number of AST variants.
- Enforce a fixed number of source files.

---

5. Required integration contracts

The following existing files must be integrated without unnecessary renaming.

File| Required responsibility
"grammar/DESIGN.md"| Defines architecture and authority
"grammar/README.md"| Documents validation navigation
"grammar/Zamani.g4"| Supplies authoritative grammar entry points
"grammar/grammar.md"| Reports actual implementation conformance
"grammar/Zamani-Grammar.md"| Supplies historical/proposed feature references
"grammar/spec/syntax.md"| Defines syntax-to-AST contracts
"grammar/spec/type-system.md"| Defines type AST requirements
"grammar/spec/semantics.md"| Defines semantic mapping requirements
"grammar/spec/source-spans.md"| Defines source provenance
"grammar/spec/compatibility.md"| Defines AST compatibility requirements
"grammar/specification/features/"| Supplies feature contracts and lifecycle metadata
"grammar/validation/README.md"| Documents validation commands and architecture
"grammar/validation/grammar-validator.md"| Defines grammar validation
"grammar/validation/semantic-coverage.md"| Defines semantic coverage
"grammar/validation/ir-coverage.md"| Defines IR coverage
"grammar/validation/source-spans.md"| Defines source-span validation
"grammar/validation/scalability.md"| Defines scalability validation
"grammar/validation/determinism.md"| Defines deterministic validation
"grammar/validation/compatibility.md"| Defines compatibility validation
"grammar/tests/"| Supplies conformance tests
"src/lexer.rs"| Supplies actual token behavior
"src/parser.rs"| Supplies actual parser behavior
"src/frontend/ast/" or canonical existing AST directory| Supplies AST schema
Semantic analyzer| Supplies actual semantic support
"quantum::ir"| Supplies canonical quantum lowering boundary

The integration contract is declarative: this document specifies the required interfaces, but does not assume that every named implementation already exists.

---

6. AST coverage model

6.1 Required coverage dimensions

Every feature must be evaluated across the following dimensions:

Dimension| Question
Specification| Is the construct formally specified?
Lexical| Can its required tokens be produced?
Grammar| Is its syntax defined?
Reachability| Can the construct be reached from the root grammar?
Parsing| Can the parser recognize it?
Construction| Can the parser construct the correct AST?
Structure| Does the AST preserve required relationships?
Provenance| Does the AST preserve source locations?
Attributes| Are required modifiers and annotations preserved?
Semantics| Is semantic interpretation defined and implemented?
IR| Is the canonical IR mapping defined and implemented?
Diagnostics| Are invalid constructs reported correctly?
Tests| Is there conformance evidence?
Compatibility| Are supported older forms preserved or migrated?
Determinism| Is AST construction deterministic where required?
Scalability| Does validation avoid arbitrary language limits?
Portability| Is target-specific realization separated from source meaning?

6.2 Coverage states

Every feature and construct must have exactly one effective conformance state.

State| Meaning
"UNSPECIFIED"| No authoritative specification exists
"SPECIFIED"| Normative specification exists
"GRAMMAR_DEFINED"| Authoritative syntax exists
"PARSER_SUPPORTED"| Parser accepts the construct
"AST_SUPPORTED"| Correct AST construction exists
"SEMANTIC_SUPPORTED"| Semantic interpretation exists
"IR_SUPPORTED"| Canonical IR mapping exists
"PARTIALLY_IMPLEMENTED"| Some required stages are incomplete
"IMPLEMENTED"| All required stages are implemented
"TESTED"| Required conformance evidence passes
"STABLE"| Stable lifecycle requirements are satisfied
"DEPRECATED"| Supported only under an explicit deprecation policy
"REJECTED"| Construct is explicitly unsupported or invalid

These states describe conformance evidence, not a mandatory sequential enumeration.

For example, a proposed construct may be "SPECIFIED" but not "PARSER_SUPPORTED".

6.3 No false completion

The following must never be accepted:

Grammar exists
AST node exists
Therefore feature is complete

The required determination is:

Authoritative contract exists
AND grammar mapping exists
AND parser mapping exists
AND AST construction exists
AND AST structural validation passes
AND semantic contract exists
AND IR contract exists where required
AND required tests pass
AND compatibility policy exists
AND portability requirements pass

The validator must support features whose required downstream IR is not yet available, but must report them as incomplete rather than fully implemented.

---

7. Canonical AST representation requirements

7.1 One canonical frontend AST

Zamani must maintain one canonical frontend AST boundary.

The implementation must not create separate competing frontend ASTs for:

- Classical computing.
- Quantum computing.
- HDL.
- AI.
- Distributed computing.
- Hardware.
- Networking.
- Sankofa.
- Nano computing.

Domain-specific constructs may have domain-specific AST node types or structured payloads, but they must belong to the canonical frontend AST architecture.

7.2 Generic universal structure

The canonical AST should distinguish:

1. Universal language constructs.
2. Domain-specific language constructs.
3. Source provenance.
4. Syntactic metadata.
5. Semantic annotations, where explicitly supported.

Domain-specific AST nodes must not force unrelated domains to depend on implementation-specific hardware structures.

7.3 AST node identity

Every AST construct must have a stable logical identity.

The identity must not depend solely on:

- Its display name.
- Its source line.
- Its memory address.
- Its parser allocation order.
- Its current position in a vector.

A suitable identity may be represented by a stable feature identifier and a stable node-kind identifier.

Concrete Rust representations must follow the existing AST architecture.

The validator must not require changing established public AST identifiers without a compatibility migration.

7.4 AST ownership

Every AST node must have one clearly defined owner.

The owner is responsible for:

- Its representation.
- Construction rules.
- Structural invariants.
- Source-span behavior.
- Serialization policy, if applicable.
- Compatibility policy.
- Semantic mapping.
- Tests.

Shared types must not be independently redefined by multiple domain modules.

7.5 No semantic duplication

The AST may represent syntax and semantic intent.

It must not independently implement:

- Quantum optimization.
- QEC.
- ZQN noise interpretation.
- Hardware routing.
- Hardware scheduling.
- HAL device discovery.
- Physical calibration.
- Runtime resource allocation.

These are downstream responsibilities.

---

8. Grammar-to-AST traceability

8.1 Mandatory mapping

Every implemented grammar construct must declare its AST mapping.

The mapping must identify:

Feature ID
Grammar file
Grammar rule
Grammar alternative
AST module
AST node kind
AST fields
Construction function or parser production
Source-span policy
Semantic mapping
IR mapping
Tests
Status

Mappings must be machine-readable where possible.

8.2 Alternative-level coverage

Rule-level coverage alone is insufficient.

Consider:

expression
    : literalExpression
    | functionCall
    | quantumExpression
    ;

Testing only "expression" does not prove that all alternatives are represented.

The validator must distinguish:

- Rule coverage.
- Alternative coverage.
- AST constructor coverage.
- AST field coverage.
- Semantic mapping coverage.

8.3 Unreachable grammar rules

A grammar rule is unreachable when no valid root production can reach it.

Unreachable rules must be classified as:

- Intentionally reusable fragment.
- Deprecated rule.
- Experimental rule.
- Unreferenced implementation defect.

A reusable fragment is not an error merely because it is not directly reachable from the root.

The validator must use explicit rule metadata to distinguish reusable fragments from accidental dead grammar.

8.4 Grammar alternative completeness

For each supported alternative, the AST mapping must account for every semantically meaningful component.

Example:

functionCall
    : qualifiedName
      genericArguments?
      arguments
  ;

Its AST contract must define:

- Qualified name.
- Generic arguments.
- Argument ordering.
- Source span.
- Relevant attributes.
- Whether trailing delimiters are syntactic-only or semantically meaningful.

An alternative must not be marked covered if the AST silently drops a required component.

8.5 Duplicate syntax

Different syntax forms may intentionally map to one AST node.

For example, two compatible syntactic forms may have identical semantics.

Such equivalence must be explicitly documented.

The validator must reject accidental AST duplication where two nodes represent the same universal construct without a semantic distinction.

Conversely, it must reject collapsing two constructs into one AST node when their semantics differ.

---

9. AST structural completeness

9.1 Required structural invariants

Every AST node must satisfy its declared invariants.

These may include:

- Required child presence.
- Valid child ordering.
- Valid parent-child relationships.
- Required identifier presence.
- Valid generic argument structure.
- Valid type-expression structure.
- Valid operator representation.
- Required domain metadata.
- Required source provenance.

The validator must distinguish structural invariants from semantic restrictions.

For example, whether two quantum operands are physically compatible is not necessarily an AST structural property.

9.2 Required fields

Every required AST field must be populated during construction.

Optional fields must have an explicitly defined absence meaning.

The following is prohibited:

Missing required AST information
    ↓
Insert an arbitrary default
    ↓
Report successful construction

Defaults must be specified by the language contract, not invented by the parser.

9.3 Ordered collections

When source order is meaningful, AST collections must preserve it.

Examples include:

- Function arguments.
- Statements.
- Imports.
- Declarations.
- Generic arguments.
- Operation targets.
- HDL port declarations.
- Quantum control operands.
- Attribute lists where ordering is specified.

A validator must not assume that every collection is a set.

9.4 Arbitrary collection sizes

AST construction must not impose artificial language-level collection limits.

The implementation may encounter allocation failure or operational budgets.

These are resource failures, not syntax restrictions.

The AST must support collections whose sizes are determined by the source program and available resources.

---

10. Source provenance and spans

10.1 Mandatory provenance

Every AST node representing a source construct must preserve source provenance according to the canonical source-span contract.

At minimum, the system must be able to associate the node with:

- Source identity.
- Start position.
- End position.
- Relevant expansion origin, where macros are involved.

The exact Rust type must be shared with the existing source-span implementation.

This specification does not create a competing span type.

10.2 Span invariants

For a valid source span:

start <= end

The span must be valid relative to its source representation.

The validator must account for the repository's chosen indexing model.

If the implementation uses UTF-8 byte offsets, offsets must respect the documented byte-index contract.

Unicode character counts must not be substituted for byte offsets.

10.3 Composite nodes

Composite AST nodes must have source provenance consistent with their represented syntax.

The span policy must be deterministic.

The validator must verify that child spans are compatible with their parent according to the language's source-location rules.

10.4 Synthetic nodes

Compiler-generated nodes must distinguish synthetic provenance from directly parsed source nodes.

A synthetic node must not pretend to have a source location that does not exist.

Macro-generated nodes must retain enough expansion information for diagnostics and tooling.

10.5 Required tests

Test:

- Empty source.
- Single-token source.
- Multiline source.
- Unicode identifiers.
- Unicode quantum literals.
- Nested expressions.
- Nested blocks.
- Invalid UTF-8 boundaries, where relevant to input handling.
- Macro expansion.
- Generated AST nodes.
- Missing source files.
- Multiple source units.

---

11. AST coverage across computing domains

The validator must treat each computing domain as a first-class conformance area while preserving universal AST foundations.

11.1 Classical computing

Validate:

- Scalar expressions.
- Integer expressions.
- Floating-point expressions.
- Boolean expressions.
- Vector and matrix constructs.
- Tensor constructs.
- Symbolic mathematics.
- Numeric operations.
- Statistical operations.
- Signal processing.
- Scientific computing.
- Control operations.
- Optimization expressions.

Do not require one AST variant per mathematical function.

Generic operation representation is permitted when the semantic contract identifies the operation sufficiently.

11.2 Quantum computing

Quantum AST coverage must include:

- Qubit declarations.
- Parameterized quantum registers.
- Quantum state expressions.
- Quantum operations.
- Operation parameters.
- Quantum targets.
- Multiple targets.
- Controls.
- Adjoint operations.
- Measurement.
- Reset.
- Barriers.
- Observables.
- Quantum channels.
- Noise intent.
- Classical feed-forward.
- Dynamic quantum control.
- Logical operations.
- Error-correction requirements.
- Circuit declarations.
- Quantum kernels.
- Resource requirements.

Critical quantum invariant

The frontend AST must not require a closed enumeration of all possible quantum gates.

The AST must support data-driven operation identity.

Conceptually:

QuantumOperation
    operation identity
    namespace
    operands
    parameters
    modifiers
    attributes
    source provenance

The actual field representation must follow the existing canonical AST conventions.

Examples that must be representable:

apply H to q

apply custom_gate to q

apply vendor.operation to q

apply operation(parameter) to q0, q1

Unknown operation names may require semantic resolution, but they must not automatically become parser failures merely because the operation is absent from a hard-coded gate list.

The AST must not impose a fixed qubit count.

Parameterized sizes must remain representable.

The canonical downstream quantum mapping is:

Source
  ↓
Frontend AST
  ↓
Semantic quantum operation
  ↓
quantum::ir

No second frontend quantum IR may be introduced.

11.3 Hybrid computing

Validate:

- Classical-to-quantum boundaries.
- Quantum-to-classical boundaries.
- Measurement results.
- Classical feed-forward.
- Synchronization.
- Shared data.
- Hybrid functions.
- Resource requirements.

The AST must preserve the ordering and dependencies between classical and quantum operations.

11.4 HDL

Validate:

- Hardware modules.
- Ports.
- Signals.
- Nets.
- Registers.
- Combinational logic.
- Sequential logic.
- Clocking.
- Reset.
- Timing intent.
- Assertions.
- Interfaces.
- Protocols.
- State machines.
- Pipelines.
- Memories.
- Parameterized arrays.
- Generate constructs.
- Synthesis intent.
- Simulation constructs.
- Verification constructs.
- Physical intent.
- Software/hardware co-design.

Hardware widths and dimensions must be represented as program-level values or symbolic parameters where required.

The validator must not interpret a particular register width as a universal AST limit.

11.5 Hardware and resources

Validate the distinction between:

- Requirement.
- Capability.
- Constraint.
- Preference.
- Hint.
- Implementation decision.

For example:

requires capability("gpu.compute")

is portable semantic intent.

A physical device identifier is a realization detail.

The AST must preserve this distinction.

11.6 Distributed computing

Validate:

- Nodes.
- Processes.
- Services.
- Actors.
- Messages.
- Communication.
- Placement intent.
- Replication.
- Partitioning.
- Consistency.
- Transactions.
- Fault tolerance.
- Collective operations.
- Topology requirements.

No fixed node count is permitted.

11.7 AI and data

Validate:

- Models.
- Tensors.
- Datasets.
- Training.
- Inference.
- Differentiable computation.
- Probabilistic computation.
- Agents.
- Pipelines.
- Model deployment.
- Data schemas.
- Streams.
- Transformations.
- Provenance.

Framework-specific representations must not become mandatory core AST types.

11.8 Sankofa, MTS and nano computing

Retain existing design features through explicit lifecycle status.

Validate their AST contracts only when authoritative feature specifications exist.

Relevant constructs include:

- Temporal reasoning.
- Memory and recall.
- Learning and inference.
- Historical provenance.
- Consensus.
- Timeline operations.
- Observation.
- Speculation.
- Fork and merge.
- Nano agents.
- Atomic and molecular intent.

A feature in "Zamani-Grammar.md" must not automatically be treated as implemented.

---

12. AST semantic mapping

12.1 Required semantic mapping

Every AST node classified as implemented must have a documented semantic interpretation.

The mapping must identify:

- Semantic construct.
- Required inputs.
- Type requirements.
- Binding requirements.
- Effect requirements.
- Capability requirements.
- Resource requirements.
- Invalid states.
- Diagnostics.
- Downstream consumers.

12.2 Separation of responsibilities

The AST validator verifies the existence and consistency of semantic mappings.

It does not perform semantic analysis itself.

12.3 Generic operations

Generic operation AST nodes must preserve sufficient identity to support semantic resolution.

A generic operation must not lose:

- Qualified name.
- Operands.
- Parameters.
- Modifiers.
- Relevant attributes.
- Source provenance.

12.4 Semantic ambiguity

A construct must not be marked fully covered if its semantic meaning is unresolved.

Where an operation name is intentionally extensible, semantic resolution may be deferred.

The feature must still define the rules governing resolution.

---

13. AST-to-IR mapping

13.1 IR contract

Every feature that requires lowering must declare its canonical IR mapping.

The mapping must identify:

- Source AST construct.
- Semantic representation.
- Target IR.
- Lowering stage.
- Preserved information.
- Intentionally discarded syntactic information.
- Unsupported conditions.
- Diagnostics.
- Tests.

13.2 Canonical quantum IR

All frontend quantum constructs must converge on the established "quantum::ir" boundary.

The validator must detect architectural violations involving:

- Duplicate frontend quantum IR.
- Independent competing quantum operation models.
- Frontend AST types that leak physical qubit allocation.
- Fixed gate lists used as the sole operation representation.
- Loss of quantum operation parameters.
- Loss of measurement dependencies.
- Loss of source provenance.

13.3 Classical and HDL IR

Classical and HDL lowering must use their established canonical representations.

This document does not prescribe new IR names or force unrelated domains into one physical IR.

The canonical semantic boundary must remain explicit.

13.4 Deferred lowering

Some constructs may intentionally remain in a high-level representation until later compilation stages.

This is permitted if the specification defines:

- The deferred representation.
- Its required invariants.
- The responsible lowering stage.
- The consumers.
- The failure behavior.

---

14. AST constructor coverage

14.1 Constructor inventory

The validator must identify all supported AST constructors.

This includes:

- Struct constructors.
- Enum variants.
- Builder functions.
- Parser construction functions.
- Generated constructors, where applicable.
- Domain-specific constructors.

14.2 Missing constructors

An AST variant that is required by the specification but cannot be constructed by the parser is an error.

An AST variant that is intentionally constructed only by a later compiler stage must be explicitly classified as synthetic.

14.3 Dead AST variants

An AST variant with no valid construction path must be investigated.

Possible classifications:

- Planned.
- Experimental.
- Synthetic.
- Deprecated.
- Obsolete.
- Accidental dead code.

A dead variant must not be counted as successful frontend coverage merely because its Rust definition exists.

14.4 Construction completeness

For each AST constructor, tests must verify:

1. Correct node kind.
2. Correct field values.
3. Correct child ordering.
4. Correct optional values.
5. Correct source span.
6. Correct attributes.
7. Correct qualified names.
8. Correct literal preservation.
9. Correct domain identity.
10. Correct downstream mapping metadata.

---

15. AST coverage manifest

15.1 Required manifest location

Use the existing:

"grammar/specification/features/"

Do not create a competing feature registry.

Each feature manifest must contain or reference the following logical fields:

id: quantum.operations
name: Quantum operations
status: proposed
version: 1.0.0

specification:
  file: spec/quantum.md
  sections: []

grammar:
  files:
    - quantum/operations.g4
  rules: []

lexer:
  tokens: []

ast:
  module: canonical_frontend_ast
  node_kinds: []
  required_fields: []
  source_span_required: true

semantic:
  contract: ""
  status: planned

ir:
  canonical_boundary: quantum::ir
  mapping: ""
  status: planned

consumers:
  - semantic_analyzer
  - compiler

tests:
  positive: []
  negative: []
  boundary: []
  scalability: []
  determinism: []
  compatibility: []

portability:
  target_independent: true
  hardware_limits_in_grammar: false

compatibility:
  policy: ""

diagnostics:
  required: true

This is an illustrative contract schema, not a claim that the repository already uses this exact YAML format.

The implementation must choose one canonical serialization format and document its schema.

The schema must be versioned.

15.2 Manifest consistency

The validator must reject:

- Missing required fields.
- Invalid feature identifiers.
- Unknown referenced files.
- Unknown AST node kinds.
- Unknown grammar rules.
- Duplicate feature identities.
- Contradictory statuses.
- Missing mappings for implemented features.
- Unsupported lifecycle transitions.
- Stale references.
- Invalid canonical IR declarations.

15.3 No copied authority

A manifest must reference authoritative definitions.

It must not become another place where grammar rules or AST structures are independently redefined.

---

16. Validation algorithm

16.1 Required stages

The validator must execute the following logical stages:

1. Discover authoritative inputs
2. Load specification contracts
3. Load feature manifests
4. Load grammar inventory
5. Load AST inventory
6. Load parser construction inventory
7. Load semantic mapping inventory
8. Load IR mapping inventory
9. Build traceability graph
10. Validate references
11. Validate reachability
12. Validate AST construction coverage
13. Validate field completeness
14. Validate source provenance
15. Validate semantic coverage
16. Validate IR coverage
17. Validate lifecycle consistency
18. Validate test evidence
19. Validate portability
20. Generate deterministic report

Each stage must be independently testable.

16.2 Traceability graph

Represent relationships conceptually as:

Feature
  |
  +-- Specification
  |
  +-- Grammar rule
  |
  +-- Parser production
  |
  +-- AST node
  |
  +-- Semantic mapping
  |
  +-- IR mapping
  |
  +-- Tests

The graph must support:

- Multiple syntax forms mapping to one AST node.
- Multiple AST nodes contributing to one semantic construct.
- One AST node participating in multiple semantic analyses.
- Conditional mappings based on feature gates.
- Versioned compatibility mappings.
- Synthetic compiler-generated nodes.

Do not assume a one-to-one mapping.

16.3 Validation failures

Each finding must contain:

- Stable diagnostic identifier.
- Severity.
- Feature identifier.
- Affected file.
- Affected grammar rule, AST node, or mapping.
- Explanation.
- Expected contract.
- Actual evidence.
- Suggested corrective action.
- Relevant source location, when available.

16.4 Severity

Use:

Severity| Meaning
"ERROR"| Invalidates required conformance
"WARNING"| Requires investigation
"INFO"| Informational observation
"DEFERRED"| Explicitly incomplete feature
"SKIPPED"| Excluded by a documented validation profile

A skipped feature must not be counted as passing.

---

17. Required diagnostics

The validator must define stable diagnostics.

Suggested identifiers:

ID| Meaning
ASTC0001| Missing specification mapping
ASTC0002| Missing grammar mapping
ASTC0003| Unreachable grammar rule
ASTC0004| Grammar alternative lacks AST mapping
ASTC0005| AST node lacks grammar origin
ASTC0006| AST constructor missing
ASTC0007| Required AST field missing
ASTC0008| Invalid AST structural invariant
ASTC0009| AST node lacks source provenance
ASTC0010| Invalid source span
ASTC0011| Parser constructs incorrect node kind
ASTC0012| Required AST information lost
ASTC0013| Semantic mapping missing
ASTC0014| Canonical IR mapping missing
ASTC0015| Duplicate competing AST representation
ASTC0016| Unsupported AST variant marked implemented
ASTC0017| Feature manifest inconsistency
ASTC0018| Invalid feature lifecycle
ASTC0019| Missing positive test evidence
ASTC0020| Missing negative test evidence
ASTC0021| Missing boundary test evidence
ASTC0022| Missing scalability evidence
ASTC0023| Missing determinism evidence
ASTC0024| Missing compatibility evidence
ASTC0025| Nonportable target-specific AST leakage
ASTC0026| Fixed resource limit in universal AST contract
ASTC0027| Quantum operation representation is unnecessarily closed
ASTC0028| Duplicate frontend quantum IR boundary
ASTC0029| AST collection ordering contract violated
ASTC0030| Inconsistent source provenance
ASTC0031| Stale traceability reference
ASTC0032| Unregistered AST node kind
ASTC0033| Unsupported generated AST node
ASTC0034| Incomplete cross-domain AST contract
ASTC0035| Nondeterministic coverage report
ASTC0036| Validation resource exhaustion
ASTC0037| Invalid manifest schema version
ASTC0038| Ambiguous AST equivalence declaration
ASTC0039| AST compatibility violation
ASTC0040| Invalid feature dependency

Identifiers are stable public diagnostic contracts and must not be reused for unrelated errors.

Additional diagnostics may be added without renumbering existing identifiers.

---

18. Scalability requirements

18.1 Fundamental principle

The AST validator must scale from tiny source programs to arbitrarily large source programs, subject only to actual available computational resources.

This is an architectural scalability objective, not a claim of physically infinite memory or execution capacity.

No language-level maximum may be inferred from validator implementation capacity.

18.2 Prohibited fixed limits

The validator must not contain universal AST or language restrictions such as:

MAX_AST_NODES
MAX_GRAMMAR_RULES
MAX_FEATURES
MAX_AST_DEPTH
MAX_SOURCE_FILES
MAX_QUANTUM_OPERATIONS
MAX_QUBITS
MAX_TENSOR_RANK
MAX_REGISTER_WIDTH
MAX_THREADS
MAX_NODES

The appearance of a number in a test, benchmark, or operational configuration is not automatically prohibited.

The prohibited behavior is treating a chosen capacity as a universal language or AST limit.

18.3 Resource-aware validation

The validator may support configurable execution budgets.

Examples:

- Maximum wall-clock time for one CI job.
- Maximum memory budget for one validation process.
- Maximum parallel validation workers.
- Maximum report size per output artifact.
- Maximum recursion depth for an individual operational execution profile.

These are operational safeguards.

They must:

1. Be configurable.
2. Be explicitly distinguished from language semantics.
3. Produce resource-exhaustion diagnostics.
4. Never silently truncate successful validation.
5. Never report an incomplete run as complete.
6. Allow incremental or partitioned validation.
7. Preserve the ability to validate larger programs using more resources.

18.4 Complexity requirements

The validator should avoid repeated full-repository scans.

Where practical:

- Build inventories once.
- Reuse parsed metadata.
- Use indexed identifiers.
- Cache stable intermediate results.
- Track dependency changes.
- Validate affected dependency closures.
- Use iterative traversal for potentially deep structures.
- Avoid unnecessary cloning of large ASTs.
- Stream large reports where appropriate.

No complexity guarantee should be claimed without measurement.

18.5 Deep ASTs

The validator must not assume that source nesting is shallow.

For potentially deep AST traversal, prefer explicit work stacks over recursive traversal where stack exhaustion is a credible risk.

A resource exhaustion event must be reported distinctly from an invalid AST.

18.6 Large ASTs

Validation must be able to process large AST inventories incrementally.

It must distinguish:

- Incomplete inventory.
- Failed inventory.
- Complete inventory.
- Cached inventory.
- Stale inventory.

Partial data must never be presented as complete coverage.

18.7 Parallel validation

Parallel validation is permitted when:

- The result is deterministic.
- Shared data is safely synchronized.
- No unsafe Rust is required.
- Output ordering is stable.
- Errors are collected deterministically.

Parallelism is an implementation strategy, not a semantic requirement.

---

19. Determinism

The same repository state, configuration, toolchain, and input artifacts must produce the same logical coverage report.

Determinism applies to:

- Feature discovery.
- Grammar inventory.
- AST inventory.
- Traceability edges.
- Diagnostic identifiers.
- Diagnostic ordering.
- Coverage summaries.
- Serialization.
- Hashing.
- Cache keys.

Do not depend on:

- Hash-map iteration order.
- Thread completion order.
- Memory addresses.
- Unstable filesystem traversal order.
- Current wall-clock time.
- Unseeded randomness.

If timestamps are included in reports, they must be excluded from the deterministic content hash.

The validator must distinguish deterministic report content from execution metadata.

---

20. Rust implementation requirements

20.1 Toolchain

Target:

[package]
edition = "2021"
rust-version = "1.97"

The actual repository manifest must be checked before adopting or changing its existing Rust version.

Rust 1.97.1 must be tested explicitly if it is the selected project toolchain.

20.2 Unsafe prohibition

No unsafe Rust is permitted.

The implementation must not introduce:

unsafe

including:

- Unsafe blocks.
- Unsafe functions.
- Unsafe trait implementations.
- Unsafe operations.
- Unsafe extern interfaces.

The validation module must use safe Rust APIs.

20.3 Error handling

The validator must use structured error types.

Errors must distinguish:

- Invalid input.
- Missing file.
- Malformed manifest.
- Invalid reference.
- Incomplete inventory.
- Structural AST violation.
- Semantic coverage gap.
- IR coverage gap.
- Resource exhaustion.
- I/O failure.
- Configuration failure.

Do not convert failures into empty successful inventories.

20.4 No panic-based validation

Untrusted repository metadata must not cause routine validation panics.

Avoid unchecked indexing and assumptions about manifest completeness.

Malformed input must produce structured diagnostics.

20.5 Dependency policy

Use existing repository dependencies where suitable.

Do not add a new dependency merely to duplicate functionality already available in the project.

New dependencies require:

- Purpose.
- Maintenance assessment.
- License compatibility.
- Rust-version compatibility.
- Security assessment.
- Determinism assessment.
- Feature justification.

20.6 Data ownership

The validator should consume immutable inventory data where possible.

It must not mutate canonical AST definitions during validation.

Generated reports must be separate from authoritative source contracts.

---

21. Test requirements

21.1 Test categories

Every validator feature must have:

positive/
negative/
boundary/
scalability/
determinism/
compatibility/
integration/

21.2 Positive tests

Verify:

1. Every valid grammar alternative maps to its expected AST node.
2. Required AST fields are populated.
3. Source spans are correct.
4. Generic arguments are preserved.
5. Qualified names are preserved.
6. Attributes are preserved.
7. Child ordering is preserved.
8. Domain identity is preserved.
9. Semantic mappings are discoverable.
10. IR mappings are discoverable.
11. Valid feature manifests pass.
12. Valid AST compatibility declarations pass.

21.3 Negative tests

Verify rejection of:

1. Missing AST mappings.
2. Unknown AST nodes.
3. Missing required fields.
4. Unreachable required grammar rules.
5. Incorrect parser constructors.
6. Invalid source spans.
7. Lost operands.
8. Lost operation parameters.
9. Missing semantic mappings.
10. Missing IR mappings for fully implemented features.
11. Duplicate feature identities.
12. Invalid manifest references.
13. Invalid lifecycle transitions.
14. Duplicate competing AST types.
15. Target-specific physical mapping in portable source AST.
16. Incorrectly closed quantum operation models.
17. Unregistered AST variants.
18. Nondeterministic report ordering.
19. Malformed metadata.
20. Stale dependency references.

21.4 Boundary tests

Test:

- Empty AST.
- Single-node AST.
- One source file.
- Multiple source files.
- Deeply nested expressions.
- Deeply nested declarations.
- Empty collections.
- Large collections.
- Unicode source.
- Multiline spans.
- Macro-generated nodes.
- Missing optional fields.
- Maximum representable integer literals.
- Symbolic resource dimensions.
- Dynamic quantum register sizes.
- Large HDL parameter expressions.
- Multiple domain boundaries.

Boundary tests must not establish artificial language maxima.

21.5 Quantum tests

Required cases:

One qubit
Two qubits
Parameterized qubit register
Symbolic qubit count
Large qubit declaration
Custom operation
Qualified operation
Operation parameters
Multiple operands
Multiple controls
Adjoint operation
Measurement
Mid-circuit measurement
Classical feed-forward
Dynamic control
Logical qubit
Physical mapping metadata
Unknown operation name

The test suite must verify that an unknown operation is handled according to semantic-resolution policy rather than rejected merely because a fixed parser enumeration does not contain it.

21.6 HDL tests

Required cases:

Parameterized module
Symbolic port width
Parameterized memory
Combinational logic
Sequential logic
Parameterized pipeline
Timing constraints
State machine
Verification assertion
Hardware/software boundary
Target-independent physical intent

21.7 Cross-domain tests

Verify AST interoperability across:

- Classical + quantum.
- Quantum + HDL.
- Classical + HDL.
- Quantum + AI.
- AI + distributed.
- Distributed + networking.
- Hardware + resource requirements.
- Effects + quantum.
- Memory + concurrency.
- Macros + domain constructs.

The objective is not to force identical AST node types across domains.

The objective is to ensure shared foundations and consistent contracts.

---

22. Coverage metrics

22.1 Required metrics

The validator must report separate metrics for:

- Specification coverage.
- Grammar rule coverage.
- Grammar alternative coverage.
- Parser construction coverage.
- AST node coverage.
- AST field coverage.
- Source provenance coverage.
- Semantic mapping coverage.
- IR mapping coverage.
- Test evidence coverage.
- Compatibility coverage.
- Scalability coverage.
- Determinism coverage.

22.2 No misleading aggregate

Do not report a single percentage as proof of production readiness.

A project can have 100% AST-node coverage while missing semantic mappings.

A project can have 100% grammar-rule coverage while losing source spans.

A project can have 100% parser test coverage while failing to lower quantum operations.

Separate metrics must remain visible.

22.3 Coverage calculation

For each measurable category:

coverage =
    verified_required_items
    /
    total_required_items

Only applicable, authoritative requirements belong in the denominator.

Unknown, missing, or unverified items must not be silently excluded.

When the denominator is zero, report "NOT_APPLICABLE", not 100%.

22.4 Status aggregation

The overall feature status is determined by the least complete mandatory stage.

Optional stages must be explicitly identified as optional in the feature contract.

A required stage that fails prevents the feature from being reported as fully conformant.

---

23. Feature lifecycle integration

AST coverage must respect the lifecycle established by "Zamani-Grammar.md".

Historical / proposed feature
          |
          v
Feature proposal
          |
          v
Semantic design
          |
          v
AST contract
          |
          v
Grammar contract
          |
          v
Implementation
          |
          v
Canonical IR mapping
          |
          v
Conformance tests
          |
          v
Stable feature

The validator must support:

- "stable"
- "proposed"
- "experimental"
- "deprecated"
- "historical"
- "not implemented"

A proposed feature may have an AST design without a working parser.

That is not a failure if correctly declared.

It is a failure if reported as implemented.

A deprecated AST node must have an explicit migration or compatibility policy.

Historical material must not automatically become valid syntax.

---

24. Compatibility requirements

24.1 Existing AST compatibility

The validator must identify whether an AST change is:

- Additive.
- Behavior-preserving.
- Breaking.
- Deprecated.
- Migration-required.

24.2 AST schema evolution

Every incompatible AST schema change must specify:

- Previous representation.
- New representation.
- Reason.
- Migration behavior.
- Compatibility impact.
- Affected consumers.
- Test evidence.

24.3 Existing frontend

Do not assume that the current AST is incomplete merely because a new specification proposes additional nodes.

Compare the specification against actual implementation.

Do not rename established AST types without evidence and a migration plan.

24.4 Serialization

If AST serialization exists, the validator must verify compatibility with the established serialization contract.

Do not introduce serialization guarantees that the project does not otherwise support.

---

25. Hard-coding audit

The AST coverage validator must inspect universal AST contracts for artificial capacity assumptions.

Suspicious identifiers include:

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

The audit must distinguish:

1. Actual language-level maximums.
2. Runtime operational budgets.
3. Target-specific constraints.
4. Test fixture values.
5. Mathematical constants.
6. Symbolic program requirements.

A numerical literal is not automatically a violation.

For example:

let n = 1024;
allocate qubits[n];

is valid program-level data.

A universal AST rule requiring all quantum registers to contain at most 1024 qubits is not acceptable.

The validator must detect architectural violations, not blindly reject every occurrence of a number.

---

26. Portability and POCO-REAF

AST coverage is necessary but not sufficient for POCO-REAF.

The AST must preserve portable intent separately from implementation decisions.

The validator must verify the distinction between:

Requirement

requires qubits >= n

Capability

requires capability("quantum.measurement")

Preference

prefer accelerator("quantum")

Implementation decision

Physical device assignment

Implementation decisions must not be silently embedded into the portable source AST as universal program meaning.

Target realization must remain downstream.

The AST must be expressive enough to preserve the source program's intent without requiring today's physical machine layout.

---

27. Validation profiles

The validator must support explicit profiles.

27.1 Development profile

Purpose:

- Fast feedback.
- Changed-feature validation.
- Local development.

Must still report incomplete validation honestly.

27.2 Feature profile

Purpose:

- Validate one feature and its dependency closure.

Must include all required upstream and downstream contracts.

27.3 Full repository profile

Purpose:

- Validate all authoritative feature contracts.

This is the required production acceptance profile.

27.4 Compatibility profile

Purpose:

- Validate supported previous AST and syntax contracts.

27.5 Release profile

Purpose:

- Validate the complete release candidate.

Must require:

- No unresolved mandatory AST errors.
- No invalid implemented feature mappings.
- No unapproved compatibility break.
- No missing required conformance tests.
- No unsafe Rust introduced by the validator.
- Deterministic reports.
- Complete validation inventory.

Profiles must not silently weaken required release checks.

---

28. Incremental validation and dependency tracking

28.1 Objective

A completed feature should not require arbitrary re-editing whenever an unrelated feature changes.

The validator must support explicit dependency tracking.

28.2 Dependency contract

Every feature must declare:

- Direct dependencies.
- Upstream contracts.
- Downstream consumers.
- Shared AST foundations.
- Shared lexical dependencies.
- Shared semantic dependencies.

28.3 Change propagation

When a file changes, validation must identify the affected dependency closure.

For example:

lexer/tokens.md
      |
      v
lexer/keywords.md
      |
      v
grammar rule
      |
      v
parser production
      |
      v
AST mapping
      |
      v
semantic mapping
      |
      v
IR mapping
      |
      v
tests

Unrelated features must not be rebuilt unnecessarily.

However, a shared foundational change must trigger all dependent validations.

28.4 Stable contract principle

A file is complete when:

1. Its own contract is complete.
2. All dependencies are declared.
3. Its public interfaces are stable.
4. Its downstream expectations are specified.
5. Its tests pass.
6. Its compatibility policy exists.
7. Its integration evidence is recorded.

Later changes to another file may require validation again, but must not require changing this file unless its declared contract changes.

This is the required distinction between revalidation and rework.

---

29. Required integration with existing validation documents

Document| Integration responsibility
"validation/README.md"| Entry point and execution profiles
"validation/grammar-validator.md"| Grammar inventory and reachability
"validation/ast-coverage.md"| Grammar-to-AST traceability
"validation/semantic-coverage.md"| AST-to-semantic completeness
"validation/ir-coverage.md"| Semantic-to-IR completeness
"validation/source-spans.md"| AST provenance correctness
"validation/determinism.md"| Deterministic AST validation
"validation/scalability.md"| Resource-aware validation
"validation/compatibility.md"| AST evolution and migrations
"validation/hard-coding.md"| Capacity and portability audit
"validation/ambiguity.md"| Grammar ambiguity
"validation/unreachable-rules.md"| Rule reachability
"validation/duplicate-tokens.md"| Lexer identity consistency
"validation/ast-coverage.md"| Cross-file AST traceability authority

These documents must reference one another without redefining each other's ownership.

---

30. Completion contract for this file

This document is complete when:

- [x] Purpose defined.
- [x] Ownership defined.
- [x] Non-ownership defined.
- [x] Authority hierarchy defined.
- [x] AST coverage model defined.
- [x] Coverage states defined.
- [x] Grammar-to-AST traceability defined.
- [x] Constructor coverage defined.
- [x] AST structural requirements defined.
- [x] Source-span requirements defined.
- [x] Semantic integration defined.
- [x] IR integration defined.
- [x] Quantum integration defined.
- [x] Classical integration defined.
- [x] HDL integration defined.
- [x] Hybrid integration defined.
- [x] Hardware/resource integration defined.
- [x] Distributed integration defined.
- [x] AI/data integration defined.
- [x] Sankofa/MTS/nano lifecycle integration defined.
- [x] Manifest contract defined.
- [x] Diagnostic identifiers defined.
- [x] Scalability contract defined.
- [x] Determinism defined.
- [x] Rust 1.97 compatibility requirement defined.
- [x] Unsafe Rust prohibited.
- [x] Positive tests defined.
- [x] Negative tests defined.
- [x] Boundary tests defined.
- [x] Scalability tests defined.
- [x] Compatibility tests defined.
- [x] Portability requirements defined.
- [x] Hard-coding audit defined.
- [x] Incremental validation defined.
- [x] Integration with existing validation files defined.
- [x] Completion criteria defined.

These checkmarks indicate specification coverage, not verification that the repository implementation already satisfies the requirements.

---

31. Production acceptance criteria

The implementation may be declared production-ready only when all applicable requirements below pass.

31.1 Structural acceptance

- Every implemented grammar alternative has an authoritative AST mapping.
- Every implemented AST node has a declared owner.
- Every required AST constructor exists.
- Every required AST field is populated.
- Every AST node satisfies its structural invariants.
- No required source information is silently discarded.
- Source provenance is valid.

31.2 Cross-phase acceptance

- Every implemented feature has a semantic contract.
- Every feature requiring lowering has a canonical IR mapping.
- Quantum lowering converges on "quantum::ir".
- No duplicate frontend quantum IR exists.
- Unsupported features are not falsely marked implemented.

31.3 Domain acceptance

- Classical computing is covered.
- Quantum computing is covered.
- Hybrid computing is covered.
- HDL is covered.
- Hardware intent is covered.
- Resource requirements are covered.
- Distributed computing is covered.
- Other accepted domains are covered according to their manifests.

31.4 Scalability acceptance

- No artificial language-level AST capacities exist.
- Operational budgets are explicitly separated from language semantics.
- Resource exhaustion is reported correctly.
- Large inventories can be processed incrementally.
- Deep structures are handled safely.
- No unsafe Rust is used.
- Deterministic output is preserved.

31.5 Testing acceptance

- Positive tests pass.
- Negative tests pass.
- Boundary tests pass.
- Scalability tests pass.
- Determinism tests pass.
- Compatibility tests pass.
- Integration tests pass.
- No mandatory validation stage is silently skipped.

31.6 Documentation acceptance

- All integration references resolve.
- All ownership boundaries are explicit.
- Feature lifecycle statuses agree.
- No competing AST authority exists.
- No duplicate grammar authority is introduced.

---

32. Final architectural guarantee

The AST coverage validator must establish the following invariant:

Every implemented Zamani language construct
has a verified, traceable, structurally correct
frontend AST representation whose meaning can
be interpreted by the appropriate semantic phase
and mapped to its canonical downstream IR.

For POCO-REAF, it must additionally establish:

Portable source intent
        |
        v
Canonical frontend AST
        |
        v
Semantic interpretation
        |
        v
Canonical IR
        |
        v
Target-independent optimization
        |
        v
Target-specific realization

The AST must represent what the programmer means, not what a particular machine happens to support today.

Physical resource availability, hardware topology, calibration, routing, scheduling, resilience, QEC, ZQN, and HAL remain downstream concerns.

Final rule: A language feature is not AST-covered merely because its grammar accepts the source or because a Rust enum contains a corresponding variant. It is covered only when its complete declared contract is traceable, constructible, structurally valid, source-preserving, semantically accounted for, and supported by reproducible conformance evidence.
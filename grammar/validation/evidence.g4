/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/validation/evidence.g4
 *
 * GRAMMAR
 * -------
 * EvidenceValidation
 *
 * STATUS
 * ------
 * Production validation/conformance facade
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file provides the validation boundary for source constructs that may
 * produce, require, consume, or be associated with semantic evidence.
 *
 * IMPORTANT:
 *
 * This file is NOT an owner of evidence source syntax.
 *
 * Evidence is a semantic and provenance concept. The current Zamani grammar
 * architecture does not require a universal source-level:
 *
 *     evidence(...)
 *
 *     evidenceStatement
 *
 *     certificate(...)
 *
 *     witness(...)
 *
 *     proof(...)
 *
 * syntax.
 *
 * Therefore this grammar deliberately does NOT invent such syntax.
 *
 * Instead, it validates canonical source constructs that may become claims,
 * obligations, assertions, contracts, properties, or verification subjects
 * to which evidence can be attached downstream.
 *
 * The architecture is:
 *
 *     canonical source construct
 *             |
 *             v
 *         domain-neutral AST
 *             |
 *             v
 *       semantic validation
 *             |
 *             v
 *       claim / obligation
 *             |
 *             v
 *           evidence
 *             |
 *             v
 *         provenance
 *             |
 *             v
 *       verification / decision
 *
 * This separation is intentional.
 *
 * ============================================================================
 * IMPLEMENTATION BASELINE
 * ============================================================================
 *
 * Rust:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust Edition 2021
 *     Safe Rust only
 *
 * ANTLR:
 *
 *     ANTLR4 parser grammar
 *
 * This grammar contains:
 *
 *     - no embedded Rust;
 *     - no semantic actions;
 *     - no semantic predicates;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware access;
 *     - no runtime execution;
 *     - no resource allocation;
 *     - no target selection;
 *     - no backend selection;
 *     - no solver selection;
 *     - no proof-engine selection;
 *     - no machine-capacity constants.
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * Provide one complete-input validation entry point for canonical source
 * constructs that may participate in the evidence/provenance pipeline.
 *
 * Evidence itself is not parser-owned.
 *
 * Evidence is attached by semantic/provenance infrastructure after canonical
 * source constructs have been parsed.
 *
 * ============================================================================
 * OWNS
 * ============================================================================
 *
 * This file owns only:
 *
 *     evidenceValidationUnit
 *     evidenceValidationSubject
 *
 * These are validation-only parser entry points.
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     evidence syntax
 *     evidence records
 *     certificates
 *     witnesses
 *     proof syntax
 *     theorem syntax
 *     solver syntax
 *     tactic syntax
 *     assertion syntax
 *     property syntax
 *     contract syntax
 *     refinement syntax
 *     expressions
 *     types
 *     identifiers
 *     names
 *     lexer tokens
 *     punctuation
 *     AST structures
 *     semantic evidence structures
 *     provenance structures
 *     proof algorithms
 *     theorem proving
 *     model checking
 *     symbolic execution
 *     solver invocation
 *     resource allocation
 *     capability resolution
 *     policy evaluation
 *     IR generation
 *     quantum lowering
 *     HDL lowering
 *     hardware realization
 *     runtime execution.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON
 * ----------
 *
 *     grammar/statements/assertions.g4
 *
 *         AssertionsParser
 *
 *         assertionStatement
 *
 *     grammar/statements/contract.g4
 *
 *         ContractStatements
 *
 *         requiresStatement
 *         ensuresStatement
 *         invariantStatement
 *         assumeStatement
 *         guaranteeStatement
 *         propertyStatement
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 *         indirectly through:
 *
 *             tokenVocab = ZamaniLexer
 *
 *     canonical expression grammar
 *
 *         indirectly through the canonical statement owners.
 *
 * ============================================================================
 * EXPORTS
 * ============================================================================
 *
 *     evidenceValidationUnit
 *     evidenceValidationSubject
 *
 * These are validation-only parser entry points.
 *
 * They are NOT production source-language constructs.
 *
 * ============================================================================
 * CONSUMED_BY
 * ============================================================================
 *
 * This facade may be consumed by:
 *
 *     grammar/tests/
 *     grammar-validation tooling
 *     parser conformance tooling
 *     semantic coverage tooling
 *     evidence-boundary tests
 *     provenance-boundary tests
 *     verification-boundary tests
 *     IDE/parser diagnostics
 *     grammar regression infrastructure.
 *
 * It MUST NOT replace the canonical production program parser.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar creates parser contexts only.
 *
 * The AST remains owned by the domain-neutral frontend AST subsystem.
 *
 * A source construct parsed through this validation facade MUST map to the
 * same canonical AST representation used by the production parser.
 *
 * There must not be an EvidenceValidationAst or EvidenceStatementAst merely
 * because the input was parsed through this facade.
 *
 * Conceptually:
 *
 *     canonical source construct
 *             |
 *             v
 *       canonical AST
 *             |
 *             v
 *       semantic claim/
 *       obligation/property
 *             |
 *             v
 *          evidence
 *
 * The exact Rust representation is owned by the AST and semantic subsystems.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis determines whether a canonical source construct:
 *
 *     - constitutes a claim;
 *     - constitutes an obligation;
 *     - requires evidence;
 *     - may provide evidence;
 *     - consumes evidence;
 *     - has sufficient evidence;
 *     - has contradictory evidence;
 *     - has incomplete evidence;
 *     - requires formal verification;
 *     - permits runtime verification;
 *     - permits probabilistic evidence;
 *     - permits experimental evidence;
 *     - permits generated evidence;
 *     - permits external evidence.
 *
 * None of these decisions are made by this grammar.
 *
 * A syntactically valid source construct is not automatically:
 *
 *     proven;
 *     verified;
 *     trusted;
 *     correct;
 *     safe;
 *     realizable;
 *     executable.
 *
 * Those properties belong to downstream semantic and verification systems.
 *
 * ============================================================================
 * EVIDENCE SEMANTIC MODEL
 * ============================================================================
 *
 * The semantic evidence model SHOULD remain domain-neutral.
 *
 * Conceptually:
 *
 *     Evidence
 *     {
 *         subject
 *         claim
 *         source
 *         kind
 *         content
 *         confidence
 *         derivation
 *         verification
 *         provenance
 *         dependencies
 *         status
 *     }
 *
 * Possible semantic evidence origins include:
 *
 *     static analysis
 *     formal proof
 *     theorem proving
 *     model checking
 *     symbolic execution
 *     testing
 *     simulation
 *     measurement
 *     experiment
 *     runtime observation
 *     compiler transformation
 *     hardware verification
 *     quantum verification
 *     HDL verification
 *     AI/model evaluation
 *     distributed-system observation
 *     external trusted source
 *     generated certificate.
 *
 * These are semantic categories.
 *
 * They are deliberately NOT turned into an ever-growing list of core grammar
 * keywords.
 *
 * ============================================================================
 * EVIDENCE OWNERSHIP BOUNDARY
 * ============================================================================
 *
 * This file owns the validation boundary only.
 *
 * Semantic evidence ownership belongs to the semantic/provenance subsystem.
 *
 * Verification infrastructure owns:
 *
 *     proof checking
 *     certificate checking
 *     model checking
 *     solver execution
 *     evidence validation
 *     evidence combination
 *     trust decisions
 *     verification status.
 *
 * Provenance infrastructure owns:
 *
 *     source
 *     derived_from
 *     generated_by
 *     transformed_by
 *     verified_by
 *     evidence
 *     decision
 *     version
 *     execution context.
 *
 * The grammar must not duplicate those systems.
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * Evidence-bearing source constructs use the canonical Zamani expression and
 * type systems.
 *
 * This grammar does not introduce:
 *
 *     EvidenceType
 *     ProofType
 *     CertificateType
 *     QuantumEvidenceType
 *     HardwareEvidenceType
 *     AIEvidenceType
 *
 * or any other domain-specific universal evidence type.
 *
 * The semantic type system determines the type of:
 *
 *     claim
 *     subject
 *     evidence
 *     confidence
 *     measurement
 *     result
 *     source
 *     derivation
 *     verification state.
 *
 * Existing type-system facilities remain authoritative, including where
 * applicable:
 *
 *     generics
 *     constraints
 *     associated types
 *     linear types
 *     affine types
 *     dependent types
 *     refinement semantics
 *     uncertainty types
 *     probability types.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Parsing evidence-bearing source constructs has no runtime effect.
 *
 * Downstream evidence production or verification may have effects including:
 *
 *     IO
 *     network
 *     native
 *     foreign
 *     distributed
 *     randomness
 *     measurement
 *     simulation
 *     learning
 *     adaptation
 *     reflection
 *     code_generation
 *
 * Effect analysis remains owned by:
 *
 *     grammar/effects/
 *
 * and the corresponding semantic implementation.
 *
 * This grammar does not create an evidence-specific effect system.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Evidence generation or verification may require capabilities.
 *
 * Examples of semantic capabilities may include:
 *
 *     capability("verification")
 *     capability("formal.verification")
 *     capability("model.checking")
 *     capability("symbolic.execution")
 *     capability("quantum.verification")
 *     capability("hardware.verification")
 *     capability("simulation")
 *
 * These are semantic capabilities.
 *
 * This grammar:
 *
 *     - does not define the capabilities;
 *     - does not inspect available hardware;
 *     - does not select a verifier;
 *     - does not select a solver;
 *     - does not authorize external execution.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Evidence generation and verification can consume resources.
 *
 * Resource interpretation belongs to:
 *
 *     grammar/resources/
 *
 * and the downstream semantic/compiler/runtime layers.
 *
 * Possible resource dimensions include:
 *
 *     memory
 *     computation
 *     parallelism
 *     storage
 *     communication
 *     accelerator availability
 *     simulation capacity
 *     verification capacity.
 *
 * This grammar introduces no fixed limits for any of them.
 *
 * In particular, this file contains no grammar-level maximum for:
 *
 *     evidence count
 *     claim count
 *     proof size
 *     certificate size
 *     premise count
 *     source size
 *     expression size
 *     memory
 *     processors
 *     accelerators
 *     QPUs
 *     qubits
 *     nodes
 *     devices
 *     tensor rank
 *     network size.
 *
 * Actual verification budgets are implementation policies or resource
 * constraints, not language ceilings.
 *
 * ============================================================================
 * CONTRACT CONTRACT
 * ============================================================================
 *
 * Evidence may be associated with canonical:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *     assert
 *
 * statements.
 *
 * These constructs remain owned by their canonical statement grammars.
 *
 * This file does NOT duplicate them.
 *
 * The relationship is:
 *
 *     canonical construct
 *             |
 *             v
 *       semantic claim/
 *       obligation
 *             |
 *             v
 *          evidence
 *
 * An evidence record therefore does not change the syntax owner of the
 * construct that generated the claim.
 *
 * ============================================================================
 * PROOF INTEGRATION
 * ============================================================================
 *
 * `grammar/validation/proofs.g4` currently validates canonical property
 * syntax as a possible source of proof obligations.
 *
 * Evidence may later be attached to those proof obligations by semantic and
 * verification infrastructure.
 *
 * Therefore:
 *
 *     property
 *         |
 *         v
 *     proof obligation
 *         |
 *         v
 *     evidence
 *         |
 *         v
 *     verification result
 *
 * This file does not redefine the proof boundary.
 *
 * ============================================================================
 * ASSERTION INTEGRATION
 * ============================================================================
 *
 * `grammar/statements/assertions.g4` is the canonical owner of:
 *
 *     assertionStatement
 *
 * This file may validate an assertion as an evidence-bearing subject.
 *
 * It does not redefine:
 *
 *     assert(...)
 *
 * syntax.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Evidence is inherently provenance-sensitive.
 *
 * Downstream infrastructure MUST be able to associate evidence with its
 * subject and preserve sufficient provenance to answer:
 *
 *     What was claimed?
 *     Where did the claim originate?
 *     What evidence supports it?
 *     How was the evidence generated?
 *     What transformations occurred?
 *     What verified the evidence?
 *     Which tool/version participated?
 *     Which policy authorized its use?
 *     Which assumptions were active?
 *     Which source version was involved?
 *
 * At minimum, provenance should preserve where applicable:
 *
 *     source file
 *     source module
 *     source span
 *     source ordering
 *     enclosing declaration
 *     canonical construct kind
 *     language version
 *     grammar version
 *     semantic version
 *     evidence derivation
 *     verification context.
 *
 * Later stages may add:
 *
 *     generated_by
 *     transformed_by
 *     verified_by
 *     evidence
 *     decision
 *     execution context
 *     target-independent compilation provenance.
 *
 * This grammar does not implement provenance storage.
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Evidence may be governed by policies concerning:
 *
 *     trust
 *     reproducibility
 *     verification methods
 *     external tools
 *     network access
 *     native execution
 *     foreign execution
 *     solver usage
 *     certificate acceptance
 *     proof caching
 *     evidence freshness
 *     evidence provenance
 *     deployment requirements.
 *
 * Policy evaluation belongs downstream.
 *
 * Parsing a canonical source construct MUST NOT grant permission to:
 *
 *     execute;
 *     verify;
 *     access external evidence;
 *     access hardware;
 *     access a solver;
 *     access a network;
 *     modify program state.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar produces NO IR.
 *
 * The complete architecture remains:
 *
 *     source
 *         |
 *         v
 *     lexer
 *         |
 *         v
 *     parser
 *         |
 *         v
 *     domain-neutral AST
 *         |
 *         v
 *     structural validation
 *         |
 *         v
 *     semantic analysis
 *         |
 *         +--> types
 *         +--> effects
 *         +--> capabilities
 *         +--> resources
 *         +--> contracts
 *         +--> policies
 *         +--> provenance
 *         +--> evidence
 *         |
 *         v
 *     canonical semantic representation
 *         |
 *         +--> classical representation
 *         +--> quantum::ir
 *         +--> HDL/hardware representation
 *         +--> distributed representation
 *         +--> accelerator representation
 *         +--> future domain representation
 *         |
 *         v
 *     optimization
 *         |
 *         v
 *     lowering
 *         |
 *         v
 *     routing / scheduling / resilience where applicable
 *         |
 *         v
 *     ZQN / HAL / target realization where applicable
 *
 * Evidence validation MUST NOT create another IR.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Evidence may concern quantum computation, including:
 *
 *     state properties
 *     circuit properties
 *     measurement results
 *     observable relationships
 *     equivalence
 *     resource requirements
 *     resilience
 *     error correction
 *     hybrid classical/quantum invariants
 *     simulation results.
 *
 * This grammar does not determine whether a subject is quantum.
 *
 * It MUST NOT:
 *
 *     - enumerate quantum operations;
 *     - enumerate physical qubits;
 *     - encode coupling maps;
 *     - encode calibration;
 *     - perform decomposition;
 *     - perform routing;
 *     - perform scheduling;
 *     - implement QEC;
 *     - select a QPU;
 *     - create another quantum IR.
 *
 * The canonical quantum path remains:
 *
 *     AST
 *       ->
 *     quantum semantic model
 *       ->
 *     quantum::ir
 *       ->
 *     optimization
 *       ->
 *     decomposition
 *       ->
 *     routing
 *       ->
 *     scheduling
 *       ->
 *     resilience / QEC
 *       ->
 *     ZQN
 *       ->
 *     HAL
 *       ->
 *     target realization
 *
 * Evidence may validate or constrain results at these stages, but it does not
 * replace the quantum pipeline.
 *
 * ============================================================================
 * HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * Evidence may concern:
 *
 *     signal relationships
 *     timing properties
 *     protocol correctness
 *     state-machine behavior
 *     synthesis properties
 *     hardware invariants
 *     simulation results
 *     implementation verification.
 *
 * This file does not encode:
 *
 *     fixed signal widths
 *     fixed register counts
 *     fixed memory capacities
 *     fixed FPGA dimensions
 *     fixed ASIC resources
 *     fixed pipeline depths
 *     fixed device counts
 *     physical device identifiers.
 *
 * Hardware realization remains downstream.
 *
 * ============================================================================
 * CLASSICAL BOUNDARY
 * ============================================================================
 *
 * Evidence may concern:
 *
 *     numerical results
 *     symbolic derivations
 *     algorithmic invariants
 *     type properties
 *     optimization correctness
 *     deterministic computations
 *     test results
 *     static analysis.
 *
 * No separate classical evidence syntax is introduced.
 *
 * ============================================================================
 * HYBRID BOUNDARY
 * ============================================================================
 *
 * Evidence may relate:
 *
 *     classical computation
 *     quantum computation
 *     measurement
 *     control
 *     resources
 *     execution state
 *     AI/model results.
 *
 * The syntax remains unchanged.
 *
 * The semantic system determines the participating domains.
 *
 * ============================================================================
 * AI / REASONING BOUNDARY
 * ============================================================================
 *
 * Evidence may support claims involving:
 *
 *     reasoning
 *     inference
 *     deduction
 *     knowledge
 *     learning
 *     adaptation
 *     uncertainty
 *     confidence
 *     decisions
 *     model behavior
 *     causal relationships.
 *
 * The grammar does not enumerate:
 *
 *     model families
 *     algorithms
 *     frameworks
 *     application categories
 *     vendor APIs.
 *
 * Those belong to semantic models, libraries, dialects, or backends.
 *
 * ============================================================================
 * DISTRIBUTED BOUNDARY
 * ============================================================================
 *
 * Evidence may concern:
 *
 *     distributed invariants
 *     consistency
 *     ordering
 *     replication
 *     communication
 *     fault tolerance
 *     protocol behavior
 *     execution observations.
 *
 * This grammar introduces no finite limit on:
 *
 *     nodes
 *     peers
 *     services
 *     messages
 *     channels
 *     replicas.
 *
 * ============================================================================
 * DATA / INTEROPERABILITY BOUNDARY
 * ============================================================================
 *
 * Evidence may originate from:
 *
 *     datasets
 *     schemas
 *     queries
 *     external formats
 *     imported artifacts
 *     generated artifacts.
 *
 * Data format syntax remains owned by the appropriate data/interoperability
 * subsystem.
 *
 * This grammar does not reproduce:
 *
 *     JSON
 *     XML
 *     SQL
 *     CSV
 *     foreign-language grammars.
 *
 * ============================================================================
 * SIMULATION BOUNDARY
 * ============================================================================
 *
 * Simulation may produce evidence.
 *
 * Examples include:
 *
 *     classical simulation result
 *     quantum simulation result
 *     HDL simulation result
 *     hardware model result
 *     distributed simulation result
 *     fault simulation result.
 *
 * Simulation remains an execution strategy.
 *
 * Evidence records the semantic relationship to the claim.
 *
 * This grammar does not define simulation syntax.
 *
 * ============================================================================
 * INTEROPERABILITY / FFI BOUNDARY
 * ============================================================================
 *
 * Evidence may originate from foreign or external tooling.
 *
 * Examples include:
 *
 *     foreign compiler result
 *     external verifier
 *     imported certificate
 *     external test result.
 *
 * FFI and ABI syntax remains owned by:
 *
 *     grammar/interoperability/
 *
 * This grammar does not define foreign-function syntax.
 *
 * Evidence originating from external tooling must still pass semantic policy,
 * provenance, compatibility, and trust checks downstream.
 *
 * ============================================================================
 * METAPROGRAMMING BOUNDARY
 * ============================================================================
 *
 * Generated source or generated verification artifacts may produce evidence.
 *
 * Metaprogramming remains owned by:
 *
 *     grammar/metaprogramming/
 *
 * This file does not introduce generation or reflection syntax.
 *
 * Generated evidence must preserve provenance sufficient to identify its
 * generating transformation.
 *
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * Structural diagnostics include malformed canonical subjects, such as:
 *
 *     property
 *     property()
 *     property(
 *     property(condition
 *     property(condition, extra)
 *     assert
 *     assert()
 *     requires
 *     ensures
 *
 * The exact diagnostic wording and recovery strategy belong to the frontend
 * diagnostic subsystem.
 *
 * Semantic/verification diagnostics may include:
 *
 *     unresolved subject
 *     invalid expression
 *     invalid predicate
 *     missing evidence
 *     insufficient evidence
 *     contradictory evidence
 *     unverifiable claim
 *     unavailable verification capability
 *     unavailable resource
 *     invalid evidence provenance
 *     untrusted evidence
 *     stale evidence
 *     policy violation
 *     unsupported verification method
 *     inconclusive verification.
 *
 * These are NOT parser errors when the source structure itself is valid.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * This file deliberately introduces no new source keyword.
 *
 * Existing canonical syntax remains owned by:
 *
 *     grammar/statements/assertions.g4
 *     grammar/statements/contract.g4
 *
 * Therefore compatibility behavior remains governed by the canonical grammar
 * and grammar/compatibility/.
 *
 * A future explicit evidence construct, if required, MUST follow:
 *
 *     specification
 *         |
 *         v
 *     lexer
 *         |
 *         v
 *     canonical grammar owner
 *         |
 *         v
 *     AST contract
 *         |
 *         v
 *     semantic evidence model
 *         |
 *         v
 *     validation facade
 *
 * Never:
 *
 *     validation facade
 *         |
 *         v
 *     private evidence syntax.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * There is no grammar-defined maximum for:
 *
 *     evidence-bearing subjects
 *     evidence count
 *     claims
 *     obligations
 *     properties
 *     assertions
 *     premises
 *     expression size
 *     source size
 *     modules
 *     functions
 *     quantum operations
 *     qubits
 *     processors
 *     accelerators
 *     devices
 *     nodes
 *     memory
 *     tensor rank
 *     network size.
 *
 * "Infinity" means that this grammar introduces no artificial finite machine
 * capacity ceiling.
 *
 * Actual limits arise from:
 *
 *     compiler resources
 *     verifier resources
 *     runtime resources
 *     target resources
 *     physical resources
 *     explicit policies
 *     explicit resource budgets.
 *
 * Such implementation constraints MUST NOT become hidden language constants.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Evidence-bearing source constructs remain target-independent.
 *
 * The same source must remain syntactically valid whether the enclosing
 * computation is intended for:
 *
 *     tiny systems
 *     embedded systems
 *     CPU
 *     multicore CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     QPU
 *     simulator
 *     HPC
 *     cluster
 *     distributed infrastructure
 *     cloud infrastructure
 *     future computational systems.
 *
 * A target may fail to provide sufficient resources or verification
 * capabilities.
 *
 * That is a semantic/resource/capability result.
 *
 * It is NOT a reason to change the source grammar.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     source text
 *     lexer vocabulary
 *     imported canonical grammars
 *     parser configuration.
 *
 * Parsing MUST NOT depend on:
 *
 *     wall-clock time
 *     randomness
 *     hardware availability
 *     filesystem state
 *     network state
 *     runtime state
 *     target selection
 *     scheduler state
 *     verifier availability.
 *
 * Repeated parsing of identical input under identical configuration must
 * produce equivalent parser structures and source spans.
 *
 * ============================================================================
 * SAFETY CONTRACT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no embedded Rust;
 *     no executable actions;
 *     no semantic predicates;
 *     no filesystem access;
 *     no network access;
 *     no hardware discovery;
 *     no resource allocation;
 *     no target selection;
 *     no runtime execution;
 *     no mutable global state.
 *
 * The consuming Zamani implementation must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and use safe Rust only.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE
 * --------
 *
 * Canonical property subject:
 *
 *     property(x > 0);
 *
 *     property(result.is_valid());
 *
 *     property(state.is_consistent());
 *
 * Canonical assertion subject:
 *
 *     assert(result.is_valid());
 *
 *     assert(measurement.is_valid());
 *
 * Canonical contract subjects:
 *
 *     requires(capability("verification"));
 *
 *     requires(memory >= required_memory);
 *
 *     ensures(result.is_valid());
 *
 *     invariant(state.is_consistent());
 *
 *     assume(input.is_valid());
 *
 *     guarantee(output.is_valid());
 *
 * Cross-domain:
 *
 *     property(capability("quantum.measurement"));
 *
 *     property(hardware_state.is_consistent());
 *
 *     property(model.is_valid());
 *
 *     assert(distributed_state.is_consistent());
 *
 *     ensures(measurement.is_valid());
 *
 * NEGATIVE
 * --------
 *
 * The validation entry point must reject incomplete canonical subjects:
 *
 *     property
 *     property()
 *     property(
 *     property(x
 *
 *     assert
 *     assert()
 *     assert(
 *
 *     requires
 *     ensures
 *     invariant
 *
 * It must also reject trailing source after an otherwise valid subject:
 *
 *     property(x > 0); trailing
 *
 *     assert(x); trailing
 *
 *     requires(x); trailing
 *
 * This is guaranteed by EOF at the validation boundary.
 *
 * The validation grammar must NOT accept invented evidence syntax such as:
 *
 *     evidence(x);
 *     evidence(...);
 *     certificate(x);
 *     witness(x);
 *
 * unless such syntax is first introduced through the canonical language
 * architecture.
 *
 * ============================================================================
 * BOUNDARY TESTS
 * ============================================================================
 *
 * Test subjects involving:
 *
 *     deeply nested expressions
 *     large symbolic expressions
 *     uncertainty
 *     probability
 *     confidence
 *     resource requirements
 *     capability requirements
 *     classical results
 *     quantum-derived results
 *     hybrid results
 *     HDL properties
 *     hardware state
 *     distributed state
 *     AI/model state
 *     simulation results
 *     foreign-tool results
 *     generated artifacts.
 *
 * The validation facade must remain unchanged as semantic domains grow.
 *
 * ============================================================================
 * SCALABILITY TESTS
 * ============================================================================
 *
 * Generate increasing:
 *
 *     subject count
 *     expression size
 *     expression nesting
 *     source-unit size
 *     module size
 *     semantic obligation count.
 *
 * The grammar must not introduce a machine-derived maximum.
 *
 * Implementation-level parser/verifier budgets may exist elsewhere, but they
 * must not become language grammar constants.
 *
 * ============================================================================
 * DETERMINISM TESTS
 * ============================================================================
 *
 * Parse identical input repeatedly with identical parser configuration.
 *
 * Verify equivalent:
 *
 *     parse-tree structure
 *     token sequence
 *     source spans
 *     selected validation alternative.
 *
 * ============================================================================
 * PORTABILITY TESTS
 * ============================================================================
 *
 * Parse identical source while varying external target descriptions,
 * resource inventories, processor counts, accelerator inventories, device
 * inventories, and hardware topologies.
 *
 * Parser output must remain determined by source and parser configuration.
 *
 * ============================================================================
 * CROSS-DOMAIN TESTS
 * ============================================================================
 *
 * Cover:
 *
 *     classical
 *     numerical
 *     scientific
 *     AI
 *     knowledge
 *     reasoning
 *     learning
 *     uncertainty
 *     quantum
 *     hybrid
 *     HDL
 *     hardware
 *     distributed
 *     networking
 *     data
 *     simulation
 *     interoperability
 *     accelerators
 *     embedded
 *     future domains.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file MUST contain:
 *
 *     no machine-capacity constants;
 *     no evidence-count constants;
 *     no proof-size constants;
 *     no solver-size constants;
 *     no target identifiers;
 *     no physical qubit identifiers;
 *     no fixed topology;
 *     no fixed processor count;
 *     no fixed memory capacity;
 *     no fixed tensor rank;
 *     no fixed network size;
 *     no backend-specific evidence syntax.
 *
 * In particular, this file must not contain language-level ceilings such as
 * any MAX_* capacity concept.
 *
 * Numeric values occurring inside canonical expressions are program values,
 * not implementation limits.
 *
 * ============================================================================
 * INTEGRATION MATRIX
 * ============================================================================
 *
 * LEXER
 * -----
 *
 * Uses:
 *
 *     tokenVocab = ZamaniLexer
 *
 * No lexer rules are defined here.
 *
 * CANONICAL ASSERTION SYNTAX
 * --------------------------
 *
 * Owned by:
 *
 *     grammar/statements/assertions.g4
 *
 * Specifically:
 *
 *     assertionStatement
 *
 * CANONICAL CONTRACT SYNTAX
 * -------------------------
 *
 * Owned by:
 *
 *     grammar/statements/contract.g4
 *
 * Specifically:
 *
 *     requiresStatement
 *     ensuresStatement
 *     invariantStatement
 *     assumeStatement
 *     guaranteeStatement
 *     propertyStatement
 *
 * AST
 * ---
 *
 * Existing domain-neutral frontend AST.
 *
 * SEMANTICS
 * ---------
 *
 * Semantic analysis determines:
 *
 *     claim
 *     obligation
 *     evidence requirement
 *     evidence relationship
 *     confidence
 *     verification status
 *     provenance
 *
 * VERIFICATION
 * ------------
 *
 * Verification infrastructure consumes semantic evidence relationships.
 *
 * PROVENANCE
 * ----------
 *
 * Provenance infrastructure records evidence lineage.
 *
 * RESOURCES
 * ---------
 *
 * Resource interpretation remains under:
 *
 *     grammar/resources/
 *
 * EFFECTS
 * -------
 *
 * Effect interpretation remains under:
 *
 *     grammar/effects/
 *
 * POLICIES
 * --------
 *
 * Policy interpretation remains downstream.
 *
 * PROOF
 * -----
 *
 * `grammar/validation/proofs.g4` consumes canonical property syntax for the
 * proof-obligation boundary.
 *
 * This file supplies the evidence boundary around such semantic obligations.
 *
 * IR
 * --
 *
 * No IR is created here.
 *
 * QUANTUM
 * -------
 *
 * Quantum semantics continue through:
 *
 *     quantum semantic model
 *         ->
 *     quantum::ir
 *
 * HDL / HARDWARE
 * --------------
 *
 * Hardware realization remains downstream.
 *
 * COMPILER
 * --------
 *
 * Evidence may participate in:
 *
 *     static validation
 *     type checking
 *     formal verification
 *     optimization validation
 *     resource validation
 *     specialization validation
 *     code-generation validation
 *     runtime validation.
 *
 * Each operation remains owned by its corresponding subsystem.
 *
 * ============================================================================
 * OWNERSHIP SUMMARY
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     ZamaniLexer
 *     AssertionsParser
 *     ContractStatements
 *     canonical expression grammar through imported owners
 *     semantic evidence model
 *     provenance infrastructure
 *     verification infrastructure
 *
 * EXPORTS:
 *
 *     evidenceValidationUnit
 *     evidenceValidationSubject
 *
 * CONSUMED_BY:
 *
 *     validation/conformance tooling
 *     grammar tests
 *     evidence-boundary test harness
 *     provenance-boundary test harness
 *     verification-boundary test harness
 *
 * AST_OWNER:
 *
 *     existing domain-neutral frontend AST
 *
 * SEMANTIC_OWNER:
 *
 *     semantic evidence/provenance subsystem
 *     semantic property/contract subsystem
 *     verification subsystem
 *
 * IR_OWNER:
 *
 *     canonical semantic model
 *     applicable domain IR
 *
 * TEST_OWNER:
 *
 *     grammar/tests/validation/evidence/
 *     validation conformance harness
 *
 * SPEC_OWNER:
 *
 *     grammar/specification/
 *     grammar/spec/
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 * [ ] It is a parser grammar.
 *
 * [ ] Its grammar name is `EvidenceValidation`.
 *
 * [ ] `tokenVocab = ZamaniLexer` is used.
 *
 * [ ] Canonical assertion syntax is imported from AssertionsParser.
 *
 * [ ] Canonical contract syntax is imported from ContractStatements.
 *
 * [ ] `evidenceValidationUnit` is the complete-input entry point.
 *
 * [ ] The entry point requires EOF.
 *
 * [ ] `evidenceValidationSubject` is the only evidence-specific structural
 *     selection rule.
 *
 * [ ] No evidence source syntax is invented.
 *
 * [ ] No `evidenceStatement` is invented.
 *
 * [ ] No `certificateStatement` is invented.
 *
 * [ ] No `witnessStatement` is invented.
 *
 * [ ] No `proofStatement` is invented.
 *
 * [ ] No theorem syntax is invented.
 *
 * [ ] No solver syntax is invented.
 *
 * [ ] No tactic syntax is invented.
 *
 * [ ] No lexer rule is defined.
 *
 * [ ] No expression grammar is duplicated.
 *
 * [ ] No assertion syntax is duplicated.
 *
 * [ ] No contract syntax is duplicated.
 *
 * [ ] No type grammar is duplicated.
 *
 * [ ] No semantic predicates are used.
 *
 * [ ] No embedded Rust is used.
 *
 * [ ] No executable actions are used.
 *
 * [ ] No resource allocation is performed.
 *
 * [ ] No hardware selection is performed.
 *
 * [ ] No target-specific syntax is introduced.
 *
 * [ ] No quantum operation catalogue is introduced.
 *
 * [ ] No second quantum IR is introduced.
 *
 * [ ] AST ownership remains downstream.
 *
 * [ ] Evidence semantics remain downstream.
 *
 * [ ] Provenance remains downstream.
 *
 * [ ] Verification remains downstream.
 *
 * [ ] Resource/capability analysis remains downstream.
 *
 * [ ] Effect analysis remains downstream.
 *
 * [ ] Policy analysis remains downstream.
 *
 * [ ] Positive tests exist.
 *
 * [ ] Negative tests exist.
 *
 * [ ] Boundary tests exist.
 *
 * [ ] Scalability tests exist.
 *
 * [ ] Determinism tests exist.
 *
 * [ ] Portability tests exist.
 *
 * [ ] Cross-domain tests exist.
 *
 * [ ] Compatibility tests exist.
 *
 * [ ] Rust 1.97 integration passes.
 *
 * [ ] Rust 1.97.1 integration passes.
 *
 * [ ] The consuming implementation remains safe Rust.
 *
 * ============================================================================
 * CRITICAL ARCHITECTURAL GUARANTEE
 * ============================================================================
 *
 * This file establishes exactly one boundary:
 *
 *     validation entry point
 *             |
 *             v
 *     canonical source construct
 *             |
 *             v
 *     domain-neutral AST
 *             |
 *             v
 *     semantic claim / obligation
 *             |
 *             v
 *          evidence
 *             |
 *             v
 *         provenance
 *             |
 *             v
 *       verification / decision
 *
 * It does NOT establish a second evidence language.
 *
 * This distinction is essential for POCO-REAF.
 *
 * ============================================================================
 */

parser grammar EvidenceValidation;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * ============================================================================
 * CANONICAL SOURCE OWNERS
 * ============================================================================
 *
 * Assertions and contracts have different canonical grammar owners.
 *
 * Assertions:
 *
 *     grammar/statements/assertions.g4
 *
 *     AssertionsParser
 *
 * Contracts/properties:
 *
 *     grammar/statements/contract.g4
 *
 *     ContractStatements
 *
 * Importing both preserves those ownership boundaries.
 */
import
    AssertionsParser,
    ContractStatements
    ;

/*
 * ============================================================================
 * COMPLETE EVIDENCE VALIDATION UNIT
 * ============================================================================
 *
 * This is a validation-only complete-input entry point.
 *
 * It accepts exactly one canonical source construct that may participate in
 * the semantic evidence pipeline.
 *
 * EOF is mandatory so a valid subject followed by unrelated source cannot be
 * silently accepted by this isolated validation parser.
 *
 * The production Zamani program parser continues to use the canonical root
 * grammar and statement composition architecture.
 */
evidenceValidationUnit
    : evidenceValidationSubject EOF
    ;

/*
 * ============================================================================
 * EVIDENCE VALIDATION SUBJECT
 * ============================================================================
 *
 * This rule does NOT define evidence syntax.
 *
 * It selects existing canonical source constructs that can become:
 *
 *     claims
 *     obligations
 *     properties
 *     assertions
 *
 * and consequently may acquire semantic evidence.
 *
 * Canonical owners remain authoritative.
 */
evidenceValidationSubject
    : assertionStatement
    | requiresStatement
    | ensuresStatement
    | invariantStatement
    | assumeStatement
    | guaranteeStatement
    | propertyStatement
    ;
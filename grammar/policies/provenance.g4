/*
 * ============================================================================
 * ZAMANI UNIVERSAL PROGRAMMING LANGUAGE
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/policies/provenance.g4
 *
 * GRAMMAR
 * -------
 * ANTLR4 parser grammar
 *
 * STATUS
 * ------
 * CANONICAL POLICY-PROVENANCE PAYLOAD GRAMMAR
 *
 * IMPLEMENTATION BASELINE
 * -----------------------
 * Rust 2021
 * Rust 1.97+
 * Safe Rust only
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * This file defines the structured payload used when a policy governs,
 * constrains, requires, records, or references provenance.
 *
 * IMPORTANT:
 *
 * This file does NOT own the outer:
 *
 *     PROVENANCE ...
 *
 * policy member.
 *
 * The outer policy-member syntax remains owned by:
 *
 *     grammar/policies/policy.g4
 *
 * That file may delegate its provenance payload to:
 *
 *     provenancePolicySpec
 *
 * This separation prevents two competing policy-provenance authorities.
 *
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 * The complete semantic path is:
 *
 *     source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       v
 *     grammar/policies/policy.g4
 *       |
 *       v
 *     provenancePolicySpec
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic provenance-policy model
 *       |
 *       +--> requirements
 *       +--> constraints
 *       +--> capabilities
 *       +--> resources
 *       +--> effects
 *       +--> contracts
 *       +--> security
 *       +--> execution
 *       +--> adaptation
 *       +--> simulation
 *       |
 *       v
 *     canonical semantic model
 *       |
 *       +--> classical semantics
 *       +--> quantum semantics
 *       +--> HDL/hardware semantics
 *       +--> AI semantics
 *       +--> distributed semantics
 *       +--> networking semantics
 *       +--> interoperability semantics
 *       |
 *       v
 *     canonical IR
 *       |
 *       +--> classical IR
 *       +--> quantum::ir
 *       +--> HDL/hardware representation
 *       +--> domain-specific IR where required
 *       |
 *       v
 *     target-independent optimization
 *       |
 *       v
 *     lowering / planning
 *       |
 *       v
 *     routing / scheduling / resilience
 *       |
 *       v
 *     target realization
 *
 *
 * ============================================================================
 * OWNERS
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     provenancePolicySpec
 *     provenancePolicyBody
 *     provenancePolicyMember
 *     provenancePolicyProperty
 *     provenancePolicyRelation
 *     provenancePolicyAssertion
 *     provenancePolicyReference
 *     provenancePolicyValue
 *     provenancePolicyKey
 *     provenancePolicyOperator
 *     provenancePolicyEntryList
 *
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     policyDeclaration
 *     policyBody
 *     policyMember
 *     policyProvenance
 *     policyRequirement
 *     policyConstraint
 *     policyCapability
 *     policyResource
 *
 * Those remain owned by:
 *
 *     grammar/policies/policy.g4
 *
 *
 * It also does NOT own:
 *
 *     identifiers
 *     qualified names
 *     expressions
 *     types
 *     contracts
 *     effects
 *     resources
 *     capabilities
 *     security authorization
 *     execution
 *     simulation
 *     deployment
 *     quantum operations
 *     hardware topology
 *     routing
 *     scheduling
 *     QEC
 *     ZQN
 *     HAL
 *     runtime enforcement
 *     IR definitions
 *
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * Provenance policy has two distinct layers:
 *
 *     policy.g4
 *         |
 *         +--> owns outer policy membership
 *
 *     provenance.g4
 *         |
 *         +--> owns structured provenance-policy payload
 *
 * Therefore:
 *
 *     policyProvenance
 *
 * MUST remain in policy.g4.
 *
 * This file MUST NOT define another rule named:
 *
 *     policyProvenance
 *
 * and MUST NOT define a competing:
 *
 *     PROVENANCE expression SEMICOLON
 *
 * rule.
 *
 *
 * ============================================================================
 * WHY THIS FILE EXISTS
 * ============================================================================
 *
 * Provenance is cross-cutting.
 *
 * A policy may need to express provenance requirements concerning:
 *
 *     source
 *     derivation
 *     transformation
 *     generation
 *     verification
 *     evidence
 *     decisions
 *     models
 *     data
 *     compilation
 *     optimization
 *     lowering
 *     execution
 *     quantum transformations
 *     hardware realization
 *     deployment
 *     adaptation
 *     simulation
 *     interoperability
 *
 * Those concepts must not become an ever-growing list of reserved keywords.
 *
 * Consequently this grammar uses:
 *
 *     qualified names
 *     expressions
 *     properties
 *     relations
 *     assertions
 *
 * to provide an open-world provenance vocabulary.
 *
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Provenance policy is source-portable.
 *
 * It MUST NOT encode:
 *
 *     physical processor identifiers
 *     physical memory identifiers
 *     physical device identifiers
 *     physical qubit identifiers
 *     vendor-specific handles
 *     scheduler state
 *     runtime pointers
 *     calibration objects
 *     placement decisions
 *
 * A provenance policy can therefore survive lowering from:
 *
 *     tiny embedded target
 *     CPU
 *     multicore CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     QPU
 *     simulator
 *     HPC system
 *     cluster
 *     distributed system
 *     cloud system
 *     heterogeneous system
 *     future computational substrate
 *
 * without changing its source-level meaning.
 *
 *
 * ============================================================================
 * OPEN-WORLD CONTRACT
 * ============================================================================
 *
 * No finite catalogue of provenance categories is encoded.
 *
 * For example, this grammar does NOT require separate universal syntax for:
 *
 *     compiler provenance
 *     AI provenance
 *     quantum provenance
 *     HDL provenance
 *     scientific provenance
 *     data provenance
 *     security provenance
 *     deployment provenance
 *
 * Instead, these may be represented through qualified names such as:
 *
 *     compiler::transformation
 *     ai::model
 *     quantum::decomposition
 *     hdl::synthesis
 *     data::source
 *     security::verification
 *     deployment::artifact
 *
 * without changing this grammar.
 *
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * The grammar imposes no language-level finite limit on:
 *
 *     provenance entries
 *     provenance properties
 *     provenance relations
 *     provenance assertions
 *     provenance references
 *     relation depth
 *     qualification depth
 *     policy nesting
 *     expression size
 *     identifier size
 *     artifact count
 *     transformation count
 *     evidence count
 *     source count
 *     derived-artifact count
 *
 * Repetition is represented with ANTLR repetition operators.
 *
 * Any actual limit encountered by:
 *
 *     parser memory
 *     compiler memory
 *     runtime storage
 *     operating system
 *     target hardware
 *     network
 *     filesystem
 *
 * is an implementation or environment constraint, not a language-defined
 * capacity ceiling.
 *
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This file MUST NOT introduce any equivalent of:
 *
 *     MAX_PROVENANCE
 *     MAX_PROVENANCE_ENTRIES
 *     MAX_EVIDENCE
 *     MAX_SOURCES
 *     MAX_ARTIFACTS
 *     MAX_TRANSFORMATIONS
 *     MAX_DECISIONS
 *     MAX_POLICY_DEPTH
 *     MAX_TARGETS
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_QUBITS
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *     MAX_NETWORK_SIZE
 *
 * No machine capacity is encoded here.
 *
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This is a PARSER grammar.
 *
 * It defines no lexer rules.
 *
 * Canonical lexical ownership remains:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * through:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * Existing lexical tokens such as:
 *
 *     PROVENANCE
 *     ASSIGN
 *     COLON
 *     COMMA
 *     SEMICOLON
 *     LBRACE
 *     RBRACE
 *
 * remain owned by the canonical lexer.
 *
 * This file deliberately does not introduce provenance-specific keyword
 * tokens such as:
 *
 *     SOURCE
 *     DERIVED_FROM
 *     GENERATED_BY
 *     VERIFIED_BY
 *     EVIDENCE
 *     DECISION
 *     TRANSFORMATION
 *
 * merely to represent future provenance concepts.
 *
 * Such concepts can be represented by open qualified names and expressions.
 *
 *
 * ============================================================================
 * GRAMMAR DEPENDENCIES
 * ============================================================================
 *
 * Direct dependencies:
 *
 *     grammar/core/qualified-names.g4
 *     grammar/expressions/...
 *
 * The canonical qualified-name grammar supplies:
 *
 *     qualifiedName
 *
 * The canonical expression grammar supplies:
 *
 *     expression
 *
 * This file does not redefine either.
 *
 *
 * ============================================================================
 * DEPENDENCY DIRECTION
 * ============================================================================
 *
 * Canonical direction:
 *
 *     lexer
 *       |
 *       v
 *     names / expressions
 *       |
 *       v
 *     policy
 *       |
 *       v
 *     provenance-policy payload
 *       |
 *       v
 *     semantic provenance policy
 *       |
 *       +--> compiler provenance
 *       +--> data provenance
 *       +--> AI provenance
 *       +--> quantum provenance
 *       +--> HDL provenance
 *       +--> security provenance
 *       +--> execution provenance
 *       +--> deployment provenance
 *
 * This file MUST NOT import a downstream semantic or runtime grammar.
 *
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Parser contexts from this grammar map to domain-neutral AST structures
 * equivalent to:
 *
 *     ProvenancePolicySpec
 *     ProvenancePolicyEntry
 *     ProvenancePolicyProperty
 *     ProvenancePolicyRelation
 *     ProvenancePolicyAssertion
 *     ProvenancePolicyReference
 *
 * The AST should preserve:
 *
 *     source span
 *     source order
 *     key
 *     operator
 *     value
 *     relation target
 *     expression structure
 *     qualification
 *
 * The AST MUST NOT resolve:
 *
 *     artifact identity
 *     compiler pass identity
 *     hardware identity
 *     physical qubit identity
 *     device identity
 *     runtime object identity
 *     evidence authenticity
 *     cryptographic signatures
 *     timestamps
 *     execution state
 *
 * Those are semantic/runtime concerns.
 *
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis interprets provenance policy entries.
 *
 * Possible semantic meanings include:
 *
 *     required provenance
 *     optional provenance
 *     prohibited provenance
 *     provenance relationship
 *     derivation requirement
 *     evidence requirement
 *     verification requirement
 *     reproducibility requirement
 *     audit requirement
 *     traceability requirement
 *     retention requirement
 *     disclosure requirement
 *     lineage requirement
 *     decision-record requirement
 *
 * The grammar does not assign those meanings.
 *
 * Meaning is determined by semantic policy resolution.
 *
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * Values are ordinary Zamani expressions.
 *
 * Therefore provenance policy can reference:
 *
 *     strings
 *     numbers
 *     booleans
 *     records
 *     collections
 *     symbolic names
 *     paths
 *     function results
 *     typed values
 *     generic values
 *     domain values
 *
 * The grammar does not impose a provenance-specific physical type system.
 *
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Parsing provenance policy has no runtime effect.
 *
 * Semantic use of provenance policy MAY require effects associated with:
 *
 *     observation
 *     IO
 *     storage
 *     network
 *     cryptography
 *     logging
 *     reflection
 *     execution tracing
 *
 * Those effects remain owned by the effect system.
 *
 * This grammar does not invent new effects.
 *
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Provenance policy may semantically require capabilities such as:
 *
 *     provenance::record
 *     provenance::verify
 *     provenance::trace
 *     audit::record
 *     cryptography::verify
 *     storage::persistent
 *
 * These are examples of open-world semantic capability names.
 *
 * This grammar does not determine whether any capability exists.
 *
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Provenance can consume resources, including:
 *
 *     storage
 *     memory
 *     bandwidth
 *     compute
 *     secure storage
 *     logging capacity
 *
 * Resource semantics remain outside this grammar.
 *
 * A provenance policy may reference resource expressions without encoding a
 * universal physical maximum.
 *
 *
 * ============================================================================
 * CONTRACT INTEGRATION
 * ============================================================================
 *
 * Provenance policies may participate in contracts.
 *
 * Examples of semantic relationships include:
 *
 *     a required provenance record must exist;
 *     an evidence relation must be verifiable;
 *     a transformation must remain traceable;
 *     a decision must have supporting evidence.
 *
 * Contract syntax itself remains owned by:
 *
 *     grammar/validation/
 *
 * This file does not redefine:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 *
 * ============================================================================
 * POLICY INTEGRATION
 * ============================================================================
 *
 * This file is subordinate to:
 *
 *     grammar/policies/policy.g4
 *
 * The intended integration is:
 *
 *     policyProvenance
 *         :
 *     PROVENANCE provenancePolicySpec SEMICOLON
 *         ;
 *
 * The outer rule remains in policy.g4.
 *
 * This file owns only:
 *
 *     provenancePolicySpec
 *
 * and its subordinate rules.
 *
 *
 * ============================================================================
 * PROVENANCE MODEL
 * ============================================================================
 *
 * The semantic provenance graph should be capable of representing arbitrary:
 *
 *     source
 *       |
 *       v
 *     transformation
 *       |
 *       v
 *     derived artifact
 *       |
 *       v
 *     verification
 *       |
 *       v
 *     decision
 *
 * relationships.
 *
 * The grammar does not impose a fixed graph shape.
 *
 * Provenance may therefore describe:
 *
 *     source -> compilation
 *     compilation -> optimization
 *     optimization -> lowering
 *     lowering -> quantum decomposition
 *     quantum decomposition -> routing
 *     routing -> scheduling
 *     scheduling -> hardware execution
 *
 * or:
 *
 *     dataset -> model
 *     model -> inference
 *     inference -> explanation
 *     explanation -> decision
 *
 * or:
 *
 *     HDL source -> synthesis
 *     synthesis -> implementation artifact
 *     implementation artifact -> verification
 *
 * or any future provenance graph.
 *
 *
 * ============================================================================
 * ENTRY MODEL
 * ============================================================================
 *
 * A provenance-policy body contains zero or more entries.
 *
 * Each entry has:
 *
 *     key
 *     operator
 *     value
 *
 * or:
 *
 *     relation key
 *     relation value
 *
 * The key is a qualified name.
 *
 * The value is a normal Zamani expression.
 *
 * This deliberately gives the semantic layer freedom to interpret a key.
 *
 *
 * ============================================================================
 * SYNTAX EXAMPLES
 * ============================================================================
 *
 * Example 1:
 *
 *     policy example {
 *         provenance {
 *             source = "dataset";
 *             verification = true;
 *         };
 *     }
 *
 * Example 2:
 *
 *     policy example {
 *         provenance {
 *             compiler::trace = true;
 *             quantum::transformation = required;
 *             security::verification = required;
 *         };
 *     }
 *
 * Example 3:
 *
 *     policy example {
 *         provenance {
 *             source : data::input;
 *             derived_from : compiler::artifact;
 *             verified_by : security::verification;
 *         };
 *     }
 *
 * The actual semantic interpretation of these keys belongs downstream.
 *
 * No key becomes a globally reserved keyword merely because it appears in a
 * provenance policy.
 *
 *
 * ============================================================================
 * 1. PROVENANCE POLICY SPECIFICATION
 * ============================================================================
 *
 * This is the public rule consumed by policy.g4.
 */

provenancePolicySpec
    : LBRACE
      provenancePolicyBody
      RBRACE
    ;


/*
 * ============================================================================
 * 2. PROVENANCE POLICY BODY
 * ============================================================================
 *
 * Unbounded sequence of entries.
 */

provenancePolicyBody
    : provenancePolicyMember*
    ;


/*
 * ============================================================================
 * 3. PROVENANCE POLICY MEMBER
 * ============================================================================
 */

provenancePolicyMember
    : provenancePolicyProperty
    | provenancePolicyRelation
    | provenancePolicyAssertion
    | provenancePolicyReference
    ;


/*
 * ============================================================================
 * 4. PROPERTY
 * ============================================================================
 *
 * Canonical form:
 *
 *     key = expression;
 *
 * The key is open-world.
 *
 * No fixed property catalogue is encoded.
 */

provenancePolicyProperty
    : provenancePolicyKey
      ASSIGN
      provenancePolicyValue
      SEMICOLON
    ;


/*
 * ============================================================================
 * 5. RELATION
 * ============================================================================
 *
 * Canonical form:
 *
 *     key : expression;
 *
 * This is intentionally generic.
 *
 * Semantic interpretation may establish relations such as:
 *
 *     source
 *     derived_from
 *     generated_by
 *     transformed_by
 *     verified_by
 *     supported_by
 *     depends_on
 *     explains
 *     records
 *
 * without requiring those names to become reserved tokens.
 */

provenancePolicyRelation
    : provenancePolicyKey
      COLON
      provenancePolicyValue
      SEMICOLON
    ;


/*
 * ============================================================================
 * 6. ASSERTION
 * ============================================================================
 *
 * An assertion is a provenance-policy condition.
 *
 * Canonical form:
 *
 *     expression;
 *
 * It is intentionally kept separate from properties and relations so the
 * semantic layer can distinguish:
 *
 *     value assignment
 *
 * from:
 *
 *     declarative condition.
 *
 * NOTE:
 *
 * The policy layer may choose to restrict which expressions are legal in this
 * position during semantic validation.
 */

provenancePolicyAssertion
    : provenancePolicyValue
      SEMICOLON
    ;


/*
 * ============================================================================
 * 7. SYMBOLIC PROVENANCE REFERENCE
 * ============================================================================
 *
 * A standalone qualified name may identify a provenance model, provenance
 * source, provenance profile, provenance schema, or another semantic object.
 *
 * Canonical form:
 *
 *     provenance::profile;
 *
 * This does not resolve the name.
 */

provenancePolicyReference
    : qualifiedName
      SEMICOLON
    ;


/*
 * ============================================================================
 * 8. PROVENANCE KEY
 * ============================================================================
 *
 * Qualified names are the extensibility boundary.
 */

provenancePolicyKey
    : qualifiedName
    ;


/*
 * ============================================================================
 * 9. PROVENANCE VALUE
 * ============================================================================
 *
 * Values are normal Zamani expressions.
 *
 * This prevents the provenance subsystem from creating a second expression
 * language.
 */

provenancePolicyValue
    : expression
    ;


/*
 * ============================================================================
 * 10. LIST WRAPPERS
 * ============================================================================
 *
 * These wrappers are useful to downstream grammar composition and testing.
 *
 * No fixed cardinality is imposed.
 */

provenancePolicyEntryList
    : provenancePolicyMember*
    ;


provenancePolicyKeyList
    : provenancePolicyKey
      (COMMA provenancePolicyKey)*
    ;


provenancePolicyValueList
    : provenancePolicyValue
      (COMMA provenancePolicyValue)*
    ;


/*
 * ============================================================================
 * AST INTEGRATION CONTRACT
 * ============================================================================
 *
 * The parser contexts should map to:
 *
 *     ProvenancePolicySpec
 *         |
 *         +--> entries
 *                |
 *                +--> property
 *                +--> relation
 *                +--> assertion
 *                +--> reference
 *
 * Every node must retain source-location information.
 *
 * Source order must be preserved.
 *
 * Qualified-name spelling must be preserved until semantic resolution has
 * completed.
 *
 * No physical target information may be inserted into these AST nodes.
 *
 *
 * ============================================================================
 * SEMANTIC INTEGRATION CONTRACT
 * ============================================================================
 *
 * The semantic pipeline is:
 *
 *     provenancePolicySpec
 *       |
 *       v
 *     structural validation
 *       |
 *       v
 *     provenance-policy semantic model
 *       |
 *       +--> name resolution
 *       +--> expression typing
 *       +--> contract analysis
 *       +--> capability analysis
 *       +--> resource analysis
 *       +--> effect analysis
 *       +--> policy conflict analysis
 *       +--> provenance graph validation
 *       |
 *       v
 *     canonical semantic representation
 *
 *
 * ============================================================================
 * RESOURCE/CAPABILITY NEGOTIATION
 * ============================================================================
 *
 * Provenance requirements must never imply a universal physical resource
 * quantity.
 *
 * For example, semantic analysis may determine:
 *
 *     capability("provenance.verify")
 *
 * or:
 *
 *     resource::persistent_storage
 *
 * but this grammar does not determine:
 *
 *     how much storage;
 *     how many processors;
 *     how many nodes;
 *     how many devices;
 *     how many qubits.
 *
 * Those are resolved by the resource/capability subsystem.
 *
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Provenance may describe quantum transformations such as:
 *
 *     circuit construction
 *     optimization
 *     decomposition
 *     routing
 *     scheduling
 *     resilience transformation
 *     measurement
 *     execution
 *
 * The semantic path remains:
 *
 *     policy provenance
 *       |
 *       v
 *     semantic provenance model
 *       |
 *       v
 *     quantum semantic model
 *       |
 *       v
 *     quantum::ir
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     decomposition
 *       |
 *       v
 *     routing
 *       |
 *       v
 *     scheduling
 *       |
 *       v
 *     resilience / QEC
 *       |
 *       v
 *     ZQN
 *       |
 *       v
 *     HAL
 *
 * This file MUST NOT contain:
 *
 *     gate definitions
 *     physical qubits
 *     coupling maps
 *     routing rules
 *     calibration
 *     pulse definitions
 *     QEC implementation
 *
 *
 * ============================================================================
 * HDL/HARDWARE INTEGRATION
 * ============================================================================
 *
 * Provenance policy can describe semantic transformations such as:
 *
 *     HDL source
 *       ->
 *     synthesis
 *       ->
 *     implementation artifact
 *       ->
 *     verification
 *       ->
 *     deployment
 *
 * It must not encode:
 *
 *     fixed wire widths
 *     fixed register widths
 *     fixed FPGA resources
 *     fixed ASIC cells
 *     fixed device counts
 *     physical placement
 *
 *
 * ============================================================================
 * AI / LEARNING INTEGRATION
 * ============================================================================
 *
 * Provenance policy may apply to:
 *
 *     datasets
 *     learned models
 *     training runs
 *     adaptation
 *     inference
 *     explanations
 *     evidence
 *     decisions
 *
 * This allows learned computation to participate in the same provenance
 * architecture as ordinary compilation and execution.
 *
 * The grammar does not require AI-specific provenance keywords.
 *
 *
 * ============================================================================
 * DATA INTEGRATION
 * ============================================================================
 *
 * Provenance policy can reference:
 *
 *     data::source
 *     data::schema
 *     data::transformation
 *     data::lineage
 *
 * without making those concepts reserved language keywords.
 *
 * SQL, JSON, XML, graph and other data formats remain dialect/interoperability
 * concerns.
 *
 *
 * ============================================================================
 * INTEROPERABILITY INTEGRATION
 * ============================================================================
 *
 * Provenance may describe:
 *
 *     foreign declarations
 *     ABI boundaries
 *     external artifacts
 *     imported data
 *     generated bindings
 *     external transformations
 *
 * FFI/ABI semantics remain owned by:
 *
 *     grammar/interoperability/
 *
 *
 * ============================================================================
 * METAPROGRAMMING INTEGRATION
 * ============================================================================
 *
 * Provenance may record:
 *
 *     generated source
 *     generated syntax
 *     compile-time transformation
 *     reflection result
 *     generated artifact
 *
 * Metaprogramming semantics remain owned by:
 *
 *     grammar/metaprogramming/
 *
 *
 * ============================================================================
 * SECURITY INTEGRATION
 * ============================================================================
 *
 * Provenance policy may participate in:
 *
 *     audit
 *     trust
 *     verification
 *     authorization evidence
 *     integrity records
 *
 * Security semantics remain owned by:
 *
 *     grammar/security/
 *
 * This file does not implement cryptography or authorization.
 *
 *
 * ============================================================================
 * EXECUTION INTEGRATION
 * ============================================================================
 *
 * Execution policy consumes provenance semantics for:
 *
 *     reproducibility
 *     auditability
 *     traceability
 *     decision records
 *     transformation records
 *     execution lineage
 *
 * The execution subsystem remains responsible for runtime behavior.
 *
 *
 * ============================================================================
 * SIMULATION INTEGRATION
 * ============================================================================
 *
 * Provenance may be recorded for:
 *
 *     simulation inputs
 *     simulation configuration
 *     generated models
 *     simulation transformations
 *     simulation results
 *     validation
 *
 * The simulation grammar does not become a provenance authority.
 *
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing must depend only on:
 *
 *     source tokens
 *     grammar version
 *     parser configuration
 *     imported grammar definitions
 *
 * It must not depend on:
 *
 *     wall-clock time
 *     randomness
 *     hardware
 *     target availability
 *     resource availability
 *     network state
 *     runtime state
 *     filesystem state
 *
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * This grammar introduces no replacement for existing policy syntax.
 *
 * Existing source:
 *
 *     policy ... {
 *         provenance ...;
 *     }
 *
 * remains owned by policy.g4.
 *
 * Integration changes should therefore be limited to delegating the payload
 * from policy.g4 to:
 *
 *     provenancePolicySpec
 *
 * Existing generic policy expressions remain valid outside the structured
 * provenance form.
 *
 *
 * ============================================================================
 * ERROR / DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Structural diagnostics should identify:
 *
 *     missing opening brace
 *     missing closing brace
 *     missing assignment value
 *     missing relation value
 *     missing semicolon
 *     malformed qualified name
 *     malformed expression
 *
 * Semantic diagnostics belong downstream.
 *
 * Examples:
 *
 *     unknown provenance key
 *     incompatible provenance value
 *     unresolved provenance reference
 *     unavailable verification capability
 *     insufficient runtime provenance capability
 *     invalid provenance relation
 *
 * must NOT be converted into parser errors merely because a target lacks a
 * capability.
 *
 *
 * ============================================================================
 * POSITIVE TEST CONTRACT
 * ============================================================================
 *
 * The test suite must accept:
 *
 *     provenancePolicySpec
 *     {
 *         source = data::input;
 *     }
 *
 *     provenancePolicySpec
 *     {
 *         compiler::transformation = true;
 *         quantum::routing = true;
 *     }
 *
 *     provenancePolicySpec
 *     {
 *         source : data::input;
 *         derived_from : compiler::artifact;
 *     }
 *
 *     provenancePolicySpec
 *     {
 *         security::verification = required;
 *         audit::record = true;
 *         reproducibility::required = true;
 *     }
 *
 *     provenancePolicySpec
 *     {
 *         provenance::profile;
 *     }
 *
 *     provenancePolicySpec
 *     {
 *         a = b;
 *         c : d;
 *         e;
 *     }
 *
 *
 * ============================================================================
 * NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * The suite must reject structurally malformed forms such as:
 *
 *     {
 *         source =
 *     }
 *
 *     {
 *         source :
 *     }
 *
 *     {
 *         = value;
 *     }
 *
 *     {
 *         : value;
 *     }
 *
 *     {
 *         source = value
 *     }
 *
 *     {
 *         :: value;
 *     }
 *
 * malformed names and expressions must be diagnosed by their canonical
 * imported grammars.
 *
 *
 * ============================================================================
 * BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Test:
 *
 *     empty provenance policy
 *     one entry
 *     many entries
 *     deeply qualified keys
 *     long identifiers
 *     Unicode identifiers accepted by the lexer
 *     nested expressions
 *     large expression values
 *     mixed properties and relations
 *     symbolic references
 *     cross-domain names
 *     quantum provenance
 *     HDL provenance
 *     AI provenance
 *     compiler provenance
 *     security provenance
 *     data provenance
 *     execution provenance
 *     deployment provenance
 *
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Progressively scale:
 *
 *     number of entries
 *     key length
 *     qualification depth
 *     expression size
 *     policy source size
 *
 * No test may assume a language-defined maximum.
 *
 * The objective is to establish:
 *
 *     parser correctness at small scale
 *     parser correctness at large scale
 *     stable AST structure
 *     deterministic parsing
 *     absence of artificial capacity ceilings
 *
 *
 * ============================================================================
 * CROSS-DOMAIN TEST CONTRACT
 * ============================================================================
 *
 * This grammar must be consumable by policy semantics associated with:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     hardware
 *     AI
 *     data
 *     distributed
 *     networking
 *     security
 *     execution
 *     simulation
 *     deployment
 *     interoperability
 *     metaprogramming
 *     future domains
 *
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar emits NO IR.
 *
 * The downstream path is:
 *
 *     parser context
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic provenance-policy model
 *       |
 *       +------------------------+
 *       |                        |
 *       v                        v
 *     classical semantics   quantum semantics
 *                                |
 *                                v
 *                            quantum::ir
 *
 * Provenance must remain orthogonal to backend-specific representation.
 *
 *
 * ============================================================================
 * TEST OWNER
 * ============================================================================
 *
 * Recommended test ownership:
 *
 *     grammar/tests/policies/provenance/
 *
 * with:
 *
 *     positive/
 *     negative/
 *     boundary/
 *     scalability/
 *     cross-domain/
 *     determinism/
 *     compatibility/
 *
 *
 * ============================================================================
 * SPECIFICATION OWNER
 * ============================================================================
 *
 * Normative semantic documentation should live under:
 *
 *     grammar/spec/policies.md
 *
 * and, where provenance is sufficiently broad, a dedicated:
 *
 *     grammar/spec/provenance.md
 *
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/core/qualified-names.g4
 *     grammar/expressions/...
 *
 * EXPORTS:
 *
 *     provenancePolicySpec
 *     provenancePolicyBody
 *     provenancePolicyMember
 *     provenancePolicyProperty
 *     provenancePolicyRelation
 *     provenancePolicyAssertion
 *     provenancePolicyReference
 *     provenancePolicyValue
 *     provenancePolicyKey
 *
 * CONSUMED_BY:
 *
 *     grammar/policies/policy.g4
 *     policy semantic analysis
 *     policy AST construction
 *     provenance semantic analysis
 *
 * AST_OWNER:
 *
 *     domain-neutral frontend AST
 *
 * SEMANTIC_OWNER:
 *
 *     policy/provenance semantic subsystem
 *
 * IR_OWNER:
 *
 *     canonical semantic model
 *     domain IR consumers
 *     quantum::ir where quantum semantics are affected
 *
 * TEST_OWNER:
 *
 *     grammar/tests/policies/provenance/
 *
 * SPEC_OWNER:
 *
 *     grammar/spec/policies.md
 *     grammar/spec/provenance.md
 *
 *
 * ============================================================================
 * REQUIRED INTEGRATION CHANGE IN policy.g4
 * ============================================================================
 *
 * The existing policy.g4 currently owns:
 *
 *     policyProvenance
 *
 * It should remain the owner of that outer rule while delegating its payload
 * here.
 *
 * Conceptually change:
 *
 *     policyProvenance
 *         : PROVENANCE policyExpression SEMICOLON
 *         ;
 *
 * to:
 *
 *     policyProvenance
 *         : PROVENANCE provenancePolicySpec SEMICOLON
 *         ;
 *
 * and import this grammar:
 *
 *     ProvenancePolicy
 *
 * alongside the other policy grammar dependencies.
 *
 * IMPORTANT:
 *
 * The exact ANTLR import spelling must follow the repository's generated
 * grammar naming convention. The source file remains:
 *
 *     grammar/policies/provenance.g4
 *
 * and its parser grammar name is:
 *
 *     ProvenancePolicy
 *
 * This integration does not create a second policy authority.
 *
 *
 * ============================================================================
 * CIRCULAR-DEPENDENCY PROHIBITION
 * ============================================================================
 *
 * This grammar MUST NOT import:
 *
 *     Policy
 *
 * because:
 *
 *     Policy -> ProvenancePolicy
 *
 * is the intended direction.
 *
 * The reverse:
 *
 *     ProvenancePolicy -> Policy
 *
 * would create a circular grammar dependency.
 *
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * The .g4 file contains no Rust actions.
 *
 * Generated Rust integration must:
 *
 *     support Rust 1.97+
 *     use Rust 2021
 *     use safe Rust
 *     avoid unsafe blocks
 *     avoid raw backend handles in AST construction
 *     avoid target-specific parser behavior
 *
 * The grammar itself performs no:
 *
 *     filesystem access
 *     network access
 *     device access
 *     FFI
 *     runtime callbacks
 *     hardware discovery
 *     scheduling
 *
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     [x] It is a parser grammar.
 *
 *     [x] It uses tokenVocab=ZamaniLexer.
 *
 *     [x] It does not define lexer tokens.
 *
 *     [x] It does not redefine policyProvenance.
 *
 *     [x] It owns the structured provenance-policy payload.
 *
 *     [x] It uses canonical qualified names.
 *
 *     [x] It uses canonical expressions.
 *
 *     [x] It is open-world.
 *
 *     [x] It has no physical-resource limits.
 *
 *     [x] It has no machine-capacity constants.
 *
 *     [x] It has no domain-specific hardware catalogue.
 *
 *     [x] It has no quantum gate catalogue.
 *
 *     [x] It has no AI algorithm catalogue.
 *
 *     [x] It has no fixed provenance vocabulary ceiling.
 *
 *     [x] It does not perform semantic resolution.
 *
 *     [x] It does not generate IR.
 *
 *     [x] It preserves the quantum::ir boundary.
 *
 *     [x] It preserves HDL/hardware boundaries.
 *
 *     [x] It integrates with the universal policy owner.
 *
 *     [x] It avoids circular grammar imports.
 *
 *     [x] It has positive tests defined.
 *
 *     [x] It has negative tests defined.
 *
 *     [x] It has boundary tests defined.
 *
 *     [x] It has scalability tests defined.
 *
 *     [x] It has cross-domain tests defined.
 *
 *     [x] It has determinism requirements defined.
 *
 *     [x] It has compatibility requirements defined.
 *
 *
 * ============================================================================
 * FINAL ARCHITECTURAL RULE
 * ============================================================================
 *
 * Provenance is a UNIVERSAL SEMANTIC CONCERN.
 *
 * It is not an AI-only feature.
 *
 * It is not a compiler-only feature.
 *
 * It is not a quantum-only feature.
 *
 * It is not a security-only feature.
 *
 * It is not a runtime-only feature.
 *
 * The source language expresses provenance policy intent.
 *
 * Semantic analysis determines meaning.
 *
 * Domain systems consume the resulting semantic model.
 *
 * Backend systems realize that meaning.
 *
 * Therefore the architecture remains:
 *
 *     ONE LANGUAGE
 *          |
 *          v
 *     ONE POLICY AUTHORITY
 *          |
 *          v
 *     ONE PROVENANCE-POLICY PAYLOAD GRAMMAR
 *          |
 *          v
 *     ONE DOMAIN-NEUTRAL AST
 *          |
 *          v
 *     ONE SEMANTIC PROVENANCE MODEL
 *          |
 *          +-------------------+
 *          |                   |
 *          v                   v
 *     classical           quantum::ir
 *          |                   |
 *          +---------+---------+
 *                    |
 *                    v
 *             target-independent
 *                 lowering
 *                    |
 *                    v
 *              target realization
 *
 * This preserves POCO-REAF while allowing provenance requirements to survive
 * compilation, optimization, lowering, simulation, adaptation, deployment,
 * and execution across arbitrary computational scales.
 *
 * ============================================================================
 */
 
parser grammar ProvenancePolicy;

options {
    tokenVocab = ZamaniLexer;
}

import
    QualifiedNames,
    Expressions
;


/*
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 */

provenancePolicySpec
    : LBRACE
      provenancePolicyBody
      RBRACE
    ;


/*
 * ============================================================================
 * POLICY BODY
 * ============================================================================
 */

provenancePolicyBody
    : provenancePolicyMember*
    ;


/*
 * ============================================================================
 * POLICY MEMBER
 * ============================================================================
 */

provenancePolicyMember
    : provenancePolicyProperty
    | provenancePolicyRelation
    | provenancePolicyAssertion
    | provenancePolicyReference
    ;


/*
 * ============================================================================
 * PROPERTY
 * ============================================================================
 */

provenancePolicyProperty
    : provenancePolicyKey
      ASSIGN
      provenancePolicyValue
      SEMICOLON
    ;


/*
 * ============================================================================
 * RELATION
 * ============================================================================
 */

provenancePolicyRelation
    : provenancePolicyKey
      COLON
      provenancePolicyValue
      SEMICOLON
    ;


/*
 * ============================================================================
 * ASSERTION
 * ============================================================================
 */

provenancePolicyAssertion
    : provenancePolicyValue
      SEMICOLON
    ;


/*
 * ============================================================================
 * SYMBOLIC REFERENCE
 * ============================================================================
 */

provenancePolicyReference
    : qualifiedName
      SEMICOLON
    ;


/*
 * ============================================================================
 * KEY
 * ============================================================================
 */

provenancePolicyKey
    : qualifiedName
    ;


/*
 * ============================================================================
 * VALUE
 * ============================================================================
 */

provenancePolicyValue
    : expression
    ;


/*
 * ============================================================================
 * LIST WRAPPERS
 * ============================================================================
 */

provenancePolicyEntryList
    : provenancePolicyMember*
    ;


provenancePolicyKeyList
    : provenancePolicyKey
      (COMMA provenancePolicyKey)*
    ;


provenancePolicyValueList
    : provenancePolicyValue
      (COMMA provenancePolicyValue)*
    ;
/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/security/zero-knowledge.g4
 *
 * ROLE
 * ----
 * Canonical parser grammar for zero-knowledge / verifiable-computation
 * source-level intent.
 *
 * STATUS
 * ------
 * PRODUCTION TARGET
 *
 * IMPLEMENTATION BASELINE
 * -----------------------
 * Rust 1.97 / Rust 1.97.1
 * Rust 2021
 * Safe Rust only.
 * No unsafe Rust is required by this grammar or its contract.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar defines portable source-level zero-knowledge intent.
 *
 * It describes:
 *
 *   - zero-knowledge protocol declarations;
 *   - proof-system declarations;
 *   - statements;
 *   - witnesses;
 *   - public inputs;
 *   - private inputs;
 *   - commitments;
 *   - claims;
 *   - proof generation intent;
 *   - verification intent;
 *   - proof references;
 *   - verifier references;
 *   - prover references;
 *   - soundness requirements;
 *   - completeness requirements;
 *   - zero-knowledge requirements;
 *   - transparency / trust assumptions;
 *   - recursion intent;
 *   - aggregation intent;
 *   - composition;
 *   - leakage constraints;
 *   - resource requirements;
 *   - capabilities;
 *   - constraints;
 *   - preferences;
 *   - implementation metadata.
 *
 * This grammar DOES NOT implement a proof system.
 *
 * It does not:
 *
 *   - generate proofs;
 *   - verify proofs;
 *   - execute cryptography;
 *   - implement commitments;
 *   - implement polynomial commitments;
 *   - implement interactive protocols;
 *   - implement non-interactive protocols;
 *   - select a proving backend;
 *   - select a verifier backend;
 *   - select a CPU;
 *   - select a GPU;
 *   - select an FPGA;
 *   - select an ASIC;
 *   - select a QPU;
 *   - allocate physical memory;
 *   - perform scheduling;
 *   - perform routing;
 *   - perform QEC;
 *   - perform ZQN;
 *   - create a second IR.
 *
 * ============================================================================
 * AUTHORITY
 * ============================================================================
 *
 * Language specification:
 *
 *     grammar/specification/
 *     grammar/spec/security.md
 *
 * Canonical parser composition:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * Canonical lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Canonical lexical composition:
 *
 *     grammar/lexer/tokens.g4
 *
 * Security composition:
 *
 *     grammar/security/security.g4
 *
 * Cryptographic primitives:
 *
 *     grammar/security/cryptography.g4
 *
 * Hashing:
 *
 *     grammar/security/hashes.g4
 *
 * Key lifecycle:
 *
 *     grammar/security/key-management.g4
 *
 * Capabilities:
 *
 *     grammar/security/capabilities.g4
 *
 * Policies:
 *
 *     grammar/security/policies.g4
 *
 * Trust:
 *
 *     grammar/security/trust.g4
 *
 * Frontend AST:
 *
 *     src/frontend/ast/
 *
 * Canonical quantum semantic boundary:
 *
 *     quantum::ir
 *
 * This grammar must never become a competing root grammar or a competing
 * security IR.
 *
 * ============================================================================
 * OPEN-WORLD PRINCIPLE
 * ============================================================================
 *
 * Zero-knowledge systems are NOT enumerated here.
 *
 * Do not create alternatives such as:
 *
 *     Groth16
 *     PLONK
 *     Halo2
 *     Nova
 *     STARK
 *     FRI
 *     Bulletproofs
 *     future_protocol
 *
 * as grammar keywords.
 *
 * They are semantic names.
 *
 * Examples:
 *
 *     zk::protocol::groth16
 *     zk::protocol::plonk
 *     zk::protocol::stark
 *     vendor::zk::prover
 *     future::zk::protocol
 *
 * The semantic/compiler layers determine whether a referenced mechanism
 * exists and whether it satisfies the declared requirements.
 *
 * ============================================================================
 * SCALABILITY / POCO-REAF
 * ============================================================================
 *
 * Zero-knowledge source programs must remain independent of target size.
 *
 * This grammar therefore imposes NO universal limits on:
 *
 *   - witness elements;
 *   - public inputs;
 *   - private inputs;
 *   - constraints;
 *   - commitments;
 *   - statements;
 *   - proof objects;
 *   - recursive layers;
 *   - aggregation groups;
 *   - parties;
 *   - sessions;
 *   - circuits;
 *   - circuit width;
 *   - circuit depth;
 *   - tensor dimensions;
 *   - field elements;
 *   - proof instances;
 *   - verifier instances;
 *   - CPUs;
 *   - GPUs;
 *   - FPGAs;
 *   - QPUs;
 *   - nodes;
 *   - memory;
 *   - storage;
 *   - devices.
 *
 * It MUST NOT define:
 *
 *     MAX_PROOFS
 *     MAX_WITNESSES
 *     MAX_PUBLIC_INPUTS
 *     MAX_PRIVATE_INPUTS
 *     MAX_CONSTRAINTS
 *     MAX_CIRCUIT_SIZE
 *     MAX_RECURSION_DEPTH
 *     MAX_AGGREGATION_SIZE
 *     MAX_PROVERS
 *     MAX_VERIFIERS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_GPUS
 *     MAX_QUBITS
 *
 * "Infinity" means:
 *
 *     the language does not impose an artificial semantic ceiling.
 *
 * Actual execution remains constrained by:
 *
 *     available resources
 *     implementation capabilities
 *     declared requirements
 *     target characteristics
 *     physical reality
 *
 * ============================================================================
 * REQUIREMENT / CAPABILITY / CONSTRAINT / PREFERENCE
 * ============================================================================
 *
 * These categories are deliberately distinct.
 *
 * REQUIREMENT
 *     Mandatory semantic condition.
 *
 * CAPABILITY
 *     Capability required from a realization.
 *
 * CONSTRAINT
 *     Restriction on legal realizations.
 *
 * PREFERENCE
 *     Desired but non-mandatory implementation property.
 *
 * HINT
 *     Guidance to compilation/lowering without changing program meaning.
 *
 * Example semantic intent:
 *
 *     requires capability("zk.prove")
 *     requires capability("zk.verify")
 *     requires proof_property("zero_knowledge")
 *     constraint leakage <= budget
 *     prefer backend("zk")
 *     hint parallel_proving
 *
 * These are not target-selection commands.
 *
 * ============================================================================
 * SECRET-MATERIAL BOUNDARY
 * ============================================================================
 *
 * A zero-knowledge witness is NOT automatically a source-level secret
 * literal.
 *
 * Source code should normally refer to a semantic value:
 *
 *     witness::identity
 *     secret::credential
 *     data::private_input
 *
 * rather than embedding permanent secret material in source.
 *
 * Secret storage remains owned by:
 *
 *     grammar/security/secrets.g4
 *
 * Key lifecycle remains owned by:
 *
 *     grammar/security/key-management.g4
 *
 * Cryptographic primitive selection remains owned by:
 *
 *     grammar/security/cryptography.g4
 *
 * This grammar may reference those semantic objects but must not duplicate
 * their declarations.
 *
 * ============================================================================
 * CRYPTOGRAPHY INTEGRATION
 * ============================================================================
 *
 * Zero-knowledge proofs frequently depend on cryptographic primitives.
 *
 * This grammar therefore permits semantic references to:
 *
 *     cryptography::...
 *
 * but does not define:
 *
 *     hash algorithms;
 *     signature algorithms;
 *     commitment algorithms;
 *     encryption algorithms;
 *     key types;
 *     randomness implementations.
 *
 * Those belong to the existing cryptography/security grammars.
 *
 * ============================================================================
 * HASH INTEGRATION
 * ============================================================================
 *
 * Hash-specific syntax remains owned by:
 *
 *     grammar/security/hashes.g4
 *
 * Zero-knowledge syntax may reference a hash semantic object by qualified
 * name or expression.
 *
 * This prevents duplicate hash grammar.
 *
 * ============================================================================
 * TRUST INTEGRATION
 * ============================================================================
 *
 * Trust assumptions may be expressed as semantic references.
 *
 * Trust relationships themselves remain owned by:
 *
 *     grammar/security/trust.g4
 *
 * The zero-knowledge grammar does not determine whether a trusted setup,
 * verifier, prover, registry, ceremony, or authority is actually trusted.
 *
 * ============================================================================
 * POLICY INTEGRATION
 * ============================================================================
 *
 * Security policies remain owned by:
 *
 *     grammar/security/policies.g4
 *
 * A ZK declaration may reference a policy but must not duplicate policy
 * declaration syntax.
 *
 * ============================================================================
 * CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Generic capability syntax remains owned by the repository's canonical
 * capability grammar.
 *
 * This grammar may carry:
 *
 *     capability("zk.prove")
 *     capability("zk.verify")
 *     capability("zk.recursive")
 *     capability("zk.aggregate")
 *
 * as expressions/semantic references.
 *
 * It must not create a second capability model.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Zero-knowledge computation may protect or verify quantum computations.
 *
 * However this grammar MUST NOT define:
 *
 *     QubitId
 *     PhysicalQubitId
 *     GateKind
 *     QuantumTopology
 *     QEC
 *     ZQN
 *     Calibration
 *     Routing
 *     Scheduling
 *
 * Quantum semantics remain under:
 *
 *     quantum::ir
 *
 * Security metadata may accompany quantum semantic operations after parsing
 * and semantic analysis.
 *
 * ============================================================================
 * CLASSICAL / HDL / DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * The same ZK source model can describe requirements around:
 *
 *     classical computation;
 *     quantum computation;
 *     hybrid computation;
 *     HDL/hardware intent;
 *     distributed computation;
 *     AI computation;
 *     networking;
 *     data processing;
 *     embedded computation;
 *     future computational domains.
 *
 * The ZK grammar remains domain-neutral.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Every production in this file maps to the domain-neutral frontend AST.
 *
 * The grammar MUST NOT create:
 *
 *     Groth16Node
 *     PlonkNode
 *     StarkNode
 *     PhysicalProverNode
 *     GPUProverNode
 *     QPUVerifierNode
 *
 * Instead, the AST should represent generic semantic structures such as:
 *
 *     declaration
 *     operation
 *     reference
 *     property
 *     requirement
 *     constraint
 *     preference
 *     metadata
 *
 * The semantic layer resolves their ZK meaning.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates NO IR.
 *
 * Zero-knowledge semantic information must lower into the repository's
 * canonical semantic/IR architecture.
 *
 * If a proof operation concerns quantum computation:
 *
 *     source
 *       ↓
 *     generic AST
 *       ↓
 *     security semantic analysis
 *       ↓
 *     quantum semantic analysis
 *       ↓
 *     quantum::ir + security metadata
 *
 * If it concerns classical computation:
 *
 *     source
 *       ↓
 *     generic AST
 *       ↓
 *     security semantic analysis
 *       ↓
 *     classical semantic/IR representation
 *
 * There is no:
 *
 *     ZeroKnowledgeIR
 *
 * unless the repository later establishes a canonical IR contract through
 * the formal specification process.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing must depend only on:
 *
 *     token stream
 *     grammar version
 *
 * It must not depend on:
 *
 *     hardware;
 *     network state;
 *     randomness;
 *     environment variables;
 *     wall-clock time;
 *     resource availability;
 *     runtime state.
 *
 * Identical source and lexical version must yield the same parse structure.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * This grammar contains no target-language actions.
 *
 * It performs no:
 *
 *     filesystem access;
 *     network access;
 *     command execution;
 *     secret access;
 *     cryptographic execution;
 *     proof generation;
 *     proof verification;
 *     hardware discovery;
 *     backend selection.
 *
 * ============================================================================
 */

parser grammar ZeroKnowledge;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * PUBLIC ENTRY POINTS
 * ========================================================================== */

/*
 * Standalone parser entry point.
 *
 * The complete Zamani parser normally reaches this grammar through:
 *
 *     Security
 *       ↓
 *     securityDeclaration
 *       ↓
 *     zeroKnowledgeDeclaration
 *
 * The root Zamani parser remains responsible for consuming the complete
 * compilation unit and EOF.
 */
zeroKnowledgeFile
    : zeroKnowledgeDeclaration* EOF
    ;


/*
 * Composition entry point used by security.g4 / ZamaniParser.g4.
 *
 * No EOF is included here because the parent compilation-unit grammar owns
 * EOF.
 */
zeroKnowledgeDeclaration
    : zeroKnowledgeProtocolDeclaration
    | zeroKnowledgeStatementDeclaration
    | zeroKnowledgeOperation
    | zeroKnowledgeRequirementDeclaration
    | zeroKnowledgeConstraintDeclaration
    | zeroKnowledgePreferenceDeclaration
    | zeroKnowledgePropertyDeclaration
    ;


/* ============================================================================
 * DECLARATION MODEL
 * ========================================================================== */

/*
 * A first-class ZK declaration begins with the dedicated lexical marker:
 *
 *     zero_knowledge
 *
 * followed by an ordinary semantic category name.
 *
 * Examples:
 *
 *     zero_knowledge protocol::identity { ... }
 *     zero_knowledge proof::credential { ... }
 *     zero_knowledge circuit::statement { ... }
 *
 * The category remains an extensible qualified name.
 *
 * It is NOT a closed keyword enumeration.
 */
zeroKnowledgeProtocolDeclaration
    : ZERO_KNOWLEDGE
      qualifiedName
      zeroKnowledgeBody?
      SEMICOLON?
    ;


/*
 * Generic statement declaration.
 *
 * This allows a source program to give a named statement semantic identity
 * without making theorem/proof systems a fixed language dictionary.
 *
 * Examples:
 *
 *     zero_knowledge statement::age
 *     zero_knowledge statement::membership
 *     zero_knowledge statement::quantum_state
 */
zeroKnowledgeStatementDeclaration
    : ZERO_KNOWLEDGE
      qualifiedName
      zeroKnowledgeStatementBody
    ;


/* ============================================================================
 * BODY
 * ========================================================================== */

zeroKnowledgeBody
    : LBRACE
      zeroKnowledgeMember*
      RBRACE
    ;


zeroKnowledgeStatementBody
    : LBRACE
      zeroKnowledgeStatementMember*
      RBRACE
    ;


zeroKnowledgeMember
    : zeroKnowledgeField
    | zeroKnowledgeOperation
    | zeroKnowledgeRequirement
    | zeroKnowledgeConstraint
    | zeroKnowledgePreference
    | zeroKnowledgeProperty
    | zeroKnowledgeReference
    | zeroKnowledgeWitnessDeclaration
    | zeroKnowledgePublicInputDeclaration
    | zeroKnowledgePrivateInputDeclaration
    | zeroKnowledgeCommitmentDeclaration
    | zeroKnowledgeClaimDeclaration
    | zeroKnowledgeProofDeclaration
    | zeroKnowledgeVerifierDeclaration
    | zeroKnowledgeProverDeclaration
    | zeroKnowledgeCompositionDeclaration
    ;


zeroKnowledgeStatementMember
    : zeroKnowledgeField
    | zeroKnowledgeRequirement
    | zeroKnowledgeConstraint
    | zeroKnowledgePreference
    | zeroKnowledgeProperty
    | zeroKnowledgeReference
    | zeroKnowledgeWitnessDeclaration
    | zeroKnowledgePublicInputDeclaration
    | zeroKnowledgePrivateInputDeclaration
    | zeroKnowledgeCommitmentDeclaration
    | zeroKnowledgeClaimDeclaration
    ;


/* ============================================================================
 * GENERIC FIELDS
 * ========================================================================== */

/*
 * Generic fields are deliberately open-world.
 *
 * Examples:
 *
 *     protocol: zk::protocol::future;
 *     field: cryptography::field::reference;
 *     commitment: cryptography::commitment::reference;
 *     transparency: trust::transparent;
 *
 * The field name is syntax.
 * Its meaning is semantic.
 */
zeroKnowledgeField
    : qualifiedName
      COLON
      expression
      SEMICOLON?
    ;


zeroKnowledgeReference
    : qualifiedName
      SEMICOLON?
    ;


/* ============================================================================
 * WITNESSES AND INPUTS
 * ========================================================================== */

/*
 * A witness declaration identifies semantic witness data.
 *
 * It does not contain a secret-storage implementation.
 */
zeroKnowledgeWitnessDeclaration
    : zeroKnowledgeWitnessKeyword
      identifier
      zeroKnowledgeValueType?
      zeroKnowledgeInitializer?
      SEMICOLON?
    ;


zeroKnowledgePublicInputDeclaration
    : zeroKnowledgePublicKeyword
      identifier
      zeroKnowledgeValueType?
      zeroKnowledgeInitializer?
      SEMICOLON?
    ;


zeroKnowledgePrivateInputDeclaration
    : zeroKnowledgePrivateKeyword
      identifier
      zeroKnowledgeValueType?
      zeroKnowledgeInitializer?
      SEMICOLON?
    ;


zeroKnowledgeValueType
    : COLON
      typeExpression
    ;


zeroKnowledgeInitializer
    : ASSIGN
      expression
    ;


/*
 * These category words intentionally remain parser literals rather than
 * additional lexer keywords.
 *
 * This permits the single ZERO_KNOWLEDGE keyword to establish the ZK
 * namespace while keeping the rest of the vocabulary extensible.
 */
zeroKnowledgeWitnessKeyword
    : identifier
    ;


zeroKnowledgePublicKeyword
    : identifier
    ;


zeroKnowledgePrivateKeyword
    : identifier
    ;


/* ============================================================================
 * COMMITMENTS
 * ========================================================================== */

zeroKnowledgeCommitmentDeclaration
    : zeroKnowledgeCommitmentKeyword
      identifier
      zeroKnowledgeInitializer?
      SEMICOLON?
    ;


zeroKnowledgeCommitmentKeyword
    : identifier
    ;


/* ============================================================================
 * CLAIMS / STATEMENTS
 * ========================================================================== */

zeroKnowledgeClaimDeclaration
    : zeroKnowledgeClaimKeyword
      identifier
      COLON
      expression
      SEMICOLON?
    ;


zeroKnowledgeClaimKeyword
    : identifier
    ;


/* ============================================================================
 * PROOF / VERIFIER / PROVER REFERENCES
 * ========================================================================== */

zeroKnowledgeProofDeclaration
    : zeroKnowledgeProofKeyword
      identifier
      zeroKnowledgeBody?
      SEMICOLON?
    ;


zeroKnowledgeVerifierDeclaration
    : zeroKnowledgeVerifierKeyword
      identifier
      zeroKnowledgeBody?
      SEMICOLON?
    ;


zeroKnowledgeProverDeclaration
    : zeroKnowledgeProverKeyword
      identifier
      zeroKnowledgeBody?
      SEMICOLON?
    ;


zeroKnowledgeProofKeyword
    : identifier
    ;


zeroKnowledgeVerifierKeyword
    : identifier
    ;


zeroKnowledgeProverKeyword
    : identifier
    ;


/* ============================================================================
 * COMPOSITION
 * ========================================================================== */

/*
 * Composition is generic.
 *
 * It can represent:
 *
 *     recursive proof systems;
 *     proof aggregation;
 *     proof composition;
 *     batching;
 *     folding;
 *     recursive verification;
 *     multi-statement proofs;
 *     multi-party proving;
 *     proof-carrying computation;
 *     future proof mechanisms.
 *
 * No particular proof technology is enumerated.
 */
zeroKnowledgeCompositionDeclaration
    : zeroKnowledgeCompositionKeyword
      qualifiedName
      zeroKnowledgeCompositionArguments?
      SEMICOLON?
    ;


zeroKnowledgeCompositionArguments
    : LPAREN
      argumentList?
      RPAREN
    ;


zeroKnowledgeCompositionKeyword
    : identifier
    ;


/* ============================================================================
 * OPERATIONS
 * ========================================================================== */

/*
 * Generic ZK operation.
 *
 * The operation name is semantic data.
 *
 * Conceptual examples:
 *
 *     apply zk::prove(statement, witness)
 *     apply zk::verify(statement, proof)
 *     apply zk::commit(value)
 *     apply zk::aggregate(proofs)
 *     apply zk::fold(instance)
 *
 * No operation catalogue is encoded here.
 */
zeroKnowledgeOperation
    : APPLY
      qualifiedName
      zeroKnowledgeOperationArguments?
      zeroKnowledgeOperationBody?
      SEMICOLON?
    ;


zeroKnowledgeOperationArguments
    : LPAREN
      argumentList?
      RPAREN
    ;


zeroKnowledgeOperationBody
    : LBRACE
      zeroKnowledgeMember*
      RBRACE
    ;


/* ============================================================================
 * REQUIREMENTS
 * ========================================================================== */

zeroKnowledgeRequirementDeclaration
    : REQUIRES
      zeroKnowledgeRequirementExpression
      SEMICOLON?
    ;


zeroKnowledgeRequirement
    : REQUIRES
      zeroKnowledgeRequirementExpression
      SEMICOLON?
    ;


zeroKnowledgeRequirementExpression
    : expression
    ;


/* ============================================================================
 * CONSTRAINTS
 * ========================================================================== */

zeroKnowledgeConstraintDeclaration
    : CONSTRAINT
      zeroKnowledgeConstraintExpression
      SEMICOLON?
    ;


zeroKnowledgeConstraint
    : CONSTRAINT
      zeroKnowledgeConstraintExpression
      SEMICOLON?
    ;


zeroKnowledgeConstraintExpression
    : expression
    ;


/* ============================================================================
 * PREFERENCES
 * ========================================================================== */

zeroKnowledgePreferenceDeclaration
    : PREFER
      zeroKnowledgePreferenceExpression
      SEMICOLON?
    ;


zeroKnowledgePreference
    : PREFER
      zeroKnowledgePreferenceExpression
      SEMICOLON?
    ;


zeroKnowledgePreferenceExpression
    : expression
    ;


/* ============================================================================
 * PROPERTIES
 * ========================================================================== */

/*
 * Properties remain generic semantic names.
 *
 * Examples:
 *
 *     property: zero_knowledge;
 *     property: soundness;
 *     property: completeness;
 *     property: extractability;
 *     property: transparency;
 *     property: post_quantum;
 *     property: recursive;
 *
 * The grammar does not decide whether a property is actually satisfied.
 */
zeroKnowledgePropertyDeclaration
    : PROPERTY
      qualifiedName
      (COLON expression)?
      SEMICOLON?
    ;


zeroKnowledgeProperty
    : PROPERTY
      qualifiedName
      (COLON expression)?
      SEMICOLON?
    ;


/* ============================================================================
 * COMMON SEMANTIC CONTRACT
 * ========================================================================== */

/*
 * The following semantic categories are intentionally NOT parser-level
 * enumerations:
 *
 *     zero_knowledge
 *     soundness
 *     completeness
 *     extractability
 *     simulation_soundness
 *     transparency
 *     trusted_setup
 *     universal_setup
 *     recursion
 *     aggregation
 *     folding
 *     batching
 *     post_quantum
 *     quantum_resistant
 *     succinctness
 *     scalability
 *     privacy
 *     non_interactive
 *     interactive
 *     publicly_verifiable
 *     privately_verifiable
 *
 * They are semantic property names.
 *
 * This is essential for long-term extensibility.
 */


/* ============================================================================
 * RESOURCE / CAPABILITY INTEGRATION
 * ========================================================================== */

/*
 * Resource expressions are delegated to the canonical expression/resource
 * model.
 *
 * Examples:
 *
 *     requires qubits >= n;
 *     requires memory >= required_memory;
 *     requires capability("zk.prove");
 *     requires capability("zk.verify");
 *     requires capability("zk.recursive");
 *     requires capability("cryptography.commitment");
 *
 * No resource maximum is defined here.
 */


/* ============================================================================
 * TARGET INDEPENDENCE
 * ========================================================================== */

/*
 * The following are intentionally NOT grammar constructs:
 *
 *     GPU0
 *     QPU0
 *     CPU0
 *     FPGA0
 *     node0
 *     memory_bank0
 *     prover_device_0
 *     verifier_device_0
 *
 * If a deployment explicitly needs target-specific realization, that belongs
 * to:
 *
 *     hardware/
 *     resources/
 *     compile/
 *     execution/
 *
 * and must remain separate from portable ZK semantics.
 */


/* ============================================================================
 * QUANTUM SECURITY BOUNDARY
 * ========================================================================== */

/*
 * A ZK declaration may semantically protect a quantum computation:
 *
 *     zero_knowledge quantum::statement {
 *         ...
 *     }
 *
 * but the ZK grammar does not own quantum semantics.
 *
 * The semantic pipeline is:
 *
 *     ZK source
 *        |
 *        v
 *     domain-neutral AST
 *        |
 *        +-----------------------+
 *        |                       |
 *        v                       v
 *     security semantics    quantum semantics
 *                                |
 *                                v
 *                           quantum::ir
 *                                |
 *                                v
 *                        optimization / routing
 *                                |
 *                           scheduling
 *                                |
 *                        resilience / QEC / ZQN
 *                                |
 *                               HAL
 *
 * Security requirements must survive those transformations.
 */


/* ============================================================================
 * HARDWARE / HDL BOUNDARY
 * ========================================================================== */

/*
 * A proof may concern hardware-generated computation or HDL-described
 * behavior.
 *
 * The ZK grammar may reference the semantic object but does not own:
 *
 *     HDL syntax;
 *     hardware topology;
 *     physical placement;
 *     synthesis;
 *     timing closure;
 *     routing;
 *     device selection.
 *
 * Those remain owned by:
 *
 *     grammar/hdl/
 *     grammar/hardware/
 *     grammar/resources/
 *     grammar/compile/
 */


/* ============================================================================
 * INTEROPERABILITY
 * ========================================================================== */

/*
 * External proof formats are interoperability concerns.
 *
 * Examples include:
 *
 *     vendor proof format
 *     proof-certificate serialization
 *     external verifier format
 *     external circuit representation
 *
 * This grammar should reference such formats semantically rather than
 * embedding their complete grammars.
 *
 * Integration belongs through:
 *
 *     grammar/interoperability/
 */


/* ============================================================================
 * DIAGNOSTIC CONTRACT
 * ========================================================================== */

/*
 * Parsing failures must be reported by the canonical Rust frontend.
 *
 * Diagnostics must preserve:
 *
 *     source file;
 *     source span;
 *     expected syntax;
 *     encountered token;
 *     stable diagnostic category.
 *
 * The grammar itself must not execute diagnostic code.
 */


/* ============================================================================
 * COMPATIBILITY CONTRACT
 * ========================================================================== */

/*
 * Because this file is newly introduced, no existing
 *
 *     grammar/security/zero-knowledge.g4
 *
 * parser contract exists to preserve.
 *
 * However, the repository already contains zero-knowledge concepts in:
 *
 *     grammar/security/cryptography.g4
 *     src/compiler/unique_ir_features.rs
 *     src/stdlib/omniversal_zkp_privacy_computing.rs
 *     src/stdlib/crypto/
 *
 * Those implementations are NOT automatically syntax authority.
 *
 * Existing source syntax must only be promoted into this grammar after it
 * has an explicit specification/compatibility decision.
 *
 * If cryptography.g4 currently exposes a legacy
 * `cryptoZeroKnowledgeDeclaration`, that rule should be migrated through a
 * compatibility adapter rather than duplicated as a second ZK language.
 */


/* ============================================================================
 * TEST CONTRACT
 * ========================================================================== */

/*
 * POSITIVE TESTS
 *
 *     zero_knowledge protocol::example;
 *     zero_knowledge protocol::example {
 *         property: zk::zero_knowledge;
 *     }
 *
 *     zero_knowledge statement::membership {
 *         property: zk::soundness;
 *     }
 *
 *     apply zk::prove(statement, witness);
 *     apply zk::verify(statement, proof);
 *     apply zk::commit(value);
 *
 *     requires capability("zk.prove");
 *     requires capability("zk.verify");
 *
 *     constraint leakage <= budget;
 *     prefer backend("zk");
 *
 * NEGATIVE TESTS
 *
 *     malformed zero_knowledge declaration
 *     missing declaration name
 *     missing closing body
 *     malformed operation arguments
 *     malformed requirement
 *     malformed constraint
 *     malformed property
 *     trailing garbage
 *
 * BOUNDARY TESTS
 *
 *     zero inputs
 *     one input
 *     many inputs
 *     deeply nested proof composition
 *     large witness references
 *     large expression lists
 *     many proof members
 *     many verifier declarations
 *
 * SCALABILITY TESTS
 *
 *     no grammar-defined maximum witness count
 *     no grammar-defined maximum proof count
 *     no grammar-defined maximum recursive composition
 *     no grammar-defined maximum aggregation size
 *     no grammar-defined maximum public/private inputs
 *     no grammar-defined machine-size limit
 *
 * OPEN-WORLD TESTS
 *
 *     unknown future proof-system name parses as qualifiedName
 *     unknown future property name parses as qualifiedName
 *     vendor proof-system reference parses as qualifiedName
 *     future namespace parses without grammar modification
 *
 * CROSS-DOMAIN TESTS
 *
 *     ZK + classical computation
 *     ZK + quantum computation
 *     ZK + hybrid computation
 *     ZK + distributed computation
 *     ZK + HDL/hardware intent
 *     ZK + networking
 *     ZK + AI/data
 *
 * DETERMINISM
 *
 *     identical token stream -> identical parse structure
 *
 * EOF
 *
 *     standalone zeroKnowledgeFile must reject trailing tokens.
 */


/* ============================================================================
 * COMPLETION CRITERIA
 * ========================================================================== */

/*
 * This grammar is complete only when:
 *
 * [ ] ZERO_KNOWLEDGE is present in the canonical keyword registry.
 *
 * [ ] No duplicate ZERO_KNOWLEDGE token exists elsewhere.
 *
 * [ ] This grammar is imported by the canonical Security composition grammar.
 *
 * [ ] ZamaniParser.g4 reaches this grammar only through Security composition.
 *
 * [ ] No second security root is introduced.
 *
 * [ ] No second zero-knowledge IR is introduced.
 *
 * [ ] AST mappings are documented.
 *
 * [ ] Semantic mappings are documented.
 *
 * [ ] Cryptography integration is documented.
 *
 * [ ] Hash integration is documented.
 *
 * [ ] Secrets integration is documented.
 *
 * [ ] Trust integration is documented.
 *
 * [ ] Policy integration is documented.
 *
 * [ ] Capability integration is documented.
 *
 * [ ] Resource integration is documented.
 *
 * [ ] Quantum integration terminates at quantum::ir.
 *
 * [ ] Hardware/HDL realization remains downstream.
 *
 * [ ] No fixed proof-system catalogue exists.
 *
 * [ ] No fixed resource capacities exist.
 *
 * [ ] No physical device identifiers are language primitives.
 *
 * [ ] Positive tests exist.
 *
 * [ ] Negative tests exist.
 *
 * [ ] Boundary tests exist.
 *
 * [ ] Scalability tests exist.
 *
 * [ ] Open-world tests exist.
 *
 * [ ] Cross-domain tests exist.
 *
 * [ ] Determinism tests exist.
 *
 * [ ] Compatibility tests exist.
 *
 * [ ] Rust implementation remains Rust 1.97/1.97.1, Rust 2021, safe Rust.
 *
 * [ ] No unsafe Rust is required.
 */
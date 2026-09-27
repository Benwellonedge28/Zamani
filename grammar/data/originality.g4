/*
 * ============================================================================
 * Zamani — Originality / Provenance Data Grammar
 * Path: grammar/data/originality.g4
 *
 * Status:
 *   Production architecture / independent parser component
 *
 * Language:
 *   Zamani
 *
 * Grammar technology:
 *   ANTLR parser grammar
 *
 * Rust integration:
 *   Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *   No unsafe Rust is required or permitted by this grammar component.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file defines portable SOURCE-LEVEL SYNTAX for describing:
 *
 *   - data provenance;
 *   - derivation;
 *   - authorship metadata;
 *   - contribution metadata;
 *   - transformation lineage;
 *   - source attribution;
 *   - origin claims;
 *   - integrity claims;
 *   - reproducibility metadata;
 *   - derivation relationships;
 *   - dependency relationships;
 *   - evidence references;
 *   - originality-related constraints and requirements.
 *
 * "Originality" in this grammar is DATA / PROGRAM METADATA.
 *
 * This grammar does NOT determine:
 *
 *   - whether something is legally original;
 *   - whether something constitutes plagiarism;
 *   - copyright ownership;
 *   - patent validity;
 *   - legal authorship;
 *   - academic misconduct;
 *   - moral rights;
 *   - jurisdiction-specific legal conclusions.
 *
 * Such interpretations belong to applications, policy engines, semantic
 * analyzers, legal/compliance systems, or external authorities.
 *
 * The grammar describes explicit claims and relationships supplied by the
 * program author.
 *
 * ============================================================================
 * ARCHITECTURAL PRINCIPLE
 * ============================================================================
 *
 * Originality information is part of DATA SEMANTICS and PROVENANCE.
 *
 * It must integrate with the existing:
 *
 *   grammar/data/
 *
 * rather than creating a second provenance language.
 *
 * The intended architecture is:
 *
 *   Zamani source
 *        |
 *        v
 *   shared lexer
 *        |
 *        v
 *   Zamani parser
 *        |
 *        v
 *   data/originality.g4
 *        |
 *        v
 *   syntax AST
 *        |
 *        v
 *   semantic provenance model
 *        |
 *        v
 *   canonical data / provenance representation
 *        |
 *        v
 *   IR / compiler
 *        |
 *        v
 *   storage / execution / distributed / quantum / HDL / AI backends
 *
 * This file MUST NOT directly depend on:
 *
 *   - databases;
 *   - filesystems;
 *   - cloud providers;
 *   - network providers;
 *   - hardware;
 *   - quantum devices;
 *   - GPUs;
 *   - FPGAs;
 *   - CPUs;
 *   - physical memory;
 *   - runtime state.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Originality/provenance metadata must remain portable under:
 *
 *   Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * A provenance declaration describes semantic relationships.
 *
 * It does NOT prescribe:
 *
 *   - where metadata is physically stored;
 *   - how many storage nodes exist;
 *   - how many replicas exist;
 *   - which database stores it;
 *   - which machine computes it;
 *   - which network transports it;
 *   - which accelerator processes it.
 *
 * Those decisions are made downstream.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There are NO grammar-level maximums for:
 *
 *   - contributors;
 *   - sources;
 *   - derivations;
 *   - transformations;
 *   - dependencies;
 *   - evidence records;
 *   - provenance edges;
 *   - datasets;
 *   - records;
 *   - lineage depth;
 *   - lineage breadth;
 *   - versions;
 *   - artifacts;
 *   - declarations.
 *
 * Repetition is represented with ANTLR's unbounded constructs.
 *
 * Practical limits are implementation/resource limits, not language limits.
 *
 * The grammar MUST NOT introduce:
 *
 *   MAX_AUTHORS
 *   MAX_SOURCES
 *   MAX_DERIVATIONS
 *   MAX_PROVENANCE_EDGES
 *   MAX_LINEAGE_DEPTH
 *   MAX_EVIDENCE
 *   MAX_VERSIONS
 *   MAX_ARTIFACTS
 *
 * or equivalent hidden restrictions.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - originality declaration syntax;
 *   - provenance claim syntax;
 *   - derivation relationship syntax;
 *   - contributor metadata syntax;
 *   - source attribution syntax;
 *   - evidence-reference syntax;
 *   - integrity claim syntax;
 *   - reproducibility metadata syntax;
 *   - lineage declarations;
 *   - originality-related requirements/constraints/preferences.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - general data schemas;
 *   - general records;
 *   - collections;
 *   - streams;
 *   - generic transformations;
 *   - serialization implementation;
 *   - cryptographic implementation;
 *   - identity implementation;
 *   - authentication;
 *   - authorization;
 *   - legal interpretation;
 *   - general expression syntax;
 *   - general type syntax;
 *   - resource discovery;
 *   - hardware;
 *   - quantum semantics;
 *   - AI model semantics;
 *   - HDL semantics.
 *
 * Those remain owned by their existing repository components.
 *
 * ============================================================================
 * INTEGRATION RULE
 * ============================================================================
 *
 * This grammar is an independent parser grammar component.
 *
 * It uses:
 *
 *     tokenVocab = Zamani;
 *
 * and therefore relies on the authoritative Zamani lexical vocabulary.
 *
 * The data facade:
 *
 *     grammar/data/data.g4
 *
 * should expose exactly one delegated entry point for this component:
 *
 *     originalityDeclaration
 *
 * or, where the repository uses a broader data statement dispatcher:
 *
 *     dataOriginalityConstruct
 *
 * The root grammar must not duplicate the alternatives defined here.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar intentionally does NOT require a new AST type to be embedded
 * here.
 *
 * The preferred AST integration is:
 *
 *   generic declaration / metadata node
 *          |
 *          +-- originality/provenance payload
 *
 * The semantic layer should map that payload into the repository's canonical
 * provenance/data model.
 *
 * Do NOT create a second AST hierarchy solely for originality.
 *
 * Every construct should preserve source spans through the normal parser ->
 * AST conversion.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * The semantic analyzer is responsible for:
 *
 *   - validating references;
 *   - resolving identifiers;
 *   - checking duplicate declarations;
 *   - checking relationship consistency;
 *   - checking required evidence;
 *   - validating integrity algorithms;
 *   - validating claims against available provenance;
 *   - checking reproducibility requirements;
 *   - checking dependency consistency;
 *   - checking temporal/version relationships.
 *
 * The grammar itself performs none of those operations.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * The semantic representation should lower to the repository's canonical
 * data/provenance representation.
 *
 * This file MUST NOT create:
 *
 *     originality::ir
 *
 * or another competing provenance IR.
 *
 * The intended boundary is:
 *
 *     data/originality syntax
 *             |
 *             v
 *     semantic provenance model
 *             |
 *             v
 *     canonical data / provenance IR
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This grammar contains no machine-capacity constants.
 *
 * It contains no:
 *
 *   MAX_CPU
 *   MAX_GPU
 *   MAX_QPU
 *   MAX_FPGA
 *   MAX_NODE
 *   MAX_MEMORY
 *   MAX_THREAD
 *   MAX_TENSOR_RANK
 *   MAX_REGISTER_WIDTH
 *   MAX_NETWORK_SIZE
 *   MAX_DEVICE_COUNT
 *
 * A numeric value appearing in source is data supplied by the programmer,
 * such as a version, timestamp, confidence value, sequence number, or
 * semantic requirement. It is not a compiler capacity limit.
 *
 * ============================================================================
 */

parser grammar ZamaniDataOriginalityParser;

options {
    tokenVocab = Zamani;
}


/*
 * ============================================================================
 * PUBLIC ENTRY POINTS
 * ============================================================================
 *
 * These are the only rules that other grammar components should import/use.
 * ============================================================================
 */

/*
 * A complete originality/provenance declaration.
 *
 * Example:
 *
 *   originality dataset {
 *       claim original;
 *       source "experiment.zm";
 *   }
 */
originalityDeclaration
    : originalityHeader
      '{'
      originalityMember*
      '}'
    ;


/*
 * A standalone provenance declaration.
 *
 * This provides a general entry point for data lineage that does not need to
 * make an explicit originality claim.
 */
provenanceDeclaration
    : 'provenance'
      provenanceSubject?
      '{'
      provenanceMember*
      '}'
    ;


/*
 * A standalone derivation declaration.
 */
derivationDeclaration
    : 'derivation'
      IDENTIFIER?
      '{'
      derivationMember*
      '}'
    ;


/*
 * ============================================================================
 * ORIGINALITY HEADER
 * ============================================================================
 */

originalityHeader
    : 'originality'
      originalitySubject?
    ;


originalitySubject
    : IDENTIFIER
    | STRING
    ;


/*
 * ============================================================================
 * ORIGINALITY MEMBERS
 * ============================================================================
 */

originalityMember
    : originalityClaim
    | originalityStatus
    | originalityContributor
    | originalitySource
    | originalityDerivation
    | originalityEvidence
    | originalityIntegrity
    | originalityReproducibility
    | originalityDependency
    | originalityVersion
    | originalityLineage
    | originalityRequirement
    | originalityConstraint
    | originalityPreference
    | originalityAttribute
    ;


/*
 * ============================================================================
 * CLAIMS
 * ============================================================================
 *
 * A claim is an explicit semantic assertion made by the program author.
 *
 * The grammar intentionally does not decide whether the claim is true.
 * ============================================================================
 */

originalityClaim
    : 'claim'
      originalityClaimKind
      originalityClaimTarget?
      originalityClaimQualifier*
      ';'
    ;


originalityClaimKind
    : 'original'
    | 'derived'
    | 'adapted'
    | 'transformed'
    | 'composed'
    | 'generated'
    | 'synthetic'
    | 'reproduced'
    | 'replicated'
    | 'unknown'
    | 'disputed'
    | IDENTIFIER
    ;


originalityClaimTarget
    : 'for'
      originalityReference
    ;


originalityClaimQualifier
    : 'because'
      expression
    | 'according_to'
      originalityReferenceList
    | 'with'
      originalityEvidenceReferenceList
    ;


/*
 * ============================================================================
 * STATUS
 * ============================================================================
 */

originalityStatus
    : 'status'
      originalityStatusKind
      ';'
    ;


originalityStatusKind
    : 'claimed'
    | 'verified'
    | 'unverified'
    | 'disputed'
    | 'unknown'
    | 'incomplete'
    | 'superseded'
    | IDENTIFIER
    ;


/*
 * ============================================================================
 * CONTRIBUTORS
 * ============================================================================
 *
 * A contributor is a logical identity reference plus optional contribution
 * metadata.
 *
 * Identity resolution belongs outside the grammar.
 * ============================================================================
 */

originalityContributor
    : 'contributor'
      originalityContributorSubject?
      '{'
      originalityContributorMember*
      '}'
    ;


originalityContributorSubject
    : IDENTIFIER
    | STRING
    ;


originalityContributorMember
    : originalityContributorRole
    | originalityContributorContribution
    | originalityContributorReference
    | originalityAttribute
    ;


originalityContributorRole
    : 'role'
      originalityIdentifierOrString
      ';'
    ;


originalityContributorContribution
    : 'contribution'
      originalityContributionKind
      originalityContributionTarget?
      ';'
    ;


originalityContributionKind
    : 'concept'
    | 'design'
    | 'implementation'
    | 'research'
    | 'analysis'
    | 'data'
    | 'experiment'
    | 'documentation'
    | 'verification'
    | 'validation'
    | 'review'
    | 'maintenance'
    | 'funding'
    | 'infrastructure'
    | 'other'
    | IDENTIFIER
    ;


originalityContributionTarget
    : 'to'
      originalityReference
    ;


originalityContributorReference
    : 'identity'
      originalityReference
      ';'
    ;


/*
 * ============================================================================
 * SOURCES
 * ============================================================================
 *
 * Sources identify logical origins.
 *
 * A source may be:
 *
 *   - a named data object;
 *   - another Zamani artifact;
 *   - a URI represented as opaque text;
 *   - a logical resource identifier;
 *   - another provenance object.
 *
 * The grammar deliberately does not interpret the string.
 * ============================================================================
 */

originalitySource
    : 'source'
      originalitySourceKind?
      originalityReference
      originalitySourceQualifier*
      ';'
    ;


originalitySourceKind
    : 'data'
    | 'code'
    | 'model'
    | 'document'
    | 'dataset'
    | 'experiment'
    | 'artifact'
    | 'human'
    | 'system'
    | 'generated'
    | 'external'
    | IDENTIFIER
    ;


originalitySourceQualifier
    : 'version'
      originalityValue
    | 'revision'
      originalityValue
    | 'at'
      originalityValue
    | 'using'
      originalityReferenceList
    | 'with'
      originalityAttributeList
    ;


/*
 * ============================================================================
 * DERIVATION
 * ============================================================================
 */

originalityDerivation
    : 'derived_from'
      originalityReferenceList
      originalityDerivationQualifier*
      ';'
    ;


originalityDerivationQualifier
    : 'by'
      originalityReference
    | 'using'
      originalityReferenceList
    | 'through'
      originalityReference
    | 'at'
      originalityValue
    | 'because'
      expression
    ;


/*
 * ============================================================================
 * EXPLICIT DERIVATION BLOCK
 * ============================================================================
 */

derivationMember
    : derivationInput
    | derivationOutput
    | derivationOperation
    | derivationContributor
    | derivationEvidence
    | derivationRequirement
    | derivationConstraint
    | derivationAttribute
    ;


derivationInput
    : 'input'
      originalityReferenceList
      ';'
    ;


derivationOutput
    : 'output'
      originalityReferenceList
      ';'
    ;


derivationOperation
    : 'operation'
      originalityReference
      ';'
    ;


derivationContributor
    : 'contributor'
      originalityReferenceList
      ';'
    ;


derivationEvidence
    : 'evidence'
      originalityEvidenceReferenceList
      ';'
    ;


derivationRequirement
    : 'requires'
      '('
      expression
      ')'
      ';'
    ;


derivationConstraint
    : 'constraint'
      '('
      expression
      ')'
      ';'
    ;


/*
 * ============================================================================
 * EVIDENCE
 * ============================================================================
 *
 * Evidence is metadata referring to material that supports a claim.
 *
 * The grammar does not decide evidentiary sufficiency.
 * ============================================================================
 */

originalityEvidence
    : 'evidence'
      originalityEvidenceKind?
      originalityEvidenceReference
      originalityEvidenceQualifier*
      ';'
    ;


originalityEvidenceKind
    : 'source'
    | 'artifact'
    | 'record'
    | 'experiment'
    | 'measurement'
    | 'test'
    | 'proof'
    | 'signature'
    | 'hash'
    | 'attestation'
    | 'review'
    | 'citation'
    | 'reference'
    | 'other'
    | IDENTIFIER
    ;


originalityEvidenceReference
    : originalityReference
    ;


originalityEvidenceReferenceList
    : originalityEvidenceReference
      (',' originalityEvidenceReference)*
    ;


originalityEvidenceQualifier
    : 'for'
      originalityReferenceList
    | 'by'
      originalityReferenceList
    | 'at'
      originalityValue
    | 'method'
      originalityReference
    | 'confidence'
      expression
    | 'hash'
      originalityHashValue
    | 'signature'
      originalityReference
    ;


/*
 * ============================================================================
 * INTEGRITY
 * ============================================================================
 *
 * Integrity is intentionally separated from originality.
 *
 * An integrity claim says something about consistency or tamper evidence.
 * It does not prove authorship or originality.
 * ============================================================================
 */

originalityIntegrity
    : 'integrity'
      originalityIntegrityTarget?
      '{'
      originalityIntegrityMember*
      '}'
    ;


originalityIntegrityTarget
    : 'of'
      originalityReference
    ;


originalityIntegrityMember
    : originalityIntegrityAlgorithm
    | originalityIntegrityDigest
    | originalityIntegritySignature
    | originalityIntegrityRequirement
    | originalityIntegrityAttribute
    ;


originalityIntegrityAlgorithm
    : 'algorithm'
      originalityIdentifierOrString
      ';'
    ;


originalityIntegrityDigest
    : 'digest'
      originalityValue
      ';'
    ;


originalityIntegritySignature
    : 'signature'
      originalityReference
      ';'
    ;


originalityIntegrityRequirement
    : 'requires'
      '('
      expression
      ')'
      ';'
    ;


originalityIntegrityAttribute
    : originalityAttribute
    ;


/*
 * ============================================================================
 * REPRODUCIBILITY
 * ============================================================================
 */

originalityReproducibility
    : 'reproducibility'
      originalityReproducibilityTarget?
      '{'
      originalityReproducibilityMember*
      '}'
    ;


originalityReproducibilityTarget
    : 'for'
      originalityReference
    ;


originalityReproducibilityMember
    : reproducibilityRequirement
    | reproducibilityInput
    | reproducibilityEnvironment
    | reproducibilityVersion
    | reproducibilitySeed
    | reproducibilityProcedure
    | reproducibilityEvidence
    | reproducibilityAttribute
    ;


reproducibilityRequirement
    : 'requires'
      '('
      expression
      ')'
      ';'
    ;


reproducibilityInput
    : 'input'
      originalityReferenceList
      ';'
    ;


reproducibilityEnvironment
    : 'environment'
      originalityReference
      ';'
    ;


reproducibilityVersion
    : 'version'
      originalityValue
      ';'
    ;


reproducibilitySeed
    : 'seed'
      expression
      ';'
    ;


reproducibilityProcedure
    : 'procedure'
      originalityReference
      ';'
    ;


reproducibilityEvidence
    : 'evidence'
      originalityEvidenceReferenceList
      ';'
    ;


/*
 * ============================================================================
 * DEPENDENCIES
 * ============================================================================
 *
 * Dependencies are logical relationships.
 *
 * They do not prescribe physical deployment.
 * ============================================================================
 */

originalityDependency
    : 'depends_on'
      originalityReferenceList
      originalityDependencyQualifier*
      ';'
    ;


originalityDependencyQualifier
    : 'version'
      originalityValue
    | 'optional'
    | 'required'
    | 'transitive'
    | 'direct'
    | 'because'
      expression
    ;


/*
 * ============================================================================
 * VERSION / REVISION
 * ============================================================================
 */

originalityVersion
    : 'version'
      originalityValue
      originalityVersionQualifier*
      ';'
    ;


originalityVersionQualifier
    : 'of'
      originalityReference
    | 'derived_from'
      originalityReferenceList
    | 'supersedes'
      originalityReferenceList
    | 'based_on'
      originalityReferenceList
    | 'at'
      originalityValue
    ;


/*
 * ============================================================================
 * LINEAGE
 * ============================================================================
 *
 * Lineage is intentionally a logical graph.
 *
 * No maximum graph size is encoded.
 * ============================================================================
 */

originalityLineage
    : 'lineage'
      originalityLineageTarget?
      '{'
      originalityLineageMember*
      '}'
    ;


originalityLineageTarget
    : 'of'
      originalityReference
    ;


originalityLineageMember
    : lineageParent
    | lineageChild
    | lineageTransform
    | lineageMerge
    | lineageSplit
    | lineageFilter
    | lineageEvidence
    | lineageAttribute
    ;


lineageParent
    : 'parent'
      originalityReferenceList
      ';'
    ;


lineageChild
    : 'child'
      originalityReferenceList
      ';'
    ;


lineageTransform
    : 'transform'
      originalityReference
      ';'
    ;


lineageMerge
    : 'merge'
      originalityReferenceList
      ';'
    ;


lineageSplit
    : 'split'
      originalityReferenceList
      ';'
    ;


lineageFilter
    : 'filter'
      expression
      ';'
    ;


lineageEvidence
    : 'evidence'
      originalityEvidenceReferenceList
      ';'
    ;


/*
 * ============================================================================
 * REQUIREMENTS
 * ============================================================================
 *
 * Requirements are semantic constraints.
 *
 * They are not hardware capacities.
 * ============================================================================
 */

originalityRequirement
    : 'requires'
      originalityRequirementKind
      originalityRequirementValue?
      ';'
    ;


originalityRequirementKind
    : 'attribution'
    | 'provenance'
    | 'traceability'
    | 'integrity'
    | 'reproducibility'
    | 'lineage'
    | 'evidence'
    | 'source'
    | 'identity'
    | 'verification'
    | 'validation'
    | 'audit'
    | IDENTIFIER
    ;


originalityRequirementValue
    : originalityValue
    | '('
      expression
      ')'
    ;


/*
 * ============================================================================
 * CONSTRAINTS
 * ============================================================================
 */

originalityConstraint
    : 'constraint'
      originalityConstraintKind?
      '('
      expression
      ')'
      ';'
    ;


originalityConstraintKind
    : 'attribution'
    | 'provenance'
    | 'integrity'
    | 'lineage'
    | 'reproducibility'
    | 'dependency'
    | 'version'
    | 'evidence'
    | IDENTIFIER
    ;


/*
 * ============================================================================
 * PREFERENCES
 * ============================================================================
 */

originalityPreference
    : 'prefer'
      originalityPreferenceKind
      originalityValue?
      ';'
    ;


originalityPreferenceKind
    : 'canonical_source'
    | 'explicit_provenance'
    | 'verifiable_evidence'
    | 'reproducible'
    | 'traceable'
    | 'content_addressed'
    | 'versioned'
    | IDENTIFIER
    ;


/*
 * ============================================================================
 * GENERIC ATTRIBUTES
 * ============================================================================
 *
 * Attributes are intentionally generic and opaque.
 *
 * This permits future provenance metadata without repeatedly changing the
 * grammar for every possible domain-specific metadata field.
 * ============================================================================
 */

originalityAttribute
    : '@'
      IDENTIFIER
      (
          '('
          argumentList?
          ')'
      )?
    ;


originalityAttributeList
    : originalityAttribute
      (',' originalityAttribute)*
    ;


/*
 * ============================================================================
 * REFERENCES
 * ============================================================================
 *
 * A reference can identify:
 *
 *   - a local Zamani symbol;
 *   - a qualified symbol;
 *   - a quoted external/logical identifier.
 *
 * Physical interpretation is downstream.
 * ============================================================================
 */

originalityReference
    : IDENTIFIER
    | originalityQualifiedReference
    | STRING
    ;


originalityQualifiedReference
    : IDENTIFIER
      (
          '::'
          IDENTIFIER
      )+
    ;


originalityReferenceList
    : originalityReference
      (',' originalityReference)*
    ;


/*
 * ============================================================================
 * VALUES
 * ============================================================================
 *
 * Values deliberately reuse the canonical Zamani expression system.
 *
 * No duplicate numeric/string/boolean literal system is created here.
 * ============================================================================
 */

originalityValue
    : expression
    ;


originalityIdentifierOrString
    : IDENTIFIER
    | STRING
    ;


originalityHashValue
    : STRING
    | IDENTIFIER
    ;


/*
 * ============================================================================
 * COMMON DATA-ORIGINALITY INTEGRATION
 * ============================================================================
 *
 * This rule is useful when data.g4 needs a single generic construct without
 * having to distinguish the individual declaration forms at its own level.
 *
 * data.g4 should delegate here rather than reproducing these alternatives.
 * ============================================================================
 */

dataOriginalityConstruct
    : originalityDeclaration
    | provenanceDeclaration
    | derivationDeclaration
    ;


/*
 * ============================================================================
 * SEMANTIC EXAMPLES
 * ============================================================================
 *
 * These examples are NON-NORMATIVE documentation only.
 *
 * Example 1 — explicit claim:
 *
 *   originality result {
 *       claim original;
 *       source "experiment-input";
 *   }
 *
 *
 * Example 2 — derived data:
 *
 *   originality result {
 *       claim derived for result;
 *       derived_from dataset_a, dataset_b;
 *   }
 *
 *
 * Example 3 — contribution:
 *
 *   originality model {
 *       contributor researcher {
 *           role "designer";
 *           contribution design to model;
 *       }
 *   }
 *
 *
 * Example 4 — provenance:
 *
 *   provenance result {
 *       source dataset_a;
 *       source dataset_b;
 *       derived_from dataset_a, dataset_b;
 *   }
 *
 *
 * Example 5 — reproducibility:
 *
 *   originality experiment {
 *       reproducibility {
 *           input dataset;
 *           version "1";
 *           seed seed_value;
 *           procedure experiment_pipeline;
 *       }
 *   }
 *
 *
 * Example 6 — integrity:
 *
 *   originality artifact {
 *       integrity of artifact {
 *           algorithm "content_digest";
 *           digest "opaque-digest";
 *       }
 *   }
 *
 *
 * Example 7 — lineage:
 *
 *   provenance output {
 *       lineage of output {
 *           parent source_data;
 *           transform transformation;
 *       }
 *   }
 *
 *
 * These examples do not imply that:
 *
 *   claim original
 *
 * is automatically true.
 *
 * Semantic verification is downstream.
 *
 * ============================================================================
 * INTEGRATION CHECKLIST
 * ============================================================================
 *
 * Before this file is marked COMPLETE:
 *
 * [ ] tokenVocab resolves against canonical Zamani vocabulary
 * [ ] no lexer rules are duplicated here
 * [ ] no second provenance AST is introduced
 * [ ] data.g4 delegates rather than duplicates these rules
 * [ ] root Zamani.g4 exposes the data entry point only once
 * [ ] existing data provenance declarations are reconciled
 * [ ] general expression syntax is reused
 * [ ] general type syntax is reused
 * [ ] identifiers are shared with the canonical lexer
 * [ ] source spans are preserved by AST conversion
 * [ ] semantic validation is external to the grammar
 * [ ] legal/originality judgments are external to the grammar
 * [ ] integrity verification is external to the grammar
 * [ ] cryptographic implementation is external to the grammar
 * [ ] no provider-specific syntax is introduced
 * [ ] no hardware-specific syntax is introduced
 * [ ] no quantum-specific hardware assumptions are introduced
 * [ ] no fixed cardinality is introduced
 * [ ] no MAX_* capacity is introduced
 * [ ] positive tests exist
 * [ ] negative tests exist
 * [ ] boundary tests exist
 * [ ] scalability tests exist
 * [ ] deterministic parsing tests exist
 * [ ] compatibility tests exist
 * [ ] Rust integration remains compatible with Rust 1.97/1.97.1
 * [ ] no unsafe Rust is required
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */
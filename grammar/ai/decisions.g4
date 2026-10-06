/**
 * Zamani Programming Language
 * File: grammar/ai/decisions.g4
 *
 * PURPOSE
 * -------
 * Defines the grammar boundary for target-independent computational decisions.
 *
 * A decision represents the selection of an outcome, action, alternative, or
 * strategy from available candidates under explicitly expressible conditions.
 *
 * This grammar is intentionally domain-neutral. A decision may be used by:
 *
 *   - classical computation
 *   - AI/ML
 *   - probabilistic computation
 *   - quantum-classical workflows
 *   - resource negotiation
 *   - execution planning
 *   - scheduling
 *   - distributed computation
 *   - security/policy evaluation
 *   - compiler optimization
 *   - hardware selection
 *   - simulation
 *   - adaptive execution
 *   - future computational domains
 *
 * The grammar describes portable intent. It does NOT describe physical target
 * topology, device counts, register widths, qubit limits, memory limits,
 * processor limits, or implementation-specific capacities.
 *
 *
 * ARCHITECTURAL OWNERSHIP
 * -----------------------
 *
 * OWNS:
 *   decisionDeclaration
 *   decisionBody
 *   decisionItem
 *   decisionCandidate
 *   decisionCriterion
 *   decisionCondition
 *   decisionOutcome
 *   decisionSelection
 *   decisionReference
 *   decisionStatement
 *   decisionExpression
 *   decisionOperation
 *   decisionArgumentList
 *   decisionArgument
 *   decisionNamedArgument
 *   decisionClause
 *
 * DOES NOT OWN:
 *   - general expressions
 *   - types
 *   - names
 *   - assertions
 *   - reasoning
 *   - inference
 *   - deduction
 *   - induction
 *   - abduction
 *   - evidence
 *   - provenance
 *   - uncertainty
 *   - probability
 *   - confidence
 *   - policies
 *   - resources
 *   - capabilities
 *   - effects
 *   - contracts
 *   - execution
 *   - scheduling
 *   - quantum IR
 *   - classical IR
 *   - hardware realization
 *
 *
 * PUBLIC RULES
 * ------------
 *
 *   decisionDeclaration
 *   decisionReference
 *   decisionStatement
 *   decisionExpression
 *
 *
 * PRIVATE RULES
 * -------------
 *
 * All remaining rules in this file are implementation details of the
 * decision grammar unless explicitly exported by the grammar composition
 * layer.
 *
 *
 * DEPENDENCY CONTRACT
 * -------------------
 *
 * DEPENDS_ON:
 *   ../expressions/expressions
 *   ../types/types
 *   ../core/names
 *   ../core/attributes
 *   ../core/modifiers
 *   ../core/metadata
 *   ../ai/reasoning
 *   ../ai/evidence
 *   ../ai/provenance
 *
 * EXPORTS:
 *   decisionDeclaration
 *   decisionReference
 *   decisionStatement
 *   decisionExpression
 *
 * CONSUMED_BY:
 *   ../ai/ai.g4
 *   ../ai/explanations.g4
 *   ../ai/reasoning.g4
 *   ../ai/agents.g4
 *   ../ai/planning.g4
 *   ../ai/policies.g4
 *   ../execution/*
 *   ../distributed/*
 *   ../quantum/*
 *   ../hybrid/*
 *   ../resources/*
 *   ../security/*
 *   ../classical/*
 *
 * AST_OWNER:
 *   Domain-neutral frontend AST / semantic model.
 *
 * SEMANTIC_OWNER:
 *   Decision semantic subsystem.
 *
 * IR_OWNER:
 *   Canonical semantic IR.
 *   Decisions must lower to generic operations/control-flow/selection
 *   structures and, where required, to domain IR such as classical IR or
 *   quantum::ir. This grammar must never select a physical target directly.
 *
 * TEST_OWNER:
 *   grammar/tests/ai/decisions/
 *
 * SPEC_OWNER:
 *   grammar/spec/ai.md
 *   grammar/specification/ai/decisions.md
 *
 *
 * AST CONTRACT
 * ------------
 *
 * A parsed decision should preserve at least:
 *
 *   identity
 *   candidates
 *   criteria
 *   conditions
 *   selection policy/reference
 *   selected outcome/action
 *   evidence references
 *   reasoning references
 *   provenance references
 *   uncertainty/confidence expressions
 *   contracts
 *   policies
 *   attributes
 *   modifiers
 *   metadata
 *   source location
 *
 * The AST must remain domain-neutral.
 *
 *
 * SEMANTIC CONTRACT
 * -----------------
 *
 * A decision is NOT synonymous with truth.
 *
 * A decision may select:
 *
 *   - a value
 *   - an expression
 *   - an action
 *   - an operation
 *   - a strategy
 *   - a resource realization
 *   - an execution alternative
 *   - another semantic entity
 *
 * The semantic layer determines the valid decision domain.
 *
 * Decisions must preserve:
 *
 *   evidence
 *   reasoning
 *   uncertainty
 *   provenance
 *   policy
 *   contract
 *   capability
 *   resource
 *   effect
 *
 * information where applicable.
 *
 *
 * TYPE CONTRACT
 * -------------
 *
 * Candidate values and outcomes are type checked by the existing type system.
 *
 * This grammar does not introduce fixed candidate types or fixed numeric
 * domains.
 *
 * Decision criteria must type-check against the values/entities they inspect.
 *
 *
 * EFFECT CONTRACT
 * ---------------
 *
 * A decision is not inherently effectful.
 *
 * Effects arise from the selected expression/action and from the semantic
 * decision operation.
 *
 * Possible effects include existing universal effects such as:
 *
 *   mutation
 *   io
 *   network
 *   randomness
 *   measurement
 *   distributed
 *   native
 *   foreign
 *   learning
 *   adaptation
 *   reflection
 *   simulation
 *
 * This grammar does not invent a second effect system.
 *
 *
 * CAPABILITY CONTRACT
 * -------------------
 *
 * Capability requirements belong to the existing capability/resource system.
 *
 * A decision may depend on capabilities, but this grammar does not enumerate
 * hardware capabilities.
 *
 *
 * RESOURCE CONTRACT
 * -----------------
 *
 * Resource requirements belong to the existing resource subsystem.
 *
 * Decision criteria may inspect symbolic resource information, but no fixed
 * resource capacity is encoded here.
 *
 * No maximum number of candidates, criteria, alternatives, branches, targets,
 * resources, devices, processors, qubits, nodes, threads, or similar entities
 * is defined.
 *
 *
 * CONTRACT CONTRACT
 * -----------------
 *
 * Decisions may participate in:
 *
 *   requires
 *   ensures
 *   invariant
 *   assume
 *   guarantee
 *   property
 *
 * The existing contract/validation subsystem owns their semantics.
 *
 *
 * POLICY CONTRACT
 * ---------------
 *
 * A decision may be constrained by policies.
 *
 * Policy semantics are owned by the policy subsystem.
 *
 * A policy can constrain:
 *
 *   - allowable outcomes
 *   - candidate eligibility
 *   - resource use
 *   - execution
 *   - security
 *   - adaptation
 *   - deployment
 *   - fallback
 *
 *
 * EVIDENCE CONTRACT
 * -----------------
 *
 * Evidence is referenced, not reimplemented.
 *
 * The evidence subsystem owns:
 *
 *   claims
 *   sources
 *   support
 *   contradiction
 *   verification
 *   evidence strength
 *
 *
 * REASONING CONTRACT
 * ------------------
 *
 * Reasoning is referenced, not reimplemented.
 *
 * The reasoning subsystem owns:
 *
 *   inference
 *   deduction
 *   induction
 *   abduction
 *   premises
 *   conclusions
 *   reasoning relations
 *
 *
 * PROVENANCE CONTRACT
 * -------------------
 *
 * Decision creation, evaluation, selection, and subsequent transformations
 * may carry provenance.
 *
 * Provenance is owned by ../ai/provenance.
 *
 *
 * UNCERTAINTY CONTRACT
 * --------------------
 *
 * Confidence, probability, belief, distributions, and uncertainty are
 * represented through expressions and existing uncertainty semantics.
 *
 * This grammar does not hard-code a probability representation.
 *
 *
 * QUANTUM BOUNDARY
 * ----------------
 *
 * Decisions may control quantum operations semantically.
 *
 * They must lower through the canonical quantum semantic boundary and
 * ultimately through quantum::ir.
 *
 * This file must not encode:
 *
 *   physical qubit identifiers
 *   hardware coupling maps
 *   routing
 *   calibration
 *   QEC
 *   pulse schedules
 *   vendor instructions
 *
 *
 * HDL BOUNDARY
 * ------------
 *
 * Decisions may participate in hardware control and verification, but HDL
 * realization belongs to the HDL subsystem.
 *
 *
 * BACKEND BOUNDARY
 * ----------------
 *
 * Backend selection is determined by semantic/resource/capability negotiation.
 *
 * This grammar never embeds:
 *
 *   CPU IDs
 *   GPU IDs
 *   FPGA IDs
 *   QPU IDs
 *   node counts
 *   memory sizes
 *   fixed register widths
 *   fixed topology sizes
 *
 *
 * SCALABILITY CONTRACT
 * --------------------
 *
 * All repetition is represented using ANTLR repetition operators.
 *
 * No grammar-level finite capacity is imposed.
 *
 * Scaling is limited only by:
 *
 *   - available source representation
 *   - parser implementation resources
 *   - semantic resources
 *   - compilation resources
 *   - execution resources
 *
 * and not by artificial language constants.
 *
 *
 * COMPATIBILITY
 * -------------
 *
 * New decision operations and decision metadata should be introduced through
 * semantic registries/dialects whenever possible rather than by expanding
 * the universal keyword set.
 *
 * Existing decision syntax must remain parseable unless explicitly deprecated.
 *
 *
 * DIAGNOSTICS
 * -----------
 *
 * Structural errors are parser errors.
 *
 * Semantic errors such as:
 *
 *   - invalid candidate type
 *   - unknown decision operation
 *   - unsatisfied policy
 *   - unavailable capability
 *   - insufficient resources
 *   - invalid evidence
 *   - invalid reasoning reference
 *   - contradictory contract
 *   - invalid provenance
 *
 * belong to later validation/semantic phases.
 *
 *
 * POSITIVE TESTS
 * --------------
 *
 *   decision basic {
 *       candidate: a;
 *       candidate: b;
 *       criterion: score(a);
 *       criterion: score(b);
 *   }
 *
 *   decision selection {
 *       candidates: alternatives;
 *       criterion: objective(alternatives);
 *       policy: selection_policy;
 *   }
 *
 *   decision adaptive {
 *       condition: state;
 *       outcome: action;
 *       evidence: evidence_ref;
 *   }
 *
 *
 * NEGATIVE TESTS
 * --------------
 *
 * Must cover:
 *
 *   - malformed decision body
 *   - malformed candidate
 *   - malformed criterion
 *   - malformed selection
 *   - malformed reference
 *   - malformed argument list
 *
 *
 * BOUNDARY TESTS
 * --------------
 *
 * Must cover decisions combined with:
 *
 *   reasoning
 *   evidence
 *   uncertainty
 *   provenance
 *   contracts
 *   policies
 *   resources
 *   capabilities
 *   quantum operations
 *   classical operations
 *   distributed execution
 *   simulation
 *
 *
 * COMPLETION CRITERIA
 * -------------------
 *
 * This file is DONE when:
 *
 *   1. It parses the complete decision syntax defined by the specification.
 *   2. It introduces no physical capacity limit.
 *   3. It does not duplicate reasoning/evidence/provenance/policy semantics.
 *   4. It integrates with the AI grammar composition.
 *   5. Its AST contract is implemented.
 *   6. Its semantic contract is implemented.
 *   7. Positive, negative, boundary, scalability, and compatibility tests pass.
 *   8. Decision lowering has a defined canonical IR path.
 *   9. No backend-specific syntax is required.
 *  10. No unsafe Rust is required by the grammar or its downstream contract.
 */


/*
 * Grammar composition
 */
grammar decisions {
    import
        ../expressions/expressions,
        ../types/types,
        ../core/names,
        ../core/attributes,
        ../core/modifiers,
        ../core/metadata,
        ../ai/reasoning,
        ../ai/evidence,
        ../ai/provenance
    ;


/*
 * --------------------------------------------------------------------------
 * PUBLIC DECISION DECLARATION
 * --------------------------------------------------------------------------
 *
 * Canonical declarative form.
 *
 * The identifier is optional so anonymous decision blocks remain possible.
 */
decisionDeclaration
    : DECISION decisionName? LBRACE decisionItem* RBRACE
    ;


/*
 * Decision names use the existing universal naming subsystem.
 */
decisionName
    : qualifiedName
    ;


/*
 * --------------------------------------------------------------------------
 * DECISION BODY
 * --------------------------------------------------------------------------
 */
decisionItem
    : decisionCandidate
    | decisionCriterion
    | decisionCondition
    | decisionOutcome
    | decisionSelection
    | decisionReferenceItem
    | decisionEvidenceItem
    | decisionReasoningItem
    | decisionProvenanceItem
    | decisionPolicyItem
    | decisionContractItem
    | decisionAttributeItem
    | decisionMetadataItem
    | decisionNamedItem
    ;


/*
 * --------------------------------------------------------------------------
 * CANDIDATES
 * --------------------------------------------------------------------------
 *
 * Candidate identity and candidate value remain expressions.
 *
 * This allows candidates to represent values, operations, resources,
 * strategies, actions, symbolic entities, or future domain abstractions.
 */
decisionCandidate
    : CANDIDATE COLON expression SEMICOLON?
    ;


/*
 * --------------------------------------------------------------------------
 * CRITERIA
 * --------------------------------------------------------------------------
 *
 * Criteria are expressions whose semantic meaning is validated later.
 *
 * This deliberately avoids hard-coding:
 *
 *   min
 *   max
 *   accuracy
 *   latency
 *   cost
 *   energy
 *   confidence
 *   utility
 *
 * as universal decision operations.
 *
 * Such concepts can be expressed through the existing expression system or
 * registered semantic operations.
 */
decisionCriterion
    : CRITERION COLON expression SEMICOLON?
    ;


/*
 * --------------------------------------------------------------------------
 * CONDITIONS
 * --------------------------------------------------------------------------
 */
decisionCondition
    : CONDITION COLON expression SEMICOLON?
    ;


/*
 * --------------------------------------------------------------------------
 * OUTCOMES
 * --------------------------------------------------------------------------
 *
 * An outcome may be a value, action, operation, or other semantic expression.
 */
decisionOutcome
    : OUTCOME COLON expression SEMICOLON?
    ;


/*
 * --------------------------------------------------------------------------
 * SELECTION
 * --------------------------------------------------------------------------
 *
 * Selection identifies the mechanism or expression used to choose among
 * candidates.
 */
decisionSelection
    : SELECTION COLON expression SEMICOLON?
    ;


/*
 * --------------------------------------------------------------------------
 * REFERENCES
 * --------------------------------------------------------------------------
 */
decisionReferenceItem
    : REFERENCE COLON decisionReference SEMICOLON?
    ;


decisionReference
    : DECISION LPAREN expression RPAREN
    ;


/*
 * --------------------------------------------------------------------------
 * EVIDENCE
 * --------------------------------------------------------------------------
 *
 * Evidence is delegated to the universal evidence subsystem.
 */
decisionEvidenceItem
    : EVIDENCE COLON evidenceReference SEMICOLON?
    ;


/*
 * --------------------------------------------------------------------------
 * REASONING
 * --------------------------------------------------------------------------
 *
 * Reasoning is referenced rather than duplicated.
 */
decisionReasoningItem
    : REASONING COLON expression SEMICOLON?
    ;


/*
 * --------------------------------------------------------------------------
 * PROVENANCE
 * --------------------------------------------------------------------------
 *
 * Provenance references existing provenance semantics.
 */
decisionProvenanceItem
    : PROVENANCE COLON provenanceReference SEMICOLON?
    ;


/*
 * --------------------------------------------------------------------------
 * POLICY
 * --------------------------------------------------------------------------
 *
 * Policy identity/contents are resolved by the policy subsystem.
 *
 * Expression is deliberately used here so policies can evolve without
 * requiring a grammar rewrite for every policy form.
 */
decisionPolicyItem
    : POLICY COLON expression SEMICOLON?
    ;


/*
 * --------------------------------------------------------------------------
 * CONTRACT
 * --------------------------------------------------------------------------
 *
 * Contract contents remain expressions.
 *
 * The validation subsystem owns the interpretation of:
 *
 *   requires
 *   ensures
 *   invariant
 *   assume
 *   guarantee
 *   property
 *
 * and other contract forms.
 */
decisionContractItem
    : CONTRACT COLON expression SEMICOLON?
    ;


/*
 * --------------------------------------------------------------------------
 * ATTRIBUTES / METADATA
 * --------------------------------------------------------------------------
 */
decisionAttributeItem
    : attributeBlock
    ;


decisionMetadataItem
    : metadataBlock
    ;


/*
 * --------------------------------------------------------------------------
 * OPEN-WORLD EXTENSION POINT
 * --------------------------------------------------------------------------
 *
 * This is intentional.
 *
 * Future decision concepts can be represented by named semantic properties
 * without forcing every new concept to become a new reserved keyword.
 *
 * Example semantic properties can include:
 *
 *   objective
 *   priority
 *   confidence
 *   probability
 *   utility
 *   fallback
 *   ranking
 *   evidence_weight
 *   resource_preference
 *   execution_strategy
 *
 * The parser remains stable while the semantic registry evolves.
 */
decisionNamedItem
    : decisionFieldName COLON expression SEMICOLON?
    ;


decisionFieldName
    : identifier
    | qualifiedName
    ;


/*
 * --------------------------------------------------------------------------
 * GENERIC DECISION STATEMENT
 * --------------------------------------------------------------------------
 *
 * Provides a controlled extensibility boundary for decision operations.
 *
 * Operation names are semantic identifiers rather than a finite grammar
 * enumeration.
 */
decisionStatement
    : DECISION decisionOperation decisionArgumentList? SEMICOLON
    ;


decisionOperation
    : qualifiedName
    ;


decisionArgumentList
    : LPAREN decisionArgument (COMMA decisionArgument)* RPAREN
    ;


decisionArgument
    : decisionNamedArgument
    | expression
    ;


decisionNamedArgument
    : identifier ASSIGN expression
    ;


/*
 * --------------------------------------------------------------------------
 * GENERIC DECISION EXPRESSION
 * --------------------------------------------------------------------------
 */
decisionExpression
    : DECISION decisionOperation decisionArgumentList?
    ;
}
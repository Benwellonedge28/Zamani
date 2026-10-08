/*
 * ============================================================================
 * ZAMANI UNIVERSAL PROGRAMMING LANGUAGE
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/networking/protocols.g4
 *
 * GRAMMAR
 * -------
 * Protocols
 *
 * STATUS
 * ------
 * Production networking protocol grammar.
 *
 * IMPLEMENTATION BASELINE
 * -----------------------
 * Rust 1.97+
 * Rust 2021
 * ANTLR4
 * Safe Rust only.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the canonical SOURCE-LEVEL grammar for logical communication
 * protocol contracts.
 *
 * A protocol expresses communication intent and behavioral contracts without
 * committing the program to a physical:
 *
 *     machine
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     QPU
 *     NIC
 *     socket
 *     address
 *     port
 *     transport
 *     provider
 *     node
 *     topology
 *
 * A protocol may express:
 *
 *     identity
 *     refinement
 *     composition
 *     logical roles
 *     message references
 *     operation contracts
 *     requirements
 *     capabilities
 *     constraints
 *     preferences
 *     properties
 *     effects
 *     policies
 *     contracts
 *     provenance
 *     evidence
 *     security obligations
 *     state machines
 *     transitions
 *     interaction flows
 *     sequencing
 *     choice
 *     parallel interaction
 *     repetition
 *     optional interaction
 *     waiting
 *
 * This grammar describes WHAT the communication contract means.
 *
 * It does not implement communication.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     protocol declaration syntax
 *     protocol identity syntax
 *     protocol refinement syntax
 *     protocol composition references
 *     logical protocol roles
 *     protocol message references
 *     protocol operation contracts
 *     protocol-local requirements
 *     protocol-local capability references
 *     protocol-local constraints
 *     protocol-local preferences
 *     protocol properties
 *     protocol-local contract clauses
 *     protocol-local effect references
 *     protocol-local policy references
 *     protocol provenance references
 *     protocol evidence references
 *     protocol state declarations
 *     protocol transitions
 *     protocol interaction flows
 *     protocol flow composition
 *
 * THIS FILE DOES NOT OWN:
 *
 *     identifiers
 *     qualified names
 *     types
 *     expressions
 *     attributes
 *     modules
 *     message schemas
 *     endpoints
 *     addresses
 *     channels
 *     services
 *     requests
 *     responses
 *     sockets
 *     streams
 *     routes
 *     service discovery
 *     network capability definitions
 *     transport implementations
 *     serialization
 *     compression
 *     cryptography
 *     authentication implementation
 *     authorization implementation
 *     identity implementation
 *     routing implementation
 *     scheduling
 *     placement
 *     distributed execution
 *     hardware discovery
 *     resource allocation
 *     runtime queues
 *     runtime communication
 *     quantum operations
 *     quantum states
 *     quantum topology
 *     QEC
 *     ZQN
 *     HAL
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     ZamaniLexer
 *     Names
 *     Types
 *     Expressions
 *     Attributes
 *
 * CONSUMED_BY:
 *
 *     grammar/networking/networking.g4
 *
 * PUBLIC ENTRY:
 *
 *     protocolConstruct
 *
 * PUBLIC DECLARATION:
 *
 *     protocolDeclaration
 *
 * AST_OWNER:
 *
 *     frontend domain-neutral AST implementation.
 *
 * SEMANTIC_OWNER:
 *
 *     networking semantic layer, with shared validation/resource/effect/
 *     capability/policy/provenance/security semantics.
 *
 * IR_OWNER:
 *
 *     canonical semantic representation and downstream canonical IR.
 *
 *     Quantum semantics MUST continue through quantum::ir.
 *
 * TEST_OWNER:
 *
 *     grammar/tests/networking/
 *     grammar/tests/parser/
 *     grammar/tests/semantic/
 *     grammar/tests/scalability/
 *     grammar/tests/portability/
 *
 * SPEC_OWNER:
 *
 *     grammar/spec/networking.md
 *     grammar/spec/resources.md
 *     grammar/spec/effects.md
 *     grammar/spec/policies.md
 *     grammar/spec/provenance.md
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     ZamaniParser
 *          |
 *          v
 *     Networking
 *          |
 *          v
 *     Protocols
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     structural validation
 *          |
 *          +--> type analysis
 *          +--> effect analysis
 *          +--> capability analysis
 *          +--> resource analysis
 *          +--> contract analysis
 *          +--> policy analysis
 *          +--> provenance analysis
 *          +--> security analysis
 *          |
 *          v
 *     networking semantic model
 *          |
 *          +--> distributed semantics
 *          +--> classical semantics
 *          +--> hybrid semantics
 *          +--> hardware communication intent
 *          +--> quantum metadata where applicable
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> classical IR
 *          +--> quantum::ir
 *          +--> HDL/hardware representation
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     lowering
 *          |
 *          v
 *     routing
 *          |
 *          v
 *     scheduling
 *          |
 *          v
 *     resilience
 *          |
 *          v
 *     ZQN
 *          |
 *          v
 *     HAL
 *          |
 *          v
 *     target realization
 *
 * THIS FILE CREATES NO IR.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * A protocol remains source-portable because this grammar does not encode
 * physical realization.
 *
 * The same protocol source may ultimately be realized on:
 *
 *     tiny embedded systems
 *     single CPUs
 *     multicore CPUs
 *     GPUs
 *     FPGAs
 *     ASICs
 *     accelerators
 *     QPUs
 *     simulators
 *     HPC systems
 *     clusters
 *     distributed systems
 *     edge systems
 *     cloud systems
 *     future computational substrates
 *
 * Physical feasibility is a semantic/resource/capability question.
 *
 * It is NOT a grammar question.
 *
 * ============================================================================
 * OPEN-WORLD PROTOCOL MODEL
 * ============================================================================
 *
 * This grammar deliberately does NOT enumerate:
 *
 *     TCP
 *     UDP
 *     QUIC
 *     HTTP
 *     HTTPS
 *     MQTT
 *     gRPC
 *     MPI
 *     RDMA
 *     InfiniBand
 *     Ethernet
 *     Bluetooth
 *     Wi-Fi
 *     vendor transports
 *     cloud transports
 *     future transports
 *
 * A protocol technology is represented by:
 *
 *     qualified names
 *     capabilities
 *     dialects
 *     libraries
 *     semantic registries
 *     target descriptions
 *
 * Adding a future protocol must therefore NOT require modification of this
 * universal grammar merely because the protocol has a new name.
 *
 * ============================================================================
 * LEXICAL AUTHORITY
 * ============================================================================
 *
 * The sole public lexer boundary is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * This grammar uses:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * No lexer rules exist here.
 *
 * IMPORTANT CURRENT TOKEN INTEGRATION
 * -----------------------------------
 *
 * The canonical lexer already reserves:
 *
 *     MESSAGE
 *     PARALLEL
 *     OPTIONAL
 *     PROPERTY
 *     REQUIRES
 *     ENSURES
 *     INVARIANT
 *     ASSUME
 *     GUARANTEE
 *     CAPABILITY
 *     CONSTRAINT
 *     PREFER
 *     EFFECT
 *     POLICY
 *     PROVENANCE
 *     EVIDENCE
 *     EXPLAIN
 *     SANDBOX
 *     SIMULATE
 *
 * This grammar therefore MUST consume those canonical tokens rather than
 * pretending they are identifiers.
 *
 * Other protocol-specific words such as:
 *
 *     protocol
 *     role
 *     operation
 *     uses
 *     provides
 *     state
 *     transition
 *     flow
 *     sequence
 *     choice
 *     repeat
 *     send
 *     receive
 *     emit
 *     wait
 *
 * remain contextual identifiers because they are not currently canonical
 * keyword tokens.
 *
 * If one of those words is promoted to a reserved lexical token in a future
 * language version, that change belongs to the lexical authority and the
 * compatibility/migration specification.
 *
 * ============================================================================
 * TOKEN-NAME CORRECTION
 * ============================================================================
 *
 * The canonical lexer defines:
 *
 *     LESS
 *     GREATER
 *     SEMICOLON
 *
 * NOT:
 *
 *     LESS_THAN
 *     GREATER_THAN
 *     SEMI
 *
 * This grammar therefore uses only the canonical names.
 *
 * ============================================================================
 * IMPORTS
 * ============================================================================
 */

parser grammar Protocols;

options {
    tokenVocab = ZamaniLexer;
}

import
    Names,
    Types,
    Expressions,
    Attributes
;


/*
 * ============================================================================
 * PUBLIC COMPOSITION ENTRY POINT
 * ============================================================================
 *
 * networking.g4 consumes protocolConstruct.
 *
 * No other grammar should copy protocolDeclaration when protocolConstruct is
 * sufficient.
 * ============================================================================
 */

protocolConstruct
    : protocolDeclaration
    | protocolReference
    ;


/*
 * ============================================================================
 * CONTEXTUAL MARKERS
 * ============================================================================
 *
 * These markers are intentionally identifier-based because their words are not
 * currently canonical lexer tokens.
 *
 * They do not create a second lexical vocabulary.
 *
 * Semantic validation must verify that the identifier has the required
 * contextual spelling.
 * ============================================================================
 */

protocolMarker
    : identifier
    ;

roleMarker
    : identifier
    ;

operationMarker
    : identifier
    ;

usesMarker
    : identifier
    ;

providesMarker
    : identifier
    ;

stateMarker
    : identifier
    ;

transitionMarker
    : identifier
    ;

onMarker
    : identifier
    ;

flowMarker
    : identifier
    ;

sequenceMarker
    : identifier
    ;

choiceMarker
    : identifier
    ;

repeatMarker
    : identifier
    ;

sendMarker
    : identifier
    ;

receiveMarker
    : identifier
    ;

emitMarker
    : identifier
    ;

waitMarker
    : identifier
    ;


/*
 * ============================================================================
 * PROTOCOL DECLARATION
 * ============================================================================
 *
 * Supported forms:
 *
 *     protocol Reliable;
 *
 *     protocol Reliable {}
 *
 *     protocol Secure extends Reliable {}
 *
 *     protocol Transport<T: Message> {}
 *
 * Attributes remain owned by Attributes.
 * ============================================================================
 */

protocolDeclaration
    : attribute*
      protocolMarker
      identifier
      protocolTypeParameterList?
      protocolInheritanceClause?
      protocolWhereClause?
      protocolBody?
      SEMICOLON?
    ;


/*
 * ============================================================================
 * GENERIC PARAMETERS
 * ============================================================================
 */

protocolTypeParameterList
    : LESS
      protocolTypeParameter
      (COMMA protocolTypeParameter)*
      COMMA?
      GREATER
    ;

protocolTypeParameter
    : identifier
      protocolTypeParameterBound?
    ;

protocolTypeParameterBound
    : COLON
      typeExpression
    ;


/*
 * ============================================================================
 * REFINEMENT / INHERITANCE
 * ============================================================================
 *
 * EXTENDS is canonical.
 *
 * Refinement is semantic composition and does not imply implementation
 * inheritance.
 * ============================================================================
 */

protocolInheritanceClause
    : EXTENDS
      qualifiedName
      (COMMA qualifiedName)*
    ;


/*
 * ============================================================================
 * WHERE CLAUSE
 * ============================================================================
 */

protocolWhereClause
    : WHERE
      expression
    ;


/*
 * ============================================================================
 * PROTOCOL BODY
 * ============================================================================
 */

protocolBody
    : LBRACE
      protocolMember*
      RBRACE
    ;


/*
 * ============================================================================
 * PROTOCOL MEMBER DISPATCH
 * ============================================================================
 */

protocolMember
    : protocolRoleDeclaration
    | protocolMessageReference
    | protocolOperationDeclaration
    | protocolUseDeclaration
    | protocolProvideDeclaration
    | protocolRequirementDeclaration
    | protocolCapabilityDeclaration
    | protocolConstraintDeclaration
    | protocolPreferenceDeclaration
    | protocolPropertyDeclaration
    | protocolContractDeclaration
    | protocolEffectDeclaration
    | protocolPolicyDeclaration
    | protocolEvidenceDeclaration
    | protocolProvenanceDeclaration
    | protocolExplanationDeclaration
    | protocolStateDeclaration
    | protocolTransitionDeclaration
    | protocolFlowDeclaration
    | protocolNestedDeclaration
    ;


/*
 * ============================================================================
 * LOGICAL ROLE
 * ============================================================================
 *
 * A role is a logical participant.
 *
 * It is not a physical node, process, machine, device, socket, address,
 * thread, CPU, GPU, FPGA, ASIC or QPU.
 * ============================================================================
 */

protocolRoleDeclaration
    : roleMarker
      identifier
      protocolRoleTypeClause?
      SEMICOLON
    ;

protocolRoleTypeClause
    : COLON
      qualifiedName
    ;


/*
 * ============================================================================
 * MESSAGE REFERENCE
 * ============================================================================
 *
 * MESSAGE is a canonical lexer token.
 *
 * Message schemas remain owned by:
 *
 *     grammar/networking/messages.g4
 *
 * This rule only references them.
 * ============================================================================
 */

protocolMessageReference
    : MESSAGE
      qualifiedName
      protocolMessageDirectionClause?
      SEMICOLON
    ;

protocolMessageDirectionClause
    : COLON
      expression
    ;


/*
 * ============================================================================
 * OPERATION CONTRACT
 * ============================================================================
 */

protocolOperationDeclaration
    : operationMarker
      identifier
      protocolTypeParameterList?
      LPAREN
      protocolParameterList?
      RPAREN
      protocolReturnClause?
      protocolOperationClause*
      protocolWhereClause?
      SEMICOLON
    ;

protocolParameterList
    : protocolParameter
      (COMMA protocolParameter)*
      COMMA?
    ;

protocolParameter
    : identifier
      COLON
      typeExpression
      protocolParameterDefault?
    ;

protocolParameterDefault
    : ASSIGN
      expression
    ;

protocolReturnClause
    : THIN_ARROW
      typeExpression
    ;


/*
 * ============================================================================
 * OPERATION CONTRACT CLAUSES
 * ============================================================================
 */

protocolOperationClause
    : protocolRequiresClause
    | protocolEnsuresClause
    | protocolInvariantClause
    | protocolAssumeClause
    | protocolGuaranteeClause
    | protocolUsesClause
    | protocolProvidesClause
    | protocolCapabilityClause
    | protocolConstraintClause
    | protocolPreferenceClause
    | protocolEffectClause
    | protocolPolicyClause
    | protocolEvidenceClause
    | protocolProvenanceClause
    ;


/*
 * ============================================================================
 * REQUIREMENTS
 * ============================================================================
 */

protocolRequiresClause
    : REQUIRES
      expression
    ;

protocolRequirementDeclaration
    : REQUIRES
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * ENSURES
 * ============================================================================
 */

protocolEnsuresClause
    : ENSURES
      expression
    ;


/*
 * ============================================================================
 * INVARIANT
 * ============================================================================
 */

protocolInvariantClause
    : INVARIANT
      expression
    ;


/*
 * ============================================================================
 * ASSUMPTION
 * ============================================================================
 */

protocolAssumeClause
    : ASSUME
      expression
    ;


/*
 * ============================================================================
 * GUARANTEE
 * ============================================================================
 */

protocolGuaranteeClause
    : GUARANTEE
      expression
    ;


/*
 * ============================================================================
 * USES
 * ============================================================================
 *
 * Uses remains contextual.
 * ============================================================================
 */

protocolUsesClause
    : usesMarker
      expression
    ;

protocolUseDeclaration
    : usesMarker
      qualifiedName
      SEMICOLON
    ;


/*
 * ============================================================================
 * PROVIDES
 * ============================================================================
 */

protocolProvidesClause
    : providesMarker
      expression
    ;

protocolProvideDeclaration
    : providesMarker
      qualifiedName
      SEMICOLON
    ;


/*
 * ============================================================================
 * CAPABILITIES
 * ============================================================================
 *
 * CAPABILITY is canonical.
 *
 * Capability discovery and negotiation occur downstream.
 * ============================================================================
 */

protocolCapabilityClause
    : CAPABILITY
      qualifiedName
    ;

protocolCapabilityDeclaration
    : CAPABILITY
      qualifiedName
      protocolCapabilityValue?
      SEMICOLON
    ;

protocolCapabilityValue
    : ASSIGN
      expression
    ;


/*
 * ============================================================================
 * CONSTRAINTS
 * ============================================================================
 */

protocolConstraintClause
    : CONSTRAINT
      expression
    ;

protocolConstraintDeclaration
    : CONSTRAINT
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * PREFERENCES
 * ============================================================================
 */

protocolPreferenceClause
    : PREFER
      expression
    ;

protocolPreferenceDeclaration
    : PREFER
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * PROPERTIES
 * ============================================================================
 */

protocolPropertyDeclaration
    : PROPERTY
      identifier
      protocolPropertyTypeClause?
      protocolPropertyInitializer?
      SEMICOLON
    ;

protocolPropertyTypeClause
    : COLON
      typeExpression
    ;

protocolPropertyInitializer
    : ASSIGN
      expression
    ;


/*
 * ============================================================================
 * CONTRACT BLOCK
 * ============================================================================
 *
 * This provides a protocol-level contract boundary while leaving the actual
 * contract semantics to validation.
 *
 * Example:
 *
 *     contract {
 *         requires capability("network.integrity");
 *         ensures delivery_complete;
 *         invariant session_valid;
 *         guarantee response_available;
 *     }
 * ============================================================================
 */

protocolContractDeclaration
    : CONTRACT
      LBRACE
      protocolContractMember*
      RBRACE
      SEMICOLON?
    ;

protocolContractMember
    : protocolRequiresClause SEMICOLON
    | protocolEnsuresClause SEMICOLON
    | protocolInvariantClause SEMICOLON
    | protocolAssumeClause SEMICOLON
    | protocolGuaranteeClause SEMICOLON
    | protocolEvidenceClause SEMICOLON
    | protocolProvenanceClause SEMICOLON
    ;


/*
 * ============================================================================
 * EFFECT DECLARATION
 * ============================================================================
 *
 * EFFECT is canonical.
 *
 * Effect identity remains open-world through qualified names.
 *
 * This does not duplicate grammar/effects/.
 * ============================================================================
 */

protocolEffectDeclaration
    : EFFECT
      protocolEffectReferenceList
      SEMICOLON
    ;

protocolEffectReferenceList
    : qualifiedName
      (COMMA qualifiedName)*
      COMMA?
    ;

protocolEffectClause
    : EFFECT
      protocolEffectReferenceList
    ;


/*
 * ============================================================================
 * POLICY DECLARATION
 * ============================================================================
 *
 * POLICY is canonical.
 *
 * The policy subsystem owns policy semantics.
 *
 * This file only establishes a protocol-local reference boundary.
 * ============================================================================
 */

protocolPolicyDeclaration
    : POLICY
      expression
      SEMICOLON
    ;

protocolPolicyClause
    : POLICY
      expression
    ;


/*
 * ============================================================================
 * EVIDENCE
 * ============================================================================
 *
 * Evidence remains an open semantic reference.
 *
 * The evidence subsystem owns evidence semantics.
 * ============================================================================
 */

protocolEvidenceDeclaration
    : EVIDENCE
      expression
      SEMICOLON
    ;

protocolEvidenceClause
    : EVIDENCE
      expression
    ;


/*
 * ============================================================================
 * PROVENANCE
 * ============================================================================
 *
 * Provenance is preserved as semantic metadata/reference information.
 * ============================================================================
 */

protocolProvenanceDeclaration
    : PROVENANCE
      expression
      SEMICOLON
    ;

protocolProvenanceClause
    : PROVENANCE
      expression
    ;


/*
 * ============================================================================
 * EXPLANATION
 * ============================================================================
 *
 * Explanation is an explicit semantic request.
 *
 * It does not make the networking grammar responsible for explanation
 * generation.
 * ============================================================================
 */

protocolExplanationDeclaration
    : EXPLAIN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * PROTOCOL STATES
 * ============================================================================
 *
 * State identifiers describe the abstract protocol state machine.
 *
 * They are not runtime process states.
 * ============================================================================
 */

protocolStateDeclaration
    : stateMarker
      identifier
      SEMICOLON
    ;


/*
 * ============================================================================
 * PROTOCOL TRANSITIONS
 * ============================================================================
 */

protocolTransitionDeclaration
    : transitionMarker
      identifier
      THIN_ARROW
      identifier
      protocolTransitionEventClause?
      protocolTransitionConditionClause*
      SEMICOLON
    ;

protocolTransitionEventClause
    : onMarker
      expression
    ;

protocolTransitionConditionClause
    : REQUIRES
      expression
    ;


/*
 * ============================================================================
 * FLOW DECLARATION
 * ============================================================================
 *
 * A flow describes logical communication interaction.
 *
 * It does not select a transport.
 * ============================================================================
 */

protocolFlowDeclaration
    : flowMarker
      identifier?
      LBRACE
      protocolFlowStep*
      RBRACE
      SEMICOLON?
    ;


/*
 * ============================================================================
 * FLOW STEP DISPATCH
 * ============================================================================
 */

protocolFlowStep
    : protocolSendStep
    | protocolReceiveStep
    | protocolEmitStep
    | protocolFlowCallStep
    | protocolSequenceStep
    | protocolChoiceStep
    | protocolParallelStep
    | protocolRepeatStep
    | protocolOptionalStep
    | protocolWaitStep
    | protocolFlowContractStep
    | protocolFlowExpressionStep
    ;


/*
 * ============================================================================
 * SEND
 * ============================================================================
 *
 * Send is logical interaction intent only.
 * ============================================================================
 */

protocolSendStep
    : sendMarker
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * RECEIVE
 * ============================================================================
 */

protocolReceiveStep
    : receiveMarker
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * EMIT
 * ============================================================================
 */

protocolEmitStep
    : emitMarker
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * FLOW CALL
 * ============================================================================
 */

protocolFlowCallStep
    : qualifiedName
      LPAREN
      protocolFlowArgumentList?
      RPAREN
      SEMICOLON
    ;

protocolFlowArgumentList
    : expression
      (COMMA expression)*
      COMMA?
    ;


/*
 * ============================================================================
 * SEQUENCE
 * ============================================================================
 */

protocolSequenceStep
    : sequenceMarker
      LBRACE
      protocolFlowStep*
      RBRACE
      SEMICOLON?
    ;


/*
 * ============================================================================
 * CHOICE
 * ============================================================================
 */

protocolChoiceStep
    : choiceMarker
      LBRACE
      protocolChoiceBranch+
      RBRACE
      SEMICOLON?
    ;

protocolChoiceBranch
    : protocolFlowStep+
    ;


/*
 * ============================================================================
 * PARALLEL
 * ============================================================================
 *
 * PARALLEL is already a canonical lexer token.
 *
 * The runtime/compiler decides whether the logical parallel interaction is
 * realized as:
 *
 *     concurrent
 *     pipelined
 *     multiplexed
 *     serialized
 *     distributed
 *
 * according to semantic constraints and target capabilities.
 * ============================================================================
 */

protocolParallelStep
    : PARALLEL
      LBRACE
      protocolFlowStep*
      RBRACE
      SEMICOLON?
    ;


/*
 * ============================================================================
 * REPEAT
 * ============================================================================
 */

protocolRepeatStep
    : repeatMarker
      protocolRepeatCondition?
      LBRACE
      protocolFlowStep*
      RBRACE
      SEMICOLON?
    ;

protocolRepeatCondition
    : expression
    ;


/*
 * ============================================================================
 * OPTIONAL
 * ============================================================================
 *
 * OPTIONAL is already a canonical lexer token.
 * ============================================================================
 */

protocolOptionalStep
    : OPTIONAL
      LBRACE
      protocolFlowStep*
      RBRACE
      SEMICOLON?
    ;


/*
 * ============================================================================
 * WAIT
 * ============================================================================
 *
 * Waiting is logical protocol behavior.
 *
 * Timing interpretation remains semantic/runtime behavior.
 * ============================================================================
 */

protocolWaitStep
    : waitMarker
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * FLOW CONTRACT
 * ============================================================================
 *
 * Contracts may appear inside interaction flows.
 * ============================================================================
 */

protocolFlowContractStep
    : REQUIRES expression SEMICOLON
    | ENSURES expression SEMICOLON
    | INVARIANT expression SEMICOLON
    | ASSUME expression SEMICOLON
    | GUARANTEE expression SEMICOLON
    ;


/*
 * ============================================================================
 * GENERIC FLOW EXPRESSION
 * ============================================================================
 *
 * This is the forward-compatible escape hatch for protocol behavior that can
 * already be represented by the ordinary expression grammar.
 *
 * It does NOT introduce a second expression language.
 * ============================================================================
 */

protocolFlowExpressionStep
    : expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * NESTED PROTOCOL
 * ============================================================================
 *
 * Nested protocols are namespace/scoping constructs.
 *
 * Their legality is semantic.
 * ============================================================================
 */

protocolNestedDeclaration
    : protocolDeclaration
    ;


/*
 * ============================================================================
 * PROTOCOL REFERENCE
 * ============================================================================
 */

protocolReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * REFERENCE LISTS
 * ============================================================================
 */

protocolReferenceList
    : protocolReference
      (COMMA protocolReference)*
      COMMA?
    ;

protocolRoleReference
    : qualifiedName
    ;

protocolRoleReferenceList
    : protocolRoleReference
      (COMMA protocolRoleReference)*
      COMMA?
    ;

protocolMessageReferenceList
    : qualifiedName
      (COMMA qualifiedName)*
      COMMA?
    ;

protocolOperationReference
    : qualifiedName
    ;

protocolOperationReferenceList
    : protocolOperationReference
      (COMMA protocolOperationReference)*
      COMMA?
    ;


/*
 * ============================================================================
 * REUSABLE EXPRESSION ADAPTERS
 * ============================================================================
 *
 * These rules do not create another expression language.
 * ============================================================================
 */

protocolValue
    : expression
    ;

protocolCondition
    : expression
    ;

protocolPredicate
    : expression
    ;


/*
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser must expose enough structure for the domain-neutral AST to
 * preserve:
 *
 *     declaration attributes
 *     protocol name
 *     generic parameters
 *     refinement parents
 *     where clause
 *     roles
 *     message references
 *     operation signatures
 *     operation contracts
 *     capabilities
 *     requirements
 *     constraints
 *     preferences
 *     properties
 *     effects
 *     policies
 *     evidence
 *     provenance
 *     explanations
 *     states
 *     transitions
 *     flows
 *     flow steps
 *     expressions
 *     source spans
 *
 * The parser MUST NOT resolve any of these semantically.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis owns:
 *
 *     protocol name resolution
 *     role resolution
 *     message resolution
 *     operation resolution
 *     type checking
 *     generic-bound checking
 *     refinement compatibility
 *     protocol state validation
 *     transition validation
 *     flow validation
 *     send/receive compatibility
 *     message compatibility
 *     request/response compatibility
 *     endpoint compatibility
 *     channel compatibility
 *     service compatibility
 *     effect checking
 *     capability checking
 *     resource checking
 *     contract checking
 *     policy checking
 *     provenance validation
 *     evidence validation
 *     security validation
 *     distributed compatibility
 *     target-independent portability validation
 *
 * The parser MUST NOT perform these operations.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Protocol requirements may express target-independent intent such as:
 *
 *     requires capability("network.reliable");
 *     requires capability("network.integrity");
 *     requires bandwidth >= required_bandwidth;
 *     requires latency <= acceptable_latency;
 *     requires topology(required_topology);
 *
 * They may also be combined with:
 *
 *     constraint ...
 *     prefer ...
 *
 * These constructs describe requirements and preferences.
 *
 * They do NOT allocate resources.
 *
 * Resource resolution occurs downstream:
 *
 *     requirement
 *          ->
 *     capability analysis
 *          ->
 *     resource analysis
 *          ->
 *     negotiation
 *          ->
 *     execution planning
 *          ->
 *     target realization
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Protocols may carry effects such as:
 *
 *     network
 *     distributed
 *     foreign
 *     native
 *     randomness
 *     measurement
 *     simulation
 *
 * and future effect namespaces.
 *
 * This file does not enumerate a closed effect catalogue.
 *
 * Effect semantics remain owned by grammar/effects/ and the semantic effect
 * system.
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Policies may constrain:
 *
 *     communication
 *     transport selection
 *     security
 *     resource realization
 *     routing
 *     deployment
 *     adaptation
 *     fallback
 *     simulation
 *
 * Policy meaning is not implemented here.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * Parsing a protocol MUST NOT:
 *
 *     open a socket
 *     connect to a network
 *     resolve DNS
 *     access credentials
 *     inspect environment state
 *     access files
 *     execute commands
 *     discover devices
 *     establish trust
 *     authorize communication
 *
 * A parsed protocol does not grant any capability.
 *
 * Security and authorization are evaluated downstream.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Protocol semantics may preserve:
 *
 *     source
 *     derived_from
 *     generated_by
 *     transformed_by
 *     verified_by
 *     evidence
 *     decision
 *     version
 *
 * The grammar preserves structure.
 *
 * The semantic/provenance subsystem records actual provenance.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Protocols may coordinate:
 *
 *     quantum services
 *     remote quantum computation
 *     distributed quantum computation
 *     quantum-classical interaction
 *     measurement result transport
 *     distributed error-correction workflows
 *
 * This grammar does NOT define:
 *
 *     qubit identifiers
 *     gates
 *     quantum states
 *     physical qubits
 *     logical qubits
 *     pulse schedules
 *     calibration
 *     coupling maps
 *     QEC implementation
 *
 * If protocol semantics contain quantum computation:
 *
 *     protocol AST
 *          ->
 *     semantic quantum model
 *          ->
 *     quantum::ir
 *
 * There is no protocol-specific quantum IR.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Protocols may describe logical interfaces used by:
 *
 *     embedded systems
 *     FPGA designs
 *     ASIC designs
 *     accelerator systems
 *     hardware/software co-design
 *
 * They do not define:
 *
 *     wires
 *     pins
 *     registers
 *     clocks
 *     physical buses
 *     physical routing
 *     FPGA placement
 *     ASIC placement
 *
 * Those belong to grammar/hdl/ and grammar/hardware/.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Networking and distributed execution remain separate semantic concerns.
 *
 * Protocols describe communication contracts.
 *
 * Distributed execution owns:
 *
 *     nodes
 *     placement
 *     replication
 *     consistency
 *     distributed scheduling
 *     distributed recovery
 *
 * No node count is encoded here.
 *
 * ============================================================================
 * ENDPOINT / CHANNEL / SERVICE INTEGRATION
 * ============================================================================
 *
 * Endpoints are owned by:
 *
 *     grammar/networking/endpoints.g4
 *
 * Channels are owned by:
 *
 *     grammar/networking/channels.g4
 *
 * Services are owned by:
 *
 *     grammar/networking/services.g4
 *
 * Messages are owned by:
 *
 *     grammar/networking/messages.g4
 *
 * Requests are owned by:
 *
 *     grammar/networking/requests.g4
 *
 * Responses are owned by:
 *
 *     grammar/networking/responses.g4
 *
 * Routes are owned by:
 *
 *     grammar/networking/routing.g4
 *
 * Discovery is owned by:
 *
 *     grammar/networking/service-discovery.g4
 *
 * Sockets are owned by:
 *
 *     grammar/networking/sockets.g4
 *
 * Streaming is owned by:
 *
 *     grammar/networking/streaming.g4
 *
 * Network capabilities are owned by:
 *
 *     grammar/networking/network-capabilities.g4
 *
 * Protocols reference these semantic entities.
 *
 * They do not duplicate their syntax.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * No artificial language-level limits exist for:
 *
 *     protocol declarations
 *     roles
 *     messages
 *     operations
 *     states
 *     transitions
 *     flow steps
 *     requirements
 *     capabilities
 *     constraints
 *     preferences
 *     policies
 *     effects
 *     evidence
 *     provenance records
 *     generic parameters
 *     qualified-name depth
 *     nesting depth
 *     source program size
 *
 * Forbidden protocol capacity constants include:
 *
 *     MAX_PROTOCOLS
 *     MAX_ROLES
 *     MAX_MESSAGES
 *     MAX_OPERATIONS
 *     MAX_STATES
 *     MAX_TRANSITIONS
 *     MAX_PARTICIPANTS
 *     MAX_ENDPOINTS
 *     MAX_CHANNELS
 *     MAX_CONNECTIONS
 *     MAX_NODES
 *     MAX_NETWORK_SIZE
 *     MAX_BANDWIDTH
 *     MAX_LATENCY
 *     MAX_MESSAGE_SIZE
 *
 * No such constants are present in this grammar.
 *
 * Practical limits are determined by:
 *
 *     available memory
 *     parser implementation
 *     compiler implementation
 *     runtime resources
 *     operating-system resources
 *     declared resource requirements
 *     target capabilities
 *
 * Those limits do not define the language.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing must depend only on:
 *
 *     source
 *     canonical token stream
 *     selected grammar
 *     explicit language configuration
 *
 * Parsing must not depend on:
 *
 *     network state
 *     DNS
 *     filesystem state
 *     hardware state
 *     target availability
 *     runtime state
 *     wall-clock time
 *     randomness
 *     scheduler state
 *
 * Identical input under identical grammar configuration must produce equivalent
 * parse structure.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates no IR.
 *
 * Protocol semantics may lower into:
 *
 *     canonical networking semantic representation
 *     distributed semantic representation
 *     classical IR
 *     hardware communication intent
 *     deployment representation
 *
 * Quantum semantics continue through:
 *
 *     quantum::ir
 *
 * No protocol-specific quantum IR is permitted.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime responsibilities include:
 *
 *     endpoint binding
 *     channel establishment
 *     transport selection
 *     serialization
 *     routing
 *     scheduling
 *     resource allocation
 *     security enforcement
 *     retries
 *     recovery
 *     migration
 *     failover
 *     monitoring
 *
 * None of these occur in this grammar.
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser diagnostics identify malformed syntax:
 *
 *     missing protocol name
 *     malformed generic parameters
 *     malformed inheritance
 *     malformed role
 *     malformed message reference
 *     malformed operation
 *     malformed type
 *     malformed contract
 *     malformed flow
 *     malformed transition
 *     malformed choice
 *     malformed parallel block
 *     malformed repeat block
 *     malformed optional block
 *     malformed expression
 *
 * Semantic diagnostics identify:
 *
 *     unknown protocol
 *     duplicate protocol
 *     unknown role
 *     unknown message
 *     unknown operation
 *     invalid state
 *     invalid transition
 *     incompatible message direction
 *     incompatible operation contract
 *     unsatisfied capability
 *     unsatisfied resource requirement
 *     violated constraint
 *     forbidden policy
 *     invalid effect
 *     invalid provenance
 *     invalid security requirement
 *
 * Syntax and semantic diagnostics MUST remain separate.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing conceptual source forms remain supported where their meaning is
 * unchanged:
 *
 *     protocol Reliable;
 *
 *     protocol Reliable {
 *         role sender;
 *         role receiver;
 *     }
 *
 *     protocol Reliable {
 *         message transport::Request;
 *         message transport::Response;
 *     }
 *
 *     protocol Session {
 *         state idle;
 *         state established;
 *         transition idle -> established;
 *     }
 *
 * The canonical token corrections in this version are implementation fixes:
 *
 *     SEMI
 *         -> SEMICOLON
 *
 *     LESS_THAN
 *         -> LESS
 *
 *     GREATER_THAN
 *         -> GREATER
 *
 * Existing semantic intent is unchanged.
 *
 * ============================================================================
 * POSITIVE CONFORMANCE EXAMPLES
 * ============================================================================
 *
 * Minimal:
 *
 *     protocol Reliable;
 *
 * Roles:
 *
 *     protocol Reliable {
 *         role sender;
 *         role receiver;
 *     }
 *
 * Messages:
 *
 *     protocol Reliable {
 *         message transport::Request;
 *         message transport::Response;
 *     }
 *
 * Operation:
 *
 *     protocol Reliable {
 *         operation send(value: Message) -> Result;
 *     }
 *
 * Generic:
 *
 *     protocol Transport<T: Message> {
 *         operation transmit(value: T) -> Result;
 *     }
 *
 * Refinement:
 *
 *     protocol Secure extends Reliable {
 *         requires capability("network.reliable");
 *         capability networking::integrity;
 *         prefer networking::low_latency;
 *     }
 *
 * Contract:
 *
 *     protocol Reliable {
 *         contract {
 *             requires capability("network.delivery");
 *             ensures delivery_complete;
 *             invariant session_valid;
 *             guarantee response_available;
 *         }
 *     }
 *
 * State machine:
 *
 *     protocol Session {
 *         state idle;
 *         state established;
 *         state closed;
 *
 *         transition idle -> established;
 *         transition established -> closed on disconnect;
 *     }
 *
 * Flow:
 *
 *     protocol RequestResponse {
 *         flow request_response {
 *             send request;
 *             receive response;
 *         }
 *     }
 *
 * Parallel flow:
 *
 *     protocol Concurrent {
 *         flow work {
 *             parallel {
 *                 send request_a;
 *                 send request_b;
 *             }
 *         }
 *     }
 *
 * Optional flow:
 *
 *     protocol OptionalTelemetry {
 *         flow telemetry {
 *             optional {
 *                 send telemetry;
 *             }
 *         }
 *     }
 *
 * Requirements and preferences:
 *
 *     protocol Adaptive {
 *         requires capability("network.adaptive");
 *         constraint latency <= required_latency;
 *         prefer locality::near;
 *     }
 *
 * Effects:
 *
 *     protocol Networked {
 *         effect network, distributed;
 *     }
 *
 * Policy:
 *
 *     protocol Restricted {
 *         policy communication_policy;
 *     }
 *
 * ============================================================================
 * NEGATIVE CONFORMANCE EXAMPLES
 * ============================================================================
 *
 * The following must be rejected syntactically:
 *
 *     protocol;
 *
 *     protocol Reliable {
 *
 *     protocol Reliable {
 *         role;
 *     }
 *
 *     protocol Reliable {
 *         message;
 *     }
 *
 *     protocol Reliable {
 *         operation send(
 *     }
 *
 *     protocol Reliable {
 *         transition idle;
 *     }
 *
 *     protocol Reliable {
 *         state;
 *     }
 *
 *     protocol Reliable {
 *         capability;
 *     }
 *
 *     protocol Reliable {
 *         requires;
 *     }
 *
 * ============================================================================
 * BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Test at minimum:
 *
 *     empty protocol body
 *     one role
 *     many roles
 *     one message
 *     many messages
 *     one operation
 *     many operations
 *     generic operations
 *     nested generic types
 *     refinement chains
 *     many requirements
 *     many capabilities
 *     many constraints
 *     many preferences
 *     many effects
 *     contract blocks
 *     state machines
 *     transition events
 *     transition requirements
 *     nested flows
 *     deeply nested sequences
 *     choices
 *     parallel blocks
 *     repetitions
 *     optional blocks
 *     flow expressions
 *     long qualified names
 *     large expressions
 *     large type expressions
 *     mixed networking/domain constructs
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Scalability tests must increase source size and structural complexity
 * without introducing a language-defined ceiling.
 *
 * Test dimensions include:
 *
 *     protocol count
 *     role count
 *     message-reference count
 *     operation count
 *     state count
 *     transition count
 *     flow-step count
 *     nesting depth
 *     qualified-name depth
 *     requirement count
 *     capability count
 *     contract count
 *
 * Tests must report implementation/resource limits separately from language
 * validity.
 *
 * ============================================================================
 * DETERMINISM TEST CONTRACT
 * ============================================================================
 *
 * Parse identical source repeatedly using identical grammar configuration.
 *
 * Verify equivalent:
 *
 *     token sequence
 *     parse structure
 *     source spans
 *     diagnostics
 *
 * The test environment must not influence parsing.
 *
 * ============================================================================
 * CROSS-DOMAIN TEST CONTRACT
 * ============================================================================
 *
 * Protocols must be tested in combination with:
 *
 *     classical computation
 *     quantum computation
 *     hybrid computation
 *     HDL
 *     hardware intent
 *     AI/data computation
 *     concurrency
 *     distributed execution
 *     security
 *     effects
 *     resources
 *     policies
 *     provenance
 *     contracts
 *
 * Cross-domain tests must verify that protocol syntax does not create a
 * competing semantic universe.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * UPSTREAM:
 *
 *     grammar/lexer/*
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/core/*
 *     grammar/types/*
 *     grammar/expressions/*
 *
 * DIRECT IMPORTS:
 *
 *     Names
 *     Types
 *     Expressions
 *     Attributes
 *
 * AGGREGATOR:
 *
 *     grammar/networking/networking.g4
 *
 * ROOT PARSER:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * NETWORKING PEERS:
 *
 *     addresses.g4
 *     endpoints.g4
 *     channels.g4
 *     messages.g4
 *     requests.g4
 *     responses.g4
 *     routing.g4
 *     service-discovery.g4
 *     services.g4
 *     sockets.g4
 *     streaming.g4
 *     distributed-compute.g4
 *     network-capabilities.g4
 *
 * SEMANTIC CONSUMERS:
 *
 *     type analysis
 *     effect analysis
 *     capability analysis
 *     resource analysis
 *     contract validation
 *     policy validation
 *     provenance
 *     security
 *     distributed semantics
 *     networking semantics
 *
 * DOWNSTREAM:
 *
 *     canonical semantic representation
 *     classical IR
 *     quantum::ir
 *     optimization
 *     lowering
 *     routing
 *     scheduling
 *     resilience
 *     ZQN
 *     HAL
 *     target realization
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * No physical protocol catalogue is encoded.
 *
 * No transport catalogue is encoded.
 *
 * No provider catalogue is encoded.
 *
 * No machine capacity is encoded.
 *
 * No node count is encoded.
 *
 * No endpoint count is encoded.
 *
 * No channel count is encoded.
 *
 * No message size ceiling is encoded.
 *
 * No bandwidth ceiling is encoded.
 *
 * No latency ceiling is encoded.
 *
 * No CPU limit is encoded.
 *
 * No GPU limit is encoded.
 *
 * No FPGA limit is encoded.
 *
 * No ASIC limit is encoded.
 *
 * No QPU limit is encoded.
 *
 * No qubit limit is encoded.
 *
 * No memory limit is encoded.
 *
 * No tensor-rank limit is encoded.
 *
 * No network-size limit is encoded.
 *
 * ============================================================================
 * SAFETY AUDIT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no Rust actions
 *     no unsafe code
 *     no semantic predicates
 *     no filesystem access
 *     no network access
 *     no hardware access
 *     no runtime callbacks
 *     no randomness
 *     no environment inspection
 *     no target discovery
 *     no resource discovery
 *
 * Generated Rust remains ordinary safe generated/parser code.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 * [ ] ANTLR accepts the grammar with the repository's canonical lexer.
 *
 * [ ] No undefined token names remain.
 *
 * [ ] No competing lexer rules exist.
 *
 * [ ] No second identifier grammar exists.
 *
 * [ ] No second type grammar exists.
 *
 * [ ] No second expression grammar exists.
 *
 * [ ] Protocol syntax has exactly one owner.
 *
 * [ ] Message schemas remain owned by messages.g4.
 *
 * [ ] Endpoints remain owned by endpoints.g4.
 *
 * [ ] Channels remain owned by channels.g4.
 *
 * [ ] Services remain owned by services.g4.
 *
 * [ ] Routes remain owned by routing.g4.
 *
 * [ ] Discovery remains owned by service-discovery.g4.
 *
 * [ ] Sockets remain owned by sockets.g4.
 *
 * [ ] Streaming remains owned by streaming.g4.
 *
 * [ ] Network capabilities remain owned by network-capabilities.g4.
 *
 * [ ] Distributed execution remains outside this grammar.
 *
 * [ ] Security implementation remains outside this grammar.
 *
 * [ ] Resource allocation remains outside this grammar.
 *
 * [ ] Target selection remains outside this grammar.
 *
 * [ ] No physical networking implementation is encoded.
 *
 * [ ] No finite protocol catalogue is encoded.
 *
 * [ ] Requirements are target-independent.
 *
 * [ ] Capabilities are target-independent references.
 *
 * [ ] Constraints remain semantic constraints.
 *
 * [ ] Preferences remain semantic preferences.
 *
 * [ ] Effects integrate with the universal effect model.
 *
 * [ ] Contracts integrate with validation.
 *
 * [ ] Policies integrate with the universal policy model.
 *
 * [ ] Evidence integrates with evidence/provenance.
 *
 * [ ] Provenance integrates with provenance/audit.
 *
 * [ ] Quantum semantics retain the quantum::ir boundary.
 *
 * [ ] Classical semantics remain compatible with classical IR.
 *
 * [ ] HDL/hardware semantics remain downstream.
 *
 * [ ] Positive parser tests pass.
 *
 * [ ] Negative parser tests pass.
 *
 * [ ] Boundary tests pass.
 *
 * [ ] Scalability tests pass without establishing a language maximum.
 *
 * [ ] Determinism tests pass.
 *
 * [ ] Compatibility tests pass.
 *
 * [ ] Cross-domain tests pass.
 *
 * [ ] Generated Rust requires no unsafe implementation.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * This grammar defines:
 *
 *     WHAT a logical communication protocol is.
 *
 * It does not define:
 *
 *     WHERE it executes.
 *     WHICH machine executes it.
 *     WHICH interface carries it.
 *     WHICH socket realizes it.
 *     WHICH address is selected.
 *     WHICH transport is selected.
 *     WHICH provider is selected.
 *     HOW many nodes exist.
 *     HOW routing is performed.
 *     HOW scheduling is performed.
 *     HOW resources are allocated.
 *     HOW security is implemented.
 *     HOW quantum computation is physically realized.
 *
 * Therefore:
 *
 *     Zamani source
 *          ->
 *     protocol intent
 *          ->
 *     semantic validation
 *          ->
 *     capability/resource negotiation
 *          ->
 *     target-independent planning
 *          ->
 *     optimization
 *          ->
 *     lowering
 *          ->
 *     routing
 *          ->
 *     scheduling
 *          ->
 *     resilience
 *          ->
 *     ZQN
 *          ->
 *     HAL
 *          ->
 *     available target
 *
 * remains compatible with:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * subject only to the actual semantics of the program, declared requirements,
 * available resources, target capabilities, and implementation feasibility.
 *
 * ============================================================================
 */
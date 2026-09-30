/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/nano/protocols.g4
 *
 * GRAMMAR IDENTITY
 * ----------------
 * NanoProtocols
 *
 * STATUS
 * ------
 * Production-ready canonical nano-domain protocol grammar component.
 *
 * LANGUAGE
 * --------
 * Zamani
 *
 * IMPLEMENTATION BASELINE
 * -----------------------
 * Rust 1.97 / Rust 1.97.1
 * Rust 2021
 * Safe Rust only
 * No unsafe Rust
 *
 * ============================================================================
 * 1. PURPOSE
 * ============================================================================
 *
 * This file owns the SOURCE-LEVEL SYNTAX for nano-domain protocols.
 *
 * A nano protocol is a target-independent computational/physical-intent
 * contract describing how nano-oriented entities, agents, materials,
 * molecular structures, sensors, actuators, processes, or other nano-domain
 * participants may coordinate.
 *
 * This grammar supports:
 *
 *   - nano protocol declarations;
 *   - protocol parameterization;
 *   - protocol refinement;
 *   - protocol roles;
 *   - participant references;
 *   - message/event references;
 *   - operation contracts;
 *   - requirements;
 *   - capabilities;
 *   - constraints;
 *   - preferences;
 *   - properties;
 *   - protocol states;
 *   - state transitions;
 *   - guards;
 *   - events;
 *   - send/receive/emit intent;
 *   - observation intent;
 *   - action intent;
 *   - sequencing;
 *   - choice;
 *   - parallel composition;
 *   - repetition;
 *   - optional behavior;
 *   - waiting;
 *   - nested protocol behavior;
 *   - protocol composition;
 *   - references to atoms;
 *   - references to molecules;
 *   - references to materials;
 *   - references to nano-agents;
 *   - references to nano capabilities;
 *   - references to ordinary Zamani expressions and types;
 *   - future nano-domain protocol extensions.
 *
 * The grammar is OPEN-WORLD.
 *
 * It deliberately does NOT enumerate:
 *
 *   - a finite list of nano protocols;
 *   - a finite list of molecules;
 *   - a finite list of materials;
 *   - a finite list of nano agents;
 *   - a finite list of sensors;
 *   - a finite list of actuators;
 *   - a finite list of fabrication methods;
 *   - a finite list of physical interaction mechanisms;
 *   - a finite list of nano devices;
 *   - a finite list of nano communication mechanisms.
 *
 * ============================================================================
 * 2. ARCHITECTURAL POSITION
 * ============================================================================
 *
 *                         Zamani source
 *                              |
 *                              v
 *                        ZamaniLexer
 *                              |
 *                              v
 *                     ZamaniParser
 *                              |
 *                              v
 *                         Nano domain
 *                              |
 *                +-------------+-------------+
 *                |             |             |
 *                v             v             v
 *           NanoAgents      NanoAtoms    NanoMolecules
 *                |             |             |
 *                +-------------+-------------+
 *                              |
 *                              v
 *                       NanoProtocols
 *                              |
 *                              v
 *                     Domain-neutral AST
 *                              |
 *                              v
 *                    Structural validation
 *                              |
 *                              v
 *                       Semantic analysis
 *                              |
 *          +-------------------+-------------------+
 *          |                   |                   |
 *          v                   v                   v
 *      nano semantics     classical semantics  quantum semantics
 *          |                   |                   |
 *          +-------------------+-------------------+
 *                              |
 *                              v
 *                    Canonical semantic model
 *                              |
 *                              v
 *                         Canonical IR
 *                              |
 *            +-----------------+-----------------+
 *            |                 |                 |
 *            v                 v                 v
 *        classical         quantum::ir       HDL/hardware
 *            |                 |                 |
 *            +-----------------+-----------------+
 *                              |
 *                              v
 *                  optimization / lowering
 *                              |
 *                  routing / scheduling
 *                              |
 *                       resilience / QEC
 *                              |
 *                             ZQN
 *                              |
 *                             HAL
 *                              |
 *                     target realization
 *
 * This file is PARSER-ONLY.
 *
 * It does not construct IR, perform simulation, discover hardware, allocate
 * resources, execute protocols, or select a physical implementation.
 *
 * ============================================================================
 * 3. OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   nanoProtocolConstruct
 *   nanoProtocolAnnotatedConstruct
 *   nanoProtocolAnnotation
 *   nanoProtocolDeclaration
 *   nanoProtocolName
 *   nanoProtocolTypeParameterList
 *   nanoProtocolTypeParameter
 *   nanoProtocolInheritanceClause
 *   nanoProtocolWhereClause
 *   nanoProtocolBody
 *   nanoProtocolMember
 *   nanoProtocolRoleDeclaration
 *   nanoProtocolParticipantDeclaration
 *   nanoProtocolMessageReference
 *   nanoProtocolEventDeclaration
 *   nanoProtocolOperationDeclaration
 *   nanoProtocolParameterList
 *   nanoProtocolParameter
 *   nanoProtocolReturnClause
 *   nanoProtocolOperationClause
 *   nanoProtocolUseDeclaration
 *   nanoProtocolProvideDeclaration
 *   nanoProtocolRequirementDeclaration
 *   nanoProtocolCapabilityDeclaration
 *   nanoProtocolConstraintDeclaration
 *   nanoProtocolPreferenceDeclaration
 *   nanoProtocolPropertyDeclaration
 *   nanoProtocolStateDeclaration
 *   nanoProtocolTransitionDeclaration
 *   nanoProtocolFlowDeclaration
 *   nanoProtocolFlowStep
 *   nanoProtocolSendStep
 *   nanoProtocolReceiveStep
 *   nanoProtocolEmitStep
 *   nanoProtocolObserveStep
 *   nanoProtocolActStep
 *   nanoProtocolCallStep
 *   nanoProtocolSequenceStep
 *   nanoProtocolChoiceStep
 *   nanoProtocolChoiceBranch
 *   nanoProtocolParallelStep
 *   nanoProtocolRepeatStep
 *   nanoProtocolOptionalStep
 *   nanoProtocolWaitStep
 *   nanoProtocolGuardClause
 *   nanoProtocolNestedDeclaration
 *   nanoProtocolReference
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - lexical token definitions;
 *   - identifiers;
 *   - qualified-name syntax;
 *   - type syntax;
 *   - expression precedence;
 *   - ordinary statements;
 *   - functions;
 *   - modules;
 *   - atom definitions;
 *   - molecular definitions;
 *   - material definitions;
 *   - nano-agent definitions;
 *   - generic networking protocols;
 *   - physical communication protocols;
 *   - networking endpoints;
 *   - networking channels;
 *   - IP addresses;
 *   - ports;
 *   - sockets;
 *   - network topology;
 *   - hardware topology;
 *   - device discovery;
 *   - physical placement;
 *   - routing;
 *   - scheduling;
 *   - calibration;
 *   - physical simulation;
 *   - chemical simulation;
 *   - quantum simulation;
 *   - QEC implementation;
 *   - ZQN implementation;
 *   - HAL implementation;
 *   - resource allocation;
 *   - runtime execution;
 *   - IR construction.
 *
 * ============================================================================
 * 4. CRITICAL DOMAIN SEPARATION
 * ============================================================================
 *
 * Nano protocols are NOT the same thing as networking protocols.
 *
 * Generic communication protocols are owned by:
 *
 *     grammar/networking/protocols.g4
 *
 * Nano protocol syntax is owned here.
 *
 * Examples of nano-domain protocol intent include:
 *
 *     coordination between nano agents;
 *     molecular interaction procedures;
 *     material-state procedures;
 *     nano-sensor/actuator coordination;
 *     nanoscale process sequencing;
 *     nano-domain observation/action contracts;
 *     nano fabrication or transformation intent;
 *     nano-agent swarm coordination contracts.
 *
 * A nano protocol MAY eventually use networking as an implementation
 * mechanism, but this grammar does not select networking.
 *
 * Therefore:
 *
 *     nano protocol
 *          !=
 *     network protocol
 *
 * A semantic implementation may relate them later.
 *
 * ============================================================================
 * 5. CANONICAL LEXICAL DEPENDENCY
 * ============================================================================
 *
 * This grammar consumes the canonical Zamani lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * through:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * This file MUST NOT:
 *
 *   - define lexer rules;
 *   - define a second identifier token;
 *   - define NANO_PROTOCOL;
 *   - define NANO_ROLE;
 *   - define NANO_MESSAGE;
 *   - define NANO_STATE;
 *   - define NANO_EVENT;
 *   - define NANO_ACTION;
 *   - define NANO_OBSERVATION;
 *   - define protocol-specific keyword tokens.
 *
 * The nano protocol vocabulary remains structurally open.
 *
 * ============================================================================
 * 6. SHARED PARSER DEPENDENCIES
 * ============================================================================
 *
 * Canonical parser components:
 *
 *     Names
 *     Types
 *     Expressions
 *     Statements
 *
 * This file therefore imports:
 *
 *     Names
 *     Types
 *     Expressions
 *     Statements
 *
 * Reused rules include:
 *
 *     identifier
 *     qualifiedName
 *     typeExpression
 *     expression
 *     argumentList
 *     statement
 *
 * This file MUST NOT redefine any of those rules.
 *
 * ============================================================================
 * 7. OPEN-WORLD ANNOTATION MODEL
 * ============================================================================
 *
 * The outer nano protocol construct uses an annotation:
 *
 *     @protocol
 *
 * However, the annotation name is intentionally represented by the ordinary
 * identifier token.
 *
 * This means future source forms can remain extensible without introducing
 * new lexer keywords.
 *
 * Examples:
 *
 *     @protocol Delivery {
 *         ...
 *     }
 *
 *     @protocol Assembly {
 *         ...
 *     }
 *
 *     @protocol Coordination<T> {
 *         ...
 *     }
 *
 * The parser does NOT inspect whether the identifier spells "protocol".
 *
 * Semantic analysis determines whether an annotation is a valid nano
 * protocol annotation in its enclosing context.
 *
 * ============================================================================
 * 8. POCO-REAF
 * ============================================================================
 *
 * Nano protocol source describes WHAT behavior or contract is required.
 *
 * It does NOT permanently describe:
 *
 *   - which nano device;
 *   - which nano agent instance;
 *   - which physical material;
 *   - which molecular structure implementation;
 *   - which CPU;
 *   - which GPU;
 *   - which FPGA;
 *   - which ASIC;
 *   - which QPU;
 *   - which accelerator;
 *   - which node;
 *   - which network interface;
 *   - which physical communication medium;
 *   - which memory bank;
 *   - which physical sensor;
 *   - which physical actuator.
 *
 * Target realization remains downstream.
 *
 * This preserves:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * subject to actual program semantics, requirements, capabilities, constraints
 * and resources available to the realization environment.
 *
 * ============================================================================
 * 9. NO ARTIFICIAL SCALABILITY CEILINGS
 * ============================================================================
 *
 * This grammar contains NO universal limits for:
 *
 *     protocols
 *     protocol parameters
 *     roles
 *     participants
 *     messages
 *     events
 *     operations
 *     properties
 *     requirements
 *     capabilities
 *     constraints
 *     preferences
 *     states
 *     transitions
 *     flow steps
 *     nested flows
 *     nested protocols
 *     protocol references
 *     atoms
 *     molecules
 *     materials
 *     nano agents
 *     devices
 *     resources
 *     nodes
 *     processes
 *     communication links
 *     timelines
 *     quantum resources
 *     classical resources
 *     hardware resources
 *
 * It MUST NOT define:
 *
 *     MAX_NANO_PROTOCOLS
 *     MAX_NANO_ROLES
 *     MAX_NANO_PARTICIPANTS
 *     MAX_NANO_MESSAGES
 *     MAX_NANO_OPERATIONS
 *     MAX_NANO_STATES
 *     MAX_NANO_TRANSITIONS
 *     MAX_NANO_AGENTS
 *     MAX_NANO_DEVICES
 *     MAX_NANO_RESOURCES
 *     MAX_NANO_DEPTH
 *     MAX_NANO_MESSAGE_SIZE
 *     MAX_NANO_MEMORY
 *     MAX_NANO_ENERGY
 *     MAX_NANO_BANDWIDTH
 *
 * Repetition is expressed structurally with `*` or `+`.
 *
 * A finite implementation resource is not a language-level semantic limit.
 *
 * ============================================================================
 * 10. NUMERIC VALUES
 * ============================================================================
 *
 * Numeric literals remain program semantics.
 *
 * For example:
 *
 *     wait 100;
 *
 * or:
 *
 *     requires nano::energy >= budget;
 *
 * does NOT establish a universal language-level limit.
 *
 * The grammar does not interpret numeric values as hardware capacity.
 *
 * ============================================================================
 * 11. REQUIREMENT / CONSTRAINT / CAPABILITY / PREFERENCE SEPARATION
 * ============================================================================
 *
 * Nano protocol syntax distinguishes:
 *
 *     requirement
 *     constraint
 *     capability
 *     preference
 *
 * They have different semantic responsibilities.
 *
 * Requirement:
 *
 *     what must be satisfied.
 *
 * Constraint:
 *
 *     what legal realizations must obey.
 *
 * Capability:
 *
 *     what a realization must provide.
 *
 * Preference:
 *
 *     what a compiler/runtime may prefer without changing correctness.
 *
 * This grammar does not evaluate any of them.
 *
 * ============================================================================
 * 12. DETERMINISM
 * ============================================================================
 *
 * Parsing is a pure source-processing operation.
 *
 * Given the same:
 *
 *     source;
 *     language version;
 *     grammar version;
 *     token vocabulary;
 *
 * the parser must produce equivalent parse structure.
 *
 * Parsing MUST NOT depend on:
 *
 *     hardware;
 *     resource availability;
 *     network state;
 *     filesystem state;
 *     DNS;
 *     environment variables;
 *     wall-clock time;
 *     randomness;
 *     device discovery;
 *     runtime state.
 *
 * ============================================================================
 * 13. SECURITY
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no Rust actions;
 *     no embedded code;
 *     no semantic predicates;
 *     no filesystem operations;
 *     no network operations;
 *     no shell execution;
 *     no hardware discovery;
 *     no credentials;
 *     no runtime callbacks.
 *
 * The generated parser therefore has no grammar-level requirement for
 * unsafe Rust.
 *
 * ============================================================================
 * 14. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * Exactly one public nano-protocol entry point is exposed:
 *
 *     nanoProtocolConstruct
 *
 * The universal parser/domain dispatcher must consume this entry point.
 *
 * This file does not consume EOF.
 *
 * ============================================================================
 */

parser grammar NanoProtocols;

options {
    tokenVocab = ZamaniLexer;
}

import Names, Types, Expressions, Statements;


/*
 * ============================================================================
 * 15. PUBLIC NANO PROTOCOL CONSTRUCT
 * ============================================================================
 *
 * The annotation-based boundary avoids ambiguity with:
 *
 *     grammar/networking/protocols.g4
 *
 * and with ordinary Zamani declarations.
 *
 * Example:
 *
 *     @protocol Delivery {
 *         ...
 *     }
 */
nanoProtocolConstruct
    : nanoProtocolAnnotatedConstruct
    ;


/*
 * ============================================================================
 * 16. ANNOTATED CONSTRUCT
 * ============================================================================
 */

nanoProtocolAnnotatedConstruct
    : nanoProtocolAnnotation
      nanoProtocolDeclaration
    ;


/*
 * ============================================================================
 * 17. PROTOCOL ANNOTATION
 * ============================================================================
 *
 * The annotation name remains an ordinary identifier.
 *
 * The canonical semantic spelling is expected to be:
 *
 *     @protocol
 *
 * but the parser deliberately does not hard-code that spelling.
 *
 * This allows future dialects/extensions to register protocol annotations
 * without changing the lexer.
 */
nanoProtocolAnnotation
    : AT identifier
    ;


/*
 * ============================================================================
 * 18. PROTOCOL DECLARATION
 * ============================================================================
 *
 * Examples:
 *
 *     @protocol Delivery;
 *
 *     @protocol Delivery {
 *         ...
 *     }
 *
 *     @protocol Delivery<T> extends Base {
 *         ...
 *     }
 */
nanoProtocolDeclaration
    : nanoProtocolName
      nanoProtocolTypeParameterList?
      nanoProtocolInheritanceClause?
      nanoProtocolWhereClause?
      nanoProtocolBody?
      SEMI?
    ;


/*
 * ============================================================================
 * 19. PROTOCOL NAME
 * ============================================================================
 */

nanoProtocolName
    : identifier
    ;


/*
 * ============================================================================
 * 20. TYPE PARAMETERS
 * ============================================================================
 *
 * Protocol parameterization reuses the canonical type system.
 *
 * There is no fixed generic arity.
 */
nanoProtocolTypeParameterList
    : LESS_THAN
      nanoProtocolTypeParameter
      (COMMA nanoProtocolTypeParameter)*
      COMMA?
      GREATER_THAN
    ;


nanoProtocolTypeParameter
    : identifier
      nanoProtocolTypeParameterBound?
    ;


nanoProtocolTypeParameterBound
    : COLON
      typeExpression
    ;


/*
 * ============================================================================
 * 21. PROTOCOL REFINEMENT
 * ============================================================================
 *
 * Refinement is semantic composition.
 *
 * It does not imply object-oriented implementation inheritance.
 */
nanoProtocolInheritanceClause
    : EXTENDS
      qualifiedName
      (COMMA qualifiedName)*
    ;


/*
 * ============================================================================
 * 22. PROTOCOL WHERE CLAUSE
 * ============================================================================
 */

nanoProtocolWhereClause
    : WHERE
      expression
    ;


/*
 * ============================================================================
 * 23. PROTOCOL BODY
 * ============================================================================
 */

nanoProtocolBody
    : LBRACE
      nanoProtocolMember*
      RBRACE
    ;


/*
 * ============================================================================
 * 24. PROTOCOL MEMBER DISPATCH
 * ============================================================================
 */

nanoProtocolMember
    : nanoProtocolRoleDeclaration
    | nanoProtocolParticipantDeclaration
    | nanoProtocolMessageReference
    | nanoProtocolEventDeclaration
    | nanoProtocolOperationDeclaration
    | nanoProtocolUseDeclaration
    | nanoProtocolProvideDeclaration
    | nanoProtocolRequirementDeclaration
    | nanoProtocolCapabilityDeclaration
    | nanoProtocolConstraintDeclaration
    | nanoProtocolPreferenceDeclaration
    | nanoProtocolPropertyDeclaration
    | nanoProtocolStateDeclaration
    | nanoProtocolTransitionDeclaration
    | nanoProtocolFlowDeclaration
    | nanoProtocolNestedDeclaration
    | statement
    ;


/*
 * ============================================================================
 * 25. ROLE
 * ============================================================================
 *
 * A role is an abstract participant position.
 *
 * It is NOT:
 *
 *     a physical nano-device;
 *     a CPU;
 *     a process;
 *     a thread;
 *     a network endpoint;
 *     a hardware identifier.
 */
nanoProtocolRoleDeclaration
    : nanoProtocolRoleMarker
      identifier
      nanoProtocolRoleTypeClause?
      SEMI
    ;


nanoProtocolRoleMarker
    : identifier
    ;


nanoProtocolRoleTypeClause
    : COLON
      typeExpression
    ;


/*
 * ============================================================================
 * 26. PARTICIPANT
 * ============================================================================
 *
 * A participant identifies a logical source-level entity that participates in
 * the protocol.
 *
 * The actual entity may later resolve to:
 *
 *     atom;
 *     molecule;
 *     material;
 *     nano-agent;
 *     sensor;
 *     actuator;
 *     classical process;
 *     quantum process;
 *     hardware component;
 *     distributed service;
 *     future domain entity.
 *
 * No physical allocation occurs here.
 */
nanoProtocolParticipantDeclaration
    : nanoProtocolParticipantMarker
      identifier
      nanoProtocolParticipantTypeClause?
      nanoProtocolParticipantBindingClause?
      SEMI
    ;


nanoProtocolParticipantMarker
    : identifier
    ;


nanoProtocolParticipantTypeClause
    : COLON
      typeExpression
    ;


nanoProtocolParticipantBindingClause
    : ASSIGN
      expression
    ;


/*
 * ============================================================================
 * 27. MESSAGE / DATA REFERENCE
 * ============================================================================
 *
 * A nano protocol may reference a message/data contract owned elsewhere.
 *
 * This grammar does not define the message schema.
 */
nanoProtocolMessageReference
    : nanoProtocolMessageMarker
      qualifiedName
      nanoProtocolMessageTypeClause?
      SEMI
    ;


nanoProtocolMessageMarker
    : identifier
    ;


nanoProtocolMessageTypeClause
    : COLON
      typeExpression
    ;


/*
 * ============================================================================
 * 28. EVENT DECLARATION
 * ============================================================================
 *
 * Events are logical protocol events.
 *
 * They may later correspond to:
 *
 *     observation;
 *     material change;
 *     molecular event;
 *     agent event;
 *     hardware event;
 *     classical event;
 *     quantum event;
 *     external event.
 *
 * No event implementation is defined here.
 */
nanoProtocolEventDeclaration
    : nanoProtocolEventMarker
      identifier
      LPAREN
      nanoProtocolParameterList?
      RPAREN
      SEMI
    ;


nanoProtocolEventMarker
    : identifier
    ;


/*
 * ============================================================================
 * 29. OPERATION CONTRACT
 * ============================================================================
 *
 * Operations describe protocol-level actions/contracts.
 *
 * They do not select a physical actuator or implementation.
 */
nanoProtocolOperationDeclaration
    : nanoProtocolOperationMarker
      identifier
      nanoProtocolTypeParameterList?
      LPAREN
      nanoProtocolParameterList?
      RPAREN
      nanoProtocolReturnClause?
      nanoProtocolOperationClause*
      nanoProtocolWhereClause?
      SEMI
    ;


nanoProtocolOperationMarker
    : identifier
    ;


/*
 * ============================================================================
 * 30. OPERATION PARAMETERS
 * ============================================================================
 */

nanoProtocolParameterList
    : nanoProtocolParameter
      (COMMA nanoProtocolParameter)*
      COMMA?
    ;


nanoProtocolParameter
    : identifier
      COLON
      typeExpression
      nanoProtocolParameterDefault?
    ;


nanoProtocolParameterDefault
    : ASSIGN
      expression
    ;


/*
 * ============================================================================
 * 31. OPERATION RETURN TYPE
 * ============================================================================
 */

nanoProtocolReturnClause
    : ARROW
      typeExpression
    ;


/*
 * ============================================================================
 * 32. OPERATION CLAUSES
 * ============================================================================
 */

nanoProtocolOperationClause
    : nanoProtocolRequiresClause
    | nanoProtocolProvidesClause
    | nanoProtocolUsesClause
    | nanoProtocolConstraintClause
    ;


/*
 * ============================================================================
 * 33. REQUIREMENT
 * ============================================================================
 */

nanoProtocolRequiresClause
    : REQUIRES
      expression
    ;


/*
 * ============================================================================
 * 34. PROVIDES
 * ============================================================================
 *
 * `provides` remains contextual rather than introducing a new lexer token.
 */
nanoProtocolProvidesClause
    : nanoProtocolProvidesMarker
      expression
    ;


nanoProtocolProvidesMarker
    : identifier
    ;


/*
 * ============================================================================
 * 35. USES
 * ============================================================================
 */

nanoProtocolUsesClause
    : nanoProtocolUsesMarker
      expression
    ;


nanoProtocolUsesMarker
    : identifier
    ;


/*
 * ============================================================================
 * 36. CONSTRAINT CLAUSE
 * ============================================================================
 */

nanoProtocolConstraintClause
    : CONSTRAINT
      expression
    ;


/*
 * ============================================================================
 * 37. PROTOCOL USE
 * ============================================================================
 *
 * This is logical protocol composition.
 *
 * It does not select an implementation.
 */
nanoProtocolUseDeclaration
    : nanoProtocolUsesMarker
      qualifiedName
      SEMI
    ;


/*
 * ============================================================================
 * 38. PROTOCOL PROVIDE
 * ============================================================================
 */

nanoProtocolProvideDeclaration
    : nanoProtocolProvidesMarker
      qualifiedName
      SEMI
    ;


/*
 * ============================================================================
 * 39. PROTOCOL REQUIREMENT DECLARATION
 * ============================================================================
 */

nanoProtocolRequirementDeclaration
    : REQUIRES
      expression
      SEMI
    ;


/*
 * ============================================================================
 * 40. CAPABILITY DECLARATION
 * ============================================================================
 *
 * A capability is a semantic requirement/reference.
 *
 * Capability discovery is downstream.
 */
nanoProtocolCapabilityDeclaration
    : CAPABILITY
      qualifiedName
      nanoProtocolCapabilityValue?
      SEMI
    ;


nanoProtocolCapabilityValue
    : ASSIGN
      expression
    ;


/*
 * ============================================================================
 * 41. CONSTRAINT DECLARATION
 * ============================================================================
 */

nanoProtocolConstraintDeclaration
    : CONSTRAINT
      expression
      SEMI
    ;


/*
 * ============================================================================
 * 42. PREFERENCE DECLARATION
 * ============================================================================
 */

nanoProtocolPreferenceDeclaration
    : PREFER
      expression
      SEMI
    ;


/*
 * ============================================================================
 * 43. PROPERTY DECLARATION
 * ============================================================================
 */

nanoProtocolPropertyDeclaration
    : PROPERTY
      identifier
      nanoProtocolPropertyTypeClause?
      nanoProtocolPropertyInitializer?
      SEMI
    ;


nanoProtocolPropertyTypeClause
    : COLON
      typeExpression
    ;


nanoProtocolPropertyInitializer
    : ASSIGN
      expression
    ;


/*
 * ============================================================================
 * 44. PROTOCOL STATE
 * ============================================================================
 *
 * States are abstract protocol states.
 *
 * They do not represent:
 *
 *     physical device states;
 *     CPU states;
 *     memory states;
 *     QPU states;
 *     hardware power states.
 */
nanoProtocolStateDeclaration
    : nanoProtocolStateMarker
      identifier
      SEMI
    ;


nanoProtocolStateMarker
    : identifier
    ;


/*
 * ============================================================================
 * 45. PROTOCOL TRANSITION
 * ============================================================================
 *
 * Example:
 *
 *     transition idle -> active;
 *
 *     transition idle -> active on stimulus;
 *
 *     transition active -> complete
 *         on completion
 *         requires capability::something;
 */
nanoProtocolTransitionDeclaration
    : nanoProtocolTransitionMarker
      identifier
      ARROW
      identifier
      nanoProtocolTransitionEventClause?
      nanoProtocolTransitionConditionClause*
      SEMI
    ;


nanoProtocolTransitionMarker
    : identifier
    ;


nanoProtocolTransitionEventClause
    : nanoProtocolOnMarker
      expression
    ;


nanoProtocolOnMarker
    : identifier
    ;


nanoProtocolTransitionConditionClause
    : REQUIRES
      expression
    ;


/*
 * ============================================================================
 * 46. FLOW DECLARATION
 * ============================================================================
 *
 * A flow describes an abstract protocol interaction sequence.
 */
nanoProtocolFlowDeclaration
    : nanoProtocolFlowMarker
      identifier?
      LBRACE
      nanoProtocolFlowStep*
      RBRACE
      SEMI?
    ;


nanoProtocolFlowMarker
    : identifier
    ;


/*
 * ============================================================================
 * 47. FLOW STEP DISPATCH
 * ============================================================================
 */

nanoProtocolFlowStep
    : nanoProtocolSendStep
    | nanoProtocolReceiveStep
    | nanoProtocolEmitStep
    | nanoProtocolObserveStep
    | nanoProtocolActStep
    | nanoProtocolCallStep
    | nanoProtocolSequenceStep
    | nanoProtocolChoiceStep
    | nanoProtocolParallelStep
    | nanoProtocolRepeatStep
    | nanoProtocolOptionalStep
    | nanoProtocolWaitStep
    | nanoProtocolGuardClause
    | nanoProtocolFlowExpressionStep
    ;


/*
 * ============================================================================
 * 48. SEND
 * ============================================================================
 *
 * Send is abstract intent.
 *
 * It does not imply:
 *
 *     socket;
 *     network interface;
 *     physical communication medium;
 *     radio;
 *     molecular channel;
 *     optical channel;
 *     quantum channel.
 */
nanoProtocolSendStep
    : nanoProtocolSendMarker
      expression
      SEMI
    ;


nanoProtocolSendMarker
    : identifier
    ;


/*
 * ============================================================================
 * 49. RECEIVE
 * ============================================================================
 */

nanoProtocolReceiveStep
    : nanoProtocolReceiveMarker
      expression
      SEMI
    ;


nanoProtocolReceiveMarker
    : identifier
    ;


/*
 * ============================================================================
 * 50. EMIT
 * ============================================================================
 *
 * Logical event emission.
 */
nanoProtocolEmitStep
    : nanoProtocolEmitMarker
      expression
      SEMI
    ;


nanoProtocolEmitMarker
    : identifier
    ;


/*
 * ============================================================================
 * 51. OBSERVE
 * ============================================================================
 *
 * Observation is a logical protocol operation.
 *
 * It may eventually correspond to:
 *
 *     sensor observation;
 *     material observation;
 *     molecular observation;
 *     classical measurement;
 *     quantum measurement;
 *     external observation.
 *
 * Quantum semantics remain owned by grammar/quantum and quantum::ir.
 */
nanoProtocolObserveStep
    : nanoProtocolObserveMarker
      expression
      SEMI
    ;


nanoProtocolObserveMarker
    : identifier
    ;


/*
 * ============================================================================
 * 52. ACT
 * ============================================================================
 *
 * Action intent is abstract.
 *
 * The semantic/backend layers determine whether the action is realized by:
 *
 *     nano actuator;
 *     material transformation;
 *     classical computation;
 *     hardware;
 *     quantum operation;
 *     distributed service.
 */
nanoProtocolActStep
    : nanoProtocolActMarker
      expression
      SEMI
    ;


nanoProtocolActMarker
    : identifier
    ;


/*
 * ============================================================================
 * 53. FLOW CALL
 * ============================================================================
 *
 * A flow may call another named protocol operation or flow.
 */
nanoProtocolCallStep
    : qualifiedName
      LPAREN
      nanoProtocolFlowArgumentList?
      RPAREN
      SEMI
    ;


nanoProtocolFlowArgumentList
    : expression
      (COMMA expression)*
      COMMA?
    ;


/*
 * ============================================================================
 * 54. SEQUENCE
 * ============================================================================
 */

nanoProtocolSequenceStep
    : nanoProtocolSequenceMarker
      LBRACE
      nanoProtocolFlowStep*
      RBRACE
      SEMI?
    ;


nanoProtocolSequenceMarker
    : identifier
    ;


/*
 * ============================================================================
 * 55. CHOICE
 * ============================================================================
 */

nanoProtocolChoiceStep
    : nanoProtocolChoiceMarker
      LBRACE
      nanoProtocolChoiceBranch+
      RBRACE
      SEMI?
    ;


nanoProtocolChoiceMarker
    : identifier
    ;


nanoProtocolChoiceBranch
    : nanoProtocolFlowStep+
    ;


/*
 * ============================================================================
 * 56. PARALLEL
 * ============================================================================
 *
 * Parallelism is semantic intent.
 *
 * The compiler/runtime may realize it using:
 *
 *     concurrency;
 *     pipelining;
 *     batching;
 *     vectorization;
 *     distributed execution;
 *     hardware acceleration;
 *     serialization where required by constraints.
 *
 * The grammar does not impose a processor/thread/device count.
 */
nanoProtocolParallelStep
    : nanoProtocolParallelMarker
      LBRACE
      nanoProtocolFlowStep*
      RBRACE
      SEMI?
    ;


nanoProtocolParallelMarker
    : identifier
    ;


/*
 * ============================================================================
 * 57. REPEAT
 * ============================================================================
 *
 * The repeat condition is a source-level expression.
 *
 * It may represent:
 *
 *     finite repetition;
 *     symbolic repetition;
 *     condition-controlled repetition;
 *     resource-dependent repetition;
 *     convergence;
 *     event-driven repetition.
 *
 * No maximum iteration count is imposed.
 */
nanoProtocolRepeatStep
    : nanoProtocolRepeatMarker
      nanoProtocolRepeatCondition?
      LBRACE
      nanoProtocolFlowStep*
      RBRACE
      SEMI?
    ;


nanoProtocolRepeatMarker
    : identifier
    ;


nanoProtocolRepeatCondition
    : expression
    ;


/*
 * ============================================================================
 * 58. OPTIONAL
 * ============================================================================
 */

nanoProtocolOptionalStep
    : nanoProtocolOptionalMarker
      LBRACE
      nanoProtocolFlowStep*
      RBRACE
      SEMI?
    ;


nanoProtocolOptionalMarker
    : identifier
    ;


/*
 * ============================================================================
 * 59. WAIT
 * ============================================================================
 *
 * Waiting is logical protocol intent.
 *
 * Time units, scheduling semantics, real-time guarantees, and implementation
 * behavior belong downstream.
 */
nanoProtocolWaitStep
    : nanoProtocolWaitMarker
      expression
      SEMI
    ;


nanoProtocolWaitMarker
    : identifier
    ;


/*
 * ============================================================================
 * 60. GUARD
 * ============================================================================
 *
 * A guard provides an explicit semantic condition around protocol behavior.
 */
nanoProtocolGuardClause
    : nanoProtocolGuardMarker
      expression
      SEMI
    ;


nanoProtocolGuardMarker
    : identifier
    ;


/*
 * ============================================================================
 * 61. GENERIC FLOW EXPRESSION
 * ============================================================================
 *
 * This provides forward compatibility without creating a second expression
 * language.
 */
nanoProtocolFlowExpressionStep
    : expression
      SEMI
    ;


/*
 * ============================================================================
 * 62. NESTED PROTOCOL
 * ============================================================================
 *
 * Nested protocol declarations provide source-level scoping/composition.
 *
 * Semantic analysis determines whether a particular nesting is legal.
 */
nanoProtocolNestedDeclaration
    : nanoProtocolAnnotatedConstruct
    ;


/*
 * ============================================================================
 * 63. PROTOCOL REFERENCE
 * ============================================================================
 *
 * A protocol reference is an ordinary qualified name.
 *
 * Semantic analysis resolves it.
 */
nanoProtocolReference
    : qualifiedName
    ;


nanoProtocolReferenceList
    : nanoProtocolReference
      (COMMA nanoProtocolReference)*
      COMMA?
    ;


/*
 * ============================================================================
 * 64. REUSABLE VALUE CONTRACTS
 * ============================================================================
 *
 * These are adapter rules only.
 *
 * They do not define a new expression language.
 */
nanoProtocolValue
    : expression
    ;


nanoProtocolCondition
    : expression
    ;


nanoProtocolPredicate
    : expression
    ;


/*
 * ============================================================================
 * 65. AST CONTRACT
 * ============================================================================
 *
 * Every rule in this file must lower into the existing domain-neutral
 * frontend AST.
 *
 * Conceptual mappings:
 *
 *     nanoProtocolDeclaration
 *         -> generic protocol/declaration representation
 *
 *     nanoProtocolRoleDeclaration
 *         -> logical role representation
 *
 *     nanoProtocolParticipantDeclaration
 *         -> participant/reference representation
 *
 *     nanoProtocolMessageReference
 *         -> message/data reference representation
 *
 *     nanoProtocolEventDeclaration
 *         -> event representation
 *
 *     nanoProtocolOperationDeclaration
 *         -> operation-contract representation
 *
 *     nanoProtocolRequirementDeclaration
 *         -> requirement representation
 *
 *     nanoProtocolCapabilityDeclaration
 *         -> capability representation
 *
 *     nanoProtocolConstraintDeclaration
 *         -> constraint representation
 *
 *     nanoProtocolPreferenceDeclaration
 *         -> preference representation
 *
 *     nanoProtocolPropertyDeclaration
 *         -> property representation
 *
 *     nanoProtocolStateDeclaration
 *         -> abstract state representation
 *
 *     nanoProtocolTransitionDeclaration
 *         -> transition representation
 *
 *     nanoProtocolFlowDeclaration
 *         -> flow/behavior representation
 *
 *     nanoProtocolSendStep
 *         -> generic communication/interaction intent
 *
 *     nanoProtocolReceiveStep
 *         -> generic communication/interaction intent
 *
 *     nanoProtocolObserveStep
 *         -> generic observation intent
 *
 *     nanoProtocolActStep
 *         -> generic action intent
 *
 * No NanoProtocolIR is introduced by this grammar.
 *
 * The AST must preserve source spans for:
 *
 *     annotations;
 *     names;
 *     types;
 *     expressions;
 *     states;
 *     transitions;
 *     flow steps;
 *     requirements;
 *     capabilities;
 *     properties.
 *
 * ============================================================================
 * 66. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis owns:
 *
 *     annotation validation;
 *     protocol name resolution;
 *     protocol scope;
 *     protocol refinement;
 *     type checking;
 *     parameter checking;
 *     role validation;
 *     participant resolution;
 *     atom resolution;
 *     molecule resolution;
 *     material resolution;
 *     nano-agent resolution;
 *     event resolution;
 *     operation resolution;
 *     state-machine validation;
 *     transition validity;
 *     flow consistency;
 *     requirement checking;
 *     capability checking;
 *     constraint checking;
 *     preference handling;
 *     security analysis;
 *     ownership/resource analysis;
 *     concurrency analysis;
 *     distributed compatibility;
 *     networking compatibility;
 *     hardware compatibility;
 *     quantum compatibility;
 *     portability analysis.
 *
 * The parser does none of these operations.
 *
 * ============================================================================
 * 67. ATOM INTEGRATION
 * ============================================================================
 *
 * Atomic entities remain owned by:
 *
 *     grammar/nano/atoms.g4
 *
 * This grammar may reference atoms through:
 *
 *     qualifiedName
 *     typeExpression
 *     expression
 *     participant declarations
 *     operation parameters
 *     properties
 *
 * It MUST NOT duplicate:
 *
 *     atomic structure;
 *     periodic-table syntax;
 *     isotope syntax;
 *     atomic-number semantics;
 *     electron-configuration semantics.
 *
 * ============================================================================
 * 68. MOLECULE INTEGRATION
 * ============================================================================
 *
 * Molecular structures remain owned by:
 *
 *     grammar/nano/molecules.g4
 *
 * Protocols may refer to molecular entities but do not define molecular
 * chemistry.
 *
 * ============================================================================
 * 69. MATERIAL INTEGRATION
 * ============================================================================
 *
 * Material structures remain owned by:
 *
 *     grammar/nano/materials.g4
 *
 * This grammar may express material-oriented protocol intent through names,
 * types, expressions, capabilities, and requirements.
 *
 * It does not embed:
 *
 *     material databases;
 *     crystal catalogues;
 *     chemical constants;
 *     fabrication catalogues;
 *     physical simulators.
 *
 * ============================================================================
 * 70. NANO-AGENT INTEGRATION
 * ============================================================================
 *
 * Nano-agent declarations remain owned by:
 *
 *     grammar/nano/agents.g4
 *
 * This grammar may coordinate logical agents through protocol roles and
 * participants.
 *
 * It does not redefine:
 *
 *     nanoAgentConstruct
 *     nanoAgentDeclaration
 *     nanoAgentBody
 *     nanoAgentInteraction
 *
 * An agent may implement, provide, require, participate in, or consume a
 * nano protocol, but that relationship is established semantically.
 *
 * ============================================================================
 * 71. INTERACTION INTEGRATION
 * ============================================================================
 *
 * If:
 *
 *     grammar/nano/interactions.g4
 *
 * is present or subsequently added, it remains the owner of generic
 * nano-domain interaction syntax.
 *
 * This protocol grammar consumes interaction intent structurally through
 * expressions and flow steps rather than defining a second interaction
 * language.
 *
 * No fixed interaction catalogue is introduced here.
 *
 * ============================================================================
 * 72. CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Nano capabilities remain open-world.
 *
 * A capability may be represented by:
 *
 *     capability::name
 *
 * or another canonical qualified name.
 *
 * This grammar does not determine whether the capability exists on the target.
 *
 * Capability discovery and negotiation are downstream.
 *
 * ============================================================================
 * 73. RESOURCE INTEGRATION
 * ============================================================================
 *
 * Resource semantics belong downstream to:
 *
 *     grammar/resources/
 *
 * A nano protocol may express:
 *
 *     requires resource::something;
 *
 *     requires expression;
 *
 *     capability resource::something;
 *
 * but this grammar does not allocate the resource.
 *
 * It does not contain:
 *
 *     MAX_MEMORY;
 *     MAX_ENERGY;
 *     MAX_DEVICES;
 *     MAX_AGENTS;
 *     MAX_ATOMS;
 *     MAX_MOLECULES.
 *
 * ============================================================================
 * 74. HARDWARE INTEGRATION
 * ============================================================================
 *
 * Hardware intent belongs to:
 *
 *     grammar/hardware/
 *
 * HDL intent belongs to:
 *
 *     grammar/hdl/
 *
 * A nano protocol may ultimately be realized by:
 *
 *     embedded hardware;
 *     FPGA;
 *     ASIC;
 *     accelerator;
 *     CPU;
 *     GPU;
 *     QPU;
 *     distributed hardware;
 *     simulation;
 *     future computational substrate.
 *
 * This grammar does not select one.
 *
 * ============================================================================
 * 75. CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Nano protocols may invoke or coordinate classical computation.
 *
 * Classical computation remains owned by:
 *
 *     grammar/classical/
 *
 * This grammar only embeds ordinary Zamani expressions/statements and
 * protocol-level intent.
 *
 * ============================================================================
 * 76. QUANTUM INTEGRATION
 * ============================================================================
 *
 * A nano protocol may coordinate quantum operations or measurements.
 *
 * Quantum syntax remains owned by:
 *
 *     grammar/quantum/
 *
 * The canonical semantic quantum boundary remains:
 *
 *     quantum::ir
 *
 * Therefore:
 *
 *     NanoProtocols
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     quantum semantic model
 *          |
 *          v
 *     quantum::ir
 *
 * NOT:
 *
 *     NanoProtocols
 *          |
 *          v
 *     NanoQuantumIR
 *
 * This grammar must not define:
 *
 *     QubitId;
 *     PhysicalQubitId;
 *     LogicalQubitId;
 *     GateKind;
 *     physical topology;
 *     calibration;
 *     pulses;
 *     QEC implementation.
 *
 * ============================================================================
 * 77. HYBRID INTEGRATION
 * ============================================================================
 *
 * Nano + classical + quantum programs remain one Zamani program.
 *
 * A semantic flow may therefore be:
 *
 *     observe nano state
 *          |
 *          v
 *     classical analysis
 *          |
 *          v
 *     quantum computation
 *          |
 *          v
 *     measurement
 *          |
 *          v
 *     classical decision
 *          |
 *          v
 *     nano action
 *
 * The grammar does not create a special hybrid IR.
 *
 * ============================================================================
 * 78. NETWORKING INTEGRATION
 * ============================================================================
 *
 * Networking protocols remain owned by:
 *
 *     grammar/networking/protocols.g4
 *
 * A nano protocol may eventually be transported over a network, but:
 *
 *     nano protocol
 *          !=
 *     network protocol
 *
 * Network implementation, endpoints, channels, addresses, transport,
 * serialization, routing and deployment remain outside this file.
 *
 * ============================================================================
 * 79. DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Nano protocols may participate in distributed computation.
 *
 * Distributed execution remains owned by:
 *
 *     grammar/distributed/
 *
 * This grammar does not define:
 *
 *     nodes;
 *     node counts;
 *     placement;
 *     replication;
 *     distributed scheduling;
 *     distributed storage.
 *
 * ============================================================================
 * 80. SECURITY INTEGRATION
 * ============================================================================
 *
 * Security semantics remain owned by:
 *
 *     grammar/security/
 *
 * Protocols may express security requirements through ordinary expressions
 * and capabilities.
 *
 * This grammar does not implement:
 *
 *     cryptography;
 *     key management;
 *     authentication engines;
 *     authorization engines;
 *     secure hardware.
 *
 * ============================================================================
 * 81. INTEROPERABILITY
 * ============================================================================
 *
 * Nano protocols may interoperate with:
 *
 *     external DSLs;
 *     HDL;
 *     QASM;
 *     QIR;
 *     C;
 *     C++;
 *     Rust;
 *     WASM;
 *     vendor interfaces;
 *     nano hardware interfaces.
 *
 * Such integration belongs to:
 *
 *     grammar/interoperability/
 *     grammar/dialects/
 *
 * This grammar remains vendor-neutral.
 *
 * ============================================================================
 * 82. COMPILER CONTRACT
 * ============================================================================
 *
 * Compiler stages may use nano protocol semantics for:
 *
 *     capability negotiation;
 *     resource planning;
 *     operation lowering;
 *     scheduling;
 *     placement;
 *     routing;
 *     simulation;
 *     hardware/software co-design;
 *     classical/quantum integration;
 *     resilience;
 *     deployment.
 *
 * None of these decisions are made by this grammar.
 *
 * ============================================================================
 * 83. RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime may:
 *
 *     instantiate protocol participants;
 *     bind logical roles;
 *     negotiate capabilities;
 *     select implementations;
 *     observe events;
 *     execute actions;
 *     recover from failures;
 *     migrate execution;
 *     choose alternative realizations.
 *
 * None of those behaviors are encoded in parser actions.
 *
 * ============================================================================
 * 84. ERROR / DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Syntax errors are parser responsibilities.
 *
 * Semantic errors include:
 *
 *     unknown protocol;
 *     invalid protocol refinement;
 *     unknown role;
 *     unknown participant;
 *     invalid atom reference;
 *     invalid molecule reference;
 *     invalid material reference;
 *     invalid nano-agent reference;
 *     invalid event;
 *     invalid operation;
 *     invalid state transition;
 *     unsatisfied requirement;
 *     unavailable capability;
 *     violated constraint;
 *     unsupported realization;
 *     invalid interoperability.
 *
 * These must be reported by downstream semantic/diagnostic infrastructure.
 *
 * The grammar must preserve sufficient source spans for useful diagnostics.
 *
 * ============================================================================
 * 85. COMPATIBILITY
 * ============================================================================
 *
 * This is a new leaf grammar because:
 *
 *     grammar/nano/protocols.g4
 *
 * does not currently exist in the inspected repository.
 *
 * Existing files are therefore not renamed.
 *
 * Existing ownership remains:
 *
 *     grammar/nano/agents.g4
 *     grammar/nano/atoms.g4
 *     grammar/nano/molecules.g4
 *     grammar/nano/materials.g4
 *
 * Existing generic networking protocol ownership remains:
 *
 *     grammar/networking/protocols.g4
 *
 * The new grammar MUST NOT replace or rename those files.
 *
 * ============================================================================
 * 86. ROOT PARSER INTEGRATION
 * ============================================================================
 *
 * The canonical composition root remains:
 *
 *     grammar/Zamani.g4
 *
 * and the canonical parser orchestration remains:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * Integration should expose:
 *
 *     nanoElement
 *         |
 *         +-- nanoProtocolConstruct
 *
 * or the equivalent existing nano-domain dispatcher.
 *
 * This file must NOT become another parser composition root.
 *
 * ============================================================================
 * 87. NANO DOMAIN DISPATCHER INTEGRATION
 * ============================================================================
 *
 * The nano domain should ultimately compose:
 *
 *     NanoAgents
 *     NanoAtoms
 *     NanoMolecules
 *     NanoMaterials
 *     NanoInteractions
 *     NanoProtocols
 *     NanoCapabilities
 *     future nano-domain components
 *
 * Conceptually:
 *
 *     nanoConstruct
 *         : nanoAgentConstruct
 *         | nanoAtomDeclaration
 *         | nanoMoleculeConstruct
 *         | nanoMaterialConstruct
 *         | nanoInteractionConstruct
 *         | nanoProtocolConstruct
 *         | nanoCapabilityConstruct
 *         ;
 *
 * The exact dispatcher rule remains owned by the nano/root parser composition
 * layer, not by this leaf grammar.
 *
 * ============================================================================
 * 88. grammar/grammar.md INTEGRATION
 * ============================================================================
 *
 * Presence of this file alone MUST NOT cause grammar.md to report the feature
 * as fully IMPLEMENTED.
 *
 * The implementation-conformance lifecycle remains:
 *
 *     SPECIFIED
 *     IMPLEMENTED
 *     PARTIALLY IMPLEMENTED
 *     PLANNED
 *     DEPRECATED
 *
 * Full implementation requires:
 *
 *     lexer conformance;
 *     parser composition;
 *     AST lowering;
 *     semantic analysis;
 *     IR integration;
 *     compiler integration;
 *     runtime integration;
 *     positive tests;
 *     negative tests;
 *     boundary tests;
 *     scalability tests;
 *     compatibility tests.
 *
 * ============================================================================
 * 89. Zamani-Grammar.md INTEGRATION
 * ============================================================================
 *
 * grammar/Zamani-Grammar.md remains historical/extended/proposed language
 * material.
 *
 * It does not override this file.
 *
 * A nano protocol feature becomes stable only through:
 *
 *     proposal
 *       |
 *       v
 *     semantic design
 *       |
 *       v
 *     AST contract
 *       |
 *       v
 *     grammar
 *       |
 *       v
 *     implementation
 *       |
 *       v
 *     IR integration
 *       |
 *       v
 *     tests
 *       |
 *       v
 *     stable
 *
 * ============================================================================
 * 90. SPECIFICATION INTEGRATION
 * ============================================================================
 *
 * This grammar implements the nano-domain intent described by the repository's
 * specification material:
 *
 *     atoms;
 *     molecules;
 *     materials;
 *     interactions;
 *     nano-agents;
 *     nanoscale processes;
 *     protocols;
 *     capabilities.
 *
 * The grammar remains a structural representation of those concepts.
 *
 * Physical meaning is established by semantic/domain implementations.
 *
 * ============================================================================
 * 91. HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no universal resource constants such as:
 *
 *     MAX_NANO_PROTOCOLS
 *     MAX_NANO_ROLES
 *     MAX_NANO_PARTICIPANTS
 *     MAX_NANO_MESSAGES
 *     MAX_NANO_OPERATIONS
 *     MAX_NANO_STATES
 *     MAX_NANO_TRANSITIONS
 *     MAX_NANO_AGENTS
 *     MAX_NANO_DEVICES
 *     MAX_NANO_RESOURCES
 *     MAX_NANO_MEMORY
 *     MAX_NANO_ENERGY
 *     MAX_NANO_BANDWIDTH
 *     MAX_NANO_DEPTH
 *
 * It also contains no:
 *
 *     fixed atom catalogue;
 *     fixed molecule catalogue;
 *     fixed material catalogue;
 *     fixed protocol catalogue;
 *     fixed device catalogue;
 *     fixed vendor catalogue;
 *     fixed physical topology;
 *     fixed quantum gate catalogue.
 *
 * ============================================================================
 * 92. REQUIRED POSITIVE TESTS
 * ============================================================================
 *
 * The following forms should be represented in conformance tests:
 *
 * Minimal:
 *
 *     @protocol Delivery;
 *
 * Protocol body:
 *
 *     @protocol Delivery {
 *     }
 *
 * Roles:
 *
 *     @protocol Delivery {
 *         role sender;
 *         role receiver;
 *     }
 *
 * Participants:
 *
 *     @protocol Delivery {
 *         participant source: NanoAgent;
 *         participant target: NanoAgent;
 *     }
 *
 * Messages:
 *
 *     @protocol Delivery {
 *         message nano::Payload: Payload;
 *     }
 *
 * Events:
 *
 *     @protocol Delivery {
 *         event delivered(value: Payload);
 *     }
 *
 * Operations:
 *
 *     @protocol Delivery {
 *         operation deliver(
 *             value: Payload
 *         ) -> Result;
 *     }
 *
 * Requirements:
 *
 *     @protocol Delivery {
 *         requires capability::transport;
 *     }
 *
 * Capabilities:
 *
 *     @protocol Delivery {
 *         capability nano::communication;
 *     }
 *
 * Constraints:
 *
 *     @protocol Delivery {
 *         constraint reliability >= required;
 *     }
 *
 * Preferences:
 *
 *     @protocol Delivery {
 *         prefer capability::locality;
 *     }
 *
 * States:
 *
 *     @protocol Delivery {
 *         state idle;
 *         state active;
 *         state complete;
 *     }
 *
 * Transitions:
 *
 *     @protocol Delivery {
 *         transition idle -> active;
 *         transition active -> complete on delivered;
 *     }
 *
 * Flow:
 *
 *     @protocol Delivery {
 *         flow execute {
 *             observe source;
 *             act target;
 *             emit delivered;
 *         }
 *     }
 *
 * Parameterization:
 *
 *     @protocol Delivery<T: Payload> {
 *     }
 *
 * Refinement:
 *
 *     @protocol SecureDelivery extends Delivery {
 *         requires security::authentication;
 *     }
 *
 * Nested composition:
 *
 *     @protocol Composite {
 *         @protocol Inner {
 *             state ready;
 *         }
 *     }
 *
 * Parallel:
 *
 *     @protocol Coordination {
 *         flow execute {
 *             parallel {
 *                 observe source;
 *                 observe target;
 *             }
 *         }
 *     }
 *
 * Repetition:
 *
 *     @protocol Monitoring {
 *         flow monitor {
 *             repeat condition {
 *                 observe sensor;
 *             }
 *         }
 *     }
 *
 * ============================================================================
 * 93. REQUIRED NEGATIVE TESTS
 * ============================================================================
 *
 * The following must be rejected where syntactically incomplete:
 *
 *     @protocol
 *
 *     @protocol {
 *     }
 *
 *     @protocol Name<
 *
 *     @protocol Name<T
 *
 *     @protocol Name extends
 *
 *     @protocol Name {
 *         role;
 *     }
 *
 *     @protocol Name {
 *         participant;
 *     }
 *
 *     @protocol Name {
 *         operation;
 *     }
 *
 *     @protocol Name {
 *         state;
 *     }
 *
 *     @protocol Name {
 *         transition idle;
 *     }
 *
 *     @protocol Name {
 *         requires;
 *     }
 *
 *     @protocol Name {
 *         capability;
 *     }
 *
 *     @protocol Name {
 *         property;
 *     }
 *
 *     @protocol Name {
 *         flow execute {
 *             send;
 *         }
 *     }
 *
 * Malformed nesting, missing delimiters, malformed type expressions and
 * malformed canonical expressions must likewise be rejected by the shared
 * parser rules.
 *
 * ============================================================================
 * 94. BOUNDARY TESTS
 * ============================================================================
 *
 * Tests must include:
 *
 *     one role;
 *     many roles;
 *     one participant;
 *     many participants;
 *     one state;
 *     many states;
 *     many transitions;
 *     deeply qualified names;
 *     deeply nested flows;
 *     deeply nested protocols;
 *     many protocol parameters;
 *     many operation parameters;
 *     large expressions;
 *     large type expressions;
 *     empty protocol bodies;
 *     empty flow bodies;
 *     custom annotation names;
 *     custom operation names;
 *     custom event names;
 *     custom capability names;
 *     custom material names;
 *     custom molecule names;
 *     custom atom names;
 *     custom agent names.
 *
 * ============================================================================
 * 95. SCALABILITY TESTS
 * ============================================================================
 *
 * Generated tests should vary:
 *
 *     protocol count;
 *     role count;
 *     participant count;
 *     state count;
 *     transition count;
 *     operation count;
 *     requirement count;
 *     capability count;
 *     property count;
 *     flow depth;
 *     flow step count;
 *     nested protocol depth;
 *     qualified-name depth;
 *     expression size;
 *     type-expression size.
 *
 * The tests MUST NOT define a maximum as a language rule.
 *
 * The practical maximum is determined by implementation resources.
 *
 * ============================================================================
 * 96. CROSS-DOMAIN TESTS
 * ============================================================================
 *
 * At minimum:
 *
 *     nano + classical;
 *     nano + quantum;
 *     nano + hybrid;
 *     nano + HDL;
 *     nano + hardware;
 *     nano + distributed;
 *     nano + networking;
 *     nano + security;
 *     nano + resources;
 *     nano + interoperability;
 *     nano + AI;
 *     nano + data.
 *
 * The same protocol source must remain syntactically target-independent.
 *
 * ============================================================================
 * 97. DETERMINISM TESTS
 * ============================================================================
 *
 * Given identical:
 *
 *     source;
 *     language version;
 *     lexer vocabulary;
 *     grammar version;
 *
 * parsing must produce equivalent parse structures.
 *
 * Tests must not depend on:
 *
 *     hardware;
 *     network state;
 *     filesystem state;
 *     runtime state;
 *     resource availability;
 *     randomness.
 *
 * ============================================================================
 * 98. RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains no Rust code.
 *
 * The generated parser/frontend integration must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and:
 *
 *     safe Rust only;
 *     no unsafe.
 *
 * No parser action in this grammar may require unsafe Rust.
 *
 * ============================================================================
 * 99. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is grammatically complete when:
 *
 *   [x] It has one parser grammar identity: NanoProtocols.
 *   [x] It consumes the canonical ZamaniLexer.
 *   [x] It imports Names.
 *   [x] It imports Types.
 *   [x] It imports Expressions.
 *   [x] It imports Statements.
 *   [x] It exposes one public nanoProtocolConstruct entry point.
 *   [x] It does not define lexer rules.
 *   [x] It does not define a second identifier grammar.
 *   [x] It does not define a second type system.
 *   [x] It does not define a second expression language.
 *   [x] It does not duplicate nano-agent syntax.
 *   [x] It does not duplicate atom syntax.
 *   [x] It does not duplicate molecular syntax.
 *   [x] It does not duplicate material syntax.
 *   [x] It distinguishes nano protocols from networking protocols.
 *   [x] It supports protocol parameterization.
 *   [x] It supports protocol refinement.
 *   [x] It supports logical roles.
 *   [x] It supports logical participants.
 *   [x] It supports message references.
 *   [x] It supports events.
 *   [x] It supports operation contracts.
 *   [x] It supports requirements.
 *   [x] It supports capabilities.
 *   [x] It supports constraints.
 *   [x] It supports preferences.
 *   [x] It supports properties.
 *   [x] It supports abstract states.
 *   [x] It supports transitions.
 *   [x] It supports flow composition.
 *   [x] It supports send/receive/emit.
 *   [x] It supports observe/act.
 *   [x] It supports sequence.
 *   [x] It supports choice.
 *   [x] It supports parallel composition.
 *   [x] It supports repetition.
 *   [x] It supports optional behavior.
 *   [x] It supports waiting.
 *   [x] It supports guards.
 *   [x] It supports nested protocol composition.
 *   [x] It supports open-world names.
 *   [x] It imposes no physical resource ceiling.
 *   [x] It imposes no hardware ceiling.
 *   [x] It imposes no nano-device ceiling.
 *   [x] It imposes no agent ceiling.
 *   [x] It imposes no atom/molecule/material ceiling.
 *   [x] It does not select physical targets.
 *   [x] It does not allocate resources.
 *   [x] It does not perform networking.
 *   [x] It does not perform physical simulation.
 *   [x] It does not perform quantum execution.
 *   [x] It preserves the canonical quantum::ir boundary.
 *   [x] It contains no embedded Rust.
 *   [x] It requires no unsafe Rust.
 *   [x] It defines AST integration.
 *   [x] It defines semantic integration.
 *   [x] It defines IR integration.
 *   [x] It defines compiler integration.
 *   [x] It defines runtime integration.
 *   [x] It defines compatibility requirements.
 *   [x] It defines positive/negative/boundary/scalability tests.
 *
 * Repository integration is complete only after the downstream parser,
 * frontend AST, semantic layer, IR, compiler, runtime and tests consume this
 * contract.
 *
 * ============================================================================
 * 100. FINAL ARCHITECTURAL INVARIANT
 * ============================================================================
 *
 * This file answers:
 *
 *     "What nano-domain protocol structures are syntactically expressible?"
 *
 * It does NOT answer:
 *
 *     "Which physical nano-device implements the protocol?"
 *     "Which molecule is physically manipulated?"
 *     "Which material database is authoritative?"
 *     "Which sensor is selected?"
 *     "Which actuator is selected?"
 *     "Which CPU/GPU/FPGA/QPU executes the protocol?"
 *     "Which network is used?"
 *     "Which node executes it?"
 *     "How is it routed?"
 *     "How is it scheduled?"
 *     "How is it calibrated?"
 *     "How is it simulated?"
 *     "How is QEC performed?"
 *     "How is ZQN evaluated?"
 *
 * Those responsibilities belong downstream.
 *
 * The complete POCO-REAF architecture remains:
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     canonical parser
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     canonical semantic model
 *          |
 *          v
 *     canonical IR
 *          |
 *          +-----------------------+
 *          |                       |
 *          v                       v
 *      classical               quantum::ir
 *          |                       |
 *          +-----------+-----------+
 *                      |
 *                      v
 *              optimization
 *                      |
 *              routing / scheduling
 *                      |
 *              resilience / QEC
 *                      |
 *                     ZQN
 *                      |
 *                     HAL
 *                      |
 *                      v
 *              target realization
 *
 * Therefore:
 *
 *     Program Once
 *          ->
 *     Compile Once
 *          ->
 *     Run Everywhere
 *          ->
 *     Run Anywhere
 *          ->
 *     Run Forever
 *
 * subject to the actual semantics of the program, declared requirements,
 * target capabilities, and resources available to the realization.
 *
 * ============================================================================
 */
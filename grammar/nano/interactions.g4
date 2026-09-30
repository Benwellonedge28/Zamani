/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/nano/interactions.g4
 *
 * GRAMMAR
 * -------
 * NanoInteractions
 *
 * STATUS
 * ------
 * CANONICAL NANO-DOMAIN INTERACTION GRAMMAR
 *
 * LANGUAGE
 * --------
 * Zamani
 *
 * COMPILER BASELINE
 * -----------------
 * Rust 1.97 / Rust 1.97.1
 * Rust 2021
 * Safe Rust only
 * No unsafe Rust
 *
 * ============================================================================
 * 1. PURPOSE
 * ============================================================================
 *
 * This grammar owns the SOURCE-LEVEL STRUCTURE of nano-domain interactions.
 *
 * An interaction represents a relationship, communication, transformation,
 * dependency, coupling, transfer, observation, actuation, or other
 * computational relationship between source-level nano-domain entities.
 *
 * The grammar is intentionally OPEN-WORLD.
 *
 * It does not enumerate:
 *
 *     - atoms;
 *     - molecules;
 *     - materials;
 *     - particles;
 *     - agents;
 *     - sensors;
 *     - actuators;
 *     - protocols;
 *     - interaction kinds;
 *     - physical forces;
 *     - chemical reactions;
 *     - biological mechanisms;
 *     - device models;
 *     - vendor mechanisms;
 *     - simulation engines;
 *     - hardware implementations.
 *
 * Those meanings belong to semantic analysis, domain models, libraries,
 * capabilities, resource analysis, simulation infrastructure, and/or
 * downstream target realization.
 *
 * ============================================================================
 * 2. ARCHITECTURAL POSITION
 * ============================================================================
 *
 * Zamani source
 *      |
 *      v
 * canonical Zamani lexer
 *      |
 *      v
 * canonical Zamani parser
 *      |
 *      v
 * NanoInteractions
 *      |
 *      v
 * domain-neutral frontend AST
 *      |
 *      v
 * structural validation
 *      |
 *      v
 * semantic analysis
 *      |
 *      +--> name resolution
 *      +--> type analysis
 *      +--> interaction validation
 *      +--> endpoint analysis
 *      +--> capability analysis
 *      +--> resource analysis
 *      +--> effect analysis
 *      +--> security analysis
 *      +--> temporal analysis
 *      +--> quantum analysis where applicable
 *      +--> classical analysis where applicable
 *      +--> hardware analysis where applicable
 *      |
 *      v
 * canonical semantic representation
 *      |
 *      v
 * canonical IR
 *      |
 *      +--> classical computation
 *      +--> quantum::ir where applicable
 *      +--> HDL/hardware representation where applicable
 *      +--> distributed representation where applicable
 *      +--> future domain representations
 *      |
 *      v
 * optimization / lowering / simulation
 *      |
 *      v
 * resource selection / routing / scheduling / resilience
 *      |
 *      v
 * target realization
 *
 * This grammar MUST NOT create a second semantic IR.
 *
 * In particular, interaction syntax that eventually represents quantum
 * computation MUST flow through the existing canonical:
 *
 *     quantum::ir
 *
 * boundary.
 *
 * ============================================================================
 * 3. OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     nanoInteractionConstruct
 *     nanoInteractionAnnotatedConstruct
 *     nanoInteractionAnnotation
 *     nanoInteractionTail
 *     nanoInteractionDirectInvocation
 *     nanoInteractionPostInvocation
 *     nanoInteractionDeclaration
 *     nanoInteractionReference
 *     nanoInteractionDeclarationTail
 *     nanoInteractionNamedInvocation
 *     nanoInteractionTypedDeclaration
 *     nanoInteractionTypeClause
 *     nanoInteractionInitializer
 *     nanoInteractionBody
 *     nanoInteractionMember
 *     nanoInteractionEndpoint
 *     nanoInteractionEndpointAnnotation
 *     nanoInteractionRelation
 *     nanoInteractionCondition
 *     nanoInteractionRequirement
 *     nanoInteractionConstraint
 *     nanoInteractionCapability
 *     nanoInteractionPreference
 *     nanoInteractionMetadata
 *     nanoInteractionExpressionBinding
 *     nanoInteractionNestedConstruct
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical token definitions;
 *     - identifiers;
 *     - qualified-name resolution;
 *     - expression precedence;
 *     - general expressions;
 *     - general types;
 *     - statements;
 *     - functions;
 *     - modules;
 *     - atom declarations;
 *     - molecule declarations;
 *     - material declarations;
 *     - agent declarations;
 *     - protocol implementations;
 *     - physical chemistry;
 *     - physics models;
 *     - quantum operation definitions;
 *     - quantum state representation;
 *     - hardware topology;
 *     - resource discovery;
 *     - device discovery;
 *     - allocation;
 *     - placement;
 *     - routing;
 *     - scheduling;
 *     - calibration;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime execution;
 *     - simulation algorithms;
 *     - compiler implementation.
 *
 * ============================================================================
 * 4. CANONICAL DEPENDENCIES
 * ============================================================================
 *
 * Canonical lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Required parser contracts:
 *
 *     Names
 *     Expressions
 *     Types
 *     Statements
 *
 * Reused canonical rules include:
 *
 *     identifier
 *     qualifiedName
 *     expression
 *     argumentList
 *     typeExpression
 *     statement
 *
 * This file MUST NOT redefine any of those rules.
 *
 * ============================================================================
 * 5. NO NEW REQUIRED LEXER VOCABULARY
 * ============================================================================
 *
 * This component deliberately does not require an INTERACTION lexer keyword.
 *
 * The existing lexer vocabulary already contains:
 *
 *     AT
 *     LPAREN
 *     RPAREN
 *     LBRACE
 *     RBRACE
 *     SEMICOLON
 *     COLON
 *     ASSIGN
 *     COMMA
 *
 * Interaction remains an ordinary source-level annotation name:
 *
 *     @interaction
 *
 * This is intentional.
 *
 * It prevents every future nano interaction concept from requiring a new
 * reserved keyword.
 *
 * Examples of valid semantic annotation names include:
 *
 *     @interaction
 *     @coupling
 *     @transfer
 *     @communication
 *     @transport
 *     @reaction
 *     @binding
 *     @observation
 *     @actuation
 *     @coordination
 *
 * The parser does not assign scientific meaning to these names.
 *
 * Semantic analysis and registered domain definitions own that meaning.
 *
 * ============================================================================
 * 6. OPEN-WORLD INTERACTION MODEL
 * ============================================================================
 *
 * The grammar MUST NOT contain a closed interaction enumeration such as:
 *
 *     interactionType
 *         : collision
 *         | reaction
 *         | transport
 *         | ...
 *
 * Such an enumeration would make future nano science, computational models,
 * biological systems, quantum systems, and hardware technologies depend on
 * parser changes.
 *
 * Instead, interaction names and metadata are represented by ordinary
 * identifiers and expressions.
 *
 * Therefore a semantic implementation may introduce:
 *
 *     interaction("transport")
 *     interaction("binding")
 *     interaction("reaction")
 *     interaction("measurement")
 *     interaction("coupling")
 *     interaction("communication")
 *
 * without modifying this grammar.
 *
 * ============================================================================
 * 7. POCO-REAF AND SCALABILITY
 * ============================================================================
 *
 * This grammar imposes NO universal maximum on:
 *
 *     interactions
 *     endpoints
 *     participants
 *     arguments
 *     parameters
 *     properties
 *     annotations
 *     nested interactions
 *     interaction members
 *     interaction graphs
 *     interaction chains
 *     interaction dependencies
 *     interaction declarations
 *     interaction instances
 *     interaction references
 *     source program size
 *     interaction graph size
 *
 * It MUST NOT introduce constants such as:
 *
 *     MAX_INTERACTIONS
 *     MAX_INTERACTION_ENDPOINTS
 *     MAX_INTERACTION_PARTICIPANTS
 *     MAX_INTERACTION_PARAMETERS
 *     MAX_INTERACTION_DEPTH
 *     MAX_INTERACTION_GRAPH_SIZE
 *     MAX_NANO_INTERACTIONS
 *     MAX_NANO_PARTICIPANTS
 *
 * or equivalent universal language limits.
 *
 * Repetition uses ordinary ANTLR cardinality:
 *
 *     *
 *     +
 *
 * Actual implementation limits are downstream resource concerns.
 *
 * The absence of a grammar limit does NOT imply that a compiler or runtime
 * must allocate unlimited memory or accept unlimited computational work.
 *
 * Implementations MAY apply explicit resource policies for:
 *
 *     parser input size;
 *     recursion depth;
 *     compilation resources;
 *     memory;
 *     execution time;
 *     graph processing;
 *     simulation resources.
 *
 * Such implementation policies MUST NOT become hidden language semantics.
 *
 * ============================================================================
 * 8. POCO-REAF RESOURCE SEPARATION
 * ============================================================================
 *
 * Interaction source code describes WHAT relationship is intended.
 *
 * It does not decide:
 *
 *     which CPU executes it;
 *     which GPU executes it;
 *     which FPGA implements it;
 *     which ASIC realizes it;
 *     which QPU realizes it;
 *     which physical atom participates;
 *     which physical molecule participates;
 *     which device is selected;
 *     which network node is selected;
 *     which memory bank is selected;
 *     which physical link is selected;
 *     which hardware topology is used.
 *
 * Source programs MAY express:
 *
 *     requirements;
 *     constraints;
 *     capabilities;
 *     preferences;
 *     hints;
 *     budgets;
 *     policies.
 *
 * Those declarations describe intent.
 *
 * Target realization belongs downstream.
 *
 * ============================================================================
 * 9. INTERACTION ENDPOINT MODEL
 * ============================================================================
 *
 * An interaction may involve:
 *
 *     one endpoint;
 *     multiple endpoints;
 *     an ordered endpoint list;
 *     a symbolic collection;
 *     an expression selecting participants;
 *     a dynamically determined participant set.
 *
 * The grammar therefore does not encode binary-only interactions.
 *
 * For example, these structures are representable:
 *
 *     @interaction observe(sensor);
 *
 *     @interaction bind(source, target);
 *
 *     @interaction couple(a, b, c);
 *
 *     @interaction synchronize(group);
 *
 *     @interaction transform(input, intermediate, output);
 *
 * Semantic analysis determines:
 *
 *     endpoint roles;
 *     cardinality;
 *     ordering;
 *     direction;
 *     type compatibility;
 *     ownership;
 *     lifetime;
 *     capabilities;
 *     resource requirements.
 *
 * ============================================================================
 * 10. ENDPOINT ROLE MODEL
 * ============================================================================
 *
 * Endpoint roles are deliberately open-ended.
 *
 * Interaction bodies MAY use annotations such as:
 *
 *     @source(x);
 *     @target(y);
 *     @control(c);
 *     @input(x);
 *     @output(y);
 *     @observer(sensor);
 *     @participant(entity);
 *
 * The grammar does not reserve these names.
 *
 * Semantic analysis decides whether a particular endpoint-role annotation is
 * registered and valid.
 *
 * This permits future interaction models without lexer expansion.
 *
 * ============================================================================
 * 11. INTERACTION DECLARATION FORMS
 * ============================================================================
 *
 * The primary form is annotation based:
 *
 *     @interaction transport(source, destination);
 *
 *     @interaction transport(source, destination) {
 *         ...
 *     }
 *
 * A typed form is also permitted:
 *
 *     @interaction transport: Interaction;
 *
 *     @interaction transport: Interaction = implementation;
 *
 * A parameterized declaration may use ordinary expressions:
 *
 *     @interaction transport(rate, mode) {
 *         ...
 *     }
 *
 * The parser does not determine whether an argument represents:
 *
 *     a participant;
 *     a parameter;
 *     a property;
 *     a capability;
 *     a resource;
 *     a value.
 *
 * Semantic analysis determines those roles.
 *
 * ============================================================================
 * 12. DIRECT INTERACTION INVOCATION
 * ============================================================================
 *
 * The annotation may be used directly:
 *
 *     @interaction(source, destination);
 *
 *     @interaction(source, destination) {
 *         ...
 *     }
 *
 * This is useful for anonymous or immediately evaluated interaction intent.
 *
 * The grammar does not require every interaction instance to have a name.
 *
 * ============================================================================
 * 13. NAMED INTERACTION
 * ============================================================================
 *
 * A named interaction has an ordinary identifier following the annotation:
 *
 *     @interaction transport(source, destination);
 *
 *     @interaction coupling(a, b) {
 *         ...
 *     }
 *
 * The name remains an ordinary identifier.
 *
 * No fixed interaction registry is embedded in the grammar.
 *
 * ============================================================================
 * 14. INTERACTION BODY
 * ============================================================================
 *
 * Interaction bodies contain interaction-specific members and ordinary
 * Zamani statements.
 *
 * This permits an interaction to participate in:
 *
 *     classical computation;
 *     quantum computation;
 *     AI computation;
 *     hardware/software co-design;
 *     networking;
 *     concurrency;
 *     distributed execution;
 *     memory operations;
 *     security policies;
 *     future domains.
 *
 * The interaction grammar does not create a separate language inside the
 * interaction body.
 *
 * ============================================================================
 * 15. REQUIREMENTS / CONSTRAINTS / CAPABILITIES
 * ============================================================================
 *
 * Interaction declarations may contain open-world metadata describing:
 *
 *     requirements;
 *     constraints;
 *     capabilities;
 *     preferences.
 *
 * These are syntactic structures only.
 *
 * Semantic analysis determines their meaning.
 *
 * The compiler may later use them for:
 *
 *     resource selection;
 *     target selection;
 *     scheduling;
 *     routing;
 *     simulation;
 *     verification;
 *     deployment.
 *
 * ============================================================================
 * 16. QUANTUM INTEGRATION
 * ============================================================================
 *
 * An interaction may refer to quantum entities or quantum computation.
 *
 * This grammar MUST NOT define:
 *
 *     quantum gates;
 *     physical qubits;
 *     coupling maps;
 *     pulse schedules;
 *     calibration;
 *     QEC;
 *     ZQN;
 *     routing;
 *     physical placement.
 *
 * Example:
 *
 *     @interaction coupling(source, target) {
 *         @requires(capability("quantum.interaction"));
 *         ...
 *     }
 *
 * If semantic analysis determines that the interaction represents quantum
 * computation, the resulting validated semantic operation MUST flow through
 * the existing:
 *
 *     quantum::ir
 *
 * boundary.
 *
 * ============================================================================
 * 17. CLASSICAL / HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * An interaction may connect:
 *
 *     software;
 *     classical computation;
 *     quantum computation;
 *     HDL constructs;
 *     hardware resources;
 *     accelerators;
 *     distributed nodes;
 *     networking endpoints.
 *
 * The grammar remains domain-neutral.
 *
 * Hardware identity, topology, placement, timing realization, synthesis, and
 * device selection remain downstream responsibilities.
 *
 * ============================================================================
 * 18. CONCURRENCY AND DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Interactions may be represented inside concurrent or distributed programs.
 *
 * The grammar does not assume:
 *
 *     thread;
 *     process;
 *     node;
 *     actor;
 *     device;
 *     network link.
 *
 * Semantic analysis may derive:
 *
 *     data dependencies;
 *     control dependencies;
 *     synchronization dependencies;
 *     resource dependencies;
 *     ownership dependencies;
 *     communication dependencies.
 *
 * Scheduling and placement remain downstream.
 *
 * ============================================================================
 * 19. TEMPORAL / MULTI-TIMELINE INTEGRATION
 * ============================================================================
 *
 * An interaction may participate in:
 *
 *     temporal computation;
 *     speculative computation;
 *     rewindable computation;
 *     multi-timeline computation;
 *     event-driven computation.
 *
 * This file does not own timeline semantics.
 *
 * It only preserves interaction structure.
 *
 * Timeline ownership remains with the appropriate execution/timeline
 * grammar and semantic infrastructure.
 *
 * ============================================================================
 * 20. SECURITY INTEGRATION
 * ============================================================================
 *
 * Interaction metadata may express security intent:
 *
 *     @interaction transfer(source, target) {
 *         @requires(capability("secure.communication"));
 *         @policy(policy_expression);
 *     }
 *
 * The grammar does not implement:
 *
 *     cryptography;
 *     authentication;
 *     authorization;
 *     key management;
 *     secure transport;
 *     trusted execution.
 *
 * Those remain owned by security and downstream execution infrastructure.
 *
 * ============================================================================
 * 21. DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no embedded Rust;
 *     no actions;
 *     no semantic predicates;
 *     no filesystem access;
 *     no network access;
 *     no hardware discovery;
 *     no randomness;
 *     no runtime execution.
 *
 * Parsing depends only on:
 *
 *     source text;
 *     canonical token vocabulary;
 *     imported parser rules;
 *     selected grammar version/dialect configuration.
 *
 * ============================================================================
 * 22. SAFETY
 * ============================================================================
 *
 * This grammar requires no unsafe Rust.
 *
 * The Rust implementation consuming/generated from this grammar MUST remain:
 *
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust
 *
 * The grammar itself cannot introduce unsafe behavior because it contains no
 * executable Rust actions.
 *
 * ============================================================================
 * 23. AST CONTRACT
 * ============================================================================
 *
 * The frontend AST remains owned by:
 *
 *     src/frontend/ast/
 *
 * This grammar MUST NOT introduce a competing nano interaction AST.
 *
 * The parser must preserve enough structure to represent conceptually:
 *
 *     interaction annotation;
 *     interaction name;
 *     invocation arguments;
 *     type;
 *     initializer;
 *     endpoint expressions;
 *     endpoint annotations;
 *     interaction members;
 *     nested constructs;
 *     requirements;
 *     constraints;
 *     capabilities;
 *     preferences;
 *     metadata;
 *     ordinary statements;
 *     source spans.
 *
 * A conceptual semantic shape is:
 *
 *     Interaction {
 *         annotation,
 *         name,
 *         arguments,
 *         type,
 *         initializer,
 *         members,
 *         source_span
 *     }
 *
 * The exact Rust type is determined by the existing domain-neutral AST.
 *
 * ============================================================================
 * 24. SOURCE SPANS
 * ============================================================================
 *
 * The implementation MUST preserve source locations for at least:
 *
 *     @interaction;
 *     interaction name;
 *     argument expressions;
 *     type expression;
 *     initializer;
 *     endpoint annotations;
 *     endpoint expressions;
 *     member annotations;
 *     requirement expressions;
 *     constraint expressions;
 *     capability expressions;
 *     preference expressions;
 *     metadata;
 *     nested constructs.
 *
 * The grammar itself does not construct spans.
 *
 * The parser/AST implementation must attach the source information.
 *
 * ============================================================================
 * 25. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis MUST determine:
 *
 *     - whether the interaction name resolves;
 *     - whether the interaction declaration is valid;
 *     - whether arguments are valid;
 *     - whether endpoint expressions are valid;
 *     - whether endpoint types are compatible;
 *     - whether endpoint roles are valid;
 *     - whether a direction is meaningful;
 *     - whether a relationship is supported;
 *     - whether capabilities are available;
 *     - whether resource requirements can be satisfied;
 *     - whether security constraints are satisfied;
 *     - whether quantum semantics apply;
 *     - whether classical semantics apply;
 *     - whether hardware semantics apply;
 *     - whether the interaction is executable, simulatable, symbolic, or
 *       otherwise supported.
 *
 * None of these decisions belong in this grammar.
 *
 * ============================================================================
 * 26. IR CONTRACT
 * ============================================================================
 *
 * This grammar does not create an IR.
 *
 * A validated interaction follows:
 *
 *     source
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic interaction model
 *       |
 *       +--> classical IR where applicable
 *       +--> quantum::ir where applicable
 *       +--> HDL/hardware IR where applicable
 *       +--> distributed IR where applicable
 *       +--> other canonical IR representations
 *
 * No interaction-specific parallel IR is permitted merely because the source
 * domain is nano.
 *
 * ============================================================================
 * 27. ERROR BOUNDARIES
 * ============================================================================
 *
 * STRUCTURAL errors belong to parsing.
 *
 * Examples:
 *
 *     @interaction
 *     @interaction(
 *     @interaction transport(
 *     @interaction transport)
 *     @interaction transport(
 *     @interaction transport {
 *
 * SEMANTIC errors belong to semantic analysis.
 *
 * Examples:
 *
 *     unknown interaction;
 *     invalid endpoint type;
 *     incompatible participants;
 *     unavailable capability;
 *     impossible physical requirement;
 *     unsupported quantum interaction;
 *     invalid material relationship.
 *
 * RESOURCE errors belong to resource analysis/runtime.
 *
 * Examples:
 *
 *     insufficient memory;
 *     insufficient compute;
 *     unavailable device;
 *     unavailable network capacity.
 *
 * TARGET errors belong to compilation/deployment.
 *
 * Examples:
 *
 *     no compatible target;
 *     unsupported backend;
 *     unavailable accelerator.
 *
 * These categories MUST NOT be collapsed into syntax errors.
 *
 * ============================================================================
 * 28. COMPATIBILITY
 * ============================================================================
 *
 * This grammar is an additive leaf component.
 *
 * It does not rename:
 *
 *     grammar/nano/agents.g4
 *     grammar/nano/atoms.g4
 *     grammar/nano/molecules.g4
 *     grammar/nano/materials.g4
 *     grammar/Zamani.g4
 *
 * Existing nano-agent interaction syntax remains owned by:
 *
 *     NanoAgents
 *
 * This grammar owns reusable nano-domain interaction declarations and
 * constructs that are not merely an internal agent-member construct.
 *
 * A future migration MUST preserve source compatibility or explicitly record
 * the required compatibility rule.
 *
 * ============================================================================
 * 29. INTEGRATION WITH EXISTING NANO GRAMMARS
 * ============================================================================
 *
 * NanoAgents
 * ----------
 *
 * grammar/nano/agents.g4 already contains:
 *
 *     nanoAgentInteraction
 *
 * That rule should remain the owner of interaction syntax that is structurally
 * local to an agent declaration.
 *
 * NanoInteractions is the reusable domain-level interaction grammar.
 *
 * If the canonical parser needs one shared interaction representation, the
 * parser composition layer SHOULD delegate both forms to this grammar after
 * resolving the ownership boundary.
 *
 * Do NOT create duplicate rules with the same semantic ownership.
 *
 * NanoAtoms
 * ---------
 *
 * Atomic constructs may reference or participate in interactions.
 *
 * NanoAtoms remains the owner of atom declarations.
 *
 * NanoMolecules
 * ------------
 *
 * Molecular constructs may reference or participate in interactions.
 *
 * NanoMolecules remains the owner of molecular declarations.
 *
 * NanoMaterials
 * -------------
 *
 * Material constructs may reference or participate in interactions.
 *
 * NanoMaterials remains the owner of material declarations.
 *
 * NanoAgents
 * ----------
 *
 * Agent declarations may contain local interaction constructs.
 *
 * NanoAgents remains the owner of agent declarations and agent-local
 * interaction dispatch.
 *
 * ============================================================================
 * 30. ROOT PARSER INTEGRATION
 * ============================================================================
 *
 * grammar/Zamani.g4 remains unchanged as the ANTLR root composition boundary.
 *
 * It already delegates actual parser composition to:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * Therefore this file MUST NOT be imported directly by:
 *
 *     grammar/Zamani.g4
 *
 * The intended dependency is:
 *
 *     Zamani.g4
 *          |
 *          v
 *     ZamaniParser.g4
 *          |
 *          v
 *     NanoInteractions
 *
 * The canonical parser composition layer should import this grammar once and
 * expose its public entry point through the appropriate nano dispatcher.
 *
 * ============================================================================
 * 31. LEXER INTEGRATION
 * ============================================================================
 *
 * No lexer change is required merely to create this grammar.
 *
 * The existing canonical lexer vocabulary supplies:
 *
 *     AT
 *     LPAREN
 *     RPAREN
 *     LBRACE
 *     RBRACE
 *     SEMICOLON
 *     COLON
 *     ASSIGN
 *     COMMA
 *
 * The word "interaction" remains an ordinary identifier.
 *
 * Therefore this file MUST NOT add:
 *
 *     INTERACTION
 *
 * merely to support this grammar.
 *
 * ============================================================================
 * 32. NO DUPLICATE GENERAL RULES
 * ============================================================================
 *
 * This file MUST NOT define:
 *
 *     identifier
 *     qualifiedName
 *     expression
 *     typeExpression
 *     argumentList
 *     statement
 *     block
 *
 * again.
 *
 * All such concepts belong to canonical shared grammars.
 *
 * ============================================================================
 * 33. VALIDATION REQUIREMENTS
 * ============================================================================
 *
 * ANTLR validation MUST verify:
 *
 *     - grammar generation;
 *     - import resolution;
 *     - token vocabulary resolution;
 *     - no undefined parser rules;
 *     - no duplicate rule names;
 *     - no accidental left recursion;
 *     - no unreachable public rules;
 *     - no accidental ambiguity introduced into the composition layer.
 *
 * Repository validation MUST additionally verify:
 *
 *     - AST coverage;
 *     - semantic coverage;
 *     - IR coverage;
 *     - diagnostics;
 *     - source spans;
 *     - compatibility;
 *     - scalability;
 *     - hard-coding audit.
 *
 * ============================================================================
 * 34. POSITIVE CONFORMANCE EXAMPLES
 * ============================================================================
 *
 * The following forms are structurally supported:
 *
 *     @interaction(source, target);
 *
 *     @interaction(source, target) {
 *     }
 *
 *     @interaction transport(source, target);
 *
 *     @interaction transport(source, target) {
 *     }
 *
 *     @interaction coupling(a, b, c);
 *
 *     @interaction transfer(input, output) {
 *         @source(input);
 *         @target(output);
 *     }
 *
 *     @interaction observation(sensor, sample) {
 *         @observer(sensor);
 *         @participant(sample);
 *     }
 *
 *     @interaction transport: Interaction;
 *
 *     @interaction transport: Interaction = implementation;
 *
 *     @interaction transport(rate, mode) {
 *         requires capability("nano.interaction");
 *     }
 *
 * The semantic implementation determines whether each construct has a valid
 * domain meaning.
 *
 * ============================================================================
 * 35. NEGATIVE CONFORMANCE EXAMPLES
 * ============================================================================
 *
 * Structurally invalid:
 *
 *     @interaction(
 *
 *     @interaction transport(
 *
 *     @interaction transport {
 *
 *     @interaction transport: ;
 *
 *     @interaction transport =
 *
 *     @interaction transport(
 *         source,
 *     ;
 *
 * Semantically invalid examples must be rejected downstream rather than by
 * this grammar:
 *
 *     @interaction unknown_entity(source, target);
 *
 *     @interaction invalid_types(source, incompatible_target);
 *
 *     @interaction impossible_requirement(source, target) {
 *         ...
 *     }
 *
 * ============================================================================
 * 36. BOUNDARY CONFORMANCE
 * ============================================================================
 *
 * Tests MUST cover:
 *
 *     zero interaction declarations;
 *     one interaction;
 *     many interactions;
 *     one endpoint;
 *     multiple endpoints;
 *     deeply nested bodies;
 *     empty bodies;
 *     parameterized interactions;
 *     typed interactions;
 *     initialized interactions;
 *     annotations;
 *     nested constructs;
 *     symbolic endpoint expressions;
 *     large endpoint lists;
 *     large interaction graphs;
 *     cross-domain interactions.
 *
 * No test may establish a universal maximum.
 *
 * ============================================================================
 * 37. SCALABILITY CONFORMANCE
 * ============================================================================
 *
 * The grammar must remain structurally valid as the number of:
 *
 *     interactions;
 *     participants;
 *     endpoints;
 *     parameters;
 *     annotations;
 *     members;
 *     nested constructs
 *
 * increases.
 *
 * Scalability tests must distinguish:
 *
 *     language expressiveness
 *     from
 *     implementation resource capacity.
 *
 * An implementation may fail safely due to actual resource exhaustion.
 *
 * Such failure MUST NOT imply that the language grammar has a semantic
 * maximum.
 *
 * ============================================================================
 * 38. HARD-CODING AUDIT
 * ============================================================================
 *
 * This file MUST remain free of universal capacity constants including:
 *
 *     MAX_INTERACTIONS
 *     MAX_ENDPOINTS
 *     MAX_PARTICIPANTS
 *     MAX_PARAMETERS
 *     MAX_DEPTH
 *     MAX_GRAPH_SIZE
 *     MAX_ATOMS
 *     MAX_MOLECULES
 *     MAX_MATERIALS
 *     MAX_AGENTS
 *     MAX_NANO_DEVICES
 *
 * It also MUST NOT contain fixed:
 *
 *     element tables;
 *     material tables;
 *     interaction catalogues;
 *     physical topology;
 *     device IDs;
 *     hardware limits;
 *     qubit limits;
 *     register widths;
 *     memory capacities.
 *
 * ============================================================================
 * 39. SECURITY AUDIT
 * ============================================================================
 *
 * The grammar MUST NOT execute interaction expressions during parsing.
 *
 * It MUST NOT:
 *
 *     inspect hardware;
 *     inspect filesystem state;
 *     access network state;
 *     access secrets;
 *     invoke runtime functions;
 *     allocate physical resources;
 *     perform simulation;
 *     resolve devices.
 *
 * All such behavior belongs downstream.
 *
 * ============================================================================
 * 40. COMPLETION CRITERIA
 * ============================================================================
 *
 * This grammar component is complete only when:
 *
 *     [ ] ANTLR generation succeeds.
 *     [ ] All imports resolve.
 *     [ ] Canonical ZamaniLexer vocabulary resolves.
 *     [ ] No duplicate lexer rules are introduced.
 *     [ ] No new interaction keyword is required.
 *     [ ] Public rule is exposed to canonical nano composition.
 *     [ ] AST mapping is implemented.
 *     [ ] Source spans are preserved.
 *     [ ] Semantic mapping is implemented.
 *     [ ] Interaction requirements are semantically classified.
 *     [ ] Endpoint semantics are implemented.
 *     [ ] Capability integration is implemented.
 *     [ ] Resource integration is implemented.
 *     [ ] Quantum interactions use quantum::ir where applicable.
 *     [ ] No second interaction-specific IR is introduced.
 *     [ ] Existing NanoAgents behavior remains compatible.
 *     [ ] Atom integration is tested.
 *     [ ] Molecule integration is tested.
 *     [ ] Material integration is tested.
 *     [ ] Agent integration is tested.
 *     [ ] Classical integration is tested.
 *     [ ] Quantum integration is tested.
 *     [ ] HDL/hardware integration is tested.
 *     [ ] Distributed integration is tested where applicable.
 *     [ ] Positive tests pass.
 *     [ ] Negative tests pass.
 *     [ ] Boundary tests pass.
 *     [ ] Scalability tests pass.
 *     [ ] Determinism tests pass.
 *     [ ] Compatibility tests pass.
 *     [ ] Hard-coding audit passes.
 *     [ ] Safe Rust requirement remains satisfied.
 *
 * ============================================================================
 * 41. FILE COMPLETION CONTRACT
 * ============================================================================
 *
 * Once this file satisfies the completion criteria, changes to unrelated
 * domains MUST NOT require editing this file merely because:
 *
 *     a new atom is added;
 *     a new molecule is added;
 *     a new material is added;
 *     a new agent is added;
 *     a new interaction type is introduced;
 *     a new quantum operation is introduced;
 *     a new hardware target is introduced;
 *     a new accelerator is introduced;
 *     a new distributed backend is introduced.
 *
 * New semantic capabilities should be registered downstream.
 *
 * This is the principal independence guarantee of this grammar component.
 *
 * ============================================================================
 */

parser grammar NanoInteractions;

options {
    tokenVocab = ZamaniLexer;
}

import
    Names,
    Expressions,
    Types,
    Statements
;


/*
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * The canonical nano parser dispatcher calls:
 *
 *     nanoInteractionConstruct
 *
 * This rule consumes exactly one interaction construct.
 *
 * It does not consume EOF.
 */

nanoInteractionConstruct
    : nanoInteractionAnnotatedConstruct
    ;


/*
 * ============================================================================
 * ANNOTATED INTERACTION
 * ============================================================================
 *
 * The interaction annotation is deliberately an ordinary identifier.
 *
 * Examples:
 *
 *     @interaction ...
 *     @coupling ...
 *     @transport ...
 *
 * Semantic analysis determines which annotations are valid.
 */

nanoInteractionAnnotatedConstruct
    : nanoInteractionAnnotation
      nanoInteractionTail
    ;


/*
 * ============================================================================
 * INTERACTION ANNOTATION
 * ============================================================================
 */

nanoInteractionAnnotation
    : AT identifier
    ;


/*
 * ============================================================================
 * INTERACTION TAIL
 * ============================================================================
 *
 * Structural alternatives:
 *
 *     @interaction(...);
 *     @interaction(...) { ... }
 *     @interaction name(...);
 *     @interaction name(...) { ... }
 *     @interaction name: Type;
 *     @interaction name: Type = expression;
 *     @interaction name { ... }
 *     @interaction name;
 */

nanoInteractionTail
    : nanoInteractionDirectInvocation
    | nanoInteractionDeclaration
    | nanoInteractionBody
    | SEMICOLON
    ;


/*
 * ============================================================================
 * DIRECT INVOCATION
 * ============================================================================
 *
 * Examples:
 *
 *     @interaction(source, target);
 *
 *     @interaction(source, target) {
 *         ...
 *     }
 *
 * The argument list is intentionally open-ended.
 */

nanoInteractionDirectInvocation
    : LPAREN
      argumentList?
      RPAREN
      nanoInteractionPostInvocation
    ;


nanoInteractionPostInvocation
    : nanoInteractionBody
    | SEMICOLON
    ;


/*
 * ============================================================================
 * NAMED INTERACTION DECLARATION
 * ============================================================================
 *
 * Examples:
 *
 *     @interaction transport(source, target);
 *
 *     @interaction transport(source, target) {
 *         ...
 *     }
 *
 *     @interaction transport: Interaction;
 *
 *     @interaction transport: Interaction = implementation;
 *
 *     @interaction transport {
 *         ...
 *     }
 */

nanoInteractionDeclaration
    : nanoInteractionReference
      nanoInteractionDeclarationTail
    ;


nanoInteractionReference
    : identifier
    ;


nanoInteractionDeclarationTail
    : nanoInteractionNamedInvocation
    | nanoInteractionTypedDeclaration
    | nanoInteractionBody
    | SEMICOLON
    ;


nanoInteractionNamedInvocation
    : LPAREN
      argumentList?
      RPAREN
      nanoInteractionPostInvocation
    ;


/*
 * ============================================================================
 * TYPED INTERACTION DECLARATION
 * ============================================================================
 *
 * Examples:
 *
 *     @interaction transport: Interaction;
 *
 *     @interaction transport: Interaction = implementation;
 *
 *     @interaction transport: Interaction {
 *         ...
 *     }
 */

nanoInteractionTypedDeclaration
    : nanoInteractionTypeClause
      nanoInteractionInitializer?
      nanoInteractionOptionalBody
    ;


nanoInteractionTypeClause
    : COLON
      typeExpression
    ;


nanoInteractionInitializer
    : ASSIGN
      expression
    ;


nanoInteractionOptionalBody
    : nanoInteractionBody
    | SEMICOLON
    ;


/*
 * ============================================================================
 * INTERACTION BODY
 * ============================================================================
 *
 * The body is intentionally reusable and contains:
 *
 *     interaction metadata;
 *     endpoint declarations;
 *     relation declarations;
 *     conditions;
 *     requirements;
 *     constraints;
 *     capabilities;
 *     preferences;
 *     ordinary Zamani statements;
 *     nested interaction constructs.
 */

nanoInteractionBody
    : LBRACE
      nanoInteractionMember*
      RBRACE
    ;


/*
 * ============================================================================
 * INTERACTION MEMBER
 * ============================================================================
 *
 * Specific interaction-owned forms appear before the generic statement
 * fallback so that their ownership remains explicit.
 */

nanoInteractionMember
    : nanoInteractionEndpointAnnotation
    | nanoInteractionEndpoint
    | nanoInteractionRelation
    | nanoInteractionCondition
    | nanoInteractionRequirement
    | nanoInteractionConstraint
    | nanoInteractionCapability
    | nanoInteractionPreference
    | nanoInteractionMetadata
    | nanoInteractionNestedConstruct
    | statement
    ;


/*
 * ============================================================================
 * ENDPOINT ANNOTATION
 * ============================================================================
 *
 * Examples:
 *
 *     @source(input);
 *     @target(output);
 *     @participant(entity);
 *     @observer(sensor);
 *     @control(controller);
 *
 * The annotation name remains open-world.
 *
 * The parser does not determine whether an annotation is a valid endpoint
 * role.
 */

nanoInteractionEndpointAnnotation
    : AT
      identifier
      (
          LPAREN
          argumentList?
          RPAREN
      )?
      SEMICOLON?
    ;


/*
 * ============================================================================
 * ENDPOINT DECLARATION
 * ============================================================================
 *
 * Examples:
 *
 *     source: expression;
 *
 *     target: expression;
 *
 *     participant: expression;
 *
 *     sample: sample_expression;
 *
 * The identifier is semantic, not a reserved endpoint vocabulary.
 */

nanoInteractionEndpoint
    : identifier
      COLON
      expression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * RELATION
 * ============================================================================
 *
 * An interaction relation is represented as a named invocation.
 *
 * Examples:
 *
 *     bind(source, target);
 *
 *     couple(a, b);
 *
 *     transfer(source, target);
 *
 *     observe(sensor, sample);
 *
 * The meaning of the relation is semantic.
 */

nanoInteractionRelation
    : identifier
      LPAREN
      argumentList?
      RPAREN
      SEMICOLON?
    ;


/*
 * ============================================================================
 * CONDITION
 * ============================================================================
 *
 * A condition is represented using an annotation rather than introducing a
 * second keyword vocabulary.
 *
 * Examples:
 *
 *     @when(condition);
 *
 *     @condition(predicate);
 *
 *     @guard(predicate);
 *
 * Semantic analysis determines whether the annotation is a condition.
 */

nanoInteractionCondition
    : AT
      identifier
      LPAREN
      argumentList?
      RPAREN
      SEMICOLON?
    ;


/*
 * ============================================================================
 * REQUIREMENT
 * ============================================================================
 *
 * This rule intentionally uses the canonical REQUIRES token.
 *
 * Examples:
 *
 *     requires capability("nano.interaction");
 *
 *     requires expression;
 *
 * The exact requirement semantics belong to resource/semantic analysis.
 */

nanoInteractionRequirement
    : REQUIRES
      expression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * CONSTRAINT
 * ============================================================================
 *
 * Constraints use the existing INVARIANT token where applicable or generic
 * constraint annotations through metadata.
 *
 * This rule remains deliberately expression-based.
 */

nanoInteractionConstraint
    : INVARIANT
      expression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * CAPABILITY
 * ============================================================================
 *
 * Capability expressions remain ordinary Zamani expressions.
 *
 * Example:
 *
 *     @capability(capability("nano.interaction"));
 *
 * or:
 *
 *     requires capability("nano.interaction");
 *
 * The semantic capability registry owns the meaning.
 */

nanoInteractionCapability
    : AT
      identifier
      LPAREN
      argumentList?
      RPAREN
      SEMICOLON?
    ;


/*
 * ============================================================================
 * PREFERENCE
 * ============================================================================
 *
 * Preferences remain annotations so that the nano interaction grammar does
 * not require a growing reserved keyword vocabulary.
 *
 * Example:
 *
 *     @prefer(strategy);
 */

nanoInteractionPreference
    : AT
      identifier
      LPAREN
      argumentList?
      RPAREN
      SEMICOLON?
    ;


/*
 * ============================================================================
 * METADATA
 * ============================================================================
 *
 * Metadata can be attached without adding domain-specific lexer vocabulary.
 *
 * Examples:
 *
 *     @metadata(value);
 *
 *     @model(model_reference);
 *
 *     @units(unit_reference);
 *
 *     @provenance(source);
 *
 * Semantic analysis owns registration and validation.
 */

nanoInteractionMetadata
    : AT
      identifier
      (
          LPAREN
          argumentList?
          RPAREN
      )?
      SEMICOLON?
    ;


/*
 * ============================================================================
 * NESTED INTERACTION
 * ============================================================================
 *
 * Nested interactions are explicitly supported.
 *
 * This allows hierarchical interaction graphs without a grammar-level depth
 * limit.
 */

nanoInteractionNestedConstruct
    : nanoInteractionConstruct
    ;
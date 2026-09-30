/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/nano/nano.g4
 *
 * GRAMMAR
 * -------
 * Nano
 *
 * STATUS
 * ------
 * CANONICAL NANO-DOMAIN COMPOSITION GRAMMAR
 *
 * LANGUAGE
 * --------
 * Zamani
 *
 * COMPILER
 * --------
 * ZUTC / Zamani Compiler
 *
 * BASELINE
 * --------
 * Rust 1.97 / Rust 1.97.1
 * Rust 2021
 * Safe Rust only
 * No unsafe Rust
 *
 * ============================================================================
 * 1. PURPOSE
 * ============================================================================
 *
 * This file is the composition boundary for the complete nano-oriented
 * language domain.
 *
 * It does NOT define a second nano language.
 *
 * It composes the independently owned nano grammar components:
 *
 *     NanoAgents
 *     NanoAtoms
 *     NanoCapabilities
 *     NanoDeployment
 *     NanoInteractions
 *     NanoMaterials
 *     NanoMolecules
 *     NanoProtocols
 *
 * The individual grammar files remain the owners of their respective syntax.
 *
 * This file owns only:
 *
 *     - nano-domain composition;
 *     - nano-domain dispatch;
 *     - the canonical nanoConstruct entry point;
 *     - composition-level integration invariants.
 *
 * ============================================================================
 * 2. ARCHITECTURAL POSITION
 * ============================================================================
 *
 *                         Zamani source
 *                              |
 *                              v
 *                       canonical lexer
 *                              |
 *                              v
 *                    canonical Zamani parser
 *                              |
 *                              v
 *                         Nano grammar
 *                              |
 *              +---------------+----------------+
 *              |               |                |
 *              v               v                v
 *           agents           atoms          molecules
 *              |               |                |
 *              +---------------+----------------+
 *                              |
 *              +---------------+----------------+
 *              |               |                |
 *              v               v                v
 *          materials      interactions       protocols
 *              |               |                |
 *              +---------------+----------------+
 *                              |
 *                     capabilities/deployment
 *                              |
 *                              v
 *                    domain-neutral frontend AST
 *                              |
 *                              v
 *                     semantic analysis
 *                              |
 *              +---------------+----------------+
 *              |               |                |
 *              v               v                v
 *        resource model   capability model   security model
 *                              |
 *                              v
 *                    canonical semantic model
 *                              |
 *                              v
 *                         canonical IR
 *                              |
 *          +-------------------+-------------------+
 *          |                   |                   |
 *          v                   v                   v
 *      classical          quantum::ir          HDL/hardware
 *          |                   |                   |
 *          +-------------------+-------------------+
 *                              |
 *                              v
 *                  optimization / lowering
 *                              |
 *                   routing / scheduling
 *                              |
 *                    resilience / QEC
 *                              |
 *                             ZQN
 *                              |
 *                             HAL
 *                              |
 *                              v
 *                       target realization
 *
 * The nano grammar therefore describes SOURCE-LEVEL STRUCTURE ONLY.
 *
 * ============================================================================
 * 3. SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * This file is NOT the owner of:
 *
 *     nano agents
 *     atoms
 *     molecules
 *     materials
 *     interactions
 *     protocols
 *     capabilities
 *     deployment semantics
 *
 * Those responsibilities belong to the imported grammar components.
 *
 * This file must not duplicate any rule from those components.
 *
 * In particular, this file must never introduce:
 *
 *     nanoAgentDeclaration
 *     nanoAtomDeclaration
 *     nanoMoleculeConstruct
 *     materialConstruct
 *     nanoInteractionConstruct
 *     nanoProtocolConstruct
 *     nanoCapabilityConstruct
 *     nanoDeploymentConstruct
 *
 * as duplicate definitions.
 *
 * It only dispatches to them.
 *
 * ============================================================================
 * 4. OPEN-WORLD PRINCIPLE
 * ============================================================================
 *
 * Nano computing is an open-ended computational domain.
 *
 * This grammar therefore does NOT enumerate:
 *
 *     atoms
 *     isotopes
 *     elements
 *     molecules
 *     compounds
 *     materials
 *     nanostructures
 *     agents
 *     sensors
 *     actuators
 *     protocols
 *     interactions
 *     physical mechanisms
 *     fabrication methods
 *     devices
 *     vendors
 *     substrates
 *     simulation engines
 *     physical constants
 *     laboratory equipment
 *
 * Names representing such entities remain ordinary Zamani identifiers and
 * qualified names. Their meaning belongs to semantic/domain infrastructure.
 *
 * ============================================================================
 * 5. POCO-REAF
 * ============================================================================
 *
 * Nano source participates in the universal Zamani portability model:
 *
 *     Program Once
 *          |
 *          v
 *     Compile Once
 *          |
 *          v
 *     Run Everywhere
 *          |
 *          v
 *     Run Anywhere
 *          |
 *          v
 *     Forever
 *
 * The nano grammar MUST NOT impose universal limits on:
 *
 *     agents
 *     atoms
 *     molecules
 *     materials
 *     interactions
 *     protocols
 *     participants
 *     roles
 *     states
 *     transitions
 *     operations
 *     properties
 *     dimensions
 *     structures
 *     devices
 *     nodes
 *     processes
 *     resources
 *     memory
 *     communication
 *     simulation size
 *     execution depth
 *
 * Practical limits are determined downstream by:
 *
 *     program semantics;
 *     representation requirements;
 *     declared requirements;
 *     target capabilities;
 *     resource availability;
 *     compiler implementation;
 *     runtime implementation.
 *
 * ============================================================================
 * 6. HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This composition grammar MUST NOT contain universal limits such as:
 *
 *     MAX_NANO_AGENTS
 *     MAX_AGENTS
 *     MAX_ATOMS
 *     MAX_MOLECULES
 *     MAX_MATERIALS
 *     MAX_INTERACTIONS
 *     MAX_PROTOCOLS
 *     MAX_PARTICIPANTS
 *     MAX_ROLES
 *     MAX_STATES
 *     MAX_TRANSITIONS
 *     MAX_NANO_DEVICES
 *     MAX_NANO_RESOURCES
 *     MAX_NANO_MEMORY
 *     MAX_NANO_ENERGY
 *     MAX_NANO_BANDWIDTH
 *     MAX_NANO_DEPTH
 *
 * It must also never encode a finite catalogue of:
 *
 *     elements;
 *     molecules;
 *     materials;
 *     protocols;
 *     devices;
 *     vendors;
 *     sensors;
 *     actuators.
 *
 * Numeric literals in source remain ordinary program semantics.
 *
 * For example:
 *
 *     let atom_count = 1024;
 *
 * is source-level data.
 *
 * It must never be interpreted as establishing a universal language maximum.
 *
 * ============================================================================
 * 7. TARGET NEUTRALITY
 * ============================================================================
 *
 * This grammar does not select:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     accelerator
 *     laboratory device
 *     nano-device
 *     network node
 *     physical sensor
 *     physical actuator
 *     material-processing system
 *
 * It does not perform:
 *
 *     placement;
 *     allocation;
 *     routing;
 *     scheduling;
 *     calibration;
 *     device discovery;
 *     resource discovery;
 *     simulation;
 *     synthesis;
 *     execution.
 *
 * Those responsibilities belong downstream.
 *
 * ============================================================================
 * 8. QUANTUM INTEGRATION
 * ============================================================================
 *
 * Nano constructs may participate in quantum and hybrid computation.
 *
 * This grammar does NOT define quantum operations.
 *
 * Quantum syntax remains owned by:
 *
 *     grammar/quantum/
 *
 * Nano constructs that semantically participate in quantum computation must
 * eventually flow through:
 *
 *     domain-neutral AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     routing
 *          |
 *          v
 *     scheduling
 *          |
 *          v
 *     QEC / resilience
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
 * This file must never create:
 *
 *     NanoQuantumIR
 *     NanoQubitIR
 *     NanoGateIR
 *     NanoDeviceIR
 *
 * ============================================================================
 * 9. CLASSICAL / HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Nano source may participate in:
 *
 *     classical computation;
 *     quantum computation;
 *     hybrid computation;
 *     AI computation;
 *     distributed computation;
 *     networking;
 *     HDL;
 *     hardware/software co-design;
 *     simulation;
 *     accelerator computation.
 *
 * The corresponding grammars remain authoritative for their domains.
 *
 * This file composes them only through the universal parser architecture.
 *
 * ============================================================================
 * 10. RESOURCE AND CAPABILITY SEPARATION
 * ============================================================================
 *
 * Nano source may express requirements or capabilities through the existing
 * nano capability grammar and canonical resource/capability systems.
 *
 * This grammar does not decide whether a requirement can be satisfied.
 *
 * The semantic/compiler pipeline determines that.
 *
 * Conceptually:
 *
 *     source requirement
 *          |
 *          v
 *     semantic requirement
 *          |
 *          v
 *     capability/resource analysis
 *          |
 *          v
 *     target negotiation
 *          |
 *          v
 *     realization
 *
 * Therefore:
 *
 *     requires capability("nano.interaction")
 *
 * is portable intent.
 *
 * It is not equivalent to:
 *
 *     use physical device X
 *
 * or:
 *
 *     use exactly N physical resources
 *
 * ============================================================================
 * 11. DETERMINISM
 * ============================================================================
 *
 * Parsing depends only upon:
 *
 *     - source tokens;
 *     - grammar version;
 *     - imported grammar versions;
 *     - canonical lexical vocabulary;
 *     - explicitly selected dialect configuration.
 *
 * Parsing must not depend upon:
 *
 *     wall-clock time;
 *     randomness;
 *     hardware availability;
 *     filesystem state;
 *     network state;
 *     environment variables;
 *     runtime state;
 *     resource availability.
 *
 * Resource availability is a semantic/compilation/runtime concern, never a
 * parsing concern.
 *
 * ============================================================================
 * 12. SAFETY
 * ============================================================================
 *
 * This is a declarative ANTLR parser grammar.
 *
 * It contains:
 *
 *     no embedded Rust;
 *     no semantic predicates;
 *     no target-language actions;
 *     no filesystem operations;
 *     no network operations;
 *     no hardware operations;
 *     no randomness;
 *     no runtime execution.
 *
 * Rust integration remains:
 *
 *     Rust 2021
 *     Rust 1.97
 *     Rust 1.97.1
 *     safe Rust only
 *     no unsafe
 *
 * ============================================================================
 * 13. CANONICAL DEPENDENCIES
 * ============================================================================
 *
 * The nano composition layer depends on the canonical Zamani lexical and
 * parser vocabulary.
 *
 * Shared parser contracts required by the existing nano components include:
 *
 *     Names
 *     Types
 *     Expressions
 *     Statements
 *     Capabilities
 *
 * The leaf grammars themselves retain ownership of their direct dependencies.
 *
 * This file does not redefine:
 *
 *     identifier
 *     qualified names
 *     typeExpression
 *     expression
 *     argumentList
 *     statement
 *     capabilityExpression
 *     capabilityReference
 *
 * ============================================================================
 * 14. IMPORTED NANO COMPONENTS
 * ============================================================================
 *
 * The existing repository already contains these nano grammar components:
 *
 *     grammar/nano/agents.g4
 *         -> NanoAgents
 *
 *     grammar/nano/atoms.g4
 *         -> NanoAtoms
 *
 *     grammar/nano/capabilities.g4
 *         -> NanoCapabilities
 *
 *     grammar/nano/deployment.g4
 *         -> NanoDeployment
 *
 *     grammar/nano/interactions.g4
 *         -> NanoInteractions
 *
 *     grammar/nano/materials.g4
 *         -> NanoMaterials
 *
 *     grammar/nano/molecules.g4
 *         -> NanoMolecules
 *
 *     grammar/nano/protocols.g4
 *         -> NanoProtocols
 *
 * These files are intentionally retained.
 *
 * They are not renamed.
 *
 * This file is the missing nano-domain composition layer.
 *
 * ============================================================================
 * 15. GRAMMAR DECLARATION
 * ============================================================================
 */

parser grammar Nano;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * The imported nano components are the owners of their respective rules.
 *
 * Shared parser grammars are included here because NanoAtoms currently
 * consumes canonical expression/type/statement rules without declaring those
 * imports itself. Including them at this composition boundary makes the nano
 * composition contract explicit and prevents this file from depending on
 * accidental transitive imports.
 */
import Names,
       Types,
       Expressions,
       Statements,
       Capabilities,
       NanoAgents,
       NanoAtoms,
       NanoCapabilities,
       NanoDeployment,
       NanoInteractions,
       NanoMaterials,
       NanoMolecules,
       NanoProtocols;


/*
 * ============================================================================
 * 16. CANONICAL PUBLIC ENTRY POINT
 * ============================================================================
 *
 * There is exactly one nano-domain composition entry point:
 *
 *     nanoConstruct
 *
 * The universal Zamani parser decides when this rule may occur.
 *
 * This rule does not consume EOF.
 *
 * It is therefore composable inside:
 *
 *     source units;
 *     declarations;
 *     statements;
 *     domain dispatch;
 *     blocks;
 *     future universal composition layers.
 *
 * ============================================================================
 */

nanoConstruct
    : nanoAgentConstruct
    | nanoAtomDeclaration
    | nanoCapabilityConstruct
    | nanoDeploymentConstruct
    | nanoInteractionConstruct
    | materialConstruct
    | nanoMoleculeConstruct
    | nanoProtocolConstruct
    ;


/*
 * ============================================================================
 * 17. DISPATCH OWNERSHIP
 * ============================================================================
 *
 * The branches above are deliberately direct delegation.
 *
 * Nano does not inspect annotation names, identifiers, types, or expressions
 * to determine semantic meaning.
 *
 * In particular, this grammar does NOT contain rules such as:
 *
 *     if annotation == "atom"
 *     if annotation == "molecule"
 *     if annotation == "material"
 *
 * Annotation meaning belongs to semantic analysis.
 *
 * Structural ownership is established by the selected nano component.
 *
 * ============================================================================
 * 18. ATOM INTEGRATION
 * ============================================================================
 *
 * Atom syntax remains owned by:
 *
 *     grammar/nano/atoms.g4
 *
 * Public rule:
 *
 *     nanoAtomDeclaration
 *
 * Nano composition delegates directly to that rule.
 *
 * Atom semantics belong downstream.
 *
 * They may include:
 *
 *     atomic identity;
 *     symbolic properties;
 *     physical properties;
 *     quantum properties;
 *     computational properties;
 *     resource requirements;
 *     capabilities;
 *     constraints;
 *     interoperability.
 *
 * This grammar does not validate any of those meanings.
 *
 * ============================================================================
 * 19. MOLECULE INTEGRATION
 * ============================================================================
 *
 * Molecular syntax remains owned by:
 *
 *     grammar/nano/molecules.g4
 *
 * Public rule:
 *
 *     nanoMoleculeConstruct
 *
 * This composition layer does not define:
 *
 *     molecular bonds;
 *     molecular geometry;
 *     chemical feasibility;
 *     reaction mechanisms;
 *     molecular dynamics;
 *     simulation algorithms.
 *
 * Those are semantic/domain responsibilities.
 *
 * ============================================================================
 * 20. MATERIAL INTEGRATION
 * ============================================================================
 *
 * Material syntax remains owned by:
 *
 *     grammar/nano/materials.g4
 *
 * Public rule:
 *
 *     materialConstruct
 *
 * The material catalogue remains open-world.
 *
 * No periodic-table-like or material-database enumeration belongs here.
 *
 * ============================================================================
 * 21. AGENT INTEGRATION
 * ============================================================================
 *
 * Agent syntax remains owned by:
 *
 *     grammar/nano/agents.g4
 *
 * Public rule:
 *
 *     nanoAgentConstruct
 *
 * Agent semantics may interact with:
 *
 *     AI;
 *     concurrency;
 *     networking;
 *     distributed computation;
 *     security;
 *     resources;
 *     quantum computation;
 *     hardware.
 *
 * Those relationships are semantic and compiler concerns, not parser
 * composition concerns.
 *
 * ============================================================================
 * 22. INTERACTION INTEGRATION
 * ============================================================================
 *
 * Interaction syntax remains owned by:
 *
 *     grammar/nano/interactions.g4
 *
 * Public rule:
 *
 *     nanoInteractionConstruct
 *
 * Interactions remain open-world.
 *
 * This grammar does not enumerate physical forces, chemical interactions,
 * biological mechanisms, communication protocols, or device mechanisms.
 *
 * ============================================================================
 * 23. PROTOCOL INTEGRATION
 * ============================================================================
 *
 * Protocol syntax remains owned by:
 *
 *     grammar/nano/protocols.g4
 *
 * Public rule:
 *
 *     nanoProtocolConstruct
 *
 * Nano protocols are distinct from generic networking protocols.
 *
 * A nano protocol may nevertheless semantically use:
 *
 *     networking;
 *     distributed computing;
 *     security;
 *     quantum computation;
 *     classical computation;
 *     hardware;
 *     resource capabilities.
 *
 * This file does not duplicate any of those domains.
 *
 * ============================================================================
 * 24. CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Nano capability syntax remains owned by:
 *
 *     grammar/nano/capabilities.g4
 *
 * Public rule:
 *
 *     nanoCapabilityConstruct
 *
 * The canonical capability model remains owned by the general capability
 * grammar.
 *
 * NanoCapabilities is an adapter, not a second capability language.
 *
 * ============================================================================
 * 25. DEPLOYMENT INTEGRATION
 * ============================================================================
 *
 * Nano deployment intent remains owned by:
 *
 *     grammar/nano/deployment.g4
 *
 * Public rule:
 *
 *     nanoDeploymentConstruct
 *
 * Deployment intent must remain separate from actual deployment.
 *
 * Actual realization belongs downstream to:
 *
 *     execution;
 *     resources;
 *     hardware;
 *     distributed;
 *     compiler;
 *     runtime;
 *     deployment infrastructure.
 *
 * ============================================================================
 * 26. DOMAIN-NEUTRAL AST CONTRACT
 * ============================================================================
 *
 * Nano grammar constructs must map into the existing domain-neutral frontend
 * architecture.
 *
 * This file MUST NOT require:
 *
 *     NanoAst
 *     NanoAgentAst
 *     NanoAtomAst
 *     NanoMoleculeAst
 *     NanoMaterialAst
 *     NanoProtocolAst
 *     NanoInteractionAst
 *
 * merely because the parser has a nano composition layer.
 *
 * Where existing AST structures can represent the construct, those structures
 * remain preferred.
 *
 * If a new semantic distinction genuinely requires an AST representation,
 * that AST change belongs to the canonical frontend AST design and must be
 * domain-neutral unless the distinction is inherently semantic.
 *
 * Required mapping direction:
 *
 *     Nano grammar
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic analysis
 *
 * Never:
 *
 *     Nano grammar
 *          |
 *          v
 *     private Nano IR
 *
 * ============================================================================
 * 27. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis owns:
 *
 *     name resolution;
 *     type checking;
 *     annotation meaning;
 *     atom validation;
 *     molecule validation;
 *     material validation;
 *     agent validation;
 *     interaction validation;
 *     protocol validation;
 *     capability validation;
 *     requirement validation;
 *     constraint validation;
 *     resource validation;
 *     security validation;
 *     ownership validation;
 *     effect validation;
 *     physical-model validation;
 *     quantum integration;
 *     hardware integration;
 *     distributed integration;
 *     interoperability validation;
 *     portability validation.
 *
 * Parsing must remain structural.
 *
 * ============================================================================
 * 28. RESOURCE CONTRACT
 * ============================================================================
 *
 * The parser accepts arbitrarily many source constructs subject only to the
 * actual parser/runtime representation and available implementation
 * resources.
 *
 * It must not impose language-level limits.
 *
 * Resource constraints are represented semantically through the canonical
 * resource/capability system.
 *
 * Examples of portable intent include:
 *
 *     requires capability("nano.interaction")
 *     requires capability("nano.sensing")
 *     requires capability("nano.actuation")
 *     requires capability("quantum.measurement")
 *     requires capability("tensor.compute")
 *
 * The grammar does not decide whether those capabilities exist.
 *
 * ============================================================================
 * 29. HARDWARE CONTRACT
 * ============================================================================
 *
 * Nano computation may eventually execute on:
 *
 *     embedded systems;
 *     CPUs;
 *     multicore systems;
 *     GPUs;
 *     FPGAs;
 *     ASICs;
 *     accelerators;
 *     QPUs;
 *     simulators;
 *     HPC systems;
 *     clusters;
 *     distributed systems;
 *     cloud systems;
 *     future computational substrates.
 *
 * This composition grammar is unaware of the target.
 *
 * Hardware selection remains owned by the hardware/resource/compiler/runtime
 * layers.
 *
 * ============================================================================
 * 30. QUANTUM CONTRACT
 * ============================================================================
 *
 * Nano constructs that require quantum computation must use the same canonical
 * quantum path as all other Zamani domains.
 *
 * The nano composition grammar must never create a separate quantum grammar
 * namespace merely because the source construct originated in nano.
 *
 * The semantic relationship is:
 *
 *     nano construct
 *          |
 *          v
 *     domain-neutral semantics
 *          |
 *          v
 *     quantum semantic analysis
 *          |
 *          v
 *     quantum::ir
 *
 * ============================================================================
 * 31. HDL / CO-DESIGN CONTRACT
 * ============================================================================
 *
 * Nano constructs may participate in hardware/software co-design.
 *
 * Hardware intent remains owned by:
 *
 *     grammar/hardware/
 *     grammar/hdl/
 *
 * Nano does not define:
 *
 *     wires;
 *     register widths;
 *     clock counts;
 *     FPGA capacities;
 *     ASIC structures;
 *     physical layouts.
 *
 * Those are separate language-domain responsibilities.
 *
 * ============================================================================
 * 32. DISTRIBUTED CONTRACT
 * ============================================================================
 *
 * Nano constructs may be distributed across arbitrary computational
 * resources.
 *
 * The nano grammar does not impose:
 *
 *     node counts;
 *     process counts;
 *     participant counts;
 *     channel counts;
 *     topology limits.
 *
 * Distributed placement remains downstream.
 *
 * ============================================================================
 * 33. SECURITY CONTRACT
 * ============================================================================
 *
 * Nano constructs may participate in secure computation and controlled
 * execution.
 *
 * Security syntax and semantics remain owned by:
 *
 *     grammar/security/
 *
 * This grammar does not invent:
 *
 *     nano-specific authentication;
 *     nano-specific cryptographic primitives;
 *     nano-specific authorization languages.
 *
 * ============================================================================
 * 34. INTEROPERABILITY CONTRACT
 * ============================================================================
 *
 * Nano computation may interoperate with:
 *
 *     classical languages;
 *     quantum formats;
 *     HDL;
 *     hardware interfaces;
 *     scientific formats;
 *     simulation systems;
 *     distributed systems;
 *     networking systems;
 *     future domains.
 *
 * Interoperability syntax remains owned by:
 *
 *     grammar/interoperability/
 *     grammar/dialects/
 *
 * This file only provides nano-domain composition.
 *
 * ============================================================================
 * 35. DIALECT CONTRACT
 * ============================================================================
 *
 * Vendor-, laboratory-, simulator-, accelerator-, material-database-, or
 * device-specific extensions MUST NOT be added to this file as hard-coded
 * alternatives.
 *
 * Such extensions belong to the dialect system.
 *
 * A dialect must establish:
 *
 *     identity;
 *     version;
 *     syntax extension;
 *     semantic extension;
 *     AST mapping;
 *     IR mapping;
 *     compatibility;
 *     feature-gating policy.
 *
 * ============================================================================
 * 36. FUTURE EXTENSION CONTRACT
 * ============================================================================
 *
 * Future nano domains should be added as independent grammar components.
 *
 * Examples may include:
 *
 *     fabrication;
 *     sensing;
 *     actuation;
 *     self-assembly;
 *     molecular robotics;
 *     nanoscale communication;
 *     nanofluidics;
 *     bio-nano computation;
 *     quantum-nano interfaces;
 *     nano-memory;
 *     nano-energy;
 *     nano-networking;
 *     nano-control;
 *     nano-verification.
 *
 * A future component should:
 *
 *     1. own its syntax;
 *     2. import canonical shared grammar dependencies;
 *     3. expose one public construct rule;
 *     4. avoid duplicate lexical rules;
 *     5. avoid duplicate type/expression systems;
 *     6. avoid physical resource limits;
 *     7. define AST mapping before stabilization;
 *     8. define semantic mapping before stabilization;
 *     9. define IR integration before stabilization;
 *    10. provide positive/negative/boundary/scalability tests.
 *
 * The composition layer can then add the component as one branch.
 *
 * ============================================================================
 * 37. COMPATIBILITY
 * ============================================================================
 *
 * Existing files are intentionally preserved:
 *
 *     agents.g4
 *     atoms.g4
 *     capabilities.g4
 *     deployment.g4
 *     interactions.g4
 *     materials.g4
 *     molecules.g4
 *     protocols.g4
 *
 * No existing nano filename is renamed by this addition.
 *
 * This file is additive.
 *
 * Existing leaf grammar rule names remain authoritative.
 *
 * ============================================================================
 * 38. ROOT PARSER INTEGRATION
 * ============================================================================
 *
 * The canonical language root remains:
 *
 *     grammar/Zamani.g4
 *
 * It must NOT import this file directly if the canonical parser composition
 * layer is responsible for all domain composition.
 *
 * Preferred architecture:
 *
 *     grammar/Zamani.g4
 *             |
 *             v
 *     canonical ZamaniParser
 *             |
 *             +--> Nano
 *                      |
 *          +-----------+-----------+
 *          |           |           |
 *        agents      atoms      molecules
 *          |           |           |
 *          +-----------+-----------+
 *                      |
 *               remaining nano domains
 *
 * Therefore the required integration point is the canonical parser
 * composition grammar, not a second top-level root.
 *
 * ============================================================================
 * 39. REQUIRED PARSER-COMPOSITION INTEGRATION
 * ============================================================================
 *
 * The canonical parser composition must import:
 *
 *     Nano
 *
 * and expose:
 *
 *     nanoConstruct
 *
 * from its universal domain dispatch.
 *
 * Conceptually:
 *
 *     domainConstruct
 *         : classicalConstruct
 *         | quantumConstruct
 *         | hybridConstruct
 *         | hdlConstruct
 *         | hardwareConstruct
 *         | nanoConstruct
 *         | ...
 *         ;
 *
 * The exact rule name belongs to the canonical parser composition layer.
 *
 * This file does not modify that parent rule.
 *
 * ============================================================================
 * 40. LEXER INTEGRATION
 * ============================================================================
 *
 * No lexer modifications are required merely to add this composition file,
 * provided the canonical lexer already supplies the punctuation and token
 * vocabulary consumed by the existing nano leaf grammars.
 *
 * In particular, this file does not introduce:
 *
 *     NANO
 *     ATOM
 *     MOLECULE
 *     MATERIAL
 *     PROTOCOL
 *     INTERACTION
 *     AGENT
 *
 * as mandatory lexer keywords.
 *
 * Existing open-world annotation/name design remains preferred.
 *
 * ============================================================================
 * 41. RUST INTEGRATION
 * ============================================================================
 *
 * This file contains no Rust implementation code.
 *
 * Generated parser code must remain compatible with:
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
 * 42. DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Syntax diagnostics are generated by the canonical parser infrastructure.
 *
 * This file must preserve the source location of the selected nano construct
 * through normal ANTLR parse-tree/source-span handling.
 *
 * Semantic diagnostics belong downstream.
 *
 * Examples include:
 *
 *     unknown nano entity;
 *     invalid atom reference;
 *     invalid molecule reference;
 *     invalid material reference;
 *     invalid agent reference;
 *     invalid interaction;
 *     invalid protocol;
 *     unknown capability;
 *     unsatisfied requirement;
 *     violated constraint;
 *     incompatible target;
 *     unsupported physical realization.
 *
 * None of these are parser-level hardware decisions.
 *
 * ============================================================================
 * 43. NEGATIVE PARSING CONTRACT
 * ============================================================================
 *
 * The composition grammar must reject incomplete constructs through the
 * imported leaf grammars.
 *
 * Examples include malformed forms such as:
 *
 *     @agent
 *     @protocol
 *     @molecule
 *     @material
 *     @interaction
 *
 * when the selected leaf grammar requires additional syntax.
 *
 * It must also reject:
 *
 *     unmatched delimiters;
 *     malformed argument lists;
 *     malformed type expressions;
 *     malformed expressions;
 *     malformed nested constructs.
 *
 * Semantic invalidity is not converted into syntax failure merely because a
 * construct names an unknown physical entity.
 *
 * ============================================================================
 * 44. BOUNDARY CONTRACT
 * ============================================================================
 *
 * Conformance tests must cover:
 *
 *     empty nano domain where allowed by the parent;
 *     one construct;
 *     many constructs;
 *     deeply nested nano constructs;
 *     deeply nested expressions;
 *     deeply qualified names;
 *     large parameter lists;
 *     large property sets;
 *     large protocol definitions;
 *     large interaction graphs;
 *     large material descriptions;
 *     large molecular structures;
 *     large agent compositions;
 *     large capability sets;
 *     large deployment descriptions.
 *
 * The test suite must not turn any observed implementation boundary into a
 * language-level maximum.
 *
 * ============================================================================
 * 45. SCALABILITY CONTRACT
 * ============================================================================
 *
 * Scalability tests should increase workload size progressively.
 *
 * Dimensions include:
 *
 *     construct count;
 *     nesting depth;
 *     identifier length;
 *     qualified-name depth;
 *     parameter count;
 *     expression size;
 *     type-expression size;
 *     agent count;
 *     atom count;
 *     molecule component count;
 *     material property count;
 *     interaction count;
 *     protocol role count;
 *     protocol participant count;
 *     protocol state count;
 *     transition count;
 *     capability count;
 *     deployment requirement count.
 *
 * These are test dimensions, NOT grammar constants.
 *
 * ============================================================================
 * 46. DETERMINISM TEST CONTRACT
 * ============================================================================
 *
 * Repeated parsing of identical source under identical grammar and lexer
 * versions must produce equivalent parse structures.
 *
 * Tests must not vary:
 *
 *     hardware;
 *     network;
 *     filesystem;
 *     resource availability;
 *     runtime state;
 *     randomness.
 *
 * ============================================================================
 * 47. CROSS-DOMAIN TEST CONTRACT
 * ============================================================================
 *
 * Nano composition must be tested together with:
 *
 *     classical;
 *     quantum;
 *     hybrid;
 *     HDL;
 *     hardware;
 *     distributed;
 *     AI;
 *     data;
 *     networking;
 *     security;
 *     resources;
 *     execution;
 *     interoperability;
 *     dialects.
 *
 * The nano grammar remains target-independent in every case.
 *
 * ============================================================================
 * 48. AST / SEMANTIC / IR COMPLETION GATE
 * ============================================================================
 *
 * Adding this file does NOT by itself make the nano domain IMPLEMENTED.
 *
 * Production conformance requires:
 *
 *     lexer conformance;
 *     parser composition;
 *     AST mapping;
 *     semantic analysis;
 *     canonical IR integration;
 *     compiler integration;
 *     runtime integration;
 *     positive tests;
 *     negative tests;
 *     boundary tests;
 *     scalability tests;
 *     determinism tests;
 *     compatibility tests.
 *
 * Until all are complete, grammar/grammar.md must not claim:
 *
 *     IMPLEMENTED
 *
 * solely because this grammar exists.
 *
 * ============================================================================
 * 49. GRAMMAR.MD INTEGRATION
 * ============================================================================
 *
 * The conformance status of the nano domain remains one of:
 *
 *     SPECIFIED
 *     IMPLEMENTED
 *     PARTIALLY IMPLEMENTED
 *     PLANNED
 *     DEPRECATED
 *
 * This file establishes the canonical composition contract.
 *
 * It does not independently promote the nano domain to stable status.
 *
 * ============================================================================
 * 50. ZAMANI-GRAMMAR.MD INTEGRATION
 * ============================================================================
 *
 * grammar/Zamani-Grammar.md remains the historical/extended design source.
 *
 * It may contain proposed nano concepts.
 *
 * Those concepts do not become syntax merely because they occur there.
 *
 * Promotion remains:
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
 *     canonical grammar
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
 * 51. VALIDATION REQUIREMENTS
 * ============================================================================
 *
 * Validation tooling should verify:
 *
 *     [x] one parser grammar identity;
 *     [x] canonical ZamaniLexer vocabulary;
 *     [x] no lexer rules;
 *     [x] no duplicate nano leaf rules;
 *     [x] no duplicate type system;
 *     [x] no duplicate expression system;
 *     [x] no duplicate statement system;
 *     [x] no fixed physical entity catalogue;
 *     [x] no universal resource ceiling;
 *     [x] no hardware selection;
 *     [x] no embedded Rust;
 *     [x] no semantic predicates;
 *     [x] no unsafe requirement;
 *     [x] one canonical nano composition entry point;
 *     [x] existing nano files retained;
 *     [x] domain-neutral AST boundary preserved;
 *     [x] canonical IR boundary preserved;
 *     [x] quantum::ir boundary preserved;
 *     [x] POCO-REAF preserved.
 *
 * ============================================================================
 * 52. HARD-CODING AUDIT
 * ============================================================================
 *
 * This file intentionally contains no:
 *
 *     MAX_* resource constant;
 *     fixed atom table;
 *     fixed molecule table;
 *     fixed material table;
 *     fixed protocol table;
 *     fixed agent table;
 *     fixed interaction table;
 *     fixed device table;
 *     fixed vendor table;
 *     fixed topology;
 *     fixed resource capacity;
 *     fixed hardware identifier.
 *
 * ============================================================================
 * 53. NO RUNTIME BEHAVIOR
 * ============================================================================
 *
 * This grammar never:
 *
 *     executes an agent;
 *     manipulates an atom;
 *     creates a molecule;
 *     changes a material;
 *     communicates with a device;
 *     performs a protocol;
 *     allocates memory;
 *     discovers hardware;
 *     schedules execution;
 *     performs simulation;
 *     performs chemistry;
 *     performs quantum computation.
 *
 * It only recognizes source-level structure.
 *
 * ============================================================================
 * 54. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete as the nano-domain COMPOSITION CONTRACT when:
 *
 *   [x] The existing nano leaf files remain unchanged in name.
 *   [x] Each leaf grammar retains ownership of its own syntax.
 *   [x] Nano has exactly one composition grammar.
 *   [x] Nano exposes nanoConstruct.
 *   [x] No lexer vocabulary is duplicated.
 *   [x] No general expression grammar is duplicated.
 *   [x] No general type grammar is duplicated.
 *   [x] No general statement grammar is duplicated.
 *   [x] No quantum IR is introduced.
 *   [x] No hardware limit is introduced.
 *   [x] No physical entity catalogue is introduced.
 *   [x] No resource allocation occurs.
 *   [x] No target selection occurs.
 *   [x] No runtime behavior occurs.
 *   [x] Cross-domain integration is explicitly defined.
 *   [x] AST integration is explicitly defined.
 *   [x] Semantic integration is explicitly defined.
 *   [x] IR integration is explicitly defined.
 *   [x] Compiler integration is explicitly defined.
 *   [x] Runtime integration is explicitly defined.
 *   [x] Compatibility integration is explicitly defined.
 *   [x] Test requirements are explicitly defined.
 *   [x] POCO-REAF remains intact.
 *
 * ============================================================================
 * 55. FINAL INVARIANT
 * ============================================================================
 *
 * This file answers:
 *
 *     "Which existing nano grammar component owns this nano construct?"
 *
 * It does NOT answer:
 *
 *     "How does the physical world realize the construct?"
 *
 * The latter remains downstream.
 *
 * The final architecture remains:
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
 *     Nano composition
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
 *          +-----------------------------+
 *          |                             |
 *          v                             v
 *      classical                    quantum::ir
 *          |                             |
 *          +-------------+---------------+
 *                        |
 *                        v
 *                 optimization
 *                        |
 *                 routing/scheduling
 *                        |
 *                 resilience/QEC
 *                        |
 *                       ZQN
 *                        |
 *                       HAL
 *                        |
 *                        v
 *                 target realization
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
 *     Forever
 *
 * subject only to the actual semantics of the program, declared
 * requirements, target capabilities, implementation constraints, and
 * resources available to the realization.
 *
 * ============================================================================
 */
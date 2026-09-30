/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/nano/materials.g4
 *
 * GRAMMAR
 * -------
 * NanoMaterials
 *
 * STATUS
 * ------
 * CANONICAL NANO-DOMAIN MATERIAL GRAMMAR
 *
 * LANGUAGE
 * --------
 * Zamani
 *
 * BASELINE
 * --------
 * Rust 1.97 / Rust 1.97.1
 * Rust 2021
 * Safe Rust only
 *
 * ============================================================================
 * 1. PURPOSE
 * ============================================================================
 *
 * This grammar owns the SOURCE-LEVEL STRUCTURE of material-oriented
 * computation inside Zamani's nano/physical domain.
 *
 * It provides an open-world syntactic contract for describing:
 *
 *     - material declarations;
 *     - material references;
 *     - material instances;
 *     - material composition;
 *     - material properties;
 *     - material parameters;
 *     - material models;
 *     - material transformations;
 *     - material relationships;
 *     - material requirements;
 *     - material capabilities;
 *     - material constraints;
 *     - material observations;
 *     - material operations;
 *     - material annotations;
 *     - nested material-oriented constructs;
 *     - future material-domain extensions.
 *
 * This grammar intentionally does NOT attempt to encode physical science
 * directly.
 *
 * It does not contain:
 *
 *     - a periodic table;
 *     - a fixed material catalogue;
 *     - a fixed list of elements;
 *     - a fixed list of compounds;
 *     - a fixed list of polymers;
 *     - a fixed list of crystal structures;
 *     - a fixed list of biological materials;
 *     - a fixed list of nanomaterials;
 *     - physical constants;
 *     - vendor material identifiers;
 *     - laboratory database identifiers;
 *     - simulation-engine implementations.
 *
 * Those belong to semantic models, libraries, databases, domain engines,
 * scientific backends, or target-specific implementations.
 *
 * ============================================================================
 * 2. ARCHITECTURAL PRINCIPLE
 * ============================================================================
 *
 * Material computation is a DOMAIN of Zamani.
 *
 * It is not a separate programming language.
 *
 * Therefore:
 *
 *     material source
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
 *     semantic material model
 *          |
 *          v
 *     canonical semantic IR
 *          |
 *          v
 *     optimization / simulation / synthesis / execution
 *          |
 *          v
 *     target realization
 *
 * This grammar owns only the syntactic boundary.
 *
 * ============================================================================
 * 3. POCO-REAF
 * ============================================================================
 *
 * Material programs MUST remain portable across implementations.
 *
 * The same source-level material intent may be realized by:
 *
 *     - symbolic computation;
 *     - numerical simulation;
 *     - molecular simulation;
 *     - atomistic simulation;
 *     - classical computation;
 *     - quantum computation;
 *     - hybrid computation;
 *     - GPU acceleration;
 *     - FPGA/ASIC acceleration;
 *     - distributed computation;
 *     - HPC;
 *     - cloud execution;
 *     - laboratory or device interfaces;
 *     - future computational substrates.
 *
 * This file MUST NOT impose universal limits on:
 *
 *     materials;
 *     material instances;
 *     components;
 *     atoms;
 *     molecules;
 *     properties;
 *     dimensions;
 *     structures;
 *     interactions;
 *     transformations;
 *     observations;
 *     simulations;
 *     resources;
 *     devices;
 *     nodes;
 *     processes;
 *     memory;
 *     compute capacity.
 *
 * It MUST NOT introduce artificial constants such as:
 *
 *     MAX_MATERIALS
 *     MAX_MATERIAL_COMPONENTS
 *     MAX_MATERIAL_PROPERTIES
 *     MAX_MATERIAL_DIMENSIONS
 *     MAX_ATOMS
 *     MAX_MOLECULES
 *     MAX_MATERIAL_INSTANCES
 *     MAX_MATERIAL_RESOURCES
 *     MAX_SIMULATION_SIZE
 *
 * Numeric values appearing in source remain ordinary program semantics.
 *
 * For example:
 *
 *     composition = 1024
 *
 * may be valid program data.
 *
 * But:
 *
 *     material components may never exceed 1024
 *
 * MUST NOT become a grammar-level restriction.
 *
 * ============================================================================
 * 4. OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     materialConstruct
 *     materialAnnotatedConstruct
 *     materialAnnotation
 *     materialTail
 *     materialDeclaration
 *     materialReference
 *     materialDeclarationTail
 *     materialInvocation
 *     materialTypedDeclaration
 *     materialInitializer
 *     materialBody
 *     materialMember
 *     materialProperty
 *     materialPropertyTail
 *     materialBinding
 *     materialComposition
 *     materialOperation
 *     materialOperationTail
 *     materialConstraint
 *     materialRequirement
 *     materialCapability
 *     materialObservation
 *     materialTransformation
 *
 * THIS FILE DOES NOT OWN:
 *
 *     lexical tokens;
 *     identifiers;
 *     qualified names;
 *     general expressions;
 *     expression precedence;
 *     general types;
 *     statements;
 *     functions;
 *     modules;
 *     resources;
 *     hardware capabilities;
 *     physical placement;
 *     scheduling;
 *     routing;
 *     simulation algorithms;
 *     chemistry;
 *     material databases;
 *     quantum operations;
 *     QEC;
 *     ZQN;
 *     HAL;
 *     compiler implementation;
 *     runtime implementation.
 *
 * ============================================================================
 * 5. CANONICAL LEXICAL DEPENDENCY
 * ============================================================================
 *
 * The canonical lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Parser grammars consume:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * This file MUST NOT:
 *
 *     - define lexer rules;
 *     - define material-specific lexer tokens;
 *     - introduce MATERIAL as a mandatory keyword;
 *     - introduce ELEMENT as a mandatory keyword;
 *     - introduce COMPOUND as a mandatory keyword;
 *     - introduce fixed scientific identifiers as keywords.
 *
 * Material concepts are represented through ordinary Zamani identifiers,
 * annotations, types, expressions, and statements.
 *
 * This preserves the open-world property of the domain.
 *
 * ============================================================================
 * 6. CANONICAL PARSER DEPENDENCIES
 * ============================================================================
 *
 * This grammar reuses the existing canonical parser contracts:
 *
 *     Types
 *     Expressions
 *     Statements
 *
 * In particular, it consumes:
 *
 *     identifier
 *     typeExpression
 *     expression
 *     argumentList
 *     statement
 *
 * It MUST NOT redefine those rules.
 *
 * ============================================================================
 * 7. OPEN-WORLD MATERIAL ANNOTATIONS
 * ============================================================================
 *
 * Material-domain concepts may be introduced using ordinary annotations.
 *
 * Examples:
 *
 *     @material
 *     @materialize
 *     @composition
 *     @property
 *     @model
 *     @transform
 *     @observe
 *     @require
 *     @capability
 *
 * The annotation name is intentionally not interpreted by the parser.
 *
 * Semantic analysis is responsible for determining:
 *
 *     - whether the annotation is registered;
 *     - what version it belongs to;
 *     - what semantic domain it denotes;
 *     - whether its arguments are valid;
 *     - whether its use is compatible with the active language version.
 *
 * Future material concepts therefore do not require a new lexer keyword.
 *
 * ============================================================================
 * 8. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * Exactly one public material-domain construct is exposed.
 *
 * The universal parser decides when this construct participates in a
 * complete Zamani program.
 */

parser grammar NanoMaterials;

options {
    tokenVocab = ZamaniLexer;
}

import Types,
       Expressions,
       Statements;


/*
 * ============================================================================
 * 9. PUBLIC CONSTRUCT
 * ============================================================================
 */

materialConstruct
    : materialAnnotatedConstruct
    ;


/*
 * ============================================================================
 * 10. ANNOTATED MATERIAL CONSTRUCT
 * ============================================================================
 *
 * Examples:
 *
 *     @material
 *
 *     @material Graphene
 *
 *     @material Graphene: Material
 *
 *     @material Graphene = model;
 *
 *     @material Graphene {
 *         ...
 *     }
 */

materialAnnotatedConstruct
    : materialAnnotation
      materialTail
    ;


/*
 * ============================================================================
 * 11. MATERIAL ANNOTATION
 * ============================================================================
 *
 * The annotation identifier is deliberately unrestricted.
 *
 * The grammar does not distinguish:
 *
 *     @material
 *     @material_model
 *     @material_property
 *     @future_material_feature
 *
 * by lexer token.
 *
 * Semantic analysis owns those distinctions.
 */

materialAnnotation
    : AT
      identifier
    ;


/*
 * ============================================================================
 * 12. MATERIAL CONSTRUCT TAIL
 * ============================================================================
 *
 * Structural dispatch is based on syntax shape rather than on a closed
 * vocabulary of material concepts.
 */

materialTail
    : materialDirectInvocation
    | materialDeclaration
    | materialExpressionBinding
    | materialBody
    | SEMICOLON
    ;


/*
 * ============================================================================
 * 13. DIRECT MATERIAL INVOCATION
 * ============================================================================
 *
 * Examples:
 *
 *     @material(model);
 *
 *     @model(graphene_model);
 *
 *     @observe(sample, sensor);
 *
 *     @capability(capability("material.simulation"));
 */

materialDirectInvocation
    : LPAREN
      argumentList?
      RPAREN
      materialPostInvocation
    ;


materialPostInvocation
    : materialBody
    | SEMICOLON
    ;


/*
 * ============================================================================
 * 14. MATERIAL DECLARATION
 * ============================================================================
 *
 * Examples:
 *
 *     @material Graphene;
 *
 *     @material Graphene: Material;
 *
 *     @material Graphene = graphene_model;
 *
 *     @material Graphene: Material = graphene_model;
 *
 *     @material Graphene {
 *         ...
 *     }
 */

materialDeclaration
    : materialReference
      materialDeclarationTail
    ;


materialReference
    : identifier
    ;


materialDeclarationTail
    : materialNamedInvocation
    | materialTypedDeclaration
    | materialBody
    | SEMICOLON
    ;


materialNamedInvocation
    : LPAREN
      argumentList?
      RPAREN
      materialPostInvocation
    ;


/*
 * ============================================================================
 * 15. TYPED MATERIAL DECLARATION
 * ============================================================================
 *
 * Type meaning belongs to the canonical type system.
 *
 * This grammar therefore accepts arbitrary valid Zamani type expressions.
 */

materialTypedDeclaration
    : materialTypeClause?
      materialInitializer?
      materialOptionalBody
    ;


materialTypeClause
    : COLON
      typeExpression
    ;


materialInitializer
    : ASSIGN
      expression
    ;


materialOptionalBody
    : materialBody
    | SEMICOLON
    ;


/*
 * ============================================================================
 * 16. MATERIAL EXPRESSION BINDING
 * ============================================================================
 *
 * Examples:
 *
 *     @material = material_model;
 *
 *     @model = simulation_model;
 *
 * The expression remains owned by Expressions.
 */

materialExpressionBinding
    : ASSIGN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 17. MATERIAL BODY
 * ============================================================================
 *
 * Material bodies support hierarchical domain composition.
 *
 * Ordinary Zamani statements remain legal inside the body.
 *
 * Material-specific constructs remain structurally open-world.
 */

materialBody
    : LBRACE
      materialMember*
      RBRACE
    ;


materialMember
    : materialAnnotatedConstruct
    | materialProperty
    | materialComposition
    | materialOperation
    | materialRequirement
    | materialCapability
    | materialConstraint
    | materialObservation
    | materialTransformation
    | statement
    ;


/*
 * ============================================================================
 * 18. MATERIAL PROPERTY
 * ============================================================================
 *
 * A property is represented structurally rather than by enumerating
 * scientific property names.
 *
 * Therefore all of these can be represented without modifying this grammar:
 *
 *     @property density = value;
 *     @property conductivity = value;
 *     @property band_gap = value;
 *     @property tensile_strength = value;
 *     @property custom_property = value;
 *
 * The semantic layer determines whether a property is meaningful.
 */

materialProperty
    : AT
      identifier
      materialPropertyTail
    ;


materialPropertyTail
    : materialNamedInvocation
    | materialBinding
    | materialBody
    | SEMICOLON
    ;


materialBinding
    : ASSIGN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 19. MATERIAL COMPOSITION
 * ============================================================================
 *
 * Composition describes logical relationships among material entities.
 *
 * It does NOT prescribe:
 *
 *     physical geometry;
 *     manufacturing process;
 *     molecular topology;
 *     atomic coordinates;
 *     laboratory equipment.
 *
 * Those are semantic or downstream concerns.
 *
 * Examples:
 *
 *     @composition substrate(material_a, material_b);
 *
 *     @composition composite {
 *         ...
 *     }
 */

materialComposition
    : AT
      identifier
      materialCompositionTail
    ;


materialCompositionTail
    : LPAREN
      argumentList?
      RPAREN
      materialPostInvocation
    | materialBody
    | SEMICOLON
    ;


/*
 * ============================================================================
 * 20. MATERIAL OPERATION
 * ============================================================================
 *
 * Operations are open-world.
 *
 * The grammar MUST NOT enumerate:
 *
 *     alloy
 *     anneal
 *     deposit
 *     etch
 *     crystallize
 *     synthesize
 *     fabricate
 *     etc.
 *
 * Those are semantic operations represented by ordinary names.
 *
 * Examples:
 *
 *     @transform(material, process);
 *
 *     @operation synthesize(source, parameters);
 *
 *     @operation custom_future_operation(...);
 */

materialOperation
    : AT
      identifier
      materialOperationTail
    ;


materialOperationTail
    : LPAREN
      argumentList?
      RPAREN
      materialPostInvocation
    | materialDeclaration
    | SEMICOLON
    ;


/*
 * ============================================================================
 * 21. MATERIAL REQUIREMENTS
 * ============================================================================
 *
 * Requirements express semantic constraints.
 *
 * Examples:
 *
 *     @requires(capability("material.simulation"));
 *
 *     @requires(material_model);
 *
 *     @requires(resource_requirement);
 *
 * The grammar does not determine whether a requirement can be satisfied.
 *
 * Resource and capability resolution occur downstream.
 */

materialRequirement
    : AT
      identifier
      LPAREN
      argumentList?
      RPAREN
      materialPostInvocation
    ;


/*
 * ============================================================================
 * 22. MATERIAL CAPABILITIES
 * ============================================================================
 *
 * Capabilities describe target/environment abilities.
 *
 * Examples:
 *
 *     @capability(capability("material.simulation"));
 *
 *     @capability(capability("molecular.compute"));
 *
 *     @capability(capability("quantum.materials"));
 *
 * The actual capability model belongs to:
 *
 *     grammar/resources/
 *     grammar/hardware/
 *
 * This grammar merely provides the structural integration point.
 */

materialCapability
    : AT
      identifier
      LPAREN
      argumentList?
      RPAREN
      materialPostInvocation
    ;


/*
 * ============================================================================
 * 23. MATERIAL CONSTRAINTS
 * ============================================================================
 *
 * Constraints express semantic conditions without becoming physical
 * implementation rules.
 *
 * Examples:
 *
 *     @constraint(condition);
 *
 *     @constraint(property >= requirement);
 *
 *     @constraint(model_parameter == value);
 */

materialConstraint
    : AT
      identifier
      LPAREN
      argumentList?
      RPAREN
      materialPostInvocation
    ;


/*
 * ============================================================================
 * 24. MATERIAL OBSERVATION
 * ============================================================================
 *
 * Observations may represent source-level requests to observe or record
 * material-domain information.
 *
 * The grammar does not prescribe:
 *
 *     a particular sensor;
 *     measurement device;
 *     laboratory instrument;
 *     simulation engine;
 *     observation database.
 *
 * Examples:
 *
 *     @observe(sample);
 *
 *     @observe(material, observable);
 *
 *     @observe(material, observable) {
 *         ...
 *     }
 */

materialObservation
    : AT
      identifier
      LPAREN
      argumentList?
      RPAREN
      materialPostInvocation
    ;


/*
 * ============================================================================
 * 25. MATERIAL TRANSFORMATION
 * ============================================================================
 *
 * Transformations are intentionally open-world.
 *
 * Examples:
 *
 *     @transform(source, transformation);
 *
 *     @transform(material, model);
 *
 *     @transform(material, target_representation);
 *
 * The semantic layer determines whether the transformation is:
 *
 *     symbolic;
 *     numerical;
 *     physical;
 *     simulated;
 *     synthesized;
 *     compiled;
 *     optimized;
 *     otherwise valid.
 */

materialTransformation
    : AT
      identifier
      LPAREN
      argumentList?
      RPAREN
      materialPostInvocation
    ;


/*
 * ============================================================================
 * 26. NESTED MATERIAL STRUCTURE
 * ============================================================================
 *
 * Material constructs may be nested without a grammar-level nesting limit.
 *
 * Example:
 *
 *     @material system {
 *         @material substrate {
 *             @property composition = value;
 *         }
 *
 *         @material coating {
 *             @property thickness = value;
 *         }
 *     }
 *
 * The grammar does not encode a maximum nesting depth.
 *
 * Parser/runtime stack limitations remain implementation concerns and must
 * not be promoted to language semantics.
 */


/*
 * ============================================================================
 * 27. ATOMIC AND MOLECULAR INTEGRATION
 * ============================================================================
 *
 * Material descriptions may reference atomic or molecular entities.
 *
 * This file deliberately does NOT define:
 *
 *     atom grammar;
 *     molecule grammar;
 *     bonding grammar;
 *     periodic-table grammar;
 *     molecular-dynamics grammar.
 *
 * Those concepts belong to their own domain contracts when present.
 *
 * Material source may therefore refer to them through:
 *
 *     identifiers;
 *     type expressions;
 *     expressions;
 *     annotations;
 *     semantic references.
 *
 * Example:
 *
 *     @material substrate {
 *         @composition atoms;
 *         @composition molecules;
 *     }
 *
 * The semantic layer determines what those references mean.
 *
 * ============================================================================
 * 28. NANO AGENT INTEGRATION
 * ============================================================================
 *
 * Nano-agent structural syntax remains owned by:
 *
 *     grammar/nano/agents.g4
 *
 * This file MUST NOT redefine nano-agent declarations.
 *
 * A material may participate in an agent program through the shared
 * composition architecture.
 *
 * Conceptually:
 *
 *     NanoAgents
 *          |
 *          +---- NanoMaterials
 *          |
 *          +---- NanoProcesses
 *          |
 *          +---- NanoInteractions
 *
 * provided those future domains are separately specified and composed by
 * the universal parser.
 *
 * No second nano-agent IR is created here.
 *
 * ============================================================================
 * 29. QUANTUM INTEGRATION
 * ============================================================================
 *
 * Material computation may require quantum computation.
 *
 * This grammar does not own quantum syntax.
 *
 * Quantum constructs remain owned by:
 *
 *     grammar/quantum/
 *
 * Their semantic path remains:
 *
 *     source
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic analysis
 *       |
 *       v
 *     quantum::ir
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     routing / scheduling
 *       |
 *       v
 *     QEC / resilience / ZQN
 *       |
 *       v
 *     HAL
 *       |
 *       v
 *     target realization
 *
 * A material grammar must never introduce:
 *
 *     physical qubits;
 *     physical gate identifiers;
 *     QPU topology;
 *     vendor calibration;
 *     QPU addresses.
 *
 * ============================================================================
 * 30. CLASSICAL AND NUMERICAL INTEGRATION
 * ============================================================================
 *
 * Material computation may use:
 *
 *     numerical methods;
 *     symbolic mathematics;
 *     linear algebra;
 *     tensor computation;
 *     statistics;
 *     optimization;
 *     simulation;
 *     data processing.
 *
 * Those capabilities remain owned by their respective domains.
 *
 * This file only provides the material-domain structural boundary.
 *
 * ============================================================================
 * 31. HARDWARE INTEGRATION
 * ============================================================================
 *
 * Material computation may eventually execute on:
 *
 *     CPU;
 *     GPU;
 *     FPGA;
 *     ASIC;
 *     accelerator;
 *     QPU;
 *     distributed system;
 *     laboratory device;
 *     future hardware.
 *
 * This file MUST NOT select one.
 *
 * Hardware capabilities remain owned by:
 *
 *     grammar/hardware/
 *
 * Resource requirements remain owned by:
 *
 *     grammar/resources/
 *
 * Device discovery, placement, routing, scheduling, calibration, and
 * physical execution remain downstream responsibilities.
 *
 * ============================================================================
 * 32. RESOURCE INTEGRATION
 * ============================================================================
 *
 * Material programs may express resource requirements through existing
 * resource/capability syntax.
 *
 * Examples:
 *
 *     @requires(resource_requirement);
 *
 *     @capability(capability("material.simulation"));
 *
 *     @requires(memory_requirement);
 *
 * This grammar does not define resource quantities or limits.
 *
 * A source-level requirement is not equivalent to a physical allocation.
 *
 * For example:
 *
 *     required material simulation capacity
 *
 * is semantically different from:
 *
 *     execute on device X
 *
 * Target realization belongs downstream.
 *
 * ============================================================================
 * 33. DATA AND PROVENANCE INTEGRATION
 * ============================================================================
 *
 * Material models may refer to:
 *
 *     datasets;
 *     measurements;
 *     observations;
 *     simulations;
 *     provenance;
 *     schemas;
 *     external scientific resources.
 *
 * Data semantics remain owned by:
 *
 *     grammar/data/
 *
 * Provenance semantics remain owned by the appropriate provenance/security
 * architecture.
 *
 * This grammar does not embed a scientific database schema.
 *
 * ============================================================================
 * 34. INTEROPERABILITY
 * ============================================================================
 *
 * Material computation may interoperate with external representations,
 * scientific tools, simulation systems, laboratory systems, or hardware
 * interfaces.
 *
 * Interoperability remains owned by:
 *
 *     grammar/interoperability/
 *
 * This grammar must not introduce vendor-specific syntax merely because
 * an external system exists.
 *
 * ============================================================================
 * 35. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for:
 *
 *     - annotation resolution;
 *     - material identity;
 *     - material scope;
 *     - reference resolution;
 *     - type checking;
 *     - property validation;
 *     - composition validation;
 *     - operation validation;
 *     - requirement validation;
 *     - capability checking;
 *     - resource checking;
 *     - effect checking;
 *     - provenance checking;
 *     - security checking;
 *     - physical-model validation;
 *     - unit consistency;
 *     - dimensional consistency;
 *     - domain interoperability;
 *     - portability;
 *     - determinism;
 *     - model compatibility.
 *
 * Parsing MUST NOT perform these operations.
 *
 * ============================================================================
 * 36. PHYSICAL-MODEL SEPARATION
 * ============================================================================
 *
 * The grammar must remain independent from physical-model implementation.
 *
 * The following do NOT belong in this file:
 *
 *     periodic-table entries;
 *     atomic weights;
 *     bond energies;
 *     crystal databases;
 *     material catalogues;
 *     empirical constants;
 *     simulation equations;
 *     solver algorithms;
 *     fabrication procedures;
 *     device calibration.
 *
 * Such information may exist in:
 *
 *     libraries;
 *     semantic registries;
 *     scientific databases;
 *     domain models;
 *     simulation backends;
 *     external resources.
 *
 * ============================================================================
 * 37. UNIT AND DIMENSION INTEGRATION
 * ============================================================================
 *
 * Material properties may require physical units.
 *
 * This grammar does not create a second unit system.
 *
 * Unit syntax and dimensional semantics remain owned by the universal
 * expression/type/value architecture.
 *
 * Therefore a material property can structurally contain:
 *
 *     expression
 *
 * without this file deciding whether the expression denotes:
 *
 *     length;
 *     mass;
 *     energy;
 *     temperature;
 *     pressure;
 *     conductivity;
 *     or another quantity.
 *
 * ============================================================================
 * 38. GENERIC MATERIAL TYPES
 * ============================================================================
 *
 * This grammar intentionally permits canonical type expressions.
 *
 * Examples:
 *
 *     @material sample: Material;
 *
 *     @material sample: Material<Model>;
 *
 *     @material sample: Material<T>;
 *
 *     @material sample: Material<Shape>;
 *
 * Exact type validity belongs to the type system.
 *
 * This grammar MUST NOT introduce a material-specific type syntax that
 * duplicates Types.
 *
 * ============================================================================
 * 39. PARAMETERIZATION
 * ============================================================================
 *
 * Material declarations may use expressions as parameters.
 *
 * Examples:
 *
 *     @material sample = model(parameter);
 *
 *     @material sample: Material = construct(parameters);
 *
 * Parameter cardinality is not bounded by this grammar.
 *
 * Parameter representation is delegated to the canonical expression grammar.
 *
 * ============================================================================
 * 40. COMPOSITIONAL SCALABILITY
 * ============================================================================
 *
 * The grammar permits:
 *
 *     material
 *       -> material
 *       -> material
 *       -> ...
 *
 * without a language-level cardinality limit.
 *
 * The same applies to:
 *
 *     properties;
 *     components;
 *     transformations;
 *     observations;
 *     nested declarations;
 *     operations;
 *     references.
 *
 * Implementation resource exhaustion is handled by the compiler/runtime
 * resource model rather than by hard-coded language constants.
 *
 * ============================================================================
 * 41. DETERMINISM
 * ============================================================================
 *
 * Parsing this grammar is deterministic with respect to:
 *
 *     source;
 *     token sequence;
 *     grammar version.
 *
 * Parsing MUST NOT depend on:
 *
 *     hardware;
 *     available memory;
 *     CPU count;
 *     GPU availability;
 *     QPU availability;
 *     filesystem state;
 *     network state;
 *     wall-clock time;
 *     randomness;
 *     external material databases;
 *     laboratory state.
 *
 * Database/model lookup is semantic or downstream behavior.
 *
 * ============================================================================
 * 42. SECURITY
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no embedded Rust;
 *     no actions;
 *     no semantic predicates;
 *     no unsafe code;
 *     no filesystem access;
 *     no network access;
 *     no process execution;
 *     no hardware discovery.
 *
 * Material annotations MUST NOT bypass:
 *
 *     capability checks;
 *     authorization;
 *     resource checks;
 *     provenance;
 *     security policy;
 *     foreign-function controls.
 *
 * ============================================================================
 * 43. SOURCE SPANS AND DIAGNOSTICS
 * ============================================================================
 *
 * The parser must preserve the normal ANTLR source context for all material
 * constructs.
 *
 * Diagnostics belong to the repository's common diagnostics architecture.
 *
 * This file does not define a second diagnostic system.
 *
 * Diagnostics should be capable of identifying:
 *
 *     annotation;
 *     material name;
 *     property;
 *     operation;
 *     argument;
 *     type expression;
 *     initializer;
 *     body;
 *     source span.
 *
 * ============================================================================
 * 44. AST CONTRACT
 * ============================================================================
 *
 * This grammar MUST map into the existing domain-neutral frontend AST.
 *
 * Preferred structural mapping:
 *
 *     materialAnnotatedConstruct
 *          -> generic annotation/declaration/statement structure
 *
 *     materialDeclaration
 *          -> generic declaration structure
 *
 *     materialProperty
 *          -> generic annotated/member structure
 *
 *     materialOperation
 *          -> generic operation/invocation structure
 *
 *     materialBody
 *          -> generic block structure
 *
 *     expression
 *          -> existing expression AST
 *
 *     typeExpression
 *          -> existing type AST
 *
 * The grammar MUST NOT require:
 *
 *     NanoMaterialAst
 *     MaterialDatabaseAst
 *     PeriodicTableAst
 *
 * merely to parse this domain.
 *
 * If specialized semantic information is required, it belongs after the
 * domain-neutral AST boundary.
 *
 * ============================================================================
 * 45. IR CONTRACT
 * ============================================================================
 *
 * This grammar does NOT define a material-specific IR.
 *
 * Material semantics must map into the repository's canonical semantic/IR
 * architecture.
 *
 * If material computation eventually requires a dedicated IR boundary, that
 * boundary must be specified independently and integrated through the common
 * semantic model.
 *
 * It MUST NOT be introduced implicitly by this grammar.
 *
 * Quantum portions of a material computation continue to terminate at:
 *
 *     quantum::ir
 *
 * Hardware portions continue through the hardware semantic boundary.
 *
 * Classical computation continues through the classical IR architecture.
 *
 * ============================================================================
 * 46. COMPILER INTEGRATION
 * ============================================================================
 *
 * The compiler consumes the semantic representation produced after parsing.
 *
 * The compiler is responsible for deciding, according to semantic rules and
 * target capabilities, whether material computation is realized through:
 *
 *     symbolic execution;
 *     numerical execution;
 *     simulation;
 *     quantum execution;
 *     classical execution;
 *     accelerator execution;
 *     distributed execution;
 *     external interoperability.
 *
 * This grammar does not select a backend.
 *
 * ============================================================================
 * 47. RUNTIME INTEGRATION
 * ============================================================================
 *
 * Runtime behavior is downstream from parsing.
 *
 * Runtime responsibilities may include:
 *
 *     model execution;
 *     simulation;
 *     observation;
 *     resource management;
 *     device interaction;
 *     fault handling;
 *     provenance;
 *     distributed execution.
 *
 * None of those behaviors may be implemented through grammar actions.
 *
 * ============================================================================
 * 48. POCO-REAF INTEGRATION
 * ============================================================================
 *
 * A valid material program should remain expressible without specifying:
 *
 *     CPU count;
 *     GPU count;
 *     FPGA count;
 *     QPU count;
 *     node count;
 *     thread count;
 *     memory capacity;
 *     register width;
 *     device identifier;
 *     physical topology.
 *
 * The compiler may specialize the material computation for available
 * resources.
 *
 * Such specialization must preserve specified semantics.
 *
 * ============================================================================
 * 49. RESOURCE FAILURE
 * ============================================================================
 *
 * If a target cannot satisfy a material program's semantic requirements,
 * downstream compilation/deployment may reject it.
 *
 * This grammar must not silently:
 *
 *     reduce precision;
 *     remove properties;
 *     alter transformations;
 *     change physical meaning;
 *     weaken correctness guarantees.
 *
 * Any permitted fallback must be defined by semantic policy.
 *
 * ============================================================================
 * 50. COMPATIBILITY
 * ============================================================================
 *
 * This file follows the existing compatibility architecture.
 *
 * Existing stable lexical tokens are reused.
 *
 * Existing general grammar rules are reused.
 *
 * No existing token is renamed here.
 *
 * No existing parser rule is redefined here.
 *
 * If material-domain syntax changes in a future language version, the change
 * must be recorded through:
 *
 *     grammar/compatibility/
 *     grammar/spec/compatibility.md
 *
 * and reflected in:
 *
 *     grammar/grammar.md
 *
 * ============================================================================
 * 51. VALIDATION REQUIREMENTS
 * ============================================================================
 *
 * grammar/validation/ must verify:
 *
 *     - grammar name matches NanoMaterials;
 *     - file ownership is unique;
 *     - tokenVocab is ZamaniLexer;
 *     - imported grammar names resolve;
 *     - no lexer rules exist here;
 *     - no semantic actions exist;
 *     - no semantic predicates exist;
 *     - no physical database is encoded;
 *     - no fixed material catalogue exists;
 *     - no universal resource ceiling exists;
 *     - no machine-specific allocation exists;
 *     - no fixed periodic table exists;
 *     - no duplicate type grammar exists;
 *     - no duplicate expression grammar exists;
 *     - no second material IR is implied;
 *     - parser structure remains target-independent.
 *
 * ============================================================================
 * 52. REQUIRED POSITIVE TEST COVERAGE
 * ============================================================================
 *
 * Tests must cover at minimum:
 *
 *     @material Graphene;
 *
 *     @material Graphene: Material;
 *
 *     @material Graphene = model;
 *
 *     @material Graphene: Material = model;
 *
 *     @material Graphene {
 *         ...
 *     }
 *
 *     @property density = value;
 *
 *     @property custom_property = value;
 *
 *     @composition composite(a, b);
 *
 *     @operation transform(a, b);
 *
 *     @requires(capability("material.simulation"));
 *
 *     @capability(capability("material.simulation"));
 *
 *     @constraint(condition);
 *
 *     @observe(sample);
 *
 *     @transform(source, target);
 *
 *     nested material declarations;
 *
 *     material declarations containing ordinary Zamani statements;
 *
 *     material declarations containing canonical expressions;
 *
 *     material declarations containing canonical type expressions.
 *
 * ============================================================================
 * 53. REQUIRED NEGATIVE TEST COVERAGE
 * ============================================================================
 *
 * Tests must reject malformed constructs such as:
 *
 *     @material
 *     @material (
 *     @material Name :
 *     @material Name =
 *     @material Name {
 *
 * where the canonical parser cannot legally complete the construct.
 *
 * Tests must also verify that invalid material syntax is not silently
 * interpreted as valid source.
 *
 * ============================================================================
 * 54. REQUIRED BOUNDARY TEST COVERAGE
 * ============================================================================
 *
 * Boundary tests must include:
 *
 *     empty material body;
 *     one property;
 *     many properties;
 *     nested materials;
 *     deeply nested valid source;
 *     large argument lists;
 *     large expressions;
 *     large symbolic dimensions;
 *     user-defined material names;
 *     unusual but valid Unicode identifiers where permitted;
 *     custom annotations;
 *     unknown material operation names;
 *     unknown property names.
 *
 * None of these tests may establish an artificial universal maximum.
 *
 * ============================================================================
 * 55. REQUIRED SCALABILITY TEST COVERAGE
 * ============================================================================
 *
 * Scalability tests must demonstrate that the grammar does not impose
 * language-level limits on:
 *
 *     material count;
 *     component count;
 *     property count;
 *     nesting;
 *     parameter count;
 *     transformation count;
 *     observation count;
 *     expression size;
 *     model size.
 *
 * Tests should use generated source rather than hard-coded grammar ceilings.
 *
 * ============================================================================
 * 56. REQUIRED PORTABILITY TEST COVERAGE
 * ============================================================================
 *
 * Material source must be tested in contexts involving:
 *
 *     classical computation;
 *     quantum computation;
 *     hybrid computation;
 *     AI/data computation;
 *     hardware-oriented computation;
 *     distributed computation.
 *
 * The same material source must not acquire a different syntactic meaning
 * merely because the eventual target differs.
 *
 * ============================================================================
 * 57. REQUIRED DETERMINISM TEST COVERAGE
 * ============================================================================
 *
 * Given identical:
 *
 *     source;
 *     language version;
 *     grammar version;
 *
 * parsing must produce equivalent parse structure.
 *
 * Test execution must not depend on:
 *
 *     target hardware;
 *     resource availability;
 *     network;
 *     filesystem;
 *     scientific databases;
 *     randomness.
 *
 * ============================================================================
 * 58. REQUIRED CROSS-DOMAIN TESTS
 * ============================================================================
 *
 * At minimum, integration testing must cover:
 *
 *     nano + classical;
 *     nano + quantum;
 *     nano + hybrid;
 *     nano + AI;
 *     nano + data;
 *     nano + hardware;
 *     nano + resources;
 *     nano + interoperability;
 *     nano + security.
 *
 * The material grammar must not duplicate those domain grammars.
 *
 * ============================================================================
 * 59. HARD-CODING AUDIT
 * ============================================================================
 *
 * This file must remain free of:
 *
 *     MAX_MATERIALS
 *     MAX_MATERIAL_COMPONENTS
 *     MAX_MATERIAL_PROPERTIES
 *     MAX_MATERIAL_DIMENSIONS
 *     MAX_ATOMS
 *     MAX_MOLECULES
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_THREADS
 *     MAX_MEMORY
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *
 * More generally, no grammar construct may turn current implementation
 * capacity into a universal language restriction.
 *
 * ============================================================================
 * 60. SAFE RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains no Rust code.
 *
 * The repository implementation consuming it must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and the requested production safety policy:
 *
 *     safe Rust only;
 *     no unsafe implementation requirement.
 *
 * No grammar action may introduce Rust unsafe behavior.
 *
 * ============================================================================
 * 61. INTEGRATION WITH grammar/nano/agents.g4
 * ============================================================================
 *
 * Existing file:
 *
 *     grammar/nano/agents.g4
 *
 * remains the owner of nano-agent syntax.
 *
 * This file does not redefine:
 *
 *     nanoAgentConstruct
 *     nanoAgentDeclaration
 *     nanoAgentBody
 *     nanoAgentInteraction
 *
 * The universal parser should compose both grammars at the domain level.
 *
 * Conceptually:
 *
 *     NanoDomain
 *       |
 *       +-- NanoAgents
 *       |
 *       +-- NanoMaterials
 *       |
 *       +-- future NanoAtoms
 *       |
 *       +-- future NanoMolecules
 *       |
 *       +-- future NanoInteractions
 *
 * This prevents the material grammar from becoming a second nano language.
 *
 * ============================================================================
 * 62. INTEGRATION WITH grammar/nano/ FUTURE FILES
 * ============================================================================
 *
 * If future files are added for:
 *
 *     atoms.g4
 *     molecules.g4
 *     interactions.g4
 *     protocols.g4
 *     processes.g4
 *
 * they MUST:
 *
 *     - use ZamaniLexer;
 *     - reuse Types/Expressions/Statements;
 *     - avoid duplicate material rules;
 *     - avoid duplicate annotations;
 *     - avoid physical database encoding;
 *     - avoid hard-coded capacity limits;
 *     - map to the domain-neutral AST;
 *     - integrate through the common semantic model.
 *
 * ============================================================================
 * 63. INTEGRATION WITH grammar/Zamani.g4
 * ============================================================================
 *
 * The root grammar:
 *
 *     grammar/Zamani.g4
 *
 * remains the composition root.
 *
 * It is responsible for making the material domain reachable from the
 * universal declaration/statement/expression dispatch.
 *
 * This file MUST NOT become a second composition root.
 *
 * The integration should conceptually expose:
 *
 *     materialConstruct
 *
 * through the appropriate universal domain dispatch.
 *
 * The exact dispatch rule remains owned by Zamani.g4.
 *
 * ============================================================================
 * 64. INTEGRATION WITH grammar/grammar.md
 * ============================================================================
 *
 * grammar/grammar.md remains the implementation-conformance reference.
 *
 * It should report this domain using the repository's conformance lifecycle:
 *
 *     SPECIFIED
 *     IMPLEMENTED
 *     PARTIALLY IMPLEMENTED
 *     PLANNED
 *     DEPRECATED
 *
 * Presence of this file alone MUST NOT mark the material domain as fully
 * implemented.
 *
 * Full implementation additionally requires:
 *
 *     lexer conformance;
 *     parser conformance;
 *     AST mapping;
 *     semantic implementation;
 *     IR integration;
 *     compiler integration;
 *     tests.
 *
 * ============================================================================
 * 65. INTEGRATION WITH grammar/Zamani-Grammar.md
 * ============================================================================
 *
 * Zamani-Grammar.md remains extended/historical/proposed language material.
 *
 * It does not override this file.
 *
 * A material feature becomes stable only through the repository's normal
 * promotion process:
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
 *     IR
 *       |
 *       v
 *     tests
 *       |
 *       v
 *     stable
 *
 * ============================================================================
 * 66. INTEGRATION WITH grammar/specification/domains.md
 * ============================================================================
 *
 * The domain specification states that nano-oriented syntax may describe:
 *
 *     atoms;
 *     molecules;
 *     materials;
 *     interactions;
 *     nano-agents;
 *     nanoscale processes;
 *     capabilities.
 *
 * It also requires that a material database, periodic table, or physical
 * simulator remain outside the universal grammar.
 *
 * This file implements that architectural rule at the material syntax
 * boundary.
 *
 * ============================================================================
 * 67. INTEGRATION WITH grammar/spec/semantics.md
 * ============================================================================
 *
 * The semantic specification remains responsible for the meaning of material
 * entities and operations.
 *
 * Therefore this grammar does not validate:
 *
 *     material composition;
 *     physical feasibility;
 *     unit compatibility;
 *     thermodynamic validity;
 *     chemical validity;
 *     simulation validity.
 *
 * Such validation belongs downstream.
 *
 * ============================================================================
 * 68. INTEGRATION WITH HARDWARE AND HAL
 * ============================================================================
 *
 * A material program may eventually be realized by hardware.
 *
 * However:
 *
 *     material syntax
 *          !=
 *     hardware topology
 *
 * and:
 *
 *     material requirement
 *          !=
 *     physical allocation.
 *
 * HAL remains responsible for target-specific hardware state and capabilities.
 *
 * ============================================================================
 * 69. INTEGRATION WITH QEC / ZQN
 * ============================================================================
 *
 * If material computation uses quantum computation:
 *
 *     material syntax
 *          |
 *          v
 *     generic AST
 *          |
 *          v
 *     semantic model
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     QEC / resilience / ZQN
 *
 * This grammar introduces no QEC or ZQN rules.
 *
 * ============================================================================
 * 70. INTEGRATION WITH COMPILATION AND EXECUTION
 * ============================================================================
 *
 * Compilation may choose an implementation based on:
 *
 *     semantic requirements;
 *     capabilities;
 *     resources;
 *     constraints;
 *     target properties;
 *     optimization policy.
 *
 * Execution may then choose:
 *
 *     CPU;
 *     GPU;
 *     FPGA;
 *     ASIC;
 *     QPU;
 *     distributed resources;
 *     simulation;
 *     external systems.
 *
 * This file never selects the realization.
 *
 * ============================================================================
 * 71. IMPLEMENTATION LIMITS VS LANGUAGE LIMITS
 * ============================================================================
 *
 * Implementations necessarily operate with finite resources.
 *
 * This does not justify introducing a grammar-level limit.
 *
 * For example, if an implementation currently cannot simulate a particular
 * material model because of memory exhaustion, that is an execution/resource
 * issue, not a reason to add:
 *
 *     MAX_MATERIAL_SIZE
 *
 * to the grammar.
 *
 * ============================================================================
 * 72. FAILURE SEMANTICS
 * ============================================================================
 *
 * Downstream failures may include:
 *
 *     invalid material reference;
 *     invalid property;
 *     invalid unit;
 *     invalid model;
 *     missing capability;
 *     insufficient resources;
 *     unsupported target;
 *     unavailable backend;
 *     security violation;
 *     invalid interoperability;
 *     semantic incompatibility.
 *
 * These belong to semantic/diagnostic/runtime layers.
 *
 * Parsing only establishes syntactic validity.
 *
 * ============================================================================
 * 73. NO VENDOR LOCK-IN
 * ============================================================================
 *
 * Vendor names may occur as ordinary source identifiers or data.
 *
 * They MUST NOT become universal material keywords solely because a vendor
 * provides a material database, simulator, fabrication process, or device.
 *
 * Vendor integration belongs to:
 *
 *     interoperability;
 *     dialects;
 *     libraries;
 *     backend adapters;
 *     target-specific compilation.
 *
 * ============================================================================
 * 74. NO SCIENTIFIC DATABASE LOCK-IN
 * ============================================================================
 *
 * This file deliberately does not encode a particular:
 *
 *     periodic table;
 *     materials database;
 *     chemistry database;
 *     crystal database;
 *     biological database;
 *     simulation catalogue.
 *
 * Scientific data must remain replaceable and versionable independently of
 * the language grammar.
 *
 * ============================================================================
 * 75. EXTENSIBILITY
 * ============================================================================
 *
 * New material concepts should preferably be expressible using:
 *
 *     annotations;
 *     identifiers;
 *     types;
 *     expressions;
 *     generic operations;
 *     semantic capabilities.
 *
 * A new keyword should be introduced only when the concept genuinely
 * requires language-level lexical or syntactic semantics.
 *
 * ============================================================================
 * 76. PRODUCTION COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is grammatically complete when:
 *
 * [x] File ownership is defined.
 * [x] Non-ownership is defined.
 * [x] Canonical lexer is identified.
 * [x] Canonical parser dependencies are identified.
 * [x] Public entry point exists.
 * [x] Material declarations are supported.
 * [x] Material references are supported.
 * [x] Material typing is delegated to Types.
 * [x] Material values are delegated to Expressions.
 * [x] Material bodies are supported.
 * [x] Material properties are supported.
 * [x] Material composition is supported.
 * [x] Material operations are open-world.
 * [x] Requirements are structurally supported.
 * [x] Capabilities are structurally supported.
 * [x] Constraints are structurally supported.
 * [x] Observations are structurally supported.
 * [x] Transformations are structurally supported.
 * [x] Nested constructs are supported.
 * [x] No fixed material catalogue exists.
 * [x] No periodic table is embedded.
 * [x] No physical constants are embedded.
 * [x] No material database is embedded.
 * [x] No artificial resource ceilings exist.
 * [x] No hardware target is selected.
 * [x] No vendor is privileged.
 * [x] No quantum gate set is embedded.
 * [x] No QEC semantics are duplicated.
 * [x] No ZQN semantics are duplicated.
 * [x] No HAL semantics are duplicated.
 * [x] No lexer rules are introduced.
 * [x] No expression grammar is duplicated.
 * [x] No type grammar is duplicated.
 * [x] No statement grammar is duplicated.
 * [x] Safe Rust compatibility is documented.
 * [x] Rust 1.97 / 1.97.1 compatibility is documented.
 * [x] POCO-REAF integration is documented.
 * [x] AST integration is documented.
 * [x] IR integration is documented.
 * [x] Compiler integration is documented.
 * [x] Runtime integration is documented.
 * [x] Validation requirements are documented.
 * [x] Test requirements are documented.
 *
 * Full feature status remains dependent on the downstream implementation
 * contracts listed above.
 *
 * ============================================================================
 * 77. FINAL ARCHITECTURAL INVARIANT
 * ============================================================================
 *
 * This file answers one question:
 *
 *     "What material-oriented source structures are syntactically valid?"
 *
 * It does NOT answer:
 *
 *     "What does this material physically mean?"
 *     "Which scientific database defines it?"
 *     "Which simulator executes it?"
 *     "Which CPU/GPU/QPU executes it?"
 *     "Which device is selected?"
 *     "How is it routed?"
 *     "How is it scheduled?"
 *     "How is it calibrated?"
 *
 * Those responsibilities remain downstream.
 *
 * The complete architecture remains:
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
 *     canonical semantic IR
 *          |
 *          v
 *     optimization
 *          |
 *          +------------------+
 *          |                  |
 *          v                  v
 *       classical         quantum::ir
 *          |                  |
 *          +--------+---------+
 *                   |
 *                   v
 *        routing / scheduling /
 *        resilience / ZQN
 *                   |
 *                   v
 *                  HAL
 *                   |
 *                   v
 *             target realization
 *
 * Therefore:
 *
 *     PROGRAM ONCE
 *          ->
 *     COMPILE ONCE
 *          ->
 *     RUN EVERYWHERE
 *          ->
 *     RUN ANYWHERE
 *          ->
 *     RUN FOREVER
 *
 * subject to the actual semantics of the program and the resources and
 * capabilities available to the realization environment.
 *
 * ============================================================================
 */
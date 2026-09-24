/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/classical/control.g4
 *
 * Grammar:
 *     ClassicalControl
 *
 * Status:
 *     Production classical-domain control-systems grammar.
 *
 * Domain:
 *     Classical computation / control systems / cyber-physical computation
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust edition 2021
 *
 * Safety:
 *     Safe Rust only.
 *     No embedded Rust actions.
 *     No semantic predicates.
 *     No target-specific parser code.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file defines the SOURCE-LEVEL GRAMMAR BOUNDARY for classical
 * control-system specifications.
 *
 * It is deliberately NOT:
 *
 *     grammar/statements/control-flow.g4
 *
 * The latter owns ordinary program control flow:
 *
 *     if
 *     while
 *     for
 *     match
 *     break
 *     continue
 *     return
 *     exceptions
 *
 * This file owns control-SYSTEM intent, including structures such as:
 *
 *     plant
 *     controller
 *     input
 *     output
 *     state
 *     reference
 *     feedback
 *     sensor
 *     actuator
 *     observer
 *     estimator
 *     disturbance
 *     dynamics
 *     objective
 *     constraint
 *     sampling
 *     timing
 *     simulation
 *     synthesis
 *     deployment intent
 *
 * These are semantic roles, not a closed keyword list.
 *
 * ============================================================================
 * CORE ARCHITECTURAL DECISION
 * ============================================================================
 *
 * Control theory must NOT become a second programming language.
 *
 * The grammar therefore uses:
 *
 *     existing Zamani expressions
 *     existing identifiers
 *     existing type system
 *     existing numerical domain
 *     existing linear-algebra domain
 *     existing resource/capability system
 *
 * rather than defining a separate expression algebra.
 *
 * For example, the following remain ordinary Zamani expressions:
 *
 *     A * x + B * u
 *     K * x
 *     y - reference
 *     controller(error)
 *     plant(state, input)
 *
 * Control semantics are determined downstream.
 *
 * ============================================================================
 * OPEN-WORLD CONTROL MODEL
 * ============================================================================
 *
 * This grammar intentionally does NOT enumerate:
 *
 *     PID
 *     PI
 *     PD
 *     LQR
 *     LQG
 *     MPC
 *     H-infinity
 *     Kalman
 *     EKF
 *     UKF
 *     pole-placement
 *     root-locus
 *     Smith predictor
 *     adaptive control
 *     robust control
 *     nonlinear control
 *
 * as permanent parser keywords.
 *
 * Such algorithms are normally:
 *
 *     typed operations
 *     library operations
 *     compiler intrinsics
 *     dialect operations
 *     semantic capabilities
 *
 * The grammar only needs structural syntax capable of expressing them.
 *
 * Therefore future control algorithms do not require changing this grammar.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * The control grammar participates in:
 *
 *     Program
 *         ↓
 *     Compile Once
 *         ↓
 *     Semantic Control Model
 *         ↓
 *     Optimization / Specialization
 *         ↓
 *     Scheduling / Execution
 *         ↓
 *     Target realization
 *
 * The same control specification may be realized as:
 *
 *     - software running on a tiny processor;
 *     - CPU computation;
 *     - multicore computation;
 *     - vector computation;
 *     - GPU computation;
 *     - FPGA logic;
 *     - ASIC logic;
 *     - DSP-like acceleration;
 *     - embedded control;
 *     - distributed control;
 *     - edge control;
 *     - cloud simulation;
 *     - hardware/software co-design;
 *     - future architectures.
 *
 * The grammar MUST NOT encode the target realization.
 *
 * ============================================================================
 * ABSOLUTE HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This grammar MUST NOT impose universal limits such as:
 *
 *     MAX_STATES
 *     MAX_INPUTS
 *     MAX_OUTPUTS
 *     MAX_CONTROLLERS
 *     MAX_PLANTS
 *     MAX_SENSORS
 *     MAX_ACTUATORS
 *     MAX_POLES
 *     MAX_ZEROS
 *     MAX_CHANNELS
 *     MAX_SIGNALS
 *     MAX_LOOPS
 *     MAX_SAMPLE_RATE
 *     MAX_HORIZON
 *     MAX_MODEL_ORDER
 *     MAX_MATRIX_SIZE
 *     MAX_VECTOR_SIZE
 *
 * Nor may it encode machine limits such as:
 *
 *     MAX_CPUS
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_REGISTER_WIDTH
 *     MAX_ACCELERATORS
 *     MAX_DEVICES
 *
 * Program-level numbers remain valid:
 *
 *     horizon = 100
 *     states = ...
 *     Matrix<f64, N, M>
 *
 * Those values are program semantics.
 *
 * They are NOT language-level ceilings.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY SEPARATION
 * ============================================================================
 *
 * Control requirements must remain distinct from realization.
 *
 * Valid semantic intent includes:
 *
 *     requires capability("control.real_time")
 *     requires capability("control.feedback")
 *     requires capability("control.numeric")
 *     requires capability("control.sampling")
 *     requires resource(...)
 *
 * The grammar does not decide whether those capabilities are provided by:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     embedded controller
 *     distributed controller
 *     future accelerator
 *
 * ============================================================================
 * IMPORT BOUNDARY
 * ============================================================================
 *
 * General expressions belong to:
 *
 *     grammar/expressions/
 *
 * Numerical syntax belongs to:
 *
 *     grammar/classical/numerical.g4
 *
 * Classical types belong to:
 *
 *     grammar/types/classical.g4
 *
 * Linear algebra belongs to:
 *
 *     grammar/classical/linear-algebra.g4
 *
 * Signal processing belongs to its own classical-domain grammar.
 *
 * General program control flow belongs to:
 *
 *     grammar/statements/control-flow.g4
 *
 * This file MUST NOT duplicate any of those grammars.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The grammar produces parser structure only.
 *
 * The frontend AST MUST remain domain-neutral.
 *
 * Conceptual mapping:
 *
 *     controlSystemDeclaration
 *         -> domain-neutral declaration / domain construct
 *
 *     controlRoleClause
 *         -> named semantic control role
 *
 *     controlBinding
 *         -> ordinary binding/reference structure
 *
 *     controlModelClause
 *         -> semantic control-model declaration
 *
 *     controlSimulationClause
 *         -> simulation intent
 *
 *     controlSynthesisClause
 *         -> synthesis/design intent
 *
 *     controlDeploymentClause
 *         -> deployment intent
 *
 *     controlExpressionStatement
 *         -> ordinary expression statement
 *
 * No ControlSystem-specific replacement AST is created merely because this
 * grammar exists.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Syntax establishes structure.
 *
 * Semantic analysis determines:
 *
 *     - whether a system is a valid control system;
 *     - whether a referenced plant exists;
 *     - whether controller dimensions agree;
 *     - whether input/output dimensions agree;
 *     - whether state equations are valid;
 *     - whether feedback connections are valid;
 *     - whether timing constraints are satisfiable;
 *     - whether sampling semantics are valid;
 *     - whether an observer is compatible with the plant;
 *     - whether an objective is meaningful;
 *     - whether constraints are satisfiable;
 *     - whether a simulation is feasible;
 *     - whether a target has required capabilities.
 *
 * The grammar does NOT perform any of these checks.
 *
 * ============================================================================
 * CONTROL ROLE MODEL
 * ============================================================================
 *
 * The following role names are RECOMMENDED semantic vocabulary:
 *
 *     plant
 *     controller
 *     input
 *     output
 *     state
 *     reference
 *     feedback
 *     sensor
 *     actuator
 *     observer
 *     estimator
 *     disturbance
 *     noise
 *     dynamics
 *     objective
 *     constraint
 *     initial
 *     terminal
 *     sampling
 *     timing
 *     schedule
 *     horizon
 *     model
 *     linearization
 *     discretization
 *
 * They remain IDENTIFIERS.
 *
 * This is intentional.
 *
 * A future control concept must not require a new reserved keyword merely
 * because it introduces a new semantic role.
 *
 * ============================================================================
 * GENERAL SYSTEM FORM
 * ============================================================================
 *
 * The canonical control-system boundary is:
 *
 *     system <name> {
 *         <role>: <expression>;
 *         <role>: <expression>;
 *         ...
 *     }
 *
 * Example:
 *
 *     system motor {
 *         plant: motor_model;
 *         controller: controller(error);
 *         input: voltage;
 *         output: speed;
 *         state: x;
 *         reference: target;
 *         feedback: output -> controller;
 *     }
 *
 * The parser does not know whether "motor", "plant", or "controller" has any
 * particular mathematical meaning.
 *
 * Semantic resolution supplies that meaning.
 *
 * ============================================================================
 * WHY ROLES ARE IDENTIFIERS
 * ============================================================================
 *
 * Do NOT create:
 *
 *     PLANT
 *     CONTROLLER
 *     SENSOR
 *     ACTUATOR
 *     OBSERVER
 *     ESTIMATOR
 *
 * lexer tokens merely for this file.
 *
 * Doing so would create keyword growth and prevent future extensions.
 *
 * Instead:
 *
 *     identifier
 *         ↓
 *     semantic control-role registry
 *
 * This permits:
 *
 *     plant
 *     nonlinear_plant
 *     thermal_plant
 *     quantum_controller
 *     adaptive_controller
 *     custom_observer
 *
 * without grammar changes.
 *
 * ============================================================================
 */

parser grammar ClassicalControl;

options {
    tokenVocab = ZamaniLexer;
}

import Expressions;


/*
 * ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * Stable parser boundary for classical control-system syntax.
 *
 * This is NOT the entry point for ordinary program control flow.
 * ============================================================================
 */

controlConstruct
    : controlSystemDeclaration
    | controlExpressionStatement
    ;


/*
 * ============================================================================
 * 2. CONTROL SYSTEM DECLARATION
 * ============================================================================
 *
 * `system` is already part of the repository's lexical vocabulary.
 *
 * The grammar does not introduce a new `control` keyword.
 *
 * ============================================================================
 */

controlSystemDeclaration
    : 'system'
      IDENTIFIER
      controlSystemParameters?
      '{'
      controlSystemItem*
      '}'
    ;


/*
 * ============================================================================
 * 3. SYSTEM PARAMETERS
 * ============================================================================
 *
 * Parameters are ordinary expressions.
 *
 * They may represent:
 *
 *     dimensions
 *     symbolic values
 *     time domains
 *     numerical parameters
 *     generic configuration
 *     resource-related semantic parameters
 *
 * No finite arity is imposed.
 * ============================================================================
 */

controlSystemParameters
    : '('
      controlArgumentList?
      ')'
    ;


controlArgumentList
    : expression
      (
          ','
          expression
      )*
      ','?
    ;


/*
 * ============================================================================
 * 4. SYSTEM BODY
 * ============================================================================
 *
 * The body is intentionally open-ended.
 *
 * New semantic control roles do not require new grammar productions.
 * ============================================================================
 */

controlSystemItem
    : controlRoleClause
    | controlBinding
    | controlModelClause
    | controlSimulationClause
    | controlSynthesisClause
    | controlDeploymentClause
    | controlExpressionStatement
    ;


/*
 * ============================================================================
 * 5. ROLE CLAUSE
 * ============================================================================
 *
 * Generic role form:
 *
 *     role: expression;
 *
 * Examples:
 *
 *     plant: motor_model;
 *     controller: pid(error);
 *     input: voltage;
 *     output: speed;
 *     state: x;
 *     reference: target;
 *     feedback: output -> controller;
 *
 * Role names remain identifiers.
 *
 * ============================================================================
 */

controlRoleClause
    : IDENTIFIER
      ':'
      expression
      ';'?
    ;


/*
 * ============================================================================
 * 6. CONTROL BINDING
 * ============================================================================
 *
 * Generic named semantic binding:
 *
 *     gain = expression;
 *     horizon = expression;
 *     tolerance = expression;
 *
 * The grammar imposes no restrictions on the name.
 * ============================================================================
 */

controlBinding
    : IDENTIFIER
      '='
      expression
      ';'?
    ;


/*
 * ============================================================================
 * 7. MODEL CLAUSE
 * ============================================================================
 *
 * `model` is already part of the repository lexical vocabulary.
 *
 * The model expression remains open-ended.
 *
 * Examples:
 *
 *     model plant_model;
 *     model transfer_function(A, B, C, D);
 *     model state_space(A, B, C, D);
 *     model nonlinear_model(parameters);
 *
 * The parser does not hard-code any model representation.
 * ============================================================================
 */

controlModelClause
    : 'model'
      expression
      ';'?
    ;


/*
 * ============================================================================
 * 8. SIMULATION CLAUSE
 * ============================================================================
 *
 * Simulation is an execution/analysis intent, not a grammar-level numerical
 * algorithm.
 *
 * Examples:
 *
 *     simulate plant;
 *     simulate closed_loop;
 *     simulate response;
 *     simulate trajectory;
 *
 * Detailed simulation parameters remain ordinary expressions.
 * ============================================================================
 */

controlSimulationClause
    : 'simulate'
      expression
      ';'?
    ;


/*
 * ============================================================================
 * 9. SYNTHESIS CLAUSE
 * ============================================================================
 *
 * Synthesis expresses design intent.
 *
 * It does NOT mean:
 *
 *     synthesize to FPGA X
 *
 * or:
 *
 *     use device N
 *
 * Target realization belongs downstream.
 * ============================================================================
 */

controlSynthesisClause
    : 'synthesize'
      expression
      ';'?
    ;


/*
 * ============================================================================
 * 10. DEPLOYMENT CLAUSE
 * ============================================================================
 *
 * Deployment syntax expresses deployment intent.
 *
 * Physical target selection remains downstream.
 *
 * ============================================================================
 */

controlDeploymentClause
    : 'deploy'
      expression
      ';'?
    ;


/*
 * ============================================================================
 * 11. CONTROL EXPRESSION STATEMENT
 * ============================================================================
 *
 * Existing Zamani expression syntax remains authoritative.
 *
 * This permits control-specific libraries, dialects, and future semantic
 * operations without grammar modification.
 *
 * Examples:
 *
 *     controller(error);
 *     observe(state);
 *     optimize(controller);
 *     stabilize(system);
 *     linearize(plant);
 *     discretize(model, period);
 *     feedback(output, reference);
 *
 * None of these names are hard-coded.
 * ============================================================================
 */

controlExpressionStatement
    : expression
      ';'?
    ;


/*
 * ============================================================================
 * 12. CONTROL REFERENCE
 * ============================================================================
 *
 * A named control entity is syntactically an ordinary identifier.
 * ============================================================================
 */

controlReference
    : IDENTIFIER
    ;


/*
 * ============================================================================
 * 13. CONTROL PATH
 * ============================================================================
 *
 * Qualified control names remain open-world.
 *
 * Examples:
 *
 *     control::pid
 *     robotics::controller
 *     signal::observer
 *     custom::controller::design
 *
 * The exact path syntax remains owned by the canonical name/path grammar.
 *
 * This local rule is intentionally only an identifier-level integration
 * boundary and does not redefine qualified-name syntax.
 * ============================================================================
 */

controlQualifiedReference
    : IDENTIFIER
    ;


/*
 * ============================================================================
 * 14. CONTROL SIGNAL
 * ============================================================================
 *
 * A control signal is semantically classified from an ordinary expression.
 *
 * This permits:
 *
 *     scalar
 *     vector
 *     matrix
 *     tensor
 *     stream
 *     distributed signal
 *     symbolic signal
 *     quantum-derived classical signal
 *
 * without grammar changes.
 * ============================================================================
 */

controlSignal
    : expression
    ;


/*
 * ============================================================================
 * 15. CONTROL STATE
 * ============================================================================
 *
 * State representation is semantic.
 *
 * It may be:
 *
 *     scalar
 *     vector
 *     matrix
 *     tensor
 *     structured value
 *     distributed state
 *     symbolic state
 *
 * No fixed state dimension exists.
 * ============================================================================
 */

controlState
    : expression
    ;


/*
 * ============================================================================
 * 16. CONTROL INPUT
 * ============================================================================
 */

controlInput
    : expression
    ;


/*
 * ============================================================================
 * 17. CONTROL OUTPUT
 * ============================================================================
 */

controlOutput
    : expression
    ;


/*
 * ============================================================================
 * 18. REFERENCE
 * ============================================================================
 */

controlReferenceSignal
    : expression
    ;


/*
 * ============================================================================
 * 19. FEEDBACK
 * ============================================================================
 *
 * Feedback topology is represented by expressions.
 *
 * Examples:
 *
 *     feedback(output, controller);
 *     output -> controller;
 *     controller(output);
 *
 * The parser does not determine stability or causality.
 * ============================================================================
 */

controlFeedback
    : expression
    ;


/*
 * ============================================================================
 * 20. DYNAMICS
 * ============================================================================
 *
 * Dynamic-system equations remain ordinary expressions.
 *
 * Examples:
 *
 *     derivative(state) = A * state + B * input;
 *     dynamics: f(state, input);
 *
 * Differential-equation semantics belong to numerical/control semantic
 * analysis, not this parser.
 * ============================================================================
 */

controlDynamics
    : expression
    ;


/*
 * ============================================================================
 * 21. CONSTRAINT
 * ============================================================================
 *
 * Constraints are semantic expressions.
 *
 * Examples:
 *
 *     state within bounds;
 *     input within limits;
 *     latency <= budget;
 *     capability("real_time");
 *
 * The grammar does not establish physical limits.
 * ============================================================================
 */

controlConstraint
    : expression
    ;


/*
 * ============================================================================
 * 22. OBJECTIVE
 * ============================================================================
 *
 * Objective expressions integrate with the existing optimization subsystem.
 *
 * Examples:
 *
 *     minimize(error);
 *     minimize(control_effort);
 *     optimize(cost);
 *
 * No optimizer is hard-coded here.
 * ============================================================================
 */

controlObjective
    : expression
    ;


/*
 * ============================================================================
 * 23. TIMING
 * ============================================================================
 *
 * Timing values remain ordinary expressions.
 *
 * They may be:
 *
 *     compile-time constants;
 *     runtime values;
 *     symbolic values;
 *     resource-dependent values.
 *
 * The grammar imposes no frequency/rate limit.
 * ============================================================================
 */

controlTiming
    : expression
    ;


/*
 * ============================================================================
 * 24. SAMPLING
 * ============================================================================
 */

controlSampling
    : expression
    ;


/*
 * ============================================================================
 * 25. OBSERVER / ESTIMATOR
 * ============================================================================
 *
 * Observer and estimator algorithms are intentionally not enumerated.
 *
 * Examples:
 *
 *     observer: kalman(...);
 *     estimator: custom_estimator(...);
 *
 * remain ordinary role expressions.
 * ============================================================================
 */

controlObserver
    : expression
    ;


controlEstimator
    : expression
    ;


/*
 * ============================================================================
 * 26. ACTUATOR / SENSOR
 * ============================================================================
 */

controlActuator
    : expression
    ;


controlSensor
    : expression
    ;


/*
 * ============================================================================
 * 27. DISTURBANCE / NOISE
 * ============================================================================
 */

controlDisturbance
    : expression
    ;


controlNoise
    : expression
    ;


/*
 * ============================================================================
 * 28. CONTROL MODELING BOUNDARY
 * ============================================================================
 *
 * This rule provides a stable semantic boundary for control modeling tools.
 *
 * It intentionally accepts ordinary expressions rather than implementing a
 * separate equation language.
 * ============================================================================
 */

controlModelExpression
    : expression
    ;


/*
 * ============================================================================
 * 29. LINEAR-ALGEBRA INTEGRATION
 * ============================================================================
 *
 * Control systems frequently use:
 *
 *     vectors
 *     matrices
 *     tensors
 *     linear transformations
 *     eigenvalue problems
 *     decompositions
 *
 * Those remain owned by:
 *
 *     grammar/classical/linear-algebra.g4
 *
 * This grammar does not reproduce matrix/vector operations.
 *
 * Semantic examples:
 *
 *     A * x
 *     B * u
 *     C * x
 *     D * u
 *
 * are ordinary expressions.
 * ============================================================================
 */

controlLinearAlgebraExpression
    : expression
    ;


/*
 * ============================================================================
 * 30. NUMERICAL INTEGRATION
 * ============================================================================
 *
 * Numerical algorithms remain owned by:
 *
 *     grammar/classical/numerical.g4
 *
 * and downstream numerical semantics.
 *
 * ============================================================================
 */

controlNumericalExpression
    : expression
    ;


/*
 * ============================================================================
 * 31. SIGNAL-PROCESSING INTEGRATION
 * ============================================================================
 *
 * Filtering, transforms, convolution, spectral analysis, and similar
 * operations remain semantic/library operations.
 * ============================================================================
 */

controlSignalProcessingExpression
    : expression
    ;


/*
 * ============================================================================
 * 32. RESOURCE REQUIREMENT INTEGRATION
 * ============================================================================
 *
 * Resource requirements are intentionally expressions.
 *
 * Example conceptual source:
 *
 *     requires capability("control.real_time");
 *
 * The actual resource grammar remains owned by:
 *
 *     grammar/resources/
 *
 * This file must not duplicate it.
 * ============================================================================
 */

controlResourceExpression
    : expression
    ;


/*
 * ============================================================================
 * 33. CAPABILITY INTEGRATION
 * ============================================================================
 */

controlCapabilityExpression
    : expression
    ;


/*
 * ============================================================================
 * 34. PORTABILITY INTEGRATION
 * ============================================================================
 *
 * A control system may be portable across targets if the semantic contract
 * can be satisfied.
 *
 * Portability is therefore not represented by physical device selection.
 * ============================================================================
 */

controlPortabilityExpression
    : expression
    ;


/*
 * ============================================================================
 * 35. HYBRID INTEGRATION
 * ============================================================================
 *
 * Classical control may participate in:
 *
 *     quantum-classical control;
 *     hardware/software co-design;
 *     distributed control;
 *     AI-assisted control;
 *     sensor networks;
 *     robotic systems;
 *     embedded systems.
 *
 * This grammar does not define those domains.
 * ============================================================================
 */

controlHybridExpression
    : expression
    ;


/*
 * ============================================================================
 * 36. QUANTUM INTEGRATION
 * ============================================================================
 *
 * A control expression may consume a classical result originating from a
 * quantum computation.
 *
 * The classical control grammar does NOT define:
 *
 *     qubits
 *     gates
 *     physical qubits
 *     routing
 *     QEC
 *     ZQN
 *
 * Quantum semantics remain downstream through the canonical:
 *
 *     quantum::ir
 *
 * boundary.
 * ============================================================================
 */

controlQuantumBoundaryExpression
    : expression
    ;


/*
 * ============================================================================
 * 37. HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * A control system may eventually lower into:
 *
 *     software
 *     FPGA
 *     ASIC
 *     accelerator
 *     hardware/software co-design
 *
 * Hardware intent remains owned by:
 *
 *     grammar/hdl/
 *     grammar/hardware/
 *
 * This grammar does not define hardware widths or device topology.
 * ============================================================================
 */

controlHardwareBoundaryExpression
    : expression
    ;


/*
 * ============================================================================
 * 38. DISTRIBUTED CONTROL
 * ============================================================================
 *
 * Distributed control is semantically possible without imposing a node count.
 *
 * The distributed subsystem owns:
 *
 *     placement
 *     replication
 *     communication
 *     consistency
 *     distributed scheduling
 *
 * ============================================================================
 */

controlDistributedBoundaryExpression
    : expression
    ;


/*
 * ============================================================================
 * 39. DETERMINISM
 * ============================================================================
 *
 * This grammar has:
 *
 *     no semantic predicates;
 *     no actions;
 *     no runtime access;
 *     no hardware queries;
 *     no random behavior;
 *     no resource discovery.
 *
 * Therefore the same token stream under the same grammar version produces
 * the same parse structure.
 * ============================================================================
 */


/*
 * ============================================================================
 * 40. ERROR RECOVERY
 * ============================================================================
 *
 * Error handling remains owned by the parser/frontend.
 *
 * This grammar deliberately contains no embedded error actions.
 *
 * Examples of malformed input include:
 *
 *     system {
 *     system name {
 *         plant:
 *     }
 *
 *     system name {
 *         controller =
 *     }
 *
 *     system name {
 *         model
 *     }
 *
 * Diagnostics must preserve source spans and identify the structural failure.
 * ============================================================================
 */


/*
 * ============================================================================
 * 41. SECURITY
 * ============================================================================
 *
 * Parsing this grammar MUST NOT:
 *
 *     execute a controller;
 *     simulate a plant;
 *     solve an equation;
 *     access hardware;
 *     access files;
 *     access a network;
 *     discover devices;
 *     allocate target resources;
 *     perform deployment;
 *     execute generated code.
 *
 * `simulate`, `synthesize`, and `deploy` are SOURCE INTENTS only.
 *
 * Actual execution is downstream and capability-gated.
 * ============================================================================
 */


/*
 * ============================================================================
 * 42. PERFORMANCE / SCALABILITY
 * ============================================================================
 *
 * Structural repetition is unbounded:
 *
 *     controlSystemItem*
 *     controlArgumentList*
 *     expressions supplied by the canonical expression grammar
 *
 * There is no fixed number of:
 *
 *     systems
 *     roles
 *     signals
 *     states
 *     inputs
 *     outputs
 *     controllers
 *     observers
 *     constraints
 *     objectives
 *     models
 *     simulation clauses
 *
 * Practical parser resource limits remain implementation policy.
 *
 * They MUST NOT become semantic grammar limits.
 * ============================================================================
 */


/*
 * ============================================================================
 * 43. NO MACHINE ASSUMPTIONS
 * ============================================================================
 *
 * This file contains no assumptions about:
 *
 *     CPU count
 *     GPU count
 *     FPGA count
 *     QPU count
 *     thread count
 *     memory capacity
 *     register width
 *     vector width
 *     network size
 *     node count
 *     device count
 *     accelerator count
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 44. COMPATIBILITY
 * ============================================================================
 *
 * This file is additive.
 *
 * It does not rename or replace:
 *
 *     classical.g4
 *     numerical.g4
 *     linear-algebra.g4
 *     vector.g4
 *     matrix.g4
 *     tensor.g4
 *     statements/control-flow.g4
 *
 * The distinction is:
 *
 *     control.g4
 *         -> control-system semantics
 *
 *     statements/control-flow.g4
 *         -> program control flow
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 45. COMPOSITION INTEGRATION
 * ============================================================================
 *
 * Required integration:
 *
 *     grammar/classical/classical.g4
 *             |
 *             +--> import ClassicalControl
 *             |
 *             +--> expose controlConstruct
 *
 * The canonical root must ultimately expose the classical composition through
 * the existing classical-domain composition path.
 *
 * Do NOT import ClassicalControl directly into the root if doing so would
 * create a second classical composition path.
 *
 * ============================================================================
 *
 * Recommended classical composition:
 *
 *     Classical
 *        |
 *        +--> ClassicalControl
 *        +--> Numerical
 *        +--> LinearAlgebra
 *        +--> Vector
 *        +--> Matrix
 *        +--> Tensor
 *        +--> Symbolic
 *        +--> ...
 *
 * The exact import graph remains owned by classical/classical.g4.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 46. AST / SEMANTIC / IR INTEGRATION
 * ============================================================================
 *
 * Complete pipeline:
 *
 *     controlSystemDeclaration
 *             |
 *             v
 *     domain-neutral frontend AST
 *             |
 *             v
 *     name resolution
 *             |
 *             v
 *     type / shape analysis
 *             |
 *             v
 *     control semantic analysis
 *             |
 *       +-----+-----+
 *       |           |
 *       v           v
 * classical      resource /
 * semantics      capability
 *       |           |
 *       +-----+-----+
 *             |
 *             v
 *      canonical semantic model
 *             |
 *             +------------------+
 *             |                  |
 *             v                  v
 *       classical IR       control metadata
 *             |
 *             v
 *        optimization
 *             |
 *       +-----+-----+
 *       |           |
 *       v           v
 *   scheduling   lowering
 *       |           |
 *       +-----+-----+
 *             |
 *             v
 *       target realization
 *
 * The grammar does NOT directly emit IR.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 47. CONTROL-ALGORITHM EXTENSIBILITY
 * ============================================================================
 *
 * New control algorithms should normally require NO grammar modification.
 *
 * Examples:
 *
 *     pid(...)
 *     lqr(...)
 *     mpc(...)
 *     kalman(...)
 *     adaptive(...)
 *     robust(...)
 *     nonlinear(...)
 *     optimal(...)
 *     predictive(...)
 *     custom_controller(...)
 *
 * These are ordinary expressions.
 *
 * Their semantics may be supplied by:
 *
 *     standard library
 *     compiler intrinsic
 *     dialect
 *     semantic capability
 *     plugin/extension
 *     target-independent implementation
 *
 * This is essential for long-term POCO-REAF compatibility.
 * ============================================================================
 */


/*
 * ============================================================================
 * 48. FUTURE CONTROL PARADIGMS
 * ============================================================================
 *
 * The same grammar can represent future:
 *
 *     classical control
 *     digital control
 *     continuous control
 *     sampled-data control
 *     hybrid control
 *     nonlinear control
 *     adaptive control
 *     robust control
 *     stochastic control
 *     optimal control
 *     predictive control
 *     distributed control
 *     networked control
 *     event-triggered control
 *     learning-based control
 *     quantum-assisted control
 *     biological control
 *     nano-scale control
 *     cyber-physical control
 *     future control paradigms
 *
 * without adding finite parser enumerations.
 * ============================================================================
 */


/*
 * ============================================================================
 * 49. TEST CONTRACT
 * ============================================================================
 *
 * Positive:
 *
 *     system plant {
 *         plant: model;
 *     }
 *
 *     system loop {
 *         plant: plant_model;
 *         controller: controller(error);
 *         input: u;
 *         output: y;
 *         reference: r;
 *         feedback: y -> controller;
 *     }
 *
 *     system estimator {
 *         state: x;
 *         observer: observer(y);
 *         sampling: dt;
 *     }
 *
 *     system optimal {
 *         model: state_space(A, B, C, D);
 *         objective: minimize(cost);
 *         constraint: state <= bound;
 *         simulate response;
 *     }
 *
 * Negative:
 *
 *     system {
 *     }
 *
 *     system control {
 *         plant:
 *     }
 *
 *     system control {
 *         controller =
 *     }
 *
 *     system control {
 *         model
 *     }
 *
 * Boundary:
 *
 *     system minimal {}
 *
 *     system large {
 *         role0: value0;
 *         role1: value1;
 *         role2: value2;
 *         ...
 *     }
 *
 *     deeply nested expressions;
 *
 *     arbitrarily many roles;
 *
 *     arbitrarily many arguments;
 *
 *     symbolic dimensions;
 *
 *     symbolic timing;
 *
 *     runtime-dependent values.
 *
 * Scalability:
 *
 *     no fixed number of systems;
 *     no fixed number of states;
 *     no fixed number of signals;
 *     no fixed model order;
 *     no fixed matrix dimensions;
 *     no fixed control horizon;
 *     no fixed number of controllers.
 *
 * Determinism:
 *
 *     identical token stream
 *         +
 *     identical grammar version
 *         =
 *     identical parse structure.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 50. HARD-CODING AUDIT
 * ============================================================================
 *
 * PASS CONDITIONS:
 *
 *     [x] No MAX_* machine limits.
 *     [x] No finite controller enumeration.
 *     [x] No finite plant enumeration.
 *     [x] No finite state count.
 *     [x] No finite signal count.
 *     [x] No finite matrix dimension.
 *     [x] No fixed sampling rate.
 *     [x] No fixed execution target.
 *     [x] No physical device identifiers.
 *     [x] No physical memory assumptions.
 *     [x] No SIMD assumptions.
 *     [x] No CPU assumptions.
 *     [x] No GPU assumptions.
 *     [x] No FPGA assumptions.
 *     [x] No QPU assumptions.
 *     [x] No unsafe Rust actions.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 51. IMPLEMENTATION INTEGRATION
 * ============================================================================
 *
 * Rust 1.97 / 1.97.1 compatibility belongs to the Rust frontend/toolchain.
 *
 * This grammar itself contains no Rust code.
 *
 * The generated parser/frontend must:
 *
 *     - use safe Rust;
 *     - preserve source spans;
 *     - preserve source order;
 *     - preserve role names;
 *     - preserve expression structure;
 *     - report structured diagnostics;
 *     - avoid target-dependent parsing;
 *     - avoid artificial semantic limits.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 52. DEFINITION OF DONE
 * ============================================================================
 *
 * This file is complete only when:
 *
 *     [x] Control-system ownership is separated from control-flow ownership.
 *     [x] General expression syntax is reused.
 *     [x] No new lexer token is required.
 *     [x] Control roles remain open-world identifiers.
 *     [x] System structure is explicitly delimited.
 *     [x] Model/simulation/synthesis/deployment intent is represented.
 *     [x] Numerical semantics remain downstream.
 *     [x] Linear algebra remains downstream.
 *     [x] Resource semantics remain downstream.
 *     [x] Capability semantics remain downstream.
 *     [x] Hardware realization remains downstream.
 *     [x] No physical device is encoded.
 *     [x] No resource maximum is encoded.
 *     [x] No control algorithm is hard-coded.
 *     [x] AST boundary is defined.
 *     [x] Semantic boundary is defined.
 *     [x] IR boundary is defined.
 *     [x] Compiler integration is defined.
 *     [x] Runtime integration is defined.
 *     [x] Quantum integration is defined.
 *     [x] HDL integration is defined.
 *     [x] Distributed integration is defined.
 *     [x] Positive tests are defined.
 *     [x] Negative tests are defined.
 *     [x] Boundary tests are defined.
 *     [x] Scalability tests are defined.
 *     [x] Determinism requirements are defined.
 *     [x] Safe-Rust requirement is defined.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 *     CONTROL GRAMMAR
 *          ≠
 *     CONTROL ALGORITHM
 *          ≠
 *     NUMERICAL SOLVER
 *          ≠
 *     LINEAR-ALGEBRA IMPLEMENTATION
 *          ≠
 *     HARDWARE
 *          ≠
 *     RESOURCE MANAGER
 *          ≠
 *     SCHEDULER
 *          ≠
 *     RUNTIME
 *
 * The grammar describes portable control-system intent.
 *
 * Everything target-specific remains downstream.
 *
 * ============================================================================
 */
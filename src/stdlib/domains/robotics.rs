
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Standard Library — Robotics Domain
//!
//! Path: `src/stdlib/domains/robotics.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provides the stable standard-library interface for robotics and
//! robotic-control computation in Zamani.
//!
//! This module defines portable robotics data contracts, validation,
//! provider-facing interfaces, motion-command contracts, trajectory
//! representations and safety preflight checks.
//!
//! Actual hardware access and execution are delegated to robotics
//! providers integrated with the canonical runtime and HAL.
//!
//! ## Architectural authorities
//!
//! Source grammar:
//!     grammar/classical/control.g4
//!
//! Effects:
//!     grammar/effects/
//!
//! Resource requirements:
//!     grammar/resources/
//!
//! Semantic model:
//!     grammar/specification/semantic-model.md
//!
//! Frontend:
//!     crate::frontend
//!
//! Semantic analysis:
//!     crate::frontend::semantic
//!
//! Standard-library errors:
//!     crate::stdlib::error
//!
//! Standard-library capabilities:
//!     crate::stdlib::capability
//!
//! Standard-library operation context:
//!     crate::stdlib::context
//!
//! Standard-library provider contracts:
//!     crate::stdlib::provider
//!
//! Generic device communication:
//!     crate::stdlib::io::devices
//!
//! AI:
//!     crate::ai
//!
//! Vision:
//!     Existing canonical vision subsystem.
//!
//! Runtime:
//!     crate::runtime
//!
//! Hardware abstraction:
//!     Canonical HAL and hardware providers.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Robotics standard-library API identity.
//! - Portable robot identifiers.
//! - Portable joint identifiers.
//! - Robotics model descriptions.
//! - Joint state representations.
//! - Motion command representations.
//! - Trajectory representations.
//! - Geometric value contracts.
//! - Robotics provider extension contracts.
//! - Robotics-specific input validation.
//! - Robotics safety preflight validation.
//!
//! This module does not own:
//!
//! - Robotics source grammar.
//! - AST construction.
//! - Semantic analysis.
//! - AI reasoning.
//! - Computer vision.
//! - Sensor fusion algorithms.
//! - Physical device discovery.
//! - Physical hardware allocation.
//! - Hardware drivers.
//! - Hardware authorization.
//! - Motor-control firmware.
//! - Real-time scheduling.
//! - Motion-planning algorithms.
//! - Collision-detection algorithms.
//! - Inverse-kinematics algorithms.
//! - Forward-kinematics algorithms.
//! - Physical actuation.
//! - Distributed scheduling.
//! - Quantum execution.
//! - Hardware-description IR.
//! - Compiler lowering.
//!
//! Those responsibilities remain with their canonical authorities.
//!
//! ## POCO-REAF
//!
//! Robotics programs describe portable computational intent.
//!
//! Compatible implementations may realize the same program using:
//!
//! - Embedded processors.
//! - Classical processors.
//! - Multicore systems.
//! - Accelerators.
//! - GPUs.
//! - FPGAs.
//! - ASICs.
//! - Distributed robotic systems.
//! - Simulation environments.
//! - Hybrid classical/quantum computation.
//! - Future computational architectures.
//!
//! No universal maximum is imposed on:
//!
//! - Robots.
//! - Joints.
//! - Sensors.
//! - Actuators.
//! - Frames.
//! - Trajectory points.
//! - Providers.
//! - Devices.
//! - Control systems.
//! - Distributed participants.
//! - Computational resources.
//!
//! Actual execution remains bounded by available memory, processing
//! resources, provider capabilities, implementation representation
//! limits and explicitly configured operational budgets.
//!
//! Literal infinite physical execution is not promised.
//!
//! The objective is to avoid arbitrary language-level and
//! standard-library-level scalability ceilings.
//!
//! ## Safety
//!
//! Local command validation is a preflight check, not a substitute
//! for independent hardware safety interlocks.
//!
//! A physical provider MUST independently enforce applicable:
//!
//! - Device safety constraints.
//! - Authorization policies.
//! - Emergency-stop mechanisms.
//! - Mechanical operating limits.
//! - Electrical operating limits.
//! - Thermal limits.
//! - Collision constraints.
//! - Watchdogs.
//! - Real-time deadlines.
//! - Fault handling.
//!
//! The standard library must never claim that a physical operation
//! succeeded merely because a command was constructed or submitted.
//!
//! ## Initialization
//!
//! This module is intentionally stateless.
//!
//! It does not initialize robotics hardware, discover devices,
//! start a runtime, or initialize unrelated standard-library modules.

use core::fmt;

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use crate::stdlib::capability::{
    CapabilityId,
    CapabilityStatus,
};

use crate::stdlib::context::OperationContext;

use crate::stdlib::error::{
    Error,
    ErrorKind,
    Result,
};

use crate::stdlib::provider::{
    Provider,
    ProviderFuture,
    ProviderId,
    ProviderStatus,
};

// ============================================================================
// MODULE IDENTITY
// ============================================================================

/// Stable standard-library robotics component identifier.
pub const COMPONENT: &str = "stdlib.domains.robotics";

/// Robotics standard-library API contract version.
pub const API_VERSION: u16 = 1;

/// Provider capability required for robot discovery.
pub const CAPABILITY_DISCOVERY: &str = "robotics.discovery";

/// Provider capability required for state observation.
pub const CAPABILITY_STATE_READ: &str = "robotics.state.read";

/// Provider capability required for motion commands.
pub const CAPABILITY_MOTION_COMMAND: &str = "robotics.motion.command";

/// Provider capability required for trajectory execution.
pub const CAPABILITY_TRAJECTORY_EXECUTION: &str =
    "robotics.trajectory.execute";

/// Provider capability required for emergency stopping.
pub const CAPABILITY_EMERGENCY_STOP: &str =
    "robotics.emergency_stop";

// ============================================================================
// ERROR CONSTRUCTION
// ============================================================================

fn robotics_error(
    kind: ErrorKind,
    operation: &'static str,
    message: impl Into<String>,
) -> Error {
    Error::new(kind, message)
        .in_component(COMPONENT)
        .during_operation(operation)
}

fn invalid_input(
    operation: &'static str,
    message: impl Into<String>,
) -> Error {
    robotics_error(
        ErrorKind::InvalidInput,
        operation,
        message,
    )
}

fn validate_identifier(
    value: &str,
    kind: &'static str,
    operation: &'static str,
) -> Result<()> {
    if value.is_empty() {
        return Err(invalid_input(
            operation,
            format!("{kind} must not be empty"),
        ));
    }

    if value.trim() != value {
        return Err(invalid_input(
            operation,
            format!("{kind} must not contain surrounding whitespace"),
        ));
    }

    if value.chars().any(char::is_control) {
        return Err(invalid_input(
            operation,
            format!("{kind} must not contain control characters"),
        ));
    }

    Ok(())
}

fn validate_finite(
    value: f64,
    name: &'static str,
    operation: &'static str,
) -> Result<()> {
    if !value.is_finite() {
        return Err(invalid_input(
            operation,
            format!("{name} must be finite"),
        ));
    }

    Ok(())
}

// ============================================================================
// OPEN-WORLD IDENTIFIERS
// ============================================================================

macro_rules! define_identifier {
    ($name:ident, $description:literal) => {
        #[doc = $description]
        #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(String);

        impl $name {
            /// Creates a validated identifier.
            ///
            /// No arbitrary maximum identifier length is imposed.
            ///
            /// # Errors
            ///
            /// Returns a structured error for empty identifiers,
            /// surrounding whitespace or control characters.
            pub fn new(value: impl Into<String>) -> Result<Self> {
                let value = value.into();

                validate_identifier(
                    &value,
                    stringify!($name),
                    "validate_identifier",
                )?;

                Ok(Self(value))
            }

            /// Returns the exact identifier.
            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }

            /// Returns the owned identifier string.
            #[must_use]
            pub fn into_string(self) -> String {
                self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(
                &self,
                formatter: &mut fmt::Formatter<'_>,
            ) -> fmt::Result {
                formatter.write_str(self.as_str())
            }
        }

        impl TryFrom<&str> for $name {
            type Error = Error;

            fn try_from(value: &str) -> Result<Self> {
                Self::new(value)
            }
        }

        impl TryFrom<String> for $name {
            type Error = Error;

            fn try_from(value: String) -> Result<Self> {
                Self::new(value)
            }
        }
    };
}

define_identifier!(
    RobotId,
    "Provider-scoped identity of a robot."
);

define_identifier!(
    JointId,
    "Open-world identity of a robot joint."
);

define_identifier!(
    FrameId,
    "Open-world identity of a coordinate frame."
);

define_identifier!(
    SensorId,
    "Open-world identity of a robotic sensor."
);

define_identifier!(
    ActuatorId,
    "Open-world identity of an actuator."
);

define_identifier!(
    JointKindId,
    "Extensible identity of a joint-motion kind."
);

// ============================================================================
// GEOMETRY
// ============================================================================

/// Three-dimensional vector.
///
/// Components must be finite.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector3 {
    /// X component.
    pub x: f64,

    /// Y component.
    pub y: f64,

    /// Z component.
    pub z: f64,
}

impl Vector3 {
    /// Creates a validated vector.
    pub fn new(
        x: f64,
        y: f64,
        z: f64,
    ) -> Result<Self> {
        validate_finite(x, "x", "vector3")?;
        validate_finite(y, "y", "vector3")?;
        validate_finite(z, "z", "vector3")?;

        Ok(Self { x, y, z })
    }

    /// Returns the squared Euclidean magnitude.
    #[must_use]
    pub fn magnitude_squared(self) -> f64 {
        self.x.mul_add(
            self.x,
            self.y.mul_add(self.y, self.z * self.z),
        )
    }
}

/// Unit quaternion.
///
/// Construction normalizes the supplied quaternion.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Quaternion {
    /// Scalar component.
    pub w: f64,

    /// X component.
    pub x: f64,

    /// Y component.
    pub y: f64,

    /// Z component.
    pub z: f64,
}

impl Quaternion {
    /// Creates a normalized quaternion.
    ///
    /// The zero quaternion and non-finite components are rejected.
    pub fn new(
        w: f64,
        x: f64,
        y: f64,
        z: f64,
    ) -> Result<Self> {
        validate_finite(w, "quaternion.w", "quaternion")?;
        validate_finite(x, "quaternion.x", "quaternion")?;
        validate_finite(y, "quaternion.y", "quaternion")?;
        validate_finite(z, "quaternion.z", "quaternion")?;

        let magnitude = w
            .hypot(x)
            .hypot(y)
            .hypot(z);

        if !magnitude.is_finite() || magnitude == 0.0 {
            return Err(invalid_input(
                "quaternion",
                "quaternion must have a finite non-zero magnitude",
            ));
        }

        Ok(Self {
            w: w / magnitude,
            x: x / magnitude,
            y: y / magnitude,
            z: z / magnitude,
        })
    }

    /// Returns the identity rotation.
    #[must_use]
    pub const fn identity() -> Self {
        Self {
            w: 1.0,
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

/// Position and orientation of a coordinate frame.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pose3 {
    /// Translation.
    pub position: Vector3,

    /// Rotation.
    pub orientation: Quaternion,
}

impl Pose3 {
    /// Creates a pose from validated components.
    #[must_use]
    pub const fn new(
        position: Vector3,
        orientation: Quaternion,
    ) -> Self {
        Self {
            position,
            orientation,
        }
    }
}

/// Identifies a pose relative to a named coordinate frame.
#[derive(Clone, Debug, PartialEq)]
pub struct FramePose {
    /// Coordinate-frame identity.
    pub frame: FrameId,

    /// Pose relative to the declared frame.
    pub pose: Pose3,
}

impl FramePose {
    /// Creates a frame pose.
    #[must_use]
    pub const fn new(
        frame: FrameId,
        pose: Pose3,
    ) -> Self {
        Self { frame, pose }
    }
}

// ============================================================================
// JOINT LIMITS
// ============================================================================

/// Optional operating limits for a joint.
///
/// An absent value means this model does not declare that particular
/// limit. It does not mean the physical hardware has no limit.
///
/// The provider remains responsible for enforcing actual device limits.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct JointLimits {
    /// Minimum position, when declared.
    pub minimum_position: Option<f64>,

    /// Maximum position, when declared.
    pub maximum_position: Option<f64>,

    /// Maximum absolute velocity, when declared.
    pub maximum_velocity: Option<f64>,

    /// Maximum absolute effort, when declared.
    pub maximum_effort: Option<f64>,
}

impl JointLimits {
    /// Constructs and validates joint limits.
    pub fn new(
        minimum_position: Option<f64>,
        maximum_position: Option<f64>,
        maximum_velocity: Option<f64>,
        maximum_effort: Option<f64>,
    ) -> Result<Self> {
        for value in [
            minimum_position,
            maximum_position,
            maximum_velocity,
            maximum_effort,
        ]
        .into_iter()
        .flatten()
        {
            validate_finite(value, "joint limit", "joint_limits")?;
        }

        if let (Some(minimum), Some(maximum)) =
            (minimum_position, maximum_position)
        {
            if minimum > maximum {
                return Err(invalid_input(
                    "joint_limits",
                    "minimum position exceeds maximum position",
                ));
            }
        }

        for (name, value) in [
            ("maximum velocity", maximum_velocity),
            ("maximum effort", maximum_effort),
        ] {
            if value.is_some_and(|value| value < 0.0) {
                return Err(invalid_input(
                    "joint_limits",
                    format!("{name} must not be negative"),
                ));
            }
        }

        Ok(Self {
            minimum_position,
            maximum_position,
            maximum_velocity,
            maximum_effort,
        })
    }

    /// Validates a requested position.
    pub fn validate_position(
        &self,
        position: f64,
    ) -> Result<()> {
        validate_finite(position, "joint position", "validate_position")?;

        if self.minimum_position.is_some_and(|min| position < min) {
            return Err(robotics_error(
                ErrorKind::Infeasible,
                "validate_position",
                "requested position is below the declared minimum",
            ));
        }

        if self.maximum_position.is_some_and(|max| position > max) {
            return Err(robotics_error(
                ErrorKind::Infeasible,
                "validate_position",
                "requested position exceeds the declared maximum",
            ));
        }

        Ok(())
    }

    /// Validates an absolute velocity.
    pub fn validate_velocity(
        &self,
        velocity: f64,
    ) -> Result<()> {
        validate_finite(velocity, "joint velocity", "validate_velocity")?;

        if self.maximum_velocity.is_some_and(|max| velocity.abs() > max) {
            return Err(robotics_error(
                ErrorKind::Infeasible,
                "validate_velocity",
                "requested velocity exceeds the declared limit",
            ));
        }

        Ok(())
    }

    /// Validates an absolute effort.
    pub fn validate_effort(
        &self,
        effort: f64,
    ) -> Result<()> {
        validate_finite(effort, "joint effort", "validate_effort")?;

        if self.maximum_effort.is_some_and(|max| effort.abs() > max) {
            return Err(robotics_error(
                ErrorKind::Infeasible,
                "validate_effort",
                "requested effort exceeds the declared limit",
            ));
        }

        Ok(())
    }
}

// ============================================================================
// ROBOT MODEL
// ============================================================================

/// Describes one joint in a robot model.
#[derive(Clone, Debug, PartialEq)]
pub struct JointSpec {
    id: JointId,
    kind: JointKindId,
    limits: JointLimits,
}

impl JointSpec {
    /// Creates a joint specification.
    #[must_use]
    pub const fn new(
        id: JointId,
        kind: JointKindId,
        limits: JointLimits,
    ) -> Self {
        Self {
            id,
            kind,
            limits,
        }
    }

    /// Returns the joint identity.
    #[must_use]
    pub fn id(&self) -> &JointId {
        &self.id
    }

    /// Returns the joint-motion kind.
    #[must_use]
    pub fn kind(&self) -> &JointKindId {
        &self.kind
    }

    /// Returns declared operating limits.
    #[must_use]
    pub const fn limits(&self) -> JointLimits {
        self.limits
    }
}

/// Portable description of a robot's logical model.
///
/// This model does not represent physical allocation or hardware
/// discovery. Its contents are supplied by the application or provider.
#[derive(Clone, Debug, PartialEq)]
pub struct RobotModel {
    name: String,
    joints: BTreeMap<JointId, JointSpec>,
}

impl RobotModel {
    /// Constructs a robot model.
    ///
    /// Duplicate joint identities are rejected.
    ///
    /// There is no arbitrary maximum joint count.
    pub fn new(
        name: impl Into<String>,
        joints: impl IntoIterator<Item = JointSpec>,
    ) -> Result<Self> {
        let name = name.into();

        if name.trim().is_empty() || name.chars().any(char::is_control) {
            return Err(invalid_input(
                "robot_model",
                "robot model name must be non-empty and contain no control characters",
            ));
        }

        let mut indexed = BTreeMap::new();

        for joint in joints {
            let id = joint.id.clone();

            if indexed.insert(id.clone(), joint).is_some() {
                return Err(robotics_error(
                    ErrorKind::AlreadyExists,
                    "robot_model",
                    format!("duplicate joint identity: {id}"),
                ));
            }
        }

        Ok(Self {
            name,
            joints: indexed,
        })
    }

    /// Returns the model name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the model's joints in deterministic identity order.
    #[must_use]
    pub fn joints(&self) -> &BTreeMap<JointId, JointSpec> {
        &self.joints
    }

    /// Looks up a joint.
    #[must_use]
    pub fn joint(&self, id: &JointId) -> Option<&JointSpec> {
        self.joints.get(id)
    }

    /// Returns the number of declared joints.
    #[must_use]
    pub fn joint_count(&self) -> usize {
        self.joints.len()
    }

    /// Returns whether the model declares a joint.
    #[must_use]
    pub fn contains_joint(&self, id: &JointId) -> bool {
        self.joints.contains_key(id)
    }
}

// ============================================================================
// ROBOT STATE
// ============================================================================

/// Observable joint state.
///
/// All values are keyed by canonical joint identity.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RobotState {
    positions: BTreeMap<JointId, f64>,
    velocities: BTreeMap<JointId, f64>,
    efforts: BTreeMap<JointId, f64>,
}

impl RobotState {
    /// Creates a robot state.
    ///
    /// Values must be finite and refer to joints present in the model.
    ///
    /// Physical readings are not rejected merely because they exceed
    /// a declared soft limit. Such a reading may represent an actual
    /// fault or unsafe physical condition and must remain observable.
    pub fn new(
        model: &RobotModel,
        positions: BTreeMap<JointId, f64>,
        velocities: BTreeMap<JointId, f64>,
        efforts: BTreeMap<JointId, f64>,
    ) -> Result<Self> {
        for (category, values) in [
            ("position", &positions),
            ("velocity", &velocities),
            ("effort", &efforts),
        ] {
            for (joint, value) in values {
                if !model.contains_joint(joint) {
                    return Err(robotics_error(
                        ErrorKind::InvalidData,
                        "robot_state",
                        format!(
                            "{category} references undeclared joint {joint}"
                        ),
                    ));
                }

                validate_finite(
                    *value,
                    "robot state value",
                    "robot_state",
                )?;
            }
        }

        Ok(Self {
            positions,
            velocities,
            efforts,
        })
    }

    /// Returns joint positions.
    #[must_use]
    pub fn positions(&self) -> &BTreeMap<JointId, f64> {
        &self.positions
    }

    /// Returns joint velocities.
    #[must_use]
    pub fn velocities(&self) -> &BTreeMap<JointId, f64> {
        &self.velocities
    }

    /// Returns joint efforts.
    #[must_use]
    pub fn efforts(&self) -> &BTreeMap<JointId, f64> {
        &self.efforts
    }

    /// Returns the position of a joint, if observed.
    #[must_use]
    pub fn position(&self, joint: &JointId) -> Option<f64> {
        self.positions.get(joint).copied()
    }
}

// ============================================================================
// MOTION COMMANDS
// ============================================================================

/// A validated motion-command request.
///
/// Constructing this value does not execute a physical command.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MotionCommand {
    target_positions: BTreeMap<JointId, f64>,
    target_velocities: BTreeMap<JointId, f64>,
    target_efforts: BTreeMap<JointId, f64>,
}

impl MotionCommand {
    /// Creates an empty motion command.
    ///
    /// An empty command is representable but cannot be submitted.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds or replaces a target joint position.
    pub fn set_position(
        &mut self,
        joint: JointId,
        value: f64,
    ) -> Result<()> {
        validate_finite(value, "target position", "set_position")?;
        self.target_positions.insert(joint, value);
        Ok(())
    }

    /// Adds or replaces a target joint velocity.
    pub fn set_velocity(
        &mut self,
        joint: JointId,
        value: f64,
    ) -> Result<()> {
        validate_finite(value, "target velocity", "set_velocity")?;
        self.target_velocities.insert(joint, value);
        Ok(())
    }

    /// Adds or replaces a target joint effort.
    pub fn set_effort(
        &mut self,
        joint: JointId,
        value: f64,
    ) -> Result<()> {
        validate_finite(value, "target effort", "set_effort")?;
        self.target_efforts.insert(joint, value);
        Ok(())
    }

    /// Returns target positions.
    #[must_use]
    pub fn positions(&self) -> &BTreeMap<JointId, f64> {
        &self.target_positions
    }

    /// Returns target velocities.
    #[must_use]
    pub fn velocities(&self) -> &BTreeMap<JointId, f64> {
        &self.target_velocities
    }

    /// Returns target efforts.
    #[must_use]
    pub fn efforts(&self) -> &BTreeMap<JointId, f64> {
        &self.target_efforts
    }

    /// Returns whether the command contains no targets.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.target_positions.is_empty()
            && self.target_velocities.is_empty()
            && self.target_efforts.is_empty()
    }

    /// Validates this command against a robot model.
    ///
    /// This is a local preflight check. It is not a guarantee that
    /// the physical device will accept or safely execute the command.
    pub fn validate_against(
        &self,
        model: &RobotModel,
    ) -> Result<()> {
        if self.is_empty() {
            return Err(invalid_input(
                "validate_command",
                "motion command must contain at least one target",
            ));
        }

        for (joint, position) in &self.target_positions {
            let spec = model.joint(joint).ok_or_else(|| {
                robotics_error(
                    ErrorKind::InvalidInput,
                    "validate_command",
                    format!("unknown joint: {joint}"),
                )
            })?;

            spec.limits.validate_position(*position)?;
        }

        for (joint, velocity) in &self.target_velocities {
            let spec = model.joint(joint).ok_or_else(|| {
                robotics_error(
                    ErrorKind::InvalidInput,
                    "validate_command",
                    format!("unknown joint: {joint}"),
                )
            })?;

            spec.limits.validate_velocity(*velocity)?;
        }

        for (joint, effort) in &self.target_efforts {
            let spec = model.joint(joint).ok_or_else(|| {
                robotics_error(
                    ErrorKind::InvalidInput,
                    "validate_command",
                    format!("unknown joint: {joint}"),
                )
            })?;

            spec.limits.validate_effort(*effort)?;
        }

        Ok(())
    }
}

/// Provider-reported command status.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum CommandStatus {
    /// The provider reports that the command was accepted.
    Accepted,

    /// The provider reports that the command was queued.
    Queued,

    /// The provider reports that the command completed.
    Completed,
}

/// Evidence-backed receipt returned by a robotics provider.
///
/// The receipt does not independently prove that a physical action
/// occurred. Its meaning depends on the provider's operation contract.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommandReceipt {
    command_id: String,
    status: CommandStatus,
}

impl CommandReceipt {
    /// Constructs a provider command receipt.
    pub fn new(
        command_id: impl Into<String>,
        status: CommandStatus,
    ) -> Result<Self> {
        let command_id = command_id.into();

        validate_identifier(
            &command_id,
            "command identity",
            "command_receipt",
        )?;

        Ok(Self {
            command_id,
            status,
        })
    }

    /// Returns the provider-defined command identity.
    #[must_use]
    pub fn command_id(&self) -> &str {
        &self.command_id
    }

    /// Returns the provider-reported status.
    #[must_use]
    pub const fn status(&self) -> CommandStatus {
        self.status
    }
}

// ============================================================================
// TRAJECTORIES
// ============================================================================

/// One time-indexed trajectory sample.
#[derive(Clone, Debug, PartialEq)]
pub struct TrajectoryPoint {
    time_from_start: std::time::Duration,
    state: RobotState,
}

impl TrajectoryPoint {
    /// Constructs a trajectory sample.
    #[must_use]
    pub const fn new(
        time_from_start: std::time::Duration,
        state: RobotState,
    ) -> Self {
        Self {
            time_from_start,
            state,
        }
    }

    /// Returns elapsed time from the trajectory origin.
    #[must_use]
    pub const fn time_from_start(&self) -> std::time::Duration {
        self.time_from_start
    }

    /// Returns the sample state.
    #[must_use]
    pub fn state(&self) -> &RobotState {
        &self.state
    }
}

/// Validated sequence of robot-state samples.
///
/// This structure represents a trajectory. It does not assert that
/// the trajectory is collision-free, dynamically feasible, or safe
/// to execute.
#[derive(Clone, Debug, PartialEq)]
pub struct Trajectory {
    points: Vec<TrajectoryPoint>,
}

impl Trajectory {
    /// Constructs a trajectory with strictly increasing sample times.
    ///
    /// Empty trajectories are rejected.
    ///
    /// There is no arbitrary maximum number of trajectory samples.
    pub fn new(
        model: &RobotModel,
        points: Vec<TrajectoryPoint>,
    ) -> Result<Self> {
        if points.is_empty() {
            return Err(invalid_input(
                "trajectory",
                "trajectory must contain at least one point",
            ));
        }

        let mut previous = None;

        for point in &points {
            if let Some(previous_time) = previous {
                if point.time_from_start <= previous_time {
                    return Err(invalid_input(
                        "trajectory",
                        "trajectory sample times must be strictly increasing",
                    ));
                }
            }

            previous = Some(point.time_from_start);

            RobotState::new(
                model,
                point.state.positions.clone(),
                point.state.velocities.clone(),
                point.state.efforts.clone(),
            )?;
        }

        Ok(Self { points })
    }

    /// Returns trajectory samples.
    #[must_use]
    pub fn points(&self) -> &[TrajectoryPoint] {
        &self.points
    }

    /// Returns the number of samples.
    #[must_use]
    pub fn len(&self) -> usize {
        self.points.len()
    }

    /// Returns whether the trajectory has no samples.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }

    /// Returns the total trajectory duration.
    #[must_use]
    pub fn duration(&self) -> std::time::Duration {
        self.points
            .last()
            .map(TrajectoryPoint::time_from_start)
            .unwrap_or_default()
    }

    /// Validates position, velocity and effort limits for every sample.
    pub fn validate_against(
        &self,
        model: &RobotModel,
    ) -> Result<()> {
        for point in &self.points {
            for (joint, value) in point.state.positions() {
                let spec = model.joint(joint).ok_or_else(|| {
                    robotics_error(
                        ErrorKind::InvalidData,
                        "validate_trajectory",
                        format!("unknown joint: {joint}"),
                    )
                })?;

                spec.limits.validate_position(*value)?;
            }

            for (joint, value) in point.state.velocities() {
                let spec = model.joint(joint).ok_or_else(|| {
                    robotics_error(
                        ErrorKind::InvalidData,
                        "validate_trajectory",
                        format!("unknown joint: {joint}"),
                    )
                })?;

                spec.limits.validate_velocity(*value)?;
            }

            for (joint, value) in point.state.efforts() {
                let spec = model.joint(joint).ok_or_else(|| {
                    robotics_error(
                        ErrorKind::InvalidData,
                        "validate_trajectory",
                        format!("unknown joint: {joint}"),
                    )
                })?;

                spec.limits.validate_effort(*value)?;
            }
        }

        Ok(())
    }
}

// ============================================================================
// ROBOT DESCRIPTORS
// ============================================================================

/// Provider-scoped description of an available robot.
///
/// A descriptor is an observation, not a physical reservation.
#[derive(Clone, Debug, PartialEq)]
pub struct RobotDescriptor {
    provider_id: ProviderId,
    id: RobotId,
    display_name: String,
    model: RobotModel,
    capabilities: BTreeSet<CapabilityId>,
}

impl RobotDescriptor {
    /// Constructs a robot descriptor.
    pub fn new(
        provider_id: ProviderId,
        id: RobotId,
        display_name: impl Into<String>,
        model: RobotModel,
        capabilities: impl IntoIterator<Item = CapabilityId>,
    ) -> Result<Self> {
        let display_name = display_name.into();

        if display_name.trim().is_empty()
            || display_name.chars().any(char::is_control)
        {
            return Err(invalid_input(
                "robot_descriptor",
                "display name must be non-empty and contain no control characters",
            ));
        }

        Ok(Self {
            provider_id,
            id,
            display_name,
            model,
            capabilities: capabilities.into_iter().collect(),
        })
    }

    /// Returns the owning provider.
    #[must_use]
    pub fn provider_id(&self) -> &ProviderId {
        &self.provider_id
    }

    /// Returns the provider-scoped robot identity.
    #[must_use]
    pub fn id(&self) -> &RobotId {
        &self.id
    }

    /// Returns the display name.
    #[must_use]
    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    /// Returns the canonical robot model description.
    #[must_use]
    pub fn model(&self) -> &RobotModel {
        &self.model
    }

    /// Returns declared robot capabilities.
    #[must_use]
    pub fn capabilities(&self) -> &BTreeSet<CapabilityId> {
        &self.capabilities
    }

    /// Checks whether the robot declares a capability.
    #[must_use]
    pub fn declares_capability(
        &self,
        capability: &CapabilityId,
    ) -> bool {
        self.capabilities.contains(capability)
    }
}

// ============================================================================
// EMERGENCY STOP CONTRACT
// ============================================================================

/// Provider-reported emergency-stop outcome.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum StopStatus {
    /// The provider has evidence that the declared stop condition
    /// has been reached.
    ConfirmedStopped,

    /// The stop request was accepted but completion is not established.
    StopRequested,

    /// The provider cannot establish the physical stop condition.
    StateUnknown,
}

/// Result of an emergency-stop request.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StopReceipt {
    status: StopStatus,
    evidence: Option<String>,
}

impl StopReceipt {
    /// Constructs an emergency-stop result.
    pub fn new(
        status: StopStatus,
        evidence: Option<String>,
    ) -> Result<Self> {
        if evidence.as_ref().is_some_and(|value| {
            value.trim().is_empty() || value.chars().any(char::is_control)
        }) {
            return Err(invalid_input(
                "stop_receipt",
                "stop evidence must be non-empty and contain no control characters",
            ));
        }

        Ok(Self { status, evidence })
    }

    /// Returns the provider-reported stop status.
    #[must_use]
    pub const fn status(&self) -> StopStatus {
        self.status
    }

    /// Returns optional descriptive evidence.
    #[must_use]
    pub fn evidence(&self) -> Option<&str> {
        self.evidence.as_deref()
    }

    /// Returns whether a stopped condition has been confirmed.
    #[must_use]
    pub const fn is_confirmed_stopped(&self) -> bool {
        matches!(self.status, StopStatus::ConfirmedStopped)
    }
}

// ============================================================================
// ROBOTICS PROVIDER CONTRACT
// ============================================================================

/// Domain-specific robotics provider.
///
/// This trait extends the repository's existing generic provider
/// contract. It does not replace provider identity, capability
/// observation, lifecycle, authentication or trust infrastructure.
///
/// Implementations perform actual discovery, sensor/state acquisition
/// and physical command submission.
///
/// Every successful result must accurately describe what the provider
/// actually established.
pub trait RoboticsProvider: Provider {
    /// Lists robots currently observable through this provider.
    ///
    /// Discovery does not reserve or allocate a robot.
    fn list_robots<'a>(
        &'a self,
        context: &'a OperationContext,
    ) -> ProviderFuture<'a, Vec<RobotDescriptor>>;

    /// Reads the latest available robot state.
    ///
    /// Implementations must preserve sensor/provider failures.
    fn read_state<'a>(
        &'a self,
        robot: &'a RobotDescriptor,
        context: &'a OperationContext,
    ) -> ProviderFuture<'a, RobotState>;

    /// Submits a motion command.
    ///
    /// Returning a receipt means only what its declared status says.
    /// A provider must not report completion when it merely accepted
    /// or queued a command.
    fn send_motion_command<'a>(
        &'a self,
        robot: &'a RobotDescriptor,
        command: &'a MotionCommand,
        context: &'a OperationContext,
    ) -> ProviderFuture<'a, CommandReceipt>;

    /// Executes a validated trajectory.
    ///
    /// The provider remains responsible for real-time behavior,
    /// collision checks, watchdogs, hardware limits and safe execution.
    fn execute_trajectory<'a>(
        &'a self,
        robot: &'a RobotDescriptor,
        trajectory: &'a Trajectory,
        context: &'a OperationContext,
    ) -> ProviderFuture<'a, CommandReceipt>;

    /// Requests an emergency stop.
    ///
    /// This operation must remain available even if the supplied
    /// operation context has already been cancelled.
    ///
    /// Providers must prioritize applicable physical safety mechanisms.
    fn emergency_stop<'a>(
        &'a self,
        robot: &'a RobotDescriptor,
        reason: &'a str,
        context: &'a OperationContext,
    ) -> ProviderFuture<'a, StopReceipt>;
}

// ============================================================================
// OPTIONAL KINEMATICS PROVIDER
// ============================================================================

/// Optional robotics kinematics implementation.
///
/// A provider may implement this trait in addition to
/// [`RoboticsProvider`].
///
/// This interface deliberately does not provide simulated or dummy
/// kinematics. The actual solver must be supplied by the implementation.
pub trait RoboticsKinematicsProvider: RoboticsProvider {
    /// Computes forward kinematics.
    ///
    /// Returns frame poses established by the solver.
    fn forward_kinematics<'a>(
        &'a self,
        robot: &'a RobotDescriptor,
        state: &'a RobotState,
        context: &'a OperationContext,
    ) -> ProviderFuture<'a, Vec<FramePose>>;

    /// Computes inverse kinematics.
    ///
    /// The seed state is supplied as an initial condition.
    ///
    /// The provider must return an explicit error when it cannot
    /// establish a valid solution.
    fn inverse_kinematics<'a>(
        &'a self,
        robot: &'a RobotDescriptor,
        targets: &'a [FramePose],
        seed: &'a RobotState,
        context: &'a OperationContext,
    ) -> ProviderFuture<'a, RobotState>;
}

// ============================================================================
// ROBOTICS CLIENT
// ============================================================================

/// Stateless client for a robotics provider.
///
/// The client does not select a provider, discover hardware independently,
/// initialize the runtime or allocate physical resources.
///
/// Provider selection and registration belong to their canonical owners.
pub struct RoboticsClient {
    provider: Arc<dyn RoboticsProvider>,
}

impl RoboticsClient {
    /// Creates a client using an already selected provider.
    #[must_use]
    pub fn new(provider: Arc<dyn RoboticsProvider>) -> Self {
        Self { provider }
    }

    /// Returns the selected provider's identity.
    #[must_use]
    pub fn provider_id(&self) -> &ProviderId {
        self.provider.descriptor().id()
    }

    /// Discovers robots through the selected provider.
    ///
    /// Duplicate identities and foreign-provider descriptors are rejected.
    pub async fn discover(
        &self,
        context: &OperationContext,
    ) -> Result<Vec<RobotDescriptor>> {
        context.check_cancelled()?;

        self.require_provider_ready("discover")?;

        self.require_capability(
            CAPABILITY_DISCOVERY,
            None,
            context,
        )
        .await?;

        let mut robots = self.provider.list_robots(context).await?;

        let mut identities = BTreeSet::new();

        for robot in &robots {
            if robot.provider_id() != self.provider_id() {
                return Err(robotics_error(
                    ErrorKind::ContractViolation,
                    "discover",
                    "provider returned a robot owned by another provider",
                ));
            }

            if !identities.insert(robot.id().clone()) {
                return Err(robotics_error(
                    ErrorKind::ContractViolation,
                    "discover",
                    format!(
                        "provider returned duplicate robot identity: {}",
                        robot.id()
                    ),
                ));
            }
        }

        robots.sort_by(|left, right| left.id().cmp(right.id()));

        Ok(robots)
    }

    /// Reads a robot's current state.
    pub async fn read_state(
        &self,
        robot: &RobotDescriptor,
        context: &OperationContext,
    ) -> Result<RobotState> {
        self.validate_robot(robot)?;
        context.check_cancelled()?;

        self.require_provider_ready("read_state")?;

        self.require_capability(
            CAPABILITY_STATE_READ,
            Some(robot),
            context,
        )
        .await?;

        let state = self.provider.read_state(robot, context).await?;

        RobotState::new(
            robot.model(),
            state.positions.clone(),
            state.velocities.clone(),
            state.efforts.clone(),
        )
    }

    /// Submits a motion command after local preflight validation.
    ///
    /// The provider must independently enforce its safety interlocks.
    pub async fn send_motion_command(
        &self,
        robot: &RobotDescriptor,
        command: &MotionCommand,
        context: &OperationContext,
    ) -> Result<CommandReceipt> {
        self.validate_robot(robot)?;
        context.check_cancelled()?;

        self.require_provider_ready("send_motion_command")?;

        command.validate_against(robot.model())?;

        self.require_capability(
            CAPABILITY_MOTION_COMMAND,
            Some(robot),
            context,
        )
        .await?;

        context.check_cancelled()?;

        self.provider
            .send_motion_command(robot, command, context)
            .await
    }

    /// Executes a trajectory after validating all its samples.
    pub async fn execute_trajectory(
        &self,
        robot: &RobotDescriptor,
        trajectory: &Trajectory,
        context: &OperationContext,
    ) -> Result<CommandReceipt> {
        self.validate_robot(robot)?;
        context.check_cancelled()?;

        self.require_provider_ready("execute_trajectory")?;

        trajectory.validate_against(robot.model())?;

        self.require_capability(
            CAPABILITY_TRAJECTORY_EXECUTION,
            Some(robot),
            context,
        )
        .await?;

        context.check_cancelled()?;

        self.provider
            .execute_trajectory(robot, trajectory, context)
            .await
    }

    /// Requests an emergency stop.
    ///
    /// Unlike ordinary motion operations, this method does not reject
    /// an already-cancelled context before calling the provider.
    ///
    /// It is the provider's responsibility to implement the actual
    /// physical emergency-stop mechanism.
    pub async fn emergency_stop(
        &self,
        robot: &RobotDescriptor,
        reason: &str,
        context: &OperationContext,
    ) -> Result<StopReceipt> {
        self.validate_robot(robot)?;

        if reason.trim().is_empty()
            || reason.chars().any(char::is_control)
        {
            return Err(invalid_input(
                "emergency_stop",
                "emergency-stop reason must be non-empty and contain no control characters",
            ));
        }

        self.require_capability(
            CAPABILITY_EMERGENCY_STOP,
            Some(robot),
            context,
        )
        .await?;

        self.provider
            .emergency_stop(robot, reason, context)
            .await
    }

    fn validate_robot(
        &self,
        robot: &RobotDescriptor,
    ) -> Result<()> {
        if robot.provider_id() != self.provider_id() {
            return Err(robotics_error(
                ErrorKind::PermissionDenied,
                "validate_robot",
                "robot descriptor belongs to another provider",
            ));
        }

        Ok(())
    }

    fn require_provider_ready(
        &self,
        operation: &'static str,
    ) -> Result<()> {
        if !self.provider.status().may_accept_operations() {
            return Err(robotics_error(
                ErrorKind::Provider,
                operation,
                format!(
                    "robotics provider is not ready: {}",
                    self.provider.status()
                ),
            ));
        }

        Ok(())
    }

    async fn require_capability(
        &self,
        capability_name: &'static str,
        robot: Option<&RobotDescriptor>,
        context: &OperationContext,
    ) -> Result<()> {
        context.check_cancelled()?;

        let capability = CapabilityId::new(capability_name)
            .map_err(|error| {
                robotics_error(
                    ErrorKind::Internal,
                    "capability_check",
                    format!("invalid built-in robotics capability identity: {error}"),
                )
            })?;

        if let Some(robot) = robot {
            if !robot.declares_capability(&capability) {
                return Err(robotics_error(
                    ErrorKind::CapabilityUnavailable,
                    "capability_check",
                    format!(
                        "robot {} does not declare capability {}",
                        robot.id(),
                        capability_name
                    ),
                ));
            }
        }

        let snapshot = self
            .provider
            .capability_snapshot(context)
            .await?;

        match snapshot.get(&capability) {
            Some(state) if state.status() == CapabilityStatus::Available => {
                Ok(())
            }

            Some(state) => Err(robotics_error(
                ErrorKind::CapabilityUnavailable,
                "capability_check",
                format!(
                    "capability {} is not established as available: {}",
                    capability_name,
                    state.status()
                ),
            )),

            None => Err(robotics_error(
                ErrorKind::CapabilityUnavailable,
                "capability_check",
                format!(
                    "provider supplied no availability evidence for capability {}",
                    capability_name
                ),
            )),
        }
    }
}

// ============================================================================
// OPTIONAL ROBOTICS COMPATIBILITY TYPES
// ============================================================================

/// Portable alias for the previous standard-library actuator command name.
///
/// New implementations should prefer [`MotionCommand`].
pub type ActuatorCommand = MotionCommand;

/// Portable alias for the previous standard-library robot-state name.
///
/// The new representation uses validated joint identities and maps.
pub type RobotStateView = RobotState;

// ============================================================================
// PUBLIC ACCESSORS
// ============================================================================

/// Returns the stable robotics domain name.
#[must_use]
pub const fn domain_name() -> &'static str {
    COMPONENT
}

/// Returns the robotics facade API version.
#[must_use]
pub const fn domain_api_version() -> u16 {
    API_VERSION
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn joint_id(value: &str) -> JointId {
        JointId::new(value).expect("valid joint identity")
    }

    fn model() -> RobotModel {
        let joint = JointSpec::new(
            joint_id("joint.shoulder"),
            JointKindId::new("revolute").expect("valid kind"),
            JointLimits::new(
                Some(-2.0),
                Some(2.0),
                Some(5.0),
                Some(20.0),
            )
            .expect("valid limits"),
        );

        RobotModel::new("example.robot", [joint])
            .expect("valid model")
    }

    #[test]
    fn domain_identity_is_stable() {
        assert_eq!(domain_name(), "stdlib.domains.robotics");
        assert_eq!(domain_api_version(), 1);
    }

    #[test]
    fn identifiers_are_open_world() {
        let id = RobotId::new(
            "future.robotic.platform.specialized_unit",
        )
        .expect("open-world identifier");

        assert_eq!(
            id.as_str(),
            "future.robotic.platform.specialized_unit"
        );
    }

    #[test]
    fn identifiers_do_not_have_an_arbitrary_length_limit() {
        let value = "r".repeat(100_000);

        let id = RobotId::new(value.clone())
            .expect("long identifier");

        assert_eq!(id.as_str(), value);
    }

    #[test]
    fn empty_identifiers_are_rejected() {
        assert!(RobotId::new("").is_err());
        assert!(JointId::new("   ").is_err());
        assert!(FrameId::new("frame\ninvalid").is_err());
    }

    #[test]
    fn vector_rejects_non_finite_components() {
        assert!(Vector3::new(f64::NAN, 0.0, 0.0).is_err());
        assert!(Vector3::new(0.0, f64::INFINITY, 0.0).is_err());
    }

    #[test]
    fn quaternion_is_normalized() {
        let quaternion = Quaternion::new(
            2.0,
            0.0,
            0.0,
            0.0,
        )
        .expect("valid quaternion");

        assert_eq!(quaternion.w, 1.0);
        assert_eq!(quaternion.x, 0.0);
    }

    #[test]
    fn zero_quaternion_is_rejected() {
        assert!(Quaternion::new(0.0, 0.0, 0.0, 0.0).is_err());
    }

    #[test]
    fn invalid_joint_limits_are_rejected() {
        assert!(
            JointLimits::new(
                Some(2.0),
                Some(-2.0),
                None,
                None,
            )
            .is_err()
        );

        assert!(
            JointLimits::new(
                None,
                None,
                Some(-1.0),
                None,
            )
            .is_err()
        );
    }

    #[test]
    fn model_rejects_duplicate_joint_ids() {
        let joint = JointSpec::new(
            joint_id("joint.one"),
            JointKindId::new("revolute").expect("kind"),
            JointLimits::default(),
        );

        assert!(
            RobotModel::new(
                "duplicate",
                [joint.clone(), joint],
            )
            .is_err()
        );
    }

    #[test]
    fn model_has_no_arbitrary_joint_count_limit() {
        let joints = (0..10_000).map(|index| {
            JointSpec::new(
                joint_id(&format!("joint.{index}")),
                JointKindId::new("custom.linear")
                    .expect("valid kind"),
                JointLimits::default(),
            )
        });

        let model = RobotModel::new("large.robot", joints)
            .expect("large model");

        assert_eq!(model.joint_count(), 10_000);
    }

    #[test]
    fn command_accepts_valid_position() {
        let model = model();

        let mut command = MotionCommand::new();

        command
            .set_position(joint_id("joint.shoulder"), 1.0)
            .expect("finite position");

        assert!(command.validate_against(&model).is_ok());
    }

    #[test]
    fn command_rejects_position_outside_limits() {
        let model = model();

        let mut command = MotionCommand::new();

        command
            .set_position(joint_id("joint.shoulder"), 3.0)
            .expect("finite position");

        assert!(command.validate_against(&model).is_err());
    }

    #[test]
    fn command_rejects_unknown_joints() {
        let model = model();

        let mut command = MotionCommand::new();

        command
            .set_position(joint_id("joint.unknown"), 0.0)
            .expect("finite position");

        assert!(command.validate_against(&model).is_err());
    }

    #[test]
    fn command_rejects_non_finite_values() {
        let mut command = MotionCommand::new();

        assert!(
            command
                .set_position(
                    joint_id("joint.shoulder"),
                    f64::NAN,
                )
                .is_err()
        );
    }

    #[test]
    fn state_rejects_unknown_joint_references() {
        let model = model();

        let mut positions = BTreeMap::new();

        positions.insert(
            joint_id("joint.unknown"),
            0.0,
        );

        assert!(
            RobotState::new(
                &model,
                positions,
                BTreeMap::new(),
                BTreeMap::new(),
            )
            .is_err()
        );
    }

    #[test]
    fn trajectory_requires_strictly_increasing_times() {
        let model = model();

        let state = RobotState::default();

        let points = vec![
            TrajectoryPoint::new(
                std::time::Duration::from_millis(100),
                state.clone(),
            ),
            TrajectoryPoint::new(
                std::time::Duration::from_millis(100),
                state,
            ),
        ];

        assert!(Trajectory::new(&model, points).is_err());
    }

    #[test]
    fn trajectory_rejects_empty_input() {
        assert!(
            Trajectory::new(&model(), Vec::new()).is_err()
        );
    }

    #[test]
    fn trajectory_duration_uses_final_sample() {
        let model = model();

        let points = vec![
            TrajectoryPoint::new(
                std::time::Duration::from_millis(10),
                RobotState::default(),
            ),
            TrajectoryPoint::new(
                std::time::Duration::from_millis(100),
                RobotState::default(),
            ),
        ];

        let trajectory = Trajectory::new(&model, points)
            .expect("valid trajectory");

        assert_eq!(
            trajectory.duration(),
            std::time::Duration::from_millis(100)
        );
    }

    #[test]
    fn actual_state_is_not_silently_clamped() {
        let model = model();

        let mut positions = BTreeMap::new();

        positions.insert(
            joint_id("joint.shoulder"),
            3.0,
        );

        let state = RobotState::new(
            &model,
            positions,
            BTreeMap::new(),
            BTreeMap::new(),
        )
        .expect("actual readings must remain observable");

        assert_eq!(
            state.position(&joint_id("joint.shoulder")),
            Some(3.0)
        );
    }

    #[test]
    fn emergency_stop_receipt_distinguishes_request_from_confirmation() {
        let requested = StopReceipt::new(
            StopStatus::StopRequested,
            Some("provider accepted request".to_owned()),
        )
        .expect("valid receipt");

        assert!(!requested.is_confirmed_stopped());

        let confirmed = StopReceipt::new(
            StopStatus::ConfirmedStopped,
            Some("independent stop feedback".to_owned()),
        )
        .expect("valid receipt");

        assert!(confirmed.is_confirmed_stopped());
    }
}

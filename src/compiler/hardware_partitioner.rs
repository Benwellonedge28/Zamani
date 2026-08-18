//! Zamani Compiler — Hardware Partitioning & Latency/Energy Trade-Off Engine
//!
//! This module performs deterministic hardware-backend selection using explicit
//! workload requirements and measurable backend characteristics.
//!
//! The partitioner is intentionally independent of backend code generation.
//! It decides *where* work should execute; the selected backend is responsible
//! for lowering IR to its actual target representation.
//!
//! Design principles:
//! - deterministic decisions
//! - validated numerical inputs
//! - explicit constraints
//! - Pareto analysis
//! - no hidden global state
//! - no fabricated hardware execution
//! - stable API suitable for compiler integration

use std::cmp::Ordering;
use std::fmt;

// -----------------------------------------------------------------------------
// Backend profile
// -----------------------------------------------------------------------------

/// Describes the estimated characteristics of a hardware execution backend.
///
/// Values are estimates supplied by the backend/toolchain configuration.
/// They are not measurements performed by this module.
#[derive(Debug, Clone, PartialEq)]
pub struct BackendProfile {
    /// Stable backend identifier.
    pub name: &'static str,

    /// Estimated latency per operation in nanoseconds.
    pub latency_ns: f64,

    /// Estimated energy per operation in femtojoules.
    pub energy_fj_per_op: f64,

    /// Estimated throughput in giga-operations per second.
    pub throughput_gops: f64,
}

impl BackendProfile {
    pub const fn new(
        name: &'static str,
        latency_ns: f64,
        energy_fj_per_op: f64,
        throughput_gops: f64,
    ) -> Self {
        Self {
            name,
            latency_ns,
            energy_fj_per_op,
            throughput_gops,
        }
    }

    /// Validates that the profile contains physically meaningful finite
    /// non-negative values.
    pub fn validate(&self) -> Result<(), PartitionError> {
        if self.name.trim().is_empty() {
            return Err(PartitionError::InvalidProfile(
                "backend name cannot be empty".to_string(),
            ));
        }

        validate_metric("latency_ns", self.latency_ns)?;
        validate_metric("energy_fj_per_op", self.energy_fj_per_op)?;
        validate_metric("throughput_gops", self.throughput_gops)?;

        Ok(())
    }
}

fn validate_metric(name: &str, value: f64) -> Result<(), PartitionError> {
    if !value.is_finite() {
        return Err(PartitionError::InvalidProfile(format!(
            "{} must be finite",
            name
        )));
    }

    if value < 0.0 {
        return Err(PartitionError::InvalidProfile(format!(
            "{} cannot be negative",
            name
        )));
    }

    Ok(())
}

// -----------------------------------------------------------------------------
// Workload requirements
// -----------------------------------------------------------------------------

/// Explicit requirements used when selecting a backend.
#[derive(Debug, Clone, PartialEq)]
pub struct WorkloadRequirements {
    /// Human-readable workload identifier.
    pub workload_type: String,

    /// Relative importance of latency.
    pub latency_weight: f64,

    /// Relative importance of energy.
    pub energy_weight: f64,

    /// Optional minimum throughput requirement.
    pub minimum_throughput_gops: Option<f64>,

    /// Optional maximum latency requirement.
    pub maximum_latency_ns: Option<f64>,

    /// Optional maximum energy requirement.
    pub maximum_energy_fj_per_op: Option<f64>,
}

impl WorkloadRequirements {
    pub fn new(
        workload_type: impl Into<String>,
        latency_weight: f64,
        energy_weight: f64,
    ) -> Result<Self, PartitionError> {
        let requirements = Self {
            workload_type: workload_type.into(),
            latency_weight,
            energy_weight,
            minimum_throughput_gops: None,
            maximum_latency_ns: None,
            maximum_energy_fj_per_op: None,
        };

        requirements.validate()?;
        Ok(requirements)
    }

    pub fn validate(&self) -> Result<(), PartitionError> {
        if self.workload_type.trim().is_empty() {
            return Err(PartitionError::InvalidWorkload(
                "workload type cannot be empty".to_string(),
            ));
        }

        validate_weight("latency_weight", self.latency_weight)?;
        validate_weight("energy_weight", self.energy_weight)?;

        if self.latency_weight == 0.0 && self.energy_weight == 0.0 {
            return Err(PartitionError::InvalidWorkload(
                "latency and energy weights cannot both be zero".to_string(),
            ));
        }

        if let Some(value) = self.minimum_throughput_gops {
            validate_metric("minimum_throughput_gops", value)?;
        }

        if let Some(value) = self.maximum_latency_ns {
            validate_metric("maximum_latency_ns", value)?;
        }

        if let Some(value) = self.maximum_energy_fj_per_op {
            validate_metric("maximum_energy_fj_per_op", value)?;
        }

        Ok(())
    }

    pub fn with_minimum_throughput(
        mut self,
        throughput_gops: f64,
    ) -> Result<Self, PartitionError> {
        validate_metric("minimum_throughput_gops", throughput_gops)?;
        self.minimum_throughput_gops = Some(throughput_gops);
        Ok(self)
    }

    pub fn with_maximum_latency(
        mut self,
        latency_ns: f64,
    ) -> Result<Self, PartitionError> {
        validate_metric("maximum_latency_ns", latency_ns)?;
        self.maximum_latency_ns = Some(latency_ns);
        Ok(self)
    }

    pub fn with_maximum_energy(
        mut self,
        energy_fj_per_op: f64,
    ) -> Result<Self, PartitionError> {
        validate_metric("maximum_energy_fj_per_op", energy_fj_per_op)?;
        self.maximum_energy_fj_per_op = Some(energy_fj_per_op);
        Ok(self)
    }
}

fn validate_weight(name: &str, value: f64) -> Result<(), PartitionError> {
    if !value.is_finite() {
        return Err(PartitionError::InvalidWorkload(format!(
            "{} must be finite",
            name
        )));
    }

    if value < 0.0 {
        return Err(PartitionError::InvalidWorkload(format!(
            "{} cannot be negative",
            name
        )));
    }

    Ok(())
}

// -----------------------------------------------------------------------------
// Selection result
// -----------------------------------------------------------------------------

/// Result of backend selection.
#[derive(Debug, Clone, PartialEq)]
pub struct BackendSelection {
    pub backend: BackendProfile,
    pub cost: f64,
    pub feasible: bool,
}

impl BackendSelection {
    pub fn backend_name(&self) -> &'static str {
        self.backend.name
    }
}

// -----------------------------------------------------------------------------
// Pareto result
// -----------------------------------------------------------------------------

/// Backend represented on the latency/energy Pareto frontier.
#[derive(Debug, Clone, PartialEq)]
pub struct ParetoBackend {
    pub backend: BackendProfile,
}

impl ParetoBackend {
    pub fn name(&self) -> &'static str {
        self.backend.name
    }
}

// -----------------------------------------------------------------------------
// Errors
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PartitionError {
    NoBackends,
    InvalidProfile(String),
    InvalidWorkload(String),
    NoFeasibleBackend(String),
}

impl fmt::Display for PartitionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoBackends => {
                write!(formatter, "hardware partitioner has no backends")
            }

            Self::InvalidProfile(message) => {
                write!(formatter, "invalid backend profile: {}", message)
            }

            Self::InvalidWorkload(message) => {
                write!(formatter, "invalid workload requirements: {}", message)
            }

            Self::NoFeasibleBackend(message) => {
                write!(formatter, "no feasible backend: {}", message)
            }
        }
    }
}

impl std::error::Error for PartitionError {}

// -----------------------------------------------------------------------------
// Hardware partitioner
// -----------------------------------------------------------------------------

/// Deterministic hardware partitioning engine.
#[derive(Debug, Clone)]
pub struct HardwarePartitioner {
    profiles: Vec<BackendProfile>,
}

impl Default for HardwarePartitioner {
    fn default() -> Self {
        Self::new()
    }
}

impl HardwarePartitioner {
    /// Creates the standard Zamani backend profile set.
    ///
    /// These are compiler estimates, not guarantees about a physical device.
    pub fn new() -> Self {
        Self {
            profiles: vec![
                BackendProfile::new(
                    "Classical CPU (RISC-V)",
                    10.0,
                    150.0,
                    60.0,
                ),
                BackendProfile::new(
                    "Advanced RTL (SystemVerilog)",
                    2.0,
                    42.0,
                    135.0,
                ),
                BackendProfile::new(
                    "Neuromorphic SNN",
                    1.0,
                    5.4,
                    850.0,
                ),
                BackendProfile::new(
                    "Silicon Photonics",
                    0.2,
                    2.1,
                    5000.0,
                ),
                BackendProfile::new(
                    "In-Memory Computing (IMC)",
                    0.1,
                    1.2,
                    10000.0,
                ),
            ],
        }
    }

    /// Creates a partitioner from explicitly supplied backend profiles.
    pub fn from_profiles(
        profiles: Vec<BackendProfile>,
    ) -> Result<Self, PartitionError> {
        if profiles.is_empty() {
            return Err(PartitionError::NoBackends);
        }

        for profile in &profiles {
            profile.validate()?;
        }

        Ok(Self { profiles })
    }

    /// Returns all configured backend profiles.
    pub fn profiles(&self) -> &[BackendProfile] {
        &self.profiles
    }

    /// Adds a backend profile after validating it.
    pub fn add_profile(
        &mut self,
        profile: BackendProfile,
    ) -> Result<(), PartitionError> {
        profile.validate()?;

        self.profiles.push(profile);

        Ok(())
    }

    /// Returns the number of configured backends.
    pub fn backend_count(&self) -> usize {
        self.profiles.len()
    }

    /// Selects the optimal backend according to latency/energy weights.
    ///
    /// Cost:
    ///
    /// `latency_weight * latency + energy_weight * energy`
    ///
    /// Hard workload constraints are applied before cost comparison.
    pub fn select(
        &self,
        requirements: &WorkloadRequirements,
    ) -> Result<BackendSelection, PartitionError> {
        requirements.validate()?;

        if self.profiles.is_empty() {
            return Err(PartitionError::NoBackends);
        }

        let mut best: Option<BackendSelection> = None;

        for profile in &self.profiles {
            profile.validate()?;

            if !self.is_feasible(profile, requirements) {
                continue;
            }

            let cost = self.cost(profile, requirements)?;

            let candidate = BackendSelection {
                backend: profile.clone(),
                cost,
                feasible: true,
            };

            let replace = match &best {
                None => true,

                Some(current) => {
                    match candidate.cost.partial_cmp(&current.cost) {
                        Some(Ordering::Less) => true,
                        Some(Ordering::Equal) => {
                            // Stable deterministic tie-break.
                            candidate.backend.name < current.backend.name
                        }
                        _ => false,
                    }
                }
            };

            if replace {
                best = Some(candidate);
            }
        }

        best.ok_or_else(|| {
            PartitionError::NoFeasibleBackend(format!(
                "no backend satisfies workload '{}'",
                requirements.workload_type
            ))
        })
    }

    /// Compatibility API for existing callers.
    pub fn select_optimal_backend(
        &self,
        workload_type: &str,
        latency_weight: f64,
        energy_weight: f64,
    ) -> Result<BackendProfile, PartitionError> {
        let requirements = WorkloadRequirements::new(
            workload_type,
            latency_weight,
            energy_weight,
        )?;

        Ok(self.select(&requirements)?.backend)
    }

    /// Computes the weighted cost of one backend.
    pub fn cost(
        &self,
        profile: &BackendProfile,
        requirements: &WorkloadRequirements,
    ) -> Result<f64, PartitionError> {
        profile.validate()?;
        requirements.validate()?;

        let cost = requirements.latency_weight * profile.latency_ns
            + requirements.energy_weight * profile.energy_fj_per_op;

        if !cost.is_finite() {
            return Err(PartitionError::InvalidWorkload(
                "weighted backend cost is not finite".to_string(),
            ));
        }

        Ok(cost)
    }

    fn is_feasible(
        &self,
        profile: &BackendProfile,
        requirements: &WorkloadRequirements,
    ) -> bool {
        if let Some(minimum) = requirements.minimum_throughput_gops {
            if profile.throughput_gops < minimum {
                return false;
            }
        }

        if let Some(maximum) = requirements.maximum_latency_ns {
            if profile.latency_ns > maximum {
                return false;
            }
        }

        if let Some(maximum) = requirements.maximum_energy_fj_per_op {
            if profile.energy_fj_per_op > maximum {
                return false;
            }
        }

        true
    }

    /// Computes the latency/energy Pareto frontier.
    ///
    /// Both latency and energy are minimized.
    pub fn pareto_frontier(
        &self,
    ) -> Result<Vec<ParetoBackend>, PartitionError> {
        if self.profiles.is_empty() {
            return Err(PartitionError::NoBackends);
        }

        for profile in &self.profiles {
            profile.validate()?;
        }

        let mut frontier = Vec::new();

        for candidate in &self.profiles {
            let dominated = self.profiles.iter().any(|other| {
                if other.name == candidate.name {
                    return false;
                }

                let no_worse_latency =
                    other.latency_ns <= candidate.latency_ns;

                let no_worse_energy =
                    other.energy_fj_per_op
                        <= candidate.energy_fj_per_op;

                let strictly_better =
                    other.latency_ns < candidate.latency_ns
                        || other.energy_fj_per_op
                            < candidate.energy_fj_per_op;

                no_worse_latency && no_worse_energy && strictly_better
            });

            if !dominated {
                frontier.push(ParetoBackend {
                    backend: candidate.clone(),
                });
            }
        }

        // Stable ordering makes compiler output reproducible.
        frontier.sort_by(|a, b| {
            a.backend
                .latency_ns
                .partial_cmp(&b.backend.latency_ns)
                .unwrap_or(Ordering::Equal)
                .then_with(|| {
                    a.backend
                        .energy_fj_per_op
                        .partial_cmp(&b.backend.energy_fj_per_op)
                        .unwrap_or(Ordering::Equal)
                })
                .then_with(|| a.backend.name.cmp(b.backend.name))
        });

        Ok(frontier)
    }

    /// Compatibility API for existing callers.
    pub fn analyze_pareto_frontier(
        &self,
    ) -> Result<Vec<(&'static str, f64, f64)>, PartitionError> {
        Ok(self
            .pareto_frontier()?
            .into_iter()
            .map(|entry| {
                (
                    entry.backend.name,
                    entry.backend.latency_ns,
                    entry.backend.energy_fj_per_op,
                )
            })
            .collect())
    }
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_partitioner_has_backends() {
        let partitioner = HardwarePartitioner::new();

        assert!(partitioner.backend_count() > 0);
    }

    #[test]
    fn default_profiles_are_valid() {
        let partitioner = HardwarePartitioner::new();

        for profile in partitioner.profiles() {
            assert!(profile.validate().is_ok());
        }
    }

    #[test]
    fn zero_weights_are_rejected() {
        let result = WorkloadRequirements::new("test", 0.0, 0.0);

        assert!(matches!(
            result,
            Err(PartitionError::InvalidWorkload(_))
        ));
    }

    #[test]
    fn negative_weights_are_rejected() {
        let result = WorkloadRequirements::new("test", -1.0, 1.0);

        assert!(result.is_err());
    }

    #[test]
    fn empty_workload_name_is_rejected() {
        let result = WorkloadRequirements::new("", 1.0, 1.0);

        assert!(result.is_err());
    }

    #[test]
    fn latency_only_selection_prefers_lowest_latency() {
        let partitioner = HardwarePartitioner::new();

        let requirements =
            WorkloadRequirements::new("latency", 1.0, 0.0)
                .unwrap();

        let selection = partitioner.select(&requirements).unwrap();

        assert_eq!(
            selection.backend.name,
            "In-Memory Computing (IMC)"
        );
    }

    #[test]
    fn energy_only_selection_prefers_lowest_energy() {
        let partitioner = HardwarePartitioner::new();

        let requirements =
            WorkloadRequirements::new("energy", 0.0, 1.0)
                .unwrap();

        let selection = partitioner.select(&requirements).unwrap();

        assert_eq!(
            selection.backend.name,
            "In-Memory Computing (IMC)"
        );
    }

    #[test]
    fn throughput_constraint_is_respected() {
        let partitioner = HardwarePartitioner::new();

        let requirements = WorkloadRequirements::new(
            "high throughput",
            1.0,
            1.0,
        )
        .unwrap()
        .with_minimum_throughput(9_000.0)
        .unwrap();

        let selection = partitioner.select(&requirements).unwrap();

        assert_eq!(
            selection.backend.name,
            "In-Memory Computing (IMC)"
        );
    }

    #[test]
    fn impossible_constraint_returns_error() {
        let partitioner = HardwarePartitioner::new();

        let requirements = WorkloadRequirements::new(
            "impossible",
            1.0,
            1.0,
        )
        .unwrap()
        .with_minimum_throughput(1_000_000.0)
        .unwrap();

        assert!(matches!(
            partitioner.select(&requirements),
            Err(PartitionError::NoFeasibleBackend(_))
        ));
    }

    #[test]
    fn pareto_frontier_is_non_empty() {
        let partitioner = HardwarePartitioner::new();

        let frontier = partitioner.pareto_frontier().unwrap();

        assert!(!frontier.is_empty());
    }

    #[test]
    fn pareto_frontier_contains_imc() {
        let partitioner = HardwarePartitioner::new();

        let frontier = partitioner.pareto_frontier().unwrap();

        assert!(frontier
            .iter()
            .any(|entry| entry.backend.name
                == "In-Memory Computing (IMC)"));
    }

    #[test]
    fn invalid_profile_is_rejected() {
        let profile = BackendProfile::new(
            "invalid",
            f64::NAN,
            1.0,
            1.0,
        );

        assert!(profile.validate().is_err());
    }

    #[test]
    fn negative_profile_metric_is_rejected() {
        let profile = BackendProfile::new(
            "invalid",
            -1.0,
            1.0,
            1.0,
        );

        assert!(profile.validate().is_err());
    }

    #[test]
    fn custom_profiles_can_be_added() {
        let mut partitioner = HardwarePartitioner::new();

        let initial_count = partitioner.backend_count();

        partitioner
            .add_profile(BackendProfile::new(
                "Custom Accelerator",
                0.05,
                0.5,
                20_000.0,
            ))
            .unwrap();

        assert_eq!(
            partitioner.backend_count(),
            initial_count + 1
        );
    }

    #[test]
    fn compatibility_api_works() {
        let partitioner = HardwarePartitioner::new();

        let result = partitioner.select_optimal_backend(
            "test",
            1.0,
            1.0,
        );

        assert!(result.is_ok());
    }

    #[test]
    fn compatibility_pareto_api_works() {
        let partitioner = HardwarePartitioner::new();

        let result = partitioner.analyze_pareto_frontier();

        assert!(result.is_ok());
        assert!(!result.unwrap().is_empty());
    }

    #[test]
    fn deterministic_selection() {
        let partitioner = HardwarePartitioner::new();

        let requirements =
            WorkloadRequirements::new("deterministic", 2.0, 3.0)
                .unwrap();

        let first = partitioner.select(&requirements).unwrap();
        let second = partitioner.select(&requirements).unwrap();

        assert_eq!(first, second);
    }
}
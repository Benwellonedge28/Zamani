//! Zamani Quantum Algorithms — Variational Quantum Algorithms
//!
//! Production-grade orchestration for variational quantum algorithms (VQAs).
//!
//! This module owns:
//!   - parameterized ansätze
//!   - parameter vectors
//!   - objective evaluation
//!   - classical parameter optimization
//!   - convergence control
//!   - optimization history
//!
//! It deliberately does NOT own:
//!   - quantum gate definitions
//!   - qubit definitions
//!   - circuit storage
//!   - hardware topology
//!   - physical qubit routing
//!   - backend execution
//!
//! Those responsibilities belong to:
//!   quantum::ir
//!   quantum::routing
//!   quantum::hardware
//!
//! The module is backend-independent.

use std::fmt;

/// Result type used by the variational subsystem.
pub type Result<T> = std::result::Result<T, VariationalError>;

/// Errors produced by variational compilation/execution.
#[derive(Debug, Clone, PartialEq)]
pub enum VariationalError {
    EmptyParameters,
    InvalidParameter {
        index: usize,
        value: f64,
    },
    InvalidLearningRate(f64),
    InvalidTolerance(f64),
    InvalidIterationLimit,
    ObjectiveEvaluationFailed(String),
    OptimizationFailed(String),
    NonFiniteObjective(f64),
}

impl fmt::Display for VariationalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyParameters => {
                write!(f, "variational parameter vector cannot be empty")
            }
            Self::InvalidParameter { index, value } => {
                write!(
                    f,
                    "invalid variational parameter at index {}: {}",
                    index, value
                )
            }
            Self::InvalidLearningRate(value) => {
                write!(f, "learning rate must be finite and > 0, got {}", value)
            }
            Self::InvalidTolerance(value) => {
                write!(f, "tolerance must be finite and > 0, got {}", value)
            }
            Self::InvalidIterationLimit => {
                write!(f, "maximum iterations must be greater than zero")
            }
            Self::ObjectiveEvaluationFailed(message) => {
                write!(f, "objective evaluation failed: {}", message)
            }
            Self::OptimizationFailed(message) => {
                write!(f, "variational optimization failed: {}", message)
            }
            Self::NonFiniteObjective(value) => {
                write!(f, "objective returned a non-finite value: {}", value)
            }
        }
    }
}

impl std::error::Error for VariationalError {}

/// Parameter vector used by a variational ansatz.
#[derive(Debug, Clone, PartialEq)]
pub struct Parameters {
    values: Vec<f64>,
}

impl Parameters {
    pub fn new(values: Vec<f64>) -> Result<Self> {
        if values.is_empty() {
            return Err(VariationalError::EmptyParameters);
        }

        for (index, value) in values.iter().copied().enumerate() {
            if !value.is_finite() {
                return Err(VariationalError::InvalidParameter { index, value });
            }
        }

        Ok(Self { values })
    }

    pub fn zeros(count: usize) -> Result<Self> {
        Self::new(vec![0.0; count])
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn as_slice(&self) -> &[f64] {
        &self.values
    }

    pub fn as_mut_slice(&mut self) -> &mut [f64] {
        &mut self.values
    }

    pub fn get(&self, index: usize) -> Option<f64> {
        self.values.get(index).copied()
    }

    pub fn set(&mut self, index: usize, value: f64) -> Result<()> {
        if !value.is_finite() {
            return Err(VariationalError::InvalidParameter { index, value });
        }

        let slot = self
            .values
            .get_mut(index)
            .ok_or_else(|| VariationalError::InvalidParameter { index, value })?;

        *slot = value;
        Ok(())
    }
}

/// Objective function evaluated by the classical optimizer.
///
/// Lower values are assumed to be better.
pub trait Objective {
    fn evaluate(&mut self, parameters: &Parameters) -> Result<f64>;
}

/// Classical optimization algorithm.
pub trait Optimizer {
    fn optimize(
        &mut self,
        objective: &mut dyn Objective,
        initial: Parameters,
        config: &OptimizationConfig,
    ) -> Result<OptimizationResult>;
}

/// Configuration for variational optimization.
#[derive(Debug, Clone, PartialEq)]
pub struct OptimizationConfig {
    pub learning_rate: f64,
    pub tolerance: f64,
    pub max_iterations: usize,
    pub gradient_epsilon: f64,
    pub patience: usize,
}

impl Default for OptimizationConfig {
    fn default() -> Self {
        Self {
            learning_rate: 0.01,
            tolerance: 1e-8,
            max_iterations: 1_000,
            gradient_epsilon: 1e-6,
            patience: 20,
        }
    }
}

impl OptimizationConfig {
    pub fn validate(&self) -> Result<()> {
        if !self.learning_rate.is_finite() || self.learning_rate <= 0.0 {
            return Err(VariationalError::InvalidLearningRate(
                self.learning_rate,
            ));
        }

        if !self.tolerance.is_finite() || self.tolerance <= 0.0 {
            return Err(VariationalError::InvalidTolerance(self.tolerance));
        }

        if self.max_iterations == 0 {
            return Err(VariationalError::InvalidIterationLimit);
        }

        if !self.gradient_epsilon.is_finite()
            || self.gradient_epsilon <= 0.0
        {
            return Err(VariationalError::InvalidTolerance(
                self.gradient_epsilon,
            ));
        }

        Ok(())
    }
}

/// One optimization observation.
#[derive(Debug, Clone, PartialEq)]
pub struct OptimizationStep {
    pub iteration: usize,
    pub objective: f64,
    pub gradient_norm: f64,
}

/// Complete optimization result.
#[derive(Debug, Clone, PartialEq)]
pub struct OptimizationResult {
    pub parameters: Parameters,
    pub objective: f64,
    pub iterations: usize,
    pub converged: bool,
    pub history: Vec<OptimizationStep>,
}

/// Simple deterministic gradient-descent optimizer.
///
/// This is intentionally backend-independent. A production backend can later
/// provide parameter-shift gradients or hardware-native gradient evaluation.
#[derive(Debug, Clone, Copy, Default)]
pub struct GradientDescent;

impl GradientDescent {
    fn numerical_gradient(
        objective: &mut dyn Objective,
        parameters: &Parameters,
        epsilon: f64,
    ) -> Result<Vec<f64>> {
        let mut gradient = vec![0.0; parameters.len()];

        for index in 0..parameters.len() {
            let mut plus = parameters.clone();
            let mut minus = parameters.clone();

            plus.set(
                index,
                parameters.as_slice()[index] + epsilon,
            )?;

            minus.set(
                index,
                parameters.as_slice()[index] - epsilon,
            )?;

            let plus_value = objective.evaluate(&plus)?;
            let minus_value = objective.evaluate(&minus)?;

            if !plus_value.is_finite() {
                return Err(VariationalError::NonFiniteObjective(
                    plus_value,
                ));
            }

            if !minus_value.is_finite() {
                return Err(VariationalError::NonFiniteObjective(
                    minus_value,
                ));
            }

            gradient[index] =
                (plus_value - minus_value) / (2.0 * epsilon);
        }

        Ok(gradient)
    }
}

impl Optimizer for GradientDescent {
    fn optimize(
        &mut self,
        objective: &mut dyn Objective,
        mut parameters: Parameters,
        config: &OptimizationConfig,
    ) -> Result<OptimizationResult> {
        config.validate()?;

        let mut history = Vec::new();
        let mut best_value = f64::INFINITY;
        let mut stagnant_iterations = 0usize;

        for iteration in 0..config.max_iterations {
            let value = objective.evaluate(&parameters)?;

            if !value.is_finite() {
                return Err(VariationalError::NonFiniteObjective(value));
            }

            if best_value - value > config.tolerance {
                best_value = value;
                stagnant_iterations = 0;
            } else {
                stagnant_iterations =
                    stagnant_iterations.saturating_add(1);
            }

            let gradient = Self::numerical_gradient(
                objective,
                &parameters,
                config.gradient_epsilon,
            )?;

            let gradient_norm = gradient
                .iter()
                .map(|v| v * v)
                .sum::<f64>()
                .sqrt();

            history.push(OptimizationStep {
                iteration,
                objective: value,
                gradient_norm,
            });

            if gradient_norm <= config.tolerance
                || stagnant_iterations >= config.patience
            {
                return Ok(OptimizationResult {
                    parameters,
                    objective: value,
                    iterations: iteration + 1,
                    converged: true,
                    history,
                });
            }

            for (parameter, gradient_value) in
                parameters.as_mut_slice().iter_mut().zip(gradient)
            {
                *parameter -= config.learning_rate * gradient_value;

                if !parameter.is_finite() {
                    return Err(VariationalError::OptimizationFailed(
                        "optimizer produced a non-finite parameter"
                            .to_string(),
                    ));
                }
            }
        }

        let objective_value = objective.evaluate(&parameters)?;

        Ok(OptimizationResult {
            parameters,
            objective: objective_value,
            iterations: config.max_iterations,
            converged: false,
            history,
        })
    }
}

/// High-level variational optimizer.
#[derive(Debug, Clone)]
pub struct VariationalOptimizer<O = GradientDescent> {
    optimizer: O,
    config: OptimizationConfig,
}

impl Default for VariationalOptimizer<GradientDescent> {
    fn default() -> Self {
        Self {
            optimizer: GradientDescent,
            config: OptimizationConfig::default(),
        }
    }
}

impl<O: Optimizer> VariationalOptimizer<O> {
    pub fn new(optimizer: O, config: OptimizationConfig) -> Result<Self> {
        config.validate()?;

        Ok(Self { optimizer, config })
    }

    pub fn optimize(
        &mut self,
        objective: &mut dyn Objective,
        initial: Parameters,
    ) -> Result<OptimizationResult> {
        self.optimizer
            .optimize(objective, initial, &self.config)
    }

    pub fn config(&self) -> &OptimizationConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Quadratic;

    impl Objective for Quadratic {
        fn evaluate(&mut self, parameters: &Parameters) -> Result<f64> {
            let x = parameters.as_slice()[0];
            Ok((x - 2.0).powi(2))
        }
    }

    #[test]
    fn parameters_reject_non_finite_values() {
        assert!(Parameters::new(vec![f64::NAN]).is_err());
        assert!(Parameters::new(vec![f64::INFINITY]).is_err());
    }

    #[test]
    fn empty_parameters_are_rejected() {
        assert!(Parameters::new(Vec::new()).is_err());
    }

    #[test]
    fn optimizer_reaches_quadratic_minimum() {
        let parameters = Parameters::new(vec![0.0]).unwrap();

        let config = OptimizationConfig {
            learning_rate: 0.1,
            tolerance: 1e-7,
            max_iterations: 500,
            gradient_epsilon: 1e-6,
            patience: 50,
        };

        let mut optimizer =
            VariationalOptimizer::new(GradientDescent, config).unwrap();

        let mut objective = Quadratic;

        let result = optimizer
            .optimize(&mut objective, parameters)
            .unwrap();

        assert!((result.parameters.as_slice()[0] - 2.0).abs() < 1e-3);
        assert!(result.objective < 1e-5);
    }

    #[test]
    fn invalid_configuration_is_rejected() {
        let config = OptimizationConfig {
            learning_rate: 0.0,
            ..OptimizationConfig::default()
        };

        assert!(VariationalOptimizer::new(GradientDescent, config).is_err());
    }
}
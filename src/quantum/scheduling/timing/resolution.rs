//! Zamani Quantum Scheduling — Timing Resolution
//!
//! Path:
//!     src/quantum/scheduling/timing/resolution.rs
//!
//! # Purpose
//!
//! This module defines the scheduling-layer representation of temporal
//! resolution supplied by an execution target.
//!
//! A `TimingResolution` answers:
//!
//! > What temporal grid, if any, must scheduled events obey on this target?
//!
//! It does NOT define:
//!
//! - quantum operation duration;
//! - a hardware clock implementation;
//! - a pulse/sample generator;
//! - a QPU connection;
//! - routing;
//! - scheduling policy;
//! - scheduling algorithms;
//! - qubit identity;
//! - hardware topology.
//!
//! Those concerns belong to their respective layers.
//!
//! # Architectural position
//!
//! ```text
//! Zamani source
//!       |
//!       v
//! quantum::ir
//!       |
//!       v
//! routing
//!       |
//!       v
//! scheduling
//!       |
//!       +-----------------------------+
//!       |                             |
//!       v                             v
//! semantic timing              target resolution
//!       |                             |
//!       +-------------+---------------+
//!                     |
//!                     v
//!               schedule construction
//!                     |
//!                     v
//!               hardware lowering
//! ```
//!
//! # Important distinction
//!
//! `Duration` and `TimingResolution` are different concepts.
//!
//! ```text
//! Duration
//!     How long an operation semantically takes.
//!
//! TimingResolution
//!     Which temporal positions the target can represent or require.
//! ```
//!
//! The canonical `Duration` type belongs to:
//!
//! ```text
//! crate::quantum::ir::timing::Duration
//! ```
//!
//! This module must not create a second `Duration` type.
//!
//! # Why this belongs in scheduling
//!
//! A quantum program should remain machine-independent.
//!
//! For example:
//!
//! ```text
//! program
//!     |
//!     +--> target A -> resolution A -> schedule A
//!     |
//!     +--> target B -> resolution B -> schedule B
//!     |
//!     +--> target C -> resolution C -> schedule C
//! ```
//!
//! The program therefore does not contain:
//!
//! ```text
//! dt = 0.222 ns
//! alignment = 16
//! channels = 8
//! ```
//!
//! Those values are properties of the selected target.
//!
//! Current hardware-oriented scheduling systems likewise expose alignment
//! constraints such as pulse and acquisition alignment as backend/target
//! properties rather than as universal circuit semantics.
//!
//! # Resolution models
//!
//! This module supports two fundamental models:
//!
//! 1. `Continuous`
//! 2. `Discrete`
//!
//! `Continuous` means that the target does not impose a discrete scheduling
//! grid at this abstraction boundary.
//!
//! `Discrete` represents an exact positive rational number of attoseconds:
//!
//! ```text
//! numerator_attoseconds / denominator
//! ```
//!
//! Rational representation is used because a backend's native sampling period
//! does not necessarily have to be representable as an integer number of
//! canonical attoseconds.
//!
//! Importantly, this does NOT introduce floating-point arithmetic.
//!
//! # Exact arithmetic
//!
//! No `f32` or `f64` is used.
//!
//! All resolution arithmetic is checked.
//!
//! This prevents:
//!
//! - floating-point drift;
//! - platform-dependent rounding;
//! - silent precision loss;
//! - non-deterministic alignment;
//! - overflow wrapping.
//!
//! # Scalability
//!
//! There is deliberately no:
//!
//! ```text
//! MAX_QUBITS
//! MAX_RESOLUTION
//! MAX_CHANNELS
//! MAX_SCHEDULE_DEPTH
//! MAX_OPERATIONS
//! ```
//!
//! in this module.
//!
//! `u128` is the numeric representation boundary for exact finite values.
//! It is not a machine-size limit.
//!
//! Actual execution limits belong to explicit resource, compiler, security,
//! or execution policies.
//!
//! # Qubit independence
//!
//! This module intentionally does NOT import:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! ```
//!
//! A timing resolution applies to temporal coordinates, not qubit identity.
//!
//! Timing-aware structures that contain qubits must use the canonical
//! `quantum::ir::qubit` types in their own modules.
//!
//! # Integration contract
//!
//! This file is consumed by:
//!
//! ```text
//! scheduling::timing::alignment
//! scheduling::timing::constraints
//! scheduling::timing::windows
//! scheduling::planners
//! scheduling::resources
//! scheduling::verification
//! scheduling::adapters::hardware
//! scheduling::context
//! ```
//!
//! Hardware supplies the resolution.
//!
//! The scheduler consumes it.
//!
//! The scheduler must never discover or invent a hardware resolution.
//!
//! # Rust contract
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! `#![forbid(unsafe_code)]` makes the safety requirement compiler-enforced.
//!
//! # Representation invariant
//!
//! For `Discrete`:
//!
//! ```text
//! denominator > 0
//! numerator > 0
//! gcd(numerator, denominator) == 1
//! ```
//!
//! The value is therefore canonicalized at construction.
//!
//! # Scheduling semantics
//!
//! For a discrete resolution `r`, an exactly representable semantic time `t`
//! is aligned when:
//!
//! ```text
//! t / r
//! ```
//!
//! is an integer.
//!
//! When a requested semantic time cannot be represented exactly on the target
//! grid, the scheduler must explicitly choose a rounding policy.
//!
//! This module does not silently round.
//!
//! # Thread safety
//!
//! `TimingResolution` contains only immutable scalar data and is therefore
//! naturally `Send + Sync` under Rust's normal auto-trait rules.
//!
//! # Serialization
//!
//! Serialization must preserve:
//!
//! - resolution kind;
//! - numerator;
//! - denominator.
//!
//! Floating-point serialization is forbidden.
//!
//! # Hashing
//!
//! Hashing is based on the canonical representation, meaning equivalent
//! rational resolutions must hash identically after normalization.
//!
//! # No hidden machine assumptions
//!
//! A target with:
//!
//! - one qubit;
//! - thousands of qubits;
//! - millions of qubits;
//! - multiple QPUs;
//! - distributed quantum nodes;
//! - future quantum architectures
//!
//! may all use this same type.
//!
//! Only the supplied target resolution changes.
//!
//! -----------------------------------------------------------------------------
//! This file owns target timing resolution semantics for scheduling.
//! It does not own quantum duration semantics.
//! -----------------------------------------------------------------------------

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::fmt;
use std::num::NonZeroU128;

use crate::quantum::ir::timing::Duration;

// =============================================================================
// Public error type
// =============================================================================

/// Errors produced while constructing or operating on a timing resolution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TimingResolutionError {
    /// The resolution is not positive.
    ZeroResolution,

    /// An arithmetic operation overflowed.
    ArithmeticOverflow,

    /// A rational value cannot be represented by the requested operation.
    Unrepresentable,

    /// A conversion would require rounding.
    InexactConversion,

    /// A requested rounding operation cannot be represented.
    RoundingOverflow,
}

impl fmt::Display for TimingResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroResolution => {
                formatter.write_str("timing resolution must be greater than zero")
            }

            Self::ArithmeticOverflow => {
                formatter.write_str("timing-resolution arithmetic overflow")
            }

            Self::Unrepresentable => {
                formatter.write_str(
                    "timing value cannot be represented by the requested \
                     resolution",
                )
            }

            Self::InexactConversion => {
                formatter.write_str(
                    "timing conversion would lose precision",
                )
            }

            Self::RoundingOverflow => {
                formatter.write_str(
                    "timing rounding operation overflowed",
                )
            }
        }
    }
}

impl std::error::Error for TimingResolutionError {}

/// Result type used by this module.
pub type TimingResolutionResult<T> =
    Result<T, TimingResolutionError>;

// =============================================================================
// Resolution
// =============================================================================

/// Exact target timing resolution.
///
/// A resolution is either:
///
/// - continuous; or
/// - a positive rational number of attoseconds.
///
/// The rational representation permits target timing models whose native
/// resolution cannot be represented as an integer number of attoseconds.
///
/// # Examples
///
/// ```
/// # use crate::quantum::scheduling::timing::resolution::TimingResolution;
/// # use crate::quantum::ir::timing::Duration;
///
/// let resolution = TimingResolution::attoseconds(1_000);
/// let time = Duration::from_attoseconds(5_000);
///
/// assert!(resolution.is_aligned(time));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TimingResolution {
    /// No discrete temporal grid is imposed at this boundary.
    Continuous,

    /// Exact positive rational resolution in attoseconds.
    ///
    /// The value is:
    ///
    /// `numerator_attoseconds / denominator`
    ///
    /// with a strictly positive denominator and normalized numerator.
    Discrete {
        /// Numerator measured in canonical attoseconds.
        numerator_attoseconds: u128,

        /// Positive denominator.
        denominator: NonZeroU128,
    },
}

impl TimingResolution {
    // =========================================================================
    // Constructors
    // =========================================================================

    /// Creates a continuous timing resolution.
    #[must_use]
    pub const fn continuous() -> Self {
        Self::Continuous
    }

    /// Creates an integer-attosecond discrete resolution.
    ///
    /// # Errors
    ///
    /// Returns `ZeroResolution` when `attoseconds == 0`.
    pub const fn attoseconds(
        attoseconds: u128,
    ) -> TimingResolutionResult<Self> {
        if attoseconds == 0 {
            return Err(TimingResolutionError::ZeroResolution);
        }

        let denominator = match NonZeroU128::new(1) {
            Some(value) => value,
            None => unreachable!(),
        };

        Ok(Self::Discrete {
            numerator_attoseconds: attoseconds,
            denominator,
        })
    }

    /// Creates a rational discrete resolution.
    ///
    /// The supplied value is:
    ///
    /// `numerator_attoseconds / denominator`
    ///
    /// Both values are normalized by their greatest common divisor.
    ///
    /// # Errors
    ///
    /// Returns `ZeroResolution` when either the numerator or denominator is
    /// zero.
    pub fn rational(
        numerator_attoseconds: u128,
        denominator: u128,
    ) -> TimingResolutionResult<Self> {
        if numerator_attoseconds == 0 || denominator == 0 {
            return Err(TimingResolutionError::ZeroResolution);
        }

        let divisor = gcd(
            numerator_attoseconds,
            denominator,
        );

        let numerator = numerator_attoseconds / divisor;
        let denominator_value = denominator / divisor;

        let denominator =
            NonZeroU128::new(denominator_value)
                .ok_or(TimingResolutionError::ZeroResolution)?;

        Ok(Self::Discrete {
            numerator_attoseconds: numerator,
            denominator,
        })
    }

    // =========================================================================
    // Inspection
    // =========================================================================

    /// Returns `true` when the resolution is continuous.
    #[must_use]
    pub const fn is_continuous(self) -> bool {
        matches!(self, Self::Continuous)
    }

    /// Returns `true` when the resolution is discrete.
    #[must_use]
    pub const fn is_discrete(self) -> bool {
        matches!(self, Self::Discrete { .. })
    }

    /// Returns the normalized numerator when the resolution is discrete.
    ///
    /// Returns `None` for continuous resolution.
    #[must_use]
    pub const fn numerator_attoseconds(self) -> Option<u128> {
        match self {
            Self::Continuous => None,
            Self::Discrete {
                numerator_attoseconds,
                ..
            } => Some(numerator_attoseconds),
        }
    }

    /// Returns the denominator when the resolution is discrete.
    ///
    /// Returns `None` for continuous resolution.
    #[must_use]
    pub const fn denominator(self) -> Option<NonZeroU128> {
        match self {
            Self::Continuous => None,
            Self::Discrete { denominator, .. } => Some(denominator),
        }
    }

    /// Returns the resolution as an exact rational pair.
    ///
    /// For continuous resolution this returns `None`.
    #[must_use]
    pub const fn rational_parts(
        self,
    ) -> Option<(u128, NonZeroU128)> {
        match self {
            Self::Continuous => None,
            Self::Discrete {
                numerator_attoseconds,
                denominator,
            } => Some((numerator_attoseconds, denominator)),
        }
    }

    // =========================================================================
    // Alignment
    // =========================================================================

    /// Tests whether a semantic duration lies exactly on the target grid.
    ///
    /// Continuous resolution accepts every duration.
    ///
    /// For a discrete resolution `r`, the duration is aligned when:
    ///
    /// `duration / r`
    ///
    /// is an integer.
    #[must_use]
    pub fn is_aligned(self, duration: Duration) -> bool {
        match self {
            Self::Continuous => true,

            Self::Discrete {
                numerator_attoseconds,
                denominator,
            } => {
                let duration_attoseconds =
                    duration.attoseconds();

                /*
                 * duration / (numerator / denominator)
                 *
                 * = duration * denominator / numerator
                 *
                 * We deliberately avoid multiplication here because
                 * duration * denominator may overflow u128 even though
                 * the final divisibility result could theoretically be
                 * represented.
                 *
                 * Instead, perform exact divisibility using gcd reduction.
                 */
                let gcd_left = gcd(
                    duration_attoseconds,
                    numerator_attoseconds,
                );

                let reduced_duration =
                    duration_attoseconds / gcd_left;

                let reduced_numerator =
                    numerator_attoseconds / gcd_left;

                /*
                 * The condition is:
                 *
                 * reduced_duration * denominator
                 *     % reduced_numerator == 0
                 *
                 * The denominator may share factors with the numerator.
                 * Reduce them first so that the multiplication has the
                 * smallest possible operands.
                 */
                let denominator_value = denominator.get();

                let gcd_right = gcd(
                    denominator_value,
                    reduced_numerator,
                );

                let reduced_denominator =
                    denominator_value / gcd_right;

                let remaining_numerator =
                    reduced_numerator / gcd_right;

                /*
                 * If the remaining numerator is one, alignment is exact.
                 * Otherwise determine whether the reduced duration is
                 * divisible by it before multiplication.
                 */
                if remaining_numerator == 1 {
                    return true;
                }

                if reduced_duration % remaining_numerator == 0 {
                    return true;
                }

                /*
                 * The remaining denominator can contribute factors to the
                 * numerator only through multiplication. We use checked
                 * multiplication to avoid wraparound.
                 */
                match reduced_duration.checked_mul(
                    reduced_denominator,
                ) {
                    Some(product) => {
                        product % remaining_numerator == 0
                    }
                    None => false,
                }
            }
        }
    }

    /// Returns the exact number of resolution units represented by a
    /// duration when the duration is aligned.
    ///
    /// For continuous resolution there is no finite discrete unit count, so
    /// this returns `None`.
    ///
    /// For discrete resolution, the returned value is exact.
    pub fn exact_ticks(
        self,
        duration: Duration,
    ) -> TimingResolutionResult<Option<u128>> {
        match self {
            Self::Continuous => Ok(None),

            Self::Discrete {
                numerator_attoseconds,
                denominator,
            } => {
                let duration_attoseconds =
                    duration.attoseconds();

                /*
                 * duration * denominator / numerator
                 *
                 * Perform cancellation before multiplication so large,
                 * valid values are not rejected merely because an
                 * intermediate multiplication would overflow.
                 */
                let left_gcd = gcd(
                    duration_attoseconds,
                    numerator_attoseconds,
                );

                let reduced_duration =
                    duration_attoseconds / left_gcd;

                let reduced_numerator =
                    numerator_attoseconds / left_gcd;

                let right_gcd = gcd(
                    denominator.get(),
                    reduced_numerator,
                );

                let reduced_denominator =
                    denominator.get() / right_gcd;

                let final_numerator =
                    reduced_numerator / right_gcd;

                if final_numerator == 0 {
                    return Err(
                        TimingResolutionError::ZeroResolution
                    );
                }

                let scaled = reduced_duration
                    .checked_mul(reduced_denominator)
                    .ok_or(
                        TimingResolutionError::ArithmeticOverflow,
                    )?;

                if scaled % final_numerator != 0 {
                    return Err(
                        TimingResolutionError::InexactConversion,
                    );
                }

                Ok(Some(scaled / final_numerator))
            }
        }
    }

    // =========================================================================
    // Quantization
    // =========================================================================

    /// Rounds a duration downward to the nearest representable grid point.
    ///
    /// For continuous resolution the input is returned unchanged.
    pub fn floor(
        self,
        duration: Duration,
    ) -> TimingResolutionResult<Duration> {
        match self {
            Self::Continuous => Ok(duration),

            Self::Discrete {
                numerator_attoseconds,
                denominator,
            } => {
                let ticks = self.floor_ticks(duration)?;

                self.duration_from_ticks(
                    ticks,
                    numerator_attoseconds,
                    denominator.get(),
                )
            }
        }
    }

    /// Rounds a duration upward to the nearest representable grid point.
    ///
    /// The operation never silently wraps.
    pub fn ceil(
        self,
        duration: Duration,
    ) -> TimingResolutionResult<Duration> {
        match self {
            Self::Continuous => Ok(duration),

            Self::Discrete {
                numerator_attoseconds,
                denominator,
            } => {
                let ticks = self.ceil_ticks(duration)?;

                self.duration_from_ticks(
                    ticks,
                    numerator_attoseconds,
                    denominator.get(),
                )
            }
        }
    }

    /// Rounds a duration to the nearest representable grid point.
    ///
    /// Ties are resolved toward the larger grid point.
    pub fn round(
        self,
        duration: Duration,
    ) -> TimingResolutionResult<Duration> {
        match self {
            Self::Continuous => Ok(duration),

            Self::Discrete {
                numerator_attoseconds,
                denominator,
            } => {
                let floor_ticks =
                    self.floor_ticks(duration)?;

                let floor_duration =
                    self.duration_from_ticks(
                        floor_ticks,
                        numerator_attoseconds,
                        denominator.get(),
                    )?;

                if floor_duration == duration {
                    return Ok(duration);
                }

                let ceil_ticks =
                    floor_ticks
                        .checked_add(1)
                        .ok_or(
                            TimingResolutionError::RoundingOverflow,
                        )?;

                let ceil_duration =
                    self.duration_from_ticks(
                        ceil_ticks,
                        numerator_attoseconds,
                        denominator.get(),
                    )?;

                let distance_down =
                    duration
                        .attoseconds()
                        .checked_sub(
                            floor_duration.attoseconds(),
                        )
                        .ok_or(
                            TimingResolutionError::ArithmeticOverflow,
                        )?;

                let distance_up =
                    ceil_duration
                        .attoseconds()
                        .checked_sub(
                            duration.attoseconds(),
                        )
                        .ok_or(
                            TimingResolutionError::ArithmeticOverflow,
                        )?;

                if distance_down < distance_up {
                    Ok(floor_duration)
                } else {
                    Ok(ceil_duration)
                }
            }
        }
    }

    /// Returns the number of complete resolution units at or below a duration.
    pub fn floor_ticks(
        self,
        duration: Duration,
    ) -> TimingResolutionResult<u128> {
        match self {
            Self::Continuous => {
                Err(TimingResolutionError::Unrepresentable)
            }

            Self::Discrete {
                numerator_attoseconds,
                denominator,
            } => {
                /*
                 * floor(
                 *     duration /
                 *     (numerator / denominator)
                 * )
                 *
                 * = floor(
                 *     duration * denominator / numerator
                 *   )
                 */
                let duration_attoseconds =
                    duration.attoseconds();

                let gcd_left = gcd(
                    duration_attoseconds,
                    numerator_attoseconds,
                );

                let reduced_duration =
                    duration_attoseconds / gcd_left;

                let reduced_numerator =
                    numerator_attoseconds / gcd_left;

                let gcd_right = gcd(
                    denominator.get(),
                    reduced_numerator,
                );

                let reduced_denominator =
                    denominator.get() / gcd_right;

                let final_numerator =
                    reduced_numerator / gcd_right;

                let scaled = reduced_duration
                    .checked_mul(reduced_denominator)
                    .ok_or(
                        TimingResolutionError::ArithmeticOverflow,
                    )?;

                Ok(scaled / final_numerator)
            }
        }
    }

    /// Returns the number of resolution units at or above a duration.
    pub fn ceil_ticks(
        self,
        duration: Duration,
    ) -> TimingResolutionResult<u128> {
        match self {
            Self::Continuous => {
                Err(TimingResolutionError::Unrepresentable)
            }

            Self::Discrete {
                numerator_attoseconds,
                denominator,
            } => {
                let floor =
                    self.floor_ticks(duration)?;

                let exact =
                    self.exact_ticks(duration)?;

                if exact.is_some() {
                    return Ok(floor);
                }

                floor
                    .checked_add(1)
                    .ok_or(
                        TimingResolutionError::RoundingOverflow,
                    )
            }
        }
    }

    // =========================================================================
    // Tick conversion
    // =========================================================================

    /// Converts an exact number of target ticks to a semantic duration.
    ///
    /// This succeeds only when the resulting semantic duration can be
    /// represented exactly in canonical attoseconds.
    pub fn duration_from_ticks(
        self,
        ticks: u128,
        numerator_attoseconds: u128,
        denominator: u128,
    ) -> TimingResolutionResult<Duration> {
        if numerator_attoseconds == 0
            || denominator == 0
        {
            return Err(TimingResolutionError::ZeroResolution);
        }

        /*
         * duration =
         *
         * ticks * numerator / denominator
         *
         * Cancel before multiplication to maximize the valid range.
         */
        let gcd_value =
            gcd(ticks, denominator);

        let reduced_ticks =
            ticks / gcd_value;

        let reduced_denominator =
            denominator / gcd_value;

        let gcd_value =
            gcd(
                numerator_attoseconds,
                reduced_denominator,
            );

        let reduced_numerator =
            numerator_attoseconds / gcd_value;

        let final_denominator =
            reduced_denominator / gcd_value;

        if final_denominator == 1 {
            let attoseconds =
                reduced_ticks
                    .checked_mul(reduced_numerator)
                    .ok_or(
                        TimingResolutionError::ArithmeticOverflow,
                    )?;

            return Ok(Duration::from_attoseconds(
                attoseconds,
            ));
        }

        /*
         * The semantic Duration representation is integral attoseconds.
         * If the rational result is not integral, do not silently round.
         */
        let product =
            reduced_ticks
                .checked_mul(reduced_numerator)
                .ok_or(
                    TimingResolutionError::ArithmeticOverflow,
                )?;

        if product % final_denominator != 0 {
            return Err(
                TimingResolutionError::Unrepresentable,
            );
        }

        Ok(Duration::from_attoseconds(
            product / final_denominator,
        ))
    }

    // =========================================================================
    // Internal helpers
    // =========================================================================

    fn floor_ticks_for_discrete(
        numerator_attoseconds: u128,
        denominator: u128,
        duration: Duration,
    ) -> TimingResolutionResult<u128> {
        if numerator_attoseconds == 0
            || denominator == 0
        {
            return Err(TimingResolutionError::ZeroResolution);
        }

        let duration_attoseconds =
            duration.attoseconds();

        let gcd_left = gcd(
            duration_attoseconds,
            numerator_attoseconds,
        );

        let reduced_duration =
            duration_attoseconds / gcd_left;

        let reduced_numerator =
            numerator_attoseconds / gcd_left;

        let gcd_right = gcd(
            denominator,
            reduced_numerator,
        );

        let reduced_denominator =
            denominator / gcd_right;

        let final_numerator =
            reduced_numerator / gcd_right;

        let scaled = reduced_duration
            .checked_mul(reduced_denominator)
            .ok_or(
                TimingResolutionError::ArithmeticOverflow,
            )?;

        Ok(scaled / final_numerator)
    }

    // Kept as a dedicated helper so all construction from target ticks
    // passes through one checked boundary.
    fn duration_from_tick_value(
        numerator_attoseconds: u128,
        denominator: u128,
        ticks: u128,
    ) -> TimingResolutionResult<Duration> {
        if numerator_attoseconds == 0
            || denominator == 0
        {
            return Err(TimingResolutionError::ZeroResolution);
        }

        let common =
            gcd(ticks, denominator);

        let ticks_reduced =
            ticks / common;

        let denominator_reduced =
            denominator / common;

        let common =
            gcd(
                numerator_attoseconds,
                denominator_reduced,
            );

        let numerator_reduced =
            numerator_attoseconds / common;

        let denominator_reduced =
            denominator_reduced / common;

        let product =
            ticks_reduced
                .checked_mul(numerator_reduced)
                .ok_or(
                    TimingResolutionError::ArithmeticOverflow,
                )?;

        if product % denominator_reduced != 0 {
            return Err(
                TimingResolutionError::Unrepresentable,
            );
        }

        Ok(Duration::from_attoseconds(
            product / denominator_reduced,
        ))
    }

    fn floor_ticks_internal(
        self,
        duration: Duration,
    ) -> TimingResolutionResult<u128> {
        match self {
            Self::Continuous => {
                Err(TimingResolutionError::Unrepresentable)
            }

            Self::Discrete {
                numerator_attoseconds,
                denominator,
            } => Self::floor_ticks_for_discrete(
                numerator_attoseconds,
                denominator.get(),
                duration,
            ),
        }
    }

    fn duration_from_ticks_internal(
        self,
        ticks: u128,
    ) -> TimingResolutionResult<Duration> {
        match self {
            Self::Continuous => {
                Err(TimingResolutionError::Unrepresentable)
            }

            Self::Discrete {
                numerator_attoseconds,
                denominator,
            } => Self::duration_from_tick_value(
                numerator_attoseconds,
                denominator.get(),
                ticks,
            ),
        }
    }
}

// =============================================================================
// Internal implementation dispatch
// =============================================================================

impl TimingResolution {
    fn floor_ticks(
        self,
        duration: Duration,
    ) -> TimingResolutionResult<u128> {
        self.floor_ticks_internal(duration)
    }

    fn duration_from_ticks(
        self,
        ticks: u128,
        numerator_attoseconds: u128,
        denominator: u128,
    ) -> TimingResolutionResult<Duration> {
        Self::duration_from_tick_value(
            numerator_attoseconds,
            denominator,
            ticks,
        )
    }
}

// =============================================================================
// Formatting
// =============================================================================

impl fmt::Display for TimingResolution {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Continuous => {
                formatter.write_str("continuous")
            }

            Self::Discrete {
                numerator_attoseconds,
                denominator,
            } => {
                if denominator.get() == 1 {
                    write!(
                        formatter,
                        "{numerator_attoseconds}as"
                    )
                } else {
                    write!(
                        formatter,
                        "{numerator_attoseconds}as/{}",
                        denominator.get()
                    )
                }
            }
        }
    }
}

// =============================================================================
// Greatest common divisor
// =============================================================================

/// Computes the greatest common divisor using an iterative Euclidean
/// algorithm.
///
/// Iteration is used instead of recursion so extremely large integer values
/// cannot consume the call stack.
const fn gcd(
    mut left: u128,
    mut right: u128,
) -> u128 {
    while right != 0 {
        let remainder = left % right;
        left = right;
        right = remainder;
    }

    left
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn duration(attoseconds: u128) -> Duration {
        Duration::from_attoseconds(attoseconds)
    }

    #[test]
    fn continuous_accepts_every_duration() {
        let resolution =
            TimingResolution::continuous();

        assert!(resolution.is_continuous());
        assert!(resolution.is_aligned(
            duration(0)
        ));
        assert!(resolution.is_aligned(
            duration(u128::MAX)
        ));
    }

    #[test]
    fn integer_resolution_is_exact() {
        let resolution =
            TimingResolution::attoseconds(1_000)
                .expect("non-zero resolution");

        assert!(resolution.is_discrete());
        assert_eq!(
            resolution.numerator_attoseconds(),
            Some(1_000)
        );
        assert_eq!(
            resolution
                .denominator()
                .expect("denominator")
                .get(),
            1
        );
    }

    #[test]
    fn aligned_integer_duration_is_detected() {
        let resolution =
            TimingResolution::attoseconds(10)
                .expect("valid resolution");

        assert!(resolution.is_aligned(
            duration(100)
        ));

        assert!(!resolution.is_aligned(
            duration(105)
        ));
    }

    #[test]
    fn exact_ticks_are_calculated() {
        let resolution =
            TimingResolution::attoseconds(10)
                .expect("valid resolution");

        assert_eq!(
            resolution
                .exact_ticks(duration(100))
                .expect("exact")
                .expect("discrete"),
            10
        );
    }

    #[test]
    fn inexact_ticks_are_rejected() {
        let resolution =
            TimingResolution::attoseconds(10)
                .expect("valid resolution");

        assert_eq!(
            resolution.exact_ticks(duration(105)),
            Err(
                TimingResolutionError::InexactConversion
            )
        );
    }

    #[test]
    fn floor_quantizes_down() {
        let resolution =
            TimingResolution::attoseconds(10)
                .expect("valid resolution");

        assert_eq!(
            resolution
                .floor(duration(105))
                .expect("floor")
                .attoseconds(),
            100
        );
    }

    #[test]
    fn ceil_quantizes_up() {
        let resolution =
            TimingResolution::attoseconds(10)
                .expect("valid resolution");

        assert_eq!(
            resolution
                .ceil(duration(101))
                .expect("ceil")
                .attoseconds(),
            110
        );
    }

    #[test]
    fn exact_value_is_not_rounded() {
        let resolution =
            TimingResolution::attoseconds(10)
                .expect("valid resolution");

        assert_eq!(
            resolution
                .round(duration(100))
                .expect("round")
                .attoseconds(),
            100
        );
    }

    #[test]
    fn nearest_rounding_uses_upper_on_tie() {
        let resolution =
            TimingResolution::attoseconds(10)
                .expect("valid resolution");

        assert_eq!(
            resolution
                .round(duration(105))
                .expect("round")
                .attoseconds(),
            110
        );
    }

    #[test]
    fn rational_resolution_is_normalized() {
        let resolution =
            TimingResolution::rational(20, 10)
                .expect("valid resolution");

        assert_eq!(
            resolution.rational_parts(),
            Some((
                2,
                NonZeroU128::new(1)
                    .expect("non-zero")
            ))
        );
    }

    #[test]
    fn rational_resolution_supports_exact_values() {
        /*
         * Resolution:
         *
         * 3 / 2 attoseconds.
         *
         * 3 ticks = 9 / 2 attoseconds, which is not an integer
         * canonical duration.
         *
         * 2 ticks = 3 attoseconds and is exactly representable.
         */
        let resolution =
            TimingResolution::rational(3, 2)
                .expect("valid resolution");

        assert_eq!(
            resolution
                .exact_ticks(duration(3))
                .expect("conversion")
                .expect("discrete"),
            2
        );
    }

    #[test]
    fn rational_resolution_rejects_unrepresentable_tick_duration() {
        let resolution =
            TimingResolution::rational(3, 2)
                .expect("valid resolution");

        assert_eq!(
            resolution.duration_from_ticks(
                1,
                3,
                2,
            ),
            Err(
                TimingResolutionError::Unrepresentable
            )
        );
    }

    #[test]
    fn zero_resolution_is_rejected() {
        assert_eq!(
            TimingResolution::attoseconds(0),
            Err(
                TimingResolutionError::ZeroResolution
            )
        );

        assert_eq!(
            TimingResolution::rational(0, 1),
            Err(
                TimingResolutionError::ZeroResolution
            )
        );

        assert_eq!(
            TimingResolution::rational(1, 0),
            Err(
                TimingResolutionError::ZeroResolution
            )
        );
    }

    #[test]
    fn maximum_duration_does_not_require_fixed_machine_limits() {
        let resolution =
            TimingResolution::attoseconds(1)
                .expect("valid resolution");

        assert_eq!(
            resolution
                .exact_ticks(
                    Duration::MAX
                )
                .expect("exact")
                .expect("discrete"),
            u128::MAX
        );
    }

    #[test]
    fn gcd_handles_large_values_iteratively() {
        assert_eq!(
            gcd(
                u128::MAX - 1,
                u128::MAX - 3
            ),
            2
        );
    }

    #[test]
    fn display_is_deterministic() {
        assert_eq!(
            TimingResolution::Continuous.to_string(),
            "continuous"
        );

        assert_eq!(
            TimingResolution::attoseconds(1_000)
                .expect("valid")
                .to_string(),
            "1000as"
        );

        assert_eq!(
            TimingResolution::rational(3, 2)
                .expect("valid")
                .to_string(),
            "3as/2"
        );
    }
}
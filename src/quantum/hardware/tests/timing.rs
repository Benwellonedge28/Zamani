//! Zamani Quantum Hardware — Timing Integration Tests
//!
//! Production conformance and regression tests for:
//!
//! `crate::quantum::hardware::timing`
//!
//! # Responsibility
//!
//! This file verifies the public, provider-neutral timing contract used by the
//! Zamani quantum hardware abstraction layer.
//!
//! It tests:
//!
//! - exact physical time units;
//! - sub-nanosecond precision;
//! - conversion correctness;
//! - conversion overflow;
//! - explicit zero semantics;
//! - custom-unit validation;
//! - clock-rate validation;
//! - cycle-to-time conversion;
//! - clock-period calculation;
//! - clock-domain isolation;
//! - timestamp ordering;
//! - timing resolution;
//! - timing intervals;
//! - instruction timing;
//! - qubit timing;
//! - synchronization constraints;
//! - timing records;
//! - timing provenance;
//! - complete timing specifications;
//! - deterministic/public API invariants;
//! - regression protection against silent precision loss.
//!
//! # Architectural boundary
//!
//! These tests intentionally depend only on the public timing API.
//!
//! They MUST NOT:
//!
//! - access private timing fields;
//! - depend on provider adapters;
//! - depend on backend implementation details;
//! - depend on calibration implementation details;
//! - depend on scheduling implementation details;
//! - depend on benchmarking;
//! - perform network I/O;
//! - access credentials;
//! - access the system clock;
//! - require physical quantum hardware.
//!
//! This makes the suite independently runnable and suitable for CI.
//!
//! # Integration contract
//!
//! The test module is intended to be included by:
//!
//! `src/quantum/hardware/tests/mod.rs`
//!
//! through:
//!
//! `mod timing;`
//!
//! It tests the already-public contract exposed by:
//!
//! `crate::quantum::hardware::timing`
//!
//! No changes to this file are required when later hardware modules are
//! implemented. If a future module needs a different timing representation,
//! that module must adapt at its own boundary rather than weakening these
//! timing guarantees.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust only.
//!
//! No nightly features are required.
//!
//! # Test philosophy
//!
//! A production hardware timing test must verify semantics rather than merely
//! whether constructors return `Ok`.
//!
//! Therefore this suite deliberately tests:
//!
//! 1. valid values;
//! 2. invalid values;
//! 3. exact conversions;
//! 4. lossy conversions;
//! 5. overflow;
//! 6. boundary values;
//! 7. cross-domain isolation;
//! 8. cross-unit arithmetic;
//! 9. consistency between related APIs;
//! 10. complete aggregate validation.
//!
//! # Determinism
//!
//! No test depends on wall-clock time, randomness, provider availability, or
//! iteration order.
//!
//! Every input is explicit and every expected result is deterministic.
//!
//! # Production acceptance criterion
//!
//! This file is complete when the public timing API satisfies every invariant
//! asserted below. New hardware/provider modules must conform to the timing
//! contract rather than changing these tests merely to accommodate a provider.
//!
//! ----------------------------------------------------------------------------
//! IMPORTANT
//! ----------------------------------------------------------------------------
//!
//! The tests use only APIs currently exposed by `hardware/timing.rs`.
//! They therefore remain independently compilable once this file is registered
//! from `hardware/tests/mod.rs`.
//!
//! ----------------------------------------------------------------------------

use crate::quantum::hardware::timing::{
    ClockDomain,
    ClockRate,
    InstructionTiming,
    QubitTiming,
    SynchronizationConstraint,
    TimeUnit,
    TimeValue,
    Timestamp,
    TimingCategory,
    TimingError,
    TimingInterval,
    TimingProvenance,
    TimingRecord,
    TimingResolution,
    TimingSource,
    TimingSpecification,
    HARDWARE_TIMING_API_VERSION,
    HARDWARE_TIMING_SCHEMA_VERSION,
};

use std::time::Duration;

// =============================================================================
// Test helpers
// =============================================================================

fn ns(value: u64) -> TimeValue {
    TimeValue::new(value, TimeUnit::Nanoseconds)
        .expect("nanosecond value must be valid")
}

fn us(value: u64) -> TimeValue {
    TimeValue::new(value, TimeUnit::Microseconds)
        .expect("microsecond value must be valid")
}

fn ms(value: u64) -> TimeValue {
    TimeValue::new(value, TimeUnit::Milliseconds)
        .expect("millisecond value must be valid")
}

fn ps(value: u64) -> TimeValue {
    TimeValue::new(value, TimeUnit::Picoseconds)
        .expect("picosecond value must be valid")
}

fn fs(value: u64) -> TimeValue {
    TimeValue::new(value, TimeUnit::Femtoseconds)
        .expect("femtosecond value must be valid")
}

fn cycles(value: u64) -> TimeValue {
    TimeValue::new(value, TimeUnit::Cycles)
        .expect("cycle value must be valid")
}

fn device_timestamp(value: u64) -> Timestamp {
    Timestamp::new(
        value,
        TimeUnit::Nanoseconds,
        ClockDomain::DeviceMonotonic,
    )
    .expect("device timestamp must be valid")
}

fn host_timestamp(value: u64) -> Timestamp {
    Timestamp::new(
        value,
        TimeUnit::Nanoseconds,
        ClockDomain::HostMonotonic,
    )
    .expect("host timestamp must be valid")
}

// =============================================================================
// Schema/API stability
// =============================================================================

#[test]
fn schema_version_is_positive() {
    assert!(HARDWARE_TIMING_SCHEMA_VERSION > 0);
}

#[test]
fn api_version_is_positive() {
    assert!(HARDWARE_TIMING_API_VERSION > 0);
}

#[test]
fn schema_and_api_versions_are_explicitly_distinct_contracts() {
    // They currently both start at version 1, but the test documents that
    // both are deliberately exposed independently.
    assert_eq!(HARDWARE_TIMING_SCHEMA_VERSION, 1);
    assert_eq!(HARDWARE_TIMING_API_VERSION, 1);
}

// =============================================================================
// TimeUnit
// =============================================================================

#[test]
fn physical_units_have_stable_symbols() {
    assert_eq!(TimeUnit::Seconds.as_str(), "s");
    assert_eq!(TimeUnit::Milliseconds.as_str(), "ms");
    assert_eq!(TimeUnit::Microseconds.as_str(), "us");
    assert_eq!(TimeUnit::Nanoseconds.as_str(), "ns");
    assert_eq!(TimeUnit::Picoseconds.as_str(), "ps");
    assert_eq!(TimeUnit::Femtoseconds.as_str(), "fs");
    assert_eq!(TimeUnit::Cycles.as_str(), "cycles");
}

#[test]
fn physical_units_are_marked_as_physical_time() {
    assert!(TimeUnit::Seconds.is_physical_time());
    assert!(TimeUnit::Milliseconds.is_physical_time());
    assert!(TimeUnit::Microseconds.is_physical_time());
    assert!(TimeUnit::Nanoseconds.is_physical_time());
    assert!(TimeUnit::Picoseconds.is_physical_time());
    assert!(TimeUnit::Femtoseconds.is_physical_time());
}

#[test]
fn cycles_are_not_physical_time_without_clock_context() {
    assert!(!TimeUnit::Cycles.is_physical_time());
}

#[test]
fn custom_units_are_not_implicitly_physical_time() {
    let unit = TimeUnit::Custom("provider_tick".to_owned());

    assert!(!unit.is_physical_time());
    assert_eq!(unit.as_str(), "provider_tick");
}

#[test]
fn custom_unit_requires_non_empty_identifier() {
    let result = TimeValue::new(
        1,
        TimeUnit::Custom(String::new()),
    );

    assert!(matches!(
        result,
        Err(TimingError::EmptyCustomUnit)
    ));
}

#[test]
fn whitespace_only_custom_unit_is_rejected() {
    let result = TimeValue::new(
        1,
        TimeUnit::Custom("   ".to_owned()),
    );

    assert!(matches!(
        result,
        Err(TimingError::EmptyCustomUnit)
    ));
}

#[test]
fn valid_custom_unit_is_preserved() {
    let value = TimeValue::new(
        7,
        TimeUnit::Custom("provider_tick".to_owned()),
    )
    .expect("custom unit should be accepted");

    assert_eq!(value.value(), 7);
    assert_eq!(
        value.unit(),
        &TimeUnit::Custom("provider_tick".to_owned())
    );
}

// =============================================================================
// TimeValue construction
// =============================================================================

#[test]
fn time_value_preserves_integer_value() {
    let value = ns(123);

    assert_eq!(value.value(), 123);
    assert_eq!(value.unit(), &TimeUnit::Nanoseconds);
}

#[test]
fn explicit_zero_is_valid() {
    let value =
        TimeValue::zero(TimeUnit::Nanoseconds)
            .expect("zero should be representable");

    assert!(value.is_zero());
    assert_eq!(value.value(), 0);
}

#[test]
fn zero_is_distinct_from_missing_optional_timing() {
    let explicit_zero =
        TimeValue::zero(TimeUnit::Nanoseconds)
            .expect("zero should be valid");

    let missing: Option<TimeValue> = None;

    assert!(explicit_zero.is_zero());
    assert!(missing.is_none());
}

#[test]
fn time_value_is_orderable() {
    assert!(ns(10) > ns(5));
    assert!(ps(10) < ns(1));
}

#[test]
fn equal_values_with_different_units_are_not_structurally_equal() {
    let one_ns = ns(1);
    let one_thousand_ps = ps(1_000);

    assert_ne!(one_ns, one_thousand_ps);
}

// =============================================================================
// Exact conversions
// =============================================================================

#[test]
fn one_second_is_exactly_one_quadrillion_femtoseconds() {
    let value = TimeValue::new(1, TimeUnit::Seconds)
        .expect("valid second");

    assert_eq!(
        value
            .to_femtoseconds()
            .expect("exact conversion"),
        1_000_000_000_000_000
    );
}

#[test]
fn one_millisecond_is_exactly_one_trillion_femtoseconds() {
    let value = ms(1);

    assert_eq!(
        value
            .to_femtoseconds()
            .expect("exact conversion"),
        1_000_000_000_000
    );
}

#[test]
fn one_microsecond_is_exactly_one_billion_femtoseconds() {
    let value = us(1);

    assert_eq!(
        value
            .to_femtoseconds()
            .expect("exact conversion"),
        1_000_000_000
    );
}

#[test]
fn one_nanosecond_is_exactly_one_million_femtoseconds() {
    let value = ns(1);

    assert_eq!(
        value
            .to_femtoseconds()
            .expect("exact conversion"),
        1_000_000
    );
}

#[test]
fn one_picosecond_is_exactly_one_thousand_femtoseconds() {
    let value = ps(1);

    assert_eq!(
        value
            .to_femtoseconds()
            .expect("exact conversion"),
        1_000
    );
}

#[test]
fn one_femtosecond_is_exactly_one_femtosecond() {
    let value = fs(1);

    assert_eq!(
        value
            .to_femtoseconds()
            .expect("exact conversion"),
        1
    );
}

#[test]
fn nanoseconds_convert_to_duration_without_loss() {
    let value = ns(42);

    assert_eq!(
        value
            .to_duration()
            .expect("nanoseconds are exactly representable"),
        Duration::from_nanos(42)
    );
}

#[test]
fn microseconds_convert_to_duration_without_loss() {
    let value = us(42);

    assert_eq!(
        value
            .to_duration()
            .expect("microseconds are exactly representable"),
        Duration::from_micros(42)
    );
}

#[test]
fn milliseconds_convert_to_duration_without_loss() {
    let value = ms(42);

    assert_eq!(
        value
            .to_duration()
            .expect("milliseconds are exactly representable"),
        Duration::from_millis(42)
    );
}

#[test]
fn picoseconds_do_not_silently_truncate_to_zero() {
    let value = ps(1);

    assert!(matches!(
        value.to_duration(),
        Err(TimingError::LossyDurationConversion { .. })
    ));
}

#[test]
fn femtoseconds_do_not_silently_truncate_to_zero() {
    let value = fs(999_999);

    assert!(matches!(
        value.to_duration(),
        Err(TimingError::LossyDurationConversion { .. })
    ));
}

#[test]
fn exact_nanosecond_boundary_from_femtoseconds_is_accepted() {
    let value = fs(1_000_000);

    assert_eq!(
        value
            .to_duration()
            .expect("exact nanosecond"),
        Duration::from_nanos(1)
    );
}

#[test]
fn exact_nanoseconds_are_recovered_from_femtoseconds() {
    let value = fs(123_000_000);

    assert_eq!(
        value
            .to_nanoseconds()
            .expect("exact nanoseconds"),
        123
    );
}

#[test]
fn non_integral_nanoseconds_are_rejected() {
    let value = fs(1_000_001);

    assert!(matches!(
        value.to_nanoseconds(),
        Err(TimingError::ConversionOverflow { .. })
    ));
}

#[test]
fn physical_unit_conversion_preserves_exactness() {
    let value = us(5);

    let converted = value
        .convert_to(TimeUnit::Nanoseconds)
        .expect("exact conversion");

    assert_eq!(converted.value(), 5_000);
    assert_eq!(converted.unit(), &TimeUnit::Nanoseconds);
}

#[test]
fn physical_conversion_can_reduce_unit_exactly() {
    let value = ns(5_000);

    let converted = value
        .convert_to(TimeUnit::Microseconds)
        .expect("exact conversion");

    assert_eq!(converted.value(), 5);
    assert_eq!(converted.unit(), &TimeUnit::Microseconds);
}

#[test]
fn non_integral_unit_conversion_is_rejected() {
    let value = ps(1);

    let result = value.convert_to(TimeUnit::Nanoseconds);

    assert!(matches!(
        result,
        Err(TimingError::ConversionOverflow { .. })
    ));
}

#[test]
fn conversion_to_same_unit_is_exact_identity() {
    let value = ns(123);

    let converted = value
        .convert_to(TimeUnit::Nanoseconds)
        .expect("identity conversion");

    assert_eq!(converted, value);
}

// =============================================================================
// Overflow
// =============================================================================

#[test]
fn femtosecond_conversion_detects_multiplication_overflow() {
    let value = TimeValue::new(
        u64::MAX,
        TimeUnit::Seconds,
    )
    .expect("construction itself does not overflow");

    assert!(matches!(
        value.to_femtoseconds(),
        Err(TimingError::ConversionOverflow { .. })
    ));
}

#[test]
fn conversion_overflow_does_not_wrap() {
    let value = TimeValue::new(
        u64::MAX,
        TimeUnit::Seconds,
    )
    .expect("valid source representation");

    let result = value.to_femtoseconds();

    assert!(result.is_err());
}

#[test]
fn nanosecond_conversion_boundary_is_checked() {
    let value = ns(u64::MAX);

    let result = value.to_nanoseconds();

    assert_eq!(
        result.expect("nanoseconds should remain representable"),
        u64::MAX
    );
}

// =============================================================================
// ClockRate
// =============================================================================

#[test]
fn zero_clock_rate_is_rejected() {
    let result = ClockRate::from_hz(0);

    assert!(matches!(
        result,
        Err(TimingError::InvalidClockRate { .. })
    ));
}

#[test]
fn positive_clock_rate_is_accepted() {
    let rate =
        ClockRate::from_hz(1_000_000_000)
            .expect("1 GHz is valid");

    assert_eq!(rate.as_hz(), Some(1_000_000_000));
}

#[test]
fn millihertz_clock_rate_is_preserved() {
    let rate =
        ClockRate::from_millihz(1_000)
            .expect("1 Hz is valid");

    assert_eq!(rate.as_millihz(), 1_000);
    assert_eq!(rate.as_hz(), Some(1));
}

#[test]
fn fractional_hertz_representation_is_preserved_in_millihertz() {
    let rate =
        ClockRate::from_millihz(1_500)
            .expect("1.5 Hz is representable");

    assert_eq!(rate.as_millihz(), 1_500);
    assert_eq!(rate.as_hz(), None);
}

#[test]
fn clock_rate_hz_multiplication_overflow_is_rejected() {
    let result = ClockRate::from_hz(u64::MAX);

    assert!(matches!(
        result,
        Err(TimingError::InvalidClockRate { .. })
    ));
}

#[test]
fn one_ghz_clock_has_one_nanosecond_period() {
    let rate =
        ClockRate::from_hz(1_000_000_000)
            .expect("1 GHz is valid");

    let period = rate
        .period_femtoseconds()
        .expect("period should be exactly representable");

    assert_eq!(period.value(), 1_000_000);
    assert_eq!(period.unit(), &TimeUnit::Femtoseconds);
}

#[test]
fn five_ghz_clock_has_two_hundred_picosecond_period() {
    let rate =
        ClockRate::from_hz(5_000_000_000)
            .expect("5 GHz is valid");

    let period = rate
        .period_femtoseconds()
        .expect("period should be exactly representable");

    assert_eq!(period.value(), 200_000);
    assert_eq!(period.unit(), &TimeUnit::Femtoseconds);
}

// =============================================================================
// Cycles
// =============================================================================

#[test]
fn cycles_cannot_be_converted_without_clock_rate() {
    let value = cycles(10);

    assert!(matches!(
        value.to_femtoseconds(),
        Err(TimingError::InvalidValue { .. })
    ));
}

#[test]
fn cycles_require_the_cycles_specific_conversion_api() {
    let value = ns(10);

    let result = value.cycles_to_femtoseconds(
        ClockRate::from_hz(1_000_000_000)
            .expect("valid clock"),
    );

    assert!(matches!(
        result,
        Err(TimingError::InvalidValue { .. })
    ));
}

#[test]
fn one_cycle_at_one_ghz_is_one_nanosecond() {
    let rate =
        ClockRate::from_hz(1_000_000_000)
            .expect("valid clock");

    let value = cycles(1);

    assert_eq!(
        value
            .cycles_to_femtoseconds(rate)
            .expect("exact cycle conversion"),
        1_000_000
    );
}

#[test]
fn ten_cycles_at_one_ghz_are_ten_nanoseconds() {
    let rate =
        ClockRate::from_hz(1_000_000_000)
            .expect("valid clock");

    let value = cycles(10);

    assert_eq!(
        value
            .cycles_to_femtoseconds(rate)
            .expect("exact cycle conversion"),
        10_000_000
    );
}

#[test]
fn cycle_conversion_rejects_non_integral_femtosecond_result() {
    let rate =
        ClockRate::from_hz(3_000_000_000)
            .expect("valid clock");

    let value = cycles(1);

    assert!(matches!(
        value.cycles_to_femtoseconds(rate),
        Err(TimingError::ConversionOverflow { .. })
    ));
}

#[test]
fn cycle_conversion_is_exact_when_rate_and_cycle_count_permit_it() {
    let rate =
        ClockRate::from_hz(250_000_000)
            .expect("valid clock");

    let value = cycles(4);

    assert_eq!(
        value
            .cycles_to_femtoseconds(rate)
            .expect("exact conversion"),
        16_000_000
    );
}

// =============================================================================
// Clock domains
// =============================================================================

#[test]
fn standard_clock_domains_have_stable_identifiers() {
    assert_eq!(
        ClockDomain::HostMonotonic.as_str(),
        "host_monotonic"
    );
    assert_eq!(
        ClockDomain::HostWallClock.as_str(),
        "host_wall_clock"
    );
    assert_eq!(
        ClockDomain::DeviceMonotonic.as_str(),
        "device_monotonic"
    );
    assert_eq!(
        ClockDomain::ProviderMonotonic.as_str(),
        "provider_monotonic"
    );
    assert_eq!(
        ClockDomain::Simulation.as_str(),
        "simulation"
    );
    assert_eq!(
        ClockDomain::External.as_str(),
        "external"
    );
    assert_eq!(
        ClockDomain::Unknown.as_str(),
        "unknown"
    );
}

#[test]
fn custom_clock_domain_requires_non_empty_identifier() {
    let result = Timestamp::new(
        1,
        TimeUnit::Nanoseconds,
        ClockDomain::Custom(String::new()),
    );

    assert!(matches!(
        result,
        Err(TimingError::EmptyCustomClock)
    ));
}

#[test]
fn whitespace_only_custom_clock_domain_is_rejected() {
    let result = Timestamp::new(
        1,
        TimeUnit::Nanoseconds,
        ClockDomain::Custom("   ".to_owned()),
    );

    assert!(matches!(
        result,
        Err(TimingError::EmptyCustomClock)
    ));
}

#[test]
fn valid_custom_clock_domain_is_preserved() {
    let clock =
        ClockDomain::Custom("device_a_clock".to_owned());

    let timestamp = Timestamp::new(
        10,
        TimeUnit::Nanoseconds,
        clock.clone(),
    )
    .expect("valid custom clock");

    assert_eq!(timestamp.clock_domain(), &clock);
}

#[test]
fn monotonic_clock_domains_support_elapsed_difference() {
    assert!(ClockDomain::HostMonotonic.supports_elapsed_difference());
    assert!(ClockDomain::DeviceMonotonic.supports_elapsed_difference());
    assert!(ClockDomain::ProviderMonotonic.supports_elapsed_difference());
    assert!(ClockDomain::Simulation.supports_elapsed_difference());
    assert!(ClockDomain::External.supports_elapsed_difference());
}

#[test]
fn_wall_clock_domain_is_not_treated_as_intrinsically_elapsed_safe() {
    assert!(!ClockDomain::HostWallClock.supports_elapsed_difference());
}

#[test]
fn unknown_clock_domain_is_not_treated_as_elapsed_safe() {
    assert!(!ClockDomain::Unknown.supports_elapsed_difference());
}

// =============================================================================
// Timestamp
// =============================================================================

#[test]
fn cycle_timestamp_is_rejected_without_explicit_clock_rate_model() {
    let result = Timestamp::new(
        10,
        TimeUnit::Cycles,
        ClockDomain::DeviceMonotonic,
    );

    assert!(matches!(
        result,
        Err(TimingError::InvalidValue { .. })
    ));
}

#[test]
fn timestamp_preserves_value_unit_and_clock_domain() {
    let timestamp = Timestamp::new(
        123,
        TimeUnit::Nanoseconds,
        ClockDomain::DeviceMonotonic,
    )
    .expect("valid timestamp");

    assert_eq!(timestamp.value(), 123);
    assert_eq!(
        timestamp.unit(),
        &TimeUnit::Nanoseconds
    );
    assert_eq!(
        timestamp.clock_domain(),
        &ClockDomain::DeviceMonotonic
    );
}

#[test]
fn timestamps_from_same_domain_can_be_subtracted() {
    let start = device_timestamp(100);
    let end = device_timestamp(150);

    let elapsed = end
        .elapsed_since(&start)
        .expect("same-domain timestamps");

    assert_eq!(elapsed.value(), 50_000_000);
    assert_eq!(
        elapsed.unit(),
        &TimeUnit::Femtoseconds
    );
}

#[test]
fn timestamp_elapsed_difference_is_independent_of_absolute_origin() {
    let first_start = device_timestamp(100);
    let first_end = device_timestamp(150);

    let second_start = device_timestamp(1_000_100);
    let second_end = device_timestamp(1_000_150);

    let first = first_end
        .elapsed_since(&first_start)
        .expect("valid elapsed time");

    let second = second_end
        .elapsed_since(&second_start)
        .expect("valid elapsed time");

    assert_eq!(first, second);
}

#[test]
fn timestamps_from_different_domains_cannot_be_subtracted() {
    let start = device_timestamp(10);
    let end = host_timestamp(20);

    assert!(matches!(
        end.elapsed_since(&start),
        Err(TimingError::ClockDomainMismatch { .. })
    ));
}

#[test]
fn reversed_timestamp_order_is_rejected() {
    let start = device_timestamp(200);
    let end = device_timestamp(100);

    assert!(matches!(
        end.elapsed_since(&start),
        Err(TimingError::EndBeforeStart { .. })
    ));
}

#[test]
fn equal_timestamps_have_zero_elapsed_time() {
    let start = device_timestamp(100);
    let end = device_timestamp(100);

    let elapsed = end
        .elapsed_since(&start)
        .expect("equal timestamps are valid");

    assert!(elapsed.is_zero());
    assert_eq!(
        elapsed.unit(),
        &TimeUnit::Femtoseconds
    );
}

#[test]
fn timestamp_elapsed_time_normalizes_to_femtoseconds() {
    let start = Timestamp::new(
        1,
        TimeUnit::Microseconds,
        ClockDomain::DeviceMonotonic,
    )
    .expect("valid timestamp");

    let end = Timestamp::new(
        2,
        TimeUnit::Microseconds,
        ClockDomain::DeviceMonotonic,
    )
    .expect("valid timestamp");

    let elapsed = end
        .elapsed_since(&start)
        .expect("valid elapsed time");

    assert_eq!(elapsed.value(), 1_000_000_000);
    assert_eq!(
        elapsed.unit(),
        &TimeUnit::Femtoseconds
    );
}

// =============================================================================
// TimingResolution
// =============================================================================

#[test]
fn timing_resolution_must_be_positive() {
    let result =
        TimingResolution::new(0, TimeUnit::Nanoseconds);

    assert!(matches!(
        result,
        Err(TimingError::InvalidValue { .. })
    ));
}

#[test]
fn timing_resolution_rejects_cycle_units() {
    let result =
        TimingResolution::new(1, TimeUnit::Cycles);

    assert!(matches!(
        result,
        Err(TimingError::InvalidValue { .. })
    ));
}

#[test]
fn timing_resolution_preserves_value_and_unit() {
    let resolution =
        TimingResolution::new(10, TimeUnit::Picoseconds)
            .expect("valid resolution");

    assert_eq!(resolution.value(), 10);
    assert_eq!(
        resolution.unit(),
        &TimeUnit::Picoseconds
    );
}

#[test]
fn timing_resolution_can_become_time_value() {
    let resolution =
        TimingResolution::new(10, TimeUnit::Nanoseconds)
            .expect("valid resolution");

    let value = resolution
        .as_time_value()
        .expect("valid timing value");

    assert_eq!(value.value(), 10);
    assert_eq!(
        value.unit(),
        &TimeUnit::Nanoseconds
    );
}

// =============================================================================
// TimingInterval
// =============================================================================

#[test]
fn timing_interval_preserves_source_and_clock() {
    let interval = TimingInterval::new(
        ns(10),
        TimingSource::Device,
        ClockDomain::DeviceMonotonic,
        None,
    )
    .expect("valid interval");

    assert_eq!(interval.duration(), &ns(10));
    assert_eq!(
        interval.source(),
        &TimingSource::Device
    );
    assert_eq!(
        interval.clock_domain(),
        &ClockDomain::DeviceMonotonic
    );
    assert!(interval.resolution().is_none());
}

#[test]
fn timing_interval_can_be_explicitly_zero() {
    let interval = TimingInterval::new(
        TimeValue::zero(TimeUnit::Nanoseconds)
            .expect("valid zero"),
        TimingSource::Device,
        ClockDomain::DeviceMonotonic,
        None,
    )
    .expect("zero interval should be valid");

    assert!(interval.is_zero());
}

#[test]
fn timing_interval_accepts_consistent_resolution() {
    let resolution =
        TimingResolution::new(10, TimeUnit::Nanoseconds)
            .expect("valid resolution");

    let interval = TimingInterval::new(
        ns(100),
        TimingSource::Device,
        ClockDomain::DeviceMonotonic,
        Some(resolution),
    )
    .expect("duration is a multiple of resolution");

    assert!(interval.resolution().is_some());
}

#[test]
fn timing_interval_rejects_inconsistent_resolution() {
    let resolution =
        TimingResolution::new(10, TimeUnit::Nanoseconds)
            .expect("valid resolution");

    let result = TimingInterval::new(
        ns(95),
        TimingSource::Device,
        ClockDomain::DeviceMonotonic,
        Some(resolution),
    );

    assert!(matches!(
        result,
        Err(TimingError::InvalidRecord { .. })
    ));
}

#[test]
fn custom_timing_source_must_not_be_empty() {
    let result = TimingInterval::new(
        ns(10),
        TimingSource::Custom(String::new()),
        ClockDomain::DeviceMonotonic,
        None,
    );

    assert!(matches!(
        result,
        Err(TimingError::EmptyCustomSource)
    ));
}

#[test]
fn whitespace_only_custom_timing_source_is_rejected() {
    let result = TimingInterval::new(
        ns(10),
        TimingSource::Custom("   ".to_owned()),
        ClockDomain::DeviceMonotonic,
        None,
    );

    assert!(matches!(
        result,
        Err(TimingError::EmptyCustomSource)
    ));
}

// =============================================================================
// InstructionTiming
// =============================================================================

#[test]
fn instruction_timing_requires_non_empty_instruction_name() {
    let result = InstructionTiming::new("   ");

    assert!(matches!(
        result,
        Err(TimingError::InvalidValue { .. })
    ));
}

#[test]
fn instruction_timing_can_be_constructed_without_duration() {
    let timing =
        InstructionTiming::new("cx")
            .expect("valid instruction");

    assert_eq!(timing.instruction, "cx");
    assert!(timing.duration.is_none());
    assert!(timing.source.is_none());
    assert!(timing.resolution.is_none());
    assert!(!timing.deterministic);
    assert!(timing.minimum_duration.is_none());
    assert!(timing.maximum_duration.is_none());
    assert!(timing.alignment.is_none());
}

#[test]
fn instruction_timing_preserves_duration() {
    let timing = InstructionTiming::new("cx")
        .expect("valid instruction")
        .with_duration(ns(300));

    assert_eq!(
        timing.duration.as_ref(),
        Some(&ns(300))
    );
}

#[test]
fn instruction_timing_can_be_marked_deterministic() {
    let timing = InstructionTiming::new("x")
        .expect("valid instruction")
        .deterministic(true);

    assert!(timing.deterministic);
}

#[test]
fn instruction_timing_can_preserve_source() {
    let timing = InstructionTiming::new("measure")
        .expect("valid instruction")
        .with_source(TimingSource::Device);

    assert_eq!(
        timing.source,
        Some(TimingSource::Device)
    );
}

#[test]
fn instruction_timing_can_preserve_resolution() {
    let resolution =
        TimingResolution::new(1, TimeUnit::Nanoseconds)
            .expect("valid resolution");

    let timing = InstructionTiming::new("x")
        .expect("valid instruction")
        .with_resolution(resolution.clone());

    assert_eq!(
        timing.resolution,
        Some(resolution)
    );
}

#[test]
fn instruction_timing_accepts_valid_minimum_and_maximum() {
    let timing = InstructionTiming::new("cx")
        .expect("valid instruction")
        .with_minimum_duration(ns(10))
        .expect("valid minimum")
        .with_maximum_duration(ns(20))
        .expect("valid maximum");

    assert!(timing.validate().is_ok());
}

#[test]
fn instruction_timing_rejects_maximum_below_minimum() {
    let timing = InstructionTiming::new("cx")
        .expect("valid instruction")
        .with_minimum_duration(ns(20))
        .expect("valid minimum");

    let result =
        timing.with_maximum_duration(ns(10));

    assert!(matches!(
        result,
        Err(TimingError::InvalidConstraint { .. })
    ));
}

#[test]
fn instruction_timing_rejects_zero_alignment() {
    let timing = InstructionTiming::new("cx")
        .expect("valid instruction");

    let result = timing.with_alignment(
        TimeValue::zero(TimeUnit::Nanoseconds)
            .expect("valid zero"),
    );

    assert!(matches!(
        result,
        Err(TimingError::InvalidAlignment { .. })
    ));
}

#[test]
fn instruction_timing_rejects_non_physical_alignment() {
    let timing = InstructionTiming::new("cx")
        .expect("valid instruction");

    let result = timing.with_alignment(cycles(1));

    assert!(matches!(
        result,
        Err(TimingError::InvalidAlignment { .. })
    ));
}

#[test]
fn instruction_timing_accepts_positive_physical_alignment() {
    let timing = InstructionTiming::new("cx")
        .expect("valid instruction")
        .with_alignment(ns(4))
        .expect("valid alignment");

    assert!(timing.validate().is_ok());
}

// =============================================================================
// QubitTiming
// =============================================================================

#[test]
fn qubit_timing_accepts_normal_qubit_identifier() {
    let timing =
        QubitTiming::new(0)
            .expect("qubit zero is valid");

    assert_eq!(timing.qubit, 0);
}

#[test]
fn reserved_max_qubit_identifier_is_rejected() {
    let result =
        QubitTiming::new(usize::MAX);

    assert!(matches!(
        result,
        Err(TimingError::InvalidValue { .. })
    ));
}

#[test]
fn qubit_timing_accepts_reset_and_measurement() {
    let mut timing =
        QubitTiming::new(0)
            .expect("valid qubit");

    timing.reset = Some(ns(100));
    timing.measurement = Some(ns(500));

    assert!(timing.validate().is_ok());
}

#[test]
fn qubit_timing_accepts_readout_and_feed_forward_latency() {
    let mut timing =
        QubitTiming::new(1)
            .expect("valid qubit");

    timing.readout_latency = Some(ns(50));
    timing.feed_forward_latency = Some(ns(20));

    assert!(timing.validate().is_ok());
}

#[test]
fn qubit_timing_rejects_cycle_based_reset_duration() {
    let mut timing =
        QubitTiming::new(0)
            .expect("valid qubit");

    timing.reset = Some(cycles(2));

    assert!(matches!(
        timing.validate(),
        Err(TimingError::InvalidValue { .. })
    ));
}

#[test]
fn qubit_timing_rejects_cycle_based_alignment() {
    let mut timing =
        QubitTiming::new(0)
            .expect("valid qubit");

    timing.alignment = Some(cycles(4));

    assert!(matches!(
        timing.validate(),
        Err(TimingError::InvalidValue { .. })
    ));
}

// =============================================================================
// SynchronizationConstraint
// =============================================================================

#[test]
fn synchronization_constraint_requires_physical_time() {
    let result =
        SynchronizationConstraint::new(cycles(1));

    assert!(matches!(
        result,
        Err(TimingError::InvalidValue { .. })
    ));
}

#[test]
fn synchronization_constraint_accepts_zero_minimum_separation() {
    let result = SynchronizationConstraint::new(ns(0));

    assert!(result.is_ok());
}

#[test]
fn synchronization_constraint_accepts_valid_range() {
    let constraint =
        SynchronizationConstraint::new(ns(10))
            .expect("valid minimum")
            .with_maximum(ns(20))
            .expect("valid maximum");

    assert!(constraint.validate().is_ok());
}

#[test]
fn synchronization_constraint_rejects_maximum_below_minimum() {
    let constraint =
        SynchronizationConstraint::new(ns(20))
            .expect("valid minimum");

    let result =
        constraint.with_maximum(ns(10));

    assert!(matches!(
        result,
        Err(TimingError::InvalidConstraint { .. })
    ));
}

#[test]
fn synchronization_constraint_can_require_clock_alignment() {
    let constraint =
        SynchronizationConstraint::new(ns(10))
            .expect("valid constraint")
            .clock_aligned(true);

    assert!(constraint.clock_aligned);
}

#[test]
fn synchronization_constraint_can_disable_clock_alignment() {
    let constraint =
        SynchronizationConstraint::new(ns(10))
            .expect("valid constraint")
            .clock_aligned(false);

    assert!(!constraint.clock_aligned);
}

// =============================================================================
// TimingRecord
// =============================================================================

#[test]
fn timing_record_can_be_constructed() {
    let record = TimingRecord::new(
        TimingSource::Device,
        ClockDomain::DeviceMonotonic,
        TimingCategory::Execution,
    )
    .expect("valid timing record");

    assert_eq!(
        record.schema_version,
        HARDWARE_TIMING_SCHEMA_VERSION
    );
    assert_eq!(
        record.source,
        TimingSource::Device
    );
    assert_eq!(
        record.clock_domain,
        ClockDomain::DeviceMonotonic
    );
    assert_eq!(
        record.category,
        TimingCategory::Execution
    );
}

#[test]
fn timing_record_can_store_explicit_duration() {
    let record = TimingRecord::new(
        TimingSource::Device,
        ClockDomain::DeviceMonotonic,
        TimingCategory::Execution,
    )
    .expect("valid record")
    .with_duration(ns(50));

    assert_eq!(
        record.duration,
        Some(ns(50))
    );
}

#[test]
fn timing_record_accepts_start_and_end_in_same_clock_domain() {
    let record = TimingRecord::new(
        TimingSource::Device,
        ClockDomain::DeviceMonotonic,
        TimingCategory::Execution,
    )
    .expect("valid record")
    .with_start(device_timestamp(100))
    .expect("compatible start")
    .with_end(device_timestamp(150))
    .expect("compatible end");

    assert!(record.validate().is_ok());
}

#[test]
fn timing_record_rejects_start_from_wrong_clock_domain() {
    let result = TimingRecord::new(
        TimingSource::Device,
        ClockDomain::DeviceMonotonic,
        TimingCategory::Execution,
    )
    .expect("valid record")
    .with_start(host_timestamp(100));

    assert!(matches!(
        result,
        Err(TimingError::ClockDomainMismatch { .. })
    ));
}

#[test]
fn timing_record_rejects_end_from_wrong_clock_domain() {
    let result = TimingRecord::new(
        TimingSource::Device,
        ClockDomain::DeviceMonotonic,
        TimingCategory::Execution,
    )
    .expect("valid record")
    .with_end(host_timestamp(100));

    assert!(matches!(
        result,
        Err(TimingError::ClockDomainMismatch { .. })
    ));
}

#[test]
fn timing_record_derives_duration_from_timestamps() {
    let record = TimingRecord::new(
        TimingSource::Device,
        ClockDomain::DeviceMonotonic,
        TimingCategory::Execution,
    )
    .expect("valid record")
    .with_start(device_timestamp(1_000))
    .expect("valid start")
    .with_end(device_timestamp(1_500))
    .expect("valid end");

    let duration = record
        .derive_duration()
        .expect("duration derivation must succeed")
        .expect("both timestamps exist");

    assert_eq!(duration.value(), 500_000_000);
    assert_eq!(
        duration.unit(),
        &TimeUnit::Femtoseconds
    );
}

#[test]
fn timing_record_without_both_timestamps_has_no_derived_duration() {
    let record = TimingRecord::new(
        TimingSource::Device,
        ClockDomain::DeviceMonotonic,
        TimingCategory::Execution,
    )
    .expect("valid record")
    .with_start(device_timestamp(100))
    .expect("valid start");

    assert_eq!(
        record
            .derive_duration()
            .expect("derivation should succeed"),
        None
    );
}

#[test]
fn timing_record_rejects_inconsistent_explicit_duration() {
    let record = TimingRecord::new(
        TimingSource::Device,
        ClockDomain::DeviceMonotonic,
        TimingCategory::Execution,
    )
    .expect("valid record")
    .with_duration(ns(20))
    .with_start(device_timestamp(1_000))
    .expect("valid start")
    .with_end(device_timestamp(1_050))
    .expect("valid end");

    assert!(matches!(
        record.validate(),
        Err(TimingError::InvalidRecord { .. })
    ));
}

#[test]
fn timing_record_accepts_consistent_explicit_duration() {
    let record = TimingRecord::new(
        TimingSource::Device,
        ClockDomain::DeviceMonotonic,
        TimingCategory::Execution,
    )
    .expect("valid record")
    .with_duration(ns(50))
    .with_start(device_timestamp(1_000))
    .expect("valid start")
    .with_end(device_timestamp(1_050))
    .expect("valid end");

    assert!(record.validate().is_ok());
}

#[test]
fn timing_record_preserves_source_id() {
    let record = TimingRecord::new(
        TimingSource::Provider,
        ClockDomain::ProviderMonotonic,
        TimingCategory::Execution,
    )
    .expect("valid record")
    .with_source_id("provider-device-01")
    .expect("valid source id");

    assert_eq!(
        record.source_id.as_deref(),
        Some("provider-device-01")
    );
}

#[test]
fn timing_record_rejects_empty_source_id() {
    let result = TimingRecord::new(
        TimingSource::Provider,
        ClockDomain::ProviderMonotonic,
        TimingCategory::Execution,
    )
    .expect("valid record")
    .with_source_id("   ");

    assert!(matches!(
        result,
        Err(TimingError::InvalidValue { .. })
    ));
}

// =============================================================================
// TimingProvenance
// =============================================================================

#[test]
fn timing_provenance_preserves_source() {
    let provenance =
        TimingProvenance::new(TimingSource::Device)
            .expect("valid provenance");

    assert_eq!(
        provenance.source,
        TimingSource::Device
    );
    assert_eq!(
        provenance.schema_version,
        HARDWARE_TIMING_SCHEMA_VERSION
    );
}

#[test]
fn timing_provenance_accepts_source_id() {
    let provenance =
        TimingProvenance::new(TimingSource::Provider)
            .expect("valid provenance")
            .with_source_id("provider-device-01")
            .expect("valid source id");

    assert_eq!(
        provenance.source_id.as_deref(),
        Some("provider-device-01")
    );
}

#[test]
fn timing_provenance_rejects_empty_source_id() {
    let result =
        TimingProvenance::new(TimingSource::Provider)
            .expect("valid provenance")
            .with_source_id("   ");

    assert!(matches!(
        result,
        Err(TimingError::InvalidValue { .. })
    ));
}

#[test]
fn timing_provenance_preserves_provider_and_firmware_versions() {
    let provenance =
        TimingProvenance::new(TimingSource::Device)
            .expect("valid provenance")
            .with_provider_version("2026.08")
            .with_firmware_version("fw-7.2");

    assert_eq!(
        provenance.provider_version.as_deref(),
        Some("2026.08")
    );
    assert_eq!(
        provenance.firmware_version.as_deref(),
        Some("fw-7.2")
    );
}

// =============================================================================
// TimingSpecification
// =============================================================================

#[test]
fn empty_timing_specification_is_valid() {
    let specification =
        TimingSpecification::new();

    assert!(specification.validate().is_ok());
    assert!(specification.instructions.is_empty());
    assert!(specification.qubits.is_empty());
}

#[test]
fn timing_specification_can_store_clock_rate() {
    let rate =
        ClockRate::from_hz(5_000_000_000)
            .expect("valid clock");

    let specification =
        TimingSpecification::new()
            .with_clock_rate(rate);

    assert_eq!(
        specification.clock_rate,
        Some(rate)
    );
}

#[test]
fn timing_specification_accepts_positive_alignment() {
    let specification =
        TimingSpecification::new()
            .with_alignment(ns(4))
            .expect("valid alignment");

    assert!(specification.validate().is_ok());
}

#[test]
fn timing_specification_rejects_zero_alignment() {
    let result =
        TimingSpecification::new()
            .with_alignment(
                TimeValue::zero(
                    TimeUnit::Nanoseconds,
                )
                .expect("valid zero"),
            );

    assert!(matches!(
        result,
        Err(TimingError::InvalidAlignment { .. })
    ));
}

#[test]
fn timing_specification_rejects_cycle_alignment() {
    let result =
        TimingSpecification::new()
            .with_alignment(cycles(4));

    assert!(matches!(
        result,
        Err(TimingError::InvalidValue { .. })
    ));
}

#[test]
fn timing_specification_can_store_classical_processing_latency() {
    let specification =
        TimingSpecification::new()
            .with_classical_processing_latency(ns(50))
            .expect("valid latency");

    assert_eq!(
        specification.classical_processing_latency,
        Some(ns(50))
    );
}

#[test]
fn timing_specification_can_store_feed_forward_latency() {
    let specification =
        TimingSpecification::new()
            .with_feed_forward_latency(ns(100))
            .expect("valid latency");

    assert_eq!(
        specification.feed_forward_latency,
        Some(ns(100))
    );
}

#[test]
fn timing_specification_can_store_synchronization_latency() {
    let specification =
        TimingSpecification::new()
            .with_synchronization_latency(ns(20))
            .expect("valid latency");

    assert_eq!(
        specification.synchronization_latency,
        Some(ns(20))
    );
}

#[test]
fn timing_specification_rejects_non_physical_classical_latency() {
    let result =
        TimingSpecification::new()
            .with_classical_processing_latency(
                cycles(1),
            );

    assert!(matches!(
        result,
        Err(TimingError::InvalidValue { .. })
    ));
}

#[test]
fn timing_specification_rejects_non_physical_feed_forward_latency() {
    let result =
        TimingSpecification::new()
            .with_feed_forward_latency(
                cycles(1),
            );

    assert!(matches!(
        result,
        Err(TimingError::InvalidValue { .. })
    ));
}

#[test]
fn timing_specification_rejects_non_physical_synchronization_latency() {
    let result =
        TimingSpecification::new()
            .with_synchronization_latency(
                cycles(1),
            );

    assert!(matches!(
        result,
        Err(TimingError::InvalidValue { .. })
    ));
}

#[test]
fn timing_specification_can_add_instruction() {
    let mut specification =
        TimingSpecification::new();

    specification
        .add_instruction(
            InstructionTiming::new("cx")
                .expect("valid instruction")
                .with_duration(ns(300))
                .deterministic(true),
        )
        .expect("instruction should be accepted");

    assert_eq!(
        specification.instructions.len(),
        1
    );
    assert_eq!(
        specification
            .instruction("cx")
            .expect("instruction must exist")
            .instruction,
        "cx"
    );
}

#[test]
fn timing_specification_can_add_qubit() {
    let mut specification =
        TimingSpecification::new();

    specification
        .add_qubit(
            QubitTiming::new(0)
                .expect("valid qubit"),
        )
        .expect("qubit timing should be accepted");

    assert_eq!(
        specification.qubits.len(),
        1
    );
    assert!(
        specification.qubit(0).is_some()
    );
}

#[test]
fn timing_specification_lookup_is_exact() {
    let mut specification =
        TimingSpecification::new();

    specification
        .add_instruction(
            InstructionTiming::new("cx")
                .expect("valid instruction"),
        )
        .expect("valid instruction");

    assert!(
        specification.instruction("cx").is_some()
    );

    assert!(
        specification.instruction("CX").is_none()
    );
}

#[test]
fn timing_specification_lookup_returns_none_for_missing_qubit() {
    let specification =
        TimingSpecification::new();

    assert!(
        specification.qubit(999).is_none()
    );
}

#[test]
fn timing_specification_validates_complete_hardware_timing() {
    let rate =
        ClockRate::from_hz(5_000_000_000)
            .expect("valid clock");

    let mut specification =
        TimingSpecification::new()
            .with_clock_rate(rate)
            .with_alignment(ns(4))
            .expect("valid alignment")
            .with_classical_processing_latency(ns(20))
            .expect("valid classical latency")
            .with_feed_forward_latency(ns(50))
            .expect("valid feed-forward latency")
            .with_synchronization_latency(ns(10))
            .expect("valid synchronization latency");

    specification
        .add_instruction(
            InstructionTiming::new("x")
                .expect("valid instruction")
                .with_duration(ns(20))
                .with_resolution(
                    TimingResolution::new(
                        1,
                        TimeUnit::Nanoseconds,
                    )
                    .expect("valid resolution"),
                )
                .with_source(TimingSource::Device)
                .deterministic(true)
                .with_minimum_duration(ns(10))
                .expect("valid minimum")
                .with_maximum_duration(ns(30))
                .expect("valid maximum")
                .with_alignment(ns(4))
                .expect("valid alignment"),
        )
        .expect("valid instruction timing");

    specification
        .add_qubit(
            QubitTiming::new(0)
                .expect("valid qubit")
        )
        .expect("valid qubit timing");

    assert!(
        specification.validate().is_ok()
    );
}

// =============================================================================
// Timing source contract
// =============================================================================

#[test]
fn standard_timing_sources_have_stable_names() {
    assert_eq!(TimingSource::Host.as_str(), "host");
    assert_eq!(TimingSource::Device.as_str(), "device");
    assert_eq!(
        TimingSource::Provider.as_str(),
        "provider"
    );
    assert_eq!(
        TimingSource::Simulator.as_str(),
        "simulator"
    );
    assert_eq!(
        TimingSource::Emulator.as_str(),
        "emulator"
    );
    assert_eq!(
        TimingSource::External.as_str(),
        "external"
    );
}

#[test]
fn valid_custom_timing_source_is_preserved() {
    let source =
        TimingSource::Custom("oscilloscope".to_owned());

    assert_eq!(
        source.as_str(),
        "oscilloscope"
    );

    assert!(source.validate().is_ok());
}

// =============================================================================
// Timing category coverage
// =============================================================================

#[test]
fn timing_categories_cover_core_hardware_phases() {
    let categories = [
        TimingCategory::Instruction,
        TimingCategory::Reset,
        TimingCategory::Measurement,
        TimingCategory::ClassicalProcessing,
        TimingCategory::FeedForward,
        TimingCategory::Synchronization,
        TimingCategory::Queue,
        TimingCategory::Execution,
        TimingCategory::Readout,
        TimingCategory::Communication,
        TimingCategory::AnalogProgram,
        TimingCategory::Annealing,
        TimingCategory::LogicalOperation,
        TimingCategory::ProviderDefined,
    ];

    assert_eq!(categories.len(), 14);
}

// =============================================================================
// Cross-layer semantic regression tests
// =============================================================================

#[test]
fn device_timing_must_not_be_implicitly_interpreted_as_host_timing() {
    let device =
        device_timestamp(1_000);

    let host =
        host_timestamp(2_000);

    assert!(matches!(
        device.elapsed_since(&host),
        Err(TimingError::ClockDomainMismatch { .. })
    ));
}

#[test]
fn provider_timing_must_not_be_subtracted_from_device_timing_without_sync() {
    let provider = Timestamp::new(
        1_000,
        TimeUnit::Nanoseconds,
        ClockDomain::ProviderMonotonic,
    )
    .expect("valid provider timestamp");

    let device = Timestamp::new(
        2_000,
        TimeUnit::Nanoseconds,
        ClockDomain::DeviceMonotonic,
    )
    .expect("valid device timestamp");

    assert!(matches!(
        device.elapsed_since(&provider),
        Err(TimingError::ClockDomainMismatch { .. })
    ));
}

#[test]
fn timing_does_not_treat_cycles_as_time_without_frequency() {
    let cycle_value = cycles(100);

    assert!(cycle_value.to_femtoseconds().is_err());
}

#[test]
fn timing_does_not_silently_truncate_sub_nanosecond_measurements() {
    let measurement = ps(250);

    assert!(matches!(
        measurement.to_duration(),
        Err(TimingError::LossyDurationConversion { .. })
    ));
}

#[test]
fn timing_preserves_exact_sub_nanosecond_measurements_in_canonical_units() {
    let measurement = ps(250);

    assert_eq!(
        measurement
            .to_femtoseconds()
            .expect("exact conversion"),
        250_000
    );
}

#[test]
fn explicit_zero_duration_remains_valid_evidence() {
    let record = TimingRecord::new(
        TimingSource::Device,
        ClockDomain::DeviceMonotonic,
        TimingCategory::Execution,
    )
    .expect("valid record")
    .with_duration(
        TimeValue::zero(TimeUnit::Nanoseconds)
            .expect("valid zero"),
    );

    assert!(record.validate().is_ok());
    assert_eq!(
        record.duration
            .expect("explicit duration")
            .value(),
        0
    );
}

#[test]
fn timing_records_remain_provider_neutral() {
    let record = TimingRecord::new(
        TimingSource::Provider,
        ClockDomain::ProviderMonotonic,
        TimingCategory::Execution,
    )
    .expect("provider timing should remain generic");

    assert_eq!(
        record.source,
        TimingSource::Provider
    );
    assert_eq!(
        record.clock_domain,
        ClockDomain::ProviderMonotonic
    );
}

#[test]
fn timing_model_supports_simulator_measurements() {
    let record = TimingRecord::new(
        TimingSource::Simulator,
        ClockDomain::Simulation,
        TimingCategory::Execution,
    )
    .expect("simulator timing must be supported")
    .with_duration(ns(100));

    assert!(record.validate().is_ok());
}

#[test]
fn timing_model_supports_emulator_measurements() {
    let record = TimingRecord::new(
        TimingSource::Emulator,
        ClockDomain::Simulation,
        TimingCategory::Execution,
    )
    .expect("emulator timing must be supported")
    .with_duration(ns(100));

    assert!(record.validate().is_ok());
}

// =============================================================================
// Determinism / value semantics
// =============================================================================

#[test]
fn identical_time_values_are_equal() {
    assert_eq!(ns(100), ns(100));
}

#[test]
fn identical_timestamps_are_equal() {
    assert_eq!(
        device_timestamp(100),
        device_timestamp(100)
    );
}

#[test]
fn identical_clock_rates_are_equal() {
    let left =
        ClockRate::from_hz(1_000_000_000)
            .expect("valid rate");

    let right =
        ClockRate::from_hz(1_000_000_000)
            .expect("valid rate");

    assert_eq!(left, right);
}

#[test]
fn identical_resolution_values_are_equal() {
    let left =
        TimingResolution::new(
            1,
            TimeUnit::Nanoseconds,
        )
        .expect("valid resolution");

    let right =
        TimingResolution::new(
            1,
            TimeUnit::Nanoseconds,
        )
        .expect("valid resolution");

    assert_eq!(left, right);
}

// =============================================================================
// Display contract
// =============================================================================

#[test]
fn time_unit_display_is_stable() {
    assert_eq!(
        TimeUnit::Nanoseconds.to_string(),
        "ns"
    );
    assert_eq!(
        TimeUnit::Cycles.to_string(),
        "cycles"
    );
}

#[test]
fn time_value_display_contains_value_and_unit() {
    let value = ns(42);

    assert_eq!(
        value.to_string(),
        "42 ns"
    );
}

#[test]
fn clock_rate_display_uses_hz_when_exact() {
    let rate =
        ClockRate::from_hz(5_000_000_000)
            .expect("valid rate");

    assert_eq!(
        rate.to_string(),
        "5000000000 Hz"
    );
}

#[test]
fn duration_display_is_explicitly_attosecond_based() {
    // The public scheduling Duration is a different type and must not be
    // confused with std::time::Duration. This test is intentionally scoped to
    // the timing module and therefore does not import scheduling::Duration.
    assert_eq!(
        ns(1).to_string(),
        "1 ns"
    );
}

// =============================================================================
// Regression: relationship consistency
// =============================================================================

#[test]
fn record_derived_duration_matches_direct_time_conversion() {
    let start = device_timestamp(10_000);
    let end = device_timestamp(10_250);

    let elapsed = end
        .elapsed_since(&start)
        .expect("valid elapsed");

    let direct = ns(250)
        .to_femtoseconds()
        .expect("valid conversion");

    assert_eq!(
        elapsed.value(),
        direct
    );
}

#[test]
fn instruction_minimum_and_maximum_use_common_physical_scale() {
    let timing = InstructionTiming::new("rz")
        .expect("valid instruction")
        .with_minimum_duration(ps(500))
        .expect("valid minimum")
        .with_maximum_duration(ns(1))
        .expect("1 ns is greater than 500 ps");

    assert!(timing.validate().is_ok());
}

#[test]
fn instruction_minimum_above_maximum_is_rejected_across_units() {
    let timing = InstructionTiming::new("rz")
        .expect("valid instruction")
        .with_minimum_duration(ns(2))
        .expect("valid minimum");

    let result =
        timing.with_maximum_duration(ps(1_500));

    assert!(matches!(
        result,
        Err(TimingError::InvalidConstraint { .. })
    ));
}

#[test]
fn synchronization_range_is_compared_across_units() {
    let constraint =
        SynchronizationConstraint::new(
            ns(1),
        )
        .expect("valid minimum")
        .with_maximum(ps(1_000))
        .expect("equal physical duration");

    assert!(constraint.validate().is_ok());
}

// =============================================================================
// Production invariant summary
// =============================================================================

#[test]
fn production_invariants_are_exercised_by_public_api() {
    // Integer/exact timing.
    assert_eq!(
        ns(1).to_femtoseconds().unwrap(),
        1_000_000
    );

    // No silent precision loss.
    assert!(
        ps(1).to_duration().is_err()
    );

    // Cycles require a clock.
    assert!(
        cycles(1).to_femtoseconds().is_err()
    );

    // Clock domains are isolated.
    assert!(matches!(
        host_timestamp(1)
            .elapsed_since(&device_timestamp(2)),
        Err(TimingError::ClockDomainMismatch { .. })
    ));

    // Invalid ranges fail.
    let instruction = InstructionTiming::new("cx")
        .expect("valid instruction")
        .with_minimum_duration(ns(20))
        .expect("valid minimum");

    assert!(
        instruction
            .with_maximum_duration(ns(10))
            .is_err()
    );

    // Complete records validate.
    let record = TimingRecord::new(
        TimingSource::Device,
        ClockDomain::DeviceMonotonic,
        TimingCategory::Execution,
    )
    .expect("valid record")
    .with_duration(ns(10));

    assert!(record.validate().is_ok());
}
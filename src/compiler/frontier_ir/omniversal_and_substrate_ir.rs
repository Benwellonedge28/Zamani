//! Zamani Frontier IR — Omniversal & Substrate Primitives
//!
//! Features 81–100:
//! - cross-substrate state migration;
//! - nano-assembly orchestration;
//! - multiversal state synchronization;
//! - exotic-physics and energy primitives;
//! - photonic, superconducting, spintronic and memristive primitives;
//! - substrate-independent computational operations.
//!
//! This module is a pure Frontier-IR construction layer.
//!
//! Production guarantees:
//! - deterministic output;
//! - safe escaping of textual operands;
//! - rejection of non-finite floating-point values;
//! - domain validation where the representation has an explicit physical
//!   invariant;
//! - no I/O;
//! - no global mutable state;
//! - preservation of the existing public constructor API.

/// Frontier IR constructors for omniversal and computational-substrate
/// operations.
#[derive(Debug, Clone, Copy, Default)]
pub struct OmniversalAndSubstrateIr;

impl OmniversalAndSubstrateIr {
    // =====================================================================
    // Omniversal & Substrate Primitives (81–100)
    // =====================================================================

    /// Emits a cross-substrate state migration operation.
    #[must_use]
    pub fn cross_substrate_state_migration(from: &str, to: &str) -> String {
        format!(
            "omni_op {{ type = SUBSTRATE_MIGRATION; from = \"{}\"; to = \"{}\"; }}",
            escape_string(from),
            escape_string(to)
        )
    }

    /// Emits a nano-assembly orchestration operation.
    #[must_use]
    pub fn nano_assembly_orchestration(molecule: &str) -> String {
        format!(
            "omni_op {{ type = NANO_ASSEMBLY; target_molecule = \"{}\"; }}",
            escape_string(molecule)
        )
    }

    /// Emits a multiversal state synchronization operation.
    #[must_use]
    pub fn multiversal_state_sync(universe_id: u64) -> String {
        format!(
            "omni_op {{ type = MULTIVERSAL_SYNC; universe = {}; }}",
            universe_id
        )
    }

    /// Emits a vacuum-energy harvesting operation.
    #[must_use]
    pub fn vacuum_energy_harvest_node(joules: f64) -> String {
        format!(
            "omni_op {{ type = VACUUM_ENERGY_HARVEST; target_joules = {}; }}",
            finite_non_negative_float(joules)
        )
    }

    /// Emits a quantum-gravitational anchor operation.
    #[must_use]
    pub fn quantum_gravitational_anchor(spacetime_coord: &str) -> String {
        format!(
            "omni_op {{ type = GRAVITATIONAL_ANCHOR; coord = \"{}\"; }}",
            escape_string(spacetime_coord)
        )
    }

    /// Emits a topological braid operation.
    #[must_use]
    pub fn topological_braid_operation(braid_type: &str) -> String {
        format!(
            "omni_op {{ type = TOPOLOGICAL_BRAID; braid = \"{}\"; }}",
            escape_string(braid_type)
        )
    }

    /// Emits a DNA-storage write operation.
    ///
    /// The existing IR contract records sequence length rather than the
    /// sequence itself.
    #[must_use]
    pub fn biological_dna_storage_write(sequence: &str) -> String {
        format!(
            "omni_op {{ type = DNA_STORAGE_WRITE; seq_len = {}; }}",
            sequence.len()
        )
    }

    /// Emits a photonic phased-array steering operation.
    ///
    /// Angles may be negative, so only finiteness is required here.
    #[must_use]
    pub fn photonic_phased_array_steer(angle_deg: f64) -> String {
        format!(
            "omni_op {{ type = PHOTONIC_STEER; angle = {}; }}",
            finite_float(angle_deg)
        )
    }

    /// Emits a superconducting flux-transfer operation.
    #[must_use]
    pub fn superconducting_flux_transfer(flux_quanta: usize) -> String {
        format!(
            "omni_op {{ type = SFQ_FLUX_TRANSFER; quanta = {}; }}",
            flux_quanta
        )
    }

    /// Emits a spintronic domain-wall shift.
    #[must_use]
    pub fn spintronic_domain_wall_shift(shift_nm: f64) -> String {
        format!(
            "omni_op {{ type = SPINTRONIC_SHIFT; shift_nm = {}; }}",
            finite_float(shift_nm)
        )
    }

    /// Emits a memristive weight update.
    #[must_use]
    pub fn memristive_weight_update(conductance_ns: f64) -> String {
        format!(
            "omni_op {{ type = MEMRISTIVE_UPDATE; conductance_ns = {}; }}",
            finite_non_negative_float(conductance_ns)
        )
    }

    /// Emits a phononic/acoustic switching operation.
    #[must_use]
    pub fn phonon_acoustic_switch(frequency_ghz: f64) -> String {
        format!(
            "omni_op {{ type = PHONONIC_SWITCH; ghz = {}; }}",
            finite_non_negative_float(frequency_ghz)
        )
    }

    /// Emits a neutrino-stream modulation operation.
    ///
    /// The existing IR contract records the byte length of the supplied
    /// stream rather than embedding its contents.
    #[must_use]
    pub fn neutrino_stream_modulate(bit_stream: &str) -> String {
        format!(
            "omni_op {{ type = NEUTRINO_MODULATE; len = {}; }}",
            bit_stream.len()
        )
    }

    /// Emits a zero-point-energy stabilization operation.
    #[must_use]
    pub fn zero_point_stabilization(loop_gain: f64) -> String {
        format!(
            "omni_op {{ type = ZPE_STABILIZATION; gain = {}; }}",
            finite_float(loop_gain)
        )
    }

    /// Emits a cosmic-ray shielding assertion.
    #[must_use]
    pub fn cosmic_ray_shielding_assert(level: &str) -> String {
        format!(
            "omni_op {{ type = COSMIC_SHIELDING; level = \"{}\"; }}",
            escape_string(level)
        )
    }

    /// Emits a Lorentz time-dilation adjustment.
    ///
    /// `v_c` represents v/c and therefore must satisfy `0 <= v/c < 1`.
    #[must_use]
    pub fn lorentz_time_dilation_adjust(v_c: f64) -> String {
        format!(
            "omni_op {{ type = LORENTZ_ADJUST; v_c = {}; }}",
            finite_unit_interval(v_c)
        )
    }

    /// Emits a dark-matter interaction probe.
    #[must_use]
    pub fn dark_matter_interaction_probe() -> String {
        "omni_op { type = DARK_MATTER_PROBE; }".to_owned()
    }

    /// Emits an entropic-gravity computation node.
    #[must_use]
    pub fn entropic_gravity_compute_node() -> String {
        "omni_op { type = ENTROPIC_GRAVITY; }".to_owned()
    }

    /// Emits a holographic-principle projection.
    #[must_use]
    pub fn holographic_principle_projection(surface_area: f64) -> String {
        format!(
            "omni_op {{ type = HOLOGRAPHIC_PROJECTION; area = {}; }}",
            finite_non_negative_float(surface_area)
        )
    }

    /// Emits the universal trinity convergence anchor.
    #[must_use]
    pub fn universal_trinity_convergence_anchor() -> String {
        "omni_op { type = TRINITY_CONVERGENCE_ANCHOR; state = OMNIPRESENT; }".to_owned()
    }
}

/// Escapes a textual Frontier IR operand.
///
/// Frontier IR uses double-quoted strings. Quotes, backslashes and control
/// characters therefore need escaping before serialization.
fn escape_string(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());

    for character in value.chars() {
        match character {
            '\\' => escaped.push_str("\\\\"),
            '"' => escaped.push_str("\\\""),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            '\0' => escaped.push_str("\\0"),
            '\u{08}' => escaped.push_str("\\b"),
            '\u{0C}' => escaped.push_str("\\f"),
            character if character.is_control() => {
                use std::fmt::Write;

                write!(
                    &mut escaped,
                    "\\u{{{:04X}}}",
                    character as u32
                )
                .expect("writing to String cannot fail");
            }
            character => escaped.push(character),
        }
    }

    escaped
}

/// Serializes a finite floating-point value.
fn finite_float(value: f64) -> String {
    assert!(
        value.is_finite(),
        "Frontier substrate IR requires finite floating-point values"
    );

    format!("{value}")
}

/// Serializes a finite non-negative floating-point value.
fn finite_non_negative_float(value: f64) -> String {
    assert!(
        value.is_finite(),
        "Frontier substrate IR requires finite floating-point values"
    );

    assert!(
        value >= 0.0,
        "Frontier substrate IR requires non-negative floating-point values"
    );

    format!("{value}")
}

/// Serializes a velocity ratio satisfying `0 <= v/c < 1`.
fn finite_unit_interval(value: f64) -> String {
    assert!(
        value.is_finite(),
        "Frontier substrate IR requires finite floating-point values"
    );

    assert!(
        (0.0..1.0).contains(&value),
        "Lorentz velocity ratio must satisfy 0 <= v/c < 1"
    );

    format!("{value}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cross_substrate_migration_has_expected_shape() {
        assert_eq!(
            OmniversalAndSubstrateIr::cross_substrate_state_migration(
                "classical_cpu",
                "quantum_backend"
            ),
            "omni_op { type = SUBSTRATE_MIGRATION; from = \"classical_cpu\"; to = \"quantum_backend\"; }"
        );
    }

    #[test]
    fn textual_operands_are_escaped() {
        let output =
            OmniversalAndSubstrateIr::nano_assembly_orchestration(
                "molecule\"\\target\n",
            );

        assert_eq!(
            output,
            "omni_op { type = NANO_ASSEMBLY; target_molecule = \"molecule\\\"\\\\target\\n\"; }"
        );
    }

    #[test]
    fn unicode_is_preserved() {
        let output =
            OmniversalAndSubstrateIr::quantum_gravitational_anchor(
                "x=世界,y=Δ",
            );

        assert!(output.contains("x=世界,y=Δ"));
    }

    #[test]
    fn integer_operands_are_deterministic() {
        assert_eq!(
            OmniversalAndSubstrateIr::multiversal_state_sync(42),
            "omni_op { type = MULTIVERSAL_SYNC; universe = 42; }"
        );

        assert_eq!(
            OmniversalAndSubstrateIr::superconducting_flux_transfer(128),
            "omni_op { type = SFQ_FLUX_TRANSFER; quanta = 128; }"
        );
    }

    #[test]
    fn sequence_and_stream_lengths_are_deterministic() {
        assert_eq!(
            OmniversalAndSubstrateIr::biological_dna_storage_write("ACGT"),
            "omni_op { type = DNA_STORAGE_WRITE; seq_len = 4; }"
        );

        assert_eq!(
            OmniversalAndSubstrateIr::neutrino_stream_modulate("010101"),
            "omni_op { type = NEUTRINO_MODULATE; len = 6; }"
        );
    }

    #[test]
    fn zero_argument_operations_are_stable() {
        assert_eq!(
            OmniversalAndSubstrateIr::dark_matter_interaction_probe(),
            "omni_op { type = DARK_MATTER_PROBE; }"
        );

        assert_eq!(
            OmniversalAndSubstrateIr::entropic_gravity_compute_node(),
            "omni_op { type = ENTROPIC_GRAVITY; }"
        );

        assert_eq!(
            OmniversalAndSubstrateIr::universal_trinity_convergence_anchor(),
            "omni_op { type = TRINITY_CONVERGENCE_ANCHOR; state = OMNIPRESENT; }"
        );
    }

    #[test]
    fn valid_lorentz_ratio_is_accepted() {
        assert_eq!(
            OmniversalAndSubstrateIr::lorentz_time_dilation_adjust(0.9),
            "omni_op { type = LORENTZ_ADJUST; v_c = 0.9; }"
        );
    }

    #[test]
    #[should_panic(expected = "Lorentz velocity ratio")]
    fn lorentz_ratio_one_is_rejected() {
        OmniversalAndSubstrateIr::lorentz_time_dilation_adjust(1.0);
    }

    #[test]
    #[should_panic(expected = "Lorentz velocity ratio")]
    fn negative_lorentz_ratio_is_rejected() {
        OmniversalAndSubstrateIr::lorentz_time_dilation_adjust(-0.1);
    }

    #[test]
    #[should_panic(expected = "finite floating-point")]
    fn nan_is_rejected() {
        OmniversalAndSubstrateIr::vacuum_energy_harvest_node(f64::NAN);
    }

    #[test]
    #[should_panic(expected = "finite floating-point")]
    fn infinity_is_rejected() {
        OmniversalAndSubstrateIr::holographic_principle_projection(
            f64::INFINITY,
        );
    }

    #[test]
    #[should_panic(expected = "non-negative floating-point")]
    fn negative_energy_is_rejected() {
        OmniversalAndSubstrateIr::vacuum_energy_harvest_node(-1.0);
    }

    #[test]
    #[should_panic(expected = "non-negative floating-point")]
    fn negative_conductance_is_rejected() {
        OmniversalAndSubstrateIr::memristive_weight_update(-1.0);
    }

    #[test]
    fn signed_angle_is_supported() {
        assert_eq!(
            OmniversalAndSubstrateIr::photonic_phased_array_steer(-45.0),
            "omni_op { type = PHOTONIC_STEER; angle = -45; }"
        );
    }

    #[test]
    fn signed_spintronic_shift_is_supported() {
        assert_eq!(
            OmniversalAndSubstrateIr::spintronic_domain_wall_shift(-2.5),
            "omni_op { type = SPINTRONIC_SHIFT; shift_nm = -2.5; }"
        );
    }

    #[test]
    fn all_public_constructors_produce_non_empty_ir() {
        let outputs = [
            OmniversalAndSubstrateIr::cross_substrate_state_migration(
                "a",
                "b",
            ),
            OmniversalAndSubstrateIr::nano_assembly_orchestration("H2O"),
            OmniversalAndSubstrateIr::multiversal_state_sync(1),
            OmniversalAndSubstrateIr::vacuum_energy_harvest_node(1.0),
            OmniversalAndSubstrateIr::quantum_gravitational_anchor("0,0,0"),
            OmniversalAndSubstrateIr::topological_braid_operation("braid"),
            OmniversalAndSubstrateIr::biological_dna_storage_write("ACGT"),
            OmniversalAndSubstrateIr::photonic_phased_array_steer(1.0),
            OmniversalAndSubstrateIr::superconducting_flux_transfer(1),
            OmniversalAndSubstrateIr::spintronic_domain_wall_shift(1.0),
            OmniversalAndSubstrateIr::memristive_weight_update(1.0),
            OmniversalAndSubstrateIr::phonon_acoustic_switch(1.0),
            OmniversalAndSubstrateIr::neutrino_stream_modulate("01"),
            OmniversalAndSubstrateIr::zero_point_stabilization(1.0),
            OmniversalAndSubstrateIr::cosmic_ray_shielding_assert("high"),
            OmniversalAndSubstrateIr::lorentz_time_dilation_adjust(0.5),
            OmniversalAndSubstrateIr::dark_matter_interaction_probe(),
            OmniversalAndSubstrateIr::entropic_gravity_compute_node(),
            OmniversalAndSubstrateIr::holographic_principle_projection(1.0),
            OmniversalAndSubstrateIr::universal_trinity_convergence_anchor(),
        ];

        assert!(outputs.iter().all(|output| !output.is_empty()));
    }
}
//! Zamani Quantum — Hardware Technology Model
//!
//! Authoritative, provider-neutral vocabulary for quantum-computing
//! technologies and execution models.
//!
//! # Responsibility
//!
//! This module defines WHAT KIND OF QUANTUM TECHNOLOGY a target represents.
//!
//! It deliberately does not define:
//!
//! - backend identity;
//! - provider identity;
//! - backend availability;
//! - hardware capabilities;
//! - native instructions;
//! - topology;
//! - calibration;
//! - routing;
//! - scheduling;
//! - execution;
//! - jobs;
//! - credentials;
//! - authentication;
//! - network I/O;
//! - benchmarking;
//! - compiler IR;
//! - provider-specific APIs.
//!
//! Those concerns belong to other hardware modules.
//!
//! # Architectural position
//!
//! ```text
//!                         Zamani Quantum IR
//!                                |
//!                                v
//!                     hardware-independent workload
//!                                |
//!                                v
//!                    +-------------------------+
//!                    | technology.rs           |
//!                    |                         |
//!                    | WHAT is this system?   |
//!                    +-------------------------+
//!                         /       |       \
//!                        /        |        \
//!                       v         v         v
//!                  capabilities topology calibration
//!                       |
//!                       v
//!                    backend
//!                       |
//!                       v
//!                    provider
//! ```
//!
//! # Important distinction
//!
//! `QuantumTechnology` is not the same thing as:
//!
//! - `BackendKind`;
//! - `QuantumExecutionModel`;
//! - `HardwareCapability`.
//!
//! For example:
//!
//! ```text
//! technology       = Superconducting
//! backend kind     = Qpu
//! execution model  = GateModel
//! capabilities     = DynamicCircuits + Measurement + Reset + ...
//! ```
//!
//! Another target can be:
//!
//! ```text
//! technology       = NeutralAtom
//! backend kind     = Qpu
//! execution model  = Analog
//! ```
//!
//! This separation is necessary for Zamani to support heterogeneous
//! quantum hardware without forcing every system into a qubit gate model.
//!
//! # Supported technology families
//!
//! The model intentionally includes:
//!
//! - superconducting;
//! - trapped ion;
//! - neutral atom;
//! - photonic;
//! - semiconductor/spin;
//! - topological;
//! - quantum dots;
//! - donor/spin systems;
//! - color-center systems;
//! - molecular/chemical platforms;
//! - bosonic systems;
//! - continuous-variable systems;
//! - quantum annealing;
//! - analog quantum computing;
//! - gate-model systems not fitting a named technology;
//! - distributed/networked quantum systems;
//! - hybrid quantum systems;
//! - simulators;
//! - hardware emulators;
//! - logical/fault-tolerant systems;
//! - research/custom technologies.
//!
//! The enum is intentionally descriptive rather than tied to a vendor.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021.
//!
//! No nightly features are required.
//!
//! # Stability
//!
//! This module is intended to be a foundational public API.
//! Later hardware modules must adapt to this vocabulary rather than
//! redefining it.

use std::fmt;
use std::str::FromStr;

// =============================================================================
// Quantum technology
// =============================================================================

/// Physical or computational technology used to realize a quantum target.
///
/// This describes the underlying computational substrate, not the provider,
/// backend lifecycle, or execution protocol.
///
/// The enum is intentionally broad enough to represent current and future
/// quantum architectures without requiring a redesign of the hardware layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum QuantumTechnology {
    /// Superconducting quantum circuits.
    Superconducting,

    /// Trapped-ion quantum processors.
    TrappedIon,

    /// Neutral-atom quantum processors.
    NeutralAtom,

    /// Photonic quantum processors.
    Photonic,

    /// Semiconductor spin-based quantum processors.
    Spin,

    /// Quantum-dot based quantum processors.
    QuantumDot,

    /// Donor-spin quantum processors.
    DonorSpin,

    /// Color-center based quantum processors, including defect-center
    /// architectures.
    ColorCenter,

    /// Topological quantum computing architectures.
    Topological,

    /// Bosonic quantum computing architectures.
    Bosonic,

    /// Continuous-variable quantum computing architectures.
    ContinuousVariable,

    /// Molecular, chemical, or other matter-based quantum architectures.
    Molecular,

    /// Quantum annealing hardware.
    Annealing,

    /// Analog quantum processors.
    Analog,

    /// Gate-model technology not represented by one of the named physical
    /// technology families.
    GateModelOther,

    /// Distributed/networked quantum computing hardware.
    Distributed,

    /// Hybrid quantum systems combining multiple physical technologies.
    Hybrid,

    /// Software-only quantum simulator.
    Simulator,

    /// Software emulator intended to model a particular hardware target.
    Emulator,

    /// Logical/fault-tolerant quantum computing target.
    Logical,

    /// Research, experimental, or otherwise custom quantum technology.
    Other,
}

impl QuantumTechnology {
    /// Returns the canonical stable identifier.
    ///
    /// These identifiers are intended for:
    ///
    /// - configuration;
    /// - serialization;
    /// - manifests;
    /// - registry queries;
    /// - compatibility reports;
    /// - telemetry labels.
    ///
    /// They must remain stable across Rust releases and provider adapters.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Superconducting => "superconducting",
            Self::TrappedIon => "trapped_ion",
            Self::NeutralAtom => "neutral_atom",
            Self::Photonic => "photonic",
            Self::Spin => "spin",
            Self::QuantumDot => "quantum_dot",
            Self::DonorSpin => "donor_spin",
            Self::ColorCenter => "color_center",
            Self::Topological => "topological",
            Self::Bosonic => "bosonic",
            Self::ContinuousVariable => "continuous_variable",
            Self::Molecular => "molecular",
            Self::Annealing => "annealing",
            Self::Analog => "analog",
            Self::GateModelOther => "gate_model_other",
            Self::Distributed => "distributed",
            Self::Hybrid => "hybrid",
            Self::Simulator => "simulator",
            Self::Emulator => "emulator",
            Self::Logical => "logical",
            Self::Other => "other",
        }
    }

    /// Returns a stable human-readable name.
    pub const fn display_name(self) -> &'static str {
        match self {
            Self::Superconducting => "Superconducting",
            Self::TrappedIon => "Trapped Ion",
            Self::NeutralAtom => "Neutral Atom",
            Self::Photonic => "Photonic",
            Self::Spin => "Spin / Semiconductor",
            Self::QuantumDot => "Quantum Dot",
            Self::DonorSpin => "Donor Spin",
            Self::ColorCenter => "Color Center",
            Self::Topological => "Topological",
            Self::Bosonic => "Bosonic",
            Self::ContinuousVariable => "Continuous Variable",
            Self::Molecular => "Molecular",
            Self::Annealing => "Quantum Annealing",
            Self::Analog => "Analog Quantum",
            Self::GateModelOther => "Other Gate Model",
            Self::Distributed => "Distributed Quantum",
            Self::Hybrid => "Hybrid Quantum",
            Self::Simulator => "Quantum Simulator",
            Self::Emulator => "Quantum Hardware Emulator",
            Self::Logical => "Logical / Fault-Tolerant",
            Self::Other => "Other Quantum Technology",
        }
    }

    /// Returns whether this technology normally represents physical hardware.
    ///
    /// `Simulator` and `Emulator` are software representations and therefore
    /// return `false`.
    ///
    /// `Logical` intentionally returns `true`: a logical target can represent
    /// a real fault-tolerant quantum system even though its computational
    /// resources are logical rather than raw physical qubits.
    pub const fn is_physical(self) -> bool {
        !matches!(self, Self::Simulator | Self::Emulator)
    }

    /// Returns whether this is software-only.
    pub const fn is_software(self) -> bool {
        matches!(self, Self::Simulator | Self::Emulator)
    }

    /// Returns whether the technology is inherently gate-model oriented.
    ///
    /// This is a classification helper, not a capability guarantee.
    ///
    /// A technology returning `true` is NOT proof that a particular backend
    /// supports a particular gate or instruction.
    pub const fn is_gate_model_family(self) -> bool {
        matches!(
            self,
            Self::Superconducting
                | Self::TrappedIon
                | Self::NeutralAtom
                | Self::Spin
                | Self::QuantumDot
                | Self::DonorSpin
                | Self::ColorCenter
                | Self::Topological
                | Self::GateModelOther
                | Self::Simulator
                | Self::Emulator
                | Self::Logical
        )
    }

    /// Returns whether the technology is primarily analog-oriented.
    pub const fn is_analog_family(self) -> bool {
        matches!(
            self,
            Self::NeutralAtom | Self::Analog
        )
    }

    /// Returns whether the technology is primarily annealing-oriented.
    pub const fn is_annealing(self) -> bool {
        matches!(self, Self::Annealing)
    }

    /// Returns whether the technology is photonic or bosonic.
    pub const fn is_photonic_or_bosonic(self) -> bool {
        matches!(self, Self::Photonic | Self::Bosonic)
    }

    /// Returns whether the technology naturally supports a continuous-variable
    /// computational model.
    pub const fn is_continuous_variable_family(self) -> bool {
        matches!(
            self,
            Self::Photonic
                | Self::Bosonic
                | Self::ContinuousVariable
        )
    }

    /// Returns whether the technology represents distributed quantum
    /// computation as its primary architectural model.
    pub const fn is_distributed(self) -> bool {
        matches!(self, Self::Distributed)
    }

    /// Returns whether the technology combines multiple technologies.
    pub const fn is_hybrid(self) -> bool {
        matches!(self, Self::Hybrid)
    }

    /// Returns whether this technology represents a logical/fault-tolerant
    /// computational layer.
    pub const fn is_logical(self) -> bool {
        matches!(self, Self::Logical)
    }

    /// Returns whether this technology is an abstract/non-physical target.
    ///
    /// This is useful for registry and discovery code.
    pub const fn is_virtual(self) -> bool {
        matches!(self, Self::Simulator | Self::Emulator)
    }

    /// Returns the technology family.
    ///
    /// The family is intentionally coarse and is useful for compatibility
    /// filtering without pretending that two technologies are equivalent.
    pub const fn family(self) -> TechnologyFamily {
        match self {
            Self::Superconducting
            | Self::TrappedIon
            | Self::NeutralAtom
            | Self::Spin
            | Self::QuantumDot
            | Self::DonorSpin
            | Self::ColorCenter
            | Self::Topological
            | Self::GateModelOther => TechnologyFamily::GateModel,

            Self::Photonic
            | Self::Bosonic
            | Self::ContinuousVariable => TechnologyFamily::PhotonicAndBosonic,

            Self::Annealing => TechnologyFamily::Annealing,

            Self::Analog => TechnologyFamily::Analog,

            Self::Distributed => TechnologyFamily::Distributed,

            Self::Hybrid => TechnologyFamily::Hybrid,

            Self::Simulator => TechnologyFamily::Simulator,

            Self::Emulator => TechnologyFamily::Emulator,

            Self::Logical => TechnologyFamily::Logical,

            Self::Molecular => TechnologyFamily::Molecular,

            Self::Other => TechnologyFamily::Other,
        }
    }

    /// Parses a canonical identifier or a supported human-readable alias.
    ///
    /// Parsing is deliberately case-insensitive and accepts `-` and `_`
    /// variants where doing so is unambiguous.
    ///
    /// Unknown values return `TechnologyParseError::UnknownTechnology`.
    pub fn parse(value: &str) -> Result<Self, TechnologyParseError> {
        let normalized = normalize_identifier(value)?;

        match normalized.as_str() {
            "superconducting" | "superconductors" => {
                Ok(Self::Superconducting)
            }

            "trapped_ion" | "trappedion" | "ion_trap" | "iontrap" => {
                Ok(Self::TrappedIon)
            }

            "neutral_atom" | "neutralatom" | "neutral_atoms" => {
                Ok(Self::NeutralAtom)
            }

            "photonic" | "photon" | "photons" => {
                Ok(Self::Photonic)
            }

            "spin" | "spin_qubit" | "spin_qubits" | "semiconductor" => {
                Ok(Self::Spin)
            }

            "quantum_dot" | "quantumdot" | "quantum_dots" => {
                Ok(Self::QuantumDot)
            }

            "donor_spin" | "donorspin" | "donor" => {
                Ok(Self::DonorSpin)
            }

            "color_center" | "colorcenter" | "color_centers" => {
                Ok(Self::ColorCenter)
            }

            "topological" | "topological_quantum" => {
                Ok(Self::Topological)
            }

            "bosonic" | "boson" | "bosonic_quantum" => {
                Ok(Self::Bosonic)
            }

            "continuous_variable"
            | "continuousvariable"
            | "continuous_variable_quantum"
            | "cv" => Ok(Self::ContinuousVariable),

            "molecular" | "chemical" | "molecular_quantum" => {
                Ok(Self::Molecular)
            }

            "annealing" | "quantum_annealing" | "quantumannealing" => {
                Ok(Self::Annealing)
            }

            "analog" | "analog_quantum" | "analog_quantum_computing" => {
                Ok(Self::Analog)
            }

            "gate_model_other" | "gate_model" | "other_gate_model" => {
                Ok(Self::GateModelOther)
            }

            "distributed"
            | "distributed_quantum"
            | "quantum_network"
            | "quantum_networked" => Ok(Self::Distributed),

            "hybrid" | "hybrid_quantum" => Ok(Self::Hybrid),

            "simulator" | "simulation" | "quantum_simulator" => {
                Ok(Self::Simulator)
            }

            "emulator" | "quantum_emulator" | "hardware_emulator" => {
                Ok(Self::Emulator)
            }

            "logical"
            | "logical_quantum"
            | "fault_tolerant"
            | "fault_tolerant_quantum"
            | "ftqc" => Ok(Self::Logical),

            "other" | "custom" | "research" | "experimental" => {
                Ok(Self::Other)
            }

            _ => Err(TechnologyParseError::UnknownTechnology {
                value: value.trim().to_owned(),
            }),
        }
    }
}

impl Default for QuantumTechnology {
    /// The default is deliberately a software simulator.
    ///
    /// This is safer than defaulting to physical hardware because a default
    /// technology must never imply that execution against a real QPU is
    /// intended.
    fn default() -> Self {
        Self::Simulator
    }
}

impl fmt::Display for QuantumTechnology {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for QuantumTechnology {
    type Err = TechnologyParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

// =============================================================================
// Technology families
// =============================================================================

/// Coarse-grained technology classification.
///
/// This is intentionally less specific than [`QuantumTechnology`].
///
/// It exists so callers can ask broad questions such as "is this an annealer?"
/// without treating individual physical technologies as interchangeable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TechnologyFamily {
    /// Digital gate-model quantum processors.
    GateModel,

    /// Photonic, bosonic, or continuous-variable systems.
    PhotonicAndBosonic,

    /// Analog quantum processors.
    Analog,

    /// Quantum annealers.
    Annealing,

    /// Distributed/networked systems.
    Distributed,

    /// Multi-technology systems.
    Hybrid,

    /// Software simulators.
    Simulator,

    /// Hardware-specific software emulators.
    Emulator,

    /// Logical/fault-tolerant systems.
    Logical,

    /// Molecular/chemical quantum systems.
    Molecular,

    /// Unclassified/custom technologies.
    Other,
}

impl TechnologyFamily {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GateModel => "gate_model",
            Self::PhotonicAndBosonic => "photonic_and_bosonic",
            Self::Analog => "analog",
            Self::Annealing => "annealing",
            Self::Distributed => "distributed",
            Self::Hybrid => "hybrid",
            Self::Simulator => "simulator",
            Self::Emulator => "emulator",
            Self::Logical => "logical",
            Self::Molecular => "molecular",
            Self::Other => "other",
        }
    }

    /// Human-readable name.
    pub const fn display_name(self) -> &'static str {
        match self {
            Self::GateModel => "Gate Model",
            Self::PhotonicAndBosonic => "Photonic / Bosonic",
            Self::Analog => "Analog",
            Self::Annealing => "Annealing",
            Self::Distributed => "Distributed",
            Self::Hybrid => "Hybrid",
            Self::Simulator => "Simulator",
            Self::Emulator => "Emulator",
            Self::Logical => "Logical / Fault-Tolerant",
            Self::Molecular => "Molecular",
            Self::Other => "Other",
        }
    }
}

impl fmt::Display for TechnologyFamily {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// =============================================================================
// Quantum execution model
// =============================================================================

/// Computational execution model independent of the physical technology.
///
/// A physical technology may support more than one execution model. Therefore
/// this enum must never be treated as a one-to-one mapping to
/// `QuantumTechnology`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum QuantumExecutionModel {
    /// Digital gate/circuit execution.
    GateModel,

    /// Dynamic circuit execution with runtime classical feedback.
    DynamicCircuit,

    /// Pulse-level control and waveform execution.
    Pulse,

    /// Analog Hamiltonian or continuous-time evolution.
    Analog,

    /// Quantum annealing / adiabatic optimization.
    Annealing,

    /// Sampling-oriented execution.
    Sampling,

    /// Logical/fault-tolerant execution.
    Logical,

    /// Distributed/networked quantum execution.
    Distributed,

    /// Continuous-variable execution.
    ContinuousVariable,

    /// Bosonic-mode execution.
    Bosonic,

    /// Hybrid quantum/classical execution.
    Hybrid,
}

impl QuantumExecutionModel {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GateModel => "gate_model",
            Self::DynamicCircuit => "dynamic_circuit",
            Self::Pulse => "pulse",
            Self::Analog => "analog",
            Self::Annealing => "annealing",
            Self::Sampling => "sampling",
            Self::Logical => "logical",
            Self::Distributed => "distributed",
            Self::ContinuousVariable => "continuous_variable",
            Self::Bosonic => "bosonic",
            Self::Hybrid => "hybrid",
        }
    }

    /// Human-readable name.
    pub const fn display_name(self) -> &'static str {
        match self {
            Self::GateModel => "Gate Model",
            Self::DynamicCircuit => "Dynamic Circuit",
            Self::Pulse => "Pulse Level",
            Self::Analog => "Analog",
            Self::Annealing => "Annealing",
            Self::Sampling => "Sampling",
            Self::Logical => "Logical / Fault-Tolerant",
            Self::Distributed => "Distributed",
            Self::ContinuousVariable => "Continuous Variable",
            Self::Bosonic => "Bosonic",
            Self::Hybrid => "Hybrid",
        }
    }

    /// Returns whether the model fundamentally describes a circuit.
    ///
    /// Dynamic circuits are included because they remain circuit programs
    /// with runtime classical control.
    pub const fn is_circuit_model(self) -> bool {
        matches!(
            self,
            Self::GateModel
                | Self::DynamicCircuit
                | Self::Logical
        )
    }

    /// Returns whether this model is pulse-level.
    pub const fn is_pulse(self) -> bool {
        matches!(self, Self::Pulse)
    }

    /// Returns whether this model is analog.
    pub const fn is_analog(self) -> bool {
        matches!(self, Self::Analog)
    }

    /// Returns whether this model is annealing-based.
    pub const fn is_annealing(self) -> bool {
        matches!(self, Self::Annealing)
    }

    /// Returns whether this model is sampling-oriented.
    pub const fn is_sampling(self) -> bool {
        matches!(self, Self::Sampling)
    }

    /// Returns whether this model is logical/fault-tolerant.
    pub const fn is_logical(self) -> bool {
        matches!(self, Self::Logical)
    }

    /// Returns whether this model requires a distributed quantum resource.
    pub const fn is_distributed(self) -> bool {
        matches!(self, Self::Distributed)
    }

    /// Returns whether this model is a continuous-variable model.
    pub const fn is_continuous_variable(self) -> bool {
        matches!(self, Self::ContinuousVariable)
    }

    /// Returns whether this model is bosonic.
    pub const fn is_bosonic(self) -> bool {
        matches!(self, Self::Bosonic)
    }

    /// Returns whether this model explicitly combines quantum and classical
    /// execution semantics.
    pub const fn is_hybrid(self) -> bool {
        matches!(self, Self::Hybrid)
    }

    /// Returns whether the model can be represented by a conventional
    /// gate/circuit intermediate representation.
    ///
    /// This is a classification aid only. It is not a backend capability test.
    pub const fn has_circuit_representation(self) -> bool {
        matches!(
            self,
            Self::GateModel
                | Self::DynamicCircuit
                | Self::Logical
                | Self::Hybrid
        )
    }
}

impl fmt::Display for QuantumExecutionModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// =============================================================================
// Physical encoding
// =============================================================================

/// Primary physical information carrier used by a quantum technology.
///
/// This is intentionally independent from the number of logical/physical
/// qubits exposed by a backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum QuantumEncoding {
    /// Two-level qubit encoding.
    Qubit,

    /// Multi-level discrete quantum system.
    Qudit,

    /// Photonic mode.
    PhotonicMode,

    /// Bosonic mode.
    BosonicMode,

    /// Continuous-variable degree of freedom.
    ContinuousVariable,

    /// Annealing spin/Ising variable.
    IsingVariable,

    /// QUBO variable.
    QuboVariable,

    /// Logical qubit encoded across physical resources.
    LogicalQubit,

    /// Multiple encoding types are simultaneously exposed.
    Hybrid,

    /// Provider/research-specific encoding.
    Other,
}

impl QuantumEncoding {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Qubit => "qubit",
            Self::Qudit => "qudit",
            Self::PhotonicMode => "photonic_mode",
            Self::BosonicMode => "bosonic_mode",
            Self::ContinuousVariable => "continuous_variable",
            Self::IsingVariable => "ising_variable",
            Self::QuboVariable => "qubo_variable",
            Self::LogicalQubit => "logical_qubit",
            Self::Hybrid => "hybrid",
            Self::Other => "other",
        }
    }

    /// Returns whether the encoding is fundamentally a qubit encoding.
    pub const fn is_qubit(self) -> bool {
        matches!(self, Self::Qubit | Self::LogicalQubit)
    }

    /// Returns whether the encoding is multi-level.
    pub const fn is_qudit(self) -> bool {
        matches!(self, Self::Qudit)
    }

    /// Returns whether the encoding represents a mode rather than a discrete
    /// qubit/qudit.
    pub const fn is_mode(self) -> bool {
        matches!(
            self,
            Self::PhotonicMode | Self::BosonicMode
        )
    }

    /// Returns whether the encoding is continuous-variable.
    pub const fn is_continuous(self) -> bool {
        matches!(self, Self::ContinuousVariable)
    }
}

impl fmt::Display for QuantumEncoding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// =============================================================================
// Technology classification descriptor
// =============================================================================

/// Immutable classification facts for a quantum technology.
///
/// This type deliberately contains only stable semantic classification.
/// Dynamic backend facts such as calibration, availability, topology and
/// capabilities do not belong here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TechnologyDescriptor {
    /// Technology identity.
    pub technology: QuantumTechnology,

    /// Coarse family.
    pub family: TechnologyFamily,

    /// Default/primary computational encoding.
    pub encoding: QuantumEncoding,

    /// Whether the technology is physical.
    pub physical: bool,

    /// Whether the technology is software-only.
    pub software: bool,

    /// Whether the technology is naturally gate-model oriented.
    pub gate_model_family: bool,
}

impl TechnologyDescriptor {
    /// Constructs the canonical descriptor for a technology.
    pub const fn for_technology(
        technology: QuantumTechnology,
    ) -> Self {
        Self {
            technology,
            family: technology.family(),
            encoding: default_encoding(technology),
            physical: technology.is_physical(),
            software: technology.is_software(),
            gate_model_family: technology.is_gate_model_family(),
        }
    }
}

impl From<QuantumTechnology> for TechnologyDescriptor {
    fn from(value: QuantumTechnology) -> Self {
        Self::for_technology(value)
    }
}

// =============================================================================
// Technology parsing errors
// =============================================================================

/// Errors returned while parsing a quantum technology identifier.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TechnologyParseError {
    /// Input contained no meaningful characters.
    Empty,

    /// Input contains an unsupported control character.
    InvalidCharacter {
        character: char,
    },

    /// Input is not a known technology identifier.
    UnknownTechnology {
        value: String,
    },
}

impl fmt::Display for TechnologyParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => {
                write!(f, "quantum technology identifier cannot be empty")
            }

            Self::InvalidCharacter { character } => {
                write!(
                    f,
                    "quantum technology identifier contains invalid \
                     control character U+{:04X}",
                    *character as u32
                )
            }

            Self::UnknownTechnology { value } => {
                write!(
                    f,
                    "unknown quantum technology '{}'",
                    value
                )
            }
        }
    }
}

impl std::error::Error for TechnologyParseError {}

// =============================================================================
// Internal helpers
// =============================================================================

/// Normalizes an external technology identifier.
///
/// Rules:
///
/// - trims surrounding whitespace;
/// - rejects control characters;
/// - converts ASCII uppercase to lowercase;
/// - converts hyphens and spaces to underscores;
/// - preserves already canonical underscores;
/// - rejects an empty result.
///
/// This deliberately does not perform fuzzy matching.
fn normalize_identifier(
    value: &str,
) -> Result<String, TechnologyParseError> {
    let trimmed = value.trim();

    if trimmed.is_empty() {
        return Err(TechnologyParseError::Empty);
    }

    let mut result = String::with_capacity(trimmed.len());

    let mut previous_was_separator = false;

    for character in trimmed.chars() {
        if character.is_control() {
            return Err(TechnologyParseError::InvalidCharacter {
                character,
            });
        }

        let normalized = match character {
            '-' | ' ' | '\t' | '\n' | '\r' => '_',
            _ => character.to_ascii_lowercase(),
        };

        if normalized == '_' {
            if previous_was_separator {
                continue;
            }

            previous_was_separator = true;
        } else {
            previous_was_separator = false;
        }

        result.push(normalized);
    }

    while result.ends_with('_') {
        result.pop();
    }

    if result.is_empty() {
        return Err(TechnologyParseError::Empty);
    }

    Ok(result)
}

/// Returns a conservative default encoding for a technology.
///
/// This is a semantic classification, not a backend capability claim.
const fn default_encoding(
    technology: QuantumTechnology,
) -> QuantumEncoding {
    match technology {
        QuantumTechnology::Superconducting
        | QuantumTechnology::TrappedIon
        | QuantumTechnology::NeutralAtom
        | QuantumTechnology::Spin
        | QuantumTechnology::QuantumDot
        | QuantumTechnology::DonorSpin
        | QuantumTechnology::ColorCenter
        | QuantumTechnology::Topological
        | QuantumTechnology::GateModelOther
        | QuantumTechnology::Simulator
        | QuantumTechnology::Emulator => QuantumEncoding::Qubit,

        QuantumTechnology::Photonic => {
            QuantumEncoding::PhotonicMode
        }

        QuantumTechnology::Bosonic => {
            QuantumEncoding::BosonicMode
        }

        QuantumTechnology::ContinuousVariable => {
            QuantumEncoding::ContinuousVariable
        }

        QuantumTechnology::Molecular => QuantumEncoding::Qudit,

        QuantumTechnology::Annealing => {
            QuantumEncoding::IsingVariable
        }

        QuantumTechnology::Analog => QuantumEncoding::Qubit,

        QuantumTechnology::Distributed => {
            QuantumEncoding::Hybrid
        }

        QuantumTechnology::Hybrid => QuantumEncoding::Hybrid,

        QuantumTechnology::Logical => {
            QuantumEncoding::LogicalQubit
        }

        QuantumTechnology::Other => QuantumEncoding::Other,
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_identifiers_are_stable() {
        assert_eq!(
            QuantumTechnology::Superconducting.as_str(),
            "superconducting"
        );

        assert_eq!(
            QuantumTechnology::TrappedIon.as_str(),
            "trapped_ion"
        );

        assert_eq!(
            QuantumTechnology::ContinuousVariable.as_str(),
            "continuous_variable"
        );

        assert_eq!(
            QuantumTechnology::QuantumDot.as_str(),
            "quantum_dot"
        );
    }

    #[test]
    fn display_names_are_human_readable() {
        assert_eq!(
            QuantumTechnology::TrappedIon.display_name(),
            "Trapped Ion"
        );

        assert_eq!(
            QuantumTechnology::NeutralAtom.display_name(),
            "Neutral Atom"
        );
    }

    #[test]
    fn technology_parsing_is_case_insensitive() {
        assert_eq!(
            QuantumTechnology::parse("Superconducting").unwrap(),
            QuantumTechnology::Superconducting
        );

        assert_eq!(
            QuantumTechnology::parse("TRAPPED-ION").unwrap(),
            QuantumTechnology::TrappedIon
        );

        assert_eq!(
            QuantumTechnology::parse("neutral atom").unwrap(),
            QuantumTechnology::NeutralAtom
        );
    }

    #[test]
    fn technology_parsing_accepts_known_aliases() {
        assert_eq!(
            QuantumTechnology::parse("qpu spin").unwrap(),
            QuantumTechnology::Spin
        );

        assert_eq!(
            QuantumTechnology::parse("CV").unwrap(),
            QuantumTechnology::ContinuousVariable
        );

        assert_eq!(
            QuantumTechnology::parse("FTQC").unwrap(),
            QuantumTechnology::Logical
        );
    }

    #[test]
    fn technology_parsing_rejects_unknown_values() {
        assert_eq!(
            QuantumTechnology::parse("not_a_real_quantum_technology"),
            Err(TechnologyParseError::UnknownTechnology {
                value: "not_a_real_quantum_technology".to_owned(),
            })
        );
    }

    #[test]
    fn technology_parsing_rejects_empty_values() {
        assert_eq!(
            QuantumTechnology::parse("   "),
            Err(TechnologyParseError::Empty)
        );
    }

    #[test]
    fn technology_parsing_rejects_control_characters() {
        assert_eq!(
            QuantumTechnology::parse("superconducting\nqpu"),
            Err(TechnologyParseError::InvalidCharacter {
                character: '\n',
            })
        );
    }

    #[test]
    fn physical_classification_is_correct() {
        assert!(QuantumTechnology::Superconducting.is_physical());
        assert!(QuantumTechnology::TrappedIon.is_physical());
        assert!(QuantumTechnology::Logical.is_physical());

        assert!(!QuantumTechnology::Simulator.is_physical());
        assert!(!QuantumTechnology::Emulator.is_physical());
    }

    #[test]
    fn software_classification_is_correct() {
        assert!(QuantumTechnology::Simulator.is_software());
        assert!(QuantumTechnology::Emulator.is_software());

        assert!(!QuantumTechnology::Superconducting.is_software());
    }

    #[test]
    fn gate_model_classification_is_not_a_capability_claim() {
        assert!(
            QuantumTechnology::Superconducting
                .is_gate_model_family()
        );

        assert!(
            QuantumTechnology::TrappedIon
                .is_gate_model_family()
        );

        assert!(
            QuantumTechnology::Simulator
                .is_gate_model_family()
        );

        assert!(
            QuantumTechnology::Logical
                .is_gate_model_family()
        );

        assert!(
            !QuantumTechnology::Annealing
                .is_gate_model_family()
        );
    }

    #[test]
    fn family_classification_is_deterministic() {
        assert_eq!(
            QuantumTechnology::Superconducting.family(),
            TechnologyFamily::GateModel
        );

        assert_eq!(
            QuantumTechnology::Photonic.family(),
            TechnologyFamily::PhotonicAndBosonic
        );

        assert_eq!(
            QuantumTechnology::Annealing.family(),
            TechnologyFamily::Annealing
        );

        assert_eq!(
            QuantumTechnology::Analog.family(),
            TechnologyFamily::Analog
        );

        assert_eq!(
            QuantumTechnology::Distributed.family(),
            TechnologyFamily::Distributed
        );

        assert_eq!(
            QuantumTechnology::Logical.family(),
            TechnologyFamily::Logical
        );
    }

    #[test]
    fn execution_model_identifiers_are_stable() {
        assert_eq!(
            QuantumExecutionModel::GateModel.as_str(),
            "gate_model"
        );

        assert_eq!(
            QuantumExecutionModel::DynamicCircuit.as_str(),
            "dynamic_circuit"
        );

        assert_eq!(
            QuantumExecutionModel::ContinuousVariable.as_str(),
            "continuous_variable"
        );
    }

    #[test]
    fn execution_model_classification_is_correct() {
        assert!(
            QuantumExecutionModel::GateModel
                .is_circuit_model()
        );

        assert!(
            QuantumExecutionModel::DynamicCircuit
                .is_circuit_model()
        );

        assert!(
            QuantumExecutionModel::Logical
                .is_circuit_model()
        );

        assert!(
            !QuantumExecutionModel::Pulse
                .is_circuit_model()
        );

        assert!(
            QuantumExecutionModel::Pulse.is_pulse()
        );

        assert!(
            QuantumExecutionModel::Analog.is_analog()
        );

        assert!(
            QuantumExecutionModel::Annealing.is_annealing()
        );

        assert!(
            QuantumExecutionModel::Distributed.is_distributed()
        );
    }

    #[test]
    fn encoding_classification_is_correct() {
        assert!(
            QuantumEncoding::Qubit.is_qubit()
        );

        assert!(
            QuantumEncoding::LogicalQubit.is_qubit()
        );

        assert!(
            QuantumEncoding::Qudit.is_qudit()
        );

        assert!(
            QuantumEncoding::PhotonicMode.is_mode()
        );

        assert!(
            QuantumEncoding::BosonicMode.is_mode()
        );

        assert!(
            QuantumEncoding::ContinuousVariable.is_continuous()
        );
    }

    #[test]
    fn descriptors_are_derived_from_authoritative_technology() {
        let descriptor =
            TechnologyDescriptor::for_technology(
                QuantumTechnology::Superconducting,
            );

        assert_eq!(
            descriptor.technology,
            QuantumTechnology::Superconducting
        );

        assert_eq!(
            descriptor.family,
            TechnologyFamily::GateModel
        );

        assert_eq!(
            descriptor.encoding,
            QuantumEncoding::Qubit
        );

        assert!(descriptor.physical);
        assert!(!descriptor.software);
        assert!(descriptor.gate_model_family);
    }

    #[test]
    fn simulator_descriptor_is_safe_by_default() {
        let descriptor =
            TechnologyDescriptor::for_technology(
                QuantumTechnology::Simulator,
            );

        assert_eq!(
            descriptor.encoding,
            QuantumEncoding::Qubit
        );

        assert!(!descriptor.physical);
        assert!(descriptor.software);
    }

    #[test]
    fn logical_descriptor_is_not_treated_as_software() {
        let descriptor =
            TechnologyDescriptor::for_technology(
                QuantumTechnology::Logical,
            );

        assert_eq!(
            descriptor.encoding,
            QuantumEncoding::LogicalQubit
        );

        assert!(descriptor.physical);
        assert!(!descriptor.software);
    }

    #[test]
    fn default_technology_is_simulator() {
        assert_eq!(
            QuantumTechnology::default(),
            QuantumTechnology::Simulator
        );
    }

    #[test]
    fn technology_ordering_is_deterministic() {
        let mut technologies = vec![
            QuantumTechnology::Photonic,
            QuantumTechnology::Superconducting,
            QuantumTechnology::Annealing,
        ];

        technologies.sort();

        assert_eq!(
            technologies,
            vec![
                QuantumTechnology::Superconducting,
                QuantumTechnology::Photonic,
                QuantumTechnology::Annealing,
            ]
        );
    }

    #[test]
    fn displays_use_canonical_identifiers() {
        assert_eq!(
            QuantumTechnology::Superconducting.to_string(),
            "superconducting"
        );

        assert_eq!(
            QuantumExecutionModel::DynamicCircuit.to_string(),
            "dynamic_circuit"
        );

        assert_eq!(
            QuantumEncoding::PhotonicMode.to_string(),
            "photonic_mode"
        );

        assert_eq!(
            TechnologyFamily::GateModel.to_string(),
            "gate_model"
        );
    }
}
//! Zamani Quantum Frontend — OpenQASM exporter.
//!
//! Production OpenQASM 3.0/3.1 exporter from the canonical Zamani Quantum IR.
//!
//! # Architectural boundary
//!
//! ```text
//! Canonical Quantum IR
//!        │
//!        ▼
//! generic frontend exporter
//!        │
//!        ▼
//! OpenQASM exporter
//!        │
//!        ├── representability validation
//!        ├── deterministic serialization
//!        └── bounded artifact construction
//!        │
//!        ▼
//! valid OpenQASM 3.x
//! ```
//!
//! This module is deliberately OpenQASM-specific.
//!
//! It MUST NOT:
//!
//! - define a second Quantum IR;
//! - mutate `QuantumCircuit`;
//! - optimize;
//! - route;
//! - schedule;
//! - map to hardware;
//! - execute a circuit;
//! - access the filesystem;
//! - access the network;
//! - spawn processes;
//! - access QPU hardware;
//! - silently discard unsupported operations;
//! - silently approximate unsupported operations;
//! - invent measurements;
//! - invent qubit operands;
//! - invent classical destinations;
//! - assume `q[i] -> c[i]` measurement mapping;
//! - depend on another frontend format.
//!
//! # Version support
//!
//! This exporter supports OpenQASM 3.0 and 3.1.
//!
//! The production constructor defaults to OpenQASM 3.1.
//!
//! The concrete exporter never changes its configured version merely because
//! a caller requests `SameMajor` compatibility. Version compatibility is
//! handled by the generic exporter contract.
//!
//! # Supported canonical IR
//!
//! Directly representable operations:
//!
//! - `I`
//! - `X`
//! - `Y`
//! - `Z`
//! - `H`
//! - `S`
//! - `Sdg`
//! - `T`
//! - `Tdg`
//! - `RX`
//! - `RY`
//! - `RZ`
//! - `Phase`
//! - `U1`
//! - `U2`
//! - `U3`
//! - `CX`
//! - `CY`
//! - `CZ`
//! - `CH`
//! - `CRX`
//! - `CRY`
//! - `CRZ`
//! - `SWAP`
//! - `CCX`
//! - `CSWAP`
//! - `Measure` when represented by a Z-basis, non-destructive measurement
//! - `Reset`
//! - `Barrier`
//!
//! Explicitly unsupported direct operations:
//!
//! - `V`
//! - `Vdg`
//! - `ISWAP`
//! - `ECR`
//!
//! The exporter does not decompose these operations. Such decomposition is a
//! downstream compiler responsibility and must not be hidden inside a format
//! serializer.
//!
//! # Measurement semantics
//!
//! OpenQASM measurement is a computational/Z-basis measurement and the
//! resulting classical value is explicitly assigned to a classical bit.
//!
//! The canonical IR measurement contains both the logical qubit and the
//! classical destination. The exporter therefore preserves those identities
//! exactly.
//!
//! If the IR explicitly requests reset-after-measurement, the exporter emits:
//!
//! ```text
//! measure q[i] -> c[j];
//! reset q[i];
//! ```
//!
//! No measurement or reset is ever invented by the exporter.
//!
//! # Parameter semantics
//!
//! Canonical IR parameters are preserved symbolically.
//!
//! The exporter never evaluates symbolic expressions because doing so could
//! change runtime/compiler semantics.
//!
//! Canonical expressions are emitted using their deterministic IR expression
//! representation.
//!
//! # Security
//!
//! Output is bounded during serialization, not after serialization.
//!
//! This is important because checking `String::len()` after constructing an
//! enormous string would still allow an attacker-controlled IR to consume
//! excessive memory.
//!
//! Every externally observable output is deterministic.
//!
//! # Rust compatibility
//!
//! Rust 2021 / Rust 1.97.1.
//!
//! No nightly features are required.
//! No additional dependencies are required.

use std::fmt::{self, Write as _};

use crate::quantum::frontend::core::errors::{
    FrontendError,
    FrontendErrorCode,
    FrontendErrorKind,
    FrontendResult,
};
use crate::quantum::frontend::exporter::{
    ExportedArtifact,
    ExportOptions,
    QuantumExporter,
};
use crate::quantum::frontend::format::{
    FormatCapabilities,
    FormatCapability,
    FormatId,
    FormatVersion,
    FrontendFormat,
};
use crate::quantum::ir::{
    Gate,
    GateKind,
    MeasurementBasis,
    MeasurementMode,
    Parameter,
    ParameterExpression,
    QuantumCircuit,
};

/// Canonical OpenQASM format identifier.
pub const OPENQASM_FORMAT_ID: &str = "openqasm";

/// OpenQASM 3.0.
pub const OPENQASM_3_0: FormatVersion =
    FormatVersion::new(3, 0, 0);

/// OpenQASM 3.1.
pub const OPENQASM_3_1: FormatVersion =
    FormatVersion::new(3, 1, 0);

/// OpenQASM standard library.
pub const STANDARD_LIBRARY_INCLUDE: &str =
    "stdgates.inc";

/// OpenQASM textual media type.
pub const OPENQASM_MEDIA_TYPE: &str =
    "text/x-openqasm";

/// Maximum symbolic identifier length accepted by this exporter.
///
/// The canonical IR currently constrains symbolic names to an ASCII-compatible
/// subset. The OpenQASM language itself permits a broader Unicode identifier
/// set, but the exporter must never silently transform an IR identifier.
const MAX_IDENTIFIER_BYTES: usize = 256;

/// Maximum generated source line.
///
/// This protects against pathological expressions, register declarations,
/// or future large-arity operations.
const MAX_OPERATION_LINE_BYTES: usize = 64 * 1024;

/// Stable exporter error code: generic format construction failure.
const CODE_FORMAT: &str = "QASM-E001";

/// Stable exporter error code: version failure.
const CODE_VERSION: &str = "QASM-E002";

/// Stable exporter error code: gate representability failure.
const CODE_GATE: &str = "QASM-E003";

/// Stable exporter error code: parameter failure.
const CODE_PARAMETER: &str = "QASM-E004";

/// Stable exporter error code: measurement failure.
const CODE_MEASUREMENT: &str = "QASM-E005";

/// Stable exporter error code: identifier failure.
const CODE_IDENTIFIER: &str = "QASM-E006";

/// Stable exporter error code: output/resource failure.
const CODE_ARTIFACT: &str = "QASM-E008";

/// Production OpenQASM exporter.
///
/// The exporter contains only immutable configuration and is therefore safe to
/// share between callers when the surrounding compiler uses it concurrently.
#[derive(Clone, Debug)]
pub struct OpenQasmExporter {
    format: FrontendFormat,
}

impl OpenQasmExporter {
    /// Creates the production OpenQASM 3.1 exporter.
    pub fn production() -> FrontendResult<Self> {
        Self::new(OPENQASM_3_1)
    }

    /// Creates an OpenQASM exporter for an explicit OpenQASM 3.x version.
    pub fn new(version: FormatVersion) -> FrontendResult<Self> {
        if version.major() != 3 {
            return Err(export_error(
                FrontendErrorKind::Unsupported,
                CODE_VERSION,
                format!(
                    "OpenQASM exporter supports only OpenQASM 3.x; \
                     requested version {version}"
                ),
            ));
        }

        if version.minor() > 1 {
            return Err(export_error(
                FrontendErrorKind::Unsupported,
                CODE_VERSION,
                format!(
                    "OpenQASM exporter supports versions 3.0 and 3.1; \
                     requested unsupported version {version}"
                ),
            ));
        }

        let id = FormatId::new(OPENQASM_FORMAT_ID)
            .map_err(|error| {
                FrontendError::internal(format!(
                    "failed to construct built-in OpenQASM \
                     format identifier: {error}"
                ))
            })?;

        let capabilities = openqasm_capabilities()?;

        Ok(Self {
            format: FrontendFormat::new(
                id,
                version,
                capabilities,
            ),
        })
    }

    /// Returns the configured OpenQASM version.
    #[must_use]
    pub const fn configured_version(&self) -> FormatVersion {
        self.format.version()
    }

    /// Returns the immutable generic format descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &FrontendFormat {
        &self.format
    }

    /// Convenience method using generic production export options.
    ///
    /// All actual validation still passes through `QuantumExporter::export`.
    pub fn export_circuit(
        &self,
        circuit: &QuantumCircuit,
    ) -> FrontendResult<ExportedArtifact> {
        self.export(
            circuit,
            &ExportOptions::default(),
        )
    }

    /// Serializes a canonical circuit using the supplied output limit.
    ///
    /// The limit is enforced while writing, rather than after a potentially
    /// unbounded allocation.
    fn serialize(
        &self,
        circuit: &QuantumCircuit,
        max_output_bytes: usize,
    ) -> FrontendResult<String> {
        if max_output_bytes == 0 {
            return Err(export_error(
                FrontendErrorKind::LimitExceeded,
                CODE_ARTIFACT,
                "OpenQASM output limit must be greater than zero",
            ));
        }

        let mut output =
            BoundedOutput::new(max_output_bytes);

        output.line(&format!(
            "OPENQASM {}.{};",
            self.format.version().major(),
            self.format.version().minor(),
        ))?;

        output.line(&format!(
            "include \"{}\";",
            STANDARD_LIBRARY_INCLUDE,
        ))?;

        /*
         * OpenQASM 3.1 permits zero-sized quantum registers.
         *
         * We therefore do not manufacture a dummy qubit for a zero-qubit
         * circuit. The canonical IR remains the source of truth.
         */
        if circuit.num_qubits() > 0 {
            output.line(&format!(
                "qubit[{}] q;",
                circuit.num_qubits(),
            ))?;
        }

        if circuit.num_classical_bits() > 0 {
            output.line(&format!(
                "bit[{}] c;",
                circuit.num_classical_bits(),
            ))?;
        }

        for gate in circuit.operations() {
            self.validate_gate_for_export(
                gate,
                circuit,
            )?;

            self.write_gate(
                &mut output,
                gate,
            )?;
        }

        Ok(output.into_string())
    }

    /// Performs all OpenQASM-specific representability checks.
    fn validate_gate_for_export(
        &self,
        gate: &Gate,
        circuit: &QuantumCircuit,
    ) -> FrontendResult<()> {
        let kind = gate.kind();

        /*
         * The canonical IR already validates these properties, but the
         * concrete format boundary checks them again so future deserialization
         * or alternate IR construction paths cannot cause invalid OpenQASM.
         */
        if !kind
            .operand_count()
            .accepts(gate.qubits().len())
        {
            return Err(export_error(
                FrontendErrorKind::Export,
                CODE_GATE,
                format!(
                    "gate {kind:?} contains {} qubit operands, \
                     but its canonical operand contract requires {}",
                    gate.qubits().len(),
                    kind.operand_count(),
                ),
            ));
        }

        if gate.parameters().len()
            != kind.parameter_count()
        {
            return Err(export_error(
                FrontendErrorKind::Export,
                CODE_PARAMETER,
                format!(
                    "gate {kind:?} contains {} parameters, \
                     but its canonical parameter contract requires {}",
                    gate.parameters().len(),
                    kind.parameter_count(),
                ),
            ));
        }

        for parameter in gate.parameters() {
            validate_parameter(parameter)?;
        }

        for qubit in gate.qubits() {
            if qubit.index() >= circuit.num_qubits() {
                return Err(export_error(
                    FrontendErrorKind::Export,
                    CODE_GATE,
                    format!(
                        "gate {kind:?} references logical qubit {} \
                         outside circuit namespace of {} qubits",
                        qubit.index(),
                        circuit.num_qubits(),
                    ),
                ));
            }
        }

        match kind {
            GateKind::Measure => {
                self.validate_measurement(
                    gate,
                    circuit,
                )?;
            }

            GateKind::V
            | GateKind::Vdg
            | GateKind::ISWAP
            | GateKind::ECR => {
                return Err(export_error(
                    FrontendErrorKind::Unsupported,
                    CODE_GATE,
                    format!(
                        "canonical gate {kind:?} has no direct \
                         semantically equivalent OpenQASM \
                         stdgates.inc operation"
                    ),
                ));
            }

            GateKind::I
            | GateKind::X
            | GateKind::Y
            | GateKind::Z
            | GateKind::H
            | GateKind::S
            | GateKind::Sdg
            | GateKind::T
            | GateKind::Tdg
            | GateKind::RX
            | GateKind::RY
            | GateKind::RZ
            | GateKind::Phase
            | GateKind::U1
            | GateKind::U2
            | GateKind::U3
            | GateKind::CX
            | GateKind::CY
            | GateKind::CZ
            | GateKind::CH
            | GateKind::CRX
            | GateKind::CRY
            | GateKind::CRZ
            | GateKind::SWAP
            | GateKind::CCX
            | GateKind::CSWAP
            | GateKind::Barrier
            | GateKind::Reset => {}
        }

        Ok(())
    }

    /// Validates the canonical measurement payload.
    fn validate_measurement(
        &self,
        gate: &Gate,
        circuit: &QuantumCircuit,
    ) -> FrontendResult<()> {
        let measurement =
            gate.measurement().ok_or_else(|| {
                export_error(
                    FrontendErrorKind::Export,
                    CODE_MEASUREMENT,
                    "measurement gate is missing its \
                     canonical Measurement payload",
                )
            })?;

        if measurement.basis()
            != MeasurementBasis::Z
        {
            return Err(export_error(
                FrontendErrorKind::Unsupported,
                CODE_MEASUREMENT,
                format!(
                    "OpenQASM direct measurement export \
                     supports only the Z basis; the IR \
                     measurement uses {} basis",
                    measurement.basis(),
                ),
            ));
        }

        if measurement.mode()
            != MeasurementMode::NonDestructive
        {
            return Err(export_error(
                FrontendErrorKind::Unsupported,
                CODE_MEASUREMENT,
                "destructive IR measurements cannot be \
                 represented directly by OpenQASM 3 \
                 measurement semantics",
            ));
        }

        let qubit =
            measurement.qubit().index();

        if qubit >= circuit.num_qubits() {
            return Err(export_error(
                FrontendErrorKind::Export,
                CODE_MEASUREMENT,
                format!(
                    "measurement references qubit {} \
                     outside circuit namespace of {} qubits",
                    qubit,
                    circuit.num_qubits(),
                ),
            ));
        }

        let classical =
            measurement.classical_bit().index();

        if classical >= circuit.num_classical_bits() {
            return Err(export_error(
                FrontendErrorKind::Export,
                CODE_MEASUREMENT,
                format!(
                    "measurement targets classical bit {} \
                     outside circuit namespace of {} bits",
                    classical,
                    circuit.num_classical_bits(),
                ),
            ));
        }

        match gate.classical_target() {
            Some(target) if target == classical => {}

            Some(target) => {
                return Err(export_error(
                    FrontendErrorKind::Export,
                    CODE_MEASUREMENT,
                    format!(
                        "measurement has inconsistent classical \
                         destinations: gate target {target} \
                         differs from measurement target {classical}"
                    ),
                ));
            }

            None => {
                return Err(export_error(
                    FrontendErrorKind::Export,
                    CODE_MEASUREMENT,
                    "measurement gate has no classical destination",
                ));
            }
        }

        Ok(())
    }

    /// Serializes one canonical IR operation.
    fn write_gate(
        &self,
        output: &mut BoundedOutput,
        gate: &Gate,
    ) -> FrontendResult<()> {
        match gate.kind() {
            GateKind::Measure => {
                let measurement =
                    gate.measurement().ok_or_else(|| {
                        export_error(
                            FrontendErrorKind::Export,
                            CODE_MEASUREMENT,
                            "measurement payload disappeared \
                             after validation",
                        )
                    })?;

                output.line(&format!(
                    "measure q[{}] -> c[{}];",
                    measurement.qubit().index(),
                    measurement
                        .classical_bit()
                        .index(),
                ))?;

                if measurement.reset_after() {
                    output.line(&format!(
                        "reset q[{}];",
                        measurement.qubit().index(),
                    ))?;
                }

                Ok(())
            }

            GateKind::Reset => {
                let qubit =
                    gate.qubits().first().ok_or_else(|| {
                        export_error(
                            FrontendErrorKind::Export,
                            CODE_GATE,
                            "reset operation has no qubit operand",
                        )
                    })?;

                output.line(&format!(
                    "reset q[{}];",
                    qubit.index(),
                ))
            }

            GateKind::Barrier => {
                let operands =
                    format_qubit_operands(gate)?;

                output.line(&format!(
                    "barrier {};",
                    operands,
                ))
            }

            kind => {
                let name =
                    gate_name(kind).ok_or_else(|| {
                        export_error(
                            FrontendErrorKind::Unsupported,
                            CODE_GATE,
                            format!(
                                "gate {kind:?} cannot be emitted \
                                 as OpenQASM 3"
                            ),
                        )
                    })?;

                let operands =
                    format_qubit_operands(gate)?;

                let parameters =
                    format_parameters(
                        gate.parameters(),
                    )?;

                if parameters.is_empty() {
                    output.line(&format!(
                        "{name} {operands};"
                    ))
                } else {
                    output.line(&format!(
                        "{name}({parameters}) {operands};"
                    ))
                }
            }
        }
    }
}

impl QuantumExporter for OpenQasmExporter {
    fn format(&self) -> &FrontendFormat {
        &self.format
    }

    fn export_impl(
        &self,
        circuit: &QuantumCircuit,
        options: &ExportOptions,
    ) -> FrontendResult<ExportedArtifact> {
        let source = self.serialize(
            circuit,
            options.max_output_bytes(),
        )?;

        ExportedArtifact::text(
            self.format.clone(),
            OPENQASM_MEDIA_TYPE,
            source,
        )
        .map_err(|error| {
            export_error(
                FrontendErrorKind::Export,
                CODE_ARTIFACT,
                error.to_string(),
            )
        })
    }
}

/// Bounded UTF-8 OpenQASM output buffer.
///
/// The important property is that output limits are enforced while data is
/// appended. This prevents an oversized intermediate `String` from being
/// constructed before the generic exporter can reject it.
struct BoundedOutput {
    value: String,
    max_bytes: usize,
}

impl BoundedOutput {
    fn new(max_bytes: usize) -> Self {
        Self {
            value: String::new(),
            max_bytes,
        }
    }

    fn line(
        &mut self,
        line: &str,
    ) -> FrontendResult<()> {
        if line.len() > MAX_OPERATION_LINE_BYTES {
            return Err(export_error(
                FrontendErrorKind::LimitExceeded,
                CODE_ARTIFACT,
                format!(
                    "generated OpenQASM line exceeds \
                     {} bytes",
                    MAX_OPERATION_LINE_BYTES,
                ),
            ));
        }

        let required = line
            .len()
            .checked_add(1)
            .ok_or_else(|| {
                export_error(
                    FrontendErrorKind::LimitExceeded,
                    CODE_ARTIFACT,
                    "OpenQASM output size calculation overflowed",
                )
            })?;

        let next = self
            .value
            .len()
            .checked_add(required)
            .ok_or_else(|| {
                export_error(
                    FrontendErrorKind::LimitExceeded,
                    CODE_ARTIFACT,
                    "OpenQASM output size calculation overflowed",
                )
            })?;

        if next > self.max_bytes {
            return Err(export_error(
                FrontendErrorKind::LimitExceeded,
                CODE_ARTIFACT,
                format!(
                    "OpenQASM output exceeds configured \
                     maximum of {} bytes",
                    self.max_bytes,
                ),
            ));
        }

        self.value.push_str(line);
        self.value.push('\n');

        Ok(())
    }

    fn into_string(self) -> String {
        self.value
    }
}

/// Constructs the OpenQASM capability declaration.
fn openqasm_capabilities()
    -> FrontendResult<FormatCapabilities>
{
    let mut capabilities =
        FormatCapabilities::new();

    let supported = [
        FormatCapability::Export,
        FormatCapability::Parameters,
        FormatCapability::Measurements,
        FormatCapability::Reset,
        FormatCapability::Barriers,
        FormatCapability::Includes,
        FormatCapability::SymbolicNames,
        FormatCapability::RegisterDeclarations,
        FormatCapability::Expressions,
    ];

    for capability in supported {
        capabilities
            .insert(capability)
            .map_err(|error| {
                FrontendError::internal(format!(
                    "failed to construct OpenQASM \
                     capability set: {error}"
                ))
            })?;
    }

    Ok(capabilities)
}

/// Maps canonical IR gates to OpenQASM standard-library names.
///
/// All mappings below are direct semantic mappings. No decomposition is
/// performed here.
fn gate_name(
    kind: GateKind,
) -> Option<&'static str> {
    match kind {
        GateKind::I => Some("id"),
        GateKind::X => Some("x"),
        GateKind::Y => Some("y"),
        GateKind::Z => Some("z"),
        GateKind::H => Some("h"),
        GateKind::S => Some("s"),
        GateKind::Sdg => Some("sdg"),
        GateKind::T => Some("t"),
        GateKind::Tdg => Some("tdg"),

        GateKind::RX => Some("rx"),
        GateKind::RY => Some("ry"),
        GateKind::RZ => Some("rz"),
        GateKind::Phase => Some("p"),

        GateKind::U1 => Some("u1"),
        GateKind::U2 => Some("u2"),
        GateKind::U3 => Some("u3"),

        GateKind::CX => Some("cx"),
        GateKind::CY => Some("cy"),
        GateKind::CZ => Some("cz"),
        GateKind::CH => Some("ch"),

        GateKind::CRX => Some("crx"),
        GateKind::CRY => Some("cry"),
        GateKind::CRZ => Some("crz"),

        GateKind::SWAP => Some("swap"),

        GateKind::CCX => Some("ccx"),
        GateKind::CSWAP => Some("cswap"),

        GateKind::V
        | GateKind::Vdg
        | GateKind::ISWAP
        | GateKind::ECR
        | GateKind::Measure
        | GateKind::Barrier
        | GateKind::Reset => None,
    }
}

/// Formats logical qubit operands.
///
/// Operand ordering is preserved exactly.
fn format_qubit_operands(
    gate: &Gate,
) -> FrontendResult<String> {
    if gate.qubits().is_empty() {
        return Err(export_error(
            FrontendErrorKind::Export,
            CODE_GATE,
            format!(
                "gate {:?} has no qubit operands",
                gate.kind(),
            ),
        ));
    }

    let mut result = String::new();

    for (index, qubit) in
        gate.qubits().iter().enumerate()
    {
        if index != 0 {
            result.push_str(", ");
        }

        write!(
            &mut result,
            "q[{}]",
            qubit.index(),
        )
        .map_err(|_| {
            FrontendError::internal(
                "failed to format OpenQASM qubit operand",
            )
        })?;
    }

    if result.len() > MAX_OPERATION_LINE_BYTES {
        return Err(export_error(
            FrontendErrorKind::LimitExceeded,
            CODE_ARTIFACT,
            "OpenQASM qubit operand list exceeds \
             the per-operation line limit",
        ));
    }

    Ok(result)
}

/// Formats all canonical parameters in deterministic order.
fn format_parameters(
    parameters: &[Parameter],
) -> FrontendResult<String> {
    let mut result = String::new();

    for (index, parameter) in
        parameters.iter().enumerate()
    {
        if index != 0 {
            result.push_str(", ");
        }

        validate_parameter(parameter)?;

        append_parameter(
            &mut result,
            parameter,
        )?;
    }

    if result.len() > MAX_OPERATION_LINE_BYTES {
        return Err(export_error(
            FrontendErrorKind::LimitExceeded,
            CODE_ARTIFACT,
            "OpenQASM parameter list exceeds \
             the per-operation line limit",
        ));
    }

    Ok(result)
}

/// Appends one parameter without evaluating it.
fn append_parameter(
    output: &mut String,
    parameter: &Parameter,
) -> FrontendResult<()> {
    let before = output.len();

    write!(
        output,
        "{parameter}",
    )
    .map_err(|_| {
        FrontendError::internal(
            "failed to format OpenQASM parameter",
        )
    })?;

    if output.len()
        .saturating_sub(before)
        > MAX_OPERATION_LINE_BYTES
    {
        return Err(export_error(
            FrontendErrorKind::LimitExceeded,
            CODE_PARAMETER,
            "OpenQASM parameter representation \
             exceeds the per-operation limit",
        ));
    }

    Ok(())
}

/// Validates that a canonical IR parameter is representable.
fn validate_parameter(
    parameter: &Parameter,
) -> FrontendResult<()> {
    match parameter {
        Parameter::Constant(value) => {
            if !value.is_finite() {
                return Err(export_error(
                    FrontendErrorKind::Export,
                    CODE_PARAMETER,
                    "OpenQASM cannot represent a \
                     non-finite parameter",
                ));
            }
        }

        Parameter::Symbol(name) => {
            validate_identifier(name)?;
        }

        Parameter::Expression(expression) => {
            validate_expression(expression)?;
        }
    }

    Ok(())
}

/// Recursively validates an IR parameter expression.
///
/// The canonical IR already enforces expression-depth limits, so this
/// traversal is a validation of representability rather than an independent
/// recursive resource model.
fn validate_expression(
    expression: &ParameterExpression,
) -> FrontendResult<()> {
    match expression {
        ParameterExpression::Add(
            left,
            right,
        )
        | ParameterExpression::Subtract(
            left,
            right,
        )
        | ParameterExpression::Multiply(
            left,
            right,
        )
        | ParameterExpression::Divide(
            left,
            right,
        ) => {
            validate_parameter(left)?;
            validate_parameter(right)?;
        }

        ParameterExpression::Negate(value) => {
            validate_parameter(value)?;
        }
    }

    Ok(())
}

/// Validates the ASCII identifier subset emitted by this exporter.
///
/// OpenQASM 3.1 itself permits Unicode identifiers. The canonical IR's
/// parameter symbols are currently intended for an ASCII-compatible symbol
/// vocabulary, so this exporter refuses characters it cannot guarantee to
/// preserve exactly.
fn validate_identifier(
    identifier: &str,
) -> FrontendResult<()> {
    if identifier.is_empty() {
        return Err(export_error(
            FrontendErrorKind::Export,
            CODE_IDENTIFIER,
            "OpenQASM identifier must not be empty",
        ));
    }

    if identifier.len()
        > MAX_IDENTIFIER_BYTES
    {
        return Err(export_error(
            FrontendErrorKind::LimitExceeded,
            CODE_IDENTIFIER,
            format!(
                "OpenQASM identifier exceeds {} bytes",
                MAX_IDENTIFIER_BYTES,
            ),
        ));
    }

    let mut characters =
        identifier.chars();

    let first =
        characters.next().ok_or_else(|| {
            export_error(
                FrontendErrorKind::Export,
                CODE_IDENTIFIER,
                "OpenQASM identifier must not be empty",
            )
        })?;

    if !(first == '_'
        || first.is_ascii_alphabetic())
    {
        return Err(export_error(
            FrontendErrorKind::Unsupported,
            CODE_IDENTIFIER,
            format!(
                "IR parameter identifier `{identifier}` \
                 cannot be emitted by the OpenQASM \
                 ASCII identifier subset",
            ),
        ));
    }

    if !characters.all(|character| {
        character == '_'
            || character.is_ascii_alphanumeric()
    }) {
        return Err(export_error(
            FrontendErrorKind::Unsupported,
            CODE_IDENTIFIER,
            format!(
                "IR parameter identifier `{identifier}` \
                 contains characters outside the \
                 OpenQASM exporter identifier subset",
            ),
        ));
    }

    if is_reserved_openqasm_identifier(identifier) {
        return Err(export_error(
            FrontendErrorKind::Unsupported,
            CODE_IDENTIFIER,
            format!(
                "IR parameter identifier `{identifier}` \
                 conflicts with an OpenQASM reserved or \
                 predefined identifier",
            ),
        ));
    }

    Ok(())
}

/// OpenQASM identifiers that cannot safely be emitted as ordinary symbolic
/// parameters.
///
/// `switch`, `case`, and `default` are intentionally NOT included: OpenQASM
/// 3.1 explicitly removed them from the reserved-identifier set.
fn is_reserved_openqasm_identifier(
    identifier: &str,
) -> bool {
    matches!(
        identifier,
        /*
         * Language keywords.
         */
        "OPENQASM"
            | "include"
            | "defcalgrammar"
            | "def"
            | "cal"
            | "defcal"
            | "gate"
            | "extern"
            | "box"
            | "let"
            | "break"
            | "continue"
            | "if"
            | "else"
            | "return"
            | "for"
            | "while"
            | "in"
            | "input"
            | "output"
            | "const"
            | "readonly"
            | "mutable"
            | "qreg"
            | "qubit"
            | "creg"
            | "bit"
            | "bool"
            | "int"
            | "uint"
            | "float"
            | "angle"
            | "complex"
            | "void"
            | "duration"
            | "stretch"
            | "measure"
            | "reset"
            | "barrier"
            | "delay"
            | "pragma"

            /*
             * Gate modifiers.
             */
            | "inv"
            | "pow"
            | "ctrl"
            | "negctrl"

            /*
             * Built-in mathematical constants.
             */
            | "pi"
            | "tau"
            | "euler"

            /*
             * Built-in mathematical functions and language intrinsics.
             *
             * These names must not be emitted as free symbolic identifiers
             * because they have predefined language meaning.
             */
            | "arccos"
            | "arcsin"
            | "arctan"
            | "ceiling"
            | "cos"
            | "exp"
            | "floor"
            | "log"
            | "mod"
            | "popcount"
            | "real"
            | "imag"
            | "rotl"
            | "rotr"
            | "sin"
            | "sqrt"
            | "tan"
            | "sizeof"
            | "durationof"

            /*
             * Built-in gate names emitted by this exporter.
             */
            | "id"
            | "x"
            | "y"
            | "z"
            | "h"
            | "s"
            | "sdg"
            | "t"
            | "tdg"
            | "rx"
            | "ry"
            | "rz"
            | "p"
            | "phase"
            | "u1"
            | "u2"
            | "u3"
            | "cx"
            | "cy"
            | "cz"
            | "ch"
            | "crx"
            | "cry"
            | "crz"
            | "swap"
            | "ccx"
            | "cswap"
            | "CX"
    )
}

/// Bounded output helper implementing `fmt::Write`.
///
/// This is used for future extensions that need direct formatted emission
/// without bypassing the global output bound.
struct BoundedFormatter<'a> {
    output: &'a mut BoundedOutput,
}

impl fmt::Write for BoundedFormatter<'_> {
    fn write_str(
        &mut self,
        value: &str,
    ) -> fmt::Result {
        let current =
            self.output.value.len();

        let next = current
            .checked_add(value.len())
            .ok_or(fmt::Error)?;

        if next > self.output.max_bytes {
            return Err(fmt::Error);
        }

        if value.len()
            > MAX_OPERATION_LINE_BYTES
        {
            return Err(fmt::Error);
        }

        self.output.value.push_str(value);

        Ok(())
    }
}

/// Creates a stable frontend export error.
fn export_error(
    kind: FrontendErrorKind,
    code: &'static str,
    message: impl Into<String>,
) -> FrontendError {
    FrontendError::with_code(
        kind,
        FrontendErrorCode::new(code),
        message.into(),
    )
    .context(
        "format",
        OPENQASM_FORMAT_ID,
    )
    .context(
        "stage",
        "export",
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::quantum::ir::{
        Gate,
        GateKind,
        Measurement,
        Parameter,
        QubitId,
    };

    fn exporter() -> OpenQasmExporter {
        OpenQasmExporter::production()
            .expect(
                "production OpenQASM exporter \
                 must construct",
            )
    }

    fn circuit(
        qubits: usize,
        classical_bits: usize,
        operations: Vec<Gate>,
    ) -> QuantumCircuit {
        QuantumCircuit::from_operations(
            qubits,
            classical_bits,
            operations,
        )
        .expect(
            "test circuit must satisfy \
             canonical IR invariants",
        )
    }

    #[test]
    fn production_targets_openqasm_3_1() {
        let exporter = exporter();

        assert_eq!(
            exporter.configured_version(),
            OPENQASM_3_1,
        );

        assert_eq!(
            exporter.format().id().as_str(),
            OPENQASM_FORMAT_ID,
        );
    }

    #[test]
    fn explicit_openqasm_3_0_is_supported() {
        let exporter =
            OpenQasmExporter::new(
                OPENQASM_3_0,
            )
            .expect(
                "OpenQASM 3.0 exporter \
                 must construct",
            );

        assert_eq!(
            exporter.configured_version(),
            OPENQASM_3_0,
        );
    }

    #[test]
    fn openqasm_2_is_rejected() {
        assert!(
            OpenQasmExporter::new(
                FormatVersion::new(2, 0, 0),
            )
            .is_err()
        );
    }

    #[test]
    fn unsupported_future_minor_is_rejected() {
        assert!(
            OpenQasmExporter::new(
                FormatVersion::new(3, 2, 0),
            )
            .is_err()
        );
    }

    #[test]
    fn capabilities_advertise_direct_features() {
        let exporter = exporter();
        let capabilities =
            exporter.format().capabilities();

        assert!(
            capabilities
                .supports(FormatCapability::Export)
        );

        assert!(
            capabilities
                .supports(FormatCapability::Parameters)
        );

        assert!(
            capabilities
                .supports(FormatCapability::Measurements)
        );

        assert!(
            capabilities
                .supports(FormatCapability::Reset)
        );

        assert!(
            capabilities
                .supports(FormatCapability::Barriers)
        );

        assert!(
            capabilities
                .supports(FormatCapability::Expressions)
        );

        assert!(
            !capabilities
                .supports(
                    FormatCapability::ClassicalControl
                )
        );

        assert!(
            !capabilities
                .supports(
                    FormatCapability::Calibration
                )
        );
    }

    #[test]
    fn gate_names_are_deterministic() {
        assert_eq!(
            gate_name(GateKind::I),
            Some("id"),
        );

        assert_eq!(
            gate_name(GateKind::Phase),
            Some("p"),
        );

        assert_eq!(
            gate_name(GateKind::CX),
            Some("cx"),
        );

        assert_eq!(
            gate_name(GateKind::CCX),
            Some("ccx"),
        );

        assert_eq!(
            gate_name(GateKind::V),
            None,
        );

        assert_eq!(
            gate_name(GateKind::ISWAP),
            None,
        );

        assert_eq!(
            gate_name(GateKind::ECR),
            None,
        );
    }

    #[test]
    fn qubit_order_is_preserved() {
        let gate = Gate::new(
            GateKind::CX,
            vec![
                QubitId::new(7),
                QubitId::new(2),
            ],
            Vec::new(),
            None,
            None,
        )
        .expect(
            "valid CX gate must construct",
        );

        assert_eq!(
            format_qubit_operands(&gate)
                .expect(
                    "qubit operands must format",
                ),
            "q[7], q[2]",
        );
    }

    #[test]
    fn symbolic_identifier_rules_are_stable() {
        assert!(
            validate_identifier("theta")
                .is_ok()
        );

        assert!(
            validate_identifier("theta_1")
                .is_ok()
        );

        assert!(
            validate_identifier("_theta")
                .is_ok()
        );

        assert!(
            validate_identifier("1theta")
                .is_err()
        );

        assert!(
            validate_identifier("theta-value")
                .is_err()
        );

        assert!(
            validate_identifier("theta.value")
                .is_err()
        );
    }

    #[test]
    fn openqasm_3_1_removed_future_switch_reservation() {
        assert!(
            validate_identifier("switch")
                .is_ok()
        );

        assert!(
            validate_identifier("case")
                .is_ok()
        );

        assert!(
            validate_identifier("default")
                .is_ok()
        );
    }

    #[test]
    fn builtins_are_not_valid_symbol_names() {
        assert!(
            validate_identifier("pi")
                .is_err()
        );

        assert!(
            validate_identifier("tau")
                .is_err()
        );

        assert!(
            validate_identifier("euler")
                .is_err()
        );

        assert!(
            validate_identifier("sin")
                .is_err()
        );

        assert!(
            validate_identifier("measure")
                .is_err()
        );
    }

    #[test]
    fn finite_parameters_are_accepted() {
        assert!(
            validate_parameter(
                &Parameter::Constant(1.25),
            )
            .is_ok()
        );
    }

    #[test]
    fn non_finite_parameters_are_rejected() {
        assert!(
            validate_parameter(
                &Parameter::Constant(
                    f64::NAN,
                ),
            )
            .is_err()
        );

        assert!(
            validate_parameter(
                &Parameter::Constant(
                    f64::INFINITY,
                ),
            )
            .is_err()
        );

        assert!(
            validate_parameter(
                &Parameter::Constant(
                    f64::NEG_INFINITY,
                ),
            )
            .is_err()
        );
    }

    #[test]
    fn measurement_preserves_explicit_mapping() {
        let measurement =
            Measurement::new(
                QubitId::new(3),
                9usize.into(),
            );

        assert_eq!(
            measurement.qubit().index(),
            3,
        );

        assert_eq!(
            measurement
                .classical_bit()
                .index(),
            9,
        );
    }

    #[test]
    fn standard_library_include_is_stable() {
        assert_eq!(
            STANDARD_LIBRARY_INCLUDE,
            "stdgates.inc",
        );
    }

    #[test]
    fn media_type_is_stable() {
        assert_eq!(
            OPENQASM_MEDIA_TYPE,
            "text/x-openqasm",
        );
    }

    #[test]
    fn bounded_output_rejects_oversized_output() {
        let mut output =
            BoundedOutput::new(8);

        assert!(
            output
                .line("12345678")
                .is_err()
        );
    }

    #[test]
    fn bounded_output_accepts_exact_limit() {
        let mut output =
            BoundedOutput::new(9);

        assert!(
            output
                .line("12345678")
                .is_ok()
        );
    }

    #[test]
    fn empty_quantum_namespace_is_not_fabricated() {
        let circuit =
            circuit(0, 1, Vec::new());

        let text = exporter()
            .export_circuit(&circuit)
            .expect(
                "zero-qubit circuit should export",
            )
            .into_text()
            .expect(
                "OpenQASM output must be UTF-8",
            );

        assert!(
            !text.contains("qubit[0] q;")
        );

        assert!(
            text.contains("bit[1] c;")
        );
    }
}
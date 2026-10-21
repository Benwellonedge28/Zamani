
#![forbid(unsafe_code)]

//! Zamani WebAssembly target emitter.
//!
//! Path: src/backend/wasm/emitter.rs
//!
//! Responsibility:
//!   Provide the stable backend-facing API for WebAssembly text (WAT)
//!   and binary emission while reusing the existing compiler backend.
//!
//! Ownership boundaries:
//!   - `crate::ir_gen` owns the source IR representation.
//!   - `crate::compiler::wasm_cfg` owns control-flow graph analysis.
//!   - `crate::compiler::wasm_backend` owns the existing Wasm lowering.
//!   - This module owns the backend-facing emission contract and its
//!     configuration/error boundary.
//!   - Output persistence belongs to a separate output stage.
//!
//! This module deliberately does not implement a second IR, a second
//! control-flow graph, or a competing WebAssembly instruction selector.
//!
//! Safety:
//!   Safe Rust only. No `unsafe`.
//!
//! Scalability:
//!   No compiler-defined maximum source size, function count, instruction
//!   count, local count, or module count is imposed here. The WebAssembly
//!   32-bit memory page limit is a target-format constraint, not a Zamani
//!   language limit. Allocation failures and process-level resource
//!   exhaustion remain possible and are not represented as guaranteed
//!   recoverable Rust errors.

use std::error::Error;
use std::fmt;

use crate::compiler::wasm_backend::WasmBackend as CompilerWasmBackend;
use crate::ir_gen::IrModule;

/// The maximum number of 64-KiB pages representable by a WebAssembly
/// memory32 memory.
///
/// This is a WebAssembly format constraint, not a universal Zamani resource
/// ceiling. A memory64 target requires a distinct target contract and
/// lowering implementation.
const WASM32_MAX_PAGES: u32 = 65_536;

/// Configuration for the WebAssembly emitter.
///
/// These options describe the requested Wasm artifact. They do not change
/// source-language semantics and must not be interpreted as guarantees that
/// a particular runtime or deployment environment has the requested memory
/// available.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WasmEmitterConfig {
    /// Export the module's linear memory as `"memory"`.
    pub export_memory: bool,

    /// Initial linear-memory size in WebAssembly pages.
    ///
    /// One WebAssembly page is 65,536 bytes.
    pub initial_memory_pages: u32,

    /// Optional declared maximum linear-memory size, in pages.
    ///
    /// `None` leaves the maximum unspecified in the module declaration.
    pub maximum_memory_pages: Option<u32>,

    /// Export the defined `main` function when it exists.
    pub export_main: bool,
}

impl Default for WasmEmitterConfig {
    fn default() -> Self {
        let defaults = CompilerWasmBackend::default();

        Self {
            export_memory: defaults.export_memory,
            initial_memory_pages: defaults.initial_memory_pages,
            maximum_memory_pages: defaults.maximum_memory_pages,
            export_main: defaults.export_main,
        }
    }
}

impl WasmEmitterConfig {
    /// Validate configuration independently of any input module.
    ///
    /// The current underlying compiler backend reserves the first two
    /// memory pages for its existing static-data/stack-pointer convention.
    /// Until that memory-layout implementation is replaced by a proper
    /// layout planner, this adapter rejects configurations that would
    /// violate that existing contract.
    pub fn validate(&self) -> Result<(), WasmEmitterError> {
        if self.initial_memory_pages < 2 {
            return Err(WasmEmitterError::InvalidConfiguration {
                message: format!(
                    "initial_memory_pages must be at least 2 for the \
                     current compiler backend memory-layout convention; \
                     received {}",
                    self.initial_memory_pages
                ),
            });
        }

        if self.initial_memory_pages > WASM32_MAX_PAGES {
            return Err(WasmEmitterError::InvalidConfiguration {
                message: format!(
                    "initial_memory_pages exceeds the WebAssembly memory32 \
                     limit of {WASM32_MAX_PAGES} pages; received {}",
                    self.initial_memory_pages
                ),
            });
        }

        if let Some(maximum) = self.maximum_memory_pages {
            if maximum < self.initial_memory_pages {
                return Err(WasmEmitterError::InvalidConfiguration {
                    message: format!(
                        "maximum_memory_pages ({maximum}) must be greater \
                         than or equal to initial_memory_pages ({})",
                        self.initial_memory_pages
                    ),
                });
            }

            if maximum > WASM32_MAX_PAGES {
                return Err(WasmEmitterError::InvalidConfiguration {
                    message: format!(
                        "maximum_memory_pages exceeds the WebAssembly \
                         memory32 limit of {WASM32_MAX_PAGES} pages; \
                         received {maximum}"
                    ),
                });
            }
        }

        Ok(())
    }
}

/// The stage at which WebAssembly emission failed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WasmEmissionStage {
    Configuration,
    Lowering,
    BinaryAssembly,
}

/// Structured WebAssembly emission failure.
///
/// This error preserves a stage identifier so callers can report failures
/// without parsing human-readable messages.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WasmEmitterError {
    /// Failed emission stage.
    pub stage: WasmEmissionStage,

    /// Human-readable explanation suitable for diagnostics.
    pub message: String,
}

impl WasmEmitterError {
    fn new(stage: WasmEmissionStage, message: impl Into<String>) -> Self {
        Self {
            stage,
            message: message.into(),
        }
    }

    fn invalid_configuration(message: impl Into<String>) -> Self {
        Self::new(
            WasmEmissionStage::Configuration,
            message,
        )
    }
}

impl fmt::Display for WasmEmitterError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let stage = match self.stage {
            WasmEmissionStage::Configuration => "configuration",
            WasmEmissionStage::Lowering => "lowering",
            WasmEmissionStage::BinaryAssembly => "binary assembly",
        };

        write!(
            formatter,
            "WebAssembly {stage} failed: {}",
            self.message
        )
    }
}

impl Error for WasmEmitterError {}

/// Stable backend-facing WebAssembly emitter.
///
/// The emitter is immutable after construction and can be shared between
/// compilations. Each invocation delegates work to a fresh instance of the
/// existing compiler backend, avoiding shared mutable emission state.
#[derive(Debug, Clone, Default)]
pub struct WasmEmitter {
    config: WasmEmitterConfig,
}

impl WasmEmitter {
    /// Construct an emitter using the repository's default Wasm settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Construct an emitter with explicit target settings.
    pub fn with_config(
        config: WasmEmitterConfig,
    ) -> Result<Self, WasmEmitterError> {
        config.validate()?;

        Ok(Self { config })
    }

    /// Borrow the active configuration.
    pub fn config(&self) -> &WasmEmitterConfig {
        &self.config
    }

    /// Generate WebAssembly Text Format (WAT).
    ///
    /// This method:
    /// - accepts the existing Zamani IR module;
    /// - delegates lowering to the canonical compiler Wasm backend;
    /// - returns the complete WAT text;
    /// - performs no file writes;
    /// - emits no console output.
    ///
    /// WAT generation does not by itself establish that every target runtime
    /// supports every feature used by the module. Runtime feature negotiation
    /// and execution tests remain separate validation stages.
    pub fn emit_wat(
        &self,
        module: &IrModule,
    ) -> Result<String, WasmEmitterError> {
        self.config.validate()?;

        let backend = self.build_compiler_backend();

        backend.emit_wat(module).map_err(|error| {
            WasmEmitterError::new(
                WasmEmissionStage::Lowering,
                error,
            )
        })
    }

    /// Generate and assemble a WebAssembly binary in memory.
    ///
    /// The existing compiler backend validates and assembles its emitted WAT
    /// through the repository's `wat` dependency.
    ///
    /// No output path is accepted because artifact persistence is owned by
    /// the caller's explicit output stage.
    pub fn emit_binary(
        &self,
        module: &IrModule,
    ) -> Result<Vec<u8>, WasmEmitterError> {
        self.config.validate()?;

        let backend = self.build_compiler_backend();

        backend.compile(module).map_err(|error| {
            let stage = if error.contains("failed WAT validation")
                || error.contains("WebAssembly")
            {
                WasmEmissionStage::BinaryAssembly
            } else {
                WasmEmissionStage::Lowering
            };

            WasmEmitterError::new(stage, error)
        })
    }

    /// Return the output format associated with this emitter.
    pub const fn text_extension(&self) -> &'static str {
        "wat"
    }

    /// Return the binary output format associated with this emitter.
    pub const fn binary_extension(&self) -> &'static str {
        "wasm"
    }

    fn build_compiler_backend(&self) -> CompilerWasmBackend {
        CompilerWasmBackend {
            export_memory: self.config.export_memory,
            initial_memory_pages: self.config.initial_memory_pages,
            maximum_memory_pages: self.config.maximum_memory_pages,
            export_main: self.config.export_main,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn empty_module() -> IrModule {
        IrModule {
            name: "emitter_test".to_owned(),
            functions: Vec::new(),
            globals: Vec::new(),
            string_literals: Default::default(),
        }
    }

    #[test]
    fn default_configuration_matches_existing_backend() {
        let config = WasmEmitterConfig::default();
        let backend = CompilerWasmBackend::default();

        assert_eq!(config.export_memory, backend.export_memory);
        assert_eq!(
            config.initial_memory_pages,
            backend.initial_memory_pages
        );
        assert_eq!(
            config.maximum_memory_pages,
            backend.maximum_memory_pages
        );
        assert_eq!(config.export_main, backend.export_main);
    }

    #[test]
    fn rejects_memory_maximum_below_initial_size() {
        let config = WasmEmitterConfig {
            initial_memory_pages: 4,
            maximum_memory_pages: Some(3),
            ..WasmEmitterConfig::default()
        };

        assert!(matches!(
            config.validate(),
            Err(WasmEmitterError {
                stage: WasmEmissionStage::Configuration,
                ..
            })
        ));
    }

    #[test]
    fn rejects_memory32_size_outside_format_limit() {
        let config = WasmEmitterConfig {
            initial_memory_pages: WASM32_MAX_PAGES + 1,
            maximum_memory_pages: None,
            ..WasmEmitterConfig::default()
        };

        assert!(config.validate().is_err());
    }

    #[test]
    fn rejects_memory_layout_incompatible_with_current_backend() {
        let config = WasmEmitterConfig {
            initial_memory_pages: 1,
            maximum_memory_pages: Some(1),
            ..WasmEmitterConfig::default()
        };

        assert!(config.validate().is_err());
    }

    #[test]
    fn configured_emitter_retains_configuration() {
        let config = WasmEmitterConfig {
            export_memory: false,
            initial_memory_pages: 3,
            maximum_memory_pages: Some(8),
            export_main: false,
        };

        let emitter = WasmEmitter::with_config(config.clone()).unwrap();

        assert_eq!(emitter.config(), &config);
    }

    #[test]
    fn emits_wat_without_writing_an_output_file() {
        let emitter = WasmEmitter::new();
        let module = empty_module();

        let wat = emitter.emit_wat(&module).unwrap();

        assert!(wat.starts_with("(module"));
        assert!(wat.trim_end().ends_with(')'));
    }

    #[test]
    fn emits_a_valid_binary_for_an_empty_module() {
        let emitter = WasmEmitter::new();
        let module = empty_module();

        let bytes = emitter.emit_binary(&module).unwrap();

        // WebAssembly binary magic and version 1.
        assert_eq!(
            bytes.get(..8),
            Some(&[0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00][..])
        );
    }

    #[test]
    fn output_format_names_are_stable() {
        let emitter = WasmEmitter::new();

        assert_eq!(emitter.text_extension(), "wat");
        assert_eq!(emitter.binary_extension(), "wasm");
    }
}

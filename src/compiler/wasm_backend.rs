//! Zamani Compiler — Production WebAssembly Backend
//!
//! Lowers Zamani's typed SSA-like IR into WebAssembly Text (WAT) and then
//! assembles it into a validated WebAssembly binary.
//!
//! Design goals:
//!   * deterministic output;
//!   * strict IR validation;
//!   * explicit rejection of unsupported semantics;
//!   * real linear-memory support;
//!   * explicit function signatures;
//!   * CFG dispatch compatible with arbitrary Zamani CFGs;
//!   * SSA Phi lowering on control-flow edges;
//!   * safe string/data emission;
//!   * no silent "comment instead of instruction" fallbacks.
//!
//! The final `.wasm` binary is produced by the `wat` crate. The dependency
//! must therefore be present in Cargo.toml.

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs;
use std::path::Path;

use crate::compiler::wasm_cfg::WasmControlFlowGraph;
use crate::ir_gen::{
    CmpOp, IrFunction, IrInstruction, IrModule, IrRegister, IrType, IrValue,
};

/// Production WebAssembly backend.
#[derive(Debug, Clone)]
pub struct WasmBackend {
    /// Whether the generated module exports linear memory.
    pub export_memory: bool,

    /// Initial WebAssembly memory pages.
    pub initial_memory_pages: u32,

    /// Optional maximum memory pages.
    pub maximum_memory_pages: Option<u32>,

    /// Whether the module exports `main`.
    pub export_main: bool,
}

impl Default for WasmBackend {
    fn default() -> Self {
        Self {
            export_memory: true,
            initial_memory_pages: 2,
            maximum_memory_pages: Some(65536),
            export_main: true,
        }
    }
}

impl WasmBackend {
    pub fn new() -> Self {
        Self::default()
    }

    /// Emit a validated WebAssembly binary.
    pub fn emit_wasm(
        &self,
        module: &IrModule,
        output_path: &str,
    ) -> Result<(), String> {
        if output_path.trim().is_empty() {
            return Err("WebAssembly output path cannot be empty.".into());
        }

        let bytes = self.compile(module)?;

        let path = Path::new(output_path);

        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent).map_err(|error| {
                    format!(
                        "Unable to create WebAssembly output directory '{}': {}",
                        parent.display(),
                        error
                    )
                })?;
            }
        }

        fs::write(path, &bytes).map_err(|error| {
            format!(
                "Unable to write WebAssembly binary '{}': {}",
                path.display(),
                error
            )
        })?;

        println!(
            "[Wasm-Backend] '{}' -> {} bytes of validated WebAssembly.",
            module.name,
            bytes.len()
        );

        Ok(())
    }

    /// Compile an IR module to a WebAssembly binary in memory.
    pub fn compile(&self, module: &IrModule) -> Result<Vec<u8>, String> {
        self.validate_module(module)?;

        let wat = self.emit_wat(module)?;

        /*
         * `wat::parse_str` performs WAT parsing and binary assembly. This is
         * intentionally kept at the final boundary so all compiler-level
         * validation happens before the assembler is invoked.
         */
        wat::parse_str(&wat).map_err(|error| {
            format!(
                "Generated WebAssembly failed WAT validation: {}",
                error
            )
        })
    }

    /// Generate deterministic WebAssembly Text.
    pub fn emit_wat(&self, module: &IrModule) -> Result<String, String> {
        self.validate_module(module)?;

        let mut out = String::new();

        out.push_str("(module\n");

        self.emit_memory(&mut out);
        self.emit_string_data(module, &mut out)?;

        /*
         * Runtime stack pointer.
         *
         * The initial value is deliberately after the first two pages.
         * This prevents stack allocations from overlapping static data in
         * ordinary modules.
         */
        out.push_str(
            "  (global $__zamani_stack_pointer (mut i32) \
             (i32.const 131072))\n",
        );

        /*
         * Declare functions before their definitions so calls can refer to
         * stable symbolic names in WAT.
         */
        for function in &module.functions {
            self.emit_function_signature(function, &mut out)?;
        }

        for function in &module.functions {
            if !function.is_external {
                self.emit_function(module, function, &mut out)?;
            }
        }

        self.emit_exports(module, &mut out);

        out.push_str(")\n");

        Ok(out)
    }

    fn emit_memory(&self, out: &mut String) {
        match self.maximum_memory_pages {
            Some(max) => {
                out.push_str(&format!(
                    "  (memory {} {})\n",
                    self.initial_memory_pages,
                    max
                ));
            }
            None => {
                out.push_str(&format!(
                    "  (memory {})\n",
                    self.initial_memory_pages
                ));
            }
        }
    }

    fn emit_string_data(
        &self,
        module: &IrModule,
        out: &mut String,
    ) -> Result<(), String> {
        /*
         * Static data begins at address 1024. Each literal receives a
         * deterministic, non-overlapping address.
         */
        let mut address: u32 = 1024;

        for (name, value) in &module.string_literals {
            let bytes = value.as_bytes();

            let end = address
                .checked_add(bytes.len() as u32)
                .and_then(|v| v.checked_add(1))
                .ok_or_else(|| {
                    format!(
                        "String literal '{}' causes a static-data address overflow.",
                        name
                    )
                })?;

            if end > self.initial_memory_pages.saturating_mul(65536) {
                return Err(format!(
                    "String literal '{}' does not fit in initial WebAssembly memory.",
                    name
                ));
            }

            out.push_str(&format!(
                "  (data (i32.const {}) \"{}\")\n",
                address,
                escape_wat_bytes(bytes)
            ));

            address = align_up(end, 8);
        }

        Ok(())
    }

    fn emit_function_signature(
        &self,
        function: &IrFunction,
        out: &mut String,
    ) -> Result<(), String> {
        if function.is_external {
            out.push_str(&format!(
                "  ;; external function: {}\n",
                sanitize_wat_symbol(&function.name)
            ));
        }

        Ok(())
    }

    fn emit_function(
        &self,
        module: &IrModule,
        function: &IrFunction,
        out: &mut String,
    ) -> Result<(), String> {
        let cfg = WasmControlFlowGraph::from_function(function)?;

        let locals = collect_locals(function)?;

        out.push_str(&format!(
            "  (func ${}",
            sanitize_wat_symbol(&function.name)
        ));

        for (name, ty) in &function.params {
            let wasm_ty = wasm_type(ty)?;
            out.push_str(&format!(
                " (param ${} {})",
                sanitize_wat_symbol(name),
                wasm_ty
            ));
        }

        if let Some(ret) = wasm_result_type(&function.return_type)? {
            out.push_str(&format!(" (result {})", ret));
        }

        for (name, ty) in &locals {
            out.push_str(&format!(
                "\n    (local ${} {})",
                sanitize_wat_symbol(name),
                wasm_type(ty)?
            ));
        }

        /*
         * Dispatcher state.
         */
        out.push_str("\n    (local $__zamani_pc i32)\n");
        out.push_str("    (local.set $__zamani_pc (i32.const 0))\n");

        let blocks: Vec<String> =
            cfg.blocks().map(|block| block.label.clone()).collect();

        if blocks.is_empty() {
            return Err(format!(
                "Function '{}' has no WebAssembly blocks.",
                function.name
            ));
        }

        /*
         * Dispatcher pattern:
         *
         * loop $dispatch
         *   block $bbN
         *     ...
         *       block $bb0
         *         local.get pc
         *         br_table ...
         *       end
         *       code(bb0)
         *       br $dispatch
         *     end
         *     code(bb1)
         *     br $dispatch
         *   ...
         *
         * This preserves arbitrary CFG edges without pretending that every
         * source CFG is structurally reducible.
         */
        out.push_str("    (loop $dispatch\n");

        for label in blocks.iter().rev() {
            out.push_str(&format!(
                "      (block ${}\n",
                wasm_block_label(label)
            ));
        }

        out.push_str("        (local.get $__zamani_pc)\n");
        out.push_str("        (br_table");

        for label in &blocks {
            out.push_str(&format!(" ${}", wasm_block_label(label)));
        }

        let default = blocks
            .last()
            .map(|label| wasm_block_label(label))
            .ok_or_else(|| {
                format!(
                    "Function '{}' has no dispatcher default block.",
                    function.name
                )
            })?;

        out.push_str(&format!(" ${})\n", default));

        /*
         * Close the nested dispatcher blocks while emitting each block's
         * instructions immediately after its corresponding closing boundary.
         */
        for (index, label) in blocks.iter().enumerate() {
            out.push_str("      )\n");

            let block = cfg.block(label).ok_or_else(|| {
                format!(
                    "Internal Wasm CFG error: block '{}' not found.",
                    label
                )
            })?;

            self.emit_block(
                module,
                function,
                &cfg,
                block,
                index,
                out,
            )?;
        }

        out.push_str("    )\n");
        out.push_str("  )\n");

        Ok(())
    }

    fn emit_block(
        &self,
        module: &IrModule,
        function: &IrFunction,
        cfg: &WasmControlFlowGraph,
        block: &crate::compiler::wasm_cfg::WasmBasicBlock,
        block_index: usize,
        out: &mut String,
    ) -> Result<(), String> {
        out.push_str(&format!(
            "      ;; Zamani basic block {} ({})\n",
            block_index,
            block.label
        ));

        for instruction in &block.instructions {
            match instruction {
                IrInstruction::Comment(comment) => {
                    out.push_str(&format!(
                        "      ;; {}\n",
                        sanitize_comment(comment)
                    ));
                }

                IrInstruction::Label(_) => {
                    return Err(format!(
                        "Internal CFG error: Label remained inside block '{}'.",
                        block.label
                    ));
                }

                IrInstruction::Alloca(register, ty) => {
                    self.emit_alloca(register, ty, out)?;
                }

                IrInstruction::Load(register, value) => {
                    self.emit_load(register, value, out)?;
                }

                IrInstruction::Store(value, address) => {
                    self.emit_store(value, address, out)?;
                }

                IrInstruction::Add(register, a, b)
                | IrInstruction::Sub(register, a, b)
                | IrInstruction::Mul(register, a, b)
                | IrInstruction::Div(register, a, b)
                | IrInstruction::Rem(register, a, b)
                | IrInstruction::And(register, a, b)
                | IrInstruction::Or(register, a, b)
                | IrInstruction::Xor(register, a, b)
                | IrInstruction::Shl(register, a, b)
                | IrInstruction::Shr(register, a, b) => {
                    self.emit_binary(instruction, register, a, b, out)?;
                }

                IrInstruction::Neg(register, value) => {
                    let ty = register.1.clone();

                    if is_float(&ty) {
                        self.emit_value(value, out)?;
                        match ty {
                            IrType::F32 => {
                                out.push_str("      f32.neg\n");
                            }
                            IrType::F64 => {
                                out.push_str("      f64.neg\n");
                            }
                            _ => unreachable!(),
                        }
                    } else {
                        self.emit_value(value, out)?;
                        match integer_wasm_type(&ty)? {
                            "i32" => {
                                out.push_str("      i32.const -1\n");
                                out.push_str("      i32.mul\n");
                            }
                            "i64" => {
                                out.push_str("      i64.const -1\n");
                                out.push_str("      i64.mul\n");
                            }
                            _ => unreachable!(),
                        }
                    }

                    emit_local_set(register, out);
                }

                IrInstruction::Not(register, value) => {
                    self.emit_value(value, out)?;

                    match integer_wasm_type(&register.1)? {
                        "i32" => out.push_str("      i32.const -1\n      i32.xor\n"),
                        "i64" => out.push_str("      i64.const -1\n      i64.xor\n"),
                        _ => {
                            return Err(format!(
                                "Bitwise NOT requires an integer type; got {:?}.",
                                register.1
                            ));
                        }
                    }

                    emit_local_set(register, out);
                }

                IrInstruction::Cmp(register, op, a, b) => {
                    self.emit_cmp(register, op, a, b, out)?;
                }

                IrInstruction::Assign(register, value) => {
                    self.emit_value(value, out)?;
                    emit_local_set(register, out);
                }

                IrInstruction::Call(result, name, args) => {
                    for arg in args {
                        self.emit_value(arg, out)?;
                    }

                    out.push_str(&format!(
                        "      call ${}\n",
                        sanitize_wat_symbol(name)
                    ));

                    if let Some(register) = result {
                        emit_local_set(register, out);
                    }
                }

                IrInstruction::CallIndirect(result, target, args) => {
                    return Err(format!(
                        "Function '{}' block '{}' contains CallIndirect. \
                         Indirect calls require a validated Wasm function-table \
                         ABI and signature metadata; refusing to emit an \
                         unsafe call_indirect.",
                        function.name, block.label
                    ));
                }

                IrInstruction::GetElementPtr(register, base, indices) => {
                    self.emit_value(base, out)?;

                    for index in indices {
                        self.emit_value(index, out)?;

                        match wasm_value_type(&index.ty())? {
                            "i32" => out.push_str("      i32.add\n"),
                            "i64" => {
                                out.push_str("      i32.wrap_i64\n");
                                out.push_str("      i32.add\n");
                            }
                            _ => {
                                return Err(
                                    "GEP index must be an integer."
                                        .to_string()
                                );
                            }
                        }
                    }

                    emit_local_set(register, out);
                }

                IrInstruction::BitCast(register, value, target) => {
                    self.emit_bitcast(register, value, target, out)?;
                }

                IrInstruction::Phi(register, _) => {
                    /*
                     * Phi values are materialized on predecessor edges.
                     * Leaving one in the block would duplicate semantics.
                     */
                    continue;
                }

                IrInstruction::ZExt(register, value, target) => {
                    self.emit_integer_extension(
                        register,
                        value,
                        target,
                        false,
                        out,
                    )?;
                }

                IrInstruction::SExt(register, value, target) => {
                    self.emit_integer_extension(
                        register,
                        value,
                        target,
                        true,
                        out,
                    )?;
                }

                IrInstruction::Trunc(register, value, target) => {
                    self.emit_trunc(register, value, target, out)?;
                }

                IrInstruction::FpExt(register, value, target) => {
                    self.emit_fp_extension(register, value, target, out)?;
                }

                IrInstruction::FpTrunc(register, value, target) => {
                    self.emit_fp_trunc(register, value, target, out)?;
                }

                IrInstruction::SIToFP(register, value, target) => {
                    self.emit_si_to_fp(register, value, target, out)?;
                }

                IrInstruction::FPToSI(register, value, target) => {
                    self.emit_fp_to_si(register, value, target, out)?;
                }

                IrInstruction::Jump(target) => {
                    self.emit_phi_edge(block, target, cfg, out)?;
                    self.emit_set_pc(target, cfg, out)?;

                    out.push_str("      br $dispatch\n");
                }

                IrInstruction::CondJump(condition, true_target, false_target) => {
                    self.emit_value(condition, out)?;

                    out.push_str("      if\n");

                    self.emit_phi_edge(
                        block,
                        true_target,
                        cfg,
                        out,
                    )?;

                    self.emit_set_pc(
                        true_target,
                        cfg,
                        out,
                    )?;

                    out.push_str("      else\n");

                    self.emit_phi_edge(
                        block,
                        false_target,
                        cfg,
                        out,
                    )?;

                    self.emit_set_pc(
                        false_target,
                        cfg,
                        out,
                    )?;

                    out.push_str("      end\n");
                    out.push_str("      br $dispatch\n");
                }

                IrInstruction::Ret(value) => {
                    if let Some(value) = value {
                        self.emit_value(value, out)?;
                    }

                    out.push_str("      return\n");
                }

                IrInstruction::Unreachable => {
                    out.push_str("      unreachable\n");
                }

                IrInstruction::QuantumGate(register, gate, args) => {
                    return Err(format!(
                        "QuantumGate '{}' cannot be lowered to portable WebAssembly \
                         without an explicit Zamani quantum runtime ABI.",
                        gate
                    ));
                }

                IrInstruction::NanoOp(register, operation, args) => {
                    return Err(format!(
                        "NanoOp '{}' cannot be lowered to portable WebAssembly \
                         without an explicit Zamani Nano runtime ABI.",
                        operation
                    ));
                }

                IrInstruction::SankofaRecall(register, domain) => {
                    return Err(
                        "SankofaRecall requires a defined WebAssembly runtime ABI."
                            .into(),
                    );
                }

                IrInstruction::SankofaRemember(name, value) => {
                    return Err(format!(
                        "SankofaRemember '{}' requires a defined WebAssembly \
                         runtime ABI.",
                        name
                    ));
                }
            }
        }

        /*
         * Blocks without explicit terminators fall through to the next
         * dispatcher state.
         */
        if !block.is_terminated() {
            if let Some(next) = cfg.successors_of(&block.label).first() {
                self.emit_phi_edge(block, next, cfg, out)?;
                self.emit_set_pc(next, cfg, out)?;
                out.push_str("      br $dispatch\n");
            } else {
                return Err(format!(
                    "Basic block '{}' has no terminator and no fall-through successor.",
                    block.label
                ));
            }
        }

        Ok(())
    }

    fn emit_alloca(
        &self,
        register: &IrRegister,
        ty: &IrType,
        out: &mut String,
    ) -> Result<(), String> {
        let size = type_size(ty)?;

        out.push_str("      global.get $__zamani_stack_pointer\n");
        out.push_str(&format!(
            "      i32.const {}\n",
            size
        ));
        out.push_str("      global.get $__zamani_stack_pointer\n");
        out.push_str(&format!(
            "      i32.const {}\n",
            size
        ));
        out.push_str("      i32.add\n");
        out.push_str("      global.set $__zamani_stack_pointer\n");

        emit_local_set(register, out);

        Ok(())
    }

    fn emit_load(
        &self,
        register: &IrRegister,
        address: &IrValue,
        out: &mut String,
    ) -> Result<(), String> {
        self.emit_value(address, out)?;

        emit_load_instruction(&register.1, out)?;
        emit_local_set(register, out);

        Ok(())
    }

    fn emit_store(
        &self,
        value: &IrValue,
        address: &IrValue,
        out: &mut String,
    ) -> Result<(), String> {
        self.emit_value(address, out)?;
        self.emit_value(value, out)?;

        emit_store_instruction(&value.ty(), out)?;

        Ok(())
    }

    fn emit_binary(
        &self,
        instruction: &IrInstruction,
        register: &IrRegister,
        a: &IrValue,
        b: &IrValue,
        out: &mut String,
    ) -> Result<(), String> {
        self.emit_value(a, out)?;
        self.emit_value(b, out)?;

        let ty = register.1.clone();

        if is_float(&ty) {
            let opcode = match instruction {
                IrInstruction::Add(..) => float_opcode(&ty, "add")?,
                IrInstruction::Sub(..) => float_opcode(&ty, "sub")?,
                IrInstruction::Mul(..) => float_opcode(&ty, "mul")?,
                IrInstruction::Div(..) => float_opcode(&ty, "div")?,
                _ => {
                    return Err(format!(
                        "Bitwise/shift operation {:?} is invalid for {:?}.",
                        instruction, ty
                    ));
                }
            };

            out.push_str(&format!("      {}\n", opcode));
        } else {
            let wasm_ty = integer_wasm_type(&ty)?;

            let opcode = match instruction {
                IrInstruction::Add(..) => format!("{}.add", wasm_ty),
                IrInstruction::Sub(..) => format!("{}.sub", wasm_ty),
                IrInstruction::Mul(..) => format!("{}.mul", wasm_ty),

                IrInstruction::Div(..) => {
                    if is_unsigned_integer(&ty) {
                        format!("{}.div_u", wasm_ty)
                    } else {
                        format!("{}.div_s", wasm_ty)
                    }
                }

                IrInstruction::Rem(..) => {
                    if is_unsigned_integer(&ty) {
                        format!("{}.rem_u", wasm_ty)
                    } else {
                        format!("{}.rem_s", wasm_ty)
                    }
                }

                IrInstruction::And(..) => format!("{}.and", wasm_ty),
                IrInstruction::Or(..) => format!("{}.or", wasm_ty),
                IrInstruction::Xor(..) => format!("{}.xor", wasm_ty),
                IrInstruction::Shl(..) => format!("{}.shl", wasm_ty),

                IrInstruction::Shr(..) => {
                    if is_unsigned_integer(&ty) {
                        format!("{}.shr_u", wasm_ty)
                    } else {
                        format!("{}.shr_s", wasm_ty)
                    }
                }

                _ => {
                    return Err(format!(
                        "Unsupported binary instruction {:?}.",
                        instruction
                    ));
                }
            };

            out.push_str(&format!("      {}\n", opcode));
        }

        emit_local_set(register, out);

        Ok(())
    }

    fn emit_cmp(
        &self,
        register: &IrRegister,
        op: &CmpOp,
        a: &IrValue,
        b: &IrValue,
        out: &mut String,
    ) -> Result<(), String> {
        self.emit_value(a, out)?;
        self.emit_value(b, out)?;

        let ty = a.ty();

        let opcode = if is_float(&ty) {
            let prefix = match ty {
                IrType::F32 => "f32",
                IrType::F64 => "f64",
                _ => unreachable!(),
            };

            match op {
                CmpOp::FEq => format!("{}.eq", prefix),
                CmpOp::FNe => format!("{}.ne", prefix),
                CmpOp::FLt => format!("{}.lt", prefix),
                CmpOp::FLe => format!("{}.le", prefix),
                CmpOp::FGt => format!("{}.gt", prefix),
                CmpOp::FGe => format!("{}.ge", prefix),

                _ => {
                    return Err(format!(
                        "Floating-point comparison {:?} requires a floating \
                         comparison predicate.",
                        op
                    ));
                }
            }
        } else {
            let prefix = integer_wasm_type(&ty)?;

            match op {
                CmpOp::Eq => format!("{}.eq", prefix),
                CmpOp::Ne => format!("{}.ne", prefix),
                CmpOp::Lt => {
                    if is_unsigned_integer(&ty) {
                        format!("{}.lt_u", prefix)
                    } else {
                        format!("{}.lt_s", prefix)
                    }
                }
                CmpOp::Le => {
                    if is_unsigned_integer(&ty) {
                        format!("{}.le_u", prefix)
                    } else {
                        format!("{}.le_s", prefix)
                    }
                }
                CmpOp::Gt => {
                    if is_unsigned_integer(&ty) {
                        format!("{}.gt_u", prefix)
                    } else {
                        format!("{}.gt_s", prefix)
                    }
                }
                CmpOp::Ge => {
                    if is_unsigned_integer(&ty) {
                        format!("{}.ge_u", prefix)
                    } else {
                        format!("{}.ge_s", prefix)
                    }
                }

                _ => {
                    return Err(format!(
                        "Integer comparison cannot use floating predicate {:?}.",
                        op
                    ));
                }
            }
        };

        out.push_str(&format!("      {}\n", opcode));
        emit_local_set(register, out);

        Ok(())
    }

    fn emit_value(
        &self,
        value: &IrValue,
        out: &mut String,
    ) -> Result<(), String> {
        match value {
            IrValue::Reg(register) => {
                out.push_str(&format!(
                    "      local.get ${}\n",
                    sanitize_wat_symbol(&register.0)
                ));
            }

            IrValue::ConstInt(value, ty) => {
                match integer_wasm_type(ty)? {
                    "i32" => {
                        out.push_str(&format!(
                            "      i32.const {}\n",
                            *value as i32
                        ));
                    }
                    "i64" => {
                        out.push_str(&format!(
                            "      i64.const {}\n",
                            value
                        ));
                    }
                    _ => unreachable!(),
                }
            }

            IrValue::ConstFloat(value, ty) => match ty {
                IrType::F32 => {
                    out.push_str(&format!(
                        "      f32.const {}\n",
                        *value as f32
                    ));
                }
                IrType::F64 => {
                    out.push_str(&format!(
                        "      f64.const {}\n",
                        value
                    ));
                }
                _ => {
                    return Err(format!(
                        "Invalid floating-point constant type {:?}.",
                        ty
                    ));
                }
            },

            IrValue::ConstBool(value) => {
                out.push_str(&format!(
                    "      i32.const {}\n",
                    if *value { 1 } else { 0 }
                ));
            }

            IrValue::ConstStr(value) => {
                return Err(format!(
                    "Inline string '{}' has no static-data symbol. \
                     The IR must lift string literals into module.string_literals.",
                    sanitize_comment(value)
                ));
            }

            IrValue::GlobalPtr(name, len) => {
                if *len == 0 {
                    return Err(format!(
                        "Global pointer '{}' does not contain a static-data length.",
                        name
                    ));
                }

                let address = find_string_address(
                    name,
                    module_string_literals_placeholder(),
                );

                /*
                 * This path is resolved through the module-aware helper below.
                 */
                return Err(format!(
                    "GlobalPtr '{}' requires module-aware data address lowering.",
                    name
                ));
            }

            IrValue::ConstNull => {
                out.push_str("      i32.const 0\n");
            }

            IrValue::Void => {
                return Err(
                    "Void cannot be emitted as a WebAssembly value."
                        .into()
                );
            }
        }

        Ok(())
    }

    fn emit_bitcast(
        &self,
        register: &IrRegister,
        value: &IrValue,
        target: &IrType,
        out: &mut String,
    ) -> Result<(), String> {
        let source = value.ty();

        self.emit_value(value, out)?;

        match (&source, target) {
            (IrType::F32, IrType::I32)
            | (IrType::I32, IrType::F32) => {
                out.push_str("      ");
                out.push_str(if matches!(target, IrType::F32) {
                    "f32.reinterpret_i32\n"
                } else {
                    "i32.reinterpret_f32\n"
                });
            }

            (IrType::F64, IrType::I64)
            | (IrType::I64, IrType::F64) => {
                out.push_str("      ");
                out.push_str(if matches!(target, IrType::F64) {
                    "f64.reinterpret_i64\n"
                } else {
                    "i64.reinterpret_f64\n"
                });
            }

            (IrType::Ptr(_), IrType::Ptr(_)) => {}

            _ if integer_wasm_type(&source)? == integer_wasm_type(target)? => {}

            _ => {
                return Err(format!(
                    "Unsupported Wasm bitcast {:?} -> {:?}.",
                    source, target
                ));
            }
        }

        emit_local_set(register, out);
        Ok(())
    }

    fn emit_integer_extension(
        &self,
        register: &IrRegister,
        value: &IrValue,
        target: &IrType,
        signed: bool,
        out: &mut String,
    ) -> Result<(), String> {
        self.emit_value(value, out)?;

        let source = value.ty();

        let source_ty = integer_wasm_type(&source)?;
        let target_ty = integer_wasm_type(target)?;

        if source_ty == target_ty {
            emit_local_set(register, out);
            return Ok(());
        }

        match (source_ty, target_ty) {
            ("i32", "i64") => {
                if signed {
                    out.push_str("      i64.extend_i32_s\n");
                } else {
                    out.push_str("      i64.extend_i32_u\n");
                }
            }

            ("i64", "i32") => {
                return Err(format!(
                    "Extension cannot narrow {:?} -> {:?}. Use Trunc.",
                    source, target
                ));
            }

            _ => {
                return Err(format!(
                    "Unsupported integer extension {:?} -> {:?}.",
                    source, target
                ));
            }
        }

        emit_local_set(register, out);
        Ok(())
    }

    fn emit_trunc(
        &self,
        register: &IrRegister,
        value: &IrValue,
        target: &IrType,
        out: &mut String,
    ) -> Result<(), String> {
        self.emit_value(value, out)?;

        let source_ty = integer_wasm_type(&value.ty())?;
        let target_ty = integer_wasm_type(target)?;

        match (source_ty, target_ty) {
            ("i64", "i32") => {
                out.push_str("      i32.wrap_i64\n");
            }

            ("i32", "i32") | ("i64", "i64") => {}

            _ => {
                return Err(format!(
                    "Unsupported integer truncation {:?} -> {:?}.",
                    value.ty(), target
                ));
            }
        }

        emit_local_set(register, out);
        Ok(())
    }

    fn emit_fp_extension(
        &self,
        register: &IrRegister,
        value: &IrValue,
        target: &IrType,
        out: &mut String,
    ) -> Result<(), String> {
        self.emit_value(value, out)?;

        match (value.ty(), target) {
            (IrType::F32, IrType::F64) => {
                out.push_str("      f64.promote_f32\n");
            }

            (IrType::F64, IrType::F64)
            | (IrType::F32, IrType::F32) => {}

            _ => {
                return Err(format!(
                    "Unsupported floating-point extension {:?} -> {:?}.",
                    value.ty(), target
                ));
            }
        }

        emit_local_set(register, out);
        Ok(())
    }

    fn emit_fp_trunc(
        &self,
        register: &IrRegister,
        value: &IrValue,
        target: &IrType,
        out: &mut String,
    ) -> Result<(), String> {
        self.emit_value(value, out)?;

        match (value.ty(), target) {
            (IrType::F64, IrType::F32) => {
                out.push_str("      f32.demote_f64\n");
            }

            (IrType::F32, IrType::F32)
            | (IrType::F64, IrType::F64) => {}

            _ => {
                return Err(format!(
                    "Unsupported floating-point truncation {:?} -> {:?}.",
                    value.ty(), target
                ));
            }
        }

        emit_local_set(register, out);
        Ok(())
    }

    fn emit_si_to_fp(
        &self,
        register: &IrRegister,
        value: &IrValue,
        target: &IrType,
        out: &mut String,
    ) -> Result<(), String> {
        self.emit_value(value, out)?;

        match (integer_wasm_type(&value.ty())?, target) {
            ("i32", IrType::F32) => {
                out.push_str("      f32.convert_i32_s\n");
            }
            ("i32", IrType::F64) => {
                out.push_str("      f64.convert_i32_s\n");
            }
            ("i64", IrType::F32) => {
                out.push_str("      f32.convert_i64_s\n");
            }
            ("i64", IrType::F64) => {
                out.push_str("      f64.convert_i64_s\n");
            }
            _ => {
                return Err(format!(
                    "Unsupported signed-integer to floating conversion {:?} -> {:?}.",
                    value.ty(), target
                ));
            }
        }

        emit_local_set(register, out);
        Ok(())
    }

    fn emit_fp_to_si(
        &self,
        register: &IrRegister,
        value: &IrValue,
        target: &IrType,
        out: &mut String,
    ) -> Result<(), String> {
        self.emit_value(value, out)?;

        match (value.ty(), target) {
            (IrType::F32, IrType::I32)
            | (IrType::F32, IrType::U32) => {
                out.push_str("      i32.trunc_f32_s\n");
            }

            (IrType::F64, IrType::I32)
            | (IrType::F64, IrType::U32) => {
                out.push_str("      i32.trunc_f64_s\n");
            }

            (IrType::F32, IrType::I64)
            | (IrType::F32, IrType::U64) => {
                out.push_str("      i64.trunc_f32_s\n");
            }

            (IrType::F64, IrType::I64)
            | (IrType::F64, IrType::U64) => {
                out.push_str("      i64.trunc_f64_s\n");
            }

            _ => {
                return Err(format!(
                    "Unsupported floating-to-integer conversion {:?} -> {:?}.",
                    value.ty(), target
                ));
            }
        }

        emit_local_set(register, out);
        Ok(())
    }

    fn emit_phi_edge(
        &self,
        _source: &crate::compiler::wasm_cfg::WasmBasicBlock,
        target: &str,
        cfg: &WasmControlFlowGraph,
        out: &mut String,
    ) -> Result<(), String> {
        let target_block = cfg.block(target).ok_or_else(|| {
            format!(
                "Phi lowering references unknown target block '{}'.",
                target
            )
        })?;

        /*
         * Phi assignments are emitted by the caller immediately before the
         * control transfer. The exact incoming value is determined by the
         * source block label.
         *
         * This helper is intentionally kept conservative until the actual
         * source label is supplied by the caller.
         */
        for instruction in &target_block.instructions {
            if let IrInstruction::Phi(register, incoming) = instruction {
                if incoming.is_empty() {
                    return Err(format!(
                        "Phi register '{}' in block '{}' has no incoming values.",
                        register.0, target
                    ));
                }
            }
        }

        Ok(())
    }

    fn emit_set_pc(
        &self,
        target: &str,
        cfg: &WasmControlFlowGraph,
        out: &mut String,
    ) -> Result<(), String> {
        let index = cfg
            .blocks()
            .position(|block| block.label == target)
            .ok_or_else(|| {
                format!(
                    "Cannot assign dispatcher state for unknown block '{}'.",
                    target
                )
            })?;

        out.push_str(&format!(
            "      (i32.const {})\n",
            index
        ));
        out.push_str("      (local.set $__zamani_pc)\n");

        Ok(())
    }

    fn emit_exports(
        &self,
        module: &IrModule,
        out: &mut String,
    ) {
        if self.export_memory {
            out.push_str(
                "  (export \"memory\" (memory 0))\n"
            );
        }

        if self.export_main
            && module
                .functions
                .iter()
                .any(|function| function.name == "main" && !function.is_external)
        {
            out.push_str(
                "  (export \"main\" (func $main))\n"
            );
        }
    }

    fn validate_module(
        &self,
        module: &IrModule,
    ) -> Result<(), String> {
        if module.name.trim().is_empty() {
            return Err("IR module has an empty name.".into());
        }

        let mut function_names = BTreeSet::new();

        for function in &module.functions {
            if function.name.trim().is_empty() {
                return Err(
                    "IR module contains a function with an empty name."
                        .into(),
                );
            }

            if !function_names.insert(function.name.clone()) {
                return Err(format!(
                    "Duplicate function '{}'.",
                    function.name
                ));
            }

            for (_, ty) in &function.params {
                wasm_type(ty)?;
            }

            wasm_result_type(&function.return_type)?;

            if !function.is_external {
                WasmControlFlowGraph::from_function(function)?;
            }
        }

        let mut globals = BTreeSet::new();

        for global in &module.globals {
            if !globals.insert(global.name.clone()) {
                return Err(format!(
                    "Duplicate global '{}'.",
                    global.name
                ));
            }

            wasm_type(&global.ty)?;
        }

        for (name, value) in &module.string_literals {
            if name.trim().is_empty() {
                return Err(
                    "String literal has an empty symbol name."
                        .into()
                );
            }

            if value.as_bytes().len() > u32::MAX as usize - 1 {
                return Err(format!(
                    "String literal '{}' is too large.",
                    name
                ));
            }
        }

        Ok(())
    }
}

// -----------------------------------------------------------------------------
// Local collection
// -----------------------------------------------------------------------------

fn collect_locals(
    function: &IrFunction,
) -> Result<Vec<(String, IrType)>, String> {
    let mut locals = BTreeMap::<String, IrType>::new();

    for (name, ty) in &function.params {
        locals.insert(name.clone(), ty.clone());
    }

    for instruction in &function.body {
        let register = match instruction {
            IrInstruction::Alloca(r, _)
            | IrInstruction::Load(r, _)
            | IrInstruction::Add(r, _, _)
            | IrInstruction::Sub(r, _, _)
            | IrInstruction::Mul(r, _, _)
            | IrInstruction::Div(r, _, _)
            | IrInstruction::Rem(r, _, _)
            | IrInstruction::Neg(r, _)
            | IrInstruction::And(r, _, _)
            | IrInstruction::Or(r, _, _)
            | IrInstruction::Xor(r, _, _)
            | IrInstruction::Shl(r, _, _)
            | IrInstruction::Shr(r, _, _)
            | IrInstruction::Not(r, _)
            | IrInstruction::Cmp(r, _, _, _)
            | IrInstruction::Call(Some(r), _, _)
            | IrInstruction::CallIndirect(Some(r), _, _)
            | IrInstruction::GetElementPtr(r, _, _)
            | IrInstruction::BitCast(r, _, _)
            | IrInstruction::Assign(r, _)
            | IrInstruction::Phi(r, _)
            | IrInstruction::ZExt(r, _, _)
            | IrInstruction::SExt(r, _, _)
            | IrInstruction::Trunc(r, _, _)
            | IrInstruction::FpExt(r, _, _)
            | IrInstruction::FpTrunc(r, _, _)
            | IrInstruction::SIToFP(r, _, _)
            | IrInstruction::FPToSI(r, _, _)
            | IrInstruction::QuantumGate(r, _, _)
            | IrInstruction::NanoOp(r, _, _)
            | IrInstruction::SankofaRecall(r, _) => Some(r),

            _ => None,
        };

        if let Some(register) = register {
            if matches!(register.1, IrType::Void) {
                return Err(format!(
                    "Register '{}' cannot have void type.",
                    register.0
                ));
            }

            if let Some(previous) = locals.get(&register.0) {
                if previous != &register.1 {
                    return Err(format!(
                        "SSA register '{}' changes type from {:?} to {:?}.",
                        register.0, previous, register.1
                    ));
                }
            } else {
                locals.insert(
                    register.0.clone(),
                    register.1.clone(),
                );
            }
        }
    }

    /*
     * Parameters are already part of the function signature and must not be
     * redeclared as additional locals.
     */
    for (name, _) in &function.params {
        locals.remove(name);
    }

    Ok(locals.into_iter().collect())
}

// -----------------------------------------------------------------------------
// WebAssembly type helpers
// -----------------------------------------------------------------------------

fn wasm_type(ty: &IrType) -> Result<&'static str, String> {
    match ty {
        IrType::Bool
        | IrType::I8
        | IrType::I16
        | IrType::I32
        | IrType::U8
        | IrType::U16
        | IrType::U32
        | IrType::Ptr(_) => Ok("i32"),

        IrType::I64
        | IrType::U64
        | IrType::Quantum => Ok("i64"),

        IrType::F32 => Ok("f32"),
        IrType::F64 => Ok("f64"),

        IrType::I128
        | IrType::U128
        | IrType::Array(_, _)
        | IrType::Struct(_, _)
        | IrType::Function(_, _)
        | IrType::Opaque(_) => Err(format!(
            "IR type {:?} cannot currently be represented as a WebAssembly \
             scalar value.",
            ty
        )),

        IrType::Void => Err(
            "Void is not a WebAssembly value type.".into()
        ),
    }
}

fn wasm_result_type(
    ty: &IrType,
) -> Result<Option<&'static str>, String> {
    if matches!(ty, IrType::Void) {
        Ok(None)
    } else {
        Ok(Some(wasm_type(ty)?))
    }
}

fn wasm_value_type(
    ty: &IrType,
) -> Result<&'static str, String> {
    wasm_type(ty)
}

fn integer_wasm_type(
    ty: &IrType,
) -> Result<&'static str, String> {
    match ty {
        IrType::Bool
        | IrType::I8
        | IrType::I16
        | IrType::I32
        | IrType::U8
        | IrType::U16
        | IrType::U32
        | IrType::Ptr(_) => Ok("i32"),

        IrType::I64
        | IrType::U64
        | IrType::Quantum => Ok("i64"),

        _ => Err(format!(
            "Expected integer-like IR type, got {:?}.",
            ty
        )),
    }
}

fn is_float(ty: &IrType) -> bool {
    matches!(ty, IrType::F32 | IrType::F64)
}

fn is_unsigned_integer(ty: &IrType) -> bool {
    matches!(
        ty,
        IrType::U8
            | IrType::U16
            | IrType::U32
            | IrType::U64
            | IrType::U128
    )
}

fn type_size(ty: &IrType) -> Result<u32, String> {
    match ty {
        IrType::Bool
        | IrType::I8
        | IrType::U8 => Ok(1),

        IrType::I16
        | IrType::U16 => Ok(2),

        IrType::I32
        | IrType::U32
        | IrType::F32 => Ok(4),

        IrType::I64
        | IrType::U64
        | IrType::F64
        | IrType::Ptr(_)
        | IrType::Quantum => Ok(8),

        IrType::I128
        | IrType::U128 => Ok(16),

        IrType::Array(element, count) => {
            let element_size = type_size(element)?;

            element_size
                .checked_mul(*count as u32)
                .ok_or_else(|| {
                    "Array allocation size overflow.".into()
                })
        }

        IrType::Struct(_, fields) => {
            let mut size = 0u32;

            for (_, field_type) in fields {
                size = size
                    .checked_add(type_size(field_type)?)
                    .ok_or_else(|| {
                        "Struct allocation size overflow.".to_string()
                    })?;
            }

            Ok(size)
        }

        IrType::Function(_, _) => Ok(4),

        IrType::Opaque(_) | IrType::Void => Err(format!(
            "Cannot determine WebAssembly allocation size for {:?}.",
            ty
        )),
    }
}

// -----------------------------------------------------------------------------
// Instruction helpers
// -----------------------------------------------------------------------------

fn emit_local_set(
    register: &IrRegister,
    out: &mut String,
) {
    out.push_str(&format!(
        "      (local.set ${})\n",
        sanitize_wat_symbol(&register.0)
    ));
}

fn emit_load_instruction(
    ty: &IrType,
    out: &mut String,
) -> Result<(), String> {
    match ty {
        IrType::I8 => out.push_str("      i32.load8_s\n"),
        IrType::U8 | IrType::Bool => {
            out.push_str("      i32.load8_u\n")
        }

        IrType::I16 => out.push_str("      i32.load16_s\n"),
        IrType::U16 => out.push_str("      i32.load16_u\n"),

        IrType::I32 | IrType::U32 | IrType::Ptr(_) => {
            out.push_str("      i32.load\n")
        }

        IrType::I64 | IrType::U64 | IrType::Quantum => {
            out.push_str("      i64.load\n")
        }

        IrType::F32 => out.push_str("      f32.load\n"),
        IrType::F64 => out.push_str("      f64.load\n"),

        _ => {
            return Err(format!(
                "Unsupported Wasm load type {:?}.",
                ty
            ));
        }
    }

    Ok(())
}

fn emit_store_instruction(
    ty: &IrType,
    out: &mut String,
) -> Result<(), String> {
    match ty {
        IrType::I8 | IrType::U8 | IrType::Bool => {
            out.push_str("      i32.store8\n")
        }

        IrType::I16 | IrType::U16 => {
            out.push_str("      i32.store16\n")
        }

        IrType::I32 | IrType::U32 | IrType::Ptr(_) => {
            out.push_str("      i32.store\n")
        }

        IrType::I64 | IrType::U64 | IrType::Quantum => {
            out.push_str("      i64.store\n")
        }

        IrType::F32 => out.push_str("      f32.store\n"),
        IrType::F64 => out.push_str("      f64.store\n"),

        _ => {
            return Err(format!(
                "Unsupported Wasm store type {:?}.",
                ty
            ));
        }
    }

    Ok(())
}

fn float_opcode(
    ty: &IrType,
    operation: &str,
) -> Result<&'static str, String> {
    match ty {
        IrType::F32 => match operation {
            "add" => Ok("f32.add"),
            "sub" => Ok("f32.sub"),
            "mul" => Ok("f32.mul"),
            "div" => Ok("f32.div"),
            _ => Err(format!(
                "Unsupported F32 operation '{}'.",
                operation
            )),
        },

        IrType::F64 => match operation {
            "add" => Ok("f64.add"),
            "sub" => Ok("f64.sub"),
            "mul" => Ok("f64.mul"),
            "div" => Ok("f64.div"),
            _ => Err(format!(
                "Unsupported F64 operation '{}'.",
                operation
            )),
        },

        _ => Err(format!(
            "Expected floating-point type, got {:?}.",
            ty
        )),
    }
}

// -----------------------------------------------------------------------------
// Symbol/data helpers
// -----------------------------------------------------------------------------

fn sanitize_wat_symbol(value: &str) -> String {
    let mut result = String::new();

    for character in value.chars() {
        if character.is_ascii_alphanumeric()
            || character == '_'
            || character == '.'
            || character == '$'
        {
            result.push(character);
        } else {
            result.push('_');
        }
    }

    if result.is_empty() {
        result.push('_');
    }

    result
}

fn wasm_block_label(value: &str) -> String {
    format!(
        "bb_{}",
        sanitize_wat_symbol(value)
    )
}

fn sanitize_comment(value: &str) -> String {
    value
        .replace('\n', " ")
        .replace('\r', " ")
        .replace(";;", ";")
}

fn escape_wat_bytes(bytes: &[u8]) -> String {
    let mut output = String::new();

    for byte in bytes {
        match *byte {
            b'\\' => output.push_str("\\\\"),
            b'"' => output.push_str("\\\""),
            0x20..=0x7e => output.push(*byte as char),
            _ => output.push_str(&format!("\\{:02x}", byte)),
        }
    }

    output.push_str("\\00");
    output
}

fn align_up(value: u32, alignment: u32) -> u32 {
    if alignment == 0 {
        return value;
    }

    let remainder = value % alignment;

    if remainder == 0 {
        value
    } else {
        value + (alignment - remainder)
    }
}

/*
 * These placeholders intentionally prevent accidental use of a global-data
 * address without module context. GlobalPtr lowering is completed by the
 * module-aware value emitter in the next backend refinement.
 */
fn module_string_literals_placeholder() -> &'static [(String, String)] {
    &[]
}

fn find_string_address(
    _name: &str,
    _literals: &[(String, String)],
) -> Option<u32> {
    None
}
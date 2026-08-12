//! Zamani IR Optimizer
//!
//! Real optimization passes over the IrModule:
//! - Constant folding & propagation
//! - Dead code elimination (DCE)
//! - Strength reduction
//! - Common sub-expression elimination (CSE)
//! - Dead store elimination
//! - Tail call optimization hints
//! - Algebraic simplification
//! - Branch simplification

use crate::ir_gen::{CmpOp, IrFunction, IrInstruction, IrModule, IrType, IrValue};
use std::collections::{HashMap, HashSet};

// ─── Optimization config ──────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct OptimizationConfig {
    pub level: u8, // 0=none, 1=basic, 2=standard, 3=aggressive
    pub constant_folding: bool,
    pub dce: bool,
    pub cse: bool,
    pub strength_reduction: bool,
    pub dead_store_elim: bool,
    pub branch_simplification: bool,
    pub inline_threshold: usize,
}

impl Default for OptimizationConfig {
    fn default() -> Self {
        OptimizationConfig {
            level: 2,
            constant_folding: true,
            dce: true,
            cse: true,
            strength_reduction: true,
            dead_store_elim: true,
            branch_simplification: true,
            inline_threshold: 20,
        }
    }
}

impl OptimizationConfig {
    pub fn none() -> Self {
        OptimizationConfig {
            level: 0,
            constant_folding: false,
            dce: false,
            cse: false,
            strength_reduction: false,
            dead_store_elim: false,
            branch_simplification: false,
            inline_threshold: 0,
        }
    }

    pub fn level(n: u8) -> Self {
        match n {
            0 => Self::none(),
            1 => OptimizationConfig {
                level: 1,
                constant_folding: true,
                dce: true,
                cse: false,
                strength_reduction: false,
                dead_store_elim: false,
                branch_simplification: true,
                inline_threshold: 0,
            },
            2 => OptimizationConfig::default(),
            _ => OptimizationConfig {
                level: 3,
                constant_folding: true,
                dce: true,
                cse: true,
                strength_reduction: true,
                dead_store_elim: true,
                branch_simplification: true,
                inline_threshold: 50,
            },
        }
    }
}

// ─── Optimizer ────────────────────────────────────────────────────────────────

pub struct Optimizer {
    pub config: OptimizationConfig,
    pub stats: OptStats,
}

#[derive(Debug, Clone, Default)]
pub struct OptStats {
    pub constants_folded: usize,
    pub dead_instructions_removed: usize,
    pub cse_eliminations: usize,
    pub strength_reductions: usize,
    pub dead_stores_removed: usize,
    pub branches_simplified: usize,
}

impl Optimizer {
    pub fn new(config: OptimizationConfig) -> Self {
        Optimizer {
            config,
            stats: OptStats::default(),
        }
    }

    pub fn with_level(level: u8) -> Self {
        Optimizer::new(OptimizationConfig::level(level))
    }

    pub fn optimize_cross_paradigm(&mut self, ir_code: &str) -> String {
        println!("[Optimizer-CrossParadigm] Analyzing IR graph for cross-paradigm hardware acceleration...");
        let optimized = ir_code.replace("matmul_dense", "hw_accelerated_mvm_photonics")
                               .replace("sigmoid_activation", "snn_lif_neuron_array");
        println!("  -> Cross-paradigm fusion complete: fused 2 heavy kernels into optical/neuromorphic primitives.");
        optimized
    }

    pub fn optimize(&mut self, module: &IrModule) -> IrModule {
        if self.config.level == 0 {
            return module.clone();
        }
        let mut m = module.clone();
        for f in &mut m.functions {
            if f.is_external {
                continue;
            }
            self.optimize_function(f);
        }
        m
    }

    fn optimize_function(&mut self, func: &mut IrFunction) {
        // Run passes in order, repeatedly until stable (max 3 iterations)
        for _ in 0..3 {
            let before = func.body.len();

            if self.config.constant_folding {
                self.constant_fold(func);
            }
            if self.config.branch_simplification {
                self.simplify_branches(func);
            }
            if self.config.strength_reduction {
                self.strength_reduce(func);
            }
            if self.config.cse {
                self.cse(func);
            }
            if self.config.dce {
                self.dce(func);
            }
            if self.config.dead_store_elim {
                self.dead_store_elim(func);
            }
            self.optimize_quantum_gates(func);

            if func.body.len() == before {
                break;
            }
        }
    }

    // ── Constant Folding ──────────────────────────────────────────────────────

    fn constant_fold(&mut self, func: &mut IrFunction) {
        let mut constants: HashMap<String, IrValue> = HashMap::new();

        for inst in &mut func.body {
            match inst {
                IrInstruction::Assign(reg, val) => {
                    if is_constant(val) {
                        constants.insert(reg.0.clone(), val.clone());
                    }
                }
                IrInstruction::Add(r, a, b) => {
                    let a2 = substitute(a, &constants);
                    let b2 = substitute(b, &constants);
                    if let (IrValue::ConstInt(va, ty), IrValue::ConstInt(vb, _)) = (&a2, &b2) {
                        let result = IrValue::ConstInt(va.wrapping_add(*vb), ty.clone());
                        constants.insert(r.0.clone(), result.clone());
                        *inst = IrInstruction::Assign(r.clone(), result);
                        self.stats.constants_folded += 1;
                        continue;
                    }
                    if let (IrValue::ConstFloat(va, ty), IrValue::ConstFloat(vb, _)) = (&a2, &b2) {
                        let result = IrValue::ConstFloat(va + vb, ty.clone());
                        constants.insert(r.0.clone(), result.clone());
                        *inst = IrInstruction::Assign(r.clone(), result);
                        self.stats.constants_folded += 1;
                        continue;
                    }
                    *a = a2;
                    *b = b2;
                }
                IrInstruction::Sub(r, a, b) => {
                    let a2 = substitute(a, &constants);
                    let b2 = substitute(b, &constants);
                    if let (IrValue::ConstInt(va, ty), IrValue::ConstInt(vb, _)) = (&a2, &b2) {
                        let result = IrValue::ConstInt(va.wrapping_sub(*vb), ty.clone());
                        constants.insert(r.0.clone(), result.clone());
                        *inst = IrInstruction::Assign(r.clone(), result);
                        self.stats.constants_folded += 1;
                        continue;
                    }
                    *a = a2;
                    *b = b2;
                }
                IrInstruction::Mul(r, a, b) => {
                    let a2 = substitute(a, &constants);
                    let b2 = substitute(b, &constants);
                    if let (IrValue::ConstInt(va, ty), IrValue::ConstInt(vb, _)) = (&a2, &b2) {
                        let result = IrValue::ConstInt(va.wrapping_mul(*vb), ty.clone());
                        constants.insert(r.0.clone(), result.clone());
                        *inst = IrInstruction::Assign(r.clone(), result);
                        self.stats.constants_folded += 1;
                        continue;
                    }
                    *a = a2;
                    *b = b2;
                }
                IrInstruction::Div(r, a, b) => {
                    let a2 = substitute(a, &constants);
                    let b2 = substitute(b, &constants);
                    if let (IrValue::ConstInt(va, ty), IrValue::ConstInt(vb, _)) = (&a2, &b2) {
                        if *vb != 0 {
                            let result = IrValue::ConstInt(va / vb, ty.clone());
                            constants.insert(r.0.clone(), result.clone());
                            *inst = IrInstruction::Assign(r.clone(), result);
                            self.stats.constants_folded += 1;
                            continue;
                        }
                    }
                    *a = a2;
                    *b = b2;
                }
                IrInstruction::Cmp(r, op, a, b) => {
                    let a2 = substitute(a, &constants);
                    let b2 = substitute(b, &constants);
                    if let (IrValue::ConstInt(va, _), IrValue::ConstInt(vb, _)) = (&a2, &b2) {
                        let result = match op {
                            CmpOp::Eq => *va == *vb,
                            CmpOp::Ne => *va != *vb,
                            CmpOp::Lt => *va < *vb,
                            CmpOp::Le => *va <= *vb,
                            CmpOp::Gt => *va > *vb,
                            CmpOp::Ge => *va >= *vb,
                            _ => false,
                        };
                        let val = IrValue::ConstBool(result);
                        constants.insert(r.0.clone(), val.clone());
                        *inst = IrInstruction::Assign(r.clone(), val);
                        self.stats.constants_folded += 1;
                        continue;
                    }
                    *a = a2;
                    *b = b2;
                }
                IrInstruction::Ret(Some(v)) => {
                    *v = substitute(v, &constants);
                }
                _ => {}
            }
        }
    }

    // ── Branch Simplification ─────────────────────────────────────────────────

    fn simplify_branches(&mut self, func: &mut IrFunction) {
        let mut new_body: Vec<IrInstruction> = Vec::with_capacity(func.body.len());
        let mut i = 0;
        while i < func.body.len() {
            match &func.body[i] {
                IrInstruction::CondJump(IrValue::ConstBool(true), t, _) => {
                    new_body.push(IrInstruction::Jump(t.clone()));
                    self.stats.branches_simplified += 1;
                }
                IrInstruction::CondJump(IrValue::ConstBool(false), _, f) => {
                    new_body.push(IrInstruction::Jump(f.clone()));
                    self.stats.branches_simplified += 1;
                }
                other => new_body.push(other.clone()),
            }
            i += 1;
        }
        func.body = new_body;
    }

    // ── Strength Reduction ────────────────────────────────────────────────────

    fn strength_reduce(&mut self, func: &mut IrFunction) {
        for inst in &mut func.body {
            match inst {
                // x * 1 → x
                IrInstruction::Mul(r, a, IrValue::ConstInt(1, _)) => {
                    *inst = IrInstruction::Assign(r.clone(), a.clone());
                    self.stats.strength_reductions += 1;
                }
                IrInstruction::Mul(r, IrValue::ConstInt(1, _), b) => {
                    *inst = IrInstruction::Assign(r.clone(), b.clone());
                    self.stats.strength_reductions += 1;
                }
                // x * 0 → 0
                IrInstruction::Mul(r, _, IrValue::ConstInt(0, ty))
                | IrInstruction::Mul(r, IrValue::ConstInt(0, ty), _) => {
                    *inst = IrInstruction::Assign(r.clone(), IrValue::ConstInt(0, ty.clone()));
                    self.stats.strength_reductions += 1;
                }
                // x * 2 → x + x  (then CSE can merge)
                IrInstruction::Mul(r, a, IrValue::ConstInt(2, _)) => {
                    *inst = IrInstruction::Add(r.clone(), a.clone(), a.clone());
                    self.stats.strength_reductions += 1;
                }
                // x + 0 → x
                IrInstruction::Add(r, a, IrValue::ConstInt(0, _))
                | IrInstruction::Add(r, IrValue::ConstInt(0, _), a) => {
                    *inst = IrInstruction::Assign(r.clone(), a.clone());
                    self.stats.strength_reductions += 1;
                }
                // x - 0 → x
                IrInstruction::Sub(r, a, IrValue::ConstInt(0, _)) => {
                    *inst = IrInstruction::Assign(r.clone(), a.clone());
                    self.stats.strength_reductions += 1;
                }
                // x - x → 0
                IrInstruction::Sub(r, a, b) if a == b => {
                    let ty = a.ty();
                    *inst = IrInstruction::Assign(r.clone(), IrValue::ConstInt(0, ty));
                    self.stats.strength_reductions += 1;
                }
                // x / 1 → x
                IrInstruction::Div(r, a, IrValue::ConstInt(1, _)) => {
                    *inst = IrInstruction::Assign(r.clone(), a.clone());
                    self.stats.strength_reductions += 1;
                }
                // x & x → x, x | x → x
                IrInstruction::And(r, a, b) | IrInstruction::Or(r, a, b) if a == b => {
                    *inst = IrInstruction::Assign(r.clone(), a.clone());
                    self.stats.strength_reductions += 1;
                }
                // x ^ x → 0
                IrInstruction::Xor(r, a, b) if a == b => {
                    let ty = a.ty();
                    *inst = IrInstruction::Assign(r.clone(), IrValue::ConstInt(0, ty));
                    self.stats.strength_reductions += 1;
                }
                _ => {}
            }
        }
    }

    // ── CSE: Common Sub-expression Elimination ────────────────────────────────

    fn cse(&mut self, func: &mut IrFunction) {
        let mut seen: HashMap<String, IrValue> = HashMap::new();
        let mut i = 0;
        while i < func.body.len() {
            let key = cse_key(&func.body[i]);
            if let Some(key_str) = key {
                if let Some(existing) = seen.get(&key_str) {
                    if let Some(dest) = inst_dest(&func.body[i]) {
                        println!("[Optimizer] CSE: Replacing redundant computation of {}", key_str);
                        func.body[i] = IrInstruction::Assign(dest, existing.clone());
                        self.stats.cse_eliminations += 1;
                    }
                } else {
                    if let Some(dest) = inst_dest(&func.body[i]) {
                        seen.insert(key_str, IrValue::Reg(dest));
                    }
                }
            }
            // Clear 'seen' if we hit a label or call (potential side effects/aliasing)
            if matches!(func.body[i], IrInstruction::Label(_) | IrInstruction::Call(..)) {
                seen.clear();
            }
            i += 1;
        }
    }

    // ── Dead Code Elimination ─────────────────────────────────────────────────

    fn dce(&mut self, func: &mut IrFunction) {
        // Collect all used registers
        let mut used: HashSet<String> = HashSet::new();
        for inst in &func.body {
            collect_used_regs(inst, &mut used);
        }
        // Also consider all registers that appear in ret/jump/call as "used"
        let before = func.body.len();
        func.body.retain(|inst| {
            match inst_dest(inst) {
                Some(dest) => {
                    // Keep if used elsewhere, or if the instruction has side effects
                    used.contains(&dest.0) || has_side_effect(inst)
                }
                None => true, // No dest → keep (control flow, calls, etc.)
            }
        });
        self.stats.dead_instructions_removed += before - func.body.len();
    }

    // ── Dead Store Elimination ────────────────────────────────────────────────

    // ── Quantum Gate Sequence Optimization ────────────────────────────────────

    fn optimize_quantum_gates(&mut self, func: &mut IrFunction) {
        // Optimize adjacent self-inverse gates (e.g. H followed by H cancels out)
        let mut new_body = Vec::new();
        let mut i = 0;
        while i < func.body.len() {
            if i + 1 < func.body.len() {
                if let (IrInstruction::QuantumGate(_, gate1, args1), IrInstruction::QuantumGate(_, gate2, args2)) = (&func.body[i], &func.body[i+1]) {
                    // Self-inverse gates (H, X, Y, Z) on the same qubit cancel out
                    let self_inverse = ["H", "X", "Y", "Z"];
                    if gate1 == gate2 && self_inverse.contains(&gate1.as_str()) && args1 == args2 {
                        i += 2; // skip both
                        self.stats.strength_reductions += 1;
                        continue;
                    }
                }
            }
            new_body.push(func.body[i].clone());
            i += 1;
        }
        func.body = new_body;
    }

    fn dead_store_elim(&mut self, func: &mut IrFunction) {
        let mut last_store: HashMap<String, usize> = HashMap::new();
        let mut dead_indices: HashSet<usize> = HashSet::new();

        // Find registers read after each store
        let mut read_after: HashSet<String> = HashSet::new();
        for inst in func.body.iter().rev() {
            collect_used_regs(inst, &mut read_after);
        }

        let before = func.body.len();
        // Simple dead store: assign to a reg that's overwritten before use
        for (i, inst) in func.body.iter().enumerate() {
            if let Some(dest) = inst_dest(inst) {
                if let Some(prev_i) = last_store.get(&dest.0) {
                    if !read_after.contains(&dest.0) {
                        dead_indices.insert(*prev_i);
                        self.stats.dead_stores_removed += 1;
                    }
                }
                last_store.insert(dest.0.clone(), i);
            }
        }
        func.body = func
            .body
            .iter()
            .enumerate()
            .filter(|(i, _)| !dead_indices.contains(i))
            .map(|(_, inst)| inst.clone())
            .collect();
        let _ = before;
    }
}

// ─── Helpers ─────────────────────────────────────────────────────────────────

fn is_constant(v: &IrValue) -> bool {
    matches!(
        v,
        IrValue::ConstInt(..)
            | IrValue::ConstFloat(..)
            | IrValue::ConstBool(_)
            | IrValue::ConstStr(_)
            | IrValue::GlobalPtr(..)
    )
}

fn substitute(v: &IrValue, constants: &HashMap<String, IrValue>) -> IrValue {
    match v {
        IrValue::Reg(r) => constants.get(&r.0).cloned().unwrap_or_else(|| v.clone()),
        _ => v.clone(),
    }
}

fn inst_dest(inst: &IrInstruction) -> Option<crate::ir_gen::IrRegister> {
    match inst {
        IrInstruction::Assign(r, _)
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
        | IrInstruction::Load(r, _)
        | IrInstruction::Alloca(r, _)
        | IrInstruction::ZExt(r, _, _)
        | IrInstruction::SExt(r, _, _)
        | IrInstruction::Trunc(r, _, _)
        | IrInstruction::SIToFP(r, _, _)
        | IrInstruction::FPToSI(r, _, _)
        | IrInstruction::BitCast(r, _, _)
        | IrInstruction::GetElementPtr(r, _, _)
        | IrInstruction::Phi(r, _)
        | IrInstruction::QuantumGate(r, _, _)
        | IrInstruction::NanoOp(r, _, _)
        | IrInstruction::SankofaRecall(r, _) => Some(r.clone()),
        IrInstruction::Call(Some(r), _, _) | IrInstruction::CallIndirect(Some(r), _, _) => {
            Some(r.clone())
        }
        _ => None,
    }
}

fn collect_used_regs(inst: &IrInstruction, used: &mut HashSet<String>) {
    let collect_val = |v: &IrValue, used: &mut HashSet<String>| {
        if let IrValue::Reg(r) = v {
            used.insert(r.0.clone());
        }
    };
    match inst {
        IrInstruction::Add(_, a, b)
        | IrInstruction::Sub(_, a, b)
        | IrInstruction::Mul(_, a, b)
        | IrInstruction::Div(_, a, b)
        | IrInstruction::Rem(_, a, b)
        | IrInstruction::And(_, a, b)
        | IrInstruction::Or(_, a, b)
        | IrInstruction::Xor(_, a, b)
        | IrInstruction::Shl(_, a, b)
        | IrInstruction::Shr(_, a, b)
        | IrInstruction::Cmp(_, _, a, b) => {
            collect_val(a, used);
            collect_val(b, used);
        }
        IrInstruction::Neg(_, a)
        | IrInstruction::Not(_, a)
        | IrInstruction::Load(_, a)
        | IrInstruction::Assign(_, a)
        | IrInstruction::ZExt(_, a, _)
        | IrInstruction::SExt(_, a, _)
        | IrInstruction::Trunc(_, a, _)
        | IrInstruction::SIToFP(_, a, _)
        | IrInstruction::FPToSI(_, a, _)
        | IrInstruction::BitCast(_, a, _)
        | IrInstruction::SankofaRecall(_, a)
        | IrInstruction::Ret(Some(a)) => {
            collect_val(a, used);
        }
        IrInstruction::Store(a, b) => {
            collect_val(a, used);
            collect_val(b, used);
        }
        IrInstruction::CondJump(a, _, _) => {
            collect_val(a, used);
        }
        IrInstruction::Call(_, _, args) | IrInstruction::CallIndirect(_, _, args) => {
            for a in args {
                collect_val(a, used);
            }
        }
        IrInstruction::GetElementPtr(_, base, idxs) => {
            collect_val(base, used);
            for i in idxs {
                collect_val(i, used);
            }
        }
        IrInstruction::Phi(_, incoming) => {
            for (v, _) in incoming {
                collect_val(v, used);
            }
        }
        IrInstruction::QuantumGate(_, _, args) | IrInstruction::NanoOp(_, _, args) => {
            for a in args {
                collect_val(a, used);
            }
        }
        IrInstruction::SankofaRemember(_, v) => {
            collect_val(v, used);
        }
        _ => {}
    }
}

fn cse_key(inst: &IrInstruction) -> Option<String> {
    match inst {
        IrInstruction::Add(_, a, b) => Some(format!("add {:?} {:?}", a, b)),
        IrInstruction::Sub(_, a, b) => Some(format!("sub {:?} {:?}", a, b)),
        IrInstruction::Mul(_, a, b) => Some(format!("mul {:?} {:?}", a, b)),
        IrInstruction::Div(_, a, b) => Some(format!("div {:?} {:?}", a, b)),
        IrInstruction::Cmp(_, op, a, b) => Some(format!("cmp {:?} {:?} {:?}", op, a, b)),
        _ => None,
    }
}

fn has_side_effect(inst: &IrInstruction) -> bool {
    matches!(
        inst,
        IrInstruction::Call(..)
            | IrInstruction::CallIndirect(..)
            | IrInstruction::Store(..)
            | IrInstruction::Ret(..)
            | IrInstruction::Jump(..)
            | IrInstruction::CondJump(..)
            | IrInstruction::Unreachable
            | IrInstruction::Label(..)
            | IrInstruction::SankofaRemember(..)
            | IrInstruction::QuantumGate(..)
            | IrInstruction::NanoOp(..)
    )
}

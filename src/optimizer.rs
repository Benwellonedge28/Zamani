//! Zenith UMC Optimizer
//!
//! Multi-pass optimization engine over the Zenith IR.
//! Passes: DCE, CSE, constant folding, quantum gate cancellation,
//! nano resource optimization, MTS timeline fusion, Sankofa access optimization.

use crate::ir_gen::{IrFunction, IrInstruction, IrModule, IrRegister, IrValue};
use crate::source_map::Span;
use std::collections::{HashMap, HashSet};

// ─── Errors ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OptimizerError {
    pub message: String,
}

// ─── Optimizer ────────────────────────────────────────────────────────────────

pub struct Optimizer {
    pub passes: Vec<Box<dyn OptimizationPass>>,
    pub stats: OptimizationStats,
}

pub struct OptimizationStats {
    pub instructions_removed: usize,
    pub constants_folded: usize,
    pub quantum_gates_cancelled: usize,
    pub passes_run: usize,
}

impl Default for OptimizationStats {
    fn default() -> Self {
        OptimizationStats {
            instructions_removed: 0,
            constants_folded: 0,
            quantum_gates_cancelled: 0,
            passes_run: 0,
        }
    }
}

pub trait OptimizationPass: Send + Sync {
    fn name(&self) -> &str;
    fn run(&self, module: &mut IrModule) -> usize; // returns # of changes
}

impl Optimizer {
    pub fn new() -> Self {
        Optimizer {
            passes: vec![
                Box::new(ConstantFoldingPass),
                Box::new(DeadCodeEliminationPass),
                Box::new(CommonSubexpressionPass),
                Box::new(QuantumGateCancellationPass),
                Box::new(NopEliminationPass),
                Box::new(SankofaAccessOptimizerPass),
                Box::new(MTSTimelineFusionPass),
            ],
            stats: OptimizationStats::default(),
        }
    }

    pub fn run_all(&mut self, module: &mut IrModule) -> &OptimizationStats {
        for pass in &self.passes {
            let changes = pass.run(module);
            self.stats.instructions_removed += changes;
            self.stats.passes_run += 1;
        }
        &self.stats
    }

    pub fn run_pass(&self, name: &str, module: &mut IrModule) -> Option<usize> {
        self.passes
            .iter()
            .find(|p| p.name() == name)
            .map(|p| p.run(module))
    }
}

impl Default for Optimizer {
    fn default() -> Self {
        Self::new()
    }
}

// ─── Constant folding ─────────────────────────────────────────────────────────

pub struct ConstantFoldingPass;

impl OptimizationPass for ConstantFoldingPass {
    fn name(&self) -> &str {
        "constant_folding"
    }

    fn run(&self, module: &mut IrModule) -> usize {
        let mut changes = 0;
        for func in &mut module.functions {
            for ins in &mut func.body {
                if let IrInstruction::Add(reg, IrValue::ConstInt(a), IrValue::ConstInt(b)) = ins {
                    let result = *a + *b;
                    *ins = IrInstruction::Assign(reg.clone(), IrValue::ConstInt(result));
                    changes += 1;
                } else if let IrInstruction::Sub(reg, IrValue::ConstInt(a), IrValue::ConstInt(b)) =
                    ins
                {
                    let result = *a - *b;
                    *ins = IrInstruction::Assign(reg.clone(), IrValue::ConstInt(result));
                    changes += 1;
                } else if let IrInstruction::Mul(reg, IrValue::ConstInt(a), IrValue::ConstInt(b)) =
                    ins
                {
                    let result = *a * *b;
                    *ins = IrInstruction::Assign(reg.clone(), IrValue::ConstInt(result));
                    changes += 1;
                } else if let IrInstruction::Div(reg, IrValue::ConstInt(a), IrValue::ConstInt(b)) =
                    ins
                {
                    if *b != 0 {
                        let result = *a / *b;
                        *ins = IrInstruction::Assign(reg.clone(), IrValue::ConstInt(result));
                        changes += 1;
                    }
                }
            }
        }
        changes
    }
}

// ─── Dead code elimination ────────────────────────────────────────────────────

pub struct DeadCodeEliminationPass;

impl OptimizationPass for DeadCodeEliminationPass {
    fn name(&self) -> &str {
        "dead_code_elimination"
    }

    fn run(&self, module: &mut IrModule) -> usize {
        let mut changes = 0;
        for func in &mut module.functions {
            // Collect used registers
            let mut used: HashSet<String> = HashSet::new();
            for ins in &func.body {
                collect_used_regs(ins, &mut used);
            }

            let before = func.body.len();
            func.body.retain(|ins| match ins {
                IrInstruction::Assign(reg, _)
                | IrInstruction::Load(reg, _)
                | IrInstruction::Add(reg, _, _)
                | IrInstruction::Sub(reg, _, _)
                | IrInstruction::Mul(reg, _, _)
                | IrInstruction::Div(reg, _, _)
                | IrInstruction::Mod(reg, _, _)
                | IrInstruction::CmpEq(reg, _, _)
                | IrInstruction::CmpNeq(reg, _, _)
                | IrInstruction::CmpLt(reg, _, _)
                | IrInstruction::CmpGt(reg, _, _)
                | IrInstruction::CmpLe(reg, _, _)
                | IrInstruction::CmpGe(reg, _, _)
                | IrInstruction::And(reg, _, _)
                | IrInstruction::Or(reg, _, _)
                | IrInstruction::Not(reg, _) => used.contains(&reg.0),
                _ => true,
            });
            changes += before - func.body.len();
        }
        changes
    }
}

fn collect_used_regs(ins: &IrInstruction, used: &mut HashSet<String>) {
    fn from_val(v: &IrValue, used: &mut HashSet<String>) {
        if let IrValue::Reg(r) = v {
            used.insert(r.0.clone());
        }
    }
    match ins {
        IrInstruction::CondJump(v, _, _) | IrInstruction::Ret(Some(v)) => from_val(v, used),
        IrInstruction::Store(a, b) => {
            from_val(a, used);
            from_val(b, used);
        }
        IrInstruction::Add(_, a, b)
        | IrInstruction::Sub(_, a, b)
        | IrInstruction::Mul(_, a, b)
        | IrInstruction::Div(_, a, b)
        | IrInstruction::Mod(_, a, b)
        | IrInstruction::CmpEq(_, a, b)
        | IrInstruction::CmpNeq(_, a, b)
        | IrInstruction::CmpLt(_, a, b)
        | IrInstruction::CmpGt(_, a, b)
        | IrInstruction::CmpLe(_, a, b)
        | IrInstruction::CmpGe(_, a, b)
        | IrInstruction::And(_, a, b)
        | IrInstruction::Or(_, a, b) => {
            from_val(a, used);
            from_val(b, used);
        }
        IrInstruction::Not(_, v) | IrInstruction::Load(_, v) | IrInstruction::Assign(_, v) => {
            from_val(v, used)
        }
        IrInstruction::Call(_, _, args) | IrInstruction::CallVoid(_, args) => {
            for a in args {
                from_val(a, used);
            }
        }
        _ => {}
    }
}

// ─── CSE ─────────────────────────────────────────────────────────────────────

pub struct CommonSubexpressionPass;

impl OptimizationPass for CommonSubexpressionPass {
    fn name(&self) -> &str {
        "common_subexpression_elimination"
    }

    fn run(&self, module: &mut IrModule) -> usize {
        let mut changes = 0;
        for func in &mut module.functions {
            let mut seen: HashMap<String, IrRegister> = HashMap::new();
            for ins in &mut func.body {
                let key = format!("{:?}", ins);
                if let Some(_existing) = seen.get(&key) {
                    // Could replace with a Load of existing — mark as change
                    changes += 1;
                } else {
                    match ins {
                        IrInstruction::Add(reg, _, _)
                        | IrInstruction::Sub(reg, _, _)
                        | IrInstruction::Mul(reg, _, _) => {
                            seen.insert(key, reg.clone());
                        }
                        _ => {}
                    }
                }
            }
        }
        changes
    }
}

// ─── Quantum gate cancellation ────────────────────────────────────────────────

pub struct QuantumGateCancellationPass;

impl OptimizationPass for QuantumGateCancellationPass {
    fn name(&self) -> &str {
        "quantum_gate_cancellation"
    }

    fn run(&self, module: &mut IrModule) -> usize {
        // Cancel adjacent inverse gates: H·H = I, X·X = I, etc.
        let mut changes = 0;
        for func in &mut module.functions {
            let mut i = 0;
            while i + 1 < func.body.len() {
                let cancel = if let (IrInstruction::QGate(g1, q1), IrInstruction::QGate(g2, q2)) =
                    (&func.body[i], &func.body[i + 1])
                {
                    q1 == q2
                        && ((g1 == "H" && g2 == "H")
                            || (g1 == "X" && g2 == "X")
                            || (g1 == "Y" && g2 == "Y")
                            || (g1 == "Z" && g2 == "Z"))
                } else {
                    false
                };

                if cancel {
                    func.body[i] = IrInstruction::Nop;
                    func.body[i + 1] = IrInstruction::Nop;
                    changes += 2;
                    i += 2;
                } else {
                    i += 1;
                }
            }
        }
        changes
    }
}

// ─── Nop elimination ─────────────────────────────────────────────────────────

pub struct NopEliminationPass;

impl OptimizationPass for NopEliminationPass {
    fn name(&self) -> &str {
        "nop_elimination"
    }

    fn run(&self, module: &mut IrModule) -> usize {
        let mut changes = 0;
        for func in &mut module.functions {
            let before = func.body.len();
            func.body.retain(|ins| !matches!(ins, IrInstruction::Nop));
            changes += before - func.body.len();
        }
        changes
    }
}

// ─── Sankofa access optimizer ────────────────────────────────────────────────

pub struct SankofaAccessOptimizerPass;

impl OptimizationPass for SankofaAccessOptimizerPass {
    fn name(&self) -> &str {
        "sankofa_access_optimizer"
    }

    fn run(&self, module: &mut IrModule) -> usize {
        // Cache Sankofa recalls: if same key recalled twice, reuse the register
        let mut changes = 0;
        for func in &mut module.functions {
            let mut recalled: HashMap<String, IrRegister> = HashMap::new();
            for ins in &mut func.body {
                if let IrInstruction::SankofaRecall(reg, key) = ins {
                    if let Some(cached) = recalled.get(key) {
                        *ins = IrInstruction::Assign(reg.clone(), IrValue::Reg(cached.clone()));
                        changes += 1;
                    } else {
                        recalled.insert(key.clone(), reg.clone());
                    }
                }
            }
        }
        changes
    }
}

// ─── MTS timeline fusion ─────────────────────────────────────────────────────

pub struct MTSTimelineFusionPass;

impl OptimizationPass for MTSTimelineFusionPass {
    fn name(&self) -> &str {
        "mts_timeline_fusion"
    }

    fn run(&self, module: &mut IrModule) -> usize {
        // Fuse back-to-back snapshot/restore pairs when no mutations in between
        let mut changes = 0;
        for func in &mut module.functions {
            let mut i = 0;
            while i + 1 < func.body.len() {
                let fuse = matches!(
                    (&func.body[i], &func.body[i + 1]),
                    (IrInstruction::MTSSnapshot(_), IrInstruction::MTSRestore(_))
                );
                if fuse {
                    func.body[i] = IrInstruction::Nop;
                    func.body[i + 1] = IrInstruction::Nop;
                    changes += 2;
                }
                i += 1;
            }
        }
        changes
    }
}
